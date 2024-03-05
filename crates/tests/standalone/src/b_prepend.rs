#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[derive(std::cmp::PartialOrd, std::cmp::Ord)]
#[repr(C)]
pub struct DateTime {
    pub UniversalTime: i64,
}
impl Copy for DateTime {}
impl Clone for DateTime {
    fn clone(&self) -> Self {
        *self
    }
}
impl core::fmt::Debug for DateTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DateTime")
            .field("UniversalTime", &self.UniversalTime)
            .finish()
    }
}
impl windows_core::TypeKind for DateTime {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DateTime {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.DateTime;i8)");
}
impl PartialEq for DateTime {
    fn eq(&self, other: &Self) -> bool {
        self.UniversalTime == other.UniversalTime
    }
}
impl Eq for DateTime {}
impl Default for DateTime {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
