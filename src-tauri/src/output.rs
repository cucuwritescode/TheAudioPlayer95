use anyhow::{Context, Result};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Stream, StreamConfig};
use ringbuf::{HeapConsumer, HeapProducer, HeapRb};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;

pub struct AudioOutput {
    device: Device,
    config: StreamConfig,
    producer: HeapProducer<f32>,
    volume: Arc<AtomicU32>,
    is_playing: Arc<AtomicBool>,
    _stream: Box<Stream>, // Keep stream alive but not accessible
}

impl AudioOutput {
    pub fn new() -> Result<Self> {
        // Get the default output device
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .context("No output device available")?;

        // Get the default output config
        let config = device
            .default_output_config()
            .context("Failed to get default output config")?;

        let config = config.into();

        // Create a ring buffer for audio data
        // Buffer size is 2 seconds worth of audio
        let buffer_size = (config.sample_rate.0 * config.channels as u32 * 2) as usize;
        let rb = HeapRb::<f32>::new(buffer_size);
        let (producer, consumer) = rb.split();

        let volume = Arc::new(AtomicU32::new(100)); // 100% volume by default
        let is_playing = Arc::new(AtomicBool::new(false));

        let stream = Self::create_stream(&device, &config, consumer, Arc::clone(&volume), Arc::clone(&is_playing))?;

        Ok(AudioOutput {
            device,
            config,
            producer,
            volume,
            is_playing,
            _stream: Box::new(stream),
        })
    }

    fn create_stream(
        device: &Device, 
        config: &StreamConfig, 
        mut consumer: HeapConsumer<f32>, 
        volume: Arc<AtomicU32>, 
        is_playing: Arc<AtomicBool>
    ) -> Result<Stream> {
        let channels = config.channels as usize;

        let stream = device.build_output_stream(
            config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // If not playing, fill with silence
                if !is_playing.load(Ordering::Relaxed) {
                    for sample in data.iter_mut() {
                        *sample = 0.0;
                    }
                    return;
                }

                // Get the current volume (0-100)
                let vol = volume.load(Ordering::Relaxed) as f32 / 100.0;

                // Read samples from the ring buffer
                let frames_needed = data.len() / channels;

                // Try to read the exact amount we need
                let read_count = consumer.pop_slice(data);

                // Apply volume
                for sample in &mut data[..read_count] {
                    *sample *= vol;
                }

                // Fill any remaining space with silence
                for sample in &mut data[read_count..] {
                    *sample = 0.0;
                }
            },
            move |err| {
                eprintln!("Audio stream error: {}", err);
            },
            None,
        )?;

        stream.play()?;
        Ok(stream)
    }

    /// Write audio samples to the output buffer
    pub fn write(&mut self, samples: &[f32]) -> Result<usize> {
        // Write as many samples as possible to the ring buffer
        let written = self.producer.push_slice(samples);
        Ok(written)
    }

    /// Check how much space is available in the buffer
    pub fn buffer_space(&self) -> usize {
        self.producer.free_len()
    }

    /// Start playing audio
    pub fn play(&self) {
        self.is_playing.store(true, Ordering::Relaxed);
    }

    /// Pause audio playback
    pub fn pause(&self) {
        self.is_playing.store(false, Ordering::Relaxed);
    }

    /// Stop audio playback and clear buffer
    pub fn stop(&mut self) {
        self.is_playing.store(false, Ordering::Relaxed);
        // Note: We don't clear the ring buffer here to avoid threading issues
    }

    /// Set volume (0-100)
    pub fn set_volume(&self, volume: u8) {
        let vol = volume.min(100);
        self.volume.store(vol as u32, Ordering::Relaxed);
    }

    /// Get current volume (0-100)
    pub fn get_volume(&self) -> u8 {
        self.volume.load(Ordering::Relaxed) as u8
    }

    /// Check if currently playing
    pub fn is_playing(&self) -> bool {
        self.is_playing.load(Ordering::Relaxed)
    }

    /// Get the sample rate
    pub fn sample_rate(&self) -> u32 {
        self.config.sample_rate.0
    }

    /// Get the number of channels
    pub fn channels(&self) -> u16 {
        self.config.channels
    }
}