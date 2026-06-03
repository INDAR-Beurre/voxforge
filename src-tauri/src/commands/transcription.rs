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

    if resampled.is_empty() {
        return Err("No audio captured".to_string());
    }

    let whisper = state.whisper.lock().map_err(|e| e.to_string())?;
    let whisper_ref = whisper.as_ref().ok_or(
        "No transcription model loaded. Please download a model in the Models tab first."
    )?;

    whisper_ref
        .transcribe(&resampled, 16000)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn transcribe_and_inject(
    state: State<'_, AppState>,
) -> Result<TranscriptionResult, String> {
    {
        *state.recording_mode.lock().map_err(|e| e.to_string())? = crate::state::RecordingState::Processing;
    }

    // Get the focused app BEFORE we do anything else (it's still focused)
    let target_app = {
        let injector = state.text_injector.lock().map_err(|e| e.to_string())?;
        injector.get_focused_app()
    };

    let (samples, sample_rate) = {
        let mut capture = state.audio_capture.lock().map_err(|e| e.to_string())?;
        let samples = capture.stop_recording().map_err(|e| e.to_string())?;
        let rate = capture.sample_rate();
        (samples, rate)
    };

    if samples.is_empty() {
        *state.recording_mode.lock().map_err(|e| e.to_string())? = crate::state::RecordingState::Idle;
        return Err("No audio captured. Make sure your microphone is working.".to_string());
    }

    log::info!("Got {} samples at {}Hz ({:.1}s)", samples.len(), sample_rate, samples.len() as f32 / sample_rate as f32);

    let resampled = if sample_rate != 16000 {
        log::info!("Resampling from {} to 16000", sample_rate);
        AudioProcessor::resample(&samples, sample_rate, 16000)
    } else {
        samples
    };

    let result = {
        let whisper = state.whisper.lock().map_err(|e| e.to_string())?;
        let whisper_ref = whisper.as_ref().ok_or(
            "No transcription model loaded. Go to Models tab and download one first."
        )?;
        whisper_ref
            .transcribe(&resampled, 16000)
            .map_err(|e| format!("Transcription failed: {}", e))?
    };

    if result.text.is_empty() {
        *state.recording_mode.lock().map_err(|e| e.to_string())? = crate::state::RecordingState::Idle;
        return Err("No speech detected. Try speaking louder or longer.".to_string());
    }

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

    // Inject text into focused app
    {
        let injector = state.text_injector.lock().map_err(|e| e.to_string())?;
        injector.inject(&text).map_err(|e| format!("Text injection failed: {}", e))?;
    }

    // Save to history
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
