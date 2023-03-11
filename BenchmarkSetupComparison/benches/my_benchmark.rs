use std::time::Instant;

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion, SamplingMode};

fn dummy(buffer: &mut Vec<i32>, n_iter: usize) {
    let mut i = 0;
    for _ in 0..n_iter {
        buffer[i] += 1;
        i += 1;
        if i >= buffer.len() {
            i = 0;
        }
    }
}

static mut CALL_COUNT: usize = 0;

fn setup(name: &str, buffer_size: usize) -> Vec<i32> {
    let do_print = false;
    unsafe {
        CALL_COUNT += 1;
        if do_print {
            println!("Running setup for '{name}'... [call: {CALL_COUNT}]");
        }
    }
    vec![0; buffer_size]
}

fn reset_call_count() {
    unsafe {
        println!("Call count: {CALL_COUNT}");
        CALL_COUNT = 0;
    }
}

///
/// Notes:
///
/// Best to run with:
///
///     cargo bench -- --verbose
///
/// Conclusion:
///
/// v1 setups once. This has the least overhead, but is prone to systematic bias, because
/// the performance of the collected samples can depend entire on the one instance used for
/// all iterations. May lead to over confidence, and false alarms in change detection.
///
/// v4 is the other extreme: Setup is called for each inner iterations. This results in the
/// largest overhead, but since every inner iteration gets a fresh instance, it should be
/// least prone to underestimating the uncertainty resulting from "instance effects".
///
/// v2 (setup inside closure) and v3 seem to have exactly the same semantics: Both run the
/// setup once for each batch / "sample" (which is a group of inner iterations). This seems
/// to be a reasonable trade-off between overhead and systematic bias. Syntactically v2
/// looks a bit nicer than v3. The downside of v2 is that criterions.rs doesn't really
/// specify what are the semantics of the outer closure, i.e., it might as well change to
/// calling it only once?
///
/// The behavior seems to be the same for flat and linear sampling methods.
///
fn criterion_benchmark(c: &mut Criterion) {
    let n_iter = 1_000_000;
    let buffer_size = 1024;

    let mut group = c.benchmark_group("dummy-group");
    group.sampling_mode(SamplingMode::Linear).sample_size(10);

    // v1: global
    let mut buffer = setup("global", buffer_size);
    group.bench_function("dummy (v1)", |b| b.iter(|| dummy(&mut buffer, n_iter)));

    reset_call_count();

    // v2: inside outer closure
    group.bench_function("dummy (v2)", |b| {
        let mut buffer = setup("inside outer closure", buffer_size);
        b.iter(|| dummy(&mut buffer, n_iter))
    });

    reset_call_count();

    // v3: iter custom
    group.bench_function("dummy (v3)", |b| {
        b.iter_custom(|iter| {
            let mut buffer = setup("iter_custom", buffer_size);
            let now = Instant::now();
            for _ in 0..iter {
                black_box(dummy(&mut buffer, n_iter))
            }
            now.elapsed()
        })
    });

    reset_call_count();

    // v4: iter batched
    group.bench_function("dummy (v4)", |b| {
        b.iter_batched(
            || setup("iter_batched", buffer_size),
            |mut buffer| dummy(&mut buffer, n_iter),
            BatchSize::PerIteration,
        )
    });

    reset_call_count();

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
