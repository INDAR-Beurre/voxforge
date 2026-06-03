use super::engine::{TranscriptionEngine, TranscriptionProvider, TranscriptionResult, TranscriptionSegment};
use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudProviderConfig {
    pub name: String,
    pub api_url: String,
    pub api_key: String,
    pub model: Option<String>,
    pub language: Option<String>,
}

pub struct CloudProvider {
    config: CloudProviderConfig,
    client: reqwest::Client,
}

impl CloudProvider {
    pub fn new(config: CloudProviderConfig) -> Self {
        Self {
            config,
            client: reqwest::Client::new(),
        }
    }

    pub fn openai_compatible(api_key: String, api_url: String) -> Self {
        Self::new(CloudProviderConfig {
            name: "openai-compatible".to_string(),
            api_url,
            api_key,
            model: Some("whisper-1".to_string()),
            language: None,
        })
    }
}

impl TranscriptionEngine for CloudProvider {
    fn transcribe(&self, audio: &[f32], sample_rate: u32) -> Result<TranscriptionResult> {
        let start = std::time::Instant::now();

        let wav_bytes = crate::audio::processor::AudioProcessor::to_wav_bytes(audio, sample_rate)?;

        let rt = tokio::runtime::Handle::try_current()
            .unwrap_or_else(|_| {
                tokio::runtime::Runtime::new().unwrap().handle().clone()
            });

        let response = rt.block_on(async {
            let form = reqwest::multipart::Form::new()
                .part(
                    "file",
                    reqwest::multipart::Part::bytes(wav_bytes)
                        .file_name("audio.wav")
                        .mime_str("audio/wav")?,
                )
                .text("model", self.config.model.clone().unwrap_or_else(|| "whisper-1".to_string()))
                .text("response_format", "verbose_json");

            let resp = self
                .client
                .post(&self.config.api_url)
                .header("Authorization", format!("Bearer {}", self.config.api_key))
                .multipart(form)
                .send()
                .await?;

            if !resp.status().is_success() {
                let status = resp.status();
                let body = resp.text().await.unwrap_or_default();
                return Err(anyhow!("Cloud API error {}: {}", status, body));
            }

            resp.json::<CloudTranscriptionResponse>().await.map_err(|e| anyhow!("{}", e))
        })?;

        let duration_ms = start.elapsed().as_millis() as u64;

        let segments = response
            .segments
            .unwrap_or_default()
            .into_iter()
            .map(|s| TranscriptionSegment {
                start_ms: (s.start * 1000.0) as u64,
                end_ms: (s.end * 1000.0) as u64,
                text: s.text,
            })
            .collect();

        Ok(TranscriptionResult {
            text: response.text.trim().to_string(),
            language: response.language,
            duration_ms,
            confidence: None,
            segments,
        })
    }

    fn provider(&self) -> TranscriptionProvider {
        TranscriptionProvider::Cloud(self.config.name.clone())
    }

    fn is_available(&self) -> bool {
        !self.config.api_key.is_empty()
    }
}

#[derive(Debug, Deserialize)]
struct CloudTranscriptionResponse {
    text: String,
    language: Option<String>,
    segments: Option<Vec<CloudSegment>>,
}

#[derive(Debug, Deserialize)]
struct CloudSegment {
    start: f64,
    end: f64,
    text: String,
}
