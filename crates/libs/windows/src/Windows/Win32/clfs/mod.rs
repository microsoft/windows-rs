#[inline]
pub unsafe fn LsnBlockOffset(plsn: *const CLFS_LSN) -> u32 {
    windows_core::link!("clfsw32.dll" "C" fn LsnBlockOffset(plsn : *const CLFS_LSN) -> u32);
    unsafe { LsnBlockOffset(plsn) }
}
#[inline]
pub unsafe fn LsnContainer(plsn: *const CLFS_LSN) -> CLFS_CONTAINER_ID {
    windows_core::link!("clfsw32.dll" "C" fn LsnContainer(plsn : *const CLFS_LSN) -> CLFS_CONTAINER_ID);
    unsafe { LsnContainer(plsn) }
}
#[inline]
pub unsafe fn LsnCreate(cidcontainer: CLFS_CONTAINER_ID, offblock: u32, crecord: u32) -> CLFS_LSN {
    windows_core::link!("clfsw32.dll" "C" fn LsnCreate(cidcontainer : CLFS_CONTAINER_ID, offblock : u32, crecord : u32) -> CLFS_LSN);
    unsafe { LsnCreate(cidcontainer, offblock, crecord) }
}
#[inline]
pub unsafe fn LsnEqual(plsn1: *const CLFS_LSN, plsn2: *const CLFS_LSN) -> bool {
    windows_core::link!("clfsw32.dll" "C" fn LsnEqual(plsn1 : *const CLFS_LSN, plsn2 : *const CLFS_LSN) -> bool);
    unsafe { LsnEqual(plsn1, plsn2) }
}
#[inline]
pub unsafe fn LsnGreater(plsn1: *const CLFS_LSN, plsn2: *const CLFS_LSN) -> bool {
    windows_core::link!("clfsw32.dll" "C" fn LsnGreater(plsn1 : *const CLFS_LSN, plsn2 : *const CLFS_LSN) -> bool);
    unsafe { LsnGreater(plsn1, plsn2) }
}
#[inline]
pub unsafe fn LsnIncrement(plsn: *const CLFS_LSN) -> CLFS_LSN {
    windows_core::link!("clfsw32.dll" "C" fn LsnIncrement(plsn : *const CLFS_LSN) -> CLFS_LSN);
    unsafe { LsnIncrement(plsn) }
}
#[inline]
pub unsafe fn LsnInvalid(plsn: *const CLFS_LSN) -> bool {
    windows_core::link!("clfsw32.dll" "C" fn LsnInvalid(plsn : *const CLFS_LSN) -> bool);
    unsafe { LsnInvalid(plsn) }
}
#[inline]
pub unsafe fn LsnLess(plsn1: *const CLFS_LSN, plsn2: *const CLFS_LSN) -> bool {
    windows_core::link!("clfsw32.dll" "C" fn LsnLess(plsn1 : *const CLFS_LSN, plsn2 : *const CLFS_LSN) -> bool);
    unsafe { LsnLess(plsn1, plsn2) }
}
#[inline]
pub unsafe fn LsnNull(plsn: *const CLFS_LSN) -> bool {
    windows_core::link!("clfsw32.dll" "C" fn LsnNull(plsn : *const CLFS_LSN) -> bool);
    unsafe { LsnNull(plsn) }
}
#[inline]
pub unsafe fn LsnRecordSequence(plsn: *const CLFS_LSN) -> u32 {
    windows_core::link!("clfsw32.dll" "C" fn LsnRecordSequence(plsn : *const CLFS_LSN) -> u32);
    unsafe { LsnRecordSequence(plsn) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CLFSSTATUS(pub u32);
pub type CLFS_ARCHIVE_DESCRIPTOR = CLS_ARCHIVE_DESCRIPTOR;
pub const CLFS_BASELOG_EXTENSION: windows_core::PCWSTR = windows_core::w!(".blf");
pub type CLFS_BLOCK_ALLOCATION = Option<unsafe extern "system" fn(cbbufferlength: u32, pvusercontext: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
pub type CLFS_BLOCK_DEALLOCATION = Option<unsafe extern "system" fn(pvbuffer: *mut core::ffi::c_void, pvusercontext: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CLFS_CONTAINER_ID(pub u32);
pub type CLFS_CONTAINER_INFORMATION = CLS_CONTAINER_INFORMATION;
pub const CLFS_CONTAINER_RELATIVE_PREFIX: windows_core::PCWSTR = windows_core::w!("%BLF%\\");
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CLFS_CONTAINER_STATE(pub CLS_CONTAINER_STATE);
pub const CLFS_CONTAINER_STREAM_PREFIX: windows_core::PCWSTR = windows_core::w!("%BLF%:");
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
pub type CLFS_LOG_ID = windows_core::GUID;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CLFS_LOG_INFORMATION_CLASS(pub CLS_LOG_INFORMATION_CLASS);
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLFS_NODE_ID {
    pub cType: u32,
    pub cbNode: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLFS_PHYSICAL_LSN_INFORMATION {
    pub StreamIdentifier: u8,
    pub VirtualLsn: CLFS_LSN,
    pub PhysicalLsn: CLFS_LSN,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CLFS_RECORD_TYPE(pub CLS_RECORD_TYPE);
#[cfg(feature = "Win32_winnt")]
pub type CLFS_SCAN_CONTEXT = CLS_SCAN_CONTEXT;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CLFS_SCAN_MODE(pub u8);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLFS_STREAM_ID_INFORMATION {
    pub StreamIdentifier: u8,
}
pub type CLFS_WRITE_ENTRY = CLS_WRITE_ENTRY;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLS_ARCHIVE_DESCRIPTOR {
    pub coffLow: u64,
    pub coffHigh: u64,
    pub infoContainer: CLS_CONTAINER_INFORMATION,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CLS_CONTAINER_STATE(pub u32);
pub type CLS_CONTEXT_MODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub Identity: windows_core::GUID,
}
pub type CLS_IOSTATS_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLS_IO_STATISTICS {
    pub hdrIoStats: CLS_IO_STATISTICS_HEADER,
    pub cFlush: u64,
    pub cbFlush: u64,
    pub cMetaFlush: u64,
    pub cbMetaFlush: u64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLS_IO_STATISTICS_HEADER {
    pub ubMajorVersion: u8,
    pub ubMinorVersion: u8,
    pub eStatsClass: CLFS_IOSTATS_CLASS,
    pub cbLength: u16,
    pub coffData: u32,
}
pub type CLS_LOG_INFORMATION_CLASS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLS_LSN {
    pub Internal: u64,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CLS_RECORD_TYPE(pub u8);
#[repr(C, align(8))]
#[cfg(target_arch = "x86")]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLS_SCAN_CONTEXT {
    pub cidNode: CLFS_NODE_ID,
    pub hLog: super::winnt::HANDLE,
    pub cIndex: u32,
    pub cContainers: u32,
    pub cContainersReturned: u32,
    pub eScanMode: CLFS_SCAN_MODE,
    pub pinfoContainer: PCLS_CONTAINER_INFORMATION,
}
#[repr(C)]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winnt")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CLS_SCAN_CONTEXT {
    pub cidNode: CLFS_NODE_ID,
    pub hLog: super::winnt::HANDLE,
    pub cIndex: u32,
    pub cContainers: u32,
    pub cContainersReturned: u32,
    pub eScanMode: CLFS_SCAN_MODE,
    pub pinfoContainer: PCLS_CONTAINER_INFORMATION,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
pub const EA_CONTAINER_NAME: windows_core::PCSTR = windows_core::s!("ContainerName");
pub const EA_CONTAINER_SIZE: windows_core::PCSTR = windows_core::s!("ContainerSize");
pub const FILE_ATTRIBUTE_DEDICATED: u32 = 256;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_ARCHIVE_DESCRIPTOR(pub *mut CLFS_ARCHIVE_DESCRIPTOR);
impl PCLFS_ARCHIVE_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_ARCHIVE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_CONTAINER_ID(pub *mut CLFS_CONTAINER_ID);
impl PCLFS_CONTAINER_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_CONTAINER_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_CONTAINER_INFORMATION(pub *mut CLFS_CONTAINER_INFORMATION);
impl PCLFS_CONTAINER_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_CONTAINER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_CONTAINER_STATE(pub *mut CLS_CONTAINER_STATE);
impl PCLFS_CONTAINER_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_CONTAINER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_CONTEXT_MODE(pub *mut CLFS_CONTEXT_MODE);
impl PCLFS_CONTEXT_MODE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_CONTEXT_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_INFORMATION(pub *mut CLFS_INFORMATION);
impl PCLFS_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_IOSTATS_CLASS(pub *mut CLFS_IOSTATS_CLASS);
impl PCLFS_IOSTATS_CLASS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_IOSTATS_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_IO_STATISTICS(pub *mut CLFS_IO_STATISTICS);
impl PCLFS_IO_STATISTICS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_IO_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_IO_STATISTICS_HEADER(pub *mut CLFS_IO_STATISTICS_HEADER);
impl PCLFS_IO_STATISTICS_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_IO_STATISTICS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_LOG_ARCHIVE_MODE(pub *mut CLFS_LOG_ARCHIVE_MODE);
impl PCLFS_LOG_ARCHIVE_MODE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_LOG_ARCHIVE_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_LOG_INFORMATION_CLASS(pub *mut CLFS_LOG_INFORMATION_CLASS);
impl PCLFS_LOG_INFORMATION_CLASS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_LOG_INFORMATION_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_LOG_NAME_INFORMATION(pub *mut CLFS_LOG_NAME_INFORMATION);
impl PCLFS_LOG_NAME_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_LOG_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_LSN(pub *mut CLFS_LSN);
impl PCLFS_LSN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_LSN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_NODE_ID(pub *mut CLFS_NODE_ID);
impl PCLFS_NODE_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_NODE_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_PHYSICAL_LSN_INFORMATION(pub *mut CLFS_PHYSICAL_LSN_INFORMATION);
impl PCLFS_PHYSICAL_LSN_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_PHYSICAL_LSN_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_RECORD_TYPE(pub *mut CLS_RECORD_TYPE);
impl PCLFS_RECORD_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_RECORD_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_SCAN_CONTEXT(pub *mut CLFS_SCAN_CONTEXT);
#[cfg(feature = "Win32_winnt")]
impl PCLFS_SCAN_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PCLFS_SCAN_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_SCAN_MODE(pub *mut u8);
impl PCLFS_SCAN_MODE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_SCAN_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_STREAM_ID_INFORMATION(pub *mut CLFS_STREAM_ID_INFORMATION);
impl PCLFS_STREAM_ID_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_STREAM_ID_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLFS_WRITE_ENTRY(pub *mut CLFS_WRITE_ENTRY);
impl PCLFS_WRITE_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLFS_WRITE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLS_ARCHIVE_DESCRIPTOR(pub *mut CLS_ARCHIVE_DESCRIPTOR);
impl PCLS_ARCHIVE_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLS_ARCHIVE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLS_CONTAINER_INFORMATION(pub *mut CLS_CONTAINER_INFORMATION);
impl PCLS_CONTAINER_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLS_CONTAINER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLS_CONTAINER_STATE(pub *mut u32);
impl PCLS_CONTAINER_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLS_CONTAINER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLS_CONTEXT_MODE(pub *mut CLS_CONTEXT_MODE);
impl PCLS_CONTEXT_MODE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLS_CONTEXT_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLS_INFORMATION(pub *mut CLS_INFORMATION);
impl PCLS_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLS_IOSTATS_CLASS(pub *mut CLS_IOSTATS_CLASS);
impl PCLS_IOSTATS_CLASS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLS_IOSTATS_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLS_IO_STATISTICS(pub *mut CLS_IO_STATISTICS);
impl PCLS_IO_STATISTICS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLS_IO_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLS_IO_STATISTICS_HEADER(pub *mut CLS_IO_STATISTICS_HEADER);
impl PCLS_IO_STATISTICS_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLS_IO_STATISTICS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLS_LOG_INFORMATION_CLASS(pub *mut CLS_LOG_INFORMATION_CLASS);
impl PCLS_LOG_INFORMATION_CLASS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLS_LOG_INFORMATION_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLS_LSN(pub *mut CLS_LSN);
impl PCLS_LSN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLS_LSN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLS_RECORD_TYPE(pub *mut u8);
impl PCLS_RECORD_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLS_RECORD_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLS_SCAN_CONTEXT(pub *mut CLS_SCAN_CONTEXT);
#[cfg(feature = "Win32_winnt")]
impl PCLS_SCAN_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PCLS_SCAN_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PCLS_WRITE_ENTRY(pub *mut CLS_WRITE_ENTRY);
impl PCLS_WRITE_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PCLS_WRITE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_corecrt_wstdio")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PFILE(pub *mut super::corecrt_wstdio::FILE);
#[cfg(feature = "Win32_corecrt_wstdio")]
impl PFILE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_corecrt_wstdio")]
impl Default for PFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_ARCHIVE_DESCRIPTOR(pub *mut *mut CLFS_ARCHIVE_DESCRIPTOR);
impl PPCLFS_ARCHIVE_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_ARCHIVE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_CONTAINER_ID(pub *mut *mut CLFS_CONTAINER_ID);
impl PPCLFS_CONTAINER_ID {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_CONTAINER_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_CONTAINER_INFORMATION(pub *mut *mut CLFS_CONTAINER_INFORMATION);
impl PPCLFS_CONTAINER_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_CONTAINER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_CONTAINER_STATE(pub *mut CLS_CONTAINER_STATE);
impl PPCLFS_CONTAINER_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_CONTAINER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_CONTEXT_MODE(pub *mut *mut CLFS_CONTEXT_MODE);
impl PPCLFS_CONTEXT_MODE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_CONTEXT_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_INFORMATION(pub *mut CLFS_INFORMATION);
impl PPCLFS_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_IOSTATS_CLASS(pub *mut *mut CLFS_IOSTATS_CLASS);
impl PPCLFS_IOSTATS_CLASS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_IOSTATS_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_IO_STATISTICS(pub *mut *mut CLFS_IO_STATISTICS);
impl PPCLFS_IO_STATISTICS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_IO_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_IO_STATISTICS_HEADER(pub *mut *mut CLFS_IO_STATISTICS_HEADER);
impl PPCLFS_IO_STATISTICS_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_IO_STATISTICS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_LOG_INFORMATION_CLASS(pub *mut *mut CLFS_LOG_INFORMATION_CLASS);
impl PPCLFS_LOG_INFORMATION_CLASS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_LOG_INFORMATION_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_LOG_NAME_INFORMATION(pub *mut *mut CLFS_LOG_NAME_INFORMATION);
impl PPCLFS_LOG_NAME_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_LOG_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_LSN(pub *mut *mut CLFS_LSN);
impl PPCLFS_LSN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_LSN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_RECORD_TYPE(pub *mut *mut CLS_RECORD_TYPE);
impl PPCLFS_RECORD_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_RECORD_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_SCAN_CONTEXT(pub *mut *mut CLFS_SCAN_CONTEXT);
#[cfg(feature = "Win32_winnt")]
impl PPCLFS_SCAN_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PPCLFS_SCAN_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_STREAM_ID_INFORMATION(pub *mut *mut CLFS_STREAM_ID_INFORMATION);
impl PPCLFS_STREAM_ID_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_STREAM_ID_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLFS_WRITE_ENTRY(pub *mut *mut CLFS_WRITE_ENTRY);
impl PPCLFS_WRITE_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLFS_WRITE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLS_ARCHIVE_DESCRIPTOR(pub *mut *mut CLS_ARCHIVE_DESCRIPTOR);
impl PPCLS_ARCHIVE_DESCRIPTOR {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLS_ARCHIVE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLS_CONTAINER_INFORMATION(pub *mut *mut CLS_CONTAINER_INFORMATION);
impl PPCLS_CONTAINER_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLS_CONTAINER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLS_CONTAINER_STATE(pub *mut u32);
impl PPCLS_CONTAINER_STATE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLS_CONTAINER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLS_CONTEXT_MODE(pub *mut *mut CLS_CONTEXT_MODE);
impl PPCLS_CONTEXT_MODE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLS_CONTEXT_MODE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLS_INFORMATION(pub *mut CLS_INFORMATION);
impl PPCLS_INFORMATION {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLS_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLS_IOSTATS_CLASS(pub *mut *mut CLS_IOSTATS_CLASS);
impl PPCLS_IOSTATS_CLASS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLS_IOSTATS_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLS_IO_STATISTICS(pub *mut *mut CLS_IO_STATISTICS);
impl PPCLS_IO_STATISTICS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLS_IO_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLS_IO_STATISTICS_HEADER(pub *mut *mut CLS_IO_STATISTICS_HEADER);
impl PPCLS_IO_STATISTICS_HEADER {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLS_IO_STATISTICS_HEADER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLS_LOG_INFORMATION_CLASS(pub *mut *mut CLS_LOG_INFORMATION_CLASS);
impl PPCLS_LOG_INFORMATION_CLASS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLS_LOG_INFORMATION_CLASS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLS_LSN(pub *mut *mut CLS_LSN);
impl PPCLS_LSN {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLS_LSN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLS_RECORD_TYPE(pub *mut *mut u8);
impl PPCLS_RECORD_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLS_RECORD_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLS_SCAN_CONTEXT(pub *mut *mut CLS_SCAN_CONTEXT);
#[cfg(feature = "Win32_winnt")]
impl PPCLS_SCAN_CONTEXT {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_winnt")]
impl Default for PPCLS_SCAN_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPCLS_WRITE_ENTRY(pub *mut *mut CLS_WRITE_ENTRY);
impl PPCLS_WRITE_ENTRY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PPCLS_WRITE_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_corecrt_wstdio")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PPFILE(pub *mut *mut super::corecrt_wstdio::FILE);
#[cfg(feature = "Win32_corecrt_wstdio")]
impl PPFILE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "Win32_corecrt_wstdio")]
impl Default for PPFILE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
