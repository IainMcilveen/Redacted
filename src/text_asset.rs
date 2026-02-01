use std::fs;
use std::io;

pub fn get_text_file(file_path: &str) -> io::Result<(String)> {

    // Read the entire file content into a String
    let contents = fs::read_to_string(file_path)?;

    Ok(contents)
}