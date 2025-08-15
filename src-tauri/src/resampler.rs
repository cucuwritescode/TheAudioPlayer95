use anyhow::Result;
use rubato::{
    FftFixedInOut, Resampler as RubatoResampler,
};

pub struct AudioResampler {
    resampler: Option<FftFixedInOut<f32>>,
    input_rate: u32,
    output_rate: u32,
    channels: usize,
    chunk_size: usize,
}

impl AudioResampler {
    pub fn new(input_rate: u32, output_rate: u32, channels: usize) -> Result<Self> {
        // If input and output rates are the same, no resampling needed
        if input_rate == output_rate {
            return Ok(AudioResampler {
                resampler: None,
                input_rate,
                output_rate,
                channels,
                chunk_size: 0,
            });
        }

        // Calculate chunk size for resampler
        let chunk_size = 1024; // This can be adjusted for performance

        // Create the resampler
        let resampler = FftFixedInOut::<f32>::new(
            input_rate as usize,
            output_rate as usize,
            chunk_size,
            channels,
        )?;

        Ok(AudioResampler {
            resampler: Some(resampler),
            input_rate,
            output_rate,
            channels,
            chunk_size,
        })
    }

    /// Process audio samples, resampling if necessary
    pub fn process(&mut self, input: &[f32]) -> Result<Vec<f32>> {
        // If no resampling needed, return input as-is
        if self.resampler.is_none() {
            return Ok(input.to_vec());
        }

        let resampler = self.resampler.as_mut().unwrap();

        // Convert interleaved samples to channel vectors
        let mut channel_data: Vec<Vec<f32>> = vec![Vec::new(); self.channels];
        
        for (i, sample) in input.iter().enumerate() {
            let channel = i % self.channels;
            channel_data[channel].push(*sample);
        }

        // Ensure we have the right chunk size
        let frames = channel_data[0].len();
        if frames != self.chunk_size {
            // For now, we'll pad or truncate to match chunk size
            // In production, you'd want to buffer partial chunks
            for channel in &mut channel_data {
                channel.resize(self.chunk_size, 0.0);
            }
        }

        // Create input and output buffers for resampler
        let input_refs: Vec<&[f32]> = channel_data.iter().map(|v| v.as_slice()).collect();
        let mut output_data = vec![vec![0.0f32; resampler.output_frames_max()]; self.channels];
        let mut output_refs: Vec<&mut [f32]> = output_data.iter_mut().map(|v| v.as_mut_slice()).collect();

        // Process through resampler
        let (_, output_frames) = resampler.process_into_buffer(&input_refs, &mut output_refs, None)?;

        // Convert back to interleaved format
        let mut output = Vec::with_capacity(output_frames * self.channels);
        for frame in 0..output_frames {
            for channel in 0..self.channels {
                output.push(output_data[channel][frame]);
            }
        }

        Ok(output)
    }

    pub fn reset(&mut self) {
        if let Some(ref mut resampler) = self.resampler {
            resampler.reset();
        }
    }

    /// Check if resampling is needed
    pub fn needs_resampling(&self) -> bool {
        self.resampler.is_some()
    }

    /// Get the output sample rate
    pub fn output_rate(&self) -> u32 {
        self.output_rate
    }
}