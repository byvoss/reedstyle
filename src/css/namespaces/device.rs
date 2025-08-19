use anyhow::Result;
use crate::config::Config;

pub struct DeviceNamespace;

impl DeviceNamespace {
    pub fn generate(_config: &Config) -> Result<String> {
        let mut css = String::new();
        
        css.push_str("  /* ========== Device Namespace ========== */\n");
        
        // Cursor
        css.push_str(&Self::generate_cursor());
        
        // Pointer events
        css.push_str(&Self::generate_pointer_events());
        
        // User select
        css.push_str(&Self::generate_user_select());
        
        // Touch action
        css.push_str(&Self::generate_touch_action());
        
        // Scroll behavior
        css.push_str(&Self::generate_scroll_behavior());
        
        // Scroll snap
        css.push_str(&Self::generate_scroll_snap());
        
        // Resize
        css.push_str(&Self::generate_resize());
        
        // Will change
        css.push_str(&Self::generate_will_change());
        
        Ok(css)
    }
    
    fn generate_cursor() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Cursor */\n");
        
        // Common cursors
        css.push_str("  reed[device*=\"cursor:pointer\"] { cursor: pointer; }\n");
        css.push_str("  reed[device*=\"cursor:default\"] { cursor: default; }\n");
        css.push_str("  reed[device*=\"cursor:none\"] { cursor: none; }\n");
        css.push_str("  reed[device*=\"cursor:wait\"] { cursor: wait; }\n");
        css.push_str("  reed[device*=\"cursor:text\"] { cursor: text; }\n");
        css.push_str("  reed[device*=\"cursor:move\"] { cursor: move; }\n");
        css.push_str("  reed[device*=\"cursor:grab\"] { cursor: grab; }\n");
        css.push_str("  reed[device*=\"cursor:grabbing\"] { cursor: grabbing; }\n");
        css.push_str("  reed[device*=\"cursor:not-allowed\"] { cursor: not-allowed; }\n");
        css.push_str("  reed[device*=\"cursor:help\"] { cursor: help; }\n");
        css.push_str("  reed[device*=\"cursor:crosshair\"] { cursor: crosshair; }\n");
        css.push_str("  reed[device*=\"cursor:zoom-in\"] { cursor: zoom-in; }\n");
        css.push_str("  reed[device*=\"cursor:zoom-out\"] { cursor: zoom-out; }\n");
        
        // Resize cursors
        css.push_str("  reed[device*=\"cursor:resize\"] { cursor: all-scroll; }\n");
        css.push_str("  reed[device*=\"cursor:n-resize\"] { cursor: n-resize; }\n");
        css.push_str("  reed[device*=\"cursor:e-resize\"] { cursor: e-resize; }\n");
        css.push_str("  reed[device*=\"cursor:s-resize\"] { cursor: s-resize; }\n");
        css.push_str("  reed[device*=\"cursor:w-resize\"] { cursor: w-resize; }\n");
        css.push_str("  reed[device*=\"cursor:ne-resize\"] { cursor: ne-resize; }\n");
        css.push_str("  reed[device*=\"cursor:nw-resize\"] { cursor: nw-resize; }\n");
        css.push_str("  reed[device*=\"cursor:se-resize\"] { cursor: se-resize; }\n");
        css.push_str("  reed[device*=\"cursor:sw-resize\"] { cursor: sw-resize; }\n");
        
        css
    }
    
    fn generate_pointer_events() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Pointer Events */\n");
        
        css.push_str("  reed[device*=\"pointer:none\"] { pointer-events: none; }\n");
        css.push_str("  reed[device*=\"pointer:auto\"] { pointer-events: auto; }\n");
        css.push_str("  reed[device*=\"pointer:all\"] { pointer-events: all; }\n");
        
        css
    }
    
    fn generate_user_select() -> String {
        let mut css = String::new();
        css.push_str("\n  /* User Select */\n");
        
        css.push_str("  reed[device*=\"select:none\"] { user-select: none; -webkit-user-select: none; }\n");
        css.push_str("  reed[device*=\"select:auto\"] { user-select: auto; -webkit-user-select: auto; }\n");
        css.push_str("  reed[device*=\"select:text\"] { user-select: text; -webkit-user-select: text; }\n");
        css.push_str("  reed[device*=\"select:all\"] { user-select: all; -webkit-user-select: all; }\n");
        
        css
    }
    
    fn generate_touch_action() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Touch Action */\n");
        
        css.push_str("  reed[device*=\"touch:none\"] { touch-action: none; }\n");
        css.push_str("  reed[device*=\"touch:auto\"] { touch-action: auto; }\n");
        css.push_str("  reed[device*=\"touch:pan-x\"] { touch-action: pan-x; }\n");
        css.push_str("  reed[device*=\"touch:pan-y\"] { touch-action: pan-y; }\n");
        css.push_str("  reed[device*=\"touch:pinch-zoom\"] { touch-action: pinch-zoom; }\n");
        css.push_str("  reed[device*=\"touch:manipulation\"] { touch-action: manipulation; }\n");
        
        css
    }
    
    fn generate_scroll_behavior() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Scroll Behavior */\n");
        
        css.push_str("  reed[device*=\"scroll:smooth\"] { scroll-behavior: smooth; }\n");
        css.push_str("  reed[device*=\"scroll:auto\"] { scroll-behavior: auto; }\n");
        
        css
    }
    
    fn generate_scroll_snap() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Scroll Snap */\n");
        
        // Snap types
        css.push_str("  reed[device*=\"snap-type:none\"] { scroll-snap-type: none; }\n");
        css.push_str("  reed[device*=\"snap-type:x\"] { scroll-snap-type: x mandatory; }\n");
        css.push_str("  reed[device*=\"snap-type:y\"] { scroll-snap-type: y mandatory; }\n");
        css.push_str("  reed[device*=\"snap-type:block\"] { scroll-snap-type: block mandatory; }\n");
        css.push_str("  reed[device*=\"snap-type:inline\"] { scroll-snap-type: inline mandatory; }\n");
        css.push_str("  reed[device*=\"snap-type:both\"] { scroll-snap-type: both mandatory; }\n");
        
        // Snap strictness
        css.push_str("  reed[device*=\"snap-type:mandatory\"] { scroll-snap-type: inherit mandatory; }\n");
        css.push_str("  reed[device*=\"snap-type:proximity\"] { scroll-snap-type: inherit proximity; }\n");
        
        // Snap align
        css.push_str("  reed[device*=\"snap-align:start\"] { scroll-snap-align: start; }\n");
        css.push_str("  reed[device*=\"snap-align:center\"] { scroll-snap-align: center; }\n");
        css.push_str("  reed[device*=\"snap-align:end\"] { scroll-snap-align: end; }\n");
        
        css
    }
    
    fn generate_resize() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Resize */\n");
        
        css.push_str("  reed[device*=\"resize:none\"] { resize: none; }\n");
        css.push_str("  reed[device*=\"resize:both\"] { resize: both; }\n");
        css.push_str("  reed[device*=\"resize:horizontal\"] { resize: horizontal; }\n");
        css.push_str("  reed[device*=\"resize:vertical\"] { resize: vertical; }\n");
        
        css
    }
    
    fn generate_will_change() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Will Change */\n");
        
        css.push_str("  reed[device*=\"will-change:transform\"] { will-change: transform; }\n");
        css.push_str("  reed[device*=\"will-change:opacity\"] { will-change: opacity; }\n");
        css.push_str("  reed[device*=\"will-change:scroll\"] { will-change: scroll-position; }\n");
        css.push_str("  reed[device*=\"will-change:contents\"] { will-change: contents; }\n");
        css.push_str("  reed[device*=\"will-change:auto\"] { will-change: auto; }\n");
        
        css
    }
    
    pub fn generate_responsive(breakpoint: &str, min_width: &str) -> Result<String> {
        let mut css = String::new();
        
        css.push_str(&format!("\n  @media (min-width: {}) {{\n", min_width));
        css.push_str(&format!("    /* Device namespace - {} */\n", breakpoint));
        
        // Cursor responsive
        css.push_str(&format!("    reed[device-{}*=\"cursor:pointer\"] {{ cursor: pointer; }}\n", breakpoint));
        css.push_str(&format!("    reed[device-{}*=\"cursor:default\"] {{ cursor: default; }}\n", breakpoint));
        
        // Touch responsive
        css.push_str(&format!("    reed[device-{}*=\"touch:auto\"] {{ touch-action: auto; }}\n", breakpoint));
        css.push_str(&format!("    reed[device-{}*=\"touch:manipulation\"] {{ touch-action: manipulation; }}\n", breakpoint));
        
        // Select responsive
        css.push_str(&format!("    reed[device-{}*=\"select:none\"] {{ user-select: none; -webkit-user-select: none; }}\n", breakpoint));
        css.push_str(&format!("    reed[device-{}*=\"select:text\"] {{ user-select: text; -webkit-user-select: text; }}\n", breakpoint));
        
        css.push_str("  }\n");
        
        Ok(css)
    }
}