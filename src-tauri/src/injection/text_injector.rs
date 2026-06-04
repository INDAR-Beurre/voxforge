use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::thread;
use std::time::Duration;

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
            strategy: InjectionStrategy::ClipboardPaste,
            preserve_clipboard: true,
            paste_delay_ms: 80,
            restore_delay_ms: 300,
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
        self.inject_via_clipboard(text)
    }

    fn inject_via_clipboard(&self, text: &str) -> Result<()> {
        let previous_clipboard = if self.config.preserve_clipboard {
            self.get_clipboard().ok()
        } else {
            None
        };

        self.set_clipboard(text)?;
        thread::sleep(Duration::from_millis(self.config.paste_delay_ms));
        self.simulate_paste_cgevent()?;

        if let Some(prev) = previous_clipboard {
            thread::sleep(Duration::from_millis(self.config.restore_delay_ms));
            let _ = self.set_clipboard(&prev);
        }

        Ok(())
    }

    fn simulate_paste_cgevent(&self) -> Result<()> {
        unsafe {
            extern "C" {
                fn CGEventCreateKeyboardEvent(
                    source: *const std::ffi::c_void,
                    virtual_key: u16,
                    key_down: bool,
                ) -> *mut std::ffi::c_void;
                fn CGEventSetFlags(event: *mut std::ffi::c_void, flags: u64);
                fn CGEventPost(tap: u32, event: *mut std::ffi::c_void);
                fn CFRelease(cf: *mut std::ffi::c_void);
            }

            // Virtual key 9 = 'V', kCGEventFlagMaskCommand = 1 << 20
            let cmd_flag: u64 = 1 << 20;

            let key_down = CGEventCreateKeyboardEvent(std::ptr::null(), 9, true);
            if key_down.is_null() {
                anyhow::bail!("Failed to create key down event");
            }
            CGEventSetFlags(key_down, cmd_flag);

            let key_up = CGEventCreateKeyboardEvent(std::ptr::null(), 9, false);
            if key_up.is_null() {
                CFRelease(key_down);
                anyhow::bail!("Failed to create key up event");
            }
            CGEventSetFlags(key_up, cmd_flag);

            // kCGHIDEventTap = 0
            CGEventPost(0, key_down);
            CGEventPost(0, key_up);

            CFRelease(key_down);
            CFRelease(key_up);
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
