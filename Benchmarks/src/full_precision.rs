use geo_types::Coordinate;
use super::helper::Float;

use rug::Rational;


#[inline]
pub fn signed_area_exact<F>(p0: Coordinate<F>, p1: Coordinate<F>, p2: Coordinate<F>) -> F
where
    F: Float,
{
    let p0x = Rational::from_f64(p0.x.into()).unwrap();
    let p0y = Rational::from_f64(p0.y.into()).unwrap();
    let p1x = Rational::from_f64(p1.x.into()).unwrap();
    let p1y = Rational::from_f64(p1.y.into()).unwrap();
    let p2x = Rational::from_f64(p2.x.into()).unwrap();
    let p2y = Rational::from_f64(p2.y.into()).unwrap();
    let result = (p0x - &p2x) * (p1y - &p2y) - (p1x - &p2x) * (p0y - &p2y);
    F::from(result.to_f64()).unwrap()
}
