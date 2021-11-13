#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct IWebApplicationActivation(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebApplicationActivation {}
impl ::core::clone::Clone for IWebApplicationActivation {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebApplicationAuthoringMode(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebApplicationAuthoringMode {}
impl ::core::clone::Clone for IWebApplicationAuthoringMode {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebApplicationHost(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebApplicationHost {}
impl ::core::clone::Clone for IWebApplicationHost {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebApplicationNavigationEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebApplicationNavigationEvents {}
impl ::core::clone::Clone for IWebApplicationNavigationEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebApplicationScriptEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebApplicationScriptEvents {}
impl ::core::clone::Clone for IWebApplicationScriptEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebApplicationUIEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebApplicationUIEvents {}
impl ::core::clone::Clone for IWebApplicationUIEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IWebApplicationUpdateEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IWebApplicationUpdateEvents {}
impl ::core::clone::Clone for IWebApplicationUpdateEvents {
    fn clone(&self) -> Self {
        *self
    }
}
pub type RegisterAuthoringClientFunctionType = unsafe extern "system" fn(authoringmodeobject: IWebApplicationAuthoringMode, host: IWebApplicationHost) -> ::windows_sys::core::HRESULT;
pub type UnregisterAuthoringClientFunctionType = unsafe extern "system" fn(host: IWebApplicationHost) -> ::windows_sys::core::HRESULT;
