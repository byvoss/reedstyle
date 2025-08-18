# Implementation Guide

How to build and integrate ReedSTYLE into your project.

## Building from Source

### Prerequisites

- Rust 1.70+ with Cargo
- Node.js 18+ with npm
- Lightning CSS (installed via Cargo)
- SWC (for fast JS transpilation) 
- TypeScript Compiler (TSC for type checking)
- Git

### Clone and Build

```bash
# Clone repository
git clone https://github.com/reedstyle/reedstyle.git
cd reedstyle

# Install dependencies
npm install  # For TypeScript/SWC

# Build with Rust (includes Lightning CSS)
cargo run

# The build process:
# 1. Rust generates CSS from namespaces
# 2. Lightning CSS optimizes and minifies
# 3. TSC type-checks TypeScript
# 4. SWC transpiles to JavaScript

# Output in dist/
ls -la dist/
# reedstyle.css      - Development version (~250KB)
# reedstyle.min.css  - Production version (~180KB) 
# reedstyle.js       - Optional JavaScript (SWC output)
# reedstyle.d.ts     - TypeScript definitions
```

### Build Configuration

Configuration files in project root:

```
project-root/
├── reedstyle.config.yaml      # Main configuration
├── reedstyle.colors.yaml      # Brand colors (brand-a to brand-f)
├── reedstyle.fonts.yaml       # Typography (font-a to font-f)
├── reedstyle.components.yaml  # Custom reed elements
└── reedstyle.spacing.yaml     # Optional spacing scale
```

Main config (`reedstyle.config.yaml`):

```yaml
version: 1.0
build:
  minify: true
  sourcemaps: false
  target: "es2020"

# Reference other config files
config:
  colors: ./reedstyle.colors.yaml
  fonts: ./reedstyle.fonts.yaml
  components: ./reedstyle.components.yaml
  
# Features
features:
  autoConvertColors: true  # Convert hex/rgb/hsl to OKLCH
  generateVariations: true # Auto-generate weak/light/intense/bright/strong
  
# Output
output:
  css: ./dist/reedstyle.css
  js: ./dist/reedstyle.js
```

Colors (`reedstyle.colors.yaml`):

```yaml
# All formats automatically converted to OKLCH internally
colors:
  brand-a: "#FF6B6B"           # Hex → OKLCH
  brand-b: "rgb(78, 205, 196)" # RGB → OKLCH
  brand-c: "hsl(200, 70%, 50%)" # HSL → OKLCH
  brand-d: "oklch(70% 0.15 120)" # Already OKLCH
  brand-e: "#FFE66D"
  brand-f: "#A8E6CF"
  
  # Semantic colors
  state-success: "#4CAF50"
  state-warning: "#FF9800"
  state-error: "#F44336"
  state-info: "#2196F3"
```

Fonts (`reedstyle.fonts.yaml`):

```yaml
fonts:
  # Pattern matches brand colors (a-f)
  font-a:
    family: "'Inter', sans-serif"
    weights:
      normal: 400
      bold: 700
  font-b:
    family: "'Playfair Display', serif"
  font-c:
    family: "'JetBrains Mono', monospace"
```

## Integration Methods

### 1. Static Files (Simplest)

```html
<!DOCTYPE html>
<html>
<head>
  <link rel="stylesheet" href="path/to/reedstyle.css">
  <script src="path/to/reedstyle.js" defer></script>
</head>
<body>
  <reed as="hero">
    <h1>Your App</h1>
  </reed>
</body>
</html>
```

### 2. NPM Package

```bash
npm install reedstyle
```

```javascript
// Import CSS
import 'reedstyle/dist/reedstyle.css';

// Optional: Import JavaScript
import ReedStyle from 'reedstyle';

// Initialize
ReedStyle.init({
  components: './components.yaml'
});
```

### 3. CDN

```html
<link rel="stylesheet" 
      href="https://cdn.jsdelivr.net/npm/reedstyle@1.0/dist/reedstyle.min.css">
<script src="https://cdn.jsdelivr.net/npm/reedstyle@1.0/dist/reedstyle.min.js" 
        defer></script>
```

## Framework Integration

### React

```jsx
// ReedElement.jsx
export function Reed({ as = 'div', children, ...props }) {
  const Element = as;
  return <Element {...props} data-reed>{children}</Element>;
}

// Usage
import { Reed } from './ReedElement';

function App() {
  return (
    <Reed as="hero">
      <h1>React App</h1>
      <Reed as="button-primary">Click me</Reed>
    </Reed>
  );
}
```

### Vue

```vue
<!-- ReedElement.vue -->
<template>
  <component :is="as" v-bind="$attrs" data-reed>
    <slot />
  </component>
</template>

<script setup>
defineProps({
  as: { type: String, default: 'div' }
});
</script>

<!-- Usage -->
<template>
  <Reed as="card">
    <h2>Vue Component</h2>
    <Reed as="button-primary">Action</Reed>
  </Reed>
</template>
```

### Angular

```typescript
// reed.component.ts
@Component({
  selector: 'reed',
  template: '<ng-content></ng-content>',
  host: { '[attr.data-reed]': 'true' }
})
export class ReedComponent {
  @Input() as: string = 'div';
  // Dynamic element creation logic
}
```

### Svelte

```svelte
<!-- Reed.svelte -->
<script>
  export let as = 'div';
</script>

<svelte:element this={as} data-reed {...$$restProps}>
  <slot />
</svelte:element>

<!-- Usage -->
<Reed as="hero">
  <h1>Svelte App</h1>
  <Reed as="button-primary">Click</Reed>
</Reed>
```

## Custom Build Pipeline

### Webpack

```javascript
// webpack.config.js
module.exports = {
  module: {
    rules: [
      {
        test: /reedstyle\.css$/,
        use: ['style-loader', 'css-loader']
      }
    ]
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        { from: 'node_modules/reedstyle/dist', to: 'assets' }
      ]
    })
  ]
};
```

### Vite

```javascript
// vite.config.js
export default {
  optimizeDeps: {
    include: ['reedstyle']
  },
  css: {
    postcss: {
      plugins: [
        // ReedSTYLE works without PostCSS
      ]
    }
  }
};
```

### Parcel

```json
// package.json
{
  "source": "src/index.html",
  "scripts": {
    "build": "parcel build"
  },
  "dependencies": {
    "reedstyle": "^1.0.0"
  }
}
```

## Component Development

### Creating Custom Presets

```yaml
# reedstyle.components.yaml
components:
  # Simple component
  notification:
    element: div
    box: "[padding:4, margin:2]"
    face: "[bg:state-info-weak, border:1:state-info, radius:md]"
    text: "[color:state-info-strong]"
    
  # Complex component with variants
  data-table:
    element: div
    layout: "[overflow-x:auto]"
    box: "[width:full]"
    children:
      table:
        face: "[border:1:base-200]"
        text: "[size:small]"
```

### JavaScript Enhancement

```javascript
// Custom reed element behavior
class ReedElement extends HTMLElement {
  connectedCallback() {
    const as = this.getAttribute('as');
    
    // Load component definition
    if (as && !as.includes('-')) {
      this.applyPreset(as);
    }
    
    // Apply responsive attributes
    this.handleResponsive();
  }
  
  applyPreset(name) {
    const preset = ReedStyle.presets[name];
    if (preset) {
      Object.entries(preset).forEach(([namespace, value]) => {
        this.setAttribute(namespace, value);
      });
    }
  }
  
  handleResponsive() {
    // Watch for viewport changes
    const mediaQueries = {
      tablet: window.matchMedia('(min-width: 560px)'),
      screen: window.matchMedia('(min-width: 960px)'),
      wide: window.matchMedia('(min-width: 1260px)')
    };
    
    // Apply responsive attributes
    Object.entries(mediaQueries).forEach(([breakpoint, mq]) => {
      if (mq.matches) {
        this.applyBreakpoint(breakpoint);
      }
    });
  }
}

customElements.define('reed', ReedElement);
```

## Performance Optimization

### CSS Loading Strategy

```html
<!-- Preload critical CSS -->
<link rel="preload" 
      href="reedstyle.css" 
      as="style">

<!-- Load CSS -->
<link rel="stylesheet" 
      href="reedstyle.css">

<!-- Optional: Async JavaScript -->
<script src="reedstyle.js" 
        async></script>
```

### Tree Shaking

Configure your bundler to remove unused namespaces:

```javascript
// reedstyle.config.js
export default {
  namespaces: {
    box: true,
    face: true,
    layout: true,
    text: true,
    fx: false,      // Exclude if not needed
    device: false   // Exclude if not needed
  }
};
```

### Critical CSS

Extract above-the-fold styles:

```javascript
// critical.js
const critical = require('critical');

critical.generate({
  base: 'dist/',
  src: 'index.html',
  css: ['reedstyle.css'],
  target: {
    css: 'critical.css',
    html: 'index-critical.html'
  }
});
```

## Testing

### Visual Regression

```javascript
// backstop.config.js
module.exports = {
  scenarios: [
    {
      label: 'Reed Components',
      url: 'http://localhost:8080/components.html',
      selectors: ['reed[as="card"]', 'reed[as="button-primary"]']
    }
  ]
};
```

### Unit Testing

```javascript
// reed.test.js
describe('Reed Element', () => {
  it('applies preset attributes', () => {
    const reed = document.createElement('reed');
    reed.setAttribute('as', 'card');
    document.body.appendChild(reed);
    
    expect(reed.hasAttribute('data-reed')).toBe(true);
    // Test preset application
  });
});
```

## Deployment

### Production Checklist

- [ ] Minified CSS (`reedstyle.min.css`)
- [ ] Gzipped assets
- [ ] CDN deployment
- [ ] Cache headers set
- [ ] CSP headers configured
- [ ] Responsive images optimized
- [ ] JavaScript optional/deferred
- [ ] Critical CSS inlined

### Docker

```dockerfile
# Dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release
RUN cargo run

FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
```

## Troubleshooting

### Common Issues

1. **Styles not applying**
   - Check CSS file is loaded
   - Verify reed element syntax
   - Check browser console for errors

2. **JavaScript errors**
   - ReedSTYLE works without JS
   - JS is only for enhancements
   - Check browser compatibility

3. **Responsive not working**
   - Verify viewport meta tag
   - Check breakpoint attributes
   - Test actual device widths

## Next: [Contributing](221-contributing.md)