# Ticket #906: Rust Build System Core

## Status: 🚧 In Progress
## Priority: 🔴 Critical

## Description
Implement the core Rust build system that generates CSS from namespace definitions and processes YAML configurations. This is the foundation for all CSS generation.

## Requirements

### Core Structure
```
src/
├── main.rs              # Entry point
├── config.rs            # YAML loader & processor
├── css_generator.rs     # CSS output builder
├── color_system.rs      # OKLCH conversion
├── layer_system.rs      # CSS layer organization
├── namespaces/
│   ├── mod.rs          # Namespace registry
│   ├── box.rs          # Box namespace
│   ├── device.rs       # Device namespace  
│   ├── face.rs         # Face namespace
│   ├── fx.rs           # Effects namespace
│   ├── layout.rs       # Layout namespace
│   └── text.rs         # Text namespace
└── lib.rs              # Library exports
```

### Dependencies (Cargo.toml)
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
serde_json = "1.0"
lightningcss = "1.0"
swc_core = "0.87"
clap = { version = "4.0", features = ["derive"] }
anyhow = "1.0"
oklch = "0.1"  # Or custom implementation
glob = "0.3"
```

### Main Entry Point
```rust
// main.rs
use clap::Parser;
use anyhow::Result;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = ".")]
    root: String,
    
    #[arg(long)]
    theme: Option<String>,
    
    #[arg(long)]
    env: Option<String>,
    
    #[arg(long)]
    minify: bool,
    
    #[arg(long)]
    watch: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // 1. Load configurations
    let config = load_config(&args.root)?;
    
    // 2. Generate CSS layers
    let css = generate_css(&config, &args)?;
    
    // 3. Process with Lightning CSS
    let optimized = optimize_css(css, args.minify)?;
    
    // 4. Write output
    write_output(&config.output, optimized)?;
    
    // 5. Compile TypeScript if exists
    if has_typescript(&args.root) {
        compile_typescript(&args.root)?;
    }
    
    println!("✅ Build complete!");
    Ok(())
}
```

### CSS Generation Pipeline
```rust
// css_generator.rs
pub fn generate_css(config: &Config, args: &Args) -> Result<String> {
    let mut css = String::new();
    
    // Layer definition
    css.push_str("@layer settings, bridge, theme, free;\n\n");
    
    // 1. Settings layer
    css.push_str("@layer settings {\n");
    css.push_str(&generate_settings(&config)?);
    css.push_str("}\n\n");
    
    // 2. Bridge layer (if configured)
    if let Some(bridge) = &config.bridge {
        css.push_str("@layer bridge {\n");
        css.push_str(&generate_bridge_layers(bridge)?);
        css.push_str("}\n\n");
    }
    
    // 3. Theme layer with sublayers
    css.push_str("@layer theme {\n");
    css.push_str(&generate_theme(&config)?);
    
    // Environment sublayers
    if let Some(env_config) = &config.env {
        for (name, env) in &env_config.environments {
            if env.enabled {
                css.push_str(&format!("  @layer {} {{\n", name));
                css.push_str(&generate_environment_styles(name, env)?);
                css.push_str("  }\n");
            }
        }
    }
    
    css.push_str("}\n\n");
    
    // 4. Free layer (empty, for user CSS)
    css.push_str("@layer free {\n");
    css.push_str("  /* User styles go here */\n");
    css.push_str("}\n");
    
    Ok(css)
}
```

### Namespace Generator
```rust
// namespaces/box.rs
use crate::config::SpacingScale;

pub fn generate(scale: &SpacingScale) -> String {
    let mut css = String::new();
    
    // Padding
    for (key, value) in scale.values() {
        css.push_str(&format!(
            "reed[box*=\"padding:{}\"] {{ padding: {}; }}\n",
            key, value
        ));
        
        // Directional padding
        css.push_str(&format!(
            "reed[box*=\"padding-x:{}\"] {{ padding-left: {}; padding-right: {}; }}\n",
            key, value, value
        ));
        css.push_str(&format!(
            "reed[box*=\"padding-y:{}\"] {{ padding-top: {}; padding-bottom: {}; }}\n",
            key, value, value
        ));
    }
    
    // Margin (same pattern)
    // Width/Height
    // Display properties
    
    css
}
```

## Implementation Plan

1. **Setup Rust project structure**
   - Create Cargo.toml with dependencies
   - Setup module structure
   - Configure build profiles

2. **Implement configuration loader**
   - YAML parsing with serde_yaml
   - Config validation
   - Default values

3. **Create CSS generator core**
   - Layer system implementation
   - Namespace modules
   - CSS string builder

4. **Integrate Lightning CSS**
   - Optimization pipeline
   - Minification
   - Vendor prefixes

5. **Add CLI interface**
   - Command parsing with clap
   - Watch mode
   - Error handling

## Acceptance Criteria

- [ ] Reads all YAML config files from project root
- [ ] Generates complete CSS with proper layers
- [ ] Converts all colors to OKLCH internally
- [ ] Outputs to configured paths
- [ ] Supports theme folders
- [ ] Supports environment sublayers
- [ ] Integrates with Lightning CSS
- [ ] Build time < 500ms for full rebuild
- [ ] Watch mode for development
- [ ] Clear error messages

## Testing Checklist

- [ ] Unit tests for each module
- [ ] Integration test for full build
- [ ] Performance benchmark test
- [ ] Error handling tests
- [ ] YAML parsing edge cases
- [ ] CSS output validation
- [ ] File system operations
- [ ] CLI argument parsing

## Testing

```bash
# Test basic build
cargo run

# Test with theme
cargo run -- --theme=dark

# Test with environment
cargo run -- --env=prod

# Test watch mode
cargo run -- --watch

# Verify output
cat dist/reedstyle.css | head -20
```

## Dependencies

- Ticket #903 (YAML Configuration) - Need config structure
- Ticket #904 (OKLCH Color System) - Need color conversion

## Blocks

- Ticket #907 (Namespace CSS Generation)
- Ticket #909 (Lightning CSS Integration)

## Verification Steps

### Code Verification
```rust
// Verify each component follows patterns
assert!(css.contains("@layer settings, bridge, theme, free;"));
assert!(css.contains("reed["));
assert!(!css.contains("data-r-")); // Old system check
```

### Output Verification
```bash
# 1. Check CSS structure
grep "@layer" dist/reedstyle.css
grep "reed\[" dist/reedstyle.css | head -20

# 2. Validate CSS syntax
npx lightningcss dist/reedstyle.css --minify --targets "> 0.5%"

# 3. Size verification
test $(wc -c < dist/reedstyle.css) -lt 358400    # <350KB
test $(wc -c < dist/reedstyle.min.css) -lt 184320 # <180KB

# 4. Performance check
time cargo run --release # Should be < 500ms
```

### Documentation Verification
- [ ] README updated with build instructions
- [ ] Cargo.toml documented
- [ ] Module documentation in code
- [ ] Architecture docs updated

## Commit Message Template
```
feat(build): Implement core Rust build system (#906)

- Setup Rust project with Lightning CSS integration
- Implement YAML configuration loading
- Create CSS layer generation system
- Add namespace module structure
- Include comprehensive test suite

Performance: ~300ms full build
Output size: CSS 340KB, min 175KB

Closes #906
```

## Notes

- Start with minimal viable generator
- Add namespaces incrementally
- Focus on correct layer structure first
- Performance optimization comes later
- This is the foundation - must be solid