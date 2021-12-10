use super::*;

pub fn gen_matrix4x4() -> TokenStream {
    quote! {
        impl Matrix4x4 {
            fn impl_add(&self, rhs: &Self) -> Self {
                Self {
                    M11: self.M11 + rhs.M11,
                    M12: self.M12 + rhs.M12,
                    M13: self.M13 + rhs.M13,
                    M14: self.M14 + rhs.M14,
                    M21: self.M21 + rhs.M21,
                    M22: self.M22 + rhs.M22,
                    M23: self.M23 + rhs.M23,
                    M24: self.M24 + rhs.M24,
                    M31: self.M31 + rhs.M31,
                    M32: self.M32 + rhs.M32,
                    M33: self.M33 + rhs.M33,
                    M34: self.M34 + rhs.M34,
                    M41: self.M41 + rhs.M41,
                    M42: self.M42 + rhs.M42,
                    M43: self.M43 + rhs.M43,
                    M44: self.M44 + rhs.M44,
                }
            }
            fn impl_sub(&self, rhs: &Self) -> Self {
                Self {
                    M11: self.M11 - rhs.M11,
                    M12: self.M12 - rhs.M12,
                    M13: self.M13 - rhs.M13,
                    M14: self.M14 - rhs.M14,
                    M21: self.M21 - rhs.M21,
                    M22: self.M22 - rhs.M22,
                    M23: self.M23 - rhs.M23,
                    M24: self.M24 - rhs.M24,
                    M31: self.M31 - rhs.M31,
                    M32: self.M32 - rhs.M32,
                    M33: self.M33 - rhs.M33,
                    M34: self.M34 - rhs.M34,
                    M41: self.M41 - rhs.M41,
                    M42: self.M42 - rhs.M42,
                    M43: self.M43 - rhs.M43,
                    M44: self.M44 - rhs.M44,
                }
            }
            fn impl_mul(&self, rhs: &Self) -> Self {
                Self {
                    M11: self.M11 * rhs.M11 + self.M12 * rhs.M21 + self.M13 * rhs.M31 + self.M14 * rhs.M41,
                    M12: self.M11 * rhs.M12 + self.M12 * rhs.M22 + self.M13 * rhs.M32 + self.M14 * rhs.M42,
                    M13: self.M11 * rhs.M13 + self.M12 * rhs.M23 + self.M13 * rhs.M33 + self.M14 * rhs.M43,
                    M14: self.M11 * rhs.M14 + self.M12 * rhs.M24 + self.M13 * rhs.M34 + self.M14 * rhs.M44,
                    M21: self.M21 * rhs.M11 + self.M22 * rhs.M21 + self.M23 * rhs.M31 + self.M24 * rhs.M41,
                    M22: self.M21 * rhs.M12 + self.M22 * rhs.M22 + self.M23 * rhs.M32 + self.M24 * rhs.M42,
                    M23: self.M21 * rhs.M13 + self.M22 * rhs.M23 + self.M23 * rhs.M33 + self.M24 * rhs.M43,
                    M24: self.M21 * rhs.M14 + self.M22 * rhs.M24 + self.M23 * rhs.M34 + self.M24 * rhs.M44,
                    M31: self.M31 * rhs.M11 + self.M32 * rhs.M21 + self.M33 * rhs.M31 + self.M34 * rhs.M41,
                    M32: self.M31 * rhs.M12 + self.M32 * rhs.M22 + self.M33 * rhs.M32 + self.M34 * rhs.M42,
                    M33: self.M31 * rhs.M13 + self.M32 * rhs.M23 + self.M33 * rhs.M33 + self.M34 * rhs.M43,
                    M34: self.M31 * rhs.M14 + self.M32 * rhs.M24 + self.M33 * rhs.M34 + self.M34 * rhs.M44,
                    M41: self.M41 * rhs.M11 + self.M42 * rhs.M21 + self.M43 * rhs.M31 + self.M44 * rhs.M41,
                    M42: self.M41 * rhs.M12 + self.M42 * rhs.M22 + self.M43 * rhs.M32 + self.M44 * rhs.M42,
                    M43: self.M41 * rhs.M13 + self.M42 * rhs.M23 + self.M43 * rhs.M33 + self.M44 * rhs.M43,
                    M44: self.M41 * rhs.M14 + self.M42 * rhs.M24 + self.M43 * rhs.M34 + self.M44 * rhs.M44,
                }
            }
            fn impl_mul_f32(&self, rhs: f32) -> Self {
                Self {
                    M11: self.M11 * rhs,
                    M12: self.M12 * rhs,
                    M13: self.M13 * rhs,
                    M14: self.M14 * rhs,
                    M21: self.M21 * rhs,
                    M22: self.M22 * rhs,
                    M23: self.M23 * rhs,
                    M24: self.M24 * rhs,
                    M31: self.M31 * rhs,
                    M32: self.M32 * rhs,
                    M33: self.M33 * rhs,
                    M34: self.M34 * rhs,
                    M41: self.M41 * rhs,
                    M42: self.M42 * rhs,
                    M43: self.M43 * rhs,
                    M44: self.M44 * rhs,
                }
            }
        }

        impl ::core::ops::Add<Matrix4x4> for Matrix4x4 {
            type Output = Matrix4x4;
            fn add(self, rhs: Matrix4x4) -> Matrix4x4 {
                self.impl_add(&rhs)
            }
        }
        impl ::core::ops::Add<&Matrix4x4> for Matrix4x4 {
            type Output = Matrix4x4;
            fn add(self, rhs: &Matrix4x4) -> Matrix4x4 {
                self.impl_add(rhs)
            }
        }
        impl ::core::ops::Add<Matrix4x4> for &Matrix4x4 {
            type Output = Matrix4x4;
            fn add(self, rhs: Matrix4x4) -> Matrix4x4 {
                self.impl_add(&rhs)
            }
        }
        impl ::core::ops::Add<&Matrix4x4> for &Matrix4x4 {
            type Output = Matrix4x4;
            fn add(self, rhs: &Matrix4x4) -> Matrix4x4 {
                self.impl_add(rhs)
            }
        }
        impl ::core::ops::Sub<Matrix4x4> for Matrix4x4 {
            type Output = Matrix4x4;
            fn sub(self, rhs: Matrix4x4) -> Matrix4x4 {
                self.impl_sub(&rhs)
            }
        }
        impl ::core::ops::Sub<&Matrix4x4> for Matrix4x4 {
            type Output = Matrix4x4;
            fn sub(self, rhs: &Matrix4x4) -> Matrix4x4 {
                self.impl_sub(rhs)
            }
        }
        impl ::core::ops::Sub<Matrix4x4> for &Matrix4x4 {
            type Output = Matrix4x4;
            fn sub(self, rhs: Matrix4x4) -> Matrix4x4 {
                self.impl_sub(&rhs)
            }
        }
        impl ::core::ops::Sub<&Matrix4x4> for &Matrix4x4 {
            type Output = Matrix4x4;
            fn sub(self, rhs: &Matrix4x4) -> Matrix4x4 {
                self.impl_sub(rhs)
            }
        }
        impl ::core::ops::Mul<Matrix4x4> for Matrix4x4 {
            type Output = Matrix4x4;
            fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
                self.impl_mul(&rhs)
            }
        }
        impl ::core::ops::Mul<&Matrix4x4> for Matrix4x4 {
            type Output = Matrix4x4;
            fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
                self.impl_mul(rhs)
            }
        }
        impl ::core::ops::Mul<Matrix4x4> for &Matrix4x4 {
            type Output = Matrix4x4;
            fn mul(self, rhs: Matrix4x4) -> Matrix4x4 {
                self.impl_mul(&rhs)
            }
        }
        impl ::core::ops::Mul<&Matrix4x4> for &Matrix4x4 {
            type Output = Matrix4x4;
            fn mul(self, rhs: &Matrix4x4) -> Matrix4x4 {
                self.impl_mul(rhs)
            }
        }
        impl ::core::ops::Mul<f32> for Matrix4x4 {
            type Output = Matrix4x4;
            fn mul(self, rhs: f32) -> Matrix4x4 {
                self.impl_mul_f32(rhs)
            }
        }
        impl ::core::ops::Mul<f32> for &Matrix4x4 {
            type Output = Matrix4x4;
            fn mul(self, rhs: f32) -> Matrix4x4 {
                self.impl_mul_f32(rhs)
            }
        }
    }
}
