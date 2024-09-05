#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Primitives {
    pub field_bool: bool,
    pub field_i8: i8,
    pub field_u8: u8,
    pub field_i16: i16,
    pub field_u16: u16,
    pub field_i32: i32,
    pub field_u32: u32,
    pub field_i64: i64,
    pub field_u64: u64,
    pub field_f32: f32,
    pub field_f64: f64,
    pub field_isize: isize,
    pub field_usize: usize,
}
impl windows_core::TypeKind for Primitives {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Primitives {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Test.Primitives;b1;i1;u1;i2;u2;i4;u4;i8;u8;f4;f8;is;us)",
    );
}
impl Default for Primitives {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
