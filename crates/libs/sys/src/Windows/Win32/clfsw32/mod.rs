#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn AddLogContainer(hlog : super::HANDLE, pcbcontainer : *const u64, pwszcontainerpath : windows_sys::core::PCWSTR, preserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn AddLogContainerSet(hlog : super::HANDLE, ccontainer : u16, pcbcontainer : *const u64, rgwszcontainerpath : *const windows_sys::core::PCWSTR, preserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn AdvanceLogBase(pvmarshal : *mut core::ffi::c_void, plsnbase : *mut super::CLFS_LSN, fflags : u32, poverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
windows_link::link!("clfsw32.dll" "system" fn AlignReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, rgcbreservation : *mut i64, pcbalignreservation : *mut i64) -> windows_sys::core::BOOL);
windows_link::link!("clfsw32.dll" "system" fn AllocReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, pcbadjustment : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn CloseAndResetLogFile(hlog : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn CreateLogContainerScanContext(hlog : super::HANDLE, cfromcontainer : u32, ccontainers : u32, escanmode : super::CLFS_SCAN_MODE, pcxscan : *mut super::CLFS_SCAN_CONTEXT, poverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn CreateLogFile(pszlogfilename : windows_sys::core::PCWSTR, fdesiredaccess : super::ACCESS_MASK, dwsharemode : u32, psalogfile : *mut super::SECURITY_ATTRIBUTES, fcreatedisposition : u32, fflagsandattributes : u32) -> super::HANDLE);
#[cfg(all(feature = "clfs", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn CreateLogMarshallingArea(hlog : super::HANDLE, pfnallocbuffer : super::CLFS_BLOCK_ALLOCATION, pfnfreebuffer : super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext : *mut core::ffi::c_void, cbmarshallingbuffer : u32, cmaxwritebuffers : u32, cmaxreadbuffers : u32, ppvmarshal : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn DeleteLogByHandle(hlog : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("clfsw32.dll" "system" fn DeleteLogFile(pszlogfilename : windows_sys::core::PCWSTR, pvreserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("clfsw32.dll" "system" fn DeleteLogMarshallingArea(pvmarshal : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "corecrt_wstdio"))]
windows_link::link!("clfsw32.dll" "system" fn DumpLogRecords(pwszlogfilename : windows_sys::core::PCWSTR, frecordtype : super::CLFS_RECORD_TYPE, plsnstart : *const super::CLFS_LSN, plsnend : *const super::CLFS_LSN, pstrmout : *const super::FILE, pfnprintrecord : CLFS_PRINT_RECORD_ROUTINE, pfnallocblock : super::CLFS_BLOCK_ALLOCATION, pfnfreeblock : super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext : *const core::ffi::c_void, cbblock : u32, cmaxblocks : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn FlushLogBuffers(pvmarshal : *mut core::ffi::c_void, poverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn FlushLogToLsn(pvmarshalcontext : *mut core::ffi::c_void, plsnflush : *mut super::CLFS_LSN, plsnlastflushed : *mut super::CLFS_LSN, poverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
windows_link::link!("clfsw32.dll" "system" fn FreeReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, pcbadjustment : *mut i64) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn GetLogContainerName(hlog : super::HANDLE, cidlogicalcontainer : super::CLFS_CONTAINER_ID, pwstrcontainername : windows_sys::core::PCWSTR, clencontainername : u32, pcactuallencontainername : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn GetLogFileInformation(hlog : super::HANDLE, pinfobuffer : *mut super::CLFS_INFORMATION, cbbuffer : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn GetLogIoStatistics(hlog : super::HANDLE, pvstatsbuffer : *mut core::ffi::c_void, cbstatsbuffer : u32, estatsclass : super::CLFS_IOSTATS_CLASS, pcbstatswritten : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("clfsw32.dll" "system" fn GetLogReservationInfo(pvmarshal : *const core::ffi::c_void, pcbrecordnumber : *mut u32, pcbuserreservation : *mut i64, pcbcommitreservation : *mut i64) -> windows_sys::core::BOOL);
#[cfg(feature = "clfs")]
windows_link::link!("clfsw32.dll" "system" fn GetNextLogArchiveExtent(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT, rgadextent : *mut super::CLFS_ARCHIVE_DESCRIPTOR, cdescriptors : u32, pcdescriptorsreturned : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn PrepareLogArchive(hlog : super::HANDLE, pszbaselogfilename : windows_sys::core::PWSTR, clen : u32, plsnlow : *const super::CLFS_LSN, plsnhigh : *const super::CLFS_LSN, pcactuallength : *mut u32, poffbaselogfiledata : *mut u64, pcbbaselogfilelength : *mut u64, plsnbase : *mut super::CLFS_LSN, plsnlast : *mut super::CLFS_LSN, plsncurrentarchivetail : *mut super::CLFS_LSN, ppvarchivecontext : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("clfsw32.dll" "system" fn ReadLogArchiveMetadata(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT, cboffset : u32, cbbytestoread : u32, pbreadbuffer : *mut u8, pcbbytesread : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReadLogRecord(pvmarshal : *mut core::ffi::c_void, plsnfirst : *mut super::CLFS_LSN, econtextmode : super::CLFS_CONTEXT_MODE, ppvreadbuffer : *mut *mut core::ffi::c_void, pcbreadbuffer : *mut u32, perecordtype : *mut super::CLS_RECORD_TYPE, plsnundonext : *mut super::CLFS_LSN, plsnprevious : *mut super::CLFS_LSN, ppvreadcontext : *mut *mut core::ffi::c_void, poverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReadLogRestartArea(pvmarshal : *mut core::ffi::c_void, ppvrestartbuffer : *mut *mut core::ffi::c_void, pcbrestartbuffer : *mut u32, plsn : *mut super::CLFS_LSN, ppvcontext : *mut *mut core::ffi::c_void, poverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReadNextLogRecord(pvreadcontext : *mut core::ffi::c_void, ppvbuffer : *mut *mut core::ffi::c_void, pcbbuffer : *mut u32, perecordtype : *mut super::CLS_RECORD_TYPE, plsnuser : *mut super::CLFS_LSN, plsnundonext : *mut super::CLFS_LSN, plsnprevious : *mut super::CLFS_LSN, plsnrecord : *mut super::CLFS_LSN, poverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReadPreviousLogRestartArea(pvreadcontext : *mut core::ffi::c_void, ppvrestartbuffer : *mut *mut core::ffi::c_void, pcbrestartbuffer : *mut u32, plsnrestart : *mut super::CLFS_LSN, poverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn RemoveLogContainer(hlog : super::HANDLE, pwszcontainerpath : windows_sys::core::PCWSTR, fforce : windows_sys::core::BOOL, preserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn RemoveLogContainerSet(hlog : super::HANDLE, ccontainer : u16, rgwszcontainerpath : *const windows_sys::core::PCWSTR, fforce : windows_sys::core::BOOL, preserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReserveAndAppendLog(pvmarshal : *mut core::ffi::c_void, rgwriteentries : *mut super::CLFS_WRITE_ENTRY, cwriteentries : u32, plsnundonext : *mut super::CLFS_LSN, plsnprevious : *mut super::CLFS_LSN, creserverecords : u32, rgcbreservation : *mut i64, fflags : u32, plsn : *mut super::CLFS_LSN, poverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReserveAndAppendLogAligned(pvmarshal : *mut core::ffi::c_void, rgwriteentries : *mut super::CLFS_WRITE_ENTRY, cwriteentries : u32, cbentryalignment : u32, plsnundonext : *mut super::CLFS_LSN, plsnprevious : *mut super::CLFS_LSN, creserverecords : u32, rgcbreservation : *mut i64, fflags : u32, plsn : *mut super::CLFS_LSN, poverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ScanLogContainers(pcxscan : *mut super::CLFS_SCAN_CONTEXT, escanmode : super::CLFS_SCAN_MODE, preserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn SetEndOfLog(hlog : super::HANDLE, plsnend : *mut super::CLFS_LSN, lpoverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn SetLogArchiveMode(hlog : super::HANDLE, emode : super::CLFS_LOG_ARCHIVE_MODE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn SetLogArchiveTail(hlog : super::HANDLE, plsnarchivetail : *mut super::CLFS_LSN, preserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("clfsw32.dll" "system" fn TerminateLogArchive(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("clfsw32.dll" "system" fn TerminateReadLog(pvcursorcontext : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn TruncateLog(pvmarshal : *const core::ffi::c_void, plsnend : *const super::CLFS_LSN, lpoverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase"))]
windows_link::link!("clfsw32.dll" "system" fn ValidateLog(pszlogfilename : windows_sys::core::PCWSTR, psalogfile : *mut super::SECURITY_ATTRIBUTES, pinfobuffer : *mut super::CLFS_INFORMATION, pcbbuffer : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn WriteLogRestartArea(pvmarshal : *mut core::ffi::c_void, pvrestartbuffer : *mut core::ffi::c_void, cbrestartbuffer : u32, plsnbase : *mut super::CLFS_LSN, fflags : u32, pcbwritten : *mut u32, plsnnext : *mut super::CLFS_LSN, poverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
pub type CLFS_LOG_ARCHIVE_CONTEXT = *mut core::ffi::c_void;
#[cfg(all(feature = "clfs", feature = "corecrt_wstdio"))]
pub type CLFS_PRINT_RECORD_ROUTINE = Option<unsafe extern "system" fn(pstrmout: *mut super::FILE, frecordtype: super::CLFS_RECORD_TYPE, pvbuffer: *mut core::ffi::c_void, cbbuffer: u32) -> u32>;
pub type PCLFS_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(pvoverlapped: *mut core::ffi::c_void, ulreserved: u32)>;
pub type PCLFS_LOG_ARCHIVE_CONTEXT = *mut *mut core::ffi::c_void;
