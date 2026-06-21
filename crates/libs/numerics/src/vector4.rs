use super::*;

impl Vector4 {
    /// Creates a vector from its `x`, `y`, `z`, and `w` components.
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
    /// Returns the vector `(0, 0, 0, 0)`.
    pub fn zero() -> Self {
        Self {
            x: 0f32,
            y: 0f32,
            z: 0f32,
            w: 0f32,
        }
    }
    /// Returns the vector `(1, 1, 1, 1)`.
    pub fn one() -> Self {
        Self {
            x: 1f32,
            y: 1f32,
            z: 1f32,
            w: 1f32,
        }
    }
    /// Returns the unit vector `(1, 0, 0, 0)`.
    pub fn unit_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
    /// Returns the unit vector `(0, 1, 0, 0)`.
    pub fn unit_y() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        }
    }
    /// Returns the unit vector `(0, 0, 1, 0)`.
    pub fn unit_z() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 0.0,
        }
    }
    /// Returns the unit vector `(0, 0, 0, 1)`.
    pub fn unit_w() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
    /// Returns the dot product of `self` and `rhs`.
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
    /// Returns the squared length of the vector, avoiding the square root
    /// computed by [`length`](Self::length).
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    /// Returns the length (magnitude) of the vector.
    #[cfg(feature = "std")]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    /// Returns the distance between the points `self` and `value`.
    #[cfg(feature = "std")]
    pub fn distance(&self, value: &Self) -> f32 {
        (self - value).length()
    }
    /// Returns the squared distance between the points `self` and `value`.
    pub fn distance_squared(&self, value: &Self) -> f32 {
        (self - value).length_squared()
    }
    /// Returns a unit vector pointing in the same direction as `self`.
    #[cfg(feature = "std")]
    pub fn normalize(&self) -> Self {
        self / self.length()
    }

    fn impl_neg(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
    fn impl_add(&self, rhs: &Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
    fn impl_sub(&self, rhs: &Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
    fn impl_div(&self, rhs: &Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
    fn impl_div_f32(&self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
    fn impl_mul(&self, rhs: &Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
    fn impl_mul_f32(&self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl core::ops::Neg for Vector4 {
    type Output = Self;
    fn neg(self) -> Self {
        self.impl_neg()
    }
}
impl core::ops::Neg for &Vector4 {
    type Output = Vector4;
    fn neg(self) -> Vector4 {
        self.impl_neg()
    }
}
impl core::ops::Add<Self> for Vector4 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Self> for Vector4 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        self.impl_add(rhs)
    }
}
impl core::ops::Add<Vector4> for &Vector4 {
    type Output = Vector4;
    fn add(self, rhs: Vector4) -> Vector4 {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn add(self, rhs: &Vector4) -> Vector4 {
        self.impl_add(rhs)
    }
}
impl core::ops::Sub<Self> for Vector4 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Self> for Vector4 {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self {
        self.impl_sub(rhs)
    }
}
impl core::ops::Sub<Vector4> for &Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: Vector4) -> Vector4 {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: &Vector4) -> Vector4 {
        self.impl_sub(rhs)
    }
}
impl core::ops::Div<Self> for Vector4 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self.impl_div(&rhs)
    }
}
impl core::ops::Div<&Self> for Vector4 {
    type Output = Self;
    fn div(self, rhs: &Self) -> Self {
        self.impl_div(rhs)
    }
}
impl core::ops::Div<Vector4> for &Vector4 {
    type Output = Vector4;
    fn div(self, rhs: Vector4) -> Vector4 {
        self.impl_div(&rhs)
    }
}
impl core::ops::Div<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn div(self, rhs: &Vector4) -> Vector4 {
        self.impl_div(rhs)
    }
}
impl core::ops::Div<f32> for Vector4 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        self.impl_div_f32(rhs)
    }
}
impl core::ops::Div<f32> for &Vector4 {
    type Output = Vector4;
    fn div(self, rhs: f32) -> Vector4 {
        self.impl_div_f32(rhs)
    }
}
impl core::ops::Mul<Self> for Vector4 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Self> for Vector4 {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<Vector4> for &Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Vector4 {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: &Vector4) -> Vector4 {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<f32> for Vector4 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        self.impl_mul_f32(rhs)
    }
}
impl core::ops::Mul<f32> for &Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: f32) -> Vector4 {
        self.impl_mul_f32(rhs)
    }
}
