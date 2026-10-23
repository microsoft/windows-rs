#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn TxfGetThreadMiniVersionForCreate() -> u16 {
    windows_core::link!("txfw32.dll" "system" fn TxfGetThreadMiniVersionForCreate(miniversion : super::PUSHORT));
    unsafe {
        let mut result__ = core::mem::zeroed();
        TxfGetThreadMiniVersionForCreate(&mut result__);
        result__
    }
}
#[cfg(all(feature = "clfs", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TxfLogCreateFileReadContext<P0>(logpath: P0, beginninglsn: super::CLFS_LSN, endinglsn: super::CLFS_LSN, txffileid: PTXF_ID, txflogcontext: *mut super::PVOID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("txfw32.dll" "system" fn TxfLogCreateFileReadContext(logpath : windows_core::PCWSTR, beginninglsn : super::CLFS_LSN, endinglsn : super::CLFS_LSN, txffileid : PTXF_ID, txflogcontext : *mut super::PVOID) -> windows_core::BOOL);
    unsafe { TxfLogCreateFileReadContext(logpath.param().abi(), beginninglsn, endinglsn, txffileid, txflogcontext as _) }
}
#[cfg(all(feature = "clfs", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TxfLogCreateRangeReadContext<P0>(logpath: P0, beginninglsn: super::CLFS_LSN, endinglsn: super::CLFS_LSN, beginningvirtualclock: super::PLARGE_INTEGER, endingvirtualclock: super::PLARGE_INTEGER, recordtypemask: super::ULONG, txflogcontext: *mut super::PVOID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("txfw32.dll" "system" fn TxfLogCreateRangeReadContext(logpath : windows_core::PCWSTR, beginninglsn : super::CLFS_LSN, endinglsn : super::CLFS_LSN, beginningvirtualclock : super::PLARGE_INTEGER, endingvirtualclock : super::PLARGE_INTEGER, recordtypemask : super::ULONG, txflogcontext : *mut super::PVOID) -> windows_core::BOOL);
    unsafe { TxfLogCreateRangeReadContext(logpath.param().abi(), beginninglsn, endinglsn, beginningvirtualclock, endingvirtualclock, recordtypemask, txflogcontext as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TxfLogDestroyReadContext(txflogcontext: super::PVOID) -> windows_core::BOOL {
    windows_core::link!("txfw32.dll" "system" fn TxfLogDestroyReadContext(txflogcontext : super::PVOID) -> windows_core::BOOL);
    unsafe { TxfLogDestroyReadContext(txflogcontext) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TxfLogReadRecords(txflogcontext: super::PVOID, bufferlength: super::ULONG, buffer: super::PVOID, bytesused: super::PULONG, recordcount: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("txfw32.dll" "system" fn TxfLogReadRecords(txflogcontext : super::PVOID, bufferlength : super::ULONG, buffer : super::PVOID, bytesused : super::PULONG, recordcount : super::PULONG) -> windows_core::BOOL);
    unsafe { TxfLogReadRecords(txflogcontext, bufferlength, buffer as _, bytesused as _, recordcount as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TxfLogRecordGetFileName(recordbuffer: super::PVOID, recordbufferlengthinbytes: super::ULONG, namebuffer: windows_core::PWSTR, namebufferlengthinbytes: super::PULONG, txfid: Option<PTXF_ID>) -> windows_core::BOOL {
    windows_core::link!("txfw32.dll" "system" fn TxfLogRecordGetFileName(recordbuffer : super::PVOID, recordbufferlengthinbytes : super::ULONG, namebuffer : windows_core::PWSTR, namebufferlengthinbytes : super::PULONG, txfid : PTXF_ID) -> windows_core::BOOL);
    unsafe { TxfLogRecordGetFileName(recordbuffer, recordbufferlengthinbytes, namebuffer, namebufferlengthinbytes as _, txfid.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TxfLogRecordGetGenericType(recordbuffer: super::PVOID, recordbufferlengthinbytes: super::ULONG, generictype: super::PULONG, virtualclock: Option<super::PLARGE_INTEGER>) -> windows_core::BOOL {
    windows_core::link!("txfw32.dll" "system" fn TxfLogRecordGetGenericType(recordbuffer : super::PVOID, recordbufferlengthinbytes : super::ULONG, generictype : super::PULONG, virtualclock : super::PLARGE_INTEGER) -> windows_core::BOOL);
    unsafe { TxfLogRecordGetGenericType(recordbuffer, recordbufferlengthinbytes, generictype as _, virtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TxfReadMetadataInfo(filehandle: super::HANDLE, txffileid: PTXF_ID, lastlsn: super::PCLFS_LSN, transactionstate: super::PULONG, lockingtransaction: PGUID) -> windows_core::BOOL {
    windows_core::link!("txfw32.dll" "system" fn TxfReadMetadataInfo(filehandle : super::HANDLE, txffileid : PTXF_ID, lastlsn : super::PCLFS_LSN, transactionstate : super::PULONG, lockingtransaction : PGUID) -> windows_core::BOOL);
    unsafe { TxfReadMetadataInfo(filehandle, txffileid as _, lastlsn as _, transactionstate as _, lockingtransaction as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn TxfSetThreadMiniVersionForCreate(miniversion: super::USHORT) {
    windows_core::link!("txfw32.dll" "system" fn TxfSetThreadMiniVersionForCreate(miniversion : super::USHORT));
    unsafe { TxfSetThreadMiniVersionForCreate(miniversion) }
}
pub type PGUID = *mut windows_core::GUID;
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
    pub KtmGuid: windows_core::GUID,
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
    pub KtmGuid: windows_core::GUID,
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
    pub KtmGuid: windows_core::GUID,
    pub ByteOffsetInFile: super::LONGLONG,
    pub NumBytesWritten: super::ULONG,
    pub ByteOffsetInStructure: super::ULONG,
    pub FileNameLength: super::ULONG,
    pub FileNameByteOffsetInStructure: super::ULONG,
}
