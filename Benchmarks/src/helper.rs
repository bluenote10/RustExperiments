use std::fmt::{Debug, Display};
use num_traits::Float as NumTraitsFloat;

pub trait Float: NumTraitsFloat + Debug + Display + Into<f64> {}
impl<T: NumTraitsFloat + Debug + Display + Into<f64>> Float for T {}
