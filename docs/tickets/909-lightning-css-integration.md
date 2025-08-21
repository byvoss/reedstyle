# Ticket #909: Lightning CSS Integration

## Status: 📋 Planned

## Overview
Integrate Lightning CSS (written in Rust) for CSS optimization, minification, and browser compatibility. Replace current basic minification with production-grade optimization while maintaining OKLCH color system and layer structure.

## Files to Read First (Understanding Current System)
Before implementing, read these files to understand the existing build process:

1. **Current Build System**:
   - `/src/builder/mod.rs` - Current file generation and basic minification
   - `/src/main.rs` - Main build orchestration
   - `/src/css/mod.rs` - CSS generation logic

2. **Dependencies & Configuration**:
   - `/Cargo.toml` - Check current dependencies (no Lightning CSS yet)
   - `/src/config/mod.rs` - Build configuration structures

3. **Documentation for Context**:
   - `/docs/develop/201-architecture.md` - Build system section
   - `/docs/tickets/done/941-distribution-strategy-[PLAN].md` - Mentions Lightning CSS plan

## Objectives
- Replace basic minification with Lightning CSS
- Maintain OKLCH color integrity (no conversion)
- Add autoprefixing for browser compatibility
- Optimize CSS variables and custom properties
- Generate source maps for development
- Achieve <180KB production CSS size

## Technical Requirements

### 1. Cargo Dependencies
Add to `Cargo.toml`:
```toml
[dependencies]
lightningcss = "1.0.0-alpha.50"  # Or latest alpha
```

### 2. Integration Points
Update `/src/builder/mod.rs`:
- Replace current minify_css function
- Use Lightning CSS for both dev and prod builds
- Configure different optimization levels
- Generate source maps when not minifying

### 3. Lightning CSS Configuration
```rust
use lightningcss::{
    stylesheet::{StyleSheet, ParserOptions, MinifyOptions},
    targets::Browsers,
    printer::PrinterOptions,
};

// Development config
let dev_options = ParserOptions {
    filename: "reedstyle.css",
    css_modules: false,
    source_index: 0,
};

// Production config with minification
let minify_options = MinifyOptions {
    targets: Browsers::from_browserlist(&[
        "last 2 versions",
        "> 1%",
        "not dead"
    ]),
    unused_symbols: HashSet::new(),
};
```

### 4. Color Preservation
CRITICAL: Lightning CSS must NOT convert OKLCH colors:
- Disable color conversion features
- Keep OKLCH values as-is
- Test that `oklch(68.5% 0.24 25)` stays unchanged

### 5. Features to Enable
- **Autoprefixing**: Add vendor prefixes automatically
- **Minification**: Remove whitespace, comments
- **Variable optimization**: Optimize CSS custom properties
- **Dead code elimination**: Remove unused @keyframes
- **Media query merging**: Combine duplicate queries

### 6. Features to Disable
- **Color conversion**: Keep OKLCH untouched
- **CSS Modules**: Not needed for ReedSTYLE
- **Import inlining**: Keep @import for bridge layer

## Implementation Steps

### Phase 1: Add Dependency
1. Update `Cargo.toml` with Lightning CSS
2. Run `cargo build` to fetch dependency
3. Verify no version conflicts

### Phase 2: Create Lightning Module
1. Create `/src/optimizer/mod.rs` for Lightning CSS logic
2. Implement optimize_css function for development
3. Implement minify_css function for production
4. Add source map generation option

### Phase 3: Integration
1. Update `/src/builder/mod.rs` to use new optimizer
2. Replace current minification logic
3. Add browser target configuration
4. Test with sample CSS

### Phase 4: Configuration
1. Add optimization settings to config
2. Allow custom browser targets
3. Configure source map generation
4. Add optimization level control

### Phase 5: Testing
1. Verify OKLCH colors unchanged
2. Check autoprefixing works
3. Measure file size reduction
4. Validate source maps
5. Test layer structure preserved

## Code Structure
```rust
// src/optimizer/mod.rs
use lightningcss::{stylesheet::*, targets::*, printer::*};

pub struct CSSOptimizer {
    browsers: Browsers,
    minify: bool,
    source_maps: bool,
}

impl CSSOptimizer {
    pub fn new(minify: bool) -> Self {
        Self {
            browsers: Browsers::from_browserlist(&["defaults"]).unwrap(),
            minify,
            source_maps: !minify,
        }
    }

    pub fn optimize(&self, css: &str) -> Result<String, Error> {
        let mut stylesheet = StyleSheet::parse(
            css,
            ParserOptions::default()
        )?;

        if self.minify {
            stylesheet.minify(MinifyOptions {
                targets: self.browsers.clone(),
                ..Default::default()
            })?;
        }

        let result = stylesheet.to_css(PrinterOptions {
            minify: self.minify,
            source_map: self.source_maps,
            targets: self.browsers.clone(),
            ..Default::default()
        })?;

        Ok(result.code)
    }
}
```

## Testing Scenarios

### Test 1: OKLCH Preservation
Input:
```css
:root {
  --color-brand-a: oklch(68.5% 0.24 25);
}
```
Output must keep OKLCH format unchanged.

### Test 2: Autoprefixing
Input:
```css
.test {
  user-select: none;
  backdrop-filter: blur(10px);
}
```
Output should have -webkit- and -moz- prefixes.

### Test 3: Minification
- Development: ~350KB with comments and formatting
- Production: <180KB minified
- Verify readability in dev, size in prod

### Test 4: Layer Preservation
Ensure @layer structure remains:
```css
@layer settings, bridge, theme, free;
```

## Success Criteria
- [ ] Lightning CSS integrated successfully
- [ ] OKLCH colors preserved exactly
- [ ] Autoprefixing works for target browsers
- [ ] Development CSS ~350KB with formatting
- [ ] Production CSS <180KB minified
- [ ] Source maps generated for development
- [ ] CSS layers structure maintained
- [ ] Build time remains <500ms
- [ ] No color format conversions

## Decision Log References
- DEC002: OKLCH conversion at build time (must preserve)
- DEC004: Lightning CSS for minification (implementing now)

## Dependencies
- Requires working build system (RS906) - COMPLETED ✅
- Works with distribution strategy (RS941) - COMPLETED ✅

## Notes
- Lightning CSS is alpha but stable enough for production
- Written in Rust - perfect fit for our build system
- Much faster than PostCSS/cssnano
- Preserves OKLCH with correct configuration
- Can replace multiple PostCSS plugins

## Potential Issues
- Alpha API may change
- Need to ensure OKLCH preservation
- Source map generation adds complexity
- Browser target configuration important