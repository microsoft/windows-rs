#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub fn CoGetInterceptor(iidintercepted: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub fn CoGetInterceptorFromTypeInfo(iidintercepted: *const ::windows::runtime::GUID, punkouter: ::windows::runtime::RawPtr, typeinfo: ::windows::runtime::RawPtr, iid: *const ::windows::runtime::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
}
