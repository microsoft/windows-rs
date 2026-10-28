#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn AddLogContainer<P2>(hlog: super::HANDLE, pcbcontainer: Option<super::PULONGLONG>, pwszcontainerpath: P2, preserved: Option<super::LPVOID>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn AddLogContainer(hlog : super::HANDLE, pcbcontainer : super::PULONGLONG, pwszcontainerpath : windows_core::PCWSTR, preserved : super::LPVOID) -> windows_core::BOOL);
    unsafe { AddLogContainer(hlog, pcbcontainer.unwrap_or(core::mem::zeroed()) as _, pwszcontainerpath.param().abi(), preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddLogContainerSet(hlog: super::HANDLE, pcbcontainer: Option<super::PULONGLONG>, rgwszcontainerpath: &[windows_core::PCWSTR], preserved: Option<super::LPVOID>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn AddLogContainerSet(hlog : super::HANDLE, ccontainer : super::USHORT, pcbcontainer : super::PULONGLONG, rgwszcontainerpath : *const windows_core::PCWSTR, preserved : super::LPVOID) -> windows_core::BOOL);
    unsafe { AddLogContainerSet(hlog, rgwszcontainerpath.len().try_into().unwrap(), pcbcontainer.unwrap_or(core::mem::zeroed()) as _, rgwszcontainerpath.as_ptr(), preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AdvanceLogBase(pvmarshal: super::PVOID, plsnbase: super::PCLFS_LSN, fflags: super::ULONG, poverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn AdvanceLogBase(pvmarshal : super::PVOID, plsnbase : super::PCLFS_LSN, fflags : super::ULONG, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { AdvanceLogBase(pvmarshal as _, plsnbase as _, fflags, poverlapped as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AlignReservedLog(pvmarshal: super::PVOID, creservedrecords: super::ULONG, rgcbreservation: *mut super::LONGLONG, pcbalignreservation: super::PLONGLONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn AlignReservedLog(pvmarshal : super::PVOID, creservedrecords : super::ULONG, rgcbreservation : *mut super::LONGLONG, pcbalignreservation : super::PLONGLONG) -> windows_core::BOOL);
    unsafe { AlignReservedLog(pvmarshal as _, creservedrecords, rgcbreservation as _, pcbalignreservation as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AllocReservedLog(pvmarshal: super::PVOID, creservedrecords: super::ULONG, pcbadjustment: super::PLONGLONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn AllocReservedLog(pvmarshal : super::PVOID, creservedrecords : super::ULONG, pcbadjustment : super::PLONGLONG) -> windows_core::BOOL);
    unsafe { AllocReservedLog(pvmarshal as _, creservedrecords, pcbadjustment as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CloseAndResetLogFile(hlog: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn CloseAndResetLogFile(hlog : super::HANDLE) -> windows_core::BOOL);
    unsafe { CloseAndResetLogFile(hlog as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateLogContainerScanContext(hlog: super::HANDLE, cfromcontainer: super::ULONG, ccontainers: super::ULONG, escanmode: super::CLFS_SCAN_MODE, pcxscan: super::PCLFS_SCAN_CONTEXT, poverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn CreateLogContainerScanContext(hlog : super::HANDLE, cfromcontainer : super::ULONG, ccontainers : super::ULONG, escanmode : super::CLFS_SCAN_MODE, pcxscan : super::PCLFS_SCAN_CONTEXT, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { CreateLogContainerScanContext(hlog as _, cfromcontainer, ccontainers, escanmode, pcxscan as _, poverlapped as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateLogFile<P0>(pszlogfilename: P0, fdesiredaccess: super::ACCESS_MASK, dwsharemode: super::DWORD, psalogfile: super::LPSECURITY_ATTRIBUTES, fcreatedisposition: super::ULONG, fflagsandattributes: super::ULONG) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn CreateLogFile(pszlogfilename : windows_core::PCWSTR, fdesiredaccess : super::ACCESS_MASK, dwsharemode : super::DWORD, psalogfile : super::LPSECURITY_ATTRIBUTES, fcreatedisposition : super::ULONG, fflagsandattributes : super::ULONG) -> super::HANDLE);
    unsafe { CreateLogFile(pszlogfilename.param().abi(), fdesiredaccess, dwsharemode, psalogfile as _, fcreatedisposition, fflagsandattributes) }
}
#[cfg(all(feature = "clfs", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateLogMarshallingArea(hlog: super::HANDLE, pfnallocbuffer: super::CLFS_BLOCK_ALLOCATION, pfnfreebuffer: super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext: super::PVOID, cbmarshallingbuffer: super::ULONG, cmaxwritebuffers: super::ULONG, cmaxreadbuffers: super::ULONG, ppvmarshal: *mut super::PVOID) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn CreateLogMarshallingArea(hlog : super::HANDLE, pfnallocbuffer : super::CLFS_BLOCK_ALLOCATION, pfnfreebuffer : super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext : super::PVOID, cbmarshallingbuffer : super::ULONG, cmaxwritebuffers : super::ULONG, cmaxreadbuffers : super::ULONG, ppvmarshal : *mut super::PVOID) -> windows_core::BOOL);
    unsafe { CreateLogMarshallingArea(hlog as _, pfnallocbuffer, pfnfreebuffer, pvblockalloccontext as _, cbmarshallingbuffer, cmaxwritebuffers, cmaxreadbuffers, ppvmarshal as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteLogByHandle(hlog: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn DeleteLogByHandle(hlog : super::HANDLE) -> windows_core::BOOL);
    unsafe { DeleteLogByHandle(hlog as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteLogFile<P0>(pszlogfilename: P0, pvreserved: super::PVOID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn DeleteLogFile(pszlogfilename : windows_core::PCWSTR, pvreserved : super::PVOID) -> windows_core::BOOL);
    unsafe { DeleteLogFile(pszlogfilename.param().abi(), pvreserved as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteLogMarshallingArea(pvmarshal: super::PVOID) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn DeleteLogMarshallingArea(pvmarshal : super::PVOID) -> windows_core::BOOL);
    unsafe { DeleteLogMarshallingArea(pvmarshal as _) }
}
#[cfg(all(feature = "clfs", feature = "corecrt_wstdio", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DumpLogRecords<P0>(pwszlogfilename: P0, frecordtype: super::CLFS_RECORD_TYPE, plsnstart: Option<super::PCLFS_LSN>, plsnend: Option<super::PCLFS_LSN>, pstrmout: Option<super::PFILE>, pfnprintrecord: CLFS_PRINT_RECORD_ROUTINE, pfnallocblock: super::CLFS_BLOCK_ALLOCATION, pfnfreeblock: super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext: Option<super::PVOID>, cbblock: super::ULONG, cmaxblocks: super::ULONG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn DumpLogRecords(pwszlogfilename : windows_core::PCWSTR, frecordtype : super::CLFS_RECORD_TYPE, plsnstart : super::PCLFS_LSN, plsnend : super::PCLFS_LSN, pstrmout : super::PFILE, pfnprintrecord : CLFS_PRINT_RECORD_ROUTINE, pfnallocblock : super::CLFS_BLOCK_ALLOCATION, pfnfreeblock : super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext : super::PVOID, cbblock : super::ULONG, cmaxblocks : super::ULONG) -> windows_core::BOOL);
    unsafe { DumpLogRecords(pwszlogfilename.param().abi(), frecordtype, plsnstart.unwrap_or(core::mem::zeroed()) as _, plsnend.unwrap_or(core::mem::zeroed()) as _, pstrmout.unwrap_or(core::mem::zeroed()) as _, pfnprintrecord, pfnallocblock, pfnfreeblock, pvblockalloccontext.unwrap_or(core::mem::zeroed()) as _, cbblock, cmaxblocks) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FlushLogBuffers(pvmarshal: super::PVOID, poverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn FlushLogBuffers(pvmarshal : super::PVOID, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { FlushLogBuffers(pvmarshal as _, poverlapped as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FlushLogToLsn(pvmarshalcontext: super::PVOID, plsnflush: super::PCLFS_LSN, plsnlastflushed: super::PCLFS_LSN, poverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn FlushLogToLsn(pvmarshalcontext : super::PVOID, plsnflush : super::PCLFS_LSN, plsnlastflushed : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { FlushLogToLsn(pvmarshalcontext as _, plsnflush as _, plsnlastflushed as _, poverlapped as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FreeReservedLog(pvmarshal: super::PVOID, creservedrecords: super::ULONG, pcbadjustment: super::PLONGLONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn FreeReservedLog(pvmarshal : super::PVOID, creservedrecords : super::ULONG, pcbadjustment : super::PLONGLONG) -> windows_core::BOOL);
    unsafe { FreeReservedLog(pvmarshal as _, creservedrecords, pcbadjustment as _) }
}
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetLogContainerName<P2>(hlog: super::HANDLE, cidlogicalcontainer: super::CLFS_CONTAINER_ID, pwstrcontainername: P2, clencontainername: super::ULONG, pcactuallencontainername: super::PULONG) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn GetLogContainerName(hlog : super::HANDLE, cidlogicalcontainer : super::CLFS_CONTAINER_ID, pwstrcontainername : windows_core::PCWSTR, clencontainername : super::ULONG, pcactuallencontainername : super::PULONG) -> windows_core::BOOL);
    unsafe { GetLogContainerName(hlog as _, cidlogicalcontainer, pwstrcontainername.param().abi(), clencontainername, pcactuallencontainername as _) }
}
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetLogFileInformation(hlog: super::HANDLE, pinfobuffer: super::PCLFS_INFORMATION, cbbuffer: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn GetLogFileInformation(hlog : super::HANDLE, pinfobuffer : super::PCLFS_INFORMATION, cbbuffer : super::PULONG) -> windows_core::BOOL);
    unsafe { GetLogFileInformation(hlog as _, pinfobuffer as _, cbbuffer as _) }
}
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetLogIoStatistics(hlog: super::HANDLE, pvstatsbuffer: super::PVOID, cbstatsbuffer: super::ULONG, estatsclass: super::CLFS_IOSTATS_CLASS, pcbstatswritten: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn GetLogIoStatistics(hlog : super::HANDLE, pvstatsbuffer : super::PVOID, cbstatsbuffer : super::ULONG, estatsclass : super::CLFS_IOSTATS_CLASS, pcbstatswritten : super::PULONG) -> windows_core::BOOL);
    unsafe { GetLogIoStatistics(hlog as _, pvstatsbuffer as _, cbstatsbuffer, estatsclass, pcbstatswritten as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetLogReservationInfo(pvmarshal: super::PVOID, pcbrecordnumber: super::PULONG, pcbuserreservation: super::PLONGLONG, pcbcommitreservation: super::PLONGLONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn GetLogReservationInfo(pvmarshal : super::PVOID, pcbrecordnumber : super::PULONG, pcbuserreservation : super::PLONGLONG, pcbcommitreservation : super::PLONGLONG) -> windows_core::BOOL);
    unsafe { GetLogReservationInfo(pvmarshal, pcbrecordnumber as _, pcbuserreservation as _, pcbcommitreservation as _) }
}
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetNextLogArchiveExtent(pvarchivecontext: CLFS_LOG_ARCHIVE_CONTEXT, rgadextent: *mut super::CLFS_ARCHIVE_DESCRIPTOR, cdescriptors: super::ULONG, pcdescriptorsreturned: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn GetNextLogArchiveExtent(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT, rgadextent : *mut super::CLFS_ARCHIVE_DESCRIPTOR, cdescriptors : super::ULONG, pcdescriptorsreturned : super::PULONG) -> windows_core::BOOL);
    unsafe { GetNextLogArchiveExtent(pvarchivecontext as _, rgadextent as _, cdescriptors, pcdescriptorsreturned as _) }
}
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PrepareLogArchive(hlog: super::HANDLE, pszbaselogfilename: &mut [u16], plsnlow: Option<super::PCLFS_LSN>, plsnhigh: Option<super::PCLFS_LSN>, pcactuallength: Option<super::PULONG>, poffbaselogfiledata: super::PULONGLONG, pcbbaselogfilelength: super::PULONGLONG, plsnbase: super::PCLFS_LSN, plsnlast: super::PCLFS_LSN, plsncurrentarchivetail: super::PCLFS_LSN, ppvarchivecontext: PCLFS_LOG_ARCHIVE_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn PrepareLogArchive(hlog : super::HANDLE, pszbaselogfilename : windows_core::PWSTR, clen : super::ULONG, plsnlow : super::PCLFS_LSN, plsnhigh : super::PCLFS_LSN, pcactuallength : super::PULONG, poffbaselogfiledata : super::PULONGLONG, pcbbaselogfilelength : super::PULONGLONG, plsnbase : super::PCLFS_LSN, plsnlast : super::PCLFS_LSN, plsncurrentarchivetail : super::PCLFS_LSN, ppvarchivecontext : PCLFS_LOG_ARCHIVE_CONTEXT) -> windows_core::BOOL);
    unsafe { PrepareLogArchive(hlog, core::mem::transmute(pszbaselogfilename.as_mut_ptr()), pszbaselogfilename.len().try_into().unwrap(), plsnlow.unwrap_or(core::mem::zeroed()) as _, plsnhigh.unwrap_or(core::mem::zeroed()) as _, pcactuallength.unwrap_or(core::mem::zeroed()) as _, poffbaselogfiledata as _, pcbbaselogfilelength as _, plsnbase as _, plsnlast as _, plsncurrentarchivetail as _, ppvarchivecontext as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn ReadLogArchiveMetadata(pvarchivecontext: CLFS_LOG_ARCHIVE_CONTEXT, cboffset: super::ULONG, cbbytestoread: super::ULONG, pbreadbuffer: super::PBYTE, pcbbytesread: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadLogArchiveMetadata(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT, cboffset : super::ULONG, cbbytestoread : super::ULONG, pbreadbuffer : super::PBYTE, pcbbytesread : super::PULONG) -> windows_core::BOOL);
    unsafe { ReadLogArchiveMetadata(pvarchivecontext as _, cboffset, cbbytestoread, pbreadbuffer as _, pcbbytesread as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadLogRecord(pvmarshal: super::PVOID, plsnfirst: super::PCLFS_LSN, econtextmode: super::CLFS_CONTEXT_MODE, ppvreadbuffer: *mut super::PVOID, pcbreadbuffer: super::PULONG, perecordtype: super::PCLFS_RECORD_TYPE, plsnundonext: super::PCLFS_LSN, plsnprevious: super::PCLFS_LSN, ppvreadcontext: *mut super::PVOID, poverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadLogRecord(pvmarshal : super::PVOID, plsnfirst : super::PCLFS_LSN, econtextmode : super::CLFS_CONTEXT_MODE, ppvreadbuffer : *mut super::PVOID, pcbreadbuffer : super::PULONG, perecordtype : super::PCLFS_RECORD_TYPE, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, ppvreadcontext : *mut super::PVOID, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadLogRecord(pvmarshal as _, plsnfirst as _, econtextmode, ppvreadbuffer as _, pcbreadbuffer as _, perecordtype as _, plsnundonext as _, plsnprevious as _, ppvreadcontext as _, poverlapped as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadLogRestartArea(pvmarshal: super::PVOID, ppvrestartbuffer: *mut super::PVOID, pcbrestartbuffer: super::PULONG, plsn: super::PCLFS_LSN, ppvcontext: *mut super::PVOID, poverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadLogRestartArea(pvmarshal : super::PVOID, ppvrestartbuffer : *mut super::PVOID, pcbrestartbuffer : super::PULONG, plsn : super::PCLFS_LSN, ppvcontext : *mut super::PVOID, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadLogRestartArea(pvmarshal as _, ppvrestartbuffer as _, pcbrestartbuffer as _, plsn as _, ppvcontext as _, poverlapped as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadNextLogRecord(pvreadcontext: super::PVOID, ppvbuffer: *mut super::PVOID, pcbbuffer: super::PULONG, perecordtype: super::PCLFS_RECORD_TYPE, plsnuser: super::PCLFS_LSN, plsnundonext: super::PCLFS_LSN, plsnprevious: super::PCLFS_LSN, plsnrecord: super::PCLFS_LSN, poverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadNextLogRecord(pvreadcontext : super::PVOID, ppvbuffer : *mut super::PVOID, pcbbuffer : super::PULONG, perecordtype : super::PCLFS_RECORD_TYPE, plsnuser : super::PCLFS_LSN, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, plsnrecord : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadNextLogRecord(pvreadcontext as _, ppvbuffer as _, pcbbuffer as _, perecordtype as _, plsnuser as _, plsnundonext as _, plsnprevious as _, plsnrecord as _, poverlapped as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadPreviousLogRestartArea(pvreadcontext: super::PVOID, ppvrestartbuffer: *mut super::PVOID, pcbrestartbuffer: super::PULONG, plsnrestart: super::PCLFS_LSN, poverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadPreviousLogRestartArea(pvreadcontext : super::PVOID, ppvrestartbuffer : *mut super::PVOID, pcbrestartbuffer : super::PULONG, plsnrestart : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadPreviousLogRestartArea(pvreadcontext as _, ppvrestartbuffer as _, pcbrestartbuffer as _, plsnrestart as _, poverlapped as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn RemoveLogContainer<P1>(hlog: super::HANDLE, pwszcontainerpath: P1, fforce: bool, preserved: Option<super::LPVOID>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn RemoveLogContainer(hlog : super::HANDLE, pwszcontainerpath : windows_core::PCWSTR, fforce : windows_core::BOOL, preserved : super::LPVOID) -> windows_core::BOOL);
    unsafe { RemoveLogContainer(hlog, pwszcontainerpath.param().abi(), fforce.into(), preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RemoveLogContainerSet(hlog: super::HANDLE, rgwszcontainerpath: &[windows_core::PCWSTR], fforce: bool, preserved: Option<super::LPVOID>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn RemoveLogContainerSet(hlog : super::HANDLE, ccontainer : super::USHORT, rgwszcontainerpath : *const windows_core::PCWSTR, fforce : windows_core::BOOL, preserved : super::LPVOID) -> windows_core::BOOL);
    unsafe { RemoveLogContainerSet(hlog, rgwszcontainerpath.len().try_into().unwrap(), rgwszcontainerpath.as_ptr(), fforce.into(), preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReserveAndAppendLog(pvmarshal: super::PVOID, rgwriteentries: super::PCLFS_WRITE_ENTRY, cwriteentries: super::ULONG, plsnundonext: super::PCLFS_LSN, plsnprevious: super::PCLFS_LSN, creserverecords: super::ULONG, rgcbreservation: *mut super::LONGLONG, fflags: super::ULONG, plsn: super::PCLFS_LSN, poverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReserveAndAppendLog(pvmarshal : super::PVOID, rgwriteentries : super::PCLFS_WRITE_ENTRY, cwriteentries : super::ULONG, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, creserverecords : super::ULONG, rgcbreservation : *mut super::LONGLONG, fflags : super::ULONG, plsn : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ReserveAndAppendLog(pvmarshal as _, rgwriteentries as _, cwriteentries, plsnundonext as _, plsnprevious as _, creserverecords, rgcbreservation as _, fflags, plsn as _, poverlapped as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReserveAndAppendLogAligned(pvmarshal: super::PVOID, rgwriteentries: super::PCLFS_WRITE_ENTRY, cwriteentries: super::ULONG, cbentryalignment: super::ULONG, plsnundonext: super::PCLFS_LSN, plsnprevious: super::PCLFS_LSN, creserverecords: super::ULONG, rgcbreservation: *mut super::LONGLONG, fflags: super::ULONG, plsn: super::PCLFS_LSN, poverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReserveAndAppendLogAligned(pvmarshal : super::PVOID, rgwriteentries : super::PCLFS_WRITE_ENTRY, cwriteentries : super::ULONG, cbentryalignment : super::ULONG, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, creserverecords : super::ULONG, rgcbreservation : *mut super::LONGLONG, fflags : super::ULONG, plsn : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ReserveAndAppendLogAligned(pvmarshal as _, rgwriteentries as _, cwriteentries, cbentryalignment, plsnundonext as _, plsnprevious as _, creserverecords, rgcbreservation as _, fflags, plsn as _, poverlapped as _) }
}
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ScanLogContainers(pcxscan: super::PCLFS_SCAN_CONTEXT, escanmode: super::CLFS_SCAN_MODE, preserved: super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ScanLogContainers(pcxscan : super::PCLFS_SCAN_CONTEXT, escanmode : super::CLFS_SCAN_MODE, preserved : super::LPVOID) -> windows_core::BOOL);
    unsafe { ScanLogContainers(pcxscan as _, escanmode, preserved as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetEndOfLog(hlog: super::HANDLE, plsnend: super::PCLFS_LSN, lpoverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn SetEndOfLog(hlog : super::HANDLE, plsnend : super::PCLFS_LSN, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { SetEndOfLog(hlog as _, plsnend as _, lpoverlapped as _) }
}
#[cfg(all(feature = "clfs", feature = "winnt"))]
#[inline]
pub unsafe fn SetLogArchiveMode(hlog: super::HANDLE, emode: super::CLFS_LOG_ARCHIVE_MODE) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn SetLogArchiveMode(hlog : super::HANDLE, emode : super::CLFS_LOG_ARCHIVE_MODE) -> windows_core::BOOL);
    unsafe { SetLogArchiveMode(hlog as _, emode) }
}
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetLogArchiveTail(hlog: super::HANDLE, plsnarchivetail: super::PCLFS_LSN, preserved: super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn SetLogArchiveTail(hlog : super::HANDLE, plsnarchivetail : super::PCLFS_LSN, preserved : super::LPVOID) -> windows_core::BOOL);
    unsafe { SetLogArchiveTail(hlog as _, plsnarchivetail as _, preserved as _) }
}
#[inline]
pub unsafe fn TerminateLogArchive(pvarchivecontext: CLFS_LOG_ARCHIVE_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn TerminateLogArchive(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT) -> windows_core::BOOL);
    unsafe { TerminateLogArchive(pvarchivecontext as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TerminateReadLog(pvcursorcontext: super::PVOID) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn TerminateReadLog(pvcursorcontext : super::PVOID) -> windows_core::BOOL);
    unsafe { TerminateReadLog(pvcursorcontext as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TruncateLog(pvmarshal: super::PVOID, plsnend: super::PCLFS_LSN, lpoverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn TruncateLog(pvmarshal : super::PVOID, plsnend : super::PCLFS_LSN, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { TruncateLog(pvmarshal, plsnend, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn ValidateLog<P0>(pszlogfilename: P0, psalogfile: super::LPSECURITY_ATTRIBUTES, pinfobuffer: super::PCLFS_INFORMATION, pcbbuffer: super::PULONG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn ValidateLog(pszlogfilename : windows_core::PCWSTR, psalogfile : super::LPSECURITY_ATTRIBUTES, pinfobuffer : super::PCLFS_INFORMATION, pcbbuffer : super::PULONG) -> windows_core::BOOL);
    unsafe { ValidateLog(pszlogfilename.param().abi(), psalogfile as _, pinfobuffer as _, pcbbuffer as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WriteLogRestartArea(pvmarshal: super::PVOID, pvrestartbuffer: super::PVOID, cbrestartbuffer: super::ULONG, plsnbase: super::PCLFS_LSN, fflags: super::ULONG, pcbwritten: super::PULONG, plsnnext: super::PCLFS_LSN, poverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn WriteLogRestartArea(pvmarshal : super::PVOID, pvrestartbuffer : super::PVOID, cbrestartbuffer : super::ULONG, plsnbase : super::PCLFS_LSN, fflags : super::ULONG, pcbwritten : super::PULONG, plsnnext : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { WriteLogRestartArea(pvmarshal as _, pvrestartbuffer as _, cbrestartbuffer, plsnbase as _, fflags, pcbwritten as _, plsnnext as _, poverlapped as _) }
}
pub type CLFS_LOG_ARCHIVE_CONTEXT = *mut core::ffi::c_void;
#[cfg(all(feature = "clfs", feature = "corecrt_wstdio", feature = "shared", feature = "winnt"))]
pub type CLFS_PRINT_RECORD_ROUTINE = Option<unsafe extern "system" fn(pstrmout: super::PFILE, frecordtype: super::CLFS_RECORD_TYPE, pvbuffer: super::PVOID, cbbuffer: super::ULONG) -> super::ULONG>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCLFS_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(pvoverlapped: super::PVOID, ulreserved: super::ULONG)>;
pub type PCLFS_LOG_ARCHIVE_CONTEXT = *mut *mut core::ffi::c_void;
