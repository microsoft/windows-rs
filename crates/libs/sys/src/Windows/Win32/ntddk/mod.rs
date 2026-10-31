#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn DbgPrompt(prompt : super::PCCH, response : super::PCH, length : super::ULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt", feature = "winternl"))]
windows_link::link!("ntdll.dll" "system" fn NtOpenProcess(processhandle : super::PHANDLE, desiredaccess : super::ACCESS_MASK, objectattributes : super::POBJECT_ATTRIBUTES, clientid : super::PCLIENT_ID) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
windows_link::link!("ntdll.dll" "system" fn RtlCompareString(string1 : *const super::STRING, string2 : *const super::STRING, caseinsensitive : super::BOOLEAN) -> super::LONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlContractHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
windows_link::link!("ntdll.dll" "system" fn RtlCopyString(destinationstring : super::PSTRING, sourcestring : *const super::STRING));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlCreateHashTable(hashtable : *mut PRTL_DYNAMIC_HASH_TABLE, shift : super::ULONG, flags : super::ULONG) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlCreateHashTableEx(hashtable : *mut PRTL_DYNAMIC_HASH_TABLE, initialsize : super::ULONG, shift : super::ULONG, flags : super::ULONG) -> super::BOOLEAN);
windows_link::link!("ntdll.dll" "system" fn RtlDelete(links : PRTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlDeleteElementGenericTable(table : PRTL_GENERIC_TABLE, buffer : super::PVOID) -> super::BOOLEAN);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlDeleteElementGenericTableAvl(table : PRTL_AVL_TABLE, buffer : super::PVOID) -> super::BOOLEAN);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlDeleteElementGenericTableAvlEx(table : PRTL_AVL_TABLE, nodeorparent : super::PVOID));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlDeleteHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE));
windows_link::link!("ntdll.dll" "system" fn RtlDeleteNoSplay(links : PRTL_SPLAY_LINKS, root : *mut PRTL_SPLAY_LINKS));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlEndEnumerationHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE, enumerator : PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlEndStrongEnumerationHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE, enumerator : PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlEndWeakEnumerationHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE, enumerator : PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlEnumerateEntryHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE, enumerator : PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlEnumerateGenericTable(table : PRTL_GENERIC_TABLE, restart : super::BOOLEAN) -> super::PVOID);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlEnumerateGenericTableAvl(table : PRTL_AVL_TABLE, restart : super::BOOLEAN) -> super::PVOID);
#[cfg(all(feature = "minwindef", feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlEnumerateGenericTableLikeADirectory(table : PRTL_AVL_TABLE, matchfunction : PRTL_AVL_MATCH_FUNCTION, matchdata : super::PVOID, nextflag : super::ULONG, restartkey : *mut super::PVOID, deletecount : super::PULONG, buffer : super::PVOID) -> super::PVOID);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlEnumerateGenericTableWithoutSplaying(table : PRTL_GENERIC_TABLE, restartkey : *mut super::PVOID) -> super::PVOID);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlEnumerateGenericTableWithoutSplayingAvl(table : PRTL_AVL_TABLE, restartkey : *mut super::PVOID) -> super::PVOID);
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
windows_link::link!("ntdll.dll" "system" fn RtlEqualString(string1 : *const super::STRING, string2 : *const super::STRING, caseinsensitive : super::BOOLEAN) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlExpandHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("ntdll.dll" "system" fn RtlGetActiveConsoleId() -> super::ULONG);
#[cfg(feature = "winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlGetCallersAddress(callersaddress : *mut super::PVOID, callerscaller : *mut super::PVOID));
#[cfg(feature = "shared")]
windows_link::link!("ntdll.dll" "system" fn RtlGetConsoleSessionForegroundProcessId() -> super::ULONGLONG);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlGetElementGenericTable(table : PRTL_GENERIC_TABLE, i : super::ULONG) -> super::PVOID);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlGetElementGenericTableAvl(table : PRTL_AVL_TABLE, i : super::ULONG) -> super::PVOID);
#[cfg(feature = "shared")]
windows_link::link!("ntdll.dll" "system" fn RtlGetEnabledExtendedFeatures(featuremask : super::ULONG64) -> super::ULONG64);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlGetNextEntryHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE, context : PRTL_DYNAMIC_HASH_TABLE_CONTEXT) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY);
#[cfg(all(feature = "ntdef", feature = "shared"))]
windows_link::link!("ntdll.dll" "system" fn RtlGetNtProductType(ntproducttype : super::PNT_PRODUCT_TYPE) -> super::BOOLEAN);
windows_link::link!("ntdll.dll" "system" fn RtlGetNtSystemRoot() -> windows_sys::core::PCWSTR);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlGetPersistedStateLocation(sourceid : windows_sys::core::PCWSTR, customvalue : windows_sys::core::PCWSTR, defaultpath : windows_sys::core::PCWSTR, statelocationtype : STATE_LOCATION_TYPE, targetpath : super::PWCHAR, bufferlengthin : super::ULONG, bufferlengthout : super::PULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("ntdll.dll" "system" fn RtlGetSuiteMask() -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInitEnumerationHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE, enumerator : PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInitStrongEnumerationHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE, enumerator : PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInitWeakEnumerationHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE, enumerator : PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> super::BOOLEAN);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInitializeGenericTable(table : PRTL_GENERIC_TABLE, compareroutine : PRTL_GENERIC_COMPARE_ROUTINE, allocateroutine : PRTL_GENERIC_ALLOCATE_ROUTINE, freeroutine : PRTL_GENERIC_FREE_ROUTINE, tablecontext : super::PVOID));
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInitializeGenericTableAvl(table : PRTL_AVL_TABLE, compareroutine : PRTL_AVL_COMPARE_ROUTINE, allocateroutine : PRTL_AVL_ALLOCATE_ROUTINE, freeroutine : PRTL_AVL_FREE_ROUTINE, tablecontext : super::PVOID));
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInsertElementGenericTable(table : PRTL_GENERIC_TABLE, buffer : super::PVOID, buffersize : super::CLONG, newelement : super::PBOOLEAN) -> super::PVOID);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInsertElementGenericTableAvl(table : PRTL_AVL_TABLE, buffer : super::PVOID, buffersize : super::CLONG, newelement : super::PBOOLEAN) -> super::PVOID);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInsertElementGenericTableFull(table : PRTL_GENERIC_TABLE, buffer : super::PVOID, buffersize : super::CLONG, newelement : super::PBOOLEAN, nodeorparent : super::PVOID, searchresult : TABLE_SEARCH_RESULT) -> super::PVOID);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInsertElementGenericTableFullAvl(table : PRTL_AVL_TABLE, buffer : super::PVOID, buffersize : super::CLONG, newelement : super::PBOOLEAN, nodeorparent : super::PVOID, searchresult : TABLE_SEARCH_RESULT) -> super::PVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInsertEntryHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE, entry : PRTL_DYNAMIC_HASH_TABLE_ENTRY, signature : super::ULONG_PTR, context : PRTL_DYNAMIC_HASH_TABLE_CONTEXT) -> super::BOOLEAN);
windows_link::link!("ntdll.dll" "system" fn RtlIsApiSetImplemented(apisetname : windows_sys::core::PCSTR) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlIsGenericTableEmpty(table : PRTL_GENERIC_TABLE) -> super::BOOLEAN);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlIsGenericTableEmptyAvl(table : PRTL_AVL_TABLE) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("ntdll.dll" "system" fn RtlIsMultiSessionSku() -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("ntdll.dll" "system" fn RtlIsMultiUsersInSessionSku() -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("ntdll.dll" "system" fn RtlIsStateSeparationEnabled() -> super::BOOLEAN);
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlLargeIntegerDivide(dividend : super::LARGE_INTEGER, divisor : super::LARGE_INTEGER, remainder : super::PLARGE_INTEGER) -> super::LARGE_INTEGER);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlLookupElementGenericTable(table : PRTL_GENERIC_TABLE, buffer : super::PVOID) -> super::PVOID);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlLookupElementGenericTableAvl(table : PRTL_AVL_TABLE, buffer : super::PVOID) -> super::PVOID);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlLookupElementGenericTableFull(table : PRTL_GENERIC_TABLE, buffer : super::PVOID, nodeorparent : *mut super::PVOID, searchresult : *mut TABLE_SEARCH_RESULT) -> super::PVOID);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlLookupElementGenericTableFullAvl(table : PRTL_AVL_TABLE, buffer : super::PVOID, nodeorparent : *mut super::PVOID, searchresult : *mut TABLE_SEARCH_RESULT) -> super::PVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlLookupEntryHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE, signature : super::ULONG_PTR, context : PRTL_DYNAMIC_HASH_TABLE_CONTEXT) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlLookupFirstMatchingElementGenericTableAvl(table : PRTL_AVL_TABLE, buffer : super::PVOID, restartkey : *mut super::PVOID) -> super::PVOID);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlMapGenericMask(accessmask : super::PACCESS_MASK, genericmapping : *const super::GENERIC_MAPPING));
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlNumberGenericTableElements(table : PRTL_GENERIC_TABLE) -> super::ULONG);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlNumberGenericTableElementsAvl(table : PRTL_AVL_TABLE) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winternl"))]
windows_link::link!("ntdll.dll" "system" fn RtlPrefixUnicodeString(string1 : super::PCUNICODE_STRING, string2 : super::PCUNICODE_STRING, caseinsensitive : super::BOOLEAN) -> super::BOOLEAN);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt", feature = "winternl"))]
windows_link::link!("ntdll.dll" "system" fn RtlQueryRegistryValueWithFallback(primaryhandle : super::HANDLE, fallbackhandle : super::HANDLE, valuename : super::PUNICODE_STRING, valuelength : super::ULONG, valuetype : super::PULONG, valuedata : super::PVOID, resultlength : super::PULONG) -> windows_sys::core::NTSTATUS);
windows_link::link!("ntdll.dll" "system" fn RtlRealPredecessor(links : PRTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS);
windows_link::link!("ntdll.dll" "system" fn RtlRealSuccessor(links : PRTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlRemoveEntryHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE, entry : PRTL_DYNAMIC_HASH_TABLE_ENTRY, context : PRTL_DYNAMIC_HASH_TABLE_CONTEXT) -> super::BOOLEAN);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlRunOnceBeginInitialize(runonce : super::PRTL_RUN_ONCE, flags : super::ULONG, context : *mut super::PVOID) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlRunOnceComplete(runonce : super::PRTL_RUN_ONCE, flags : super::ULONG, context : super::PVOID) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlRunOnceExecuteOnce(runonce : super::PRTL_RUN_ONCE, initfn : PRTL_RUN_ONCE_INIT_FN, parameter : super::PVOID, context : *mut super::PVOID) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "winnt")]
windows_link::link!("ntdll.dll" "system" fn RtlRunOnceInitialize(runonce : super::PRTL_RUN_ONCE));
windows_link::link!("ntdll.dll" "system" fn RtlSplay(links : PRTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlStronglyEnumerateEntryHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE, enumerator : PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY);
windows_link::link!("ntdll.dll" "system" fn RtlSubtreePredecessor(links : PRTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS);
windows_link::link!("ntdll.dll" "system" fn RtlSubtreeSuccessor(links : PRTL_SPLAY_LINKS) -> PRTL_SPLAY_LINKS);
#[cfg(all(feature = "shared", feature = "winternl"))]
windows_link::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeString(destinationstring : super::PUNICODE_STRING, sourcestring : super::PCUNICODE_STRING, allocatedestinationstring : super::BOOLEAN) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "shared")]
windows_link::link!("ntdll.dll" "system" fn RtlUpperChar(character : super::CHAR) -> super::CHAR);
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
windows_link::link!("ntdll.dll" "system" fn RtlUpperString(destinationstring : super::PSTRING, sourcestring : *const super::STRING));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlWalkFrameChain(callers : *mut super::PVOID, count : super::ULONG, flags : super::ULONG) -> super::ULONG);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlWeaklyEnumerateEntryHashTable(hashtable : PRTL_DYNAMIC_HASH_TABLE, enumerator : PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> PRTL_DYNAMIC_HASH_TABLE_ENTRY);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn ZwAllocateLocallyUniqueId(luid : super::PLUID) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "winnt")]
windows_link::link!("ntdll.dll" "system" fn ZwCancelTimer(timerhandle : super::HANDLE, currentstate : super::PBOOLEAN) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt", feature = "winternl"))]
windows_link::link!("ntdll.dll" "system" fn ZwCreateTimer(timerhandle : super::PHANDLE, desiredaccess : super::ACCESS_MASK, objectattributes : super::POBJECT_ATTRIBUTES, timertype : super::TIMER_TYPE) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "shared", feature = "winternl"))]
windows_link::link!("ntdll.dll" "system" fn ZwDisplayString(string : super::PUNICODE_STRING) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt", feature = "winternl"))]
windows_link::link!("ntdll.dll" "system" fn ZwOpenProcess(processhandle : super::PHANDLE, desiredaccess : super::ACCESS_MASK, objectattributes : super::POBJECT_ATTRIBUTES, clientid : super::PCLIENT_ID) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
windows_link::link!("ntdll.dll" "system" fn ZwOpenTimer(timerhandle : super::PHANDLE, desiredaccess : super::ACCESS_MASK, objectattributes : super::POBJECT_ATTRIBUTES) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn ZwPowerInformation(informationlevel : super::POWER_INFORMATION_LEVEL, inputbuffer : super::PVOID, inputbufferlength : super::ULONG, outputbuffer : super::PVOID, outputbufferlength : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn ZwSetInformationThread(threadhandle : super::HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : super::PVOID, threadinformationlength : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn ZwSetTimer(timerhandle : super::HANDLE, duetime : super::PLARGE_INTEGER, timerapcroutine : PTIMER_APC_ROUTINE, timercontext : super::PVOID, resumetimer : super::BOOLEAN, period : super::LONG, previousstate : super::PBOOLEAN) -> windows_sys::core::NTSTATUS);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn ZwSetTimerEx(timerhandle : super::HANDLE, timersetinformationclass : TIMER_SET_INFORMATION_CLASS, timersetinformation : super::PVOID, timersetinformationlength : super::ULONG) -> windows_sys::core::NTSTATUS);
#[cfg(feature = "winnt")]
windows_link::link!("ntdll.dll" "system" fn ZwTerminateProcess(processhandle : super::HANDLE, exitstatus : windows_sys::core::NTSTATUS) -> windows_sys::core::NTSTATUS);
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ACPI_DEBUGGING_DEVICE_IN_USE {
    pub NameSpacePathLength: super::ULONG,
    pub NameSpacePath: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for ACPI_DEBUGGING_DEVICE_IN_USE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union AER_BRIDGE_DESCRIPTOR_FLAGS {
    pub Anonymous: AER_BRIDGE_DESCRIPTOR_FLAGS_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for AER_BRIDGE_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct AER_BRIDGE_DESCRIPTOR_FLAGS_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union AER_ENDPOINT_DESCRIPTOR_FLAGS {
    pub Anonymous: AER_ENDPOINT_DESCRIPTOR_FLAGS_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for AER_ENDPOINT_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct AER_ENDPOINT_DESCRIPTOR_FLAGS_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union AER_ROOTPORT_DESCRIPTOR_FLAGS {
    pub Anonymous: AER_ROOTPORT_DESCRIPTOR_FLAGS_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for AER_ROOTPORT_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct AER_ROOTPORT_DESCRIPTOR_FLAGS_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct AGP_TARGET_BUS_INTERFACE_STANDARD {
    pub Size: super::USHORT,
    pub Version: super::USHORT,
    pub Context: super::PVOID,
    pub InterfaceReference: super::PINTERFACE_REFERENCE,
    pub InterfaceDereference: super::PINTERFACE_DEREFERENCE,
    pub SetBusData: super::PGET_SET_DEVICE_DATA,
    pub GetBusData: super::PGET_SET_DEVICE_DATA,
    pub CapabilityID: super::UCHAR,
}
pub const AMD_DRAM_TO_NORMALIZED_PRM_HANDLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0639bd1c_3e33_4055_bae7_36cceba8376e);
pub const AMD_DRAM_TO_SPA_PRM_HANDLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x69aa0a9c_e3fc_4b0d_929e_aa1bde5d9a9b);
pub const AMD_NORMALIZED_TO_DRAM_PRM_HANDLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x7626c6ae_f973_429c_a91c_107d7be298b0);
pub const AMD_NORMALIZED_TO_SPA_PRM_HANDLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe7180659_a65d_451d_92cd_2b56f12beba6);
pub const AMD_SPA_TO_DRAM_PRM_HANDLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd1c6b8f2_f9ac_4bf0_855e_dbd582ce4b20);
pub const AMD_SPA_TO_NORMALIZED_PRM_HANDLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00c77891_7fcb_4d01_94e1_72f8e4ee1af7);
pub type ARBITER_ACTION = i32;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct ARBITER_ADD_RESERVED_PARAMETERS {
    pub ReserveDevice: super::PDEVICE_OBJECT,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct ARBITER_BOOT_ALLOCATION_PARAMETERS {
    pub ArbitrationList: super::PLIST_ENTRY,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct ARBITER_CONFLICT_INFO {
    pub OwningObject: super::PDEVICE_OBJECT,
    pub Start: super::ULONGLONG,
    pub End: super::ULONGLONG,
}
pub const ARBITER_FLAG_BOOT_CONFIG: i32 = 1;
pub const ARBITER_FLAG_OTHER_ENUM: i32 = 4;
pub const ARBITER_FLAG_ROOT_ENUM: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct ARBITER_INTERFACE {
    pub Size: super::USHORT,
    pub Version: super::USHORT,
    pub Context: super::PVOID,
    pub InterfaceReference: super::PINTERFACE_REFERENCE,
    pub InterfaceDereference: super::PINTERFACE_DEREFERENCE,
    pub ArbiterHandler: PARBITER_HANDLER,
    pub Flags: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct ARBITER_LIST_ENTRY {
    pub ListEntry: super::LIST_ENTRY,
    pub AlternativeCount: super::ULONG,
    pub Alternatives: super::PIO_RESOURCE_DESCRIPTOR,
    pub PhysicalDeviceObject: super::PDEVICE_OBJECT,
    pub RequestSource: ARBITER_REQUEST_SOURCE,
    pub Flags: super::ULONG,
    pub WorkSpace: super::LONG_PTR,
    pub InterfaceType: super::INTERFACE_TYPE,
    pub SlotNumber: super::ULONG,
    pub BusNumber: super::ULONG,
    pub Assignment: super::PCM_PARTIAL_RESOURCE_DESCRIPTOR,
    pub SelectedAlternative: super::PIO_RESOURCE_DESCRIPTOR,
    pub Result: ARBITER_RESULT,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct ARBITER_PARAMETERS {
    pub Parameters: ARBITER_PARAMETERS_0,
}
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for ARBITER_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
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
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for ARBITER_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ARBITER_PARTIAL: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ARBITER_QUERY_ALLOCATED_RESOURCES_PARAMETERS {
    pub AllocatedResources: *mut super::PCM_PARTIAL_RESOURCE_LIST,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct ARBITER_QUERY_ARBITRATE_PARAMETERS {
    pub ArbitrationList: super::PLIST_ENTRY,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct ARBITER_QUERY_CONFLICT_PARAMETERS {
    pub PhysicalDeviceObject: super::PDEVICE_OBJECT,
    pub ConflictingResource: super::PIO_RESOURCE_DESCRIPTOR,
    pub ConflictCount: super::PULONG,
    pub Conflicts: *mut PARBITER_CONFLICT_INFO,
}
pub type ARBITER_REQUEST_SOURCE = i32;
pub type ARBITER_RESULT = i32;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ARBITER_RETEST_ALLOCATION_PARAMETERS {
    pub ArbitrationList: super::PLIST_ENTRY,
    pub AllocateFromCount: super::ULONG,
    pub AllocateFrom: super::PCM_PARTIAL_RESOURCE_DESCRIPTOR,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ARBITER_TEST_ALLOCATION_PARAMETERS {
    pub ArbitrationList: super::PLIST_ENTRY,
    pub AllocateFromCount: super::ULONG,
    pub AllocateFrom: super::PCM_PARTIAL_RESOURCE_DESCRIPTOR,
}
#[cfg(target_arch = "aarch64")]
pub const ARM64_PCR_RESERVED_MASK: i32 = 4095;
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
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct BDCB_IMAGE_INFORMATION {
    pub Classification: BDCB_CLASSIFICATION,
    pub ImageFlags: super::ULONG,
    pub ImageName: super::UNICODE_STRING,
    pub RegistryPath: super::UNICODE_STRING,
    pub CertificatePublisher: super::UNICODE_STRING,
    pub CertificateIssuer: super::UNICODE_STRING,
    pub ImageHash: super::PVOID,
    pub CertificateThumbprint: super::PVOID,
    pub ImageHashAlgorithm: super::ULONG,
    pub ThumbprintHashAlgorithm: super::ULONG,
    pub ImageHashLength: super::ULONG,
    pub CertificateThumbprintLength: super::ULONG,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct BDCB_STATUS_UPDATE_CONTEXT {
    pub StatusType: BDCB_STATUS_UPDATE_TYPE,
}
pub type BDCB_STATUS_UPDATE_TYPE = i32;
pub const BMC_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x487565ba_6494_4367_95ca_4eff893522f6);
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
pub type BOOT_DRIVER_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(callbackcontext: super::PVOID, classification: BDCB_CALLBACK_TYPE, imageinformation: PBDCB_IMAGE_INFORMATION)>;
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
#[cfg(target_arch = "aarch64")]
pub const BtiVbarBhbClr: PCR_BTI_VBAR_INDEX = 5;
#[cfg(target_arch = "aarch64")]
pub const BtiVbarBhbDsbIsb: PCR_BTI_VBAR_INDEX = 3;
#[cfg(target_arch = "aarch64")]
pub const BtiVbarBhbSb: PCR_BTI_VBAR_INDEX = 4;
#[cfg(target_arch = "aarch64")]
pub const BtiVbarNone: PCR_BTI_VBAR_INDEX = 0;
#[cfg(target_arch = "aarch64")]
pub const BtiVbarTrapHvc: PCR_BTI_VBAR_INDEX = 1;
#[cfg(target_arch = "aarch64")]
pub const BtiVbarTrapSmc: PCR_BTI_VBAR_INDEX = 2;
pub const BusWidth32Bits: PCI_BUS_WIDTH = 0;
pub const BusWidth64Bits: PCI_BUS_WIDTH = 1;
pub const CMCI_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x919448b2_3739_4b7f_a8f1_e0062805c2a3);
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "mce", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CMC_DRIVER_INFO {
    pub ExceptionCallback: PDRIVER_CMC_EXCEPTION_CALLBACK,
    pub DpcCallback: super::PKDEFERRED_ROUTINE,
    pub DeviceContext: super::PVOID,
}
pub const CMC_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2dce8bb1_bdd7_450e_b9ad_9cf4ebd4f890);
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct CM_PCCARD_DEVICE_DATA {
    pub Flags: super::UCHAR,
    pub ErrorCode: super::UCHAR,
    pub Reserved: super::USHORT,
    pub BusData: super::ULONG,
    pub DeviceId: super::ULONG,
    pub LegacyBaseAddress: super::ULONG,
    pub IRQMap: [super::UCHAR; 16],
}
#[cfg(feature = "shared")]
impl Default for CM_PCCARD_DEVICE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CONFIGURATION_INFORMATION {
    pub DiskCount: super::ULONG,
    pub FloppyCount: super::ULONG,
    pub CdRomCount: super::ULONG,
    pub TapeCount: super::ULONG,
    pub ScsiPortCount: super::ULONG,
    pub SerialCount: super::ULONG,
    pub ParallelCount: super::ULONG,
    pub AtDiskPrimaryAddressClaimed: super::BOOLEAN,
    pub AtDiskSecondaryAddressClaimed: super::BOOLEAN,
    pub Version: super::ULONG,
    pub MediumChangerCount: super::ULONG,
}
pub type CONFIGURATION_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CONTROLLER_OBJECT {
    pub Type: super::CSHORT,
    pub Size: super::CSHORT,
    pub ControllerExtension: super::PVOID,
    pub DeviceWaitQueue: super::KDEVICE_QUEUE,
    pub Spare1: super::ULONG,
    pub Spare2: super::LARGE_INTEGER,
}
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for CONTROLLER_OBJECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CPER_EMPTY_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "mce", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CPE_DRIVER_INFO {
    pub ExceptionCallback: PDRIVER_CPE_EXCEPTION_CALLBACK,
    pub DpcCallback: super::PKDEFERRED_ROUTINE,
    pub DeviceContext: super::PVOID,
}
pub const CPE_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4e292f96_d843_4a55_a8c2_d481f27ebeee);
pub const CP_GET_ERROR: i32 = 2;
pub const CP_GET_NODATA: i32 = 1;
pub const CP_GET_SUCCESS: i32 = 0;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CREATE_USER_PROCESS_ECP_CONTEXT {
    pub Size: super::USHORT,
    pub Reserved: super::USHORT,
    pub AccessToken: super::PACCESS_TOKEN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct CXL_BUS_OSC_CONTROL_FIELD {
    pub u: CXL_BUS_OSC_CONTROL_FIELD_0,
}
#[cfg(feature = "shared")]
impl Default for CXL_BUS_OSC_CONTROL_FIELD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union CXL_BUS_OSC_CONTROL_FIELD_0 {
    pub Anonymous: CXL_BUS_OSC_CONTROL_FIELD_0_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for CXL_BUS_OSC_CONTROL_FIELD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CXL_BUS_OSC_CONTROL_FIELD_0_0 {
    pub _bitfield: u32,
}
pub const CXL_BUS_OSC_METHOD_CAPABILITY_REVISION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct CXL_BUS_OSC_SUPPORT_FIELD {
    pub u: CXL_BUS_OSC_SUPPORT_FIELD_0,
}
#[cfg(feature = "shared")]
impl Default for CXL_BUS_OSC_SUPPORT_FIELD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union CXL_BUS_OSC_SUPPORT_FIELD_0 {
    pub Anonymous: CXL_BUS_OSC_SUPPORT_FIELD_0_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for CXL_BUS_OSC_SUPPORT_FIELD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
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
pub const CmResourceTypeMaximum: i32 = 8;
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
pub const DBG_DEVICE_FLAG_BARS_MAPPED: i32 = 2;
pub const DBG_DEVICE_FLAG_HAL_SCRATCH_ALLOCATED: i32 = 1;
pub const DBG_DEVICE_FLAG_HOST_VISIBLE_ALLOCATED: i32 = 32;
pub const DBG_DEVICE_FLAG_SCRATCH_ALLOCATED: i32 = 4;
pub const DBG_DEVICE_FLAG_SYNTHETIC: i32 = 16;
pub const DBG_DEVICE_FLAG_UNCACHED_MEMORY: i32 = 8;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DEBUGGING_DEVICE_IN_USE {
    pub NameSpace: KD_NAMESPACE_ENUM,
    pub StructureLength: super::ULONG,
    pub Anonymous: DEBUGGING_DEVICE_IN_USE_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DEBUGGING_DEVICE_IN_USE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DEBUGGING_DEVICE_IN_USE_0 {
    pub AcpiDevice: ACPI_DEBUGGING_DEVICE_IN_USE,
    pub PciDevice: PCI_DEBUGGING_DEVICE_IN_USE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DEBUGGING_DEVICE_IN_USE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DEBUGGING_DEVICE_IN_USE_INFORMATION {
    pub DeviceCount: super::ULONG,
    pub Device: [DEBUGGING_DEVICE_IN_USE; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DEBUGGING_DEVICE_IN_USE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct DEBUG_DEVICE_ADDRESS {
    pub Type: super::UCHAR,
    pub Valid: super::BOOLEAN,
    pub Anonymous: DEBUG_DEVICE_ADDRESS_0,
    pub TranslatedAddress: super::PUCHAR,
    pub Length: super::ULONG,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for DEBUG_DEVICE_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy)]
pub union DEBUG_DEVICE_ADDRESS_0 {
    pub Reserved: [super::UCHAR; 2],
    pub Anonymous: DEBUG_DEVICE_ADDRESS_0_0,
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
impl Default for DEBUG_DEVICE_ADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_DEVICE_ADDRESS_0_0 {
    pub BitWidth: super::UCHAR,
    pub AccessSize: super::UCHAR,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DEBUG_DEVICE_DESCRIPTOR {
    pub Bus: super::ULONG,
    pub Slot: super::ULONG,
    pub Segment: super::USHORT,
    pub VendorID: super::USHORT,
    pub DeviceID: super::USHORT,
    pub BaseClass: super::UCHAR,
    pub SubClass: super::UCHAR,
    pub ProgIf: super::UCHAR,
    pub Anonymous: DEBUG_DEVICE_DESCRIPTOR_0,
    pub Initialized: super::BOOLEAN,
    pub Configured: super::BOOLEAN,
    pub BaseAddress: [DEBUG_DEVICE_ADDRESS; 6],
    pub Memory: DEBUG_MEMORY_REQUIREMENTS,
    pub Dbg2TableIndex: super::ULONG,
    pub PortType: super::USHORT,
    pub PortSubtype: super::USHORT,
    pub OemData: super::PVOID,
    pub OemDataLength: super::ULONG,
    pub NameSpace: KD_NAMESPACE_ENUM,
    pub NameSpacePath: super::PWCHAR,
    pub NameSpacePathLength: super::ULONG,
    pub TransportType: super::ULONG,
    pub TransportData: DEBUG_TRANSPORT_DATA,
    pub EfiIoMmuData: DEBUG_EFI_IOMMU_DATA,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for DEBUG_DEVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union DEBUG_DEVICE_DESCRIPTOR_0 {
    pub Flags: super::UCHAR,
    pub Anonymous: DEBUG_DEVICE_DESCRIPTOR_0_0,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for DEBUG_DEVICE_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_DEVICE_DESCRIPTOR_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_EFI_IOMMU_DATA {
    pub PciIoProtocolHandle: super::PVOID,
    pub Mapping: super::PVOID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct DEBUG_MEMORY_REQUIREMENTS {
    pub Start: super::PHYSICAL_ADDRESS,
    pub MaxEnd: super::PHYSICAL_ADDRESS,
    pub VirtualAddress: super::PVOID,
    pub Length: super::ULONG,
    pub Cached: super::BOOLEAN,
    pub Aligned: super::BOOLEAN,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for DEBUG_MEMORY_REQUIREMENTS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DEBUG_TRANSPORT_DATA {
    pub HwContextSize: super::ULONG,
    pub SharedVisibleDataSize: super::ULONG,
    pub UseSerialFraming: super::BOOLEAN,
    pub ValidUSBCoreId: super::BOOLEAN,
    pub USBCoreId: super::UCHAR,
    pub DevControlInitialized: super::BOOLEAN,
    pub DevControlPciLocation: DEBUG_TRANSPORT_DATA_0,
}
#[cfg(feature = "shared")]
impl Default for DEBUG_TRANSPORT_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DEBUG_TRANSPORT_DATA_0 {
    pub RawPciLocation: super::UINT32,
    pub Fields: DEBUG_TRANSPORT_DATA_0_0,
}
#[cfg(feature = "shared")]
impl Default for DEBUG_TRANSPORT_DATA_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DEBUG_TRANSPORT_DATA_0_0 {
    pub _bitfield: u32,
}
pub const DEFAULT_DEVICE_DRIVER_CREATOR_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57217c8d_5e66_44fb_8033_9b74cacedf5b);
pub const DEVICE_DRIVER_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0033f803_2e70_4e88_992c_6f26daf3db7a);
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DIMM_ADDRESS {
    pub Ddr4: DIMM_ADDRESS_0,
    pub Ddr5: DIMM_ADDRESS_1,
}
#[cfg(feature = "shared")]
impl Default for DIMM_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DIMM_ADDRESS_0 {
    pub _bitfield: u64,
    pub Row: super::UINT32,
    pub Column: super::UINT32,
    pub Info: super::UINT64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DIMM_ADDRESS_1 {
    pub _bitfield: u64,
    pub Row: super::UINT32,
    pub Column: super::UINT32,
    pub Info: super::UINT64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DIMM_ADDR_VALID_BITS {
    pub VB_DDR4: DIMM_ADDR_VALID_BITS_DDR4,
    pub VB_DDR5: DIMM_ADDR_VALID_BITS_DDR5,
    pub AsUINT32: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for DIMM_ADDR_VALID_BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DIMM_ADDR_VALID_BITS_DDR4 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DIMM_ADDR_VALID_BITS_DDR5 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DIMM_INFO {
    pub DimmAddress: DIMM_ADDRESS,
    pub ValidBits: DIMM_ADDR_VALID_BITS,
}
#[cfg(feature = "shared")]
impl Default for DIMM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DISK_SIGNATURE {
    pub PartitionStyle: super::ULONG,
    pub Anonymous: DISK_SIGNATURE_0,
}
#[cfg(feature = "shared")]
impl Default for DISK_SIGNATURE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DISK_SIGNATURE_0 {
    pub Mbr: DISK_SIGNATURE_0_0,
    pub Gpt: DISK_SIGNATURE_0_1,
}
#[cfg(feature = "shared")]
impl Default for DISK_SIGNATURE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DISK_SIGNATURE_0_0 {
    pub Signature: super::ULONG,
    pub CheckSum: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DISK_SIGNATURE_0_1 {
    pub DiskId: windows_sys::core::GUID,
}
pub const DOE_DISCOVERY_LENGTH: i32 = 3;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DOE_DISCOVERY_REQUEST {
    pub Anonymous: DOE_DISCOVERY_REQUEST_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for DOE_DISCOVERY_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DOE_DISCOVERY_REQUEST_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DOE_DISCOVERY_RESPONSE {
    pub Anonymous: DOE_DISCOVERY_RESPONSE_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for DOE_DISCOVERY_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DOE_DISCOVERY_RESPONSE_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DOE_HEADER_1 {
    pub Anonymous: DOE_HEADER_1_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for DOE_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DOE_HEADER_1_0 {
    pub _bitfield1: u16,
    pub _bitfield2: u8,
    pub _bitfield3: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DOE_HEADER_2 {
    pub Anonymous: DOE_HEADER_2_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for DOE_HEADER_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct DOE_HEADER_2_0 {
    pub _bitfield: u32,
}
pub const DOE_HEADER_COUNT: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union DOE_OBJECT_HEADER {
    pub Anonymous: DOE_OBJECT_HEADER_0,
    pub AsULONGLONG: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for DOE_OBJECT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct DOE_OBJECT_HEADER_0 {
    pub header1: DOE_HEADER_1,
    pub header2: DOE_HEADER_2,
}
#[cfg(feature = "shared")]
impl Default for DOE_OBJECT_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOE_READ_DELAY: i32 = 10000;
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type DRIVER_REINITIALIZE = Option<unsafe extern "system" fn(driverobject: *const super::DRIVER_OBJECT, context: super::PVOID, count: super::ULONG)>;
pub const DRIVER_VERIFIER_FORCE_IRQL_CHECKING: i32 = 2;
pub const DRIVER_VERIFIER_INJECT_ALLOCATION_FAILURES: i32 = 4;
pub const DRIVER_VERIFIER_IO_CHECKING: i32 = 16;
pub const DRIVER_VERIFIER_SPECIAL_POOLING: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct DRIVER_VERIFIER_THUNK_PAIRS {
    pub PristineRoutine: PDRIVER_VERIFIER_THUNK_ROUTINE,
    pub NewRoutine: PDRIVER_VERIFIER_THUNK_ROUTINE,
}
pub const DRIVER_VERIFIER_TRACK_POOL_ALLOCATIONS: i32 = 8;
pub const DRVO_BOOTREINIT_REGISTERED: i32 = 32;
pub const DRVO_INITIALIZED: i32 = 16;
pub const DRVO_LEGACY_RESOURCES: i32 = 64;
pub const DRVO_REINIT_REGISTERED: i32 = 8;
pub const DiskController: CONFIGURATION_TYPE = 13;
pub const DiskPeripheral: CONFIGURATION_TYPE = 25;
pub const DisplayController: CONFIGURATION_TYPE = 19;
pub const DockingInformation: CONFIGURATION_TYPE = 38;
pub const DtiAdapter: CONFIGURATION_TYPE = 11;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct EFI_ACPI_RAS_SIGNAL_TABLE {
    pub Header: WHEA_ACPI_HEADER,
    pub NumberRecord: super::UINT32,
    pub Entries: [SIGNAL_REG_VALUE; 1],
}
#[cfg(feature = "shared")]
impl Default for EFI_ACPI_RAS_SIGNAL_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct ERROR_SOURCE_INFO {
    pub ErrorCount: super::ULONG,
    pub ErrorSourceId: super::ULONG,
}
pub const ERRTYP_BUS: i32 = 16;
pub const ERRTYP_CACHE: i32 = 6;
pub const ERRTYP_FLOW: i32 = 9;
pub const ERRTYP_FUNCTION: i32 = 7;
pub const ERRTYP_IMPROPER: i32 = 18;
pub const ERRTYP_INTERNAL: i32 = 1;
pub const ERRTYP_LOSSOFLOCKSTEP: i32 = 20;
pub const ERRTYP_MAP: i32 = 17;
pub const ERRTYP_MEM: i32 = 4;
pub const ERRTYP_PARITY: i32 = 22;
pub const ERRTYP_PATHERROR: i32 = 24;
pub const ERRTYP_POISONED: i32 = 26;
pub const ERRTYP_PROTOCOL: i32 = 23;
pub const ERRTYP_RESPONSE: i32 = 21;
pub const ERRTYP_SELFTEST: i32 = 8;
pub const ERRTYP_TIMEOUT: i32 = 25;
pub const ERRTYP_TLB: i32 = 5;
pub const ERRTYP_UNIMPL: i32 = 19;
#[cfg(feature = "winnt")]
pub type EXPAND_STACK_CALLOUT = Option<unsafe extern "system" fn(parameter: super::PVOID)>;
pub type EXTENDED_AGP_REGISTER = i32;
pub const EXTINT_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfe84086e_b557_43cf_ac1b_17982e078470);
pub const EisaAdapter: CONFIGURATION_TYPE = 8;
pub const EisaConfiguration: BUS_DATA_TYPE = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FILE_ALIGNMENT_INFORMATION {
    pub AlignmentRequirement: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FILE_ATTRIBUTE_TAG_INFORMATION {
    pub FileAttributes: super::ULONG,
    pub ReparseTag: super::ULONG,
}
pub const FILE_CHARACTERISTICS_EXPECT_ORDERLY_REMOVAL_DEPRECATED: i32 = 512;
pub const FILE_CHARACTERISTICS_EXPECT_ORDERLY_REMOVAL_EX: i32 = 16384;
pub const FILE_CHARACTERISTICS_EXPECT_SURPRISE_REMOVAL_DEPRECATED: i32 = 768;
pub const FILE_CHARACTERISTICS_EXPECT_SURPRISE_REMOVAL_EX: i32 = 32768;
pub const FILE_CHARACTERISTICS_PROPAGATED: i32 = 327951;
pub const FILE_CHARACTERISTICS_REMOVAL_POLICY_MASK_DEPRECATED: i32 = 768;
pub const FILE_CHARACTERISTICS_REMOVAL_POLICY_MASK_EX: i32 = 49152;
pub const FILE_DISPOSITION_DELETE: i32 = 1;
pub const FILE_DISPOSITION_DO_NOT_DELETE: i32 = 0;
pub const FILE_DISPOSITION_FORCE_IMAGE_SECTION_CHECK: i32 = 4;
pub const FILE_DISPOSITION_IGNORE_READONLY_ATTRIBUTE: i32 = 16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FILE_DISPOSITION_INFORMATION {
    pub DeleteFile: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FILE_DISPOSITION_INFORMATION_EX {
    pub Flags: super::ULONG,
}
pub const FILE_DISPOSITION_ON_CLOSE: i32 = 8;
pub const FILE_DISPOSITION_POSIX_SEMANTICS: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_END_OF_FILE_INFORMATION {
    pub EndOfFile: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_END_OF_FILE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_FS_FULL_SIZE_INFORMATION {
    pub TotalAllocationUnits: super::LARGE_INTEGER,
    pub CallerAvailableAllocationUnits: super::LARGE_INTEGER,
    pub ActualAvailableAllocationUnits: super::LARGE_INTEGER,
    pub SectorsPerAllocationUnit: super::ULONG,
    pub BytesPerSector: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_FS_FULL_SIZE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FILE_FS_FULL_SIZE_INFORMATION_EX {
    pub ActualTotalAllocationUnits: super::ULONGLONG,
    pub ActualAvailableAllocationUnits: super::ULONGLONG,
    pub ActualPoolUnavailableAllocationUnits: super::ULONGLONG,
    pub CallerTotalAllocationUnits: super::ULONGLONG,
    pub CallerAvailableAllocationUnits: super::ULONGLONG,
    pub CallerPoolUnavailableAllocationUnits: super::ULONGLONG,
    pub UsedAllocationUnits: super::ULONGLONG,
    pub TotalReservedAllocationUnits: super::ULONGLONG,
    pub VolumeStorageReserveAllocationUnits: super::ULONGLONG,
    pub AvailableCommittedAllocationUnits: super::ULONGLONG,
    pub PoolAvailableAllocationUnits: super::ULONGLONG,
    pub SectorsPerAllocationUnit: super::ULONG,
    pub BytesPerSector: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_FS_LABEL_INFORMATION {
    pub VolumeLabelLength: super::ULONG,
    pub VolumeLabel: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_FS_LABEL_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_FS_METADATA_SIZE_INFORMATION {
    pub TotalMetadataAllocationUnits: super::LARGE_INTEGER,
    pub SectorsPerAllocationUnit: super::ULONG,
    pub BytesPerSector: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_FS_METADATA_SIZE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct FILE_FS_OBJECTID_INFORMATION {
    pub ObjectId: [super::UCHAR; 16],
    pub ExtendedInfo: [super::UCHAR; 48],
}
#[cfg(feature = "shared")]
impl Default for FILE_FS_OBJECTID_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct FILE_FS_SECTOR_SIZE_INFORMATION {
    pub LogicalBytesPerSector: super::ULONG,
    pub PhysicalBytesPerSectorForAtomicity: super::ULONG,
    pub PhysicalBytesPerSectorForPerformance: super::ULONG,
    pub FileSystemEffectivePhysicalBytesPerSectorForAtomicity: super::ULONG,
    pub Flags: super::ULONG,
    pub ByteOffsetForSectorAlignment: super::ULONG,
    pub ByteOffsetForPartitionAlignment: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_FS_SIZE_INFORMATION {
    pub TotalAllocationUnits: super::LARGE_INTEGER,
    pub AvailableAllocationUnits: super::LARGE_INTEGER,
    pub SectorsPerAllocationUnit: super::ULONG,
    pub BytesPerSector: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_FS_SIZE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_FS_VOLUME_INFORMATION {
    pub VolumeCreationTime: super::LARGE_INTEGER,
    pub VolumeSerialNumber: super::ULONG,
    pub VolumeLabelLength: super::ULONG,
    pub SupportsObjects: super::BOOLEAN,
    pub VolumeLabel: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_FS_VOLUME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_NAME_INFORMATION {
    pub FileNameLength: super::ULONG,
    pub FileName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_VALID_DATA_LENGTH_INFORMATION {
    pub ValidDataLength: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_VALID_DATA_LENGTH_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FIRMWARE_ERROR_RECORD_REFERENCE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x81212a96_09ed_4996_9471_8d729c8e69ed);
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_16GB: i32 = 6;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_1GB: i32 = 2;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_1MB: i32 = 9;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_256MB: i32 = 0;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_2GB: i32 = 3;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_32GB: i32 = 7;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_4GB: i32 = 4;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_512MB: i32 = 1;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_8GB: i32 = 5;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_16MB: i32 = 4;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_1MB: i32 = 0;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_2MB: i32 = 1;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_4MB: i32 = 2;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_8MB: i32 = 3;
pub const FPB_MEM_VECTOR_GRANULARITY_1B: i32 = 8;
pub const FPB_RID_VECTOR_GRANULARITY_256RIDS: i32 = 5;
pub const FPB_RID_VECTOR_GRANULARITY_64RIDS: i32 = 3;
pub const FPB_RID_VECTOR_GRANULARITY_8RIDS: i32 = 0;
pub const FPB_VECTOR_SELECT_MEM_HIGH: i32 = 2;
pub const FPB_VECTOR_SELECT_MEM_LOW: i32 = 1;
pub const FPB_VECTOR_SELECT_RID: i32 = 0;
pub const FPB_VECTOR_SIZE_SUPPORTED_1KBITS: i32 = 2;
pub const FPB_VECTOR_SIZE_SUPPORTED_256BITS: i32 = 0;
pub const FPB_VECTOR_SIZE_SUPPORTED_2KBITS: i32 = 3;
pub const FPB_VECTOR_SIZE_SUPPORTED_4KBITS: i32 = 4;
pub const FPB_VECTOR_SIZE_SUPPORTED_512BITS: i32 = 1;
pub const FPB_VECTOR_SIZE_SUPPORTED_8KBITS: i32 = 5;
pub const FloatingPointProcessor: CONFIGURATION_TYPE = 2;
pub const FloppyDiskPeripheral: CONFIGURATION_TYPE = 26;
pub const GENERIC_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3e62a467_ab40_409a_a698_f362d464b38f);
pub const GENERIC_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe71254e8_c1b9_4940_ab76_909703a4320f);
pub const GENPROC_FLAGS_CORRECTED: i32 = 8;
pub const GENPROC_FLAGS_OVERFLOW: i32 = 4;
pub const GENPROC_FLAGS_PRECISEIP: i32 = 2;
pub const GENPROC_FLAGS_RESTARTABLE: i32 = 1;
pub const GENPROC_OP_DATAREAD: i32 = 1;
pub const GENPROC_OP_DATAWRITE: i32 = 2;
pub const GENPROC_OP_GENERIC: i32 = 0;
pub const GENPROC_OP_INSTRUCTIONEXE: i32 = 3;
pub const GENPROC_PROCERRTYPE_BUS: i32 = 4;
pub const GENPROC_PROCERRTYPE_CACHE: i32 = 1;
pub const GENPROC_PROCERRTYPE_MAE: i32 = 8;
pub const GENPROC_PROCERRTYPE_TLB: i32 = 2;
pub const GENPROC_PROCERRTYPE_UNKNOWN: i32 = 0;
pub const GENPROC_PROCISA_ARM32: i32 = 4;
pub const GENPROC_PROCISA_ARM64: i32 = 8;
pub const GENPROC_PROCISA_IPF: i32 = 1;
pub const GENPROC_PROCISA_X64: i32 = 2;
pub const GENPROC_PROCISA_X86: i32 = 0;
pub const GENPROC_PROCTYPE_ARM: i32 = 2;
pub const GENPROC_PROCTYPE_IPF: i32 = 1;
pub const GENPROC_PROCTYPE_XPF: i32 = 0;
pub const GUID_ECP_CREATE_USER_PROCESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe0e429ff_6ddc_4e65_aab6_45d05a038a08);
pub const GartHigh: EXTENDED_AGP_REGISTER = 5;
pub const GartLow: EXTENDED_AGP_REGISTER = 4;
pub const GenericEqual: RTL_GENERIC_COMPARE_RESULTS = 2;
pub const GenericGreaterThan: RTL_GENERIC_COMPARE_RESULTS = 1;
pub const GenericLessThan: RTL_GENERIC_COMPARE_RESULTS = 0;
pub const GetMemoryDetailsErr: WHEA_OFFLINE_ERRS = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct HAL_AMLI_BAD_IO_ADDRESS_LIST {
    pub BadAddrBegin: super::ULONG,
    pub BadAddrSize: super::ULONG,
    pub OSVersionTrigger: super::ULONG,
    pub IOHandler: PHALIOREADWRITEHANDLER,
}
pub type HAL_APIC_DESTINATION_MODE = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct HAL_BUS_INFORMATION {
    pub BusType: super::INTERFACE_TYPE,
    pub ConfigurationType: BUS_DATA_TYPE,
    pub BusNumber: super::ULONG,
    pub Reserved: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct HAL_CALLBACKS {
    pub SetSystemInformation: super::PCALLBACK_OBJECT,
    pub BusCheck: super::PCALLBACK_OBJECT,
}
#[cfg(feature = "shared")]
pub const HAL_CMC_DISABLED: super::ULONG = 0;
#[cfg(feature = "shared")]
pub const HAL_CMC_INTERRUPTS_BASED: super::ULONG = 4294967295;
#[cfg(feature = "shared")]
pub const HAL_CPE_DISABLED: super::ULONG = 0;
#[cfg(feature = "shared")]
pub const HAL_CPE_INTERRUPTS_BASED: super::ULONG = 4294967295;
pub const HAL_DISPATCH_VERSION: i32 = 7;
pub type HAL_DISPLAY_BIOS_INFORMATION = i32;
pub type HAL_DMA_CRASH_DUMP_REGISTER_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct HAL_ERROR_INFO {
    pub Version: super::ULONG,
    pub InitMaxSize: super::ULONG,
    pub McaMaxSize: super::ULONG,
    pub McaPreviousEventsCount: super::ULONG,
    pub McaCorrectedEventsCount: super::ULONG,
    pub McaKernelDeliveryFails: super::ULONG,
    pub McaDriverDpcQueueFails: super::ULONG,
    pub McaReserved: super::ULONG,
    pub CmcMaxSize: super::ULONG,
    pub CmcPollingInterval: super::ULONG,
    pub CmcInterruptsCount: super::ULONG,
    pub CmcKernelDeliveryFails: super::ULONG,
    pub CmcDriverDpcQueueFails: super::ULONG,
    pub CmcGetStateFails: super::ULONG,
    pub CmcClearStateFails: super::ULONG,
    pub CmcReserved: super::ULONG,
    pub CmcLogId: super::ULONGLONG,
    pub CpeMaxSize: super::ULONG,
    pub CpePollingInterval: super::ULONG,
    pub CpeInterruptsCount: super::ULONG,
    pub CpeKernelDeliveryFails: super::ULONG,
    pub CpeDriverDpcQueueFails: super::ULONG,
    pub CpeGetStateFails: super::ULONG,
    pub CpeClearStateFails: super::ULONG,
    pub CpeInterruptSources: super::ULONG,
    pub CpeLogId: super::ULONGLONG,
    pub KernelReserved: [super::ULONGLONG; 4],
}
#[cfg(feature = "shared")]
impl Default for HAL_ERROR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub const HAL_MCA_DISABLED: super::ULONG = 0;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HAL_MCA_INTERFACE {
    pub Lock: PHALMCAINTERFACELOCK,
    pub Unlock: PHALMCAINTERFACEUNLOCK,
    pub ReadRegister: PHALMCAINTERFACEREADREGISTER,
}
#[cfg(feature = "shared")]
pub const HAL_MCA_INTERRUPTS_BASED: super::ULONG = 4294967295;
#[cfg(feature = "shared")]
pub const HAL_MCE_DISABLED: super::ULONG = 0;
#[cfg(feature = "shared")]
pub const HAL_MCE_INTERRUPTS_BASED: super::ULONG = 4294967295;
pub const HAL_PLATFORM_ACPI_TABLES_CACHED: i32 = 32;
pub const HAL_PLATFORM_DISABLE_PTCG: i32 = 4;
pub const HAL_PLATFORM_DISABLE_UC_MAIN_MEMORY: i32 = 8;
pub const HAL_PLATFORM_DISABLE_WRITE_COMBINING: i32 = 1;
pub const HAL_PLATFORM_ENABLE_WRITE_COMBINING_MMIO: i32 = 16;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HAL_PLATFORM_INFORMATION {
    pub PlatformFlags: super::ULONG,
}
pub const HAL_PMU_NOTIFCATION_CALLBACK_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HAL_PMU_NOTIFICATION {
    pub Version: super::USHORT,
    pub Size: super::USHORT,
    pub NotificationType: HAL_PMU_NOTIFICATION_TYPE,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type HAL_PMU_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(context: super::PVOID, notification: PHAL_PMU_NOTIFICATION)>;
pub type HAL_PMU_NOTIFICATION_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HAL_POST_SLEEP_STATS {
    pub PostSleepTimeMs: super::ULONG,
    pub PostSleepMPTimeMs: super::ULONG,
    pub InterruptReinitTimeMs: super::ULONG,
    pub IommuReinitTimeMs: super::ULONG,
    pub WakeProcessorsTimeMs: super::ULONG,
    pub UpdateMicrocodeTimeMs: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HAL_POWER_INFORMATION {
    pub TBD: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HAL_PROCESSOR_FEATURE {
    pub UsableFeatureBits: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HAL_PROCESSOR_SPEED_INFORMATION {
    pub ProcessorSpeed: super::ULONG,
}
pub type HAL_QUERY_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HAL_REGISTER_PMU_NOTIFICATION_INPUT {
    pub Version: super::USHORT,
    pub Size: super::USHORT,
    pub OwnerTag: super::ULONG,
    pub CallbackRoutine: PHAL_PMU_NOTIFICATION_CALLBACK,
    pub CallbackContext: super::PVOID,
}
pub const HAL_REGISTER_PMU_NOTIFICATION_INPUT_VERSION: i32 = 1;
pub type HAL_SET_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct HAL_UNREGISTER_PMU_NOTIFICATION_INPUT {
    pub Version: super::USHORT,
    pub Size: super::USHORT,
    pub OwnerTag: super::ULONG,
}
pub const HAL_UNREGISTER_PMU_NOTIFICATION_INPUT_VERSION: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct HARDWARE_COUNTER {
    pub Type: super::HARDWARE_COUNTER_TYPE,
    pub Reserved: super::ULONG,
    pub Index: super::ULONG64,
}
#[cfg(feature = "shared")]
pub type HVL_WHEA_ERROR_NOTIFICATION = Option<unsafe extern "system" fn(recoverycontext: PWHEA_RECOVERY_CONTEXT, poisoned: super::BOOLEAN) -> windows_sys::core::NTSTATUS>;
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
pub const IMAGE_ADDRESSING_MODE_32BIT: i32 = 3;
pub const INIT_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcc5263e8_9308_454a_89d0_340bd39bc98e);
pub const INJECT_ERRTYPE_MEMORY_CORRECTABLE: i32 = 8;
pub const INJECT_ERRTYPE_MEMORY_UNCORRECTABLEFATAL: i32 = 32;
pub const INJECT_ERRTYPE_MEMORY_UNCORRECTABLENONFATAL: i32 = 16;
pub const INJECT_ERRTYPE_PCIEXPRESS_CORRECTABLE: i32 = 64;
pub const INJECT_ERRTYPE_PCIEXPRESS_UNCORRECTABLEFATAL: i32 = 256;
pub const INJECT_ERRTYPE_PCIEXPRESS_UNCORRECTABLENONFATAL: i32 = 128;
pub const INJECT_ERRTYPE_PLATFORM_CORRECTABLE: i32 = 512;
pub const INJECT_ERRTYPE_PLATFORM_UNCORRECTABLEFATAL: i32 = 2048;
pub const INJECT_ERRTYPE_PLATFORM_UNCORRECTABLENONFATAL: i32 = 1024;
pub const INJECT_ERRTYPE_PLATFORM_VENDOR_DEFINED: u32 = 2147483648;
pub const INJECT_ERRTYPE_PROCESSOR_CORRECTABLE: i32 = 1;
pub const INJECT_ERRTYPE_PROCESSOR_UNCORRECTABLEFATAL: i32 = 4;
pub const INJECT_ERRTYPE_PROCESSOR_UNCORRECTABLENONFATAL: i32 = 2;
pub const INTEL_ADDRESS_TRANSLATION_PRM_HANDLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1de4055d_d2f3_4e11_b7d9_7d6c19173fee);
#[cfg(target_arch = "x86")]
pub type INTERLOCKED_RESULT = i32;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type INTERLOCKED_RESULT = i32;
pub const IOCTL_IPMI_INTERNAL_RECORD_SEL_EVENT: i32 = 2232320;
pub const IO_ATTACH_DEVICE: i32 = 1024;
pub const IO_CHECK_CREATE_PARAMETERS: i32 = 512;
#[repr(C)]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct IO_DRIVER_CREATE_CONTEXT {
    pub Size: super::CSHORT,
    pub ExtraCreateParameter: *mut _ECP_LIST,
    pub DeviceObjectHint: super::PVOID,
    pub TxnParameters: PTXN_PARAMETER_BLOCK,
    pub SiloContext: PESILO,
}
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct IO_FOEXT_SHADOW_FILE {
    pub BackingFileObject: super::PFILE_OBJECT,
    pub BackingFltInstance: super::PVOID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IO_FOEXT_SILO_PARAMETERS {
    pub Length: super::ULONG,
    pub Anonymous: IO_FOEXT_SILO_PARAMETERS_0,
    pub SiloContext: PESILO,
}
#[cfg(feature = "shared")]
impl Default for IO_FOEXT_SILO_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union IO_FOEXT_SILO_PARAMETERS_0 {
    pub Anonymous: IO_FOEXT_SILO_PARAMETERS_0_0,
    pub Flags: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for IO_FOEXT_SILO_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct IO_FOEXT_SILO_PARAMETERS_0_0 {
    pub _bitfield: u32,
}
pub const IO_IGNORE_SHARE_ACCESS_CHECK: i32 = 2048;
pub const IO_KEYBOARD_INCREMENT: i32 = 6;
pub const IO_MOUSE_INCREMENT: i32 = 6;
pub const IO_PARALLEL_INCREMENT: i32 = 1;
pub type IO_QUERY_DEVICE_DATA_FORMAT = i32;
pub const IO_SERIAL_INCREMENT: i32 = 2;
pub const IO_SOUND_INCREMENT: i32 = 8;
pub const IO_USE_AMBIENT_SILO: PESILO = 1 as _;
pub const IO_VIDEO_INCREMENT: i32 = 1;
pub const IPF_PROCESSOR_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe429faf1_3cb7_11d4_bca7_0080c73c8881);
pub const IPF_SAL_RECORD_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f3380d1_6eb0_497f_a578_4d4c65a71617);
pub const IPMI_IOCTL_INDEX: i32 = 1024;
pub const IPMI_MSR_DUMP_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1c15b445_9b06_4667_ac25_33c056b88803);
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct IPMI_OS_SEL_RECORD {
    pub Signature: super::ULONG,
    pub Version: super::ULONG,
    pub Length: super::ULONG,
    pub RecordType: IPMI_OS_SEL_RECORD_TYPE,
    pub DataLength: super::ULONG,
    pub Data: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for IPMI_OS_SEL_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IPMI_OS_SEL_RECORD_MASK: i32 = 65535;
pub const IPMI_OS_SEL_RECORD_SIGNATURE: i32 = 1381192527;
pub type IPMI_OS_SEL_RECORD_TYPE = i32;
pub const IPMI_OS_SEL_RECORD_VERSION: i32 = 1;
pub const IPMI_OS_SEL_RECORD_VERSION_1: i32 = 1;
pub const IRP_MN_COMPLETE: i32 = 4;
pub const IRP_MN_COMPLETE_MDL: i32 = 6;
pub const IRP_MN_COMPLETE_MDL_DPC: i32 = 7;
pub const IRP_MN_COMPRESSED: i32 = 8;
pub const IRP_MN_DPC: i32 = 1;
pub const IRP_MN_FLUSH_AND_PURGE: i32 = 1;
pub const IRP_MN_FLUSH_DATA_ONLY: i32 = 2;
pub const IRP_MN_FLUSH_DATA_SYNC_ONLY: i32 = 4;
pub const IRP_MN_FLUSH_NO_SYNC: i32 = 3;
pub const IRP_MN_KERNEL_CALL: i32 = 4;
pub const IRP_MN_LOAD_FILE_SYSTEM: i32 = 3;
pub const IRP_MN_LOCK: i32 = 1;
pub const IRP_MN_MDL: i32 = 2;
pub const IRP_MN_MDL_DPC: i32 = 3;
pub const IRP_MN_MOUNT_VOLUME: i32 = 1;
pub const IRP_MN_NORMAL: i32 = 0;
pub const IRP_MN_NOTIFY_CHANGE_DIRECTORY: i32 = 2;
pub const IRP_MN_NOTIFY_CHANGE_DIRECTORY_EX: i32 = 3;
pub const IRP_MN_QUERY_DIRECTORY: i32 = 1;
pub const IRP_MN_QUERY_LEGACY_BUS_INFORMATION: i32 = 24;
pub const IRP_MN_TRACK_LINK: i32 = 4;
pub const IRP_MN_UNLOCK_ALL: i32 = 3;
pub const IRP_MN_UNLOCK_ALL_BY_KEY: i32 = 4;
pub const IRP_MN_UNLOCK_SINGLE: i32 = 2;
pub const IRP_MN_USER_FS_REQUEST: i32 = 0;
pub const IRP_MN_VERIFY_VOLUME: i32 = 2;
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
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KARM64_VFP_STATE {
    pub Link: *mut Self,
    pub Fpcr: super::ULONG,
    pub Fpsr: super::ULONG,
    pub V: [super::NEON128; 32],
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for KARM64_VFP_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KD_CALLBACK_ACTION = i32;
pub type KD_NAMESPACE_ENUM = i32;
#[cfg(target_arch = "x86")]
pub const KERNEL_LARGE_STACK_COMMIT: i32 = 12288;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const KERNEL_LARGE_STACK_COMMIT: i32 = 24576;
#[cfg(target_arch = "aarch64")]
pub const KERNEL_LARGE_STACK_COMMIT: i32 = 32768;
#[cfg(target_arch = "x86")]
pub const KERNEL_LARGE_STACK_SIZE: i32 = 61440;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const KERNEL_LARGE_STACK_SIZE: i32 = 73728;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const KERNEL_MCA_EXCEPTION_STACK_SIZE: i32 = 8192;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const KERNEL_SHADOW_STACK_SIZE: i32 = 4096;
#[cfg(target_arch = "x86")]
pub const KERNEL_STACK_SIZE: i32 = 12288;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const KERNEL_STACK_SIZE: i32 = 24576;
#[cfg(target_arch = "aarch64")]
pub const KERNEL_STACK_SIZE: i32 = 32768;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KERNEL_USER_TIMES {
    pub CreateTime: super::LARGE_INTEGER,
    pub ExitTime: super::LARGE_INTEGER,
    pub KernelTime: super::LARGE_INTEGER,
    pub UserTime: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for KERNEL_USER_TIMES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KEXCEPTION_FRAME {
    pub P1Home: super::ULONG64,
    pub P2Home: super::ULONG64,
    pub P3Home: super::ULONG64,
    pub P4Home: super::ULONG64,
    pub P5: super::ULONG64,
    pub Spare1: super::ULONG64,
    pub Xmm6: super::M128A,
    pub Xmm7: super::M128A,
    pub Xmm8: super::M128A,
    pub Xmm9: super::M128A,
    pub Xmm10: super::M128A,
    pub Xmm11: super::M128A,
    pub Xmm12: super::M128A,
    pub Xmm13: super::M128A,
    pub Xmm14: super::M128A,
    pub Xmm15: super::M128A,
    pub TrapFrame: super::ULONG64,
    pub OutputBuffer: super::ULONG64,
    pub OutputLength: super::ULONG64,
    pub Spare2: super::ULONG64,
    pub MxCsr: super::ULONG64,
    pub Rbp: super::ULONG64,
    pub Rbx: super::ULONG64,
    pub Rdi: super::ULONG64,
    pub Rsi: super::ULONG64,
    pub R12: super::ULONG64,
    pub R13: super::ULONG64,
    pub R14: super::ULONG64,
    pub R15: super::ULONG64,
    pub Return: super::ULONG64,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct KEXCEPTION_FRAME {
    pub X19: super::ULONG64,
    pub X20: super::ULONG64,
    pub X21: super::ULONG64,
    pub X22: super::ULONG64,
    pub X23: super::ULONG64,
    pub X24: super::ULONG64,
    pub X25: super::ULONG64,
    pub X26: super::ULONG64,
    pub X27: super::ULONG64,
    pub X28: super::ULONG64,
    pub Fp: super::ULONG64,
    pub Return: super::ULONG64,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KEY_CACHED_INFORMATION {
    pub LastWriteTime: super::LARGE_INTEGER,
    pub TitleIndex: super::ULONG,
    pub SubKeys: super::ULONG,
    pub MaxNameLen: super::ULONG,
    pub Values: super::ULONG,
    pub MaxValueNameLen: super::ULONG,
    pub MaxValueDataLen: super::ULONG,
    pub NameLength: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for KEY_CACHED_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct KEY_LAYER_INFORMATION {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KEY_NAME_INFORMATION {
    pub NameLength: super::ULONG,
    pub Name: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for KEY_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct KEY_VIRTUALIZATION_INFORMATION {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KFRED_TRAP_FRAME {
    pub TrapFrame: KTRAP_FRAME,
    pub EventInformation: super::ULONG64,
    pub Reserved: super::ULONG64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KFRED_TRAP_FRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const KFRED_TRAP_FRAME_LENGTH: u64 = 416;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KTRAP_FRAME {
    pub P1Home: super::ULONG64,
    pub P2Home: super::ULONG64,
    pub P3Home: super::ULONG64,
    pub P4Home: super::ULONG64,
    pub P5: super::ULONG64,
    pub PreviousMode: super::KPROCESSOR_MODE,
    pub PreviousIrql: super::KIRQL,
    pub FaultIndicator: super::UCHAR,
    pub ExceptionActive: super::UCHAR,
    pub MxCsr: super::ULONG,
    pub Rax: super::ULONG64,
    pub Rcx: super::ULONG64,
    pub Rdx: super::ULONG64,
    pub R8: super::ULONG64,
    pub R9: super::ULONG64,
    pub R10: super::ULONG64,
    pub R11: super::ULONG64,
    pub Anonymous: KTRAP_FRAME_0,
    pub Xmm0: super::M128A,
    pub Xmm1: super::M128A,
    pub Xmm2: super::M128A,
    pub Xmm3: super::M128A,
    pub Xmm4: super::M128A,
    pub Xmm5: super::M128A,
    pub Anonymous2: KTRAP_FRAME_1,
    pub Anonymous3: KTRAP_FRAME_2,
    pub Anonymous4: KTRAP_FRAME_3,
    pub SegDs: super::USHORT,
    pub SegEs: super::USHORT,
    pub SegFs: super::USHORT,
    pub SegGs: super::USHORT,
    pub TrapFrame: super::ULONG64,
    pub Rbx: super::ULONG64,
    pub Rdi: super::ULONG64,
    pub Rsi: super::ULONG64,
    pub Rbp: super::ULONG64,
    pub Anonymous5: KTRAP_FRAME_4,
    pub Rip: super::ULONG64,
    pub Anonymous6: KTRAP_FRAME_5,
    pub EFlags: super::ULONG,
    pub Fill2: super::ULONG,
    pub Rsp: super::ULONG64,
    pub Anonymous7: KTRAP_FRAME_6,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_0 {
    pub GsBase: super::ULONG64,
    pub GsSwap: super::ULONG64,
    pub VectorMask: super::ULONG64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_1 {
    pub FaultAddress: super::ULONG64,
    pub ContextRecord: super::ULONG64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_2 {
    pub Anonymous: KTRAP_FRAME_2_0,
    pub Anonymous2: KTRAP_FRAME_2_1,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KTRAP_FRAME_2_0 {
    pub Dr0: super::ULONG64,
    pub Dr1: super::ULONG64,
    pub Dr2: super::ULONG64,
    pub Dr3: super::ULONG64,
    pub Dr6: super::ULONG64,
    pub Dr7: super::ULONG64,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KTRAP_FRAME_2_1 {
    pub ShadowStackFrame: super::ULONG64,
    pub Spare: [super::ULONG64; 5],
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME_2_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KTRAP_FRAME_3 {
    pub DebugControl: super::ULONG64,
    pub LastBranchToRip: super::ULONG64,
    pub LastBranchFromRip: super::ULONG64,
    pub LastExceptionToRip: super::ULONG64,
    pub LastExceptionFromRip: super::ULONG64,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_4 {
    pub ErrorCode: super::ULONG64,
    pub ExceptionFrame: super::ULONG64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME_4 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_5 {
    pub Anonymous: KTRAP_FRAME_5_0,
    pub Anonymous2: KTRAP_FRAME_5_1,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KTRAP_FRAME_5_0 {
    pub SegCs: super::USHORT,
    pub Fill0: super::UCHAR,
    pub Logging: super::UCHAR,
    pub Fill1: [super::USHORT; 2],
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME_5_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KTRAP_FRAME_5_1 {
    pub _bitfield: u64,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_6 {
    pub Anonymous: KTRAP_FRAME_6_0,
    pub Anonymous2: KTRAP_FRAME_6_1,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KTRAP_FRAME_6_0 {
    pub SegSs: super::USHORT,
    pub Fill3: super::USHORT,
    pub Fill4: super::ULONG,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KTRAP_FRAME_6_1 {
    pub Anonymous: KTRAP_FRAME_6_1_0,
    pub _bitfield: u32,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME_6_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_6_1_0 {
    pub Anonymous: KTRAP_FRAME_6_1_0_0,
    pub Anonymous2: KTRAP_FRAME_6_1_0_1,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME_6_1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KTRAP_FRAME_6_1_0_0 {
    pub EventInformation: super::ULONG,
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KTRAP_FRAME_6_1_0_1 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KTRAP_FRAME {
    pub ExceptionActive: super::UCHAR,
    pub ContextFromKFramesUnwound: super::UCHAR,
    pub DebugRegistersValid: super::UCHAR,
    pub Anonymous: KTRAP_FRAME_0,
    pub Reserved: super::ULONG,
    pub Anonymous2: KTRAP_FRAME_1,
    pub VfpState: PKARM64_VFP_STATE,
    pub Bcr: [super::ULONG; 8],
    pub Bvr: [super::ULONG64; 8],
    pub Wcr: [super::ULONG; 2],
    pub Wvr: [super::ULONG64; 2],
    pub Spsr: super::ULONG,
    pub Esr: super::ULONG,
    pub Sp: super::ULONG64,
    pub Anonymous3: KTRAP_FRAME_2,
    pub Lr: super::ULONG64,
    pub Fp: super::ULONG64,
    pub Pc: super::ULONG64,
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_0 {
    pub PreviousMode: super::KPROCESSOR_MODE,
    pub PreviousIrql: super::KIRQL,
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_1 {
    pub FaultAddress: super::ULONG64,
    pub TrapFrame: super::ULONG64,
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KTRAP_FRAME_2 {
    pub X: [super::ULONG64; 19],
    pub Anonymous: KTRAP_FRAME_2_0,
}
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KTRAP_FRAME_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KTRAP_FRAME_2_0 {
    pub X0: super::ULONG64,
    pub X1: super::ULONG64,
    pub X2: super::ULONG64,
    pub X3: super::ULONG64,
    pub X4: super::ULONG64,
    pub X5: super::ULONG64,
    pub X6: super::ULONG64,
    pub X7: super::ULONG64,
    pub X8: super::ULONG64,
    pub X9: super::ULONG64,
    pub X10: super::ULONG64,
    pub X11: super::ULONG64,
    pub X12: super::ULONG64,
    pub X13: super::ULONG64,
    pub X14: super::ULONG64,
    pub X15: super::ULONG64,
    pub X16: super::ULONG64,
    pub X17: super::ULONG64,
    pub X18: super::ULONG64,
}
#[cfg(target_arch = "aarch64")]
pub const KTRAP_FRAME_ARGUMENTS: i32 = 80;
#[cfg(target_arch = "aarch64")]
pub const KTRAP_FRAME_SIGNATURE: i32 = 1886540875;
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct KUMS_CONTEXT_HEADER {
    pub P1Home: super::ULONG64,
    pub P2Home: super::ULONG64,
    pub P3Home: super::ULONG64,
    pub P4Home: super::ULONG64,
    pub StackTop: super::PVOID,
    pub StackSize: super::ULONG64,
    pub RspOffset: super::ULONG64,
    pub Rip: super::ULONG64,
    pub FltSave: super::PXMM_SAVE_AREA32,
    pub Anonymous: KUMS_CONTEXT_HEADER_0,
    pub TrapFrame: PKTRAP_FRAME,
    pub ExceptionFrame: PKEXCEPTION_FRAME,
    pub SourceThread: *mut super::_KTHREAD,
    pub Return: super::ULONG64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KUMS_CONTEXT_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union KUMS_CONTEXT_HEADER_0 {
    pub Anonymous: KUMS_CONTEXT_HEADER_0_0,
    pub Flags: super::ULONG64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for KUMS_CONTEXT_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct KUMS_CONTEXT_HEADER_0_0 {
    pub _bitfield: u64,
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const KUMS_UCH_VOLATILE_BIT: i32 = 0;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
pub const KUMS_UCH_VOLATILE_MASK: u64 = 1;
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
pub const LINKSPEED_MTS_16000: i32 = 16000;
pub const LINKSPEED_MTS_2500: i32 = 2500;
pub const LINKSPEED_MTS_32000: i32 = 32000;
pub const LINKSPEED_MTS_5000: i32 = 5000;
pub const LINKSPEED_MTS_64000: i32 = 64000;
pub const LINKSPEED_MTS_8000: i32 = 8000;
pub const LinePeripheral: CONFIGURATION_TYPE = 35;
pub const LocationTypeFileSystem: STATE_LOCATION_TYPE = 1;
pub const LocationTypeMaximum: STATE_LOCATION_TYPE = 2;
pub const LocationTypeRegistry: STATE_LOCATION_TYPE = 0;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MAP_REGISTER_ENTRY {
    pub MapRegister: super::PVOID,
    pub WriteToDevice: super::BOOLEAN,
}
pub const MAXIMUM_DEBUG_BARS: i32 = 6;
#[cfg(target_arch = "x86")]
pub const MAXIMUM_EXPANSION_SIZE: i32 = 59392;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const MAXIMUM_EXPANSION_SIZE: i32 = 71680;
pub const MAX_SEL_RAW_EVENT_PAYLOAD_LENGTH: i32 = 256;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "mce", feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct MCA_DRIVER_INFO {
    pub ExceptionCallback: PDRIVER_MCA_EXCEPTION_CALLBACK,
    pub DpcCallback: super::PKDEFERRED_ROUTINE,
    pub DeviceContext: super::PVOID,
}
pub const MCE_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe8f56ffe_919c_4cc5_ba88_65abe14913bb);
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MCG_CAP {
    pub Anonymous: MCG_CAP_0,
    pub QuadPart: super::ULONG64,
}
#[cfg(feature = "shared")]
impl Default for MCG_CAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MCG_CAP_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MCG_STATUS {
    pub Anonymous: MCG_STATUS_0,
    pub QuadPart: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for MCG_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MCG_STATUS_0 {
    pub _bitfield: u32,
    pub Reserved2: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MCI_STATUS {
    pub CommonBits: MCI_STATUS_BITS_COMMON,
    pub AmdBits: MCI_STATUS_AMD_BITS,
    pub IntelBits: MCI_STATUS_INTEL_BITS,
    pub QuadPart: super::ULONG64,
}
#[cfg(feature = "shared")]
impl Default for MCI_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MCI_STATUS: i32 = 2068;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MCI_STATUS_AMD_BITS {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MCI_STATUS_BITS_COMMON {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MCI_STATUS_INTEL_BITS {
    pub _bitfield: u64,
}
pub const MEMORY_CORRECTABLE_ERROR_SUMMARY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0e36c93e_ca15_4a83_ba8a_cbe80f7f0017);
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct MEMORY_DEFECT {
    pub Version: super::UINT16,
    pub Flags: MEMORY_DEFECT_FLAGS,
    pub DimmInfo: DIMM_INFO,
    pub ErrType: PAGE_OFFLINE_ERROR_TYPES,
}
#[cfg(feature = "shared")]
impl Default for MEMORY_DEFECT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union MEMORY_DEFECT_FLAGS {
    pub Anonymous: MEMORY_DEFECT_FLAGS_0,
    pub AsUINT16: super::UINT16,
}
#[cfg(feature = "shared")]
impl Default for MEMORY_DEFECT_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MEMORY_DEFECT_FLAGS_0 {
    pub _bitfield: u16,
}
pub const MEMORY_ERROR_EXT_SECTION_AMD_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0d3a4a3b_774a_4c72_95e2_152d43180374);
pub const MEMORY_ERROR_EXT_SECTION_INTEL_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe16edb28_6113_4263_a41d_e53f8de78751);
pub const MEMORY_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa5bc1114_6f64_4ede_b863_3e83ed7c83b1);
pub const MM_ADD_PHYSICAL_MEMORY_ALREADY_ZEROED: i32 = 1;
pub const MM_ADD_PHYSICAL_MEMORY_HUGE_PAGES_ONLY: i32 = 4;
pub const MM_ADD_PHYSICAL_MEMORY_LARGE_PAGES_ONLY: i32 = 2;
pub const MM_ALLOCATE_CONTIGUOUS_MEMORY_FAST_ONLY: i32 = 1;
#[cfg(feature = "winnt")]
pub const MM_ALL_PARTITIONS_OBJECT: super::PVOID = -2 as _;
pub const MM_ANY_NODE_OK: u32 = 2147483648;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct MM_COPY_ADDRESS {
    pub Anonymous: MM_COPY_ADDRESS_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MM_COPY_ADDRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union MM_COPY_ADDRESS_0 {
    pub VirtualAddress: super::PVOID,
    pub PhysicalAddress: super::PHYSICAL_ADDRESS,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for MM_COPY_ADDRESS_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MM_COPY_MEMORY_PHYSICAL: i32 = 1;
pub const MM_COPY_MEMORY_VIRTUAL: i32 = 2;
#[cfg(feature = "winnt")]
pub const MM_CURRENT_PROCESS_PARTITION_OBJECT: super::PVOID = -1 as _;
pub const MM_GET_CACHE_ATTRIBUTE_IO_SPACE: i32 = 1;
pub const MM_GET_PHYSICAL_MEMORY_RANGES_INCLUDE_ALL_PARTITIONS: i32 = 2;
pub const MM_GET_PHYSICAL_MEMORY_RANGES_INCLUDE_FILE_ONLY: i32 = 1;
#[cfg(feature = "winnt")]
pub const MM_LOWEST_USER_ADDRESS: super::PVOID = 65536 as _;
pub const MM_REMOVE_PHYSICAL_MEMORY_BAD_ONLY: i32 = 1;
pub type MM_ROTATE_DIRECTION = i32;
pub const MM_SECURE_EXCLUSIVE: i32 = 1;
pub const MM_SECURE_NO_CHANGE: i32 = 2;
pub const MM_SECURE_NO_INHERIT: i32 = 8;
pub const MM_SECURE_USER_MODE_ONLY: i32 = 4;
pub const MM_SYSTEM_PARTITION_OBJECT: i32 = 0;
#[cfg(target_arch = "x86")]
pub const MM_SYSTEM_SPACE_END: u32 = 4294967295;
pub const MM_SYSTEM_VIEW_EXCEPTIONS_FOR_INPAGE_ERRORS: i32 = 1;
pub const MPIConfiguration: BUS_DATA_TYPE = 8;
pub const MPSAConfiguration: BUS_DATA_TYPE = 9;
pub const MRLClosed: PCI_EXPRESS_MRL_STATE = 0;
pub const MRLOpen: PCI_EXPRESS_MRL_STATE = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct MU_TELEMETRY_SECTION {
    pub ComponentID: windows_sys::core::GUID,
    pub SubComponentID: windows_sys::core::GUID,
    pub Reserved: super::UINT32,
    pub ErrorStatusValue: super::UINT32,
    pub AdditionalInfo1: super::UINT64,
    pub AdditionalInfo2: super::UINT64,
}
pub const MU_TELEMETRY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x85183a8b_9c41_429c_939c_5c3c087ca280);
pub const MaxPayload1024Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 3;
pub const MaxPayload128Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 0;
pub const MaxPayload2048Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 4;
pub const MaxPayload256Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 1;
pub const MaxPayload4096Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 5;
pub const MaxPayload512Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 2;
pub const MaxProcessInfoClass: PROCESSINFOCLASS = 118;
pub const MaxSubsystemInformationType: SUBSYSTEM_INFORMATION_TYPE = 2;
pub const MaxThreadInfoClass: THREADINFOCLASS = 60;
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
pub const NX_SUPPORT_POLICY_ALWAYSOFF: i32 = 0;
pub const NX_SUPPORT_POLICY_ALWAYSON: i32 = 1;
pub const NX_SUPPORT_POLICY_OPTIN: i32 = 2;
pub const NX_SUPPORT_POLICY_OPTOUT: i32 = 3;
pub const NetworkController: CONFIGURATION_TYPE = 18;
pub const NetworkPeripheral: CONFIGURATION_TYPE = 36;
pub const NoAspmSupport: PCI_EXPRESS_ASPM_SUPPORT = 0;
pub const NoCxlSupport: CXL_PROTOCOL_VERSION = 0;
pub const NoMemoryForWrapper: WHEA_OFFLINE_ERRS = 11;
pub const NotDdr: WHEA_OFFLINE_ERRS = 8;
pub const NuBusConfiguration: BUS_DATA_TYPE = 6;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct OPLOCK_KEY_CONTEXT {
    pub Version: super::USHORT,
    pub Flags: super::USHORT,
    pub ParentOplockKey: windows_sys::core::GUID,
    pub TargetOplockKey: windows_sys::core::GUID,
    pub Reserved: super::ULONG,
}
pub const OPLOCK_KEY_FLAG_PARENT_KEY: i32 = 1;
pub const OPLOCK_KEY_FLAG_TARGET_KEY: i32 = 2;
pub const OPLOCK_KEY_VERSION_WIN7: i32 = 1;
pub const OPLOCK_KEY_VERSION_WIN8: i32 = 2;
pub const OSC_CAPABILITIES_MASKED: i32 = 16;
pub const OSC_FIRMWARE_FAILURE: i32 = 2;
pub const OSC_UNRECOGNIZED_REVISION: i32 = 8;
pub const OSC_UNRECOGNIZED_UUID: i32 = 4;
pub const OtherController: CONFIGURATION_TYPE = 24;
pub const OtherPeripheral: CONFIGURATION_TYPE = 34;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PACPI_DEBUGGING_DEVICE_IN_USE = *mut ACPI_DEBUGGING_DEVICE_IN_USE;
#[cfg(feature = "shared")]
pub type PAER_BRIDGE_DESCRIPTOR_FLAGS = *mut AER_BRIDGE_DESCRIPTOR_FLAGS;
#[cfg(feature = "shared")]
pub type PAER_ENDPOINT_DESCRIPTOR_FLAGS = *mut AER_ENDPOINT_DESCRIPTOR_FLAGS;
#[cfg(feature = "shared")]
pub type PAER_ROOTPORT_DESCRIPTOR_FLAGS = *mut AER_ROOTPORT_DESCRIPTOR_FLAGS;
pub type PAGE_OFFLINE_ERROR_TYPES = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PAGE_OFFLINE_VALID_BITS {
    pub Anonymous: PAGE_OFFLINE_VALID_BITS_0,
    pub AsUINT8: super::UINT8,
}
#[cfg(feature = "shared")]
impl Default for PAGE_OFFLINE_VALID_BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PAGE_OFFLINE_VALID_BITS_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PAGE_PRIORITY_INFORMATION {
    pub PagePriority: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PAGP_TARGET_BUS_INTERFACE_STANDARD = *mut AGP_TARGET_BUS_INTERFACE_STANDARD;
pub type PARBITER_ACTION = *mut ARBITER_ACTION;
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PARBITER_ADD_RESERVED_PARAMETERS = *mut ARBITER_ADD_RESERVED_PARAMETERS;
#[cfg(feature = "winnt")]
pub type PARBITER_BOOT_ALLOCATION_PARAMETERS = *mut ARBITER_BOOT_ALLOCATION_PARAMETERS;
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PARBITER_CONFLICT_INFO = *mut ARBITER_CONFLICT_INFO;
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PARBITER_HANDLER = Option<unsafe extern "system" fn(context: super::PVOID, action: ARBITER_ACTION, parameters: PARBITER_PARAMETERS) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PARBITER_INTERFACE = *mut ARBITER_INTERFACE;
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PARBITER_LIST_ENTRY = *mut ARBITER_LIST_ENTRY;
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PARBITER_PARAMETERS = *mut ARBITER_PARAMETERS;
#[cfg(all(feature = "basetsd", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PARBITER_QUERY_ALLOCATED_RESOURCES_PARAMETERS = *mut ARBITER_QUERY_ALLOCATED_RESOURCES_PARAMETERS;
#[cfg(feature = "winnt")]
pub type PARBITER_QUERY_ARBITRATE_PARAMETERS = *mut ARBITER_QUERY_ARBITRATE_PARAMETERS;
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PARBITER_QUERY_CONFLICT_PARAMETERS = *mut ARBITER_QUERY_CONFLICT_PARAMETERS;
#[cfg(all(feature = "basetsd", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PARBITER_RETEST_ALLOCATION_PARAMETERS = *mut ARBITER_RETEST_ALLOCATION_PARAMETERS;
#[cfg(all(feature = "basetsd", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PARBITER_TEST_ALLOCATION_PARAMETERS = *mut ARBITER_TEST_ALLOCATION_PARAMETERS;
pub type PBDCB_CALLBACK_TYPE = *mut BDCB_CALLBACK_TYPE;
pub type PBDCB_CLASSIFICATION = *mut BDCB_CLASSIFICATION;
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
pub type PBDCB_IMAGE_INFORMATION = *mut BDCB_IMAGE_INFORMATION;
pub type PBDCB_STATUS_UPDATE_CONTEXT = *mut BDCB_STATUS_UPDATE_CONTEXT;
pub type PBDCB_STATUS_UPDATE_TYPE = *mut BDCB_STATUS_UPDATE_TYPE;
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
pub type PBOOT_DRIVER_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(callbackcontext: super::PVOID, classification: BDCB_CALLBACK_TYPE, imageinformation: PBDCB_IMAGE_INFORMATION)>;
pub type PBUS_DATA_TYPE = *mut BUS_DATA_TYPE;
pub const PCCARD_DEVICE_PCI: i32 = 16;
pub const PCCARD_DUP_LEGACY_BASE: i32 = 6;
pub const PCCARD_MAP_ERROR: i32 = 1;
pub const PCCARD_MAP_ZERO: i32 = 2;
pub const PCCARD_NO_CONTROLLERS: i32 = 7;
pub const PCCARD_NO_LEGACY_BASE: i32 = 5;
pub const PCCARD_NO_PIC: i32 = 4;
pub const PCCARD_NO_TIMER: i32 = 3;
pub const PCCARD_SCAN_DISABLED: i32 = 1;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PCDEBUG_DEVICE_DESCRIPTOR = *const DEBUG_DEVICE_DESCRIPTOR;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PCIBUSDATA {
    pub Tag: super::ULONG,
    pub Version: super::ULONG,
    pub ReadConfig: PciReadWriteConfig,
    pub WriteConfig: PciReadWriteConfig,
    pub Pin2Line: PciPin2Line,
    pub Line2Pin: PciLine2Pin,
    pub ParentSlot: super::PCI_SLOT_NUMBER,
    pub Reserved: [super::PVOID; 4],
}
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
impl Default for PCIBUSDATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PCIConfiguration: BUS_DATA_TYPE = 4;
pub const PCIEXPRESS_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd995e954_bbc1_430f_ad91_b44dcb3c6f35);
pub const PCIE_CORRECTABLE_ERROR_SUMMARY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe96eca99_53e2_4f52_9be7_d2dbe9508ed0);
pub const PCIXBUS_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc5753963_3b84_4095_bf78_eddad3f9c9dd);
pub const PCIXBUS_ERRTYPE_ADDRESSPARITY: i32 = 6;
pub const PCIXBUS_ERRTYPE_BUSTIMEOUT: i32 = 4;
pub const PCIXBUS_ERRTYPE_COMMANDPARITY: i32 = 7;
pub const PCIXBUS_ERRTYPE_DATAPARITY: i32 = 1;
pub const PCIXBUS_ERRTYPE_MASTERABORT: i32 = 3;
pub const PCIXBUS_ERRTYPE_MASTERDATAPARITY: i32 = 5;
pub const PCIXBUS_ERRTYPE_SYSTEM: i32 = 2;
pub const PCIXBUS_ERRTYPE_UNKNOWN: i32 = 0;
pub const PCIXDEVICE_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeb5e4685_ca66_4769_b6a2_26068b001326);
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCIX_BRIDGE_CAPABILITY {
    pub Header: super::PCI_CAPABILITIES_HEADER,
    pub SecondaryStatus: PCIX_BRIDGE_CAPABILITY_0,
    pub BridgeStatus: PCIX_BRIDGE_CAPABILITY_1,
    pub UpstreamSplitTransactionCapacity: super::USHORT,
    pub UpstreamSplitTransactionLimit: super::USHORT,
    pub DownstreamSplitTransactionCapacity: super::USHORT,
    pub DownstreamSplitTransactionLimit: super::USHORT,
    pub EccControlStatus: PCIX_BRIDGE_CAPABILITY_2,
    pub EccFirstAddress: super::ULONG,
    pub EccSecondAddress: super::ULONG,
    pub EccAttribute: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCIX_BRIDGE_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub union PCIX_BRIDGE_CAPABILITY_0 {
    pub Anonymous: PCIX_BRIDGE_CAPABILITY_0_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCIX_BRIDGE_CAPABILITY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct PCIX_BRIDGE_CAPABILITY_0_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub union PCIX_BRIDGE_CAPABILITY_1 {
    pub Anonymous: PCIX_BRIDGE_CAPABILITY_1_0,
    pub AsULONG: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCIX_BRIDGE_CAPABILITY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct PCIX_BRIDGE_CAPABILITY_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub union PCIX_BRIDGE_CAPABILITY_2 {
    pub Anonymous: PCIX_BRIDGE_CAPABILITY_2_0,
    pub AsULONG: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCIX_BRIDGE_CAPABILITY_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct PCIX_BRIDGE_CAPABILITY_2_0 {
    pub _bitfield: u32,
}
pub const PCIX_MODE1_100MHZ: i32 = 2;
pub const PCIX_MODE1_133MHZ: i32 = 3;
pub const PCIX_MODE1_66MHZ: i32 = 1;
pub const PCIX_MODE2_266_100MHZ: i32 = 10;
pub const PCIX_MODE2_266_133MHZ: i32 = 11;
pub const PCIX_MODE2_266_66MHZ: i32 = 9;
pub const PCIX_MODE2_533_100MHZ: i32 = 14;
pub const PCIX_MODE2_533_133MHZ: i32 = 15;
pub const PCIX_MODE2_533_66MHZ: i32 = 13;
pub const PCIX_MODE_CONVENTIONAL_PCI: i32 = 0;
pub const PCIX_VERSION_DUAL_MODE_ECC: i32 = 2;
pub const PCIX_VERSION_MODE1_ONLY: i32 = 0;
pub const PCIX_VERSION_MODE2_ECC: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_ADVANCED_FEATURES_CAPABILITY {
    pub Header: super::PCI_CAPABILITIES_HEADER,
    pub Length: super::UCHAR,
    pub Capabilities: PCI_ADVANCED_FEATURES_CAPABILITY_0,
    pub Control: PCI_ADVANCED_FEATURES_CAPABILITY_1,
    pub Status: PCI_ADVANCED_FEATURES_CAPABILITY_2,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_ADVANCED_FEATURES_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub union PCI_ADVANCED_FEATURES_CAPABILITY_0 {
    pub Anonymous: PCI_ADVANCED_FEATURES_CAPABILITY_0_0,
    pub AsUCHAR: super::UCHAR,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_ADVANCED_FEATURES_CAPABILITY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct PCI_ADVANCED_FEATURES_CAPABILITY_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub union PCI_ADVANCED_FEATURES_CAPABILITY_1 {
    pub Anonymous: PCI_ADVANCED_FEATURES_CAPABILITY_1_0,
    pub AsUCHAR: super::UCHAR,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_ADVANCED_FEATURES_CAPABILITY_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct PCI_ADVANCED_FEATURES_CAPABILITY_1_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub union PCI_ADVANCED_FEATURES_CAPABILITY_2 {
    pub Anonymous: PCI_ADVANCED_FEATURES_CAPABILITY_2_0,
    pub AsUCHAR: super::UCHAR,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_ADVANCED_FEATURES_CAPABILITY_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct PCI_ADVANCED_FEATURES_CAPABILITY_2_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_APERTURE_PAGE_SIZE {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_CAPABILITY {
    pub Header: super::PCI_CAPABILITIES_HEADER,
    pub _bitfield: u16,
    pub AGPStatus: PCI_AGP_CAPABILITY__PCI_AGP_STATUS,
    pub AGPCommand: PCI_AGP_CAPABILITY__PCI_AGP_COMMAND,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_CAPABILITY__PCI_AGP_COMMAND {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_CAPABILITY__PCI_AGP_STATUS {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_CONTROL {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_EXTENDED_CAPABILITY {
    pub IsochStatus: PCI_AGP_ISOCH_STATUS,
    pub AgpControl: PCI_AGP_CONTROL,
    pub ApertureSize: super::USHORT,
    pub AperturePageSize: PCI_AGP_APERTURE_PAGE_SIZE,
    pub GartLow: super::ULONG,
    pub GartHigh: super::ULONG,
    pub IsochCommand: PCI_AGP_ISOCH_COMMAND,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_ISOCH_COMMAND {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_AGP_ISOCH_STATUS {
    pub _bitfield: u32,
}
pub const PCI_AGP_RATE_1X: i32 = 1;
pub const PCI_AGP_RATE_2X: i32 = 2;
pub const PCI_AGP_RATE_4X: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PCI_BUS_INTERFACE_STANDARD {
    pub Size: super::USHORT,
    pub Version: super::USHORT,
    pub Context: super::PVOID,
    pub InterfaceReference: super::PINTERFACE_REFERENCE,
    pub InterfaceDereference: super::PINTERFACE_DEREFERENCE,
    pub ReadConfig: PPCI_READ_WRITE_CONFIG,
    pub WriteConfig: PPCI_READ_WRITE_CONFIG,
    pub PinToLine: PPCI_PIN_TO_LINE,
    pub LineToPin: PPCI_LINE_TO_PIN,
    pub RootBusCapability: PPCI_ROOT_BUS_CAPABILITY,
    pub ExpressWakeControl: PPCI_EXPRESS_WAKE_CONTROL,
    pub PrepareMultistageResume: PPCI_PREPARE_MULTISTAGE_RESUME,
}
pub const PCI_BUS_INTERFACE_STANDARD_VERSION: i32 = 2;
#[cfg(target_arch = "x86")]
#[cfg(feature = "shared")]
pub const PCI_BUS_INTERFACE_STANDARD_VERSION_1_LENGTH: super::LONG = 0x28_u32 as _;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
pub const PCI_BUS_INTERFACE_STANDARD_VERSION_1_LENGTH: super::LONG = 0x50_u32 as _;
pub type PCI_BUS_WIDTH = i32;
pub const PCI_DATA_TAG: i32 = 541672272;
pub const PCI_DATA_VERSION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_DEBUGGING_DEVICE_IN_USE {
    pub Segment: super::USHORT,
    pub Bus: super::ULONG,
    pub Slot: super::ULONG,
}
pub type PCI_DEVICE_D3COLD_STATE_REASON = i32;
pub const PCI_DOE_OBJECT_TYPE_ASYNC_MESSAGE: i32 = 5;
pub const PCI_DOE_OBJECT_TYPE_CMA_SPDM: i32 = 1;
pub const PCI_DOE_OBJECT_TYPE_CMA_SPDM_WITH_CONNECTION_ID: i32 = 3;
pub const PCI_DOE_OBJECT_TYPE_DISCOVERY: i32 = 0;
pub const PCI_DOE_OBJECT_TYPE_SECURED_CMA_SPDM: i32 = 2;
pub const PCI_DOE_OBJECT_TYPE_SECURED_CMA_SPDM_WITH_CONNECTION_ID: i32 = 4;
pub const PCI_DOE_VENDOR_ID_RESERVED: i32 = 1;
pub type PCI_ERROR_HANDLER_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PCI_EXPRESS_ASPM_CONTROL = i32;
pub type PCI_EXPRESS_ASPM_SUPPORT = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_CAPABILITIES_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CAPABILITY {
    pub Header: super::PCI_CAPABILITIES_HEADER,
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
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_EXPRESS_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PCI_EXPRESS_CARD_PRESENCE = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CXL_DVSEC_CAPABILITY {
    pub Header: super::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DvsecHeader1: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1,
    pub DvsecHeader2: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2,
    pub Reserved: [super::UCHAR; 46],
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_EXPRESS_CXL_DVSEC_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11 {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V2 {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V2_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V2_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CXL_DVSEC_CAPABILITY_V11 {
    pub Header: super::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DvsecHeader1: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1,
    pub DvsecHeader2: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2,
    pub Capability: PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11,
    pub Control: PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER,
    pub Status: PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER,
    pub Control2: super::USHORT,
    pub Status2: super::USHORT,
    pub Lock: PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER,
    pub Reserved: super::USHORT,
    pub Range1SizeHigh: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_HIGH_REGISTER,
    pub Range1SizeLow: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11,
    pub Range1BaseHigh: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_HIGH_REGISTER,
    pub Range1BaseLow: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER,
    pub Range2SizeHigh: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_HIGH_REGISTER,
    pub Range2SizeLow: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11,
    pub Range2BaseHigh: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_HIGH_REGISTER,
    pub Range2BaseLow: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_EXPRESS_CXL_DVSEC_CAPABILITY_V11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_HIGH_REGISTER {
    pub MemBaseHigh: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_HIGH_REGISTER {
    pub MemSizeHigh: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11 {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DATA_OBJECT_EXCHANGE_EXTENDED_CAPABILITY {
    pub Header: super::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DoeCapabilities: PCI_EXPRESS_DOE_CAPABILITIES_REGISTER,
    pub DoeControl: PCI_EXPRESS_DOE_CONTROL_REGISTER,
    pub DoeStatus: PCI_EXPRESS_DOE_STATUS_REGISTER,
    pub WriteMailbox: PCI_EXPRESS_DOE_WRITE_MAILBOX_REGISTER,
    pub ReadMailbox: PCI_EXPRESS_DOE_READ_MAILBOX_REGISTER,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_EXPRESS_DATA_OBJECT_EXCHANGE_EXTENDED_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY {
    pub Header: super::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DvsecHeader1: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1,
    pub DvsecHeader2: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2,
    pub DvsecRegisters: [super::USHORT; 1],
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1 {
    pub Anonymous: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2 {
    pub Anonymous: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_CONTROL_REGISTER_0,
    pub Anonymous2: PCI_EXPRESS_DEVICE_CONTROL_REGISTER_1,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DEVICE_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DEVICE_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DEVICE_CONTROL_REGISTER_1 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_STATUS_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_STATUS_2_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DEVICE_STATUS_2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DEVICE_STATUS_2_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_STATUS_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DEVICE_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DEVICE_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
pub type PCI_EXPRESS_DEVICE_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DOE_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_DOE_CAPABILITIES_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DOE_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DOE_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DOE_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_DOE_CONTROL_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DOE_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DOE_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DOE_READ_MAILBOX_REGISTER {
    pub ReadDataMailbox: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DOE_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_DOE_STATUS_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DOE_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DOE_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DOE_WRITE_MAILBOX_REGISTER {
    pub WriteDataMailbox: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_CAPABILITY {
    pub Header: super::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
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
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_EXPRESS_DPC_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_CAPS_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_CAPS_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DPC_CAPS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_CAPS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_CONTROL_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DPC_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_ERROR_SOURCE_ID {
    pub Anonymous: PCI_EXPRESS_DPC_ERROR_SOURCE_ID_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DPC_ERROR_SOURCE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_ERROR_SOURCE_ID_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_RP_PIO_HEADERLOG_REGISTER {
    pub PioHeaderLogRegister: [super::ULONG; 4],
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DPC_RP_PIO_HEADERLOG_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_IMPSPECLOG_REGISTER {
    pub PioImpSpecLog: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DPC_RP_PIO_IMPSPECLOG_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_RP_PIO_TLPPREFIXLOG_REGISTER {
    pub PioTlpPrefixLogRegister: [super::ULONG; 4],
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DPC_RP_PIO_TLPPREFIXLOG_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_STATUS_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_DPC_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_DPC_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_EVENT_COLLECTOR_ENDPOINT_ASSOCIATION_CAPABILITY {
    pub Header: super::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub AssociationBitmap: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_IDE_ADDRESS_ASSOCIATION_BLOCK {
    pub Reserved: [super::ULONG; 3],
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_IDE_ADDRESS_ASSOCIATION_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_IDE_CAPABILITY {
    pub Header: super::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capability: PCI_EXPRESS_IDE_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_IDE_CONTROL_REGISTER,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_EXPRESS_IDE_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_IDE_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_IDE_CAPABILITY_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_IDE_CAPABILITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_IDE_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_IDE_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_IDE_CONTROL_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_IDE_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_IDE_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_1 {
    pub Anonymous: PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_1_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_2 {
    pub Anonymous: PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_2_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_2_0 {
    pub _bitfield: u32,
}
pub type PCI_EXPRESS_INDICATOR_STATE = i32;
pub type PCI_EXPRESS_L0s_EXIT_LATENCY = i32;
pub type PCI_EXPRESS_L1_EXIT_LATENCY = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_L1_PM_SS_CAPABILITY {
    pub Header: super::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub L1PmSsCapabilities: PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER,
    pub L1PmSsControl1: PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER,
    pub L1PmSsControl2: PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_EXPRESS_L1_PM_SS_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER {
    pub Anonymous: PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LANE_ERROR_STATUS {
    pub LaneBitmap: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_CAPABILITIES_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_LINK_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LINK_CONTROL3 {
    pub Anonymous: PCI_EXPRESS_LINK_CONTROL3_0,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_LINK_CONTROL3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CONTROL3_0 {
    pub Anonymous: PCI_EXPRESS_LINK_CONTROL3_0_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_LINK_CONTROL3_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_CONTROL3_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CONTROL_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_CONTROL_2_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_LINK_CONTROL_2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_CONTROL_2_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_CONTROL_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_LINK_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LINK_IDE_BLOCK {
    pub Control: PCI_EXPRESS_LINK_IDE_STREAM_CONTROL_REGISTER,
    pub Status: PCI_EXPRESS_LINK_IDE_STREAM_STATUS_REGISTER,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_LINK_IDE_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_IDE_STREAM_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_IDE_STREAM_CONTROL_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_LINK_IDE_STREAM_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_IDE_STREAM_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_IDE_STREAM_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_IDE_STREAM_STATUS_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_LINK_IDE_STREAM_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_IDE_STREAM_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_STATUS_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_STATUS_2_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_LINK_STATUS_2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_STATUS_2_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_STATUS_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_LINK_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LINK_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
pub type PCI_EXPRESS_LINK_SUBSTATE = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LTR_CAPABILITY {
    pub Header: super::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Latency: PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_EXPRESS_LTR_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER {
    pub Anonymous: PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER_0 {
    pub _bitfield: u32,
}
pub type PCI_EXPRESS_MAX_PAYLOAD_SIZE = i32;
pub type PCI_EXPRESS_MRL_STATE = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_NPEM_CAPABILITY {
    pub Header: super::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capability: PCI_EXPRESS_NPEM_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_NPEM_CONTROL_REGISTER,
    pub Status: PCI_EXPRESS_NPEM_STATUS_REGISTER,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_EXPRESS_NPEM_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_NPEM_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_NPEM_CAPABILITY_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_NPEM_CAPABILITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_NPEM_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_NPEM_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_NPEM_CONTROL_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_NPEM_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_NPEM_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_NPEM_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_NPEM_STATUS_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_NPEM_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_NPEM_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PME_REQUESTOR_ID {
    pub Anonymous: PCI_EXPRESS_PME_REQUESTOR_ID_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_PME_REQUESTOR_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_PME_REQUESTOR_ID_0 {
    pub _bitfield: u16,
}
pub type PCI_EXPRESS_POWER_STATE = i32;
pub type PCI_EXPRESS_RCB = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY {
    pub Header: super::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Entry: [PCI_EXPRESS_RESIZABLE_BAR_ENTRY; 6],
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_RESIZABLE_BAR_ENTRY {
    pub Capability: PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_RESIZABLE_BAR_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ROOT_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_ROOT_CONTROL_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_ROOT_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_ROOT_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ROOT_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_ROOT_STATUS_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_ROOT_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_ROOT_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SECONDARY_CAPABILITY {
    pub Header: super::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub LinkControl3: PCI_EXPRESS_LINK_CONTROL3,
    pub LaneErrorStatus: PCI_EXPRESS_LANE_ERROR_STATUS,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_EXPRESS_SECONDARY_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SELECTIVE_IDE_BLOCK {
    pub Capability: PCI_EXPRESS_SELECTIVE_IDE_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_SELECTIVE_IDE_CONTROL_REGISTER,
    pub Status: PCI_EXPRESS_SELECTIVE_IDE_STATUS_REGISTER,
    pub RidAssociation1: PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_1,
    pub RidAssociation2: PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_2,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_SELECTIVE_IDE_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SELECTIVE_IDE_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_SELECTIVE_IDE_CAPABILITY_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_SELECTIVE_IDE_CAPABILITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_SELECTIVE_IDE_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SELECTIVE_IDE_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_SELECTIVE_IDE_CONTROL_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_SELECTIVE_IDE_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_SELECTIVE_IDE_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SELECTIVE_IDE_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_SELECTIVE_IDE_STATUS_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_SELECTIVE_IDE_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_SELECTIVE_IDE_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SLOT_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_SLOT_CONTROL_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_SLOT_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_SLOT_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SLOT_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_SLOT_STATUS_REGISTER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_SLOT_STATUS_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_SLOT_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_TPH_REQUESTER_CAPABILITY {
    pub Header: super::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub RequesterCapability: PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER,
    pub RequesterControl: PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_EXPRESS_TPH_REQUESTER_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
pub const PCI_EXPRESS_TPH_ST_LOCATION_MSIX_TABLE: i32 = 2;
pub const PCI_EXPRESS_TPH_ST_LOCATION_NONE: i32 = 0;
pub const PCI_EXPRESS_TPH_ST_LOCATION_RESERVED: i32 = 3;
pub const PCI_EXPRESS_TPH_ST_LOCATION_TPH_CAPABILITY: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_TPH_ST_TABLE_ENTRY {
    pub Anonymous: PCI_EXPRESS_TPH_ST_TABLE_ENTRY_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_TPH_ST_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_TPH_ST_TABLE_ENTRY_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY {
    pub Header: super::PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DvsecHeader1: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1,
    pub DvsecHeader2: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2,
    pub DvsecHeader3: PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_HEADER,
    pub PortSpecificAttributes: PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_HEADER {
    pub Anonymous: PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_HEADER_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_HEADER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES {
    pub Anonymous: PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0 {
    pub type0: PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0_0,
    pub type1: PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0_1,
    pub type2: PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0_2,
}
#[cfg(feature = "shared")]
impl Default for PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0_0 {
    pub _bitfield1: u16,
    pub _bitfield2: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0_1 {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES_0_2 {
    pub _bitfield: u32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCI_EXPRESS_WAKE_CONTROL = Option<unsafe extern "system" fn(context: super::PVOID, enablewake: super::BOOLEAN)>;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PCI_FIRMWARE_BUS_CAPS {
    pub Type: super::USHORT,
    pub Length: super::USHORT,
    pub Anonymous: PCI_FIRMWARE_BUS_CAPS_0,
    pub CurrentSpeedAndMode: super::UCHAR,
    pub SupportedSpeedsAndModesLowByte: super::UCHAR,
    pub SupportedSpeedsAndModesHighByte: super::UCHAR,
    pub Voltage: super::UCHAR,
    pub Reserved2: [super::UCHAR; 7],
}
#[cfg(feature = "shared")]
impl Default for PCI_FIRMWARE_BUS_CAPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_FIRMWARE_BUS_CAPS_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_FIRMWARE_BUS_CAPS_RETURN_BUFFER {
    pub Version: super::USHORT,
    pub Status: super::USHORT,
    pub Length: super::ULONG,
    pub Caps: PCI_FIRMWARE_BUS_CAPS,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_FPB_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_FPB_CAPABILITIES_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_FPB_CAPABILITIES_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
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
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for PCI_FPB_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_CAPABILITY_HEADER {
    pub Header: super::PCI_CAPABILITIES_HEADER,
    pub Reserved: super::USHORT,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER {
    pub Anonymous: PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_MEM_HIGH_VECTOR_CONTROL2_REGISTER {
    pub MemHighVectorStartUpper: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER {
    pub Anonymous: PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_FPB_RID_VECTOR_CONTROL1_REGISTER {
    pub Anonymous: PCI_FPB_RID_VECTOR_CONTROL1_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_FPB_RID_VECTOR_CONTROL1_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_RID_VECTOR_CONTROL1_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_FPB_RID_VECTOR_CONTROL2_REGISTER {
    pub Anonymous: PCI_FPB_RID_VECTOR_CONTROL2_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_FPB_RID_VECTOR_CONTROL2_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_RID_VECTOR_CONTROL2_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER {
    pub Anonymous: PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_FPB_VECTOR_ACCESS_DATA_REGISTER {
    pub VectorAccessData: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCI_GET_LINK_INFORMATION = Option<unsafe extern "system" fn(context: super::PVOID, linkinformation: PPCI_LINK_INFORMATION) -> windows_sys::core::NTSTATUS>;
pub type PCI_HARDWARE_INTERFACE = i32;
pub const PCI_INVALID_ALTERNATE_FUNCTION_NUMBER: i32 = 255;
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PCI_LINE_TO_PIN = Option<unsafe extern "system" fn(context: super::PVOID, pcinewdata: super::PPCI_COMMON_CONFIG, pciolddata: super::PPCI_COMMON_CONFIG)>;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PCI_LINK_CONFIG_INTERFACE_V1 {
    pub Size: super::USHORT,
    pub Version: super::USHORT,
    pub Context: super::PVOID,
    pub InterfaceReference: super::PINTERFACE_REFERENCE,
    pub InterfaceDereference: super::PINTERFACE_DEREFERENCE,
    pub SetMaximumLinkBandwidth: PPCI_SET_MAX_LINK_BANDWIDTH,
    pub GetLinkInformation: PPCI_GET_LINK_INFORMATION,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PCI_LINK_INFORMATION {
    pub AvailableLinkSpeeds: PCI_LINK_INFORMATION_0,
    pub LinkWidth: super::USHORT,
    pub CurrentLinkSpeed: PCI_LINK_SPEED,
}
#[cfg(feature = "shared")]
impl Default for PCI_LINK_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_LINK_INFORMATION_0 {
    pub Anonymous: PCI_LINK_INFORMATION_0_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_LINK_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_LINK_INFORMATION_0_0 {
    pub _bitfield: u32,
}
pub type PCI_LINK_SPEED = i32;
pub type PCI_LINK_WIDTH = i32;
pub const PCI_LNKINTRF_VERSION: i32 = 1;
pub type PCI_OSC_CONTROL_BITS = i32;
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PCI_PIN_TO_LINE = Option<unsafe extern "system" fn(context: super::PVOID, pcidata: super::PPCI_COMMON_CONFIG)>;
#[cfg(feature = "winnt")]
pub type PCI_PREPARE_MULTISTAGE_RESUME = Option<unsafe extern "system" fn(context: super::PVOID)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCI_READ_WRITE_CONFIG = Option<unsafe extern "system" fn(context: super::PVOID, busoffset: super::ULONG, slot: super::ULONG, buffer: super::PVOID, offset: super::ULONG, length: super::ULONG) -> super::ULONG>;
pub const PCI_RECOVERY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdd060800_f6e1_4204_ac27_c4bca9568402);
pub const PCI_RESERVED_VENDOR_ID: i32 = 1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCI_ROOT_BUS_CAPABILITY = Option<unsafe extern "system" fn(context: super::PVOID, hardwarecapability: PPCI_ROOT_BUS_HARDWARE_CAPABILITY)>;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PCI_ROOT_BUS_HARDWARE_CAPABILITY {
    pub SecondaryInterface: PCI_HARDWARE_INTERFACE,
    pub Anonymous: PCI_ROOT_BUS_HARDWARE_CAPABILITY_0,
    pub OscFeatureSupport: PCI_ROOT_BUS_OSC_SUPPORT_FIELD,
    pub OscControlRequest: PCI_ROOT_BUS_OSC_CONTROL_FIELD,
    pub OscControlGranted: PCI_ROOT_BUS_OSC_CONTROL_FIELD,
    pub CxlCapable: super::BOOLEAN,
    pub CxlVersionSupport: CXL_PROTOCOL_VERSION,
    pub CxlOscFeatureSupport: CXL_BUS_OSC_SUPPORT_FIELD,
    pub CxlOscControlRequest: CXL_BUS_OSC_CONTROL_FIELD,
    pub CxlOscControlGranted: CXL_BUS_OSC_CONTROL_FIELD,
}
#[cfg(feature = "shared")]
impl Default for PCI_ROOT_BUS_HARDWARE_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_ROOT_BUS_HARDWARE_CAPABILITY_0 {
    pub BusCapabilitiesFound: super::BOOLEAN,
    pub CurrentSpeedAndMode: super::ULONG,
    pub SupportedSpeedsAndModes: super::ULONG,
    pub DeviceIDMessagingCapable: super::BOOLEAN,
    pub SecondaryBusWidth: PCI_BUS_WIDTH,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PCI_ROOT_BUS_OSC_CONTROL_FIELD {
    pub u: PCI_ROOT_BUS_OSC_CONTROL_FIELD_0,
}
#[cfg(feature = "shared")]
impl Default for PCI_ROOT_BUS_OSC_CONTROL_FIELD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_ROOT_BUS_OSC_CONTROL_FIELD_0 {
    pub Anonymous: PCI_ROOT_BUS_OSC_CONTROL_FIELD_0_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_ROOT_BUS_OSC_CONTROL_FIELD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_ROOT_BUS_OSC_CONTROL_FIELD_0_0 {
    pub _bitfield: u32,
}
pub const PCI_ROOT_BUS_OSC_METHOD_CAPABILITY_REVISION: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PCI_ROOT_BUS_OSC_SUPPORT_FIELD {
    pub u: PCI_ROOT_BUS_OSC_SUPPORT_FIELD_0,
}
#[cfg(feature = "shared")]
impl Default for PCI_ROOT_BUS_OSC_SUPPORT_FIELD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PCI_ROOT_BUS_OSC_SUPPORT_FIELD_0 {
    pub Anonymous: PCI_ROOT_BUS_OSC_SUPPORT_FIELD_0_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PCI_ROOT_BUS_OSC_SUPPORT_FIELD_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PCI_ROOT_BUS_OSC_SUPPORT_FIELD_0_0 {
    pub _bitfield: u32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCI_SET_MAX_LINK_BANDWIDTH = Option<unsafe extern "system" fn(context: super::PVOID, targetbandwidth: super::ULONG, waitforretrain: super::BOOLEAN, linkthrottled: super::PBOOLEAN, newlinkspeed: PPCI_LINK_SPEED) -> windows_sys::core::NTSTATUS>;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct PCI_SUBSYSTEM_IDS_CAPABILITY {
    pub Header: super::PCI_CAPABILITIES_HEADER,
    pub Reserved: super::USHORT,
    pub SubVendorID: super::USHORT,
    pub SubSystemID: super::USHORT,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy, Default)]
pub struct PCI_VENDOR_SPECIFIC_CAPABILITY {
    pub Header: super::PCI_CAPABILITIES_HEADER,
    pub VscLength: super::UCHAR,
    pub VendorSpecific: super::UCHAR,
}
pub const PCIe_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcf93c01f_1a16_4dfc_b8bc_9c4daf67c104);
pub const PCMCIAConfiguration: BUS_DATA_TYPE = 7;
#[cfg(all(feature = "basetsd", feature = "mce", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PCMC_DRIVER_INFO = *mut CMC_DRIVER_INFO;
#[cfg(feature = "shared")]
pub type PCM_PCCARD_DEVICE_DATA = *mut CM_PCCARD_DEVICE_DATA;
#[cfg(feature = "shared")]
pub type PCONFIGURATION_INFORMATION = *mut CONFIGURATION_INFORMATION;
pub type PCONFIGURATION_TYPE = *mut CONFIGURATION_TYPE;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PCONTROLLER_OBJECT = *mut CONTROLLER_OBJECT;
#[cfg(all(feature = "basetsd", feature = "mce", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PCPE_DRIVER_INFO = *mut CPE_DRIVER_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCREATE_PROCESS_NOTIFY_ROUTINE = Option<unsafe extern "system" fn(parentid: super::HANDLE, processid: super::HANDLE, create: super::BOOLEAN)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCREATE_THREAD_NOTIFY_ROUTINE = Option<unsafe extern "system" fn(processid: super::HANDLE, threadid: super::HANDLE, create: super::BOOLEAN)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCREATE_USER_PROCESS_ECP_CONTEXT = *mut CREATE_USER_PROCESS_ECP_CONTEXT;
#[cfg(target_arch = "aarch64")]
pub const PCR_BTI_MITIGATION_CSWAP_HVC: i32 = 16;
#[cfg(target_arch = "aarch64")]
pub const PCR_BTI_MITIGATION_CSWAP_SMC: i32 = 32;
#[cfg(target_arch = "aarch64")]
pub const PCR_BTI_MITIGATION_TRAP_HVC: i32 = 2;
#[cfg(target_arch = "aarch64")]
pub const PCR_BTI_MITIGATION_VBAR_MASK: i32 = 14;
#[cfg(target_arch = "aarch64")]
pub type PCR_BTI_VBAR_INDEX = i32;
#[cfg(target_arch = "aarch64")]
pub const PCR_KVA_MITIGATION_VBAR_MASK: i32 = 1;
pub const PCR_MAJOR_VERSION: i32 = 1;
pub const PCR_MINOR_VERSION: i32 = 1;
#[cfg(target_arch = "aarch64")]
pub const PCR_MITIGATION_VBAR_MASK: i32 = 15;
#[cfg(feature = "shared")]
pub type PCXL_BUS_OSC_CONTROL_FIELD = *mut CXL_BUS_OSC_CONTROL_FIELD;
#[cfg(feature = "shared")]
pub type PCXL_BUS_OSC_SUPPORT_FIELD = *mut CXL_BUS_OSC_SUPPORT_FIELD;
pub type PCXL_DVSEC_IDS = *mut CXL_DVSEC_IDS;
pub type PCXL_OSC_CONTROL_BITS = *mut CXL_OSC_CONTROL_BITS;
pub type PCXL_PROTOCOL_VERSION = *mut CXL_PROTOCOL_VERSION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDEBUGGING_DEVICE_IN_USE = *mut DEBUGGING_DEVICE_IN_USE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDEBUGGING_DEVICE_IN_USE_INFORMATION = *mut DEBUGGING_DEVICE_IN_USE_INFORMATION;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PDEBUG_DEVICE_ADDRESS = *mut DEBUG_DEVICE_ADDRESS;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PDEBUG_DEVICE_DESCRIPTOR = *mut DEBUG_DEVICE_DESCRIPTOR;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PDEBUG_DEVICE_FOUND_FUNCTION = Option<unsafe extern "system" fn(device: PDEBUG_DEVICE_DESCRIPTOR) -> KD_CALLBACK_ACTION>;
#[cfg(feature = "winnt")]
pub type PDEBUG_EFI_IOMMU_DATA = *mut DEBUG_EFI_IOMMU_DATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDEBUG_MEMORY_REQUIREMENTS = *mut DEBUG_MEMORY_REQUIREMENTS;
#[cfg(feature = "shared")]
pub type PDEBUG_TRANSPORT_DATA = *mut DEBUG_TRANSPORT_DATA;
#[cfg(target_arch = "x86")]
pub const PDE_PER_PAGE: i32 = 512;
#[cfg(feature = "shared")]
pub type PDIMM_ADDRESS = *mut DIMM_ADDRESS;
#[cfg(feature = "shared")]
pub type PDIMM_ADDR_VALID_BITS = *mut DIMM_ADDR_VALID_BITS;
#[cfg(feature = "shared")]
pub type PDIMM_ADDR_VALID_BITS_DDR4 = *mut DIMM_ADDR_VALID_BITS_DDR4;
#[cfg(feature = "shared")]
pub type PDIMM_ADDR_VALID_BITS_DDR5 = *mut DIMM_ADDR_VALID_BITS_DDR5;
#[cfg(feature = "shared")]
pub type PDIMM_INFO = *mut DIMM_INFO;
#[cfg(feature = "shared")]
pub type PDISK_SIGNATURE = *mut DISK_SIGNATURE;
#[cfg(target_arch = "x86")]
pub const PDI_SHIFT: i32 = 21;
#[cfg(feature = "shared")]
pub type PDOE_DISCOVERY_REQUEST = *mut DOE_DISCOVERY_REQUEST;
#[cfg(feature = "shared")]
pub type PDOE_DISCOVERY_RESPONSE = *mut DOE_DISCOVERY_RESPONSE;
#[cfg(feature = "shared")]
pub type PDOE_HEADER_1 = *mut DOE_HEADER_1;
#[cfg(feature = "shared")]
pub type PDOE_HEADER_2 = *mut DOE_HEADER_2;
#[cfg(feature = "shared")]
pub type PDOE_OBJECT_HEADER = *mut DOE_OBJECT_HEADER;
#[cfg(all(feature = "mce", feature = "shared", feature = "winnt"))]
pub type PDRIVER_CMC_EXCEPTION_CALLBACK = Option<unsafe extern "system" fn(context: super::PVOID, cmclog: super::PCMC_EXCEPTION)>;
#[cfg(all(feature = "mce", feature = "shared", feature = "winnt"))]
pub type PDRIVER_CPE_EXCEPTION_CALLBACK = Option<unsafe extern "system" fn(context: super::PVOID, cmclog: super::PCPE_EXCEPTION)>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "mce", feature = "shared", feature = "winnt"))]
pub type PDRIVER_EXCPTN_CALLBACK = Option<unsafe extern "system" fn(context: super::PVOID, banklog: super::PMCA_EXCEPTION)>;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "mce", feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PDRIVER_EXCPTN_CALLBACK = Option<unsafe extern "C" fn(context: super::PVOID, trapframe: *const KTRAP_FRAME, exceptionframe: *const KEXCEPTION_FRAME, exception: super::PMCA_EXCEPTION) -> super::ERROR_SEVERITY>;
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PDRIVER_EXCPTN_CALLBACK = Option<unsafe extern "C" fn(context: super::PVOID, trapframe: *const KTRAP_FRAME, exceptionframe: *const KEXCEPTION_FRAME, exception: super::PVOID)>;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "mce", feature = "shared", feature = "winnt"))]
pub type PDRIVER_MCA_EXCEPTION_CALLBACK = Option<unsafe extern "system" fn(context: super::PVOID, banklog: super::PMCA_EXCEPTION)>;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "mce", feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PDRIVER_MCA_EXCEPTION_CALLBACK = Option<unsafe extern "C" fn(context: super::PVOID, trapframe: *const KTRAP_FRAME, exceptionframe: *const KEXCEPTION_FRAME, exception: super::PMCA_EXCEPTION) -> super::ERROR_SEVERITY>;
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PDRIVER_MCA_EXCEPTION_CALLBACK = Option<unsafe extern "C" fn(context: super::PVOID, trapframe: *const KTRAP_FRAME, exceptionframe: *const KEXCEPTION_FRAME, exception: super::PVOID)>;
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PDRIVER_REINITIALIZE = Option<unsafe extern "system" fn(driverobject: *const super::DRIVER_OBJECT, context: super::PVOID, count: super::ULONG)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDRIVER_VERIFIER_THUNK_PAIRS = *mut DRIVER_VERIFIER_THUNK_PAIRS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PDRIVER_VERIFIER_THUNK_ROUTINE = Option<unsafe extern "system" fn(context: super::PVOID) -> super::ULONG_PTR>;
#[cfg(feature = "shared")]
pub type PEFI_ACPI_RAS_SIGNAL_TABLE = *mut EFI_ACPI_RAS_SIGNAL_TABLE;
pub const PEI_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x09a9d5ac_5204_4214_96e5_94992e752bcd);
pub type PEJOB = *mut _EJOB;
#[cfg(feature = "shared")]
pub type PERROR_SOURCE_INFO = *mut ERROR_SOURCE_INFO;
pub type PESILO = *mut _EJOB;
#[cfg(feature = "winnt")]
pub type PEXPAND_STACK_CALLOUT = Option<unsafe extern "system" fn(parameter: super::PVOID)>;
pub type PEXTENDED_AGP_REGISTER = *mut EXTENDED_AGP_REGISTER;
#[cfg(feature = "shared")]
pub type PFILE_ALIGNMENT_INFORMATION = *mut FILE_ALIGNMENT_INFORMATION;
#[cfg(feature = "shared")]
pub type PFILE_ATTRIBUTE_TAG_INFORMATION = *mut FILE_ATTRIBUTE_TAG_INFORMATION;
#[cfg(feature = "shared")]
pub type PFILE_DISPOSITION_INFORMATION = *mut FILE_DISPOSITION_INFORMATION;
#[cfg(feature = "shared")]
pub type PFILE_DISPOSITION_INFORMATION_EX = *mut FILE_DISPOSITION_INFORMATION_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_END_OF_FILE_INFORMATION = *mut FILE_END_OF_FILE_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_FS_FULL_SIZE_INFORMATION = *mut FILE_FS_FULL_SIZE_INFORMATION;
#[cfg(feature = "shared")]
pub type PFILE_FS_FULL_SIZE_INFORMATION_EX = *mut FILE_FS_FULL_SIZE_INFORMATION_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_FS_LABEL_INFORMATION = *mut FILE_FS_LABEL_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_FS_METADATA_SIZE_INFORMATION = *mut FILE_FS_METADATA_SIZE_INFORMATION;
#[cfg(feature = "shared")]
pub type PFILE_FS_OBJECTID_INFORMATION = *mut FILE_FS_OBJECTID_INFORMATION;
#[cfg(feature = "shared")]
pub type PFILE_FS_SECTOR_SIZE_INFORMATION = *mut FILE_FS_SECTOR_SIZE_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_FS_SIZE_INFORMATION = *mut FILE_FS_SIZE_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_FS_VOLUME_INFORMATION = *mut FILE_FS_VOLUME_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_NAME_INFORMATION = *mut FILE_NAME_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_VALID_DATA_LENGTH_INFORMATION = *mut FILE_VALID_DATA_LENGTH_INFORMATION;
#[cfg(feature = "shared")]
pub type PFNFTH = Option<unsafe extern "C" fn(systemfirmwaretableinfo: PSYSTEM_FIRMWARE_TABLE_INFORMATION) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PFN_IN_USE_PAGE_OFFLINE_NOTIFY = Option<unsafe extern "system" fn(page: super::PFN_NUMBER, poisoned: super::BOOLEAN, context: super::PVOID, callbackstatus: super::PNTSTATUS) -> super::BOOLEAN>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFN_WHEA_HIGH_IRQL_LOG_SEL_EVENT_HANDLER = Option<unsafe extern "system" fn(context: super::PVOID, osselrecord: PIPMI_OS_SEL_RECORD) -> windows_sys::core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type PGET_LOCATION_STRING = Option<unsafe extern "system" fn(context: super::PVOID, locationstrings: *mut super::PZZWSTR) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PHALIOREADWRITEHANDLER = Option<unsafe extern "system" fn(fread: super::BOOLEAN, dwaddr: super::ULONG, dwsize: super::ULONG, pdwdata: super::PULONG) -> windows_sys::core::NTSTATUS>;
pub type PHALMCAINTERFACELOCK = Option<unsafe extern "system" fn()>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHALMCAINTERFACEREADREGISTER = Option<unsafe extern "system" fn(banknumber: super::UCHAR, exception: super::PVOID) -> windows_sys::core::NTSTATUS>;
pub type PHALMCAINTERFACEUNLOCK = Option<unsafe extern "system" fn()>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PHAL_AMLI_BAD_IO_ADDRESS_LIST = *mut HAL_AMLI_BAD_IO_ADDRESS_LIST;
pub type PHAL_APIC_DESTINATION_MODE = *mut HAL_APIC_DESTINATION_MODE;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PHAL_BUS_INFORMATION = *mut HAL_BUS_INFORMATION;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PHAL_CALLBACKS = *mut HAL_CALLBACKS;
pub type PHAL_DISPLAY_BIOS_INFORMATION = *mut HAL_DISPLAY_BIOS_INFORMATION;
pub type PHAL_DMA_CRASH_DUMP_REGISTER_TYPE = *mut HAL_DMA_CRASH_DUMP_REGISTER_TYPE;
#[cfg(feature = "shared")]
pub type PHAL_ERROR_INFO = *mut HAL_ERROR_INFO;
#[cfg(feature = "shared")]
pub type PHAL_PLATFORM_INFORMATION = *mut HAL_PLATFORM_INFORMATION;
#[cfg(feature = "shared")]
pub type PHAL_PMU_NOTIFICATION = *mut HAL_PMU_NOTIFICATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHAL_PMU_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(context: super::PVOID, notification: PHAL_PMU_NOTIFICATION)>;
#[cfg(feature = "shared")]
pub type PHAL_POST_SLEEP_STATS = *mut HAL_POST_SLEEP_STATS;
#[cfg(feature = "shared")]
pub type PHAL_POWER_INFORMATION = *mut HAL_POWER_INFORMATION;
#[cfg(feature = "shared")]
pub type PHAL_PROCESSOR_SPEED_INFORMATION = *mut HAL_PROCESSOR_SPEED_INFORMATION;
pub type PHAL_QUERY_INFORMATION_CLASS = *mut HAL_QUERY_INFORMATION_CLASS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHAL_REGISTER_PMU_NOTIFICATION_INPUT = *mut HAL_REGISTER_PMU_NOTIFICATION_INPUT;
#[cfg(feature = "shared")]
pub type PHAL_RESET_DISPLAY_PARAMETERS = Option<unsafe extern "system" fn(columns: super::ULONG, rows: super::ULONG) -> super::BOOLEAN>;
pub type PHAL_SET_INFORMATION_CLASS = *mut HAL_SET_INFORMATION_CLASS;
#[cfg(feature = "shared")]
pub type PHAL_UNREGISTER_PMU_NOTIFICATION_INPUT = *mut HAL_UNREGISTER_PMU_NOTIFICATION_INPUT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PHARDWARE_COUNTER = *mut HARDWARE_COUNTER;
#[cfg(feature = "shared")]
pub type PHVL_WHEA_ERROR_NOTIFICATION = Option<unsafe extern "system" fn(recoverycontext: PWHEA_RECOVERY_CONTEXT, poisoned: super::BOOLEAN) -> windows_sys::core::NTSTATUS>;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PHYSICAL_COUNTER_EVENT_BUFFER_CONFIGURATION {
    pub OverflowHandler: PPHYSICAL_COUNTER_EVENT_BUFFER_OVERFLOW_HANDLER,
    pub CustomEventBufferEntrySize: super::ULONG,
    pub EventThreshold: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR {
    pub Type: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE,
    pub Flags: super::ULONG,
    pub u: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_0 {
    pub CounterIndex: super::ULONG,
    pub Range: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_0_0,
    pub OverflowHandler: PPHYSICAL_COUNTER_OVERFLOW_HANDLER,
    pub EventBufferConfiguration: PHYSICAL_COUNTER_EVENT_BUFFER_CONFIGURATION,
    pub IdentificationTag: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_0_0 {
    pub Begin: super::ULONG,
    pub End: super::ULONG,
}
pub type PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PHYSICAL_COUNTER_RESOURCE_LIST {
    pub Count: super::ULONG,
    pub Descriptors: [PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PHYSICAL_COUNTER_RESOURCE_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PHYSICAL_MEMORY_RANGE {
    pub BaseAddress: super::PHYSICAL_ADDRESS,
    pub NumberOfBytes: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PHYSICAL_MEMORY_RANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type PIO_DRIVER_CREATE_CONTEXT = *mut IO_DRIVER_CREATE_CONTEXT;
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PIO_FOEXT_SHADOW_FILE = *mut IO_FOEXT_SHADOW_FILE;
#[cfg(feature = "shared")]
pub type PIO_FOEXT_SILO_PARAMETERS = *mut IO_FOEXT_SILO_PARAMETERS;
pub type PIO_QUERY_DEVICE_DATA_FORMAT = *mut IO_QUERY_DEVICE_DATA_FORMAT;
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PIO_QUERY_DEVICE_ROUTINE = Option<unsafe extern "system" fn(context: super::PVOID, pathname: super::PUNICODE_STRING, bustype: super::INTERFACE_TYPE, busnumber: super::ULONG, businformation: *const super::PKEY_VALUE_FULL_INFORMATION, controllertype: CONFIGURATION_TYPE, controllernumber: super::ULONG, controllerinformation: *const super::PKEY_VALUE_FULL_INFORMATION, peripheraltype: CONFIGURATION_TYPE, peripheralnumber: super::ULONG, peripheralinformation: *const super::PKEY_VALUE_FULL_INFORMATION) -> windows_sys::core::NTSTATUS>;
#[cfg(feature = "shared")]
pub type PIPMI_OS_SEL_RECORD = *mut IPMI_OS_SEL_RECORD;
pub type PIPMI_OS_SEL_RECORD_TYPE = *mut IPMI_OS_SEL_RECORD_TYPE;
#[cfg(target_arch = "aarch64")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PKARM64_VFP_STATE = *mut KARM64_VFP_STATE;
pub type PKD_CALLBACK_ACTION = *mut KD_CALLBACK_ACTION;
pub type PKD_NAMESPACE_ENUM = *mut KD_NAMESPACE_ENUM;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PKERNEL_USER_TIMES = *mut KERNEL_USER_TIMES;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PKEXCEPTION_FRAME = *mut KEXCEPTION_FRAME;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PKEY_CACHED_INFORMATION = *mut KEY_CACHED_INFORMATION;
#[cfg(feature = "shared")]
pub type PKEY_LAYER_INFORMATION = *mut KEY_LAYER_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PKEY_NAME_INFORMATION = *mut KEY_NAME_INFORMATION;
#[cfg(feature = "shared")]
pub type PKEY_VIRTUALIZATION_INFORMATION = *mut KEY_VIRTUALIZATION_INFORMATION;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PKFRED_TRAP_FRAME = *mut KFRED_TRAP_FRAME;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PKTRAP_FRAME = *mut KTRAP_FRAME;
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PKUMS_CONTEXT_HEADER = *mut KUMS_CONTEXT_HEADER;
pub const PLATFORM_EXTENDED_RAS_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc6749ac0_16fb_4868_b976_65a61299834f);
pub type PLOADER_PARAMETER_BLOCK = *mut _LOADER_PARAMETER_BLOCK;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMAP_REGISTER_ENTRY = *mut MAP_REGISTER_ENTRY;
#[cfg(all(feature = "basetsd", feature = "mce", feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PMCA_DRIVER_INFO = *mut MCA_DRIVER_INFO;
#[cfg(feature = "shared")]
pub type PMCG_CAP = *mut MCG_CAP;
#[cfg(feature = "shared")]
pub type PMCG_STATUS = *mut MCG_STATUS;
#[cfg(feature = "shared")]
pub type PMCI_STATUS = *mut MCI_STATUS;
#[cfg(feature = "shared")]
pub type PMCI_STATUS_AMD_BITS = *mut MCI_STATUS_AMD_BITS;
#[cfg(feature = "shared")]
pub type PMCI_STATUS_BITS_COMMON = *mut MCI_STATUS_BITS_COMMON;
#[cfg(feature = "shared")]
pub type PMCI_STATUS_INTEL_BITS = *mut MCI_STATUS_INTEL_BITS;
#[cfg(feature = "shared")]
pub type PMEMORY_DEFECT = *mut MEMORY_DEFECT;
#[cfg(feature = "shared")]
pub type PMEMORY_DEFECT_FLAGS = *mut MEMORY_DEFECT_FLAGS;
pub const PMEM_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x81687003_dbfd_4728_9ffd_f0904f97597d);
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PMMCOPY_ADDRESS = *mut MM_COPY_ADDRESS;
#[cfg(all(feature = "usb", feature = "winnt"))]
pub type PMM_ROTATE_COPY_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(destinationmdl: super::PMDL, sourcemdl: super::PMDL, context: super::PVOID) -> windows_sys::core::NTSTATUS>;
pub type PMM_ROTATE_DIRECTION = *mut MM_ROTATE_DIRECTION;
#[cfg(feature = "shared")]
pub type PMU_TELEMETRY_SECTION = *mut MU_TELEMETRY_SECTION;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PM_DISPATCH_TABLE {
    pub Signature: super::ULONG,
    pub Version: super::ULONG,
    pub Function: [super::PVOID; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PM_DISPATCH_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PNPISAConfiguration: BUS_DATA_TYPE = 10;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PNP_LOCATION_INTERFACE {
    pub Size: super::USHORT,
    pub Version: super::USHORT,
    pub Context: super::PVOID,
    pub InterfaceReference: super::PINTERFACE_REFERENCE,
    pub InterfaceDereference: super::PINTERFACE_DEREFERENCE,
    pub GetLocationString: PGET_LOCATION_STRING,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct POOLED_USAGE_AND_LIMITS {
    pub PeakPagedPoolUsage: super::SIZE_T,
    pub PagedPoolUsage: super::SIZE_T,
    pub PagedPoolLimit: super::SIZE_T,
    pub PeakNonPagedPoolUsage: super::SIZE_T,
    pub NonPagedPoolUsage: super::SIZE_T,
    pub NonPagedPoolLimit: super::SIZE_T,
    pub PeakPagefileUsage: super::SIZE_T,
    pub PagefileUsage: super::SIZE_T,
    pub PagefileLimit: super::SIZE_T,
}
#[cfg(feature = "shared")]
pub type POPLOCK_KEY_CONTEXT = *mut OPLOCK_KEY_CONTEXT;
pub const POWER_THROTTLING_PROCESS_CURRENT_VERSION: i32 = 1;
pub const POWER_THROTTLING_PROCESS_DELAYTIMERS: i32 = 2;
pub const POWER_THROTTLING_PROCESS_EXECUTION_SPEED: i32 = 1;
pub const POWER_THROTTLING_PROCESS_IGNORE_TIMER_RESOLUTION: i32 = 4;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct POWER_THROTTLING_PROCESS_STATE {
    pub Version: super::ULONG,
    pub ControlMask: super::ULONG,
    pub StateMask: super::ULONG,
}
pub const POWER_THROTTLING_PROCESS_VALID_FLAGS: i32 = 7;
pub const POWER_THROTTLING_THREAD_CURRENT_VERSION: i32 = 1;
pub const POWER_THROTTLING_THREAD_EXECUTION_SPEED: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct POWER_THROTTLING_THREAD_STATE {
    pub Version: super::ULONG,
    pub ControlMask: super::ULONG,
    pub StateMask: super::ULONG,
}
pub const POWER_THROTTLING_THREAD_VALID_FLAGS: i32 = 1;
pub type PPAGE_OFFLINE_ERROR_TYPES = *mut PAGE_OFFLINE_ERROR_TYPES;
#[cfg(feature = "shared")]
pub type PPAGE_OFFLINE_VALID_BITS = *mut PAGE_OFFLINE_VALID_BITS;
#[cfg(feature = "shared")]
pub type PPAGE_PRIORITY_INFORMATION = *mut PAGE_PRIORITY_INFORMATION;
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PPCIBUSDATA = *mut PCIBUSDATA;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCIX_BRIDGE_CAPABILITY = *mut PCIX_BRIDGE_CAPABILITY;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_ADVANCED_FEATURES_CAPABILITY = *mut PCI_ADVANCED_FEATURES_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_AGP_APERTURE_PAGE_SIZE = *mut PCI_AGP_APERTURE_PAGE_SIZE;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_AGP_CAPABILITY = *mut PCI_AGP_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_AGP_CONTROL = *mut PCI_AGP_CONTROL;
#[cfg(feature = "shared")]
pub type PPCI_AGP_EXTENDED_CAPABILITY = *mut PCI_AGP_EXTENDED_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_AGP_ISOCH_COMMAND = *mut PCI_AGP_ISOCH_COMMAND;
#[cfg(feature = "shared")]
pub type PPCI_AGP_ISOCH_STATUS = *mut PCI_AGP_ISOCH_STATUS;
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PPCI_BUS_INTERFACE_STANDARD = *mut PCI_BUS_INTERFACE_STANDARD;
#[cfg(feature = "shared")]
pub type PPCI_DEBUGGING_DEVICE_IN_USE = *mut PCI_DEBUGGING_DEVICE_IN_USE;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_CAPABILITIES_REGISTER = *mut PCI_EXPRESS_CAPABILITIES_REGISTER;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_EXPRESS_CAPABILITY = *mut PCI_EXPRESS_CAPABILITY;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_EXPRESS_CXL_DVSEC_CAPABILITY = *mut PCI_EXPRESS_CXL_DVSEC_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11 = *mut PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V2 = *mut PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V2;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_EXPRESS_CXL_DVSEC_CAPABILITY_V11 = *mut PCI_EXPRESS_CXL_DVSEC_CAPABILITY_V11;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER = *mut PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER = *mut PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_CXL_DVSEC_RANGE_BASE_HIGH_REGISTER = *mut PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_HIGH_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER = *mut PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_HIGH_REGISTER = *mut PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_HIGH_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11 = *mut PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER = *mut PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_EXPRESS_DATA_OBJECT_EXCHANGE_EXTENDED_CAPABILITY = *mut PCI_EXPRESS_DATA_OBJECT_EXCHANGE_EXTENDED_CAPABILITY;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY = *mut PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1 = *mut PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2 = *mut PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER = *mut PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER = *mut PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DEVICE_CONTROL_2_REGISTER = *mut PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DEVICE_CONTROL_REGISTER = *mut PCI_EXPRESS_DEVICE_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DEVICE_STATUS_2_REGISTER = *mut PCI_EXPRESS_DEVICE_STATUS_2_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DEVICE_STATUS_REGISTER = *mut PCI_EXPRESS_DEVICE_STATUS_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DOE_CAPABILITIES_REGISTER = *mut PCI_EXPRESS_DOE_CAPABILITIES_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DOE_CONTROL_REGISTER = *mut PCI_EXPRESS_DOE_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DOE_READ_MAILBOX_REGISTER = *mut PCI_EXPRESS_DOE_READ_MAILBOX_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DOE_STATUS_REGISTER = *mut PCI_EXPRESS_DOE_STATUS_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DOE_WRITE_MAILBOX_REGISTER = *mut PCI_EXPRESS_DOE_WRITE_MAILBOX_REGISTER;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_EXPRESS_DPC_CAPABILITY = *mut PCI_EXPRESS_DPC_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DPC_CAPS_REGISTER = *mut PCI_EXPRESS_DPC_CAPS_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DPC_CONTROL_REGISTER = *mut PCI_EXPRESS_DPC_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DPC_ERROR_SOURCE_ID = *mut PCI_EXPRESS_DPC_ERROR_SOURCE_ID;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DPC_RP_PIO_HEADERLOG_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_HEADERLOG_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DPC_RP_PIO_IMPSPECLOG_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_IMPSPECLOG_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DPC_RP_PIO_TLPPREFIXLOG_REGISTER = *mut PCI_EXPRESS_DPC_RP_PIO_TLPPREFIXLOG_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_DPC_STATUS_REGISTER = *mut PCI_EXPRESS_DPC_STATUS_REGISTER;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_EXPRESS_EVENT_COLLECTOR_ENDPOINT_ASSOCIATION_CAPABILITY = *mut PCI_EXPRESS_EVENT_COLLECTOR_ENDPOINT_ASSOCIATION_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_IDE_ADDRESS_ASSOCIATION_BLOCK = *mut PCI_EXPRESS_IDE_ADDRESS_ASSOCIATION_BLOCK;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_EXPRESS_IDE_CAPABILITY = *mut PCI_EXPRESS_IDE_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_IDE_CAPABILITY_REGISTER = *mut PCI_EXPRESS_IDE_CAPABILITY_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_IDE_CONTROL_REGISTER = *mut PCI_EXPRESS_IDE_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_1 = *mut PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_1;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_2 = *mut PCI_EXPRESS_IDE_RID_ASSOCIATION_REGISTER_2;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER = *mut PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_EXPRESS_L1_PM_SS_CAPABILITY = *mut PCI_EXPRESS_L1_PM_SS_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER = *mut PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER = *mut PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_LANE_ERROR_STATUS = *mut PCI_EXPRESS_LANE_ERROR_STATUS;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER = *mut PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_LINK_CAPABILITIES_REGISTER = *mut PCI_EXPRESS_LINK_CAPABILITIES_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_LINK_CONTROL3 = *mut PCI_EXPRESS_LINK_CONTROL3;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_LINK_CONTROL_2_REGISTER = *mut PCI_EXPRESS_LINK_CONTROL_2_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_LINK_CONTROL_REGISTER = *mut PCI_EXPRESS_LINK_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_LINK_IDE_BLOCK = *mut PCI_EXPRESS_LINK_IDE_BLOCK;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_LINK_IDE_STREAM_CONTROL_REGISTER = *mut PCI_EXPRESS_LINK_IDE_STREAM_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_LINK_IDE_STREAM_STATUS_REGISTER = *mut PCI_EXPRESS_LINK_IDE_STREAM_STATUS_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_LINK_STATUS_2_REGISTER = *mut PCI_EXPRESS_LINK_STATUS_2_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_LINK_STATUS_REGISTER = *mut PCI_EXPRESS_LINK_STATUS_REGISTER;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_EXPRESS_LTR_CAPABILITY = *mut PCI_EXPRESS_LTR_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_LTR_MAX_LATENCY_REGISTER = *mut PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_EXPRESS_NPEM_CAPABILITY = *mut PCI_EXPRESS_NPEM_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_NPEM_CAPABILITY_REGISTER = *mut PCI_EXPRESS_NPEM_CAPABILITY_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_NPEM_CONTROL_REGISTER = *mut PCI_EXPRESS_NPEM_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_NPEM_STATUS_REGISTER = *mut PCI_EXPRESS_NPEM_STATUS_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_PME_REQUESTOR_ID = *mut PCI_EXPRESS_PME_REQUESTOR_ID;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_EXPRESS_RESIZABLE_BAR_CAPABILITY = *mut PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER = *mut PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER = *mut PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_RESIZABLE_BAR_ENTRY = *mut PCI_EXPRESS_RESIZABLE_BAR_ENTRY;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_ROOT_CAPABILITIES_REGISTER = *mut PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_ROOT_CONTROL_REGISTER = *mut PCI_EXPRESS_ROOT_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_ROOT_STATUS_REGISTER = *mut PCI_EXPRESS_ROOT_STATUS_REGISTER;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_EXPRESS_SECONDARY_CAPABILITY = *mut PCI_EXPRESS_SECONDARY_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_SELECTIVE_IDE_BLOCK = *mut PCI_EXPRESS_SELECTIVE_IDE_BLOCK;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_SELECTIVE_IDE_CAPABILITY_REGISTER = *mut PCI_EXPRESS_SELECTIVE_IDE_CAPABILITY_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_SELECTIVE_IDE_CONTROL_REGISTER = *mut PCI_EXPRESS_SELECTIVE_IDE_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_SELECTIVE_IDE_STATUS_REGISTER = *mut PCI_EXPRESS_SELECTIVE_IDE_STATUS_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_SLOT_CAPABILITIES_REGISTER = *mut PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_SLOT_CONTROL_REGISTER = *mut PCI_EXPRESS_SLOT_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_SLOT_STATUS_REGISTER = *mut PCI_EXPRESS_SLOT_STATUS_REGISTER;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_EXPRESS_TPH_REQUESTER_CAPABILITY = *mut PCI_EXPRESS_TPH_REQUESTER_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER = *mut PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER = *mut PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_TPH_ST_TABLE_ENTRY = *mut PCI_EXPRESS_TPH_ST_TABLE_ENTRY;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_HEADER = *mut PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_HEADER;
#[cfg(feature = "shared")]
pub type PPCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES = *mut PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_PORT_ATTRIBUTES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPCI_EXPRESS_WAKE_CONTROL = Option<unsafe extern "system" fn(context: super::PVOID, enablewake: super::BOOLEAN)>;
#[cfg(feature = "shared")]
pub type PPCI_FIRMWARE_BUS_CAPS = *mut PCI_FIRMWARE_BUS_CAPS;
#[cfg(feature = "shared")]
pub type PPCI_FIRMWARE_BUS_CAPS_RETURN_BUFFER = *mut PCI_FIRMWARE_BUS_CAPS_RETURN_BUFFER;
#[cfg(feature = "shared")]
pub type PPCI_FPB_CAPABILITIES_REGISTER = *mut PCI_FPB_CAPABILITIES_REGISTER;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_FPB_CAPABILITY = *mut PCI_FPB_CAPABILITY;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_FPB_CAPABILITY_HEADER = *mut PCI_FPB_CAPABILITY_HEADER;
#[cfg(feature = "shared")]
pub type PPCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER = *mut PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_FPB_MEM_HIGH_VECTOR_CONTROL2_REGISTER = *mut PCI_FPB_MEM_HIGH_VECTOR_CONTROL2_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER = *mut PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_FPB_RID_VECTOR_CONTROL1_REGISTER = *mut PCI_FPB_RID_VECTOR_CONTROL1_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_FPB_RID_VECTOR_CONTROL2_REGISTER = *mut PCI_FPB_RID_VECTOR_CONTROL2_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER = *mut PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER;
#[cfg(feature = "shared")]
pub type PPCI_FPB_VECTOR_ACCESS_DATA_REGISTER = *mut PCI_FPB_VECTOR_ACCESS_DATA_REGISTER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPCI_GET_LINK_INFORMATION = Option<unsafe extern "system" fn(context: super::PVOID, linkinformation: PPCI_LINK_INFORMATION) -> windows_sys::core::NTSTATUS>;
pub type PPCI_HARDWARE_INTERFACE = *mut PCI_HARDWARE_INTERFACE;
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PPCI_LINE_TO_PIN = Option<unsafe extern "system" fn(context: super::PVOID, pcinewdata: super::PPCI_COMMON_CONFIG, pciolddata: super::PPCI_COMMON_CONFIG)>;
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PPCI_LINK_CONFIG_INTERFACE_V1 = *mut PCI_LINK_CONFIG_INTERFACE_V1;
#[cfg(feature = "shared")]
pub type PPCI_LINK_INFORMATION = *mut PCI_LINK_INFORMATION;
pub type PPCI_LINK_SPEED = *mut PCI_LINK_SPEED;
pub type PPCI_LINK_WIDTH = *mut PCI_LINK_WIDTH;
pub type PPCI_OSC_CONTROL_BITS = *mut PCI_OSC_CONTROL_BITS;
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PPCI_PIN_TO_LINE = Option<unsafe extern "system" fn(context: super::PVOID, pcidata: super::PPCI_COMMON_CONFIG)>;
#[cfg(feature = "winnt")]
pub type PPCI_PREPARE_MULTISTAGE_RESUME = Option<unsafe extern "system" fn(context: super::PVOID)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPCI_READ_WRITE_CONFIG = Option<unsafe extern "system" fn(context: super::PVOID, busoffset: super::ULONG, slot: super::ULONG, buffer: super::PVOID, offset: super::ULONG, length: super::ULONG) -> super::ULONG>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPCI_ROOT_BUS_CAPABILITY = Option<unsafe extern "system" fn(context: super::PVOID, hardwarecapability: PPCI_ROOT_BUS_HARDWARE_CAPABILITY)>;
#[cfg(feature = "shared")]
pub type PPCI_ROOT_BUS_HARDWARE_CAPABILITY = *mut PCI_ROOT_BUS_HARDWARE_CAPABILITY;
#[cfg(feature = "shared")]
pub type PPCI_ROOT_BUS_OSC_CONTROL_FIELD = *mut PCI_ROOT_BUS_OSC_CONTROL_FIELD;
#[cfg(feature = "shared")]
pub type PPCI_ROOT_BUS_OSC_SUPPORT_FIELD = *mut PCI_ROOT_BUS_OSC_SUPPORT_FIELD;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPCI_SET_MAX_LINK_BANDWIDTH = Option<unsafe extern "system" fn(context: super::PVOID, targetbandwidth: super::ULONG, waitforretrain: super::BOOLEAN, linkthrottled: super::PBOOLEAN, newlinkspeed: PPCI_LINK_SPEED) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_SUBSYSTEM_IDS_CAPABILITY = *mut PCI_SUBSYSTEM_IDS_CAPABILITY;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_USB4_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY = *mut PCI_EXPRESS_USB4_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PPCI_VENDOR_SPECIFIC_CAPABILITY = *mut PCI_VENDOR_SPECIFIC_CAPABILITY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPHYSICAL_COUNTER_EVENT_BUFFER_CONFIGURATION = *mut PHYSICAL_COUNTER_EVENT_BUFFER_CONFIGURATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPHYSICAL_COUNTER_EVENT_BUFFER_OVERFLOW_HANDLER = Option<unsafe extern "system" fn(eventbuffer: super::PVOID, entrysize: super::SIZE_T, numberofentries: super::SIZE_T, owninghandle: super::HANDLE)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPHYSICAL_COUNTER_OVERFLOW_HANDLER = Option<unsafe extern "system" fn(overflowbits: super::ULONGLONG, owninghandle: super::HANDLE)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPHYSICAL_COUNTER_RESOURCE_DESCRIPTOR = *mut PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPHYSICAL_COUNTER_RESOURCE_LIST = *mut PHYSICAL_COUNTER_RESOURCE_LIST;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPHYSICAL_MEMORY_RANGE = *mut PHYSICAL_MEMORY_RANGE;
#[cfg(target_arch = "x86")]
pub const PPI_SHIFT: i32 = 30;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPM_DISPATCH_TABLE = *mut PM_DISPATCH_TABLE;
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PPNP_LOCATION_INTERFACE = *mut PNP_LOCATION_INTERFACE;
#[cfg(feature = "shared")]
pub type PPOOLED_USAGE_AND_LIMITS = *mut POOLED_USAGE_AND_LIMITS;
#[cfg(feature = "shared")]
pub type PPOWER_THROTTLING_PROCESS_STATE = *mut POWER_THROTTLING_PROCESS_STATE;
#[cfg(feature = "shared")]
pub type PPOWER_THROTTLING_THREAD_STATE = *mut POWER_THROTTLING_THREAD_STATE;
#[cfg(feature = "winnt")]
pub type PPROCESS_ACCESS_TOKEN = *mut PROCESS_ACCESS_TOKEN;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPROCESS_DEVICEMAP_INFORMATION = *mut PROCESS_DEVICEMAP_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPROCESS_DEVICEMAP_INFORMATION_EX = *mut PROCESS_DEVICEMAP_INFORMATION_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPROCESS_EXCEPTION_PORT = *mut PROCESS_EXCEPTION_PORT;
#[cfg(feature = "shared")]
pub type PPROCESS_HANDLE_TRACING_ENABLE = *mut PROCESS_HANDLE_TRACING_ENABLE;
#[cfg(feature = "shared")]
pub type PPROCESS_HANDLE_TRACING_ENABLE_EX = *mut PROCESS_HANDLE_TRACING_ENABLE_EX;
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
pub type PPROCESS_HANDLE_TRACING_ENTRY = *mut PROCESS_HANDLE_TRACING_ENTRY;
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
pub type PPROCESS_HANDLE_TRACING_QUERY = *mut PROCESS_HANDLE_TRACING_QUERY;
#[cfg(feature = "shared")]
pub type PPROCESS_KEEPALIVE_COUNT_INFORMATION = *mut PROCESS_KEEPALIVE_COUNT_INFORMATION;
#[cfg(feature = "shared")]
pub type PPROCESS_MEMBERSHIP_INFORMATION = *mut PROCESS_MEMBERSHIP_INFORMATION;
#[cfg(all(feature = "shared", feature = "winternl"))]
pub type PPROCESS_REVOKE_FILE_HANDLES_INFORMATION = *mut PROCESS_REVOKE_FILE_HANDLES_INFORMATION;
#[cfg(feature = "shared")]
pub type PPROCESS_SESSION_INFORMATION = *mut PROCESS_SESSION_INFORMATION;
#[cfg(feature = "shared")]
pub type PPROCESS_SYSCALL_PROVIDER_INFORMATION = *mut PROCESS_SYSCALL_PROVIDER_INFORMATION;
#[cfg(feature = "winnt")]
pub type PPROCESS_WS_WATCH_INFORMATION = *mut PROCESS_WS_WATCH_INFORMATION;
#[cfg(feature = "shared")]
pub type PPSHED_MEMORY_DETAILS = *mut PSHED_MEMORY_DETAILS;
#[cfg(feature = "shared")]
pub type PPSHED_MEMORY_DETAILS_VALID_BITS = *mut PSHED_MEMORY_DETAILS_VALID_BITS;
pub type PPSHED_PI_ERR_READING_PCIE_OVERRIDES = *mut PSHED_PI_ERR_READING_PCIE_OVERRIDES;
pub type PPSHED_PI_VENDOR_DEFINED_ACTION = *mut PSHED_PI_VENDOR_DEFINED_ACTION;
#[cfg(feature = "shared")]
pub type PPSHED_PLATFORM_DETAILS = *mut PSHED_PLATFORM_DETAILS;
#[cfg(feature = "shared")]
pub type PPSHED_PLATFORM_DETAILS_VALID_BITS = *mut PSHED_PLATFORM_DETAILS_VALID_BITS;
pub type PROCESSINFOCLASS = i32;
pub const PROCESSOR_GENERIC_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9876ccad_47b4_4bdb_b65e_16f193c4f3db);
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_ACCESS_TOKEN {
    pub Token: super::HANDLE,
    pub Thread: super::HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION {
    pub Anonymous: PROCESS_DEVICEMAP_INFORMATION_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PROCESS_DEVICEMAP_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union PROCESS_DEVICEMAP_INFORMATION_0 {
    pub Set: PROCESS_DEVICEMAP_INFORMATION_0_0,
    pub Query: PROCESS_DEVICEMAP_INFORMATION_0_1,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PROCESS_DEVICEMAP_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_DEVICEMAP_INFORMATION_0_0 {
    pub DirectoryHandle: super::HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION_0_1 {
    pub DriveMap: super::ULONG,
    pub DriveType: [super::UCHAR; 32],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PROCESS_DEVICEMAP_INFORMATION_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION_EX {
    pub Anonymous: PROCESS_DEVICEMAP_INFORMATION_EX_0,
    pub Flags: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PROCESS_DEVICEMAP_INFORMATION_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union PROCESS_DEVICEMAP_INFORMATION_EX_0 {
    pub Set: PROCESS_DEVICEMAP_INFORMATION_EX_0_0,
    pub Query: PROCESS_DEVICEMAP_INFORMATION_EX_0_1,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PROCESS_DEVICEMAP_INFORMATION_EX_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_DEVICEMAP_INFORMATION_EX_0_0 {
    pub DirectoryHandle: super::HANDLE,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION_EX_0_1 {
    pub DriveMap: super::ULONG,
    pub DriveType: [super::UCHAR; 32],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PROCESS_DEVICEMAP_INFORMATION_EX_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_EXCEPTION_PORT {
    pub ExceptionPortHandle: super::HANDLE,
    pub StateFlags: super::ULONG,
}
pub const PROCESS_EXCEPTION_PORT_ALL_STATE_BITS: u32 = 3;
#[cfg(feature = "shared")]
pub const PROCESS_EXCEPTION_PORT_ALL_STATE_FLAGS: super::ULONG_PTR = 7;
pub const PROCESS_HANDLE_EXCEPTIONS_ENABLED: u32 = 1;
pub const PROCESS_HANDLE_RAISE_UM_EXCEPTION_ON_INVALID_HANDLE_CLOSE_DISABLED: u32 = 0;
pub const PROCESS_HANDLE_RAISE_UM_EXCEPTION_ON_INVALID_HANDLE_CLOSE_ENABLED: u32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_HANDLE_TRACING_ENABLE {
    pub Flags: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_HANDLE_TRACING_ENABLE_EX {
    pub Flags: super::ULONG,
    pub TotalSlots: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct PROCESS_HANDLE_TRACING_ENTRY {
    pub Handle: super::HANDLE,
    pub ClientId: super::CLIENT_ID,
    pub Type: super::ULONG,
    pub Stacks: [super::PVOID; 16],
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
impl Default for PROCESS_HANDLE_TRACING_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROCESS_HANDLE_TRACING_MAX_STACKS: i32 = 16;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct PROCESS_HANDLE_TRACING_QUERY {
    pub Handle: super::HANDLE,
    pub TotalTraces: super::ULONG,
    pub HandleTrace: [PROCESS_HANDLE_TRACING_ENTRY; 1],
}
#[cfg(all(feature = "shared", feature = "winnt", feature = "winternl"))]
impl Default for PROCESS_HANDLE_TRACING_QUERY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_KEEPALIVE_COUNT_INFORMATION {
    pub WakeCount: super::ULONG,
    pub NoWakeCount: super::ULONG,
}
pub const PROCESS_LUID_DOSDEVICES_ONLY: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_MEMBERSHIP_INFORMATION {
    pub ServerSiloId: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_REVOKE_FILE_HANDLES_INFORMATION {
    pub TargetDevicePath: super::UNICODE_STRING,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_SESSION_INFORMATION {
    pub SessionId: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_SYSCALL_PROVIDER_INFORMATION {
    pub ProviderId: windows_sys::core::GUID,
    pub Level: super::UCHAR,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_WS_WATCH_INFORMATION {
    pub FaultingPc: super::PVOID,
    pub FaultingVa: super::PVOID,
}
pub const PROTECTED_POOL: i32 = 0;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type PRTL_AVL_ALLOCATE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, bytesize: super::CLONG) -> super::PVOID>;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type PRTL_AVL_COMPARE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, firststruct: super::PVOID, secondstruct: super::PVOID) -> RTL_GENERIC_COMPARE_RESULTS>;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type PRTL_AVL_FREE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, buffer: super::PVOID)>;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type PRTL_AVL_MATCH_FUNCTION = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, userdata: super::PVOID, matchdata: super::PVOID) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type PRTL_AVL_TABLE = *mut RTL_AVL_TABLE;
#[cfg(feature = "shared")]
pub type PRTL_BALANCED_LINKS = *mut RTL_BALANCED_LINKS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRTL_DYNAMIC_HASH_TABLE = *mut RTL_DYNAMIC_HASH_TABLE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRTL_DYNAMIC_HASH_TABLE_CONTEXT = *mut RTL_DYNAMIC_HASH_TABLE_CONTEXT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRTL_DYNAMIC_HASH_TABLE_ENTRY = *mut RTL_DYNAMIC_HASH_TABLE_ENTRY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRTL_DYNAMIC_HASH_TABLE_ENUMERATOR = *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type PRTL_GENERIC_ALLOCATE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, bytesize: super::CLONG) -> super::PVOID>;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type PRTL_GENERIC_COMPARE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, firststruct: super::PVOID, secondstruct: super::PVOID) -> RTL_GENERIC_COMPARE_RESULTS>;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type PRTL_GENERIC_FREE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, buffer: super::PVOID)>;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type PRTL_GENERIC_TABLE = *mut RTL_GENERIC_TABLE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRTL_RUN_ONCE_INIT_FN = Option<unsafe extern "system" fn(runonce: super::PRTL_RUN_ONCE, parameter: super::PVOID, context: *mut super::PVOID) -> super::ULONG>;
pub type PRTL_SPLAY_LINKS = *mut RTL_SPLAY_LINKS;
pub type PSCREATEPROCESSNOTIFYTYPE = i32;
pub type PSCREATETHREADNOTIFYTYPE = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PSHED_MEMORY_DETAILS {
    pub Version: super::UINT16,
    pub Vb: PSHED_MEMORY_DETAILS_VALID_BITS,
    pub DdrVersion: super::UINT16,
    pub IsClosedPaged: super::BOOLEAN,
    pub ColsPerRow: super::UINT16,
    pub PagesPerRow: super::UINT16,
    pub SocketCnt: super::UINT8,
    pub ChaOnSktCnt: super::UINT8,
    pub DimmSlotCnt: super::UINT8,
    pub SubchannelCnt: super::UINT8,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PSHED_MEMORY_DETAILS_VALID_BITS {
    pub _bitfield: u32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSHED_PI_ATTEMPT_ERROR_RECOVERY = Option<unsafe extern "system" fn(plugincontext: super::PVOID, bufferlength: super::ULONG, errorrecord: PWHEA_ERROR_RECORD) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSHED_PI_CLEAR_ERROR_RECORD = Option<unsafe extern "system" fn(plugincontext: super::PVOID, flags: super::ULONG, errorrecordid: super::ULONGLONG) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PSHED_PI_CLEAR_ERROR_STATUS = Option<unsafe extern "system" fn(plugincontext: super::PVOID, errorsource: PWHEA_ERROR_SOURCE_DESCRIPTOR, bufferlength: super::ULONG, errorrecord: PWHEA_ERROR_RECORD) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PSHED_PI_DISABLE_ERROR_SOURCE = Option<unsafe extern "system" fn(plugincontext: super::PVOID, errorsource: PWHEA_ERROR_SOURCE_DESCRIPTOR) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PSHED_PI_ENABLE_ERROR_SOURCE = Option<unsafe extern "system" fn(plugincontext: super::PVOID, errorsource: PWHEA_ERROR_SOURCE_DESCRIPTOR) -> windows_sys::core::NTSTATUS>;
pub type PSHED_PI_ERR_READING_PCIE_OVERRIDES = i32;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PSHED_PI_FINALIZE_ERROR_RECORD = Option<unsafe extern "system" fn(plugincontext: super::PVOID, errorsource: PWHEA_ERROR_SOURCE_DESCRIPTOR, bufferlength: super::ULONG, errorrecord: PWHEA_ERROR_RECORD) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PSHED_PI_GET_ALL_ERROR_SOURCES = Option<unsafe extern "system" fn(plugincontext: super::PVOID, count: super::PULONG, errorsrcs: *mut PWHEA_ERROR_SOURCE_DESCRIPTOR, length: super::PULONG) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PSHED_PI_GET_ERROR_SOURCE_INFO = Option<unsafe extern "system" fn(plugincontext: super::PVOID, errorsource: PWHEA_ERROR_SOURCE_DESCRIPTOR) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSHED_PI_GET_INJECTION_CAPABILITIES = Option<unsafe extern "system" fn(plugincontext: super::PVOID, capabilities: PWHEA_ERROR_INJECTION_CAPABILITIES) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSHED_PI_INJECT_ERROR = Option<unsafe extern "system" fn(plugincontext: super::PVOID, errortype: super::ULONGLONG, parameter1: super::ULONGLONG, parameter2: super::ULONGLONG, parameter3: super::ULONGLONG, parameter4: super::ULONGLONG) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PSHED_PI_READ_ERROR_RECORD = Option<unsafe extern "system" fn(plugincontext: super::PVOID, flags: super::ULONG, errorrecordid: super::ULONGLONG, nexterrorrecordid: super::PULONGLONG, recordlength: super::PULONG, errorrecord: PWHEA_ERROR_RECORD) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PSHED_PI_RETRIEVE_ERROR_INFO = Option<unsafe extern "system" fn(plugincontext: super::PVOID, errorsource: PWHEA_ERROR_SOURCE_DESCRIPTOR, bufferlength: super::ULONGLONG, packet: PWHEA_ERROR_PACKET) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PSHED_PI_SET_ERROR_SOURCE_INFO = Option<unsafe extern "system" fn(plugincontext: super::PVOID, errorsource: PWHEA_ERROR_SOURCE_DESCRIPTOR) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "shared"))]
pub type PSHED_PI_VENDOR_DEFINED = Option<unsafe extern "system" fn(vendordefinedaction: PSHED_PI_VENDOR_DEFINED_ACTION, offset: super::UINT32, inputlength: super::UINT32, inputbuffer: super::PUINT8, outputlength: super::PUINT32, outputbuffer: super::PUINT8) -> windows_sys::core::NTSTATUS>;
pub type PSHED_PI_VENDOR_DEFINED_ACTION = i32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSHED_PI_WRITE_ERROR_RECORD = Option<unsafe extern "system" fn(plugincontext: super::PVOID, flags: super::ULONG, recordlength: super::ULONG, errorrecord: PWHEA_ERROR_RECORD) -> windows_sys::core::NTSTATUS>;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PSHED_PLATFORM_DETAILS {
    pub Version: super::UINT32,
    pub Vb: PSHED_PLATFORM_DETAILS_VALID_BITS,
    pub GicRegisterOffset: super::UINT32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct PSHED_PLATFORM_DETAILS_VALID_BITS {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
pub type PSIGNAL_REG_VALUE = *mut SIGNAL_REG_VALUE;
pub type PSILO_MONITOR = *mut _SILO_MONITOR;
#[cfg(all(feature = "shared", feature = "winternl"))]
pub type PSILO_MONITOR_REGISTRATION = *mut SILO_MONITOR_REGISTRATION;
#[cfg(feature = "shared")]
pub type PSOC_SUBSYSTEM_FAILURE_DETAILS = *mut SOC_SUBSYSTEM_FAILURE_DETAILS;
pub type PSOC_SUBSYSTEM_TYPE = *mut SOC_SUBSYSTEM_TYPE;
pub type PSUBSYSTEM_INFORMATION_TYPE = *mut SUBSYSTEM_INFORMATION_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYSTEM_FIRMWARE_TABLE_HANDLER = *mut SYSTEM_FIRMWARE_TABLE_HANDLER;
#[cfg(feature = "shared")]
pub type PSYSTEM_FIRMWARE_TABLE_INFORMATION = *mut SYSTEM_FIRMWARE_TABLE_INFORMATION;
pub const PS_IMAGE_NOTIFY_CONFLICTING_ARCHITECTURE: i32 = 1;
pub const PS_INVALID_SILO_CONTEXT_SLOT: u32 = 4294967295;
#[cfg(target_arch = "x86")]
pub const PTE_PER_PAGE: i32 = 512;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTIMER_APC_ROUTINE = Option<unsafe extern "system" fn(timercontext: super::PVOID, timerlowvalue: super::ULONG, timerhighvalue: super::LONG)>;
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PTIMER_SET_COALESCABLE_TIMER_INFO = *mut TIMER_SET_COALESCABLE_TIMER_INFO;
#[cfg(target_arch = "x86")]
pub const PTI_SHIFT: i32 = 12;
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PTRANSLATE_RESOURCE_HANDLER = Option<unsafe extern "system" fn(context: super::PVOID, source: super::PCM_PARTIAL_RESOURCE_DESCRIPTOR, direction: RESOURCE_TRANSLATION_DIRECTION, alternativescount: super::ULONG, alternatives: *const super::IO_RESOURCE_DESCRIPTOR, physicaldeviceobject: super::PDEVICE_OBJECT, target: super::PCM_PARTIAL_RESOURCE_DESCRIPTOR) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PTRANSLATE_RESOURCE_REQUIREMENTS_HANDLER = Option<unsafe extern "system" fn(context: super::PVOID, source: super::PIO_RESOURCE_DESCRIPTOR, physicaldeviceobject: super::PDEVICE_OBJECT, targetcount: super::PULONG, target: *mut super::PIO_RESOURCE_DESCRIPTOR) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type PTRANSLATOR_INTERFACE = *mut TRANSLATOR_INTERFACE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTXN_PARAMETER_BLOCK = *mut TXN_PARAMETER_BLOCK;
#[cfg(feature = "shared")]
pub type PVM_COUNTERS = *mut VM_COUNTERS;
#[cfg(feature = "shared")]
pub type PVM_COUNTERS_EX = *mut VM_COUNTERS_EX;
#[cfg(feature = "shared")]
pub type PVM_COUNTERS_EX2 = *mut VM_COUNTERS_EX2;
#[cfg(feature = "shared")]
pub type PWEHA_CXL_DEVICE_ID = *mut WHEA_CXL_DEVICE_ID;
#[cfg(feature = "shared")]
pub type PWEHA_CXL_PCIE_DEVICE_ID = *mut WHEA_CXL_PCIE_DEVICE_ID;
#[cfg(feature = "shared")]
pub type PWHEA128A = *mut WHEA128A;
#[cfg(feature = "shared")]
pub type PWHEAP_ACPI_TIMEOUT_EVENT = *mut WHEAP_ACPI_TIMEOUT_EVENT;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWHEAP_ADD_REMOVE_ERROR_SOURCE_EVENT = *mut WHEAP_ADD_REMOVE_ERROR_SOURCE_EVENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEAP_ATTEMPT_RECOVERY_EVENT = *mut WHEAP_ATTEMPT_RECOVERY_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_BAD_HEST_NOTIFY_DATA_EVENT = *mut WHEAP_BAD_HEST_NOTIFY_DATA_EVENT;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PWHEAP_BIT_OFFLINE_EVENT = *mut WHEAP_BIT_OFFLINE_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_CLEARED_POISON_EVENT = *mut WHEAP_CLEARED_POISON_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_CMCI_IMPLEMENTED_EVENT = *mut WHEAP_CMCI_IMPLEMENTED_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_CMCI_INITERR_EVENT = *mut WHEAP_CMCI_INITERR_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_CMCI_RESTART_EVENT = *mut WHEAP_CMCI_RESTART_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_CREATE_GENERIC_RECORD_EVENT = *mut WHEAP_CREATE_GENERIC_RECORD_EVENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEAP_DEFERRED_EVENT = *mut WHEAP_DEFERRED_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_DEVICE_DRV_EVENT = *mut WHEAP_DEVICE_DRV_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_DPC_ERROR_EVENT = *mut WHEAP_DPC_ERROR_EVENT;
pub type PWHEAP_DPC_ERROR_EVENT_TYPE = *mut WHEAP_DPC_ERROR_EVENT_TYPE;
#[cfg(feature = "shared")]
pub type PWHEAP_DROPPED_CORRECTED_ERROR_EVENT = *mut WHEAP_DROPPED_CORRECTED_ERROR_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_EDPC_ENABLED_EVENT = *mut WHEAP_EDPC_ENABLED_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_ERROR_CLEARED_EVENT = *mut WHEAP_ERROR_CLEARED_EVENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEAP_ERROR_RECORD_EVENT = *mut WHEAP_ERROR_RECORD_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_ERR_SRC_ARRAY_INVALID_EVENT = *mut WHEAP_ERR_SRC_ARRAY_INVALID_EVENT;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWHEAP_ERR_SRC_INVALID_EVENT = *mut WHEAP_ERR_SRC_INVALID_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_FOUND_ERROR_IN_BANK_EVENT = *mut WHEAP_FOUND_ERROR_IN_BANK_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_GENERIC_ERR_MEM_MAP_EVENT = *mut WHEAP_GENERIC_ERR_MEM_MAP_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_OSC_IMPLEMENTED = *mut WHEAP_OSC_IMPLEMENTED;
#[cfg(feature = "shared")]
pub type PWHEAP_PCIE_CONFIG_INFO = *mut WHEAP_PCIE_CONFIG_INFO;
#[cfg(feature = "shared")]
pub type PWHEAP_PCIE_OVERRIDE_INFO = *mut WHEAP_PCIE_OVERRIDE_INFO;
#[cfg(feature = "shared")]
pub type PWHEAP_PCIE_READ_OVERRIDES_ERR = *mut WHEAP_PCIE_READ_OVERRIDES_ERR;
#[cfg(feature = "shared")]
pub type PWHEAP_PFA_MEMORY_OFFLINED = *mut WHEAP_PFA_MEMORY_OFFLINED;
#[cfg(feature = "shared")]
pub type PWHEAP_PFA_MEMORY_OFFLINED_NOTIFY_CALLBACK_ACTION = *mut WHEAP_PFA_MEMORY_OFFLINED_NOTIFY_CALLBACK_ACTION;
#[cfg(feature = "shared")]
pub type PWHEAP_PFA_MEMORY_POLICY = *mut WHEAP_PFA_MEMORY_POLICY;
#[cfg(feature = "shared")]
pub type PWHEAP_PFA_MEMORY_REMOVE_MONITOR = *mut WHEAP_PFA_MEMORY_REMOVE_MONITOR;
pub type PWHEAP_PFA_OFFLINE_DECISION_TYPE = *mut WHEAP_PFA_OFFLINE_DECISION_TYPE;
#[cfg(feature = "shared")]
pub type PWHEAP_PLUGIN_DEFECT_LIST_CORRUPT = *mut WHEAP_PLUGIN_DEFECT_LIST_CORRUPT;
#[cfg(feature = "shared")]
pub type PWHEAP_PLUGIN_DEFECT_LIST_FULL_EVENT = *mut WHEAP_PLUGIN_DEFECT_LIST_FULL_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_PLUGIN_DEFECT_LIST_UEFI_VAR_FAILED = *mut WHEAP_PLUGIN_DEFECT_LIST_UEFI_VAR_FAILED;
#[cfg(feature = "shared")]
pub type PWHEAP_PLUGIN_PFA_EVENT = *mut WHEAP_PLUGIN_PFA_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_PROCESS_EINJ_EVENT = *mut WHEAP_PROCESS_EINJ_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_PROCESS_EINJ_EVENT2 = *mut WHEAP_PROCESS_EINJ_EVENT2;
#[cfg(feature = "shared")]
pub type PWHEAP_PROCESS_HEST_EVENT = *mut WHEAP_PROCESS_HEST_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_PROMOTED_AER_ERROR_EVENT = *mut WHEAP_PROMOTED_AER_ERROR_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_PSHED_INJECT_ERROR = *mut WHEAP_PSHED_INJECT_ERROR;
#[cfg(feature = "shared")]
pub type PWHEAP_PSHED_PLUGIN_REGISTER = *mut WHEAP_PSHED_PLUGIN_REGISTER;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PWHEAP_ROW_FAILURE_EVENT = *mut WHEAP_ROW_FAILURE_EVENT;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PWHEAP_ROW_OFFLINE_EVENT = *mut WHEAP_ROW_OFFLINE_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_SPURIOUS_AER_EVENT = *mut WHEAP_SPURIOUS_AER_EVENT;
#[cfg(feature = "shared")]
pub type PWHEAP_STARTED_REPORT_HW_ERROR = *mut WHEAP_STARTED_REPORT_HW_ERROR;
#[cfg(feature = "shared")]
pub type PWHEAP_STUCK_ERROR_EVENT = *mut WHEAP_STUCK_ERROR_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_ACPI_HEADER = *mut WHEA_ACPI_HEADER;
#[cfg(feature = "shared")]
pub type PWHEA_AER_BRIDGE_DESCRIPTOR = *mut WHEA_AER_BRIDGE_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PWHEA_AER_ENDPOINT_DESCRIPTOR = *mut WHEA_AER_ENDPOINT_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PWHEA_AER_ROOTPORT_DESCRIPTOR = *mut WHEA_AER_ROOTPORT_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PWHEA_AMD_EXTENDED_REGISTERS = *mut WHEA_AMD_EXTENDED_REGISTERS;
#[cfg(feature = "shared")]
pub type PWHEA_ARMV8_AARCH32_GPRS = *mut WHEA_ARMV8_AARCH32_GPRS;
#[cfg(feature = "shared")]
pub type PWHEA_ARMV8_AARCH64_EL3_CSR = *mut WHEA_ARMV8_AARCH64_EL3_CSR;
#[cfg(feature = "shared")]
pub type PWHEA_ARMV8_AARCH64_GPRS = *mut WHEA_ARMV8_AARCH64_GPRS;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_AARCH32_EL1 = *mut WHEA_ARM_AARCH32_EL1_CSR;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_AARCH32_EL2_CSR = *mut WHEA_ARM_AARCH32_EL2_CSR;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_AARCH32_SECURE_CSR = *mut WHEA_ARM_AARCH32_SECURE_CSR;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_AARCH64_EL1_CSR = *mut WHEA_ARM_AARCH64_EL1_CSR;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_AARCH64_EL2_CSR = *mut WHEA_ARM_AARCH64_EL2_CSR;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_BUS_ERROR = *mut WHEA_ARM_BUS_ERROR;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_BUS_ERROR_VALID_BITS = *mut WHEA_ARM_BUS_ERROR_VALID_BITS;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_CACHE_ERROR = *mut WHEA_ARM_CACHE_ERROR;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_CACHE_ERROR_VALID_BITS = *mut WHEA_ARM_CACHE_ERROR_VALID_BITS;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_MISC_CSR = *mut WHEA_ARM_MISC_CSR;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_PROCESSOR_ERROR = *mut WHEA_ARM_PROCESSOR_ERROR;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER = *mut WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS = *mut WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_PROCESSOR_ERROR_INFORMATION = *mut WHEA_ARM_PROCESSOR_ERROR_INFORMATION;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS = *mut WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_PROCESSOR_ERROR_SECTION = *mut WHEA_ARM_PROCESSOR_ERROR_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS = *mut WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS;
pub type PWHEA_ARM_RAS_NODE_INTERFACES = *mut WHEA_ARM_RAS_NODE_INTERFACES;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_RAS_NODE_SECTION = *mut WHEA_ARM_RAS_NODE_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_TLB_ERROR = *mut WHEA_ARM_TLB_ERROR;
#[cfg(feature = "shared")]
pub type PWHEA_ARM_TLB_ERROR_VALID_BITS = *mut WHEA_ARM_TLB_ERROR_VALID_BITS;
#[cfg(feature = "shared")]
pub type PWHEA_AZCC_ROOT_BUS_ERR_EVENT = *mut WHEA_AZCC_ROOT_BUS_ERR_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_AZCC_ROOT_BUS_LIST_EVENT = *mut WHEA_AZCC_ROOT_BUS_LIST_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_AZCC_SET_POISON_EVENT = *mut WHEA_AZCC_SET_POISON_EVENT;
pub type PWHEA_BUGCHECK_RECOVERY_LOG_TYPE = *mut WHEA_BUGCHECK_RECOVERY_LOG_TYPE;
pub type PWHEA_CPU_VENDOR = *mut WHEA_CPU_VENDOR;
#[cfg(feature = "shared")]
pub type PWHEA_CRASHDUMP_EVENT_LOG_ENTRY_ULONG1 = *mut WHEA_CRASHDUMP_EVENT_LOG_ENTRY_ULONG1;
#[cfg(feature = "shared")]
pub type PWHEA_CRASHDUMP_EVENT_LOG_ENTRY_WITH_STATUS = *mut WHEA_CRASHDUMP_EVENT_LOG_ENTRY_WITH_STATUS;
#[cfg(feature = "shared")]
pub type PWHEA_CXL_AGENT_ADDRESS = *mut WHEA_CXL_AGENT_ADDRES;
pub type PWHEA_CXL_AGENT_TYPE = *mut WHEA_CXL_AGENT_TYPE;
#[cfg(feature = "shared")]
pub type PWHEA_CXL_COMPONENT_EVENTS_SECTION = *mut WHEA_CXL_COMPONENT_EVENTS_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_CXL_COMPONENT_EVENTS_SECTION_VALIDBITS = *mut WHEA_CXL_COMPONENT_EVENTS_SECTION_VALIDBITS;
#[cfg(feature = "shared")]
pub type PWHEA_CXL_DEVICE_SERIAL_NUMBER = *mut WHEA_CXL_DEVICE_SERIAL_NUMBER;
#[cfg(feature = "shared")]
pub type PWHEA_CXL_PROTOCOL_ERROR_SECTION = *mut WHEA_CXL_PROTOCOL_ERROR_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_CXL_PROTOCOL_ERROR_SECTION_VALIDBITS = *mut WHEA_CXL_PROTOCOL_ERROR_SECTION_VALIDBITS;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWHEA_DEVICE_DRIVER_DESCRIPTOR = *mut WHEA_DEVICE_DRIVER_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PWHEA_DEVICE_INFO = *mut WHEA_DEVICE_INFO;
pub type PWHEA_DEVICE_TYPE = *mut WHEA_DEVICE_TYPE;
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
pub type PWHEA_DRIVER_BUFFER_SET = *mut WHEA_DRIVER_BUFFER_SET;
#[cfg(feature = "shared")]
pub type PWHEA_ERROR_INJECTION_CAPABILITIES = *mut WHEA_ERROR_INJECTION_CAPABILITIES;
#[cfg(feature = "shared")]
pub type PWHEA_ERROR_PACKET = *mut WHEA_ERROR_PACKET_V2;
pub type PWHEA_ERROR_PACKET_DATA_FORMAT = *mut WHEA_ERROR_PACKET_DATA_FORMAT;
#[cfg(feature = "shared")]
pub type PWHEA_ERROR_PACKET_FLAGS = *mut WHEA_ERROR_PACKET_FLAGS;
#[cfg(feature = "shared")]
pub type PWHEA_ERROR_PACKET_V1 = *mut WHEA_ERROR_PACKET_V1;
#[cfg(feature = "shared")]
pub type PWHEA_ERROR_PACKET_V2 = *mut WHEA_ERROR_PACKET_V2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_ERROR_RECORD = *mut WHEA_ERROR_RECORD;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_ERROR_RECORD_HEADER = *mut WHEA_ERROR_RECORD_HEADER;
#[cfg(feature = "shared")]
pub type PWHEA_ERROR_RECORD_HEADER_FLAGS = *mut WHEA_ERROR_RECORD_HEADER_FLAGS;
#[cfg(feature = "shared")]
pub type PWHEA_ERROR_RECORD_HEADER_VALIDBITS = *mut WHEA_ERROR_RECORD_HEADER_VALIDBITS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_ERROR_RECORD_SECTION_DESCRIPTOR = *mut WHEA_ERROR_RECORD_SECTION_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PWHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS = *mut WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS;
#[cfg(feature = "shared")]
pub type PWHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS = *mut WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type PWHEA_ERROR_RECOVERY_INFO_SECTION = *mut WHEA_ERROR_RECOVERY_INFO_SECTION;
pub type PWHEA_ERROR_SEVERITY = *mut WHEA_ERROR_SEVERITY;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWHEA_ERROR_SOURCE_CONFIGURATION = *mut WHEA_ERROR_SOURCE_CONFIGURATION;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWHEA_ERROR_SOURCE_CONFIGURATION_DD = *mut WHEA_ERROR_SOURCE_CONFIGURATION_DD;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER = *mut WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1 = *mut WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWHEA_ERROR_SOURCE_DESCRIPTOR = *mut WHEA_ERROR_SOURCE_DESCRIPTOR;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWHEA_ERROR_SOURCE_DESCRIPTOR_V2 = *mut WHEA_ERROR_SOURCE_DESCRIPTOR_V2;
#[cfg(feature = "shared")]
pub type PWHEA_ERROR_SOURCE_OVERRIDE_SETTINGS = *mut WHEA_ERROR_SOURCE_OVERRIDE_SETTINGS;
pub type PWHEA_ERROR_SOURCE_STATE = *mut WHEA_ERROR_SOURCE_STATE;
pub type PWHEA_ERROR_SOURCE_TYPE = *mut WHEA_ERROR_SOURCE_TYPE;
#[cfg(feature = "shared")]
pub type PWHEA_ERROR_STATUS = *mut WHEA_ERROR_STATUS;
pub type PWHEA_ERROR_TYPE = *mut WHEA_ERROR_TYPE;
#[cfg(feature = "shared")]
pub type PWHEA_ETW_OVERFLOW_EVENT = *mut WHEA_ETW_OVERFLOW_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_EVENT_LOG_ENTRY = *mut WHEA_EVENT_LOG_ENTRY;
#[cfg(feature = "shared")]
pub type PWHEA_EVENT_LOG_ENTRY_FLAGS = *mut WHEA_EVENT_LOG_ENTRY_FLAGS;
#[cfg(feature = "shared")]
pub type PWHEA_EVENT_LOG_ENTRY_HEADER = *mut WHEA_EVENT_LOG_ENTRY_HEADER;
pub type PWHEA_EVENT_LOG_ENTRY_ID = *mut WHEA_EVENT_LOG_ENTRY_ID;
pub type PWHEA_EVENT_LOG_ENTRY_TYPE = *mut WHEA_EVENT_LOG_ENTRY_TYPE;
#[cfg(feature = "shared")]
pub type PWHEA_EXTENDED_RAS = *mut WHEA_EXTENDED_RAS;
#[cfg(feature = "shared")]
pub type PWHEA_FAILED_ADD_DEFECT_LIST_EVENT = *mut WHEA_FAILED_ADD_DEFECT_LIST_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_FIRMWARE_ERROR_RECORD_REFERENCE = *mut WHEA_FIRMWARE_ERROR_RECORD_REFERENCE;
pub type PWHEA_GAS_ERRORS = *mut WHEA_GAS_ERRORS;
#[cfg(feature = "shared")]
pub type PWHEA_GAS_ERROR_EVENT = *mut WHEA_GAS_ERROR_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_GENERIC_ERROR = *mut WHEA_GENERIC_ERROR;
#[cfg(feature = "shared")]
pub type PWHEA_GENERIC_ERROR_BLOCKSTATUS = *mut WHEA_GENERIC_ERROR_BLOCKSTATUS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_GENERIC_ERROR_DATA_ENTRY = *mut WHEA_GENERIC_ERROR_DATA_ENTRY_V2;
#[cfg(feature = "shared")]
pub type PWHEA_GENERIC_ERROR_DATA_ENTRY_V1 = *mut WHEA_GENERIC_ERROR_DATA_ENTRY_V1;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_GENERIC_ERROR_DATA_ENTRY_V2 = *mut WHEA_GENERIC_ERROR_DATA_ENTRY_V2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_GENERIC_ERROR_DESCRIPTOR = *mut WHEA_GENERIC_ERROR_DESCRIPTOR;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_GENERIC_ERROR_DESCRIPTOR_V2 = *mut WHEA_GENERIC_ERROR_DESCRIPTOR_V2;
#[cfg(feature = "shared")]
pub type PWHEA_IN_USE_PAGE_NOTIFY_FLAGS = *mut WHEA_IN_USE_PAGE_NOTIFY_FLAGS;
#[cfg(feature = "shared")]
pub type PWHEA_IPF_CMC_DESCRIPTOR = *mut WHEA_IPF_CMC_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PWHEA_IPF_CPE_DESCRIPTOR = *mut WHEA_IPF_CPE_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PWHEA_IPF_MCA_DESCRIPTOR = *mut WHEA_IPF_MCA_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PWHEA_IPMI_LOAD_EVENT = *mut WHEA_IPMI_LOAD_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_IPMI_SUBSCRIBE_EVENT = *mut WHEA_IPMI_SUBSCRIBE_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_CORRECTABLE_ERROR_DATA = *mut WHEA_MEMORY_CORRECTABLE_ERROR_DATA;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_CORRECTABLE_ERROR_HEADER = *mut WHEA_MEMORY_CORRECTABLE_ERROR_HEADER;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_CORRECTABLE_ERROR_SECTION = *mut WHEA_MEMORY_CORRECTABLE_ERROR_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS = *mut WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS;
pub type PWHEA_MEMORY_DEFINITION = *mut WHEA_MEMORY_DEFINITION;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_ERROR_EXT_SECTION_AMD = *mut WHEA_MEMORY_ERROR_EXT_SECTION_AMD;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_ERROR_EXT_SECTION_AMD_VALIDBITS = *mut WHEA_MEMORY_ERROR_EXT_SECTION_AMD_VALIDBITS;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_ERROR_EXT_SECTION_FLAGS = *mut WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_ERROR_EXT_SECTION_INTEL = *mut WHEA_MEMORY_ERROR_EXT_SECTION_INTEL;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS = *mut WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_ERROR_SECTION = *mut WHEA_MEMORY_ERROR_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_ERROR_SECTION_VALIDBITS = *mut WHEA_MEMORY_ERROR_SECTION_VALIDBITS;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_HARDWARE_ADDRESS_AMD = *mut WHEA_MEMORY_HARDWARE_ADDRESS_AMD;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_HARDWARE_ADDRESS_INTEL = *mut WHEA_MEMORY_HARDWARE_ADDRESS_INTEL;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_RANGE = *mut WHEA_MEMORY_RANGE;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_RANGE_ERROR_SECTION = *mut WHEA_MEMORY_RANGE_ERROR_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_RANGE_ERROR_SECTION_VALIDBITS = *mut WHEA_MEMORY_RANGE_ERROR_SECTION_VALIDBITS;
#[cfg(feature = "shared")]
pub type PWHEA_MEMORY_THROTTLE_SUMMARY_FAILED_EVENT = *mut WHEA_MEMORY_THROTTLE_SUMMARY_FAILED_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_MSR_DUMP_SECTION = *mut WHEA_MSR_DUMP_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_NMI_ERROR_SECTION = *mut WHEA_NMI_ERROR_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_NMI_ERROR_SECTION_FLAGS = *mut WHEA_NMI_ERROR_SECTION_FLAGS;
#[cfg(feature = "shared")]
pub type PWHEA_NOTIFICATION_DESCRIPTOR = *mut WHEA_NOTIFICATION_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PWHEA_NOTIFICATION_FLAGS = *mut WHEA_NOTIFICATION_FLAGS;
#[cfg(feature = "shared")]
pub type PWHEA_OFFLINE_DONE_EVENT = *mut WHEA_OFFLINE_DONE_EVENT;
pub type PWHEA_OFFLINE_ERRS = *mut WHEA_OFFLINE_ERRS;
#[cfg(feature = "shared")]
pub type PWHEA_PACKET_LOG_DATA = *mut WHEA_PACKET_LOG_DATA;
#[cfg(feature = "shared")]
pub type PWHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS = *mut WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS;
#[cfg(feature = "shared")]
pub type PWHEA_PCIEXPRESS_COMMAND_STATUS = *mut WHEA_PCIEXPRESS_COMMAND_STATUS;
#[cfg(feature = "shared")]
pub type PWHEA_PCIEXPRESS_DEVICE_ID = *mut WHEA_PCIEXPRESS_DEVICE_ID;
#[cfg(feature = "shared")]
pub type PWHEA_PCIEXPRESS_ERROR_SECTION = *mut WHEA_PCIEXPRESS_ERROR_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS = *mut WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS;
#[cfg(feature = "shared")]
pub type PWHEA_PCIEXPRESS_VERSION = *mut WHEA_PCIEXPRESS_VERSION;
#[cfg(feature = "shared")]
pub type PWHEA_PCIE_ADDRESS = *mut WHEA_PCIE_ADDRESS;
#[cfg(feature = "shared")]
pub type PWHEA_PCIE_CORRECTABLE_ERROR_DEVICES = *mut WHEA_PCIE_CORRECTABLE_ERROR_DEVICES;
#[cfg(feature = "shared")]
pub type PWHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS = *mut WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS;
#[cfg(feature = "shared")]
pub type PWHEA_PCIE_CORRECTABLE_ERROR_SECTION = *mut WHEA_PCIE_CORRECTABLE_ERROR_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_PCIE_CORRECTABLE_ERROR_SECTION_HEADER = *mut WHEA_PCIE_CORRECTABLE_ERROR_SECTION_HEADER;
#[cfg(feature = "shared")]
pub type PWHEA_PCIXBUS_COMMAND = *mut WHEA_PCIXBUS_COMMAND;
#[cfg(feature = "shared")]
pub type PWHEA_PCIXBUS_ERROR_SECTION = *mut WHEA_PCIXBUS_ERROR_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_PCIXBUS_ERROR_SECTION_VALIDBITS = *mut WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS;
#[cfg(feature = "shared")]
pub type PWHEA_PCIXBUS_ID = *mut WHEA_PCIXBUS_ID;
#[cfg(feature = "shared")]
pub type PWHEA_PCIXDEVICE_ERROR_SECTION = *mut WHEA_PCIXDEVICE_ERROR_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS = *mut WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS;
#[cfg(feature = "shared")]
pub type PWHEA_PCIXDEVICE_ID = *mut WHEA_PCIXDEVICE_ID;
#[cfg(feature = "shared")]
pub type PWHEA_PCIXDEVICE_REGISTER_PAIR = *mut WHEA_PCIXDEVICE_REGISTER_PAIR;
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PWHEA_PCI_DPC_SECTION = *mut PCI_EXPRESS_DPC_CAPABILITY;
#[cfg(feature = "shared")]
pub type PWHEA_PCI_RECOVERY_SECTION = *mut WHEA_PCI_RECOVERY_SECTION;
pub type PWHEA_PCI_RECOVERY_SIGNAL = *mut WHEA_PCI_RECOVERY_SIGNAL;
pub type PWHEA_PCI_RECOVERY_STATUS = *mut WHEA_PCI_RECOVERY_STATUS;
#[cfg(feature = "shared")]
pub type PWHEA_PCI_SLOT_NUMBER = *mut WHEA_PCI_SLOT_NUMBER;
#[cfg(feature = "shared")]
pub type PWHEA_PERSISTENCE_INFO = *mut WHEA_PERSISTENCE_INFO;
pub type PWHEA_PFA_REMOVE_TRIGGER = *mut WHEA_PFA_REMOVE_TRIGGER;
#[cfg(feature = "shared")]
pub type PWHEA_PMEM_ERROR_SECTION = *mut WHEA_PMEM_ERROR_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_PMEM_ERROR_SECTION_VALIDBITS = *mut WHEA_PMEM_ERROR_SECTION_VALIDBITS;
#[cfg(feature = "shared")]
pub type PWHEA_PMEM_PAGE_RANGE = *mut WHEA_PMEM_PAGE_RANGE;
#[cfg(feature = "shared")]
pub type PWHEA_PRM_ADDRESS_TRANSLATION_BUFFER_INTEL = *mut WHEA_PRM_ADDRESS_TRANSLATION_BUFFER_INTEL;
#[cfg(feature = "shared")]
pub type PWHEA_PRM_DRAM_TO_NORMALIZED_OUT_BUFFER_AMD = *mut WHEA_PRM_DRAM_TO_NORMALIZED_OUT_BUFFER_AMD;
#[cfg(feature = "shared")]
pub type PWHEA_PRM_DRAM_TO_NORMALIZED_PARAM_BUFFER_AMD = *mut WHEA_PRM_DRAM_TO_NORMALIZED_PARAM_BUFFER_AMD;
#[cfg(feature = "shared")]
pub type PWHEA_PRM_DRAM_TO_SPA_OUT_BUFFER_AMD = *mut WHEA_PRM_DRAM_TO_SPA_OUT_BUFFER_AMD;
#[cfg(feature = "shared")]
pub type PWHEA_PRM_DRAM_TO_SPA_PARAM_BUFFER_AMD = *mut WHEA_PRM_DRAM_TO_SPA_PARAM_BUFFER_AMD;
#[cfg(feature = "shared")]
pub type PWHEA_PRM_NORMALIZED_TO_DRAM_OUT_BUFFER_AMD = *mut WHEA_PRM_NORMALIZED_TO_DRAM_OUT_BUFFER_AMD;
#[cfg(feature = "shared")]
pub type PWHEA_PRM_NORMALIZED_TO_DRAM_PARAM_BUFFER_AMD = *mut WHEA_PRM_NORMALIZED_TO_DRAM_PARAM_BUFFER_AMD;
#[cfg(feature = "shared")]
pub type PWHEA_PRM_NORMALIZED_TO_SPA_OUT_BUFFER_AMD = *mut WHEA_PRM_NORMALIZED_TO_SPA_OUT_BUFFER_AMD;
#[cfg(feature = "shared")]
pub type PWHEA_PRM_NORMALIZED_TO_SPA_PARAM_BUFFER_AMD = *mut WHEA_PRM_NORMALIZED_TO_SPA_PARAM_BUFFER_AMD;
#[cfg(feature = "shared")]
pub type PWHEA_PRM_SPA_TO_DRAM_OUT_BUFFER_AMD = *mut WHEA_PRM_SPA_TO_DRAM_OUT_BUFFER_AMD;
#[cfg(feature = "shared")]
pub type PWHEA_PRM_SPA_TO_DRAM_PARAM_BUFFER_AMD = *mut WHEA_PRM_SPA_TO_DRAM_PARAM_BUFFER_AMD;
#[cfg(feature = "shared")]
pub type PWHEA_PRM_SPA_TO_NORMALIZED_OUT_BUFFER_AMD = *mut WHEA_PRM_SPA_TO_NORMALIZED_OUT_BUFFER_AMD;
#[cfg(feature = "shared")]
pub type PWHEA_PRM_SPA_TO_NORMALIZED_PARAM_BUFFER_AMD = *mut WHEA_PRM_SPA_TO_NORMALIZED_PARAM_BUFFER_AMD;
#[cfg(feature = "shared")]
pub type PWHEA_PROCESSOR_FAMILY_INFO = *mut WHEA_PROCESSOR_FAMILY_INFO;
#[cfg(feature = "shared")]
pub type PWHEA_PROCESSOR_GENERIC_ERROR_SECTION = *mut WHEA_PROCESSOR_GENERIC_ERROR_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS = *mut WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS;
#[cfg(feature = "shared")]
pub type PWHEA_PSHED_PI_CPUID = *mut WHEA_PSHED_PI_CPUID;
#[cfg(feature = "shared")]
pub type PWHEA_PSHED_PI_CPU_BUSES_INIT_FAILED_EVENT = *mut WHEA_PSHED_PI_CPU_BUSES_INIT_FAILED_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_PSHED_PI_ERROR_RECORD_FULL_EVENT = *mut WHEA_PSHED_PI_ERROR_RECORD_FULL_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_PSHED_PI_SERVER_TYPE_EVENT = *mut WHEA_PSHED_PI_SERVER_TYPE_EVENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_PSHED_PI_TRACE_EVENT = *mut WHEA_PSHED_PI_TRACE_EVENT;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWHEA_PSHED_PLUGIN_CALLBACKS = *mut WHEA_PSHED_PLUGIN_CALLBACKS;
#[cfg(feature = "shared")]
pub type PWHEA_PSHED_PLUGIN_DIMM_MISMATCH = *mut WHEA_PSHED_PLUGIN_DIMM_MISMATCH;
pub type PWHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS = *mut WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS;
#[cfg(feature = "shared")]
pub type PWHEA_PSHED_PLUGIN_ENABLE_NOTIFY_FAILED_EVENT = *mut WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_FAILED_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_PSHED_PLUGIN_HEARTBEAT = *mut WHEA_PSHED_PLUGIN_HEARTBEAT;
#[cfg(feature = "shared")]
pub type PWHEA_PSHED_PLUGIN_INIT_FAILED_EVENT = *mut WHEA_PSHED_PLUGIN_INIT_FAILED_EVENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_PSHED_PLUGIN_LOAD_EVENT = *mut WHEA_PSHED_PLUGIN_LOAD_EVENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_PSHED_PLUGIN_PLATFORM_SUPPORT_EVENT = *mut WHEA_PSHED_PLUGIN_PLATFORM_SUPPORT_EVENT;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWHEA_PSHED_PLUGIN_REGISTRATION_PACKET = *mut WHEA_PSHED_PLUGIN_REGISTRATION_PACKET_V2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_PSHED_PLUGIN_UNLOAD_EVENT = *mut WHEA_PSHED_PLUGIN_UNLOAD_EVENT;
pub type PWHEA_RAW_DATA_FORMAT = *mut WHEA_RAW_DATA_FORMAT;
#[cfg(feature = "shared")]
pub type PWHEA_READ_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY = *mut WHEA_READ_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY;
#[cfg(feature = "shared")]
pub type PWHEA_RECOVERY_ACTION = *mut WHEA_RECOVERY_ACTION;
#[cfg(feature = "shared")]
pub type PWHEA_RECOVERY_CONTEXT = *mut WHEA_RECOVERY_CONTEXT;
pub type PWHEA_RECOVERY_CONTEXT_ACTION_TAKEN = *mut WHEA_RECOVERY_CONTEXT_ACTION_TAKEN;
#[cfg(feature = "shared")]
pub type PWHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO = *mut WHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO;
pub type PWHEA_RECOVERY_CONTEXT_ERROR_TYPE = *mut WHEA_RECOVERY_CONTEXT_ERROR_TYPE;
#[cfg(feature = "shared")]
pub type PWHEA_RECOVERY_CONTEXT_PAGE_INFO = *mut WHEA_RECOVERY_CONTEXT_PAGE_INFO;
pub type PWHEA_RECOVERY_FAILURE_REASON = *mut WHEA_RECOVERY_FAILURE_REASON;
pub type PWHEA_RECOVERY_TYPE = *mut WHEA_RECOVERY_TYPE;
#[cfg(feature = "shared")]
pub type PWHEA_REGISTER_KEY_NOTIFICATION_FAILED_EVENT = *mut WHEA_REGISTER_KEY_NOTIFICATION_FAILED_EVENT;
pub type PWHEA_REGISTRY_ERRORS = *mut WHEA_REGISTRY_ERRORS;
#[cfg(feature = "shared")]
pub type PWHEA_REGISTRY_ERROR_EVENT = *mut WHEA_REGISTRY_ERROR_EVENT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_REGNOTIFY_POLICY_CHANGE_EVENT = *mut WHEA_REGNOTIFY_POLICY_CHANGE_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS = *mut WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS;
#[cfg(feature = "shared")]
pub type PWHEA_REVISION = *mut WHEA_REVISION;
#[cfg(feature = "shared")]
pub type PWHEA_SEA_SECTION = *mut WHEA_SEA_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_SEI_SECTION = *mut WHEA_SEI_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_SEL_BUGCHECK_PROGRESS = *mut WHEA_SEL_BUGCHECK_PROGRESS;
#[cfg(feature = "shared")]
pub type PWHEA_SEL_BUGCHECK_RECOVERY_STATUS_MULTIPLE_BUGCHECK_EVENT = *mut WHEA_SEL_BUGCHECK_RECOVERY_STATUS_MULTIPLE_BUGCHECK_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT = *mut WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE2_EVENT = *mut WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE2_EVENT;
#[cfg(all(feature = "ntdef", feature = "shared"))]
pub type PWHEA_SEL_BUGCHECK_RECOVERY_STATUS_START_EVENT = *mut WHEA_SEL_BUGCHECK_RECOVERY_STATUS_START_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_SEL_RAW_EVENT = *mut WHEA_SEL_RAW_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_SRAR_DETAIL_EVENT = *mut WHEA_SRAR_DETAIL_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_SRAS_TABLE_ENTRIES_EVENT = *mut WHEA_SRAS_TABLE_ENTRIES_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_SRAS_TABLE_ERROR = *mut WHEA_SRAS_TABLE_ERROR;
#[cfg(feature = "shared")]
pub type PWHEA_SRAS_TABLE_NOT_FOUND = *mut WHEA_SRAS_TABLE_NOT_FOUND;
#[cfg(feature = "shared")]
pub type PWHEA_THROTTLE_ADD_ERR_SRC_FAILED_EVENT = *mut WHEA_THROTTLE_ADD_ERR_SRC_FAILED_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_THROTTLE_MEMORY_ADD_OR_REMOVE_EVENT = *mut WHEA_THROTTLE_MEMORY_ADD_OR_REMOVE_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_THROTTLE_PCIE_ADD_EVENT = *mut WHEA_THROTTLE_PCIE_ADD_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_THROTTLE_PCIE_REMOVE_EVENT = *mut WHEA_THROTTLE_PCIE_REMOVE_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_THROTTLE_REGISTRY_CORRUPT_EVENT = *mut WHEA_THROTTLE_REGISTRY_CORRUPT_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_THROTTLE_REG_DATA_IGNORED_EVENT = *mut WHEA_THROTTLE_REG_DATA_IGNORED_EVENT;
pub type PWHEA_THROTTLE_TYPE = *mut WHEA_THROTTLE_TYPE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_TIMESTAMP = *mut WHEA_TIMESTAMP;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_VERSION_MISMATCH_EVENT = *mut WHEA_VERSION_MISMATCH_EVENT;
#[cfg(feature = "shared")]
pub type PWHEA_WRITE_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY = *mut WHEA_WRITE_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY;
#[cfg(feature = "shared")]
pub type PWHEA_X64_REGISTER_STATE = *mut WHEA_X64_REGISTER_STATE;
#[cfg(feature = "shared")]
pub type PWHEA_X86_REGISTER_STATE = *mut WHEA_X86_REGISTER_STATE;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_BUS_CHECK = *mut WHEA_XPF_BUS_CHECK;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_CACHE_CHECK = *mut WHEA_XPF_CACHE_CHECK;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_CMC_DESCRIPTOR = *mut WHEA_XPF_CMC_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_CMC_DESCRIPTOR_V2 = *mut WHEA_XPF_CMC_DESCRIPTOR_V2;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_CONTEXT_INFO = *mut WHEA_XPF_CONTEXT_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWHEA_XPF_MCA_SECTION = *mut WHEA_XPF_MCA_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_MCE_DESCRIPTOR = *mut WHEA_XPF_MCE_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_MCE_DESCRIPTOR_V2 = *mut WHEA_XPF_MCE_DESCRIPTOR_V2;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_MC_BANK_DESCRIPTOR = *mut WHEA_XPF_MC_BANK_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_MS_CHECK = *mut WHEA_XPF_MS_CHECK;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_NMI_DESCRIPTOR = *mut WHEA_XPF_NMI_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_PROCESSOR_ERROR_SECTION = *mut WHEA_XPF_PROCESSOR_ERROR_SECTION;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS = *mut WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_PROCINFO = *mut WHEA_XPF_PROCINFO;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_PROCINFO_VALIDBITS = *mut WHEA_XPF_PROCINFO_VALIDBITS;
#[cfg(feature = "shared")]
pub type PWHEA_XPF_TLB_CHECK = *mut WHEA_XPF_TLB_CHECK;
#[cfg(feature = "shared")]
pub type PXPF_MCE_FLAGS = *mut XPF_MCE_FLAGS;
#[cfg(feature = "shared")]
pub type PXPF_MC_BANK_FLAGS = *mut XPF_MC_BANK_FLAGS;
#[cfg(feature = "shared")]
pub type PXPF_RECOVERY_INFO = *mut XPF_RECOVERY_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PZONE_HEADER = *mut ZONE_HEADER;
#[cfg(feature = "winnt")]
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
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PciLine2Pin = Option<unsafe extern "system" fn(bushandler: *const _BUS_HANDLER, roothandler: *const _BUS_HANDLER, slotnumber: super::PCI_SLOT_NUMBER, pcinewdata: super::PPCI_COMMON_CONFIG, pciolddata: super::PPCI_COMMON_CONFIG)>;
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
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type PciPin2Line = Option<unsafe extern "system" fn(bushandler: *const _BUS_HANDLER, roothandler: *const _BUS_HANDLER, slotnumber: super::PCI_SLOT_NUMBER, pcidata: super::PPCI_COMMON_CONFIG)>;
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt"))]
pub type PciReadWriteConfig = Option<unsafe extern "system" fn(bushandler: *const _BUS_HANDLER, slot: super::PCI_SLOT_NUMBER, buffer: super::PVOID, offset: super::ULONG, length: super::ULONG)>;
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
pub const ProcessAccessToken: PROCESSINFOCLASS = 9;
pub const ProcessAffinityMask: PROCESSINFOCLASS = 21;
pub const ProcessAffinityUpdateMode: PROCESSINFOCLASS = 45;
pub const ProcessBasePriority: PROCESSINFOCLASS = 5;
pub const ProcessBasicInformation: PROCESSINFOCLASS = 0;
pub const ProcessBreakOnTermination: PROCESSINFOCLASS = 29;
pub const ProcessCheckStackExtentsMode: PROCESSINFOCLASS = 59;
pub const ProcessCommandLineInformation: PROCESSINFOCLASS = 60;
pub const ProcessCommitReleaseInformation: PROCESSINFOCLASS = 65;
pub const ProcessCookie: PROCESSINFOCLASS = 36;
pub const ProcessCycleTime: PROCESSINFOCLASS = 38;
pub const ProcessDebugFlags: PROCESSINFOCLASS = 31;
pub const ProcessDebugObjectHandle: PROCESSINFOCLASS = 30;
pub const ProcessDebugPort: PROCESSINFOCLASS = 7;
pub const ProcessDefaultHardErrorMode: PROCESSINFOCLASS = 12;
pub const ProcessDeviceMap: PROCESSINFOCLASS = 23;
pub const ProcessDynamicFunctionTableInformation: PROCESSINFOCLASS = 53;
pub const ProcessEnableAlignmentFaultFixup: PROCESSINFOCLASS = 17;
pub const ProcessEnergyTrackingState: PROCESSINFOCLASS = 82;
pub const ProcessExceptionPort: PROCESSINFOCLASS = 8;
pub const ProcessExecuteFlags: PROCESSINFOCLASS = 34;
pub const ProcessFaultInformation: PROCESSINFOCLASS = 63;
pub const ProcessForegroundInformation: PROCESSINFOCLASS = 25;
pub const ProcessGroupInformation: PROCESSINFOCLASS = 47;
pub const ProcessHandleCheckingMode: PROCESSINFOCLASS = 54;
pub const ProcessHandleCount: PROCESSINFOCLASS = 20;
pub const ProcessHandleInformation: PROCESSINFOCLASS = 51;
pub const ProcessHandleTable: PROCESSINFOCLASS = 58;
pub const ProcessHandleTracing: PROCESSINFOCLASS = 32;
pub const ProcessImageFileMapping: PROCESSINFOCLASS = 44;
pub const ProcessImageFileName: PROCESSINFOCLASS = 27;
pub const ProcessImageFileNameWin32: PROCESSINFOCLASS = 43;
pub const ProcessImageInformation: PROCESSINFOCLASS = 37;
pub const ProcessInPrivate: PROCESSINFOCLASS = 70;
pub const ProcessInstrumentationCallback: PROCESSINFOCLASS = 40;
pub const ProcessIoCounters: PROCESSINFOCLASS = 2;
pub const ProcessIoPortHandlers: PROCESSINFOCLASS = 13;
pub const ProcessIoPriority: PROCESSINFOCLASS = 33;
pub const ProcessKeepAliveCount: PROCESSINFOCLASS = 55;
pub const ProcessLUIDDeviceMapsEnabled: PROCESSINFOCLASS = 28;
pub const ProcessLdtInformation: PROCESSINFOCLASS = 10;
pub const ProcessLdtSize: PROCESSINFOCLASS = 11;
pub const ProcessMemoryAllocationMode: PROCESSINFOCLASS = 46;
pub const ProcessMemoryExhaustion: PROCESSINFOCLASS = 62;
pub const ProcessMitigationPolicy: PROCESSINFOCLASS = 52;
pub const ProcessNetworkIoCounters: PROCESSINFOCLASS = 114;
pub const ProcessOwnerInformation: PROCESSINFOCLASS = 49;
pub const ProcessPagePriority: PROCESSINFOCLASS = 39;
pub const ProcessPooledUsageAndLimits: PROCESSINFOCLASS = 14;
pub const ProcessPriorityBoost: PROCESSINFOCLASS = 22;
pub const ProcessPriorityClass: PROCESSINFOCLASS = 18;
pub const ProcessProtectionInformation: PROCESSINFOCLASS = 61;
pub const ProcessQuotaLimits: PROCESSINFOCLASS = 1;
pub const ProcessRaisePriority: PROCESSINFOCLASS = 6;
pub const ProcessRaiseUMExceptionOnInvalidHandleClose: PROCESSINFOCLASS = 71;
pub const ProcessReserved1Information: PROCESSINFOCLASS = 66;
pub const ProcessReserved2Information: PROCESSINFOCLASS = 67;
pub const ProcessRevokeFileHandles: PROCESSINFOCLASS = 56;
pub const ProcessSessionInformation: PROCESSINFOCLASS = 24;
pub const ProcessSubsystemInformation: PROCESSINFOCLASS = 75;
pub const ProcessSubsystemProcess: PROCESSINFOCLASS = 68;
pub const ProcessTelemetryIdInformation: PROCESSINFOCLASS = 64;
pub const ProcessThreadStackAllocation: PROCESSINFOCLASS = 41;
pub const ProcessTimes: PROCESSINFOCLASS = 4;
pub const ProcessTlsInformation: PROCESSINFOCLASS = 35;
pub const ProcessTokenVirtualizationEnabled: PROCESSINFOCLASS = 48;
pub const ProcessUserModeIOPL: PROCESSINFOCLASS = 16;
pub const ProcessVmCounters: PROCESSINFOCLASS = 3;
pub const ProcessWin32kSyscallFilterInformation: PROCESSINFOCLASS = 79;
pub const ProcessWindowInformation: PROCESSINFOCLASS = 50;
pub const ProcessWorkingSetControl: PROCESSINFOCLASS = 57;
pub const ProcessWorkingSetWatch: PROCESSINFOCLASS = 15;
pub const ProcessWorkingSetWatchEx: PROCESSINFOCLASS = 42;
pub const ProcessWow64Information: PROCESSINFOCLASS = 26;
pub const ProcessWx86Information: PROCESSINFOCLASS = 19;
pub const PsCreateProcessNotifySubsystems: PSCREATEPROCESSNOTIFYTYPE = 0;
pub const PsCreateThreadNotifyNonSystem: PSCREATETHREADNOTIFYTYPE = 0;
pub const PsCreateThreadNotifySubsystems: PSCREATETHREADNOTIFYTYPE = 1;
pub const PshedFADiscovery: i32 = 1;
pub const PshedFAErrorInfoRetrieval: i32 = 8;
pub const PshedFAErrorInjection: i32 = 32;
pub const PshedFAErrorRecordPersistence: i32 = 4;
pub const PshedFAErrorRecovery: i32 = 16;
pub const PshedFAErrorSourceControl: i32 = 2;
pub const PshedFAVendorDefinedErrInj: i32 = 48;
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
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type RTL_AVL_ALLOCATE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, bytesize: super::CLONG) -> super::PVOID>;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type RTL_AVL_COMPARE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, firststruct: super::PVOID, secondstruct: super::PVOID) -> RTL_GENERIC_COMPARE_RESULTS>;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type RTL_AVL_FREE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, buffer: super::PVOID)>;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type RTL_AVL_MATCH_FUNCTION = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, userdata: super::PVOID, matchdata: super::PVOID) -> windows_sys::core::NTSTATUS>;
#[repr(C)]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct RTL_AVL_TABLE {
    pub BalancedRoot: RTL_BALANCED_LINKS,
    pub OrderedPointer: super::PVOID,
    pub WhichOrderedElement: super::ULONG,
    pub NumberGenericTableElements: super::ULONG,
    pub DepthOfTree: super::ULONG,
    pub RestartKey: PRTL_BALANCED_LINKS,
    pub DeleteCount: super::ULONG,
    pub CompareRoutine: PRTL_AVL_COMPARE_ROUTINE,
    pub AllocateRoutine: PRTL_AVL_ALLOCATE_ROUTINE,
    pub FreeRoutine: PRTL_AVL_FREE_ROUTINE,
    pub TableContext: super::PVOID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct RTL_BALANCED_LINKS {
    pub Parent: *mut Self,
    pub LeftChild: *mut Self,
    pub RightChild: *mut Self,
    pub Balance: super::CHAR,
    pub Reserved: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl Default for RTL_BALANCED_LINKS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct RTL_DYNAMIC_HASH_TABLE {
    pub Flags: super::ULONG,
    pub Shift: super::ULONG,
    pub TableSize: super::ULONG,
    pub Pivot: super::ULONG,
    pub DivisorMask: super::ULONG,
    pub NumEntries: super::ULONG,
    pub NonEmptyBuckets: super::ULONG,
    pub NumEnumerators: super::ULONG,
    pub Directory: super::PVOID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct RTL_DYNAMIC_HASH_TABLE_CONTEXT {
    pub ChainHead: super::PLIST_ENTRY,
    pub PrevLinkage: super::PLIST_ENTRY,
    pub Signature: super::ULONG_PTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct RTL_DYNAMIC_HASH_TABLE_ENTRY {
    pub Linkage: super::LIST_ENTRY,
    pub Signature: super::ULONG_PTR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct RTL_DYNAMIC_HASH_TABLE_ENUMERATOR {
    pub Anonymous: RTL_DYNAMIC_HASH_TABLE_ENUMERATOR_0,
    pub ChainHead: super::PLIST_ENTRY,
    pub BucketIndex: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RTL_DYNAMIC_HASH_TABLE_ENUMERATOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union RTL_DYNAMIC_HASH_TABLE_ENUMERATOR_0 {
    pub HashEntry: RTL_DYNAMIC_HASH_TABLE_ENTRY,
    pub CurEntry: super::PLIST_ENTRY,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RTL_DYNAMIC_HASH_TABLE_ENUMERATOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type RTL_GENERIC_ALLOCATE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, bytesize: super::CLONG) -> super::PVOID>;
pub type RTL_GENERIC_COMPARE_RESULTS = i32;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type RTL_GENERIC_COMPARE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, firststruct: super::PVOID, secondstruct: super::PVOID) -> RTL_GENERIC_COMPARE_RESULTS>;
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type RTL_GENERIC_FREE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, buffer: super::PVOID)>;
#[repr(C)]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct RTL_GENERIC_TABLE {
    pub TableRoot: PRTL_SPLAY_LINKS,
    pub InsertOrderList: super::LIST_ENTRY,
    pub OrderedPointer: super::PLIST_ENTRY,
    pub WhichOrderedElement: super::ULONG,
    pub NumberGenericTableElements: super::ULONG,
    pub CompareRoutine: PRTL_GENERIC_COMPARE_ROUTINE,
    pub AllocateRoutine: PRTL_GENERIC_ALLOCATE_ROUTINE,
    pub FreeRoutine: PRTL_GENERIC_FREE_ROUTINE,
    pub TableContext: super::PVOID,
}
pub const RTL_HASH_ALLOCATED_HEADER: i32 = 1;
pub const RTL_HASH_RESERVED_SIGNATURE: i32 = 0;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type RTL_RUN_ONCE_INIT_FN = Option<unsafe extern "system" fn(runonce: super::PRTL_RUN_ONCE, parameter: super::PVOID, context: *mut super::PVOID) -> super::ULONG>;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RTL_SPLAY_LINKS {
    pub Parent: *mut Self,
    pub LeftChild: *mut Self,
    pub RightChild: *mut Self,
}
pub const RTL_STACK_WALKING_MODE_FRAMES_TO_SKIP_SHIFT: i32 = 8;
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
#[cfg(target_arch = "x86")]
pub const ResultNegative: INTERLOCKED_RESULT = 32768;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ResultNegative: INTERLOCKED_RESULT = 1;
#[cfg(target_arch = "x86")]
pub const ResultPositive: INTERLOCKED_RESULT = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ResultPositive: INTERLOCKED_RESULT = 2;
#[cfg(target_arch = "x86")]
pub const ResultZero: INTERLOCKED_RESULT = 16384;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const ResultZero: INTERLOCKED_RESULT = 0;
pub const RowErrorDdr4: PAGE_OFFLINE_ERROR_TYPES = 1;
pub const RowErrorDdr5: PAGE_OFFLINE_ERROR_TYPES = 3;
pub const SCI_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe9d59197_94ee_4a4f_8ad8_9b7d8bd93d2e);
pub const SEA_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9a78788a_bbe8_11e4_809e_67611e5d46b0);
pub const SEA_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf5fe48a6_84ce_4c1e_aa64_20c9a53099f1);
pub const SEH_VALIDATION_POLICY_DEFER: i32 = 3;
pub const SEH_VALIDATION_POLICY_OFF: i32 = 1;
pub const SEH_VALIDATION_POLICY_ON: i32 = 0;
pub const SEH_VALIDATION_POLICY_TELEMETRY: i32 = 2;
pub const SEI_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5c284c81_b0ae_4e87_a322_b04c85624323);
pub const SEI_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf2a4a152_9c6d_4020_aecf_7695b389251b);
pub const SE_UNSOLICITED_INPUT_PRIVILEGE: i32 = 6;
pub const SHARED_GLOBAL_FLAGS_ADMINAPPROVALMODE_TYPE_SHADOWADMIN: u32 = 4096;
pub const SHARED_GLOBAL_FLAGS_ADMINAPPROVALMODE_TYPE_SHADOWADMIN_V: i32 = 12;
pub const SHARED_GLOBAL_FLAGS_ADMINAPPROVALMODE_TYPE_SPLITTOKEN: u32 = 2048;
pub const SHARED_GLOBAL_FLAGS_ADMINAPPROVALMODE_TYPE_SPLITTOKEN_V: i32 = 11;
pub const SHARED_GLOBAL_FLAGS_CLEAR_GLOBAL_DATA_FLAG: u32 = 2147483648;
pub const SHARED_GLOBAL_FLAGS_CONSOLE_BROKER_ENABLED: u32 = 64;
pub const SHARED_GLOBAL_FLAGS_CONSOLE_BROKER_ENABLED_V: i32 = 6;
pub const SHARED_GLOBAL_FLAGS_DYNAMIC_PROC_ENABLED: u32 = 32;
pub const SHARED_GLOBAL_FLAGS_DYNAMIC_PROC_ENABLED_V: i32 = 5;
pub const SHARED_GLOBAL_FLAGS_ELEVATION_ENABLED: u32 = 2;
pub const SHARED_GLOBAL_FLAGS_ELEVATION_ENABLED_V: i32 = 1;
pub const SHARED_GLOBAL_FLAGS_ERROR_PORT: u32 = 1;
pub const SHARED_GLOBAL_FLAGS_ERROR_PORT_V: i32 = 0;
pub const SHARED_GLOBAL_FLAGS_INSTALLER_DETECT_ENABLED: u32 = 8;
pub const SHARED_GLOBAL_FLAGS_INSTALLER_DETECT_ENABLED_V: i32 = 3;
pub const SHARED_GLOBAL_FLAGS_LKG_ENABLED: u32 = 16;
pub const SHARED_GLOBAL_FLAGS_LKG_ENABLED_V: i32 = 4;
pub const SHARED_GLOBAL_FLAGS_MULTIUSERS_IN_SESSION_SKU: u32 = 512;
pub const SHARED_GLOBAL_FLAGS_MULTIUSERS_IN_SESSION_SKU_V: i32 = 9;
pub const SHARED_GLOBAL_FLAGS_MULTI_SESSION_SKU: u32 = 256;
pub const SHARED_GLOBAL_FLAGS_MULTI_SESSION_SKU_V: i32 = 8;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_A73_ERRATA: i32 = 64;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_DISABLE_32BIT: i32 = 4;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_ENABLED: i32 = 1;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_HV_PAGE: i32 = 2;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_LFENCE: i32 = 32;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_MFENCE: i32 = 16;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_RDTSCP: i32 = 128;
pub const SHARED_GLOBAL_FLAGS_SECURE_BOOT_ENABLED: u32 = 128;
pub const SHARED_GLOBAL_FLAGS_SECURE_BOOT_ENABLED_V: i32 = 7;
pub const SHARED_GLOBAL_FLAGS_SET_GLOBAL_DATA_FLAG: i32 = 1073741824;
pub const SHARED_GLOBAL_FLAGS_STATE_SEPARATION_ENABLED: u32 = 1024;
pub const SHARED_GLOBAL_FLAGS_STATE_SEPARATION_ENABLED_V: i32 = 10;
pub const SHARED_GLOBAL_FLAGS_VIRT_ENABLED: u32 = 4;
pub const SHARED_GLOBAL_FLAGS_VIRT_ENABLED_V: i32 = 2;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SIGNAL_REG_VALUE {
    pub RegName: [super::UINT8; 32],
    pub MsrAddr: super::UINT32,
    pub Value: super::UINT64,
}
#[cfg(feature = "shared")]
impl Default for SIGNAL_REG_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type SILO_CONTEXT_CLEANUP_CALLBACK = Option<unsafe extern "system" fn(silocontext: super::PVOID)>;
pub type SILO_MONITOR_CREATE_CALLBACK = Option<unsafe extern "system" fn(silo: PESILO) -> windows_sys::core::NTSTATUS>;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct SILO_MONITOR_REGISTRATION {
    pub Version: super::UCHAR,
    pub MonitorHost: super::BOOLEAN,
    pub MonitorExistingSilos: super::BOOLEAN,
    pub Reserved: [super::UCHAR; 5],
    pub Anonymous: SILO_MONITOR_REGISTRATION_0,
    pub CreateCallback: SILO_MONITOR_CREATE_CALLBACK,
    pub TerminateCallback: SILO_MONITOR_TERMINATE_CALLBACK,
}
#[cfg(all(feature = "shared", feature = "winternl"))]
impl Default for SILO_MONITOR_REGISTRATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winternl"))]
#[derive(Clone, Copy)]
pub union SILO_MONITOR_REGISTRATION_0 {
    pub DriverObjectName: super::PUNICODE_STRING,
    pub ComponentName: super::PUNICODE_STRING,
}
#[cfg(all(feature = "shared", feature = "winternl"))]
impl Default for SILO_MONITOR_REGISTRATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SILO_MONITOR_REGISTRATION_VERSION: i32 = 1;
pub type SILO_MONITOR_TERMINATE_CALLBACK = Option<unsafe extern "system" fn(silo: PESILO)>;
pub const SINGLE_GROUP_LEGACY_API: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SOC_SUBSYSTEM_FAILURE_DETAILS {
    pub SubsysType: SOC_SUBSYSTEM_TYPE,
    pub FirmwareVersion: super::ULONG64,
    pub HardwareVersion: super::ULONG64,
    pub UnifiedFailureRegionSize: super::ULONG,
    pub UnifiedFailureRegion: [super::CHAR; 1],
}
#[cfg(feature = "shared")]
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
pub const SSINFO_FLAGS_ALIGNED_DEVICE: i32 = 1;
pub const SSINFO_FLAGS_BYTE_ADDRESSABLE: i32 = 16;
pub const SSINFO_FLAGS_NO_SEEK_PENALTY: i32 = 4;
pub const SSINFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE: i32 = 2;
pub const SSINFO_FLAGS_TRIM_ENABLED: i32 = 8;
pub const SSINFO_OFFSET_UNKNOWN: u32 = 4294967295;
pub type STATE_LOCATION_TYPE = i32;
pub type SUBSYSTEM_INFORMATION_TYPE = i32;
pub const SYSTEM_CALL_INT_2E: i32 = 1;
pub const SYSTEM_CALL_SYSCALL: i32 = 0;
pub type SYSTEM_FIRMWARE_TABLE_ACTION = i32;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct SYSTEM_FIRMWARE_TABLE_HANDLER {
    pub ProviderSignature: super::ULONG,
    pub Register: super::BOOLEAN,
    pub FirmwareTableHandler: PFNFTH,
    pub DriverObject: super::PVOID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct SYSTEM_FIRMWARE_TABLE_INFORMATION {
    pub ProviderSignature: super::ULONG,
    pub Action: SYSTEM_FIRMWARE_TABLE_ACTION,
    pub TableID: super::ULONG,
    pub TableBufferLength: super::ULONG,
    pub TableBuffer: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
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
pub type THREADINFOCLASS = i32;
pub const THREAD_CSWITCH_PMU_DISABLE: i32 = 0;
pub const THREAD_CSWITCH_PMU_ENABLE: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy)]
pub struct TIMER_SET_COALESCABLE_TIMER_INFO {
    pub DueTime: super::LARGE_INTEGER,
    pub TimerApcRoutine: PTIMER_APC_ROUTINE,
    pub TimerContext: super::PVOID,
    pub WakeContext: *mut super::COUNTED_REASON_CONTEXT,
    pub Period: super::ULONG,
    pub TolerableDelay: super::ULONG,
    pub PreviousState: super::PBOOLEAN,
}
#[cfg(all(feature = "shared", feature = "wdm", feature = "winnt", feature = "winternl"))]
impl Default for TIMER_SET_COALESCABLE_TIMER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type TIMER_SET_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
#[derive(Clone, Copy, Default)]
pub struct TRANSLATOR_INTERFACE {
    pub Size: super::USHORT,
    pub Version: super::USHORT,
    pub Context: super::PVOID,
    pub InterfaceReference: super::PINTERFACE_REFERENCE,
    pub InterfaceDereference: super::PINTERFACE_DEREFERENCE,
    pub TranslateResources: PTRANSLATE_RESOURCE_HANDLER,
    pub TranslateResourceRequirements: PTRANSLATE_RESOURCE_REQUIREMENTS_HANDLER,
}
pub const TXF_MINIVERSION_DEFAULT_VIEW: i32 = 65534;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct TXN_PARAMETER_BLOCK {
    pub Length: super::USHORT,
    pub TxFsContext: super::USHORT,
    pub TransactionObject: super::PVOID,
}
pub const TableEmptyTree: TABLE_SEARCH_RESULT = 0;
pub const TableFoundNode: TABLE_SEARCH_RESULT = 1;
pub const TableInsertAsLeft: TABLE_SEARCH_RESULT = 2;
pub const TableInsertAsRight: TABLE_SEARCH_RESULT = 3;
pub const TapeController: CONFIGURATION_TYPE = 14;
pub const TapePeripheral: CONFIGURATION_TYPE = 27;
pub const TcAdapter: CONFIGURATION_TYPE = 9;
pub const TerminalPeripheral: CONFIGURATION_TYPE = 33;
pub const ThreadActualBasePriority: THREADINFOCLASS = 25;
pub const ThreadActualGroupAffinity: THREADINFOCLASS = 41;
pub const ThreadAffinityMask: THREADINFOCLASS = 4;
pub const ThreadAmILastThread: THREADINFOCLASS = 12;
pub const ThreadBasePriority: THREADINFOCLASS = 3;
pub const ThreadBasicInformation: THREADINFOCLASS = 0;
pub const ThreadBreakOnTermination: THREADINFOCLASS = 18;
pub const ThreadCSwitchMon: THREADINFOCLASS = 27;
pub const ThreadCSwitchPmu: THREADINFOCLASS = 28;
pub const ThreadCounterProfiling: THREADINFOCLASS = 32;
pub const ThreadCpuAccountingInformation: THREADINFOCLASS = 34;
pub const ThreadCycleTime: THREADINFOCLASS = 23;
pub const ThreadDescriptorTableEntry: THREADINFOCLASS = 6;
pub const ThreadDynamicCodePolicyInfo: THREADINFOCLASS = 42;
pub const ThreadEnableAlignmentFaultFixup: THREADINFOCLASS = 7;
pub const ThreadEventPair_Reusable: THREADINFOCLASS = 8;
pub const ThreadGroupInformation: THREADINFOCLASS = 30;
pub const ThreadHideFromDebugger: THREADINFOCLASS = 17;
pub const ThreadIdealProcessor: THREADINFOCLASS = 13;
pub const ThreadIdealProcessorEx: THREADINFOCLASS = 33;
pub const ThreadImpersonationToken: THREADINFOCLASS = 5;
pub const ThreadIoPriority: THREADINFOCLASS = 22;
pub const ThreadIsIoPending: THREADINFOCLASS = 16;
pub const ThreadIsTerminated: THREADINFOCLASS = 20;
pub const ThreadLastSystemCall: THREADINFOCLASS = 21;
pub const ThreadNameInformation: THREADINFOCLASS = 38;
pub const ThreadPagePriority: THREADINFOCLASS = 24;
pub const ThreadPerformanceCount: THREADINFOCLASS = 11;
pub const ThreadPriority: THREADINFOCLASS = 2;
pub const ThreadPriorityBoost: THREADINFOCLASS = 14;
pub const ThreadQuerySetWin32StartAddress: THREADINFOCLASS = 9;
pub const ThreadSetTlsArrayAddress: THREADINFOCLASS = 15;
pub const ThreadSubsystemInformation: THREADINFOCLASS = 45;
pub const ThreadSuspendCount: THREADINFOCLASS = 35;
pub const ThreadSwitchLegacyState: THREADINFOCLASS = 19;
pub const ThreadTebInformation: THREADINFOCLASS = 26;
pub const ThreadTimes: THREADINFOCLASS = 1;
pub const ThreadUmsInformation: THREADINFOCLASS = 31;
pub const ThreadWow64Context: THREADINFOCLASS = 29;
pub const ThreadZeroTlsCell: THREADINFOCLASS = 10;
pub const TimerSetCoalescableTimer: TIMER_SET_INFORMATION_CLASS = 0;
pub const TranslateChildToParent: RESOURCE_TRANSLATION_DIRECTION = 0;
pub const TranslateParentToChild: RESOURCE_TRANSLATION_DIRECTION = 1;
pub const UnsupportedDdrVersion: WHEA_OFFLINE_ERRS = 9;
pub const VENDOR_RESERVED_DOE_TYPE_DISCOVERY: i32 = 0;
pub const VMEConfiguration: BUS_DATA_TYPE = 5;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct VM_COUNTERS {
    pub PeakVirtualSize: super::SIZE_T,
    pub VirtualSize: super::SIZE_T,
    pub PageFaultCount: super::ULONG,
    pub PeakWorkingSetSize: super::SIZE_T,
    pub WorkingSetSize: super::SIZE_T,
    pub QuotaPeakPagedPoolUsage: super::SIZE_T,
    pub QuotaPagedPoolUsage: super::SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: super::SIZE_T,
    pub QuotaNonPagedPoolUsage: super::SIZE_T,
    pub PagefileUsage: super::SIZE_T,
    pub PeakPagefileUsage: super::SIZE_T,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct VM_COUNTERS_EX {
    pub PeakVirtualSize: super::SIZE_T,
    pub VirtualSize: super::SIZE_T,
    pub PageFaultCount: super::ULONG,
    pub PeakWorkingSetSize: super::SIZE_T,
    pub WorkingSetSize: super::SIZE_T,
    pub QuotaPeakPagedPoolUsage: super::SIZE_T,
    pub QuotaPagedPoolUsage: super::SIZE_T,
    pub QuotaPeakNonPagedPoolUsage: super::SIZE_T,
    pub QuotaNonPagedPoolUsage: super::SIZE_T,
    pub PagefileUsage: super::SIZE_T,
    pub PeakPagefileUsage: super::SIZE_T,
    pub PrivateUsage: super::SIZE_T,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct VM_COUNTERS_EX2 {
    pub CountersEx: VM_COUNTERS_EX,
    pub PrivateWorkingSetSize: super::SIZE_T,
    pub SharedCommitUsage: super::ULONGLONG,
}
pub const VendorDefinedRead: PSHED_PI_VENDOR_DEFINED_ACTION = 0;
pub const VendorDefinedWrite: PSHED_PI_VENDOR_DEFINED_ACTION = 1;
pub const WCS_RAS_REGISTER_NAME_MAX_LENGTH: i32 = 32;
#[cfg(feature = "shared")]
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct WHEA128A {
    pub Low: super::ULONGLONG,
    pub High: super::LONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA128A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_ACPI_TIMEOUT_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub TableType: [super::CHAR; 32],
    pub TableRequest: [super::CHAR; 32],
}
#[cfg(feature = "shared")]
impl Default for WHEAP_ACPI_TIMEOUT_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEAP_ADD_REMOVE_ERROR_SOURCE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Descriptor: WHEA_ERROR_SOURCE_DESCRIPTOR,
    pub Status: windows_sys::core::NTSTATUS,
    pub IsRemove: super::BOOLEAN,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for WHEAP_ADD_REMOVE_ERROR_SOURCE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEAP_ATTEMPT_RECOVERY_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorHeader: WHEA_ERROR_RECORD_HEADER,
    pub ArchitecturalRecovery: super::BOOLEAN,
    pub PshedRecovery: super::BOOLEAN,
    pub Status: windows_sys::core::NTSTATUS,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEAP_ATTEMPT_RECOVERY_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_BAD_HEST_NOTIFY_DATA_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub SourceId: super::USHORT,
    pub Reserved: super::USHORT,
    pub NotifyDesc: WHEA_NOTIFICATION_DESCRIPTOR,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_BAD_HEST_NOTIFY_DATA_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct WHEAP_BIT_OFFLINE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Page: super::PFN_NUMBER,
    pub Status: windows_sys::core::NTSTATUS,
    pub ErrorReason: WHEA_OFFLINE_ERRS,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for WHEAP_BIT_OFFLINE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_CLEARED_POISON_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PhysicalAddress: super::ULONG64,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_CLEARED_POISON_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_CMCI_IMPLEMENTED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub CmciAvailable: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_CMCI_IMPLEMENTED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_CMCI_INITERR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Msr: super::ULONGLONG,
    pub Type: super::ULONG,
    pub Bank: super::ULONG,
    pub EpIndex: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_CMCI_INITERR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_CMCI_RESTART_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub CmciRestoreAttempts: super::ULONG,
    pub MaxCmciRestoreLimit: super::ULONG,
    pub MaxCorrectedErrorsFound: super::ULONG,
    pub MaxCorrectedErrorLimit: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_CMCI_RESTART_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_CREATE_GENERIC_RECORD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Error: [super::CHAR; 32],
    pub EntryCount: super::ULONG,
    pub Status: windows_sys::core::NTSTATUS,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_CREATE_GENERIC_RECORD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEAP_DEFERRED_EVENT {
    pub ListEntry: super::LIST_ENTRY,
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEAP_DEFERRED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_DEVICE_DRV_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Function: [super::CHAR; 32],
}
#[cfg(feature = "shared")]
impl Default for WHEAP_DEVICE_DRV_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_DPC_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrType: WHEAP_DPC_ERROR_EVENT_TYPE,
    pub Bus: super::ULONG,
    pub Device: super::ULONG,
    pub Function: super::ULONG,
    pub DeviceId: super::USHORT,
    pub VendorId: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_DPC_ERROR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEAP_DPC_ERROR_EVENT_TYPE = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_DROPPED_CORRECTED_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorSourceType: WHEA_ERROR_SOURCE_TYPE,
    pub ErrorSourceId: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_DROPPED_CORRECTED_ERROR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_EDPC_ENABLED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub eDPCEnabled: super::BOOLEAN,
    pub eDPCRecovEnabled: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_EDPC_ENABLED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_ERROR_CLEARED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub EpIndex: super::ULONG,
    pub Bank: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_ERROR_CLEARED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEAP_ERROR_RECORD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Record: PWHEA_ERROR_RECORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEAP_ERROR_RECORD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_ERR_SRC_ARRAY_INVALID_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorSourceCount: super::ULONG,
    pub ReportedLength: super::ULONG,
    pub ExpectedLength: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_ERR_SRC_ARRAY_INVALID_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEAP_ERR_SRC_INVALID_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrDescriptor: WHEA_ERROR_SOURCE_DESCRIPTOR,
    pub Error: [super::CHAR; 32],
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for WHEAP_ERR_SRC_INVALID_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_FOUND_ERROR_IN_BANK_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub EpIndex: super::ULONG,
    pub Bank: super::ULONG,
    pub MciStatus: super::ULONG64,
    pub ErrorType: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_FOUND_ERROR_IN_BANK_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_GENERIC_ERR_MEM_MAP_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub MapReason: [super::CHAR; 32],
    pub PhysicalAddress: super::ULONG64,
    pub Length: super::ULONG64,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_GENERIC_ERR_MEM_MAP_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEAP_MAX_SET_ERROR_TYPE_WITH_ADDRESS_SIZE: i32 = 256;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_OSC_IMPLEMENTED {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub OscImplemented: super::BOOLEAN,
    pub DebugChecked: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_OSC_IMPLEMENTED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PCIE_CONFIG_INFO {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Segment: super::UINT32,
    pub Bus: super::UINT32,
    pub Device: super::UINT32,
    pub Function: super::UINT32,
    pub Offset: super::UINT32,
    pub Length: super::UINT32,
    pub Value: super::UINT64,
    pub Succeeded: super::UINT8,
    pub Reserved: [super::UINT8; 3],
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PCIE_CONFIG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PCIE_OVERRIDE_INFO {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Segment: super::UINT32,
    pub Bus: super::UINT32,
    pub Device: super::UINT32,
    pub Function: super::UINT32,
    pub ValidBits: super::UINT8,
    pub Reserved: [super::UINT8; 3],
    pub UncorrectableErrorMask: super::UINT32,
    pub UncorrectableErrorSeverity: super::UINT32,
    pub CorrectableErrorMask: super::UINT32,
    pub CapAndControl: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PCIE_OVERRIDE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PCIE_READ_OVERRIDES_ERR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub FailureReason: super::UINT32,
    pub FailureStatus: windows_sys::core::NTSTATUS,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PCIE_READ_OVERRIDES_ERR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PFA_MEMORY_OFFLINED {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub DecisionType: WHEAP_PFA_OFFLINE_DECISION_TYPE,
    pub ImmediateSuccess: super::BOOLEAN,
    pub Page: super::ULONG,
    pub NotifyVid: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PFA_MEMORY_OFFLINED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PFA_MEMORY_OFFLINED_NOTIFY_CALLBACK_ACTION {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Page: super::ULONG,
    pub ComponentTag: super::ULONG,
    pub Status: windows_sys::core::NTSTATUS,
    pub ActionTaken: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN,
    pub ActionTakenAdditionalInfo: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PFA_MEMORY_OFFLINED_NOTIFY_CALLBACK_ACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PFA_MEMORY_POLICY {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RegistryKeysPresent: super::ULONG,
    pub DisableOffline: super::BOOLEAN,
    pub PersistOffline: super::BOOLEAN,
    pub PfaDisabled: super::BOOLEAN,
    pub PageCount: super::ULONG,
    pub ErrorThreshold: super::ULONG,
    pub TimeOut: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PFA_MEMORY_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PFA_MEMORY_REMOVE_MONITOR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RemoveTrigger: WHEA_PFA_REMOVE_TRIGGER,
    pub TimeInList: super::ULONG,
    pub ErrorCount: super::ULONG,
    pub Page: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PFA_MEMORY_REMOVE_MONITOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEAP_PFA_OFFLINE_DECISION_TYPE = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PLUGIN_DEFECT_LIST_CORRUPT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PLUGIN_DEFECT_LIST_CORRUPT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PLUGIN_DEFECT_LIST_FULL_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PLUGIN_DEFECT_LIST_FULL_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PLUGIN_DEFECT_LIST_UEFI_VAR_FAILED {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PLUGIN_DEFECT_LIST_UEFI_VAR_FAILED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PLUGIN_PFA_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub NoFurtherPfa: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PLUGIN_PFA_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PROCESS_EINJ_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Error: [super::CHAR; 32],
    pub InjectionActionTableValid: super::BOOLEAN,
    pub BeginInjectionInstructionCount: super::ULONG,
    pub GetTriggerErrorActionTableInstructionCount: super::ULONG,
    pub SetErrorTypeInstructionCount: super::ULONG,
    pub GetErrorTypeInstructionCount: super::ULONG,
    pub EndOperationInstructionCount: super::ULONG,
    pub ExecuteOperationInstructionCount: super::ULONG,
    pub CheckBusyStatusInstructionCount: super::ULONG,
    pub GetCommandStatusInstructionCount: super::ULONG,
    pub SetErrorTypeWithAddressInstructionCount: super::ULONG,
    pub GetExecuteOperationTimingsInstructionCount: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PROCESS_EINJ_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PROCESS_EINJ_EVENT2 {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Error: [super::CHAR; 32],
    pub InjectionActionTableValid: super::BOOLEAN,
    pub BeginInjectionInstructionCount: super::ULONG,
    pub GetTriggerErrorActionTableInstructionCount: super::ULONG,
    pub SetErrorTypeInstructionCount: super::ULONG,
    pub GetErrorTypeInstructionCount: super::ULONG,
    pub EndOperationInstructionCount: super::ULONG,
    pub ExecuteOperationInstructionCount: super::ULONG,
    pub CheckBusyStatusInstructionCount: super::ULONG,
    pub GetCommandStatusInstructionCount: super::ULONG,
    pub SetErrorTypeWithAddressInstructionCount: super::ULONG,
    pub GetExecuteOperationTimingsInstructionCount: super::ULONG,
    pub SetErrorTypeWithAddressPa: super::ULONG64,
    pub SetErrorTypeWithAddress: [super::UCHAR; 256],
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PROCESS_EINJ_EVENT2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PROCESS_HEST_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Error: [super::CHAR; 32],
    pub EntryType: [super::CHAR; 32],
    pub EntryIndex: super::ULONG,
    pub HestValid: super::BOOLEAN,
    pub CmcCount: super::ULONG,
    pub MceCount: super::ULONG,
    pub NmiCount: super::ULONG,
    pub AerRootCount: super::ULONG,
    pub AerBridgeCount: super::ULONG,
    pub AerEndPointCount: super::ULONG,
    pub GenericV1Count: super::ULONG,
    pub GenericV2Count: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PROCESS_HEST_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PROMOTED_AER_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub ErrorHandlerType: super::ULONG,
    pub ErrorSourceId: super::ULONG,
    pub RootErrorCommand: super::ULONG,
    pub RootErrorStatus: super::ULONG,
    pub DeviceAssociationBitmap: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PROMOTED_AER_ERROR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PSHED_INJECT_ERROR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorType: super::ULONG,
    pub Parameter1: super::ULONGLONG,
    pub Parameter2: super::ULONGLONG,
    pub Parameter3: super::ULONGLONG,
    pub Parameter4: super::ULONGLONG,
    pub InjectionStatus: windows_sys::core::NTSTATUS,
    pub InjectionAttempted: super::BOOLEAN,
    pub InjectionByPlugin: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PSHED_INJECT_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_PSHED_PLUGIN_REGISTER {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Version: super::ULONG,
    pub Length: super::ULONG,
    pub FunctionalAreaMask: super::ULONG,
    pub Status: windows_sys::core::NTSTATUS,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_PSHED_PLUGIN_REGISTER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct WHEAP_ROW_FAILURE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub LowOrderPage: super::PFN_NUMBER,
    pub HighOrderPage: super::PFN_NUMBER,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for WHEAP_ROW_FAILURE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "wdm"))]
#[derive(Clone, Copy)]
pub struct WHEAP_ROW_OFFLINE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub FirstPage: super::PFN_NUMBER,
    pub LastPage: super::PFN_NUMBER,
    pub Range: super::UINT32,
    pub Status: windows_sys::core::NTSTATUS,
    pub ErrorReason: WHEA_OFFLINE_ERRS,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
impl Default for WHEAP_ROW_OFFLINE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_SPURIOUS_AER_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub ErrorHandlerType: super::ULONG,
    pub SpuriousErrorSourceId: super::ULONG,
    pub RootErrorCommand: super::ULONG,
    pub RootErrorStatus: super::ULONG,
    pub DeviceAssociationBitmap: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_SPURIOUS_AER_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_STARTED_REPORT_HW_ERROR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorPacket: PWHEA_ERROR_PACKET,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_STARTED_REPORT_HW_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEAP_STUCK_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub EpIndex: super::ULONG,
    pub Bank: super::ULONG,
    pub MciStatus: super::ULONG64,
}
#[cfg(feature = "shared")]
impl Default for WHEAP_STUCK_ERROR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_ACPI_HEADER {
    pub Signature: super::UINT32,
    pub Length: super::UINT32,
    pub Revision: super::UINT8,
    pub Checksum: super::UINT8,
    pub OemId: [super::UINT8; 6],
    pub OemTableId: super::UINT64,
    pub OemRevision: super::UINT32,
    pub CreatorId: super::UINT32,
    pub CreatorRevision: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ACPI_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_AER_BRIDGE_DESCRIPTOR {
    pub Type: super::USHORT,
    pub Enabled: super::BOOLEAN,
    pub Reserved: super::UCHAR,
    pub BusNumber: super::ULONG,
    pub Slot: WHEA_PCI_SLOT_NUMBER,
    pub DeviceControl: super::USHORT,
    pub Flags: AER_BRIDGE_DESCRIPTOR_FLAGS,
    pub UncorrectableErrorMask: super::ULONG,
    pub UncorrectableErrorSeverity: super::ULONG,
    pub CorrectableErrorMask: super::ULONG,
    pub AdvancedCapsAndControl: super::ULONG,
    pub SecondaryUncorrectableErrorMask: super::ULONG,
    pub SecondaryUncorrectableErrorSev: super::ULONG,
    pub SecondaryCapsAndControl: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_AER_BRIDGE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_AER_ENDPOINT_DESCRIPTOR {
    pub Type: super::USHORT,
    pub Enabled: super::BOOLEAN,
    pub Reserved: super::UCHAR,
    pub BusNumber: super::ULONG,
    pub Slot: WHEA_PCI_SLOT_NUMBER,
    pub DeviceControl: super::USHORT,
    pub Flags: AER_ENDPOINT_DESCRIPTOR_FLAGS,
    pub UncorrectableErrorMask: super::ULONG,
    pub UncorrectableErrorSeverity: super::ULONG,
    pub CorrectableErrorMask: super::ULONG,
    pub AdvancedCapsAndControl: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_AER_ENDPOINT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_AER_ROOTPORT_DESCRIPTOR {
    pub Type: super::USHORT,
    pub Enabled: super::BOOLEAN,
    pub Reserved: super::UCHAR,
    pub BusNumber: super::ULONG,
    pub Slot: WHEA_PCI_SLOT_NUMBER,
    pub DeviceControl: super::USHORT,
    pub Flags: AER_ROOTPORT_DESCRIPTOR_FLAGS,
    pub UncorrectableErrorMask: super::ULONG,
    pub UncorrectableErrorSeverity: super::ULONG,
    pub CorrectableErrorMask: super::ULONG,
    pub AdvancedCapsAndControl: super::ULONG,
    pub RootErrorCommand: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_AER_ROOTPORT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_AMD_EXTENDED_REGISTERS {
    pub IPID: super::ULONGLONG,
    pub SYND: super::ULONGLONG,
    pub CONFIG: super::ULONGLONG,
    pub DESTAT: super::ULONGLONG,
    pub DEADDR: super::ULONGLONG,
    pub MISC1: super::ULONGLONG,
    pub MISC2: super::ULONGLONG,
    pub MISC3: super::ULONGLONG,
    pub MISC4: super::ULONGLONG,
    pub RasCap: super::ULONGLONG,
    pub Reserved: [super::ULONGLONG; 14],
}
#[cfg(feature = "shared")]
impl Default for WHEA_AMD_EXTENDED_REGISTERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_AMD_EXT_REG_NUM: i32 = 10;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARMV8_AARCH32_GPRS {
    pub R0: super::ULONG,
    pub R1: super::ULONG,
    pub R2: super::ULONG,
    pub R3: super::ULONG,
    pub R4: super::ULONG,
    pub R5: super::ULONG,
    pub R6: super::ULONG,
    pub R7: super::ULONG,
    pub R8: super::ULONG,
    pub R9: super::ULONG,
    pub R10: super::ULONG,
    pub R11: super::ULONG,
    pub R12: super::ULONG,
    pub R13: super::ULONG,
    pub R14: super::ULONG,
    pub R15: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARMV8_AARCH64_EL3_CSR {
    pub ELR_EL3: super::ULONGLONG,
    pub ESR_EL3: super::ULONGLONG,
    pub FAR_EL3: super::ULONGLONG,
    pub MAIR_EL3: super::ULONGLONG,
    pub SCTLR_EL3: super::ULONGLONG,
    pub SP_EL3: super::ULONGLONG,
    pub SPSR_EL3: super::ULONGLONG,
    pub TCR_EL3: super::ULONGLONG,
    pub TPIDR_EL3: super::ULONGLONG,
    pub TTBR0_EL3: super::ULONGLONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARMV8_AARCH64_GPRS {
    pub X0: super::ULONGLONG,
    pub X1: super::ULONGLONG,
    pub X2: super::ULONGLONG,
    pub X3: super::ULONGLONG,
    pub X4: super::ULONGLONG,
    pub X5: super::ULONGLONG,
    pub X6: super::ULONGLONG,
    pub X7: super::ULONGLONG,
    pub X8: super::ULONGLONG,
    pub X9: super::ULONGLONG,
    pub X10: super::ULONGLONG,
    pub X11: super::ULONGLONG,
    pub X12: super::ULONGLONG,
    pub X13: super::ULONGLONG,
    pub X14: super::ULONGLONG,
    pub X15: super::ULONGLONG,
    pub X16: super::ULONGLONG,
    pub X17: super::ULONGLONG,
    pub X18: super::ULONGLONG,
    pub X19: super::ULONGLONG,
    pub X20: super::ULONGLONG,
    pub X21: super::ULONGLONG,
    pub X22: super::ULONGLONG,
    pub X23: super::ULONGLONG,
    pub X24: super::ULONGLONG,
    pub X25: super::ULONGLONG,
    pub X26: super::ULONGLONG,
    pub X27: super::ULONGLONG,
    pub X28: super::ULONGLONG,
    pub X29: super::ULONGLONG,
    pub X30: super::ULONGLONG,
    pub SP: super::ULONGLONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_AARCH32_EL1_CSR {
    pub DFAR: super::ULONG,
    pub DFSR: super::ULONG,
    pub IFAR: super::ULONG,
    pub ISR: super::ULONG,
    pub MAIR0: super::ULONG,
    pub MAIR1: super::ULONG,
    pub MIDR: super::ULONG,
    pub MPIDR: super::ULONG,
    pub NMRR: super::ULONG,
    pub PRRR: super::ULONG,
    pub SCTLR: super::ULONG,
    pub SPSR: super::ULONG,
    pub SPSR_abt: super::ULONG,
    pub SPSR_fiq: super::ULONG,
    pub SPSR_irq: super::ULONG,
    pub SPSR_svc: super::ULONG,
    pub SPSR_und: super::ULONG,
    pub TPIDRPRW: super::ULONG,
    pub TPIDRURO: super::ULONG,
    pub TPIDRURW: super::ULONG,
    pub TTBCR: super::ULONG,
    pub TTBR0: super::ULONG,
    pub TTBR1: super::ULONG,
    pub DACR: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_AARCH32_EL2_CSR {
    pub ELR_hyp: super::ULONG,
    pub HAMAIR0: super::ULONG,
    pub HAMAIR1: super::ULONG,
    pub HCR: super::ULONG,
    pub HCR2: super::ULONG,
    pub HDFAR: super::ULONG,
    pub HIFAR: super::ULONG,
    pub HPFAR: super::ULONG,
    pub HSR: super::ULONG,
    pub HTCR: super::ULONG,
    pub HTPIDR: super::ULONG,
    pub HTTBR: super::ULONG,
    pub SPSR_hyp: super::ULONG,
    pub VTCR: super::ULONG,
    pub VTTBR: super::ULONG,
    pub DACR32_EL2: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_AARCH32_SECURE_CSR {
    pub SCTLR: super::ULONG,
    pub SPSR_mon: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_AARCH64_EL1_CSR {
    pub ELR_EL1: super::ULONGLONG,
    pub ESR_EL2: super::ULONGLONG,
    pub FAR_EL1: super::ULONGLONG,
    pub ISR_EL1: super::ULONGLONG,
    pub MAIR_EL1: super::ULONGLONG,
    pub MIDR_EL1: super::ULONGLONG,
    pub MPIDR_EL1: super::ULONGLONG,
    pub SCTLR_EL1: super::ULONGLONG,
    pub SP_EL0: super::ULONGLONG,
    pub SP_EL1: super::ULONGLONG,
    pub SPSR_EL1: super::ULONGLONG,
    pub TCR_EL1: super::ULONGLONG,
    pub TPIDR_EL0: super::ULONGLONG,
    pub TPIDR_EL1: super::ULONGLONG,
    pub TPIDRRO_EL0: super::ULONGLONG,
    pub TTBR0_EL1: super::ULONGLONG,
    pub TTBR1_EL1: super::ULONGLONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_AARCH64_EL2_CSR {
    pub ELR_EL2: super::ULONGLONG,
    pub ESR_EL2: super::ULONGLONG,
    pub FAR_EL2: super::ULONGLONG,
    pub HACR_EL2: super::ULONGLONG,
    pub HCR_EL2: super::ULONGLONG,
    pub HPFAR_EL2: super::ULONGLONG,
    pub MAIR_EL2: super::ULONGLONG,
    pub SCTLR_EL2: super::ULONGLONG,
    pub SP_EL2: super::ULONGLONG,
    pub SPSR_EL2: super::ULONGLONG,
    pub TCR_EL2: super::ULONGLONG,
    pub TPIDR_EL2: super::ULONGLONG,
    pub TTBR0_EL2: super::ULONGLONG,
    pub VTCR_EL2: super::ULONGLONG,
    pub VTTBR_EL2: super::ULONGLONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
impl Default for WHEA_ARM_BUS_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_ARM_BUS_ERROR_VALID_BITS {
    pub Anonymous: WHEA_ARM_BUS_ERROR_VALID_BITS_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ARM_BUS_ERROR_VALID_BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_BUS_ERROR_VALID_BITS_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_CACHE_ERROR {
    pub ValidationBit: WHEA_ARM_CACHE_ERROR_VALID_BITS,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u64,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ARM_CACHE_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_ARM_CACHE_ERROR_VALID_BITS {
    pub Anonymous: WHEA_ARM_CACHE_ERROR_VALID_BITS_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ARM_CACHE_ERROR_VALID_BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_CACHE_ERROR_VALID_BITS_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_MISC_CSR {
    pub MRSEncoding: super::USHORT,
    pub Value: super::ULONGLONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_ARM_PROCESSOR_ERROR {
    pub CacheError: WHEA_ARM_CACHE_ERROR,
    pub TlbError: WHEA_ARM_TLB_ERROR,
    pub BusError: WHEA_ARM_BUS_ERROR,
    pub AsULONGLONG: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ARM_PROCESSOR_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER {
    pub Version: super::USHORT,
    pub RegisterContextType: super::USHORT,
    pub RegisterArraySize: super::ULONG,
    pub RegisterArray: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS {
    pub Anonymous: WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_PROCESSOR_ERROR_INFORMATION {
    pub Version: super::UCHAR,
    pub Length: super::UCHAR,
    pub ValidationBit: WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS,
    pub Type: super::UCHAR,
    pub MultipleError: super::USHORT,
    pub Flags: super::UCHAR,
    pub ErrorInformation: super::ULONGLONG,
    pub VirtualFaultAddress: super::ULONGLONG,
    pub PhysicalFaultAddress: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ARM_PROCESSOR_ERROR_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS {
    pub Anonymous: WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_PROCESSOR_ERROR_SECTION {
    pub ValidBits: WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS,
    pub ErrorInformationStructures: super::USHORT,
    pub ContextInformationStructures: super::USHORT,
    pub SectionLength: super::ULONG,
    pub ErrorAffinityLevel: super::UCHAR,
    pub Reserved: [super::UCHAR; 3],
    pub MPIDR_EL1: super::ULONGLONG,
    pub MIDR_EL1: super::ULONGLONG,
    pub RunningState: super::ULONG,
    pub PSCIState: super::ULONG,
    pub Data: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_ARM_PROCESSOR_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS {
    pub Anonymous: WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS_0 {
    pub _bitfield: u32,
}
pub const WHEA_ARM_RAS_NODE_FIELD_COUNT: i32 = 8;
pub type WHEA_ARM_RAS_NODE_INTERFACES = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_RAS_NODE_SECTION {
    pub NodeFieldCount: super::UINT32,
    pub NodeIndex: super::UINT32,
    pub InterfaceType: super::UINT8,
    pub AestNodeType: super::UINT8,
    pub Reserved: [super::UINT8; 6],
    pub ErrFr: super::UINT64,
    pub ErrCtlr: super::UINT64,
    pub ErrStatus: super::UINT64,
    pub ErrAddr: super::UINT64,
    pub ErrMisc0: super::UINT64,
    pub ErrMisc1: super::UINT64,
    pub ErrMisc2: super::UINT64,
    pub ErrMisc3: super::UINT64,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ARM_RAS_NODE_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_TLB_ERROR {
    pub ValidationBit: WHEA_ARM_TLB_ERROR_VALID_BITS,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u64,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ARM_TLB_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_ARM_TLB_ERROR_VALID_BITS {
    pub Anonymous: WHEA_ARM_TLB_ERROR_VALID_BITS_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ARM_TLB_ERROR_VALID_BITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ARM_TLB_ERROR_VALID_BITS_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_AZCC_ROOT_BUS_ERR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub MaxBusCountPassed: super::BOOLEAN,
    pub InvalidBusMSR: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for WHEA_AZCC_ROOT_BUS_ERR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_AZCC_ROOT_BUS_LIST_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RootBusCount: super::UINT32,
    pub RootBuses: [super::UINT32; 8],
}
#[cfg(feature = "shared")]
impl Default for WHEA_AZCC_ROOT_BUS_LIST_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_AZCC_SET_POISON_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Bus: super::UINT32,
    pub ReadSuccess: super::BOOLEAN,
    pub WriteSuccess: super::BOOLEAN,
    pub IsEnable: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for WHEA_AZCC_SET_POISON_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_BAD_PAGE_LIST_LOCATION: i32 = 15;
pub const WHEA_BAD_PAGE_LIST_MAX_SIZE: i32 = 14;
pub type WHEA_BUGCHECK_RECOVERY_LOG_TYPE = i32;
pub const WHEA_BUSCHECK_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1cf3f8b3_c5b1_49a2_aa59_5eef92ffa63c);
pub const WHEA_CACHECHECK_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa55701f5_e3ef_43de_ac72_249b573fad2c);
pub const WHEA_CMCI_THRESHOLD_COUNT: i32 = 10;
pub const WHEA_CMCI_THRESHOLD_POLL_COUNT: i32 = 12;
pub const WHEA_CMCI_THRESHOLD_TIME: i32 = 11;
pub type WHEA_CPU_VENDOR = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_CRASHDUMP_EVENT_LOG_ENTRY_ULONG1 {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Value: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_CRASHDUMP_EVENT_LOG_ENTRY_ULONG1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_CRASHDUMP_EVENT_LOG_ENTRY_WITH_STATUS {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub SourceLocationId: super::ULONG,
    pub Status: windows_sys::core::NTSTATUS,
    pub IOStatus: windows_sys::core::NTSTATUS,
}
#[cfg(feature = "shared")]
impl Default for WHEA_CRASHDUMP_EVENT_LOG_ENTRY_WITH_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_CXL_AGENT_ADDRES {
    pub Anonymous: WHEA_CXL_AGENT_ADDRES_0,
    pub Anonymous2: WHEA_CXL_AGENT_ADDRES_1,
}
#[cfg(feature = "shared")]
impl Default for WHEA_CXL_AGENT_ADDRES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_CXL_AGENT_ADDRES_0 {
    pub FunctionNumber: super::UCHAR,
    pub DeviceNumber: super::UCHAR,
    pub BusNumber: super::UCHAR,
    pub SegmentNumber: super::USHORT,
    pub Reserved: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl Default for WHEA_CXL_AGENT_ADDRES_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_CXL_AGENT_ADDRES_1 {
    pub CxlPortRcrbBaseAddress: [super::UCHAR; 8],
}
#[cfg(feature = "shared")]
impl Default for WHEA_CXL_AGENT_ADDRES_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_CXL_AGENT_TYPE = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_CXL_COMPONENT_EVENTS_SECTION {
    pub Length: super::ULONG,
    pub ValidBits: WHEA_CXL_COMPONENT_EVENTS_SECTION_VALIDBITS,
    pub DeviceID: WHEA_CXL_PCIE_DEVICE_ID,
    pub DeviceSerialNumber: WHEA_CXL_DEVICE_SERIAL_NUMBER,
    pub CxlComponentEventLog: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_CXL_COMPONENT_EVENTS_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_CXL_COMPONENT_EVENTS_SECTION_VALIDBITS {
    pub Anonymous: WHEA_CXL_COMPONENT_EVENTS_SECTION_VALIDBITS_0,
    pub ValidBits: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_CXL_COMPONENT_EVENTS_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_CXL_COMPONENT_EVENTS_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_CXL_DEVICE_ID {
    pub VendorID: super::USHORT,
    pub DeviceID: super::USHORT,
    pub SubsystemVendorID: super::USHORT,
    pub SubsystemDeviceID: super::USHORT,
    pub ClassCode: super::USHORT,
    pub Anonymous: WHEA_CXL_DEVICE_ID_0,
    pub Reserved2: [super::UCHAR; 4],
}
#[cfg(feature = "shared")]
impl Default for WHEA_CXL_DEVICE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_CXL_DEVICE_ID_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_CXL_DEVICE_SERIAL_NUMBER {
    pub Anonymous: WHEA_CXL_DEVICE_SERIAL_NUMBER_0,
    pub AsUlonglong: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_CXL_DEVICE_SERIAL_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_CXL_DEVICE_SERIAL_NUMBER_0 {
    pub CxlDeviceSerialNumberLowerDW: super::ULONG,
    pub CxlDeviceSerialNumberUpperDW: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_CXL_PCIE_DEVICE_ID {
    pub VendorID: super::USHORT,
    pub DeviceID: super::USHORT,
    pub FunctionNumber: super::UCHAR,
    pub DeviceNumber: super::UCHAR,
    pub BusNumber: super::UCHAR,
    pub SegmentNumber: super::USHORT,
    pub Anonymous: WHEA_CXL_PCIE_DEVICE_ID_0,
    pub Reserved2: super::UCHAR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_CXL_PCIE_DEVICE_ID_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_CXL_PROTOCOL_ERROR_SECTION {
    pub ValidBits: WHEA_CXL_PROTOCOL_ERROR_SECTION_VALIDBITS,
    pub CxlAgentType: super::UCHAR,
    pub Reserved: [super::UCHAR; 7],
    pub CxlAgentAddress: WHEA_CXL_AGENT_ADDRES,
    pub DeviceID: WHEA_CXL_DEVICE_ID,
    pub DeviceSerialNumber: WHEA_CXL_DEVICE_SERIAL_NUMBER,
    pub CapabilityStructure: [super::UCHAR; 60],
    pub CxlDvsecLength: super::USHORT,
    pub CxlErrorLogLength: super::USHORT,
    pub Reserved2: [super::UCHAR; 4],
    pub CxlDvsecAndErrorLog: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_CXL_PROTOCOL_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_CXL_PROTOCOL_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_CXL_PROTOCOL_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_CXL_PROTOCOL_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_CXL_PROTOCOL_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_MAX: i32 = 1;
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_MIN: i32 = 1;
pub const WHEA_DEVICE_DRIVER_BUFFER_SET_V1: i32 = 1;
pub const WHEA_DEVICE_DRIVER_CONFIG_MAX: i32 = 2;
pub const WHEA_DEVICE_DRIVER_CONFIG_MIN: i32 = 1;
pub const WHEA_DEVICE_DRIVER_CONFIG_V1: i32 = 1;
pub const WHEA_DEVICE_DRIVER_CONFIG_V2: i32 = 2;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_DEVICE_DRIVER_DESCRIPTOR {
    pub Type: super::USHORT,
    pub Enabled: super::BOOLEAN,
    pub Reserved: super::UCHAR,
    pub SourceGuid: windows_sys::core::GUID,
    pub LogTag: super::USHORT,
    pub Reserved2: super::USHORT,
    pub PacketLength: super::ULONG,
    pub PacketCount: super::ULONG,
    pub PacketBuffer: super::PUCHAR,
    pub Config: WHEA_ERROR_SOURCE_CONFIGURATION_DD,
    pub CreatorId: windows_sys::core::GUID,
    pub PartitionId: windows_sys::core::GUID,
    pub MaxSectionDataLength: super::ULONG,
    pub MaxSectionsPerRecord: super::ULONG,
    pub PacketStateBuffer: super::PUCHAR,
    pub OpenHandles: super::LONG,
}
pub const WHEA_DEVICE_ERROR_SUMMARY_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x990b31e9_541a_4db0_a42f_837d344f6923);
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_DEVICE_INFO {
    pub CXL: WHEA_DEVICE_INFO_0,
    pub AsUCHAR: [super::UCHAR; 72],
}
#[cfg(feature = "shared")]
impl Default for WHEA_DEVICE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_DEVICE_INFO_0 {
    pub DeviceID: WHEA_CXL_PCIE_DEVICE_ID,
    pub DeviceSerialNumber: WHEA_CXL_DEVICE_SERIAL_NUMBER,
}
#[cfg(feature = "shared")]
impl Default for WHEA_DEVICE_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_DEVICE_TYPE = i32;
pub const WHEA_DISABLE_DUMMY_WRITE: i32 = 6;
pub const WHEA_DISABLE_OFFLINE: i32 = 0;
pub const WHEA_DISABLE_PRM_ADDRESS_TRANSLATION: i32 = 20;
pub const WHEA_DPC_CAPABILITY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xec49534b_30e7_4358_972f_eca6958fae3b);
#[repr(C, packed(1))]
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_DRIVER_BUFFER_SET {
    pub Version: super::ULONG,
    pub Data: super::PUCHAR,
    pub DataSize: super::ULONG,
    pub SectionTypeGuid: super::LPGUID,
    pub SectionFriendlyName: super::PUCHAR,
    pub Flags: super::PUCHAR,
}
pub const WHEA_ENABLE_BATCHED_ROW_OFFLINE: i32 = 21;
#[cfg(feature = "winnt")]
pub type WHEA_ERROR_HANDLE = super::PVOID;
pub const WHEA_ERROR_HANDLE_INVALID: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_INJECTION_CAPABILITIES {
    pub Anonymous: WHEA_ERROR_INJECTION_CAPABILITIES_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ERROR_INJECTION_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_INJECTION_CAPABILITIES_0 {
    pub _bitfield: u32,
}
pub const WHEA_ERROR_LOG_ENTRY_ACPI: i32 = 1229996865;
pub const WHEA_ERROR_LOG_ENTRY_HAL: i32 = 541868360;
pub const WHEA_ERROR_LOG_ENTRY_HYPERV: i32 = 1347836232;
pub const WHEA_ERROR_LOG_ENTRY_KERNEL: i32 = 1280201291;
pub const WHEA_ERROR_LOG_ENTRY_PCI: i32 = 541672272;
pub const WHEA_ERROR_LOG_ENTRY_PSHED: i32 = 1145590608;
pub const WHEA_ERROR_LOG_ENTRY_PSHED_PI: i32 = 1230000976;
pub const WHEA_ERROR_LOG_ENTRY_SIGNATURE: i32 = 1733060695;
pub const WHEA_ERROR_LOG_ENTRY_VERSION: i32 = 1;
#[cfg(feature = "shared")]
pub type WHEA_ERROR_PACKET = WHEA_ERROR_PACKET_V2;
pub type WHEA_ERROR_PACKET_DATA_FORMAT = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_PACKET_FLAGS {
    pub Anonymous: WHEA_ERROR_PACKET_FLAGS_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ERROR_PACKET_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_PACKET_FLAGS_0 {
    pub _bitfield: u32,
}
pub const WHEA_ERROR_PACKET_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe71254e9_c1b9_4940_ab76_909703a4320f);
pub const WHEA_ERROR_PACKET_SIGNATURE: i32 = 1095059543;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_PACKET_V1 {
    pub Signature: super::ULONG,
    pub Flags: WHEA_ERROR_PACKET_FLAGS,
    pub Size: super::ULONG,
    pub RawDataLength: super::ULONG,
    pub Reserved1: super::ULONGLONG,
    pub Context: super::ULONGLONG,
    pub ErrorType: WHEA_ERROR_TYPE,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub ErrorSourceId: super::ULONG,
    pub ErrorSourceType: WHEA_ERROR_SOURCE_TYPE,
    pub Reserved2: super::ULONG,
    pub Version: super::ULONG,
    pub Cpu: super::ULONGLONG,
    pub u: WHEA_ERROR_PACKET_V1_0,
    pub RawDataFormat: WHEA_RAW_DATA_FORMAT,
    pub RawDataOffset: super::ULONG,
    pub RawData: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_ERROR_PACKET_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
impl Default for WHEA_ERROR_PACKET_V1_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_ERROR_PACKET_V1_SIGNATURE: i32 = 1951429189;
pub const WHEA_ERROR_PACKET_V1_VERSION: i32 = 2;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_PACKET_V2 {
    pub Signature: super::ULONG,
    pub Version: super::ULONG,
    pub Length: super::ULONG,
    pub Flags: WHEA_ERROR_PACKET_FLAGS,
    pub ErrorType: WHEA_ERROR_TYPE,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub ErrorSourceId: super::ULONG,
    pub ErrorSourceType: WHEA_ERROR_SOURCE_TYPE,
    pub NotifyType: windows_sys::core::GUID,
    pub Context: super::ULONGLONG,
    pub DataFormat: WHEA_ERROR_PACKET_DATA_FORMAT,
    pub Reserved1: super::ULONG,
    pub DataOffset: super::ULONG,
    pub DataLength: super::ULONG,
    pub PshedDataOffset: super::ULONG,
    pub PshedDataLength: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ERROR_PACKET_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_ERROR_PACKET_V2_SIGNATURE: i32 = 1095059543;
pub const WHEA_ERROR_PACKET_V2_VERSION: i32 = 3;
pub const WHEA_ERROR_PACKET_VERSION: i32 = 3;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD {
    pub Header: WHEA_ERROR_RECORD_HEADER,
    pub SectionDescriptor: [WHEA_ERROR_RECORD_SECTION_DESCRIPTOR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_ERROR_RECORD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_ERROR_RECORD_FLAGS_DEVICE_DRIVER: i32 = 8;
pub const WHEA_ERROR_RECORD_FLAGS_PREVIOUSERROR: i32 = 2;
pub const WHEA_ERROR_RECORD_FLAGS_RECOVERED: i32 = 1;
pub const WHEA_ERROR_RECORD_FLAGS_SIMULATED: i32 = 4;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD_HEADER {
    pub Signature: super::ULONG,
    pub Revision: WHEA_REVISION,
    pub SignatureEnd: super::ULONG,
    pub SectionCount: super::USHORT,
    pub Severity: WHEA_ERROR_SEVERITY,
    pub ValidBits: WHEA_ERROR_RECORD_HEADER_VALIDBITS,
    pub Length: super::ULONG,
    pub Timestamp: WHEA_TIMESTAMP,
    pub PlatformId: windows_sys::core::GUID,
    pub PartitionId: windows_sys::core::GUID,
    pub CreatorId: windows_sys::core::GUID,
    pub NotifyType: windows_sys::core::GUID,
    pub RecordId: super::ULONGLONG,
    pub Flags: WHEA_ERROR_RECORD_HEADER_FLAGS,
    pub PersistenceInfo: WHEA_PERSISTENCE_INFO,
    pub Anonymous: WHEA_ERROR_RECORD_HEADER_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_ERROR_RECORD_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_HEADER_0 {
    pub Anonymous: WHEA_ERROR_RECORD_HEADER_0_0,
    pub Reserved: [super::UCHAR; 12],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_ERROR_RECORD_HEADER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD_HEADER_0_0 {
    pub OsBuildNumber: super::ULONG,
    pub Reserved2: [super::UCHAR; 8],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_ERROR_RECORD_HEADER_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_HEADER_FLAGS {
    pub Anonymous: WHEA_ERROR_RECORD_HEADER_FLAGS_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ERROR_RECORD_HEADER_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_RECORD_HEADER_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_HEADER_VALIDBITS {
    pub Anonymous: WHEA_ERROR_RECORD_HEADER_VALIDBITS_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ERROR_RECORD_HEADER_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_RECORD_HEADER_VALIDBITS_0 {
    pub _bitfield: u32,
}
pub const WHEA_ERROR_RECORD_REVISION: i32 = 528;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD_SECTION_DESCRIPTOR {
    pub SectionOffset: super::ULONG,
    pub SectionLength: super::ULONG,
    pub Revision: WHEA_REVISION,
    pub ValidBits: WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS,
    pub Reserved: super::UCHAR,
    pub Flags: WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS,
    pub SectionType: windows_sys::core::GUID,
    pub FRUId: windows_sys::core::GUID,
    pub SectionSeverity: WHEA_ERROR_SEVERITY,
    pub FRUText: [super::CCHAR; 20],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_ERROR_RECORD_SECTION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS {
    pub Anonymous: WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS_0 {
    pub _bitfield: u32,
}
pub const WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_REVISION: i32 = 768;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS {
    pub Anonymous: WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS_0,
    pub AsUCHAR: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS_0 {
    pub _bitfield: u8,
}
pub const WHEA_ERROR_RECORD_SIGNATURE: i32 = 1380274243;
pub const WHEA_ERROR_RECORD_SIGNATURE_END: u32 = 4294967295;
pub const WHEA_ERROR_RECORD_VALID_PARTITIONID: i32 = 4;
pub const WHEA_ERROR_RECORD_VALID_PLATFORMID: i32 = 1;
pub const WHEA_ERROR_RECORD_VALID_TIMESTAMP: i32 = 2;
#[repr(C, packed(1))]
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECOVERY_INFO_SECTION {
    pub RecoveryKernel: super::BOOLEAN,
    pub RecoveryAction: WHEA_RECOVERY_ACTION,
    pub RecoveryType: WHEA_RECOVERY_TYPE,
    pub Irql: super::KIRQL,
    pub RecoverySucceeded: super::BOOLEAN,
    pub FailureReason: WHEA_RECOVERY_FAILURE_REASON,
    pub ProcessName: [super::CCHAR; 20],
}
#[cfg(all(feature = "ntdef", feature = "shared", feature = "winnt"))]
impl Default for WHEA_ERROR_RECOVERY_INFO_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_ERROR_SEVERITY = i32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION {
    pub Flags: super::ULONG,
    pub Correct: WHEA_ERROR_SOURCE_CORRECT,
    pub Initialize: WHEA_ERROR_SOURCE_INITIALIZE,
    pub CreateRecord: WHEA_ERROR_SOURCE_CREATE_RECORD,
    pub Recover: WHEA_ERROR_SOURCE_RECOVER,
    pub Uninitialize: WHEA_ERROR_SOURCE_UNINITIALIZE,
    pub Reserved: super::PVOID,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DD {
    pub Initialize: WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER,
    pub Uninitialize: WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER,
    pub Correct: WHEA_ERROR_SOURCE_CORRECT_DEVICE_DRIVER,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER {
    pub Version: super::ULONG,
    pub SourceGuid: windows_sys::core::GUID,
    pub LogTag: super::USHORT,
    pub Reserved: [super::UCHAR; 6],
    pub Initialize: WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER,
    pub Uninitialize: WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER,
    pub MaxSectionDataLength: super::ULONG,
    pub MaxSectionsPerReport: super::ULONG,
    pub CreatorId: windows_sys::core::GUID,
    pub PartitionId: windows_sys::core::GUID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1 {
    pub Version: super::ULONG,
    pub SourceGuid: windows_sys::core::GUID,
    pub LogTag: super::USHORT,
    pub Reserved: [super::UCHAR; 6],
    pub Initialize: WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER,
    pub Uninitialize: WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type WHEA_ERROR_SOURCE_CORRECT = Option<unsafe extern "system" fn(errorsource: PWHEA_ERROR_SOURCE_DESCRIPTOR, maximumsectionlength: super::PULONG) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type WHEA_ERROR_SOURCE_CORRECT_DEVICE_DRIVER = Option<unsafe extern "system" fn(errorsourcedesc: super::PVOID, maximumsectionlength: super::PULONG) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type WHEA_ERROR_SOURCE_CREATE_RECORD = Option<unsafe extern "system" fn(errorsource: PWHEA_ERROR_SOURCE_DESCRIPTOR, errorpacket: PWHEA_ERROR_PACKET, errorrecord: PWHEA_ERROR_RECORD, buffersize: super::ULONG, context: super::PVOID) -> windows_sys::core::NTSTATUS>;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_SOURCE_DESCRIPTOR {
    pub Length: super::ULONG,
    pub Version: super::ULONG,
    pub Type: WHEA_ERROR_SOURCE_TYPE,
    pub State: WHEA_ERROR_SOURCE_STATE,
    pub MaxRawDataLength: super::ULONG,
    pub NumRecordsToPreallocate: super::ULONG,
    pub MaxSectionsPerRecord: super::ULONG,
    pub ErrorSourceId: super::ULONG,
    pub PlatformErrorSourceId: super::ULONG,
    pub Flags: super::ULONG,
    pub Info: WHEA_ERROR_SOURCE_DESCRIPTOR_0,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for WHEA_ERROR_SOURCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for WHEA_ERROR_SOURCE_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERBRIDGE: i32 = 8;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERENDPOINT: i32 = 7;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_AERROOTPORT: i32 = 6;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_GENERIC: i32 = 9;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_GENERIC_V2: i32 = 10;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFCMC: i32 = 4;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFCPE: i32 = 5;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_IPFMCA: i32 = 3;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFCMC: i32 = 1;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFMCE: i32 = 0;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_TYPE_XPFNMI: i32 = 2;
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_SOURCE_DESCRIPTOR_V2 {
    pub Length: super::ULONG,
    pub Version: super::ULONG,
    pub Type: WHEA_ERROR_SOURCE_TYPE,
    pub State: WHEA_ERROR_SOURCE_STATE,
    pub MaxRawDataLength: super::ULONG,
    pub NumRecordsToPreallocate: super::ULONG,
    pub MaxSectionsPerRecord: super::ULONG,
    pub ErrorSourceId: super::ULONG,
    pub PlatformErrorSourceId: super::ULONG,
    pub Flags: super::ULONG,
    pub Info: WHEA_ERROR_SOURCE_DESCRIPTOR_V2_0,
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for WHEA_ERROR_SOURCE_DESCRIPTOR_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
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
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for WHEA_ERROR_SOURCE_DESCRIPTOR_V2_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_VERSION_10: i32 = 10;
pub const WHEA_ERROR_SOURCE_DESCRIPTOR_VERSION_11: i32 = 11;
pub const WHEA_ERROR_SOURCE_FLAG_DEFAULTSOURCE: u32 = 2147483648;
pub const WHEA_ERROR_SOURCE_FLAG_FIRMWAREFIRST: i32 = 1;
pub const WHEA_ERROR_SOURCE_FLAG_GHES_ASSIST: i32 = 4;
pub const WHEA_ERROR_SOURCE_FLAG_GLOBAL: i32 = 2;
pub const WHEA_ERROR_SOURCE_FLAG_V2_DESCRIPTOR: i32 = 8;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type WHEA_ERROR_SOURCE_INITIALIZE = Option<unsafe extern "system" fn(phase: super::ULONG, errorsource: PWHEA_ERROR_SOURCE_DESCRIPTOR, context: super::PVOID) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER = Option<unsafe extern "system" fn(context: super::PVOID, errorsourceid: super::ULONG) -> windows_sys::core::NTSTATUS>;
pub const WHEA_ERROR_SOURCE_INVALID_RELATED_SOURCE: i32 = 65535;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_SOURCE_OVERRIDE_SETTINGS {
    pub Type: WHEA_ERROR_SOURCE_TYPE,
    pub MaxRawDataLength: super::ULONG,
    pub NumRecordsToPreallocate: super::ULONG,
    pub MaxSectionsPerRecord: super::ULONG,
}
#[cfg(feature = "winnt")]
pub type WHEA_ERROR_SOURCE_RECOVER = Option<unsafe extern "system" fn(recoverycontext: super::PVOID, severity: PWHEA_ERROR_SEVERITY) -> windows_sys::core::NTSTATUS>;
pub type WHEA_ERROR_SOURCE_STATE = i32;
pub type WHEA_ERROR_SOURCE_TYPE = i32;
#[cfg(feature = "winnt")]
pub type WHEA_ERROR_SOURCE_UNINITIALIZE = Option<unsafe extern "system" fn(context: super::PVOID)>;
#[cfg(feature = "winnt")]
pub type WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER = Option<unsafe extern "system" fn(context: super::PVOID)>;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_STATUS {
    pub ErrorStatus: super::ULONGLONG,
    pub Anonymous: WHEA_ERROR_STATUS_0,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ERROR_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_ERROR_STATUS_0 {
    pub _bitfield: u64,
}
pub const WHEA_ERROR_TEXT_LEN: i32 = 32;
pub type WHEA_ERROR_TYPE = i32;
pub const WHEA_ERR_SRC_OVERRIDE_FLAG: i32 = 1073741824;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_ETW_OVERFLOW_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RecordId: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_ETW_OVERFLOW_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_EVENT_LOG_ENTRY {
    pub Header: WHEA_EVENT_LOG_ENTRY_HEADER,
}
#[cfg(feature = "shared")]
impl Default for WHEA_EVENT_LOG_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_EVENT_LOG_ENTRY_FLAGS {
    pub Anonymous: WHEA_EVENT_LOG_ENTRY_FLAGS_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_EVENT_LOG_ENTRY_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_EVENT_LOG_ENTRY_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_EVENT_LOG_ENTRY_HEADER {
    pub Signature: super::ULONG,
    pub Version: super::ULONG,
    pub Length: super::ULONG,
    pub Type: WHEA_EVENT_LOG_ENTRY_TYPE,
    pub OwnerTag: super::ULONG,
    pub Id: WHEA_EVENT_LOG_ENTRY_ID,
    pub Flags: WHEA_EVENT_LOG_ENTRY_FLAGS,
    pub PayloadLength: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_EVENT_LOG_ENTRY_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_EVENT_LOG_ENTRY_ID = i32;
pub type WHEA_EVENT_LOG_ENTRY_TYPE = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_EXTENDED_RAS {
    pub Flag: super::UINT32,
    pub DimmLocation: super::UINT32,
    pub SectionLength: super::UINT64,
    pub SectionData: [super::UINT64; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_EXTENDED_RAS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_FAILED_ADD_DEFECT_LIST_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[cfg(feature = "shared")]
impl Default for WHEA_FAILED_ADD_DEFECT_LIST_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_FIRMWARE_ERROR_RECORD_REFERENCE {
    pub Type: super::UCHAR,
    pub Reserved: [super::UCHAR; 7],
    pub FirmwareRecordId: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_FIRMWARE_ERROR_RECORD_REFERENCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_FIRMWARE_RECORD_TYPE_IPFSAL: i32 = 0;
pub type WHEA_GAS_ERRORS = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_GAS_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Error: WHEA_GAS_ERRORS,
}
#[cfg(feature = "shared")]
impl Default for WHEA_GAS_ERROR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_GENERIC_ENTRY_TEXT_LEN: i32 = 20;
pub const WHEA_GENERIC_ENTRY_V2_VERSION: i32 = 768;
pub const WHEA_GENERIC_ENTRY_VERSION: i32 = 768;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_GENERIC_ERROR {
    pub BlockStatus: WHEA_GENERIC_ERROR_BLOCKSTATUS,
    pub RawDataOffset: super::ULONG,
    pub RawDataLength: super::ULONG,
    pub DataLength: super::ULONG,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub Data: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_GENERIC_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_GENERIC_ERROR_BLOCKSTATUS {
    pub Anonymous: WHEA_GENERIC_ERROR_BLOCKSTATUS_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_GENERIC_ERROR_BLOCKSTATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_GENERIC_ERROR_BLOCKSTATUS_0 {
    pub _bitfield: u32,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type WHEA_GENERIC_ERROR_DATA_ENTRY = WHEA_GENERIC_ERROR_DATA_ENTRY_V2;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_GENERIC_ERROR_DATA_ENTRY_V1 {
    pub SectionType: windows_sys::core::GUID,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub Revision: WHEA_REVISION,
    pub ValidBits: super::UCHAR,
    pub Flags: super::UCHAR,
    pub ErrorDataLength: super::ULONG,
    pub FRUId: windows_sys::core::GUID,
    pub FRUText: [super::UCHAR; 20],
    pub Data: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_GENERIC_ERROR_DATA_ENTRY_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_GENERIC_ERROR_DATA_ENTRY_V2 {
    pub SectionType: windows_sys::core::GUID,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub Revision: WHEA_REVISION,
    pub ValidBits: super::UCHAR,
    pub Flags: super::UCHAR,
    pub ErrorDataLength: super::ULONG,
    pub FRUId: windows_sys::core::GUID,
    pub FRUText: [super::UCHAR; 20],
    pub Timestamp: WHEA_TIMESTAMP,
    pub Data: [super::UCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_GENERIC_ERROR_DATA_ENTRY_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_GENERIC_ERROR_DESCRIPTOR {
    pub Type: super::USHORT,
    pub Reserved: super::UCHAR,
    pub Enabled: super::UCHAR,
    pub ErrStatusBlockLength: super::ULONG,
    pub RelatedErrorSourceId: super::ULONG,
    pub ErrStatusAddressSpaceID: super::UCHAR,
    pub ErrStatusAddressBitWidth: super::UCHAR,
    pub ErrStatusAddressBitOffset: super::UCHAR,
    pub ErrStatusAddressAccessSize: super::UCHAR,
    pub ErrStatusAddress: super::LARGE_INTEGER,
    pub Notify: WHEA_NOTIFICATION_DESCRIPTOR,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_GENERIC_ERROR_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_GENERIC_ERROR_DESCRIPTOR_V2 {
    pub Type: super::USHORT,
    pub Reserved: super::UCHAR,
    pub Enabled: super::UCHAR,
    pub ErrStatusBlockLength: super::ULONG,
    pub RelatedErrorSourceId: super::ULONG,
    pub ErrStatusAddressSpaceID: super::UCHAR,
    pub ErrStatusAddressBitWidth: super::UCHAR,
    pub ErrStatusAddressBitOffset: super::UCHAR,
    pub ErrStatusAddressAccessSize: super::UCHAR,
    pub ErrStatusAddress: super::LARGE_INTEGER,
    pub Notify: WHEA_NOTIFICATION_DESCRIPTOR,
    pub ReadAckAddressSpaceID: super::UCHAR,
    pub ReadAckAddressBitWidth: super::UCHAR,
    pub ReadAckAddressBitOffset: super::UCHAR,
    pub ReadAckAddressAccessSize: super::UCHAR,
    pub ReadAckAddress: super::LARGE_INTEGER,
    pub ReadAckPreserveMask: super::ULONGLONG,
    pub ReadAckWriteMask: super::ULONGLONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_GENERIC_ERROR_DESCRIPTOR_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_INJECTION_VENDOR_DEFINED_REASON_CODES = i32;
pub const WHEA_INVALID_ERR_SRC_ID: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_IN_USE_PAGE_NOTIFY_FLAGS {
    pub Bits: WHEA_IN_USE_PAGE_NOTIFY_FLAGS_0,
    pub AsUCHAR: super::UINT8,
}
#[cfg(feature = "shared")]
impl Default for WHEA_IN_USE_PAGE_NOTIFY_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_IN_USE_PAGE_NOTIFY_FLAGS_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_IPF_CMC_DESCRIPTOR {
    pub Type: super::USHORT,
    pub Enabled: super::UCHAR,
    pub Reserved: super::UCHAR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_IPF_CPE_DESCRIPTOR {
    pub Type: super::USHORT,
    pub Enabled: super::UCHAR,
    pub Reserved: super::UCHAR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_IPF_MCA_DESCRIPTOR {
    pub Type: super::USHORT,
    pub Enabled: super::UCHAR,
    pub Reserved: super::UCHAR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_IPMI_LOAD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub IsOnline: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for WHEA_IPMI_LOAD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_IPMI_SUBSCRIBE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub IsRegistered: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for WHEA_IPMI_SUBSCRIBE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_MAX_LOG_DATA_LEN: i32 = 36;
pub const WHEA_MAX_MC_BANKS: i32 = 32;
pub const WHEA_MAX_MC_BANKS64: i32 = 64;
pub const WHEA_MEMERRTYPE_INVALIDADDRESS: i32 = 10;
pub const WHEA_MEMERRTYPE_MASTERABORT: i32 = 6;
pub const WHEA_MEMERRTYPE_MEMORYSPARING: i32 = 12;
pub const WHEA_MEMERRTYPE_MIRRORBROKEN: i32 = 11;
pub const WHEA_MEMERRTYPE_MULTIBITECC: i32 = 3;
pub const WHEA_MEMERRTYPE_MULTISYMCHIPKILL: i32 = 5;
pub const WHEA_MEMERRTYPE_NOERROR: i32 = 1;
pub const WHEA_MEMERRTYPE_PARITYERROR: i32 = 8;
pub const WHEA_MEMERRTYPE_SINGLEBITECC: i32 = 2;
pub const WHEA_MEMERRTYPE_SINGLESYMCHIPKILL: i32 = 4;
pub const WHEA_MEMERRTYPE_TARGETABORT: i32 = 7;
pub const WHEA_MEMERRTYPE_UNKNOWN: i32 = 0;
pub const WHEA_MEMERRTYPE_WATCHDOGTIMEOUT: i32 = 9;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_CORRECTABLE_ERROR_DATA {
    pub ValidBits: WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS,
    pub SocketId: super::UINT32,
    pub ChannelId: super::UINT32,
    pub DimmSlot: super::UINT32,
    pub CorrectableErrorCount: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_CORRECTABLE_ERROR_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_CORRECTABLE_ERROR_HEADER {
    pub Version: super::UINT16,
    pub Count: super::UINT16,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_CORRECTABLE_ERROR_SECTION {
    pub Header: WHEA_MEMORY_CORRECTABLE_ERROR_HEADER,
    pub Data: [WHEA_MEMORY_CORRECTABLE_ERROR_DATA; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_CORRECTABLE_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
pub type WHEA_MEMORY_DEFINITION = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_ERROR_EXT_SECTION_AMD {
    pub Flags: WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS,
    pub ValidBits: WHEA_MEMORY_ERROR_EXT_SECTION_AMD_VALIDBITS,
    pub HardwareAddress: WHEA_MEMORY_HARDWARE_ADDRESS_AMD,
    pub Reserved: [super::UINT8; 40],
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_ERROR_EXT_SECTION_AMD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_MEMORY_ERROR_EXT_SECTION_AMD_VALIDBITS {
    pub Anonymous: WHEA_MEMORY_ERROR_EXT_SECTION_AMD_VALIDBITS_0,
    pub ValidBits: super::UINT64,
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_ERROR_EXT_SECTION_AMD_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_ERROR_EXT_SECTION_AMD_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS {
    pub Anonymous: WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS_0,
    pub AsUINT64: super::UINT64,
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_ERROR_EXT_SECTION_INTEL {
    pub Flags: WHEA_MEMORY_ERROR_EXT_SECTION_FLAGS,
    pub ValidBits: WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS,
    pub HardwareAddress: WHEA_MEMORY_HARDWARE_ADDRESS_INTEL,
    pub Reserved: [super::UINT8; 40],
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_ERROR_EXT_SECTION_INTEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS {
    pub Anonymous: WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS_0,
    pub ValidBits: super::UINT64,
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_ERROR_EXT_SECTION_INTEL_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_ERROR_SECTION {
    pub ValidBits: WHEA_MEMORY_ERROR_SECTION_VALIDBITS,
    pub ErrorStatus: WHEA_ERROR_STATUS,
    pub PhysicalAddress: super::ULONGLONG,
    pub PhysicalAddressMask: super::ULONGLONG,
    pub Node: super::USHORT,
    pub Card: super::USHORT,
    pub Module: super::USHORT,
    pub Bank: super::USHORT,
    pub Device: super::USHORT,
    pub Row: super::USHORT,
    pub Column: super::USHORT,
    pub BitPosition: super::USHORT,
    pub RequesterId: super::ULONGLONG,
    pub ResponderId: super::ULONGLONG,
    pub TargetId: super::ULONGLONG,
    pub ErrorType: super::UCHAR,
    pub Extended: super::UCHAR,
    pub RankNumber: super::USHORT,
    pub CardHandle: super::USHORT,
    pub ModuleHandle: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_MEMORY_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_MEMORY_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_HARDWARE_ADDRESS_AMD {
    pub SystemPhysicalAddress: super::UINT64,
    pub NormalizedAddress: super::UINT64,
    pub UmcBankInstanceId: super::UINT64,
    pub SocketNumber: super::UINT8,
    pub ChipSelect: super::UINT8,
    pub BankGroup: super::UINT8,
    pub BankAddress: super::UINT8,
    pub RowAddress: super::UINT32,
    pub ColumnAddress: super::UINT16,
    pub RankMultiplier: super::UINT8,
    pub SubChannel: super::UINT8,
    pub ChannelId: super::UINT8,
    pub Reserved: [super::UINT8; 40],
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_HARDWARE_ADDRESS_AMD {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_HARDWARE_ADDRESS_INTEL {
    pub MemDef: WHEA_MEMORY_DEFINITION,
    pub SystemAddress: super::UINT64,
    pub SpareSystemAddress: super::UINT64,
    pub DevicePhysicalAddress: super::UINT64,
    pub ChannelAddress: super::UINT64,
    pub RankAddress: super::UINT64,
    pub ProcessorSocketId: super::UINT8,
    pub MemoryControllerId: super::UINT8,
    pub TargetId: super::UINT8,
    pub LogicalChannelId: super::UINT8,
    pub ChannelId: super::UINT8,
    pub SubChannelId: super::UINT8,
    pub PhysicalRankId: super::UINT8,
    pub DimmSlotId: super::UINT8,
    pub DimmRankId: super::UINT8,
    pub Bank: super::UINT8,
    pub BankGroup: super::UINT8,
    pub Row: super::UINT32,
    pub Column: super::UINT32,
    pub LockStepRank: super::UINT8,
    pub LockStepPhysicalRank: super::UINT8,
    pub LockStepBank: super::UINT8,
    pub LockStepBankGroup: super::UINT8,
    pub ChipSelect: super::UINT8,
    pub Node: super::UINT8,
    pub ChipId: super::UINT8,
    pub Reserved: [super::UINT8; 40],
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_HARDWARE_ADDRESS_INTEL {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_RANGE {
    pub StartSystemPhysicalAddress: super::ULONGLONG,
    pub LengthInBytes: super::ULONGLONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_RANGE_ERROR_SECTION {
    pub ValidBits: WHEA_MEMORY_RANGE_ERROR_SECTION_VALIDBITS,
    pub Version: super::ULONG,
    pub DeviceInfo: WHEA_DEVICE_INFO,
    pub DeviceType: super::UCHAR,
    pub Reserved: super::UCHAR,
    pub RangeCount: super::USHORT,
    pub Ranges: [WHEA_MEMORY_RANGE; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_RANGE_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_MEMORY_RANGE_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcc245308_61d5_4356_a707_25172ec2f5ea);
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_MEMORY_RANGE_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_MEMORY_RANGE_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_RANGE_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_MEMORY_RANGE_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
pub const WHEA_MEMORY_RANGE_ERROR_SECTION_VERSION: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_THROTTLE_SUMMARY_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Status: windows_sys::core::NTSTATUS,
}
#[cfg(feature = "shared")]
impl Default for WHEA_MEMORY_THROTTLE_SUMMARY_FAILED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_MEM_PERSISTOFFLINE: i32 = 1;
pub const WHEA_MEM_PFA_DISABLE: i32 = 2;
pub const WHEA_MEM_PFA_PAGECOUNT: i32 = 3;
pub const WHEA_MEM_PFA_THRESHOLD: i32 = 4;
pub const WHEA_MEM_PFA_TIMEOUT: i32 = 5;
pub const WHEA_MSCHECK_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x48ab7f57_dc34_4f6c_a7d3_b0b5b0a74314);
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_MSR_DUMP_SECTION {
    pub MsrDumpBuffer: super::UCHAR,
    pub MsrDumpLength: super::ULONG,
    pub MsrDumpData: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_MSR_DUMP_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_NMI_ERROR_SECTION {
    pub Data: [super::UCHAR; 8],
    pub Flags: WHEA_NMI_ERROR_SECTION_FLAGS,
}
#[cfg(feature = "shared")]
impl Default for WHEA_NMI_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_NMI_ERROR_SECTION_FLAGS {
    pub Anonymous: WHEA_NMI_ERROR_SECTION_FLAGS_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_NMI_ERROR_SECTION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NMI_ERROR_SECTION_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR {
    pub Type: super::UCHAR,
    pub Length: super::UCHAR,
    pub Flags: WHEA_NOTIFICATION_FLAGS,
    pub u: WHEA_NOTIFICATION_DESCRIPTOR_0,
}
#[cfg(feature = "shared")]
impl Default for WHEA_NOTIFICATION_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
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
#[cfg(feature = "shared")]
impl Default for WHEA_NOTIFICATION_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_0 {
    pub PollInterval: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_1 {
    pub PollInterval: super::ULONG,
    pub Vector: super::ULONG,
    pub SwitchToPollingThreshold: super::ULONG,
    pub SwitchToPollingWindow: super::ULONG,
    pub ErrorThreshold: super::ULONG,
    pub ErrorThresholdWindow: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_2 {
    pub PollInterval: super::ULONG,
    pub Vector: super::ULONG,
    pub SwitchToPollingThreshold: super::ULONG,
    pub SwitchToPollingWindow: super::ULONG,
    pub ErrorThreshold: super::ULONG,
    pub ErrorThresholdWindow: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_3 {
    pub PollInterval: super::ULONG,
    pub Vector: super::ULONG,
    pub SwitchToPollingThreshold: super::ULONG,
    pub SwitchToPollingWindow: super::ULONG,
    pub ErrorThreshold: super::ULONG,
    pub ErrorThresholdWindow: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_4 {
    pub PollInterval: super::ULONG,
    pub Vector: super::ULONG,
    pub SwitchToPollingThreshold: super::ULONG,
    pub SwitchToPollingWindow: super::ULONG,
    pub ErrorThreshold: super::ULONG,
    pub ErrorThresholdWindow: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_5 {
    pub PollInterval: super::ULONG,
    pub Vector: super::ULONG,
    pub SwitchToPollingThreshold: super::ULONG,
    pub SwitchToPollingWindow: super::ULONG,
    pub ErrorThreshold: super::ULONG,
    pub ErrorThresholdWindow: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_6 {
    pub PollInterval: super::ULONG,
    pub Vector: super::ULONG,
    pub SwitchToPollingThreshold: super::ULONG,
    pub SwitchToPollingWindow: super::ULONG,
    pub ErrorThreshold: super::ULONG,
    pub ErrorThresholdWindow: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_DESCRIPTOR_0_7 {
    pub PollInterval: super::ULONG,
    pub Vector: super::ULONG,
    pub SwitchToPollingThreshold: super::ULONG,
    pub SwitchToPollingWindow: super::ULONG,
    pub ErrorThreshold: super::ULONG,
    pub ErrorThresholdWindow: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_NOTIFICATION_FLAGS {
    pub Anonymous: WHEA_NOTIFICATION_FLAGS_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for WHEA_NOTIFICATION_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_NOTIFICATION_FLAGS_0 {
    pub _bitfield: u16,
}
pub const WHEA_NOTIFICATION_TYPE_ARMV8_SEA: i32 = 8;
pub const WHEA_NOTIFICATION_TYPE_ARMV8_SEI: i32 = 9;
pub const WHEA_NOTIFICATION_TYPE_CMCI: i32 = 5;
pub const WHEA_NOTIFICATION_TYPE_EXTERNALINTERRUPT: i32 = 1;
pub const WHEA_NOTIFICATION_TYPE_EXTERNALINTERRUPT_GSIV: i32 = 10;
pub const WHEA_NOTIFICATION_TYPE_GPIO_SIGNAL: i32 = 7;
pub const WHEA_NOTIFICATION_TYPE_LOCALINTERRUPT: i32 = 2;
pub const WHEA_NOTIFICATION_TYPE_MCE: i32 = 6;
pub const WHEA_NOTIFICATION_TYPE_NMI: i32 = 4;
pub const WHEA_NOTIFICATION_TYPE_POLLED: i32 = 0;
pub const WHEA_NOTIFICATION_TYPE_SCI: i32 = 3;
pub const WHEA_NOTIFICATION_TYPE_SDEI: i32 = 11;
pub const WHEA_NOTIFY_ALL_OFFLINES: i32 = 16;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_OFFLINE_DONE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Address: super::ULONG64,
}
#[cfg(feature = "shared")]
impl Default for WHEA_OFFLINE_DONE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_OFFLINE_ERRS = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PACKET_LOG_DATA {
    pub LogData: [super::UCHAR; 36],
    pub ExtraBytes: [super::UCHAR; 36],
    pub BcParam3: super::ULONG_PTR,
    pub BcParam4: super::ULONG_PTR,
    pub LogDataLength: super::ULONG,
    pub LogTag: super::USHORT,
    pub Reserved: super::USHORT,
    pub Flags: WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PACKET_LOG_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS {
    pub Anonymous: WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS_0 {
    pub BridgeSecondaryStatus: super::USHORT,
    pub BridgeControl: super::USHORT,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_PCIEXPRESS_COMMAND_STATUS {
    pub Anonymous: WHEA_PCIEXPRESS_COMMAND_STATUS_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCIEXPRESS_COMMAND_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIEXPRESS_COMMAND_STATUS_0 {
    pub Command: super::USHORT,
    pub Status: super::USHORT,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIEXPRESS_DEVICE_ID {
    pub VendorID: super::USHORT,
    pub DeviceID: super::USHORT,
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub _bitfield3: u32,
}
pub type WHEA_PCIEXPRESS_DEVICE_TYPE = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PCIEXPRESS_ERROR_SECTION {
    pub ValidBits: WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS,
    pub PortType: WHEA_PCIEXPRESS_DEVICE_TYPE,
    pub Version: WHEA_PCIEXPRESS_VERSION,
    pub CommandStatus: WHEA_PCIEXPRESS_COMMAND_STATUS,
    pub Reserved: super::ULONG,
    pub DeviceId: WHEA_PCIEXPRESS_DEVICE_ID,
    pub DeviceSerialNumber: super::ULONGLONG,
    pub BridgeControlStatus: WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS,
    pub ExpressCapability: [super::UCHAR; 60],
    pub AerInfo: [super::UCHAR; 96],
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCIEXPRESS_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_PCIEXPRESS_VERSION {
    pub Anonymous: WHEA_PCIEXPRESS_VERSION_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCIEXPRESS_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIEXPRESS_VERSION_0 {
    pub MinorVersion: super::UCHAR,
    pub MajorVersion: super::UCHAR,
    pub Reserved: super::USHORT,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIE_ADDRESS {
    pub Segment: super::UINT32,
    pub Bus: super::UINT32,
    pub Device: super::UINT32,
    pub Function: super::UINT32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PCIE_CORRECTABLE_ERROR_DEVICES {
    pub ValidBits: WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS,
    pub Address: WHEA_PCIE_ADDRESS,
    pub Mask: super::UINT32,
    pub CorrectableErrorCount: [super::UINT32; 32],
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCIE_CORRECTABLE_ERROR_DEVICES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS {
    pub Anonymous: WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS_0,
    pub ValidBits: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PCIE_CORRECTABLE_ERROR_SECTION {
    pub Header: WHEA_PCIE_CORRECTABLE_ERROR_SECTION_HEADER,
    pub Devices: [WHEA_PCIE_CORRECTABLE_ERROR_DEVICES; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCIE_CORRECTABLE_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_PCIE_CORRECTABLE_ERROR_SECTION_COUNT_SIZE: i32 = 32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIE_CORRECTABLE_ERROR_SECTION_HEADER {
    pub Version: super::UINT16,
    pub Count: super::UINT16,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_PCIXBUS_COMMAND {
    pub Anonymous: WHEA_PCIXBUS_COMMAND_0,
    pub AsULONGLONG: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCIXBUS_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIXBUS_COMMAND_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PCIXBUS_ERROR_SECTION {
    pub ValidBits: WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS,
    pub ErrorStatus: WHEA_ERROR_STATUS,
    pub ErrorType: super::USHORT,
    pub BusId: WHEA_PCIXBUS_ID,
    pub Reserved: super::ULONG,
    pub BusAddress: super::ULONGLONG,
    pub BusData: super::ULONGLONG,
    pub BusCommand: WHEA_PCIXBUS_COMMAND,
    pub RequesterId: super::ULONGLONG,
    pub CompleterId: super::ULONGLONG,
    pub TargetId: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCIXBUS_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_PCIXBUS_ID {
    pub Anonymous: WHEA_PCIXBUS_ID_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCIXBUS_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIXBUS_ID_0 {
    pub BusNumber: super::UCHAR,
    pub BusSegment: super::UCHAR,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PCIXDEVICE_ERROR_SECTION {
    pub ValidBits: WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS,
    pub ErrorStatus: WHEA_ERROR_STATUS,
    pub IdInfo: WHEA_PCIXDEVICE_ID,
    pub MemoryNumber: super::ULONG,
    pub IoNumber: super::ULONG,
    pub RegisterDataPairs: [WHEA_PCIXDEVICE_REGISTER_PAIR; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCIXDEVICE_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIXDEVICE_ID {
    pub VendorId: super::USHORT,
    pub DeviceId: super::USHORT,
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub Reserved2: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCIXDEVICE_REGISTER_PAIR {
    pub Register: super::ULONGLONG,
    pub Data: super::ULONGLONG,
}
#[cfg(all(feature = "shared", feature = "wdm"))]
pub type WHEA_PCI_DPC_SECTION = PCI_EXPRESS_DPC_CAPABILITY;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCI_RECOVERY_SECTION {
    pub SignalType: super::UINT8,
    pub RecoveryAttempted: super::BOOLEAN,
    pub RecoveryStatus: super::UINT8,
}
pub type WHEA_PCI_RECOVERY_SIGNAL = i32;
pub type WHEA_PCI_RECOVERY_STATUS = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PCI_SLOT_NUMBER {
    pub u: WHEA_PCI_SLOT_NUMBER_0,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCI_SLOT_NUMBER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_PCI_SLOT_NUMBER_0 {
    pub bits: WHEA_PCI_SLOT_NUMBER_0_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PCI_SLOT_NUMBER_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PCI_SLOT_NUMBER_0_0 {
    pub _bitfield: u32,
}
pub const WHEA_PENDING_PAGE_LIST_SZ: i32 = 13;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_PERSISTENCE_INFO {
    pub Anonymous: WHEA_PERSISTENCE_INFO_0,
    pub AsULONGLONG: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PERSISTENCE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PERSISTENCE_INFO_0 {
    pub _bitfield: u64,
}
pub const WHEA_PFA_PAGE_RANGE_MAX: i32 = 256;
pub type WHEA_PFA_REMOVE_TRIGGER = i32;
pub const WHEA_PLUGIN_REGISTRATION_PACKET_V1: i32 = 65536;
pub const WHEA_PLUGIN_REGISTRATION_PACKET_V2: i32 = 131072;
pub const WHEA_PLUGIN_REGISTRATION_PACKET_VERSION: i32 = 131072;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PMEM_ERROR_SECTION {
    pub ValidBits: WHEA_PMEM_ERROR_SECTION_VALIDBITS,
    pub LocationInfo: [super::UCHAR; 64],
    pub ErrorStatus: WHEA_ERROR_STATUS,
    pub NFITHandle: super::ULONG,
    pub PageRangeCount: super::ULONG,
    pub PageRange: [WHEA_PMEM_PAGE_RANGE; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_PMEM_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_PMEM_ERROR_SECTION_LOCATION_INFO_SIZE: i32 = 64;
pub const WHEA_PMEM_ERROR_SECTION_MAX_PAGES: i32 = 50;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_PMEM_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PMEM_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PMEM_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PMEM_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PMEM_PAGE_RANGE {
    pub StartingPfn: super::ULONG64,
    pub PageCount: super::ULONG64,
    pub MarkedBadBitmap: super::ULONG64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_ADDRESS_TRANSLATION_BUFFER_INTEL {
    pub SwSmi: super::UINT32,
    pub Command: super::UINT32,
    pub Status: super::UINT32,
    pub SystemAddress: super::UINT64,
    pub NmSystemAddress: super::UINT64,
    pub SpareSystemAddress: super::UINT64,
    pub DevicePhysicalAddress: super::UINT64,
    pub ProcessorSocketId: super::UINT64,
    pub MemoryControllerId: super::UINT64,
    pub NmMemoryControllerId: super::UINT64,
    pub TargetId: super::UINT64,
    pub LogicalChannelId: super::UINT64,
    pub ChannelAddress: super::UINT64,
    pub NmChannelAddress: super::UINT64,
    pub ChannelId: super::UINT64,
    pub NmChannelId: super::UINT64,
    pub RankAddress: super::UINT64,
    pub NmRankAddress: super::UINT64,
    pub PhysicalRankId: super::UINT64,
    pub NmPhysicalRankId: super::UINT64,
    pub DimmSlotId: super::UINT64,
    pub NmDimmSlotId: super::UINT64,
    pub DimmRankId: super::UINT64,
    pub Row: super::UINT64,
    pub NmRow: super::UINT64,
    pub Column: super::UINT64,
    pub NmColumn: super::UINT64,
    pub Bank: super::UINT64,
    pub NmBank: super::UINT64,
    pub BankGroup: super::UINT64,
    pub NmBankGroup: super::UINT64,
    pub LockStepRank: super::UINT64,
    pub LockStepPhysicalRank: super::UINT64,
    pub LockStepBank: super::UINT64,
    pub LockStepBankGroup: super::UINT64,
    pub ChipSelect: super::UINT64,
    pub NmChipSelect: super::UINT64,
    pub Node: super::UINT64,
    pub ChipId: super::UINT64,
    pub NmChipId: super::UINT64,
}
pub const WHEA_PRM_ADDRESS_TRANSLATION_INTEL_FORWARD_ADDRESS_TRANSLATE: i32 = 2;
pub const WHEA_PRM_ADDRESS_TRANSLATION_INTEL_GET_ADDRESS_PARAMETERS: i32 = 1;
pub const WHEA_PRM_ADDRESS_TRANSLATION_INTEL_INTERNAL_ERROR: i32 = 3;
pub const WHEA_PRM_ADDRESS_TRANSLATION_INTEL_INVALID_COMMAND: i32 = 2;
pub const WHEA_PRM_ADDRESS_TRANSLATION_INTEL_REVERSE_ADDRESS_TRANSLATE: i32 = 3;
pub const WHEA_PRM_ADDRESS_TRANSLATION_INTEL_SUCCESS: i32 = 0;
pub const WHEA_PRM_ADDRESS_TRANSLATION_INTEL_UNKNOWN_FAILURE: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_DRAM_TO_NORMALIZED_OUT_BUFFER_AMD {
    pub NormalizedAddress: super::UINT64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_DRAM_TO_NORMALIZED_PARAM_BUFFER_AMD {
    pub SocketNumber: super::UINT8,
    pub UmcBankInstanceId: super::UINT64,
    pub ChipSelect: super::UINT8,
    pub BankGroup: super::UINT8,
    pub BankAddress: super::UINT8,
    pub RowAddress: super::UINT32,
    pub ColumnAddress: super::UINT16,
    pub RankMultiplier: super::UINT8,
    pub SubChannel: super::UINT8,
    pub OutputBuffer: PWHEA_PRM_DRAM_TO_NORMALIZED_OUT_BUFFER_AMD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_DRAM_TO_SPA_OUT_BUFFER_AMD {
    pub SystemPhysicalAddress: super::UINT64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_DRAM_TO_SPA_PARAM_BUFFER_AMD {
    pub SocketNumber: super::UINT8,
    pub UmcBankInstanceId: super::UINT64,
    pub ChipSelect: super::UINT8,
    pub BankGroup: super::UINT8,
    pub BankAddress: super::UINT8,
    pub RowAddress: super::UINT32,
    pub ColumnAddress: super::UINT16,
    pub RankMultiplier: super::UINT8,
    pub SubChannel: super::UINT8,
    pub OutputBuffer: PWHEA_PRM_DRAM_TO_SPA_OUT_BUFFER_AMD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_NORMALIZED_TO_DRAM_OUT_BUFFER_AMD {
    pub ChipSelect: super::UINT8,
    pub BankGroup: super::UINT8,
    pub BankAddress: super::UINT8,
    pub RowAddress: super::UINT32,
    pub ColumnAddress: super::UINT16,
    pub RankMultiplier: super::UINT8,
    pub SubChannel: super::UINT8,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_NORMALIZED_TO_DRAM_PARAM_BUFFER_AMD {
    pub NormalizedAddress: super::UINT64,
    pub SocketNumber: super::UINT8,
    pub UmcBankInstanceId: super::UINT64,
    pub OutputBuffer: PWHEA_PRM_NORMALIZED_TO_DRAM_OUT_BUFFER_AMD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_NORMALIZED_TO_SPA_OUT_BUFFER_AMD {
    pub SystemPhysicalAddress: super::UINT64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_NORMALIZED_TO_SPA_PARAM_BUFFER_AMD {
    pub NormalizedAddress: super::UINT64,
    pub SocketNumber: super::UINT8,
    pub UmcBankInstanceId: super::UINT64,
    pub OutputBuffer: PWHEA_PRM_NORMALIZED_TO_SPA_OUT_BUFFER_AMD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_SPA_TO_DRAM_OUT_BUFFER_AMD {
    pub ChipSelect: super::UINT8,
    pub BankGroup: super::UINT8,
    pub BankAddress: super::UINT8,
    pub RowAddress: super::UINT32,
    pub ColumnAddress: super::UINT16,
    pub RankMultiplier: super::UINT8,
    pub SubChannel: super::UINT8,
    pub SocketNumber: super::UINT8,
    pub UmcBankInstanceId: super::UINT64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_SPA_TO_DRAM_PARAM_BUFFER_AMD {
    pub SystemPhysicalAddress: super::UINT64,
    pub OutputBuffer: PWHEA_PRM_SPA_TO_DRAM_OUT_BUFFER_AMD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_SPA_TO_NORMALIZED_OUT_BUFFER_AMD {
    pub NormalizedAddress: super::UINT64,
    pub SocketNumber: super::UINT8,
    pub UmcBankInstanceId: super::UINT64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PRM_SPA_TO_NORMALIZED_PARAM_BUFFER_AMD {
    pub SystemPhysicalAddress: super::UINT64,
    pub OutputBuffer: PWHEA_PRM_SPA_TO_NORMALIZED_OUT_BUFFER_AMD,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_PROCESSOR_FAMILY_INFO {
    pub Anonymous: WHEA_PROCESSOR_FAMILY_INFO_0,
    pub AsULONGLONG: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PROCESSOR_FAMILY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PROCESSOR_FAMILY_INFO_0 {
    pub _bitfield: u32,
    pub NativeModelId: super::ULONG,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PROCESSOR_GENERIC_ERROR_SECTION {
    pub ValidBits: WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS,
    pub ProcessorType: super::UCHAR,
    pub InstructionSet: super::UCHAR,
    pub ErrorType: super::UCHAR,
    pub Operation: super::UCHAR,
    pub Flags: super::UCHAR,
    pub Level: super::UCHAR,
    pub Reserved: super::USHORT,
    pub CPUVersion: super::ULONGLONG,
    pub CPUBrandString: [super::UCHAR; 128],
    pub ProcessorId: super::ULONGLONG,
    pub TargetAddress: super::ULONGLONG,
    pub RequesterId: super::ULONGLONG,
    pub ResponderId: super::ULONGLONG,
    pub InstructionPointer: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PROCESSOR_GENERIC_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PI_CPUID {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub CpuVendor: super::UINT32,
    pub CpuFamily: super::UINT32,
    pub CpuModel: super::UINT32,
    pub CpuStepping: super::UINT32,
    pub NumBanks: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PSHED_PI_CPUID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PI_CPU_BUSES_INIT_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Status: windows_sys::core::NTSTATUS,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PSHED_PI_CPU_BUSES_INIT_FAILED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PI_ERROR_RECORD_FULL_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PlatformName: [super::CHAR; 32],
}
#[cfg(feature = "shared")]
impl Default for WHEA_PSHED_PI_ERROR_RECORD_FULL_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PI_SERVER_TYPE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PlatformName: [super::CHAR; 32],
    pub ServerType: [super::CHAR; 32],
}
#[cfg(feature = "shared")]
impl Default for WHEA_PSHED_PI_SERVER_TYPE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PI_TRACE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Buffer: [super::CCHAR; 256],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_PSHED_PI_TRACE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PSHED_PLUGIN_CALLBACKS {
    pub GetAllErrorSources: PSHED_PI_GET_ALL_ERROR_SOURCES,
    pub Reserved: super::PVOID,
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
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_DIMM_MISMATCH {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub FirmwareBank: super::UINT16,
    pub FirmwareCol: super::UINT16,
    pub FirmwareRow: super::UINT16,
    pub RetryRdBank: super::UINT16,
    pub RetryRdCol: super::UINT16,
    pub RetryRdRow: super::UINT16,
    pub TaBank: super::UINT16,
    pub TaCol: super::UINT16,
    pub TaRow: super::UINT16,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PSHED_PLUGIN_DIMM_MISMATCH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub EnableError: WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_FAILED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_HEARTBEAT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PSHED_PLUGIN_HEARTBEAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_INIT_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Status: windows_sys::core::NTSTATUS,
}
#[cfg(feature = "shared")]
impl Default for WHEA_PSHED_PLUGIN_INIT_FAILED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_LOAD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PluginName: [super::WCHAR; 32],
    pub MajorVersion: super::ULONG,
    pub MinorVersion: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_PSHED_PLUGIN_LOAD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_PLATFORM_SUPPORT_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PluginName: [super::WCHAR; 32],
    pub Supported: super::BOOLEAN,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_PSHED_PLUGIN_PLATFORM_SUPPORT_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type WHEA_PSHED_PLUGIN_REGISTRATION_PACKET = WHEA_PSHED_PLUGIN_REGISTRATION_PACKET_V2;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PSHED_PLUGIN_REGISTRATION_PACKET_V1 {
    pub Length: super::ULONG,
    pub Version: super::ULONG,
    pub Context: super::PVOID,
    pub FunctionalAreaMask: super::ULONG,
    pub Reserved: super::ULONG,
    pub Callbacks: WHEA_PSHED_PLUGIN_CALLBACKS,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_PSHED_PLUGIN_REGISTRATION_PACKET_V2 {
    pub Length: super::ULONG,
    pub Version: super::ULONG,
    pub Context: super::PVOID,
    pub FunctionalAreaMask: super::ULONG,
    pub Reserved: super::ULONG,
    pub Callbacks: WHEA_PSHED_PLUGIN_CALLBACKS,
    pub PluginHandle: super::PVOID,
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_UNLOAD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PluginName: [super::WCHAR; 32],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_PSHED_PLUGIN_UNLOAD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_RAW_DATA_FORMAT = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_READ_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Offset: super::UINT32,
    pub BytesToRead: super::UINT32,
    pub OutputBufferLength: super::UINT32,
    pub Status: windows_sys::core::NTSTATUS,
    pub Reason: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for WHEA_READ_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_RECORD_CREATOR_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcf07c4bd_b789_4e18_b3c4_1f732cb57131);
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_RECOVERY_ACTION {
    pub Anonymous: WHEA_RECOVERY_ACTION_0,
    pub AsUINT64: super::UINT64,
}
#[cfg(feature = "shared")]
impl Default for WHEA_RECOVERY_ACTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_RECOVERY_ACTION_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_RECOVERY_CONTEXT {
    pub Anonymous: WHEA_RECOVERY_CONTEXT_0,
    pub PartitionId: super::UINT64,
    pub VpIndex: super::UINT32,
    pub ErrorType: WHEA_RECOVERY_CONTEXT_ERROR_TYPE,
    pub PageCount: super::ULONG,
    pub PageInfo: [WHEA_RECOVERY_CONTEXT_PAGE_INFO; 256],
    pub ChildPartitionId: super::UINT64,
}
#[cfg(feature = "shared")]
impl Default for WHEA_RECOVERY_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_RECOVERY_CONTEXT_0 {
    pub MemoryError: WHEA_RECOVERY_CONTEXT_0_0,
    pub PmemError: WHEA_RECOVERY_CONTEXT_0_1,
}
#[cfg(feature = "shared")]
impl Default for WHEA_RECOVERY_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_RECOVERY_CONTEXT_0_0 {
    pub Address: super::ULONG_PTR,
    pub Consumed: super::BOOLEAN,
    pub ErrorCode: super::UINT16,
    pub ErrorIpValid: super::BOOLEAN,
    pub RestartIpValid: super::BOOLEAN,
    pub ClearPoison: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_RECOVERY_CONTEXT_0_1 {
    pub PmemErrInfo: super::ULONG_PTR,
}
pub type WHEA_RECOVERY_CONTEXT_ACTION_TAKEN = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO {
    pub Bits: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO_0,
    pub AsULONG64: super::ULONG64,
}
#[cfg(feature = "shared")]
impl Default for WHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO_0 {
    pub _bitfield: u64,
}
pub type WHEA_RECOVERY_CONTEXT_ERROR_TYPE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_RECOVERY_CONTEXT_PAGE_INFO {
    pub ComponentTag: super::ULONG,
    pub PageStatus: windows_sys::core::NTSTATUS,
    pub ActionTaken: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN,
    pub NotifyFlags: WHEA_IN_USE_PAGE_NOTIFY_FLAGS,
    pub ImmediateSuccess: super::BOOLEAN,
    pub Reserved: super::UINT16,
    pub ActionTakenAdditionalInfo: WHEA_RECOVERY_CONTEXT_ACTION_TAKEN_ADDITIONAL_INFO,
}
#[cfg(feature = "shared")]
impl Default for WHEA_RECOVERY_CONTEXT_PAGE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_RECOVERY_FAILURE_REASON = i32;
pub type WHEA_RECOVERY_TYPE = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_REGISTER_KEY_NOTIFICATION_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[cfg(feature = "shared")]
impl Default for WHEA_REGISTER_KEY_NOTIFICATION_FAILED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_REGISTRY_ERRORS = i32;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_REGISTRY_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RegErr: WHEA_REGISTRY_ERRORS,
    pub Status: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for WHEA_REGISTRY_ERROR_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_REGNOTIFY_POLICY_CHANGE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PolicyName: [super::CCHAR; 32],
    pub PolicyIndex: super::ULONG,
    pub PolicyValue: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_REGNOTIFY_POLICY_CHANGE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS {
    pub Anonymous: WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS_0 {
    pub _bitfield: u32,
}
pub const WHEA_RESTORE_CMCI_ATTEMPTS: i32 = 8;
pub const WHEA_RESTORE_CMCI_ENABLED: i32 = 7;
pub const WHEA_RESTORE_CMCI_ERR_LIMIT: i32 = 9;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_REVISION {
    pub Anonymous: WHEA_REVISION_0,
    pub AsUSHORT: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for WHEA_REVISION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_REVISION_0 {
    pub MinorRevision: super::UCHAR,
    pub MajorRevision: super::UCHAR,
}
pub const WHEA_ROW_FAIL_CHECK_ENABLE: i32 = 18;
pub const WHEA_ROW_FAIL_CHECK_EXTENT: i32 = 17;
pub const WHEA_ROW_FAIL_CHECK_THRESHOLD: i32 = 19;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_SEA_SECTION {
    pub Esr: super::ULONG,
    pub Far: super::ULONG64,
    pub Par: super::ULONG64,
    pub WasKernel: super::BOOLEAN,
}
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_CONTAINMENTWRN: i32 = 2;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_FRU_TEXT_BY_PLUGIN: i32 = 128;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_LATENTERROR: i32 = 32;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_PRIMARY: i32 = 1;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_PROPAGATED: i32 = 64;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_RESET: i32 = 4;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_RESOURCENA: i32 = 16;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_THRESHOLDEXCEEDED: i32 = 8;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_SEI_SECTION {
    pub Esr: super::ULONG,
    pub Far: super::ULONG64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_PROGRESS {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub BugCheckCode: super::ULONG,
    pub BugCheckProgressSummary: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_SEL_BUGCHECK_PROGRESS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_MULTIPLE_BUGCHECK_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub IsBugcheckOwner: super::BOOLEAN,
    pub RecursionCount: super::UCHAR,
    pub IsBugcheckRecoveryOwner: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for WHEA_SEL_BUGCHECK_RECOVERY_STATUS_MULTIPLE_BUGCHECK_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Success: super::BOOLEAN,
    pub Version: super::UCHAR,
    pub EntryCount: super::USHORT,
    pub Data: WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT_0,
}
#[cfg(feature = "shared")]
impl Default for WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT_0 {
    pub DumpPolicy: super::UCHAR,
    pub Reserved: [super::UCHAR; 3],
}
#[cfg(feature = "shared")]
impl Default for WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_VERSION: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE2_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub BootId: super::ULONG,
    pub Success: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE2_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "ntdef", feature = "shared"))]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_START_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub StartingIrql: super::KIRQL,
}
#[cfg(all(feature = "ntdef", feature = "shared"))]
impl Default for WHEA_SEL_BUGCHECK_RECOVERY_STATUS_START_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_RAW_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Payload: [super::UCHAR; 256],
}
#[cfg(feature = "shared")]
impl Default for WHEA_SEL_RAW_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type WHEA_SIGNAL_HANDLER_OVERRIDE_CALLBACK = Option<unsafe extern "system" fn(context: super::UINT_PTR) -> super::BOOLEAN>;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_SRAR_DETAIL_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RecoveryContextFlags: super::UINT32,
    pub RecoveryContextPa: super::UINT64,
    pub PageOfflineStatus: windows_sys::core::NTSTATUS,
    pub KernelConsumerError: super::BOOLEAN,
}
#[cfg(feature = "shared")]
impl Default for WHEA_SRAR_DETAIL_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_SRAS_TABLE_ENTRIES_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub LogNumber: super::UINT32,
    pub NumberSignals: super::UINT32,
    pub Data: [super::UINT8; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_SRAS_TABLE_ENTRIES_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_SRAS_TABLE_ERROR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[cfg(feature = "shared")]
impl Default for WHEA_SRAS_TABLE_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_SRAS_TABLE_NOT_FOUND {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[cfg(feature = "shared")]
impl Default for WHEA_SRAS_TABLE_NOT_FOUND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_ADD_ERR_SRC_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[cfg(feature = "shared")]
impl Default for WHEA_THROTTLE_ADD_ERR_SRC_FAILED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_MEMORY_ADD_OR_REMOVE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub SocketId: super::UINT32,
    pub ChannelId: super::UINT32,
    pub DimmSlot: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for WHEA_THROTTLE_MEMORY_ADD_OR_REMOVE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_PCIE_ADD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Address: WHEA_PCIE_ADDRESS,
    pub Mask: super::UINT32,
    pub Updated: super::BOOLEAN,
    pub Status: windows_sys::core::NTSTATUS,
}
#[cfg(feature = "shared")]
impl Default for WHEA_THROTTLE_PCIE_ADD_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_PCIE_REMOVE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Address: WHEA_PCIE_ADDRESS,
    pub Mask: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for WHEA_THROTTLE_PCIE_REMOVE_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_REGISTRY_CORRUPT_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ThrottleType: WHEA_THROTTLE_TYPE,
}
#[cfg(feature = "shared")]
impl Default for WHEA_THROTTLE_REGISTRY_CORRUPT_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_REG_DATA_IGNORED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ThrottleType: WHEA_THROTTLE_TYPE,
}
#[cfg(feature = "shared")]
impl Default for WHEA_THROTTLE_REG_DATA_IGNORED_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WHEA_THROTTLE_TYPE = i32;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union WHEA_TIMESTAMP {
    pub Anonymous: WHEA_TIMESTAMP_0,
    pub AsLARGE_INTEGER: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_TIMESTAMP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct WHEA_TIMESTAMP_0 {
    pub _bitfield: u64,
}
pub const WHEA_TLBCHECK_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfc06b535_5e1f_4562_9f25_0a3b9adb63c3);
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_VERSION_MISMATCH_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ComponentName: [super::WCHAR; 32],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_VERSION_MISMATCH_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_WRITE_FLAG_DUMMY: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_WRITE_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Offset: super::UINT32,
    pub InputLength: super::UINT32,
    pub Status: windows_sys::core::NTSTATUS,
    pub Reason: super::UINT32,
}
#[cfg(feature = "shared")]
impl Default for WHEA_WRITE_VENDOR_DEFINED_INJECTION_AREA_LOG_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct WHEA_X64_REGISTER_STATE {
    pub Rax: super::ULONGLONG,
    pub Rbx: super::ULONGLONG,
    pub Rcx: super::ULONGLONG,
    pub Rdx: super::ULONGLONG,
    pub Rsi: super::ULONGLONG,
    pub Rdi: super::ULONGLONG,
    pub Rbp: super::ULONGLONG,
    pub Rsp: super::ULONGLONG,
    pub R8: super::ULONGLONG,
    pub R9: super::ULONGLONG,
    pub R10: super::ULONGLONG,
    pub R11: super::ULONGLONG,
    pub R12: super::ULONGLONG,
    pub R13: super::ULONGLONG,
    pub R14: super::ULONGLONG,
    pub R15: super::ULONGLONG,
    pub Cs: super::USHORT,
    pub Ds: super::USHORT,
    pub Ss: super::USHORT,
    pub Es: super::USHORT,
    pub Fs: super::USHORT,
    pub Gs: super::USHORT,
    pub Reserved: super::ULONG,
    pub Rflags: super::ULONGLONG,
    pub Eip: super::ULONGLONG,
    pub Cr0: super::ULONGLONG,
    pub Cr1: super::ULONGLONG,
    pub Cr2: super::ULONGLONG,
    pub Cr3: super::ULONGLONG,
    pub Cr4: super::ULONGLONG,
    pub Cr8: super::ULONGLONG,
    pub Gdtr: WHEA128A,
    pub Idtr: WHEA128A,
    pub Ldtr: super::USHORT,
    pub Tr: super::USHORT,
}
#[cfg(feature = "shared")]
impl Default for WHEA_X64_REGISTER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_X86_REGISTER_STATE {
    pub Eax: super::ULONG,
    pub Ebx: super::ULONG,
    pub Ecx: super::ULONG,
    pub Edx: super::ULONG,
    pub Esi: super::ULONG,
    pub Edi: super::ULONG,
    pub Ebp: super::ULONG,
    pub Esp: super::ULONG,
    pub Cs: super::USHORT,
    pub Ds: super::USHORT,
    pub Ss: super::USHORT,
    pub Es: super::USHORT,
    pub Fs: super::USHORT,
    pub Gs: super::USHORT,
    pub Eflags: super::ULONG,
    pub Eip: super::ULONG,
    pub Cr0: super::ULONG,
    pub Cr1: super::ULONG,
    pub Cr2: super::ULONG,
    pub Cr3: super::ULONG,
    pub Cr4: super::ULONG,
    pub Gdtr: super::ULONGLONG,
    pub Idtr: super::ULONGLONG,
    pub Ldtr: super::USHORT,
    pub Tr: super::USHORT,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_XPF_BUS_CHECK {
    pub Anonymous: WHEA_XPF_BUS_CHECK_0,
    pub XpfBusCheck: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_XPF_BUS_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_BUS_CHECK_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_XPF_CACHE_CHECK {
    pub Anonymous: WHEA_XPF_CACHE_CHECK_0,
    pub XpfCacheCheck: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_XPF_CACHE_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_CACHE_CHECK_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_CMC_DESCRIPTOR {
    pub Type: super::USHORT,
    pub Enabled: super::BOOLEAN,
    pub NumberOfBanks: super::UCHAR,
    pub Reserved: super::ULONG,
    pub Notify: WHEA_NOTIFICATION_DESCRIPTOR,
    pub Banks: [WHEA_XPF_MC_BANK_DESCRIPTOR; 32],
}
#[cfg(feature = "shared")]
impl Default for WHEA_XPF_CMC_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_CMC_DESCRIPTOR_V2 {
    pub Type: super::USHORT,
    pub Enabled: super::BOOLEAN,
    pub NumberOfBanks: super::UCHAR,
    pub Reserved: super::ULONG,
    pub Notify: WHEA_NOTIFICATION_DESCRIPTOR,
    pub Banks: [WHEA_XPF_MC_BANK_DESCRIPTOR; 64],
}
#[cfg(feature = "shared")]
impl Default for WHEA_XPF_CMC_DESCRIPTOR_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_CONTEXT_INFO {
    pub RegisterContextType: super::USHORT,
    pub RegisterDataSize: super::USHORT,
    pub MSRAddress: super::ULONG,
    pub MmRegisterAddress: super::ULONGLONG,
}
pub const WHEA_XPF_MCA_EXBANK_COUNT: i32 = 64;
pub const WHEA_XPF_MCA_EXTREG_MAX_COUNT: i32 = 24;
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_MCA_SECTION {
    pub VersionNumber: super::ULONG,
    pub CpuVendor: WHEA_CPU_VENDOR,
    pub Timestamp: super::LARGE_INTEGER,
    pub ProcessorNumber: super::ULONG,
    pub GlobalStatus: MCG_STATUS,
    pub InstructionPointer: super::ULONGLONG,
    pub BankNumber: super::ULONG,
    pub Status: MCI_STATUS,
    pub Address: super::ULONGLONG,
    pub Misc: super::ULONGLONG,
    pub ExtendedRegisterCount: super::ULONG,
    pub ApicId: super::ULONG,
    pub Anonymous: WHEA_XPF_MCA_SECTION_0,
    pub GlobalCapability: MCG_CAP,
    pub RecoveryInfo: XPF_RECOVERY_INFO,
    pub ExBankCount: super::ULONG,
    pub BankNumberEx: [super::ULONG; 64],
    pub StatusEx: [MCI_STATUS; 64],
    pub AddressEx: [super::ULONGLONG; 64],
    pub MiscEx: [super::ULONGLONG; 64],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_XPF_MCA_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_MCA_SECTION_0 {
    pub ExtendedRegisters: [super::ULONGLONG; 24],
    pub AMDExtendedRegisters: WHEA_AMD_EXTENDED_REGISTERS,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WHEA_XPF_MCA_SECTION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_XPF_MCA_SECTION_VERSION: i32 = 5;
pub const WHEA_XPF_MCA_SECTION_VERSION_2: i32 = 2;
pub const WHEA_XPF_MCA_SECTION_VERSION_3: i32 = 3;
pub const WHEA_XPF_MCA_SECTION_VERSION_4: i32 = 4;
pub const WHEA_XPF_MCA_SECTION_VERSION_5: i32 = 5;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_MCE_DESCRIPTOR {
    pub Type: super::USHORT,
    pub Enabled: super::UCHAR,
    pub NumberOfBanks: super::UCHAR,
    pub Flags: XPF_MCE_FLAGS,
    pub MCG_Capability: super::ULONGLONG,
    pub MCG_GlobalControl: super::ULONGLONG,
    pub Banks: [WHEA_XPF_MC_BANK_DESCRIPTOR; 32],
}
#[cfg(feature = "shared")]
impl Default for WHEA_XPF_MCE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_MCE_DESCRIPTOR_V2 {
    pub Type: super::USHORT,
    pub Enabled: super::UCHAR,
    pub NumberOfBanks: super::UCHAR,
    pub Flags: XPF_MCE_FLAGS,
    pub MCG_Capability: super::ULONGLONG,
    pub MCG_GlobalControl: super::ULONGLONG,
    pub Banks: [WHEA_XPF_MC_BANK_DESCRIPTOR; 64],
}
#[cfg(feature = "shared")]
impl Default for WHEA_XPF_MCE_DESCRIPTOR_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_MC_BANK_DESCRIPTOR {
    pub BankNumber: super::UCHAR,
    pub ClearOnInitialization: super::BOOLEAN,
    pub StatusDataFormat: super::UCHAR,
    pub Flags: XPF_MC_BANK_FLAGS,
    pub ControlMsr: super::ULONG,
    pub StatusMsr: super::ULONG,
    pub AddressMsr: super::ULONG,
    pub MiscMsr: super::ULONG,
    pub ControlData: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_XPF_MC_BANK_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_AMD64MCA: i32 = 2;
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_IA32MCA: i32 = 0;
pub const WHEA_XPF_MC_BANK_STATUSFORMAT_Intel64MCA: i32 = 1;
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_XPF_MS_CHECK {
    pub Anonymous: WHEA_XPF_MS_CHECK_0,
    pub XpfMsCheck: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_XPF_MS_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_MS_CHECK_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_NMI_DESCRIPTOR {
    pub Type: super::USHORT,
    pub Enabled: super::BOOLEAN,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_PROCESSOR_ERROR_SECTION {
    pub ValidBits: WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS,
    pub LocalAPICId: super::ULONGLONG,
    pub CpuId: [super::UCHAR; 48],
    pub VariableInfo: [super::UCHAR; 1],
}
#[cfg(feature = "shared")]
impl Default for WHEA_XPF_PROCESSOR_ERROR_SECTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_PROCINFO {
    pub CheckInfoId: windows_sys::core::GUID,
    pub ValidBits: WHEA_XPF_PROCINFO_VALIDBITS,
    pub CheckInfo: WHEA_XPF_PROCINFO_0,
    pub TargetId: super::ULONGLONG,
    pub RequesterId: super::ULONGLONG,
    pub ResponderId: super::ULONGLONG,
    pub InstructionPointer: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_XPF_PROCINFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_XPF_PROCINFO_0 {
    pub CacheCheck: WHEA_XPF_CACHE_CHECK,
    pub TlbCheck: WHEA_XPF_TLB_CHECK,
    pub BusCheck: WHEA_XPF_BUS_CHECK,
    pub MsCheck: WHEA_XPF_MS_CHECK,
    pub AsULONGLONG: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_XPF_PROCINFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_XPF_PROCINFO_VALIDBITS {
    pub Anonymous: WHEA_XPF_PROCINFO_VALIDBITS_0,
    pub ValidBits: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_XPF_PROCINFO_VALIDBITS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WHEA_XPF_PROCINFO_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union WHEA_XPF_TLB_CHECK {
    pub Anonymous: WHEA_XPF_TLB_CHECK_0,
    pub XpfTLBCheck: super::ULONGLONG,
}
#[cfg(feature = "shared")]
impl Default for WHEA_XPF_TLB_CHECK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
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
pub const XPF_BUS_CHECK_ADDRESS_IO: i32 = 2;
pub const XPF_BUS_CHECK_ADDRESS_MEMORY: i32 = 0;
pub const XPF_BUS_CHECK_ADDRESS_OTHER: i32 = 3;
pub const XPF_BUS_CHECK_ADDRESS_RESERVED: i32 = 1;
pub const XPF_BUS_CHECK_OPERATION_DATAREAD: i32 = 3;
pub const XPF_BUS_CHECK_OPERATION_DATAWRITE: i32 = 4;
pub const XPF_BUS_CHECK_OPERATION_GENERIC: i32 = 0;
pub const XPF_BUS_CHECK_OPERATION_GENREAD: i32 = 1;
pub const XPF_BUS_CHECK_OPERATION_GENWRITE: i32 = 2;
pub const XPF_BUS_CHECK_OPERATION_INSTRUCTIONFETCH: i32 = 5;
pub const XPF_BUS_CHECK_OPERATION_PREFETCH: i32 = 6;
pub const XPF_BUS_CHECK_PARTICIPATION_GENERIC: i32 = 3;
pub const XPF_BUS_CHECK_PARTICIPATION_PROCOBSERVED: i32 = 2;
pub const XPF_BUS_CHECK_PARTICIPATION_PROCORIGINATED: i32 = 0;
pub const XPF_BUS_CHECK_PARTICIPATION_PROCRESPONDED: i32 = 1;
pub const XPF_BUS_CHECK_TRANSACTIONTYPE_DATAACCESS: i32 = 1;
pub const XPF_BUS_CHECK_TRANSACTIONTYPE_GENERIC: i32 = 2;
pub const XPF_BUS_CHECK_TRANSACTIONTYPE_INSTRUCTION: i32 = 0;
pub const XPF_CACHE_CHECK_OPERATION_DATAREAD: i32 = 3;
pub const XPF_CACHE_CHECK_OPERATION_DATAWRITE: i32 = 4;
pub const XPF_CACHE_CHECK_OPERATION_EVICTION: i32 = 7;
pub const XPF_CACHE_CHECK_OPERATION_GENERIC: i32 = 0;
pub const XPF_CACHE_CHECK_OPERATION_GENREAD: i32 = 1;
pub const XPF_CACHE_CHECK_OPERATION_GENWRITE: i32 = 2;
pub const XPF_CACHE_CHECK_OPERATION_INSTRUCTIONFETCH: i32 = 5;
pub const XPF_CACHE_CHECK_OPERATION_PREFETCH: i32 = 6;
pub const XPF_CACHE_CHECK_OPERATION_SNOOP: i32 = 8;
pub const XPF_CACHE_CHECK_TRANSACTIONTYPE_DATAACCESS: i32 = 1;
pub const XPF_CACHE_CHECK_TRANSACTIONTYPE_GENERIC: i32 = 2;
pub const XPF_CACHE_CHECK_TRANSACTIONTYPE_INSTRUCTION: i32 = 0;
pub const XPF_CONTEXT_INFO_32BITCONTEXT: i32 = 2;
pub const XPF_CONTEXT_INFO_32BITDEBUGREGS: i32 = 5;
pub const XPF_CONTEXT_INFO_64BITCONTEXT: i32 = 3;
pub const XPF_CONTEXT_INFO_64BITDEBUGREGS: i32 = 6;
pub const XPF_CONTEXT_INFO_FXSAVE: i32 = 4;
pub const XPF_CONTEXT_INFO_MMREGISTERS: i32 = 7;
pub const XPF_CONTEXT_INFO_MSRREGISTERS: i32 = 1;
pub const XPF_CONTEXT_INFO_UNCLASSIFIEDDATA: i32 = 0;
pub const XPF_MCA_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8a1e1d01_42f9_4557_9c33_565e5cc3f7e8);
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union XPF_MCE_FLAGS {
    pub Anonymous: XPF_MCE_FLAGS_0,
    pub AsULONG: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for XPF_MCE_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XPF_MCE_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union XPF_MC_BANK_FLAGS {
    pub Anonymous: XPF_MC_BANK_FLAGS_0,
    pub AsUCHAR: super::UCHAR,
}
#[cfg(feature = "shared")]
impl Default for XPF_MC_BANK_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XPF_MC_BANK_FLAGS_0 {
    pub _bitfield: u8,
}
pub const XPF_MS_CHECK_ERRORTYPE_EXTERNAL: i32 = 3;
pub const XPF_MS_CHECK_ERRORTYPE_FRC: i32 = 4;
pub const XPF_MS_CHECK_ERRORTYPE_INTERNALUNCLASSIFIED: i32 = 5;
pub const XPF_MS_CHECK_ERRORTYPE_MCROMPARITY: i32 = 2;
pub const XPF_MS_CHECK_ERRORTYPE_NOERROR: i32 = 0;
pub const XPF_MS_CHECK_ERRORTYPE_UNCLASSIFIED: i32 = 1;
pub const XPF_PROCESSOR_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdc3ea0b0_a144_4797_b95b_53fa242b6e1d);
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XPF_RECOVERY_INFO {
    pub FailureReason: XPF_RECOVERY_INFO_0,
    pub Action: XPF_RECOVERY_INFO_1,
    pub ActionRequired: super::BOOLEAN,
    pub RecoverySucceeded: super::BOOLEAN,
    pub RecoveryKernel: super::BOOLEAN,
    pub Reserved: super::UINT8,
    pub Reserved2: super::UINT16,
    pub Reserved3: super::UINT16,
    pub Reserved4: super::UINT32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XPF_RECOVERY_INFO_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct XPF_RECOVERY_INFO_1 {
    pub _bitfield: u32,
}
pub const XPF_TLB_CHECK_OPERATION_DATAREAD: i32 = 3;
pub const XPF_TLB_CHECK_OPERATION_DATAWRITE: i32 = 4;
pub const XPF_TLB_CHECK_OPERATION_GENERIC: i32 = 0;
pub const XPF_TLB_CHECK_OPERATION_GENREAD: i32 = 1;
pub const XPF_TLB_CHECK_OPERATION_GENWRITE: i32 = 2;
pub const XPF_TLB_CHECK_OPERATION_INSTRUCTIONFETCH: i32 = 5;
pub const XPF_TLB_CHECK_OPERATION_PREFETCH: i32 = 6;
pub const XPF_TLB_CHECK_TRANSACTIONTYPE_DATAACCESS: i32 = 1;
pub const XPF_TLB_CHECK_TRANSACTIONTYPE_GENERIC: i32 = 2;
pub const XPF_TLB_CHECK_TRANSACTIONTYPE_INSTRUCTION: i32 = 0;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct ZONE_HEADER {
    pub FreeList: super::SINGLE_LIST_ENTRY,
    pub SegmentList: super::SINGLE_LIST_ENTRY,
    pub BlockSize: super::ULONG,
    pub TotalSegmentSize: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct ZONE_SEGMENT_HEADER {
    pub SegmentList: super::SINGLE_LIST_ENTRY,
    pub Reserved: super::PVOID,
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
pub struct _ECP_LIST(pub u8);
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
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct _KGDTENTRY(pub u8);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct _KIDTENTRY(pub u8);
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct _KPRCB(pub u8);
#[repr(C)]
#[cfg(target_arch = "x86")]
#[derive(Clone, Copy, Default)]
pub struct _KTSS(pub u8);
#[repr(C)]
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[derive(Clone, Copy, Default)]
pub struct _KTSS64(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _LOADER_PARAMETER_BLOCK(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct _SILO_MONITOR(pub u8);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type _WHEA_ERROR_SOURCE_CORRECT = Option<unsafe extern "system" fn(errorsource: PWHEA_ERROR_SOURCE_DESCRIPTOR, maximumsectionlength: super::PULONG) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type _WHEA_ERROR_SOURCE_CORRECT_DEVICE_DRIVER = Option<unsafe extern "system" fn(errorsourcedesc: super::PVOID, maximumsectionlength: super::PULONG) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type _WHEA_ERROR_SOURCE_CREATE_RECORD = Option<unsafe extern "system" fn(errorsource: PWHEA_ERROR_SOURCE_DESCRIPTOR, errorpacket: PWHEA_ERROR_PACKET, errorrecord: PWHEA_ERROR_RECORD, buffersize: super::ULONG, context: super::PVOID) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type _WHEA_ERROR_SOURCE_INITIALIZE = Option<unsafe extern "system" fn(phase: super::ULONG, errorsource: PWHEA_ERROR_SOURCE_DESCRIPTOR, context: super::PVOID) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type _WHEA_ERROR_SOURCE_INITIALIZE_DEVICE_DRIVER = Option<unsafe extern "system" fn(context: super::PVOID, errorsourceid: super::ULONG) -> windows_sys::core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type _WHEA_ERROR_SOURCE_RECOVER = Option<unsafe extern "system" fn(recoverycontext: super::PVOID, severity: PWHEA_ERROR_SEVERITY) -> windows_sys::core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type _WHEA_ERROR_SOURCE_UNINITIALIZE = Option<unsafe extern "system" fn(context: super::PVOID)>;
#[cfg(feature = "winnt")]
pub type _WHEA_ERROR_SOURCE_UNINITIALIZE_DEVICE_DRIVER = Option<unsafe extern "system" fn(context: super::PVOID)>;
#[cfg(feature = "shared")]
pub type _WHEA_SIGNAL_HANDLER_OVERRIDE_CALLBACK = Option<unsafe extern "system" fn(context: super::UINT_PTR) -> super::BOOLEAN>;
#[cfg(all(feature = "basetsd", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type pHalAssignSlotResources = Option<unsafe extern "system" fn(registrypath: super::PUNICODE_STRING, driverclassname: super::PUNICODE_STRING, driverobject: super::PDRIVER_OBJECT, deviceobject: super::PDEVICE_OBJECT, bustype: super::INTERFACE_TYPE, busnumber: super::ULONG, slotnumber: super::ULONG, allocatedresources: *mut super::PCM_RESOURCE_LIST) -> windows_sys::core::NTSTATUS>;
#[cfg(feature = "shared")]
pub type pHalEndMirroring = Option<unsafe extern "system" fn(passnumber: super::ULONG) -> windows_sys::core::NTSTATUS>;
pub type pHalEndOfBoot = Option<unsafe extern "system" fn()>;
pub type pHalFfaMsgSendDirectReq2 = Option<unsafe extern "system" fn(parameters: *mut _FFA_MSG_SEND_DIRECT_REQ2_PARAMETERS) -> windows_sys::core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type pHalFfaRegisterNotification = Option<unsafe extern "system" fn(registrationparameters: *const _FFA_NOTIFICATION_REGISTRATION_PARAMETERS, token: *mut super::PVOID) -> windows_sys::core::NTSTATUS>;
pub type pHalFfaRunTarget = Option<unsafe extern "system" fn(inputparameters: *const _FFA_RUN_TARGET_INPUT_PARAMETERS, outputparameters: *mut _FFA_RUN_TARGET_OUTPUT_PARAMETERS) -> windows_sys::core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type pHalFfaUnregisterNotification = Option<unsafe extern "system" fn(token: super::PVOID) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "ntdef", feature = "shared", feature = "winnt"))]
pub type pHalFindBusAddressTranslation = Option<unsafe extern "system" fn(busaddress: super::PHYSICAL_ADDRESS, addressspace: super::PULONG, translatedaddress: super::PPHYSICAL_ADDRESS, context: super::PULONG_PTR, nextbus: super::BOOLEAN) -> super::BOOLEAN>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type pHalGetAcpiTable = Option<unsafe extern "system" fn(signature: super::ULONG, oemid: windows_sys::core::PCSTR, oemtableid: windows_sys::core::PCSTR) -> super::PVOID>;
#[cfg(all(feature = "basetsd", feature = "minwindef", feature = "ntdef", feature = "ntifs", feature = "shared", feature = "usb", feature = "wdm", feature = "winnt", feature = "winternl"))]
pub type pHalGetInterruptTranslator = Option<unsafe extern "system" fn(parentinterfacetype: super::INTERFACE_TYPE, parentbusnumber: super::ULONG, bridgeinterfacetype: super::INTERFACE_TYPE, size: super::USHORT, version: super::USHORT, translator: PTRANSLATOR_INTERFACE, bridgebusnumber: super::PULONG) -> windows_sys::core::NTSTATUS>;
#[cfg(feature = "winnt")]
pub type pHalGetPrmCache = Option<unsafe extern "system" fn(firmwarelist: *mut super::PLIST_ENTRY, updatelist: *mut super::PLIST_ENTRY)>;
pub type pHalHaltSystem = Option<unsafe extern "system" fn()>;
pub type pHalInitPnpDriver = Option<unsafe extern "system" fn() -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type pHalInitPowerManagement = Option<unsafe extern "system" fn(pmdriverdispatchtable: PPM_DISPATCH_TABLE, pmhaldispatchtable: *mut PPM_DISPATCH_TABLE) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "guiddef", feature = "winnt"))]
pub type pHalInvokePrmFwHandler = Option<unsafe extern "system" fn(handlerguid: super::LPGUID, parameterbuffer: super::PVOID, contextbuffer: super::PVOID) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type pHalMirrorPhysicalMemory = Option<unsafe extern "system" fn(physicaladdress: super::PHYSICAL_ADDRESS, numberofbytes: super::LARGE_INTEGER) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type pHalMirrorVerify = Option<unsafe extern "system" fn(physicaladdress: super::PHYSICAL_ADDRESS, numberofbytes: super::LARGE_INTEGER) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type pHalQueryBusSlots = Option<unsafe extern "system" fn(bushandler: super::PBUS_HANDLER, buffersize: super::ULONG, slotnumbers: super::PULONG, returnedlength: super::PULONG) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type pHalQuerySystemInformation = Option<unsafe extern "system" fn(informationclass: HAL_QUERY_INFORMATION_CLASS, buffersize: super::ULONG, buffer: super::PVOID, returnedlength: super::PULONG) -> windows_sys::core::NTSTATUS>;
#[cfg(feature = "shared")]
pub type pHalResetDisplay = Option<unsafe extern "system" fn() -> super::BOOLEAN>;
pub type pHalSetPciErrorHandlerCallback = Option<unsafe extern "system" fn(callback: PCI_ERROR_HANDLER_CALLBACK)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type pHalSetSystemInformation = Option<unsafe extern "system" fn(informationclass: HAL_SET_INFORMATION_CLASS, buffersize: super::ULONG, buffer: super::PVOID) -> windows_sys::core::NTSTATUS>;
pub type pHalStartMirroring = Option<unsafe extern "system" fn() -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "ntdef", feature = "shared", feature = "wdm", feature = "winnt"))]
pub type pHalTranslateBusAddress = Option<unsafe extern "system" fn(interfacetype: super::INTERFACE_TYPE, busnumber: super::ULONG, busaddress: super::PHYSICAL_ADDRESS, addressspace: super::PULONG, translatedaddress: super::PPHYSICAL_ADDRESS) -> super::BOOLEAN>;
#[cfg(feature = "shared")]
pub type pHalVectorToIDTEntry = Option<unsafe extern "system" fn(vector: super::ULONG) -> super::UCHAR>;
pub type pKdCheckPowerButton = Option<unsafe extern "system" fn()>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type pKdEnumerateDebuggingDevices = Option<unsafe extern "system" fn(loaderblock: super::PVOID, device: PDEBUG_DEVICE_DESCRIPTOR, callback: PDEBUG_DEVICE_FOUND_FUNCTION) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type pKdGetAcpiTablePhase0 = Option<unsafe extern "system" fn(loaderblock: *const _LOADER_PARAMETER_BLOCK, signature: super::ULONG) -> super::PVOID>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type pKdGetPciDataByOffset = Option<unsafe extern "system" fn(busnumber: super::ULONG, slotnumber: super::ULONG, buffer: super::PVOID, offset: super::ULONG, length: super::ULONG) -> super::ULONG>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type pKdMapPhysicalMemory64 = Option<unsafe extern "system" fn(physicaladdress: super::PHYSICAL_ADDRESS, numberpages: super::ULONG, flushcurrenttlb: super::BOOLEAN) -> super::PVOID>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type pKdReleaseIntegratedDeviceForDebugging = Option<unsafe extern "system" fn(integrateddevice: PDEBUG_DEVICE_DESCRIPTOR) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type pKdReleasePciDeviceForDebugging = Option<unsafe extern "system" fn(pcidevice: PDEBUG_DEVICE_DESCRIPTOR) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type pKdSetPciDataByOffset = Option<unsafe extern "system" fn(busnumber: super::ULONG, slotnumber: super::ULONG, buffer: super::PVOID, offset: super::ULONG, length: super::ULONG) -> super::ULONG>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type pKdSetupIntegratedDeviceForDebugging = Option<unsafe extern "system" fn(loaderblock: super::PVOID, integrateddevice: PDEBUG_DEVICE_DESCRIPTOR) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type pKdSetupPciDeviceForDebugging = Option<unsafe extern "system" fn(loaderblock: super::PVOID, pcidevice: PDEBUG_DEVICE_DESCRIPTOR) -> windows_sys::core::NTSTATUS>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type pKdUnmapVirtualAddress = Option<unsafe extern "system" fn(virtualaddress: super::PVOID, numberpages: super::ULONG, flushcurrenttlb: super::BOOLEAN)>;
