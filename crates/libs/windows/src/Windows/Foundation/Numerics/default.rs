impl ::core::default::Default for Matrix3x2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Matrix3x2 {
    fn eq(&self, other: &Self) -> bool {
        self.M11 == other.M11 && self.M12 == other.M12 && self.M21 == other.M21 && self.M22 == other.M22 && self.M31 == other.M31 && self.M32 == other.M32
    }
}
impl ::core::cmp::Eq for Matrix3x2 {}
impl ::core::fmt::Debug for Matrix3x2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Matrix3x2").field("M11", &self.M11).field("M12", &self.M12).field("M21", &self.M21).field("M22", &self.M22).field("M31", &self.M31).field("M32", &self.M32).finish()
    }
}
impl ::core::default::Default for Matrix4x4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Matrix4x4 {
    fn eq(&self, other: &Self) -> bool {
        self.M11 == other.M11 && self.M12 == other.M12 && self.M13 == other.M13 && self.M14 == other.M14 && self.M21 == other.M21 && self.M22 == other.M22 && self.M23 == other.M23 && self.M24 == other.M24 && self.M31 == other.M31 && self.M32 == other.M32 && self.M33 == other.M33 && self.M34 == other.M34 && self.M41 == other.M41 && self.M42 == other.M42 && self.M43 == other.M43 && self.M44 == other.M44
    }
}
impl ::core::cmp::Eq for Matrix4x4 {}
impl ::core::fmt::Debug for Matrix4x4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Matrix4x4").field("M11", &self.M11).field("M12", &self.M12).field("M13", &self.M13).field("M14", &self.M14).field("M21", &self.M21).field("M22", &self.M22).field("M23", &self.M23).field("M24", &self.M24).field("M31", &self.M31).field("M32", &self.M32).field("M33", &self.M33).field("M34", &self.M34).field("M41", &self.M41).field("M42", &self.M42).field("M43", &self.M43).field("M44", &self.M44).finish()
    }
}
impl ::core::default::Default for Plane {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.Normal == other.Normal && self.D == other.D
    }
}
impl ::core::cmp::Eq for Plane {}
impl ::core::fmt::Debug for Plane {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Plane").field("Normal", &self.Normal).field("D", &self.D).finish()
    }
}
impl ::core::default::Default for Quaternion {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Quaternion {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z && self.W == other.W
    }
}
impl ::core::cmp::Eq for Quaternion {}
impl ::core::fmt::Debug for Quaternion {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Quaternion").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).field("W", &self.W).finish()
    }
}
impl ::core::default::Default for Rational {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Rational {
    fn eq(&self, other: &Self) -> bool {
        self.Numerator == other.Numerator && self.Denominator == other.Denominator
    }
}
impl ::core::cmp::Eq for Rational {}
impl ::core::fmt::Debug for Rational {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Rational").field("Numerator", &self.Numerator).field("Denominator", &self.Denominator).finish()
    }
}
impl ::core::default::Default for Vector2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Vector2 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for Vector2 {}
impl ::core::fmt::Debug for Vector2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Vector2").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::core::default::Default for Vector3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z
    }
}
impl ::core::cmp::Eq for Vector3 {}
impl ::core::fmt::Debug for Vector3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Vector3").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).finish()
    }
}
impl ::core::default::Default for Vector4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for Vector4 {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z && self.W == other.W
    }
}
impl ::core::cmp::Eq for Vector4 {}
impl ::core::fmt::Debug for Vector4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("Vector4").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).field("W", &self.W).finish()
    }
}
