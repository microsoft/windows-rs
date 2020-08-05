use winrt::foundation::numerics::{Matrix3x2, Matrix4x4, Vector2, Vector3, Vector4};

// Vector2
impl ::std::ops::Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl ::std::ops::Add<&Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl ::std::ops::Add<Vector2> for &Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl ::std::ops::Add<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn add(self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl ::std::ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ::std::ops::Sub<&Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ::std::ops::Sub<Vector2> for &Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ::std::ops::Sub<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ::std::ops::Div<Vector2> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl ::std::ops::Div<&Vector2> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl ::std::ops::Div<Vector2> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl ::std::ops::Div<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}
impl ::std::ops::Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl ::std::ops::Div<f32> for &Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl ::std::ops::Mul<Vector2> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl ::std::ops::Mul<&Vector2> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl ::std::ops::Mul<Vector2> for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: Vector2) -> Vector2 {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl ::std::ops::Mul<&Vector2> for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: &Vector2) -> Vector2 {
        Vector2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}
impl ::std::ops::Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl ::std::ops::Mul<f32> for &Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f32) -> Vector2 {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl Vector2 {
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
    pub fn dot(&self, rhs: &Vector2) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn distance(&self, value: &Vector2) -> f32 {
        (self - value).length()
    }
    pub fn distance_squared(&self, value: &Vector2) -> f32 {
        (self - value).length_squared()
    }
    pub fn normalize(&self) -> Self {
        self / self.length()
    }
}

// Vector3
impl ::std::ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl ::std::ops::Add<&Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl ::std::ops::Add<Vector3> for &Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl ::std::ops::Add<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn add(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl ::std::ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl ::std::ops::Sub<&Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl ::std::ops::Sub<Vector3> for &Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl ::std::ops::Sub<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl ::std::ops::Div<Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl ::std::ops::Div<&Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl ::std::ops::Div<Vector3> for &Vector3 {
    type Output = Vector3;
    fn div(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl ::std::ops::Div<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn div(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}
impl ::std::ops::Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl ::std::ops::Div<f32> for &Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl ::std::ops::Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl ::std::ops::Mul<&Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl ::std::ops::Mul<Vector3> for &Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl ::std::ops::Mul<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}
impl ::std::ops::Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl ::std::ops::Mul<f32> for &Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f32) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Vector3 {
    pub fn zero() -> Self {
        Self {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }
    pub fn one() -> Self {
        Self {
            x: 1f32,
            y: 1f32,
            z: 1f32,
        }
    }
    pub fn unit_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn unit_y() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        }
    }
    pub fn unit_z() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        }
    }
    pub fn dot(&self, rhs: &Vector3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn distance(&self, value: &Vector3) -> f32 {
        (self - value).length()
    }
    pub fn distance_squared(&self, value: &Vector3) -> f32 {
        (self - value).length_squared()
    }
    pub fn normalize(&self) -> Self {
        self / self.length()
    }
}

// Vector4
impl ::std::ops::Add<Vector4> for Vector4 {
    type Output = Vector4;
    fn add(self, rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}
impl ::std::ops::Add<&Vector4> for Vector4 {
    type Output = Vector4;
    fn add(self, rhs: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}
impl ::std::ops::Add<Vector4> for &Vector4 {
    type Output = Vector4;
    fn add(self, rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}
impl ::std::ops::Add<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn add(self, rhs: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}
impl ::std::ops::Sub<Vector4> for Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
impl ::std::ops::Sub<&Vector4> for Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
impl ::std::ops::Sub<Vector4> for &Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
impl ::std::ops::Sub<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn sub(self, rhs: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
impl ::std::ops::Div<Vector4> for Vector4 {
    type Output = Vector4;
    fn div(self, rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}
impl ::std::ops::Div<&Vector4> for Vector4 {
    type Output = Vector4;
    fn div(self, rhs: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}
impl ::std::ops::Div<Vector4> for &Vector4 {
    type Output = Vector4;
    fn div(self, rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}
impl ::std::ops::Div<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn div(self, rhs: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
            w: self.w / rhs.w,
        }
    }
}
impl ::std::ops::Div<f32> for Vector4 {
    type Output = Vector4;
    fn div(self, rhs: f32) -> Vector4 {
        Vector4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}
impl ::std::ops::Div<f32> for &Vector4 {
    type Output = Vector4;
    fn div(self, rhs: f32) -> Vector4 {
        Vector4 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}
impl ::std::ops::Mul<Vector4> for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}
impl ::std::ops::Mul<&Vector4> for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}
impl ::std::ops::Mul<Vector4> for &Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: Vector4) -> Vector4 {
        Vector4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}
impl ::std::ops::Mul<&Vector4> for &Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: &Vector4) -> Vector4 {
        Vector4 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
            w: self.w * rhs.w,
        }
    }
}
impl ::std::ops::Mul<f32> for Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: f32) -> Vector4 {
        Vector4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
impl ::std::ops::Mul<f32> for &Vector4 {
    type Output = Vector4;
    fn mul(self, rhs: f32) -> Vector4 {
        Vector4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
impl Vector4 {
    pub fn zero() -> Self {
        Self {
            x: 0f32,
            y: 0f32,
            z: 0f32,
            w: 0f32,
        }
    }
    pub fn one() -> Self {
        Self {
            x: 1f32,
            y: 1f32,
            z: 1f32,
            w: 1f32,
        }
    }
    pub fn unit_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        }
    }
    pub fn unit_y() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0,
            w: 0.0,
        }
    }
    pub fn unit_z() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0,
            w: 0.0,
        }
    }
    pub fn unit_w() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
    pub fn dot(&self, rhs: &Vector4) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }
    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn distance(&self, value: &Vector4) -> f32 {
        (self - value).length()
    }
    pub fn distance_squared(&self, value: &Vector4) -> f32 {
        (self - value).length_squared()
    }
    pub fn normalize(&self) -> Self {
        self / self.length()
    }
}

// Matrix3x2
impl ::std::ops::Add<Matrix3x2> for Matrix3x2 {
    type Output = Matrix3x2;
    fn add(self, rhs: Matrix3x2) -> Matrix3x2 {
        Matrix3x2 {
            m11: self.m11 + rhs.m11,
            m12: self.m12 + rhs.m12,
            m21: self.m21 + rhs.m21,
            m22: self.m22 + rhs.m22,
            m31: self.m31 + rhs.m31,
            m32: self.m32 + rhs.m32,
        }
    }
}
impl ::std::ops::Add<&Matrix3x2> for Matrix3x2 {
    type Output = Matrix3x2;
    fn add(self, rhs: &Matrix3x2) -> Matrix3x2 {
        Matrix3x2 {
            m11: self.m11 + rhs.m11,
            m12: self.m12 + rhs.m12,
            m21: self.m21 + rhs.m21,
            m22: self.m22 + rhs.m22,
            m31: self.m31 + rhs.m31,
            m32: self.m32 + rhs.m32,
        }
    }
}
impl ::std::ops::Add<Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn add(self, rhs: Matrix3x2) -> Matrix3x2 {
        Matrix3x2 {
            m11: self.m11 + rhs.m11,
            m12: self.m12 + rhs.m12,
            m21: self.m21 + rhs.m21,
            m22: self.m22 + rhs.m22,
            m31: self.m31 + rhs.m31,
            m32: self.m32 + rhs.m32,
        }
    }
}
impl ::std::ops::Add<&Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn add(self, rhs: &Matrix3x2) -> Matrix3x2 {
        Matrix3x2 {
            m11: self.m11 + rhs.m11,
            m12: self.m12 + rhs.m12,
            m21: self.m21 + rhs.m21,
            m22: self.m22 + rhs.m22,
            m31: self.m31 + rhs.m31,
            m32: self.m32 + rhs.m32,
        }
    }
}
impl ::std::ops::Sub<Matrix3x2> for Matrix3x2 {
    type Output = Matrix3x2;
    fn sub(self, rhs: Matrix3x2) -> Matrix3x2 {
        Matrix3x2 {
            m11: self.m11 - rhs.m11,
            m12: self.m12 - rhs.m12,
            m21: self.m21 - rhs.m21,
            m22: self.m22 - rhs.m22,
            m31: self.m31 - rhs.m31,
            m32: self.m32 - rhs.m32,
        }
    }
}
impl ::std::ops::Sub<&Matrix3x2> for Matrix3x2 {
    type Output = Matrix3x2;
    fn sub(self, rhs: &Matrix3x2) -> Matrix3x2 {
        Matrix3x2 {
            m11: self.m11 - rhs.m11,
            m12: self.m12 - rhs.m12,
            m21: self.m21 - rhs.m21,
            m22: self.m22 - rhs.m22,
            m31: self.m31 - rhs.m31,
            m32: self.m32 - rhs.m32,
        }
    }
}
impl ::std::ops::Sub<Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn sub(self, rhs: Matrix3x2) -> Matrix3x2 {
        Matrix3x2 {
            m11: self.m11 - rhs.m11,
            m12: self.m12 - rhs.m12,
            m21: self.m21 - rhs.m21,
            m22: self.m22 - rhs.m22,
            m31: self.m31 - rhs.m31,
            m32: self.m32 - rhs.m32,
        }
    }
}
impl ::std::ops::Sub<&Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn sub(self, rhs: &Matrix3x2) -> Matrix3x2 {
        Matrix3x2 {
            m11: self.m11 - rhs.m11,
            m12: self.m12 - rhs.m12,
            m21: self.m21 - rhs.m21,
            m22: self.m22 - rhs.m22,
            m31: self.m31 - rhs.m31,
            m32: self.m32 - rhs.m32,
        }
    }
}
impl ::std::ops::Mul<f32> for Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: f32) -> Matrix3x2 {
        Matrix3x2 {
            m11: self.m11 * rhs,
            m12: self.m12 * rhs,
            m21: self.m21 * rhs,
            m22: self.m22 * rhs,
            m31: self.m31 * rhs,
            m32: self.m32 * rhs,
        }
    }
}
impl ::std::ops::Mul<f32> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: f32) -> Matrix3x2 {
        Matrix3x2 {
            m11: self.m11 * rhs,
            m12: self.m12 * rhs,
            m21: self.m21 * rhs,
            m22: self.m22 * rhs,
            m31: self.m31 * rhs,
            m32: self.m32 * rhs,
        }
    }
}
impl ::std::ops::Mul<Matrix3x2> for Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: Matrix3x2) -> Matrix3x2 {
        Matrix3x2 {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,
            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + rhs.m31,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + rhs.m32,
        }
    }
}
impl ::std::ops::Mul<&Matrix3x2> for Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: &Matrix3x2) -> Matrix3x2 {
        Matrix3x2 {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,
            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + rhs.m31,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + rhs.m32,
        }
    }
}
impl ::std::ops::Mul<Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: Matrix3x2) -> Matrix3x2 {
        Matrix3x2 {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,
            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + rhs.m31,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + rhs.m32,
        }
    }
}
impl ::std::ops::Mul<&Matrix3x2> for &Matrix3x2 {
    type Output = Matrix3x2;
    fn mul(self, rhs: &Matrix3x2) -> Matrix3x2 {
        Matrix3x2 {
            m11: self.m11 * rhs.m11 + self.m12 * rhs.m21,
            m12: self.m11 * rhs.m12 + self.m12 * rhs.m22,
            m21: self.m21 * rhs.m11 + self.m22 * rhs.m21,
            m22: self.m21 * rhs.m12 + self.m22 * rhs.m22,
            m31: self.m31 * rhs.m11 + self.m32 * rhs.m21 + rhs.m31,
            m32: self.m31 * rhs.m12 + self.m32 * rhs.m22 + rhs.m32,
        }
    }
}

// Matrix4x4
impl ::std::ops::Add<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
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
}
impl ::std::ops::Add<&Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: &Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
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
}
impl ::std::ops::Add<Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
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
}
impl ::std::ops::Add<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn add(self, rhs: &Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
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
}
impl ::std::ops::Sub<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
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
}
impl ::std::ops::Sub<&Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: &Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
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
}
impl ::std::ops::Sub<Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
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
}
impl ::std::ops::Sub<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn sub(self, rhs: &Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
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
}
impl ::std::ops::Mul<f32> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: f32) -> Matrix4x4 {
        Matrix4x4 {
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
impl ::std::ops::Mul<f32> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: f32) -> Matrix4x4 {
        Matrix4x4 {
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
impl ::std::ops::Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
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
}
impl ::std::ops::Mul<&Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
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
}
impl ::std::ops::Mul<Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
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
}
impl ::std::ops::Mul<&Matrix4x4> for &Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
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
}
