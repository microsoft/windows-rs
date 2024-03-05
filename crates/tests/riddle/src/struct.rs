#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(C)]
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
impl Copy for Primitives {}
impl Clone for Primitives {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for Primitives {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Primitives")
            .field("field_bool", &self.field_bool)
            .field("field_i8", &self.field_i8)
            .field("field_u8", &self.field_u8)
            .field("field_i16", &self.field_i16)
            .field("field_u16", &self.field_u16)
            .field("field_i32", &self.field_i32)
            .field("field_u32", &self.field_u32)
            .field("field_i64", &self.field_i64)
            .field("field_u64", &self.field_u64)
            .field("field_f32", &self.field_f32)
            .field("field_f64", &self.field_f64)
            .field("field_isize", &self.field_isize)
            .field("field_usize", &self.field_usize)
            .finish()
    }
}
impl windows_core::TypeKind for Primitives {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Primitives {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Test.Primitives;b1;i1;u1;i2;u2;i4;u4;i8;u8;f4;f8;is;us)",
    );
}
impl PartialEq for Primitives {
    fn eq(&self, other: &Self) -> bool {
        self.field_bool == other.field_bool
            && self.field_i8 == other.field_i8
            && self.field_u8 == other.field_u8
            && self.field_i16 == other.field_i16
            && self.field_u16 == other.field_u16
            && self.field_i32 == other.field_i32
            && self.field_u32 == other.field_u32
            && self.field_i64 == other.field_i64
            && self.field_u64 == other.field_u64
            && self.field_f32 == other.field_f32
            && self.field_f64 == other.field_f64
            && self.field_isize == other.field_isize
            && self.field_usize == other.field_usize
    }
}
impl Eq for Primitives {}
impl Default for Primitives {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
