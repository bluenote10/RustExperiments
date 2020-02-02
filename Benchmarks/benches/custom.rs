extern crate rand;

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

fn custom(c: &mut Criterion) {
    c.bench_function("test", |b| b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            || noop(black_box(1.0_f64)),
            || black_box(1.0_f64).abs(),
        )
    }));

}

/*
fn custom_test(c: &mut Criterion) {
    c.bench_function("noop (const)", |b| b.iter_custom(|iters| {
        let start = Instant::now();
        for _i in 0..iters {
            black_box(-1.0_f64);
        }
        let t1 = start.elapsed();

        let start = Instant::now();
        for _i in 0..iters {
            black_box(black_box(-1.0_f64).abs());
        }
        let t2 = start.elapsed();

        /*
        if iters % 2 == 0 {
            std::time::Duration::from_nanos(1 * iters)
        } else {
            std::time::Duration::from_nanos(2 * iters)
        }
        */
        //println!("{:?} {:?} {:?}", t2, t1, t2-t1);

        let diff = t2.checked_sub(t1);
        if let Some(diff) = diff {
            diff
        } else {
            std::time::Duration::from_nanos(0)
        }
        /*
        if (t2 - t1) > std::time::Duration::from_secs(0) {
            (t2 - t1)
        } else {
            std::time::Duration::from_nanos(1)
        }
        */
    }));

}
*/

//criterion_group!(benches, basic_test);
criterion_group!(benches, custom);
criterion_main!(benches);
