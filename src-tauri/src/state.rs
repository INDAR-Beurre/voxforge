use crate::audio::AudioCapture;
use crate::database::Database;
use crate::injection::TextInjector;
use crate::models::ModelManager;
use crate::postprocess::PostProcessingRule;
use crate::transcription::WhisperLocal;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecordingState {
    Idle,
    Recording,
    Processing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HotkeyMode {
    PushToTalk,
    Toggle,
}

pub struct AppState {
    pub audio_capture: Mutex<AudioCapture>,
    pub whisper: Mutex<Option<WhisperLocal>>,
    pub text_injector: Mutex<TextInjector>,
    pub database: Mutex<Option<Database>>,
    pub model_manager: Mutex<Option<ModelManager>>,
    pub recording_mode: Mutex<RecordingState>,
    pub hotkey_mode: Mutex<HotkeyMode>,
    pub post_processing_rules: Mutex<Vec<PostProcessingRule>>,
}

impl AppState {
    pub fn new() -> Self {
        let audio_capture = AudioCapture::new().expect("Failed to initialize audio capture");
        let text_injector = TextInjector::new(Default::default());

        Self {
            audio_capture: Mutex::new(audio_capture),
            whisper: Mutex::new(None),
            text_injector: Mutex::new(text_injector),
            database: Mutex::new(None),
            model_manager: Mutex::new(None),
            recording_mode: Mutex::new(RecordingState::Idle),
            hotkey_mode: Mutex::new(HotkeyMode::PushToTalk),
            post_processing_rules: Mutex::new(PostProcessingRule::defaults()),
        }
    }

    pub fn get_post_processing_rules(&self) -> Vec<PostProcessingRule> {
        self.post_processing_rules.lock().unwrap().clone()
    }
}
