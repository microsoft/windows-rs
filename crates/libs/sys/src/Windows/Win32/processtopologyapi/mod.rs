#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessGroupAffinity(hprocess : super::HANDLE, groupcount : *mut u16, grouparray : *mut u16) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadGroupAffinity(hthread : super::HANDLE, groupaffinity : *mut super::GROUP_AFFINITY) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadGroupAffinity(hthread : super::HANDLE, groupaffinity : *const super::GROUP_AFFINITY, previousgroupaffinity : *mut super::GROUP_AFFINITY) -> windows_sys::core::BOOL);
