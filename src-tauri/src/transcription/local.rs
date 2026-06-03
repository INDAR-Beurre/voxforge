use super::engine::{TranscriptionEngine, TranscriptionProvider, TranscriptionResult, TranscriptionSegment};
use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::sync::Mutex;
use whisper_rs::{FullParams, SamplingStrategy, WhisperContext, WhisperContextParameters};

pub struct WhisperLocal {
    context: Mutex<Option<WhisperContext>>,
    model_path: PathBuf,
    language: Option<String>,
}

unsafe impl Send for WhisperLocal {}
unsafe impl Sync for WhisperLocal {}

impl WhisperLocal {
    pub fn new(model_path: PathBuf) -> Self {
        Self {
            context: Mutex::new(None),
            model_path,
            language: None,
        }
    }

    pub fn set_language(&mut self, language: Option<String>) {
        self.language = language;
    }

    pub fn load_model(&self) -> Result<()> {
        let params = WhisperContextParameters::default();
        let ctx = WhisperContext::new_with_params(
            self.model_path.to_str().ok_or_else(|| anyhow!("Invalid model path"))?,
            params,
        )
        .map_err(|e| anyhow!("Failed to load Whisper model: {}", e))?;

        *self.context.lock().unwrap() = Some(ctx);
        Ok(())
    }

    pub fn is_loaded(&self) -> bool {
        self.context.lock().unwrap().is_some()
    }

    pub fn unload_model(&self) {
        *self.context.lock().unwrap() = None;
    }
}

impl TranscriptionEngine for WhisperLocal {
    fn transcribe(&self, audio: &[f32], _sample_rate: u32) -> Result<TranscriptionResult> {
        let guard = self.context.lock().unwrap();
        let ctx = guard
            .as_ref()
            .ok_or_else(|| anyhow!("Model not loaded"))?;

        let mut state = ctx.create_state().map_err(|e| anyhow!("Failed to create state: {}", e))?;

        let mut params = FullParams::new(SamplingStrategy::Greedy { best_of: 1 });
        params.set_print_progress(false);
        params.set_print_special(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        params.set_suppress_blank(true);
        params.set_suppress_non_speech_tokens(true);

        if let Some(ref lang) = self.language {
            params.set_language(Some(lang));
        } else {
            params.set_language(Some("en"));
        }

        let start = std::time::Instant::now();

        state
            .full(params, audio)
            .map_err(|e| anyhow!("Transcription failed: {}", e))?;

        let duration_ms = start.elapsed().as_millis() as u64;

        let num_segments = state.full_n_segments().map_err(|e| anyhow!("Failed to get segments: {}", e))?;
        let mut text = String::new();
        let mut segments = Vec::new();

        for i in 0..num_segments {
            let segment_text = state
                .full_get_segment_text(i)
                .map_err(|e| anyhow!("Failed to get segment text: {}", e))?;
            let start_ms = (state.full_get_segment_t0(i).map_err(|e| anyhow!("{}", e))? * 10) as u64;
            let end_ms = (state.full_get_segment_t1(i).map_err(|e| anyhow!("{}", e))? * 10) as u64;

            text.push_str(&segment_text);
            segments.push(TranscriptionSegment {
                start_ms,
                end_ms,
                text: segment_text,
            });
        }

        Ok(TranscriptionResult {
            text: text.trim().to_string(),
            language: self.language.clone(),
            duration_ms,
            confidence: None,
            segments,
        })
    }

    fn provider(&self) -> TranscriptionProvider {
        TranscriptionProvider::Local
    }

    fn is_available(&self) -> bool {
        self.is_loaded()
    }
}
