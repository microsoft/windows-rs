#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Struct {
    pub x: i32,
    pub y: i32,
}
impl windows_core::TypeKind for Struct {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Struct {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Test.Struct;i4;i4)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.Struct");
}
