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

    pub fn append(&self, text: &str) -> Result<()> {
        let mut clipboard = arboard::Clipboard::new()?;
        let existing = clipboard.get_text()?;
        let new_text = if existing.is_empty() {
            text.to_string()
        } else {
            format!("{}\n\n---\n\n{}", existing, text)
        };
        clipboard.set_text(&new_text)?;
        Ok(())
    }

    pub fn show(&self) -> Result<()> {
        let mut clipboard = arboard::Clipboard::new()?;
        let content = clipboard.get_text()?;
        println!("{}", content);
        Ok(())
    }
} 