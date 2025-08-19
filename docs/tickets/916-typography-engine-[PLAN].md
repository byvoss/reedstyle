# RS916 Typography Engine - Implementation Plan

## Session Recovery
If session is lost, start here:
1. Check branch: `git branch` (should be feature/RS916-typography-engine)
2. Check stash: `git stash list` (RS916 typography code may be stashed)
3. Read this plan file first
4. Continue from last completed step

## Files to Read (in order)
1. `/docs/tickets/916-typography-engine.md` - Main ticket specification
2. `/docs/standards/102-ReedSTYLE-Box-Namespace.md` - Box namespace for spacing
3. `/docs/standards/107-ReedSTYLE-Text-Namespace.md` - Text namespace integration
4. `/src/css/namespaces/text.rs` - Existing text namespace implementation
5. `/CLAUDE.md` - Development guidelines

## Critical Rules from Specification

### Typography Filters
- `minimal` - Basic corrections only
- `smart` - Smart quotes, dashes, ellipsis  
- `professional` - All enhancements including hyphenation

### Language Support Priority
1. German (DIN 5008 compliant) - CRITICAL
2. English (US & GB variants)
3. French (guillemets and spacing)

### DIN 5008 Requirements (German)
- Quotes: „primary" and ‚secondary'
- Narrow no-break space (U+202F) before units
- Non-breaking space (U+00A0) for abbreviations
- Number ranges with en dash (–)
- Thousands separator with narrow no-break space

### CSS Features (No JS required)
- OpenType features (ligatures, small-caps, etc.)
- Hyphenation control
- Line height (leading) variants
- Measure (line length) control

## Implementation Steps

### Phase 1: Rust Typography Module
1. Create `/src/typography/mod.rs` - Main module structure ✓ (stashed)
2. Create `/src/typography/german.rs` - DIN 5008 rules ✓ (stashed)
3. Create `/src/typography/english.rs` - US/GB rules ✓ (stashed)
4. Create `/src/typography/french.rs` - French rules
5. Add typography CSS generation to text namespace

### Phase 2: CSS OpenType Features
1. Add to `/src/css/namespaces/text.rs`:
   - Ligatures control
   - Small caps
   - Number variants (tabular, oldstyle)
   - Hyphenation settings
   - Leading control
   - Measure control

### Phase 3: JavaScript Engine
1. Create `/dist/reedstyle-typography.js`
2. Language detection from DOM
3. Text filter processing
4. Smart quote replacement
5. DIN 5008 compliance for German

### Phase 4: Testing
1. Create `/test/test-typography.html`
2. Test German DIN 5008
3. Test English US/GB quotes
4. Test French guillemets
5. Test OpenType features

## Testing Checklist
- [ ] German quotes convert correctly
- [ ] German abbreviations get NBSP
- [ ] German numbers format per DIN 5008
- [ ] English US uses double quotes primarily
- [ ] English GB uses single quotes primarily
- [ ] French uses guillemets with spaces
- [ ] OpenType features work without JS
- [ ] Language detection works from lang attribute
- [ ] Filter levels apply correctly

## Decision References
- DEC003: System fonts as defaults
- DEC007: OKLCH color system (for text colors)

## Notes
- Typography code already started and stashed
- Retrieve with: `git stash pop` (when ready)
- CSS features must work without JavaScript
- JavaScript adds progressive enhancement only