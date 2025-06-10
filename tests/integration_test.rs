use std::fs::{self, File};
use std::io::Write;
use std::process::Command;
use tempfile::TempDir;

fn setup_test_files() -> TempDir {
    let temp_dir = TempDir::new().unwrap();
    
    // Create test files
    let file1_path = temp_dir.path().join("test1.txt");
    let mut file1 = File::create(&file1_path).unwrap();
    writeln!(file1, "Content 1").unwrap();
    
    let file2_path = temp_dir.path().join("test2.txt");
    let mut file2 = File::create(&file2_path).unwrap();
    writeln!(file2, "Content 2").unwrap();
    
    // Create a subdirectory with a file
    let sub_dir = temp_dir.path().join("subdir");
    fs::create_dir(&sub_dir).unwrap();
    let file3_path = sub_dir.join("test3.txt");
    let mut file3 = File::create(&file3_path).unwrap();
    writeln!(file3, "Content 3").unwrap();
    
    temp_dir
}

#[test]
fn test_basic_file_processing() {
    let temp_dir = setup_test_files();
    let file_path = temp_dir.path().join("test1.txt");
    
    let output = Command::new("cargo")
        .args(["run", "--", file_path.to_str().unwrap()])
        .output()
        .unwrap();
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("test1.txt"));
    assert!(stdout.contains("Content 1"));
}

#[test]
fn test_directory_processing() {
    let temp_dir = setup_test_files();
    
    let output = Command::new("cargo")
        .args(["run", "--", temp_dir.path().to_str().unwrap()])
        .output()
        .unwrap();
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("test1.txt"));
    assert!(stdout.contains("test2.txt"));
    assert!(stdout.contains("test3.txt"));
    assert!(stdout.contains("Content 1"));
    assert!(stdout.contains("Content 2"));
    assert!(stdout.contains("Content 3"));
}

#[test]
fn test_exclude_extensions() {
    let temp_dir = setup_test_files();
    
    // Create a file with excluded extension
    let git_file = temp_dir.path().join("test.git");
    let mut file = File::create(&git_file).unwrap();
    writeln!(file, "Git content").unwrap();
    
    let output = Command::new("cargo")
        .args(["run", "--", "-e", ".git", temp_dir.path().to_str().unwrap()])
        .output()
        .unwrap();
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("test1.txt"));
    assert!(stdout.contains("test2.txt"));
    assert!(stdout.contains("test3.txt"));
    assert!(!stdout.contains("test.git"));
    assert!(!stdout.contains("Git content"));
}