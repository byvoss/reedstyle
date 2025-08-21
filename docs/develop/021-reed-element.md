# The Reed Element

## Overview

The `<r-s>` element is the heart of ReedSTYLE. It's a custom HTML element that accepts styling attributes directly, eliminating the need for classes or inline styles.

## Basic Syntax

```html
<r-s as="div">Content</r-s>
```

The `as` attribute determines the component type to render.

### Component Naming Rules

- **Only lowercase letters** `a-z` and hyphen `-` allowed
- **Maximum 2 hyphens** recommended for readability
- **No underscores, numbers, or capitals**

✅ Valid names:
- `card`
- `button-primary`
- `hero-section`

❌ Invalid names:
- `Card` (capitals not allowed)
- `button_primary` (underscores not allowed)
- `button-1` (numbers not allowed)
- `my-super-long-component-name` (too many hyphens)

## Attributes

### `as` - Element Type

Determines what to render:

```html
<!-- HTML elements -->
<r-s as="div">Division</r-s>
<r-s as="section">Section</r-s>
<r-s as="article">Article</r-s>
<r-s as="button">Button</r-s>
<r-s as="a" href="/link">Link</r-s>

<!-- Preset components -->
<r-s as="card">Card component</r-s>
<r-s as="hero">Hero section</r-s>
<r-s as="container">Centered container</r-s>
```

### Namespace Attributes

Apply styling through namespace attributes:

```html
<r-s as="div"
      layout="[grid:3, gap:4]"
      box="[padding:6, margin:2]"
      face="[bg:brand-a, radius:lg]"
      text="[size:large, weight:bold]"
      fx="[hover:lift, transition:smooth]"
      device="[cursor:pointer]">
  Styled content
</r-s>
```

### Responsive Variants

Add breakpoint suffixes for responsive behavior:

```html
<r-s as="div"
      layout="[flex:column]"
      layout-tablet="[flex:row, gap:4]"
      layout-screen="[grid:3, gap:6]">
  Responsive layout
</r-s>
```

## Preset Components

### Built-in Presets

ReedSTYLE includes common UI patterns:

```html
<!-- Layout -->
<r-s as="container">Max-width centered container</r-s>
<r-s as="section">Full-width section with padding</r-s>
<r-s as="hero">Full-height hero section</r-s>

<!-- Components -->
<r-s as="card">Styled card with shadow and radius</r-s>
<r-s as="card-header">Card header section</r-s>
<r-s as="card-body">Card content area</r-s>
<r-s as="card-footer">Card footer section</r-s>

<!-- Buttons -->
<r-s as="button-primary">Primary action</r-s>
<r-s as="button-secondary">Secondary action</r-s>
<r-s as="button-ghost">Ghost button</r-s>
<r-s as="button-group">Button container</r-s>

<!-- Navigation -->
<r-s as="nav">Navigation bar</r-s>
<r-s as="nav-brand">Brand/logo area</r-s>
<r-s as="nav-links">Navigation links container</r-s>

<!-- Forms -->
<r-s as="form">Styled form container</r-s>
<r-s as="field">Form field wrapper</r-s>
<r-s as="field-group">Grouped fields</r-s>

<!-- Modals -->
<r-s as="modal">Modal container</r-s>
<r-s as="modal-header">Modal header</r-s>
<r-s as="modal-body">Modal content</r-s>
<r-s as="modal-footer">Modal actions</r-s>
```

### Customizing Presets

Override preset styles with namespace attributes:

```html
<!-- Start with card preset, customize background -->
<r-s as="card" face="bg:brand-a">
  Branded card
</r-s>

<!-- Button with custom size -->
<r-s as="button-primary" text="size:small">
  Small button
</r-s>
```

## User-Defined Components

Create custom components via YAML:

```yaml
# reedstyle.components.yaml
components:
  product-card:
    element: article
    layout: "[flex:column]"
    box: "[padding:6]"
    face: "[bg:base-0, radius:xl, shadow:lg]"
    
  testimonial:
    element: blockquote
    box: "[padding:8, margin:4]"
    face: "[bg:brand-a-weak, border-left:4:brand-a]"
    text: "[size:large, style:italic]"
```

Use them like built-in presets:

```html
<r-s as="product-card">
  <h3>Product Name</h3>
  <p>Description</p>
  <r-s as="button-primary">Buy Now</r-s>
</r-s>

<r-s as="testimonial">
  "This framework changed how I build websites!"
</r-s>
```

## How It Works

### CSS Selection

ReedSTYLE CSS targets the reed element directly:

```css
/* Selects <r-s as="card"> */
reed[as="card"] {
  /* card styles */
}

/* Selects <r-s layout="grid:3"> */
reed[layout*="grid:3"] {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
}
```

### JavaScript Enhancement

The optional JavaScript:
1. Processes the `as` attribute
2. Loads user components from YAML
3. Applies dynamic effects (fx namespace)
4. Manages state and interactions

### No Conversion Needed

Unlike the old `data-r-*` system, reed attributes are used directly by CSS:
- More efficient (35% smaller CSS)
- Faster rendering (no JavaScript conversion)
- Works without JavaScript for core styling

## Array Syntax

Combine multiple properties efficiently:

```html
<!-- Single property -->
<r-s as="div" box="padding:4">

<!-- Multiple properties in array -->
<r-s as="div" box="[padding:4, margin:2, width:full]">

<!-- Nested values -->
<r-s as="div" face="[bg:brand-a, border:2:solid:brand-b]">
```

## Property Values

### Numeric Scale
```html
<!-- Spacing: 0, 1, 2, 3, 4, 6, 8, 10, 12, 16, 20, 24 -->
<r-s as="div" box="padding:4">  <!-- 1rem -->
<r-s as="div" box="margin:8">   <!-- 2rem -->
```

### Keywords
```html
<r-s as="div" box="width:full">      <!-- 100% -->
<r-s as="div" layout="position:fixed"> <!-- position: fixed -->
<r-s as="div" text="align:center">   <!-- text-align: center -->
```

### Colors
```html
<r-s as="div" face="bg:brand-a">     <!-- Brand color A -->
<r-s as="div" text="color:base-900"> <!-- Dark neutral -->
<r-s as="div" face="border:state-error"> <!-- Error color -->
```

## Best Practices

### 1. Start with Presets
```html
<!-- Good: Use preset when available -->
<r-s as="card">Content</r-s>

<!-- Avoid: Recreating presets -->
<r-s as="div" box="[padding:6]" face="[bg:base-0, radius:lg, shadow:md]">
```

### 2. Mobile-First
```html
<!-- Good: Base styles first, then breakpoints -->
<r-s as="div" 
      layout="[flex:column]"
      layout-tablet="[flex:row]">

<!-- Avoid: Desktop-first -->
<r-s as="div" 
      layout="[grid:4]"
      layout-tablet="[flex:column]">
```

### 3. Semantic HTML
```html
<!-- Good: Semantic element -->
<r-s as="article">Article content</r-s>

<!-- Avoid: Generic div for everything -->
<r-s as="div">Article content</r-s>
```

### 4. Minimal Attributes
```html
<!-- Good: Only what's needed -->
<r-s as="hero">
  <h1>Title</h1>
</r-s>

<!-- Avoid: Redundant styling -->
<r-s as="hero" text="[size:huge, weight:bold]">
  <h1>Title</h1>  <!-- h1 already has these styles -->
</r-s>
```

## Browser Compatibility

The reed element works in all modern browsers:
- Uses Web Components API where available
- Falls back to custom element polyfill
- CSS works regardless of JavaScript support

## Next Steps

- [Presets](031-presets.md) - All built-in components
- [Namespaces](101-namespaces-overview.md) - Property system
- [Custom Components](301-custom-components.md) - Create your own