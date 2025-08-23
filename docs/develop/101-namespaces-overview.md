# Namespaces Overview

ReedSTYLE organizes all CSS properties into 6 logical namespaces, each controlling a specific aspect of styling.

## The 6 Core Namespaces

| Namespace | Purpose | Common Properties |
|-----------|---------|-------------------|
| **[box](102-box.md)** | Size & spacing | width, height, padding, margin, overflow |
| **[device](103-device.md)** | User interaction | cursor, pointer-events, user-select, touch |
| **[face](104-face.md)** | Visual appearance | background, border, shadow, opacity, radius |
| **[fx](105-fx.md)** | Effects & animations | transform, transition, filter, animation |
| **[layout](106-layout.md)** | Spatial arrangement | flex, grid, position, z-index, gap |
| **[text](107-text.md)** | Typography | font, size, weight, color, align, line-height |

## Using Namespaces

### Basic Syntax

```html
<r-s as="div" [namespace]="[property:value]">
```

### Single Property

```html
<r-s as="div" box="padding:4">
<r-s as="div" text="align:center">
<r-s as="div" face="bg:brand-a">
```

### Multiple Properties (Array Syntax)

```html
<r-s as="div" box="[padding:4, margin:2, width:full]">
<r-s as="div" layout="[grid:3, gap:4, align:center]">
<r-s as="div" face="[bg:brand-a, radius:lg, shadow:md]">
```

## Property Value Types

### 1. Numeric Scale

Used for spacing, sizes, and dimensions:

```
0  = 0
1  = 0.25rem
2  = 0.5rem
3  = 0.75rem
4  = 1rem
6  = 1.5rem
8  = 2rem
10 = 2.5rem
12 = 3rem
16 = 4rem
20 = 5rem
24 = 6rem
```

Example:
```html
<r-s as="div" box="[padding:4, margin:8]">
<!-- padding: 1rem, margin: 2rem -->
```

### 2. Keywords

Semantic values that map to CSS:

```html
<r-s as="div" box="width:full">        <!-- width: 100% -->
<r-s as="div" box="width:half">        <!-- width: 50% -->
<r-s as="div" layout="position:fixed"> <!-- position: fixed -->
<r-s as="div" text="weight:bold">      <!-- font-weight: 700 -->
```

### 3. Colors (1-9 Scale)

All colors automatically generate a 1-9 scale. Use the base name for default, or specify scale:

```html
<!-- Brand colors - base name uses middle value (5) -->
<r-s as="div" face="bg:brand-a">       <!-- Same as brand-a-5 -->
<r-s as="div" face="bg:brand-b">       <!-- Same as brand-b-5 -->

<!-- Or use specific scale values (1=light, 9=dark) -->
<r-s as="div" face="bg:brand-a-2">     <!-- Light variant -->
<r-s as="div" face="bg:brand-a-7">     <!-- Dark variant -->

<!-- Neutral colors -->
<r-s as="div" face="bg:neutral-1">     <!-- White -->
<r-s as="div" face="bg:neutral-5">     <!-- Medium gray -->
<r-s as="div" face="bg:neutral-9">     <!-- Black -->

<!-- State colors (same pattern) -->
<r-s as="div" face="bg:success">       <!-- Same as success-5 -->
<r-s as="div" face="bg:warning-2">     <!-- Light warning -->
<r-s as="div" face="bg:error-8">       <!-- Dark error -->
```

### 4. Complex Values

Multiple values separated by colons:

```html
<!-- border: width : style : color -->
<r-s as="div" face="border:2:solid:brand-a">

<!-- transform: function : value -->
<r-s as="div" fx="transform:rotate:45deg">

<!-- shadow: size : color -->
<r-s as="div" face="shadow:lg:brand-a">
```

## Responsive System

### Breakpoints

Every namespace supports responsive variants:

| Suffix | Min-Width | Use Case |
|--------|-----------|----------|
| (none) | 0px | Mobile-first base |
| `-tablet` | 560px | Tablets & large phones |
| `-screen` | 960px | Desktop screens |

### Responsive Usage

```html
<!-- Different layouts at each breakpoint -->
<r-s as="div" 
      layout="[flex:column]"
      layout-tablet="[flex:row, gap:4]"
      layout-screen="[grid:3, gap:6]">
```

### Mobile-First Approach

Start with mobile styles, add larger breakpoints:

```html
<!-- Base (mobile) -->
<r-s as="div" 
      box="[padding:2]"
      text="[size:small]">
  
  <!-- Tablet and up -->
  <r-s as="div"
        box-tablet="[padding:4]"
        text-tablet="[size:medium]">
    
    <!-- Desktop and up -->
    <r-s as="div"
          box-screen="[padding:6]"
          text-screen="[size:large]">
```

## Namespace Interactions

### Complementary Usage

Namespaces work together to create complete designs:

```html
<r-s as="card"
      layout="[flex:column]"        <!-- Structure -->
      box="[padding:6, margin:4]"   <!-- Spacing -->
      face="[bg:base-0, radius:lg, shadow:md]" <!-- Appearance -->
      text="[align:center]"          <!-- Typography -->
      fx="[hover:lift]"              <!-- Effects -->
      device="[cursor:pointer]">     <!-- Interaction -->
  Complete component styling
</r-s>
```

### Priority and Cascading

1. **Preset styles** are applied first
2. **Namespace attributes** override presets
3. **Responsive variants** override base styles
4. **Later properties** in arrays override earlier ones

```html
<!-- Preset provides defaults -->
<r-s as="card" 
      face="bg:brand-a">  <!-- Overrides preset background -->
```

## Best Practices

### 1. Use the Right Namespace

Each namespace has a specific purpose:

```html
<!-- Good: Using appropriate namespaces -->
<r-s as="div" 
      layout="[grid:3]"      <!-- Grid is layout -->
      box="[padding:4]"      <!-- Padding is box -->
      face="[bg:brand-a]">   <!-- Background is face -->

<!-- Avoid: Wrong namespace for property -->
<!-- (These would not work) -->
<r-s as="div" 
      box="[grid:3]"         <!-- Grid isn't in box -->
      layout="[padding:4]">  <!-- Padding isn't in layout -->
```

### 2. Combine Properties Efficiently

Use arrays to group related properties:

```html
<!-- Good: Combined in array -->
<r-s as="div" box="[padding:4, margin:2, width:full]">

<!-- Less efficient: Separate attributes -->
<r-s as="div" 
      box="padding:4"
      box-margin="2"
      box-width="full">
```

### 3. Mobile-First Responsive

```html
<!-- Good: Start small, add breakpoints -->
<r-s as="div" 
      layout="[flex:column]"
      layout-tablet="[flex:row]">

<!-- Avoid: Starting with desktop -->
<r-s as="div" 
      layout="[flex:row]"
      layout-phone="[flex:column]">
```

### 4. Semantic Over Visual

Choose properties based on meaning, not just appearance:

```html
<!-- Good: Semantic color -->
<r-s as="alert" face="bg:state-error">

<!-- Avoid: Visual color -->
<r-s as="alert" face="bg:red">
```

## Common Patterns

### Centered Container

```html
<r-s as="div" 
      box="[width:full, max-width:1200, margin-x:auto, padding-x:4]">
```

### Responsive Grid

```html
<r-s as="div" 
      layout="[flex:column]"
      layout-tablet="[grid:2, gap:4]"
      layout-screen="[grid:3, gap:6]">
```

### Interactive Card

```html
<r-s as="div" 
      box="[padding:6]"
      face="[bg:base-0, radius:lg, shadow:md]"
      fx="[hover:lift, transition:smooth]"
      device="[cursor:pointer]">
```

## Namespace Reference

Detailed documentation for each namespace:

- **[Box Namespace](102-box.md)** - Dimensions and spacing
- **[Device Namespace](103-device.md)** - User interactions
- **[Face Namespace](104-face.md)** - Visual styling
- **[FX Namespace](105-fx.md)** - Effects and animations
- **[Layout Namespace](106-layout.md)** - Positioning and arrangement
- **[Text Namespace](107-text.md)** - Typography control

## Next Steps

- Explore individual [namespace documentation](102-box.md)
- Learn about [responsive design](311-responsive-design.md)
- See [examples](https://reedstyle.dev/examples) in action