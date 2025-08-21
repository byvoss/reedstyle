# Presets

Built-in components that cover 90% of common UI patterns.

## What Are Presets?

Presets are pre-styled reed elements that provide instant, professional designs:

```html
<!-- Instead of this... -->
<r-s as="div" 
      box="[padding:6, margin:4]"
      face="[bg:base-0, radius:lg, shadow:md]">
  Content
</r-s>

<!-- Just write this -->
<r-s as="card">
  Content
</r-s>
```

## Layout Presets

### container
Centered max-width container with responsive padding.

```html
<r-s as="container">
  <!-- Content constrained to max-width with auto margins -->
</r-s>
```

### section
Full-width section with vertical padding.

```html
<r-s as="section">
  <!-- Full-width content area -->
</r-s>
```

### hero
Full-height centered hero section.

```html
<r-s as="hero">
  <h1>Big Title</h1>
  <p>Subtitle text</p>
  <r-s as="button-primary">Call to Action</r-s>
</r-s>
```

### sidebar-layout
Two-column layout with sidebar.

```html
<r-s as="sidebar-layout">
  <r-s as="sidebar">
    <!-- Sidebar content -->
  </r-s>
  <r-s as="main-content">
    <!-- Main content -->
  </r-s>
</r-s>
```

## Component Presets

### card
Content card with shadow, radius, and padding.

```html
<r-s as="card">
  <h3>Card Title</h3>
  <p>Card content</p>
</r-s>

<!-- With sections -->
<r-s as="card">
  <r-s as="card-header">Header</r-s>
  <r-s as="card-body">Content</r-s>
  <r-s as="card-footer">Footer</r-s>
</r-s>
```

### alert
Notification/alert component.

```html
<r-s as="alert">Default alert</r-s>
<r-s as="alert-success">Success message</r-s>
<r-s as="alert-warning">Warning message</r-s>
<r-s as="alert-error">Error message</r-s>
<r-s as="alert-info">Info message</r-s>
```

### badge
Small label/badge component.

```html
<r-s as="badge">Default</r-s>
<r-s as="badge-primary">Primary</r-s>
<r-s as="badge-success">Success</r-s>
<r-s as="badge-warning">Warning</r-s>
<r-s as="badge-error">Error</r-s>
```

## Button Presets

### button-primary
Primary action button.

```html
<r-s as="button-primary">Save Changes</r-s>
```

### button-secondary
Secondary action button.

```html
<r-s as="button-secondary">Cancel</r-s>
```

### button-ghost
Transparent button with border.

```html
<r-s as="button-ghost">Learn More</r-s>
```

### button-link
Button that looks like a link.

```html
<r-s as="button-link">View Details</r-s>
```

### button-group
Container for multiple buttons.

```html
<r-s as="button-group">
  <r-s as="button-secondary">Back</r-s>
  <r-s as="button-primary">Next</r-s>
</r-s>
```

## Navigation Presets

### nav
Navigation bar container.

```html
<r-s as="nav">
  <r-s as="nav-brand">Logo</r-s>
  <r-s as="nav-links">
    <a href="/">Home</a>
    <a href="/about">About</a>
    <a href="/contact">Contact</a>
  </r-s>
</r-s>
```

### breadcrumb
Breadcrumb navigation.

```html
<r-s as="breadcrumb">
  <a href="/">Home</a>
  <span>/</span>
  <a href="/products">Products</a>
  <span>/</span>
  <span>Current Page</span>
</r-s>
```

### tabs
Tab navigation component.

```html
<r-s as="tabs">
  <r-s as="tab-list">
    <r-s as="tab" active>Tab 1</r-s>
    <r-s as="tab">Tab 2</r-s>
    <r-s as="tab">Tab 3</r-s>
  </r-s>
  <r-s as="tab-panel">
    Tab 1 content
  </r-s>
</r-s>
```

## Form Presets

### form
Styled form container.

```html
<r-s as="form">
  <r-s as="field">
    <label>Name</label>
    <input type="text">
  </r-s>
  <r-s as="field">
    <label>Email</label>
    <input type="email">
  </r-s>
  <r-s as="button-primary">Submit</r-s>
</r-s>
```

### field
Form field wrapper with label spacing.

```html
<r-s as="field">
  <label>Field Label</label>
  <input type="text">
  <r-s as="field-help">Helper text</r-s>
  <r-s as="field-error">Error message</r-s>
</r-s>
```

### field-group
Group related fields.

```html
<r-s as="field-group">
  <r-s as="field">
    <label>First Name</label>
    <input type="text">
  </r-s>
  <r-s as="field">
    <label>Last Name</label>
    <input type="text">
  </r-s>
</r-s>
```

## Modal Presets

### modal
Modal dialog container.

```html
<r-s as="modal" id="my-modal">
  <r-s as="modal-header">
    <h3>Modal Title</h3>
    <r-s as="modal-close">&times;</r-s>
  </r-s>
  <r-s as="modal-body">
    Modal content goes here
  </r-s>
  <r-s as="modal-footer">
    <r-s as="button-secondary">Cancel</r-s>
    <r-s as="button-primary">Confirm</r-s>
  </r-s>
</r-s>
```

### drawer
Slide-out drawer component.

```html
<r-s as="drawer" position="left">
  <r-s as="drawer-header">
    <h3>Menu</h3>
    <r-s as="drawer-close">&times;</r-s>
  </r-s>
  <r-s as="drawer-body">
    <!-- Menu items -->
  </r-s>
</r-s>
```

## List Presets

### list
Styled list container.

```html
<r-s as="list">
  <r-s as="list-item">Item 1</r-s>
  <r-s as="list-item">Item 2</r-s>
  <r-s as="list-item">Item 3</r-s>
</r-s>
```

### menu
Vertical menu component.

```html
<r-s as="menu">
  <r-s as="menu-item" active>Dashboard</r-s>
  <r-s as="menu-item">Profile</r-s>
  <r-s as="menu-item">Settings</r-s>
  <r-s as="menu-divider"></r-s>
  <r-s as="menu-item">Logout</r-s>
</r-s>
```

## Data Display Presets

### table
Styled table wrapper.

```html
<r-s as="table">
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
</r-s>
```

### stats
Statistics display component.

```html
<r-s as="stats">
  <r-s as="stat">
    <r-s as="stat-value">42</r-s>
    <r-s as="stat-label">Users</r-s>
  </r-s>
  <r-s as="stat">
    <r-s as="stat-value">$1,234</r-s>
    <r-s as="stat-label">Revenue</r-s>
  </r-s>
</r-s>
```

### progress
Progress bar component.

```html
<r-s as="progress" value="60">
  60% Complete
</r-s>
```

## Customizing Presets

### Override Specific Properties

```html
<!-- Change card background -->
<r-s as="card" face="bg:brand-a">
  Branded card
</r-s>

<!-- Adjust button size -->
<r-s as="button-primary" text="size:small">
  Small button
</r-s>
```

### Combine Presets with Custom Styles

```html
<r-s as="hero" 
      text="[align:left]"
      box="[padding:12]">
  <h1>Custom Hero</h1>
</r-s>
```

### Responsive Preset Modifications

```html
<r-s as="card"
      box="[padding:4]"
      box-tablet="[padding:6]"
      box-screen="[padding:8]">
  Responsive card padding
</r-s>
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
<r-s as="feature-card">
  <h3>Feature Name</h3>
  <p>Feature description</p>
</r-s>

<r-s as="pricing-card">
  <h3>Pro Plan</h3>
  <r-s as="price">$29/mo</r-s>
  <r-s as="button-primary">Subscribe</r-s>
</r-s>
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