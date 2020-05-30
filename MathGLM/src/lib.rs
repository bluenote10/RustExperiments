use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::f32::consts::PI;

// ----------------------------------------------------------------------------
// Vec3f
// ----------------------------------------------------------------------------

pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

// ----------------------------------------------------------------------------
// Vector/Vector operations
// ----------------------------------------------------------------------------

impl Add for &Vec3f {
    fn add(self, that: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.x + that.x,
            y: self.y + that.y,
            z: self.z + that.z,
        }
    }
    type Output = Vec3f;
}

impl Sub for &Vec3f {
    fn sub(self, that: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.x - that.x,
            y: self.y - that.y,
            z: self.z - that.z,
        }
    }
    type Output = Vec3f;
}

impl Mul for &Vec3f {
    fn mul(self, that: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.x * that.x,
            y: self.y * that.y,
            z: self.z * that.z,
        }
    }
    type Output = Vec3f;
}

// ----------------------------------------------------------------------------
// Scalar operations
// ----------------------------------------------------------------------------

impl Add<f32> for &Vec3f {
    fn add(self, scalar: f32) -> Vec3f {
        Vec3f {
            x: self.x + scalar,
            y: self.y + scalar,
            z: self.z + scalar,
        }
    }
    type Output = Vec3f;
}

impl Sub<f32> for &Vec3f {
    fn sub(self, scalar: f32) -> Vec3f {
        Vec3f {
            x: self.x - scalar,
            y: self.y - scalar,
            z: self.z - scalar,
        }
    }
    type Output = Vec3f;
}

impl Mul<f32> for &Vec3f {
    fn mul(self, scalar: f32) -> Vec3f {
        Vec3f {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
    type Output = Vec3f;
}

impl Div<f32> for &Vec3f {
    fn div(self, scalar: f32) -> Vec3f {
        Vec3f {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
    type Output = Vec3f;
}

// ----------------------------------------------------------------------------
// Methods
// ----------------------------------------------------------------------------

impl Vec3f {

    pub fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f{x: x, y: y, z: z}
    }

    pub fn update(&mut self, that: &Vec3f) {
        self.x = that.x;
        self.y = that.y;
        self.z = that.z;
        // return self?
    }

    pub fn cross(&self, that: &Vec3f) -> Vec3f {
        Vec3f {
            x: self.y*that.z - self.z*that.y,
            y: self.z*that.x - self.x*that.z,
            z: self.x*that.y - self.y*that.x,
        }
    }

    pub fn mid(&self, that: &Vec3f) -> Vec3f {
        Vec3f {
            x: 0.5f32 * (self.x + that.x),
            y: 0.5f32 * (self.y + that.y),
            z: 0.5f32 * (self.z + that.z),
        }
    }

    pub fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn normalized_with_length(&self, l: f32) -> Vec3f {
        return self * (l / self.length())
    }

    pub fn normalized(&self) -> Vec3f {
        return self / self.length()
    }

    pub fn normalized_in_place(&mut self) {
        let length = self.length();
        self.x = self.x / length;
        self.y = self.y / length;
        self.z = self.z / length;
    }

    pub fn negate(&self) -> Vec3f {
        Vec3f {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}


// ----------------------------------------------------------------------------
// Vec4f
// ----------------------------------------------------------------------------

pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

// ----------------------------------------------------------------------------
// Vector/Vector operations
// ----------------------------------------------------------------------------

impl Add for &Vec4f {
    fn add(self, that: &Vec4f) -> Vec4f {
        Vec4f {
            x: self.x + that.x,
            y: self.y + that.y,
            z: self.z + that.z,
            w: self.w + that.w,
        }
    }
    type Output = Vec4f;
}

impl Sub for &Vec4f {
    fn sub(self, that: &Vec4f) -> Vec4f {
        Vec4f {
            x: self.x - that.x,
            y: self.y - that.y,
            z: self.z - that.z,
            w: self.w - that.w,
        }
    }
    type Output = Vec4f;
}

impl Mul for &Vec4f {
    fn mul(self, that: &Vec4f) -> Vec4f {
        Vec4f {
            x: self.x * that.x,
            y: self.y * that.y,
            z: self.z * that.z,
            w: self.w * that.w,
        }
    }
    type Output = Vec4f;
}

// ----------------------------------------------------------------------------
// Scalar operations
// ----------------------------------------------------------------------------

impl Add<f32> for &Vec4f {
    fn add(self, scalar: f32) -> Vec4f {
        Vec4f {
            x: self.x + scalar,
            y: self.y + scalar,
            z: self.z + scalar,
            w: self.w + scalar,
        }
    }
    type Output = Vec4f;
}

impl Sub<f32> for &Vec4f {
    fn sub(self, scalar: f32) -> Vec4f {
        Vec4f {
            x: self.x - scalar,
            y: self.y - scalar,
            z: self.z - scalar,
            w: self.w - scalar,
        }
    }
    type Output = Vec4f;
}

impl Mul<f32> for &Vec4f {
    fn mul(self, scalar: f32) -> Vec4f {
        Vec4f {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
    type Output = Vec4f;
}

impl Div<f32> for &Vec4f {
    fn div(self, scalar: f32) -> Vec4f {
        Vec4f {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
    type Output = Vec4f;
}

// ----------------------------------------------------------------------------
// Methods
// ----------------------------------------------------------------------------

impl Vec4f {

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vec4f {
        Vec4f{x: x, y: y, z: z, w: w}
    }

    pub fn update(&mut self, that: &Vec4f) {
        self.x = that.x;
        self.y = that.y;
        self.z = that.z;
        self.w = that.w;
        // return self?
    }

    pub fn cross(&self, that: &Vec4f) -> Vec4f {
        Vec4f {
            x: self.y*that.z - self.z*that.y,
            y: self.z*that.x - self.x*that.z,
            z: self.x*that.y - self.y*that.x,
            w: self.w*that.w - self.w*that.w,
        }
    }

    pub fn mid(&self, that: &Vec4f) -> Vec4f {
        Vec4f {
            x: 0.5f32 * (self.x + that.x),
            y: 0.5f32 * (self.y + that.y),
            z: 0.5f32 * (self.z + that.z),
            w: 0.5f32 * (self.w + that.w),
        }
    }

    pub fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt()
    }

    pub fn normalized_with_length(&self, l: f32) -> Vec4f {
        return self * (l / self.length())
    }

    pub fn normalized(&self) -> Vec4f {
        return self / self.length()
    }

    pub fn normalized_in_place(&mut self) {
        let length = self.length();
        self.x = self.x / length;
        self.y = self.y / length;
        self.z = self.z / length;
        self.w = self.w / length;
    }

    pub fn negate(&self) -> Vec4f {
        Vec4f {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}


// ----------------------------------------------------------------------------
// Vec4f
// ----------------------------------------------------------------------------

pub struct Mat4f {
    pub m00: f32, pub m10: f32, pub m20: f32, pub m30: f32,
    pub m01: f32, pub m11: f32, pub m21: f32, pub m31: f32,
    pub m02: f32, pub m12: f32, pub m22: f32, pub m32: f32,
    pub m03: f32, pub m13: f32, pub m23: f32, pub m33: f32,
}

// ----------------------------------------------------------------------------
// Matrix/Matrix operations
// ----------------------------------------------------------------------------

impl Add for &Mat4f {
    fn add(self, that: &Mat4f) -> Mat4f {
        Mat4f::new(
            self.m00+that.m00, self.m10+that.m10, self.m20+that.m20, self.m30+that.m30,
            self.m01+that.m01, self.m11+that.m11, self.m21+that.m21, self.m31+that.m31,
            self.m02+that.m02, self.m12+that.m12, self.m22+that.m22, self.m32+that.m32,
            self.m03+that.m03, self.m13+that.m13, self.m23+that.m23, self.m33+that.m33,
        )
    }
    type Output = Mat4f;
}

impl Sub for &Mat4f {
    fn sub(self, that: &Mat4f) -> Mat4f {
        Mat4f::new(
            self.m00-that.m00, self.m10-that.m10, self.m20-that.m20, self.m30-that.m30,
            self.m01-that.m01, self.m11-that.m11, self.m21-that.m21, self.m31-that.m31,
            self.m02-that.m02, self.m12-that.m12, self.m22-that.m22, self.m32-that.m32,
            self.m03-that.m03, self.m13-that.m13, self.m23-that.m23, self.m33-that.m33,
        )
    }
    type Output = Mat4f;
}

impl Mul for &Mat4f {
    fn mul(self, that: &Mat4f) -> Mat4f {
        let nm00 = self.m00 * that.m00 + self.m10 * that.m01 + self.m20 * that.m02 + self.m30 * that.m03;
        let nm01 = self.m01 * that.m00 + self.m11 * that.m01 + self.m21 * that.m02 + self.m31 * that.m03;
        let nm02 = self.m02 * that.m00 + self.m12 * that.m01 + self.m22 * that.m02 + self.m32 * that.m03;
        let nm03 = self.m03 * that.m00 + self.m13 * that.m01 + self.m23 * that.m02 + self.m33 * that.m03;
        let nm10 = self.m00 * that.m10 + self.m10 * that.m11 + self.m20 * that.m12 + self.m30 * that.m13;
        let nm11 = self.m01 * that.m10 + self.m11 * that.m11 + self.m21 * that.m12 + self.m31 * that.m13;
        let nm12 = self.m02 * that.m10 + self.m12 * that.m11 + self.m22 * that.m12 + self.m32 * that.m13;
        let nm13 = self.m03 * that.m10 + self.m13 * that.m11 + self.m23 * that.m12 + self.m33 * that.m13;
        let nm20 = self.m00 * that.m20 + self.m10 * that.m21 + self.m20 * that.m22 + self.m30 * that.m23;
        let nm21 = self.m01 * that.m20 + self.m11 * that.m21 + self.m21 * that.m22 + self.m31 * that.m23;
        let nm22 = self.m02 * that.m20 + self.m12 * that.m21 + self.m22 * that.m22 + self.m32 * that.m23;
        let nm23 = self.m03 * that.m20 + self.m13 * that.m21 + self.m23 * that.m22 + self.m33 * that.m23;
        let nm30 = self.m00 * that.m30 + self.m10 * that.m31 + self.m20 * that.m32 + self.m30 * that.m33;
        let nm31 = self.m01 * that.m30 + self.m11 * that.m31 + self.m21 * that.m32 + self.m31 * that.m33;
        let nm32 = self.m02 * that.m30 + self.m12 * that.m31 + self.m22 * that.m32 + self.m32 * that.m33;
        let nm33 = self.m03 * that.m30 + self.m13 * that.m31 + self.m23 * that.m32 + self.m33 * that.m33;
        Mat4f::new(
            nm00, nm10, nm20, nm30,
            nm01, nm11, nm21, nm31,
            nm02, nm12, nm22, nm32,
            nm03, nm13, nm23, nm33,
        )
    }
    type Output = Mat4f;
}

// ----------------------------------------------------------------------------
// Matrix/Vector operations
// ----------------------------------------------------------------------------

impl Mul<&Vec4f> for &Mat4f {
    fn mul(self, v: &Vec4f) -> Vec4f {
        Vec4f {
            x: self.m00*v.x + self.m10*v.y + self.m20*v.z + self.m30*v.w,
            y: self.m01*v.x + self.m11*v.y + self.m21*v.z + self.m31*v.w,
            z: self.m02*v.x + self.m12*v.y + self.m22*v.z + self.m32*v.w,
            w: self.m03*v.x + self.m13*v.y + self.m23*v.z + self.m33*v.w,
        }
    }
    type Output = Vec4f;
}

// ----------------------------------------------------------------------------
// Scalar operations
// ----------------------------------------------------------------------------

impl Mul<f32> for &Mat4f {
    fn mul(self, s: f32) -> Mat4f {
        Mat4f::new(
            self.m00*s, self.m10*s, self.m20*s, self.m30*s,
            self.m01*s, self.m11*s, self.m21*s, self.m31*s,
            self.m02*s, self.m12*s, self.m22*s, self.m32*s,
            self.m03*s, self.m13*s, self.m23*s, self.m33*s,
        )
    }
    type Output = Mat4f;
}

// ----------------------------------------------------------------------------
// Methods
// ----------------------------------------------------------------------------

impl Mat4f {
    pub fn new(
        m00: f32, m10: f32, m20: f32, m30: f32,
        m01: f32, m11: f32, m21: f32, m31: f32,
        m02: f32, m12: f32, m22: f32, m32: f32,
        m03: f32, m13: f32, m23: f32, m33: f32,
    ) -> Mat4f {
        Mat4f {
            m00: m00, m10: m10, m20: m20, m30: m30,
            m01: m01, m11: m11, m21: m21, m31: m31,
            m02: m02, m12: m12, m22: m22, m32: m32,
            m03: m03, m13: m13, m23: m23, m33: m33,
        }
    }

    pub fn update(&mut self, that: &Mat4f) {
        self.m00 = that.m00;
        self.m01 = that.m01;
        self.m02 = that.m02;
        self.m03 = that.m03;
        self.m10 = that.m10;
        self.m11 = that.m11;
        self.m12 = that.m12;
        self.m13 = that.m13;
        self.m20 = that.m20;
        self.m21 = that.m21;
        self.m22 = that.m22;
        self.m23 = that.m23;
        self.m30 = that.m30;
        self.m31 = that.m31;
        self.m32 = that.m32;
        self.m33 = that.m33;
    }

    pub fn transpose(&self) -> Mat4f {
        Mat4f::new(
            self.m00, self.m01, self.m02, self.m03,
            self.m10, self.m11, self.m12, self.m13,
            self.m20, self.m21, self.m22, self.m23,
            self.m30, self.m31, self.m32, self.m33,
        )
    }

    pub fn frobenius_distance(&self, that: &Mat4f) -> f32 {
        (
            (self.m00-that.m00)*(self.m00-that.m00) +
            (self.m01-that.m01)*(self.m01-that.m01) +
            (self.m02-that.m02)*(self.m02-that.m02) +
            (self.m03-that.m03)*(self.m03-that.m03) +
            (self.m10-that.m10)*(self.m10-that.m10) +
            (self.m11-that.m11)*(self.m11-that.m11) +
            (self.m12-that.m12)*(self.m12-that.m12) +
            (self.m13-that.m13)*(self.m13-that.m13) +
            (self.m20-that.m20)*(self.m20-that.m20) +
            (self.m21-that.m21)*(self.m21-that.m21) +
            (self.m22-that.m22)*(self.m22-that.m22) +
            (self.m23-that.m23)*(self.m23-that.m23) +
            (self.m30-that.m30)*(self.m30-that.m30) +
            (self.m31-that.m31)*(self.m31-that.m31) +
            (self.m32-that.m32)*(self.m32-that.m32) +
            (self.m33-that.m33)*(self.m33-that.m33)
        ).sqrt()
    }

    // --------------------------------------------------------------
    // Constructors
    // --------------------------------------------------------------

    pub fn create_identity() -> Mat4f {
        Mat4f::new(
            1f32, 0f32, 0f32, 0f32,
            0f32, 1f32, 0f32, 0f32,
            0f32, 0f32, 1f32, 0f32,
            0f32, 0f32, 0f32, 1f32,
        )
    }
    pub fn create_zero() -> Mat4f {
        Mat4f::new(
            0f32, 0f32, 0f32, 0f32,
            0f32, 0f32, 0f32, 0f32,
            0f32, 0f32, 0f32, 0f32,
            0f32, 0f32, 0f32, 0f32,
        )
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Mat4f {
        Mat4f::new(
            1f32, 0f32, 0f32,    x,
            0f32, 1f32, 0f32,    y,
            0f32, 0f32, 1f32,    z,
            0f32, 0f32, 0f32, 1f32,
        )
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Mat4f {
        Mat4f::new(
               x, 0f32, 0f32, 0f32,
            0f32,    y, 0f32, 0f32,
            0f32, 0f32,    z, 0f32,
            0f32, 0f32, 0f32, 1f32,
        )
    }

    /**
     * Reimplementation of glRotate:
     *   https://www.opengl.org/sdk/docs/man2/xhtml/glRotate.xml
     *
     * Convention: angles in DEG
     */
    pub fn rotate(angle: f32, _x: f32, _y: f32, _z: f32) -> Mat4f {
        let angle_rad = angle * PI / 180f32;
        let axis = Vec3f::new(_x, _y, _z).normalized();
        let x = axis.x;
        let y = axis.y;
        let z = axis.z;
        let s = angle_rad.sin();
        let c = angle_rad.cos();

        Mat4f::new(
            x*x*(1f32-c)+c,   x*y*(1f32-c)-z*s, x*z*(1f32-c)+y*s, 0f32,
            y*x*(1f32-c)+z*s, y*y*(1f32-c)+c,   y*z*(1f32-c)-x*s, 0f32,
            x*z*(1f32-c)-y*s, y*z*(1f32-c)+x*s, z*z*(1f32-c)+c,   0f32,
            0f32,             0f32,             0f32,             1f32,
        )
    }

    /**
     * http://www.flipcode.com/documents/matrfaq.html#Q36
     * http://www.songho.ca/opengl/gl_anglestoaxes.html
     *
     *       |  CE      -CF      -D   0 |
     *  M  = | -BDE+AF   BDF+AE  -BC  0 |
     *       |  ADE+BF  -ADF+BE   AC  0 |
     *       |  0        0        0   1 |
     *   where A,B are the cosine and sine of the X-axis rotation axis, (pitch)
     *         C,D are the cosine and sine of the Y-axis rotation axis, (yaw)
     *         E,F are the cosine and sine of the Z-axis rotation axis. (roll)
     *
     * Convention: angles in DEG
     */
    pub fn rotate_yaw_pitch_roll(yaw: f32, pitch: f32, roll: f32) -> Mat4f {
        let a = (pitch * PI / 180f32).cos();
        let b = (pitch * PI / 180f32).sin();
        let c = (yaw   * PI / 180f32).cos();
        let d = (yaw   * PI / 180f32).sin();
        let e = (roll  * PI / 180f32).cos();
        let f = (roll  * PI / 180f32).sin();
        Mat4f::new(
                   c*e,       -c*f,   -d, 0f32,
            -b*d*e+a*f,  b*d*f+a*e, -b*c, 0f32,
             a*d*e+b*f, -a*d*f+b*e,  a*c, 0f32,
                  0f32,       0f32, 0f32, 1f32,
        )
    }

    /**
     * Though the above approach is mentioned frequently, it looks like is still has gimbal lock issues.
     * Therefore proper solution using quaternions:
     * - convert each euler angle to a quaternion
     * - multiply quaternions (in correct order!)
     * - convert the resulting quaternion to a rotation matrix.
     * Regarding the order of the multiplication:
     * This depends on the definition of the euler angles.
     * Here the order is "optimized" for Oculus Rift.
     * In case of trouble: trial & error...
     *
     * Convention: angles in DEG
     */
    pub fn rotate_yaw_pitch_roll_quaternions(yaw: f32, pitch: f32, roll: f32) -> Mat4f {
        let quat_pitch = Quaternion::create(pitch, 1f32, 0f32, 0f32);
        let quat_yaw   = Quaternion::create(yaw,   0f32, 1f32, 0f32);
        let quat_roll  = Quaternion::create(roll,  0f32, 0f32, 1f32);
        // order that does not work: pitch * yaw * roll (roll gets inverted when yaw != 0)
        (&(&quat_roll * &quat_pitch) * &quat_yaw).normalized().cast_to_orientation_matrix()
    }

}

/*

/**
 * 4 dimensional Matrix implementation.
 * I know there are Java possibilities:
 * - https://github.com/jroyalty/jglm/blob/master/src/main/java/com/hackoeur/jglm/Mat4.java
 * - https://github.com/LWJGL/lwjgl/blob/master/src/java/org/lwjgl/util/vector/Matrix4f.java
 * But preferrable in Scala...
 */
class Mat4f(
  var m00: Float, var m10: Float, var m20: Float, var m30: Float,
  var m01: Float, var m11: Float, var m21: Float, var m31: Float,
  var m02: Float, var m12: Float, var m22: Float, var m32: Float,
  var m03: Float, var m13: Float, var m23: Float, var m33: Float
) {

  def rotateYawPitchRoll(yaw: Float, pitch: Float, roll: Float, leftMultiply: Boolean = false): Mat4f = {
    if (!leftMultiply) {
      return this * Mat4f.rotateYawPitchRoll(yaw, pitch, roll)
    } else {
      return Mat4f.rotateYawPitchRoll(yaw, pitch, roll) * this
    }
  }
  def rotateYawPitchRollQuaternions(yaw: Float, pitch: Float, roll: Float, leftMultiply: Boolean = false): Mat4f = {
    if (!leftMultiply) {
      return this * Mat4f.rotateYawPitchRollQuaternions(yaw, pitch, roll)
    } else {
      return Mat4f.rotateYawPitchRollQuaternions(yaw, pitch, roll) * this
    }
  }
  def rotate(angle: Float, x: Float, y: Float, z: Float, leftMultiply: Boolean = false): Mat4f = {
    if (!leftMultiply) {
      return this * Mat4f.rotate(angle, x, y, z)
    } else {
      return Mat4f.rotate(angle, x, y, z) * this
    }
  }
  def translate(x: Float, y: Float, z: Float, leftMultiply: Boolean = false): Mat4f = {
    if (!leftMultiply) {
      return this * Mat4f.translate(x, y, z)
    } else {
      return Mat4f.translate(x, y, z) * this
    }
  }
  def scale(x: Float, y: Float, z: Float): Mat4f = {
    return this * Mat4f.scale(x, y, z)
  }

  /**
   * Inversion of affine matrix
   *   http://stackoverflow.com/questions/2624422/efficient-4x4-matrix-inverse-affine-transform
   */
  def inverseAffine(): Mat4f = {
    val blockInv = Mat3f.createFromMat4f(this).inverse
    val posVec = blockInv * Vec3f(this.m30, this.m31, this.m32).negate
    Mat4f.createFromAffine(blockInv, posVec)
  }

  /**
   * This just uses the length of the first column vector to extract the scale of the matrix,
   * which is actually just the scale in x direction. But since our transformation are all
   * uniform scale, we can use any of the columns...
   *   http://math.stackexchange.com/a/1463487/65103
   */
  def extractUniformScale(): Float = {
    math.sqrt(m00*m00 + m01*m01 + m02*m02).toFloat
  }

}


object Mat4f {

  /**
   * The above definitions are not prefixed with "create" in order to directly
   * resemble the operator API.
   * All remaining builders follow the create... convention.
   */

  def createFromAffine(M: Mat3f, v: Vec3f) = new Mat4f(
    M.m00, M.m10, M.m20, v.x,
    M.m01, M.m11, M.m21, v.y,
    M.m02, M.m12, M.m22, v.z,
        0,     0,     0,   1
  )
  def createFromMat3f(that: Mat3f): Mat4f = {
    new Mat4f(
      that.m00, that.m10, that.m20, 0,
      that.m01, that.m11, that.m21, 0,
      that.m02, that.m12, that.m22, 0,
             0,        0,        0, 1
    )
  }

  def createFromFloatBuffer(buf: FloatBuffer): Mat4f = {
    val arr = new Array[Float](16)
    buf.get(arr)
    createFromColumnMajorArray(arr)
  }

  def createFromColumnMajorArray(arr: Array[Float]): Mat4f = {
    new Mat4f(
      arr(0), arr(4), arr( 8), arr(12),
      arr(1), arr(5), arr( 9), arr(13),
      arr(2), arr(6), arr(10), arr(14),
      arr(3), arr(7), arr(11), arr(15)
    )
  }

  def createFromRowMajorArray(arr: Array[Float]): Mat4f = {
    new Mat4f(
      arr( 0), arr( 1), arr( 2), arr( 3),
      arr( 4), arr( 5), arr( 6), arr( 7),
      arr( 8), arr( 9), arr(10), arr(11),
      arr(12), arr(13), arr(14), arr(15)
    )
  }

  /**
   * Creates a projection matrix (according to Oculus Rift documentation):
   * @param yfov     vertical fov, in RAD
   * @param a        aspect ratio (width/height)
   * @param zn       near clipping plane
   * @param zf       far clipping plane
   */
  def createProjectionFrustumAccordingToOVRSDK(yfov: Float, a: Float, zn: Float, zf: Float): Mat4f = {
    val P = Mat4f.createZero()
    P.m00 =  1f / (a*math.tan(yfov/2).toFloat)
    P.m11 =  1f /   (math.tan(yfov/2).toFloat)
    P.m22 =  (zf-zn) / (zn-zf)             // numerator zf (like SDK docs) or zf-zn like in canonical OpenGL
    P.m23 = -1f
    P.m32 =  2f * (zf*zn) / (zn-zf)   // check: times 2? http://nykl.net/?page_id=175
    P
  }
  /**
   * Creates a projection matrix (according OpenGL canonical form):
   * Whether or not this gives different results in comparison to the formula
   * in the Oculus Rift documentation still unclear...
   *   http://nykl.net/?page_id=175
   *
   * The obvious difference is the parameterization, since glFrustum uses l/r/b/t instead of fov/aspect
   * The parameterization here follows exactly the OpenGL "glFrustum" function:
   *   http://www.opengl.org/sdk/docs/man2/xhtml/glFrustum.xml

   * @param l         left clipping plane
   * @param r         right clipping plane
   * @param b         bottom clipping plane
   * @param t         top clipping plane
   * @param zn        near clipping plane
   * @param zf        far clipping plane
   */
  def createProjectionFrustumOpenGLCanonical(l: Float, r: Float, b: Float, t: Float, zn: Float, zf: Float): Mat4f = {
    val P = Mat4f.createZero()
    val A = (r+l) / (r-l)   // this terms is zero for symmetrical frustums
    val B = (t+b) / (t-b)   // this terms is zero for symmetrical frustums
    val C = - (zf+zn) / (zf-zn)
    val D = - 2f*zf*zn / (zf-zn)
    P.m00 =  2f*zn / (r-l)
    P.m11 =  2f*zn / (t-b)
    P.m20 =  A
    P.m21 =  B
    P.m22 =  C
    P.m23 = -1f
    P.m32 =  D
    P
  }

  def createProjectionOrthogonal(w: Float, h: Float, zn: Float, zf: Float): Mat4f = {
    // http://stackoverflow.com/questions/688240/formula-for-a-orthogonal-projection-matrix
    val P = Mat4f.createZero()
    P.m00 =  2f / w
    P.m11 =  2f / h
    P.m22 =  1f / (zf - zn)
    P.m33 =  1f
    P.m32 =  -zn / (zf - zn)
    P
  }



}




class Mat3f(
  var m00: Float, var m10: Float, var m20: Float,
  var m01: Float, var m11: Float, var m21: Float,
  var m02: Float, var m12: Float, var m22: Float
) {

  def +(that: Mat3f): Mat3f = {
    new Mat3f(
      this.m00+that.m00, this.m10+that.m10, this.m20+that.m20,
      this.m01+that.m01, this.m11+that.m11, this.m21+that.m21,
      this.m02+that.m02, this.m12+that.m12, this.m22+that.m22
    )
  }

  def *(that: Mat3f): Mat3f = {
    val nm00 = this.m00 * that.m00 + this.m10 * that.m01 + this.m20 * that.m02
    val nm01 = this.m01 * that.m00 + this.m11 * that.m01 + this.m21 * that.m02
    val nm02 = this.m02 * that.m00 + this.m12 * that.m01 + this.m22 * that.m02
    val nm10 = this.m00 * that.m10 + this.m10 * that.m11 + this.m20 * that.m12
    val nm11 = this.m01 * that.m10 + this.m11 * that.m11 + this.m21 * that.m12
    val nm12 = this.m02 * that.m10 + this.m12 * that.m11 + this.m22 * that.m12
    val nm20 = this.m00 * that.m20 + this.m10 * that.m21 + this.m20 * that.m22
    val nm21 = this.m01 * that.m20 + this.m11 * that.m21 + this.m21 * that.m22
    val nm22 = this.m02 * that.m20 + this.m12 * that.m21 + this.m22 * that.m22
    new Mat3f(
      nm00, nm10, nm20,
      nm01, nm11, nm21,
      nm02, nm12, nm22
    )
  }
  def *(v: Vec3f): Vec3f = {
    Vec3f(
      m00*v.x + m10*v.y + m20*v.z,
      m01*v.x + m11*v.y + m21*v.z,
      m02*v.x + m12*v.y + m22*v.z
    )
  }
  def *(s: Float): Mat3f = {
    new Mat3f(
      s*m00, s*m10, s*m20,
      s*m01, s*m11, s*m21,
      s*m02, s*m12, s*m22
    )
  }

  def frobeniusDistance(that: Mat3f): Float = {
    math.sqrt(
      (this.m00-that.m00)*(this.m00-that.m00) +
      (this.m01-that.m01)*(this.m01-that.m01) +
      (this.m02-that.m02)*(this.m02-that.m02) +
      (this.m10-that.m10)*(this.m10-that.m10) +
      (this.m11-that.m11)*(this.m11-that.m11) +
      (this.m12-that.m12)*(this.m12-that.m12) +
      (this.m20-that.m20)*(this.m20-that.m20) +
      (this.m21-that.m21)*(this.m21-that.m21) +
      (this.m22-that.m22)*(this.m22-that.m22)
    ).toFloat
  }

  def asFloatBuffer(): FloatBuffer = {
    val buf = ScalaBufferUtils.createFloatBuffer(9)
    storeInBuffer(buf, true)
    buf.flip()
    return buf
  }
  def asFloatBuffer(columnMajor: Boolean): FloatBuffer = {
    val buf = ScalaBufferUtils.createFloatBuffer(9)
    storeInBuffer(buf, columnMajor)
    buf.flip()
    return buf
  }

  def storeInBuffer(buf: FloatBuffer, columnMajor: Boolean = true) {
    if (columnMajor) {
      buf.put(m00);
      buf.put(m01);
      buf.put(m02);
      buf.put(m10);
      buf.put(m11);
      buf.put(m12);
      buf.put(m20);
      buf.put(m21);
      buf.put(m22);
    } else {
      buf.put(m00);
      buf.put(m10);
      buf.put(m20);
      buf.put(m01);
      buf.put(m11);
      buf.put(m21);
      buf.put(m02);
      buf.put(m12);
      buf.put(m22);
    }
  }

  /*
  def rotate(angleX: Float, angleY: Float, angleZ: Float, leftMultiply: Boolean = false): Mat4f = {
    if (!leftMultiply) {
      return this * Mat4f.createRotationFromYawPitchRoll(angleZ, angleX, angleY)
    } else {
      return Mat4f.createRotationFromYawPitchRoll(angleZ, angleX, angleY) * this
    }
  }
  def translate(x: Float, y: Float, z: Float): Mat4f = {
    return this * Mat4f.createTranslation(x, y, z)
  }
  */

  def transpose(): Mat3f = {
    new Mat3f(
      m00, m01, m02,
      m10, m11, m12,
      m20, m21, m22
    )
  }
  /**
   * https://github.com/LWJGL/lwjgl/blob/master/src/java/org/lwjgl/util/vector/Matrix3f.java
   * http://ardoris.wordpress.com/2008/07/18/general-formula-for-the-inverse-of-a-3x3-matrix/
   */
  def inverse(): Mat3f = {
    val a = m00
    val b = m10
    val c = m20
    val d = m01
    val e = m11
    val f = m21
    val g = m02
    val h = m12
    val i = m22
    val det = 1f / (a*(e*i-f*h) - b*(d*i-f*g) + c*(d*h-e*g))
    new Mat3f(
      det*(e*i-f*h), det*(c*h-b*i), det*(b*f-c*e),
      det*(f*g-d*i), det*(a*i-c*g), det*(c*d-a*f),
      det*(d*h-e*g), det*(b*g-a*h), det*(a*e-b*d)
    )
  }

  override def toString() = f"Mat3f(\n" +
    f"  $m00%8.3f, $m10%8.3f, $m20%8.3f\n" +
    f"  $m01%8.3f, $m11%8.3f, $m21%8.3f\n" +
    f"  $m02%8.3f, $m12%8.3f, $m22%8.3f\n" +
    f")"

  def arr = Array(m00, m01, m02, m10, m11, m12, m20, m21, m22)

  def ~=(that: Mat3f)(implicit precision: MatCmpPrecision): Boolean = {
    val thisArr = this.arr
    val thatArr = that.arr
    val maxDiff = Range(0,9).map(i => math.abs(thisArr(i)-thatArr(i))).max
    return maxDiff < precision.precision
  }

}


object Mat3f {

  def createIdentity(): Mat3f = {
    new Mat3f(
      1, 0, 0,
      0, 1, 0,
      0, 0, 1
    )
  }

  def createFromMat4f(that: Mat4f): Mat3f = {
    new Mat3f(
      that.m00, that.m10, that.m20,
      that.m01, that.m11, that.m21,
      that.m02, that.m12, that.m22
    )
  }

}
*/

// ----------------------------------------------------------------------------
// HandedSystem
// ----------------------------------------------------------------------------

pub enum HandedSystem {
    R,
    L,
}

// ----------------------------------------------------------------------------
// Quaternion
// ----------------------------------------------------------------------------


pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

// ----------------------------------------------------------------------------
// Vector/Vector operations
// ----------------------------------------------------------------------------

impl Mul for &Quaternion {
    fn mul(self, that: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.w*that.x + self.x*that.w + self.y*that.z - self.z*that.y,
            y: self.w*that.y + self.y*that.w + self.z*that.x - self.x*that.z,
            z: self.w*that.z + self.z*that.w + self.x*that.y - self.y*that.x,
            w: self.w*that.w - self.x*that.x - self.y*that.y - self.z*that.z
        }
    }
    type Output = Quaternion;
}

// ----------------------------------------------------------------------------
// Scalar operations
// ----------------------------------------------------------------------------

impl Add<f32> for &Quaternion {
    fn add(self, scalar: f32) -> Quaternion {
        Quaternion {
            x: self.x + scalar,
            y: self.y + scalar,
            z: self.z + scalar,
            w: self.w + scalar,
        }
    }
    type Output = Quaternion;
}

impl Sub<f32> for &Quaternion {
    fn sub(self, scalar: f32) -> Quaternion {
        Quaternion {
            x: self.x - scalar,
            y: self.y - scalar,
            z: self.z - scalar,
            w: self.w - scalar,
        }
    }
    type Output = Quaternion;
}

impl Mul<f32> for &Quaternion {
    fn mul(self, scalar: f32) -> Quaternion {
        Quaternion {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
    type Output = Quaternion;
}

impl Div<f32> for &Quaternion {
    fn div(self, scalar: f32) -> Quaternion {
        Quaternion {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }
    type Output = Quaternion;
}

// ----------------------------------------------------------------------------
// Methods
// ----------------------------------------------------------------------------

impl Quaternion {

    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion{x: x, y: y, z: z, w: w}
    }

    /**
     * theta is assumed to be in DEG.
     */
    pub fn create(theta_deg: f32, x: f32, y: f32, z: f32) -> Quaternion {
        let theta_half = theta_deg/2f32 * PI/180f32;
        let sin_theta_half = theta_half.sin();
        let cos_theta_half = theta_half.cos();
        Quaternion::new(
            x*sin_theta_half,
            y*sin_theta_half,
            z*sin_theta_half,
            cos_theta_half,
        )
    }

    pub fn update(&mut self, that: &Quaternion) {
        self.x = that.x;
        self.y = that.y;
        self.z = that.z;
        self.w = that.w;
        // return self?
    }

    pub fn cross(&self, that: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.y*that.z - self.z*that.y,
            y: self.z*that.x - self.x*that.z,
            z: self.x*that.y - self.y*that.x,
            w: self.w*that.w - self.w*that.w,
        }
    }

    pub fn mid(&self, that: &Quaternion) -> Quaternion {
        Quaternion {
            x: 0.5f32 * (self.x + that.x),
            y: 0.5f32 * (self.y + that.y),
            z: 0.5f32 * (self.z + that.z),
            w: 0.5f32 * (self.w + that.w),
        }
    }

    pub fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt()
    }

    pub fn normalized_with_length(&self, l: f32) -> Quaternion {
        return self * (l / self.length())
    }

    pub fn normalized(&self) -> Quaternion {
        return self / self.length()
    }

    pub fn normalized_in_place(&mut self) {
        let length = self.length();
        self.x = self.x / length;
        self.y = self.y / length;
        self.z = self.z / length;
        self.w = self.w / length;
    }

    pub fn inverse(&self) -> Quaternion {
        Quaternion {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w:  self.w,
        }
    }

    pub fn inverse_in_place(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self.w =  self.w;
    }

    pub fn cast_to_orientation_matrix(&self) -> Mat4f {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let w = self.w;
        Mat4f::new(
            1f32-2f32*y*y-2f32*z*z,      2f32*x*y-2f32*w*z,      2f32*x*z+2f32*w*y,   0f32,
                 2f32*x*y+2f32*w*z, 1f32-2f32*x*x-2f32*z*z,      2f32*y*z-2f32*w*x,   0f32,
                 2f32*x*z-2f32*w*y,      2f32*y*z+2f32*w*x, 1f32-2f32*x*x-2f32*y*y,   0f32,
                              0f32,                   0f32,                   0f32,   1f32,
        )
    }
    pub fn cast_to_orientation_matrix_rh(&self) -> Mat4f {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        let w = self.w;
        Mat4f::new(
            1f32-2f32*y*y-2f32*z*z,      2f32*x*y+2f32*w*z,      2f32*x*z-2f32*w*y,   0f32,
                 2f32*x*y-2f32*w*z, 1f32-2f32*x*x-2f32*z*z,      2f32*y*z+2f32*w*x,   0f32,
                 2f32*x*z+2f32*w*y,      2f32*y*z-2f32*w*x, 1f32-2f32*x*x-2f32*y*y,   0f32,
                              0f32,                   0f32,                   0f32,   1f32,
        )
    }

}




/*




sealed trait HandedSystem {
  val value: Float
}
object HandedSystem {
  object R extends HandedSystem { val value = +1f }
  object L extends HandedSystem { val value = -1f }
}

sealed trait RotateDirection{
  val value: Float
}
object RotateDirection {
  object CCW extends RotateDirection { val value = +1f }
  object CW  extends RotateDirection { val value = -1f }
}

/*
 * Before I was using a value class like that, but I think sealed trait is nicer here.
 *
class RotateDirection(val value: Float) extends AnyVal
object RotateDirection {
  val CCW = new RotateDirection(+1f)
  val CW  = new RotateDirection(-1f)
}
*/



/**
 * Quaternion implementation to handle rotations without gimbal lock
 */
case class Quaternion(var x: Float, var y: Float, var z: Float, var w: Float) {
  def *(that: Quaternion): Quaternion = {
    new Quaternion(
      this.w*that.x + this.x*that.w + this.y*that.z - this.z*that.y,
      this.w*that.y + this.y*that.w + this.z*that.x - this.x*that.z,
      this.w*that.z + this.z*that.w + this.x*that.y - this.y*that.x,
      this.w*that.w - this.x*that.x - this.y*that.y - this.z*that.z
    )
  }

  def :<=(that: Quaternion) {
    this.x = that.x
    this.y = that.y
    this.z = that.z
    this.w = that.w
  }

  def norm(): Quaternion = {
    val sum = math.sqrt(x*x+y*y+z*z+w*w).toFloat
    Quaternion(x/sum, y/sum, z/sum, w/sum)
  }
  def normMut() {
    val sum = math.sqrt(x*x+y*y+z*z+w*w).toFloat
    x /= sum
    y /= sum
    z /= sum
    w /= sum
  }

  def inv(): Quaternion = Quaternion(-x, -y, -z, w)
  def invMut() {
    x = -x
    y = -y
    z = -z
  }

  /**
   * inspired by: http://www.euclideanspace.com/maths/geometry/rotations/conversions/quaternionToEuler/
   * seems to be the wrong convention...
   */
  def toEulerV1(): EulerAngles = {
    val test = x*y + z*w
    if (test > 0.499) { // singularity at north pole
      val yaw   = 2 * math.atan2(x, w)
      val roll  = Math.PI/2
      val pitch = 0f
      return EulerAngles(yaw.toFloat*180/Math.PI.toFloat, pitch.toFloat*180/Math.PI.toFloat, roll.toFloat*180/Math.PI.toFloat)
    }
    if (test < -0.499) { // singularity at south pole
      val yaw   = -2 * Math.atan2(x, w)
      val roll  = -Math.PI/2
      val pitch = 0
      return EulerAngles(yaw.toFloat*180/Math.PI.toFloat, pitch.toFloat*180/Math.PI.toFloat, roll.toFloat*180/Math.PI.toFloat)
    }
    val sqx = x * x
    val sqy = y * y
    val sqz = z * z
    val yaw   = Math.atan2(2*y*w-2*x*z, 1 - 2*sqy - 2*sqz)
    val roll  = Math.asin(2*test)
    val pitch = Math.atan2(2*x*w-2*y*z, 1 - 2*sqx - 2*sqz)
    return EulerAngles(yaw.toFloat*180/Math.PI.toFloat, pitch.toFloat*180/Math.PI.toFloat, roll.toFloat*180/Math.PI.toFloat)
  }

  /**
   * Adapted from:
   *   http://stackoverflow.com/a/18115837/1804173
   * also not the right convention...
   */
  def toEulerV2(): EulerAngles = {
    val yaw   = math.atan2(2.0*(y*z + w*x), w*w - x*x - y*y + z*z)
    val pitch = math.asin(-2.0*(x*z - w*y))
    val roll  = math.atan2(2.0*(x*y + w*z), w*w + x*x - y*y - z*z)
    return EulerAngles(yaw.toFloat*180/Math.PI.toFloat, pitch.toFloat*180/Math.PI.toFloat, roll.toFloat*180/Math.PI.toFloat)
  }

  /**
   * Adapted from:
   *   http://answers.unity3d.com/questions/416169/finding-pitchrollyaw-from-quaternions.html
   * also wrong...
   */
  def toEulerV3(): EulerAngles = {
    //val roll  = math.atan2(2*y*w - 2*x*z, 1 - 2*y*y - 2*z*z);
    //val pitch = math.atan2(2*x*w - 2*y*z, 1 - 2*x*x - 2*z*z);
    //val yaw   = math.asin(2*x*y + 2*z*w);
    val yaw   =  math.atan2(2*x*y + 2*w*z, w*w + x*x - y*y - z*z)
    val pitch = -math.asin(2*w*y - 2*x*z)
    val roll  = -math.atan2(2*y*z + 2*w*x, -w*w + x*x + y*y - z*z)
    return EulerAngles(yaw.toFloat*180/Math.PI.toFloat, pitch.toFloat*180/Math.PI.toFloat, roll.toFloat*180/Math.PI.toFloat)
  }

  /**
   * Let's do it right -- this is adaped from the SDK: src/kernel/OVR_Math.h
   */
  def toEuler(A1: Int = 1, A2: Int = 0, A3: Int = 2, D: RotateDirection = RotateDirection.CCW, S: HandedSystem = HandedSystem.R): EulerAngles = {
    assert(A1 != A2 && A2 != A3 && A1 != A3)

    val MATH_DOUBLE_SINGULARITYRADIUS = 0.000000000001
    val MATH_DOUBLE_PIOVER2 = math.Pi / 2

    val Q = Array(x, y, z)
    val ww  = w*w;
    val Q11 = Q(A1)*Q(A1);
    val Q22 = Q(A2)*Q(A2);
    val Q33 = Q(A3)*Q(A3);

    // Determine whether even permutation
    val psign = if (((A1 + 1) % 3 == A2) && ((A2 + 1) % 3 == A3))
        +1
      else
        -1

    val s2 = psign * 2 * (psign*w*Q(A2) + Q(A1)*Q(A3))

    var a = 0d
    var b = 0d
    var c = 0d

    if (s2 < -1 + MATH_DOUBLE_SINGULARITYRADIUS)
    { // South pole singularity
        a = 0
        b = -S.value*D.value*MATH_DOUBLE_PIOVER2;
        c = S.value*D.value*math.atan2(2*(psign*Q(A1)*Q(A2) + w*Q(A3)),
                   ww + Q22 - Q11 - Q33 );
    }
    else if (s2 > 1 - MATH_DOUBLE_SINGULARITYRADIUS)
    {  // North pole singularity
        a = 0
        b = S.value*D.value*MATH_DOUBLE_PIOVER2;
        c = S.value*D.value*math.atan2(2*(psign*Q(A1)*Q(A2) + w*Q(A3)),
                   ww + Q22 - Q11 - Q33);
    }
    else
    {
        a = -S.value*D.value*math.atan2(-2*(w*Q(A1) - psign*Q(A2)*Q(A3)),
                    ww + Q33 - Q11 - Q22);
        b = S.value*D.value*math.asin(s2);
        c = S.value*D.value*math.atan2(2*(w*Q(A3) - psign*Q(A1)*Q(A2)),
                   ww + Q11 - Q22 - Q33);
    }
    return EulerAngles(a.toFloat*180/Math.PI.toFloat, b.toFloat*180/Math.PI.toFloat, c.toFloat*180/Math.PI.toFloat)

  }
  /*
    // GetEulerAngles extracts Euler angles from the quaternion, in the specified order of
    // axis rotations and the specified coordinate system. Right-handed coordinate system
    // is the default, with CCW rotations while looking in the negative axis direction.
    // Here a,b,c, are the Yaw/Pitch/Roll angles to be returned.
    // rotation a around axis A1
    // is followed by rotation b around axis A2
    // is followed by rotation c around axis A3
    // rotations are CCW or CW (D) in LH or RH coordinate system (S)
  template <Axis A1, Axis A2, Axis A3, RotateDirection D, HandedSystem S>
    void GetEulerAngles(T *a, T *b, T *c) const
    {
        static_assert((A1 != A2) && (A2 != A3) && (A1 != A3), "(A1 != A2) && (A2 != A3) && (A1 != A3)");

        T Q[3] = { x, y, z };  //Quaternion components x,y,z

        T ww  = w*w;
        T Q11 = Q[A1]*Q[A1];
        T Q22 = Q[A2]*Q[A2];
        T Q33 = Q[A3]*Q[A3];

        T psign = T(-1);
        // Determine whether even permutation
        if (((A1 + 1) % 3 == A2) && ((A2 + 1) % 3 == A3))
            psign = T(1);

        T s2 = psign * T(2) * (psign*w*Q[A2] + Q[A1]*Q[A3]);

        if (s2 < T(-1) + ((T)MATH_DOUBLE_SINGULARITYRADIUS))
        { // South pole singularity
            *a = T(0);
            *b = -S*D*((T)MATH_DOUBLE_PIOVER2);
            *c = S*D*atan2(T(2)*(psign*Q[A1]*Q[A2] + w*Q[A3]),
                       ww + Q22 - Q11 - Q33 );
        }
        else if (s2 > T(1) - ((T)MATH_DOUBLE_SINGULARITYRADIUS))
        {  // North pole singularity
            *a = T(0);
            *b = S*D*((T)MATH_DOUBLE_PIOVER2);
            *c = S*D*atan2(T(2)*(psign*Q[A1]*Q[A2] + w*Q[A3]),
                       ww + Q22 - Q11 - Q33);
        }
        else
        {
            *a = -S*D*atan2(T(-2)*(w*Q[A1] - psign*Q[A2]*Q[A3]),
                        ww + Q33 - Q11 - Q22);
            *b = S*D*asin(s2);
            *c = S*D*atan2(T(2)*(w*Q[A3] - psign*Q[A1]*Q[A2]),
                       ww + Q11 - Q22 - Q33);
        }
        return;
    }
   */
}

object Quaternion {

  /**
   * m must be a pure rotation (orthogonal), no scale/shear allowed
   *
   * http://www.cs.princeton.edu/~gewang/projects/darth/stuff/quat_faq.html#Q55
   */
  def createFromRotationMatrix(m: Mat3f): Quaternion = {

    val trace = m.m00 + m.m11 + m.m22 + 1

    if (trace > 0.0000001) {
      val s = math.sqrt(trace).toFloat * 2
      val x = (m.m12 - m.m21) / s
      val y = (m.m20 - m.m02) / s
      val z = (m.m01 - m.m10) / s
      val w = 0.25f * s
      new Quaternion(x, y, z, w)
    } else if (m.m00 > m.m11 && m.m00 > m.m22) {
      val s = math.sqrt(1f + m.m00 - m.m11 - m.m22).toFloat * 2
      val x = 0.25f * s
      val y = (m.m01 + m.m10) / s
      val z = (m.m20 + m.m02) / s
      val w = (m.m12 - m.m21) / s
      new Quaternion(x, y, z, w)
    } else if (m.m11 > m.m22) {
      val s = math.sqrt(1f + m.m11 - m.m00 - m.m22).toFloat * 2
      val x = (m.m01 + m.m10) / s
      val y = 0.25f * s
      val z = (m.m12 + m.m21) / s
      val w = (m.m20 - m.m02) / s
      new Quaternion(x, y, z, w)
    } else {
      val s = math.sqrt(1f + m.m22 - m.m00 - m.m11).toFloat * 2
      val x = (m.m20 + m.m02) / s
      val y = (m.m12 + m.m21) / s
      val z = 0.25f * s
      val w = (m.m01 - m.m10) / s
      new Quaternion(x, y, z, w)
    }
  }
}

case class EulerAngles(yaw: Float, pitch: Float, roll: Float) {
  override def toString(): String = f"EulerAngles(yaw = $yaw%8.3f, pitch = $pitch%8.3f, roll = $roll%8.3f)"
}


*/