#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Inner {
    pub field_i32: i32,
}
impl windows_core::TypeKind for Inner {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Inner {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Test.Inner;i4)");
}
impl Default for Inner {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Outer {
    pub field_bool: bool,
    pub field_inner: Inner,
    pub field_usize: usize,
}
impl windows_core::TypeKind for Outer {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Outer {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Test.Outer;b1;struct(Test.Inner;i4);us)",
    );
}
impl Default for Outer {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
