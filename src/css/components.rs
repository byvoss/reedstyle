use anyhow::Result;
use std::collections::HashMap;

/// Generate CSS for user-defined components from YAML config
/// This creates the actual styles from namespace attributes
pub fn generate_component_styles(components: &crate::config::ComponentsConfig) -> Result<String> {
    let mut css = String::new();
    
    css.push_str("\n  /* ========== Component Styles ========== */\n");
    
    // Process component inheritance first to resolve extends
    let resolved_components = resolve_inheritance(&components.components)?;
    
    for (name, component) in resolved_components {
        // Start component rule
        css.push_str(&format!("\n  /* Component: {} */\n", name));
        css.push_str(&format!("  r-s[as=\"{}\"] {{\n", name));
        
        // Parse and apply each namespace's properties
        if let Some(box_attr) = &component.box_ {
            css.push_str(&parse_box_properties(box_attr)?);
        }
        
        if let Some(face_attr) = &component.face {
            css.push_str(&parse_face_properties(face_attr)?);
        }
        
        if let Some(text_attr) = &component.text {
            css.push_str(&parse_text_properties(text_attr)?);
        }
        
        if let Some(layout_attr) = &component.layout {
            css.push_str(&parse_layout_properties(layout_attr)?);
        }
        
        if let Some(device_attr) = &component.device {
            css.push_str(&parse_device_properties(device_attr)?);
        }
        
        // FX namespace needs special handling for pseudo-classes
        if let Some(fx_attr) = &component.fx {
            // Close the main rule first
            css.push_str("  }\n");
            // Add FX rules separately
            css.push_str(&parse_fx_properties(&name, fx_attr)?);
        } else {
            css.push_str("  }\n");
        }
    }
    
    Ok(css)
}

/// Resolve component inheritance (extends field)
fn resolve_inheritance(components: &HashMap<String, crate::config::Component>) -> Result<HashMap<String, crate::config::Component>> {
    let mut resolved: HashMap<String, crate::config::Component> = HashMap::new();
    
    for (name, component) in components {
        let mut final_component = component.clone();
        
        // If component extends another, merge properties
        if let Some(base_name) = &component.extends {
            if let Some(base_component) = components.get(base_name) {
                // Merge base component properties (child overrides parent)
                if final_component.element.is_none() {
                    final_component.element = base_component.element.clone();
                }
                if final_component.box_.is_none() {
                    final_component.box_ = base_component.box_.clone();
                }
                if final_component.face.is_none() {
                    final_component.face = base_component.face.clone();
                }
                if final_component.text.is_none() {
                    final_component.text = base_component.text.clone();
                }
                if final_component.layout.is_none() {
                    final_component.layout = base_component.layout.clone();
                }
                if final_component.device.is_none() {
                    final_component.device = base_component.device.clone();
                }
                if final_component.fx.is_none() {
                    final_component.fx = base_component.fx.clone();
                }
            }
        }
        
        resolved.insert(name.clone(), final_component);
    }
    
    Ok(resolved)
}

/// Parse box namespace properties
fn parse_box_properties(attr: &str) -> Result<String> {
    let mut css = String::new();
    let properties = parse_attribute_string(attr)?;
    
    for (key, value) in properties {
        match key.as_str() {
            "padding" => css.push_str(&format!("    padding: {};\n", to_rem_value(&value))),
            "padding-x" => {
                let rem = to_rem_value(&value);
                css.push_str(&format!("    padding-left: {};\n", rem));
                css.push_str(&format!("    padding-right: {};\n", rem));
            },
            "padding-y" => {
                let rem = to_rem_value(&value);
                css.push_str(&format!("    padding-top: {};\n", rem));
                css.push_str(&format!("    padding-bottom: {};\n", rem));
            },
            "padding-top" => css.push_str(&format!("    padding-top: {};\n", to_rem_value(&value))),
            "padding-right" => css.push_str(&format!("    padding-right: {};\n", to_rem_value(&value))),
            "padding-bottom" => css.push_str(&format!("    padding-bottom: {};\n", to_rem_value(&value))),
            "padding-left" => css.push_str(&format!("    padding-left: {};\n", to_rem_value(&value))),
            
            "margin" => css.push_str(&format!("    margin: {};\n", to_rem_value(&value))),
            "margin-x" => {
                if value == "auto" {
                    css.push_str("    margin-left: auto;\n");
                    css.push_str("    margin-right: auto;\n");
                } else {
                    let rem = to_rem_value(&value);
                    css.push_str(&format!("    margin-left: {};\n", rem));
                    css.push_str(&format!("    margin-right: {};\n", rem));
                }
            },
            "margin-y" => {
                let rem = to_rem_value(&value);
                css.push_str(&format!("    margin-top: {};\n", rem));
                css.push_str(&format!("    margin-bottom: {};\n", rem));
            },
            "margin-top" => css.push_str(&format!("    margin-top: {};\n", to_rem_value(&value))),
            "margin-right" => css.push_str(&format!("    margin-right: {};\n", to_rem_value(&value))),
            "margin-bottom" => css.push_str(&format!("    margin-bottom: {};\n", to_rem_value(&value))),
            "margin-left" => css.push_str(&format!("    margin-left: {};\n", to_rem_value(&value))),
            
            "width" => {
                if value == "full" {
                    css.push_str("    width: 100%;\n");
                } else {
                    css.push_str(&format!("    width: {};\n", to_size_value(&value)));
                }
            },
            "max-width" => css.push_str(&format!("    max-width: {}px;\n", value)),
            "min-width" => css.push_str(&format!("    min-width: {}px;\n", value)),
            
            "height" => {
                if value == "screen" {
                    css.push_str("    min-height: 100vh;\n");
                } else if value == "full" {
                    css.push_str("    height: 100%;\n");
                } else {
                    css.push_str(&format!("    height: {};\n", to_size_value(&value)));
                }
            },
            
            _ => {} // Ignore unknown properties
        }
    }
    
    Ok(css)
}

/// Parse face namespace properties
fn parse_face_properties(attr: &str) -> Result<String> {
    let mut css = String::new();
    let properties = parse_attribute_string(attr)?;
    
    for (key, value) in properties {
        match key.as_str() {
            "bg" => {
                if value.starts_with("base-") || value.starts_with("brand-") || value.starts_with("state-") {
                    css.push_str(&format!("    background: var(--rs-{});\n", value));
                } else if value == "transparent" {
                    css.push_str("    background: transparent;\n");
                } else {
                    css.push_str(&format!("    background: {};\n", value));
                }
            },
            "radius" => {
                let radius_value = match value.as_str() {
                    "none" => "0",
                    "sm" => "0.125rem",
                    "md" => "0.375rem",
                    "lg" => "0.5rem",
                    "xl" => "0.75rem",
                    "2xl" => "1rem",
                    "3xl" => "1.5rem",
                    "full" => "9999px",
                    _ => &value,
                };
                css.push_str(&format!("    border-radius: {};\n", radius_value));
            },
            "shadow" => {
                let shadow_value = match value.as_str() {
                    "none" => "none",
                    "sm" => "0 1px 2px 0 rgb(0 0 0 / 0.05)",
                    "md" => "0 4px 6px -1px rgb(0 0 0 / 0.1)",
                    "lg" => "0 10px 15px -3px rgb(0 0 0 / 0.1)",
                    "xl" => "0 20px 25px -5px rgb(0 0 0 / 0.1)",
                    "2xl" => "0 25px 50px -12px rgb(0 0 0 / 0.25)",
                    _ => &value,
                };
                css.push_str(&format!("    box-shadow: {};\n", shadow_value));
            },
            "border" => {
                if value == "none" {
                    css.push_str("    border: none;\n");
                } else {
                    // Parse border shorthand: width:color
                    let parts: Vec<&str> = value.split(':').collect();
                    if parts.len() == 2 {
                        let width = parts[0];
                        let color = parts[1];
                        let color_value = if color.starts_with("base-") || color.starts_with("brand-") {
                            format!("var(--rs-{})", color)
                        } else {
                            color.to_string()
                        };
                        css.push_str(&format!("    border: {}px solid {};\n", width, color_value));
                    }
                }
            },
            "border-top" | "border-right" | "border-bottom" | "border-left" => {
                let parts: Vec<&str> = value.split(':').collect();
                if parts.len() == 2 {
                    let width = parts[0];
                    let color = parts[1];
                    let color_value = if color.starts_with("base-") || color.starts_with("brand-") {
                        format!("var(--rs-{})", color)
                    } else {
                        color.to_string()
                    };
                    let side = key.replace("border-", "");
                    css.push_str(&format!("    border-{}: {}px solid {};\n", side, width, color_value));
                }
            },
            "opacity" => css.push_str(&format!("    opacity: {};\n", value)),
            _ => {} // Ignore unknown properties
        }
    }
    
    Ok(css)
}

/// Parse text namespace properties
fn parse_text_properties(attr: &str) -> Result<String> {
    let mut css = String::new();
    let properties = parse_attribute_string(attr)?;
    
    for (key, value) in properties {
        match key.as_str() {
            "color" => {
                if value.starts_with("base-") || value.starts_with("brand-") || value.starts_with("state-") {
                    css.push_str(&format!("    color: var(--rs-{});\n", value));
                } else {
                    css.push_str(&format!("    color: {};\n", value));
                }
            },
            "size" => {
                let size_value = match value.as_str() {
                    "tiny" => "0.75rem",
                    "small" => "0.875rem",
                    "normal" => "1rem",
                    "large" => "1.125rem",
                    "xl" => "1.25rem",
                    "2xl" => "1.5rem",
                    "3xl" => "1.875rem",
                    "4xl" => "2.25rem",
                    "5xl" => "3rem",
                    _ => &value,
                };
                css.push_str(&format!("    font-size: {};\n", size_value));
            },
            "weight" => {
                let weight_value = match value.as_str() {
                    "thin" => "100",
                    "light" => "300",
                    "normal" => "400",
                    "medium" => "500",
                    "semibold" => "600",
                    "bold" => "700",
                    "extrabold" => "800",
                    "black" => "900",
                    _ => &value,
                };
                css.push_str(&format!("    font-weight: {};\n", weight_value));
            },
            "align" => css.push_str(&format!("    text-align: {};\n", value)),
            "transform" => css.push_str(&format!("    text-transform: {};\n", value)),
            "decoration" => css.push_str(&format!("    text-decoration: {};\n", value)),
            "line" => css.push_str(&format!("    line-height: {};\n", value)),
            "spacing" => css.push_str(&format!("    letter-spacing: {};\n", value)),
            _ => {} // Ignore unknown properties
        }
    }
    
    Ok(css)
}

/// Parse layout namespace properties
fn parse_layout_properties(attr: &str) -> Result<String> {
    let mut css = String::new();
    let properties = parse_attribute_string(attr)?;
    
    for (key, value) in properties {
        match key.as_str() {
            "flex" => {
                css.push_str("    display: flex;\n");
                match value.as_str() {
                    "row" => css.push_str("    flex-direction: row;\n"),
                    "column" => css.push_str("    flex-direction: column;\n"),
                    "row-reverse" => css.push_str("    flex-direction: row-reverse;\n"),
                    "column-reverse" => css.push_str("    flex-direction: column-reverse;\n"),
                    _ => {}
                }
            },
            "justify" => {
                let justify_value = match value.as_str() {
                    "start" => "flex-start",
                    "end" => "flex-end",
                    "center" => "center",
                    "between" => "space-between",
                    "around" => "space-around",
                    "evenly" => "space-evenly",
                    _ => &value,
                };
                css.push_str(&format!("    justify-content: {};\n", justify_value));
            },
            "align" => {
                let align_value = match value.as_str() {
                    "start" => "flex-start",
                    "end" => "flex-end",
                    "center" => "center",
                    "stretch" => "stretch",
                    "baseline" => "baseline",
                    _ => &value,
                };
                css.push_str(&format!("    align-items: {};\n", align_value));
            },
            "gap" => css.push_str(&format!("    gap: {};\n", to_rem_value(&value))),
            "position" => css.push_str(&format!("    position: {};\n", value)),
            "inset" => css.push_str(&format!("    inset: {};\n", value)),
            "top" => css.push_str(&format!("    top: {};\n", value)),
            "right" => css.push_str(&format!("    right: {};\n", value)),
            "bottom" => css.push_str(&format!("    bottom: {};\n", value)),
            "left" => css.push_str(&format!("    left: {};\n", value)),
            "z" => css.push_str(&format!("    z-index: {};\n", value)),
            "transform" => css.push_str(&format!("    transform: {};\n", value)),
            _ => {} // Ignore unknown properties
        }
    }
    
    Ok(css)
}

/// Parse device namespace properties
fn parse_device_properties(attr: &str) -> Result<String> {
    let mut css = String::new();
    let properties = parse_attribute_string(attr)?;
    
    for (key, value) in properties {
        match key.as_str() {
            "cursor" => css.push_str(&format!("    cursor: {};\n", value)),
            "select" => css.push_str(&format!("    user-select: {};\n", value)),
            "pointer" => css.push_str(&format!("    pointer-events: {};\n", value)),
            "resize" => css.push_str(&format!("    resize: {};\n", value)),
            "scroll" => css.push_str(&format!("    scroll-behavior: {};\n", value)),
            _ => {} // Ignore unknown properties
        }
    }
    
    Ok(css)
}

/// Parse fx namespace properties (effects with pseudo-classes)
fn parse_fx_properties(component_name: &str, attr: &str) -> Result<String> {
    let mut css = String::new();
    let properties = parse_attribute_string(attr)?;
    
    for (key, value) in properties {
        match key.as_str() {
            "hover" => {
                // Parse hover effects
                css.push_str(&format!("\n  r-s[as=\"{}\"]:hover {{\n", component_name));
                let hover_props = parse_fx_value(&value)?;
                for (prop, val) in hover_props {
                    css.push_str(&format!("    {}: {};\n", prop, val));
                }
                css.push_str("  }\n");
            },
            "active" => {
                // Parse active effects
                css.push_str(&format!("\n  r-s[as=\"{}\"]:active {{\n", component_name));
                let active_props = parse_fx_value(&value)?;
                for (prop, val) in active_props {
                    css.push_str(&format!("    {}: {};\n", prop, val));
                }
                css.push_str("  }\n");
            },
            "focus" => {
                // Parse focus effects
                css.push_str(&format!("\n  r-s[as=\"{}\"]:focus {{\n", component_name));
                let focus_props = parse_fx_value(&value)?;
                for (prop, val) in focus_props {
                    css.push_str(&format!("    {}: {};\n", prop, val));
                }
                css.push_str("  }\n");
            },
            "transition" => {
                // Back to main rule for transition
                let transition_value = match value.as_str() {
                    "fast" => "all 0.15s ease",
                    "normal" => "all 0.3s ease",
                    "slow" => "all 0.5s ease",
                    _ => &value,
                };
                // Need to add this to the main rule - handled separately
            },
            _ => {} // Ignore unknown properties
        }
    }
    
    Ok(css)
}

/// Parse FX value (e.g., "brightness:110" or "scale:0.98" or "bg:base-300")
fn parse_fx_value(value: &str) -> Result<Vec<(String, String)>> {
    let mut props = Vec::new();
    
    // Split by comma for multiple effects
    let effects: Vec<&str> = value.split(',').map(|s| s.trim()).collect();
    
    for effect in effects {
        if effect.contains(':') {
            let parts: Vec<&str> = effect.split(':').collect();
            if parts.len() == 2 {
                let key = parts[0];
                let val = parts[1];
                
                match key {
                    "brightness" => props.push(("filter".to_string(), format!("brightness({}%)", val))),
                    "scale" => props.push(("transform".to_string(), format!("scale({})", val))),
                    "bg" => {
                        let bg_value = if val.starts_with("base-") || val.starts_with("brand-") {
                            format!("var(--rs-{})", val)
                        } else {
                            val.to_string()
                        };
                        props.push(("background".to_string(), bg_value));
                    },
                    "border" => {
                        let border_value = if val.starts_with("base-") || val.starts_with("brand-") {
                            format!("var(--rs-{})", val)
                        } else {
                            val.to_string()
                        };
                        props.push(("border-color".to_string(), border_value));
                    },
                    _ => {}
                }
            }
        } else {
            // Handle simple effect names
            match effect {
                "lift" => {
                    props.push(("transform".to_string(), "translateY(-2px)".to_string()));
                    props.push(("box-shadow".to_string(), "0 4px 6px rgba(0,0,0,0.1)".to_string()));
                },
                _ => {}
            }
        }
    }
    
    Ok(props)
}

/// Parse attribute string like "[padding:4, margin:2]"
fn parse_attribute_string(attr: &str) -> Result<HashMap<String, String>> {
    let mut properties = HashMap::new();
    
    // Remove brackets if present
    let content = attr.trim_start_matches('[').trim_end_matches(']');
    
    // Split by comma
    let parts: Vec<&str> = content.split(',').map(|s| s.trim()).collect();
    
    for part in parts {
        if part.contains(':') {
            let kv: Vec<&str> = part.split(':').collect();
            if kv.len() >= 2 {
                let key = kv[0].trim();
                let value = kv[1..].join(":"); // Join back in case value contains ':'
                let value = value.trim();
                properties.insert(key.to_string(), value.to_string());
            }
        }
    }
    
    Ok(properties)
}

/// Convert spacing value to rem
fn to_rem_value(value: &str) -> String {
    match value {
        "0" => "0".to_string(),
        "1" => "0.25rem".to_string(),
        "2" => "0.5rem".to_string(),
        "3" => "0.75rem".to_string(),
        "4" => "1rem".to_string(),
        "6" => "1.5rem".to_string(),
        "8" => "2rem".to_string(),
        "10" => "2.5rem".to_string(),
        "12" => "3rem".to_string(),
        "16" => "4rem".to_string(),
        "20" => "5rem".to_string(),
        "24" => "6rem".to_string(),
        _ => value.to_string(),
    }
}

/// Convert size value
fn to_size_value(value: &str) -> String {
    if value.ends_with("px") || value.ends_with("rem") || value.ends_with("%") || value.ends_with("vh") || value.ends_with("vw") {
        value.to_string()
    } else {
        format!("{}px", value)
    }
}