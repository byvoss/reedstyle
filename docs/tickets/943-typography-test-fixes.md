# Ticket #943: Typography Test Fixes

## Status: 📋 Planned

## Priority: 🟡 Medium

## Decision Log References
- DEC018 - Typography Tests (disabled for CI/CD)

## Description
Fix failing typography tests that were temporarily disabled to unblock CI/CD pipeline. The regex patterns in typography modules are not correctly handling quotes and abbreviations.

## Requirements

### Failing Tests to Fix

#### English (src/typography/english.rs)
- [ ] test_us_quotes - Extra quote character being added
- [ ] test_gb_quotes - Extra quote character being added  
- [ ] test_contractions - Not preserving correct apostrophes

#### German (src/typography/german.rs)
- [ ] test_quotes - Extra quote character being added
- [ ] test_abbreviations - Not applying non-breaking spaces correctly

#### French (src/typography/french.rs)
- [ ] test_guillemets - Extra quote character being added

### Root Cause
The regex patterns for quote replacement are incorrectly capturing and replacing, leading to extra quote characters at the end of strings.

## Implementation Plan

1. **Analyze Quote Handling**
   - Review regex patterns in apply_quotes functions
   - Fix the closing quote detection logic
   - Ensure quotes are properly paired

2. **Fix Abbreviation Spacing**
   - German abbreviations need proper NBSP handling
   - Update format_abbreviations function

3. **Fix Contractions**
   - English contractions need proper apostrophe replacement
   - Update the replacement patterns

4. **Re-enable Tests**
   - Remove #[ignore] attributes
   - Verify all tests pass
   - Run full test suite

## Acceptance Criteria

- [ ] All typography tests pass without #[ignore]
- [ ] Quote handling works correctly for all languages
- [ ] Abbreviations have proper non-breaking spaces
- [ ] Contractions use correct typographic apostrophes
- [ ] No regression in other tests
- [ ] CI/CD pipeline remains green

## Testing Checklist

- [ ] Run `cargo test typography` locally
- [ ] Test with various quote combinations
- [ ] Test nested quotes
- [ ] Test edge cases (quotes at start/end)
- [ ] Verify in actual JS output

## Dependencies

- Requires: #916 (Typography Engine)
- Blocks: None (tests are disabled, not blocking)

## Notes

- Tests were disabled in RS942 to unblock CI/CD
- This is not blocking production but should be fixed soon
- Consider adding more comprehensive test cases
- May need to refactor regex approach entirely