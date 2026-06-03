use anyhow::Result;

pub struct AudioProcessor;

impl AudioProcessor {
    pub fn resample(samples: &[f32], from_rate: u32, to_rate: u32) -> Vec<f32> {
        if from_rate == to_rate {
            return samples.to_vec();
        }

        let ratio = to_rate as f64 / from_rate as f64;
        let output_len = (samples.len() as f64 * ratio) as usize;
        let mut output = Vec::with_capacity(output_len);

        for i in 0..output_len {
            let src_idx = i as f64 / ratio;
            let idx_floor = src_idx.floor() as usize;
            let idx_ceil = (idx_floor + 1).min(samples.len() - 1);
            let frac = src_idx - idx_floor as f64;

            let sample = samples[idx_floor] as f64 * (1.0 - frac)
                + samples[idx_ceil] as f64 * frac;
            output.push(sample as f32);
        }

        output
    }

    pub fn normalize(samples: &[f32]) -> Vec<f32> {
        let max_amplitude = samples
            .iter()
            .map(|s| s.abs())
            .fold(0.0f32, f32::max);

        if max_amplitude < 1e-6 {
            return samples.to_vec();
        }

        let scale = 0.95 / max_amplitude;
        samples.iter().map(|s| s * scale).collect()
    }

    pub fn detect_silence(samples: &[f32], threshold: f32, window_ms: u32, sample_rate: u32) -> bool {
        let window_samples = (sample_rate * window_ms / 1000) as usize;
        if samples.len() < window_samples {
            return false;
        }

        let tail = &samples[samples.len() - window_samples..];
        let rms = (tail.iter().map(|s| s * s).sum::<f32>() / tail.len() as f32).sqrt();
        rms < threshold
    }

    pub fn to_wav_bytes(samples: &[f32], sample_rate: u32) -> Result<Vec<u8>> {
        let mut cursor = std::io::Cursor::new(Vec::new());
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::new(&mut cursor, spec)?;
        for &sample in samples {
            let amplitude = (sample * i16::MAX as f32) as i16;
            writer.write_sample(amplitude)?;
        }
        writer.finalize()?;

        Ok(cursor.into_inner())
    }
}
