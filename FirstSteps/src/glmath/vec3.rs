
use std::num::Float;

#[deriving(Show, PartialEq)]
pub struct Vec3 {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

/*
impl PartialEq for Vec3 {
  fn eq(&self, that: &Vec3) -> bool {
    self.x == that.x &&
    self.y == that.y &&
    self.z == that.z
  }
}
*/

impl Vec3 {
  fn new(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3{x: x, y: y, z: z}
  }
  
  fn length(&self) -> f32 {
    Float::sqrt(self.x*self.x + self.y*self.y + self.z*self.z)
  }
  
  fn scalar_add(&self, s: f32) -> Vec3 {
    Vec3::new(self.x + s, self.y + s, self.z + s)
  }

  fn scalar_mul(&self, s: f32) -> Vec3 {
    Vec3::new(self.x * s, self.y * s, self.z * s)
  }
  
  fn normalize(&self) -> Vec3 {
    self.set_length_to(1.0)
  }
  fn set_length_to(&self, l_new: f32) -> Vec3 {
    let l_old = self.length();
    self.scalar_mul(l_new / l_old)
  }

}


// -------------------------------------------------------
// Operator overloading
// -------------------------------------------------------

impl Add<Vec3, Vec3> for Vec3 {
  fn add(&self, that: &Vec3) -> Vec3 {
    Vec3::new(self.x + that.x, self.y + that.y, self.z + that.z)
  }
}
impl Sub<Vec3, Vec3> for Vec3 {
  fn sub(&self, that: &Vec3) -> Vec3 {
    Vec3::new(self.x - that.x, self.y - that.y, self.z - that.z)
  }
}
impl Mul<Vec3, Vec3> for Vec3 {
  fn mul(&self, that: &Vec3) -> Vec3 {
    Vec3::new(self.x * that.x, self.y * that.y, self.z * that.z)
  }
}
impl Div<Vec3, Vec3> for Vec3 {
  fn div(&self, that: &Vec3) -> Vec3 {
    Vec3::new(self.x / that.x, self.y / that.y, self.z / that.z)
  }
}


#[test]
fn test_add() {
  let v1 = Vec3::new(1., 2., 3.);
  let v2 = Vec3::new(4., 5., 6.);
  assert!(v1 + v2 == Vec3::new(5., 7., 9.));
}
#[test]
fn test_sub() {
  let v1 = Vec3::new(1., 2., 3.);
  let v2 = Vec3::new(4., 5., 6.);
  assert!(v1 - v2 == Vec3::new(-3., -3., -3.));
}
#[test]
fn test_mul() {
  let v1 = Vec3::new(1., 2., 3.);
  let v2 = Vec3::new(4., 5., 6.);
  assert!(v1 * v2 == Vec3::new(4., 10., 18.));
}
#[test]
fn test_div() {
  let v1 = Vec3::new(1., 2., 3.);
  let v2 = Vec3::new(4., 5., 6.);
  assert!(v1 / v2 == Vec3::new(1./4., 2./5., 3./6.));
}

#[test]
fn test_normalize() {
  let v1 = Vec3::new(1., 2., 3.);
  let v2 = Vec3::new(4., 5., 6.);
  println!("{}",v1.normalize().length() + 0.00001f32);
  println!("{}",v2.normalize().length());
  assert!(v1.normalize().length().almost_equal(1f32));
  assert!(v2.normalize().length().almost_equal(1f32));
}

#[cfg(test)]
trait AlmostEqual {
  fn almost_equal(&self, that: f32) -> bool;
}

#[cfg(test)]
impl AlmostEqual for f32 {
  fn almost_equal(&self, that: f32) -> bool {
    self.abs() - that < Float::epsilon()
  }
}




