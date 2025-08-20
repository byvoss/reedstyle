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
        
        // Shadow levels (Visual scope)
        css.push_str(&Self::generate_shadow_levels());
        
        // Duration controls (Dimension scope)
        css.push_str(&Self::generate_duration_controls());
        
        // Glass/blur effects
        css.push_str(&Self::generate_blur_effects());
        
        // Scroll-triggered animations
        css.push_str(&Self::generate_scroll_animations());
        
        // Reduced motion support
        css.push_str(&Self::generate_reduced_motion());
        
        Ok(css)
    }
    
    fn generate_keyframes() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Keyframes */\n");
        
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
    
    fn generate_transform() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Transform */\n");
        
        // Scale
        css.push_str("  r-s[fx*=\"scale:0.9\"] { transform: scale(0.9); }\n");
        css.push_str("  r-s[fx*=\"scale:0.95\"] { transform: scale(0.95); }\n");
        css.push_str("  r-s[fx*=\"scale:1\"] { transform: scale(1); }\n");
        css.push_str("  r-s[fx*=\"scale:1.05\"] { transform: scale(1.05); }\n");
        css.push_str("  r-s[fx*=\"scale:1.1\"] { transform: scale(1.1); }\n");
        css.push_str("  r-s[fx*=\"scale-x:1.5\"] { transform: scaleX(1.5); }\n");
        css.push_str("  r-s[fx*=\"scale-y:0.5\"] { transform: scaleY(0.5); }\n");
        
        // Rotate
        css.push_str("  r-s[fx*=\"rotate:45\"] { transform: rotate(45deg); }\n");
        css.push_str("  r-s[fx*=\"rotate:90\"] { transform: rotate(90deg); }\n");
        css.push_str("  r-s[fx*=\"rotate:180\"] { transform: rotate(180deg); }\n");
        css.push_str("  r-s[fx*=\"rotate:-45\"] { transform: rotate(-45deg); }\n");
        css.push_str("  r-s[fx*=\"rotate:5\"] { transform: rotate(5deg); }\n");
        
        // Translate (using spacing scale)
        css.push_str("  r-s[fx*=\"translate-x:2\"] { transform: translateX(0.5rem); }\n");
        css.push_str("  r-s[fx*=\"translate-x:4\"] { transform: translateX(1rem); }\n");
        css.push_str("  r-s[fx*=\"translate-x:8\"] { transform: translateX(2rem); }\n");
        css.push_str("  r-s[fx*=\"translate-x:-2\"] { transform: translateX(-0.5rem); }\n");
        css.push_str("  r-s[fx*=\"translate-x:-4\"] { transform: translateX(-1rem); }\n");
        
        css.push_str("  r-s[fx*=\"translate-y:2\"] { transform: translateY(0.5rem); }\n");
        css.push_str("  r-s[fx*=\"translate-y:4\"] { transform: translateY(1rem); }\n");
        css.push_str("  r-s[fx*=\"translate-y:8\"] { transform: translateY(2rem); }\n");
        css.push_str("  r-s[fx*=\"translate-y:-2\"] { transform: translateY(-0.5rem); }\n");
        css.push_str("  r-s[fx*=\"translate-y:-4\"] { transform: translateY(-1rem); }\n");
        
        // Skew
        css.push_str("  r-s[fx*=\"skew-x:12\"] { transform: skewX(12deg); }\n");
        css.push_str("  r-s[fx*=\"skew-y:12\"] { transform: skewY(12deg); }\n");
        
        // 3D Transforms
        css.push_str("  r-s[fx*=\"rotate-x:45\"] { transform: rotateX(45deg); }\n");
        css.push_str("  r-s[fx*=\"rotate-y:45\"] { transform: rotateY(45deg); }\n");
        css.push_str("  r-s[fx*=\"perspective:1000\"] { perspective: 1000px; }\n");
        
        css
    }
    
    fn generate_transition() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Transition */\n");
        
        css.push_str("  r-s[fx*=\"transition:none\"] { transition: none; }\n");
        css.push_str("  r-s[fx*=\"transition:fast\"] { transition: all 150ms ease; }\n");
        css.push_str("  r-s[fx*=\"transition:smooth\"] { transition: all 300ms ease; }\n");
        css.push_str("  r-s[fx*=\"transition:slow\"] { transition: all 500ms ease; }\n");
        
        // Specific properties
        css.push_str("  r-s[fx*=\"transition:opacity:300ms\"] { transition: opacity 300ms ease; }\n");
        css.push_str("  r-s[fx*=\"transition:transform:500ms\"] { transition: transform 500ms ease; }\n");
        css.push_str("  r-s[fx*=\"transition:all:300ms\"] { transition: all 300ms ease; }\n");
        
        // Easing functions
        css.push_str("  r-s[fx*=\"transition:300ms:ease\"] { transition: all 300ms ease; }\n");
        css.push_str("  r-s[fx*=\"transition:300ms:ease-in\"] { transition: all 300ms ease-in; }\n");
        css.push_str("  r-s[fx*=\"transition:300ms:ease-out\"] { transition: all 300ms ease-out; }\n");
        css.push_str("  r-s[fx*=\"transition:300ms:ease-in-out\"] { transition: all 300ms ease-in-out; }\n");
        css.push_str("  r-s[fx*=\"transition:300ms:linear\"] { transition: all 300ms linear; }\n");
        
        css
    }
    
    fn generate_animation() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Animation */\n");
        
        css.push_str("  r-s[fx*=\"animate:fade-in\"] { animation: fade-in 300ms ease-out; }\n");
        css.push_str("  r-s[fx*=\"animate:fade-out\"] { animation: fade-out 300ms ease-out; }\n");
        css.push_str("  r-s[fx*=\"animate:slide-up\"] { animation: slide-up 300ms ease-out; }\n");
        css.push_str("  r-s[fx*=\"animate:slide-down\"] { animation: slide-down 300ms ease-out; }\n");
        css.push_str("  r-s[fx*=\"animate:slide-left\"] { animation: slide-left 300ms ease-out; }\n");
        css.push_str("  r-s[fx*=\"animate:slide-right\"] { animation: slide-right 300ms ease-out; }\n");
        css.push_str("  r-s[fx*=\"animate:zoom-in\"] { animation: zoom-in 300ms ease-out; }\n");
        css.push_str("  r-s[fx*=\"animate:zoom-out\"] { animation: zoom-out 300ms ease-out; }\n");
        css.push_str("  r-s[fx*=\"animate:spin\"] { animation: spin 1s linear infinite; }\n");
        css.push_str("  r-s[fx*=\"animate:pulse\"] { animation: pulse 2s ease-in-out infinite; }\n");
        css.push_str("  r-s[fx*=\"animate:bounce\"] { animation: bounce 1s ease-in-out infinite; }\n");
        css.push_str("  r-s[fx*=\"animate:shake\"] { animation: shake 0.5s ease-in-out; }\n");
        
        // Animation modifiers
        css.push_str("  r-s[fx*=\"duration:1s\"] { animation-duration: 1s; }\n");
        css.push_str("  r-s[fx*=\"duration:2s\"] { animation-duration: 2s; }\n");
        css.push_str("  r-s[fx*=\"delay:200ms\"] { animation-delay: 200ms; }\n");
        css.push_str("  r-s[fx*=\"delay:500ms\"] { animation-delay: 500ms; }\n");
        css.push_str("  r-s[fx*=\"infinite\"] { animation-iteration-count: infinite; }\n");
        
        css
    }
    
    fn generate_filter() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Filter */\n");
        
        // Blur
        css.push_str("  r-s[fx*=\"blur:sm\"] { filter: blur(4px); }\n");
        css.push_str("  r-s[fx*=\"blur:md\"] { filter: blur(8px); }\n");
        css.push_str("  r-s[fx*=\"blur:lg\"] { filter: blur(16px); }\n");
        
        // Brightness
        css.push_str("  r-s[fx*=\"brightness:50\"] { filter: brightness(0.5); }\n");
        css.push_str("  r-s[fx*=\"brightness:75\"] { filter: brightness(0.75); }\n");
        css.push_str("  r-s[fx*=\"brightness:90\"] { filter: brightness(0.9); }\n");
        css.push_str("  r-s[fx*=\"brightness:110\"] { filter: brightness(1.1); }\n");
        css.push_str("  r-s[fx*=\"brightness:125\"] { filter: brightness(1.25); }\n");
        css.push_str("  r-s[fx*=\"brightness:150\"] { filter: brightness(1.5); }\n");
        
        // Contrast
        css.push_str("  r-s[fx*=\"contrast:50\"] { filter: contrast(0.5); }\n");
        css.push_str("  r-s[fx*=\"contrast:150\"] { filter: contrast(1.5); }\n");
        
        // Grayscale
        css.push_str("  r-s[fx*=\"grayscale:100\"] { filter: grayscale(100%); }\n");
        css.push_str("  r-s[fx*=\"grayscale:50\"] { filter: grayscale(50%); }\n");
        
        // Sepia
        css.push_str("  r-s[fx*=\"sepia:100\"] { filter: sepia(100%); }\n");
        css.push_str("  r-s[fx*=\"sepia:50\"] { filter: sepia(50%); }\n");
        
        // Saturate
        css.push_str("  r-s[fx*=\"saturate:0\"] { filter: saturate(0); }\n");
        css.push_str("  r-s[fx*=\"saturate:150\"] { filter: saturate(1.5); }\n");
        
        // Hue Rotate
        css.push_str("  r-s[fx*=\"hue:90\"] { filter: hue-rotate(90deg); }\n");
        css.push_str("  r-s[fx*=\"hue:180\"] { filter: hue-rotate(180deg); }\n");
        
        // Invert
        css.push_str("  r-s[fx*=\"invert:100\"] { filter: invert(100%); }\n");
        
        // Drop Shadow
        css.push_str("  r-s[fx*=\"drop-shadow:md\"] { filter: drop-shadow(0 4px 6px rgb(0 0 0 / 0.1)); }\n");
        
        css
    }
    
    fn generate_hover_effects() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Hover Effects */\n");
        
        // Prepare for hover effects with will-change
        css.push_str("  r-s[fx*=\"hover:\"] { transition: all 200ms ease-out; will-change: transform; }\n\n");
        
        // Transform hover effects
        css.push_str("  r-s[fx*=\"hover:lift\"] { transition: transform 200ms ease-out, box-shadow 200ms ease-out; }\n");
        css.push_str("  r-s[fx*=\"hover:lift\"]:hover { transform: translateY(-4px) translateZ(0); box-shadow: 0 8px 16px rgba(0,0,0,0.15); }\n");
        css.push_str("  r-s[fx*=\"hover:sink\"]:hover { transform: translateY(2px); }\n");
        css.push_str("  r-s[fx*=\"hover:grow\"]:hover { transform: scale(1.05); }\n");
        css.push_str("  r-s[fx*=\"hover:shrink\"]:hover { transform: scale(0.95); }\n");
        css.push_str("  r-s[fx*=\"hover:rotate\"]:hover { transform: rotate(3deg); }\n");
        css.push_str("  r-s[fx*=\"hover:skew\"]:hover { transform: skew(5deg); }\n");
        css.push_str("  r-s[fx*=\"hover:flip\"]:hover { transform: perspective(400px) rotateY(180deg); }\n");
        
        // Visual hover effects
        css.push_str("  r-s[fx*=\"hover:glow\"]:hover { box-shadow: 0 0 20px rgba(var(--rs-brand-a), 0.5); }\n");
        css.push_str("  r-s[fx*=\"hover:blur\"]:hover { filter: blur(2px); }\n");
        css.push_str("  r-s[fx*=\"hover:brighten\"]:hover { filter: brightness(1.1); }\n");
        css.push_str("  r-s[fx*=\"hover:darken\"]:hover { filter: brightness(0.9); }\n");
        css.push_str("  r-s[fx*=\"hover:saturate\"]:hover { filter: saturate(1.5); }\n");
        css.push_str("  r-s[fx*=\"hover:desaturate\"]:hover { filter: saturate(0.5); }\n");
        
        // Combined hover effects
        css.push_str("  r-s[fx*=\"hover:lift-rotate\"]:hover { transform: translateY(-4px) rotate(2deg); }\n");
        css.push_str("  r-s[fx*=\"hover:grow-glow\"]:hover { transform: scale(1.05); box-shadow: 0 0 20px rgba(var(--rs-brand-a), 0.3); }\n");
        
        css
    }
    
    fn generate_active_effects() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Active/Click Effects */\n");
        
        // Scale effects on click
        css.push_str("  r-s[fx*=\"click:scale\"]:active { transform: scale(0.95); }\n");
        css.push_str("  r-s[fx*=\"click:scale-down\"]:active { transform: scale(0.9); }\n");
        css.push_str("  r-s[fx*=\"click:scale-up\"]:active { transform: scale(1.05); }\n");
        
        // Visual effects on click
        css.push_str("  r-s[fx*=\"click:brightness\"]:active { filter: brightness(0.9); }\n");
        css.push_str("  r-s[fx*=\"click:darken\"]:active { filter: brightness(0.8); }\n");
        css.push_str("  r-s[fx*=\"click:lighten\"]:active { filter: brightness(1.2); }\n");
        
        // Click animations
        css.push_str("  r-s[fx*=\"click:pulse\"]:active { animation: pulse 300ms ease-out; }\n");
        css.push_str("  r-s[fx*=\"click:bounce\"]:active { animation: bounce 300ms ease-out; }\n");
        css.push_str("  r-s[fx*=\"click:flash\"]:active { animation: flash 300ms ease-out; }\n");
        
        // Ripple effect placeholder (needs JS for positioning)
        css.push_str("  r-s[fx*=\"click:ripple\"] { position: relative; overflow: hidden; }\n");
        
        css
    }
    
    fn generate_shadow_levels() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Shadow Levels (Visual Scope) */\n");
        
        css.push_str("  r-s[fx*=\"shadow:weak\"] { box-shadow: 0 1px 2px rgba(0,0,0,0.05); }\n");
        css.push_str("  r-s[fx*=\"shadow:light\"] { box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
        css.push_str("  r-s[fx*=\"shadow:normal\"] { box-shadow: 0 4px 8px rgba(0,0,0,0.15); }\n");
        css.push_str("  r-s[fx*=\"shadow:intense\"] { box-shadow: 0 8px 16px rgba(0,0,0,0.2); }\n");
        css.push_str("  r-s[fx*=\"shadow:bright\"] { box-shadow: 0 12px 24px rgba(0,0,0,0.25); }\n");
        css.push_str("  r-s[fx*=\"shadow:strong\"] { box-shadow: 0 16px 32px rgba(0,0,0,0.3); }\n");
        
        css
    }
    
    fn generate_duration_controls() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Duration Controls (Dimension Scope) */\n");
        
        css.push_str("  r-s[fx*=\"duration:tiny\"] { animation-duration: 50ms; transition-duration: 50ms; }\n");
        css.push_str("  r-s[fx*=\"duration:small\"] { animation-duration: 100ms; transition-duration: 100ms; }\n");
        css.push_str("  r-s[fx*=\"duration:normal\"] { animation-duration: 200ms; transition-duration: 200ms; }\n");
        css.push_str("  r-s[fx*=\"duration:large\"] { animation-duration: 300ms; transition-duration: 300ms; }\n");
        css.push_str("  r-s[fx*=\"duration:huge\"] { animation-duration: 500ms; transition-duration: 500ms; }\n");
        css.push_str("  r-s[fx*=\"duration:mega\"] { animation-duration: 750ms; transition-duration: 750ms; }\n");
        css.push_str("  r-s[fx*=\"duration:ultra\"] { animation-duration: 1000ms; transition-duration: 1000ms; }\n");
        
        css
    }
    
    fn generate_blur_effects() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Glass/Blur Effects */\n");
        
        css.push_str("  r-s[fx*=\"blur:glass\"] {\n");
        css.push_str("    background: rgba(255, 255, 255, 0.1);\n");
        css.push_str("    backdrop-filter: blur(10px);\n");
        css.push_str("    -webkit-backdrop-filter: blur(10px);\n");
        css.push_str("    border: 1px solid rgba(255, 255, 255, 0.2);\n");
        css.push_str("  }\n\n");
        
        css.push_str("  r-s[fx*=\"blur:frost\"] {\n");
        css.push_str("    background: rgba(255, 255, 255, 0.5);\n");
        css.push_str("    backdrop-filter: blur(20px) saturate(180%);\n");
        css.push_str("    -webkit-backdrop-filter: blur(20px) saturate(180%);\n");
        css.push_str("  }\n\n");
        
        css.push_str("  r-s[fx*=\"blur:heavy\"] {\n");
        css.push_str("    backdrop-filter: blur(30px);\n");
        css.push_str("    -webkit-backdrop-filter: blur(30px);\n");
        css.push_str("  }\n\n");
        
        css
    }
    
    fn generate_scroll_animations() -> String {
        let mut css = String::new();
        css.push_str("\n  /* Scroll-triggered Animations */\n");
        
        css.push_str("  r-s[fx*=\"scroll:fade-in\"] { opacity: 0; }\n");
        css.push_str("  r-s[fx*=\"scroll:fade-in\"].in-view { animation: fade-in 300ms ease-out forwards; }\n\n");
        
        css.push_str("  r-s[fx*=\"scroll:slide-up\"] { opacity: 0; transform: translateY(20px); }\n");
        css.push_str("  r-s[fx*=\"scroll:slide-up\"].in-view { animation: slide-up 300ms ease-out forwards; }\n\n");
        
        css.push_str("  r-s[fx*=\"scroll:slide-down\"] { opacity: 0; transform: translateY(-20px); }\n");
        css.push_str("  r-s[fx*=\"scroll:slide-down\"].in-view { animation: slide-down 300ms ease-out forwards; }\n\n");
        
        css.push_str("  r-s[fx*=\"scroll:zoom-in\"] { opacity: 0; transform: scale(0.9); }\n");
        css.push_str("  r-s[fx*=\"scroll:zoom-in\"].in-view { animation: zoom-in 300ms ease-out forwards; }\n\n");
        
        css.push_str("  r-s[fx*=\"scroll:zoom-out\"] { opacity: 0; transform: scale(1.1); }\n");
        css.push_str("  r-s[fx*=\"scroll:zoom-out\"].in-view { animation: zoom-out 300ms ease-out forwards; }\n\n");
        
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
    
    pub fn generate_responsive(breakpoint: &str, min_width: &str) -> Result<String> {
        let mut css = String::new();
        
        css.push_str(&format!("\n  @media (min-width: {}) {{\n", min_width));
        css.push_str(&format!("    /* FX namespace - {} */\n", breakpoint));
        
        // Transform responsive
        css.push_str(&format!("    r-s[fx-{}*=\"scale:1.1\"] {{ transform: scale(1.1); }}\n", breakpoint));
        css.push_str(&format!("    r-s[fx-{}*=\"rotate:45\"] {{ transform: rotate(45deg); }}\n", breakpoint));
        
        // Transition responsive
        css.push_str(&format!("    r-s[fx-{}*=\"transition:smooth\"] {{ transition: all 300ms ease; }}\n", breakpoint));
        
        // Animation responsive
        css.push_str(&format!("    r-s[fx-{}*=\"animate:fade-in\"] {{ animation: fade-in 300ms ease-out; }}\n", breakpoint));
        
        css.push_str("  }\n");
        
        Ok(css)
    }
}