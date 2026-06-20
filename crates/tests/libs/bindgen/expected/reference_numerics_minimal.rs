#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Transform {
    pub position: windows_numerics::Vector3,
}
impl windows_core::TypeKind for Transform {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for Transform {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(
        b"struct(Test.Transform;struct(Windows.Foundation.Numerics.Vector3;f4;f4;f4))",
    );
}
