#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
pub struct IWebApplicationActivation(pub *mut ::core::ffi::c_void);
pub struct IWebApplicationAuthoringMode(pub *mut ::core::ffi::c_void);
pub struct IWebApplicationHost(pub *mut ::core::ffi::c_void);
pub struct IWebApplicationNavigationEvents(pub *mut ::core::ffi::c_void);
pub struct IWebApplicationScriptEvents(pub *mut ::core::ffi::c_void);
pub struct IWebApplicationUIEvents(pub *mut ::core::ffi::c_void);
pub struct IWebApplicationUpdateEvents(pub *mut ::core::ffi::c_void);
pub struct RegisterAuthoringClientFunctionType(i32);
pub struct UnregisterAuthoringClientFunctionType(i32);
