# Custom Components

Create reusable components with YAML configuration.

## Overview

Custom components extend ReedSTYLE's preset system, allowing you to define your own reusable patterns without writing CSS.

## Configuration Location

Components are defined in `reedstyle.components.yaml` in your project root:

```
project-root/
├── reedstyle.config.yaml      # References component file
├── reedstyle.components.yaml  # Your custom components
├── reedstyle.colors.yaml      # Brand colors (used in components)
└── reedstyle.fonts.yaml       # Typography (used in components)
```

## Basic Component Definition

### Simple Component

```yaml
# reedstyle.components.yaml
components:
  alert-box:
    element: div
    box: "[padding:4, margin:2]"
    face: "[bg:state-warning-weak, border:2:state-warning, radius:md]"
    text: "[color:state-warning-strong, weight:medium]"
```

Usage:

```html
<reed as="alert-box">
  Warning: This action cannot be undone.
</reed>
```

### Component with Children

```yaml
components:
  feature-card:
    element: article
    layout: "[flex:column, gap:4]"
    box: "[padding:6]"
    face: "[bg:base-0, radius:xl, shadow:lg]"
    children:
      icon:
        element: div
        box: "[width:12, height:12]"
        face: "[bg:brand-a, radius:full]"
      title:
        element: h3
        text: "[size:large, weight:bold]"
      description:
        element: p
        text: "[color:base-700]"
```

Usage:

```html
<reed as="feature-card">
  <reed as="feature-card-icon">🚀</reed>
  <reed as="feature-card-title">Fast Performance</reed>
  <reed as="feature-card-description">
    Lightning-fast load times with optimized CSS.
  </reed>
</reed>
```

## Advanced Components

### Component Variants

```yaml
components:
  button:
    element: button
    box: "[padding-x:6, padding-y:3]"
    face: "[radius:md]"
    text: "[weight:medium]"
    device: "[cursor:pointer]"
    variants:
      primary:
        face: "[bg:brand-a]"
        text: "[color:base-0]"
        fx: "[hover:brightness:110]"
      secondary:
        face: "[bg:base-200]"
        text: "[color:base-900]"
        fx: "[hover:bg:base-300]"
      ghost:
        face: "[bg:transparent, border:1:base-400]"
        text: "[color:base-700]"
        fx: "[hover:bg:base-100]"
```

Usage:

```html
<reed as="button-primary">Save Changes</reed>
<reed as="button-secondary">Cancel</reed>
<reed as="button-ghost">Learn More</reed>
```

### Responsive Components

```yaml
components:
  responsive-grid:
    element: div
    layout: "[flex:column, gap:4]"
    layout-tablet: "[grid:2, gap:6]"
    layout-screen: "[grid:3, gap:8]"
    layout-wide: "[grid:4, gap:10]"
```

### Nested Components

```yaml
components:
  pricing-card:
    element: div
    layout: "[flex:column]"
    box: "[padding:8]"
    face: "[bg:base-0, border:2:brand-a, radius:xl]"
    children:
      header:
        element: div
        box: "[padding-bottom:6]"
        face: "[border-bottom:1:base-200]"
      price:
        element: div
        layout: "[flex:row, align:baseline, gap:2]"
        children:
          currency:
            element: span
            text: "[size:large, color:base-600]"
          amount:
            element: span
            text: "[size:huge, weight:bold, color:brand-a]"
          period:
            element: span
            text: "[size:small, color:base-600]"
      features:
        element: ul
        box: "[padding-y:6]"
        text: "[list:none]"
        children:
          item:
            element: li
            box: "[padding-y:2]"
            layout: "[flex:row, gap:2]"
      action:
        element: div
        box: "[padding-top:6]"
        face: "[border-top:1:base-200]"
```

Usage:

```html
<reed as="pricing-card">
  <reed as="pricing-card-header">
    <h3>Professional</h3>
  </reed>
  <reed as="pricing-card-price">
    <reed as="pricing-card-currency">$</reed>
    <reed as="pricing-card-amount">29</reed>
    <reed as="pricing-card-period">/month</reed>
  </reed>
  <reed as="pricing-card-features">
    <reed as="pricing-card-item">✓ Unlimited projects</reed>
    <reed as="pricing-card-item">✓ Priority support</reed>
    <reed as="pricing-card-item">✓ Advanced analytics</reed>
  </reed>
  <reed as="pricing-card-action">
    <reed as="button-primary" box="width:full">Subscribe</reed>
  </reed>
</reed>
```

## Component Composition

### Extending Existing Components

```yaml
components:
  # Base card
  card:
    element: div
    box: "[padding:6]"
    face: "[bg:base-0, radius:lg, shadow:md]"
    
  # Extended card with hover
  card-interactive:
    extends: card
    device: "[cursor:pointer]"
    fx: "[hover:lift, transition:smooth]"
    face-hover: "[shadow:xl]"
    
  # Further extension
  product-card:
    extends: card-interactive
    layout: "[flex:column, gap:4]"
    children:
      image:
        element: img
        box: "[width:full, aspect:photo]"
        face: "[radius:md]"
      content:
        element: div
        layout: "[flex:column, gap:2]"
```

### Mixins

```yaml
mixins:
  glass-effect:
    face: "[bg:base-0, opacity:80, backdrop:blur-md]"
    
  brand-gradient:
    face: "[bg:gradient-primary]"
    
  centered:
    layout: "[flex, align:center, justify:center]"

components:
  glass-card:
    element: div
    mixins: [glass-effect]
    box: "[padding:6]"
    face: "[border:1:base-200, radius:xl]"
    
  hero-section:
    element: section
    mixins: [brand-gradient, centered]
    box: "[height:screen, padding:8]"
```

## Dynamic Components

### State-Based Styling

```yaml
components:
  form-input:
    element: input
    box: "[padding:3, width:full]"
    face: "[bg:base-0, border:1:base-300, radius:md]"
    text: "[size:normal]"
    states:
      focus:
        face: "[border:2:brand-a, outline:none]"
      invalid:
        face: "[border:2:state-error]"
        text: "[color:state-error]"
      disabled:
        face: "[bg:base-100, opacity:60]"
        device: "[cursor:not-allowed]"
```

### Conditional Properties

```yaml
components:
  notification:
    element: div
    box: "[padding:4]"
    face: "[radius:md]"
    layout: "[flex:row, gap:3, align:center]"
    conditions:
      type:
        success:
          face: "[bg:state-success-weak, border:1:state-success]"
          text: "[color:state-success-strong]"
        warning:
          face: "[bg:state-warning-weak, border:1:state-warning]"
          text: "[color:state-warning-strong]"
        error:
          face: "[bg:state-error-weak, border:1:state-error]"
          text: "[color:state-error-strong]"
```

Usage:

```html
<reed as="notification" type="success">
  Operation completed successfully!
</reed>
```

## Component Library

### Building a Design System

```yaml
# design-system.yaml
theme:
  spacing:
    compact: 2
    normal: 4
    relaxed: 6
    spacious: 8
    
components:
  # Typography components
  heading-1:
    element: h1
    text: "[size:mega, weight:bold, leading:tight]"
    
  heading-2:
    element: h2
    text: "[size:huge, weight:semibold]"
    
  body-text:
    element: p
    text: "[size:normal, leading:relaxed]"
    
  # Layout components
  container:
    element: div
    box: "[width:full, max-width:1200, margin-x:auto, padding-x:4]"
    
  section:
    element: section
    box: "[padding-y:16]"
    
  # UI components
  chip:
    element: span
    box: "[padding-x:3, padding-y:1]"
    face: "[bg:base-200, radius:full]"
    text: "[size:small]"
```

### Sharing Components

Components can be shared via npm:

```json
// package.json
{
  "name": "@myorg/reedstyle-components",
  "version": "1.0.0",
  "main": "components.yaml",
  "files": ["components.yaml"]
}
```

Install and use:

```yaml
# reedstyle.config.yaml
components:
  - "./reedstyle.components.yaml"
  - "node_modules/@myorg/reedstyle-components/components.yaml"
```

## Best Practices

### 1. Naming Conventions

```yaml
# Good: Descriptive, hierarchical
components:
  card: {}
  card-header: {}
  card-body: {}
  card-footer: {}
  
# Avoid: Generic or conflicting names
components:
  box: {}      # Too generic
  header: {}   # Conflicts with HTML
```

### 2. Component Organization

```yaml
# Group related components
components:
  # Navigation
  nav: {}
  nav-brand: {}
  nav-links: {}
  
  # Forms
  form: {}
  form-field: {}
  form-error: {}
  
  # Data display
  table: {}
  table-header: {}
  table-row: {}
```

### 3. Property Efficiency

```yaml
# Good: Use arrays for multiple properties
components:
  efficient:
    box: "[padding:4, margin:2, width:full]"
    
# Less efficient: Separate attributes
components:
  inefficient:
    padding: "4"
    margin: "2"
    width: "full"
```

### 4. Reusability

```yaml
# Good: Flexible base components
components:
  button-base:
    element: button
    box: "[padding-x:6, padding-y:3]"
    face: "[radius:md]"
    device: "[cursor:pointer]"
    
# Extend for variants
  button-primary:
    extends: button-base
    face: "[bg:brand-a]"
    text: "[color:base-0]"
```

## Component Testing

### Visual Testing

Create a test page:

```html
<!DOCTYPE html>
<html>
<head>
  <link rel="stylesheet" href="reedstyle.css">
</head>
<body>
  <reed as="container">
    <h1>Component Gallery</h1>
    
    <!-- Test each component -->
    <section>
      <h2>Buttons</h2>
      <reed as="button-primary">Primary</reed>
      <reed as="button-secondary">Secondary</reed>
      <reed as="button-ghost">Ghost</reed>
    </section>
    
    <section>
      <h2>Cards</h2>
      <reed as="card">Basic Card</reed>
      <reed as="product-card">Product Card</reed>
    </section>
  </reed>
</body>
</html>
```

### Automated Testing

```javascript
// component.test.js
describe('Custom Components', () => {
  test('loads component definitions', () => {
    const components = ReedStyle.loadComponents('./components.yaml');
    expect(components).toHaveProperty('card');
  });
  
  test('applies component styles', () => {
    const el = document.createElement('reed');
    el.setAttribute('as', 'card');
    
    ReedStyle.applyComponent(el, 'card');
    
    expect(el.getAttribute('box')).toContain('padding:6');
  });
});
```

## Next: [Responsive Design](311-responsive-design.md)