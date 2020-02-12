#![allow(dead_code)]
extern crate rand;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use geo_types::Coordinate;

mod utils;
use utils::{bench_function_with_noop, iter_noop_batched};

use mycrate::{
    signed_area, signed_area_alt, signed_area_fast, signed_area_exact,
    intersection, LineIntersection,
    rand_geo, robust_alt,
};


#[inline]
fn noop<T>(x: T) -> T {
    x
}


fn bench_signed_area(c: &mut Criterion) {
    c.bench_function("orient2d (const)", |b| b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            || black_box(1.0),
            || robust_alt::orient2d(
                1.0, 2.0,
                3.0, 4.0,
                5.0, 6.0,
            ),
        )
    }));
    c.bench_function("signed_area (const)", |b| b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            || black_box(1.0),
            || signed_area(
                Coordinate::from(black_box((1.0, 2.0))),
                Coordinate::from(black_box((3.0, 4.0))),
                Coordinate::from(black_box((5.0, 6.0))),
            ),
        )
    }));
    c.bench_function("signed_area_alt (const)", |b| b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            || black_box(1.0),
            || signed_area_alt(
                Coordinate::from(black_box((1.0, 2.0))),
                Coordinate::from(black_box((3.0, 4.0))),
                Coordinate::from(black_box((5.0, 6.0))),
            ),
        )
    }));
    c.bench_function("signed_area_fast (const)", |b| b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            || black_box(1.0),
            || signed_area_fast(
                Coordinate::from(black_box((1.0, 2.0))),
                Coordinate::from(black_box((3.0, 4.0))),
                Coordinate::from(black_box((5.0, 6.0))),
            ),
        )
    }));
    c.bench_function("signed_area_exact (const)", |b| b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            || black_box(1.0),
            || signed_area_exact(
                Coordinate::from(black_box((1.0, 2.0))),
                Coordinate::from(black_box((3.0, 4.0))),
                Coordinate::from(black_box((5.0, 6.0))),
            ),
        )
    }));

    c.bench_function("orient2d (rand)", |b| iter_noop_batched(b,
        |_| rand_geo::three_points(),
        |(_, _, _)| 0.0_f64,
        |(a, b, c)| robust_alt::orient2d(a.x, a.y, b.x, b.y, c.x, c.y),
    ));
    c.bench_function("signed_area (rand)", |b| iter_noop_batched(b,
        |_| rand_geo::three_points(),
        |(_, _, _)| 0.0_f64,
        |(a, b, c)| signed_area(a, b, c),
    ));
    c.bench_function("signed_area_alt (rand)", |b| iter_noop_batched(b,
        |_| rand_geo::three_points(),
        |(_, _, _)| 0.0_f64,
        |(a, b, c)| signed_area_alt(a, b, c),
    ));
    c.bench_function("signed_area_fast (rand)", |b| iter_noop_batched(b,
        |_| rand_geo::three_points(),
        |(_, _, _)| 0.0_f64,
        |(a, b, c)| signed_area_fast(a, b, c),
    ));
    c.bench_function("signed_area_exact (rand)", |b| iter_noop_batched(b,
        |_| rand_geo::three_points(),
        |(_, _, _)| 0.0_f64,
        |(a, b, c)| signed_area_exact(a, b, c),
    ));

    c.bench_function("orient2d (rand almost colinear)", |b| iter_noop_batched(b,
        |_| rand_geo::three_points_almost_colinear(),
        |(_, _, _)| 0.0_f64,
        |(a, b, c)| robust_alt::orient2d(a.x, a.y, b.x, b.y, c.x, c.y),
    ));
    c.bench_function("signed_area (rand almost colinear)", |b| iter_noop_batched(b,
        |_| rand_geo::three_points_almost_colinear(),
        |(_, _, _)| 0.0_f64,
        |(a, b, c)| signed_area(a, b, c),
    ));
    c.bench_function("signed_area_alt (rand almost colinear)", |b| iter_noop_batched(b,
        |_| rand_geo::three_points_almost_colinear(),
        |(_, _, _)| 0.0_f64,
        |(a, b, c)| signed_area_alt(a, b, c),
    ));
    c.bench_function("signed_area_fast (rand almost colinear)", |b| iter_noop_batched(b,
        |_| rand_geo::three_points_almost_colinear(),
        |(_, _, _)| 0.0_f64,
        |(a, b, c)| signed_area_fast(a, b, c),
    ));
    c.bench_function("signed_area_exact (rand almost colinear)", |b| iter_noop_batched(b,
        |_| rand_geo::three_points_almost_colinear(),
        |(_, _, _)| 0.0_f64,
        |(a, b, c)| signed_area_exact(a, b, c),
    ));
}


fn bench_intersection(c: &mut Criterion) {
    c.bench_function("intersection", |b| b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            || LineIntersection::Point(Coordinate::from(black_box((1.0, 2.0)))),
            || intersection(
                Coordinate::from(black_box((1.0, 2.0))),
                Coordinate::from(black_box((3.0, 4.0))),
                Coordinate::from(black_box((5.0, 6.0))),
                Coordinate::from(black_box((7.0, 8.0))),
            ),
        )
    }));
}


criterion_group!(
    benches,
    //basics,
    bench_signed_area,
    //bench_intersection,
);
criterion_main!(benches);
