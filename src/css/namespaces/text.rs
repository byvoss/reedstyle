use anyhow::Result;
use crate::config::{Config, FontsConfig, ColorsConfig};
use crate::css::breakpoints::BREAKPOINTS;

pub struct TextNamespace;

impl TextNamespace {
    /// Generate text namespace CSS for all breakpoints
    pub fn generate(_config: &Config, fonts: &FontsConfig, colors: &ColorsConfig) -> Result<String> {
        let mut css = String::new();
        
        css.push_str("  /* ========== Text Namespace (Responsive) ========== */\n");
        
        // Generate for each breakpoint
        for (suffix, min_width) in BREAKPOINTS {
            let namespace = format!("text{}", suffix);
            let breakpoint_css = Self::generate_for_namespace(&namespace, fonts, colors)?;
            
            // Wrap in media query if needed
            match min_width {
                Some(width) => {
                    css.push_str(&format!("\n  @media (min-width: {}) {{\n", width));
                    css.push_str(&breakpoint_css);
                    css.push_str("  }\n");
                },
                None => {
                    css.push_str(&breakpoint_css);
                }
            }
        }
        
        Ok(css)
    }
    
    /// Generate all text properties for a specific namespace
    fn generate_for_namespace(namespace: &str, fonts: &FontsConfig, _colors: &ColorsConfig) -> Result<String> {
        let mut css = String::new();
        
        css.push_str(&format!("\n    /* {} */\n", namespace));
        
        // Generate all text properties
        css.push_str(&Self::generate_font_family(namespace, fonts));
        css.push_str(&Self::generate_font_size(namespace));
        css.push_str(&Self::generate_font_weight(namespace));
        css.push_str(&Self::generate_text_color(namespace));
        css.push_str(&Self::generate_text_align(namespace));
        css.push_str(&Self::generate_line_height(namespace));
        css.push_str(&Self::generate_letter_spacing(namespace));
        css.push_str(&Self::generate_text_decoration(namespace));
        css.push_str(&Self::generate_text_transform(namespace));
        css.push_str(&Self::generate_text_style(namespace));
        css.push_str(&Self::generate_text_overflow(namespace));
        css.push_str(&Self::generate_whitespace(namespace));
        css.push_str(&Self::generate_word_break(namespace));
        css.push_str(&Self::generate_list_style(namespace));
        
        // Typography features only for base namespace
        if namespace == "text" {
            css.push_str(&Self::generate_typography_features()?);
        }
        
        Ok(css)
    }
    
    fn generate_font_family(namespace: &str, _fonts: &FontsConfig) -> String {
        let mut css = String::new();
        
        // Named fonts from config (font-a through font-f)
        for letter in ['a', 'b', 'c', 'd', 'e', 'f'] {
            let font_name = format!("font-{}", letter);
            css.push_str(&format!("    r-s[{}*=\"font:{}\"] {{ font-family: var(--rs-{}); }}\n", namespace, font_name, font_name));
        }
        
        // Semantic mappings
        css.push_str(&format!("    r-s[{}*=\"font:system\"] {{ font-family: var(--rs-font-a); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"font:heading\"] {{ font-family: var(--rs-font-b); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"font:body\"] {{ font-family: var(--rs-font-a); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"font:code\"] {{ font-family: var(--rs-font-c); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"font:mono\"] {{ font-family: var(--rs-font-c); }}\n", namespace));
        
        css
    }
    
    fn generate_font_size(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"size:tiny\"] {{ font-size: 0.75rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"size:small\"] {{ font-size: 0.875rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"size:normal\"] {{ font-size: 1rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"size:large\"] {{ font-size: 1.25rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"size:huge\"] {{ font-size: 1.5rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"size:mega\"] {{ font-size: 2rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"size:ultra\"] {{ font-size: 3rem; }}\n", namespace));
        
        // Additional sizes for headings
        css.push_str(&format!("    r-s[{}*=\"size:2xl\"] {{ font-size: 1.5rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"size:3xl\"] {{ font-size: 1.875rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"size:4xl\"] {{ font-size: 2.25rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"size:5xl\"] {{ font-size: 3rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"size:6xl\"] {{ font-size: 3.75rem; }}\n", namespace));
        
        css
    }
    
    fn generate_font_weight(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"weight:thin\"] {{ font-weight: 100; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"weight:light\"] {{ font-weight: 300; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"weight:normal\"] {{ font-weight: 400; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"weight:regular\"] {{ font-weight: 400; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"weight:medium\"] {{ font-weight: 500; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"weight:semibold\"] {{ font-weight: 600; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"weight:bold\"] {{ font-weight: 700; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"weight:extrabold\"] {{ font-weight: 800; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"weight:black\"] {{ font-weight: 900; }}\n", namespace));
        
        css
    }
    
    fn generate_text_color(namespace: &str) -> String {
        let mut css = String::new();
        
        // Brand colors
        for letter in ['a', 'b', 'c', 'd', 'e', 'f'] {
            let color_name = format!("brand-{}", letter);
            css.push_str(&format!("    r-s[{}*=\"color:{}\"] {{ color: var(--rs-{}); }}\n", namespace, color_name, color_name));
            
            // 1-9 scale variations
            for scale in 1..=9 {
                css.push_str(&format!("    r-s[{}*=\"color:{}-{}\"] {{ color: var(--rs-color-{}-{}); }}\n", 
                    namespace, color_name, scale, color_name, scale));
            }
        }
        
        // Neutral colors
        for scale in 1..=9 {
            css.push_str(&format!("    r-s[{}*=\"color:neutral-{}\"] {{ color: var(--rs-color-neutral-{}); }}\n", namespace, scale, scale));
        }
        
        // Semantic colors
        for semantic in ["success", "warning", "error", "info"] {
            css.push_str(&format!("    r-s[{}*=\"color:state-{}\"] {{ color: var(--rs-state-{}); }}\n", namespace, semantic, semantic));
        }
        
        // Special
        css.push_str(&format!("    r-s[{}*=\"color:current\"] {{ color: currentColor; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"color:inherit\"] {{ color: inherit; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"color:transparent\"] {{ color: transparent; }}\n", namespace));
        
        css
    }
    
    fn generate_text_align(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"align:left\"] {{ text-align: left; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"align:center\"] {{ text-align: center; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"align:right\"] {{ text-align: right; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"align:justify\"] {{ text-align: justify; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"align:start\"] {{ text-align: start; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"align:end\"] {{ text-align: end; }}\n", namespace));
        
        css
    }
    
    fn generate_line_height(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"leading:none\"] {{ line-height: 1; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"leading:tight\"] {{ line-height: 1.25; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"leading:snug\"] {{ line-height: 1.375; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"leading:normal\"] {{ line-height: 1.5; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"leading:relaxed\"] {{ line-height: 1.625; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"leading:loose\"] {{ line-height: 2; }}\n", namespace));
        
        css
    }
    
    fn generate_letter_spacing(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"tracking:tighter\"] {{ letter-spacing: -0.05em; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"tracking:tight\"] {{ letter-spacing: -0.025em; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"tracking:normal\"] {{ letter-spacing: 0; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"tracking:wide\"] {{ letter-spacing: 0.025em; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"tracking:wider\"] {{ letter-spacing: 0.05em; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"tracking:widest\"] {{ letter-spacing: 0.1em; }}\n", namespace));
        
        css
    }
    
    fn generate_text_decoration(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"decoration:none\"] {{ text-decoration: none; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"decoration:underline\"] {{ text-decoration: underline; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"decoration:overline\"] {{ text-decoration: overline; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"decoration:line-through\"] {{ text-decoration: line-through; }}\n", namespace));
        
        // Decoration style
        css.push_str(&format!("    r-s[{}*=\"decoration:solid\"] {{ text-decoration-style: solid; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"decoration:double\"] {{ text-decoration-style: double; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"decoration:dotted\"] {{ text-decoration-style: dotted; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"decoration:dashed\"] {{ text-decoration-style: dashed; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"decoration:wavy\"] {{ text-decoration-style: wavy; }}\n", namespace));
        
        // Decoration color (using brand colors)
        for letter in ['a', 'b', 'c'] {
            let color = format!("brand-{}", letter);
            css.push_str(&format!("    r-s[{}*=\"decoration:{}\"] {{ text-decoration-color: var(--rs-{}); }}\n", namespace, color, color));
        }
        
        css
    }
    
    fn generate_text_transform(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"transform:none\"] {{ text-transform: none; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"transform:uppercase\"] {{ text-transform: uppercase; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"transform:lowercase\"] {{ text-transform: lowercase; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"transform:capitalize\"] {{ text-transform: capitalize; }}\n", namespace));
        
        css
    }
    
    fn generate_text_style(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"style:normal\"] {{ font-style: normal; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"style:italic\"] {{ font-style: italic; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"style:oblique\"] {{ font-style: oblique; }}\n", namespace));
        
        css
    }
    
    fn generate_text_overflow(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"overflow:clip\"] {{ text-overflow: clip; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"overflow:ellipsis\"] {{ text-overflow: ellipsis; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"truncate\"] {{ overflow: hidden; text-overflow: ellipsis; white-space: nowrap; }}\n", namespace));
        
        css
    }
    
    fn generate_whitespace(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"whitespace:normal\"] {{ white-space: normal; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"whitespace:nowrap\"] {{ white-space: nowrap; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"whitespace:pre\"] {{ white-space: pre; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"whitespace:pre-line\"] {{ white-space: pre-line; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"whitespace:pre-wrap\"] {{ white-space: pre-wrap; }}\n", namespace));
        
        css
    }
    
    fn generate_word_break(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"break:normal\"] {{ word-break: normal; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"break:words\"] {{ word-break: break-word; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"break:all\"] {{ word-break: break-all; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"break:keep\"] {{ word-break: keep-all; }}\n", namespace));
        
        css
    }
    
    fn generate_list_style(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"list:none\"] {{ list-style: none; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"list:disc\"] {{ list-style-type: disc; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"list:circle\"] {{ list-style-type: circle; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"list:square\"] {{ list-style-type: square; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"list:decimal\"] {{ list-style-type: decimal; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"list:roman\"] {{ list-style-type: lower-roman; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"list:alpha\"] {{ list-style-type: lower-alpha; }}\n", namespace));
        
        css
    }
    
    fn generate_typography_features() -> Result<String> {
        // Use the typography module to generate CSS
        crate::typography::generate_css()
    }
}