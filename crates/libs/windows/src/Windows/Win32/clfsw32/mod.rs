#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddLogContainer<P2>(hlog: super::HANDLE, pcbcontainer: Option<*const u64>, pwszcontainerpath: P2, preserved: Option<*mut core::ffi::c_void>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn AddLogContainer(hlog : super::HANDLE, pcbcontainer : *const u64, pwszcontainerpath : windows_core::PCWSTR, preserved : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { AddLogContainer(hlog, pcbcontainer.unwrap_or(core::mem::zeroed()) as _, pwszcontainerpath.param().abi(), preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddLogContainerSet(hlog: super::HANDLE, pcbcontainer: Option<*const u64>, rgwszcontainerpath: &[windows_core::PCWSTR], preserved: Option<*mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn AddLogContainerSet(hlog : super::HANDLE, ccontainer : u16, pcbcontainer : *const u64, rgwszcontainerpath : *const windows_core::PCWSTR, preserved : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { AddLogContainerSet(hlog, rgwszcontainerpath.len().try_into().unwrap(), pcbcontainer.unwrap_or(core::mem::zeroed()) as _, rgwszcontainerpath.as_ptr(), preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn AdvanceLogBase(pvmarshal: *mut core::ffi::c_void, plsnbase: *const super::CLFS_LSN, fflags: u32, poverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn AdvanceLogBase(pvmarshal : *mut core::ffi::c_void, plsnbase : *const super::CLFS_LSN, fflags : u32, poverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { AdvanceLogBase(pvmarshal as _, plsnbase, fflags, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn AlignReservedLog(pvmarshal: *mut core::ffi::c_void, creservedrecords: u32, rgcbreservation: *const i64, pcbalignreservation: *mut i64) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn AlignReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, rgcbreservation : *const i64, pcbalignreservation : *mut i64) -> windows_core::BOOL);
    unsafe { AlignReservedLog(pvmarshal as _, creservedrecords, rgcbreservation, pcbalignreservation as _) }
}
#[inline]
pub unsafe fn AllocReservedLog(pvmarshal: *mut core::ffi::c_void, creservedrecords: u32, pcbadjustment: *mut i64) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn AllocReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, pcbadjustment : *mut i64) -> windows_core::BOOL);
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
pub unsafe fn CreateLogContainerScanContext(hlog: super::HANDLE, cfromcontainer: u32, ccontainers: u32, escanmode: super::CLFS_SCAN_MODE, pcxscan: *mut super::CLFS_SCAN_CONTEXT, poverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn CreateLogContainerScanContext(hlog : super::HANDLE, cfromcontainer : u32, ccontainers : u32, escanmode : super::CLFS_SCAN_MODE, pcxscan : *mut super::CLFS_SCAN_CONTEXT, poverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { CreateLogContainerScanContext(hlog, cfromcontainer, ccontainers, escanmode, pcxscan as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn CreateLogFile<P0>(pszlogfilename: P0, fdesiredaccess: super::ACCESS_MASK, dwsharemode: u32, psalogfile: Option<*const super::SECURITY_ATTRIBUTES>, fcreatedisposition: u32, fflagsandattributes: u32) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn CreateLogFile(pszlogfilename : windows_core::PCWSTR, fdesiredaccess : super::ACCESS_MASK, dwsharemode : u32, psalogfile : *const super::SECURITY_ATTRIBUTES, fcreatedisposition : u32, fflagsandattributes : u32) -> super::HANDLE);
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
pub unsafe fn DumpLogRecords<P0>(pwszlogfilename: P0, frecordtype: super::CLFS_RECORD_TYPE, plsnstart: Option<*const super::CLFS_LSN>, plsnend: Option<*const super::CLFS_LSN>, pstrmout: Option<*const super::FILE>, pfnprintrecord: CLFS_PRINT_RECORD_ROUTINE, pfnallocblock: super::CLFS_BLOCK_ALLOCATION, pfnfreeblock: super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext: Option<*const core::ffi::c_void>, cbblock: u32, cmaxblocks: u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn DumpLogRecords(pwszlogfilename : windows_core::PCWSTR, frecordtype : super::CLFS_RECORD_TYPE, plsnstart : *const super::CLFS_LSN, plsnend : *const super::CLFS_LSN, pstrmout : *const super::FILE, pfnprintrecord : CLFS_PRINT_RECORD_ROUTINE, pfnallocblock : super::CLFS_BLOCK_ALLOCATION, pfnfreeblock : super::CLFS_BLOCK_DEALLOCATION, pvblockalloccontext : *const core::ffi::c_void, cbblock : u32, cmaxblocks : u32) -> windows_core::BOOL);
    unsafe { DumpLogRecords(pwszlogfilename.param().abi(), frecordtype, plsnstart.unwrap_or(core::mem::zeroed()) as _, plsnend.unwrap_or(core::mem::zeroed()) as _, pstrmout.unwrap_or(core::mem::zeroed()) as _, pfnprintrecord, pfnallocblock, pfnfreeblock, pvblockalloccontext.unwrap_or(core::mem::zeroed()) as _, cbblock, cmaxblocks) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn FlushLogBuffers(pvmarshal: *const core::ffi::c_void, poverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn FlushLogBuffers(pvmarshal : *const core::ffi::c_void, poverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { FlushLogBuffers(pvmarshal, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn FlushLogToLsn(pvmarshalcontext: *const core::ffi::c_void, plsnflush: *const super::CLFS_LSN, plsnlastflushed: Option<*mut super::CLFS_LSN>, poverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn FlushLogToLsn(pvmarshalcontext : *const core::ffi::c_void, plsnflush : *const super::CLFS_LSN, plsnlastflushed : *mut super::CLFS_LSN, poverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { FlushLogToLsn(pvmarshalcontext, plsnflush, plsnlastflushed.unwrap_or(core::mem::zeroed()) as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn FreeReservedLog(pvmarshal: *mut core::ffi::c_void, creservedrecords: u32, pcbadjustment: *mut i64) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn FreeReservedLog(pvmarshal : *mut core::ffi::c_void, creservedrecords : u32, pcbadjustment : *mut i64) -> windows_core::BOOL);
    unsafe { FreeReservedLog(pvmarshal as _, creservedrecords, pcbadjustment as _) }
}
#[cfg(all(feature = "clfs", feature = "winnt"))]
#[inline]
pub unsafe fn GetLogContainerName(hlog: super::HANDLE, cidlogicalcontainer: super::CLFS_CONTAINER_ID, pwstrcontainername: windows_core::PWSTR, clencontainername: u32, pcactuallencontainername: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn GetLogContainerName(hlog : super::HANDLE, cidlogicalcontainer : super::CLFS_CONTAINER_ID, pwstrcontainername : windows_core::PWSTR, clencontainername : u32, pcactuallencontainername : *mut u32) -> windows_core::BOOL);
    unsafe { GetLogContainerName(hlog, cidlogicalcontainer, pwstrcontainername, clencontainername, pcactuallencontainername.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "winnt"))]
#[inline]
pub unsafe fn GetLogFileInformation(hlog: super::HANDLE, pinfobuffer: *mut super::CLFS_INFORMATION, cbbuffer: *mut u32) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn GetLogFileInformation(hlog : super::HANDLE, pinfobuffer : *mut super::CLFS_INFORMATION, cbbuffer : *mut u32) -> windows_core::BOOL);
    unsafe { GetLogFileInformation(hlog, pinfobuffer as _, cbbuffer as _) }
}
#[cfg(all(feature = "clfs", feature = "winnt"))]
#[inline]
pub unsafe fn GetLogIoStatistics(hlog: super::HANDLE, pvstatsbuffer: *mut core::ffi::c_void, cbstatsbuffer: u32, estatsclass: super::CLFS_IOSTATS_CLASS, pcbstatswritten: Option<*mut u32>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn GetLogIoStatistics(hlog : super::HANDLE, pvstatsbuffer : *mut core::ffi::c_void, cbstatsbuffer : u32, estatsclass : super::CLFS_IOSTATS_CLASS, pcbstatswritten : *mut u32) -> windows_core::BOOL);
    unsafe { GetLogIoStatistics(hlog, pvstatsbuffer as _, cbstatsbuffer, estatsclass, pcbstatswritten.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn GetLogReservationInfo(pvmarshal: *const core::ffi::c_void, pcbrecordnumber: *mut u32, pcbuserreservation: *mut i64, pcbcommitreservation: *mut i64) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn GetLogReservationInfo(pvmarshal : *const core::ffi::c_void, pcbrecordnumber : *mut u32, pcbuserreservation : *mut i64, pcbcommitreservation : *mut i64) -> windows_core::BOOL);
    unsafe { GetLogReservationInfo(pvmarshal, pcbrecordnumber as _, pcbuserreservation as _, pcbcommitreservation as _) }
}
#[cfg(feature = "clfs")]
#[inline]
pub unsafe fn GetNextLogArchiveExtent(pvarchivecontext: CLFS_LOG_ARCHIVE_CONTEXT, rgadextent: *mut super::CLFS_ARCHIVE_DESCRIPTOR, cdescriptors: u32, pcdescriptorsreturned: *mut u32) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn GetNextLogArchiveExtent(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT, rgadextent : *mut super::CLFS_ARCHIVE_DESCRIPTOR, cdescriptors : u32, pcdescriptorsreturned : *mut u32) -> windows_core::BOOL);
    unsafe { GetNextLogArchiveExtent(pvarchivecontext, rgadextent as _, cdescriptors, pcdescriptorsreturned as _) }
}
#[cfg(all(feature = "clfs", feature = "winnt"))]
#[inline]
pub unsafe fn PrepareLogArchive(hlog: super::HANDLE, pszbaselogfilename: &mut [u16], plsnlow: Option<*const super::CLFS_LSN>, plsnhigh: Option<*const super::CLFS_LSN>, pcactuallength: Option<*mut u32>, poffbaselogfiledata: *mut u64, pcbbaselogfilelength: *mut u64, plsnbase: *mut super::CLFS_LSN, plsnlast: *mut super::CLFS_LSN, plsncurrentarchivetail: *mut super::CLFS_LSN, ppvarchivecontext: *mut *mut core::ffi::c_void) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn PrepareLogArchive(hlog : super::HANDLE, pszbaselogfilename : windows_core::PWSTR, clen : u32, plsnlow : *const super::CLFS_LSN, plsnhigh : *const super::CLFS_LSN, pcactuallength : *mut u32, poffbaselogfiledata : *mut u64, pcbbaselogfilelength : *mut u64, plsnbase : *mut super::CLFS_LSN, plsnlast : *mut super::CLFS_LSN, plsncurrentarchivetail : *mut super::CLFS_LSN, ppvarchivecontext : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { PrepareLogArchive(hlog, core::mem::transmute(pszbaselogfilename.as_mut_ptr()), pszbaselogfilename.len().try_into().unwrap(), plsnlow.unwrap_or(core::mem::zeroed()) as _, plsnhigh.unwrap_or(core::mem::zeroed()) as _, pcactuallength.unwrap_or(core::mem::zeroed()) as _, poffbaselogfiledata as _, pcbbaselogfilelength as _, plsnbase as _, plsnlast as _, plsncurrentarchivetail as _, ppvarchivecontext as _) }
}
#[inline]
pub unsafe fn ReadLogArchiveMetadata(pvarchivecontext: CLFS_LOG_ARCHIVE_CONTEXT, cboffset: u32, cbbytestoread: u32, pbreadbuffer: *mut u8, pcbbytesread: *mut u32) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadLogArchiveMetadata(pvarchivecontext : CLFS_LOG_ARCHIVE_CONTEXT, cboffset : u32, cbbytestoread : u32, pbreadbuffer : *mut u8, pcbbytesread : *mut u32) -> windows_core::BOOL);
    unsafe { ReadLogArchiveMetadata(pvarchivecontext, cboffset, cbbytestoread, pbreadbuffer as _, pcbbytesread as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn ReadLogRecord(pvmarshal: *const core::ffi::c_void, plsnfirst: *const super::CLFS_LSN, econtextmode: super::CLFS_CONTEXT_MODE, ppvreadbuffer: *mut *mut core::ffi::c_void, pcbreadbuffer: *mut u32, perecordtype: *mut super::CLS_RECORD_TYPE, plsnundonext: *mut super::CLFS_LSN, plsnprevious: *mut super::CLFS_LSN, ppvreadcontext: *mut *mut core::ffi::c_void, poverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadLogRecord(pvmarshal : *const core::ffi::c_void, plsnfirst : *const super::CLFS_LSN, econtextmode : super::CLFS_CONTEXT_MODE, ppvreadbuffer : *mut *mut core::ffi::c_void, pcbreadbuffer : *mut u32, perecordtype : *mut super::CLS_RECORD_TYPE, plsnundonext : *mut super::CLFS_LSN, plsnprevious : *mut super::CLFS_LSN, ppvreadcontext : *mut *mut core::ffi::c_void, poverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadLogRecord(pvmarshal, plsnfirst, econtextmode, ppvreadbuffer as _, pcbreadbuffer as _, perecordtype as _, plsnundonext as _, plsnprevious as _, ppvreadcontext as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn ReadLogRestartArea(pvmarshal: *const core::ffi::c_void, ppvrestartbuffer: *mut *mut core::ffi::c_void, pcbrestartbuffer: *mut u32, plsn: *mut super::CLFS_LSN, ppvcontext: *mut *mut core::ffi::c_void, poverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadLogRestartArea(pvmarshal : *const core::ffi::c_void, ppvrestartbuffer : *mut *mut core::ffi::c_void, pcbrestartbuffer : *mut u32, plsn : *mut super::CLFS_LSN, ppvcontext : *mut *mut core::ffi::c_void, poverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadLogRestartArea(pvmarshal, ppvrestartbuffer as _, pcbrestartbuffer as _, plsn as _, ppvcontext as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn ReadNextLogRecord(pvreadcontext: *mut core::ffi::c_void, ppvbuffer: *mut *mut core::ffi::c_void, pcbbuffer: *mut u32, perecordtype: *mut super::CLS_RECORD_TYPE, plsnuser: Option<*const super::CLFS_LSN>, plsnundonext: *mut super::CLFS_LSN, plsnprevious: *mut super::CLFS_LSN, plsnrecord: *mut super::CLFS_LSN, poverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadNextLogRecord(pvreadcontext : *mut core::ffi::c_void, ppvbuffer : *mut *mut core::ffi::c_void, pcbbuffer : *mut u32, perecordtype : *mut super::CLS_RECORD_TYPE, plsnuser : *const super::CLFS_LSN, plsnundonext : *mut super::CLFS_LSN, plsnprevious : *mut super::CLFS_LSN, plsnrecord : *mut super::CLFS_LSN, poverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { ReadNextLogRecord(pvreadcontext as _, ppvbuffer as _, pcbbuffer as _, perecordtype as _, plsnuser.unwrap_or(core::mem::zeroed()) as _, plsnundonext as _, plsnprevious as _, plsnrecord as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn ReadPreviousLogRestartArea(pvreadcontext: *const core::ffi::c_void, ppvrestartbuffer: *mut *mut core::ffi::c_void, pcbrestartbuffer: *mut u32, plsnrestart: *mut super::CLFS_LSN, poverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReadPreviousLogRestartArea(pvreadcontext : *const core::ffi::c_void, ppvrestartbuffer : *mut *mut core::ffi::c_void, pcbrestartbuffer : *mut u32, plsnrestart : *mut super::CLFS_LSN, poverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
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
pub unsafe fn ReserveAndAppendLog(pvmarshal: *const core::ffi::c_void, rgwriteentries: Option<*const super::CLFS_WRITE_ENTRY>, cwriteentries: u32, plsnundonext: Option<*const super::CLFS_LSN>, plsnprevious: Option<*const super::CLFS_LSN>, creserverecords: u32, rgcbreservation: Option<*mut i64>, fflags: u32, plsn: Option<*mut super::CLFS_LSN>, poverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReserveAndAppendLog(pvmarshal : *const core::ffi::c_void, rgwriteentries : *const super::CLFS_WRITE_ENTRY, cwriteentries : u32, plsnundonext : *const super::CLFS_LSN, plsnprevious : *const super::CLFS_LSN, creserverecords : u32, rgcbreservation : *mut i64, fflags : u32, plsn : *mut super::CLFS_LSN, poverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { ReserveAndAppendLog(pvmarshal, rgwriteentries.unwrap_or(core::mem::zeroed()) as _, cwriteentries, plsnundonext.unwrap_or(core::mem::zeroed()) as _, plsnprevious.unwrap_or(core::mem::zeroed()) as _, creserverecords, rgcbreservation.unwrap_or(core::mem::zeroed()) as _, fflags, plsn.unwrap_or(core::mem::zeroed()) as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn ReserveAndAppendLogAligned(pvmarshal: *const core::ffi::c_void, rgwriteentries: Option<*const super::CLFS_WRITE_ENTRY>, cwriteentries: u32, cbentryalignment: u32, plsnundonext: Option<*const super::CLFS_LSN>, plsnprevious: Option<*const super::CLFS_LSN>, creserverecords: u32, rgcbreservation: Option<*mut i64>, fflags: u32, plsn: Option<*mut super::CLFS_LSN>, poverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ReserveAndAppendLogAligned(pvmarshal : *const core::ffi::c_void, rgwriteentries : *const super::CLFS_WRITE_ENTRY, cwriteentries : u32, cbentryalignment : u32, plsnundonext : *const super::CLFS_LSN, plsnprevious : *const super::CLFS_LSN, creserverecords : u32, rgcbreservation : *mut i64, fflags : u32, plsn : *mut super::CLFS_LSN, poverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { ReserveAndAppendLogAligned(pvmarshal, rgwriteentries.unwrap_or(core::mem::zeroed()) as _, cwriteentries, cbentryalignment, plsnundonext.unwrap_or(core::mem::zeroed()) as _, plsnprevious.unwrap_or(core::mem::zeroed()) as _, creserverecords, rgcbreservation.unwrap_or(core::mem::zeroed()) as _, fflags, plsn.unwrap_or(core::mem::zeroed()) as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "winnt"))]
#[inline]
pub unsafe fn ScanLogContainers(pcxscan: *mut super::CLFS_SCAN_CONTEXT, escanmode: super::CLFS_SCAN_MODE, preserved: Option<*mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn ScanLogContainers(pcxscan : *mut super::CLFS_SCAN_CONTEXT, escanmode : super::CLFS_SCAN_MODE, preserved : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { ScanLogContainers(pcxscan as _, escanmode, preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn SetEndOfLog(hlog: super::HANDLE, plsnend: *const super::CLFS_LSN, lpoverlapped: *mut super::OVERLAPPED) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn SetEndOfLog(hlog : super::HANDLE, plsnend : *const super::CLFS_LSN, lpoverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
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
pub unsafe fn SetLogArchiveTail(hlog: super::HANDLE, plsnarchivetail: *const super::CLFS_LSN, preserved: Option<*mut core::ffi::c_void>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn SetLogArchiveTail(hlog : super::HANDLE, plsnarchivetail : *const super::CLFS_LSN, preserved : *mut core::ffi::c_void) -> windows_core::BOOL);
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
pub unsafe fn TruncateLog(pvmarshal: *const core::ffi::c_void, plsnend: *const super::CLFS_LSN, lpoverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn TruncateLog(pvmarshal : *const core::ffi::c_void, plsnend : *const super::CLFS_LSN, lpoverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { TruncateLog(pvmarshal, plsnend, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase"))]
#[inline]
pub unsafe fn ValidateLog<P0>(pszlogfilename: P0, psalogfile: Option<*const super::SECURITY_ATTRIBUTES>, pinfobuffer: Option<*mut super::CLFS_INFORMATION>, pcbbuffer: *mut u32) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("clfsw32.dll" "system" fn ValidateLog(pszlogfilename : windows_core::PCWSTR, psalogfile : *const super::SECURITY_ATTRIBUTES, pinfobuffer : *mut super::CLFS_INFORMATION, pcbbuffer : *mut u32) -> windows_core::BOOL);
    unsafe { ValidateLog(pszlogfilename.param().abi(), psalogfile.unwrap_or(core::mem::zeroed()) as _, pinfobuffer.unwrap_or(core::mem::zeroed()) as _, pcbbuffer as _) }
}
#[cfg(all(feature = "clfs", feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn WriteLogRestartArea(pvmarshal: *mut core::ffi::c_void, pvrestartbuffer: *const core::ffi::c_void, cbrestartbuffer: u32, plsnbase: Option<*const super::CLFS_LSN>, fflags: u32, pcbwritten: Option<*mut u32>, plsnnext: Option<*mut super::CLFS_LSN>, poverlapped: Option<*mut super::OVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("clfsw32.dll" "system" fn WriteLogRestartArea(pvmarshal : *mut core::ffi::c_void, pvrestartbuffer : *const core::ffi::c_void, cbrestartbuffer : u32, plsnbase : *const super::CLFS_LSN, fflags : u32, pcbwritten : *mut u32, plsnnext : *mut super::CLFS_LSN, poverlapped : *mut super::OVERLAPPED) -> windows_core::BOOL);
    unsafe { WriteLogRestartArea(pvmarshal as _, pvrestartbuffer, cbrestartbuffer, plsnbase.unwrap_or(core::mem::zeroed()) as _, fflags, pcbwritten.unwrap_or(core::mem::zeroed()) as _, plsnnext.unwrap_or(core::mem::zeroed()) as _, poverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CLFS_LOG_ARCHIVE_CONTEXT(pub *mut core::ffi::c_void);
#[cfg(all(feature = "clfs", feature = "corecrt_wstdio"))]
pub type CLFS_PRINT_RECORD_ROUTINE = Option<unsafe extern "system" fn(pstrmout: *mut super::FILE, frecordtype: super::CLFS_RECORD_TYPE, pvbuffer: *mut core::ffi::c_void, cbbuffer: u32) -> u32>;
pub type PCLFS_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(pvoverlapped: *mut core::ffi::c_void, ulreserved: u32)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PCLFS_LOG_ARCHIVE_CONTEXT(pub *mut *mut core::ffi::c_void);
