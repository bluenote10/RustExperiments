use geo_types::Coordinate;
use super::helper::Float;
use super::helper::NextAfter;

use rug::Rational;


#[inline]
pub fn signed_area_exact_impl<F>(p0: Coordinate<F>, p1: Coordinate<F>, p2: Coordinate<F>) -> Rational
where
    F: Float,
{
    let p0x = Rational::from_f64(p0.x.into()).unwrap();
    let p0y = Rational::from_f64(p0.y.into()).unwrap();
    let p1x = Rational::from_f64(p1.x.into()).unwrap();
    let p1y = Rational::from_f64(p1.y.into()).unwrap();
    let p2x = Rational::from_f64(p2.x.into()).unwrap();
    let p2y = Rational::from_f64(p2.y.into()).unwrap();
    (p0x - &p2x) * (p1y - &p2y) - (p1x - &p2x) * (p0y - &p2y)
}


#[inline]
pub fn get_length_squared<F>(a: Coordinate<F>, b: Coordinate<F>) -> Rational
where
    F: Float,
{
    let ax = Rational::from_f64(a.x.into()).unwrap();
    let ay = Rational::from_f64(a.y.into()).unwrap();
    let bx = Rational::from_f64(b.x.into()).unwrap();
    let by = Rational::from_f64(b.y.into()).unwrap();
    let delta_x_sqr = (ax.clone() - bx.clone()) * (ax.clone() - bx.clone());
    let delta_y_sqr = (ay.clone() - by.clone()) * (ay.clone() - by.clone());
    delta_x_sqr + delta_y_sqr
}


#[inline]
pub fn signed_area_exact<F>(p0: Coordinate<F>, p1: Coordinate<F>, p2: Coordinate<F>) -> F
where
    F: Float,
{
    let result = signed_area_exact_impl(p0, p1, p2);
    F::from(result.to_f64()).unwrap()
}


pub fn analyze_grid<C>(
    a1: Coordinate<f64>,
    a2: Coordinate<f64>,
    b1: Coordinate<f64>,
    b2: Coordinate<f64>,
    center: Coordinate<f64>,
    delta: i32,
    mut cb: C,
)
where
    C: FnMut(i32, i32, Coordinate<f64>, f64)
{
    for i in -delta ..= delta {
        for j in -delta ..= delta {
            let p = Coordinate{x: center.x.nextafter_steps(i), y: center.y.nextafter_steps(j)};
            let perp_a = signed_area_exact_impl(a1, a2, p);
            let perp_b = signed_area_exact_impl(b1, b2, p);
            let length_squared_a = get_length_squared(a1, a2);
            let length_squared_b = get_length_squared(b1, b2);
            let dist_squared_a = perp_a.clone() * perp_a.clone() / length_squared_a;
            let dist_squared_b = perp_b.clone() * perp_b.clone() / length_squared_b;
            // let sum_dist_squared = (dist_squared_a + dist_squared_b).to_f64();
            let sum_dist = dist_squared_a.to_f64().sqrt() + dist_squared_b.to_f64().sqrt();
            cb(i, j, p, sum_dist);
        }
    }
}


#[inline]
fn convert_to_f64(r: &Rational) -> f64 {
    let x = r.to_f64();
    let x_lo = x.nextafter_steps(-1);
    let x_hi = x.nextafter_steps(1);
    let delta_lo = (Rational::from_f64(x_lo).unwrap() - r).abs().to_f64();
    let delta_mid = (Rational::from_f64(x).unwrap() - r).abs().to_f64();
    let delta_hi = (Rational::from_f64(x_hi).unwrap() - r).abs().to_f64();

    if delta_lo < delta_mid && delta_lo < delta_mid {
        x_lo
    } else if delta_hi < delta_mid && delta_hi < delta_mid {
        x_hi
    } else {
        x
    }
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
        x: F::from(convert_to_f64(&i_x)).unwrap(),
        y: F::from(convert_to_f64(&i_y)).unwrap(),
    })
}

