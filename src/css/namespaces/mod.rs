pub mod r#box;
pub mod device;
pub mod face;
pub mod fx;
pub mod layout;
pub mod text;

use anyhow::Result;
use crate::config::{Config, ColorsConfig, FontsConfig};

pub fn generate_all(config: &Config, colors: &ColorsConfig, fonts: &FontsConfig) -> Result<String> {
    let mut css = String::new();
    
    // Generate all namespace CSS
    css.push_str(&r#box::BoxNamespace::generate(config)?);
    css.push_str(&device::DeviceNamespace::generate(config)?);
    css.push_str(&face::FaceNamespace::generate(config, colors)?);
    css.push_str(&fx::FxNamespace::generate(config)?);
    css.push_str(&layout::LayoutNamespace::generate(config)?);
    css.push_str(&text::TextNamespace::generate(config, fonts, colors)?);
    
    Ok(css)
}

pub fn generate_responsive_all(config: &Config, colors: &ColorsConfig, fonts: &FontsConfig) -> Result<String> {
    let mut css = String::new();
    
    // Breakpoints
    let breakpoints = [
        ("tablet", "560px"),
        ("screen", "960px"),
    ];
    
    for (name, min_width) in breakpoints {
        css.push_str(&r#box::BoxNamespace::generate_responsive(name, min_width)?);
        css.push_str(&device::DeviceNamespace::generate_responsive(name, min_width)?);
        css.push_str(&face::FaceNamespace::generate_responsive(name, min_width)?);
        css.push_str(&fx::FxNamespace::generate_responsive(name, min_width)?);
        css.push_str(&layout::LayoutNamespace::generate_responsive(name, min_width)?);
        css.push_str(&text::TextNamespace::generate_responsive(name, min_width)?);
    }
    
    Ok(css)
}