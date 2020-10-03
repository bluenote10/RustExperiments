use criterion::{criterion_group, criterion_main, BatchSize, Criterion, BenchmarkId};
use std::time::Duration;

use rustfft::FFTplanner;
use rustfft::FFTnum;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;

// ----------------------------------------------------------------------------
// rustfft
// ----------------------------------------------------------------------------

pub fn real_to_complex(data: &[f32], window: &[f32]) -> Vec<Complex<f32>> {
    assert_eq!(data.len(), window.len());
    (0 .. data.len()).map(|i| Complex::new(window[i] * data[i], 0.0)).collect()
}

pub fn fft_from_complex_with_output<T>(input: &mut [Complex<T>], output: &mut Vec<Complex<T>>)
where
    T: FFTnum
{
    let mut planner = FFTplanner::new(false);
    let fft = planner.plan_fft(input.len());
    fft.process(input, output);
}

pub fn fft_from_complex<T>(input: &mut [Complex<T>]) -> Vec<Complex<T>>
where
    T: FFTnum
{
    let mut output: Vec<Complex<T>> = vec![Complex::zero(); input.len()];
    fft_from_complex_with_output(input, &mut output);
    output
}

pub fn fft(data: &[f32], window: &[f32]) -> Vec<Complex<f32>> {
    let mut input = real_to_complex(data, window);
    let output = fft_from_complex(&mut input);
    output
}

// ----------------------------------------------------------------------------
// helpers
// ----------------------------------------------------------------------------

pub fn init_data(len: usize) -> Vec<f32> {
    let mut result = Vec::with_capacity(len);
    for i in 0..len {
        result.push(i as f32);
    }
    result
}

pub fn init_window(len: usize) -> Vec<f32> {
    vec![1.0; len]
}

// ----------------------------------------------------------------------------
// benchmarks
// ----------------------------------------------------------------------------

#[rustfmt::skip]
fn benchmarks(c: &mut Criterion) {
    let mut g = c.benchmark_group("fftrust");

    let sizes = [128, 512, 1024, 4096];

    for size in sizes.iter() {
        let size = *size;

        g.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| b.iter_batched(
            || (init_data(size), init_window(size)),
            |(data, window)| {
                fft(&data, &window)
            },
            BatchSize::SmallInput,
        ));
    }
}

fn config() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs_f64(3.0))
        .warm_up_time(Duration::from_secs_f64(0.1))
}

criterion_group! {
    name = benches;
    config = config();
    targets = benchmarks
}
criterion_main!(benches);
