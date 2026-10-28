#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetNumaHighestNodeNumber(highestnodenumber: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaHighestNodeNumber(highestnodenumber : super::PULONG) -> windows_core::BOOL);
    unsafe { GetNumaHighestNodeNumber(highestnodenumber as _) }
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetNumaNodeProcessorMask2(nodenumber: super::USHORT, processormasks: Option<super::PGROUP_AFFINITY>, processormaskcount: super::USHORT, requiredmaskcount: super::PUSHORT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMask2(nodenumber : super::USHORT, processormasks : super::PGROUP_AFFINITY, processormaskcount : super::USHORT, requiredmaskcount : super::PUSHORT) -> windows_core::BOOL);
    unsafe { GetNumaNodeProcessorMask2(nodenumber, processormasks.unwrap_or(core::mem::zeroed()) as _, processormaskcount, requiredmaskcount as _) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetNumaNodeProcessorMaskEx(node: super::USHORT, processormask: super::PGROUP_AFFINITY) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMaskEx(node : super::USHORT, processormask : super::PGROUP_AFFINITY) -> windows_core::BOOL);
    unsafe { GetNumaNodeProcessorMaskEx(node, processormask as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetNumaProximityNodeEx(proximityid: super::ULONG, nodenumber: super::PUSHORT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaProximityNodeEx(proximityid : super::ULONG, nodenumber : super::PUSHORT) -> windows_core::BOOL);
    unsafe { GetNumaProximityNodeEx(proximityid, nodenumber as _) }
}
