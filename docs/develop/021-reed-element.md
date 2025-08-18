# The Reed Element

## Overview

The `<reed>` element is the heart of ReedSTYLE. It's a custom HTML element that accepts styling attributes directly, eliminating the need for classes or inline styles.

## Basic Syntax

```html
<reed as="div">Content</reed>
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
<reed as="div">Division</reed>
<reed as="section">Section</reed>
<reed as="article">Article</reed>
<reed as="button">Button</reed>
<reed as="a" href="/link">Link</reed>

<!-- Preset components -->
<reed as="card">Card component</reed>
<reed as="hero">Hero section</reed>
<reed as="container">Centered container</reed>
```

### Namespace Attributes

Apply styling through namespace attributes:

```html
<reed as="div"
      layout="[grid:3, gap:4]"
      box="[padding:6, margin:2]"
      face="[bg:brand-a, radius:lg]"
      text="[size:large, weight:bold]"
      fx="[hover:lift, transition:smooth]"
      device="[cursor:pointer]">
  Styled content
</reed>
```

### Responsive Variants

Add breakpoint suffixes for responsive behavior:

```html
<reed as="div"
      layout="[flex:column]"
      layout-phone="[flex:column, gap:2]"
      layout-tablet="[flex:row, gap:4]"
      layout-screen="[grid:3, gap:6]"
      layout-wide="[grid:4, gap:8]">
  Responsive layout
</reed>
```

## Preset Components

### Built-in Presets

ReedSTYLE includes common UI patterns:

```html
<!-- Layout -->
<reed as="container">Max-width centered container</reed>
<reed as="section">Full-width section with padding</reed>
<reed as="hero">Full-height hero section</reed>

<!-- Components -->
<reed as="card">Styled card with shadow and radius</reed>
<reed as="card-header">Card header section</reed>
<reed as="card-body">Card content area</reed>
<reed as="card-footer">Card footer section</reed>

<!-- Buttons -->
<reed as="button-primary">Primary action</reed>
<reed as="button-secondary">Secondary action</reed>
<reed as="button-ghost">Ghost button</reed>
<reed as="button-group">Button container</reed>

<!-- Navigation -->
<reed as="nav">Navigation bar</reed>
<reed as="nav-brand">Brand/logo area</reed>
<reed as="nav-links">Navigation links container</reed>

<!-- Forms -->
<reed as="form">Styled form container</reed>
<reed as="field">Form field wrapper</reed>
<reed as="field-group">Grouped fields</reed>

<!-- Modals -->
<reed as="modal">Modal container</reed>
<reed as="modal-header">Modal header</reed>
<reed as="modal-body">Modal content</reed>
<reed as="modal-footer">Modal actions</reed>
```

### Customizing Presets

Override preset styles with namespace attributes:

```html
<!-- Start with card preset, customize background -->
<reed as="card" face="bg:brand-a">
  Branded card
</reed>

<!-- Button with custom size -->
<reed as="button-primary" text="size:small">
  Small button
</reed>
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
<reed as="product-card">
  <h3>Product Name</h3>
  <p>Description</p>
  <reed as="button-primary">Buy Now</reed>
</reed>

<reed as="testimonial">
  "This framework changed how I build websites!"
</reed>
```

## How It Works

### CSS Selection

ReedSTYLE CSS targets the reed element directly:

```css
/* Selects <reed as="card"> */
reed[as="card"] {
  /* card styles */
}

/* Selects <reed layout="grid:3"> */
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
<reed as="div" box="padding:4">

<!-- Multiple properties in array -->
<reed as="div" box="[padding:4, margin:2, width:full]">

<!-- Nested values -->
<reed as="div" face="[bg:brand-a, border:2:solid:brand-b]">
```

## Property Values

### Numeric Scale
```html
<!-- Spacing: 0, 1, 2, 3, 4, 6, 8, 10, 12, 16, 20, 24 -->
<reed as="div" box="padding:4">  <!-- 1rem -->
<reed as="div" box="margin:8">   <!-- 2rem -->
```

### Keywords
```html
<reed as="div" box="width:full">      <!-- 100% -->
<reed as="div" layout="position:fixed"> <!-- position: fixed -->
<reed as="div" text="align:center">   <!-- text-align: center -->
```

### Colors
```html
<reed as="div" face="bg:brand-a">     <!-- Brand color A -->
<reed as="div" text="color:base-900"> <!-- Dark neutral -->
<reed as="div" face="border:state-error"> <!-- Error color -->
```

## Best Practices

### 1. Start with Presets
```html
<!-- Good: Use preset when available -->
<reed as="card">Content</reed>

<!-- Avoid: Recreating presets -->
<reed as="div" box="[padding:6]" face="[bg:base-0, radius:lg, shadow:md]">
```

### 2. Mobile-First
```html
<!-- Good: Base styles first, then breakpoints -->
<reed as="div" 
      layout="[flex:column]"
      layout-tablet="[flex:row]">

<!-- Avoid: Desktop-first -->
<reed as="div" 
      layout="[grid:4]"
      layout-phone="[flex:column]">
```

### 3. Semantic HTML
```html
<!-- Good: Semantic element -->
<reed as="article">Article content</reed>

<!-- Avoid: Generic div for everything -->
<reed as="div">Article content</reed>
```

### 4. Minimal Attributes
```html
<!-- Good: Only what's needed -->
<reed as="hero">
  <h1>Title</h1>
</reed>

<!-- Avoid: Redundant styling -->
<reed as="hero" text="[size:huge, weight:bold]">
  <h1>Title</h1>  <!-- h1 already has these styles -->
</reed>
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