#[cfg(feature = "winnt")]
windows_link::link!("msctfmonitor.dll" "system" fn DoMsCtfMonitor(dwflags : u32, heventforservicestop : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("msctfmonitor.dll" "system" fn InitLocalMsCtfMonitor(dwflags : u32) -> windows_sys::core::HRESULT);
windows_link::link!("msctfmonitor.dll" "system" fn UninitLocalMsCtfMonitor() -> windows_sys::core::HRESULT);
pub const DCM_FLAGS_CTFMON: u32 = 2;
pub const DCM_FLAGS_LOCALTHREADTSF: u32 = 4;
pub const DCM_FLAGS_TASKENG: u32 = 1;
pub const ILMCM_CHECKLAYOUTANDTIPENABLED: u32 = 1;
pub const ILMCM_LANGUAGEBAROFF: u32 = 2;
