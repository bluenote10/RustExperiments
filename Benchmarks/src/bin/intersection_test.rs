extern crate mycrate;
use geo_types::Coordinate;

use mycrate::{intersection_impl, signed_area, signed_area_fast, LineIntersection, Float};

use float_extras::f64::nextafter;

pub trait NextAfter {
    fn nextafter(self, up: bool) -> Self;
}

impl NextAfter for f64 {
    fn nextafter(self, up: bool) -> Self {
        if up {
            nextafter(self, std::f64::INFINITY)
        } else {
            nextafter(self, std::f64::NEG_INFINITY)
        }
    }
}

pub fn refine<F>(
  a1: Coordinate<F>,
  a2: Coordinate<F>,
  b1: Coordinate<F>,
  b2: Coordinate<F>,
  i: Coordinate<F>,
  cur_score: F,
) -> Coordinate<F>
where
  F: Float + NextAfter,
{
    let p00 = Coordinate{x: i.x.nextafter(false), y: i.y.nextafter(false)};
    let p01 = Coordinate{x: i.x.nextafter(false), y: i.y};
    let p02 = Coordinate{x: i.x.nextafter(false), y: i.y.nextafter(true)};
    let p10 = Coordinate{x: i.x, y: i.y.nextafter(false)};
    let p12 = Coordinate{x: i.x, y: i.y.nextafter(true)};
    let p20 = Coordinate{x: i.x.nextafter(true), y: i.y.nextafter(false)};
    let p21 = Coordinate{x: i.x.nextafter(true), y: i.y};
    let p22 = Coordinate{x: i.x.nextafter(true), y: i.y.nextafter(true)};
    let neighbors = [p00, p01, p02, p10, p12, p20, p21, p22];
    let mut min = cur_score;
    let mut best_idx = -1;
    println!("{}", cur_score);
    for (idx, n) in neighbors.iter().enumerate() {
        let area1 = signed_area_fast(a1, a2, n.clone());
        let area2 = signed_area_fast(b1, b2, n.clone());
        let score = area1.abs() + area2.abs();
        if score < min {
            min = score;
            best_idx = idx as i64;
        }
        println!("{:?} {:?}", score, n);
    }
    if best_idx == -1 {
        return i;
    } else {
        println!("{:?} {:?} {:?}", min, best_idx, neighbors[best_idx as usize]);
        refine(a1, a2, b1, b2, neighbors[best_idx as usize].clone(), min)
    }
}


fn get_ulp(x: f64) -> f64 {
    x.nextafter(true) - x
}

fn ulp_test() {
    println!("{}", get_ulp(1.0));
    println!("{}", get_ulp(1.0.nextafter(false)));
    let mut x = 2.0 - 1e-10;
    let mut ulp = get_ulp(x);
    while x < 2.1 {
        let next_x = x.nextafter(true);
        let next_ulp = get_ulp(next_x);
        if next_ulp != ulp {
            println!("{} {}", next_x, next_ulp);
            ulp = next_ulp;
        }
        //println!("{} {}", x, ulp);
        x = next_x;
    }
}

fn main() {
    ulp_test();
    println!("Testing...");

    //Coordinate { x: -98.0, y: 530.0 } Coordinate { x: 530.0, y: 530.0 } Coordinate { x: 1.250012525025, y: 531.0 } Coordinate { x: 1.2500125250252, y: -531.0 }
    //s = 0.15804142121819267 => Coordinate { x: 1.2500125250249994, y: 530.0 }
    let a1 = Coordinate { x: -98.0, y: 530.0 };
    let a2 = Coordinate { x: 530.0, y: 530.0 };
    let b1 = Coordinate { x: 1.250012525025, y: 531.0 };
    let b2 = Coordinate { x: 1.2500125250252, y: -531.0 };

    let inter = intersection_impl(a1, a2, b1, b2);
    let inter = match inter {
        LineIntersection::Point(p) => Some(p),
        _ => None
    }.unwrap();

    println!("Intersection: {:?}", inter);

    println!("area 1: {}", signed_area_fast(a1, a2, inter));
    println!("area 2: {}", signed_area_fast(b1, b2, inter));

    println!("area 1 exact: {}", signed_area(a1, a2, inter));
    println!("area 2 exact: {}", signed_area(b1, b2, inter));

    let best = refine(a1, a2, b1, b2, inter, signed_area(a1, a2, inter).abs() + signed_area(b1, b2, inter).abs());
    println!("best found: {:?}", best);
}