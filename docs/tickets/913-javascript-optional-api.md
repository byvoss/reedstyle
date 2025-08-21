# Ticket #913: JavaScript Optional API

## Status: 📋 Planned

## Overview
Implement the complete optional JavaScript API that provides progressive enhancement for ReedSTYLE. The API adds dynamic features like component loading, state management, effects control, and form enhancements while keeping CSS as the foundation.

## Files to Read First (Understanding Current System)
Before implementing, read these files to understand the existing JavaScript generation:

1. **Current JavaScript System**:
   - `/src/js/mod.rs` - Current JavaScript generation including Typography and Effects engines
   - `/src/typography/mod.rs` - Typography engine (already integrated)
   - `/src/builder/mod.rs` - How JavaScript files are built

2. **Effects System**:
   - `/docs/tickets/done/917-effects-system-[PLAN].md` - Effects implementation
   - `/test/test-effects.html` - Effects testing

3. **Documentation for Reference**:
   - `/docs/develop/321-javascript-api.md` - Complete API specification
   - `/docs/develop/021-reed-element.md` - How r-s elements work

## Objectives
- Create comprehensive JavaScript API for progressive enhancement
- Implement component registration and loading from YAML
- Add state management system
- Provide form validation and Ajax handling
- Enable dynamic effects and animations
- Maintain zero-dependency for core CSS functionality

## Technical Requirements

### 1. Core API Structure
```javascript
window.ReedStyle = {
  // Version info
  version: '1.0.0',
  
  // Core methods
  init: function(config) {},
  createElement: function(as, props) {},
  
  // Component system
  components: {
    register: function(name, definition) {},
    load: function(yamlPath) {},
    apply: function(element) {}
  },
  
  // State management
  state: {
    create: function(initialState) {},
    subscribe: function(callback) {},
    set: function(newState) {},
    get: function() {}
  },
  
  // Effects (already exists)
  effects: { /* existing */ },
  
  // Typography (already exists)
  typography: { /* existing */ },
  
  // Forms
  forms: {
    validate: function(formId, rules) {},
    ajax: function(formId, options) {}
  },
  
  // Utilities
  utils: {
    query: function(selector) {},
    queryAll: function(selector) {},
    on: function(event, selector, handler) {},
    debounce: function(fn, delay) {},
    throttle: function(fn, delay) {}
  }
};
```

### 2. Component Registration
Dynamic component loading from YAML:
```javascript
// Load components from YAML
ReedStyle.components.load('./reedstyle.components.yaml')
  .then(components => {
    console.log('Loaded', Object.keys(components).length, 'components');
  });

// Register single component
ReedStyle.components.register('my-card', {
  element: 'div',
  box: '[padding:6]',
  face: '[bg:base-0, radius:lg]'
});

// Apply to element
const element = document.querySelector('r-s[as="my-card"]');
ReedStyle.components.apply(element);
```

### 3. State Management
Simple reactive state system:
```javascript
// Create store
const store = ReedStyle.state.create({
  theme: 'light',
  user: null,
  cart: []
});

// Subscribe to changes
store.subscribe((state, prevState) => {
  if (state.theme !== prevState.theme) {
    document.body.className = `theme-${state.theme}`;
  }
});

// Update state
store.set({ theme: 'dark' });

// Get current state
const currentTheme = store.get().theme;
```

### 4. Form Enhancements
Validation and Ajax submission:
```javascript
// Validation
ReedStyle.forms.validate('contact-form', {
  rules: {
    email: { required: true, email: true },
    phone: { pattern: /^\d{10}$/ }
  },
  messages: {
    email: {
      required: 'Email is required',
      email: 'Invalid email format'
    }
  },
  onValid: (data) => console.log('Valid:', data),
  onInvalid: (errors) => console.log('Errors:', errors)
});

// Ajax submission
ReedStyle.forms.ajax('newsletter', {
  url: '/api/subscribe',
  method: 'POST',
  onSuccess: (response) => {
    ReedStyle.notify('Subscribed!', 'success');
  },
  onError: (error) => {
    ReedStyle.notify('Failed', 'error');
  }
});
```

### 5. Element Creation
Programmatic element creation:
```javascript
// Create element
const card = ReedStyle.createElement('card', {
  box: '[padding:8]',
  face: '[bg:brand-a]',
  children: [
    { as: 'h3', text: 'Title' },
    { as: 'p', text: 'Content' }
  ]
});

// Append to DOM
document.body.appendChild(card);
```

### 6. Event Delegation
Efficient event handling:
```javascript
// Delegate click events
ReedStyle.utils.on('click', 'r-s[as="button"]', (e) => {
  console.log('Button clicked:', e.target);
});

// Debounced search
const search = ReedStyle.utils.debounce((query) => {
  console.log('Searching:', query);
}, 300);
```

## Implementation Steps

### Phase 1: Core Structure
1. Update `/src/js/mod.rs` with full API structure
2. Add version management
3. Create initialization system
4. Set up module pattern

### Phase 2: Component System
1. Implement YAML loader (fetch and parse)
2. Create component registry
3. Add dynamic application to elements
4. Support component inheritance

### Phase 3: State Management
1. Create reactive store
2. Implement pub/sub pattern
3. Add state persistence option
4. Create computed properties

### Phase 4: Form System
1. Add validation engine
2. Create Ajax handler
3. Implement error display
4. Add progress indicators

### Phase 5: Utilities
1. Create DOM query helpers
2. Add event delegation
3. Implement debounce/throttle
4. Add animation utilities

### Phase 6: Integration
1. Ensure works with existing Typography engine
2. Ensure works with existing Effects engine
3. Add initialization auto-detection
4. Create comprehensive examples

## Testing Scenarios

### Test 1: Component Loading
```javascript
// Should load and apply components
ReedStyle.init({
  components: './test-components.yaml'
});
```

### Test 2: State Management
```javascript
// Should trigger updates
const store = ReedStyle.state.create({ count: 0 });
store.subscribe(state => {
  document.querySelector('#count').textContent = state.count;
});
store.set({ count: 5 });
```

### Test 3: Form Validation
```html
<r-s as="form" id="test-form">
  <input name="email" type="email" required>
  <button type="submit">Submit</button>
</r-s>
```

### Test 4: Event Delegation
```javascript
// Should handle dynamic elements
ReedStyle.utils.on('click', 'r-s[fx*="ripple"]', (e) => {
  ReedStyle.effects.ripple(e.target, e);
});
```

## Success Criteria
- [ ] Complete API structure implemented
- [ ] Component loading from YAML works
- [ ] State management functional
- [ ] Form validation operational
- [ ] Ajax forms working
- [ ] Event delegation efficient
- [ ] All utilities functional
- [ ] Works with Typography engine
- [ ] Works with Effects engine
- [ ] Documentation examples work
- [ ] No breaking changes to CSS-only usage
- [ ] File size <100KB unminified, <40KB minified

## Decision Log References
- JavaScript is optional enhancement
- CSS provides core functionality
- Progressive enhancement approach

## Dependencies
- Typography engine (RS916) - COMPLETED ✅
- Effects system (RS917) - COMPLETED ✅
- JavaScript generation (RS906) - COMPLETED ✅

## Notes
- Keep API surface small but powerful
- Focus on common use cases
- Maintain backward compatibility
- Consider future plugin system
- Test without JavaScript enabled

## Code Generation in Rust
Update `/src/js/mod.rs`:
```rust
pub fn generate_javascript() -> String {
    let mut js = String::new();
    
    // Add header
    js.push_str("/*! ReedStyle v1.0.0 | Apache-2.0 */\n");
    
    // Create namespace
    js.push_str("window.ReedStyle = (function() {\n");
    
    // Add modules
    js.push_str(&generate_core_module());
    js.push_str(&generate_component_module());
    js.push_str(&generate_state_module());
    js.push_str(&generate_form_module());
    js.push_str(&generate_utils_module());
    
    // Add existing engines
    js.push_str(&typography::generate_js());
    js.push_str(&effects::generate_js());
    
    // Return public API
    js.push_str("  return { init, createElement, components, state, forms, utils, typography, effects };\n");
    js.push_str("})();\n");
    
    // Auto-initialize
    js.push_str("if (document.readyState === 'loading') {\n");
    js.push_str("  document.addEventListener('DOMContentLoaded', () => ReedStyle.init());\n");
    js.push_str("} else {\n");
    js.push_str("  ReedStyle.init();\n");
    js.push_str("}\n");
    
    js
}
```