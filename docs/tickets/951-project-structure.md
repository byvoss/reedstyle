# Ticket #951: Project Structure

## Status: 📋 Planned

## Overview
Establish the complete project directory structure for ReedSTYLE, organizing source code, documentation, tests, and distribution files according to best practices for a Rust-based CSS framework project.

## Files to Read First (Understanding Current System)
Before implementing, read these files to understand the existing structure:

1. **Current Project Layout**:
   - `/src/` - Current source code organization
   - `/docs/` - Documentation structure
   - `/test/` - Test files location
   - `/dist/` - Distribution output

2. **Build System**:
   - `/Cargo.toml` - Rust project configuration
   - `/package.json` - Node dependencies (for CI/CD)
   - `/src/main.rs` - Entry point

3. **Documentation**:
   - `/docs/develop/` - Development documentation
   - `/docs/tickets/` - Ticket system
   - `/CLAUDE.md` - AI development guide

## Objectives
- Create clear, logical directory structure
- Separate concerns (source, tests, docs, dist)
- Support future growth and features
- Enable easy navigation and discovery
- Follow Rust and web development conventions

## Technical Requirements

### 1. Complete Directory Structure
```
reedstyle/
├── src/                        # Rust source code
│   ├── main.rs                 # CLI entry point
│   ├── lib.rs                  # Library entry point
│   ├── builder/                # Build system
│   │   ├── mod.rs
│   │   ├── minify.rs
│   │   └── sri.rs
│   ├── css/                    # CSS generation
│   │   ├── mod.rs
│   │   ├── breakpoints.rs
│   │   ├── defaults.rs
│   │   ├── namespaces/         # Namespace modules
│   │   │   ├── mod.rs
│   │   │   ├── box.rs
│   │   │   ├── device.rs
│   │   │   ├── face.rs
│   │   │   ├── fx.rs
│   │   │   ├── layout.rs
│   │   │   └── text.rs
│   │   └── presets/            # Component presets
│   │       ├── mod.rs
│   │       ├── buttons.rs
│   │       ├── cards.rs
│   │       ├── forms.rs
│   │       └── navigation.rs
│   ├── config/                 # Configuration loading
│   │   ├── mod.rs
│   │   ├── colors.rs
│   │   ├── components.rs
│   │   ├── fonts.rs
│   │   └── theme.rs
│   ├── color/                  # Color system
│   │   ├── mod.rs
│   │   └── oklch.rs
│   ├── typography/             # Typography engine
│   │   ├── mod.rs
│   │   ├── german.rs
│   │   ├── english.rs
│   │   └── french.rs
│   ├── js/                     # JavaScript generation
│   │   ├── mod.rs
│   │   ├── api.rs
│   │   └── effects.rs
│   ├── optimizer/              # CSS/JS optimization
│   │   ├── mod.rs
│   │   └── lightning.rs
│   └── cli/                    # CLI tool
│       ├── mod.rs
│       ├── commands.rs
│       └── server.rs
│
├── src/typescript/             # TypeScript source
│   ├── index.ts
│   ├── core.ts
│   ├── components.ts
│   ├── effects.ts
│   └── typography.ts
│
├── tests/                      # Rust tests
│   ├── integration/
│   │   ├── build_test.rs
│   │   ├── css_test.rs
│   │   └── js_test.rs
│   └── fixtures/              # Test fixtures
│       ├── components.yaml
│       └── test.html
│
├── test/                       # HTML test pages
│   ├── test-element-defaults.html
│   ├── test-effects.html
│   ├── test-typography.html
│   └── test-responsive.html
│
├── docs/                       # Documentation
│   ├── develop/               # Development docs
│   │   ├── 000-index.md
│   │   ├── 001-introduction.md
│   │   └── ...
│   ├── tickets/               # Ticket system
│   │   ├── done/             # Completed tickets
│   │   ├── 900-roadmap-index.md
│   │   └── ...
│   └── api/                   # API documentation
│       └── index.md
│
├── examples/                   # Example projects
│   ├── basic/
│   ├── blog/
│   ├── dashboard/
│   └── e-commerce/
│
├── themes/                     # Theme configurations
│   ├── default/
│   │   ├── reedstyle.config.yaml
│   │   ├── reedstyle.colors.yaml
│   │   ├── reedstyle.fonts.yaml
│   │   └── reedstyle.components.yaml
│   └── dark/
│       └── reedstyle.colors.yaml
│
├── dist/                       # Build output
│   ├── reedstyle.css
│   ├── reedstyle.min.css
│   ├── reedstyle.js
│   ├── reedstyle.min.js
│   ├── reedstyle.d.ts
│   └── LICENSE
│
├── scripts/                    # Utility scripts
│   ├── release.sh
│   ├── verify-ticket.sh
│   └── migrate.sh
│
├── .github/                    # GitHub configuration
│   ├── workflows/
│   │   ├── build.yml
│   │   ├── release.yml
│   │   └── test.yml
│   ├── ISSUE_TEMPLATE/
│   └── PULL_REQUEST_TEMPLATE.md
│
├── .archiv/                    # Archived old code
│   └── ...
│
├── Cargo.toml                  # Rust dependencies
├── Cargo.lock                  # Locked dependencies
├── package.json                # Node dependencies
├── package-lock.json           # Locked Node deps
├── tsconfig.json               # TypeScript config
├── .swcrc                      # SWC config
├── .gitignore                  # Git ignore rules
├── LICENSE                     # Apache 2.0
├── README.md                   # Project readme
├── CONTRIBUTING.md             # Contribution guide
├── CHANGELOG.md                # Version history
├── CLAUDE.md                   # AI guide (not committed)
└── decisions.csv               # Technical decisions
```

### 2. Directory Purposes

#### Source Code (`/src/`)
- **Rust modules**: Core framework logic
- **TypeScript**: Optional JavaScript enhancements
- **Clear separation**: Each module has single responsibility

#### Tests (`/tests/` and `/test/`)
- **`/tests/`**: Rust integration tests
- **`/test/`**: HTML test pages for browser testing
- **Fixtures**: Test data and configurations

#### Documentation (`/docs/`)
- **develop**: Technical documentation
- **tickets**: Development tickets
- **api**: API reference

#### Examples (`/examples/`)
- **Complete projects**: Show real usage
- **Various patterns**: Different site types
- **Copy-paste ready**: Users can start from these

#### Themes (`/themes/`)
- **Configuration sets**: Different visual themes
- **Inheritance support**: Base and variants
- **Client-specific**: Isolated customizations

#### Distribution (`/dist/`)
- **Build output**: Generated files only
- **Never committed**: Except for releases
- **Clean separation**: Source vs distribution

### 3. File Organization Rules

#### Naming Conventions
- **Rust files**: `snake_case.rs`
- **TypeScript**: `camelCase.ts`
- **Documentation**: `kebab-case.md`
- **YAML configs**: `reedstyle.{type}.yaml`

#### Module Structure
Each Rust module should have:
- `mod.rs` - Module entry point
- Clear exports
- Tests in same file or `tests.rs`
- Documentation comments

#### Import Organization
```rust
// External crates
use serde::{Deserialize, Serialize};

// Standard library
use std::fs;
use std::path::Path;

// Internal modules
use crate::config::Config;
use crate::color::oklch;

// Module code...
```

## Implementation Steps

### Phase 1: Create Structure
1. Create all directories
2. Add `.gitkeep` to empty directories
3. Move existing files to correct locations
4. Update import paths

### Phase 2: Organize Source
1. Split large modules into sub-modules
2. Create clear module boundaries
3. Add module documentation
4. Update Cargo.toml paths

### Phase 3: Setup Tests
1. Create test structure
2. Move test files
3. Add test fixtures
4. Create test runner scripts

### Phase 4: Documentation
1. Organize existing docs
2. Create missing sections
3. Add navigation index
4. Update links

### Phase 5: Examples
1. Create example projects
2. Add README to each
3. Include build scripts
4. Test examples work

## Testing Scenarios

### Test 1: Build Works
```bash
cargo build
cargo run
# Should generate files in dist/
```

### Test 2: Tests Run
```bash
cargo test
# All tests should pass
```

### Test 3: Documentation Builds
```bash
# If using mdbook or similar
mdbook build docs/
```

### Test 4: Examples Work
```bash
cd examples/basic
# Should be able to use ReedSTYLE
```

## Success Criteria
- [ ] All directories created
- [ ] Files organized logically
- [ ] Imports updated and working
- [ ] Build process unchanged
- [ ] Tests still pass
- [ ] Documentation accessible
- [ ] Examples functional
- [ ] No broken links
- [ ] Clear navigation

## Decision Log References
- Project structure for maintainability
- Separation of concerns
- Convention over configuration

## Dependencies
- Git repository setup (RS950) - Planned

## Notes
- Keep structure flat where possible
- Group related functionality
- Consider future growth
- Make discovery easy
- Follow Rust conventions

## Migration Checklist
When moving files:
1. Update import paths
2. Fix relative links
3. Update build scripts
4. Test functionality
5. Update documentation

## Benefits
- **Clear organization**: Easy to find files
- **Scalability**: Room for growth
- **Maintainability**: Logical grouping
- **Onboarding**: New developers understand quickly
- **Testing**: Clear test structure
- **Distribution**: Clean separation