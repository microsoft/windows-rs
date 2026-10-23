#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetNumaHighestNodeNumber(highestnodenumber : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMask2(nodenumber : super::USHORT, processormasks : super::PGROUP_AFFINITY, processormaskcount : super::USHORT, requiredmaskcount : super::PUSHORT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMaskEx(node : super::USHORT, processormask : super::PGROUP_AFFINITY) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("kernel32.dll" "system" fn GetNumaProximityNodeEx(proximityid : super::ULONG, nodenumber : super::PUSHORT) -> windows_sys::core::BOOL);
