#![allow(dead_code)]

extern crate rand;

mod utils;
use utils::bench_function_with_noop;

use mycrate::{signed_area, signed_area_fast, intersection, LineIntersection};

use geo_types::Coordinate;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::time::{Duration, Instant};


#[inline]
fn noop<T>(x: T) -> T {
    x
}


fn basics(c: &mut Criterion) {
    let x = 12345678.90123456_f64;
    c.bench_function("multiply", |b| b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            || black_box(x),
            || black_box(x) * black_box(x),
        )
    }));
    c.bench_function("add", |b| b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            || black_box(x),
            || black_box(x) + black_box(x),
        )
    }));
    c.bench_function("abs", |b| b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            || black_box(1.0_f64),
            || black_box(1.0_f64).abs(),
        )
    }));
}


fn bench_signed_area(c: &mut Criterion) {
    c.bench_function("signed_area (robust)", |b| b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            || black_box(1.0),
            || signed_area(
                Coordinate::from(black_box((1.0, 2.0))),
                Coordinate::from(black_box((3.0, 4.0))),
                Coordinate::from(black_box((5.0, 6.0))),
            ),
        )
    }));
    c.bench_function("signed_area (fast)", |b| b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            || black_box(1.0),
            || signed_area_fast(
                Coordinate::from(black_box((1.0, 2.0))),
                Coordinate::from(black_box((3.0, 4.0))),
                Coordinate::from(black_box((5.0, 6.0))),
            ),
        )
    }));
}


fn bench_intersection(c: &mut Criterion) {
    c.bench_function("intersection", |b| b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            || LineIntersection::Point(Coordinate::from(black_box((1.0, 2.0)))),
            || intersection(
                Coordinate::from(black_box((1.0, 2.0))),
                Coordinate::from(black_box((3.0, 4.0))),
                Coordinate::from(black_box((5.0, 6.0))),
                Coordinate::from(black_box((7.0, 8.0))),
            ),
        )
    }));
}


criterion_group!(
    benches,
    //basics,
    //bench_signed_area,
    bench_intersection,
);
criterion_main!(benches);
