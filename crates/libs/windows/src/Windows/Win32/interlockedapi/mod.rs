#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitializeSListHead() -> super::SLIST_HEADER {
    windows_core::link!("kernel32.dll" "system" fn InitializeSListHead(listhead : super::PSLIST_HEADER));
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitializeSListHead(&mut result__);
        result__
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InterlockedFlushSList(listhead: super::PSLIST_HEADER) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" fn InterlockedFlushSList(listhead : super::PSLIST_HEADER) -> super::PSLIST_ENTRY);
    unsafe { InterlockedFlushSList(listhead as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InterlockedPopEntrySList(listhead: super::PSLIST_HEADER) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" fn InterlockedPopEntrySList(listhead : super::PSLIST_HEADER) -> super::PSLIST_ENTRY);
    unsafe { InterlockedPopEntrySList(listhead as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InterlockedPushEntrySList(listhead: super::PSLIST_HEADER, listentry: super::PSLIST_ENTRY) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" fn InterlockedPushEntrySList(listhead : super::PSLIST_HEADER, listentry : super::PSLIST_ENTRY) -> super::PSLIST_ENTRY);
    unsafe { InterlockedPushEntrySList(listhead as _, listentry as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InterlockedPushListSListEx(listhead: super::PSLIST_HEADER, list: super::PSLIST_ENTRY, listend: super::PSLIST_ENTRY, count: u32) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" fn InterlockedPushListSListEx(listhead : super::PSLIST_HEADER, list : super::PSLIST_ENTRY, listend : super::PSLIST_ENTRY, count : u32) -> super::PSLIST_ENTRY);
    unsafe { InterlockedPushListSListEx(listhead as _, list as _, listend as _, count) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryDepthSList(listhead: super::PSLIST_HEADER) -> u16 {
    windows_core::link!("kernel32.dll" "system" fn QueryDepthSList(listhead : super::PSLIST_HEADER) -> u16);
    unsafe { QueryDepthSList(listhead) }
}
