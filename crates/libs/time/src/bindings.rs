#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DateTime {
    pub UniversalTime: i64,
}
impl windows_core::TypeKind for DateTime {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DateTime {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.DateTime;i8)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.DateTime");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TimeSpan {
    pub Duration: i64,
}
impl windows_core::TypeKind for TimeSpan {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for TimeSpan {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Windows.Foundation.TimeSpan;i8)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Windows.Foundation.TimeSpan");
}
