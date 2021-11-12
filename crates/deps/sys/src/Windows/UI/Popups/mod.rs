#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IMessageDialog(pub *mut ::core::ffi::c_void);
pub struct IMessageDialogFactory(pub *mut ::core::ffi::c_void);
pub struct IPopupMenu(pub *mut ::core::ffi::c_void);
pub struct IUICommand(pub *mut ::core::ffi::c_void);
pub struct IUICommandFactory(pub *mut ::core::ffi::c_void);
pub struct MessageDialog(i32);
pub struct MessageDialogOptions(i32);
pub struct Placement(i32);
pub struct PopupMenu(i32);
pub struct UICommand(i32);
pub struct UICommandInvokedHandler(pub *mut ::core::ffi::c_void);
pub struct UICommandSeparator(i32);
