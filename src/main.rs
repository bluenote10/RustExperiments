#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_assignments)]

extern crate riftlib;

use riftlib::glmath;



/// structure used in several tests
struct Point {
  x: int,
  y: int,
}


#[test]
fn stuff_from_the_guide() {
  fn nested(x: int) -> int {
    2*x
  }
  println!("Hello, world!! {}", nested(2));
  
  
  let mut point = Point { 
    x: 0, 
    y: 0, 
  };
  //println!("{}", point)
  
  let x = 5i;
  let y = 10i;

  fn cmp(a: int, b: int) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
  }

  let z = match cmp(x, y) {
    Less    => println!("less"),
    Greater => println!("greater"),
    Equal   => println!("equal"),
  };
  
  println!("{}", z);
  
  for x in range(0i, 10i) {
    println!("{}", x);
  }
}


#[test]
fn reference_reassignment() {

  { // i and j immutable
    let i = 0i;
    let j = 1i;
  
    let mut rf_mu = &i;
    let     rf_im = &i;

    // rf_im = &j; // error: re-assignment of immutable variable `rf_im`
    rf_mu = &j;
    rf_mu = &i; // and back...

    // assigning a constant is not allowed
    // rf_mu = &42; // error: borrowed value does not live long enough
    
    // the mutability of the value depends on the mutability of what we borrowed
    // *rf_mu = 5; // error: cannot assign to immutable dereference of `&`-pointer `*rf_mu`
    // *rf_im = 5; // error: cannot assign to immutable dereference of `&`-pointer `*rf_im`
  }
  { // in order to borrow &mut, i must be mut itself
    let i = 0i;
    // let     r = &mut i; // error: cannot borrow immutable local variable `i` as mutable
    // let mut r = &mut i; // error: cannot borrow immutable local variable `i` as mutable
  }
  { // now with mutable i and j
    let mut i = 0i;
    let mut j = 1i;
    let mut k = 2i;
  
    let mut rf_mu = &mut i;
    //let     rf_im = &mut i; // error: cannot borrow `i` as mutable more than once at a time
    let     rf_im = &mut j;

    // rf_im = &mut k; // error: re-assignment of immutable variable `rf_im`
    // rf_mu = &mut i; // error: cannot borrow `i` as mutable more than once at a time 
    rf_mu = &mut k;
    
    *rf_im = 3; // now k = 2 ?
    // assert_eq!(j, 4) // error: cannot borrow `j` as immutable because it is also borrowed as mutable
    *rf_mu = 4; // now j = 3 ?
    // suprisingly, re_mu has borrowed both k and i!
    //assert_eq!(k, 3) // error: cannot borrow `k` as immutable because it is also borrowed as mutable
    //assert_eq!(i, 3) // error: cannot borrow `i` as immutable because it is also borrowed as mutable
    
    // all these also do not work since we cannot borrow at all!
    // println!("{}", i);
    // println!("{}", j);
    // println!("{}", k);
  }


}



fn main() {

  let v = glmath::Vec3{x: 0f32, y: 0f32, z: 0f32};

}


