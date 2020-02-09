use std::fmt::{Debug, Display};
use num_traits::Float as NumTraitsFloat;
use float_extras::f64::nextafter;

pub trait Float: NumTraitsFloat + Debug + Display + NextAfter + Into<f64> {}

impl<T: NumTraitsFloat + Debug + Display + NextAfter + Into<f64>> Float for T {}


pub trait NextAfter: NumTraitsFloat {
    fn nextafter(self, up: bool) -> Self;
    fn nextafter_steps(self, steps: i32) -> Self;

    fn ulp(self) -> Self {
        if self > Self::zero() {
            self.nextafter(true) - self
        } else {
            self.nextafter(false) - self
        }
    }
}

impl NextAfter for f64 {
    fn nextafter(self, up: bool) -> Self {
        if up {
            nextafter(self, std::f64::INFINITY)
        } else {
            nextafter(self, std::f64::NEG_INFINITY)
        }
    }

    fn nextafter_steps(self, steps: i32) -> Self {
        let mut x = self;
        for _ in 0..steps.abs() {
            x = x.nextafter(steps > 0);
        }
        x
    }
}
