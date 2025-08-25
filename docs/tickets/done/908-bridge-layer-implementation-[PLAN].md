# RS908 Implementation Plan

## Ticket: Bridge Layer Implementation

### Critical Files to Read (IN THIS ORDER!)

#### 1. Requirements
- `/docs/tickets/908-bridge-layer-implementation.md` - Ticket specification

#### 2. Architecture Understanding
- `/docs/develop/201-architecture.md` - CSS Layer system (Layer 2: Bridge)
- `/docs/develop/501-migration.md` - Migration guide with bridge examples

#### 3. Current Implementation
- `/src/css/mod.rs` - CSS generation and layer structure
- `/src/main.rs` - Main build orchestration
- `/src/builder/mod.rs` - Build process

#### 4. Configuration System
- `/src/config/mod.rs` - Configuration structures
- `/reedstyle.config.yaml` - Main config example

### Critical Implementation Rules

#### Layer Order (MUST MAINTAIN)
```css
@layer settings, bridge, theme, free;
```

#### Bridge Sublayers Pattern
```css
@layer bridge {
  @layer bootstrap {
    /* Import + overrides */
  }
  @layer tailwind {
    /* Import + overrides */
  }
}
```

#### Configuration Structure
```yaml
bridge:
  framework_name:
    enabled: true/false
    path: "./path/to/css"
    overrides: |
      /* CSS overrides */
```

### Implementation Steps

1. **Update Configuration System**
   - Add `BridgeConfig` struct to `/src/config/mod.rs`
   - Support loading from `reedstyle.bridge.yaml`
   - Handle optional file (no bridge if missing)

2. **Implement Bridge Layer Generation**
   - Modify `/src/css/mod.rs` to generate bridge layer
   - Only create sublayers for enabled frameworks
   - Use @import for external CSS files
   - Apply overrides after imports

3. **Build System Integration**
   - Update `/src/main.rs` to load bridge config
   - Pass bridge config to CSS generator
   - Report enabled/disabled frameworks
   - Show file sizes for imported CSS

4. **Testing**
   - Create test fixtures with sample CSS
   - Test enable/disable toggle
   - Verify cascade order
   - Check no overhead when disabled

### Testing Checklist

- [ ] Bridge config loads from YAML
- [ ] Sublayers only created when enabled
- [ ] @import statements generated correctly
- [ ] Toggle on/off works
- [ ] Build output shows status
- [ ] Cascade order: settings → bridge → theme → free
- [ ] No CSS when all disabled

### Decision References
- CSS Layer architecture established
- Bridge layer for migration support
- Zero overhead when disabled

### Session Recovery Note
If session is lost, this file contains ALL context needed to continue RS908.
Start by reading this file, then the referenced documentation in order.
Key points:
- Bridge layer enables migration from other frameworks
- Uses @import to include external CSS
- Sublayers isolate each framework
- Toggle on/off via YAML config