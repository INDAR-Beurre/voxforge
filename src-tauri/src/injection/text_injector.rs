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
            preserve_clipboard: false,
            paste_delay_ms: 50,
            restore_delay_ms: 500,
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
        self.set_clipboard(text)?;
        thread::sleep(Duration::from_millis(self.config.paste_delay_ms));
        self.simulate_paste()?;
        Ok(())
    }

    fn simulate_paste(&self) -> Result<()> {
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

            // macOS virtual key code 9 = 'V'
            // kCGEventFlagMaskCommand = NX_COMMANDMASK = 0x00100000 (1 << 20)
            let cmd_flag: u64 = 0x00100000;

            let key_down = CGEventCreateKeyboardEvent(std::ptr::null(), 9, true);
            if key_down.is_null() {
                anyhow::bail!("Failed to create CGEvent for Cmd+V");
            }
            CGEventSetFlags(key_down, cmd_flag);

            let key_up = CGEventCreateKeyboardEvent(std::ptr::null(), 9, false);
            if key_up.is_null() {
                CFRelease(key_down);
                anyhow::bail!("Failed to create CGEvent for Cmd+V key up");
            }
            CGEventSetFlags(key_up, cmd_flag);

            // Post at kCGAnnotatedSessionEventTap (1) for session-level delivery
            CGEventPost(0, key_down);
            thread::sleep(Duration::from_millis(20));
            CGEventPost(0, key_up);

            CFRelease(key_down);
            CFRelease(key_up);
        }

        Ok(())
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
