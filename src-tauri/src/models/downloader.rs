use anyhow::Result;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

pub struct ModelDownloader;

impl ModelDownloader {
    pub async fn download(
        url: &str,
        dest: PathBuf,
        progress_callback: impl Fn(f32) + Send + 'static,
    ) -> Result<()> {
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;

        let total_size = response.content_length().unwrap_or(0);
        let mut downloaded: u64 = 0;

        let temp_path = dest.with_extension("bin.part");
        let mut file = tokio::fs::File::create(&temp_path).await?;

        let mut stream = response.bytes_stream();
        use futures_util::StreamExt;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;

            if total_size > 0 {
                let progress = downloaded as f32 / total_size as f32;
                progress_callback(progress);
            }
        }

        file.flush().await?;
        drop(file);

        tokio::fs::rename(&temp_path, &dest).await?;

        Ok(())
    }

    pub async fn cancel_download(dest: PathBuf) -> Result<()> {
        let temp_path = dest.with_extension("bin.part");
        if temp_path.exists() {
            tokio::fs::remove_file(&temp_path).await?;
        }
        Ok(())
    }
}
