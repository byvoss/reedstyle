/// ReedSTYLE Responsive Breakpoint System
/// Mobile-first approach with fixed breakpoints

/// Breakpoint definitions: (suffix, min-width)
/// Empty suffix = base (mobile), None = no media query
pub const BREAKPOINTS: &[(&str, Option<&str>)] = &[
    ("", None),                    // Base (mobile-first, no media query)
    ("-phone", Some("320px")),     // Small phones
    ("-tablet", Some("560px")),    // Tablets & large phones  
    ("-screen", Some("960px")),    // Desktop screens
    ("-wide", Some("1260px")),     // Wide screens
];

/// Get the attribute name for a namespace at a specific breakpoint
/// Example: namespace_attribute("box", "-tablet") -> "box-tablet"
pub fn namespace_attribute(namespace: &str, suffix: &str) -> String {
    format!("{}{}", namespace, suffix)
}

/// Wrap CSS in media query if needed
pub fn wrap_in_media_query(css: &str, min_width: Option<&str>) -> String {
    match min_width {
        Some(width) => format!("@media (min-width: {}) {{\n{}}}\n", width, css),
        None => css.to_string(),
    }
}

/// Generate responsive CSS for a namespace
/// This is a helper that each namespace can use
pub fn generate_responsive<F>(namespace: &str, generate_fn: F) -> String 
where
    F: Fn(&str) -> String
{
    let mut css = String::new();
    
    for (suffix, min_width) in BREAKPOINTS {
        let attr_name = namespace_attribute(namespace, suffix);
        let namespace_css = generate_fn(&attr_name);
        
        if !namespace_css.is_empty() {
            css.push_str(&wrap_in_media_query(&namespace_css, *min_width));
        }
    }
    
    css
}