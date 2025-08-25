# RS909 Implementation Plan

## Ticket: Lightning CSS Integration

### Critical Files to Read (IN THIS ORDER!)

#### 1. Requirements
- `/docs/tickets/909-lightning-css-integration.md` - Ticket specification

#### 2. Current Minification System
- `/src/builder/mod.rs` - Current basic minification implementation
- `/src/main.rs` - Build orchestration
- `/Cargo.toml` - Current dependencies (no Lightning CSS yet)

#### 3. CSS Generation
- `/src/css/mod.rs` - CSS generation that outputs to minifier
- `/src/lib.rs` - Build flow

### Critical Implementation Rules

#### OKLCH Colors (MUST PRESERVE!)
- Lightning CSS must NOT convert OKLCH colors
- `oklch(68.5% 0.24 25)` must stay exactly as-is
- Disable ALL color conversion features
- This is CRITICAL for our color system

#### Layer Structure
```css
@layer settings, bridge, theme, free;
```
Must be preserved exactly

#### Import Statements
- @import in bridge layer must NOT be inlined
- Keep external CSS references intact

### Implementation Steps

1. **Add Lightning CSS Dependency**
   - Add to Cargo.toml: `lightningcss = "1.0.0-alpha.57"` (latest)
   - Run `cargo build` to verify

2. **Create Optimizer Module**
   - Create `/src/optimizer/mod.rs`
   - Implement CSSOptimizer struct
   - Two modes: development (formatted) and production (minified)

3. **Replace Current Minification**
   - Update `/src/builder/mod.rs`
   - Replace `minify_css()` function
   - Use Lightning CSS for both dev and prod

4. **Configure Lightning CSS**
   - Disable color conversion
   - Enable autoprefixing
   - Set browser targets
   - Configure minification options

5. **Testing Checklist**
   - Verify OKLCH unchanged
   - Check file sizes (dev ~350KB, prod <180KB)
   - Test autoprefixing works
   - Validate layer structure preserved
   - Ensure @import not inlined

### Key Configuration

```rust
MinifyOptions {
    targets: Browsers::from_browserlist(&["defaults"]),
    // CRITICAL: Don't transform colors!
    ..Default::default()
}

PrinterOptions {
    minify: true,
    // MAXIMUM minification:
    inline_style_blocks: true,
    targets: browsers,
    // But preserve OKLCH colors!
}
```

### Maximum Minification Features
- Remove ALL whitespace and line breaks
- Shorten color values where possible (but NOT OKLCH!)
- Optimize calc() expressions
- Merge duplicate rules
- Remove unused @keyframes
- Compress CSS variables
- Use shortest possible syntax

### Testing Checklist

- [ ] OKLCH colors unchanged (critical!)
- [ ] CSS layers preserved
- [ ] @import statements not inlined
- [ ] Autoprefixing works
- [ ] Dev build ~350KB formatted
- [ ] Prod build <180KB minified
- [ ] Build time <500ms
- [ ] No runtime errors

### Decision References
- DEC002: OKLCH at build time (must preserve)
- DEC004: Lightning CSS for optimization

### Session Recovery Note
If session is lost, this file contains ALL context needed to continue RS909.
Start by reading this file, then the referenced documentation in order.

Key points:
- Lightning CSS replaces basic minification
- MUST preserve OKLCH colors exactly
- Written in Rust - fits perfectly
- Alpha but stable for production use