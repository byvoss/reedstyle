# Configuration Guide

ReedSTYLE uses YAML configuration files for customization. All color formats are automatically converted to OKLCH internally for consistent color manipulation.

## File Structure

```
project-root/
├── reedstyle.config.yaml      # Main configuration
├── reedstyle.colors.yaml      # Color definitions
├── reedstyle.fonts.yaml       # Typography settings
├── reedstyle.components.yaml  # Custom components
└── reedstyle.spacing.yaml     # Optional: spacing overrides
```

## Main Configuration

`reedstyle.config.yaml` - Central configuration file:

```yaml
version: 1.0
build:
  minify: true
  sourcemaps: false
  
config:
  colors: ./reedstyle.colors.yaml
  fonts: ./reedstyle.fonts.yaml
  components: ./reedstyle.components.yaml
  
features:
  autoConvertColors: true  # Convert to OKLCH
  generateVariations: true # Auto-generate color variations
```

## Color Configuration

`reedstyle.colors.yaml` - Define brand and theme colors:

### Input Formats

All formats are automatically converted to OKLCH:

```yaml
colors:
  # Hex format
  brand-a: "#FF6B6B"
  
  # RGB/RGBA format
  brand-b: "rgb(78, 205, 196)"
  brand-c: "rgba(78, 205, 196, 0.9)"
  
  # HSL/HSLA format
  brand-d: "hsl(200, 70%, 50%)"
  brand-e: "hsla(200, 70%, 50%, 0.9)"
  
  # Native OKLCH (no conversion needed)
  brand-f: "oklch(70% 0.15 120)"
```

### Automatic Variation Generation

ReedSTYLE automatically generates 6 variations from each brand color:

```yaml
# Input
brand-a: "#FF6B6B"

# Generates (in OKLCH):
# brand-a-weak    → Very light (backgrounds)
# brand-a-light   → Light (subtle accents)
# brand-a         → Normal (base color)
# brand-a-intense → Intense (more saturated)
# brand-a-bright  → Bright (lighter & saturated)
# brand-a-strong  → Strong (darker & saturated)
```

### Manual Variation Control

Override automatic generation:

```yaml
advanced:
  brand-a:
    base: "oklch(65% 0.25 25)"
    weak: "oklch(85% 0.10 25)"
    light: "oklch(75% 0.18 25)"
    intense: "oklch(55% 0.30 25)"
    bright: "oklch(70% 0.35 25)"
    strong: "oklch(45% 0.28 25)"
```

## Font Configuration

`reedstyle.fonts.yaml` - Typography settings:

### Font Stacks (font-a through font-f)

```yaml
fonts:
  font-a:
    family: "'Inter', -apple-system, sans-serif"
    fallback: sans-serif
    weights:
      thin: 100
      light: 300
      normal: 400
      medium: 500
      semibold: 600
      bold: 700
      
  font-b:
    family: "'Playfair Display', serif"
    fallback: serif
    
  font-c:
    family: "'JetBrains Mono', monospace"
    fallback: monospace
```

### Semantic Assignments

```yaml
semantic:
  system: font-a   # UI elements
  heading: font-b  # Headings
  body: font-a     # Body text
  code: font-c     # Code blocks
```

## Component Configuration

`reedstyle.components.yaml` - Custom reed elements:

### Basic Component

```yaml
components:
  product-card:
    element: article
    box: "[padding:6]"
    face: "[bg:base-0, radius:xl, shadow:lg]"
    layout: "[flex:column, gap:4]"
```

Usage:
```html
<reed as="product-card">
  <!-- Content -->
</reed>
```

### Component with Children

```yaml
components:
  pricing-table:
    element: div
    layout: "[grid:3, gap:6]"
    children:
      item:
        element: div
        box: "[padding:6]"
        face: "[bg:base-0, radius:lg]"
      price:
        element: span
        text: "[size:huge, weight:bold]"
      features:
        element: ul
        text: "[list:none]"
```

### Component Extension

```yaml
components:
  card:
    element: div
    box: "[padding:6]"
    face: "[bg:base-0, radius:lg]"
    
  card-interactive:
    extends: card
    device: "[cursor:pointer]"
    fx: "[hover:lift]"
```

## Color Conversion Examples

### JavaScript API

```javascript
// Automatic conversion happens internally
ReedStyle.setColor('brand-a', '#FF6B6B');
// Internally stored as: oklch(68.5% 0.24 25)

// All variations automatically generated
// brand-a-weak: oklch(88.5% 0.10 25)
// brand-a-light: oklch(78.5% 0.17 25)
// etc.
```

### Rust Implementation

```rust
// src/color_converter.rs
pub fn to_oklch(color: &str) -> String {
    match detect_format(color) {
        ColorFormat::Hex => hex_to_oklch(color),
        ColorFormat::RGB => rgb_to_oklch(color),
        ColorFormat::HSL => hsl_to_oklch(color),
        ColorFormat::OKLCH => color.to_string(),
    }
}
```

## Configuration Loading Order & CSS Layers

### Loading Order
1. **Default Values** - Built-in ReedSTYLE defaults
2. **Config Files** - Your YAML configurations
3. **Environment Variables** - Override specific values
4. **Runtime Updates** - JavaScript API changes

### CSS Layer Assignment
```css
@layer settings {
  /* ReedSTYLE framework defaults */
}

@layer bridge {
  /* Third-party integration & HTML resets */
}

@layer theme {
  /* YOUR CONFIG FILES GO HERE:
   * - reedstyle.colors.yaml → CSS variables
   * - reedstyle.fonts.yaml → Font stacks
   * - reedstyle.components.yaml → Component styles
   * Plus all ReedSTYLE namespace implementations
   */
}

@layer free {
  /* YOUR CUSTOM CSS
   * Anything you write directly
   * Always wins over theme layer
   */
}
```

## Best Practices

### Color Definition

```yaml
# Good: Use any format you prefer
colors:
  brand-a: "#FF6B6B"        # Designer-friendly hex
  brand-b: "rgb(78, 205, 196)" # From design tools
  
# Better: Use OKLCH for precise control
colors:
  brand-a: "oklch(68.5% 0.24 25)"
  brand-b: "oklch(82% 0.15 175)"
```

### Font Organization

```yaml
# Use semantic pattern matching brand colors
fonts:
  font-a: # Primary font (matches brand-a)
  font-b: # Secondary font (matches brand-b)
  font-c: # Tertiary font (matches brand-c)
```

### Component Naming

```yaml
# Good: Descriptive, hierarchical
components:
  card: {}
  card-header: {}
  card-body: {}
  
# Avoid: Generic names
components:
  box: {}     # Too generic
  header: {}  # Conflicts with HTML
```

## Environment-Specific Configuration

### Development

```yaml
# reedstyle.config.dev.yaml
build:
  minify: false
  sourcemaps: true
  watch: true
```

### Production

```yaml
# reedstyle.config.prod.yaml
build:
  minify: true
  sourcemaps: false
  optimize: maximum
```

### Usage

```bash
# Development
cargo run --config reedstyle.config.dev.yaml

# Production
cargo run --config reedstyle.config.prod.yaml
```

## Validation

ReedSTYLE validates configuration on build:

```bash
cargo run --validate

# Output:
✓ Colors: 6 brand colors defined
✓ Fonts: 3 font stacks configured
✓ Components: 24 components registered
✓ OKLCH conversion: All colors valid
```

## Migration from Other Systems

### From Tailwind

```yaml
# Map Tailwind colors to ReedSTYLE
colors:
  brand-a: "#3B82F6"  # blue-500
  brand-b: "#10B981"  # green-500
  state-error: "#EF4444"  # red-500
```

### From Bootstrap

```yaml
# Map Bootstrap variables
colors:
  brand-a: "#0D6EFD"  # $primary
  brand-b: "#6C757D"  # $secondary
  state-success: "#198754"  # $success
```

## Next: [Examples](https://reedstyle.dev/examples)