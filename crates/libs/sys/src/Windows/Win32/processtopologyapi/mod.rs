#[cfg(feature = "Win32_winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessGroupAffinity(hprocess : super::winnt::HANDLE, groupcount : *mut u16, grouparray : *mut u16) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_basetsd", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadGroupAffinity(hthread : super::winnt::HANDLE, groupaffinity : *mut super::winnt::GROUP_AFFINITY) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_basetsd", feature = "Win32_winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadGroupAffinity(hthread : super::winnt::HANDLE, groupaffinity : *const super::winnt::GROUP_AFFINITY, previousgroupaffinity : *mut super::winnt::GROUP_AFFINITY) -> windows_sys::core::BOOL);
