# RS943: Typography Test Fixes - Implementation Plan

## Session Recovery Instructions
If session is lost, resume with:
1. Read this plan file
2. Check git branch: `git branch --show-current`
3. Run `cargo test typography` to see current state
4. Continue from last completed TODO

## Files to Read (in order)
1. `/docs/tickets/943-typography-test-fixes.md` - Main ticket
2. `/src/typography/english.rs` - 3 failing tests
3. `/src/typography/german.rs` - 2 failing tests
4. `/src/typography/french.rs` - 1 failing test
5. Check test output patterns

## Critical Issues Identified

### Problem 1: Quote Regex Adding Extra Characters
The regex patterns are capturing and then adding quotes incorrectly.
Example: `"Hello"` becomes `"Hello""` (extra quote at end)

### Problem 2: German Abbreviations
The `apply_all` function isn't applying NBSP correctly for abbreviations.
Expected: `z.\u{00A0}B.` but getting: `z. B.`

### Problem 3: English Contractions
The replacement isn't working, likely needs to happen before quote processing.

## Implementation TODOs

### Phase 1: Debug Quote Handling
- [ ] Add debug prints to see what regex is matching
- [ ] Fix the closing quote regex pattern
- [ ] Test with simple strings first
- [ ] Ensure no double replacement

### Phase 2: Fix Each Language
- [ ] English: Fix quote regex patterns
- [ ] English: Fix contraction replacements (apply before quotes)
- [ ] German: Fix quote patterns
- [ ] German: Fix abbreviation NBSP application
- [ ] French: Fix guillemet patterns

### Phase 3: Re-enable Tests
- [ ] Remove #[ignore] from all 6 tests
- [ ] Run cargo test typography
- [ ] Verify all pass
- [ ] Check JS output still works

## Test Commands
```bash
# Run only typography tests
cargo test typography

# Run specific test
cargo test test_us_quotes

# Run with output
cargo test typography -- --nocapture
```

## Decision References
- DEC018: Tests temporarily disabled for CI/CD

## Notes
- The issue is likely in the regex replacement logic
- Consider simplifying to non-regex approach if too complex
- Make sure changes don't break the JS output