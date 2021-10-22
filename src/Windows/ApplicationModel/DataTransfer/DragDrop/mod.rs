#![allow(
    unused_variables,
    non_upper_case_globals,
    non_snake_case,
    unused_unsafe,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
#[cfg(feature = "ApplicationModel_DataTransfer_DragDrop_Core")]
pub mod Core;
#[derive(
    :: std :: cmp :: PartialEq,
    :: std :: cmp :: Eq,
    :: std :: marker :: Copy,
    :: std :: clone :: Clone,
    :: std :: default :: Default,
    :: std :: fmt :: Debug,
)]
#[repr(transparent)]
pub struct DragDropModifiers(pub u32);
impl DragDropModifiers {
    pub const None: DragDropModifiers = DragDropModifiers(0u32);
    pub const Shift: DragDropModifiers = DragDropModifiers(1u32);
    pub const Control: DragDropModifiers = DragDropModifiers(2u32);
    pub const Alt: DragDropModifiers = DragDropModifiers(4u32);
    pub const LeftButton: DragDropModifiers = DragDropModifiers(8u32);
    pub const MiddleButton: DragDropModifiers = DragDropModifiers(16u32);
    pub const RightButton: DragDropModifiers = DragDropModifiers(32u32);
}
impl ::std::convert::From<u32> for DragDropModifiers {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::runtime::Abi for DragDropModifiers {
    type Abi = Self;
    type DefaultType = Self;
}
unsafe impl ::windows::runtime::RuntimeType for DragDropModifiers {
    const SIGNATURE: ::windows::runtime::ConstBuffer = ::windows::runtime::ConstBuffer::from_slice(
        b"enum(Windows.ApplicationModel.DataTransfer.DragDrop.DragDropModifiers;u4)",
    );
}
impl ::std::ops::BitOr for DragDropModifiers {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::std::ops::BitAnd for DragDropModifiers {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::std::ops::BitOrAssign for DragDropModifiers {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::std::ops::BitAndAssign for DragDropModifiers {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::std::ops::Not for DragDropModifiers {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
