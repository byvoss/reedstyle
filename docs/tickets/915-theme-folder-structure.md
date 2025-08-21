# Ticket #915: Theme Folder Structure

## Status: 📋 Planned

## Overview
Implement theme folder structure for managing multiple design systems, client variations, and color schemes. This is the RECOMMENDED approach instead of environments for handling different visual themes.

## Files to Read First (Understanding Current System)
Before implementing, read these files to understand the existing configuration system:

1. **Current Configuration Loading**:
   - `/src/config/mod.rs` - How YAML configs are loaded
   - `/src/main.rs` - Main entry point with config loading
   - `/src/builder/mod.rs` - Build process

2. **Current Config Files**:
   - `/reedstyle.config.yaml` - Main configuration
   - `/reedstyle.colors.yaml` - Color definitions
   - `/reedstyle.components.yaml` - Component definitions

3. **Documentation for Context**:
   - `/docs/develop/401-configuration.md` - Configuration guide
   - `/docs/develop/601-environments.md` - Theme folders as alternative to environments

## Objectives
- Support multiple theme folders with isolated configurations
- Enable theme selection at build time
- Maintain clean separation between themes
- Support inheritance from base theme
- Generate separate CSS files per theme
- Simplify multi-client deployments

## Technical Requirements

### 1. Folder Structure
Standard theme organization:
```
project-root/
├── themes/
│   ├── default/                    # Base theme
│   │   ├── reedstyle.config.yaml
│   │   ├── reedstyle.colors.yaml
│   │   ├── reedstyle.fonts.yaml
│   │   └── reedstyle.components.yaml
│   │
│   ├── dark/                       # Dark mode variant
│   │   ├── extends: default        # Inheritance marker
│   │   └── reedstyle.colors.yaml   # Only override colors
│   │
│   ├── client-acme/                # Client-specific theme
│   │   ├── reedstyle.colors.yaml   # Brand colors
│   │   ├── reedstyle.fonts.yaml    # Brand fonts
│   │   └── reedstyle.components.yaml
│   │
│   └── seasonal-winter/            # Seasonal theme
│       ├── extends: default
│       ├── reedstyle.colors.yaml
│       └── reedstyle.components.yaml
```

### 2. Theme Selection
Build-time theme selection:
```bash
# Build default theme
cargo run

# Build specific theme
cargo run --theme=dark

# Build client theme
cargo run --theme=client-acme

# Environment variable
REEDSTYLE_THEME=dark cargo run
```

### 3. Theme Inheritance
Support extending base themes:
```yaml
# themes/dark/.theme.yaml
extends: default
override:
  - colors    # Replace colors entirely
merge:
  - components  # Merge with base components
```

### 4. Configuration Loading Logic
```rust
// src/config/theme.rs
pub fn load_theme(theme_name: &str) -> Result<Config> {
    let theme_path = format!("themes/{}", theme_name);
    
    // Check for inheritance
    let theme_meta = load_theme_meta(&theme_path)?;
    
    let mut config = if let Some(base) = theme_meta.extends {
        // Load base theme first
        load_theme(&base)?
    } else {
        Config::default()
    };
    
    // Load theme-specific configs
    config.merge_colors(load_yaml(&format!("{}/reedstyle.colors.yaml", theme_path))?);
    config.merge_fonts(load_yaml(&format!("{}/reedstyle.fonts.yaml", theme_path))?);
    config.merge_components(load_yaml(&format!("{}/reedstyle.components.yaml", theme_path))?);
    
    Ok(config)
}
```

### 5. Output Structure
Generate theme-specific files:
```
dist/
├── default/
│   ├── reedstyle.css
│   ├── reedstyle.min.css
│   ├── reedstyle.js
│   └── reedstyle.min.js
├── dark/
│   ├── reedstyle.css
│   └── reedstyle.min.css
├── client-acme/
│   ├── reedstyle.css
│   └── reedstyle.min.css
```

### 6. Theme Discovery
Auto-discover available themes:
```rust
pub fn discover_themes() -> Vec<String> {
    let themes_dir = Path::new("themes");
    if !themes_dir.exists() {
        return vec!["default".to_string()];
    }
    
    fs::read_dir(themes_dir)
        .unwrap()
        .filter_map(|entry| {
            entry.ok().and_then(|e| {
                e.file_name().to_str().map(String::from)
            })
        })
        .collect()
}
```

## Implementation Steps

### Phase 1: Core Structure
1. Create theme loading module
2. Implement theme discovery
3. Add CLI argument parsing
4. Create default theme folder

### Phase 2: Inheritance System
1. Implement theme metadata loading
2. Add inheritance resolution
3. Support partial overrides
4. Handle circular dependencies

### Phase 3: Build Integration
1. Update build process for themes
2. Generate theme-specific output
3. Add theme validation
4. Show active theme in output

### Phase 4: Migration Tools
1. Create script to migrate existing configs
2. Move current configs to themes/default
3. Update documentation
4. Add theme scaffolding command

### Phase 5: Advanced Features
1. Theme preview generation
2. Theme switching demo page
3. Theme documentation generation
4. Theme validation tools

## Testing Scenarios

### Test 1: Basic Theme Loading
```bash
cargo run --theme=default
# Should load themes/default/* configs
```

### Test 2: Theme Inheritance
```yaml
# themes/dark/extends.yaml
extends: default
```
Should inherit all default configs, override colors.

### Test 3: Client Theme
```bash
cargo run --theme=client-acme
# Should generate completely custom CSS
```

### Test 4: Missing Theme
```bash
cargo run --theme=nonexistent
# Should error with helpful message
```

## Success Criteria
- [ ] Theme folder structure implemented
- [ ] Theme discovery works
- [ ] CLI argument parsing functional
- [ ] Inheritance system operational
- [ ] Separate output per theme
- [ ] Build shows active theme
- [ ] Migration from flat configs works
- [ ] Documentation updated
- [ ] No regression in single-theme setup

## Decision Log References
- Theme folders preferred over environments
- Build-time theme selection for simplicity
- Inheritance reduces duplication

## Dependencies
- Configuration system (RS903) - COMPLETED ✅
- Build system (RS906) - COMPLETED ✅

## Notes
- This is the RECOMMENDED approach for variations
- Cleaner than environment attributes
- Better for multi-client projects
- Easier to maintain
- Clear separation of concerns

## Example Themes

### Dark Theme
```yaml
# themes/dark/reedstyle.colors.yaml
colors:
  # Override base colors for dark mode
  base-0: "oklch(10% 0 0)"      # Near black
  base-100: "oklch(15% 0 0)"    # Dark gray
  base-900: "oklch(95% 0 0)"    # Near white
  
  # Keep brand colors
  brand-a: "oklch(68.5% 0.24 25)"
```

### Client Theme
```yaml
# themes/client-acme/reedstyle.colors.yaml
colors:
  # Client's brand colors
  brand-a: "#0066CC"  # ACME Blue
  brand-b: "#FF6600"  # ACME Orange
  
# themes/client-acme/reedstyle.fonts.yaml
fonts:
  font-a:
    family: "'ACME Sans', sans-serif"
```

### Seasonal Theme
```yaml
# themes/seasonal-winter/reedstyle.colors.yaml
colors:
  brand-a: "oklch(60% 0.15 220)"  # Ice blue
  brand-b: "oklch(95% 0.02 0)"    # Snow white
  
  state-success: "oklch(55% 0.20 150)"  # Pine green
```

## Migration Path
For existing projects:
```bash
# 1. Create themes directory
mkdir -p themes/default

# 2. Move existing configs
mv reedstyle.*.yaml themes/default/

# 3. Update build command
cargo run --theme=default

# 4. Create additional themes as needed
cp -r themes/default themes/dark
# Edit themes/dark/reedstyle.colors.yaml
```