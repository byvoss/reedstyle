pub mod builder;
pub mod color;
pub mod config;
pub mod css;
pub mod js;
pub mod optimizer;
pub mod parser;
pub mod typography;
pub mod utils;

use anyhow::Result;

pub struct ReedStyle {
    config: config::Config,
}

impl ReedStyle {
    pub fn new() -> Result<Self> {
        let config = config::Config::load()?;
        Ok(Self { config })
    }

    pub fn build(&self) -> Result<()> {
        println!("Building ReedSTYLE...");
        
        // Load configurations
        let colors = self.config.load_colors()?;
        let fonts = self.config.load_fonts()?;
        let components = self.config.load_components()?;
        let bridge = self.config.load_bridge()?;
        
        // Generate CSS
        let css_output = css::generate(&self.config, &colors, &fonts, &components, &bridge)?;
        
        // Generate JavaScript
        let js_output = js::generate(&components)?;
        
        // Write output files
        builder::write_output(&css_output, &js_output)?;
        
        Ok(())
    }
}