#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IMessageDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMessageDialogFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IPopupMenu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUICommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUICommandFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessageDialog(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MessageDialogOptions(pub u32);
impl MessageDialogOptions {
    pub const None: Self = Self(0u32);
    pub const AcceptUserInputAfterDelay: Self = Self(1u32);
}
#[repr(transparent)]
pub struct Placement(pub i32);
impl Placement {
    pub const Default: Self = Self(0i32);
    pub const Above: Self = Self(1i32);
    pub const Below: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
    pub const Right: Self = Self(4i32);
}
#[repr(transparent)]
pub struct PopupMenu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UICommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UICommandInvokedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UICommandSeparator(pub *mut ::core::ffi::c_void);
