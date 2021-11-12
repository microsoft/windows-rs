#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWebUICommandBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUICommandBar {}
impl ::core::clone::Clone for IWebUICommandBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUICommandBarBitmapIcon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUICommandBarBitmapIcon {}
impl ::core::clone::Clone for IWebUICommandBarBitmapIcon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUICommandBarBitmapIconFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUICommandBarBitmapIconFactory {}
impl ::core::clone::Clone for IWebUICommandBarBitmapIconFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUICommandBarConfirmationButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUICommandBarConfirmationButton {}
impl ::core::clone::Clone for IWebUICommandBarConfirmationButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUICommandBarElement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUICommandBarElement {}
impl ::core::clone::Clone for IWebUICommandBarElement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUICommandBarIcon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUICommandBarIcon {}
impl ::core::clone::Clone for IWebUICommandBarIcon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUICommandBarIconButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUICommandBarIconButton {}
impl ::core::clone::Clone for IWebUICommandBarIconButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUICommandBarItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUICommandBarItemInvokedEventArgs {}
impl ::core::clone::Clone for IWebUICommandBarItemInvokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUICommandBarSizeChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUICommandBarSizeChangedEventArgs {}
impl ::core::clone::Clone for IWebUICommandBarSizeChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUICommandBarStatics(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUICommandBarStatics {}
impl ::core::clone::Clone for IWebUICommandBarStatics {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUICommandBarSymbolIcon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUICommandBarSymbolIcon {}
impl ::core::clone::Clone for IWebUICommandBarSymbolIcon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebUICommandBarSymbolIconFactory(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebUICommandBarSymbolIconFactory {}
impl ::core::clone::Clone for IWebUICommandBarSymbolIconFactory {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuClosedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuClosedEventHandler {}
impl ::core::clone::Clone for MenuClosedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MenuOpenedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for MenuOpenedEventHandler {}
impl ::core::clone::Clone for MenuOpenedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct SizeChangedEventHandler(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for SizeChangedEventHandler {}
impl ::core::clone::Clone for SizeChangedEventHandler {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUICommandBar(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUICommandBar {}
impl ::core::clone::Clone for WebUICommandBar {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUICommandBarBitmapIcon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUICommandBarBitmapIcon {}
impl ::core::clone::Clone for WebUICommandBarBitmapIcon {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUICommandBarClosedDisplayMode(pub i32);
impl WebUICommandBarClosedDisplayMode {
    pub const Default: Self = Self(0i32);
    pub const Minimal: Self = Self(1i32);
    pub const Compact: Self = Self(2i32);
}
impl ::core::marker::Copy for WebUICommandBarClosedDisplayMode {}
impl ::core::clone::Clone for WebUICommandBarClosedDisplayMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUICommandBarConfirmationButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUICommandBarConfirmationButton {}
impl ::core::clone::Clone for WebUICommandBarConfirmationButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUICommandBarIconButton(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUICommandBarIconButton {}
impl ::core::clone::Clone for WebUICommandBarIconButton {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUICommandBarItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUICommandBarItemInvokedEventArgs {}
impl ::core::clone::Clone for WebUICommandBarItemInvokedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUICommandBarSizeChangedEventArgs(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUICommandBarSizeChangedEventArgs {}
impl ::core::clone::Clone for WebUICommandBarSizeChangedEventArgs {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct WebUICommandBarSymbolIcon(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for WebUICommandBarSymbolIcon {}
impl ::core::clone::Clone for WebUICommandBarSymbolIcon {
    fn clone(&self) -> Self {
        *self
    }
}
