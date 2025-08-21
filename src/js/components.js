/**
 * ReedSTYLE Component System
 * Loads component definitions from YAML and applies them to r-s elements
 */

class ComponentSystem {
    constructor() {
        this.components = {};
        this.initialized = false;
    }

    /**
     * Initialize the component system
     */
    async init() {
        if (this.initialized) return;
        
        try {
            // Try to load components from YAML file
            await this.loadComponents();
            
            // Apply components to existing elements
            this.applyComponents();
            
            // Watch for new elements
            this.observeDOM();
            
            this.initialized = true;
            console.log('ReedSTYLE Component System initialized');
        } catch (error) {
            console.warn('Component system initialization failed:', error);
        }
    }

    /**
     * Load component definitions from YAML
     */
    async loadComponents() {
        try {
            // Try to fetch the components YAML file
            const response = await fetch('/reedstyle.components.yaml');
            if (!response.ok) {
                throw new Error('Components YAML not found');
            }
            
            const yamlText = await response.text();
            
            // Parse YAML (we need to include a YAML parser)
            // For now, we'll use a simple approach or expect the build process to convert to JSON
            this.components = this.parseYAML(yamlText);
            
        } catch (error) {
            // If YAML loading fails, try JSON fallback
            try {
                const response = await fetch('/reedstyle.components.json');
                if (response.ok) {
                    const data = await response.json();
                    this.components = data.components || {};
                }
            } catch (jsonError) {
                console.warn('No component definitions found');
            }
        }
    }

    /**
     * Simple YAML parser for component definitions
     * In production, you'd use a proper YAML library
     */
    parseYAML(yamlText) {
        const components = {};
        const lines = yamlText.split('\n');
        let currentComponent = null;
        let inComponents = false;
        
        for (let line of lines) {
            // Skip comments and empty lines
            if (line.trim().startsWith('#') || !line.trim()) continue;
            
            // Check for components section
            if (line.trim() === 'components:') {
                inComponents = true;
                continue;
            }
            
            if (!inComponents) continue;
            
            // Check indentation level
            const indent = line.search(/\S/);
            
            // Component name (2 spaces indent)
            if (indent === 2 && line.includes(':')) {
                const name = line.trim().replace(':', '');
                currentComponent = name;
                components[name] = {};
            }
            // Component properties (4 spaces indent)
            else if (indent === 4 && currentComponent && line.includes(':')) {
                const [key, ...valueParts] = line.trim().split(':');
                const value = valueParts.join(':').trim().replace(/^["']|["']$/g, '');
                
                // Handle special 'box' key rename
                const propKey = key.trim() === 'box' ? 'box_' : key.trim();
                components[currentComponent][propKey] = value;
            }
        }
        
        return components;
    }

    /**
     * Apply components to all matching r-s elements
     */
    applyComponents() {
        const elements = document.querySelectorAll('r-s[as]');
        
        elements.forEach(element => {
            const componentName = element.getAttribute('as');
            
            // Check if this is a component definition
            if (this.components[componentName]) {
                this.applyComponentToElement(element, componentName);
            }
        });
    }

    /**
     * Apply a component definition to an element
     */
    applyComponentToElement(element, componentName) {
        const component = this.resolveComponent(componentName);
        if (!component) return;
        
        // Don't override existing attributes
        // Components provide defaults that can be overridden
        
        // Apply namespace attributes if not already present
        if (component.box_ && !element.hasAttribute('box')) {
            element.setAttribute('box', component.box_);
        }
        
        if (component.face && !element.hasAttribute('face')) {
            element.setAttribute('face', component.face);
        }
        
        if (component.text && !element.hasAttribute('text')) {
            element.setAttribute('text', component.text);
        }
        
        if (component.layout && !element.hasAttribute('layout')) {
            element.setAttribute('layout', component.layout);
        }
        
        if (component.device && !element.hasAttribute('device')) {
            element.setAttribute('device', component.device);
        }
        
        if (component.fx && !element.hasAttribute('fx')) {
            element.setAttribute('fx', component.fx);
        }
        
        // Mark element as component-processed
        element.dataset.component = componentName;
    }

    /**
     * Resolve a component with inheritance
     */
    resolveComponent(componentName) {
        const component = this.components[componentName];
        if (!component) return null;
        
        // If component extends another, merge properties
        if (component.extends) {
            const baseComponent = this.resolveComponent(component.extends);
            if (baseComponent) {
                // Merge with base component (current component overrides base)
                return {
                    ...baseComponent,
                    ...component,
                    extends: undefined // Don't include extends in final result
                };
            }
        }
        
        return component;
    }

    /**
     * Observe DOM for new r-s elements
     */
    observeDOM() {
        const observer = new MutationObserver((mutations) => {
            mutations.forEach((mutation) => {
                mutation.addedNodes.forEach((node) => {
                    if (node.nodeType === 1) { // Element node
                        // Check if it's an r-s element with 'as' attribute
                        if (node.tagName === 'R-S' && node.hasAttribute('as')) {
                            const componentName = node.getAttribute('as');
                            if (this.components[componentName]) {
                                this.applyComponentToElement(node, componentName);
                            }
                        }
                        
                        // Also check children
                        const children = node.querySelectorAll?.('r-s[as]');
                        children?.forEach(child => {
                            const componentName = child.getAttribute('as');
                            if (this.components[componentName]) {
                                this.applyComponentToElement(child, componentName);
                            }
                        });
                    }
                });
            });
        });
        
        observer.observe(document.body, {
            childList: true,
            subtree: true
        });
    }
}

// Auto-initialize when DOM is ready
if (typeof window !== 'undefined') {
    window.ReedStyleComponents = new ComponentSystem();
    
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => {
            window.ReedStyleComponents.init();
        });
    } else {
        // DOM already loaded
        window.ReedStyleComponents.init();
    }
}

// Export for module usage
if (typeof module !== 'undefined' && module.exports) {
    module.exports = ComponentSystem;
}