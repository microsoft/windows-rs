#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWebUICommandBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUICommandBarBitmapIcon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUICommandBarBitmapIconFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUICommandBarConfirmationButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUICommandBarElement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUICommandBarIcon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUICommandBarIconButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUICommandBarItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUICommandBarSizeChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUICommandBarStatics(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUICommandBarSymbolIcon(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebUICommandBarSymbolIconFactory(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuClosedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct MenuOpenedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct SizeChangedEventHandler(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUICommandBar(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUICommandBarBitmapIcon(pub *mut ::core::ffi::c_void);
pub struct WebUICommandBarClosedDisplayMode(i32);
#[repr(transparent)]
pub struct WebUICommandBarConfirmationButton(pub *mut ::core::ffi::c_void);
pub struct WebUICommandBarContract(i32);
#[repr(transparent)]
pub struct WebUICommandBarIconButton(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUICommandBarItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUICommandBarSizeChangedEventArgs(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WebUICommandBarSymbolIcon(pub *mut ::core::ffi::c_void);
