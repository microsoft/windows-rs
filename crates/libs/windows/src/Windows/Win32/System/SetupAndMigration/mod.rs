#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OOBEComplete(isoobecomplete: &mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn OOBEComplete(isoobecomplete: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    }
    OOBEComplete(::core::mem::transmute(isoobecomplete))
}
pub type OOBE_COMPLETED_CALLBACK = ::core::option::Option<unsafe extern "system" fn(callbackcontext: *const ::core::ffi::c_void)>;
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterWaitUntilOOBECompleted(oobecompletedcallback: OOBE_COMPLETED_CALLBACK, callbackcontext: *const ::core::ffi::c_void, waithandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn RegisterWaitUntilOOBECompleted(oobecompletedcallback: *mut ::core::ffi::c_void, callbackcontext: *const ::core::ffi::c_void, waithandle: *mut *mut ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    RegisterWaitUntilOOBECompleted(::core::mem::transmute(oobecompletedcallback), ::core::mem::transmute(callbackcontext), ::core::mem::transmute(waithandle))
}
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterWaitUntilOOBECompleted(waithandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg_attr(windows, link(name = "windows"))]
    extern "system" {
        fn UnregisterWaitUntilOOBECompleted(waithandle: *const ::core::ffi::c_void) -> super::super::Foundation::BOOL;
    }
    UnregisterWaitUntilOOBECompleted(::core::mem::transmute(waithandle))
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
