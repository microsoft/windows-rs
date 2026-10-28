#[cfg(feature = "minwindef")]
windows_link::link!("txfw32.dll" "system" fn TxfGetThreadMiniVersionForCreate(miniversion : super::PUSHORT));
#[cfg(all(feature = "clfs", feature = "shared", feature = "winnt"))]
windows_link::link!("txfw32.dll" "system" fn TxfLogCreateFileReadContext(logpath : windows_sys::core::PCWSTR, beginninglsn : super::CLFS_LSN, endinglsn : super::CLFS_LSN, txffileid : PTXF_ID, txflogcontext : *mut super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "shared", feature = "winnt"))]
windows_link::link!("txfw32.dll" "system" fn TxfLogCreateRangeReadContext(logpath : windows_sys::core::PCWSTR, beginninglsn : super::CLFS_LSN, endinglsn : super::CLFS_LSN, beginningvirtualclock : super::PLARGE_INTEGER, endingvirtualclock : super::PLARGE_INTEGER, recordtypemask : super::ULONG, txflogcontext : *mut super::PVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("txfw32.dll" "system" fn TxfLogDestroyReadContext(txflogcontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("txfw32.dll" "system" fn TxfLogReadRecords(txflogcontext : super::PVOID, bufferlength : super::ULONG, buffer : super::PVOID, bytesused : super::PULONG, recordcount : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("txfw32.dll" "system" fn TxfLogRecordGetFileName(recordbuffer : super::PVOID, recordbufferlengthinbytes : super::ULONG, namebuffer : windows_sys::core::PWSTR, namebufferlengthinbytes : super::PULONG, txfid : PTXF_ID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("txfw32.dll" "system" fn TxfLogRecordGetGenericType(recordbuffer : super::PVOID, recordbufferlengthinbytes : super::ULONG, generictype : super::PULONG, virtualclock : super::PLARGE_INTEGER) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("txfw32.dll" "system" fn TxfReadMetadataInfo(filehandle : super::HANDLE, txffileid : PTXF_ID, lastlsn : super::PCLFS_LSN, transactionstate : super::PULONG, lockingtransaction : PGUID) -> windows_sys::core::BOOL);
#[cfg(feature = "shared")]
windows_link::link!("txfw32.dll" "system" fn TxfSetThreadMiniVersionForCreate(miniversion : super::USHORT));
pub type PGUID = *mut windows_sys::core::GUID;
#[cfg(feature = "shared")]
pub type PTXF_ID = *mut TXF_ID;
#[cfg(feature = "shared")]
pub type PTXF_LOG_RECORD_AFFECTED_FILE = *mut TXF_LOG_RECORD_AFFECTED_FILE;
#[cfg(feature = "shared")]
pub type PTXF_LOG_RECORD_BASE = *mut TXF_LOG_RECORD_BASE;
#[cfg(feature = "shared")]
pub type PTXF_LOG_RECORD_TRUNCATE = *mut TXF_LOG_RECORD_TRUNCATE;
#[cfg(feature = "shared")]
pub type PTXF_LOG_RECORD_WRITE = *mut TXF_LOG_RECORD_WRITE;
pub const TXFS_MINIVERSION_COMMITTED_VIEW: i32 = 0;
pub const TXFS_MINIVERSION_DEFAULT_VIEW: i32 = 65534;
pub const TXFS_MINIVERSION_DIRTY_VIEW: i32 = 65535;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TXF_ID {
    pub Anonymous: TXF_ID_0,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TXF_ID_0 {
    pub LowPart: super::LONGLONG,
    pub HighPart: super::LONGLONG,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TXF_LOG_RECORD_AFFECTED_FILE {
    pub Version: super::USHORT,
    pub RecordLength: super::ULONG,
    pub Flags: super::ULONG,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: windows_sys::core::GUID,
    pub FileNameLength: super::ULONG,
    pub FileNameByteOffsetInStructure: super::ULONG,
}
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TXF_LOG_RECORD_BASE {
    pub Version: super::USHORT,
    pub RecordType: super::USHORT,
    pub RecordLength: super::ULONG,
}
pub const TXF_LOG_RECORD_GENERIC_TYPE_ABORT: i32 = 2;
pub const TXF_LOG_RECORD_GENERIC_TYPE_COMMIT: i32 = 1;
pub const TXF_LOG_RECORD_GENERIC_TYPE_DATA: i32 = 8;
pub const TXF_LOG_RECORD_GENERIC_TYPE_PREPARE: i32 = 4;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TXF_LOG_RECORD_TRUNCATE {
    pub Version: super::USHORT,
    pub RecordType: super::USHORT,
    pub RecordLength: super::ULONG,
    pub Flags: super::ULONG,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: windows_sys::core::GUID,
    pub NewFileSize: super::LONGLONG,
    pub FileNameLength: super::ULONG,
    pub FileNameByteOffsetInStructure: super::ULONG,
}
pub const TXF_LOG_RECORD_TYPE_AFFECTED_FILE: i32 = 4;
pub const TXF_LOG_RECORD_TYPE_TRUNCATE: i32 = 2;
pub const TXF_LOG_RECORD_TYPE_WRITE: i32 = 1;
#[repr(C, packed(4))]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct TXF_LOG_RECORD_WRITE {
    pub Version: super::USHORT,
    pub RecordType: super::USHORT,
    pub RecordLength: super::ULONG,
    pub Flags: super::ULONG,
    pub TxfFileId: TXF_ID,
    pub KtmGuid: windows_sys::core::GUID,
    pub ByteOffsetInFile: super::LONGLONG,
    pub NumBytesWritten: super::ULONG,
    pub ByteOffsetInStructure: super::ULONG,
    pub FileNameLength: super::ULONG,
    pub FileNameByteOffsetInStructure: super::ULONG,
}
