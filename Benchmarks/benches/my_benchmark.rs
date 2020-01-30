use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::iter;

use criterion::BenchmarkId;
//use criterion::Criterion;
use criterion::Throughput;

fn do_something(x: f64) -> f64 {
    //x.abs().sin().exp()
    if x > 0.0 { x } else { -x }
}

fn from_elem(c: &mut Criterion) {
    let size: usize = 1024;

    for x in [0.0, 1.0, -1.0, 123.456, -123.456].iter() {
        c.bench_with_input(BenchmarkId::new("input_example", x), x, |b, &x| {
            b.iter(|| do_something(x));
        });
    }
}

criterion_group!(benches, from_elem);
criterion_main!(benches);