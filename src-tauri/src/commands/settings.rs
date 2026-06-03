use crate::state::AppState;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub hotkey_mode: String,
    pub push_to_talk_key: String,
    pub toggle_key: String,
    pub transcription_provider: String,
    pub language: String,
    pub auto_detect_language: bool,
    pub silence_timeout_ms: u64,
    pub play_sounds: bool,
    pub start_sound: bool,
    pub stop_sound: bool,
    pub injection_strategy: String,
    pub preserve_clipboard: bool,
    pub appearance: String,
    pub compact_mode: bool,
    pub translucency: bool,
    pub reduced_motion: bool,
    pub sound_cues: bool,
    pub cloud_provider_url: String,
    pub cloud_provider_name: String,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            hotkey_mode: "push_to_talk".to_string(),
            push_to_talk_key: "CommandOrControl+Shift+Space".to_string(),
            toggle_key: "CommandOrControl+Shift+V".to_string(),
            transcription_provider: "local".to_string(),
            language: "en".to_string(),
            auto_detect_language: false,
            silence_timeout_ms: 2000,
            play_sounds: true,
            start_sound: true,
            stop_sound: true,
            injection_strategy: "clipboard_paste".to_string(),
            preserve_clipboard: true,
            appearance: "system".to_string(),
            compact_mode: false,
            translucency: true,
            reduced_motion: false,
            sound_cues: true,
            cloud_provider_url: String::new(),
            cloud_provider_name: String::new(),
        }
    }
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let db = state.database.lock().map_err(|e| e.to_string())?;
    let db = db.as_ref().ok_or("Database not initialized")?;

    let mut settings = AppSettings::default();

    if let Ok(Some(v)) = db.get_setting("hotkey_mode") { settings.hotkey_mode = v; }
    if let Ok(Some(v)) = db.get_setting("push_to_talk_key") { settings.push_to_talk_key = v; }
    if let Ok(Some(v)) = db.get_setting("toggle_key") { settings.toggle_key = v; }
    if let Ok(Some(v)) = db.get_setting("transcription_provider") { settings.transcription_provider = v; }
    if let Ok(Some(v)) = db.get_setting("language") { settings.language = v; }
    if let Ok(Some(v)) = db.get_setting("auto_detect_language") { settings.auto_detect_language = v == "true"; }
    if let Ok(Some(v)) = db.get_setting("silence_timeout_ms") { settings.silence_timeout_ms = v.parse().unwrap_or(2000); }
    if let Ok(Some(v)) = db.get_setting("play_sounds") { settings.play_sounds = v == "true"; }
    if let Ok(Some(v)) = db.get_setting("injection_strategy") { settings.injection_strategy = v; }
    if let Ok(Some(v)) = db.get_setting("preserve_clipboard") { settings.preserve_clipboard = v == "true"; }
    if let Ok(Some(v)) = db.get_setting("appearance") { settings.appearance = v; }
    if let Ok(Some(v)) = db.get_setting("compact_mode") { settings.compact_mode = v == "true"; }
    if let Ok(Some(v)) = db.get_setting("translucency") { settings.translucency = v == "true"; }
    if let Ok(Some(v)) = db.get_setting("reduced_motion") { settings.reduced_motion = v == "true"; }
    if let Ok(Some(v)) = db.get_setting("sound_cues") { settings.sound_cues = v == "true"; }
    if let Ok(Some(v)) = db.get_setting("cloud_provider_url") { settings.cloud_provider_url = v; }
    if let Ok(Some(v)) = db.get_setting("cloud_provider_name") { settings.cloud_provider_name = v; }

    Ok(settings)
}

#[tauri::command]
pub async fn save_setting(
    state: State<'_, AppState>,
    key: String,
    value: String,
) -> Result<(), String> {
    let db = state.database.lock().map_err(|e| e.to_string())?;
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.set_setting(&key, &value).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_settings(
    state: State<'_, AppState>,
    settings: AppSettings,
) -> Result<(), String> {
    let db = state.database.lock().map_err(|e| e.to_string())?;
    let db = db.as_ref().ok_or("Database not initialized")?;

    db.set_setting("hotkey_mode", &settings.hotkey_mode).map_err(|e| e.to_string())?;
    db.set_setting("push_to_talk_key", &settings.push_to_talk_key).map_err(|e| e.to_string())?;
    db.set_setting("toggle_key", &settings.toggle_key).map_err(|e| e.to_string())?;
    db.set_setting("transcription_provider", &settings.transcription_provider).map_err(|e| e.to_string())?;
    db.set_setting("language", &settings.language).map_err(|e| e.to_string())?;
    db.set_setting("auto_detect_language", &settings.auto_detect_language.to_string()).map_err(|e| e.to_string())?;
    db.set_setting("silence_timeout_ms", &settings.silence_timeout_ms.to_string()).map_err(|e| e.to_string())?;
    db.set_setting("play_sounds", &settings.play_sounds.to_string()).map_err(|e| e.to_string())?;
    db.set_setting("injection_strategy", &settings.injection_strategy).map_err(|e| e.to_string())?;
    db.set_setting("preserve_clipboard", &settings.preserve_clipboard.to_string()).map_err(|e| e.to_string())?;
    db.set_setting("appearance", &settings.appearance).map_err(|e| e.to_string())?;
    db.set_setting("compact_mode", &settings.compact_mode.to_string()).map_err(|e| e.to_string())?;
    db.set_setting("translucency", &settings.translucency.to_string()).map_err(|e| e.to_string())?;
    db.set_setting("reduced_motion", &settings.reduced_motion.to_string()).map_err(|e| e.to_string())?;
    db.set_setting("sound_cues", &settings.sound_cues.to_string()).map_err(|e| e.to_string())?;
    db.set_setting("cloud_provider_url", &settings.cloud_provider_url).map_err(|e| e.to_string())?;
    db.set_setting("cloud_provider_name", &settings.cloud_provider_name).map_err(|e| e.to_string())?;

    Ok(())
}
