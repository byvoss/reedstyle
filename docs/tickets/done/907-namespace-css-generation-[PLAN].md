# RS907 Implementation Plan

## Ticket: Namespace CSS Generation

### Critical Files to Read (IN THIS ORDER!)

#### 1. Requirements
- `/docs/tickets/907-namespace-css-generation.md` - Ticket specification

#### 2. Namespace Documentation (MUST FOLLOW EXACTLY)
- `/docs/develop/102-box.md` - Box namespace (spacing, sizing)
- `/docs/develop/103-device.md` - Device namespace (user interaction)
- `/docs/develop/104-face.md` - Face namespace (visual appearance)
- `/docs/develop/105-fx.md` - FX namespace (effects, animations)
- `/docs/develop/106-layout.md` - Layout namespace (flex, grid)
- `/docs/develop/107-text.md` - Text namespace (typography)

#### 3. Architecture & Patterns
- `/docs/develop/201-architecture.md` - CSS layers, selector patterns
- `/docs/develop/101-namespaces-overview.md` - Core principles

#### 4. Existing Code Base
- `/src/css/mod.rs` - Current CSS generation logic
- `/src/config/mod.rs` - Configuration structures
- `/src/color/mod.rs` - OKLCH conversion

### Critical Implementation Rules

#### Selector Pattern
```css
/* Base selector */
reed[namespace*="property:value"] { }

/* Responsive variants */
reed[namespace-tablet*="property:value"] { }
reed[namespace-screen*="property:value"] { }

/* Multiple properties */
reed[namespace*="property1:value1, property2:value2"] { }
```

#### Scales (MUST USE THESE EXACT VALUES)

**Dimension Scale:**
- `tiny` → `small` → `normal` → `large` → `huge` → `mega` → `ultra`

**Visual Scope:**
- `weak` → `light` → `normal` → `intense` → `bright` → `strong`

**Typography Scale:**
- `thin` → `light` → `regular` → `medium` → `bold` → `black`

#### Breakpoints
- `phone`: 0px (default, no media query)
- `tablet`: min-width: 560px
- `screen`: min-width: 960px

#### CSS Layer Structure
```css
@layer theme {
  /* ALL namespace CSS goes here */
}
```

### Implementation Steps

1. **Create namespace modules**
   - `src/css/namespaces/box.rs`
   - `src/css/namespaces/device.rs`
   - `src/css/namespaces/face.rs`
   - `src/css/namespaces/fx.rs`
   - `src/css/namespaces/layout.rs`
   - `src/css/namespaces/text.rs`

2. **Each namespace must:**
   - Generate base properties
   - Generate responsive variants (tablet, screen)
   - Use OKLCH for ALL colors
   - Follow exact selector pattern
   - Use correct scale values

3. **Box Namespace Properties:**
   - padding, margin (all directions)
   - width, height, min/max variants
   - overflow, display

4. **Device Namespace Properties:**
   - cursor, pointer-events
   - user-select, touch-action
   - scroll behavior

5. **Face Namespace Properties:**
   - background (color, gradient, image)
   - border (width, style, color, radius)
   - shadow, opacity, filter

6. **FX Namespace Properties:**
   - transition, animation
   - transform (scale, rotate, translate)
   - hover/active/focus states

7. **Layout Namespace Properties:**
   - flex (direction, wrap, justify, align)
   - grid (template, gap, areas)
   - position, z-index

8. **Text Namespace Properties:**
   - font (family, size, weight, style)
   - color, align, decoration
   - line-height, letter-spacing

### Testing Checklist

- [ ] Generated CSS contains `reed[` selectors
- [ ] All colors are in OKLCH format
- [ ] Responsive variants work (tablet, screen)
- [ ] CSS is within `@layer theme`
- [ ] File size < 350KB unminified
- [ ] All scale values match documentation
- [ ] No `data-r-` attributes (old system)

### Decision References
- DEC001: Pre-built files only
- DEC002: OKLCH at build time
- DEC007: ALL colors as OKLCH internally

### Session Recovery Note
If session is lost, this file contains ALL context needed to continue RS907.
Start by reading this file, then the referenced documentation in order.