/*
SEAMLESS LOOP
For more information see README.md.
---
License:
do what the fuck you want to
*/

use clap::Parser;
use std::fs::File;
use std::fmt::{Debug, Display};
use anyhow::{Result, Context};

/// Turn a .wav file into a seamless loop.
#[derive(Parser, Debug)]
struct Args {
	/// Input file
	file: String,
	
	/// Output file. Defaults to "x-seamless.wav" for input file "x.wav".
	#[arg(short)]
	output: Option<String>,
	
	/// Duration in seconds of the fade.
	#[arg(short, default_value_t = 0.03)]
	duration: f32,
}

trait AudioSample: Copy + Debug + Display {
	fn interpolate(self, other: Self, t: f32) -> Self;
}

macro_rules! impl_audio_sample {
	($type:ty, $min:expr, $max:expr) => {
		impl AudioSample for $type {
			fn interpolate(self, other: Self, t: f32) -> Self {
				let a = self as f32;
				let b = other as f32;
				(a * (1.0 - t) + b * t).clamp($min, $max) as Self
			}
		}
	}
}

impl_audio_sample!(u8, 0.0, 255.0);
impl_audio_sample!(i16, -32767.0, 32767.0);
// NOTE: twenty-two bit samples are shifted left by 8 by the wav crate, so this is correct for them
impl_audio_sample!(i32, -i32::MAX as f32, i32::MAX as f32);
impl_audio_sample!(f32, -1.0, 1.0);

fn make_seamless<T: AudioSample>(data: &mut Vec<T>, channels: u16, fade_samples: usize) -> Result<()> {
	let channels: usize = channels.into();
	let audio_samples = data.len();
	if fade_samples * 2 >= audio_samples {
		return Err(anyhow::anyhow!("Fade duration is too long (must be less than half of audio file's duration)."));
	}
	if audio_samples % channels != 0 {
		return Err(anyhow::anyhow!("Sample count is not multiple of channel count (this shouldn't happen)."));
	}
	let fade_frames = fade_samples / channels;
	let audio_frames = audio_samples / channels;
	for i in 0..fade_frames {
		let t = i as f32 / (fade_frames as f32);
		let j = audio_frames - fade_frames + i;
		for c in 0..channels {
			data[channels * i + c] = data[channels * j + c].interpolate(data[channels * i + c], t);
		}
	}
	data.truncate((audio_frames - fade_frames) * channels);
	Ok(())
}

fn main() -> Result<()> {
	let args = Args::parse();
	let input = &args.file;
	let output = args.output.unwrap_or_else(|| {
		let name = input.strip_suffix(".wav").unwrap_or(input);
		name.to_string() + "-seamless.wav"
	});
	let mut input_file = File::open(input).with_context(|| format!("Couldn't open input file {input}"))?;
	let (header, mut data) = wav::read(&mut input_file)?;
	drop(input_file);
	
	let samples = header.sampling_rate as f32 * args.duration;
	if !samples.is_finite() || samples < 0.0 || samples > usize::MAX as f32 {
		return Err(anyhow::anyhow!("Bad duration"));
	}
	let samples = samples as usize;
	let channels = header.channel_count;
	use wav::BitDepth;
	match &mut data {
		BitDepth::Eight(data) => make_seamless(data, channels, samples)?,
		BitDepth::Sixteen(data) => make_seamless(data, channels, samples)?,
		BitDepth::TwentyFour(data) => make_seamless(data, channels, samples)?,
		BitDepth::ThirtyTwoFloat(data) => make_seamless(data, channels, samples)?,
		BitDepth::Empty => return Err(anyhow::anyhow!("No audio data")),
	}
	
	let mut output_file = File::create(&output).with_context(|| format!("Couldn't open output file {output}"))?;
	wav::write(header, &data, &mut output_file)?;
	
	Ok(())
}
