use anyhow::Result;
use std::fs;
use std::path::Path;
use crate::optimizer;

// Version constant - single source of truth
pub const VERSION: &str = "0.1.0";
pub const LICENSE: &str = "Apache-2.0";
pub const WEBSITE: &str = "https://reedstyle.dev";

pub fn write_output(css: &str, js: &str) -> Result<()> {
    // Create dist directory
    fs::create_dir_all("dist")?;
    
    // Add headers to files
    let css_with_header = add_css_header(css, false);
    let js_with_header = add_js_header(js, false);
    
    // Write development CSS file
    let css_path = Path::new("dist/reedstyle.css");
    fs::write(css_path, &css_with_header)?;
    let css_size = css_with_header.len() / 1024;
    println!("✓ Written: dist/reedstyle.css ({}KB)", css_size);
    
    // Write minified CSS using Lightning CSS
    let minified_css = optimizer::minify_css(&css)?;
    let minified_css_with_header = add_css_header(&minified_css, true);
    let min_css_path = Path::new("dist/reedstyle.min.css");
    fs::write(min_css_path, &minified_css_with_header)?;
    let min_css_size = minified_css_with_header.len() / 1024;
    println!("✓ Written: dist/reedstyle.min.css ({}KB)", min_css_size);
    
    // Write development JS file
    let js_path = Path::new("dist/reedstyle.js");
    fs::write(js_path, &js_with_header)?;
    let js_size = js_with_header.len() / 1024;
    println!("✓ Written: dist/reedstyle.js ({}KB)", js_size);
    
    // Write minified JS
    let minified_js = minify_js(&js)?;
    let minified_js_with_header = add_js_header(&minified_js, true);
    let min_js_path = Path::new("dist/reedstyle.min.js");
    fs::write(min_js_path, &minified_js_with_header)?;
    let min_js_size = minified_js_with_header.len() / 1024;
    println!("✓ Written: dist/reedstyle.min.js ({}KB)", min_js_size);
    
    // Copy LICENSE file to dist
    if Path::new("LICENSE").exists() {
        fs::copy("LICENSE", "dist/LICENSE")?;
        println!("✓ Copied: dist/LICENSE");
    } else {
        // Create a basic LICENSE file if it doesn't exist
        let license_content = include_str!("../../LICENSE.template");
        fs::write("dist/LICENSE", license_content)?;
        println!("✓ Created: dist/LICENSE");
    }
    
    // Print size summary
    println!("\n📊 Size Summary:");
    println!("  CSS: {}KB → {}KB ({}% reduction)", 
        css_size, min_css_size, 
        ((css_size - min_css_size) * 100 / css_size));
    println!("  JS:  {}KB → {}KB ({}% reduction)", 
        js_size, min_js_size,
        ((js_size - min_js_size) * 100 / js_size));
    
    Ok(())
}

fn add_css_header(css: &str, minified: bool) -> String {
    let header = if minified {
        format!("/* ReedSTYLE v{} | {} | {} */\n", VERSION, LICENSE, WEBSITE)
    } else {
        format!(
            r#"/**
 * ReedSTYLE v{}
 * Semantic HTML styling system - Write HTML, get beautiful designs
 * 
 * @license {}
 * @website {}
 * 
 * Copyright 2024 ByVoss Technologies
 * Licensed under the Apache License, Version 2.0
 */

"#,
            VERSION, LICENSE, WEBSITE
        )
    };
    
    format!("{}{}", header, css)
}

fn add_js_header(js: &str, minified: bool) -> String {
    let header = if minified {
        format!("/* ReedSTYLE v{} | {} | {} */\n", VERSION, LICENSE, WEBSITE)
    } else {
        format!(
            r#"/**
 * ReedSTYLE JavaScript Enhancement v{}
 * Optional progressive enhancement for ReedSTYLE
 * 
 * @license {}
 * @website {}
 * 
 * Copyright 2024 ByVoss Technologies
 * Licensed under the Apache License, Version 2.0
 */

"#,
            VERSION, LICENSE, WEBSITE
        )
    };
    
    format!("{}{}", header, js)
}

// Old minify_css function replaced by Lightning CSS in optimizer module

fn minify_js(js: &str) -> Result<String> {
    // For now, just use basic minification since minify-js has issues
    // TODO: Fix minify-js or use a different library
    Ok(basic_minify_js(js))
}

fn basic_minify_js(js: &str) -> String {
    js.lines()
        .map(|line| {
            // Remove line comments
            if let Some(pos) = line.find("//") {
                // Check if it's not in a string
                let before = &line[..pos];
                if !before.contains('"') && !before.contains('\'') {
                    before.trim().to_string()
                } else {
                    line.trim().to_string()
                }
            } else {
                line.trim().to_string()
            }
        })
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
        .replace("  ", " ")
        .replace(" = ", "=")
        .replace(" + ", "+")
        .replace(" - ", "-")
        .replace(" * ", "*")
        .replace(" / ", "/")
        .replace(" { ", "{")
        .replace(" } ", "}")
        .replace("; ", ";")
        .replace(": ", ":")
        .replace(", ", ",")
}