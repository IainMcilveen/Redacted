use std::fs;
use std::io;
    
    fn remove_all_newlines(text: &str) -> String {
        text.replace("\r\n", " ") // Handle Windows line endings
            .replace("\r", " ")   // Handle old Mac line endings
            .replace("\n", " ")   // Handle Unix/Linux line endings
    }

pub fn get_text_file(file_path: &str) -> io::Result<(String)> {
    // Read the entire file content into a String
    let contents = fs::read_to_string(file_path)?;

    Ok(remove_all_newlines(&contents))
}