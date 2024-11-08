#[cfg(feature = "ApplicationModel_DataTransfer_DragDrop_Core")]
pub mod Core;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct DragDropModifiers(pub u32);
impl DragDropModifiers {
    pub const None: Self = Self(0u32);
    pub const Shift: Self = Self(1u32);
    pub const Control: Self = Self(2u32);
    pub const Alt: Self = Self(4u32);
    pub const LeftButton: Self = Self(8u32);
    pub const MiddleButton: Self = Self(16u32);
    pub const RightButton: Self = Self(32u32);
}
impl windows_core::TypeKind for DragDropModifiers {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for DragDropModifiers {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.DragDrop.DragDropModifiers;u4)");
}
