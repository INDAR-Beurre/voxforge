use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    pub text: String,
    pub language: Option<String>,
    pub duration_ms: u64,
    pub confidence: Option<f32>,
    pub segments: Vec<TranscriptionSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    pub start_ms: u64,
    pub end_ms: u64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TranscriptionProvider {
    Local,
    Cloud(String),
}

pub trait TranscriptionEngine: Send + Sync {
    fn transcribe(&self, audio: &[f32], sample_rate: u32) -> Result<TranscriptionResult>;
    fn provider(&self) -> TranscriptionProvider;
    fn is_available(&self) -> bool;
}
