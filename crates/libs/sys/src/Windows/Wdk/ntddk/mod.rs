windows_link::link!("ntdll.dll" "system" fn DbgPrompt(prompt : windows_sys::core::PCSTR, response : *mut i8, length : u32) -> u32);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_d3dkmthk", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt", feature = "Win32_winternl"))]
windows_link::link!("ntdll.dll" "system" fn NtOpenProcess(processhandle : *mut super::super::Win32::winnt::HANDLE, desiredaccess : super::super::Win32::winnt::ACCESS_MASK, objectattributes : *const super::super::Win32::d3dkmthk::OBJECT_ATTRIBUTES, clientid : *const super::super::Win32::winternl::CLIENT_ID) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlCompareString(string1 : *const super::super::Win32::ntsecapi::STRING, string2 : *const super::super::Win32::ntsecapi::STRING, caseinsensitive : bool) -> i32);
windows_link::link!("ntdll.dll" "system" fn RtlContractHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE) -> bool);
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlCopyString(destinationstring : *mut super::super::Win32::ntsecapi::STRING, sourcestring : *const super::super::Win32::ntsecapi::STRING));
windows_link::link!("ntdll.dll" "system" fn RtlCreateHashTable(hashtable : *mut PRTL_DYNAMIC_HASH_TABLE, shift : u32, flags : u32) -> bool);
windows_link::link!("ntdll.dll" "system" fn RtlCreateHashTableEx(hashtable : *mut PRTL_DYNAMIC_HASH_TABLE, initialsize : u32, shift : u32, flags : u32) -> bool);
windows_link::link!("ntdll.dll" "system" fn RtlDelete(links : *const RTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS);
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlDeleteElementGenericTable(table : *const RTL_GENERIC_TABLE, buffer : *const core::ffi::c_void) -> bool);
#[cfg(feature = "Wdk_ntdef")]
windows_link::link!("ntdll.dll" "system" fn RtlDeleteElementGenericTableAvl(table : *const RTL_AVL_TABLE, buffer : *const core::ffi::c_void) -> bool);
#[cfg(feature = "Wdk_ntdef")]
windows_link::link!("ntdll.dll" "system" fn RtlDeleteElementGenericTableAvlEx(table : *const RTL_AVL_TABLE, nodeorparent : *const core::ffi::c_void));
windows_link::link!("ntdll.dll" "system" fn RtlDeleteHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE));
windows_link::link!("ntdll.dll" "system" fn RtlDeleteNoSplay(links : *const RTL_SPLAY_LINKS, root : *mut PRTL_SPLAY_LINKS));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlEndEnumerationHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlEndStrongEnumerationHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlEndWeakEnumerationHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR));
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlEnumerateEntryHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY);
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlEnumerateGenericTable(table : *const RTL_GENERIC_TABLE, restart : bool) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_ntdef")]
windows_link::link!("ntdll.dll" "system" fn RtlEnumerateGenericTableAvl(table : *const RTL_AVL_TABLE, restart : bool) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_bcrypt"))]
windows_link::link!("ntdll.dll" "system" fn RtlEnumerateGenericTableLikeADirectory(table : *const RTL_AVL_TABLE, matchfunction : PRTL_AVL_MATCH_FUNCTION, matchdata : *const core::ffi::c_void, nextflag : u32, restartkey : *mut *mut core::ffi::c_void, deletecount : *mut u32, buffer : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlEnumerateGenericTableWithoutSplaying(table : *const RTL_GENERIC_TABLE, restartkey : *mut *mut core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_ntdef")]
windows_link::link!("ntdll.dll" "system" fn RtlEnumerateGenericTableWithoutSplayingAvl(table : *const RTL_AVL_TABLE, restartkey : *mut *mut core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlEqualString(string1 : *const super::super::Win32::ntsecapi::STRING, string2 : *const super::super::Win32::ntsecapi::STRING, caseinsensitive : bool) -> bool);
windows_link::link!("ntdll.dll" "system" fn RtlExpandHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE) -> bool);
windows_link::link!("ntdll.dll" "system" fn RtlGetActiveConsoleId() -> u32);
windows_link::link!("ntdll.dll" "system" fn RtlGetCallersAddress(callersaddress : *mut *mut core::ffi::c_void, callerscaller : *mut *mut core::ffi::c_void));
windows_link::link!("ntdll.dll" "system" fn RtlGetConsoleSessionForegroundProcessId() -> u64);
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlGetElementGenericTable(table : *const RTL_GENERIC_TABLE, i : u32) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_ntdef")]
windows_link::link!("ntdll.dll" "system" fn RtlGetElementGenericTableAvl(table : *const RTL_AVL_TABLE, i : u32) -> *mut core::ffi::c_void);
windows_link::link!("ntdll.dll" "system" fn RtlGetEnabledExtendedFeatures(featuremask : u64) -> u64);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlGetNextEntryHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, context : *const RTL_DYNAMIC_HASH_TABLE_CONTEXT) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY);
#[cfg(feature = "Wdk_ntdef")]
windows_link::link!("ntdll.dll" "system" fn RtlGetNtProductType(ntproducttype : *mut super::ntdef::NT_PRODUCT_TYPE) -> bool);
windows_link::link!("ntdll.dll" "system" fn RtlGetNtSystemRoot() -> windows_sys::core::PCWSTR);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("ntdll.dll" "system" fn RtlGetPersistedStateLocation(sourceid : windows_sys::core::PCWSTR, customvalue : windows_sys::core::PCWSTR, defaultpath : windows_sys::core::PCWSTR, statelocationtype : STATE_LOCATION_TYPE, targetpath : *mut u16, bufferlengthin : u32, bufferlengthout : *mut u32) -> super::super::Win32::bcrypt::NTSTATUS);
windows_link::link!("ntdll.dll" "system" fn RtlGetSuiteMask() -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlInitEnumerationHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlInitStrongEnumerationHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> bool);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlInitWeakEnumerationHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> bool);
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInitializeGenericTable(table : *mut RTL_GENERIC_TABLE, compareroutine : PRTL_GENERIC_COMPARE_ROUTINE, allocateroutine : PRTL_GENERIC_ALLOCATE_ROUTINE, freeroutine : PRTL_GENERIC_FREE_ROUTINE, tablecontext : *const core::ffi::c_void));
#[cfg(feature = "Wdk_ntdef")]
windows_link::link!("ntdll.dll" "system" fn RtlInitializeGenericTableAvl(table : *mut RTL_AVL_TABLE, compareroutine : PRTL_AVL_COMPARE_ROUTINE, allocateroutine : PRTL_AVL_ALLOCATE_ROUTINE, freeroutine : PRTL_AVL_FREE_ROUTINE, tablecontext : *const core::ffi::c_void));
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInsertElementGenericTable(table : *const RTL_GENERIC_TABLE, buffer : *const core::ffi::c_void, buffersize : super::ntdef::CLONG, newelement : *mut bool) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_ntdef")]
windows_link::link!("ntdll.dll" "system" fn RtlInsertElementGenericTableAvl(table : *const RTL_AVL_TABLE, buffer : *const core::ffi::c_void, buffersize : super::ntdef::CLONG, newelement : *mut bool) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInsertElementGenericTableFull(table : *const RTL_GENERIC_TABLE, buffer : *const core::ffi::c_void, buffersize : super::ntdef::CLONG, newelement : *mut bool, nodeorparent : *const core::ffi::c_void, searchresult : TABLE_SEARCH_RESULT) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_ntdef")]
windows_link::link!("ntdll.dll" "system" fn RtlInsertElementGenericTableFullAvl(table : *const RTL_AVL_TABLE, buffer : *const core::ffi::c_void, buffersize : super::ntdef::CLONG, newelement : *mut bool, nodeorparent : *const core::ffi::c_void, searchresult : TABLE_SEARCH_RESULT) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlInsertEntryHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, entry : *const RTL_DYNAMIC_HASH_TABLE_ENTRY, signature : usize, context : *mut RTL_DYNAMIC_HASH_TABLE_CONTEXT) -> bool);
#[cfg(feature = "Win32_bcrypt")]
windows_link::link!("ntdll.dll" "system" fn RtlIsApiSetImplemented(apisetname : windows_sys::core::PCSTR) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlIsGenericTableEmpty(table : *const RTL_GENERIC_TABLE) -> bool);
#[cfg(feature = "Wdk_ntdef")]
windows_link::link!("ntdll.dll" "system" fn RtlIsGenericTableEmptyAvl(table : *const RTL_AVL_TABLE) -> bool);
windows_link::link!("ntdll.dll" "system" fn RtlIsMultiSessionSku() -> bool);
windows_link::link!("ntdll.dll" "system" fn RtlIsMultiUsersInSessionSku() -> bool);
windows_link::link!("ntdll.dll" "system" fn RtlIsStateSeparationEnabled() -> bool);
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlLookupElementGenericTable(table : *const RTL_GENERIC_TABLE, buffer : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_ntdef")]
windows_link::link!("ntdll.dll" "system" fn RtlLookupElementGenericTableAvl(table : *const RTL_AVL_TABLE, buffer : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlLookupElementGenericTableFull(table : *const RTL_GENERIC_TABLE, buffer : *const core::ffi::c_void, nodeorparent : *mut *mut core::ffi::c_void, searchresult : *mut TABLE_SEARCH_RESULT) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_ntdef")]
windows_link::link!("ntdll.dll" "system" fn RtlLookupElementGenericTableFullAvl(table : *const RTL_AVL_TABLE, buffer : *const core::ffi::c_void, nodeorparent : *mut *mut core::ffi::c_void, searchresult : *mut TABLE_SEARCH_RESULT) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlLookupEntryHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, signature : usize, context : *mut RTL_DYNAMIC_HASH_TABLE_CONTEXT) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY);
#[cfg(feature = "Wdk_ntdef")]
windows_link::link!("ntdll.dll" "system" fn RtlLookupFirstMatchingElementGenericTableAvl(table : *const RTL_AVL_TABLE, buffer : *const core::ffi::c_void, restartkey : *mut *mut core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlMapGenericMask(accessmask : *mut super::super::Win32::winnt::ACCESS_MASK, genericmapping : *const super::super::Win32::winnt::GENERIC_MAPPING));
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlNumberGenericTableElements(table : *const RTL_GENERIC_TABLE) -> u32);
#[cfg(feature = "Wdk_ntdef")]
windows_link::link!("ntdll.dll" "system" fn RtlNumberGenericTableElementsAvl(table : *const RTL_AVL_TABLE) -> u32);
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
windows_link::link!("ntdll.dll" "system" fn RtlPrefixUnicodeString(string1 : *const super::super::Win32::ntsecapi::UNICODE_STRING, string2 : *const super::super::Win32::ntsecapi::UNICODE_STRING, caseinsensitive : bool) -> bool);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlQueryRegistryValueWithFallback(primaryhandle : super::super::Win32::winnt::HANDLE, fallbackhandle : super::super::Win32::winnt::HANDLE, valuename : *const super::super::Win32::ntsecapi::UNICODE_STRING, valuelength : u32, valuetype : *mut u32, valuedata : *mut core::ffi::c_void, resultlength : *mut u32) -> super::super::Win32::bcrypt::NTSTATUS);
windows_link::link!("ntdll.dll" "system" fn RtlRealPredecessor(links : *const RTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS);
windows_link::link!("ntdll.dll" "system" fn RtlRealSuccessor(links : *const RTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlRemoveEntryHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, entry : *const RTL_DYNAMIC_HASH_TABLE_ENTRY, context : *mut RTL_DYNAMIC_HASH_TABLE_CONTEXT) -> bool);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlRunOnceBeginInitialize(runonce : *mut super::super::Win32::winnt::RTL_RUN_ONCE, flags : u32, context : *mut *mut core::ffi::c_void) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlRunOnceComplete(runonce : *mut super::super::Win32::winnt::RTL_RUN_ONCE, flags : u32, context : *const core::ffi::c_void) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlRunOnceExecuteOnce(runonce : *mut super::super::Win32::winnt::RTL_RUN_ONCE, initfn : PRTL_RUN_ONCE_INIT_FN, parameter : *mut core::ffi::c_void, context : *mut *mut core::ffi::c_void) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlRunOnceInitialize(runonce : *mut super::super::Win32::winnt::RTL_RUN_ONCE));
windows_link::link!("ntdll.dll" "system" fn RtlSplay(links : *mut RTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlStronglyEnumerateEntryHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY);
windows_link::link!("ntdll.dll" "system" fn RtlSubtreePredecessor(links : *const RTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS);
windows_link::link!("ntdll.dll" "system" fn RtlSubtreeSuccessor(links : *const RTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
windows_link::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeString(destinationstring : *mut super::super::Win32::ntsecapi::UNICODE_STRING, sourcestring : *const super::super::Win32::ntsecapi::UNICODE_STRING, allocatedestinationstring : bool) -> super::super::Win32::bcrypt::NTSTATUS);
windows_link::link!("ntdll.dll" "system" fn RtlUpperChar(character : i8) -> i8);
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlUpperString(destinationstring : *mut super::super::Win32::ntsecapi::STRING, sourcestring : *const super::super::Win32::ntsecapi::STRING));
windows_link::link!("ntdll.dll" "system" fn RtlWalkFrameChain(callers : *mut *mut core::ffi::c_void, count : u32, flags : u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlWeaklyEnumerateEntryHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn ZwAllocateLocallyUniqueId(luid : *mut super::super::Win32::winnt::LUID) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "C" fn ZwCancelTimer(timerhandle : super::super::Win32::winnt::HANDLE, currentstate : *mut bool) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_bcrypt", feature = "Win32_d3dkmthk", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "C" fn ZwCreateTimer(timerhandle : *mut super::super::Win32::winnt::HANDLE, desiredaccess : super::super::Win32::winnt::ACCESS_MASK, objectattributes : *const super::super::Win32::d3dkmthk::OBJECT_ATTRIBUTES, timertype : super::ntdef::TIMER_TYPE) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt", feature = "Win32_winternl"))]
windows_link::link!("ntdll.dll" "system" fn ZwDeviceIoControlFile(filehandle : super::super::Win32::winnt::HANDLE, event : super::super::Win32::winnt::HANDLE, apcroutine : super::super::Win32::winternl::PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::Win32::winternl::IO_STATUS_BLOCK, iocontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
windows_link::link!("ntdll.dll" "system" fn ZwDisplayString(string : *const super::super::Win32::ntsecapi::UNICODE_STRING) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_d3dkmthk", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt", feature = "Win32_winternl"))]
windows_link::link!("ntdll.dll" "system" fn ZwOpenProcess(processhandle : *mut super::super::Win32::winnt::HANDLE, desiredaccess : super::super::Win32::winnt::ACCESS_MASK, objectattributes : *const super::super::Win32::d3dkmthk::OBJECT_ATTRIBUTES, clientid : *const super::super::Win32::winternl::CLIENT_ID) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_d3dkmthk", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "C" fn ZwOpenTimer(timerhandle : *mut super::super::Win32::winnt::HANDLE, desiredaccess : super::super::Win32::winnt::ACCESS_MASK, objectattributes : *const super::super::Win32::d3dkmthk::OBJECT_ATTRIBUTES) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn ZwPowerInformation(informationlevel : super::super::Win32::winnt::POWER_INFORMATION_LEVEL, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt", feature = "Win32_winnt", feature = "Win32_winternl"))]
windows_link::link!("ntdll.dll" "system" fn ZwQueryVolumeInformationFile(filehandle : super::super::Win32::winnt::HANDLE, iostatusblock : *mut super::super::Win32::winternl::IO_STATUS_BLOCK, fsinformation : *mut core::ffi::c_void, length : u32, fsinformationclass : super::wdm::FS_INFORMATION_CLASS) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt", feature = "Win32_winternl"))]
windows_link::link!("ntdll.dll" "system" fn ZwSetInformationThread(threadhandle : super::super::Win32::winnt::HANDLE, threadinformationclass : super::super::Win32::winternl::THREADINFOCLASS, threadinformation : *const core::ffi::c_void, threadinformationlength : u32) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "C" fn ZwSetTimer(timerhandle : super::super::Win32::winnt::HANDLE, duetime : *const i64, timerapcroutine : PTIMER_APC_ROUTINE, timercontext : *const core::ffi::c_void, resumetimer : bool, period : i32, previousstate : *mut bool) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "C" fn ZwSetTimerEx(timerhandle : super::super::Win32::winnt::HANDLE, timersetinformationclass : TIMER_SET_INFORMATION_CLASS, timersetinformation : *mut core::ffi::c_void, timersetinformationlength : u32) -> super::super::Win32::bcrypt::NTSTATUS);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
windows_link::link!("ntdll.dll" "system" fn ZwTerminateProcess(processhandle : super::super::Win32::winnt::HANDLE, exitstatus : super::super::Win32::bcrypt::NTSTATUS) -> super::super::Win32::bcrypt::NTSTATUS);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ACPI_DEBUGGING_DEVICE_IN_USE {
    pub NameSpacePathLength: u32,
    pub NameSpacePath: [u16; 1],
}
impl Default for ACPI_DEBUGGING_DEVICE_IN_USE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union AER_BRIDGE_DESCRIPTOR_FLAGS {
    pub Anonymous: AER_BRIDGE_DESCRIPTOR_FLAGS_0,
    pub AsUSHORT: u16,
}
impl Default for AER_BRIDGE_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct AER_BRIDGE_DESCRIPTOR_FLAGS_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union AER_ENDPOINT_DESCRIPTOR_FLAGS {
    pub Anonymous: AER_ENDPOINT_DESCRIPTOR_FLAGS_0,
    pub AsUSHORT: u16,
}
impl Default for AER_ENDPOINT_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct AER_ENDPOINT_DESCRIPTOR_FLAGS_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union AER_ROOTPORT_DESCRIPTOR_FLAGS {
    pub Anonymous: AER_ROOTPORT_DESCRIPTOR_FLAGS_0,
    pub AsUSHORT: u16,
}
impl Default for AER_ROOTPORT_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct AER_ROOTPORT_DESCRIPTOR_FLAGS_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct AGP_TARGET_BUS_INTERFACE_STANDARD {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: super::wdm::PINTERFACE_REFERENCE,
    pub InterfaceDereference: super::wdm::PINTERFACE_DEREFERENCE,
    pub SetBusData: super::wdm::PGET_SET_DEVICE_DATA,
    pub GetBusData: super::wdm::PGET_SET_DEVICE_DATA,
    pub CapabilityID: u8,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for AGP_TARGET_BUS_INTERFACE_STANDARD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const AMD_DRAM_TO_NORMALIZED_PRM_HANDLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0639bd1c_3e33_4055_bae7_36cceba8376e);
pub const AMD_DRAM_TO_SPA_PRM_HANDLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x69aa0a9c_e3fc_4b0d_929e_aa1bde5d9a9b);
pub const AMD_NORMALIZED_TO_DRAM_PRM_HANDLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7626c6ae_f973_429c_a91c_107d7be298b0);
pub const AMD_NORMALIZED_TO_SPA_PRM_HANDLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe7180659_a65d_451d_92cd_2b56f12beba6);
pub const AMD_SPA_TO_DRAM_PRM_HANDLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd1c6b8f2_f9ac_4bf0_855e_dbd582ce4b20);
pub const AMD_SPA_TO_NORMALIZED_PRM_HANDLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00c77891_7fcb_4d01_94e1_72f8e4ee1af7);
pub type ARBITER_ACTION = i32;
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub struct ARBITER_ADD_RESERVED_PARAMETERS {
    pub ReserveDevice: super::wdm::PDEVICE_OBJECT,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for ARBITER_ADD_RESERVED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct ARBITER_BOOT_ALLOCATION_PARAMETERS {
    pub ArbitrationList: super::super::Win32::winnt::PLIST_ENTRY,
}
#[cfg(feature = "Win32_winnt")]
impl Default for ARBITER_BOOT_ALLOCATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub struct ARBITER_CONFLICT_INFO {
    pub OwningObject: super::wdm::PDEVICE_OBJECT,
    pub Start: u64,
    pub End: u64,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for ARBITER_CONFLICT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ARBITER_FLAG_BOOT_CONFIG: u32 = 1;
pub const ARBITER_FLAG_OTHER_ENUM: u32 = 4;
pub const ARBITER_FLAG_ROOT_ENUM: u32 = 2;
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub struct ARBITER_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: super::wdm::PINTERFACE_REFERENCE,
    pub InterfaceDereference: super::wdm::PINTERFACE_DEREFERENCE,
    pub ArbiterHandler: PARBITER_HANDLER,
    pub Flags: u32,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for ARBITER_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub struct ARBITER_LIST_ENTRY {
    pub ListEntry: super::super::Win32::winnt::LIST_ENTRY,
    pub AlternativeCount: u32,
    pub Alternatives: super::wdm::PIO_RESOURCE_DESCRIPTOR,
    pub PhysicalDeviceObject: super::wdm::PDEVICE_OBJECT,
    pub RequestSource: ARBITER_REQUEST_SOURCE,
    pub Flags: u32,
    pub WorkSpace: isize,
    pub InterfaceType: super::wdm::INTERFACE_TYPE,
    pub SlotNumber: u32,
    pub BusNumber: u32,
    pub Assignment: super::wdm::PCM_PARTIAL_RESOURCE_DESCRIPTOR,
    pub SelectedAlternative: super::wdm::PIO_RESOURCE_DESCRIPTOR,
    pub Result: ARBITER_RESULT,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for ARBITER_LIST_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub struct ARBITER_PARAMETERS {
    pub Parameters: ARBITER_PARAMETERS_0,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for ARBITER_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub union ARBITER_PARAMETERS_0 {
    pub TestAllocation: ARBITER_TEST_ALLOCATION_PARAMETERS,
    pub RetestAllocation: ARBITER_RETEST_ALLOCATION_PARAMETERS,
    pub BootAllocation: ARBITER_BOOT_ALLOCATION_PARAMETERS,
    pub QueryAllocatedResources: ARBITER_QUERY_ALLOCATED_RESOURCES_PARAMETERS,
    pub QueryConflict: ARBITER_QUERY_CONFLICT_PARAMETERS,
    pub QueryArbitrate: ARBITER_QUERY_ARBITRATE_PARAMETERS,
    pub AddReserved: ARBITER_ADD_RESERVED_PARAMETERS,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for ARBITER_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ARBITER_PARTIAL: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_usb"))]
#[derive(Clone, Copy)]
pub struct ARBITER_QUERY_ALLOCATED_RESOURCES_PARAMETERS {
    pub AllocatedResources: *mut super::wdm::PCM_PARTIAL_RESOURCE_LIST,
}
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_usb"))]
impl Default for ARBITER_QUERY_ALLOCATED_RESOURCES_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct ARBITER_QUERY_ARBITRATE_PARAMETERS {
    pub ArbitrationList: super::super::Win32::winnt::PLIST_ENTRY,
}
#[cfg(feature = "Win32_winnt")]
impl Default for ARBITER_QUERY_ARBITRATE_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub struct ARBITER_QUERY_CONFLICT_PARAMETERS {
    pub PhysicalDeviceObject: super::wdm::PDEVICE_OBJECT,
    pub ConflictingResource: super::wdm::PIO_RESOURCE_DESCRIPTOR,
    pub ConflictCount: super::super::Win32::minwindef::PULONG,
    pub Conflicts: *mut PARBITER_CONFLICT_INFO,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for ARBITER_QUERY_CONFLICT_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type ARBITER_REQUEST_SOURCE = i32;
pub type ARBITER_RESULT = i32;
#[repr(C)]
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_usb", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct ARBITER_RETEST_ALLOCATION_PARAMETERS {
    pub ArbitrationList: super::super::Win32::winnt::PLIST_ENTRY,
    pub AllocateFromCount: u32,
    pub AllocateFrom: super::wdm::PCM_PARTIAL_RESOURCE_DESCRIPTOR,
}
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_usb", feature = "Win32_winnt"))]
impl Default for ARBITER_RETEST_ALLOCATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_usb", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct ARBITER_TEST_ALLOCATION_PARAMETERS {
    pub ArbitrationList: super::super::Win32::winnt::PLIST_ENTRY,
    pub AllocateFromCount: u32,
    pub AllocateFrom: super::wdm::PCM_PARTIAL_RESOURCE_DESCRIPTOR,
}
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_usb", feature = "Win32_winnt"))]
impl Default for ARBITER_TEST_ALLOCATION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ARM_PROCESSOR_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe19e3d16_bc11_11e4_9caa_c2051d5d46b0);
pub const ARM_RAS_NODE_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe3ebf4a2_df50_4708_b2d7_0b29ec2f7aa9);
pub const AgpControl: EXTENDED_AGP_REGISTER = 1;
pub const AperturePageSize: EXTENDED_AGP_REGISTER = 3;
pub const ApertureSize: EXTENDED_AGP_REGISTER = 2;
pub const ApicDestinationModeLogicalClustered: HAL_APIC_DESTINATION_MODE = 3;
pub const ApicDestinationModeLogicalFlat: HAL_APIC_DESTINATION_MODE = 2;
pub const ApicDestinationModePhysical: HAL_APIC_DESTINATION_MODE = 1;
pub const ApicDestinationModeUnknown: HAL_APIC_DESTINATION_MODE = 4;
pub const ArbiterActionAddReserved: ARBITER_ACTION = 8;
pub const ArbiterActionBootAllocation: ARBITER_ACTION = 9;
pub const ArbiterActionCommitAllocation: ARBITER_ACTION = 2;
pub const ArbiterActionQueryAllocatedResources: ARBITER_ACTION = 4;
pub const ArbiterActionQueryArbitrate: ARBITER_ACTION = 7;
pub const ArbiterActionQueryConflict: ARBITER_ACTION = 6;
pub const ArbiterActionRetestAllocation: ARBITER_ACTION = 1;
pub const ArbiterActionRollbackAllocation: ARBITER_ACTION = 3;
pub const ArbiterActionTestAllocation: ARBITER_ACTION = 0;
pub const ArbiterActionWriteReservedResources: ARBITER_ACTION = 5;
pub const ArbiterRequestHalReported: ARBITER_REQUEST_SOURCE = 1;
pub const ArbiterRequestLegacyAssigned: ARBITER_REQUEST_SOURCE = 2;
pub const ArbiterRequestLegacyReported: ARBITER_REQUEST_SOURCE = 0;
pub const ArbiterRequestPnpDetected: ARBITER_REQUEST_SOURCE = 3;
pub const ArbiterRequestPnpEnumerated: ARBITER_REQUEST_SOURCE = 4;
pub const ArbiterRequestUndefined: ARBITER_REQUEST_SOURCE = -1;
pub const ArbiterResultExternalConflict: ARBITER_RESULT = 1;
pub const ArbiterResultNullRequest: ARBITER_RESULT = 2;
pub const ArbiterResultSuccess: ARBITER_RESULT = 0;
pub const ArbiterResultUndefined: ARBITER_RESULT = -1;
pub const ArcSystem: CONFIGURATION_TYPE = 0;
pub const AudioController: CONFIGURATION_TYPE = 23;
pub type BDCB_CALLBACK_TYPE = i32;
pub type BDCB_CLASSIFICATION = i32;
pub const BDCB_IMAGEFLAGS_FAILED_CODE_INTEGRITY: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
#[derive(Clone, Copy)]
pub struct BDCB_IMAGE_INFORMATION {
    pub Classification: BDCB_CLASSIFICATION,
    pub ImageFlags: u32,
    pub ImageName: super::super::Win32::ntsecapi::UNICODE_STRING,
    pub RegistryPath: super::super::Win32::ntsecapi::UNICODE_STRING,
    pub CertificatePublisher: super::super::Win32::ntsecapi::UNICODE_STRING,
    pub CertificateIssuer: super::super::Win32::ntsecapi::UNICODE_STRING,
    pub ImageHash: *mut core::ffi::c_void,
    pub CertificateThumbprint: *mut core::ffi::c_void,
    pub ImageHashAlgorithm: u32,
    pub ThumbprintHashAlgorithm: u32,
    pub ImageHashLength: u32,
    pub CertificateThumbprintLength: u32,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
impl Default for BDCB_IMAGE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BDCB_STATUS_UPDATE_CONTEXT {
    pub StatusType: BDCB_STATUS_UPDATE_TYPE,
}
pub type BDCB_STATUS_UPDATE_TYPE = i32;
pub const BMC_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x487565ba_6494_4367_95ca_4eff893522f6);
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
pub type BOOT_DRIVER_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(callbackcontext: *const core::ffi::c_void, classification: BDCB_CALLBACK_TYPE, imageinformation: *mut BDCB_IMAGE_INFORMATION)>;
pub const BOOT_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3d61a466_ab40_409a_a698_f362d464b38f);
pub type BUS_DATA_TYPE = i32;
pub const BadPageRange: WHEA_OFFLINE_ERRS = 6;
pub const BdCbClassificationEnd: BDCB_CLASSIFICATION = 4;
pub const BdCbClassificationKnownBadImage: BDCB_CLASSIFICATION = 2;
pub const BdCbClassificationKnownBadImageBootCritical: BDCB_CLASSIFICATION = 3;
pub const BdCbClassificationKnownGoodImage: BDCB_CLASSIFICATION = 1;
pub const BdCbClassificationUnknownImage: BDCB_CLASSIFICATION = 0;
pub const BdCbInitializeImage: BDCB_CALLBACK_TYPE = 1;
pub const BdCbStatusPrepareForDependencyLoad: BDCB_STATUS_UPDATE_TYPE = 0;
pub const BdCbStatusPrepareForDriverLoad: BDCB_STATUS_UPDATE_TYPE = 1;
pub const BdCbStatusPrepareForUnload: BDCB_STATUS_UPDATE_TYPE = 2;
pub const BdCbStatusUpdate: BDCB_CALLBACK_TYPE = 0;
pub const BitErrorDdr4: PAGE_OFFLINE_ERROR_TYPES = 0;
pub const BitErrorDdr5: PAGE_OFFLINE_ERROR_TYPES = 2;
pub const BusWidth32Bits: PCI_BUS_WIDTH = 0;
pub const BusWidth64Bits: PCI_BUS_WIDTH = 1;
pub const CMCI_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x919448b2_3739_4b7f_a8f1_e0062805c2a3);
#[repr(C)]
#[cfg(all(feature = "Wdk_mce", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct CMC_DRIVER_INFO {
    pub ExceptionCallback: PDRIVER_CMC_EXCEPTION_CALLBACK,
    pub DpcCallback: super::wdm::PKDEFERRED_ROUTINE,
    pub DeviceContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_mce", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_winnt"))]
impl Default for CMC_DRIVER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CMC_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2dce8bb1_bdd7_450e_b9ad_9cf4ebd4f890);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_PCCARD_DEVICE_DATA {
    pub Flags: u8,
    pub ErrorCode: u8,
    pub Reserved: u16,
    pub BusData: u32,
    pub DeviceId: u32,
    pub LegacyBaseAddress: u32,
    pub IRQMap: [u8; 16],
}
impl Default for CM_PCCARD_DEVICE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CONFIGURATION_INFORMATION {
    pub DiskCount: u32,
    pub FloppyCount: u32,
    pub CdRomCount: u32,
    pub TapeCount: u32,
    pub ScsiPortCount: u32,
    pub SerialCount: u32,
    pub ParallelCount: u32,
    pub AtDiskPrimaryAddressClaimed: bool,
    pub AtDiskSecondaryAddressClaimed: bool,
    pub Version: u32,
    pub MediumChangerCount: u32,
}
pub type CONFIGURATION_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct CONTROLLER_OBJECT {
    pub Type: super::ntdef::CSHORT,
    pub Size: super::ntdef::CSHORT,
    pub ControllerExtension: *mut core::ffi::c_void,
    pub DeviceWaitQueue: super::wdm::KDEVICE_QUEUE,
    pub Spare1: u32,
    pub Spare2: i64,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for CONTROLLER_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CPER_EMPTY_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
#[repr(C)]
#[cfg(all(feature = "Wdk_mce", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct CPE_DRIVER_INFO {
    pub ExceptionCallback: PDRIVER_CPE_EXCEPTION_CALLBACK,
    pub DpcCallback: super::wdm::PKDEFERRED_ROUTINE,
    pub DeviceContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_mce", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_winnt"))]
impl Default for CPE_DRIVER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CPE_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4e292f96_d843_4a55_a8c2_d481f27ebeee);
pub const CP_GET_ERROR: u32 = 2;
pub const CP_GET_NODATA: u32 = 1;
pub const CP_GET_SUCCESS: u32 = 0;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct CREATE_USER_PROCESS_ECP_CONTEXT {
    pub Size: u16,
    pub Reserved: u16,
    pub AccessToken: super::super::Win32::winnt::PACCESS_TOKEN,
}
#[cfg(feature = "Win32_winnt")]
impl Default for CREATE_USER_PROCESS_ECP_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CXL_BUS_OSC_CONTROL_FIELD {
    pub u: CXL_BUS_OSC_CONTROL_FIELD_0,
}
impl Default for CXL_BUS_OSC_CONTROL_FIELD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CXL_BUS_OSC_CONTROL_FIELD_0 {
    pub Anonymous: CXL_BUS_OSC_CONTROL_FIELD_0_0,
    pub AsULONG: u32,
}
impl Default for CXL_BUS_OSC_CONTROL_FIELD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXL_BUS_OSC_CONTROL_FIELD_0_0 {
    pub _bitfield: u32,
}
pub const CXL_BUS_OSC_METHOD_CAPABILITY_REVISION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CXL_BUS_OSC_SUPPORT_FIELD {
    pub u: CXL_BUS_OSC_SUPPORT_FIELD_0,
}
impl Default for CXL_BUS_OSC_SUPPORT_FIELD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CXL_BUS_OSC_SUPPORT_FIELD_0 {
    pub Anonymous: CXL_BUS_OSC_SUPPORT_FIELD_0_0,
    pub AsULONG: u32,
}
impl Default for CXL_BUS_OSC_SUPPORT_FIELD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CXL_BUS_OSC_SUPPORT_FIELD_0_0 {
    pub _bitfield: u32,
}
pub const CXL_COMPONENT_EVENTS_SECTION_DRAM_EVENT_RECORD_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xb3cb1d60_069c_ab4e_b8af_4e9bfb5c9624);
pub const CXL_COMPONENT_EVENTS_SECTION_MEMORY_MODULE_EVENT_RECORD_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x757492fe_59dd_3943_a586_79bab113b774);
pub type CXL_DVSEC_IDS = i32;
pub type CXL_OSC_CONTROL_BITS = i32;
pub const CXL_PROTOCOL_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x80b9efb4_52b5_4de3_a777_68784b771048);
pub type CXL_PROTOCOL_VERSION = i32;
pub const CardPresent: PCI_EXPRESS_CARD_PRESENCE = 1;
pub const CbusConfiguration: BUS_DATA_TYPE = 3;
pub const CdromController: CONFIGURATION_TYPE = 15;
pub const CentralProcessor: CONFIGURATION_TYPE = 1;
pub const ClosedPage: WHEA_OFFLINE_ERRS = 5;
pub const CmResourceTypeMaximum: u32 = 8;
pub const Cmos: BUS_DATA_TYPE = 0;
pub const ConfigurationSpaceUndefined: BUS_DATA_TYPE = -1;
pub const Cxl11Support: CXL_PROTOCOL_VERSION = 1;
pub const Cxl20Support: CXL_PROTOCOL_VERSION = 2;
pub const Cxl30Support: CXL_PROTOCOL_VERSION = 3;
pub const CxlAgentType_CxlRcdRCiEP: WHEA_CXL_AGENT_TYPE = 0;
pub const CxlAgentType_CxlRchDownstreamPortRCRB: WHEA_CXL_AGENT_TYPE = 1;
pub const CxlAgentType_DownstreamSwitchPort: WHEA_CXL_AGENT_TYPE = 6;
pub const CxlAgentType_EndpointDevice: WHEA_CXL_AGENT_TYPE = 2;
pub const CxlAgentType_FabricManagerManagedLogicalDevice: WHEA_CXL_AGENT_TYPE = 4;
pub const CxlAgentType_LogicalDevice: WHEA_CXL_AGENT_TYPE = 3;
pub const CxlAgentType_Max: WHEA_CXL_AGENT_TYPE = 8;
pub const CxlAgentType_RootPort: WHEA_CXL_AGENT_TYPE = 5;
pub const CxlAgentType_UpstreamSwitchPort: WHEA_CXL_AGENT_TYPE = 7;
pub const CxlDvsec0Id: CXL_DVSEC_IDS = 0;
pub const CxlDvsecExtensions: CXL_DVSEC_IDS = 3;
pub const CxlDvsecFlexBusPort: CXL_DVSEC_IDS = 7;
pub const CxlDvsecFunctionMap: CXL_DVSEC_IDS = 2;
pub const CxlDvsecGpfDevice: CXL_DVSEC_IDS = 5;
pub const CxlDvsecGpfPort: CXL_DVSEC_IDS = 4;
pub const CxlDvsecMld: CXL_DVSEC_IDS = 9;
pub const CxlDvsecRegisterLocator: CXL_DVSEC_IDS = 8;
pub const CxlDvsecTestCapability: CXL_DVSEC_IDS = 10;
pub const CxlOscControlBitMemoryErrorReportingControl: CXL_OSC_CONTROL_BITS = 1;
pub const DBG_DEVICE_FLAG_BARS_MAPPED: u32 = 2;
pub const DBG_DEVICE_FLAG_HAL_SCRATCH_ALLOCATED: u32 = 1;
pub const DBG_DEVICE_FLAG_HOST_VISIBLE_ALLOCATED: u32 = 32;
pub const DBG_DEVICE_FLAG_SCRATCH_ALLOCATED: u32 = 4;
pub const DBG_DEVICE_FLAG_SYNTHETIC: u32 = 16;
pub const DBG_DEVICE_FLAG_UNCACHED_MEMORY: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUGGING_DEVICE_IN_USE {
    pub NameSpace: KD_NAMESPACE_ENUM,
    pub StructureLength: u32,
    pub Anonymous: DEBUGGING_DEVICE_IN_USE_0,
}
impl Default for DEBUGGING_DEVICE_IN_USE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEBUGGING_DEVICE_IN_USE_0 {
    pub AcpiDevice: ACPI_DEBUGGING_DEVICE_IN_USE,
    pub PciDevice: PCI_DEBUGGING_DEVICE_IN_USE,
}
impl Default for DEBUGGING_DEVICE_IN_USE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUGGING_DEVICE_IN_USE_INFORMATION {
    pub DeviceCount: u32,
    pub Device: [DEBUGGING_DEVICE_IN_USE; 1],
}
impl Default for DEBUGGING_DEVICE_IN_USE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub struct DEBUG_DEVICE_ADDRESS {
    pub Type: u8,
    pub Valid: bool,
    pub Anonymous: DEBUG_DEVICE_ADDRESS_0,
    pub TranslatedAddress: super::super::Win32::minwindef::PUCHAR,
    pub Length: u32,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DEBUG_DEVICE_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy)]
pub union DEBUG_DEVICE_ADDRESS_0 {
    pub Reserved: [u8; 2],
    pub Anonymous: DEBUG_DEVICE_ADDRESS_0_0,
}
#[cfg(feature = "Win32_minwindef")]
impl Default for DEBUG_DEVICE_ADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_minwindef")]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_DEVICE_ADDRESS_0_0 {
    pub BitWidth: u8,
    pub AccessSize: u8,
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_usb", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct DEBUG_DEVICE_DESCRIPTOR {
    pub Bus: u32,
    pub Slot: u32,
    pub Segment: u16,
    pub VendorID: u16,
    pub DeviceID: u16,
    pub BaseClass: u8,
    pub SubClass: u8,
    pub ProgIf: u8,
    pub Anonymous: DEBUG_DEVICE_DESCRIPTOR_0,
    pub Initialized: bool,
    pub Configured: bool,
    pub BaseAddress: [DEBUG_DEVICE_ADDRESS; 6],
    pub Memory: DEBUG_MEMORY_REQUIREMENTS,
    pub Dbg2TableIndex: u32,
    pub PortType: u16,
    pub PortSubtype: u16,
    pub OemData: *mut core::ffi::c_void,
    pub OemDataLength: u32,
    pub NameSpace: KD_NAMESPACE_ENUM,
    pub NameSpacePath: super::super::Win32::winnt::PWCHAR,
    pub NameSpacePathLength: u32,
    pub TransportType: u32,
    pub TransportData: DEBUG_TRANSPORT_DATA,
    pub EfiIoMmuData: DEBUG_EFI_IOMMU_DATA,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_usb", feature = "Win32_winnt"))]
impl Default for DEBUG_DEVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_usb", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union DEBUG_DEVICE_DESCRIPTOR_0 {
    pub Flags: u8,
    pub Anonymous: DEBUG_DEVICE_DESCRIPTOR_0_0,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_usb", feature = "Win32_winnt"))]
impl Default for DEBUG_DEVICE_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_usb", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_DEVICE_DESCRIPTOR_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_EFI_IOMMU_DATA {
    pub PciIoProtocolHandle: *mut core::ffi::c_void,
    pub Mapping: *mut core::ffi::c_void,
}
impl Default for DEBUG_EFI_IOMMU_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_usb")]
#[derive(Clone, Copy)]
pub struct DEBUG_MEMORY_REQUIREMENTS {
    pub Start: super::super::Win32::usb::PHYSICAL_ADDRESS,
    pub MaxEnd: super::super::Win32::usb::PHYSICAL_ADDRESS,
    pub VirtualAddress: *mut core::ffi::c_void,
    pub Length: u32,
    pub Cached: bool,
    pub Aligned: bool,
}
#[cfg(feature = "Win32_usb")]
impl Default for DEBUG_MEMORY_REQUIREMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_TRANSPORT_DATA {
    pub HwContextSize: u32,
    pub SharedVisibleDataSize: u32,
    pub UseSerialFraming: bool,
    pub ValidUSBCoreId: bool,
    pub USBCoreId: u8,
    pub DevControlInitialized: bool,
    pub DevControlPciLocation: DEBUG_TRANSPORT_DATA_0,
}
impl Default for DEBUG_TRANSPORT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEBUG_TRANSPORT_DATA_0 {
    pub RawPciLocation: u32,
    pub Fields: DEBUG_TRANSPORT_DATA_0_0,
}
impl Default for DEBUG_TRANSPORT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_TRANSPORT_DATA_0_0 {
    pub _bitfield: u32,
}
pub const DEFAULT_DEVICE_DRIVER_CREATOR_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57217c8d_5e66_44fb_8033_9b74cacedf5b);
pub const DEVICE_DRIVER_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0033f803_2e70_4e88_992c_6f26daf3db7a);
#[repr(C)]
#[derive(Clone, Copy)]
pub union DIMM_ADDRESS {
    pub Ddr4: DIMM_ADDRESS_0,
    pub Ddr5: DIMM_ADDRESS_1,
}
impl Default for DIMM_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DIMM_ADDRESS_0 {
    pub _bitfield: u64,
    pub Row: u32,
    pub Column: u32,
    pub Info: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DIMM_ADDRESS_1 {
    pub _bitfield: u64,
    pub Row: u32,
    pub Column: u32,
    pub Info: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union DIMM_ADDR_VALID_BITS {
    pub VB_DDR4: DIMM_ADDR_VALID_BITS_DDR4,
    pub VB_DDR5: DIMM_ADDR_VALID_BITS_DDR5,
    pub AsUINT32: u32,
}
impl Default for DIMM_ADDR_VALID_BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DIMM_ADDR_VALID_BITS_DDR4 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct DIMM_ADDR_VALID_BITS_DDR5 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DIMM_INFO {
    pub DimmAddress: DIMM_ADDRESS,
    pub ValidBits: DIMM_ADDR_VALID_BITS,
}
impl Default for DIMM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISK_SIGNATURE {
    pub PartitionStyle: u32,
    pub Anonymous: DISK_SIGNATURE_0,
}
impl Default for DISK_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISK_SIGNATURE_0 {
    pub Mbr: DISK_SIGNATURE_0_0,
    pub Gpt: DISK_SIGNATURE_0_1,
}
impl Default for DISK_SIGNATURE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DISK_SIGNATURE_0_0 {
    pub Signature: u32,
    pub CheckSum: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DISK_SIGNATURE_0_1 {
    pub DiskId: windows_sys::core::GUID,
}
pub const DOE_DISCOVERY_LENGTH: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub union DOE_DISCOVERY_REQUEST {
    pub Anonymous: DOE_DISCOVERY_REQUEST_0,
    pub AsULONG: u32,
}
impl Default for DOE_DISCOVERY_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DOE_DISCOVERY_REQUEST_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DOE_DISCOVERY_RESPONSE {
    pub Anonymous: DOE_DISCOVERY_RESPONSE_0,
    pub AsULONG: u32,
}
impl Default for DOE_DISCOVERY_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DOE_DISCOVERY_RESPONSE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DOE_HEADER_1 {
    pub Anonymous: DOE_HEADER_1_0,
    pub AsULONG: u32,
}
impl Default for DOE_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DOE_HEADER_1_0 {
    pub _bitfield1: u16,
    pub _bitfield2: u8,
    pub _bitfield3: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DOE_HEADER_2 {
    pub Anonymous: DOE_HEADER_2_0,
    pub AsULONG: u32,
}
impl Default for DOE_HEADER_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DOE_HEADER_2_0 {
    pub _bitfield: u32,
}
pub const DOE_HEADER_COUNT: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub union DOE_OBJECT_HEADER {
    pub Anonymous: DOE_OBJECT_HEADER_0,
    pub AsULONGLONG: u64,
}
impl Default for DOE_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOE_OBJECT_HEADER_0 {
    pub header1: DOE_HEADER_1,
    pub header2: DOE_HEADER_2,
}
impl Default for DOE_OBJECT_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOE_READ_DELAY: u32 = 10000;
pub const DO_BOOT_CRITICAL: u32 = 536870912;
pub const DO_DEVICE_HAS_NAME: u32 = 64;
pub const DO_DEVICE_IRP_REQUIRES_EXTENSION: u32 = 134217728;
pub const DO_DISALLOW_EXECUTE: u32 = 8388608;
pub const DO_FORCE_NEITHER_IO: u32 = 524288;
pub const DO_LONG_TERM_REQUESTS: u32 = 512;
pub const DO_LOW_PRIORITY_FILESYSTEM: u32 = 65536;
pub const DO_NEVER_LAST_DEVICE: u32 = 1024;
pub const DO_SUPPORTS_TRANSACTIONS: u32 = 262144;
pub const DO_SYSTEM_BOOT_PARTITION: u32 = 256;
pub const DO_SYSTEM_CRITICAL_PARTITION: u32 = 4194304;
pub const DO_SYSTEM_SYSTEM_PARTITION: u32 = 2097152;
pub const DO_VOLUME_DEVICE_OBJECT: u32 = 1048576;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type DRIVER_REINITIALIZE = Option<unsafe extern "system" fn(driverobject: *const super::wdm::DRIVER_OBJECT, context: *const core::ffi::c_void, count: u32)>;
pub const DRIVER_VERIFIER_FORCE_IRQL_CHECKING: u32 = 2;
pub const DRIVER_VERIFIER_INJECT_ALLOCATION_FAILURES: u32 = 4;
pub const DRIVER_VERIFIER_IO_CHECKING: u32 = 16;
pub const DRIVER_VERIFIER_SPECIAL_POOLING: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_VERIFIER_THUNK_PAIRS {
    pub PristineRoutine: PDRIVER_VERIFIER_THUNK_ROUTINE,
    pub NewRoutine: PDRIVER_VERIFIER_THUNK_ROUTINE,
}
pub const DRIVER_VERIFIER_TRACK_POOL_ALLOCATIONS: u32 = 8;
pub const DRVO_BOOTREINIT_REGISTERED: u32 = 32;
pub const DRVO_INITIALIZED: u32 = 16;
pub const DRVO_LEGACY_RESOURCES: u32 = 64;
pub const DRVO_REINIT_REGISTERED: u32 = 8;
pub const DiskController: CONFIGURATION_TYPE = 13;
pub const DiskPeripheral: CONFIGURATION_TYPE = 25;
pub const DisplayController: CONFIGURATION_TYPE = 19;
pub const DockingInformation: CONFIGURATION_TYPE = 38;
pub const DtiAdapter: CONFIGURATION_TYPE = 11;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ECP_LIST(pub u8);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct EFI_ACPI_RAS_SIGNAL_TABLE {
    pub Header: WHEA_ACPI_HEADER,
    pub NumberRecord: u32,
    pub Entries: [SIGNAL_REG_VALUE; 1],
}
impl Default for EFI_ACPI_RAS_SIGNAL_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ERROR_SOURCE_INFO {
    pub ErrorCount: u32,
    pub ErrorSourceId: u32,
}
pub const ERRTYP_BUS: u32 = 16;
pub const ERRTYP_CACHE: u32 = 6;
pub const ERRTYP_FLOW: u32 = 9;
pub const ERRTYP_FUNCTION: u32 = 7;
pub const ERRTYP_IMPROPER: u32 = 18;
pub const ERRTYP_INTERNAL: u32 = 1;
pub const ERRTYP_LOSSOFLOCKSTEP: u32 = 20;
pub const ERRTYP_MAP: u32 = 17;
pub const ERRTYP_MEM: u32 = 4;
pub const ERRTYP_PARITY: u32 = 22;
pub const ERRTYP_PATHERROR: u32 = 24;
pub const ERRTYP_POISONED: u32 = 26;
pub const ERRTYP_PROTOCOL: u32 = 23;
pub const ERRTYP_RESPONSE: u32 = 21;
pub const ERRTYP_SELFTEST: u32 = 8;
pub const ERRTYP_TIMEOUT: u32 = 25;
pub const ERRTYP_TLB: u32 = 5;
pub const ERRTYP_UNIMPL: u32 = 19;
pub type EXPAND_STACK_CALLOUT = Option<unsafe extern "system" fn(parameter: *const core::ffi::c_void)>;
pub type EXTENDED_AGP_REGISTER = i32;
pub const EXTINT_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfe84086e_b557_43cf_ac1b_17982e078470);
pub const EisaAdapter: CONFIGURATION_TYPE = 8;
pub const EisaConfiguration: BUS_DATA_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_ALIGNMENT_INFORMATION {
    pub AlignmentRequirement: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_ATTRIBUTE_TAG_INFORMATION {
    pub FileAttributes: u32,
    pub ReparseTag: u32,
}
pub const FILE_CHARACTERISTICS_EXPECT_ORDERLY_REMOVAL_DEPRECATED: u32 = 512;
pub const FILE_CHARACTERISTICS_EXPECT_ORDERLY_REMOVAL_EX: u32 = 16384;
pub const FILE_CHARACTERISTICS_EXPECT_SURPRISE_REMOVAL_DEPRECATED: u32 = 768;
pub const FILE_CHARACTERISTICS_EXPECT_SURPRISE_REMOVAL_EX: u32 = 32768;
pub const FILE_CHARACTERISTICS_PROPAGATED: u32 = 327951;
pub const FILE_CHARACTERISTICS_REMOVAL_POLICY_MASK_DEPRECATED: u32 = 768;
pub const FILE_CHARACTERISTICS_REMOVAL_POLICY_MASK_EX: u32 = 49152;
pub const FILE_DISPOSITION_DELETE: u32 = 1;
pub const FILE_DISPOSITION_DO_NOT_DELETE: u32 = 0;
pub const FILE_DISPOSITION_FORCE_IMAGE_SECTION_CHECK: u32 = 4;
pub const FILE_DISPOSITION_IGNORE_READONLY_ATTRIBUTE: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_DISPOSITION_INFORMATION {
    pub DeleteFile: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_DISPOSITION_INFORMATION_EX {
    pub Flags: u32,
}
pub const FILE_DISPOSITION_ON_CLOSE: u32 = 8;
pub const FILE_DISPOSITION_POSIX_SEMANTICS: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_END_OF_FILE_INFORMATION {
    pub EndOfFile: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_FS_FULL_SIZE_INFORMATION {
    pub TotalAllocationUnits: i64,
    pub CallerAvailableAllocationUnits: i64,
    pub ActualAvailableAllocationUnits: i64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_FS_FULL_SIZE_INFORMATION_EX {
    pub ActualTotalAllocationUnits: u64,
    pub ActualAvailableAllocationUnits: u64,
    pub ActualPoolUnavailableAllocationUnits: u64,
    pub CallerTotalAllocationUnits: u64,
    pub CallerAvailableAllocationUnits: u64,
    pub CallerPoolUnavailableAllocationUnits: u64,
    pub UsedAllocationUnits: u64,
    pub TotalReservedAllocationUnits: u64,
    pub VolumeStorageReserveAllocationUnits: u64,
    pub AvailableCommittedAllocationUnits: u64,
    pub PoolAvailableAllocationUnits: u64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_FS_LABEL_INFORMATION {
    pub VolumeLabelLength: u32,
    pub VolumeLabel: [u16; 1],
}
impl Default for FILE_FS_LABEL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_FS_METADATA_SIZE_INFORMATION {
    pub TotalMetadataAllocationUnits: i64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_FS_OBJECTID_INFORMATION {
    pub ObjectId: [u8; 16],
    pub ExtendedInfo: [u8; 48],
}
impl Default for FILE_FS_OBJECTID_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_FS_SECTOR_SIZE_INFORMATION {
    pub LogicalBytesPerSector: u32,
    pub PhysicalBytesPerSectorForAtomicity: u32,
    pub PhysicalBytesPerSectorForPerformance: u32,
    pub FileSystemEffectivePhysicalBytesPerSectorForAtomicity: u32,
    pub Flags: u32,
    pub ByteOffsetForSectorAlignment: u32,
    pub ByteOffsetForPartitionAlignment: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_FS_SIZE_INFORMATION {
    pub TotalAllocationUnits: i64,
    pub AvailableAllocationUnits: i64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_FS_VOLUME_INFORMATION {
    pub VolumeCreationTime: i64,
    pub VolumeSerialNumber: u32,
    pub VolumeLabelLength: u32,
    pub SupportsObjects: bool,
    pub VolumeLabel: [u16; 1],
}
impl Default for FILE_FS_VOLUME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_NAME_INFORMATION {
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl Default for FILE_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_VALID_DATA_LENGTH_INFORMATION {
    pub ValidDataLength: i64,
}
pub const FIRMWARE_ERROR_RECORD_REFERENCE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x81212a96_09ed_4996_9471_8d729c8e69ed);
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_16GB: u32 = 6;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_1GB: u32 = 2;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_1MB: u32 = 9;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_256MB: u32 = 0;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_2GB: u32 = 3;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_32GB: u32 = 7;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_4GB: u32 = 4;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_512MB: u32 = 1;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_8GB: u32 = 5;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_16MB: u32 = 4;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_1MB: u32 = 0;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_2MB: u32 = 1;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_4MB: u32 = 2;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_8MB: u32 = 3;
pub const FPB_MEM_VECTOR_GRANULARITY_1B: u32 = 8;
pub const FPB_RID_VECTOR_GRANULARITY_256RIDS: u32 = 5;
pub const FPB_RID_VECTOR_GRANULARITY_64RIDS: u32 = 3;
pub const FPB_RID_VECTOR_GRANULARITY_8RIDS: u32 = 0;
pub const FPB_VECTOR_SELECT_MEM_HIGH: u32 = 2;
pub const FPB_VECTOR_SELECT_MEM_LOW: u32 = 1;
pub const FPB_VECTOR_SELECT_RID: u32 = 0;
pub const FPB_VECTOR_SIZE_SUPPORTED_1KBITS: u32 = 2;
pub const FPB_VECTOR_SIZE_SUPPORTED_256BITS: u32 = 0;
pub const FPB_VECTOR_SIZE_SUPPORTED_2KBITS: u32 = 3;
pub const FPB_VECTOR_SIZE_SUPPORTED_4KBITS: u32 = 4;
pub const FPB_VECTOR_SIZE_SUPPORTED_512BITS: u32 = 1;
pub const FPB_VECTOR_SIZE_SUPPORTED_8KBITS: u32 = 5;
pub const FloatingPointProcessor: CONFIGURATION_TYPE = 2;
pub const FloppyDiskPeripheral: CONFIGURATION_TYPE = 26;
pub const GENERIC_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3e62a467_ab40_409a_a698_f362d464b38f);
pub const GENERIC_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe71254e8_c1b9_4940_ab76_909703a4320f);
pub const GENPROC_FLAGS_CORRECTED: u32 = 8;
pub const GENPROC_FLAGS_OVERFLOW: u32 = 4;
pub const GENPROC_FLAGS_PRECISEIP: u32 = 2;
pub const GENPROC_FLAGS_RESTARTABLE: u32 = 1;
pub const GENPROC_OP_DATAREAD: u32 = 1;
pub const GENPROC_OP_DATAWRITE: u32 = 2;
pub const GENPROC_OP_GENERIC: u32 = 0;
pub const GENPROC_OP_INSTRUCTIONEXE: u32 = 3;
pub const GENPROC_PROCERRTYPE_BUS: u32 = 4;
pub const GENPROC_PROCERRTYPE_CACHE: u32 = 1;
pub const GENPROC_PROCERRTYPE_MAE: u32 = 8;
pub const GENPROC_PROCERRTYPE_TLB: u32 = 2;
pub const GENPROC_PROCERRTYPE_UNKNOWN: u32 = 0;
pub const GENPROC_PROCISA_ARM32: u32 = 4;
pub const GENPROC_PROCISA_ARM64: u32 = 8;
pub const GENPROC_PROCISA_IPF: u32 = 1;
pub const GENPROC_PROCISA_X64: u32 = 2;
pub const GENPROC_PROCISA_X86: u32 = 0;
pub const GENPROC_PROCTYPE_ARM: u32 = 2;
pub const GENPROC_PROCTYPE_IPF: u32 = 1;
pub const GENPROC_PROCTYPE_XPF: u32 = 0;
pub const GUID_ECP_CREATE_USER_PROCESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe0e429ff_6ddc_4e65_aab6_45d05a038a08);
pub const GartHigh: EXTENDED_AGP_REGISTER = 5;
pub const GartLow: EXTENDED_AGP_REGISTER = 4;
pub const GenericEqual: RTL_GENERIC_COMPARE_RESULTS = 2;
pub const GenericGreaterThan: RTL_GENERIC_COMPARE_RESULTS = 1;
pub const GenericLessThan: RTL_GENERIC_COMPARE_RESULTS = 0;
pub const GetMemoryDetailsErr: WHEA_OFFLINE_ERRS = 1;
#[repr(C)]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy, Default)]
pub struct HAL_AMLI_BAD_IO_ADDRESS_LIST {
    pub BadAddrBegin: u32,
    pub BadAddrSize: u32,
    pub OSVersionTrigger: u32,
    pub IOHandler: PHALIOREADWRITEHANDLER,
}
pub type HAL_APIC_DESTINATION_MODE = i32;
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy, Default)]
pub struct HAL_BUS_INFORMATION {
    pub BusType: super::wdm::INTERFACE_TYPE,
    pub ConfigurationType: BUS_DATA_TYPE,
    pub BusNumber: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct HAL_CALLBACKS {
    pub SetSystemInformation: super::wdm::PCALLBACK_OBJECT,
    pub BusCheck: super::wdm::PCALLBACK_OBJECT,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for HAL_CALLBACKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HAL_CMC_DISABLED: u32 = 0;
pub const HAL_CMC_INTERRUPTS_BASED: i32 = -1;
pub const HAL_CPE_DISABLED: u32 = 0;
pub const HAL_CPE_INTERRUPTS_BASED: i32 = -1;
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy, Default)]
pub struct HAL_DISPATCH {
    pub Version: u32,
    pub HalQuerySystemInformation: pHalQuerySystemInformation,
    pub HalSetSystemInformation: pHalSetSystemInformation,
    pub HalQueryBusSlots: pHalQueryBusSlots,
    pub Spare1: u32,
    pub HalExamineMBR: pHalExamineMBR,
    pub HalIoReadPartitionTable: pHalIoReadPartitionTable,
    pub HalIoSetPartitionInformation: pHalIoSetPartitionInformation,
    pub HalIoWritePartitionTable: pHalIoWritePartitionTable,
    pub HalReferenceHandlerForBus: pHalHandlerForBus,
    pub HalReferenceBusHandler: pHalReferenceBusHandler,
    pub HalDereferenceBusHandler: pHalReferenceBusHandler,
    pub HalInitPnpDriver: pHalInitPnpDriver,
    pub HalInitPowerManagement: pHalInitPowerManagement,
    pub HalGetDmaAdapter: pHalGetDmaAdapter,
    pub HalGetInterruptTranslator: pHalGetInterruptTranslator,
    pub HalStartMirroring: pHalStartMirroring,
    pub HalEndMirroring: pHalEndMirroring,
    pub HalMirrorPhysicalMemory: pHalMirrorPhysicalMemory,
    pub HalEndOfBoot: pHalEndOfBoot,
    pub HalMirrorVerify: pHalMirrorVerify,
    pub HalGetCachedAcpiTable: pHalGetAcpiTable,
    pub HalSetPciErrorHandlerCallback: pHalSetPciErrorHandlerCallback,
    pub HalGetPrmCache: pHalGetPrmCache,
    pub HalInvokePrmFwHandler: pHalInvokePrmFwHandler,
    pub HalFfaMsgSendDirectReq2: pHalFfaMsgSendDirectReq2,
    pub HalFfaRunTarget: pHalFfaRunTarget,
    pub HalFfaRegisterNotification: pHalFfaRegisterNotification,
    pub HalFfaUnregisterNotification: pHalFfaUnregisterNotification,
}
pub const HAL_DISPATCH_VERSION: u32 = 7;
pub type HAL_DISPLAY_BIOS_INFORMATION = i32;
pub type HAL_DMA_CRASH_DUMP_REGISTER_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HAL_ERROR_INFO {
    pub Version: u32,
    pub InitMaxSize: u32,
    pub McaMaxSize: u32,
    pub McaPreviousEventsCount: u32,
    pub McaCorrectedEventsCount: u32,
    pub McaKernelDeliveryFails: u32,
    pub McaDriverDpcQueueFails: u32,
    pub McaReserved: u32,
    pub CmcMaxSize: u32,
    pub CmcPollingInterval: u32,
    pub CmcInterruptsCount: u32,
    pub CmcKernelDeliveryFails: u32,
    pub CmcDriverDpcQueueFails: u32,
    pub CmcGetStateFails: u32,
    pub CmcClearStateFails: u32,
    pub CmcReserved: u32,
    pub CmcLogId: u64,
    pub CpeMaxSize: u32,
    pub CpePollingInterval: u32,
    pub CpeInterruptsCount: u32,
    pub CpeKernelDeliveryFails: u32,
    pub CpeDriverDpcQueueFails: u32,
    pub CpeGetStateFails: u32,
    pub CpeClearStateFails: u32,
    pub CpeInterruptSources: u32,
    pub CpeLogId: u64,
    pub KernelReserved: [u64; 4],
}
impl Default for HAL_ERROR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HAL_MCA_DISABLED: u32 = 0;
#[repr(C)]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy, Default)]
pub struct HAL_MCA_INTERFACE {
    pub Lock: PHALMCAINTERFACELOCK,
    pub Unlock: PHALMCAINTERFACEUNLOCK,
    pub ReadRegister: PHALMCAINTERFACEREADREGISTER,
}
pub const HAL_MCA_INTERRUPTS_BASED: i32 = -1;
pub const HAL_MCE_DISABLED: u32 = 0;
pub const HAL_MCE_INTERRUPTS_BASED: u32 = 4294967295;
pub const HAL_PLATFORM_ACPI_TABLES_CACHED: u32 = 32;
pub const HAL_PLATFORM_DISABLE_PTCG: u32 = 4;
pub const HAL_PLATFORM_DISABLE_UC_MAIN_MEMORY: u32 = 8;
pub const HAL_PLATFORM_DISABLE_WRITE_COMBINING: u32 = 1;
pub const HAL_PLATFORM_ENABLE_WRITE_COMBINING_MMIO: u32 = 16;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HAL_PLATFORM_INFORMATION {
    pub PlatformFlags: u32,
}
pub const HAL_PMU_NOTIFCATION_CALLBACK_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HAL_PMU_NOTIFICATION {
    pub Version: u16,
    pub Size: u16,
    pub NotificationType: HAL_PMU_NOTIFICATION_TYPE,
}
pub type HAL_PMU_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, notification: *const HAL_PMU_NOTIFICATION)>;
pub type HAL_PMU_NOTIFICATION_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HAL_POST_SLEEP_STATS {
    pub PostSleepTimeMs: u32,
    pub PostSleepMPTimeMs: u32,
    pub InterruptReinitTimeMs: u32,
    pub IommuReinitTimeMs: u32,
    pub WakeProcessorsTimeMs: u32,
    pub UpdateMicrocodeTimeMs: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HAL_POWER_INFORMATION {
    pub TBD: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HAL_PROCESSOR_FEATURE {
    pub UsableFeatureBits: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HAL_PROCESSOR_SPEED_INFORMATION {
    pub ProcessorSpeed: u32,
}
pub type HAL_QUERY_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HAL_REGISTER_PMU_NOTIFICATION_INPUT {
    pub Version: u16,
    pub Size: u16,
    pub OwnerTag: u32,
    pub CallbackRoutine: PHAL_PMU_NOTIFICATION_CALLBACK,
    pub CallbackContext: *mut core::ffi::c_void,
}
impl Default for HAL_REGISTER_PMU_NOTIFICATION_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const HAL_REGISTER_PMU_NOTIFICATION_INPUT_VERSION: u32 = 1;
pub type HAL_SET_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct HAL_UNREGISTER_PMU_NOTIFICATION_INPUT {
    pub Version: u16,
    pub Size: u16,
    pub OwnerTag: u32,
}
pub const HAL_UNREGISTER_PMU_NOTIFICATION_INPUT_VERSION: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct HARDWARE_COUNTER {
    pub Type: super::super::Win32::winnt::HARDWARE_COUNTER_TYPE,
    pub Reserved: u32,
    pub Index: u64,
}
#[cfg(feature = "Win32_bcrypt")]
pub type HVL_WHEA_ERROR_NOTIFICATION = Option<unsafe extern "system" fn(recoverycontext: *const WHEA_RECOVERY_CONTEXT, poisoned: bool) -> super::super::Win32::bcrypt::NTSTATUS>;
pub const HalAcpiAuditInformation: HAL_QUERY_INFORMATION_CLASS = 26;
pub const HalAmuInformation: HAL_QUERY_INFORMATION_CLASS = 55;
pub const HalCallbackInformation: HAL_QUERY_INFORMATION_CLASS = 5;
pub const HalChannelTopologyInformation: HAL_QUERY_INFORMATION_CLASS = 31;
pub const HalCmcLog: HAL_SET_INFORMATION_CLASS = 7;
pub const HalCmcLogInformation: HAL_QUERY_INFORMATION_CLASS = 13;
pub const HalCmcRegisterDriver: HAL_SET_INFORMATION_CLASS = 4;
pub const HalCpeLog: HAL_SET_INFORMATION_CLASS = 8;
pub const HalCpeLogInformation: HAL_QUERY_INFORMATION_CLASS = 14;
pub const HalCpeRegisterDriver: HAL_SET_INFORMATION_CLASS = 5;
pub const HalDisplayBiosInformation: HAL_QUERY_INFORMATION_CLASS = 9;
pub const HalDisplayEmulatedBios: HAL_DISPLAY_BIOS_INFORMATION = 1;
pub const HalDisplayInt10Bios: HAL_DISPLAY_BIOS_INFORMATION = 0;
pub const HalDisplayNoBios: HAL_DISPLAY_BIOS_INFORMATION = 2;
pub const HalDmaCrashDumpRegisterSet1: HAL_DMA_CRASH_DUMP_REGISTER_TYPE = 0;
pub const HalDmaCrashDumpRegisterSet2: HAL_DMA_CRASH_DUMP_REGISTER_TYPE = 1;
pub const HalDmaCrashDumpRegisterSetMax: HAL_DMA_CRASH_DUMP_REGISTER_TYPE = 2;
pub const HalDmaRemappingInformation: HAL_QUERY_INFORMATION_CLASS = 48;
pub const HalEnlightenment: HAL_SET_INFORMATION_CLASS = 11;
pub const HalErrorInformation: HAL_QUERY_INFORMATION_CLASS = 12;
pub const HalExternalCacheInformation: HAL_QUERY_INFORMATION_CLASS = 32;
pub const HalFrameBufferCachingInformation: HAL_QUERY_INFORMATION_CLASS = 8;
pub const HalFrequencyInformation: HAL_QUERY_INFORMATION_CLASS = 22;
pub const HalFwBootPerformanceInformation: HAL_QUERY_INFORMATION_CLASS = 34;
pub const HalFwS3PerformanceInformation: HAL_QUERY_INFORMATION_CLASS = 35;
pub const HalGenerateCmcInterrupt: HAL_SET_INFORMATION_CLASS = 9;
pub const HalGetChannelPowerInformation: HAL_QUERY_INFORMATION_CLASS = 36;
pub const HalGicInformation: HAL_QUERY_INFORMATION_CLASS = 59;
pub const HalHardwareWatchdogInformation: HAL_QUERY_INFORMATION_CLASS = 47;
pub const HalHeterogeneousMemoryAttributesInterface: HAL_QUERY_INFORMATION_CLASS = 50;
pub const HalHypervisorInformation: HAL_QUERY_INFORMATION_CLASS = 24;
pub const HalI386ExceptionChainTerminatorInformation: HAL_SET_INFORMATION_CLASS = 15;
pub const HalInformationClassUnused1: HAL_QUERY_INFORMATION_CLASS = 2;
pub const HalInitLogInformation: HAL_QUERY_INFORMATION_CLASS = 21;
pub const HalInstalledBusInformation: HAL_QUERY_INFORMATION_CLASS = 0;
pub const HalInterruptControllerInformation: HAL_QUERY_INFORMATION_CLASS = 39;
pub const HalIrtInformation: HAL_QUERY_INFORMATION_CLASS = 27;
pub const HalKernelErrorHandler: HAL_SET_INFORMATION_CLASS = 3;
pub const HalMapRegisterInformation: HAL_QUERY_INFORMATION_CLASS = 6;
pub const HalMcaLog: HAL_SET_INFORMATION_CLASS = 6;
pub const HalMcaLogInformation: HAL_QUERY_INFORMATION_CLASS = 7;
pub const HalMcaRegisterDriver: HAL_SET_INFORMATION_CLASS = 2;
pub const HalNumaRangeTableInformation: HAL_QUERY_INFORMATION_CLASS = 30;
pub const HalNumaTopologyInterface: HAL_QUERY_INFORMATION_CLASS = 11;
pub const HalParkingPageInformation: HAL_QUERY_INFORMATION_CLASS = 29;
pub const HalPartitionIpiInterface: HAL_QUERY_INFORMATION_CLASS = 18;
pub const HalPlatformInformation: HAL_QUERY_INFORMATION_CLASS = 19;
pub const HalPlatformTimerInformation: HAL_QUERY_INFORMATION_CLASS = 25;
pub const HalPmuAvailableNotification: HAL_PMU_NOTIFICATION_TYPE = 0;
pub const HalPmuReleaseNotification: HAL_PMU_NOTIFICATION_TYPE = 1;
pub const HalPostSleepInformation: HAL_QUERY_INFORMATION_CLASS = 60;
pub const HalPowerInformation: HAL_QUERY_INFORMATION_CLASS = 3;
pub const HalProcessorBrandString: HAL_QUERY_INFORMATION_CLASS = 23;
pub const HalProcessorFeatureInformation: HAL_QUERY_INFORMATION_CLASS = 10;
pub const HalProcessorSpeedInformation: HAL_QUERY_INFORMATION_CLASS = 4;
pub const HalProfileDpgoSourceInterruptHandler: HAL_SET_INFORMATION_CLASS = 12;
pub const HalProfileSourceAdd: HAL_SET_INFORMATION_CLASS = 20;
pub const HalProfileSourceInformation: HAL_QUERY_INFORMATION_CLASS = 1;
pub const HalProfileSourceInterruptHandler: HAL_SET_INFORMATION_CLASS = 1;
pub const HalProfileSourceInterval: HAL_SET_INFORMATION_CLASS = 0;
pub const HalProfileSourceRemove: HAL_SET_INFORMATION_CLASS = 21;
pub const HalProfileSourceTimerHandler: HAL_SET_INFORMATION_CLASS = 10;
pub const HalPsciInformation: HAL_QUERY_INFORMATION_CLASS = 38;
pub const HalQueryAMLIIllegalIOPortAddresses: HAL_QUERY_INFORMATION_CLASS = 16;
pub const HalQueryAcpiWakeAlarmSystemPowerStateInformation: HAL_QUERY_INFORMATION_CLASS = 43;
pub const HalQueryArm64PlatformInformation: HAL_QUERY_INFORMATION_CLASS = 57;
pub const HalQueryArmErrataInformation: HAL_QUERY_INFORMATION_CLASS = 41;
pub const HalQueryDebuggerInformation: HAL_QUERY_INFORMATION_CLASS = 33;
pub const HalQueryHibernateResumePc: HAL_QUERY_INFORMATION_CLASS = 56;
pub const HalQueryHyperlaunchEntrypoint: HAL_QUERY_INFORMATION_CLASS = 46;
pub const HalQueryIommuReservedRegionInformation: HAL_QUERY_INFORMATION_CLASS = 40;
pub const HalQueryMaxHotPlugMemoryAddress: HAL_QUERY_INFORMATION_CLASS = 17;
pub const HalQueryMcaInterface: HAL_QUERY_INFORMATION_CLASS = 15;
pub const HalQueryMpamInformation: HAL_QUERY_INFORMATION_CLASS = 54;
pub const HalQueryPerDeviceMsiLimitInformation: HAL_QUERY_INFORMATION_CLASS = 51;
pub const HalQueryPnpBusDriverInformation: HAL_QUERY_INFORMATION_CLASS = 58;
pub const HalQueryProcessorEfficiencyInformation: HAL_QUERY_INFORMATION_CLASS = 42;
pub const HalQueryProfileCorruptionStatus: HAL_QUERY_INFORMATION_CLASS = 52;
pub const HalQueryProfileCounterOwnership: HAL_QUERY_INFORMATION_CLASS = 53;
pub const HalQueryProfileIsTimerBasedProfiling: HAL_QUERY_INFORMATION_CLASS = 44;
pub const HalQueryProfileNumberOfCounters: HAL_QUERY_INFORMATION_CLASS = 45;
pub const HalQueryProfileSourceList: HAL_QUERY_INFORMATION_CLASS = 20;
pub const HalQueryStateElementInformation: HAL_QUERY_INFORMATION_CLASS = 37;
pub const HalQueryUnused0001: HAL_QUERY_INFORMATION_CLASS = 49;
pub const HalRegisterPmuNotification: HAL_SET_INFORMATION_CLASS = 24;
pub const HalRegisterSecondaryInterruptInterface: HAL_SET_INFORMATION_CLASS = 13;
pub const HalSecondaryInterruptInformation: HAL_QUERY_INFORMATION_CLASS = 28;
pub const HalSetChannelPowerInformation: HAL_SET_INFORMATION_CLASS = 14;
pub const HalSetClockTimerMinimumInterval: HAL_SET_INFORMATION_CLASS = 23;
pub const HalSetHvciEnabled: HAL_SET_INFORMATION_CLASS = 18;
pub const HalSetProcessorTraceInterruptHandler: HAL_SET_INFORMATION_CLASS = 19;
pub const HalSetPsciSuspendMode: HAL_SET_INFORMATION_CLASS = 17;
pub const HalSetResetParkDisposition: HAL_SET_INFORMATION_CLASS = 16;
pub const HalSetSwInterruptHandler: HAL_SET_INFORMATION_CLASS = 22;
pub const HalUnregisterPmuNotification: HAL_SET_INFORMATION_CLASS = 25;
pub const IMAGE_ADDRESSING_MODE_32BIT: u32 = 3;
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_filter", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub struct IMAGE_INFO_EX {
    pub Size: usize,
    pub ImageInfo: super::super::Win32::filter::IMAGE_INFO,
    pub FileObject: *mut super::wdm::FILE_OBJECT,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_filter", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for IMAGE_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INIT_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcc5263e8_9308_454a_89d0_340bd39bc98e);
pub const INJECT_ERRTYPE_MEMORY_CORRECTABLE: u32 = 8;
pub const INJECT_ERRTYPE_MEMORY_UNCORRECTABLEFATAL: u32 = 32;
pub const INJECT_ERRTYPE_MEMORY_UNCORRECTABLENONFATAL: u32 = 16;
pub const INJECT_ERRTYPE_PCIEXPRESS_CORRECTABLE: u32 = 64;
pub const INJECT_ERRTYPE_PCIEXPRESS_UNCORRECTABLEFATAL: u32 = 256;
pub const INJECT_ERRTYPE_PCIEXPRESS_UNCORRECTABLENONFATAL: u32 = 128;
pub const INJECT_ERRTYPE_PLATFORM_CORRECTABLE: u32 = 512;
pub const INJECT_ERRTYPE_PLATFORM_UNCORRECTABLEFATAL: u32 = 2048;
pub const INJECT_ERRTYPE_PLATFORM_UNCORRECTABLENONFATAL: u32 = 1024;
pub const INJECT_ERRTYPE_PLATFORM_VENDOR_DEFINED: u32 = 2147483648;
pub const INJECT_ERRTYPE_PROCESSOR_CORRECTABLE: u32 = 1;
pub const INJECT_ERRTYPE_PROCESSOR_UNCORRECTABLEFATAL: u32 = 4;
pub const INJECT_ERRTYPE_PROCESSOR_UNCORRECTABLENONFATAL: u32 = 2;
pub const INTEL_ADDRESS_TRANSLATION_PRM_HANDLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1de4055d_d2f3_4e11_b7d9_7d6c19173fee);
pub type INTERLOCKED_RESULT = i32;
pub const IOCTL_IPMI_INTERNAL_RECORD_SEL_EVENT: u32 = 2232320;
pub const IO_ATTACH_DEVICE: u32 = 1024;
pub const IO_CHECK_CREATE_PARAMETERS: u32 = 512;
#[repr(C)]
#[cfg(feature = "Wdk_ntdef")]
#[derive(Clone, Copy)]
pub struct IO_DRIVER_CREATE_CONTEXT {
    pub Size: super::ntdef::CSHORT,
    pub ExtraCreateParameter: *mut ECP_LIST,
    pub DeviceObjectHint: *mut core::ffi::c_void,
    pub TxnParameters: PTXN_PARAMETER_BLOCK,
    pub SiloContext: PESILO,
}
#[cfg(feature = "Wdk_ntdef")]
impl Default for IO_DRIVER_CREATE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub struct IO_FOEXT_SHADOW_FILE {
    pub BackingFileObject: super::wdm::PFILE_OBJECT,
    pub BackingFltInstance: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for IO_FOEXT_SHADOW_FILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_FOEXT_SILO_PARAMETERS {
    pub Length: u32,
    pub Anonymous: IO_FOEXT_SILO_PARAMETERS_0,
    pub SiloContext: PESILO,
}
impl Default for IO_FOEXT_SILO_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IO_FOEXT_SILO_PARAMETERS_0 {
    pub Anonymous: IO_FOEXT_SILO_PARAMETERS_0_0,
    pub Flags: u32,
}
impl Default for IO_FOEXT_SILO_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct IO_FOEXT_SILO_PARAMETERS_0_0 {
    pub _bitfield: u32,
}
pub const IO_IGNORE_SHARE_ACCESS_CHECK: u32 = 2048;
pub type IO_QUERY_DEVICE_DATA_FORMAT = i32;
pub const IO_USE_AMBIENT_SILO: PESILO = 1 as _;
pub const IPF_PROCESSOR_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe429faf1_3cb7_11d4_bca7_0080c73c8881);
pub const IPF_SAL_RECORD_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f3380d1_6eb0_497f_a578_4d4c65a71617);
pub const IPMI_IOCTL_INDEX: u32 = 1024;
pub const IPMI_MSR_DUMP_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1c15b445_9b06_4667_ac25_33c056b88803);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct IPMI_OS_SEL_RECORD {
    pub Signature: u32,
    pub Version: u32,
    pub Length: u32,
    pub RecordType: IPMI_OS_SEL_RECORD_TYPE,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
impl Default for IPMI_OS_SEL_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IPMI_OS_SEL_RECORD_MASK: u32 = 65535;
pub const IPMI_OS_SEL_RECORD_SIGNATURE: u32 = 1381192527;
pub type IPMI_OS_SEL_RECORD_TYPE = i32;
pub const IPMI_OS_SEL_RECORD_VERSION: u32 = 1;
pub const IPMI_OS_SEL_RECORD_VERSION_1: u32 = 1;
pub const IRP_MN_COMPLETE: u32 = 4;
pub const IRP_MN_COMPLETE_MDL: u32 = 6;
pub const IRP_MN_COMPLETE_MDL_DPC: u32 = 7;
pub const IRP_MN_COMPRESSED: u32 = 8;
pub const IRP_MN_DPC: u32 = 1;
pub const IRP_MN_FLUSH_AND_PURGE: u32 = 1;
pub const IRP_MN_FLUSH_DATA_ONLY: u32 = 2;
pub const IRP_MN_FLUSH_DATA_SYNC_ONLY: u32 = 4;
pub const IRP_MN_FLUSH_NO_SYNC: u32 = 3;
pub const IRP_MN_KERNEL_CALL: u32 = 4;
pub const IRP_MN_LOAD_FILE_SYSTEM: u32 = 3;
pub const IRP_MN_LOCK: u32 = 1;
pub const IRP_MN_MDL: u32 = 2;
pub const IRP_MN_MDL_DPC: u32 = 3;
pub const IRP_MN_MOUNT_VOLUME: u32 = 1;
pub const IRP_MN_NORMAL: u32 = 0;
pub const IRP_MN_NOTIFY_CHANGE_DIRECTORY: u32 = 2;
pub const IRP_MN_NOTIFY_CHANGE_DIRECTORY_EX: u32 = 3;
pub const IRP_MN_QUERY_DIRECTORY: u32 = 1;
pub const IRP_MN_QUERY_LEGACY_BUS_INFORMATION: u32 = 24;
pub const IRP_MN_TRACK_LINK: u32 = 4;
pub const IRP_MN_UNLOCK_ALL: u32 = 3;
pub const IRP_MN_UNLOCK_ALL_BY_KEY: u32 = 4;
pub const IRP_MN_UNLOCK_SINGLE: u32 = 2;
pub const IRP_MN_USER_FS_REQUEST: u32 = 0;
pub const IRP_MN_VERIFY_VOLUME: u32 = 2;
pub const IncorrectDdrVersion: WHEA_OFFLINE_ERRS = 10;
pub const IndicatorBlink: PCI_EXPRESS_INDICATOR_STATE = 2;
pub const IndicatorOff: PCI_EXPRESS_INDICATOR_STATE = 3;
pub const IndicatorOn: PCI_EXPRESS_INDICATOR_STATE = 1;
pub const InvalidData: WHEA_OFFLINE_ERRS = 7;
pub const IoQueryDeviceComponentInformation: IO_QUERY_DEVICE_DATA_FORMAT = 2;
pub const IoQueryDeviceConfigurationData: IO_QUERY_DEVICE_DATA_FORMAT = 1;
pub const IoQueryDeviceIdentifier: IO_QUERY_DEVICE_DATA_FORMAT = 0;
pub const IoQueryDeviceMaxData: IO_QUERY_DEVICE_DATA_FORMAT = 3;
pub const IpmiOsSelRecordTypeBugcheckData: IPMI_OS_SEL_RECORD_TYPE = 9;
pub const IpmiOsSelRecordTypeBugcheckRecovery: IPMI_OS_SEL_RECORD_TYPE = 8;
pub const IpmiOsSelRecordTypeDriver: IPMI_OS_SEL_RECORD_TYPE = 7;
pub const IpmiOsSelRecordTypeMax: IPMI_OS_SEL_RECORD_TYPE = 10;
pub const IpmiOsSelRecordTypeOther: IPMI_OS_SEL_RECORD_TYPE = 1;
pub const IpmiOsSelRecordTypeRaw: IPMI_OS_SEL_RECORD_TYPE = 6;
pub const IpmiOsSelRecordTypeWhea: IPMI_OS_SEL_RECORD_TYPE = 0;
pub const IpmiOsSelRecordTypeWheaErrorNmi: IPMI_OS_SEL_RECORD_TYPE = 4;
pub const IpmiOsSelRecordTypeWheaErrorOther: IPMI_OS_SEL_RECORD_TYPE = 5;
pub const IpmiOsSelRecordTypeWheaErrorPci: IPMI_OS_SEL_RECORD_TYPE = 3;
pub const IpmiOsSelRecordTypeWheaErrorXpfMca: IPMI_OS_SEL_RECORD_TYPE = 2;
pub const IsochCommand: EXTENDED_AGP_REGISTER = 6;
pub const IsochStatus: EXTENDED_AGP_REGISTER = 0;
pub type KD_CALLBACK_ACTION = i32;
pub type KD_NAMESPACE_ENUM = i32;
pub const KERNEL_LARGE_STACK_COMMIT: u32 = 24576;
pub const KERNEL_LARGE_STACK_SIZE: u32 = 73728;
pub const KERNEL_MCA_EXCEPTION_STACK_SIZE: u32 = 8192;
pub const KERNEL_SHADOW_STACK_SIZE: u32 = 4096;
pub const KERNEL_STACK_SIZE: u32 = 24576;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KERNEL_USER_TIMES {
    pub CreateTime: i64,
    pub ExitTime: i64,
    pub KernelTime: i64,
    pub UserTime: i64,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct KEXCEPTION_FRAME {
    pub P1Home: u64,
    pub P2Home: u64,
    pub P3Home: u64,
    pub P4Home: u64,
    pub P5: u64,
    pub Spare1: u64,
    pub Xmm6: super::super::Win32::winnt::M128A,
    pub Xmm7: super::super::Win32::winnt::M128A,
    pub Xmm8: super::super::Win32::winnt::M128A,
    pub Xmm9: super::super::Win32::winnt::M128A,
    pub Xmm10: super::super::Win32::winnt::M128A,
    pub Xmm11: super::super::Win32::winnt::M128A,
    pub Xmm12: super::super::Win32::winnt::M128A,
    pub Xmm13: super::super::Win32::winnt::M128A,
    pub Xmm14: super::super::Win32::winnt::M128A,
    pub Xmm15: super::super::Win32::winnt::M128A,
    pub TrapFrame: u64,
    pub OutputBuffer: u64,
    pub OutputLength: u64,
    pub Spare2: u64,
    pub MxCsr: u64,
    pub Rbp: u64,
    pub Rbx: u64,
    pub Rdi: u64,
    pub Rsi: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,
    pub Return: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KEY_CACHED_INFORMATION {
    pub LastWriteTime: i64,
    pub TitleIndex: u32,
    pub SubKeys: u32,
    pub MaxNameLen: u32,
    pub Values: u32,
    pub MaxValueNameLen: u32,
    pub MaxValueDataLen: u32,
    pub NameLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KEY_LAYER_INFORMATION {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_NAME_INFORMATION {
    pub NameLength: u32,
    pub Name: [u16; 1],
}
impl Default for KEY_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct KEY_VIRTUALIZATION_INFORMATION {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct KFRED_TRAP_FRAME {
    pub TrapFrame: KTRAP_FRAME,
    pub EventInformation: u64,
    pub Reserved: u64,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KFRED_TRAP_FRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_excpt", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct KPCR {
    pub Anonymous: KPCR_0,
    pub IdtBase: *mut _KIDTENTRY64,
    pub Unused: [u64; 2],
    pub Irql: super::ntdef::KIRQL,
    pub SecondLevelCacheAssociativity: u8,
    pub ObsoleteNumber: u8,
    pub Fill0: u8,
    pub Unused0: [u32; 3],
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub StallScaleFactor: u32,
    pub Unused1: [*mut core::ffi::c_void; 3],
    pub KernelReserved: [u32; 15],
    pub SecondLevelCacheSize: u32,
    pub HalReserved: [u32; 16],
    pub Unused2: u32,
    pub KdVersionBlock: *mut core::ffi::c_void,
    pub Unused3: *mut core::ffi::c_void,
    pub PcrAlign1: [u32; 24],
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_excpt", feature = "Win32_winnt"))]
impl Default for KPCR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_excpt", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union KPCR_0 {
    pub NtTib: super::super::Win32::winnt::NT_TIB,
    pub Anonymous: KPCR_0_0,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_excpt", feature = "Win32_winnt"))]
impl Default for KPCR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_excpt", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct KPCR_0_0 {
    pub GdtBase: *mut _KGDTENTRY64,
    pub TssBase: *mut _KTSS64,
    pub UserRsp: u64,
    pub Self_: *mut KPCR,
    pub CurrentPrcb: *mut _KPRCB,
    pub LockArray: super::wdm::PKSPIN_LOCK_QUEUE,
    pub Used_Self: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_excpt", feature = "Win32_winnt"))]
impl Default for KPCR_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct KTRAP_FRAME {
    pub P1Home: u64,
    pub P2Home: u64,
    pub P3Home: u64,
    pub P4Home: u64,
    pub P5: u64,
    pub PreviousMode: super::wdm::KPROCESSOR_MODE,
    pub PreviousIrql: super::ntdef::KIRQL,
    pub FaultIndicator: u8,
    pub ExceptionActive: u8,
    pub MxCsr: u32,
    pub Rax: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub Anonymous: KTRAP_FRAME_0,
    pub Xmm0: super::super::Win32::winnt::M128A,
    pub Xmm1: super::super::Win32::winnt::M128A,
    pub Xmm2: super::super::Win32::winnt::M128A,
    pub Xmm3: super::super::Win32::winnt::M128A,
    pub Xmm4: super::super::Win32::winnt::M128A,
    pub Xmm5: super::super::Win32::winnt::M128A,
    pub Anonymous2: KTRAP_FRAME_1,
    pub Anonymous3: KTRAP_FRAME_2,
    pub Anonymous4: KTRAP_FRAME_3,
    pub SegDs: u16,
    pub SegEs: u16,
    pub SegFs: u16,
    pub SegGs: u16,
    pub TrapFrame: u64,
    pub Rbx: u64,
    pub Rdi: u64,
    pub Rsi: u64,
    pub Rbp: u64,
    pub Anonymous5: KTRAP_FRAME_4,
    pub Rip: u64,
    pub Anonymous6: KTRAP_FRAME_5,
    pub EFlags: u32,
    pub Fill2: u32,
    pub Rsp: u64,
    pub Anonymous7: KTRAP_FRAME_6,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KTRAP_FRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_0 {
    pub GsBase: u64,
    pub GsSwap: u64,
    pub VectorMask: u64,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KTRAP_FRAME_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_1 {
    pub FaultAddress: u64,
    pub ContextRecord: u64,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KTRAP_FRAME_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_2 {
    pub Anonymous: KTRAP_FRAME_2_0,
    pub Anonymous2: KTRAP_FRAME_2_1,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KTRAP_FRAME_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KTRAP_FRAME_2_0 {
    pub Dr0: u64,
    pub Dr1: u64,
    pub Dr2: u64,
    pub Dr3: u64,
    pub Dr6: u64,
    pub Dr7: u64,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct KTRAP_FRAME_2_1 {
    pub ShadowStackFrame: u64,
    pub Spare: [u64; 5],
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KTRAP_FRAME_2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KTRAP_FRAME_3 {
    pub DebugControl: u64,
    pub LastBranchToRip: u64,
    pub LastBranchFromRip: u64,
    pub LastExceptionToRip: u64,
    pub LastExceptionFromRip: u64,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_4 {
    pub ErrorCode: u64,
    pub ExceptionFrame: u64,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KTRAP_FRAME_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_5 {
    pub Anonymous: KTRAP_FRAME_5_0,
    pub Anonymous2: KTRAP_FRAME_5_1,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KTRAP_FRAME_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct KTRAP_FRAME_5_0 {
    pub SegCs: u16,
    pub Fill0: u8,
    pub Logging: u8,
    pub Fill1: [u16; 2],
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KTRAP_FRAME_5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KTRAP_FRAME_5_1 {
    pub _bitfield: u64,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_6 {
    pub Anonymous: KTRAP_FRAME_6_0,
    pub Anonymous2: KTRAP_FRAME_6_1,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KTRAP_FRAME_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KTRAP_FRAME_6_0 {
    pub SegSs: u16,
    pub Fill3: u16,
    pub Fill4: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct KTRAP_FRAME_6_1 {
    pub Anonymous: KTRAP_FRAME_6_1_0,
    pub _bitfield: u32,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KTRAP_FRAME_6_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_6_1_0 {
    pub Anonymous: KTRAP_FRAME_6_1_0_0,
    pub Anonymous2: KTRAP_FRAME_6_1_0_1,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KTRAP_FRAME_6_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KTRAP_FRAME_6_1_0_0 {
    pub EventInformation: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KTRAP_FRAME_6_1_0_1 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct KUMS_CONTEXT_HEADER {
    pub P1Home: u64,
    pub P2Home: u64,
    pub P3Home: u64,
    pub P4Home: u64,
    pub StackTop: *mut core::ffi::c_void,
    pub StackSize: u64,
    pub RspOffset: u64,
    pub Rip: u64,
    pub FltSave: super::super::Win32::winnt::PXMM_SAVE_AREA32,
    pub Anonymous: KUMS_CONTEXT_HEADER_0,
    pub TrapFrame: PKTRAP_FRAME,
    pub ExceptionFrame: PKEXCEPTION_FRAME,
    pub SourceThread: *mut super::wdm::_KTHREAD,
    pub Return: u64,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KUMS_CONTEXT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union KUMS_CONTEXT_HEADER_0 {
    pub Anonymous: KUMS_CONTEXT_HEADER_0_0,
    pub Flags: u64,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KUMS_CONTEXT_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KUMS_CONTEXT_HEADER_0_0 {
    pub _bitfield: u64,
}
pub const KUMS_UCH_VOLATILE_BIT: u32 = 0;
pub const KUMS_UCH_VOLATILE_MASK: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct KUSER_SHARED_DATA {
    pub TickCountLowDeprecated: u32,
    pub TickCountMultiplier: u32,
    pub InterruptTime: super::wdm::KSYSTEM_TIME,
    pub SystemTime: super::wdm::KSYSTEM_TIME,
    pub TimeZoneBias: super::wdm::KSYSTEM_TIME,
    pub ImageNumberLow: u16,
    pub ImageNumberHigh: u16,
    pub NtSystemRoot: [u16; 260],
    pub MaxStackTraceDepth: u32,
    pub CryptoExponent: u32,
    pub TimeZoneId: u32,
    pub LargePageMinimum: u32,
    pub AitSamplingValue: u32,
    pub AppCompatFlag: u32,
    pub RNGSeedVersion: u64,
    pub GlobalValidationRunlevel: u32,
    pub TimeZoneBiasStamp: i32,
    pub NtBuildNumber: u32,
    pub NtProductType: super::ntdef::NT_PRODUCT_TYPE,
    pub ProductTypeIsValid: bool,
    pub Reserved0: [bool; 1],
    pub NativeProcessorArchitecture: u16,
    pub NtMajorVersion: u32,
    pub NtMinorVersion: u32,
    pub ProcessorFeatures: [bool; 64],
    pub Reserved1: u32,
    pub Reserved3: u32,
    pub TimeSlip: u32,
    pub AlternativeArchitecture: super::wdm::ALTERNATIVE_ARCHITECTURE_TYPE,
    pub BootId: u32,
    pub SystemExpirationDate: i64,
    pub SuiteMask: u32,
    pub KdDebuggerEnabled: bool,
    pub Anonymous: KUSER_SHARED_DATA_0,
    pub CyclesPerYield: u16,
    pub ActiveConsoleId: u32,
    pub DismountCount: u32,
    pub ComPlusPackage: u32,
    pub LastSystemRITEventTickCount: u32,
    pub NumberOfPhysicalPages: u32,
    pub SafeBootMode: bool,
    pub Anonymous2: KUSER_SHARED_DATA_1,
    pub Reserved12: [u8; 2],
    pub Anonymous3: KUSER_SHARED_DATA_2,
    pub DataFlagsPad: [u32; 1],
    pub TestRetInstruction: u64,
    pub QpcFrequency: i64,
    pub SystemCall: u32,
    pub Reserved2: u32,
    pub FullNumberOfPhysicalPages: u64,
    pub SystemCallPad: [u64; 1],
    pub Anonymous4: KUSER_SHARED_DATA_3,
    pub Cookie: u32,
    pub CookiePad: [u32; 1],
    pub ConsoleSessionForegroundProcessId: i64,
    pub TimeUpdateLock: u64,
    pub BaselineSystemTimeQpc: u64,
    pub BaselineInterruptTimeQpc: u64,
    pub QpcSystemTimeIncrement: u64,
    pub QpcInterruptTimeIncrement: u64,
    pub QpcSystemTimeIncrementShift: u8,
    pub QpcInterruptTimeIncrementShift: u8,
    pub UnparkedProcessorCount: u16,
    pub EnclaveFeatureMask: [u32; 4],
    pub TelemetryCoverageRound: u32,
    pub UserModeGlobalLogger: [u16; 16],
    pub ImageFileExecutionOptions: u32,
    pub LangGenerationCount: u32,
    pub Reserved4: u64,
    pub InterruptTimeBias: u64,
    pub QpcBias: u64,
    pub ActiveProcessorCount: u32,
    pub ActiveGroupCount: u8,
    pub Reserved9: u8,
    pub Anonymous5: KUSER_SHARED_DATA_4,
    pub TimeZoneBiasEffectiveStart: i64,
    pub TimeZoneBiasEffectiveEnd: i64,
    pub XState: super::super::Win32::winnt::XSTATE_CONFIGURATION,
    pub UserPointerAuthMask: u64,
    pub Reserved10: [u32; 214],
    pub FeatureConfigurationChangeStamp: super::wdm::KSYSTEM_TIME,
    pub Spare: u32,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KUSER_SHARED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union KUSER_SHARED_DATA_0 {
    pub MitigationPolicies: u8,
    pub Anonymous: KUSER_SHARED_DATA_0_0,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KUSER_SHARED_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KUSER_SHARED_DATA_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union KUSER_SHARED_DATA_1 {
    pub VirtualizationFlags: u8,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KUSER_SHARED_DATA_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union KUSER_SHARED_DATA_2 {
    pub SharedDataFlags: u32,
    pub Anonymous: KUSER_SHARED_DATA_2_0,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KUSER_SHARED_DATA_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KUSER_SHARED_DATA_2_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union KUSER_SHARED_DATA_3 {
    pub TickCount: super::wdm::KSYSTEM_TIME,
    pub TickCountQuad: u64,
    pub Anonymous: KUSER_SHARED_DATA_3_0,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KUSER_SHARED_DATA_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct KUSER_SHARED_DATA_3_0 {
    pub ReservedTickCountOverlay: [u32; 3],
    pub TickCountPad: [u32; 1],
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KUSER_SHARED_DATA_3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub union KUSER_SHARED_DATA_4 {
    pub QpcData: u16,
    pub Anonymous: KUSER_SHARED_DATA_4_0,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
impl Default for KUSER_SHARED_DATA_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KUSER_SHARED_DATA_4_0 {
    pub QpcBypassEnabled: u8,
    pub QpcReserved: u8,
}
pub const KdConfigureDeviceAndContinue: KD_CALLBACK_ACTION = 0;
pub const KdConfigureDeviceAndStop: KD_CALLBACK_ACTION = 2;
pub const KdNameSpaceACPI: KD_NAMESPACE_ENUM = 1;
pub const KdNameSpaceAny: KD_NAMESPACE_ENUM = 2;
pub const KdNameSpaceMax: KD_NAMESPACE_ENUM = 4;
pub const KdNameSpaceNone: KD_NAMESPACE_ENUM = 3;
pub const KdNameSpacePCI: KD_NAMESPACE_ENUM = 0;
pub const KdSkipDeviceAndContinue: KD_CALLBACK_ACTION = 1;
pub const KdSkipDeviceAndStop: KD_CALLBACK_ACTION = 3;
pub const KeyboardController: CONFIGURATION_TYPE = 22;
pub const KeyboardPeripheral: CONFIGURATION_TYPE = 32;
pub const L0sAndL1EntryDisabled: PCI_EXPRESS_ASPM_CONTROL = 0;
pub const L0sAndL1EntryEnabled: PCI_EXPRESS_ASPM_CONTROL = 3;
pub const L0sAndL1EntrySupport: PCI_EXPRESS_ASPM_SUPPORT = 3;
pub const L0sEntryEnabled: PCI_EXPRESS_ASPM_CONTROL = 1;
pub const L0sEntrySupport: PCI_EXPRESS_ASPM_SUPPORT = 1;
pub const L0s_128ns_256ns: PCI_EXPRESS_L0s_EXIT_LATENCY = 2;
pub const L0s_1us_2us: PCI_EXPRESS_L0s_EXIT_LATENCY = 5;
pub const L0s_256ns_512ns: PCI_EXPRESS_L0s_EXIT_LATENCY = 3;
pub const L0s_2us_4us: PCI_EXPRESS_L0s_EXIT_LATENCY = 6;
pub const L0s_512ns_1us: PCI_EXPRESS_L0s_EXIT_LATENCY = 4;
pub const L0s_64ns_128ns: PCI_EXPRESS_L0s_EXIT_LATENCY = 1;
pub const L0s_Above4us: PCI_EXPRESS_L0s_EXIT_LATENCY = 7;
pub const L0s_Below64ns: PCI_EXPRESS_L0s_EXIT_LATENCY = 0;
pub const L1EntryEnabled: PCI_EXPRESS_ASPM_CONTROL = 2;
pub const L1EntrySupport: PCI_EXPRESS_ASPM_SUPPORT = 2;
pub const L1_16us_32us: PCI_EXPRESS_L1_EXIT_LATENCY = 5;
pub const L1_1us_2us: PCI_EXPRESS_L1_EXIT_LATENCY = 1;
pub const L1_2us_4us: PCI_EXPRESS_L1_EXIT_LATENCY = 2;
pub const L1_32us_64us: PCI_EXPRESS_L1_EXIT_LATENCY = 6;
pub const L1_4us_8us: PCI_EXPRESS_L1_EXIT_LATENCY = 3;
pub const L1_8us_16us: PCI_EXPRESS_L1_EXIT_LATENCY = 4;
pub const L1_Above64us: PCI_EXPRESS_L1_EXIT_LATENCY = 7;
pub const L1_Below1us: PCI_EXPRESS_L1_EXIT_LATENCY = 0;
pub const LINKSPEED_MTS_16000: u32 = 16000;
pub const LINKSPEED_MTS_2500: u32 = 2500;
pub const LINKSPEED_MTS_32000: u32 = 32000;
pub const LINKSPEED_MTS_5000: u32 = 5000;
pub const LINKSPEED_MTS_64000: u32 = 64000;
pub const LINKSPEED_MTS_8000: u32 = 8000;
pub const LinePeripheral: CONFIGURATION_TYPE = 35;
pub const LocationTypeFileSystem: STATE_LOCATION_TYPE = 1;
pub const LocationTypeMaximum: STATE_LOCATION_TYPE = 2;
pub const LocationTypeRegistry: STATE_LOCATION_TYPE = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MAP_REGISTER_ENTRY {
    pub MapRegister: *mut core::ffi::c_void,
    pub WriteToDevice: bool,
}
impl Default for MAP_REGISTER_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAXIMUM_DEBUG_BARS: u32 = 6;
pub const MAXIMUM_EXPANSION_SIZE: u32 = 71680;
pub const MAX_SEL_RAW_EVENT_PAYLOAD_LENGTH: u32 = 256;
#[repr(C)]
#[cfg(all(feature = "Wdk_mce", feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct MCA_DRIVER_INFO {
    pub ExceptionCallback: PDRIVER_MCA_EXCEPTION_CALLBACK,
    pub DpcCallback: super::wdm::PKDEFERRED_ROUTINE,
    pub DeviceContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_mce", feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_winnt"))]
impl Default for MCA_DRIVER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCE_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe8f56ffe_919c_4cc5_ba88_65abe14913bb);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MCG_CAP {
    pub Anonymous: MCG_CAP_0,
    pub QuadPart: u64,
}
impl Default for MCG_CAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCG_CAP_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MCG_STATUS {
    pub Anonymous: MCG_STATUS_0,
    pub QuadPart: u64,
}
impl Default for MCG_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCG_STATUS_0 {
    pub _bitfield: u32,
    pub Reserved2: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MCI_STATUS {
    pub CommonBits: MCI_STATUS_BITS_COMMON,
    pub AmdBits: MCI_STATUS_AMD_BITS,
    pub IntelBits: MCI_STATUS_INTEL_BITS,
    pub QuadPart: u64,
}
impl Default for MCI_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_STATUS: u32 = 2068;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_STATUS_AMD_BITS {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_STATUS_BITS_COMMON {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MCI_STATUS_INTEL_BITS {
    pub _bitfield: u64,
}
pub const MEMORY_CORRECTABLE_ERROR_SUMMARY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0e36c93e_ca15_4a83_ba8a_cbe80f7f0017);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MEMORY_DEFECT {
    pub Version: u16,
    pub Flags: MEMORY_DEFECT_FLAGS,
    pub DimmInfo: DIMM_INFO,
    pub ErrType: PAGE_OFFLINE_ERROR_TYPES,
}
impl Default for MEMORY_DEFECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MEMORY_DEFECT_FLAGS {
    pub Anonymous: MEMORY_DEFECT_FLAGS_0,
    pub AsUINT16: u16,
}
impl Default for MEMORY_DEFECT_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MEMORY_DEFECT_FLAGS_0 {
    pub _bitfield: u16,
}
pub const MEMORY_ERROR_EXT_SECTION_AMD_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0d3a4a3b_774a_4c72_95e2_152d43180374);
pub const MEMORY_ERROR_EXT_SECTION_INTEL_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe16edb28_6113_4263_a41d_e53f8de78751);
pub const MEMORY_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa5bc1114_6f64_4ede_b863_3e83ed7c83b1);
pub const MM_ADD_PHYSICAL_MEMORY_ALREADY_ZEROED: u32 = 1;
pub const MM_ADD_PHYSICAL_MEMORY_HUGE_PAGES_ONLY: u32 = 4;
pub const MM_ADD_PHYSICAL_MEMORY_LARGE_PAGES_ONLY: u32 = 2;
#[repr(C)]
#[cfg(feature = "Win32_usb")]
#[derive(Clone, Copy)]
pub struct MM_COPY_ADDRESS {
    pub Anonymous: MM_COPY_ADDRESS_0,
}
#[cfg(feature = "Win32_usb")]
impl Default for MM_COPY_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_usb")]
#[derive(Clone, Copy)]
pub union MM_COPY_ADDRESS_0 {
    pub VirtualAddress: *mut core::ffi::c_void,
    pub PhysicalAddress: super::super::Win32::usb::PHYSICAL_ADDRESS,
}
#[cfg(feature = "Win32_usb")]
impl Default for MM_COPY_ADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MM_COPY_MEMORY_PHYSICAL: u32 = 1;
pub const MM_COPY_MEMORY_VIRTUAL: u32 = 2;
pub const MM_GET_CACHE_ATTRIBUTE_IO_SPACE: u32 = 1;
pub const MM_GET_PHYSICAL_MEMORY_RANGES_INCLUDE_ALL_PARTITIONS: u32 = 2;
pub const MM_GET_PHYSICAL_MEMORY_RANGES_INCLUDE_FILE_ONLY: u32 = 1;
pub const MM_REMOVE_PHYSICAL_MEMORY_BAD_ONLY: u32 = 1;
pub type MM_ROTATE_DIRECTION = i32;
pub const MM_SECURE_EXCLUSIVE: u32 = 1;
pub const MM_SECURE_NO_CHANGE: u32 = 2;
pub const MM_SECURE_NO_INHERIT: u32 = 8;
pub const MM_SECURE_USER_MODE_ONLY: u32 = 4;
pub const MM_SYSTEM_PARTITION_OBJECT: u32 = 0;
pub const MM_SYSTEM_VIEW_EXCEPTIONS_FOR_INPAGE_ERRORS: u32 = 1;
pub const MPIConfiguration: BUS_DATA_TYPE = 8;
pub const MPSAConfiguration: BUS_DATA_TYPE = 9;
pub const MRLClosed: PCI_EXPRESS_MRL_STATE = 0;
pub const MRLOpen: PCI_EXPRESS_MRL_STATE = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct MU_TELEMETRY_SECTION {
    pub ComponentID: windows_sys::core::GUID,
    pub SubComponentID: windows_sys::core::GUID,
    pub Reserved: u32,
    pub ErrorStatusValue: u32,
    pub AdditionalInfo1: u64,
    pub AdditionalInfo2: u64,
}
pub const MU_TELEMETRY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x85183a8b_9c41_429c_939c_5c3c087ca280);
pub const MaxPayload1024Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 3;
pub const MaxPayload128Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 0;
pub const MaxPayload2048Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 4;
pub const MaxPayload256Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 1;
pub const MaxPayload4096Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 5;
pub const MaxPayload512Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 2;
pub const MaxSubsystemInformationType: SUBSYSTEM_INFORMATION_TYPE = 2;
pub const MaxTimerInfoClass: TIMER_SET_INFORMATION_CLASS = 1;
pub const MaximumBusDataType: BUS_DATA_TYPE = 12;
pub const MaximumType: CONFIGURATION_TYPE = 41;
pub const MmMaximumRotateDirection: MM_ROTATE_DIRECTION = 4;
pub const MmToFrameBuffer: MM_ROTATE_DIRECTION = 0;
pub const MmToFrameBufferNoCopy: MM_ROTATE_DIRECTION = 1;
pub const MmToRegularMemory: MM_ROTATE_DIRECTION = 2;
pub const MmToRegularMemoryNoCopy: MM_ROTATE_DIRECTION = 3;
pub const ModemPeripheral: CONFIGURATION_TYPE = 28;
pub const MonitorPeripheral: CONFIGURATION_TYPE = 29;
pub const MultiFunctionAdapter: CONFIGURATION_TYPE = 12;
pub const NMI_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5bad89ff_b7e6_42c9_814a_cf2485d6e98a);
pub const NMI_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe71254e7_c1b9_4940_ab76_909703a4320f);
pub const NX_SUPPORT_POLICY_ALWAYSOFF: u32 = 0;
pub const NX_SUPPORT_POLICY_ALWAYSON: u32 = 1;
pub const NX_SUPPORT_POLICY_OPTIN: u32 = 2;
pub const NX_SUPPORT_POLICY_OPTOUT: u32 = 3;
pub const NetworkController: CONFIGURATION_TYPE = 18;
pub const NetworkPeripheral: CONFIGURATION_TYPE = 36;
pub const NoAspmSupport: PCI_EXPRESS_ASPM_SUPPORT = 0;
pub const NoCxlSupport: CXL_PROTOCOL_VERSION = 0;
pub const NoMemoryForWrapper: WHEA_OFFLINE_ERRS = 11;
pub const NotDdr: WHEA_OFFLINE_ERRS = 8;
pub const NuBusConfiguration: BUS_DATA_TYPE = 6;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OPLOCK_KEY_CONTEXT {
    pub Version: u16,
    pub Flags: u16,
    pub ParentOplockKey: windows_sys::core::GUID,
    pub TargetOplockKey: windows_sys::core::GUID,
    pub Reserved: u32,
}
pub const OPLOCK_KEY_FLAG_PARENT_KEY: u32 = 1;
pub const OPLOCK_KEY_FLAG_TARGET_KEY: u32 = 2;
pub const OPLOCK_KEY_VERSION_WIN7: u32 = 1;
pub const OPLOCK_KEY_VERSION_WIN8: u32 = 2;
pub const OSC_CAPABILITIES_MASKED: u32 = 16;
pub const OSC_FIRMWARE_FAILURE: u32 = 2;
pub const OSC_UNRECOGNIZED_REVISION: u32 = 8;
pub const OSC_UNRECOGNIZED_UUID: u32 = 4;
pub const OtherController: CONFIGURATION_TYPE = 24;
pub const OtherPeripheral: CONFIGURATION_TYPE = 34;
pub type PACPI_DEBUGGING_DEVICE_IN_USE = *mut ACPI_DEBUGGING_DEVICE_IN_USE;
pub type PAER_BRIDGE_DESCRIPTOR_FLAGS = *mut AER_BRIDGE_DESCRIPTOR_FLAGS;
pub type PAER_ENDPOINT_DESCRIPTOR_FLAGS = *mut AER_ENDPOINT_DESCRIPTOR_FLAGS;
pub type PAER_ROOTPORT_DESCRIPTOR_FLAGS = *mut AER_ROOTPORT_DESCRIPTOR_FLAGS;
pub type PAGE_OFFLINE_ERROR_TYPES = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PAGE_OFFLINE_VALID_BITS {
    pub Anonymous: PAGE_OFFLINE_VALID_BITS_0,
    pub AsUINT8: u8,
}
impl Default for PAGE_OFFLINE_VALID_BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PAGE_OFFLINE_VALID_BITS_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PAGE_PRIORITY_INFORMATION {
    pub PagePriority: u32,
}
#[cfg(feature = "Wdk_wdm")]
pub type PAGP_TARGET_BUS_INTERFACE_STANDARD = *mut AGP_TARGET_BUS_INTERFACE_STANDARD;
pub type PARBITER_ACTION = *mut ARBITER_ACTION;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PARBITER_ADD_RESERVED_PARAMETERS = *mut ARBITER_ADD_RESERVED_PARAMETERS;
#[cfg(feature = "Win32_winnt")]
pub type PARBITER_BOOT_ALLOCATION_PARAMETERS = *mut ARBITER_BOOT_ALLOCATION_PARAMETERS;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PARBITER_CONFLICT_INFO = *mut ARBITER_CONFLICT_INFO;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PARBITER_HANDLER = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, action: ARBITER_ACTION, parameters: *mut ARBITER_PARAMETERS) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PARBITER_INTERFACE = *mut ARBITER_INTERFACE;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PARBITER_LIST_ENTRY = *mut ARBITER_LIST_ENTRY;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PARBITER_PARAMETERS = *mut ARBITER_PARAMETERS;
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_usb"))]
pub type PARBITER_QUERY_ALLOCATED_RESOURCES_PARAMETERS = *mut ARBITER_QUERY_ALLOCATED_RESOURCES_PARAMETERS;
#[cfg(feature = "Win32_winnt")]
pub type PARBITER_QUERY_ARBITRATE_PARAMETERS = *mut ARBITER_QUERY_ARBITRATE_PARAMETERS;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_minwindef", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PARBITER_QUERY_CONFLICT_PARAMETERS = *mut ARBITER_QUERY_CONFLICT_PARAMETERS;
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_usb", feature = "Win32_winnt"))]
pub type PARBITER_RETEST_ALLOCATION_PARAMETERS = *mut ARBITER_RETEST_ALLOCATION_PARAMETERS;
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_usb", feature = "Win32_winnt"))]
pub type PARBITER_TEST_ALLOCATION_PARAMETERS = *mut ARBITER_TEST_ALLOCATION_PARAMETERS;
pub type PBDCB_CALLBACK_TYPE = *mut BDCB_CALLBACK_TYPE;
pub type PBDCB_CLASSIFICATION = *mut BDCB_CLASSIFICATION;
pub type PBDCB_STATUS_UPDATE_CONTEXT = *mut BDCB_STATUS_UPDATE_CONTEXT;
pub type PBDCB_STATUS_UPDATE_TYPE = *mut BDCB_STATUS_UPDATE_TYPE;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
pub type PBOOT_DRIVER_CALLBACK_FUNCTION = *mut BOOT_DRIVER_CALLBACK_FUNCTION;
pub type PBUS_DATA_TYPE = *mut BUS_DATA_TYPE;
pub type PBUS_HANDLER = *mut _BUS_HANDLER;
pub const PCCARD_DEVICE_PCI: u32 = 16;
pub const PCCARD_DUP_LEGACY_BASE: u32 = 6;
pub const PCCARD_MAP_ERROR: u32 = 1;
pub const PCCARD_MAP_ZERO: u32 = 2;
pub const PCCARD_NO_CONTROLLERS: u32 = 7;
pub const PCCARD_NO_LEGACY_BASE: u32 = 5;
pub const PCCARD_NO_PIC: u32 = 4;
pub const PCCARD_NO_TIMER: u32 = 3;
pub const PCCARD_SCAN_DISABLED: u32 = 1;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_usb", feature = "Win32_winnt"))]
pub type PCDEBUG_DEVICE_DESCRIPTOR = *const DEBUG_DEVICE_DESCRIPTOR;
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCIBUSDATA {
    pub Tag: u32,
    pub Version: u32,
    pub ReadConfig: PciReadWriteConfig,
    pub WriteConfig: PciReadWriteConfig,
    pub Pin2Line: PciPin2Line,
    pub Line2Pin: PciLine2Pin,
    pub ParentSlot: super::wdm::PCI_SLOT_NUMBER,
    pub Reserved: [*mut core::ffi::c_void; 4],
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCIBUSDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCIConfiguration: BUS_DATA_TYPE = 4;
pub const PCIEXPRESS_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd995e954_bbc1_430f_ad91_b44dcb3c6f35);
pub const PCIE_CORRECTABLE_ERROR_SUMMARY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe96eca99_53e2_4f52_9be7_d2dbe9508ed0);
pub const PCIXBUS_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc5753963_3b84_4095_bf78_eddad3f9c9dd);
pub const PCIXBUS_ERRTYPE_ADDRESSPARITY: u32 = 6;
pub const PCIXBUS_ERRTYPE_BUSTIMEOUT: u32 = 4;
pub const PCIXBUS_ERRTYPE_COMMANDPARITY: u32 = 7;
pub const PCIXBUS_ERRTYPE_DATAPARITY: u32 = 1;
pub const PCIXBUS_ERRTYPE_MASTERABORT: u32 = 3;
pub const PCIXBUS_ERRTYPE_MASTERDATAPARITY: u32 = 5;
pub const PCIXBUS_ERRTYPE_SYSTEM: u32 = 2;
pub const PCIXBUS_ERRTYPE_UNKNOWN: u32 = 0;
pub const PCIXDEVICE_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeb5e4685_ca66_4769_b6a2_26068b001326);
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCIX_BRIDGE_CAPABILITY {
    pub Header: super::wdm::PCI_CAPABILITIES_HEADER,
    pub SecondaryStatus: PCIX_BRIDGE_CAPABILITY_0,
    pub BridgeStatus: PCIX_BRIDGE_CAPABILITY_1,
    pub UpstreamSplitTransactionCapacity: u16,
    pub UpstreamSplitTransactionLimit: u16,
    pub DownstreamSplitTransactionCapacity: u16,
    pub DownstreamSplitTransactionLimit: u16,
    pub EccControlStatus: PCIX_BRIDGE_CAPABILITY_2,
    pub EccFirstAddress: u32,
    pub EccSecondAddress: u32,
    pub EccAttribute: u32,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCIX_BRIDGE_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub union PCIX_BRIDGE_CAPABILITY_0 {
    pub Anonymous: PCIX_BRIDGE_CAPABILITY_0_0,
    pub AsUSHORT: u16,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCIX_BRIDGE_CAPABILITY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy, Default)]
pub struct PCIX_BRIDGE_CAPABILITY_0_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub union PCIX_BRIDGE_CAPABILITY_1 {
    pub Anonymous: PCIX_BRIDGE_CAPABILITY_1_0,
    pub AsULONG: u32,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCIX_BRIDGE_CAPABILITY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy, Default)]
pub struct PCIX_BRIDGE_CAPABILITY_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub union PCIX_BRIDGE_CAPABILITY_2 {
    pub Anonymous: PCIX_BRIDGE_CAPABILITY_2_0,
    pub AsULONG: u32,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCIX_BRIDGE_CAPABILITY_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy, Default)]
pub struct PCIX_BRIDGE_CAPABILITY_2_0 {
    pub _bitfield: u32,
}
pub const PCIX_MODE1_100MHZ: u32 = 2;
pub const PCIX_MODE1_133MHZ: u32 = 3;
pub const PCIX_MODE1_66MHZ: u32 = 1;
pub const PCIX_MODE2_266_100MHZ: u32 = 10;
pub const PCIX_MODE2_266_133MHZ: u32 = 11;
pub const PCIX_MODE2_266_66MHZ: u32 = 9;
pub const PCIX_MODE2_533_100MHZ: u32 = 14;
pub const PCIX_MODE2_533_133MHZ: u32 = 15;
pub const PCIX_MODE2_533_66MHZ: u32 = 13;
pub const PCIX_MODE_CONVENTIONAL_PCI: u32 = 0;
pub const PCIX_VERSION_DUAL_MODE_ECC: u32 = 2;
pub const PCIX_VERSION_MODE1_ONLY: u32 = 0;
pub const PCIX_VERSION_MODE2_ECC: u32 = 1;
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_ADVANCED_FEATURES_CAPABILITY {
    pub Header: super::wdm::PCI_CAPABILITIES_HEADER,
    pub Length: u8,
    pub Capabilities: PCI_ADVANCED_FEATURES_CAPABILITY_0,
    pub Control: PCI_ADVANCED_FEATURES_CAPABILITY_1,
    pub Status: PCI_ADVANCED_FEATURES_CAPABILITY_2,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_ADVANCED_FEATURES_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub union PCI_ADVANCED_FEATURES_CAPABILITY_0 {
    pub Anonymous: PCI_ADVANCED_FEATURES_CAPABILITY_0_0,
    pub AsUCHAR: u8,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_ADVANCED_FEATURES_CAPABILITY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy, Default)]
pub struct PCI_ADVANCED_FEATURES_CAPABILITY_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub union PCI_ADVANCED_FEATURES_CAPABILITY_1 {
    pub Anonymous: PCI_ADVANCED_FEATURES_CAPABILITY_1_0,
    pub AsUCHAR: u8,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_ADVANCED_FEATURES_CAPABILITY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy, Default)]
pub struct PCI_ADVANCED_FEATURES_CAPABILITY_1_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub union PCI_ADVANCED_FEATURES_CAPABILITY_2 {
    pub Anonymous: PCI_ADVANCED_FEATURES_CAPABILITY_2_0,
    pub AsUCHAR: u8,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_ADVANCED_FEATURES_CAPABILITY_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy, Default)]
pub struct PCI_ADVANCED_FEATURES_CAPABILITY_2_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_APERTURE_PAGE_SIZE {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_CAPABILITY {
    pub Header: super::wdm::PCI_CAPABILITIES_HEADER,
    pub _bitfield: u16,
    pub AGPStatus: PCI_AGP_CAPABILITY_0,
    pub AGPCommand: PCI_AGP_CAPABILITY_1,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_CAPABILITY_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_CAPABILITY_1 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_CONTROL {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_EXTENDED_CAPABILITY {
    pub IsochStatus: PCI_AGP_ISOCH_STATUS,
    pub AgpControl: PCI_AGP_CONTROL,
    pub ApertureSize: u16,
    pub AperturePageSize: PCI_AGP_APERTURE_PAGE_SIZE,
    pub GartLow: u32,
    pub GartHigh: u32,
    pub IsochCommand: PCI_AGP_ISOCH_COMMAND,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_ISOCH_COMMAND {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_ISOCH_STATUS {
    pub _bitfield: u32,
}
pub const PCI_AGP_RATE_1X: u32 = 1;
pub const PCI_AGP_RATE_2X: u32 = 2;
pub const PCI_AGP_RATE_4X: u32 = 4;
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_BUS_INTERFACE_STANDARD {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: super::wdm::PINTERFACE_REFERENCE,
    pub InterfaceDereference: super::wdm::PINTERFACE_DEREFERENCE,
    pub ReadConfig: PPCI_READ_WRITE_CONFIG,
    pub WriteConfig: PPCI_READ_WRITE_CONFIG,
    pub PinToLine: PPCI_PIN_TO_LINE,
    pub LineToPin: PPCI_LINE_TO_PIN,
    pub RootBusCapability: PPCI_ROOT_BUS_CAPABILITY,
    pub ExpressWakeControl: PPCI_EXPRESS_WAKE_CONTROL,
    pub PrepareMultistageResume: PPCI_PREPARE_MULTISTAGE_RESUME,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_BUS_INTERFACE_STANDARD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCI_BUS_INTERFACE_STANDARD_VERSION: u32 = 2;
pub const PCI_BUS_INTERFACE_STANDARD_VERSION_1_LENGTH: u32 = 80;
pub type PCI_BUS_WIDTH = i32;
pub const PCI_DATA_TAG: u32 = 541672272;
pub const PCI_DATA_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_DEBUGGING_DEVICE_IN_USE {
    pub Segment: u16,
    pub Bus: u32,
    pub Slot: u32,
}
pub type PCI_DEVICE_D3COLD_STATE_REASON = i32;
pub const PCI_DOE_OBJECT_TYPE_ASYNC_MESSAGE: u32 = 5;
pub const PCI_DOE_OBJECT_TYPE_CMA_SPDM: u32 = 1;
pub const PCI_DOE_OBJECT_TYPE_CMA_SPDM_WITH_CONNECTION_ID: u32 = 3;
pub const PCI_DOE_OBJECT_TYPE_DISCOVERY: u32 = 0;
pub const PCI_DOE_OBJECT_TYPE_SECURED_CMA_SPDM: u32 = 2;
pub const PCI_DOE_OBJECT_TYPE_SECURED_CMA_SPDM_WITH_CONNECTION_ID: u32 = 4;
pub const PCI_DOE_VENDOR_ID_RESERVED: u32 = 1;
pub type PCI_ERROR_HANDLER_CALLBACK = Option<unsafe extern "system" fn()>;
pub const PCI_EXPRESS_AER_DEVICE_CONTROL_MASK: u32 = 7;
pub const PCI_EXPRESS_AER_DEVICE_STATUS_MASK: u32 = 15;
pub type PCI_EXPRESS_ASPM_CONTROL = i32;
pub type PCI_EXPRESS_ASPM_SUPPORT = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_CAPABILITIES_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CAPABILITY {
    pub Header: super::wdm::PCI_CAPABILITIES_HEADER,
    pub ExpressCapabilities: PCI_EXPRESS_CAPABILITIES_REGISTER,
    pub DeviceCapabilities: PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER,
    pub DeviceControl: PCI_EXPRESS_DEVICE_CONTROL_REGISTER,
    pub DeviceStatus: PCI_EXPRESS_DEVICE_STATUS_REGISTER,
    pub LinkCapabilities: PCI_EXPRESS_LINK_CAPABILITIES_REGISTER,
    pub LinkControl: PCI_EXPRESS_LINK_CONTROL_REGISTER,
    pub LinkStatus: PCI_EXPRESS_LINK_STATUS_REGISTER,
    pub SlotCapabilities: PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER,
    pub SlotControl: PCI_EXPRESS_SLOT_CONTROL_REGISTER,
    pub SlotStatus: PCI_EXPRESS_SLOT_STATUS_REGISTER,
    pub RootControl: PCI_EXPRESS_ROOT_CONTROL_REGISTER,
    pub RootCapabilities: PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER,
    pub RootStatus: PCI_EXPRESS_ROOT_STATUS_REGISTER,
    pub DeviceCapabilities2: PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER,
    pub DeviceControl2: PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER,
    pub DeviceStatus2: PCI_EXPRESS_DEVICE_STATUS_2_REGISTER,
    pub LinkCapabilities2: PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER,
    pub LinkControl2: PCI_EXPRESS_LINK_CONTROL_2_REGISTER,
    pub LinkStatus2: PCI_EXPRESS_LINK_STATUS_2_REGISTER,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_EXPRESS_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PCI_EXPRESS_CARD_PRESENCE = i32;
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CXL_DVSEC_CAPABILITY {
    pub Header: super::wdm::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DvsecHeader1: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1,
    pub DvsecHeader2: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2,
    pub Reserved: [u8; 46],
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_EXPRESS_CXL_DVSEC_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11 {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V2 {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V2_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V2_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CXL_DVSEC_CAPABILITY_V11 {
    pub Header: super::wdm::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DvsecHeader1: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1,
    pub DvsecHeader2: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2,
    pub Capability: PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11,
    pub Control: PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER,
    pub Status: PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER,
    pub Control2: u16,
    pub Status2: u16,
    pub Lock: PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER,
    pub Reserved: u16,
    pub Range1SizeHigh: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_HIGH_REGISTER,
    pub Range1SizeLow: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11,
    pub Range1BaseHigh: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_HIGH_REGISTER,
    pub Range1BaseLow: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER,
    pub Range2SizeHigh: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_HIGH_REGISTER,
    pub Range2SizeLow: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11,
    pub Range2BaseHigh: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_HIGH_REGISTER,
    pub Range2BaseLow: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_EXPRESS_CXL_DVSEC_CAPABILITY_V11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_HIGH_REGISTER {
    pub MemBaseHigh: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_HIGH_REGISTER {
    pub MemSizeHigh: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11 {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DATA_OBJECT_EXCHANGE_EXTENDED_CAPABILITY {
    pub Header: super::wdm::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DoeCapabilities: PCI_EXPRESS_DOE_CAPABILITIES_REGISTER,
    pub DoeControl: PCI_EXPRESS_DOE_CONTROL_REGISTER,
    pub DoeStatus: PCI_EXPRESS_DOE_STATUS_REGISTER,
    pub WriteMailbox: PCI_EXPRESS_DOE_WRITE_MAILBOX_REGISTER,
    pub ReadMailbox: PCI_EXPRESS_DOE_READ_MAILBOX_REGISTER,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_EXPRESS_DATA_OBJECT_EXCHANGE_EXTENDED_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY {
    pub Header: super::wdm::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DvsecHeader1: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1,
    pub DvsecHeader2: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2,
    pub DvsecRegisters: [u16; 1],
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1 {
    pub Anonymous: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2 {
    pub Anonymous: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_CONTROL_REGISTER_0,
    pub Anonymous2: PCI_EXPRESS_DEVICE_CONTROL_REGISTER_1,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_DEVICE_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DEVICE_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DEVICE_CONTROL_REGISTER_1 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_STATUS_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_STATUS_2_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_DEVICE_STATUS_2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DEVICE_STATUS_2_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_STATUS_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_DEVICE_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DEVICE_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
pub type PCI_EXPRESS_DEVICE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DOE_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_DOE_CAPABILITIES_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_DOE_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DOE_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DOE_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_DOE_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_DOE_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DOE_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DOE_READ_MAILBOX_REGISTER {
    pub ReadDataMailbox: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DOE_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_DOE_STATUS_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_DOE_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DOE_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DOE_WRITE_MAILBOX_REGISTER {
    pub WriteDataMailbox: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_CAPABILITY {
    pub Header: super::wdm::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DpcCapabilities: PCI_EXPRESS_DPC_CAPS_REGISTER,
    pub DpcControl: PCI_EXPRESS_DPC_CONTROL_REGISTER,
    pub DpcStatus: PCI_EXPRESS_DPC_STATUS_REGISTER,
    pub DpcErrSrcId: PCI_EXPRESS_DPC_ERROR_SOURCE_ID,
    pub RpPioStatus: PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER,
    pub RpPioMask: PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER,
    pub RpPioSeverity: PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER,
    pub RpPioSysError: PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER,
    pub RpPioException: PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER,
    pub RpPioHeaderLog: PCI_EXPRESS_DPC_RP_PIO_HEADERLOG_REGISTER,
    pub RpPioImpSpecLog: PCI_EXPRESS_DPC_RP_PIO_IMPSPECLOG_REGISTER,
    pub RpPioPrefixLog: PCI_EXPRESS_DPC_RP_PIO_TLPPREFIXLOG_REGISTER,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_EXPRESS_DPC_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_CAPS_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_CAPS_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_DPC_CAPS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_CAPS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_DPC_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_ERROR_SOURCE_ID {
    pub Anonymous: PCI_EXPRESS_DPC_ERROR_SOURCE_ID_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_DPC_ERROR_SOURCE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_ERROR_SOURCE_ID_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_RP_PIO_HEADERLOG_REGISTER {
    pub PioHeaderLogRegister: [u32; 4],
}
impl Default for PCI_EXPRESS_DPC_RP_PIO_HEADERLOG_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_IMPSPECLOG_REGISTER {
    pub PioImpSpecLog: u32,
}
impl Default for PCI_EXPRESS_DPC_RP_PIO_IMPSPECLOG_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_RP_PIO_TLPPREFIXLOG_REGISTER {
    pub PioTlpPrefixLogRegister: [u32; 4],
}
impl Default for PCI_EXPRESS_DPC_RP_PIO_TLPPREFIXLOG_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_STATUS_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_DPC_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_EVENT_COLLECTOR_ENDPOINT_ASSOCIATION_CAPABILITY {
    pub Header: super::wdm::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub AssociationBitmap: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_IDE_ADDRESS_ASSOCIATION_BLOCK {
    pub Reserved: [u32; 3],
}
impl Default for PCI_EXPRESS_IDE_ADDRESS_ASSOCIATION_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_IDE_CAPABILITY {
    pub Header: super::wdm::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capability: PCI_EXPRESS_IDE_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_IDE_CONTROL_REGISTER,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_EXPRESS_IDE_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_IDE_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_IDE_CAPABILITY_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_IDE_CAPABILITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_IDE_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_IDE_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_IDE_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_IDE_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_IDE_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_1 {
    pub Anonymous: PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_1_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_2 {
    pub Anonymous: PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_2_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_2_0 {
    pub _bitfield: u32,
}
pub type PCI_EXPRESS_INDICATOR_STATE = i32;
pub type PCI_EXPRESS_L0s_EXIT_LATENCY = i32;
pub type PCI_EXPRESS_L1_EXIT_LATENCY = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_L1_PM_SS_CAPABILITY {
    pub Header: super::wdm::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub L1PmSsCapabilities: PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER,
    pub L1PmSsControl1: PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER,
    pub L1PmSsControl2: PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_EXPRESS_L1_PM_SS_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER {
    pub Anonymous: PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LANE_ERROR_STATUS {
    pub LaneBitmap: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_CAPABILITIES_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_LINK_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LINK_CONTROL3 {
    pub Anonymous: PCI_EXPRESS_LINK_CONTROL3_0,
}
impl Default for PCI_EXPRESS_LINK_CONTROL3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CONTROL3_0 {
    pub Anonymous: PCI_EXPRESS_LINK_CONTROL3_0_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_LINK_CONTROL3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_CONTROL3_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CONTROL_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_CONTROL_2_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_LINK_CONTROL_2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_CONTROL_2_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_LINK_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LINK_IDE_BLOCK {
    pub Control: PCI_EXPRESS_LINK_IDE_STREAM_CONTROL_REGISTER,
    pub Status: PCI_EXPRESS_LINK_IDE_STREAM_STATUS_REGISTER,
}
impl Default for PCI_EXPRESS_LINK_IDE_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_IDE_STREAM_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_IDE_STREAM_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_LINK_IDE_STREAM_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_IDE_STREAM_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_IDE_STREAM_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_IDE_STREAM_STATUS_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_LINK_IDE_STREAM_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_IDE_STREAM_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_STATUS_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_STATUS_2_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_LINK_STATUS_2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_STATUS_2_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_STATUS_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_LINK_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
pub type PCI_EXPRESS_LINK_SUBSTATE = i32;
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LTR_CAPABILITY {
    pub Header: super::wdm::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Latency: PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_EXPRESS_LTR_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER {
    pub Anonymous: PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER_0 {
    pub _bitfield: u32,
}
pub type PCI_EXPRESS_MAX_PAYLOAD_SIZE = i32;
pub type PCI_EXPRESS_MRL_STATE = i32;
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_NPEM_CAPABILITY {
    pub Header: super::wdm::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capability: PCI_EXPRESS_NPEM_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_NPEM_CONTROL_REGISTER,
    pub Status: PCI_EXPRESS_NPEM_STATUS_REGISTER,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_EXPRESS_NPEM_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_NPEM_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_NPEM_CAPABILITY_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_NPEM_CAPABILITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_NPEM_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_NPEM_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_NPEM_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_NPEM_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_NPEM_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_NPEM_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_NPEM_STATUS_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_NPEM_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_NPEM_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PME_REQUESTOR_ID {
    pub Anonymous: PCI_EXPRESS_PME_REQUESTOR_ID_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_PME_REQUESTOR_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_PME_REQUESTOR_ID_0 {
    pub _bitfield: u16,
}
pub type PCI_EXPRESS_POWER_STATE = i32;
pub type PCI_EXPRESS_RCB = i32;
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY {
    pub Header: super::wdm::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Entry: [PCI_EXPRESS_RESIZABLE_BAR_ENTRY; 6],
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_RESIZABLE_BAR_ENTRY {
    pub Capability: PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER,
}
impl Default for PCI_EXPRESS_RESIZABLE_BAR_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ROOT_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_ROOT_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_ROOT_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_ROOT_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ROOT_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_ROOT_STATUS_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_ROOT_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_ROOT_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SECONDARY_CAPABILITY {
    pub Header: super::wdm::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub LinkControl3: PCI_EXPRESS_LINK_CONTROL3,
    pub LaneErrorStatus: PCI_EXPRESS_LANE_ERROR_STATUS,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_EXPRESS_SECONDARY_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SELECTIVE_IDE_BLOCK {
    pub Capability: PCI_EXPRESS_SELECTIVE_IDE_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_SELECTIVE_IDE_CONTROL_REGISTER,
    pub Status: PCI_EXPRESS_SELECTIVE_IDE_STATUS_REGISTER,
    pub RidAssociation1: PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_1,
    pub RidAssociation2: PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_2,
}
impl Default for PCI_EXPRESS_SELECTIVE_IDE_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SELECTIVE_IDE_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_SELECTIVE_IDE_CAPABILITY_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_SELECTIVE_IDE_CAPABILITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_SELECTIVE_IDE_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SELECTIVE_IDE_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_SELECTIVE_IDE_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_SELECTIVE_IDE_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_SELECTIVE_IDE_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SELECTIVE_IDE_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_SELECTIVE_IDE_STATUS_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_SELECTIVE_IDE_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_SELECTIVE_IDE_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SLOT_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_SLOT_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_SLOT_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_SLOT_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SLOT_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_SLOT_STATUS_REGISTER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_SLOT_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_SLOT_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_TPH_REQUESTER_CAPABILITY {
    pub Header: super::wdm::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub RequesterCapability: PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER,
    pub RequesterControl: PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_EXPRESS_TPH_REQUESTER_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
pub const PCI_EXPRESS_TPH_ST_LOCATION_MSIX_TABLE: u32 = 2;
pub const PCI_EXPRESS_TPH_ST_LOCATION_NONE: u32 = 0;
pub const PCI_EXPRESS_TPH_ST_LOCATION_RESERVED: u32 = 3;
pub const PCI_EXPRESS_TPH_ST_LOCATION_TPH_CAPABILITY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_TPH_ST_TABLE_ENTRY {
    pub Anonymous: PCI_EXPRESS_TPH_ST_TABLE_ENTRY_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_TPH_ST_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_TPH_ST_TABLE_ENTRY_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY {
    pub Header: super::wdm::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DvsecHeader1: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1,
    pub DvsecHeader2: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2,
    pub DvsecHeader3: PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_HEADER,
    pub PortSpecificAttributes: PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_HEADER {
    pub Anonymous: PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_HEADER_0,
    pub AsUSHORT: u16,
}
impl Default for PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_HEADER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES {
    pub Anonymous: PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0,
    pub AsULONG: u32,
}
impl Default for PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0 {
    pub type0: PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0_0,
    pub type1: PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0_1,
    pub type2: PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0_2,
}
impl Default for PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0_0 {
    pub _bitfield1: u16,
    pub _bitfield2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0_1 {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0_2 {
    pub _bitfield: u32,
}
pub type PCI_EXPRESS_WAKE_CONTROL = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, enablewake: bool)>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_FIRMWARE_BUS_CAPS {
    pub Type: u16,
    pub Length: u16,
    pub Anonymous: PCI_FIRMWARE_BUS_CAPS_0,
    pub CurrentSpeedAndMode: u8,
    pub SupportedSpeedsAndModesLowByte: u8,
    pub SupportedSpeedsAndModesHighByte: u8,
    pub Voltage: u8,
    pub Reserved2: [u8; 7],
}
impl Default for PCI_FIRMWARE_BUS_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_FIRMWARE_BUS_CAPS_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_FIRMWARE_BUS_CAPS_RETURN_BUFFER {
    pub Version: u16,
    pub Status: u16,
    pub Length: u32,
    pub Caps: PCI_FIRMWARE_BUS_CAPS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_FPB_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_FPB_CAPABILITIES_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_FPB_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct PCI_FPB_CAPABILITY {
    pub Header: PCI_FPB_CAPABILITY_HEADER,
    pub CapabilitiesRegister: PCI_FPB_CAPABILITIES_REGISTER,
    pub RidVectorControl1Register: PCI_FPB_RID_VECTOR_CONTROL1_REGISTER,
    pub RidVectorControl2Register: PCI_FPB_RID_VECTOR_CONTROL2_REGISTER,
    pub MemLowVectorControlRegister: PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER,
    pub MemHighVectorControl1Register: PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER,
    pub MemHighVectorControl2Register: PCI_FPB_MEM_HIGH_VECTOR_CONTROL2_REGISTER,
    pub VectorAccessControlRegister: PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER,
    pub VectorAccessDataRegister: PCI_FPB_VECTOR_ACCESS_DATA_REGISTER,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for PCI_FPB_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_CAPABILITY_HEADER {
    pub Header: super::wdm::PCI_CAPABILITIES_HEADER,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER {
    pub Anonymous: PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_MEM_HIGH_VECTOR_CONTROL2_REGISTER {
    pub MemHighVectorStartUpper: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER {
    pub Anonymous: PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_FPB_RID_VECTOR_CONTROL1_REGISTER {
    pub Anonymous: PCI_FPB_RID_VECTOR_CONTROL1_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_FPB_RID_VECTOR_CONTROL1_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_RID_VECTOR_CONTROL1_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_FPB_RID_VECTOR_CONTROL2_REGISTER {
    pub Anonymous: PCI_FPB_RID_VECTOR_CONTROL2_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_FPB_RID_VECTOR_CONTROL2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_RID_VECTOR_CONTROL2_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER {
    pub Anonymous: PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
impl Default for PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_VECTOR_ACCESS_DATA_REGISTER {
    pub VectorAccessData: u32,
}
#[cfg(feature = "Win32_bcrypt")]
pub type PCI_GET_LINK_INFORMATION = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, linkinformation: *mut PCI_LINK_INFORMATION) -> super::super::Win32::bcrypt::NTSTATUS>;
pub type PCI_HARDWARE_INTERFACE = i32;
pub const PCI_INVALID_ALTERNATE_FUNCTION_NUMBER: u32 = 255;
#[cfg(feature = "Wdk_wdm")]
pub type PCI_LINE_TO_PIN = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, pcinewdata: *const super::wdm::PCI_COMMON_CONFIG, pciolddata: *const super::wdm::PCI_COMMON_CONFIG)>;
#[repr(C)]
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt"))]
#[derive(Clone, Copy)]
pub struct PCI_LINK_CONFIG_INTERFACE_V1 {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: super::wdm::PINTERFACE_REFERENCE,
    pub InterfaceDereference: super::wdm::PINTERFACE_DEREFERENCE,
    pub SetMaximumLinkBandwidth: PPCI_SET_MAX_LINK_BANDWIDTH,
    pub GetLinkInformation: PPCI_GET_LINK_INFORMATION,
}
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt"))]
impl Default for PCI_LINK_CONFIG_INTERFACE_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_LINK_INFORMATION {
    pub AvailableLinkSpeeds: PCI_LINK_INFORMATION_0,
    pub LinkWidth: u16,
    pub CurrentLinkSpeed: PCI_LINK_SPEED,
}
impl Default for PCI_LINK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_LINK_INFORMATION_0 {
    pub Anonymous: PCI_LINK_INFORMATION_0_0,
    pub AsULONG: u32,
}
impl Default for PCI_LINK_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_LINK_INFORMATION_0_0 {
    pub _bitfield: u32,
}
pub type PCI_LINK_SPEED = i32;
pub type PCI_LINK_WIDTH = i32;
pub const PCI_LNKINTRF_VERSION: u32 = 1;
pub type PCI_OSC_CONTROL_BITS = i32;
#[cfg(feature = "Wdk_wdm")]
pub type PCI_PIN_TO_LINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, pcidata: *const super::wdm::PCI_COMMON_CONFIG)>;
pub type PCI_PREPARE_MULTISTAGE_RESUME = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type PCI_READ_WRITE_CONFIG = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, busoffset: u32, slot: u32, buffer: *const core::ffi::c_void, offset: u32, length: u32) -> u32>;
pub const PCI_RECOVERY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdd060800_f6e1_4204_ac27_c4bca9568402);
pub const PCI_RESERVED_VENDOR_ID: u32 = 1;
pub type PCI_ROOT_BUS_CAPABILITY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, hardwarecapability: *mut PCI_ROOT_BUS_HARDWARE_CAPABILITY)>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_ROOT_BUS_HARDWARE_CAPABILITY {
    pub SecondaryInterface: PCI_HARDWARE_INTERFACE,
    pub Anonymous: PCI_ROOT_BUS_HARDWARE_CAPABILITY_0,
    pub OscFeatureSupport: PCI_ROOT_BUS_OSC_SUPPORT_FIELD,
    pub OscControlRequest: PCI_ROOT_BUS_OSC_CONTROL_FIELD,
    pub OscControlGranted: PCI_ROOT_BUS_OSC_CONTROL_FIELD,
    pub CxlCapable: bool,
    pub CxlVersionSupport: CXL_PROTOCOL_VERSION,
    pub CxlOscFeatureSupport: CXL_BUS_OSC_SUPPORT_FIELD,
    pub CxlOscControlRequest: CXL_BUS_OSC_CONTROL_FIELD,
    pub CxlOscControlGranted: CXL_BUS_OSC_CONTROL_FIELD,
}
impl Default for PCI_ROOT_BUS_HARDWARE_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_ROOT_BUS_HARDWARE_CAPABILITY_0 {
    pub BusCapabilitiesFound: bool,
    pub CurrentSpeedAndMode: u32,
    pub SupportedSpeedsAndModes: u32,
    pub DeviceIDMessagingCapable: bool,
    pub SecondaryBusWidth: PCI_BUS_WIDTH,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_ROOT_BUS_OSC_CONTROL_FIELD {
    pub u: PCI_ROOT_BUS_OSC_CONTROL_FIELD_0,
}
impl Default for PCI_ROOT_BUS_OSC_CONTROL_FIELD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_ROOT_BUS_OSC_CONTROL_FIELD_0 {
    pub Anonymous: PCI_ROOT_BUS_OSC_CONTROL_FIELD_0_0,
    pub AsULONG: u32,
}
impl Default for PCI_ROOT_BUS_OSC_CONTROL_FIELD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_ROOT_BUS_OSC_CONTROL_FIELD_0_0 {
    pub _bitfield: u32,
}
pub const PCI_ROOT_BUS_OSC_METHOD_CAPABILITY_REVISION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_ROOT_BUS_OSC_SUPPORT_FIELD {
    pub u: PCI_ROOT_BUS_OSC_SUPPORT_FIELD_0,
}
impl Default for PCI_ROOT_BUS_OSC_SUPPORT_FIELD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_ROOT_BUS_OSC_SUPPORT_FIELD_0 {
    pub Anonymous: PCI_ROOT_BUS_OSC_SUPPORT_FIELD_0_0,
    pub AsULONG: u32,
}
impl Default for PCI_ROOT_BUS_OSC_SUPPORT_FIELD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PCI_ROOT_BUS_OSC_SUPPORT_FIELD_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "Win32_bcrypt")]
pub type PCI_SET_MAX_LINK_BANDWIDTH = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, targetbandwidth: u32, waitforretrain: bool, linkthrottled: *mut bool, newlinkspeed: *mut PCI_LINK_SPEED) -> super::super::Win32::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy, Default)]
pub struct PCI_SUBSYSTEM_IDS_CAPABILITY {
    pub Header: super::wdm::PCI_CAPABILITIES_HEADER,
    pub Reserved: u16,
    pub SubVendorID: u16,
    pub SubSystemID: u16,
}
#[repr(C)]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy, Default)]
pub struct PCI_VENDOR_SPECIFIC_CAPABILITY {
    pub Header: super::wdm::PCI_CAPABILITIES_HEADER,
    pub VscLength: u8,
    pub VendorSpecific: u8,
}
pub const PCIe_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcf93c01f_1a16_4dfc_b8bc_9c4daf67c104);
pub const PCMCIAConfiguration: BUS_DATA_TYPE = 7;
#[cfg(all(feature = "Wdk_mce", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_winnt"))]
pub type PCMC_DRIVER_INFO = *mut CMC_DRIVER_INFO;
pub type PCM_PCCARD_DEVICE_DATA = *mut CM_PCCARD_DEVICE_DATA;
pub type PCONFIGURATION_INFORMATION = *mut CONFIGURATION_INFORMATION;
pub type PCONFIGURATION_TYPE = *mut CONFIGURATION_TYPE;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
pub type PCONTROLLER_OBJECT = *mut CONTROLLER_OBJECT;
#[cfg(all(feature = "Wdk_mce", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_winnt"))]
pub type PCPE_DRIVER_INFO = *mut CPE_DRIVER_INFO;
#[cfg(feature = "Win32_winnt")]
pub type PCREATE_PROCESS_NOTIFY_ROUTINE = Option<unsafe extern "system" fn(parentid: super::super::Win32::winnt::HANDLE, processid: super::super::Win32::winnt::HANDLE, create: bool)>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PCREATE_PROCESS_NOTIFY_ROUTINE_EX = Option<unsafe extern "system" fn(process: *mut super::wdm::_KPROCESS, processid: super::super::Win32::winnt::HANDLE, createinfo: *mut PS_CREATE_NOTIFY_INFO)>;
#[cfg(feature = "Win32_winnt")]
pub type PCREATE_THREAD_NOTIFY_ROUTINE = Option<unsafe extern "system" fn(processid: super::super::Win32::winnt::HANDLE, threadid: super::super::Win32::winnt::HANDLE, create: bool)>;
#[cfg(feature = "Win32_winnt")]
pub type PCREATE_USER_PROCESS_ECP_CONTEXT = *mut CREATE_USER_PROCESS_ECP_CONTEXT;
pub const PCR_MAJOR_VERSION: u32 = 1;
pub const PCR_MINOR_VERSION: u32 = 1;
pub type PCXL_BUS_OSC_CONTROL_FIELD = *mut CXL_BUS_OSC_CONTROL_FIELD;
pub type PCXL_BUS_OSC_SUPPORT_FIELD = *mut CXL_BUS_OSC_SUPPORT_FIELD;
pub type PCXL_DVSEC_IDS = *mut CXL_DVSEC_IDS;
pub type PCXL_OSC_CONTROL_BITS = *mut CXL_OSC_CONTROL_BITS;
pub type PCXL_PROTOCOL_VERSION = *mut CXL_PROTOCOL_VERSION;
pub type PDEBUGGING_DEVICE_IN_USE = *mut DEBUGGING_DEVICE_IN_USE;
pub type PDEBUGGING_DEVICE_IN_USE_INFORMATION = *mut DEBUGGING_DEVICE_IN_USE_INFORMATION;
#[cfg(feature = "Win32_minwindef")]
pub type PDEBUG_DEVICE_ADDRESS = *mut DEBUG_DEVICE_ADDRESS;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_usb", feature = "Win32_winnt"))]
pub type PDEBUG_DEVICE_DESCRIPTOR = *mut DEBUG_DEVICE_DESCRIPTOR;
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_usb", feature = "Win32_winnt"))]
pub type PDEBUG_DEVICE_FOUND_FUNCTION = Option<unsafe extern "system" fn(device: *mut DEBUG_DEVICE_DESCRIPTOR) -> KD_CALLBACK_ACTION>;
pub type PDEBUG_EFI_IOMMU_DATA = *mut DEBUG_EFI_IOMMU_DATA;
#[cfg(feature = "Win32_usb")]
pub type PDEBUG_MEMORY_REQUIREMENTS = *mut DEBUG_MEMORY_REQUIREMENTS;
pub type PDEBUG_TRANSPORT_DATA = *mut DEBUG_TRANSPORT_DATA;
pub type PDEVICE_HANDLER_OBJECT = *mut _DEVICE_HANDLER_OBJECT;
pub type PDIMM_ADDRESS = *mut DIMM_ADDRESS;
pub type PDIMM_ADDR_VALID_BITS = *mut DIMM_ADDR_VALID_BITS;
pub type PDIMM_ADDR_VALID_BITS_DDR4 = *mut DIMM_ADDR_VALID_BITS_DDR4;
pub type PDIMM_ADDR_VALID_BITS_DDR5 = *mut DIMM_ADDR_VALID_BITS_DDR5;
pub type PDIMM_INFO = *mut DIMM_INFO;
pub type PDISK_SIGNATURE = *mut DISK_SIGNATURE;
pub type PDOE_DISCOVERY_REQUEST = *mut DOE_DISCOVERY_REQUEST;
pub type PDOE_DISCOVERY_RESPONSE = *mut DOE_DISCOVERY_RESPONSE;
pub type PDOE_HEADER_1 = *mut DOE_HEADER_1;
pub type PDOE_HEADER_2 = *mut DOE_HEADER_2;
pub type PDOE_OBJECT_HEADER = *mut DOE_OBJECT_HEADER;
#[cfg(feature = "Wdk_mce")]
pub type PDRIVER_CMC_EXCEPTION_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, cmclog: *const super::mce::MCA_EXCEPTION)>;
#[cfg(feature = "Wdk_mce")]
pub type PDRIVER_CPE_EXCEPTION_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, cmclog: *const super::mce::MCA_EXCEPTION)>;
#[cfg(all(feature = "Wdk_mce", feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
pub type PDRIVER_EXCPTN_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, trapframe: *const KTRAP_FRAME, exceptionframe: *const KEXCEPTION_FRAME, exception: *const super::mce::MCA_EXCEPTION) -> super::mce::ERROR_SEVERITY>;
#[cfg(all(feature = "Wdk_mce", feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
pub type PDRIVER_MCA_EXCEPTION_CALLBACK = PDRIVER_EXCPTN_CALLBACK;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PDRIVER_REINITIALIZE = *mut DRIVER_REINITIALIZE;
pub type PDRIVER_VERIFIER_THUNK_PAIRS = *mut DRIVER_VERIFIER_THUNK_PAIRS;
pub type PDRIVER_VERIFIER_THUNK_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> usize>;
pub type PEFI_ACPI_RAS_SIGNAL_TABLE = *mut EFI_ACPI_RAS_SIGNAL_TABLE;
pub const PEI_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x09a9d5ac_5204_4214_96e5_94992e752bcd);
pub type PEJOB = *mut _EJOB;
pub type PERROR_SOURCE_INFO = *mut ERROR_SOURCE_INFO;
pub type PESILO = *mut _EJOB;
pub type PEXPAND_STACK_CALLOUT = Option<unsafe extern "system" fn(parameter: *const core::ffi::c_void)>;
pub type PEXTENDED_AGP_REGISTER = *mut EXTENDED_AGP_REGISTER;
pub type PFILE_ALIGNMENT_INFORMATION = *mut FILE_ALIGNMENT_INFORMATION;
pub type PFILE_ATTRIBUTE_TAG_INFORMATION = *mut FILE_ATTRIBUTE_TAG_INFORMATION;
pub type PFILE_DISPOSITION_INFORMATION = *mut FILE_DISPOSITION_INFORMATION;
pub type PFILE_DISPOSITION_INFORMATION_EX = *mut FILE_DISPOSITION_INFORMATION_EX;
pub type PFILE_END_OF_FILE_INFORMATION = *mut FILE_END_OF_FILE_INFORMATION;
pub type PFILE_FS_FULL_SIZE_INFORMATION = *mut FILE_FS_FULL_SIZE_INFORMATION;
pub type PFILE_FS_FULL_SIZE_INFORMATION_EX = *mut FILE_FS_FULL_SIZE_INFORMATION_EX;
pub type PFILE_FS_LABEL_INFORMATION = *mut FILE_FS_LABEL_INFORMATION;
pub type PFILE_FS_METADATA_SIZE_INFORMATION = *mut FILE_FS_METADATA_SIZE_INFORMATION;
pub type PFILE_FS_OBJECTID_INFORMATION = *mut FILE_FS_OBJECTID_INFORMATION;
pub type PFILE_FS_SECTOR_SIZE_INFORMATION = *mut FILE_FS_SECTOR_SIZE_INFORMATION;
pub type PFILE_FS_SIZE_INFORMATION = *mut FILE_FS_SIZE_INFORMATION;
pub type PFILE_FS_VOLUME_INFORMATION = *mut FILE_FS_VOLUME_INFORMATION;
pub type PFILE_NAME_INFORMATION = *mut FILE_NAME_INFORMATION;
pub type PFILE_VALID_DATA_LENGTH_INFORMATION = *mut FILE_VALID_DATA_LENGTH_INFORMATION;
#[cfg(feature = "Win32_bcrypt")]
pub type PFNFTH = Option<unsafe extern "C" fn(systemfirmwaretableinfo: *mut SYSTEM_FIRMWARE_TABLE_INFORMATION) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt"))]
pub type PFN_IN_USE_PAGE_OFFLINE_NOTIFY = Option<unsafe extern "system" fn(page: super::wdm::PFN_NUMBER, poisoned: bool, context: *mut core::ffi::c_void, callbackstatus: *mut super::super::Win32::bcrypt::NTSTATUS) -> bool>;
#[cfg(feature = "Win32_bcrypt")]
pub type PFN_WHEA_HIGH_IRQL_LOG_SEL_EVENT_HANDLER = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, osselrecord: *const IPMI_OS_SEL_RECORD) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
pub type PGET_LOCATION_STRING = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, locationstrings: *mut super::super::Win32::winnt::PZZWSTR) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type PHALIOREADWRITEHANDLER = Option<unsafe extern "system" fn(fread: bool, dwaddr: u32, dwsize: u32, pdwdata: *mut u32) -> super::super::Win32::bcrypt::NTSTATUS>;
pub type PHALMCAINTERFACELOCK = Option<unsafe extern "system" fn()>;
#[cfg(feature = "Win32_bcrypt")]
pub type PHALMCAINTERFACEREADREGISTER = Option<unsafe extern "system" fn(banknumber: u8, exception: *mut core::ffi::c_void) -> super::super::Win32::bcrypt::NTSTATUS>;
pub type PHALMCAINTERFACEUNLOCK = Option<unsafe extern "system" fn()>;
#[cfg(feature = "Win32_bcrypt")]
pub type PHAL_AMLI_BAD_IO_ADDRESS_LIST = *mut HAL_AMLI_BAD_IO_ADDRESS_LIST;
pub type PHAL_APIC_DESTINATION_MODE = *mut HAL_APIC_DESTINATION_MODE;
#[cfg(feature = "Wdk_wdm")]
pub type PHAL_BUS_INFORMATION = *mut HAL_BUS_INFORMATION;
#[cfg(feature = "Wdk_wdm")]
pub type PHAL_CALLBACKS = *mut HAL_CALLBACKS;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PHAL_DISPATCH = *mut HAL_DISPATCH;
pub type PHAL_DISPLAY_BIOS_INFORMATION = *mut HAL_DISPLAY_BIOS_INFORMATION;
pub type PHAL_DMA_CRASH_DUMP_REGISTER_TYPE = *mut HAL_DMA_CRASH_DUMP_REGISTER_TYPE;
pub type PHAL_ERROR_INFO = *mut HAL_ERROR_INFO;
pub type PHAL_PLATFORM_INFORMATION = *mut HAL_PLATFORM_INFORMATION;
pub type PHAL_PMU_NOTIFICATION = *mut HAL_PMU_NOTIFICATION;
pub type PHAL_PMU_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, notification: *const HAL_PMU_NOTIFICATION)>;
pub type PHAL_POST_SLEEP_STATS = *mut HAL_POST_SLEEP_STATS;
pub type PHAL_POWER_INFORMATION = *mut HAL_POWER_INFORMATION;
pub type PHAL_PROCESSOR_SPEED_INFORMATION = *mut HAL_PROCESSOR_SPEED_INFORMATION;
pub type PHAL_QUERY_INFORMATION_CLASS = *mut HAL_QUERY_INFORMATION_CLASS;
pub type PHAL_REGISTER_PMU_NOTIFICATION_INPUT = *mut HAL_REGISTER_PMU_NOTIFICATION_INPUT;
pub type PHAL_RESET_DISPLAY_PARAMETERS = Option<unsafe extern "system" fn(columns: u32, rows: u32) -> bool>;
pub type PHAL_SET_INFORMATION_CLASS = *mut HAL_SET_INFORMATION_CLASS;
pub type PHAL_UNREGISTER_PMU_NOTIFICATION_INPUT = *mut HAL_UNREGISTER_PMU_NOTIFICATION_INPUT;
#[cfg(feature = "Win32_winnt")]
pub type PHARDWARE_COUNTER = *mut HARDWARE_COUNTER;
#[cfg(feature = "Win32_bcrypt")]
pub type PHVL_WHEA_ERROR_NOTIFICATION = *mut HVL_WHEA_ERROR_NOTIFICATION;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct PHYSICAL_COUNTER_EVENT_BUFFER_CONFIGURATION {
    pub OverflowHandler: PPHYSICAL_COUNTER_EVENT_BUFFER_OVERFLOW_HANDLER,
    pub CustomEventBufferEntrySize: u32,
    pub EventThreshold: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR {
    pub Type: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE,
    pub Flags: u32,
    pub u: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_0,
}
#[cfg(feature = "Win32_winnt")]
impl Default for PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_0 {
    pub CounterIndex: u32,
    pub Range: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_0_0,
    pub OverflowHandler: PPHYSICAL_COUNTER_OVERFLOW_HANDLER,
    pub EventBufferConfiguration: PHYSICAL_COUNTER_EVENT_BUFFER_CONFIGURATION,
    pub IdentificationTag: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_0_0 {
    pub Begin: u32,
    pub End: u32,
}
pub type PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct PHYSICAL_COUNTER_RESOURCE_LIST {
    pub Count: u32,
    pub Descriptors: [PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR; 1],
}
#[cfg(feature = "Win32_winnt")]
impl Default for PHYSICAL_COUNTER_RESOURCE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_usb")]
#[derive(Clone, Copy, Default)]
pub struct PHYSICAL_MEMORY_RANGE {
    pub BaseAddress: super::super::Win32::usb::PHYSICAL_ADDRESS,
    pub NumberOfBytes: i64,
}
#[cfg(feature = "Win32_filter")]
pub type PIMAGE_INFO = *mut super::super::Win32::filter::IMAGE_INFO;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_filter", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PIMAGE_INFO_EX = *mut IMAGE_INFO_EX;
#[cfg(feature = "Wdk_ntdef")]
pub type PIO_DRIVER_CREATE_CONTEXT = *mut IO_DRIVER_CREATE_CONTEXT;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PIO_FOEXT_SHADOW_FILE = *mut IO_FOEXT_SHADOW_FILE;
pub type PIO_FOEXT_SILO_PARAMETERS = *mut IO_FOEXT_SILO_PARAMETERS;
pub type PIO_QUERY_DEVICE_DATA_FORMAT = *mut IO_QUERY_DEVICE_DATA_FORMAT;
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
pub type PIO_QUERY_DEVICE_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, pathname: *const super::super::Win32::ntsecapi::UNICODE_STRING, bustype: super::wdm::INTERFACE_TYPE, busnumber: u32, businformation: *const super::wdm::PKEY_VALUE_FULL_INFORMATION, controllertype: CONFIGURATION_TYPE, controllernumber: u32, controllerinformation: *const super::wdm::PKEY_VALUE_FULL_INFORMATION, peripheraltype: CONFIGURATION_TYPE, peripheralnumber: u32, peripheralinformation: *const super::wdm::PKEY_VALUE_FULL_INFORMATION) -> super::super::Win32::bcrypt::NTSTATUS>;
pub type PIPMI_OS_SEL_RECORD = *mut IPMI_OS_SEL_RECORD;
pub type PIPMI_OS_SEL_RECORD_TYPE = *mut IPMI_OS_SEL_RECORD_TYPE;
pub type PKD_CALLBACK_ACTION = *mut KD_CALLBACK_ACTION;
pub type PKD_NAMESPACE_ENUM = *mut KD_NAMESPACE_ENUM;
pub type PKERNEL_USER_TIMES = *mut KERNEL_USER_TIMES;
#[cfg(feature = "Win32_winnt")]
pub type PKEXCEPTION_FRAME = *mut KEXCEPTION_FRAME;
pub type PKEY_CACHED_INFORMATION = *mut KEY_CACHED_INFORMATION;
pub type PKEY_LAYER_INFORMATION = *mut KEY_LAYER_INFORMATION;
pub type PKEY_NAME_INFORMATION = *mut KEY_NAME_INFORMATION;
pub type PKEY_VIRTUALIZATION_INFORMATION = *mut KEY_VIRTUALIZATION_INFORMATION;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
pub type PKFRED_TRAP_FRAME = *mut KFRED_TRAP_FRAME;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_excpt", feature = "Win32_winnt"))]
pub type PKPCR = *mut KPCR;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
pub type PKTRAP_FRAME = *mut KTRAP_FRAME;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
pub type PKUMS_CONTEXT_HEADER = *mut KUMS_CONTEXT_HEADER;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_winnt"))]
pub type PKUSER_SHARED_DATA = *mut KUSER_SHARED_DATA;
pub const PLATFORM_EXTENDED_RAS_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc6749ac0_16fb_4868_b976_65a61299834f);
pub type PLOADER_PARAMETER_BLOCK = *mut _LOADER_PARAMETER_BLOCK;
#[cfg(all(feature = "Win32_filter", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt"))]
pub type PLOAD_IMAGE_NOTIFY_ROUTINE = Option<unsafe extern "system" fn(fullimagename: *const super::super::Win32::ntsecapi::UNICODE_STRING, processid: super::super::Win32::winnt::HANDLE, imageinfo: *const super::super::Win32::filter::IMAGE_INFO)>;
pub type PMAP_REGISTER_ENTRY = *mut MAP_REGISTER_ENTRY;
#[cfg(all(feature = "Wdk_mce", feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_winnt"))]
pub type PMCA_DRIVER_INFO = *mut MCA_DRIVER_INFO;
pub type PMCG_CAP = *mut MCG_CAP;
pub type PMCG_STATUS = *mut MCG_STATUS;
pub type PMCI_STATUS = *mut MCI_STATUS;
pub type PMCI_STATUS_AMD_BITS = *mut MCI_STATUS_AMD_BITS;
pub type PMCI_STATUS_BITS_COMMON = *mut MCI_STATUS_BITS_COMMON;
pub type PMCI_STATUS_INTEL_BITS = *mut MCI_STATUS_INTEL_BITS;
pub type PMEMORY_DEFECT = *mut MEMORY_DEFECT;
pub type PMEMORY_DEFECT_FLAGS = *mut MEMORY_DEFECT_FLAGS;
pub const PMEM_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x81687003_dbfd_4728_9ffd_f0904f97597d);
#[cfg(feature = "Win32_usb")]
pub type PMMCOPY_ADDRESS = *mut MM_COPY_ADDRESS;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_wdm", feature = "Win32_bcrypt"))]
pub type PMM_ROTATE_COPY_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(destinationmdl: *const super::wdm::MDL, sourcemdl: *const super::wdm::MDL, context: *const core::ffi::c_void) -> super::super::Win32::bcrypt::NTSTATUS>;
pub type PMM_ROTATE_DIRECTION = *mut MM_ROTATE_DIRECTION;
pub type PMU_TELEMETRY_SECTION = *mut MU_TELEMETRY_SECTION;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PM_DISPATCH_TABLE {
    pub Signature: u32,
    pub Version: u32,
    pub Function: [*mut core::ffi::c_void; 1],
}
impl Default for PM_DISPATCH_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PNPISAConfiguration: BUS_DATA_TYPE = 10;
#[repr(C)]
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct PNP_LOCATION_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: super::wdm::PINTERFACE_REFERENCE,
    pub InterfaceDereference: super::wdm::PINTERFACE_DEREFERENCE,
    pub GetLocationString: PGET_LOCATION_STRING,
}
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt", feature = "Win32_winnt"))]
impl Default for PNP_LOCATION_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POOLED_USAGE_AND_LIMITS {
    pub PeakPagedPoolUsage: usize,
    pub PagedPoolUsage: usize,
    pub PagedPoolLimit: usize,
    pub PeakNonPagedPoolUsage: usize,
    pub NonPagedPoolUsage: usize,
    pub NonPagedPoolLimit: usize,
    pub PeakPagefileUsage: usize,
    pub PagefileUsage: usize,
    pub PagefileLimit: usize,
}
pub type POPLOCK_KEY_CONTEXT = *mut OPLOCK_KEY_CONTEXT;
#[cfg(feature = "Wdk_ntifs")]
pub type POPLOCK_KEY_ECP_CONTEXT = *mut super::ntifs::OPLOCK_KEY_ECP_CONTEXT;
pub const POWER_THROTTLING_PROCESS_CURRENT_VERSION: u32 = 1;
pub const POWER_THROTTLING_PROCESS_DELAYTIMERS: u32 = 2;
pub const POWER_THROTTLING_PROCESS_EXECUTION_SPEED: u32 = 1;
pub const POWER_THROTTLING_PROCESS_IGNORE_TIMER_RESOLUTION: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POWER_THROTTLING_PROCESS_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
pub const POWER_THROTTLING_PROCESS_VALID_FLAGS: u32 = 7;
pub const POWER_THROTTLING_THREAD_CURRENT_VERSION: u32 = 1;
pub const POWER_THROTTLING_THREAD_EXECUTION_SPEED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct POWER_THROTTLING_THREAD_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
pub const POWER_THROTTLING_THREAD_VALID_FLAGS: u32 = 1;
pub type PPAGE_OFFLINE_ERROR_TYPES = *mut PAGE_OFFLINE_ERROR_TYPES;
pub type PPAGE_OFFLINE_VALID_BITS = *mut PAGE_OFFLINE_VALID_BITS;
pub type PPAGE_PRIORITY_INFORMATION = *mut PAGE_PRIORITY_INFORMATION;
#[cfg(feature = "Wdk_wdm")]
pub type PPCIBUSDATA = *mut PCIBUSDATA;
#[cfg(feature = "Wdk_wdm")]
pub type PPCIX_BRIDGE_CAPABILITY = *mut PCIX_BRIDGE_CAPABILITY;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_ADVANCED_FEATURES_CAPABILITY = *mut PCI_ADVANCED_FEATURES_CAPABILITY;
pub type PPCI_AGP_APERTURE_PAGE_SIZE = *mut PCI_AGP_APERTURE_PAGE_SIZE;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_AGP_CAPABILITY = *mut PCI_AGP_CAPABILITY;
pub type PPCI_AGP_CONTROL = *mut PCI_AGP_CONTROL;
pub type PPCI_AGP_EXTENDED_CAPABILITY = *mut PCI_AGP_EXTENDED_CAPABILITY;
pub type PPCI_AGP_ISOCH_COMMAND = *mut PCI_AGP_ISOCH_COMMAND;
pub type PPCI_AGP_ISOCH_STATUS = *mut PCI_AGP_ISOCH_STATUS;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_BUS_INTERFACE_STANDARD = *mut PCI_BUS_INTERFACE_STANDARD;
pub type PPCI_DEBUGGING_DEVICE_IN_USE = *mut PCI_DEBUGGING_DEVICE_IN_USE;
pub type PPCI_EXPRESS_CAPABILITIES_REGISTER = *mut PCI_EXPRESS_CAPABILITIES_REGISTER;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_EXPRESS_CAPABILITY = *mut PCI_EXPRESS_CAPABILITY;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_EXPRESS_CXL_DVSEC_CAPABILITY = *mut PCI_EXPRESS_CXL_DVSEC_CAPABILITY;
pub type PPCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11 = *mut PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11;
pub type PPCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V2 = *mut PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V2;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_EXPRESS_CXL_DVSEC_CAPABILITY_V11 = *mut PCI_EXPRESS_CXL_DVSEC_CAPABILITY_V11;
pub type PPCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER = *mut PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER;
pub type PPCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER = *mut PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER;
pub type PPCI_EXPRESS_CXL_DVSEC_RANGE_BASE_HIGH_REGISTER = *mut PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_HIGH_REGISTER;
pub type PPCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER = *mut PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER;
pub type PPCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_HIGH_REGISTER = *mut PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_HIGH_REGISTER;
pub type PPCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11 = *mut PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11;
pub type PPCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER = *mut PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_EXPRESS_DATA_OBJECT_EXCHANGE_EXTENDED_CAPABILITY = *mut PCI_EXPRESS_DATA_OBJECT_EXCHANGE_EXTENDED_CAPABILITY;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY = *mut PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY;
pub type PPCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1 = *mut PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1;
pub type PPCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2 = *mut PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2;
pub type PPCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER = *mut PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER;
pub type PPCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER = *mut PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER;
pub type PPCI_EXPRESS_DEVICE_CONTROL_2_REGISTER = *mut PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER;
pub type PPCI_EXPRESS_DEVICE_CONTROL_REGISTER = *mut PCI_EXPRESS_DEVICE_CONTROL_REGISTER;
pub type PPCI_EXPRESS_DEVICE_STATUS_2_REGISTER = *mut PCI_EXPRESS_DEVICE_STATUS_2_REGISTER;
pub type PPCI_EXPRESS_DEVICE_STATUS_REGISTER = *mut PCI_EXPRESS_DEVICE_STATUS_REGISTER;
pub type PPCI_EXPRESS_DOE_CAPABILITIES_REGISTER = *mut PCI_EXPRESS_DOE_CAPABILITIES_REGISTER;
pub type PPCI_EXPRESS_DOE_CONTROL_REGISTER = *mut PCI_EXPRESS_DOE_CONTROL_REGISTER;
pub type PPCI_EXPRESS_DOE_READ_MAILBOX_REGISTER = *mut PCI_EXPRESS_DOE_READ_MAILBOX_REGISTER;
pub type PPCI_EXPRESS_DOE_STATUS_REGISTER = *mut PCI_EXPRESS_DOE_STATUS_REGISTER;
pub type PPCI_EXPRESS_DOE_WRITE_MAILBOX_REGISTER = *mut PCI_EXPRESS_DOE_WRITE_MAILBOX_REGISTER;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_EXPRESS_DPC_CAPABILITY = *mut PCI_EXPRESS_DPC_CAPABILITY;
pub type PPCI_EXPRESS_DPC_CAPS_REGISTER = *mut PCI_EXPRESS_DPC_CAPS_REGISTER;
pub type PPCI_EXPRESS_DPC_CONTROL_REGISTER = *mut PCI_EXPRESS_DPC_CONTROL_REGISTER;
pub type PPCI_EXPRESS_DPC_ERROR_SOURCE_ID = *mut PCI_EXPRESS_DPC_ERROR_SOURCE_ID;
pub type PPCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER;
pub type PPCI_EXPRESS_DPC_RP_PIO_HEADERLOG_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_HEADERLOG_REGISTER;
pub type PPCI_EXPRESS_DPC_RP_PIO_IMPSPECLOG_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_IMPSPECLOG_REGISTER;
pub type PPCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER;
pub type PPCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER;
pub type PPCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER;
pub type PPCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER;
pub type PPCI_EXPRESS_DPC_RP_PIO_TLPPREFIXLOG_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_TLPPREFIXLOG_REGISTER;
pub type PPCI_EXPRESS_DPC_STATUS_REGISTER = *mut PCI_EXPRESS_DPC_STATUS_REGISTER;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_EXPRESS_EVENT_COLLECTOR_ENDPOINT_ASSOCIATION_CAPABILITY = *mut PCI_EXPRESS_EVENT_COLLECTOR_ENDPOINT_ASSOCIATION_CAPABILITY;
pub type PPCI_EXPRESS_IDE_ADDRESS_ASSOCIATION_BLOCK = *mut PCI_EXPRESS_IDE_ADDRESS_ASSOCIATION_BLOCK;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_EXPRESS_IDE_CAPABILITY = *mut PCI_EXPRESS_IDE_CAPABILITY;
pub type PPCI_EXPRESS_IDE_CAPABILITY_REGISTER = *mut PCI_EXPRESS_IDE_CAPABILITY_REGISTER;
pub type PPCI_EXPRESS_IDE_CONTROL_REGISTER = *mut PCI_EXPRESS_IDE_CONTROL_REGISTER;
pub type PPCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_1 = *mut PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_1;
pub type PPCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_2 = *mut PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_2;
pub type PPCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER = *mut PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_EXPRESS_L1_PM_SS_CAPABILITY = *mut PCI_EXPRESS_L1_PM_SS_CAPABILITY;
pub type PPCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER = *mut PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER;
pub type PPCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER = *mut PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER;
pub type PPCI_EXPRESS_LANE_ERROR_STATUS = *mut PCI_EXPRESS_LANE_ERROR_STATUS;
pub type PPCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER = *mut PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER;
pub type PPCI_EXPRESS_LINK_CAPABILITIES_REGISTER = *mut PCI_EXPRESS_LINK_CAPABILITIES_REGISTER;
pub type PPCI_EXPRESS_LINK_CONTROL3 = *mut PCI_EXPRESS_LINK_CONTROL3;
pub type PPCI_EXPRESS_LINK_CONTROL_2_REGISTER = *mut PCI_EXPRESS_LINK_CONTROL_2_REGISTER;
pub type PPCI_EXPRESS_LINK_CONTROL_REGISTER = *mut PCI_EXPRESS_LINK_CONTROL_REGISTER;
pub type PPCI_EXPRESS_LINK_IDE_BLOCK = *mut PCI_EXPRESS_LINK_IDE_BLOCK;
pub type PPCI_EXPRESS_LINK_IDE_STREAM_CONTROL_REGISTER = *mut PCI_EXPRESS_LINK_IDE_STREAM_CONTROL_REGISTER;
pub type PPCI_EXPRESS_LINK_IDE_STREAM_STATUS_REGISTER = *mut PCI_EXPRESS_LINK_IDE_STREAM_STATUS_REGISTER;
pub type PPCI_EXPRESS_LINK_STATUS_2_REGISTER = *mut PCI_EXPRESS_LINK_STATUS_2_REGISTER;
pub type PPCI_EXPRESS_LINK_STATUS_REGISTER = *mut PCI_EXPRESS_LINK_STATUS_REGISTER;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_EXPRESS_LTR_CAPABILITY = *mut PCI_EXPRESS_LTR_CAPABILITY;
pub type PPCI_EXPRESS_LTR_MAX_LATENCY_REGISTER = *mut PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_EXPRESS_NPEM_CAPABILITY = *mut PCI_EXPRESS_NPEM_CAPABILITY;
pub type PPCI_EXPRESS_NPEM_CAPABILITY_REGISTER = *mut PCI_EXPRESS_NPEM_CAPABILITY_REGISTER;
pub type PPCI_EXPRESS_NPEM_CONTROL_REGISTER = *mut PCI_EXPRESS_NPEM_CONTROL_REGISTER;
pub type PPCI_EXPRESS_NPEM_STATUS_REGISTER = *mut PCI_EXPRESS_NPEM_STATUS_REGISTER;
pub type PPCI_EXPRESS_PME_REQUESTOR_ID = *mut PCI_EXPRESS_PME_REQUESTOR_ID;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_EXPRESS_RESIZABLE_BAR_CAPABILITY = *mut PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY;
pub type PPCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER = *mut PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER;
pub type PPCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER = *mut PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER;
pub type PPCI_EXPRESS_RESIZABLE_BAR_ENTRY = *mut PCI_EXPRESS_RESIZABLE_BAR_ENTRY;
pub type PPCI_EXPRESS_ROOT_CAPABILITIES_REGISTER = *mut PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER;
pub type PPCI_EXPRESS_ROOT_CONTROL_REGISTER = *mut PCI_EXPRESS_ROOT_CONTROL_REGISTER;
pub type PPCI_EXPRESS_ROOT_STATUS_REGISTER = *mut PCI_EXPRESS_ROOT_STATUS_REGISTER;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_EXPRESS_SECONDARY_CAPABILITY = *mut PCI_EXPRESS_SECONDARY_CAPABILITY;
pub type PPCI_EXPRESS_SELECTIVE_IDE_BLOCK = *mut PCI_EXPRESS_SELECTIVE_IDE_BLOCK;
pub type PPCI_EXPRESS_SELECTIVE_IDE_CAPABILITY_REGISTER = *mut PCI_EXPRESS_SELECTIVE_IDE_CAPABILITY_REGISTER;
pub type PPCI_EXPRESS_SELECTIVE_IDE_CONTROL_REGISTER = *mut PCI_EXPRESS_SELECTIVE_IDE_CONTROL_REGISTER;
pub type PPCI_EXPRESS_SELECTIVE_IDE_STATUS_REGISTER = *mut PCI_EXPRESS_SELECTIVE_IDE_STATUS_REGISTER;
pub type PPCI_EXPRESS_SLOT_CAPABILITIES_REGISTER = *mut PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER;
pub type PPCI_EXPRESS_SLOT_CONTROL_REGISTER = *mut PCI_EXPRESS_SLOT_CONTROL_REGISTER;
pub type PPCI_EXPRESS_SLOT_STATUS_REGISTER = *mut PCI_EXPRESS_SLOT_STATUS_REGISTER;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_EXPRESS_TPH_REQUESTER_CAPABILITY = *mut PCI_EXPRESS_TPH_REQUESTER_CAPABILITY;
pub type PPCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER = *mut PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER;
pub type PPCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER = *mut PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER;
pub type PPCI_EXPRESS_TPH_ST_TABLE_ENTRY = *mut PCI_EXPRESS_TPH_ST_TABLE_ENTRY;
pub type PPCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_HEADER = *mut PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_HEADER;
pub type PPCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES = *mut PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES;
pub type PPCI_EXPRESS_WAKE_CONTROL = *mut PCI_EXPRESS_WAKE_CONTROL;
pub type PPCI_FIRMWARE_BUS_CAPS = *mut PCI_FIRMWARE_BUS_CAPS;
pub type PPCI_FIRMWARE_BUS_CAPS_RETURN_BUFFER = *mut PCI_FIRMWARE_BUS_CAPS_RETURN_BUFFER;
pub type PPCI_FPB_CAPABILITIES_REGISTER = *mut PCI_FPB_CAPABILITIES_REGISTER;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_FPB_CAPABILITY = *mut PCI_FPB_CAPABILITY;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_FPB_CAPABILITY_HEADER = *mut PCI_FPB_CAPABILITY_HEADER;
pub type PPCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER = *mut PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER;
pub type PPCI_FPB_MEM_HIGH_VECTOR_CONTROL2_REGISTER = *mut PCI_FPB_MEM_HIGH_VECTOR_CONTROL2_REGISTER;
pub type PPCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER = *mut PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER;
pub type PPCI_FPB_RID_VECTOR_CONTROL1_REGISTER = *mut PCI_FPB_RID_VECTOR_CONTROL1_REGISTER;
pub type PPCI_FPB_RID_VECTOR_CONTROL2_REGISTER = *mut PCI_FPB_RID_VECTOR_CONTROL2_REGISTER;
pub type PPCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER = *mut PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER;
pub type PPCI_FPB_VECTOR_ACCESS_DATA_REGISTER = *mut PCI_FPB_VECTOR_ACCESS_DATA_REGISTER;
#[cfg(feature = "Win32_bcrypt")]
pub type PPCI_GET_LINK_INFORMATION = *mut PCI_GET_LINK_INFORMATION;
pub type PPCI_HARDWARE_INTERFACE = *mut PCI_HARDWARE_INTERFACE;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_LINE_TO_PIN = *mut PCI_LINE_TO_PIN;
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt"))]
pub type PPCI_LINK_CONFIG_INTERFACE_V1 = *mut PCI_LINK_CONFIG_INTERFACE_V1;
pub type PPCI_LINK_INFORMATION = *mut PCI_LINK_INFORMATION;
pub type PPCI_LINK_SPEED = *mut PCI_LINK_SPEED;
pub type PPCI_LINK_WIDTH = *mut PCI_LINK_WIDTH;
pub type PPCI_OSC_CONTROL_BITS = *mut PCI_OSC_CONTROL_BITS;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_PIN_TO_LINE = *mut PCI_PIN_TO_LINE;
pub type PPCI_PREPARE_MULTISTAGE_RESUME = *mut PCI_PREPARE_MULTISTAGE_RESUME;
pub type PPCI_READ_WRITE_CONFIG = *mut PCI_READ_WRITE_CONFIG;
pub type PPCI_ROOT_BUS_CAPABILITY = *mut PCI_ROOT_BUS_CAPABILITY;
pub type PPCI_ROOT_BUS_HARDWARE_CAPABILITY = *mut PCI_ROOT_BUS_HARDWARE_CAPABILITY;
pub type PPCI_ROOT_BUS_OSC_CONTROL_FIELD = *mut PCI_ROOT_BUS_OSC_CONTROL_FIELD;
pub type PPCI_ROOT_BUS_OSC_SUPPORT_FIELD = *mut PCI_ROOT_BUS_OSC_SUPPORT_FIELD;
#[cfg(feature = "Win32_bcrypt")]
pub type PPCI_SET_MAX_LINK_BANDWIDTH = *mut PCI_SET_MAX_LINK_BANDWIDTH;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_SUBSYSTEM_IDS_CAPABILITY = *mut PCI_SUBSYSTEM_IDS_CAPABILITY;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_USB4_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY = *mut PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY;
#[cfg(feature = "Wdk_wdm")]
pub type PPCI_VENDOR_SPECIFIC_CAPABILITY = *mut PCI_VENDOR_SPECIFIC_CAPABILITY;
#[cfg(feature = "Win32_winnt")]
pub type PPHYSICAL_COUNTER_EVENT_BUFFER_CONFIGURATION = *mut PHYSICAL_COUNTER_EVENT_BUFFER_CONFIGURATION;
#[cfg(feature = "Win32_winnt")]
pub type PPHYSICAL_COUNTER_EVENT_BUFFER_OVERFLOW_HANDLER = Option<unsafe extern "system" fn(eventbuffer: *const core::ffi::c_void, entrysize: usize, numberofentries: usize, owninghandle: super::super::Win32::winnt::HANDLE)>;
#[cfg(feature = "Win32_winnt")]
pub type PPHYSICAL_COUNTER_OVERFLOW_HANDLER = Option<unsafe extern "system" fn(overflowbits: u64, owninghandle: super::super::Win32::winnt::HANDLE)>;
#[cfg(feature = "Win32_winnt")]
pub type PPHYSICAL_COUNTER_RESOURCE_DESCRIPTOR = *mut PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR;
#[cfg(feature = "Win32_winnt")]
pub type PPHYSICAL_COUNTER_RESOURCE_LIST = *mut PHYSICAL_COUNTER_RESOURCE_LIST;
#[cfg(feature = "Win32_usb")]
pub type PPHYSICAL_MEMORY_RANGE = *mut PHYSICAL_MEMORY_RANGE;
pub type PPM_DISPATCH_TABLE = *mut PM_DISPATCH_TABLE;
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt", feature = "Win32_winnt"))]
pub type PPNP_LOCATION_INTERFACE = *mut PNP_LOCATION_INTERFACE;
pub type PPOOLED_USAGE_AND_LIMITS = *mut POOLED_USAGE_AND_LIMITS;
pub type PPOWER_THROTTLING_PROCESS_STATE = *mut POWER_THROTTLING_PROCESS_STATE;
pub type PPOWER_THROTTLING_THREAD_STATE = *mut POWER_THROTTLING_THREAD_STATE;
#[cfg(feature = "Win32_winnt")]
pub type PPROCESS_ACCESS_TOKEN = *mut PROCESS_ACCESS_TOKEN;
#[cfg(feature = "Win32_winnt")]
pub type PPROCESS_DEVICEMAP_INFORMATION = *mut PROCESS_DEVICEMAP_INFORMATION;
#[cfg(feature = "Win32_winnt")]
pub type PPROCESS_DEVICEMAP_INFORMATION_EX = *mut PROCESS_DEVICEMAP_INFORMATION_EX;
#[cfg(feature = "Win32_winnt")]
pub type PPROCESS_EXCEPTION_PORT = *mut PROCESS_EXCEPTION_PORT;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PPROCESS_EXTENDED_BASIC_INFORMATION = *mut PROCESS_EXTENDED_BASIC_INFORMATION;
pub type PPROCESS_HANDLE_TRACING_ENABLE = *mut PROCESS_HANDLE_TRACING_ENABLE;
pub type PPROCESS_HANDLE_TRACING_ENABLE_EX = *mut PROCESS_HANDLE_TRACING_ENABLE_EX;
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PPROCESS_HANDLE_TRACING_ENTRY = *mut PROCESS_HANDLE_TRACING_ENTRY;
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PPROCESS_HANDLE_TRACING_QUERY = *mut PROCESS_HANDLE_TRACING_QUERY;
pub type PPROCESS_KEEPALIVE_COUNT_INFORMATION = *mut PROCESS_KEEPALIVE_COUNT_INFORMATION;
pub type PPROCESS_MEMBERSHIP_INFORMATION = *mut PROCESS_MEMBERSHIP_INFORMATION;
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
pub type PPROCESS_REVOKE_FILE_HANDLES_INFORMATION = *mut PROCESS_REVOKE_FILE_HANDLES_INFORMATION;
pub type PPROCESS_SESSION_INFORMATION = *mut PROCESS_SESSION_INFORMATION;
pub type PPROCESS_SYSCALL_PROVIDER_INFORMATION = *mut PROCESS_SYSCALL_PROVIDER_INFORMATION;
pub type PPROCESS_WS_WATCH_INFORMATION = *mut PROCESS_WS_WATCH_INFORMATION;
pub type PPSHED_MEMORY_DETAILS = *mut PSHED_MEMORY_DETAILS;
pub type PPSHED_MEMORY_DETAILS_VALID_BITS = *mut PSHED_MEMORY_DETAILS_VALID_BITS;
pub type PPSHED_PI_ERR_READING_PCIE_OVERRIDES = *mut PSHED_PI_ERR_READING_PCIE_OVERRIDES;
pub type PPSHED_PI_VENDOR_DEFINED_ACTION = *mut PSHED_PI_VENDOR_DEFINED_ACTION;
pub type PPSHED_PLATFORM_DETAILS = *mut PSHED_PLATFORM_DETAILS;
pub type PPSHED_PLATFORM_DETAILS_VALID_BITS = *mut PSHED_PLATFORM_DETAILS_VALID_BITS;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PPS_CREATE_NOTIFY_INFO = *mut PS_CREATE_NOTIFY_INFO;
pub const PROCESSOR_GENERIC_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9876ccad_47b4_4bdb_b65e_16f193c4f3db);
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct PROCESS_ACCESS_TOKEN {
    pub Token: super::super::Win32::winnt::HANDLE,
    pub Thread: super::super::Win32::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for PROCESS_ACCESS_TOKEN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION {
    pub Anonymous: PROCESS_DEVICEMAP_INFORMATION_0,
}
#[cfg(feature = "Win32_winnt")]
impl Default for PROCESS_DEVICEMAP_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union PROCESS_DEVICEMAP_INFORMATION_0 {
    pub Set: PROCESS_DEVICEMAP_INFORMATION_0_0,
    pub Query: PROCESS_DEVICEMAP_INFORMATION_0_1,
}
#[cfg(feature = "Win32_winnt")]
impl Default for PROCESS_DEVICEMAP_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION_0_0 {
    pub DirectoryHandle: super::super::Win32::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for PROCESS_DEVICEMAP_INFORMATION_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION_0_1 {
    pub DriveMap: u32,
    pub DriveType: [u8; 32],
}
#[cfg(feature = "Win32_winnt")]
impl Default for PROCESS_DEVICEMAP_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION_EX {
    pub Anonymous: PROCESS_DEVICEMAP_INFORMATION_EX_0,
    pub Flags: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for PROCESS_DEVICEMAP_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union PROCESS_DEVICEMAP_INFORMATION_EX_0 {
    pub Set: PROCESS_DEVICEMAP_INFORMATION_EX_0_0,
    pub Query: PROCESS_DEVICEMAP_INFORMATION_EX_0_1,
}
#[cfg(feature = "Win32_winnt")]
impl Default for PROCESS_DEVICEMAP_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION_EX_0_0 {
    pub DirectoryHandle: super::super::Win32::winnt::HANDLE,
}
#[cfg(feature = "Win32_winnt")]
impl Default for PROCESS_DEVICEMAP_INFORMATION_EX_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION_EX_0_1 {
    pub DriveMap: u32,
    pub DriveType: [u8; 32],
}
#[cfg(feature = "Win32_winnt")]
impl Default for PROCESS_DEVICEMAP_INFORMATION_EX_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct PROCESS_EXCEPTION_PORT {
    pub ExceptionPortHandle: super::super::Win32::winnt::HANDLE,
    pub StateFlags: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for PROCESS_EXCEPTION_PORT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROCESS_EXCEPTION_PORT_ALL_STATE_BITS: u32 = 3;
pub const PROCESS_EXCEPTION_PORT_ALL_STATE_FLAGS: u32 = 7;
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub struct PROCESS_EXTENDED_BASIC_INFORMATION {
    pub Size: usize,
    pub BasicInfo: super::super::Win32::winternl::PROCESS_BASIC_INFORMATION,
    pub Anonymous: PROCESS_EXTENDED_BASIC_INFORMATION_0,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for PROCESS_EXTENDED_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub union PROCESS_EXTENDED_BASIC_INFORMATION_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_EXTENDED_BASIC_INFORMATION_0_0,
}
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for PROCESS_EXTENDED_BASIC_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_EXTENDED_BASIC_INFORMATION_0_0 {
    pub _bitfield: u32,
}
pub const PROCESS_HANDLE_EXCEPTIONS_ENABLED: u32 = 1;
pub const PROCESS_HANDLE_RAISE_UM_EXCEPTION_ON_INVALID_HANDLE_CLOSE_DISABLED: u32 = 0;
pub const PROCESS_HANDLE_RAISE_UM_EXCEPTION_ON_INVALID_HANDLE_CLOSE_ENABLED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_HANDLE_TRACING_ENABLE {
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_HANDLE_TRACING_ENABLE_EX {
    pub Flags: u32,
    pub TotalSlots: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub struct PROCESS_HANDLE_TRACING_ENTRY {
    pub Handle: super::super::Win32::winnt::HANDLE,
    pub ClientId: super::super::Win32::winternl::CLIENT_ID,
    pub Type: u32,
    pub Stacks: [*mut core::ffi::c_void; 16],
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for PROCESS_HANDLE_TRACING_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROCESS_HANDLE_TRACING_MAX_STACKS: u32 = 16;
#[repr(C)]
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub struct PROCESS_HANDLE_TRACING_QUERY {
    pub Handle: super::super::Win32::winnt::HANDLE,
    pub TotalTraces: u32,
    pub HandleTrace: [PROCESS_HANDLE_TRACING_ENTRY; 1],
}
#[cfg(all(feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for PROCESS_HANDLE_TRACING_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_KEEPALIVE_COUNT_INFORMATION {
    pub WakeCount: u32,
    pub NoWakeCount: u32,
}
pub const PROCESS_LUID_DOSDEVICES_ONLY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_MEMBERSHIP_INFORMATION {
    pub ServerSiloId: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_REVOKE_FILE_HANDLES_INFORMATION {
    pub TargetDevicePath: super::super::Win32::ntsecapi::UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_SESSION_INFORMATION {
    pub SessionId: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_SYSCALL_PROVIDER_INFORMATION {
    pub ProviderId: windows_sys::core::GUID,
    pub Level: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_WS_WATCH_INFORMATION {
    pub FaultingPc: *mut core::ffi::c_void,
    pub FaultingVa: *mut core::ffi::c_void,
}
impl Default for PROCESS_WS_WATCH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROTECTED_POOL: u32 = 0;
#[cfg(feature = "Wdk_ntdef")]
pub type PRTL_AVL_ALLOCATE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, bytesize: super::ntdef::CLONG) -> *mut core::ffi::c_void>;
#[cfg(feature = "Wdk_ntdef")]
pub type PRTL_AVL_COMPARE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, firststruct: *const core::ffi::c_void, secondstruct: *const core::ffi::c_void) -> RTL_GENERIC_COMPARE_RESULTS>;
#[cfg(feature = "Wdk_ntdef")]
pub type PRTL_AVL_FREE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, buffer: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_bcrypt"))]
pub type PRTL_AVL_MATCH_FUNCTION = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, userdata: *const core::ffi::c_void, matchdata: *const core::ffi::c_void) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Wdk_ntdef")]
pub type PRTL_AVL_TABLE = *mut RTL_AVL_TABLE;
pub type PRTL_BALANCED_LINKS = *mut RTL_BALANCED_LINKS;
pub type PRTL_DYNAMIC_HASH_TABLE = *mut RTL_DYNAMIC_HASH_TABLE;
#[cfg(feature = "Win32_winnt")]
pub type PRTL_DYNAMIC_HASH_TABLE_CONTEXT = *mut RTL_DYNAMIC_HASH_TABLE_CONTEXT;
#[cfg(feature = "Win32_winnt")]
pub type PRTL_DYNAMIC_HASH_TABLE_ENTRY = *mut RTL_DYNAMIC_HASH_TABLE_ENTRY;
#[cfg(feature = "Win32_winnt")]
pub type PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR = *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR;
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
pub type PRTL_GENERIC_ALLOCATE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, bytesize: super::ntdef::CLONG) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
pub type PRTL_GENERIC_COMPARE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, firststruct: *const core::ffi::c_void, secondstruct: *const core::ffi::c_void) -> RTL_GENERIC_COMPARE_RESULTS>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
pub type PRTL_GENERIC_FREE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, buffer: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
pub type PRTL_GENERIC_TABLE = *mut RTL_GENERIC_TABLE;
#[cfg(feature = "Win32_winnt")]
pub type PRTL_RUN_ONCE_INIT_FN = Option<unsafe extern "system" fn(runonce: *mut super::super::Win32::winnt::RTL_RUN_ONCE, parameter: *mut core::ffi::c_void, context: *mut *mut core::ffi::c_void) -> u32>;
pub type PRTL_SPLAY_LINKS = *mut RTL_SPLAY_LINKS;
pub type PSCREATEPROCESSNOTIFYTYPE = i32;
pub type PSCREATETHREADNOTIFYTYPE = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct PSHED_MEMORY_DETAILS {
    pub Version: u16,
    pub Vb: PSHED_MEMORY_DETAILS_VALID_BITS,
    pub DdrVersion: u16,
    pub IsClosedPaged: bool,
    pub ColsPerRow: u16,
    pub PagesPerRow: u16,
    pub SocketCnt: u8,
    pub ChaOnSktCnt: u8,
    pub DimmSlotCnt: u8,
    pub SubchannelCnt: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct PSHED_MEMORY_DETAILS_VALID_BITS {
    pub _bitfield: u32,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
pub type PSHED_PI_ATTEMPT_ERROR_RECOVERY = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, bufferlength: u32, errorrecord: *const WHEA_ERROR_RECORD) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type PSHED_PI_CLEAR_ERROR_RECORD = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, flags: u32, errorrecordid: u64) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PSHED_PI_CLEAR_ERROR_STATUS = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errorsource: *const WHEA_ERROR_SOURCE_DESCRIPTOR, bufferlength: u32, errorrecord: *const WHEA_ERROR_RECORD) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type PSHED_PI_DISABLE_ERROR_SOURCE = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errorsource: *const WHEA_ERROR_SOURCE_DESCRIPTOR) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type PSHED_PI_ENABLE_ERROR_SOURCE = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errorsource: *const WHEA_ERROR_SOURCE_DESCRIPTOR) -> super::super::Win32::bcrypt::NTSTATUS>;
pub type PSHED_PI_ERR_READING_PCIE_OVERRIDES = i32;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PSHED_PI_FINALIZE_ERROR_RECORD = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errorsource: *const WHEA_ERROR_SOURCE_DESCRIPTOR, bufferlength: u32, errorrecord: *mut WHEA_ERROR_RECORD) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type PSHED_PI_GET_ALL_ERROR_SOURCES = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, count: *mut u32, errorsrcs: *mut PWHEA_ERROR_SOURCE_DESCRIPTOR, length: *mut u32) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type PSHED_PI_GET_ERROR_SOURCE_INFO = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errorsource: *mut WHEA_ERROR_SOURCE_DESCRIPTOR) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type PSHED_PI_GET_INJECTION_CAPABILITIES = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, capabilities: *mut WHEA_ERROR_INJECTION_CAPABILITIES) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type PSHED_PI_INJECT_ERROR = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errortype: u64, parameter1: u64, parameter2: u64, parameter3: u64, parameter4: u64) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
pub type PSHED_PI_READ_ERROR_RECORD = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, flags: u32, errorrecordid: u64, nexterrorrecordid: *mut u64, recordlength: *mut u32, errorrecord: *mut WHEA_ERROR_RECORD) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type PSHED_PI_RETRIEVE_ERROR_INFO = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errorsource: *const WHEA_ERROR_SOURCE_DESCRIPTOR, bufferlength: u64, packet: *mut WHEA_ERROR_PACKET_V2) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type PSHED_PI_SET_ERROR_SOURCE_INFO = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errorsource: *const WHEA_ERROR_SOURCE_DESCRIPTOR) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type PSHED_PI_VENDOR_DEFINED = Option<unsafe extern "system" fn(vendordefinedaction: PSHED_PI_VENDOR_DEFINED_ACTION, offset: u32, inputlength: u32, inputbuffer: *const u8, outputlength: *mut u32, outputbuffer: *mut u8) -> super::super::Win32::bcrypt::NTSTATUS>;
pub type PSHED_PI_VENDOR_DEFINED_ACTION = i32;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_winnt"))]
pub type PSHED_PI_WRITE_ERROR_RECORD = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, flags: u32, recordlength: u32, errorrecord: *const WHEA_ERROR_RECORD) -> super::super::Win32::bcrypt::NTSTATUS>;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct PSHED_PLATFORM_DETAILS {
    pub Version: u32,
    pub Vb: PSHED_PLATFORM_DETAILS_VALID_BITS,
    pub GicRegisterOffset: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct PSHED_PLATFORM_DETAILS_VALID_BITS {
    pub _bitfield: u32,
}
pub type PSIGNAL_REG_VALUE = *mut SIGNAL_REG_VALUE;
pub type PSILO_MONITOR = *mut _SILO_MONITOR;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
pub type PSILO_MONITOR_REGISTRATION = *mut SILO_MONITOR_REGISTRATION;
pub type PSOC_SUBSYSTEM_FAILURE_DETAILS = *mut SOC_SUBSYSTEM_FAILURE_DETAILS;
pub type PSOC_SUBSYSTEM_TYPE = *mut SOC_SUBSYSTEM_TYPE;
pub type PSUBSYSTEM_INFORMATION_TYPE = *mut SUBSYSTEM_INFORMATION_TYPE;
#[cfg(feature = "Win32_bcrypt")]
pub type PSYSTEM_FIRMWARE_TABLE_HANDLER = *mut SYSTEM_FIRMWARE_TABLE_HANDLER;
pub type PSYSTEM_FIRMWARE_TABLE_INFORMATION = *mut SYSTEM_FIRMWARE_TABLE_INFORMATION;
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub struct PS_CREATE_NOTIFY_INFO {
    pub Size: usize,
    pub Anonymous: PS_CREATE_NOTIFY_INFO_0,
    pub ParentProcessId: super::super::Win32::winnt::HANDLE,
    pub CreatingThreadId: super::super::Win32::winternl::CLIENT_ID,
    pub FileObject: *mut super::wdm::FILE_OBJECT,
    pub ImageFileName: super::super::Win32::winternl::PCUNICODE_STRING,
    pub CommandLine: super::super::Win32::winternl::PCUNICODE_STRING,
    pub CreationStatus: super::super::Win32::bcrypt::NTSTATUS,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for PS_CREATE_NOTIFY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub union PS_CREATE_NOTIFY_INFO_0 {
    pub Flags: u32,
    pub Anonymous: PS_CREATE_NOTIFY_INFO_0_0,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for PS_CREATE_NOTIFY_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy, Default)]
pub struct PS_CREATE_NOTIFY_INFO_0_0 {
    pub _bitfield: u32,
}
pub const PS_IMAGE_NOTIFY_CONFLICTING_ARCHITECTURE: u32 = 1;
pub const PS_INVALID_SILO_CONTEXT_SLOT: u32 = 4294967295;
pub type PTIMER_APC_ROUTINE = Option<unsafe extern "system" fn(timercontext: *const core::ffi::c_void, timerlowvalue: u32, timerhighvalue: i32)>;
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt"))]
pub type PTIMER_SET_COALESCABLE_TIMER_INFO = *mut TIMER_SET_COALESCABLE_TIMER_INFO;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PTRANSLATE_RESOURCE_HANDLER = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, source: *const super::wdm::CM_PARTIAL_RESOURCE_DESCRIPTOR, direction: RESOURCE_TRANSLATION_DIRECTION, alternativescount: u32, alternatives: *const super::wdm::IO_RESOURCE_DESCRIPTOR, physicaldeviceobject: *const super::wdm::DEVICE_OBJECT, target: *mut super::wdm::CM_PARTIAL_RESOURCE_DESCRIPTOR) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PTRANSLATE_RESOURCE_REQUIREMENTS_HANDLER = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, source: *const super::wdm::IO_RESOURCE_DESCRIPTOR, physicaldeviceobject: *const super::wdm::DEVICE_OBJECT, targetcount: *mut u32, target: *mut super::wdm::PIO_RESOURCE_DESCRIPTOR) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type PTRANSLATOR_INTERFACE = *mut TRANSLATOR_INTERFACE;
pub type PTXN_PARAMETER_BLOCK = *mut TXN_PARAMETER_BLOCK;
pub type PVM_COUNTERS = *mut VM_COUNTERS;
pub type PVM_COUNTERS_EX = *mut VM_COUNTERS_EX;
pub type PVM_COUNTERS_EX2 = *mut VM_COUNTERS_EX2;
pub type PWEHA_CXL_DEVICE_ID = *mut WHEA_CXL_DEVICE_ID;
pub type PWEHA_CXL_PCIE_DEVICE_ID = *mut WHEA_CXL_PCIE_DEVICE_ID;
pub type PWHEA128A = *mut WHEA128A;
pub type PWHEAP_ACPI_TIMEOUT_EVENT = *mut WHEAP_ACPI_TIMEOUT_EVENT;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type PWHEAP_ADD_REMOVE_ERROR_SOURCE_EVENT = *mut WHEAP_ADD_REMOVE_ERROR_SOURCE_EVENT;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEAP_ATTEMPT_RECOVERY_EVENT = *mut WHEAP_ATTEMPT_RECOVERY_EVENT;
pub type PWHEAP_BAD_HEST_NOTIFY_DATA_EVENT = *mut WHEAP_BAD_HEST_NOTIFY_DATA_EVENT;
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt"))]
pub type PWHEAP_BIT_OFFLINE_EVENT = *mut WHEAP_BIT_OFFLINE_EVENT;
pub type PWHEAP_CLEARED_POISON_EVENT = *mut WHEAP_CLEARED_POISON_EVENT;
pub type PWHEAP_CMCI_IMPLEMENTED_EVENT = *mut WHEAP_CMCI_IMPLEMENTED_EVENT;
pub type PWHEAP_CMCI_INITERR_EVENT = *mut WHEAP_CMCI_INITERR_EVENT;
pub type PWHEAP_CMCI_RESTART_EVENT = *mut WHEAP_CMCI_RESTART_EVENT;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEAP_CREATE_GENERIC_RECORD_EVENT = *mut WHEAP_CREATE_GENERIC_RECORD_EVENT;
#[cfg(feature = "Win32_winnt")]
pub type PWHEAP_DEFERRED_EVENT = *mut WHEAP_DEFERRED_EVENT;
pub type PWHEAP_DEVICE_DRV_EVENT = *mut WHEAP_DEVICE_DRV_EVENT;
pub type PWHEAP_DPC_ERROR_EVENT = *mut WHEAP_DPC_ERROR_EVENT;
pub type PWHEAP_DPC_ERROR_EVENT_TYPE = *mut WHEAP_DPC_ERROR_EVENT_TYPE;
pub type PWHEAP_DROPPED_CORRECTED_ERROR_EVENT = *mut WHEAP_DROPPED_CORRECTED_ERROR_EVENT;
pub type PWHEAP_EDPC_ENABLED_EVENT = *mut WHEAP_EDPC_ENABLED_EVENT;
pub type PWHEAP_ERROR_CLEARED_EVENT = *mut WHEAP_ERROR_CLEARED_EVENT;
#[cfg(feature = "Win32_winnt")]
pub type PWHEAP_ERROR_RECORD_EVENT = *mut WHEAP_ERROR_RECORD_EVENT;
pub type PWHEAP_ERR_SRC_ARRAY_INVALID_EVENT = *mut WHEAP_ERR_SRC_ARRAY_INVALID_EVENT;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type PWHEAP_ERR_SRC_INVALID_EVENT = *mut WHEAP_ERR_SRC_INVALID_EVENT;
pub type PWHEAP_FOUND_ERROR_IN_BANK_EVENT = *mut WHEAP_FOUND_ERROR_IN_BANK_EVENT;
pub type PWHEAP_GENERIC_ERR_MEM_MAP_EVENT = *mut WHEAP_GENERIC_ERR_MEM_MAP_EVENT;
pub type PWHEAP_OSC_IMPLEMENTED = *mut WHEAP_OSC_IMPLEMENTED;
pub type PWHEAP_PCIE_CONFIG_INFO = *mut WHEAP_PCIE_CONFIG_INFO;
pub type PWHEAP_PCIE_OVERRIDE_INFO = *mut WHEAP_PCIE_OVERRIDE_INFO;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEAP_PCIE_READ_OVERRIDES_ERR = *mut WHEAP_PCIE_READ_OVERRIDES_ERR;
pub type PWHEAP_PFA_MEMORY_OFFLINED = *mut WHEAP_PFA_MEMORY_OFFLINED;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEAP_PFA_MEMORY_OFFLINED_NOTIFY_CALLBACK_ACTION = *mut WHEAP_PFA_MEMORY_OFFLINED_NOTIFY_CALLBACK_ACTION;
pub type PWHEAP_PFA_MEMORY_POLICY = *mut WHEAP_PFA_MEMORY_POLICY;
pub type PWHEAP_PFA_MEMORY_REMOVE_MONITOR = *mut WHEAP_PFA_MEMORY_REMOVE_MONITOR;
pub type PWHEAP_PFA_OFFLINE_DECISION_TYPE = *mut WHEAP_PFA_OFFLINE_DECISION_TYPE;
pub type PWHEAP_PLUGIN_DEFECT_LIST_CORRUPT = *mut WHEAP_PLUGIN_DEFECT_LIST_CORRUPT;
pub type PWHEAP_PLUGIN_DEFECT_LIST_FULL_EVENT = *mut WHEAP_PLUGIN_DEFECT_LIST_FULL_EVENT;
pub type PWHEAP_PLUGIN_DEFECT_LIST_UEFI_VAR_FAILED = *mut WHEAP_PLUGIN_DEFECT_LIST_UEFI_VAR_FAILED;
pub type PWHEAP_PLUGIN_PFA_EVENT = *mut WHEAP_PLUGIN_PFA_EVENT;
pub type PWHEAP_PROCESS_EINJ_EVENT = *mut WHEAP_PROCESS_EINJ_EVENT;
pub type PWHEAP_PROCESS_EINJ_EVENT2 = *mut WHEAP_PROCESS_EINJ_EVENT2;
pub type PWHEAP_PROCESS_HEST_EVENT = *mut WHEAP_PROCESS_HEST_EVENT;
pub type PWHEAP_PROMOTED_AER_ERROR_EVENT = *mut WHEAP_PROMOTED_AER_ERROR_EVENT;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEAP_PSHED_INJECT_ERROR = *mut WHEAP_PSHED_INJECT_ERROR;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEAP_PSHED_PLUGIN_REGISTER = *mut WHEAP_PSHED_PLUGIN_REGISTER;
#[cfg(feature = "Wdk_wdm")]
pub type PWHEAP_ROW_FAILURE_EVENT = *mut WHEAP_ROW_FAILURE_EVENT;
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt"))]
pub type PWHEAP_ROW_OFFLINE_EVENT = *mut WHEAP_ROW_OFFLINE_EVENT;
pub type PWHEAP_SPURIOUS_AER_EVENT = *mut WHEAP_SPURIOUS_AER_EVENT;
pub type PWHEAP_STARTED_REPORT_HW_ERROR = *mut WHEAP_STARTED_REPORT_HW_ERROR;
pub type PWHEAP_STUCK_ERROR_EVENT = *mut WHEAP_STUCK_ERROR_EVENT;
pub type PWHEA_ACPI_HEADER = *mut WHEA_ACPI_HEADER;
pub type PWHEA_AER_BRIDGE_DESCRIPTOR = *mut WHEA_AER_BRIDGE_DESCRIPTOR;
pub type PWHEA_AER_ENDPOINT_DESCRIPTOR = *mut WHEA_AER_ENDPOINT_DESCRIPTOR;
pub type PWHEA_AER_ROOTPORT_DESCRIPTOR = *mut WHEA_AER_ROOTPORT_DESCRIPTOR;
pub type PWHEA_AMD_EXTENDED_REGISTERS = *mut WHEA_AMD_EXTENDED_REGISTERS;
pub type PWHEA_ARMV8_AARCH32_GPRS = *mut WHEA_ARMV8_AARCH32_GPRS;
pub type PWHEA_ARMV8_AARCH64_EL3_CSR = *mut WHEA_ARMV8_AARCH64_EL3_CSR;
pub type PWHEA_ARMV8_AARCH64_GPRS = *mut WHEA_ARMV8_AARCH64_GPRS;
pub type PWHEA_ARM_AARCH32_EL1 = *mut WHEA_ARM_AARCH32_EL1_CSR;
pub type PWHEA_ARM_AARCH32_EL2_CSR = *mut WHEA_ARM_AARCH32_EL2_CSR;
pub type PWHEA_ARM_AARCH32_SECURE_CSR = *mut WHEA_ARM_AARCH32_SECURE_CSR;
pub type PWHEA_ARM_AARCH64_EL1_CSR = *mut WHEA_ARM_AARCH64_EL1_CSR;
pub type PWHEA_ARM_AARCH64_EL2_CSR = *mut WHEA_ARM_AARCH64_EL2_CSR;
pub type PWHEA_ARM_BUS_ERROR = *mut WHEA_ARM_BUS_ERROR;
pub type PWHEA_ARM_BUS_ERROR_VALID_BITS = *mut WHEA_ARM_BUS_ERROR_VALID_BITS;
pub type PWHEA_ARM_CACHE_ERROR = *mut WHEA_ARM_CACHE_ERROR;
pub type PWHEA_ARM_CACHE_ERROR_VALID_BITS = *mut WHEA_ARM_CACHE_ERROR_VALID_BITS;
pub type PWHEA_ARM_MISC_CSR = *mut WHEA_ARM_MISC_CSR;
pub type PWHEA_ARM_PROCESSOR_ERROR = *mut WHEA_ARM_PROCESSOR_ERROR;
pub type PWHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER = *mut WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER;
pub type PWHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS = *mut WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS;
pub type PWHEA_ARM_PROCESSOR_ERROR_INFORMATION = *mut WHEA_ARM_PROCESSOR_ERROR_INFORMATION;
pub type PWHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS = *mut WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS;
pub type PWHEA_ARM_PROCESSOR_ERROR_SECTION = *mut WHEA_ARM_PROCESSOR_ERROR_SECTION;
pub type PWHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS = *mut WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS;
pub type PWHEA_ARM_RAS_NODE_INTERFACES = *mut WHEA_ARM_RAS_NODE_INTERFACES;
pub type PWHEA_ARM_RAS_NODE_SECTION = *mut WHEA_ARM_RAS_NODE_SECTION;
pub type PWHEA_ARM_TLB_ERROR = *mut WHEA_ARM_TLB_ERROR;
pub type PWHEA_ARM_TLB_ERROR_VALID_BITS = *mut WHEA_ARM_TLB_ERROR_VALID_BITS;
pub type PWHEA_AZCC_ROOT_BUS_ERR_EVENT = *mut WHEA_AZCC_ROOT_BUS_ERR_EVENT;
pub type PWHEA_AZCC_ROOT_BUS_LIST_EVENT = *mut WHEA_AZCC_ROOT_BUS_LIST_EVENT;
pub type PWHEA_AZCC_SET_POISON_EVENT = *mut WHEA_AZCC_SET_POISON_EVENT;
pub type PWHEA_BUGCHECK_RECOVERY_LOG_TYPE = *mut WHEA_BUGCHECK_RECOVERY_LOG_TYPE;
pub type PWHEA_CPU_VENDOR = *mut WHEA_CPU_VENDOR;
pub type PWHEA_CRASHDUMP_EVENT_LOG_ENTRY_ULONG1 = *mut WHEA_CRASHDUMP_EVENT_LOG_ENTRY_ULONG1;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEA_CRASHDUMP_EVENT_LOG_ENTRY_WITH_STATUS = *mut WHEA_CRASHDUMP_EVENT_LOG_ENTRY_WITH_STATUS;
pub type PWHEA_CXL_AGENT_ADDRESS = *mut WHEA_CXL_AGENT_ADDRES;
pub type PWHEA_CXL_AGENT_TYPE = *mut WHEA_CXL_AGENT_TYPE;
pub type PWHEA_CXL_COMPONENT_EVENTS_SECTION = *mut WHEA_CXL_COMPONENT_EVENTS_SECTION;
pub type PWHEA_CXL_COMPONENT_EVENTS_SECTION_VALIDBITS = *mut WHEA_CXL_COMPONENT_EVENTS_SECTION_VALIDBITS;
pub type PWHEA_CXL_DEVICE_SERIAL_NUMBER = *mut WHEA_CXL_DEVICE_SERIAL_NUMBER;
pub type PWHEA_CXL_PROTOCOL_ERROR_SECTION = *mut WHEA_CXL_PROTOCOL_ERROR_SECTION;
pub type PWHEA_CXL_PROTOCOL_ERROR_SECTION_VALIDBITS = *mut WHEA_CXL_PROTOCOL_ERROR_SECTION_VALIDBITS;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type PWHEA_DEVICE_DRIVER_DESCRIPTOR = *mut WHEA_DEVICE_DRIVER_DESCRIPTOR;
pub type PWHEA_DEVICE_INFO = *mut WHEA_DEVICE_INFO;
pub type PWHEA_DEVICE_TYPE = *mut WHEA_DEVICE_TYPE;
#[cfg(all(feature = "Win32_guiddef", feature = "Win32_minwindef"))]
pub type PWHEA_DRIVER_BUFFER_SET = *mut WHEA_DRIVER_BUFFER_SET;
pub type PWHEA_ERROR_INJECTION_CAPABILITIES = *mut WHEA_ERROR_INJECTION_CAPABILITIES;
pub type PWHEA_ERROR_PACKET = *mut WHEA_ERROR_PACKET_V2;
pub type PWHEA_ERROR_PACKET_DATA_FORMAT = *mut WHEA_ERROR_PACKET_DATA_FORMAT;
pub type PWHEA_ERROR_PACKET_FLAGS = *mut WHEA_ERROR_PACKET_FLAGS;
pub type PWHEA_ERROR_PACKET_V1 = *mut WHEA_ERROR_PACKET_V1;
pub type PWHEA_ERROR_PACKET_V2 = *mut WHEA_ERROR_PACKET_V2;
#[cfg(feature = "Win32_winnt")]
pub type PWHEA_ERROR_RECORD = *mut WHEA_ERROR_RECORD;
pub type PWHEA_ERROR_RECORD_HEADER = *mut WHEA_ERROR_RECORD_HEADER;
pub type PWHEA_ERROR_RECORD_HEADER_FLAGS = *mut WHEA_ERROR_RECORD_HEADER_FLAGS;
pub type PWHEA_ERROR_RECORD_HEADER_VALIDBITS = *mut WHEA_ERROR_RECORD_HEADER_VALIDBITS;
#[cfg(feature = "Win32_winnt")]
pub type PWHEA_ERROR_RECORD_SECTION_DESCRIPTOR = *mut WHEA_ERROR_RECORD_SECTION_DESCRIPTOR;
pub type PWHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS = *mut WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS;
pub type PWHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS = *mut WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS;
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
pub type PWHEA_ERROR_RECOVERY_INFO_SECTION = *mut WHEA_ERROR_RECOVERY_INFO_SECTION;
pub type PWHEA_ERROR_SEVERITY = *mut WHEA_ERROR_SEVERITY;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PWHEA_ERROR_SOURCE_CONFIGURATION = *mut WHEA_ERROR_SOURCE_CONFIGURATION;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEA_ERROR_SOURCE_CONFIGURATION_DD = *mut WHEA_ERROR_SOURCE_CONFIGURATION_DD;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER = *mut WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1 = *mut WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type PWHEA_ERROR_SOURCE_DESCRIPTOR = *mut WHEA_ERROR_SOURCE_DESCRIPTOR;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type PWHEA_ERROR_SOURCE_DESCRIPTOR_V2 = *mut WHEA_ERROR_SOURCE_DESCRIPTOR_V2;
pub type PWHEA_ERROR_SOURCE_OVERRIDE_SETTINGS = *mut WHEA_ERROR_SOURCE_OVERRIDE_SETTINGS;
pub type PWHEA_ERROR_SOURCE_STATE = *mut WHEA_ERROR_SOURCE_STATE;
pub type PWHEA_ERROR_SOURCE_TYPE = *mut WHEA_ERROR_SOURCE_TYPE;
pub type PWHEA_ERROR_STATUS = *mut WHEA_ERROR_STATUS;
pub type PWHEA_ERROR_TYPE = *mut WHEA_ERROR_TYPE;
pub type PWHEA_ETW_OVERFLOW_EVENT = *mut WHEA_ETW_OVERFLOW_EVENT;
pub type PWHEA_EVENT_LOG_ENTRY = *mut WHEA_EVENT_LOG_ENTRY;
pub type PWHEA_EVENT_LOG_ENTRY_FLAGS = *mut WHEA_EVENT_LOG_ENTRY_FLAGS;
pub type PWHEA_EVENT_LOG_ENTRY_HEADER = *mut WHEA_EVENT_LOG_ENTRY_HEADER;
pub type PWHEA_EVENT_LOG_ENTRY_ID = *mut WHEA_EVENT_LOG_ENTRY_ID;
pub type PWHEA_EVENT_LOG_ENTRY_TYPE = *mut WHEA_EVENT_LOG_ENTRY_TYPE;
pub type PWHEA_EXTENDED_RAS = *mut WHEA_EXTENDED_RAS;
pub type PWHEA_FAILED_ADD_DEFECT_LIST_EVENT = *mut WHEA_FAILED_ADD_DEFECT_LIST_EVENT;
pub type PWHEA_FIRMWARE_ERROR_RECORD_REFERENCE = *mut WHEA_FIRMWARE_ERROR_RECORD_REFERENCE;
pub type PWHEA_GAS_ERRORS = *mut WHEA_GAS_ERRORS;
pub type PWHEA_GAS_ERROR_EVENT = *mut WHEA_GAS_ERROR_EVENT;
pub type PWHEA_GENERIC_ERROR = *mut WHEA_GENERIC_ERROR;
pub type PWHEA_GENERIC_ERROR_BLOCKSTATUS = *mut WHEA_GENERIC_ERROR_BLOCKSTATUS;
pub type PWHEA_GENERIC_ERROR_DATA_ENTRY = *mut WHEA_GENERIC_ERROR_DATA_ENTRY_V2;
pub type PWHEA_GENERIC_ERROR_DATA_ENTRY_V1 = *mut WHEA_GENERIC_ERROR_DATA_ENTRY_V1;
pub type PWHEA_GENERIC_ERROR_DATA_ENTRY_V2 = *mut WHEA_GENERIC_ERROR_DATA_ENTRY_V2;
pub type PWHEA_GENERIC_ERROR_DESCRIPTOR = *mut WHEA_GENERIC_ERROR_DESCRIPTOR;
pub type PWHEA_GENERIC_ERROR_DESCRIPTOR_V2 = *mut WHEA_GENERIC_ERROR_DESCRIPTOR_V2;
pub type PWHEA_IN_USE_PAGE_NOTIFY_FLAGS = *mut WHEA_IN_USE_PAGE_NOTIFY_FLAGS;
pub type PWHEA_IPF_CMC_DESCRIPTOR = *mut WHEA_IPF_CMC_DESCRIPTOR;
pub type PWHEA_IPF_CPE_DESCRIPTOR = *mut WHEA_IPF_CPE_DESCRIPTOR;
pub type PWHEA_IPF_MCA_DESCRIPTOR = *mut WHEA_IPF_MCA_DESCRIPTOR;
pub type PWHEA_IPMI_LOAD_EVENT = *mut WHEA_IPMI_LOAD_EVENT;
pub type PWHEA_IPMI_SUBSCRIBE_EVENT = *mut WHEA_IPMI_SUBSCRIBE_EVENT;
pub type PWHEA_MEMORY_CORRECTABLE_ERROR_DATA = *mut WHEA_MEMORY_CORRECTABLE_ERROR_DATA;
pub type PWHEA_MEMORY_CORRECTABLE_ERROR_HEADER = *mut WHEA_MEMORY_CORRECTABLE_ERROR_HEADER;
pub type PWHEA_MEMORY_CORRECTABLE_ERROR_SECTION = *mut WHEA_MEMORY_CORRECTABLE_ERROR_SECTION;
pub type PWHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS = *mut WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS;
pub type PWHEA_MEMORY_DEFINITION = *mut WHEA_MEMORY_DEFINITION;
pub type PWHEA_MEMORY_ERROR_EXT_SECTION_AMD = *mut WHEA_MEMORY_ERROR_EXT_SECTION_AMD;
pub type PWHEA_MEMORY_ERROR_EXT_SECTION_AMD_VALIDBITS = *mut WHEA_MEMORY_ERROR_EXT_SECTION_AMD_VALIDBITS;
pub type PWHEA_MEMORY_ERROR_EXT_SECTION_FLAGS = *mut WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS;
pub type PWHEA_MEMORY_ERROR_EXT_SECTION_INTEL = *mut WHEA_MEMORY_ERROR_EXT_SECTION_INTEL;
pub type PWHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS = *mut WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS;
pub type PWHEA_MEMORY_ERROR_SECTION = *mut WHEA_MEMORY_ERROR_SECTION;
pub type PWHEA_MEMORY_ERROR_SECTION_VALIDBITS = *mut WHEA_MEMORY_ERROR_SECTION_VALIDBITS;
pub type PWHEA_MEMORY_HARDWARE_ADDRESS_AMD = *mut WHEA_MEMORY_HARDWARE_ADDRESS_AMD;
pub type PWHEA_MEMORY_HARDWARE_ADDRESS_INTEL = *mut WHEA_MEMORY_HARDWARE_ADDRESS_INTEL;
pub type PWHEA_MEMORY_RANGE = *mut WHEA_MEMORY_RANGE;
pub type PWHEA_MEMORY_RANGE_ERROR_SECTION = *mut WHEA_MEMORY_RANGE_ERROR_SECTION;
pub type PWHEA_MEMORY_RANGE_ERROR_SECTION_VALIDBITS = *mut WHEA_MEMORY_RANGE_ERROR_SECTION_VALIDBITS;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEA_MEMORY_THROTTLE_SUMMARY_FAILED_EVENT = *mut WHEA_MEMORY_THROTTLE_SUMMARY_FAILED_EVENT;
pub type PWHEA_MSR_DUMP_SECTION = *mut WHEA_MSR_DUMP_SECTION;
pub type PWHEA_NMI_ERROR_SECTION = *mut WHEA_NMI_ERROR_SECTION;
pub type PWHEA_NMI_ERROR_SECTION_FLAGS = *mut WHEA_NMI_ERROR_SECTION_FLAGS;
pub type PWHEA_NOTIFICATION_DESCRIPTOR = *mut WHEA_NOTIFICATION_DESCRIPTOR;
pub type PWHEA_NOTIFICATION_FLAGS = *mut WHEA_NOTIFICATION_FLAGS;
pub type PWHEA_OFFLINE_DONE_EVENT = *mut WHEA_OFFLINE_DONE_EVENT;
pub type PWHEA_OFFLINE_ERRS = *mut WHEA_OFFLINE_ERRS;
pub type PWHEA_PACKET_LOG_DATA = *mut WHEA_PACKET_LOG_DATA;
pub type PWHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS = *mut WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS;
pub type PWHEA_PCIEXPRESS_COMMAND_STATUS = *mut WHEA_PCIEXPRESS_COMMAND_STATUS;
pub type PWHEA_PCIEXPRESS_DEVICE_ID = *mut WHEA_PCIEXPRESS_DEVICE_ID;
pub type PWHEA_PCIEXPRESS_ERROR_SECTION = *mut WHEA_PCIEXPRESS_ERROR_SECTION;
pub type PWHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS = *mut WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS;
pub type PWHEA_PCIEXPRESS_VERSION = *mut WHEA_PCIEXPRESS_VERSION;
pub type PWHEA_PCIE_ADDRESS = *mut WHEA_PCIE_ADDRESS;
pub type PWHEA_PCIE_CORRECTABLE_ERROR_DEVICES = *mut WHEA_PCIE_CORRECTABLE_ERROR_DEVICES;
pub type PWHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS = *mut WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS;
pub type PWHEA_PCIE_CORRECTABLE_ERROR_SECTION = *mut WHEA_PCIE_CORRECTABLE_ERROR_SECTION;
pub type PWHEA_PCIE_CORRECTABLE_ERROR_SECTION_HEADER = *mut WHEA_PCIE_CORRECTABLE_ERROR_SECTION_HEADER;
pub type PWHEA_PCIXBUS_COMMAND = *mut WHEA_PCIXBUS_COMMAND;
pub type PWHEA_PCIXBUS_ERROR_SECTION = *mut WHEA_PCIXBUS_ERROR_SECTION;
pub type PWHEA_PCIXBUS_ERROR_SECTION_VALIDBITS = *mut WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS;
pub type PWHEA_PCIXBUS_ID = *mut WHEA_PCIXBUS_ID;
pub type PWHEA_PCIXDEVICE_ERROR_SECTION = *mut WHEA_PCIXDEVICE_ERROR_SECTION;
pub type PWHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS = *mut WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS;
pub type PWHEA_PCIXDEVICE_ID = *mut WHEA_PCIXDEVICE_ID;
pub type PWHEA_PCIXDEVICE_REGISTER_PAIR = *mut WHEA_PCIXDEVICE_REGISTER_PAIR;
#[cfg(feature = "Wdk_wdm")]
pub type PWHEA_PCI_DPC_SECTION = *mut PCI_EXPRESS_DPC_CAPABILITY;
pub type PWHEA_PCI_RECOVERY_SECTION = *mut WHEA_PCI_RECOVERY_SECTION;
pub type PWHEA_PCI_RECOVERY_SIGNAL = *mut WHEA_PCI_RECOVERY_SIGNAL;
pub type PWHEA_PCI_RECOVERY_STATUS = *mut WHEA_PCI_RECOVERY_STATUS;
pub type PWHEA_PCI_SLOT_NUMBER = *mut WHEA_PCI_SLOT_NUMBER;
pub type PWHEA_PERSISTENCE_INFO = *mut WHEA_PERSISTENCE_INFO;
pub type PWHEA_PFA_REMOVE_TRIGGER = *mut WHEA_PFA_REMOVE_TRIGGER;
pub type PWHEA_PMEM_ERROR_SECTION = *mut WHEA_PMEM_ERROR_SECTION;
pub type PWHEA_PMEM_ERROR_SECTION_VALIDBITS = *mut WHEA_PMEM_ERROR_SECTION_VALIDBITS;
pub type PWHEA_PMEM_PAGE_RANGE = *mut WHEA_PMEM_PAGE_RANGE;
pub type PWHEA_PRM_ADDRESS_TRANSLATION_BUFFER_INTEL = *mut WHEA_PRM_ADDRESS_TRANSLATION_BUFFER_INTEL;
pub type PWHEA_PRM_DRAM_TO_NORMALIZED_OUT_BUFFER_AMD = *mut WHEA_PRM_DRAM_TO_NORMALIZED_OUT_BUFFER_AMD;
pub type PWHEA_PRM_DRAM_TO_NORMALIZED_PARAM_BUFFER_AMD = *mut WHEA_PRM_DRAM_TO_NORMALIZED_PARAM_BUFFER_AMD;
pub type PWHEA_PRM_DRAM_TO_SPA_OUT_BUFFER_AMD = *mut WHEA_PRM_DRAM_TO_SPA_OUT_BUFFER_AMD;
pub type PWHEA_PRM_DRAM_TO_SPA_PARAM_BUFFER_AMD = *mut WHEA_PRM_DRAM_TO_SPA_PARAM_BUFFER_AMD;
pub type PWHEA_PRM_NORMALIZED_TO_DRAM_OUT_BUFFER_AMD = *mut WHEA_PRM_NORMALIZED_TO_DRAM_OUT_BUFFER_AMD;
pub type PWHEA_PRM_NORMALIZED_TO_DRAM_PARAM_BUFFER_AMD = *mut WHEA_PRM_NORMALIZED_TO_DRAM_PARAM_BUFFER_AMD;
pub type PWHEA_PRM_NORMALIZED_TO_SPA_OUT_BUFFER_AMD = *mut WHEA_PRM_NORMALIZED_TO_SPA_OUT_BUFFER_AMD;
pub type PWHEA_PRM_NORMALIZED_TO_SPA_PARAM_BUFFER_AMD = *mut WHEA_PRM_NORMALIZED_TO_SPA_PARAM_BUFFER_AMD;
pub type PWHEA_PRM_SPA_TO_DRAM_OUT_BUFFER_AMD = *mut WHEA_PRM_SPA_TO_DRAM_OUT_BUFFER_AMD;
pub type PWHEA_PRM_SPA_TO_DRAM_PARAM_BUFFER_AMD = *mut WHEA_PRM_SPA_TO_DRAM_PARAM_BUFFER_AMD;
pub type PWHEA_PRM_SPA_TO_NORMALIZED_OUT_BUFFER_AMD = *mut WHEA_PRM_SPA_TO_NORMALIZED_OUT_BUFFER_AMD;
pub type PWHEA_PRM_SPA_TO_NORMALIZED_PARAM_BUFFER_AMD = *mut WHEA_PRM_SPA_TO_NORMALIZED_PARAM_BUFFER_AMD;
pub type PWHEA_PROCESSOR_FAMILY_INFO = *mut WHEA_PROCESSOR_FAMILY_INFO;
pub type PWHEA_PROCESSOR_GENERIC_ERROR_SECTION = *mut WHEA_PROCESSOR_GENERIC_ERROR_SECTION;
pub type PWHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS = *mut WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS;
pub type PWHEA_PSHED_PI_CPUID = *mut WHEA_PSHED_PI_CPUID;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEA_PSHED_PI_CPU_BUSES_INIT_FAILED_EVENT = *mut WHEA_PSHED_PI_CPU_BUSES_INIT_FAILED_EVENT;
pub type PWHEA_PSHED_PI_ERROR_RECORD_FULL_EVENT = *mut WHEA_PSHED_PI_ERROR_RECORD_FULL_EVENT;
pub type PWHEA_PSHED_PI_SERVER_TYPE_EVENT = *mut WHEA_PSHED_PI_SERVER_TYPE_EVENT;
#[cfg(feature = "Win32_winnt")]
pub type PWHEA_PSHED_PI_TRACE_EVENT = *mut WHEA_PSHED_PI_TRACE_EVENT;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PWHEA_PSHED_PLUGIN_CALLBACKS = *mut WHEA_PSHED_PLUGIN_CALLBACKS;
pub type PWHEA_PSHED_PLUGIN_DIMM_MISMATCH = *mut WHEA_PSHED_PLUGIN_DIMM_MISMATCH;
pub type PWHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS = *mut WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS;
pub type PWHEA_PSHED_PLUGIN_ENABLE_NOTIFY_FAILED_EVENT = *mut WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_FAILED_EVENT;
pub type PWHEA_PSHED_PLUGIN_HEARTBEAT = *mut WHEA_PSHED_PLUGIN_HEARTBEAT;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEA_PSHED_PLUGIN_INIT_FAILED_EVENT = *mut WHEA_PSHED_PLUGIN_INIT_FAILED_EVENT;
pub type PWHEA_PSHED_PLUGIN_LOAD_EVENT = *mut WHEA_PSHED_PLUGIN_LOAD_EVENT;
pub type PWHEA_PSHED_PLUGIN_PLATFORM_SUPPORT_EVENT = *mut WHEA_PSHED_PLUGIN_PLATFORM_SUPPORT_EVENT;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type PWHEA_PSHED_PLUGIN_REGISTRATION_PACKET = *mut WHEA_PSHED_PLUGIN_REGISTRATION_PACKET;
pub type PWHEA_PSHED_PLUGIN_UNLOAD_EVENT = *mut WHEA_PSHED_PLUGIN_UNLOAD_EVENT;
pub type PWHEA_RAW_DATA_FORMAT = *mut WHEA_RAW_DATA_FORMAT;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEA_READ_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY = *mut WHEA_READ_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY;
pub type PWHEA_RECOVERY_ACTION = *mut WHEA_RECOVERY_ACTION;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEA_RECOVERY_CONTEXT = *mut WHEA_RECOVERY_CONTEXT;
pub type PWHEA_RECOVERY_CONTEXT_ACTION_TAKEN = *mut WHEA_RECOVERY_CONTEXT_ACTION_TAKEN;
pub type PWHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO = *mut WHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO;
pub type PWHEA_RECOVERY_CONTEXT_ERROR_TYPE = *mut WHEA_RECOVERY_CONTEXT_ERROR_TYPE;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEA_RECOVERY_CONTEXT_PAGE_INFO = *mut WHEA_RECOVERY_CONTEXT_PAGE_INFO;
pub type PWHEA_RECOVERY_FAILURE_REASON = *mut WHEA_RECOVERY_FAILURE_REASON;
pub type PWHEA_RECOVERY_TYPE = *mut WHEA_RECOVERY_TYPE;
pub type PWHEA_REGISTER_KEY_NOTIFICATION_FAILED_EVENT = *mut WHEA_REGISTER_KEY_NOTIFICATION_FAILED_EVENT;
pub type PWHEA_REGISTRY_ERRORS = *mut WHEA_REGISTRY_ERRORS;
pub type PWHEA_REGISTRY_ERROR_EVENT = *mut WHEA_REGISTRY_ERROR_EVENT;
#[cfg(feature = "Win32_winnt")]
pub type PWHEA_REGNOTIFY_POLICY_CHANGE_EVENT = *mut WHEA_REGNOTIFY_POLICY_CHANGE_EVENT;
pub type PWHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS = *mut WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS;
pub type PWHEA_REVISION = *mut WHEA_REVISION;
pub type PWHEA_SEA_SECTION = *mut WHEA_SEA_SECTION;
pub type PWHEA_SEI_SECTION = *mut WHEA_SEI_SECTION;
pub type PWHEA_SEL_BUGCHECK_PROGRESS = *mut WHEA_SEL_BUGCHECK_PROGRESS;
pub type PWHEA_SEL_BUGCHECK_RECOVERY_STATUS_MULTIPLE_BUGCHECK_EVENT = *mut WHEA_SEL_BUGCHECK_RECOVERY_STATUS_MULTIPLE_BUGCHECK_EVENT;
pub type PWHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT = *mut WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT;
pub type PWHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE2_EVENT = *mut WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE2_EVENT;
#[cfg(feature = "Wdk_ntdef")]
pub type PWHEA_SEL_BUGCHECK_RECOVERY_STATUS_START_EVENT = *mut WHEA_SEL_BUGCHECK_RECOVERY_STATUS_START_EVENT;
pub type PWHEA_SEL_RAW_EVENT = *mut WHEA_SEL_RAW_EVENT;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEA_SRAR_DETAIL_EVENT = *mut WHEA_SRAR_DETAIL_EVENT;
pub type PWHEA_SRAS_TABLE_ENTRIES_EVENT = *mut WHEA_SRAS_TABLE_ENTRIES_EVENT;
pub type PWHEA_SRAS_TABLE_ERROR = *mut WHEA_SRAS_TABLE_ERROR;
pub type PWHEA_SRAS_TABLE_NOT_FOUND = *mut WHEA_SRAS_TABLE_NOT_FOUND;
pub type PWHEA_THROTTLE_ADD_ERR_SRC_FAILED_EVENT = *mut WHEA_THROTTLE_ADD_ERR_SRC_FAILED_EVENT;
pub type PWHEA_THROTTLE_MEMORY_ADD_OR_REMOVE_EVENT = *mut WHEA_THROTTLE_MEMORY_ADD_OR_REMOVE_EVENT;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEA_THROTTLE_PCIE_ADD_EVENT = *mut WHEA_THROTTLE_PCIE_ADD_EVENT;
pub type PWHEA_THROTTLE_PCIE_REMOVE_EVENT = *mut WHEA_THROTTLE_PCIE_REMOVE_EVENT;
pub type PWHEA_THROTTLE_REGISTRY_CORRUPT_EVENT = *mut WHEA_THROTTLE_REGISTRY_CORRUPT_EVENT;
pub type PWHEA_THROTTLE_REG_DATA_IGNORED_EVENT = *mut WHEA_THROTTLE_REG_DATA_IGNORED_EVENT;
pub type PWHEA_THROTTLE_TYPE = *mut WHEA_THROTTLE_TYPE;
pub type PWHEA_TIMESTAMP = *mut WHEA_TIMESTAMP;
pub type PWHEA_VERSION_MISMATCH_EVENT = *mut WHEA_VERSION_MISMATCH_EVENT;
#[cfg(feature = "Win32_bcrypt")]
pub type PWHEA_WRITE_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY = *mut WHEA_WRITE_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY;
pub type PWHEA_X64_REGISTER_STATE = *mut WHEA_X64_REGISTER_STATE;
pub type PWHEA_X86_REGISTER_STATE = *mut WHEA_X86_REGISTER_STATE;
pub type PWHEA_XPF_BUS_CHECK = *mut WHEA_XPF_BUS_CHECK;
pub type PWHEA_XPF_CACHE_CHECK = *mut WHEA_XPF_CACHE_CHECK;
pub type PWHEA_XPF_CMC_DESCRIPTOR = *mut WHEA_XPF_CMC_DESCRIPTOR;
pub type PWHEA_XPF_CMC_DESCRIPTOR_V2 = *mut WHEA_XPF_CMC_DESCRIPTOR_V2;
pub type PWHEA_XPF_CONTEXT_INFO = *mut WHEA_XPF_CONTEXT_INFO;
pub type PWHEA_XPF_MCA_SECTION = *mut WHEA_XPF_MCA_SECTION;
pub type PWHEA_XPF_MCE_DESCRIPTOR = *mut WHEA_XPF_MCE_DESCRIPTOR;
pub type PWHEA_XPF_MCE_DESCRIPTOR_V2 = *mut WHEA_XPF_MCE_DESCRIPTOR_V2;
pub type PWHEA_XPF_MC_BANK_DESCRIPTOR = *mut WHEA_XPF_MC_BANK_DESCRIPTOR;
pub type PWHEA_XPF_MS_CHECK = *mut WHEA_XPF_MS_CHECK;
pub type PWHEA_XPF_NMI_DESCRIPTOR = *mut WHEA_XPF_NMI_DESCRIPTOR;
pub type PWHEA_XPF_PROCESSOR_ERROR_SECTION = *mut WHEA_XPF_PROCESSOR_ERROR_SECTION;
pub type PWHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS = *mut WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS;
pub type PWHEA_XPF_PROCINFO = *mut WHEA_XPF_PROCINFO;
pub type PWHEA_XPF_PROCINFO_VALIDBITS = *mut WHEA_XPF_PROCINFO_VALIDBITS;
pub type PWHEA_XPF_TLB_CHECK = *mut WHEA_XPF_TLB_CHECK;
pub type PXPF_MCE_FLAGS = *mut XPF_MCE_FLAGS;
pub type PXPF_MC_BANK_FLAGS = *mut XPF_MC_BANK_FLAGS;
pub type PXPF_RECOVERY_INFO = *mut XPF_RECOVERY_INFO;
#[cfg(feature = "Win32_winnt")]
pub type PZONE_HEADER = *mut ZONE_HEADER;
#[cfg(feature = "Win32_winnt")]
pub type PZONE_SEGMENT_HEADER = *mut ZONE_SEGMENT_HEADER;
pub const ParallelController: CONFIGURATION_TYPE = 20;
pub const PciConventional: PCI_HARDWARE_INTERFACE = 0;
pub const PciDeviceD3Cold_Reason_Default_State_BitIndex: PCI_DEVICE_D3COLD_STATE_REASON = 8;
pub const PciDeviceD3Cold_Reason_INF_BitIndex: PCI_DEVICE_D3COLD_STATE_REASON = 9;
pub const PciDeviceD3Cold_Reason_Interface_Api_BitIndex: PCI_DEVICE_D3COLD_STATE_REASON = 10;
pub const PciDeviceD3Cold_State_Disabled_BitIndex: PCI_DEVICE_D3COLD_STATE_REASON = 1;
pub const PciDeviceD3Cold_State_Disabled_Bridge_HackFlags_BitIndex: PCI_DEVICE_D3COLD_STATE_REASON = 4;
pub const PciDeviceD3Cold_State_Enabled_BitIndex: PCI_DEVICE_D3COLD_STATE_REASON = 2;
pub const PciDeviceD3Cold_State_ParentRootPortS0WakeSupported_BitIndex: PCI_DEVICE_D3COLD_STATE_REASON = 3;
pub const PciExpress: PCI_HARDWARE_INTERFACE = 3;
pub const PciExpressASPMLinkSubState_L11_BitIndex: PCI_EXPRESS_LINK_SUBSTATE = 2;
pub const PciExpressASPMLinkSubState_L12_BitIndex: PCI_EXPRESS_LINK_SUBSTATE = 3;
pub const PciExpressDownstreamSwitchPort: PCI_EXPRESS_DEVICE_TYPE = 6;
pub const PciExpressEndpoint: PCI_EXPRESS_DEVICE_TYPE = 0;
pub const PciExpressLegacyEndpoint: PCI_EXPRESS_DEVICE_TYPE = 1;
pub const PciExpressPciPmLinkSubState_L11_BitIndex: PCI_EXPRESS_LINK_SUBSTATE = 0;
pub const PciExpressPciPmLinkSubState_L12_BitIndex: PCI_EXPRESS_LINK_SUBSTATE = 1;
pub const PciExpressRootComplexEventCollector: PCI_EXPRESS_DEVICE_TYPE = 10;
pub const PciExpressRootComplexIntegratedEndpoint: PCI_EXPRESS_DEVICE_TYPE = 9;
pub const PciExpressRootPort: PCI_EXPRESS_DEVICE_TYPE = 4;
pub const PciExpressToPciXBridge: PCI_EXPRESS_DEVICE_TYPE = 7;
pub const PciExpressUpstreamSwitchPort: PCI_EXPRESS_DEVICE_TYPE = 5;
#[cfg(feature = "Wdk_wdm")]
pub type PciLine2Pin = Option<unsafe extern "system" fn(bushandler: *const _BUS_HANDLER, roothandler: *const _BUS_HANDLER, slotnumber: super::wdm::PCI_SLOT_NUMBER, pcinewdata: *const super::wdm::PCI_COMMON_CONFIG, pciolddata: *const super::wdm::PCI_COMMON_CONFIG)>;
pub const PciLinkSpeed16_0Gts: PCI_LINK_SPEED = 3;
pub const PciLinkSpeed2_5Gts: PCI_LINK_SPEED = 0;
pub const PciLinkSpeed32_0Gts: PCI_LINK_SPEED = 4;
pub const PciLinkSpeed5_0Gts: PCI_LINK_SPEED = 1;
pub const PciLinkSpeed64_0Gts: PCI_LINK_SPEED = 5;
pub const PciLinkSpeed8_0Gts: PCI_LINK_SPEED = 2;
pub const PciLinkSpeedMax: PCI_LINK_SPEED = 6;
pub const PciLinkWidthMax: PCI_LINK_WIDTH = 32;
pub const PciLinkWidth_16x: PCI_LINK_WIDTH = 16;
pub const PciLinkWidth_1x: PCI_LINK_WIDTH = 1;
pub const PciLinkWidth_2x: PCI_LINK_WIDTH = 2;
pub const PciLinkWidth_4x: PCI_LINK_WIDTH = 4;
pub const PciLinkWidth_8x: PCI_LINK_WIDTH = 8;
pub const PciOscControlBitAllOnesMmioInvalidToDrivers: PCI_OSC_CONTROL_BITS = 64;
pub const PciOscControlBitCompletionTimeout: PCI_OSC_CONTROL_BITS = 256;
pub const PciOscControlBitDownstreamPortContainment: PCI_OSC_CONTROL_BITS = 128;
pub const PciOscControlBitExpressAdvancedErrorReporting: PCI_OSC_CONTROL_BITS = 8;
pub const PciOscControlBitExpressCapabilityStructure: PCI_OSC_CONTROL_BITS = 16;
pub const PciOscControlBitExpressNativeHotPlug: PCI_OSC_CONTROL_BITS = 1;
pub const PciOscControlBitExpressNativePME: PCI_OSC_CONTROL_BITS = 4;
pub const PciOscControlBitFirmwareIntermediaryConfig: PCI_OSC_CONTROL_BITS = 512;
pub const PciOscControlBitLatencyToleranceReporting: PCI_OSC_CONTROL_BITS = 32;
pub const PciOscControlBitShpcNativeHotPlug: PCI_OSC_CONTROL_BITS = 2;
#[cfg(feature = "Wdk_wdm")]
pub type PciPin2Line = Option<unsafe extern "system" fn(bushandler: *const _BUS_HANDLER, roothandler: *const _BUS_HANDLER, slotnumber: super::wdm::PCI_SLOT_NUMBER, pcidata: *const super::wdm::PCI_COMMON_CONFIG)>;
#[cfg(feature = "Wdk_wdm")]
pub type PciReadWriteConfig = Option<unsafe extern "system" fn(bushandler: *const _BUS_HANDLER, slot: super::wdm::PCI_SLOT_NUMBER, buffer: *const core::ffi::c_void, offset: u32, length: u32)>;
pub const PciXMode1: PCI_HARDWARE_INTERFACE = 1;
pub const PciXMode2: PCI_HARDWARE_INTERFACE = 2;
pub const PciXToExpressBridge: PCI_EXPRESS_DEVICE_TYPE = 8;
pub const PointerController: CONFIGURATION_TYPE = 21;
pub const PointerPeripheral: CONFIGURATION_TYPE = 31;
pub const Pos: BUS_DATA_TYPE = 2;
pub const PowerOff: PCI_EXPRESS_POWER_STATE = 1;
pub const PowerOn: PCI_EXPRESS_POWER_STATE = 0;
pub const PrimaryDcache: CONFIGURATION_TYPE = 4;
pub const PrimaryIcache: CONFIGURATION_TYPE = 3;
pub const PrinterPeripheral: CONFIGURATION_TYPE = 30;
pub const PsCreateProcessNotifySubsystems: PSCREATEPROCESSNOTIFYTYPE = 0;
pub const PsCreateThreadNotifyNonSystem: PSCREATETHREADNOTIFYTYPE = 0;
pub const PsCreateThreadNotifySubsystems: PSCREATETHREADNOTIFYTYPE = 1;
pub const PshedFADiscovery: u32 = 1;
pub const PshedFAErrorInfoRetrieval: u32 = 8;
pub const PshedFAErrorInjection: u32 = 32;
pub const PshedFAErrorRecordPersistence: u32 = 4;
pub const PshedFAErrorRecovery: u32 = 16;
pub const PshedFAErrorSourceControl: u32 = 2;
pub const PshedFAVendorDefinedErrInj: u32 = 48;
pub const PshedPiEnableNotifyErrorCreateNotifyEvent: WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS = 1;
pub const PshedPiEnableNotifyErrorCreateSystemThread: WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS = 2;
pub const PshedPiEnableNotifyErrorMax: WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS = 3;
pub const PshedPiErrReadingPcieOverridesBadSignature: PSHED_PI_ERR_READING_PCIE_OVERRIDES = 4;
pub const PshedPiErrReadingPcieOverridesBadSize: PSHED_PI_ERR_READING_PCIE_OVERRIDES = 3;
pub const PshedPiErrReadingPcieOverridesNoCapOffset: PSHED_PI_ERR_READING_PCIE_OVERRIDES = 5;
pub const PshedPiErrReadingPcieOverridesNoErr: PSHED_PI_ERR_READING_PCIE_OVERRIDES = 0;
pub const PshedPiErrReadingPcieOverridesNoMemory: PSHED_PI_ERR_READING_PCIE_OVERRIDES = 1;
pub const PshedPiErrReadingPcieOverridesNotBinary: PSHED_PI_ERR_READING_PCIE_OVERRIDES = 6;
pub const PshedPiErrReadingPcieOverridesQueryErr: PSHED_PI_ERR_READING_PCIE_OVERRIDES = 2;
pub const RCB128Bytes: PCI_EXPRESS_RCB = 1;
pub const RCB64Bytes: PCI_EXPRESS_RCB = 0;
pub const RECOVERY_INFO_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc34832a1_02c3_4c52_a9f1_9f1d5d7723fc);
pub type RESOURCE_TRANSLATION_DIRECTION = i32;
pub const RESULT_NEGATIVE: u32 = 1;
pub const RESULT_POSITIVE: u32 = 2;
pub const RESULT_ZERO: u32 = 0;
#[cfg(feature = "Wdk_ntdef")]
pub type RTL_AVL_ALLOCATE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, bytesize: super::ntdef::CLONG) -> *mut core::ffi::c_void>;
#[cfg(feature = "Wdk_ntdef")]
pub type RTL_AVL_COMPARE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, firststruct: *const core::ffi::c_void, secondstruct: *const core::ffi::c_void) -> RTL_GENERIC_COMPARE_RESULTS>;
#[cfg(feature = "Wdk_ntdef")]
pub type RTL_AVL_FREE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, buffer: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_bcrypt"))]
pub type RTL_AVL_MATCH_FUNCTION = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, userdata: *const core::ffi::c_void, matchdata: *const core::ffi::c_void) -> super::super::Win32::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(feature = "Wdk_ntdef")]
#[derive(Clone, Copy)]
pub struct RTL_AVL_TABLE {
    pub BalancedRoot: RTL_BALANCED_LINKS,
    pub OrderedPointer: *mut core::ffi::c_void,
    pub WhichOrderedElement: u32,
    pub NumberGenericTableElements: u32,
    pub DepthOfTree: u32,
    pub RestartKey: PRTL_BALANCED_LINKS,
    pub DeleteCount: u32,
    pub CompareRoutine: PRTL_AVL_COMPARE_ROUTINE,
    pub AllocateRoutine: PRTL_AVL_ALLOCATE_ROUTINE,
    pub FreeRoutine: PRTL_AVL_FREE_ROUTINE,
    pub TableContext: *mut core::ffi::c_void,
}
#[cfg(feature = "Wdk_ntdef")]
impl Default for RTL_AVL_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTL_BALANCED_LINKS {
    pub Parent: *mut Self,
    pub LeftChild: *mut Self,
    pub RightChild: *mut Self,
    pub Balance: i8,
    pub Reserved: [u8; 3],
}
impl Default for RTL_BALANCED_LINKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTL_DYNAMIC_HASH_TABLE {
    pub Flags: u32,
    pub Shift: u32,
    pub TableSize: u32,
    pub Pivot: u32,
    pub DivisorMask: u32,
    pub NumEntries: u32,
    pub NonEmptyBuckets: u32,
    pub NumEnumerators: u32,
    pub Directory: *mut core::ffi::c_void,
}
impl Default for RTL_DYNAMIC_HASH_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct RTL_DYNAMIC_HASH_TABLE_CONTEXT {
    pub ChainHead: super::super::Win32::winnt::PLIST_ENTRY,
    pub PrevLinkage: super::super::Win32::winnt::PLIST_ENTRY,
    pub Signature: usize,
}
#[cfg(feature = "Win32_winnt")]
impl Default for RTL_DYNAMIC_HASH_TABLE_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct RTL_DYNAMIC_HASH_TABLE_ENTRY {
    pub Linkage: super::super::Win32::winnt::LIST_ENTRY,
    pub Signature: usize,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct RTL_DYNAMIC_HASH_TABLE_ENUMERATOR {
    pub Anonymous: RTL_DYNAMIC_HASH_TABLE_ENUMERATOR_0,
    pub ChainHead: super::super::Win32::winnt::PLIST_ENTRY,
    pub BucketIndex: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for RTL_DYNAMIC_HASH_TABLE_ENUMERATOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub union RTL_DYNAMIC_HASH_TABLE_ENUMERATOR_0 {
    pub HashEntry: RTL_DYNAMIC_HASH_TABLE_ENTRY,
    pub CurEntry: super::super::Win32::winnt::PLIST_ENTRY,
}
#[cfg(feature = "Win32_winnt")]
impl Default for RTL_DYNAMIC_HASH_TABLE_ENUMERATOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
pub type RTL_GENERIC_ALLOCATE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, bytesize: super::ntdef::CLONG) -> *mut core::ffi::c_void>;
pub type RTL_GENERIC_COMPARE_RESULTS = i32;
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
pub type RTL_GENERIC_COMPARE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, firststruct: *const core::ffi::c_void, secondstruct: *const core::ffi::c_void) -> RTL_GENERIC_COMPARE_RESULTS>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
pub type RTL_GENERIC_FREE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, buffer: *const core::ffi::c_void)>;
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct RTL_GENERIC_TABLE {
    pub TableRoot: PRTL_SPLAY_LINKS,
    pub InsertOrderList: super::super::Win32::winnt::LIST_ENTRY,
    pub OrderedPointer: super::super::Win32::winnt::PLIST_ENTRY,
    pub WhichOrderedElement: u32,
    pub NumberGenericTableElements: u32,
    pub CompareRoutine: PRTL_GENERIC_COMPARE_ROUTINE,
    pub AllocateRoutine: PRTL_GENERIC_ALLOCATE_ROUTINE,
    pub FreeRoutine: PRTL_GENERIC_FREE_ROUTINE,
    pub TableContext: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
impl Default for RTL_GENERIC_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RTL_HASH_ALLOCATED_HEADER: u32 = 1;
pub const RTL_HASH_RESERVED_SIGNATURE: u32 = 0;
#[cfg(feature = "Win32_winnt")]
pub type RTL_RUN_ONCE_INIT_FN = Option<unsafe extern "system" fn(runonce: *mut super::super::Win32::winnt::RTL_RUN_ONCE, parameter: *mut core::ffi::c_void, context: *mut *mut core::ffi::c_void) -> u32>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTL_SPLAY_LINKS {
    pub Parent: *mut Self,
    pub LeftChild: *mut Self,
    pub RightChild: *mut Self,
}
impl Default for RTL_SPLAY_LINKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RTL_STACK_WALKING_MODE_FRAMES_TO_SKIP_SHIFT: u32 = 8;
pub const RatFailure: WHEA_OFFLINE_ERRS = 2;
pub const RatFailureFirstCol: WHEA_OFFLINE_ERRS = 3;
pub const RatFailureLastCol: WHEA_OFFLINE_ERRS = 4;
pub const RealModeIrqRoutingTable: CONFIGURATION_TYPE = 39;
pub const RealModePCIEnumeration: CONFIGURATION_TYPE = 40;
pub const ResourceTypeEventBuffer: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = 4;
pub const ResourceTypeExtendedCounterConfiguration: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = 2;
pub const ResourceTypeIdenitificationTag: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = 5;
pub const ResourceTypeMax: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = 6;
pub const ResourceTypeOverflow: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = 3;
pub const ResourceTypeRange: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = 1;
pub const ResourceTypeSingle: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = 0;
pub const ResultNegative: INTERLOCKED_RESULT = 1;
pub const ResultPositive: INTERLOCKED_RESULT = 2;
pub const ResultZero: INTERLOCKED_RESULT = 0;
pub const RowErrorDdr4: PAGE_OFFLINE_ERROR_TYPES = 1;
pub const RowErrorDdr5: PAGE_OFFLINE_ERROR_TYPES = 3;
pub const SCI_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe9d59197_94ee_4a4f_8ad8_9b7d8bd93d2e);
pub const SEA_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9a78788a_bbe8_11e4_809e_67611e5d46b0);
pub const SEA_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf5fe48a6_84ce_4c1e_aa64_20c9a53099f1);
pub const SEH_VALIDATION_POLICY_DEFER: u32 = 3;
pub const SEH_VALIDATION_POLICY_OFF: u32 = 1;
pub const SEH_VALIDATION_POLICY_ON: u32 = 0;
pub const SEH_VALIDATION_POLICY_TELEMETRY: u32 = 2;
pub const SEI_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5c284c81_b0ae_4e87_a322_b04c85624323);
pub const SEI_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf2a4a152_9c6d_4020_aecf_7695b389251b);
pub const SE_UNSOLICITED_INPUT_PRIVILEGE: u32 = 6;
pub const SHARED_GLOBAL_FLAGS_ADMINAPPROVALMODE_TYPE_SHADOWADMIN: u32 = 4096;
pub const SHARED_GLOBAL_FLAGS_ADMINAPPROVALMODE_TYPE_SHADOWADMIN_V: u32 = 12;
pub const SHARED_GLOBAL_FLAGS_ADMINAPPROVALMODE_TYPE_SPLITTOKEN: u32 = 2048;
pub const SHARED_GLOBAL_FLAGS_ADMINAPPROVALMODE_TYPE_SPLITTOKEN_V: u32 = 11;
pub const SHARED_GLOBAL_FLAGS_CLEAR_GLOBAL_DATA_FLAG: u32 = 2147483648;
pub const SHARED_GLOBAL_FLAGS_CONSOLE_BROKER_ENABLED: u32 = 64;
pub const SHARED_GLOBAL_FLAGS_CONSOLE_BROKER_ENABLED_V: u32 = 6;
pub const SHARED_GLOBAL_FLAGS_DYNAMIC_PROC_ENABLED: u32 = 32;
pub const SHARED_GLOBAL_FLAGS_DYNAMIC_PROC_ENABLED_V: u32 = 5;
pub const SHARED_GLOBAL_FLAGS_ELEVATION_ENABLED: u32 = 2;
pub const SHARED_GLOBAL_FLAGS_ELEVATION_ENABLED_V: u32 = 1;
pub const SHARED_GLOBAL_FLAGS_ERROR_PORT: u32 = 1;
pub const SHARED_GLOBAL_FLAGS_ERROR_PORT_V: u32 = 0;
pub const SHARED_GLOBAL_FLAGS_INSTALLER_DETECT_ENABLED: u32 = 8;
pub const SHARED_GLOBAL_FLAGS_INSTALLER_DETECT_ENABLED_V: u32 = 3;
pub const SHARED_GLOBAL_FLAGS_LKG_ENABLED: u32 = 16;
pub const SHARED_GLOBAL_FLAGS_LKG_ENABLED_V: u32 = 4;
pub const SHARED_GLOBAL_FLAGS_MULTIUSERS_IN_SESSION_SKU: u32 = 512;
pub const SHARED_GLOBAL_FLAGS_MULTIUSERS_IN_SESSION_SKU_V: u32 = 9;
pub const SHARED_GLOBAL_FLAGS_MULTI_SESSION_SKU: u32 = 256;
pub const SHARED_GLOBAL_FLAGS_MULTI_SESSION_SKU_V: u32 = 8;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_A73_ERRATA: u32 = 64;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_DISABLE_32BIT: u32 = 4;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_ENABLED: u32 = 1;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_HV_PAGE: u32 = 2;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_LFENCE: u32 = 32;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_MFENCE: u32 = 16;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_RDTSCP: u32 = 128;
pub const SHARED_GLOBAL_FLAGS_SECURE_BOOT_ENABLED: u32 = 128;
pub const SHARED_GLOBAL_FLAGS_SECURE_BOOT_ENABLED_V: u32 = 7;
pub const SHARED_GLOBAL_FLAGS_SET_GLOBAL_DATA_FLAG: u32 = 1073741824;
pub const SHARED_GLOBAL_FLAGS_STATE_SEPARATION_ENABLED: u32 = 1024;
pub const SHARED_GLOBAL_FLAGS_STATE_SEPARATION_ENABLED_V: u32 = 10;
pub const SHARED_GLOBAL_FLAGS_VIRT_ENABLED: u32 = 4;
pub const SHARED_GLOBAL_FLAGS_VIRT_ENABLED_V: u32 = 2;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SIGNAL_REG_VALUE {
    pub RegName: [u8; 32],
    pub MsrAddr: u32,
    pub Value: u64,
}
impl Default for SIGNAL_REG_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SILO_CONTEXT_CLEANUP_CALLBACK = Option<unsafe extern "system" fn(silocontext: *const core::ffi::c_void)>;
#[cfg(feature = "Win32_bcrypt")]
pub type SILO_MONITOR_CREATE_CALLBACK = Option<unsafe extern "system" fn(silo: *const _EJOB) -> super::super::Win32::bcrypt::NTSTATUS>;
#[repr(C)]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
#[derive(Clone, Copy)]
pub struct SILO_MONITOR_REGISTRATION {
    pub Version: u8,
    pub MonitorHost: bool,
    pub MonitorExistingSilos: bool,
    pub Reserved: [u8; 5],
    pub Anonymous: SILO_MONITOR_REGISTRATION_0,
    pub CreateCallback: SILO_MONITOR_CREATE_CALLBACK,
    pub TerminateCallback: SILO_MONITOR_TERMINATE_CALLBACK,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
impl Default for SILO_MONITOR_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
#[derive(Clone, Copy)]
pub union SILO_MONITOR_REGISTRATION_0 {
    pub DriverObjectName: super::super::Win32::ntsecapi::PUNICODE_STRING,
    pub ComponentName: super::super::Win32::ntsecapi::PUNICODE_STRING,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi"))]
impl Default for SILO_MONITOR_REGISTRATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SILO_MONITOR_REGISTRATION_VERSION: u32 = 1;
pub type SILO_MONITOR_TERMINATE_CALLBACK = Option<unsafe extern "system" fn(silo: *const _EJOB)>;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SOC_SUBSYSTEM_FAILURE_DETAILS {
    pub SubsysType: SOC_SUBSYSTEM_TYPE,
    pub FirmwareVersion: u64,
    pub HardwareVersion: u64,
    pub UnifiedFailureRegionSize: u32,
    pub UnifiedFailureRegion: [i8; 1],
}
impl Default for SOC_SUBSYSTEM_FAILURE_DETAILS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type SOC_SUBSYSTEM_TYPE = i32;
pub const SOC_SUBSYS_AUDIO_DSP: SOC_SUBSYSTEM_TYPE = 1;
pub const SOC_SUBSYS_COMPUTE_DSP: SOC_SUBSYSTEM_TYPE = 4;
pub const SOC_SUBSYS_SECURE_PROC: SOC_SUBSYSTEM_TYPE = 5;
pub const SOC_SUBSYS_SENSORS: SOC_SUBSYSTEM_TYPE = 3;
pub const SOC_SUBSYS_VENDOR_DEFINED: SOC_SUBSYSTEM_TYPE = 65536;
pub const SOC_SUBSYS_WIRELESS_MODEM: SOC_SUBSYSTEM_TYPE = 0;
pub const SOC_SUBSYS_WIRELSS_CONNECTIVITY: SOC_SUBSYSTEM_TYPE = 2;
pub const SSINFO_FLAGS_ALIGNED_DEVICE: u32 = 1;
pub const SSINFO_FLAGS_BYTE_ADDRESSABLE: u32 = 16;
pub const SSINFO_FLAGS_NO_SEEK_PENALTY: u32 = 4;
pub const SSINFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE: u32 = 2;
pub const SSINFO_FLAGS_TRIM_ENABLED: u32 = 8;
pub const SSINFO_OFFSET_UNKNOWN: u32 = 4294967295;
pub type STATE_LOCATION_TYPE = i32;
pub type SUBSYSTEM_INFORMATION_TYPE = i32;
pub const SYSTEM_CALL_INT_2E: u32 = 1;
pub const SYSTEM_CALL_SYSCALL: u32 = 0;
pub type SYSTEM_FIRMWARE_TABLE_ACTION = i32;
#[repr(C)]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct SYSTEM_FIRMWARE_TABLE_HANDLER {
    pub ProviderSignature: u32,
    pub Register: bool,
    pub FirmwareTableHandler: PFNFTH,
    pub DriverObject: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for SYSTEM_FIRMWARE_TABLE_HANDLER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_FIRMWARE_TABLE_INFORMATION {
    pub ProviderSignature: u32,
    pub Action: SYSTEM_FIRMWARE_TABLE_ACTION,
    pub TableID: u32,
    pub TableBufferLength: u32,
    pub TableBuffer: [u8; 1],
}
impl Default for SYSTEM_FIRMWARE_TABLE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ScsiAdapter: CONFIGURATION_TYPE = 10;
pub const SecondaryCache: CONFIGURATION_TYPE = 7;
pub const SecondaryDcache: CONFIGURATION_TYPE = 6;
pub const SecondaryIcache: CONFIGURATION_TYPE = 5;
pub const SerialController: CONFIGURATION_TYPE = 17;
pub const SgiInternalConfiguration: BUS_DATA_TYPE = 11;
pub const SlotEmpty: PCI_EXPRESS_CARD_PRESENCE = 0;
pub const SubsystemInformationTypeWSL: SUBSYSTEM_INFORMATION_TYPE = 1;
pub const SubsystemInformationTypeWin32: SUBSYSTEM_INFORMATION_TYPE = 0;
pub const SystemFirmwareTable_Enumerate: SYSTEM_FIRMWARE_TABLE_ACTION = 0;
pub const SystemFirmwareTable_Get: SYSTEM_FIRMWARE_TABLE_ACTION = 1;
pub const SystemMemory: CONFIGURATION_TYPE = 37;
pub type TABLE_SEARCH_RESULT = i32;
pub const THREAD_CSWITCH_PMU_DISABLE: u32 = 0;
pub const THREAD_CSWITCH_PMU_ENABLE: u32 = 1;
#[repr(C)]
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct TIMER_SET_COALESCABLE_TIMER_INFO {
    pub DueTime: i64,
    pub TimerApcRoutine: PTIMER_APC_ROUTINE,
    pub TimerContext: *mut core::ffi::c_void,
    pub WakeContext: *mut super::wdm::COUNTED_REASON_CONTEXT,
    pub Period: u32,
    pub TolerableDelay: u32,
    pub PreviousState: super::super::Win32::winnt::PBOOLEAN,
}
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_winnt"))]
impl Default for TIMER_SET_COALESCABLE_TIMER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TIMER_SET_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
#[derive(Clone, Copy)]
pub struct TRANSLATOR_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: super::wdm::PINTERFACE_REFERENCE,
    pub InterfaceDereference: super::wdm::PINTERFACE_DEREFERENCE,
    pub TranslateResources: PTRANSLATE_RESOURCE_HANDLER,
    pub TranslateResourceRequirements: PTRANSLATE_RESOURCE_REQUIREMENTS_HANDLER,
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
impl Default for TRANSLATOR_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TXF_MINIVERSION_DEFAULT_VIEW: u32 = 65534;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TXN_PARAMETER_BLOCK {
    pub Length: u16,
    pub TxFsContext: u16,
    pub TransactionObject: *mut core::ffi::c_void,
}
impl Default for TXN_PARAMETER_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const TableEmptyTree: TABLE_SEARCH_RESULT = 0;
pub const TableFoundNode: TABLE_SEARCH_RESULT = 1;
pub const TableInsertAsLeft: TABLE_SEARCH_RESULT = 2;
pub const TableInsertAsRight: TABLE_SEARCH_RESULT = 3;
pub const TapeController: CONFIGURATION_TYPE = 14;
pub const TapePeripheral: CONFIGURATION_TYPE = 27;
pub const TcAdapter: CONFIGURATION_TYPE = 9;
pub const TerminalPeripheral: CONFIGURATION_TYPE = 33;
pub const TimerSetCoalescableTimer: TIMER_SET_INFORMATION_CLASS = 0;
pub const TranslateChildToParent: RESOURCE_TRANSLATION_DIRECTION = 0;
pub const TranslateParentToChild: RESOURCE_TRANSLATION_DIRECTION = 1;
pub const UnsupportedDdrVersion: WHEA_OFFLINE_ERRS = 9;
pub const VENDOR_RESERVED_DOE_TYPE_DISCOVERY: u32 = 0;
pub const VMEConfiguration: BUS_DATA_TYPE = 5;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VM_COUNTERS {
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VM_COUNTERS_EX {
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivateUsage: usize,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct VM_COUNTERS_EX2 {
    pub CountersEx: VM_COUNTERS_EX,
    pub PrivateWorkingSetSize: usize,
    pub SharedCommitUsage: u64,
}
pub const VendorDefinedRead: PSHED_PI_VENDOR_DEFINED_ACTION = 0;
pub const VendorDefinedWrite: PSHED_PI_VENDOR_DEFINED_ACTION = 1;
pub const WCS_RAS_REGISTER_NAME_MAX_LENGTH: u32 = 32;
#[repr(C, align(16))]
#[derive(Clone, Copy, Default)]
pub struct WHEA128A {
    pub Low: u64,
    pub High: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_ACPI_TIMEOUT_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub TableType: [i8; 32],
    pub TableRequest: [i8; 32],
}
impl Default for WHEAP_ACPI_TIMEOUT_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct WHEAP_ADD_REMOVE_ERROR_SOURCE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Descriptor: WHEA_ERROR_SOURCE_DESCRIPTOR,
    pub Status: super::super::Win32::bcrypt::NTSTATUS,
    pub IsRemove: bool,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
impl Default for WHEAP_ADD_REMOVE_ERROR_SOURCE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEAP_ATTEMPT_RECOVERY_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorHeader: WHEA_ERROR_RECORD_HEADER,
    pub ArchitecturalRecovery: bool,
    pub PshedRecovery: bool,
    pub Status: super::super::Win32::bcrypt::NTSTATUS,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEAP_ATTEMPT_RECOVERY_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_BAD_HEST_NOTIFY_DATA_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub SourceId: u16,
    pub Reserved: u16,
    pub NotifyDesc: WHEA_NOTIFICATION_DESCRIPTOR,
}
impl Default for WHEAP_BAD_HEST_NOTIFY_DATA_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt"))]
#[derive(Clone, Copy)]
pub struct WHEAP_BIT_OFFLINE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Page: super::wdm::PFN_NUMBER,
    pub Status: super::super::Win32::bcrypt::NTSTATUS,
    pub ErrorReason: WHEA_OFFLINE_ERRS,
}
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt"))]
impl Default for WHEAP_BIT_OFFLINE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_CLEARED_POISON_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PhysicalAddress: u64,
}
impl Default for WHEAP_CLEARED_POISON_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_CMCI_IMPLEMENTED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub CmciAvailable: bool,
}
impl Default for WHEAP_CMCI_IMPLEMENTED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_CMCI_INITERR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Msr: u64,
    pub Type: u32,
    pub Bank: u32,
    pub EpIndex: u32,
}
impl Default for WHEAP_CMCI_INITERR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_CMCI_RESTART_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub CmciRestoreAttempts: u32,
    pub MaxCmciRestoreLimit: u32,
    pub MaxCorrectedErrorsFound: u32,
    pub MaxCorrectedErrorLimit: u32,
}
impl Default for WHEAP_CMCI_RESTART_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEAP_CREATE_GENERIC_RECORD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Error: [i8; 32],
    pub EntryCount: u32,
    pub Status: super::super::Win32::bcrypt::NTSTATUS,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEAP_CREATE_GENERIC_RECORD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct WHEAP_DEFERRED_EVENT {
    pub ListEntry: super::super::Win32::winnt::LIST_ENTRY,
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[cfg(feature = "Win32_winnt")]
impl Default for WHEAP_DEFERRED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_DEVICE_DRV_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Function: [i8; 32],
}
impl Default for WHEAP_DEVICE_DRV_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_DPC_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrType: WHEAP_DPC_ERROR_EVENT_TYPE,
    pub Bus: u32,
    pub Device: u32,
    pub Function: u32,
    pub DeviceId: u16,
    pub VendorId: u16,
}
impl Default for WHEAP_DPC_ERROR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEAP_DPC_ERROR_EVENT_TYPE = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_DROPPED_CORRECTED_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorSourceType: WHEA_ERROR_SOURCE_TYPE,
    pub ErrorSourceId: u32,
}
impl Default for WHEAP_DROPPED_CORRECTED_ERROR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_EDPC_ENABLED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub eDPCEnabled: bool,
    pub eDPCRecovEnabled: bool,
}
impl Default for WHEAP_EDPC_ENABLED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_ERROR_CLEARED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub EpIndex: u32,
    pub Bank: u32,
}
impl Default for WHEAP_ERROR_CLEARED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct WHEAP_ERROR_RECORD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Record: PWHEA_ERROR_RECORD,
}
#[cfg(feature = "Win32_winnt")]
impl Default for WHEAP_ERROR_RECORD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_ERR_SRC_ARRAY_INVALID_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorSourceCount: u32,
    pub ReportedLength: u32,
    pub ExpectedLength: u32,
}
impl Default for WHEAP_ERR_SRC_ARRAY_INVALID_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct WHEAP_ERR_SRC_INVALID_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrDescriptor: WHEA_ERROR_SOURCE_DESCRIPTOR,
    pub Error: [i8; 32],
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
impl Default for WHEAP_ERR_SRC_INVALID_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_FOUND_ERROR_IN_BANK_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub EpIndex: u32,
    pub Bank: u32,
    pub MciStatus: u64,
    pub ErrorType: u32,
}
impl Default for WHEAP_FOUND_ERROR_IN_BANK_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_GENERIC_ERR_MEM_MAP_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub MapReason: [i8; 32],
    pub PhysicalAddress: u64,
    pub Length: u64,
}
impl Default for WHEAP_GENERIC_ERR_MEM_MAP_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEAP_MAX_SET_ERROR_TYPE_WITH_ADDRESS_SIZE: u32 = 256;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_OSC_IMPLEMENTED {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub OscImplemented: bool,
    pub DebugChecked: bool,
}
impl Default for WHEAP_OSC_IMPLEMENTED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PCIE_CONFIG_INFO {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Segment: u32,
    pub Bus: u32,
    pub Device: u32,
    pub Function: u32,
    pub Offset: u32,
    pub Length: u32,
    pub Value: u64,
    pub Succeeded: u8,
    pub Reserved: [u8; 3],
}
impl Default for WHEAP_PCIE_CONFIG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PCIE_OVERRIDE_INFO {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Segment: u32,
    pub Bus: u32,
    pub Device: u32,
    pub Function: u32,
    pub ValidBits: u8,
    pub Reserved: [u8; 3],
    pub UncorrectableErrorMask: u32,
    pub UncorrectableErrorSeverity: u32,
    pub CorrectableErrorMask: u32,
    pub CapAndControl: u32,
}
impl Default for WHEAP_PCIE_OVERRIDE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEAP_PCIE_READ_OVERRIDES_ERR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub FailureReason: u32,
    pub FailureStatus: super::super::Win32::bcrypt::NTSTATUS,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEAP_PCIE_READ_OVERRIDES_ERR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PFA_MEMORY_OFFLINED {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub DecisionType: WHEAP_PFA_OFFLINE_DECISION_TYPE,
    pub ImmediateSuccess: bool,
    pub Page: u32,
    pub NotifyVid: bool,
}
impl Default for WHEAP_PFA_MEMORY_OFFLINED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEAP_PFA_MEMORY_OFFLINED_NOTIFY_CALLBACK_ACTION {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Page: u32,
    pub ComponentTag: u32,
    pub Status: super::super::Win32::bcrypt::NTSTATUS,
    pub ActionTaken: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN,
    pub ActionTakenAdditionalInfo: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEAP_PFA_MEMORY_OFFLINED_NOTIFY_CALLBACK_ACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PFA_MEMORY_POLICY {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RegistryKeysPresent: u32,
    pub DisableOffline: bool,
    pub PersistOffline: bool,
    pub PfaDisabled: bool,
    pub PageCount: u32,
    pub ErrorThreshold: u32,
    pub TimeOut: u32,
}
impl Default for WHEAP_PFA_MEMORY_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PFA_MEMORY_REMOVE_MONITOR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RemoveTrigger: WHEA_PFA_REMOVE_TRIGGER,
    pub TimeInList: u32,
    pub ErrorCount: u32,
    pub Page: u32,
}
impl Default for WHEAP_PFA_MEMORY_REMOVE_MONITOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEAP_PFA_OFFLINE_DECISION_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_PLUGIN_DEFECT_LIST_CORRUPT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
impl Default for WHEAP_PLUGIN_DEFECT_LIST_CORRUPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_PLUGIN_DEFECT_LIST_FULL_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
impl Default for WHEAP_PLUGIN_DEFECT_LIST_FULL_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_PLUGIN_DEFECT_LIST_UEFI_VAR_FAILED {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
impl Default for WHEAP_PLUGIN_DEFECT_LIST_UEFI_VAR_FAILED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_PLUGIN_PFA_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub NoFurtherPfa: bool,
}
impl Default for WHEAP_PLUGIN_PFA_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PROCESS_EINJ_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Error: [i8; 32],
    pub InjectionActionTableValid: bool,
    pub BeginInjectionInstructionCount: u32,
    pub GetTriggerErrorActionTableInstructionCount: u32,
    pub SetErrorTypeInstructionCount: u32,
    pub GetErrorTypeInstructionCount: u32,
    pub EndOperationInstructionCount: u32,
    pub ExecuteOperationInstructionCount: u32,
    pub CheckBusyStatusInstructionCount: u32,
    pub GetCommandStatusInstructionCount: u32,
    pub SetErrorTypeWithAddressInstructionCount: u32,
    pub GetExecuteOperationTimingsInstructionCount: u32,
}
impl Default for WHEAP_PROCESS_EINJ_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PROCESS_EINJ_EVENT2 {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Error: [i8; 32],
    pub InjectionActionTableValid: bool,
    pub BeginInjectionInstructionCount: u32,
    pub GetTriggerErrorActionTableInstructionCount: u32,
    pub SetErrorTypeInstructionCount: u32,
    pub GetErrorTypeInstructionCount: u32,
    pub EndOperationInstructionCount: u32,
    pub ExecuteOperationInstructionCount: u32,
    pub CheckBusyStatusInstructionCount: u32,
    pub GetCommandStatusInstructionCount: u32,
    pub SetErrorTypeWithAddressInstructionCount: u32,
    pub GetExecuteOperationTimingsInstructionCount: u32,
    pub SetErrorTypeWithAddressPa: u64,
    pub SetErrorTypeWithAddress: [u8; 256],
}
impl Default for WHEAP_PROCESS_EINJ_EVENT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PROCESS_HEST_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Error: [i8; 32],
    pub EntryType: [i8; 32],
    pub EntryIndex: u32,
    pub HestValid: bool,
    pub CmcCount: u32,
    pub MceCount: u32,
    pub NmiCount: u32,
    pub AerRootCount: u32,
    pub AerBridgeCount: u32,
    pub AerEndPointCount: u32,
    pub GenericV1Count: u32,
    pub GenericV2Count: u32,
}
impl Default for WHEAP_PROCESS_HEST_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PROMOTED_AER_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub ErrorHandlerType: u32,
    pub ErrorSourceId: u32,
    pub RootErrorCommand: u32,
    pub RootErrorStatus: u32,
    pub DeviceAssociationBitmap: u32,
}
impl Default for WHEAP_PROMOTED_AER_ERROR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEAP_PSHED_INJECT_ERROR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorType: u32,
    pub Parameter1: u64,
    pub Parameter2: u64,
    pub Parameter3: u64,
    pub Parameter4: u64,
    pub InjectionStatus: super::super::Win32::bcrypt::NTSTATUS,
    pub InjectionAttempted: bool,
    pub InjectionByPlugin: bool,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEAP_PSHED_INJECT_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEAP_PSHED_PLUGIN_REGISTER {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Version: u32,
    pub Length: u32,
    pub FunctionalAreaMask: u32,
    pub Status: super::super::Win32::bcrypt::NTSTATUS,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEAP_PSHED_PLUGIN_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Wdk_wdm")]
#[derive(Clone, Copy)]
pub struct WHEAP_ROW_FAILURE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub LowOrderPage: super::wdm::PFN_NUMBER,
    pub HighOrderPage: super::wdm::PFN_NUMBER,
}
#[cfg(feature = "Wdk_wdm")]
impl Default for WHEAP_ROW_FAILURE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt"))]
#[derive(Clone, Copy)]
pub struct WHEAP_ROW_OFFLINE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub FirstPage: super::wdm::PFN_NUMBER,
    pub LastPage: super::wdm::PFN_NUMBER,
    pub Range: u32,
    pub Status: super::super::Win32::bcrypt::NTSTATUS,
    pub ErrorReason: WHEA_OFFLINE_ERRS,
}
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_bcrypt"))]
impl Default for WHEAP_ROW_OFFLINE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_SPURIOUS_AER_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub ErrorHandlerType: u32,
    pub SpuriousErrorSourceId: u32,
    pub RootErrorCommand: u32,
    pub RootErrorStatus: u32,
    pub DeviceAssociationBitmap: u32,
}
impl Default for WHEAP_SPURIOUS_AER_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_STARTED_REPORT_HW_ERROR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorPacket: PWHEA_ERROR_PACKET,
}
impl Default for WHEAP_STARTED_REPORT_HW_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_STUCK_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub EpIndex: u32,
    pub Bank: u32,
    pub MciStatus: u64,
}
impl Default for WHEAP_STUCK_ERROR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ACPI_HEADER {
    pub Signature: u32,
    pub Length: u32,
    pub Revision: u8,
    pub Checksum: u8,
    pub OemId: [u8; 6],
    pub OemTableId: u64,
    pub OemRevision: u32,
    pub CreatorId: u32,
    pub CreatorRevision: u32,
}
impl Default for WHEA_ACPI_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_AER_BRIDGE_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: bool,
    pub Reserved: u8,
    pub BusNumber: u32,
    pub Slot: WHEA_PCI_SLOT_NUMBER,
    pub DeviceControl: u16,
    pub Flags: AER_BRIDGE_DESCRIPTOR_FLAGS,
    pub UncorrectableErrorMask: u32,
    pub UncorrectableErrorSeverity: u32,
    pub CorrectableErrorMask: u32,
    pub AdvancedCapsAndControl: u32,
    pub SecondaryUncorrectableErrorMask: u32,
    pub SecondaryUncorrectableErrorSev: u32,
    pub SecondaryCapsAndControl: u32,
}
impl Default for WHEA_AER_BRIDGE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_AER_ENDPOINT_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: bool,
    pub Reserved: u8,
    pub BusNumber: u32,
    pub Slot: WHEA_PCI_SLOT_NUMBER,
    pub DeviceControl: u16,
    pub Flags: AER_ENDPOINT_DESCRIPTOR_FLAGS,
    pub UncorrectableErrorMask: u32,
    pub UncorrectableErrorSeverity: u32,
    pub CorrectableErrorMask: u32,
    pub AdvancedCapsAndControl: u32,
}
impl Default for WHEA_AER_ENDPOINT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_AER_ROOTPORT_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: bool,
    pub Reserved: u8,
    pub BusNumber: u32,
    pub Slot: WHEA_PCI_SLOT_NUMBER,
    pub DeviceControl: u16,
    pub Flags: AER_ROOTPORT_DESCRIPTOR_FLAGS,
    pub UncorrectableErrorMask: u32,
    pub UncorrectableErrorSeverity: u32,
    pub CorrectableErrorMask: u32,
    pub AdvancedCapsAndControl: u32,
    pub RootErrorCommand: u32,
}
impl Default for WHEA_AER_ROOTPORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_AMD_EXTENDED_REGISTERS {
    pub IPID: u64,
    pub SYND: u64,
    pub CONFIG: u64,
    pub DESTAT: u64,
    pub DEADDR: u64,
    pub MISC1: u64,
    pub MISC2: u64,
    pub MISC3: u64,
    pub MISC4: u64,
    pub RasCap: u64,
    pub Reserved: [u64; 14],
}
impl Default for WHEA_AMD_EXTENDED_REGISTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_AMD_EXT_REG_NUM: u32 = 10;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARMV8_AARCH32_GPRS {
    pub R0: u32,
    pub R1: u32,
    pub R2: u32,
    pub R3: u32,
    pub R4: u32,
    pub R5: u32,
    pub R6: u32,
    pub R7: u32,
    pub R8: u32,
    pub R9: u32,
    pub R10: u32,
    pub R11: u32,
    pub R12: u32,
    pub R13: u32,
    pub R14: u32,
    pub R15: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARMV8_AARCH64_EL3_CSR {
    pub ELR_EL3: u64,
    pub ESR_EL3: u64,
    pub FAR_EL3: u64,
    pub MAIR_EL3: u64,
    pub SCTLR_EL3: u64,
    pub SP_EL3: u64,
    pub SPSR_EL3: u64,
    pub TCR_EL3: u64,
    pub TPIDR_EL3: u64,
    pub TTBR0_EL3: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARMV8_AARCH64_GPRS {
    pub X0: u64,
    pub X1: u64,
    pub X2: u64,
    pub X3: u64,
    pub X4: u64,
    pub X5: u64,
    pub X6: u64,
    pub X7: u64,
    pub X8: u64,
    pub X9: u64,
    pub X10: u64,
    pub X11: u64,
    pub X12: u64,
    pub X13: u64,
    pub X14: u64,
    pub X15: u64,
    pub X16: u64,
    pub X17: u64,
    pub X18: u64,
    pub X19: u64,
    pub X20: u64,
    pub X21: u64,
    pub X22: u64,
    pub X23: u64,
    pub X24: u64,
    pub X25: u64,
    pub X26: u64,
    pub X27: u64,
    pub X28: u64,
    pub X29: u64,
    pub X30: u64,
    pub SP: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_AARCH32_EL1_CSR {
    pub DFAR: u32,
    pub DFSR: u32,
    pub IFAR: u32,
    pub ISR: u32,
    pub MAIR0: u32,
    pub MAIR1: u32,
    pub MIDR: u32,
    pub MPIDR: u32,
    pub NMRR: u32,
    pub PRRR: u32,
    pub SCTLR: u32,
    pub SPSR: u32,
    pub SPSR_abt: u32,
    pub SPSR_fiq: u32,
    pub SPSR_irq: u32,
    pub SPSR_svc: u32,
    pub SPSR_und: u32,
    pub TPIDRPRW: u32,
    pub TPIDRURO: u32,
    pub TPIDRURW: u32,
    pub TTBCR: u32,
    pub TTBR0: u32,
    pub TTBR1: u32,
    pub DACR: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_AARCH32_EL2_CSR {
    pub ELR_hyp: u32,
    pub HAMAIR0: u32,
    pub HAMAIR1: u32,
    pub HCR: u32,
    pub HCR2: u32,
    pub HDFAR: u32,
    pub HIFAR: u32,
    pub HPFAR: u32,
    pub HSR: u32,
    pub HTCR: u32,
    pub HTPIDR: u32,
    pub HTTBR: u32,
    pub SPSR_hyp: u32,
    pub VTCR: u32,
    pub VTTBR: u32,
    pub DACR32_EL2: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_AARCH32_SECURE_CSR {
    pub SCTLR: u32,
    pub SPSR_mon: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_AARCH64_EL1_CSR {
    pub ELR_EL1: u64,
    pub ESR_EL2: u64,
    pub FAR_EL1: u64,
    pub ISR_EL1: u64,
    pub MAIR_EL1: u64,
    pub MIDR_EL1: u64,
    pub MPIDR_EL1: u64,
    pub SCTLR_EL1: u64,
    pub SP_EL0: u64,
    pub SP_EL1: u64,
    pub SPSR_EL1: u64,
    pub TCR_EL1: u64,
    pub TPIDR_EL0: u64,
    pub TPIDR_EL1: u64,
    pub TPIDRRO_EL0: u64,
    pub TTBR0_EL1: u64,
    pub TTBR1_EL1: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_AARCH64_EL2_CSR {
    pub ELR_EL2: u64,
    pub ESR_EL2: u64,
    pub FAR_EL2: u64,
    pub HACR_EL2: u64,
    pub HCR_EL2: u64,
    pub HPFAR_EL2: u64,
    pub MAIR_EL2: u64,
    pub SCTLR_EL2: u64,
    pub SP_EL2: u64,
    pub SPSR_EL2: u64,
    pub TCR_EL2: u64,
    pub TPIDR_EL2: u64,
    pub TTBR0_EL2: u64,
    pub VTCR_EL2: u64,
    pub VTTBR_EL2: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_BUS_ERROR {
    pub ValidationBit: WHEA_ARM_BUS_ERROR_VALID_BITS,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u8,
    pub _bitfield4: u16,
    pub _bitfield5: u8,
    pub _bitfield6: u32,
}
impl Default for WHEA_ARM_BUS_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ARM_BUS_ERROR_VALID_BITS {
    pub Anonymous: WHEA_ARM_BUS_ERROR_VALID_BITS_0,
    pub AsUSHORT: u16,
}
impl Default for WHEA_ARM_BUS_ERROR_VALID_BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_BUS_ERROR_VALID_BITS_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_CACHE_ERROR {
    pub ValidationBit: WHEA_ARM_CACHE_ERROR_VALID_BITS,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u64,
}
impl Default for WHEA_ARM_CACHE_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ARM_CACHE_ERROR_VALID_BITS {
    pub Anonymous: WHEA_ARM_CACHE_ERROR_VALID_BITS_0,
    pub AsUSHORT: u16,
}
impl Default for WHEA_ARM_CACHE_ERROR_VALID_BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_CACHE_ERROR_VALID_BITS_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_MISC_CSR {
    pub MRSEncoding: u16,
    pub Value: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ARM_PROCESSOR_ERROR {
    pub CacheError: WHEA_ARM_CACHE_ERROR,
    pub TlbError: WHEA_ARM_TLB_ERROR,
    pub BusError: WHEA_ARM_BUS_ERROR,
    pub AsULONGLONG: u64,
}
impl Default for WHEA_ARM_PROCESSOR_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER {
    pub Version: u16,
    pub RegisterContextType: u16,
    pub RegisterArraySize: u32,
    pub RegisterArray: [u8; 1],
}
impl Default for WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS {
    pub Anonymous: WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS_0,
    pub AsULONG: u32,
}
impl Default for WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_PROCESSOR_ERROR_INFORMATION {
    pub Version: u8,
    pub Length: u8,
    pub ValidationBit: WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS,
    pub Type: u8,
    pub MultipleError: u16,
    pub Flags: u8,
    pub ErrorInformation: u64,
    pub VirtualFaultAddress: u64,
    pub PhysicalFaultAddress: u64,
}
impl Default for WHEA_ARM_PROCESSOR_ERROR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS {
    pub Anonymous: WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS_0,
    pub AsUSHORT: u16,
}
impl Default for WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_PROCESSOR_ERROR_SECTION {
    pub ValidBits: WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS,
    pub ErrorInformationStructures: u16,
    pub ContextInformationStructures: u16,
    pub SectionLength: u32,
    pub ErrorAffinityLevel: u8,
    pub Reserved: [u8; 3],
    pub MPIDR_EL1: u64,
    pub MIDR_EL1: u64,
    pub RunningState: u32,
    pub PSCIState: u32,
    pub Data: [u8; 1],
}
impl Default for WHEA_ARM_PROCESSOR_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS {
    pub Anonymous: WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS_0,
    pub AsULONG: u32,
}
impl Default for WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS_0 {
    pub _bitfield: u32,
}
pub const WHEA_ARM_RAS_NODE_FIELD_COUNT: u32 = 8;
pub type WHEA_ARM_RAS_NODE_INTERFACES = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_RAS_NODE_SECTION {
    pub NodeFieldCount: u32,
    pub NodeIndex: u32,
    pub InterfaceType: u8,
    pub AestNodeType: u8,
    pub Reserved: [u8; 6],
    pub ErrFr: u64,
    pub ErrCtlr: u64,
    pub ErrStatus: u64,
    pub ErrAddr: u64,
    pub ErrMisc0: u64,
    pub ErrMisc1: u64,
    pub ErrMisc2: u64,
    pub ErrMisc3: u64,
}
impl Default for WHEA_ARM_RAS_NODE_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_TLB_ERROR {
    pub ValidationBit: WHEA_ARM_TLB_ERROR_VALID_BITS,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u64,
}
impl Default for WHEA_ARM_TLB_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ARM_TLB_ERROR_VALID_BITS {
    pub Anonymous: WHEA_ARM_TLB_ERROR_VALID_BITS_0,
    pub AsUSHORT: u16,
}
impl Default for WHEA_ARM_TLB_ERROR_VALID_BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_TLB_ERROR_VALID_BITS_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_AZCC_ROOT_BUS_ERR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub MaxBusCountPassed: bool,
    pub InvalidBusMSR: bool,
}
impl Default for WHEA_AZCC_ROOT_BUS_ERR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_AZCC_ROOT_BUS_LIST_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RootBusCount: u32,
    pub RootBuses: [u32; 8],
}
impl Default for WHEA_AZCC_ROOT_BUS_LIST_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_AZCC_SET_POISON_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Bus: u32,
    pub ReadSuccess: bool,
    pub WriteSuccess: bool,
    pub IsEnable: bool,
}
impl Default for WHEA_AZCC_SET_POISON_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_BAD_PAGE_LIST_LOCATION: u32 = 15;
pub const WHEA_BAD_PAGE_LIST_MAX_SIZE: u32 = 14;
pub type WHEA_BUGCHECK_RECOVERY_LOG_TYPE = i32;
pub const WHEA_BUSCHECK_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1cf3f8b3_c5b1_49a2_aa59_5eef92ffa63c);
pub const WHEA_CACHECHECK_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa55701f5_e3ef_43de_ac72_249b573fad2c);
pub const WHEA_CMCI_THRESHOLD_COUNT: u32 = 10;
pub const WHEA_CMCI_THRESHOLD_POLL_COUNT: u32 = 12;
pub const WHEA_CMCI_THRESHOLD_TIME: u32 = 11;
pub type WHEA_CPU_VENDOR = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_CRASHDUMP_EVENT_LOG_ENTRY_ULONG1 {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Value: u32,
}
impl Default for WHEA_CRASHDUMP_EVENT_LOG_ENTRY_ULONG1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEA_CRASHDUMP_EVENT_LOG_ENTRY_WITH_STATUS {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub SourceLocationId: u32,
    pub Status: super::super::Win32::bcrypt::NTSTATUS,
    pub IOStatus: super::super::Win32::bcrypt::NTSTATUS,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEA_CRASHDUMP_EVENT_LOG_ENTRY_WITH_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_CXL_AGENT_ADDRES {
    pub Anonymous: WHEA_CXL_AGENT_ADDRES_0,
    pub Anonymous2: WHEA_CXL_AGENT_ADDRES_1,
}
impl Default for WHEA_CXL_AGENT_ADDRES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_CXL_AGENT_ADDRES_0 {
    pub FunctionNumber: u8,
    pub DeviceNumber: u8,
    pub BusNumber: u8,
    pub SegmentNumber: u16,
    pub Reserved: [u8; 3],
}
impl Default for WHEA_CXL_AGENT_ADDRES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_CXL_AGENT_ADDRES_1 {
    pub CxlPortRcrbBaseAddress: [u8; 8],
}
impl Default for WHEA_CXL_AGENT_ADDRES_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_CXL_AGENT_TYPE = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_CXL_COMPONENT_EVENTS_SECTION {
    pub Length: u32,
    pub ValidBits: WHEA_CXL_COMPONENT_EVENTS_SECTION_VALIDBITS,
    pub DeviceID: WHEA_CXL_PCIE_DEVICE_ID,
    pub DeviceSerialNumber: WHEA_CXL_DEVICE_SERIAL_NUMBER,
    pub CxlComponentEventLog: [u8; 1],
}
impl Default for WHEA_CXL_COMPONENT_EVENTS_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_CXL_COMPONENT_EVENTS_SECTION_VALIDBITS {
    pub Anonymous: WHEA_CXL_COMPONENT_EVENTS_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_CXL_COMPONENT_EVENTS_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_CXL_COMPONENT_EVENTS_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_CXL_DEVICE_ID {
    pub VendorID: u16,
    pub DeviceID: u16,
    pub SubsystemVendorID: u16,
    pub SubsystemDeviceID: u16,
    pub ClassCode: u16,
    pub Anonymous: WHEA_CXL_DEVICE_ID_0,
    pub Reserved2: [u8; 4],
}
impl Default for WHEA_CXL_DEVICE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_CXL_DEVICE_ID_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_CXL_DEVICE_SERIAL_NUMBER {
    pub Anonymous: WHEA_CXL_DEVICE_SERIAL_NUMBER_0,
    pub AsUlonglong: u64,
}
impl Default for WHEA_CXL_DEVICE_SERIAL_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_CXL_DEVICE_SERIAL_NUMBER_0 {
    pub CxlDeviceSerialNumberLowerDW: u32,
    pub CxlDeviceSerialNumberUpperDW: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_CXL_PCIE_DEVICE_ID {
    pub VendorID: u16,
    pub DeviceID: u16,
    pub FunctionNumber: u8,
    pub DeviceNumber: u8,
    pub BusNumber: u8,
    pub SegmentNumber: u16,
    pub Anonymous: WHEA_CXL_PCIE_DEVICE_ID_0,
    pub Reserved2: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_CXL_PCIE_DEVICE_ID_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_CXL_PROTOCOL_ERROR_SECTION {
    pub ValidBits: WHEA_CXL_PROTOCOL_ERROR_SECTION_VALIDBITS,
    pub CxlAgentType: u8,
    pub Reserved: [u8; 7],
    pub CxlAgentAddress: WHEA_CXL_AGENT_ADDRES,
    pub DeviceID: WHEA_CXL_DEVICE_ID,
    pub DeviceSerialNumber: WHEA_CXL_DEVICE_SERIAL_NUMBER,
    pub CapabilityStructure: [u8; 60],
    pub CxlDvsecLength: u16,
    pub CxlErrorLogLength: u16,
    pub Reserved2: [u8; 4],
    pub CxlDvsecAndErrorLog: [u8; 1],
}
impl Default for WHEA_CXL_PROTOCOL_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_CXL_PROTOCOL_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_CXL_PROTOCOL_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_CXL_PROTOCOL_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_CXL_PROTOCOL_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_MAX: u32 = 1;
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_MIN: u32 = 1;
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_V1: u32 = 1;
pub const WHEA_DEVICE_DRIVER_CONFIG_MAX: u32 = 2;
pub const WHEA_DEVICE_DRIVER_CONFIG_MIN: u32 = 1;
pub const WHEA_DEVICE_DRIVER_CONFIG_V1: u32 = 1;
pub const WHEA_DEVICE_DRIVER_CONFIG_V2: u32 = 2;
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct WHEA_DEVICE_DRIVER_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: bool,
    pub Reserved: u8,
    pub SourceGuid: windows_sys::core::GUID,
    pub LogTag: u16,
    pub Reserved2: u16,
    pub PacketLength: u32,
    pub PacketCount: u32,
    pub PacketBuffer: super::super::Win32::minwindef::PUCHAR,
    pub Config: WHEA_ERROR_SOURCE_CONFIGURATION_DD,
    pub CreatorId: windows_sys::core::GUID,
    pub PartitionId: windows_sys::core::GUID,
    pub MaxSectionDataLength: u32,
    pub MaxSectionsPerRecord: u32,
    pub PacketStateBuffer: super::super::Win32::minwindef::PUCHAR,
    pub OpenHandles: i32,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
impl Default for WHEA_DEVICE_DRIVER_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_DEVICE_ERROR_SUMMARY_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x990b31e9_541a_4db0_a42f_837d344f6923);
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_DEVICE_INFO {
    pub CXL: WHEA_DEVICE_INFO_0,
    pub AsUCHAR: [u8; 72],
}
impl Default for WHEA_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_DEVICE_INFO_0 {
    pub DeviceID: WHEA_CXL_PCIE_DEVICE_ID,
    pub DeviceSerialNumber: WHEA_CXL_DEVICE_SERIAL_NUMBER,
}
impl Default for WHEA_DEVICE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_DEVICE_TYPE = i32;
pub const WHEA_DISABLE_DUMMY_WRITE: u32 = 6;
pub const WHEA_DISABLE_OFFLINE: u32 = 0;
pub const WHEA_DISABLE_PRM_ADDRESS_TRANSLATION: u32 = 20;
pub const WHEA_DPC_CAPABILITY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xec49534b_30e7_4358_972f_eca6958fae3b);
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_guiddef", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct WHEA_DRIVER_BUFFER_SET {
    pub Version: u32,
    pub Data: super::super::Win32::minwindef::PUCHAR,
    pub DataSize: u32,
    pub SectionTypeGuid: super::super::Win32::guiddef::LPGUID,
    pub SectionFriendlyName: super::super::Win32::minwindef::PUCHAR,
    pub Flags: super::super::Win32::minwindef::PUCHAR,
}
#[cfg(all(feature = "Win32_guiddef", feature = "Win32_minwindef"))]
impl Default for WHEA_DRIVER_BUFFER_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_ENABLE_BATCHED_ROW_OFFLINE: u32 = 21;
pub type WHEA_ERROR_HANDLE = *mut core::ffi::c_void;
pub const WHEA_ERROR_HANDLE_INVALID: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_INJECTION_CAPABILITIES {
    pub Anonymous: WHEA_ERROR_INJECTION_CAPABILITIES_0,
    pub AsULONG: u32,
}
impl Default for WHEA_ERROR_INJECTION_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_INJECTION_CAPABILITIES_0 {
    pub _bitfield: u32,
}
pub const WHEA_ERROR_LOG_ENTRY_ACPI: u32 = 1229996865;
pub const WHEA_ERROR_LOG_ENTRY_HAL: u32 = 541868360;
pub const WHEA_ERROR_LOG_ENTRY_HYPERV: u32 = 1347836232;
pub const WHEA_ERROR_LOG_ENTRY_KERNEL: u32 = 1280201291;
pub const WHEA_ERROR_LOG_ENTRY_PCI: u32 = 541672272;
pub const WHEA_ERROR_LOG_ENTRY_PSHED: u32 = 1145590608;
pub const WHEA_ERROR_LOG_ENTRY_PSHED_PI: u32 = 1230000976;
pub const WHEA_ERROR_LOG_ENTRY_SIGNATURE: u32 = 1733060695;
pub const WHEA_ERROR_LOG_ENTRY_VERSION: u32 = 1;
pub type WHEA_ERROR_PACKET = WHEA_ERROR_PACKET_V2;
pub type WHEA_ERROR_PACKET_DATA_FORMAT = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_PACKET_FLAGS {
    pub Anonymous: WHEA_ERROR_PACKET_FLAGS_0,
    pub AsULONG: u32,
}
impl Default for WHEA_ERROR_PACKET_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_PACKET_FLAGS_0 {
    pub _bitfield: u32,
}
pub const WHEA_ERROR_PACKET_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe71254e9_c1b9_4940_ab76_909703a4320f);
pub const WHEA_ERROR_PACKET_SIGNATURE: u32 = 1095059543;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_PACKET_V1 {
    pub Signature: u32,
    pub Flags: WHEA_ERROR_PACKET_FLAGS,
    pub Size: u32,
    pub RawDataLength: u32,
    pub Reserved1: u64,
    pub Context: u64,
    pub ErrorType: WHEA_ERROR_TYPE,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub ErrorSourceId: u32,
    pub ErrorSourceType: WHEA_ERROR_SOURCE_TYPE,
    pub Reserved2: u32,
    pub Version: u32,
    pub Cpu: u64,
    pub u: WHEA_ERROR_PACKET_V1_0,
    pub RawDataFormat: WHEA_RAW_DATA_FORMAT,
    pub RawDataOffset: u32,
    pub RawData: [u8; 1],
}
impl Default for WHEA_ERROR_PACKET_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_PACKET_V1_0 {
    pub ProcessorError: WHEA_PROCESSOR_GENERIC_ERROR_SECTION,
    pub MemoryError: WHEA_MEMORY_ERROR_SECTION,
    pub NmiError: WHEA_NMI_ERROR_SECTION,
    pub PciExpressError: WHEA_PCIEXPRESS_ERROR_SECTION,
    pub PciXBusError: WHEA_PCIXBUS_ERROR_SECTION,
    pub PciXDeviceError: WHEA_PCIXDEVICE_ERROR_SECTION,
    pub PmemError: WHEA_PMEM_ERROR_SECTION,
}
impl Default for WHEA_ERROR_PACKET_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_ERROR_PACKET_V1_SIGNATURE: u32 = 1951429189;
pub const WHEA_ERROR_PACKET_V1_VERSION: u32 = 2;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_PACKET_V2 {
    pub Signature: u32,
    pub Version: u32,
    pub Length: u32,
    pub Flags: WHEA_ERROR_PACKET_FLAGS,
    pub ErrorType: WHEA_ERROR_TYPE,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub ErrorSourceId: u32,
    pub ErrorSourceType: WHEA_ERROR_SOURCE_TYPE,
    pub NotifyType: windows_sys::core::GUID,
    pub Context: u64,
    pub DataFormat: WHEA_ERROR_PACKET_DATA_FORMAT,
    pub Reserved1: u32,
    pub DataOffset: u32,
    pub DataLength: u32,
    pub PshedDataOffset: u32,
    pub PshedDataLength: u32,
}
impl Default for WHEA_ERROR_PACKET_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_ERROR_PACKET_V2_SIGNATURE: u32 = 1095059543;
pub const WHEA_ERROR_PACKET_V2_VERSION: u32 = 3;
pub const WHEA_ERROR_PACKET_VERSION: u32 = 3;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD {
    pub Header: WHEA_ERROR_RECORD_HEADER,
    pub SectionDescriptor: [WHEA_ERROR_RECORD_SECTION_DESCRIPTOR; 1],
}
#[cfg(feature = "Win32_winnt")]
impl Default for WHEA_ERROR_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_ERROR_RECORD_FLAGS_DEVICE_DRIVER: u32 = 8;
pub const WHEA_ERROR_RECORD_FLAGS_PREVIOUSERROR: u32 = 2;
pub const WHEA_ERROR_RECORD_FLAGS_RECOVERED: u32 = 1;
pub const WHEA_ERROR_RECORD_FLAGS_SIMULATED: u32 = 4;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD_HEADER {
    pub Signature: u32,
    pub Revision: WHEA_REVISION,
    pub SignatureEnd: u32,
    pub SectionCount: u16,
    pub Severity: WHEA_ERROR_SEVERITY,
    pub ValidBits: WHEA_ERROR_RECORD_HEADER_VALIDBITS,
    pub Length: u32,
    pub Timestamp: WHEA_TIMESTAMP,
    pub PlatformId: windows_sys::core::GUID,
    pub PartitionId: windows_sys::core::GUID,
    pub CreatorId: windows_sys::core::GUID,
    pub NotifyType: windows_sys::core::GUID,
    pub RecordId: u64,
    pub Flags: WHEA_ERROR_RECORD_HEADER_FLAGS,
    pub PersistenceInfo: WHEA_PERSISTENCE_INFO,
    pub Anonymous: WHEA_ERROR_RECORD_HEADER_0,
}
impl Default for WHEA_ERROR_RECORD_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_HEADER_0 {
    pub Anonymous: WHEA_ERROR_RECORD_HEADER_0_0,
    pub Reserved: [u8; 12],
}
impl Default for WHEA_ERROR_RECORD_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD_HEADER_0_0 {
    pub OsBuildNumber: u32,
    pub Reserved2: [u8; 8],
}
impl Default for WHEA_ERROR_RECORD_HEADER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_HEADER_FLAGS {
    pub Anonymous: WHEA_ERROR_RECORD_HEADER_FLAGS_0,
    pub AsULONG: u32,
}
impl Default for WHEA_ERROR_RECORD_HEADER_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_RECORD_HEADER_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_HEADER_VALIDBITS {
    pub Anonymous: WHEA_ERROR_RECORD_HEADER_VALIDBITS_0,
    pub AsULONG: u32,
}
impl Default for WHEA_ERROR_RECORD_HEADER_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_RECORD_HEADER_VALIDBITS_0 {
    pub _bitfield: u32,
}
pub const WHEA_ERROR_RECORD_REVISION: u32 = 528;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD_SECTION_DESCRIPTOR {
    pub SectionOffset: u32,
    pub SectionLength: u32,
    pub Revision: WHEA_REVISION,
    pub ValidBits: WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS,
    pub Reserved: u8,
    pub Flags: WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS,
    pub SectionType: windows_sys::core::GUID,
    pub FRUId: windows_sys::core::GUID,
    pub SectionSeverity: WHEA_ERROR_SEVERITY,
    pub FRUText: [super::super::Win32::winnt::CCHAR; 20],
}
#[cfg(feature = "Win32_winnt")]
impl Default for WHEA_ERROR_RECORD_SECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS {
    pub Anonymous: WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS_0,
    pub AsULONG: u32,
}
impl Default for WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS_0 {
    pub _bitfield: u32,
}
pub const WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_REVISION: u32 = 768;
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS {
    pub Anonymous: WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS_0,
    pub AsUCHAR: u8,
}
impl Default for WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS_0 {
    pub _bitfield: u8,
}
pub const WHEA_ERROR_RECORD_SIGNATURE: u32 = 1380274243;
pub const WHEA_ERROR_RECORD_SIGNATURE_END: u32 = 4294967295;
pub const WHEA_ERROR_RECORD_VALID_PARTITIONID: u32 = 4;
pub const WHEA_ERROR_RECORD_VALID_PLATFORMID: u32 = 1;
pub const WHEA_ERROR_RECORD_VALID_TIMESTAMP: u32 = 2;
#[repr(C, packed(1))]
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECOVERY_INFO_SECTION {
    pub RecoveryKernel: bool,
    pub RecoveryAction: WHEA_RECOVERY_ACTION,
    pub RecoveryType: WHEA_RECOVERY_TYPE,
    pub Irql: super::ntdef::KIRQL,
    pub RecoverySucceeded: bool,
    pub FailureReason: WHEA_RECOVERY_FAILURE_REASON,
    pub ProcessName: [super::super::Win32::winnt::CCHAR; 20],
}
#[cfg(all(feature = "Wdk_ntdef", feature = "Win32_winnt"))]
impl Default for WHEA_ERROR_RECOVERY_INFO_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_ERROR_SEVERITY = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION {
    pub Flags: u32,
    pub Correct: WHEA_ERROR_SOURCE_CORRECT,
    pub Initialize: WHEA_ERROR_SOURCE_INITIALIZE,
    pub CreateRecord: WHEA_ERROR_SOURCE_CREATE_RECORD,
    pub Recover: WHEA_ERROR_SOURCE_RECOVER,
    pub Uninitialize: WHEA_ERROR_SOURCE_UNINITIALIZE,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for WHEA_ERROR_SOURCE_CONFIGURATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DD {
    pub Initialize: WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER,
    pub Uninitialize: WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER,
    pub Correct: WHEA_ERROR_SOURCE_CORRECT_DEVICE_DRIVER,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEA_ERROR_SOURCE_CONFIGURATION_DD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER {
    pub Version: u32,
    pub SourceGuid: windows_sys::core::GUID,
    pub LogTag: u16,
    pub Reserved: [u8; 6],
    pub Initialize: WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER,
    pub Uninitialize: WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER,
    pub MaxSectionDataLength: u32,
    pub MaxSectionsPerReport: u32,
    pub CreatorId: windows_sys::core::GUID,
    pub PartitionId: windows_sys::core::GUID,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1 {
    pub Version: u32,
    pub SourceGuid: windows_sys::core::GUID,
    pub LogTag: u16,
    pub Reserved: [u8; 6],
    pub Initialize: WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER,
    pub Uninitialize: WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type WHEA_ERROR_SOURCE_CORRECT = *mut _WHEA_ERROR_SOURCE_CORRECT;
#[cfg(feature = "Win32_bcrypt")]
pub type WHEA_ERROR_SOURCE_CORRECT_DEVICE_DRIVER = *mut _WHEA_ERROR_SOURCE_CORRECT_DEVICE_DRIVER;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type WHEA_ERROR_SOURCE_CREATE_RECORD = *mut _WHEA_ERROR_SOURCE_CREATE_RECORD;
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_SOURCE_DESCRIPTOR {
    pub Length: u32,
    pub Version: u32,
    pub Type: WHEA_ERROR_SOURCE_TYPE,
    pub State: WHEA_ERROR_SOURCE_STATE,
    pub MaxRawDataLength: u32,
    pub NumRecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub ErrorSourceId: u32,
    pub PlatformErrorSourceId: u32,
    pub Flags: u32,
    pub Info: WHEA_ERROR_SOURCE_DESCRIPTOR_0,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
impl Default for WHEA_ERROR_SOURCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_SOURCE_DESCRIPTOR_0 {
    pub XpfMceDescriptor: WHEA_XPF_MCE_DESCRIPTOR,
    pub XpfCmcDescriptor: WHEA_XPF_CMC_DESCRIPTOR,
    pub XpfNmiDescriptor: WHEA_XPF_NMI_DESCRIPTOR,
    pub IpfMcaDescriptor: WHEA_IPF_MCA_DESCRIPTOR,
    pub IpfCmcDescriptor: WHEA_IPF_CMC_DESCRIPTOR,
    pub IpfCpeDescriptor: WHEA_IPF_CPE_DESCRIPTOR,
    pub AerRootportDescriptor: WHEA_AER_ROOTPORT_DESCRIPTOR,
    pub AerEndpointDescriptor: WHEA_AER_ENDPOINT_DESCRIPTOR,
    pub AerBridgeDescriptor: WHEA_AER_BRIDGE_DESCRIPTOR,
    pub GenErrDescriptor: WHEA_GENERIC_ERROR_DESCRIPTOR,
    pub GenErrDescriptorV2: WHEA_GENERIC_ERROR_DESCRIPTOR_V2,
    pub DeviceDriverDescriptor: WHEA_DEVICE_DRIVER_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
impl Default for WHEA_ERROR_SOURCE_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERBRIDGE: u32 = 8;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERENDPOINT: u32 = 7;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERROOTPORT: u32 = 6;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_GENERIC: u32 = 9;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_GENERIC_V2: u32 = 10;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFCMC: u32 = 4;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFCPE: u32 = 5;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFMCA: u32 = 3;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFCMC: u32 = 1;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFMCE: u32 = 0;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFNMI: u32 = 2;
#[repr(C, packed(1))]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_SOURCE_DESCRIPTOR_V2 {
    pub Length: u32,
    pub Version: u32,
    pub Type: WHEA_ERROR_SOURCE_TYPE,
    pub State: WHEA_ERROR_SOURCE_STATE,
    pub MaxRawDataLength: u32,
    pub NumRecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
    pub ErrorSourceId: u32,
    pub PlatformErrorSourceId: u32,
    pub Flags: u32,
    pub Info: WHEA_ERROR_SOURCE_DESCRIPTOR_V2_0,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
impl Default for WHEA_ERROR_SOURCE_DESCRIPTOR_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_SOURCE_DESCRIPTOR_V2_0 {
    pub XpfMceDescriptorV2: WHEA_XPF_MCE_DESCRIPTOR_V2,
    pub XpfCmcDescriptorV2: WHEA_XPF_CMC_DESCRIPTOR_V2,
    pub XpfNmiDescriptor: WHEA_XPF_NMI_DESCRIPTOR,
    pub IpfMcaDescriptor: WHEA_IPF_MCA_DESCRIPTOR,
    pub IpfCmcDescriptor: WHEA_IPF_CMC_DESCRIPTOR,
    pub IpfCpeDescriptor: WHEA_IPF_CPE_DESCRIPTOR,
    pub AerRootportDescriptor: WHEA_AER_ROOTPORT_DESCRIPTOR,
    pub AerEndpointDescriptor: WHEA_AER_ENDPOINT_DESCRIPTOR,
    pub AerBridgeDescriptor: WHEA_AER_BRIDGE_DESCRIPTOR,
    pub GenErrDescriptor: WHEA_GENERIC_ERROR_DESCRIPTOR,
    pub GenErrDescriptorV2: WHEA_GENERIC_ERROR_DESCRIPTOR_V2,
    pub DeviceDriverDescriptor: WHEA_DEVICE_DRIVER_DESCRIPTOR,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
impl Default for WHEA_ERROR_SOURCE_DESCRIPTOR_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_VERSION_10: u32 = 10;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_VERSION_11: u32 = 11;
pub const WHEA_ERROR_SOURCE_FLAG_DEFAULTSOURCE: u32 = 2147483648;
pub const WHEA_ERROR_SOURCE_FLAG_FIRMWAREFIRST: u32 = 1;
pub const WHEA_ERROR_SOURCE_FLAG_GHES_ASSIST: u32 = 4;
pub const WHEA_ERROR_SOURCE_FLAG_GLOBAL: u32 = 2;
pub const WHEA_ERROR_SOURCE_FLAG_V2_DESCRIPTOR: u32 = 8;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type WHEA_ERROR_SOURCE_INITIALIZE = *mut _WHEA_ERROR_SOURCE_INITIALIZE;
#[cfg(feature = "Win32_bcrypt")]
pub type WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER = *mut _WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER;
pub const WHEA_ERROR_SOURCE_INVALID_RELATED_SOURCE: u32 = 65535;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_SOURCE_OVERRIDE_SETTINGS {
    pub Type: WHEA_ERROR_SOURCE_TYPE,
    pub MaxRawDataLength: u32,
    pub NumRecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
}
#[cfg(feature = "Win32_bcrypt")]
pub type WHEA_ERROR_SOURCE_RECOVER = *mut _WHEA_ERROR_SOURCE_RECOVER;
pub type WHEA_ERROR_SOURCE_STATE = i32;
pub type WHEA_ERROR_SOURCE_TYPE = i32;
pub type WHEA_ERROR_SOURCE_UNINITIALIZE = *mut _WHEA_ERROR_SOURCE_UNINITIALIZE;
pub type WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER = *mut _WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_STATUS {
    pub ErrorStatus: u64,
    pub Anonymous: WHEA_ERROR_STATUS_0,
}
impl Default for WHEA_ERROR_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_STATUS_0 {
    pub _bitfield: u64,
}
pub const WHEA_ERROR_TEXT_LEN: u32 = 32;
pub type WHEA_ERROR_TYPE = i32;
pub const WHEA_ERR_SRC_OVERRIDE_FLAG: u32 = 1073741824;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ETW_OVERFLOW_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RecordId: u64,
}
impl Default for WHEA_ETW_OVERFLOW_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_EVENT_LOG_ENTRY {
    pub Header: WHEA_EVENT_LOG_ENTRY_HEADER,
}
impl Default for WHEA_EVENT_LOG_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_EVENT_LOG_ENTRY_FLAGS {
    pub Anonymous: WHEA_EVENT_LOG_ENTRY_FLAGS_0,
    pub AsULONG: u32,
}
impl Default for WHEA_EVENT_LOG_ENTRY_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_EVENT_LOG_ENTRY_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_EVENT_LOG_ENTRY_HEADER {
    pub Signature: u32,
    pub Version: u32,
    pub Length: u32,
    pub Type: WHEA_EVENT_LOG_ENTRY_TYPE,
    pub OwnerTag: u32,
    pub Id: WHEA_EVENT_LOG_ENTRY_ID,
    pub Flags: WHEA_EVENT_LOG_ENTRY_FLAGS,
    pub PayloadLength: u32,
}
impl Default for WHEA_EVENT_LOG_ENTRY_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_EVENT_LOG_ENTRY_ID = i32;
pub type WHEA_EVENT_LOG_ENTRY_TYPE = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_EXTENDED_RAS {
    pub Flag: u32,
    pub DimmLocation: u32,
    pub SectionLength: u64,
    pub SectionData: [u64; 1],
}
impl Default for WHEA_EXTENDED_RAS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_FAILED_ADD_DEFECT_LIST_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
impl Default for WHEA_FAILED_ADD_DEFECT_LIST_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_FIRMWARE_ERROR_RECORD_REFERENCE {
    pub Type: u8,
    pub Reserved: [u8; 7],
    pub FirmwareRecordId: u64,
}
impl Default for WHEA_FIRMWARE_ERROR_RECORD_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_FIRMWARE_RECORD_TYPE_IPFSAL: u32 = 0;
pub type WHEA_GAS_ERRORS = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_GAS_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Error: WHEA_GAS_ERRORS,
}
impl Default for WHEA_GAS_ERROR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_GENERIC_ENTRY_TEXT_LEN: u32 = 20;
pub const WHEA_GENERIC_ENTRY_V2_VERSION: u32 = 768;
pub const WHEA_GENERIC_ENTRY_VERSION: u32 = 768;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_GENERIC_ERROR {
    pub BlockStatus: WHEA_GENERIC_ERROR_BLOCKSTATUS,
    pub RawDataOffset: u32,
    pub RawDataLength: u32,
    pub DataLength: u32,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub Data: [u8; 1],
}
impl Default for WHEA_GENERIC_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_GENERIC_ERROR_BLOCKSTATUS {
    pub Anonymous: WHEA_GENERIC_ERROR_BLOCKSTATUS_0,
    pub AsULONG: u32,
}
impl Default for WHEA_GENERIC_ERROR_BLOCKSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_GENERIC_ERROR_BLOCKSTATUS_0 {
    pub _bitfield: u32,
}
pub type WHEA_GENERIC_ERROR_DATA_ENTRY = WHEA_GENERIC_ERROR_DATA_ENTRY_V2;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_GENERIC_ERROR_DATA_ENTRY_V1 {
    pub SectionType: windows_sys::core::GUID,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub Revision: WHEA_REVISION,
    pub ValidBits: u8,
    pub Flags: u8,
    pub ErrorDataLength: u32,
    pub FRUId: windows_sys::core::GUID,
    pub FRUText: [u8; 20],
    pub Data: [u8; 1],
}
impl Default for WHEA_GENERIC_ERROR_DATA_ENTRY_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_GENERIC_ERROR_DATA_ENTRY_V2 {
    pub SectionType: windows_sys::core::GUID,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub Revision: WHEA_REVISION,
    pub ValidBits: u8,
    pub Flags: u8,
    pub ErrorDataLength: u32,
    pub FRUId: windows_sys::core::GUID,
    pub FRUText: [u8; 20],
    pub Timestamp: WHEA_TIMESTAMP,
    pub Data: [u8; 1],
}
impl Default for WHEA_GENERIC_ERROR_DATA_ENTRY_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_GENERIC_ERROR_DESCRIPTOR {
    pub Type: u16,
    pub Reserved: u8,
    pub Enabled: u8,
    pub ErrStatusBlockLength: u32,
    pub RelatedErrorSourceId: u32,
    pub ErrStatusAddressSpaceID: u8,
    pub ErrStatusAddressBitWidth: u8,
    pub ErrStatusAddressBitOffset: u8,
    pub ErrStatusAddressAccessSize: u8,
    pub ErrStatusAddress: i64,
    pub Notify: WHEA_NOTIFICATION_DESCRIPTOR,
}
impl Default for WHEA_GENERIC_ERROR_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_GENERIC_ERROR_DESCRIPTOR_V2 {
    pub Type: u16,
    pub Reserved: u8,
    pub Enabled: u8,
    pub ErrStatusBlockLength: u32,
    pub RelatedErrorSourceId: u32,
    pub ErrStatusAddressSpaceID: u8,
    pub ErrStatusAddressBitWidth: u8,
    pub ErrStatusAddressBitOffset: u8,
    pub ErrStatusAddressAccessSize: u8,
    pub ErrStatusAddress: i64,
    pub Notify: WHEA_NOTIFICATION_DESCRIPTOR,
    pub ReadAckAddressSpaceID: u8,
    pub ReadAckAddressBitWidth: u8,
    pub ReadAckAddressBitOffset: u8,
    pub ReadAckAddressAccessSize: u8,
    pub ReadAckAddress: i64,
    pub ReadAckPreserveMask: u64,
    pub ReadAckWriteMask: u64,
}
impl Default for WHEA_GENERIC_ERROR_DESCRIPTOR_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_INJECTION_VENDOR_DEFINED_REASON_CODES = i32;
pub const WHEA_INVALID_ERR_SRC_ID: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_IN_USE_PAGE_NOTIFY_FLAGS {
    pub Bits: WHEA_IN_USE_PAGE_NOTIFY_FLAGS_0,
    pub AsUCHAR: u8,
}
impl Default for WHEA_IN_USE_PAGE_NOTIFY_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WHEA_IN_USE_PAGE_NOTIFY_FLAGS_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_IPF_CMC_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: u8,
    pub Reserved: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_IPF_CPE_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: u8,
    pub Reserved: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_IPF_MCA_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: u8,
    pub Reserved: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_IPMI_LOAD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub IsOnline: bool,
}
impl Default for WHEA_IPMI_LOAD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_IPMI_SUBSCRIBE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub IsRegistered: bool,
}
impl Default for WHEA_IPMI_SUBSCRIBE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_MAX_LOG_DATA_LEN: u32 = 36;
pub const WHEA_MAX_MC_BANKS: u32 = 32;
pub const WHEA_MAX_MC_BANKS64: u32 = 64;
pub const WHEA_MEMERRTYPE_INVALIDADDRESS: u32 = 10;
pub const WHEA_MEMERRTYPE_MASTERABORT: u32 = 6;
pub const WHEA_MEMERRTYPE_MEMORYSPARING: u32 = 12;
pub const WHEA_MEMERRTYPE_MIRRORBROKEN: u32 = 11;
pub const WHEA_MEMERRTYPE_MULTIBITECC: u32 = 3;
pub const WHEA_MEMERRTYPE_MULTISYMCHIPKILL: u32 = 5;
pub const WHEA_MEMERRTYPE_NOERROR: u32 = 1;
pub const WHEA_MEMERRTYPE_PARITYERROR: u32 = 8;
pub const WHEA_MEMERRTYPE_SINGLEBITECC: u32 = 2;
pub const WHEA_MEMERRTYPE_SINGLESYMCHIPKILL: u32 = 4;
pub const WHEA_MEMERRTYPE_TARGETABORT: u32 = 7;
pub const WHEA_MEMERRTYPE_UNKNOWN: u32 = 0;
pub const WHEA_MEMERRTYPE_WATCHDOGTIMEOUT: u32 = 9;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_CORRECTABLE_ERROR_DATA {
    pub ValidBits: WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS,
    pub SocketId: u32,
    pub ChannelId: u32,
    pub DimmSlot: u32,
    pub CorrectableErrorCount: u32,
}
impl Default for WHEA_MEMORY_CORRECTABLE_ERROR_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_CORRECTABLE_ERROR_HEADER {
    pub Version: u16,
    pub Count: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_CORRECTABLE_ERROR_SECTION {
    pub Header: WHEA_MEMORY_CORRECTABLE_ERROR_HEADER,
    pub Data: [WHEA_MEMORY_CORRECTABLE_ERROR_DATA; 1],
}
impl Default for WHEA_MEMORY_CORRECTABLE_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
pub type WHEA_MEMORY_DEFINITION = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_ERROR_EXT_SECTION_AMD {
    pub Flags: WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS,
    pub ValidBits: WHEA_MEMORY_ERROR_EXT_SECTION_AMD_VALIDBITS,
    pub HardwareAddress: WHEA_MEMORY_HARDWARE_ADDRESS_AMD,
    pub Reserved: [u8; 40],
}
impl Default for WHEA_MEMORY_ERROR_EXT_SECTION_AMD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_MEMORY_ERROR_EXT_SECTION_AMD_VALIDBITS {
    pub Anonymous: WHEA_MEMORY_ERROR_EXT_SECTION_AMD_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_MEMORY_ERROR_EXT_SECTION_AMD_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_ERROR_EXT_SECTION_AMD_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS {
    pub Anonymous: WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS_0,
    pub AsUINT64: u64,
}
impl Default for WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_ERROR_EXT_SECTION_INTEL {
    pub Flags: WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS,
    pub ValidBits: WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS,
    pub HardwareAddress: WHEA_MEMORY_HARDWARE_ADDRESS_INTEL,
    pub Reserved: [u8; 40],
}
impl Default for WHEA_MEMORY_ERROR_EXT_SECTION_INTEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS {
    pub Anonymous: WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_ERROR_SECTION {
    pub ValidBits: WHEA_MEMORY_ERROR_SECTION_VALIDBITS,
    pub ErrorStatus: WHEA_ERROR_STATUS,
    pub PhysicalAddress: u64,
    pub PhysicalAddressMask: u64,
    pub Node: u16,
    pub Card: u16,
    pub Module: u16,
    pub Bank: u16,
    pub Device: u16,
    pub Row: u16,
    pub Column: u16,
    pub BitPosition: u16,
    pub RequesterId: u64,
    pub ResponderId: u64,
    pub TargetId: u64,
    pub ErrorType: u8,
    pub Extended: u8,
    pub RankNumber: u16,
    pub CardHandle: u16,
    pub ModuleHandle: u16,
}
impl Default for WHEA_MEMORY_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_MEMORY_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_MEMORY_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_MEMORY_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_HARDWARE_ADDRESS_AMD {
    pub SystemPhysicalAddress: u64,
    pub NormalizedAddress: u64,
    pub UmcBankInstanceId: u64,
    pub SocketNumber: u8,
    pub ChipSelect: u8,
    pub BankGroup: u8,
    pub BankAddress: u8,
    pub RowAddress: u32,
    pub ColumnAddress: u16,
    pub RankMultiplier: u8,
    pub SubChannel: u8,
    pub ChannelId: u8,
    pub Reserved: [u8; 40],
}
impl Default for WHEA_MEMORY_HARDWARE_ADDRESS_AMD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_HARDWARE_ADDRESS_INTEL {
    pub MemDef: WHEA_MEMORY_DEFINITION,
    pub SystemAddress: u64,
    pub SpareSystemAddress: u64,
    pub DevicePhysicalAddress: u64,
    pub ChannelAddress: u64,
    pub RankAddress: u64,
    pub ProcessorSocketId: u8,
    pub MemoryControllerId: u8,
    pub TargetId: u8,
    pub LogicalChannelId: u8,
    pub ChannelId: u8,
    pub SubChannelId: u8,
    pub PhysicalRankId: u8,
    pub DimmSlotId: u8,
    pub DimmRankId: u8,
    pub Bank: u8,
    pub BankGroup: u8,
    pub Row: u32,
    pub Column: u32,
    pub LockStepRank: u8,
    pub LockStepPhysicalRank: u8,
    pub LockStepBank: u8,
    pub LockStepBankGroup: u8,
    pub ChipSelect: u8,
    pub Node: u8,
    pub ChipId: u8,
    pub Reserved: [u8; 40],
}
impl Default for WHEA_MEMORY_HARDWARE_ADDRESS_INTEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_RANGE {
    pub StartSystemPhysicalAddress: u64,
    pub LengthInBytes: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_RANGE_ERROR_SECTION {
    pub ValidBits: WHEA_MEMORY_RANGE_ERROR_SECTION_VALIDBITS,
    pub Version: u32,
    pub DeviceInfo: WHEA_DEVICE_INFO,
    pub DeviceType: u8,
    pub Reserved: u8,
    pub RangeCount: u16,
    pub Ranges: [WHEA_MEMORY_RANGE; 1],
}
impl Default for WHEA_MEMORY_RANGE_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_MEMORY_RANGE_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcc245308_61d5_4356_a707_25172ec2f5ea);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_MEMORY_RANGE_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_MEMORY_RANGE_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_MEMORY_RANGE_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_RANGE_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
pub const WHEA_MEMORY_RANGE_ERROR_SECTION_VERSION: u32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_THROTTLE_SUMMARY_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Status: super::super::Win32::bcrypt::NTSTATUS,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEA_MEMORY_THROTTLE_SUMMARY_FAILED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_MEM_PERSISTOFFLINE: u32 = 1;
pub const WHEA_MEM_PFA_DISABLE: u32 = 2;
pub const WHEA_MEM_PFA_PAGECOUNT: u32 = 3;
pub const WHEA_MEM_PFA_THRESHOLD: u32 = 4;
pub const WHEA_MEM_PFA_TIMEOUT: u32 = 5;
pub const WHEA_MSCHECK_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x48ab7f57_dc34_4f6c_a7d3_b0b5b0a74314);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_MSR_DUMP_SECTION {
    pub MsrDumpBuffer: u8,
    pub MsrDumpLength: u32,
    pub MsrDumpData: [u8; 1],
}
impl Default for WHEA_MSR_DUMP_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_NMI_ERROR_SECTION {
    pub Data: [u8; 8],
    pub Flags: WHEA_NMI_ERROR_SECTION_FLAGS,
}
impl Default for WHEA_NMI_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_NMI_ERROR_SECTION_FLAGS {
    pub Anonymous: WHEA_NMI_ERROR_SECTION_FLAGS_0,
    pub AsULONG: u32,
}
impl Default for WHEA_NMI_ERROR_SECTION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NMI_ERROR_SECTION_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR {
    pub Type: u8,
    pub Length: u8,
    pub Flags: WHEA_NOTIFICATION_FLAGS,
    pub u: WHEA_NOTIFICATION_DESCRIPTOR_0,
}
impl Default for WHEA_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_NOTIFICATION_DESCRIPTOR_0 {
    pub Polled: WHEA_NOTIFICATION_DESCRIPTOR_0_0,
    pub Interrupt: WHEA_NOTIFICATION_DESCRIPTOR_0_1,
    pub LocalInterrupt: WHEA_NOTIFICATION_DESCRIPTOR_0_2,
    pub Sci: WHEA_NOTIFICATION_DESCRIPTOR_0_3,
    pub Nmi: WHEA_NOTIFICATION_DESCRIPTOR_0_4,
    pub Sea: WHEA_NOTIFICATION_DESCRIPTOR_0_5,
    pub Sei: WHEA_NOTIFICATION_DESCRIPTOR_0_6,
    pub Gsiv: WHEA_NOTIFICATION_DESCRIPTOR_0_7,
}
impl Default for WHEA_NOTIFICATION_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_0 {
    pub PollInterval: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_1 {
    pub PollInterval: u32,
    pub Vector: u32,
    pub SwitchToPollingThreshold: u32,
    pub SwitchToPollingWindow: u32,
    pub ErrorThreshold: u32,
    pub ErrorThresholdWindow: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_2 {
    pub PollInterval: u32,
    pub Vector: u32,
    pub SwitchToPollingThreshold: u32,
    pub SwitchToPollingWindow: u32,
    pub ErrorThreshold: u32,
    pub ErrorThresholdWindow: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_3 {
    pub PollInterval: u32,
    pub Vector: u32,
    pub SwitchToPollingThreshold: u32,
    pub SwitchToPollingWindow: u32,
    pub ErrorThreshold: u32,
    pub ErrorThresholdWindow: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_4 {
    pub PollInterval: u32,
    pub Vector: u32,
    pub SwitchToPollingThreshold: u32,
    pub SwitchToPollingWindow: u32,
    pub ErrorThreshold: u32,
    pub ErrorThresholdWindow: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_5 {
    pub PollInterval: u32,
    pub Vector: u32,
    pub SwitchToPollingThreshold: u32,
    pub SwitchToPollingWindow: u32,
    pub ErrorThreshold: u32,
    pub ErrorThresholdWindow: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_6 {
    pub PollInterval: u32,
    pub Vector: u32,
    pub SwitchToPollingThreshold: u32,
    pub SwitchToPollingWindow: u32,
    pub ErrorThreshold: u32,
    pub ErrorThresholdWindow: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_7 {
    pub PollInterval: u32,
    pub Vector: u32,
    pub SwitchToPollingThreshold: u32,
    pub SwitchToPollingWindow: u32,
    pub ErrorThreshold: u32,
    pub ErrorThresholdWindow: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_NOTIFICATION_FLAGS {
    pub Anonymous: WHEA_NOTIFICATION_FLAGS_0,
    pub AsUSHORT: u16,
}
impl Default for WHEA_NOTIFICATION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_FLAGS_0 {
    pub _bitfield: u16,
}
pub const WHEA_NOTIFICATION_TYPE_ARMV8_SEA: u32 = 8;
pub const WHEA_NOTIFICATION_TYPE_ARMV8_SEI: u32 = 9;
pub const WHEA_NOTIFICATION_TYPE_CMCI: u32 = 5;
pub const WHEA_NOTIFICATION_TYPE_EXTERNALINTERRUPT: u32 = 1;
pub const WHEA_NOTIFICATION_TYPE_EXTERNALINTERRUPT_GSIV: u32 = 10;
pub const WHEA_NOTIFICATION_TYPE_GPIO_SIGNAL: u32 = 7;
pub const WHEA_NOTIFICATION_TYPE_LOCALINTERRUPT: u32 = 2;
pub const WHEA_NOTIFICATION_TYPE_MCE: u32 = 6;
pub const WHEA_NOTIFICATION_TYPE_NMI: u32 = 4;
pub const WHEA_NOTIFICATION_TYPE_POLLED: u32 = 0;
pub const WHEA_NOTIFICATION_TYPE_SCI: u32 = 3;
pub const WHEA_NOTIFICATION_TYPE_SDEI: u32 = 11;
pub const WHEA_NOTIFY_ALL_OFFLINES: u32 = 16;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_OFFLINE_DONE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Address: u64,
}
impl Default for WHEA_OFFLINE_DONE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_OFFLINE_ERRS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_PACKET_LOG_DATA {
    pub LogData: [u8; 36],
    pub ExtraBytes: [u8; 36],
    pub BcParam3: usize,
    pub BcParam4: usize,
    pub LogDataLength: u32,
    pub LogTag: u16,
    pub Reserved: u16,
    pub Flags: WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS,
}
impl Default for WHEA_PACKET_LOG_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS {
    pub Anonymous: WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS_0,
    pub AsULONG: u32,
}
impl Default for WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS_0 {
    pub BridgeSecondaryStatus: u16,
    pub BridgeControl: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIEXPRESS_COMMAND_STATUS {
    pub Anonymous: WHEA_PCIEXPRESS_COMMAND_STATUS_0,
    pub AsULONG: u32,
}
impl Default for WHEA_PCIEXPRESS_COMMAND_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIEXPRESS_COMMAND_STATUS_0 {
    pub Command: u16,
    pub Status: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIEXPRESS_DEVICE_ID {
    pub VendorID: u16,
    pub DeviceID: u16,
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub _bitfield3: u32,
}
pub type WHEA_PCIEXPRESS_DEVICE_TYPE = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIEXPRESS_ERROR_SECTION {
    pub ValidBits: WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS,
    pub PortType: WHEA_PCIEXPRESS_DEVICE_TYPE,
    pub Version: WHEA_PCIEXPRESS_VERSION,
    pub CommandStatus: WHEA_PCIEXPRESS_COMMAND_STATUS,
    pub Reserved: u32,
    pub DeviceId: WHEA_PCIEXPRESS_DEVICE_ID,
    pub DeviceSerialNumber: u64,
    pub BridgeControlStatus: WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS,
    pub ExpressCapability: [u8; 60],
    pub AerInfo: [u8; 96],
}
impl Default for WHEA_PCIEXPRESS_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIEXPRESS_VERSION {
    pub Anonymous: WHEA_PCIEXPRESS_VERSION_0,
    pub AsULONG: u32,
}
impl Default for WHEA_PCIEXPRESS_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIEXPRESS_VERSION_0 {
    pub MinorVersion: u8,
    pub MajorVersion: u8,
    pub Reserved: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIE_ADDRESS {
    pub Segment: u32,
    pub Bus: u32,
    pub Device: u32,
    pub Function: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIE_CORRECTABLE_ERROR_DEVICES {
    pub ValidBits: WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS,
    pub Address: WHEA_PCIE_ADDRESS,
    pub Mask: u32,
    pub CorrectableErrorCount: [u32; 32],
}
impl Default for WHEA_PCIE_CORRECTABLE_ERROR_DEVICES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS {
    pub Anonymous: WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_PCIE_CORRECTABLE_ERROR_SECTION {
    pub Header: WHEA_PCIE_CORRECTABLE_ERROR_SECTION_HEADER,
    pub Devices: [WHEA_PCIE_CORRECTABLE_ERROR_DEVICES; 1],
}
impl Default for WHEA_PCIE_CORRECTABLE_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_PCIE_CORRECTABLE_ERROR_SECTION_COUNT_SIZE: u32 = 32;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIE_CORRECTABLE_ERROR_SECTION_HEADER {
    pub Version: u16,
    pub Count: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIXBUS_COMMAND {
    pub Anonymous: WHEA_PCIXBUS_COMMAND_0,
    pub AsULONGLONG: u64,
}
impl Default for WHEA_PCIXBUS_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIXBUS_COMMAND_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIXBUS_ERROR_SECTION {
    pub ValidBits: WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS,
    pub ErrorStatus: WHEA_ERROR_STATUS,
    pub ErrorType: u16,
    pub BusId: WHEA_PCIXBUS_ID,
    pub Reserved: u32,
    pub BusAddress: u64,
    pub BusData: u64,
    pub BusCommand: WHEA_PCIXBUS_COMMAND,
    pub RequesterId: u64,
    pub CompleterId: u64,
    pub TargetId: u64,
}
impl Default for WHEA_PCIXBUS_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIXBUS_ID {
    pub Anonymous: WHEA_PCIXBUS_ID_0,
    pub AsUSHORT: u16,
}
impl Default for WHEA_PCIXBUS_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIXBUS_ID_0 {
    pub BusNumber: u8,
    pub BusSegment: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIXDEVICE_ERROR_SECTION {
    pub ValidBits: WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS,
    pub ErrorStatus: WHEA_ERROR_STATUS,
    pub IdInfo: WHEA_PCIXDEVICE_ID,
    pub MemoryNumber: u32,
    pub IoNumber: u32,
    pub RegisterDataPairs: [WHEA_PCIXDEVICE_REGISTER_PAIR; 1],
}
impl Default for WHEA_PCIXDEVICE_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIXDEVICE_ID {
    pub VendorId: u16,
    pub DeviceId: u16,
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub Reserved2: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIXDEVICE_REGISTER_PAIR {
    pub Register: u64,
    pub Data: u64,
}
#[cfg(feature = "Wdk_wdm")]
pub type WHEA_PCI_DPC_SECTION = PCI_EXPRESS_DPC_CAPABILITY;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCI_RECOVERY_SECTION {
    pub SignalType: u8,
    pub RecoveryAttempted: bool,
    pub RecoveryStatus: u8,
}
pub type WHEA_PCI_RECOVERY_SIGNAL = i32;
pub type WHEA_PCI_RECOVERY_STATUS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_PCI_SLOT_NUMBER {
    pub u: WHEA_PCI_SLOT_NUMBER_0,
}
impl Default for WHEA_PCI_SLOT_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCI_SLOT_NUMBER_0 {
    pub bits: WHEA_PCI_SLOT_NUMBER_0_0,
    pub AsULONG: u32,
}
impl Default for WHEA_PCI_SLOT_NUMBER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCI_SLOT_NUMBER_0_0 {
    pub _bitfield: u32,
}
pub const WHEA_PENDING_PAGE_LIST_SZ: u32 = 13;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PERSISTENCE_INFO {
    pub Anonymous: WHEA_PERSISTENCE_INFO_0,
    pub AsULONGLONG: u64,
}
impl Default for WHEA_PERSISTENCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PERSISTENCE_INFO_0 {
    pub _bitfield: u64,
}
pub const WHEA_PFA_PAGE_RANGE_MAX: u32 = 256;
pub type WHEA_PFA_REMOVE_TRIGGER = i32;
pub const WHEA_PLUGIN_REGISTRATION_PACKET_V1: u32 = 65536;
pub const WHEA_PLUGIN_REGISTRATION_PACKET_V2: u32 = 131072;
pub const WHEA_PLUGIN_REGISTRATION_PACKET_VERSION: u32 = 131072;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PMEM_ERROR_SECTION {
    pub ValidBits: WHEA_PMEM_ERROR_SECTION_VALIDBITS,
    pub LocationInfo: [u8; 64],
    pub ErrorStatus: WHEA_ERROR_STATUS,
    pub NFITHandle: u32,
    pub PageRangeCount: u32,
    pub PageRange: [WHEA_PMEM_PAGE_RANGE; 1],
}
impl Default for WHEA_PMEM_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_PMEM_ERROR_SECTION_LOCATION_INFO_SIZE: u32 = 64;
pub const WHEA_PMEM_ERROR_SECTION_MAX_PAGES: u32 = 50;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PMEM_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PMEM_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_PMEM_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PMEM_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PMEM_PAGE_RANGE {
    pub StartingPfn: u64,
    pub PageCount: u64,
    pub MarkedBadBitmap: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_ADDRESS_TRANSLATION_BUFFER_INTEL {
    pub SwSmi: u32,
    pub Command: u32,
    pub Status: u32,
    pub SystemAddress: u64,
    pub NmSystemAddress: u64,
    pub SpareSystemAddress: u64,
    pub DevicePhysicalAddress: u64,
    pub ProcessorSocketId: u64,
    pub MemoryControllerId: u64,
    pub NmMemoryControllerId: u64,
    pub TargetId: u64,
    pub LogicalChannelId: u64,
    pub ChannelAddress: u64,
    pub NmChannelAddress: u64,
    pub ChannelId: u64,
    pub NmChannelId: u64,
    pub RankAddress: u64,
    pub NmRankAddress: u64,
    pub PhysicalRankId: u64,
    pub NmPhysicalRankId: u64,
    pub DimmSlotId: u64,
    pub NmDimmSlotId: u64,
    pub DimmRankId: u64,
    pub Row: u64,
    pub NmRow: u64,
    pub Column: u64,
    pub NmColumn: u64,
    pub Bank: u64,
    pub NmBank: u64,
    pub BankGroup: u64,
    pub NmBankGroup: u64,
    pub LockStepRank: u64,
    pub LockStepPhysicalRank: u64,
    pub LockStepBank: u64,
    pub LockStepBankGroup: u64,
    pub ChipSelect: u64,
    pub NmChipSelect: u64,
    pub Node: u64,
    pub ChipId: u64,
    pub NmChipId: u64,
}
pub const WHEA_PRM_ADDRESS_TRANSLATION_INTEL_FORWARD_ADDRESS_TRANSLATE: u32 = 2;
pub const WHEA_PRM_ADDRESS_TRANSLATION_INTEL_GET_ADDRESS_PARAMETERS: u32 = 1;
pub const WHEA_PRM_ADDRESS_TRANSLATION_INTEL_INTERNAL_ERROR: u32 = 3;
pub const WHEA_PRM_ADDRESS_TRANSLATION_INTEL_INVALID_COMMAND: u32 = 2;
pub const WHEA_PRM_ADDRESS_TRANSLATION_INTEL_REVERSE_ADDRESS_TRANSLATE: u32 = 3;
pub const WHEA_PRM_ADDRESS_TRANSLATION_INTEL_SUCCESS: u32 = 0;
pub const WHEA_PRM_ADDRESS_TRANSLATION_INTEL_UNKNOWN_FAILURE: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_DRAM_TO_NORMALIZED_OUT_BUFFER_AMD {
    pub NormalizedAddress: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PRM_DRAM_TO_NORMALIZED_PARAM_BUFFER_AMD {
    pub SocketNumber: u8,
    pub UmcBankInstanceId: u64,
    pub ChipSelect: u8,
    pub BankGroup: u8,
    pub BankAddress: u8,
    pub RowAddress: u32,
    pub ColumnAddress: u16,
    pub RankMultiplier: u8,
    pub SubChannel: u8,
    pub OutputBuffer: PWHEA_PRM_DRAM_TO_NORMALIZED_OUT_BUFFER_AMD,
}
impl Default for WHEA_PRM_DRAM_TO_NORMALIZED_PARAM_BUFFER_AMD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_DRAM_TO_SPA_OUT_BUFFER_AMD {
    pub SystemPhysicalAddress: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PRM_DRAM_TO_SPA_PARAM_BUFFER_AMD {
    pub SocketNumber: u8,
    pub UmcBankInstanceId: u64,
    pub ChipSelect: u8,
    pub BankGroup: u8,
    pub BankAddress: u8,
    pub RowAddress: u32,
    pub ColumnAddress: u16,
    pub RankMultiplier: u8,
    pub SubChannel: u8,
    pub OutputBuffer: PWHEA_PRM_DRAM_TO_SPA_OUT_BUFFER_AMD,
}
impl Default for WHEA_PRM_DRAM_TO_SPA_PARAM_BUFFER_AMD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_NORMALIZED_TO_DRAM_OUT_BUFFER_AMD {
    pub ChipSelect: u8,
    pub BankGroup: u8,
    pub BankAddress: u8,
    pub RowAddress: u32,
    pub ColumnAddress: u16,
    pub RankMultiplier: u8,
    pub SubChannel: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PRM_NORMALIZED_TO_DRAM_PARAM_BUFFER_AMD {
    pub NormalizedAddress: u64,
    pub SocketNumber: u8,
    pub UmcBankInstanceId: u64,
    pub OutputBuffer: PWHEA_PRM_NORMALIZED_TO_DRAM_OUT_BUFFER_AMD,
}
impl Default for WHEA_PRM_NORMALIZED_TO_DRAM_PARAM_BUFFER_AMD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_NORMALIZED_TO_SPA_OUT_BUFFER_AMD {
    pub SystemPhysicalAddress: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PRM_NORMALIZED_TO_SPA_PARAM_BUFFER_AMD {
    pub NormalizedAddress: u64,
    pub SocketNumber: u8,
    pub UmcBankInstanceId: u64,
    pub OutputBuffer: PWHEA_PRM_NORMALIZED_TO_SPA_OUT_BUFFER_AMD,
}
impl Default for WHEA_PRM_NORMALIZED_TO_SPA_PARAM_BUFFER_AMD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_SPA_TO_DRAM_OUT_BUFFER_AMD {
    pub ChipSelect: u8,
    pub BankGroup: u8,
    pub BankAddress: u8,
    pub RowAddress: u32,
    pub ColumnAddress: u16,
    pub RankMultiplier: u8,
    pub SubChannel: u8,
    pub SocketNumber: u8,
    pub UmcBankInstanceId: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PRM_SPA_TO_DRAM_PARAM_BUFFER_AMD {
    pub SystemPhysicalAddress: u64,
    pub OutputBuffer: PWHEA_PRM_SPA_TO_DRAM_OUT_BUFFER_AMD,
}
impl Default for WHEA_PRM_SPA_TO_DRAM_PARAM_BUFFER_AMD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_SPA_TO_NORMALIZED_OUT_BUFFER_AMD {
    pub NormalizedAddress: u64,
    pub SocketNumber: u8,
    pub UmcBankInstanceId: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PRM_SPA_TO_NORMALIZED_PARAM_BUFFER_AMD {
    pub SystemPhysicalAddress: u64,
    pub OutputBuffer: PWHEA_PRM_SPA_TO_NORMALIZED_OUT_BUFFER_AMD,
}
impl Default for WHEA_PRM_SPA_TO_NORMALIZED_PARAM_BUFFER_AMD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PROCESSOR_FAMILY_INFO {
    pub Anonymous: WHEA_PROCESSOR_FAMILY_INFO_0,
    pub AsULONGLONG: u64,
}
impl Default for WHEA_PROCESSOR_FAMILY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PROCESSOR_FAMILY_INFO_0 {
    pub _bitfield: u32,
    pub NativeModelId: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PROCESSOR_GENERIC_ERROR_SECTION {
    pub ValidBits: WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS,
    pub ProcessorType: u8,
    pub InstructionSet: u8,
    pub ErrorType: u8,
    pub Operation: u8,
    pub Flags: u8,
    pub Level: u8,
    pub Reserved: u16,
    pub CPUVersion: u64,
    pub CPUBrandString: [u8; 128],
    pub ProcessorId: u64,
    pub TargetAddress: u64,
    pub RequesterId: u64,
    pub ResponderId: u64,
    pub InstructionPointer: u64,
}
impl Default for WHEA_PROCESSOR_GENERIC_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PI_CPUID {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub CpuVendor: u32,
    pub CpuFamily: u32,
    pub CpuModel: u32,
    pub CpuStepping: u32,
    pub NumBanks: u32,
}
impl Default for WHEA_PSHED_PI_CPUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PI_CPU_BUSES_INIT_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Status: super::super::Win32::bcrypt::NTSTATUS,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEA_PSHED_PI_CPU_BUSES_INIT_FAILED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PI_ERROR_RECORD_FULL_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PlatformName: [i8; 32],
}
impl Default for WHEA_PSHED_PI_ERROR_RECORD_FULL_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PI_SERVER_TYPE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PlatformName: [i8; 32],
    pub ServerType: [i8; 32],
}
impl Default for WHEA_PSHED_PI_SERVER_TYPE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PI_TRACE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Buffer: [super::super::Win32::winnt::CCHAR; 256],
}
#[cfg(feature = "Win32_winnt")]
impl Default for WHEA_PSHED_PI_TRACE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_CALLBACKS {
    pub GetAllErrorSources: PSHED_PI_GET_ALL_ERROR_SOURCES,
    pub Reserved: *mut core::ffi::c_void,
    pub GetErrorSourceInfo: PSHED_PI_GET_ERROR_SOURCE_INFO,
    pub SetErrorSourceInfo: PSHED_PI_SET_ERROR_SOURCE_INFO,
    pub EnableErrorSource: PSHED_PI_ENABLE_ERROR_SOURCE,
    pub DisableErrorSource: PSHED_PI_DISABLE_ERROR_SOURCE,
    pub WriteErrorRecord: PSHED_PI_WRITE_ERROR_RECORD,
    pub ReadErrorRecord: PSHED_PI_READ_ERROR_RECORD,
    pub ClearErrorRecord: PSHED_PI_CLEAR_ERROR_RECORD,
    pub RetrieveErrorInfo: PSHED_PI_RETRIEVE_ERROR_INFO,
    pub FinalizeErrorRecord: PSHED_PI_FINALIZE_ERROR_RECORD,
    pub ClearErrorStatus: PSHED_PI_CLEAR_ERROR_STATUS,
    pub AttemptRecovery: PSHED_PI_ATTEMPT_ERROR_RECOVERY,
    pub GetInjectionCapabilities: PSHED_PI_GET_INJECTION_CAPABILITIES,
    pub InjectError: PSHED_PI_INJECT_ERROR,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for WHEA_PSHED_PLUGIN_CALLBACKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_DIMM_MISMATCH {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub FirmwareBank: u16,
    pub FirmwareCol: u16,
    pub FirmwareRow: u16,
    pub RetryRdBank: u16,
    pub RetryRdCol: u16,
    pub RetryRdRow: u16,
    pub TaBank: u16,
    pub TaCol: u16,
    pub TaRow: u16,
}
impl Default for WHEA_PSHED_PLUGIN_DIMM_MISMATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub EnableError: WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS,
}
impl Default for WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_FAILED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_HEARTBEAT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
impl Default for WHEA_PSHED_PLUGIN_HEARTBEAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_INIT_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Status: super::super::Win32::bcrypt::NTSTATUS,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEA_PSHED_PLUGIN_INIT_FAILED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_LOAD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PluginName: [u16; 32],
    pub MajorVersion: u32,
    pub MinorVersion: u32,
}
impl Default for WHEA_PSHED_PLUGIN_LOAD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_PLATFORM_SUPPORT_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PluginName: [u16; 32],
    pub Supported: bool,
}
impl Default for WHEA_PSHED_PLUGIN_PLATFORM_SUPPORT_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type WHEA_PSHED_PLUGIN_REGISTRATION_PACKET = WHEA_PSHED_PLUGIN_REGISTRATION_PACKET_V2;
#[repr(C)]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_REGISTRATION_PACKET_V1 {
    pub Length: u32,
    pub Version: u32,
    pub Context: *mut core::ffi::c_void,
    pub FunctionalAreaMask: u32,
    pub Reserved: u32,
    pub Callbacks: WHEA_PSHED_PLUGIN_CALLBACKS,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for WHEA_PSHED_PLUGIN_REGISTRATION_PACKET_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_REGISTRATION_PACKET_V2 {
    pub Length: u32,
    pub Version: u32,
    pub Context: *mut core::ffi::c_void,
    pub FunctionalAreaMask: u32,
    pub Reserved: u32,
    pub Callbacks: WHEA_PSHED_PLUGIN_CALLBACKS,
    pub PluginHandle: *mut core::ffi::c_void,
}
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
impl Default for WHEA_PSHED_PLUGIN_REGISTRATION_PACKET_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_UNLOAD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PluginName: [u16; 32],
}
impl Default for WHEA_PSHED_PLUGIN_UNLOAD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_RAW_DATA_FORMAT = i32;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEA_READ_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Offset: u32,
    pub BytesToRead: u32,
    pub OutputBufferLength: u32,
    pub Status: super::super::Win32::bcrypt::NTSTATUS,
    pub Reason: u32,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEA_READ_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_RECORD_CREATOR_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcf07c4bd_b789_4e18_b3c4_1f732cb57131);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_RECOVERY_ACTION {
    pub Anonymous: WHEA_RECOVERY_ACTION_0,
    pub AsUINT64: u64,
}
impl Default for WHEA_RECOVERY_ACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_RECOVERY_ACTION_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEA_RECOVERY_CONTEXT {
    pub Anonymous: WHEA_RECOVERY_CONTEXT_0,
    pub PartitionId: u64,
    pub VpIndex: u32,
    pub ErrorType: WHEA_RECOVERY_CONTEXT_ERROR_TYPE,
    pub PageCount: u32,
    pub PageInfo: [WHEA_RECOVERY_CONTEXT_PAGE_INFO; 256],
    pub ChildPartitionId: u64,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEA_RECOVERY_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub union WHEA_RECOVERY_CONTEXT_0 {
    pub MemoryError: WHEA_RECOVERY_CONTEXT_0_0,
    pub PmemError: WHEA_RECOVERY_CONTEXT_0_1,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEA_RECOVERY_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_RECOVERY_CONTEXT_0_0 {
    pub Address: usize,
    pub Consumed: bool,
    pub ErrorCode: u16,
    pub ErrorIpValid: bool,
    pub RestartIpValid: bool,
    pub ClearPoison: bool,
}
#[repr(C)]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_RECOVERY_CONTEXT_0_1 {
    pub PmemErrInfo: usize,
}
pub type WHEA_RECOVERY_CONTEXT_ACTION_TAKEN = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO {
    pub Bits: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO_0,
    pub AsULONG64: u64,
}
impl Default for WHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO_0 {
    pub _bitfield: u64,
}
pub type WHEA_RECOVERY_CONTEXT_ERROR_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEA_RECOVERY_CONTEXT_PAGE_INFO {
    pub ComponentTag: u32,
    pub PageStatus: super::super::Win32::bcrypt::NTSTATUS,
    pub ActionTaken: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN,
    pub NotifyFlags: WHEA_IN_USE_PAGE_NOTIFY_FLAGS,
    pub ImmediateSuccess: bool,
    pub Reserved: u16,
    pub ActionTakenAdditionalInfo: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEA_RECOVERY_CONTEXT_PAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_RECOVERY_FAILURE_REASON = i32;
pub type WHEA_RECOVERY_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_REGISTER_KEY_NOTIFICATION_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
impl Default for WHEA_REGISTER_KEY_NOTIFICATION_FAILED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_REGISTRY_ERRORS = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_REGISTRY_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RegErr: WHEA_REGISTRY_ERRORS,
    pub Status: u32,
}
impl Default for WHEA_REGISTRY_ERROR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct WHEA_REGNOTIFY_POLICY_CHANGE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PolicyName: [super::super::Win32::winnt::CCHAR; 32],
    pub PolicyIndex: u32,
    pub PolicyValue: u32,
}
#[cfg(feature = "Win32_winnt")]
impl Default for WHEA_REGNOTIFY_POLICY_CHANGE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS {
    pub Anonymous: WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS_0,
    pub AsULONG: u32,
}
impl Default for WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS_0 {
    pub _bitfield: u32,
}
pub const WHEA_RESTORE_CMCI_ATTEMPTS: u32 = 8;
pub const WHEA_RESTORE_CMCI_ENABLED: u32 = 7;
pub const WHEA_RESTORE_CMCI_ERR_LIMIT: u32 = 9;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_REVISION {
    pub Anonymous: WHEA_REVISION_0,
    pub AsUSHORT: u16,
}
impl Default for WHEA_REVISION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WHEA_REVISION_0 {
    pub MinorRevision: u8,
    pub MajorRevision: u8,
}
pub const WHEA_ROW_FAIL_CHECK_ENABLE: u32 = 18;
pub const WHEA_ROW_FAIL_CHECK_EXTENT: u32 = 17;
pub const WHEA_ROW_FAIL_CHECK_THRESHOLD: u32 = 19;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_SEA_SECTION {
    pub Esr: u32,
    pub Far: u64,
    pub Par: u64,
    pub WasKernel: bool,
}
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_CONTAINMENTWRN: u32 = 2;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_FRU_TEXT_BY_PLUGIN: u32 = 128;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_LATENTERROR: u32 = 32;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_PRIMARY: u32 = 1;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_PROPAGATED: u32 = 64;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_RESET: u32 = 4;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_RESOURCENA: u32 = 16;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_THRESHOLDEXCEEDED: u32 = 8;
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_SEI_SECTION {
    pub Esr: u32,
    pub Far: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_PROGRESS {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub BugCheckCode: u32,
    pub BugCheckProgressSummary: u32,
}
impl Default for WHEA_SEL_BUGCHECK_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_MULTIPLE_BUGCHECK_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub IsBugcheckOwner: bool,
    pub RecursionCount: u8,
    pub IsBugcheckRecoveryOwner: bool,
}
impl Default for WHEA_SEL_BUGCHECK_RECOVERY_STATUS_MULTIPLE_BUGCHECK_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Success: bool,
    pub Version: u8,
    pub EntryCount: u16,
    pub Data: WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT_0,
}
impl Default for WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT_0 {
    pub DumpPolicy: u8,
    pub Reserved: [u8; 3],
}
impl Default for WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_VERSION: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE2_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub BootId: u32,
    pub Success: bool,
}
impl Default for WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE2_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Wdk_ntdef")]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_START_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub StartingIrql: super::ntdef::KIRQL,
}
#[cfg(feature = "Wdk_ntdef")]
impl Default for WHEA_SEL_BUGCHECK_RECOVERY_STATUS_START_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_RAW_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Payload: [u8; 256],
}
impl Default for WHEA_SEL_RAW_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_SIGNAL_HANDLER_OVERRIDE_CALLBACK = *mut _WHEA_SIGNAL_HANDLER_OVERRIDE_CALLBACK;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEA_SRAR_DETAIL_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RecoveryContextFlags: u32,
    pub RecoveryContextPa: u64,
    pub PageOfflineStatus: super::super::Win32::bcrypt::NTSTATUS,
    pub KernelConsumerError: bool,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEA_SRAR_DETAIL_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_SRAS_TABLE_ENTRIES_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub LogNumber: u32,
    pub NumberSignals: u32,
    pub Data: [u8; 1],
}
impl Default for WHEA_SRAS_TABLE_ENTRIES_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_SRAS_TABLE_ERROR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
impl Default for WHEA_SRAS_TABLE_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_SRAS_TABLE_NOT_FOUND {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
impl Default for WHEA_SRAS_TABLE_NOT_FOUND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_ADD_ERR_SRC_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
impl Default for WHEA_THROTTLE_ADD_ERR_SRC_FAILED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_MEMORY_ADD_OR_REMOVE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub SocketId: u32,
    pub ChannelId: u32,
    pub DimmSlot: u32,
}
impl Default for WHEA_THROTTLE_MEMORY_ADD_OR_REMOVE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_PCIE_ADD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Address: WHEA_PCIE_ADDRESS,
    pub Mask: u32,
    pub Updated: bool,
    pub Status: super::super::Win32::bcrypt::NTSTATUS,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEA_THROTTLE_PCIE_ADD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_PCIE_REMOVE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Address: WHEA_PCIE_ADDRESS,
    pub Mask: u32,
}
impl Default for WHEA_THROTTLE_PCIE_REMOVE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_REGISTRY_CORRUPT_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ThrottleType: WHEA_THROTTLE_TYPE,
}
impl Default for WHEA_THROTTLE_REGISTRY_CORRUPT_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_REG_DATA_IGNORED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ThrottleType: WHEA_THROTTLE_TYPE,
}
impl Default for WHEA_THROTTLE_REG_DATA_IGNORED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_THROTTLE_TYPE = i32;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_TIMESTAMP {
    pub Anonymous: WHEA_TIMESTAMP_0,
    pub AsLARGE_INTEGER: i64,
}
impl Default for WHEA_TIMESTAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_TIMESTAMP_0 {
    pub _bitfield: u64,
}
pub const WHEA_TLBCHECK_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfc06b535_5e1f_4562_9f25_0a3b9adb63c3);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_VERSION_MISMATCH_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ComponentName: [u16; 32],
}
impl Default for WHEA_VERSION_MISMATCH_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_WRITE_FLAG_DUMMY: u32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "Win32_bcrypt")]
#[derive(Clone, Copy)]
pub struct WHEA_WRITE_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Offset: u32,
    pub InputLength: u32,
    pub Status: super::super::Win32::bcrypt::NTSTATUS,
    pub Reason: u32,
}
#[cfg(feature = "Win32_bcrypt")]
impl Default for WHEA_WRITE_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WHEA_X64_REGISTER_STATE {
    pub Rax: u64,
    pub Rbx: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub Rbp: u64,
    pub Rsp: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,
    pub Cs: u16,
    pub Ds: u16,
    pub Ss: u16,
    pub Es: u16,
    pub Fs: u16,
    pub Gs: u16,
    pub Reserved: u32,
    pub Rflags: u64,
    pub Eip: u64,
    pub Cr0: u64,
    pub Cr1: u64,
    pub Cr2: u64,
    pub Cr3: u64,
    pub Cr4: u64,
    pub Cr8: u64,
    pub Gdtr: WHEA128A,
    pub Idtr: WHEA128A,
    pub Ldtr: u16,
    pub Tr: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_X86_REGISTER_STATE {
    pub Eax: u32,
    pub Ebx: u32,
    pub Ecx: u32,
    pub Edx: u32,
    pub Esi: u32,
    pub Edi: u32,
    pub Ebp: u32,
    pub Esp: u32,
    pub Cs: u16,
    pub Ds: u16,
    pub Ss: u16,
    pub Es: u16,
    pub Fs: u16,
    pub Gs: u16,
    pub Eflags: u32,
    pub Eip: u32,
    pub Cr0: u32,
    pub Cr1: u32,
    pub Cr2: u32,
    pub Cr3: u32,
    pub Cr4: u32,
    pub Gdtr: u64,
    pub Idtr: u64,
    pub Ldtr: u16,
    pub Tr: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_BUS_CHECK {
    pub Anonymous: WHEA_XPF_BUS_CHECK_0,
    pub XpfBusCheck: u64,
}
impl Default for WHEA_XPF_BUS_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_BUS_CHECK_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_CACHE_CHECK {
    pub Anonymous: WHEA_XPF_CACHE_CHECK_0,
    pub XpfCacheCheck: u64,
}
impl Default for WHEA_XPF_CACHE_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_CACHE_CHECK_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_CMC_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: bool,
    pub NumberOfBanks: u8,
    pub Reserved: u32,
    pub Notify: WHEA_NOTIFICATION_DESCRIPTOR,
    pub Banks: [WHEA_XPF_MC_BANK_DESCRIPTOR; 32],
}
impl Default for WHEA_XPF_CMC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_CMC_DESCRIPTOR_V2 {
    pub Type: u16,
    pub Enabled: bool,
    pub NumberOfBanks: u8,
    pub Reserved: u32,
    pub Notify: WHEA_NOTIFICATION_DESCRIPTOR,
    pub Banks: [WHEA_XPF_MC_BANK_DESCRIPTOR; 64],
}
impl Default for WHEA_XPF_CMC_DESCRIPTOR_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_CONTEXT_INFO {
    pub RegisterContextType: u16,
    pub RegisterDataSize: u16,
    pub MSRAddress: u32,
    pub MmRegisterAddress: u64,
}
pub const WHEA_XPF_MCA_EXBANK_COUNT: u32 = 64;
pub const WHEA_XPF_MCA_EXTREG_MAX_COUNT: u32 = 24;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_MCA_SECTION {
    pub VersionNumber: u32,
    pub CpuVendor: WHEA_CPU_VENDOR,
    pub Timestamp: i64,
    pub ProcessorNumber: u32,
    pub GlobalStatus: MCG_STATUS,
    pub InstructionPointer: u64,
    pub BankNumber: u32,
    pub Status: MCI_STATUS,
    pub Address: u64,
    pub Misc: u64,
    pub ExtendedRegisterCount: u32,
    pub ApicId: u32,
    pub Anonymous: WHEA_XPF_MCA_SECTION_0,
    pub GlobalCapability: MCG_CAP,
    pub RecoveryInfo: XPF_RECOVERY_INFO,
    pub ExBankCount: u32,
    pub BankNumberEx: [u32; 64],
    pub StatusEx: [MCI_STATUS; 64],
    pub AddressEx: [u64; 64],
    pub MiscEx: [u64; 64],
}
impl Default for WHEA_XPF_MCA_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_MCA_SECTION_0 {
    pub ExtendedRegisters: [u64; 24],
    pub AMDExtendedRegisters: WHEA_AMD_EXTENDED_REGISTERS,
}
impl Default for WHEA_XPF_MCA_SECTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_XPF_MCA_SECTION_VERSION: u32 = 5;
pub const WHEA_XPF_MCA_SECTION_VERSION_2: u32 = 2;
pub const WHEA_XPF_MCA_SECTION_VERSION_3: u32 = 3;
pub const WHEA_XPF_MCA_SECTION_VERSION_4: u32 = 4;
pub const WHEA_XPF_MCA_SECTION_VERSION_5: u32 = 5;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_MCE_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: u8,
    pub NumberOfBanks: u8,
    pub Flags: XPF_MCE_FLAGS,
    pub MCG_Capability: u64,
    pub MCG_GlobalControl: u64,
    pub Banks: [WHEA_XPF_MC_BANK_DESCRIPTOR; 32],
}
impl Default for WHEA_XPF_MCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_MCE_DESCRIPTOR_V2 {
    pub Type: u16,
    pub Enabled: u8,
    pub NumberOfBanks: u8,
    pub Flags: XPF_MCE_FLAGS,
    pub MCG_Capability: u64,
    pub MCG_GlobalControl: u64,
    pub Banks: [WHEA_XPF_MC_BANK_DESCRIPTOR; 64],
}
impl Default for WHEA_XPF_MCE_DESCRIPTOR_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_MC_BANK_DESCRIPTOR {
    pub BankNumber: u8,
    pub ClearOnInitialization: bool,
    pub StatusDataFormat: u8,
    pub Flags: XPF_MC_BANK_FLAGS,
    pub ControlMsr: u32,
    pub StatusMsr: u32,
    pub AddressMsr: u32,
    pub MiscMsr: u32,
    pub ControlData: u64,
}
impl Default for WHEA_XPF_MC_BANK_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_AMD64MCA: u32 = 2;
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_IA32MCA: u32 = 0;
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_Intel64MCA: u32 = 1;
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_MS_CHECK {
    pub Anonymous: WHEA_XPF_MS_CHECK_0,
    pub XpfMsCheck: u64,
}
impl Default for WHEA_XPF_MS_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_MS_CHECK_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_NMI_DESCRIPTOR {
    pub Type: u16,
    pub Enabled: bool,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_PROCESSOR_ERROR_SECTION {
    pub ValidBits: WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS,
    pub LocalAPICId: u64,
    pub CpuId: [u8; 48],
    pub VariableInfo: [u8; 1],
}
impl Default for WHEA_XPF_PROCESSOR_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_PROCINFO {
    pub CheckInfoId: windows_sys::core::GUID,
    pub ValidBits: WHEA_XPF_PROCINFO_VALIDBITS,
    pub CheckInfo: WHEA_XPF_PROCINFO_0,
    pub TargetId: u64,
    pub RequesterId: u64,
    pub ResponderId: u64,
    pub InstructionPointer: u64,
}
impl Default for WHEA_XPF_PROCINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_PROCINFO_0 {
    pub CacheCheck: WHEA_XPF_CACHE_CHECK,
    pub TlbCheck: WHEA_XPF_TLB_CHECK,
    pub BusCheck: WHEA_XPF_BUS_CHECK,
    pub MsCheck: WHEA_XPF_MS_CHECK,
    pub AsULONGLONG: u64,
}
impl Default for WHEA_XPF_PROCINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_PROCINFO_VALIDBITS {
    pub Anonymous: WHEA_XPF_PROCINFO_VALIDBITS_0,
    pub ValidBits: u64,
}
impl Default for WHEA_XPF_PROCINFO_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_PROCINFO_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_TLB_CHECK {
    pub Anonymous: WHEA_XPF_TLB_CHECK_0,
    pub XpfTLBCheck: u64,
}
impl Default for WHEA_XPF_TLB_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_TLB_CHECK_0 {
    pub _bitfield: u64,
}
pub const WheaArmRasNodeInterfaceMmio: WHEA_ARM_RAS_NODE_INTERFACES = 1;
pub const WheaArmRasNodeInterfaceSystemRegister: WHEA_ARM_RAS_NODE_INTERFACES = 0;
pub const WheaCpuVendorAmd: WHEA_CPU_VENDOR = 2;
pub const WheaCpuVendorIntel: WHEA_CPU_VENDOR = 1;
pub const WheaCpuVendorOther: WHEA_CPU_VENDOR = 0;
pub const WheaDataFormatGeneric: WHEA_ERROR_PACKET_DATA_FORMAT = 7;
pub const WheaDataFormatIPFSalRecord: WHEA_ERROR_PACKET_DATA_FORMAT = 0;
pub const WheaDataFormatMax: WHEA_ERROR_PACKET_DATA_FORMAT = 8;
pub const WheaDataFormatMemory: WHEA_ERROR_PACKET_DATA_FORMAT = 2;
pub const WheaDataFormatNMIPort: WHEA_ERROR_PACKET_DATA_FORMAT = 4;
pub const WheaDataFormatPCIExpress: WHEA_ERROR_PACKET_DATA_FORMAT = 3;
pub const WheaDataFormatPCIXBus: WHEA_ERROR_PACKET_DATA_FORMAT = 5;
pub const WheaDataFormatPCIXDevice: WHEA_ERROR_PACKET_DATA_FORMAT = 6;
pub const WheaDataFormatXPFMCA: WHEA_ERROR_PACKET_DATA_FORMAT = 1;
pub const WheaDeviceType_Cxl: WHEA_DEVICE_TYPE = 0;
pub const WheaDeviceType_Max: WHEA_DEVICE_TYPE = 1;
pub const WheaErrSevCorrected: WHEA_ERROR_SEVERITY = 2;
pub const WheaErrSevFatal: WHEA_ERROR_SEVERITY = 1;
pub const WheaErrSevInformational: WHEA_ERROR_SEVERITY = 3;
pub const WheaErrSevRecoverable: WHEA_ERROR_SEVERITY = 0;
pub const WheaErrSrcStateRemovePending: WHEA_ERROR_SOURCE_STATE = 4;
pub const WheaErrSrcStateRemoved: WHEA_ERROR_SOURCE_STATE = 3;
pub const WheaErrSrcStateStarted: WHEA_ERROR_SOURCE_STATE = 2;
pub const WheaErrSrcStateStopped: WHEA_ERROR_SOURCE_STATE = 1;
pub const WheaErrSrcTypeBMC: WHEA_ERROR_SOURCE_TYPE = 14;
pub const WheaErrSrcTypeBOOT: WHEA_ERROR_SOURCE_TYPE = 7;
pub const WheaErrSrcTypeCMC: WHEA_ERROR_SOURCE_TYPE = 1;
pub const WheaErrSrcTypeCPE: WHEA_ERROR_SOURCE_TYPE = 2;
pub const WheaErrSrcTypeDeviceDriver: WHEA_ERROR_SOURCE_TYPE = 16;
pub const WheaErrSrcTypeGeneric: WHEA_ERROR_SOURCE_TYPE = 5;
pub const WheaErrSrcTypeGenericV2: WHEA_ERROR_SOURCE_TYPE = 12;
pub const WheaErrSrcTypeINIT: WHEA_ERROR_SOURCE_TYPE = 6;
pub const WheaErrSrcTypeIPFCMC: WHEA_ERROR_SOURCE_TYPE = 10;
pub const WheaErrSrcTypeIPFCPE: WHEA_ERROR_SOURCE_TYPE = 11;
pub const WheaErrSrcTypeIPFMCA: WHEA_ERROR_SOURCE_TYPE = 9;
pub const WheaErrSrcTypeMCE: WHEA_ERROR_SOURCE_TYPE = 0;
pub const WheaErrSrcTypeMax: WHEA_ERROR_SOURCE_TYPE = 19;
pub const WheaErrSrcTypeNMI: WHEA_ERROR_SOURCE_TYPE = 3;
pub const WheaErrSrcTypePCIe: WHEA_ERROR_SOURCE_TYPE = 4;
pub const WheaErrSrcTypePMEM: WHEA_ERROR_SOURCE_TYPE = 15;
pub const WheaErrSrcTypeSCIGeneric: WHEA_ERROR_SOURCE_TYPE = 8;
pub const WheaErrSrcTypeSCIGenericV2: WHEA_ERROR_SOURCE_TYPE = 13;
pub const WheaErrSrcTypeSea: WHEA_ERROR_SOURCE_TYPE = 17;
pub const WheaErrSrcTypeSei: WHEA_ERROR_SOURCE_TYPE = 18;
pub const WheaErrTypeGeneric: WHEA_ERROR_TYPE = 6;
pub const WheaErrTypeMemory: WHEA_ERROR_TYPE = 1;
pub const WheaErrTypeNMI: WHEA_ERROR_TYPE = 3;
pub const WheaErrTypePCIExpress: WHEA_ERROR_TYPE = 2;
pub const WheaErrTypePCIXBus: WHEA_ERROR_TYPE = 4;
pub const WheaErrTypePCIXDevice: WHEA_ERROR_TYPE = 5;
pub const WheaErrTypePmem: WHEA_ERROR_TYPE = 7;
pub const WheaErrTypeProcessor: WHEA_ERROR_TYPE = 0;
pub const WheaEventBugCheckRecoveryEntry: WHEA_BUGCHECK_RECOVERY_LOG_TYPE = 0;
pub const WheaEventBugCheckRecoveryMax: WHEA_BUGCHECK_RECOVERY_LOG_TYPE = 2;
pub const WheaEventBugCheckRecoveryReturn: WHEA_BUGCHECK_RECOVERY_LOG_TYPE = 1;
pub const WheaEventLogAzccRootBusList: WHEA_EVENT_LOG_ENTRY_ID = -2147483617;
pub const WheaEventLogAzccRootBusPoisonSet: WHEA_EVENT_LOG_ENTRY_ID = -2147483602;
pub const WheaEventLogAzccRootBusSearchErr: WHEA_EVENT_LOG_ENTRY_ID = -2147483618;
pub const WheaEventLogCmciFinalRestart: WHEA_EVENT_LOG_ENTRY_ID = -2147483620;
pub const WheaEventLogCmciRestart: WHEA_EVENT_LOG_ENTRY_ID = -2147483621;
pub const WheaEventLogEntryEarlyError: WHEA_EVENT_LOG_ENTRY_ID = -2147483594;
pub const WheaEventLogEntryEtwOverFlow: WHEA_EVENT_LOG_ENTRY_ID = -2147483619;
pub const WheaEventLogEntryIdAcpiTimeOut: WHEA_EVENT_LOG_ENTRY_ID = -2147483622;
pub const WheaEventLogEntryIdAddRemoveErrorSource: WHEA_EVENT_LOG_ENTRY_ID = -2147483636;
pub const WheaEventLogEntryIdAerNotGrantedToOs: WHEA_EVENT_LOG_ENTRY_ID = -2147483624;
pub const WheaEventLogEntryIdAttemptErrorRecovery: WHEA_EVENT_LOG_ENTRY_ID = -2147483634;
pub const WheaEventLogEntryIdBadGasFields: WHEA_EVENT_LOG_ENTRY_ID = -2147483546;
pub const WheaEventLogEntryIdBadHestNotifyData: WHEA_EVENT_LOG_ENTRY_ID = -2147483565;
pub const WheaEventLogEntryIdBadPageLimitReached: WHEA_EVENT_LOG_ENTRY_ID = -2147483596;
pub const WheaEventLogEntryIdBitOfflineEvent: WHEA_EVENT_LOG_ENTRY_ID = -2147483547;
pub const WheaEventLogEntryIdClearedPoison: WHEA_EVENT_LOG_ENTRY_ID = -2147483630;
pub const WheaEventLogEntryIdCmcPollingTimeout: WHEA_EVENT_LOG_ENTRY_ID = -2147483647;
pub const WheaEventLogEntryIdCmcSwitchToPolling: WHEA_EVENT_LOG_ENTRY_ID = -2147483645;
pub const WheaEventLogEntryIdCmciImplPresent: WHEA_EVENT_LOG_ENTRY_ID = -2147483608;
pub const WheaEventLogEntryIdCmciInitError: WHEA_EVENT_LOG_ENTRY_ID = -2147483607;
pub const WheaEventLogEntryIdCpuBusesInitFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483571;
pub const WheaEventLogEntryIdCpusFrozen: WHEA_EVENT_LOG_ENTRY_ID = -2147483552;
pub const WheaEventLogEntryIdCpusFrozenNoCrashDump: WHEA_EVENT_LOG_ENTRY_ID = -2147483551;
pub const WheaEventLogEntryIdCrashDumpCheckpoint: WHEA_EVENT_LOG_ENTRY_ID = -2147483544;
pub const WheaEventLogEntryIdCrashDumpError: WHEA_EVENT_LOG_ENTRY_ID = -2147483545;
pub const WheaEventLogEntryIdCrashDumpGuid: WHEA_EVENT_LOG_ENTRY_ID = -2147483539;
pub const WheaEventLogEntryIdCrashDumpProgressPercent: WHEA_EVENT_LOG_ENTRY_ID = -2147483543;
pub const WheaEventLogEntryIdCreateGenericRecord: WHEA_EVENT_LOG_ENTRY_ID = -2147483627;
pub const WheaEventLogEntryIdDefectListCorrupt: WHEA_EVENT_LOG_ENTRY_ID = -2147483566;
pub const WheaEventLogEntryIdDefectListFull: WHEA_EVENT_LOG_ENTRY_ID = -2147483568;
pub const WheaEventLogEntryIdDefectListUEFIVarFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483567;
pub const WheaEventLogEntryIdDeviceDriver: WHEA_EVENT_LOG_ENTRY_ID = -2147483609;
pub const WheaEventLogEntryIdDriFsStatus: WHEA_EVENT_LOG_ENTRY_ID = -2147483556;
pub const WheaEventLogEntryIdDroppedCorrectedError: WHEA_EVENT_LOG_ENTRY_ID = -2147483644;
pub const WheaEventLogEntryIdDrvErrSrcInvalid: WHEA_EVENT_LOG_ENTRY_ID = -2147483605;
pub const WheaEventLogEntryIdDrvHandleBusy: WHEA_EVENT_LOG_ENTRY_ID = -2147483604;
pub const WheaEventLogEntryIdDumpGUIDStatus: WHEA_EVENT_LOG_ENTRY_ID = -2147483538;
pub const WheaEventLogEntryIdEnableKeyNotifFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483580;
pub const WheaEventLogEntryIdErrDimmInfoMismatch: WHEA_EVENT_LOG_ENTRY_ID = -2147483600;
pub const WheaEventLogEntryIdErrSrcArrayInvalid: WHEA_EVENT_LOG_ENTRY_ID = -2147483623;
pub const WheaEventLogEntryIdErrSrcInvalid: WHEA_EVENT_LOG_ENTRY_ID = -2147483616;
pub const WheaEventLogEntryIdErrorRecord: WHEA_EVENT_LOG_ENTRY_ID = -2147483626;
pub const WheaEventLogEntryIdErrorRecordFull: WHEA_EVENT_LOG_ENTRY_ID = -2147483531;
pub const WheaEventLogEntryIdErrorRecordLimit: WHEA_EVENT_LOG_ENTRY_ID = -2147483625;
pub const WheaEventLogEntryIdFailedAddToDefectList: WHEA_EVENT_LOG_ENTRY_ID = -2147483569;
pub const WheaEventLogEntryIdGenericErrMemMap: WHEA_EVENT_LOG_ENTRY_ID = -2147483615;
pub const WheaEventLogEntryIdImpiOnline: WHEA_EVENT_LOG_ENTRY_ID = -2147483534;
pub const WheaEventLogEntryIdImpiSubscribe: WHEA_EVENT_LOG_ENTRY_ID = -2147483533;
pub const WheaEventLogEntryIdKeyNotificationFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483579;
pub const WheaEventLogEntryIdMcaErrorCleared: WHEA_EVENT_LOG_ENTRY_ID = -2147483631;
pub const WheaEventLogEntryIdMcaFoundErrorInBank: WHEA_EVENT_LOG_ENTRY_ID = -2147483633;
pub const WheaEventLogEntryIdMcaStuckErrorCheck: WHEA_EVENT_LOG_ENTRY_ID = -2147483632;
pub const WheaEventLogEntryIdMemoryAddDevice: WHEA_EVENT_LOG_ENTRY_ID = -2147483575;
pub const WheaEventLogEntryIdMemoryRemoveDevice: WHEA_EVENT_LOG_ENTRY_ID = -2147483574;
pub const WheaEventLogEntryIdMemorySummaryFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483573;
pub const WheaEventLogEntryIdOscCapabilities: WHEA_EVENT_LOG_ENTRY_ID = -2147483638;
pub const WheaEventLogEntryIdPFAMemoryOfflined: WHEA_EVENT_LOG_ENTRY_ID = -2147483642;
pub const WheaEventLogEntryIdPFAMemoryPolicy: WHEA_EVENT_LOG_ENTRY_ID = -2147483640;
pub const WheaEventLogEntryIdPFAMemoryRemoveMonitor: WHEA_EVENT_LOG_ENTRY_ID = -2147483641;
pub const WheaEventLogEntryIdPFANotifyCallbackAction: WHEA_EVENT_LOG_ENTRY_ID = -2147483560;
pub const WheaEventLogEntryIdPcieAddDevice: WHEA_EVENT_LOG_ENTRY_ID = -2147483577;
pub const WheaEventLogEntryIdPcieConfigInfo: WHEA_EVENT_LOG_ENTRY_ID = -2147483591;
pub const WheaEventLogEntryIdPcieDpcError: WHEA_EVENT_LOG_ENTRY_ID = -2147483572;
pub const WheaEventLogEntryIdPcieOverrideInfo: WHEA_EVENT_LOG_ENTRY_ID = -2147483593;
pub const WheaEventLogEntryIdPciePromotedAerErr: WHEA_EVENT_LOG_ENTRY_ID = -2147483540;
pub const WheaEventLogEntryIdPcieRemoveDevice: WHEA_EVENT_LOG_ENTRY_ID = -2147483578;
pub const WheaEventLogEntryIdPcieSpuriousErrSource: WHEA_EVENT_LOG_ENTRY_ID = -2147483576;
pub const WheaEventLogEntryIdPcieSummaryFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483584;
pub const WheaEventLogEntryIdPluginIncorrectBufferRead: WHEA_EVENT_LOG_ENTRY_ID = -2147483530;
pub const WheaEventLogEntryIdPreviousCrashBugCheckProgress: WHEA_EVENT_LOG_ENTRY_ID = -2147483542;
pub const WheaEventLogEntryIdProcessEINJ: WHEA_EVENT_LOG_ENTRY_ID = -2147483629;
pub const WheaEventLogEntryIdProcessEINJ2: WHEA_EVENT_LOG_ENTRY_ID = -2147483537;
pub const WheaEventLogEntryIdProcessHEST: WHEA_EVENT_LOG_ENTRY_ID = -2147483628;
pub const WheaEventLogEntryIdPshedCallbackCollision: WHEA_EVENT_LOG_ENTRY_ID = -2147483614;
pub const WheaEventLogEntryIdPshedInjectError: WHEA_EVENT_LOG_ENTRY_ID = -2147483639;
pub const WheaEventLogEntryIdPshedPiCpuid: WHEA_EVENT_LOG_ENTRY_ID = -2147483558;
pub const WheaEventLogEntryIdPshedPiTraceLog: WHEA_EVENT_LOG_ENTRY_ID = -2147221488;
pub const WheaEventLogEntryIdPshedPluginInitFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483570;
pub const WheaEventLogEntryIdPshedPluginLoad: WHEA_EVENT_LOG_ENTRY_ID = -2147483612;
pub const WheaEventLogEntryIdPshedPluginRegister: WHEA_EVENT_LOG_ENTRY_ID = -2147483637;
pub const WheaEventLogEntryIdPshedPluginSupported: WHEA_EVENT_LOG_ENTRY_ID = -2147483610;
pub const WheaEventLogEntryIdPshedPluginUnload: WHEA_EVENT_LOG_ENTRY_ID = -2147483611;
pub const WheaEventLogEntryIdPshedReadVendorDefined: WHEA_EVENT_LOG_ENTRY_ID = -2147483535;
pub const WheaEventLogEntryIdPshedWriteVendorDefined: WHEA_EVENT_LOG_ENTRY_ID = -2147483536;
pub const WheaEventLogEntryIdReadPcieOverridesErr: WHEA_EVENT_LOG_ENTRY_ID = -2147483592;
pub const WheaEventLogEntryIdRegError: WHEA_EVENT_LOG_ENTRY_ID = -2147483549;
pub const WheaEventLogEntryIdRegNotifyPolicyChange: WHEA_EVENT_LOG_ENTRY_ID = -2147483550;
pub const WheaEventLogEntryIdRowFailure: WHEA_EVENT_LOG_ENTRY_ID = -2147483564;
pub const WheaEventLogEntryIdRowOfflineEvent: WHEA_EVENT_LOG_ENTRY_ID = -2147483548;
pub const WheaEventLogEntryIdSELBugCheckCpusQuiesced: WHEA_EVENT_LOG_ENTRY_ID = -2147483559;
pub const WheaEventLogEntryIdSELBugCheckInfo: WHEA_EVENT_LOG_ENTRY_ID = -2147483601;
pub const WheaEventLogEntryIdSELBugCheckProgress: WHEA_EVENT_LOG_ENTRY_ID = -2147483613;
pub const WheaEventLogEntryIdSELBugCheckRecovery: WHEA_EVENT_LOG_ENTRY_ID = -2147483606;
pub const WheaEventLogEntryIdSELBugCheckStackDump: WHEA_EVENT_LOG_ENTRY_ID = -2147483541;
pub const WheaEventLogEntryIdServerType: WHEA_EVENT_LOG_ENTRY_ID = -2147483532;
pub const WheaEventLogEntryIdSrasTableBadData: WHEA_EVENT_LOG_ENTRY_ID = -2147483557;
pub const WheaEventLogEntryIdSrasTableEntries: WHEA_EVENT_LOG_ENTRY_ID = -2147483561;
pub const WheaEventLogEntryIdSrasTableError: WHEA_EVENT_LOG_ENTRY_ID = -2147483562;
pub const WheaEventLogEntryIdSrasTableNotFound: WHEA_EVENT_LOG_ENTRY_ID = -2147483563;
pub const WheaEventLogEntryIdStartedReportHwError: WHEA_EVENT_LOG_ENTRY_ID = -2147483643;
pub const WheaEventLogEntryIdThrottleAddErrSrcFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483582;
pub const WheaEventLogEntryIdThrottleRegCorrupt: WHEA_EVENT_LOG_ENTRY_ID = -2147483583;
pub const WheaEventLogEntryIdThrottleRegDataIgnored: WHEA_EVENT_LOG_ENTRY_ID = -2147483581;
pub const WheaEventLogEntryIdWheaHeartbeat: WHEA_EVENT_LOG_ENTRY_ID = -2147483603;
pub const WheaEventLogEntryIdWheaInit: WHEA_EVENT_LOG_ENTRY_ID = -2147483646;
pub const WheaEventLogEntryIdWorkQueueItem: WHEA_EVENT_LOG_ENTRY_ID = -2147483635;
pub const WheaEventLogEntryIdeDpcEnabled: WHEA_EVENT_LOG_ENTRY_ID = -2147483599;
pub const WheaEventLogEntryPageOfflineDone: WHEA_EVENT_LOG_ENTRY_ID = -2147483598;
pub const WheaEventLogEntryPageOfflinePendMax: WHEA_EVENT_LOG_ENTRY_ID = -2147483597;
pub const WheaEventLogEntrySrarDetail: WHEA_EVENT_LOG_ENTRY_ID = -2147483595;
pub const WheaEventLogEntryTypeError: WHEA_EVENT_LOG_ENTRY_TYPE = 2;
pub const WheaEventLogEntryTypeInformational: WHEA_EVENT_LOG_ENTRY_TYPE = 0;
pub const WheaEventLogEntryTypeWarning: WHEA_EVENT_LOG_ENTRY_TYPE = 1;
pub const WheaGasErrInvalidAccessSize: WHEA_GAS_ERRORS = 3;
pub const WheaGasErrInvalidStructFields: WHEA_GAS_ERRORS = 2;
pub const WheaGasErrNone: WHEA_GAS_ERRORS = 0;
pub const WheaGasErrUnexpectedAddressSpaceId: WHEA_GAS_ERRORS = 1;
pub const WheaInjectionVendorDefinedBadBuffer: WHEA_INJECTION_VENDOR_DEFINED_REASON_CODES = 5;
pub const WheaInjectionVendorDefinedNoCaps: WHEA_INJECTION_VENDOR_DEFINED_REASON_CODES = 1;
pub const WheaInjectionVendorDefinedPlatformVendorDefinedNotSet: WHEA_INJECTION_VENDOR_DEFINED_REASON_CODES = 3;
pub const WheaInjectionVendorDefinedPluginReturnedFailure: WHEA_INJECTION_VENDOR_DEFINED_REASON_CODES = 8;
pub const WheaInjectionVendorDefinedRequestOverflow: WHEA_INJECTION_VENDOR_DEFINED_REASON_CODES = 7;
pub const WheaInjectionVendorDefinedRequestTooBig: WHEA_INJECTION_VENDOR_DEFINED_REASON_CODES = 4;
pub const WheaInjectionVendorDefinedReservedBitsNonZero: WHEA_INJECTION_VENDOR_DEFINED_REASON_CODES = 2;
pub const WheaInjectionVendorDefinedVendorExtIsNull: WHEA_INJECTION_VENDOR_DEFINED_REASON_CODES = 6;
pub const WheaMemoryFm: WHEA_MEMORY_DEFINITION = 1;
pub const WheaMemoryHbm: WHEA_MEMORY_DEFINITION = 3;
pub const WheaMemoryMax: WHEA_MEMORY_DEFINITION = 4;
pub const WheaMemoryNm: WHEA_MEMORY_DEFINITION = 2;
pub const WheaMemoryThrottle: WHEA_THROTTLE_TYPE = 1;
pub const WheaMemoryUndefined: WHEA_MEMORY_DEFINITION = 0;
pub const WheaOfflineNoError: WHEA_OFFLINE_ERRS = 0;
pub const WheaPciExpressDownstreamSwitchPort: WHEA_PCIEXPRESS_DEVICE_TYPE = 6;
pub const WheaPciExpressEndpoint: WHEA_PCIEXPRESS_DEVICE_TYPE = 0;
pub const WheaPciExpressLegacyEndpoint: WHEA_PCIEXPRESS_DEVICE_TYPE = 1;
pub const WheaPciExpressRootComplexEventCollector: WHEA_PCIEXPRESS_DEVICE_TYPE = 10;
pub const WheaPciExpressRootComplexIntegratedEndpoint: WHEA_PCIEXPRESS_DEVICE_TYPE = 9;
pub const WheaPciExpressRootPort: WHEA_PCIEXPRESS_DEVICE_TYPE = 4;
pub const WheaPciExpressToPciXBridge: WHEA_PCIEXPRESS_DEVICE_TYPE = 7;
pub const WheaPciExpressUpstreamSwitchPort: WHEA_PCIEXPRESS_DEVICE_TYPE = 5;
pub const WheaPciREcoveryStatusUnknown: WHEA_PCI_RECOVERY_STATUS = 0;
pub const WheaPciRecoverySignalAer: WHEA_PCI_RECOVERY_SIGNAL = 1;
pub const WheaPciRecoverySignalDpc: WHEA_PCI_RECOVERY_SIGNAL = 2;
pub const WheaPciRecoverySignalUnknown: WHEA_PCI_RECOVERY_SIGNAL = 0;
pub const WheaPciRecoveryStatusBusNotFound: WHEA_PCI_RECOVERY_STATUS = 6;
pub const WheaPciRecoveryStatusComplexTree: WHEA_PCI_RECOVERY_STATUS = 5;
pub const WheaPciRecoveryStatusCxlAerNotRecoverable: WHEA_PCI_RECOVERY_STATUS = 11;
pub const WheaPciRecoveryStatusDdaAerNotRecoverable: WHEA_PCI_RECOVERY_STATUS = 8;
pub const WheaPciRecoveryStatusDeviceNotFound: WHEA_PCI_RECOVERY_STATUS = 7;
pub const WheaPciRecoveryStatusFailedRecovery: WHEA_PCI_RECOVERY_STATUS = 9;
pub const WheaPciRecoveryStatusLinkDisableTimeout: WHEA_PCI_RECOVERY_STATUS = 2;
pub const WheaPciRecoveryStatusLinkEnableTimeout: WHEA_PCI_RECOVERY_STATUS = 3;
pub const WheaPciRecoveryStatusNoError: WHEA_PCI_RECOVERY_STATUS = 1;
pub const WheaPciRecoveryStatusRecoveredNoDevices: WHEA_PCI_RECOVERY_STATUS = 10;
pub const WheaPciRecoveryStatusRpBusyTimeout: WHEA_PCI_RECOVERY_STATUS = 4;
pub const WheaPciXToExpressBridge: WHEA_PCIEXPRESS_DEVICE_TYPE = 8;
pub const WheaPcieThrottle: WHEA_THROTTLE_TYPE = 0;
pub const WheaPfaRemoveCapacity: WHEA_PFA_REMOVE_TRIGGER = 3;
pub const WheaPfaRemoveErrorThreshold: WHEA_PFA_REMOVE_TRIGGER = 1;
pub const WheaPfaRemoveTimeout: WHEA_PFA_REMOVE_TRIGGER = 2;
pub const WheaRawDataFormatAMD64MCA: WHEA_RAW_DATA_FORMAT = 3;
pub const WheaRawDataFormatGeneric: WHEA_RAW_DATA_FORMAT = 9;
pub const WheaRawDataFormatIA32MCA: WHEA_RAW_DATA_FORMAT = 1;
pub const WheaRawDataFormatIPFSalRecord: WHEA_RAW_DATA_FORMAT = 0;
pub const WheaRawDataFormatIntel64MCA: WHEA_RAW_DATA_FORMAT = 2;
pub const WheaRawDataFormatMax: WHEA_RAW_DATA_FORMAT = 10;
pub const WheaRawDataFormatMemory: WHEA_RAW_DATA_FORMAT = 4;
pub const WheaRawDataFormatNMIPort: WHEA_RAW_DATA_FORMAT = 6;
pub const WheaRawDataFormatPCIExpress: WHEA_RAW_DATA_FORMAT = 5;
pub const WheaRawDataFormatPCIXBus: WHEA_RAW_DATA_FORMAT = 7;
pub const WheaRawDataFormatPCIXDevice: WHEA_RAW_DATA_FORMAT = 8;
pub const WheaRecoveryContextActionTakenMax: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN = 4;
pub const WheaRecoveryContextActionTakenNone: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN = 0;
pub const WheaRecoveryContextActionTakenOfflineDemotion: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN = 1;
pub const WheaRecoveryContextActionTakenPageNotReplaced: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN = 2;
pub const WheaRecoveryContextActionTakenPageReplaced: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN = 3;
pub const WheaRecoveryContextErrorTypeMax: WHEA_RECOVERY_CONTEXT_ERROR_TYPE = 3;
pub const WheaRecoveryContextErrorTypeMemory: WHEA_RECOVERY_CONTEXT_ERROR_TYPE = 1;
pub const WheaRecoveryContextErrorTypePmem: WHEA_RECOVERY_CONTEXT_ERROR_TYPE = 2;
pub const WheaRecoveryFailureReasonFarNotValid: WHEA_RECOVERY_FAILURE_REASON = 17;
pub const WheaRecoveryFailureReasonHighIrql: WHEA_RECOVERY_FAILURE_REASON = 10;
pub const WheaRecoveryFailureReasonInsufficientAltContextWrappers: WHEA_RECOVERY_FAILURE_REASON = 11;
pub const WheaRecoveryFailureReasonInterruptsDisabled: WHEA_RECOVERY_FAILURE_REASON = 12;
pub const WheaRecoveryFailureReasonInvalidAddressMode: WHEA_RECOVERY_FAILURE_REASON = 9;
pub const WheaRecoveryFailureReasonKernelCouldNotMarkMemoryBad: WHEA_RECOVERY_FAILURE_REASON = 1;
pub const WheaRecoveryFailureReasonKernelMarkMemoryBadTimedOut: WHEA_RECOVERY_FAILURE_REASON = 2;
pub const WheaRecoveryFailureReasonKernelWillPageFaultBCAtCurrentIrql: WHEA_RECOVERY_FAILURE_REASON = 16;
pub const WheaRecoveryFailureReasonMax: WHEA_RECOVERY_FAILURE_REASON = 18;
pub const WheaRecoveryFailureReasonMiscOrAddrNotValid: WHEA_RECOVERY_FAILURE_REASON = 8;
pub const WheaRecoveryFailureReasonNoRecoveryContext: WHEA_RECOVERY_FAILURE_REASON = 3;
pub const WheaRecoveryFailureReasonNotContinuable: WHEA_RECOVERY_FAILURE_REASON = 4;
pub const WheaRecoveryFailureReasonNotSupported: WHEA_RECOVERY_FAILURE_REASON = 7;
pub const WheaRecoveryFailureReasonOverflow: WHEA_RECOVERY_FAILURE_REASON = 6;
pub const WheaRecoveryFailureReasonPcc: WHEA_RECOVERY_FAILURE_REASON = 5;
pub const WheaRecoveryFailureReasonStackOverflow: WHEA_RECOVERY_FAILURE_REASON = 14;
pub const WheaRecoveryFailureReasonSwapBusy: WHEA_RECOVERY_FAILURE_REASON = 13;
pub const WheaRecoveryFailureReasonUnexpectedFailure: WHEA_RECOVERY_FAILURE_REASON = 15;
pub const WheaRecoveryTypeActionOptional: WHEA_RECOVERY_TYPE = 2;
pub const WheaRecoveryTypeActionRequired: WHEA_RECOVERY_TYPE = 1;
pub const WheaRecoveryTypeMax: WHEA_RECOVERY_TYPE = 3;
pub const WheaRegErrFailedToCreatePolicyKey: WHEA_REGISTRY_ERRORS = 2;
pub const WheaRegErrFailedToCreateWheaKey: WHEA_REGISTRY_ERRORS = 1;
pub const WheaRegErrFailedToOpenHandle: WHEA_REGISTRY_ERRORS = 3;
pub const WheaRegErrNone: WHEA_REGISTRY_ERRORS = 0;
pub const WheapDpcErrBusNotFound: WHEAP_DPC_ERROR_EVENT_TYPE = 1;
pub const WheapDpcErrDeviceIdBad: WHEAP_DPC_ERROR_EVENT_TYPE = 3;
pub const WheapDpcErrDpcedSubtree: WHEAP_DPC_ERROR_EVENT_TYPE = 2;
pub const WheapDpcErrNoChildren: WHEAP_DPC_ERROR_EVENT_TYPE = 5;
pub const WheapDpcErrNoErr: WHEAP_DPC_ERROR_EVENT_TYPE = 0;
pub const WheapDpcErrResetFailed: WHEAP_DPC_ERROR_EVENT_TYPE = 4;
pub const WheapPfaOfflinePredictiveFailure: WHEAP_PFA_OFFLINE_DECISION_TYPE = 1;
pub const WheapPfaOfflineUncorrectedError: WHEAP_PFA_OFFLINE_DECISION_TYPE = 2;
pub const WormController: CONFIGURATION_TYPE = 16;
pub const XPF_BUS_CHECK_ADDRESS_IO: u32 = 2;
pub const XPF_BUS_CHECK_ADDRESS_MEMORY: u32 = 0;
pub const XPF_BUS_CHECK_ADDRESS_OTHER: u32 = 3;
pub const XPF_BUS_CHECK_ADDRESS_RESERVED: u32 = 1;
pub const XPF_BUS_CHECK_OPERATION_DATAREAD: u32 = 3;
pub const XPF_BUS_CHECK_OPERATION_DATAWRITE: u32 = 4;
pub const XPF_BUS_CHECK_OPERATION_GENERIC: u32 = 0;
pub const XPF_BUS_CHECK_OPERATION_GENREAD: u32 = 1;
pub const XPF_BUS_CHECK_OPERATION_GENWRITE: u32 = 2;
pub const XPF_BUS_CHECK_OPERATION_INSTRUCTIONFETCH: u32 = 5;
pub const XPF_BUS_CHECK_OPERATION_PREFETCH: u32 = 6;
pub const XPF_BUS_CHECK_PARTICIPATION_GENERIC: u32 = 3;
pub const XPF_BUS_CHECK_PARTICIPATION_PROCOBSERVED: u32 = 2;
pub const XPF_BUS_CHECK_PARTICIPATION_PROCORIGINATED: u32 = 0;
pub const XPF_BUS_CHECK_PARTICIPATION_PROCRESPONDED: u32 = 1;
pub const XPF_BUS_CHECK_TRANSACTIONTYPE_DATAACCESS: u32 = 1;
pub const XPF_BUS_CHECK_TRANSACTIONTYPE_GENERIC: u32 = 2;
pub const XPF_BUS_CHECK_TRANSACTIONTYPE_INSTRUCTION: u32 = 0;
pub const XPF_CACHE_CHECK_OPERATION_DATAREAD: u32 = 3;
pub const XPF_CACHE_CHECK_OPERATION_DATAWRITE: u32 = 4;
pub const XPF_CACHE_CHECK_OPERATION_EVICTION: u32 = 7;
pub const XPF_CACHE_CHECK_OPERATION_GENERIC: u32 = 0;
pub const XPF_CACHE_CHECK_OPERATION_GENREAD: u32 = 1;
pub const XPF_CACHE_CHECK_OPERATION_GENWRITE: u32 = 2;
pub const XPF_CACHE_CHECK_OPERATION_INSTRUCTIONFETCH: u32 = 5;
pub const XPF_CACHE_CHECK_OPERATION_PREFETCH: u32 = 6;
pub const XPF_CACHE_CHECK_OPERATION_SNOOP: u32 = 8;
pub const XPF_CACHE_CHECK_TRANSACTIONTYPE_DATAACCESS: u32 = 1;
pub const XPF_CACHE_CHECK_TRANSACTIONTYPE_GENERIC: u32 = 2;
pub const XPF_CACHE_CHECK_TRANSACTIONTYPE_INSTRUCTION: u32 = 0;
pub const XPF_CONTEXT_INFO_32BITCONTEXT: u32 = 2;
pub const XPF_CONTEXT_INFO_32BITDEBUGREGS: u32 = 5;
pub const XPF_CONTEXT_INFO_64BITCONTEXT: u32 = 3;
pub const XPF_CONTEXT_INFO_64BITDEBUGREGS: u32 = 6;
pub const XPF_CONTEXT_INFO_FXSAVE: u32 = 4;
pub const XPF_CONTEXT_INFO_MMREGISTERS: u32 = 7;
pub const XPF_CONTEXT_INFO_MSRREGISTERS: u32 = 1;
pub const XPF_CONTEXT_INFO_UNCLASSIFIEDDATA: u32 = 0;
pub const XPF_MCA_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8a1e1d01_42f9_4557_9c33_565e5cc3f7e8);
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union XPF_MCE_FLAGS {
    pub Anonymous: XPF_MCE_FLAGS_0,
    pub AsULONG: u32,
}
impl Default for XPF_MCE_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XPF_MCE_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union XPF_MC_BANK_FLAGS {
    pub Anonymous: XPF_MC_BANK_FLAGS_0,
    pub AsUCHAR: u8,
}
impl Default for XPF_MC_BANK_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct XPF_MC_BANK_FLAGS_0 {
    pub _bitfield: u8,
}
pub const XPF_MS_CHECK_ERRORTYPE_EXTERNAL: u32 = 3;
pub const XPF_MS_CHECK_ERRORTYPE_FRC: u32 = 4;
pub const XPF_MS_CHECK_ERRORTYPE_INTERNALUNCLASSIFIED: u32 = 5;
pub const XPF_MS_CHECK_ERRORTYPE_MCROMPARITY: u32 = 2;
pub const XPF_MS_CHECK_ERRORTYPE_NOERROR: u32 = 0;
pub const XPF_MS_CHECK_ERRORTYPE_UNCLASSIFIED: u32 = 1;
pub const XPF_PROCESSOR_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdc3ea0b0_a144_4797_b95b_53fa242b6e1d);
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XPF_RECOVERY_INFO {
    pub FailureReason: XPF_RECOVERY_INFO_0,
    pub Action: XPF_RECOVERY_INFO_1,
    pub ActionRequired: bool,
    pub RecoverySucceeded: bool,
    pub RecoveryKernel: bool,
    pub Reserved: u8,
    pub Reserved2: u16,
    pub Reserved3: u16,
    pub Reserved4: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XPF_RECOVERY_INFO_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy, Default)]
pub struct XPF_RECOVERY_INFO_1 {
    pub _bitfield: u32,
}
pub const XPF_TLB_CHECK_OPERATION_DATAREAD: u32 = 3;
pub const XPF_TLB_CHECK_OPERATION_DATAWRITE: u32 = 4;
pub const XPF_TLB_CHECK_OPERATION_GENERIC: u32 = 0;
pub const XPF_TLB_CHECK_OPERATION_GENREAD: u32 = 1;
pub const XPF_TLB_CHECK_OPERATION_GENWRITE: u32 = 2;
pub const XPF_TLB_CHECK_OPERATION_INSTRUCTIONFETCH: u32 = 5;
pub const XPF_TLB_CHECK_OPERATION_PREFETCH: u32 = 6;
pub const XPF_TLB_CHECK_TRANSACTIONTYPE_DATAACCESS: u32 = 1;
pub const XPF_TLB_CHECK_TRANSACTIONTYPE_GENERIC: u32 = 2;
pub const XPF_TLB_CHECK_TRANSACTIONTYPE_INSTRUCTION: u32 = 0;
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Default)]
pub struct ZONE_HEADER {
    pub FreeList: super::super::Win32::winnt::SINGLE_LIST_ENTRY,
    pub SegmentList: super::super::Win32::winnt::SINGLE_LIST_ENTRY,
    pub BlockSize: u32,
    pub TotalSegmentSize: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct ZONE_SEGMENT_HEADER {
    pub SegmentList: super::super::Win32::winnt::SINGLE_LIST_ENTRY,
    pub Reserved: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_winnt")]
impl Default for ZONE_SEGMENT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _BUS_HANDLER(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _CREATE_DISK(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _DEVICE_HANDLER_OBJECT(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _EJOB(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _FFA_MSG_SEND_DIRECT_REQ2_PARAMETERS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _FFA_NOTIFICATION_REGISTRATION_PARAMETERS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _FFA_RUN_TARGET_INPUT_PARAMETERS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _FFA_RUN_TARGET_OUTPUT_PARAMETERS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _IMAGE_NT_HEADERS(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _IMAGE_NT_HEADERS64(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _KGDTENTRY64(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _KIDTENTRY64(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _KPRCB(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _KTSS64(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _LOADER_PARAMETER_BLOCK(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _PEB(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _SILO_MONITOR(pub u8);
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type _WHEA_ERROR_SOURCE_CORRECT = Option<unsafe extern "system" fn(errorsource: *mut WHEA_ERROR_SOURCE_DESCRIPTOR, maximumsectionlength: *mut u32) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type _WHEA_ERROR_SOURCE_CORRECT_DEVICE_DRIVER = Option<unsafe extern "system" fn(errorsourcedesc: *mut core::ffi::c_void, maximumsectionlength: *mut u32) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_winnt"))]
pub type _WHEA_ERROR_SOURCE_CREATE_RECORD = Option<unsafe extern "system" fn(errorsource: *mut WHEA_ERROR_SOURCE_DESCRIPTOR, errorpacket: *mut WHEA_ERROR_PACKET_V2, errorrecord: *mut WHEA_ERROR_RECORD, buffersize: u32, context: *mut core::ffi::c_void) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef"))]
pub type _WHEA_ERROR_SOURCE_INITIALIZE = Option<unsafe extern "system" fn(phase: u32, errorsource: *mut WHEA_ERROR_SOURCE_DESCRIPTOR, context: *mut core::ffi::c_void) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type _WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, errorsourceid: u32) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type _WHEA_ERROR_SOURCE_RECOVER = Option<unsafe extern "system" fn(recoverycontext: *mut core::ffi::c_void, severity: *mut WHEA_ERROR_SEVERITY) -> super::super::Win32::bcrypt::NTSTATUS>;
pub type _WHEA_ERROR_SOURCE_UNINITIALIZE = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
pub type _WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
pub type _WHEA_SIGNAL_HANDLER_OVERRIDE_CALLBACK = Option<unsafe extern "system" fn(context: usize) -> bool>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type pHalAssignSlotResources = Option<unsafe extern "system" fn(registrypath: *const super::super::Win32::ntsecapi::UNICODE_STRING, driverclassname: *const super::super::Win32::ntsecapi::UNICODE_STRING, driverobject: *const super::wdm::DRIVER_OBJECT, deviceobject: *const super::wdm::DEVICE_OBJECT, bustype: super::wdm::INTERFACE_TYPE, busnumber: u32, slotnumber: u32, allocatedresources: *mut super::wdm::PCM_RESOURCE_LIST) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type pHalEndMirroring = Option<unsafe extern "system" fn(passnumber: u32) -> super::super::Win32::bcrypt::NTSTATUS>;
pub type pHalEndOfBoot = Option<unsafe extern "system" fn()>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type pHalExamineMBR = Option<unsafe extern "system" fn(deviceobject: *const super::wdm::DEVICE_OBJECT, sectorsize: u32, mbrtypeidentifier: u32, buffer: *mut *mut core::ffi::c_void)>;
#[cfg(feature = "Win32_bcrypt")]
pub type pHalFfaMsgSendDirectReq2 = Option<unsafe extern "system" fn(parameters: *mut _FFA_MSG_SEND_DIRECT_REQ2_PARAMETERS) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type pHalFfaRegisterNotification = Option<unsafe extern "system" fn(registrationparameters: *const _FFA_NOTIFICATION_REGISTRATION_PARAMETERS, token: *mut *mut core::ffi::c_void) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type pHalFfaRunTarget = Option<unsafe extern "system" fn(inputparameters: *const _FFA_RUN_TARGET_INPUT_PARAMETERS, outputparameters: *mut _FFA_RUN_TARGET_OUTPUT_PARAMETERS) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type pHalFfaUnregisterNotification = Option<unsafe extern "system" fn(token: *const core::ffi::c_void) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_usb")]
pub type pHalFindBusAddressTranslation = Option<unsafe extern "system" fn(busaddress: super::super::Win32::usb::PHYSICAL_ADDRESS, addressspace: *mut u32, translatedaddress: *mut i64, context: *mut u64, nextbus: bool) -> bool>;
pub type pHalGetAcpiTable = Option<unsafe extern "system" fn(signature: u32, oemid: windows_sys::core::PCSTR, oemtableid: windows_sys::core::PCSTR) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type pHalGetDmaAdapter = Option<unsafe extern "system" fn(physicaldeviceobject: *const core::ffi::c_void, devicedescriptor: *const super::wdm::DEVICE_DESCRIPTION, numberofmapregisters: *mut u32) -> *mut super::wdm::DMA_ADAPTER>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type pHalGetInterruptTranslator = Option<unsafe extern "system" fn(parentinterfacetype: super::wdm::INTERFACE_TYPE, parentbusnumber: u32, bridgeinterfacetype: super::wdm::INTERFACE_TYPE, size: u16, version: u16, translator: *mut TRANSLATOR_INTERFACE, bridgebusnumber: *mut u32) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_winnt")]
pub type pHalGetPrmCache = Option<unsafe extern "system" fn(firmwarelist: *mut super::super::Win32::winnt::PLIST_ENTRY, updatelist: *mut super::super::Win32::winnt::PLIST_ENTRY)>;
pub type pHalHaltSystem = Option<unsafe extern "system" fn()>;
#[cfg(feature = "Wdk_wdm")]
pub type pHalHandlerForBus = Option<unsafe extern "system" fn(interfacetype: super::wdm::INTERFACE_TYPE, busnumber: u32) -> PBUS_HANDLER>;
#[cfg(feature = "Win32_bcrypt")]
pub type pHalInitPnpDriver = Option<unsafe extern "system" fn() -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type pHalInitPowerManagement = Option<unsafe extern "system" fn(pmdriverdispatchtable: *const PM_DISPATCH_TABLE, pmhaldispatchtable: *mut PPM_DISPATCH_TABLE) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type pHalInvokePrmFwHandler = Option<unsafe extern "system" fn(handlerguid: *const windows_sys::core::GUID, parameterbuffer: *const core::ffi::c_void, contextbuffer: *const core::ffi::c_void) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type pHalIoReadPartitionTable = Option<unsafe extern "system" fn(deviceobject: *const super::wdm::DEVICE_OBJECT, sectorsize: u32, returnrecognizedpartitions: bool, partitionbuffer: *mut *mut super::wdm::_DRIVE_LAYOUT_INFORMATION) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type pHalIoSetPartitionInformation = Option<unsafe extern "system" fn(deviceobject: *const super::wdm::DEVICE_OBJECT, sectorsize: u32, partitionnumber: u32, partitiontype: u32) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Wdk_ntdef", feature = "Wdk_ntifs", feature = "Wdk_wdm", feature = "Win32_basetsd", feature = "Win32_bcrypt", feature = "Win32_lsalookup", feature = "Win32_ntsecapi", feature = "Win32_usb", feature = "Win32_winnt", feature = "Win32_winternl"))]
pub type pHalIoWritePartitionTable = Option<unsafe extern "system" fn(deviceobject: *const super::wdm::DEVICE_OBJECT, sectorsize: u32, sectorspertrack: u32, numberofheads: u32, partitionbuffer: *const super::wdm::_DRIVE_LAYOUT_INFORMATION) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_usb"))]
pub type pHalMirrorPhysicalMemory = Option<unsafe extern "system" fn(physicaladdress: super::super::Win32::usb::PHYSICAL_ADDRESS, numberofbytes: i64) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_usb"))]
pub type pHalMirrorVerify = Option<unsafe extern "system" fn(physicaladdress: super::super::Win32::usb::PHYSICAL_ADDRESS, numberofbytes: i64) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type pHalQueryBusSlots = Option<unsafe extern "system" fn(bushandler: *const _BUS_HANDLER, buffersize: u32, slotnumbers: *mut u32, returnedlength: *mut u32) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type pHalQuerySystemInformation = Option<unsafe extern "system" fn(informationclass: HAL_QUERY_INFORMATION_CLASS, buffersize: u32, buffer: *mut core::ffi::c_void, returnedlength: *mut u32) -> super::super::Win32::bcrypt::NTSTATUS>;
pub type pHalReferenceBusHandler = Option<unsafe extern "system" fn(bushandler: *const _BUS_HANDLER)>;
pub type pHalResetDisplay = Option<unsafe extern "system" fn() -> bool>;
pub type pHalSetPciErrorHandlerCallback = Option<unsafe extern "system" fn(callback: PCI_ERROR_HANDLER_CALLBACK)>;
#[cfg(feature = "Win32_bcrypt")]
pub type pHalSetSystemInformation = Option<unsafe extern "system" fn(informationclass: HAL_SET_INFORMATION_CLASS, buffersize: u32, buffer: *const core::ffi::c_void) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(feature = "Win32_bcrypt")]
pub type pHalStartMirroring = Option<unsafe extern "system" fn() -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Wdk_wdm", feature = "Win32_usb"))]
pub type pHalTranslateBusAddress = Option<unsafe extern "system" fn(interfacetype: super::wdm::INTERFACE_TYPE, busnumber: u32, busaddress: super::super::Win32::usb::PHYSICAL_ADDRESS, addressspace: *mut u32, translatedaddress: *mut i64) -> bool>;
pub type pHalVectorToIDTEntry = Option<unsafe extern "system" fn(vector: u32) -> u8>;
pub type pKdCheckPowerButton = Option<unsafe extern "system" fn()>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_usb", feature = "Win32_winnt"))]
pub type pKdEnumerateDebuggingDevices = Option<unsafe extern "system" fn(loaderblock: *const core::ffi::c_void, device: *mut DEBUG_DEVICE_DESCRIPTOR, callback: PDEBUG_DEVICE_FOUND_FUNCTION) -> super::super::Win32::bcrypt::NTSTATUS>;
pub type pKdGetAcpiTablePhase0 = Option<unsafe extern "system" fn(loaderblock: *const _LOADER_PARAMETER_BLOCK, signature: u32) -> *mut core::ffi::c_void>;
pub type pKdGetPciDataByOffset = Option<unsafe extern "system" fn(busnumber: u32, slotnumber: u32, buffer: *mut core::ffi::c_void, offset: u32, length: u32) -> u32>;
#[cfg(feature = "Win32_usb")]
pub type pKdMapPhysicalMemory64 = Option<unsafe extern "system" fn(physicaladdress: super::super::Win32::usb::PHYSICAL_ADDRESS, numberpages: u32, flushcurrenttlb: bool) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_usb", feature = "Win32_winnt"))]
pub type pKdReleaseIntegratedDeviceForDebugging = Option<unsafe extern "system" fn(integrateddevice: *mut DEBUG_DEVICE_DESCRIPTOR) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_usb", feature = "Win32_winnt"))]
pub type pKdReleasePciDeviceForDebugging = Option<unsafe extern "system" fn(pcidevice: *mut DEBUG_DEVICE_DESCRIPTOR) -> super::super::Win32::bcrypt::NTSTATUS>;
pub type pKdSetPciDataByOffset = Option<unsafe extern "system" fn(busnumber: u32, slotnumber: u32, buffer: *const core::ffi::c_void, offset: u32, length: u32) -> u32>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_usb", feature = "Win32_winnt"))]
pub type pKdSetupIntegratedDeviceForDebugging = Option<unsafe extern "system" fn(loaderblock: *const core::ffi::c_void, integrateddevice: *mut DEBUG_DEVICE_DESCRIPTOR) -> super::super::Win32::bcrypt::NTSTATUS>;
#[cfg(all(feature = "Win32_bcrypt", feature = "Win32_minwindef", feature = "Win32_usb", feature = "Win32_winnt"))]
pub type pKdSetupPciDeviceForDebugging = Option<unsafe extern "system" fn(loaderblock: *const core::ffi::c_void, pcidevice: *mut DEBUG_DEVICE_DESCRIPTOR) -> super::super::Win32::bcrypt::NTSTATUS>;
pub type pKdUnmapVirtualAddress = Option<unsafe extern "system" fn(virtualaddress: *const core::ffi::c_void, numberpages: u32, flushcurrenttlb: bool)>;
