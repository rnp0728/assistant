use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// List of file paths to process
    #[arg(required = true)]
    pub files: Vec<PathBuf>,

    /// Copy output to clipboard
    #[arg(long)]
    pub copy: bool,
} 