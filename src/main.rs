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

    // Show clipboard and exit if requested
    if args.show_clipboard {
        let clipboard = Clipboard::new()?;
        clipboard.show()?;
        return Ok(());
    }
    
    // Format files
    let formatter = FileFormatter::new(args.files);
    let output = formatter.format()?;

    // Handle output
    if args.copy || args.append {
        let clipboard = Clipboard::new()?;
        if args.append {
            clipboard.append(&output)?;
            println!("Content appended to clipboard!");
        } else {
            clipboard.copy(&output)?;
            println!("Content copied to clipboard!");
        }
    } else {
        println!("{}", output);
    }

    Ok(())
}