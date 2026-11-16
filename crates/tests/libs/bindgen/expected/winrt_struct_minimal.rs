#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Struct {
    pub x: i32,
    pub y: i32,
}
impl windows_core::imp::TypeKind for Struct {
    type TypeKind = windows_core::imp::CopyType;
}
impl windows_core::RuntimeType for Struct {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Test.Struct;i4;i4)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.Struct");
}
