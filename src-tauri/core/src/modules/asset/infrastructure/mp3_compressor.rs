use crate::modules::asset::domain::errors::optimization_error::OptimizationError;
use crate::modules::asset::domain::ports::audio_compressor::AudioCompressor;
use shine_rs::{Mp3Encoder, Mp3EncoderConfig, StereoMode};
use std::io::Cursor;
use symphonia::core::audio::AudioBuffer;
use symphonia::core::audio::Signal;
use symphonia::core::io::MediaSourceStream;

pub struct Mp3Compressor;

impl Default for Mp3Compressor {
    fn default() -> Self {
        Self::new()
    }
}

impl Mp3Compressor {
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

    fn decode_mp3(&self, input: &[u8]) -> Result<(Vec<f32>, u32, u16), OptimizationError> {
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

    fn encode_mp3(
        &self,
        samples: &[f32],
        rate: u32,
        channels: u16,
    ) -> Result<Vec<u8>, OptimizationError> {
        let stereo_mode = match channels {
            1 => StereoMode::Mono,
            _ => StereoMode::Stereo,
        };
        let bitrate = match channels {
            1 => 64,
            _ => 128,
        };
        let config = Mp3EncoderConfig::new()
            .sample_rate(rate)
            .channels(channels as u8)
            .stereo_mode(stereo_mode)
            .bitrate(bitrate);

        let mut encoder = Mp3Encoder::new(config)
            .map_err(|e| OptimizationError::CompressionError(e.to_string()))?;

        let pcm_i16: Vec<i16> = samples
            .iter()
            .map(|&s| (s * 32767.0).clamp(-32768.0, 32767.0) as i16)
            .collect();

        let chunk_size = encoder.samples_per_frame();
        let mut mp3 = Vec::new();
        for chunk in pcm_i16.chunks(chunk_size) {
            let mut input_chunk = chunk.to_vec();
            if input_chunk.len() < chunk_size {
                input_chunk.resize(chunk_size, 0);
            }
            if let Ok(encoded) = encoder.encode_interleaved(&input_chunk) {
                for frame in encoded {
                    mp3.extend_from_slice(&frame);
                }
            }
        }
        if let Ok(final_data) = encoder.finish() {
            mp3.extend_from_slice(&final_data);
        }
        Ok(mp3)
    }
}

impl AudioCompressor for Mp3Compressor {
    fn compress(
        &self,
        input: &[u8],
        target_sample_rate: u32,
    ) -> Result<Vec<u8>, OptimizationError> {
        self.validate_rate(target_sample_rate)?;
        let (pcm, rate, channels) = self.decode_mp3(input)?;
        let resampled = self.resample(&pcm, rate, target_sample_rate, channels);
        self.encode_mp3(&resampled, target_sample_rate, channels)
    }
}
