use super::*;

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
    pub fn zero() -> Self {
        Self { x: 0f32, y: 0f32 }
    }
    pub fn one() -> Self {
        Self { x: 1f32, y: 1f32 }
    }
    pub fn unit_x() -> Self {
        Self { x: 1.0, y: 0.0 }
    }
    pub fn unit_y() -> Self {
        Self { x: 0.0, y: 1.0 }
    }
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    #[cfg(feature = "std")]
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    #[cfg(feature = "std")]
    pub fn distance(&self, value: &Self) -> f32 {
        (self - value).length()
    }
    pub fn distance_squared(&self, value: &Self) -> f32 {
        (self - value).length_squared()
    }
    #[cfg(feature = "std")]
    pub fn normalize(&self) -> Self {
        self / self.length()
    }

    fn impl_neg(&self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
    fn impl_add(&self, rhs: &Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
    fn impl_sub(&self, rhs: &Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
    fn impl_div(&self, rhs: &Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
    fn impl_div_f32(&self, rhs: f32) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
    fn impl_mul(&self, rhs: &Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
    fn impl_mul_f32(&self, rhs: f32) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl core::ops::Neg for Vector2 {
    type Output = Self;
    fn neg(self) -> Self {
        self.impl_neg()
    }
}
impl core::ops::Neg for &Vector2 {
    type Output = Vector2;
    fn neg(self) -> Vector2 {
        self.impl_neg()
    }
}
impl core::ops::Add<Self> for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Self> for Vector2 {
    type Output = Self;
    fn add(self, rhs: &Self) -> Self {
        self.impl_add(rhs)
    }
}
impl core::ops::Add<Vector2> for &Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        self.impl_add(&rhs)
    }
}
impl core::ops::Add<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn add(self, rhs: &Vector2) -> Vector2 {
        self.impl_add(rhs)
    }
}
impl core::ops::Sub<Self> for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Self> for Vector2 {
    type Output = Self;
    fn sub(self, rhs: &Self) -> Self {
        self.impl_sub(rhs)
    }
}
impl core::ops::Sub<Vector2> for &Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        self.impl_sub(&rhs)
    }
}
impl core::ops::Sub<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: &Vector2) -> Vector2 {
        self.impl_sub(rhs)
    }
}
impl core::ops::Div<Self> for Vector2 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        self.impl_div(&rhs)
    }
}
impl core::ops::Div<&Self> for Vector2 {
    type Output = Self;
    fn div(self, rhs: &Self) -> Self {
        self.impl_div(rhs)
    }
}
impl core::ops::Div<Vector2> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: Vector2) -> Vector2 {
        self.impl_div(&rhs)
    }
}
impl core::ops::Div<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: &Vector2) -> Vector2 {
        self.impl_div(rhs)
    }
}
impl core::ops::Div<f32> for Vector2 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self {
        self.impl_div_f32(rhs)
    }
}
impl core::ops::Div<f32> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Vector2 {
        self.impl_div_f32(rhs)
    }
}
impl core::ops::Mul<Self> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Self> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: &Self) -> Self {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<Vector2> for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        self.impl_mul(&rhs)
    }
}
impl core::ops::Mul<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: &Vector2) -> Vector2 {
        self.impl_mul(rhs)
    }
}
impl core::ops::Mul<f32> for Vector2 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self {
        self.impl_mul_f32(rhs)
    }
}
impl core::ops::Mul<f32> for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Vector2 {
        self.impl_mul_f32(rhs)
    }
}
