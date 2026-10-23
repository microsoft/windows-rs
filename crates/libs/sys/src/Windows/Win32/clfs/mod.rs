#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" "LsnBlockOffset" fn ClfsLsnBlockOffset(plsn : *const CLFS_LSN) -> super::ULONG);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" "LsnContainer" fn ClfsLsnContainer(plsn : *const CLFS_LSN) -> CLFS_CONTAINER_ID);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" "LsnCreate" fn ClfsLsnCreate(cidcontainer : CLFS_CONTAINER_ID, offblock : super::ULONG, crecord : super::ULONG) -> CLFS_LSN);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" "LsnEqual" fn ClfsLsnEqual(plsn1 : *const CLFS_LSN, plsn2 : *const CLFS_LSN) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" "LsnGreater" fn ClfsLsnGreater(plsn1 : *const CLFS_LSN, plsn2 : *const CLFS_LSN) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" "LsnIncrement" fn ClfsLsnIncrement(plsn : PCLFS_LSN) -> CLFS_LSN);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" "LsnInvalid" fn ClfsLsnInvalid(plsn : *const CLFS_LSN) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" "LsnLess" fn ClfsLsnLess(plsn1 : *const CLFS_LSN, plsn2 : *const CLFS_LSN) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" "LsnNull" fn ClfsLsnNull(plsn : *const CLFS_LSN) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" "LsnRecordSequence" fn ClfsLsnRecordSequence(plsn : *const CLFS_LSN) -> super::ULONG);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" fn LsnEqual(plsn1 : *const CLFS_LSN, plsn2 : *const CLFS_LSN) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" fn LsnGreater(plsn1 : *const CLFS_LSN, plsn2 : *const CLFS_LSN) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" fn LsnIncrement(plsn : PCLFS_LSN) -> CLFS_LSN);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" fn LsnInvalid(plsn : *const CLFS_LSN) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" fn LsnLess(plsn1 : *const CLFS_LSN, plsn2 : *const CLFS_LSN) -> super::BOOLEAN);
#[cfg(feature = "shared")]
windows_link::link!("clfsw32.dll" "system" fn LsnNull(plsn : *const CLFS_LSN) -> super::BOOLEAN);
pub type CLFSSTATUS = u32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type CLFS_ARCHIVE_DESCRIPTOR = CLS_ARCHIVE_DESCRIPTOR;
pub const CLFS_BASELOG_EXTENSION: windows_sys::core::PCWSTR = windows_sys::core::w!(".blf");
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type CLFS_BLOCK_ALLOCATION = Option<unsafe extern "C" fn(cbbufferlength: super::ULONG, pvusercontext: super::PVOID) -> super::PVOID>;
#[cfg(feature = "winnt")]
pub type CLFS_BLOCK_DEALLOCATION = Option<unsafe extern "C" fn(pvbuffer: super::PVOID, pvusercontext: super::PVOID)>;
pub type CLFS_CONTAINER_ID = u32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type CLFS_CONTAINER_INFORMATION = CLS_CONTAINER_INFORMATION;
pub const CLFS_CONTAINER_RELATIVE_PREFIX: windows_sys::core::PCWSTR = windows_sys::core::w!("%BLF%\\");
pub type CLFS_CONTAINER_STATE = u32;
pub const CLFS_CONTAINER_STREAM_PREFIX: windows_sys::core::PCWSTR = windows_sys::core::w!("%BLF%:");
pub type CLFS_CONTEXT_MODE = i32;
pub const CLFS_FLAG_FILTER_INTERMEDIATE_LEVEL: i32 = 16;
pub const CLFS_FLAG_FILTER_TOP_LEVEL: i32 = 32;
pub const CLFS_FLAG_FORCE_APPEND: i32 = 1;
pub const CLFS_FLAG_FORCE_FLUSH: i32 = 2;
pub const CLFS_FLAG_HIDDEN_SYSTEM_LOG: i32 = 512;
pub const CLFS_FLAG_IGNORE_SHARE_ACCESS: i32 = 64;
pub const CLFS_FLAG_MINIFILTER_LEVEL: i32 = 256;
pub const CLFS_FLAG_NON_REENTRANT_FILTER: i32 = 16;
pub const CLFS_FLAG_NO_FLAGS: i32 = 0;
pub const CLFS_FLAG_READ_IN_PROGRESS: i32 = 128;
pub const CLFS_FLAG_REENTRANT_FILE_SYSTEM: i32 = 8;
pub const CLFS_FLAG_REENTRANT_FILTER: i32 = 32;
pub const CLFS_FLAG_USE_RESERVATION: i32 = 4;
#[cfg(feature = "shared")]
pub type CLFS_INFORMATION = CLS_INFORMATION;
pub type CLFS_IOSTATS_CLASS = i32;
#[cfg(feature = "shared")]
pub type CLFS_IO_STATISTICS = CLS_IO_STATISTICS;
#[cfg(feature = "shared")]
pub type CLFS_IO_STATISTICS_HEADER = CLS_IO_STATISTICS_HEADER;
pub type CLFS_LOG_ARCHIVE_MODE = i32;
pub type CLFS_LOG_ID = windows_sys::core::GUID;
pub type CLFS_LOG_INFORMATION_CLASS = CLS_LOG_INFORMATION_CLASS;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CLFS_LOG_NAME_INFORMATION {
    pub NameLengthInBytes: super::USHORT,
    pub Name: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CLFS_LOG_NAME_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub type CLFS_LSN = CLS_LSN;
pub const CLFS_MARSHALLING_FLAG_DISABLE_BUFF_INIT: i32 = 1;
pub const CLFS_MARSHALLING_FLAG_NONE: i32 = 0;
#[cfg(feature = "shared")]
pub const CLFS_MAX_CONTAINER_INFO: super::ULONG = 256;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CLFS_NODE_ID {
    pub cType: super::ULONG,
    pub cbNode: super::ULONG,
}
#[repr(C, packed(8))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CLFS_PHYSICAL_LSN_INFORMATION {
    pub StreamIdentifier: super::UCHAR,
    pub VirtualLsn: CLFS_LSN,
    pub PhysicalLsn: CLFS_LSN,
}
pub type CLFS_RECORD_TYPE = u8;
#[cfg(feature = "shared")]
pub const CLFS_SCAN_BACKWARD: super::UCHAR = 4;
#[cfg(feature = "shared")]
pub const CLFS_SCAN_BUFFERED: super::UCHAR = 32;
#[cfg(feature = "shared")]
pub const CLFS_SCAN_CLOSE: super::UCHAR = 8;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type CLFS_SCAN_CONTEXT = CLS_SCAN_CONTEXT;
#[cfg(feature = "shared")]
pub const CLFS_SCAN_FORWARD: super::UCHAR = 2;
#[cfg(feature = "shared")]
pub const CLFS_SCAN_INIT: super::UCHAR = 1;
#[cfg(feature = "shared")]
pub const CLFS_SCAN_INITIALIZED: super::UCHAR = 16;
pub type CLFS_SCAN_MODE = u8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CLFS_STREAM_ID_INFORMATION {
    pub StreamIdentifier: super::UCHAR,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type CLFS_WRITE_ENTRY = CLS_WRITE_ENTRY;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CLS_ARCHIVE_DESCRIPTOR {
    pub coffLow: super::ULONGLONG,
    pub coffHigh: super::ULONGLONG,
    pub infoContainer: CLS_CONTAINER_INFORMATION,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CLS_CONTAINER_INFORMATION {
    pub FileAttributes: super::ULONG,
    pub CreationTime: super::ULONGLONG,
    pub LastAccessTime: super::ULONGLONG,
    pub LastWriteTime: super::ULONGLONG,
    pub ContainerSize: super::LONGLONG,
    pub FileNameActualLength: super::ULONG,
    pub FileNameLength: super::ULONG,
    pub FileName: [super::WCHAR; 256],
    pub State: CLFS_CONTAINER_STATE,
    pub PhysicalContainerId: CLFS_CONTAINER_ID,
    pub LogicalContainerId: CLFS_CONTAINER_ID,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CLS_CONTAINER_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type CLS_CONTAINER_STATE = u32;
pub type CLS_CONTEXT_MODE = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CLS_INFORMATION {
    pub TotalAvailable: super::LONGLONG,
    pub CurrentAvailable: super::LONGLONG,
    pub TotalReservation: super::LONGLONG,
    pub BaseFileSize: super::ULONGLONG,
    pub ContainerSize: super::ULONGLONG,
    pub TotalContainers: super::ULONG,
    pub FreeContainers: super::ULONG,
    pub TotalClients: super::ULONG,
    pub Attributes: super::ULONG,
    pub FlushThreshold: super::ULONG,
    pub SectorSize: super::ULONG,
    pub MinArchiveTailLsn: CLS_LSN,
    pub BaseLsn: CLS_LSN,
    pub LastFlushedLsn: CLS_LSN,
    pub LastLsn: CLS_LSN,
    pub RestartLsn: CLS_LSN,
    pub Identity: windows_sys::core::GUID,
}
pub type CLS_IOSTATS_CLASS = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CLS_IO_STATISTICS {
    pub hdrIoStats: CLS_IO_STATISTICS_HEADER,
    pub cFlush: super::ULONGLONG,
    pub cbFlush: super::ULONGLONG,
    pub cMetaFlush: super::ULONGLONG,
    pub cbMetaFlush: super::ULONGLONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CLS_IO_STATISTICS_HEADER {
    pub ubMajorVersion: super::UCHAR,
    pub ubMinorVersion: super::UCHAR,
    pub eStatsClass: CLFS_IOSTATS_CLASS,
    pub cbLength: super::USHORT,
    pub coffData: super::ULONG,
}
pub type CLS_LOG_INFORMATION_CLASS = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct CLS_LSN {
    pub Internal: super::ULONGLONG,
}
pub type CLS_RECORD_TYPE = u8;
#[repr(C, align(8))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct CLS_SCAN_CONTEXT {
    pub cidNode: CLFS_NODE_ID,
    pub hLog: super::HANDLE,
    #[doc(hidden)]
    pub __padding_2: [core::mem::MaybeUninit<u8>; {
        let offset = 0usize;
        let alignment =::core::mem::align_of::<CLFS_NODE_ID>();
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<CLFS_NODE_ID>();
        let alignment =::core::mem::align_of::<super::HANDLE>();
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<super::HANDLE>();
        let natural =::core::mem::align_of::<super::ULONG>();
        let required = if natural < 8 { 8 } else { natural };
        ((offset + required - 1) & !(required - 1)) - ((offset + natural - 1) & !(natural - 1))
    }],
    pub cIndex: super::ULONG,
    #[doc(hidden)]
    pub __padding_3: [core::mem::MaybeUninit<u8>; {
        let offset = 0usize;
        let alignment =::core::mem::align_of::<CLFS_NODE_ID>();
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<CLFS_NODE_ID>();
        let alignment =::core::mem::align_of::<super::HANDLE>();
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<super::HANDLE>();
        let alignment =::core::mem::align_of::<super::ULONG>();
        let alignment = if alignment < 8 { 8 } else { alignment };
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<super::ULONG>();
        let natural =::core::mem::align_of::<super::ULONG>();
        let required = if natural < 8 { 8 } else { natural };
        ((offset + required - 1) & !(required - 1)) - ((offset + natural - 1) & !(natural - 1))
    }],
    pub cContainers: super::ULONG,
    #[doc(hidden)]
    pub __padding_4: [core::mem::MaybeUninit<u8>; {
        let offset = 0usize;
        let alignment =::core::mem::align_of::<CLFS_NODE_ID>();
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<CLFS_NODE_ID>();
        let alignment =::core::mem::align_of::<super::HANDLE>();
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<super::HANDLE>();
        let alignment =::core::mem::align_of::<super::ULONG>();
        let alignment = if alignment < 8 { 8 } else { alignment };
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<super::ULONG>();
        let alignment =::core::mem::align_of::<super::ULONG>();
        let alignment = if alignment < 8 { 8 } else { alignment };
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<super::ULONG>();
        let natural =::core::mem::align_of::<super::ULONG>();
        let required = if natural < 8 { 8 } else { natural };
        ((offset + required - 1) & !(required - 1)) - ((offset + natural - 1) & !(natural - 1))
    }],
    pub cContainersReturned: super::ULONG,
    #[doc(hidden)]
    pub __padding_5: [core::mem::MaybeUninit<u8>; {
        let offset = 0usize;
        let alignment =::core::mem::align_of::<CLFS_NODE_ID>();
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<CLFS_NODE_ID>();
        let alignment =::core::mem::align_of::<super::HANDLE>();
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<super::HANDLE>();
        let alignment =::core::mem::align_of::<super::ULONG>();
        let alignment = if alignment < 8 { 8 } else { alignment };
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<super::ULONG>();
        let alignment =::core::mem::align_of::<super::ULONG>();
        let alignment = if alignment < 8 { 8 } else { alignment };
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<super::ULONG>();
        let alignment =::core::mem::align_of::<super::ULONG>();
        let alignment = if alignment < 8 { 8 } else { alignment };
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<super::ULONG>();
        let natural =::core::mem::align_of::<CLFS_SCAN_MODE>();
        let required = if natural < 8 { 8 } else { natural };
        ((offset + required - 1) & !(required - 1)) - ((offset + natural - 1) & !(natural - 1))
    }],
    pub eScanMode: CLFS_SCAN_MODE,
    #[doc(hidden)]
    pub __padding_6: [core::mem::MaybeUninit<u8>; {
        let offset = 0usize;
        let alignment =::core::mem::align_of::<CLFS_NODE_ID>();
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<CLFS_NODE_ID>();
        let alignment =::core::mem::align_of::<super::HANDLE>();
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<super::HANDLE>();
        let alignment =::core::mem::align_of::<super::ULONG>();
        let alignment = if alignment < 8 { 8 } else { alignment };
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<super::ULONG>();
        let alignment =::core::mem::align_of::<super::ULONG>();
        let alignment = if alignment < 8 { 8 } else { alignment };
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<super::ULONG>();
        let alignment =::core::mem::align_of::<super::ULONG>();
        let alignment = if alignment < 8 { 8 } else { alignment };
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<super::ULONG>();
        let alignment =::core::mem::align_of::<CLFS_SCAN_MODE>();
        let alignment = if alignment < 8 { 8 } else { alignment };
        let offset = ((offset + alignment - 1) & !(alignment - 1)) +::core::mem::size_of::<CLFS_SCAN_MODE>();
        let natural =::core::mem::align_of::<PCLS_CONTAINER_INFORMATION>();
        let required = if natural < 8 { 8 } else { natural };
        ((offset + required - 1) & !(required - 1)) - ((offset + natural - 1) & !(natural - 1))
    }],
    pub pinfoContainer: PCLS_CONTAINER_INFORMATION,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for CLS_SCAN_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Default)]
pub struct CLS_WRITE_ENTRY {
    pub Buffer: super::PVOID,
    pub ByteLength: super::ULONG,
}
#[cfg(feature = "shared")]
pub const ClfsClientRecord: super::UCHAR = 3;
pub const ClfsContainerActive: CLFS_CONTAINER_STATE = 4;
pub const ClfsContainerActivePendingDelete: CLFS_CONTAINER_STATE = 8;
pub const ClfsContainerInactive: CLFS_CONTAINER_STATE = 2;
pub const ClfsContainerInitializing: CLFS_CONTAINER_STATE = 1;
pub const ClfsContainerPendingArchive: CLFS_CONTAINER_STATE = 16;
pub const ClfsContainerPendingArchiveAndDelete: CLFS_CONTAINER_STATE = 32;
pub const ClfsContextForward: CLFS_CONTEXT_MODE = 3;
pub const ClfsContextNone: CLFS_CONTEXT_MODE = 0;
pub const ClfsContextPrevious: CLFS_CONTEXT_MODE = 2;
pub const ClfsContextUndoNext: CLFS_CONTEXT_MODE = 1;
#[cfg(feature = "shared")]
pub const ClfsDataRecord: super::UCHAR = 1;
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
#[cfg(feature = "shared")]
pub const ClfsNullRecord: super::UCHAR = 0;
#[cfg(feature = "shared")]
pub const ClfsRestartRecord: super::UCHAR = 2;
pub const ClsContainerActive: CLFS_CONTAINER_STATE = 4;
pub const ClsContainerActivePendingDelete: CLFS_CONTAINER_STATE = 8;
pub const ClsContainerInactive: CLFS_CONTAINER_STATE = 2;
pub const ClsContainerInitializing: CLFS_CONTAINER_STATE = 1;
pub const ClsContainerPendingArchive: CLFS_CONTAINER_STATE = 16;
pub const ClsContainerPendingArchiveAndDelete: CLFS_CONTAINER_STATE = 32;
pub const ClsContextForward: CLS_CONTEXT_MODE = 3;
pub const ClsContextNone: CLS_CONTEXT_MODE = 0;
pub const ClsContextPrevious: CLS_CONTEXT_MODE = 2;
pub const ClsContextUndoNext: CLS_CONTEXT_MODE = 1;
pub const ClsIoStatsDefault: CLS_IOSTATS_CLASS = 0;
pub const ClsIoStatsMax: CLS_IOSTATS_CLASS = 65535;
pub const EA_CONTAINER_NAME: windows_sys::core::PCSTR = windows_sys::core::s!("ContainerName");
pub const EA_CONTAINER_SIZE: windows_sys::core::PCSTR = windows_sys::core::s!("ContainerSize");
pub const FILE_ATTRIBUTE_DEDICATED: i32 = 256;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCLFS_ARCHIVE_DESCRIPTOR = *mut CLS_ARCHIVE_DESCRIPTOR;
pub type PCLFS_CONTAINER_ID = *mut u32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCLFS_CONTAINER_INFORMATION = *mut CLS_CONTAINER_INFORMATION;
pub type PCLFS_CONTAINER_STATE = *mut u32;
pub type PCLFS_CONTEXT_MODE = *mut CLFS_CONTEXT_MODE;
#[cfg(feature = "shared")]
pub type PCLFS_INFORMATION = *mut CLS_INFORMATION;
pub type PCLFS_IOSTATS_CLASS = *mut CLFS_IOSTATS_CLASS;
#[cfg(feature = "shared")]
pub type PCLFS_IO_STATISTICS = *mut CLS_IO_STATISTICS;
#[cfg(feature = "shared")]
pub type PCLFS_IO_STATISTICS_HEADER = *mut CLS_IO_STATISTICS_HEADER;
pub type PCLFS_LOG_ARCHIVE_MODE = *mut CLFS_LOG_ARCHIVE_MODE;
pub type PCLFS_LOG_INFORMATION_CLASS = *mut CLS_LOG_INFORMATION_CLASS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCLFS_LOG_NAME_INFORMATION = *mut CLFS_LOG_NAME_INFORMATION;
#[cfg(feature = "shared")]
pub type PCLFS_LSN = *mut CLS_LSN;
#[cfg(feature = "shared")]
pub type PCLFS_NODE_ID = *mut CLFS_NODE_ID;
#[cfg(feature = "shared")]
pub type PCLFS_PHYSICAL_LSN_INFORMATION = *mut CLFS_PHYSICAL_LSN_INFORMATION;
pub type PCLFS_RECORD_TYPE = *mut u8;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCLFS_SCAN_CONTEXT = *mut CLS_SCAN_CONTEXT;
pub type PCLFS_SCAN_MODE = *mut u8;
#[cfg(feature = "shared")]
pub type PCLFS_STREAM_ID_INFORMATION = *mut CLFS_STREAM_ID_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCLFS_WRITE_ENTRY = *mut CLS_WRITE_ENTRY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCLS_ARCHIVE_DESCRIPTOR = *mut CLS_ARCHIVE_DESCRIPTOR;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCLS_CONTAINER_INFORMATION = *mut CLS_CONTAINER_INFORMATION;
pub type PCLS_CONTAINER_STATE = *mut u32;
pub type PCLS_CONTEXT_MODE = *mut CLS_CONTEXT_MODE;
#[cfg(feature = "shared")]
pub type PCLS_INFORMATION = *mut CLS_INFORMATION;
pub type PCLS_IOSTATS_CLASS = *mut CLS_IOSTATS_CLASS;
#[cfg(feature = "shared")]
pub type PCLS_IO_STATISTICS = *mut CLS_IO_STATISTICS;
#[cfg(feature = "shared")]
pub type PCLS_IO_STATISTICS_HEADER = *mut CLS_IO_STATISTICS_HEADER;
pub type PCLS_LOG_INFORMATION_CLASS = *mut CLS_LOG_INFORMATION_CLASS;
#[cfg(feature = "shared")]
pub type PCLS_LSN = *mut CLS_LSN;
pub type PCLS_RECORD_TYPE = *mut u8;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCLS_SCAN_CONTEXT = *mut CLS_SCAN_CONTEXT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCLS_WRITE_ENTRY = *mut CLS_WRITE_ENTRY;
#[cfg(feature = "corecrt_wstdio")]
pub type PFILE = *mut super::FILE;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPCLFS_ARCHIVE_DESCRIPTOR = *mut *mut CLS_ARCHIVE_DESCRIPTOR;
pub type PPCLFS_CONTAINER_ID = *mut *mut u32;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPCLFS_CONTAINER_INFORMATION = *mut *mut CLS_CONTAINER_INFORMATION;
pub type PPCLFS_CONTAINER_STATE = *mut u32;
pub type PPCLFS_CONTEXT_MODE = *mut *mut CLFS_CONTEXT_MODE;
#[cfg(feature = "shared")]
pub type PPCLFS_INFORMATION = *mut CLS_INFORMATION;
pub type PPCLFS_IOSTATS_CLASS = *mut *mut CLFS_IOSTATS_CLASS;
#[cfg(feature = "shared")]
pub type PPCLFS_IO_STATISTICS = *mut *mut CLS_IO_STATISTICS;
#[cfg(feature = "shared")]
pub type PPCLFS_IO_STATISTICS_HEADER = *mut *mut CLS_IO_STATISTICS_HEADER;
pub type PPCLFS_LOG_INFORMATION_CLASS = *mut *mut CLS_LOG_INFORMATION_CLASS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPCLFS_LOG_NAME_INFORMATION = *mut *mut CLFS_LOG_NAME_INFORMATION;
#[cfg(feature = "shared")]
pub type PPCLFS_LSN = *mut *mut CLS_LSN;
pub type PPCLFS_RECORD_TYPE = *mut *mut u8;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPCLFS_SCAN_CONTEXT = *mut *mut CLS_SCAN_CONTEXT;
#[cfg(feature = "shared")]
pub type PPCLFS_STREAM_ID_INFORMATION = *mut *mut CLFS_STREAM_ID_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPCLFS_WRITE_ENTRY = *mut *mut CLS_WRITE_ENTRY;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPCLS_ARCHIVE_DESCRIPTOR = *mut *mut CLS_ARCHIVE_DESCRIPTOR;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPCLS_CONTAINER_INFORMATION = *mut *mut CLS_CONTAINER_INFORMATION;
pub type PPCLS_CONTAINER_STATE = *mut u32;
pub type PPCLS_CONTEXT_MODE = *mut *mut CLS_CONTEXT_MODE;
#[cfg(feature = "shared")]
pub type PPCLS_INFORMATION = *mut CLS_INFORMATION;
pub type PPCLS_IOSTATS_CLASS = *mut *mut CLS_IOSTATS_CLASS;
#[cfg(feature = "shared")]
pub type PPCLS_IO_STATISTICS = *mut *mut CLS_IO_STATISTICS;
#[cfg(feature = "shared")]
pub type PPCLS_IO_STATISTICS_HEADER = *mut *mut CLS_IO_STATISTICS_HEADER;
pub type PPCLS_LOG_INFORMATION_CLASS = *mut *mut CLS_LOG_INFORMATION_CLASS;
#[cfg(feature = "shared")]
pub type PPCLS_LSN = *mut *mut CLS_LSN;
pub type PPCLS_RECORD_TYPE = *mut *mut u8;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPCLS_SCAN_CONTEXT = *mut *mut CLS_SCAN_CONTEXT;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PPCLS_WRITE_ENTRY = *mut *mut CLS_WRITE_ENTRY;
#[cfg(feature = "corecrt_wstdio")]
pub type PPFILE = *mut *mut super::FILE;
