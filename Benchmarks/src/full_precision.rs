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


#[inline]
#[allow(dead_code)]
pub fn intersection_exact<F>(
    a1: Coordinate<F>,
    a2: Coordinate<F>,
    b1: Coordinate<F>,
    b2: Coordinate<F>,
) -> Option<Coordinate<F>>
where
    F: Float,
{
    // https://stackoverflow.com/a/14795484/1804173
    let a1x = Rational::from_f64(a1.x.into()).unwrap();
    let a1y = Rational::from_f64(a1.y.into()).unwrap();
    let a2x = Rational::from_f64(a2.x.into()).unwrap();
    let a2y = Rational::from_f64(a2.y.into()).unwrap();
    let b1x = Rational::from_f64(b1.x.into()).unwrap();
    let b1y = Rational::from_f64(b1.y.into()).unwrap();
    let b2x = Rational::from_f64(b2.x.into()).unwrap();
    let b2y = Rational::from_f64(b2.y.into()).unwrap();
    let zero = Rational::from(0);

    let ax = a2x.clone() - a1x.clone();
    let ay = a2y.clone() - a1y.clone();
    let bx = b2x.clone() - b1x.clone();
    let by = b2y.clone() - b1y.clone();

    let denom = (ax.clone() * by.clone()) - (bx.clone() * ay.clone());

    if denom == 0 {
        return None;
    }
    let denom_positive = denom > zero;

    let ba_x = a1x.clone() - b1x;
    let ba_y = a1y.clone() - b1y;

    let s = ax.clone() * &ba_y - ay.clone() * ba_x.clone();
    if (s < 0) == denom_positive {
        return None;
    }

    let t = bx * ba_y.clone() - by * ba_x.clone();
    if (t < 0) == denom_positive {
        return None;
    }

    if (s > denom) == denom_positive || (t > denom) == denom_positive {
        return None;
    }

    let t = t / denom;
    let i_x = a1x.clone() + (&t * ax);
    let i_y = a1y.clone() + (&t * ay);

    Some(Coordinate{
        x: F::from(i_x.to_f64()).unwrap(),
        y: F::from(i_y.to_f64()).unwrap(),
    })
}

