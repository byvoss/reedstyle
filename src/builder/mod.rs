use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn write_output(css: &str, js: &str) -> Result<()> {
    // Create dist directory
    fs::create_dir_all("dist")?;
    
    // Write CSS file
    let css_path = Path::new("dist/reedstyle.css");
    fs::write(css_path, css)?;
    println!("✓ Written: dist/reedstyle.css");
    
    // Write minified CSS
    let minified_css = minify_css(css)?;
    let min_css_path = Path::new("dist/reedstyle.min.css");
    fs::write(min_css_path, minified_css)?;
    println!("✓ Written: dist/reedstyle.min.css");
    
    // Write JS file
    let js_path = Path::new("dist/reedstyle.js");
    fs::write(js_path, js)?;
    println!("✓ Written: dist/reedstyle.js");
    
    // Write minified JS
    let min_js_path = Path::new("dist/reedstyle.min.js");
    fs::write(min_js_path, js)?; // TODO: Implement proper JS minification
    println!("✓ Written: dist/reedstyle.min.js");
    
    Ok(())
}

fn minify_css(css: &str) -> Result<String> {
    // For production, we'll use external tools via CLI
    // For now, just do basic minification ourselves
    let minified = css
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("")
        .replace(": ", ":")
        .replace("; ", ";")
        .replace(" {", "{")
        .replace("{ ", "{")
        .replace(" }", "}")
        .replace("} ", "}")
        .replace(", ", ",");
    
    Ok(minified)
}