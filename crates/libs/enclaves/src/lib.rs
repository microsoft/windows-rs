#![doc = include_str!("../readme.md")]
#![cfg(windows)]
#![allow(non_camel_case_types, non_snake_case)]

mod bindings;
pub use bindings::*;

// The following types should be generated but are not currently included in the Win32 metadata.

pub type PDELAYLOAD_FAILURE_SYSTEM_ROUTINE = Option<
    unsafe extern "system" fn(dllname: PCSTR, procedurename: PCSTR) -> *mut core::ffi::c_void,
>;

windows_link::link!("vertdll.dll" "system" fn EnclaveCopyIntoEnclave(enclaveaddress: *mut core::ffi::c_void, unsecureaddress: *const core::ffi::c_void, numberofbytes: usize) -> HRESULT);
windows_link::link!("vertdll.dll" "system" fn EnclaveCopyOutOfEnclave(unsecureaddress: *mut core::ffi::c_void, enclaveaddress: *const core::ffi::c_void, numberofbytes: usize) -> HRESULT);
windows_link::link!("vertdll.dll" "system" fn EnclaveRestrictContainingProcessAccess(restrictaccess: BOOL, previouslyrestricted: *mut BOOL) -> HRESULT);
windows_link::link!("vertdll.dll" "system" fn LdrDisableThreadCalloutsForDll(baseaddress: *mut core::ffi::c_void) -> NTSTATUS);
windows_link::link!("vertdll.dll" "system" fn LdrResolveDelayLoadedAPI(parentmodulebase: *const core::ffi::c_void, delayloaddescriptor: *const IMAGE_DELAYLOAD_DESCRIPTOR, failuredllhook: PDELAYLOAD_FAILURE_DLL_CALLBACK, failuresystemhook: PDELAYLOAD_FAILURE_SYSTEM_ROUTINE, thunkaddress: *mut IMAGE_THUNK_DATA32, flags: u32) -> *mut core::ffi::c_void);
windows_link::link!("vertdll.dll" "system" fn RtlGetLastcrateNTSTATUS() -> NTSTATUS);
windows_link::link!("vertdll.dll" "system" fn RtlRaiseStatus(status: NTSTATUS));
windows_link::link!("vertdll.dll" "system" fn RtlUnhandledExceptionFilter(exceptionpointers: *const EXCEPTION_POINTERS) -> i32);
