use anyhow::Result;

pub struct Clipboard;

impl Clipboard {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }

    pub fn copy(&self, text: &str) -> Result<()> {
        let mut clipboard = arboard::Clipboard::new()?;
        clipboard.set_text(text)?;
        Ok(())
    }
} 