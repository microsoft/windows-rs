#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {
    pub fn CoGetInterceptor(iidintercepted: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    pub fn CoGetInterceptorFromTypeInfo(iidintercepted: *const ::windows_sys::core::GUID, punkouter: ::windows_sys::core::IUnknown, typeinfo: super::ITypeInfo, iid: *const ::windows_sys::core::GUID, ppv: *mut *mut ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAMEINFO {
    pub iMethod: u32,
    pub fHasInValues: super::super::super::Foundation::BOOL,
    pub fHasInOutValues: super::super::super::Foundation::BOOL,
    pub fHasOutValues: super::super::super::Foundation::BOOL,
    pub fDerivesFromIDispatch: super::super::super::Foundation::BOOL,
    pub cInInterfacesMax: i32,
    pub cInOutInterfacesMax: i32,
    pub cOutInterfacesMax: i32,
    pub cTopLevelInInterfaces: i32,
    pub iid: ::windows_sys::core::GUID,
    pub cMethod: u32,
    pub cParams: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALLFRAMEINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAMEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAMEPARAMINFO {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub fOut: super::super::super::Foundation::BOOLEAN,
    pub stackOffset: u32,
    pub cbParam: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALLFRAMEPARAMINFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAMEPARAMINFO {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CALLFRAME_COPY = i32;
pub const CALLFRAME_COPY_NESTED: CALLFRAME_COPY = 1i32;
pub const CALLFRAME_COPY_INDEPENDENT: CALLFRAME_COPY = 2i32;
pub type CALLFRAME_FREE = i32;
pub const CALLFRAME_FREE_NONE: CALLFRAME_FREE = 0i32;
pub const CALLFRAME_FREE_IN: CALLFRAME_FREE = 1i32;
pub const CALLFRAME_FREE_INOUT: CALLFRAME_FREE = 2i32;
pub const CALLFRAME_FREE_OUT: CALLFRAME_FREE = 4i32;
pub const CALLFRAME_FREE_TOP_INOUT: CALLFRAME_FREE = 8i32;
pub const CALLFRAME_FREE_TOP_OUT: CALLFRAME_FREE = 16i32;
pub const CALLFRAME_FREE_ALL: CALLFRAME_FREE = 31i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct CALLFRAME_MARSHALCONTEXT {
    pub fIn: super::super::super::Foundation::BOOLEAN,
    pub dwDestContext: u32,
    pub pvDestContext: *mut ::core::ffi::c_void,
    pub punkReserved: ::windows_sys::core::IUnknown,
    pub guidTransferSyntax: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for CALLFRAME_MARSHALCONTEXT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for CALLFRAME_MARSHALCONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
pub type CALLFRAME_NULL = i32;
pub const CALLFRAME_NULL_NONE: CALLFRAME_NULL = 0i32;
pub const CALLFRAME_NULL_INOUT: CALLFRAME_NULL = 2i32;
pub const CALLFRAME_NULL_OUT: CALLFRAME_NULL = 4i32;
pub const CALLFRAME_NULL_ALL: CALLFRAME_NULL = 6i32;
pub type CALLFRAME_WALK = i32;
pub const CALLFRAME_WALK_IN: CALLFRAME_WALK = 1i32;
pub const CALLFRAME_WALK_INOUT: CALLFRAME_WALK = 2i32;
pub const CALLFRAME_WALK_OUT: CALLFRAME_WALK = 4i32;
#[repr(transparent)]
pub struct ICallFrame(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICallFrame {}
impl ::core::clone::Clone for ICallFrame {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICallFrameEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICallFrameEvents {}
impl ::core::clone::Clone for ICallFrameEvents {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICallFrameWalker(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICallFrameWalker {}
impl ::core::clone::Clone for ICallFrameWalker {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICallIndirect(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICallIndirect {}
impl ::core::clone::Clone for ICallIndirect {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICallInterceptor(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICallInterceptor {}
impl ::core::clone::Clone for ICallInterceptor {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct ICallUnmarshal(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for ICallUnmarshal {}
impl ::core::clone::Clone for ICallUnmarshal {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IInterfaceRelated(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IInterfaceRelated {}
impl ::core::clone::Clone for IInterfaceRelated {
    fn clone(&self) -> Self {
        *self
    }
}
