pub mod audio;
pub mod commands;
pub mod database;
pub mod injection;
pub mod models;
pub mod postprocess;
pub mod state;
pub mod transcription;

use state::AppState;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();

    let app_state = AppState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .manage(app_state)
        .setup(|app| {
            let app_handle = app.handle().clone();
            initialize_app(&app_handle)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::recording::start_recording,
            commands::recording::stop_recording,
            commands::recording::get_recording_state,
            commands::recording::get_audio_level,
            commands::recording::list_audio_devices,
            commands::recording::set_audio_device,
            commands::transcription::transcribe_audio,
            commands::transcription::transcribe_and_inject,
            commands::history::get_history,
            commands::history::search_history,
            commands::history::toggle_favorite,
            commands::history::delete_transcription,
            commands::history::get_history_count,
            commands::dictionary::get_dictionary,
            commands::dictionary::add_dictionary_entry,
            commands::dictionary::delete_dictionary_entry,
            commands::dictionary::seed_developer_dictionary,
            commands::models::get_available_models,
            commands::models::download_model,
            commands::models::set_active_model,
            commands::models::delete_model,
            commands::models::get_disk_usage,
            commands::settings::get_settings,
            commands::settings::save_setting,
            commands::settings::save_settings,
            commands::stats::get_overall_stats,
            commands::stats::get_daily_stats,
            commands::injection::inject_text,
            commands::injection::get_focused_app,
            commands::permissions::open_accessibility_settings,
            commands::permissions::open_microphone_settings,
            commands::permissions::check_accessibility_permission,
            commands::permissions::check_microphone_permission,
            commands::permissions::request_accessibility_permission,
            commands::permissions::request_microphone_permission,
        ])
        .run(tauri::generate_context!())
        .expect("error while running VoxForge");
}

fn initialize_app(app_handle: &tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");

    std::fs::create_dir_all(&app_data_dir)?;

    let db_path = app_data_dir.join("voxforge.db");
    let db = database::Database::new(db_path)?;

    let models_dir = app_data_dir.join("models");
    let model_manager = models::ModelManager::new(models_dir.clone())?;

    let state = app_handle.state::<AppState>();
    *state.database.lock().unwrap() = Some(db);
    *state.model_manager.lock().unwrap() = Some(model_manager);

    // Trigger microphone permission prompt by briefly opening an input stream
    {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
        let host = cpal::default_host();
        if let Some(device) = host.default_input_device() {
            if let Ok(config) = device.default_input_config() {
                let stream_config = cpal::StreamConfig {
                    channels: 1,
                    sample_rate: config.sample_rate(),
                    buffer_size: cpal::BufferSize::Default,
                };
                if let Ok(stream) = device.build_input_stream(
                    &stream_config,
                    |_data: &[f32], _| {},
                    |_err| {},
                    None,
                ) {
                    let _ = stream.play();
                    std::thread::sleep(std::time::Duration::from_millis(100));
                    drop(stream);
                }
            }
        }
        log::info!("Microphone permission triggered");
    }

    // Auto-load first available model
    let model_files = ["ggml-base.bin", "ggml-small.bin", "ggml-tiny.bin", "ggml-medium.bin", "ggml-large-v3.bin"];
    for filename in &model_files {
        let path = models_dir.join(filename);
        if path.exists() {
            log::info!("Auto-loading model: {}", filename);
            let whisper = transcription::WhisperLocal::new(path);
            match whisper.load_model() {
                Ok(_) => {
                    *state.whisper.lock().unwrap() = Some(whisper);
                    log::info!("Model {} loaded successfully", filename);
                    break;
                }
                Err(e) => {
                    log::error!("Failed to load model {}: {}", filename, e);
                }
            }
        }
    }

    Ok(())
}
