#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetNumaHighestNodeNumber(highestnodenumber : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMask2(nodenumber : u16, processormasks : super::PGROUP_AFFINITY, processormaskcount : u16, requiredmaskcount : super::PUSHORT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "basetsd", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMaskEx(node : u16, processormask : super::PGROUP_AFFINITY) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetNumaProximityNodeEx(proximityid : u32, nodenumber : super::PUSHORT) -> windows_sys::core::BOOL);
