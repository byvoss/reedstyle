# Ticket #919: Element Migration (reed → r-s) - IMPLEMENTATION PLAN

## Status: 🚧 In Progress

## Branch: `feature/rs919-element-migration`

## Critical Session Recovery Information

### Files to Read (IN ORDER)
1. `/CLAUDE.md` - Development guidelines
2. `/docs/tickets/919-element-migration-reed-to-rs.md` - This ticket specification
3. `/decisions.csv` - Check DEC017
4. `/src/css/namespaces/box.rs` - Current namespace implementation
5. `/src/css/namespaces/face.rs`
6. `/src/css/namespaces/text.rs`
7. `/src/css/namespaces/layout.rs`
8. `/src/css/namespaces/device.rs`
9. `/src/css/namespaces/fx.rs`
10. `/src/css/defaults.rs` - HTML element defaults
11. `/src/js/mod.rs` - JavaScript generation
12. `/test/test-element-defaults.html` - Test file to migrate
13. `/test/test-effects.html` - Test file to migrate

### Critical Rules from Documentation
- **EVERY commit MUST reference ticket**: `feat(RS919): Description`
- **r-s is NOT a Web Component** - just an unknown element
- **No registration needed** - works purely with CSS
- **W3C valid name** - has required hyphen
- **Breaking change** - all existing code must be updated

### Decision Log References
- DEC017 - Change from reed to r-s for W3C validator compliance

## Systematic Migration Approach

### Phase 1: Rust CSS Generation (Core)
Replace all `reed` with `r-s` in CSS generation:

#### 1.1 Namespace Modules
```rust
// Change all instances of:
css.push_str("  reed[
// To:
css.push_str("  r-s[
```

Files to update:
- [ ] `/src/css/namespaces/box.rs`
- [ ] `/src/css/namespaces/face.rs`
- [ ] `/src/css/namespaces/text.rs`
- [ ] `/src/css/namespaces/layout.rs`
- [ ] `/src/css/namespaces/device.rs`
- [ ] `/src/css/namespaces/fx.rs`

#### 1.2 Defaults Module
```rust
// In /src/css/defaults.rs
// Change:
css.push_str("  reed {
css.push_str("  reed[as=
// To:
css.push_str("  r-s {
css.push_str("  r-s[as=
```

- [ ] `/src/css/defaults.rs`

### Phase 2: JavaScript Updates
Replace all `reed` selectors with `r-s`:

```javascript
// Change:
document.querySelectorAll('reed[fx*="click:"]')
// To:
document.querySelectorAll('r-s[fx*="click:"]')
```

- [ ] `/src/js/mod.rs`

### Phase 3: Test Files Migration
Update all HTML test files:

```html
<!-- Change all: -->
<reed as="...">
</reed>
<!-- To: -->
<r-s as="...">
</r-s>
```

- [ ] `/test/test-element-defaults.html`
- [ ] `/test/test-effects.html`
- [ ] Any other test files found

### Phase 4: Documentation Updates
- [ ] Update examples in ticket files
- [ ] Check CLAUDE.md for reed references
- [ ] Update README if exists

## Search & Replace Strategy

### Global Search Patterns
1. **CSS Generation**: `"reed[` → `"r-s[`
2. **CSS Strings**: `reed {` → `r-s {`
3. **JavaScript**: `'reed` → `'r-s`
4. **HTML**: `<reed` → `<r-s`
5. **HTML Close**: `</reed>` → `</r-s>`

### Verification Grep Commands
```bash
# Find remaining reed references
grep -r "reed\[" src/
grep -r "<reed" test/
grep -r "'reed" src/js/
grep -r "\"reed" src/

# Verify r-s is everywhere
grep -r "r-s\[" src/
grep -r "<r-s" test/
```

## Testing Checklist

### Build & Generate
- [ ] `cargo build` - No errors
- [ ] `cargo run` - Generates CSS/JS
- [ ] Check generated CSS has `r-s[` selectors

### Visual Testing
- [ ] Open test-element-defaults.html
- [ ] Open test-effects.html
- [ ] Verify styling works
- [ ] Test hover effects
- [ ] Test click effects
- [ ] Test scroll animations

### Validation
- [ ] Run HTML through W3C validator
- [ ] No "invalid element" errors for r-s
- [ ] Check browser console - no errors

## Commit Strategy

```bash
# After each phase
git add -A
git commit -m "feat(RS919): Migrate [component] from reed to r-s"

# Final squash
git reset --soft main
git commit -m "feat(RS919): Complete element migration from reed to r-s

- Updated all CSS namespace generators
- Migrated JavaScript selectors
- Converted all test files
- Updated documentation

Decisions:
- Change from reed to r-s for W3C compliance (DEC017)

Closes #919"
```

## Known Issues to Watch
1. **Case sensitivity** - Ensure all replacements maintain case
2. **String literals** - Don't replace "reed" in comments/docs unintentionally
3. **Component names** - Only element name changes, not component names
4. **Attribute selectors** - Ensure `[as=` still works

## Rollback Plan
If issues occur:
```bash
git checkout main
git branch -D feature/rs919-element-migration
```

## Success Criteria
- Zero references to `<reed` in test files
- Zero references to `reed[` in CSS generation
- W3C validator passes
- All tests still work
- Effects system fully functional