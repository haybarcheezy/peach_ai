use clipboard::{ClipboardContext, ClipboardProvider};
use std::error::Error;

pub fn get_clipboard_content() -> Result<String, Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    Ok(ctx.get_contents().expect("Error getting clipboard content"))
}

pub fn set_clipboard_content(content: &str) -> Result<(), Box<dyn Error>> {
    let mut ctx: ClipboardContext = ClipboardProvider::new()?;
    ctx.set_contents(content.to_owned())?;
    Ok(())
}
