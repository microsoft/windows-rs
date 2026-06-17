#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Matrix3x2 {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub m31: f32,
    pub m32: f32,
}
impl windows_core::TypeKind for Matrix3x2 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Matrix3x2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.Foundation.Numerics.Matrix3x2;f4;f4;f4;f4;f4;f4)",
    );
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Matrix4x4 {
    pub m11: f32,
    pub m12: f32,
    pub m13: f32,
    pub m14: f32,
    pub m21: f32,
    pub m22: f32,
    pub m23: f32,
    pub m24: f32,
    pub m31: f32,
    pub m32: f32,
    pub m33: f32,
    pub m34: f32,
    pub m41: f32,
    pub m42: f32,
    pub m43: f32,
    pub m44: f32,
}
impl windows_core::TypeKind for Matrix4x4 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Matrix4x4 {
    const SIGNATURE : windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice (b"struct(Windows.Foundation.Numerics.Matrix4x4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4;f4)") ;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}
impl windows_core::TypeKind for Vector2 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Vector2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.Foundation.Numerics.Vector2;f4;f4)",
    );
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl windows_core::TypeKind for Vector3 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Vector3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4)",
    );
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl windows_core::TypeKind for Vector4 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Vector4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.Foundation.Numerics.Vector4;f4;f4;f4;f4)",
    );
}
