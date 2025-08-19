use anyhow::Result;
use crate::config::ComponentsConfig;

pub fn generate(components: &ComponentsConfig) -> Result<String> {
    let mut js = String::new();
    
    // ReedSTYLE JavaScript initialization
    js.push_str("(function() {\n");
    js.push_str("  'use strict';\n\n");
    
    // Define custom element
    js.push_str("  class ReedElement extends HTMLElement {\n");
    js.push_str("    constructor() {\n");
    js.push_str("      super();\n");
    js.push_str("    }\n\n");
    
    js.push_str("    connectedCallback() {\n");
    js.push_str("      // Set proper display based on 'as' attribute\n");
    js.push_str("      const as = this.getAttribute('as');\n");
    js.push_str("      if (as) {\n");
    js.push_str("        // Map semantic elements to display types\n");
    js.push_str("        const displayMap = {\n");
    js.push_str("          'div': 'block',\n");
    js.push_str("          'span': 'inline',\n");
    js.push_str("          'article': 'block',\n");
    js.push_str("          'section': 'block',\n");
    js.push_str("          'header': 'block',\n");
    js.push_str("          'footer': 'block',\n");
    js.push_str("          'nav': 'block',\n");
    js.push_str("          'button': 'inline-block',\n");
    js.push_str("        };\n");
    js.push_str("        if (!this.style.display && displayMap[as]) {\n");
    js.push_str("          this.style.display = displayMap[as];\n");
    js.push_str("        }\n");
    js.push_str("      }\n");
    js.push_str("    }\n");
    js.push_str("  }\n\n");
    
    // Register custom element
    js.push_str("  // Register the reed element\n");
    js.push_str("  if (typeof customElements !== 'undefined' && !customElements.get('reed')) {\n");
    js.push_str("    customElements.define('reed', ReedElement);\n");
    js.push_str("  }\n\n");
    
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