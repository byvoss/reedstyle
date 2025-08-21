# Implementation Plan: RS941 - Distribution Strategy

## Session Recovery
If session dies, start here:
1. Read this file first
2. Check current branch: `git branch --show-current`
3. Continue where TODO status is not "done"

## Files to Read (in order)
1. `/docs/tickets/941-distribution-strategy.md` - Ticket specification
2. `/CLAUDE.md` - Development guidelines and rules
3. `/decisions.csv` - Technical decisions log
4. `/src/main.rs` - Current build system
5. `/Cargo.toml` - Project dependencies
6. `/dist/` - Check current distribution files

## Critical Rules from Spec
1. **Element Name**: Now using `<r-s>` not `<reed>` (from RS919)
2. **No Web Component**: r-s works without registration
3. **Files Required**:
   - `dist/reedstyle.css` - Development version
   - `dist/reedstyle.min.css` - Production minified
   - `dist/reedstyle.js` - Optional JavaScript
   - `dist/reedstyle.min.js` - Production minified JS
4. **JavaScript Integration**: Typography and Effects engines already integrated
5. **Version Headers**: Must include version and license in file headers

## Implementation Tasks

### TODO 1: Update Build System for Minification
- [ ] Add minification step to main.rs
- [ ] Generate both development and production CSS
- [ ] Generate both development and production JS
- [ ] Add version headers to all output files

### TODO 2: Add License File to dist/
- [ ] Copy LICENSE file to dist/ during build
- [ ] Ensure Apache 2.0 license is clear

### TODO 3: Create Version Management
- [ ] Add version constant to main.rs
- [ ] Include version in file headers
- [ ] Update Cargo.toml version

### TODO 4: Add TypeScript Definitions
- [ ] Create reedstyle.d.ts file
- [ ] Define ReedStyle global object interface
- [ ] Define r-s element types

### TODO 5: Update Build Output
- [ ] Ensure all 4 main files are generated
- [ ] Add file size reporting
- [ ] Verify minification works

### TODO 6: Create Usage Examples
- [ ] Create example HTML files showing CDN usage
- [ ] Test with and without JavaScript
- [ ] Document in README

## Decision References
- DEC001: Pre-built distribution files
- DEC004: Lightning CSS for minification (not yet implemented)
- DEC017: Change from reed to r-s

## Notes
- Currently we don't have minification implemented
- Need to add a minification library (minify-js or similar)
- TypeScript definitions are new requirement
- Version management needs to be centralized