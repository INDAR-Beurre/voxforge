use crate::database::history::TranscriptionRecord;
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_history(
    state: State<'_, AppState>,
    limit: u32,
    offset: u32,
) -> Result<Vec<TranscriptionRecord>, String> {
    let db = state.database.lock().map_err(|e| e.to_string())?;
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.get_transcriptions(limit, offset).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_history(
    state: State<'_, AppState>,
    query: String,
    limit: u32,
) -> Result<Vec<TranscriptionRecord>, String> {
    let db = state.database.lock().map_err(|e| e.to_string())?;
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.search_transcriptions(&query, limit).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn toggle_favorite(
    state: State<'_, AppState>,
    id: String,
) -> Result<bool, String> {
    let db = state.database.lock().map_err(|e| e.to_string())?;
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.toggle_favorite(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_transcription(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let db = state.database.lock().map_err(|e| e.to_string())?;
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.delete_transcription(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_history_count(
    state: State<'_, AppState>,
) -> Result<u64, String> {
    let db = state.database.lock().map_err(|e| e.to_string())?;
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.get_transcription_count().map_err(|e| e.to_string())
}
