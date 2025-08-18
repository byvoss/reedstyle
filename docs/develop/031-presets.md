# Presets

Built-in components that cover 90% of common UI patterns.

## What Are Presets?

Presets are pre-styled reed elements that provide instant, professional designs:

```html
<!-- Instead of this... -->
<reed as="div" 
      box="[padding:6, margin:4]"
      face="[bg:base-0, radius:lg, shadow:md]">
  Content
</reed>

<!-- Just write this -->
<reed as="card">
  Content
</reed>
```

## Layout Presets

### container
Centered max-width container with responsive padding.

```html
<reed as="container">
  <!-- Content constrained to max-width with auto margins -->
</reed>
```

### section
Full-width section with vertical padding.

```html
<reed as="section">
  <!-- Full-width content area -->
</reed>
```

### hero
Full-height centered hero section.

```html
<reed as="hero">
  <h1>Big Title</h1>
  <p>Subtitle text</p>
  <reed as="button-primary">Call to Action</reed>
</reed>
```

### sidebar-layout
Two-column layout with sidebar.

```html
<reed as="sidebar-layout">
  <reed as="sidebar">
    <!-- Sidebar content -->
  </reed>
  <reed as="main-content">
    <!-- Main content -->
  </reed>
</reed>
```

## Component Presets

### card
Content card with shadow, radius, and padding.

```html
<reed as="card">
  <h3>Card Title</h3>
  <p>Card content</p>
</reed>

<!-- With sections -->
<reed as="card">
  <reed as="card-header">Header</reed>
  <reed as="card-body">Content</reed>
  <reed as="card-footer">Footer</reed>
</reed>
```

### alert
Notification/alert component.

```html
<reed as="alert">Default alert</reed>
<reed as="alert-success">Success message</reed>
<reed as="alert-warning">Warning message</reed>
<reed as="alert-error">Error message</reed>
<reed as="alert-info">Info message</reed>
```

### badge
Small label/badge component.

```html
<reed as="badge">Default</reed>
<reed as="badge-primary">Primary</reed>
<reed as="badge-success">Success</reed>
<reed as="badge-warning">Warning</reed>
<reed as="badge-error">Error</reed>
```

## Button Presets

### button-primary
Primary action button.

```html
<reed as="button-primary">Save Changes</reed>
```

### button-secondary
Secondary action button.

```html
<reed as="button-secondary">Cancel</reed>
```

### button-ghost
Transparent button with border.

```html
<reed as="button-ghost">Learn More</reed>
```

### button-link
Button that looks like a link.

```html
<reed as="button-link">View Details</reed>
```

### button-group
Container for multiple buttons.

```html
<reed as="button-group">
  <reed as="button-secondary">Back</reed>
  <reed as="button-primary">Next</reed>
</reed>
```

## Navigation Presets

### nav
Navigation bar container.

```html
<reed as="nav">
  <reed as="nav-brand">Logo</reed>
  <reed as="nav-links">
    <a href="/">Home</a>
    <a href="/about">About</a>
    <a href="/contact">Contact</a>
  </reed>
</reed>
```

### breadcrumb
Breadcrumb navigation.

```html
<reed as="breadcrumb">
  <a href="/">Home</a>
  <span>/</span>
  <a href="/products">Products</a>
  <span>/</span>
  <span>Current Page</span>
</reed>
```

### tabs
Tab navigation component.

```html
<reed as="tabs">
  <reed as="tab-list">
    <reed as="tab" active>Tab 1</reed>
    <reed as="tab">Tab 2</reed>
    <reed as="tab">Tab 3</reed>
  </reed>
  <reed as="tab-panel">
    Tab 1 content
  </reed>
</reed>
```

## Form Presets

### form
Styled form container.

```html
<reed as="form">
  <reed as="field">
    <label>Name</label>
    <input type="text">
  </reed>
  <reed as="field">
    <label>Email</label>
    <input type="email">
  </reed>
  <reed as="button-primary">Submit</reed>
</reed>
```

### field
Form field wrapper with label spacing.

```html
<reed as="field">
  <label>Field Label</label>
  <input type="text">
  <reed as="field-help">Helper text</reed>
  <reed as="field-error">Error message</reed>
</reed>
```

### field-group
Group related fields.

```html
<reed as="field-group">
  <reed as="field">
    <label>First Name</label>
    <input type="text">
  </reed>
  <reed as="field">
    <label>Last Name</label>
    <input type="text">
  </reed>
</reed>
```

## Modal Presets

### modal
Modal dialog container.

```html
<reed as="modal" id="my-modal">
  <reed as="modal-header">
    <h3>Modal Title</h3>
    <reed as="modal-close">&times;</reed>
  </reed>
  <reed as="modal-body">
    Modal content goes here
  </reed>
  <reed as="modal-footer">
    <reed as="button-secondary">Cancel</reed>
    <reed as="button-primary">Confirm</reed>
  </reed>
</reed>
```

### drawer
Slide-out drawer component.

```html
<reed as="drawer" position="left">
  <reed as="drawer-header">
    <h3>Menu</h3>
    <reed as="drawer-close">&times;</reed>
  </reed>
  <reed as="drawer-body">
    <!-- Menu items -->
  </reed>
</reed>
```

## List Presets

### list
Styled list container.

```html
<reed as="list">
  <reed as="list-item">Item 1</reed>
  <reed as="list-item">Item 2</reed>
  <reed as="list-item">Item 3</reed>
</reed>
```

### menu
Vertical menu component.

```html
<reed as="menu">
  <reed as="menu-item" active>Dashboard</reed>
  <reed as="menu-item">Profile</reed>
  <reed as="menu-item">Settings</reed>
  <reed as="menu-divider"></reed>
  <reed as="menu-item">Logout</reed>
</reed>
```

## Data Display Presets

### table
Styled table wrapper.

```html
<reed as="table">
  <table>
    <thead>
      <tr>
        <th>Name</th>
        <th>Email</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <td>John Doe</td>
        <td>john@example.com</td>
      </tr>
    </tbody>
  </table>
</reed>
```

### stats
Statistics display component.

```html
<reed as="stats">
  <reed as="stat">
    <reed as="stat-value">42</reed>
    <reed as="stat-label">Users</reed>
  </reed>
  <reed as="stat">
    <reed as="stat-value">$1,234</reed>
    <reed as="stat-label">Revenue</reed>
  </reed>
</reed>
```

### progress
Progress bar component.

```html
<reed as="progress" value="60">
  60% Complete
</reed>
```

## Customizing Presets

### Override Specific Properties

```html
<!-- Change card background -->
<reed as="card" face="bg:brand-a">
  Branded card
</reed>

<!-- Adjust button size -->
<reed as="button-primary" text="size:small">
  Small button
</reed>
```

### Combine Presets with Custom Styles

```html
<reed as="hero" 
      text="[align:left]"
      box="[padding:12]">
  <h1>Custom Hero</h1>
</reed>
```

### Responsive Preset Modifications

```html
<reed as="card"
      box="[padding:4]"
      box-tablet="[padding:6]"
      box-screen="[padding:8]">
  Responsive card padding
</reed>
```

## Creating Your Own Presets

Define custom presets in `reedstyle.components.yaml`:

```yaml
components:
  feature-card:
    element: article
    layout: "[flex:column, align:center]"
    box: "[padding:8]"
    face: "[bg:gradient-primary, radius:xl]"
    text: "[align:center]"
    
  pricing-card:
    element: div
    layout: "[flex:column]"
    box: "[padding:6, margin:4]"
    face: "[bg:base-0, border:2:brand-a, radius:lg]"
    fx: "[hover:lift]"
```

Use them like built-in presets:

```html
<reed as="feature-card">
  <h3>Feature Name</h3>
  <p>Feature description</p>
</reed>

<reed as="pricing-card">
  <h3>Pro Plan</h3>
  <reed as="price">$29/mo</reed>
  <reed as="button-primary">Subscribe</reed>
</reed>
```

## Best Practices

### 1. Use Presets First
Always check if a preset exists before creating custom styles.

### 2. Semantic Naming
Choose preset names that describe purpose, not appearance.

### 3. Minimal Overrides
If you're heavily customizing a preset, consider creating a new one.

### 4. Consistent Patterns
Follow ReedSTYLE conventions when creating custom presets.

## Next Steps

- [Namespaces Overview](101-namespaces-overview.md) - Understanding properties
- [Custom Components](301-custom-components.md) - Advanced preset creation
- [Examples](https://reedstyle.dev/gallery) - See all presets in action