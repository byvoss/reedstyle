# Quick Start

Build your first ReedSTYLE page in 5 minutes.

## Installation

### Distribution Files

ReedSTYLE ships pre-built files - no build process needed:

- **reedstyle.css** - Development version with comments (~350KB)
- **reedstyle.min.css** - Production minified (~180KB)  
- **reedstyle.js** - Optional JavaScript enhancement (~100KB)
- **reedstyle.min.js** - Production minified JS (~40KB)

### Option 1: CDN (Quickest)

```html
<!DOCTYPE html>
<html>
<head>
  <!-- Required: CSS -->
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/reedstyle/dist/reedstyle.min.css">
  
  <!-- Optional: JavaScript for reed web component -->
  <script src="https://cdn.jsdelivr.net/npm/reedstyle/dist/reedstyle.min.js" defer></script>
</head>
<body>
  <!-- Works with CSS only! -->
  <reed as="hero">
    <h1>Welcome</h1>
  </reed>
</body>
</html>
```

### Option 2: Download

```bash
# Download files
curl -O https://cdn.reedstyle.dev/latest/reedstyle.min.css
curl -O https://cdn.reedstyle.dev/latest/reedstyle.min.js
```

Then include:
```html
<!-- Development -->
<link rel="stylesheet" href="reedstyle.css">
<script src="reedstyle.js" defer></script>

<!-- Production -->
<link rel="stylesheet" href="reedstyle.min.css">
<script src="reedstyle.min.js" defer></script>
```

### Option 3: NPM

```bash
npm install reedstyle
```

Then include:
```html
<link rel="stylesheet" href="node_modules/reedstyle/dist/reedstyle.min.css">
<script src="node_modules/reedstyle/dist/reedstyle.min.js" defer></script>
```

**Important:** JavaScript is optional! The CSS alone provides full styling.

## Configuration (Optional)

Customize ReedSTYLE by creating YAML config files in your project root:

```yaml
# reedstyle.colors.yaml - Define your brand colors
colors:
  brand-a: "#FF6B6B"  # Automatically converted to OKLCH
  brand-b: "rgb(78, 205, 196)"  # Any format works
  brand-c: "hsl(200, 70%, 50%)"
  
# reedstyle.fonts.yaml - Typography settings  
fonts:
  font-a:
    family: "'Inter', sans-serif"
  font-b:
    family: "'Playfair Display', serif"
    
# reedstyle.components.yaml - Custom components
components:
  my-card:
    element: div
    box: "[padding:6]"
    face: "[bg:brand-a-weak, radius:lg]"
```

## Your First Page

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>My First ReedSTYLE Page</title>
  <link rel="stylesheet" href="reedstyle.css">
  <script src="reedstyle.js" defer></script>
</head>
<body>
  <!-- Hero Section -->
  <reed as="hero">
    <h1>Welcome to ReedSTYLE</h1>
    <p>Build beautiful websites with just HTML</p>
    <reed as="button-primary">Get Started</reed>
  </reed>

  <!-- Features Grid -->
  <reed as="section" layout="[grid:3, gap:6]" box="[padding:8]">
    <reed as="card">
      <h3>Simple</h3>
      <p>No build tools required. Just include and use.</p>
    </reed>
    
    <reed as="card">
      <h3>Powerful</h3>
      <p>Full control when you need it.</p>
    </reed>
    
    <reed as="card">
      <h3>Fast</h3>
      <p>35% smaller CSS than traditional frameworks.</p>
    </reed>
  </reed>

  <!-- Call to Action -->
  <reed as="section" text="align:center" box="[padding:10]">
    <h2>Ready to simplify your workflow?</h2>
    <reed as="button-group">
      <reed as="button-primary">Documentation</reed>
      <reed as="button-secondary">Examples</reed>
    </reed>
  </reed>
</body>
</html>
```

## Understanding the Basics

### 1. The Reed Element

```html
<reed as="div">Content</reed>
```

The `as` attribute determines the component type. Naming rules:
- Only lowercase letters `a-z` and hyphen `-`
- Maximum 2 hyphens recommended
- Examples: `card`, `button-primary`, `hero-section`

Common presets:
- `card`, `hero`, `container`
- `button`, `button-primary`, `button-secondary`
- `nav`, `nav-brand`, `nav-links`

### 2. Namespace Attributes

```html
<reed as="div" 
      layout="[grid:2]"
      box="[padding:4]"
      face="[bg:brand-a, radius:lg]">
  Content
</reed>
```

Each namespace controls specific properties:
- `layout` - Positioning and arrangement
- `box` - Size and spacing
- `face` - Visual appearance
- `text` - Typography
- `fx` - Effects and animations
- `device` - User interaction

### 3. Array Syntax

Combine multiple properties using arrays:

```html
<!-- Single property -->
<reed as="div" box="padding:4">

<!-- Multiple properties -->
<reed as="div" box="[padding:4, margin:2, width:full]">
```

### 4. Responsive Design

Add breakpoint suffixes for responsive behavior:

```html
<reed as="div" 
      layout="[flex:column]"
      layout-tablet="[flex:row]"
      layout-screen="[grid:3]">
  Responsive content
</reed>
```

## Common Patterns

### Navigation Bar

```html
<reed as="nav">
  <reed as="brand">MyApp</reed>
  <reed as="nav-links">
    <a href="/">Home</a>
    <a href="/about">About</a>
    <a href="/contact">Contact</a>
  </reed>
</reed>
```

### Form

```html
<reed as="form">
  <reed as="field">
    <label>Email</label>
    <input type="email" required>
  </reed>
  
  <reed as="field">
    <label>Message</label>
    <textarea rows="4"></textarea>
  </reed>
  
  <reed as="button-primary">Send</reed>
</reed>
```

### Modal

```html
<reed as="modal" id="my-modal">
  <reed as="modal-header">
    <h3>Modal Title</h3>
    <reed as="close">&times;</reed>
  </reed>
  <reed as="modal-body">
    Modal content goes here
  </reed>
  <reed as="modal-footer">
    <reed as="button-secondary">Cancel</reed>
    <reed as="button-primary">Confirm</reed>
  </reed>
</reed>
```

## Next Steps

- [Reed Element](021-reed-element.md) - All about the reed element
- [Presets](031-presets.md) - Built-in components
- [Namespaces](101-namespaces-overview.md) - Property system
- [Examples](https://reedstyle.dev/examples) - Live demos