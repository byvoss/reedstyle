use anyhow::Result;

// Unicode constants for French typography
const LAQUO: &str = "\u{00AB}";    // Left guillemet «
const RAQUO: &str = "\u{00BB}";    // Right guillemet »
const LDQUO: &str = "\u{201C}";    // Left double quote "
const RDQUO: &str = "\u{201D}";    // Right double quote "
const NBSP: &str = "\u{00A0}";     // Non-breaking space
const NNBSP: &str = "\u{202F}";    // Narrow no-break space
const ELLIPSIS: &str = "\u{2026}"; // Ellipsis …
const ENDASH: &str = "\u{2013}";   // En dash –
const EMDASH: &str = "\u{2014}";   // Em dash —

/// French typography rules
pub struct FrenchRules;

impl FrenchRules {
    /// Get all replacement patterns for French typography
    pub fn get_replacements() -> Vec<(&'static str, &'static str)> {
        vec![
            // Three dots to ellipsis
            ("...", "…"),
            
            // Double hyphen to em dash (French uses spaced em dash)
            ("--", " — "),
            
            // Single hyphen with spaces to en dash
            (" - ", " – "),
            
            // Common abbreviations with non-breaking space
            ("M. ", "M.\u{00A0}"),
            ("Mme ", "Mme\u{00A0}"),
            ("Mlle ", "Mlle\u{00A0}"),
            ("Dr ", "Dr\u{00A0}"),
            ("Me ", "Me\u{00A0}"),  // Maître
            
            // Numbers before certain punctuation need narrow no-break space
            // This is complex and would need regex in real implementation
        ]
    }
    
    /// Apply French quotation marks (guillemets)
    pub fn apply_quotes(text: &str) -> String {
        let mut result = text.to_string();
        
        // Simple approach: replace paired double quotes with guillemets
        if result.contains("\"") {
            let parts: Vec<&str> = result.split("\"").collect();
            if parts.len() >= 3 {
                // We have at least one pair of quotes
                let mut new_result = String::new();
                for (i, part) in parts.iter().enumerate() {
                    new_result.push_str(part);
                    if i < parts.len() - 1 {
                        // Alternate between opening and closing guillemets
                        // French guillemets have spaces inside
                        if i % 2 == 0 {
                            new_result.push_str(&format!("{} ", LAQUO)); // « with space
                        } else {
                            new_result.push_str(&format!(" {}", RAQUO)); // » with space
                        }
                    }
                }
                result = new_result;
            }
        }
        
        // Handle single quotes (secondary quotes in French use English style)
        if result.contains("'") {
            let parts: Vec<&str> = result.split("'").collect();
            if parts.len() >= 3 {
                let mut new_result = String::new();
                for (i, part) in parts.iter().enumerate() {
                    new_result.push_str(part);
                    if i < parts.len() - 1 {
                        if i % 2 == 0 {
                            new_result.push_str(LDQUO); // English opening "
                        } else {
                            new_result.push_str(RDQUO); // English closing "
                        }
                    }
                }
                result = new_result;
            }
        }
        
        result
    }
    
    /// Format numbers with French spacing rules
    pub fn format_numbers(text: &str) -> String {
        let mut result = text.to_string();
        
        // Format thousands with non-breaking space (French standard)
        let re = regex::Regex::new(r"\b(\d{1,3})(\d{3})\b").unwrap();
        result = re.replace_all(&result, "$1\u{00A0}$2").to_string();
        
        // Format decimals with comma (French uses comma, not period)
        let re_decimal = regex::Regex::new(r"(\d+)\.(\d+)").unwrap();
        result = re_decimal.replace_all(&result, "$1,$2").to_string();
        
        // Add narrow no-break space before : ; ! ?
        let re_punctuation = regex::Regex::new(r"(\w)\s*([;:!?])").unwrap();
        result = re_punctuation.replace_all(&result, "$1\u{202F}$2").to_string();
        
        // Currency (Euro sign after with non-breaking space)
        let re_currency = regex::Regex::new(r"(\d+(?:,\d{2})?)\s*(€|EUR)").unwrap();
        result = re_currency.replace_all(&result, "$1\u{00A0}$2").to_string();
        
        result
    }
    
    /// Apply all French typography rules
    pub fn apply_all(text: &str) -> String {
        let mut result = text.to_string();
        
        // Apply replacements
        for (pattern, replacement) in Self::get_replacements() {
            if pattern.contains('(') {
                // Regex pattern
                if let Ok(re) = regex::Regex::new(pattern) {
                    result = re.replace_all(&result, replacement).to_string();
                }
            } else {
                // Simple string replacement
                result = result.replace(pattern, replacement);
            }
        }
        
        // Apply number formatting
        result = Self::format_numbers(&result);
        
        // Apply quotes
        result = Self::apply_quotes(&result);
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_guillemets() {
        assert_eq!(
            FrenchRules::apply_quotes("\"Bonjour\""),
            format!("{} Bonjour {}", LAQUO, RAQUO)
        );
        assert_eq!(
            FrenchRules::apply_quotes("'Salut'"),
            format!("{}Salut{}", LDQUO, RDQUO)
        );
    }
    
    #[test]
    fn test_number_formatting() {
        assert_eq!(
            FrenchRules::format_numbers("10000"),
            format!("10{}000", NBSP)
        );
        assert_eq!(
            FrenchRules::format_numbers("29.99"),
            "29,99"
        );
        assert_eq!(
            FrenchRules::format_numbers("Prix : 29,99 €"),
            format!("Prix{}: 29,99{}€", NNBSP, NBSP)
        );
    }
    
    #[test]
    fn test_punctuation_spacing() {
        let text = "Quoi ? C'est vrai !";
        let result = FrenchRules::format_numbers(text);
        assert!(result.contains(&format!("{}?", NNBSP)));
        assert!(result.contains(&format!("{}!", NNBSP)));
    }
}