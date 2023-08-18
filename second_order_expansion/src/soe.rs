// Copyright 2017 The Spade Developers.
// Copyright 2020 The GeoRust Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// These values are precomputed from the "exactinit" method of the c-source code. They should? be
// the same in all IEEE-754 environments, including rust f64
const SPLITTER: f64 = 134_217_729f64;

fn scale_expansion_zeroelim(e: &[f64], b: f64, h: &mut [f64]) -> usize {
    let (bhi, blo) = split(b);
    let (mut q, hh) = two_product_presplit(e[0], b, bhi, blo);
    let mut hindex = 0;
    if hh != 0.0 {
        h[hindex] = hh;
        hindex += 1;
    }
    for eindex in 1..e.len() {
        let enow = e[eindex];
        let (product1, product0) = two_product_presplit(enow, b, bhi, blo);
        let (sum, hh) = two_sum(q, product0);
        if hh != 0.0 {
            h[hindex] = hh;
            hindex += 1;
        }
        let (new_q, hh) = fast_two_sum(product1, sum);
        q = new_q;
        if hh != 0.0 {
            h[hindex] = hh;
            hindex += 1;
        }
    }
    if q != 0.0 || hindex == 0 {
        h[hindex] = q;
        hindex += 1;
    }
    hindex
}

#[inline]
fn two_product(a: f64, b: f64) -> (f64, f64) {
    let x = a * b;
    (x, two_product_tail(a, b, x))
}

#[inline]
fn two_product_tail(a: f64, b: f64, x: f64) -> f64 {
    let (ahi, alo) = split(a);
    let (bhi, blo) = split(b);
    let err1 = x - (ahi * bhi);
    let err2 = err1 - (alo * bhi);
    let err3 = err2 - (ahi * blo);
    // println!("a = {} b = {} x = {}", a, b, x);
    (alo * blo) - err3
}

#[inline]
fn split(a: f64) -> (f64, f64) {
    let c = SPLITTER * a;
    let abig = c - a;
    let ahi = c - abig;
    let alo = a - ahi;
    (ahi, alo)
}

#[inline]
fn two_product_presplit(a: f64, b: f64, bhi: f64, blo: f64) -> (f64, f64) {
    let x = a * b;
    let (ahi, alo) = split(a);
    let err1 = x - ahi * bhi;
    let err2 = err1 - alo * bhi;
    let err3 = err2 - ahi * blo;
    let y = alo * blo - err3;
    (x, y)
}

#[inline]
fn two_two_diff(a1: f64, a0: f64, b1: f64, b0: f64) -> (f64, f64, f64, f64) {
    let (j, _r0, x0) = two_one_diff(a1, a0, b0);
    let (x3, x2, x1) = two_one_diff(j, _r0, b1);
    (x3, x2, x1, x0)
}

#[inline]
fn two_one_diff(a1: f64, a0: f64, b: f64) -> (f64, f64, f64) {
    let (i, x0) = two_diff(a0, b);
    let (x2, x1) = two_sum(a1, i);
    (x2, x1, x0)
}

#[inline]
fn two_diff(a: f64, b: f64) -> (f64, f64) {
    let x = a - b;
    (x, two_diff_tail(a, b, x))
}

#[inline]
fn two_diff_tail(a: f64, b: f64, x: f64) -> f64 {
    let bvirt = a - x;
    let avirt = x + bvirt;
    let bround = bvirt - b;
    let around = a - avirt;
    around + bround
}

#[inline]
fn two_sum(a: f64, b: f64) -> (f64, f64) {
    let x = a + b;
    (x, two_sum_tail(a, b, x))
}

#[inline]
fn two_sum_tail(a: f64, b: f64, x: f64) -> f64 {
    let bvirt = x - a;
    let avirt = x - bvirt;
    let bround = b - bvirt;
    let around = a - avirt;
    around + bround
}

#[inline]
fn fast_two_sum_tail(a: f64, b: f64, x: f64) -> f64 {
    let bvirt = x - a;
    b - bvirt
}

#[inline]
fn fast_two_sum(a: f64, b: f64) -> (f64, f64) {
    let x = a + b;
    (x, fast_two_sum_tail(a, b, x))
}

#[inline]
fn two_one_sum(a1: f64, a0: f64, b: f64) -> (f64, f64, f64) {
    let (_i, x0) = two_sum(a0, b);
    let (x2, x1) = two_sum(a1, _i);
    (x2, x1, x0)
}

#[inline]
fn two_two_sum(a1: f64, a0: f64, b1: f64, b0: f64) -> (f64, f64, f64, f64) {
    let (_j, _r0, x0) = two_one_sum(a1, a0, b0);
    let (x3, x2, x1) = two_one_sum(_j, _r0, b1);
    (x3, x2, x1, x0)
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SOE {
    pub x_maj: f64,
    pub x_min: f64,
}

impl From<f64> for SOE {
    fn from(value: f64) -> Self {
        SOE::from_f64(value)
    }
}

impl SOE {
    pub fn from_f64(x: f64) -> SOE {
        SOE {
            x_maj: x,
            x_min: 0.,
        }
    }

    pub fn from_add(a: f64, b: f64) -> SOE {
        let (x_maj, x_min) = two_sum(a, b);
        SOE { x_maj, x_min }
    }

    pub fn from_sub(a: f64, b: f64) -> SOE {
        let (x_maj, x_min) = two_diff(a, b);
        // println!("{:.17} {:.17}", x_maj, x_min);
        SOE { x_maj, x_min }
    }

    pub fn from_mul(a: f64, b: f64) -> SOE {
        let (x_maj, x_min) = two_product(a, b);
        SOE { x_maj, x_min }
    }

    pub fn from_expansion(x3: f64, x2: f64, x1: f64, x0: f64) -> SOE {
        let t = x3 + x2;
        if t - x3 - x2 != 0. {
            // println!("Constructing from 1. term");
            return SOE {
                x_maj: x3,
                x_min: x0 + x1 + x2,
            };
        }

        let t = x3 + x2 + x1;
        if t - x3 - x2 - x1 != 0. {
            // println!("Constructing from 1. + 2. term");
            return SOE {
                x_maj: x2 + x3,
                x_min: x0 + x1,
            };
        }

        // println!("Constructing from 1. + 2. + 3. term");
        return SOE {
            x_maj: x1 + x2 + x3,
            x_min: x0,
        };
    }

    pub fn from_scale_expansion(x: SOE, b: f64) -> SOE {
        let mut temp = [0f64; 4];
        let _ = scale_expansion_zeroelim(&[x.x_maj, x.x_min], b, &mut temp);
        SOE::from_expansion(temp[0], temp[1], temp[2], temp[3])
    }

    pub fn to_f64(self) -> f64 {
        self.x_maj + self.x_min
    }
}

use std::fmt::Display;

impl Display for SOE {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut buffer1 = ryu::Buffer::new();
        let mut buffer2 = ryu::Buffer::new();
        let a = buffer1.format(self.x_maj);
        let b = buffer2.format(self.x_min);
        write!(f, "({}, {})", a, b)
    }
}

use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

impl Add for SOE {
    #[inline]
    fn add(self, that: Self) -> Self {
        let (x3, x2, x1, x0) = two_two_sum(self.x_maj, self.x_min, that.x_maj, that.x_min);
        SOE::from_expansion(x3, x2, x1, x0)
    }
    type Output = Self;
}

impl Sub for SOE {
    #[inline]
    fn sub(self, that: Self) -> Self {
        let (x3, x2, x1, x0) = two_two_diff(self.x_maj, self.x_min, that.x_maj, that.x_min);
        SOE::from_expansion(x3, x2, x1, x0)
    }
    type Output = Self;
}

impl Mul for SOE {
    #[inline]
    fn mul(self, that: Self) -> Self {
        let a = SOE::from_mul(self.x_maj, that.x_maj);
        let b = SOE::from_mul(self.x_maj, that.x_min);
        let c = SOE::from_mul(self.x_min, that.x_maj);
        let d = SOE::from_mul(self.x_min, that.x_min);
        // println!("a = {}", a);
        // println!("b = {}", b);
        // println!("c = {}", c);
        // println!("d = {}", d);
        return d + c + b + a;
        /*
        let soe_hi = SOE::from_scale_expansion(self, that.x_maj);
        let soe_lo = SOE::from_scale_expansion(self, that.x_min);
        soe_hi + soe_lo
        */
    }
    type Output = Self;
}

impl Div for SOE {
    #[inline]
    fn div(self, that: Self) -> Self {
        let fac1 = self.to_f64() / that.to_f64();
        // println!("fac1 = {}", fac1);
        // let tmp = SOE{x_maj: that.x_maj * fac1, x_min: that.x_min * fac1};
        let tmp = that
            * SOE {
                x_maj: fac1,
                x_min: 0.,
            };
        let rem = self - tmp;
        // println!("tmp = {}", tmp);
        // println!("rem = {}", rem);
        let fac2 = rem.to_f64() / that.to_f64();
        SOE {
            x_maj: fac1,
            x_min: fac2,
        }
    }
    type Output = Self;
}

#[cfg(test)]
mod test {
    use super::SOE;

    #[test]
    fn soe() {
        let a = SOE {
            x_maj: 1e300,
            x_min: 1e-300,
        };
        let b = SOE {
            x_maj: 1e300,
            x_min: 1e-300,
        };
        let c = a + b;
        assert_eq!(c.x_maj, 2e300);
        assert_eq!(c.x_min, 2e-300);

        let a = SOE {
            x_maj: 2e300,
            x_min: 2e-300,
        };
        let b = SOE {
            x_maj: 1e300,
            x_min: 1e-300,
        };
        let c = a - b;
        assert_eq!(c.x_maj, 1e300);
        assert_eq!(c.x_min, 1e-300);

        let a = SOE {
            x_maj: 1e100,
            x_min: 1e-100,
        };
        let b = SOE {
            x_maj: 2.,
            x_min: 0.,
        };
        let c = a * b;
        assert_eq!(c.x_maj, 2e100);
        assert_eq!(c.x_min, 2e-100);

        let a = SOE {
            x_maj: 1e10,
            x_min: 1e-10,
        };
        let b = SOE {
            x_maj: 2e10,
            x_min: 2e-10,
        };
        let c = a * b;
        assert_eq!(c.x_maj, 2e20);
        assert_eq!(c.x_min, 4.);

        let a = SOE {
            x_maj: 1e10,
            x_min: 1e-10,
        };
        let c = SOE {
            x_maj: 2e20,
            x_min: 4.,
        };
        let b = c / a;
        println!("{}", b);
        assert_eq!(b.x_maj, 2e10);
        assert_eq!(b.x_min, 2e-10);

        let b = SOE {
            x_maj: 2e10,
            x_min: 2e-10,
        };
        let c = SOE {
            x_maj: 2e20,
            x_min: 4.,
        };
        let a = c / b;
        println!("{}", a);
        assert_eq!(a.x_maj, 1e10);
        assert_eq!(a.x_min, 1e-10);
    }
}
