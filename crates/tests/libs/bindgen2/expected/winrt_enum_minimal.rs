#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Enum(pub i32);
impl Enum {
    pub const First: Self = Self(0);
    pub const Second: Self = Self(1);
    pub const Third: Self = Self(2);
}
impl windows_core::TypeKind for Enum {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Enum {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"enum(Test.Enum;i4)");
}
