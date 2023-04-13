use serde::de::DeserializeOwned;
use std::fs;

pub fn load_from_json<T: DeserializeOwned>(file_path: &str) -> Result<Vec<T>, Box<dyn std::error::Error>> {
    let file_contents = fs::read_to_string(file_path)?;
    let data: Vec<T> = serde_json::from_str(&file_contents)?;
    Ok(data)
}