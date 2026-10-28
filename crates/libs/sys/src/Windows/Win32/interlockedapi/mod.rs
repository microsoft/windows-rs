#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InitializeSListHead(listhead : super::PSLIST_HEADER));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InterlockedFlushSList(listhead : super::PSLIST_HEADER) -> super::PSLIST_ENTRY);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InterlockedPopEntrySList(listhead : super::PSLIST_HEADER) -> super::PSLIST_ENTRY);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InterlockedPushEntrySList(listhead : super::PSLIST_HEADER, listentry : super::PSLIST_ENTRY) -> super::PSLIST_ENTRY);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" "InterlockedPushListSListEx" fn InterlockedPushListSList(listhead : super::PSLIST_HEADER, list : super::PSLIST_ENTRY, listend : super::PSLIST_ENTRY, count : super::ULONG) -> super::PSLIST_ENTRY);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn InterlockedPushListSListEx(listhead : super::PSLIST_HEADER, list : super::PSLIST_ENTRY, listend : super::PSLIST_ENTRY, count : super::ULONG) -> super::PSLIST_ENTRY);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn QueryDepthSList(listhead : super::PSLIST_HEADER) -> super::USHORT);
