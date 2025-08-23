pub mod namespaces;
pub mod defaults;
pub mod components;
pub mod breakpoints;

use anyhow::Result;
use crate::config::{Config, ColorsConfig, FontsConfig, ComponentsConfig};

pub fn generate(
    config: &Config,
    colors: &ColorsConfig,
    fonts: &FontsConfig,
    components: &ComponentsConfig,
) -> Result<String> {
    let mut css = String::new();
    
    // Add CSS layers
    css.push_str("@layer settings, bridge, theme, free;\n\n");
    
    // Generate settings layer
    css.push_str("@layer settings {\n");
    css.push_str("  :root {\n");
    
    // Generate color variables
    for (name, color) in &colors.colors {
        let base_color = format_color(color)?;
        
        // Generate 1-9 scale for brand and state colors
        if name.starts_with("brand-") || name.starts_with("state-") {
            let scale = crate::color::generate_color_scale(&base_color)?;
            css.push_str(&format!("    --rs-color-{}-1: {};\n", name, scale.scale_1));
            css.push_str(&format!("    --rs-color-{}-2: {};\n", name, scale.scale_2));
            css.push_str(&format!("    --rs-color-{}-3: {};\n", name, scale.scale_3));
            css.push_str(&format!("    --rs-color-{}-4: {};\n", name, scale.scale_4));
            css.push_str(&format!("    --rs-color-{}-5: {};\n", name, scale.scale_5));
            css.push_str(&format!("    --rs-color-{}-6: {};\n", name, scale.scale_6));
            css.push_str(&format!("    --rs-color-{}-7: {};\n", name, scale.scale_7));
            css.push_str(&format!("    --rs-color-{}-8: {};\n", name, scale.scale_8));
            css.push_str(&format!("    --rs-color-{}-9: {};\n", name, scale.scale_9));
            
            // Keep old format temporarily for backwards compatibility
            let variations = crate::color::generate_variations(&base_color)?;
            css.push_str(&format!("    --rs-{}-weak: {};\n", name, variations.weak));
            css.push_str(&format!("    --rs-{}-light: {};\n", name, variations.light));
            css.push_str(&format!("    --rs-{}: {};\n", name, base_color));
            css.push_str(&format!("    --rs-{}-intense: {};\n", name, variations.intense));
            css.push_str(&format!("    --rs-{}-bright: {};\n", name, variations.bright));
            css.push_str(&format!("    --rs-{}-strong: {};\n", name, variations.strong));
        } else {
            // For other colors, just set the base value
            css.push_str(&format!("    --rs-{}: {};\n", name, base_color));
        }
    }
    
    // Add neutral color scale (always available)
    let neutral_scale = crate::color::generate_neutral_scale();
    css.push_str(&format!("    --rs-color-neutral-1: {};\n", neutral_scale.scale_1));
    css.push_str(&format!("    --rs-color-neutral-2: {};\n", neutral_scale.scale_2));
    css.push_str(&format!("    --rs-color-neutral-3: {};\n", neutral_scale.scale_3));
    css.push_str(&format!("    --rs-color-neutral-4: {};\n", neutral_scale.scale_4));
    css.push_str(&format!("    --rs-color-neutral-5: {};\n", neutral_scale.scale_5));
    css.push_str(&format!("    --rs-color-neutral-6: {};\n", neutral_scale.scale_6));
    css.push_str(&format!("    --rs-color-neutral-7: {};\n", neutral_scale.scale_7));
    css.push_str(&format!("    --rs-color-neutral-8: {};\n", neutral_scale.scale_8));
    css.push_str(&format!("    --rs-color-neutral-9: {};\n", neutral_scale.scale_9));
    
    // Generate font variables
    for (name, font) in &fonts.fonts {
        css.push_str(&format!("    --rs-{}: {};\n", name, font.family));
    }
    
    css.push_str("  }\n");
    
    // Generate HTML element defaults in settings layer
    css.push_str(&defaults::generate_html_defaults());
    
    // Generate custom component defaults from YAML
    css.push_str(&defaults::generate_component_defaults(components)?);
    
    css.push_str("}\n\n");
    
    // Generate theme layer with namespace styles
    css.push_str("@layer theme {\n");
    
    // Generate namespace styles
    generate_namespaces(&mut css, config, colors, fonts)?;
    
    // Generate component styles from YAML
    css.push_str(&components::generate_component_styles(components)?);
    
    css.push_str("}\n\n");
    
    Ok(css)
}

fn format_color(color: &crate::config::Color) -> Result<String> {
    use crate::config::Color;
    use crate::color::to_oklch;
    
    match color {
        Color::Hex(hex) => {
            // Convert to OKLCH internally
            to_oklch(hex)
        },
        Color::Reference(reference) => Ok(format!("var(--rs-{})", reference)),
        Color::Object { hex, oklch, reference } => {
            if let Some(oklch) = oklch {
                Ok(oklch.clone())
            } else if let Some(hex) = hex {
                // Convert hex to OKLCH
                to_oklch(hex)
            } else if let Some(reference) = reference {
                Ok(format!("var(--rs-{})", reference))
            } else {
                Ok("transparent".to_string())
            }
        }
    }
}

fn generate_namespaces(css: &mut String, config: &Config, colors: &ColorsConfig, fonts: &FontsConfig) -> Result<()> {
    // Generate all namespace CSS (now includes responsive)
    css.push_str(&namespaces::generate_all(config, colors, fonts)?);
    
    Ok(())
}