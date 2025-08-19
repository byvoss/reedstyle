pub mod german;
pub mod english;  
pub mod french;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyConfig {
    pub default_filter: String,
    pub features: TypographyFeatures,
    pub languages: HashMap<String, LanguageRules>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyFeatures {
    pub ligatures: bool,
    pub kerning: bool,
    pub hyphenation: String,
    pub hanging_punctuation: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageRules {
    pub quotes: (String, String),
    pub single_quotes: (String, String),
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacements: Option<Vec<(String, String)>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space_rules: Option<SpaceRules>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpaceRules {
    pub before_unit: String,
    pub in_number: String,
    pub before_currency: String,
    pub after_abbr: String,
}

impl Default for TypographyConfig {
    fn default() -> Self {
        let mut languages = HashMap::new();
        
        // German rules (DIN 5008)
        let mut de = LanguageRules {
            quotes: ("\u{201E}".to_string(), "\u{201C}".to_string()),  // „ and "
            single_quotes: ("\u{201A}".to_string(), "\u{2018}".to_string()),  // ‚ and '
            replacements: Some(vec![
                (r"(\d+)-(\d+)".to_string(), r"$1\u{2013}$2".to_string()),  // en dash
            ]),
            space_rules: Some(SpaceRules {
                before_unit: "\u{202F}".to_string(),     // Narrow no-break space
                in_number: "\u{202F}".to_string(),       // Narrow no-break space
                before_currency: "\u{00A0}".to_string(), // Non-breaking space
                after_abbr: "\u{00A0}".to_string(),      // Non-breaking space
            }),
        };
        languages.insert("de".to_string(), de);
        
        // English US rules
        let en_us = LanguageRules {
            quotes: ("\u{201C}".to_string(), "\u{201D}".to_string()),  // " and "
            single_quotes: ("\u{2018}".to_string(), "\u{2019}".to_string()),  // ' and '
            replacements: None,
            space_rules: None,
        };
        languages.insert("en-US".to_string(), en_us);
        
        // English GB rules
        let en_gb = LanguageRules {
            quotes: ("\u{2018}".to_string(), "\u{2019}".to_string()),  // ' and '
            single_quotes: ("\u{201C}".to_string(), "\u{201D}".to_string()),  // " and "
            replacements: None,
            space_rules: None,
        };
        languages.insert("en-GB".to_string(), en_gb);
        
        // French rules
        let fr = LanguageRules {
            quotes: ("\u{00AB} ".to_string(), " \u{00BB}".to_string()),  // « and »
            single_quotes: ("\u{201C}".to_string(), "\u{201D}".to_string()),  // " and "
            replacements: None,
            space_rules: Some(SpaceRules {
                before_unit: "\u{00A0}".to_string(),
                in_number: "\u{00A0}".to_string(),
                before_currency: "\u{00A0}".to_string(),
                after_abbr: "\u{00A0}".to_string(),
            }),
        };
        languages.insert("fr".to_string(), fr);
        
        TypographyConfig {
            default_filter: "smart".to_string(),
            features: TypographyFeatures {
                ligatures: true,
                kerning: true,
                hyphenation: "auto".to_string(),
                hanging_punctuation: false,
            },
            languages,
        }
    }
}

/// Generate CSS for OpenType features and typography settings
pub fn generate_css() -> Result<String> {
    let mut css = String::new();
    
    css.push_str("  /* ========== Typography Features ========== */\n");
    
    // OpenType features
    css.push_str(r#"  reed[text*="ligatures:true"] {
    font-variant-ligatures: common-ligatures;
    font-feature-settings: "liga" 1, "clig" 1;
  }

"#);
    
    css.push_str(r#"  reed[text*="small-caps:true"] {
    font-variant-caps: small-caps;
    font-feature-settings: "smcp" 1;
  }

"#);
    
    css.push_str(r#"  reed[text*="numbers:tabular"] {
    font-variant-numeric: tabular-nums;
    font-feature-settings: "tnum" 1;
  }

"#);
    
    css.push_str(r#"  reed[text*="numbers:oldstyle"] {
    font-variant-numeric: oldstyle-nums;
    font-feature-settings: "onum" 1;
  }

"#);
    
    // Hyphenation
    css.push_str(r#"  reed[text*="hyphenate:true"] {
    hyphens: auto;
    -webkit-hyphens: auto;
    hyphenate-limit-lines: 2;
    hyphenate-limit-last: always;
  }

"#);
    
    // Hanging punctuation
    css.push_str(r#"  reed[text*="hanging-punctuation:true"] {
    hanging-punctuation: first last;
  }

"#);
    
    // Line height (leading)
    css.push_str(r#"  reed[text*="leading:tight"] { line-height: 1.25; }
  reed[text*="leading:normal"] { line-height: 1.5; }
  reed[text*="leading:relaxed"] { line-height: 1.75; }
  reed[text*="leading:loose"] { line-height: 2; }

"#);
    
    // Measure (line length)
    css.push_str(r#"  reed[text*="measure:narrow"] { max-width: 45ch; }
  reed[text*="measure:normal"] { max-width: 65ch; }
  reed[text*="measure:wide"] { max-width: 85ch; }

"#);
    
    // Text filters (placeholder for JS processing)
    css.push_str(r#"  reed[text*="filter:minimal"] { /* Processed by JS */ }
  reed[text*="filter:smart"] { /* Processed by JS */ }
  reed[text*="filter:professional"] { /* Processed by JS */ }

"#);
    
    Ok(css)
}