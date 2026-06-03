use crate::audio::AudioProcessor;
use crate::state::AppState;
use crate::transcription::{TranscriptionEngine, TranscriptionResult};
use tauri::State;

#[tauri::command]
pub async fn transcribe_audio(
    state: State<'_, AppState>,
    samples: Vec<f32>,
) -> Result<TranscriptionResult, String> {
    let sample_rate = {
        let capture = state.audio_capture.lock().map_err(|e| e.to_string())?;
        capture.sample_rate()
    };

    let resampled = if sample_rate != 16000 {
        AudioProcessor::resample(&samples, sample_rate, 16000)
    } else {
        samples
    };

    let whisper = state.whisper.lock().map_err(|e| e.to_string())?;
    let whisper_ref = whisper.as_ref().ok_or("Whisper model not loaded")?;

    let result = whisper_ref
        .transcribe(&resampled, 16000)
        .map_err(|e| e.to_string())?;

    Ok(result)
}

#[tauri::command]
pub async fn transcribe_and_inject(
    state: State<'_, AppState>,
) -> Result<TranscriptionResult, String> {
    {
        *state.recording_mode.lock().map_err(|e| e.to_string())? = crate::state::RecordingState::Processing;
    }

    let (samples, sample_rate) = {
        let mut capture = state.audio_capture.lock().map_err(|e| e.to_string())?;
        let samples = capture.stop_recording().map_err(|e| e.to_string())?;
        let rate = capture.sample_rate();
        (samples, rate)
    };

    let resampled = if sample_rate != 16000 {
        AudioProcessor::resample(&samples, sample_rate, 16000)
    } else {
        samples
    };

    let result = {
        let whisper = state.whisper.lock().map_err(|e| e.to_string())?;
        let whisper_ref = whisper.as_ref().ok_or("Whisper model not loaded")?;
        whisper_ref
            .transcribe(&resampled, 16000)
            .map_err(|e| e.to_string())?
    };

    let mut text = result.text.clone();

    // Apply dictionary replacements
    {
        let db = state.database.lock().map_err(|e| e.to_string())?;
        if let Some(ref db) = *db {
            if let Ok(replaced) = db.apply_dictionary(&text) {
                text = replaced;
            }
        }
    }

    // Apply post-processing
    text = crate::postprocess::apply_rules(&text, &state.get_post_processing_rules());

    // Inject text
    {
        let injector = state.text_injector.lock().map_err(|e| e.to_string())?;
        injector.inject(&text).map_err(|e| e.to_string())?;
    }

    // Save to history
    let target_app = {
        let injector = state.text_injector.lock().map_err(|e| e.to_string())?;
        injector.get_focused_app()
    };

    {
        let db = state.database.lock().map_err(|e| e.to_string())?;
        if let Some(ref db) = *db {
            let record = crate::database::history::TranscriptionRecord {
                id: uuid::Uuid::new_v4().to_string(),
                text: text.clone(),
                timestamp: chrono::Local::now().to_rfc3339(),
                duration_ms: result.duration_ms,
                word_count: text.split_whitespace().count() as u32,
                mode: format!("{:?}", *state.hotkey_mode.lock().map_err(|e| e.to_string())?),
                provider: "local".to_string(),
                model_name: None,
                language: result.language.clone(),
                target_app,
                is_favorite: false,
            };
            let _ = db.insert_transcription(&record);
            let _ = db.record_transcription_stats(record.word_count, record.duration_ms);
        }
    }

    *state.recording_mode.lock().map_err(|e| e.to_string())? = crate::state::RecordingState::Idle;

    Ok(TranscriptionResult {
        text,
        ..result
    })
}
