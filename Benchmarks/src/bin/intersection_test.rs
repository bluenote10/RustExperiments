extern crate mycrate;
extern crate rand;

use geo_types::Coordinate;

use mycrate::{
    intersection_impl, signed_area, signed_area_fast, LineIntersection, Float,
    intersection_search, intersection_search2,
};

use float_extras::f64::nextafter;
use rand::Rng;


pub trait NextAfter {
    fn nextafter(self, up: bool) -> Self;
    fn nextafter_steps(self, steps: i32) -> Self;
}

impl NextAfter for f64 {
    fn nextafter(self, up: bool) -> Self {
        if up {
            nextafter(self, std::f64::INFINITY)
        } else {
            nextafter(self, std::f64::NEG_INFINITY)
        }
    }

    fn nextafter_steps(self, steps: i32) -> Self {
        let mut x = self;
        for _ in 0..steps.abs() {
            x = x.nextafter(steps > 0);
        }
        x
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


fn refinement_test() {
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


fn random_coord() -> Coordinate<f64> {
    let mut rng = rand::thread_rng();
    Coordinate{x: rng.gen_range(-1e9, 1e9), y: rng.gen_range(-1e9, 1e9)}
}


fn signed_area_precision_test() {
    let n = 1000;
    let mut i = 0;
    let mut found = 0;
    while found < 100 {
        let a = random_coord();
        let b = random_coord();
        let c = random_coord();
        let sa_exact = signed_area(a, b, c);
        let sa_fast = signed_area_fast(a, b, c);
        let diff = sa_fast - sa_exact;
        if diff != 0.0 {
            println!("{:?} {:?} {:?} {} {}", a, b, c, diff, i);
            found += 1;
        }
        i += 1;
    }
}


fn compare_signed_area(ax: f64, ay: f64, bx: f64, by: f64, cx: f64, cy: f64) {
    let a = Coordinate{x: ax, y: ay};
    let b = Coordinate{x: bx, y: by};
    let c = Coordinate{x: cx, y: cy};
    let sa_exact = signed_area(a, b, c);
    let sa_fast = signed_area_fast(a, b, c);
    let diff = sa_fast - sa_exact;
    println!("{} {} {} {:?} {:?} {:?}", diff, sa_fast, sa_exact, a, b, c);
}


fn main() {
    // ulp_test();

    //compare_signed_area(1000., 1000., -1000., -1000., 0., 0_f64.nextafter_steps(1000));
    //compare_signed_area(1000., 1000., 0., 1e-13, -1000., -1000.); // <= apparently it is possible to overestimate
    //compare_signed_area(0., 1e-13, 1000., 1000., -1000., -1000.);
    // signed_area_precision_test();

    /*
    let a1 = Coordinate { x: -98.0, y: 530.0 };
    let a2 = Coordinate { x: 530.0, y: 530.0 };
    let b1 = Coordinate { x: 1.250012525025, y: 531.0 };
    let b2 = Coordinate { x: 1.2500125250252, y: -531.0 };
    */

    let a1 = Coordinate { x: 1.51, y: 2.0 };
    let a2 = Coordinate { x: 1.51, y: 0.0 };
    let b1 = Coordinate { x: 1.0, y: 1.0 };
    let b2 = Coordinate { x: 2.0.nextafter_steps(-1), y: 1.0.nextafter_steps(3) };

    let inter = intersection_search2(a1, a2, b1, b2);
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

    fn divide(x: f64, n: usize) -> f64 {
        let mut x = x;
        for i in 0..n {
            x /= 2.0;
        }
        x
    }

    //let x = 1.0.nextafter(false);
    // println!("{}\n{}", x / 1024., divide(x, 10));

}