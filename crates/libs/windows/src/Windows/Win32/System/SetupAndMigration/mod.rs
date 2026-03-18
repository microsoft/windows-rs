#[inline]
pub unsafe fn OOBEComplete(isoobecomplete: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn OOBEComplete(isoobecomplete : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { OOBEComplete(isoobecomplete as _) }
}
#[inline]
pub unsafe fn RegisterWaitUntilOOBECompleted(oobecompletedcallback: OOBE_COMPLETED_CALLBACK, callbackcontext: *mut core::ffi::c_void, waithandle: *mut *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn RegisterWaitUntilOOBECompleted(oobecompletedcallback : OOBE_COMPLETED_CALLBACK, callbackcontext : *mut core::ffi::c_void, waithandle : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RegisterWaitUntilOOBECompleted(oobecompletedcallback, callbackcontext as _, waithandle as _) }
}
#[inline]
pub unsafe fn UnregisterWaitUntilOOBECompleted(waithandle: *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnregisterWaitUntilOOBECompleted(waithandle : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { UnregisterWaitUntilOOBECompleted(waithandle as _) }
}
pub type OOBE_COMPLETED_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: *mut core::ffi::c_void)>;
