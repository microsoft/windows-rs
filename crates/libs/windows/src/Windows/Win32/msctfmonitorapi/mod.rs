#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn DoMsCtfMonitor(dwflags: u32, heventforservicestop: super::winnt::HANDLE) -> windows_core::BOOL {
    windows_core::link!("msctfmonitor.dll" "system" fn DoMsCtfMonitor(dwflags : u32, heventforservicestop : super::winnt::HANDLE) -> windows_core::BOOL);
    unsafe { DoMsCtfMonitor(dwflags, heventforservicestop) }
}
#[inline]
pub unsafe fn InitLocalMsCtfMonitor(dwflags: u32) -> windows_core::HRESULT {
    windows_core::link!("msctfmonitor.dll" "system" fn InitLocalMsCtfMonitor(dwflags : u32) -> windows_core::HRESULT);
    unsafe { InitLocalMsCtfMonitor(dwflags) }
}
#[inline]
pub unsafe fn UninitLocalMsCtfMonitor() -> windows_core::HRESULT {
    windows_core::link!("msctfmonitor.dll" "system" fn UninitLocalMsCtfMonitor() -> windows_core::HRESULT);
    unsafe { UninitLocalMsCtfMonitor() }
}
pub const DCM_FLAGS_CTFMON: u32 = 2;
pub const DCM_FLAGS_LOCALTHREADTSF: u32 = 4;
pub const DCM_FLAGS_TASKENG: u32 = 1;
pub const ILMCM_CHECKLAYOUTANDTIPENABLED: u32 = 1;
pub const ILMCM_LANGUAGEBAROFF: u32 = 2;
