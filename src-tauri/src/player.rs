use crate::decoder::{AudioDecoder, TrackMetadata};
use crate::output::AudioOutput;
use crate::resampler::AudioResampler;
use anyhow::{Context, Result};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub enum PlayerCommand {
    Play,
    Pause,
    Stop,
    Seek(f64), // Seek to position in seconds
    SetVolume(u8),
    LoadTrack(PathBuf),
}

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub is_playing: bool,
    pub current_track: Option<PathBuf>,
    pub position: f64, // Current position in seconds
    pub duration: f64, // Total duration in seconds
    pub volume: u8,
    pub metadata: Option<TrackMetadata>,
}

pub struct AudioPlayer {
    output: Arc<Mutex<AudioOutput>>,
    command_tx: mpsc::UnboundedSender<PlayerCommand>,
    state: Arc<Mutex<PlayerState>>,
    decode_thread: Option<thread::JoinHandle<()>>,
    should_stop: Arc<AtomicBool>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let output = Arc::new(Mutex::new(AudioOutput::new()?));
        let (command_tx, _command_rx) = mpsc::unbounded_channel();
        let state = Arc::new(Mutex::new(PlayerState {
            is_playing: false,
            current_track: None,
            position: 0.0,
            duration: 0.0,
            volume: 100,
            metadata: None,
        }));

        let should_stop = Arc::new(AtomicBool::new(false));

        Ok(AudioPlayer {
            output,
            command_tx,
            state,
            decode_thread: None,
            should_stop,
        })
    }

    /// Load and start playing a track
    pub async fn load_track(&mut self, file_path: PathBuf) -> Result<()> {
        // Stop current playback if any
        self.stop_playback();

        // Create decoder for the new track
        let mut decoder = AudioDecoder::new(&file_path)
            .with_context(|| format!("Failed to decode file: {:?}", file_path))?;

        // Get metadata
        let metadata = decoder.metadata();

        // Update state
        {
            let mut state = self.state.lock().unwrap();
            state.current_track = Some(file_path.clone());
            state.metadata = Some(metadata);
            state.position = 0.0;
            // TODO: Calculate actual duration
            state.duration = 0.0;
        }

        // Start decode thread
        self.start_decode_thread(decoder)?;

        Ok(())
    }

    fn start_decode_thread(&mut self, mut decoder: AudioDecoder) -> Result<()> {
        let output = Arc::clone(&self.output);
        let state = Arc::clone(&self.state);
        let should_stop = Arc::clone(&self.should_stop);
        should_stop.store(false, Ordering::Relaxed);

        // Get output configuration
        let output_rate = output.lock().unwrap().sample_rate();
        let output_channels = output.lock().unwrap().channels() as usize;

        // Create resampler if needed
        let input_rate = decoder.sample_rate();
        let input_channels = decoder.channels();
        
        let mut resampler = if input_rate != output_rate || input_channels != output_channels {
            Some(AudioResampler::new(input_rate, output_rate, input_channels)?)
        } else {
            None
        };

        let thread_handle = thread::spawn(move || {
            let mut samples_played = 0u64;

            while !should_stop.load(Ordering::Relaxed) {
                // Check if we should be playing
                let is_playing = {
                    let state = state.lock().unwrap();
                    state.is_playing
                };

                if !is_playing {
                    thread::sleep(std::time::Duration::from_millis(10));
                    continue;
                }

                // Check buffer space
                let space = output.lock().unwrap().buffer_space();
                if space < 4096 {
                    // Buffer is full, wait a bit
                    thread::sleep(std::time::Duration::from_millis(10));
                    continue;
                }

                // Decode next packet
                match decoder.decode_next() {
                    Ok(Some(mut samples)) => {
                        // Resample if needed
                        if let Some(ref mut resampler) = resampler {
                            match resampler.process(&samples) {
                                Ok(resampled) => samples = resampled,
                                Err(e) => {
                                    eprintln!("Resampling error: {}", e);
                                    continue;
                                }
                            }
                        }

                        // Write to output
                        if let Ok(mut output_guard) = output.lock() {
                            match output_guard.write(&samples) {
                                Ok(written) => {
                                    samples_played += written as u64;
                                    
                                    // Update position
                                    let position = samples_played as f64 / output_rate as f64 / output_channels as f64;
                                    if let Ok(mut state) = state.lock() {
                                        state.position = position;
                                    }
                                }
                                Err(e) => eprintln!("Failed to write audio: {}", e),
                            }
                        }
                    }
                    Ok(None) => {
                        // End of file
                        println!("End of track");
                        break;
                    }
                    Err(e) => {
                        eprintln!("Decode error: {}", e);
                        break;
                    }
                }
            }

            // Stop playback
            if let Ok(mut output_guard) = output.lock() {
                output_guard.stop();
            }

            // Update state
            if let Ok(mut state_guard) = state.lock() {
                state_guard.is_playing = false;
            }
        });

        self.decode_thread = Some(thread_handle);
        Ok(())
    }

    fn stop_playback(&mut self) {
        // Signal thread to stop
        self.should_stop.store(true, Ordering::Relaxed);

        // Wait for thread to finish
        if let Some(thread) = self.decode_thread.take() {
            let _ = thread.join();
        }

        // Stop output
        if let Ok(mut output) = self.output.lock() {
            output.stop();
        }
    }

    /// Play the current track
    pub fn play(&self) -> Result<()> {
        if let Ok(mut state) = self.state.lock() {
            state.is_playing = true;
        }
        if let Ok(output) = self.output.lock() {
            output.play();
        }
        Ok(())
    }

    /// Pause playback
    pub fn pause(&self) -> Result<()> {
        if let Ok(mut state) = self.state.lock() {
            state.is_playing = false;
        }
        if let Ok(output) = self.output.lock() {
            output.pause();
        }
        Ok(())
    }

    /// Stop playback
    pub fn stop(&mut self) -> Result<()> {
        self.stop_playback();
        if let Ok(mut state) = self.state.lock() {
            state.is_playing = false;
            state.position = 0.0;
        }
        Ok(())
    }

    /// Set volume (0-100)
    pub fn set_volume(&self, volume: u8) -> Result<()> {
        if let Ok(output) = self.output.lock() {
            output.set_volume(volume);
        }
        if let Ok(mut state) = self.state.lock() {
            state.volume = volume;
        }
        Ok(())
    }

    /// Get current player state
    pub fn get_state(&self) -> PlayerState {
        self.state.lock().unwrap().clone()
    }

    /// Seek to position in seconds
    pub fn seek(&mut self, _position: f64) -> Result<()> {
        // TODO: Implement seeking by restarting decoder at position
        Ok(())
    }
}