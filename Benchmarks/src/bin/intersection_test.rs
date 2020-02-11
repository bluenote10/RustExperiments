extern crate mycrate;
extern crate rand;

use std::fs::File;
use geo_types::Coordinate;
use serde_json::{Value, json};

use mycrate::{
    intersection_impl, signed_area, signed_area_fast, signed_area_exact,
    LineIntersection, Float,
    intersection, intersection_search, intersection_exact, intersection_new,
    analyze_grid,
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
    let mut records: Vec<Value> = Vec::new();
    let n = 2000;
    let mut i = 0;
    while i < n {
        let (a, b, c) = rand_geo::three_points_almost_colinear();
        let sa_exact = signed_area_exact(a, b, c);
        let sa_robust = signed_area(a, b, c);
        let sa_fast = signed_area_fast(a, b, c);
        let diff = sa_fast - sa_exact;
        if diff != 0.0 {
            //println!("{} {} {}", sa_exact, sa_robust, sa_fast);
            println!("{:?} {:?} {:?} {} {}", a, b, c, diff, i);
        }
        records.push(json!({
            "sa_exact": sa_exact,
            "sa_robust": sa_robust,
            "sa_fast": sa_fast,
        }));
        i += 1;
    }
    let f = File::create("sa_data.json").expect("Unable to create json file.");
    serde_json::to_writer_pretty(f, &records).expect("Unable to write json file.");
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


fn get_ulp_distance(a: Coordinate<f64>, b: Coordinate<f64>) -> (i64, i64) {
    let mut delta_x = 0;
    let mut delta_y = 0;
    let mut x = a.x;
    let mut y = a.y;
    while x != b.x {
        x = x.nextafter(b.x > a.x);
        delta_x += if b.x > a.x { 1 } else { -1 };
    }
    while y != b.y {
        y = y.nextafter(b.y > a.y);
        delta_y += if b.y > a.y { 1 } else { -1 };
    }
    (delta_x, delta_y)
}


fn intersection_comparison() {
    let n = 1000;
    let with_grid = true;
    let grid_size = 5;

    let mut records: Vec<Value> = Vec::new();
    let mut i = 0;
    while i < n {
        let (a1, a2, b1, b2) = rand_geo::intersecting_segments();
        // println!("{:?} {:?} {:?} {:?}", a1, a2, b1, b2);
        let i_fast = intersection(a1, a2, b1, b2);
        let i_exact = intersection_exact(a1, a2, b1, b2);
        let i_new = intersection_new(a1, a2, b1, b2);
        // let i_search = intersection_search(a1, a2, b1, b2);

        let i_fast = match i_fast {
            LineIntersection::Point(p) => Some(p),
            _ => None,
        };
        let i_new = match i_new {
            LineIntersection::Point(p) => Some(p),
            _ => None,
        };
        /*
        let i_search = match i_search {
            LineIntersection::Point(p) => Some(p),
            _ => None,
        };
        */
        if i_fast.is_none() || i_new.is_none() || i_exact.is_none() {
            println!("WARNING: Skipping iterations because a result was missing:");
            println!("{:?} {:?}", i_fast, i_exact);
            continue;
        }
        let i_fast = i_fast.unwrap();
        let i_new = i_new.unwrap();
        //let i_search = i_search.unwrap();
        let i_exact = i_exact.unwrap();
        // println!("{:?} {:?} {:?} {:?}", a1, a2, b1, b2);
        // println!("{:?} {:?} {:?}", i_fast, i_search, i_exact);

        let mut grid: Vec<Value> = Vec::new();
        let mut i_min = i_exact;
        let mut dist_min = std::f64::MAX;
        analyze_grid(a1, a2, b1, b2, i_exact, grid_size, |i, j, p, dist| {
            grid.push(json!({
                "i": i,
                "j": j,
                "dist": dist,
            }));
            if dist < dist_min {
                dist_min = dist;
                i_min = p;
            }
        });

        records.push(json!({
            "a1": [a1.x, a1.y],
            "a2": [a2.x, a2.y],
            "b1": [b1.x, b1.y],
            "b2": [b2.x, b2.y],
            "i_fast": {
                "p": [i_fast.x, i_fast.y],
                "ulp_dist": get_ulp_distance(i_exact, i_fast),
            },
            "i_new": {
                "p": [i_new.x, i_new.y],
                "ulp_dist": get_ulp_distance(i_exact, i_new),
            },
            /*
            "i_search": {
                "p": [i_search.x, i_search.y],
                "ulp_dist": get_ulp_distance(i_exact, i_search),
            },
            */
            "i_exact": {
                "p": [i_exact.x, i_exact.y],
                "ulp_dist": get_ulp_distance(i_exact, i_exact),
            },
            "i_min": {
                "p": [i_min.x, i_min.y],
                "ulp_dist": get_ulp_distance(i_exact, i_min),
            },
            "grid": if with_grid { json!(grid) } else { Value::Null },
        }));
        i += 1;
    }
    let f = File::create("intersection_data.json").expect("Unable to create json file.");
    serde_json::to_writer_pretty(f, &records).expect("Unable to write json file.");
}


fn main() {
    // ulp_test();
    // signed_area_precision_test();
    // intersection_search_test()
    intersection_comparison();
}