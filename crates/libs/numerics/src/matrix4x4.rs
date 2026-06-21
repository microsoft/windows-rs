use super::*;

impl Matrix4x4 {
    /// Returns a matrix that translates by `(x, y, z)`.
    pub const fn translation(x: f32, y: f32, z: f32) -> Self {
        Self {
            m11: 1.0,
            m12: 0.0,
            m13: 0.0,
            m14: 0.0,
            m21: 0.0,
            m22: 1.0,
            m23: 0.0,
            m24: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 1.0,
            m34: 0.0,
            m41: x,
            m42: y,
            m43: z,
            m44: 1.0,
        }
    }
    /// Returns a matrix that rotates by `degree` degrees about the y-axis.
    #[cfg(feature = "std")]
    pub fn rotation_y(degree: f32) -> Self {
        let (sin, cos) = degree.to_radians().sin_cos();
        Self {
            m11: cos,
            m12: 0.0,
            m13: -sin,
            m14: 0.0,
            m21: 0.0,
            m22: 1.0,
            m23: 0.0,
            m24: 0.0,
            m31: sin,
            m32: 0.0,
            m33: cos,
            m34: 0.0,
            m41: 0.0,
            m42: 0.0,
            m43: 0.0,
            m44: 1.0,
        }
    }
    /// Returns a perspective projection matrix for the given `depth`. A
    /// non-positive `depth` produces no perspective foreshortening.
    pub fn perspective_projection(depth: f32) -> Self {
        let projection = if depth > 0.0 { -1.0 / depth } else { 0.0 };
        Self {
            m11: 1.0,
            m12: 0.0,
            m13: 0.0,
            m14: 0.0,
            m21: 0.0,
            m22: 1.0,
            m23: 0.0,
            m24: 0.0,
            m31: 0.0,
            m32: 0.0,
            m33: 1.0,
            m34: projection,
            m41: 0.0,
            m42: 0.0,
            m43: 0.0,
            m44: 1.0,
        }
    }
    fn impl_add(&self, rhs: &Self) -> Self {
        Self {
            m11: self.m11 + rhs.m11,
            m12: self.m12 + rhs.m12,
            m13: self.m13 + rhs.m13,
            m14: self.m14 + rhs.m14,
            m21: self.m21 + rhs.m21,
            m22: self.m22 + rhs.m22,
            m23: self.m23 + rhs.m23,
            m24: self.m24 + rhs.m24,
            m31: self.m31 + rhs.m31,
            m32: self.m32 + rhs.m32,
            m33: self.m33 + rhs.m33,
            m34: self.m34 + rhs.m34,
            m41: self.m41 + rhs.m41,
            m42: self.m42 + rhs.m42,
            m43: self.m43 + rhs.m43,
            m44: self.m44 + rhs.m44,
        }
    }
    fn impl_sub(&self, rhs: &Self) -> Self {
        Self {
            m11: self.m11 - rhs.m11,
            m12: self.m12 - rhs.m12,
            m13: self.m13 - rhs.m13,
            m14: self.m14 - rhs.m14,
            m21: self.m21 - rhs.m21,
            m22: self.m22 - rhs.m22,
            m23: self.m23 - rhs.m23,
            m24: self.m24 - rhs.m24,
            m31: self.m31 - rhs.m31,
            m32: self.m32 - rhs.m32,
            m33: self.m33 - rhs.m33,
            m34: self.m34 - rhs.m34,
            m41: self.m41 - rhs.m41,
            m42: self.m42 - rhs.m42,
            m43: self.m43 - rhs.m43,
            m44: self.m44 - rhs.m44,
        }
    }
    fn impl_mul(&self, rhs: &Self) -> Self {
        Self {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21 + self.m13 * rhs.m31 + self.m14 * rhs.m41,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22 + self.m13 * rhs.m32 + self.m14 * rhs.m42,
            m13: self.m11 * rhs.m13 + self.m12 * rhs.m23 + self.m13 * rhs.m33 + self.m14 * rhs.m43,
            m14: self.m11 * rhs.m14 + self.m12 * rhs.m24 + self.m13 * rhs.m34 + self.m14 * rhs.m44,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21 + self.m23 * rhs.m31 + self.m24 * rhs.m41,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22 + self.m23 * rhs.m32 + self.m24 * rhs.m42,
            m23: self.m21 * rhs.m13 + self.m22 * rhs.m23 + self.m23 * rhs.m33 + self.m24 * rhs.m43,
            m24: self.m21 * rhs.m14 + self.m22 * rhs.m24 + self.m23 * rhs.m34 + self.m24 * rhs.m44,
            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + self.m33 * rhs.m31 + self.m34 * rhs.m41,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + self.m33 * rhs.m32 + self.m34 * rhs.m42,
            m33: self.m31 * rhs.m13 + self.m32 * rhs.m23 + self.m33 * rhs.m33 + self.m34 * rhs.m43,
            m34: self.m31 * rhs.m14 + self.m32 * rhs.m24 + self.m33 * rhs.m34 + self.m34 * rhs.m44,
            m41: self.m41 * rhs.m11 + self.m42 * rhs.m21 + self.m43 * rhs.m31 + self.m44 * rhs.m41,
            m42: self.m41 * rhs.m12 + self.m42 * rhs.m22 + self.m43 * rhs.m32 + self.m44 * rhs.m42,
            m43: self.m41 * rhs.m13 + self.m42 * rhs.m23 + self.m43 * rhs.m33 + self.m44 * rhs.m43,
            m44: self.m41 * rhs.m14 + self.m42 * rhs.m24 + self.m43 * rhs.m34 + self.m44 * rhs.m44,
        }
    }
    fn impl_mul_f32(&self, rhs: f32) -> Self {
        Self {
            m11: self.m11 * rhs,
            m12: self.m12 * rhs,
            m13: self.m13 * rhs,
            m14: self.m14 * rhs,
            m21: self.m21 * rhs,
            m22: self.m22 * rhs,
            m23: self.m23 * rhs,
            m24: self.m24 * rhs,
            m31: self.m31 * rhs,
            m32: self.m32 * rhs,
            m33: self.m33 * rhs,
            m34: self.m34 * rhs,
            m41: self.m41 * rhs,
            m42: self.m42 * rhs,
            m43: self.m43 * rhs,
            m44: self.m44 * rhs,
        }
    }
}

impl core::ops::Add<Self> for Matrix4x4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Self> for Matrix4x4 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        self.impl_add(rhs)
    }
}
impl core::ops::Add<Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_add(rhs)
    }
}
impl core::ops::Sub<Self> for Matrix4x4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Self> for Matrix4x4 {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self {
        self.impl_sub(rhs)
    }
}
impl core::ops::Sub<Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_sub(rhs)
    }
}
impl core::ops::Mul<Self> for Matrix4x4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Self> for Matrix4x4 {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<f32> for Matrix4x4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        self.impl_mul_f32(rhs)
    }
}
impl core::ops::Mul<f32> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: f32) -> Matrix4x4 {
        self.impl_mul_f32(rhs)
    }
}
