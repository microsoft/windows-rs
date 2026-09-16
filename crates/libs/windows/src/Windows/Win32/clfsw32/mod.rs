#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddLogContainer<P2>(hlog: super::HANDLE, pcbcontainer: Option<super::PULONGLONG>, pwszcontainerpath: P2, preserved: Option<*mut core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn AddLogContainer(hlog : super::HANDLE, pcbcontainer : super::PULONGLONG, pwszcontainerpath : windows_core::PCWSTR, preserved : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { AddLogContainer(hlog, pcbcontainer.unwrap_or(core::mem::zeroed()) as _, pwszcontainerpath.param().abi(), preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddLogContainerSet(hlog: super::HANDLE, pcbcontainer: Option<super::PULONGLONG>, rgwszcontainerpath: &[windows_core::PCWSTR], preserved: Option<*mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn AddLogContainerSet(hlog : super::HANDLE, ccontainer : u16, pcbcontainer : super::PULONGLONG, rgwszcontainerpath : *const windows_core::PCWSTR, preserved : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { AddLogContainerSet(hlog, rgwszcontainerpath.len().try_into().unwrap(), pcbcontainer.unwrap_or(core::mem::zeroed()) as _, rgwszcontainerpath.as_ptr(), preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn AdvanceLogBase(pvmarshal: *mut core::ffi::c_void, plsnbase: super::PCLFS_LSN, fflags: u32, poverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn AdvanceLogBase(pvmarshal : *mut core::ffi::c_void, plsnbase : super::PCLFS_LSN, fflags : u32, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { AdvanceLogBase(pvmarshal as _, plsnbase, fflags, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AlignReservedLog(pvmarshal: *mut core::ffi::c_void, creservedrecords: u32, rgcbreservation: *const i64, pcbalignreservation: super::PLONGLONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn AlignReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, rgcbreservation : *const i64, pcbalignreservation : super::PLONGLONG) -> windows_core::BOOL);
    unsafe { AlignReservedLog(pvmarshal as _, creservedrecords, rgcbreservation, pcbalignreservation as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AllocReservedLog(pvmarshal: *mut core::ffi::c_void, creservedrecords: u32, pcbadjustment: super::PLONGLONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn AllocReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, pcbadjustment : super::PLONGLONG) -> windows_core::BOOL);
    unsafe { AllocReservedLog(pvmarshal as _, creservedrecords, pcbadjustment as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CloseAndResetLogFile(hlog: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn CloseAndResetLogFile(hlog : super::HANDLE) -> windows_core::BOOL);
    unsafe { CloseAndResetLogFile(hlog) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateLogContainerScanContext(hlog: super::HANDLE, cfromcontainer: u32, ccontainers: u32, escanmode: super::CLFS_SCAN_MODE, pcxscan: super::PCLFS_SCAN_CONTEXT, poverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn CreateLogContainerScanContext(hlog : super::HANDLE, cfromcontainer : u32, ccontainers : u32, escanmode : super::CLFS_SCAN_MODE, pcxscan : super::PCLFS_SCAN_CONTEXT, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { CreateLogContainerScanContext(hlog, cfromcontainer, ccontainers, escanmode, pcxscan as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateLogFile<P0>(pszlogfilename: P0, fdesiredaccess: super::ACCESS_MASK, dwsharemode: u32, psalogfile: Option<super::LPSECURITY_ATTRIBUTES>, fcreatedisposition: u32, fflagsandattributes: u32) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn CreateLogFile(pszlogfilename : windows_core::PCWSTR, fdesiredaccess : super::ACCESS_MASK, dwsharemode : u32, psalogfile : super::LPSECURITY_ATTRIBUTES, fcreatedisposition : u32, fflagsandattributes : u32) -> super::HANDLE);
    unsafe { CreateLogFile(pszlogfilename.param().abi(), fdesiredaccess, dwsharemode, psalogfile.unwrap_or(core::mem::zeroed()) as _, fcreatedisposition, fflagsandattributes) }
}
#[cfg(all(feature = "clfs", feature = "winnt"))]
#[inline]
pub unsafe fn CreateLogMarshallingArea(hlog: super::HANDLE, pfnallocbuffer: super::CLFS_BLOCK_ALLOCATION, pfnfreebuffer: super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext: Option<*const core::ffi::c_void>, cbmarshallingbuffer: u32, cmaxwritebuffers: u32, cmaxreadbuffers: u32, ppvmarshal: *mut *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn CreateLogMarshallingArea(hlog : super::HANDLE, pfnallocbuffer : super::CLFS_BLOCK_ALLOCATION, pfnfreebuffer : super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext : *const core::ffi::c_void, cbmarshallingbuffer : u32, cmaxwritebuffers : u32, cmaxreadbuffers : u32, ppvmarshal : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { CreateLogMarshallingArea(hlog, pfnallocbuffer, pfnfreebuffer, pvblockalloccontext.unwrap_or(core::mem::zeroed()) as _, cbmarshallingbuffer, cmaxwritebuffers, cmaxreadbuffers, ppvmarshal as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteLogByHandle(hlog: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn DeleteLogByHandle(hlog : super::HANDLE) -> windows_core::BOOL);
    unsafe { DeleteLogByHandle(hlog) }
}
#[inline]
pub unsafe fn DeleteLogFile<P0>(pszlogfilename: P0, pvreserved: Option<*const core::ffi::c_void>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn DeleteLogFile(pszlogfilename : windows_core::PCWSTR, pvreserved : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DeleteLogFile(pszlogfilename.param().abi(), pvreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn DeleteLogMarshallingArea(pvmarshal: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn DeleteLogMarshallingArea(pvmarshal : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { DeleteLogMarshallingArea(pvmarshal) }
}
#[cfg(all(feature = "clfs", feature = "corecrt_wstdio"))]
#[inline]
pub unsafe fn DumpLogRecords<P0>(pwszlogfilename: P0, frecordtype: super::CLFS_RECORD_TYPE, plsnstart: Option<super::PCLFS_LSN>, plsnend: Option<super::PCLFS_LSN>, pstrmout: Option<super::PFILE>, pfnprintrecord: CLFS_PRINT_RECORD_ROUTINE, pfnallocblock: super::CLFS_BLOCK_ALLOCATION, pfnfreeblock: super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext: Option<*const core::ffi::c_void>, cbblock: u32, cmaxblocks: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn DumpLogRecords(pwszlogfilename : windows_core::PCWSTR, frecordtype : super::CLFS_RECORD_TYPE, plsnstart : super::PCLFS_LSN, plsnend : super::PCLFS_LSN, pstrmout : super::PFILE, pfnprintrecord : CLFS_PRINT_RECORD_ROUTINE, pfnallocblock : super::CLFS_BLOCK_ALLOCATION, pfnfreeblock : super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext : *const core::ffi::c_void, cbblock : u32, cmaxblocks : u32) -> windows_core::BOOL);
    unsafe { DumpLogRecords(pwszlogfilename.param().abi(), frecordtype, plsnstart.unwrap_or(core::mem::zeroed()) as _, plsnend.unwrap_or(core::mem::zeroed()) as _, pstrmout.unwrap_or(core::mem::zeroed()) as _, pfnprintrecord, pfnallocblock, pfnfreeblock, pvblockalloccontext.unwrap_or(core::mem::zeroed()) as _, cbblock, cmaxblocks) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn FlushLogBuffers(pvmarshal: *const core::ffi::c_void, poverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn FlushLogBuffers(pvmarshal : *const core::ffi::c_void, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { FlushLogBuffers(pvmarshal, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn FlushLogToLsn(pvmarshalcontext: *const core::ffi::c_void, plsnflush: super::PCLFS_LSN, plsnlastflushed: Option<super::PCLFS_LSN>, poverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn FlushLogToLsn(pvmarshalcontext : *const core::ffi::c_void, plsnflush : super::PCLFS_LSN, plsnlastflushed : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { FlushLogToLsn(pvmarshalcontext, plsnflush, plsnlastflushed.unwrap_or(core::mem::zeroed()) as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FreeReservedLog(pvmarshal: *mut core::ffi::c_void, creservedrecords: u32, pcbadjustment: super::PLONGLONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn FreeReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, pcbadjustment : super::PLONGLONG) -> windows_core::BOOL);
    unsafe { FreeReservedLog(pvmarshal as _, creservedrecords, pcbadjustment as _) }
}
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetLogContainerName(hlog: super::HANDLE, cidlogicalcontainer: super::CLFS_CONTAINER_ID, pwstrcontainername: windows_core::PCWSTR, clencontainername: u32, pcactuallencontainername: Option<super::PULONG>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn GetLogContainerName(hlog : super::HANDLE, cidlogicalcontainer : super::CLFS_CONTAINER_ID, pwstrcontainername : windows_core::PCWSTR, clencontainername : u32, pcactuallencontainername : super::PULONG) -> windows_core::BOOL);
    unsafe { GetLogContainerName(hlog, cidlogicalcontainer, pwstrcontainername, clencontainername, pcactuallencontainername.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetLogFileInformation(hlog: super::HANDLE, pinfobuffer: super::PCLFS_INFORMATION, cbbuffer: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn GetLogFileInformation(hlog : super::HANDLE, pinfobuffer : super::PCLFS_INFORMATION, cbbuffer : super::PULONG) -> windows_core::BOOL);
    unsafe { GetLogFileInformation(hlog, pinfobuffer as _, cbbuffer as _) }
}
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetLogIoStatistics(hlog: super::HANDLE, pvstatsbuffer: *mut core::ffi::c_void, cbstatsbuffer: u32, estatsclass: super::CLFS_IOSTATS_CLASS, pcbstatswritten: Option<super::PULONG>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn GetLogIoStatistics(hlog : super::HANDLE, pvstatsbuffer : *mut core::ffi::c_void, cbstatsbuffer : u32, estatsclass : super::CLFS_IOSTATS_CLASS, pcbstatswritten : super::PULONG) -> windows_core::BOOL);
    unsafe { GetLogIoStatistics(hlog, pvstatsbuffer as _, cbstatsbuffer, estatsclass, pcbstatswritten.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetLogReservationInfo(pvmarshal: *const core::ffi::c_void, pcbrecordnumber: super::PULONG, pcbuserreservation: super::PLONGLONG, pcbcommitreservation: super::PLONGLONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn GetLogReservationInfo(pvmarshal : *const core::ffi::c_void, pcbrecordnumber : super::PULONG, pcbuserreservation : super::PLONGLONG, pcbcommitreservation : super::PLONGLONG) -> windows_core::BOOL);
    unsafe { GetLogReservationInfo(pvmarshal, pcbrecordnumber as _, pcbuserreservation as _, pcbcommitreservation as _) }
}
#[cfg(all(feature = "clfs", feature = "minwindef"))]
#[inline]
pub unsafe fn GetNextLogArchiveExtent(pvarchivecontext: CLFS_LOG_ARCHIVE_CONTEXT, rgadextent: *mut super::CLFS_ARCHIVE_DESCRIPTOR, cdescriptors: u32, pcdescriptorsreturned: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn GetNextLogArchiveExtent(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT, rgadextent : *mut super::CLFS_ARCHIVE_DESCRIPTOR, cdescriptors : u32, pcdescriptorsreturned : super::PULONG) -> windows_core::BOOL);
    unsafe { GetNextLogArchiveExtent(pvarchivecontext, rgadextent as _, cdescriptors, pcdescriptorsreturned as _) }
}
#[cfg(feature = "clfs")]
#[inline]
pub unsafe fn LsnBlockOffset(plsn: *const super::CLFS_LSN) -> u32 {
    windows_core::link!("clfsw32.dll" "system" fn LsnBlockOffset(plsn : *const super::CLFS_LSN) -> u32);
    unsafe { LsnBlockOffset(plsn) }
}
#[cfg(feature = "clfs")]
#[inline]
pub unsafe fn LsnContainer(plsn: *const super::CLFS_LSN) -> super::CLFS_CONTAINER_ID {
    windows_core::link!("clfsw32.dll" "system" fn LsnContainer(plsn : *const super::CLFS_LSN) -> super::CLFS_CONTAINER_ID);
    unsafe { LsnContainer(plsn) }
}
#[cfg(feature = "clfs")]
#[inline]
pub unsafe fn LsnCreate(cidcontainer: super::CLFS_CONTAINER_ID, offblock: u32, crecord: u32) -> super::CLFS_LSN {
    windows_core::link!("clfsw32.dll" "system" fn LsnCreate(cidcontainer : super::CLFS_CONTAINER_ID, offblock : u32, crecord : u32) -> super::CLFS_LSN);
    unsafe { LsnCreate(cidcontainer, offblock, crecord) }
}
#[cfg(feature = "clfs")]
#[inline]
pub unsafe fn LsnRecordSequence(plsn: *const super::CLFS_LSN) -> u32 {
    windows_core::link!("clfsw32.dll" "system" fn LsnRecordSequence(plsn : *const super::CLFS_LSN) -> u32);
    unsafe { LsnRecordSequence(plsn) }
}
#[cfg(all(feature = "clfs", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn PrepareLogArchive(hlog: super::HANDLE, pszbaselogfilename: &mut [u16], plsnlow: Option<super::PCLFS_LSN>, plsnhigh: Option<super::PCLFS_LSN>, pcactuallength: Option<super::PULONG>, poffbaselogfiledata: super::PULONGLONG, pcbbaselogfilelength: super::PULONGLONG, plsnbase: super::PCLFS_LSN, plsnlast: super::PCLFS_LSN, plsncurrentarchivetail: super::PCLFS_LSN, ppvarchivecontext: PCLFS_LOG_ARCHIVE_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn PrepareLogArchive(hlog : super::HANDLE, pszbaselogfilename : windows_core::PWSTR, clen : u32, plsnlow : super::PCLFS_LSN, plsnhigh : super::PCLFS_LSN, pcactuallength : super::PULONG, poffbaselogfiledata : super::PULONGLONG, pcbbaselogfilelength : super::PULONGLONG, plsnbase : super::PCLFS_LSN, plsnlast : super::PCLFS_LSN, plsncurrentarchivetail : super::PCLFS_LSN, ppvarchivecontext : PCLFS_LOG_ARCHIVE_CONTEXT) -> windows_core::BOOL);
    unsafe { PrepareLogArchive(hlog, core::mem::transmute(pszbaselogfilename.as_mut_ptr()), pszbaselogfilename.len().try_into().unwrap(), plsnlow.unwrap_or(core::mem::zeroed()) as _, plsnhigh.unwrap_or(core::mem::zeroed()) as _, pcactuallength.unwrap_or(core::mem::zeroed()) as _, poffbaselogfiledata as _, pcbbaselogfilelength as _, plsnbase as _, plsnlast as _, plsncurrentarchivetail as _, ppvarchivecontext as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ReadLogArchiveMetadata(pvarchivecontext: CLFS_LOG_ARCHIVE_CONTEXT, cboffset: u32, cbbytestoread: u32, pbreadbuffer: super::PBYTE, pcbbytesread: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadLogArchiveMetadata(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT, cboffset : u32, cbbytestoread : u32, pbreadbuffer : super::PBYTE, pcbbytesread : super::PULONG) -> windows_core::BOOL);
    unsafe { ReadLogArchiveMetadata(pvarchivecontext, cboffset, cbbytestoread, pbreadbuffer as _, pcbbytesread as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn ReadLogRecord(pvmarshal: *const core::ffi::c_void, plsnfirst: super::PCLFS_LSN, econtextmode: super::CLFS_CONTEXT_MODE, ppvreadbuffer: *mut *mut core::ffi::c_void, pcbreadbuffer: super::PULONG, perecordtype: super::PCLFS_RECORD_TYPE, plsnundonext: super::PCLFS_LSN, plsnprevious: super::PCLFS_LSN, ppvreadcontext: *mut *mut core::ffi::c_void, poverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadLogRecord(pvmarshal : *const core::ffi::c_void, plsnfirst : super::PCLFS_LSN, econtextmode : super::CLFS_CONTEXT_MODE, ppvreadbuffer : *mut *mut core::ffi::c_void, pcbreadbuffer : super::PULONG, perecordtype : super::PCLFS_RECORD_TYPE, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, ppvreadcontext : *mut *mut core::ffi::c_void, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadLogRecord(pvmarshal, plsnfirst, econtextmode, ppvreadbuffer as _, pcbreadbuffer as _, perecordtype as _, plsnundonext as _, plsnprevious as _, ppvreadcontext as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn ReadLogRestartArea(pvmarshal: *const core::ffi::c_void, ppvrestartbuffer: *mut *mut core::ffi::c_void, pcbrestartbuffer: super::PULONG, plsn: super::PCLFS_LSN, ppvcontext: *mut *mut core::ffi::c_void, poverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadLogRestartArea(pvmarshal : *const core::ffi::c_void, ppvrestartbuffer : *mut *mut core::ffi::c_void, pcbrestartbuffer : super::PULONG, plsn : super::PCLFS_LSN, ppvcontext : *mut *mut core::ffi::c_void, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadLogRestartArea(pvmarshal, ppvrestartbuffer as _, pcbrestartbuffer as _, plsn as _, ppvcontext as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn ReadNextLogRecord(pvreadcontext: *mut core::ffi::c_void, ppvbuffer: *mut *mut core::ffi::c_void, pcbbuffer: super::PULONG, perecordtype: super::PCLFS_RECORD_TYPE, plsnuser: Option<super::PCLFS_LSN>, plsnundonext: super::PCLFS_LSN, plsnprevious: super::PCLFS_LSN, plsnrecord: super::PCLFS_LSN, poverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadNextLogRecord(pvreadcontext : *mut core::ffi::c_void, ppvbuffer : *mut *mut core::ffi::c_void, pcbbuffer : super::PULONG, perecordtype : super::PCLFS_RECORD_TYPE, plsnuser : super::PCLFS_LSN, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, plsnrecord : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadNextLogRecord(pvreadcontext as _, ppvbuffer as _, pcbbuffer as _, perecordtype as _, plsnuser.unwrap_or(core::mem::zeroed()) as _, plsnundonext as _, plsnprevious as _, plsnrecord as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn ReadPreviousLogRestartArea(pvreadcontext: *const core::ffi::c_void, ppvrestartbuffer: *mut *mut core::ffi::c_void, pcbrestartbuffer: super::PULONG, plsnrestart: super::PCLFS_LSN, poverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadPreviousLogRestartArea(pvreadcontext : *const core::ffi::c_void, ppvrestartbuffer : *mut *mut core::ffi::c_void, pcbrestartbuffer : super::PULONG, plsnrestart : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadPreviousLogRestartArea(pvreadcontext, ppvrestartbuffer as _, pcbrestartbuffer as _, plsnrestart as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RemoveLogContainer<P1>(hlog: super::HANDLE, pwszcontainerpath: P1, fforce: bool, preserved: Option<*mut core::ffi::c_void>) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn RemoveLogContainer(hlog : super::HANDLE, pwszcontainerpath : windows_core::PCWSTR, fforce : windows_core::BOOL, preserved : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RemoveLogContainer(hlog, pwszcontainerpath.param().abi(), fforce.into(), preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RemoveLogContainerSet(hlog: super::HANDLE, rgwszcontainerpath: &[windows_core::PCWSTR], fforce: bool, preserved: Option<*mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn RemoveLogContainerSet(hlog : super::HANDLE, ccontainer : u16, rgwszcontainerpath : *const windows_core::PCWSTR, fforce : windows_core::BOOL, preserved : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RemoveLogContainerSet(hlog, rgwszcontainerpath.len().try_into().unwrap(), rgwszcontainerpath.as_ptr(), fforce.into(), preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn ReserveAndAppendLog(pvmarshal: *const core::ffi::c_void, rgwriteentries: Option<super::PCLFS_WRITE_ENTRY>, cwriteentries: u32, plsnundonext: Option<super::PCLFS_LSN>, plsnprevious: Option<super::PCLFS_LSN>, creserverecords: u32, rgcbreservation: Option<*mut i64>, fflags: u32, plsn: Option<super::PCLFS_LSN>, poverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReserveAndAppendLog(pvmarshal : *const core::ffi::c_void, rgwriteentries : super::PCLFS_WRITE_ENTRY, cwriteentries : u32, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, creserverecords : u32, rgcbreservation : *mut i64, fflags : u32, plsn : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ReserveAndAppendLog(pvmarshal, rgwriteentries.unwrap_or(core::mem::zeroed()) as _, cwriteentries, plsnundonext.unwrap_or(core::mem::zeroed()) as _, plsnprevious.unwrap_or(core::mem::zeroed()) as _, creserverecords, rgcbreservation.unwrap_or(core::mem::zeroed()) as _, fflags, plsn.unwrap_or(core::mem::zeroed()) as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn ReserveAndAppendLogAligned(pvmarshal: *const core::ffi::c_void, rgwriteentries: Option<super::PCLFS_WRITE_ENTRY>, cwriteentries: u32, cbentryalignment: u32, plsnundonext: Option<super::PCLFS_LSN>, plsnprevious: Option<super::PCLFS_LSN>, creserverecords: u32, rgcbreservation: Option<*mut i64>, fflags: u32, plsn: Option<super::PCLFS_LSN>, poverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReserveAndAppendLogAligned(pvmarshal : *const core::ffi::c_void, rgwriteentries : super::PCLFS_WRITE_ENTRY, cwriteentries : u32, cbentryalignment : u32, plsnundonext : super::PCLFS_LSN, plsnprevious : super::PCLFS_LSN, creserverecords : u32, rgcbreservation : *mut i64, fflags : u32, plsn : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { ReserveAndAppendLogAligned(pvmarshal, rgwriteentries.unwrap_or(core::mem::zeroed()) as _, cwriteentries, cbentryalignment, plsnundonext.unwrap_or(core::mem::zeroed()) as _, plsnprevious.unwrap_or(core::mem::zeroed()) as _, creserverecords, rgcbreservation.unwrap_or(core::mem::zeroed()) as _, fflags, plsn.unwrap_or(core::mem::zeroed()) as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "winnt"))]
#[inline]
pub unsafe fn ScanLogContainers(pcxscan: super::PCLFS_SCAN_CONTEXT, escanmode: super::CLFS_SCAN_MODE, preserved: Option<*mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ScanLogContainers(pcxscan : super::PCLFS_SCAN_CONTEXT, escanmode : super::CLFS_SCAN_MODE, preserved : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ScanLogContainers(pcxscan as _, escanmode, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn SetEndOfLog(hlog: super::HANDLE, plsnend: super::PCLFS_LSN, lpoverlapped: super::LPOVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn SetEndOfLog(hlog : super::HANDLE, plsnend : super::PCLFS_LSN, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { SetEndOfLog(hlog, plsnend, lpoverlapped as _) }
}
#[cfg(all(feature = "clfs", feature = "winnt"))]
#[inline]
pub unsafe fn SetLogArchiveMode(hlog: super::HANDLE, emode: super::CLFS_LOG_ARCHIVE_MODE) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn SetLogArchiveMode(hlog : super::HANDLE, emode : super::CLFS_LOG_ARCHIVE_MODE) -> windows_core::BOOL);
    unsafe { SetLogArchiveMode(hlog, emode) }
}
#[cfg(all(feature = "clfs", feature = "winnt"))]
#[inline]
pub unsafe fn SetLogArchiveTail(hlog: super::HANDLE, plsnarchivetail: super::PCLFS_LSN, preserved: Option<*mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn SetLogArchiveTail(hlog : super::HANDLE, plsnarchivetail : super::PCLFS_LSN, preserved : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { SetLogArchiveTail(hlog, plsnarchivetail, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn TerminateLogArchive(pvarchivecontext: CLFS_LOG_ARCHIVE_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn TerminateLogArchive(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT) -> windows_core::BOOL);
    unsafe { TerminateLogArchive(pvarchivecontext) }
}
#[inline]
pub unsafe fn TerminateReadLog(pvcursorcontext: *const core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn TerminateReadLog(pvcursorcontext : *const core::ffi::c_void) -> windows_core::BOOL);
    unsafe { TerminateReadLog(pvcursorcontext) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn TruncateLog(pvmarshal: *const core::ffi::c_void, plsnend: super::PCLFS_LSN, lpoverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn TruncateLog(pvmarshal : *const core::ffi::c_void, plsnend : super::PCLFS_LSN, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { TruncateLog(pvmarshal, plsnend, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef"))]
#[inline]
pub unsafe fn ValidateLog<P0>(pszlogfilename: P0, psalogfile: Option<super::LPSECURITY_ATTRIBUTES>, pinfobuffer: Option<super::PCLFS_INFORMATION>, pcbbuffer: super::PULONG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn ValidateLog(pszlogfilename : windows_core::PCWSTR, psalogfile : super::LPSECURITY_ATTRIBUTES, pinfobuffer : super::PCLFS_INFORMATION, pcbbuffer : super::PULONG) -> windows_core::BOOL);
    unsafe { ValidateLog(pszlogfilename.param().abi(), psalogfile.unwrap_or(core::mem::zeroed()) as _, pinfobuffer.unwrap_or(core::mem::zeroed()) as _, pcbbuffer as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn WriteLogRestartArea(pvmarshal: *mut core::ffi::c_void, pvrestartbuffer: *const core::ffi::c_void, cbrestartbuffer: u32, plsnbase: Option<super::PCLFS_LSN>, fflags: u32, pcbwritten: Option<super::PULONG>, plsnnext: Option<super::PCLFS_LSN>, poverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn WriteLogRestartArea(pvmarshal : *mut core::ffi::c_void, pvrestartbuffer : *const core::ffi::c_void, cbrestartbuffer : u32, plsnbase : super::PCLFS_LSN, fflags : u32, pcbwritten : super::PULONG, plsnnext : super::PCLFS_LSN, poverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { WriteLogRestartArea(pvmarshal as _, pvrestartbuffer, cbrestartbuffer, plsnbase.unwrap_or(core::mem::zeroed()) as _, fflags, pcbwritten.unwrap_or(core::mem::zeroed()) as _, plsnnext.unwrap_or(core::mem::zeroed()) as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
pub type CLFS_LOG_ARCHIVE_CONTEXT = *mut core::ffi::c_void;
#[cfg(all(feature = "clfs", feature = "corecrt_wstdio"))]
pub type CLFS_PRINT_RECORD_ROUTINE = Option<unsafe extern "system" fn(pstrmout: super::PFILE, frecordtype: super::CLFS_RECORD_TYPE, pvbuffer: *mut core::ffi::c_void, cbbuffer: u32) -> u32>;
pub type PCLFS_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(pvoverlapped: *mut core::ffi::c_void, ulreserved: u32)>;
pub type PCLFS_LOG_ARCHIVE_CONTEXT = *mut *mut core::ffi::c_void;
