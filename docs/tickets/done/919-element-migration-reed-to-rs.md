# Ticket #919: Element Name Migration (reed → r-s)

## Status: ✅ Completed

## Priority: 🔴 Critical

## Decision Log References
- DEC017 - Change from reed to r-s for W3C validator compliance

## Description
Migrate all ReedSTYLE elements from `<reed>` to `<r-s>` to achieve W3C validator compliance while maintaining framework functionality. The element `r-s` is valid for custom elements (has hyphen) but won't be registered as a Web Component - it will work as an unknown element styled with CSS.

## Requirements

### Functional Requirements
- [x] Replace all `reed` references with `r-s` in CSS generation
- [x] Update all JavaScript selectors from `reed` to `r-s`
- [x] Migrate all test files to use `r-s` elements
- [x] Update documentation to reflect new element name
- [x] Ensure all effects and namespaces work with `r-s`

### Technical Requirements
- [x] No Web Component registration (r-s remains unknown element)
- [x] Maintain all existing functionality
- [x] CSS file size should not increase significantly
- [x] JavaScript continues to work as progressive enhancement

## Implementation Plan

### Phase 1: CSS Generation
1. Update all namespace modules to generate `r-s[` selectors
   - `/src/css/namespaces/box.rs`
   - `/src/css/namespaces/face.rs`
   - `/src/css/namespaces/text.rs`
   - `/src/css/namespaces/layout.rs`
   - `/src/css/namespaces/device.rs`
   - `/src/css/namespaces/fx.rs`
2. Update defaults module
   - `/src/css/defaults.rs`

### Phase 2: JavaScript Updates
1. Update all selectors in `/src/js/mod.rs`
   - Change `querySelectorAll('reed[`)
   - Update element checks

### Phase 3: Test Files Migration
1. Update all test HTML files
   - `/test/test-element-defaults.html`
   - `/test/test-effects.html`
   - Any other test files

### Phase 4: Documentation
1. Update all documentation references
2. Update examples in tickets
3. Update CLAUDE.md if needed

## Migration Examples

### Before
```html
<reed as="card" box="[padding:4]" face="[bg:brand-a]">
  <reed as="h1">Title</reed>
  <reed as="p">Content</reed>
</reed>
```

### After
```html
<r-s as="card" box="[padding:4]" face="[bg:brand-a]">
  <r-s as="h1">Title</r-s>
  <r-s as="p">Content</r-s>
</r-s>
```

### CSS Before
```css
reed[as="h1"] { font-size: 2em; }
reed[box*="padding:4"] { padding: 1rem; }
```

### CSS After
```css
r-s[as="h1"] { font-size: 2em; }
r-s[box*="padding:4"] { padding: 1rem; }
```

## Acceptance Criteria
- [ ] W3C validator accepts `r-s` elements
- [ ] All namespaces work with `r-s`
- [ ] All effects work with `r-s`
- [ ] Test pages render correctly
- [ ] No regression in functionality
- [ ] JavaScript effects continue to work
- [ ] CSS file size increase < 5%

## Testing Checklist
- [ ] Run cargo build successfully
- [ ] Generate CSS/JS files
- [ ] Test all namespace attributes
- [ ] Test hover effects
- [ ] Test click effects (ripple, etc.)
- [ ] Test scroll animations
- [ ] Validate HTML with W3C validator
- [ ] Check browser console for errors

## Dependencies
- Requires: Completed RS917 (Effects System)
- Blocks: Future development

## Verification Steps
1. [ ] All `reed` replaced with `r-s`
2. [ ] No hardcoded `reed` strings remain
3. [ ] Test pages work identically
4. [ ] W3C validator passes

## Notes
- This is a breaking change for any existing implementations
- Consider providing migration script for users
- Element remains unregistered (not a Web Component)
- Browser treats `r-s` as unknown element like `div`