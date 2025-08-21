# Ticket #918: Reed Element Default System

## Status: ✅ Complete

## Decision Log References
- DEC014 - Reed elements default to div behavior
- DEC015 - Components without element field default to div
- DEC016 - HTML element defaults are hard-coded in Rust

## Description
Implement proper default behavior for reed elements based on their `as` attribute, with automatic inheritance from HTML elements and support for custom components from YAML configuration.

## Requirements

### Core Behavior
1. **Default Behavior**
   - `<reed>` without `as` attribute → behaves as `<div>`
   - Custom components without `element` field → behave as `<div>`
   - Standard HTML elements → inherit their native defaults

### Implementation Tasks

1. **Remove Hard-coded Defaults**
   - Remove manually added `reed[as="h1"]` etc. styles from `/src/css/mod.rs`
   - Move to systematic generation

2. **Create Default Styles Module**
   ```rust
   // src/css/defaults.rs
   pub fn generate_element_defaults() -> String {
       let mut css = String::new();
       
       // Hard-coded HTML element defaults in Rust
       // These generate CSS but are NOT dynamically created
       css.push_str("reed[as=\"h1\"] { font-size: 2em; font-weight: bold; margin: 0.67em 0; }\n");
       css.push_str("reed[as=\"h2\"] { font-size: 1.5em; font-weight: bold; margin: 0.83em 0; }\n");
       css.push_str("reed[as=\"p\"] { display: block; margin: 1em 0; }\n");
       // ... all HTML elements hard-coded here
       
       css
   }
   ```

3. **Process Components from YAML**
   ```rust
   // For each component in components.yaml:
   // 1. Read element field (default: div)
   // 2. Apply base element styles
   // 3. Apply namespace attributes
   // 4. Generate CSS rule for reed[as="component-name"]
   ```

4. **CSS Generation Flow**
   ```
   components.yaml → Rust processor → CSS rules
   
   Example:
   card:
     element: article  # optional, defaults to div
     box: "[padding:6]"
     face: "[bg:base-0]"
   
   Generates:
   reed[as="card"] {
     /* article defaults */
     display: block;
     /* namespace styles */
     padding: 1.5rem;
     background: var(--rs-base-0);
   }
   ```

### Standard HTML Element Defaults

```css
/* Block elements */
reed[as="div"], reed[as="section"], reed[as="article"], 
reed[as="header"], reed[as="footer"], reed[as="main"], 
reed[as="nav"], reed[as="aside"] {
  display: block;
}

/* Inline elements */
reed[as="span"], reed[as="a"], reed[as="strong"], 
reed[as="em"], reed[as="code"], reed[as="small"] {
  display: inline;
}

/* Headings */
reed[as="h1"] { display: block; font-size: 2em; font-weight: bold; margin: 0.67em 0; }
reed[as="h2"] { display: block; font-size: 1.5em; font-weight: bold; margin: 0.83em 0; }
reed[as="h3"] { display: block; font-size: 1.17em; font-weight: bold; margin: 1em 0; }
reed[as="h4"] { display: block; font-size: 1em; font-weight: bold; margin: 1.33em 0; }
reed[as="h5"] { display: block; font-size: 0.83em; font-weight: bold; margin: 1.67em 0; }
reed[as="h6"] { display: block; font-size: 0.67em; font-weight: bold; margin: 2.33em 0; }

/* Text elements */
reed[as="p"] { display: block; margin: 1em 0; }
reed[as="blockquote"] { display: block; margin: 1em 40px; }
reed[as="pre"] { display: block; font-family: monospace; white-space: pre; margin: 1em 0; }

/* Lists */
reed[as="ul"] { display: block; list-style-type: disc; margin: 1em 0; padding-left: 40px; }
reed[as="ol"] { display: block; list-style-type: decimal; margin: 1em 0; padding-left: 40px; }
reed[as="li"] { display: list-item; }

/* Tables */
reed[as="table"] { display: table; border-collapse: separate; border-spacing: 2px; }
reed[as="thead"] { display: table-header-group; }
reed[as="tbody"] { display: table-row-group; }
reed[as="tfoot"] { display: table-footer-group; }
reed[as="tr"] { display: table-row; }
reed[as="td"] { display: table-cell; padding: 1px; }
reed[as="th"] { display: table-cell; font-weight: bold; text-align: center; padding: 1px; }

/* Form elements */
reed[as="button"] { display: inline-block; }
reed[as="input"] { display: inline-block; }
reed[as="textarea"] { display: inline-block; }
reed[as="select"] { display: inline-block; }
reed[as="label"] { display: inline; }

/* Media */
reed[as="img"] { display: inline-block; }
reed[as="video"] { display: inline-block; }
reed[as="audio"] { display: inline-block; }

/* Other */
reed[as="hr"] { display: block; margin: 0.5em auto; border-style: inset; border-width: 1px; }
reed[as="br"] { display: inline; }
```

## Acceptance Criteria

- [x] Reed elements without `as` attribute display as block (div behavior)
- [x] Standard HTML elements get correct default styles
- [x] Custom components from YAML work correctly
- [x] Components without `element` field default to div
- [x] Components with `element` field inherit from that element
- [x] All namespace attributes are applied on top of defaults
- [x] CSS is generated systematically, not hard-coded
- [x] Test page shows rendering for reed elements

## Testing

Create test page comparing:
1. Standard HTML elements
2. Reed elements with `as` attribute
3. Custom components from YAML

All three should render identically when given the same content.

```html
<!-- Should all look the same -->
<h1>Standard H1</h1>
<reed as="h1">Reed H1</reed>

<!-- Custom component -->
<reed as="card">
  <reed as="card-header">Header</reed>
  <reed as="card-body">Content</reed>
</reed>
```

## Dependencies

- Ticket #907 (Namespace CSS Generation) - Complete ✓
- Ticket #916 (Typography Engine) - Complete ✓

## Blocks

None

## Notes

- This replaces the temporary hard-coded solution from RS916
- Makes the system truly extensible via YAML configuration
- Ensures consistent behavior across all reed elements
- References DEC014 and DEC015 for design decisions