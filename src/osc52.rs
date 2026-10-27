use anyhow::Result;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use std::io::{self, Write};

const MAX_OSC52_SIZE: usize = 500_000;

/// Copies text to clipboard via OSC 52 escape sequence
pub fn copy_to_clipboard(text: &str) -> Result<()> {
    if text.len() > MAX_OSC52_SIZE {
        anyhow::bail!(
            "Text too large for OSC 52 ({} bytes > {} bytes limit). Use file output instead.",
            text.len(),
            MAX_OSC52_SIZE
        );
    }

    let encoded = STANDARD.encode(text.as_bytes());
    let osc52_sequence = format!("\x1b]52;c;{}\x07", encoded);
    
    let mut stdout = io::stdout().lock();
    stdout.write_all(osc52_sequence.as_bytes())?;
    stdout.flush()?;
    
    Ok(())
}

/// Checks if the terminal supports OSC 52
pub fn is_osc52_available() -> bool {
    std::env::var("TERM_PROGRAM").map_or(false, |v| {
        matches!(
            v.to_lowercase().as_str(),
            "kitty" | "wezterm" | "iterm.app" | "ghostty"
        )
    }) || std::env::var("TMUX").is_ok()
        || std::env::var("ZELLIJ").is_ok()
        || std::env::var("KITTY_WINDOW_ID").is_ok()
        || std::env::var("WEZTERM_EXECUTABLE_DIR").is_ok()
}
