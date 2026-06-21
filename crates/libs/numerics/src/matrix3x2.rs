use super::*;

impl Matrix3x2 {
    /// Returns the identity matrix (no transform).
    pub const fn identity() -> Self {
        Self {
            m11: 1.0,
            m12: 0.0,
            m21: 0.0,
            m22: 1.0,
            m31: 0.0,
            m32: 0.0,
        }
    }
    /// Returns a matrix that translates by `(x, y)`.
    pub const fn translation(x: f32, y: f32) -> Self {
        Self {
            m11: 1.0,
            m12: 0.0,
            m21: 0.0,
            m22: 1.0,
            m31: x,
            m32: y,
        }
    }
    /// Returns a matrix that rotates by `angle` degrees about the origin.
    #[cfg(feature = "std")]
    pub fn rotation(angle: f32) -> Self {
        Self::rotation_around(angle, Vector2::zero())
    }
    /// Returns a matrix that rotates by `angle` degrees about `center`.
    #[cfg(feature = "std")]
    pub fn rotation_around(angle: f32, center: Vector2) -> Self {
        let (sin, cos) = angle.to_radians().sin_cos();
        Self {
            m11: cos,
            m12: sin,
            m21: -sin,
            m22: cos,
            m31: center.x * (1.0 - cos) + center.y * sin,
            m32: center.y * (1.0 - cos) - center.x * sin,
        }
    }
    /// Returns a matrix that scales by `scale_x` and `scale_y` about the origin.
    pub fn scale(scale_x: f32, scale_y: f32) -> Self {
        Self::scale_around(scale_x, scale_y, Vector2::zero())
    }
    /// Returns a matrix that scales by `scale_x` and `scale_y` about `center`.
    pub fn scale_around(scale_x: f32, scale_y: f32, center: Vector2) -> Self {
        Self {
            m11: scale_x,
            m12: 0.0,
            m21: 0.0,
            m22: scale_y,
            m31: center.x - scale_x * center.x,
            m32: center.y - scale_y * center.y,
        }
    }
    /// Returns a matrix that skews by `angle_x` and `angle_y` degrees about
    /// the origin.
    #[cfg(feature = "std")]
    pub fn skew(angle_x: f32, angle_y: f32) -> Self {
        Self::skew_around(angle_x, angle_y, Vector2::zero())
    }
    /// Returns a matrix that skews by `angle_x` and `angle_y` degrees about
    /// `center`.
    #[cfg(feature = "std")]
    pub fn skew_around(angle_x: f32, angle_y: f32, center: Vector2) -> Self {
        let tan_x = angle_x.to_radians().tan();
        let tan_y = angle_y.to_radians().tan();
        Self {
            m11: 1.0,
            m12: tan_y,
            m21: tan_x,
            m22: 1.0,
            m31: -center.y * tan_x,
            m32: -center.x * tan_y,
        }
    }
    fn impl_add(&self, rhs: &Self) -> Self {
        Self {
            m11: self.m11 + rhs.m11,
            m12: self.m12 + rhs.m12,
            m21: self.m21 + rhs.m21,
            m22: self.m22 + rhs.m22,
            m31: self.m31 + rhs.m31,
            m32: self.m32 + rhs.m32,
        }
    }
    fn impl_sub(&self, rhs: &Self) -> Self {
        Self {
            m11: self.m11 - rhs.m11,
            m12: self.m12 - rhs.m12,
            m21: self.m21 - rhs.m21,
            m22: self.m22 - rhs.m22,
            m31: self.m31 - rhs.m31,
            m32: self.m32 - rhs.m32,
        }
    }
    fn impl_mul(&self, rhs: &Self) -> Self {
        Self {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,
            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + rhs.m31,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + rhs.m32,
        }
    }
    fn impl_mul_f32(&self, rhs: f32) -> Self {
        Self {
            m11: self.m11 * rhs,
            m12: self.m12 * rhs,
            m21: self.m21 * rhs,
            m22: self.m22 * rhs,
            m31: self.m31 * rhs,
            m32: self.m32 * rhs,
        }
    }
}

impl core::ops::Add<Self> for Matrix3x2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Self> for Matrix3x2 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        self.impl_add(rhs)
    }
}
impl core::ops::Add<Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn add(self, rhs: Matrix3x2) -> Matrix3x2 {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn add(self, rhs: &Matrix3x2) -> Matrix3x2 {
        self.impl_add(rhs)
    }
}
impl core::ops::Sub<Self> for Matrix3x2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Self> for Matrix3x2 {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self {
        self.impl_sub(rhs)
    }
}
impl core::ops::Sub<Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn sub(self, rhs: Matrix3x2) -> Matrix3x2 {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn sub(self, rhs: &Matrix3x2) -> Matrix3x2 {
        self.impl_sub(rhs)
    }
}
impl core::ops::Mul<Self> for Matrix3x2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Self> for Matrix3x2 {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: Matrix3x2) -> Matrix3x2 {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: &Matrix3x2) -> Matrix3x2 {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<f32> for Matrix3x2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        self.impl_mul_f32(rhs)
    }
}
impl core::ops::Mul<f32> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: f32) -> Matrix3x2 {
        self.impl_mul_f32(rhs)
    }
}
