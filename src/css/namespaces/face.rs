use anyhow::Result;
use crate::config::{Config, ColorsConfig};
use crate::css::breakpoints::BREAKPOINTS;

pub struct FaceNamespace;

impl FaceNamespace {
    /// Generate face namespace CSS for all breakpoints
    pub fn generate(_config: &Config, colors: &ColorsConfig) -> Result<String> {
        let mut css = String::new();
        
        css.push_str("  /* ========== Face Namespace (Responsive) ========== */\n");
        
        // Generate for each breakpoint
        for (suffix, min_width) in BREAKPOINTS {
            let namespace = format!("face{}", suffix);
            let breakpoint_css = Self::generate_for_namespace(&namespace, colors);
            
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
    
    /// Generate all face properties for a specific namespace
    fn generate_for_namespace(namespace: &str, _colors: &ColorsConfig) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("\n    /* {} */\n", namespace));
        
        // Generate all face properties
        css.push_str(&Self::generate_backgrounds(namespace));
        css.push_str(&Self::generate_borders(namespace));
        css.push_str(&Self::generate_radius(namespace));
        css.push_str(&Self::generate_shadows(namespace));
        css.push_str(&Self::generate_opacity(namespace));
        css.push_str(&Self::generate_outline(namespace));
        css.push_str(&Self::generate_backdrop(namespace));
        
        css
    }
    
    fn generate_backgrounds(namespace: &str) -> String {
        let mut css = String::new();
        
        // Brand colors (brand-a through brand-f)
        for letter in ['a', 'b', 'c', 'd', 'e', 'f'] {
            let color_name = format!("brand-{}", letter);
            css.push_str(&format!("    r-s[{}*=\"bg:{}\"] {{ background-color: var(--rs-{}); }}\n", namespace, color_name, color_name));
            
            // Visual scope variations
            for variant in ["weak", "light", "intense", "bright", "strong"] {
                css.push_str(&format!("    r-s[{}*=\"bg:{}-{}\"] {{ background-color: var(--rs-{}-{}); }}\n", 
                    namespace, color_name, variant, color_name, variant));
            }
        }
        
        // Base colors (neutrals) - base-0 to base-1000
        for value in [0, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000] {
            css.push_str(&format!("    r-s[{}*=\"bg:base-{}\"] {{ background-color: var(--rs-base-{}); }}\n", namespace, value, value));
        }
        
        // Semantic colors
        for semantic in ["success", "warning", "error", "info"] {
            css.push_str(&format!("    r-s[{}*=\"bg:state-{}\"] {{ background-color: var(--rs-state-{}); }}\n", namespace, semantic, semantic));
        }
        
        // Gradients
        css.push_str(&format!("    r-s[{}*=\"bg:gradient-primary\"] {{ background: linear-gradient(135deg, var(--rs-brand-a), var(--rs-brand-b)); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"bg:gradient-secondary\"] {{ background: linear-gradient(135deg, var(--rs-brand-b), var(--rs-brand-c)); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"bg:gradient-radial\"] {{ background: radial-gradient(circle, var(--rs-brand-a), var(--rs-brand-b)); }}\n", namespace));
        
        css
    }
    
    fn generate_borders(namespace: &str) -> String {
        let mut css = String::new();
        
        // Simple borders
        css.push_str(&format!("    r-s[{}*=\"border:none\"] {{ border: none; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border:1\"] {{ border-width: 1px; border-style: solid; border-color: var(--rs-base-200); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border:2\"] {{ border-width: 2px; border-style: solid; border-color: var(--rs-base-200); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border:4\"] {{ border-width: 4px; border-style: solid; border-color: var(--rs-base-200); }}\n", namespace));
        
        // Border colors
        for letter in ['a', 'b', 'c', 'd', 'e', 'f'] {
            let color = format!("brand-{}", letter);
            css.push_str(&format!("    r-s[{}*=\"border:1:{}\"] {{ border: 1px solid var(--rs-{}); }}\n", namespace, color, color));
            css.push_str(&format!("    r-s[{}*=\"border:2:{}\"] {{ border: 2px solid var(--rs-{}); }}\n", namespace, color, color));
        }
        
        // Semantic border colors
        for semantic in ["success", "warning", "error", "info"] {
            css.push_str(&format!("    r-s[{}*=\"border:1:state-{}\"] {{ border: 1px solid var(--rs-state-{}); }}\n", namespace, semantic, semantic));
            css.push_str(&format!("    r-s[{}*=\"border:2:state-{}\"] {{ border: 2px solid var(--rs-state-{}); }}\n", namespace, semantic, semantic));
        }
        
        // Border styles
        css.push_str(&format!("    r-s[{}*=\"border:1:dashed\"] {{ border: 1px dashed var(--rs-base-400); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border:1:dotted\"] {{ border: 1px dotted var(--rs-base-400); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border:2:dashed\"] {{ border: 2px dashed var(--rs-base-400); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border:2:dotted\"] {{ border: 2px dotted var(--rs-base-400); }}\n", namespace));
        
        // Individual sides
        css.push_str(&format!("    r-s[{}*=\"border-top:1\"] {{ border-top: 1px solid var(--rs-base-200); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border-right:1\"] {{ border-right: 1px solid var(--rs-base-200); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border-bottom:1\"] {{ border-bottom: 1px solid var(--rs-base-200); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border-left:1\"] {{ border-left: 1px solid var(--rs-base-200); }}\n", namespace));
        
        css.push_str(&format!("    r-s[{}*=\"border-top:2\"] {{ border-top: 2px solid var(--rs-base-200); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border-right:2\"] {{ border-right: 2px solid var(--rs-base-200); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border-bottom:2\"] {{ border-bottom: 2px solid var(--rs-base-200); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border-left:2\"] {{ border-left: 2px solid var(--rs-base-200); }}\n", namespace));
        
        // Axis shortcuts
        css.push_str(&format!("    r-s[{}*=\"border-x:1\"] {{ border-left: 1px solid var(--rs-base-200); border-right: 1px solid var(--rs-base-200); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border-y:1\"] {{ border-top: 1px solid var(--rs-base-200); border-bottom: 1px solid var(--rs-base-200); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border-x:2\"] {{ border-left: 2px solid var(--rs-base-200); border-right: 2px solid var(--rs-base-200); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"border-y:2\"] {{ border-top: 2px solid var(--rs-base-200); border-bottom: 2px solid var(--rs-base-200); }}\n", namespace));
        
        css
    }
    
    fn generate_radius(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"radius:none\"] {{ border-radius: 0; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"radius:sm\"] {{ border-radius: 0.125rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"radius:md\"] {{ border-radius: 0.25rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"radius:lg\"] {{ border-radius: 0.5rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"radius:xl\"] {{ border-radius: 0.75rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"radius:2xl\"] {{ border-radius: 1rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"radius:3xl\"] {{ border-radius: 1.5rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"radius:full\"] {{ border-radius: 9999px; }}\n", namespace));
        
        // Individual corners
        css.push_str(&format!("    r-s[{}*=\"radius-tl:lg\"] {{ border-top-left-radius: 0.5rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"radius-tr:lg\"] {{ border-top-right-radius: 0.5rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"radius-bl:lg\"] {{ border-bottom-left-radius: 0.5rem; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"radius-br:lg\"] {{ border-bottom-right-radius: 0.5rem; }}\n", namespace));
        
        css
    }
    
    fn generate_shadows(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"shadow:none\"] {{ box-shadow: none; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"shadow:sm\"] {{ box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"shadow:md\"] {{ box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"shadow:lg\"] {{ box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"shadow:xl\"] {{ box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"shadow:2xl\"] {{ box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"shadow:inner\"] {{ box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05); }}\n", namespace));
        
        // Colored shadows
        for letter in ['a', 'b', 'c'] {
            let color = format!("brand-{}", letter);
            css.push_str(&format!("    r-s[{}*=\"shadow:lg:{}\"] {{ box-shadow: 0 10px 15px -3px var(--rs-{}-weak); }}\n", namespace, color, color));
            css.push_str(&format!("    r-s[{}*=\"shadow:md:{}\"] {{ box-shadow: 0 4px 6px -1px var(--rs-{}-weak); }}\n", namespace, color, color));
        }
        
        css
    }
    
    fn generate_opacity(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"opacity:0\"] {{ opacity: 0; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"opacity:10\"] {{ opacity: 0.1; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"opacity:25\"] {{ opacity: 0.25; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"opacity:50\"] {{ opacity: 0.5; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"opacity:75\"] {{ opacity: 0.75; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"opacity:90\"] {{ opacity: 0.9; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"opacity:100\"] {{ opacity: 1; }}\n", namespace));
        
        css
    }
    
    fn generate_outline(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"outline:none\"] {{ outline: none; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"outline:1\"] {{ outline: 1px solid var(--rs-brand-a); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"outline:2\"] {{ outline: 2px solid var(--rs-brand-a); }}\n", namespace));
        
        // Outline with colors
        for letter in ['a', 'b', 'c'] {
            let color = format!("brand-{}", letter);
            css.push_str(&format!("    r-s[{}*=\"outline:1:{}\"] {{ outline: 1px solid var(--rs-{}); }}\n", namespace, color, color));
            css.push_str(&format!("    r-s[{}*=\"outline:2:{}\"] {{ outline: 2px solid var(--rs-{}); }}\n", namespace, color, color));
        }
        
        // Outline offset
        css.push_str(&format!("    r-s[{}*=\"outline-offset:2\"] {{ outline-offset: 2px; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"outline-offset:4\"] {{ outline-offset: 4px; }}\n", namespace));
        
        css
    }
    
    fn generate_backdrop(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"backdrop:blur-sm\"] {{ backdrop-filter: blur(4px); -webkit-backdrop-filter: blur(4px); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"backdrop:blur-md\"] {{ backdrop-filter: blur(8px); -webkit-backdrop-filter: blur(8px); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"backdrop:blur-lg\"] {{ backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); }}\n", namespace));
        
        css.push_str(&format!("    r-s[{}*=\"backdrop:bright-50\"] {{ backdrop-filter: brightness(0.5); -webkit-backdrop-filter: brightness(0.5); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"backdrop:bright-75\"] {{ backdrop-filter: brightness(0.75); -webkit-backdrop-filter: brightness(0.75); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"backdrop:bright-125\"] {{ backdrop-filter: brightness(1.25); -webkit-backdrop-filter: brightness(1.25); }}\n", namespace));
        
        css
    }
}