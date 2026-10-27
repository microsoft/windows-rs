#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InitializeSListHead() -> super::SLIST_HEADER {
    windows_core::link!("kernel32.dll" "system" fn InitializeSListHead(listhead : super::PSLIST_HEADER));
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitializeSListHead(&mut result__);
        result__
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InterlockedFlushSList(listhead: super::PSLIST_HEADER) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" fn InterlockedFlushSList(listhead : super::PSLIST_HEADER) -> super::PSLIST_ENTRY);
    unsafe { InterlockedFlushSList(listhead as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InterlockedPopEntrySList(listhead: super::PSLIST_HEADER) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" fn InterlockedPopEntrySList(listhead : super::PSLIST_HEADER) -> super::PSLIST_ENTRY);
    unsafe { InterlockedPopEntrySList(listhead as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InterlockedPushEntrySList(listhead: super::PSLIST_HEADER, listentry: super::PSLIST_ENTRY) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" fn InterlockedPushEntrySList(listhead : super::PSLIST_HEADER, listentry : super::PSLIST_ENTRY) -> super::PSLIST_ENTRY);
    unsafe { InterlockedPushEntrySList(listhead as _, listentry as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InterlockedPushListSList(listhead: super::PSLIST_HEADER, list: super::PSLIST_ENTRY, listend: super::PSLIST_ENTRY, count: super::ULONG) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" "InterlockedPushListSListEx" fn InterlockedPushListSList(listhead : super::PSLIST_HEADER, list : super::PSLIST_ENTRY, listend : super::PSLIST_ENTRY, count : super::ULONG) -> super::PSLIST_ENTRY);
    unsafe { InterlockedPushListSList(listhead as _, list as _, listend as _, count) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InterlockedPushListSListEx(listhead: super::PSLIST_HEADER, list: super::PSLIST_ENTRY, listend: super::PSLIST_ENTRY, count: super::ULONG) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" fn InterlockedPushListSListEx(listhead : super::PSLIST_HEADER, list : super::PSLIST_ENTRY, listend : super::PSLIST_ENTRY, count : super::ULONG) -> super::PSLIST_ENTRY);
    unsafe { InterlockedPushListSListEx(listhead as _, list as _, listend as _, count) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryDepthSList(listhead: super::PSLIST_HEADER) -> super::USHORT {
    windows_core::link!("kernel32.dll" "system" fn QueryDepthSList(listhead : super::PSLIST_HEADER) -> super::USHORT);
    unsafe { QueryDepthSList(listhead) }
}
