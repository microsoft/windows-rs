#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InitializeSListHead() -> super::SLIST_HEADER {
    windows_core::link!("kernel32.dll" "system" fn InitializeSListHead(listhead : *mut super::SLIST_HEADER));
    unsafe {
        let mut result__ = core::mem::zeroed();
        InitializeSListHead(&mut result__);
        result__
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InterlockedFlushSList(listhead: *mut super::SLIST_HEADER) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" fn InterlockedFlushSList(listhead : *mut super::SLIST_HEADER) -> super::PSLIST_ENTRY);
    unsafe { InterlockedFlushSList(listhead as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InterlockedPopEntrySList(listhead: *mut super::SLIST_HEADER) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" fn InterlockedPopEntrySList(listhead : *mut super::SLIST_HEADER) -> super::PSLIST_ENTRY);
    unsafe { InterlockedPopEntrySList(listhead as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InterlockedPushEntrySList(listhead: *mut super::SLIST_HEADER, listentry: *mut super::SINGLE_LIST_ENTRY) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" fn InterlockedPushEntrySList(listhead : *mut super::SLIST_HEADER, listentry : *mut super::SINGLE_LIST_ENTRY) -> super::PSLIST_ENTRY);
    unsafe { InterlockedPushEntrySList(listhead as _, listentry as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InterlockedPushEntrySList(listhead: *mut super::SLIST_HEADER, listentry: *mut super::SLIST_ENTRY) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" fn InterlockedPushEntrySList(listhead : *mut super::SLIST_HEADER, listentry : *mut super::SLIST_ENTRY) -> super::PSLIST_ENTRY);
    unsafe { InterlockedPushEntrySList(listhead as _, listentry as _) }
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InterlockedPushListSListEx(listhead: *mut super::SLIST_HEADER, list: *mut super::SINGLE_LIST_ENTRY, listend: *mut super::SINGLE_LIST_ENTRY, count: u32) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" fn InterlockedPushListSListEx(listhead : *mut super::SLIST_HEADER, list : *mut super::SINGLE_LIST_ENTRY, listend : *mut super::SINGLE_LIST_ENTRY, count : u32) -> super::PSLIST_ENTRY);
    unsafe { InterlockedPushListSListEx(listhead as _, list as _, listend as _, count) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn InterlockedPushListSListEx(listhead: *mut super::SLIST_HEADER, list: *mut super::SLIST_ENTRY, listend: *mut super::SLIST_ENTRY, count: u32) -> super::PSLIST_ENTRY {
    windows_core::link!("kernel32.dll" "system" fn InterlockedPushListSListEx(listhead : *mut super::SLIST_HEADER, list : *mut super::SLIST_ENTRY, listend : *mut super::SLIST_ENTRY, count : u32) -> super::PSLIST_ENTRY);
    unsafe { InterlockedPushListSListEx(listhead as _, list as _, listend as _, count) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn QueryDepthSList(listhead: *const super::SLIST_HEADER) -> u16 {
    windows_core::link!("kernel32.dll" "system" fn QueryDepthSList(listhead : *const super::SLIST_HEADER) -> u16);
    unsafe { QueryDepthSList(listhead) }
}
