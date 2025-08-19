# Ticket #941: Distribution Strategy

## Status: 📋 Planned

## Decision Log References
- DEC001 - Build Process (pre-built distribution files)
- DEC004 - Lightning CSS (minification strategy)

## Description
Define and implement the distribution strategy for ReedSTYLE - delivering pre-built files that users can directly include without any build process.

## Requirements

### File Structure
```
dist/
├── reedstyle.css         # Development version with comments (~350KB)
├── reedstyle.min.css     # Production minified (~180KB)
├── reedstyle.js          # Optional JavaScript with reed component (~100KB)
├── reedstyle.min.js      # Production minified JS (~40KB)
├── reedstyle.d.ts        # TypeScript definitions
└── LICENSE               # Apache 2.0

cdn/
├── latest/               # Latest stable version
│   └── [files above]
└── v1.0.0/              # Version-specific
    └── [files above]
```

### CSS File Structure

#### reedstyle.css (Development)
```css
/**
 * ReedSTYLE v1.0.0
 * Apache License 2.0
 * https://reedstyle.dev
 */

/* Layer Definitions */
@layer settings, bridge, theme, free;

/* ===== SETTINGS LAYER ===== */
@layer settings {
  /* Framework defaults */
  :root {
    /* Spacing Scale */
    --rs-space-1: 0.25rem;
    --rs-space-2: 0.5rem;
    /* ... */
  }
}

/* ===== BRIDGE LAYER ===== */
@layer bridge {
  /* Reset and normalizations */
  *, *::before, *::after {
    box-sizing: border-box;
  }
  
  /* Semantic HTML defaults */
  button { cursor: pointer; }
  /* ... */
}

/* ===== THEME LAYER ===== */
@layer theme {
  /* User-configurable values */
  :root {
    /* Brand Colors (OKLCH) */
    --rs-brand-a: oklch(68.5% 0.24 25);
    /* ... */
  }
  
  /* Namespace: Box */
  reed[box*="padding:1"] { padding: var(--rs-space-1); }
  /* ... all namespaces ... */
  
  /* Environment Sublayers */
  @layer dev {
    /* Development overrides */
  }
  
  @layer prod {
    /* Production overrides */
  }
}

/* ===== FREE LAYER ===== */
@layer free {
  /* User custom styles go here */
}
```

#### reedstyle.min.css (Production)
- Minified version of above
- No comments
- Compressed selectors
- Optimized with Lightning CSS

### JavaScript File Structure

#### reedstyle.js (Development)
```javascript
/**
 * ReedSTYLE v1.0.0
 * Optional JavaScript Enhancement
 * Apache License 2.0
 */

(function() {
  'use strict';
  
  // Reed Web Component Definition
  class ReedElement extends HTMLElement {
    constructor() {
      super();
    }
    
    connectedCallback() {
      // Process 'as' attribute
      const as = this.getAttribute('as');
      if (as) {
        this.setupComponent(as);
      }
      
      // Process namespace attributes
      this.processNamespaces();
    }
    
    setupComponent(componentName) {
      // Validate component name (a-z and -)
      if (!/^[a-z]+(-[a-z]+){0,2}$/.test(componentName)) {
        console.warn(`Invalid component name: ${componentName}`);
        return;
      }
      
      // Apply component preset if exists
      this.classList.add(`rs-${componentName}`);
    }
    
    processNamespaces() {
      // Process each namespace attribute
      const namespaces = ['box', 'device', 'face', 'fx', 'layout', 'text'];
      
      namespaces.forEach(ns => {
        // Base namespace
        const value = this.getAttribute(ns);
        if (value) this.parseNamespaceValue(ns, value);
        
        // Breakpoint variants
        ['phone', 'tablet', 'screen', 'wide'].forEach(bp => {
          const bpValue = this.getAttribute(`${ns}-${bp}`);
          if (bpValue) this.parseNamespaceValue(`${ns}-${bp}`, bpValue);
        });
        
        // Environment variants
        ['dev', 'staging', 'prod'].forEach(env => {
          const envValue = this.getAttribute(`${ns}-${env}`);
          if (envValue) this.parseNamespaceValue(`${ns}-${env}`, envValue);
        });
      });
    }
    
    parseNamespaceValue(namespace, value) {
      // Array syntax: [property:value, property:value]
      // Already handled by CSS attribute selectors
      // This is for future enhancements
    }
  }
  
  // Register the element
  customElements.define('reed', ReedElement);
  
  // Optional API
  window.ReedStyle = {
    version: '1.0.0',
    
    // Environment control
    environment: {
      current: 'prod',
      
      set(env) {
        this.current = env;
        document.documentElement.setAttribute('data-env', env);
      },
      
      toggle(envName, enabled) {
        // Control environment sublayers
        const style = document.createElement('style');
        style.textContent = enabled 
          ? `@layer theme.${envName} { /* enabled */ }`
          : `@layer theme.${envName} { /* disabled */ }`;
        document.head.appendChild(style);
      }
    },
    
    // Theme control
    theme: {
      set(themeName) {
        document.documentElement.setAttribute('data-theme', themeName);
      }
    },
    
    // Utility functions
    utils: {
      // Convert color to OKLCH
      toOKLCH(color) {
        // Implementation here
      }
    }
  };
  
  // Auto-initialization
  console.log('ReedSTYLE initialized');
  
})();
```

### Usage Instructions

#### Basic HTML Template
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>ReedSTYLE App</title>
  
  <!-- Required: ReedSTYLE CSS -->
  <link rel="stylesheet" href="https://cdn.reedstyle.dev/latest/reedstyle.min.css">
  
  <!-- Optional: JavaScript for reed element -->
  <script src="https://cdn.reedstyle.dev/latest/reedstyle.min.js" defer></script>
</head>
<body>
  <!-- Works without JavaScript -->
  <reed as="hero" layout="[flex:column, align:center]" box="[padding:8]">
    <h1>Welcome to ReedSTYLE</h1>
    <p>No build process required!</p>
    <reed as="button-primary">Get Started</reed>
  </reed>
</body>
</html>
```

### CDN Integration

#### jsDelivr
```html
<!-- Latest version -->
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/reedstyle/dist/reedstyle.min.css">

<!-- Specific version -->
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/reedstyle@1.0.0/dist/reedstyle.min.css">
```

#### unpkg
```html
<!-- Latest version -->
<link rel="stylesheet" href="https://unpkg.com/reedstyle/dist/reedstyle.min.css">

<!-- Specific version -->
<link rel="stylesheet" href="https://unpkg.com/reedstyle@1.0.0/dist/reedstyle.min.css">
```

### Build Process (Internal Only)

Users never run this - we build and distribute:

```bash
# Build all distribution files
cargo run --release

# This generates:
# 1. Full CSS from Rust namespaces
# 2. Minified CSS via Lightning CSS
# 3. JavaScript from TypeScript
# 4. Minified JS via SWC
# 5. TypeScript definitions

# Output to dist/
ls -la dist/
# reedstyle.css      350KB
# reedstyle.min.css  180KB
# reedstyle.js       100KB
# reedstyle.min.js   40KB
# reedstyle.d.ts     20KB
```

## Acceptance Criteria

- [ ] Four distribution files generated
- [ ] CSS works without JavaScript
- [ ] JavaScript auto-initializes when included
- [ ] Reed element works as web component
- [ ] Component names validated (a-z and -)
- [ ] Files under performance budget
- [ ] CDN-ready file structure
- [ ] Version info in file headers
- [ ] Apache 2.0 license included

## Testing

```html
<!-- Test 1: CSS only -->
<link rel="stylesheet" href="dist/reedstyle.css">
<reed as="card">Should be styled without JS</reed>

<!-- Test 2: With JavaScript -->
<script src="dist/reedstyle.js"></script>
<reed as="button-primary">Enhanced with JS</reed>

<!-- Test 3: Invalid component name -->
<reed as="Invalid_Name">Should warn in console</reed>

<!-- Test 4: Production files -->
<link rel="stylesheet" href="dist/reedstyle.min.css">
<script src="dist/reedstyle.min.js"></script>
```

## Dependencies

- Ticket #906 (Rust Build System)
- Ticket #907 (Namespace CSS Generation)
- Ticket #909 (Lightning CSS Integration)
- Ticket #910 (SWC/TSC Pipeline)

## Blocks

- Ticket #927 (CDN Distribution)
- Ticket #928 (Package Publishing)

## Notes

- Keep development files readable with comments
- Production files optimized for size
- No build tools required for users
- JavaScript is truly optional enhancement
- Component validation follows naming rules