#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FlsAlloc(lpcallback : super::PFLS_CALLBACK_FUNCTION) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("kernel32.dll" "system" fn FlsFree(dwflsindex : super::DWORD) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FlsGetValue(dwflsindex : super::DWORD) -> super::PVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FlsGetValue2(dwtlsindex : super::DWORD) -> super::PVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FlsSetValue(dwflsindex : super::DWORD, lpflsdata : super::PVOID) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsThreadAFiber() -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
pub const FLS_OUT_OF_INDEXES: super::DWORD = 4294967295;
