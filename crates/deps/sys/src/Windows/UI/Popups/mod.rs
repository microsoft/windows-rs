#![allow(non_snake_case, non_camel_case_types)]
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
pub struct MessageDialogOptions(i32);
pub struct Placement(i32);
#[repr(transparent)]
pub struct PopupMenu(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UICommand(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UICommandInvokedHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct UICommandSeparator(pub *mut ::core::ffi::c_void);
