use anyhow::Result;

pub fn parse_reed_attributes(attr: &str) -> Result<Vec<(String, String)>> {
    let mut properties = Vec::new();
    
    // Parse attributes like "padding:4, margin:2"
    for part in attr.split(',') {
        let part = part.trim();
        if let Some((key, value)) = part.split_once(':') {
            properties.push((key.trim().to_string(), value.trim().to_string()));
        }
    }
    
    Ok(properties)
}