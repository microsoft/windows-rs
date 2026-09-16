#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn AddLogContainer(hlog : super::HANDLE, pcbcontainer : super::PULONGLONG, pwszcontainerpath : windows_sys::core::PCWSTR, preserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn AddLogContainerSet(hlog : super::HANDLE, ccontainer : u16, pcbcontainer : super::PULONGLONG, rgwszcontainerpath : *const windows_sys::core::PCWSTR, preserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn AdvanceLogBase(pvmarshal : *mut core::ffi::c_void, plsnbase : super::PCLFS_LSN, fflags : u32, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn AlignReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, rgcbreservation : *const i64, pcbalignreservation : super::PLONGLONG) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn AllocReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, pcbadjustment : super::PLONGLONG) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn CloseAndResetLogFile(hlog : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn CreateLogContainerScanContext(hlog : super::HANDLE, cfromcontainer : u32, ccontainers : u32, escanmode : super::CLFS_SCAN_MODE, pcxscan : super::PCLFS_SCAN_CONTEXT, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn CreateLogFile(pszlogfilename : windows_sys::core::PCWSTR, fdesiredaccess : super::ACCESS_MASK, dwsharemode : u32, psalogfile : super::LPSECURITY_ATTRIBUTES, fcreatedisposition : u32, fflagsandattributes : u32) -> super::HANDLE);
#[cfg(all(feature = "clfs", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn CreateLogMarshallingArea(hlog : super::HANDLE, pfnallocbuffer : super::CLFS_BLOCK_ALLOCATION, pfnfreebuffer : super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext : *const core::ffi::c_void, cbmarshallingbuffer : u32, cmaxwritebuffers : u32, cmaxreadbuffers : u32, ppvmarshal : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn DeleteLogByHandle(hlog : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("clfsw32.dll" "system" fn DeleteLogFile(pszlogfilename : windows_sys::core::PCWSTR, pvreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("clfsw32.dll" "system" fn DeleteLogMarshallingArea(pvmarshal : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "corecrt_wstdio"))]
windows_link::link!("clfsw32.dll" "system" fn DumpLogRecords(pwszlogfilename : windows_sys::core::PCWSTR, frecordtype : super::CLFS_RECORD_TYPE, plsnstart : super::PCLFS_LSN, plsnend : super::PCLFS_LSN, pstrmout : super::PFILE, pfnprintrecord : CLFS_PRINT_RECORD_ROUTINE, pfnallocblock : super::CLFS_BLOCK_ALLOCATION, pfnfreeblock : super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext : *const core::ffi::c_void, cbblock : u32, cmaxblocks : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn FlushLogBuffers(pvmarshal : *const core::ffi::c_void, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn FlushLogToLsn(pvmarshalcontext : *const core::ffi::c_void, plsnflush : super::PCLFS_LSN, plsnlastflushed : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn FreeReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, pcbadjustment : super::PLONGLONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn GetLogContainerName(hlog : super::HANDLE, cidlogicalcontainer : super::CLFS_CONTAINER_ID, pwstrcontainername : windows_sys::core::PCWSTR, clencontainername : u32, pcactuallencontainername : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn GetLogFileInformation(hlog : super::HANDLE, pinfobuffer : super::PCLFS_INFORMATION, cbbuffer : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn GetLogIoStatistics(hlog : super::HANDLE, pvstatsbuffer : *mut core::ffi::c_void, cbstatsbuffer : u32, estatsclass : super::CLFS_IOSTATS_CLASS, pcbstatswritten : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn GetLogReservationInfo(pvmarshal : *const core::ffi::c_void, pcbrecordnumber : super::PULONG, pcbuserreservation : super::PLONGLONG, pcbcommitreservation : super::PLONGLONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwindef"))]
windows_link::link!("clfsw32.dll" "system" fn GetNextLogArchiveExtent(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT, rgadextent : *mut super::CLFS_ARCHIVE_DESCRIPTOR, cdescriptors : u32, pcdescriptorsreturned : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(feature = "clfs")]
windows_link::link!("clfsw32.dll" "system" fn LsnBlockOffset(plsn : *const super::CLFS_LSN) -> u32);
#[cfg(feature = "clfs")]
windows_link::link!("clfsw32.dll" "system" fn LsnContainer(plsn : *const super::CLFS_LSN) -> super::CLFS_CONTAINER_ID);
#[cfg(feature = "clfs")]
windows_link::link!("clfsw32.dll" "system" fn LsnCreate(cidcontainer : super::CLFS_CONTAINER_ID, offblock : u32, crecord : u32) -> super::CLFS_LSN);
#[cfg(feature = "clfs")]
windows_link::link!("clfsw32.dll" "system" fn LsnRecordSequence(plsn : *const super::CLFS_LSN) -> u32);
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn PrepareLogArchive(hlog : super::HANDLE, pszbaselogfilename : windows_sys::core::PWSTR, clen : u32, plsnlow : super::PCLFS_LSN, plsnhigh : super::PCLFS_LSN, pcactuallength : super::PULONG, poffbaselogfiledata : super::PULONGLONG, pcbbaselogfilelength : super::PULONGLONG, plsnbase : super::PCLFS_LSN, plsnlast : super::PCLFS_LSN, plsncurrentarchivetail : super::PCLFS_LSN, ppvarchivecontext : PCLFS_LOG_ARCHIVE_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("clfsw32.dll" "system" fn ReadLogArchiveMetadata(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT, cboffset : u32, cbbytestoread : u32, pbreadbuffer : super::PBYTE, pcbbytesread : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReadLogRecord(pvmarshal : *const core::ffi::c_void, plsnfirst : super::PCLFS_LSN, econtextmode : super::CLFS_CONTEXT_MODE, ppvreadbuffer : *mut *mut core::ffi::c_void, pcbreadbuffer : super::PULONG, perecordtype : super::PCLFS_RECORD_TYPE, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, ppvreadcontext : *mut *mut core::ffi::c_void, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReadLogRestartArea(pvmarshal : *const core::ffi::c_void, ppvrestartbuffer : *mut *mut core::ffi::c_void, pcbrestartbuffer : super::PULONG, plsn : super::PCLFS_LSN, ppvcontext : *mut *mut core::ffi::c_void, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReadNextLogRecord(pvreadcontext : *mut core::ffi::c_void, ppvbuffer : *mut *mut core::ffi::c_void, pcbbuffer : super::PULONG, perecordtype : super::PCLFS_RECORD_TYPE, plsnuser : super::PCLFS_LSN, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, plsnrecord : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReadPreviousLogRestartArea(pvreadcontext : *const core::ffi::c_void, ppvrestartbuffer : *mut *mut core::ffi::c_void, pcbrestartbuffer : super::PULONG, plsnrestart : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn RemoveLogContainer(hlog : super::HANDLE, pwszcontainerpath : windows_sys::core::PCWSTR, fforce : windows_sys::core::BOOL, preserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn RemoveLogContainerSet(hlog : super::HANDLE, ccontainer : u16, rgwszcontainerpath : *const windows_sys::core::PCWSTR, fforce : windows_sys::core::BOOL, preserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReserveAndAppendLog(pvmarshal : *const core::ffi::c_void, rgwriteentries : super::PCLFS_WRITE_ENTRY, cwriteentries : u32, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, creserverecords : u32, rgcbreservation : *mut i64, fflags : u32, plsn : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReserveAndAppendLogAligned(pvmarshal : *const core::ffi::c_void, rgwriteentries : super::PCLFS_WRITE_ENTRY, cwriteentries : u32, cbentryalignment : u32, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, creserverecords : u32, rgcbreservation : *mut i64, fflags : u32, plsn : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ScanLogContainers(pcxscan : super::PCLFS_SCAN_CONTEXT, escanmode : super::CLFS_SCAN_MODE, preserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn SetEndOfLog(hlog : super::HANDLE, plsnend : super::PCLFS_LSN, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn SetLogArchiveMode(hlog : super::HANDLE, emode : super::CLFS_LOG_ARCHIVE_MODE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn SetLogArchiveTail(hlog : super::HANDLE, plsnarchivetail : super::PCLFS_LSN, preserved : *mut core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("clfsw32.dll" "system" fn TerminateLogArchive(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("clfsw32.dll" "system" fn TerminateReadLog(pvcursorcontext : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn TruncateLog(pvmarshal : *const core::ffi::c_void, plsnend : super::PCLFS_LSN, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef"))]
windows_link::link!("clfsw32.dll" "system" fn ValidateLog(pszlogfilename : windows_sys::core::PCWSTR, psalogfile : super::LPSECURITY_ATTRIBUTES, pinfobuffer : super::PCLFS_INFORMATION, pcbbuffer : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn WriteLogRestartArea(pvmarshal : *mut core::ffi::c_void, pvrestartbuffer : *const core::ffi::c_void, cbrestartbuffer : u32, plsnbase : super::PCLFS_LSN, fflags : u32, pcbwritten : super::PULONG, plsnnext : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
pub type CLFS_LOG_ARCHIVE_CONTEXT = *mut core::ffi::c_void;
#[cfg(all(feature = "clfs", feature = "corecrt_wstdio"))]
pub type CLFS_PRINT_RECORD_ROUTINE = Option<unsafe extern "system" fn(pstrmout: super::PFILE, frecordtype: super::CLFS_RECORD_TYPE, pvbuffer: *mut core::ffi::c_void, cbbuffer: u32) -> u32>;
pub type PCLFS_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(pvoverlapped: *mut core::ffi::c_void, ulreserved: u32)>;
pub type PCLFS_LOG_ARCHIVE_CONTEXT = *mut *mut core::ffi::c_void;
