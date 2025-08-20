# RS911: Component Preset System - Implementation Plan

## Session Recovery Instructions
If session is lost, resume with:
1. Read this plan file
2. Check git branch: `git branch --show-current` (should be feature/RS911-component-presets)
3. Check current progress against TODO list
4. Continue from last unchecked item

## Files to Read (IN ORDER!)
CRITICAL: Read these files first to understand the existing system:

### 1. Understanding Current Configuration System
- `/src/config/mod.rs` - Component configuration structure (Component struct)
- `/reedstyle.components.yaml` - Current user-defined components structure
- `/src/css/mod.rs` - How CSS is generated from config

### 2. Namespace System (Components will use these)
- `/src/css/namespaces/mod.rs` - How namespaces work
- `/src/css/namespaces/box.rs` - Box namespace properties
- `/src/css/namespaces/face.rs` - Face namespace properties
- `/src/css/namespaces/layout.rs` - Layout namespace properties
- `/src/css/namespaces/text.rs` - Text namespace properties

### 3. Documentation References
- `/docs/develop/301-custom-components.md` - Custom component creation
- `/docs/develop/021-reed-element.md` - How as="component" works

## Critical Understanding

### What This Ticket Is About
This ticket establishes the **SYSTEM** for user-defined components via YAML.
Users can define components with ALL properties predefined, so they only need:
```html
<r-s as="my-component">Content</r-s>
```

Instead of:
```html
<r-s as="div" box="[padding:6]" face="[bg:base-0, radius:lg]" text="[align:center]">Content</r-s>
```

**WICHTIG - ACTUAL IMPLEMENTATION**:
- Components werden NICHT zur Build-Zeit in CSS kompiliert
- Die JS liest die Component-Definitionen zur LAUFZEIT
- JS setzt die Namespace-Attribute dynamisch
- CSS rendert basierend auf den gesetzten Attributen
- User muss NICHTS kompilieren - nur YAML editieren!

### Component Definition in YAML
```yaml
# reedstyle.components.yaml
components:
  my-card:
    element: div  # What HTML element to use
    box: "[padding:6, margin:2]"
    face: "[bg:base-0, radius:lg, shadow:md]"
    text: "[align:center]"
    layout: "[flex:column]"
    
  my-button:
    element: button
    box: "[padding-x:4, padding-y:2]"
    face: "[bg:brand-a, radius:md]"
    text: "[color:base-0, weight:bold]"
    device: "[cursor:pointer]"
```

## Implementation TODOs (ACTUAL)

### Phase 1: Parse YAML Components ✅
- [x] Read `/src/config/mod.rs` to understand Component struct
- [x] Ensure YAML parsing loads components correctly
- [x] Verify component structure has all namespace fields (box, face, text, layout, device, fx)
- [x] Test YAML loading with example components

### Phase 2: Embed Components in JS ✅
- [x] Update `/src/js/mod.rs` to embed component definitions
- [x] For each component in YAML, create JS object with properties
- [x] Include all namespace attributes in JS
- [x] Handle the `element` field (fallback to div if not specified)

### Phase 3: Runtime Application via JS ✅
- [x] Create `applyComponents()` function in JS
- [x] Apply namespace attributes to elements with matching `as` value
- [x] Only apply if attribute not already present (allow overrides)
- [x] Observe DOM for new elements with MutationObserver

### Phase 4: Add Example Components to YAML ✅
- [x] Create 3 useful example components in `reedstyle.components.yaml`:
  - `card` - Basic card with padding, background, shadow
  - `button-primary` - Primary button with hover state  
  - `container` - Centered max-width container
- [x] Test these components work in HTML

### Phase 5: Implement Component Extension ✅
- [x] Add support for `extends` field in YAML
- [x] Component can inherit from another component
- [x] Override specific properties while keeping others
- [x] Test extension mechanism via `resolveComponent()` in JS

### Phase 6: Testing ✅
- [x] Create `/test/test-components.html` test page
- [x] Create `/test/simple-components-test.html` for basic test
- [x] Test all 3 example components render correctly
- [x] Test user can add new components to YAML
- [x] Run `cargo build` and verify output

## Code Structure (ACTUAL)

### Component Processing (`/src/js/mod.rs` - NOT CSS!)
```rust
// Embed component definitions in JavaScript
js.push_str("    componentDefinitions: {\n");
for (name, component) in &components.components {
    js.push_str(&format!("      '{}': {{\n", name));
    if let Some(element) = &component.element {
        js.push_str(&format!("        element: '{}',\n", element));
    }
    if let Some(box_attr) = &component.box_ {
        js.push_str(&format!("        box: '{}',\n", box_attr));
    }
    if let Some(face) = &component.face {
        js.push_str(&format!("        face: '{}',\n", face));
    }
    // ... other namespaces
    js.push_str("      },\n");
}
js.push_str("    },\n");
```

### Runtime Application (JavaScript)
```javascript
applyComponentToElement: function(element, component) {
    // Only apply if attribute not already present
    if (component.box && !element.hasAttribute('box')) {
        element.setAttribute('box', component.box);
    }
    if (component.face && !element.hasAttribute('face')) {
        element.setAttribute('face', component.face);
    }
    // ... other namespaces
}
```

### Example YAML Components
```yaml
# reedstyle.components.yaml
components:
  # Example 1: Card component
  card:
    element: div
    box: "[padding:6, margin:2]"
    face: "[bg:base-0, radius:lg, shadow:md]"
    
  # Example 2: Primary button
  button-primary:
    element: button
    box: "[padding-x:6, padding-y:3]"
    face: "[bg:brand-a, radius:md, border:none]"
    text: "[color:base-0, weight:medium]"
    device: "[cursor:pointer]"
    
  # Example 3: Container
  container:
    element: div
    box: "[width:full, max-width:1200, margin-x:auto, padding-x:4]"
```

## Testing Checklist ✅
- [x] YAML components are parsed correctly
- [x] JS embeds component definitions
- [x] Components work in HTML with just `as="name"`
- [x] Example components (card, button-primary, container) work
- [x] User can add new components to YAML
- [x] Extension mechanism works
- [x] Runtime application functions correctly

## Decision References
- Components defined in YAML for easy customization
- No hardcoded presets in Rust - all from YAML
- Support component extension via `extends` field
- Keep system flexible and user-controlled

## Common Pitfalls to Avoid (LESSONS LEARNED)
1. **Misunderstanding the goal** - This is about the SYSTEM, not built-in presets ✅
2. **Hardcoding components** - Everything comes from YAML ✅
3. **WRONG: Generating CSS at build-time** - Components work at RUNTIME via JS! ⚠️
4. **Not parsing namespaces** - Must handle box, face, text, layout, device, fx ✅
5. **Thinking users need to compile** - NO! Just edit YAML and reload! ✅

## Success Criteria ✅
- [x] User can define components in YAML
- [x] Components apply at runtime via JS
- [x] `<r-s as="my-component">` works
- [x] 3 example components in YAML work
- [x] Extension mechanism functional
- [x] System is flexible and extensible
- [x] Build succeeds without errors
- [x] NO BUILD STEP REQUIRED FOR USERS!

## Git Workflow
```bash
# Create feature branch
git checkout -b feature/RS911-component-presets

# Regular commits as you progress
git add -A
git commit -m "feat(RS911): Add layout presets"
git commit -m "feat(RS911): Add button presets with hover states"

# Final squash before merge
git reset --soft main
git commit -m "feat(RS911): Implement complete component preset system

- Added 40+ built-in presets across 7 categories
- Implemented CSS variables for all values
- Support user-defined components
- Added component extension via 'extends' field
- Created comprehensive test page

Closes #911"
```

## Notes
- This is CRITICAL for real-world usage
- Users expect these components to "just work"
- Keep implementation simple and maintainable
- Test thoroughly - this is user-facing
- Consider performance with many presets