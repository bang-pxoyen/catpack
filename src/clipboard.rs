use anyhow::Result;
use arboard::Clipboard;

use crate::osc52;

/// Smart copy: tries OSC 52 first, then falls back to system clipboard
pub fn copy_to_clipboard(text: &str) -> Result<()> {
    if osc52::is_osc52_available() {
        match osc52::copy_to_clipboard(text) {
            Ok(_) => {
                println!("📋 Copied via OSC 52 (terminal clipboard)");
                return Ok(());
            }
            Err(e) => {
                eprintln!("⚠️ OSC 52 failed: {}, trying system clipboard...", e);
            }
        }
    }

    match Clipboard::new() {
        Ok(mut clipboard) => {
            clipboard.set_text(text.to_string())?;
            println!("📋 Copied to system clipboard");
            Ok(())
        }
        Err(e) => {
            anyhow::bail!(
                "Could not copy to clipboard. Try:\n\
                 - Install xclip (X11) or wl-clipboard (Wayland)\n\
                 - Or use OSC 52 compatible terminal (Zellij, tmux, Kitty)\n\
                 Error: {}",
                e
            )
        }
    }
}

/// Force copy via OSC 52 (ignoring system clipboard)
pub fn copy_to_clipboard_osc52_only(text: &str) -> Result<()> {
    osc52::copy_to_clipboard(text)
}
