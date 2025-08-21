# Ticket #927: CDN Distribution

## Status: 📋 Planned

## Overview
Set up CDN distribution for ReedSTYLE via jsDelivr, unpkg, and CDN services. Enable users to include ReedSTYLE directly from CDN without downloading or installing.

## Files to Read First (Understanding Current System)
Before implementing, read these files to understand the distribution system:

1. **Current Distribution**:
   - `/src/builder/mod.rs` - How dist files are generated
   - `/dist/` - Current distribution files
   - `/package.json` - NPM package configuration

2. **CI/CD System**:
   - `/.github/workflows/build.yml` - Current build workflow
   - `/.github/workflows/release.yml` - Release workflow
   - `/docs/tickets/done/942-ci-cd-pipeline-[PLAN].md` - CI/CD implementation

3. **Version Management**:
   - `/src/builder/mod.rs` - VERSION constant
   - `/Cargo.toml` - Package version

## Objectives
- Publish to NPM for CDN availability
- Configure jsDelivr and unpkg optimization
- Set up CDN-specific builds
- Provide versioned and latest URLs
- Enable subresource integrity (SRI)
- Document CDN usage patterns

## Technical Requirements

### 1. NPM Package Structure
Prepare package.json for NPM:
```json
{
  "name": "reedstyle",
  "version": "1.0.0",
  "description": "Semantic HTML styling system",
  "main": "dist/reedstyle.js",
  "style": "dist/reedstyle.css",
  "files": [
    "dist/",
    "LICENSE",
    "README.md"
  ],
  "keywords": [
    "css",
    "framework",
    "reedstyle",
    "semantic",
    "html"
  ],
  "homepage": "https://reedstyle.dev",
  "repository": {
    "type": "git",
    "url": "https://github.com/byvoss/reedstyle.git"
  },
  "license": "Apache-2.0",
  "author": "ByVoss Technologies",
  "publishConfig": {
    "access": "public"
  }
}
```

### 2. CDN URLs Structure
After NPM publish, available at:

#### jsDelivr
```html
<!-- Latest version -->
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/reedstyle/dist/reedstyle.min.css">
<script src="https://cdn.jsdelivr.net/npm/reedstyle/dist/reedstyle.min.js"></script>

<!-- Specific version -->
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/reedstyle@1.0.0/dist/reedstyle.min.css">

<!-- Version range -->
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/reedstyle@^1/dist/reedstyle.min.css">
```

#### unpkg
```html
<!-- Latest -->
<link rel="stylesheet" href="https://unpkg.com/reedstyle/dist/reedstyle.min.css">

<!-- Specific version -->
<link rel="stylesheet" href="https://unpkg.com/reedstyle@1.0.0/dist/reedstyle.min.css">
```

### 3. Subresource Integrity (SRI)
Generate SRI hashes for security:
```rust
// src/builder/sri.rs
use sha2::{Sha384, Digest};
use base64::{engine::general_purpose, Engine};

pub fn generate_sri(content: &str) -> String {
    let mut hasher = Sha384::new();
    hasher.update(content.as_bytes());
    let hash = hasher.finalize();
    let b64 = general_purpose::STANDARD.encode(&hash);
    format!("sha384-{}", b64)
}

// Generate during build
let css_sri = generate_sri(&css_content);
let js_sri = generate_sri(&js_content);

// Output to sri.json
let sri_data = json!({
    "css": css_sri,
    "js": js_sri,
    "version": VERSION
});
```

Usage with SRI:
```html
<link rel="stylesheet" 
      href="https://cdn.jsdelivr.net/npm/reedstyle@1.0.0/dist/reedstyle.min.css"
      integrity="sha384-oqVuAfXRKap7fdgcCY5uykM6+R9GKQ8K/31aQ5L3v5Y="
      crossorigin="anonymous">
```

### 4. CDN Optimization
Create CDN-specific builds:
```rust
// Optimizations for CDN
pub fn build_for_cdn() {
    // 1. Maximum compression
    let css = minify_css(&css, MinifyOptions {
        level: OptimizationLevel::Three,
    });
    
    // 2. Remove source maps (separate file)
    // 3. Add cache headers hints in comments
    css.insert_str(0, "/*! Cache-Control: public, max-age=31536000 */\n");
    
    // 4. Optimize for gzip/brotli
    // Pre-compress files
    compress_gzip(&css)?;
    compress_brotli(&css)?;
}
```

### 5. Version Management
Automated version bumping:
```yaml
# .github/workflows/release.yml
name: Release to NPM
on:
  push:
    tags:
      - 'v*'

jobs:
  publish:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node
        uses: actions/setup-node@v4
        with:
          node-version: '18'
          registry-url: 'https://registry.npmjs.org'
      
      - name: Build
        run: cargo run --release
      
      - name: Generate SRI
        run: cargo run --bin generate-sri
      
      - name: Publish to NPM
        run: npm publish
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
      
      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            dist/*.css
            dist/*.js
            dist/sri.json
```

### 6. CDN Fallback Pattern
Document fallback pattern:
```html
<!-- Primary CDN with fallback -->
<link rel="stylesheet" 
      href="https://cdn.jsdelivr.net/npm/reedstyle@1/dist/reedstyle.min.css"
      onerror="this.onerror=null;this.href='https://unpkg.com/reedstyle@1/dist/reedstyle.min.css';">

<!-- JavaScript fallback -->
<script src="https://cdn.jsdelivr.net/npm/reedstyle@1/dist/reedstyle.min.js"></script>
<script>
  window.ReedStyle || document.write('<script src="/fallback/reedstyle.min.js"><\/script>');
</script>
```

## Implementation Steps

### Phase 1: NPM Preparation
1. Update package.json with all metadata
2. Create .npmignore file
3. Add NPM scripts for validation
4. Test local NPM pack

### Phase 2: SRI Generation
1. Add SRI generation to build
2. Create sri.json output
3. Add SRI to documentation
4. Test integrity verification

### Phase 3: Release Automation
1. Update GitHub Actions workflow
2. Add NPM publish step
3. Configure NPM token secret
4. Test with beta release

### Phase 4: CDN Optimization
1. Add CDN-specific build options
2. Pre-compress files
3. Add cache headers
4. Optimize for CDN delivery

### Phase 5: Documentation
1. Create CDN usage guide
2. Add to quick start
3. Document version strategies
4. Add troubleshooting section

## Testing Scenarios

### Test 1: NPM Publish (Dry Run)
```bash
npm pack
npm publish --dry-run
# Verify package contents
```

### Test 2: CDN URLs
After publish, test:
- jsDelivr latest works
- unpkg latest works
- Specific version works
- SRI validation passes

### Test 3: Fallback Pattern
Test with network blocking:
- Primary CDN blocked
- Fallback loads
- Local fallback works

### Test 4: Performance
Measure CDN performance:
- Time to first byte
- Download time
- Gzip efficiency
- Cache headers

## Success Criteria
- [ ] NPM package published successfully
- [ ] jsDelivr serves files correctly
- [ ] unpkg serves files correctly
- [ ] SRI hashes generated and valid
- [ ] Version URLs work
- [ ] Latest URLs work
- [ ] Fallback pattern documented
- [ ] Cache headers optimal
- [ ] Documentation complete
- [ ] Quick start updated

## Decision Log References
- CDN distribution for easy adoption
- NPM as primary distribution
- SRI for security

## Dependencies
- Distribution files (RS941) - COMPLETED ✅
- CI/CD pipeline (RS942) - COMPLETED ✅
- Version management in build

## Notes
- jsDelivr is primary CDN (faster, more reliable)
- unpkg as secondary/fallback
- Consider Cloudflare CDN later
- SRI important for security
- Cache headers critical for performance

## CDN Benefits
- No installation required
- Automatic updates (with version ranges)
- Global edge caching
- Shared cache across sites
- Bandwidth savings

## Example Usage Documentation
```markdown
## Quick Start with CDN

### Latest Version (Auto-updates)
\```html
<!DOCTYPE html>
<html>
<head>
  <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/reedstyle/dist/reedstyle.min.css">
  <script src="https://cdn.jsdelivr.net/npm/reedstyle/dist/reedstyle.min.js" defer></script>
</head>
<body>
  <r-s as="hero">
    <h1>Welcome to ReedSTYLE</h1>
  </r-s>
</body>
</html>
\```

### Specific Version (Stable)
\```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/reedstyle@1.0.0/dist/reedstyle.min.css">
\```

### With SRI (Secure)
\```html
<link rel="stylesheet" 
      href="https://cdn.jsdelivr.net/npm/reedstyle@1.0.0/dist/reedstyle.min.css"
      integrity="sha384-[hash]"
      crossorigin="anonymous">
\```
```