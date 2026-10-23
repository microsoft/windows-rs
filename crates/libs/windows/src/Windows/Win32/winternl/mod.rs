#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NtClose(handle: super::HANDLE) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtClose(handle : super::HANDLE) -> windows_core::NTSTATUS);
    unsafe { NtClose(handle as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NtCreateFile(filehandle: super::PHANDLE, desiredaccess: super::ACCESS_MASK, objectattributes: POBJECT_ATTRIBUTES, iostatusblock: PIO_STATUS_BLOCK, allocationsize: super::PLARGE_INTEGER, fileattributes: super::ULONG, shareaccess: super::ULONG, createdisposition: super::ULONG, createoptions: super::ULONG, eabuffer: super::PVOID, ealength: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtCreateFile(filehandle : super::PHANDLE, desiredaccess : super::ACCESS_MASK, objectattributes : POBJECT_ATTRIBUTES, iostatusblock : PIO_STATUS_BLOCK, allocationsize : super::PLARGE_INTEGER, fileattributes : super::ULONG, shareaccess : super::ULONG, createdisposition : super::ULONG, createoptions : super::ULONG, eabuffer : super::PVOID, ealength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { NtCreateFile(filehandle as _, desiredaccess, objectattributes as _, iostatusblock as _, allocationsize as _, fileattributes, shareaccess, createdisposition, createoptions, eabuffer as _, ealength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NtDeviceIoControlFile(filehandle: super::HANDLE, event: super::HANDLE, apcroutine: PIO_APC_ROUTINE, apccontext: super::PVOID, iostatusblock: PIO_STATUS_BLOCK, iocontrolcode: super::ULONG, inputbuffer: super::PVOID, inputbufferlength: super::ULONG, outputbuffer: super::PVOID, outputbufferlength: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtDeviceIoControlFile(filehandle : super::HANDLE, event : super::HANDLE, apcroutine : PIO_APC_ROUTINE, apccontext : super::PVOID, iostatusblock : PIO_STATUS_BLOCK, iocontrolcode : super::ULONG, inputbuffer : super::PVOID, inputbufferlength : super::ULONG, outputbuffer : super::PVOID, outputbufferlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { NtDeviceIoControlFile(filehandle as _, event as _, apcroutine, apccontext as _, iostatusblock as _, iocontrolcode, inputbuffer as _, inputbufferlength, outputbuffer as _, outputbufferlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NtNotifyChangeMultipleKeys(masterkeyhandle: super::HANDLE, subordinateobjects: Option<&[OBJECT_ATTRIBUTES]>, event: Option<super::HANDLE>, apcroutine: PIO_APC_ROUTINE, apccontext: Option<super::PVOID>, iostatusblock: PIO_STATUS_BLOCK, completionfilter: super::ULONG, watchtree: super::BOOLEAN, buffer: Option<super::PVOID>, buffersize: super::ULONG, asynchronous: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtNotifyChangeMultipleKeys(masterkeyhandle : super::HANDLE, count : super::ULONG, subordinateobjects : *const OBJECT_ATTRIBUTES, event : super::HANDLE, apcroutine : PIO_APC_ROUTINE, apccontext : super::PVOID, iostatusblock : PIO_STATUS_BLOCK, completionfilter : super::ULONG, watchtree : super::BOOLEAN, buffer : super::PVOID, buffersize : super::ULONG, asynchronous : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { NtNotifyChangeMultipleKeys(masterkeyhandle, subordinateobjects.map_or(0, |slice| slice.len().try_into().unwrap()), subordinateobjects.map_or(core::ptr::null(), |slice| slice.as_ptr()), event.unwrap_or(core::mem::zeroed()) as _, apcroutine, apccontext.unwrap_or(core::mem::zeroed()) as _, iostatusblock as _, completionfilter, watchtree, buffer.unwrap_or(core::mem::zeroed()) as _, buffersize, asynchronous) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NtOpenFile(filehandle: super::PHANDLE, desiredaccess: super::ACCESS_MASK, objectattributes: POBJECT_ATTRIBUTES, iostatusblock: PIO_STATUS_BLOCK, shareaccess: super::ULONG, openoptions: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtOpenFile(filehandle : super::PHANDLE, desiredaccess : super::ACCESS_MASK, objectattributes : POBJECT_ATTRIBUTES, iostatusblock : PIO_STATUS_BLOCK, shareaccess : super::ULONG, openoptions : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { NtOpenFile(filehandle as _, desiredaccess, objectattributes as _, iostatusblock as _, shareaccess, openoptions) }
}
#[cfg(all(feature = "minwindef", feature = "ntddk", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NtQueryInformationProcess(processhandle: super::HANDLE, processinformationclass: super::PROCESSINFOCLASS, processinformation: super::PVOID, processinformationlength: super::ULONG, returnlength: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationProcess(processhandle : super::HANDLE, processinformationclass : super::PROCESSINFOCLASS, processinformation : super::PVOID, processinformationlength : super::ULONG, returnlength : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { NtQueryInformationProcess(processhandle as _, processinformationclass, processinformation as _, processinformationlength, returnlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "ntddk", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NtQueryInformationThread(threadhandle: super::HANDLE, threadinformationclass: super::THREADINFOCLASS, threadinformation: super::PVOID, threadinformationlength: super::ULONG, returnlength: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryInformationThread(threadhandle : super::HANDLE, threadinformationclass : super::THREADINFOCLASS, threadinformation : super::PVOID, threadinformationlength : super::ULONG, returnlength : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { NtQueryInformationThread(threadhandle as _, threadinformationclass, threadinformation as _, threadinformationlength, returnlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NtQueryMultipleValueKey(keyhandle: super::HANDLE, valueentries: &mut [KEY_VALUE_ENTRY], valuebuffer: super::PVOID, bufferlength: super::PULONG, requiredbufferlength: Option<super::PULONG>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryMultipleValueKey(keyhandle : super::HANDLE, valueentries : PKEY_VALUE_ENTRY, entrycount : super::ULONG, valuebuffer : super::PVOID, bufferlength : super::PULONG, requiredbufferlength : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { NtQueryMultipleValueKey(keyhandle, core::mem::transmute(valueentries.as_mut_ptr()), valueentries.len().try_into().unwrap(), valuebuffer as _, bufferlength as _, requiredbufferlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NtQueryObject(handle: Option<super::HANDLE>, objectinformationclass: OBJECT_INFORMATION_CLASS, objectinformation: Option<super::PVOID>, objectinformationlength: super::ULONG, returnlength: Option<super::PULONG>) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryObject(handle : super::HANDLE, objectinformationclass : OBJECT_INFORMATION_CLASS, objectinformation : super::PVOID, objectinformationlength : super::ULONG, returnlength : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { NtQueryObject(handle.unwrap_or(core::mem::zeroed()) as _, objectinformationclass, objectinformation.unwrap_or(core::mem::zeroed()) as _, objectinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NtQuerySystemInformation(systeminformationclass: SYSTEM_INFORMATION_CLASS, systeminformation: super::PVOID, systeminformationlength: super::ULONG, returnlength: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQuerySystemInformation(systeminformationclass : SYSTEM_INFORMATION_CLASS, systeminformation : super::PVOID, systeminformationlength : super::ULONG, returnlength : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { NtQuerySystemInformation(systeminformationclass, systeminformation as _, systeminformationlength, returnlength as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NtQuerySystemTime(systemtime: super::PLARGE_INTEGER) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQuerySystemTime(systemtime : super::PLARGE_INTEGER) -> windows_core::NTSTATUS);
    unsafe { NtQuerySystemTime(systemtime as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn NtQueryTimerResolution(maximumtime: super::PULONG, minimumtime: super::PULONG, currenttime: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtQueryTimerResolution(maximumtime : super::PULONG, minimumtime : super::PULONG, currenttime : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { NtQueryTimerResolution(maximumtime as _, minimumtime as _, currenttime as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NtRenameKey(keyhandle: super::HANDLE, newname: PUNICODE_STRING) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtRenameKey(keyhandle : super::HANDLE, newname : PUNICODE_STRING) -> windows_core::NTSTATUS);
    unsafe { NtRenameKey(keyhandle, newname) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NtSetInformationKey(keyhandle: super::HANDLE, keysetinformationclass: KEY_SET_INFORMATION_CLASS, keysetinformation: super::PVOID, keysetinformationlength: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationKey(keyhandle : super::HANDLE, keysetinformationclass : KEY_SET_INFORMATION_CLASS, keysetinformation : super::PVOID, keysetinformationlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { NtSetInformationKey(keyhandle, keysetinformationclass, keysetinformation, keysetinformationlength) }
}
#[cfg(all(feature = "ntddk", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NtSetInformationThread(threadhandle: super::HANDLE, threadinformationclass: super::THREADINFOCLASS, threadinformation: super::PVOID, threadinformationlength: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtSetInformationThread(threadhandle : super::HANDLE, threadinformationclass : super::THREADINFOCLASS, threadinformation : super::PVOID, threadinformationlength : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { NtSetInformationThread(threadhandle, threadinformationclass, threadinformation, threadinformationlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn NtWaitForSingleObject(handle: super::HANDLE, alertable: super::BOOLEAN, timeout: super::PLARGE_INTEGER) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn NtWaitForSingleObject(handle : super::HANDLE, alertable : super::BOOLEAN, timeout : super::PLARGE_INTEGER) -> windows_core::NTSTATUS);
    unsafe { NtWaitForSingleObject(handle as _, alertable, timeout as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RtlAnsiStringToUnicodeString(destinationstring: PUNICODE_STRING, sourcestring: PCANSI_STRING, allocatedestinationstring: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlAnsiStringToUnicodeString(destinationstring : PUNICODE_STRING, sourcestring : PCANSI_STRING, allocatedestinationstring : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlAnsiStringToUnicodeString(destinationstring as _, sourcestring as _, allocatedestinationstring) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn RtlCharToInteger(string: PCSZ, base: super::ULONG, value: super::PULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlCharToInteger(string : PCSZ, base : super::ULONG, value : super::PULONG) -> windows_core::NTSTATUS);
    unsafe { RtlCharToInteger(string as _, base, value as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RtlConvertSidToUnicodeString(unicodestring: PUNICODE_STRING, sid: super::PSID, allocatedestinationstring: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlConvertSidToUnicodeString(unicodestring : PUNICODE_STRING, sid : super::PSID, allocatedestinationstring : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlConvertSidToUnicodeString(unicodestring as _, sid as _, allocatedestinationstring) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RtlFreeAnsiString(ansistring: PANSI_STRING) {
    windows_core::link!("ntdll.dll" "system" fn RtlFreeAnsiString(ansistring : PANSI_STRING));
    unsafe { RtlFreeAnsiString(ansistring as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RtlFreeOemString(oemstring: POEM_STRING) {
    windows_core::link!("ntdll.dll" "system" fn RtlFreeOemString(oemstring : POEM_STRING));
    unsafe { RtlFreeOemString(oemstring as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RtlFreeUnicodeString(unicodestring: PUNICODE_STRING) {
    windows_core::link!("ntdll.dll" "system" fn RtlFreeUnicodeString(unicodestring : PUNICODE_STRING));
    unsafe { RtlFreeUnicodeString(unicodestring as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RtlInitAnsiString(destinationstring: PANSI_STRING, sourcestring: PCSZ) {
    windows_core::link!("ntdll.dll" "system" fn RtlInitAnsiString(destinationstring : PANSI_STRING, sourcestring : PCSZ));
    unsafe { RtlInitAnsiString(destinationstring as _, sourcestring as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RtlInitAnsiStringEx(destinationstring: PANSI_STRING, sourcestring: PCSZ) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlInitAnsiStringEx(destinationstring : PANSI_STRING, sourcestring : PCSZ) -> windows_core::NTSTATUS);
    unsafe { RtlInitAnsiStringEx(destinationstring as _, sourcestring as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RtlInitString(destinationstring: PSTRING, sourcestring: PCSZ) {
    windows_core::link!("ntdll.dll" "system" fn RtlInitString(destinationstring : PSTRING, sourcestring : PCSZ));
    unsafe { RtlInitString(destinationstring as _, sourcestring as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RtlInitStringEx(destinationstring: PSTRING, sourcestring: PCSZ) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlInitStringEx(destinationstring : PSTRING, sourcestring : PCSZ) -> windows_core::NTSTATUS);
    unsafe { RtlInitStringEx(destinationstring as _, sourcestring as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RtlInitUnicodeString(destinationstring: PUNICODE_STRING, sourcestring: &windows_core::PCWSTR) {
    windows_core::link!("ntdll.dll" "system" fn RtlInitUnicodeString(destinationstring : PUNICODE_STRING, sourcestring : windows_core::PCWSTR));
    unsafe { RtlInitUnicodeString(destinationstring as _, core::mem::transmute_copy(sourcestring)) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RtlIsNameLegalDOS8Dot3(name: PUNICODE_STRING, oemname: POEM_STRING, namecontainsspaces: super::PBOOLEAN) -> super::BOOLEAN {
    windows_core::link!("ntdll.dll" "system" fn RtlIsNameLegalDOS8Dot3(name : PUNICODE_STRING, oemname : POEM_STRING, namecontainsspaces : super::PBOOLEAN) -> super::BOOLEAN);
    unsafe { RtlIsNameLegalDOS8Dot3(name as _, oemname as _, namecontainsspaces as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RtlLocalTimeToSystemTime(localtime: super::PLARGE_INTEGER, systemtime: super::PLARGE_INTEGER) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlLocalTimeToSystemTime(localtime : super::PLARGE_INTEGER, systemtime : super::PLARGE_INTEGER) -> windows_core::NTSTATUS);
    unsafe { RtlLocalTimeToSystemTime(localtime as _, systemtime as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RtlNtStatusToDosError(status: windows_core::NTSTATUS) -> super::ULONG {
    windows_core::link!("ntdll.dll" "system" fn RtlNtStatusToDosError(status : windows_core::NTSTATUS) -> super::ULONG);
    unsafe { RtlNtStatusToDosError(status) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RtlTimeToSecondsSince1970(time: super::PLARGE_INTEGER, elapsedseconds: super::PULONG) -> super::BOOLEAN {
    windows_core::link!("ntdll.dll" "system" fn RtlTimeToSecondsSince1970(time : super::PLARGE_INTEGER, elapsedseconds : super::PULONG) -> super::BOOLEAN);
    unsafe { RtlTimeToSecondsSince1970(time as _, elapsedseconds as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUnicodeStringToAnsiString(destinationstring: PANSI_STRING, sourcestring: PCUNICODE_STRING, allocatedestinationstring: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeStringToAnsiString(destinationstring : PANSI_STRING, sourcestring : PCUNICODE_STRING, allocatedestinationstring : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlUnicodeStringToAnsiString(destinationstring as _, sourcestring as _, allocatedestinationstring) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUnicodeStringToOemString(destinationstring: POEM_STRING, sourcestring: PCUNICODE_STRING, allocatedestinationstring: super::BOOLEAN) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeStringToOemString(destinationstring : POEM_STRING, sourcestring : PCUNICODE_STRING, allocatedestinationstring : super::BOOLEAN) -> windows_core::NTSTATUS);
    unsafe { RtlUnicodeStringToOemString(destinationstring as _, sourcestring as _, allocatedestinationstring) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RtlUnicodeToMultiByteSize(bytesinmultibytestring: super::PULONG, unicodestring: super::PWCH, bytesinunicodestring: super::ULONG) -> windows_core::NTSTATUS {
    windows_core::link!("ntdll.dll" "system" fn RtlUnicodeToMultiByteSize(bytesinmultibytestring : super::PULONG, unicodestring : super::PWCH, bytesinunicodestring : super::ULONG) -> windows_core::NTSTATUS);
    unsafe { RtlUnicodeToMultiByteSize(bytesinmultibytestring as _, unicodestring, bytesinunicodestring) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn RtlUniform(seed: super::PULONG) -> super::ULONG {
    windows_core::link!("ntdll.dll" "system" fn RtlUniform(seed : super::PULONG) -> super::ULONG);
    unsafe { RtlUniform(seed as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type ANSI_STRING = STRING;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CLIENT_ID {
    pub UniqueProcess: super::HANDLE,
    pub UniqueThread: super::HANDLE,
}
pub const CODEINTEGRITY_OPTION_DEBUGMODE_ENABLED: i32 = 128;
pub const CODEINTEGRITY_OPTION_ENABLED: i32 = 1;
pub const CODEINTEGRITY_OPTION_FLIGHTING_ENABLED: i32 = 512;
pub const CODEINTEGRITY_OPTION_FLIGHT_BUILD: i32 = 256;
pub const CODEINTEGRITY_OPTION_HVCI_IUM_ENABLED: i32 = 8192;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_AUDITMODE_ENABLED: i32 = 2048;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_ENABLED: i32 = 1024;
pub const CODEINTEGRITY_OPTION_HVCI_KMCI_STRICTMODE_ENABLED: i32 = 4096;
pub const CODEINTEGRITY_OPTION_PREPRODUCTION_BUILD: i32 = 64;
pub const CODEINTEGRITY_OPTION_TESTSIGN: i32 = 2;
pub const CODEINTEGRITY_OPTION_TEST_BUILD: i32 = 32;
pub const CODEINTEGRITY_OPTION_UMCI_AUDITMODE_ENABLED: i32 = 8;
pub const CODEINTEGRITY_OPTION_UMCI_ENABLED: i32 = 4;
pub const CODEINTEGRITY_OPTION_UMCI_EXCLUSIONPATHS_ENABLED: i32 = 16;
pub const FILE_COMPLETE_IF_OPLOCKED: i32 = 256;
pub const FILE_CREATE: i32 = 2;
pub const FILE_CREATED: i32 = 2;
pub const FILE_CREATE_TREE_CONNECTION: i32 = 128;
pub const FILE_DELETE_ON_CLOSE: i32 = 4096;
pub const FILE_DIRECTORY_FILE: i32 = 1;
pub const FILE_DOES_NOT_EXIST: i32 = 5;
pub const FILE_EXISTS: i32 = 4;
pub const FILE_MAXIMUM_DISPOSITION: i32 = 5;
pub const FILE_NON_DIRECTORY_FILE: i32 = 64;
pub const FILE_NO_COMPRESSION: i32 = 32768;
pub const FILE_NO_EA_KNOWLEDGE: i32 = 512;
pub const FILE_NO_INTERMEDIATE_BUFFERING: i32 = 8;
pub const FILE_OPEN: i32 = 1;
pub const FILE_OPENED: i32 = 1;
pub const FILE_OPEN_BY_FILE_ID: i32 = 8192;
pub const FILE_OPEN_FOR_BACKUP_INTENT: i32 = 16384;
pub const FILE_OPEN_FOR_FREE_SPACE_QUERY: i32 = 8388608;
pub const FILE_OPEN_IF: i32 = 3;
pub const FILE_OPEN_NO_RECALL: i32 = 4194304;
pub const FILE_OPEN_REMOTE_INSTANCE: i32 = 1024;
pub const FILE_OPEN_REPARSE_POINT: i32 = 2097152;
pub const FILE_OPEN_REQUIRING_OPLOCK: i32 = 65536;
pub const FILE_OVERWRITE: i32 = 4;
pub const FILE_OVERWRITE_IF: i32 = 5;
pub const FILE_OVERWRITTEN: i32 = 3;
pub const FILE_RANDOM_ACCESS: i32 = 2048;
pub const FILE_RESERVE_OPFILTER: i32 = 1048576;
pub const FILE_SEQUENTIAL_ONLY: i32 = 4;
pub const FILE_SUPERSEDE: i32 = 0;
pub const FILE_SUPERSEDED: i32 = 0;
pub const FILE_SYNCHRONOUS_IO_ALERT: i32 = 16;
pub const FILE_SYNCHRONOUS_IO_NONALERT: i32 = 32;
pub const FILE_VALID_MAILSLOT_OPTION_FLAGS: i32 = 50;
pub const FILE_VALID_OPTION_FLAGS: i32 = 16777215;
pub const FILE_VALID_PIPE_OPTION_FLAGS: i32 = 50;
pub const FILE_VALID_SET_FLAGS: i32 = 54;
pub const FILE_WRITE_THROUGH: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct IO_STATUS_BLOCK {
    pub Anonymous: IO_STATUS_BLOCK_0,
    pub Information: super::ULONG_PTR,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IO_STATUS_BLOCK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union IO_STATUS_BLOCK_0 {
    pub Status: windows_core::NTSTATUS,
    pub Pointer: super::PVOID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for IO_STATUS_BLOCK_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type KEY_SET_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct KEY_VALUE_ENTRY {
    pub ValueName: PUNICODE_STRING,
    pub DataLength: super::ULONG,
    pub DataOffset: super::ULONG,
    pub Type: super::ULONG,
}
pub type KPRIORITY = i32;
pub const KeyControlFlagsInformation: KEY_SET_INFORMATION_CLASS = 2;
pub const KeySetDebugInformation: KEY_SET_INFORMATION_CLASS = 4;
pub const KeySetHandleTagsInformation: KEY_SET_INFORMATION_CLASS = 5;
pub const KeySetVirtualizationInformation: KEY_SET_INFORMATION_CLASS = 3;
pub const KeyWow64FlagsInformation: KEY_SET_INFORMATION_CLASS = 1;
pub const KeyWriteTimeInformation: KEY_SET_INFORMATION_CLASS = 0;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct LDR_DATA_TABLE_ENTRY {
    pub Reserved1: [super::PVOID; 2],
    pub InMemoryOrderLinks: super::LIST_ENTRY,
    pub Reserved2: [super::PVOID; 2],
    pub DllBase: super::PVOID,
    pub Reserved3: [super::PVOID; 2],
    pub FullDllName: UNICODE_STRING,
    pub Reserved4: [super::BYTE; 8],
    pub Reserved5: [super::PVOID; 3],
    pub Anonymous: LDR_DATA_TABLE_ENTRY_0,
    pub TimeDateStamp: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for LDR_DATA_TABLE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union LDR_DATA_TABLE_ENTRY_0 {
    pub CheckSum: super::ULONG,
    pub Reserved6: super::PVOID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for LDR_DATA_TABLE_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub const LOGONID_CURRENT: super::ULONG = 4294967295;
pub const MaxKeySetInfoClass: KEY_SET_INFORMATION_CLASS = 6;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OBJECT_ATTRIBUTES {
    pub Length: super::ULONG,
    pub RootDirectory: super::HANDLE,
    pub ObjectName: PUNICODE_STRING,
    pub Attributes: super::ULONG,
    pub SecurityDescriptor: super::PVOID,
    pub SecurityQualityOfService: super::PVOID,
}
pub type OBJECT_INFORMATION_CLASS = i32;
pub const OBJ_CASE_INSENSITIVE: i32 = 64;
pub const OBJ_DONT_REPARSE: i32 = 4096;
pub const OBJ_EXCLUSIVE: i32 = 32;
pub const OBJ_FORCE_ACCESS_CHECK: i32 = 1024;
pub const OBJ_IGNORE_IMPERSONATED_DEVICEMAP: i32 = 2048;
pub const OBJ_INHERIT: i32 = 2;
pub const OBJ_KERNEL_HANDLE: i32 = 512;
pub const OBJ_OPENIF: i32 = 128;
pub const OBJ_OPENLINK: i32 = 256;
pub const OBJ_PERMANENT: i32 = 16;
pub const OBJ_VALID_ATTRIBUTES: i32 = 8178;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type OEM_STRING = STRING;
pub const ObjectBasicInformation: OBJECT_INFORMATION_CLASS = 0;
pub const ObjectTypeInformation: OBJECT_INFORMATION_CLASS = 2;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PANSI_STRING = *mut STRING;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCANSI_STRING = *mut STRING;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCOEM_STRING = *const STRING;
pub type PCSZ = *const i8;
#[cfg(feature = "shared")]
pub type PCUNICODE_STRING = *const UNICODE_STRING;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug)]
pub struct PEB {
    pub Reserved1: [super::BYTE; 2],
    pub BeingDebugged: super::BYTE,
    pub Reserved2: [super::BYTE; 1],
    pub Reserved3: [super::PVOID; 2],
    pub Ldr: PPEB_LDR_DATA,
    pub ProcessParameters: PRTL_USER_PROCESS_PARAMETERS,
    pub Reserved4: [super::PVOID; 3],
    pub AtlThunkSListPtr: super::PVOID,
    pub Reserved5: super::PVOID,
    pub Reserved6: super::ULONG,
    pub Reserved7: super::PVOID,
    pub Reserved8: super::ULONG,
    pub AtlThunkSListPtr32: super::ULONG,
    pub Reserved9: [super::PVOID; 45],
    pub Reserved10: [super::BYTE; 96],
    pub PostProcessInitRoutine: PPS_POST_PROCESS_INIT_ROUTINE,
    pub Reserved11: [super::BYTE; 128],
    pub Reserved12: [super::PVOID; 1],
    pub SessionId: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PEB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PEB_LDR_DATA {
    pub Reserved1: [super::BYTE; 8],
    pub Reserved2: [super::PVOID; 3],
    pub InMemoryOrderModuleList: super::LIST_ENTRY,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PEB_LDR_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIO_APC_ROUTINE = Option<unsafe extern "system" fn(apccontext: super::PVOID, iostatusblock: PIO_STATUS_BLOCK, reserved: super::ULONG)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PIO_STATUS_BLOCK = *mut IO_STATUS_BLOCK;
#[cfg(feature = "shared")]
pub type PKEY_VALUE_ENTRY = *mut KEY_VALUE_ENTRY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PLDR_DATA_TABLE_ENTRY = *mut LDR_DATA_TABLE_ENTRY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type POBJECT_ATTRIBUTES = *mut OBJECT_ATTRIBUTES;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type POEM_STRING = *mut STRING;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPEB = *mut PEB;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPEB_LDR_DATA = *mut PEB_LDR_DATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPROCESS_BASIC_INFORMATION = *mut PROCESS_BASIC_INFORMATION;
pub type PPS_POST_PROCESS_INIT_ROUTINE = Option<unsafe extern "system" fn()>;
#[cfg(feature = "shared")]
pub type PPUBLIC_OBJECT_BASIC_INFORMATION = *mut PUBLIC_OBJECT_BASIC_INFORMATION;
#[cfg(feature = "shared")]
pub type PPUBLIC_OBJECT_TYPE_INFORMATION = *mut PUBLIC_OBJECT_TYPE_INFORMATION;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROCESS_BASIC_INFORMATION {
    pub Reserved1: super::PVOID,
    pub PebBaseAddress: PPEB,
    pub Reserved2: [super::PVOID; 2],
    pub UniqueProcessId: super::ULONG_PTR,
    pub Reserved3: super::PVOID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for PROCESS_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PRTL_USER_PROCESS_PARAMETERS = *mut RTL_USER_PROCESS_PARAMETERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSTRING = *mut STRING;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYSTEM_BASICPROCESS_INFORMATION = *mut SYSTEM_BASICPROCESS_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYSTEM_BASIC_INFORMATION = *mut SYSTEM_BASIC_INFORMATION;
#[cfg(feature = "shared")]
pub type PSYSTEM_CODEINTEGRITY_INFORMATION = *mut SYSTEM_CODEINTEGRITY_INFORMATION;
#[cfg(feature = "shared")]
pub type PSYSTEM_EXCEPTION_INFORMATION = *mut SYSTEM_EXCEPTION_INFORMATION;
#[cfg(feature = "shared")]
pub type PSYSTEM_HANDLECOUNT_INFORMATION = *mut SYSTEM_HANDLECOUNT_INFORMATION;
#[cfg(feature = "shared")]
pub type PSYSTEM_INTERRUPT_INFORMATION = *mut SYSTEM_INTERRUPT_INFORMATION;
#[cfg(feature = "shared")]
pub type PSYSTEM_LOOKASIDE_INFORMATION = *mut SYSTEM_LOOKASIDE_INFORMATION;
#[cfg(feature = "shared")]
pub type PSYSTEM_PERFORMANCE_INFORMATION = *mut SYSTEM_PERFORMANCE_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYSTEM_POLICY_INFORMATION = *mut SYSTEM_POLICY_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYSTEM_PROCESSOR_PERFORMANCE_INFORMATION = *mut SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYSTEM_PROCESS_INFORMATION = *mut SYSTEM_PROCESS_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYSTEM_REGISTRY_QUOTA_INFORMATION = *mut SYSTEM_REGISTRY_QUOTA_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PSYSTEM_THREAD_INFORMATION = *mut SYSTEM_THREAD_INFORMATION;
#[cfg(feature = "shared")]
pub type PSYSTEM_TIMEOFDAY_INFORMATION = *mut SYSTEM_TIMEOFDAY_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PTEB = *mut TEB;
#[cfg(feature = "shared")]
pub type PTHREAD_NAME_INFORMATION = *mut THREAD_NAME_INFORMATION;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PUBLIC_OBJECT_BASIC_INFORMATION {
    pub Attributes: super::ULONG,
    pub GrantedAccess: super::ACCESS_MASK,
    pub HandleCount: super::ULONG,
    pub PointerCount: super::ULONG,
    pub Reserved: [super::ULONG; 10],
}
#[cfg(feature = "shared")]
impl Default for PUBLIC_OBJECT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PUBLIC_OBJECT_TYPE_INFORMATION {
    pub TypeName: UNICODE_STRING,
    pub Reserved: [super::ULONG; 22],
}
#[cfg(feature = "shared")]
impl Default for PUBLIC_OBJECT_TYPE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type PUNICODE_STRING = *mut UNICODE_STRING;
#[cfg(feature = "shared")]
pub type PWINSTATIONINFORMATIONW = *mut WINSTATIONINFORMATIONW;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PWINSTATIONQUERYINFORMATIONW = Option<unsafe extern "system" fn(param0: super::HANDLE, param1: super::ULONG, param2: WINSTATIONINFOCLASS, param3: super::PVOID, param4: super::ULONG, param5: super::PULONG) -> super::BOOLEAN>;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RTL_USER_PROCESS_PARAMETERS {
    pub Reserved1: [super::BYTE; 16],
    pub Reserved2: [super::PVOID; 10],
    pub ImagePathName: UNICODE_STRING,
    pub CommandLine: UNICODE_STRING,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for RTL_USER_PROCESS_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub const SERVERNAME_CURRENT: super::HANDLE = 0 as _;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STRING {
    pub Length: super::USHORT,
    pub MaximumLength: super::USHORT,
    pub Buffer: super::PCHAR,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEM_BASICPROCESS_INFORMATION {
    pub NextEntryOffset: super::ULONG,
    pub UniqueProcessId: super::HANDLE,
    pub InheritedFromUniqueProcessId: super::HANDLE,
    pub SequenceNumber: super::ULONG64,
    pub ImageName: UNICODE_STRING,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_BASIC_INFORMATION {
    pub Reserved1: [super::BYTE; 24],
    pub Reserved2: [super::PVOID; 4],
    pub NumberOfProcessors: super::CCHAR,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SYSTEM_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEM_CODEINTEGRITY_INFORMATION {
    pub Length: super::ULONG,
    pub CodeIntegrityOptions: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_EXCEPTION_INFORMATION {
    pub Reserved1: [super::BYTE; 16],
}
#[cfg(feature = "shared")]
impl Default for SYSTEM_EXCEPTION_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEM_HANDLECOUNT_INFORMATION {
    pub ProcessCount: super::ULONG,
    pub ThreadCount: super::ULONG,
    pub HandleCount: super::ULONG,
}
pub type SYSTEM_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_INTERRUPT_INFORMATION {
    pub Reserved1: [super::BYTE; 24],
}
#[cfg(feature = "shared")]
impl Default for SYSTEM_INTERRUPT_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_LOOKASIDE_INFORMATION {
    pub Reserved1: [super::BYTE; 32],
}
#[cfg(feature = "shared")]
impl Default for SYSTEM_LOOKASIDE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_PERFORMANCE_INFORMATION {
    pub Reserved1: [super::BYTE; 312],
}
#[cfg(feature = "shared")]
impl Default for SYSTEM_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_POLICY_INFORMATION {
    pub Reserved1: [super::PVOID; 2],
    pub Reserved2: [super::ULONG; 3],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SYSTEM_POLICY_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    pub IdleTime: super::LARGE_INTEGER,
    pub KernelTime: super::LARGE_INTEGER,
    pub UserTime: super::LARGE_INTEGER,
    pub Reserved1: [super::LARGE_INTEGER; 2],
    pub Reserved2: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SYSTEM_PROCESSOR_PERFORMANCE_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SYSTEM_PROCESS_INFORMATION {
    pub NextEntryOffset: super::ULONG,
    pub NumberOfThreads: super::ULONG,
    pub Reserved1: [super::BYTE; 48],
    pub ImageName: UNICODE_STRING,
    pub BasePriority: KPRIORITY,
    pub UniqueProcessId: super::HANDLE,
    pub InheritedFromUniqueProcessId: super::HANDLE,
    pub HandleCount: super::ULONG,
    pub SessionId: super::ULONG,
    pub Reserved3: super::PVOID,
    pub PeakVirtualSize: super::SIZE_T,
    pub VirtualSize: super::SIZE_T,
    pub Reserved4: super::ULONG,
    pub PeakWorkingSetSize: super::SIZE_T,
    pub WorkingSetSize: super::SIZE_T,
    pub Reserved5: super::PVOID,
    pub QuotaPagedPoolUsage: super::SIZE_T,
    pub Reserved6: super::PVOID,
    pub QuotaNonPagedPoolUsage: super::SIZE_T,
    pub PagefileUsage: super::SIZE_T,
    pub PeakPagefileUsage: super::SIZE_T,
    pub PrivatePageCount: super::SIZE_T,
    pub Reserved7: [super::LARGE_INTEGER; 6],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SYSTEM_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEM_REGISTRY_QUOTA_INFORMATION {
    pub RegistryQuotaAllowed: super::ULONG,
    pub RegistryQuotaUsed: super::ULONG,
    pub Reserved1: super::PVOID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct SYSTEM_THREAD_INFORMATION {
    pub Reserved1: [super::LARGE_INTEGER; 3],
    pub Reserved2: super::ULONG,
    pub StartAddress: super::PVOID,
    pub ClientId: CLIENT_ID,
    pub Priority: KPRIORITY,
    pub BasePriority: super::LONG,
    pub Reserved3: super::ULONG,
    pub ThreadState: super::ULONG,
    pub WaitReason: super::ULONG,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SYSTEM_TIMEOFDAY_INFORMATION {
    pub Reserved1: [super::BYTE; 48],
}
#[cfg(feature = "shared")]
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
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TEB {
    pub Reserved1: [super::PVOID; 12],
    pub ProcessEnvironmentBlock: PPEB,
    pub Reserved2: [super::PVOID; 399],
    pub Reserved3: [super::BYTE; 1952],
    pub TlsSlots: [super::PVOID; 64],
    pub Reserved4: [super::BYTE; 8],
    pub Reserved5: [super::PVOID; 26],
    pub ReservedForOle: super::PVOID,
    pub Reserved6: [super::PVOID; 4],
    pub TlsExpansionSlots: super::PVOID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for TEB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct THREAD_NAME_INFORMATION {
    pub ThreadName: UNICODE_STRING,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UNICODE_STRING {
    pub Length: super::USHORT,
    pub MaximumLength: super::USHORT,
    pub Buffer: windows_core::PWSTR,
}
pub type WINSTATIONINFOCLASS = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WINSTATIONINFORMATIONW {
    pub Reserved2: [super::BYTE; 70],
    pub LogonId: super::ULONG,
    pub Reserved3: [super::BYTE; 1140],
}
#[cfg(feature = "shared")]
impl Default for WINSTATIONINFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WinStationInformation: WINSTATIONINFOCLASS = 8;
