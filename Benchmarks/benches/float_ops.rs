use criterion::{black_box, criterion_group, criterion_main, Criterion};

mod utils;
use utils::bench_function_with_noop;


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


criterion_group!(
    benches,
    basics,
);
criterion_main!(benches);
