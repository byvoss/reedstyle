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
        css.push_str(&format!("    --rs-{}: {};\n", name, format_color(color)?));
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
    
    // Generate namespace styles
    generate_namespaces(&mut css)?;
    
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

fn generate_namespaces(css: &mut String) -> Result<()> {
    // Box namespace
    css.push_str("  /* Box Namespace */\n");
    css.push_str("  reed[box*=\"padding:1\"] { padding: 0.25rem; }\n");
    css.push_str("  reed[box*=\"padding:2\"] { padding: 0.5rem; }\n");
    css.push_str("  reed[box*=\"padding:3\"] { padding: 0.75rem; }\n");
    css.push_str("  reed[box*=\"padding:4\"] { padding: 1rem; }\n\n");
    
    // Face namespace
    css.push_str("  /* Face Namespace */\n");
    css.push_str("  reed[face*=\"bg:base-0\"] { background-color: var(--rs-base-0); }\n");
    css.push_str("  reed[face*=\"radius:md\"] { border-radius: 0.375rem; }\n");
    css.push_str("  reed[face*=\"shadow:sm\"] { box-shadow: 0 1px 2px 0 rgb(0 0 0 / 0.05); }\n\n");
    
    // Text namespace
    css.push_str("  /* Text Namespace */\n");
    css.push_str("  reed[text*=\"size:small\"] { font-size: 0.875rem; }\n");
    css.push_str("  reed[text*=\"size:normal\"] { font-size: 1rem; }\n");
    css.push_str("  reed[text*=\"size:large\"] { font-size: 1.125rem; }\n");
    css.push_str("  reed[text*=\"weight:medium\"] { font-weight: 500; }\n");
    css.push_str("  reed[text*=\"weight:bold\"] { font-weight: 700; }\n\n");
    
    // Layout namespace
    css.push_str("  /* Layout Namespace */\n");
    css.push_str("  reed[layout*=\"flex\"] { display: flex; }\n");
    css.push_str("  reed[layout*=\"grid\"] { display: grid; }\n");
    css.push_str("  reed[layout*=\"justify:center\"] { justify-content: center; }\n");
    css.push_str("  reed[layout*=\"align:center\"] { align-items: center; }\n\n");
    
    // FX namespace
    css.push_str("  /* FX Namespace */\n");
    css.push_str("  reed[fx*=\"transition:fast\"] { transition: all 150ms ease; }\n");
    css.push_str("  reed[fx*=\"hover:scale:1.05\"] { &:hover { transform: scale(1.05); } }\n\n");
    
    // Device namespace
    css.push_str("  /* Device Namespace */\n");
    css.push_str("  reed[device*=\"cursor:pointer\"] { cursor: pointer; }\n");
    css.push_str("  reed[device*=\"select:none\"] { user-select: none; }\n\n");
    
    // Responsive breakpoints
    css.push_str("  /* Tablet Breakpoint */\n");
    css.push_str("  @media (min-width: 560px) {\n");
    css.push_str("    reed[box-tablet*=\"padding:6\"] { padding: 1.5rem; }\n");
    css.push_str("    reed[text-tablet*=\"size:large\"] { font-size: 1.25rem; }\n");
    css.push_str("  }\n\n");
    
    css.push_str("  /* Screen Breakpoint */\n");
    css.push_str("  @media (min-width: 960px) {\n");
    css.push_str("    reed[box-screen*=\"padding:8\"] { padding: 2rem; }\n");
    css.push_str("    reed[text-screen*=\"size:huge\"] { font-size: 1.5rem; }\n");
    css.push_str("  }\n");
    
    Ok(())
}