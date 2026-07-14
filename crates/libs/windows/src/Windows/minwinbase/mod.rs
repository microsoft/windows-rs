pub const CONTROL_C_EXIT: i32 = -1073741510;
pub const CREATE_PROCESS_DEBUG_EVENT: u32 = 3;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug)]
pub struct CREATE_PROCESS_DEBUG_INFO {
    pub hFile: super::winnt::HANDLE,
    pub hProcess: super::winnt::HANDLE,
    pub hThread: super::winnt::HANDLE,
    pub lpBaseOfImage: *mut core::ffi::c_void,
    pub dwDebugInfoFileOffset: u32,
    pub nDebugInfoSize: u32,
    pub lpThreadLocalBase: *mut core::ffi::c_void,
    pub lpStartAddress: LPTHREAD_START_ROUTINE,
    pub lpImageName: *mut core::ffi::c_void,
    pub fUnicode: u16,
}
#[cfg(feature = "winnt")]
impl Default for CREATE_PROCESS_DEBUG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const CREATE_THREAD_DEBUG_EVENT: u32 = 2;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug)]
pub struct CREATE_THREAD_DEBUG_INFO {
    pub hThread: super::winnt::HANDLE,
    pub lpThreadLocalBase: *mut core::ffi::c_void,
    pub lpStartAddress: LPTHREAD_START_ROUTINE,
}
#[cfg(feature = "winnt")]
impl Default for CREATE_THREAD_DEBUG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "winnt")]
pub type CRITICAL_SECTION = super::winnt::RTL_CRITICAL_SECTION;
#[cfg(feature = "winnt")]
pub type CRITICAL_SECTION_DEBUG = super::winnt::RTL_CRITICAL_SECTION_DEBUG;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct DEBUG_EVENT {
    pub dwDebugEventCode: u32,
    pub dwProcessId: u32,
    pub dwThreadId: u32,
    pub u: DEBUG_EVENT_0,
}
#[cfg(feature = "winnt")]
impl Default for DEBUG_EVENT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union DEBUG_EVENT_0 {
    pub Exception: EXCEPTION_DEBUG_INFO,
    pub CreateThread: CREATE_THREAD_DEBUG_INFO,
    pub CreateProcessInfo: CREATE_PROCESS_DEBUG_INFO,
    pub ExitThread: EXIT_THREAD_DEBUG_INFO,
    pub ExitProcess: EXIT_PROCESS_DEBUG_INFO,
    pub LoadDll: LOAD_DLL_DEBUG_INFO,
    pub UnloadDll: UNLOAD_DLL_DEBUG_INFO,
    pub DebugString: OUTPUT_DEBUG_STRING_INFO,
    pub RipInfo: RIP_INFO,
}
#[cfg(feature = "winnt")]
impl Default for DEBUG_EVENT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXCEPTION_ACCESS_VIOLATION: i32 = -1073741819;
pub const EXCEPTION_ARRAY_BOUNDS_EXCEEDED: i32 = -1073741684;
pub const EXCEPTION_BREAKPOINT: i32 = -2147483645;
pub const EXCEPTION_DATATYPE_MISALIGNMENT: i32 = -2147483646;
pub const EXCEPTION_DEBUG_EVENT: u32 = 1;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXCEPTION_DEBUG_INFO {
    pub ExceptionRecord: super::winnt::EXCEPTION_RECORD,
    pub dwFirstChance: u32,
}
pub const EXCEPTION_FLT_DENORMAL_OPERAND: i32 = -1073741683;
pub const EXCEPTION_FLT_DIVIDE_BY_ZERO: i32 = -1073741682;
pub const EXCEPTION_FLT_INEXACT_RESULT: i32 = -1073741681;
pub const EXCEPTION_FLT_INVALID_OPERATION: i32 = -1073741680;
pub const EXCEPTION_FLT_OVERFLOW: i32 = -1073741679;
pub const EXCEPTION_FLT_STACK_CHECK: i32 = -1073741678;
pub const EXCEPTION_FLT_UNDERFLOW: i32 = -1073741677;
pub const EXCEPTION_GUARD_PAGE: i32 = -2147483647;
pub const EXCEPTION_ILLEGAL_INSTRUCTION: i32 = -1073741795;
pub const EXCEPTION_INT_DIVIDE_BY_ZERO: i32 = -1073741676;
pub const EXCEPTION_INT_OVERFLOW: i32 = -1073741675;
pub const EXCEPTION_INVALID_DISPOSITION: i32 = -1073741786;
pub const EXCEPTION_INVALID_HANDLE: i32 = -1073741816;
pub const EXCEPTION_IN_PAGE_ERROR: i32 = -1073741818;
pub const EXCEPTION_NONCONTINUABLE_EXCEPTION: i32 = -1073741787;
pub const EXCEPTION_POSSIBLE_DEADLOCK: i32 = -1073741420;
pub const EXCEPTION_PRIV_INSTRUCTION: i32 = -1073741674;
pub const EXCEPTION_SINGLE_STEP: i32 = -2147483644;
pub const EXCEPTION_STACK_OVERFLOW: i32 = -1073741571;
pub const EXIT_PROCESS_DEBUG_EVENT: u32 = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXIT_PROCESS_DEBUG_INFO {
    pub dwExitCode: u32,
}
pub const EXIT_THREAD_DEBUG_EVENT: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EXIT_THREAD_DEBUG_INFO {
    pub dwExitCode: u32,
}
pub type FILE_INFO_BY_HANDLE_CLASS = i32;
pub type FILE_INFO_BY_NAME_CLASS = i32;
pub type FINDEX_INFO_LEVELS = i32;
pub type FINDEX_SEARCH_OPS = i32;
pub const FIND_FIRST_EX_CASE_SENSITIVE: u32 = 1;
pub const FIND_FIRST_EX_LARGE_FETCH: u32 = 2;
pub const FIND_FIRST_EX_ON_DISK_ENTRIES_ONLY: u32 = 4;
pub const FileAlignmentInfo: FILE_INFO_BY_HANDLE_CLASS = 17;
pub const FileAllocationInfo: FILE_INFO_BY_HANDLE_CLASS = 5;
pub const FileAttributeTagInfo: FILE_INFO_BY_HANDLE_CLASS = 9;
pub const FileBasicInfo: FILE_INFO_BY_HANDLE_CLASS = 0;
pub const FileCaseSensitiveByNameInfo: FILE_INFO_BY_NAME_CLASS = 2;
pub const FileCaseSensitiveInfo: FILE_INFO_BY_HANDLE_CLASS = 23;
pub const FileCompressionInfo: FILE_INFO_BY_HANDLE_CLASS = 8;
pub const FileDispositionInfo: FILE_INFO_BY_HANDLE_CLASS = 4;
pub const FileDispositionInfoEx: FILE_INFO_BY_HANDLE_CLASS = 21;
pub const FileEndOfFileInfo: FILE_INFO_BY_HANDLE_CLASS = 6;
pub const FileFullDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = 14;
pub const FileFullDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = 15;
pub const FileIdBothDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = 10;
pub const FileIdBothDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = 11;
pub const FileIdExtdDirectoryInfo: FILE_INFO_BY_HANDLE_CLASS = 19;
pub const FileIdExtdDirectoryRestartInfo: FILE_INFO_BY_HANDLE_CLASS = 20;
pub const FileIdInfo: FILE_INFO_BY_HANDLE_CLASS = 18;
pub const FileIoPriorityHintInfo: FILE_INFO_BY_HANDLE_CLASS = 12;
pub const FileNameInfo: FILE_INFO_BY_HANDLE_CLASS = 2;
pub const FileNormalizedNameInfo: FILE_INFO_BY_HANDLE_CLASS = 24;
pub const FileRemoteProtocolInfo: FILE_INFO_BY_HANDLE_CLASS = 13;
pub const FileRenameInfo: FILE_INFO_BY_HANDLE_CLASS = 3;
pub const FileRenameInfoEx: FILE_INFO_BY_HANDLE_CLASS = 22;
pub const FileStandardInfo: FILE_INFO_BY_HANDLE_CLASS = 1;
pub const FileStatBasicByNameInfo: FILE_INFO_BY_NAME_CLASS = 3;
pub const FileStatByNameInfo: FILE_INFO_BY_NAME_CLASS = 0;
pub const FileStatLxByNameInfo: FILE_INFO_BY_NAME_CLASS = 1;
pub const FileStorageInfo: FILE_INFO_BY_HANDLE_CLASS = 16;
pub const FileStreamInfo: FILE_INFO_BY_HANDLE_CLASS = 7;
pub const FindExInfoBasic: FINDEX_INFO_LEVELS = 1;
pub const FindExInfoMaxInfoLevel: FINDEX_INFO_LEVELS = 2;
pub const FindExInfoStandard: FINDEX_INFO_LEVELS = 0;
pub const FindExSearchLimitToDevices: FINDEX_SEARCH_OPS = 2;
pub const FindExSearchLimitToDirectories: FINDEX_SEARCH_OPS = 1;
pub const FindExSearchMaxSearchOp: FINDEX_SEARCH_OPS = 3;
pub const FindExSearchNameMatch: FINDEX_SEARCH_OPS = 0;
pub type GET_FILEEX_INFO_LEVELS = i32;
pub const GetFileExInfoStandard: GET_FILEEX_INFO_LEVELS = 0;
pub const GetFileExMaxInfoLevel: GET_FILEEX_INFO_LEVELS = 1;
pub const LHND: u32 = 66;
pub const LMEM_DISCARDABLE: u32 = 3840;
pub const LMEM_DISCARDED: u32 = 16384;
pub const LMEM_FIXED: u32 = 0;
pub const LMEM_INVALID_HANDLE: u32 = 32768;
pub const LMEM_LOCKCOUNT: u32 = 255;
pub const LMEM_MODIFY: u32 = 128;
pub const LMEM_MOVEABLE: u32 = 2;
pub const LMEM_NOCOMPACT: u32 = 16;
pub const LMEM_NODISCARD: u32 = 32;
pub const LMEM_VALID_FLAGS: u32 = 3954;
pub const LMEM_ZEROINIT: u32 = 64;
pub const LOAD_DLL_DEBUG_EVENT: u32 = 6;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LOAD_DLL_DEBUG_INFO {
    pub hFile: super::winnt::HANDLE,
    pub lpBaseOfDll: *mut core::ffi::c_void,
    pub dwDebugInfoFileOffset: u32,
    pub nDebugInfoSize: u32,
    pub lpImageName: *mut core::ffi::c_void,
    pub fUnicode: u16,
}
#[cfg(feature = "winnt")]
impl Default for LOAD_DLL_DEBUG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const LOCKFILE_EXCLUSIVE_LOCK: u32 = 2;
pub const LOCKFILE_FAIL_IMMEDIATELY: u32 = 1;
#[cfg(feature = "winnt")]
pub type LPCONTEXT = super::winnt::PCONTEXT;
#[cfg(feature = "winnt")]
pub type LPCREATE_PROCESS_DEBUG_INFO = *mut CREATE_PROCESS_DEBUG_INFO;
#[cfg(feature = "winnt")]
pub type LPCREATE_THREAD_DEBUG_INFO = *mut CREATE_THREAD_DEBUG_INFO;
#[cfg(feature = "winnt")]
pub type LPCRITICAL_SECTION = super::winnt::PRTL_CRITICAL_SECTION;
#[cfg(feature = "winnt")]
pub type LPCRITICAL_SECTION_DEBUG = super::winnt::PRTL_CRITICAL_SECTION_DEBUG;
#[cfg(feature = "winnt")]
pub type LPDEBUG_EVENT = *mut DEBUG_EVENT;
pub type LPENCLAVE_ROUTINE = PENCLAVE_ROUTINE;
#[cfg(feature = "winnt")]
pub type LPEXCEPTION_DEBUG_INFO = *mut EXCEPTION_DEBUG_INFO;
pub type LPEXIT_PROCESS_DEBUG_INFO = *mut EXIT_PROCESS_DEBUG_INFO;
pub type LPEXIT_THREAD_DEBUG_INFO = *mut EXIT_THREAD_DEBUG_INFO;
#[cfg(feature = "winnt")]
pub type LPLOAD_DLL_DEBUG_INFO = *mut LOAD_DLL_DEBUG_INFO;
pub type LPOUTPUT_DEBUG_STRING_INFO = *mut OUTPUT_DEBUG_STRING_INFO;
#[cfg(feature = "winnt")]
pub type LPOVERLAPPED = *mut OVERLAPPED;
#[cfg(feature = "winnt")]
pub type LPOVERLAPPED_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(dwerrorcode: u32, dwnumberofbytestransfered: u32, lpoverlapped: *mut OVERLAPPED)>;
#[cfg(feature = "winnt")]
pub type LPOVERLAPPED_ENTRY = *mut OVERLAPPED_ENTRY;
#[cfg(feature = "winnt")]
pub type LPPROCESS_HEAP_ENTRY = *mut PROCESS_HEAP_ENTRY;
pub type LPRIP_INFO = *mut RIP_INFO;
pub type LPSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
pub type LPSYSTEMTIME = *mut SYSTEMTIME;
pub type LPTHREAD_START_ROUTINE = PTHREAD_START_ROUTINE;
pub const LPTR: u32 = 64;
pub type LPUNLOAD_DLL_DEBUG_INFO = *mut UNLOAD_DLL_DEBUG_INFO;
#[cfg(feature = "minwindef")]
pub type LPWIN32_FIND_DATA = LPWIN32_FIND_DATAA;
#[cfg(feature = "minwindef")]
pub type LPWIN32_FIND_DATAA = *mut WIN32_FIND_DATAA;
#[cfg(feature = "minwindef")]
pub type LPWIN32_FIND_DATAW = *mut WIN32_FIND_DATAW;
pub const MaximumFileInfoByHandleClass: FILE_INFO_BY_HANDLE_CLASS = 25;
pub const MaximumFileInfoByNameClass: FILE_INFO_BY_NAME_CLASS = 4;
pub const NONZEROLHND: u32 = 2;
pub const NONZEROLPTR: u32 = 0;
pub const NUMA_NO_PREFERRED_NODE: u32 = 4294967295;
pub const OUTPUT_DEBUG_STRING_EVENT: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OUTPUT_DEBUG_STRING_INFO {
    pub lpDebugStringData: windows_core::PSTR,
    pub fUnicode: u16,
    pub nDebugStringLength: u16,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct OVERLAPPED {
    pub Internal: usize,
    pub InternalHigh: usize,
    pub Anonymous: OVERLAPPED_0,
    pub hEvent: super::winnt::HANDLE,
}
#[cfg(feature = "winnt")]
impl Default for OVERLAPPED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union OVERLAPPED_0 {
    pub Anonymous: OVERLAPPED_0_0,
    pub Pointer: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for OVERLAPPED_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OVERLAPPED_0_0 {
    pub Offset: u32,
    pub OffsetHigh: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OVERLAPPED_ENTRY {
    pub lpCompletionKey: usize,
    pub lpOverlapped: LPOVERLAPPED,
    pub Internal: usize,
    pub dwNumberOfBytesTransferred: u32,
}
#[cfg(feature = "winnt")]
pub type PCRITICAL_SECTION = super::winnt::PRTL_CRITICAL_SECTION;
#[cfg(feature = "winnt")]
pub type PCRITICAL_SECTION_DEBUG = super::winnt::PRTL_CRITICAL_SECTION_DEBUG;
pub type PENCLAVE_ROUTINE = Option<unsafe extern "system" fn(lpthreadparameter: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
pub type PFILE_INFO_BY_HANDLE_CLASS = *mut FILE_INFO_BY_HANDLE_CLASS;
pub type PFILE_INFO_BY_NAME_CLASS = *mut FILE_INFO_BY_NAME_CLASS;
#[cfg(feature = "winnt")]
pub type PPROCESS_HEAP_ENTRY = *mut PROCESS_HEAP_ENTRY;
pub type PREAD_DIRECTORY_NOTIFY_INFORMATION_CLASS = *mut READ_DIRECTORY_NOTIFY_INFORMATION_CLASS;
#[cfg(feature = "minwindef")]
pub type PREASON_CONTEXT = *mut REASON_CONTEXT;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct PROCESS_HEAP_ENTRY {
    pub lpData: *mut core::ffi::c_void,
    pub cbData: u32,
    pub cbOverhead: u8,
    pub iRegionIndex: u8,
    pub wFlags: u16,
    pub Anonymous: PROCESS_HEAP_ENTRY_0,
}
#[cfg(feature = "winnt")]
impl Default for PROCESS_HEAP_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union PROCESS_HEAP_ENTRY_0 {
    pub Block: PROCESS_HEAP_ENTRY_0_0,
    pub Region: PROCESS_HEAP_ENTRY_0_1,
}
#[cfg(feature = "winnt")]
impl Default for PROCESS_HEAP_ENTRY_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROCESS_HEAP_ENTRY_0_0 {
    pub hMem: super::winnt::HANDLE,
    pub dwReserved: [u32; 3],
}
#[cfg(feature = "winnt")]
impl Default for PROCESS_HEAP_ENTRY_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PROCESS_HEAP_ENTRY_0_1 {
    pub dwCommittedSize: u32,
    pub dwUnCommittedSize: u32,
    pub lpFirstBlock: *mut core::ffi::c_void,
    pub lpLastBlock: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for PROCESS_HEAP_ENTRY_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const PROCESS_HEAP_ENTRY_BUSY: u32 = 4;
pub const PROCESS_HEAP_ENTRY_DDESHARE: u32 = 32;
pub const PROCESS_HEAP_ENTRY_MOVEABLE: u32 = 16;
pub const PROCESS_HEAP_REGION: u32 = 1;
pub const PROCESS_HEAP_SEG_ALLOC: u32 = 8;
pub const PROCESS_HEAP_UNCOMMITTED_RANGE: u32 = 2;
pub type PSECURITY_ATTRIBUTES = *mut SECURITY_ATTRIBUTES;
pub type PSYSTEMTIME = *mut SYSTEMTIME;
pub type PTHREAD_START_ROUTINE = Option<unsafe extern "system" fn(lpthreadparameter: *mut core::ffi::c_void) -> u32>;
#[cfg(feature = "minwindef")]
pub type PWIN32_FIND_DATA = PWIN32_FIND_DATAA;
#[cfg(feature = "minwindef")]
pub type PWIN32_FIND_DATAA = *mut WIN32_FIND_DATAA;
#[cfg(feature = "minwindef")]
pub type PWIN32_FIND_DATAW = *mut WIN32_FIND_DATAW;
pub type READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub struct REASON_CONTEXT {
    pub Version: u32,
    pub Flags: u32,
    pub Reason: REASON_CONTEXT_0,
}
#[cfg(feature = "minwindef")]
impl Default for REASON_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy)]
pub union REASON_CONTEXT_0 {
    pub Detailed: REASON_CONTEXT_0_0,
    pub SimpleReasonString: windows_core::PWSTR,
}
#[cfg(feature = "minwindef")]
impl Default for REASON_CONTEXT_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct REASON_CONTEXT_0_0 {
    pub LocalizedReasonModule: super::minwindef::HMODULE,
    pub LocalizedReasonId: u32,
    pub ReasonStringCount: u32,
    pub ReasonStrings: *mut windows_core::PWSTR,
}
#[cfg(feature = "minwindef")]
impl Default for REASON_CONTEXT_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const RIP_EVENT: u32 = 9;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RIP_INFO {
    pub dwError: u32,
    pub dwType: u32,
}
pub const ReadDirectoryNotifyExtendedInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = 2;
pub const ReadDirectoryNotifyFullInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = 3;
pub const ReadDirectoryNotifyInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = 1;
pub const ReadDirectoryNotifyMaximumInformation: READ_DIRECTORY_NOTIFY_INFORMATION_CLASS = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SECURITY_ATTRIBUTES {
    pub nLength: u32,
    pub lpSecurityDescriptor: *mut core::ffi::c_void,
    pub bInheritHandle: windows_core::BOOL,
}
impl Default for SECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STILL_ACTIVE: u32 = 259;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEMTIME {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDayOfWeek: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
pub const UNLOAD_DLL_DEBUG_EVENT: u32 = 7;
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UNLOAD_DLL_DEBUG_INFO {
    pub lpBaseOfDll: *mut core::ffi::c_void,
}
impl Default for UNLOAD_DLL_DEBUG_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
pub type WIN32_FIND_DATA = WIN32_FIND_DATAA;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WIN32_FIND_DATAA {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::minwindef::FILETIME,
    pub ftLastAccessTime: super::minwindef::FILETIME,
    pub ftLastWriteTime: super::minwindef::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub dwReserved0: u32,
    pub dwReserved1: u32,
    pub cFileName: [i8; 260],
    pub cAlternateFileName: [i8; 14],
}
#[cfg(feature = "minwindef")]
impl Default for WIN32_FIND_DATAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WIN32_FIND_DATAW {
    pub dwFileAttributes: u32,
    pub ftCreationTime: super::minwindef::FILETIME,
    pub ftLastAccessTime: super::minwindef::FILETIME,
    pub ftLastWriteTime: super::minwindef::FILETIME,
    pub nFileSizeHigh: u32,
    pub nFileSizeLow: u32,
    pub dwReserved0: u32,
    pub dwReserved1: u32,
    pub cFileName: [u16; 260],
    pub cAlternateFileName: [u16; 14],
}
#[cfg(feature = "minwindef")]
impl Default for WIN32_FIND_DATAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
