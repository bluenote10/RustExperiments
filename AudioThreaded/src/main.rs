use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::sync::Arc;

use color_eyre::eyre::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BuildStreamError, Device, Sample, SampleFormat, Stream, SupportedStreamConfig};
use ringbuf::{Consumer, HeapRb, Producer, SharedRb};

fn main() -> Result<()> {
    color_eyre::install()?;
    threaded_demo()
}

fn threaded_demo() -> Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("no output device available");

    let mut supported_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let (stream, mut producer) = device.create_audio_stream_with_buffer(supported_config)?;

    stream.play()?;

    let mut num_frames = 0;

    loop {
        let to_fill = producer.free_len();
        if to_fill > 0 {
            println!("Pushing {} frames [{num_frames}]", to_fill);
            let frames: Vec<_> = (0..to_fill)
                .map(|_| {
                    let value = 0.1
                        * (2.0 * std::f32::consts::PI * 440.0 * num_frames as f32 / 44100.0).sin();
                    num_frames += 1;
                    AudioFrame::new(value, value)
                })
                .collect();
            producer.push_slice(&frames);
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct AudioFrame {
    pub left: f32,
    pub right: f32,
}

impl AudioFrame {
    pub fn new(left: f32, right: f32) -> Self {
        Self { left, right }
    }
}

///
/// Extensions to `cpal::Device`
/// Inspired by:
/// https://github.com/RustAudio/rodio/blob/eda5934a209056c39cea3a4734394d5680a2a8a3/src/stream.rs#L192
///
pub(crate) trait CpalDeviceExt {
    fn create_audio_stream_with_buffer(
        &self,
        format: SupportedStreamConfig,
    ) -> Result<(Stream, AudioProducer), BuildStreamError>;
}

type AudioProducer = Producer<AudioFrame, Arc<SharedRb<AudioFrame, Vec<MaybeUninit<AudioFrame>>>>>;
type AudioConsumer = Consumer<AudioFrame, Arc<SharedRb<AudioFrame, Vec<MaybeUninit<AudioFrame>>>>>;

impl CpalDeviceExt for Device {
    fn create_audio_stream_with_buffer(
        &self,
        format: SupportedStreamConfig,
    ) -> Result<(Stream, AudioProducer), BuildStreamError> {
        // Create the ring buffer and split it.
        let ring_buffer = HeapRb::<AudioFrame>::new(1024);
        let (producer, consumer) = ring_buffer.split();

        let error_callback = |err| eprintln!("an error occurred on output stream: {}", err);

        let mut ctx = Context {
            consumer,
            overflow: None,
        };

        let stream = match format.sample_format() {
            SampleFormat::F32 => self.build_output_stream::<f32, _, _>(
                &format.config(),
                move |buffer, _output_callback_info| process_audio(&mut ctx, buffer),
                error_callback,
                None,
            ),
            SampleFormat::U8 => self.build_output_stream::<u8, _, _>(
                &format.config(),
                move |buffer, _output_callback_info| process_audio(&mut ctx, buffer),
                error_callback,
                None,
            ),
            _ => return Err(BuildStreamError::StreamConfigNotSupported),
        }?;

        Ok((stream, producer))
    }
}

struct Context {
    consumer: AudioConsumer,
    overflow: Option<f32>,
}

fn process_audio<S>(ctx: &mut Context, buffer: &mut [S])
where
    S: Sample + cpal::FromSample<f32> + Debug,
{
    println!("Pulling {} samples", buffer.len());

    let mut iter = buffer.iter_mut();

    if let Some(overflow) = ctx.overflow {
        if let Some(first) = iter.next() {
            *first = Sample::from_sample(overflow);
            ctx.overflow = None;
        }
    }

    let mut cur_frame: Option<AudioFrame> = None;
    for s in iter {
        let value = if let Some(frame) = cur_frame {
            cur_frame = None;
            frame.right
        } else {
            let frame = ctx.consumer.pop();
            if let Some(frame) = frame {
                cur_frame = Some(frame);
                frame.left
            } else {
                Sample::EQUILIBRIUM
            }
        };
        *s = Sample::from_sample(value);
    }
    if let Some(frame) = cur_frame {
        ctx.overflow = Some(frame.right);
    }
}
