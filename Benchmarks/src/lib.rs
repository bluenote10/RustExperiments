use geo_types::Coordinate;
use robust::{Coord, orient2d};

mod helper;
mod full_precision;
pub mod rand_geo;

pub use helper::Float;
pub use helper::NextAfter;
pub use full_precision::{
    signed_area_exact, intersection_exact, analyze_grid,
};

// ----------------------------------------------------------------------------
// Original signed area
// ----------------------------------------------------------------------------

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
    //res *= -F::one()
    F::from(res).unwrap()
    /*
    if res > 0f64 {
        F::one()
    } else if res < 0f64 {
        -F::one()
    } else {
        F::zero()
    }
    */
}

#[inline]
pub fn signed_area_fast<F>(p0: Coordinate<F>, p1: Coordinate<F>, p2: Coordinate<F>) -> F
where
    F: Float,
{
    (p0.x - p2.x) * (p1.y - p2.y) - (p1.x - p2.x) * (p0.y - p2.y)
}

// ----------------------------------------------------------------------------
// Original intersection
// ----------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LineIntersection<F>
where
    F: Float,
{
    None,
    Point(Coordinate<F>),
    Overlap(Coordinate<F>, Coordinate<F>),
}

pub fn intersection<F>(
    a1: Coordinate<F>,
    a2: Coordinate<F>,
    b1: Coordinate<F>,
    b2: Coordinate<F>,
) -> LineIntersection<F>
where
    F: Float,
{
    let inter = intersection_impl(a1, a2, b1, b2);
    match inter {
        LineIntersection::Point(mut p) => {
            let a_min_x = a1.x.min(a2.x);
            let a_max_x = a1.x.max(a2.x);
            let a_min_y = a1.y.min(a2.y);
            let a_max_y = a1.y.max(a2.y);
            let b_min_x = b1.x.min(b2.x);
            let b_max_x = b1.x.max(b2.x);
            let b_min_y = b1.y.min(b2.y);
            let b_max_y = b1.y.max(b2.y);
            let min_x = a_min_x.max(b_min_x);
            let max_x = a_max_x.min(b_max_x);
            let min_y = a_min_y.max(b_min_y);
            let max_y = a_max_y.min(b_max_y);
            if p.x < min_x {
                p.x = min_x;
            }
            if p.x > max_x {
                p.x = max_x;
            }
            if p.y < min_y {
                p.y = min_y;
            }
            if p.y > max_y {
                p.y = max_y;
            }
            LineIntersection::Point(p)
        },
        _ => inter
    }
}

pub fn intersection_impl<F>(
    a1: Coordinate<F>,
    a2: Coordinate<F>,
    b1: Coordinate<F>,
    b2: Coordinate<F>,
) -> LineIntersection<F>
where
    F: Float,
{
    // println!("{:?} {:?} {:?} {:?}", a1, a2, b1, b2);
    let va = Coordinate {
        x: a2.x - a1.x,
        y: a2.y - a1.y,
    };
    let vb = Coordinate {
        x: b2.x - b1.x,
        y: b2.y - b1.y,
    };
    let e = Coordinate {
        x: b1.x - a1.x,
        y: b1.y - a1.y,
    };
    let mut kross = cross_product(va, vb);
    let mut sqr_kross = kross * kross;
    let sqr_len_a = dot_product(va, va);

    if sqr_kross > F::zero() {
        let s = cross_product(e, vb) / kross;
        if s < F::zero() || s > F::one() {
            return LineIntersection::None;
        }
        let t = cross_product(e, va) / kross;
        if t < F::zero() || t > F::one() {
            return LineIntersection::None;
        }

        if s == F::zero() || s == F::one() {
            return LineIntersection::Point(mid_point(a1, s, va));
        }
        if t == F::zero() || t == F::one() {
            return LineIntersection::Point(mid_point(b1, t, vb));
        }

        //println!("s = {:?} => {:?}", s, mid_point(a1, s, va));
        //return LineIntersection::Point(mid_point(b1, t, vb));
        return LineIntersection::Point(mid_point(a1, s, va));
    }

    kross = cross_product(e, va);
    sqr_kross = kross * kross;

    if sqr_kross > F::zero() {
        return LineIntersection::None;
    }

    let sa = dot_product(va, e) / sqr_len_a;
    let sb = sa + dot_product(va, vb) / sqr_len_a;
    let smin = sa.min(sb);
    let smax = sa.max(sb);

    if smin <= F::one() && smax >= F::zero() {
        if smin == F::one() {
            return LineIntersection::Point(mid_point(a1, smin, va));
        }
        if smax == F::zero() {
            return LineIntersection::Point(mid_point(a1, smax, va));
        }

        return LineIntersection::Overlap(
            mid_point(a1, smin.max(F::zero()), va),
            mid_point(a1, smax.min(F::one()), va),
        );
    }

    LineIntersection::None
}


pub fn intersection_new<F>(
    a1: Coordinate<F>,
    a2: Coordinate<F>,
    b1: Coordinate<F>,
    b2: Coordinate<F>,
) -> LineIntersection<F>
where
    F: Float,
{
    // println!("{:?} {:?} {:?} {:?}", a1, a2, b1, b2);
    let va = Coordinate {
        x: a2.x - a1.x,
        y: a2.y - a1.y,
    };
    let vb = Coordinate {
        x: b2.x - b1.x,
        y: b2.y - b1.y,
    };
    let e = Coordinate {
        x: b1.x - a1.x,
        y: b1.y - a1.y,
    };
    let denom = cross_product(va, vb);

    if denom.abs() > F::zero() {
        let s = cross_product(e, vb) / denom;
        if s < F::zero() || s > F::one() {
            return LineIntersection::None;
        }
        let t = cross_product(e, va) / denom;
        if t < F::zero() || t > F::one() {
            return LineIntersection::None;
        }

        if s == F::zero() || s == F::one() {
            return LineIntersection::Point(mid_point(a1, s, va));
        }
        if t == F::zero() || t == F::one() {
            return LineIntersection::Point(mid_point(b1, t, vb));
        }

        let len_va = va.x.abs().max(va.y.abs());
        let len_vb = vb.x.abs().max(vb.y.abs());
        if len_va < len_vb {
            return LineIntersection::Point(mid_point(a1, s, va));
        } else {
            return LineIntersection::Point(mid_point(b1, t, vb));
        }
    }

    let sqr_len_a = dot_product(va, va);
    let kross = cross_product(e, va);
    let sqr_kross = kross * kross;

    if sqr_kross > F::zero() {
        return LineIntersection::None;
    }

    let sa = dot_product(va, e) / sqr_len_a;
    let sb = sa + dot_product(va, vb) / sqr_len_a;
    let smin = sa.min(sb);
    let smax = sa.max(sb);

    if smin <= F::one() && smax >= F::zero() {
        if smin == F::one() {
            return LineIntersection::Point(mid_point(a1, smin, va));
        }
        if smax == F::zero() {
            return LineIntersection::Point(mid_point(a1, smax, va));
        }

        return LineIntersection::Overlap(
            mid_point(a1, smin.max(F::zero()), va),
            mid_point(a1, smax.min(F::one()), va),
        );
    }

    LineIntersection::None
}


fn mid_point<F>(p: Coordinate<F>, s: F, d: Coordinate<F>) -> Coordinate<F>
where
    F: Float,
{
    let result = Coordinate {
        x: p.x + s * d.x,
        y: p.y + s * d.y,
    };
    //println!("{:?} {:?} {:?} {:?}", p, s, d, result);
    result
}

#[inline]
fn cross_product<F>(a: Coordinate<F>, b: Coordinate<F>) -> F
where
    F: Float,
{
    a.x * b.y - a.y * b.x
}

#[inline]
fn dot_product<F>(a: Coordinate<F>, b: Coordinate<F>) -> F
where
    F: Float,
{
    a.x * b.x + a.y * b.y
}

// ----------------------------------------------------------------------------
// Intersection binary search
// ----------------------------------------------------------------------------

/// First implementation of intersection binary search using segment division.
/// The problem with this approach is that subdividing the segments can accumulate
/// rounding errors in the midpoint calculation, i.e., after repeatedly taking
/// the midpoint, we end up on a segment that no longer lies on the original
/// segment.
pub fn intersection_search_segment_divide<F>(
    a1: Coordinate<F>,
    a2: Coordinate<F>,
    b1: Coordinate<F>,
    b2: Coordinate<F>,
) -> LineIntersection<F>
where
    F: Float,
{
    let sa1 = signed_area(a1, a2, b1);
    let sa2 = signed_area(a1, a2, b2);
    if (sa1 > F::zero() && sa2 > F::zero()) || (sa1 < F::zero() && sa2 < F::zero()) {
        return LineIntersection::None;
    }

    let (mut b_neg, mut sa_neg, mut b_pos, mut sa_pos) = if sa1 < sa2 {
        (b1, sa1, b2, sa2)
    } else {
        (b2, sa2, b1, sa1)
    };

    let two = F::from(2.0).unwrap();
    let mut i = 0;

    loop {

        let mid = Coordinate{x: (b_neg.x + b_pos.x) / two, y: (b_neg.y + b_pos.y) / two};
        let sa_mid = signed_area(a1, a2, mid);

        // println!("{:?} {:?} {:?} {}", b_neg, mid, b_pos, sa_mid);

        if sa_mid < F::zero() {
            if b_neg == mid {
                break;
            }
            b_neg = mid;
            sa_neg = sa_mid;
        } else {
            if b_pos == mid {
                break;
            }
            b_pos = mid;
            sa_pos = sa_mid;
        }

        i += 1;
    }

    // println!("{} {} {}", i, sa_neg, sa_pos);

    if -sa_neg < sa_pos {
        LineIntersection::Point(Coordinate{x: b_neg.x, y: b_neg.y})
    } else {
        LineIntersection::Point(Coordinate{x: b_pos.x, y: b_pos.y})
    }
}


/// Improved binary intersection search, which avoids the midpoint rounding error,
/// by repeatedly reprojecting the midpoint using the segment formula.
pub fn intersection_search<F>(
    a1: Coordinate<F>,
    a2: Coordinate<F>,
    b1: Coordinate<F>,
    b2: Coordinate<F>,
) -> LineIntersection<F>
where
    F: Float,
{
    let sa1 = signed_area(a1, a2, b1);
    let sa2 = signed_area(a1, a2, b2);
    if (sa1 > F::zero() && sa2 > F::zero()) || (sa1 < F::zero() && sa2 < F::zero()) {
        return LineIntersection::None;
    }

    let (sa_l, sa_r) = if b1.x < b2.x {
        (signed_area(a1, a2, b1), signed_area(a1, a2, b2))
    } else {
        (signed_area(a1, a2, b2), signed_area(a1, a2, b1))
    };

    let vb = Coordinate{x: b2.x - b1.x, y: b2.y - b1.y};

    let two = F::from(2.0).unwrap();
    let mut i = 0;

    if vb.x.abs() > vb.y.abs() {
        // This would search the entire b x-range, but we can actually limit to overlap.
        // let mut x_min = b1.x.min(b2.x);
        // let mut x_max = b1.x.max(b2.x);
        let a_min_x = a1.x.min(a2.x);
        let a_max_x = a1.x.max(a2.x);
        let b_min_x = b1.x.min(b2.x);
        let b_max_x = b1.x.max(b2.x);
        let mut x_min = a_min_x.max(b_min_x);
        let mut x_max = a_max_x.min(b_max_x);

        let mut sa_l_cur = sa_l;
        let mut sa_r_cur = sa_r;

        loop {
            let x_mid = (x_min + x_max) / two;
            let y_mid = get_y(b1, vb, x_mid);
            if x_mid == x_min || x_mid == x_max {
                break;
            }

            let mid = Coordinate{x: x_mid, y: y_mid};
            let sa_mid = signed_area(a1, a2, mid);

            // println!("{:?} {} {} {}", mid, x_min, x_max, sa_mid);
            if (sa_mid < F::zero()) == (sa_l < F::zero()) {
                x_min = x_mid;
                sa_l_cur = sa_mid;
            } else {
                x_max = x_mid;
                sa_r_cur = sa_mid;
            }

            i += 1;
        }
        // println!("{} {} {}", i, sa_l_cur, sa_r_cur);

        if -sa_l_cur < sa_r_cur {
            return LineIntersection::Point(Coordinate{x: x_min, y: get_y(b1, vb, x_min)})
        } else {
            return LineIntersection::Point(Coordinate{x: x_max, y: get_y(b1, vb, x_max)})
        }

    }

    LineIntersection::None
}

pub fn get_y<F>(
    p: Coordinate<F>,
    d: Coordinate<F>,
    x: F,
) -> F
where
    F: Float,
{
    let t = (x - p.x) / d.x;
    p.y + t * d.y
}

// ----------------------------------------------------------------------------
// Iterative refinement
// ----------------------------------------------------------------------------

/*
pub fn refine_intersection<F, S>(
    a1: Coordinate<F>,
    a2: Coordinate<F>,
    b1: Coordinate<F>,
    b2: Coordinate<F>,
    i: Coordinate<F>,
    score: S,
) -> (u64, Coordinate<F>)
where
    F: Float,
    S: FnMut(Coordinate<F>, Coordinate<F>, Coordinate<F>, Coordinate<F>, Coordinate<F>) -> F,
{
    let mut p00 = Coordinate{x: i.x.nextafter(false), y: i.y.nextafter(false)};
    let mut p01 = Coordinate{x: i.x.nextafter(false), y: i.y};
    let mut p02 = Coordinate{x: i.x.nextafter(false), y: i.y.nextafter(true)};
    let mut p10 = Coordinate{x: i.x, y: i.y.nextafter(false)};
    let mut p11 = Coordinate{x: i.x, y: i.y};
    let mut p12 = Coordinate{x: i.x, y: i.y.nextafter(true)};
    let mut p20 = Coordinate{x: i.x.nextafter(true), y: i.y.nextafter(false)};
    let mut p21 = Coordinate{x: i.x.nextafter(true), y: i.y};
    let mut p22 = Coordinate{x: i.x.nextafter(true), y: i.y.nextafter(true)};

    //let mut cur_score = score(a1, a2, b1, b2, i);

    let mut scores = [
        score(a1, a2, b1, b2, p00),
        score(a1, a2, b1, b2, p01),
        score(a1, a2, b1, b2, p02),
        score(a1, a2, b1, b2, p10),
        score(a1, a2, b1, b2, p11),
        score(a1, a2, b1, b2, p12),
        score(a1, a2, b1, b2, p20),
        score(a1, a2, b1, b2, p21),
        score(a1, a2, b1, b2, p22),
    ];

    let mut iterations = 0;

    loop {
        let index_best = scores
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(index, _)| index)
            .unwrap();

        if index_best == 4 {
            return (iterations, p11);
        } else if index_best == 0 {
            let x = p00.x.nextafter(false);
            let y = p00.y.nextafter(false);
            p00.x = x;

        }
    }

    let mut neighbors = [p00, p01, p02, p10, p12, p20, p21, p22];
    let mut min = cur_score;
    let mut best_idx = -1;

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

    unimplemented!()
}
*/

// ----------------------------------------------------------------------------
// Orig tests
// ----------------------------------------------------------------------------

#[cfg(test)]
mod test {
    use super::*;

    pub fn xy<X: Into<f64>, Y: Into<f64>>(x: X, y: Y) -> Coordinate<f64> {
        Coordinate {
            x: x.into(),
            y: y.into(),
        }
    }

    #[test]
    fn test_intersection() {
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(1, 0), xy(2, 2)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(1, 0), xy(10, 2)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(2, 2), xy(3, 3), xy(0, 6), xy(2, 4)),
            LineIntersection::None
        );

        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(1, 0), xy(0, 1)),
            LineIntersection::Point(xy(0.5, 0.5))
        );

        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(0, 1), xy(0, 0)),
            LineIntersection::Point(xy(0, 0))
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(0, 1), xy(1, 1)),
            LineIntersection::Point(xy(1, 1))
        );

        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(0.5, 0.5), xy(1, 0)),
            LineIntersection::Point(xy(0.5, 0.5))
        );

        assert_eq!(
            intersection(xy(0, 0), xy(10, 10), xy(1, 1), xy(5, 5)),
            LineIntersection::Overlap(xy(1, 1), xy(5, 5))
        );
        assert_eq!(
            intersection(xy(1, 1), xy(10, 10), xy(1, 1), xy(5, 5)),
            LineIntersection::Overlap(xy(1, 1), xy(5, 5))
        );
        assert_eq!(
            intersection(xy(3, 3), xy(10, 10), xy(0, 0), xy(5, 5)),
            LineIntersection::Overlap(xy(3, 3), xy(5, 5))
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(0, 0), xy(1, 1)),
            LineIntersection::Overlap(xy(0, 0), xy(1, 1))
        );
        assert_eq!(
            intersection(xy(1, 1), xy(0, 0), xy(0, 0), xy(1, 1)),
            LineIntersection::Overlap(xy(1, 1), xy(0, 0))
        );

        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(1, 1), xy(2, 2)),
            LineIntersection::Point(xy(1, 1))
        );
        assert_eq!(
            intersection(xy(1, 1), xy(0, 0), xy(1, 1), xy(2, 2)),
            LineIntersection::Point(xy(1, 1))
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(2, 2), xy(4, 4)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(0, 0), xy(1, 1), xy(0, -1), xy(1, 0)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(1, 1), xy(0, 0), xy(0, -1), xy(1, 0)),
            LineIntersection::None
        );
        assert_eq!(
            intersection(xy(0, -1), xy(1, 0), xy(0, 0), xy(1, 1)),
            LineIntersection::None
        );

        assert_eq!(
            intersection(xy(0, 0.5), xy(1, 1.5), xy(0, 1), xy(1, 0)),
            LineIntersection::Point(xy(0.25, 0.75))
        );
    }
}

