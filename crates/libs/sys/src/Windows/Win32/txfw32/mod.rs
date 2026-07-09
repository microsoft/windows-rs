windows_link::link!("txfw32.dll" "system" fn TxfGetThreadMiniVersionForCreate(miniversion : *mut u16));
#[cfg(feature = "Win32_clfs")]
windows_link::link!("txfw32.dll" "system" fn TxfLogCreateFileReadContext(logpath : windows_sys::core::PCWSTR, beginninglsn : super::clfs::CLFS_LSN, endinglsn : super::clfs::CLFS_LSN, txffileid : *const TXF_ID, txflogcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "Win32_clfs")]
windows_link::link!("txfw32.dll" "system" fn TxfLogCreateRangeReadContext(logpath : windows_sys::core::PCWSTR, beginninglsn : super::clfs::CLFS_LSN, endinglsn : super::clfs::CLFS_LSN, beginningvirtualclock : *const i64, endingvirtualclock : *const i64, recordtypemask : u32, txflogcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("txfw32.dll" "system" fn TxfLogDestroyReadContext(txflogcontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("txfw32.dll" "system" fn TxfLogReadRecords(txflogcontext : *const core::ffi::c_void, bufferlength : u32, buffer : *mut core::ffi::c_void, bytesused : *mut u32, recordcount : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("txfw32.dll" "system" fn TxfLogRecordGetFileName(recordbuffer : *const core::ffi::c_void, recordbufferlengthinbytes : u32, namebuffer : windows_sys::core::PWSTR, namebufferlengthinbytes : *mut u32, txfid : *mut TXF_ID) -> windows_sys::core::BOOL);
windows_link::link!("txfw32.dll" "system" fn TxfLogRecordGetGenericType(recordbuffer : *const core::ffi::c_void, recordbufferlengthinbytes : u32, generictype : *mut u32, virtualclock : *mut i64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "Win32_clfs", feature = "Win32_winnt"))]
windows_link::link!("txfw32.dll" "system" fn TxfReadMetadataInfo(filehandle : super::winnt::HANDLE, txffileid : *mut TXF_ID, lastlsn : *mut super::clfs::CLFS_LSN, transactionstate : *mut u32, lockingtransaction : *mut windows_sys::core::GUID) -> windows_sys::core::BOOL);
windows_link::link!("txfw32.dll" "system" fn TxfSetThreadMiniVersionForCreate(miniversion : u16));
pub type PGUID = *mut windows_sys::core::GUID;
pub type PTXF_ID = *mut TXF_ID;
pub type PTXF_LOG_RECORD_AFFECTED_FILE = *mut TXF_LOG_RECORD_AFFECTED_FILE;
pub type PTXF_LOG_RECORD_BASE = *mut TXF_LOG_RECORD_BASE;
pub type PTXF_LOG_RECORD_TRUNCATE = *mut TXF_LOG_RECORD_TRUNCATE;
pub type PTXF_LOG_RECORD_WRITE = *mut TXF_LOG_RECORD_WRITE;
pub const TXFS_MINIVERSION_COMMITTED_VIEW: u32 = 0;
pub const TXFS_MINIVERSION_DEFAULT_VIEW: u32 = 65534;
pub const TXFS_MINIVERSION_DIRTY_VIEW: u32 = 65535;
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
pub const TXF_LOG_RECORD_GENERIC_TYPE_ABORT: u32 = 2;
pub const TXF_LOG_RECORD_GENERIC_TYPE_COMMIT: u32 = 1;
pub const TXF_LOG_RECORD_GENERIC_TYPE_DATA: u32 = 8;
pub const TXF_LOG_RECORD_GENERIC_TYPE_PREPARE: u32 = 4;
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
pub const TXF_LOG_RECORD_TYPE_AFFECTED_FILE: u32 = 4;
pub const TXF_LOG_RECORD_TYPE_TRUNCATE: u32 = 2;
pub const TXF_LOG_RECORD_TYPE_WRITE: u32 = 1;
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
