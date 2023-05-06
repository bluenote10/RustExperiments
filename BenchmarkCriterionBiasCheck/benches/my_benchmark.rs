use criterion::{criterion_group, criterion_main, Criterion, SamplingMode};
use rand::Rng;
use std::time::Instant;

fn dummy() {
    let now = Instant::now();

    let waiting_time_micros = rand::thread_rng().gen_range(1000..=2000);

    loop {
        let elapsed = now.elapsed().as_micros();
        if elapsed > waiting_time_micros {
            break;
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("dummy-group");
    group.sampling_mode(SamplingMode::Linear);
    group.bench_function("dummy", |b| b.iter(|| dummy()));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
