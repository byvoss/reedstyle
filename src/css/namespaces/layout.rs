use anyhow::Result;
use crate::config::Config;
use crate::css::breakpoints::BREAKPOINTS;

pub struct LayoutNamespace;

impl LayoutNamespace {
    /// Generate layout namespace CSS for all breakpoints
    pub fn generate(_config: &Config) -> Result<String> {
        let mut css = String::new();
        
        css.push_str("  /* ========== Layout Namespace (Responsive) ========== */\n");
        
        // Generate for each breakpoint
        for (suffix, min_width) in BREAKPOINTS {
            let namespace = format!("layout{}", suffix);
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
    
    /// Generate all layout properties for a specific namespace
    fn generate_for_namespace(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("\n    /* {} */\n", namespace));
        
        // Generate all layout properties
        css.push_str(&Self::generate_flexbox(namespace));
        css.push_str(&Self::generate_grid(namespace));
        css.push_str(&Self::generate_gap(namespace));
        css.push_str(&Self::generate_position(namespace));
        css.push_str(&Self::generate_z_index(namespace));
        css.push_str(&Self::generate_float(namespace));
        
        css
    }
    
    fn generate_flexbox(namespace: &str) -> String {
        let mut css = String::new();
        
        // Enable flex and direction
        css.push_str(&format!("    r-s[{}*=\"flex\"] {{ display: flex; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"flex:row\"] {{ display: flex; flex-direction: row; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"flex:column\"] {{ display: flex; flex-direction: column; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"flex:row-reverse\"] {{ display: flex; flex-direction: row-reverse; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"flex:column-reverse\"] {{ display: flex; flex-direction: column-reverse; }}\n", namespace));
        
        // Justify content (main axis)
        css.push_str(&format!("    r-s[{}*=\"justify:start\"] {{ justify-content: flex-start; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"justify:center\"] {{ justify-content: center; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"justify:end\"] {{ justify-content: flex-end; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"justify:between\"] {{ justify-content: space-between; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"justify:around\"] {{ justify-content: space-around; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"justify:evenly\"] {{ justify-content: space-evenly; }}\n", namespace));
        
        // Align items (cross axis)
        css.push_str(&format!("    r-s[{}*=\"align:start\"] {{ align-items: flex-start; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"align:center\"] {{ align-items: center; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"align:end\"] {{ align-items: flex-end; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"align:stretch\"] {{ align-items: stretch; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"align:baseline\"] {{ align-items: baseline; }}\n", namespace));
        
        // Align content (multi-line)
        css.push_str(&format!("    r-s[{}*=\"content:start\"] {{ align-content: flex-start; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"content:center\"] {{ align-content: center; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"content:end\"] {{ align-content: flex-end; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"content:between\"] {{ align-content: space-between; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"content:around\"] {{ align-content: space-around; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"content:stretch\"] {{ align-content: stretch; }}\n", namespace));
        
        // Flex wrap
        css.push_str(&format!("    r-s[{}*=\"wrap:nowrap\"] {{ flex-wrap: nowrap; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"wrap:wrap\"] {{ flex-wrap: wrap; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"wrap:wrap-reverse\"] {{ flex-wrap: wrap-reverse; }}\n", namespace));
        
        // Flex item properties
        css.push_str(&format!("    r-s[{}*=\"grow:0\"] {{ flex-grow: 0; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"grow:1\"] {{ flex-grow: 1; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"shrink:0\"] {{ flex-shrink: 0; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"shrink:1\"] {{ flex-shrink: 1; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"basis:auto\"] {{ flex-basis: auto; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"basis:full\"] {{ flex-basis: 100%; }}\n", namespace));
        
        // Self alignment
        css.push_str(&format!("    r-s[{}*=\"self:auto\"] {{ align-self: auto; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"self:start\"] {{ align-self: flex-start; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"self:center\"] {{ align-self: center; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"self:end\"] {{ align-self: flex-end; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"self:stretch\"] {{ align-self: stretch; }}\n", namespace));
        
        // Order
        css.push_str(&format!("    r-s[{}*=\"order:-1\"] {{ order: -1; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"order:0\"] {{ order: 0; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"order:1\"] {{ order: 1; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"order:2\"] {{ order: 2; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"order:3\"] {{ order: 3; }}\n", namespace));
        
        // Common patterns
        css.push_str(&format!("    r-s[{}*=\"stack\"] {{ display: flex; flex-direction: column; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"chain\"] {{ display: flex; flex-direction: row; flex-wrap: wrap; }}\n", namespace));
        
        css
    }
    
    fn generate_grid(namespace: &str) -> String {
        let mut css = String::new();
        
        // Enable grid
        css.push_str(&format!("    r-s[{}*=\"grid\"] {{ display: grid; }}\n", namespace));
        
        // Column templates
        css.push_str(&format!("    r-s[{}*=\"grid:1\"] {{ display: grid; grid-template-columns: 1fr; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"grid:2\"] {{ display: grid; grid-template-columns: repeat(2, 1fr); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"grid:3\"] {{ display: grid; grid-template-columns: repeat(3, 1fr); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"grid:4\"] {{ display: grid; grid-template-columns: repeat(4, 1fr); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"grid:5\"] {{ display: grid; grid-template-columns: repeat(5, 1fr); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"grid:6\"] {{ display: grid; grid-template-columns: repeat(6, 1fr); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"grid:8\"] {{ display: grid; grid-template-columns: repeat(8, 1fr); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"grid:12\"] {{ display: grid; grid-template-columns: repeat(12, 1fr); }}\n", namespace));
        
        // Custom grid patterns
        css.push_str(&format!("    r-s[{}*=\"cols:1fr:2fr\"] {{ grid-template-columns: 1fr 2fr; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cols:1fr:2fr:1fr\"] {{ grid-template-columns: 1fr 2fr 1fr; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cols:200px:1fr\"] {{ grid-template-columns: 200px 1fr; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cols:1fr:200px\"] {{ grid-template-columns: 1fr 200px; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cols:250px:1fr:250px\"] {{ grid-template-columns: 250px 1fr 250px; }}\n", namespace));
        
        // Row templates
        css.push_str(&format!("    r-s[{}*=\"rows:auto\"] {{ grid-template-rows: auto; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"rows:1fr\"] {{ grid-template-rows: 1fr; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"rows:1fr:2fr\"] {{ grid-template-rows: 1fr 2fr; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"rows:auto:1fr:auto\"] {{ grid-template-rows: auto 1fr auto; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"rows:100px:1fr:100px\"] {{ grid-template-rows: 100px 1fr 100px; }}\n", namespace));
        
        // Auto flow
        css.push_str(&format!("    r-s[{}*=\"flow:row\"] {{ grid-auto-flow: row; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"flow:column\"] {{ grid-auto-flow: column; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"flow:dense\"] {{ grid-auto-flow: dense; }}\n", namespace));
        
        // Grid alignment
        css.push_str(&format!("    r-s[{}*=\"justify-items:start\"] {{ justify-items: start; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"justify-items:center\"] {{ justify-items: center; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"justify-items:end\"] {{ justify-items: end; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"justify-items:stretch\"] {{ justify-items: stretch; }}\n", namespace));
        
        css.push_str(&format!("    r-s[{}*=\"align-items:start\"] {{ align-items: start; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"align-items:center\"] {{ align-items: center; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"align-items:end\"] {{ align-items: end; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"align-items:stretch\"] {{ align-items: stretch; }}\n", namespace));
        
        css.push_str(&format!("    r-s[{}*=\"place:center\"] {{ place-items: center; }}\n", namespace));
        
        // Grid item properties
        css.push_str(&format!("    r-s[{}*=\"col-span:1\"] {{ grid-column: span 1 / span 1; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"col-span:2\"] {{ grid-column: span 2 / span 2; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"col-span:3\"] {{ grid-column: span 3 / span 3; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"col-span:4\"] {{ grid-column: span 4 / span 4; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"col-span:6\"] {{ grid-column: span 6 / span 6; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"col-span:12\"] {{ grid-column: span 12 / span 12; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"col-span:full\"] {{ grid-column: 1 / -1; }}\n", namespace));
        
        css.push_str(&format!("    r-s[{}*=\"row-span:1\"] {{ grid-row: span 1 / span 1; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"row-span:2\"] {{ grid-row: span 2 / span 2; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"row-span:3\"] {{ grid-row: span 3 / span 3; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"row-span:4\"] {{ grid-row: span 4 / span 4; }}\n", namespace));
        
        // Named areas
        css.push_str(&format!("    r-s[{}*=\"area:header\"] {{ grid-area: header; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"area:sidebar\"] {{ grid-area: sidebar; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"area:content\"] {{ grid-area: content; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"area:footer\"] {{ grid-area: footer; }}\n", namespace));
        
        css
    }
    
    fn generate_gap(namespace: &str) -> String {
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
        ];
        
        for (key, value) in &scale {
            css.push_str(&format!("    r-s[{}*=\"gap:{}\"] {{ gap: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"gap-x:{}\"] {{ column-gap: {}; }}\n", namespace, key, value));
            css.push_str(&format!("    r-s[{}*=\"gap-y:{}\"] {{ row-gap: {}; }}\n", namespace, key, value));
        }
        
        css
    }
    
    fn generate_position(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"position:static\"] {{ position: static; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"position:relative\"] {{ position: relative; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"position:absolute\"] {{ position: absolute; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"position:fixed\"] {{ position: fixed; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"position:sticky\"] {{ position: sticky; }}\n", namespace));
        
        // Position values
        let positions = ["0", "1", "2", "4", "8"];
        for pos in &positions {
            let value = if pos == &"0" { "0" } else { &format!("{}rem", pos) };
            css.push_str(&format!("    r-s[{}*=\"top:{}\"] {{ top: {}; }}\n", namespace, pos, value));
            css.push_str(&format!("    r-s[{}*=\"right:{}\"] {{ right: {}; }}\n", namespace, pos, value));
            css.push_str(&format!("    r-s[{}*=\"bottom:{}\"] {{ bottom: {}; }}\n", namespace, pos, value));
            css.push_str(&format!("    r-s[{}*=\"left:{}\"] {{ left: {}; }}\n", namespace, pos, value));
        }
        
        // Inset shortcuts
        css.push_str(&format!("    r-s[{}*=\"inset:0\"] {{ top: 0; right: 0; bottom: 0; left: 0; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"inset:1\"] {{ top: 0.25rem; right: 0.25rem; bottom: 0.25rem; left: 0.25rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"inset:2\"] {{ top: 0.5rem; right: 0.5rem; bottom: 0.5rem; left: 0.5rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"inset:4\"] {{ top: 1rem; right: 1rem; bottom: 1rem; left: 1rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"inset-x:0\"] {{ left: 0; right: 0; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"inset-y:0\"] {{ top: 0; bottom: 0; }}\n", namespace));
        
        css
    }
    
    fn generate_z_index(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"z:auto\"] {{ z-index: auto; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"z:-1\"] {{ z-index: -1; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"z:0\"] {{ z-index: 0; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"z:10\"] {{ z-index: 10; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"z:20\"] {{ z-index: 20; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"z:30\"] {{ z-index: 30; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"z:40\"] {{ z-index: 40; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"z:50\"] {{ z-index: 50; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"z:100\"] {{ z-index: 100; }}\n", namespace));
        
        css
    }
    
    fn generate_float(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"float:left\"] {{ float: left; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"float:right\"] {{ float: right; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"float:none\"] {{ float: none; }}\n", namespace));
        
        css.push_str(&format!("    r-s[{}*=\"clear:left\"] {{ clear: left; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"clear:right\"] {{ clear: right; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"clear:both\"] {{ clear: both; }}\n", namespace));
        
        css
    }
}