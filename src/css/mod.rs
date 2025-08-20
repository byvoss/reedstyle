pub mod namespaces;
pub mod defaults;

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
        css.push_str(&format!("    --rs-{}: {};\n", name, base_color));
        
        // Generate Visual Scope variations if it's a brand or state color
        if name.starts_with("brand-") || name.starts_with("state-") {
            // Generate proper OKLCH color variations
            let variations = crate::color::generate_variations(&base_color)?;
            css.push_str(&format!("    --rs-{}-weak: {};\n", name, variations.weak));
            css.push_str(&format!("    --rs-{}-light: {};\n", name, variations.light));
            css.push_str(&format!("    --rs-{}-intense: {};\n", name, variations.intense));
            css.push_str(&format!("    --rs-{}-bright: {};\n", name, variations.bright));
            css.push_str(&format!("    --rs-{}-strong: {};\n", name, variations.strong));
        }
    }
    
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
    // Generate all namespace CSS
    css.push_str(&namespaces::generate_all(config, colors, fonts)?);
    
    // Generate responsive variants
    css.push_str(&namespaces::generate_responsive_all(config, colors, fonts)?);
    
    Ok(())
}