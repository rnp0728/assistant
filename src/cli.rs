use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// List of file or folder paths to process
    #[arg(required = false)]
    pub paths: Vec<PathBuf>,

    /// Copy output to clipboard
    #[arg(short = 'c', long = "copy")]
    pub copy: bool,

    /// Append to existing clipboard content
    #[arg(short = 'a', long = "append")]
    pub append: bool,

    /// Show current clipboard contents
    #[arg(short = 's', long = "show-clipboard")]
    pub show_clipboard: bool,

    /// File extensions to exclude (e.g., .git, .DS_Store)
    #[arg(short = 'e', long = "exclude-ext")]
    pub exclude_extensions: Vec<String>,
} 