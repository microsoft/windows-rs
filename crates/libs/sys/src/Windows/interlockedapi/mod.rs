#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InitializeSListHead(listhead : *mut super::winnt::SLIST_HEADER));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InterlockedFlushSList(listhead : *mut super::winnt::SLIST_HEADER) -> super::winnt::PSLIST_ENTRY);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InterlockedPopEntrySList(listhead : *mut super::winnt::SLIST_HEADER) -> super::winnt::PSLIST_ENTRY);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InterlockedPushEntrySList(listhead : *mut super::winnt::SLIST_HEADER, listentry : *mut super::winnt::SINGLE_LIST_ENTRY) -> super::winnt::PSLIST_ENTRY);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InterlockedPushEntrySList(listhead : *mut super::winnt::SLIST_HEADER, listentry : *mut super::winnt::SLIST_ENTRY) -> super::winnt::PSLIST_ENTRY);
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InterlockedPushListSListEx(listhead : *mut super::winnt::SLIST_HEADER, list : *mut super::winnt::SINGLE_LIST_ENTRY, listend : *mut super::winnt::SINGLE_LIST_ENTRY, count : u32) -> super::winnt::PSLIST_ENTRY);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InterlockedPushListSListEx(listhead : *mut super::winnt::SLIST_HEADER, list : *mut super::winnt::SLIST_ENTRY, listend : *mut super::winnt::SLIST_ENTRY, count : u32) -> super::winnt::PSLIST_ENTRY);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryDepthSList(listhead : *const super::winnt::SLIST_HEADER) -> u16);
