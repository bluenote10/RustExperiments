use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dashu_ratio::Relaxed;
use fraction::{DynaFraction, Fraction};
use num_rational::Ratio;
use rand::prelude::*;
use second_order_expansion::SOE;
use std::time::Instant;

#[derive(Clone, Copy)]
struct Vec32 {
    x: f32,
    y: f32,
}

fn transform_f32(scale: Vec32, offset: Vec32, v: Vec32) -> Vec32 {
    Vec32 {
        x: scale.x * v.x + offset.x,
        y: scale.y * v.y + offset.y,
    }
}

#[derive(Clone, Copy)]
struct Vec64 {
    x: f64,
    y: f64,
}

fn transform_f64(scale: Vec64, offset: Vec64, v: Vec64) -> Vec64 {
    Vec64 {
        x: scale.x * v.x + offset.x,
        y: scale.y * v.y + offset.y,
    }
}

#[derive(Clone)]
struct VecFraction {
    x: DynaFraction<u64>,
    y: DynaFraction<u64>,
}

fn transform_fraction(scale: &VecFraction, offset: &VecFraction, v: &VecFraction) -> VecFraction {
    VecFraction {
        x: scale.x.clone() * v.x.clone() + offset.x.clone(),
        y: scale.y.clone() * v.y.clone() + offset.y.clone(),
    }
}

#[derive(Clone)]
struct VecRatio {
    x: Ratio<num_bigint::BigInt>,
    y: Ratio<num_bigint::BigInt>,
}

fn transform_ratio(scale: &VecRatio, offset: &VecRatio, v: &VecRatio) -> VecRatio {
    VecRatio {
        x: scale.x.clone() * v.x.clone() + offset.x.clone(),
        y: scale.y.clone() * v.y.clone() + offset.y.clone(),
    }
}

#[derive(Clone)]
struct VecDashu {
    x: Relaxed,
    y: Relaxed,
}

fn transform_dashu(scale: &VecDashu, offset: &VecDashu, v: &VecDashu) -> VecDashu {
    VecDashu {
        x: scale.x.clone() * v.x.clone() + offset.x.clone(),
        y: scale.y.clone() * v.y.clone() + offset.y.clone(),
    }
}

#[derive(Clone, Copy)]
struct VecSOE {
    x: SOE,
    y: SOE,
}

fn transform_soe(scale: VecSOE, offset: VecSOE, v: VecSOE) -> VecSOE {
    VecSOE {
        x: scale.x * v.x + offset.x,
        y: scale.y * v.y + offset.y,
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();

    let mut group = c.benchmark_group("transform");
    // group.sampling_mode(SamplingMode::Linear).sample_size(10);

    group.bench_function("soe", |b| {
        b.iter_custom(|iter| {
            let scale = VecSOE {
                x: SOE::from(rng.gen::<f64>()),
                y: SOE::from(rng.gen::<f64>()),
            };
            let offset = VecSOE {
                x: SOE::from(rng.gen::<f64>()),
                y: SOE::from(rng.gen::<f64>()),
            };
            let v = VecSOE {
                x: SOE::from(rng.gen::<f64>()),
                y: SOE::from(rng.gen::<f64>()),
            };
            let now = Instant::now();
            for _ in 0..iter {
                black_box(transform_soe(scale, offset, v));
            }
            now.elapsed()
        })
    });

    group.bench_function("dashu", |b| {
        b.iter_custom(|iter| {
            let scale = VecDashu {
                x: Relaxed::try_from(rng.gen::<f64>()).unwrap(),
                y: Relaxed::try_from(rng.gen::<f64>()).unwrap(),
            };
            let offset = VecDashu {
                x: Relaxed::try_from(rng.gen::<f64>()).unwrap(),
                y: Relaxed::try_from(rng.gen::<f64>()).unwrap(),
            };
            let v = VecDashu {
                x: Relaxed::try_from(rng.gen::<f64>()).unwrap(),
                y: Relaxed::try_from(rng.gen::<f64>()).unwrap(),
            };
            let now = Instant::now();
            for _ in 0..iter {
                black_box(transform_dashu(&scale, &offset, &v));
            }
            now.elapsed()
        })
    });

    group.bench_function("num_ratio", |b| {
        b.iter_custom(|iter| {
            let scale = VecRatio {
                x: Ratio::from_float(rng.gen::<f64>()).unwrap(),
                y: Ratio::from_float(rng.gen::<f64>()).unwrap(),
            };
            let offset = VecRatio {
                x: Ratio::from_float(rng.gen::<f64>()).unwrap(),
                y: Ratio::from_float(rng.gen::<f64>()).unwrap(),
            };
            let v = VecRatio {
                x: Ratio::from_float(rng.gen::<f64>()).unwrap(),
                y: Ratio::from_float(rng.gen::<f64>()).unwrap(),
            };
            let now = Instant::now();
            for _ in 0..iter {
                black_box(transform_ratio(&scale, &offset, &v));
            }
            now.elapsed()
        })
    });

    group.bench_function("fraction", |b| {
        b.iter_custom(|iter| {
            let scale = VecFraction {
                x: DynaFraction::<u64>::from_fraction(Fraction::from(rng.gen::<f64>())),
                y: DynaFraction::<u64>::from_fraction(Fraction::from(rng.gen::<f64>())),
            };
            let offset = VecFraction {
                x: DynaFraction::<u64>::from_fraction(Fraction::from(rng.gen::<f64>())),
                y: DynaFraction::<u64>::from_fraction(Fraction::from(rng.gen::<f64>())),
            };
            let v = VecFraction {
                x: DynaFraction::<u64>::from_fraction(Fraction::from(rng.gen::<f64>())),
                y: DynaFraction::<u64>::from_fraction(Fraction::from(rng.gen::<f64>())),
            };
            let now = Instant::now();
            for _ in 0..iter {
                black_box(transform_fraction(&scale, &offset, &v));
            }
            now.elapsed()
        })
    });

    group.bench_function("f32", |b| {
        b.iter_custom(|iter| {
            let scale = Vec32 {
                x: rng.gen(),
                y: rng.gen(),
            };
            let offset = Vec32 {
                x: rng.gen(),
                y: rng.gen(),
            };
            let v = Vec32 {
                x: rng.gen(),
                y: rng.gen(),
            };
            let now = Instant::now();
            for _ in 0..iter {
                black_box(transform_f32(scale, offset, v));
            }
            now.elapsed()
        })
    });

    group.bench_function("f64", |b| {
        b.iter_custom(|iter| {
            let scale = Vec64 {
                x: rng.gen(),
                y: rng.gen(),
            };
            let offset = Vec64 {
                x: rng.gen(),
                y: rng.gen(),
            };
            let v = Vec64 {
                x: rng.gen(),
                y: rng.gen(),
            };
            let now = Instant::now();
            for _ in 0..iter {
                black_box(transform_f64(scale, offset, v));
            }
            now.elapsed()
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
