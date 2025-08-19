use anyhow::Result;
use crate::config::Config;

pub struct BoxNamespace;

impl BoxNamespace {
    pub fn generate(config: &Config) -> Result<String> {
        let mut css = String::new();
        
        css.push_str("  /* ========== Box Namespace ========== */\n");
        
        // Padding - all directions
        css.push_str(&Self::generate_padding());
        
        // Margin - all directions
        css.push_str(&Self::generate_margin());
        
        // Width
        css.push_str(&Self::generate_width());
        
        // Height
        css.push_str(&Self::generate_height());
        
        // Display
        css.push_str(&Self::generate_display());
        
        // Position
        css.push_str(&Self::generate_position());
        
        // Overflow
        css.push_str(&Self::generate_overflow());
        
        // Box sizing
        css.push_str(&Self::generate_box_sizing());
        
        // Aspect ratio
        css.push_str(&Self::generate_aspect_ratio());
        
        Ok(css)
    }
    
    fn generate_padding() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Padding */\n");
        
        // Standard scale: 0, 1, 2, 3, 4, 6, 8, 10, 12, 16, 20, 24
        let scale = [
            ("0", "0"),
            ("1", "0.25rem"),
            ("2", "0.5rem"),
            ("3", "0.75rem"),
            ("4", "1rem"),
            ("6", "1.5rem"),
            ("8", "2rem"),
            ("10", "2.5rem"),
            ("12", "3rem"),
            ("16", "4rem"),
            ("20", "5rem"),
            ("24", "6rem"),
        ];
        
        for (key, value) in &scale {
            // All sides
            css.push_str(&format!("  reed[box*=\"padding:{}\"] {{ padding: {}; }}\n", key, value));
            
            // Individual sides
            css.push_str(&format!("  reed[box*=\"padding-top:{}\"] {{ padding-top: {}; }}\n", key, value));
            css.push_str(&format!("  reed[box*=\"padding-right:{}\"] {{ padding-right: {}; }}\n", key, value));
            css.push_str(&format!("  reed[box*=\"padding-bottom:{}\"] {{ padding-bottom: {}; }}\n", key, value));
            css.push_str(&format!("  reed[box*=\"padding-left:{}\"] {{ padding-left: {}; }}\n", key, value));
            
            // Axis shortcuts
            css.push_str(&format!("  reed[box*=\"padding-x:{}\"] {{ padding-left: {}; padding-right: {}; }}\n", key, value, value));
            css.push_str(&format!("  reed[box*=\"padding-y:{}\"] {{ padding-top: {}; padding-bottom: {}; }}\n", key, value, value));
        }
        
        css
    }
    
    fn generate_margin() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Margin */\n");
        
        let scale = [
            ("0", "0"),
            ("1", "0.25rem"),
            ("2", "0.5rem"),
            ("3", "0.75rem"),
            ("4", "1rem"),
            ("6", "1.5rem"),
            ("8", "2rem"),
            ("10", "2.5rem"),
            ("12", "3rem"),
            ("16", "4rem"),
            ("20", "5rem"),
            ("24", "6rem"),
        ];
        
        // Auto margin
        css.push_str("  reed[box*=\"margin:auto\"] { margin: auto; }\n");
        css.push_str("  reed[box*=\"margin-x:auto\"] { margin-left: auto; margin-right: auto; }\n");
        css.push_str("  reed[box*=\"margin-y:auto\"] { margin-top: auto; margin-bottom: auto; }\n");
        css.push_str("  reed[box*=\"margin-left:auto\"] { margin-left: auto; }\n");
        css.push_str("  reed[box*=\"margin-right:auto\"] { margin-right: auto; }\n");
        
        for (key, value) in &scale {
            // All sides
            css.push_str(&format!("  reed[box*=\"margin:{}\"] {{ margin: {}; }}\n", key, value));
            
            // Individual sides
            css.push_str(&format!("  reed[box*=\"margin-top:{}\"] {{ margin-top: {}; }}\n", key, value));
            css.push_str(&format!("  reed[box*=\"margin-right:{}\"] {{ margin-right: {}; }}\n", key, value));
            css.push_str(&format!("  reed[box*=\"margin-bottom:{}\"] {{ margin-bottom: {}; }}\n", key, value));
            css.push_str(&format!("  reed[box*=\"margin-left:{}\"] {{ margin-left: {}; }}\n", key, value));
            
            // Axis shortcuts
            css.push_str(&format!("  reed[box*=\"margin-x:{}\"] {{ margin-left: {}; margin-right: {}; }}\n", key, value, value));
            css.push_str(&format!("  reed[box*=\"margin-y:{}\"] {{ margin-top: {}; margin-bottom: {}; }}\n", key, value, value));
            
            // Negative margins
            if key != &"0" {
                css.push_str(&format!("  reed[box*=\"margin:-{}\"] {{ margin: -{}; }}\n", key, value));
                css.push_str(&format!("  reed[box*=\"margin-top:-{}\"] {{ margin-top: -{}; }}\n", key, value));
                css.push_str(&format!("  reed[box*=\"margin-right:-{}\"] {{ margin-right: -{}; }}\n", key, value));
                css.push_str(&format!("  reed[box*=\"margin-bottom:-{}\"] {{ margin-bottom: -{}; }}\n", key, value));
                css.push_str(&format!("  reed[box*=\"margin-left:-{}\"] {{ margin-left: -{}; }}\n", key, value));
            }
        }
        
        css
    }
    
    fn generate_width() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Width */\n");
        
        // Keywords
        css.push_str("  reed[box*=\"width:full\"] { width: 100%; }\n");
        css.push_str("  reed[box*=\"width:half\"] { width: 50%; }\n");
        css.push_str("  reed[box*=\"width:third\"] { width: 33.333333%; }\n");
        css.push_str("  reed[box*=\"width:quarter\"] { width: 25%; }\n");
        css.push_str("  reed[box*=\"width:screen\"] { width: 100vw; }\n");
        css.push_str("  reed[box*=\"width:auto\"] { width: auto; }\n");
        css.push_str("  reed[box*=\"width:min\"] { width: min-content; }\n");
        css.push_str("  reed[box*=\"width:max\"] { width: max-content; }\n");
        css.push_str("  reed[box*=\"width:fit\"] { width: fit-content; }\n");
        
        // Numeric (rem)
        for size in [32, 48, 64, 80, 96] {
            css.push_str(&format!("  reed[box*=\"width:{}\"] {{ width: {}rem; }}\n", size, size));
        }
        
        // Min/Max
        for size in [320, 480, 640, 768, 960, 1200, 1440] {
            css.push_str(&format!("  reed[box*=\"min-width:{}\"] {{ min-width: {}px; }}\n", size, size));
            css.push_str(&format!("  reed[box*=\"max-width:{}\"] {{ max-width: {}px; }}\n", size, size));
        }
        
        css
    }
    
    fn generate_height() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Height */\n");
        
        // Keywords
        css.push_str("  reed[box*=\"height:full\"] { height: 100%; }\n");
        css.push_str("  reed[box*=\"height:screen\"] { height: 100vh; }\n");
        css.push_str("  reed[box*=\"height:auto\"] { height: auto; }\n");
        css.push_str("  reed[box*=\"height:min\"] { height: min-content; }\n");
        css.push_str("  reed[box*=\"height:max\"] { height: max-content; }\n");
        
        // Numeric (rem)
        for size in [16, 24, 32, 48, 64] {
            css.push_str(&format!("  reed[box*=\"height:{}\"] {{ height: {}rem; }}\n", size, size));
        }
        
        // Min/Max
        for size in [100, 200, 300, 400, 500, 600] {
            css.push_str(&format!("  reed[box*=\"min-height:{}\"] {{ min-height: {}px; }}\n", size, size));
            css.push_str(&format!("  reed[box*=\"max-height:{}\"] {{ max-height: {}px; }}\n", size, size));
        }
        
        css
    }
    
    fn generate_display() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Display */\n");
        
        css.push_str("  reed[box*=\"display:block\"] { display: block; }\n");
        css.push_str("  reed[box*=\"display:inline\"] { display: inline; }\n");
        css.push_str("  reed[box*=\"display:inline-block\"] { display: inline-block; }\n");
        css.push_str("  reed[box*=\"display:none\"] { display: none; }\n");
        css.push_str("  reed[box*=\"display:contents\"] { display: contents; }\n");
        
        css
    }
    
    fn generate_position() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Position */\n");
        
        css.push_str("  reed[box*=\"position:static\"] { position: static; }\n");
        css.push_str("  reed[box*=\"position:relative\"] { position: relative; }\n");
        css.push_str("  reed[box*=\"position:absolute\"] { position: absolute; }\n");
        css.push_str("  reed[box*=\"position:fixed\"] { position: fixed; }\n");
        css.push_str("  reed[box*=\"position:sticky\"] { position: sticky; }\n");
        
        css
    }
    
    fn generate_overflow() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Overflow */\n");
        
        css.push_str("  reed[box*=\"overflow:hidden\"] { overflow: hidden; }\n");
        css.push_str("  reed[box*=\"overflow:auto\"] { overflow: auto; }\n");
        css.push_str("  reed[box*=\"overflow:scroll\"] { overflow: scroll; }\n");
        css.push_str("  reed[box*=\"overflow:visible\"] { overflow: visible; }\n");
        css.push_str("  reed[box*=\"overflow:clip\"] { overflow: clip; }\n");
        
        css.push_str("  reed[box*=\"overflow-x:hidden\"] { overflow-x: hidden; }\n");
        css.push_str("  reed[box*=\"overflow-x:auto\"] { overflow-x: auto; }\n");
        css.push_str("  reed[box*=\"overflow-x:scroll\"] { overflow-x: scroll; }\n");
        css.push_str("  reed[box*=\"overflow-x:visible\"] { overflow-x: visible; }\n");
        
        css.push_str("  reed[box*=\"overflow-y:hidden\"] { overflow-y: hidden; }\n");
        css.push_str("  reed[box*=\"overflow-y:auto\"] { overflow-y: auto; }\n");
        css.push_str("  reed[box*=\"overflow-y:scroll\"] { overflow-y: scroll; }\n");
        css.push_str("  reed[box*=\"overflow-y:visible\"] { overflow-y: visible; }\n");
        
        css
    }
    
    fn generate_box_sizing() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Box Sizing */\n");
        
        css.push_str("  reed[box*=\"sizing:border\"] { box-sizing: border-box; }\n");
        css.push_str("  reed[box*=\"sizing:content\"] { box-sizing: content-box; }\n");
        
        css
    }
    
    fn generate_aspect_ratio() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Aspect Ratio */\n");
        
        css.push_str("  reed[box*=\"aspect:square\"] { aspect-ratio: 1 / 1; }\n");
        css.push_str("  reed[box*=\"aspect:video\"] { aspect-ratio: 16 / 9; }\n");
        css.push_str("  reed[box*=\"aspect:photo\"] { aspect-ratio: 4 / 3; }\n");
        css.push_str("  reed[box*=\"aspect:wide\"] { aspect-ratio: 21 / 9; }\n");
        css.push_str("  reed[box*=\"aspect:portrait\"] { aspect-ratio: 3 / 4; }\n");
        
        css
    }
    
    pub fn generate_responsive(breakpoint: &str, min_width: &str) -> Result<String> {
        let mut css = String::new();
        
        css.push_str(&format!("\n  @media (min-width: {}) {{\n", min_width));
        css.push_str(&format!("    /* Box namespace - {} */\n", breakpoint));
        
        // Padding responsive
        let scale = [
            ("0", "0"),
            ("2", "0.5rem"),
            ("4", "1rem"),
            ("6", "1.5rem"),
            ("8", "2rem"),
            ("12", "3rem"),
            ("16", "4rem"),
        ];
        
        for (key, value) in &scale {
            css.push_str(&format!("    reed[box-{}*=\"padding:{}\"] {{ padding: {}; }}\n", breakpoint, key, value));
            css.push_str(&format!("    reed[box-{}*=\"padding-x:{}\"] {{ padding-left: {}; padding-right: {}; }}\n", breakpoint, key, value, value));
            css.push_str(&format!("    reed[box-{}*=\"padding-y:{}\"] {{ padding-top: {}; padding-bottom: {}; }}\n", breakpoint, key, value, value));
        }
        
        // Display responsive
        css.push_str(&format!("    reed[box-{}*=\"display:block\"] {{ display: block; }}\n", breakpoint));
        css.push_str(&format!("    reed[box-{}*=\"display:none\"] {{ display: none; }}\n", breakpoint));
        css.push_str(&format!("    reed[box-{}*=\"display:flex\"] {{ display: flex; }}\n", breakpoint));
        css.push_str(&format!("    reed[box-{}*=\"display:grid\"] {{ display: grid; }}\n", breakpoint));
        
        css.push_str("  }\n");
        
        Ok(css)
    }
}