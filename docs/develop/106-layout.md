# Layout Namespace

Controls spatial arrangement: flexbox, grid, positioning, and z-index.

## Flexbox Properties

### Flex Container

```html
<!-- Enable flex -->
<reed as="div" layout="flex">

<!-- Flex direction -->
<reed as="div" layout="flex:row">       <!-- Default: horizontal -->
<reed as="div" layout="flex:column">    <!-- Vertical -->
<reed as="div" layout="flex:row-reverse">
<reed as="div" layout="flex:column-reverse">

<!-- Combined setup -->
<reed as="div" layout="[flex:row, gap:4, align:center, justify:between]">
```

### Flex Alignment

```html
<!-- Justify content (main axis) -->
<reed as="div" layout="justify:start">    <!-- flex-start -->
<reed as="div" layout="justify:center">   <!-- center -->
<reed as="div" layout="justify:end">      <!-- flex-end -->
<reed as="div" layout="justify:between">  <!-- space-between -->
<reed as="div" layout="justify:around">   <!-- space-around -->
<reed as="div" layout="justify:evenly">   <!-- space-evenly -->

<!-- Align items (cross axis) -->
<reed as="div" layout="align:start">      <!-- flex-start -->
<reed as="div" layout="align:center">     <!-- center -->
<reed as="div" layout="align:end">        <!-- flex-end -->
<reed as="div" layout="align:stretch">    <!-- stretch -->
<reed as="div" layout="align:baseline">   <!-- baseline -->

<!-- Align content (multi-line) -->
<reed as="div" layout="content:start">
<reed as="div" layout="content:center">
<reed as="div" layout="content:end">
<reed as="div" layout="content:between">
<reed as="div" layout="content:around">
<reed as="div" layout="content:stretch">
```

### Flex Wrap

```html
<reed as="div" layout="wrap:nowrap">     <!-- Default -->
<reed as="div" layout="wrap:wrap">
<reed as="div" layout="wrap:wrap-reverse">
```

### Flex Item Properties

```html
<!-- Flex grow/shrink/basis -->
<reed as="div" layout="grow:1">          <!-- flex-grow: 1 -->
<reed as="div" layout="grow:0">          <!-- flex-grow: 0 -->
<reed as="div" layout="shrink:1">        <!-- flex-shrink: 1 -->
<reed as="div" layout="shrink:0">        <!-- flex-shrink: 0 -->
<reed as="div" layout="basis:auto">      <!-- flex-basis: auto -->
<reed as="div" layout="basis:full">      <!-- flex-basis: 100% -->

<!-- Self alignment -->
<reed as="div" layout="self:auto">
<reed as="div" layout="self:start">
<reed as="div" layout="self:center">
<reed as="div" layout="self:end">
<reed as="div" layout="self:stretch">

<!-- Order -->
<reed as="div" layout="order:1">
<reed as="div" layout="order:2">
<reed as="div" layout="order:-1">        <!-- Move to front -->
```

## Grid Properties

### Grid Container

```html
<!-- Enable grid -->
<reed as="div" layout="grid">

<!-- Column templates -->
<reed as="div" layout="grid:2">         <!-- 2 equal columns -->
<reed as="div" layout="grid:3">         <!-- 3 equal columns -->
<reed as="div" layout="grid:4">         <!-- 4 equal columns -->
<reed as="div" layout="grid:6">         <!-- 6 equal columns -->
<reed as="div" layout="grid:12">        <!-- 12 equal columns -->

<!-- Custom grid -->
<reed as="div" layout="[grid, cols:1fr:2fr:1fr]">
<reed as="div" layout="[grid, cols:200px:1fr:200px]">

<!-- Row templates -->
<reed as="div" layout="rows:auto">
<reed as="div" layout="rows:1fr:2fr">
<reed as="div" layout="rows:100px:1fr:100px">

<!-- Auto flow -->
<reed as="div" layout="flow:row">       <!-- Default -->
<reed as="div" layout="flow:column">
<reed as="div" layout="flow:dense">
```

### Grid Alignment

```html
<!-- Justify items -->
<reed as="div" layout="justify-items:start">
<reed as="div" layout="justify-items:center">
<reed as="div" layout="justify-items:end">
<reed as="div" layout="justify-items:stretch">

<!-- Align items -->
<reed as="div" layout="align-items:start">
<reed as="div" layout="align-items:center">
<reed as="div" layout="align-items:end">
<reed as="div" layout="align-items:stretch">

<!-- Place items (shorthand) -->
<reed as="div" layout="place:center">    <!-- Center both -->
```

### Grid Item Properties

```html
<!-- Column span -->
<reed as="div" layout="col-span:2">      <!-- Span 2 columns -->
<reed as="div" layout="col-span:3">
<reed as="div" layout="col-span:full">   <!-- Span all columns -->

<!-- Row span -->
<reed as="div" layout="row-span:2">      <!-- Span 2 rows -->
<reed as="div" layout="row-span:3">

<!-- Grid area -->
<reed as="div" layout="area:header">     <!-- Named area -->
<reed as="div" layout="area:sidebar">
<reed as="div" layout="area:content">
```

## Gap (Flexbox & Grid)

```html
<!-- Universal gap -->
<reed as="div" layout="gap:0">          <!-- No gap -->
<reed as="div" layout="gap:1">          <!-- 0.25rem -->
<reed as="div" layout="gap:2">          <!-- 0.5rem -->
<reed as="div" layout="gap:4">          <!-- 1rem -->
<reed as="div" layout="gap:6">          <!-- 1.5rem -->
<reed as="div" layout="gap:8">          <!-- 2rem -->

<!-- Separate row/column gaps -->
<reed as="div" layout="gap-x:4">        <!-- Column gap -->
<reed as="div" layout="gap-y:2">        <!-- Row gap -->
<reed as="div" layout="[gap-x:4, gap-y:2]">
```

## Position

```html
<!-- Position types -->
<reed as="div" layout="position:static">   <!-- Default -->
<reed as="div" layout="position:relative">
<reed as="div" layout="position:absolute">
<reed as="div" layout="position:fixed">
<reed as="div" layout="position:sticky">

<!-- Position values -->
<reed as="div" layout="[position:absolute, top:0, left:0]">
<reed as="div" layout="[position:fixed, bottom:4, right:4]">
<reed as="div" layout="[position:sticky, top:0]">

<!-- Inset shortcuts -->
<reed as="div" layout="inset:0">         <!-- All sides 0 -->
<reed as="div" layout="inset:4">         <!-- All sides 1rem -->
<reed as="div" layout="inset-x:0">       <!-- Left & right 0 -->
<reed as="div" layout="inset-y:0">       <!-- Top & bottom 0 -->
```

## Z-Index

```html
<reed as="div" layout="z:0">            <!-- z-index: 0 -->
<reed as="div" layout="z:10">           <!-- z-index: 10 -->
<reed as="div" layout="z:20">           <!-- z-index: 20 -->
<reed as="div" layout="z:30">           <!-- z-index: 30 -->
<reed as="div" layout="z:40">           <!-- z-index: 40 -->
<reed as="div" layout="z:50">           <!-- z-index: 50 -->
<reed as="div" layout="z:auto">         <!-- z-index: auto -->
<reed as="div" layout="z:-1">           <!-- Behind content -->
```

## Float & Clear

```html
<!-- Float -->
<reed as="div" layout="float:left">
<reed as="div" layout="float:right">
<reed as="div" layout="float:none">

<!-- Clear -->
<reed as="div" layout="clear:left">
<reed as="div" layout="clear:right">
<reed as="div" layout="clear:both">
```

## Common Patterns

### Centered Flexbox

```html
<reed as="div" 
      layout="[flex, align:center, justify:center]">
  Centered content
</reed>
```

### Responsive Grid

```html
<reed as="div" 
      layout="[grid:1, gap:4]"
      layout-tablet="[grid:2, gap:6]"
      layout-screen="[grid:3, gap:8]">
  <reed as="card">Item 1</reed>
  <reed as="card">Item 2</reed>
  <reed as="card">Item 3</reed>
</reed>
```

### Sidebar Layout

```html
<reed as="div" 
      layout="[grid, cols:250px:1fr, gap:6]">
  <reed as="aside">Sidebar</reed>
  <reed as="main">Content</reed>
</reed>
```

### Holy Grail Layout

```html
<reed as="div" 
      layout="[grid, rows:auto:1fr:auto, cols:200px:1fr:200px]">
  <reed as="header" layout="col-span:3">Header</reed>
  <reed as="nav">Left</reed>
  <reed as="main">Content</reed>
  <reed as="aside">Right</reed>
  <reed as="footer" layout="col-span:3">Footer</reed>
</reed>
```

### Sticky Header

```html
<reed as="header" 
      layout="[position:sticky, top:0, z:50]">
  Sticky navigation
</reed>
```

### Modal Overlay

```html
<reed as="div" 
      layout="[position:fixed, inset:0, z:100]"
      face="[bg:base-1000, opacity:50]">
  <reed as="modal" 
        layout="[position:absolute, top:50%, left:50%, transform:translate:-50%:-50%]">
    Modal content
  </reed>
</reed>
```

### Masonry Grid

```html
<reed as="div" 
      layout="[grid:3, gap:4, flow:dense]">
  <reed as="card" layout="row-span:2">Tall</reed>
  <reed as="card">Normal</reed>
  <reed as="card" layout="col-span:2">Wide</reed>
</reed>
```

## Best Practices

1. **Use flexbox for 1D layouts** (rows or columns)
2. **Use grid for 2D layouts** (rows and columns)
3. **Mobile-first responsive** grids
4. **Consistent gap values** from the scale
5. **Semantic z-index levels** (10, 20, 30...)
6. **Position sparingly** - prefer flex/grid

## Next: [Text Namespace](107-text.md)