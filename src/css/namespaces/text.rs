use anyhow::Result;
use crate::config::{Config, FontsConfig, ColorsConfig};

pub struct TextNamespace;

impl TextNamespace {
    pub fn generate(_config: &Config, fonts: &FontsConfig, colors: &ColorsConfig) -> Result<String> {
        let mut css = String::new();
        
        css.push_str("  /* ========== Text Namespace ========== */\n");
        
        // Font family
        css.push_str(&Self::generate_font_family(fonts));
        
        // Font size
        css.push_str(&Self::generate_font_size());
        
        // Font weight
        css.push_str(&Self::generate_font_weight());
        
        // Text color
        css.push_str(&Self::generate_text_color());
        
        // Text alignment
        css.push_str(&Self::generate_text_align());
        
        // Line height
        css.push_str(&Self::generate_line_height());
        
        // Letter spacing
        css.push_str(&Self::generate_letter_spacing());
        
        // Text decoration
        css.push_str(&Self::generate_text_decoration());
        
        // Text transform
        css.push_str(&Self::generate_text_transform());
        
        // Text style
        css.push_str(&Self::generate_text_style());
        
        // Text overflow
        css.push_str(&Self::generate_text_overflow());
        
        // Whitespace
        css.push_str(&Self::generate_whitespace());
        
        // Word break
        css.push_str(&Self::generate_word_break());
        
        // List style
        css.push_str(&Self::generate_list_style());
        
        // Typography features (OpenType, hyphenation, etc.)
        css.push_str(&Self::generate_typography_features()?);
        
        Ok(css)
    }
    
    fn generate_font_family(fonts: &FontsConfig) -> String {
        let mut css = String::new();
        css.push_str("\n  /* Font Family */\n");
        
        // Named fonts from config (font-a through font-f)
        for letter in ['a', 'b', 'c', 'd', 'e', 'f'] {
            let font_name = format!("font-{}", letter);
            css.push_str(&format!("  reed[text*=\"font:{}\"] {{ font-family: var(--rs-{}); }}\n", font_name, font_name));
        }
        
        // Semantic mappings
        css.push_str("  reed[text*=\"font:system\"] { font-family: var(--rs-font-a); }\n");
        css.push_str("  reed[text*=\"font:heading\"] { font-family: var(--rs-font-b); }\n");
        css.push_str("  reed[text*=\"font:body\"] { font-family: var(--rs-font-a); }\n");
        css.push_str("  reed[text*=\"font:code\"] { font-family: var(--rs-font-c); }\n");
        css.push_str("  reed[text*=\"font:mono\"] { font-family: var(--rs-font-c); }\n");
        
        css
    }
    
    fn generate_font_size() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Font Size (Dimension Scale) */\n");
        
        css.push_str("  reed[text*=\"size:tiny\"] { font-size: 0.75rem; }\n");
        css.push_str("  reed[text*=\"size:small\"] { font-size: 0.875rem; }\n");
        css.push_str("  reed[text*=\"size:normal\"] { font-size: 1rem; }\n");
        css.push_str("  reed[text*=\"size:large\"] { font-size: 1.25rem; }\n");
        css.push_str("  reed[text*=\"size:huge\"] { font-size: 1.5rem; }\n");
        css.push_str("  reed[text*=\"size:mega\"] { font-size: 2rem; }\n");
        css.push_str("  reed[text*=\"size:ultra\"] { font-size: 3rem; }\n");
        
        // Additional sizes for headings
        css.push_str("  reed[text*=\"size:2xl\"] { font-size: 1.5rem; }\n");
        css.push_str("  reed[text*=\"size:3xl\"] { font-size: 1.875rem; }\n");
        css.push_str("  reed[text*=\"size:4xl\"] { font-size: 2.25rem; }\n");
        css.push_str("  reed[text*=\"size:5xl\"] { font-size: 3rem; }\n");
        css.push_str("  reed[text*=\"size:6xl\"] { font-size: 3.75rem; }\n");
        
        css
    }
    
    fn generate_font_weight() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Font Weight (Typography Scale) */\n");
        
        css.push_str("  reed[text*=\"weight:thin\"] { font-weight: 100; }\n");
        css.push_str("  reed[text*=\"weight:light\"] { font-weight: 300; }\n");
        css.push_str("  reed[text*=\"weight:normal\"] { font-weight: 400; }\n");
        css.push_str("  reed[text*=\"weight:regular\"] { font-weight: 400; }\n");
        css.push_str("  reed[text*=\"weight:medium\"] { font-weight: 500; }\n");
        css.push_str("  reed[text*=\"weight:semibold\"] { font-weight: 600; }\n");
        css.push_str("  reed[text*=\"weight:bold\"] { font-weight: 700; }\n");
        css.push_str("  reed[text*=\"weight:extrabold\"] { font-weight: 800; }\n");
        css.push_str("  reed[text*=\"weight:black\"] { font-weight: 900; }\n");
        
        css
    }
    
    fn generate_text_color() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Text Color */\n");
        
        // Brand colors
        for letter in ['a', 'b', 'c', 'd', 'e', 'f'] {
            let color_name = format!("brand-{}", letter);
            css.push_str(&format!("  reed[text*=\"color:{}\"] {{ color: var(--rs-{}); }}\n", color_name, color_name));
            
            // Visual scope variations
            for variant in ["weak", "light", "intense", "bright", "strong"] {
                css.push_str(&format!("  reed[text*=\"color:{}-{}\"] {{ color: var(--rs-{}-{}); }}\n", 
                    color_name, variant, color_name, variant));
            }
        }
        
        // Base colors (neutrals)
        for value in [0, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000] {
            css.push_str(&format!("  reed[text*=\"color:base-{}\"] {{ color: var(--rs-base-{}); }}\n", value, value));
        }
        
        // Semantic colors
        for semantic in ["success", "warning", "error", "info"] {
            css.push_str(&format!("  reed[text*=\"color:state-{}\"] {{ color: var(--rs-state-{}); }}\n", semantic, semantic));
        }
        
        // Special
        css.push_str("  reed[text*=\"color:current\"] { color: currentColor; }\n");
        css.push_str("  reed[text*=\"color:inherit\"] { color: inherit; }\n");
        css.push_str("  reed[text*=\"color:transparent\"] { color: transparent; }\n");
        
        css
    }
    
    fn generate_text_align() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Text Alignment */\n");
        
        css.push_str("  reed[text*=\"align:left\"] { text-align: left; }\n");
        css.push_str("  reed[text*=\"align:center\"] { text-align: center; }\n");
        css.push_str("  reed[text*=\"align:right\"] { text-align: right; }\n");
        css.push_str("  reed[text*=\"align:justify\"] { text-align: justify; }\n");
        css.push_str("  reed[text*=\"align:start\"] { text-align: start; }\n");
        css.push_str("  reed[text*=\"align:end\"] { text-align: end; }\n");
        
        css
    }
    
    fn generate_line_height() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Line Height */\n");
        
        css.push_str("  reed[text*=\"leading:none\"] { line-height: 1; }\n");
        css.push_str("  reed[text*=\"leading:tight\"] { line-height: 1.25; }\n");
        css.push_str("  reed[text*=\"leading:snug\"] { line-height: 1.375; }\n");
        css.push_str("  reed[text*=\"leading:normal\"] { line-height: 1.5; }\n");
        css.push_str("  reed[text*=\"leading:relaxed\"] { line-height: 1.625; }\n");
        css.push_str("  reed[text*=\"leading:loose\"] { line-height: 2; }\n");
        
        css
    }
    
    fn generate_letter_spacing() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Letter Spacing */\n");
        
        css.push_str("  reed[text*=\"tracking:tighter\"] { letter-spacing: -0.05em; }\n");
        css.push_str("  reed[text*=\"tracking:tight\"] { letter-spacing: -0.025em; }\n");
        css.push_str("  reed[text*=\"tracking:normal\"] { letter-spacing: 0; }\n");
        css.push_str("  reed[text*=\"tracking:wide\"] { letter-spacing: 0.025em; }\n");
        css.push_str("  reed[text*=\"tracking:wider\"] { letter-spacing: 0.05em; }\n");
        css.push_str("  reed[text*=\"tracking:widest\"] { letter-spacing: 0.1em; }\n");
        
        css
    }
    
    fn generate_text_decoration() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Text Decoration */\n");
        
        css.push_str("  reed[text*=\"decoration:none\"] { text-decoration: none; }\n");
        css.push_str("  reed[text*=\"decoration:underline\"] { text-decoration: underline; }\n");
        css.push_str("  reed[text*=\"decoration:overline\"] { text-decoration: overline; }\n");
        css.push_str("  reed[text*=\"decoration:line-through\"] { text-decoration: line-through; }\n");
        
        // Decoration style
        css.push_str("  reed[text*=\"decoration:solid\"] { text-decoration-style: solid; }\n");
        css.push_str("  reed[text*=\"decoration:double\"] { text-decoration-style: double; }\n");
        css.push_str("  reed[text*=\"decoration:dotted\"] { text-decoration-style: dotted; }\n");
        css.push_str("  reed[text*=\"decoration:dashed\"] { text-decoration-style: dashed; }\n");
        css.push_str("  reed[text*=\"decoration:wavy\"] { text-decoration-style: wavy; }\n");
        
        // Decoration color (using brand colors)
        for letter in ['a', 'b', 'c'] {
            let color = format!("brand-{}", letter);
            css.push_str(&format!("  reed[text*=\"decoration:{}\"] {{ text-decoration-color: var(--rs-{}); }}\n", color, color));
        }
        
        css
    }
    
    fn generate_text_transform() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Text Transform */\n");
        
        css.push_str("  reed[text*=\"transform:none\"] { text-transform: none; }\n");
        css.push_str("  reed[text*=\"transform:uppercase\"] { text-transform: uppercase; }\n");
        css.push_str("  reed[text*=\"transform:lowercase\"] { text-transform: lowercase; }\n");
        css.push_str("  reed[text*=\"transform:capitalize\"] { text-transform: capitalize; }\n");
        
        css
    }
    
    fn generate_text_style() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Text Style */\n");
        
        css.push_str("  reed[text*=\"style:normal\"] { font-style: normal; }\n");
        css.push_str("  reed[text*=\"style:italic\"] { font-style: italic; }\n");
        css.push_str("  reed[text*=\"style:oblique\"] { font-style: oblique; }\n");
        
        css
    }
    
    fn generate_text_overflow() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Text Overflow */\n");
        
        css.push_str("  reed[text*=\"overflow:clip\"] { text-overflow: clip; }\n");
        css.push_str("  reed[text*=\"overflow:ellipsis\"] { text-overflow: ellipsis; }\n");
        css.push_str("  reed[text*=\"truncate\"] { overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }\n");
        
        css
    }
    
    fn generate_whitespace() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Whitespace */\n");
        
        css.push_str("  reed[text*=\"whitespace:normal\"] { white-space: normal; }\n");
        css.push_str("  reed[text*=\"whitespace:nowrap\"] { white-space: nowrap; }\n");
        css.push_str("  reed[text*=\"whitespace:pre\"] { white-space: pre; }\n");
        css.push_str("  reed[text*=\"whitespace:pre-line\"] { white-space: pre-line; }\n");
        css.push_str("  reed[text*=\"whitespace:pre-wrap\"] { white-space: pre-wrap; }\n");
        
        css
    }
    
    fn generate_word_break() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Word Break */\n");
        
        css.push_str("  reed[text*=\"break:normal\"] { word-break: normal; }\n");
        css.push_str("  reed[text*=\"break:words\"] { word-break: break-word; }\n");
        css.push_str("  reed[text*=\"break:all\"] { word-break: break-all; }\n");
        css.push_str("  reed[text*=\"break:keep\"] { word-break: keep-all; }\n");
        
        css
    }
    
    fn generate_list_style() -> String {
        let mut css = String::new();
        css.push_str("\n  /* List Style */\n");
        
        css.push_str("  reed[text*=\"list:none\"] { list-style: none; }\n");
        css.push_str("  reed[text*=\"list:disc\"] { list-style-type: disc; }\n");
        css.push_str("  reed[text*=\"list:circle\"] { list-style-type: circle; }\n");
        css.push_str("  reed[text*=\"list:square\"] { list-style-type: square; }\n");
        css.push_str("  reed[text*=\"list:decimal\"] { list-style-type: decimal; }\n");
        css.push_str("  reed[text*=\"list:roman\"] { list-style-type: lower-roman; }\n");
        css.push_str("  reed[text*=\"list:alpha\"] { list-style-type: lower-alpha; }\n");
        
        css
    }
    
    pub fn generate_responsive(breakpoint: &str, min_width: &str) -> Result<String> {
        let mut css = String::new();
        
        css.push_str(&format!("\n  @media (min-width: {}) {{\n", min_width));
        css.push_str(&format!("    /* Text namespace - {} */\n", breakpoint));
        
        // Font size responsive
        css.push_str(&format!("    reed[text-{}*=\"size:small\"] {{ font-size: 0.875rem; }}\n", breakpoint));
        css.push_str(&format!("    reed[text-{}*=\"size:normal\"] {{ font-size: 1rem; }}\n", breakpoint));
        css.push_str(&format!("    reed[text-{}*=\"size:large\"] {{ font-size: 1.25rem; }}\n", breakpoint));
        css.push_str(&format!("    reed[text-{}*=\"size:huge\"] {{ font-size: 1.5rem; }}\n", breakpoint));
        css.push_str(&format!("    reed[text-{}*=\"size:mega\"] {{ font-size: 2rem; }}\n", breakpoint));
        css.push_str(&format!("    reed[text-{}*=\"size:ultra\"] {{ font-size: 3rem; }}\n", breakpoint));
        css.push_str(&format!("    reed[text-{}*=\"size:3xl\"] {{ font-size: 1.875rem; }}\n", breakpoint));
        css.push_str(&format!("    reed[text-{}*=\"size:4xl\"] {{ font-size: 2.25rem; }}\n", breakpoint));
        css.push_str(&format!("    reed[text-{}*=\"size:5xl\"] {{ font-size: 3rem; }}\n", breakpoint));
        
        // Text align responsive
        css.push_str(&format!("    reed[text-{}*=\"align:left\"] {{ text-align: left; }}\n", breakpoint));
        css.push_str(&format!("    reed[text-{}*=\"align:center\"] {{ text-align: center; }}\n", breakpoint));
        css.push_str(&format!("    reed[text-{}*=\"align:right\"] {{ text-align: right; }}\n", breakpoint));
        
        // Font weight responsive
        css.push_str(&format!("    reed[text-{}*=\"weight:normal\"] {{ font-weight: 400; }}\n", breakpoint));
        css.push_str(&format!("    reed[text-{}*=\"weight:bold\"] {{ font-weight: 700; }}\n", breakpoint));
        
        css.push_str("  }\n");
        
        Ok(css)
    }
    
    fn generate_typography_features() -> Result<String> {
        // Use the typography module to generate CSS
        crate::typography::generate_css()
    }
}