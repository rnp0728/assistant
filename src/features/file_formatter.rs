use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::collections::HashSet;

pub struct FileFormatter {
    paths: Vec<PathBuf>,
    exclude_extensions: HashSet<String>,
}

impl FileFormatter {
    pub fn new(paths: Vec<PathBuf>, exclude_extensions: Vec<String>) -> Self {
        let exclude_extensions = exclude_extensions
            .into_iter()
            .map(|ext| if ext.starts_with('.') { ext } else { format!(".{}", ext) })
            .collect();
        
        Self { 
            paths,
            exclude_extensions,
        }
    }

    fn should_process_file(&self, path: &PathBuf) -> bool {
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                let ext_with_dot = format!(".{}", ext_str);
                return !self.exclude_extensions.contains(&ext_with_dot);
            }
        }
        true
    }

    fn get_files_from_path(&self, path: &PathBuf) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        if !path.exists() {
            return Ok(files);
        }

        if path.is_file() {
            if self.should_process_file(path) {
                files.push(path.clone());
            }
        } else if path.is_dir() {
            self.process_directory(path, &mut files)?;
        }
        
        Ok(files)
    }

    fn process_directory(&self, dir: &PathBuf, files: &mut Vec<PathBuf>) -> Result<()> {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && self.should_process_file(&path) {
                files.push(path);
            } else if path.is_dir() {
                self.process_directory(&path, files)?;
            }
        }
        Ok(())
    }

    pub fn format(&self) -> Result<String> {
        let mut output = String::new();
        let mut skipped_paths = Vec::new();
        let mut processed_files = Vec::new();

        // Collect all files from paths
        for path in &self.paths {
            match self.get_files_from_path(path) {
                Ok(files) => processed_files.extend(files),
                Err(e) => skipped_paths.push(format!("Failed to process {:?}: {}", path, e)),
            }
        }

        // Sort files for consistent output
        processed_files.sort();

        // Process each file
        for file_path in processed_files {
            let file_name = match file_path.file_name().and_then(|name| name.to_str()) {
                Some(name) => name,
                None => {
                    skipped_paths.push(format!("Invalid file name: {:?}", file_path));
                    continue;
                }
            };

            let content = match fs::read_to_string(&file_path) {
                Ok(content) => content,
                Err(e) => {
                    skipped_paths.push(format!("Failed to read {:?}: {}", file_path, e));
                    continue;
                }
            };

            output.push_str(&format!("FILE NAME: {}\n", file_name));
            output.push_str("FILE CONTENT\n```\n");
            output.push_str(&content);
            output.push_str("\n```\n\n");
            output.push_str("---\n\n");
        }

        if !skipped_paths.is_empty() {
            eprintln!("\nSkipped paths:");
            for msg in skipped_paths {
                eprintln!("- {}", msg);
            }
            eprintln!();
        }

        Ok(output)
    }
} 