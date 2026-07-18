#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InitializeSListHead(listhead : *mut super::SLIST_HEADER));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InterlockedFlushSList(listhead : *mut super::SLIST_HEADER) -> super::PSLIST_ENTRY);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InterlockedPopEntrySList(listhead : *mut super::SLIST_HEADER) -> super::PSLIST_ENTRY);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InterlockedPushEntrySList(listhead : *mut super::SLIST_HEADER, listentry : *mut super::SINGLE_LIST_ENTRY) -> super::PSLIST_ENTRY);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InterlockedPushEntrySList(listhead : *mut super::SLIST_HEADER, listentry : *mut super::SLIST_ENTRY) -> super::PSLIST_ENTRY);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InterlockedPushListSListEx(listhead : *mut super::SLIST_HEADER, list : *mut super::SINGLE_LIST_ENTRY, listend : *mut super::SINGLE_LIST_ENTRY, count : u32) -> super::PSLIST_ENTRY);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InterlockedPushListSListEx(listhead : *mut super::SLIST_HEADER, list : *mut super::SLIST_ENTRY, listend : *mut super::SLIST_ENTRY, count : u32) -> super::PSLIST_ENTRY);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryDepthSList(listhead : *const super::SLIST_HEADER) -> u16);
