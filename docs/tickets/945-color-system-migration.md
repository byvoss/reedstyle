# Ticket #945: Color System Migration to 1-9 Scale

## Status: 🚧 In Progress
**Created**: 2025-08-21  
**Priority**: ⭐⭐⭐ CRITICAL (Blocks #911)

## Description

Migrate from the weak/light/intense/bright/strong color variation system to a unified 1-9 numerical scale for all colors (brand, neutral, state).

## Decisions

- DEC020: Unified 1-9 scale for all colors
- DEC021: Each brand color generates 9 variants automatically
- DEC022: Neutral colors use same 1-9 scale
- DEC023: Colors set via body data-root or JS

## Requirements

### Color Scale Definition
- [ ] 1 = Lightest (near white)
- [ ] 2-4 = Light variations
- [ ] 5 = Medium (often the base color)
- [ ] 6-8 = Dark variations
- [ ] 9 = Darkest (near black)

### Brand Colors (brand-a through brand-f)
- [ ] User defines single color (e.g., `brand-a: "#FF6B6B"`)
- [ ] System generates 9 variants (brand-a-1 through brand-a-9)
- [ ] Automatic OKLCH conversion
- [ ] Perceptually uniform lightness steps

### Neutral Colors
- [ ] neutral-1 = white
- [ ] neutral-2 through neutral-8 = gray scale
- [ ] neutral-9 = black
- [ ] Remove all base-100, base-200 etc. references

### State Colors
- [ ] success-1 through success-9
- [ ] warning-1 through warning-9
- [ ] error-1 through error-9
- [ ] info-1 through info-9

## Implementation Tasks

### 1. Rust/Build System
- [ ] Update color generation in `src/color/mod.rs`
- [ ] Remove weak/strong variant generation
- [ ] Implement 1-9 scale generation algorithm
- [ ] Update OKLCH conversion for perceptual uniformity

### 2. CSS Generation
- [ ] Update CSS variable naming (--rs-color-brand-a-1 etc.)
- [ ] Remove old variant CSS variables
- [ ] Update all namespace files that reference colors

### 3. JavaScript Updates
- [ ] Update color parsing to generate 1-9 scale
- [ ] Handle data-root attribute parsing
- [ ] Set CSS variables before CSS loads

### 4. Configuration Files
- [ ] Update reedstyle.colors.yaml documentation
- [ ] Remove variation settings (weak/strong definitions)
- [ ] Update examples to show new system

### 5. Documentation Updates
- [ ] Update all color references in docs/develop/
- [ ] Update configuration documentation
- [ ] Update examples with new color names
- [ ] Add migration guide for existing users

## Files to Update

### Source Code
- `src/color/mod.rs` - Core color system
- `src/css/namespaces/face.rs` - Background/border colors
- `src/css/namespaces/text.rs` - Text colors
- `src/css/namespaces/fx.rs` - Effect colors
- `src/config/defaults.rs` - Default color definitions
- `src/builder/mod.rs` - Build process

### Documentation
- `docs/develop/104-face.md` - Face namespace colors
- `docs/develop/101-namespaces-overview.md` - Color examples
- `docs/develop/401-configuration.md` - Configuration docs
- `docs/develop/011-quick-start.md` - Quick start examples
- All other docs with color references

### Configuration
- `reedstyle.colors.yaml` - Example configuration
- Test files with color usage

## Migration Strategy

1. **Phase 1**: Implement new system alongside old (backwards compatible)
2. **Phase 2**: Update all internal usage to new system
3. **Phase 3**: Remove old system completely
4. **Phase 4**: Update documentation

## Testing Requirements

- [ ] Color generation produces correct OKLCH values
- [ ] 1-9 scale is perceptually uniform
- [ ] JavaScript correctly sets CSS variables
- [ ] Backwards compatibility during migration
- [ ] All namespaces work with new colors

## Success Criteria

- Consistent 1-9 scale across all color types
- No more weak/strong/intense terminology
- Perceptually uniform color scales
- Simple user configuration (one color → 9 variants)
- All tests passing

## Dependencies

None - this is foundational

## Blocks

- #911 - Component Preset System (needs new color system)

## Notes

This is a breaking change but necessary for consistency. The 1-9 scale is more intuitive and aligns with industry standards (Tailwind, Material Design).

## Implementation Log

### 2025-08-21 - Planning
- Created ticket based on discussion
- Defined 1-9 scale system
- Identified all files needing updates