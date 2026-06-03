use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhisperModel {
    pub id: String,
    pub name: String,
    pub size_bytes: u64,
    pub description: String,
    pub download_url: String,
    pub filename: String,
    pub recommended: bool,
    pub status: ModelStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ModelStatus {
    NotDownloaded,
    Downloading { progress: f32 },
    Downloaded,
    Active,
}

pub struct ModelManager {
    models_dir: PathBuf,
    active_model: Option<String>,
}

impl ModelManager {
    pub fn new(models_dir: PathBuf) -> Result<Self> {
        std::fs::create_dir_all(&models_dir)?;
        Ok(Self {
            models_dir,
            active_model: None,
        })
    }

    pub fn available_models(&self) -> Vec<WhisperModel> {
        let base_url = "https://huggingface.co/ggerganov/whisper.cpp/resolve/main";

        vec![
            WhisperModel {
                id: "tiny".to_string(),
                name: "Tiny".to_string(),
                size_bytes: 75_000_000,
                description: "Fastest, lowest accuracy. Good for testing.".to_string(),
                download_url: format!("{}/ggml-tiny.bin", base_url),
                filename: "ggml-tiny.bin".to_string(),
                recommended: false,
                status: self.get_model_status("ggml-tiny.bin"),
            },
            WhisperModel {
                id: "base".to_string(),
                name: "Base".to_string(),
                size_bytes: 142_000_000,
                description: "Recommended balance of speed and accuracy.".to_string(),
                download_url: format!("{}/ggml-base.bin", base_url),
                filename: "ggml-base.bin".to_string(),
                recommended: true,
                status: self.get_model_status("ggml-base.bin"),
            },
            WhisperModel {
                id: "small".to_string(),
                name: "Small".to_string(),
                size_bytes: 466_000_000,
                description: "Better accuracy, moderate speed.".to_string(),
                download_url: format!("{}/ggml-small.bin", base_url),
                filename: "ggml-small.bin".to_string(),
                recommended: false,
                status: self.get_model_status("ggml-small.bin"),
            },
            WhisperModel {
                id: "medium".to_string(),
                name: "Medium".to_string(),
                size_bytes: 1_500_000_000,
                description: "High accuracy, slower transcription.".to_string(),
                download_url: format!("{}/ggml-medium.bin", base_url),
                filename: "ggml-medium.bin".to_string(),
                recommended: false,
                status: self.get_model_status("ggml-medium.bin"),
            },
            WhisperModel {
                id: "large-v3".to_string(),
                name: "Large v3".to_string(),
                size_bytes: 3_100_000_000,
                description: "Best accuracy, requires significant resources.".to_string(),
                download_url: format!("{}/ggml-large-v3.bin", base_url),
                filename: "ggml-large-v3.bin".to_string(),
                recommended: false,
                status: self.get_model_status("ggml-large-v3.bin"),
            },
        ]
    }

    fn get_model_status(&self, filename: &str) -> ModelStatus {
        let path = self.models_dir.join(filename);
        if path.exists() {
            if self.active_model.as_deref() == Some(filename) {
                ModelStatus::Active
            } else {
                ModelStatus::Downloaded
            }
        } else {
            ModelStatus::NotDownloaded
        }
    }

    pub fn model_path(&self, model_id: &str) -> Result<PathBuf> {
        let models = self.available_models();
        let model = models
            .iter()
            .find(|m| m.id == model_id)
            .ok_or_else(|| anyhow!("Model not found: {}", model_id))?;

        let path = self.models_dir.join(&model.filename);
        if !path.exists() {
            return Err(anyhow!("Model not downloaded: {}", model_id));
        }
        Ok(path)
    }

    pub fn set_active_model(&mut self, model_id: &str) -> Result<PathBuf> {
        let path = self.model_path(model_id)?;
        let models = self.available_models();
        let model = models.iter().find(|m| m.id == model_id).unwrap();
        self.active_model = Some(model.filename.clone());
        Ok(path)
    }

    pub fn delete_model(&self, model_id: &str) -> Result<()> {
        let path = self.model_path(model_id)?;
        std::fs::remove_file(&path)?;
        Ok(())
    }

    pub fn models_dir(&self) -> &PathBuf {
        &self.models_dir
    }

    pub fn disk_usage(&self) -> Result<u64> {
        let mut total = 0u64;
        if self.models_dir.exists() {
            for entry in std::fs::read_dir(&self.models_dir)? {
                let entry = entry?;
                if entry.path().extension().map(|e| e == "bin").unwrap_or(false) {
                    total += entry.metadata()?.len();
                }
            }
        }
        Ok(total)
    }
}
