use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InjectionStrategy {
    ClipboardPaste,
    KeyboardSimulation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InjectionConfig {
    pub strategy: InjectionStrategy,
    pub preserve_clipboard: bool,
    pub paste_delay_ms: u64,
    pub restore_delay_ms: u64,
}

impl Default for InjectionConfig {
    fn default() -> Self {
        Self {
            strategy: InjectionStrategy::KeyboardSimulation,
            preserve_clipboard: true,
            paste_delay_ms: 100,
            restore_delay_ms: 200,
        }
    }
}

pub struct TextInjector {
    config: InjectionConfig,
}

impl TextInjector {
    pub fn new(config: InjectionConfig) -> Self {
        Self { config }
    }

    pub fn inject(&self, text: &str) -> Result<()> {
        match self.config.strategy {
            InjectionStrategy::KeyboardSimulation => self.inject_via_keyboard(text),
            InjectionStrategy::ClipboardPaste => self.inject_via_clipboard(text),
        }
    }

    fn inject_via_keyboard(&self, text: &str) -> Result<()> {
        let escaped = text
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\" & return & \"");

        let script = format!(
            r#"tell application "System Events" to keystroke "{}""#,
            escaped
        );

        let output = Command::new("osascript")
            .arg("-e")
            .arg(&script)
            .output()?;

        if !output.status.success() {
            let err = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("keystroke failed: {}", err);
        }

        Ok(())
    }

    fn inject_via_clipboard(&self, text: &str) -> Result<()> {
        let previous_clipboard = if self.config.preserve_clipboard {
            self.get_clipboard().ok()
        } else {
            None
        };

        self.set_clipboard(text)?;
        std::thread::sleep(std::time::Duration::from_millis(self.config.paste_delay_ms));
        self.simulate_paste()?;

        if let Some(prev) = previous_clipboard {
            std::thread::sleep(std::time::Duration::from_millis(self.config.restore_delay_ms));
            let _ = self.set_clipboard(&prev);
        }

        Ok(())
    }

    fn get_clipboard(&self) -> Result<String> {
        let output = Command::new("pbpaste").output()?;
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn set_clipboard(&self, text: &str) -> Result<()> {
        use std::io::Write;
        let mut child = Command::new("pbcopy")
            .stdin(std::process::Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }
        child.wait()?;
        Ok(())
    }

    fn simulate_paste(&self) -> Result<()> {
        let script = r#"tell application "System Events" to keystroke "v" using command down"#;
        Command::new("osascript")
            .arg("-e")
            .arg(script)
            .output()?;
        Ok(())
    }

    pub fn get_focused_app(&self) -> Option<String> {
        let output = Command::new("osascript")
            .arg("-e")
            .arg(r#"tell application "System Events" to get name of first application process whose frontmost is true"#)
            .output()
            .ok()?;

        let name = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if name.is_empty() {
            None
        } else {
            Some(name)
        }
    }
}
