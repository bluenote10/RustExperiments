extern crate sorted_slot_array;

use std::cmp::{Eq, Ord, Ordering};

use rand::Rng;

use sorted_slot_array::sorted_array::SortedArray;
use sorted_slot_array::array_tree::ArrayTree;
use sorted_slot_array::splay::SplaySet;
use sorted_slot_array::vec_set::VecSet;
use std::collections::BTreeSet;
// use ordered_float::OrderedFloat;

use std::time::{Duration, Instant};

use pretty_assertions::assert_eq;

mod helpers;


macro_rules! create_cmp {
    ($func:ident, $get:ident, $count:ident) => {
        static mut $count: u64 = 0;

        #[inline]
        fn $func(a: &f64, b: &f64) -> std::cmp::Ordering {
            unsafe {
                $count += 1;
            }
            a.exp().partial_cmp(&b.exp()).unwrap()
        }

        fn $get() -> u64 {
            unsafe {
                $count
            }
        }
    };
}

#[derive(Debug)]
struct FloatWrapper(f64);

impl Eq for FloatWrapper {}

impl PartialEq for FloatWrapper {
    fn eq(&self, other: &Self) -> bool {
        panic!("eq called");
        if self.0.is_nan() && other.0.is_nan() {
            true
        } else {
            self.0 == other.0
        }
    }
}

impl PartialOrd for FloatWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        panic!("partial_cmp called");
        Some(self.cmp(other))
    }
}

create_cmp!(cmp_b_tree, get_num_calls_b_tree, NUM_CALLS_B_TREE);

impl Ord for FloatWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        cmp_b_tree(&self.0, &other.0)
    }
}

fn gen_rand_values(n: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let values: Vec<f64> = (0..n).map(|_| rng.gen()).collect();
    values
}

create_cmp!(cmp_array_tree, get_num_calls_array_tree, NUM_CALLS_ARRAY_TREE);
create_cmp!(cmp_splay_tree, get_num_calls_splay_tree, NUM_CALLS_SPLAY_TREE);


fn generic_fill_benchmark<T, F1, F2, F3>(values: &[f64], measure_every: i32, init: F1, insert: F2, get_len: F3) -> Vec<(usize, f64)>
where
    F1: Fn() -> T,
    F2: Fn(&mut T, f64),
    F3: Fn(&T) -> usize,

{
    let mut set = init();

    let mut elapsed_times = Vec::with_capacity(values.len());

    let start = Instant::now();
    for (i, x) in values.iter().enumerate() {
        insert(&mut set, *x);

        let len = i + 1;
        if len % measure_every as usize == 0 {
            elapsed_times.push((len, start.elapsed().as_secs_f64()));
        }
    }
    assert_eq!(get_len(&set), values.len());

    elapsed_times

}

/*
fn benchmark_fill_array_tree(values: &[f64]) -> usize {
    let mut set = ArrayTree::new(cmp_array_tree, 16);
    for x in values {
        set.insert(*x);
    }
    set.len()
}

fn benchmark_fill_splay_tree(values: &[f64]) -> usize {
    let mut set = SplaySet::new(cmp_splay_tree);
    for x in values {
        set.insert(*x);
    }
    set.len()
}

fn benchmark_fill_b_tree(values: &[f64]) -> usize {
    let mut set = BTreeSet::new();
    for x in values {
        set.insert(FloatWrapper(*x));
    }
    set.len()
}
*/

/*
struct Benchmark<F>
where
    F: Fn(&[f64]) -> usize
{
    name: String,
    func: F,
}
*/
#[derive(Clone)]
struct Benchmark<'a>
{
    name: String,
    //func: fn(&[f64]) -> usize,
    func: &'a dyn Fn(&[f64]) -> Vec<(usize, f64)>,
    run: i32,
}


fn run_fill_benchmarks() {

    let n = 1000000;
    let measure_every = 25;
    let num_runs = 3;

    let fill_array_tree = |values: &[f64]| {
        generic_fill_benchmark(
            &values,
            measure_every,
            || ArrayTree::new(cmp_array_tree, 512),
            |set, x| { set.insert(x); },
            |set| set.len(),
        )
    };

    let fill_splay_tree = |values: &[f64]| {
        generic_fill_benchmark(
            &values,
            measure_every,
            || SplaySet::new(cmp_splay_tree),
            |set, x| { set.insert(x); },
            |set| set.len(),
        )
    };


    let fill_b_tree = |values: &[f64]| {
        generic_fill_benchmark(
            &values,
            measure_every,
            || BTreeSet::new(),
            |set, x| { set.insert(FloatWrapper(x)); },
            |set| set.len(),
        )
    };

    for run in 0..=num_runs {
        let benchmarks: Vec<Benchmark> = vec![
            Benchmark {
                run,
                name: "SplayTree".to_string(),
                func: &fill_splay_tree,
            },
            Benchmark {
                run,
                name: "BTree".to_string(),
                func: &fill_b_tree,
            },
            Benchmark {
                run,
                name: "ArrayStump".to_string(),
                func: &fill_array_tree,
            },
        ];
        let benchmarks = helpers::shuffle(&benchmarks);

        let values = gen_rand_values(n);
        assert_eq!(values.len(), n);

        for benchmark in benchmarks {
            println!("Running benchmark: {} / {}", benchmark.name, benchmark.run);

            let measurements = (benchmark.func)(&values);

            let iters: Vec<_> = measurements.iter().map(|i_t| i_t.0).collect();
            let times: Vec<_> = measurements.iter().map(|i_t| i_t.1).collect();

            if run > 0 {
                helpers::export_elapsed_times(
                    &benchmark.name,
                    benchmark.run,
                    &format!("results/fill_avg_{}_{}.json", benchmark.name, benchmark.run),
                    &iters,
                    &times,
                );
            }
        }
    }

    helpers::call_plots();
}


fn run_fill_statistics() {
    let n = 1000000;
    let values = gen_rand_values(n);

    let mut set = ArrayTree::new(cmp_array_tree, 256);

    let mut iters = Vec::new();
    let mut times = Vec::new();
    let mut fill_ratio = Vec::new();
    let mut num_blocks = Vec::new();
    let mut capacity = Vec::new();


    let start = Instant::now();
    for (i, x) in values.iter().enumerate() {
        set.insert(*x);
        if (i + 1) % 10 == 0 {
            iters.push(i + 1);
            times.push(start.elapsed().as_secs_f64());
            fill_ratio.push(set.get_leaf_fill_ratio());
            num_blocks.push(set.get_num_blocks());
            capacity.push(set.get_capacity());
        }
    }
    assert_eq!(set.len(), values.len());

    helpers::export_stats(&iters, &times, &fill_ratio, &num_blocks, &capacity);
}


fn main() {
    if cfg!(debug_assertions) {
        println!("WARNING: Debug assertions are enabled. Benchmarking should be done in `--release`.");
    }

    run_fill_benchmarks();
    //run_fill_statistics();

    /*
    // let a = if 1 < 2 { benchmark_fill_array_tree } else { benchmark_fill_splay_tree };
    let arr: Vec<fn(&[f64]) -> usize> = vec![benchmark_fill_array_tree, benchmark_fill_splay_tree];

    let mut rng = rand::thread_rng();

    let n = 100;
    let vals: Vec<f64> = (0..n).map(|_| rng.gen()).collect();

    let mut set_a = SplaySet::new(cmp_a);
    let mut set_b = SortedArray::new(cmp_b, 20, 4);
    let mut set_c = VecSet::new(cmp_c, 20);
    let mut set_d = ArrayTree::new(cmp_d, 16);

    for x in &vals {
        set_a.insert(*x);
        set_b.insert(*x);
        set_c.insert(*x);
        set_d.insert(*x);
    }

    set_b.debug();

    let data_a: Vec<_> = set_a.into_iter().collect();
    let data_b = set_b.collect();
    let data_c = set_c.collect();
    let data_d = set_d.collect();

    println!("{:?}", vals);
    assert_eq!(data_a.len(), n);
    assert_eq!(data_b.len(), n);
    assert_eq!(data_c.len(), n);
    assert_eq!(data_d.len(), n);
    assert_eq!(data_a, data_b);
    assert_eq!(data_a, data_c);
    assert_eq!(data_a, data_d);
    */

    println!("Num calls array tree: {:12}", get_num_calls_array_tree());
    println!("Num calls splay tree: {:12}", get_num_calls_splay_tree());
    println!("Num calls B tree:     {:12}", get_num_calls_b_tree());

}