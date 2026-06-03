use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, SampleFormat, StreamConfig};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

pub struct AudioCapture {
    host: Host,
    device_name: Option<String>,
    buffer: Arc<Mutex<Vec<f32>>>,
    is_recording: Arc<AtomicBool>,
    sample_rate: u32,
}

unsafe impl Send for AudioCapture {}
unsafe impl Sync for AudioCapture {}

impl AudioCapture {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();

        Ok(Self {
            host,
            device_name: None,
            buffer: Arc::new(Mutex::new(Vec::new())),
            is_recording: Arc::new(AtomicBool::new(false)),
            sample_rate: 16000,
        })
    }

    pub fn list_devices(&self) -> Vec<String> {
        self.host
            .input_devices()
            .map(|devices| devices.filter_map(|d| d.name().ok()).collect())
            .unwrap_or_default()
    }

    pub fn set_device(&mut self, name: &str) -> Result<()> {
        let _device = self
            .host
            .input_devices()?
            .find(|d| d.name().map(|n| n == name).unwrap_or(false))
            .ok_or_else(|| anyhow!("Device not found: {}", name))?;
        self.device_name = Some(name.to_string());
        Ok(())
    }

    fn get_device(&self) -> Result<Device> {
        if let Some(ref name) = self.device_name {
            self.host
                .input_devices()?
                .find(|d| d.name().map(|n| n == *name).unwrap_or(false))
                .ok_or_else(|| anyhow!("Device not found"))
        } else {
            self.host
                .default_input_device()
                .ok_or_else(|| anyhow!("No default input device"))
        }
    }

    pub fn start_recording(&mut self) -> Result<()> {
        let device = self.get_device()?;
        let supported_config = device.default_input_config()?;
        self.sample_rate = supported_config.sample_rate().0;

        let config = StreamConfig {
            channels: 1,
            sample_rate: supported_config.sample_rate(),
            buffer_size: cpal::BufferSize::Default,
        };

        let buffer = self.buffer.clone();
        let is_recording = self.is_recording.clone();

        {
            let mut buf = self.buffer.lock().unwrap();
            buf.clear();
        }
        self.is_recording.store(true, Ordering::SeqCst);

        let stream = match supported_config.sample_format() {
            SampleFormat::F32 => device.build_input_stream(
                &config,
                move |data: &[f32], _| {
                    if is_recording.load(Ordering::SeqCst) {
                        if let Ok(mut buf) = buffer.lock() {
                            buf.extend_from_slice(data);
                        }
                    }
                },
                |err| log::error!("Audio stream error: {}", err),
                None,
            )?,
            SampleFormat::I16 => {
                let buffer = self.buffer.clone();
                let is_recording = self.is_recording.clone();
                device.build_input_stream(
                    &config,
                    move |data: &[i16], _| {
                        if is_recording.load(Ordering::SeqCst) {
                            let floats: Vec<f32> =
                                data.iter().map(|&s| s as f32 / i16::MAX as f32).collect();
                            if let Ok(mut buf) = buffer.lock() {
                                buf.extend_from_slice(&floats);
                            }
                        }
                    },
                    |err| log::error!("Audio stream error: {}", err),
                    None,
                )?
            }
            _ => return Err(anyhow!("Unsupported sample format")),
        };

        stream.play()?;
        // Leak the stream to keep it alive - it will be "stopped" by the is_recording flag
        std::mem::forget(stream);
        Ok(())
    }

    pub fn stop_recording(&mut self) -> Result<Vec<f32>> {
        self.is_recording.store(false, Ordering::SeqCst);
        let samples = self.buffer.lock().unwrap().clone();
        Ok(samples)
    }

    pub fn is_recording(&self) -> bool {
        self.is_recording.load(Ordering::SeqCst)
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn get_level(&self) -> f32 {
        let buffer = self.buffer.lock().unwrap();
        if buffer.is_empty() {
            return 0.0;
        }
        let recent = &buffer[buffer.len().saturating_sub(1024)..];
        let rms = (recent.iter().map(|s| s * s).sum::<f32>() / recent.len() as f32).sqrt();
        (rms * 10.0).min(1.0)
    }
}
