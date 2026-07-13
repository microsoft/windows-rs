#[inline]
pub unsafe fn TxfGetThreadMiniVersionForCreate() -> u16 {
    windows_core::link!("txfw32.dll" "system" fn TxfGetThreadMiniVersionForCreate(miniversion : *mut u16));
    unsafe {
        let mut result__ = core::mem::zeroed();
        TxfGetThreadMiniVersionForCreate(&mut result__);
        result__
    }
}
#[cfg(feature = "clfs")]
#[inline]
pub unsafe fn TxfLogCreateFileReadContext<P0>(logpath: P0, beginninglsn: super::clfs::CLFS_LSN, endinglsn: super::clfs::CLFS_LSN, txffileid: *const TXF_ID, txflogcontext: *mut *mut core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("txfw32.dll" "system" fn TxfLogCreateFileReadContext(logpath : windows_core::PCWSTR, beginninglsn : super::clfs::CLFS_LSN, endinglsn : super::clfs::CLFS_LSN, txffileid : *const TXF_ID, txflogcontext : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { TxfLogCreateFileReadContext(logpath.param().abi(), core::mem::transmute(beginninglsn), core::mem::transmute(endinglsn), txffileid, txflogcontext as _) }
}
#[cfg(feature = "clfs")]
#[inline]
pub unsafe fn TxfLogCreateRangeReadContext<P0>(logpath: P0, beginninglsn: super::clfs::CLFS_LSN, endinglsn: super::clfs::CLFS_LSN, beginningvirtualclock: *const i64, endingvirtualclock: *const i64, recordtypemask: u32, txflogcontext: *mut *mut core::ffi::c_void) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("txfw32.dll" "system" fn TxfLogCreateRangeReadContext(logpath : windows_core::PCWSTR, beginninglsn : super::clfs::CLFS_LSN, endinglsn : super::clfs::CLFS_LSN, beginningvirtualclock : *const i64, endingvirtualclock : *const i64, recordtypemask : u32, txflogcontext : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { TxfLogCreateRangeReadContext(logpath.param().abi(), core::mem::transmute(beginninglsn), core::mem::transmute(endinglsn), beginningvirtualclock, endingvirtualclock, recordtypemask, txflogcontext as _) }
}
#[inline]
pub unsafe fn TxfLogDestroyReadContext(txflogcontext: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("txfw32.dll" "system" fn TxfLogDestroyReadContext(txflogcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { TxfLogDestroyReadContext(txflogcontext) }
}
#[inline]
pub unsafe fn TxfLogReadRecords(txflogcontext: *const core::ffi::c_void, bufferlength: u32, buffer: *mut core::ffi::c_void, bytesused: *mut u32, recordcount: *mut u32) -> windows_core::BOOL {
    windows_core::link!("txfw32.dll" "system" fn TxfLogReadRecords(txflogcontext : *const core::ffi::c_void, bufferlength : u32, buffer : *mut core::ffi::c_void, bytesused : *mut u32, recordcount : *mut u32) -> windows_core::BOOL);
    unsafe { TxfLogReadRecords(txflogcontext, bufferlength, buffer as _, bytesused as _, recordcount as _) }
}
#[inline]
pub unsafe fn TxfLogRecordGetFileName(recordbuffer: *const core::ffi::c_void, recordbufferlengthinbytes: u32, namebuffer: windows_core::PWSTR, namebufferlengthinbytes: *mut u32, txfid: Option<*mut TXF_ID>) -> windows_core::BOOL {
    windows_core::link!("txfw32.dll" "system" fn TxfLogRecordGetFileName(recordbuffer : *const core::ffi::c_void, recordbufferlengthinbytes : u32, namebuffer : windows_core::PWSTR, namebufferlengthinbytes : *mut u32, txfid : *mut TXF_ID) -> windows_core::BOOL);
    unsafe { TxfLogRecordGetFileName(recordbuffer, recordbufferlengthinbytes, core::mem::transmute(namebuffer), namebufferlengthinbytes as _, txfid.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TxfLogRecordGetGenericType(recordbuffer: *const core::ffi::c_void, recordbufferlengthinbytes: u32, generictype: *mut u32, virtualclock: Option<*mut i64>) -> windows_core::BOOL {
    windows_core::link!("txfw32.dll" "system" fn TxfLogRecordGetGenericType(recordbuffer : *const core::ffi::c_void, recordbufferlengthinbytes : u32, generictype : *mut u32, virtualclock : *mut i64) -> windows_core::BOOL);
    unsafe { TxfLogRecordGetGenericType(recordbuffer, recordbufferlengthinbytes, generictype as _, virtualclock.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "winnt"))]
#[inline]
pub unsafe fn TxfReadMetadataInfo(filehandle: super::winnt::HANDLE, txffileid: *mut TXF_ID, lastlsn: *mut super::clfs::CLFS_LSN, transactionstate: *mut u32, lockingtransaction: *mut windows_core::GUID) -> windows_core::BOOL {
    windows_core::link!("txfw32.dll" "system" fn TxfReadMetadataInfo(filehandle : super::winnt::HANDLE, txffileid : *mut TXF_ID, lastlsn : *mut super::clfs::CLFS_LSN, transactionstate : *mut u32, lockingtransaction : *mut windows_core::GUID) -> windows_core::BOOL);
    unsafe { TxfReadMetadataInfo(filehandle, txffileid as _, lastlsn as _, transactionstate as _, lockingtransaction as _) }
}
#[inline]
pub unsafe fn TxfSetThreadMiniVersionForCreate(miniversion: u16) {
    windows_core::link!("txfw32.dll" "system" fn TxfSetThreadMiniVersionForCreate(miniversion : u16));
    unsafe { TxfSetThreadMiniVersionForCreate(miniversion) }
}
pub type PGUID = *mut windows_core::GUID;
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
    pub KtmGuid: windows_core::GUID,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
    pub KtmGuid: windows_core::GUID,
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
    pub KtmGuid: windows_core::GUID,
    pub ByteOffsetInFile: i64,
    pub NumBytesWritten: u32,
    pub ByteOffsetInStructure: u32,
    pub FileNameLength: u32,
    pub FileNameByteOffsetInStructure: u32,
}
