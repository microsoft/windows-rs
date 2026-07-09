#[cfg(feature = "minwinbase")]
#[inline]
pub unsafe fn CallEnclave(lproutine: super::minwinbase::LPENCLAVE_ROUTINE, lpparameter: *const core::ffi::c_void, fwaitforthread: bool, lpreturnvalue: *mut *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("vertdll.dll" "system" fn CallEnclave(lproutine : super::minwinbase::LPENCLAVE_ROUTINE, lpparameter : *const core::ffi::c_void, fwaitforthread : windows_core::BOOL, lpreturnvalue : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CallEnclave(lproutine, lpparameter, fwaitforthread.into(), lpreturnvalue as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CreateEnclave(hprocess: super::winnt::HANDLE, lpaddress: Option<*const core::ffi::c_void>, dwsize: usize, dwinitialcommitment: usize, flenclavetype: u32, lpenclaveinformation: *const core::ffi::c_void, dwinfolength: u32, lpenclaveerror: Option<*mut u32>) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn CreateEnclave(hprocess : super::winnt::HANDLE, lpaddress : *const core::ffi::c_void, dwsize : usize, dwinitialcommitment : usize, flenclavetype : u32, lpenclaveinformation : *const core::ffi::c_void, dwinfolength : u32, lpenclaveerror : *mut u32) -> *mut core::ffi::c_void);
    unsafe { CreateEnclave(hprocess, lpaddress.unwrap_or(core::mem::zeroed()) as _, dwsize, dwinitialcommitment, flenclavetype, lpenclaveinformation, dwinfolength, lpenclaveerror.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DeleteEnclave(lpaddress: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn DeleteEnclave(lpaddress : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DeleteEnclave(lpaddress) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitializeEnclave(hprocess: super::winnt::HANDLE, lpaddress: *const core::ffi::c_void, lpenclaveinformation: *const core::ffi::c_void, dwinfolength: u32, lpenclaveerror: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitializeEnclave(hprocess : super::winnt::HANDLE, lpaddress : *const core::ffi::c_void, lpenclaveinformation : *const core::ffi::c_void, dwinfolength : u32, lpenclaveerror : *mut u32) -> windows_core::BOOL);
    unsafe { InitializeEnclave(hprocess, lpaddress, lpenclaveinformation, dwinfolength, lpenclaveerror.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn IsEnclaveTypeSupported(flenclavetype: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsEnclaveTypeSupported(flenclavetype : u32) -> windows_core::BOOL);
    unsafe { IsEnclaveTypeSupported(flenclavetype) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn LoadEnclaveData(hprocess: super::winnt::HANDLE, lpaddress: *const core::ffi::c_void, lpbuffer: *const core::ffi::c_void, nsize: usize, flprotect: u32, lppageinformation: *const core::ffi::c_void, dwinfolength: u32, lpnumberofbyteswritten: *mut usize, lpenclaveerror: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn LoadEnclaveData(hprocess : super::winnt::HANDLE, lpaddress : *const core::ffi::c_void, lpbuffer : *const core::ffi::c_void, nsize : usize, flprotect : u32, lppageinformation : *const core::ffi::c_void, dwinfolength : u32, lpnumberofbyteswritten : *mut usize, lpenclaveerror : *mut u32) -> windows_core::BOOL);
    unsafe { LoadEnclaveData(hprocess, lpaddress, lpbuffer, nsize, flprotect, lppageinformation, dwinfolength, lpnumberofbyteswritten as _, lpenclaveerror.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn LoadEnclaveImageA<P1>(lpenclaveaddress: *const core::ffi::c_void, lpimagename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn LoadEnclaveImageA(lpenclaveaddress : *const core::ffi::c_void, lpimagename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { LoadEnclaveImageA(lpenclaveaddress, lpimagename.param().abi()) }
}
#[inline]
pub unsafe fn LoadEnclaveImageW<P1>(lpenclaveaddress: *const core::ffi::c_void, lpimagename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-enclave-l1-1-1.dll" "system" fn LoadEnclaveImageW(lpenclaveaddress : *const core::ffi::c_void, lpimagename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { LoadEnclaveImageW(lpenclaveaddress, lpimagename.param().abi()) }
}
#[inline]
pub unsafe fn TerminateEnclave(lpaddress: *const core::ffi::c_void, fwait: bool) -> windows_core::BOOL {
    windows_core::link!("vertdll.dll" "system" fn TerminateEnclave(lpaddress : *const core::ffi::c_void, fwait : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { TerminateEnclave(lpaddress, fwait.into()) }
}
