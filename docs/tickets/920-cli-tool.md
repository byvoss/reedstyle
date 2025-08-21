# Ticket #920: CLI Tool

## Status: 📋 Planned

## Overview
Create a comprehensive CLI tool for ReedSTYLE development, including project initialization, theme management, component scaffolding, and development server. This tool simplifies the ReedSTYLE workflow for developers.

## Files to Read First (Understanding Current System)
Before implementing, read these files to understand the existing build system:

1. **Current Build System**:
   - `/src/main.rs` - Current CLI entry point (basic)
   - `/src/builder/mod.rs` - Build process
   - `/Cargo.toml` - Current binary configuration

2. **Configuration System**:
   - `/src/config/mod.rs` - Configuration structures
   - Theme and component loading logic

3. **For CLI Design Reference**:
   - Modern Rust CLI patterns with clap
   - Consider existing tools like Vite, Parcel

## Objectives
- Create user-friendly CLI for ReedSTYLE operations
- Support project initialization and scaffolding
- Enable theme and component management
- Provide development server with hot reload
- Include validation and optimization commands
- Maintain backward compatibility with current cargo run

## Technical Requirements

### 1. CLI Structure
Using clap for argument parsing:
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "reedstyle")]
#[command(about = "ReedSTYLE CSS Framework CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new ReedSTYLE project
    Init {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        theme: Option<String>,
    },
    
    /// Build CSS and JS files
    Build {
        #[arg(long)]
        theme: Option<String>,
        #[arg(long)]
        watch: bool,
        #[arg(long)]
        minify: bool,
    },
    
    /// Start development server
    Dev {
        #[arg(long, default_value = "8080")]
        port: u16,
        #[arg(long)]
        open: bool,
    },
    
    /// Manage themes
    Theme {
        #[command(subcommand)]
        action: ThemeCommands,
    },
    
    /// Manage components
    Component {
        #[command(subcommand)]
        action: ComponentCommands,
    },
    
    /// Validate configuration
    Validate,
    
    /// Show version info
    Version,
}
```

### 2. Command Implementations

#### Init Command
```bash
reedstyle init --name my-project
```
Creates:
```
my-project/
├── themes/
│   └── default/
│       ├── reedstyle.config.yaml
│       ├── reedstyle.colors.yaml
│       ├── reedstyle.fonts.yaml
│       └── reedstyle.components.yaml
├── src/
│   └── index.html
├── dist/
└── .gitignore
```

#### Build Command
```bash
# Basic build
reedstyle build

# With theme
reedstyle build --theme=dark

# Watch mode
reedstyle build --watch

# Production
reedstyle build --minify
```

#### Dev Server
```bash
# Start dev server
reedstyle dev

# Custom port
reedstyle dev --port 3000

# Open browser
reedstyle dev --open
```

#### Theme Management
```bash
# List themes
reedstyle theme list

# Create new theme
reedstyle theme new seasonal-summer

# Clone existing theme
reedstyle theme clone default dark

# Switch active theme
reedstyle theme use dark

# Validate theme
reedstyle theme validate client-acme
```

#### Component Management
```bash
# List components
reedstyle component list

# Create new component
reedstyle component new my-card

# Show component info
reedstyle component show card

# Validate components
reedstyle component validate
```

### 3. Development Server
Implement with warp or actix-web:
```rust
async fn dev_server(port: u16) {
    // Watch for file changes
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
            Ok(event) => {
                // Rebuild on change
                rebuild();
                // Notify browser via WebSocket
                notify_clients();
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }).unwrap();
    
    // Serve static files
    let routes = warp::fs::dir(".")
        .or(websocket_route());
    
    warp::serve(routes)
        .run(([127, 0, 0, 1], port))
        .await;
}
```

### 4. Configuration File
Support `.reedstyle.toml` for project config:
```toml
[project]
name = "my-project"
version = "1.0.0"

[build]
default_theme = "default"
output_dir = "dist"
minify = true

[dev]
port = 8080
open = true
hot_reload = true

[themes]
available = ["default", "dark", "client-acme"]
```

### 5. Interactive Mode
Add interactive prompts for better UX:
```rust
use dialoguer::{Select, Input, Confirm};

fn init_interactive() {
    let name = Input::new()
        .with_prompt("Project name")
        .default("my-reedstyle-project")
        .interact()
        .unwrap();
    
    let theme_type = Select::new()
        .with_prompt("Choose starting theme")
        .items(&["Default", "Dark", "Custom"])
        .interact()
        .unwrap();
    
    let use_typescript = Confirm::new()
        .with_prompt("Use TypeScript?")
        .default(true)
        .interact()
        .unwrap();
}
```

## Implementation Steps

### Phase 1: Core CLI Structure
1. Add clap dependency
2. Create command structure
3. Implement basic commands
4. Add help text and examples

### Phase 2: Init & Scaffolding
1. Implement project initialization
2. Create template files
3. Add interactive mode
4. Support different project types

### Phase 3: Build Commands
1. Refactor current build logic
2. Add watch mode
3. Implement theme selection
4. Add progress indicators

### Phase 4: Development Server
1. Add web server dependency
2. Implement file watching
3. Add WebSocket for hot reload
4. Create browser auto-open

### Phase 5: Theme & Component Tools
1. Implement theme commands
2. Add component scaffolding
3. Create validation tools
4. Add migration helpers

### Phase 6: Polish & Distribution
1. Add colored output
2. Improve error messages
3. Create installation script
4. Package for distribution

## Testing Scenarios

### Test 1: Project Init
```bash
reedstyle init --name test-project
cd test-project
reedstyle build
```
Should create working project.

### Test 2: Watch Mode
```bash
reedstyle build --watch
# Edit a YAML file
# Should auto-rebuild
```

### Test 3: Dev Server
```bash
reedstyle dev --port 3000
# Open http://localhost:3000
# Edit files
# Should hot reload
```

### Test 4: Theme Management
```bash
reedstyle theme list
reedstyle theme new my-theme
reedstyle build --theme=my-theme
```

## Success Criteria
- [ ] CLI tool compiles and runs
- [ ] All commands implemented
- [ ] Project init creates valid structure
- [ ] Build command maintains compatibility
- [ ] Watch mode detects changes
- [ ] Dev server serves files
- [ ] Hot reload works
- [ ] Theme commands functional
- [ ] Component commands work
- [ ] Good error messages
- [ ] Colored output
- [ ] Installation simple

## Decision Log References
- CLI tool for better developer experience
- Maintain backward compatibility
- Focus on common workflows

## Dependencies
- Current build system (RS906) - COMPLETED ✅
- Theme system (RS915) - Planned
- Component system (RS911) - Planned

## Notes
- Keep CLI simple and intuitive
- Follow common CLI conventions
- Provide good defaults
- Support both interactive and scriptable modes
- Consider future plugin system

## Installation Methods

### Cargo Install
```bash
cargo install reedstyle-cli
```

### Homebrew (macOS)
```bash
brew tap reedstyle/tap
brew install reedstyle
```

### npm/npx
```bash
npm install -g @reedstyle/cli
# or
npx @reedstyle/cli init
```

### Direct Download
Provide pre-built binaries for:
- macOS (x64, ARM64)
- Linux (x64, ARM64)
- Windows (x64)

## Example Workflow
Complete workflow with CLI:
```bash
# 1. Create project
reedstyle init --name my-app

# 2. Enter directory
cd my-app

# 3. Start dev server
reedstyle dev

# 4. Create custom theme
reedstyle theme new brand

# 5. Edit theme
edit themes/brand/reedstyle.colors.yaml

# 6. Build for production
reedstyle build --theme=brand --minify

# 7. Deploy dist/ folder
```