#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub fn CoGetInterceptor(iidintercepted: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_System_Com_CallObj`*"]
    pub fn CoGetInterceptorFromTypeInfo(iidintercepted: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, typeinfo: super::ITypeInfo, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAMEINFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAMEPARAMINFO(i32);
pub struct CALLFRAME_COPY(i32);
pub struct CALLFRAME_FREE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAME_MARSHALCONTEXT(i32);
pub struct CALLFRAME_NULL(i32);
pub struct CALLFRAME_WALK(i32);
pub struct ICallFrame(i32);
pub struct ICallFrameEvents(i32);
pub struct ICallFrameWalker(i32);
pub struct ICallIndirect(i32);
pub struct ICallInterceptor(i32);
pub struct ICallUnmarshal(i32);
pub struct IInterfaceRelated(i32);
