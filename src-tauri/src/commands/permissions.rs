#[cfg(target_os = "macos")]
use std::process::Command;

#[tauri::command]
pub async fn open_accessibility_settings() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        // Try macOS 13+ URL first, fall back to older one
        let result = Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Accessibility")
            .spawn();

        if result.is_err() {
            Command::new("open")
                .arg("x-apple.systempreferences:com.apple.settings.PrivacySecurity.extension?Privacy_Accessibility")
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Windows doesn't require accessibility permissions
        log::info!("Windows doesn't require accessibility permissions");
    }

    Ok(())
}

#[tauri::command]
pub async fn open_microphone_settings() -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        // macOS 13+ (Ventura)
        let result = Command::new("open")
            .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_Microphone")
            .spawn();

        if result.is_err() {
            Command::new("open")
                .arg("x-apple.systempreferences:com.apple.settings.PrivacySecurity.extension?Privacy_Microphone")
                .spawn()
                .map_err(|e| e.to_string())?;
        }
    }

    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        // Open Windows Privacy > Microphone settings
        Command::new("cmd")
            .args(&["/C", "start", "ms-settings:privacy-microphone"])
            .spawn()
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
pub async fn check_accessibility_permission() -> Result<bool, String> {
    #[cfg(target_os = "macos")]
    {
        extern "C" {
            fn AXIsProcessTrusted() -> bool;
        }
        let trusted = unsafe { AXIsProcessTrusted() };
        Ok(trusted)
    }

    #[cfg(not(target_os = "macos"))]
    {
        Ok(true)
    }
}

#[tauri::command]
pub async fn check_microphone_permission() -> Result<String, String> {
    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        // Try to list audio devices - if we get any, we have permission
        // If permission was never asked, attempting to record will trigger the prompt
        let output = Command::new("osascript")
            .arg("-e")
            .arg(r#"do shell script "system_profiler SPAudioDataType 2>/dev/null | head -1""#)
            .output()
            .map_err(|e| e.to_string())?;

        if output.status.success() {
            Ok("granted".to_string())
        } else {
            Ok("unknown".to_string())
        }
    }

    #[cfg(target_os = "windows")]
    {
        // On Windows, assume granted - actual check happens when recording starts
        Ok("granted".to_string())
    }
}

#[tauri::command]
pub async fn request_accessibility_permission() -> Result<bool, String> {
    #[cfg(target_os = "macos")]
    {
        use core_foundation::base::TCFType;
        use core_foundation::boolean::CFBoolean;
        use core_foundation::dictionary::CFDictionary;
        use core_foundation::string::CFString;

        extern "C" {
            fn AXIsProcessTrustedWithOptions(
                options: core_foundation::base::CFTypeRef,
            ) -> bool;
        }

        let key = CFString::new("AXTrustedCheckOptionPrompt");
        let value = CFBoolean::true_value();

        let options = CFDictionary::from_CFType_pairs(&[(
            key.as_CFType(),
            value.as_CFType(),
        )]);

        let trusted = unsafe {
            AXIsProcessTrustedWithOptions(options.as_concrete_TypeRef() as _)
        };

        Ok(trusted)
    }

    #[cfg(not(target_os = "macos"))]
    {
        Ok(true)
    }
}

#[tauri::command]
pub async fn request_microphone_permission() -> Result<bool, String> {
    // Trigger the microphone permission prompt by briefly attempting to capture
    use cpal::traits::{DeviceTrait, HostTrait};

    let host = cpal::default_host();
    match host.default_input_device() {
        Some(device) => {
            // Just checking the config triggers the permission prompt on first use
            match device.default_input_config() {
                Ok(_) => Ok(true),
                Err(_) => Ok(false),
            }
        }
        None => Ok(false),
    }
}
