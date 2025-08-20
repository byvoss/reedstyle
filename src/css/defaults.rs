use anyhow::Result;

/// Generate CSS for standard HTML element defaults
/// These are hard-coded because HTML standards are universal
pub fn generate_html_defaults() -> String {
    let mut css = String::new();
    
    css.push_str("\n  /* ========== HTML Element Defaults ========== */\n");
    
    // Base reed element (no 'as' attribute = div)
    css.push_str("  r-s {\n");
    css.push_str("    display: block;\n");
    css.push_str("  }\n\n");
    
    // Block elements
    css.push_str("  /* Block elements */\n");
    css.push_str("  r-s[as=\"div\"], r-s[as=\"section\"], r-s[as=\"article\"],\n");
    css.push_str("  r-s[as=\"header\"], r-s[as=\"footer\"], r-s[as=\"main\"],\n");
    css.push_str("  r-s[as=\"nav\"], r-s[as=\"aside\"], r-s[as=\"figure\"],\n");
    css.push_str("  r-s[as=\"figcaption\"], r-s[as=\"address\"] {\n");
    css.push_str("    display: block;\n");
    css.push_str("  }\n\n");
    
    // Headings
    css.push_str("  /* Headings */\n");
    css.push_str("  r-s[as=\"h1\"] { display: block; font-size: 2em; font-weight: bold; margin: 0.67em 0; }\n");
    css.push_str("  r-s[as=\"h2\"] { display: block; font-size: 1.5em; font-weight: bold; margin: 0.83em 0; }\n");
    css.push_str("  r-s[as=\"h3\"] { display: block; font-size: 1.17em; font-weight: bold; margin: 1em 0; }\n");
    css.push_str("  r-s[as=\"h4\"] { display: block; font-size: 1em; font-weight: bold; margin: 1.33em 0; }\n");
    css.push_str("  r-s[as=\"h5\"] { display: block; font-size: 0.83em; font-weight: bold; margin: 1.67em 0; }\n");
    css.push_str("  r-s[as=\"h6\"] { display: block; font-size: 0.67em; font-weight: bold; margin: 2.33em 0; }\n\n");
    
    // Text block elements
    css.push_str("  /* Text blocks */\n");
    css.push_str("  r-s[as=\"p\"] { display: block; margin: 1em 0; }\n");
    css.push_str("  r-s[as=\"blockquote\"] { display: block; margin: 1em 40px; }\n");
    css.push_str("  r-s[as=\"pre\"] { display: block; font-family: monospace; white-space: pre; margin: 1em 0; }\n\n");
    
    // Inline elements
    css.push_str("  /* Inline elements */\n");
    css.push_str("  r-s[as=\"span\"], r-s[as=\"a\"], r-s[as=\"abbr\"],\n");
    css.push_str("  r-s[as=\"cite\"], r-s[as=\"code\"], r-s[as=\"dfn\"],\n");
    css.push_str("  r-s[as=\"kbd\"], r-s[as=\"mark\"], r-s[as=\"q\"],\n");
    css.push_str("  r-s[as=\"samp\"], r-s[as=\"time\"], r-s[as=\"var\"] {\n");
    css.push_str("    display: inline;\n");
    css.push_str("  }\n\n");
    
    // Inline formatting
    css.push_str("  /* Inline formatting */\n");
    css.push_str("  r-s[as=\"strong\"], r-s[as=\"b\"] { display: inline; font-weight: bold; }\n");
    css.push_str("  r-s[as=\"em\"], r-s[as=\"i\"] { display: inline; font-style: italic; }\n");
    css.push_str("  r-s[as=\"u\"] { display: inline; text-decoration: underline; }\n");
    css.push_str("  r-s[as=\"s\"], r-s[as=\"strike\"], r-s[as=\"del\"] { display: inline; text-decoration: line-through; }\n");
    css.push_str("  r-s[as=\"ins\"] { display: inline; text-decoration: underline; }\n");
    css.push_str("  r-s[as=\"small\"] { display: inline; font-size: smaller; }\n");
    css.push_str("  r-s[as=\"sub\"] { display: inline; vertical-align: sub; font-size: smaller; }\n");
    css.push_str("  r-s[as=\"sup\"] { display: inline; vertical-align: super; font-size: smaller; }\n");
    css.push_str("  r-s[as=\"code\"] { display: inline; font-family: monospace; font-size: 0.875em; }\n");
    css.push_str("  r-s[as=\"kbd\"] { display: inline; font-family: monospace; font-size: 0.875em; }\n");
    css.push_str("  r-s[as=\"samp\"] { display: inline; font-family: monospace; font-size: 0.875em; }\n");
    css.push_str("  r-s[as=\"var\"] { display: inline; font-style: italic; }\n");
    css.push_str("  r-s[as=\"mark\"] { display: inline; background-color: yellow; color: black; }\n\n");
    
    // Lists
    css.push_str("  /* Lists */\n");
    css.push_str("  r-s[as=\"ul\"] { display: block; list-style-type: disc; margin: 1em 0; padding-left: 40px; }\n");
    css.push_str("  r-s[as=\"ol\"] { display: block; list-style-type: decimal; margin: 1em 0; padding-left: 40px; }\n");
    css.push_str("  r-s[as=\"li\"] { display: list-item; }\n");
    css.push_str("  r-s[as=\"dl\"] { display: block; margin: 1em 0; }\n");
    css.push_str("  r-s[as=\"dt\"] { display: block; font-weight: bold; }\n");
    css.push_str("  r-s[as=\"dd\"] { display: block; margin-left: 40px; }\n\n");
    
    // Tables
    css.push_str("  /* Tables */\n");
    css.push_str("  r-s[as=\"table\"] { display: table; border-collapse: separate; border-spacing: 2px; }\n");
    css.push_str("  r-s[as=\"caption\"] { display: table-caption; text-align: center; }\n");
    css.push_str("  r-s[as=\"thead\"] { display: table-header-group; vertical-align: middle; }\n");
    css.push_str("  r-s[as=\"tbody\"] { display: table-row-group; vertical-align: middle; }\n");
    css.push_str("  r-s[as=\"tfoot\"] { display: table-footer-group; vertical-align: middle; }\n");
    css.push_str("  r-s[as=\"tr\"] { display: table-row; vertical-align: inherit; }\n");
    css.push_str("  r-s[as=\"td\"] { display: table-cell; vertical-align: inherit; padding: 1px; }\n");
    css.push_str("  r-s[as=\"th\"] { display: table-cell; vertical-align: inherit; font-weight: bold; text-align: center; padding: 1px; }\n");
    css.push_str("  r-s[as=\"colgroup\"] { display: table-column-group; }\n");
    css.push_str("  r-s[as=\"col\"] { display: table-column; }\n\n");
    
    // Forms
    css.push_str("  /* Forms */\n");
    css.push_str("  r-s[as=\"form\"] { display: block; margin: 0; }\n");
    css.push_str("  r-s[as=\"fieldset\"] { display: block; margin: 0 2px; padding: 0.35em 0.625em 0.75em; border: 2px groove; }\n");
    css.push_str("  r-s[as=\"legend\"] { display: block; padding: 0 2px; }\n");
    css.push_str("  r-s[as=\"label\"] { display: inline; cursor: default; }\n");
    css.push_str("  r-s[as=\"button\"] { display: inline-block; }\n");
    css.push_str("  r-s[as=\"input\"] { display: inline-block; }\n");
    css.push_str("  r-s[as=\"select\"] { display: inline-block; }\n");
    css.push_str("  r-s[as=\"datalist\"] { display: none; }\n");
    css.push_str("  r-s[as=\"optgroup\"] { display: block; }\n");
    css.push_str("  r-s[as=\"option\"] { display: block; }\n");
    css.push_str("  r-s[as=\"textarea\"] { display: inline-block; }\n");
    css.push_str("  r-s[as=\"output\"] { display: inline; }\n");
    css.push_str("  r-s[as=\"progress\"] { display: inline-block; vertical-align: baseline; }\n");
    css.push_str("  r-s[as=\"meter\"] { display: inline-block; vertical-align: baseline; }\n\n");
    
    // Interactive elements
    css.push_str("  /* Interactive elements */\n");
    css.push_str("  r-s[as=\"details\"] { display: block; }\n");
    css.push_str("  r-s[as=\"summary\"] { display: block; }\n");
    css.push_str("  r-s[as=\"dialog\"] { display: block; position: absolute; left: 0; right: 0; margin: auto; border: solid; padding: 1em; }\n\n");
    
    // Media
    css.push_str("  /* Media */\n");
    css.push_str("  r-s[as=\"img\"] { display: inline-block; }\n");
    css.push_str("  r-s[as=\"picture\"] { display: inline-block; }\n");
    css.push_str("  r-s[as=\"source\"] { display: none; }\n");
    css.push_str("  r-s[as=\"svg\"] { display: inline-block; }\n");
    css.push_str("  r-s[as=\"video\"] { display: inline-block; }\n");
    css.push_str("  r-s[as=\"audio\"] { display: inline-block; }\n");
    css.push_str("  r-s[as=\"track\"] { display: none; }\n");
    css.push_str("  r-s[as=\"map\"] { display: inline; }\n");
    css.push_str("  r-s[as=\"area\"] { display: none; }\n");
    css.push_str("  r-s[as=\"canvas\"] { display: inline-block; }\n");
    css.push_str("  r-s[as=\"iframe\"] { display: inline-block; border: 2px inset; }\n");
    css.push_str("  r-s[as=\"embed\"] { display: inline-block; }\n");
    css.push_str("  r-s[as=\"object\"] { display: inline-block; }\n");
    css.push_str("  r-s[as=\"param\"] { display: none; }\n\n");
    
    // Other
    css.push_str("  /* Other */\n");
    css.push_str("  r-s[as=\"hr\"] { display: block; margin: 0.5em auto; border-style: inset; border-width: 1px; }\n");
    css.push_str("  r-s[as=\"br\"] { display: inline; }\n");
    css.push_str("  r-s[as=\"wbr\"] { display: inline; }\n\n");
    
    // Hidden elements
    css.push_str("  /* Hidden by default */\n");
    css.push_str("  r-s[as=\"script\"], r-s[as=\"style\"], r-s[as=\"template\"],\n");
    css.push_str("  r-s[as=\"head\"], r-s[as=\"meta\"], r-s[as=\"link\"],\n");
    css.push_str("  r-s[as=\"base\"], r-s[as=\"title\"], r-s[as=\"noscript\"] {\n");
    css.push_str("    display: none;\n");
    css.push_str("  }\n\n");
    
    css
}

/// Generate CSS for custom components from YAML config
pub fn generate_component_defaults(components: &crate::config::ComponentsConfig) -> Result<String> {
    let mut css = String::new();
    
    css.push_str("\n  /* ========== Custom Components ========== */\n");
    
    for (name, component) in &components.components {
        // Start component rule
        css.push_str(&format!("  r-s[as=\"{}\"] {{\n", name));
        
        // Get base element (default to div)
        let base_element = component.element.as_deref().unwrap_or("div");
        
        // Apply base element defaults based on type
        css.push_str(&get_element_base_styles(base_element));
        
        // Note: Namespace styles (box, face, text, etc.) will be handled
        // by the namespace processors when the element is rendered
        // This just ensures the correct display type and basic defaults
        
        css.push_str("  }\n");
    }
    
    css.push_str("\n");
    Ok(css)
}

/// Get the base styles for a given HTML element
fn get_element_base_styles(element: &str) -> String {
    match element {
        // Block elements
        "div" | "section" | "article" | "header" | "footer" | 
        "main" | "nav" | "aside" | "figure" | "figcaption" | 
        "address" | "form" | "fieldset" => {
            "    display: block;\n".to_string()
        }
        
        // Headings
        "h1" => "    display: block; font-size: 2em; font-weight: bold; margin: 0.67em 0;\n".to_string(),
        "h2" => "    display: block; font-size: 1.5em; font-weight: bold; margin: 0.83em 0;\n".to_string(),
        "h3" => "    display: block; font-size: 1.17em; font-weight: bold; margin: 1em 0;\n".to_string(),
        "h4" => "    display: block; font-size: 1em; font-weight: bold; margin: 1.33em 0;\n".to_string(),
        "h5" => "    display: block; font-size: 0.83em; font-weight: bold; margin: 1.67em 0;\n".to_string(),
        "h6" => "    display: block; font-size: 0.67em; font-weight: bold; margin: 2.33em 0;\n".to_string(),
        
        // Text blocks
        "p" => "    display: block; margin: 1em 0;\n".to_string(),
        "blockquote" => "    display: block; margin: 1em 40px;\n".to_string(),
        "pre" => "    display: block; font-family: monospace; white-space: pre; margin: 1em 0;\n".to_string(),
        
        // Inline elements
        "span" | "a" | "abbr" | "cite" | "code" | "dfn" |
        "kbd" | "mark" | "q" | "samp" | "time" | "var" => {
            "    display: inline;\n".to_string()
        }
        
        // Inline formatting
        "strong" | "b" => "    display: inline; font-weight: bold;\n".to_string(),
        "em" | "i" => "    display: inline; font-style: italic;\n".to_string(),
        "small" => "    display: inline; font-size: smaller;\n".to_string(),
        
        // Lists
        "ul" => "    display: block; list-style-type: disc; margin: 1em 0; padding-left: 40px;\n".to_string(),
        "ol" => "    display: block; list-style-type: decimal; margin: 1em 0; padding-left: 40px;\n".to_string(),
        "li" => "    display: list-item;\n".to_string(),
        "dl" => "    display: block; margin: 1em 0;\n".to_string(),
        "dt" => "    display: block; font-weight: bold;\n".to_string(),
        "dd" => "    display: block; margin-left: 40px;\n".to_string(),
        
        // Tables
        "table" => "    display: table; border-collapse: separate; border-spacing: 2px;\n".to_string(),
        "caption" => "    display: table-caption; text-align: center;\n".to_string(),
        "thead" => "    display: table-header-group; vertical-align: middle;\n".to_string(),
        "tbody" => "    display: table-row-group; vertical-align: middle;\n".to_string(),
        "tfoot" => "    display: table-footer-group; vertical-align: middle;\n".to_string(),
        "tr" => "    display: table-row; vertical-align: inherit;\n".to_string(),
        "td" => "    display: table-cell; vertical-align: inherit; padding: 1px;\n".to_string(),
        "th" => "    display: table-cell; vertical-align: inherit; font-weight: bold; text-align: center; padding: 1px;\n".to_string(),
        
        // Form elements
        "button" | "input" | "select" | "textarea" => {
            "    display: inline-block;\n".to_string()
        }
        "label" => "    display: inline; cursor: default;\n".to_string(),
        
        // Media
        "img" | "picture" | "svg" | "video" | "audio" | "canvas" | "iframe" | "embed" | "object" => {
            "    display: inline-block;\n".to_string()
        }
        
        // Default to block
        _ => "    display: block;\n".to_string()
    }
}