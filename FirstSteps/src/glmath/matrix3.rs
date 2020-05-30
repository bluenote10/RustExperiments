use std::num::Float;

#[deriving(Show, PartialEq)]
pub struct Matrix3 {
  pub m00: f32, pub m10: f32, pub m20: f32,
  pub m01: f32, pub m11: f32, pub m21: f32,
  pub m02: f32, pub m12: f32, pub m22: f32,
}
