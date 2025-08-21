# Ticket #914: Environment Sublayer System

## Status: 📋 Planned

## Overview
Implement the environment system using CSS theme sublayers for context-aware styling. This feature should be used SPARINGLY as documented - primarily for debug helpers, A/B testing, and critical production optimizations. NOT for general theming.

## Files to Read First (Understanding Current System)
Before implementing, read these files to understand the existing layer system:

1. **Current CSS Layer System**:
   - `/src/css/mod.rs` - Current layer implementation (@layer theme)
   - `/src/builder/mod.rs` - Build process

2. **Configuration System**:
   - `/src/config/mod.rs` - Configuration structures
   - `/reedstyle.config.yaml` - Main config example

3. **Documentation for Reference**:
   - `/docs/develop/601-environments.md` - CRITICAL: Read warnings about sparse usage
   - `/docs/develop/201-architecture.md` - Layer system architecture

## Objectives
- Implement environment sublayers within theme layer
- Enable toggle on/off for each environment
- Support dev/staging/prod environments
- Allow A/B testing variants
- Maintain zero overhead when disabled
- WARN users about complexity

## Technical Requirements

### 1. Environment Configuration
Create `reedstyle.env.yaml` support:
```yaml
environments:
  # Development environment
  dev:
    name: "development"
    enabled: false  # Disabled by default!
    config:
      debug: true
      minify: false
      
  # Production environment  
  prod:
    name: "production"
    enabled: true  # Only one enabled typically
    config:
      debug: false
      minify: true
      
  # A/B test variant
  variant-b:
    name: "test-variant-b"
    enabled: false
```

### 2. CSS Sublayer Generation
Generate environment sublayers within theme layer:
```css
@layer theme {
  /* Base theme styles (always active) */
  r-s[box*="padding:4"] { padding: 1rem; }
  
  /* Environment sublayers (toggleable) */
  @layer dev {
    r-s[box-dev*="padding:8"] { padding: 2rem; }
    r-s[face-dev*="outline:1:red"] { outline: 1px solid red; }
  }
  
  @layer prod {
    r-s[box-prod*="padding:2"] { padding: 0.5rem; }
    /* Optimized for production */
  }
  
  @layer variant-b {
    r-s[text-variant-b*="size:large"] { font-size: 1.125rem; }
  }
}
```

### 3. Attribute Pattern
Support environment-specific attributes:
```html
<!-- Base styles -->
<r-s box="[padding:4]"
     box-dev="[padding:8, outline:1:debug]"
     box-prod="[padding:2]">
  Content adapts to environment
</r-s>

<!-- With responsive -->
<r-s box-dev="[padding:4]"
     box-dev-tablet="[padding:8]"
     box-prod="[padding:2]"
     box-prod-tablet="[padding:4]">
  Environment + responsive
</r-s>
```

### 4. Build-Time Control
Environment selection at build:
```bash
# Build with specific environment
REEDSTYLE_ENV=prod cargo run

# Or via config
cargo run --env=dev

# Multiple environments (A/B testing)
REEDSTYLE_ENV=prod,variant-b cargo run
```

### 5. Runtime Toggle (JavaScript)
Optional runtime control:
```javascript
// Toggle environment dynamically
ReedStyle.environment.set('dev');

// Enable multiple
ReedStyle.environment.enable(['prod', 'variant-b']);

// Get current
const current = ReedStyle.environment.current();
```

### 6. WARNING System
Add prominent warnings:
```rust
// In build output
if env_count > 1 {
    println!("⚠️  WARNING: {} environments enabled", env_count);
    println!("   Environments add complexity. Use sparingly!");
    println!("   Consider separate theme folders instead.");
}
```

## Implementation Steps

### Phase 1: Configuration
1. Create environment config structure
2. Load `reedstyle.env.yaml` if exists
3. Default to NO environments if missing
4. Add warning when environments detected

### Phase 2: CSS Generation
1. Update namespace modules to support env variants
2. Generate sublayers only for enabled environments
3. Maintain priority order (env+breakpoint > env > breakpoint > base)
4. Keep sublayers within theme layer

### Phase 3: Build Integration
1. Add environment selection to build
2. Support environment variables
3. Show enabled/disabled in output
4. Calculate overhead of environments

### Phase 4: JavaScript API
1. Add environment module to ReedStyle
2. Implement toggle functionality
3. Add detection from URL/domain
4. Create migration helpers

### Phase 5: Documentation
1. Add clear warnings about usage
2. Show alternatives (theme folders)
3. Provide migration path
4. Document performance impact

## Testing Scenarios

### Test 1: Debug Environment
```yaml
environments:
  dev:
    enabled: true
```
```html
<r-s face-dev="[outline:1:red]">
  Should show red outline in dev only
</r-s>
```

### Test 2: A/B Testing
```yaml
environments:
  variant-a:
    enabled: true
  variant-b:
    enabled: false
```
```html
<r-s text-variant-a="[size:normal]"
     text-variant-b="[size:large]">
  Different sizes per variant
</r-s>
```

### Test 3: Production Optimization
```yaml
environments:
  prod:
    enabled: true
```
```html
<r-s fx="[hover:scale:1.1, transition:smooth]"
     fx-prod="[hover:brightness:110]">
  Simpler effects in production
</r-s>
```

## Success Criteria
- [ ] Environment configuration loads
- [ ] Sublayers created only when enabled
- [ ] CSS properly scoped in sublayers
- [ ] Build shows warnings for multiple envs
- [ ] Zero overhead when disabled
- [ ] Priority cascade works correctly
- [ ] JavaScript toggle functional
- [ ] Documentation emphasizes sparse usage
- [ ] Alternative approaches documented

## Decision Log References
- Environments via sublayers for clean control
- Should be used sparingly (complexity warning)
- Theme folders preferred for different clients

## Dependencies
- CSS Layer system (RS902) - COMPLETED ✅
- Namespace system (RS907) - COMPLETED ✅
- Configuration system (RS903) - COMPLETED ✅

## Notes
- **USE SPARINGLY** - This adds complexity
- Prefer separate theme folders for different clients
- Good for: debug, A/B tests, critical optimizations
- Bad for: general theming, client variations
- Always warn users about complexity

## Alternative Approach (Recommended)
Instead of environments, use theme folders:
```
themes/
├── default/
│   ├── reedstyle.colors.yaml
│   └── reedstyle.components.yaml
├── client-a/
│   ├── reedstyle.colors.yaml
│   └── reedstyle.components.yaml
└── dark/
    └── reedstyle.colors.yaml
```

Build with:
```bash
cargo run --theme=client-a
```

This keeps HTML clean and avoids environment attributes.

## Warning Messages
Add to build output:
```
⚠️  Environment System Warning:
   You have enabled 2 environments (dev, staging)
   
   Environments add significant complexity to your CSS.
   Consider these alternatives:
   
   1. For different themes: Use separate theme folders
   2. For client variations: Build separate CSS files
   3. For dark mode: Use CSS custom properties
   
   Only use environments for:
   - Debug helpers in development
   - A/B testing specific elements
   - Critical production optimizations
   
   See docs/develop/601-environments.md for details.
```