use anyhow::Result;
use crate::config::Config;
use crate::css::breakpoints::BREAKPOINTS;

pub struct FxNamespace;

impl FxNamespace {
    /// Generate FX namespace CSS for all breakpoints
    pub fn generate(_config: &Config) -> Result<String> {
        let mut css = String::new();
        
        css.push_str("  /* ========== FX Namespace (Responsive) ========== */\n");
        
        // Keyframes are defined only once (not per breakpoint)
        css.push_str(&Self::generate_keyframes());
        
        // Generate responsive styles for each breakpoint
        for (suffix, min_width) in BREAKPOINTS {
            let namespace = format!("fx{}", suffix);
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
        
        // Reduced motion support (applies to all breakpoints)
        css.push_str(&Self::generate_reduced_motion());
        
        Ok(css)
    }
    
    /// Generate all FX properties for a specific namespace
    fn generate_for_namespace(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("\n    /* {} */\n", namespace));
        
        // Generate all FX properties
        css.push_str(&Self::generate_transform(namespace));
        css.push_str(&Self::generate_transition(namespace));
        css.push_str(&Self::generate_animation(namespace));
        css.push_str(&Self::generate_filter(namespace));
        css.push_str(&Self::generate_hover_effects(namespace));
        css.push_str(&Self::generate_active_effects(namespace));
        css.push_str(&Self::generate_shadow_levels(namespace));
        css.push_str(&Self::generate_duration_controls(namespace));
        css.push_str(&Self::generate_blur_effects(namespace));
        css.push_str(&Self::generate_scroll_animations(namespace));
        
        css
    }
    
    fn generate_keyframes() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Keyframes (defined once for all breakpoints) */\n");
        
        // Fade animations
        css.push_str("  @keyframes fade-in {\n");
        css.push_str("    from { opacity: 0; }\n");
        css.push_str("    to { opacity: 1; }\n");
        css.push_str("  }\n\n");
        
        css.push_str("  @keyframes fade-out {\n");
        css.push_str("    from { opacity: 1; }\n");
        css.push_str("    to { opacity: 0; }\n");
        css.push_str("  }\n\n");
        
        // Slide animations
        css.push_str("  @keyframes slide-up {\n");
        css.push_str("    from { transform: translateY(20px); opacity: 0; }\n");
        css.push_str("    to { transform: translateY(0); opacity: 1; }\n");
        css.push_str("  }\n\n");
        
        css.push_str("  @keyframes slide-down {\n");
        css.push_str("    from { transform: translateY(-20px); opacity: 0; }\n");
        css.push_str("    to { transform: translateY(0); opacity: 1; }\n");
        css.push_str("  }\n\n");
        
        css.push_str("  @keyframes slide-left {\n");
        css.push_str("    from { transform: translateX(20px); opacity: 0; }\n");
        css.push_str("    to { transform: translateX(0); opacity: 1; }\n");
        css.push_str("  }\n\n");
        
        css.push_str("  @keyframes slide-right {\n");
        css.push_str("    from { transform: translateX(-20px); opacity: 0; }\n");
        css.push_str("    to { transform: translateX(0); opacity: 1; }\n");
        css.push_str("  }\n\n");
        
        // Zoom animations
        css.push_str("  @keyframes zoom-in {\n");
        css.push_str("    from { transform: scale(0.9); opacity: 0; }\n");
        css.push_str("    to { transform: scale(1); opacity: 1; }\n");
        css.push_str("  }\n\n");
        
        css.push_str("  @keyframes zoom-out {\n");
        css.push_str("    from { transform: scale(1.1); opacity: 0; }\n");
        css.push_str("    to { transform: scale(1); opacity: 1; }\n");
        css.push_str("  }\n\n");
        
        // Rotation and movement
        css.push_str("  @keyframes spin {\n");
        css.push_str("    from { transform: rotate(0deg); }\n");
        css.push_str("    to { transform: rotate(360deg); }\n");
        css.push_str("  }\n\n");
        
        css.push_str("  @keyframes pulse {\n");
        css.push_str("    0%, 100% { opacity: 1; transform: scale(1); }\n");
        css.push_str("    50% { opacity: 0.8; transform: scale(1.05); }\n");
        css.push_str("  }\n\n");
        
        css.push_str("  @keyframes bounce {\n");
        css.push_str("    0%, 100% { transform: translateY(0); animation-timing-function: cubic-bezier(0.8, 0, 1, 1); }\n");
        css.push_str("    50% { transform: translateY(-25%); animation-timing-function: cubic-bezier(0, 0, 0.2, 1); }\n");
        css.push_str("  }\n\n");
        
        css.push_str("  @keyframes shake {\n");
        css.push_str("    0%, 100% { transform: translateX(0); }\n");
        css.push_str("    10%, 30%, 50%, 70%, 90% { transform: translateX(-10px); }\n");
        css.push_str("    20%, 40%, 60%, 80% { transform: translateX(10px); }\n");
        css.push_str("  }\n\n");
        
        // Ripple effect for clicks
        css.push_str("  @keyframes ripple {\n");
        css.push_str("    to {\n");
        css.push_str("      transform: scale(4);\n");
        css.push_str("      opacity: 0;\n");
        css.push_str("    }\n");
        css.push_str("  }\n\n");
        
        // Flash effect
        css.push_str("  @keyframes flash {\n");
        css.push_str("    0%, 50%, 100% { opacity: 1; }\n");
        css.push_str("    25%, 75% { opacity: 0; }\n");
        css.push_str("  }\n\n");
        
        // 3D Flip
        css.push_str("  @keyframes flip {\n");
        css.push_str("    from { transform: perspective(400px) rotateY(0); }\n");
        css.push_str("    to { transform: perspective(400px) rotateY(360deg); }\n");
        css.push_str("  }\n\n");
        
        css
    }
    
    fn generate_transform(namespace: &str) -> String {
        let mut css = String::new();
        
        // Scale
        css.push_str(&format!("    r-s[{}*=\"scale:0.9\"] {{ transform: scale(0.9); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"scale:0.95\"] {{ transform: scale(0.95); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"scale:1\"] {{ transform: scale(1); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"scale:1.05\"] {{ transform: scale(1.05); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"scale:1.1\"] {{ transform: scale(1.1); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"scale-x:1.5\"] {{ transform: scaleX(1.5); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"scale-y:0.5\"] {{ transform: scaleY(0.5); }}\n", namespace));
        
        // Rotate
        css.push_str(&format!("    r-s[{}*=\"rotate:45\"] {{ transform: rotate(45deg); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"rotate:90\"] {{ transform: rotate(90deg); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"rotate:180\"] {{ transform: rotate(180deg); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"rotate:-45\"] {{ transform: rotate(-45deg); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"rotate:5\"] {{ transform: rotate(5deg); }}\n", namespace));
        
        // Translate (using spacing scale)
        css.push_str(&format!("    r-s[{}*=\"translate-x:2\"] {{ transform: translateX(0.5rem); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"translate-x:4\"] {{ transform: translateX(1rem); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"translate-x:8\"] {{ transform: translateX(2rem); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"translate-x:-2\"] {{ transform: translateX(-0.5rem); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"translate-x:-4\"] {{ transform: translateX(-1rem); }}\n", namespace));
        
        css.push_str(&format!("    r-s[{}*=\"translate-y:2\"] {{ transform: translateY(0.5rem); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"translate-y:4\"] {{ transform: translateY(1rem); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"translate-y:8\"] {{ transform: translateY(2rem); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"translate-y:-2\"] {{ transform: translateY(-0.5rem); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"translate-y:-4\"] {{ transform: translateY(-1rem); }}\n", namespace));
        
        // Skew
        css.push_str(&format!("    r-s[{}*=\"skew-x:12\"] {{ transform: skewX(12deg); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"skew-y:12\"] {{ transform: skewY(12deg); }}\n", namespace));
        
        // 3D Transforms
        css.push_str(&format!("    r-s[{}*=\"rotate-x:45\"] {{ transform: rotateX(45deg); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"rotate-y:45\"] {{ transform: rotateY(45deg); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"perspective:1000\"] {{ perspective: 1000px; }}\n", namespace));
        
        css
    }
    
    fn generate_transition(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"transition:none\"] {{ transition: none; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"transition:fast\"] {{ transition: all 150ms ease; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"transition:smooth\"] {{ transition: all 300ms ease; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"transition:slow\"] {{ transition: all 500ms ease; }}\n", namespace));
        
        // Specific properties
        css.push_str(&format!("    r-s[{}*=\"transition:opacity:300ms\"] {{ transition: opacity 300ms ease; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"transition:transform:500ms\"] {{ transition: transform 500ms ease; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"transition:all:300ms\"] {{ transition: all 300ms ease; }}\n", namespace));
        
        // Easing functions
        css.push_str(&format!("    r-s[{}*=\"transition:300ms:ease\"] {{ transition: all 300ms ease; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"transition:300ms:ease-in\"] {{ transition: all 300ms ease-in; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"transition:300ms:ease-out\"] {{ transition: all 300ms ease-out; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"transition:300ms:ease-in-out\"] {{ transition: all 300ms ease-in-out; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"transition:300ms:linear\"] {{ transition: all 300ms linear; }}\n", namespace));
        
        css
    }
    
    fn generate_animation(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"animate:fade-in\"] {{ animation: fade-in 300ms ease-out; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"animate:fade-out\"] {{ animation: fade-out 300ms ease-out; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"animate:slide-up\"] {{ animation: slide-up 300ms ease-out; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"animate:slide-down\"] {{ animation: slide-down 300ms ease-out; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"animate:slide-left\"] {{ animation: slide-left 300ms ease-out; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"animate:slide-right\"] {{ animation: slide-right 300ms ease-out; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"animate:zoom-in\"] {{ animation: zoom-in 300ms ease-out; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"animate:zoom-out\"] {{ animation: zoom-out 300ms ease-out; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"animate:spin\"] {{ animation: spin 1s linear infinite; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"animate:pulse\"] {{ animation: pulse 2s ease-in-out infinite; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"animate:bounce\"] {{ animation: bounce 1s ease-in-out infinite; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"animate:shake\"] {{ animation: shake 0.5s ease-in-out; }}\n", namespace));
        
        // Animation modifiers
        css.push_str(&format!("    r-s[{}*=\"duration:1s\"] {{ animation-duration: 1s; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"duration:2s\"] {{ animation-duration: 2s; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"delay:200ms\"] {{ animation-delay: 200ms; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"delay:500ms\"] {{ animation-delay: 500ms; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"infinite\"] {{ animation-iteration-count: infinite; }}\n", namespace));
        
        css
    }
    
    fn generate_filter(namespace: &str) -> String {
        let mut css = String::new();
        
        // Blur
        css.push_str(&format!("    r-s[{}*=\"blur:sm\"] {{ filter: blur(4px); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"blur:md\"] {{ filter: blur(8px); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"blur:lg\"] {{ filter: blur(16px); }}\n", namespace));
        
        // Brightness
        css.push_str(&format!("    r-s[{}*=\"brightness:50\"] {{ filter: brightness(0.5); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"brightness:75\"] {{ filter: brightness(0.75); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"brightness:90\"] {{ filter: brightness(0.9); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"brightness:110\"] {{ filter: brightness(1.1); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"brightness:125\"] {{ filter: brightness(1.25); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"brightness:150\"] {{ filter: brightness(1.5); }}\n", namespace));
        
        // Contrast
        css.push_str(&format!("    r-s[{}*=\"contrast:50\"] {{ filter: contrast(0.5); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"contrast:150\"] {{ filter: contrast(1.5); }}\n", namespace));
        
        // Grayscale
        css.push_str(&format!("    r-s[{}*=\"grayscale:100\"] {{ filter: grayscale(100%); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"grayscale:50\"] {{ filter: grayscale(50%); }}\n", namespace));
        
        // Sepia
        css.push_str(&format!("    r-s[{}*=\"sepia:100\"] {{ filter: sepia(100%); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"sepia:50\"] {{ filter: sepia(50%); }}\n", namespace));
        
        // Saturate
        css.push_str(&format!("    r-s[{}*=\"saturate:0\"] {{ filter: saturate(0); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"saturate:150\"] {{ filter: saturate(1.5); }}\n", namespace));
        
        // Hue Rotate
        css.push_str(&format!("    r-s[{}*=\"hue:90\"] {{ filter: hue-rotate(90deg); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"hue:180\"] {{ filter: hue-rotate(180deg); }}\n", namespace));
        
        // Invert
        css.push_str(&format!("    r-s[{}*=\"invert:100\"] {{ filter: invert(100%); }}\n", namespace));
        
        // Drop Shadow
        css.push_str(&format!("    r-s[{}*=\"drop-shadow:md\"] {{ filter: drop-shadow(0 4px 6px rgb(0 0 0 / 0.1)); }}\n", namespace));
        
        css
    }
    
    fn generate_hover_effects(namespace: &str) -> String {
        let mut css = String::new();
        
        // Prepare for hover effects with will-change
        css.push_str(&format!("    r-s[{}*=\"hover:\"] {{ transition: all 200ms ease-out; will-change: transform; }}\n\n", namespace));
        
        // Transform hover effects
        css.push_str(&format!("    r-s[{}*=\"hover:lift\"] {{ transition: transform 200ms ease-out, box-shadow 200ms ease-out; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"hover:lift\"]:hover {{ transform: translateY(-4px) translateZ(0); box-shadow: 0 8px 16px rgba(0,0,0,0.15); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"hover:sink\"]:hover {{ transform: translateY(2px); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"hover:grow\"]:hover {{ transform: scale(1.05); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"hover:shrink\"]:hover {{ transform: scale(0.95); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"hover:rotate\"]:hover {{ transform: rotate(3deg); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"hover:skew\"]:hover {{ transform: skew(5deg); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"hover:flip\"]:hover {{ transform: perspective(400px) rotateY(180deg); }}\n", namespace));
        
        // Visual hover effects
        css.push_str(&format!("    r-s[{}*=\"hover:glow\"]:hover {{ box-shadow: 0 0 20px rgba(var(--rs-brand-a), 0.5); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"hover:blur\"]:hover {{ filter: blur(2px); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"hover:brighten\"]:hover {{ filter: brightness(1.1); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"hover:darken\"]:hover {{ filter: brightness(0.9); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"hover:saturate\"]:hover {{ filter: saturate(1.5); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"hover:desaturate\"]:hover {{ filter: saturate(0.5); }}\n", namespace));
        
        // Combined hover effects
        css.push_str(&format!("    r-s[{}*=\"hover:lift-rotate\"]:hover {{ transform: translateY(-4px) rotate(2deg); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"hover:grow-glow\"]:hover {{ transform: scale(1.05); box-shadow: 0 0 20px rgba(var(--rs-brand-a), 0.3); }}\n", namespace));
        
        css
    }
    
    fn generate_active_effects(namespace: &str) -> String {
        let mut css = String::new();
        
        // Scale effects on click
        css.push_str(&format!("    r-s[{}*=\"click:scale\"]:active {{ transform: scale(0.95); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"click:scale-down\"]:active {{ transform: scale(0.9); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"click:scale-up\"]:active {{ transform: scale(1.05); }}\n", namespace));
        
        // Visual effects on click
        css.push_str(&format!("    r-s[{}*=\"click:brightness\"]:active {{ filter: brightness(0.9); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"click:darken\"]:active {{ filter: brightness(0.8); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"click:lighten\"]:active {{ filter: brightness(1.2); }}\n", namespace));
        
        // Click animations
        css.push_str(&format!("    r-s[{}*=\"click:pulse\"]:active {{ animation: pulse 300ms ease-out; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"click:bounce\"]:active {{ animation: bounce 300ms ease-out; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"click:flash\"]:active {{ animation: flash 300ms ease-out; }}\n", namespace));
        
        // Ripple effect placeholder (needs JS for positioning)
        css.push_str(&format!("    r-s[{}*=\"click:ripple\"] {{ position: relative; overflow: hidden; }}\n", namespace));
        
        css
    }
    
    fn generate_shadow_levels(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"shadow:weak\"] {{ box-shadow: 0 1px 2px rgba(0,0,0,0.05); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"shadow:light\"] {{ box-shadow: 0 2px 4px rgba(0,0,0,0.1); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"shadow:normal\"] {{ box-shadow: 0 4px 8px rgba(0,0,0,0.15); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"shadow:intense\"] {{ box-shadow: 0 8px 16px rgba(0,0,0,0.2); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"shadow:bright\"] {{ box-shadow: 0 12px 24px rgba(0,0,0,0.25); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"shadow:strong\"] {{ box-shadow: 0 16px 32px rgba(0,0,0,0.3); }}\n", namespace));
        
        css
    }
    
    fn generate_duration_controls(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"duration:tiny\"] {{ animation-duration: 50ms; transition-duration: 50ms; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"duration:small\"] {{ animation-duration: 100ms; transition-duration: 100ms; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"duration:normal\"] {{ animation-duration: 200ms; transition-duration: 200ms; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"duration:large\"] {{ animation-duration: 300ms; transition-duration: 300ms; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"duration:huge\"] {{ animation-duration: 500ms; transition-duration: 500ms; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"duration:mega\"] {{ animation-duration: 750ms; transition-duration: 750ms; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"duration:ultra\"] {{ animation-duration: 1000ms; transition-duration: 1000ms; }}\n", namespace));
        
        css
    }
    
    fn generate_blur_effects(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"blur:glass\"] {{\n", namespace));
        css.push_str("      background: rgba(255, 255, 255, 0.1);\n");
        css.push_str("      backdrop-filter: blur(10px);\n");
        css.push_str("      -webkit-backdrop-filter: blur(10px);\n");
        css.push_str("      border: 1px solid rgba(255, 255, 255, 0.2);\n");
        css.push_str("    }\n\n");
        
        css.push_str(&format!("    r-s[{}*=\"blur:frost\"] {{\n", namespace));
        css.push_str("      background: rgba(255, 255, 255, 0.5);\n");
        css.push_str("      backdrop-filter: blur(20px) saturate(180%);\n");
        css.push_str("      -webkit-backdrop-filter: blur(20px) saturate(180%);\n");
        css.push_str("    }\n\n");
        
        css.push_str(&format!("    r-s[{}*=\"blur:heavy\"] {{\n", namespace));
        css.push_str("      backdrop-filter: blur(30px);\n");
        css.push_str("      -webkit-backdrop-filter: blur(30px);\n");
        css.push_str("    }\n\n");
        
        css
    }
    
    fn generate_scroll_animations(namespace: &str) -> String {
        let mut css = String::new();
        
        css.push_str(&format!("    r-s[{}*=\"scroll:fade-in\"] {{ opacity: 0; }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"scroll:fade-in\"].in-view {{ animation: fade-in 300ms ease-out forwards; }}\n\n", namespace));
        
        css.push_str(&format!("    r-s[{}*=\"scroll:slide-up\"] {{ opacity: 0; transform: translateY(20px); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"scroll:slide-up\"].in-view {{ animation: slide-up 300ms ease-out forwards; }}\n\n", namespace));
        
        css.push_str(&format!("    r-s[{}*=\"scroll:slide-down\"] {{ opacity: 0; transform: translateY(-20px); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"scroll:slide-down\"].in-view {{ animation: slide-down 300ms ease-out forwards; }}\n\n", namespace));
        
        css.push_str(&format!("    r-s[{}*=\"scroll:zoom-in\"] {{ opacity: 0; transform: scale(0.9); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"scroll:zoom-in\"].in-view {{ animation: zoom-in 300ms ease-out forwards; }}\n\n", namespace));
        
        css.push_str(&format!("    r-s[{}*=\"scroll:zoom-out\"] {{ opacity: 0; transform: scale(1.1); }}\n", namespace));
        css.push_str(&format!("    r-s[{}*=\"scroll:zoom-out\"].in-view {{ animation: zoom-out 300ms ease-out forwards; }}\n\n", namespace));
        
        css
    }
    
    fn generate_reduced_motion() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Reduced Motion Support */\n");
        css.push_str("  @media (prefers-reduced-motion: reduce) {\n");
        css.push_str("    r-s[fx*=\"animate\"],\n");
        css.push_str("    r-s[fx*=\"duration\"],\n");
        css.push_str("    r-s[fx*=\"scroll:\"] {\n");
        css.push_str("      animation-duration: 0.01ms !important;\n");
        css.push_str("      animation-iteration-count: 1 !important;\n");
        css.push_str("      transition-duration: 0.01ms !important;\n");
        css.push_str("      scroll-behavior: auto !important;\n");
        css.push_str("    }\n\n");
        
        css.push_str("    r-s[fx*=\"scroll:\"] {\n");
        css.push_str("      opacity: 1 !important;\n");
        css.push_str("      transform: none !important;\n");
        css.push_str("    }\n");
        css.push_str("  }\n");
        
        css
    }
}