#[inline]
pub unsafe fn GetNumaHighestNodeNumber(highestnodenumber: *mut u32) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaHighestNodeNumber(highestnodenumber : *mut u32) -> windows_core::BOOL);
    unsafe { GetNumaHighestNodeNumber(highestnodenumber as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn GetNumaNodeProcessorMask2(nodenumber: u16, processormasks: Option<&mut [super::GROUP_AFFINITY]>, requiredmaskcount: *mut u16) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMask2(nodenumber : u16, processormasks : *mut super::GROUP_AFFINITY, processormaskcount : u16, requiredmaskcount : *mut u16) -> windows_core::BOOL);
    unsafe { GetNumaNodeProcessorMask2(nodenumber, processormasks.as_deref().map_or(core::ptr::null_mut(), |slice| slice.as_ptr().cast_mut()), processormasks.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()), requiredmaskcount as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn GetNumaNodeProcessorMaskEx(node: u16, processormask: *mut super::GROUP_AFFINITY) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMaskEx(node : u16, processormask : *mut super::GROUP_AFFINITY) -> windows_core::BOOL);
    unsafe { GetNumaNodeProcessorMaskEx(node, processormask as _) }
}
#[inline]
pub unsafe fn GetNumaProximityNodeEx(proximityid: u32, nodenumber: *mut u16) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaProximityNodeEx(proximityid : u32, nodenumber : *mut u16) -> windows_core::BOOL);
    unsafe { GetNumaProximityNodeEx(proximityid, nodenumber as _) }
}
