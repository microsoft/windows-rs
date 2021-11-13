#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IMessageDialog(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMessageDialog {}
impl ::core::clone::Clone for IMessageDialog {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMessageDialogFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMessageDialogFactory {}
impl ::core::clone::Clone for IMessageDialogFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IPopupMenu(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IPopupMenu {}
impl ::core::clone::Clone for IPopupMenu {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUICommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUICommand {}
impl ::core::clone::Clone for IUICommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUICommandFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUICommandFactory {}
impl ::core::clone::Clone for IUICommandFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MessageDialog(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MessageDialog {}
impl ::core::clone::Clone for MessageDialog {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MessageDialogOptions(pub u32);
impl MessageDialogOptions {
    pub const None: Self = Self(0u32);
    pub const AcceptUserInputAfterDelay: Self = Self(1u32);
}
impl ::core::marker::Copy for MessageDialogOptions {}
impl ::core::clone::Clone for MessageDialogOptions {
    fn clone(&self) -> Self {
        *self
    }
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
impl ::core::marker::Copy for Placement {}
impl ::core::clone::Clone for Placement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct PopupMenu(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for PopupMenu {}
impl ::core::clone::Clone for PopupMenu {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UICommand(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UICommand {}
impl ::core::clone::Clone for UICommand {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UICommandInvokedHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UICommandInvokedHandler {}
impl ::core::clone::Clone for UICommandInvokedHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct UICommandSeparator(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for UICommandSeparator {}
impl ::core::clone::Clone for UICommandSeparator {
    fn clone(&self) -> Self {
        *self
    }
}
