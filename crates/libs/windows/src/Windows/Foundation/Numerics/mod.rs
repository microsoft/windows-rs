#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3x2 {
    pub M11: f32,
    pub M12: f32,
    pub M21: f32,
    pub M22: f32,
    pub M31: f32,
    pub M32: f32,
}
impl windows_core::TypeKind for Matrix3x2 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Matrix3x2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Matrix3x2;f4;f4;f4;f4;f4;f4)");
}
impl Default for Matrix3x2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4x4 {
    pub M11: f32,
    pub M12: f32,
    pub M13: f32,
    pub M14: f32,
    pub M21: f32,
    pub M22: f32,
    pub M23: f32,
    pub M24: f32,
    pub M31: f32,
    pub M32: f32,
    pub M33: f32,
    pub M34: f32,
    pub M41: f32,
    pub M42: f32,
    pub M43: f32,
    pub M44: f32,
}
impl windows_core::TypeKind for Matrix4x4 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Matrix4x4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Matrix4x4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4)");
}
impl Default for Matrix4x4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Plane {
    pub Normal: Vector3,
    pub D: f32,
}
impl windows_core::TypeKind for Plane {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Plane {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Plane;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4);f4)");
}
impl Default for Plane {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quaternion {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
    pub W: f32,
}
impl windows_core::TypeKind for Quaternion {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Quaternion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Quaternion;f4;f4;f4;f4)");
}
impl Default for Quaternion {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rational {
    pub Numerator: u32,
    pub Denominator: u32,
}
impl windows_core::TypeKind for Rational {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Rational {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Rational;u4;u4)");
}
impl Default for Rational {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2 {
    pub X: f32,
    pub Y: f32,
}
impl windows_core::TypeKind for Vector2 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Vector2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Vector2;f4;f4)");
}
impl Default for Vector2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
}
impl windows_core::TypeKind for Vector3 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Vector3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4)");
}
impl Default for Vector3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector4 {
    pub X: f32,
    pub Y: f32,
    pub Z: f32,
    pub W: f32,
}
impl windows_core::TypeKind for Vector4 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Vector4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Numerics.Vector4;f4;f4;f4;f4)");
}
impl Default for Vector4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
