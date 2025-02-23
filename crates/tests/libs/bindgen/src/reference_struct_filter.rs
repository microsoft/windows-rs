#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct InkTrailPoint {
    pub Point: Point,
    pub Radius: f32,
}
impl windows_core::TypeKind for InkTrailPoint {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for InkTrailPoint {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Windows.UI.Composition.InkTrailPoint;struct(Windows.Foundation.Point;f4;f4);f4)",
    );
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point {
    pub X: f32,
    pub Y: f32,
}
impl windows_core::TypeKind for Point {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Point {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.Point;f4;f4)");
}
