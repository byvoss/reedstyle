use anyhow::Result;
use crate::config::Config;

pub struct FxNamespace;

impl FxNamespace {
    pub fn generate(_config: &Config) -> Result<String> {
        let mut css = String::new();
        
        css.push_str("  /* ========== FX Namespace ========== */\n");
        
        // Keyframes definitions first
        css.push_str(&Self::generate_keyframes());
        
        // Transform
        css.push_str(&Self::generate_transform());
        
        // Transition
        css.push_str(&Self::generate_transition());
        
        // Animation
        css.push_str(&Self::generate_animation());
        
        // Filter
        css.push_str(&Self::generate_filter());
        
        // Hover effects
        css.push_str(&Self::generate_hover_effects());
        
        // Active effects
        css.push_str(&Self::generate_active_effects());
        
        Ok(css)
    }
    
    fn generate_keyframes() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Keyframes */\n");
        
        css.push_str("  @keyframes fade-in {\n");
        css.push_str("    from { opacity: 0; }\n");
        css.push_str("    to { opacity: 1; }\n");
        css.push_str("  }\n\n");
        
        css.push_str("  @keyframes fade-out {\n");
        css.push_str("    from { opacity: 1; }\n");
        css.push_str("    to { opacity: 0; }\n");
        css.push_str("  }\n\n");
        
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
        
        css.push_str("  @keyframes zoom-in {\n");
        css.push_str("    from { transform: scale(0.9); opacity: 0; }\n");
        css.push_str("    to { transform: scale(1); opacity: 1; }\n");
        css.push_str("  }\n\n");
        
        css.push_str("  @keyframes zoom-out {\n");
        css.push_str("    from { transform: scale(1.1); opacity: 0; }\n");
        css.push_str("    to { transform: scale(1); opacity: 1; }\n");
        css.push_str("  }\n\n");
        
        css.push_str("  @keyframes spin {\n");
        css.push_str("    from { transform: rotate(0deg); }\n");
        css.push_str("    to { transform: rotate(360deg); }\n");
        css.push_str("  }\n\n");
        
        css.push_str("  @keyframes pulse {\n");
        css.push_str("    0%, 100% { opacity: 1; }\n");
        css.push_str("    50% { opacity: 0.5; }\n");
        css.push_str("  }\n\n");
        
        css.push_str("  @keyframes bounce {\n");
        css.push_str("    0%, 100% { transform: translateY(0); }\n");
        css.push_str("    50% { transform: translateY(-10px); }\n");
        css.push_str("  }\n\n");
        
        css.push_str("  @keyframes shake {\n");
        css.push_str("    0%, 100% { transform: translateX(0); }\n");
        css.push_str("    25% { transform: translateX(-5px); }\n");
        css.push_str("    75% { transform: translateX(5px); }\n");
        css.push_str("  }\n\n");
        
        css
    }
    
    fn generate_transform() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Transform */\n");
        
        // Scale
        css.push_str("  reed[fx*=\"scale:0.9\"] { transform: scale(0.9); }\n");
        css.push_str("  reed[fx*=\"scale:0.95\"] { transform: scale(0.95); }\n");
        css.push_str("  reed[fx*=\"scale:1\"] { transform: scale(1); }\n");
        css.push_str("  reed[fx*=\"scale:1.05\"] { transform: scale(1.05); }\n");
        css.push_str("  reed[fx*=\"scale:1.1\"] { transform: scale(1.1); }\n");
        css.push_str("  reed[fx*=\"scale-x:1.5\"] { transform: scaleX(1.5); }\n");
        css.push_str("  reed[fx*=\"scale-y:0.5\"] { transform: scaleY(0.5); }\n");
        
        // Rotate
        css.push_str("  reed[fx*=\"rotate:45\"] { transform: rotate(45deg); }\n");
        css.push_str("  reed[fx*=\"rotate:90\"] { transform: rotate(90deg); }\n");
        css.push_str("  reed[fx*=\"rotate:180\"] { transform: rotate(180deg); }\n");
        css.push_str("  reed[fx*=\"rotate:-45\"] { transform: rotate(-45deg); }\n");
        css.push_str("  reed[fx*=\"rotate:5\"] { transform: rotate(5deg); }\n");
        
        // Translate (using spacing scale)
        css.push_str("  reed[fx*=\"translate-x:2\"] { transform: translateX(0.5rem); }\n");
        css.push_str("  reed[fx*=\"translate-x:4\"] { transform: translateX(1rem); }\n");
        css.push_str("  reed[fx*=\"translate-x:8\"] { transform: translateX(2rem); }\n");
        css.push_str("  reed[fx*=\"translate-x:-2\"] { transform: translateX(-0.5rem); }\n");
        css.push_str("  reed[fx*=\"translate-x:-4\"] { transform: translateX(-1rem); }\n");
        
        css.push_str("  reed[fx*=\"translate-y:2\"] { transform: translateY(0.5rem); }\n");
        css.push_str("  reed[fx*=\"translate-y:4\"] { transform: translateY(1rem); }\n");
        css.push_str("  reed[fx*=\"translate-y:8\"] { transform: translateY(2rem); }\n");
        css.push_str("  reed[fx*=\"translate-y:-2\"] { transform: translateY(-0.5rem); }\n");
        css.push_str("  reed[fx*=\"translate-y:-4\"] { transform: translateY(-1rem); }\n");
        
        // Skew
        css.push_str("  reed[fx*=\"skew-x:12\"] { transform: skewX(12deg); }\n");
        css.push_str("  reed[fx*=\"skew-y:12\"] { transform: skewY(12deg); }\n");
        
        // 3D Transforms
        css.push_str("  reed[fx*=\"rotate-x:45\"] { transform: rotateX(45deg); }\n");
        css.push_str("  reed[fx*=\"rotate-y:45\"] { transform: rotateY(45deg); }\n");
        css.push_str("  reed[fx*=\"perspective:1000\"] { perspective: 1000px; }\n");
        
        css
    }
    
    fn generate_transition() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Transition */\n");
        
        css.push_str("  reed[fx*=\"transition:none\"] { transition: none; }\n");
        css.push_str("  reed[fx*=\"transition:fast\"] { transition: all 150ms ease; }\n");
        css.push_str("  reed[fx*=\"transition:smooth\"] { transition: all 300ms ease; }\n");
        css.push_str("  reed[fx*=\"transition:slow\"] { transition: all 500ms ease; }\n");
        
        // Specific properties
        css.push_str("  reed[fx*=\"transition:opacity:300ms\"] { transition: opacity 300ms ease; }\n");
        css.push_str("  reed[fx*=\"transition:transform:500ms\"] { transition: transform 500ms ease; }\n");
        css.push_str("  reed[fx*=\"transition:all:300ms\"] { transition: all 300ms ease; }\n");
        
        // Easing functions
        css.push_str("  reed[fx*=\"transition:300ms:ease\"] { transition: all 300ms ease; }\n");
        css.push_str("  reed[fx*=\"transition:300ms:ease-in\"] { transition: all 300ms ease-in; }\n");
        css.push_str("  reed[fx*=\"transition:300ms:ease-out\"] { transition: all 300ms ease-out; }\n");
        css.push_str("  reed[fx*=\"transition:300ms:ease-in-out\"] { transition: all 300ms ease-in-out; }\n");
        css.push_str("  reed[fx*=\"transition:300ms:linear\"] { transition: all 300ms linear; }\n");
        
        css
    }
    
    fn generate_animation() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Animation */\n");
        
        css.push_str("  reed[fx*=\"animate:fade-in\"] { animation: fade-in 300ms ease-out; }\n");
        css.push_str("  reed[fx*=\"animate:fade-out\"] { animation: fade-out 300ms ease-out; }\n");
        css.push_str("  reed[fx*=\"animate:slide-up\"] { animation: slide-up 300ms ease-out; }\n");
        css.push_str("  reed[fx*=\"animate:slide-down\"] { animation: slide-down 300ms ease-out; }\n");
        css.push_str("  reed[fx*=\"animate:slide-left\"] { animation: slide-left 300ms ease-out; }\n");
        css.push_str("  reed[fx*=\"animate:slide-right\"] { animation: slide-right 300ms ease-out; }\n");
        css.push_str("  reed[fx*=\"animate:zoom-in\"] { animation: zoom-in 300ms ease-out; }\n");
        css.push_str("  reed[fx*=\"animate:zoom-out\"] { animation: zoom-out 300ms ease-out; }\n");
        css.push_str("  reed[fx*=\"animate:spin\"] { animation: spin 1s linear infinite; }\n");
        css.push_str("  reed[fx*=\"animate:pulse\"] { animation: pulse 2s ease-in-out infinite; }\n");
        css.push_str("  reed[fx*=\"animate:bounce\"] { animation: bounce 1s ease-in-out infinite; }\n");
        css.push_str("  reed[fx*=\"animate:shake\"] { animation: shake 0.5s ease-in-out; }\n");
        
        // Animation modifiers
        css.push_str("  reed[fx*=\"duration:1s\"] { animation-duration: 1s; }\n");
        css.push_str("  reed[fx*=\"duration:2s\"] { animation-duration: 2s; }\n");
        css.push_str("  reed[fx*=\"delay:200ms\"] { animation-delay: 200ms; }\n");
        css.push_str("  reed[fx*=\"delay:500ms\"] { animation-delay: 500ms; }\n");
        css.push_str("  reed[fx*=\"infinite\"] { animation-iteration-count: infinite; }\n");
        
        css
    }
    
    fn generate_filter() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Filter */\n");
        
        // Blur
        css.push_str("  reed[fx*=\"blur:sm\"] { filter: blur(4px); }\n");
        css.push_str("  reed[fx*=\"blur:md\"] { filter: blur(8px); }\n");
        css.push_str("  reed[fx*=\"blur:lg\"] { filter: blur(16px); }\n");
        
        // Brightness
        css.push_str("  reed[fx*=\"brightness:50\"] { filter: brightness(0.5); }\n");
        css.push_str("  reed[fx*=\"brightness:75\"] { filter: brightness(0.75); }\n");
        css.push_str("  reed[fx*=\"brightness:90\"] { filter: brightness(0.9); }\n");
        css.push_str("  reed[fx*=\"brightness:110\"] { filter: brightness(1.1); }\n");
        css.push_str("  reed[fx*=\"brightness:125\"] { filter: brightness(1.25); }\n");
        css.push_str("  reed[fx*=\"brightness:150\"] { filter: brightness(1.5); }\n");
        
        // Contrast
        css.push_str("  reed[fx*=\"contrast:50\"] { filter: contrast(0.5); }\n");
        css.push_str("  reed[fx*=\"contrast:150\"] { filter: contrast(1.5); }\n");
        
        // Grayscale
        css.push_str("  reed[fx*=\"grayscale:100\"] { filter: grayscale(100%); }\n");
        css.push_str("  reed[fx*=\"grayscale:50\"] { filter: grayscale(50%); }\n");
        
        // Sepia
        css.push_str("  reed[fx*=\"sepia:100\"] { filter: sepia(100%); }\n");
        css.push_str("  reed[fx*=\"sepia:50\"] { filter: sepia(50%); }\n");
        
        // Saturate
        css.push_str("  reed[fx*=\"saturate:0\"] { filter: saturate(0); }\n");
        css.push_str("  reed[fx*=\"saturate:150\"] { filter: saturate(1.5); }\n");
        
        // Hue Rotate
        css.push_str("  reed[fx*=\"hue:90\"] { filter: hue-rotate(90deg); }\n");
        css.push_str("  reed[fx*=\"hue:180\"] { filter: hue-rotate(180deg); }\n");
        
        // Invert
        css.push_str("  reed[fx*=\"invert:100\"] { filter: invert(100%); }\n");
        
        // Drop Shadow
        css.push_str("  reed[fx*=\"drop-shadow:md\"] { filter: drop-shadow(0 4px 6px rgb(0 0 0 / 0.1)); }\n");
        
        css
    }
    
    fn generate_hover_effects() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Hover Effects */\n");
        
        css.push_str("  reed[fx*=\"hover:lift\"]:hover { transform: translateY(-2px); box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1); }\n");
        css.push_str("  reed[fx*=\"hover:glow\"]:hover { box-shadow: 0 0 20px var(--rs-brand-a-weak); }\n");
        css.push_str("  reed[fx*=\"hover:grow\"]:hover { transform: scale(1.05); }\n");
        css.push_str("  reed[fx*=\"hover:shrink\"]:hover { transform: scale(0.95); }\n");
        css.push_str("  reed[fx*=\"hover:rotate\"]:hover { transform: rotate(5deg); }\n");
        css.push_str("  reed[fx*=\"hover:skew\"]:hover { transform: skew(5deg); }\n");
        css.push_str("  reed[fx*=\"hover:blur\"]:hover { filter: blur(2px); }\n");
        
        // Transform on hover
        css.push_str("  reed[fx*=\"hover:scale:1.05\"]:hover { transform: scale(1.05); }\n");
        css.push_str("  reed[fx*=\"hover:scale:1.1\"]:hover { transform: scale(1.1); }\n");
        css.push_str("  reed[fx*=\"hover:rotate:5\"]:hover { transform: rotate(5deg); }\n");
        css.push_str("  reed[fx*=\"hover:translate-y:-2\"]:hover { transform: translateY(-0.5rem); }\n");
        
        // Visual changes on hover
        css.push_str("  reed[fx*=\"hover:brightness:110\"]:hover { filter: brightness(1.1); }\n");
        css.push_str("  reed[fx*=\"hover:brightness:90\"]:hover { filter: brightness(0.9); }\n");
        css.push_str("  reed[fx*=\"hover:shadow:lg\"]:hover { box-shadow: 0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1); }\n");
        css.push_str("  reed[fx*=\"hover:shadow:xl\"]:hover { box-shadow: 0 20px 25px -5px rgb(0 0 0 / 0.1), 0 8px 10px -6px rgb(0 0 0 / 0.1); }\n");
        
        css
    }
    
    fn generate_active_effects() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Active Effects */\n");
        
        css.push_str("  reed[fx*=\"active:scale:0.95\"]:active { transform: scale(0.95); }\n");
        css.push_str("  reed[fx*=\"active:scale:0.9\"]:active { transform: scale(0.9); }\n");
        css.push_str("  reed[fx*=\"active:brightness:90\"]:active { filter: brightness(0.9); }\n");
        css.push_str("  reed[fx*=\"active:brightness:110\"]:active { filter: brightness(1.1); }\n");
        
        // Click effects (pseudo for active state)
        css.push_str("  reed[fx*=\"click:scale:0.95\"]:active { transform: scale(0.95); }\n");
        css.push_str("  reed[fx*=\"click:brightness:90\"]:active { filter: brightness(0.9); }\n");
        
        css
    }
    
    pub fn generate_responsive(breakpoint: &str, min_width: &str) -> Result<String> {
        let mut css = String::new();
        
        css.push_str(&format!("\n  @media (min-width: {}) {{\n", min_width));
        css.push_str(&format!("    /* FX namespace - {} */\n", breakpoint));
        
        // Transform responsive
        css.push_str(&format!("    reed[fx-{}*=\"scale:1.1\"] {{ transform: scale(1.1); }}\n", breakpoint));
        css.push_str(&format!("    reed[fx-{}*=\"rotate:45\"] {{ transform: rotate(45deg); }}\n", breakpoint));
        
        // Transition responsive
        css.push_str(&format!("    reed[fx-{}*=\"transition:smooth\"] {{ transition: all 300ms ease; }}\n", breakpoint));
        
        // Animation responsive
        css.push_str(&format!("    reed[fx-{}*=\"animate:fade-in\"] {{ animation: fade-in 300ms ease-out; }}\n", breakpoint));
        
        css.push_str("  }\n");
        
        // Add reduced motion support
        css.push_str("\n  @media (prefers-reduced-motion: reduce) {\n");
        css.push_str("    reed[fx*=\"animate\"] { animation: none !important; }\n");
        css.push_str("    reed[fx*=\"transition\"] { transition: none !important; }\n");
        css.push_str("  }\n");
        
        Ok(css)
    }
}