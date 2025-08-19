use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

mod defaults;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub version: String,
    pub build: BuildConfig,
    pub config: ConfigPaths,
    pub output: OutputPaths,
    pub features: Features,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BuildConfig {
    pub minify: bool,
    pub sourcemaps: bool,
    pub target: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigPaths {
    pub colors: String,
    pub fonts: String,
    pub components: String,
    pub bridge: String,
    pub env: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OutputPaths {
    pub css: String,
    pub js: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Features {
    #[serde(rename = "autoConvertColors")]
    pub auto_convert_colors: bool,
    #[serde(rename = "generateVariations")]
    pub generate_variations: bool,
    #[serde(rename = "customProperties")]
    pub custom_properties: bool,
}

impl Config {
    pub fn load() -> Result<Self> {
        let path = Path::new("reedstyle.config.yaml");
        if path.exists() {
            let content = fs::read_to_string(path)?;
            let config: Config = serde_yaml::from_str(&content)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    pub fn load_colors(&self) -> Result<ColorsConfig> {
        if Path::new(&self.config.colors).exists() {
            let content = fs::read_to_string(&self.config.colors)?;
            let colors: ColorsConfig = serde_yaml::from_str(&content)?;
            Ok(colors)
        } else {
            Ok(ColorsConfig::default())
        }
    }

    pub fn load_fonts(&self) -> Result<FontsConfig> {
        if Path::new(&self.config.fonts).exists() {
            let content = fs::read_to_string(&self.config.fonts)?;
            let fonts: FontsConfig = serde_yaml::from_str(&content)?;
            Ok(fonts)
        } else {
            Ok(FontsConfig::default())
        }
    }

    pub fn load_components(&self) -> Result<ComponentsConfig> {
        if Path::new(&self.config.components).exists() {
            let content = fs::read_to_string(&self.config.components)?;
            let components: ComponentsConfig = serde_yaml::from_str(&content)?;
            Ok(components)
        } else {
            Ok(ComponentsConfig::default())
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ColorsConfig {
    pub colors: std::collections::HashMap<String, Color>,
    pub semantic: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Color {
    Hex(String),
    Reference(String),
    Object {
        hex: Option<String>,
        oklch: Option<String>,
        reference: Option<String>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FontsConfig {
    pub fonts: std::collections::HashMap<String, FontStack>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FontStack {
    pub family: String,
    pub fallback: String,
    pub weights: Option<std::collections::HashMap<String, u16>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentsConfig {
    pub components: std::collections::HashMap<String, Component>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Component {
    pub element: Option<String>,  // Optional for extends
    pub extends: Option<String>,
    #[serde(rename = "box")]
    pub box_: Option<String>,
    pub face: Option<String>,
    pub text: Option<String>,
    pub layout: Option<String>,
    pub device: Option<String>,
    pub fx: Option<String>,
}