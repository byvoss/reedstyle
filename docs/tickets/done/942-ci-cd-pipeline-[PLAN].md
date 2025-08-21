# RS942: CI/CD Pipeline - Implementation Plan

## Session Recovery Instructions
If session is lost, resume with:
1. Read this plan file
2. Check git branch: `git branch --show-current`
3. Check current progress in `.github/workflows/`
4. Continue from last completed TODO

## Files to Read (in order)
1. `/docs/tickets/942-ci-cd-pipeline.md` - Main ticket requirements
2. `/package.json` - Check current scripts and version
3. `/Cargo.toml` - Check build configuration
4. `/src/builder/mod.rs` - Understand build output process
5. `/.github/workflows/` - Check existing workflows (if any)

## Critical Rules from Specification
1. **NO TypeScript definitions file** - We use JSDoc in JS file
2. **4 distribution files only**: CSS, min.CSS, JS, min.JS + LICENSE
3. **File size limits**: CSS < 200KB, JS < 50KB minified
4. **Version in one place**: src/builder/mod.rs VERSION constant
5. **Rust builds everything** - No Node.js build process for CSS

## Implementation TODOs

### Phase 1: GitHub Actions Setup
- [ ] Create `.github/workflows/` directory
- [ ] Create `build.yml` for CI on push/PR
- [ ] Create `release.yml` for version tags
- [ ] Add size check workflow for PRs

### Phase 2: Build Workflow
- [ ] Setup Rust toolchain in CI
- [ ] Cache Cargo dependencies
- [ ] Run `cargo run --release` to build
- [ ] Run `cargo test` for Rust tests
- [ ] Validate output files exist
- [ ] Check file sizes are within limits
- [ ] Upload artifacts for inspection

### Phase 3: Release Workflow
- [ ] Trigger on version tags (v*)
- [ ] Build release files
- [ ] Create GitHub release with files
- [ ] Prepare for future NPM publishing (package.json)
- [ ] Add release notes template

### Phase 4: Testing & Validation
- [ ] Add file size assertions
- [ ] Validate CSS contains expected selectors
- [ ] Check JS has JSDoc comments
- [ ] Ensure LICENSE is included

### Phase 5: Local Scripts
- [ ] Create `scripts/release.sh` for version bumping
- [ ] Update version in src/builder/mod.rs
- [ ] Create release checklist

## Testing Checklist
- [ ] Push to feature branch triggers build
- [ ] PR shows file size changes
- [ ] Tag push creates GitHub release
- [ ] Release contains all 5 files (4 + LICENSE)
- [ ] Files are properly minified

## Decision References
- DEC001: Users receive pre-built files only
- DEC004: Lightning CSS for minification (we use basic for now)

## Notes
- No NPM publishing yet (future ticket)
- No CDN deployment yet (future ticket)
- Focus on GitHub Actions and releases
- Keep it simple and working

## Current State
- Distribution strategy complete (RS941)
- Build system generates all files correctly
- Ready for CI/CD automation