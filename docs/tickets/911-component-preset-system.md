# Ticket #911: Component Preset System

## Status: ✅ Done

## Overview
Implement the component preset SYSTEM that allows users to define their own components via YAML configuration. This system enables users to create reusable components with predefined properties, so they only need to use `<r-s as="component-name">` without any additional attributes.

**WICHTIG**: Das System funktioniert zur LAUFZEIT via JavaScript, nicht zur Build-Zeit! Die Komponenten werden NICHT in CSS kompiliert, sondern die JS liest die Definitionen und setzt die entsprechenden Namespace-Attribute dynamisch.

## Files to Read First (Understanding Current System)
Before implementing, read these files to understand the existing component system:

1. **Current Component System**:
   - `/src/config/mod.rs` - Component configuration structures
   - `/src/css/mod.rs` - How components are currently processed
   - `/src/css/defaults.rs` - HTML element defaults system
   - `/reedstyle.components.yaml` - Current component definitions

2. **Namespace System** (presets use these):
   - `/src/css/namespaces/box.rs` - Spacing and sizing
   - `/src/css/namespaces/face.rs` - Visual appearance
   - `/src/css/namespaces/layout.rs` - Layout properties
   - `/src/css/namespaces/text.rs` - Typography

3. **Documentation for Reference**:
   - `/docs/develop/031-presets.md` - Complete preset specification
   - `/docs/develop/021-reed-element.md` - How presets work with reed/r-s

## Objectives
- Create system for user-defined components via YAML
- Parse component definitions from `reedstyle.components.yaml`
- Generate CSS for each component automatically
- Support component inheritance via `extends` field
- Provide 3 example components in YAML for testing

## Technical Requirements

### 1. YAML Component Structure
Components are defined in `reedstyle.components.yaml`:
```yaml
components:
  component-name:
    element: div           # HTML element (optional, defaults to div)
    box: "[padding:4]"     # Box namespace properties
    face: "[bg:base-0]"    # Face namespace properties
    text: "[color:base-900]" # Text namespace properties
    layout: "[flex:row]"   # Layout namespace properties
    device: "[cursor:pointer]" # Device namespace properties
    fx: "[hover:lift]"     # Effects namespace properties
    extends: other-component # Inherit from another component (optional)
```

### 2. CSS Generation
For each component in YAML, generate CSS:
```css
/* Component: card */
r-s[as="card"] {
  /* Properties from box namespace */
  padding: 1.5rem;
  margin: 0.5rem;
  
  /* Properties from face namespace */
  background: var(--reedstyle-color-base-0);
  border-radius: 0.5rem;
  box-shadow: 0 4px 6px rgba(0,0,0,0.1);
}
```

### 3. Component Extension System
Support extending components:
```yaml
components:
  card:
    box: "[padding:6]"
    face: "[bg:base-0, radius:lg]"
    
  card-interactive:
    extends: card  # Inherits all card properties
    device: "[cursor:pointer]"  # Adds pointer cursor
    fx: "[hover:lift]"  # Adds hover effect
```

### 4. Implementation Approach (ACTUAL)
**WICHTIG**: Components werden NICHT zur Build-Zeit in CSS kompiliert!

Das System funktioniert komplett zur Laufzeit:
1. Component-Definitionen werden in die JS eingebettet
2. JS liest bei `as="component-name"` die Definition
3. JS setzt die entsprechenden Namespace-Attribute
4. CSS rendert basierend auf den gesetzten Attributen

Keine `/src/css/presets/` Directory nötig - alles läuft über JS!

### 5. User-Defined Components (ACTUAL)
Components werden in `/src/js/mod.rs` verarbeitet:
```rust
// Embed component definitions in JS
for (name, component) in &components.components {
    js.push_str(&format!("      '{}': {{\n", name));
    if let Some(box_attr) = &component.box_ {
        js.push_str(&format!("        box: '{}',\n", box_attr));
    }
    // ... other namespace attributes
    js.push_str("      },\n");
}
```

Die JS wendet dann zur Laufzeit die Attribute an:
```javascript
applyComponentToElement: function(element, component) {
    if (component.box && !element.hasAttribute('box')) {
        element.setAttribute('box', component.box);
    }
    // ... other namespaces
}
```

## Implementation Steps

### Phase 1: Core Infrastructure
1. Create `/src/css/presets/` directory structure
2. Implement base preset generation system
3. Add CSS variable references for consistency
4. Integrate with main CSS generation

### Phase 2: Layout Presets
1. Implement container (max-width, centered)
2. Implement section (full-width, padding)
3. Implement hero (full-height, centered)
4. Test responsive behavior

### Phase 3: Component Presets
1. Implement card system (card, header, body, footer)
2. Implement alert variants
3. Implement badge variants
4. Add proper color mappings

### Phase 4: Interactive Presets
1. Implement button variants
2. Add hover/active states
3. Implement button-group layout
4. Test interaction states

### Phase 5: Form & Navigation
1. Implement form field presets
2. Create navigation components
3. Add modal/drawer presets
4. Implement data display presets

### Phase 6: User Components
1. Process YAML definitions
2. Support extends functionality
3. Handle component children
4. Test override behavior

## Testing Scenarios

### Test 1: Basic Presets
```html
<r-s as="card">
  <r-s as="card-header">Header</r-s>
  <r-s as="card-body">Content</r-s>
  <r-s as="card-footer">Footer</r-s>
</r-s>
```
Should render properly styled card.

### Test 2: Button Variants
```html
<r-s as="button-group">
  <r-s as="button-primary">Save</r-s>
  <r-s as="button-secondary">Cancel</r-s>
  <r-s as="button-ghost">Delete</r-s>
</r-s>
```
All buttons should have correct styling.

### Test 3: Component Extension
```yaml
# User defines:
my-special-card:
  extends: card
  face: "[bg:brand-a]"
```
Should inherit card styles plus custom background.

### Test 4: Responsive Behavior
```html
<r-s as="container">
  <r-s as="hero">
    Should be responsive
  </r-s>
</r-s>
```

## Success Criteria
- [x] Runtime component system implemented
- [x] Components defined in YAML work
- [x] JS applies namespace attributes dynamically
- [x] Component extension via 'extends' works
- [x] User-defined components supported
- [x] No build step required for users
- [x] Test pages demonstrate functionality
- [x] 3 example components provided
- [x] No regression in existing functionality

## Decision Log References
- DEC014: Reed elements default to div
- DEC015: Components without element field default to div
- Component system uses namespace attributes

## Dependencies
- Namespace system (RS907) - COMPLETED ✅
- HTML defaults system (RS918) - COMPLETED ✅
- Element system (RS901/919) - COMPLETED ✅

## Notes
- Presets are convenience - users can still use namespaces directly
- Keep presets minimal but complete
- Use CSS variables for all values (theming)
- Follow established patterns from other frameworks
- This is critical for real-world usage

## Example Generated CSS
```css
/* Container preset */
r-s[as="container"] {
  width: 100%;
  max-width: 1200px;
  margin-left: auto;
  margin-right: auto;
  padding-left: var(--reedstyle-space-4);
  padding-right: var(--reedstyle-space-4);
}

/* Hero preset */
r-s[as="hero"] {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  padding: var(--reedstyle-space-8);
  text-align: center;
}

/* Card preset */
r-s[as="card"] {
  display: block;
  padding: var(--reedstyle-space-6);
  background: var(--reedstyle-color-base-0);
  border-radius: var(--reedstyle-radius-lg);
  box-shadow: var(--reedstyle-shadow-md);
}
```