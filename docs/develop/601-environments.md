# Environment System

⚠️ **Use Sparingly** - Environments add complexity. For different themes, consider separate theme folders instead.

## Overview

The environment system is available for special cases where you need context-aware styling. However, it should be used rarely to maintain code clarity.

## When to Use Environments

✅ **Good Use Cases:**
- Debug outlines/helpers in development only
- A/B testing specific elements
- Critical production optimizations

❌ **Better Alternatives:**
- **Different client themes** → Use separate theme folders
- **Dark/Light mode** → Use CSS custom properties or prefers-color-scheme
- **Major layout changes** → Create different templates
- **Multiple brands** → Separate ReedSTYLE builds

## Configuration

Environments are defined in `reedstyle.env.yaml`:

```yaml
environments:
  dev:
    name: "development"
    enabled: true
    config:
      debug: true
      minify: false
      
  prod:
    name: "production"
    enabled: false
    config:
      debug: false
      minify: true
```

## Usage in HTML

### Basic Environment Attributes

```html
<!-- Different padding in different environments -->
<r-s as="div" 
      box="[padding:4]"           <!-- Default -->
      box-dev="[padding:8]"        <!-- Development: more space -->
      box-prod="[padding:2]">      <!-- Production: compact -->
  Environment-aware spacing
</r-s>
```

### Combining Environments with Breakpoints

Environments and breakpoints can be combined for ultimate flexibility:

```html
<r-s as="section"
      box="[padding:2]"                <!-- Mobile default -->
      box-tablet="[padding:4]"          <!-- Tablet -->
      box-screen="[padding:6]"          <!-- Desktop -->
      
      box-dev="[padding:4]"             <!-- Dev mobile -->
      box-dev-tablet="[padding:8]"      <!-- Dev tablet -->
      box-dev-screen="[padding:12]"     <!-- Dev desktop -->
      
      box-prod="[padding:1]"            <!-- Prod mobile (compact) -->
      box-prod-tablet="[padding:2]"     <!-- Prod tablet -->
      box-prod-screen="[padding:4]">    <!-- Prod desktop -->
  
  Fully responsive and environment-aware
</r-s>
```

### Priority Order

The cascade priority (highest wins):

1. Environment + Breakpoint: `box-prod-tablet`
2. Environment: `box-prod`
3. Breakpoint: `box-tablet`
4. Base: `box`

## Recommended Approach: Separate Theme Folders

Instead of complex environment attributes, organize themes in folders:

```
project-root/
├── themes/
│   ├── default/
│   │   ├── reedstyle.colors.yaml
│   │   ├── reedstyle.fonts.yaml
│   │   └── reedstyle.components.yaml
│   ├── client-a/
│   │   ├── reedstyle.colors.yaml
│   │   ├── reedstyle.fonts.yaml
│   │   └── reedstyle.components.yaml
│   └── dark-mode/
│       └── reedstyle.colors.yaml
```

Build command:
```bash
# Build default theme
cargo run --theme=default

# Build client theme
cargo run --theme=client-a
```

This keeps your HTML clean:
```html
<!-- Same HTML for all themes -->
<r-s as="card" box="[padding:4]">
  Content
</r-s>
```

## Limited Use Cases

### 1. Debug Mode in Development (Acceptable)

```yaml
# reedstyle.env.yaml
environments:
  dev:
    enabled: true
    variables:
      --debug-outline: "1px solid red"
      --debug-bg: "rgba(255, 0, 0, 0.1)"
```

```html
<!-- Show debug outlines only in dev -->
<r-s as="div"
      face-dev="[outline:1:red]"
      box-dev="[padding:8]">
  Debug helpers in development
</r-s>
```

### 2. A/B Testing (Acceptable)

```yaml
environments:
  test-b:
    name: "variant-b"
    enabled: false  # Enable only for test group
```

```html
<!-- Test different CTA sizes -->
<r-s as="button-primary"
      text="[size:normal]"
      text-test-b="[size:large]">
  Buy Now
</r-s>
```

### 3. Production Optimizations (Acceptable)

```yaml
environments:
  prod:
    name: "production"
    config:
      optimize: maximum
      remove_unused: true
```

```html
<!-- Simpler effects in production -->
<r-s as="button"
      fx="[hover:scale:1.1, transition:smooth]"      <!-- Dev: fancy -->
      fx-prod="[hover:brightness:110]">              <!-- Prod: simple -->
  Performance-conscious effects
</r-s>
```

## Why NOT to Use Environments

### Complexity Grows Quickly

```yaml
environments:
  variant-a:
    name: "test-variant-a"
    variables:
      --cta-color: "oklch(60% 0.2 25)"
      
  variant-b:
    name: "test-variant-b"
    variables:
      --cta-color: "oklch(70% 0.25 120)"
```

```html
<!-- Different CTA styles for testing -->
<r-s as="button-primary"
      face-variant-a="[bg:brand-a]"
      face-variant-b="[bg:brand-b]"
      text-variant-a="[size:large]"
      text-variant-b="[size:huge]">
  A/B Test Button
</r-s>
```

## Environment Detection

### Automatic Detection

```yaml
detection:
  auto:
    enabled: true
    rules:
      - domain: "localhost"
        environment: dev
      - domain: "*.staging.com"
        environment: staging
      - domain: "*.production.com"
        environment: prod
```

### Manual Override

```bash
# Set via environment variable
REEDSTYLE_ENV=prod npm run build

# Or via query parameter (for testing)
https://mysite.com?env=staging
```

### JavaScript API

```javascript
// Get current environment
const env = ReedStyle.environment.current();

// Switch environment dynamically
ReedStyle.environment.set('dark');

// Listen for environment changes
ReedStyle.environment.on('change', (newEnv, oldEnv) => {
  console.log(`Switched from ${oldEnv} to ${newEnv}`);
});
```

## CSS Generation (Theme Sublayers)

### Sublayer Strategy

Environments are generated as sublayers within the theme layer:

```css
@layer theme {
  /* Base theme styles - always active */
  reed[box*="padding:4"] { padding: 1rem; }
  
  /* Environment sublayers - can be toggled */
  @layer dev {
    reed[box-dev*="padding:8"] { padding: 2rem; }
  }
  
  @layer prod {
    reed[box-prod*="padding:2"] { padding: 0.5rem; }
  }
}
```

### Benefits of Sublayers

1. **Global Control** - Enable/disable entire environments with one switch
2. **CMS Integration** - Toggle layers from external systems
3. **Runtime Control** - Change active environment without rebuild
4. **Clean Cascade** - Environment styles properly layered
5. **Zero Overhead** - Disabled layers have no impact

### External Control

```javascript
// From CMS
ReedStyle.environment.setFromConfig({
  environments: {
    dev: false,
    staging: false,
    prod: true
  }
});

// From React
<r-sStyleProvider environments={{ prod: true }}>
  <App />
</r-sStyleProvider>

// From build
REEDSTYLE_ENV=prod cargo run
```

## Environment Combinations

Combine multiple environments:

```yaml
combinations:
  dev-dark:
    extends: [dev, dark]
    name: "development-dark"
    
  prod-client-a:
    extends: [prod, client-a]
    name: "production-client-a"
```

```html
<!-- Works with combined environments -->
<r-s as="div"
      face="[bg:base-0]"
      face-dev-dark="[bg:base-900, outline:1:debug]">
  Combined environment styling
</r-s>
```

## Best Practices

### 1. Environment Naming

Use clear, consistent names:
- `dev`, `staging`, `prod` (deployment stages)
- `dark`, `light` (themes)
- `client-{name}` (multi-tenant)
- `variant-{id}` (A/B testing)

### 2. Progressive Enhancement

```html
<!-- Start simple, enhance per environment -->
<r-s as="card"
      box="[padding:4]"           <!-- Base: works everywhere -->
      fx-dev="[animate:fade-in]"  <!-- Dev: see animations -->
      fx-prod="">                 <!-- Prod: no animation -->
```

### 3. Performance Awareness

```yaml
environments:
  prod:
    config:
      remove_unused: true     # Strip unused styles
      tree_shake: true        # Remove dead code
      minify: true           # Minimize output
```

### 4. Testing Environments

```html
<!-- Test-specific attributes -->
<r-s as="button"
      data-test-id="submit-button"      <!-- Always present -->
      box-test="[outline:2:green]">     <!-- Only in test env -->
  Testable button
</r-s>
```

## Environment Indicators

Show current environment to users (useful in dev/staging):

```yaml
features:
  indicator:
    enabled: true
    position: "top-right"
    style: |
      background: var(--env-color);
      padding: 0.5rem;
```

Result: Small badge showing "DEV", "STAGING", etc.

## Migration Example

Gradually migrate with environments:

```html
<!-- Old framework in dev, ReedSTYLE in prod -->
<div class="old-card" data-env-dev>
  <!-- Old implementation for dev -->
</div>

<r-s as="card" data-env-prod>
  <!-- New implementation for prod -->
</r-s>
```

## Build Optimization

Different builds per environment:

```bash
# Development build (all features)
REEDSTYLE_ENV=dev cargo run

# Production build (optimized)
REEDSTYLE_ENV=prod cargo run

# Output
dist/reedstyle-dev.css    # 400KB with sourcemaps
dist/reedstyle-prod.css   # 346KB minified
```

## Summary

The environment system provides:
- **Context-aware styling** - Different looks for different environments
- **Safe testing** - Try changes in dev before prod
- **Client theming** - Multi-tenant support
- **Performance tuning** - Optimize per environment
- **A/B testing** - Built-in variant support

Combined with breakpoints, you get complete control over how your styles adapt to both viewport AND context.