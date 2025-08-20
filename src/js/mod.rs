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
    js.push_str("    init: function() {\n");
    js.push_str("      console.log('ReedSTYLE initialized');\n");
    js.push_str("      // Apply component definitions\n");
    js.push_str("      this.applyComponents();\n");
    js.push_str("      // Watch for new components\n");
    js.push_str("      this.observeComponents();\n");
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
    
    // Auto-init
    js.push_str("  // Auto-initialize\n");
    js.push_str("  if (document.readyState === 'loading') {\n");
    js.push_str("    document.addEventListener('DOMContentLoaded', window.ReedStyle.init);\n");
    js.push_str("  } else {\n");
    js.push_str("    window.ReedStyle.init();\n");
    js.push_str("  }\n");
    
    js.push_str("})();\n");
    
    Ok(js)
}