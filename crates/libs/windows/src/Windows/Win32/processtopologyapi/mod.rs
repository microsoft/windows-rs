#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessGroupAffinity(hprocess: super::HANDLE, groupcount: super::PUSHORT, grouparray: super::PUSHORT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessGroupAffinity(hprocess : super::HANDLE, groupcount : super::PUSHORT, grouparray : super::PUSHORT) -> windows_core::BOOL);
    unsafe { GetProcessGroupAffinity(hprocess, groupcount as _, grouparray as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn GetThreadGroupAffinity(hthread: super::HANDLE, groupaffinity: super::PGROUP_AFFINITY) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadGroupAffinity(hthread : super::HANDLE, groupaffinity : super::PGROUP_AFFINITY) -> windows_core::BOOL);
    unsafe { GetThreadGroupAffinity(hthread, groupaffinity as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn SetThreadGroupAffinity(hthread: super::HANDLE, groupaffinity: *const super::GROUP_AFFINITY, previousgroupaffinity: Option<super::PGROUP_AFFINITY>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetThreadGroupAffinity(hthread : super::HANDLE, groupaffinity : *const super::GROUP_AFFINITY, previousgroupaffinity : super::PGROUP_AFFINITY) -> windows_core::BOOL);
    unsafe { SetThreadGroupAffinity(hthread, groupaffinity, previousgroupaffinity.unwrap_or(core::mem::zeroed()) as _) }
}
