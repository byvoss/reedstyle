use anyhow::Result;

/// Generate CSS for standard HTML element defaults
/// These are hard-coded because HTML standards are universal
pub fn generate_html_defaults() -> String {
    let mut css = String::new();
    
    css.push_str("\n  /* ========== HTML Element Defaults ========== */\n");
    
    // Base reed element (no 'as' attribute = div)
    css.push_str("  reed {\n");
    css.push_str("    display: block;\n");
    css.push_str("  }\n\n");
    
    // Block elements
    css.push_str("  /* Block elements */\n");
    css.push_str("  reed[as=\"div\"], reed[as=\"section\"], reed[as=\"article\"],\n");
    css.push_str("  reed[as=\"header\"], reed[as=\"footer\"], reed[as=\"main\"],\n");
    css.push_str("  reed[as=\"nav\"], reed[as=\"aside\"], reed[as=\"figure\"],\n");
    css.push_str("  reed[as=\"figcaption\"], reed[as=\"address\"] {\n");
    css.push_str("    display: block;\n");
    css.push_str("  }\n\n");
    
    // Headings
    css.push_str("  /* Headings */\n");
    css.push_str("  reed[as=\"h1\"] { display: block; font-size: 2em; font-weight: bold; margin: 0.67em 0; }\n");
    css.push_str("  reed[as=\"h2\"] { display: block; font-size: 1.5em; font-weight: bold; margin: 0.83em 0; }\n");
    css.push_str("  reed[as=\"h3\"] { display: block; font-size: 1.17em; font-weight: bold; margin: 1em 0; }\n");
    css.push_str("  reed[as=\"h4\"] { display: block; font-size: 1em; font-weight: bold; margin: 1.33em 0; }\n");
    css.push_str("  reed[as=\"h5\"] { display: block; font-size: 0.83em; font-weight: bold; margin: 1.67em 0; }\n");
    css.push_str("  reed[as=\"h6\"] { display: block; font-size: 0.67em; font-weight: bold; margin: 2.33em 0; }\n\n");
    
    // Text block elements
    css.push_str("  /* Text blocks */\n");
    css.push_str("  reed[as=\"p\"] { display: block; margin: 1em 0; }\n");
    css.push_str("  reed[as=\"blockquote\"] { display: block; margin: 1em 40px; }\n");
    css.push_str("  reed[as=\"pre\"] { display: block; font-family: monospace; white-space: pre; margin: 1em 0; }\n\n");
    
    // Inline elements
    css.push_str("  /* Inline elements */\n");
    css.push_str("  reed[as=\"span\"], reed[as=\"a\"], reed[as=\"abbr\"],\n");
    css.push_str("  reed[as=\"cite\"], reed[as=\"code\"], reed[as=\"dfn\"],\n");
    css.push_str("  reed[as=\"kbd\"], reed[as=\"mark\"], reed[as=\"q\"],\n");
    css.push_str("  reed[as=\"samp\"], reed[as=\"time\"], reed[as=\"var\"] {\n");
    css.push_str("    display: inline;\n");
    css.push_str("  }\n\n");
    
    // Inline formatting
    css.push_str("  /* Inline formatting */\n");
    css.push_str("  reed[as=\"strong\"], reed[as=\"b\"] { display: inline; font-weight: bold; }\n");
    css.push_str("  reed[as=\"em\"], reed[as=\"i\"] { display: inline; font-style: italic; }\n");
    css.push_str("  reed[as=\"u\"] { display: inline; text-decoration: underline; }\n");
    css.push_str("  reed[as=\"s\"], reed[as=\"strike\"], reed[as=\"del\"] { display: inline; text-decoration: line-through; }\n");
    css.push_str("  reed[as=\"ins\"] { display: inline; text-decoration: underline; }\n");
    css.push_str("  reed[as=\"small\"] { display: inline; font-size: smaller; }\n");
    css.push_str("  reed[as=\"sub\"] { display: inline; vertical-align: sub; font-size: smaller; }\n");
    css.push_str("  reed[as=\"sup\"] { display: inline; vertical-align: super; font-size: smaller; }\n");
    css.push_str("  reed[as=\"code\"] { display: inline; font-family: monospace; font-size: 0.875em; }\n");
    css.push_str("  reed[as=\"kbd\"] { display: inline; font-family: monospace; font-size: 0.875em; }\n");
    css.push_str("  reed[as=\"samp\"] { display: inline; font-family: monospace; font-size: 0.875em; }\n");
    css.push_str("  reed[as=\"var\"] { display: inline; font-style: italic; }\n");
    css.push_str("  reed[as=\"mark\"] { display: inline; background-color: yellow; color: black; }\n\n");
    
    // Lists
    css.push_str("  /* Lists */\n");
    css.push_str("  reed[as=\"ul\"] { display: block; list-style-type: disc; margin: 1em 0; padding-left: 40px; }\n");
    css.push_str("  reed[as=\"ol\"] { display: block; list-style-type: decimal; margin: 1em 0; padding-left: 40px; }\n");
    css.push_str("  reed[as=\"li\"] { display: list-item; }\n");
    css.push_str("  reed[as=\"dl\"] { display: block; margin: 1em 0; }\n");
    css.push_str("  reed[as=\"dt\"] { display: block; font-weight: bold; }\n");
    css.push_str("  reed[as=\"dd\"] { display: block; margin-left: 40px; }\n\n");
    
    // Tables
    css.push_str("  /* Tables */\n");
    css.push_str("  reed[as=\"table\"] { display: table; border-collapse: separate; border-spacing: 2px; }\n");
    css.push_str("  reed[as=\"caption\"] { display: table-caption; text-align: center; }\n");
    css.push_str("  reed[as=\"thead\"] { display: table-header-group; vertical-align: middle; }\n");
    css.push_str("  reed[as=\"tbody\"] { display: table-row-group; vertical-align: middle; }\n");
    css.push_str("  reed[as=\"tfoot\"] { display: table-footer-group; vertical-align: middle; }\n");
    css.push_str("  reed[as=\"tr\"] { display: table-row; vertical-align: inherit; }\n");
    css.push_str("  reed[as=\"td\"] { display: table-cell; vertical-align: inherit; padding: 1px; }\n");
    css.push_str("  reed[as=\"th\"] { display: table-cell; vertical-align: inherit; font-weight: bold; text-align: center; padding: 1px; }\n");
    css.push_str("  reed[as=\"colgroup\"] { display: table-column-group; }\n");
    css.push_str("  reed[as=\"col\"] { display: table-column; }\n\n");
    
    // Forms
    css.push_str("  /* Forms */\n");
    css.push_str("  reed[as=\"form\"] { display: block; margin: 0; }\n");
    css.push_str("  reed[as=\"fieldset\"] { display: block; margin: 0 2px; padding: 0.35em 0.625em 0.75em; border: 2px groove; }\n");
    css.push_str("  reed[as=\"legend\"] { display: block; padding: 0 2px; }\n");
    css.push_str("  reed[as=\"label\"] { display: inline; cursor: default; }\n");
    css.push_str("  reed[as=\"button\"] { display: inline-block; }\n");
    css.push_str("  reed[as=\"input\"] { display: inline-block; }\n");
    css.push_str("  reed[as=\"select\"] { display: inline-block; }\n");
    css.push_str("  reed[as=\"datalist\"] { display: none; }\n");
    css.push_str("  reed[as=\"optgroup\"] { display: block; }\n");
    css.push_str("  reed[as=\"option\"] { display: block; }\n");
    css.push_str("  reed[as=\"textarea\"] { display: inline-block; }\n");
    css.push_str("  reed[as=\"output\"] { display: inline; }\n");
    css.push_str("  reed[as=\"progress\"] { display: inline-block; vertical-align: baseline; }\n");
    css.push_str("  reed[as=\"meter\"] { display: inline-block; vertical-align: baseline; }\n\n");
    
    // Interactive elements
    css.push_str("  /* Interactive elements */\n");
    css.push_str("  reed[as=\"details\"] { display: block; }\n");
    css.push_str("  reed[as=\"summary\"] { display: block; }\n");
    css.push_str("  reed[as=\"dialog\"] { display: block; position: absolute; left: 0; right: 0; margin: auto; border: solid; padding: 1em; }\n\n");
    
    // Media
    css.push_str("  /* Media */\n");
    css.push_str("  reed[as=\"img\"] { display: inline-block; }\n");
    css.push_str("  reed[as=\"picture\"] { display: inline-block; }\n");
    css.push_str("  reed[as=\"source\"] { display: none; }\n");
    css.push_str("  reed[as=\"svg\"] { display: inline-block; }\n");
    css.push_str("  reed[as=\"video\"] { display: inline-block; }\n");
    css.push_str("  reed[as=\"audio\"] { display: inline-block; }\n");
    css.push_str("  reed[as=\"track\"] { display: none; }\n");
    css.push_str("  reed[as=\"map\"] { display: inline; }\n");
    css.push_str("  reed[as=\"area\"] { display: none; }\n");
    css.push_str("  reed[as=\"canvas\"] { display: inline-block; }\n");
    css.push_str("  reed[as=\"iframe\"] { display: inline-block; border: 2px inset; }\n");
    css.push_str("  reed[as=\"embed\"] { display: inline-block; }\n");
    css.push_str("  reed[as=\"object\"] { display: inline-block; }\n");
    css.push_str("  reed[as=\"param\"] { display: none; }\n\n");
    
    // Other
    css.push_str("  /* Other */\n");
    css.push_str("  reed[as=\"hr\"] { display: block; margin: 0.5em auto; border-style: inset; border-width: 1px; }\n");
    css.push_str("  reed[as=\"br\"] { display: inline; }\n");
    css.push_str("  reed[as=\"wbr\"] { display: inline; }\n\n");
    
    // Hidden elements
    css.push_str("  /* Hidden by default */\n");
    css.push_str("  reed[as=\"script\"], reed[as=\"style\"], reed[as=\"template\"],\n");
    css.push_str("  reed[as=\"head\"], reed[as=\"meta\"], reed[as=\"link\"],\n");
    css.push_str("  reed[as=\"base\"], reed[as=\"title\"], reed[as=\"noscript\"] {\n");
    css.push_str("    display: none;\n");
    css.push_str("  }\n\n");
    
    css
}

/// Generate CSS for custom components from YAML config
pub fn generate_component_defaults(components: &crate::config::ComponentsConfig) -> Result<String> {
    let mut css = String::new();
    
    css.push_str("\n  /* ========== Custom Components ========== */\n");
    
    for (name, component) in &components.components {
        // Start component rule
        css.push_str(&format!("  reed[as=\"{}\"] {{\n", name));
        
        // Get base element (default to div)
        let base_element = component.element.as_deref().unwrap_or("div");
        
        // Apply base element defaults based on type
        css.push_str(&get_element_base_styles(base_element));
        
        // Note: Namespace styles (box, face, text, etc.) will be handled
        // by the namespace processors when the element is rendered
        // This just ensures the correct display type and basic defaults
        
        css.push_str("  }\n");
    }
    
    css.push_str("\n");
    Ok(css)
}

/// Get the base styles for a given HTML element
fn get_element_base_styles(element: &str) -> String {
    match element {
        // Block elements
        "div" | "section" | "article" | "header" | "footer" | 
        "main" | "nav" | "aside" | "figure" | "figcaption" | 
        "address" | "form" | "fieldset" => {
            "    display: block;\n".to_string()
        }
        
        // Headings
        "h1" => "    display: block; font-size: 2em; font-weight: bold; margin: 0.67em 0;\n".to_string(),
        "h2" => "    display: block; font-size: 1.5em; font-weight: bold; margin: 0.83em 0;\n".to_string(),
        "h3" => "    display: block; font-size: 1.17em; font-weight: bold; margin: 1em 0;\n".to_string(),
        "h4" => "    display: block; font-size: 1em; font-weight: bold; margin: 1.33em 0;\n".to_string(),
        "h5" => "    display: block; font-size: 0.83em; font-weight: bold; margin: 1.67em 0;\n".to_string(),
        "h6" => "    display: block; font-size: 0.67em; font-weight: bold; margin: 2.33em 0;\n".to_string(),
        
        // Text blocks
        "p" => "    display: block; margin: 1em 0;\n".to_string(),
        "blockquote" => "    display: block; margin: 1em 40px;\n".to_string(),
        "pre" => "    display: block; font-family: monospace; white-space: pre; margin: 1em 0;\n".to_string(),
        
        // Inline elements
        "span" | "a" | "abbr" | "cite" | "code" | "dfn" |
        "kbd" | "mark" | "q" | "samp" | "time" | "var" => {
            "    display: inline;\n".to_string()
        }
        
        // Inline formatting
        "strong" | "b" => "    display: inline; font-weight: bold;\n".to_string(),
        "em" | "i" => "    display: inline; font-style: italic;\n".to_string(),
        "small" => "    display: inline; font-size: smaller;\n".to_string(),
        
        // Lists
        "ul" => "    display: block; list-style-type: disc; margin: 1em 0; padding-left: 40px;\n".to_string(),
        "ol" => "    display: block; list-style-type: decimal; margin: 1em 0; padding-left: 40px;\n".to_string(),
        "li" => "    display: list-item;\n".to_string(),
        "dl" => "    display: block; margin: 1em 0;\n".to_string(),
        "dt" => "    display: block; font-weight: bold;\n".to_string(),
        "dd" => "    display: block; margin-left: 40px;\n".to_string(),
        
        // Tables
        "table" => "    display: table; border-collapse: separate; border-spacing: 2px;\n".to_string(),
        "caption" => "    display: table-caption; text-align: center;\n".to_string(),
        "thead" => "    display: table-header-group; vertical-align: middle;\n".to_string(),
        "tbody" => "    display: table-row-group; vertical-align: middle;\n".to_string(),
        "tfoot" => "    display: table-footer-group; vertical-align: middle;\n".to_string(),
        "tr" => "    display: table-row; vertical-align: inherit;\n".to_string(),
        "td" => "    display: table-cell; vertical-align: inherit; padding: 1px;\n".to_string(),
        "th" => "    display: table-cell; vertical-align: inherit; font-weight: bold; text-align: center; padding: 1px;\n".to_string(),
        
        // Form elements
        "button" | "input" | "select" | "textarea" => {
            "    display: inline-block;\n".to_string()
        }
        "label" => "    display: inline; cursor: default;\n".to_string(),
        
        // Media
        "img" | "picture" | "svg" | "video" | "audio" | "canvas" | "iframe" | "embed" | "object" => {
            "    display: inline-block;\n".to_string()
        }
        
        // Default to block
        _ => "    display: block;\n".to_string()
    }
}