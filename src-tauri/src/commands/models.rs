use crate::models::manager::WhisperModel;
use crate::state::AppState;
use tauri::{Emitter, State};

#[tauri::command]
pub async fn get_available_models(
    state: State<'_, AppState>,
) -> Result<Vec<WhisperModel>, String> {
    let manager = state.model_manager.lock().map_err(|e| e.to_string())?;
    let manager = manager.as_ref().ok_or("Model manager not initialized")?;
    Ok(manager.available_models())
}

#[tauri::command]
pub async fn download_model(
    app: tauri::AppHandle,
    state: State<'_, AppState>,
    model_id: String,
) -> Result<(), String> {
    let (url, dest) = {
        let manager = state.model_manager.lock().map_err(|e| e.to_string())?;
        let manager = manager.as_ref().ok_or("Model manager not initialized")?;
        let models = manager.available_models();
        let model = models
            .iter()
            .find(|m| m.id == model_id)
            .ok_or_else(|| format!("Model not found: {}", model_id))?;
        let dest = manager.models_dir().join(&model.filename);
        (model.download_url.clone(), dest)
    };

    let app_handle = app.clone();
    let mid = model_id.clone();

    tokio::spawn(async move {
        let result = crate::models::downloader::ModelDownloader::download(
            &url,
            dest,
            move |progress| {
                let _ = app_handle.emit(
                    "model-download-progress",
                    serde_json::json!({
                        "model_id": mid,
                        "progress": progress,
                    }),
                );
            },
        )
        .await;

        match result {
            Ok(_) => {
                let _ = app.emit(
                    "model-download-complete",
                    serde_json::json!({
                        "model_id": model_id,
                        "success": true,
                    }),
                );
            }
            Err(e) => {
                let _ = app.emit(
                    "model-download-complete",
                    serde_json::json!({
                        "model_id": model_id,
                        "success": false,
                        "error": e.to_string(),
                    }),
                );
            }
        }
    });

    Ok(())
}

#[tauri::command]
pub async fn set_active_model(
    state: State<'_, AppState>,
    model_id: String,
) -> Result<(), String> {
    let model_path = {
        let mut manager = state.model_manager.lock().map_err(|e| e.to_string())?;
        let manager = manager.as_mut().ok_or("Model manager not initialized")?;
        manager.set_active_model(&model_id).map_err(|e| e.to_string())?
    };

    let whisper = crate::transcription::WhisperLocal::new(model_path);
    whisper.load_model().map_err(|e| e.to_string())?;
    *state.whisper.lock().map_err(|e| e.to_string())? = Some(whisper);

    Ok(())
}

#[tauri::command]
pub async fn delete_model(
    state: State<'_, AppState>,
    model_id: String,
) -> Result<(), String> {
    let manager = state.model_manager.lock().map_err(|e| e.to_string())?;
    let manager = manager.as_ref().ok_or("Model manager not initialized")?;
    manager.delete_model(&model_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_disk_usage(
    state: State<'_, AppState>,
) -> Result<u64, String> {
    let manager = state.model_manager.lock().map_err(|e| e.to_string())?;
    let manager = manager.as_ref().ok_or("Model manager not initialized")?;
    manager.disk_usage().map_err(|e| e.to_string())
}
