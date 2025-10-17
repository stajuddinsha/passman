use anyhow::Result;
use std::process::Command;
use std::time::Duration;
use std::thread;

#[derive(Clone)]
pub struct ClipboardManager {
    timeout: Duration,
}

impl ClipboardManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            timeout: Duration::from_secs(20),
        })
    }

    pub fn copy_to_clipboard(&self, text: &str) -> Result<()> {
        // Try Wayland first (wl-copy), then fall back to X11 (xclip)
        if self.try_wayland_copy(text) {
            return Ok(());
        }

        if self.try_x11_copy(text) {
            return Ok(());
        }

        Err(anyhow::anyhow!("Failed to copy to clipboard: neither wl-copy nor xclip available"))
    }

    fn try_wayland_copy(&self, text: &str) -> bool {
        Command::new("wl-copy")
            .arg(text)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn try_x11_copy(&self, text: &str) -> bool {
        Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .arg(text)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    pub fn copy_with_auto_clear(&self, text: &str) -> Result<()> {
        // Copy to clipboard
        self.copy_to_clipboard(text)?;

        // Schedule auto-clear
        let timeout = self.timeout;
        thread::spawn(move || {
            thread::sleep(timeout);
            Self::clear_clipboard().ok();
        });

        Ok(())
    }

    fn clear_clipboard() -> Result<()> {
        // Try Wayland first
        if Command::new("wl-copy")
            .arg("")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
        {
            return Ok(());
        }

        // Fall back to X11
        if Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .arg("")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
        {
            return Ok(());
        }

        Err(anyhow::anyhow!("Failed to clear clipboard"))
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }
}