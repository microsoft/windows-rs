#![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
#[doc = "*Required features: `Win32_System_SetupAndMigration`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OOBEComplete(isoobecomplete: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OOBEComplete(isoobecomplete: *mut super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(OOBEComplete(::std::mem::transmute(isoobecomplete)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SetupAndMigration`*"]
pub type OOBE_COMPLETED_CALLBACK = unsafe extern "system" fn(callbackcontext: *const ::std::ffi::c_void);
#[doc = "*Required features: `Win32_System_SetupAndMigration`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn RegisterWaitUntilOOBECompleted(oobecompletedcallback: ::std::option::Option<OOBE_COMPLETED_CALLBACK>, callbackcontext: *const ::std::ffi::c_void, waithandle: *mut *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn RegisterWaitUntilOOBECompleted(oobecompletedcallback: ::windows::runtime::RawPtr, callbackcontext: *const ::std::ffi::c_void, waithandle: *mut *mut ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(RegisterWaitUntilOOBECompleted(::std::mem::transmute(oobecompletedcallback), ::std::mem::transmute(callbackcontext), ::std::mem::transmute(waithandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `Win32_System_SetupAndMigration`, `Win32_Foundation`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn UnregisterWaitUntilOOBECompleted(waithandle: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn UnregisterWaitUntilOOBECompleted(waithandle: *const ::std::ffi::c_void) -> super::super::Foundation::BOOL;
        }
        ::std::mem::transmute(UnregisterWaitUntilOOBECompleted(::std::mem::transmute(waithandle)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
