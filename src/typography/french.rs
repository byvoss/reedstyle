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
        
        // Replace double quotes with guillemets
        // Opening quotes after space or start
        let re_open = regex::Regex::new(r#"(^|\s)"([^"]*)"#).unwrap();
        result = re_open.replace_all(&result, format!("$1{} $2", LAQUO).as_str()).to_string();
        
        // Closing quotes
        let re_close = regex::Regex::new(&format!(r#"{} ([^"]*)"#, LAQUO)).unwrap();
        result = re_close.replace_all(&result, format!("{} $1 {}", LAQUO, RAQUO).as_str()).to_string();
        
        // Secondary quotes (English style in French)
        let re_single_open = regex::Regex::new(r"(^|\s)'([^']*)").unwrap();
        result = re_single_open.replace_all(&result, format!("$1{}$2", LDQUO).as_str()).to_string();
        
        let re_single_close = regex::Regex::new(&format!(r"{}([^']*)'", LDQUO)).unwrap();
        result = re_single_close.replace_all(&result, format!("{}$1{}", LDQUO, RDQUO).as_str()).to_string();
        
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