use anyhow::Result;
use std::fs;
use std::path::PathBuf;

pub struct FileFormatter {
    files: Vec<PathBuf>,
}

impl FileFormatter {
    pub fn new(files: Vec<PathBuf>) -> Self {
        Self { files }
    }

    pub fn format(&self) -> Result<String> {
        let mut output = String::new();
        let mut skipped_files = Vec::new();

        for file_path in &self.files {
            let file_name = match file_path.file_name().and_then(|name| name.to_str()) {
                Some(name) => name,
                None => {
                    skipped_files.push(format!("Invalid file name: {:?}", file_path));
                    continue;
                }
            };

            let content = match fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(e) => {
                    skipped_files.push(format!("Failed to read {:?}: {}", file_path, e));
                    continue;
                }
            };

            output.push_str(&format!("FILE NAME: {}\n", file_name));
            output.push_str("FILE CONTENT\n```\n");
            output.push_str(&content);
            output.push_str("\n```\n\n");
            output.push_str("---\n\n");
        }

        if !skipped_files.is_empty() {
            eprintln!("\nSkipped files:");
            for msg in skipped_files {
                eprintln!("- {}", msg);
            }
            eprintln!();
        }

        Ok(output)
    }
} 