# RS918 Reed Element Defaults - Implementation Plan

## Session Recovery
If session is lost, start here:
1. Check branch: `git branch` (should be feature/RS918-reed-element-defaults)
2. Read this plan file first
3. Check `/src/css/mod.rs` for temporary hard-coded styles to remove
4. Continue from last completed step

## Files to Read (in order)
1. `/docs/tickets/918-reed-element-defaults.md` - Main ticket specification
2. `/reedstyle.components.yaml` - Component definitions with element field
3. `/src/css/mod.rs` - Current hard-coded defaults (lines 54-88)
4. `/src/config/mod.rs` - Component structure with element field
5. `/decisions.csv` - DEC014 and DEC015 for design decisions

## Critical Design Decisions

### DEC014: Reed Element Defaults
- Reed elements without `as` attribute default to div behavior
- Everything is a block element unless specified otherwise

### DEC015: Component Element Inheritance  
- Components without `element` field in YAML default to div
- Consistent with reed default behavior

## IMPORTANT CLARIFICATION

### What is Hard-coded (in Rust):
- ALL standard HTML element defaults (h1, p, div, span, etc.)
- These are universal and never change
- Defined directly in Rust code that generates CSS

### What is Dynamic (from YAML):
- Custom components (card, button-primary, etc.)
- These read from components.yaml
- They inherit from HTML elements + add their own styles

## Implementation Steps

### Phase 1: Create Defaults Module
1. Create `/src/css/defaults.rs`
2. Hard-code HTML element defaults in Rust code
3. These Rust functions generate CSS strings
4. NOT reading from any config - these are universal standards
5. Export generation function

### Phase 2: Process Components
1. Read components from YAML
2. For each component:
   - Get element field (default: div)
   - Look up element defaults
   - Merge with namespace attributes
   - Generate `reed[as="component"]` CSS

### Phase 3: Integration
1. Remove hard-coded styles from `/src/css/mod.rs` (lines 54-88)
2. Import and call defaults module
3. Ensure proper CSS layer ordering
4. Test build process

### Phase 4: Testing
1. Create `/test/test-element-defaults.html`
2. Compare standard HTML vs reed elements
3. Test custom components
4. Verify inheritance works

## HTML Element Defaults Reference

```rust
// Standard defaults to implement
const ELEMENT_DEFAULTS: &[(&str, &str)] = &[
    // Block elements
    ("div", "display: block;"),
    ("section", "display: block;"),
    ("article", "display: block;"),
    ("header", "display: block;"),
    ("footer", "display: block;"),
    ("main", "display: block;"),
    ("nav", "display: block;"),
    ("aside", "display: block;"),
    
    // Headings
    ("h1", "display: block; font-size: 2em; font-weight: bold; margin: 0.67em 0;"),
    ("h2", "display: block; font-size: 1.5em; font-weight: bold; margin: 0.83em 0;"),
    ("h3", "display: block; font-size: 1.17em; font-weight: bold; margin: 1em 0;"),
    ("h4", "display: block; font-size: 1em; font-weight: bold; margin: 1.33em 0;"),
    ("h5", "display: block; font-size: 0.83em; font-weight: bold; margin: 1.67em 0;"),
    ("h6", "display: block; font-size: 0.67em; font-weight: bold; margin: 2.33em 0;"),
    
    // Text
    ("p", "display: block; margin: 1em 0;"),
    ("span", "display: inline;"),
    ("strong", "display: inline; font-weight: bold;"),
    ("b", "display: inline; font-weight: bold;"),
    ("em", "display: inline; font-style: italic;"),
    ("i", "display: inline; font-style: italic;"),
    ("small", "display: inline; font-size: smaller;"),
    
    // Lists
    ("ul", "display: block; list-style-type: disc; margin: 1em 0; padding-left: 40px;"),
    ("ol", "display: block; list-style-type: decimal; margin: 1em 0; padding-left: 40px;"),
    ("li", "display: list-item;"),
    
    // Tables
    ("table", "display: table; border-collapse: separate; border-spacing: 2px;"),
    ("thead", "display: table-header-group;"),
    ("tbody", "display: table-row-group;"),
    ("tr", "display: table-row;"),
    ("td", "display: table-cell; padding: 1px;"),
    ("th", "display: table-cell; font-weight: bold; text-align: center; padding: 1px;"),
];
```

## Component Processing Logic

```rust
// Pseudo-code for component processing
for (name, component) in components {
    let base_element = component.element.unwrap_or("div");
    let base_styles = get_element_defaults(base_element);
    
    let mut css = format!("reed[as=\"{}\"] {{\n", name);
    css.push_str(&base_styles);
    
    if let Some(box_attr) = component.box_ {
        css.push_str(&parse_box_namespace(box_attr));
    }
    
    if let Some(face_attr) = component.face {
        css.push_str(&parse_face_namespace(face_attr));
    }
    
    // ... other namespaces
    
    css.push_str("}\n");
}
```

## Testing Checklist
- [ ] Reed without `as` displays as block
- [ ] Reed with `as="h1"` looks like real h1
- [ ] Custom component `card` works
- [ ] Component with `element: button` inherits button styles
- [ ] Namespace attributes override defaults correctly
- [ ] No regression in existing functionality

## Notes
- Keep defaults minimal and standard
- Don't include browser-specific resets
- Focus on display type and basic typography
- Let namespaces handle advanced styling