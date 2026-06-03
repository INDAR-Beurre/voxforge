use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn inject_text(state: State<'_, AppState>, text: String) -> Result<(), String> {
    let injector = state.text_injector.lock().map_err(|e| e.to_string())?;
    injector.inject(&text).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_focused_app(state: State<'_, AppState>) -> Result<Option<String>, String> {
    let injector = state.text_injector.lock().map_err(|e| e.to_string())?;
    Ok(injector.get_focused_app())
}
