#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn CallEnclave(lproutine : super::LPENCLAVE_ROUTINE, lpparameter : super::LPVOID, fwaitforthread : windows_sys::core::BOOL, lpreturnvalue : *mut super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateEnclave(hprocess : super::HANDLE, lpaddress : super::LPVOID, dwsize : super::SIZE_T, dwinitialcommitment : super::SIZE_T, flenclavetype : super::DWORD, lpenclaveinformation : super::LPCVOID, dwinfolength : super::DWORD, lpenclaveerror : super::LPDWORD) -> super::LPVOID);
#[cfg(feature = "minwindef")]
windows_link::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn DeleteEnclave(lpaddress : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitializeEnclave(hprocess : super::HANDLE, lpaddress : super::LPVOID, lpenclaveinformation : super::LPCVOID, dwinfolength : super::DWORD, lpenclaveerror : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn IsEnclaveTypeSupported(flenclavetype : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LoadEnclaveData(hprocess : super::HANDLE, lpaddress : super::LPVOID, lpbuffer : super::LPCVOID, nsize : super::SIZE_T, flprotect : super::DWORD, lppageinformation : super::LPCVOID, dwinfolength : super::DWORD, lpnumberofbyteswritten : super::PSIZE_T, lpenclaveerror : super::LPDWORD) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn LoadEnclaveImageA(lpenclaveaddress : super::LPVOID, lpimagename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn LoadEnclaveImageW(lpenclaveaddress : super::LPVOID, lpimagename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn TerminateEnclave(lpaddress : super::LPVOID, fwait : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
