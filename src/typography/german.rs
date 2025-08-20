use anyhow::Result;

// Unicode constants for better readability
const NBSP: &str = "\u{00A0}";        // Non-breaking space
const NNBSP: &str = "\u{202F}";       // Narrow no-break space
const LDQUO_DE: &str = "\u{201E}";    // German opening double quote „
const RDQUO_DE: &str = "\u{201C}";    // German closing double quote "
const LSQUO_DE: &str = "\u{201A}";    // German opening single quote ‚
const RSQUO_DE: &str = "\u{2018}";    // German closing single quote '
const ELLIPSIS: &str = "\u{2026}";    // Ellipsis …
const ENDASH: &str = "\u{2013}";      // En dash –
const EMDASH: &str = "\u{2014}";      // Em dash —

/// German typography rules following DIN 5008
pub struct GermanRules;

impl GermanRules {
    /// Get all replacement patterns for German typography
    pub fn get_replacements() -> Vec<(&'static str, &'static str)> {
        vec![
            // Number ranges with en dash
            (r"(\d+)\s*-\s*(\d+)", "$1–$2"),
            
            // Common abbreviations with non-breaking space (will be fixed in format_abbreviations)
            ("z. B.", "z. B."),
            ("d. h.", "d. h."),
            ("u. a.", "u. a."),
            ("o. ä.", "o. ä."),
            ("u. U.", "u. U."),
            ("m. E.", "m. E."),
            ("i. d. R.", "i. d. R."),
            ("z. T.", "z. T."),
            ("s. o.", "s. o."),
            ("s. u.", "s. u."),
            
            // Titles with non-breaking space
            ("Dr. ", "Dr. "),
            ("Prof. ", "Prof. "),
            ("Dipl.-Ing. ", "Dipl.-Ing. "),
            
            // Three dots to ellipsis
            ("...", "…"),
            
            // Double hyphen to em dash
            ("--", "—"),
            
            // Single hyphen with spaces to en dash
            (" - ", " – "),
        ]
    }
    
    /// Apply number formatting according to DIN 5008
    pub fn format_numbers(text: &str) -> String {
        let mut result = text.to_string();
        
        // Format thousands with narrow no-break space (DIN 5008)
        // Matches numbers with 4+ digits
        let re = regex::Regex::new(r"\b(\d{1,3})(\d{3})\b").unwrap();
        result = re.replace_all(&result, format!("$1{}$2", NNBSP).as_str()).to_string();
        
        // Format numbers with units (narrow no-break space)
        let units = vec![
            "kg", "g", "mg", "t",           // Weight
            "km", "m", "cm", "mm",          // Length
            "l", "ml", "cl", "dl",          // Volume
            "h", "min", "s", "ms",          // Time
            "°C", "°F", "K",                // Temperature
            "%", "‰",                       // Percentage
            "kW", "MW", "W",                // Power
            "V", "A", "Ω",                  // Electrical
            "bar", "Pa", "hPa",             // Pressure
        ];
        
        for unit in units {
            let pattern = format!(r"(\d+)\s+{}", regex::escape(unit));
            let replacement = format!("$1{}{}", NNBSP, unit);
            if let Ok(re) = regex::Regex::new(&pattern) {
                result = re.replace_all(&result, replacement.as_str()).to_string();
            }
        }
        
        // Format currency (non-breaking space before € and EUR)
        let currency_re = regex::Regex::new(r"(\d+(?:,\d{2})?)\s*(€|EUR)").unwrap();
        result = currency_re.replace_all(&result, format!("$1{}$2", NBSP).as_str()).to_string();
        
        // Format percentages (no space before %)
        let percent_re = regex::Regex::new(r"(\d+)\s+%").unwrap();
        result = percent_re.replace_all(&result, "$1%").to_string();
        
        result
    }
    
    /// Format abbreviations with proper non-breaking spaces
    pub fn format_abbreviations(text: &str) -> String {
        let mut result = text.to_string();
        
        // Common abbreviations - replace space with NBSP
        let abbreviations = vec![
            ("z. B.", format!("z.{}B.", NBSP)),
            ("d. h.", format!("d.{}h.", NBSP)),
            ("u. a.", format!("u.{}a.", NBSP)),
            ("o. ä.", format!("o.{}ä.", NBSP)),
            ("u. U.", format!("u.{}U.", NBSP)),
            ("m. E.", format!("m.{}E.", NBSP)),
            ("i. d. R.", format!("i.{}d.{}R.", NBSP, NBSP)),
            ("z. T.", format!("z.{}T.", NBSP)),
            ("s. o.", format!("s.{}o.", NBSP)),
            ("s. u.", format!("s.{}u.", NBSP)),
            // Titles
            ("Dr. ", format!("Dr.{}", NBSP)),
            ("Prof. ", format!("Prof.{}", NBSP)),
            ("Dipl.-Ing. ", format!("Dipl.-Ing.{}", NBSP)),
        ];
        
        for (pattern, replacement) in abbreviations {
            result = result.replace(pattern, &replacement);
        }
        
        result
    }
    
    /// Apply German quotation marks
    pub fn apply_quotes(text: &str) -> String {
        let mut result = text.to_string();
        
        // Simple approach: replace paired double quotes
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
                            new_result.push_str(LDQUO_DE); // German opening „
                        } else {
                            new_result.push_str(RDQUO_DE); // German closing "
                        }
                    }
                }
                result = new_result;
            }
        }
        
        // Handle single quotes similarly
        if result.contains("'") {
            let parts: Vec<&str> = result.split("'").collect();
            if parts.len() >= 3 {
                let mut new_result = String::new();
                for (i, part) in parts.iter().enumerate() {
                    new_result.push_str(part);
                    if i < parts.len() - 1 {
                        if i % 2 == 0 {
                            new_result.push_str(LSQUO_DE); // German opening ‚
                        } else {
                            new_result.push_str(RSQUO_DE); // German closing '
                        }
                    }
                }
                result = new_result;
            }
        }
        
        result
    }
    
    /// Apply all German typography rules
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
        
        // Apply abbreviations formatting
        result = Self::format_abbreviations(&result);
        
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
    fn test_abbreviations() {
        assert_eq!(
            GermanRules::apply_all("z. B. diese"),
            format!("z.{}B. diese", NBSP)
        );
        assert_eq!(
            GermanRules::apply_all("d. h. das"),
            format!("d.{}h. das", NBSP)
        );
    }
    
    #[test]
    fn test_number_formatting() {
        assert_eq!(
            GermanRules::format_numbers("10000"),
            format!("10{}000", NNBSP)
        );
        assert_eq!(
            GermanRules::format_numbers("10 kg"),
            format!("10{}kg", NNBSP)
        );
        assert_eq!(
            GermanRules::format_numbers("29,99 €"),
            format!("29,99{}€", NBSP)
        );
    }
    
    #[test]
    fn test_quotes() {
        assert_eq!(
            GermanRules::apply_quotes("\"Hallo Welt\""),
            format!("{}Hallo Welt{}", LDQUO_DE, RDQUO_DE)
        );
        assert_eq!(
            GermanRules::apply_quotes("'Hallo'"),
            format!("{}Hallo{}", LSQUO_DE, RSQUO_DE)
        );
    }
}