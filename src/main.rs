#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]


extern crate riftlib;

use riftlib::glmath;




struct Point {
    x: int,
    y: int,
    z: int,
}


fn main() {

  let v = glmath::Vec3{x: 0f32, y: 0f32, z: 0f32};

  fn nested(x: int) -> int {
    2*x
  }
  println!("Hello, world!! {}", nested(2));
  
  
  let mut point = Point { 
    z: 0, 
    x: 0, 
    y: 0, 
  };
  //println!("{}", point)
  
  let x = 5i;
  let y = 10i;

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

fn cmp(a: int, b: int) -> Ordering {
    if a < b { Less }
    else if a > b { Greater }
    else { Equal }
}
