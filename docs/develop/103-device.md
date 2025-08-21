# Device Namespace

Controls user interaction properties: cursor, selection, scrolling, and touch.

## Properties

### Cursor

```html
<!-- Common cursors -->
<r-s as="button" device="cursor:pointer">
<r-s as="div" device="cursor:default">
<r-s as="div" device="cursor:none">
<r-s as="div" device="cursor:wait">
<r-s as="div" device="cursor:text">
<r-s as="div" device="cursor:move">
<r-s as="div" device="cursor:grab">
<r-s as="div" device="cursor:grabbing">
<r-s as="div" device="cursor:not-allowed">
<r-s as="div" device="cursor:help">
<r-s as="div" device="cursor:crosshair">
<r-s as="div" device="cursor:zoom-in">
<r-s as="div" device="cursor:zoom-out">

<!-- Resize cursors -->
<r-s as="div" device="cursor:resize">
<r-s as="div" device="cursor:n-resize">
<r-s as="div" device="cursor:e-resize">
<r-s as="div" device="cursor:s-resize">
<r-s as="div" device="cursor:w-resize">
<r-s as="div" device="cursor:ne-resize">
<r-s as="div" device="cursor:nw-resize">
<r-s as="div" device="cursor:se-resize">
<r-s as="div" device="cursor:sw-resize">
```

### Pointer Events

```html
<r-s as="div" device="pointer:none">     <!-- pointer-events: none -->
<r-s as="div" device="pointer:auto">     <!-- pointer-events: auto -->
<r-s as="div" device="pointer:all">      <!-- pointer-events: all -->
```

### User Select

```html
<r-s as="div" device="select:none">      <!-- user-select: none -->
<r-s as="div" device="select:auto">      <!-- user-select: auto -->
<r-s as="div" device="select:text">      <!-- user-select: text -->
<r-s as="div" device="select:all">       <!-- user-select: all -->
```

### Touch Action

```html
<r-s as="div" device="touch:none">       <!-- touch-action: none -->
<r-s as="div" device="touch:auto">       <!-- touch-action: auto -->
<r-s as="div" device="touch:pan-x">      <!-- touch-action: pan-x -->
<r-s as="div" device="touch:pan-y">      <!-- touch-action: pan-y -->
<r-s as="div" device="touch:pinch-zoom"> <!-- touch-action: pinch-zoom -->
<r-s as="div" device="touch:manipulation"> <!-- touch-action: manipulation -->
```

### Scroll Behavior

```html
<r-s as="div" device="scroll:smooth">    <!-- scroll-behavior: smooth -->
<r-s as="div" device="scroll:auto">      <!-- scroll-behavior: auto -->
```

### Scroll Snap

```html
<!-- Container -->
<r-s as="div" device="[snap-type:x, snap-type:mandatory]">
  <!-- scroll-snap-type: x mandatory -->
  
  <!-- Children -->
  <r-s as="div" device="snap-align:start">   <!-- scroll-snap-align: start -->
  <r-s as="div" device="snap-align:center">  <!-- scroll-snap-align: center -->
  <r-s as="div" device="snap-align:end">     <!-- scroll-snap-align: end -->
</r-s>

<!-- Snap types -->
<r-s as="div" device="snap-type:none">
<r-s as="div" device="snap-type:x">
<r-s as="div" device="snap-type:y">
<r-s as="div" device="snap-type:block">
<r-s as="div" device="snap-type:inline">
<r-s as="div" device="snap-type:both">

<!-- Snap strictness -->
<r-s as="div" device="snap-type:mandatory">
<r-s as="div" device="snap-type:proximity">
```

### Resize

```html
<r-s as="textarea" device="resize:none">
<r-s as="div" device="resize:both">
<r-s as="div" device="resize:horizontal">
<r-s as="div" device="resize:vertical">
```

### Will Change

```html
<r-s as="div" device="will-change:transform">
<r-s as="div" device="will-change:opacity">
<r-s as="div" device="will-change:scroll">
<r-s as="div" device="will-change:contents">
<r-s as="div" device="will-change:auto">
```

## Common Patterns

### Interactive Button

```html
<r-s as="button" 
      device="[cursor:pointer, select:none]">
  Click me
</r-s>
```

### Disabled State

```html
<r-s as="div" 
      device="[cursor:not-allowed, pointer:none, select:none]"
      face="opacity:50">
  Disabled element
</r-s>
```

### Draggable Element

```html
<r-s as="div" 
      device="[cursor:grab, select:none, touch:none]"
      device-active="cursor:grabbing">
  Drag me
</r-s>
```

### Smooth Scrolling Container

```html
<r-s as="div" 
      device="[scroll:smooth, snap-type:y:mandatory]"
      box="[height:screen, overflow-y:scroll]">
  <r-s as="section" device="snap-align:start">
    Section 1
  </r-s>
  <r-s as="section" device="snap-align:start">
    Section 2
  </r-s>
</r-s>
```

### Text Selection Control

```html
<!-- Prevent selection -->
<r-s as="nav" device="select:none">
  Navigation (not selectable)
</r-s>

<!-- Force selection -->
<r-s as="code" device="select:all">
  Click to select all
</r-s>
```

### Mobile Touch Optimization

```html
<r-s as="div" 
      device="[touch:manipulation, select:none]"
      device-tablet="[touch:auto, select:text]">
  Mobile-optimized interaction
</r-s>
```

## Accessibility Considerations

### Cursor States

```html
<!-- Indicate interactivity -->
<r-s as="div" 
      role="button"
      tabindex="0"
      device="cursor:pointer">
  Clickable element
</r-s>

<!-- Show disabled state -->
<r-s as="button" 
      disabled
      device="cursor:not-allowed">
  Disabled button
</r-s>
```

### Selection Control

```html
<!-- Important text should be selectable -->
<r-s as="article" device="select:text">
  Article content
</r-s>

<!-- UI chrome typically not selectable -->
<r-s as="header" device="select:none">
  App header
</r-s>
```

## Performance Tips

### Will Change

```html
<!-- Optimize for animation -->
<r-s as="div" 
      device="will-change:transform"
      fx="[transform:scale:1.1:hover, transition:smooth]">
  Animated element
</r-s>

<!-- Remove after animation -->
<r-s as="div" 
      device="will-change:auto">
  Static element
</r-s>
```

### Touch Performance

```html
<!-- Optimize scrolling -->
<r-s as="div" 
      device="touch:pan-y"
      box="overflow-y:scroll">
  Vertical scroll only
</r-s>
```

## Best Practices

1. **Match cursor to function** - pointer for clickable, text for editable
2. **Disable selection on UI** - but keep for content
3. **Optimize touch for mobile** - use touch:manipulation
4. **Consider accessibility** - ensure keyboard navigation works
5. **Use will-change sparingly** - only for animated elements

## Next: [Face Namespace](104-face.md)