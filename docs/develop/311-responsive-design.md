# Responsive Design

Build adaptive layouts with ReedSTYLE's mobile-first responsive system.

## Breakpoint System

### Default Breakpoints

| Name | Min-Width | Target Devices |
|------|-----------|----------------|
| (base) | 0px | Mobile phones (default) |
| phone | 320px | Small phones |
| tablet | 560px | Tablets & large phones |
| screen | 960px | Desktop screens |
| wide | 1260px | Wide screens |

### Mobile-First Approach

Start with mobile styles, add complexity for larger screens:

```html
<!-- Mobile: Single column -->
<!-- Tablet: 2 columns -->
<!-- Desktop: 3 columns -->
<!-- Wide: 4 columns -->
<reed as="div"
      layout="[flex:column, gap:4]"
      layout-tablet="[grid:2, gap:6]"
      layout-screen="[grid:3, gap:8]"
      layout-wide="[grid:4, gap:10]">
  <!-- Content -->
</reed>
```

## Responsive Syntax

### Basic Pattern

```html
<reed as="element" 
      [namespace]="mobile styles"
      [namespace]-[breakpoint]="larger screen styles"
      [namespace]-[environment]="environment styles"
      [namespace]-[environment]-[breakpoint]="combined styles">
```

### All Namespaces Support Breakpoints

```html
<reed as="section"
      box="[padding:4]"
      box-tablet="[padding:6]"
      box-screen="[padding:8]"
      
      text="[size:small, align:center]"
      text-tablet="[size:normal]"
      text-screen="[size:large, align:left]"
      
      face="[bg:base-100]"
      face-screen="[bg:gradient-primary]"
      
      layout="[flex:column]"
      layout-tablet="[flex:row]"
      layout-screen="[grid:3]">
  Fully responsive element
</reed>
```

## Common Responsive Patterns

### Responsive Navigation

```html
<reed as="nav"
      layout="[flex:column, gap:2]"
      layout-screen="[flex:row, justify:between, align:center]">
  
  <!-- Brand -->
  <reed as="nav-brand">Logo</reed>
  
  <!-- Mobile menu button -->
  <reed as="button"
        box="display:block"
        box-screen="display:none">
    ☰ Menu
  </reed>
  
  <!-- Navigation links -->
  <reed as="nav-links"
        box="display:none"
        box-screen="display:flex"
        layout-screen="[flex:row, gap:4]">
    <a href="/">Home</a>
    <a href="/about">About</a>
    <a href="/contact">Contact</a>
  </reed>
</reed>
```

### Responsive Grid

```html
<!-- Product grid -->
<reed as="div"
      layout="[grid:1, gap:4]"
      layout-phone="[grid:2, gap:4]"
      layout-tablet="[grid:3, gap:6]"
      layout-screen="[grid:4, gap:8]"
      layout-wide="[grid:6, gap:8]">
  
  <reed as="product-card">Product 1</reed>
  <reed as="product-card">Product 2</reed>
  <reed as="product-card">Product 3</reed>
  <!-- More products -->
</reed>
```

### Responsive Typography

```html
<reed as="h1"
      text="[size:large, weight:bold]"
      text-tablet="[size:huge]"
      text-screen="[size:mega]">
  Responsive Heading
</reed>

<reed as="p"
      text="[size:small, leading:normal]"
      text-tablet="[size:normal, leading:relaxed]"
      text-screen="[size:large, leading:loose]">
  Body text that scales with viewport
</reed>
```

### Responsive Spacing

```html
<reed as="section"
      box="[padding:4, margin-y:4]"
      box-tablet="[padding:6, margin-y:6]"
      box-screen="[padding:8, margin-y:8]"
      box-wide="[padding:12, margin-y:12]">
  Content with responsive spacing
</reed>
```

### Hide/Show Elements

```html
<!-- Mobile only -->
<reed as="div"
      box="display:block"
      box-tablet="display:none">
  Mobile menu
</reed>

<!-- Desktop only -->
<reed as="aside"
      box="display:none"
      box-screen="display:block">
  Desktop sidebar
</reed>

<!-- Tablet and up -->
<reed as="div"
      box="display:none"
      box-tablet="display:block">
  Tablet+ content
</reed>
```

## Layout Strategies

### Container Queries (Future)

```html
<!-- When container queries are supported -->
<reed as="card"
      container="inline-size"
      layout="[flex:column]"
      layout-container="[flex:row]">
  Responds to container, not viewport
</reed>
```

### Fluid Typography

```html
<reed as="h1"
      text="[size:clamp(1.5rem,4vw,3rem)]">
  Fluid heading
</reed>
```

### Responsive Images

```html
<reed as="figure"
      box="[width:full, max-width:600]">
  <img srcset="small.jpg 320w,
               medium.jpg 768w,
               large.jpg 1200w"
       sizes="(max-width: 320px) 320px,
              (max-width: 768px) 768px,
              1200px"
       alt="Responsive image">
</reed>
```

## Advanced Responsive Techniques

### Orientation-Based Styles

```html
<reed as="div"
      layout="[grid:2]"
      layout-landscape="[grid:4]"
      layout-portrait="[flex:column]">
  Orientation-aware layout
</reed>
```

### Responsive Components

```yaml
# reedstyle.components.yaml
components:
  responsive-hero:
    element: section
    # Mobile
    box: "[height:screen, padding:4]"
    layout: "[flex:column, justify:center]"
    text: "[align:center]"
    # Tablet
    box-tablet: "[padding:6]"
    # Desktop
    box-screen: "[padding:8]"
    layout-screen: "[grid:2, align:center]"
    text-screen: "[align:left]"
```

### Responsive Utilities

```html
<!-- Responsive flex utilities -->
<reed as="div"
      layout="[flex:column]"
      layout-tablet="[flex:row, wrap:wrap]"
      layout-screen="[flex:row, wrap:nowrap]">
  
  <!-- Responsive flex children -->
  <reed as="div"
        layout="[basis:full]"
        layout-tablet="[basis:half]"
        layout-screen="[basis:third]">
    Responsive flex item
  </reed>
</reed>
```

## Testing Responsive Designs

### Browser DevTools

```javascript
// Test breakpoints programmatically
const breakpoints = {
  phone: 320,
  tablet: 560,
  screen: 960,
  wide: 1260
};

Object.entries(breakpoints).forEach(([name, width]) => {
  window.resizeTo(width, 800);
  console.log(`Testing ${name} breakpoint at ${width}px`);
});
```

### Viewport Meta Tag

Always include for proper mobile rendering:

```html
<meta name="viewport" content="width=device-width, initial-scale=1.0">
```

### Testing Checklist

- [ ] Test all breakpoints
- [ ] Check landscape/portrait
- [ ] Verify touch targets (min 44x44px)
- [ ] Test on real devices
- [ ] Check font scaling
- [ ] Verify image loading
- [ ] Test with slow connection

## Performance Considerations

### Responsive Images

```html
<!-- Use modern formats with fallbacks -->
<picture>
  <source type="image/webp" 
          srcset="image.webp"
          media="(min-width: 960px)">
  <source type="image/jpeg" 
          srcset="image-mobile.jpg"
          media="(max-width: 559px)">
  <img src="image.jpg" alt="Fallback">
</picture>
```

### Conditional Loading

```javascript
// Load components based on viewport
if (window.matchMedia('(min-width: 960px)').matches) {
  // Load desktop-only features
  import('./desktop-features.js');
}
```

### CSS Containment

```html
<reed as="div"
      layout="contain:layout"
      layout-screen="contain:none">
  Optimized for mobile rendering
</reed>
```

## Responsive Design Patterns

### The Hamburger Menu

```html
<reed as="header">
  <reed as="nav"
        layout="[flex:row, justify:between, align:center]">
    
    <!-- Logo -->
    <reed as="logo">Brand</reed>
    
    <!-- Mobile menu toggle -->
    <reed as="button"
          id="menu-toggle"
          box="display:block"
          box-screen="display:none"
          face="[bg:transparent, border:none]"
          text="size:large">
      ☰
    </reed>
    
    <!-- Menu items -->
    <reed as="nav-menu"
          id="menu"
          box="[display:none, position:absolute, top:full, left:0, width:full]"
          box-screen="[display:flex, position:static, width:auto]"
          layout-screen="[flex:row, gap:4]"
          face="[bg:base-0]"
          face-screen="[bg:transparent]">
      <a href="/">Home</a>
      <a href="/about">About</a>
      <a href="/services">Services</a>
      <a href="/contact">Contact</a>
    </reed>
  </reed>
</reed>
```

### Card to List Transform

```html
<!-- Cards on desktop, list on mobile -->
<reed as="div"
      layout="[flex:column, gap:2]"
      layout-screen="[grid:3, gap:6]">
  
  <reed as="item"
        layout="[flex:row, gap:3, padding:3]"
        layout-screen="[flex:column, padding:6]"
        face="[border-bottom:1:base-200]"
        face-screen="[border:none, shadow:md, radius:lg]">
    
    <reed as="thumbnail"
          box="[width:20, height:20]"
          box-screen="[width:full, height:40]">
      <img src="thumb.jpg" alt="">
    </reed>
    
    <reed as="content"
          layout="[flex:column, justify:center]">
      <h3>Item Title</h3>
      <p>Description</p>
    </reed>
  </reed>
</reed>
```

### Responsive Tables

```html
<!-- Table on desktop, cards on mobile -->
<reed as="div"
      box="overflow-x:auto">
  
  <!-- Desktop table -->
  <reed as="table"
        box="display:none"
        box-tablet="display:table">
    <table>
      <!-- Table content -->
    </table>
  </reed>
  
  <!-- Mobile cards -->
  <reed as="div"
        box="display:block"
        box-tablet="display:none"
        layout="[flex:column, gap:4]">
    
    <reed as="card">
      <!-- Card representation of table row -->
    </reed>
  </reed>
</reed>
```

## Best Practices

1. **Mobile-first always** - Start simple, add complexity
2. **Test real devices** - Emulators aren't enough
3. **Fluid over fixed** - Use relative units
4. **Touch-friendly** - Min 44x44px tap targets
5. **Performance matters** - Optimize for mobile networks
6. **Accessible** - Responsive shouldn't break accessibility

## Next: [JavaScript API](321-javascript-api.md)