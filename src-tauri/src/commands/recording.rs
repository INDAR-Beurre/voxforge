use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn start_recording(state: State<'_, AppState>) -> Result<(), String> {
    let mut capture = state.audio_capture.lock().map_err(|e| e.to_string())?;
    capture.start_recording().map_err(|e| e.to_string())?;
    *state.recording_mode.lock().map_err(|e| e.to_string())? = crate::state::RecordingState::Recording;
    Ok(())
}

#[tauri::command]
pub async fn stop_recording(state: State<'_, AppState>) -> Result<Vec<f32>, String> {
    let mut capture = state.audio_capture.lock().map_err(|e| e.to_string())?;
    let samples = capture.stop_recording().map_err(|e| e.to_string())?;
    *state.recording_mode.lock().map_err(|e| e.to_string())? = crate::state::RecordingState::Idle;
    Ok(samples)
}

#[tauri::command]
pub async fn get_recording_state(state: State<'_, AppState>) -> Result<String, String> {
    let mode = state.recording_mode.lock().map_err(|e| e.to_string())?.clone();
    serde_json::to_string(&mode).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_audio_level(state: State<'_, AppState>) -> Result<f32, String> {
    let capture = state.audio_capture.lock().map_err(|e| e.to_string())?;
    Ok(capture.get_level())
}

#[tauri::command]
pub async fn list_audio_devices(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let capture = state.audio_capture.lock().map_err(|e| e.to_string())?;
    Ok(capture.list_devices())
}

#[tauri::command]
pub async fn set_audio_device(state: State<'_, AppState>, name: String) -> Result<(), String> {
    let mut capture = state.audio_capture.lock().map_err(|e| e.to_string())?;
    capture.set_device(&name).map_err(|e| e.to_string())
}
