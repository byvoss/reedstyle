use anyhow::Result;
use crate::config::{Config, ColorsConfig};

pub struct FaceNamespace;

impl FaceNamespace {
    pub fn generate(config: &Config, colors: &ColorsConfig) -> Result<String> {
        let mut css = String::new();
        
        css.push_str("  /* ========== Face Namespace ========== */\n");
        
        // Background colors
        css.push_str(&Self::generate_backgrounds(colors));
        
        // Borders
        css.push_str(&Self::generate_borders());
        
        // Border radius
        css.push_str(&Self::generate_radius());
        
        // Shadows
        css.push_str(&Self::generate_shadows());
        
        // Opacity
        css.push_str(&Self::generate_opacity());
        
        // Outline
        css.push_str(&Self::generate_outline());
        
        // Backdrop
        css.push_str(&Self::generate_backdrop());
        
        Ok(css)
    }
    
    fn generate_backgrounds(colors: &ColorsConfig) -> String {
        let mut css = String::new();
        css.push_str("\n  /* Background Colors */\n");
        
        // Brand colors (brand-a through brand-f)
        for letter in ['a', 'b', 'c', 'd', 'e', 'f'] {
            let color_name = format!("brand-{}", letter);
            css.push_str(&format!("  r-s[face*=\"bg:{}\"] {{ background-color: var(--rs-{}); }}\n", color_name, color_name));
            
            // Visual scope variations
            for variant in ["weak", "light", "intense", "bright", "strong"] {
                css.push_str(&format!("  r-s[face*=\"bg:{}-{}\"] {{ background-color: var(--rs-{}-{}); }}\n", 
                    color_name, variant, color_name, variant));
            }
        }
        
        // Base colors (neutrals) - base-0 to base-1000
        for value in [0, 50, 100, 200, 300, 400, 500, 600, 700, 800, 900, 1000] {
            css.push_str(&format!("  r-s[face*=\"bg:base-{}\"] {{ background-color: var(--rs-base-{}); }}\n", value, value));
        }
        
        // Semantic colors
        for semantic in ["success", "warning", "error", "info"] {
            css.push_str(&format!("  r-s[face*=\"bg:state-{}\"] {{ background-color: var(--rs-state-{}); }}\n", semantic, semantic));
        }
        
        // Gradients
        css.push_str("  r-s[face*=\"bg:gradient-primary\"] { background: linear-gradient(135deg, var(--rs-brand-a), var(--rs-brand-b)); }\n");
        css.push_str("  r-s[face*=\"bg:gradient-secondary\"] { background: linear-gradient(135deg, var(--rs-brand-b), var(--rs-brand-c)); }\n");
        css.push_str("  r-s[face*=\"bg:gradient-radial\"] { background: radial-gradient(circle, var(--rs-brand-a), var(--rs-brand-b)); }\n");
        
        css
    }
    
    fn generate_borders() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Borders */\n");
        
        // Simple borders
        css.push_str("  r-s[face*=\"border:none\"] { border: none; }\n");
        css.push_str("  r-s[face*=\"border:1\"] { border-width: 1px; border-style: solid; border-color: var(--rs-base-200); }\n");
        css.push_str("  r-s[face*=\"border:2\"] { border-width: 2px; border-style: solid; border-color: var(--rs-base-200); }\n");
        css.push_str("  r-s[face*=\"border:4\"] { border-width: 4px; border-style: solid; border-color: var(--rs-base-200); }\n");
        
        // Border colors
        for letter in ['a', 'b', 'c', 'd', 'e', 'f'] {
            let color = format!("brand-{}", letter);
            css.push_str(&format!("  r-s[face*=\"border:1:{}\"] {{ border: 1px solid var(--rs-{}); }}\n", color, color));
            css.push_str(&format!("  r-s[face*=\"border:2:{}\"] {{ border: 2px solid var(--rs-{}); }}\n", color, color));
        }
        
        // Semantic border colors
        for semantic in ["success", "warning", "error", "info"] {
            css.push_str(&format!("  r-s[face*=\"border:1:state-{}\"] {{ border: 1px solid var(--rs-state-{}); }}\n", semantic, semantic));
            css.push_str(&format!("  r-s[face*=\"border:2:state-{}\"] {{ border: 2px solid var(--rs-state-{}); }}\n", semantic, semantic));
        }
        
        // Border styles
        css.push_str("  r-s[face*=\"border:1:dashed\"] { border: 1px dashed var(--rs-base-400); }\n");
        css.push_str("  r-s[face*=\"border:1:dotted\"] { border: 1px dotted var(--rs-base-400); }\n");
        css.push_str("  r-s[face*=\"border:2:dashed\"] { border: 2px dashed var(--rs-base-400); }\n");
        css.push_str("  r-s[face*=\"border:2:dotted\"] { border: 2px dotted var(--rs-base-400); }\n");
        
        // Individual sides
        css.push_str("  r-s[face*=\"border-top:1\"] { border-top: 1px solid var(--rs-base-200); }\n");
        css.push_str("  r-s[face*=\"border-right:1\"] { border-right: 1px solid var(--rs-base-200); }\n");
        css.push_str("  r-s[face*=\"border-bottom:1\"] { border-bottom: 1px solid var(--rs-base-200); }\n");
        css.push_str("  r-s[face*=\"border-left:1\"] { border-left: 1px solid var(--rs-base-200); }\n");
        
        css.push_str("  r-s[face*=\"border-top:2\"] { border-top: 2px solid var(--rs-base-200); }\n");
        css.push_str("  r-s[face*=\"border-right:2\"] { border-right: 2px solid var(--rs-base-200); }\n");
        css.push_str("  r-s[face*=\"border-bottom:2\"] { border-bottom: 2px solid var(--rs-base-200); }\n");
        css.push_str("  r-s[face*=\"border-left:2\"] { border-left: 2px solid var(--rs-base-200); }\n");
        
        // Axis shortcuts
        css.push_str("  r-s[face*=\"border-x:1\"] { border-left: 1px solid var(--rs-base-200); border-right: 1px solid var(--rs-base-200); }\n");
        css.push_str("  r-s[face*=\"border-y:1\"] { border-top: 1px solid var(--rs-base-200); border-bottom: 1px solid var(--rs-base-200); }\n");
        css.push_str("  r-s[face*=\"border-x:2\"] { border-left: 2px solid var(--rs-base-200); border-right: 2px solid var(--rs-base-200); }\n");
        css.push_str("  r-s[face*=\"border-y:2\"] { border-top: 2px solid var(--rs-base-200); border-bottom: 2px solid var(--rs-base-200); }\n");
        
        css
    }
    
    fn generate_radius() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Border Radius */\n");
        
        css.push_str("  r-s[face*=\"radius:none\"] { border-radius: 0; }\n");
        css.push_str("  r-s[face*=\"radius:sm\"] { border-radius: 0.125rem; }\n");
        css.push_str("  r-s[face*=\"radius:md\"] { border-radius: 0.25rem; }\n");
        css.push_str("  r-s[face*=\"radius:lg\"] { border-radius: 0.5rem; }\n");
        css.push_str("  r-s[face*=\"radius:xl\"] { border-radius: 0.75rem; }\n");
        css.push_str("  r-s[face*=\"radius:2xl\"] { border-radius: 1rem; }\n");
        css.push_str("  r-s[face*=\"radius:3xl\"] { border-radius: 1.5rem; }\n");
        css.push_str("  r-s[face*=\"radius:full\"] { border-radius: 9999px; }\n");
        
        // Individual corners
        css.push_str("  r-s[face*=\"radius-tl:lg\"] { border-top-left-radius: 0.5rem; }\n");
        css.push_str("  r-s[face*=\"radius-tr:lg\"] { border-top-right-radius: 0.5rem; }\n");
        css.push_str("  r-s[face*=\"radius-bl:lg\"] { border-bottom-left-radius: 0.5rem; }\n");
        css.push_str("  r-s[face*=\"radius-br:lg\"] { border-bottom-right-radius: 0.5rem; }\n");
        
        css
    }
    
    fn generate_shadows() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Shadows */\n");
        
        css.push_str("  r-s[face*=\"shadow:none\"] { box-shadow: none; }\n");
        css.push_str("  r-s[face*=\"shadow:sm\"] { box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); }\n");
        css.push_str("  r-s[face*=\"shadow:md\"] { box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1); }\n");
        css.push_str("  r-s[face*=\"shadow:lg\"] { box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1); }\n");
        css.push_str("  r-s[face*=\"shadow:xl\"] { box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1); }\n");
        css.push_str("  r-s[face*=\"shadow:2xl\"] { box-shadow: 0 25px 50px -12px rgb(0 0 0 / 0.25); }\n");
        css.push_str("  r-s[face*=\"shadow:inner\"] { box-shadow: inset 0 2px 4px 0 rgb(0 0 0 / 0.05); }\n");
        
        // Colored shadows
        for letter in ['a', 'b', 'c'] {
            let color = format!("brand-{}", letter);
            css.push_str(&format!("  r-s[face*=\"shadow:lg:{}\"] {{ box-shadow: 0 10px 15px -3px var(--rs-{}-weak); }}\n", color, color));
            css.push_str(&format!("  r-s[face*=\"shadow:md:{}\"] {{ box-shadow: 0 4px 6px -1px var(--rs-{}-weak); }}\n", color, color));
        }
        
        css
    }
    
    fn generate_opacity() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Opacity */\n");
        
        css.push_str("  r-s[face*=\"opacity:0\"] { opacity: 0; }\n");
        css.push_str("  r-s[face*=\"opacity:10\"] { opacity: 0.1; }\n");
        css.push_str("  r-s[face*=\"opacity:25\"] { opacity: 0.25; }\n");
        css.push_str("  r-s[face*=\"opacity:50\"] { opacity: 0.5; }\n");
        css.push_str("  r-s[face*=\"opacity:75\"] { opacity: 0.75; }\n");
        css.push_str("  r-s[face*=\"opacity:90\"] { opacity: 0.9; }\n");
        css.push_str("  r-s[face*=\"opacity:100\"] { opacity: 1; }\n");
        
        css
    }
    
    fn generate_outline() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Outline */\n");
        
        css.push_str("  r-s[face*=\"outline:none\"] { outline: none; }\n");
        css.push_str("  r-s[face*=\"outline:1\"] { outline: 1px solid var(--rs-brand-a); }\n");
        css.push_str("  r-s[face*=\"outline:2\"] { outline: 2px solid var(--rs-brand-a); }\n");
        
        // Outline with colors
        for letter in ['a', 'b', 'c'] {
            let color = format!("brand-{}", letter);
            css.push_str(&format!("  r-s[face*=\"outline:1:{}\"] {{ outline: 1px solid var(--rs-{}); }}\n", color, color));
            css.push_str(&format!("  r-s[face*=\"outline:2:{}\"] {{ outline: 2px solid var(--rs-{}); }}\n", color, color));
        }
        
        // Outline offset
        css.push_str("  r-s[face*=\"outline-offset:2\"] { outline-offset: 2px; }\n");
        css.push_str("  r-s[face*=\"outline-offset:4\"] { outline-offset: 4px; }\n");
        
        css
    }
    
    fn generate_backdrop() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Backdrop */\n");
        
        css.push_str("  r-s[face*=\"backdrop:blur-sm\"] { backdrop-filter: blur(4px); -webkit-backdrop-filter: blur(4px); }\n");
        css.push_str("  r-s[face*=\"backdrop:blur-md\"] { backdrop-filter: blur(8px); -webkit-backdrop-filter: blur(8px); }\n");
        css.push_str("  r-s[face*=\"backdrop:blur-lg\"] { backdrop-filter: blur(16px); -webkit-backdrop-filter: blur(16px); }\n");
        
        css.push_str("  r-s[face*=\"backdrop:bright-50\"] { backdrop-filter: brightness(0.5); -webkit-backdrop-filter: brightness(0.5); }\n");
        css.push_str("  r-s[face*=\"backdrop:bright-75\"] { backdrop-filter: brightness(0.75); -webkit-backdrop-filter: brightness(0.75); }\n");
        css.push_str("  r-s[face*=\"backdrop:bright-125\"] { backdrop-filter: brightness(1.25); -webkit-backdrop-filter: brightness(1.25); }\n");
        
        css
    }
    
    pub fn generate_responsive(breakpoint: &str, min_width: &str) -> Result<String> {
        let mut css = String::new();
        
        css.push_str(&format!("\n  @media (min-width: {}) {{\n", min_width));
        css.push_str(&format!("    /* Face namespace - {} */\n", breakpoint));
        
        // Background responsive
        css.push_str(&format!("    r-s[face-{}*=\"bg:brand-a\"] {{ background-color: var(--rs-brand-a); }}\n", breakpoint));
        css.push_str(&format!("    r-s[face-{}*=\"bg:base-0\"] {{ background-color: var(--rs-base-0); }}\n", breakpoint));
        css.push_str(&format!("    r-s[face-{}*=\"bg:gradient-primary\"] {{ background: linear-gradient(135deg, var(--rs-brand-a), var(--rs-brand-b)); }}\n", breakpoint));
        
        // Border responsive
        css.push_str(&format!("    r-s[face-{}*=\"border:1\"] {{ border: 1px solid var(--rs-base-200); }}\n", breakpoint));
        css.push_str(&format!("    r-s[face-{}*=\"border:2\"] {{ border: 2px solid var(--rs-base-200); }}\n", breakpoint));
        css.push_str(&format!("    r-s[face-{}*=\"radius:lg\"] {{ border-radius: 0.5rem; }}\n", breakpoint));
        
        // Shadow responsive
        css.push_str(&format!("    r-s[face-{}*=\"shadow:md\"] {{ box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1); }}\n", breakpoint));
        css.push_str(&format!("    r-s[face-{}*=\"shadow:lg\"] {{ box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1); }}\n", breakpoint));
        
        css.push_str("  }\n");
        
        Ok(css)
    }
}