use anyhow::Result;
use crate::config::ComponentsConfig;

pub fn generate(components: &ComponentsConfig) -> Result<String> {
    let mut js = String::new();
    
    // ReedSTYLE JavaScript initialization
    js.push_str("(function() {\n");
    js.push_str("  'use strict';\n\n");
    
    // Note: reed is not a custom element, just a regular HTML element styled with CSS
    js.push_str("  // Reed elements work purely through CSS - no custom element registration needed\n");
    js.push_str("  // The browser treats <reed> as an unknown element (like <div>) and our CSS handles the rest\n\n");
    
    // Global ReedStyle object with JSDoc type definitions
    js.push_str("  /**\n");
    js.push_str("   * @typedef {Object} ReedStyleAPI\n");
    js.push_str("   * @property {string} version - Current version of ReedSTYLE\n");
    js.push_str("   * @property {function(): void} init - Initialize ReedSTYLE\n");
    js.push_str("   * @property {Object.<string, boolean>} components - Available components\n");
    js.push_str("   * @property {EffectsEngine} [effects] - Effects engine instance\n");
    js.push_str("   * @property {TypographyEngine} [typography] - Typography engine instance\n");
    js.push_str("   */\n\n");
    
    js.push_str("  /**\n");
    js.push_str("   * @typedef {Object} EffectsEngine\n");
    js.push_str("   * @property {boolean} initialized - Whether effects are initialized\n");
    js.push_str("   * @property {function(): void} init - Initialize effects\n");
    js.push_str("   * @property {function(Event, HTMLElement): void} createRipple - Create ripple effect\n");
    js.push_str("   * @property {function(HTMLElement, string): void} applyStagger - Apply stagger animation\n");
    js.push_str("   */\n\n");
    
    js.push_str("  /**\n");
    js.push_str("   * @typedef {Object} TypographyEngine\n");
    js.push_str("   * @property {boolean} initialized - Whether typography is initialized\n");
    js.push_str("   * @property {function(): void} init - Initialize typography\n");
    js.push_str("   * @property {function(): void} processAll - Process all elements\n");
    js.push_str("   * @property {function(HTMLElement): void} processElement - Process single element\n");
    js.push_str("   * @property {function(HTMLElement): string} detectLanguage - Detect element language\n");
    js.push_str("   */\n\n");
    
    js.push_str("  /** @type {ReedStyleAPI} */\n");
    js.push_str("  window.ReedStyle = {\n");
    js.push_str("    version: '0.1.0',\n");
    js.push_str("    /** Initialize ReedSTYLE framework */\n");
    js.push_str("    init: async function() {\n");
    js.push_str("      console.log('ReedSTYLE initializing...');\n");
    js.push_str("      // Initialize color system FIRST (before CSS loads)\n");
    js.push_str("      await this.initColors();\n");
    js.push_str("      // Apply component definitions\n");
    js.push_str("      this.applyComponents();\n");
    js.push_str("      // Watch for new components\n");
    js.push_str("      this.observeComponents();\n");
    js.push_str("      console.log('ReedSTYLE initialized');\n");
    js.push_str("    },\n");
    
    // Add component definitions with full data
    js.push_str("    /** Component definitions from reedstyle.components.yaml */\n");
    js.push_str("    componentDefinitions: {\n");
    for (name, component) in &components.components {
        js.push_str(&format!("      '{}': {{\n", name));
        if let Some(element) = &component.element {
            js.push_str(&format!("        element: '{}',\n", element));
        }
        if let Some(extends) = &component.extends {
            js.push_str(&format!("        extends: '{}',\n", extends));
        }
        if let Some(box_attr) = &component.box_ {
            js.push_str(&format!("        box: '{}',\n", box_attr));
        }
        if let Some(face) = &component.face {
            js.push_str(&format!("        face: '{}',\n", face));
        }
        if let Some(text) = &component.text {
            js.push_str(&format!("        text: '{}',\n", text));
        }
        if let Some(layout) = &component.layout {
            js.push_str(&format!("        layout: '{}',\n", layout));
        }
        if let Some(device) = &component.device {
            js.push_str(&format!("        device: '{}',\n", device));
        }
        if let Some(fx) = &component.fx {
            js.push_str(&format!("        fx: '{}',\n", fx));
        }
        js.push_str("      },\n");
    }
    js.push_str("    },\n");
    
    // Add component system methods
    js.push_str("    /** Apply component definitions to elements */\n");
    js.push_str("    applyComponents: function() {\n");
    js.push_str("      const elements = document.querySelectorAll('r-s[as]');\n");
    js.push_str("      elements.forEach(element => {\n");
    js.push_str("        const componentName = element.getAttribute('as');\n");
    js.push_str("        const component = this.resolveComponent(componentName);\n");
    js.push_str("        if (component) {\n");
    js.push_str("          this.applyComponentToElement(element, component);\n");
    js.push_str("        }\n");
    js.push_str("      });\n");
    js.push_str("    },\n");
    
    js.push_str("    /** Resolve component with inheritance */\n");
    js.push_str("    resolveComponent: function(name) {\n");
    js.push_str("      const component = this.componentDefinitions[name];\n");
    js.push_str("      if (!component) return null;\n");
    js.push_str("      \n");
    js.push_str("      if (component.extends) {\n");
    js.push_str("        const base = this.resolveComponent(component.extends);\n");
    js.push_str("        if (base) {\n");
    js.push_str("          return Object.assign({}, base, component, {extends: undefined});\n");
    js.push_str("        }\n");
    js.push_str("      }\n");
    js.push_str("      return component;\n");
    js.push_str("    },\n");
    
    js.push_str("    /** Apply component to element */\n");
    js.push_str("    applyComponentToElement: function(element, component) {\n");
    js.push_str("      // Only apply if attribute not already present\n");
    js.push_str("      if (component.box && !element.hasAttribute('box')) {\n");
    js.push_str("        element.setAttribute('box', component.box);\n");
    js.push_str("      }\n");
    js.push_str("      if (component.face && !element.hasAttribute('face')) {\n");
    js.push_str("        element.setAttribute('face', component.face);\n");
    js.push_str("      }\n");
    js.push_str("      if (component.text && !element.hasAttribute('text')) {\n");
    js.push_str("        element.setAttribute('text', component.text);\n");
    js.push_str("      }\n");
    js.push_str("      if (component.layout && !element.hasAttribute('layout')) {\n");
    js.push_str("        element.setAttribute('layout', component.layout);\n");
    js.push_str("      }\n");
    js.push_str("      if (component.device && !element.hasAttribute('device')) {\n");
    js.push_str("        element.setAttribute('device', component.device);\n");
    js.push_str("      }\n");
    js.push_str("      if (component.fx && !element.hasAttribute('fx')) {\n");
    js.push_str("        element.setAttribute('fx', component.fx);\n");
    js.push_str("      }\n");
    js.push_str("    },\n");
    
    js.push_str("    /** Observe DOM for new components */\n");
    js.push_str("    observeComponents: function() {\n");
    js.push_str("      const observer = new MutationObserver((mutations) => {\n");
    js.push_str("        mutations.forEach((mutation) => {\n");
    js.push_str("          mutation.addedNodes.forEach((node) => {\n");
    js.push_str("            if (node.nodeType === 1 && node.tagName === 'R-S' && node.hasAttribute('as')) {\n");
    js.push_str("              const componentName = node.getAttribute('as');\n");
    js.push_str("              const component = this.resolveComponent(componentName);\n");
    js.push_str("              if (component) {\n");
    js.push_str("                this.applyComponentToElement(node, component);\n");
    js.push_str("              }\n");
    js.push_str("            }\n");
    js.push_str("            // Check children\n");
    js.push_str("            const children = node.querySelectorAll?.('r-s[as]');\n");
    js.push_str("            children?.forEach(child => {\n");
    js.push_str("              const componentName = child.getAttribute('as');\n");
    js.push_str("              const component = this.resolveComponent(componentName);\n");
    js.push_str("              if (component) {\n");
    js.push_str("                this.applyComponentToElement(child, component);\n");
    js.push_str("              }\n");
    js.push_str("            });\n");
    js.push_str("          });\n");
    js.push_str("        });\n");
    js.push_str("      });\n");
    js.push_str("      observer.observe(document.body, {childList: true, subtree: true});\n");
    js.push_str("    },\n");
    
    // Add Color Engine methods
    js.push_str("    /** Initialize color system from data-root or YAML */\n");
    js.push_str("    initColors: async function() {\n");
    js.push_str("      // Check for data-root attribute on body first (highest priority)\n");
    js.push_str("      const dataRoot = document.body.getAttribute('data-root');\n");
    js.push_str("      if (dataRoot) {\n");
    js.push_str("        this.parseDataRoot(dataRoot);\n");
    js.push_str("        return; // data-root overrides YAML\n");
    js.push_str("      }\n");
    js.push_str("      \n");
    js.push_str("      // Try to load YAML configuration file\n");
    js.push_str("      try {\n");
    js.push_str("        const response = await fetch('/reedstyle.colors.yaml');\n");
    js.push_str("        if (response.ok) {\n");
    js.push_str("          const yamlText = await response.text();\n");
    js.push_str("          this.parseYamlColors(yamlText);\n");
    js.push_str("        }\n");
    js.push_str("      } catch (e) {\n");
    js.push_str("        // No YAML file is fine - use defaults from CSS\n");
    js.push_str("        console.log('No custom colors defined (reedstyle.colors.yaml not found)');\n");
    js.push_str("      }\n");
    js.push_str("    },\n");
    
    js.push_str("    /** Parse YAML colors and apply them */\n");
    js.push_str("    parseYamlColors: function(yamlText) {\n");
    js.push_str("      // Simple YAML parser for color definitions\n");
    js.push_str("      const lines = yamlText.split('\\n');\n");
    js.push_str("      let inColors = false;\n");
    js.push_str("      \n");
    js.push_str("      // Create style element for color overrides - MUST be AFTER CSS!\n");
    js.push_str("      let styleEl = document.getElementById('reedstyle-colors');\n");
    js.push_str("      if (!styleEl) {\n");
    js.push_str("        styleEl = document.createElement('style');\n");
    js.push_str("        styleEl.id = 'reedstyle-colors';\n");
    js.push_str("        // Insert AFTER all link elements to override CSS\n");
    js.push_str("        const links = document.head.querySelectorAll('link[rel=\"stylesheet\"]');\n");
    js.push_str("        if (links.length > 0) {\n");
    js.push_str("          // Insert after last stylesheet\n");
    js.push_str("          links[links.length - 1].after(styleEl);\n");
    js.push_str("        } else {\n");
    js.push_str("          // No stylesheets yet, append to head\n");
    js.push_str("          document.head.appendChild(styleEl);\n");
    js.push_str("        }\n");
    js.push_str("      }\n");
    js.push_str("      \n");
    js.push_str("      let cssText = '@layer settings {\\n  :root {\\n';\n");
    js.push_str("      \n");
    js.push_str("      lines.forEach(line => {\n");
    js.push_str("        // Skip comments and empty lines\n");
    js.push_str("        if (line.trim().startsWith('#') || !line.trim()) return;\n");
    js.push_str("        \n");
    js.push_str("        // Check if we're in the colors section\n");
    js.push_str("        if (line.trim() === 'colors:') {\n");
    js.push_str("          inColors = true;\n");
    js.push_str("          return;\n");
    js.push_str("        }\n");
    js.push_str("        \n");
    js.push_str("        // Parse color definitions\n");
    js.push_str("        if (inColors && line.startsWith('  ')) {\n");
    js.push_str("          const match = line.match(/^\\s+(\\S+):\\s*[\"']?([^\"']+)[\"']?/);\n");
    js.push_str("          if (match) {\n");
    js.push_str("            const [, name, value] = match;\n");
    js.push_str("            const oklch = this.toOklch(value.trim());\n");
    js.push_str("            if (oklch) {\n");
    js.push_str("              // Generate 1-9 scale\n");
    js.push_str("              const scale = this.generateColorScale(oklch);\n");
    js.push_str("              // Add CSS variables to style text\n");
    js.push_str("              for (let i = 1; i <= 9; i++) {\n");
    js.push_str("                cssText += `    --rs-color-${name}-${i}: ${scale[i - 1]};\\n`;\n");
    js.push_str("              }\n");
    js.push_str("              // Also set base color (maps to scale-5)\n");
    js.push_str("              cssText += `    --rs-color-${name}: ${scale[4]};\\n`;\n");
    js.push_str("            }\n");
    js.push_str("          }\n");
    js.push_str("        }\n");
    js.push_str("      });\n");
    js.push_str("      \n");
    js.push_str("      cssText += '  }\\n}';\n");
    js.push_str("      styleEl.textContent = cssText;\n");
    js.push_str("      console.log('Injected YAML colors CSS:', cssText);\n");
    js.push_str("    },\n");
    
    js.push_str("    /** Parse data-root attribute and set colors */\n");
    js.push_str("    parseDataRoot: function(dataRoot) {\n");
    js.push_str("      console.log('Parsing data-root:', dataRoot);\n");
    js.push_str("      // Parse format: [brand-a:#FF6B6B, brand-b:rgb(78,205,196)]\n");
    js.push_str("      // More flexible pattern that handles spaces\n");
    js.push_str("      // Pattern that handles all color formats including spaces\n");
    js.push_str("      const colorPattern = /(brand-[a-f]|state-\\w+)\\s*:\\s*(#[0-9a-fA-F]{6}|rgb[a]?\\([^)]+\\)|hsl[a]?\\([^)]+\\)|oklch\\([^)]+\\))/gi;\n");
    js.push_str("      let match;\n");
    js.push_str("      \n");
    js.push_str("      // Debug: Try simple test first\n");
    js.push_str("      console.log('Testing pattern against:', dataRoot);\n");
    js.push_str("      console.log('Pattern test:', colorPattern.test(dataRoot));\n");
    js.push_str("      colorPattern.lastIndex = 0; // Reset after test\n");
    js.push_str("      \n");
    js.push_str("      // Create style element for color overrides - MUST be AFTER CSS!\n");
    js.push_str("      let styleEl = document.getElementById('reedstyle-colors');\n");
    js.push_str("      if (!styleEl) {\n");
    js.push_str("        styleEl = document.createElement('style');\n");
    js.push_str("        styleEl.id = 'reedstyle-colors';\n");
    js.push_str("        // Insert AFTER all link elements to override CSS\n");
    js.push_str("        const links = document.head.querySelectorAll('link[rel=\"stylesheet\"]');\n");
    js.push_str("        if (links.length > 0) {\n");
    js.push_str("          // Insert after last stylesheet\n");
    js.push_str("          links[links.length - 1].after(styleEl);\n");
    js.push_str("        } else {\n");
    js.push_str("          // No stylesheets yet, append to head\n");
    js.push_str("          document.head.appendChild(styleEl);\n");
    js.push_str("        }\n");
    js.push_str("      }\n");
    js.push_str("      \n");
    js.push_str("      let cssText = '@layer settings {\\n  :root {\\n';\n");
    js.push_str("      \n");
    js.push_str("      while ((match = colorPattern.exec(dataRoot)) !== null) {\n");
    js.push_str("        const [, name, value] = match;\n");
    js.push_str("        console.log(`Found color: ${name} = ${value}`);\n");
    js.push_str("        const oklch = this.toOklch(value);\n");
    js.push_str("        console.log(`OKLCH conversion:`, oklch);\n");
    js.push_str("        if (oklch) {\n");
    js.push_str("          // Generate 1-9 scale\n");
    js.push_str("          const scale = this.generateColorScale(oklch);\n");
    js.push_str("          console.log(`Generated scale for ${name}:`, scale);\n");
    js.push_str("          // Add CSS variables to style text\n");
    js.push_str("          for (let i = 1; i <= 9; i++) {\n");
    js.push_str("            cssText += `    --rs-color-${name}-${i}: ${scale[i - 1]};\\n`;\n");
    js.push_str("          }\n");
    js.push_str("          // Also set base color (maps to scale-5)\n");
    js.push_str("          cssText += `    --rs-color-${name}: ${scale[4]};\\n`;\n");
    js.push_str("        } else {\n");
    js.push_str("          console.error(`Failed to convert ${name} color: ${value}`);\n");
    js.push_str("        }\n");
    js.push_str("      }\n");
    js.push_str("      \n");
    js.push_str("      cssText += '  }\\n}';\n");
    js.push_str("      styleEl.textContent = cssText;\n");
    js.push_str("      console.log('Injected color CSS:', cssText);\n");
    js.push_str("    },\n");
    
    js.push_str("    /** Convert any color format to OKLCH */\n");
    js.push_str("    toOklch: function(color) {\n");
    js.push_str("      console.log('toOklch input:', color);\n");
    js.push_str("      let r, g, b;\n");
    js.push_str("      \n");
    js.push_str("      // Parse hex format\n");
    js.push_str("      if (color.startsWith('#')) {\n");
    js.push_str("        console.log('Detected HEX format');\n");
    js.push_str("        const hex = color.slice(1);\n");
    js.push_str("        r = parseInt(hex.substr(0, 2), 16) / 255;\n");
    js.push_str("        g = parseInt(hex.substr(2, 2), 16) / 255;\n");
    js.push_str("        b = parseInt(hex.substr(4, 2), 16) / 255;\n");
    js.push_str("      }\n");
    js.push_str("      // Parse rgb/rgba format\n");
    js.push_str("      else if (color.startsWith('rgb')) {\n");
    js.push_str("        const match = color.match(/rgba?\\(\\s*(\\d+)\\s*,\\s*(\\d+)\\s*,\\s*(\\d+)/);\n");
    js.push_str("        if (match) {\n");
    js.push_str("          r = parseInt(match[1]) / 255;\n");
    js.push_str("          g = parseInt(match[2]) / 255;\n");
    js.push_str("          b = parseInt(match[3]) / 255;\n");
    js.push_str("        }\n");
    js.push_str("      }\n");
    js.push_str("      // Parse hsl/hsla format\n");
    js.push_str("      else if (color.startsWith('hsl')) {\n");
    js.push_str("        const match = color.match(/hsla?\\(\\s*(\\d+)\\s*,\\s*(\\d+)%\\s*,\\s*(\\d+)%/);\n");
    js.push_str("        if (match) {\n");
    js.push_str("          const h = parseInt(match[1]) / 360;\n");
    js.push_str("          const s = parseInt(match[2]) / 100;\n");
    js.push_str("          const l = parseInt(match[3]) / 100;\n");
    js.push_str("          \n");
    js.push_str("          // HSL to RGB conversion\n");
    js.push_str("          const c = (1 - Math.abs(2 * l - 1)) * s;\n");
    js.push_str("          const x = c * (1 - Math.abs((h * 6) % 2 - 1));\n");
    js.push_str("          const m = l - c / 2;\n");
    js.push_str("          \n");
    js.push_str("          if (h < 1/6) { r = c + m; g = x + m; b = m; }\n");
    js.push_str("          else if (h < 2/6) { r = x + m; g = c + m; b = m; }\n");
    js.push_str("          else if (h < 3/6) { r = m; g = c + m; b = x + m; }\n");
    js.push_str("          else if (h < 4/6) { r = m; g = x + m; b = c + m; }\n");
    js.push_str("          else if (h < 5/6) { r = x + m; g = m; b = c + m; }\n");
    js.push_str("          else { r = c + m; g = m; b = x + m; }\n");
    js.push_str("        }\n");
    js.push_str("      }\n");
    js.push_str("      // Parse oklch format (pass through)\n");
    js.push_str("      else if (color.startsWith('oklch')) {\n");
    js.push_str("        const match = color.match(/oklch\\((\\d+(?:\\.\\d+)?)%\\s+(\\d+(?:\\.\\d+)?)\\s+(\\d+(?:\\.\\d+)?)/);\n");
    js.push_str("        if (match) {\n");
    js.push_str("          return {\n");
    js.push_str("            l: parseFloat(match[1]) / 100,\n");
    js.push_str("            c: parseFloat(match[2]),\n");
    js.push_str("            h: parseFloat(match[3])\n");
    js.push_str("          };\n");
    js.push_str("        }\n");
    js.push_str("      }\n");
    js.push_str("      else {\n");
    js.push_str("        return null;\n");
    js.push_str("      }\n");
    js.push_str("      \n");
    js.push_str("      // RGB to OKLCH conversion (simplified but functional)\n");
    js.push_str("      // Linear RGB\n");
    js.push_str("      const toLinear = (c) => c <= 0.04045 ? c / 12.92 : Math.pow((c + 0.055) / 1.055, 2.4);\n");
    js.push_str("      const rLin = toLinear(r);\n");
    js.push_str("      const gLin = toLinear(g);\n");
    js.push_str("      const bLin = toLinear(b);\n");
    js.push_str("      \n");
    js.push_str("      // Approximate OKLAB conversion\n");
    js.push_str("      const l = 0.4122214708 * rLin + 0.5363325363 * gLin + 0.0514459929 * bLin;\n");
    js.push_str("      const m = 0.2119034982 * rLin + 0.6806995451 * gLin + 0.1073969566 * bLin;\n");
    js.push_str("      const s = 0.0883024619 * rLin + 0.2817188376 * gLin + 0.6299787005 * bLin;\n");
    js.push_str("      \n");
    js.push_str("      const lRoot = Math.cbrt(l);\n");
    js.push_str("      const mRoot = Math.cbrt(m);\n");
    js.push_str("      const sRoot = Math.cbrt(s);\n");
    js.push_str("      \n");
    js.push_str("      const L = 0.2104542553 * lRoot + 0.7936177850 * mRoot - 0.0040720468 * sRoot;\n");
    js.push_str("      const a = 1.9779984951 * lRoot - 2.4285922050 * mRoot + 0.4505937099 * sRoot;\n");
    js.push_str("      const b_val = 0.0259040371 * lRoot + 0.7827717662 * mRoot - 0.8086757660 * sRoot;\n");
    js.push_str("      \n");
    js.push_str("      // Convert to LCH\n");
    js.push_str("      const C = Math.sqrt(a * a + b_val * b_val);\n");
    js.push_str("      let H = Math.atan2(b_val, a) * 180 / Math.PI;\n");
    js.push_str("      if (H < 0) H += 360;\n");
    js.push_str("      \n");
    js.push_str("      return { l: L, c: C, h: H };\n");
    js.push_str("    },\n");
    
    js.push_str("    /** Generate 1-9 color scale from OKLCH */\n");
    js.push_str("    generateColorScale: function(oklch) {\n");
    js.push_str("      const scale = [];\n");
    js.push_str("      const targetLightness = [0.95, 0.85, 0.75, 0.65, 0.55, 0.45, 0.35, 0.25, 0.15];\n");
    js.push_str("      \n");
    js.push_str("      for (let i = 0; i < 9; i++) {\n");
    js.push_str("        const l = targetLightness[i];\n");
    js.push_str("        // Reduce chroma at extremes\n");
    js.push_str("        let chromaFactor = 1;\n");
    js.push_str("        if (i === 0) chromaFactor = 0.1;\n");
    js.push_str("        else if (i === 1) chromaFactor = 0.3;\n");
    js.push_str("        else if (i === 2) chromaFactor = 0.5;\n");
    js.push_str("        else if (i >= 6) chromaFactor = 0.8 - (i - 6) * 0.2;\n");
    js.push_str("        \n");
    js.push_str("        const c = oklch.c * chromaFactor;\n");
    js.push_str("        scale.push(`oklch(${(l * 100).toFixed(2)}% ${c.toFixed(3)} ${oklch.h.toFixed(1)})`);\n");
    js.push_str("      }\n");
    js.push_str("      return scale;\n");
    js.push_str("    }\n");
    
    js.push_str("  };\n\n");
    
    // Typography Engine Integration
    js.push_str("  // Typography Engine\n");
    js.push_str("  const TYPO_CHARS = {\n");
    js.push_str("    LDQUO_DE: '\\u201E', RDQUO_DE: '\\u201C',\n");
    js.push_str("    LSQUO_DE: '\\u201A', RSQUO_DE: '\\u2018',\n");
    js.push_str("    LDQUO: '\\u201C', RDQUO: '\\u201D',\n");
    js.push_str("    LSQUO: '\\u2018', RSQUO: '\\u2019',\n");
    js.push_str("    LAQUO: '\\u00AB', RAQUO: '\\u00BB',\n");
    js.push_str("    NBSP: '\\u00A0', NNBSP: '\\u202F',\n");
    js.push_str("    ELLIPSIS: '\\u2026', ENDASH: '\\u2013', EMDASH: '\\u2014'\n");
    js.push_str("  };\n\n");
    
    js.push_str("  /** Typography engine for smart quotes and typographic enhancements */\n");
    js.push_str("  class TypographyEngine {\n");
    js.push_str("    constructor() {\n");
    js.push_str("      this.initialized = false;\n");
    js.push_str("      this.rules = this.initRules();\n");
    js.push_str("    }\n\n");
    
    js.push_str("    initRules() {\n");
    js.push_str("      return {\n");
    js.push_str("        'de': {\n");
    js.push_str("          quotes: [TYPO_CHARS.LDQUO_DE, TYPO_CHARS.RDQUO_DE],\n");
    js.push_str("          singleQuotes: [TYPO_CHARS.LSQUO_DE, TYPO_CHARS.RSQUO_DE],\n");
    js.push_str("          replacements: [\n");
    js.push_str("            [/\\.\\.\\./g, TYPO_CHARS.ELLIPSIS],\n");
    js.push_str("            [/--/g, TYPO_CHARS.EMDASH],\n");
    js.push_str("            [/ - /g, ` ${TYPO_CHARS.ENDASH} `]\n");
    js.push_str("          ]\n");
    js.push_str("        },\n");
    js.push_str("        'en': {\n");
    js.push_str("          quotes: [TYPO_CHARS.LDQUO, TYPO_CHARS.RDQUO],\n");
    js.push_str("          singleQuotes: [TYPO_CHARS.LSQUO, TYPO_CHARS.RSQUO],\n");
    js.push_str("          replacements: [\n");
    js.push_str("            [/\\.\\.\\./g, TYPO_CHARS.ELLIPSIS],\n");
    js.push_str("            [/--/g, TYPO_CHARS.EMDASH],\n");
    js.push_str("            [/ - /g, ` ${TYPO_CHARS.ENDASH} `]\n");
    js.push_str("          ]\n");
    js.push_str("        },\n");
    js.push_str("        'fr': {\n");
    js.push_str("          quotes: [`${TYPO_CHARS.LAQUO} `, ` ${TYPO_CHARS.RAQUO}`],\n");
    js.push_str("          singleQuotes: [TYPO_CHARS.LDQUO, TYPO_CHARS.RDQUO],\n");
    js.push_str("          replacements: [\n");
    js.push_str("            [/\\.\\.\\./g, TYPO_CHARS.ELLIPSIS],\n");
    js.push_str("            [/--/g, ` ${TYPO_CHARS.EMDASH} `]\n");
    js.push_str("          ]\n");
    js.push_str("        }\n");
    js.push_str("      };\n");
    js.push_str("    }\n\n");
    
    js.push_str("    init() {\n");
    js.push_str("      if (this.initialized) return;\n");
    js.push_str("      this.processAll();\n");
    js.push_str("      this.observeChanges();\n");
    js.push_str("      this.initialized = true;\n");
    js.push_str("      console.log('Typography Engine initialized');\n");
    js.push_str("    }\n\n");
    
    js.push_str("    processAll() {\n");
    js.push_str("      const elements = document.querySelectorAll('r-s[text*=\"filter:\"]');\n");
    js.push_str("      elements.forEach(element => this.processElement(element));\n");
    js.push_str("    }\n\n");
    
    js.push_str("    processElement(element) {\n");
    js.push_str("      const textAttr = element.getAttribute('text');\n");
    js.push_str("      if (!textAttr) return;\n");
    js.push_str("      \n");
    js.push_str("      const match = textAttr.match(/filter:(\\w+)/);\n");
    js.push_str("      const filter = match ? match[1] : null;\n");
    js.push_str("      if (!filter) return;\n");
    js.push_str("      \n");
    js.push_str("      const lang = this.detectLanguage(element);\n");
    js.push_str("      const rules = this.rules[lang] || this.rules['en'];\n");
    js.push_str("      \n");
    js.push_str("      this.applyTypography(element, rules, filter);\n");
    js.push_str("    }\n\n");
    
    js.push_str("    detectLanguage(element) {\n");
    js.push_str("      let current = element;\n");
    js.push_str("      while (current) {\n");
    js.push_str("        if (current.lang) return current.lang.split('-')[0];\n");
    js.push_str("        current = current.parentElement;\n");
    js.push_str("      }\n");
    js.push_str("      return document.documentElement.lang?.split('-')[0] || 'en';\n");
    js.push_str("    }\n\n");
    
    js.push_str("    applyTypography(element, rules, filter) {\n");
    js.push_str("      const walker = document.createTreeWalker(\n");
    js.push_str("        element,\n");
    js.push_str("        NodeFilter.SHOW_TEXT,\n");
    js.push_str("        {\n");
    js.push_str("          acceptNode: function(node) {\n");
    js.push_str("            const parent = node.parentElement;\n");
    js.push_str("            if (parent.tagName === 'SCRIPT' || parent.tagName === 'STYLE') {\n");
    js.push_str("              return NodeFilter.FILTER_REJECT;\n");
    js.push_str("            }\n");
    js.push_str("            return NodeFilter.FILTER_ACCEPT;\n");
    js.push_str("          }\n");
    js.push_str("        }\n");
    js.push_str("      );\n");
    js.push_str("      \n");
    js.push_str("      const textNodes = [];\n");
    js.push_str("      let node;\n");
    js.push_str("      while (node = walker.nextNode()) {\n");
    js.push_str("        textNodes.push(node);\n");
    js.push_str("      }\n");
    js.push_str("      \n");
    js.push_str("      textNodes.forEach(node => {\n");
    js.push_str("        let text = node.textContent;\n");
    js.push_str("        \n");
    js.push_str("        if (filter === 'smart' || filter === 'professional') {\n");
    js.push_str("          // Apply quotes\n");
    js.push_str("          if (rules.quotes) {\n");
    js.push_str("            const [open, close] = rules.quotes;\n");
    js.push_str("            text = text.replace(/(^|\\s)\"([^\"]+)\"/g, `$1${open}$2${close}`);\n");
    js.push_str("          }\n");
    js.push_str("          if (rules.singleQuotes) {\n");
    js.push_str("            const [open, close] = rules.singleQuotes;\n");
    js.push_str("            text = text.replace(/(^|\\s)'([^']+)'/g, `$1${open}$2${close}`);\n");
    js.push_str("          }\n");
    js.push_str("          // Apply replacements\n");
    js.push_str("          if (rules.replacements) {\n");
    js.push_str("            rules.replacements.forEach(([pattern, replacement]) => {\n");
    js.push_str("              text = text.replace(pattern, replacement);\n");
    js.push_str("            });\n");
    js.push_str("          }\n");
    js.push_str("        }\n");
    js.push_str("        \n");
    js.push_str("        if (text !== node.textContent) {\n");
    js.push_str("          node.textContent = text;\n");
    js.push_str("        }\n");
    js.push_str("      });\n");
    js.push_str("    }\n\n");
    
    js.push_str("    observeChanges() {\n");
    js.push_str("      const observer = new MutationObserver(mutations => {\n");
    js.push_str("        mutations.forEach(mutation => {\n");
    js.push_str("          mutation.addedNodes.forEach(node => {\n");
    js.push_str("            if (node.nodeType === 1) {\n");
    js.push_str("              if (node.tagName === 'R-S' && node.getAttribute('text')?.includes('filter:')) {\n");
    js.push_str("                this.processElement(node);\n");
    js.push_str("              }\n");
    js.push_str("              const children = node.querySelectorAll?.('r-s[text*=\"filter:\"]');\n");
    js.push_str("              children?.forEach(child => this.processElement(child));\n");
    js.push_str("            }\n");
    js.push_str("          });\n");
    js.push_str("        });\n");
    js.push_str("      });\n");
    js.push_str("      \n");
    js.push_str("      observer.observe(document.body, {\n");
    js.push_str("        childList: true,\n");
    js.push_str("        subtree: true\n");
    js.push_str("      });\n");
    js.push_str("    }\n");
    js.push_str("  }\n\n");
    
    // Effects Engine
    js.push_str("  // Effects Engine\n");
    js.push_str("  /** Effects engine for animations and interactions */\n");
    js.push_str("  class EffectsEngine {\n");
    js.push_str("    constructor() {\n");
    js.push_str("      this.observers = new Map();\n");
    js.push_str("      this.init();\n");
    js.push_str("    }\n\n");
    
    js.push_str("    init() {\n");
    js.push_str("      this.initClickEffects();\n");
    js.push_str("      this.initScrollEffects();\n");
    js.push_str("    }\n\n");
    
    // Click effects
    js.push_str("    initClickEffects() {\n");
    js.push_str("      document.addEventListener('click', (e) => {\n");
    js.push_str("        const reed = e.target.closest('r-s[fx*=\"click:\"]');\n");
    js.push_str("        if (!reed) return;\n");
    js.push_str("        const fx = reed.getAttribute('fx');\n");
    js.push_str("        if (fx.includes('click:ripple')) {\n");
    js.push_str("          this.createRipple(e, reed);\n");
    js.push_str("        }\n");
    js.push_str("      });\n");
    js.push_str("    }\n\n");
    
    // Ripple effect
    js.push_str("    createRipple(event, element) {\n");
    js.push_str("      const ripple = document.createElement('span');\n");
    js.push_str("      ripple.className = 'r-s-ripple';\n");
    js.push_str("      const rect = element.getBoundingClientRect();\n");
    js.push_str("      const x = event.clientX - rect.left;\n");
    js.push_str("      const y = event.clientY - rect.top;\n");
    js.push_str("      ripple.style.cssText = `\n");
    js.push_str("        position: absolute;\n");
    js.push_str("        left: ${x}px;\n");
    js.push_str("        top: ${y}px;\n");
    js.push_str("        transform: translate(-50%, -50%);\n");
    js.push_str("        width: 20px;\n");
    js.push_str("        height: 20px;\n");
    js.push_str("        border-radius: 50%;\n");
    js.push_str("        background: rgba(255, 255, 255, 0.5);\n");
    js.push_str("        animation: ripple 600ms ease-out;\n");
    js.push_str("        pointer-events: none;\n");
    js.push_str("      `;\n");
    js.push_str("      element.appendChild(ripple);\n");
    js.push_str("      setTimeout(() => ripple.remove(), 600);\n");
    js.push_str("    }\n\n");
    
    // Scroll effects
    js.push_str("    initScrollEffects() {\n");
    js.push_str("      const elements = document.querySelectorAll('r-s[fx*=\"scroll:\"]');\n");
    js.push_str("      if (elements.length === 0) return;\n\n");
    js.push_str("      console.log('Found', elements.length, 'elements with scroll effects');\n\n");
    js.push_str("      const observer = new IntersectionObserver(\n");
    js.push_str("        (entries) => {\n");
    js.push_str("          entries.forEach(entry => {\n");
    js.push_str("            if (entry.isIntersecting) {\n");
    js.push_str("              const fx = entry.target.getAttribute('fx');\n");
    js.push_str("              console.log('Element in view:', fx);\n");
    js.push_str("              entry.target.classList.add('in-view');\n");
    js.push_str("              \n");
    js.push_str("              // Handle stagger\n");
    js.push_str("              if (fx && fx.includes('stagger:')) {\n");
    js.push_str("                this.applyStagger(entry.target, fx);\n");
    js.push_str("              }\n");
    js.push_str("              \n");
    js.push_str("              // Unobserve if not repeating\n");
    js.push_str("              if (!fx || !fx.includes('repeat:true')) {\n");
    js.push_str("                observer.unobserve(entry.target);\n");
    js.push_str("              }\n");
    js.push_str("            } else {\n");
    js.push_str("              const fx = entry.target.getAttribute('fx');\n");
    js.push_str("              if (fx && fx.includes('repeat:true')) {\n");
    js.push_str("                entry.target.classList.remove('in-view');\n");
    js.push_str("              }\n");
    js.push_str("            }\n");
    js.push_str("          });\n");
    js.push_str("        },\n");
    js.push_str("        {\n");
    js.push_str("          threshold: 0.1,\n");
    js.push_str("          rootMargin: '0px'\n");
    js.push_str("        }\n");
    js.push_str("      );\n\n");
    js.push_str("      elements.forEach(el => {\n");
    js.push_str("        observer.observe(el);\n");
    js.push_str("        console.log('Observing element:', el.getAttribute('fx'));\n");
    js.push_str("      });\n");
    js.push_str("    }\n\n");
    
    // Stagger support
    js.push_str("    applyStagger(element, fx) {\n");
    js.push_str("      const siblings = [...element.parentElement.children].filter(el => \n");
    js.push_str("        el.tagName === 'R-S' && el.getAttribute('fx')?.includes('stagger:')\n");
    js.push_str("      );\n");
    js.push_str("      const index = siblings.indexOf(element);\n");
    js.push_str("      const staggerMatch = fx.match(/stagger:(\\w+)/);\n");
    js.push_str("      const delays = {\n");
    js.push_str("        'tiny': 50,\n");
    js.push_str("        'small': 100,\n");
    js.push_str("        'normal': 200,\n");
    js.push_str("        'large': 300\n");
    js.push_str("      };\n");
    js.push_str("      const delay = staggerMatch ? delays[staggerMatch[1]] || 100 : 100;\n");
    js.push_str("      element.style.animationDelay = `${index * delay}ms`;\n");
    js.push_str("    }\n");
    js.push_str("  }\n\n");
    
    // Add CSS for ripple effect
    js.push_str("  // Inject CSS for dynamic effects\n");
    js.push_str("  const style = document.createElement('style');\n");
    js.push_str("  style.textContent = `\n");
    js.push_str("    .r-s-ripple {\n");
    js.push_str("      position: absolute;\n");
    js.push_str("      border-radius: 50%;\n");
    js.push_str("      background: rgba(255, 255, 255, 0.6);\n");
    js.push_str("      transform: translate(-50%, -50%);\n");
    js.push_str("      pointer-events: none;\n");
    js.push_str("      animation: ripple 600ms ease-out;\n");
    js.push_str("    }\n");
    js.push_str("    @keyframes ripple {\n");
    js.push_str("      to {\n");
    js.push_str("        width: 200px;\n");
    js.push_str("        height: 200px;\n");
    js.push_str("        opacity: 0;\n");
    js.push_str("      }\n");
    js.push_str("    }\n");
    js.push_str("  `;\n");
    js.push_str("  document.head.appendChild(style);\n\n");
    
    // Initialize effects and typography engines
    js.push_str("  // Initialize effects and typography when ReedStyle initializes\n");
    js.push_str("  const originalInit = window.ReedStyle.init;\n");
    js.push_str("  window.ReedStyle.init = function() {\n");
    js.push_str("    originalInit.call(this);\n");
    js.push_str("    window.ReedStyle.effects = new EffectsEngine();\n");
    js.push_str("    window.ReedStyle.typography = new TypographyEngine();\n");
    js.push_str("    window.ReedStyle.typography.init();\n");
    js.push_str("    console.log('Effects Engine initialized');\n");
    js.push_str("  };\n\n");
    
    // Auto-init with immediate color setup
    js.push_str("  // Auto-initialize - colors MUST be set before CSS loads!\n");
    js.push_str("  (function() {\n");
    js.push_str("    // Immediately set colors if body exists\n");
    js.push_str("    if (document.body) {\n");
    js.push_str("      const dataRoot = document.body.getAttribute('data-root');\n");
    js.push_str("      if (dataRoot) {\n");
    js.push_str("        console.log('Immediate color init with data-root:', dataRoot);\n");
    js.push_str("        window.ReedStyle.parseDataRoot(dataRoot);\n");
    js.push_str("      }\n");
    js.push_str("    }\n");
    js.push_str("    \n");
    js.push_str("    // Full init when DOM ready\n");
    js.push_str("    if (document.readyState === 'loading') {\n");
    js.push_str("      document.addEventListener('DOMContentLoaded', () => window.ReedStyle.init());\n");
    js.push_str("    } else {\n");
    js.push_str("      window.ReedStyle.init();\n");
    js.push_str("    }\n");
    js.push_str("  })();\n");
    
    js.push_str("})();\n");
    
    Ok(js)
}