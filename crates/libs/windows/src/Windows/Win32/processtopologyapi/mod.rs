#[cfg(feature = "Win32_winnt")]
#[inline]
pub unsafe fn GetProcessGroupAffinity(hprocess: super::winnt::HANDLE, groupcount: *mut u16, grouparray: *mut u16) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessGroupAffinity(hprocess : super::winnt::HANDLE, groupcount : *mut u16, grouparray : *mut u16) -> windows_core::BOOL);
    unsafe { GetProcessGroupAffinity(hprocess, groupcount as _, grouparray as _) }
}
#[cfg(all(feature = "Win32_basetsd", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn GetThreadGroupAffinity(hthread: super::winnt::HANDLE, groupaffinity: *mut super::winnt::GROUP_AFFINITY) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadGroupAffinity(hthread : super::winnt::HANDLE, groupaffinity : *mut super::winnt::GROUP_AFFINITY) -> windows_core::BOOL);
    unsafe { GetThreadGroupAffinity(hthread, groupaffinity as _) }
}
#[cfg(all(feature = "Win32_basetsd", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn SetThreadGroupAffinity(hthread: super::winnt::HANDLE, groupaffinity: *const super::winnt::GROUP_AFFINITY, previousgroupaffinity: Option<*mut super::winnt::GROUP_AFFINITY>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadGroupAffinity(hthread : super::winnt::HANDLE, groupaffinity : *const super::winnt::GROUP_AFFINITY, previousgroupaffinity : *mut super::winnt::GROUP_AFFINITY) -> windows_core::BOOL);
    unsafe { SetThreadGroupAffinity(hthread, groupaffinity, previousgroupaffinity.unwrap_or(core::mem::zeroed()) as _) }
}
