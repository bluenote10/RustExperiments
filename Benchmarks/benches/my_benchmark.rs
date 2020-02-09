#![allow(dead_code)]

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


fn compare_small(c: &mut Criterion) {
    use criterion::black_box;

    let mut group = c.benchmark_group("small");
    /*
    group.bench_with_input("unlooped", &10, |b, i| b.iter(|| i + 10));
    group.bench_with_input("looped", &10, |b, i| b.iter(|| {
        for _ in 0..10000 {
            black_box(i + 10);
        }
    }));
    */
    group.bench_function("noop (const)", |b| b.iter(|| 1.0));
    group.bench_function("noop (const black_box)", |b| b.iter(|| black_box(1.0)));
    group.bench_function("noop (call)", |b| b.iter(|| noop(1.0)));
    group.bench_function("noop (call black_box)", |b| b.iter(|| noop(black_box(1.0))));
    group.bench_with_input("input noop (nothing)", &0.0, |b, x| b.iter(|| x));
    group.bench_with_input("input noop (deref)", &0.0, |b, x| b.iter(|| *x));
    group.bench_with_input("input noop (call)", &0.0, |b, x| b.iter(|| noop(*x)));
    group.bench_with_input("input noop (call, no-deref)", &0.0, |b, &x| b.iter(|| noop(x)));
    group.bench_with_input("abs1(0.0)", &0.0, |b, &x| b.iter(|| abs1(black_box(x))));
    group.bench_with_input("abs1(1.0)", &1.0, |b, &x| b.iter(|| abs1(black_box(x))));
    group.bench_with_input("abs1(-1.0)", &-1.0, |b, &x| b.iter(|| abs1(black_box(x))));
    group.bench_with_input("abs2(0.0)", &0.0, |b, &x| b.iter(|| abs2(black_box(x))));
    group.bench_with_input("abs2(1.0)", &1.0, |b, &x| b.iter(|| abs2(black_box(x))));
    group.bench_with_input("abs2(-1.0)", &-1.0, |b, &x| b.iter(|| abs2(black_box(x))));

    group.finish();
}

criterion_group!(
    benches,
    //bench_noop,
    //bench_abs1,
    //bench_abs2,
    //bench_old,
    compare_small
);
criterion_main!(benches);

// look into: https://github.com/bheisler/criterion.rs/issues/23
// for per iteration setup