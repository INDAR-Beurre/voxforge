use crate::database::stats::{DailyStats, UsageStats};
use crate::state::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_overall_stats(state: State<'_, AppState>) -> Result<UsageStats, String> {
    let db = state.database.lock().map_err(|e| e.to_string())?;
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.get_overall_stats().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_daily_stats(
    state: State<'_, AppState>,
    days: u32,
) -> Result<Vec<DailyStats>, String> {
    let db = state.database.lock().map_err(|e| e.to_string())?;
    let db = db.as_ref().ok_or("Database not initialized")?;
    db.get_daily_stats(days).map_err(|e| e.to_string())
}
