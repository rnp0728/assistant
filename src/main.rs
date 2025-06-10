mod cli;
mod features;
mod utils;

use anyhow::Result;
use clap::Parser;
use cli::Args;
use features::file_formatter::FileFormatter;
use utils::clipboard::Clipboard;

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Format files
    let formatter = FileFormatter::new(args.files);
    let output = formatter.format()?;

    // Handle output
    if args.copy {
        let clipboard = Clipboard::new()?;
        clipboard.copy(&output)?;
        println!("Content copied to clipboard!");
    } else {
        println!("{}", output);
    }

    Ok(())
}