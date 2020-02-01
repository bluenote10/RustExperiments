extern crate rand;

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, BatchSize, Throughput};

use std::iter;
use rand::Rng;

#[inline]
fn noop(x: f64) -> f64 {
    x
}

#[inline]
fn abs1(x: f64) -> f64 {
    x.abs()
}

#[inline]
fn abs2(x: f64) -> f64 {
    if x > 0.0 { x } else { -x }
}

fn bench_old(c: &mut Criterion) {
    for x in [0.0, 1.0, -1.0, 123.456, -123.456].iter() {
        c.bench_with_input(BenchmarkId::new("input_example", x), x, |b, &x| {
            b.iter(|| abs2(black_box(x)));
        });
    }
}

fn bench_noop(c: &mut Criterion) {
    c.bench_function("noop", move |b| {
        b.iter_batched(|| 0.0, |x| noop(x), BatchSize::SmallInput)
    });

    let mut rng = rand::thread_rng();
    c.bench_function("noop (rand)", move |b| {
        b.iter_batched(|| rng.gen::<f64>(), |x| noop(x), BatchSize::SmallInput)
    });
}

fn bench_abs1(c: &mut Criterion) {
    c.bench_function("abs1 (0.0)", move |b| {
        b.iter_batched(|| 0.0, |x| abs1(x), BatchSize::SmallInput)
    });
    c.bench_function("abs1 (+1.0)", move |b| {
        b.iter_batched(|| 1.0, |x| abs1(x), BatchSize::SmallInput)
    });
    c.bench_function("abs1 (-1.0)", move |b| {
        b.iter_batched(|| -1.0, |x| abs1(x), BatchSize::SmallInput)
    });
    c.bench_function("abs1 (+12345678.90123456)", move |b| {
        b.iter_batched(|| 12345678.90123456, |x| abs1(x), BatchSize::SmallInput)
    });
    c.bench_function("abs1 (-12345678.90123456)", move |b| {
        b.iter_batched(|| -12345678.90123456, |x| abs1(x), BatchSize::SmallInput)
    });
    let mut rng = rand::thread_rng();
    c.bench_function("abs1 (rand)", move |b| {
        b.iter_batched(|| rng.gen::<f64>(), |x| abs1(x), BatchSize::SmallInput)
    });
}

fn bench_abs2(c: &mut Criterion) {
    c.bench_function("abs2 (0.0)", move |b| {
        b.iter_batched(|| 0.0, |x| abs2(black_box(x)), BatchSize::SmallInput)
    });
    c.bench_function("abs2 (+1.0)", move |b| {
        b.iter_batched(|| 1.0, |x| abs2(black_box(x)), BatchSize::SmallInput)
    });
    c.bench_function("abs2 (-1.0)", move |b| {
        b.iter_batched(|| -1.0, |x| abs2(black_box(x)), BatchSize::SmallInput)
    });
    c.bench_function("abs2 (+12345678.90123456)", move |b| {
        b.iter_batched(|| 12345678.90123456, |x| abs2(black_box(x)), BatchSize::SmallInput)
    });
    c.bench_function("abs2 (-12345678.90123456)", move |b| {
        b.iter_batched(|| -12345678.90123456, |x| abs2(black_box(x)), BatchSize::SmallInput)
    });
    let mut rng = rand::thread_rng();
    c.bench_function("abs2 (rand)", move |b| {
        b.iter_batched(|| rng.gen::<f64>(), |x| abs2(black_box(x)), BatchSize::SmallInput)
    });
}


criterion_group!(
    benches,
    bench_noop,
    bench_abs1,
    bench_abs2,
    bench_old,
);
criterion_main!(benches);

// look into: https://github.com/bheisler/criterion.rs/issues/23
// for per iteration setup