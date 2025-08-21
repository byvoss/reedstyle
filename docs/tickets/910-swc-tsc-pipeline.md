# Ticket #910: SWC/TSC Pipeline

## Status: 📋 Planned

## Overview
Implement TypeScript/JavaScript compilation pipeline using SWC (Super-fast Web Compiler) for transpilation and TSC (TypeScript Compiler) for type checking. This replaces any Node.js build tools with Rust-based SWC for 100x faster compilation.

## Files to Read First (Understanding Current System)
Before implementing, read these files to understand the existing JavaScript generation:

1. **Current JavaScript Generation**:
   - `/src/js/mod.rs` - Current JavaScript generation in Rust
   - `/src/typography/mod.rs` - Typography engine JavaScript
   - `/src/builder/mod.rs` - How JS files are currently created

2. **Build System**:
   - `/src/main.rs` - Main build orchestration
   - `/Cargo.toml` - Current dependencies

3. **Documentation for Context**:
   - `/docs/develop/201-architecture.md` - TypeScript/SWC pipeline section
   - `/docs/develop/321-javascript-api.md` - JavaScript API that needs typing

## Objectives
- Add TypeScript support for better development experience
- Use SWC for lightning-fast transpilation (Rust-based)
- Use TSC for type checking only (no emit)
- Generate JSDoc comments in output for IDE support
- Maintain optional JavaScript (CSS works without it)
- Keep build time under 500ms total

## Technical Requirements

### 1. Dual Tool Strategy
- **TSC**: Type checking only (`--noEmit`)
- **SWC**: Fast transpilation and minification
- **Why both**: TSC for accurate types, SWC for speed

### 2. TypeScript Source Structure
Create TypeScript source files:
```
src/typescript/
├── core.ts          # Core ReedStyle API
├── effects.ts       # Effects engine
├── typography.ts    # Typography engine
├── components.ts    # Component system
└── index.ts        # Main entry point
```

### 3. SWC Configuration
`.swcrc` or inline config:
```json
{
  "jsc": {
    "parser": {
      "syntax": "typescript",
      "tsx": false
    },
    "target": "es2020",
    "minify": {
      "compress": true,
      "mangle": false  // Keep function names for debugging
    }
  },
  "module": {
    "type": "es6"
  },
  "minify": true  // For production builds
}
```

### 4. Type Definitions
Generate `.d.ts` files for TypeScript users:
```typescript
// reedstyle.d.ts
declare namespace ReedStyle {
  interface Config {
    components?: string;
    effects?: boolean;
    lazy?: boolean;
  }
  
  function init(config?: Config): void;
  function createElement(type: string, props?: object): HTMLElement;
  
  // Typography engine
  namespace typography {
    function init(lang?: string): void;
    function process(text: string, filter?: 'minimal' | 'smart' | 'professional'): string;
  }
  
  // Effects engine
  namespace effects {
    function animate(element: HTMLElement, effect: string): void;
    function scroll(element: HTMLElement, options?: object): void;
  }
}

// Custom element
interface RSElement extends HTMLElement {
  as: string;
  box?: string;
  face?: string;
  text?: string;
  layout?: string;
  device?: string;
  fx?: string;
}
```

### 5. JSDoc Generation
Add JSDoc comments to generated JavaScript:
```javascript
/**
 * @typedef {Object} ReedStyleConfig
 * @property {string} [components] - Path to components YAML
 * @property {boolean} [effects] - Enable effects engine
 */

/**
 * Initialize ReedStyle
 * @param {ReedStyleConfig} [config] - Configuration options
 * @returns {void}
 */
function init(config) { /* ... */ }
```

### 6. Build Integration
Update Rust build system to:
1. Compile TypeScript with SWC
2. Run TSC for type checking
3. Generate both .js and .min.js
4. Include JSDoc in development build
5. Strip comments in production build

## Implementation Steps

### Phase 1: Setup TypeScript
1. Create `/src/typescript/` directory
2. Port existing JS from `/src/js/mod.rs` to TypeScript
3. Add proper types and interfaces
4. Create index.ts entry point

### Phase 2: SWC Integration
1. Add SWC as Rust dependency or CLI tool
2. Create SWC configuration
3. Add compilation step to build process
4. Test transpilation works

### Phase 3: Type Checking
1. Add TSC to package.json (dev dependency)
2. Configure tsconfig.json for checking only
3. Add type checking to build process
4. Ensure no emit (only checking)

### Phase 4: Type Definitions
1. Generate .d.ts file from TypeScript
2. Include in distribution
3. Add JSDoc comments for JavaScript users
4. Test IntelliSense works

### Phase 5: Build Pipeline
1. Update `/src/builder/mod.rs`
2. Add TypeScript compilation step
3. Generate dev and prod versions
4. Measure build performance

## Code Structure Example
```rust
// src/builder/typescript.rs
use std::process::Command;

pub fn compile_typescript() -> Result<(), Error> {
    // Type check with TSC (no emit)
    let tsc_output = Command::new("npx")
        .args(&["tsc", "--noEmit"])
        .output()?;
    
    if !tsc_output.status.success() {
        return Err(Error::TypeCheckFailed);
    }
    
    // Transpile with SWC
    let swc_output = Command::new("npx")
        .args(&["swc", "src/typescript", "-d", "dist/temp"])
        .output()?;
    
    // Bundle and minify
    bundle_javascript()?;
    
    Ok(())
}
```

## Testing Scenarios

### Test 1: Type Checking
Create intentional type error:
```typescript
const num: number = "string"; // Should fail TSC
```

### Test 2: JSDoc Generation
Verify IntelliSense in VS Code:
```javascript
ReedStyle.init({ 
  // Should show autocomplete for config options
});
```

### Test 3: Build Performance
- Measure full build time
- Should stay under 500ms total
- SWC should be <50ms

### Test 4: Output Validation
- Development: Readable with JSDoc
- Production: Minified without comments
- Both should work in browser

## Success Criteria
- [ ] TypeScript source files created
- [ ] SWC successfully transpiles to ES2020
- [ ] TSC type checking passes
- [ ] .d.ts file generated for TypeScript users
- [ ] JSDoc present in development build
- [ ] Production build properly minified
- [ ] Build time remains <500ms
- [ ] IntelliSense works in VS Code
- [ ] No runtime errors in browser

## Decision Log References
- Build system uses Rust primarily
- JavaScript is optional enhancement
- Need fast compilation (hence SWC)

## Dependencies
- Working JavaScript generation (RS913)
- Build system (RS906) - COMPLETED ✅

## Notes
- SWC is written in Rust, fits our ecosystem
- 100x faster than Babel
- TSC only for type checking, not compilation
- Can call SWC from Rust directly or via CLI
- JSDoc provides IDE support without TypeScript

## Alternative Approach
If SWC integration is complex, consider:
- Using SWC as Rust library directly
- Embedding TypeScript compiler in Rust
- Pre-compiling TypeScript separately
- Using esbuild as alternative