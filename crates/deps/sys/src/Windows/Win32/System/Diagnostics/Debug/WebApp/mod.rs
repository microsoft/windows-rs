#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWebApplicationActivation(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebApplicationAuthoringMode(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebApplicationHost(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebApplicationNavigationEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebApplicationScriptEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebApplicationUIEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWebApplicationUpdateEvents(pub *mut ::core::ffi::c_void);
pub type RegisterAuthoringClientFunctionType = unsafe extern "system" fn(authoringmodeobject: IWebApplicationAuthoringMode, host: IWebApplicationHost) -> ::windows_sys::core::HRESULT;
pub type UnregisterAuthoringClientFunctionType = unsafe extern "system" fn(host: IWebApplicationHost) -> ::windows_sys::core::HRESULT;
