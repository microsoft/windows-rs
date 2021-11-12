#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IWebUICommandBar(pub *mut ::core::ffi::c_void);
pub struct IWebUICommandBarBitmapIcon(pub *mut ::core::ffi::c_void);
pub struct IWebUICommandBarBitmapIconFactory(pub *mut ::core::ffi::c_void);
pub struct IWebUICommandBarConfirmationButton(pub *mut ::core::ffi::c_void);
pub struct IWebUICommandBarElement(pub *mut ::core::ffi::c_void);
pub struct IWebUICommandBarIcon(pub *mut ::core::ffi::c_void);
pub struct IWebUICommandBarIconButton(pub *mut ::core::ffi::c_void);
pub struct IWebUICommandBarItemInvokedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebUICommandBarSizeChangedEventArgs(pub *mut ::core::ffi::c_void);
pub struct IWebUICommandBarStatics(pub *mut ::core::ffi::c_void);
pub struct IWebUICommandBarSymbolIcon(pub *mut ::core::ffi::c_void);
pub struct IWebUICommandBarSymbolIconFactory(pub *mut ::core::ffi::c_void);
pub struct MenuClosedEventHandler(pub *mut ::core::ffi::c_void);
pub struct MenuOpenedEventHandler(pub *mut ::core::ffi::c_void);
pub struct SizeChangedEventHandler(pub *mut ::core::ffi::c_void);
pub struct WebUICommandBar(i32);
pub struct WebUICommandBarBitmapIcon(i32);
pub struct WebUICommandBarClosedDisplayMode(i32);
pub struct WebUICommandBarConfirmationButton(i32);
pub struct WebUICommandBarContract(i32);
pub struct WebUICommandBarIconButton(i32);
pub struct WebUICommandBarItemInvokedEventArgs(i32);
pub struct WebUICommandBarSizeChangedEventArgs(i32);
pub struct WebUICommandBarSymbolIcon(i32);
