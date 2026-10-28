#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn AddLogContainer(hlog : super::HANDLE, pcbcontainer : super::PULONGLONG, pwszcontainerpath : windows_sys::core::PCWSTR, preserved : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn AddLogContainerSet(hlog : super::HANDLE, ccontainer : super::USHORT, pcbcontainer : super::PULONGLONG, rgwszcontainerpath : *const windows_sys::core::PCWSTR, preserved : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn AdvanceLogBase(pvmarshal : super::PVOID, plsnbase : super::PCLFS_LSN, fflags : super::ULONG, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn AlignReservedLog(pvmarshal : super::PVOID, creservedrecords : super::ULONG, rgcbreservation : *mut super::LONGLONG, pcbalignreservation : super::PLONGLONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn AllocReservedLog(pvmarshal : super::PVOID, creservedrecords : super::ULONG, pcbadjustment : super::PLONGLONG) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn CloseAndResetLogFile(hlog : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn CreateLogContainerScanContext(hlog : super::HANDLE, cfromcontainer : super::ULONG, ccontainers : super::ULONG, escanmode : super::CLFS_SCAN_MODE, pcxscan : super::PCLFS_SCAN_CONTEXT, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn CreateLogFile(pszlogfilename : windows_sys::core::PCWSTR, fdesiredaccess : super::ACCESS_MASK, dwsharemode : super::DWORD, psalogfile : super::LPSECURITY_ATTRIBUTES, fcreatedisposition : super::ULONG, fflagsandattributes : super::ULONG) -> super::HANDLE);
#[cfg(all(feature = "clfs", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn CreateLogMarshallingArea(hlog : super::HANDLE, pfnallocbuffer : super::CLFS_BLOCK_ALLOCATION, pfnfreebuffer : super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext : super::PVOID, cbmarshallingbuffer : super::ULONG, cmaxwritebuffers : super::ULONG, cmaxreadbuffers : super::ULONG, ppvmarshal : *mut super::PVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn DeleteLogByHandle(hlog : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn DeleteLogFile(pszlogfilename : windows_sys::core::PCWSTR, pvreserved : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn DeleteLogMarshallingArea(pvmarshal : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "corecrt_wstdio", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn DumpLogRecords(pwszlogfilename : windows_sys::core::PCWSTR, frecordtype : super::CLFS_RECORD_TYPE, plsnstart : super::PCLFS_LSN, plsnend : super::PCLFS_LSN, pstrmout : super::PFILE, pfnprintrecord : CLFS_PRINT_RECORD_ROUTINE, pfnallocblock : super::CLFS_BLOCK_ALLOCATION, pfnfreeblock : super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext : super::PVOID, cbblock : super::ULONG, cmaxblocks : super::ULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn FlushLogBuffers(pvmarshal : super::PVOID, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn FlushLogToLsn(pvmarshalcontext : super::PVOID, plsnflush : super::PCLFS_LSN, plsnlastflushed : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn FreeReservedLog(pvmarshal : super::PVOID, creservedrecords : super::ULONG, pcbadjustment : super::PLONGLONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn GetLogContainerName(hlog : super::HANDLE, cidlogicalcontainer : super::CLFS_CONTAINER_ID, pwstrcontainername : windows_sys::core::PCWSTR, clencontainername : super::ULONG, pcactuallencontainername : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn GetLogFileInformation(hlog : super::HANDLE, pinfobuffer : super::PCLFS_INFORMATION, cbbuffer : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn GetLogIoStatistics(hlog : super::HANDLE, pvstatsbuffer : super::PVOID, cbstatsbuffer : super::ULONG, estatsclass : super::CLFS_IOSTATS_CLASS, pcbstatswritten : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn GetLogReservationInfo(pvmarshal : super::PVOID, pcbrecordnumber : super::PULONG, pcbuserreservation : super::PLONGLONG, pcbcommitreservation : super::PLONGLONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn GetNextLogArchiveExtent(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT, rgadextent : *mut super::CLFS_ARCHIVE_DESCRIPTOR, cdescriptors : super::ULONG, pcdescriptorsreturned : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn PrepareLogArchive(hlog : super::HANDLE, pszbaselogfilename : windows_sys::core::PWSTR, clen : super::ULONG, plsnlow : super::PCLFS_LSN, plsnhigh : super::PCLFS_LSN, pcactuallength : super::PULONG, poffbaselogfiledata : super::PULONGLONG, pcbbaselogfilelength : super::PULONGLONG, plsnbase : super::PCLFS_LSN, plsnlast : super::PCLFS_LSN, plsncurrentarchivetail : super::PCLFS_LSN, ppvarchivecontext : PCLFS_LOG_ARCHIVE_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("clfsw32.dll" "system" fn ReadLogArchiveMetadata(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT, cboffset : super::ULONG, cbbytestoread : super::ULONG, pbreadbuffer : super::PBYTE, pcbbytesread : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReadLogRecord(pvmarshal : super::PVOID, plsnfirst : super::PCLFS_LSN, econtextmode : super::CLFS_CONTEXT_MODE, ppvreadbuffer : *mut super::PVOID, pcbreadbuffer : super::PULONG, perecordtype : super::PCLFS_RECORD_TYPE, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, ppvreadcontext : *mut super::PVOID, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReadLogRestartArea(pvmarshal : super::PVOID, ppvrestartbuffer : *mut super::PVOID, pcbrestartbuffer : super::PULONG, plsn : super::PCLFS_LSN, ppvcontext : *mut super::PVOID, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReadNextLogRecord(pvreadcontext : super::PVOID, ppvbuffer : *mut super::PVOID, pcbbuffer : super::PULONG, perecordtype : super::PCLFS_RECORD_TYPE, plsnuser : super::PCLFS_LSN, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, plsnrecord : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReadPreviousLogRestartArea(pvreadcontext : super::PVOID, ppvrestartbuffer : *mut super::PVOID, pcbrestartbuffer : super::PULONG, plsnrestart : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn RemoveLogContainer(hlog : super::HANDLE, pwszcontainerpath : windows_sys::core::PCWSTR, fforce : windows_sys::core::BOOL, preserved : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn RemoveLogContainerSet(hlog : super::HANDLE, ccontainer : super::USHORT, rgwszcontainerpath : *const windows_sys::core::PCWSTR, fforce : windows_sys::core::BOOL, preserved : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReserveAndAppendLog(pvmarshal : super::PVOID, rgwriteentries : super::PCLFS_WRITE_ENTRY, cwriteentries : super::ULONG, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, creserverecords : super::ULONG, rgcbreservation : *mut super::LONGLONG, fflags : super::ULONG, plsn : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ReserveAndAppendLogAligned(pvmarshal : super::PVOID, rgwriteentries : super::PCLFS_WRITE_ENTRY, cwriteentries : super::ULONG, cbentryalignment : super::ULONG, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, creserverecords : super::ULONG, rgcbreservation : *mut super::LONGLONG, fflags : super::ULONG, plsn : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn ScanLogContainers(pcxscan : super::PCLFS_SCAN_CONTEXT, escanmode : super::CLFS_SCAN_MODE, preserved : super::LPVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn SetEndOfLog(hlog : super::HANDLE, plsnend : super::PCLFS_LSN, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn SetLogArchiveMode(hlog : super::HANDLE, emode : super::CLFS_LOG_ARCHIVE_MODE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn SetLogArchiveTail(hlog : super::HANDLE, plsnarchivetail : super::PCLFS_LSN, preserved : super::LPVOID) -> windows_sys::core::BOOL);
windows_link::link!("clfsw32.dll" "system" fn TerminateLogArchive(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("clfsw32.dll" "system" fn TerminateReadLog(pvcursorcontext : super::PVOID) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn TruncateLog(pvmarshal : super::PVOID, plsnend : super::PCLFS_LSN, lpoverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "shared"))]
windows_link::link!("clfsw32.dll" "system" fn ValidateLog(pszlogfilename : windows_sys::core::PCWSTR, psalogfile : super::LPSECURITY_ATTRIBUTES, pinfobuffer : super::PCLFS_INFORMATION, pcbbuffer : super::PULONG) -> windows_sys::core::BOOL);
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
windows_link::link!("clfsw32.dll" "system" fn WriteLogRestartArea(pvmarshal : super::PVOID, pvrestartbuffer : super::PVOID, cbrestartbuffer : super::ULONG, plsnbase : super::PCLFS_LSN, fflags : super::ULONG, pcbwritten : super::PULONG, plsnnext : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_sys::core::BOOL);
pub type CLFS_LOG_ARCHIVE_CONTEXT = *mut core::ffi::c_void;
#[cfg(all(feature = "clfs", feature = "corecrt_wstdio", feature = "shared", feature = "winnt"))]
pub type CLFS_PRINT_RECORD_ROUTINE = Option<unsafe extern "system" fn(pstrmout: super::PFILE, frecordtype: super::CLFS_RECORD_TYPE, pvbuffer: super::PVOID, cbbuffer: super::ULONG) -> super::ULONG>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCLFS_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(pvoverlapped: super::PVOID, ulreserved: super::ULONG)>;
pub type PCLFS_LOG_ARCHIVE_CONTEXT = *mut *mut core::ffi::c_void;
