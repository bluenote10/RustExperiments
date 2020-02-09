extern crate mycrate;
extern crate rand;

use geo_types::Coordinate;

use mycrate::{
    intersection_impl, signed_area, signed_area_fast, signed_area_exact,
    LineIntersection, Float,
    intersection_search,
    NextAfter,
    rand_geo,
};


pub fn refine<F>(
  a1: Coordinate<F>,
  a2: Coordinate<F>,
  b1: Coordinate<F>,
  b2: Coordinate<F>,
  i: Coordinate<F>,
  cur_score: F,
) -> Coordinate<F>
where
  F: Float,
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
    println!("{}", (1.0).ulp());
    println!("{}", (1.0.nextafter(false)).ulp());
    let mut x = 2.0 - 1e-10;
    let mut ulp = x.ulp();
    while x < 2.1 {
        let next_x = x.nextafter(true);
        let next_ulp = next_x.ulp();
        if next_ulp != ulp {
            println!("{} {}", next_x, next_ulp);
            ulp = next_ulp;
        }
        //println!("{} {}", x, ulp);
        x = next_x;
    }
}


fn signed_area_precision_test() {
    let n = 1000;
    let mut i = 0;
    let mut found = 0;
    while found < 100 {
        let (a, b, c) = rand_geo::three_points_almost_colinear();
        let sa_exact = signed_area_exact(a, b, c);
        let sa_robust = signed_area(a, b, c);
        let sa_fast = signed_area_fast(a, b, c);
        let diff = sa_fast - sa_exact;
        if diff != 0.0 {
            //println!("{} {} {}", sa_exact, sa_robust, sa_fast);
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

fn check_signed_area_error() {
    // This was a quick check if the approximate version of signed error only
    // has a problem with artificially outputting zero when it shouldn't.
    // But apparently, it is also possible that its output overestimates
    // the value by the robust implementation.
    compare_signed_area(1000., 1000., -1000., -1000., 0., 0_f64.nextafter_steps(1000));
    compare_signed_area(1000., 1000., 0., 1e-13, -1000., -1000.); // overestimates!
    compare_signed_area(0., 1e-13, 1000., 1000., -1000., -1000.);
}


fn intersection_search_test() {
    /*
    let a1 = Coordinate { x: -98.0, y: 530.0 };
    let a2 = Coordinate { x: 530.0, y: 530.0 };
    let b1 = Coordinate { x: 1.250012525025, y: 531.0 };
    let b2 = Coordinate { x: 1.2500125250252, y: -531.0 };
    */

    // This test case let the first implementation (based on repeated
    // midpoint computation) fail due to roundoff errors.
    let a1 = Coordinate { x: 1.51, y: 2.0 };
    let a2 = Coordinate { x: 1.51, y: 0.0 };
    let b1 = Coordinate { x: 1.0, y: 1.0 };
    let b2 = Coordinate { x: 2.0.nextafter_steps(-1), y: 1.0.nextafter_steps(3) };

    let inter = intersection_search(a1, a2, b1, b2);
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


fn main() {
    // ulp_test();
    signed_area_precision_test();
    // intersection_search_test()
}