use crate::modules::asset::domain::errors::optimization_error::OptimizationError;
use crate::modules::asset::domain::ports::audio_compressor::AudioCompressor;
use std::io::Cursor;
use symphonia::core::audio::AudioBuffer;
use symphonia::core::audio::Signal;
use symphonia::core::io::MediaSourceStream;

pub struct WavCompressor;

impl Default for WavCompressor {
    fn default() -> Self {
        Self::new()
    }
}

impl WavCompressor {
    pub const fn new() -> Self {
        Self
    }

    fn validate_rate(&self, rate: u32) -> Result<(), OptimizationError> {
        if rate != 11025 && rate != 22050 && rate != 44100 {
            return Err(OptimizationError::ValidationError(format!(
                "Unsupported rate: {}. Must be 11025, 22050 or 44100.",
                rate
            )));
        }
        Ok(())
    }

    fn decode_wav(&self, input: &[u8]) -> Result<(Vec<f32>, u32, u16), OptimizationError> {
        let cursor = Box::new(Cursor::new(input.to_vec()));
        let mss = MediaSourceStream::new(cursor, Default::default());
        let probed = symphonia::default::get_probe()
            .format(
                &Default::default(),
                mss,
                &Default::default(),
                &Default::default(),
            )
            .map_err(|e| OptimizationError::CompressionError(e.to_string()))?;

        let mut format = probed.format;
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != symphonia::core::codecs::CODEC_TYPE_NULL)
            .ok_or_else(|| OptimizationError::CompressionError("No audio track".to_string()))?;

        let rate = track
            .codec_params
            .sample_rate
            .ok_or_else(|| OptimizationError::CompressionError("No sample rate".to_string()))?;

        let channels = track
            .codec_params
            .channels
            .ok_or_else(|| OptimizationError::CompressionError("No channels".to_string()))?
            .count() as u16;

        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &Default::default())
            .map_err(|e| OptimizationError::CompressionError(e.to_string()))?;

        let mut pcm = Vec::new();
        while let Ok(packet) = format.next_packet() {
            if let Ok(decoded) = decoder.decode(&packet) {
                let spec = *decoded.spec();
                let mut buf = AudioBuffer::new(decoded.capacity() as u64, spec);
                decoded.convert(&mut buf);
                let frames = buf.frames();
                for f in 0..frames {
                    for c in 0..spec.channels.count() {
                        pcm.push(buf.chan(c)[f]);
                    }
                }
            }
        }

        Ok((pcm, rate, channels))
    }

    fn resample(&self, samples: &[f32], from: u32, to: u32, channels: u16) -> Vec<f32> {
        if from == to {
            return samples.to_vec();
        }
        let ratio = from as f64 / to as f64;
        let input_frames = samples.len() / channels as usize;
        let output_frames = (input_frames as f64 / ratio).round() as usize;
        let mut output = Vec::with_capacity(output_frames * channels as usize);
        for i in 0..output_frames {
            let pos = i as f64 * ratio;
            let idx = pos.floor() as usize;
            let next = (idx + 1).min(input_frames - 1);
            let w = (pos - idx as f64) as f32;
            for c in 0..channels as usize {
                let s1 = samples[idx * channels as usize + c];
                let s2 = samples[next * channels as usize + c];
                output.push(s1 * (1.0 - w) + s2 * w);
            }
        }
        output
    }

    fn write_header(&self, rate: u32, channels: u16, data_size: usize) -> Vec<u8> {
        let mut header = Vec::with_capacity(44);
        header.extend_from_slice(b"RIFF");
        header.extend_from_slice(&((36 + data_size) as u32).to_le_bytes());
        header.extend_from_slice(b"WAVEfmt ");
        header.extend_from_slice(&16u32.to_le_bytes());
        header.extend_from_slice(&1u16.to_le_bytes());
        header.extend_from_slice(&channels.to_le_bytes());
        header.extend_from_slice(&rate.to_le_bytes());
        header.extend_from_slice(&(rate * channels as u32 * 2).to_le_bytes());
        header.extend_from_slice(&(channels * 2).to_le_bytes());
        header.extend_from_slice(&16u16.to_le_bytes());
        header.extend_from_slice(b"data");
        header.extend_from_slice(&(data_size as u32).to_le_bytes());
        header
    }

    fn to_bytes(&self, samples: &[f32]) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(samples.len() * 2);
        for &s in samples {
            let scaled = (s * 32767.0).clamp(-32768.0, 32767.0) as i16;
            bytes.extend_from_slice(&scaled.to_le_bytes());
        }
        bytes
    }
}

impl AudioCompressor for WavCompressor {
    fn compress(
        &self,
        input: &[u8],
        target_sample_rate: u32,
    ) -> Result<Vec<u8>, OptimizationError> {
        self.validate_rate(target_sample_rate)?;
        let (pcm, rate, channels) = self.decode_wav(input)?;
        let resampled = self.resample(&pcm, rate, target_sample_rate, channels);
        let pcm_bytes = self.to_bytes(&resampled);
        let mut wav = self.write_header(target_sample_rate, channels, pcm_bytes.len());
        wav.extend_from_slice(&pcm_bytes);
        Ok(wav)
    }
}
