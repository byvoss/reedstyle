use anyhow::Result;
use crate::config::Config;
use crate::css::breakpoints::BREAKPOINTS;

pub struct DeviceNamespace;

impl DeviceNamespace {
    /// Generate device namespace CSS for all breakpoints
    pub fn generate(_config: &Config) -> Result<String> {
        let mut css = String::new();
        
        css.push_str("  /* ========== Device Namespace (Responsive) ========== */\n");
        
        // Generate for each breakpoint
        for (suffix, min_width) in BREAKPOINTS {
            let namespace = format!("device{}", suffix);
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
    
    /// Generate all device properties for a specific namespace
    fn generate_for_namespace(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("\n    /* {} */\n", namespace));
        
        // Generate all device properties
        css.push_str(&Self::generate_cursor(namespace));
        css.push_str(&Self::generate_pointer_events(namespace));
        css.push_str(&Self::generate_user_select(namespace));
        css.push_str(&Self::generate_touch_action(namespace));
        css.push_str(&Self::generate_scroll_behavior(namespace));
        css.push_str(&Self::generate_scroll_snap(namespace));
        css.push_str(&Self::generate_resize(namespace));
        css.push_str(&Self::generate_will_change(namespace));
        
        css
    }
    
    fn generate_cursor(namespace: &str) -> String {
        let mut css = String::new();
        
        // Common cursors
        css.push_str(&format!("    r-s[{}*=\"cursor:pointer\"] {{ cursor: pointer; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:default\"] {{ cursor: default; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:none\"] {{ cursor: none; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:wait\"] {{ cursor: wait; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:text\"] {{ cursor: text; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:move\"] {{ cursor: move; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:grab\"] {{ cursor: grab; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:grabbing\"] {{ cursor: grabbing; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:not-allowed\"] {{ cursor: not-allowed; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:help\"] {{ cursor: help; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:crosshair\"] {{ cursor: crosshair; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:zoom-in\"] {{ cursor: zoom-in; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:zoom-out\"] {{ cursor: zoom-out; }}\n", namespace));
        
        // Resize cursors
        css.push_str(&format!("    r-s[{}*=\"cursor:resize\"] {{ cursor: all-scroll; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:n-resize\"] {{ cursor: n-resize; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:e-resize\"] {{ cursor: e-resize; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:s-resize\"] {{ cursor: s-resize; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:w-resize\"] {{ cursor: w-resize; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:ne-resize\"] {{ cursor: ne-resize; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:nw-resize\"] {{ cursor: nw-resize; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:se-resize\"] {{ cursor: se-resize; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"cursor:sw-resize\"] {{ cursor: sw-resize; }}\n", namespace));
        
        css
    }
    
    fn generate_pointer_events(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"pointer:none\"] {{ pointer-events: none; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"pointer:auto\"] {{ pointer-events: auto; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"pointer:all\"] {{ pointer-events: all; }}\n", namespace));
        
        css
    }
    
    fn generate_user_select(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"select:none\"] {{ user-select: none; -webkit-user-select: none; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"select:auto\"] {{ user-select: auto; -webkit-user-select: auto; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"select:text\"] {{ user-select: text; -webkit-user-select: text; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"select:all\"] {{ user-select: all; -webkit-user-select: all; }}\n", namespace));
        
        css
    }
    
    fn generate_touch_action(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"touch:none\"] {{ touch-action: none; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"touch:auto\"] {{ touch-action: auto; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"touch:pan-x\"] {{ touch-action: pan-x; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"touch:pan-y\"] {{ touch-action: pan-y; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"touch:pinch-zoom\"] {{ touch-action: pinch-zoom; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"touch:manipulation\"] {{ touch-action: manipulation; }}\n", namespace));
        
        css
    }
    
    fn generate_scroll_behavior(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"scroll:smooth\"] {{ scroll-behavior: smooth; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"scroll:auto\"] {{ scroll-behavior: auto; }}\n", namespace));
        
        css
    }
    
    fn generate_scroll_snap(namespace: &str) -> String {
        let mut css = String::new();
        
        // Snap types
        css.push_str(&format!("    r-s[{}*=\"snap-type:none\"] {{ scroll-snap-type: none; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"snap-type:x\"] {{ scroll-snap-type: x mandatory; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"snap-type:y\"] {{ scroll-snap-type: y mandatory; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"snap-type:block\"] {{ scroll-snap-type: block mandatory; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"snap-type:inline\"] {{ scroll-snap-type: inline mandatory; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"snap-type:both\"] {{ scroll-snap-type: both mandatory; }}\n", namespace));
        
        // Snap strictness
        css.push_str(&format!("    r-s[{}*=\"snap-type:mandatory\"] {{ scroll-snap-type: inherit mandatory; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"snap-type:proximity\"] {{ scroll-snap-type: inherit proximity; }}\n", namespace));
        
        // Snap align
        css.push_str(&format!("    r-s[{}*=\"snap-align:start\"] {{ scroll-snap-align: start; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"snap-align:center\"] {{ scroll-snap-align: center; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"snap-align:end\"] {{ scroll-snap-align: end; }}\n", namespace));
        
        css
    }
    
    fn generate_resize(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"resize:none\"] {{ resize: none; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"resize:both\"] {{ resize: both; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"resize:horizontal\"] {{ resize: horizontal; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"resize:vertical\"] {{ resize: vertical; }}\n", namespace));
        
        css
    }
    
    fn generate_will_change(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"will-change:transform\"] {{ will-change: transform; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"will-change:opacity\"] {{ will-change: opacity; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"will-change:scroll\"] {{ will-change: scroll-position; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"will-change:contents\"] {{ will-change: contents; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"will-change:auto\"] {{ will-change: auto; }}\n", namespace));
        
        css
    }
}