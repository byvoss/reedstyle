# Face Namespace

Controls visual appearance: backgrounds, borders, shadows, opacity, and other decorative properties.

## Properties

### Background

```html
<!-- Brand colors -->
<r-s as="div" face="bg:brand-a">     <!-- Primary brand -->
<r-s as="div" face="bg:brand-b">     <!-- Secondary brand -->
<r-s as="div" face="bg:brand-c">     <!-- Tertiary brand -->
<r-s as="div" face="bg:brand-d">
<r-s as="div" face="bg:brand-e">
<r-s as="div" face="bg:brand-f">

<!-- Base colors (neutrals) -->
<r-s as="div" face="bg:base-0">      <!-- White -->
<r-s as="div" face="bg:base-50">     <!-- Lightest gray -->
<r-s as="div" face="bg:base-100">
<r-s as="div" face="bg:base-200">
<r-s as="div" face="bg:base-300">
<r-s as="div" face="bg:base-400">
<r-s as="div" face="bg:base-500">    <!-- Medium gray -->
<r-s as="div" face="bg:base-600">
<r-s as="div" face="bg:base-700">
<r-s as="div" face="bg:base-800">
<r-s as="div" face="bg:base-900">    <!-- Darkest gray -->
<r-s as="div" face="bg:base-1000">   <!-- Black -->

<!-- Semantic colors -->
<r-s as="div" face="bg:state-success">
<r-s as="div" face="bg:state-warning">
<r-s as="div" face="bg:state-error">
<r-s as="div" face="bg:state-info">

<!-- Color variations (Visual Scope) -->
<r-s as="div" face="bg:brand-a-weak">    <!-- Sehr hell (Hintergründe) -->
<r-s as="div" face="bg:brand-a-light">   <!-- Hell (subtile Akzente) -->
<r-s as="div" face="bg:brand-a">         <!-- Normal (Standard) -->
<r-s as="div" face="bg:brand-a-intense"> <!-- Intensiver (mehr Sättigung) -->
<r-s as="div" face="bg:brand-a-bright">  <!-- Leuchtend (heller und satter) -->
<r-s as="div" face="bg:brand-a-strong">  <!-- Kräftig (dunkler und satter) -->

<!-- Gradients -->
<r-s as="div" face="bg:gradient-primary">   <!-- brand-a to brand-b -->
<r-s as="div" face="bg:gradient-secondary"> <!-- brand-b to brand-c -->
<r-s as="div" face="bg:gradient-radial">    <!-- Radial gradient -->
```

### Border

```html
<!-- Simple borders -->
<r-s as="div" face="border:1">       <!-- 1px solid base-200 -->
<r-s as="div" face="border:2">       <!-- 2px solid base-200 -->
<r-s as="div" face="border:4">       <!-- 4px solid base-200 -->

<!-- Border with color -->
<r-s as="div" face="border:2:brand-a">     <!-- 2px solid brand-a -->
<r-s as="div" face="border:1:state-error"> <!-- 1px solid error -->

<!-- Full syntax: width:style:color -->
<r-s as="div" face="border:2:solid:brand-a">
<r-s as="div" face="border:1:dashed:base-400">
<r-s as="div" face="border:3:dotted:brand-b">

<!-- Individual sides -->
<r-s as="div" face="border-top:2:brand-a">
<r-s as="div" face="border-right:1:base-200">
<r-s as="div" face="border-bottom:4:brand-b">
<r-s as="div" face="border-left:2:state-success">

<!-- Axis shortcuts -->
<r-s as="div" face="border-x:1">     <!-- left and right -->
<r-s as="div" face="border-y:2">     <!-- top and bottom -->
```

### Border Radius

```html
<!-- Preset sizes -->
<r-s as="div" face="radius:none">    <!-- 0 -->
<r-s as="div" face="radius:sm">      <!-- 0.125rem -->
<r-s as="div" face="radius:md">      <!-- 0.25rem -->
<r-s as="div" face="radius:lg">      <!-- 0.5rem -->
<r-s as="div" face="radius:xl">      <!-- 0.75rem -->
<r-s as="div" face="radius:2xl">     <!-- 1rem -->
<r-s as="div" face="radius:3xl">     <!-- 1.5rem -->
<r-s as="div" face="radius:full">    <!-- 9999px (circle/pill) -->

<!-- Individual corners -->
<r-s as="div" face="radius-tl:lg">   <!-- top-left -->
<r-s as="div" face="radius-tr:lg">   <!-- top-right -->
<r-s as="div" face="radius-bl:lg">   <!-- bottom-left -->
<r-s as="div" face="radius-br:lg">   <!-- bottom-right -->
```

### Shadow

```html
<!-- Preset shadows -->
<r-s as="div" face="shadow:none">
<r-s as="div" face="shadow:sm">      <!-- Small shadow -->
<r-s as="div" face="shadow:md">      <!-- Medium shadow -->
<r-s as="div" face="shadow:lg">      <!-- Large shadow -->
<r-s as="div" face="shadow:xl">      <!-- Extra large -->
<r-s as="div" face="shadow:2xl">     <!-- 2x large -->
<r-s as="div" face="shadow:inner">   <!-- Inset shadow -->

<!-- Colored shadows -->
<r-s as="div" face="shadow:lg:brand-a">    <!-- Large brand-a shadow -->
<r-s as="div" face="shadow:md:state-error"> <!-- Medium error shadow -->
```

### Opacity

```html
<r-s as="div" face="opacity:0">      <!-- Fully transparent -->
<r-s as="div" face="opacity:10">     <!-- 10% -->
<r-s as="div" face="opacity:25">     <!-- 25% -->
<r-s as="div" face="opacity:50">     <!-- 50% -->
<r-s as="div" face="opacity:75">     <!-- 75% -->
<r-s as="div" face="opacity:90">     <!-- 90% -->
<r-s as="div" face="opacity:100">    <!-- Fully opaque -->
```

### Outline

```html
<!-- Outline styles -->
<r-s as="div" face="outline:none">
<r-s as="div" face="outline:1:brand-a">
<r-s as="div" face="outline:2:solid:brand-b">
<r-s as="div" face="outline:2:dashed:base-400">

<!-- Outline offset -->
<r-s as="div" face="outline-offset:2">  <!-- 2px offset -->
<r-s as="div" face="outline-offset:4">  <!-- 4px offset -->
```

### Backdrop

```html
<!-- Backdrop blur -->
<r-s as="div" face="backdrop:blur-sm">
<r-s as="div" face="backdrop:blur-md">
<r-s as="div" face="backdrop:blur-lg">

<!-- Backdrop brightness -->
<r-s as="div" face="backdrop:bright-50">
<r-s as="div" face="backdrop:bright-75">
<r-s as="div" face="backdrop:bright-125">
```

## Common Patterns

### Card Design

```html
<r-s as="div" 
      face="[bg:base-0, radius:lg, shadow:md, border:1:base-100]">
  Standard card appearance
</r-s>
```

### Gradient Background

```html
<r-s as="section" 
      face="[bg:gradient-primary, opacity:90]">
  Gradient section
</r-s>
```

### Glass Morphism

```html
<r-s as="div" 
      face="[bg:base-0, opacity:80, backdrop:blur-md, border:1:base-200]">
  Frosted glass effect
</r-s>
```

### Neumorphism

```html
<r-s as="div" 
      face="[bg:base-100, radius:xl, shadow:lg, shadow:inner]">
  Soft UI style
</r-s>
```

### Focus Ring

```html
<r-s as="button" 
      face="[radius:md]"
      face-focus="[outline:2:brand-a, outline-offset:2]">
  Accessible button
</r-s>
```

### Image Overlay

```html
<r-s as="div" 
      face="[bg:base-1000, opacity:60]"
      layout="position:absolute">
  Dark overlay
</r-s>
```

## Responsive Examples

### Adaptive Shadows

```html
<r-s as="card" 
      face="shadow:sm"
      face-tablet="shadow:md"
      face-screen="shadow:lg">
  Shadow grows with viewport
</r-s>
```

### Responsive Borders

```html
<r-s as="div" 
      face="border:1"
      face-screen="[border:2, radius:lg]">
  Thicker border on desktop
</r-s>
```

### Mobile-First Background

```html
<r-s as="section" 
      face="bg:base-50"
      face-tablet="bg:base-0"
      face-screen="bg:gradient-primary">
  Progressive enhancement
</r-s>
```

## Color System

ReedSTYLE uses OKLCH internally for perceptually uniform colors. All color formats are automatically converted to OKLCH.

### Configuration

Colors are defined in `reedstyle.colors.yaml` in your project root:

```yaml
# Any format - all converted to OKLCH internally
colors:
  brand-a: "#FF6B6B"           # Hex → OKLCH
  brand-b: "rgb(78, 205, 196)" # RGB → OKLCH
  brand-c: "hsl(200, 70%, 50%)" # HSL → OKLCH
  brand-d: "rgba(255, 107, 107, 0.9)" # RGBA → OKLCH
  brand-e: "hsla(45, 100%, 70%, 0.9)" # HSLA → OKLCH
  brand-f: "oklch(70% 0.15 120)" # Already OKLCH
```

### Automatic OKLCH Conversion

The build system converts all color formats:

```
Input: #FF6B6B
Output: oklch(68.5% 0.24 25)

Input: rgb(78, 205, 196)
Output: oklch(82% 0.15 175)

Input: hsl(200, 70%, 50%)
Output: oklch(67% 0.18 230)
```

### Brand Colors (brand-a bis brand-f)
- **brand-a**: Hauptmarkenfarbe
- **brand-b**: Zweite Markenfarbe  
- **brand-c**: Dritte Markenfarbe
- **brand-d**: Vierte Markenfarbe
- **brand-e**: Fünfte Markenfarbe
- **brand-f**: Sechste Markenfarbe

### Base Colors (Neutrals)
- **base-0 bis base-1000**: Grauskala von Weiß bis Schwarz

### Semantic Colors
- **success**: Grün (Erfolg, positive Aktionen)
- **warning**: Orange (Warnungen, Vorsicht)
- **error**: Rot (Fehler, Gefahr)
- **info**: Blau (Information, Hinweise)

### Visual Scope (Farbvariationen)

Automatically generated from each brand color:

- **weak**: Sehr hell (für Hintergründe) - L+20%, C-60%
- **light**: Hell (für subtile Akzente) - L+10%, C-30%
- **(normal)**: Standardfarbe - Original
- **intense**: Intensiver (mehr Sättigung) - L-10%, C+20%
- **bright**: Leuchtend (heller und satter) - L+5%, C+40%
- **strong**: Kräftig (dunkler und satter) - L-20%, C+10%

Example:
```yaml
# Input
brand-a: "#FF6B6B"

# Generates (all in OKLCH)
brand-a-weak    # Very light background
brand-a-light   # Subtle accent
brand-a         # Normal
brand-a-intense # More saturated
brand-a-bright  # Bright and vivid
brand-a-strong  # Dark and strong
```

## Best Practices

1. **Use semantic colors** over arbitrary values
2. **Layer shadows** for depth, not just size
3. **Consistent radius** across similar components
4. **Subtle borders** (1px) for definition
5. **Test opacity** with different backgrounds
6. **Consider dark mode** when choosing colors

## Next: [FX Namespace](105-fx.md)