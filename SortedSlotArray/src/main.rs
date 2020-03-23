extern crate sorted_slot_array;

use rand::Rng;

use sorted_slot_array::sorted_array::SortedArray;
use sorted_slot_array::splay::SplaySet;
use sorted_slot_array::vec_set::VecSet;

use pretty_assertions::assert_eq;

static mut NUM_CALLS_A: u64 = 0;
static mut NUM_CALLS_B: u64 = 0;
static mut NUM_CALLS_C: u64 = 0;


fn cmp_a(a: &f64, b: &f64) -> std::cmp::Ordering {
    unsafe {
        NUM_CALLS_A += 1;
    }
    a.partial_cmp(b).unwrap()
}

fn cmp_b(a: &f64, b: &f64) -> std::cmp::Ordering {
    unsafe {
        NUM_CALLS_B += 1;
    }
    a.partial_cmp(b).unwrap()
}

fn cmp_c(a: &f64, b: &f64) -> std::cmp::Ordering {
    unsafe {
        NUM_CALLS_C += 1;
    }
    a.partial_cmp(b).unwrap()
}

fn main() {

    let mut rng = rand::thread_rng();

    let n = 100;
    let vals: Vec<f64> = (0..n).map(|_| rng.gen()).collect();

    let mut set_a = SplaySet::new(cmp_a);
    let mut set_b = SortedArray::new(cmp_b, 20, 4);
    let mut set_c = VecSet::new(cmp_c, 20);

    for x in &vals {
        set_a.insert(*x);
        set_b.insert(*x);
        set_c.insert(*x);
    }

    set_b.debug();

    let data_a: Vec<_> = set_a.into_iter().collect();
    let data_b = set_b.collect();
    let data_c = set_c.collect();

    assert_eq!(data_a.len(), n);
    assert_eq!(data_b.len(), n);
    assert_eq!(data_c.len(), n);
    assert_eq!(data_a, data_b);
    assert_eq!(data_a, data_c);

    unsafe {
        println!("Num calls A: {}", NUM_CALLS_A);
        println!("Num calls B: {}", NUM_CALLS_B);
        println!("Num calls C: {}", NUM_CALLS_C);
    }

}