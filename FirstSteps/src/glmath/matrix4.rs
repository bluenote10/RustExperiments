use std::num::Float;

#[deriving(Show, PartialEq)]
pub struct Matrix4 {
  pub m00: f32, pub m10: f32, pub m20: f32, pub m30: f32,
  pub m01: f32, pub m11: f32, pub m21: f32, pub m31: f32,
  pub m02: f32, pub m12: f32, pub m22: f32, pub m32: f32,
  pub m03: f32, pub m13: f32, pub m23: f32, pub m33: f32,
}


impl Matrix4 {
  fn new_by_row(
    m00: f32, m10: f32, m20: f32, m30: f32,
    m01: f32, m11: f32, m21: f32, m31: f32,
    m02: f32, m12: f32, m22: f32, m32: f32,
    m03: f32, m13: f32, m23: f32, m33: f32,
  ) -> Matrix4 {
    Matrix4{
      m00: m00, m10: m10, m20: m20, m30: m30,
      m01: m01, m11: m11, m21: m21, m31: m31,
      m02: m02, m12: m12, m22: m22, m32: m32,
      m03: m03, m13: m13, m23: m23, m33: m33,
    }
  }

  fn new_by_col(
    m00: f32, m01: f32, m02: f32, m03: f32,
    m10: f32, m11: f32, m12: f32, m13: f32,
    m20: f32, m21: f32, m22: f32, m23: f32,
    m30: f32, m31: f32, m32: f32, m33: f32,
  ) -> Matrix4 {
    Matrix4{
      m00: m00, m10: m10, m20: m20, m30: m30,
      m01: m01, m11: m11, m21: m21, m31: m31,
      m02: m02, m12: m12, m22: m22, m32: m32,
      m03: m03, m13: m13, m23: m23, m33: m33,
    }
  }



}
  



