#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[cfg(feature = "ApplicationModel_DataTransfer_DragDrop_Core")]
pub mod Core;
#[link(name = "windows")]
extern "system" {}
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
