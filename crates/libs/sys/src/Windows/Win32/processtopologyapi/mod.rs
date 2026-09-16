#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetProcessGroupAffinity(hprocess : super::HANDLE, groupcount : super::PUSHORT, grouparray : super::PUSHORT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetThreadGroupAffinity(hthread : super::HANDLE, groupaffinity : super::PGROUP_AFFINITY) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn SetThreadGroupAffinity(hthread : super::HANDLE, groupaffinity : *const super::GROUP_AFFINITY, previousgroupaffinity : super::PGROUP_AFFINITY) -> windows_sys::core::BOOL);
