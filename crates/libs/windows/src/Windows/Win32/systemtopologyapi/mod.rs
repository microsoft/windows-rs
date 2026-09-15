#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetNumaHighestNodeNumber(highestnodenumber: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaHighestNodeNumber(highestnodenumber : super::PULONG) -> windows_core::BOOL);
    unsafe { GetNumaHighestNodeNumber(highestnodenumber as _) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetNumaNodeProcessorMask2(nodenumber: u16, processormasks: Option<super::PGROUP_AFFINITY>, processormaskcount: u16, requiredmaskcount: super::PUSHORT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMask2(nodenumber : u16, processormasks : super::PGROUP_AFFINITY, processormaskcount : u16, requiredmaskcount : super::PUSHORT) -> windows_core::BOOL);
    unsafe { GetNumaNodeProcessorMask2(nodenumber, processormasks.unwrap_or(core::mem::zeroed()) as _, processormaskcount, requiredmaskcount as _) }
}
#[cfg(all(feature = "basetsd", feature = "winnt"))]
#[inline]
pub unsafe fn GetNumaNodeProcessorMaskEx(node: u16, processormask: super::PGROUP_AFFINITY) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMaskEx(node : u16, processormask : super::PGROUP_AFFINITY) -> windows_core::BOOL);
    unsafe { GetNumaNodeProcessorMaskEx(node, processormask as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetNumaProximityNodeEx(proximityid: u32, nodenumber: super::PUSHORT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaProximityNodeEx(proximityid : u32, nodenumber : super::PUSHORT) -> windows_core::BOOL);
    unsafe { GetNumaProximityNodeEx(proximityid, nodenumber as _) }
}
