use anyhow::Result;
use std::collections::HashMap;

/// Advanced CSS minification with deduplication and grouping
pub fn minify_css(css: &str) -> Result<String> {
    // First, remove comments
    let css = remove_comments(css);
    
    // Parse CSS into rules
    let rules = parse_css_rules(&css);
    
    // Group selectors with identical properties
    let grouped = group_identical_rules(rules);
    
    // Find common values that could be variables
    let (variables, optimized_rules) = extract_common_values(grouped);
    
    // Build optimized CSS
    let mut output = String::new();
    
    // Add @layer declaration first
    if css.contains("@layer") {
        output.push_str("@layer settings,bridge,theme,free;");
    }
    
    // Add common variables if worthwhile
    if !variables.is_empty() {
        output.push_str(":root{");
        for (var_name, value) in &variables {
            output.push_str(&format!("--_{}:{};", var_name, value));
        }
        output.push_str("}");
    }
    
    // Add optimized rules
    for (selectors, properties) in optimized_rules {
        if selectors.is_empty() || properties.is_empty() {
            continue;
        }
        
        // Join multiple selectors with same properties
        let selector_str = selectors.join(",");
        output.push_str(&selector_str);
        output.push('{');
        
        for (prop, value) in properties {
            // Use variable reference if available
            if let Some(var_name) = variables.iter()
                .find(|(_, v)| *v == &value)
                .map(|(n, _)| n) 
            {
                // Only use variable if it saves space
                let var_ref = format!("var(--_{})", var_name);
                if var_ref.len() < value.len() {
                    output.push_str(&format!("{}:{}", prop, var_ref));
                } else {
                    output.push_str(&format!("{}:{}", prop, value));
                }
            } else {
                output.push_str(&format!("{}:{}", prop, value));
            }
            output.push(';');
        }
        
        // Remove last semicolon to save bytes
        output.pop();
        output.push('}');
    }
    
    // Final optimizations
    let output = output
        .replace(":0px", ":0")
        .replace(":0em", ":0")
        .replace(":0rem", ":0")
        .replace(" 0px", " 0")
        .replace(" 0em", " 0")
        .replace(" 0rem", " 0")
        .replace(":0 0 0 0", ":0")
        .replace(":0 0 0", ":0")
        .replace(":0 0", ":0");
    
    Ok(output)
}

fn remove_comments(css: &str) -> String {
    let mut result = String::new();
    let mut in_comment = false;
    let mut chars = css.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if !in_comment && ch == '/' && chars.peek() == Some(&'*') {
            chars.next();
            in_comment = true;
        } else if in_comment && ch == '*' && chars.peek() == Some(&'/') {
            chars.next();
            in_comment = false;
        } else if !in_comment {
            result.push(ch);
        }
    }
    
    result
}

#[derive(Debug, Clone)]
struct CssRule {
    selector: String,
    properties: Vec<(String, String)>,
}

fn parse_css_rules(css: &str) -> Vec<CssRule> {
    let mut rules = Vec::new();
    let mut current_selector = String::new();
    let mut current_properties = Vec::new();
    let mut in_selector = true;
    let mut in_value = false;
    let mut current_prop = String::new();
    let mut current_val = String::new();
    let mut brace_depth = 0;
    
    for ch in css.chars() {
        match ch {
            '{' => {
                brace_depth += 1;
                if brace_depth == 1 {
                    in_selector = false;
                    current_selector = current_selector.trim().to_string();
                }
            }
            '}' => {
                brace_depth -= 1;
                if brace_depth == 0 {
                    // Save current property if any
                    if !current_prop.is_empty() {
                        current_properties.push((
                            current_prop.trim().to_string(),
                            current_val.trim().to_string()
                        ));
                    }
                    
                    // Save rule if it has properties
                    if !current_selector.is_empty() && !current_properties.is_empty() {
                        rules.push(CssRule {
                            selector: current_selector.clone(),
                            properties: current_properties.clone(),
                        });
                    }
                    
                    // Reset for next rule
                    current_selector.clear();
                    current_properties.clear();
                    current_prop.clear();
                    current_val.clear();
                    in_selector = true;
                    in_value = false;
                }
            }
            ':' if brace_depth == 1 && !in_value => {
                in_value = true;
            }
            ';' if brace_depth == 1 => {
                if !current_prop.is_empty() {
                    current_properties.push((
                        current_prop.trim().to_string(),
                        current_val.trim().to_string()
                    ));
                }
                current_prop.clear();
                current_val.clear();
                in_value = false;
            }
            '\n' | '\r' | '\t' => {
                // Replace with space
                if in_selector {
                    current_selector.push(' ');
                } else if in_value {
                    current_val.push(' ');
                } else if brace_depth == 1 {
                    current_prop.push(' ');
                }
            }
            _ => {
                if in_selector {
                    current_selector.push(ch);
                } else if in_value {
                    current_val.push(ch);
                } else if brace_depth == 1 {
                    current_prop.push(ch);
                }
            }
        }
    }
    
    rules
}

fn group_identical_rules(rules: Vec<CssRule>) -> Vec<(Vec<String>, Vec<(String, String)>)> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    
    for rule in rules {
        // Create a key from sorted properties
        let mut props = rule.properties.clone();
        props.sort_by(|a, b| a.0.cmp(&b.0));
        let key = props.iter()
            .map(|(k, v)| format!("{}:{}", k, v))
            .collect::<Vec<_>>()
            .join(";");
        
        groups.entry(key)
            .or_insert_with(Vec::new)
            .push(rule.selector);
    }
    
    // Convert back to rules format
    let mut result = Vec::new();
    for (prop_key, selectors) in groups {
        if prop_key.is_empty() {
            continue;
        }
        
        let properties: Vec<(String, String)> = prop_key
            .split(';')
            .filter_map(|p| {
                let parts: Vec<&str> = p.splitn(2, ':').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string()))
                } else {
                    None
                }
            })
            .collect();
        
        result.push((selectors, properties));
    }
    
    result
}

fn extract_common_values(
    rules: Vec<(Vec<String>, Vec<(String, String)>)>
) -> (HashMap<String, String>, Vec<(Vec<String>, Vec<(String, String)>)>) {
    let mut value_counts: HashMap<String, usize> = HashMap::new();
    
    // Count value occurrences
    for (_, properties) in &rules {
        for (_, value) in properties {
            // Only consider values that are worth making into variables
            if value.len() > 10 && !value.starts_with("var(") && !value.contains("oklch") {
                *value_counts.entry(value.clone()).or_insert(0) += 1;
            }
        }
    }
    
    // Create variables for frequently used values
    let mut variables = HashMap::new();
    let mut var_counter = 0;
    
    for (value, count) in value_counts {
        // Only create variable if used 5+ times and saves space
        if count >= 5 {
            let var_name = format!("{}", var_counter);
            let savings = (value.len() * count) - (var_name.len() + value.len() + 10);
            if savings > 0 {
                variables.insert(var_name, value);
                var_counter += 1;
            }
        }
    }
    
    (variables, rules)
}

/// For development - no minification
pub fn optimize_css(css: &str) -> Result<String> {
    Ok(css.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grouping() {
        let css = r#"
            .a { color: red; padding: 10px; }
            .b { color: red; padding: 10px; }
            .c { color: blue; }
        "#;
        
        let result = minify_css(css).unwrap();
        // .a and .b should be grouped
        assert!(result.contains(".a,.b{") || result.contains(".b,.a{"));
    }
    
    #[test]
    fn test_oklch_preservation() {
        let css = ":root { --color: oklch(68.5% 0.24 25); }";
        let result = minify_css(css).unwrap();
        assert!(result.contains("oklch(68.5% 0.24 25)"));
    }
}