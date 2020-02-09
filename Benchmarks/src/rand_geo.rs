use super::helper::NextAfter;

use rand::Rng;
use geo_types::Coordinate;

#[inline]
pub fn rand_default_range() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(-1e3_f64, 1e3_f64)
}


pub fn three_points() -> (Coordinate<f64>, Coordinate<f64>, Coordinate<f64>) {
    let a = Coordinate{x: rand_default_range(), y: rand_default_range()};
    let b = Coordinate{x: rand_default_range(), y: rand_default_range()};
    let c = Coordinate{x: rand_default_range(), y: rand_default_range()};
    (a, b, c)
}

pub fn three_points_approx_colinear() -> (Coordinate<f64>, Coordinate<f64>, Coordinate<f64>) {
    let mut rng = rand::thread_rng();

    let a = Coordinate{x: rand_default_range(), y: rand_default_range()};
    let b = Coordinate{x: rand_default_range(), y: rand_default_range()};

    let delta_x = b.x - a.x;
    let delta_y = b.y - a.y;

    let kind = rng.gen_range(0, 3);
    let s = match kind {
        0 => rng.gen_range(0_f64, 1_f64),
        1 => rng.gen_range(1e2_f64, 1e6_f64),
        2 => rng.gen_range(1e2_f64, 1e6_f64),
        _ => panic!("Invalid value"),
    };

    let c = Coordinate{x: a.x + s * delta_x, y: a.y + s * delta_y};

    random_permutation(a, b, c)
}


pub fn three_points_almost_colinear() -> (Coordinate<f64>, Coordinate<f64>, Coordinate<f64>) {
    let mut rng = rand::thread_rng();

    let (a, b, c) = three_points_approx_colinear();

    let delta_ulp_x = rng.gen_range(-50, 50);
    let delta_ulp_y = rng.gen_range(-50, 50);

    let x = a.x.nextafter_steps(delta_ulp_x);
    let y = a.y.nextafter_steps(delta_ulp_y);

    (Coordinate{x: x, y: y}, b, c)
}


pub fn random_permutation(a: Coordinate<f64>, b: Coordinate<f64>, c: Coordinate<f64>) -> (Coordinate<f64>, Coordinate<f64>, Coordinate<f64>) {
    let mut rng = rand::thread_rng();
    let kind = rng.gen_range(0, 6);
    match kind {
        0 => (a, b, c),
        1 => (a, c, b),
        2 => (b, a, c),
        3 => (b, c, a),
        4 => (c, a, b),
        5 => (c, b, a),
        _ => panic!("Invalid value"),
    }
}
