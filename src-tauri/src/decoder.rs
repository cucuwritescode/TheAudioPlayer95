use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::path::Path;
use symphonia::core::audio::{AudioBufferRef, SampleBuffer, SignalSpec};
use symphonia::core::codecs::{Decoder, DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

pub struct AudioDecoder {
    format: Box<dyn FormatReader>,
    decoder: Box<dyn Decoder>,
    track_id: u32,
    spec: SignalSpec,
    sample_buf: Option<SampleBuffer<f32>>,
}

impl AudioDecoder {
    pub fn new(file_path: &Path) -> Result<Self> {
        // Open the file
        let file = File::open(file_path)
            .with_context(|| format!("Failed to open file: {:?}", file_path))?;

        // Create a media source stream
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        // Create a hint based on the file extension
        let mut hint = Hint::new();
        if let Some(ext) = file_path.extension() {
            if let Some(ext_str) = ext.to_str() {
                hint.with_extension(ext_str);
            }
        }

        // Probe the media source
        let meta_opts: MetadataOptions = Default::default();
        let fmt_opts: FormatOptions = Default::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .context("Failed to probe audio format")?;

        let format = probed.format;

        // Find the first audio track
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .ok_or_else(|| anyhow::anyhow!("No audio tracks found"))?;

        let track_id = track.id;

        // Create a decoder for the track
        let dec_opts: DecoderOptions = Default::default();
        let decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .context("Failed to create decoder")?;

        // Get the audio specification
        let spec = *decoder
            .codec_params()
            .audio()
            .ok_or_else(|| anyhow::anyhow!("No audio specification available"))?;

        Ok(AudioDecoder {
            format,
            decoder,
            track_id,
            spec,
            sample_buf: None,
        })
    }

    pub fn spec(&self) -> &SignalSpec {
        &self.spec
    }

    pub fn sample_rate(&self) -> u32 {
        self.spec.rate
    }

    pub fn channels(&self) -> usize {
        self.spec.channels.count()
    }

    /// Decode the next packet and return samples as f32
    pub fn decode_next(&mut self) -> Result<Option<Vec<f32>>> {
        // Get the next packet from the format reader
        let packet = match self.format.next_packet() {
            Ok(packet) => packet,
            Err(Error::IoError(err)) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                return Ok(None);
            }
            Err(err) => return Err(err.into()),
        };

        // Skip packets from other tracks
        if packet.track_id() != self.track_id {
            return self.decode_next();
        }

        // Decode the packet
        let decoded = self.decoder.decode(&packet)?;

        // Convert to f32 samples
        let samples = self.convert_to_f32(&decoded)?;
        Ok(Some(samples))
    }

    fn convert_to_f32(&mut self, audio_buf: &AudioBufferRef) -> Result<Vec<f32>> {
        // Get the capacity for the sample buffer
        let spec = *audio_buf.spec();
        let duration = audio_buf.capacity() as u64;

        // Create or resize the sample buffer
        if self.sample_buf.is_none() || self.sample_buf.as_ref().unwrap().capacity() < audio_buf.capacity() {
            self.sample_buf = Some(SampleBuffer::<f32>::new(duration, spec));
        }

        // Copy the audio buffer to the sample buffer
        if let Some(ref mut sample_buf) = self.sample_buf {
            sample_buf.copy_interleaved_ref(audio_buf);
            Ok(sample_buf.samples().to_vec())
        } else {
            Err(anyhow::anyhow!("Failed to create sample buffer"))
        }
    }

    /// Seek to a specific position in seconds
    pub fn seek(&mut self, seconds: f64) -> Result<()> {
        // Seek to the target timestamp
        let _seeked_to = self.format.seek(
            symphonia::core::formats::SeekMode::Accurate,
            symphonia::core::formats::SeekTo::Time {
                time: symphonia::core::units::Time {
                    seconds: seconds as u64,
                    frac: seconds.fract(),
                },
                track_id: Some(self.track_id),
            },
        )?;

        // Reset the decoder after seeking
        self.decoder.reset();

        Ok(())
    }

    /// Get metadata about the track
    pub fn metadata(&mut self) -> TrackMetadata {
        let mut metadata = TrackMetadata::default();

        if let Some(meta) = self.format.metadata().current() {
            for tag in meta.tags() {
                match tag.std_key {
                    Some(symphonia::core::meta::StandardTagKey::TrackTitle) => {
                        metadata.title = tag.value.to_string();
                    }
                    Some(symphonia::core::meta::StandardTagKey::Artist) => {
                        metadata.artist = tag.value.to_string();
                    }
                    Some(symphonia::core::meta::StandardTagKey::Album) => {
                        metadata.album = tag.value.to_string();
                    }
                    Some(symphonia::core::meta::StandardTagKey::Genre) => {
                        metadata.genre = tag.value.to_string();
                    }
                    _ => {}
                }
            }
        }

        metadata
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TrackMetadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub genre: String,
}