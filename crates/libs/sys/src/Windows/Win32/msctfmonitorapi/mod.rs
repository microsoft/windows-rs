#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("msctfmonitor.dll" "system" fn DoMsCtfMonitor(dwflags : super::DWORD, heventforservicestop : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("msctfmonitor.dll" "system" fn InitLocalMsCtfMonitor(dwflags : super::DWORD) -> windows_sys::core::HRESULT);
windows_link::link!("msctfmonitor.dll" "system" fn UninitLocalMsCtfMonitor() -> windows_sys::core::HRESULT);
pub const DCM_FLAGS_CTFMON: i32 = 2;
pub const DCM_FLAGS_LOCALTHREADTSF: i32 = 4;
pub const DCM_FLAGS_TASKENG: i32 = 1;
pub const ILMCM_CHECKLAYOUTANDTIPENABLED: i32 = 1;
pub const ILMCM_LANGUAGEBAROFF: i32 = 2;
