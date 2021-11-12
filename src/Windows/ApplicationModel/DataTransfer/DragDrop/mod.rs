#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[cfg(feature = "ApplicationModel_DataTransfer_DragDrop_Core")]
pub mod Core;
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq, :: core :: marker :: Copy, :: core :: clone :: Clone, :: core :: default :: Default, :: core :: fmt :: Debug)]
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
impl ::core::convert::From<u32> for DragDropModifiers {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
unsafe impl ::windows::core::Abi for DragDropModifiers {
    type Abi = Self;
}
unsafe impl ::windows::core::RuntimeType for DragDropModifiers {
    const SIGNATURE: ::windows::core::ConstBuffer = ::windows::core::ConstBuffer::from_slice(b"enum(Windows.ApplicationModel.DataTransfer.DragDrop.DragDropModifiers;u4)");
}
impl ::windows::core::DefaultType for DragDropModifiers {
    type DefaultType = Self;
}
impl ::core::ops::BitOr for DragDropModifiers {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}
impl ::core::ops::BitAnd for DragDropModifiers {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}
impl ::core::ops::BitOrAssign for DragDropModifiers {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0.bitor_assign(rhs.0)
    }
}
impl ::core::ops::BitAndAssign for DragDropModifiers {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0.bitand_assign(rhs.0)
    }
}
impl ::core::ops::Not for DragDropModifiers {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
