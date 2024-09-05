#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Type {
    pub field: i32,
}
impl windows_core::TypeKind for Type {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Type {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Test.Type;i4)");
}
impl Default for Type {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
