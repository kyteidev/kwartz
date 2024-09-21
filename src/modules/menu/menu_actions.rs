use std::fs;

use crate::modules::textarea::TextArea;

pub fn open_file(textarea: &mut TextArea, path: &str) {
    if let Ok(contents) = fs::read_to_string(path) {
        textarea.replace_content(contents);
    } else {
        textarea.replace_content("Failed to read file".to_string());
    }
}
