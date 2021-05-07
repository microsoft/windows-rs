use super::*;

pub fn gen_matrix3x2() -> TokenStream {
    quote! {
        impl Matrix3x2 {
            pub fn identity() -> Self {
                Self {
                    M11: 1.0,
                    M12: 0.0,
                    M21: 0.0,
                    M22: 1.0,
                    M31: 0.0,
                    M32: 0.0,
                }
            }
            pub fn translation(x: f32, y: f32) -> Self {
                Self {
                    M11: 1.0,
                    M12: 0.0,
                    M21: 0.0,
                    M22: 1.0,
                    M31: x,
                    M32: y,
                }
            }
            pub fn rotation(angle: f32, x: f32, y: f32) -> Self {
                let mut matrix = Self::default();
                unsafe {
                    super::super::Win32::Graphics::Direct2D::D2D1MakeRotateMatrix(angle, super::super::Win32::Graphics::Direct2D::D2D_POINT_2F{x, y}, &mut matrix);
                }
                matrix
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

        impl ::std::ops::Add<Matrix3x2> for Matrix3x2 {
            type Output = Matrix3x2;
            fn add(self, rhs: Matrix3x2) -> Matrix3x2 {
                self.impl_add(&rhs)
            }
        }
        impl ::std::ops::Add<&Matrix3x2> for Matrix3x2 {
            type Output = Matrix3x2;
            fn add(self, rhs: &Matrix3x2) -> Matrix3x2 {
                self.impl_add(rhs)
            }
        }
        impl ::std::ops::Add<Matrix3x2> for &Matrix3x2 {
            type Output = Matrix3x2;
            fn add(self, rhs: Matrix3x2) -> Matrix3x2 {
                self.impl_add(&rhs)
            }
        }
        impl ::std::ops::Add<&Matrix3x2> for &Matrix3x2 {
            type Output = Matrix3x2;
            fn add(self, rhs: &Matrix3x2) -> Matrix3x2 {
                self.impl_add(rhs)
            }
        }
        impl ::std::ops::Sub<Matrix3x2> for Matrix3x2 {
            type Output = Matrix3x2;
            fn sub(self, rhs: Matrix3x2) -> Matrix3x2 {
                self.impl_sub(&rhs)
            }
        }
        impl ::std::ops::Sub<&Matrix3x2> for Matrix3x2 {
            type Output = Matrix3x2;
            fn sub(self, rhs: &Matrix3x2) -> Matrix3x2 {
                self.impl_sub(rhs)
            }
        }
        impl ::std::ops::Sub<Matrix3x2> for &Matrix3x2 {
            type Output = Matrix3x2;
            fn sub(self, rhs: Matrix3x2) -> Matrix3x2 {
                self.impl_sub(&rhs)
            }
        }
        impl ::std::ops::Sub<&Matrix3x2> for &Matrix3x2 {
            type Output = Matrix3x2;
            fn sub(self, rhs: &Matrix3x2) -> Matrix3x2 {
                self.impl_sub(rhs)
            }
        }
        impl ::std::ops::Mul<Matrix3x2> for Matrix3x2 {
            type Output = Matrix3x2;
            fn mul(self, rhs: Matrix3x2) -> Matrix3x2 {
                self.impl_mul(&rhs)
            }
        }
        impl ::std::ops::Mul<&Matrix3x2> for Matrix3x2 {
            type Output = Matrix3x2;
            fn mul(self, rhs: &Matrix3x2) -> Matrix3x2 {
                self.impl_mul(rhs)
            }
        }
        impl ::std::ops::Mul<Matrix3x2> for &Matrix3x2 {
            type Output = Matrix3x2;
            fn mul(self, rhs: Matrix3x2) -> Matrix3x2 {
                self.impl_mul(&rhs)
            }
        }
        impl ::std::ops::Mul<&Matrix3x2> for &Matrix3x2 {
            type Output = Matrix3x2;
            fn mul(self, rhs: &Matrix3x2) -> Matrix3x2 {
                self.impl_mul(rhs)
            }
        }
        impl ::std::ops::Mul<f32> for Matrix3x2 {
            type Output = Matrix3x2;
            fn mul(self, rhs: f32) -> Matrix3x2 {
                self.impl_mul_f32(rhs)
            }
        }
        impl ::std::ops::Mul<f32> for &Matrix3x2 {
            type Output = Matrix3x2;
            fn mul(self, rhs: f32) -> Matrix3x2 {
                self.impl_mul_f32(rhs)
            }
        }
    }
}
