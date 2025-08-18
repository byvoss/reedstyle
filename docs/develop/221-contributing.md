# Contributing to ReedSTYLE

We welcome contributions! This guide will help you get started.

## Code of Conduct

Be respectful, inclusive, and constructive. We're building something great together.

## Getting Started

### Fork and Clone

```bash
# Fork on GitHub, then:
git clone https://github.com/YOUR_USERNAME/reedstyle.git
cd reedstyle
git remote add upstream https://github.com/reedstyle/reedstyle.git
```

### Development Setup

```bash
# Install Rust (includes Cargo)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Lightning CSS via Cargo
cargo install lightningcss-cli

# Install Node.js dependencies (SWC + TypeScript)
npm install

# Build the project
cargo run  # Runs our Rust build system

# The build uses:
# - Rust: CSS generation from namespaces
# - Lightning CSS: Optimization & minification
# - TSC: Type checking
# - SWC: Fast TypeScript → JavaScript

# Start development server
python3 -m http.server 8000
```

## Project Structure

```
reedstyle/
├── src/
│   ├── namespaces/     # Namespace implementations
│   ├── typescript/     # JavaScript source
│   ├── main.rs        # Rust entry point
│   └── settings.rs    # Configuration
├── docs/
│   └── develop/       # Documentation
├── examples/          # Example HTML files
├── tests/            # Test files
└── dist/             # Build output
```

## Contributing Code

### Namespace Development

To add or modify a namespace:

1. Create/edit file in `src/namespaces/`
2. Follow the existing pattern:

```rust
// src/namespaces/your_namespace.rs
use crate::utils::*;

pub fn generate() -> Result<String> {
    let mut css = String::new();
    
    // Add your namespace CSS
    css.push_str("/* Your Namespace */\n");
    
    // Generate rules
    css.push_str(&generate_rules());
    
    Ok(css)
}

fn generate_rules() -> String {
    // Your implementation
}
```

3. Register in `src/main.rs`:

```rust
mod namespaces {
    // ...
    pub mod your_namespace;
}

// In generate_css()
css.push_str(&namespaces::your_namespace::generate()?);
```

### Adding Presets

Create new presets in `src/presets/`:

```rust
// src/presets/my_preset.rs
pub fn generate() -> String {
    r#"
    reed[as="my-preset"] {
        /* Default styles */
        display: block;
        padding: var(--reedstyle-space-4);
    }
    "#.to_string()
}
```

### JavaScript Features

Add features in `src/typescript/`:

```typescript
// src/typescript/features/myfeature.ts
export class MyFeature {
    constructor(private element: HTMLElement) {}
    
    init() {
        // Your feature implementation
    }
}
```

## Testing

### Running Tests

```bash
# Rust tests
cargo test

# JavaScript tests
npm test

# Visual regression tests
npm run test:visual
```

### Writing Tests

Rust tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_spacing_scale() {
        let css = generate_spacing();
        assert!(css.contains("padding: 1rem"));
    }
}
```

JavaScript tests:

```javascript
// tests/reed.test.js
describe('Reed Element', () => {
    test('applies attributes correctly', () => {
        const el = document.createElement('reed');
        el.setAttribute('as', 'card');
        
        expect(el.getAttribute('as')).toBe('card');
    });
});
```

## Documentation

### Writing Docs

- Use clear, concise language
- Include code examples
- Follow the existing structure
- Test all examples

### Documentation Structure

```
docs/develop/
├── 0xx - Getting Started
├── 1xx - Namespaces
├── 2xx - Developer Guide
└── 3xx - Advanced
```

## Submission Process

### 1. Create Feature Branch

```bash
git checkout -b feature/your-feature
# or
git checkout -b fix/issue-description
```

### 2. Make Changes

- Write clean, documented code
- Follow existing patterns
- Add tests if applicable
- Update documentation

### 3. Commit Guidelines

Use semantic commits:

```bash
git commit -m "feat: add new color variant system"
git commit -m "fix: correct padding calculation"
git commit -m "docs: update responsive examples"
git commit -m "test: add fx namespace tests"
git commit -m "refactor: simplify grid generation"
```

### 4. Push and PR

```bash
git push origin feature/your-feature
```

Then create a Pull Request on GitHub with:
- Clear title and description
- Link to related issues
- Screenshots if visual changes
- Test results

## Style Guidelines

### Rust Code

```rust
// Good: Clear, idiomatic Rust
pub fn generate_padding(scale: &HashMap<i32, String>) -> String {
    scale.iter()
        .map(|(key, value)| format!(
            "reed[box*=\"padding:{}\"] {{ padding: {}; }}\n",
            key, value
        ))
        .collect()
}

// Avoid: Unclear variable names, missing types
pub fn gen(s) -> String {
    // ...
}
```

### CSS Output

```css
/* Good: Organized, commented */
/* Box Namespace - Padding */
reed[box*="padding:4"] {
    padding: var(--reedstyle-space-4);
}

/* Avoid: No organization */
reed[box*="padding:4"]{padding:1rem}
```

### TypeScript

```typescript
// Good: Typed, documented
/**
 * Apply responsive attributes to element
 */
export function applyResponsive(
    element: HTMLElement,
    breakpoint: Breakpoint
): void {
    // Implementation
}

// Avoid: Untyped, undocumented
function apply(el, bp) {
    // ...
}
```

## Performance Considerations

When contributing, consider:

1. **CSS Size**: Keep selectors efficient
2. **Runtime Performance**: Use GPU-accelerated properties
3. **Build Time**: Optimize Rust code
4. **Browser Compatibility**: Test in multiple browsers

## Common Patterns

### Adding a Property

```rust
// 1. Define the property map
const PROPERTY_MAP: &[(&str, &str)] = &[
    ("value1", "css-value-1"),
    ("value2", "css-value-2"),
];

// 2. Generate CSS
for (key, value) in PROPERTY_MAP {
    css.push_str(&format!(
        "reed[namespace*=\"property:{}\"] {{ css-property: {}; }}\n",
        key, value
    ));
}
```

### Adding Responsive Support

```rust
// Generate for each breakpoint
for breakpoint in BREAKPOINTS {
    css.push_str(&generate_responsive(
        namespace,
        property,
        value,
        breakpoint
    ));
}
```

## Getting Help

- **Discord**: [Join our community](https://discord.gg/reedstyle)
- **Issues**: [GitHub Issues](https://github.com/reedstyle/reedstyle/issues)
- **Discussions**: [GitHub Discussions](https://github.com/reedstyle/reedstyle/discussions)

## Recognition

Contributors are recognized in:
- README.md contributors section
- Release notes
- Website credits

## License

By contributing, you agree that your contributions will be licensed under the Apache License 2.0.

## Next: [Custom Components](301-custom-components.md)