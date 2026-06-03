use anyhow::{anyhow, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, SampleFormat, Stream, StreamConfig};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex as StdMutex;

struct StreamHolder {
    stream: Option<Box<dyn StreamTrait>>,
}

unsafe impl Send for StreamHolder {}
unsafe impl Sync for StreamHolder {}

pub struct AudioCapture {
    host: Host,
    device_name: Option<String>,
    buffer: Arc<StdMutex<Vec<f32>>>,
    is_recording: Arc<AtomicBool>,
    sample_rate: u32,
    stream_holder: Arc<StdMutex<StreamHolder>>,
}

unsafe impl Send for AudioCapture {}
unsafe impl Sync for AudioCapture {}

trait StreamTraitExt: StreamTrait {}
impl StreamTraitExt for Stream {}

impl AudioCapture {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();

        Ok(Self {
            host,
            device_name: None,
            buffer: Arc::new(StdMutex::new(Vec::new())),
            is_recording: Arc::new(AtomicBool::new(false)),
            sample_rate: 16000,
            stream_holder: Arc::new(StdMutex::new(StreamHolder { stream: None })),
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
        // Stop any existing stream first
        {
            let mut holder = self.stream_holder.lock().unwrap();
            holder.stream = None;
        }

        let device = self.get_device()?;
        let supported_config = device.default_input_config()?;
        self.sample_rate = supported_config.sample_rate().0;

        let config = StreamConfig {
            channels: 1,
            sample_rate: supported_config.sample_rate(),
            buffer_size: cpal::BufferSize::Default,
        };

        // Clear buffer
        self.buffer.lock().unwrap().clear();
        self.is_recording.store(true, Ordering::SeqCst);

        let buffer = self.buffer.clone();
        let is_recording = self.is_recording.clone();

        let stream = match supported_config.sample_format() {
            SampleFormat::F32 => device.build_input_stream(
                &config,
                move |data: &[f32], _| {
                    if is_recording.load(Ordering::Relaxed) {
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
                        if is_recording.load(Ordering::Relaxed) {
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
            SampleFormat::U8 => {
                let buffer = self.buffer.clone();
                let is_recording = self.is_recording.clone();
                device.build_input_stream(
                    &config,
                    move |data: &[u8], _| {
                        if is_recording.load(Ordering::Relaxed) {
                            let floats: Vec<f32> =
                                data.iter().map(|&s| (s as f32 - 128.0) / 128.0).collect();
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

        // Store stream to keep it alive
        let boxed: Box<dyn StreamTrait> = Box::new(stream);
        {
            let mut holder = self.stream_holder.lock().unwrap();
            holder.stream = Some(unsafe { std::mem::transmute(boxed) });
        }

        log::info!("Recording started, sample_rate={}", self.sample_rate);
        Ok(())
    }

    pub fn stop_recording(&mut self) -> Result<Vec<f32>> {
        self.is_recording.store(false, Ordering::SeqCst);

        // Drop the stream to stop recording
        {
            let mut holder = self.stream_holder.lock().unwrap();
            holder.stream = None;
        }

        let samples = self.buffer.lock().unwrap().clone();
        log::info!("Recording stopped, {} samples captured", samples.len());
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
        let recent = &buffer[buffer.len().saturating_sub(2048)..];
        let rms = (recent.iter().map(|s| s * s).sum::<f32>() / recent.len() as f32).sqrt();
        (rms * 5.0).min(1.0)
    }
}
