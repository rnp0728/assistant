use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// List of file paths to process
    #[arg(required = false)]
    pub files: Vec<PathBuf>,

    /// Copy output to clipboard
    #[arg(short = 'c', long = "copy")]
    pub copy: bool,

    /// Append to existing clipboard content
    #[arg(short = 'a', long = "append")]
    pub append: bool,

    /// Show current clipboard contents
    #[arg(short = 's', long = "show-clipboard")]
    pub show_clipboard: bool,
} 