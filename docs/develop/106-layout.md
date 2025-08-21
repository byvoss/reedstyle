# Layout Namespace

Controls spatial arrangement: flexbox, grid, positioning, and z-index.

## Flexbox Properties

### Flex Container

```html
<!-- Enable flex -->
<r-s as="div" layout="flex">

<!-- Flex direction -->
<r-s as="div" layout="flex:row">       <!-- Default: horizontal -->
<r-s as="div" layout="flex:column">    <!-- Vertical -->
<r-s as="div" layout="flex:row-reverse">
<r-s as="div" layout="flex:column-reverse">

<!-- Combined setup -->
<r-s as="div" layout="[flex:row, gap:4, align:center, justify:between]">
```

### Flex Alignment

```html
<!-- Justify content (main axis) -->
<r-s as="div" layout="justify:start">    <!-- flex-start -->
<r-s as="div" layout="justify:center">   <!-- center -->
<r-s as="div" layout="justify:end">      <!-- flex-end -->
<r-s as="div" layout="justify:between">  <!-- space-between -->
<r-s as="div" layout="justify:around">   <!-- space-around -->
<r-s as="div" layout="justify:evenly">   <!-- space-evenly -->

<!-- Align items (cross axis) -->
<r-s as="div" layout="align:start">      <!-- flex-start -->
<r-s as="div" layout="align:center">     <!-- center -->
<r-s as="div" layout="align:end">        <!-- flex-end -->
<r-s as="div" layout="align:stretch">    <!-- stretch -->
<r-s as="div" layout="align:baseline">   <!-- baseline -->

<!-- Align content (multi-line) -->
<r-s as="div" layout="content:start">
<r-s as="div" layout="content:center">
<r-s as="div" layout="content:end">
<r-s as="div" layout="content:between">
<r-s as="div" layout="content:around">
<r-s as="div" layout="content:stretch">
```

### Flex Wrap

```html
<r-s as="div" layout="wrap:nowrap">     <!-- Default -->
<r-s as="div" layout="wrap:wrap">
<r-s as="div" layout="wrap:wrap-reverse">
```

### Flex Item Properties

```html
<!-- Flex grow/shrink/basis -->
<r-s as="div" layout="grow:1">          <!-- flex-grow: 1 -->
<r-s as="div" layout="grow:0">          <!-- flex-grow: 0 -->
<r-s as="div" layout="shrink:1">        <!-- flex-shrink: 1 -->
<r-s as="div" layout="shrink:0">        <!-- flex-shrink: 0 -->
<r-s as="div" layout="basis:auto">      <!-- flex-basis: auto -->
<r-s as="div" layout="basis:full">      <!-- flex-basis: 100% -->

<!-- Self alignment -->
<r-s as="div" layout="self:auto">
<r-s as="div" layout="self:start">
<r-s as="div" layout="self:center">
<r-s as="div" layout="self:end">
<r-s as="div" layout="self:stretch">

<!-- Order -->
<r-s as="div" layout="order:1">
<r-s as="div" layout="order:2">
<r-s as="div" layout="order:-1">        <!-- Move to front -->
```

## Grid Properties

### Grid Container

```html
<!-- Enable grid -->
<r-s as="div" layout="grid">

<!-- Column templates -->
<r-s as="div" layout="grid:2">         <!-- 2 equal columns -->
<r-s as="div" layout="grid:3">         <!-- 3 equal columns -->
<r-s as="div" layout="grid:4">         <!-- 4 equal columns -->
<r-s as="div" layout="grid:6">         <!-- 6 equal columns -->
<r-s as="div" layout="grid:12">        <!-- 12 equal columns -->

<!-- Custom grid -->
<r-s as="div" layout="[grid, cols:1fr:2fr:1fr]">
<r-s as="div" layout="[grid, cols:200px:1fr:200px]">

<!-- Row templates -->
<r-s as="div" layout="rows:auto">
<r-s as="div" layout="rows:1fr:2fr">
<r-s as="div" layout="rows:100px:1fr:100px">

<!-- Auto flow -->
<r-s as="div" layout="flow:row">       <!-- Default -->
<r-s as="div" layout="flow:column">
<r-s as="div" layout="flow:dense">
```

### Grid Alignment

```html
<!-- Justify items -->
<r-s as="div" layout="justify-items:start">
<r-s as="div" layout="justify-items:center">
<r-s as="div" layout="justify-items:end">
<r-s as="div" layout="justify-items:stretch">

<!-- Align items -->
<r-s as="div" layout="align-items:start">
<r-s as="div" layout="align-items:center">
<r-s as="div" layout="align-items:end">
<r-s as="div" layout="align-items:stretch">

<!-- Place items (shorthand) -->
<r-s as="div" layout="place:center">    <!-- Center both -->
```

### Grid Item Properties

```html
<!-- Column span -->
<r-s as="div" layout="col-span:2">      <!-- Span 2 columns -->
<r-s as="div" layout="col-span:3">
<r-s as="div" layout="col-span:full">   <!-- Span all columns -->

<!-- Row span -->
<r-s as="div" layout="row-span:2">      <!-- Span 2 rows -->
<r-s as="div" layout="row-span:3">

<!-- Grid area -->
<r-s as="div" layout="area:header">     <!-- Named area -->
<r-s as="div" layout="area:sidebar">
<r-s as="div" layout="area:content">
```

## Gap (Flexbox & Grid)

```html
<!-- Universal gap -->
<r-s as="div" layout="gap:0">          <!-- No gap -->
<r-s as="div" layout="gap:1">          <!-- 0.25rem -->
<r-s as="div" layout="gap:2">          <!-- 0.5rem -->
<r-s as="div" layout="gap:4">          <!-- 1rem -->
<r-s as="div" layout="gap:6">          <!-- 1.5rem -->
<r-s as="div" layout="gap:8">          <!-- 2rem -->

<!-- Separate row/column gaps -->
<r-s as="div" layout="gap-x:4">        <!-- Column gap -->
<r-s as="div" layout="gap-y:2">        <!-- Row gap -->
<r-s as="div" layout="[gap-x:4, gap-y:2]">
```

## Position

```html
<!-- Position types -->
<r-s as="div" layout="position:static">   <!-- Default -->
<r-s as="div" layout="position:relative">
<r-s as="div" layout="position:absolute">
<r-s as="div" layout="position:fixed">
<r-s as="div" layout="position:sticky">

<!-- Position values -->
<r-s as="div" layout="[position:absolute, top:0, left:0]">
<r-s as="div" layout="[position:fixed, bottom:4, right:4]">
<r-s as="div" layout="[position:sticky, top:0]">

<!-- Inset shortcuts -->
<r-s as="div" layout="inset:0">         <!-- All sides 0 -->
<r-s as="div" layout="inset:4">         <!-- All sides 1rem -->
<r-s as="div" layout="inset-x:0">       <!-- Left & right 0 -->
<r-s as="div" layout="inset-y:0">       <!-- Top & bottom 0 -->
```

## Z-Index

```html
<r-s as="div" layout="z:0">            <!-- z-index: 0 -->
<r-s as="div" layout="z:10">           <!-- z-index: 10 -->
<r-s as="div" layout="z:20">           <!-- z-index: 20 -->
<r-s as="div" layout="z:30">           <!-- z-index: 30 -->
<r-s as="div" layout="z:40">           <!-- z-index: 40 -->
<r-s as="div" layout="z:50">           <!-- z-index: 50 -->
<r-s as="div" layout="z:auto">         <!-- z-index: auto -->
<r-s as="div" layout="z:-1">           <!-- Behind content -->
```

## Float & Clear

```html
<!-- Float -->
<r-s as="div" layout="float:left">
<r-s as="div" layout="float:right">
<r-s as="div" layout="float:none">

<!-- Clear -->
<r-s as="div" layout="clear:left">
<r-s as="div" layout="clear:right">
<r-s as="div" layout="clear:both">
```

## Common Patterns

### Centered Flexbox

```html
<r-s as="div" 
      layout="[flex, align:center, justify:center]">
  Centered content
</r-s>
```

### Responsive Grid

```html
<r-s as="div" 
      layout="[grid:1, gap:4]"
      layout-tablet="[grid:2, gap:6]"
      layout-screen="[grid:3, gap:8]">
  <r-s as="card">Item 1</r-s>
  <r-s as="card">Item 2</r-s>
  <r-s as="card">Item 3</r-s>
</r-s>
```

### Sidebar Layout

```html
<r-s as="div" 
      layout="[grid, cols:250px:1fr, gap:6]">
  <r-s as="aside">Sidebar</r-s>
  <r-s as="main">Content</r-s>
</r-s>
```

### Holy Grail Layout

```html
<r-s as="div" 
      layout="[grid, rows:auto:1fr:auto, cols:200px:1fr:200px]">
  <r-s as="header" layout="col-span:3">Header</r-s>
  <r-s as="nav">Left</r-s>
  <r-s as="main">Content</r-s>
  <r-s as="aside">Right</r-s>
  <r-s as="footer" layout="col-span:3">Footer</r-s>
</r-s>
```

### Sticky Header

```html
<r-s as="header" 
      layout="[position:sticky, top:0, z:50]">
  Sticky navigation
</r-s>
```

### Modal Overlay

```html
<r-s as="div" 
      layout="[position:fixed, inset:0, z:100]"
      face="[bg:base-1000, opacity:50]">
  <r-s as="modal" 
        layout="[position:absolute, top:50%, left:50%, transform:translate:-50%:-50%]">
    Modal content
  </r-s>
</r-s>
```

### Masonry Grid

```html
<r-s as="div" 
      layout="[grid:3, gap:4, flow:dense]">
  <r-s as="card" layout="row-span:2">Tall</r-s>
  <r-s as="card">Normal</r-s>
  <r-s as="card" layout="col-span:2">Wide</r-s>
</r-s>
```

## Best Practices

1. **Use flexbox for 1D layouts** (rows or columns)
2. **Use grid for 2D layouts** (rows and columns)
3. **Mobile-first responsive** grids
4. **Consistent gap values** from the scale
5. **Semantic z-index levels** (10, 20, 30...)
6. **Position sparingly** - prefer flex/grid

## Next: [Text Namespace](107-text.md)