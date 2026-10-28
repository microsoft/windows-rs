#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FlsAlloc(lpcallback: super::PFLS_CALLBACK_FUNCTION) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn FlsAlloc(lpcallback : super::PFLS_CALLBACK_FUNCTION) -> super::DWORD);
    unsafe { FlsAlloc(lpcallback) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn FlsFree(dwflsindex: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FlsFree(dwflsindex : super::DWORD) -> windows_core::BOOL);
    unsafe { FlsFree(dwflsindex) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FlsGetValue(dwflsindex: super::DWORD) -> super::PVOID {
    windows_core::link!("kernel32.dll" "system" fn FlsGetValue(dwflsindex : super::DWORD) -> super::PVOID);
    unsafe { FlsGetValue(dwflsindex) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FlsGetValue2(dwtlsindex: super::DWORD) -> super::PVOID {
    windows_core::link!("kernel32.dll" "system" fn FlsGetValue2(dwtlsindex : super::DWORD) -> super::PVOID);
    unsafe { FlsGetValue2(dwtlsindex) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FlsSetValue(dwflsindex: super::DWORD, lpflsdata: Option<super::PVOID>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FlsSetValue(dwflsindex : super::DWORD, lpflsdata : super::PVOID) -> windows_core::BOOL);
    unsafe { FlsSetValue(dwflsindex, lpflsdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn IsThreadAFiber() -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsThreadAFiber() -> windows_core::BOOL);
    unsafe { IsThreadAFiber() }
}
#[cfg(feature = "shared")]
pub const FLS_OUT_OF_INDEXES: super::DWORD = 4294967295;
