windows_link::link!("clfsw32.dll" "C" fn LsnBlockOffset(plsn : *const CLFS_LSN) -> u32);
windows_link::link!("clfsw32.dll" "C" fn LsnContainer(plsn : *const CLFS_LSN) -> CLFS_CONTAINER_ID);
windows_link::link!("clfsw32.dll" "C" fn LsnCreate(cidcontainer : CLFS_CONTAINER_ID, offblock : u32, crecord : u32) -> CLFS_LSN);
windows_link::link!("clfsw32.dll" "C" fn LsnEqual(plsn1 : *const CLFS_LSN, plsn2 : *const CLFS_LSN) -> bool);
windows_link::link!("clfsw32.dll" "C" fn LsnGreater(plsn1 : *const CLFS_LSN, plsn2 : *const CLFS_LSN) -> bool);
windows_link::link!("clfsw32.dll" "C" fn LsnIncrement(plsn : *const CLFS_LSN) -> CLFS_LSN);
windows_link::link!("clfsw32.dll" "C" fn LsnInvalid(plsn : *const CLFS_LSN) -> bool);
windows_link::link!("clfsw32.dll" "C" fn LsnLess(plsn1 : *const CLFS_LSN, plsn2 : *const CLFS_LSN) -> bool);
windows_link::link!("clfsw32.dll" "C" fn LsnNull(plsn : *const CLFS_LSN) -> bool);
windows_link::link!("clfsw32.dll" "C" fn LsnRecordSequence(plsn : *const CLFS_LSN) -> u32);
pub type CLFSSTATUS = u32;
pub type CLFS_ARCHIVE_DESCRIPTOR = CLS_ARCHIVE_DESCRIPTOR;
pub const CLFS_BASELOG_EXTENSION: windows_sys::core::PCWSTR = windows_sys::core::w!(".blf");
pub type CLFS_BLOCK_ALLOCATION = Option<unsafe extern "system" fn(cbbufferlength: u32, pvusercontext: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
pub type CLFS_BLOCK_DEALLOCATION = Option<unsafe extern "system" fn(pvbuffer: *mut core::ffi::c_void, pvusercontext: *mut core::ffi::c_void)>;
pub type CLFS_CONTAINER_ID = u32;
pub type CLFS_CONTAINER_INFORMATION = CLS_CONTAINER_INFORMATION;
pub const CLFS_CONTAINER_RELATIVE_PREFIX: windows_sys::core::PCWSTR = windows_sys::core::w!("%BLF%\\");
pub type CLFS_CONTAINER_STATE = CLS_CONTAINER_STATE;
pub const CLFS_CONTAINER_STREAM_PREFIX: windows_sys::core::PCWSTR = windows_sys::core::w!("%BLF%:");
pub type CLFS_CONTEXT_MODE = i32;
pub const CLFS_FLAG_FILTER_INTERMEDIATE_LEVEL: u32 = 16;
pub const CLFS_FLAG_FILTER_TOP_LEVEL: u32 = 32;
pub const CLFS_FLAG_FORCE_APPEND: u32 = 1;
pub const CLFS_FLAG_FORCE_FLUSH: u32 = 2;
pub const CLFS_FLAG_HIDDEN_SYSTEM_LOG: u32 = 512;
pub const CLFS_FLAG_IGNORE_SHARE_ACCESS: u32 = 64;
pub const CLFS_FLAG_MINIFILTER_LEVEL: u32 = 256;
pub const CLFS_FLAG_NON_REENTRANT_FILTER: u32 = 16;
pub const CLFS_FLAG_NO_FLAGS: u32 = 0;
pub const CLFS_FLAG_READ_IN_PROGRESS: u32 = 128;
pub const CLFS_FLAG_REENTRANT_FILE_SYSTEM: u32 = 8;
pub const CLFS_FLAG_REENTRANT_FILTER: u32 = 32;
pub const CLFS_FLAG_USE_RESERVATION: u32 = 4;
pub type CLFS_INFORMATION = CLS_INFORMATION;
pub type CLFS_IOSTATS_CLASS = i32;
pub type CLFS_IO_STATISTICS = CLS_IO_STATISTICS;
pub type CLFS_IO_STATISTICS_HEADER = CLS_IO_STATISTICS_HEADER;
pub type CLFS_LOG_ARCHIVE_MODE = i32;
pub type CLFS_LOG_ID = windows_sys::core::GUID;
pub type CLFS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLFS_LOG_NAME_INFORMATION {
    pub NameLengthInBytes: u16,
    pub Name: [u16; 1],
}
impl Default for CLFS_LOG_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLFS_LSN = CLS_LSN;
pub const CLFS_MARSHALLING_FLAG_DISABLE_BUFF_INIT: u32 = 1;
pub const CLFS_MARSHALLING_FLAG_NONE: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CLFS_NODE_ID {
    pub cType: u32,
    pub cbNode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CLFS_PHYSICAL_LSN_INFORMATION {
    pub StreamIdentifier: u8,
    pub VirtualLsn: CLFS_LSN,
    pub PhysicalLsn: CLFS_LSN,
}
pub type CLFS_RECORD_TYPE = CLS_RECORD_TYPE;
#[cfg(feature = "Win32_winnt")]
pub type CLFS_SCAN_CONTEXT = CLS_SCAN_CONTEXT;
pub type CLFS_SCAN_MODE = u8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CLFS_STREAM_ID_INFORMATION {
    pub StreamIdentifier: u8,
}
pub type CLFS_WRITE_ENTRY = CLS_WRITE_ENTRY;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CLS_ARCHIVE_DESCRIPTOR {
    pub coffLow: u64,
    pub coffHigh: u64,
    pub infoContainer: CLS_CONTAINER_INFORMATION,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLS_CONTAINER_INFORMATION {
    pub FileAttributes: u32,
    pub CreationTime: u64,
    pub LastAccessTime: u64,
    pub LastWriteTime: u64,
    pub ContainerSize: i64,
    pub FileNameActualLength: u32,
    pub FileNameLength: u32,
    pub FileName: [u16; 256],
    pub State: CLFS_CONTAINER_STATE,
    pub PhysicalContainerId: CLFS_CONTAINER_ID,
    pub LogicalContainerId: CLFS_CONTAINER_ID,
}
impl Default for CLS_CONTAINER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLS_CONTAINER_STATE = u32;
pub type CLS_CONTEXT_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CLS_INFORMATION {
    pub TotalAvailable: i64,
    pub CurrentAvailable: i64,
    pub TotalReservation: i64,
    pub BaseFileSize: u64,
    pub ContainerSize: u64,
    pub TotalContainers: u32,
    pub FreeContainers: u32,
    pub TotalClients: u32,
    pub Attributes: u32,
    pub FlushThreshold: u32,
    pub SectorSize: u32,
    pub MinArchiveTailLsn: CLS_LSN,
    pub BaseLsn: CLS_LSN,
    pub LastFlushedLsn: CLS_LSN,
    pub LastLsn: CLS_LSN,
    pub RestartLsn: CLS_LSN,
    pub Identity: windows_sys::core::GUID,
}
pub type CLS_IOSTATS_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CLS_IO_STATISTICS {
    pub hdrIoStats: CLS_IO_STATISTICS_HEADER,
    pub cFlush: u64,
    pub cbFlush: u64,
    pub cMetaFlush: u64,
    pub cbMetaFlush: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CLS_IO_STATISTICS_HEADER {
    pub ubMajorVersion: u8,
    pub ubMinorVersion: u8,
    pub eStatsClass: CLFS_IOSTATS_CLASS,
    pub cbLength: u16,
    pub coffData: u32,
}
pub type CLS_LOG_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct CLS_LSN {
    pub Internal: u64,
}
pub type CLS_RECORD_TYPE = u8;
#[repr(C, align(8))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct CLS_SCAN_CONTEXT {
    pub cidNode: CLFS_NODE_ID,
    pub hLog: super::winnt::HANDLE,
    pub cIndex: u32,
    pub cContainers: u32,
    pub cContainersReturned: u32,
    pub eScanMode: CLFS_SCAN_MODE,
    pub pinfoContainer: PCLS_CONTAINER_INFORMATION,
}
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_winnt")]
impl Default for CLS_SCAN_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy)]
pub struct CLS_SCAN_CONTEXT {
    pub cidNode: CLFS_NODE_ID,
    pub hLog: super::winnt::HANDLE,
    pub cIndex: u32,
    pub cContainers: u32,
    pub cContainersReturned: u32,
    pub eScanMode: CLFS_SCAN_MODE,
    pub pinfoContainer: PCLS_CONTAINER_INFORMATION,
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winnt")]
impl Default for CLS_SCAN_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CLS_WRITE_ENTRY {
    pub Buffer: *mut core::ffi::c_void,
    pub ByteLength: u32,
}
impl Default for CLS_WRITE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ClfsContextForward: CLFS_CONTEXT_MODE = 3;
pub const ClfsContextNone: CLFS_CONTEXT_MODE = 0;
pub const ClfsContextPrevious: CLFS_CONTEXT_MODE = 2;
pub const ClfsContextUndoNext: CLFS_CONTEXT_MODE = 1;
pub const ClfsIoStatsDefault: CLFS_IOSTATS_CLASS = 0;
pub const ClfsIoStatsMax: CLFS_IOSTATS_CLASS = 65535;
pub const ClfsLogArchiveDisabled: CLFS_LOG_ARCHIVE_MODE = 2;
pub const ClfsLogArchiveEnabled: CLFS_LOG_ARCHIVE_MODE = 1;
pub const ClfsLogBasicInformation: CLS_LOG_INFORMATION_CLASS = 0;
pub const ClfsLogBasicInformationPhysical: CLS_LOG_INFORMATION_CLASS = 1;
pub const ClfsLogPhysicalLsnInformation: CLS_LOG_INFORMATION_CLASS = 5;
pub const ClfsLogPhysicalNameInformation: CLS_LOG_INFORMATION_CLASS = 2;
pub const ClfsLogStreamIdentifierInformation: CLS_LOG_INFORMATION_CLASS = 3;
pub const ClfsLogSystemMarkingInformation: CLS_LOG_INFORMATION_CLASS = 4;
pub const ClsContextForward: CLS_CONTEXT_MODE = 3;
pub const ClsContextNone: CLS_CONTEXT_MODE = 0;
pub const ClsContextPrevious: CLS_CONTEXT_MODE = 2;
pub const ClsContextUndoNext: CLS_CONTEXT_MODE = 1;
pub const ClsIoStatsDefault: CLS_IOSTATS_CLASS = 0;
pub const ClsIoStatsMax: CLS_IOSTATS_CLASS = 65535;
pub const EA_CONTAINER_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("ContainerName");
pub const EA_CONTAINER_SIZE: windows_sys::core::PCSTR = windows_sys::core::s!("ContainerSize");
pub const FILE_ATTRIBUTE_DEDICATED: u32 = 256;
pub type PCLFS_ARCHIVE_DESCRIPTOR = *mut CLFS_ARCHIVE_DESCRIPTOR;
pub type PCLFS_CONTAINER_ID = *mut CLFS_CONTAINER_ID;
pub type PCLFS_CONTAINER_INFORMATION = *mut CLFS_CONTAINER_INFORMATION;
pub type PCLFS_CONTAINER_STATE = *mut CLS_CONTAINER_STATE;
pub type PCLFS_CONTEXT_MODE = *mut CLFS_CONTEXT_MODE;
pub type PCLFS_INFORMATION = *mut CLFS_INFORMATION;
pub type PCLFS_IOSTATS_CLASS = *mut CLFS_IOSTATS_CLASS;
pub type PCLFS_IO_STATISTICS = *mut CLFS_IO_STATISTICS;
pub type PCLFS_IO_STATISTICS_HEADER = *mut CLFS_IO_STATISTICS_HEADER;
pub type PCLFS_LOG_ARCHIVE_MODE = *mut CLFS_LOG_ARCHIVE_MODE;
pub type PCLFS_LOG_INFORMATION_CLASS = *mut CLFS_LOG_INFORMATION_CLASS;
pub type PCLFS_LOG_NAME_INFORMATION = *mut CLFS_LOG_NAME_INFORMATION;
pub type PCLFS_LSN = *mut CLFS_LSN;
pub type PCLFS_NODE_ID = *mut CLFS_NODE_ID;
pub type PCLFS_PHYSICAL_LSN_INFORMATION = *mut CLFS_PHYSICAL_LSN_INFORMATION;
pub type PCLFS_RECORD_TYPE = *mut CLS_RECORD_TYPE;
#[cfg(feature = "Win32_winnt")]
pub type PCLFS_SCAN_CONTEXT = *mut CLFS_SCAN_CONTEXT;
pub type PCLFS_SCAN_MODE = *mut u8;
pub type PCLFS_STREAM_ID_INFORMATION = *mut CLFS_STREAM_ID_INFORMATION;
pub type PCLFS_WRITE_ENTRY = *mut CLFS_WRITE_ENTRY;
pub type PCLS_ARCHIVE_DESCRIPTOR = *mut CLS_ARCHIVE_DESCRIPTOR;
pub type PCLS_CONTAINER_INFORMATION = *mut CLS_CONTAINER_INFORMATION;
pub type PCLS_CONTAINER_STATE = *mut u32;
pub type PCLS_CONTEXT_MODE = *mut CLS_CONTEXT_MODE;
pub type PCLS_INFORMATION = *mut CLS_INFORMATION;
pub type PCLS_IOSTATS_CLASS = *mut CLS_IOSTATS_CLASS;
pub type PCLS_IO_STATISTICS = *mut CLS_IO_STATISTICS;
pub type PCLS_IO_STATISTICS_HEADER = *mut CLS_IO_STATISTICS_HEADER;
pub type PCLS_LOG_INFORMATION_CLASS = *mut CLS_LOG_INFORMATION_CLASS;
pub type PCLS_LSN = *mut CLS_LSN;
pub type PCLS_RECORD_TYPE = *mut u8;
#[cfg(feature = "Win32_winnt")]
pub type PCLS_SCAN_CONTEXT = *mut CLS_SCAN_CONTEXT;
pub type PCLS_WRITE_ENTRY = *mut CLS_WRITE_ENTRY;
#[cfg(feature = "Win32_corecrt_wstdio")]
pub type PFILE = *mut super::corecrt_wstdio::FILE;
pub type PPCLFS_ARCHIVE_DESCRIPTOR = *mut *mut CLFS_ARCHIVE_DESCRIPTOR;
pub type PPCLFS_CONTAINER_ID = *mut *mut CLFS_CONTAINER_ID;
pub type PPCLFS_CONTAINER_INFORMATION = *mut *mut CLFS_CONTAINER_INFORMATION;
pub type PPCLFS_CONTAINER_STATE = *mut CLS_CONTAINER_STATE;
pub type PPCLFS_CONTEXT_MODE = *mut *mut CLFS_CONTEXT_MODE;
pub type PPCLFS_INFORMATION = *mut CLFS_INFORMATION;
pub type PPCLFS_IOSTATS_CLASS = *mut *mut CLFS_IOSTATS_CLASS;
pub type PPCLFS_IO_STATISTICS = *mut *mut CLFS_IO_STATISTICS;
pub type PPCLFS_IO_STATISTICS_HEADER = *mut *mut CLFS_IO_STATISTICS_HEADER;
pub type PPCLFS_LOG_INFORMATION_CLASS = *mut *mut CLFS_LOG_INFORMATION_CLASS;
pub type PPCLFS_LOG_NAME_INFORMATION = *mut *mut CLFS_LOG_NAME_INFORMATION;
pub type PPCLFS_LSN = *mut *mut CLFS_LSN;
pub type PPCLFS_RECORD_TYPE = *mut *mut CLS_RECORD_TYPE;
#[cfg(feature = "Win32_winnt")]
pub type PPCLFS_SCAN_CONTEXT = *mut *mut CLFS_SCAN_CONTEXT;
pub type PPCLFS_STREAM_ID_INFORMATION = *mut *mut CLFS_STREAM_ID_INFORMATION;
pub type PPCLFS_WRITE_ENTRY = *mut *mut CLFS_WRITE_ENTRY;
pub type PPCLS_ARCHIVE_DESCRIPTOR = *mut *mut CLS_ARCHIVE_DESCRIPTOR;
pub type PPCLS_CONTAINER_INFORMATION = *mut *mut CLS_CONTAINER_INFORMATION;
pub type PPCLS_CONTAINER_STATE = *mut u32;
pub type PPCLS_CONTEXT_MODE = *mut *mut CLS_CONTEXT_MODE;
pub type PPCLS_INFORMATION = *mut CLS_INFORMATION;
pub type PPCLS_IOSTATS_CLASS = *mut *mut CLS_IOSTATS_CLASS;
pub type PPCLS_IO_STATISTICS = *mut *mut CLS_IO_STATISTICS;
pub type PPCLS_IO_STATISTICS_HEADER = *mut *mut CLS_IO_STATISTICS_HEADER;
pub type PPCLS_LOG_INFORMATION_CLASS = *mut *mut CLS_LOG_INFORMATION_CLASS;
pub type PPCLS_LSN = *mut *mut CLS_LSN;
pub type PPCLS_RECORD_TYPE = *mut *mut u8;
#[cfg(feature = "Win32_winnt")]
pub type PPCLS_SCAN_CONTEXT = *mut *mut CLS_SCAN_CONTEXT;
pub type PPCLS_WRITE_ENTRY = *mut *mut CLS_WRITE_ENTRY;
#[cfg(feature = "Win32_corecrt_wstdio")]
pub type PPFILE = *mut *mut super::corecrt_wstdio::FILE;
