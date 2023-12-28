use std::fs;

pub fn get_contents_as_text(file_path: &str) -> Result<String, String> {
    let result = fs::read_to_string(file_path);
    match result {
        Ok(content) => Ok(content),
        Err(_) => Err(format!("Unable to read file: {}", file_path)),
    }
}
