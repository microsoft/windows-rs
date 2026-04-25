use super::*;

impl Matrix3x2 {
    pub const fn identity() -> Self {
        Self {
            M11: 1.0,
            M12: 0.0,
            M21: 0.0,
            M22: 1.0,
            M31: 0.0,
            M32: 0.0,
        }
    }
    pub const fn translation(x: f32, y: f32) -> Self {
        Self {
            M11: 1.0,
            M12: 0.0,
            M21: 0.0,
            M22: 1.0,
            M31: x,
            M32: y,
        }
    }
    #[cfg(feature = "std")]
    pub fn rotation(angle: f32) -> Self {
        Self::rotation_around(angle, Vector2::zero())
    }
    #[cfg(feature = "std")]
    pub fn rotation_around(angle: f32, center: Vector2) -> Self {
        let (sin, cos) = angle.to_radians().sin_cos();
        Self {
            M11: cos,
            M12: sin,
            M21: -sin,
            M22: cos,
            M31: center.X * (1.0 - cos) + center.Y * sin,
            M32: center.Y * (1.0 - cos) - center.X * sin,
        }
    }
    pub fn scale(scale_x: f32, scale_y: f32) -> Self {
        Self::scale_around(scale_x, scale_y, Vector2::zero())
    }
    pub fn scale_around(scale_x: f32, scale_y: f32, center: Vector2) -> Self {
        Self {
            M11: scale_x,
            M12: 0.0,
            M21: 0.0,
            M22: scale_y,
            M31: center.X - scale_x * center.X,
            M32: center.Y - scale_y * center.Y,
        }
    }
    #[cfg(feature = "std")]
    pub fn skew(angle_x: f32, angle_y: f32) -> Self {
        Self::skew_around(angle_x, angle_y, Vector2::zero())
    }
    #[cfg(feature = "std")]
    pub fn skew_around(angle_x: f32, angle_y: f32, center: Vector2) -> Self {
        let tan_x = angle_x.to_radians().tan();
        let tan_y = angle_y.to_radians().tan();
        Self {
            M11: 1.0,
            M12: tan_y,
            M21: tan_x,
            M22: 1.0,
            M31: -center.Y * tan_x,
            M32: -center.X * tan_y,
        }
    }
    fn impl_add(&self, rhs: &Self) -> Self {
        Self {
            M11: self.M11 + rhs.M11,
            M12: self.M12 + rhs.M12,
            M21: self.M21 + rhs.M21,
            M22: self.M22 + rhs.M22,
            M31: self.M31 + rhs.M31,
            M32: self.M32 + rhs.M32,
        }
    }
    fn impl_sub(&self, rhs: &Self) -> Self {
        Self {
            M11: self.M11 - rhs.M11,
            M12: self.M12 - rhs.M12,
            M21: self.M21 - rhs.M21,
            M22: self.M22 - rhs.M22,
            M31: self.M31 - rhs.M31,
            M32: self.M32 - rhs.M32,
        }
    }
    fn impl_mul(&self, rhs: &Self) -> Self {
        Self {
            M11: self.M11 * rhs.M11 + self.M12 * rhs.M21,
            M12: self.M11 * rhs.M12 + self.M12 * rhs.M22,
            M21: self.M21 * rhs.M11 + self.M22 * rhs.M21,
            M22: self.M21 * rhs.M12 + self.M22 * rhs.M22,
            M31: self.M31 * rhs.M11 + self.M32 * rhs.M21 + rhs.M31,
            M32: self.M31 * rhs.M12 + self.M32 * rhs.M22 + rhs.M32,
        }
    }
    fn impl_mul_f32(&self, rhs: f32) -> Self {
        Self {
            M11: self.M11 * rhs,
            M12: self.M12 * rhs,
            M21: self.M21 * rhs,
            M22: self.M22 * rhs,
            M31: self.M31 * rhs,
            M32: self.M32 * rhs,
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
