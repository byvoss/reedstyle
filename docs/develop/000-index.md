# ReedSTYLE Documentation

## Welcome to ReedSTYLE

The HTML-first styling system that eliminates custom CSS and JavaScript in 99% of use cases.

## Documentation Structure

### 🚀 Getting Started
- [001 - Introduction](001-introduction.md) - What is ReedSTYLE?
- [011 - Quick Start](011-quick-start.md) - Build your first page in 5 minutes
- [021 - Reed Element](021-reed-element.md) - Understanding the `<r-s>` element
- [031 - Presets](031-presets.md) - Built-in components

### 📦 Namespaces
- [101 - Overview](101-namespaces-overview.md) - The 6 core namespaces
- [102 - Box](102-box.md) - Size and spacing
- [103 - Device](103-device.md) - User interaction
- [104 - Face](104-face.md) - Visual appearance
- [105 - FX](105-fx.md) - Effects and animations
- [106 - Layout](106-layout.md) - Spatial arrangement
- [107 - Text](107-text.md) - Typography

### 🔧 Developer Guide
- [201 - Architecture](201-architecture.md) - Technical architecture
- [211 - Implementation](211-implementation.md) - Building ReedSTYLE
- [221 - Contributing](221-contributing.md) - How to contribute

### 🎯 Advanced Features
- [301 - Custom Components](301-custom-components.md) - YAML components
- [311 - Responsive Design](311-responsive-design.md) - Breakpoints system
- [321 - JavaScript API](321-javascript-api.md) - Enhancement features
- [401 - Configuration](401-configuration.md) - Colors, fonts & components setup
- [501 - Migration](501-migration.md) - Migrate from other frameworks
- [601 - Environments](601-environments.md) - Dev, staging, prod & custom environments

### 📚 Resources
- [Examples](https://reedstyle.dev/examples)
- [Component Gallery](https://reedstyle.dev/gallery)
- [GitHub](https://github.com/reedstyle)

## Quick Example

```html
<!DOCTYPE html>
<html>
<head>
  <link rel="stylesheet" href="reedstyle.css">
  <script src="reedstyle.js"></script>
</head>
<body>
  <r-s as="hero">
    <h1>Welcome to ReedSTYLE</h1>
    <p>Build beautiful websites with just HTML</p>
    <r-s as="button-primary">Get Started</r-s>
  </r-s>
  
  <r-s as="section" layout="[grid:3, gap:6]" box="[padding:8]">
    <r-s as="card">
      <h3>Simple</h3>
      <p>No build tools required</p>
    </r-s>
    <r-s as="card">
      <h3>Powerful</h3>
      <p>Full styling control</p>
    </r-s>
    <r-s as="card">
      <h3>Fast</h3>
      <p>35% smaller CSS</p>
    </r-s>
  </r-s>
</body>
</html>
```

## Philosophy

**Write HTML. Get Design.**

No CSS files. No JavaScript frameworks. No build process. Just semantic HTML with the `<r-s>` element.

## Browser Support

- ✅ Chrome/Edge 90+
- ✅ Firefox 88+
- ✅ Safari 14+
- ✅ All mobile browsers

## License

Apache License 2.0 - Use freely in personal and commercial projects.