use crate::description;
use serde_json::Value;
use std::fs;
use std::io::{Error, ErrorKind};

pub fn read_and_generate_description(file_path: &str) -> Result<Vec<(String, String)>, Error> {
    let file_content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => return Err(Error::new(ErrorKind::NotFound, "File not found 📂")),
    };

    let tasks: Vec<Value> = match serde_json::from_str(&file_content) {
        Ok(json) => json,
        Err(_) => return Err(Error::new(ErrorKind::InvalidData, "Invalid JSON 🙁")),
    };

    let mut result = Vec::new();
    for task in tasks {
        let mut description_text = String::new();
        let mut title_text = String::new();
        if let Some(description) = task.get("description") {
            description_text.push_str(&description::generate_description(description, 0));
        }
        if let Some(title) = task.get("title") {
            if let Value::String(s) = title {
                title_text.push_str(s);
            }
        }
        result.push((title_text, description_text));
    }

    Ok(result)
}
