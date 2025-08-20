use anyhow::Result;

// Unicode constants for English typography
const LDQUO: &str = "\u{201C}";    // Left double quote "
const RDQUO: &str = "\u{201D}";    // Right double quote "
const LSQUO: &str = "\u{2018}";    // Left single quote '
const RSQUO: &str = "\u{2019}";    // Right single quote '
const ELLIPSIS: &str = "\u{2026}"; // Ellipsis …
const ENDASH: &str = "\u{2013}";   // En dash –
const EMDASH: &str = "\u{2014}";   // Em dash —

/// English typography rules (US and GB variants)
pub struct EnglishRules;

impl EnglishRules {
    /// Get replacements for US English
    pub fn get_us_replacements() -> Vec<(&'static str, &'static str)> {
        vec![
            // Three dots to ellipsis
            ("...", "\u{2026}"),
            
            // Double hyphen to em dash (US style, no spaces)
            ("--", "\u{2014}"),
            
            // Hyphen with spaces to en dash (for ranges)
            (" - ", " \u{2013} "),
            
            // Common contractions
            ("won't", "won\u{2019}t"),
            ("can't", "can\u{2019}t"),
            ("shouldn't", "shouldn\u{2019}t"),
            ("wouldn't", "wouldn\u{2019}t"),
            ("couldn't", "couldn\u{2019}t"),
            ("didn't", "didn\u{2019}t"),
            ("doesn't", "doesn\u{2019}t"),
            ("don't", "don\u{2019}t"),
            ("isn't", "isn\u{2019}t"),
            ("aren't", "aren\u{2019}t"),
            ("wasn't", "wasn\u{2019}t"),
            ("weren't", "weren\u{2019}t"),
            ("hasn't", "hasn\u{2019}t"),
            ("haven't", "haven\u{2019}t"),
            ("hadn't", "hadn\u{2019}t"),
            ("it's", "it\u{2019}s"),
            ("that's", "that\u{2019}s"),
            ("what's", "what\u{2019}s"),
            ("there's", "there\u{2019}s"),
            ("here's", "here\u{2019}s"),
            ("who's", "who\u{2019}s"),
            ("she's", "she\u{2019}s"),
            ("he's", "he\u{2019}s"),
            ("let's", "let\u{2019}s"),
            ("I've", "I\u{2019}ve"),
            ("you've", "you\u{2019}ve"),
            ("we've", "we\u{2019}ve"),
            ("they've", "they\u{2019}ve"),
            ("I'll", "I\u{2019}ll"),
            ("you'll", "you\u{2019}ll"),
            ("we'll", "we\u{2019}ll"),
            ("they'll", "they\u{2019}ll"),
            ("I'd", "I\u{2019}d"),
            ("you'd", "you\u{2019}d"),
            ("we'd", "we\u{2019}d"),
            ("they'd", "they\u{2019}d"),
            ("I'm", "I\u{2019}m"),
            ("you're", "you\u{2019}re"),
            ("we're", "we\u{2019}re"),
            ("they're", "they\u{2019}re"),
        ]
    }
    
    /// Get replacements for British English  
    pub fn get_gb_replacements() -> Vec<(&'static str, &'static str)> {
        let mut replacements = Self::get_us_replacements();
        
        // British English uses spaced en dashes instead of unspaced em dashes
        replacements.retain(|(pattern, _)| *pattern != "--");
        replacements.push(("--", " – "));
        
        replacements
    }
    
    /// Apply US English quotation marks (double quotes primary)
    pub fn apply_us_quotes(text: &str) -> String {
        let mut result = text.to_string();
        
        // Simple approach: replace paired quotes
        if result.contains("\"") {
            let parts: Vec<&str> = result.split("\"").collect();
            if parts.len() >= 3 {
                // We have at least one pair of quotes
                let mut new_result = String::new();
                for (i, part) in parts.iter().enumerate() {
                    new_result.push_str(part);
                    if i < parts.len() - 1 {
                        // Alternate between opening and closing quotes
                        if i % 2 == 0 {
                            new_result.push_str(LDQUO);
                        } else {
                            new_result.push_str(RDQUO);
                        }
                    }
                }
                result = new_result;
            }
        }
        
        // Handle single quotes similarly
        if result.contains("'") && !result.contains("'t") && !result.contains("'s") {
            let parts: Vec<&str> = result.split("'").collect();
            if parts.len() >= 3 {
                let mut new_result = String::new();
                for (i, part) in parts.iter().enumerate() {
                    new_result.push_str(part);
                    if i < parts.len() - 1 {
                        if i % 2 == 0 {
                            new_result.push_str(LSQUO);
                        } else {
                            new_result.push_str(RSQUO);
                        }
                    }
                }
                result = new_result;
            }
        }
        
        result
    }
    
    /// Apply British English quotation marks (single quotes primary)
    pub fn apply_gb_quotes(text: &str) -> String {
        let mut result = text.to_string();
        
        // In British English, single quotes are primary
        // Simple approach: replace paired single quotes first
        if result.contains("'") && !result.contains("'t") && !result.contains("'s") {
            let parts: Vec<&str> = result.split("'").collect();
            if parts.len() >= 3 {
                let mut new_result = String::new();
                for (i, part) in parts.iter().enumerate() {
                    new_result.push_str(part);
                    if i < parts.len() - 1 {
                        if i % 2 == 0 {
                            new_result.push_str(LSQUO);
                        } else {
                            new_result.push_str(RSQUO);
                        }
                    }
                }
                result = new_result;
            }
        }
        
        // Then handle double quotes (secondary in GB)
        if result.contains("\"") {
            let parts: Vec<&str> = result.split("\"").collect();
            if parts.len() >= 3 {
                let mut new_result = String::new();
                for (i, part) in parts.iter().enumerate() {
                    new_result.push_str(part);
                    if i < parts.len() - 1 {
                        if i % 2 == 0 {
                            new_result.push_str(LDQUO);
                        } else {
                            new_result.push_str(RDQUO);
                        }
                    }
                }
                result = new_result;
            }
        }
        
        result
    }
    
    /// Format numbers with commas (US/GB style)
    pub fn format_numbers(text: &str) -> String {
        let mut result = text.to_string();
        
        // Add commas to large numbers (1,000 format)
        let re = regex::Regex::new(r"\b(\d{1,3})(\d{3})\b").unwrap();
        result = re.replace_all(&result, "$1,$2").to_string();
        
        // Handle larger numbers (1,000,000)
        let re_million = regex::Regex::new(r"\b(\d{1,3}),(\d{3}),(\d{3})\b").unwrap();
        if re_million.is_match(&result) {
            // Already formatted correctly
        } else {
            let re_big = regex::Regex::new(r"\b(\d{1,3})(\d{3})(\d{3})\b").unwrap();
            result = re_big.replace_all(&result, "$1,$2,$3").to_string();
        }
        
        result
    }
    
    /// Apply all US English typography rules
    pub fn apply_us_all(text: &str) -> String {
        let mut result = text.to_string();
        
        // Apply replacements
        for (pattern, replacement) in Self::get_us_replacements() {
            result = result.replace(pattern, replacement);
        }
        
        // Apply quotes
        result = Self::apply_us_quotes(&result);
        
        // Apply number formatting
        result = Self::format_numbers(&result);
        
        result
    }
    
    /// Apply all British English typography rules
    pub fn apply_gb_all(text: &str) -> String {
        let mut result = text.to_string();
        
        // Apply replacements
        for (pattern, replacement) in Self::get_gb_replacements() {
            result = result.replace(pattern, replacement);
        }
        
        // Apply quotes
        result = Self::apply_gb_quotes(&result);
        
        // Apply number formatting
        result = Self::format_numbers(&result);
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_us_quotes() {
        assert_eq!(
            EnglishRules::apply_us_quotes("\"Hello world\""),
            format!("{}Hello world{}", LDQUO, RDQUO)
        );
        assert_eq!(
            EnglishRules::apply_us_quotes("'Hello'"),
            format!("{}Hello{}", LSQUO, RSQUO)
        );
    }
    
    #[test]
    fn test_gb_quotes() {
        assert_eq!(
            EnglishRules::apply_gb_quotes("'Hello world'"),
            format!("{}Hello world{}", LSQUO, RSQUO)
        );
        assert_eq!(
            EnglishRules::apply_gb_quotes("\"Hello\""),
            format!("{}Hello{}", LDQUO, RDQUO)
        );
    }
    
    #[test]
    fn test_number_formatting() {
        assert_eq!(
            EnglishRules::format_numbers("10000"),
            "10,000"
        );
        assert_eq!(
            EnglishRules::format_numbers("1000000"),
            "1,000,000"
        );
    }
    
    #[test]
    fn test_contractions() {
        let text = "I can't believe it's working!";
        let result = EnglishRules::apply_us_all(text);
        // Check that contractions have the right apostrophe
        assert!(result.contains(&format!("can{}t", RSQUO)));
        assert!(result.contains(&format!("it{}s", RSQUO)));
    }
}