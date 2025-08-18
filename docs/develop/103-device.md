# Device Namespace

Controls user interaction properties: cursor, selection, scrolling, and touch.

## Properties

### Cursor

```html
<!-- Common cursors -->
<reed as="button" device="cursor:pointer">
<reed as="div" device="cursor:default">
<reed as="div" device="cursor:none">
<reed as="div" device="cursor:wait">
<reed as="div" device="cursor:text">
<reed as="div" device="cursor:move">
<reed as="div" device="cursor:grab">
<reed as="div" device="cursor:grabbing">
<reed as="div" device="cursor:not-allowed">
<reed as="div" device="cursor:help">
<reed as="div" device="cursor:crosshair">
<reed as="div" device="cursor:zoom-in">
<reed as="div" device="cursor:zoom-out">

<!-- Resize cursors -->
<reed as="div" device="cursor:resize">
<reed as="div" device="cursor:n-resize">
<reed as="div" device="cursor:e-resize">
<reed as="div" device="cursor:s-resize">
<reed as="div" device="cursor:w-resize">
<reed as="div" device="cursor:ne-resize">
<reed as="div" device="cursor:nw-resize">
<reed as="div" device="cursor:se-resize">
<reed as="div" device="cursor:sw-resize">
```

### Pointer Events

```html
<reed as="div" device="pointer:none">     <!-- pointer-events: none -->
<reed as="div" device="pointer:auto">     <!-- pointer-events: auto -->
<reed as="div" device="pointer:all">      <!-- pointer-events: all -->
```

### User Select

```html
<reed as="div" device="select:none">      <!-- user-select: none -->
<reed as="div" device="select:auto">      <!-- user-select: auto -->
<reed as="div" device="select:text">      <!-- user-select: text -->
<reed as="div" device="select:all">       <!-- user-select: all -->
```

### Touch Action

```html
<reed as="div" device="touch:none">       <!-- touch-action: none -->
<reed as="div" device="touch:auto">       <!-- touch-action: auto -->
<reed as="div" device="touch:pan-x">      <!-- touch-action: pan-x -->
<reed as="div" device="touch:pan-y">      <!-- touch-action: pan-y -->
<reed as="div" device="touch:pinch-zoom"> <!-- touch-action: pinch-zoom -->
<reed as="div" device="touch:manipulation"> <!-- touch-action: manipulation -->
```

### Scroll Behavior

```html
<reed as="div" device="scroll:smooth">    <!-- scroll-behavior: smooth -->
<reed as="div" device="scroll:auto">      <!-- scroll-behavior: auto -->
```

### Scroll Snap

```html
<!-- Container -->
<reed as="div" device="[snap-type:x, snap-type:mandatory]">
  <!-- scroll-snap-type: x mandatory -->
  
  <!-- Children -->
  <reed as="div" device="snap-align:start">   <!-- scroll-snap-align: start -->
  <reed as="div" device="snap-align:center">  <!-- scroll-snap-align: center -->
  <reed as="div" device="snap-align:end">     <!-- scroll-snap-align: end -->
</reed>

<!-- Snap types -->
<reed as="div" device="snap-type:none">
<reed as="div" device="snap-type:x">
<reed as="div" device="snap-type:y">
<reed as="div" device="snap-type:block">
<reed as="div" device="snap-type:inline">
<reed as="div" device="snap-type:both">

<!-- Snap strictness -->
<reed as="div" device="snap-type:mandatory">
<reed as="div" device="snap-type:proximity">
```

### Resize

```html
<reed as="textarea" device="resize:none">
<reed as="div" device="resize:both">
<reed as="div" device="resize:horizontal">
<reed as="div" device="resize:vertical">
```

### Will Change

```html
<reed as="div" device="will-change:transform">
<reed as="div" device="will-change:opacity">
<reed as="div" device="will-change:scroll">
<reed as="div" device="will-change:contents">
<reed as="div" device="will-change:auto">
```

## Common Patterns

### Interactive Button

```html
<reed as="button" 
      device="[cursor:pointer, select:none]">
  Click me
</reed>
```

### Disabled State

```html
<reed as="div" 
      device="[cursor:not-allowed, pointer:none, select:none]"
      face="opacity:50">
  Disabled element
</reed>
```

### Draggable Element

```html
<reed as="div" 
      device="[cursor:grab, select:none, touch:none]"
      device-active="cursor:grabbing">
  Drag me
</reed>
```

### Smooth Scrolling Container

```html
<reed as="div" 
      device="[scroll:smooth, snap-type:y:mandatory]"
      box="[height:screen, overflow-y:scroll]">
  <reed as="section" device="snap-align:start">
    Section 1
  </reed>
  <reed as="section" device="snap-align:start">
    Section 2
  </reed>
</reed>
```

### Text Selection Control

```html
<!-- Prevent selection -->
<reed as="nav" device="select:none">
  Navigation (not selectable)
</reed>

<!-- Force selection -->
<reed as="code" device="select:all">
  Click to select all
</reed>
```

### Mobile Touch Optimization

```html
<reed as="div" 
      device="[touch:manipulation, select:none]"
      device-tablet="[touch:auto, select:text]">
  Mobile-optimized interaction
</reed>
```

## Accessibility Considerations

### Cursor States

```html
<!-- Indicate interactivity -->
<reed as="div" 
      role="button"
      tabindex="0"
      device="cursor:pointer">
  Clickable element
</reed>

<!-- Show disabled state -->
<reed as="button" 
      disabled
      device="cursor:not-allowed">
  Disabled button
</reed>
```

### Selection Control

```html
<!-- Important text should be selectable -->
<reed as="article" device="select:text">
  Article content
</reed>

<!-- UI chrome typically not selectable -->
<reed as="header" device="select:none">
  App header
</reed>
```

## Performance Tips

### Will Change

```html
<!-- Optimize for animation -->
<reed as="div" 
      device="will-change:transform"
      fx="[transform:scale:1.1:hover, transition:smooth]">
  Animated element
</reed>

<!-- Remove after animation -->
<reed as="div" 
      device="will-change:auto">
  Static element
</reed>
```

### Touch Performance

```html
<!-- Optimize scrolling -->
<reed as="div" 
      device="touch:pan-y"
      box="overflow-y:scroll">
  Vertical scroll only
</reed>
```

## Best Practices

1. **Match cursor to function** - pointer for clickable, text for editable
2. **Disable selection on UI** - but keep for content
3. **Optimize touch for mobile** - use touch:manipulation
4. **Consider accessibility** - ensure keyboard navigation works
5. **Use will-change sparingly** - only for animated elements

## Next: [Face Namespace](104-face.md)