extern crate sorted_slot_array;

use std::cmp::{Eq, Ord, Ordering};

use rand::Rng;

use sorted_slot_array::sorted_array::SortedArray;
use sorted_slot_array::array_tree::ArrayTree;
use sorted_slot_array::splay::SplaySet;
use sorted_slot_array::vec_set::VecSet;
use std::collections::BTreeSet;
use ordered_float::OrderedFloat;

use std::time::{Duration, Instant};

use pretty_assertions::assert_eq;

macro_rules! create_cmp {
    ($func:ident, $get:ident, $count:ident) => {
        static mut $count: u64 = 0;

        #[inline]
        fn $func(a: &f64, b: &f64) -> std::cmp::Ordering {
            unsafe {
                $count += 1;
            }
            a.partial_cmp(b).unwrap()
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

fn benchmark_fill_array_tree(values: &[f64]) -> usize {
    let mut set = ArrayTree::new(cmp_array_tree, 16);
    for x in values {
        set.insert(*x);
    }
    set.len()
}

create_cmp!(cmp_splay_tree, get_num_calls_splay_tree, NUM_CALLS_SPLAY_TREE);

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

/*
struct Benchmark<F>
where
    F: Fn(&[f64]) -> usize
{
    name: String,
    func: F,
}
*/
struct Benchmark
{
    name: String,
    func: fn(&[f64]) -> usize,
}

fn main() {

    let benchmarks: Vec<Benchmark> = vec![
        Benchmark {
            name: "SplayTree".to_string(),
            func: benchmark_fill_splay_tree,
        },
        Benchmark {
            name: "BTree".to_string(),
            func: benchmark_fill_b_tree,
        },
        Benchmark {
            name: "ArrayStump".to_string(),
            func: benchmark_fill_array_tree,
        },
    ];

    for benchmark in benchmarks {
        println!("Running benchmark: {}", benchmark.name);
        let n = 100000;
        let values = gen_rand_values(n);
        assert_eq!(values.len(), n);

        let start = Instant::now();
        let len = (benchmark.func)(&values);
        let time = start.elapsed();

        println!("{:?}", time);

        assert_eq!(len, n)
    }

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