# Migration Guide

Migrate from other CSS frameworks to ReedSTYLE using the Bridge Layer system.

## Bridge Layer System

The Bridge Layer (`@layer bridge`) provides seamless integration with existing frameworks through sublayers configured in `reedstyle.bridge.yaml`.

## Configuration

### Setup Bridge Configuration

Create `reedstyle.bridge.yaml` in your project root:

```yaml
bridge:
  # Enable the frameworks you're migrating from
  bootstrap:
    enabled: true
    version: 5.3
    # Path to Bootstrap CSS (will be imported into @layer bridge.bootstrap)
    path: "./node_modules/bootstrap/dist/css/bootstrap.min.css"
  
  tailwind:
    enabled: true
    # Path to Tailwind CSS
    path: "./dist/tailwind-output.css"
    
  # Add custom integrations
  custom:
    enabled: true
    name: "my-old-styles"
    # Path to your legacy CSS
    path: "./legacy/old-styles.css"
```

### How It Works

Each enabled integration:
1. Creates a CSS sublayer (`@layer bridge.{name}`)
2. Imports the framework's CSS file into that sublayer
3. Applies your overrides on top

Generated output:

```css
@layer bridge {
  @layer bootstrap {
    /* Contents of bootstrap.min.css imported here */
    @import "./node_modules/bootstrap/dist/css/bootstrap.min.css";
    
    /* Your overrides */
    .btn {
      padding: var(--reedstyle-space-3);
    }
  }
  
  @layer tailwind {
    /* Contents of tailwind-output.css imported here */
    @import "./dist/tailwind-output.css";
    
    /* Your mappings */
  }
  
  @layer custom {
    /* Contents of old-styles.css imported here */
    @import "./legacy/old-styles.css";
    
    /* Your migrations */
  }
}
```

The layer cascade ensures ReedSTYLE wins:
1. `bridge.{framework}` - Original framework CSS
2. `theme` - ReedSTYLE and your configs
3. `free` - Your custom CSS (highest priority)

## Enabling/Disabling Frameworks

Toggle frameworks on/off during migration:

```yaml
bridge:
  bootstrap:
    enabled: true   # ← Toggle on/off like a switch
    path: "./node_modules/bootstrap/dist/css/bootstrap.min.css"
    
  tailwind:
    enabled: false  # ← Disabled, CSS not loaded
    path: "./dist/tailwind.css"
```

When `enabled: false`:
- The sublayer is not created
- The CSS file is not imported
- No performance impact
- Clean testing of migration progress

### Testing Migration Progress

```yaml
# Step 1: Both frameworks active
bridge:
  bootstrap:
    enabled: true   # Old framework active
    
# Step 2: Test without old framework
bridge:
  bootstrap:
    enabled: false  # See what breaks
    
# Step 3: Fix issues, re-enable for comparison
bridge:
  bootstrap:
    enabled: true   # Compare old vs new
    
# Step 4: Final migration
bridge:
  bootstrap:
    enabled: false  # Permanently disabled
```

### Dynamic Toggle (Development)

Use environment variables for quick switching:

```yaml
bridge:
  bootstrap:
    enabled: ${ENABLE_BOOTSTRAP:-false}
    path: "./node_modules/bootstrap/dist/css/bootstrap.min.css"
```

```bash
# Enable Bootstrap temporarily
ENABLE_BOOTSTRAP=true cargo run

# Disable for testing
ENABLE_BOOTSTRAP=false cargo run
```

### Using Profiles

Define profiles for different scenarios:

```yaml
# reedstyle.bridge.yaml
profiles:
  test:
    bootstrap: false  # All frameworks off for testing
    tailwind: false
    
  migration:
    bootstrap: true   # Old framework on during migration
    tailwind: false
    
  production:
    bootstrap: false  # All off in production
    tailwind: false
```

```bash
# Use profiles
cargo run --profile=test       # Everything disabled
cargo run --profile=migration  # Bootstrap enabled
cargo run --profile=production # Clean ReedSTYLE only
```

### Build Output

The build system shows what's enabled:

```bash
$ cargo run
Building ReedSTYLE...
✓ Bridge layer 'bootstrap' enabled (342KB)
✗ Bridge layer 'tailwind' disabled
✗ Bridge layer 'material' disabled
Total CSS: 592KB (250KB ReedSTYLE + 342KB Bootstrap)

$ cargo run --profile=production
Building ReedSTYLE...
✗ Bridge layer 'bootstrap' disabled
✗ Bridge layer 'tailwind' disabled
✗ Bridge layer 'material' disabled
Total CSS: 250KB (ReedSTYLE only)
```

## Migration Strategies

### 1. Gradual Migration (Recommended)

Keep both systems running during migration:

```yaml
# reedstyle.bridge.yaml
migration:
  gradual:
    enabled: true
    allow_mixed: true  # Use both old and new syntax
    warnings: true     # Show migration hints in console
```

Use both syntaxes in HTML:

```html
<!-- Old Bootstrap -->
<div class="btn btn-primary">Old Button</div>

<!-- New ReedSTYLE -->
<r-s as="button-primary">New Button</r-s>

<!-- Mixed during migration -->
<div class="card">
  <r-s as="card-header">Header</r-s>
  <div class="card-body">Body</div>
</div>
```

### 2. Automated Class Mapping

Map existing classes to reed attributes:

```yaml
migration:
  class_map:
    # Bootstrap
    "btn": "button"
    "btn-primary": "button-primary"
    "card": "card"
    "container": "container"
    
    # Tailwind
    "p-4": "box='padding:4'"
    "bg-white": "face='bg:base-0'"
    "flex": "layout='flex'"
```

### 3. Full Replacement

Replace framework styles completely:

```yaml
bridge:
  bootstrap:
    enabled: true
    overrides:
      - selector: "*"
        rules: |
          /* Reset all Bootstrap styles */
          all: revert-layer;
```

## Framework-Specific Migrations

### Bootstrap → ReedSTYLE

```yaml
# reedstyle.bridge.yaml
bridge:
  bootstrap:
    enabled: true
    overrides:
      - selector: ".btn"
        rules: |
          /* Use ReedSTYLE spacing */
          padding: var(--reedstyle-space-3) var(--reedstyle-space-6);
          border-radius: var(--reedstyle-radius-md);
          
      - selector: ".container"
        rules: |
          max-width: var(--reedstyle-container-width);
          padding: 0 var(--reedstyle-space-4);
```

Migration mapping:

| Bootstrap | ReedSTYLE |
|-----------|-----------|
| `.btn` | `<r-s as="button">` |
| `.btn-primary` | `<r-s as="button-primary">` |
| `.card` | `<r-s as="card">` |
| `.container` | `<r-s as="container">` |
| `.row` | `<r-s layout="[grid:12]">` |
| `.col-md-6` | `<r-s layout="[col-span:6]">` |
| `.p-4` | `<r-s box="padding:4">` |
| `.mt-3` | `<r-s box="margin-top:3">` |

### Tailwind → ReedSTYLE

```yaml
bridge:
  tailwind:
    enabled: true
    mappings:
      # Spacing
      "p-4": "box='padding:4'"
      "m-2": "box='margin:2'"
      "px-6": "box='padding-x:6'"
      
      # Colors
      "bg-blue-500": "face='bg:brand-a'"
      "text-white": "text='color:base-0'"
      
      # Layout
      "flex": "layout='flex'"
      "grid": "layout='grid'"
      "grid-cols-3": "layout='grid:3'"
```

Migration mapping:

| Tailwind | ReedSTYLE |
|----------|-----------|
| `flex` | `layout="flex"` |
| `grid` | `layout="grid"` |
| `p-4` | `box="padding:4"` |
| `bg-white` | `face="bg:base-0"` |
| `rounded-lg` | `face="radius:lg"` |
| `shadow-md` | `face="shadow:md"` |
| `text-xl` | `text="size:large"` |
| `font-bold` | `text="weight:bold"` |

### Material UI → ReedSTYLE

```yaml
bridge:
  material:
    enabled: true
    overrides:
      - selector: ".MuiButton-root"
        rules: |
          font-family: var(--reedstyle-font-a);
          border-radius: var(--reedstyle-radius-md);
          text-transform: none;
          
      - selector: ".MuiPaper-root"
        rules: |
          background: var(--reedstyle-color-base-0);
          box-shadow: var(--reedstyle-shadow-md);
```

## Custom Framework Migration

For proprietary or custom frameworks:

```yaml
bridge:
  custom:
    enabled: true
    name: "old-framework"
    css: |
      /* Map old classes to ReedSTYLE variables */
      .old-button {
        padding: var(--reedstyle-space-3) var(--reedstyle-space-6);
        background: var(--reedstyle-color-brand-a);
        border-radius: var(--reedstyle-radius-md);
      }
      
      .old-card {
        padding: var(--reedstyle-space-6);
        background: var(--reedstyle-color-base-0);
        box-shadow: var(--reedstyle-shadow-md);
      }
      
    # Optional migration script
    script: |
      // Auto-convert elements
      document.querySelectorAll('.old-button').forEach(el => {
        const reed = document.createElement('reed');
        reed.setAttribute('as', 'button-primary');
        reed.innerHTML = el.innerHTML;
        el.parentNode.replaceChild(reed, el);
      });
```

## Migration Tools

### 1. Class Scanner

Identify classes in your project:

```javascript
// migration-scanner.js
const classes = new Set();

document.querySelectorAll('*').forEach(el => {
  el.classList.forEach(cls => classes.add(cls));
});

console.log('Classes to migrate:', [...classes].sort());
```

### 2. Auto Converter

Convert HTML automatically:

```javascript
// auto-migrate.js
import { classMap } from './reedstyle.bridge.yaml';

function migrateElement(element) {
  const classes = [...element.classList];
  const reedAttrs = {};
  
  classes.forEach(cls => {
    if (classMap[cls]) {
      const [attr, value] = classMap[cls].split('=');
      reedAttrs[attr] = value;
    }
  });
  
  // Create reed element
  const reed = document.createElement('reed');
  Object.entries(reedAttrs).forEach(([key, val]) => {
    reed.setAttribute(key, val);
  });
  
  // Replace original
  reed.innerHTML = element.innerHTML;
  element.parentNode.replaceChild(reed, element);
}
```

### 3. Migration Validator

Check migration completeness:

```javascript
// validate-migration.js
function validateMigration() {
  const oldClasses = document.querySelectorAll('[class]');
  const reedElements = document.querySelectorAll('reed');
  
  console.log(`Old elements: ${oldClasses.length}`);
  console.log(`Reed elements: ${reedElements.length}`);
  
  if (oldClasses.length > 0) {
    console.warn('Still using old classes:', oldClasses);
  }
}
```

## Step-by-Step Migration

### Phase 1: Setup
1. Install ReedSTYLE alongside existing framework
2. Create `reedstyle.bridge.yaml`
3. Enable gradual migration mode

### Phase 2: Component Migration
1. Start with simple components (buttons, cards)
2. Map utility classes
3. Convert layouts
4. Migrate complex components

### Phase 3: Cleanup
1. Remove old framework dependencies
2. Disable bridge layer
3. Delete migration configs

## Testing During Migration

### Visual Regression Testing

```javascript
// Compare old vs new
const scenarios = [
  { old: '.btn', new: 'reed[as="button"]' },
  { old: '.card', new: 'reed[as="card"]' },
];

scenarios.forEach(({ old, new }) => {
  const oldEl = document.querySelector(old);
  const newEl = document.querySelector(new);
  
  // Compare computed styles
  const oldStyles = getComputedStyle(oldEl);
  const newStyles = getComputedStyle(newEl);
  
  // Check key properties
  ['padding', 'margin', 'background'].forEach(prop => {
    if (oldStyles[prop] !== newStyles[prop]) {
      console.warn(`Mismatch in ${prop}`);
    }
  });
});
```

## Common Issues

### Specificity Conflicts

```yaml
# Increase bridge layer specificity if needed
bridge:
  bootstrap:
    important: true  # Add !important to overrides
```

### JavaScript Dependencies

Some frameworks require JavaScript. Map to ReedSTYLE equivalents:

```javascript
// Old: Bootstrap modal
$('#modal').modal('show');

// New: ReedSTYLE modal
ReedStyle.modal.show('#modal');
```

### Missing Features

If ReedSTYLE doesn't have an equivalent:

```yaml
# Keep specific framework features
bridge:
  bootstrap:
    keep:
      - ".carousel"  # Keep Bootstrap carousel
      - ".tooltip"   # Keep Bootstrap tooltips
```

## Post-Migration

### Remove Bridge Layer

Once migration is complete:

```yaml
# reedstyle.config.yaml
config:
  # bridge: ./reedstyle.bridge.yaml  # Comment out or remove
```

### Optimize Build

```yaml
build:
  tree_shake: true  # Remove unused styles
  purge_bridge: true  # Remove bridge layer from production
```

## Resources

- [Bootstrap Migration Examples](https://reedstyle.dev/migrate/bootstrap)
- [Tailwind Migration Examples](https://reedstyle.dev/migrate/tailwind)
- [Migration CLI Tool](https://github.com/reedstyle/migrator)
- [Community Migrations](https://github.com/reedstyle/bridge-configs)