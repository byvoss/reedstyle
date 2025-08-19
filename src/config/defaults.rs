use super::*;
use std::collections::HashMap;

impl Default for Config {
    fn default() -> Self {
        Config {
            version: "1.0".to_string(),
            build: BuildConfig {
                minify: true,
                sourcemaps: false,
                target: "es2020".to_string(),
            },
            config: ConfigPaths {
                colors: "./reedstyle.colors.yaml".to_string(),
                fonts: "./reedstyle.fonts.yaml".to_string(),
                components: "./reedstyle.components.yaml".to_string(),
                bridge: "./reedstyle.bridge.yaml".to_string(),
                env: "./reedstyle.env.yaml".to_string(),
            },
            output: OutputPaths {
                css: "./dist/reedstyle.css".to_string(),
                js: "./dist/reedstyle.js".to_string(),
            },
            features: Features {
                auto_convert_colors: true,
                generate_variations: true,
                custom_properties: true,
            },
        }
    }
}

impl Default for ColorsConfig {
    fn default() -> Self {
        let mut colors = HashMap::new();
        
        // Professional color palette inspired by Tailwind CSS
        // Converted to OKLCH for better color manipulation
        
        // Brand colors (vibrant, professional)
        colors.insert("brand-a".to_string(), Color::Hex("#3B82F6".to_string())); // Blue
        colors.insert("brand-b".to_string(), Color::Hex("#10B981".to_string())); // Emerald
        colors.insert("brand-c".to_string(), Color::Hex("#8B5CF6".to_string())); // Violet
        colors.insert("brand-d".to_string(), Color::Hex("#F59E0B".to_string())); // Amber
        colors.insert("brand-e".to_string(), Color::Hex("#EF4444".to_string())); // Red
        colors.insert("brand-f".to_string(), Color::Hex("#EC4899".to_string())); // Pink
        
        // Semantic state colors
        colors.insert("state-success".to_string(), Color::Hex("#22C55E".to_string()));
        colors.insert("state-warning".to_string(), Color::Hex("#F59E0B".to_string()));
        colors.insert("state-error".to_string(), Color::Hex("#EF4444".to_string()));
        colors.insert("state-info".to_string(), Color::Hex("#3B82F6".to_string()));
        
        // Base grayscale (Tailwind's zinc scale)
        colors.insert("base-0".to_string(), Color::Hex("#FFFFFF".to_string()));
        colors.insert("base-50".to_string(), Color::Hex("#FAFAFA".to_string()));
        colors.insert("base-100".to_string(), Color::Hex("#F4F4F5".to_string()));
        colors.insert("base-200".to_string(), Color::Hex("#E4E4E7".to_string()));
        colors.insert("base-300".to_string(), Color::Hex("#D4D4D8".to_string()));
        colors.insert("base-400".to_string(), Color::Hex("#A1A1AA".to_string()));
        colors.insert("base-500".to_string(), Color::Hex("#71717A".to_string()));
        colors.insert("base-600".to_string(), Color::Hex("#52525B".to_string()));
        colors.insert("base-700".to_string(), Color::Hex("#3F3F46".to_string()));
        colors.insert("base-800".to_string(), Color::Hex("#27272A".to_string()));
        colors.insert("base-900".to_string(), Color::Hex("#18181B".to_string()));
        colors.insert("base-950".to_string(), Color::Hex("#09090B".to_string()));
        colors.insert("base-1000".to_string(), Color::Hex("#000000".to_string()));
        
        let mut semantic = HashMap::new();
        semantic.insert("primary".to_string(), "brand-a".to_string());
        semantic.insert("secondary".to_string(), "brand-b".to_string());
        semantic.insert("success".to_string(), "state-success".to_string());
        semantic.insert("warning".to_string(), "state-warning".to_string());
        semantic.insert("error".to_string(), "state-error".to_string());
        semantic.insert("info".to_string(), "state-info".to_string());
        semantic.insert("text".to_string(), "base-900".to_string());
        semantic.insert("background".to_string(), "base-0".to_string());
        semantic.insert("border".to_string(), "base-300".to_string());
        
        ColorsConfig { colors, semantic }
    }
}

impl Default for FontsConfig {
    fn default() -> Self {
        let mut fonts = HashMap::new();
        
        // System font stacks for optimal performance
        fonts.insert("font-a".to_string(), FontStack {
            family: "system-ui, -apple-system, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif".to_string(),
            fallback: "sans-serif".to_string(),
            weights: {
                let mut weights = HashMap::new();
                weights.insert("thin".to_string(), 100);
                weights.insert("light".to_string(), 300);
                weights.insert("normal".to_string(), 400);
                weights.insert("medium".to_string(), 500);
                weights.insert("semibold".to_string(), 600);
                weights.insert("bold".to_string(), 700);
                weights.insert("extrabold".to_string(), 800);
                weights.insert("black".to_string(), 900);
                Some(weights)
            },
        });
        
        fonts.insert("font-b".to_string(), FontStack {
            family: "'Georgia', 'Cambria', 'Times New Roman', Times, serif".to_string(),
            fallback: "serif".to_string(),
            weights: None,
        });
        
        fonts.insert("font-c".to_string(), FontStack {
            family: "'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace".to_string(),
            fallback: "monospace".to_string(),
            weights: None,
        });
        
        FontsConfig { fonts }
    }
}

impl Default for ComponentsConfig {
    fn default() -> Self {
        // Empty by default - components are optional
        ComponentsConfig {
            components: HashMap::new(),
        }
    }
}