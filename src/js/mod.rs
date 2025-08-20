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
    
    // Global ReedStyle object
    js.push_str("  // Global ReedStyle object\n");
    js.push_str("  window.ReedStyle = {\n");
    js.push_str("    version: '0.1.0',\n");
    js.push_str("    init: function() {\n");
    js.push_str("      console.log('ReedSTYLE initialized');\n");
    js.push_str("    },\n");
    
    // Add component definitions
    js.push_str("    components: {\n");
    for (name, _component) in &components.components {
        js.push_str(&format!("      '{}': true,\n", name));
    }
    js.push_str("    }\n");
    js.push_str("  };\n\n");
    
    // Effects Engine
    js.push_str("  // Effects Engine\n");
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
    js.push_str("        const reed = e.target.closest('reed[fx*=\"click:\"]');\n");
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
    js.push_str("      ripple.className = 'reed-ripple';\n");
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
    js.push_str("      const elements = document.querySelectorAll('reed[fx*=\"scroll:\"]');\n");
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
    js.push_str("        el.tagName === 'REED' && el.getAttribute('fx')?.includes('stagger:')\n");
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
    js.push_str("    .reed-ripple {\n");
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
    
    // Initialize effects engine
    js.push_str("  // Initialize effects when ReedStyle initializes\n");
    js.push_str("  const originalInit = window.ReedStyle.init;\n");
    js.push_str("  window.ReedStyle.init = function() {\n");
    js.push_str("    originalInit.call(this);\n");
    js.push_str("    window.ReedStyle.effects = new EffectsEngine();\n");
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