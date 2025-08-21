# Architecture

## Technical Overview

ReedSTYLE is built with a modern, performant architecture that prioritizes developer experience and runtime efficiency.

## CSS Layer System

ReedSTYLE uses CSS Layers to create a predictable cascade without `!important`:

```css
@layer settings, bridge, theme, free;
```

### Layer 1: Settings
Base framework defaults and variables.

```css
@layer settings {
  :root {
    /* ReedSTYLE defaults */
    --reedstyle-size-normal: 1rem;
    --reedstyle-space-4: 1rem;
    --reedstyle-radius-md: 0.375rem;
    /* Framework-provided defaults */
  }
}
```

### Layer 2: Bridge
Third-party integrations and migrations (configured in `reedstyle.bridge.yaml`).

```css
@layer bridge {
  /* Reset styles */
  *, *::before, *::after {
    box-sizing: border-box;
  }
  
  /* Semantic HTML defaults */
  button {
    cursor: pointer;
    font: inherit;
  }
  
  /* Sublayers for each integration */
  @layer bootstrap {
    /* Bootstrap overrides from bridge.yaml */
    .btn {
      all: revert;
      padding: var(--reedstyle-space-3);
    }
  }
  
  @layer tailwind {
    /* Tailwind migration mappings */
    .tw-reset {
      all: revert;
    }
  }
  
  @layer material {
    /* Material UI adjustments */
    .MuiButton-root {
      text-transform: none;
    }
  }
}
```

Each integration creates its own sublayer:
- `@layer bridge.bootstrap`
- `@layer bridge.tailwind`
- `@layer bridge.material`
- `@layer bridge.custom`

### Layer 3: Theme
User configuration from YAML files + ReedSTYLE core with environment sublayers.

```css
@layer theme {
  /* Base theme layer - always active */
  /* From reedstyle.colors.yaml */
  :root {
    --reedstyle-color-brand-a: oklch(68.5% 0.24 25);
    --reedstyle-color-brand-b: oklch(82% 0.15 175);
    /* All user-configured colors */
  }
  
  /* From reedstyle.fonts.yaml */
  :root {
    --reedstyle-font-a: 'Inter', sans-serif;
    --reedstyle-font-b: 'Playfair Display', serif;
    /* All user-configured fonts */
  }
  
  /* From reedstyle.components.yaml */
  reed[as="card"] {
    /* User-defined component styles */
  }
  
  /* ReedSTYLE namespace implementations */
  reed[box*="padding:4"] {
    padding: var(--reedstyle-space-4);
  }
  
  reed[face*="bg:brand-a"] {
    background: var(--reedstyle-color-brand-a);
  }
  
  /* Environment sublayers - can be toggled globally */
  @layer dev {
    /* Development environment overrides */
    reed[box-dev*="padding:8"] {
      padding: var(--reedstyle-space-8);
    }
  }
  
  @layer staging {
    /* Staging environment overrides */
    reed[box-staging*="padding:6"] {
      padding: var(--reedstyle-space-6);
    }
  }
  
  @layer prod {
    /* Production environment overrides */
    reed[box-prod*="padding:2"] {
      padding: var(--reedstyle-space-2);
    }
  }
}
```

### Layer 4: Free
User's custom CSS - highest priority.

```css
@layer free {
  /* User's hand-written CSS */
  .my-special-override {
    /* Always wins over all other layers */
  }
  
  /* Custom modifications */
  reed[as="my-component"] {
    /* User's direct customizations */
  }
}
```

### Cascade Benefits

1. **No !important needed** - Layer order determines specificity
2. **Predictable overrides** - User styles always win
3. **Framework friendly** - Bridge layer handles conflicts
4. **Clean separation** - Each layer has clear purpose

## Build System

### Rust Core

The build system is written in Rust for maximum performance:

```rust
// src/main.rs
fn main() {
    // 1. Load configuration
    let config = load_config();
    
    // 2. Generate CSS layers
    generate_settings_layer(&config);
    generate_bridge_layer();
    generate_theme_layer(&config);
    
    // 3. Optimize with Lightning CSS
    optimize_css();
    
    // 4. Compile TypeScript
    compile_typescript();
}
```

### CSS Generation

Each namespace has its own Rust module:

```rust
// src/namespaces/box.rs
pub fn generate() -> Result<String> {
    let mut css = String::new();
    
    // Generate padding rules
    for (key, value) in SPACING_SCALE {
        css.push_str(&format!(
            "reed[box*=\"padding:{}\"] {{ padding: {}; }}\n",
            key, value
        ));
    }
    
    Ok(css)
}
```

### Lightning CSS

Used for optimization and browser compatibility:

- Autoprefixing
- Minification
- Color conversion (hex/rgb → OKLCH)
- Custom property optimization
- Dead code elimination

### TypeScript/SWC/TSC Pipeline

JavaScript compilation uses our optimized toolchain:

1. **TSC (TypeScript Compiler)** → Type checking & .d.ts generation
2. **SWC** → Lightning-fast transpilation (100x faster than Babel)
3. **Lightning CSS** → Also handles CSS imports in JS
4. **Tree shaking** → Remove unused code

```bash
# Our exact pipeline:
tsc --noEmit        # Type check only
swc src/typescript  # Fast transpilation  
# No Webpack, no Parcel, no Vite needed!
```

## Configuration System

### Configuration Files

ReedSTYLE uses YAML configuration files in the project root:

```
project-root/
├── reedstyle.config.yaml      # Main configuration
├── reedstyle.colors.yaml      # Color definitions (brand-a to brand-f)
├── reedstyle.fonts.yaml       # Typography (font-a to font-f)
├── reedstyle.components.yaml  # Custom reed elements
├── reedstyle.bridge.yaml      # Third-party integrations
├── reedstyle.env.yaml         # Environment configurations (use sparingly!)
└── reedstyle.spacing.yaml     # Optional spacing scale
```

### Color System (OKLCH)

All colors are internally converted to OKLCH for consistent manipulation:

```rust
// src/color_system.rs
pub fn process_color(input: &str) -> OklchColor {
    match detect_format(input) {
        Format::Hex => hex_to_oklch(input),      // #FF6B6B
        Format::RGB => rgb_to_oklch(input),      // rgb(255, 107, 107)
        Format::HSL => hsl_to_oklch(input),      // hsl(0, 100%, 71%)
        Format::OKLCH => parse_oklch(input),     // oklch(68.5% 0.24 25)
    }
}

// Generate variations automatically
pub fn generate_variations(base: OklchColor) -> ColorVariations {
    ColorVariations {
        weak: adjust_oklch(base, l: +20, c: -60%),    // Very light
        light: adjust_oklch(base, l: +10, c: -30%),   // Light
        normal: base,                                  // Original
        intense: adjust_oklch(base, l: -10, c: +20%), // Intense
        bright: adjust_oklch(base, l: +5, c: +40%),   // Bright
        strong: adjust_oklch(base, l: -20, c: +10%),  // Strong
    }
}
```

### Configuration Loading

```rust
// src/config.rs
pub fn load_config() -> Config {
    // 1. Load main config
    let main = load_yaml("reedstyle.config.yaml");
    
    // 2. Load referenced configs
    let colors = load_yaml(&main.config.colors);
    let fonts = load_yaml(&main.config.fonts);
    let components = load_yaml(&main.config.components);
    let bridge = load_yaml(&main.config.bridge);
    let env = load_yaml(&main.config.env);  // Environment configs
    
    // 3. Process colors to OKLCH
    let processed_colors = colors.iter()
        .map(|(name, value)| (name, process_color(value)))
        .collect();
    
    // 4. Generate CSS variables
    generate_css_variables(processed_colors);
    
    // 5. Process bridge integrations
    generate_bridge_layers(bridge);
    
    // 6. Process environment configurations (if needed)
    if env.is_enabled() {
        generate_environment_styles(env);
    }
}

// Generate bridge sublayers with imported CSS
pub fn generate_bridge_layers(bridge: BridgeConfig) -> String {
    let mut css = String::new();
    
    css.push_str("@layer bridge {\n");
    
    for (name, config) in bridge.integrations {
        // Only process if enabled (like a switch)
        if config.enabled {
            println!("✓ Bridge layer '{}' enabled", name);
            css.push_str(&format!("  @layer {} {{\n", name));
            
            // Import the framework's CSS file
            if let Some(path) = config.path {
                let framework_css = read_file(&path);
                css.push_str(&format!("    /* {} CSS imported from {} */\n", name, path));
                css.push_str(&framework_css);
            }
            
            // Add overrides
            css.push_str("\n    /* ReedSTYLE Overrides */\n");
            css.push_str(&config.overrides);
            
            css.push_str("  }\n");
        } else {
            println!("✗ Bridge layer '{}' disabled", name);
            // Layer not created, CSS not imported, zero overhead
        }
    }
    
    css.push_str("}\n");
    css
}
```

### Environment System (Via Theme Sublayers)

Environments are implemented as sublayers within the theme layer for easy global control:

```rust
// src/environments.rs
pub fn generate_environment_styles(env: EnvConfig) -> String {
    let mut css = String::new();
    
    // Always generate in theme layer
    css.push_str("@layer theme {\n");
    
    for (name, config) in env.environments {
        if config.enabled {
            // Create sublayer for each environment
            css.push_str(&format!("  @layer {} {{\n", name));
            
            // Generate environment-specific selectors
            css.push_str(&generate_env_selectors(name, config));
            
            // Generate environment + breakpoint combinations
            for breakpoint in BREAKPOINTS {
                css.push_str(&generate_env_breakpoint_selectors(name, breakpoint));
            }
            
            css.push_str("  }\n");
        }
    }
    
    css.push_str("}\n");
    css
}

// Dynamic environment control
pub fn toggle_environment_layer(env_name: &str, enabled: bool) {
    // Can be controlled via:
    // 1. CMS settings
    // 2. React props
    // 3. Build flags
    // 4. Runtime JavaScript
    
    if enabled {
        // Include @layer theme.{env_name} in output
        println!("✓ Environment '{}' layer enabled", env_name);
    } else {
        // Exclude @layer theme.{env_name} from output
        println!("✗ Environment '{}' layer disabled", env_name);
    }
}

// JavaScript runtime control
pub fn generate_env_toggle_js() -> &'static str {
    r#"
    // Toggle environment sublayers dynamically
    ReedStyle.environment = {
        toggle(envName, enabled) {
            const sheet = document.styleSheets[0];
            const layerName = `theme.${envName}`;
            
            if (enabled) {
                // Enable the sublayer
                sheet.insertRule(`@layer ${layerName} {}`, 0);
            } else {
                // Disable by removing rules
                // Or use CSS.supports API for layer control
            }
        },
        
        // Set from external system (CMS, React, etc)
        setFromConfig(config) {
            Object.entries(config.environments).forEach(([name, enabled]) => {
                this.toggle(name, enabled);
            });
        }
    };
    "#
}
```

## Component System

### Built-in Presets

Defined in Rust, compiled to CSS:

```rust
// src/presets/card.rs
pub fn card_preset() -> String {
    r#"
    reed[as="card"] {
        display: block;
        padding: var(--reedstyle-space-6);
        background: var(--reedstyle-color-base-0);
        border-radius: var(--reedstyle-radius-lg);
        box-shadow: var(--reedstyle-shadow-md);
    }
    "#
}
```

### User Components (YAML)

Loaded and processed at build time:

```yaml
# reedstyle.components.yaml
components:
  product-card:
    element: article
    layout: "[flex:column]"
    box: "[padding:6]"
    face: "[bg:base-0, radius:xl, shadow:lg]"
```

Generates:

```css
reed[as="product-card"] {
    display: flex;
    flex-direction: column;
    padding: var(--reedstyle-space-6);
    /* ... */
}
```

## Namespace Implementation

### Direct Attribute Selection

No JavaScript conversion needed:

```css
/* Direct CSS attribute selectors */
reed[layout*="grid:3"] {
    display: grid;
    grid-template-columns: repeat(3, 1fr);
}

reed[box*="padding:4"] {
    padding: var(--reedstyle-space-4);
}
```

### Array Syntax Parsing

CSS handles array syntax natively:

```css
/* Matches: box="[padding:4, margin:2]" */
reed[box*="padding:4"] { padding: 1rem; }
reed[box*="margin:2"] { margin: 0.5rem; }
```

### Responsive System

Breakpoint-specific attributes:

```css
/* Mobile first */
reed[layout*="flex:column"] { 
    flex-direction: column; 
}

/* Tablet */
@media (min-width: 560px) {
    reed[layout-tablet*="flex:row"] { 
        flex-direction: row; 
    }
}

/* Desktop */
@media (min-width: 960px) {
    reed[layout-screen*="grid:3"] { 
        display: grid;
        grid-template-columns: repeat(3, 1fr);
    }
}
```

## Performance Optimizations

### CSS Size Reduction

- **Direct selectors**: 35% smaller than class-based
- **CSS Variables**: Dynamic theming without duplication
- **Layer organization**: Efficient rule matching
- **Partial matching**: `*=` for flexible arrays

### Runtime Performance

- **GPU acceleration**: Transform/opacity for animations
- **CSS containment**: Layout isolation
- **Will-change hints**: Prepare for animations
- **Intersection Observer**: Lazy loading effects

### Build Performance

Our optimized toolchain delivers exceptional speed:

- **Rust**: ~200ms for full CSS generation
- **Lightning CSS**: Native speed optimization (written in Rust)
- **SWC**: ~20ms for TypeScript transpilation (Rust-based)
- **TSC**: Type checking only (no emit)
- **Total build**: <500ms for complete rebuild
- **No PostCSS**: Direct CSS generation
- **No Babel**: SWC is 100x faster
- **No bundler overhead**: Direct output

## Distribution Strategy

### Pre-Built Files Only

ReedSTYLE ships as pre-built files - users never need build tools:

```
dist/
├── reedstyle.css         # Development CSS with comments (~350KB)
├── reedstyle.min.css     # Production minified CSS (~346KB)
├── reedstyle.js          # Optional JavaScript enhancement (~100KB)
├── reedstyle.min.js      # Production minified JS (~40KB)
└── reedstyle.d.ts        # TypeScript definitions (~20KB)
```

### Usage Without Build Process

```html
<!DOCTYPE html>
<html>
<head>
  <!-- Just include the CSS -->
  <link rel="stylesheet" href="reedstyle.min.css">
  
  <!-- JavaScript is optional! -->
  <script src="reedstyle.min.js" defer></script>
</head>
<body>
  <!-- Works immediately -->
  <r-s as="hero">
    <h1>No build required!</h1>
  </r-s>
</body>
</html>
```

### File Structure

src/
├── namespaces/           # Rust namespace modules
│   ├── box.rs
│   ├── device.rs
│   ├── face.rs
│   ├── fx.rs
│   ├── layout.rs
│   └── text.rs
├── typescript/           # TypeScript source
│   ├── core.ts
│   ├── effects.ts
│   └── components.ts
├── main.rs              # Rust entry point
└── settings.rs          # Configuration
```

## CI/CD Pipeline

### Build Process

ReedSTYLE uses a Rust-powered build system with minimal Node.js dependencies:

```bash
# Internal build process (not for users)
cargo run --release

# This executes:
# 1. CSS generation from namespaces
# 2. Lightning CSS optimization
# 3. TypeScript compilation with SWC
# 4. Minification and bundling
```

### GitHub Actions Workflow

```yaml
# .github/workflows/build.yml
name: Build and Test

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Build ReedSTYLE
        run: cargo run --release
      
      - name: Test Output
        run: |
          test -f dist/reedstyle.css
          test -f dist/reedstyle.min.css
          test -f dist/reedstyle.js
          test -f dist/reedstyle.min.js
      
      - name: Check Sizes
        run: |
          test $(wc -c < dist/reedstyle.min.css) -lt 204800  # <200KB
          test $(wc -c < dist/reedstyle.min.js) -lt 51200    # <50KB
```

### Release Process

```yaml
# .github/workflows/release.yml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - name: Build Release
        run: cargo run --release
      
      - name: Publish to NPM
        run: npm publish
        
      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: dist/*
```

### Distribution Strategy

1. **Pre-built Files Only**
   - Users receive ready-to-use files
   - No build process required
   - Just include CSS/JS files

2. **CDN Distribution**
   ```html
   <!-- jsDelivr (auto-updates from NPM) -->
   <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/reedstyle/dist/reedstyle.min.css">
   
   <!-- unpkg (also auto-updates) -->
   <link rel="stylesheet" href="https://unpkg.com/reedstyle/dist/reedstyle.min.css">
   ```

3. **Version Management**
   - Semantic versioning (major.minor.patch)
   - Synchronized between Cargo.toml and package.json
   - Tagged releases trigger automatic deployment

### Testing Strategy

```rust
// tests/css_generation.rs
#[test]
fn test_reed_selectors() {
    let css = generate_box_namespace();
    assert!(css.contains("reed[box*=\"padding:4\"]"));
}

#[test]
fn test_layer_structure() {
    let css = generate_complete_css();
    assert!(css.contains("@layer settings, bridge, theme, free;"));
}
```

### Performance Monitoring

- **Bundle size limits**: CSS <200KB, JS <50KB
- **Build time target**: <500ms for complete rebuild
- **Lighthouse CI**: Performance testing on every PR

## Security Considerations

### Content Security Policy

ReedSTYLE works with strict CSP:

```html
<meta http-equiv="Content-Security-Policy" 
      content="style-src 'self'; script-src 'self';">
```

### No Inline Styles

All styles via attributes, no `style=""` needed.

### XSS Protection

- No `eval()` or `innerHTML`
- Sanitized attribute values
- Trusted types compatible

## Browser Support

### CSS Features Used

- CSS Layers (2022+)
- CSS Custom Properties
- OKLCH colors (with fallbacks)
- Grid & Flexbox
- CSS Containment
- Cascade Layers

### Minimum Versions

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- All mobile browsers

## Next: [Implementation Guide](211-implementation.md)