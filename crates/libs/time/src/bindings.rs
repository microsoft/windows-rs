windows_core::link!("kernel32.dll" "system" fn FileTimeToLocalFileTime(lpfiletime : *const FILETIME, lplocalfiletime : *mut FILETIME) -> windows_core::BOOL);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DateTime {
    pub universal_time: i64,
}
impl windows_core::TypeKind for DateTime {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DateTime {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.DateTime;i8)");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILETIME {
    pub dwLowDateTime: u32,
    pub dwHighDateTime: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct TimeSpan {
    pub duration: i64,
}
impl windows_core::TypeKind for TimeSpan {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TimeSpan {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.TimeSpan;i8)");
}
