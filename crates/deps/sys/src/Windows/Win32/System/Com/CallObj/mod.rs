#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    pub fn CoGetInterceptor(iidintercepted: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
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
#[repr(transparent)]
pub struct ICallFrame(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICallFrameEvents(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICallFrameWalker(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICallIndirect(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICallInterceptor(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct ICallUnmarshal(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IInterfaceRelated(pub *mut ::core::ffi::c_void);
