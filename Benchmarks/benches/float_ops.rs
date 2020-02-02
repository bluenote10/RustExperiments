extern crate rand;
use mycrate::{signed_area, signed_area_fast};

use geo_types::Coordinate;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::time::{Duration, Instant};


fn bench_function_with_noop<O, N, R>(iters: u64, mut noop: N, mut routine: R) -> Duration
where
    N: FnMut() -> O,
    R: FnMut() -> O,
{
    let start = Instant::now();
    for _i in 0..iters {
        black_box(noop());
    }
    let t_noop = start.elapsed();

    let start = Instant::now();
    for _i in 0..iters {
        black_box(routine());
    }
    let t_routine = start.elapsed();

    if let Some(diff) = t_routine.checked_sub(t_noop) {
        diff
    } else {
        std::time::Duration::from_nanos(0)
    }

}


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
                Coordinate::from(black_box((2.0, 3.0))),
                Coordinate::from(black_box((4.0, 5.0))),
            ),
        )
    }));
    c.bench_function("signed_area (fast)", |b| b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            || black_box(1.0),
            || signed_area_fast(
                Coordinate::from(black_box((1.0, 2.0))),
                Coordinate::from(black_box((2.0, 3.0))),
                Coordinate::from(black_box((4.0, 5.0))),
            ),
        )
    }));
}


criterion_group!(
    benches,
    //basics,
    bench_signed_area,
);
criterion_main!(benches);
