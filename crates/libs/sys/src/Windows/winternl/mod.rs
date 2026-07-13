#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn NtClose(handle : super::winnt::HANDLE) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn NtCreateFile(filehandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *mut super::d3dkmthk::OBJECT_ATTRIBUTES, iostatusblock : *mut IO_STATUS_BLOCK, allocationsize : *mut i64, fileattributes : u32, shareaccess : u32, createdisposition : u32, createoptions : u32, eabuffer : *mut core::ffi::c_void, ealength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn NtDeviceIoControlFile(filehandle : super::winnt::HANDLE, event : super::winnt::HANDLE, apcroutine : PIO_APC_ROUTINE, apccontext : *mut core::ffi::c_void, iostatusblock : *mut IO_STATUS_BLOCK, iocontrolcode : u32, inputbuffer : *mut core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn NtNotifyChangeMultipleKeys(masterkeyhandle : super::winnt::HANDLE, count : u32, subordinateobjects : *const super::d3dkmthk::OBJECT_ATTRIBUTES, event : super::winnt::HANDLE, apcroutine : PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut IO_STATUS_BLOCK, completionfilter : u32, watchtree : bool, buffer : *mut core::ffi::c_void, buffersize : u32, asynchronous : bool) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "d3dkmthk", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn NtOpenFile(filehandle : *mut super::winnt::HANDLE, desiredaccess : super::winnt::ACCESS_MASK, objectattributes : *mut super::d3dkmthk::OBJECT_ATTRIBUTES, iostatusblock : *mut IO_STATUS_BLOCK, shareaccess : u32, openoptions : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn NtQueryInformationProcess(processhandle : super::winnt::HANDLE, processinformationclass : PROCESSINFOCLASS, processinformation : *mut core::ffi::c_void, processinformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn NtQueryInformationThread(threadhandle : super::winnt::HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *mut core::ffi::c_void, threadinformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn NtQueryMultipleValueKey(keyhandle : super::winnt::HANDLE, valueentries : *mut KEY_VALUE_ENTRY, entrycount : u32, valuebuffer : *mut core::ffi::c_void, bufferlength : *mut u32, requiredbufferlength : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn NtQueryObject(handle : super::winnt::HANDLE, objectinformationclass : OBJECT_INFORMATION_CLASS, objectinformation : *mut core::ffi::c_void, objectinformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "bcrypt")]
windows_link::link!("ntdll.dll" "system" fn NtQuerySystemInformation(systeminformationclass : SYSTEM_INFORMATION_CLASS, systeminformation : *mut core::ffi::c_void, systeminformationlength : u32, returnlength : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "bcrypt")]
windows_link::link!("ntdll.dll" "system" fn NtQuerySystemTime(systemtime : *mut i64) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "bcrypt")]
windows_link::link!("ntdll.dll" "system" fn NtQueryTimerResolution(maximumtime : *mut u32, minimumtime : *mut u32, currenttime : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn NtRenameKey(keyhandle : super::winnt::HANDLE, newname : *const super::ntsecapi::UNICODE_STRING) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn NtSetInformationKey(keyhandle : super::winnt::HANDLE, keysetinformationclass : KEY_SET_INFORMATION_CLASS, keysetinformation : *const core::ffi::c_void, keysetinformationlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn NtSetInformationThread(threadhandle : super::winnt::HANDLE, threadinformationclass : THREADINFOCLASS, threadinformation : *const core::ffi::c_void, threadinformationlength : u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn NtWaitForSingleObject(handle : super::winnt::HANDLE, alertable : bool, timeout : *mut i64) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlAnsiStringToUnicodeString(destinationstring : *mut super::ntsecapi::UNICODE_STRING, sourcestring : PCANSI_STRING, allocatedestinationstring : bool) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "bcrypt")]
windows_link::link!("ntdll.dll" "system" fn RtlCharToInteger(string : *const i8, base : u32, value : *mut u32) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlConvertSidToUnicodeString(unicodestring : *mut super::ntsecapi::UNICODE_STRING, sid : super::winnt::PSID, allocatedestinationstring : bool) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlFreeAnsiString(ansistring : PANSI_STRING));
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlFreeOemString(oemstring : POEM_STRING));
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
windows_link::link!("ntdll.dll" "system" fn RtlFreeUnicodeString(unicodestring : *mut super::ntsecapi::UNICODE_STRING));
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInitAnsiString(destinationstring : PANSI_STRING, sourcestring : *const i8));
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInitAnsiStringEx(destinationstring : PANSI_STRING, sourcestring : *const i8) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInitString(destinationstring : *mut super::ntsecapi::STRING, sourcestring : *const i8));
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlInitStringEx(destinationstring : *mut super::ntsecapi::STRING, sourcestring : *const i8) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
windows_link::link!("ntdll.dll" "system" fn RtlInitUnicodeString(destinationstring : *mut super::ntsecapi::UNICODE_STRING, sourcestring : windows_sys::core::PCWSTR));
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlIsNameLegalDOS8Dot3(name : *mut super::ntsecapi::UNICODE_STRING, oemname : POEM_STRING, namecontainsspaces : *mut bool) -> bool);
#[cfg(feature = "bcrypt")]
windows_link::link!("ntdll.dll" "system" fn RtlLocalTimeToSystemTime(localtime : *mut i64, systemtime : *mut i64) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "bcrypt")]
windows_link::link!("ntdll.dll" "system" fn RtlNtStatusToDosError(status : super::bcrypt::NTSTATUS) -> u32);
windows_link::link!("ntdll.dll" "system" fn RtlTimeToSecondsSince1970(time : *mut i64, elapsedseconds : *mut u32) -> bool);
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlUnicodeStringToAnsiString(destinationstring : PANSI_STRING, sourcestring : *const super::ntsecapi::UNICODE_STRING, allocatedestinationstring : bool) -> super::bcrypt::NTSTATUS);
#[cfg(all(feature = "bcrypt", feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
windows_link::link!("ntdll.dll" "system" fn RtlUnicodeStringToOemString(destinationstring : POEM_STRING, sourcestring : *const super::ntsecapi::UNICODE_STRING, allocatedestinationstring : bool) -> super::bcrypt::NTSTATUS);
#[cfg(feature = "bcrypt")]
windows_link::link!("ntdll.dll" "system" fn RtlUnicodeToMultiByteSize(bytesinmultibytestring : *mut u32, unicodestring : *const u16, bytesinunicodestring : u32) -> super::bcrypt::NTSTATUS);
windows_link::link!("ntdll.dll" "system" fn RtlUniform(seed : *mut u32) -> u32);
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type ANSI_STRING = super::ntsecapi::STRING;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct CLIENT_ID {
    pub UniqueProcess: super::winnt::HANDLE,
    pub UniqueThread: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for CLIENT_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CODEINTEGRITY_OPTION_DEBUGMODE_ENABLED: u32 = 128;
pub const CODEINTEGRITY_OPTION_ENABLED: u32 = 1;
pub const CODEINTEGRITY_OPTION_FLIGHTING_ENABLED: u32 = 512;
pub const CODEINTEGRITY_OPTION_FLIGHT_BUILD: u32 = 256;
pub const CODEINTEGRITY_OPTION_HVCI_IUM_ENABLED: u32 = 8192;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_AUDITMODE_ENABLED: u32 = 2048;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_ENABLED: u32 = 1024;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_STRICTMODE_ENABLED: u32 = 4096;
pub const CODEINTEGRITY_OPTION_PREPRODUCTION_BUILD: u32 = 64;
pub const CODEINTEGRITY_OPTION_TESTSIGN: u32 = 2;
pub const CODEINTEGRITY_OPTION_TEST_BUILD: u32 = 32;
pub const CODEINTEGRITY_OPTION_UMCI_AUDITMODE_ENABLED: u32 = 8;
pub const CODEINTEGRITY_OPTION_UMCI_ENABLED: u32 = 4;
pub const CODEINTEGRITY_OPTION_UMCI_EXCLUSIONPATHS_ENABLED: u32 = 16;
pub const FILE_COMPLETE_IF_OPLOCKED: u32 = 256;
pub const FILE_CREATE: u32 = 2;
pub const FILE_CREATED: u32 = 2;
pub const FILE_CREATE_TREE_CONNECTION: u32 = 128;
pub const FILE_DELETE_ON_CLOSE: u32 = 4096;
pub const FILE_DIRECTORY_FILE: u32 = 1;
pub const FILE_DOES_NOT_EXIST: u32 = 5;
pub const FILE_EXISTS: u32 = 4;
pub type FILE_INFORMATION_CLASS = i32;
pub const FILE_MAXIMUM_DISPOSITION: u32 = 5;
pub const FILE_NON_DIRECTORY_FILE: u32 = 64;
pub const FILE_NO_COMPRESSION: u32 = 32768;
pub const FILE_NO_EA_KNOWLEDGE: u32 = 512;
pub const FILE_NO_INTERMEDIATE_BUFFERING: u32 = 8;
pub const FILE_OPEN: u32 = 1;
pub const FILE_OPENED: u32 = 1;
pub const FILE_OPEN_BY_FILE_ID: u32 = 8192;
pub const FILE_OPEN_FOR_BACKUP_INTENT: u32 = 16384;
pub const FILE_OPEN_FOR_FREE_SPACE_QUERY: u32 = 8388608;
pub const FILE_OPEN_IF: u32 = 3;
pub const FILE_OPEN_NO_RECALL: u32 = 4194304;
pub const FILE_OPEN_REMOTE_INSTANCE: u32 = 1024;
pub const FILE_OPEN_REPARSE_POINT: u32 = 2097152;
pub const FILE_OPEN_REQUIRING_OPLOCK: u32 = 65536;
pub const FILE_OVERWRITE: u32 = 4;
pub const FILE_OVERWRITE_IF: u32 = 5;
pub const FILE_OVERWRITTEN: u32 = 3;
pub const FILE_RANDOM_ACCESS: u32 = 2048;
pub const FILE_RESERVE_OPFILTER: u32 = 1048576;
pub const FILE_SEQUENTIAL_ONLY: u32 = 4;
pub const FILE_SUPERSEDE: u32 = 0;
pub const FILE_SUPERSEDED: u32 = 0;
pub const FILE_SYNCHRONOUS_IO_ALERT: u32 = 16;
pub const FILE_SYNCHRONOUS_IO_NONALERT: u32 = 32;
pub const FILE_VALID_MAILSLOT_OPTION_FLAGS: u32 = 50;
pub const FILE_VALID_OPTION_FLAGS: u32 = 16777215;
pub const FILE_VALID_PIPE_OPTION_FLAGS: u32 = 50;
pub const FILE_VALID_SET_FLAGS: u32 = 54;
pub const FILE_WRITE_THROUGH: u32 = 2;
pub const FileDirectoryInformation: FILE_INFORMATION_CLASS = 1;
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy)]
pub struct IO_STATUS_BLOCK {
    pub Anonymous: IO_STATUS_BLOCK_0,
    pub Information: usize,
}
#[cfg(feature = "bcrypt")]
impl Default for IO_STATUS_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "bcrypt")]
#[derive(Clone, Copy)]
pub union IO_STATUS_BLOCK_0 {
    pub Status: super::bcrypt::NTSTATUS,
    pub Pointer: *mut core::ffi::c_void,
}
#[cfg(feature = "bcrypt")]
impl Default for IO_STATUS_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KEY_SET_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy)]
pub struct KEY_VALUE_ENTRY {
    pub ValueName: super::ntsecapi::PUNICODE_STRING,
    pub DataLength: u32,
    pub DataOffset: u32,
    pub Type: u32,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
impl Default for KEY_VALUE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KPRIORITY = i32;
pub const KeyControlFlagsInformation: KEY_SET_INFORMATION_CLASS = 2;
pub const KeySetDebugInformation: KEY_SET_INFORMATION_CLASS = 4;
pub const KeySetHandleTagsInformation: KEY_SET_INFORMATION_CLASS = 5;
pub const KeySetVirtualizationInformation: KEY_SET_INFORMATION_CLASS = 3;
pub const KeyWow64FlagsInformation: KEY_SET_INFORMATION_CLASS = 1;
pub const KeyWriteTimeInformation: KEY_SET_INFORMATION_CLASS = 0;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct LDR_DATA_TABLE_ENTRY {
    pub Reserved1: [*mut core::ffi::c_void; 2],
    pub InMemoryOrderLinks: super::winnt::LIST_ENTRY,
    pub Reserved2: [*mut core::ffi::c_void; 2],
    pub DllBase: *mut core::ffi::c_void,
    pub Reserved3: [*mut core::ffi::c_void; 2],
    pub FullDllName: super::ntsecapi::UNICODE_STRING,
    pub Reserved4: [u8; 8],
    pub Reserved5: [*mut core::ffi::c_void; 3],
    pub Anonymous: LDR_DATA_TABLE_ENTRY_0,
    pub TimeDateStamp: u32,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for LDR_DATA_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union LDR_DATA_TABLE_ENTRY_0 {
    pub CheckSum: u32,
    pub Reserved6: *mut core::ffi::c_void,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for LDR_DATA_TABLE_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LOGONID_CURRENT: u32 = 4294967295;
pub const MaxKeySetInfoClass: KEY_SET_INFORMATION_CLASS = 6;
pub type OBJECT_INFORMATION_CLASS = i32;
pub const OBJ_CASE_INSENSITIVE: u32 = 64;
pub const OBJ_DONT_REPARSE: u32 = 4096;
pub const OBJ_EXCLUSIVE: u32 = 32;
pub const OBJ_FORCE_ACCESS_CHECK: u32 = 1024;
pub const OBJ_IGNORE_IMPERSONATED_DEVICEMAP: u32 = 2048;
pub const OBJ_INHERIT: u32 = 2;
pub const OBJ_KERNEL_HANDLE: u32 = 512;
pub const OBJ_OPENIF: u32 = 128;
pub const OBJ_OPENLINK: u32 = 256;
pub const OBJ_PERMANENT: u32 = 16;
pub const OBJ_VALID_ATTRIBUTES: u32 = 8178;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type OEM_STRING = super::ntsecapi::STRING;
pub const ObjectBasicInformation: OBJECT_INFORMATION_CLASS = 0;
pub const ObjectTypeInformation: OBJECT_INFORMATION_CLASS = 2;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PANSI_STRING = super::ntsecapi::PSTRING;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PCANSI_STRING = super::ntsecapi::PSTRING;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PCOEM_STRING = *const super::ntsecapi::STRING;
pub type PCSZ = *const i8;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PCUNICODE_STRING = *const super::ntsecapi::UNICODE_STRING;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PEB {
    pub Reserved1: [u8; 2],
    pub BeingDebugged: u8,
    pub Reserved2: [u8; 1],
    pub Reserved3: [*mut core::ffi::c_void; 2],
    pub Ldr: PPEB_LDR_DATA,
    pub ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
    pub Reserved4: [*mut core::ffi::c_void; 3],
    pub AtlThunkSListPtr: *mut core::ffi::c_void,
    pub Reserved5: *mut core::ffi::c_void,
    pub Reserved6: u32,
    pub Reserved7: *mut core::ffi::c_void,
    pub Reserved8: u32,
    pub AtlThunkSListPtr32: u32,
    pub Reserved9: [*mut core::ffi::c_void; 45],
    pub Reserved10: [u8; 96],
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    pub Reserved11: [u8; 128],
    pub Reserved12: [*mut core::ffi::c_void; 1],
    pub SessionId: u32,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for PEB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct PEB_LDR_DATA {
    pub Reserved1: [u8; 8],
    pub Reserved2: [*mut core::ffi::c_void; 3],
    pub InMemoryOrderModuleList: super::winnt::LIST_ENTRY,
}
#[cfg(feature = "winnt")]
impl Default for PEB_LDR_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "bcrypt")]
pub type PIO_APC_ROUTINE = Option<unsafe extern "system" fn(apccontext: *mut core::ffi::c_void, iostatusblock: *mut IO_STATUS_BLOCK, reserved: u32)>;
#[cfg(feature = "bcrypt")]
pub type PIO_STATUS_BLOCK = *mut IO_STATUS_BLOCK;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PKEY_VALUE_ENTRY = *mut KEY_VALUE_ENTRY;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PLDR_DATA_TABLE_ENTRY = *mut LDR_DATA_TABLE_ENTRY;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type POEM_STRING = super::ntsecapi::PSTRING;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PPEB = *mut PEB;
#[cfg(feature = "winnt")]
pub type PPEB_LDR_DATA = *mut PEB_LDR_DATA;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PPROCESS_BASIC_INFORMATION = *mut PROCESS_BASIC_INFORMATION;
pub type PPS_POST_PROCESS_INIT_ROUTINE = Option<unsafe extern "system" fn()>;
#[cfg(feature = "winnt")]
pub type PPUBLIC_OBJECT_BASIC_INFORMATION = *mut PUBLIC_OBJECT_BASIC_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PPUBLIC_OBJECT_TYPE_INFORMATION = *mut PUBLIC_OBJECT_TYPE_INFORMATION;
pub type PROCESSINFOCLASS = i32;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct PROCESS_BASIC_INFORMATION {
    pub Reserved1: *mut core::ffi::c_void,
    pub PebBaseAddress: PPEB,
    pub Reserved2: [*mut core::ffi::c_void; 2],
    pub UniqueProcessId: usize,
    pub Reserved3: *mut core::ffi::c_void,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for PROCESS_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PRTL_USER_PROCESS_PARAMETERS = *mut RTL_USER_PROCESS_PARAMETERS;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PSYSTEM_BASICPROCESS_INFORMATION = *mut SYSTEM_BASICPROCESS_INFORMATION;
#[cfg(feature = "winnt")]
pub type PSYSTEM_BASIC_INFORMATION = *mut SYSTEM_BASIC_INFORMATION;
pub type PSYSTEM_CODEINTEGRITY_INFORMATION = *mut SYSTEM_CODEINTEGRITY_INFORMATION;
pub type PSYSTEM_EXCEPTION_INFORMATION = *mut SYSTEM_EXCEPTION_INFORMATION;
pub type PSYSTEM_HANDLECOUNT_INFORMATION = *mut SYSTEM_HANDLECOUNT_INFORMATION;
pub type PSYSTEM_INTERRUPT_INFORMATION = *mut SYSTEM_INTERRUPT_INFORMATION;
pub type PSYSTEM_LOOKASIDE_INFORMATION = *mut SYSTEM_LOOKASIDE_INFORMATION;
pub type PSYSTEM_PERFORMANCE_INFORMATION = *mut SYSTEM_PERFORMANCE_INFORMATION;
pub type PSYSTEM_POLICY_INFORMATION = *mut SYSTEM_POLICY_INFORMATION;
pub type PSYSTEM_PROCESSOR_PERFORMANCE_INFORMATION = *mut SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PSYSTEM_PROCESS_INFORMATION = *mut SYSTEM_PROCESS_INFORMATION;
pub type PSYSTEM_REGISTRY_QUOTA_INFORMATION = *mut SYSTEM_REGISTRY_QUOTA_INFORMATION;
#[cfg(feature = "winnt")]
pub type PSYSTEM_THREAD_INFORMATION = *mut SYSTEM_THREAD_INFORMATION;
pub type PSYSTEM_TIMEOFDAY_INFORMATION = *mut SYSTEM_TIMEOFDAY_INFORMATION;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
pub type PTEB = *mut TEB;
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
pub type PTHREAD_NAME_INFORMATION = *mut THREAD_NAME_INFORMATION;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct PUBLIC_OBJECT_BASIC_INFORMATION {
    pub Attributes: u32,
    pub GrantedAccess: super::winnt::ACCESS_MASK,
    pub HandleCount: u32,
    pub PointerCount: u32,
    pub Reserved: [u32; 10],
}
#[cfg(feature = "winnt")]
impl Default for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy)]
pub struct PUBLIC_OBJECT_TYPE_INFORMATION {
    pub TypeName: super::ntsecapi::UNICODE_STRING,
    pub Reserved: [u32; 22],
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
impl Default for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PWINSTATIONINFORMATIONW = *mut WINSTATIONINFORMATIONW;
#[cfg(feature = "winnt")]
pub type PWINSTATIONQUERYINFORMATIONW = Option<unsafe extern "system" fn(param0: super::winnt::HANDLE, param1: u32, param2: WINSTATIONINFOCLASS, param3: *mut core::ffi::c_void, param4: u32, param5: *mut u32) -> bool>;
pub const ProcessBasicInformation: PROCESSINFOCLASS = 0;
pub const ProcessBreakOnTermination: PROCESSINFOCLASS = 29;
pub const ProcessDebugPort: PROCESSINFOCLASS = 7;
pub const ProcessImageFileName: PROCESSINFOCLASS = 27;
pub const ProcessWow64Information: PROCESSINFOCLASS = 26;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy)]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub Reserved1: [u8; 16],
    pub Reserved2: [*mut core::ffi::c_void; 10],
    pub ImagePathName: super::ntsecapi::UNICODE_STRING,
    pub CommandLine: super::ntsecapi::UNICODE_STRING,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
impl Default for RTL_USER_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SYSTEM_BASICPROCESS_INFORMATION {
    pub NextEntryOffset: u32,
    pub UniqueProcessId: super::winnt::HANDLE,
    pub InheritedFromUniqueProcessId: super::winnt::HANDLE,
    pub SequenceNumber: u64,
    pub ImageName: super::ntsecapi::UNICODE_STRING,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for SYSTEM_BASICPROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct SYSTEM_BASIC_INFORMATION {
    pub Reserved1: [u8; 24],
    pub Reserved2: [*mut core::ffi::c_void; 4],
    pub NumberOfProcessors: super::winnt::CCHAR,
}
#[cfg(feature = "winnt")]
impl Default for SYSTEM_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SYSTEM_CODEINTEGRITY_INFORMATION {
    pub Length: u32,
    pub CodeIntegrityOptions: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_EXCEPTION_INFORMATION {
    pub Reserved1: [u8; 16],
}
impl Default for SYSTEM_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SYSTEM_HANDLECOUNT_INFORMATION {
    pub ProcessCount: u32,
    pub ThreadCount: u32,
    pub HandleCount: u32,
}
pub type SYSTEM_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_INTERRUPT_INFORMATION {
    pub Reserved1: [u8; 24],
}
impl Default for SYSTEM_INTERRUPT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_LOOKASIDE_INFORMATION {
    pub Reserved1: [u8; 32],
}
impl Default for SYSTEM_LOOKASIDE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_PERFORMANCE_INFORMATION {
    pub Reserved1: [u8; 312],
}
impl Default for SYSTEM_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_POLICY_INFORMATION {
    pub Reserved1: [*mut core::ffi::c_void; 2],
    pub Reserved2: [u32; 3],
}
impl Default for SYSTEM_POLICY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    pub IdleTime: i64,
    pub KernelTime: i64,
    pub UserTime: i64,
    pub Reserved1: [i64; 2],
    pub Reserved2: u32,
}
impl Default for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SYSTEM_PROCESS_INFORMATION {
    pub NextEntryOffset: u32,
    pub NumberOfThreads: u32,
    pub Reserved1: [u8; 48],
    pub ImageName: super::ntsecapi::UNICODE_STRING,
    pub BasePriority: KPRIORITY,
    pub UniqueProcessId: super::winnt::HANDLE,
    pub InheritedFromUniqueProcessId: super::winnt::HANDLE,
    pub HandleCount: u32,
    pub SessionId: u32,
    pub Reserved3: *mut core::ffi::c_void,
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub Reserved4: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub Reserved5: *mut core::ffi::c_void,
    pub QuotaPagedPoolUsage: usize,
    pub Reserved6: *mut core::ffi::c_void,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivatePageCount: usize,
    pub Reserved7: [i64; 6],
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for SYSTEM_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_REGISTRY_QUOTA_INFORMATION {
    pub RegistryQuotaAllowed: u32,
    pub RegistryQuotaUsed: u32,
    pub Reserved1: *mut core::ffi::c_void,
}
impl Default for SYSTEM_REGISTRY_QUOTA_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct SYSTEM_THREAD_INFORMATION {
    pub Reserved1: [i64; 3],
    pub Reserved2: u32,
    pub StartAddress: *mut core::ffi::c_void,
    pub ClientId: CLIENT_ID,
    pub Priority: KPRIORITY,
    pub BasePriority: i32,
    pub Reserved3: u32,
    pub ThreadState: u32,
    pub WaitReason: u32,
}
#[cfg(feature = "winnt")]
impl Default for SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_TIMEOFDAY_INFORMATION {
    pub Reserved1: [u8; 48],
}
impl Default for SYSTEM_TIMEOFDAY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const SystemBasicInformation: SYSTEM_INFORMATION_CLASS = 0;
pub const SystemBasicProcessInformation: SYSTEM_INFORMATION_CLASS = 252;
pub const SystemCodeIntegrityInformation: SYSTEM_INFORMATION_CLASS = 103;
pub const SystemExceptionInformation: SYSTEM_INFORMATION_CLASS = 33;
pub const SystemHandleCountInformation: SYSTEM_INFORMATION_CLASS = 253;
pub const SystemInterruptInformation: SYSTEM_INFORMATION_CLASS = 23;
pub const SystemLookasideInformation: SYSTEM_INFORMATION_CLASS = 45;
pub const SystemPerformanceInformation: SYSTEM_INFORMATION_CLASS = 2;
pub const SystemPolicyInformation: SYSTEM_INFORMATION_CLASS = 134;
pub const SystemProcessInformation: SYSTEM_INFORMATION_CLASS = 5;
pub const SystemProcessorPerformanceInformation: SYSTEM_INFORMATION_CLASS = 8;
pub const SystemRegistryQuotaInformation: SYSTEM_INFORMATION_CLASS = 37;
pub const SystemTimeOfDayInformation: SYSTEM_INFORMATION_CLASS = 3;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct TEB {
    pub Reserved1: [*mut core::ffi::c_void; 12],
    pub ProcessEnvironmentBlock: PPEB,
    pub Reserved2: [*mut core::ffi::c_void; 399],
    pub Reserved3: [u8; 1952],
    pub TlsSlots: [*mut core::ffi::c_void; 64],
    pub Reserved4: [u8; 8],
    pub Reserved5: [*mut core::ffi::c_void; 26],
    pub ReservedForOle: *mut core::ffi::c_void,
    pub Reserved6: [*mut core::ffi::c_void; 4],
    pub TlsExpansionSlots: *mut core::ffi::c_void,
}
#[cfg(all(feature = "lsalookup", feature = "ntsecapi", feature = "winnt"))]
impl Default for TEB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type THREADINFOCLASS = i32;
#[repr(C)]
#[cfg(all(feature = "lsalookup", feature = "ntsecapi"))]
#[derive(Clone, Copy, Default)]
pub struct THREAD_NAME_INFORMATION {
    pub ThreadName: super::ntsecapi::UNICODE_STRING,
}
pub const ThreadIsIoPending: THREADINFOCLASS = 16;
pub const ThreadNameInformation: THREADINFOCLASS = 38;
pub type WINSTATIONINFOCLASS = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WINSTATIONINFORMATIONW {
    pub Reserved2: [u8; 70],
    pub LogonId: u32,
    pub Reserved3: [u8; 1140],
}
impl Default for WINSTATIONINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WinStationInformation: WINSTATIONINFOCLASS = 8;
