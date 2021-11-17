#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
pub type IWebApplicationActivation = *mut ::core::ffi::c_void;
pub type IWebApplicationAuthoringMode = *mut ::core::ffi::c_void;
pub type IWebApplicationHost = *mut ::core::ffi::c_void;
pub type IWebApplicationNavigationEvents = *mut ::core::ffi::c_void;
pub type IWebApplicationScriptEvents = *mut ::core::ffi::c_void;
pub type IWebApplicationUIEvents = *mut ::core::ffi::c_void;
pub type IWebApplicationUpdateEvents = *mut ::core::ffi::c_void;
pub type RegisterAuthoringClientFunctionType = unsafe extern "system" fn(authoringmodeobject: ::core::option::Option<IWebApplicationAuthoringMode>, host: ::core::option::Option<IWebApplicationHost>) -> ::windows_sys::core::HRESULT;
pub type UnregisterAuthoringClientFunctionType = unsafe extern "system" fn(host: ::core::option::Option<IWebApplicationHost>) -> ::windows_sys::core::HRESULT;
