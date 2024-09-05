#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub struct RefWithFloat {
    pub ReferenceFloat: Option<windows::Foundation::IReference<f32>>,
    pub ReferenceWithFloat: Option<windows::Foundation::IReference<WithFloat>>,
}
impl windows_core::TypeKind for RefWithFloat {
    type TypeKind = windows_core::CloneType;
}
impl windows_core::RuntimeType for RefWithFloat {
    const SIGNATURE :windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice ( b"struct(test_reference_float.RefWithFloat;pinterface({61c17706-2d65-11e0-9ae8-d48564015472};f4);pinterface({61c17706-2d65-11e0-9ae8-d48564015472};struct(test_reference_float.WithFloat;f4)))" ) ;
}
impl Default for RefWithFloat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WithFloat {
    pub Value: f32,
}
impl windows_core::TypeKind for WithFloat {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for WithFloat {
    const SIGNATURE: windows_core::imp::ConstBuffer =
        windows_core::imp::ConstBuffer::from_slice(b"struct(test_reference_float.WithFloat;f4)");
}
impl Default for WithFloat {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
