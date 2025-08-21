# Ticket #917: Effects System - IMPLEMENTATION PLAN

## Status: 🚧 In Progress

## Branch: `feature/rs917-effects-system`

## Critical Session Recovery Information

### Files to Read (IN ORDER)
1. `/CLAUDE.md` - Development guidelines and rules
2. `/docs/tickets/900-roadmap-index.md` - Ticket system overview
3. `/docs/tickets/900-workflow-process.md` - Development workflow
4. `/docs/tickets/917-effects-system.md` - This ticket specification
5. `/decisions.csv` - Technical decisions log
6. `/src/css/namespaces/fx.rs` - Current FX namespace implementation
7. `/src/css/namespaces/mod.rs` - Namespace module integration
8. `/src/js/mod.rs` - JavaScript generation module
9. `/test/test-element-defaults.html` - Example test page structure

### Critical Rules from Documentation
- **EVERY commit MUST reference ticket**: `feat(RS917): Description`
- **Update decisions.csv BEFORE coding** any architectural changes
- **Effects are progressive enhancement** - CSS base, JS extras
- **Use GPU-accelerated properties** (transform, opacity)
- **Respect prefers-reduced-motion** always
- **Keep animations under 300ms** for responsiveness
- **Shadow levels follow Visual scope**: weak→light→normal→intense→bright→strong
- **Duration follows Dimension scope**: tiny→small→normal→large→huge→mega→ultra

### Decision Log References
- DEC002 - Color Conversion (effects use CSS variables)
- DEC007 - ALL colors internally as OKLCH

## Implementation Approach

### Phase 1: CSS Effects Enhancement ✅
Enhance the existing fx.rs module with comprehensive effect definitions.

#### 1.1 Hover Effects
```rust
// In fx.rs - Add comprehensive hover effects
- lift (translateY + shadow)
- sink (translateY down)
- grow (scale up)
- shrink (scale down)
- glow (box-shadow with color)
- blur (filter blur)
- rotate (small rotation)
- skew (skew transform)
- flip (3D rotation)
```

#### 1.2 Click/Active Effects
```rust
// Active state effects
- scale (scale down on click)
- brightness (darken/lighten)
- pulse (animation)
- bounce (animation)
- flash (brightness flash)
```

#### 1.3 Scroll Animations
```rust
// Keyframe animations for scroll
- fade-in/out
- slide-up/down/left/right
- zoom-in/out
- reveal (clip-path)
- parallax (transform3d)
```

#### 1.4 Shadow & Duration Scales
```rust
// Visual scope for shadows
shadow: weak|light|normal|intense|bright|strong

// Dimension scope for durations
duration: tiny|small|normal|large|huge|mega|ultra
```

### Phase 2: JavaScript Effects Engine ⏳

#### 2.1 Core EffectsEngine Class
```javascript
class EffectsEngine {
  - initClickEffects() // Ripple, pulse
  - initScrollEffects() // IntersectionObserver
  - initHoverEnhancements() // Advanced hover
  - createRipple() // Ripple animation
  - applyStagger() // Staggered animations
}
```

#### 2.2 Scroll Observer System
```javascript
// IntersectionObserver for scroll triggers
- Threshold control (0-1)
- Stagger delays for groups
- Repeat options
- Performance optimization
```

#### 2.3 Dynamic CSS Injection
```javascript
// Runtime CSS for effects
- Ripple keyframes
- Dynamic animations
- Custom properties
```

### Phase 3: Integration & Testing 📝

#### 3.1 Build System Updates
- Ensure fx namespace is properly called in namespaces/mod.rs
- Update JS generation in js/mod.rs

#### 3.2 Test Page Creation
- Create comprehensive test-effects.html
- Test all effect categories
- Verify progressive enhancement
- Check reduced motion support

### Phase 4: Documentation 📚
- Update ticket status
- Add usage examples
- Document API

## File Changes Overview

```
src/
├── css/
│   └── namespaces/
│       └── fx.rs (ENHANCE)
├── js/
│   └── mod.rs (ENHANCE)
test/
└── test-effects.html (CREATE)
docs/
└── tickets/
    └── 917-effects-system.md (UPDATE status)
```

## Implementation Checklist

### CSS Effects (fx.rs)
- [ ] Enhanced hover effects
- [ ] Click/active effects  
- [ ] Scroll animation keyframes
- [ ] Shadow levels (Visual scope)
- [ ] Duration controls (Dimension scope)
- [ ] Glass/frost effects
- [ ] Reduced motion support
- [ ] GPU acceleration hints

### JavaScript (js/mod.rs)
- [ ] EffectsEngine class structure
- [ ] Click effect handlers
- [ ] Scroll observer system
- [ ] Stagger support
- [ ] Dynamic CSS injection
- [ ] Event delegation
- [ ] Performance optimizations

### Testing
- [ ] test-effects.html page
- [ ] Hover effects validation
- [ ] Click effects validation
- [ ] Scroll animations validation
- [ ] Combined effects testing
- [ ] Reduced motion testing
- [ ] Cross-browser testing

### Documentation
- [ ] Update ticket status
- [ ] Add code examples
- [ ] Document parameters
- [ ] Performance notes

## Technical Decisions

1. **Progressive Enhancement**: CSS provides base functionality, JS adds enhancements
2. **Performance First**: Use transform/opacity for animations (GPU accelerated)
3. **Accessibility**: Always respect prefers-reduced-motion
4. **Modularity**: Effects can be combined via space-separated values in fx attribute
5. **Consistency**: Follow existing scope patterns (Visual, Dimension)

## Example Usage

```html
<!-- Simple hover effect -->
<reed as="button" fx="[hover:lift]">Hover me</reed>

<!-- Combined effects -->
<reed as="card" fx="[hover:lift, click:ripple, shadow:normal]">
  Interactive Card
</reed>

<!-- Scroll animation with options -->
<reed as="div" fx="[scroll:fade-in, threshold:0.3, duration:normal, stagger:tiny]">
  Animated content
</reed>

<!-- Glass morphism -->
<reed as="modal" fx="[blur:glass, shadow:strong]">
  Modern glass effect
</reed>
```

## Testing Commands

```bash
# Build the system
cargo build

# Run to generate files
cargo run

# Open test page
open test/test-effects.html
```

## Notes

- Keep animations under 300ms for perceived responsiveness
- Use will-change sparingly to avoid memory issues
- Debounce/throttle scroll events for performance
- Test on low-end devices for performance validation