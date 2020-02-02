extern crate rand;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
//use std::iter;

/*
#[inline]
fn noop<T>(x: T) -> T {
    x
}
*/

fn basic_test(c: &mut Criterion) {
    c.bench_function("noop (const)", |b| b.iter(|| 1.0));
    c.bench_function("noop (const w/ black_box)", |b| b.iter(|| black_box(1.0)));

    //c.bench_function("noop (call 1)", |b| b.iter(|| noop(1.0)));
    //c.bench_function("noop (call 2)", |b| b.iter(|| noop(black_box(1.0))));
    //c.bench_function("noop (call 3)", |b| b.iter(|| black_box(noop(1.0))));

    c.bench_function("abs (call 1)", |b| b.iter(|| ((-1.0_f64)).abs()));
    c.bench_function("abs (call 2)", |b| b.iter(|| black_box(-1.0_f64).abs()));
    c.bench_function("abs (call 3)", |b| b.iter(|| black_box(-1.0_f64.abs())));
}

criterion_group!(benches, basic_test);
criterion_main!(benches);
