use std::mem::MaybeUninit;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{BuildStreamError, Device, Sample, SampleFormat, Stream, SupportedStreamConfig};
use ringbuf::{HeapRb, Producer, SharedRb};

fn _basic_demo() {
    println!("Hello, world!");

    fn write_silence<T: Sample>(data: &mut [T], _: &cpal::OutputCallbackInfo) {
        for sample in data.iter_mut() {
            *sample = Sample::EQUILIBRIUM;
        }
    }

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

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let sample_format = supported_config.sample_format();
    let config = supported_config.into();
    let stream = match sample_format {
        SampleFormat::F32 => {
            device.build_output_stream(&config, write_silence::<f32>, err_fn, None)
        }
        SampleFormat::I16 => {
            device.build_output_stream(&config, write_silence::<i16>, err_fn, None)
        }
        SampleFormat::U16 => {
            device.build_output_stream(&config, write_silence::<u16>, err_fn, None)
        }
        SampleFormat::U8 => device.build_output_stream(&config, write_silence::<u8>, err_fn, None),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
    .unwrap();

    stream.play().unwrap();

    sleep(Duration::from_secs_f32(3.0));
}

fn threaded_demo() {
    println!("Hello, world!");

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

    let (stream, mut producer) = device.create_audio_stream_f32(supported_config).unwrap();

    stream.play().unwrap();

    let mut num_samples = 0;

    loop {
        let to_fill = producer.free_len();
        if to_fill > 0 {
            println!("Pushing {} samples", to_fill);
            let frames: Vec<_> = (0..to_fill)
                .map(|_| {
                    let value = 0.1
                        * (2.0 * std::f32::consts::PI * 440.0 * num_samples as f32 / 44100.0).sin();
                    num_samples += 1;
                    value
                })
                .collect();
            producer.push_slice(&frames);
        }
    }
}

fn main() {
    threaded_demo()
}

///
/// Extensions to `cpal::Device`
/// Inspired by:
/// https://github.com/RustAudio/rodio/blob/eda5934a209056c39cea3a4734394d5680a2a8a3/src/stream.rs#L192
///
pub(crate) trait CpalDeviceExt {
    fn create_audio_stream_f32(
        &self,
        format: SupportedStreamConfig,
    ) -> Result<(Stream, AudioProducer), BuildStreamError>;
}

type AudioProducer = Producer<f32, Arc<SharedRb<f32, Vec<MaybeUninit<f32>>>>>;

impl CpalDeviceExt for Device {
    fn create_audio_stream_f32(
        &self,
        format: SupportedStreamConfig,
    ) -> Result<(Stream, AudioProducer), BuildStreamError> {
        // Create the ring buffer and split it.
        let ring_buffer = HeapRb::<f32>::new(1024);
        let (producer, mut consumer) = ring_buffer.split();

        let error_callback = |err| eprintln!("an error occurred on output stream: {}", err);

        let stream = match format.sample_format() {
            SampleFormat::F32 => self.build_output_stream::<f32, _, _>(
                &format.config(),
                move |data, _output_callback_info| {
                    data.iter_mut()
                        .for_each(|d| *d = consumer.pop().unwrap_or(0f32))
                },
                error_callback,
                None,
            ),
            SampleFormat::U8 => self.build_output_stream::<u8, _, _>(
                &format.config(),
                move |data, _output_callback_info| {
                    println!("Pulling {} samples", data.len());
                    data.iter_mut().for_each(|d| {
                        *d = consumer
                            .pop()
                            .map(Sample::from_sample)
                            .unwrap_or(u8::max_value() / 2)
                    })
                },
                error_callback,
                None,
            ),
            _ => return Err(BuildStreamError::StreamConfigNotSupported),
        }?;

        Ok((stream, producer))
    }
}
