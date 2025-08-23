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

/// Generate color scale (1-9) from a base color
/// 1 = lightest (near white), 9 = darkest (near black)
pub fn generate_color_scale(oklch_color: &str) -> Result<ColorScale> {
    // Parse OKLCH string - format: "oklch(71.16% 0.181 22.8)"
    let content = oklch_color.trim_start_matches("oklch(").trim_end_matches(')');
    let parts: Vec<&str> = content.split_whitespace().collect();
    
    if parts.len() < 3 {
        // If parsing fails, return grayscale
        return Ok(ColorScale::grayscale());
    }
    
    let base_lightness = parts[0].trim_end_matches('%').parse::<f32>().unwrap_or(50.0) / 100.0;
    let base_chroma = parts[1].parse::<f32>().unwrap_or(0.1);
    let hue = parts[2].parse::<f32>().unwrap_or(0.0);
    
    // Generate perceptually uniform scale
    // Target lightness values for each step (1-9)
    let target_lightness = [
        0.95,  // 1 - near white
        0.85,  // 2
        0.75,  // 3
        0.65,  // 4
        0.55,  // 5 - middle
        0.45,  // 6
        0.35,  // 7
        0.25,  // 8
        0.15,  // 9 - near black
    ];
    
    // Adjust chroma based on lightness (less chroma at extremes)
    let mut scale = Vec::with_capacity(9);
    for (i, &target_l) in target_lightness.iter().enumerate() {
        // Reduce chroma at very light and very dark ends
        let chroma_factor = if i == 0 {
            0.1  // Very light - minimal chroma
        } else if i == 1 {
            0.3
        } else if i == 2 {
            0.5
        } else if i >= 6 {
            0.8 - (i - 6) as f32 * 0.2  // Gradually reduce for dark colors
        } else {
            1.0  // Full chroma in middle range
        };
        
        let adjusted_chroma = base_chroma * chroma_factor;
        
        scale.push(format!(
            "oklch({:.2}% {:.3} {:.1})",
            target_l * 100.0,
            adjusted_chroma,
            hue
        ));
    }
    
    Ok(ColorScale {
        scale_1: scale[0].clone(),
        scale_2: scale[1].clone(),
        scale_3: scale[2].clone(),
        scale_4: scale[3].clone(),
        scale_5: scale[4].clone(),
        scale_6: scale[5].clone(),
        scale_7: scale[6].clone(),
        scale_8: scale[7].clone(),
        scale_9: scale[8].clone(),
    })
}

/// Generate neutral color scale (grayscale)
pub fn generate_neutral_scale() -> ColorScale {
    ColorScale {
        scale_1: "oklch(99% 0 0)".to_string(),  // White
        scale_2: "oklch(95% 0 0)".to_string(),
        scale_3: "oklch(85% 0 0)".to_string(),
        scale_4: "oklch(70% 0 0)".to_string(),
        scale_5: "oklch(55% 0 0)".to_string(),  // Middle gray
        scale_6: "oklch(40% 0 0)".to_string(),
        scale_7: "oklch(30% 0 0)".to_string(),
        scale_8: "oklch(20% 0 0)".to_string(),
        scale_9: "oklch(10% 0 0)".to_string(),  // Near black
    }
}

pub struct ColorScale {
    pub scale_1: String,
    pub scale_2: String,
    pub scale_3: String,
    pub scale_4: String,
    pub scale_5: String,
    pub scale_6: String,
    pub scale_7: String,
    pub scale_8: String,
    pub scale_9: String,
}

impl ColorScale {
    /// Create a grayscale fallback
    fn grayscale() -> Self {
        generate_neutral_scale()
    }
}

// Keep old struct temporarily for backwards compatibility
pub struct ColorVariations {
    pub weak: String,
    pub light: String,
    pub normal: String,
    pub intense: String,
    pub bright: String,
    pub strong: String,
}

// Temporary compatibility function - maps new scale to old names
pub fn generate_variations(oklch_color: &str) -> Result<ColorVariations> {
    let scale = generate_color_scale(oklch_color)?;
    Ok(ColorVariations {
        weak: scale.scale_2.clone(),     // Very light
        light: scale.scale_3.clone(),    // Light
        normal: scale.scale_5.clone(),   // Middle
        intense: scale.scale_6.clone(),  // Darker
        bright: scale.scale_4.clone(),   // Slightly light
        strong: scale.scale_7.clone(),   // Dark
    })
}