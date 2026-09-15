#[cfg(feature = "minwindef")]
windows_link::link!("txfw32.dll" "system" fn TxfGetThreadMiniVersionForCreate(miniversion : super::PUSHORT));
#[cfg(feature = "clfs")]
windows_link::link!("txfw32.dll" "system" fn TxfLogCreateFileReadContext(logpath : windows_sys::core::PCWSTR, beginninglsn : super::CLFS_LSN, endinglsn : super::CLFS_LSN, txffileid : PTXF_ID, txflogcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "winnt"))]
windows_link::link!("txfw32.dll" "system" fn TxfLogCreateRangeReadContext(logpath : windows_sys::core::PCWSTR, beginninglsn : super::CLFS_LSN, endinglsn : super::CLFS_LSN, beginningvirtualclock : super::PLARGE_INTEGER, endingvirtualclock : super::PLARGE_INTEGER, recordtypemask : u32, txflogcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("txfw32.dll" "system" fn TxfLogDestroyReadContext(txflogcontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("txfw32.dll" "system" fn TxfLogReadRecords(txflogcontext : *const core::ffi::c_void, bufferlength : u32, buffer : *mut core::ffi::c_void, bytesused : super::PULONG, recordcount : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("txfw32.dll" "system" fn TxfLogRecordGetFileName(recordbuffer : *const core::ffi::c_void, recordbufferlengthinbytes : u32, namebuffer : windows_sys::core::PWSTR, namebufferlengthinbytes : super::PULONG, txfid : PTXF_ID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("txfw32.dll" "system" fn TxfLogRecordGetGenericType(recordbuffer : *const core::ffi::c_void, recordbufferlengthinbytes : u32, generictype : super::PULONG, virtualclock : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "winnt"))]
windows_link::link!("txfw32.dll" "system" fn TxfReadMetadataInfo(filehandle : super::HANDLE, txffileid : PTXF_ID, lastlsn : super::PCLFS_LSN, transactionstate : super::PULONG, lockingtransaction : PGUID) -> windows_sys::core::BOOL);
windows_link::link!("txfw32.dll" "system" fn TxfSetThreadMiniVersionForCreate(miniversion : u16));
pub type PGUID = *mut windows_sys::core::GUID;
pub type PTXF_ID = *mut TXF_ID;
pub type PTXF_LOG_RECORD_AFFECTED_FILE = *mut TXF_LOG_RECORD_AFFECTED_FILE;
pub type PTXF_LOG_RECORD_BASE = *mut TXF_LOG_RECORD_BASE;
pub type PTXF_LOG_RECORD_TRUNCATE = *mut TXF_LOG_RECORD_TRUNCATE;
pub type PTXF_LOG_RECORD_WRITE = *mut TXF_LOG_RECORD_WRITE;
pub const TXFS_MINIVERSION_COMMITTED_VIEW: i32 = 0;
pub const TXFS_MINIVERSION_DEFAULT_VIEW: i32 = 65534;
pub const TXFS_MINIVERSION_DIRTY_VIEW: i32 = 65535;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TXF_ID {
    pub Anonymous: TXF_ID_0,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct TXF_ID_0 {
    pub LowPart: i64,
    pub HighPart: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TXF_LOG_RECORD_AFFECTED_FILE {
    pub Version: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: windows_sys::core::GUID,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct TXF_LOG_RECORD_BASE {
    pub Version: u16,
    pub RecordType: u16,
    pub RecordLength: u32,
}
pub const TXF_LOG_RECORD_GENERIC_TYPE_ABORT: i32 = 2;
pub const TXF_LOG_RECORD_GENERIC_TYPE_COMMIT: i32 = 1;
pub const TXF_LOG_RECORD_GENERIC_TYPE_DATA: i32 = 8;
pub const TXF_LOG_RECORD_GENERIC_TYPE_PREPARE: i32 = 4;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct TXF_LOG_RECORD_TRUNCATE {
    pub Version: u16,
    pub RecordType: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: windows_sys::core::GUID,
    pub NewFileSize: i64,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
pub const TXF_LOG_RECORD_TYPE_AFFECTED_FILE: i32 = 4;
pub const TXF_LOG_RECORD_TYPE_TRUNCATE: i32 = 2;
pub const TXF_LOG_RECORD_TYPE_WRITE: i32 = 1;
#[repr(C, packed(4))]
#[derive(Clone, Copy, Default)]
pub struct TXF_LOG_RECORD_WRITE {
    pub Version: u16,
    pub RecordType: u16,
    pub RecordLength: u32,
    pub Flags: u32,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: windows_sys::core::GUID,
    pub ByteOffsetInFile: i64,
    pub NumBytesWritten: u32,
    pub ByteOffsetInStructure: u32,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
