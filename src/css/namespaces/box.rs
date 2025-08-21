use anyhow::Result;
use crate::config::Config;
use crate::css::breakpoints::BREAKPOINTS;

pub struct BoxNamespace;

impl BoxNamespace {
    /// Generate box namespace CSS for all breakpoints
    pub fn generate(_config: &Config) -> Result<String> {
        let mut css = String::new();
        
        css.push_str("  /* ========== Box Namespace (Responsive) ========== */\n");
        
        // Generate for each breakpoint
        for (suffix, min_width) in BREAKPOINTS {
            let namespace = format!("box{}", suffix);
            let breakpoint_css = Self::generate_for_namespace(&namespace);
            
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
    
    /// Generate all box properties for a specific namespace
    fn generate_for_namespace(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("\n    /* {} */\n", namespace));
        
        // Generate all box properties
        css.push_str(&Self::generate_padding(namespace));
        css.push_str(&Self::generate_margin(namespace));
        css.push_str(&Self::generate_width(namespace));
        css.push_str(&Self::generate_height(namespace));
        css.push_str(&Self::generate_display(namespace));
        css.push_str(&Self::generate_position(namespace));
        css.push_str(&Self::generate_overflow(namespace));
        css.push_str(&Self::generate_box_sizing(namespace));
        css.push_str(&Self::generate_aspect_ratio(namespace));
        
        css
    }
    
    fn generate_padding(namespace: &str) -> String {
        let mut css = String::new();
        
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
            css.push_str(&format!("    r-s[{}*=\"padding:{}\"] {{ padding: {}; }}\n", namespace, key, value));
            
            // Individual sides
            css.push_str(&format!("    r-s[{}*=\"padding-top:{}\"] {{ padding-top: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"padding-right:{}\"] {{ padding-right: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"padding-bottom:{}\"] {{ padding-bottom: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"padding-left:{}\"] {{ padding-left: {}; }}\n", namespace, key, value));
            
            // Axis shortcuts
            css.push_str(&format!("    r-s[{}*=\"padding-x:{}\"] {{ padding-left: {}; padding-right: {}; }}\n", namespace, key, value, value));
            css.push_str(&format!("    r-s[{}*=\"padding-y:{}\"] {{ padding-top: {}; padding-bottom: {}; }}\n", namespace, key, value, value));
        }
        
        css
    }
    
    fn generate_margin(namespace: &str) -> String {
        let mut css = String::new();
        
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
        css.push_str(&format!("    r-s[{}*=\"margin:auto\"] {{ margin: auto; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"margin-x:auto\"] {{ margin-left: auto; margin-right: auto; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"margin-y:auto\"] {{ margin-top: auto; margin-bottom: auto; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"margin-left:auto\"] {{ margin-left: auto; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"margin-right:auto\"] {{ margin-right: auto; }}\n", namespace));
        
        for (key, value) in &scale {
            // All sides
            css.push_str(&format!("    r-s[{}*=\"margin:{}\"] {{ margin: {}; }}\n", namespace, key, value));
            
            // Individual sides
            css.push_str(&format!("    r-s[{}*=\"margin-top:{}\"] {{ margin-top: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"margin-right:{}\"] {{ margin-right: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"margin-bottom:{}\"] {{ margin-bottom: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"margin-left:{}\"] {{ margin-left: {}; }}\n", namespace, key, value));
            
            // Axis shortcuts
            css.push_str(&format!("    r-s[{}*=\"margin-x:{}\"] {{ margin-left: {}; margin-right: {}; }}\n", namespace, key, value, value));
            css.push_str(&format!("    r-s[{}*=\"margin-y:{}\"] {{ margin-top: {}; margin-bottom: {}; }}\n", namespace, key, value, value));
            
            // Negative margins
            if key != &"0" {
                css.push_str(&format!("    r-s[{}*=\"margin:-{}\"] {{ margin: -{}; }}\n", namespace, key, value));
                css.push_str(&format!("    r-s[{}*=\"margin-top:-{}\"] {{ margin-top: -{}; }}\n", namespace, key, value));
                css.push_str(&format!("    r-s[{}*=\"margin-right:-{}\"] {{ margin-right: -{}; }}\n", namespace, key, value));
                css.push_str(&format!("    r-s[{}*=\"margin-bottom:-{}\"] {{ margin-bottom: -{}; }}\n", namespace, key, value));
                css.push_str(&format!("    r-s[{}*=\"margin-left:-{}\"] {{ margin-left: -{}; }}\n", namespace, key, value));
            }
        }
        
        css
    }
    
    fn generate_width(namespace: &str) -> String {
        let mut css = String::new();
        
        // Keywords
        css.push_str(&format!("    r-s[{}*=\"width:full\"] {{ width: 100%; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"width:half\"] {{ width: 50%; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"width:third\"] {{ width: 33.333333%; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"width:quarter\"] {{ width: 25%; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"width:screen\"] {{ width: 100vw; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"width:auto\"] {{ width: auto; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"width:min\"] {{ width: min-content; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"width:max\"] {{ width: max-content; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"width:fit\"] {{ width: fit-content; }}\n", namespace));
        
        // Fixed widths
        let fixed_widths = [
            ("320", "320px"),
            ("480", "480px"),
            ("640", "640px"),
            ("768", "768px"),
            ("960", "960px"),
            ("1024", "1024px"),
            ("1200", "1200px"),
            ("1440", "1440px"),
        ];
        
        for (key, value) in &fixed_widths {
            css.push_str(&format!("    r-s[{}*=\"width:{}\"] {{ width: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"max-width:{}\"] {{ max-width: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"min-width:{}\"] {{ min-width: {}; }}\n", namespace, key, value));
        }
        
        css
    }
    
    fn generate_height(namespace: &str) -> String {
        let mut css = String::new();
        
        // Keywords
        css.push_str(&format!("    r-s[{}*=\"height:full\"] {{ height: 100%; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"height:screen\"] {{ height: 100vh; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"height:min\"] {{ min-height: 0; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"height:max\"] {{ height: max-content; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"height:fit\"] {{ height: fit-content; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"height:auto\"] {{ height: auto; }}\n", namespace));
        
        // Fixed heights
        let fixed_heights = [
            ("0", "0"),
            ("10", "2.5rem"),
            ("12", "3rem"),
            ("16", "4rem"),
            ("20", "5rem"),
            ("24", "6rem"),
            ("32", "8rem"),
            ("40", "10rem"),
            ("48", "12rem"),
            ("56", "14rem"),
            ("64", "16rem"),
        ];
        
        for (key, value) in &fixed_heights {
            css.push_str(&format!("    r-s[{}*=\"height:{}\"] {{ height: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"min-height:{}\"] {{ min-height: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"max-height:{}\"] {{ max-height: {}; }}\n", namespace, key, value));
        }
        
        css
    }
    
    fn generate_display(namespace: &str) -> String {
        let mut css = String::new();
        
        let displays = [
            "block", "inline-block", "inline", "flex", "inline-flex",
            "grid", "inline-grid", "table", "table-row", "table-cell",
            "contents", "list-item", "none",
        ];
        
        for display in &displays {
            css.push_str(&format!("    r-s[{}*=\"display:{}\"] {{ display: {}; }}\n", namespace, display, display));
        }
        
        css
    }
    
    fn generate_position(namespace: &str) -> String {
        let mut css = String::new();
        
        let positions = ["static", "relative", "absolute", "fixed", "sticky"];
        
        for position in &positions {
            css.push_str(&format!("    r-s[{}*=\"position:{}\"] {{ position: {}; }}\n", namespace, position, position));
        }
        
        // Position values
        let position_values = [
            ("0", "0"),
            ("1", "0.25rem"),
            ("2", "0.5rem"),
            ("4", "1rem"),
            ("8", "2rem"),
            ("auto", "auto"),
        ];
        
        for (key, value) in &position_values {
            css.push_str(&format!("    r-s[{}*=\"top:{}\"] {{ top: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"right:{}\"] {{ right: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"bottom:{}\"] {{ bottom: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"left:{}\"] {{ left: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"inset:{}\"] {{ inset: {}; }}\n", namespace, key, value));
        }
        
        // Z-index
        let z_indices = [("0", "0"), ("10", "10"), ("20", "20"), ("30", "30"), ("40", "40"), ("50", "50"), ("auto", "auto")];
        for (key, value) in &z_indices {
            css.push_str(&format!("    r-s[{}*=\"z:{}\"] {{ z-index: {}; }}\n", namespace, key, value));
        }
        
        css
    }
    
    fn generate_overflow(namespace: &str) -> String {
        let mut css = String::new();
        
        let overflows = ["auto", "hidden", "visible", "scroll"];
        
        for overflow in &overflows {
            css.push_str(&format!("    r-s[{}*=\"overflow:{}\"] {{ overflow: {}; }}\n", namespace, overflow, overflow));
            css.push_str(&format!("    r-s[{}*=\"overflow-x:{}\"] {{ overflow-x: {}; }}\n", namespace, overflow, overflow));
            css.push_str(&format!("    r-s[{}*=\"overflow-y:{}\"] {{ overflow-y: {}; }}\n", namespace, overflow, overflow));
        }
        
        css
    }
    
    fn generate_box_sizing(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"box-sizing:border\"] {{ box-sizing: border-box; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"box-sizing:content\"] {{ box-sizing: content-box; }}\n", namespace));
        
        css
    }
    
    fn generate_aspect_ratio(namespace: &str) -> String {
        let mut css = String::new();
        
        let ratios = [
            ("square", "1 / 1"),
            ("video", "16 / 9"),
            ("4/3", "4 / 3"),
            ("5/4", "5 / 4"),
            ("16/10", "16 / 10"),
            ("21/9", "21 / 9"),
        ];
        
        for (key, value) in &ratios {
            css.push_str(&format!("    r-s[{}*=\"aspect:{}\"] {{ aspect-ratio: {}; }}\n", namespace, key, value));
        }
        
        css
    }
    
    /// Legacy method for compatibility - now returns empty
    pub fn generate_responsive(_breakpoint: &str, _min_width: &str) -> Result<String> {
        // This method is no longer used but kept for compatibility
        // The main generate() method now handles all breakpoints
        Ok(String::new())
    }
}