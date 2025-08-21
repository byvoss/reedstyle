# Ticket #908: Bridge Layer Implementation

## Status: 📋 Planned

## Overview
Implement the Bridge Layer system (`@layer bridge`) for seamless third-party CSS framework integration. This layer enables gradual migration from Bootstrap, Tailwind, Material UI, and other frameworks by creating isolated sublayers that can be toggled on/off.

## Files to Read First (Understanding Current System)
Before implementing, read these files to understand the existing architecture:

1. **Core Architecture & Layer System**:
   - `/src/css/mod.rs` - Current CSS generation and layer structure
   - `/src/main.rs` - Main build system that orchestrates CSS generation
   - `/src/builder/mod.rs` - Build process and file generation

2. **Configuration System**:
   - `/src/config/mod.rs` - Configuration loading and structures
   - `/reedstyle.config.yaml` - Main configuration example

3. **Documentation for Context**:
   - `/docs/develop/201-architecture.md` - CSS Layer system documentation (Layer 2: Bridge)
   - `/docs/develop/501-migration.md` - Complete migration guide with bridge examples

## Objectives
- Create configurable bridge sublayers for third-party CSS frameworks
- Enable toggle on/off functionality for each framework during migration
- Import existing CSS files into appropriate sublayers
- Support gradual migration without breaking existing styles
- Zero overhead when frameworks are disabled

## Technical Requirements

### 1. Configuration Structure
Create `reedstyle.bridge.yaml` support:
```yaml
bridge:
  bootstrap:
    enabled: true/false
    version: 5.3
    path: "./node_modules/bootstrap/dist/css/bootstrap.min.css"
    overrides: |
      /* Custom overrides */
  
  tailwind:
    enabled: true/false
    path: "./dist/tailwind-output.css"
    
  custom:
    enabled: true/false
    name: "legacy-styles"
    path: "./legacy/old-styles.css"
```

### 2. CSS Layer Generation
Generate bridge sublayers in correct cascade order:
```css
@layer settings, bridge, theme, free;

@layer bridge {
  /* Only enabled frameworks appear here */
  @layer bootstrap {
    @import "./node_modules/bootstrap/dist/css/bootstrap.min.css";
    /* Overrides */
  }
  
  @layer tailwind {
    @import "./dist/tailwind-output.css";
  }
}
```

### 3. Implementation in Rust
Update `/src/css/mod.rs` to:
- Load bridge configuration
- Generate sublayers only for enabled frameworks
- Import CSS files into sublayers
- Apply overrides after imports
- Show enabled/disabled status during build

### 4. Build System Integration
- Read `reedstyle.bridge.yaml` if exists
- Process each integration based on enabled flag
- Import CSS files when enabled
- Skip entirely when disabled (no sublayer created)
- Report file sizes for enabled frameworks

## Implementation Steps

### Phase 1: Configuration Loading
1. Create bridge configuration structure in `/src/config/mod.rs`
2. Add bridge field to main Config struct
3. Load `reedstyle.bridge.yaml` if present
4. Default to no bridge if file missing

### Phase 2: CSS Generation
1. Update `/src/css/mod.rs` to generate bridge layer
2. Create sublayers only for enabled frameworks
3. Generate @import statements for CSS files
4. Add override rules after imports
5. Ensure proper layer ordering

### Phase 3: Build Output
1. Show which frameworks are enabled/disabled
2. Report total CSS size including imported frameworks
3. Validate imported files exist
4. Handle missing files gracefully

### Phase 4: Testing
1. Create test configuration with Bootstrap enabled
2. Verify CSS imports work correctly
3. Test toggle on/off functionality
4. Confirm no overhead when disabled

## Testing Scenarios

### Test 1: Bootstrap Migration
```yaml
bridge:
  bootstrap:
    enabled: true
    path: "./test/fixtures/bootstrap.css"
```
- Verify Bootstrap CSS is imported
- Test that ReedSTYLE wins cascade conflicts
- Toggle to disabled and verify no Bootstrap CSS

### Test 2: Multiple Frameworks
Enable Bootstrap and Tailwind simultaneously:
- Both sublayers should be created
- Order should be maintained
- Each can be toggled independently

### Test 3: Custom Legacy CSS
```yaml
bridge:
  custom:
    enabled: true
    name: "old-styles"
    path: "./legacy.css"
```
- Custom sublayer name works
- Legacy CSS properly imported

## Success Criteria
- [ ] Bridge configuration loads from YAML
- [ ] Sublayers only created when enabled
- [ ] CSS files properly imported via @import
- [ ] Toggle on/off works without errors
- [ ] Build shows enabled/disabled status
- [ ] No performance impact when disabled
- [ ] Cascade order correct (bridge before theme)
- [ ] Migration from other frameworks possible

## Decision Log References
- CSS Layer system architecture (see `/docs/develop/201-architecture.md`)
- Migration strategy (see `/docs/develop/501-migration.md`)

## Dependencies
- Requires RS902 (CSS Layer System) - COMPLETED ✅
- Works with RS903 (YAML Configuration) - COMPLETED ✅

## Notes
- Bridge layer is temporary for migration
- Should be disabled in production builds
- Each framework gets isolated sublayer
- @import allows external CSS without copying
- This enables gradual migration strategy

## Example Usage
```bash
# Enable Bootstrap during migration
echo "bridge:
  bootstrap:
    enabled: true
    path: './node_modules/bootstrap/dist/css/bootstrap.min.css'" > reedstyle.bridge.yaml

# Build with bridge
cargo run

# Output shows:
# ✓ Bridge layer 'bootstrap' enabled (342KB)
# ✗ Bridge layer 'tailwind' disabled

# Later, disable Bootstrap
# Set enabled: false in YAML
# Rebuild - Bootstrap CSS not included
```