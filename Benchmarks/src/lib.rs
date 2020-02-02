use geo_types::Coordinate;
use robust::{Coord, orient2d};

use std::fmt::{Debug, Display};
use num_traits::Float as NumTraitsFloat;

pub trait Float: NumTraitsFloat + Debug + Display {}
impl<T: NumTraitsFloat + Debug + Display> Float for T {}


#[inline]
pub fn coordinate_to_robust<F>(p : Coordinate<F>) -> Coord
where
    F: Float,
{
    Coord{x: p.x.to_f64().unwrap(), y: p.y.to_f64().unwrap()}
}


#[inline]
pub fn signed_area<F>(p0: Coordinate<F>, p1: Coordinate<F>, p2: Coordinate<F>) -> F
where
    F: Float,
{
    let res = orient2d(
        coordinate_to_robust(p0),
        coordinate_to_robust(p1),
        coordinate_to_robust(p2),
    );
    if res > 0f64 {
        F::one()
    } else if res < 0f64 {
        -F::one()
    } else {
        F::zero()
    }
}

#[inline]
pub fn signed_area_fast<F>(p0: Coordinate<F>, p1: Coordinate<F>, p2: Coordinate<F>) -> F
where
    F: Float,
{
    (p0.x - p2.x) * (p1.y - p2.y) - (p1.x - p2.x) * (p0.y - p2.y)
}
