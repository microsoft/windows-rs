#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
#[inline]
pub unsafe fn CallEnclave(lproutine: super::LPENCLAVE_ROUTINE, lpparameter: super::LPVOID, fwaitforthread: bool, lpreturnvalue: *mut super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn CallEnclave(lproutine : super::LPENCLAVE_ROUTINE, lpparameter : super::LPVOID, fwaitforthread : windows_core::BOOL, lpreturnvalue : *mut super::LPVOID) -> windows_core::BOOL);
    unsafe { CallEnclave(lproutine, lpparameter, fwaitforthread.into(), lpreturnvalue as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateEnclave(hprocess: super::HANDLE, lpaddress: Option<super::LPVOID>, dwsize: super::SIZE_T, dwinitialcommitment: super::SIZE_T, flenclavetype: super::DWORD, lpenclaveinformation: super::LPCVOID, dwinfolength: super::DWORD, lpenclaveerror: Option<super::LPDWORD>) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn CreateEnclave(hprocess : super::HANDLE, lpaddress : super::LPVOID, dwsize : super::SIZE_T, dwinitialcommitment : super::SIZE_T, flenclavetype : super::DWORD, lpenclaveinformation : super::LPCVOID, dwinfolength : super::DWORD, lpenclaveerror : super::LPDWORD) -> super::LPVOID);
    unsafe { CreateEnclave(hprocess, lpaddress.unwrap_or(core::mem::zeroed()) as _, dwsize, dwinitialcommitment, flenclavetype, lpenclaveinformation, dwinfolength, lpenclaveerror.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DeleteEnclave(lpaddress: super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn DeleteEnclave(lpaddress : super::LPVOID) -> windows_core::BOOL);
    unsafe { DeleteEnclave(lpaddress) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InitializeEnclave(hprocess: super::HANDLE, lpaddress: super::LPVOID, lpenclaveinformation: super::LPCVOID, dwinfolength: super::DWORD, lpenclaveerror: Option<super::LPDWORD>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitializeEnclave(hprocess : super::HANDLE, lpaddress : super::LPVOID, lpenclaveinformation : super::LPCVOID, dwinfolength : super::DWORD, lpenclaveerror : super::LPDWORD) -> windows_core::BOOL);
    unsafe { InitializeEnclave(hprocess, lpaddress, lpenclaveinformation, dwinfolength, lpenclaveerror.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn IsEnclaveTypeSupported(flenclavetype: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsEnclaveTypeSupported(flenclavetype : super::DWORD) -> windows_core::BOOL);
    unsafe { IsEnclaveTypeSupported(flenclavetype) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LoadEnclaveData(hprocess: super::HANDLE, lpaddress: super::LPVOID, lpbuffer: super::LPCVOID, nsize: super::SIZE_T, flprotect: super::DWORD, lppageinformation: super::LPCVOID, dwinfolength: super::DWORD, lpnumberofbyteswritten: super::PSIZE_T, lpenclaveerror: Option<super::LPDWORD>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn LoadEnclaveData(hprocess : super::HANDLE, lpaddress : super::LPVOID, lpbuffer : super::LPCVOID, nsize : super::SIZE_T, flprotect : super::DWORD, lppageinformation : super::LPCVOID, dwinfolength : super::DWORD, lpnumberofbyteswritten : super::PSIZE_T, lpenclaveerror : super::LPDWORD) -> windows_core::BOOL);
    unsafe { LoadEnclaveData(hprocess, lpaddress, lpbuffer, nsize, flprotect, lppageinformation, dwinfolength, lpnumberofbyteswritten as _, lpenclaveerror.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LoadEnclaveImageA<P1>(lpenclaveaddress: super::LPVOID, lpimagename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn LoadEnclaveImageA(lpenclaveaddress : super::LPVOID, lpimagename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { LoadEnclaveImageA(lpenclaveaddress, lpimagename.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LoadEnclaveImageW<P1>(lpenclaveaddress: super::LPVOID, lpimagename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn LoadEnclaveImageW(lpenclaveaddress : super::LPVOID, lpimagename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { LoadEnclaveImageW(lpenclaveaddress, lpimagename.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn TerminateEnclave(lpaddress: super::LPVOID, fwait: bool) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn TerminateEnclave(lpaddress : super::LPVOID, fwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { TerminateEnclave(lpaddress, fwait.into()) }
}
