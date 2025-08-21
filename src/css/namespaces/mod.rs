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
    
    // Generate all namespace CSS (now includes responsive)
    css.push_str(&r#box::BoxNamespace::generate(config)?);
    css.push_str(&device::DeviceNamespace::generate(config)?);
    css.push_str(&face::FaceNamespace::generate(config, colors)?);
    css.push_str(&fx::FxNamespace::generate(config)?);
    css.push_str(&layout::LayoutNamespace::generate(config)?);
    css.push_str(&text::TextNamespace::generate(config, fonts, colors)?);
    
    Ok(css)
}