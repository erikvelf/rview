use std::io::IsTerminal;

/// Simple ANSI color utilities for terminal output
pub struct Colors;

impl Colors {
    /// Green color for success/added items
    pub fn green(text: &str) -> String {
        format!("\x1b[32m{}\x1b[0m", text)
    }
    
    /// Yellow color for warnings/excluded items
    pub fn yellow(text: &str) -> String {
        format!("\x1b[33m{}\x1b[0m", text)
    }
    
    /// Blue color for info
    pub fn blue(text: &str) -> String {
        format!("\x1b[34m{}\x1b[0m", text)
    }
    
    /// Checks if colors should be used (if output is to a terminal)
    pub fn should_use_colors() -> bool {
        IsTerminal::is_terminal(&std::io::stdout())
    }
}

/// Conditionally colors text only if output is to a terminal
pub fn green_if_tty(text: &str) -> String {
    if Colors::should_use_colors() {
        Colors::green(text)
    } else {
        text.to_string()
    }
}

/// Conditionally colors text only if output is to a terminal
pub fn yellow_if_tty(text: &str) -> String {
    if Colors::should_use_colors() {
        Colors::yellow(text)
    } else {
        text.to_string()
    }
}

/// Conditionally colors text only if output is to a terminal
pub fn blue_if_tty(text: &str) -> String {
    if Colors::should_use_colors() {
        Colors::blue(text)
    } else {
        text.to_string()
    }
}