#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FlsAlloc(lpcallback : super::winnt::PFLS_CALLBACK_FUNCTION) -> u32);
windows_link::link!("kernel32.dll" "system" fn FlsFree(dwflsindex : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn FlsGetValue(dwflsindex : u32) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn FlsGetValue2(dwtlsindex : u32) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn FlsSetValue(dwflsindex : u32, lpflsdata : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsThreadAFiber() -> windows_sys::core::BOOL);
pub const FLS_OUT_OF_INDEXES: u32 = 4294967295;
