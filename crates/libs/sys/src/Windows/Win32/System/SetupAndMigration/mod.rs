pub type OOBEComplete = unsafe extern "system" fn(isoobecomplete: *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn OOBEComplete(isoobecomplete : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
pub type RegisterWaitUntilOOBECompleted = unsafe extern "system" fn(oobecompletedcallback: OOBE_COMPLETED_CALLBACK, callbackcontext: *const core::ffi::c_void, waithandle: *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn RegisterWaitUntilOOBECompleted(oobecompletedcallback : OOBE_COMPLETED_CALLBACK, callbackcontext : *const core::ffi::c_void, waithandle : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
pub type UnregisterWaitUntilOOBECompleted = unsafe extern "system" fn(waithandle: *const core::ffi::c_void) -> windows_sys::core::BOOL;
windows_link::link!("kernel32.dll" "system" fn UnregisterWaitUntilOOBECompleted(waithandle : *const core::ffi::c_void) -> windows_sys::core::BOOL);
pub type OOBE_COMPLETED_CALLBACK = Option<unsafe extern "system" fn(callbackcontext: *const core::ffi::c_void)>;
