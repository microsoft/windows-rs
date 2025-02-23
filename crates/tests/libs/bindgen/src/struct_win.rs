#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct RectInt32 {
    pub X: i32,
    pub Y: i32,
    pub Width: i32,
    pub Height: i32,
}
impl windows_core::TypeKind for RectInt32 {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for RectInt32 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.Graphics.RectInt32;i4;i4;i4;i4)",
    );
}
