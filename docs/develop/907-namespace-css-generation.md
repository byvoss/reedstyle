# Ticket #907: Namespace CSS Generation

## Status: 🚧 In Progress

## Description
Generate complete CSS for all 6 ReedSTYLE namespaces with proper attribute selectors for the `<reed>` element.

## Requirements

### Namespace Structure

1. **Box** - Spacing, sizing, display
2. **Device** - User interactions, states
3. **Face** - Visual appearance, colors
4. **FX** - Effects and animations
5. **Layout** - Flex and Grid arrangements
6. **Text** - Typography properties

### CSS Selector Strategy

```css
/* Base namespace */
reed[box*="padding:4"] { }

/* Breakpoint variant */
reed[box-tablet*="padding:6"] { }

/* Environment variant */
reed[box-prod*="padding:2"] { }

/* Environment + Breakpoint */
reed[box-prod-tablet*="padding:3"] { }
```

### Implementation per Namespace

#### Box Namespace (box.rs)
```rust
pub fn generate(config: &Config) -> String {
    let mut css = String::new();
    let scale = &config.spacing_scale;
    
    // Padding (all directions)
    for (size, value) in scale {
        // All sides
        css.push_str(&format!(
            r#"reed[box*="padding:{}"] {{ padding: {}; }}
"#,
            size, value
        ));
        
        // Horizontal
        css.push_str(&format!(
            r#"reed[box*="padding-x:{}"] {{ padding-left: {}; padding-right: {}; }}
"#,
            size, value, value
        ));
        
        // Vertical
        css.push_str(&format!(
            r#"reed[box*="padding-y:{}"] {{ padding-top: {}; padding-bottom: {}; }}
"#,
            size, value, value
        ));
        
        // Individual sides
        css.push_str(&format!(
            r#"reed[box*="padding-top:{}"] {{ padding-top: {}; }}
"#,
            size, value
        ));
        // ... left, right, bottom
    }
    
    // Margin (same pattern as padding)
    
    // Width/Height
    css.push_str(r#"
reed[box*="width:full"] { width: 100%; }
reed[box*="width:screen"] { width: 100vw; }
reed[box*="width:auto"] { width: auto; }
reed[box*="height:full"] { height: 100%; }
reed[box*="height:screen"] { height: 100vh; }
reed[box*="height:auto"] { height: auto; }
"#);
    
    // Display
    css.push_str(r#"
reed[box*="display:block"] { display: block; }
reed[box*="display:inline"] { display: inline; }
reed[box*="display:inline-block"] { display: inline-block; }
reed[box*="display:none"] { display: none; }
reed[box*="display:flex"] { display: flex; }
reed[box*="display:grid"] { display: grid; }
"#);
    
    // Position
    css.push_str(r#"
reed[box*="position:relative"] { position: relative; }
reed[box*="position:absolute"] { position: absolute; }
reed[box*="position:fixed"] { position: fixed; }
reed[box*="position:sticky"] { position: sticky; }
"#);
    
    // Overflow
    css.push_str(r#"
reed[box*="overflow:hidden"] { overflow: hidden; }
reed[box*="overflow:auto"] { overflow: auto; }
reed[box*="overflow:scroll"] { overflow: scroll; }
reed[box*="overflow:visible"] { overflow: visible; }
reed[box*="overflow-x:auto"] { overflow-x: auto; }
reed[box*="overflow-y:auto"] { overflow-y: auto; }
"#);
    
    css
}
```

#### Layout Namespace (layout.rs)
```rust
pub fn generate(config: &Config) -> String {
    let mut css = String::new();
    
    // Flex layouts
    css.push_str(r#"
/* Flex Direction */
reed[layout*="flex:row"] { display: flex; flex-direction: row; }
reed[layout*="flex:column"] { display: flex; flex-direction: column; }
reed[layout*="flex:row-reverse"] { display: flex; flex-direction: row-reverse; }
reed[layout*="flex:column-reverse"] { display: flex; flex-direction: column-reverse; }

/* Flex Wrap */
reed[layout*="wrap:wrap"] { flex-wrap: wrap; }
reed[layout*="wrap:nowrap"] { flex-wrap: nowrap; }
reed[layout*="wrap:reverse"] { flex-wrap: wrap-reverse; }

/* Justify Content */
reed[layout*="justify:start"] { justify-content: flex-start; }
reed[layout*="justify:end"] { justify-content: flex-end; }
reed[layout*="justify:center"] { justify-content: center; }
reed[layout*="justify:between"] { justify-content: space-between; }
reed[layout*="justify:around"] { justify-content: space-around; }
reed[layout*="justify:evenly"] { justify-content: space-evenly; }

/* Align Items */
reed[layout*="align:start"] { align-items: flex-start; }
reed[layout*="align:end"] { align-items: flex-end; }
reed[layout*="align:center"] { align-items: center; }
reed[layout*="align:stretch"] { align-items: stretch; }
reed[layout*="align:baseline"] { align-items: baseline; }
"#);
    
    // Grid layouts
    css.push_str(r#"
/* Grid Columns */
reed[layout*="grid:1"] { display: grid; grid-template-columns: 1fr; }
reed[layout*="grid:2"] { display: grid; grid-template-columns: repeat(2, 1fr); }
reed[layout*="grid:3"] { display: grid; grid-template-columns: repeat(3, 1fr); }
reed[layout*="grid:4"] { display: grid; grid-template-columns: repeat(4, 1fr); }
reed[layout*="grid:5"] { display: grid; grid-template-columns: repeat(5, 1fr); }
reed[layout*="grid:6"] { display: grid; grid-template-columns: repeat(6, 1fr); }
reed[layout*="grid:12"] { display: grid; grid-template-columns: repeat(12, 1fr); }
"#);
    
    // Gap (for both flex and grid)
    for (size, value) in &config.spacing_scale {
        css.push_str(&format!(
            r#"reed[layout*="gap:{}"] {{ gap: {}; }}
"#,
            size, value
        ));
    }
    
    // Stack (vertical flex with gap)
    css.push_str(r#"
reed[layout*="stack"] { display: flex; flex-direction: column; }
reed[layout*="chain"] { display: flex; flex-direction: row; flex-wrap: wrap; }
"#);
    
    css
}
```

#### Face Namespace (face.rs)
```rust
pub fn generate(config: &Config) -> String {
    let mut css = String::new();
    
    // Background colors
    for (name, color) in &config.colors {
        // Base color
        css.push_str(&format!(
            r#"reed[face*="bg:{}"] {{ background: {}; }}
"#,
            name, color.to_css()
        ));
        
        // Variations (if generated)
        if config.features.generate_variations {
            for variant in ["weak", "light", "intense", "bright", "strong"] {
                let varied = color.variant(variant);
                css.push_str(&format!(
                    r#"reed[face*="bg:{}-{}"] {{ background: {}; }}
"#,
                    name, variant, varied.to_css()
                ));
            }
        }
    }
    
    // Border
    css.push_str(r#"
reed[face*="border:none"] { border: none; }
reed[face*="border:1"] { border-width: 1px; border-style: solid; }
reed[face*="border:2"] { border-width: 2px; border-style: solid; }
reed[face*="border:4"] { border-width: 4px; border-style: solid; }
"#);
    
    // Border radius
    css.push_str(r#"
reed[face*="radius:none"] { border-radius: 0; }
reed[face*="radius:sm"] { border-radius: 0.125rem; }
reed[face*="radius:md"] { border-radius: 0.375rem; }
reed[face*="radius:lg"] { border-radius: 0.5rem; }
reed[face*="radius:xl"] { border-radius: 1rem; }
reed[face*="radius:full"] { border-radius: 9999px; }
"#);
    
    // Shadow
    css.push_str(r#"
reed[face*="shadow:none"] { box-shadow: none; }
reed[face*="shadow:sm"] { box-shadow: 0 1px 2px rgba(0,0,0,0.05); }
reed[face*="shadow:md"] { box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
reed[face*="shadow:lg"] { box-shadow: 0 10px 15px rgba(0,0,0,0.1); }
reed[face*="shadow:xl"] { box-shadow: 0 20px 25px rgba(0,0,0,0.1); }
"#);
    
    // Opacity
    css.push_str(r#"
reed[face*="opacity:0"] { opacity: 0; }
reed[face*="opacity:25"] { opacity: 0.25; }
reed[face*="opacity:50"] { opacity: 0.5; }
reed[face*="opacity:75"] { opacity: 0.75; }
reed[face*="opacity:100"] { opacity: 1; }
"#);
    
    css
}
```

#### Text Namespace (text.rs)
```rust
pub fn generate(config: &Config) -> String {
    let mut css = String::new();
    
    // Font families
    for (name, font) in &config.fonts {
        css.push_str(&format!(
            r#"reed[text*="font:{}"] {{ font-family: {}; }}
"#,
            name, font.stack
        ));
    }
    
    // Font sizes (dimension scale)
    css.push_str(r#"
reed[text*="size:tiny"] { font-size: 0.75rem; }
reed[text*="size:small"] { font-size: 0.875rem; }
reed[text*="size:normal"] { font-size: 1rem; }
reed[text*="size:large"] { font-size: 1.25rem; }
reed[text*="size:huge"] { font-size: 1.5rem; }
reed[text*="size:mega"] { font-size: 2rem; }
reed[text*="size:ultra"] { font-size: 3rem; }
"#);
    
    // Font weight (typography scale)
    css.push_str(r#"
reed[text*="weight:thin"] { font-weight: 100; }
reed[text*="weight:light"] { font-weight: 300; }
reed[text*="weight:regular"] { font-weight: 400; }
reed[text*="weight:medium"] { font-weight: 500; }
reed[text*="weight:bold"] { font-weight: 700; }
reed[text*="weight:black"] { font-weight: 900; }
"#);
    
    // Text alignment
    css.push_str(r#"
reed[text*="align:left"] { text-align: left; }
reed[text*="align:center"] { text-align: center; }
reed[text*="align:right"] { text-align: right; }
reed[text*="align:justify"] { text-align: justify; }
"#);
    
    // Line height
    css.push_str(r#"
reed[text*="leading:tight"] { line-height: 1.25; }
reed[text*="leading:normal"] { line-height: 1.5; }
reed[text*="leading:relaxed"] { line-height: 1.75; }
reed[text*="leading:loose"] { line-height: 2; }
"#);
    
    // Text color
    for (name, color) in &config.colors {
        css.push_str(&format!(
            r#"reed[text*="color:{}"] {{ color: {}; }}
"#,
            name, color.to_css()
        ));
    }
    
    css
}
```

#### FX Namespace (fx.rs)
```rust
pub fn generate(config: &Config) -> String {
    let mut css = String::new();
    
    // Transitions
    css.push_str(r#"
reed[fx*="transition:none"] { transition: none; }
reed[fx*="transition:fast"] { transition: all 150ms ease; }
reed[fx*="transition:smooth"] { transition: all 300ms ease; }
reed[fx*="transition:slow"] { transition: all 500ms ease; }
"#);
    
    // Hover effects
    css.push_str(r#"
reed[fx*="hover:lift"]:hover { transform: translateY(-2px); }
reed[fx*="hover:scale"]:hover { transform: scale(1.05); }
reed[fx*="hover:brightness"]:hover { filter: brightness(1.1); }
reed[fx*="hover:shadow"]:hover { box-shadow: 0 10px 15px rgba(0,0,0,0.1); }
"#);
    
    // Click effects
    css.push_str(r#"
reed[fx*="click:scale"]:active { transform: scale(0.95); }
reed[fx*="click:brightness"]:active { filter: brightness(0.9); }
"#);
    
    // Animations
    css.push_str(r#"
@keyframes fade-in {
    from { opacity: 0; }
    to { opacity: 1; }
}

@keyframes slide-up {
    from { transform: translateY(20px); opacity: 0; }
    to { transform: translateY(0); opacity: 1; }
}

reed[fx*="animate:fade-in"] { animation: fade-in 300ms ease-out; }
reed[fx*="animate:slide-up"] { animation: slide-up 300ms ease-out; }
"#);
    
    css
}
```

#### Device Namespace (device.rs)
```rust
pub fn generate(config: &Config) -> String {
    let mut css = String::new();
    
    // Cursor
    css.push_str(r#"
reed[device*="cursor:pointer"] { cursor: pointer; }
reed[device*="cursor:default"] { cursor: default; }
reed[device*="cursor:move"] { cursor: move; }
reed[device*="cursor:not-allowed"] { cursor: not-allowed; }
reed[device*="cursor:wait"] { cursor: wait; }
"#);
    
    // User select
    css.push_str(r#"
reed[device*="select:none"] { user-select: none; }
reed[device*="select:text"] { user-select: text; }
reed[device*="select:all"] { user-select: all; }
reed[device*="select:auto"] { user-select: auto; }
"#);
    
    // Pointer events
    css.push_str(r#"
reed[device*="pointer:none"] { pointer-events: none; }
reed[device*="pointer:auto"] { pointer-events: auto; }
"#);
    
    // Touch action
    css.push_str(r#"
reed[device*="touch:none"] { touch-action: none; }
reed[device*="touch:pan-x"] { touch-action: pan-x; }
reed[device*="touch:pan-y"] { touch-action: pan-y; }
reed[device*="touch:manipulation"] { touch-action: manipulation; }
"#);
    
    // Scroll behavior
    css.push_str(r#"
reed[device*="scroll:smooth"] { scroll-behavior: smooth; }
reed[device*="scroll:auto"] { scroll-behavior: auto; }
"#);
    
    css
}
```

### Responsive Generation

Each namespace needs breakpoint variants:

```rust
pub fn generate_responsive(namespace: &str, rules: &str) -> String {
    let mut css = String::new();
    
    // Base rules
    css.push_str(rules);
    
    // Breakpoint variants
    for (name, min_width) in BREAKPOINTS {
        css.push_str(&format!("\n@media (min-width: {}) {{\n", min_width));
        
        // Transform rules for breakpoint
        let responsive_rules = rules.replace(
            &format!("[{}*=", namespace),
            &format!("[{}-{}*=", namespace, name)
        );
        
        css.push_str(&responsive_rules);
        css.push_str("}\n");
    }
    
    css
}
```

## Acceptance Criteria

- [ ] All 6 namespaces generate complete CSS
- [ ] Attribute selectors use `*=` for flexible matching
- [ ] Breakpoint variants generated for all properties
- [ ] Environment variants handled via sublayers
- [ ] Spacing scale from configuration used
- [ ] Colors converted to OKLCH
- [ ] Readable, non-minified output for development
- [ ] Comments for each section

## Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_box_namespace() {
        let config = test_config();
        let css = box::generate(&config);
        
        assert!(css.contains("reed[box*=\"padding:4\"]"));
        assert!(css.contains("reed[box*=\"margin:2\"]"));
        assert!(css.contains("reed[box*=\"width:full\"]"));
    }
    
    #[test]
    fn test_responsive_generation() {
        let base = "reed[box*=\"padding:4\"] { padding: 1rem; }";
        let responsive = generate_responsive("box", base);
        
        assert!(responsive.contains("reed[box-tablet*=\"padding:4\"]"));
        assert!(responsive.contains("@media (min-width: 560px)"));
    }
}
```

## Dependencies

- Ticket #906 (Rust Build System) - Need core structure
- Ticket #904 (OKLCH Color System) - Need color conversion

## Blocks

- Ticket #911 (Component Preset System)
- Ticket #912 (Responsive Breakpoint System)

## Notes

- Start with box and layout namespaces (most used)
- Keep CSS readable in development mode
- Test with real HTML to verify selectors work
- Consider CSS size optimization for production