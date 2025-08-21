# Introduction to ReedSTYLE

## What is ReedSTYLE?

ReedSTYLE is an HTML-first styling system that eliminates the need for custom CSS and JavaScript in 99% of use cases. Instead of writing styles, you write semantic HTML with the `<r-s>` element.

## The Problem It Solves

Traditional web development requires:
- Writing HTML for structure
- Writing CSS for styling  
- Writing JavaScript for interactions
- Managing complex build tools
- Dealing with framework-specific syntax

This creates:
- **Complexity**: Three languages for one interface
- **Maintenance burden**: Styles scattered across files
- **Performance issues**: Large CSS/JS bundles
- **Learning curve**: Framework-specific conventions

## The ReedSTYLE Solution

```html
<!-- Traditional approach -->
<div class="flex flex-row gap-4 p-6 bg-white rounded-lg shadow-md">
  <div class="text-xl font-bold text-gray-900">Title</div>
</div>

<!-- ReedSTYLE approach -->
<r-s as="card">
  <h3>Title</h3>
</r-s>
```

## Core Philosophy

### 1. HTML is the API
No classes, no inline styles. Just HTML elements with intuitive attributes.

### 2. Presets for Common Patterns
Built-in components like `card`, `hero`, `button-primary` cover 90% of use cases.

### 3. Extensible When Needed
When presets aren't enough, use namespace attributes for fine control.

```html
<!-- Use preset -->
<r-s as="card">Standard card</r-s>

<!-- Customize preset -->
<r-s as="card" face="bg:brand-a">Branded card</r-s>

<!-- Full control -->
<r-s as="div" 
      layout="[grid:3, gap:4]"
      box="[padding:6]"
      face="[bg:base-100, radius:lg, shadow:md]">
  Custom layout
</r-s>
```

## Key Benefits

### 35% Smaller CSS
Direct attribute selectors are more efficient than class-based systems.

### No Build Step Required
Include `reedstyle.css` and optionally `reedstyle.js`. That's it.

### Framework Agnostic
Works with React, Vue, Svelte, or vanilla HTML.

### Semantic HTML
Your markup remains clean and meaningful.

### Progressive Enhancement
JavaScript adds features but isn't required for core styling.

## How It Works

1. **Reed Element**: Custom HTML element that accepts styling attributes
2. **Namespace System**: 6 logical groups for all CSS properties
3. **Array Syntax**: Combine multiple properties efficiently
4. **Responsive Variants**: Every property supports breakpoint suffixes
5. **CSS Layers**: Predictable cascade management

## Browser Support

- Chrome/Edge 90+
- Firefox 88+
- Safari 14+
- All mobile browsers

## Next Steps

- [Quick Start](011-quick-start.md) - Build your first page
- [Reed Element](021-reed-element.md) - Deep dive into the reed element
- [Namespaces](101-namespaces-overview.md) - Understanding the property system