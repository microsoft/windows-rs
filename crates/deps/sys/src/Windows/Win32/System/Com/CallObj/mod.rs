#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    pub fn CoGetInterceptor(iidintercepted: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CoGetInterceptorFromTypeInfo(iidintercepted: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, typeinfo: super::ITypeInfo, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CALLFRAMEINFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CALLFRAMEPARAMINFO(i32);
#[repr(transparent)]
pub struct CALLFRAME_COPY(pub i32);
pub const CALLFRAME_COPY_NESTED: CALLFRAME_COPY = CALLFRAME_COPY(1i32);
pub const CALLFRAME_COPY_INDEPENDENT: CALLFRAME_COPY = CALLFRAME_COPY(2i32);
#[repr(transparent)]
pub struct CALLFRAME_FREE(pub i32);
pub const CALLFRAME_FREE_NONE: CALLFRAME_FREE = CALLFRAME_FREE(0i32);
pub const CALLFRAME_FREE_IN: CALLFRAME_FREE = CALLFRAME_FREE(1i32);
pub const CALLFRAME_FREE_INOUT: CALLFRAME_FREE = CALLFRAME_FREE(2i32);
pub const CALLFRAME_FREE_OUT: CALLFRAME_FREE = CALLFRAME_FREE(4i32);
pub const CALLFRAME_FREE_TOP_INOUT: CALLFRAME_FREE = CALLFRAME_FREE(8i32);
pub const CALLFRAME_FREE_TOP_OUT: CALLFRAME_FREE = CALLFRAME_FREE(16i32);
pub const CALLFRAME_FREE_ALL: CALLFRAME_FREE = CALLFRAME_FREE(31i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct CALLFRAME_MARSHALCONTEXT(i32);
#[repr(transparent)]
pub struct CALLFRAME_NULL(pub i32);
pub const CALLFRAME_NULL_NONE: CALLFRAME_NULL = CALLFRAME_NULL(0i32);
pub const CALLFRAME_NULL_INOUT: CALLFRAME_NULL = CALLFRAME_NULL(2i32);
pub const CALLFRAME_NULL_OUT: CALLFRAME_NULL = CALLFRAME_NULL(4i32);
pub const CALLFRAME_NULL_ALL: CALLFRAME_NULL = CALLFRAME_NULL(6i32);
#[repr(transparent)]
pub struct CALLFRAME_WALK(pub i32);
pub const CALLFRAME_WALK_IN: CALLFRAME_WALK = CALLFRAME_WALK(1i32);
pub const CALLFRAME_WALK_INOUT: CALLFRAME_WALK = CALLFRAME_WALK(2i32);
pub const CALLFRAME_WALK_OUT: CALLFRAME_WALK = CALLFRAME_WALK(4i32);
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
