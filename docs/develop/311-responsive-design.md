# Responsive Design

Build adaptive layouts with ReedSTYLE's mobile-first responsive system.

## Breakpoint System

### Default Breakpoints

| Name | Min-Width | Target Devices |
|------|-----------|----------------|
| (base) | 0px | Mobile phones (default) |
| tablet | 560px | Tablets & large phones |
| screen | 960px | Desktop screens |

### Mobile-First Approach

Start with mobile styles, add complexity for larger screens:

```html
<!-- Mobile: Single column -->
<!-- Tablet: 2 columns -->
<!-- Desktop: 3 columns -->
<r-s as="div"
      layout="[flex:column, gap:4]"
      layout-tablet="[grid:2, gap:6]"
      layout-screen="[grid:3, gap:8]">
  <!-- Content -->
</r-s>
```

## Responsive Syntax

### Basic Pattern

```html
<r-s as="element" 
      [namespace]="mobile styles"
      [namespace]-[breakpoint]="larger screen styles">
```

### All Namespaces Support Breakpoints

```html
<r-s as="section"
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
</r-s>
```

## Common Responsive Patterns

### Responsive Navigation

```html
<r-s as="nav"
      layout="[flex:column, gap:2]"
      layout-screen="[flex:row, justify:between, align:center]">
  
  <!-- Brand -->
  <r-s as="nav-brand">Logo</r-s>
  
  <!-- Mobile menu button -->
  <r-s as="button"
        box="display:block"
        box-screen="display:none">
    ☰ Menu
  </r-s>
  
  <!-- Navigation links -->
  <r-s as="nav-links"
        box="display:none"
        box-screen="display:flex"
        layout-screen="[flex:row, gap:4]">
    <a href="/">Home</a>
    <a href="/about">About</a>
    <a href="/contact">Contact</a>
  </r-s>
</r-s>
```

### Responsive Grid

```html
<!-- Product grid -->
<r-s as="div"
      layout="[grid:1, gap:4]"
      layout-tablet="[grid:2, gap:6]"
      layout-screen="[grid:4, gap:8]">
  
  <r-s as="product-card">Product 1</r-s>
  <r-s as="product-card">Product 2</r-s>
  <r-s as="product-card">Product 3</r-s>
  <!-- More products -->
</r-s>
```

### Responsive Typography

```html
<r-s as="h1"
      text="[size:large, weight:bold]"
      text-tablet="[size:huge]"
      text-screen="[size:mega]">
  Responsive Heading
</r-s>

<r-s as="p"
      text="[size:small, leading:normal]"
      text-tablet="[size:normal, leading:relaxed]"
      text-screen="[size:large, leading:loose]">
  Body text that scales with viewport
</r-s>
```

### Responsive Spacing

```html
<r-s as="section"
      box="[padding:4, margin-y:4]"
      box-tablet="[padding:6, margin-y:6]"
      box-screen="[padding:8, margin-y:8]">
  Content with responsive spacing
</r-s>
```

### Hide/Show Elements

```html
<!-- Mobile only -->
<r-s as="div"
      box="display:block"
      box-tablet="display:none">
  Mobile menu
</r-s>

<!-- Desktop only -->
<r-s as="aside"
      box="display:none"
      box-screen="display:block">
  Desktop sidebar
</r-s>

<!-- Tablet and up -->
<r-s as="div"
      box="display:none"
      box-tablet="display:block">
  Tablet+ content
</r-s>
```

## Layout Strategies

### Container Queries (Future)

```html
<!-- When container queries are supported -->
<r-s as="card"
      container="inline-size"
      layout="[flex:column]"
      layout-container="[flex:row]">
  Responds to container, not viewport
</r-s>
```

### Fluid Typography

```html
<r-s as="h1"
      text="[size:clamp(1.5rem,4vw,3rem)]">
  Fluid heading
</r-s>
```

### Responsive Images

```html
<r-s as="figure"
      box="[width:full, max-width:600]">
  <img srcset="small.jpg 320w,
               medium.jpg 768w,
               large.jpg 1200w"
       sizes="(max-width: 320px) 320px,
              (max-width: 768px) 768px,
              1200px"
       alt="Responsive image">
</r-s>
```

## Advanced Responsive Techniques

### Future: Orientation-Based Styles

```html
<!-- Planned for future release -->
<r-s as="div"
      layout="[grid:2]"
      layout-landscape="[grid:4]"
      layout-portrait="[flex:column]">
  Orientation-aware layout
</r-s>
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
<r-s as="div"
      layout="[flex:column]"
      layout-tablet="[flex:row, wrap:wrap]"
      layout-screen="[flex:row, wrap:nowrap]">
  
  <!-- Responsive flex children -->
  <r-s as="div"
        layout="[basis:full]"
        layout-tablet="[basis:half]"
        layout-screen="[basis:third]">
    Responsive flex item
  </r-s>
</r-s>
```

## Testing Responsive Designs

### Browser DevTools

```javascript
// Test breakpoints programmatically
const breakpoints = {
  tablet: 560,
  screen: 960
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
<r-s as="div"
      layout="contain:layout"
      layout-screen="contain:none">
  Optimized for mobile rendering
</r-s>
```

## Responsive Design Patterns

### The Hamburger Menu

```html
<r-s as="header">
  <r-s as="nav"
        layout="[flex:row, justify:between, align:center]">
    
    <!-- Logo -->
    <r-s as="logo">Brand</r-s>
    
    <!-- Mobile menu toggle -->
    <r-s as="button"
          id="menu-toggle"
          box="display:block"
          box-screen="display:none"
          face="[bg:transparent, border:none]"
          text="size:large">
      ☰
    </r-s>
    
    <!-- Menu items -->
    <r-s as="nav-menu"
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
    </r-s>
  </r-s>
</r-s>
```

### Card to List Transform

```html
<!-- Cards on desktop, list on mobile -->
<r-s as="div"
      layout="[flex:column, gap:2]"
      layout-screen="[grid:3, gap:6]">
  
  <r-s as="item"
        layout="[flex:row, gap:3, padding:3]"
        layout-screen="[flex:column, padding:6]"
        face="[border-bottom:1:base-200]"
        face-screen="[border:none, shadow:md, radius:lg]">
    
    <r-s as="thumbnail"
          box="[width:20, height:20]"
          box-screen="[width:full, height:40]">
      <img src="thumb.jpg" alt="">
    </r-s>
    
    <r-s as="content"
          layout="[flex:column, justify:center]">
      <h3>Item Title</h3>
      <p>Description</p>
    </r-s>
  </r-s>
</r-s>
```

### Responsive Tables

```html
<!-- Table on desktop, cards on mobile -->
<r-s as="div"
      box="overflow-x:auto">
  
  <!-- Desktop table -->
  <r-s as="table"
        box="display:none"
        box-tablet="display:table">
    <table>
      <!-- Table content -->
    </table>
  </r-s>
  
  <!-- Mobile cards -->
  <r-s as="div"
        box="display:block"
        box-tablet="display:none"
        layout="[flex:column, gap:4]">
    
    <r-s as="card">
      <!-- Card representation of table row -->
    </r-s>
  </r-s>
</r-s>
```

## Best Practices

1. **Mobile-first always** - Start simple, add complexity
2. **Test real devices** - Emulators aren't enough
3. **Fluid over fixed** - Use relative units
4. **Touch-friendly** - Min 44x44px tap targets
5. **Performance matters** - Optimize for mobile networks
6. **Accessible** - Responsive shouldn't break accessibility

## Next: [JavaScript API](321-javascript-api.md)