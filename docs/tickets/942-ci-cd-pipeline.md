# Ticket #942: CI/CD Pipeline

## Status: ✅ Completed

## Decision Log References
- DEC001 - Build Process (automated distribution builds)

## Description
Implement complete CI/CD pipeline for building, testing, and distributing ReedSTYLE pre-built files.

## Requirements

### Core Build System (Rust)

Since users receive pre-built files, our internal build process is critical:

```toml
# Cargo.toml
[package]
name = "reedstyle"
version = "1.0.0"

[dependencies]
lightningcss = "1.0.0"
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
anyhow = "1.0"
walkdir = "2.5"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

### Build Output Structure

```
dist/
├── reedstyle.css         # Development CSS (~350KB)
├── reedstyle.min.css     # Production CSS (~180KB)
├── reedstyle.js          # Optional JavaScript (~100KB)
├── reedstyle.min.js      # Production JS (~40KB)
├── reedstyle.d.ts        # TypeScript definitions
├── LICENSE               # Apache 2.0
└── README.md            # Usage instructions
```

### GitHub Actions Workflow

#### Build & Test (.github/workflows/build.yml)
```yaml
name: Build and Test

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  build:
    runs-on: ubuntu-latest
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          override: true
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
      
      - name: Cache Rust dependencies
        uses: actions/cache@v3
        with:
          path: |
            ~/.cargo/registry
            ~/.cargo/git
            target
          key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
      
      - name: Install minimal NPM deps
        run: npm ci --production=false
      
      - name: Build with Rust
        run: cargo run --release
      
      - name: Type check TypeScript
        run: npm run type-check
      
      - name: Test CSS generation
        run: cargo test
      
      - name: Validate output files
        run: |
          test -f dist/reedstyle.css
          test -f dist/reedstyle.min.css
          test -f dist/reedstyle.js
          test -f dist/reedstyle.min.js
      
      - name: Check file sizes
        run: |
          echo "CSS size: $(wc -c < dist/reedstyle.min.css) bytes"
          echo "JS size: $(wc -c < dist/reedstyle.min.js) bytes"
          # Fail if CSS > 200KB or JS > 50KB
          test $(wc -c < dist/reedstyle.min.css) -lt 204800
          test $(wc -c < dist/reedstyle.min.js) -lt 51200
      
      - name: Upload artifacts
        uses: actions/upload-artifact@v3
        with:
          name: dist
          path: dist/
```

#### Release Workflow (.github/workflows/release.yml)
```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  release:
    runs-on: ubuntu-latest
    
    permissions:
      contents: write
      packages: write
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          registry-url: 'https://registry.npmjs.org'
      
      - name: Install dependencies
        run: |
          npm ci --production=false
      
      - name: Build release
        run: |
          cargo run --release
          npm run type-check
      
      - name: Create npm package
        run: |
          cp package.json dist/
          cp README.md dist/
          cp LICENSE dist/
      
      - name: Publish to NPM
        working-directory: ./dist
        run: npm publish
        env:
          NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}
      
      - name: Deploy to CDN (jsDelivr auto-updates from NPM)
        run: echo "jsDelivr will auto-update from NPM"
      
      - name: Create GitHub Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            dist/reedstyle.css
            dist/reedstyle.min.css
            dist/reedstyle.js
            dist/reedstyle.min.js
            dist/reedstyle.d.ts
          body: |
            ## Installation
            
            ### CDN
            ```html
            <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/reedstyle@${{ github.ref_name }}/dist/reedstyle.min.css">
            <script src="https://cdn.jsdelivr.net/npm/reedstyle@${{ github.ref_name }}/dist/reedstyle.min.js" defer></script>
            ```
            
            ### NPM
            ```bash
            npm install reedstyle@${{ github.ref_name }}
            ```
```

### Testing Strategy

#### CSS Testing (Rust)
```rust
// tests/css_generation.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reed_element_selectors() {
        let css = generate_box_namespace();
        assert!(css.contains("reed[box*=\"padding:4\"]"));
        assert!(css.contains("reed[box-tablet*=\"padding:4\"]"));
    }
    
    #[test]
    fn test_layer_structure() {
        let css = generate_complete_css();
        assert!(css.contains("@layer settings, bridge, theme, free;"));
        assert!(css.contains("@layer theme {"));
    }
    
    #[test]
    fn test_oklch_conversion() {
        let color = convert_to_oklch("#FF6B6B");
        assert!(color.starts_with("oklch("));
    }
}
```

#### JavaScript Testing
```javascript
// tests/reed-element.test.js
import { describe, it, expect } from 'vitest';
import '../dist/reedstyle.js';

describe('Reed Element', () => {
  it('registers custom element', () => {
    expect(customElements.get('reed')).toBeDefined();
  });
  
  it('validates component names', () => {
    const reed = document.createElement('reed');
    reed.setAttribute('as', 'button-primary');
    document.body.appendChild(reed);
    
    // Should accept valid name
    expect(reed.classList.contains('rs-button-primary')).toBe(true);
  });
  
  it('rejects invalid component names', () => {
    const reed = document.createElement('reed');
    reed.setAttribute('as', 'Button_Primary'); // Invalid
    
    // Should log warning
    expect(console.warn).toHaveBeenCalled();
  });
});
```

### Performance Monitoring

#### Bundle Size Tracking
```yaml
# .github/workflows/size-check.yml
name: Bundle Size Check

on: [pull_request]

jobs:
  size:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: preactjs/compressed-size-action@v2
        with:
          pattern: "dist/**/*.{js,css}"
          max-size: "200KB CSS, 50KB JS"
```

### Local Development

```bash
# Development build with watch
cargo watch -x run

# Type checking in watch mode  
npm run type-check -- --watch

# Serve locally for testing
python3 -m http.server 8000 --directory dist

# Run all tests
cargo test && npm test
```

### Version Management

```bash
# Bump version (updates Cargo.toml and package.json)
./scripts/bump-version.sh patch  # or minor/major

# Create release tag
git tag v1.0.1
git push origin v1.0.1  # Triggers release workflow
```

### CDN Strategy

1. **Primary:** NPM → jsDelivr (automatic)
2. **Fallback:** GitHub releases
3. **Custom:** Self-hosted CDN option

```html
<!-- jsDelivr (auto-updates from NPM) -->
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/reedstyle/dist/reedstyle.min.css">

<!-- unpkg (also auto-updates) -->
<link rel="stylesheet" href="https://unpkg.com/reedstyle/dist/reedstyle.min.css">

<!-- GitHub CDN -->
<link rel="stylesheet" href="https://raw.githubusercontent.com/reedstyle/reedstyle/v1.0.0/dist/reedstyle.min.css">
```

## Acceptance Criteria

- [ ] Rust builds complete CSS with all namespaces
- [ ] Lightning CSS optimizes output
- [ ] SWC compiles TypeScript to JavaScript
- [ ] File sizes within limits (CSS <200KB, JS <50KB)
- [ ] GitHub Actions automates build/test/release
- [ ] NPM package publishes automatically on tag
- [ ] CDN updates automatically from NPM
- [ ] Version numbers sync across Cargo.toml/package.json
- [ ] Tests pass for CSS generation and JS functionality

## Dependencies

- Ticket #906 (Rust Build System)
- Ticket #907 (Namespace CSS Generation)
- Ticket #909 (Lightning CSS Integration)
- Ticket #910 (SWC/TSC Pipeline)

## Blocks

- Ticket #927 (CDN Distribution)
- Ticket #928 (Package Publishing)

## Notes

- Users never see the build process
- We ship only the final dist files
- No build tools required for users
- CI/CD is internal only
- Focus on file size and performance