#[cfg(feature = "minwinbase")]
windows_link::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn CallEnclave(lproutine : super::LPENCLAVE_ROUTINE, lpparameter : *const core::ffi::c_void, fwaitforthread : windows_sys::core::BOOL, lpreturnvalue : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateEnclave(hprocess : super::HANDLE, lpaddress : *const core::ffi::c_void, dwsize : usize, dwinitialcommitment : usize, flenclavetype : u32, lpenclaveinformation : *const core::ffi::c_void, dwinfolength : u32, lpenclaveerror : *mut u32) -> *mut core::ffi::c_void);
windows_link::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn DeleteEnclave(lpaddress : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InitializeEnclave(hprocess : super::HANDLE, lpaddress : *const core::ffi::c_void, lpenclaveinformation : *const core::ffi::c_void, dwinfolength : u32, lpenclaveerror : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsEnclaveTypeSupported(flenclavetype : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn LoadEnclaveData(hprocess : super::HANDLE, lpaddress : *const core::ffi::c_void, lpbuffer : *const core::ffi::c_void, nsize : usize, flprotect : u32, lppageinformation : *const core::ffi::c_void, dwinfolength : u32, lpnumberofbyteswritten : *mut usize, lpenclaveerror : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn LoadEnclaveImageA(lpenclaveaddress : *const core::ffi::c_void, lpimagename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn LoadEnclaveImageW(lpenclaveaddress : *const core::ffi::c_void, lpimagename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn TerminateEnclave(lpaddress : *const core::ffi::c_void, fwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
