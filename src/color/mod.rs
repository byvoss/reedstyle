use anyhow::Result;
use palette::{Srgb, IntoColor, Oklch};

/// Convert any color format to OKLCH
pub fn to_oklch(color: &str) -> Result<String> {
    // Parse hex color
    if color.starts_with('#') {
        let hex = color.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16)? as f32 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16)? as f32 / 255.0; 
        let b = u8::from_str_radix(&hex[4..6], 16)? as f32 / 255.0;
        
        let srgb = Srgb::new(r, g, b);
        let oklch: Oklch = srgb.into_color();
        
        // Format as CSS oklch() function
        Ok(format!(
            "oklch({:.2}% {:.3} {:.1})",
            oklch.l * 100.0,  // Lightness as percentage
            oklch.chroma,     // Chroma
            oklch.hue.into_positive_degrees() // Hue in degrees
        ))
    } else if color.starts_with("oklch(") {
        // Already in OKLCH format
        Ok(color.to_string())
    } else if color.starts_with("rgb(") || color.starts_with("rgba(") {
        // Parse RGB(A) format
        let content = color.trim_start_matches("rgb").trim_start_matches("a")
            .trim_start_matches('(').trim_end_matches(')');
        let parts: Vec<&str> = content.split(',').collect();
        
        let r = parts[0].trim().parse::<f32>()? / 255.0;
        let g = parts[1].trim().parse::<f32>()? / 255.0;
        let b = parts[2].trim().parse::<f32>()? / 255.0;
        
        let srgb = Srgb::new(r, g, b);
        let oklch: Oklch = srgb.into_color();
        
        Ok(format!(
            "oklch({:.2}% {:.3} {:.1})",
            oklch.l * 100.0,
            oklch.chroma,
            oklch.hue.into_positive_degrees()
        ))
    } else if color.starts_with("hsl(") || color.starts_with("hsla(") {
        // Parse HSL(A) format
        let content = color.trim_start_matches("hsl").trim_start_matches("a")
            .trim_start_matches('(').trim_end_matches(')');
        let parts: Vec<&str> = content.split(',').collect();
        
        let h = parts[0].trim().parse::<f32>()?;
        let s = parts[1].trim().trim_end_matches('%').parse::<f32>()? / 100.0;
        let l = parts[2].trim().trim_end_matches('%').parse::<f32>()? / 100.0;
        
        // Convert HSL to RGB
        let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = l - c / 2.0;
        
        let (r_temp, g_temp, b_temp) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        
        let r = r_temp + m;
        let g = g_temp + m;
        let b = b_temp + m;
        
        let srgb = Srgb::new(r, g, b);
        let oklch: Oklch = srgb.into_color();
        
        Ok(format!(
            "oklch({:.2}% {:.3} {:.1})",
            oklch.l * 100.0,
            oklch.chroma,
            oklch.hue.into_positive_degrees()
        ))
    } else {
        // Fallback - return as-is for now
        Ok(color.to_string())
    }
}

/// Generate color variations (weak, light, normal, intense, bright, strong)
pub fn generate_variations(oklch_color: &str) -> Result<ColorVariations> {
    // Parse OKLCH string - format: "oklch(71.16% 0.181 22.8)"
    let content = oklch_color.trim_start_matches("oklch(").trim_end_matches(')');
    let parts: Vec<&str> = content.split_whitespace().collect();
    
    if parts.len() < 3 {
        // If parsing fails, return the same color for all variations
        return Ok(ColorVariations {
            weak: oklch_color.to_string(),
            light: oklch_color.to_string(),
            normal: oklch_color.to_string(),
            intense: oklch_color.to_string(),
            bright: oklch_color.to_string(),
            strong: oklch_color.to_string(),
        });
    }
    
    let lightness = parts[0].trim_end_matches('%').parse::<f32>().unwrap_or(50.0) / 100.0;
    let chroma = parts[1].parse::<f32>().unwrap_or(0.1);
    let hue = parts[2].parse::<f32>().unwrap_or(0.0);
    
    Ok(ColorVariations {
        weak: format!("oklch({:.2}% {:.3} {:.1})", (lightness + 0.2).min(1.0) * 100.0, chroma * 0.4, hue),
        light: format!("oklch({:.2}% {:.3} {:.1})", (lightness + 0.1).min(1.0) * 100.0, chroma * 0.7, hue),
        normal: oklch_color.to_string(),
        intense: format!("oklch({:.2}% {:.3} {:.1})", (lightness - 0.1).max(0.0) * 100.0, chroma * 1.2, hue),
        bright: format!("oklch({:.2}% {:.3} {:.1})", lightness * 100.0, chroma * 1.4, hue),
        strong: format!("oklch({:.2}% {:.3} {:.1})", (lightness - 0.2).max(0.0) * 100.0, chroma * 1.1, hue),
    })
}

pub struct ColorVariations {
    pub weak: String,
    pub light: String,
    pub normal: String,
    pub intense: String,
    pub bright: String,
    pub strong: String,
}