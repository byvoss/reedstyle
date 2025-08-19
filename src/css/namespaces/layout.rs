use anyhow::Result;
use crate::config::Config;

pub struct LayoutNamespace;

impl LayoutNamespace {
    pub fn generate(_config: &Config) -> Result<String> {
        let mut css = String::new();
        
        css.push_str("  /* ========== Layout Namespace ========== */\n");
        
        // Flexbox
        css.push_str(&Self::generate_flexbox());
        
        // Grid
        css.push_str(&Self::generate_grid());
        
        // Gap (for both flex and grid)
        css.push_str(&Self::generate_gap());
        
        // Position
        css.push_str(&Self::generate_position());
        
        // Z-index
        css.push_str(&Self::generate_z_index());
        
        // Float & Clear
        css.push_str(&Self::generate_float());
        
        Ok(css)
    }
    
    fn generate_flexbox() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Flexbox */\n");
        
        // Enable flex and direction
        css.push_str("  reed[layout*=\"flex\"] { display: flex; }\n");
        css.push_str("  reed[layout*=\"flex:row\"] { display: flex; flex-direction: row; }\n");
        css.push_str("  reed[layout*=\"flex:column\"] { display: flex; flex-direction: column; }\n");
        css.push_str("  reed[layout*=\"flex:row-reverse\"] { display: flex; flex-direction: row-reverse; }\n");
        css.push_str("  reed[layout*=\"flex:column-reverse\"] { display: flex; flex-direction: column-reverse; }\n");
        
        // Justify content (main axis)
        css.push_str("\n  /* Justify Content */\n");
        css.push_str("  reed[layout*=\"justify:start\"] { justify-content: flex-start; }\n");
        css.push_str("  reed[layout*=\"justify:center\"] { justify-content: center; }\n");
        css.push_str("  reed[layout*=\"justify:end\"] { justify-content: flex-end; }\n");
        css.push_str("  reed[layout*=\"justify:between\"] { justify-content: space-between; }\n");
        css.push_str("  reed[layout*=\"justify:around\"] { justify-content: space-around; }\n");
        css.push_str("  reed[layout*=\"justify:evenly\"] { justify-content: space-evenly; }\n");
        
        // Align items (cross axis)
        css.push_str("\n  /* Align Items */\n");
        css.push_str("  reed[layout*=\"align:start\"] { align-items: flex-start; }\n");
        css.push_str("  reed[layout*=\"align:center\"] { align-items: center; }\n");
        css.push_str("  reed[layout*=\"align:end\"] { align-items: flex-end; }\n");
        css.push_str("  reed[layout*=\"align:stretch\"] { align-items: stretch; }\n");
        css.push_str("  reed[layout*=\"align:baseline\"] { align-items: baseline; }\n");
        
        // Align content (multi-line)
        css.push_str("\n  /* Align Content */\n");
        css.push_str("  reed[layout*=\"content:start\"] { align-content: flex-start; }\n");
        css.push_str("  reed[layout*=\"content:center\"] { align-content: center; }\n");
        css.push_str("  reed[layout*=\"content:end\"] { align-content: flex-end; }\n");
        css.push_str("  reed[layout*=\"content:between\"] { align-content: space-between; }\n");
        css.push_str("  reed[layout*=\"content:around\"] { align-content: space-around; }\n");
        css.push_str("  reed[layout*=\"content:stretch\"] { align-content: stretch; }\n");
        
        // Flex wrap
        css.push_str("\n  /* Flex Wrap */\n");
        css.push_str("  reed[layout*=\"wrap:nowrap\"] { flex-wrap: nowrap; }\n");
        css.push_str("  reed[layout*=\"wrap:wrap\"] { flex-wrap: wrap; }\n");
        css.push_str("  reed[layout*=\"wrap:wrap-reverse\"] { flex-wrap: wrap-reverse; }\n");
        
        // Flex item properties
        css.push_str("\n  /* Flex Item Properties */\n");
        css.push_str("  reed[layout*=\"grow:0\"] { flex-grow: 0; }\n");
        css.push_str("  reed[layout*=\"grow:1\"] { flex-grow: 1; }\n");
        css.push_str("  reed[layout*=\"shrink:0\"] { flex-shrink: 0; }\n");
        css.push_str("  reed[layout*=\"shrink:1\"] { flex-shrink: 1; }\n");
        css.push_str("  reed[layout*=\"basis:auto\"] { flex-basis: auto; }\n");
        css.push_str("  reed[layout*=\"basis:full\"] { flex-basis: 100%; }\n");
        
        // Self alignment
        css.push_str("\n  /* Self Alignment */\n");
        css.push_str("  reed[layout*=\"self:auto\"] { align-self: auto; }\n");
        css.push_str("  reed[layout*=\"self:start\"] { align-self: flex-start; }\n");
        css.push_str("  reed[layout*=\"self:center\"] { align-self: center; }\n");
        css.push_str("  reed[layout*=\"self:end\"] { align-self: flex-end; }\n");
        css.push_str("  reed[layout*=\"self:stretch\"] { align-self: stretch; }\n");
        
        // Order
        css.push_str("\n  /* Order */\n");
        css.push_str("  reed[layout*=\"order:-1\"] { order: -1; }\n");
        css.push_str("  reed[layout*=\"order:0\"] { order: 0; }\n");
        css.push_str("  reed[layout*=\"order:1\"] { order: 1; }\n");
        css.push_str("  reed[layout*=\"order:2\"] { order: 2; }\n");
        css.push_str("  reed[layout*=\"order:3\"] { order: 3; }\n");
        
        // Common patterns
        css.push_str("\n  /* Common Patterns */\n");
        css.push_str("  reed[layout*=\"stack\"] { display: flex; flex-direction: column; }\n");
        css.push_str("  reed[layout*=\"chain\"] { display: flex; flex-direction: row; flex-wrap: wrap; }\n");
        
        css
    }
    
    fn generate_grid() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Grid */\n");
        
        // Enable grid
        css.push_str("  reed[layout*=\"grid\"] { display: grid; }\n");
        
        // Column templates
        css.push_str("\n  /* Grid Columns */\n");
        css.push_str("  reed[layout*=\"grid:1\"] { display: grid; grid-template-columns: 1fr; }\n");
        css.push_str("  reed[layout*=\"grid:2\"] { display: grid; grid-template-columns: repeat(2, 1fr); }\n");
        css.push_str("  reed[layout*=\"grid:3\"] { display: grid; grid-template-columns: repeat(3, 1fr); }\n");
        css.push_str("  reed[layout*=\"grid:4\"] { display: grid; grid-template-columns: repeat(4, 1fr); }\n");
        css.push_str("  reed[layout*=\"grid:5\"] { display: grid; grid-template-columns: repeat(5, 1fr); }\n");
        css.push_str("  reed[layout*=\"grid:6\"] { display: grid; grid-template-columns: repeat(6, 1fr); }\n");
        css.push_str("  reed[layout*=\"grid:8\"] { display: grid; grid-template-columns: repeat(8, 1fr); }\n");
        css.push_str("  reed[layout*=\"grid:12\"] { display: grid; grid-template-columns: repeat(12, 1fr); }\n");
        
        // Custom grid patterns
        css.push_str("\n  /* Custom Grid Templates */\n");
        css.push_str("  reed[layout*=\"cols:1fr:2fr\"] { grid-template-columns: 1fr 2fr; }\n");
        css.push_str("  reed[layout*=\"cols:1fr:2fr:1fr\"] { grid-template-columns: 1fr 2fr 1fr; }\n");
        css.push_str("  reed[layout*=\"cols:200px:1fr\"] { grid-template-columns: 200px 1fr; }\n");
        css.push_str("  reed[layout*=\"cols:1fr:200px\"] { grid-template-columns: 1fr 200px; }\n");
        css.push_str("  reed[layout*=\"cols:250px:1fr:250px\"] { grid-template-columns: 250px 1fr 250px; }\n");
        
        // Row templates
        css.push_str("\n  /* Grid Rows */\n");
        css.push_str("  reed[layout*=\"rows:auto\"] { grid-template-rows: auto; }\n");
        css.push_str("  reed[layout*=\"rows:1fr\"] { grid-template-rows: 1fr; }\n");
        css.push_str("  reed[layout*=\"rows:1fr:2fr\"] { grid-template-rows: 1fr 2fr; }\n");
        css.push_str("  reed[layout*=\"rows:auto:1fr:auto\"] { grid-template-rows: auto 1fr auto; }\n");
        css.push_str("  reed[layout*=\"rows:100px:1fr:100px\"] { grid-template-rows: 100px 1fr 100px; }\n");
        
        // Auto flow
        css.push_str("\n  /* Grid Auto Flow */\n");
        css.push_str("  reed[layout*=\"flow:row\"] { grid-auto-flow: row; }\n");
        css.push_str("  reed[layout*=\"flow:column\"] { grid-auto-flow: column; }\n");
        css.push_str("  reed[layout*=\"flow:dense\"] { grid-auto-flow: dense; }\n");
        
        // Grid alignment
        css.push_str("\n  /* Grid Alignment */\n");
        css.push_str("  reed[layout*=\"justify-items:start\"] { justify-items: start; }\n");
        css.push_str("  reed[layout*=\"justify-items:center\"] { justify-items: center; }\n");
        css.push_str("  reed[layout*=\"justify-items:end\"] { justify-items: end; }\n");
        css.push_str("  reed[layout*=\"justify-items:stretch\"] { justify-items: stretch; }\n");
        
        css.push_str("  reed[layout*=\"align-items:start\"] { align-items: start; }\n");
        css.push_str("  reed[layout*=\"align-items:center\"] { align-items: center; }\n");
        css.push_str("  reed[layout*=\"align-items:end\"] { align-items: end; }\n");
        css.push_str("  reed[layout*=\"align-items:stretch\"] { align-items: stretch; }\n");
        
        css.push_str("  reed[layout*=\"place:center\"] { place-items: center; }\n");
        
        // Grid item properties
        css.push_str("\n  /* Grid Item Properties */\n");
        css.push_str("  reed[layout*=\"col-span:1\"] { grid-column: span 1 / span 1; }\n");
        css.push_str("  reed[layout*=\"col-span:2\"] { grid-column: span 2 / span 2; }\n");
        css.push_str("  reed[layout*=\"col-span:3\"] { grid-column: span 3 / span 3; }\n");
        css.push_str("  reed[layout*=\"col-span:4\"] { grid-column: span 4 / span 4; }\n");
        css.push_str("  reed[layout*=\"col-span:6\"] { grid-column: span 6 / span 6; }\n");
        css.push_str("  reed[layout*=\"col-span:12\"] { grid-column: span 12 / span 12; }\n");
        css.push_str("  reed[layout*=\"col-span:full\"] { grid-column: 1 / -1; }\n");
        
        css.push_str("  reed[layout*=\"row-span:1\"] { grid-row: span 1 / span 1; }\n");
        css.push_str("  reed[layout*=\"row-span:2\"] { grid-row: span 2 / span 2; }\n");
        css.push_str("  reed[layout*=\"row-span:3\"] { grid-row: span 3 / span 3; }\n");
        css.push_str("  reed[layout*=\"row-span:4\"] { grid-row: span 4 / span 4; }\n");
        
        // Named areas
        css.push_str("\n  /* Grid Areas */\n");
        css.push_str("  reed[layout*=\"area:header\"] { grid-area: header; }\n");
        css.push_str("  reed[layout*=\"area:sidebar\"] { grid-area: sidebar; }\n");
        css.push_str("  reed[layout*=\"area:content\"] { grid-area: content; }\n");
        css.push_str("  reed[layout*=\"area:footer\"] { grid-area: footer; }\n");
        
        css
    }
    
    fn generate_gap() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Gap (Flexbox & Grid) */\n");
        
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
            css.push_str(&format!("  reed[layout*=\"gap:{}\"] {{ gap: {}; }}\n", key, value));
            css.push_str(&format!("  reed[layout*=\"gap-x:{}\"] {{ column-gap: {}; }}\n", key, value));
            css.push_str(&format!("  reed[layout*=\"gap-y:{}\"] {{ row-gap: {}; }}\n", key, value));
        }
        
        css
    }
    
    fn generate_position() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Position */\n");
        
        css.push_str("  reed[layout*=\"position:static\"] { position: static; }\n");
        css.push_str("  reed[layout*=\"position:relative\"] { position: relative; }\n");
        css.push_str("  reed[layout*=\"position:absolute\"] { position: absolute; }\n");
        css.push_str("  reed[layout*=\"position:fixed\"] { position: fixed; }\n");
        css.push_str("  reed[layout*=\"position:sticky\"] { position: sticky; }\n");
        
        // Position values
        css.push_str("\n  /* Position Values */\n");
        let positions = ["0", "1", "2", "4", "8"];
        for pos in &positions {
            let value = if pos == &"0" { "0" } else { &format!("{}rem", pos) };
            css.push_str(&format!("  reed[layout*=\"top:{}\"] {{ top: {}; }}\n", pos, value));
            css.push_str(&format!("  reed[layout*=\"right:{}\"] {{ right: {}; }}\n", pos, value));
            css.push_str(&format!("  reed[layout*=\"bottom:{}\"] {{ bottom: {}; }}\n", pos, value));
            css.push_str(&format!("  reed[layout*=\"left:{}\"] {{ left: {}; }}\n", pos, value));
        }
        
        // Inset shortcuts
        css.push_str("\n  /* Inset Shortcuts */\n");
        css.push_str("  reed[layout*=\"inset:0\"] { top: 0; right: 0; bottom: 0; left: 0; }\n");
        css.push_str("  reed[layout*=\"inset:1\"] { top: 0.25rem; right: 0.25rem; bottom: 0.25rem; left: 0.25rem; }\n");
        css.push_str("  reed[layout*=\"inset:2\"] { top: 0.5rem; right: 0.5rem; bottom: 0.5rem; left: 0.5rem; }\n");
        css.push_str("  reed[layout*=\"inset:4\"] { top: 1rem; right: 1rem; bottom: 1rem; left: 1rem; }\n");
        css.push_str("  reed[layout*=\"inset-x:0\"] { left: 0; right: 0; }\n");
        css.push_str("  reed[layout*=\"inset-y:0\"] { top: 0; bottom: 0; }\n");
        
        css
    }
    
    fn generate_z_index() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Z-Index */\n");
        
        css.push_str("  reed[layout*=\"z:auto\"] { z-index: auto; }\n");
        css.push_str("  reed[layout*=\"z:-1\"] { z-index: -1; }\n");
        css.push_str("  reed[layout*=\"z:0\"] { z-index: 0; }\n");
        css.push_str("  reed[layout*=\"z:10\"] { z-index: 10; }\n");
        css.push_str("  reed[layout*=\"z:20\"] { z-index: 20; }\n");
        css.push_str("  reed[layout*=\"z:30\"] { z-index: 30; }\n");
        css.push_str("  reed[layout*=\"z:40\"] { z-index: 40; }\n");
        css.push_str("  reed[layout*=\"z:50\"] { z-index: 50; }\n");
        css.push_str("  reed[layout*=\"z:100\"] { z-index: 100; }\n");
        
        css
    }
    
    fn generate_float() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Float & Clear */\n");
        
        css.push_str("  reed[layout*=\"float:left\"] { float: left; }\n");
        css.push_str("  reed[layout*=\"float:right\"] { float: right; }\n");
        css.push_str("  reed[layout*=\"float:none\"] { float: none; }\n");
        
        css.push_str("  reed[layout*=\"clear:left\"] { clear: left; }\n");
        css.push_str("  reed[layout*=\"clear:right\"] { clear: right; }\n");
        css.push_str("  reed[layout*=\"clear:both\"] { clear: both; }\n");
        
        css
    }
    
    pub fn generate_responsive(breakpoint: &str, min_width: &str) -> Result<String> {
        let mut css = String::new();
        
        css.push_str(&format!("\n  @media (min-width: {}) {{\n", min_width));
        css.push_str(&format!("    /* Layout namespace - {} */\n", breakpoint));
        
        // Flexbox responsive
        css.push_str(&format!("    reed[layout-{}*=\"flex\"] {{ display: flex; }}\n", breakpoint));
        css.push_str(&format!("    reed[layout-{}*=\"flex:row\"] {{ display: flex; flex-direction: row; }}\n", breakpoint));
        css.push_str(&format!("    reed[layout-{}*=\"flex:column\"] {{ display: flex; flex-direction: column; }}\n", breakpoint));
        css.push_str(&format!("    reed[layout-{}*=\"justify:center\"] {{ justify-content: center; }}\n", breakpoint));
        css.push_str(&format!("    reed[layout-{}*=\"align:center\"] {{ align-items: center; }}\n", breakpoint));
        
        // Grid responsive
        css.push_str(&format!("    reed[layout-{}*=\"grid:1\"] {{ display: grid; grid-template-columns: 1fr; }}\n", breakpoint));
        css.push_str(&format!("    reed[layout-{}*=\"grid:2\"] {{ display: grid; grid-template-columns: repeat(2, 1fr); }}\n", breakpoint));
        css.push_str(&format!("    reed[layout-{}*=\"grid:3\"] {{ display: grid; grid-template-columns: repeat(3, 1fr); }}\n", breakpoint));
        css.push_str(&format!("    reed[layout-{}*=\"grid:4\"] {{ display: grid; grid-template-columns: repeat(4, 1fr); }}\n", breakpoint));
        
        // Gap responsive
        css.push_str(&format!("    reed[layout-{}*=\"gap:4\"] {{ gap: 1rem; }}\n", breakpoint));
        css.push_str(&format!("    reed[layout-{}*=\"gap:6\"] {{ gap: 1.5rem; }}\n", breakpoint));
        css.push_str(&format!("    reed[layout-{}*=\"gap:8\"] {{ gap: 2rem; }}\n", breakpoint));
        
        css.push_str("  }\n");
        
        Ok(css)
    }
}