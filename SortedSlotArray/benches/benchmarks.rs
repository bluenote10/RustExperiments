use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use std::time::Duration;

use sorted_slot_array::sorted_array::SortedArray;
use sorted_slot_array::splay::SplaySet;
use sorted_slot_array::vec_set::VecSet;

use rand::Rng;

static mut NUM_CALLS_A: u64 = 0;
static mut NUM_CALLS_B: u64 = 0;


#[inline]
fn cmp_a(a: &f64, b: &f64) -> std::cmp::Ordering {
    unsafe {
        NUM_CALLS_A += 1;
    }
    a.partial_cmp(b).unwrap()
}

#[inline]
fn cmp_b(a: &f64, b: &f64) -> std::cmp::Ordering {
    unsafe {
        NUM_CALLS_B += 1;
    }
    a.partial_cmp(b).unwrap()
}

#[inline]
fn cmp_c(a: &f64, b: &f64) -> std::cmp::Ordering {
    unsafe {
        NUM_CALLS_B += 1;
    }
    a.partial_cmp(b).unwrap()
}

fn gen_data() -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let vals: Vec<f64> = (0..1000).map(|_| rng.gen()).collect();
    vals
}

#[rustfmt::skip]
fn benchmarks(c: &mut Criterion) {

    c.bench_function("splay insert", |b| b.iter_batched(
        || gen_data(),
        |data| {
            let mut set = SplaySet::new(cmp_a);
            for x in &data {
                set.insert(*x);
            }
            //let out: Vec<_> = set.into_iter().collect();
            //out
            set.len()

        },
        BatchSize::SmallInput,
    ));

    c.bench_function("sarray insert", |b| b.iter_batched(
        || gen_data(),
        |data| {
            let mut set = SortedArray::new(cmp_b, 10, 3);
            for x in &data {
                set.insert(*x);
            }
            //set.collect()
            set.len()
        },
        BatchSize::SmallInput,
    ));

    c.bench_function("vecset insert", |b| b.iter_batched(
        || gen_data(),
        |data| {
            let mut set = VecSet::new(cmp_c, 10);
            for x in &data {
                set.insert(*x);
            }
            //set.collect()
            set.len()
        },
        BatchSize::SmallInput,
    ));

    c.bench_function("splay delete", |b| b.iter_batched(
        || {
            let data = gen_data();
            let mut set = SplaySet::new(cmp_a);
            for x in &data {
                set.insert(*x);
            }
            (data, set)
        },
        |(data, mut set)| {
            for x in &data {
                set.remove(x);
            }
            set.len()
        },
        BatchSize::SmallInput,
    ));

    c.bench_function("sarray delete", |b| b.iter_batched(
        || {
            let data = gen_data();
            let mut set = SortedArray::new(cmp_b, 1000, 3);
            for x in &data {
                set.insert(*x);
            }
            (data, set)
        },
        |(data, mut set)| {
            for x in &data {
                set.remove(x);
            }
            set.len()
        },
        BatchSize::SmallInput,
    ));

    c.bench_function("vecset delete", |b| b.iter_batched(
        || {
            let data = gen_data();
            let mut set = VecSet::new(cmp_c, 1000);
            for x in &data {
                set.insert(*x);
            }
            (data, set)
        },
        |(data, mut set)| {
            for x in &data {
                set.remove(x);
            }
            set.len()
        },
        BatchSize::SmallInput,
    ));

}

fn config() -> Criterion {
    Criterion::default()
        .measurement_time(Duration::from_secs_f64(3.0))
        .warm_up_time(Duration::from_secs_f64(0.1))
}

criterion_group! {
    name = benches;
    config = config();
    targets = benchmarks
}
criterion_main!(benches);
