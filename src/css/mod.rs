pub mod namespaces;

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
    css.push_str("}\n\n");
    
    // Generate theme layer with reed selectors
    css.push_str("@layer theme {\n");
    
    // Base reed element styles
    css.push_str("  reed {\n");
    css.push_str("    display: block;\n");
    css.push_str("  }\n\n");
    
    // Semantic element mappings for reed[as="..."]
    css.push_str("  /* Heading defaults */\n");
    css.push_str("  reed[as=\"h1\"] { font-size: 2.5rem; font-weight: 700; margin: 0.67em 0; }\n");
    css.push_str("  reed[as=\"h2\"] { font-size: 2rem; font-weight: 700; margin: 0.75em 0; }\n");
    css.push_str("  reed[as=\"h3\"] { font-size: 1.75rem; font-weight: 700; margin: 0.83em 0; }\n");
    css.push_str("  reed[as=\"h4\"] { font-size: 1.5rem; font-weight: 700; margin: 1.12em 0; }\n");
    css.push_str("  reed[as=\"h5\"] { font-size: 1.25rem; font-weight: 700; margin: 1.3em 0; }\n");
    css.push_str("  reed[as=\"h6\"] { font-size: 1rem; font-weight: 700; margin: 1.5em 0; }\n");
    
    css.push_str("  /* Paragraph and text defaults */\n");
    css.push_str("  reed[as=\"p\"] { margin: 1em 0; }\n");
    css.push_str("  reed[as=\"strong\"], reed[as=\"b\"] { font-weight: bold; }\n");
    css.push_str("  reed[as=\"em\"], reed[as=\"i\"] { font-style: italic; }\n");
    css.push_str("  reed[as=\"small\"] { font-size: 0.875rem; }\n");
    
    css.push_str("  /* List defaults */\n");
    css.push_str("  reed[as=\"ul\"] { list-style-type: disc; margin: 1em 0; padding-left: 2em; }\n");
    css.push_str("  reed[as=\"ol\"] { list-style-type: decimal; margin: 1em 0; padding-left: 2em; }\n");
    css.push_str("  reed[as=\"li\"] { display: list-item; }\n");
    
    css.push_str("  /* Table defaults */\n");
    css.push_str("  reed[as=\"table\"] { display: table; border-collapse: collapse; }\n");
    css.push_str("  reed[as=\"thead\"] { display: table-header-group; }\n");
    css.push_str("  reed[as=\"tbody\"] { display: table-row-group; }\n");
    css.push_str("  reed[as=\"tr\"] { display: table-row; }\n");
    css.push_str("  reed[as=\"td\"] { display: table-cell; padding: 0.25rem 0.5rem; }\n");
    css.push_str("  reed[as=\"th\"] { display: table-cell; padding: 0.25rem 0.5rem; font-weight: bold; }\n");
    
    css.push_str("  /* Code defaults */\n");
    css.push_str("  reed[as=\"code\"] { font-family: monospace; font-size: 0.875em; }\n");
    css.push_str("  reed[as=\"pre\"] { font-family: monospace; white-space: pre; margin: 1em 0; }\n");
    
    css.push_str("  /* Block defaults */\n");
    css.push_str("  reed[as=\"blockquote\"] { margin: 1em 2em; padding-left: 1em; border-left: 4px solid #ccc; }\n");
    css.push_str("  reed[as=\"hr\"] { border: none; border-top: 1px solid #ccc; margin: 2em 0; }\n\n");
    
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