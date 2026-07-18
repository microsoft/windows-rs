#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FlsAlloc(lpcallback: super::PFLS_CALLBACK_FUNCTION) -> u32 {
    windows_core::link!("kernel32.dll" "system" fn FlsAlloc(lpcallback : super::PFLS_CALLBACK_FUNCTION) -> u32);
    unsafe { FlsAlloc(lpcallback) }
}
#[inline]
pub unsafe fn FlsFree(dwflsindex: u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FlsFree(dwflsindex : u32) -> windows_core::BOOL);
    unsafe { FlsFree(dwflsindex) }
}
#[inline]
pub unsafe fn FlsGetValue(dwflsindex: u32) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn FlsGetValue(dwflsindex : u32) -> *mut core::ffi::c_void);
    unsafe { FlsGetValue(dwflsindex) }
}
#[inline]
pub unsafe fn FlsGetValue2(dwtlsindex: u32) -> *mut core::ffi::c_void {
    windows_core::link!("kernel32.dll" "system" fn FlsGetValue2(dwtlsindex : u32) -> *mut core::ffi::c_void);
    unsafe { FlsGetValue2(dwtlsindex) }
}
#[inline]
pub unsafe fn FlsSetValue(dwflsindex: u32, lpflsdata: Option<*const core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FlsSetValue(dwflsindex : u32, lpflsdata : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { FlsSetValue(dwflsindex, lpflsdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn IsThreadAFiber() -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsThreadAFiber() -> windows_core::BOOL);
    unsafe { IsThreadAFiber() }
}
pub const FLS_OUT_OF_INDEXES: u32 = 4294967295;
