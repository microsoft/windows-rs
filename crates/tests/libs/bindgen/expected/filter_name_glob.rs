#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Widening {
    pub y: i32,
}
impl windows_core::TypeKind for Widening {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Widening {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Test.Widening;i4)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.Widening");
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Widget {
    pub x: i32,
}
impl windows_core::TypeKind for Widget {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Widget {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(Test.Widget;i4)");
    const NAME: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"Test.Widget");
}
