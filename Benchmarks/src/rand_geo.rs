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