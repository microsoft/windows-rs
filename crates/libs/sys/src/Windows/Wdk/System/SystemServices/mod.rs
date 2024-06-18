#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsAddLogContainer(plfolog : *const super::super::Foundation:: FILE_OBJECT, pcbcontainer : *const u64, puszcontainerpath : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsAddLogContainerSet(plfolog : *const super::super::Foundation:: FILE_OBJECT, ccontainers : u16, pcbcontainer : *const u64, rguszcontainerpath : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsAdvanceLogBase(pvmarshalcontext : *mut core::ffi::c_void, plsnbase : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN, fflags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("clfs.sys" "system" fn ClfsAlignReservedLog(pvmarshalcontext : *const core::ffi::c_void, crecords : u32, rgcbreservation : *const i64, pcbalignreservation : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("clfs.sys" "system" fn ClfsAllocReservedLog(pvmarshalcontext : *const core::ffi::c_void, crecords : u32, pcbadjustment : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsCloseAndResetLogFile(plfolog : *const super::super::Foundation:: FILE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsCloseLogFileObject(plfolog : *const super::super::Foundation:: FILE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsCreateLogFile(pplfolog : *mut *mut super::super::Foundation:: FILE_OBJECT, puszlogfilename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, fdesiredaccess : u32, dwsharemode : u32, psdlogfile : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, fcreatedisposition : u32, fcreateoptions : u32, fflagsandattributes : u32, flogoptionflag : u32, pvcontext : *const core::ffi::c_void, cbcontext : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsCreateMarshallingArea(plfolog : *const super::super::Foundation:: FILE_OBJECT, epooltype : super::super::Foundation:: POOL_TYPE, pfnallocbuffer : PALLOCATE_FUNCTION, pfnfreebuffer : super::super::Foundation:: PFREE_FUNCTION, cbmarshallingbuffer : u32, cmaxwritebuffers : u32, cmaxreadbuffers : u32, ppvmarshalcontext : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsCreateMarshallingAreaEx(plfolog : *const super::super::Foundation:: FILE_OBJECT, epooltype : super::super::Foundation:: POOL_TYPE, pfnallocbuffer : PALLOCATE_FUNCTION, pfnfreebuffer : super::super::Foundation:: PFREE_FUNCTION, cbmarshallingbuffer : u32, cmaxwritebuffers : u32, cmaxreadbuffers : u32, calignmentsize : u32, fflags : u64, ppvmarshalcontext : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsCreateScanContext(plfolog : *const super::super::Foundation:: FILE_OBJECT, cfromcontainer : u32, ccontainers : u32, escanmode : u8, pcxscan : *mut super::super::super::Win32::Storage::FileSystem:: CLS_SCAN_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsDeleteLogByPointer(plfolog : *const super::super::Foundation:: FILE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("clfs.sys" "system" fn ClfsDeleteLogFile(puszlogfilename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, pvreserved : *const core::ffi::c_void, flogoptionflag : u32, pvcontext : *const core::ffi::c_void, cbcontext : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("clfs.sys" "system" fn ClfsDeleteMarshallingArea(pvmarshalcontext : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsEarlierLsn(plsn : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Storage::FileSystem:: CLS_LSN);
windows_targets::link!("clfs.sys" "system" fn ClfsFinalize());
windows_targets::link!("clfs.sys" "system" fn ClfsFlushBuffers(pvmarshalcontext : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsFlushToLsn(pvmarshalcontext : *const core::ffi::c_void, plsnflush : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN, plsnlastflushed : *mut super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("clfs.sys" "system" fn ClfsFreeReservedLog(pvmarshalcontext : *const core::ffi::c_void, crecords : u32, pcbadjustment : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsGetContainerName(plfolog : *const super::super::Foundation:: FILE_OBJECT, cidlogicalcontainer : u32, puszcontainername : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, pcactuallencontainername : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsGetIoStatistics(plfolog : *const super::super::Foundation:: FILE_OBJECT, pvstatsbuffer : *mut core::ffi::c_void, cbstatsbuffer : u32, estatsclass : super::super::super::Win32::Storage::FileSystem:: CLFS_IOSTATS_CLASS, pcbstatswritten : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsGetLogFileInformation(plfolog : *const super::super::Foundation:: FILE_OBJECT, pinfobuffer : *mut super::super::super::Win32::Storage::FileSystem:: CLS_INFORMATION, pcbinfobuffer : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("clfs.sys" "system" fn ClfsInitialize() -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsLaterLsn(plsn : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Storage::FileSystem:: CLS_LSN);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsLsnBlockOffset(plsn : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> u32);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsLsnContainer(plsn : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> u32);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsLsnCreate(cidcontainer : u32, offblock : u32, crecord : u32) -> super::super::super::Win32::Storage::FileSystem:: CLS_LSN);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsLsnDifference(plsnstart : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN, plsnfinish : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN, cbcontainer : u32, cbmaxblock : u32, pcbdifference : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsLsnEqual(plsn1 : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN, plsn2 : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsLsnGreater(plsn1 : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN, plsn2 : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsLsnInvalid(plsn : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsLsnLess(plsn1 : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN, plsn2 : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsLsnNull(plsn : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsLsnRecordSequence(plsn : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> u32);
windows_targets::link!("clfs.sys" "system" fn ClfsMgmtDeregisterManagedClient(clientcookie : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("clfs.sys" "system" fn ClfsMgmtHandleLogFileFull(client : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsMgmtInstallPolicy(logfile : *const super::super::Foundation:: FILE_OBJECT, policy : *const super::super::super::Win32::Storage::FileSystem:: CLFS_MGMT_POLICY, policylength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsMgmtQueryPolicy(logfile : *const super::super::Foundation:: FILE_OBJECT, policytype : super::super::super::Win32::Storage::FileSystem:: CLFS_MGMT_POLICY_TYPE, policy : *mut super::super::super::Win32::Storage::FileSystem:: CLFS_MGMT_POLICY, policylength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsMgmtRegisterManagedClient(logfile : *const super::super::Foundation:: FILE_OBJECT, registrationdata : *const CLFS_MGMT_CLIENT_REGISTRATION, clientcookie : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsMgmtRemovePolicy(logfile : *const super::super::Foundation:: FILE_OBJECT, policytype : super::super::super::Win32::Storage::FileSystem:: CLFS_MGMT_POLICY_TYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsMgmtSetLogFileSize(logfile : *const super::super::Foundation:: FILE_OBJECT, newsizeincontainers : *const u64, resultingsizeincontainers : *mut u64, completionroutine : PCLFS_SET_LOG_SIZE_COMPLETE_CALLBACK, completionroutinedata : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsMgmtSetLogFileSizeAsClient(logfile : *const super::super::Foundation:: FILE_OBJECT, clientcookie : *const *const core::ffi::c_void, newsizeincontainers : *const u64, resultingsizeincontainers : *mut u64, completionroutine : PCLFS_SET_LOG_SIZE_COMPLETE_CALLBACK, completionroutinedata : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("clfs.sys" "system" fn ClfsMgmtTailAdvanceFailure(client : *const core::ffi::c_void, reason : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsQueryLogFileInformation(plfolog : *const super::super::Foundation:: FILE_OBJECT, einformationclass : super::super::super::Win32::Storage::FileSystem:: CLS_LOG_INFORMATION_CLASS, pinfoinputbuffer : *const core::ffi::c_void, cbinfoinputbuffer : u32, pinfobuffer : *mut core::ffi::c_void, pcbinfobuffer : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsReadLogRecord(pvmarshalcontext : *const core::ffi::c_void, plsnfirst : *mut super::super::super::Win32::Storage::FileSystem:: CLS_LSN, pecontextmode : super::super::super::Win32::Storage::FileSystem:: CLFS_CONTEXT_MODE, ppvreadbuffer : *mut *mut core::ffi::c_void, pcbreadbuffer : *mut u32, perecordtype : *mut u8, plsnundonext : *mut super::super::super::Win32::Storage::FileSystem:: CLS_LSN, plsnprevious : *mut super::super::super::Win32::Storage::FileSystem:: CLS_LSN, ppvreadcontext : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsReadNextLogRecord(pvreadcontext : *mut core::ffi::c_void, ppvbuffer : *mut *mut core::ffi::c_void, pcbbuffer : *mut u32, perecordtype : *mut u8, plsnuser : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN, plsnundonext : *mut super::super::super::Win32::Storage::FileSystem:: CLS_LSN, plsnprevious : *mut super::super::super::Win32::Storage::FileSystem:: CLS_LSN, plsnrecord : *mut super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsReadPreviousRestartArea(pvreadcontext : *const core::ffi::c_void, ppvrestartbuffer : *mut *mut core::ffi::c_void, pcbrestartbuffer : *mut u32, plsnrestart : *mut super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsReadRestartArea(pvmarshalcontext : *mut core::ffi::c_void, ppvrestartbuffer : *mut *mut core::ffi::c_void, pcbrestartbuffer : *mut u32, plsn : *mut super::super::super::Win32::Storage::FileSystem:: CLS_LSN, ppvreadcontext : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsRemoveLogContainer(plfolog : *const super::super::Foundation:: FILE_OBJECT, puszcontainerpath : *const super::super::super::Win32::Foundation:: UNICODE_STRING, fforce : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsRemoveLogContainerSet(plfolog : *const super::super::Foundation:: FILE_OBJECT, ccontainers : u16, rgwszcontainerpath : *const super::super::super::Win32::Foundation:: UNICODE_STRING, fforce : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsReserveAndAppendLog(pvmarshalcontext : *const core::ffi::c_void, rgwriteentries : *const super::super::super::Win32::Storage::FileSystem:: CLS_WRITE_ENTRY, cwriteentries : u32, plsnundonext : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN, plsnprevious : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN, creserverecords : u32, rgcbreservation : *mut i64, fflags : u32, plsn : *mut super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsReserveAndAppendLogAligned(pvmarshalcontext : *const core::ffi::c_void, rgwriteentries : *const super::super::super::Win32::Storage::FileSystem:: CLS_WRITE_ENTRY, cwriteentries : u32, cbentryalignment : u32, plsnundonext : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN, plsnprevious : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN, creserverecords : u32, rgcbreservation : *mut i64, fflags : u32, plsn : *mut super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsScanLogContainers(pcxscan : *mut super::super::super::Win32::Storage::FileSystem:: CLS_SCAN_CONTEXT, escanmode : u8) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsSetArchiveTail(plfolog : *const super::super::Foundation:: FILE_OBJECT, plsnarchivetail : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsSetEndOfLog(plfolog : *const super::super::Foundation:: FILE_OBJECT, plsnend : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("clfs.sys" "system" fn ClfsSetLogFileInformation(plfolog : *const super::super::Foundation:: FILE_OBJECT, einformationclass : super::super::super::Win32::Storage::FileSystem:: CLS_LOG_INFORMATION_CLASS, pinfobuffer : *const core::ffi::c_void, cbbuffer : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("clfs.sys" "system" fn ClfsTerminateReadLog(pvcursorcontext : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("clfs.sys" "system" fn ClfsWriteRestartArea(pvmarshalcontext : *mut core::ffi::c_void, pvrestartbuffer : *const core::ffi::c_void, cbrestartbuffer : u32, plsnbase : *const super::super::super::Win32::Storage::FileSystem:: CLS_LSN, fflags : u32, pcbwritten : *mut u32, plsnnext : *mut super::super::super::Win32::Storage::FileSystem:: CLS_LSN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn CmCallbackGetKeyObjectID(cookie : *const i64, object : *const core::ffi::c_void, objectid : *mut usize, objectname : *mut *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn CmCallbackGetKeyObjectIDEx(cookie : *const i64, object : *const core::ffi::c_void, objectid : *mut usize, objectname : *mut *mut super::super::super::Win32::Foundation:: UNICODE_STRING, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn CmCallbackReleaseKeyObjectIDEx(objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING));
windows_targets::link!("ntoskrnl.exe" "system" fn CmGetBoundTransaction(cookie : *const i64, object : *const core::ffi::c_void) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn CmGetCallbackVersion(major : *mut u32, minor : *mut u32));
windows_targets::link!("ntoskrnl.exe" "system" fn CmRegisterCallback(function : PEX_CALLBACK_FUNCTION, context : *const core::ffi::c_void, cookie : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn CmRegisterCallbackEx(function : PEX_CALLBACK_FUNCTION, altitude : *const super::super::super::Win32::Foundation:: UNICODE_STRING, driver : *const core::ffi::c_void, context : *const core::ffi::c_void, cookie : *mut i64, reserved : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn CmSetCallbackObjectContext(object : *mut core::ffi::c_void, cookie : *const i64, newcontext : *const core::ffi::c_void, oldcontext : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn CmUnRegisterCallback(cookie : i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn DbgBreakPointWithStatus(status : u32));
windows_targets::link!("ntdll.dll" "cdecl" fn DbgPrint(format : windows_sys::core::PCSTR, ...) -> u32);
windows_targets::link!("ntdll.dll" "cdecl" fn DbgPrintEx(componentid : u32, level : u32, format : windows_sys::core::PCSTR, ...) -> u32);
windows_targets::link!("ntdll.dll" "cdecl" fn DbgPrintReturnControlC(format : windows_sys::core::PCSTR, ...) -> u32);
windows_targets::link!("ntdll.dll" "system" fn DbgPrompt(prompt : windows_sys::core::PCSTR, response : windows_sys::core::PSTR, length : u32) -> u32);
windows_targets::link!("ntdll.dll" "system" fn DbgQueryDebugFilterState(componentid : u32, level : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn DbgSetDebugFilterState(componentid : u32, level : u32, state : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntoskrnl.exe" "system" fn DbgSetDebugPrintCallback(debugprintcallback : PDEBUG_PRINT_CALLBACK, enable : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn EtwActivityIdControl(controlcode : u32, activityid : *mut windows_sys::core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
windows_targets::link!("ntdll.dll" "system" fn EtwEventEnabled(reghandle : super::super::super::Win32::System::Diagnostics::Etw:: REGHANDLE, eventdescriptor : *const super::super::super::Win32::System::Diagnostics::Etw:: EVENT_DESCRIPTOR) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
windows_targets::link!("ntoskrnl.exe" "system" fn EtwProviderEnabled(reghandle : super::super::super::Win32::System::Diagnostics::Etw:: REGHANDLE, level : u8, keyword : u64) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn EtwRegister(providerid : *const windows_sys::core::GUID, enablecallback : PETWENABLECALLBACK, callbackcontext : *const core::ffi::c_void, reghandle : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
windows_targets::link!("ntoskrnl.exe" "system" fn EtwSetInformation(reghandle : super::super::super::Win32::System::Diagnostics::Etw:: REGHANDLE, informationclass : super::super::super::Win32::System::Diagnostics::Etw:: EVENT_INFO_CLASS, eventinformation : *const core::ffi::c_void, informationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
windows_targets::link!("ntoskrnl.exe" "system" fn EtwUnregister(reghandle : super::super::super::Win32::System::Diagnostics::Etw:: REGHANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
windows_targets::link!("ntoskrnl.exe" "system" fn EtwWrite(reghandle : super::super::super::Win32::System::Diagnostics::Etw:: REGHANDLE, eventdescriptor : *const super::super::super::Win32::System::Diagnostics::Etw:: EVENT_DESCRIPTOR, activityid : *const windows_sys::core::GUID, userdatacount : u32, userdata : *const super::super::super::Win32::System::Diagnostics::Etw:: EVENT_DATA_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
windows_targets::link!("ntoskrnl.exe" "system" fn EtwWriteEx(reghandle : super::super::super::Win32::System::Diagnostics::Etw:: REGHANDLE, eventdescriptor : *const super::super::super::Win32::System::Diagnostics::Etw:: EVENT_DESCRIPTOR, filter : u64, flags : u32, activityid : *const windows_sys::core::GUID, relatedactivityid : *const windows_sys::core::GUID, userdatacount : u32, userdata : *const super::super::super::Win32::System::Diagnostics::Etw:: EVENT_DATA_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
windows_targets::link!("ntoskrnl.exe" "system" fn EtwWriteString(reghandle : super::super::super::Win32::System::Diagnostics::Etw:: REGHANDLE, level : u8, keyword : u64, activityid : *const windows_sys::core::GUID, string : windows_sys::core::PCWSTR) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
windows_targets::link!("ntoskrnl.exe" "system" fn EtwWriteTransfer(reghandle : super::super::super::Win32::System::Diagnostics::Etw:: REGHANDLE, eventdescriptor : *const super::super::super::Win32::System::Diagnostics::Etw:: EVENT_DESCRIPTOR, activityid : *const windows_sys::core::GUID, relatedactivityid : *const windows_sys::core::GUID, userdatacount : u32, userdata : *const super::super::super::Win32::System::Diagnostics::Etw:: EVENT_DATA_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquireFastMutex(fastmutex : *mut super::super::Foundation:: FAST_MUTEX));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquireFastMutexUnsafe(fastmutex : *mut super::super::Foundation:: FAST_MUTEX));
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquirePushLockExclusiveEx(pushlock : *mut usize, flags : u32));
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquirePushLockSharedEx(pushlock : *mut usize, flags : u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquireResourceExclusiveLite(resource : *mut super::super::Foundation:: ERESOURCE, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquireResourceSharedLite(resource : *mut super::super::Foundation:: ERESOURCE, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquireRundownProtection(runref : *mut EX_RUNDOWN_REF) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquireRundownProtectionCacheAware(runrefcacheaware : super::super::Foundation:: PEX_RUNDOWN_REF_CACHE_AWARE) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquireRundownProtectionCacheAwareEx(runrefcacheaware : super::super::Foundation:: PEX_RUNDOWN_REF_CACHE_AWARE, count : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquireRundownProtectionEx(runref : *mut EX_RUNDOWN_REF, count : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquireSharedStarveExclusive(resource : *mut super::super::Foundation:: ERESOURCE, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquireSharedWaitForExclusive(resource : *mut super::super::Foundation:: ERESOURCE, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquireSpinLockExclusive(spinlock : *mut i32) -> u8);
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquireSpinLockExclusiveAtDpcLevel(spinlock : *mut i32));
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquireSpinLockShared(spinlock : *mut i32) -> u8);
windows_targets::link!("ntoskrnl.exe" "system" fn ExAcquireSpinLockSharedAtDpcLevel(spinlock : *mut i32));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAllocateCacheAwareRundownProtection(pooltype : super::super::Foundation:: POOL_TYPE, pooltag : u32) -> super::super::Foundation:: PEX_RUNDOWN_REF_CACHE_AWARE);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAllocatePool(pooltype : super::super::Foundation:: POOL_TYPE, numberofbytes : usize) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn ExAllocatePool2(flags : u64, numberofbytes : usize, tag : u32) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn ExAllocatePool3(flags : u64, numberofbytes : usize, tag : u32, extendedparameters : *const POOL_EXTENDED_PARAMETER, extendedparameterscount : u32) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAllocatePoolWithQuota(pooltype : super::super::Foundation:: POOL_TYPE, numberofbytes : usize) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAllocatePoolWithQuotaTag(pooltype : super::super::Foundation:: POOL_TYPE, numberofbytes : usize, tag : u32) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAllocatePoolWithTag(pooltype : super::super::Foundation:: POOL_TYPE, numberofbytes : usize, tag : u32) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAllocatePoolWithTagPriority(pooltype : super::super::Foundation:: POOL_TYPE, numberofbytes : usize, tag : u32, priority : EX_POOL_PRIORITY) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExAllocateTimer(callback : PEXT_CALLBACK, callbackcontext : *const core::ffi::c_void, attributes : u32) -> super::super::Foundation:: PEX_TIMER);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExCancelTimer(timer : super::super::Foundation:: PEX_TIMER, parameters : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExCleanupRundownProtectionCacheAware(runrefcacheaware : super::super::Foundation:: PEX_RUNDOWN_REF_CACHE_AWARE));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExConvertExclusiveToSharedLite(resource : *mut super::super::Foundation:: ERESOURCE));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExCreateCallback(callbackobject : *mut super::super::Foundation:: PCALLBACK_OBJECT, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, create : super::super::super::Win32::Foundation:: BOOLEAN, allowmultiplecallbacks : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn ExCreatePool(flags : u32, tag : usize, params : *const POOL_CREATE_EXTENDED_PARAMS, poolhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExDeleteResourceLite(resource : *mut super::super::Foundation:: ERESOURCE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExDeleteTimer(timer : super::super::Foundation:: PEX_TIMER, cancel : super::super::super::Win32::Foundation:: BOOLEAN, wait : super::super::super::Win32::Foundation:: BOOLEAN, parameters : *const EXT_DELETE_PARAMETERS) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn ExDestroyPool(poolhandle : super::super::super::Win32::Foundation:: HANDLE));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExEnterCriticalRegionAndAcquireResourceExclusive(resource : *mut super::super::Foundation:: ERESOURCE) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExEnterCriticalRegionAndAcquireResourceShared(resource : *mut super::super::Foundation:: ERESOURCE) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExEnterCriticalRegionAndAcquireSharedWaitForExclusive(resource : *mut super::super::Foundation:: ERESOURCE) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn ExEnumerateSystemFirmwareTables(firmwaretableprovidersignature : u32, firmwaretablebuffer : *mut core::ffi::c_void, bufferlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExExtendZone(zone : *mut ZONE_HEADER, segment : *mut core::ffi::c_void, segmentsize : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExFreeCacheAwareRundownProtection(runrefcacheaware : super::super::Foundation:: PEX_RUNDOWN_REF_CACHE_AWARE));
windows_targets::link!("ntoskrnl.exe" "system" fn ExFreePool(p : *mut core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn ExFreePool2(p : *mut core::ffi::c_void, tag : u32, extendedparameters : *const POOL_EXTENDED_PARAMETER, extendedparameterscount : u32));
windows_targets::link!("ntoskrnl.exe" "system" fn ExFreePoolWithTag(p : *mut core::ffi::c_void, tag : u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExGetExclusiveWaiterCount(resource : *const super::super::Foundation:: ERESOURCE) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn ExGetFirmwareEnvironmentVariable(variablename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, vendorguid : *const windows_sys::core::GUID, value : *mut core::ffi::c_void, valuelength : *mut u32, attributes : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExGetFirmwareType() -> super::super::super::Win32::System::SystemInformation:: FIRMWARE_TYPE);
windows_targets::link!("ntoskrnl.exe" "system" fn ExGetPreviousMode() -> i8);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExGetSharedWaiterCount(resource : *const super::super::Foundation:: ERESOURCE) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn ExGetSystemFirmwareTable(firmwaretableprovidersignature : u32, firmwaretableid : u32, firmwaretablebuffer : *mut core::ffi::c_void, bufferlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn ExInitializePushLock(pushlock : *mut usize));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExInitializeResourceLite(resource : *mut super::super::Foundation:: ERESOURCE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn ExInitializeRundownProtection(runref : *mut EX_RUNDOWN_REF));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExInitializeRundownProtectionCacheAware(runrefcacheaware : super::super::Foundation:: PEX_RUNDOWN_REF_CACHE_AWARE, runrefsize : usize));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExInitializeRundownProtectionCacheAwareEx(runrefcacheaware : super::super::Foundation:: PEX_RUNDOWN_REF_CACHE_AWARE, flags : u32));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExInitializeZone(zone : *mut ZONE_HEADER, blocksize : u32, initialsegment : *mut core::ffi::c_void, initialsegmentsize : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn ExInterlockedAddLargeInteger(addend : *mut i64, increment : i64, lock : *mut usize) -> i64);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExInterlockedExtendZone(zone : *mut ZONE_HEADER, segment : *mut core::ffi::c_void, segmentsize : u32, lock : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn ExIsManufacturingModeEnabled() -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn ExIsProcessorFeaturePresent(processorfeature : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExIsResourceAcquiredExclusiveLite(resource : *const super::super::Foundation:: ERESOURCE) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExIsResourceAcquiredSharedLite(resource : *const super::super::Foundation:: ERESOURCE) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn ExIsSoftBoot() -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn ExLocalTimeToSystemTime(localtime : *const i64, systemtime : *mut i64));
windows_targets::link!("ntoskrnl.exe" "system" fn ExNotifyCallback(callbackobject : *const core::ffi::c_void, argument1 : *const core::ffi::c_void, argument2 : *const core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn ExQueryTimerResolution(maximumtime : *mut u32, minimumtime : *mut u32, currenttime : *mut u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExQueueWorkItem(workitem : *mut super::super::Foundation:: WORK_QUEUE_ITEM, queuetype : WORK_QUEUE_TYPE));
windows_targets::link!("ntoskrnl.exe" "system" fn ExRaiseAccessViolation());
windows_targets::link!("ntoskrnl.exe" "system" fn ExRaiseDatatypeMisalignment());
windows_targets::link!("ntoskrnl.exe" "system" fn ExRaiseStatus(status : super::super::super::Win32::Foundation:: NTSTATUS));
windows_targets::link!("ntoskrnl.exe" "system" fn ExReInitializeRundownProtection(runref : *mut EX_RUNDOWN_REF));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExReInitializeRundownProtectionCacheAware(runrefcacheaware : super::super::Foundation:: PEX_RUNDOWN_REF_CACHE_AWARE));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExRegisterCallback(callbackobject : super::super::Foundation:: PCALLBACK_OBJECT, callbackfunction : PCALLBACK_FUNCTION, callbackcontext : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExReinitializeResourceLite(resource : *mut super::super::Foundation:: ERESOURCE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleaseFastMutex(fastmutex : *mut super::super::Foundation:: FAST_MUTEX));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleaseFastMutexUnsafe(fastmutex : *mut super::super::Foundation:: FAST_MUTEX));
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleasePushLockExclusiveEx(pushlock : *mut usize, flags : u32));
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleasePushLockSharedEx(pushlock : *mut usize, flags : u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleaseResourceAndLeaveCriticalRegion(resource : *mut super::super::Foundation:: ERESOURCE));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleaseResourceForThreadLite(resource : *mut super::super::Foundation:: ERESOURCE, resourcethreadid : usize));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleaseResourceLite(resource : *mut super::super::Foundation:: ERESOURCE));
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleaseRundownProtection(runref : *mut EX_RUNDOWN_REF));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleaseRundownProtectionCacheAware(runrefcacheaware : super::super::Foundation:: PEX_RUNDOWN_REF_CACHE_AWARE));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleaseRundownProtectionCacheAwareEx(runref : super::super::Foundation:: PEX_RUNDOWN_REF_CACHE_AWARE, count : u32));
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleaseRundownProtectionEx(runref : *mut EX_RUNDOWN_REF, count : u32));
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleaseSpinLockExclusive(spinlock : *mut i32, oldirql : u8));
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleaseSpinLockExclusiveFromDpcLevel(spinlock : *mut i32));
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleaseSpinLockShared(spinlock : *mut i32, oldirql : u8));
windows_targets::link!("ntoskrnl.exe" "system" fn ExReleaseSpinLockSharedFromDpcLevel(spinlock : *mut i32));
windows_targets::link!("ntoskrnl.exe" "system" fn ExRundownCompleted(runref : *mut EX_RUNDOWN_REF));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExRundownCompletedCacheAware(runrefcacheaware : super::super::Foundation:: PEX_RUNDOWN_REF_CACHE_AWARE));
windows_targets::link!("ntoskrnl.exe" "system" fn ExSecurePoolUpdate(securepoolhandle : super::super::super::Win32::Foundation:: HANDLE, tag : u32, allocation : *const core::ffi::c_void, cookie : usize, offset : usize, size : usize, buffer : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn ExSecurePoolValidate(securepoolhandle : super::super::super::Win32::Foundation:: HANDLE, tag : u32, allocation : *const core::ffi::c_void, cookie : usize) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn ExSetFirmwareEnvironmentVariable(variablename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, vendorguid : *const windows_sys::core::GUID, value : *const core::ffi::c_void, valuelength : u32, attributes : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExSetResourceOwnerPointer(resource : *mut super::super::Foundation:: ERESOURCE, ownerpointer : *const core::ffi::c_void));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExSetResourceOwnerPointerEx(resource : *mut super::super::Foundation:: ERESOURCE, ownerpointer : *const core::ffi::c_void, flags : u32));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExSetTimer(timer : super::super::Foundation:: PEX_TIMER, duetime : i64, period : i64, parameters : *const _EXT_SET_PARAMETERS_V0) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn ExSetTimerResolution(desiredtime : u32, setresolution : super::super::super::Win32::Foundation:: BOOLEAN) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn ExSizeOfRundownProtectionCacheAware() -> usize);
windows_targets::link!("ntoskrnl.exe" "system" fn ExSystemTimeToLocalTime(systemtime : *const i64, localtime : *mut i64));
windows_targets::link!("ntoskrnl.exe" "system" fn ExTryAcquireSpinLockExclusiveAtDpcLevel(spinlock : *mut i32) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn ExTryAcquireSpinLockSharedAtDpcLevel(spinlock : *mut i32) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn ExTryConvertSharedSpinLockExclusive(spinlock : *mut i32) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn ExTryToAcquireFastMutex(fastmutex : *mut super::super::Foundation:: FAST_MUTEX) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn ExUnregisterCallback(callbackregistration : *mut core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn ExUuidCreate(uuid : *mut windows_sys::core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExVerifySuite(suitetype : super::super::super::Win32::System::Kernel:: SUITE_TYPE) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn ExWaitForRundownProtectionRelease(runref : *mut EX_RUNDOWN_REF));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ExWaitForRundownProtectionReleaseCacheAware(runref : super::super::Foundation:: PEX_RUNDOWN_REF_CACHE_AWARE));
windows_targets::link!("ntoskrnl.exe" "system" fn FsRtlIsTotalDeviceFailure(status : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("hal.dll" "system" fn HalAcquireDisplayOwnership(resetdisplayparameters : PHAL_RESET_DISPLAY_PARAMETERS));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_IscsiDisc", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("hal.dll" "system" fn HalAllocateAdapterChannel(adapterobject : *const super::super::super::Win32::Storage::IscsiDisc:: _ADAPTER_OBJECT, wcb : *const WAIT_CONTEXT_BLOCK, numberofmapregisters : u32, executionroutine : super::super::Foundation:: DRIVER_CONTROL) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_IscsiDisc")]
windows_targets::link!("hal.dll" "system" fn HalAllocateCommonBuffer(adapterobject : *const super::super::super::Win32::Storage::IscsiDisc:: _ADAPTER_OBJECT, length : u32, logicaladdress : *mut i64, cacheenabled : super::super::super::Win32::Foundation:: BOOLEAN) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_Storage_IscsiDisc")]
windows_targets::link!("hal.dll" "system" fn HalAllocateCrashDumpRegisters(adapterobject : *const super::super::super::Win32::Storage::IscsiDisc:: _ADAPTER_OBJECT, numberofmapregisters : *mut u32) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("hal.dll" "system" fn HalAllocateHardwareCounters(groupaffinty : *const super::super::super::Win32::System::SystemInformation:: GROUP_AFFINITY, groupcount : u32, resourcelist : *const PHYSICAL_COUNTER_RESOURCE_LIST, countersethandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("hal.dll" "system" fn HalAssignSlotResources(registrypath : *const super::super::super::Win32::Foundation:: UNICODE_STRING, driverclassname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, driverobject : *const super::super::Foundation:: DRIVER_OBJECT, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, bustype : INTERFACE_TYPE, busnumber : u32, slotnumber : u32, allocatedresources : *mut *mut CM_RESOURCE_LIST) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("hal.dll" "system" fn HalBugCheckSystem(errorsource : *const super::super::super::Win32::System::Diagnostics::Debug:: WHEA_ERROR_SOURCE_DESCRIPTOR, errorrecord : *const WHEA_ERROR_RECORD));
#[cfg(feature = "Win32_Storage_IscsiDisc")]
windows_targets::link!("hal.dll" "system" fn HalDmaAllocateCrashDumpRegistersEx(adapter : *const super::super::super::Win32::Storage::IscsiDisc:: _ADAPTER_OBJECT, numberofmapregisters : u32, r#type : HAL_DMA_CRASH_DUMP_REGISTER_TYPE, mapregisterbase : *mut *mut core::ffi::c_void, mapregistersavailable : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_IscsiDisc")]
windows_targets::link!("hal.dll" "system" fn HalDmaFreeCrashDumpRegistersEx(adapter : *const super::super::super::Win32::Storage::IscsiDisc:: _ADAPTER_OBJECT, r#type : HAL_DMA_CRASH_DUMP_REGISTER_TYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn HalExamineMBR(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, sectorsize : u32, mbrtypeidentifier : u32, buffer : *mut *mut core::ffi::c_void));
#[cfg(feature = "Win32_Storage_IscsiDisc")]
windows_targets::link!("hal.dll" "system" fn HalFreeCommonBuffer(adapterobject : *const super::super::super::Win32::Storage::IscsiDisc:: _ADAPTER_OBJECT, length : u32, logicaladdress : i64, virtualaddress : *const core::ffi::c_void, cacheenabled : super::super::super::Win32::Foundation:: BOOLEAN));
windows_targets::link!("hal.dll" "system" fn HalFreeHardwareCounters(countersethandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_IscsiDisc")]
windows_targets::link!("hal.dll" "system" fn HalGetAdapter(devicedescription : *const DEVICE_DESCRIPTION, numberofmapregisters : *mut u32) -> *mut super::super::super::Win32::Storage::IscsiDisc:: _ADAPTER_OBJECT);
windows_targets::link!("hal.dll" "system" fn HalGetBusData(busdatatype : BUS_DATA_TYPE, busnumber : u32, slotnumber : u32, buffer : *mut core::ffi::c_void, length : u32) -> u32);
windows_targets::link!("hal.dll" "system" fn HalGetBusDataByOffset(busdatatype : BUS_DATA_TYPE, busnumber : u32, slotnumber : u32, buffer : *mut core::ffi::c_void, offset : u32, length : u32) -> u32);
windows_targets::link!("hal.dll" "system" fn HalGetInterruptVector(interfacetype : INTERFACE_TYPE, busnumber : u32, businterruptlevel : u32, businterruptvector : u32, irql : *mut u8, affinity : *mut usize) -> u32);
windows_targets::link!("hal.dll" "system" fn HalMakeBeep(frequency : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Storage_IscsiDisc")]
windows_targets::link!("hal.dll" "system" fn HalReadDmaCounter(adapterobject : *const super::super::super::Win32::Storage::IscsiDisc:: _ADAPTER_OBJECT) -> u32);
windows_targets::link!("hal.dll" "system" fn HalSetBusData(busdatatype : BUS_DATA_TYPE, busnumber : u32, slotnumber : u32, buffer : *const core::ffi::c_void, length : u32) -> u32);
windows_targets::link!("hal.dll" "system" fn HalSetBusDataByOffset(busdatatype : BUS_DATA_TYPE, busnumber : u32, slotnumber : u32, buffer : *const core::ffi::c_void, offset : u32, length : u32) -> u32);
windows_targets::link!("hal.dll" "system" fn HalTranslateBusAddress(interfacetype : INTERFACE_TYPE, busnumber : u32, busaddress : i64, addressspace : *mut u32, translatedaddress : *mut i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn HvlRegisterWheaErrorNotification(callback : PHVL_WHEA_ERROR_NOTIFICATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn HvlUnregisterWheaErrorNotification(callback : PHVL_WHEA_ERROR_NOTIFICATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoAcquireCancelSpinLock(irql : *mut u8));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAcquireKsrPersistentMemory(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, buffer : *mut core::ffi::c_void, size : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAcquireKsrPersistentMemoryEx(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, physicaldeviceid : *const super::super::super::Win32::Foundation:: UNICODE_STRING, datatag : *const u16, dataversion : *mut u32, buffer : *mut core::ffi::c_void, size : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAcquireRemoveLockEx(removelock : *mut IO_REMOVE_LOCK, tag : *const core::ffi::c_void, file : windows_sys::core::PCSTR, line : u32, remlocksize : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_IscsiDisc", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAllocateAdapterChannel(adapterobject : *const super::super::super::Win32::Storage::IscsiDisc:: _ADAPTER_OBJECT, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, numberofmapregisters : u32, executionroutine : super::super::Foundation:: DRIVER_CONTROL, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAllocateController(controllerobject : *const CONTROLLER_OBJECT, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, executionroutine : super::super::Foundation:: DRIVER_CONTROL, context : *const core::ffi::c_void));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAllocateDriverObjectExtension(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, clientidentificationaddress : *const core::ffi::c_void, driverobjectextensionsize : u32, driverobjectextension : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoAllocateErrorLogEntry(ioobject : *const core::ffi::c_void, entrysize : u8) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAllocateIrp(stacksize : i8, chargequota : super::super::super::Win32::Foundation:: BOOLEAN) -> *mut super::super::Foundation:: IRP);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAllocateIrpEx(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, stacksize : i8, chargequota : super::super::super::Win32::Foundation:: BOOLEAN) -> *mut super::super::Foundation:: IRP);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAllocateMdl(virtualaddress : *const core::ffi::c_void, length : u32, secondarybuffer : super::super::super::Win32::Foundation:: BOOLEAN, chargequota : super::super::super::Win32::Foundation:: BOOLEAN, irp : *mut super::super::Foundation:: IRP) -> *mut super::super::Foundation:: MDL);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAllocateSfioStreamIdentifier(fileobject : *const super::super::Foundation:: FILE_OBJECT, length : u32, signature : *const core::ffi::c_void, streamidentifier : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAllocateWorkItem(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::Foundation:: PIO_WORKITEM);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAssignResources(registrypath : *const super::super::super::Win32::Foundation:: UNICODE_STRING, driverclassname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, driverobject : *const super::super::Foundation:: DRIVER_OBJECT, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, requestedresources : *const IO_RESOURCE_REQUIREMENTS_LIST, allocatedresources : *mut *mut CM_RESOURCE_LIST) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAttachDevice(sourcedevice : *const super::super::Foundation:: DEVICE_OBJECT, targetdevice : *const super::super::super::Win32::Foundation:: UNICODE_STRING, attacheddevice : *mut *mut super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAttachDeviceByPointer(sourcedevice : *const super::super::Foundation:: DEVICE_OBJECT, targetdevice : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAttachDeviceToDeviceStack(sourcedevice : *const super::super::Foundation:: DEVICE_OBJECT, targetdevice : *const super::super::Foundation:: DEVICE_OBJECT) -> *mut super::super::Foundation:: DEVICE_OBJECT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoAttachDeviceToDeviceStackSafe(sourcedevice : *const super::super::Foundation:: DEVICE_OBJECT, targetdevice : *const super::super::Foundation:: DEVICE_OBJECT, attachedtodeviceobject : *mut *mut super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoBuildAsynchronousFsdRequest(majorfunction : u32, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, buffer : *mut core::ffi::c_void, length : u32, startingoffset : *const i64, iostatusblock : *const super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> *mut super::super::Foundation:: IRP);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoBuildDeviceIoControlRequest(iocontrolcode : u32, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32, internaldeviceiocontrol : super::super::super::Win32::Foundation:: BOOLEAN, event : *const super::super::Foundation:: KEVENT, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> *mut super::super::Foundation:: IRP);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoBuildPartialMdl(sourcemdl : *const super::super::Foundation:: MDL, targetmdl : *mut super::super::Foundation:: MDL, virtualaddress : *mut core::ffi::c_void, length : u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoBuildSynchronousFsdRequest(majorfunction : u32, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, buffer : *mut core::ffi::c_void, length : u32, startingoffset : *const i64, event : *const super::super::Foundation:: KEVENT, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK) -> *mut super::super::Foundation:: IRP);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCancelFileOpen(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, fileobject : *const super::super::Foundation:: FILE_OBJECT));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCancelIrp(irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCheckLinkShareAccess(desiredaccess : u32, desiredshareaccess : u32, fileobject : *mut super::super::Foundation:: FILE_OBJECT, shareaccess : *mut SHARE_ACCESS, linkshareaccess : *mut LINK_SHARE_ACCESS, ioshareaccessflags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCheckShareAccess(desiredaccess : u32, desiredshareaccess : u32, fileobject : *mut super::super::Foundation:: FILE_OBJECT, shareaccess : *mut SHARE_ACCESS, update : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCheckShareAccessEx(desiredaccess : u32, desiredshareaccess : u32, fileobject : *mut super::super::Foundation:: FILE_OBJECT, shareaccess : *mut SHARE_ACCESS, update : super::super::super::Win32::Foundation:: BOOLEAN, writepermission : *const super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCleanupIrp(irp : *mut super::super::Foundation:: IRP));
windows_targets::link!("ntoskrnl.exe" "system" fn IoClearActivityIdThread(originalid : *const windows_sys::core::GUID));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoClearIrpExtraCreateParameter(irp : *mut super::super::Foundation:: IRP));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoConnectInterrupt(interruptobject : *mut super::super::Foundation:: PKINTERRUPT, serviceroutine : PKSERVICE_ROUTINE, servicecontext : *const core::ffi::c_void, spinlock : *const usize, vector : u32, irql : u8, synchronizeirql : u8, interruptmode : KINTERRUPT_MODE, sharevector : super::super::super::Win32::Foundation:: BOOLEAN, processorenablemask : usize, floatingsave : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoConnectInterruptEx(parameters : *mut IO_CONNECT_INTERRUPT_PARAMETERS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateController(size : u32) -> *mut CONTROLLER_OBJECT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateDevice(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, deviceextensionsize : u32, devicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, devicetype : u32, devicecharacteristics : u32, exclusive : super::super::super::Win32::Foundation:: BOOLEAN, deviceobject : *mut *mut super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Ioctl", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateDisk(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, disk : *const super::super::super::Win32::System::Ioctl:: CREATE_DISK) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_IO"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateFile(filehandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, allocationsize : *const i64, fileattributes : u32, shareaccess : u32, disposition : u32, createoptions : u32, eabuffer : *const core::ffi::c_void, ealength : u32, createfiletype : CREATE_FILE_TYPE, internalparameters : *const core::ffi::c_void, options : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_IO"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateFileEx(filehandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, allocationsize : *const i64, fileattributes : u32, shareaccess : u32, disposition : u32, createoptions : u32, eabuffer : *const core::ffi::c_void, ealength : u32, createfiletype : CREATE_FILE_TYPE, internalparameters : *const core::ffi::c_void, options : u32, drivercontext : *const IO_DRIVER_CREATE_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_IO"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateFileSpecifyDeviceObjectHint(filehandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, allocationsize : *const i64, fileattributes : u32, shareaccess : u32, disposition : u32, createoptions : u32, eabuffer : *const core::ffi::c_void, ealength : u32, createfiletype : CREATE_FILE_TYPE, internalparameters : *const core::ffi::c_void, options : u32, deviceobject : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateNotificationEvent(eventname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, eventhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> *mut super::super::Foundation:: KEVENT);
windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateSymbolicLink(symboliclinkname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, devicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateSynchronizationEvent(eventname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, eventhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> *mut super::super::Foundation:: KEVENT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_WindowsProgramming"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateSystemThread(ioobject : *mut core::ffi::c_void, threadhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, processhandle : super::super::super::Win32::Foundation:: HANDLE, clientid : *mut super::super::super::Win32::System::WindowsProgramming:: CLIENT_ID, startroutine : PKSTART_ROUTINE, startcontext : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoCreateUnprotectedSymbolicLink(symboliclinkname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, devicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCsqInitialize(csq : *mut IO_CSQ, csqinsertirp : PIO_CSQ_INSERT_IRP, csqremoveirp : PIO_CSQ_REMOVE_IRP, csqpeeknextirp : PIO_CSQ_PEEK_NEXT_IRP, csqacquirelock : PIO_CSQ_ACQUIRE_LOCK, csqreleaselock : PIO_CSQ_RELEASE_LOCK, csqcompletecanceledirp : PIO_CSQ_COMPLETE_CANCELED_IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCsqInitializeEx(csq : *mut IO_CSQ, csqinsertirp : PIO_CSQ_INSERT_IRP_EX, csqremoveirp : PIO_CSQ_REMOVE_IRP, csqpeeknextirp : PIO_CSQ_PEEK_NEXT_IRP, csqacquirelock : PIO_CSQ_ACQUIRE_LOCK, csqreleaselock : PIO_CSQ_RELEASE_LOCK, csqcompletecanceledirp : PIO_CSQ_COMPLETE_CANCELED_IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCsqInsertIrp(csq : *mut IO_CSQ, irp : *mut super::super::Foundation:: IRP, context : *mut IO_CSQ_IRP_CONTEXT));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCsqInsertIrpEx(csq : *mut IO_CSQ, irp : *mut super::super::Foundation:: IRP, context : *mut IO_CSQ_IRP_CONTEXT, insertcontext : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCsqRemoveIrp(csq : *mut IO_CSQ, context : *mut IO_CSQ_IRP_CONTEXT) -> *mut super::super::Foundation:: IRP);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoCsqRemoveNextIrp(csq : *mut IO_CSQ, peekcontext : *const core::ffi::c_void) -> *mut super::super::Foundation:: IRP);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoDecrementKeepAliveCount(fileobject : *mut super::super::Foundation:: FILE_OBJECT, process : super::super::Foundation:: PEPROCESS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoDeleteController(controllerobject : *const CONTROLLER_OBJECT));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoDeleteDevice(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT));
windows_targets::link!("ntoskrnl.exe" "system" fn IoDeleteSymbolicLink(symboliclinkname : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoDetachDevice(targetdevice : *mut super::super::Foundation:: DEVICE_OBJECT));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoDisconnectInterrupt(interruptobject : super::super::Foundation:: PKINTERRUPT));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoDisconnectInterruptEx(parameters : *const IO_DISCONNECT_INTERRUPT_PARAMETERS));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoEnumerateKsrPersistentMemoryEx(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, physicaldeviceid : *const super::super::super::Win32::Foundation:: UNICODE_STRING, callback : PIO_PERSISTED_MEMORY_ENUMERATION_CALLBACK, callbackcontext : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Storage_IscsiDisc"))]
windows_targets::link!("hal.dll" "system" fn IoFlushAdapterBuffers(adapterobject : *const super::super::super::Win32::Storage::IscsiDisc:: _ADAPTER_OBJECT, mdl : *const super::super::Foundation:: MDL, mapregisterbase : *const core::ffi::c_void, currentva : *const core::ffi::c_void, length : u32, writetodevice : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoForwardIrpSynchronously(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Storage_IscsiDisc")]
windows_targets::link!("hal.dll" "system" fn IoFreeAdapterChannel(adapterobject : *const super::super::super::Win32::Storage::IscsiDisc:: _ADAPTER_OBJECT));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoFreeController(controllerobject : *const CONTROLLER_OBJECT));
windows_targets::link!("ntoskrnl.exe" "system" fn IoFreeErrorLogEntry(elentry : *const core::ffi::c_void));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoFreeIrp(irp : *const super::super::Foundation:: IRP));
windows_targets::link!("ntoskrnl.exe" "system" fn IoFreeKsrPersistentMemory(datahandle : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_IscsiDisc")]
windows_targets::link!("hal.dll" "system" fn IoFreeMapRegisters(adapterobject : *const super::super::super::Win32::Storage::IscsiDisc:: _ADAPTER_OBJECT, mapregisterbase : *const core::ffi::c_void, numberofmapregisters : u32));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoFreeMdl(mdl : *mut super::super::Foundation:: MDL));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoFreeSfioStreamIdentifier(fileobject : *const super::super::Foundation:: FILE_OBJECT, signature : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoFreeWorkItem(ioworkitem : super::super::Foundation:: PIO_WORKITEM));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetActivityIdIrp(irp : *const super::super::Foundation:: IRP, guid : *mut windows_sys::core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetActivityIdThread() -> *mut windows_sys::core::GUID);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_SystemInformation"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetAffinityInterrupt(interruptobject : super::super::Foundation:: PKINTERRUPT, groupaffinity : *mut super::super::super::Win32::System::SystemInformation:: GROUP_AFFINITY) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetAttachedDeviceReference(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> *mut super::super::Foundation:: DEVICE_OBJECT);
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetBootDiskInformation(bootdiskinformation : *mut BOOTDISK_INFORMATION, size : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetBootDiskInformationLite(bootdiskinformation : *mut *mut BOOTDISK_INFORMATION_LITE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetConfigurationInformation() -> *mut CONFIGURATION_INFORMATION);
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetContainerInformation(informationclass : IO_CONTAINER_INFORMATION_CLASS, containerobject : *const core::ffi::c_void, buffer : *mut core::ffi::c_void, bufferlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetCurrentProcess() -> super::super::Foundation:: PEPROCESS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetDeviceDirectory(physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, directorytype : DEVICE_DIRECTORY_TYPE, flags : u32, reserved : *const core::ffi::c_void, devicedirectoryhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetDeviceInterfaceAlias(symboliclinkname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, aliasinterfaceclassguid : *const windows_sys::core::GUID, aliassymboliclinkname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Devices_Properties")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetDeviceInterfacePropertyData(symboliclinkname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, propertykey : *const super::super::super::Win32::Devices::Properties:: DEVPROPKEY, lcid : u32, flags : u32, size : u32, data : *mut core::ffi::c_void, requiredsize : *mut u32, r#type : *mut super::super::super::Win32::Devices::Properties:: DEVPROPTYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetDeviceInterfaces(interfaceclassguid : *const windows_sys::core::GUID, physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, flags : u32, symboliclinklist : *mut windows_sys::core::PWSTR) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetDeviceNumaNode(pdo : *const super::super::Foundation:: DEVICE_OBJECT, nodenumber : *mut u16) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetDeviceObjectPointer(objectname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, desiredaccess : u32, fileobject : *mut *mut super::super::Foundation:: FILE_OBJECT, deviceobject : *mut *mut super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetDeviceProperty(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, deviceproperty : DEVICE_REGISTRY_PROPERTY, bufferlength : u32, propertybuffer : *mut core::ffi::c_void, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Devices_Properties", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetDevicePropertyData(pdo : *const super::super::Foundation:: DEVICE_OBJECT, propertykey : *const super::super::super::Win32::Devices::Properties:: DEVPROPKEY, lcid : u32, flags : u32, size : u32, data : *mut core::ffi::c_void, requiredsize : *mut u32, r#type : *mut super::super::super::Win32::Devices::Properties:: DEVPROPTYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetDmaAdapter(physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, devicedescription : *const DEVICE_DESCRIPTION, numberofmapregisters : *mut u32) -> *mut DMA_ADAPTER);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetDriverDirectory(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, directorytype : DRIVER_DIRECTORY_TYPE, flags : u32, driverdirectoryhandle : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetDriverObjectExtension(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, clientidentificationaddress : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetFileObjectGenericMapping() -> *mut super::super::super::Win32::Security:: GENERIC_MAPPING);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetFsZeroingOffset(irp : *const super::super::Foundation:: IRP, zeroingoffset : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetInitialStack() -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetInitiatorProcess(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> super::super::Foundation:: PEPROCESS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetIoAttributionHandle(irp : *const super::super::Foundation:: IRP, ioattributionhandle : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetIoPriorityHint(irp : *const super::super::Foundation:: IRP) -> super::super::Foundation:: IO_PRIORITY_HINT);
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetIommuInterface(version : u32, interfaceout : *mut DMA_IOMMU_INTERFACE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetIommuInterfaceEx(version : u32, flags : u64, interfaceout : *mut DMA_IOMMU_INTERFACE_EX) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetIrpExtraCreateParameter(irp : *const super::super::Foundation:: IRP, extracreateparameter : *mut *mut isize) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetPagingIoPriority(irp : *const super::super::Foundation:: IRP) -> IO_PAGING_PRIORITY);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetRelatedDeviceObject(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> *mut super::super::Foundation:: DEVICE_OBJECT);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetSfioStreamIdentifier(fileobject : *const super::super::Foundation:: FILE_OBJECT, signature : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetSilo(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> super::super::Foundation:: PESILO);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetSiloParameters(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> *mut IO_FOEXT_SILO_PARAMETERS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetStackLimits(lowlimit : *mut usize, highlimit : *mut usize));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetTopLevelIrp() -> *mut super::super::Foundation:: IRP);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoGetTransactionParameterBlock(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> *mut TXN_PARAMETER_BLOCK);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoIncrementKeepAliveCount(fileobject : *mut super::super::Foundation:: FILE_OBJECT, process : super::super::Foundation:: PEPROCESS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoInitializeIrp(irp : *mut super::super::Foundation:: IRP, packetsize : u16, stacksize : i8));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoInitializeIrpEx(irp : *mut super::super::Foundation:: IRP, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, packetsize : u16, stacksize : i8));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoInitializeRemoveLockEx(lock : *mut IO_REMOVE_LOCK, allocatetag : u32, maxlockedminutes : u32, highwatermark : u32, remlocksize : u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoInitializeTimer(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, timerroutine : PIO_TIMER_ROUTINE, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoInitializeWorkItem(ioobject : *const core::ffi::c_void, ioworkitem : super::super::Foundation:: PIO_WORKITEM));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoInvalidateDeviceRelations(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, r#type : DEVICE_RELATION_TYPE));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoInvalidateDeviceState(physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoIs32bitProcess(irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoIsFileObjectIgnoringSharing(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoIsFileOriginRemote(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoIsInitiator32bitProcess(irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn IoIsValidIrpStatus(status : super::super::super::Win32::Foundation:: NTSTATUS) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn IoIsWdmVersionAvailable(majorversion : u8, minorversion : u8) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoMakeAssociatedIrp(irp : *const super::super::Foundation:: IRP, stacksize : i8) -> *mut super::super::Foundation:: IRP);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoMakeAssociatedIrpEx(irp : *const super::super::Foundation:: IRP, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, stacksize : i8) -> *mut super::super::Foundation:: IRP);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Storage_IscsiDisc"))]
windows_targets::link!("hal.dll" "system" fn IoMapTransfer(adapterobject : *const super::super::super::Win32::Storage::IscsiDisc:: _ADAPTER_OBJECT, mdl : *const super::super::Foundation:: MDL, mapregisterbase : *const core::ffi::c_void, currentva : *const core::ffi::c_void, length : *mut u32, writetodevice : super::super::super::Win32::Foundation:: BOOLEAN) -> i64);
windows_targets::link!("ntoskrnl.exe" "system" fn IoOpenDeviceInterfaceRegistryKey(symboliclinkname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, desiredaccess : u32, deviceinterfaceregkey : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoOpenDeviceRegistryKey(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, devinstkeytype : u32, desiredaccess : u32, deviceregkey : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoOpenDriverRegistryKey(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, regkeytype : DRIVER_REGKEY_TYPE, desiredaccess : u32, flags : u32, driverregkey : *mut super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoPropagateActivityIdToThread(irp : *const super::super::Foundation:: IRP, propagatedid : *mut windows_sys::core::GUID, originalid : *mut *mut windows_sys::core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoQueryDeviceDescription(bustype : *const INTERFACE_TYPE, busnumber : *const u32, controllertype : *const CONFIGURATION_TYPE, controllernumber : *const u32, peripheraltype : *const CONFIGURATION_TYPE, peripheralnumber : *const u32, calloutroutine : PIO_QUERY_DEVICE_ROUTINE, context : *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoQueryFullDriverPath(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, fullpath : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_System_IO"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoQueryInformationByName(objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::super::Storage::FileSystem:: FILE_INFORMATION_CLASS, options : u32, drivercontext : *const IO_DRIVER_CREATE_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoQueryKsrPersistentMemorySize(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, buffersize : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoQueryKsrPersistentMemorySizeEx(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, physicaldeviceid : *const super::super::super::Win32::Foundation:: UNICODE_STRING, datatag : *const u16, dataversion : *mut u32, buffersize : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoQueueWorkItem(ioworkitem : super::super::Foundation:: PIO_WORKITEM, workerroutine : PIO_WORKITEM_ROUTINE, queuetype : WORK_QUEUE_TYPE, context : *const core::ffi::c_void));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoQueueWorkItemEx(ioworkitem : super::super::Foundation:: PIO_WORKITEM, workerroutine : PIO_WORKITEM_ROUTINE_EX, queuetype : WORK_QUEUE_TYPE, context : *const core::ffi::c_void));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoRaiseHardError(irp : *const super::super::Foundation:: IRP, vpb : *const super::super::Foundation:: VPB, realdeviceobject : *const super::super::Foundation:: DEVICE_OBJECT));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoRaiseInformationalHardError(errorstatus : super::super::super::Win32::Foundation:: NTSTATUS, string : *const super::super::super::Win32::Foundation:: UNICODE_STRING, thread : super::super::Foundation:: PKTHREAD) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReadDiskSignature(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, bytespersector : u32, signature : *mut DISK_SIGNATURE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Ioctl", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReadPartitionTable(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, sectorsize : u32, returnrecognizedpartitions : super::super::super::Win32::Foundation:: BOOLEAN, partitionbuffer : *mut *mut super::super::super::Win32::System::Ioctl:: DRIVE_LAYOUT_INFORMATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Ioctl", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReadPartitionTableEx(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, drivelayout : *mut *mut super::super::super::Win32::System::Ioctl:: DRIVE_LAYOUT_INFORMATION_EX) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoRecordIoAttribution(opaquehandle : *mut core::ffi::c_void, attributioninformation : *const IO_ATTRIBUTION_INFORMATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoRegisterBootDriverCallback(callbackfunction : PBOOT_DRIVER_CALLBACK_FUNCTION, callbackcontext : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoRegisterBootDriverReinitialization(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, driverreinitializationroutine : super::super::Foundation:: DRIVER_REINITIALIZE, context : *const core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn IoRegisterContainerNotification(notificationclass : IO_CONTAINER_NOTIFICATION_CLASS, callbackfunction : PIO_CONTAINER_NOTIFICATION_FUNCTION, notificationinformation : *const core::ffi::c_void, notificationinformationlength : u32, callbackregistration : *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoRegisterDeviceInterface(physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, interfaceclassguid : *const windows_sys::core::GUID, referencestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING, symboliclinkname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoRegisterDriverReinitialization(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, driverreinitializationroutine : super::super::Foundation:: DRIVER_REINITIALIZE, context : *const core::ffi::c_void));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoRegisterLastChanceShutdownNotification(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoRegisterPlugPlayNotification(eventcategory : IO_NOTIFICATION_EVENT_CATEGORY, eventcategoryflags : u32, eventcategorydata : *const core::ffi::c_void, driverobject : *const super::super::Foundation:: DRIVER_OBJECT, callbackroutine : super::super::Foundation:: DRIVER_NOTIFICATION_CALLBACK_ROUTINE, context : *mut core::ffi::c_void, notificationentry : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoRegisterShutdownNotification(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoReleaseCancelSpinLock(irql : u8));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReleaseRemoveLockAndWaitEx(removelock : *mut IO_REMOVE_LOCK, tag : *const core::ffi::c_void, remlocksize : u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReleaseRemoveLockEx(removelock : *mut IO_REMOVE_LOCK, tag : *const core::ffi::c_void, remlocksize : u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoRemoveLinkShareAccess(fileobject : *const super::super::Foundation:: FILE_OBJECT, shareaccess : *mut SHARE_ACCESS, linkshareaccess : *mut LINK_SHARE_ACCESS));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoRemoveLinkShareAccessEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, shareaccess : *mut SHARE_ACCESS, linkshareaccess : *mut LINK_SHARE_ACCESS, ioshareaccessflags : u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoRemoveShareAccess(fileobject : *const super::super::Foundation:: FILE_OBJECT, shareaccess : *mut SHARE_ACCESS));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReplacePartitionUnit(targetpdo : *const super::super::Foundation:: DEVICE_OBJECT, sparepdo : *const super::super::Foundation:: DEVICE_OBJECT, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReportDetectedDevice(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, legacybustype : INTERFACE_TYPE, busnumber : u32, slotnumber : u32, resourcelist : *const CM_RESOURCE_LIST, resourcerequirements : *const IO_RESOURCE_REQUIREMENTS_LIST, resourceassigned : super::super::super::Win32::Foundation:: BOOLEAN, deviceobject : *mut *mut super::super::Foundation:: DEVICE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReportInterruptActive(parameters : *const IO_REPORT_INTERRUPT_ACTIVE_STATE_PARAMETERS));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReportInterruptInactive(parameters : *const IO_REPORT_INTERRUPT_ACTIVE_STATE_PARAMETERS));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReportResourceForDetection(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, driverlist : *const CM_RESOURCE_LIST, driverlistsize : u32, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, devicelist : *const CM_RESOURCE_LIST, devicelistsize : u32, conflictdetected : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReportResourceUsage(driverclassname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, driverobject : *const super::super::Foundation:: DRIVER_OBJECT, driverlist : *const CM_RESOURCE_LIST, driverlistsize : u32, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, devicelist : *const CM_RESOURCE_LIST, devicelistsize : u32, overrideconflict : super::super::super::Win32::Foundation:: BOOLEAN, conflictdetected : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReportRootDevice(driverobject : *const super::super::Foundation:: DRIVER_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReportTargetDeviceChange(physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, notificationstructure : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReportTargetDeviceChangeAsynchronous(physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, notificationstructure : *const core::ffi::c_void, callback : PDEVICE_CHANGE_COMPLETE_CALLBACK, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoRequestDeviceEject(physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoRequestDeviceEjectEx(physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, callback : PIO_DEVICE_EJECT_CALLBACK, context : *const core::ffi::c_void, driverobject : *const super::super::Foundation:: DRIVER_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReserveKsrPersistentMemory(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, size : usize, flags : u32, datahandle : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReserveKsrPersistentMemoryEx(driverobject : *const super::super::Foundation:: DRIVER_OBJECT, physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, physicaldeviceid : *const super::super::super::Win32::Foundation:: UNICODE_STRING, datatag : *const u16, dataversion : u32, size : usize, flags : u32, datahandle : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoReuseIrp(irp : *mut super::super::Foundation:: IRP, iostatus : super::super::super::Win32::Foundation:: NTSTATUS));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetActivityIdIrp(irp : *mut super::super::Foundation:: IRP, guid : *const windows_sys::core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetActivityIdThread(activityid : *const windows_sys::core::GUID) -> *mut windows_sys::core::GUID);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetCompletionRoutineEx(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, irp : *const super::super::Foundation:: IRP, completionroutine : super::super::Foundation:: PIO_COMPLETION_ROUTINE, context : *const core::ffi::c_void, invokeonsuccess : super::super::super::Win32::Foundation:: BOOLEAN, invokeonerror : super::super::super::Win32::Foundation:: BOOLEAN, invokeoncancel : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Devices_Properties")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetDeviceInterfacePropertyData(symboliclinkname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, propertykey : *const super::super::super::Win32::Devices::Properties:: DEVPROPKEY, lcid : u32, flags : u32, r#type : u32, size : u32, data : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetDeviceInterfaceState(symboliclinkname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, enable : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Devices_Properties", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetDevicePropertyData(pdo : *const super::super::Foundation:: DEVICE_OBJECT, propertykey : *const super::super::super::Win32::Devices::Properties:: DEVPROPKEY, lcid : u32, flags : u32, r#type : u32, size : u32, data : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetFileObjectIgnoreSharing(fileobject : *const super::super::Foundation:: FILE_OBJECT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetFileOrigin(fileobject : *const super::super::Foundation:: FILE_OBJECT, remote : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetFsZeroingOffset(irp : *mut super::super::Foundation:: IRP, zeroingoffset : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetFsZeroingOffsetRequired(irp : *mut super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetHardErrorOrVerifyDevice(irp : *const super::super::Foundation:: IRP, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetIoAttributionIrp(irp : *mut super::super::Foundation:: IRP, attributionsource : *const core::ffi::c_void, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetIoPriorityHint(irp : *const super::super::Foundation:: IRP, priorityhint : super::super::Foundation:: IO_PRIORITY_HINT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetIrpExtraCreateParameter(irp : *mut super::super::Foundation:: IRP, extracreateparameter : *const isize) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetLinkShareAccess(desiredaccess : u32, desiredshareaccess : u32, fileobject : *mut super::super::Foundation:: FILE_OBJECT, shareaccess : *mut SHARE_ACCESS, linkshareaccess : *mut LINK_SHARE_ACCESS, ioshareaccessflags : u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetMasterIrpStatus(masterirp : *mut super::super::Foundation:: IRP, status : super::super::super::Win32::Foundation:: NTSTATUS));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetPartitionInformation(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, sectorsize : u32, partitionnumber : u32, partitiontype : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Ioctl", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetPartitionInformationEx(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, partitionnumber : u32, partitioninfo : *const super::super::super::Win32::System::Ioctl:: SET_PARTITION_INFORMATION_EX) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetShareAccess(desiredaccess : u32, desiredshareaccess : u32, fileobject : *mut super::super::Foundation:: FILE_OBJECT, shareaccess : *mut SHARE_ACCESS));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetShareAccessEx(desiredaccess : u32, desiredshareaccess : u32, fileobject : *mut super::super::Foundation:: FILE_OBJECT, shareaccess : *mut SHARE_ACCESS, writepermission : *const super::super::super::Win32::Foundation:: BOOLEAN));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetStartIoAttributes(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, deferredstartio : super::super::super::Win32::Foundation:: BOOLEAN, noncancelable : super::super::super::Win32::Foundation:: BOOLEAN));
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetSystemPartition(volumenamestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetThreadHardErrorMode(enableharderrors : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSetTopLevelIrp(irp : *const super::super::Foundation:: IRP));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSizeOfIrpEx(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, stacksize : i8) -> u16);
windows_targets::link!("ntoskrnl.exe" "system" fn IoSizeofWorkItem() -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoStartNextPacket(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, cancelable : super::super::super::Win32::Foundation:: BOOLEAN));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoStartNextPacketByKey(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, cancelable : super::super::super::Win32::Foundation:: BOOLEAN, key : u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoStartPacket(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, irp : *const super::super::Foundation:: IRP, key : *const u32, cancelfunction : super::super::Foundation:: DRIVER_CANCEL));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoStartTimer(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoStopTimer(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoSynchronousCallDriver(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoTransferActivityId(activityid : *const windows_sys::core::GUID, relatedactivityid : *const windows_sys::core::GUID));
windows_targets::link!("ntoskrnl.exe" "system" fn IoTranslateBusAddress(interfacetype : INTERFACE_TYPE, busnumber : u32, busaddress : i64, addressspace : *mut u32, translatedaddress : *mut i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoTryQueueWorkItem(ioworkitem : super::super::Foundation:: PIO_WORKITEM, workerroutine : PIO_WORKITEM_ROUTINE_EX, queuetype : WORK_QUEUE_TYPE, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn IoUninitializeWorkItem(ioworkitem : super::super::Foundation:: PIO_WORKITEM));
windows_targets::link!("ntoskrnl.exe" "system" fn IoUnregisterBootDriverCallback(callbackhandle : *const core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn IoUnregisterContainerNotification(callbackregistration : *const core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn IoUnregisterPlugPlayNotification(notificationentry : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoUnregisterPlugPlayNotificationEx(notificationentry : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoUnregisterShutdownNotification(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoUpdateLinkShareAccess(fileobject : *const super::super::Foundation:: FILE_OBJECT, shareaccess : *mut SHARE_ACCESS, linkshareaccess : *mut LINK_SHARE_ACCESS));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoUpdateLinkShareAccessEx(fileobject : *const super::super::Foundation:: FILE_OBJECT, shareaccess : *mut SHARE_ACCESS, linkshareaccess : *mut LINK_SHARE_ACCESS, ioshareaccessflags : u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoUpdateShareAccess(fileobject : *const super::super::Foundation:: FILE_OBJECT, shareaccess : *mut SHARE_ACCESS));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoValidateDeviceIoControlAccess(irp : *const super::super::Foundation:: IRP, requiredaccess : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoVerifyPartitionTable(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, fixerrors : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoVolumeDeviceNameToGuid(volumedevicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, guid : *mut windows_sys::core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoVolumeDeviceNameToGuidPath(volumedevicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, guidpath : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoVolumeDeviceToDosName(volumedeviceobject : *const core::ffi::c_void, dosname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoVolumeDeviceToGuid(volumedeviceobject : *const core::ffi::c_void, guid : *mut windows_sys::core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoVolumeDeviceToGuidPath(volumedeviceobject : *const core::ffi::c_void, guidpath : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMIAllocateInstanceIds(guid : *const windows_sys::core::GUID, instancecount : u32, firstinstanceid : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMIDeviceObjectToInstanceName(datablockobject : *const core::ffi::c_void, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, instancename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMIExecuteMethod(datablockobject : *const core::ffi::c_void, instancename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, methodid : u32, inbuffersize : u32, outbuffersize : *mut u32, inoutbuffer : *mut u8) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMIHandleToInstanceName(datablockobject : *const core::ffi::c_void, filehandle : super::super::super::Win32::Foundation:: HANDLE, instancename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMIOpenBlock(guid : *const windows_sys::core::GUID, desiredaccess : u32, datablockobject : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMIQueryAllData(datablockobject : *const core::ffi::c_void, inoutbuffersize : *mut u32, outbuffer : *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMIQueryAllDataMultiple(datablockobjectlist : *const *const core::ffi::c_void, objectcount : u32, inoutbuffersize : *mut u32, outbuffer : *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMIQuerySingleInstance(datablockobject : *const core::ffi::c_void, instancename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, inoutbuffersize : *mut u32, outbuffer : *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMIQuerySingleInstanceMultiple(datablockobjectlist : *const *const core::ffi::c_void, instancenames : *const super::super::super::Win32::Foundation:: UNICODE_STRING, objectcount : u32, inoutbuffersize : *mut u32, outbuffer : *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMIRegistrationControl(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, action : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMISetNotificationCallback(object : *mut core::ffi::c_void, callback : WMI_NOTIFICATION_CALLBACK, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMISetSingleInstance(datablockobject : *const core::ffi::c_void, instancename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, version : u32, valuebuffersize : u32, valuebuffer : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMISetSingleItem(datablockobject : *const core::ffi::c_void, instancename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, dataitemid : u32, version : u32, valuebuffersize : u32, valuebuffer : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMISuggestInstanceName(physicaldeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, symboliclinkname : *const super::super::super::Win32::Foundation:: UNICODE_STRING, combinenames : super::super::super::Win32::Foundation:: BOOLEAN, suggestedinstancename : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoWMIWriteEvent(wnodeeventitem : *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn IoWithinStackLimits(regionstart : usize, regionsize : usize) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn IoWriteErrorLogEntry(elentry : *const core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn IoWriteKsrPersistentMemory(datahandle : *const core::ffi::c_void, buffer : *const core::ffi::c_void, size : usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Ioctl", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoWritePartitionTable(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, sectorsize : u32, sectorspertrack : u32, numberofheads : u32, partitionbuffer : *const super::super::super::Win32::System::Ioctl:: DRIVE_LAYOUT_INFORMATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Ioctl", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IoWritePartitionTableEx(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, drivelayout : *const super::super::super::Win32::System::Ioctl:: DRIVE_LAYOUT_INFORMATION_EX) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IofCallDriver(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, irp : *mut super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn IofCompleteRequest(irp : *const super::super::Foundation:: IRP, priorityboost : i8));
windows_targets::link!("ntoskrnl.exe" "system" fn KdChangeOption(option : KD_OPTION, inbufferbytes : u32, inbuffer : *const core::ffi::c_void, outbufferbytes : u32, outbuffer : *mut core::ffi::c_void, outbufferneeded : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KdDisableDebugger() -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KdEnableDebugger() -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KdRefreshDebuggerNotPresent() -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeAcquireGuardedMutex(mutex : *mut super::super::Foundation:: FAST_MUTEX));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeAcquireGuardedMutexUnsafe(fastmutex : *mut super::super::Foundation:: FAST_MUTEX));
windows_targets::link!("ntoskrnl.exe" "system" fn KeAcquireInStackQueuedSpinLock(spinlock : *mut usize, lockhandle : *mut KLOCK_QUEUE_HANDLE));
windows_targets::link!("ntoskrnl.exe" "system" fn KeAcquireInStackQueuedSpinLockAtDpcLevel(spinlock : *mut usize, lockhandle : *mut KLOCK_QUEUE_HANDLE));
windows_targets::link!("ntoskrnl.exe" "system" fn KeAcquireInStackQueuedSpinLockForDpc(spinlock : *mut usize, lockhandle : *mut KLOCK_QUEUE_HANDLE));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeAcquireInterruptSpinLock(interrupt : super::super::Foundation:: PKINTERRUPT) -> u8);
windows_targets::link!("ntoskrnl.exe" "system" fn KeAcquireSpinLockForDpc(spinlock : *mut usize) -> u8);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeAddTriageDumpDataBlock(ktriagedumpdataarray : *mut KTRIAGE_DUMP_DATA_ARRAY, address : *const core::ffi::c_void, size : usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KeAreAllApcsDisabled() -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn KeAreApcsDisabled() -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeBugCheck(bugcheckcode : super::super::super::Win32::System::Diagnostics::Debug:: BUGCHECK_ERROR));
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeBugCheckEx(bugcheckcode : super::super::super::Win32::System::Diagnostics::Debug:: BUGCHECK_ERROR, bugcheckparameter1 : usize, bugcheckparameter2 : usize, bugcheckparameter3 : usize, bugcheckparameter4 : usize));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeCancelTimer(param0 : *mut KTIMER) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeClearEvent(event : *mut super::super::Foundation:: KEVENT));
windows_targets::link!("ntoskrnl.exe" "system" fn KeConvertAuxiliaryCounterToPerformanceCounter(auxiliarycountervalue : u64, performancecountervalue : *mut u64, conversionerror : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KeConvertPerformanceCounterToAuxiliaryCounter(performancecountervalue : u64, auxiliarycountervalue : *mut u64, conversionerror : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KeDelayExecutionThread(waitmode : i8, alertable : super::super::super::Win32::Foundation:: BOOLEAN, interval : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KeDeregisterBoundCallback(handle : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeDeregisterBugCheckCallback(callbackrecord : *mut KBUGCHECK_CALLBACK_RECORD) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeDeregisterBugCheckReasonCallback(callbackrecord : *mut KBUGCHECK_REASON_CALLBACK_RECORD) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn KeDeregisterNmiCallback(handle : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KeDeregisterProcessorChangeCallback(callbackhandle : *const core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn KeEnterCriticalRegion());
windows_targets::link!("ntoskrnl.exe" "system" fn KeEnterGuardedRegion());
windows_targets::link!("ntoskrnl.exe" "system" fn KeExpandKernelStackAndCallout(callout : PEXPAND_STACK_CALLOUT, parameter : *const core::ffi::c_void, size : usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KeExpandKernelStackAndCalloutEx(callout : PEXPAND_STACK_CALLOUT, parameter : *const core::ffi::c_void, size : usize, wait : super::super::super::Win32::Foundation:: BOOLEAN, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeFlushIoBuffers(mdl : *const super::super::Foundation:: MDL, readoperation : super::super::super::Win32::Foundation:: BOOLEAN, dmaoperation : super::super::super::Win32::Foundation:: BOOLEAN));
windows_targets::link!("ntoskrnl.exe" "system" fn KeFlushQueuedDpcs());
windows_targets::link!("hal.dll" "system" fn KeFlushWriteBuffer());
windows_targets::link!("ntoskrnl.exe" "system" fn KeGetCurrentIrql() -> u8);
windows_targets::link!("ntoskrnl.exe" "system" fn KeGetCurrentNodeNumber() -> u16);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeGetCurrentProcessorNumberEx(procnumber : *mut super::super::super::Win32::System::Kernel:: PROCESSOR_NUMBER) -> u32);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeGetProcessorIndexFromNumber(procnumber : *const super::super::super::Win32::System::Kernel:: PROCESSOR_NUMBER) -> u32);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeGetProcessorNumberFromIndex(procindex : u32, procnumber : *mut super::super::super::Win32::System::Kernel:: PROCESSOR_NUMBER) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KeGetRecommendedSharedDataAlignment() -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn KeInitializeCrashDumpHeader(dumptype : u32, flags : u32, buffer : *mut core::ffi::c_void, buffersize : u32, bufferneeded : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeInitializeDeviceQueue(devicequeue : *mut super::super::Foundation:: KDEVICE_QUEUE));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeInitializeDpc(dpc : *mut super::super::Foundation:: KDPC, deferredroutine : super::super::Foundation:: PKDEFERRED_ROUTINE, deferredcontext : *const core::ffi::c_void));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeInitializeEvent(event : *mut super::super::Foundation:: KEVENT, r#type : super::super::super::Win32::System::Kernel:: EVENT_TYPE, state : super::super::super::Win32::Foundation:: BOOLEAN));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeInitializeGuardedMutex(mutex : *mut super::super::Foundation:: FAST_MUTEX));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeInitializeMutex(mutex : *mut super::super::Foundation:: KMUTANT, level : u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeInitializeSemaphore(semaphore : *mut KSEMAPHORE, count : i32, limit : i32));
windows_targets::link!("ntoskrnl.exe" "system" fn KeInitializeSpinLock(spinlock : *mut usize));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeInitializeThreadedDpc(dpc : *mut super::super::Foundation:: KDPC, deferredroutine : super::super::Foundation:: PKDEFERRED_ROUTINE, deferredcontext : *const core::ffi::c_void));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeInitializeTimer(timer : *mut KTIMER));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeInitializeTimerEx(timer : *mut KTIMER, r#type : super::super::super::Win32::System::Kernel:: TIMER_TYPE));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeInitializeTriageDumpDataArray(ktriagedumpdataarray : *mut KTRIAGE_DUMP_DATA_ARRAY, size : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeInsertByKeyDeviceQueue(devicequeue : *mut super::super::Foundation:: KDEVICE_QUEUE, devicequeueentry : *mut KDEVICE_QUEUE_ENTRY, sortkey : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeInsertDeviceQueue(devicequeue : *mut super::super::Foundation:: KDEVICE_QUEUE, devicequeueentry : *mut KDEVICE_QUEUE_ENTRY) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeInsertQueueDpc(dpc : *mut super::super::Foundation:: KDPC, systemargument1 : *const core::ffi::c_void, systemargument2 : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn KeInvalidateAllCaches() -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn KeInvalidateRangeAllCaches(baseaddress : *const core::ffi::c_void, length : u32));
windows_targets::link!("ntoskrnl.exe" "system" fn KeIpiGenericCall(broadcastfunction : PKIPI_BROADCAST_WORKER, context : usize) -> usize);
windows_targets::link!("ntoskrnl.exe" "system" fn KeIsExecutingDpc() -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn KeLeaveCriticalRegion());
windows_targets::link!("ntoskrnl.exe" "system" fn KeLeaveGuardedRegion());
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KePulseEvent(event : *mut super::super::Foundation:: KEVENT, increment : i32, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> i32);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryActiveGroupCount() -> u16);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryActiveProcessorCount(activeprocessors : *mut usize) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryActiveProcessorCountEx(groupnumber : u16) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryActiveProcessors() -> usize);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryAuxiliaryCounterFrequency(auxiliarycounterfrequency : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryDpcWatchdogInformation(watchdoginformation : *mut KDPC_WATCHDOG_INFORMATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryGroupAffinity(groupnumber : u16) -> usize);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryHardwareCounterConfiguration(counterarray : *mut HARDWARE_COUNTER, maximumcount : u32, count : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryHighestNodeNumber() -> u16);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryInterruptTimePrecise(qpctimestamp : *mut u64) -> u64);
#[cfg(all(feature = "Win32_System_Kernel", feature = "Win32_System_SystemInformation"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryLogicalProcessorRelationship(processornumber : *const super::super::super::Win32::System::Kernel:: PROCESSOR_NUMBER, relationshiptype : super::super::super::Win32::System::SystemInformation:: LOGICAL_PROCESSOR_RELATIONSHIP, information : *mut super::super::super::Win32::System::SystemInformation:: SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX, length : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryMaximumGroupCount() -> u16);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryMaximumProcessorCount() -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryMaximumProcessorCountEx(groupnumber : u16) -> u32);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryNodeActiveAffinity(nodenumber : u16, affinity : *mut super::super::super::Win32::System::SystemInformation:: GROUP_AFFINITY, count : *mut u16));
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryNodeActiveAffinity2(nodenumber : u16, groupaffinities : *mut super::super::super::Win32::System::SystemInformation:: GROUP_AFFINITY, groupaffinitiescount : u16, groupaffinitiesrequired : *mut u16) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryNodeActiveProcessorCount(nodenumber : u16) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryNodeMaximumProcessorCount(nodenumber : u16) -> u16);
windows_targets::link!("hal.dll" "system" fn KeQueryPerformanceCounter(performancefrequency : *mut i64) -> i64);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryPriorityThread(thread : super::super::Foundation:: PKTHREAD) -> i32);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryRuntimeThread(thread : super::super::Foundation:: PKTHREAD, usertime : *mut u32) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQuerySystemTimePrecise(currenttime : *mut i64));
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryTimeIncrement() -> u32);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryTotalCycleTimeThread(thread : super::super::Foundation:: PKTHREAD, cycletimestamp : *mut u64) -> u64);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryUnbiasedInterruptTime() -> u64);
windows_targets::link!("ntoskrnl.exe" "system" fn KeQueryUnbiasedInterruptTimePrecise(qpctimestamp : *mut u64) -> u64);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeReadStateEvent(event : *const super::super::Foundation:: KEVENT) -> i32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeReadStateMutex(mutex : *const super::super::Foundation:: KMUTANT) -> i32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeReadStateSemaphore(semaphore : *const KSEMAPHORE) -> i32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeReadStateTimer(timer : *const KTIMER) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn KeRegisterBoundCallback(callbackroutine : PBOUND_CALLBACK) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeRegisterBugCheckCallback(callbackrecord : *mut KBUGCHECK_CALLBACK_RECORD, callbackroutine : PKBUGCHECK_CALLBACK_ROUTINE, buffer : *const core::ffi::c_void, length : u32, component : *const u8) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeRegisterBugCheckReasonCallback(callbackrecord : *mut KBUGCHECK_REASON_CALLBACK_RECORD, callbackroutine : PKBUGCHECK_REASON_CALLBACK_ROUTINE, reason : KBUGCHECK_CALLBACK_REASON, component : *const u8) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn KeRegisterNmiCallback(callbackroutine : PNMI_CALLBACK, context : *const core::ffi::c_void) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn KeRegisterProcessorChangeCallback(callbackfunction : PPROCESSOR_CALLBACK_FUNCTION, callbackcontext : *const core::ffi::c_void, flags : u32) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeReleaseGuardedMutex(mutex : *mut super::super::Foundation:: FAST_MUTEX));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeReleaseGuardedMutexUnsafe(fastmutex : *mut super::super::Foundation:: FAST_MUTEX));
windows_targets::link!("ntoskrnl.exe" "system" fn KeReleaseInStackQueuedSpinLock(lockhandle : *const KLOCK_QUEUE_HANDLE));
windows_targets::link!("ntoskrnl.exe" "system" fn KeReleaseInStackQueuedSpinLockForDpc(lockhandle : *const KLOCK_QUEUE_HANDLE));
windows_targets::link!("ntoskrnl.exe" "system" fn KeReleaseInStackQueuedSpinLockFromDpcLevel(lockhandle : *const KLOCK_QUEUE_HANDLE));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeReleaseInterruptSpinLock(interrupt : super::super::Foundation:: PKINTERRUPT, oldirql : u8));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeReleaseMutex(mutex : *mut super::super::Foundation:: KMUTANT, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> i32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeReleaseSemaphore(semaphore : *mut KSEMAPHORE, increment : i32, adjustment : i32, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> i32);
windows_targets::link!("ntoskrnl.exe" "system" fn KeReleaseSpinLockForDpc(spinlock : *mut usize, oldirql : u8));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeRemoveByKeyDeviceQueue(devicequeue : *mut super::super::Foundation:: KDEVICE_QUEUE, sortkey : u32) -> *mut KDEVICE_QUEUE_ENTRY);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeRemoveByKeyDeviceQueueIfBusy(devicequeue : *mut super::super::Foundation:: KDEVICE_QUEUE, sortkey : u32) -> *mut KDEVICE_QUEUE_ENTRY);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeRemoveDeviceQueue(devicequeue : *mut super::super::Foundation:: KDEVICE_QUEUE) -> *mut KDEVICE_QUEUE_ENTRY);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeRemoveEntryDeviceQueue(devicequeue : *mut super::super::Foundation:: KDEVICE_QUEUE, devicequeueentry : *mut KDEVICE_QUEUE_ENTRY) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeRemoveQueueDpc(dpc : *mut super::super::Foundation:: KDPC) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeRemoveQueueDpcEx(dpc : *mut super::super::Foundation:: KDPC, waitifactive : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeResetEvent(event : *mut super::super::Foundation:: KEVENT) -> i32);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeRestoreExtendedProcessorState(xstatesave : *const XSTATE_SAVE));
windows_targets::link!("ntoskrnl.exe" "system" fn KeRevertToUserAffinityThread());
windows_targets::link!("ntoskrnl.exe" "system" fn KeRevertToUserAffinityThreadEx(affinity : usize));
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeRevertToUserGroupAffinityThread(previousaffinity : *const super::super::super::Win32::System::SystemInformation:: GROUP_AFFINITY));
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeSaveExtendedProcessorState(mask : u64, xstatesave : *mut XSTATE_SAVE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeSetBasePriorityThread(thread : super::super::Foundation:: PKTHREAD, increment : i32) -> i32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeSetCoalescableTimer(timer : *mut KTIMER, duetime : i64, period : u32, tolerabledelay : u32, dpc : *const super::super::Foundation:: KDPC) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeSetEvent(event : *mut super::super::Foundation:: KEVENT, increment : i32, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> i32);
windows_targets::link!("ntoskrnl.exe" "system" fn KeSetHardwareCounterConfiguration(counterarray : *const HARDWARE_COUNTER, count : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeSetImportanceDpc(dpc : *mut super::super::Foundation:: KDPC, importance : KDPC_IMPORTANCE));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeSetPriorityThread(thread : super::super::Foundation:: PKTHREAD, priority : i32) -> i32);
windows_targets::link!("ntoskrnl.exe" "system" fn KeSetSystemAffinityThread(affinity : usize));
windows_targets::link!("ntoskrnl.exe" "system" fn KeSetSystemAffinityThreadEx(affinity : usize) -> usize);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeSetSystemGroupAffinityThread(affinity : *const super::super::super::Win32::System::SystemInformation:: GROUP_AFFINITY, previousaffinity : *mut super::super::super::Win32::System::SystemInformation:: GROUP_AFFINITY));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeSetTargetProcessorDpc(dpc : *mut super::super::Foundation:: KDPC, number : i8));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeSetTargetProcessorDpcEx(dpc : *mut super::super::Foundation:: KDPC, procnumber : *const super::super::super::Win32::System::Kernel:: PROCESSOR_NUMBER) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeSetTimer(timer : *mut KTIMER, duetime : i64, dpc : *const super::super::Foundation:: KDPC) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeSetTimerEx(timer : *mut KTIMER, duetime : i64, period : i32, dpc : *const super::super::Foundation:: KDPC) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn KeShouldYieldProcessor() -> u32);
windows_targets::link!("hal.dll" "system" fn KeStallExecutionProcessor(microseconds : u32));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn KeSynchronizeExecution(interrupt : super::super::Foundation:: PKINTERRUPT, synchronizeroutine : PKSYNCHRONIZE_ROUTINE, synchronizecontext : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn KeTestSpinLock(spinlock : *const usize) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeTryToAcquireGuardedMutex(mutex : *mut super::super::Foundation:: FAST_MUTEX) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn KeTryToAcquireSpinLockAtDpcLevel(spinlock : *mut usize) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntoskrnl.exe" "system" fn KeWaitForMultipleObjects(count : u32, object : *const *const core::ffi::c_void, waittype : super::super::super::Win32::System::Kernel:: WAIT_TYPE, waitreason : KWAIT_REASON, waitmode : i8, alertable : super::super::super::Win32::Foundation:: BOOLEAN, timeout : *const i64, waitblockarray : *mut super::super::Foundation:: KWAIT_BLOCK) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KeWaitForSingleObject(object : *const core::ffi::c_void, waitreason : KWAIT_REASON, waitmode : i8, alertable : super::super::super::Win32::Foundation:: BOOLEAN, timeout : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn KfRaiseIrql(newirql : u8) -> u8);
windows_targets::link!("ntoskrnl.exe" "system" fn MmAddPhysicalMemory(startaddress : *const i64, numberofbytes : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmAddVerifierSpecialThunks(entryroutine : usize, thunkbuffer : *const core::ffi::c_void, thunkbuffersize : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmAddVerifierThunks(thunkbuffer : *const core::ffi::c_void, thunkbuffersize : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmAdvanceMdl(mdl : *mut super::super::Foundation:: MDL, numberofbytes : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmAllocateContiguousMemory(numberofbytes : usize, highestacceptableaddress : i64) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn MmAllocateContiguousMemoryEx(numberofbytes : *const usize, lowestacceptableaddress : i64, highestacceptableaddress : i64, boundaryaddressmultiple : i64, preferrednode : u32, protect : u32, partitionobject : *const core::ffi::c_void, tag : u32, flags : u32, baseaddress : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmAllocateContiguousMemorySpecifyCache(numberofbytes : usize, lowestacceptableaddress : i64, highestacceptableaddress : i64, boundaryaddressmultiple : i64, cachetype : MEMORY_CACHING_TYPE) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn MmAllocateContiguousMemorySpecifyCacheNode(numberofbytes : usize, lowestacceptableaddress : i64, highestacceptableaddress : i64, boundaryaddressmultiple : i64, cachetype : MEMORY_CACHING_TYPE, preferrednode : u32) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn MmAllocateContiguousNodeMemory(numberofbytes : usize, lowestacceptableaddress : i64, highestacceptableaddress : i64, boundaryaddressmultiple : i64, protect : u32, preferrednode : u32) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn MmAllocateMappingAddress(numberofbytes : usize, pooltag : u32) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn MmAllocateMappingAddressEx(numberofbytes : usize, pooltag : u32, flags : u32) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmAllocateMdlForIoSpace(physicaladdresslist : *const MM_PHYSICAL_ADDRESS_LIST, numberofentries : usize, newmdl : *mut *mut super::super::Foundation:: MDL) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmAllocateNodePagesForMdlEx(lowaddress : i64, highaddress : i64, skipbytes : i64, totalbytes : usize, cachetype : MEMORY_CACHING_TYPE, idealnode : u32, flags : u32) -> *mut super::super::Foundation:: MDL);
windows_targets::link!("ntoskrnl.exe" "system" fn MmAllocateNonCachedMemory(numberofbytes : usize) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmAllocatePagesForMdl(lowaddress : i64, highaddress : i64, skipbytes : i64, totalbytes : usize) -> *mut super::super::Foundation:: MDL);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmAllocatePagesForMdlEx(lowaddress : i64, highaddress : i64, skipbytes : i64, totalbytes : usize, cachetype : MEMORY_CACHING_TYPE, flags : u32) -> *mut super::super::Foundation:: MDL);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmAllocatePartitionNodePagesForMdlEx(lowaddress : i64, highaddress : i64, skipbytes : i64, totalbytes : usize, cachetype : MEMORY_CACHING_TYPE, idealnode : u32, flags : u32, partitionobject : *const core::ffi::c_void) -> *mut super::super::Foundation:: MDL);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmAreMdlPagesCached(memorydescriptorlist : *const super::super::Foundation:: MDL) -> u32);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmBuildMdlForNonPagedPool(memorydescriptorlist : *mut super::super::Foundation:: MDL));
windows_targets::link!("ntoskrnl.exe" "system" fn MmCopyMemory(targetaddress : *const core::ffi::c_void, sourceaddress : MM_COPY_ADDRESS, numberofbytes : usize, flags : u32, numberofbytestransferred : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmCreateMdl(memorydescriptorlist : *mut super::super::Foundation:: MDL, base : *const core::ffi::c_void, length : usize) -> *mut super::super::Foundation:: MDL);
windows_targets::link!("ntoskrnl.exe" "system" fn MmCreateMirror() -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmFreeContiguousMemory(baseaddress : *const core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn MmFreeContiguousMemorySpecifyCache(baseaddress : *const core::ffi::c_void, numberofbytes : usize, cachetype : MEMORY_CACHING_TYPE));
windows_targets::link!("ntoskrnl.exe" "system" fn MmFreeMappingAddress(baseaddress : *const core::ffi::c_void, pooltag : u32));
windows_targets::link!("ntoskrnl.exe" "system" fn MmFreeNonCachedMemory(baseaddress : *const core::ffi::c_void, numberofbytes : usize));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmFreePagesFromMdl(memorydescriptorlist : *mut super::super::Foundation:: MDL));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmFreePagesFromMdlEx(memorydescriptorlist : *mut super::super::Foundation:: MDL, flags : u32));
windows_targets::link!("ntoskrnl.exe" "system" fn MmGetCacheAttribute(physicaladdress : i64, cachetype : *mut MEMORY_CACHING_TYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmGetCacheAttributeEx(physicaladdress : i64, flags : u32, cachetype : *mut MEMORY_CACHING_TYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmGetPhysicalAddress(baseaddress : *const core::ffi::c_void) -> i64);
windows_targets::link!("ntoskrnl.exe" "system" fn MmGetPhysicalMemoryRanges() -> *mut PHYSICAL_MEMORY_RANGE);
windows_targets::link!("ntoskrnl.exe" "system" fn MmGetPhysicalMemoryRangesEx(partitionobject : *const core::ffi::c_void) -> *mut PHYSICAL_MEMORY_RANGE);
windows_targets::link!("ntoskrnl.exe" "system" fn MmGetPhysicalMemoryRangesEx2(partitionobject : *const core::ffi::c_void, flags : u32) -> *mut PHYSICAL_MEMORY_RANGE);
windows_targets::link!("ntoskrnl.exe" "system" fn MmGetSystemRoutineAddress(systemroutinename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn MmGetVirtualForPhysical(physicaladdress : i64) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn MmIsAddressValid(virtualaddress : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn MmIsDriverSuspectForVerifier(driverobject : *const super::super::Foundation:: DRIVER_OBJECT) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn MmIsDriverVerifying(driverobject : *const super::super::Foundation:: DRIVER_OBJECT) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn MmIsDriverVerifyingByAddress(addresswithinsection : *const core::ffi::c_void) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn MmIsIoSpaceActive(startaddress : i64, numberofbytes : usize) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn MmIsNonPagedSystemAddressValid(virtualaddress : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn MmIsThisAnNtAsSystem() -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn MmIsVerifierEnabled(verifierflags : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmLockPagableDataSection(addresswithinsection : *const core::ffi::c_void) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn MmLockPagableSectionByHandle(imagesectionhandle : *const core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn MmMapIoSpace(physicaladdress : i64, numberofbytes : usize, cachetype : MEMORY_CACHING_TYPE) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn MmMapIoSpaceEx(physicaladdress : i64, numberofbytes : usize, protect : u32) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmMapLockedPages(memorydescriptorlist : *mut super::super::Foundation:: MDL, accessmode : i8) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmMapLockedPagesSpecifyCache(memorydescriptorlist : *mut super::super::Foundation:: MDL, accessmode : i8, cachetype : MEMORY_CACHING_TYPE, requestedaddress : *const core::ffi::c_void, bugcheckonfailure : u32, priority : u32) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmMapLockedPagesWithReservedMapping(mappingaddress : *const core::ffi::c_void, pooltag : u32, memorydescriptorlist : *mut super::super::Foundation:: MDL, cachetype : MEMORY_CACHING_TYPE) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmMapMdl(memorydescriptorlist : *mut super::super::Foundation:: MDL, protection : u32, driverroutine : PMM_MDL_ROUTINE, drivercontext : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmMapMemoryDumpMdlEx(va : *const core::ffi::c_void, pagetotal : u32, memorydumpmdl : *mut super::super::Foundation:: MDL, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmMapUserAddressesToPage(baseaddress : *const core::ffi::c_void, numberofbytes : usize, pageaddress : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmMapVideoDisplay(physicaladdress : i64, numberofbytes : usize, cachetype : MEMORY_CACHING_TYPE) -> *mut core::ffi::c_void);
windows_targets::link!("ntoskrnl.exe" "system" fn MmMapViewInSessionSpace(section : *const core::ffi::c_void, mappedbase : *mut *mut core::ffi::c_void, viewsize : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmMapViewInSessionSpaceEx(section : *const core::ffi::c_void, mappedbase : *mut *mut core::ffi::c_void, viewsize : *mut usize, sectionoffset : *mut i64, flags : usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmMapViewInSystemSpace(section : *const core::ffi::c_void, mappedbase : *mut *mut core::ffi::c_void, viewsize : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmMapViewInSystemSpaceEx(section : *const core::ffi::c_void, mappedbase : *mut *mut core::ffi::c_void, viewsize : *mut usize, sectionoffset : *mut i64, flags : usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmMdlPageContentsState(memorydescriptorlist : *mut super::super::Foundation:: MDL, state : MM_MDL_PAGE_CONTENTS_STATE) -> MM_MDL_PAGE_CONTENTS_STATE);
windows_targets::link!("ntoskrnl.exe" "system" fn MmPageEntireDriver(addresswithinsection : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmProbeAndLockPages(memorydescriptorlist : *mut super::super::Foundation:: MDL, accessmode : i8, operation : LOCK_OPERATION));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmProbeAndLockProcessPages(memorydescriptorlist : *mut super::super::Foundation:: MDL, process : super::super::Foundation:: PEPROCESS, accessmode : i8, operation : LOCK_OPERATION));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Storage_FileSystem"))]
windows_targets::link!("ntoskrnl.exe" "system" fn MmProbeAndLockSelectedPages(memorydescriptorlist : *mut super::super::Foundation:: MDL, segmentarray : *const super::super::super::Win32::Storage::FileSystem:: FILE_SEGMENT_ELEMENT, accessmode : i8, operation : LOCK_OPERATION));
windows_targets::link!("ntoskrnl.exe" "system" fn MmProtectDriverSection(addresswithinsection : *const core::ffi::c_void, size : usize, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmProtectMdlSystemAddress(memorydescriptorlist : *const super::super::Foundation:: MDL, newprotect : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmQuerySystemSize() -> MM_SYSTEMSIZE);
windows_targets::link!("ntoskrnl.exe" "system" fn MmRemovePhysicalMemory(startaddress : *const i64, numberofbytes : *mut i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmResetDriverPaging(addresswithinsection : *const core::ffi::c_void));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmRotatePhysicalView(virtualaddress : *const core::ffi::c_void, numberofbytes : *mut usize, newmdl : *const super::super::Foundation:: MDL, direction : MM_ROTATE_DIRECTION, copyfunction : PMM_ROTATE_COPY_CALLBACK_FUNCTION, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmSecureVirtualMemory(address : *const core::ffi::c_void, size : usize, probemode : u32) -> super::super::super::Win32::Foundation:: HANDLE);
windows_targets::link!("ntoskrnl.exe" "system" fn MmSecureVirtualMemoryEx(address : *const core::ffi::c_void, size : usize, probemode : u32, flags : u32) -> super::super::super::Win32::Foundation:: HANDLE);
windows_targets::link!("ntoskrnl.exe" "system" fn MmSetPermanentCacheAttribute(startaddress : i64, numberofbytes : i64, cachetype : MEMORY_CACHING_TYPE, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmSizeOfMdl(base : *const core::ffi::c_void, length : usize) -> usize);
windows_targets::link!("ntoskrnl.exe" "system" fn MmUnlockPagableImageSection(imagesectionhandle : *const core::ffi::c_void));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmUnlockPages(memorydescriptorlist : *mut super::super::Foundation:: MDL));
windows_targets::link!("ntoskrnl.exe" "system" fn MmUnmapIoSpace(baseaddress : *const core::ffi::c_void, numberofbytes : usize));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmUnmapLockedPages(baseaddress : *const core::ffi::c_void, memorydescriptorlist : *mut super::super::Foundation:: MDL));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn MmUnmapReservedMapping(baseaddress : *const core::ffi::c_void, pooltag : u32, memorydescriptorlist : *mut super::super::Foundation:: MDL));
windows_targets::link!("ntoskrnl.exe" "system" fn MmUnmapVideoDisplay(baseaddress : *const core::ffi::c_void, numberofbytes : usize));
windows_targets::link!("ntoskrnl.exe" "system" fn MmUnmapViewInSessionSpace(mappedbase : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmUnmapViewInSystemSpace(mappedbase : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn MmUnsecureVirtualMemory(securehandle : super::super::super::Win32::Foundation:: HANDLE));
windows_targets::link!("ntdll.dll" "system" fn NtAllocateLocallyUniqueId(luid : *mut super::super::super::Win32::Foundation:: LUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtCommitComplete(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtCommitEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtCommitTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn NtCreateEnlistment(enlistmenthandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE, transactionhandle : super::super::super::Win32::Foundation:: HANDLE, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, createoptions : u32, notificationmask : u32, enlistmentkey : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn NtCreateResourceManager(resourcemanagerhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, tmhandle : super::super::super::Win32::Foundation:: HANDLE, rmguid : *const windows_sys::core::GUID, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, createoptions : u32, description : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn NtCreateTransaction(transactionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, uow : *const windows_sys::core::GUID, tmhandle : super::super::super::Win32::Foundation:: HANDLE, createoptions : u32, isolationlevel : u32, isolationflags : u32, timeout : *const i64, description : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn NtCreateTransactionManager(tmhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, logfilename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, createoptions : u32, commitstrength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtDisplayString(string : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn NtEnumerateTransactionObject(rootobjecthandle : super::super::super::Win32::Foundation:: HANDLE, querytype : super::super::super::Win32::System::SystemServices:: KTMOBJECT_TYPE, objectcursor : *mut super::super::super::Win32::System::SystemServices:: KTMOBJECT_CURSOR, objectcursorlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("ntdll.dll" "system" fn NtGetNotificationResourceManager(resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE, transactionnotification : *mut super::super::super::Win32::Storage::FileSystem:: TRANSACTION_NOTIFICATION, notificationlength : u32, timeout : *const i64, returnlength : *mut u32, asynchronous : u32, asynchronouscontext : usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtLoadDriver(driverservicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtMakeTemporaryObject(handle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtManagePartition(targethandle : super::super::super::Win32::Foundation:: HANDLE, sourcehandle : super::super::super::Win32::Foundation:: HANDLE, partitioninformationclass : PARTITION_INFORMATION_CLASS, partitioninformation : *mut core::ffi::c_void, partitioninformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn NtOpenEnlistment(enlistmenthandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE, enlistmentguid : *const windows_sys::core::GUID, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn NtOpenResourceManager(resourcemanagerhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, tmhandle : super::super::super::Win32::Foundation:: HANDLE, resourcemanagerguid : *const windows_sys::core::GUID, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn NtOpenTransaction(transactionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, uow : *const windows_sys::core::GUID, tmhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn NtOpenTransactionManager(tmhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, logfilename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, tmidentity : *const windows_sys::core::GUID, openoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Power")]
windows_targets::link!("ntdll.dll" "system" fn NtPowerInformation(informationlevel : super::super::super::Win32::System::Power:: POWER_INFORMATION_LEVEL, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtPrePrepareComplete(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtPrePrepareEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtPrepareComplete(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtPrepareEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtPropagationComplete(resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE, requestcookie : u32, bufferlength : u32, buffer : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtPropagationFailed(resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE, requestcookie : u32, propstatus : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn NtQueryInformationEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, enlistmentinformationclass : super::super::super::Win32::System::SystemServices:: ENLISTMENT_INFORMATION_CLASS, enlistmentinformation : *mut core::ffi::c_void, enlistmentinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn NtQueryInformationResourceManager(resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE, resourcemanagerinformationclass : super::super::super::Win32::System::SystemServices:: RESOURCEMANAGER_INFORMATION_CLASS, resourcemanagerinformation : *mut core::ffi::c_void, resourcemanagerinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn NtQueryInformationTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, transactioninformationclass : super::super::super::Win32::System::SystemServices:: TRANSACTION_INFORMATION_CLASS, transactioninformation : *mut core::ffi::c_void, transactioninformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn NtQueryInformationTransactionManager(transactionmanagerhandle : super::super::super::Win32::Foundation:: HANDLE, transactionmanagerinformationclass : super::super::super::Win32::System::SystemServices:: TRANSACTIONMANAGER_INFORMATION_CLASS, transactionmanagerinformation : *mut core::ffi::c_void, transactionmanagerinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtReadOnlyEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtRecoverEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, enlistmentkey : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtRecoverResourceManager(resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtRecoverTransactionManager(transactionmanagerhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtRegisterProtocolAddressInformation(resourcemanager : super::super::super::Win32::Foundation:: HANDLE, protocolid : *const windows_sys::core::GUID, protocolinformationsize : u32, protocolinformation : *const core::ffi::c_void, createoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtRenameTransactionManager(logfilename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, existingtransactionmanagerguid : *const windows_sys::core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtRollbackComplete(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtRollbackEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtRollbackTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtRollforwardTransactionManager(transactionmanagerhandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn NtSetInformationEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, enlistmentinformationclass : super::super::super::Win32::System::SystemServices:: ENLISTMENT_INFORMATION_CLASS, enlistmentinformation : *const core::ffi::c_void, enlistmentinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn NtSetInformationResourceManager(resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE, resourcemanagerinformationclass : super::super::super::Win32::System::SystemServices:: RESOURCEMANAGER_INFORMATION_CLASS, resourcemanagerinformation : *const core::ffi::c_void, resourcemanagerinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn NtSetInformationTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, transactioninformationclass : super::super::super::Win32::System::SystemServices:: TRANSACTION_INFORMATION_CLASS, transactioninformation : *const core::ffi::c_void, transactioninformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn NtSetInformationTransactionManager(tmhandle : super::super::super::Win32::Foundation:: HANDLE, transactionmanagerinformationclass : super::super::super::Win32::System::SystemServices:: TRANSACTIONMANAGER_INFORMATION_CLASS, transactionmanagerinformation : *const core::ffi::c_void, transactionmanagerinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtSinglePhaseReject(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn NtUnloadDriver(driverservicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn ObCloseHandle(handle : super::super::super::Win32::Foundation:: HANDLE, previousmode : i8) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn ObDereferenceObjectDeferDelete(object : *const core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn ObDereferenceObjectDeferDeleteWithTag(object : *const core::ffi::c_void, tag : u32));
windows_targets::link!("ntoskrnl.exe" "system" fn ObGetFilterVersion() -> u16);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("ntoskrnl.exe" "system" fn ObGetObjectSecurity(object : *const core::ffi::c_void, securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, memoryallocated : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ObReferenceObjectByHandle(handle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objecttype : super::super::Foundation:: POBJECT_TYPE, accessmode : i8, object : *mut *mut core::ffi::c_void, handleinformation : *mut OBJECT_HANDLE_INFORMATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ObReferenceObjectByHandleWithTag(handle : super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objecttype : super::super::Foundation:: POBJECT_TYPE, accessmode : i8, tag : u32, object : *mut *mut core::ffi::c_void, handleinformation : *mut OBJECT_HANDLE_INFORMATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ObReferenceObjectByPointer(object : *const core::ffi::c_void, desiredaccess : u32, objecttype : super::super::Foundation:: POBJECT_TYPE, accessmode : i8) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ObReferenceObjectByPointerWithTag(object : *const core::ffi::c_void, desiredaccess : u32, objecttype : super::super::Foundation:: POBJECT_TYPE, accessmode : i8, tag : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn ObReferenceObjectSafe(object : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn ObReferenceObjectSafeWithTag(object : *const core::ffi::c_void, tag : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn ObRegisterCallbacks(callbackregistration : *const OB_CALLBACK_REGISTRATION, registrationhandle : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("ntoskrnl.exe" "system" fn ObReleaseObjectSecurity(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, memoryallocated : super::super::super::Win32::Foundation:: BOOLEAN));
windows_targets::link!("ntoskrnl.exe" "system" fn ObUnRegisterCallbacks(registrationhandle : *const core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn ObfDereferenceObject(object : *const core::ffi::c_void) -> isize);
windows_targets::link!("ntoskrnl.exe" "system" fn ObfDereferenceObjectWithTag(object : *const core::ffi::c_void, tag : u32) -> isize);
windows_targets::link!("ntoskrnl.exe" "system" fn ObfReferenceObject(object : *const core::ffi::c_void) -> isize);
windows_targets::link!("ntoskrnl.exe" "system" fn ObfReferenceObjectWithTag(object : *const core::ffi::c_void, tag : u32) -> isize);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PcwAddInstance(buffer : super::super::Foundation:: PPCW_BUFFER, name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, id : u32, count : u32, data : *const PCW_DATA) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PcwCloseInstance(instance : super::super::Foundation:: PPCW_INSTANCE));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PcwCreateInstance(instance : *mut super::super::Foundation:: PPCW_INSTANCE, registration : super::super::Foundation:: PPCW_REGISTRATION, name : *const super::super::super::Win32::Foundation:: UNICODE_STRING, count : u32, data : *const PCW_DATA) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PcwRegister(registration : *mut super::super::Foundation:: PPCW_REGISTRATION, info : *const PCW_REGISTRATION_INFORMATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PcwUnregister(registration : super::super::Foundation:: PPCW_REGISTRATION));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoCallDriver(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, irp : *mut super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Power")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoClearPowerRequest(powerrequest : *mut core::ffi::c_void, r#type : super::super::super::Win32::System::Power:: POWER_REQUEST_TYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoCreatePowerRequest(powerrequest : *mut *mut core::ffi::c_void, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, context : *const COUNTED_REASON_CONTEXT) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoCreateThermalRequest(thermalrequest : *mut *mut core::ffi::c_void, targetdeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, policydeviceobject : *const super::super::Foundation:: DEVICE_OBJECT, context : *const COUNTED_REASON_CONTEXT, flags : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PoDeletePowerRequest(powerrequest : *mut core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn PoDeleteThermalRequest(thermalrequest : *mut core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn PoEndDeviceBusy(idlepointer : *mut u32));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxActivateComponent(handle : super::super::Foundation:: POHANDLE, component : u32, flags : u32));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxCompleteDevicePowerNotRequired(handle : super::super::Foundation:: POHANDLE));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxCompleteDirectedPowerDown(handle : super::super::Foundation:: POHANDLE));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxCompleteIdleCondition(handle : super::super::Foundation:: POHANDLE, component : u32));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxCompleteIdleState(handle : super::super::Foundation:: POHANDLE, component : u32));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxIdleComponent(handle : super::super::Foundation:: POHANDLE, component : u32, flags : u32));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxIssueComponentPerfStateChange(handle : super::super::Foundation:: POHANDLE, flags : u32, component : u32, perfchange : *const PO_FX_PERF_STATE_CHANGE, context : *const core::ffi::c_void));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxIssueComponentPerfStateChangeMultiple(handle : super::super::Foundation:: POHANDLE, flags : u32, component : u32, perfchangescount : u32, perfchanges : *const PO_FX_PERF_STATE_CHANGE, context : *const core::ffi::c_void));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxNotifySurprisePowerOn(pdo : *const super::super::Foundation:: DEVICE_OBJECT));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxPowerControl(handle : super::super::Foundation:: POHANDLE, powercontrolcode : *const windows_sys::core::GUID, inbuffer : *const core::ffi::c_void, inbuffersize : usize, outbuffer : *mut core::ffi::c_void, outbuffersize : usize, bytesreturned : *mut usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxPowerOnCrashdumpDevice(handle : super::super::Foundation:: POHANDLE, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxQueryCurrentComponentPerfState(handle : super::super::Foundation:: POHANDLE, flags : u32, component : u32, setindex : u32, currentperf : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxRegisterComponentPerfStates(handle : super::super::Foundation:: POHANDLE, component : u32, flags : u64, componentperfstatecallback : PPO_FX_COMPONENT_PERF_STATE_CALLBACK, inputstateinfo : *const PO_FX_COMPONENT_PERF_INFO, outputstateinfo : *mut *mut PO_FX_COMPONENT_PERF_INFO) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxRegisterCrashdumpDevice(handle : super::super::Foundation:: POHANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxRegisterDevice(pdo : *const super::super::Foundation:: DEVICE_OBJECT, device : *const PO_FX_DEVICE_V1, handle : *mut super::super::Foundation:: POHANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxRegisterDripsWatchdogCallback(handle : super::super::Foundation:: POHANDLE, callback : PPO_FX_DRIPS_WATCHDOG_CALLBACK, includechilddevices : super::super::super::Win32::Foundation:: BOOLEAN, matchingdriverobject : *const super::super::Foundation:: DRIVER_OBJECT));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxReportDevicePoweredOn(handle : super::super::Foundation:: POHANDLE));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxSetComponentLatency(handle : super::super::Foundation:: POHANDLE, component : u32, latency : u64));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxSetComponentResidency(handle : super::super::Foundation:: POHANDLE, component : u32, residency : u64));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxSetComponentWake(handle : super::super::Foundation:: POHANDLE, component : u32, wakehint : super::super::super::Win32::Foundation:: BOOLEAN));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxSetDeviceIdleTimeout(handle : super::super::Foundation:: POHANDLE, idletimeout : u64));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxSetTargetDripsDevicePowerState(handle : super::super::Foundation:: POHANDLE, targetstate : super::super::super::Win32::System::Power:: DEVICE_POWER_STATE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxStartDevicePowerManagement(handle : super::super::Foundation:: POHANDLE));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoFxUnregisterDevice(handle : super::super::Foundation:: POHANDLE));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoGetSystemWake(irp : *const super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn PoGetThermalRequestSupport(thermalrequest : *const core::ffi::c_void, r#type : PO_THERMAL_REQUEST_TYPE) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoQueryWatchdogTime(pdo : *const super::super::Foundation:: DEVICE_OBJECT, secondsremaining : *mut u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoRegisterDeviceForIdleDetection(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, conservationidletime : u32, performanceidletime : u32, state : super::super::super::Win32::System::Power:: DEVICE_POWER_STATE) -> *mut u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoRegisterPowerSettingCallback(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, settingguid : *const windows_sys::core::GUID, callback : PPOWER_SETTING_CALLBACK, context : *const core::ffi::c_void, handle : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PoRegisterSystemState(statehandle : *mut core::ffi::c_void, flags : u32) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoRequestPowerIrp(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, minorfunction : u8, powerstate : POWER_STATE, completionfunction : PREQUEST_POWER_COMPLETE, context : *const core::ffi::c_void, irp : *mut *mut super::super::Foundation:: IRP) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PoSetDeviceBusyEx(idlepointer : *mut u32));
windows_targets::link!("ntoskrnl.exe" "system" fn PoSetHiberRange(memorymap : *const core::ffi::c_void, flags : u32, address : *const core::ffi::c_void, length : usize, tag : u32));
#[cfg(feature = "Win32_System_Power")]
windows_targets::link!("ntoskrnl.exe" "system" fn PoSetPowerRequest(powerrequest : *mut core::ffi::c_void, r#type : super::super::super::Win32::System::Power:: POWER_REQUEST_TYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoSetPowerState(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, r#type : POWER_STATE_TYPE, state : POWER_STATE) -> POWER_STATE);
windows_targets::link!("ntoskrnl.exe" "system" fn PoSetSystemState(flags : u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoSetSystemWake(irp : *mut super::super::Foundation:: IRP));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoSetSystemWakeDevice(deviceobject : *const super::super::Foundation:: DEVICE_OBJECT));
windows_targets::link!("ntoskrnl.exe" "system" fn PoSetThermalActiveCooling(thermalrequest : *mut core::ffi::c_void, engaged : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PoSetThermalPassiveCooling(thermalrequest : *mut core::ffi::c_void, throttle : u8) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PoStartDeviceBusy(idlepointer : *mut u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PoStartNextPowerIrp(irp : *mut super::super::Foundation:: IRP));
windows_targets::link!("ntoskrnl.exe" "system" fn PoUnregisterPowerSettingCallback(handle : *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PoUnregisterSystemState(statehandle : *mut core::ffi::c_void));
windows_targets::link!("ntoskrnl.exe" "system" fn ProbeForRead(address : *const core::ffi::c_void, length : usize, alignment : u32));
windows_targets::link!("ntoskrnl.exe" "system" fn ProbeForWrite(address : *mut core::ffi::c_void, length : usize, alignment : u32));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsAcquireSiloHardReference(silo : super::super::Foundation:: PESILO) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PsAllocSiloContextSlot(reserved : usize, returnedcontextslot : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsAllocateAffinityToken(affinitytoken : *mut super::super::Foundation:: PAFFINITY_TOKEN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsAttachSiloToCurrentThread(silo : super::super::Foundation:: PESILO) -> super::super::Foundation:: PESILO);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsCreateSiloContext(silo : super::super::Foundation:: PESILO, size : u32, pooltype : super::super::Foundation:: POOL_TYPE, contextcleanupcallback : SILO_CONTEXT_CLEANUP_CALLBACK, returnedsilocontext : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_WindowsProgramming"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PsCreateSystemThread(threadhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, processhandle : super::super::super::Win32::Foundation:: HANDLE, clientid : *mut super::super::super::Win32::System::WindowsProgramming:: CLIENT_ID, startroutine : PKSTART_ROUTINE, startcontext : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PsDereferenceSiloContext(silocontext : *const core::ffi::c_void));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsDetachSiloFromCurrentThread(previoussilo : super::super::Foundation:: PESILO));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsFreeAffinityToken(affinitytoken : super::super::Foundation:: PAFFINITY_TOKEN));
windows_targets::link!("ntoskrnl.exe" "system" fn PsFreeSiloContextSlot(contextslot : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetCurrentProcessId() -> super::super::super::Win32::Foundation:: HANDLE);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetCurrentServerSilo() -> super::super::Foundation:: PESILO);
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetCurrentServerSiloName() -> *mut super::super::super::Win32::Foundation:: UNICODE_STRING);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetCurrentSilo() -> super::super::Foundation:: PESILO);
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetCurrentThreadId() -> super::super::super::Win32::Foundation:: HANDLE);
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetCurrentThreadTeb() -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetEffectiveServerSilo(silo : super::super::Foundation:: PESILO) -> super::super::Foundation:: PESILO);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetHostSilo() -> super::super::Foundation:: PESILO);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetJobServerSilo(job : super::super::Foundation:: PEJOB, serversilo : *mut super::super::Foundation:: PESILO) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetJobSilo(job : super::super::Foundation:: PEJOB, silo : *mut super::super::Foundation:: PESILO) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetParentSilo(job : super::super::Foundation:: PEJOB) -> super::super::Foundation:: PESILO);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetPermanentSiloContext(silo : super::super::Foundation:: PESILO, contextslot : u32, returnedsilocontext : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetProcessCreateTimeQuadPart(process : super::super::Foundation:: PEPROCESS) -> i64);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetProcessExitStatus(process : super::super::Foundation:: PEPROCESS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetProcessId(process : super::super::Foundation:: PEPROCESS) -> super::super::super::Win32::Foundation:: HANDLE);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetProcessStartKey(process : super::super::Foundation:: PEPROCESS) -> u64);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetServerSiloServiceSessionId(silo : super::super::Foundation:: PESILO) -> u32);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetSiloContainerId(silo : super::super::Foundation:: PESILO) -> *mut windows_sys::core::GUID);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetSiloContext(silo : super::super::Foundation:: PESILO, contextslot : u32, returnedsilocontext : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetSiloMonitorContextSlot(monitor : super::super::Foundation:: PSILO_MONITOR) -> u32);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetThreadCreateTime(thread : super::super::Foundation:: PETHREAD) -> i64);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetThreadExitStatus(thread : super::super::Foundation:: PETHREAD) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetThreadId(thread : super::super::Foundation:: PETHREAD) -> super::super::super::Win32::Foundation:: HANDLE);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetThreadProcessId(thread : super::super::Foundation:: PETHREAD) -> super::super::super::Win32::Foundation:: HANDLE);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetThreadProperty(thread : super::super::Foundation:: PETHREAD, key : usize, flags : u32) -> *mut core::ffi::c_void);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetThreadServerSilo(thread : super::super::Foundation:: PETHREAD) -> super::super::Foundation:: PESILO);
windows_targets::link!("ntoskrnl.exe" "system" fn PsGetVersion(majorversion : *mut u32, minorversion : *mut u32, buildnumber : *mut u32, csdversion : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsInsertPermanentSiloContext(silo : super::super::Foundation:: PESILO, contextslot : u32, silocontext : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsInsertSiloContext(silo : super::super::Foundation:: PESILO, contextslot : u32, silocontext : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PsIsCurrentThreadInServerSilo() -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn PsIsCurrentThreadPrefetching() -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsIsHostSilo(silo : super::super::Foundation:: PESILO) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsMakeSiloContextPermanent(silo : super::super::Foundation:: PESILO, contextslot : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsQueryTotalCycleTimeProcess(process : super::super::Foundation:: PEPROCESS, cycletimestamp : *mut u64) -> u64);
windows_targets::link!("ntoskrnl.exe" "system" fn PsReferenceSiloContext(silocontext : *const core::ffi::c_void));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsRegisterSiloMonitor(registration : *const SILO_MONITOR_REGISTRATION, returnedmonitor : *mut super::super::Foundation:: PSILO_MONITOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsReleaseSiloHardReference(silo : super::super::Foundation:: PESILO));
windows_targets::link!("ntoskrnl.exe" "system" fn PsRemoveCreateThreadNotifyRoutine(notifyroutine : PCREATE_THREAD_NOTIFY_ROUTINE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PsRemoveLoadImageNotifyRoutine(notifyroutine : PLOAD_IMAGE_NOTIFY_ROUTINE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsRemoveSiloContext(silo : super::super::Foundation:: PESILO, contextslot : u32, removedsilocontext : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsReplaceSiloContext(silo : super::super::Foundation:: PESILO, contextslot : u32, newsilocontext : *const core::ffi::c_void, oldsilocontext : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsRevertToUserMultipleGroupAffinityThread(affinitytoken : super::super::Foundation:: PAFFINITY_TOKEN));
windows_targets::link!("ntoskrnl.exe" "system" fn PsSetCreateProcessNotifyRoutine(notifyroutine : PCREATE_PROCESS_NOTIFY_ROUTINE, remove : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PsSetCreateProcessNotifyRoutineEx(notifyroutine : PCREATE_PROCESS_NOTIFY_ROUTINE_EX, remove : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PsSetCreateProcessNotifyRoutineEx2(notifytype : PSCREATEPROCESSNOTIFYTYPE, notifyinformation : *const core::ffi::c_void, remove : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PsSetCreateThreadNotifyRoutine(notifyroutine : PCREATE_THREAD_NOTIFY_ROUTINE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PsSetCreateThreadNotifyRoutineEx(notifytype : PSCREATETHREADNOTIFYTYPE, notifyinformation : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PsSetCurrentThreadPrefetching(prefetching : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn PsSetLoadImageNotifyRoutine(notifyroutine : PLOAD_IMAGE_NOTIFY_ROUTINE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn PsSetLoadImageNotifyRoutineEx(notifyroutine : PLOAD_IMAGE_NOTIFY_ROUTINE, flags : usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_SystemInformation"))]
windows_targets::link!("ntoskrnl.exe" "system" fn PsSetSystemMultipleGroupAffinityThread(groupaffinities : *const super::super::super::Win32::System::SystemInformation:: GROUP_AFFINITY, groupcount : u16, affinitytoken : super::super::Foundation:: PAFFINITY_TOKEN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsStartSiloMonitor(monitor : super::super::Foundation:: PSILO_MONITOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsTerminateServerSilo(serversilo : super::super::Foundation:: PESILO, exitstatus : super::super::super::Win32::Foundation:: NTSTATUS));
windows_targets::link!("ntoskrnl.exe" "system" fn PsTerminateSystemThread(exitstatus : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn PsUnregisterSiloMonitor(monitor : super::super::Foundation:: PSILO_MONITOR));
windows_targets::link!("ntoskrnl.exe" "system" fn PsWrapApcWow64Thread(apccontext : *mut *mut core::ffi::c_void, apcroutine : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("pshed.dll" "system" fn PshedAllocateMemory(size : u32) -> *mut core::ffi::c_void);
windows_targets::link!("pshed.dll" "system" fn PshedFreeMemory(address : *const core::ffi::c_void));
windows_targets::link!("pshed.dll" "system" fn PshedIsSystemWheaEnabled() -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("pshed.dll" "system" fn PshedRegisterPlugin(packet : *mut WHEA_PSHED_PLUGIN_REGISTRATION_PACKET_V2) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("pshed.dll" "system" fn PshedSynchronizeExecution(errorsource : *const super::super::super::Win32::System::Diagnostics::Debug:: WHEA_ERROR_SOURCE_DESCRIPTOR, synchronizeroutine : PKSYNCHRONIZE_ROUTINE, synchronizecontext : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("pshed.dll" "system" fn PshedUnregisterPlugin(pluginhandle : *const core::ffi::c_void));
windows_targets::link!("ntdll.dll" "system" fn RtlAppendUnicodeStringToString(destination : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, source : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlAppendUnicodeToString(destination : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, source : windows_sys::core::PCWSTR) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlAreBitsClear(bitmapheader : *const RTL_BITMAP, startingindex : u32, length : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlAreBitsSet(bitmapheader : *const RTL_BITMAP, startingindex : u32, length : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlAssert(voidfailedassertion : *const core::ffi::c_void, voidfilename : *const core::ffi::c_void, linenumber : u32, mutablemessage : windows_sys::core::PCSTR));
windows_targets::link!("ntdll.dll" "system" fn RtlCheckRegistryKey(relativeto : u32, path : windows_sys::core::PCWSTR) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlClearAllBits(bitmapheader : *const RTL_BITMAP));
windows_targets::link!("ntdll.dll" "system" fn RtlClearBit(bitmapheader : *const RTL_BITMAP, bitnumber : u32));
windows_targets::link!("ntdll.dll" "system" fn RtlClearBits(bitmapheader : *const RTL_BITMAP, startingindex : u32, numbertoclear : u32));
windows_targets::link!("ntdll.dll" "system" fn RtlCmDecodeMemIoResource(descriptor : *const CM_PARTIAL_RESOURCE_DESCRIPTOR, start : *mut u64) -> u64);
windows_targets::link!("ntdll.dll" "system" fn RtlCmEncodeMemIoResource(descriptor : *const CM_PARTIAL_RESOURCE_DESCRIPTOR, r#type : u8, length : u64, start : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlCompareString(string1 : *const super::super::super::Win32::System::Kernel:: STRING, string2 : *const super::super::super::Win32::System::Kernel:: STRING, caseinsensitive : super::super::super::Win32::Foundation:: BOOLEAN) -> i32);
windows_targets::link!("ntdll.dll" "system" fn RtlCompareUnicodeString(string1 : *const super::super::super::Win32::Foundation:: UNICODE_STRING, string2 : *const super::super::super::Win32::Foundation:: UNICODE_STRING, caseinsensitive : super::super::super::Win32::Foundation:: BOOLEAN) -> i32);
windows_targets::link!("ntdll.dll" "system" fn RtlCompareUnicodeStrings(string1 : *const u16, string1length : usize, string2 : *const u16, string2length : usize, caseinsensitive : super::super::super::Win32::Foundation:: BOOLEAN) -> i32);
windows_targets::link!("ntdll.dll" "system" fn RtlContractHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlCopyBitMap(source : *const RTL_BITMAP, destination : *const RTL_BITMAP, targetbit : u32));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlCopyString(destinationstring : *mut super::super::super::Win32::System::Kernel:: STRING, sourcestring : *const super::super::super::Win32::System::Kernel:: STRING));
windows_targets::link!("ntdll.dll" "system" fn RtlCopyUnicodeString(destinationstring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, sourcestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING));
windows_targets::link!("ntdll.dll" "system" fn RtlCreateHashTable(hashtable : *mut *mut RTL_DYNAMIC_HASH_TABLE, shift : u32, flags : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlCreateHashTableEx(hashtable : *mut *mut RTL_DYNAMIC_HASH_TABLE, initialsize : u32, shift : u32, flags : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlCreateRegistryKey(relativeto : u32, path : windows_sys::core::PCWSTR) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("ntdll.dll" "system" fn RtlCreateSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, revision : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn RtlDelete(links : *const super::super::Foundation:: RTL_SPLAY_LINKS) -> *mut super::super::Foundation:: RTL_SPLAY_LINKS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntdll.dll" "system" fn RtlDeleteElementGenericTable(table : *const RTL_GENERIC_TABLE, buffer : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlDeleteElementGenericTableAvl(table : *const RTL_AVL_TABLE, buffer : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlDeleteElementGenericTableAvlEx(table : *const RTL_AVL_TABLE, nodeorparent : *const core::ffi::c_void));
windows_targets::link!("ntdll.dll" "system" fn RtlDeleteHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE));
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn RtlDeleteNoSplay(links : *const super::super::Foundation:: RTL_SPLAY_LINKS, root : *mut *mut super::super::Foundation:: RTL_SPLAY_LINKS));
windows_targets::link!("ntdll.dll" "system" fn RtlDeleteRegistryValue(relativeto : u32, path : windows_sys::core::PCWSTR, valuename : windows_sys::core::PCWSTR) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlDowncaseUnicodeChar(sourcecharacter : u16) -> u16);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlEndEnumerationHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlEndStrongEnumerationHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlEndWeakEnumerationHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlEnumerateEntryHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> *mut RTL_DYNAMIC_HASH_TABLE_ENTRY);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntdll.dll" "system" fn RtlEnumerateGenericTable(table : *const RTL_GENERIC_TABLE, restart : super::super::super::Win32::Foundation:: BOOLEAN) -> *mut core::ffi::c_void);
windows_targets::link!("ntdll.dll" "system" fn RtlEnumerateGenericTableAvl(table : *const RTL_AVL_TABLE, restart : super::super::super::Win32::Foundation:: BOOLEAN) -> *mut core::ffi::c_void);
windows_targets::link!("ntdll.dll" "system" fn RtlEnumerateGenericTableLikeADirectory(table : *const RTL_AVL_TABLE, matchfunction : PRTL_AVL_MATCH_FUNCTION, matchdata : *const core::ffi::c_void, nextflag : u32, restartkey : *mut *mut core::ffi::c_void, deletecount : *mut u32, buffer : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntdll.dll" "system" fn RtlEnumerateGenericTableWithoutSplaying(table : *const RTL_GENERIC_TABLE, restartkey : *mut *mut core::ffi::c_void) -> *mut core::ffi::c_void);
windows_targets::link!("ntdll.dll" "system" fn RtlEnumerateGenericTableWithoutSplayingAvl(table : *const RTL_AVL_TABLE, restartkey : *mut *mut core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlEqualString(string1 : *const super::super::super::Win32::System::Kernel:: STRING, string2 : *const super::super::super::Win32::System::Kernel:: STRING, caseinsensitive : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlEqualUnicodeString(string1 : *const super::super::super::Win32::Foundation:: UNICODE_STRING, string2 : *const super::super::super::Win32::Foundation:: UNICODE_STRING, caseinsensitive : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlExpandHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlExtractBitMap(source : *const RTL_BITMAP, destination : *const RTL_BITMAP, targetbit : u32, numberofbits : u32));
windows_targets::link!("ntdll.dll" "system" fn RtlFindClearBits(bitmapheader : *const RTL_BITMAP, numbertofind : u32, hintindex : u32) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlFindClearBitsAndSet(bitmapheader : *const RTL_BITMAP, numbertofind : u32, hintindex : u32) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlFindClearRuns(bitmapheader : *const RTL_BITMAP, runarray : *mut RTL_BITMAP_RUN, sizeofrunarray : u32, locatelongestruns : super::super::super::Win32::Foundation:: BOOLEAN) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlFindClosestEncodableLength(sourcelength : u64, targetlength : *mut u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn RtlFindFirstRunClear(bitmapheader : *const RTL_BITMAP, startingindex : *mut u32) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlFindLastBackwardRunClear(bitmapheader : *const RTL_BITMAP, fromindex : u32, startingrunindex : *mut u32) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlFindLeastSignificantBit(set : u64) -> i8);
windows_targets::link!("ntdll.dll" "system" fn RtlFindLongestRunClear(bitmapheader : *const RTL_BITMAP, startingindex : *mut u32) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlFindMostSignificantBit(set : u64) -> i8);
windows_targets::link!("ntdll.dll" "system" fn RtlFindNextForwardRunClear(bitmapheader : *const RTL_BITMAP, fromindex : u32, startingrunindex : *mut u32) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlFindSetBits(bitmapheader : *const RTL_BITMAP, numbertofind : u32, hintindex : u32) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlFindSetBitsAndClear(bitmapheader : *const RTL_BITMAP, numbertofind : u32, hintindex : u32) -> u32);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlFreeUTF8String(utf8string : *mut super::super::super::Win32::System::Kernel:: STRING));
windows_targets::link!("ntdll.dll" "system" fn RtlGUIDFromString(guidstring : *const super::super::super::Win32::Foundation:: UNICODE_STRING, guid : *mut windows_sys::core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn RtlGenerateClass5Guid(namespaceguid : *const windows_sys::core::GUID, buffer : *const core::ffi::c_void, buffersize : u32, guid : *mut windows_sys::core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlGetActiveConsoleId() -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlGetCallersAddress(callersaddress : *mut *mut core::ffi::c_void, callerscaller : *mut *mut core::ffi::c_void));
windows_targets::link!("ntdll.dll" "system" fn RtlGetConsoleSessionForegroundProcessId() -> u64);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntdll.dll" "system" fn RtlGetElementGenericTable(table : *const RTL_GENERIC_TABLE, i : u32) -> *mut core::ffi::c_void);
windows_targets::link!("ntdll.dll" "system" fn RtlGetElementGenericTableAvl(table : *const RTL_AVL_TABLE, i : u32) -> *mut core::ffi::c_void);
windows_targets::link!("ntdll.dll" "system" fn RtlGetEnabledExtendedFeatures(featuremask : u64) -> u64);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlGetNextEntryHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, context : *const RTL_DYNAMIC_HASH_TABLE_CONTEXT) -> *mut RTL_DYNAMIC_HASH_TABLE_ENTRY);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlGetNtProductType(ntproducttype : *mut super::super::super::Win32::System::Kernel:: NT_PRODUCT_TYPE) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlGetNtSystemRoot() -> windows_sys::core::PCWSTR);
windows_targets::link!("ntdll.dll" "system" fn RtlGetPersistedStateLocation(sourceid : windows_sys::core::PCWSTR, customvalue : windows_sys::core::PCWSTR, defaultpath : windows_sys::core::PCWSTR, statelocationtype : STATE_LOCATION_TYPE, targetpath : windows_sys::core::PWSTR, bufferlengthin : u32, bufferlengthout : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlGetSuiteMask() -> u32);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("ntdll.dll" "system" fn RtlGetVersion(lpversioninformation : *mut super::super::super::Win32::System::SystemInformation:: OSVERSIONINFOW) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlHashUnicodeString(string : *const super::super::super::Win32::Foundation:: UNICODE_STRING, caseinsensitive : super::super::super::Win32::Foundation:: BOOLEAN, hashalgorithm : u32, hashvalue : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlInitEnumerationHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlInitStrongEnumerationHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlInitUTF8String(destinationstring : *mut super::super::super::Win32::System::Kernel:: STRING, sourcestring : *const i8));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlInitUTF8StringEx(destinationstring : *mut super::super::super::Win32::System::Kernel:: STRING, sourcestring : *const i8) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlInitWeakEnumerationHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlInitializeBitMap(bitmapheader : *mut RTL_BITMAP, bitmapbuffer : *const u32, sizeofbitmap : u32));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntdll.dll" "system" fn RtlInitializeGenericTable(table : *mut RTL_GENERIC_TABLE, compareroutine : PRTL_GENERIC_COMPARE_ROUTINE, allocateroutine : PRTL_GENERIC_ALLOCATE_ROUTINE, freeroutine : PRTL_GENERIC_FREE_ROUTINE, tablecontext : *const core::ffi::c_void));
windows_targets::link!("ntdll.dll" "system" fn RtlInitializeGenericTableAvl(table : *mut RTL_AVL_TABLE, compareroutine : PRTL_AVL_COMPARE_ROUTINE, allocateroutine : PRTL_AVL_ALLOCATE_ROUTINE, freeroutine : PRTL_AVL_FREE_ROUTINE, tablecontext : *const core::ffi::c_void));
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntdll.dll" "system" fn RtlInsertElementGenericTable(table : *const RTL_GENERIC_TABLE, buffer : *const core::ffi::c_void, buffersize : u32, newelement : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> *mut core::ffi::c_void);
windows_targets::link!("ntdll.dll" "system" fn RtlInsertElementGenericTableAvl(table : *const RTL_AVL_TABLE, buffer : *const core::ffi::c_void, buffersize : u32, newelement : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntdll.dll" "system" fn RtlInsertElementGenericTableFull(table : *const RTL_GENERIC_TABLE, buffer : *const core::ffi::c_void, buffersize : u32, newelement : *mut super::super::super::Win32::Foundation:: BOOLEAN, nodeorparent : *const core::ffi::c_void, searchresult : TABLE_SEARCH_RESULT) -> *mut core::ffi::c_void);
windows_targets::link!("ntdll.dll" "system" fn RtlInsertElementGenericTableFullAvl(table : *const RTL_AVL_TABLE, buffer : *const core::ffi::c_void, buffersize : u32, newelement : *mut super::super::super::Win32::Foundation:: BOOLEAN, nodeorparent : *const core::ffi::c_void, searchresult : TABLE_SEARCH_RESULT) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlInsertEntryHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, entry : *const RTL_DYNAMIC_HASH_TABLE_ENTRY, signature : usize, context : *mut RTL_DYNAMIC_HASH_TABLE_CONTEXT) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlInt64ToUnicodeString(value : u64, base : u32, string : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlIntegerToUnicodeString(value : u32, base : u32, string : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlIoDecodeMemIoResource(descriptor : *const IO_RESOURCE_DESCRIPTOR, alignment : *mut u64, minimumaddress : *mut u64, maximumaddress : *mut u64) -> u64);
windows_targets::link!("ntdll.dll" "system" fn RtlIoEncodeMemIoResource(descriptor : *const IO_RESOURCE_DESCRIPTOR, r#type : u8, length : u64, alignment : u64, minimumaddress : u64, maximumaddress : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlIsApiSetImplemented(apisetname : windows_sys::core::PCSTR) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntdll.dll" "system" fn RtlIsGenericTableEmpty(table : *const RTL_GENERIC_TABLE) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlIsGenericTableEmptyAvl(table : *const RTL_AVL_TABLE) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlIsMultiSessionSku() -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlIsMultiUsersInSessionSku() -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn RtlIsNtDdiVersionAvailable(version : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn RtlIsServicePackVersionInstalled(version : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlIsStateSeparationEnabled() -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlIsUntrustedObject(handle : super::super::super::Win32::Foundation:: HANDLE, object : *const core::ffi::c_void, untrustedobject : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("ntdll.dll" "system" fn RtlLengthSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntdll.dll" "system" fn RtlLookupElementGenericTable(table : *const RTL_GENERIC_TABLE, buffer : *const core::ffi::c_void) -> *mut core::ffi::c_void);
windows_targets::link!("ntdll.dll" "system" fn RtlLookupElementGenericTableAvl(table : *const RTL_AVL_TABLE, buffer : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntdll.dll" "system" fn RtlLookupElementGenericTableFull(table : *const RTL_GENERIC_TABLE, buffer : *const core::ffi::c_void, nodeorparent : *mut *mut core::ffi::c_void, searchresult : *mut TABLE_SEARCH_RESULT) -> *mut core::ffi::c_void);
windows_targets::link!("ntdll.dll" "system" fn RtlLookupElementGenericTableFullAvl(table : *const RTL_AVL_TABLE, buffer : *const core::ffi::c_void, nodeorparent : *mut *mut core::ffi::c_void, searchresult : *mut TABLE_SEARCH_RESULT) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlLookupEntryHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, signature : usize, context : *mut RTL_DYNAMIC_HASH_TABLE_CONTEXT) -> *mut RTL_DYNAMIC_HASH_TABLE_ENTRY);
windows_targets::link!("ntdll.dll" "system" fn RtlLookupFirstMatchingElementGenericTableAvl(table : *const RTL_AVL_TABLE, buffer : *const core::ffi::c_void, restartkey : *mut *mut core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("ntdll.dll" "system" fn RtlMapGenericMask(accessmask : *mut u32, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING));
#[cfg(feature = "Win32_Security")]
windows_targets::link!("ntdll.dll" "system" fn RtlNormalizeSecurityDescriptor(securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, securitydescriptorlength : u32, newsecuritydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, newsecuritydescriptorlength : *mut u32, checkonly : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
windows_targets::link!("ntdll.dll" "system" fn RtlNumberGenericTableElements(table : *const RTL_GENERIC_TABLE) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlNumberGenericTableElementsAvl(table : *const RTL_AVL_TABLE) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlNumberOfClearBits(bitmapheader : *const RTL_BITMAP) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlNumberOfClearBitsInRange(bitmapheader : *const RTL_BITMAP, startingindex : u32, length : u32) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlNumberOfSetBits(bitmapheader : *const RTL_BITMAP) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlNumberOfSetBitsInRange(bitmapheader : *const RTL_BITMAP, startingindex : u32, length : u32) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlNumberOfSetBitsUlongPtr(target : usize) -> u32);
windows_targets::link!("ntoskrnl.exe" "system" fn RtlPrefetchMemoryNonTemporal(source : *const core::ffi::c_void, length : usize));
windows_targets::link!("ntdll.dll" "system" fn RtlPrefixUnicodeString(string1 : *const super::super::super::Win32::Foundation:: UNICODE_STRING, string2 : *const super::super::super::Win32::Foundation:: UNICODE_STRING, caseinsensitive : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlQueryRegistryValueWithFallback(primaryhandle : super::super::super::Win32::Foundation:: HANDLE, fallbackhandle : super::super::super::Win32::Foundation:: HANDLE, valuename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, valuelength : u32, valuetype : *mut u32, valuedata : *mut core::ffi::c_void, resultlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlQueryRegistryValues(relativeto : u32, path : windows_sys::core::PCWSTR, querytable : *mut RTL_QUERY_REGISTRY_TABLE, context : *const core::ffi::c_void, environment : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlQueryValidationRunlevel(componentname : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> u32);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn RtlRealPredecessor(links : *const super::super::Foundation:: RTL_SPLAY_LINKS) -> *mut super::super::Foundation:: RTL_SPLAY_LINKS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn RtlRealSuccessor(links : *const super::super::Foundation:: RTL_SPLAY_LINKS) -> *mut super::super::Foundation:: RTL_SPLAY_LINKS);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlRemoveEntryHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, entry : *const RTL_DYNAMIC_HASH_TABLE_ENTRY, context : *mut RTL_DYNAMIC_HASH_TABLE_CONTEXT) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_System_Threading")]
windows_targets::link!("ntdll.dll" "system" fn RtlRunOnceBeginInitialize(runonce : *mut super::super::super::Win32::System::Threading:: INIT_ONCE, flags : u32, context : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Threading")]
windows_targets::link!("ntdll.dll" "system" fn RtlRunOnceComplete(runonce : *mut super::super::super::Win32::System::Threading:: INIT_ONCE, flags : u32, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Threading")]
windows_targets::link!("ntdll.dll" "system" fn RtlRunOnceExecuteOnce(runonce : *mut super::super::super::Win32::System::Threading:: INIT_ONCE, initfn : PRTL_RUN_ONCE_INIT_FN, parameter : *mut core::ffi::c_void, context : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Threading")]
windows_targets::link!("ntdll.dll" "system" fn RtlRunOnceInitialize(runonce : *mut super::super::super::Win32::System::Threading:: INIT_ONCE));
windows_targets::link!("ntdll.dll" "system" fn RtlSetAllBits(bitmapheader : *const RTL_BITMAP));
windows_targets::link!("ntdll.dll" "system" fn RtlSetBit(bitmapheader : *const RTL_BITMAP, bitnumber : u32));
windows_targets::link!("ntdll.dll" "system" fn RtlSetBits(bitmapheader : *const RTL_BITMAP, startingindex : u32, numbertoset : u32));
#[cfg(feature = "Win32_Security")]
windows_targets::link!("ntdll.dll" "system" fn RtlSetDaclSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, daclpresent : super::super::super::Win32::Foundation:: BOOLEAN, dacl : *const super::super::super::Win32::Security:: ACL, dacldefaulted : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("ntoskrnl.exe" "system" fn RtlSetSystemGlobalData(dataid : super::super::super::Win32::System::SystemInformation:: RTL_SYSTEM_GLOBAL_DATA_ID, buffer : *const core::ffi::c_void, size : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn RtlSplay(links : *mut super::super::Foundation:: RTL_SPLAY_LINKS) -> *mut super::super::Foundation:: RTL_SPLAY_LINKS);
windows_targets::link!("ntdll.dll" "system" fn RtlStringFromGUID(guid : *const windows_sys::core::GUID, guidstring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlStronglyEnumerateEntryHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> *mut RTL_DYNAMIC_HASH_TABLE_ENTRY);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn RtlSubtreePredecessor(links : *const super::super::Foundation:: RTL_SPLAY_LINKS) -> *mut super::super::Foundation:: RTL_SPLAY_LINKS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn RtlSubtreeSuccessor(links : *const super::super::Foundation:: RTL_SPLAY_LINKS) -> *mut super::super::Foundation:: RTL_SPLAY_LINKS);
windows_targets::link!("ntoskrnl.exe" "system" fn RtlSuffixUnicodeString(string1 : *const super::super::super::Win32::Foundation:: UNICODE_STRING, string2 : *const super::super::super::Win32::Foundation:: UNICODE_STRING, caseinsensitive : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlTestBit(bitmapheader : *const RTL_BITMAP, bitnumber : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlTimeFieldsToTime(timefields : *const TIME_FIELDS, time : *mut i64) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntdll.dll" "system" fn RtlTimeToTimeFields(time : *const i64, timefields : *mut TIME_FIELDS));
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlUTF8StringToUnicodeString(destinationstring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, sourcestring : *const super::super::super::Win32::System::Kernel:: STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlUTF8ToUnicodeN(unicodestringdestination : windows_sys::core::PWSTR, unicodestringmaxbytecount : u32, unicodestringactualbytecount : *mut u32, utf8stringsource : windows_sys::core::PCSTR, utf8stringbytecount : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn RtlUnicodeStringToInt64(string : *const super::super::super::Win32::Foundation:: UNICODE_STRING, base : u32, number : *mut i64, endpointer : *mut windows_sys::core::PWSTR) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlUnicodeStringToInteger(string : *const super::super::super::Win32::Foundation:: UNICODE_STRING, base : u32, value : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlUnicodeStringToUTF8String(destinationstring : *mut super::super::super::Win32::System::Kernel:: STRING, sourcestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlUnicodeToUTF8N(utf8stringdestination : windows_sys::core::PSTR, utf8stringmaxbytecount : u32, utf8stringactualbytecount : *mut u32, unicodestringsource : *const u16, unicodestringbytecount : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeChar(sourcecharacter : u16) -> u16);
windows_targets::link!("ntdll.dll" "system" fn RtlUpcaseUnicodeString(destinationstring : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, sourcestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING, allocatedestinationstring : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlUpperChar(character : i8) -> i8);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlUpperString(destinationstring : *mut super::super::super::Win32::System::Kernel:: STRING, sourcestring : *const super::super::super::Win32::System::Kernel:: STRING));
#[cfg(feature = "Win32_Security")]
windows_targets::link!("ntdll.dll" "system" fn RtlValidRelativeSecurityDescriptor(securitydescriptorinput : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, securitydescriptorlength : u32, requiredinformation : u32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("ntdll.dll" "system" fn RtlValidSecurityDescriptor(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_System_SystemInformation")]
windows_targets::link!("ntdll.dll" "system" fn RtlVerifyVersionInfo(versioninfo : *const super::super::super::Win32::System::SystemInformation:: OSVERSIONINFOEXW, typemask : u32, conditionmask : u64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn RtlVolumeDeviceToDosName(volumedeviceobject : *const core::ffi::c_void, dosname : *mut super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn RtlWalkFrameChain(callers : *mut *mut core::ffi::c_void, count : u32, flags : u32) -> u32);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlWeaklyEnumerateEntryHashTable(hashtable : *const RTL_DYNAMIC_HASH_TABLE, enumerator : *mut RTL_DYNAMIC_HASH_TABLE_ENUMERATOR) -> *mut RTL_DYNAMIC_HASH_TABLE_ENTRY);
windows_targets::link!("ntdll.dll" "system" fn RtlWriteRegistryValue(relativeto : u32, path : windows_sys::core::PCWSTR, valuename : windows_sys::core::PCWSTR, valuetype : u32, valuedata : *const core::ffi::c_void, valuelength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Kernel")]
windows_targets::link!("ntdll.dll" "system" fn RtlxAnsiStringToUnicodeSize(ansistring : *const super::super::super::Win32::System::Kernel:: STRING) -> u32);
windows_targets::link!("ntdll.dll" "system" fn RtlxUnicodeStringToAnsiSize(unicodestring : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> u32);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
windows_targets::link!("ntoskrnl.exe" "system" fn SeAccessCheck(securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, subjectsecuritycontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, subjectcontextlocked : super::super::super::Win32::Foundation:: BOOLEAN, desiredaccess : u32, previouslygrantedaccess : u32, privileges : *mut *mut super::super::super::Win32::Security:: PRIVILEGE_SET, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, accessmode : i8, grantedaccess : *mut u32, accessstatus : *mut i32) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
windows_targets::link!("ntoskrnl.exe" "system" fn SeAssignSecurity(parentdescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, explicitdescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, newdescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, isdirectoryobject : super::super::super::Win32::Foundation:: BOOLEAN, subjectcontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, pooltype : super::super::Foundation:: POOL_TYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
windows_targets::link!("ntoskrnl.exe" "system" fn SeAssignSecurityEx(parentdescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, explicitdescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, newdescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, objecttype : *const windows_sys::core::GUID, isdirectoryobject : super::super::super::Win32::Foundation:: BOOLEAN, autoinheritflags : u32, subjectcontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT, genericmapping : *const super::super::super::Win32::Security:: GENERIC_MAPPING, pooltype : super::super::Foundation:: POOL_TYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
windows_targets::link!("ntoskrnl.exe" "system" fn SeCaptureSubjectContext(subjectcontext : *mut super::super::Foundation:: SECURITY_SUBJECT_CONTEXT));
#[cfg(feature = "Win32_Security")]
windows_targets::link!("ntoskrnl.exe" "system" fn SeComputeAutoInheritByObjectType(objecttype : *const core::ffi::c_void, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR, parentsecuritydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> u32);
#[cfg(feature = "Win32_Security")]
windows_targets::link!("ntoskrnl.exe" "system" fn SeDeassignSecurity(securitydescriptor : *mut super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn SeEtwWriteKMCveEvent(cveid : *const super::super::super::Win32::Foundation:: UNICODE_STRING, additionaldetails : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
windows_targets::link!("ntoskrnl.exe" "system" fn SeLockSubjectContext(subjectcontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT));
windows_targets::link!("ntoskrnl.exe" "system" fn SeRegisterImageVerificationCallback(imagetype : SE_IMAGE_TYPE, callbacktype : SE_IMAGE_VERIFICATION_CALLBACK_TYPE, callbackfunction : PSE_IMAGE_VERIFICATION_CALLBACK_FUNCTION, callbackcontext : *const core::ffi::c_void, token : *const core::ffi::c_void, callbackhandle : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
windows_targets::link!("ntoskrnl.exe" "system" fn SeReleaseSubjectContext(subjectcontext : *mut super::super::Foundation:: SECURITY_SUBJECT_CONTEXT));
#[cfg(feature = "Win32_Security_Authentication_Identity")]
windows_targets::link!("ntoskrnl.exe" "system" fn SeReportSecurityEvent(flags : u32, sourcename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, usersid : super::super::super::Win32::Security:: PSID, auditparameters : *const super::super::super::Win32::Security::Authentication::Identity:: SE_ADT_PARAMETER_ARRAY) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Security_Authentication_Identity")]
windows_targets::link!("ntoskrnl.exe" "system" fn SeSetAuditParameter(auditparameters : *mut super::super::super::Win32::Security::Authentication::Identity:: SE_ADT_PARAMETER_ARRAY, r#type : super::super::super::Win32::Security::Authentication::Identity:: SE_ADT_PARAMETER_TYPE, index : u32, data : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn SeSinglePrivilegeCheck(privilegevalue : super::super::super::Win32::Foundation:: LUID, previousmode : i8) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_Security"))]
windows_targets::link!("ntoskrnl.exe" "system" fn SeUnlockSubjectContext(subjectcontext : *const super::super::Foundation:: SECURITY_SUBJECT_CONTEXT));
windows_targets::link!("ntoskrnl.exe" "system" fn SeUnregisterImageVerificationCallback(callbackhandle : *const core::ffi::c_void));
#[cfg(feature = "Win32_Security")]
windows_targets::link!("ntoskrnl.exe" "system" fn SeValidSecurityDescriptor(length : u32, securitydescriptor : super::super::super::Win32::Security:: PSECURITY_DESCRIPTOR) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmCommitComplete(enlistment : *const super::super::Foundation:: KENLISTMENT, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmCommitEnlistment(enlistment : *const super::super::Foundation:: KENLISTMENT, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmCommitTransaction(transaction : *const super::super::Foundation:: KTRANSACTION, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmCreateEnlistment(enlistmenthandle : *mut super::super::super::Win32::Foundation:: HANDLE, previousmode : i8, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, resourcemanager : *const isize, transaction : *const super::super::Foundation:: KTRANSACTION, createoptions : u32, notificationmask : u32, enlistmentkey : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmDereferenceEnlistmentKey(enlistment : *const super::super::Foundation:: KENLISTMENT, lastreference : *mut super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmEnableCallbacks(resourcemanager : *const super::super::Foundation:: KRESOURCEMANAGER, callbackroutine : PTM_RM_NOTIFICATION, rmkey : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmGetTransactionId(transaction : *const super::super::Foundation:: KTRANSACTION, transactionid : *mut windows_sys::core::GUID));
windows_targets::link!("ntoskrnl.exe" "system" fn TmInitializeTransactionManager(transactionmanager : *const isize, logfilename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, tmid : *const windows_sys::core::GUID, createoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmIsTransactionActive(transaction : *const super::super::Foundation:: KTRANSACTION) -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmPrePrepareComplete(enlistment : *const super::super::Foundation:: KENLISTMENT, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmPrePrepareEnlistment(enlistment : *const super::super::Foundation:: KENLISTMENT, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmPrepareComplete(enlistment : *const super::super::Foundation:: KENLISTMENT, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmPrepareEnlistment(enlistment : *const super::super::Foundation:: KENLISTMENT, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmPropagationComplete(resourcemanager : *const super::super::Foundation:: KRESOURCEMANAGER, requestcookie : u32, bufferlength : u32, buffer : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmPropagationFailed(resourcemanager : *const super::super::Foundation:: KRESOURCEMANAGER, requestcookie : u32, status : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmReadOnlyEnlistment(enlistment : *const super::super::Foundation:: KENLISTMENT, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmRecoverEnlistment(enlistment : *const super::super::Foundation:: KENLISTMENT, enlistmentkey : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmRecoverResourceManager(resourcemanager : *const super::super::Foundation:: KRESOURCEMANAGER) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmRecoverTransactionManager(tm : *const super::super::Foundation:: KTM, targetvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmReferenceEnlistmentKey(enlistment : *const super::super::Foundation:: KENLISTMENT, key : *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn TmRenameTransactionManager(logfilename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, existingtransactionmanagerguid : *const windows_sys::core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmRequestOutcomeEnlistment(enlistment : *const super::super::Foundation:: KENLISTMENT, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmRollbackComplete(enlistment : *const super::super::Foundation:: KENLISTMENT, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmRollbackEnlistment(enlistment : *const super::super::Foundation:: KENLISTMENT, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmRollbackTransaction(transaction : *const super::super::Foundation:: KTRANSACTION, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn TmSinglePhaseReject(enlistment : *const super::super::Foundation:: KENLISTMENT, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntoskrnl.exe" "system" fn VslCreateSecureSection(handle : *mut super::super::super::Win32::Foundation:: HANDLE, targetprocess : super::super::Foundation:: PEPROCESS, mdl : *const super::super::Foundation:: MDL, devicepageprotection : u32, attributes : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn VslDeleteSecureSection(globalhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("ntoskrnl.exe" "system" fn WheaAddErrorSource(errorsource : *const super::super::super::Win32::System::Diagnostics::Debug:: WHEA_ERROR_SOURCE_DESCRIPTOR, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("ntoskrnl.exe" "system" fn WheaAddErrorSourceDeviceDriver(context : *mut core::ffi::c_void, configuration : *const super::super::super::Win32::System::Diagnostics::Debug:: WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER, numberpreallocatederrorreports : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("ntoskrnl.exe" "system" fn WheaAddErrorSourceDeviceDriverV1(context : *mut core::ffi::c_void, configuration : *const super::super::super::Win32::System::Diagnostics::Debug:: WHEA_ERROR_SOURCE_CONFIGURATION_DEVICE_DRIVER, numbufferstopreallocate : u32, maxdatalength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("ntoskrnl.exe" "system" fn WheaAddHwErrorReportSectionDeviceDriver(errorhandle : *const core::ffi::c_void, sectiondatalength : u32, bufferset : *mut super::super::super::Win32::System::Diagnostics::Debug:: WHEA_DRIVER_BUFFER_SET) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("ntoskrnl.exe" "system" fn WheaConfigureErrorSource(sourcetype : super::super::super::Win32::System::Diagnostics::Debug:: WHEA_ERROR_SOURCE_TYPE, configuration : *const WHEA_ERROR_SOURCE_CONFIGURATION) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn WheaCreateHwErrorReportDeviceDriver(errorsourceid : u32, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("ntoskrnl.exe" "system" fn WheaErrorSourceGetState(errorsourceid : u32) -> super::super::super::Win32::System::Diagnostics::Debug:: WHEA_ERROR_SOURCE_STATE);
windows_targets::link!("ntoskrnl.exe" "system" fn WheaGetNotifyAllOfflinesPolicy() -> super::super::super::Win32::Foundation:: BOOLEAN);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("ntoskrnl.exe" "system" fn WheaHighIrqlLogSelEventHandlerRegister(handler : PFN_WHEA_HIGH_IRQL_LOG_SEL_EVENT_HANDLER, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn WheaHighIrqlLogSelEventHandlerUnregister());
windows_targets::link!("ntoskrnl.exe" "system" fn WheaHwErrorReportAbandonDeviceDriver(errorhandle : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("ntoskrnl.exe" "system" fn WheaHwErrorReportSetSectionNameDeviceDriver(bufferset : *const super::super::super::Win32::System::Diagnostics::Debug:: WHEA_DRIVER_BUFFER_SET, namelength : u32, name : *const u8) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn WheaHwErrorReportSetSeverityDeviceDriver(errorhandle : *const core::ffi::c_void, errorseverity : WHEA_ERROR_SEVERITY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn WheaHwErrorReportSubmitDeviceDriver(errorhandle : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn WheaInitializeRecordHeader(header : *mut WHEA_ERROR_RECORD_HEADER) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn WheaIsCriticalState() -> super::super::super::Win32::Foundation:: BOOLEAN);
windows_targets::link!("ntoskrnl.exe" "system" fn WheaLogInternalEvent(entry : *const WHEA_EVENT_LOG_ENTRY));
windows_targets::link!("ntoskrnl.exe" "system" fn WheaRegisterInUsePageOfflineNotification(callback : PFN_IN_USE_PAGE_OFFLINE_NOTIFY, context : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn WheaRemoveErrorSource(errorsourceid : u32));
windows_targets::link!("ntoskrnl.exe" "system" fn WheaRemoveErrorSourceDeviceDriver(errorsourceid : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("ntoskrnl.exe" "system" fn WheaReportHwError(errorpacket : *mut WHEA_ERROR_PACKET_V2) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
windows_targets::link!("ntoskrnl.exe" "system" fn WheaReportHwErrorDeviceDriver(errorsourceid : u32, deviceobject : *const super::super::Foundation:: DEVICE_OBJECT, errordata : *const u8, errordatalength : u32, sectiontypeguid : *const windows_sys::core::GUID, errorseverity : WHEA_ERROR_SEVERITY, devicefriendlyname : windows_sys::core::PCSTR) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
windows_targets::link!("ntoskrnl.exe" "system" fn WheaUnconfigureErrorSource(sourcetype : super::super::super::Win32::System::Diagnostics::Debug:: WHEA_ERROR_SOURCE_TYPE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn WheaUnregisterInUsePageOfflineNotification(callback : PFN_IN_USE_PAGE_OFFLINE_NOTIFY) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntoskrnl.exe" "system" fn WmiQueryTraceInformation(traceinformationclass : TRACE_INFORMATION_CLASS, traceinformation : *mut core::ffi::c_void, traceinformationlength : u32, requiredlength : *mut u32, buffer : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwAllocateLocallyUniqueId(luid : *mut super::super::super::Win32::Foundation:: LUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwClose(handle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwCommitComplete(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwCommitEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwCommitTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn ZwCreateEnlistment(enlistmenthandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE, transactionhandle : super::super::super::Win32::Foundation:: HANDLE, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, createoptions : u32, notificationmask : u32, enlistmentkey : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_IO"))]
windows_targets::link!("ntdll.dll" "system" fn ZwCreateFile(filehandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, allocationsize : *const i64, fileattributes : u32, shareaccess : u32, createdisposition : u32, createoptions : u32, eabuffer : *const core::ffi::c_void, ealength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn ZwCreateResourceManager(resourcemanagerhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, tmhandle : super::super::super::Win32::Foundation:: HANDLE, resourcemanagerguid : *const windows_sys::core::GUID, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, createoptions : u32, description : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn ZwCreateSection(sectionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, maximumsize : *const i64, sectionpageprotection : u32, allocationattributes : u32, filehandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn ZwCreateTransaction(transactionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, uow : *const windows_sys::core::GUID, tmhandle : super::super::super::Win32::Foundation:: HANDLE, createoptions : u32, isolationlevel : u32, isolationflags : u32, timeout : *const i64, description : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn ZwCreateTransactionManager(tmhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, logfilename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, createoptions : u32, commitstrength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("ntdll.dll" "system" fn ZwDeviceIoControlFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, iocontrolcode : u32, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwDisplayString(string : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn ZwEnumerateTransactionObject(rootobjecthandle : super::super::super::Win32::Foundation:: HANDLE, querytype : super::super::super::Win32::System::SystemServices:: KTMOBJECT_TYPE, objectcursor : *mut super::super::super::Win32::System::SystemServices:: KTMOBJECT_CURSOR, objectcursorlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_Storage_FileSystem")]
windows_targets::link!("ntdll.dll" "system" fn ZwGetNotificationResourceManager(resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE, transactionnotification : *mut super::super::super::Win32::Storage::FileSystem:: TRANSACTION_NOTIFICATION, notificationlength : u32, timeout : *const i64, returnlength : *mut u32, asynchronous : u32, asynchronouscontext : usize) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwLoadDriver(driverservicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwMakeTemporaryObject(handle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwManagePartition(targethandle : super::super::super::Win32::Foundation:: HANDLE, sourcehandle : super::super::super::Win32::Foundation:: HANDLE, partitioninformationclass : PARTITION_INFORMATION_CLASS, partitioninformation : *mut core::ffi::c_void, partitioninformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn ZwOpenEnlistment(enlistmenthandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, rmhandle : super::super::super::Win32::Foundation:: HANDLE, enlistmentguid : *const windows_sys::core::GUID, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_IO"))]
windows_targets::link!("ntdll.dll" "system" fn ZwOpenFile(filehandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, shareaccess : u32, openoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn ZwOpenResourceManager(resourcemanagerhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, tmhandle : super::super::super::Win32::Foundation:: HANDLE, resourcemanagerguid : *const windows_sys::core::GUID, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn ZwOpenSymbolicLinkObject(linkhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn ZwOpenTransaction(transactionhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, uow : *const windows_sys::core::GUID, tmhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Wdk_Foundation")]
windows_targets::link!("ntdll.dll" "system" fn ZwOpenTransactionManager(tmhandle : *mut super::super::super::Win32::Foundation:: HANDLE, desiredaccess : u32, objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, logfilename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, tmidentity : *const windows_sys::core::GUID, openoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_Power")]
windows_targets::link!("ntdll.dll" "system" fn ZwPowerInformation(informationlevel : super::super::super::Win32::System::Power:: POWER_INFORMATION_LEVEL, inputbuffer : *const core::ffi::c_void, inputbufferlength : u32, outputbuffer : *mut core::ffi::c_void, outputbufferlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwPrePrepareComplete(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwPrePrepareEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwPrepareComplete(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwPrepareEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwPropagationComplete(resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE, requestcookie : u32, bufferlength : u32, buffer : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwPropagationFailed(resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE, requestcookie : u32, propstatus : super::super::super::Win32::Foundation:: NTSTATUS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_System_IO"))]
windows_targets::link!("ntdll.dll" "system" fn ZwQueryInformationByName(objectattributes : *const super::super::Foundation:: OBJECT_ATTRIBUTES, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::super::Storage::FileSystem:: FILE_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn ZwQueryInformationEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, enlistmentinformationclass : super::super::super::Win32::System::SystemServices:: ENLISTMENT_INFORMATION_CLASS, enlistmentinformation : *mut core::ffi::c_void, enlistmentinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Win32_System_IO"))]
windows_targets::link!("ntdll.dll" "system" fn ZwQueryInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fileinformation : *mut core::ffi::c_void, length : u32, fileinformationclass : super::super::Storage::FileSystem:: FILE_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn ZwQueryInformationResourceManager(resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE, resourcemanagerinformationclass : super::super::super::Win32::System::SystemServices:: RESOURCEMANAGER_INFORMATION_CLASS, resourcemanagerinformation : *mut core::ffi::c_void, resourcemanagerinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn ZwQueryInformationTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, transactioninformationclass : super::super::super::Win32::System::SystemServices:: TRANSACTION_INFORMATION_CLASS, transactioninformation : *mut core::ffi::c_void, transactioninformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn ZwQueryInformationTransactionManager(transactionmanagerhandle : super::super::super::Win32::Foundation:: HANDLE, transactionmanagerinformationclass : super::super::super::Win32::System::SystemServices:: TRANSACTIONMANAGER_INFORMATION_CLASS, transactionmanagerinformation : *mut core::ffi::c_void, transactionmanagerinformationlength : u32, returnlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwQuerySymbolicLinkObject(linkhandle : super::super::super::Win32::Foundation:: HANDLE, linktarget : *mut super::super::super::Win32::Foundation:: UNICODE_STRING, returnedlength : *mut u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("ntdll.dll" "system" fn ZwReadFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *mut core::ffi::c_void, length : u32, byteoffset : *const i64, key : *const u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwReadOnlyEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwRecoverEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, enlistmentkey : *const core::ffi::c_void) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwRecoverResourceManager(resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwRecoverTransactionManager(transactionmanagerhandle : super::super::super::Win32::Foundation:: HANDLE) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwRegisterProtocolAddressInformation(resourcemanager : super::super::super::Win32::Foundation:: HANDLE, protocolid : *const windows_sys::core::GUID, protocolinformationsize : u32, protocolinformation : *const core::ffi::c_void, createoptions : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwRenameTransactionManager(logfilename : *const super::super::super::Win32::Foundation:: UNICODE_STRING, existingtransactionmanagerguid : *const windows_sys::core::GUID) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwRollbackComplete(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwRollbackEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwRollbackTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, wait : super::super::super::Win32::Foundation:: BOOLEAN) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwRollforwardTransactionManager(transactionmanagerhandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn ZwSetInformationEnlistment(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, enlistmentinformationclass : super::super::super::Win32::System::SystemServices:: ENLISTMENT_INFORMATION_CLASS, enlistmentinformation : *const core::ffi::c_void, enlistmentinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(all(feature = "Wdk_Storage_FileSystem", feature = "Win32_System_IO"))]
windows_targets::link!("ntdll.dll" "system" fn ZwSetInformationFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, fileinformation : *const core::ffi::c_void, length : u32, fileinformationclass : super::super::Storage::FileSystem:: FILE_INFORMATION_CLASS) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn ZwSetInformationResourceManager(resourcemanagerhandle : super::super::super::Win32::Foundation:: HANDLE, resourcemanagerinformationclass : super::super::super::Win32::System::SystemServices:: RESOURCEMANAGER_INFORMATION_CLASS, resourcemanagerinformation : *const core::ffi::c_void, resourcemanagerinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn ZwSetInformationTransaction(transactionhandle : super::super::super::Win32::Foundation:: HANDLE, transactioninformationclass : super::super::super::Win32::System::SystemServices:: TRANSACTION_INFORMATION_CLASS, transactioninformation : *const core::ffi::c_void, transactioninformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_SystemServices")]
windows_targets::link!("ntdll.dll" "system" fn ZwSetInformationTransactionManager(tmhandle : super::super::super::Win32::Foundation:: HANDLE, transactionmanagerinformationclass : super::super::super::Win32::System::SystemServices:: TRANSACTIONMANAGER_INFORMATION_CLASS, transactionmanagerinformation : *const core::ffi::c_void, transactionmanagerinformationlength : u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwSinglePhaseReject(enlistmenthandle : super::super::super::Win32::Foundation:: HANDLE, tmvirtualclock : *const i64) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn ZwUnloadDriver(driverservicename : *const super::super::super::Win32::Foundation:: UNICODE_STRING) -> super::super::super::Win32::Foundation:: NTSTATUS);
#[cfg(feature = "Win32_System_IO")]
windows_targets::link!("ntdll.dll" "system" fn ZwWriteFile(filehandle : super::super::super::Win32::Foundation:: HANDLE, event : super::super::super::Win32::Foundation:: HANDLE, apcroutine : super::super::super::Win32::System::IO:: PIO_APC_ROUTINE, apccontext : *const core::ffi::c_void, iostatusblock : *mut super::super::super::Win32::System::IO:: IO_STATUS_BLOCK, buffer : *const core::ffi::c_void, length : u32, byteoffset : *const i64, key : *const u32) -> super::super::super::Win32::Foundation:: NTSTATUS);
windows_targets::link!("ntdll.dll" "system" fn vDbgPrintEx(componentid : u32, level : u32, format : windows_sys::core::PCSTR, arglist : *const i8) -> u32);
windows_targets::link!("ntdll.dll" "system" fn vDbgPrintExWithPrefix(prefix : windows_sys::core::PCSTR, componentid : u32, level : u32, format : windows_sys::core::PCSTR, arglist : *const i8) -> u32);
pub const ACPIBus: INTERFACE_TYPE = 17i32;
pub const ADAPTER_INFO_API_BYPASS: u32 = 2u32;
pub const ADAPTER_INFO_SYNCHRONOUS_CALLBACK: u32 = 1u32;
pub const ALLOC_DATA_PRAGMA: u32 = 1u32;
pub const ALLOC_PRAGMA: u32 = 1u32;
pub const ANY_SIZE: u32 = 1u32;
pub const APC_LEVEL: u32 = 1u32;
pub const ARBITER_FLAG_BOOT_CONFIG: u32 = 1u32;
pub const ARBITER_FLAG_OTHER_ENUM: u32 = 4u32;
pub const ARBITER_FLAG_ROOT_ENUM: u32 = 2u32;
pub const ARBITER_PARTIAL: u32 = 1u32;
pub const ARM64_PCR_RESERVED_MASK: u32 = 4095u32;
pub const ARM_PROCESSOR_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe19e3d16_bc11_11e4_9caa_c2051d5d46b0);
pub const ATS_DEVICE_SVM_OPTOUT: u32 = 1u32;
pub const AccessFlagFault: FAULT_INFORMATION_ARM64_TYPE = 5i32;
pub const AddressSizeFault: FAULT_INFORMATION_ARM64_TYPE = 1i32;
pub const AgpControl: EXTENDED_AGP_REGISTER = 1i32;
pub const AllLoggerHandlesClass: TRACE_INFORMATION_CLASS = 6i32;
pub const AperturePageSize: EXTENDED_AGP_REGISTER = 3i32;
pub const ApertureSize: EXTENDED_AGP_REGISTER = 2i32;
pub const ApicDestinationModeLogicalClustered: HAL_APIC_DESTINATION_MODE = 3i32;
pub const ApicDestinationModeLogicalFlat: HAL_APIC_DESTINATION_MODE = 2i32;
pub const ApicDestinationModePhysical: HAL_APIC_DESTINATION_MODE = 1i32;
pub const ApicDestinationModeUnknown: HAL_APIC_DESTINATION_MODE = 4i32;
pub const ArbiterActionAddReserved: ARBITER_ACTION = 8i32;
pub const ArbiterActionBootAllocation: ARBITER_ACTION = 9i32;
pub const ArbiterActionCommitAllocation: ARBITER_ACTION = 2i32;
pub const ArbiterActionQueryAllocatedResources: ARBITER_ACTION = 4i32;
pub const ArbiterActionQueryArbitrate: ARBITER_ACTION = 7i32;
pub const ArbiterActionQueryConflict: ARBITER_ACTION = 6i32;
pub const ArbiterActionRetestAllocation: ARBITER_ACTION = 1i32;
pub const ArbiterActionRollbackAllocation: ARBITER_ACTION = 3i32;
pub const ArbiterActionTestAllocation: ARBITER_ACTION = 0i32;
pub const ArbiterActionWriteReservedResources: ARBITER_ACTION = 5i32;
pub const ArbiterRequestHalReported: ARBITER_REQUEST_SOURCE = 1i32;
pub const ArbiterRequestLegacyAssigned: ARBITER_REQUEST_SOURCE = 2i32;
pub const ArbiterRequestLegacyReported: ARBITER_REQUEST_SOURCE = 0i32;
pub const ArbiterRequestPnpDetected: ARBITER_REQUEST_SOURCE = 3i32;
pub const ArbiterRequestPnpEnumerated: ARBITER_REQUEST_SOURCE = 4i32;
pub const ArbiterRequestUndefined: ARBITER_REQUEST_SOURCE = -1i32;
pub const ArbiterResultExternalConflict: ARBITER_RESULT = 1i32;
pub const ArbiterResultNullRequest: ARBITER_RESULT = 2i32;
pub const ArbiterResultSuccess: ARBITER_RESULT = 0i32;
pub const ArbiterResultUndefined: ARBITER_RESULT = -1i32;
pub const ArcSystem: CONFIGURATION_TYPE = 0i32;
pub const AssignSecurityDescriptor: SECURITY_OPERATION_CODE = 3i32;
pub const AudioController: CONFIGURATION_TYPE = 23i32;
pub const BMC_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x487565ba_6494_4367_95ca_4eff893522f6);
pub const BOOT_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3d61a466_ab40_409a_a698_f362d464b38f);
pub const BackgroundWorkQueue: WORK_QUEUE_TYPE = 4i32;
pub const BdCbClassificationEnd: BDCB_CLASSIFICATION = 4i32;
pub const BdCbClassificationKnownBadImage: BDCB_CLASSIFICATION = 2i32;
pub const BdCbClassificationKnownBadImageBootCritical: BDCB_CLASSIFICATION = 3i32;
pub const BdCbClassificationKnownGoodImage: BDCB_CLASSIFICATION = 1i32;
pub const BdCbClassificationUnknownImage: BDCB_CLASSIFICATION = 0i32;
pub const BdCbInitializeImage: BDCB_CALLBACK_TYPE = 1i32;
pub const BdCbStatusPrepareForDependencyLoad: BDCB_STATUS_UPDATE_TYPE = 0i32;
pub const BdCbStatusPrepareForDriverLoad: BDCB_STATUS_UPDATE_TYPE = 1i32;
pub const BdCbStatusPrepareForUnload: BDCB_STATUS_UPDATE_TYPE = 2i32;
pub const BdCbStatusUpdate: BDCB_CALLBACK_TYPE = 0i32;
pub const BoundExceptionContinueSearch: BOUND_CALLBACK_STATUS = 0i32;
pub const BoundExceptionError: BOUND_CALLBACK_STATUS = 2i32;
pub const BoundExceptionHandled: BOUND_CALLBACK_STATUS = 1i32;
pub const BoundExceptionMaximum: BOUND_CALLBACK_STATUS = 3i32;
pub const BufferEmpty: KBUGCHECK_BUFFER_DUMP_STATE = 0i32;
pub const BufferFinished: KBUGCHECK_BUFFER_DUMP_STATE = 3i32;
pub const BufferIncomplete: KBUGCHECK_BUFFER_DUMP_STATE = 4i32;
pub const BufferInserted: KBUGCHECK_BUFFER_DUMP_STATE = 1i32;
pub const BufferStarted: KBUGCHECK_BUFFER_DUMP_STATE = 2i32;
pub const BusQueryCompatibleIDs: BUS_QUERY_ID_TYPE = 2i32;
pub const BusQueryContainerID: BUS_QUERY_ID_TYPE = 5i32;
pub const BusQueryDeviceID: BUS_QUERY_ID_TYPE = 0i32;
pub const BusQueryDeviceSerialNumber: BUS_QUERY_ID_TYPE = 4i32;
pub const BusQueryHardwareIDs: BUS_QUERY_ID_TYPE = 1i32;
pub const BusQueryInstanceID: BUS_QUERY_ID_TYPE = 3i32;
pub const BusRelations: DEVICE_RELATION_TYPE = 0i32;
pub const BusWidth32Bits: PCI_BUS_WIDTH = 0i32;
pub const BusWidth64Bits: PCI_BUS_WIDTH = 1i32;
pub const CBus: INTERFACE_TYPE = 9i32;
pub const CLFS_MAX_CONTAINER_INFO: u32 = 256u32;
pub const CLFS_SCAN_BACKWARD: u8 = 4u8;
pub const CLFS_SCAN_BUFFERED: u8 = 32u8;
pub const CLFS_SCAN_CLOSE: u8 = 8u8;
pub const CLFS_SCAN_FORWARD: u8 = 2u8;
pub const CLFS_SCAN_INIT: u8 = 1u8;
pub const CLFS_SCAN_INITIALIZED: u8 = 16u8;
pub const CLOCK1_LEVEL: u32 = 28u32;
pub const CLOCK2_LEVEL: u32 = 28u32;
pub const CLOCK_LEVEL: u32 = 28u32;
pub const CMCI_LEVEL: u32 = 5u32;
pub const CMCI_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x919448b2_3739_4b7f_a8f1_e0062805c2a3);
pub const CMC_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2dce8bb1_bdd7_450e_b9ad_9cf4ebd4f890);
pub const CM_RESOURCE_CONNECTION_CLASS_FUNCTION_CONFIG: u32 = 3u32;
pub const CM_RESOURCE_CONNECTION_CLASS_GPIO: u32 = 1u32;
pub const CM_RESOURCE_CONNECTION_CLASS_SERIAL: u32 = 2u32;
pub const CM_RESOURCE_CONNECTION_TYPE_FUNCTION_CONFIG: u32 = 1u32;
pub const CM_RESOURCE_CONNECTION_TYPE_GPIO_IO: u32 = 2u32;
pub const CM_RESOURCE_CONNECTION_TYPE_SERIAL_I2C: u32 = 1u32;
pub const CM_RESOURCE_CONNECTION_TYPE_SERIAL_SPI: u32 = 2u32;
pub const CM_RESOURCE_CONNECTION_TYPE_SERIAL_UART: u32 = 3u32;
pub const CM_RESOURCE_DMA_16: u32 = 1u32;
pub const CM_RESOURCE_DMA_32: u32 = 2u32;
pub const CM_RESOURCE_DMA_8: u32 = 0u32;
pub const CM_RESOURCE_DMA_8_AND_16: u32 = 4u32;
pub const CM_RESOURCE_DMA_BUS_MASTER: u32 = 8u32;
pub const CM_RESOURCE_DMA_TYPE_A: u32 = 16u32;
pub const CM_RESOURCE_DMA_TYPE_B: u32 = 32u32;
pub const CM_RESOURCE_DMA_TYPE_F: u32 = 64u32;
pub const CM_RESOURCE_DMA_V3: u32 = 128u32;
pub const CM_RESOURCE_INTERRUPT_LATCHED: u32 = 1u32;
pub const CM_RESOURCE_INTERRUPT_LEVEL_LATCHED_BITS: u32 = 1u32;
pub const CM_RESOURCE_INTERRUPT_LEVEL_SENSITIVE: u32 = 0u32;
pub const CM_RESOURCE_INTERRUPT_MESSAGE: u32 = 2u32;
pub const CM_RESOURCE_INTERRUPT_POLICY_INCLUDED: u32 = 4u32;
pub const CM_RESOURCE_INTERRUPT_SECONDARY_INTERRUPT: u32 = 16u32;
pub const CM_RESOURCE_INTERRUPT_WAKE_HINT: u32 = 32u32;
pub const CM_RESOURCE_MEMORY_24: u32 = 16u32;
pub const CM_RESOURCE_MEMORY_BAR: u32 = 128u32;
pub const CM_RESOURCE_MEMORY_CACHEABLE: u32 = 32u32;
pub const CM_RESOURCE_MEMORY_COMBINEDWRITE: u32 = 8u32;
pub const CM_RESOURCE_MEMORY_COMPAT_FOR_INACCESSIBLE_RANGE: u32 = 256u32;
pub const CM_RESOURCE_MEMORY_LARGE: u32 = 3584u32;
pub const CM_RESOURCE_MEMORY_LARGE_40: u32 = 512u32;
pub const CM_RESOURCE_MEMORY_LARGE_40_MAXLEN: u64 = 1099511627520u64;
pub const CM_RESOURCE_MEMORY_LARGE_48: u32 = 1024u32;
pub const CM_RESOURCE_MEMORY_LARGE_48_MAXLEN: u64 = 281474976645120u64;
pub const CM_RESOURCE_MEMORY_LARGE_64: u32 = 2048u32;
pub const CM_RESOURCE_MEMORY_LARGE_64_MAXLEN: u64 = 18446744069414584320u64;
pub const CM_RESOURCE_MEMORY_PREFETCHABLE: u32 = 4u32;
pub const CM_RESOURCE_MEMORY_READ_ONLY: u32 = 1u32;
pub const CM_RESOURCE_MEMORY_READ_WRITE: u32 = 0u32;
pub const CM_RESOURCE_MEMORY_WINDOW_DECODE: u32 = 64u32;
pub const CM_RESOURCE_MEMORY_WRITEABILITY_MASK: u32 = 3u32;
pub const CM_RESOURCE_MEMORY_WRITE_ONLY: u32 = 2u32;
pub const CM_RESOURCE_PORT_10_BIT_DECODE: u32 = 4u32;
pub const CM_RESOURCE_PORT_12_BIT_DECODE: u32 = 8u32;
pub const CM_RESOURCE_PORT_16_BIT_DECODE: u32 = 16u32;
pub const CM_RESOURCE_PORT_BAR: u32 = 256u32;
pub const CM_RESOURCE_PORT_IO: u32 = 1u32;
pub const CM_RESOURCE_PORT_MEMORY: u32 = 0u32;
pub const CM_RESOURCE_PORT_PASSIVE_DECODE: u32 = 64u32;
pub const CM_RESOURCE_PORT_POSITIVE_DECODE: u32 = 32u32;
pub const CM_RESOURCE_PORT_WINDOW_DECODE: u32 = 128u32;
pub const CM_SERVICE_MEASURED_BOOT_LOAD: u32 = 32u32;
pub const CONNECT_CURRENT_VERSION: u32 = 5u32;
pub const CONNECT_FULLY_SPECIFIED: u32 = 1u32;
pub const CONNECT_FULLY_SPECIFIED_GROUP: u32 = 4u32;
pub const CONNECT_LINE_BASED: u32 = 2u32;
pub const CONNECT_MESSAGE_BASED: u32 = 3u32;
pub const CONNECT_MESSAGE_BASED_PASSIVE: u32 = 5u32;
pub const CP15_PCR_RESERVED_MASK: u32 = 4095u32;
pub const CPER_EMPTY_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const CPE_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x4e292f96_d843_4a55_a8c2_d481f27ebeee);
pub const CP_GET_ERROR: u32 = 2u32;
pub const CP_GET_NODATA: u32 = 1u32;
pub const CP_GET_SUCCESS: u32 = 0u32;
pub const CardPresent: PCI_EXPRESS_CARD_PRESENCE = 1i32;
pub const CbusConfiguration: BUS_DATA_TYPE = 3i32;
pub const CdromController: CONFIGURATION_TYPE = 15i32;
pub const CentralProcessor: CONFIGURATION_TYPE = 1i32;
pub const ClfsClientRecord: u8 = 3u8;
pub const ClfsContainerActive: u32 = 4u32;
pub const ClfsContainerActivePendingDelete: u32 = 8u32;
pub const ClfsContainerInactive: u32 = 2u32;
pub const ClfsContainerInitializing: u32 = 1u32;
pub const ClfsContainerPendingArchive: u32 = 16u32;
pub const ClfsContainerPendingArchiveAndDelete: u32 = 32u32;
pub const ClfsDataRecord: u8 = 1u8;
pub const ClfsNullRecord: u8 = 0u8;
pub const ClfsRestartRecord: u8 = 2u8;
pub const ClsContainerActive: u32 = 4u32;
pub const ClsContainerActivePendingDelete: u32 = 8u32;
pub const ClsContainerInactive: u32 = 2u32;
pub const ClsContainerInitializing: u32 = 1u32;
pub const ClsContainerPendingArchive: u32 = 16u32;
pub const ClsContainerPendingArchiveAndDelete: u32 = 32u32;
pub const CmResourceShareDeviceExclusive: CM_SHARE_DISPOSITION = 1i32;
pub const CmResourceShareDriverExclusive: CM_SHARE_DISPOSITION = 2i32;
pub const CmResourceShareShared: CM_SHARE_DISPOSITION = 3i32;
pub const CmResourceShareUndetermined: CM_SHARE_DISPOSITION = 0i32;
pub const CmResourceTypeBusNumber: u32 = 6u32;
pub const CmResourceTypeConfigData: u32 = 128u32;
pub const CmResourceTypeConnection: u32 = 132u32;
pub const CmResourceTypeDevicePrivate: u32 = 129u32;
pub const CmResourceTypeDeviceSpecific: u32 = 5u32;
pub const CmResourceTypeDma: u32 = 4u32;
pub const CmResourceTypeInterrupt: u32 = 2u32;
pub const CmResourceTypeMaximum: u32 = 8u32;
pub const CmResourceTypeMemory: u32 = 3u32;
pub const CmResourceTypeMemoryLarge: u32 = 7u32;
pub const CmResourceTypeMfCardConfig: u32 = 131u32;
pub const CmResourceTypeNonArbitrated: u32 = 128u32;
pub const CmResourceTypeNull: u32 = 0u32;
pub const CmResourceTypePcCardConfig: u32 = 130u32;
pub const CmResourceTypePort: u32 = 1u32;
pub const Cmos: BUS_DATA_TYPE = 0i32;
pub const CommonBufferConfigTypeHardwareAccessPermissions: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_TYPE = 2i32;
pub const CommonBufferConfigTypeLogicalAddressLimits: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_TYPE = 0i32;
pub const CommonBufferConfigTypeMax: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_TYPE = 3i32;
pub const CommonBufferConfigTypeSubSection: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_TYPE = 1i32;
pub const CommonBufferHardwareAccessMax: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_ACCESS_TYPE = 3i32;
pub const CommonBufferHardwareAccessReadOnly: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_ACCESS_TYPE = 0i32;
pub const CommonBufferHardwareAccessReadWrite: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_ACCESS_TYPE = 2i32;
pub const CommonBufferHardwareAccessWriteOnly: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_ACCESS_TYPE = 1i32;
pub const Compatible: DMA_SPEED = 0i32;
pub const ConfigurationSpaceUndefined: BUS_DATA_TYPE = -1i32;
pub const ContinueCompletion: IO_COMPLETION_ROUTINE_RESULT = 0i32;
pub const CreateFileTypeMailslot: CREATE_FILE_TYPE = 2i32;
pub const CreateFileTypeNamedPipe: CREATE_FILE_TYPE = 1i32;
pub const CreateFileTypeNone: CREATE_FILE_TYPE = 0i32;
pub const CriticalWorkQueue: WORK_QUEUE_TYPE = 0i32;
pub const CustomPriorityWorkQueue: WORK_QUEUE_TYPE = 32i32;
pub const D3COLD_SUPPORT_INTERFACE_VERSION: u32 = 1u32;
pub const DBG_DEVICE_FLAG_BARS_MAPPED: u32 = 2u32;
pub const DBG_DEVICE_FLAG_HAL_SCRATCH_ALLOCATED: u32 = 1u32;
pub const DBG_DEVICE_FLAG_HOST_VISIBLE_ALLOCATED: u32 = 32u32;
pub const DBG_DEVICE_FLAG_SCRATCH_ALLOCATED: u32 = 4u32;
pub const DBG_DEVICE_FLAG_SYNTHETIC: u32 = 16u32;
pub const DBG_DEVICE_FLAG_UNCACHED_MEMORY: u32 = 8u32;
pub const DBG_STATUS_BUGCHECK_FIRST: u32 = 3u32;
pub const DBG_STATUS_BUGCHECK_SECOND: u32 = 4u32;
pub const DBG_STATUS_CONTROL_C: u32 = 1u32;
pub const DBG_STATUS_DEBUG_CONTROL: u32 = 6u32;
pub const DBG_STATUS_FATAL: u32 = 5u32;
pub const DBG_STATUS_SYSRQ: u32 = 2u32;
pub const DBG_STATUS_WORKER: u32 = 7u32;
pub const DEFAULT_DEVICE_DRIVER_CREATOR_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x57217c8d_5e66_44fb_8033_9b74cacedf5b);
pub const DEVICE_DESCRIPTION_VERSION: u32 = 0u32;
pub const DEVICE_DESCRIPTION_VERSION1: u32 = 1u32;
pub const DEVICE_DESCRIPTION_VERSION2: u32 = 2u32;
pub const DEVICE_DESCRIPTION_VERSION3: u32 = 3u32;
pub const DEVICE_DRIVER_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0033f803_2e70_4e88_992c_6f26daf3db7a);
pub const DEVICE_INTERFACE_INCLUDE_NONACTIVE: u32 = 1u32;
pub const DEVICE_RESET_INTERFACE_VERSION: u32 = 1u32;
pub const DEVICE_RESET_INTERFACE_VERSION_1: u32 = 1u32;
pub const DEVICE_RESET_INTERFACE_VERSION_2: u32 = 2u32;
pub const DEVICE_RESET_INTERFACE_VERSION_3: u32 = 3u32;
pub const DIRECTORY_CREATE_OBJECT: u32 = 4u32;
pub const DIRECTORY_CREATE_SUBDIRECTORY: u32 = 8u32;
pub const DIRECTORY_QUERY: u32 = 1u32;
pub const DIRECTORY_TRAVERSE: u32 = 2u32;
pub const DISPATCH_LEVEL: u32 = 2u32;
pub const DMAV3_TRANFER_WIDTH_128: u32 = 4u32;
pub const DMAV3_TRANFER_WIDTH_16: u32 = 1u32;
pub const DMAV3_TRANFER_WIDTH_256: u32 = 5u32;
pub const DMAV3_TRANFER_WIDTH_32: u32 = 2u32;
pub const DMAV3_TRANFER_WIDTH_64: u32 = 3u32;
pub const DMAV3_TRANFER_WIDTH_8: u32 = 0u32;
pub const DMA_ADAPTER_INFO_VERSION1: u32 = 1u32;
pub const DMA_FAIL_ON_BOUNCE: u32 = 4u32;
pub const DMA_IOMMU_INTERFACE_EX_VERSION: u32 = 1u32;
pub const DMA_IOMMU_INTERFACE_EX_VERSION_1: u32 = 1u32;
pub const DMA_IOMMU_INTERFACE_EX_VERSION_2: u32 = 2u32;
pub const DMA_IOMMU_INTERFACE_EX_VERSION_MAX: u32 = 2u32;
pub const DMA_IOMMU_INTERFACE_EX_VERSION_MIN: u32 = 1u32;
pub const DMA_IOMMU_INTERFACE_VERSION: u32 = 1u32;
pub const DMA_IOMMU_INTERFACE_VERSION_1: u32 = 1u32;
pub const DMA_SYNCHRONOUS_CALLBACK: u32 = 1u32;
pub const DMA_TRANSFER_CONTEXT_SIZE_V1: u32 = 128u32;
pub const DMA_TRANSFER_CONTEXT_VERSION1: u32 = 1u32;
pub const DMA_TRANSFER_INFO_VERSION1: u32 = 1u32;
pub const DMA_TRANSFER_INFO_VERSION2: u32 = 2u32;
pub const DMA_ZERO_BUFFERS: u32 = 2u32;
pub const DOMAIN_COMMON_BUFFER_LARGE_PAGE: u32 = 1u32;
pub const DPC_NORMAL: u32 = 0u32;
pub const DPC_THREADED: u32 = 1u32;
pub const DPC_WATCHDOG_GLOBAL_TRIAGE_BLOCK_REVISION_1: u32 = 1u32;
pub const DPC_WATCHDOG_GLOBAL_TRIAGE_BLOCK_SIGNATURE: u32 = 2931740382u32;
pub const DRIVER_VERIFIER_FORCE_IRQL_CHECKING: u32 = 2u32;
pub const DRIVER_VERIFIER_INJECT_ALLOCATION_FAILURES: u32 = 4u32;
pub const DRIVER_VERIFIER_IO_CHECKING: u32 = 16u32;
pub const DRIVER_VERIFIER_SPECIAL_POOLING: u32 = 1u32;
pub const DRIVER_VERIFIER_TRACK_POOL_ALLOCATIONS: u32 = 8u32;
pub const DRS_LEVEL: u32 = 14u32;
pub const DRVO_BOOTREINIT_REGISTERED: u32 = 32u32;
pub const DRVO_BUILTIN_DRIVER: u32 = 4u32;
pub const DRVO_INITIALIZED: u32 = 16u32;
pub const DRVO_LEGACY_DRIVER: u32 = 2u32;
pub const DRVO_LEGACY_RESOURCES: u32 = 64u32;
pub const DRVO_REINIT_REGISTERED: u32 = 8u32;
pub const DRVO_UNLOAD_INVOKED: u32 = 1u32;
pub const DUPLICATE_SAME_ATTRIBUTES: u32 = 4u32;
pub const DeallocateObject: IO_ALLOCATION_ACTION = 2i32;
pub const DeallocateObjectKeepRegisters: IO_ALLOCATION_ACTION = 3i32;
pub const DelayExecution: KWAIT_REASON = 4i32;
pub const DelayedWorkQueue: WORK_QUEUE_TYPE = 1i32;
pub const DeleteSecurityDescriptor: SECURITY_OPERATION_CODE = 2i32;
pub const DeviceDirectoryData: DEVICE_DIRECTORY_TYPE = 0i32;
pub const DevicePowerState: POWER_STATE_TYPE = 1i32;
pub const DevicePropertyAddress: DEVICE_REGISTRY_PROPERTY = 16i32;
pub const DevicePropertyAllocatedResources: DEVICE_REGISTRY_PROPERTY = 21i32;
pub const DevicePropertyBootConfiguration: DEVICE_REGISTRY_PROPERTY = 3i32;
pub const DevicePropertyBootConfigurationTranslated: DEVICE_REGISTRY_PROPERTY = 4i32;
pub const DevicePropertyBusNumber: DEVICE_REGISTRY_PROPERTY = 14i32;
pub const DevicePropertyBusTypeGuid: DEVICE_REGISTRY_PROPERTY = 8204i32;
pub const DevicePropertyClassGuid: DEVICE_REGISTRY_PROPERTY = 4102i32;
pub const DevicePropertyClassName: DEVICE_REGISTRY_PROPERTY = 4101i32;
pub const DevicePropertyCompatibleIDs: DEVICE_REGISTRY_PROPERTY = 16386i32;
pub const DevicePropertyContainerID: DEVICE_REGISTRY_PROPERTY = 4118i32;
pub const DevicePropertyDeviceDescription: DEVICE_REGISTRY_PROPERTY = 4096i32;
pub const DevicePropertyDriverKeyName: DEVICE_REGISTRY_PROPERTY = 4103i32;
pub const DevicePropertyEnumeratorName: DEVICE_REGISTRY_PROPERTY = 4111i32;
pub const DevicePropertyFriendlyName: DEVICE_REGISTRY_PROPERTY = 4105i32;
pub const DevicePropertyHardwareID: DEVICE_REGISTRY_PROPERTY = 16385i32;
pub const DevicePropertyInstallState: DEVICE_REGISTRY_PROPERTY = 18i32;
pub const DevicePropertyLegacyBusType: DEVICE_REGISTRY_PROPERTY = 13i32;
pub const DevicePropertyLocationInformation: DEVICE_REGISTRY_PROPERTY = 4106i32;
pub const DevicePropertyManufacturer: DEVICE_REGISTRY_PROPERTY = 4104i32;
pub const DevicePropertyPhysicalDeviceObjectName: DEVICE_REGISTRY_PROPERTY = 4107i32;
pub const DevicePropertyRemovalPolicy: DEVICE_REGISTRY_PROPERTY = 19i32;
pub const DevicePropertyResourceRequirements: DEVICE_REGISTRY_PROPERTY = 20i32;
pub const DevicePropertyUINumber: DEVICE_REGISTRY_PROPERTY = 17i32;
pub const DeviceTextDescription: DEVICE_TEXT_TYPE = 0i32;
pub const DeviceTextLocationInformation: DEVICE_TEXT_TYPE = 1i32;
pub const DeviceUsageTypeBoot: DEVICE_USAGE_NOTIFICATION_TYPE = 4i32;
pub const DeviceUsageTypeDumpFile: DEVICE_USAGE_NOTIFICATION_TYPE = 3i32;
pub const DeviceUsageTypeGuestAssigned: DEVICE_USAGE_NOTIFICATION_TYPE = 6i32;
pub const DeviceUsageTypeHibernation: DEVICE_USAGE_NOTIFICATION_TYPE = 2i32;
pub const DeviceUsageTypePaging: DEVICE_USAGE_NOTIFICATION_TYPE = 1i32;
pub const DeviceUsageTypePostDisplay: DEVICE_USAGE_NOTIFICATION_TYPE = 5i32;
pub const DeviceUsageTypeUndefined: DEVICE_USAGE_NOTIFICATION_TYPE = 0i32;
pub const DeviceWakeDepthD0: DEVICE_WAKE_DEPTH = 1i32;
pub const DeviceWakeDepthD1: DEVICE_WAKE_DEPTH = 2i32;
pub const DeviceWakeDepthD2: DEVICE_WAKE_DEPTH = 3i32;
pub const DeviceWakeDepthD3cold: DEVICE_WAKE_DEPTH = 5i32;
pub const DeviceWakeDepthD3hot: DEVICE_WAKE_DEPTH = 4i32;
pub const DeviceWakeDepthMaximum: DEVICE_WAKE_DEPTH = 6i32;
pub const DeviceWakeDepthNotWakeable: DEVICE_WAKE_DEPTH = 0i32;
pub const DirectoryNotifyExtendedInformation: DIRECTORY_NOTIFY_INFORMATION_CLASS = 2i32;
pub const DirectoryNotifyFullInformation: DIRECTORY_NOTIFY_INFORMATION_CLASS = 3i32;
pub const DirectoryNotifyInformation: DIRECTORY_NOTIFY_INFORMATION_CLASS = 1i32;
pub const DirectoryNotifyMaximumInformation: DIRECTORY_NOTIFY_INFORMATION_CLASS = 4i32;
pub const DisabledControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 11i32;
pub const DiskController: CONFIGURATION_TYPE = 13i32;
pub const DiskIoNotifyRoutinesClass: TRACE_INFORMATION_CLASS = 11i32;
pub const DiskPeripheral: CONFIGURATION_TYPE = 25i32;
pub const DisplayController: CONFIGURATION_TYPE = 19i32;
pub const DmaAborted: DMA_COMPLETION_STATUS = 1i32;
pub const DmaCancelled: DMA_COMPLETION_STATUS = 3i32;
pub const DmaComplete: DMA_COMPLETION_STATUS = 0i32;
pub const DmaError: DMA_COMPLETION_STATUS = 2i32;
pub const DockingInformation: CONFIGURATION_TYPE = 38i32;
pub const DomainConfigurationArm64: DOMAIN_CONFIGURATION_ARCH = 0i32;
pub const DomainConfigurationInvalid: DOMAIN_CONFIGURATION_ARCH = 2i32;
pub const DomainConfigurationX64: DOMAIN_CONFIGURATION_ARCH = 1i32;
pub const DomainTypeMax: IOMMU_DMA_DOMAIN_TYPE = 3i32;
pub const DomainTypePassThrough: IOMMU_DMA_DOMAIN_TYPE = 1i32;
pub const DomainTypeTranslate: IOMMU_DMA_DOMAIN_TYPE = 0i32;
pub const DomainTypeUnmanaged: IOMMU_DMA_DOMAIN_TYPE = 2i32;
pub const DriverDirectoryData: DRIVER_DIRECTORY_TYPE = 1i32;
pub const DriverDirectoryImage: DRIVER_DIRECTORY_TYPE = 0i32;
pub const DriverDirectorySharedData: DRIVER_DIRECTORY_TYPE = 2i32;
pub const DriverRegKeyParameters: DRIVER_REGKEY_TYPE = 0i32;
pub const DriverRegKeyPersistentState: DRIVER_REGKEY_TYPE = 1i32;
pub const DriverRegKeySharedPersistentState: DRIVER_REGKEY_TYPE = 2i32;
pub const DrvRtPoolNxOptIn: DRIVER_RUNTIME_INIT_FLAGS = 1i32;
pub const DtiAdapter: CONFIGURATION_TYPE = 11i32;
pub const EFLAG_SIGN: u32 = 32768u32;
pub const EFLAG_ZERO: u32 = 16384u32;
pub const EISA_EMPTY_SLOT: u32 = 131u32;
pub const EISA_FREE_FORM_DATA: u32 = 64u32;
pub const EISA_FUNCTION_ENABLED: u32 = 128u32;
pub const EISA_HAS_DMA_ENTRY: u32 = 8u32;
pub const EISA_HAS_IRQ_ENTRY: u32 = 4u32;
pub const EISA_HAS_MEMORY_ENTRY: u32 = 2u32;
pub const EISA_HAS_PORT_INIT_ENTRY: u32 = 32u32;
pub const EISA_HAS_PORT_RANGE: u32 = 16u32;
pub const EISA_HAS_TYPE_ENTRY: u32 = 1u32;
pub const EISA_INVALID_BIOS_CALL: u32 = 134u32;
pub const EISA_INVALID_CONFIGURATION: u32 = 130u32;
pub const EISA_INVALID_FUNCTION: u32 = 129u32;
pub const EISA_INVALID_SLOT: u32 = 128u32;
pub const EISA_MEMORY_TYPE_RAM: u32 = 1u32;
pub const EISA_MORE_ENTRIES: u32 = 128u32;
pub const EISA_SYSTEM_MEMORY: u32 = 0u32;
pub const ERROR_LOG_LIMIT_SIZE: u32 = 240u32;
pub const ERROR_MAJOR_REVISION_SAL_03_00: u32 = 0u32;
pub const ERROR_MEMORY_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe429faf2_3cb7_11d4_bca7_0080c73c8881);
pub const ERROR_MINOR_REVISION_SAL_03_00: u32 = 2u32;
pub const ERROR_PCI_BUS_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe429faf4_3cb7_11d4_bca7_0080c73c8881);
pub const ERROR_PCI_COMPONENT_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe429faf6_3cb7_11d4_bca7_0080c73c8881);
pub const ERROR_PLATFORM_BUS_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe429faf9_3cb7_11d4_bca7_0080c73c8881);
pub const ERROR_PLATFORM_HOST_CONTROLLER_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe429faf8_3cb7_11d4_bca7_0080c73c8881);
pub const ERROR_PLATFORM_SPECIFIC_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe429faf7_3cb7_11d4_bca7_0080c73c8881);
pub const ERROR_PROCESSOR_STATE_PARAMETER_BUS_CHECK_MASK: u32 = 1u32;
pub const ERROR_PROCESSOR_STATE_PARAMETER_BUS_CHECK_SHIFT: u32 = 61u32;
pub const ERROR_PROCESSOR_STATE_PARAMETER_CACHE_CHECK_MASK: u32 = 1u32;
pub const ERROR_PROCESSOR_STATE_PARAMETER_CACHE_CHECK_SHIFT: u32 = 59u32;
pub const ERROR_PROCESSOR_STATE_PARAMETER_MICROARCH_CHECK_MASK: u32 = 1u32;
pub const ERROR_PROCESSOR_STATE_PARAMETER_MICROARCH_CHECK_SHIFT: u32 = 63u32;
pub const ERROR_PROCESSOR_STATE_PARAMETER_REG_CHECK_MASK: u32 = 1u32;
pub const ERROR_PROCESSOR_STATE_PARAMETER_REG_CHECK_SHIFT: u32 = 62u32;
pub const ERROR_PROCESSOR_STATE_PARAMETER_TLB_CHECK_MASK: u32 = 1u32;
pub const ERROR_PROCESSOR_STATE_PARAMETER_TLB_CHECK_SHIFT: u32 = 60u32;
pub const ERROR_PROCESSOR_STATE_PARAMETER_UNKNOWN_CHECK_MASK: u32 = 1u32;
pub const ERROR_PROCESSOR_STATE_PARAMETER_UNKNOWN_CHECK_SHIFT: u32 = 63u32;
pub const ERROR_SMBIOS_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe429faf5_3cb7_11d4_bca7_0080c73c8881);
pub const ERROR_SYSTEM_EVENT_LOG_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe429faf3_3cb7_11d4_bca7_0080c73c8881);
pub const ERRTYP_BUS: u32 = 16u32;
pub const ERRTYP_CACHE: u32 = 6u32;
pub const ERRTYP_FLOW: u32 = 9u32;
pub const ERRTYP_FUNCTION: u32 = 7u32;
pub const ERRTYP_IMPROPER: u32 = 18u32;
pub const ERRTYP_INTERNAL: u32 = 1u32;
pub const ERRTYP_LOSSOFLOCKSTEP: u32 = 20u32;
pub const ERRTYP_MAP: u32 = 17u32;
pub const ERRTYP_MEM: u32 = 4u32;
pub const ERRTYP_PARITY: u32 = 22u32;
pub const ERRTYP_PATHERROR: u32 = 24u32;
pub const ERRTYP_POISONED: u32 = 26u32;
pub const ERRTYP_PROTOCOL: u32 = 23u32;
pub const ERRTYP_RESPONSE: u32 = 21u32;
pub const ERRTYP_SELFTEST: u32 = 8u32;
pub const ERRTYP_TIMEOUT: u32 = 25u32;
pub const ERRTYP_TLB: u32 = 5u32;
pub const ERRTYP_UNIMPL: u32 = 19u32;
pub const EVENT_QUERY_STATE: u32 = 1u32;
pub const EXCEPTION_ALIGNMENT_CHECK: u32 = 17u32;
pub const EXCEPTION_BOUND_CHECK: u32 = 5u32;
pub const EXCEPTION_CP_FAULT: u32 = 21u32;
pub const EXCEPTION_DEBUG: u32 = 1u32;
pub const EXCEPTION_DIVIDED_BY_ZERO: u32 = 0u32;
pub const EXCEPTION_DOUBLE_FAULT: u32 = 8u32;
pub const EXCEPTION_GP_FAULT: u32 = 13u32;
pub const EXCEPTION_INT3: u32 = 3u32;
pub const EXCEPTION_INVALID_OPCODE: u32 = 6u32;
pub const EXCEPTION_INVALID_TSS: u32 = 10u32;
pub const EXCEPTION_NMI: u32 = 2u32;
pub const EXCEPTION_NPX_ERROR: u32 = 16u32;
pub const EXCEPTION_NPX_NOT_AVAILABLE: u32 = 7u32;
pub const EXCEPTION_NPX_OVERRUN: u32 = 9u32;
pub const EXCEPTION_RESERVED_TRAP: u32 = 15u32;
pub const EXCEPTION_SEGMENT_NOT_PRESENT: u32 = 11u32;
pub const EXCEPTION_SE_FAULT: u32 = 23u32;
pub const EXCEPTION_SOFTWARE_ORIGINATE: u32 = 128u32;
pub const EXCEPTION_STACK_FAULT: u32 = 12u32;
pub const EXCEPTION_VIRTUALIZATION_FAULT: u32 = 32u32;
pub const EXTINT_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfe84086e_b557_43cf_ac1b_17982e078470);
pub const EX_CARR_ALLOCATE_NONPAGED_POOL: u32 = 1u32;
pub const EX_CARR_ALLOCATE_PAGED_POOL: u32 = 0u32;
pub const EX_CARR_DISABLE_EXPANSION: u32 = 2u32;
pub const EX_CREATE_FLAG_FILE_DEST_OPEN_FOR_COPY: u32 = 2u32;
pub const EX_CREATE_FLAG_FILE_SOURCE_OPEN_FOR_COPY: u32 = 1u32;
pub const EX_DEFAULT_PUSH_LOCK_FLAGS: u32 = 0u32;
pub const EX_LOOKASIDE_LIST_EX_FLAGS_FAIL_NO_RAISE: u32 = 2u32;
pub const EX_LOOKASIDE_LIST_EX_FLAGS_RAISE_ON_FAIL: u32 = 1u32;
pub const EX_MAXIMUM_LOOKASIDE_DEPTH_BASE: u32 = 256u32;
pub const EX_MAXIMUM_LOOKASIDE_DEPTH_LIMIT: u32 = 1024u32;
pub const EX_RUNDOWN_ACTIVE: u32 = 1u32;
pub const EX_RUNDOWN_COUNT_SHIFT: u32 = 1u32;
pub const EX_TIMER_HIGH_RESOLUTION: u32 = 4u32;
pub const EX_TIMER_NO_WAKE: u32 = 8u32;
pub const Eisa: INTERFACE_TYPE = 2i32;
pub const EisaAdapter: CONFIGURATION_TYPE = 8i32;
pub const EisaConfiguration: BUS_DATA_TYPE = 1i32;
pub const EjectionRelations: DEVICE_RELATION_TYPE = 1i32;
pub const EndAlternatives: ALTERNATIVE_ARCHITECTURE_TYPE = 2i32;
pub const EventCategoryDeviceInterfaceChange: IO_NOTIFICATION_EVENT_CATEGORY = 2i32;
pub const EventCategoryHardwareProfileChange: IO_NOTIFICATION_EVENT_CATEGORY = 1i32;
pub const EventCategoryKernelSoftRestart: IO_NOTIFICATION_EVENT_CATEGORY = 4i32;
pub const EventCategoryReserved: IO_NOTIFICATION_EVENT_CATEGORY = 0i32;
pub const EventCategoryTargetDeviceChange: IO_NOTIFICATION_EVENT_CATEGORY = 3i32;
pub const EventLoggerHandleClass: TRACE_INFORMATION_CLASS = 5i32;
pub const Executive: KWAIT_REASON = 0i32;
pub const ExternalFault: FAULT_INFORMATION_ARM64_TYPE = 3i32;
pub const FILE_128_BYTE_ALIGNMENT: u32 = 127u32;
pub const FILE_256_BYTE_ALIGNMENT: u32 = 255u32;
pub const FILE_32_BYTE_ALIGNMENT: u32 = 31u32;
pub const FILE_512_BYTE_ALIGNMENT: u32 = 511u32;
pub const FILE_64_BYTE_ALIGNMENT: u32 = 63u32;
pub const FILE_ATTRIBUTE_VALID_FLAGS: u32 = 32695u32;
pub const FILE_ATTRIBUTE_VALID_KERNEL_SET_FLAGS: u32 = 5910951u32;
pub const FILE_ATTRIBUTE_VALID_SET_FLAGS: u32 = 12711u32;
pub const FILE_AUTOGENERATED_DEVICE_NAME: u32 = 128u32;
pub const FILE_BYTE_ALIGNMENT: u32 = 0u32;
pub const FILE_CHARACTERISTICS_EXPECT_ORDERLY_REMOVAL: u32 = 512u32;
pub const FILE_CHARACTERISTICS_EXPECT_ORDERLY_REMOVAL_DEPRECATED: u32 = 512u32;
pub const FILE_CHARACTERISTICS_EXPECT_ORDERLY_REMOVAL_EX: u32 = 16384u32;
pub const FILE_CHARACTERISTICS_EXPECT_SURPRISE_REMOVAL: u32 = 768u32;
pub const FILE_CHARACTERISTICS_EXPECT_SURPRISE_REMOVAL_DEPRECATED: u32 = 768u32;
pub const FILE_CHARACTERISTICS_EXPECT_SURPRISE_REMOVAL_EX: u32 = 32768u32;
pub const FILE_CHARACTERISTICS_REMOVAL_POLICY_MASK: u32 = 768u32;
pub const FILE_CHARACTERISTICS_REMOVAL_POLICY_MASK_DEPRECATED: u32 = 768u32;
pub const FILE_CHARACTERISTICS_REMOVAL_POLICY_MASK_EX: u32 = 768u32;
pub const FILE_CHARACTERISTIC_CSV: u32 = 65536u32;
pub const FILE_CHARACTERISTIC_PNP_DEVICE: u32 = 2048u32;
pub const FILE_CHARACTERISTIC_TS_DEVICE: u32 = 4096u32;
pub const FILE_CHARACTERISTIC_WEBDAV_DEVICE: u32 = 8192u32;
pub const FILE_DEVICE_ALLOW_APPCONTAINER_TRAVERSAL: u32 = 131072u32;
pub const FILE_DEVICE_IS_MOUNTED: u32 = 32u32;
pub const FILE_DEVICE_REQUIRE_SECURITY_CHECK: u32 = 1048576u32;
pub const FILE_DEVICE_SECURE_OPEN: u32 = 256u32;
pub const FILE_FLOPPY_DISKETTE: u32 = 4u32;
pub const FILE_LONG_ALIGNMENT: u32 = 3u32;
pub const FILE_OCTA_ALIGNMENT: u32 = 15u32;
pub const FILE_PORTABLE_DEVICE: u32 = 262144u32;
pub const FILE_QUAD_ALIGNMENT: u32 = 7u32;
pub const FILE_QUERY_INDEX_SPECIFIED: u32 = 4u32;
pub const FILE_QUERY_NO_CURSOR_UPDATE: u32 = 16u32;
pub const FILE_QUERY_RESTART_SCAN: u32 = 1u32;
pub const FILE_QUERY_RETURN_ON_DISK_ENTRIES_ONLY: u32 = 8u32;
pub const FILE_QUERY_RETURN_SINGLE_ENTRY: u32 = 2u32;
pub const FILE_READ_ONLY_DEVICE: u32 = 2u32;
pub const FILE_REMOTE_DEVICE: u32 = 16u32;
pub const FILE_REMOTE_DEVICE_VSMB: u32 = 524288u32;
pub const FILE_REMOVABLE_MEDIA: u32 = 1u32;
pub const FILE_SHARE_VALID_FLAGS: u32 = 7u32;
pub const FILE_SKIP_SET_USER_EVENT_ON_FAST_IO: u32 = 4u32;
pub const FILE_USE_FILE_POINTER_POSITION: u32 = 4294967294u32;
pub const FILE_VALID_EXTENDED_OPTION_FLAGS: u32 = 268435456u32;
pub const FILE_VIRTUAL_VOLUME: u32 = 64u32;
pub const FILE_WORD_ALIGNMENT: u32 = 1u32;
pub const FILE_WRITE_ONCE_MEDIA: u32 = 8u32;
pub const FILE_WRITE_TO_END_OF_FILE: u32 = 4294967295u32;
pub const FIRMWARE_ERROR_RECORD_REFERENCE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x81212a96_09ed_4996_9471_8d729c8e69ed);
pub const FLAG_OWNER_POINTER_IS_THREAD: u32 = 1u32;
pub const FLUSH_MULTIPLE_MAXIMUM: u32 = 32u32;
pub const FM_LOCK_BIT: u32 = 1u32;
pub const FM_LOCK_BIT_V: u32 = 0u32;
pub const FO_ALERTABLE_IO: u32 = 4u32;
pub const FO_BYPASS_IO_ENABLED: u32 = 8388608u32;
pub const FO_CACHE_SUPPORTED: u32 = 64u32;
pub const FO_CLEANUP_COMPLETE: u32 = 16384u32;
pub const FO_DELETE_ON_CLOSE: u32 = 65536u32;
pub const FO_DIRECT_DEVICE_OPEN: u32 = 2048u32;
pub const FO_DISALLOW_EXCLUSIVE: u32 = 33554432u32;
pub const FO_FILE_FAST_IO_READ: u32 = 524288u32;
pub const FO_FILE_MODIFIED: u32 = 4096u32;
pub const FO_FILE_OPEN: u32 = 1u32;
pub const FO_FILE_OPEN_CANCELLED: u32 = 2097152u32;
pub const FO_FILE_SIZE_CHANGED: u32 = 8192u32;
pub const FO_FLAGS_VALID_ONLY_DURING_CREATE: u32 = 33554432u32;
pub const FO_GENERATE_AUDIT_ON_CLOSE: u32 = 1024u32;
pub const FO_HANDLE_CREATED: u32 = 262144u32;
pub const FO_INDIRECT_WAIT_OBJECT: u32 = 268435456u32;
pub const FO_MAILSLOT: u32 = 512u32;
pub const FO_NAMED_PIPE: u32 = 128u32;
pub const FO_NO_INTERMEDIATE_BUFFERING: u32 = 8u32;
pub const FO_OPENED_CASE_SENSITIVE: u32 = 131072u32;
pub const FO_QUEUE_IRP_TO_THREAD: u32 = 1024u32;
pub const FO_RANDOM_ACCESS: u32 = 1048576u32;
pub const FO_REMOTE_ORIGIN: u32 = 16777216u32;
pub const FO_SECTION_MINSTORE_TREATMENT: u32 = 536870912u32;
pub const FO_SEQUENTIAL_ONLY: u32 = 32u32;
pub const FO_SKIP_COMPLETION_PORT: u32 = 33554432u32;
pub const FO_SKIP_SET_EVENT: u32 = 67108864u32;
pub const FO_SKIP_SET_FAST_IO: u32 = 134217728u32;
pub const FO_STREAM_FILE: u32 = 256u32;
pub const FO_SYNCHRONOUS_IO: u32 = 2u32;
pub const FO_TEMPORARY_FILE: u32 = 32768u32;
pub const FO_VOLUME_OPEN: u32 = 4194304u32;
pub const FO_WRITE_THROUGH: u32 = 16u32;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_16GB: u32 = 6u32;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_1GB: u32 = 2u32;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_1MB: u32 = 9u32;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_256MB: u32 = 0u32;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_2GB: u32 = 3u32;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_32GB: u32 = 7u32;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_4GB: u32 = 4u32;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_512MB: u32 = 1u32;
pub const FPB_MEM_HIGH_VECTOR_GRANULARITY_8GB: u32 = 5u32;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_16MB: u32 = 4u32;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_1MB: u32 = 0u32;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_2MB: u32 = 1u32;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_4MB: u32 = 2u32;
pub const FPB_MEM_LOW_VECTOR_GRANULARITY_8MB: u32 = 3u32;
pub const FPB_MEM_VECTOR_GRANULARITY_1B: u32 = 8u32;
pub const FPB_RID_VECTOR_GRANULARITY_256RIDS: u32 = 5u32;
pub const FPB_RID_VECTOR_GRANULARITY_64RIDS: u32 = 3u32;
pub const FPB_RID_VECTOR_GRANULARITY_8RIDS: u32 = 0u32;
pub const FPB_VECTOR_SELECT_MEM_HIGH: u32 = 2u32;
pub const FPB_VECTOR_SELECT_MEM_LOW: u32 = 1u32;
pub const FPB_VECTOR_SELECT_RID: u32 = 0u32;
pub const FPB_VECTOR_SIZE_SUPPORTED_1KBITS: u32 = 2u32;
pub const FPB_VECTOR_SIZE_SUPPORTED_256BITS: u32 = 0u32;
pub const FPB_VECTOR_SIZE_SUPPORTED_2KBITS: u32 = 3u32;
pub const FPB_VECTOR_SIZE_SUPPORTED_4KBITS: u32 = 4u32;
pub const FPB_VECTOR_SIZE_SUPPORTED_512BITS: u32 = 1u32;
pub const FPB_VECTOR_SIZE_SUPPORTED_8KBITS: u32 = 5u32;
pub const FailControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 4i32;
pub const FaultInformationArm64: FAULT_INFORMATION_ARCH = 1i32;
pub const FaultInformationInvalid: FAULT_INFORMATION_ARCH = 0i32;
pub const FaultInformationX64: FAULT_INFORMATION_ARCH = 2i32;
pub const FloatingPointProcessor: CONFIGURATION_TYPE = 2i32;
pub const FloppyDiskPeripheral: CONFIGURATION_TYPE = 26i32;
pub const FltIoNotifyRoutinesClass: TRACE_INFORMATION_CLASS = 13i32;
pub const FreePage: KWAIT_REASON = 1i32;
pub const FunctionLevelDeviceReset: DEVICE_RESET_TYPE = 0i32;
pub const GENERIC_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x3e62a467_ab40_409a_a698_f362d464b38f);
pub const GENERIC_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe71254e8_c1b9_4940_ab76_909703a4320f);
pub const GENPROC_FLAGS_CORRECTED: u32 = 8u32;
pub const GENPROC_FLAGS_OVERFLOW: u32 = 4u32;
pub const GENPROC_FLAGS_PRECISEIP: u32 = 2u32;
pub const GENPROC_FLAGS_RESTARTABLE: u32 = 1u32;
pub const GENPROC_OP_DATAREAD: u32 = 1u32;
pub const GENPROC_OP_DATAWRITE: u32 = 2u32;
pub const GENPROC_OP_GENERIC: u32 = 0u32;
pub const GENPROC_OP_INSTRUCTIONEXE: u32 = 3u32;
pub const GENPROC_PROCERRTYPE_BUS: u32 = 4u32;
pub const GENPROC_PROCERRTYPE_CACHE: u32 = 1u32;
pub const GENPROC_PROCERRTYPE_MAE: u32 = 8u32;
pub const GENPROC_PROCERRTYPE_TLB: u32 = 2u32;
pub const GENPROC_PROCERRTYPE_UNKNOWN: u32 = 0u32;
pub const GENPROC_PROCISA_ARM32: u32 = 4u32;
pub const GENPROC_PROCISA_ARM64: u32 = 8u32;
pub const GENPROC_PROCISA_IPF: u32 = 1u32;
pub const GENPROC_PROCISA_X64: u32 = 2u32;
pub const GENPROC_PROCISA_X86: u32 = 0u32;
pub const GENPROC_PROCTYPE_ARM: u32 = 2u32;
pub const GENPROC_PROCTYPE_IPF: u32 = 1u32;
pub const GENPROC_PROCTYPE_XPF: u32 = 0u32;
pub const GUID_ECP_CREATE_USER_PROCESS: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe0e429ff_6ddc_4e65_aab6_45d05a038a08);
pub const GartHigh: EXTENDED_AGP_REGISTER = 5i32;
pub const GartLow: EXTENDED_AGP_REGISTER = 4i32;
pub const GenericEqual: RTL_GENERIC_COMPARE_RESULTS = 2i32;
pub const GenericGreaterThan: RTL_GENERIC_COMPARE_RESULTS = 1i32;
pub const GenericLessThan: RTL_GENERIC_COMPARE_RESULTS = 0i32;
pub const GlobalLoggerHandleClass: TRACE_INFORMATION_CLASS = 4i32;
pub const GroupAffinityAllGroupZero: IRQ_GROUP_POLICY = 0i32;
pub const GroupAffinityDontCare: IRQ_GROUP_POLICY = 1i32;
pub const HAL_DISPATCH_VERSION: u32 = 5u32;
pub const HAL_DMA_ADAPTER_VERSION_1: u32 = 1u32;
pub const HAL_MASK_UNMASK_FLAGS_NONE: u32 = 0u32;
pub const HAL_MASK_UNMASK_FLAGS_SERVICING_COMPLETE: u32 = 2u32;
pub const HAL_MASK_UNMASK_FLAGS_SERVICING_DEFERRED: u32 = 1u32;
pub const HAL_MCA_RECORD: MCA_EXCEPTION_TYPE = 1i32;
pub const HAL_MCE_RECORD: MCA_EXCEPTION_TYPE = 0i32;
pub const HAL_PLATFORM_ACPI_TABLES_CACHED: i32 = 32i32;
pub const HAL_PLATFORM_DISABLE_PTCG: i32 = 4i32;
pub const HAL_PLATFORM_DISABLE_UC_MAIN_MEMORY: i32 = 8i32;
pub const HAL_PLATFORM_DISABLE_WRITE_COMBINING: i32 = 1i32;
pub const HAL_PLATFORM_ENABLE_WRITE_COMBINING_MMIO: i32 = 16i32;
pub const HASH_STRING_ALGORITHM_DEFAULT: u32 = 0u32;
pub const HASH_STRING_ALGORITHM_INVALID: u32 = 4294967295u32;
pub const HASH_STRING_ALGORITHM_X65599: u32 = 1u32;
pub const HIGH_LEVEL: u32 = 31u32;
pub const HIGH_PRIORITY: u32 = 31u32;
pub const HalAcpiAuditInformation: HAL_QUERY_INFORMATION_CLASS = 26i32;
pub const HalCallbackInformation: HAL_QUERY_INFORMATION_CLASS = 5i32;
pub const HalChannelTopologyInformation: HAL_QUERY_INFORMATION_CLASS = 31i32;
pub const HalCmcLog: HAL_SET_INFORMATION_CLASS = 7i32;
pub const HalCmcLogInformation: HAL_QUERY_INFORMATION_CLASS = 13i32;
pub const HalCmcRegisterDriver: HAL_SET_INFORMATION_CLASS = 4i32;
pub const HalCpeLog: HAL_SET_INFORMATION_CLASS = 8i32;
pub const HalCpeLogInformation: HAL_QUERY_INFORMATION_CLASS = 14i32;
pub const HalCpeRegisterDriver: HAL_SET_INFORMATION_CLASS = 5i32;
pub const HalDisplayBiosInformation: HAL_QUERY_INFORMATION_CLASS = 9i32;
pub const HalDisplayEmulatedBios: HAL_DISPLAY_BIOS_INFORMATION = 1i32;
pub const HalDisplayInt10Bios: HAL_DISPLAY_BIOS_INFORMATION = 0i32;
pub const HalDisplayNoBios: HAL_DISPLAY_BIOS_INFORMATION = 2i32;
pub const HalDmaCrashDumpRegisterSet1: HAL_DMA_CRASH_DUMP_REGISTER_TYPE = 0i32;
pub const HalDmaCrashDumpRegisterSet2: HAL_DMA_CRASH_DUMP_REGISTER_TYPE = 1i32;
pub const HalDmaCrashDumpRegisterSetMax: HAL_DMA_CRASH_DUMP_REGISTER_TYPE = 2i32;
pub const HalDmaRemappingInformation: HAL_QUERY_INFORMATION_CLASS = 47i32;
pub const HalEnlightenment: HAL_SET_INFORMATION_CLASS = 11i32;
pub const HalErrorInformation: HAL_QUERY_INFORMATION_CLASS = 12i32;
pub const HalExternalCacheInformation: HAL_QUERY_INFORMATION_CLASS = 32i32;
pub const HalFrameBufferCachingInformation: HAL_QUERY_INFORMATION_CLASS = 8i32;
pub const HalFrequencyInformation: HAL_QUERY_INFORMATION_CLASS = 22i32;
pub const HalFwBootPerformanceInformation: HAL_QUERY_INFORMATION_CLASS = 34i32;
pub const HalFwS3PerformanceInformation: HAL_QUERY_INFORMATION_CLASS = 35i32;
pub const HalGenerateCmcInterrupt: HAL_SET_INFORMATION_CLASS = 9i32;
pub const HalGetChannelPowerInformation: HAL_QUERY_INFORMATION_CLASS = 36i32;
pub const HalHardwareWatchdogInformation: HAL_QUERY_INFORMATION_CLASS = 46i32;
pub const HalHeterogeneousMemoryAttributesInterface: HAL_QUERY_INFORMATION_CLASS = 49i32;
pub const HalHypervisorInformation: HAL_QUERY_INFORMATION_CLASS = 24i32;
pub const HalI386ExceptionChainTerminatorInformation: HAL_SET_INFORMATION_CLASS = 15i32;
pub const HalInformationClassUnused1: HAL_QUERY_INFORMATION_CLASS = 2i32;
pub const HalInitLogInformation: HAL_QUERY_INFORMATION_CLASS = 21i32;
pub const HalInstalledBusInformation: HAL_QUERY_INFORMATION_CLASS = 0i32;
pub const HalInterruptControllerInformation: HAL_QUERY_INFORMATION_CLASS = 39i32;
pub const HalIrtInformation: HAL_QUERY_INFORMATION_CLASS = 27i32;
pub const HalKernelErrorHandler: HAL_SET_INFORMATION_CLASS = 3i32;
pub const HalMapRegisterInformation: HAL_QUERY_INFORMATION_CLASS = 6i32;
pub const HalMcaLog: HAL_SET_INFORMATION_CLASS = 6i32;
pub const HalMcaLogInformation: HAL_QUERY_INFORMATION_CLASS = 7i32;
pub const HalMcaRegisterDriver: HAL_SET_INFORMATION_CLASS = 2i32;
pub const HalNumaRangeTableInformation: HAL_QUERY_INFORMATION_CLASS = 30i32;
pub const HalNumaTopologyInterface: HAL_QUERY_INFORMATION_CLASS = 11i32;
pub const HalParkingPageInformation: HAL_QUERY_INFORMATION_CLASS = 29i32;
pub const HalPartitionIpiInterface: HAL_QUERY_INFORMATION_CLASS = 18i32;
pub const HalPlatformInformation: HAL_QUERY_INFORMATION_CLASS = 19i32;
pub const HalPlatformTimerInformation: HAL_QUERY_INFORMATION_CLASS = 25i32;
pub const HalPowerInformation: HAL_QUERY_INFORMATION_CLASS = 3i32;
pub const HalProcessorBrandString: HAL_QUERY_INFORMATION_CLASS = 23i32;
pub const HalProcessorFeatureInformation: HAL_QUERY_INFORMATION_CLASS = 10i32;
pub const HalProcessorSpeedInformation: HAL_QUERY_INFORMATION_CLASS = 4i32;
pub const HalProfileDpgoSourceInterruptHandler: HAL_SET_INFORMATION_CLASS = 12i32;
pub const HalProfileSourceAdd: HAL_SET_INFORMATION_CLASS = 20i32;
pub const HalProfileSourceInformation: HAL_QUERY_INFORMATION_CLASS = 1i32;
pub const HalProfileSourceInterruptHandler: HAL_SET_INFORMATION_CLASS = 1i32;
pub const HalProfileSourceInterval: HAL_SET_INFORMATION_CLASS = 0i32;
pub const HalProfileSourceRemove: HAL_SET_INFORMATION_CLASS = 21i32;
pub const HalProfileSourceTimerHandler: HAL_SET_INFORMATION_CLASS = 10i32;
pub const HalPsciInformation: HAL_QUERY_INFORMATION_CLASS = 38i32;
pub const HalQueryAMLIIllegalIOPortAddresses: HAL_QUERY_INFORMATION_CLASS = 16i32;
pub const HalQueryAcpiWakeAlarmSystemPowerStateInformation: HAL_QUERY_INFORMATION_CLASS = 43i32;
pub const HalQueryArmErrataInformation: HAL_QUERY_INFORMATION_CLASS = 41i32;
pub const HalQueryDebuggerInformation: HAL_QUERY_INFORMATION_CLASS = 33i32;
pub const HalQueryHyperlaunchEntrypoint: HAL_QUERY_INFORMATION_CLASS = 45i32;
pub const HalQueryIommuReservedRegionInformation: HAL_QUERY_INFORMATION_CLASS = 40i32;
pub const HalQueryMaxHotPlugMemoryAddress: HAL_QUERY_INFORMATION_CLASS = 17i32;
pub const HalQueryMcaInterface: HAL_QUERY_INFORMATION_CLASS = 15i32;
pub const HalQueryPerDeviceMsiLimitInformation: HAL_QUERY_INFORMATION_CLASS = 50i32;
pub const HalQueryProcessorEfficiencyInformation: HAL_QUERY_INFORMATION_CLASS = 42i32;
pub const HalQueryProfileCorruptionStatus: HAL_QUERY_INFORMATION_CLASS = 51i32;
pub const HalQueryProfileCounterOwnership: HAL_QUERY_INFORMATION_CLASS = 52i32;
pub const HalQueryProfileNumberOfCounters: HAL_QUERY_INFORMATION_CLASS = 44i32;
pub const HalQueryProfileSourceList: HAL_QUERY_INFORMATION_CLASS = 20i32;
pub const HalQueryStateElementInformation: HAL_QUERY_INFORMATION_CLASS = 37i32;
pub const HalQueryUnused0001: HAL_QUERY_INFORMATION_CLASS = 48i32;
pub const HalRegisterSecondaryInterruptInterface: HAL_SET_INFORMATION_CLASS = 13i32;
pub const HalSecondaryInterruptInformation: HAL_QUERY_INFORMATION_CLASS = 28i32;
pub const HalSetChannelPowerInformation: HAL_SET_INFORMATION_CLASS = 14i32;
pub const HalSetClockTimerMinimumInterval: HAL_SET_INFORMATION_CLASS = 23i32;
pub const HalSetHvciEnabled: HAL_SET_INFORMATION_CLASS = 18i32;
pub const HalSetProcessorTraceInterruptHandler: HAL_SET_INFORMATION_CLASS = 19i32;
pub const HalSetPsciSuspendMode: HAL_SET_INFORMATION_CLASS = 17i32;
pub const HalSetResetParkDisposition: HAL_SET_INFORMATION_CLASS = 16i32;
pub const HalSetSwInterruptHandler: HAL_SET_INFORMATION_CLASS = 22i32;
pub const HighImportance: KDPC_IMPORTANCE = 2i32;
pub const HighPagePriority: MM_PAGE_PRIORITY = 32i32;
pub const HighPoolPriority: EX_POOL_PRIORITY = 32i32;
pub const HighPoolPrioritySpecialPoolOverrun: EX_POOL_PRIORITY = 40i32;
pub const HighPoolPrioritySpecialPoolUnderrun: EX_POOL_PRIORITY = 41i32;
pub const HotSpareControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 7i32;
pub const HyperCriticalWorkQueue: WORK_QUEUE_TYPE = 2i32;
pub const IMAGE_ADDRESSING_MODE_32BIT: u32 = 3u32;
pub const INITIAL_PRIVILEGE_COUNT: u32 = 3u32;
pub const INIT_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcc5263e8_9308_454a_89d0_340bd39bc98e);
pub const INJECT_ERRTYPE_MEMORY_CORRECTABLE: u32 = 8u32;
pub const INJECT_ERRTYPE_MEMORY_UNCORRECTABLEFATAL: u32 = 32u32;
pub const INJECT_ERRTYPE_MEMORY_UNCORRECTABLENONFATAL: u32 = 16u32;
pub const INJECT_ERRTYPE_PCIEXPRESS_CORRECTABLE: u32 = 64u32;
pub const INJECT_ERRTYPE_PCIEXPRESS_UNCORRECTABLEFATAL: u32 = 256u32;
pub const INJECT_ERRTYPE_PCIEXPRESS_UNCORRECTABLENONFATAL: u32 = 128u32;
pub const INJECT_ERRTYPE_PLATFORM_CORRECTABLE: u32 = 512u32;
pub const INJECT_ERRTYPE_PLATFORM_UNCORRECTABLEFATAL: u32 = 2048u32;
pub const INJECT_ERRTYPE_PLATFORM_UNCORRECTABLENONFATAL: u32 = 1024u32;
pub const INJECT_ERRTYPE_PROCESSOR_CORRECTABLE: u32 = 1u32;
pub const INJECT_ERRTYPE_PROCESSOR_UNCORRECTABLEFATAL: u32 = 4u32;
pub const INJECT_ERRTYPE_PROCESSOR_UNCORRECTABLENONFATAL: u32 = 2u32;
pub const IOCTL_CANCEL_DEVICE_WAKE: u32 = 2719752u32;
pub const IOCTL_QUERY_DEVICE_POWER_STATE: u32 = 2703360u32;
pub const IOCTL_SET_DEVICE_WAKE: u32 = 2719748u32;
pub const IOMMU_ACCESS_NONE: u32 = 0u32;
pub const IOMMU_ACCESS_READ: u32 = 1u32;
pub const IOMMU_ACCESS_WRITE: u32 = 2u32;
pub const IO_ATTACH_DEVICE: u32 = 1024u32;
pub const IO_ATTRIBUTION_INFO_V1: u32 = 1u32;
pub const IO_CHECK_CREATE_PARAMETERS: u32 = 512u32;
pub const IO_CHECK_SHARE_ACCESS_DONT_CHECK_DELETE: u32 = 16u32;
pub const IO_CHECK_SHARE_ACCESS_DONT_CHECK_READ: u32 = 4u32;
pub const IO_CHECK_SHARE_ACCESS_DONT_CHECK_WRITE: u32 = 8u32;
pub const IO_CHECK_SHARE_ACCESS_DONT_UPDATE_FILE_OBJECT: u32 = 2u32;
pub const IO_CHECK_SHARE_ACCESS_FORCE_CHECK: u32 = 32u32;
pub const IO_CHECK_SHARE_ACCESS_FORCE_USING_SCB: u32 = 64u32;
pub const IO_CHECK_SHARE_ACCESS_UPDATE_SHARE_ACCESS: u32 = 1u32;
pub const IO_FORCE_ACCESS_CHECK: u32 = 1u32;
pub const IO_IGNORE_SHARE_ACCESS_CHECK: u32 = 2048u32;
pub const IO_KEYBOARD_INCREMENT: u32 = 6u32;
pub const IO_MOUSE_INCREMENT: u32 = 6u32;
pub const IO_NO_PARAMETER_CHECKING: u32 = 256u32;
pub const IO_PARALLEL_INCREMENT: u32 = 1u32;
pub const IO_REMOUNT: u32 = 1u32;
pub const IO_REPARSE: u32 = 0u32;
pub const IO_REPARSE_GLOBAL: u32 = 2u32;
pub const IO_RESOURCE_ALTERNATIVE: u32 = 8u32;
pub const IO_RESOURCE_DEFAULT: u32 = 2u32;
pub const IO_RESOURCE_PREFERRED: u32 = 1u32;
pub const IO_SERIAL_INCREMENT: u32 = 2u32;
pub const IO_SESSION_MAX_PAYLOAD_SIZE: i32 = 256i32;
pub const IO_SESSION_STATE_ALL_EVENTS: u32 = 4294967295u32;
pub const IO_SESSION_STATE_CONNECT_EVENT: u32 = 4u32;
pub const IO_SESSION_STATE_CREATION_EVENT: u32 = 1u32;
pub const IO_SESSION_STATE_DISCONNECT_EVENT: u32 = 8u32;
pub const IO_SESSION_STATE_LOGOFF_EVENT: u32 = 32u32;
pub const IO_SESSION_STATE_LOGON_EVENT: u32 = 16u32;
pub const IO_SESSION_STATE_TERMINATION_EVENT: u32 = 2u32;
pub const IO_SESSION_STATE_VALID_EVENT_MASK: u32 = 63u32;
pub const IO_SET_IRP_IO_ATTRIBUTION_FLAGS_MASK: u32 = 3u32;
pub const IO_SET_IRP_IO_ATTRIBUTION_FROM_PROCESS: u32 = 2u32;
pub const IO_SET_IRP_IO_ATTRIBUTION_FROM_THREAD: u32 = 1u32;
pub const IO_SHARE_ACCESS_NON_PRIMARY_STREAM: u32 = 128u32;
pub const IO_SHARE_ACCESS_NO_WRITE_PERMISSION: u32 = 2147483648u32;
pub const IO_SOUND_INCREMENT: u32 = 8u32;
pub const IO_TYPE_ADAPTER: u32 = 1u32;
pub const IO_TYPE_CONTROLLER: u32 = 2u32;
pub const IO_TYPE_CSQ: u32 = 2u32;
pub const IO_TYPE_CSQ_EX: u32 = 3u32;
pub const IO_TYPE_CSQ_IRP_CONTEXT: u32 = 1u32;
pub const IO_TYPE_DEVICE: u32 = 3u32;
pub const IO_TYPE_DEVICE_OBJECT_EXTENSION: u32 = 13u32;
pub const IO_TYPE_DRIVER: u32 = 4u32;
pub const IO_TYPE_ERROR_LOG: u32 = 11u32;
pub const IO_TYPE_ERROR_MESSAGE: u32 = 12u32;
pub const IO_TYPE_FILE: u32 = 5u32;
pub const IO_TYPE_IORING: u32 = 14u32;
pub const IO_TYPE_IRP: u32 = 6u32;
pub const IO_TYPE_MASTER_ADAPTER: u32 = 7u32;
pub const IO_TYPE_OPEN_PACKET: u32 = 8u32;
pub const IO_TYPE_TIMER: u32 = 9u32;
pub const IO_TYPE_VPB: u32 = 10u32;
pub const IO_VIDEO_INCREMENT: u32 = 1u32;
pub const IPF_SAL_RECORD_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6f3380d1_6eb0_497f_a578_4d4c65a71617);
pub const IPI_LEVEL: u32 = 29u32;
pub const IPMI_MSR_DUMP_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1c15b445_9b06_4667_ac25_33c056b88803);
pub const IRP_ALLOCATED_FIXED_SIZE: u32 = 4u32;
pub const IRP_ALLOCATED_MUST_SUCCEED: u32 = 2u32;
pub const IRP_ASSOCIATED_IRP: u32 = 8u32;
pub const IRP_BUFFERED_IO: u32 = 16u32;
pub const IRP_CLOSE_OPERATION: u32 = 1024u32;
pub const IRP_CREATE_OPERATION: u32 = 128u32;
pub const IRP_DEALLOCATE_BUFFER: u32 = 32u32;
pub const IRP_DEFER_IO_COMPLETION: u32 = 2048u32;
pub const IRP_HOLD_DEVICE_QUEUE: u32 = 8192u32;
pub const IRP_INPUT_OPERATION: u32 = 64u32;
pub const IRP_LOOKASIDE_ALLOCATION: u32 = 8u32;
pub const IRP_MJ_CLEANUP: u32 = 18u32;
pub const IRP_MJ_CLOSE: u32 = 2u32;
pub const IRP_MJ_CREATE: u32 = 0u32;
pub const IRP_MJ_CREATE_MAILSLOT: u32 = 19u32;
pub const IRP_MJ_CREATE_NAMED_PIPE: u32 = 1u32;
pub const IRP_MJ_DEVICE_CHANGE: u32 = 24u32;
pub const IRP_MJ_DEVICE_CONTROL: u32 = 14u32;
pub const IRP_MJ_DIRECTORY_CONTROL: u32 = 12u32;
pub const IRP_MJ_FILE_SYSTEM_CONTROL: u32 = 13u32;
pub const IRP_MJ_FLUSH_BUFFERS: u32 = 9u32;
pub const IRP_MJ_INTERNAL_DEVICE_CONTROL: u32 = 15u32;
pub const IRP_MJ_LOCK_CONTROL: u32 = 17u32;
pub const IRP_MJ_MAXIMUM_FUNCTION: u32 = 27u32;
pub const IRP_MJ_PNP: u32 = 27u32;
pub const IRP_MJ_PNP_POWER: u32 = 27u32;
pub const IRP_MJ_POWER: u32 = 22u32;
pub const IRP_MJ_QUERY_EA: u32 = 7u32;
pub const IRP_MJ_QUERY_INFORMATION: u32 = 5u32;
pub const IRP_MJ_QUERY_QUOTA: u32 = 25u32;
pub const IRP_MJ_QUERY_SECURITY: u32 = 20u32;
pub const IRP_MJ_QUERY_VOLUME_INFORMATION: u32 = 10u32;
pub const IRP_MJ_READ: u32 = 3u32;
pub const IRP_MJ_SCSI: u32 = 15u32;
pub const IRP_MJ_SET_EA: u32 = 8u32;
pub const IRP_MJ_SET_INFORMATION: u32 = 6u32;
pub const IRP_MJ_SET_QUOTA: u32 = 26u32;
pub const IRP_MJ_SET_SECURITY: u32 = 21u32;
pub const IRP_MJ_SET_VOLUME_INFORMATION: u32 = 11u32;
pub const IRP_MJ_SHUTDOWN: u32 = 16u32;
pub const IRP_MJ_SYSTEM_CONTROL: u32 = 23u32;
pub const IRP_MJ_WRITE: u32 = 4u32;
pub const IRP_MN_CANCEL_REMOVE_DEVICE: u32 = 3u32;
pub const IRP_MN_CANCEL_STOP_DEVICE: u32 = 6u32;
pub const IRP_MN_CHANGE_SINGLE_INSTANCE: u32 = 2u32;
pub const IRP_MN_CHANGE_SINGLE_ITEM: u32 = 3u32;
pub const IRP_MN_COMPLETE: u32 = 4u32;
pub const IRP_MN_COMPRESSED: u32 = 8u32;
pub const IRP_MN_DEVICE_ENUMERATED: u32 = 25u32;
pub const IRP_MN_DEVICE_USAGE_NOTIFICATION: u32 = 22u32;
pub const IRP_MN_DISABLE_COLLECTION: u32 = 7u32;
pub const IRP_MN_DISABLE_EVENTS: u32 = 5u32;
pub const IRP_MN_DPC: u32 = 1u32;
pub const IRP_MN_EJECT: u32 = 17u32;
pub const IRP_MN_ENABLE_COLLECTION: u32 = 6u32;
pub const IRP_MN_ENABLE_EVENTS: u32 = 4u32;
pub const IRP_MN_EXECUTE_METHOD: u32 = 9u32;
pub const IRP_MN_FILTER_RESOURCE_REQUIREMENTS: u32 = 13u32;
pub const IRP_MN_FLUSH_AND_PURGE: u32 = 1u32;
pub const IRP_MN_FLUSH_DATA_ONLY: u32 = 2u32;
pub const IRP_MN_FLUSH_DATA_SYNC_ONLY: u32 = 4u32;
pub const IRP_MN_FLUSH_NO_SYNC: u32 = 3u32;
pub const IRP_MN_KERNEL_CALL: u32 = 4u32;
pub const IRP_MN_LOAD_FILE_SYSTEM: u32 = 3u32;
pub const IRP_MN_LOCK: u32 = 1u32;
pub const IRP_MN_MDL: u32 = 2u32;
pub const IRP_MN_MOUNT_VOLUME: u32 = 1u32;
pub const IRP_MN_NORMAL: u32 = 0u32;
pub const IRP_MN_NOTIFY_CHANGE_DIRECTORY: u32 = 2u32;
pub const IRP_MN_NOTIFY_CHANGE_DIRECTORY_EX: u32 = 3u32;
pub const IRP_MN_POWER_SEQUENCE: u32 = 1u32;
pub const IRP_MN_QUERY_ALL_DATA: u32 = 0u32;
pub const IRP_MN_QUERY_BUS_INFORMATION: u32 = 21u32;
pub const IRP_MN_QUERY_CAPABILITIES: u32 = 9u32;
pub const IRP_MN_QUERY_DEVICE_RELATIONS: u32 = 7u32;
pub const IRP_MN_QUERY_DEVICE_TEXT: u32 = 12u32;
pub const IRP_MN_QUERY_DIRECTORY: u32 = 1u32;
pub const IRP_MN_QUERY_ID: u32 = 19u32;
pub const IRP_MN_QUERY_INTERFACE: u32 = 8u32;
pub const IRP_MN_QUERY_LEGACY_BUS_INFORMATION: u32 = 24u32;
pub const IRP_MN_QUERY_PNP_DEVICE_STATE: u32 = 20u32;
pub const IRP_MN_QUERY_POWER: u32 = 3u32;
pub const IRP_MN_QUERY_REMOVE_DEVICE: u32 = 1u32;
pub const IRP_MN_QUERY_RESOURCES: u32 = 10u32;
pub const IRP_MN_QUERY_RESOURCE_REQUIREMENTS: u32 = 11u32;
pub const IRP_MN_QUERY_SINGLE_INSTANCE: u32 = 1u32;
pub const IRP_MN_QUERY_STOP_DEVICE: u32 = 5u32;
pub const IRP_MN_READ_CONFIG: u32 = 15u32;
pub const IRP_MN_REGINFO: u32 = 8u32;
pub const IRP_MN_REGINFO_EX: u32 = 11u32;
pub const IRP_MN_REMOVE_DEVICE: u32 = 2u32;
pub const IRP_MN_SCSI_CLASS: u32 = 1u32;
pub const IRP_MN_SET_LOCK: u32 = 18u32;
pub const IRP_MN_SET_POWER: u32 = 2u32;
pub const IRP_MN_START_DEVICE: u32 = 0u32;
pub const IRP_MN_STOP_DEVICE: u32 = 4u32;
pub const IRP_MN_SURPRISE_REMOVAL: u32 = 23u32;
pub const IRP_MN_TRACK_LINK: u32 = 4u32;
pub const IRP_MN_UNLOCK_ALL: u32 = 3u32;
pub const IRP_MN_UNLOCK_ALL_BY_KEY: u32 = 4u32;
pub const IRP_MN_UNLOCK_SINGLE: u32 = 2u32;
pub const IRP_MN_USER_FS_REQUEST: u32 = 0u32;
pub const IRP_MN_VERIFY_VOLUME: u32 = 2u32;
pub const IRP_MN_WAIT_WAKE: u32 = 0u32;
pub const IRP_MN_WRITE_CONFIG: u32 = 16u32;
pub const IRP_MOUNT_COMPLETION: u32 = 2u32;
pub const IRP_NOCACHE: u32 = 1u32;
pub const IRP_OB_QUERY_NAME: u32 = 4096u32;
pub const IRP_PAGING_IO: u32 = 2u32;
pub const IRP_QUOTA_CHARGED: u32 = 1u32;
pub const IRP_READ_OPERATION: u32 = 256u32;
pub const IRP_SYNCHRONOUS_API: u32 = 4u32;
pub const IRP_SYNCHRONOUS_PAGING_IO: u32 = 64u32;
pub const IRP_UM_DRIVER_INITIATED_IO: u32 = 4194304u32;
pub const IRP_WRITE_OPERATION: u32 = 512u32;
pub const InACriticalArrayControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 8i32;
pub const InAFailedArrayControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 9i32;
pub const IndicatorBlink: PCI_EXPRESS_INDICATOR_STATE = 2i32;
pub const IndicatorOff: PCI_EXPRESS_INDICATOR_STATE = 3i32;
pub const IndicatorOn: PCI_EXPRESS_INDICATOR_STATE = 1i32;
pub const InitiateReset: NPEM_CONTROL_STANDARD_CONTROL_BIT = 1i32;
pub const InstallStateFailedInstall: DEVICE_INSTALL_STATE = 2i32;
pub const InstallStateFinishInstall: DEVICE_INSTALL_STATE = 3i32;
pub const InstallStateInstalled: DEVICE_INSTALL_STATE = 0i32;
pub const InstallStateNeedsReinstall: DEVICE_INSTALL_STATE = 1i32;
pub const IntelCacheData: INTEL_CACHE_TYPE = 1i32;
pub const IntelCacheInstruction: INTEL_CACHE_TYPE = 2i32;
pub const IntelCacheNull: INTEL_CACHE_TYPE = 0i32;
pub const IntelCacheRam: INTEL_CACHE_TYPE = 4i32;
pub const IntelCacheTrace: INTEL_CACHE_TYPE = 5i32;
pub const IntelCacheUnified: INTEL_CACHE_TYPE = 3i32;
pub const InterfaceTypeUndefined: INTERFACE_TYPE = -1i32;
pub const Internal: INTERFACE_TYPE = 0i32;
pub const InternalPowerBus: INTERFACE_TYPE = 13i32;
pub const InterruptActiveBoth: KINTERRUPT_POLARITY = 3i32;
pub const InterruptActiveBothTriggerHigh: KINTERRUPT_POLARITY = 4i32;
pub const InterruptActiveBothTriggerLow: KINTERRUPT_POLARITY = 3i32;
pub const InterruptActiveHigh: KINTERRUPT_POLARITY = 1i32;
pub const InterruptActiveLow: KINTERRUPT_POLARITY = 2i32;
pub const InterruptFallingEdge: KINTERRUPT_POLARITY = 2i32;
pub const InterruptPolarityUnknown: KINTERRUPT_POLARITY = 0i32;
pub const InterruptRisingEdge: KINTERRUPT_POLARITY = 1i32;
pub const InvalidDeviceTypeControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 10i32;
pub const IoMaxContainerInformationClass: IO_CONTAINER_INFORMATION_CLASS = 1i32;
pub const IoMaxContainerNotificationClass: IO_CONTAINER_NOTIFICATION_CLASS = 1i32;
pub const IoModifyAccess: LOCK_OPERATION = 2i32;
pub const IoPagingPriorityHigh: IO_PAGING_PRIORITY = 2i32;
pub const IoPagingPriorityInvalid: IO_PAGING_PRIORITY = 0i32;
pub const IoPagingPriorityNormal: IO_PAGING_PRIORITY = 1i32;
pub const IoPagingPriorityReserved1: IO_PAGING_PRIORITY = 3i32;
pub const IoPagingPriorityReserved2: IO_PAGING_PRIORITY = 4i32;
pub const IoQueryDeviceComponentInformation: IO_QUERY_DEVICE_DATA_FORMAT = 2i32;
pub const IoQueryDeviceConfigurationData: IO_QUERY_DEVICE_DATA_FORMAT = 1i32;
pub const IoQueryDeviceIdentifier: IO_QUERY_DEVICE_DATA_FORMAT = 0i32;
pub const IoQueryDeviceMaxData: IO_QUERY_DEVICE_DATA_FORMAT = 3i32;
pub const IoReadAccess: LOCK_OPERATION = 0i32;
pub const IoSessionEventConnected: IO_SESSION_EVENT = 3i32;
pub const IoSessionEventCreated: IO_SESSION_EVENT = 1i32;
pub const IoSessionEventDisconnected: IO_SESSION_EVENT = 4i32;
pub const IoSessionEventIgnore: IO_SESSION_EVENT = 0i32;
pub const IoSessionEventLogoff: IO_SESSION_EVENT = 6i32;
pub const IoSessionEventLogon: IO_SESSION_EVENT = 5i32;
pub const IoSessionEventMax: IO_SESSION_EVENT = 7i32;
pub const IoSessionEventTerminated: IO_SESSION_EVENT = 2i32;
pub const IoSessionStateConnected: IO_SESSION_STATE = 3i32;
pub const IoSessionStateCreated: IO_SESSION_STATE = 1i32;
pub const IoSessionStateDisconnected: IO_SESSION_STATE = 4i32;
pub const IoSessionStateDisconnectedLoggedOn: IO_SESSION_STATE = 5i32;
pub const IoSessionStateInformation: IO_CONTAINER_INFORMATION_CLASS = 0i32;
pub const IoSessionStateInitialized: IO_SESSION_STATE = 2i32;
pub const IoSessionStateLoggedOff: IO_SESSION_STATE = 7i32;
pub const IoSessionStateLoggedOn: IO_SESSION_STATE = 6i32;
pub const IoSessionStateMax: IO_SESSION_STATE = 9i32;
pub const IoSessionStateNotification: IO_CONTAINER_NOTIFICATION_CLASS = 0i32;
pub const IoSessionStateTerminated: IO_SESSION_STATE = 8i32;
pub const IoWriteAccess: LOCK_OPERATION = 1i32;
pub const IommuDeviceCreationConfigTypeAcpi: IOMMU_DEVICE_CREATION_CONFIGURATION_TYPE = 1i32;
pub const IommuDeviceCreationConfigTypeDeviceId: IOMMU_DEVICE_CREATION_CONFIGURATION_TYPE = 2i32;
pub const IommuDeviceCreationConfigTypeMax: IOMMU_DEVICE_CREATION_CONFIGURATION_TYPE = 3i32;
pub const IommuDeviceCreationConfigTypeNone: IOMMU_DEVICE_CREATION_CONFIGURATION_TYPE = 0i32;
pub const IommuDmaLogicalAllocatorBuddy: IOMMU_DMA_LOGICAL_ALLOCATOR_TYPE = 1i32;
pub const IommuDmaLogicalAllocatorMax: IOMMU_DMA_LOGICAL_ALLOCATOR_TYPE = 2i32;
pub const IommuDmaLogicalAllocatorNone: IOMMU_DMA_LOGICAL_ALLOCATOR_TYPE = 0i32;
pub const IrqPolicyAllCloseProcessors: IRQ_DEVICE_POLICY = 1i32;
pub const IrqPolicyAllProcessorsInMachine: IRQ_DEVICE_POLICY = 3i32;
pub const IrqPolicyAllProcessorsInMachineWhenSteered: IRQ_DEVICE_POLICY = 6i32;
pub const IrqPolicyMachineDefault: IRQ_DEVICE_POLICY = 0i32;
pub const IrqPolicyOneCloseProcessor: IRQ_DEVICE_POLICY = 2i32;
pub const IrqPolicySpecifiedProcessors: IRQ_DEVICE_POLICY = 4i32;
pub const IrqPolicySpreadMessagesAcrossAllProcessors: IRQ_DEVICE_POLICY = 5i32;
pub const IrqPriorityHigh: IRQ_PRIORITY = 3i32;
pub const IrqPriorityLow: IRQ_PRIORITY = 1i32;
pub const IrqPriorityNormal: IRQ_PRIORITY = 2i32;
pub const IrqPriorityUndefined: IRQ_PRIORITY = 0i32;
pub const Isa: INTERFACE_TYPE = 1i32;
pub const IsochCommand: EXTENDED_AGP_REGISTER = 6i32;
pub const IsochStatus: EXTENDED_AGP_REGISTER = 0i32;
pub const KADDRESS_BASE: u32 = 0u32;
pub const KB_ADD_PAGES_FLAG_ADDITIONAL_RANGES_EXIST: u32 = 2147483648u32;
pub const KB_ADD_PAGES_FLAG_PHYSICAL_ADDRESS: u32 = 2u32;
pub const KB_ADD_PAGES_FLAG_VIRTUAL_ADDRESS: u32 = 1u32;
pub const KB_REMOVE_PAGES_FLAG_ADDITIONAL_RANGES_EXIST: u32 = 2147483648u32;
pub const KB_REMOVE_PAGES_FLAG_PHYSICAL_ADDRESS: u32 = 2u32;
pub const KB_REMOVE_PAGES_FLAG_VIRTUAL_ADDRESS: u32 = 1u32;
pub const KB_SECONDARY_DATA_FLAG_ADDITIONAL_DATA: u32 = 1u32;
pub const KB_SECONDARY_DATA_FLAG_NO_DEVICE_ACCESS: u32 = 2u32;
pub const KB_TRIAGE_DUMP_DATA_FLAG_BUGCHECK_ACTIVE: u32 = 1u32;
pub const KD_OPTION_SET_BLOCK_ENABLE: KD_OPTION = 0i32;
pub const KENCODED_TIMER_PROCESSOR: u32 = 1u32;
pub const KERNEL_LARGE_STACK_COMMIT: u32 = 12288u32;
pub const KERNEL_LARGE_STACK_SIZE: u32 = 61440u32;
pub const KERNEL_MCA_EXCEPTION_STACK_SIZE: u32 = 8192u32;
pub const KERNEL_SOFT_RESTART_NOTIFICATION_VERSION: u32 = 1u32;
pub const KERNEL_STACK_SIZE: u32 = 12288u32;
pub const KE_MAX_TRIAGE_DUMP_DATA_MEMORY_SIZE: u32 = 33554432u32;
pub const KE_PROCESSOR_CHANGE_ADD_EXISTING: u32 = 1u32;
pub const KI_USER_SHARED_DATA: u32 = 4292804608u32;
pub const KUMS_UCH_VOLATILE_BIT: u32 = 0u32;
pub const KbCallbackAddPages: KBUGCHECK_CALLBACK_REASON = 4i32;
pub const KbCallbackDumpIo: KBUGCHECK_CALLBACK_REASON = 3i32;
pub const KbCallbackInvalid: KBUGCHECK_CALLBACK_REASON = 0i32;
pub const KbCallbackRemovePages: KBUGCHECK_CALLBACK_REASON = 6i32;
pub const KbCallbackReserved1: KBUGCHECK_CALLBACK_REASON = 1i32;
pub const KbCallbackReserved2: KBUGCHECK_CALLBACK_REASON = 8i32;
pub const KbCallbackSecondaryDumpData: KBUGCHECK_CALLBACK_REASON = 2i32;
pub const KbCallbackSecondaryMultiPartDumpData: KBUGCHECK_CALLBACK_REASON = 5i32;
pub const KbCallbackTriageDumpData: KBUGCHECK_CALLBACK_REASON = 7i32;
pub const KbDumpIoBody: KBUGCHECK_DUMP_IO_TYPE = 2i32;
pub const KbDumpIoComplete: KBUGCHECK_DUMP_IO_TYPE = 4i32;
pub const KbDumpIoHeader: KBUGCHECK_DUMP_IO_TYPE = 1i32;
pub const KbDumpIoInvalid: KBUGCHECK_DUMP_IO_TYPE = 0i32;
pub const KbDumpIoSecondaryData: KBUGCHECK_DUMP_IO_TYPE = 3i32;
pub const KdConfigureDeviceAndContinue: KD_CALLBACK_ACTION = 0i32;
pub const KdConfigureDeviceAndStop: KD_CALLBACK_ACTION = 2i32;
pub const KdNameSpaceACPI: KD_NAMESPACE_ENUM = 1i32;
pub const KdNameSpaceAny: KD_NAMESPACE_ENUM = 2i32;
pub const KdNameSpaceMax: KD_NAMESPACE_ENUM = 4i32;
pub const KdNameSpaceNone: KD_NAMESPACE_ENUM = 3i32;
pub const KdNameSpacePCI: KD_NAMESPACE_ENUM = 0i32;
pub const KdSkipDeviceAndContinue: KD_CALLBACK_ACTION = 1i32;
pub const KdSkipDeviceAndStop: KD_CALLBACK_ACTION = 3i32;
pub const KeProcessorAddCompleteNotify: KE_PROCESSOR_CHANGE_NOTIFY_STATE = 1i32;
pub const KeProcessorAddFailureNotify: KE_PROCESSOR_CHANGE_NOTIFY_STATE = 2i32;
pub const KeProcessorAddStartNotify: KE_PROCESSOR_CHANGE_NOTIFY_STATE = 0i32;
pub const KeepObject: IO_ALLOCATION_ACTION = 1i32;
pub const KernelMode: MODE = 0i32;
pub const KeyboardController: CONFIGURATION_TYPE = 22i32;
pub const KeyboardPeripheral: CONFIGURATION_TYPE = 32i32;
pub const L0sAndL1EntryDisabled: PCI_EXPRESS_ASPM_CONTROL = 0i32;
pub const L0sAndL1EntryEnabled: PCI_EXPRESS_ASPM_CONTROL = 3i32;
pub const L0sAndL1EntrySupport: PCI_EXPRESS_ASPM_SUPPORT = 3i32;
pub const L0sEntryEnabled: PCI_EXPRESS_ASPM_CONTROL = 1i32;
pub const L0sEntrySupport: PCI_EXPRESS_ASPM_SUPPORT = 1i32;
pub const L0s_128ns_256ns: PCI_EXPRESS_L0s_EXIT_LATENCY = 2i32;
pub const L0s_1us_2us: PCI_EXPRESS_L0s_EXIT_LATENCY = 5i32;
pub const L0s_256ns_512ns: PCI_EXPRESS_L0s_EXIT_LATENCY = 3i32;
pub const L0s_2us_4us: PCI_EXPRESS_L0s_EXIT_LATENCY = 6i32;
pub const L0s_512ns_1us: PCI_EXPRESS_L0s_EXIT_LATENCY = 4i32;
pub const L0s_64ns_128ns: PCI_EXPRESS_L0s_EXIT_LATENCY = 1i32;
pub const L0s_Above4us: PCI_EXPRESS_L0s_EXIT_LATENCY = 7i32;
pub const L0s_Below64ns: PCI_EXPRESS_L0s_EXIT_LATENCY = 0i32;
pub const L1EntryEnabled: PCI_EXPRESS_ASPM_CONTROL = 2i32;
pub const L1EntrySupport: PCI_EXPRESS_ASPM_SUPPORT = 2i32;
pub const L1_16us_32us: PCI_EXPRESS_L1_EXIT_LATENCY = 5i32;
pub const L1_1us_2us: PCI_EXPRESS_L1_EXIT_LATENCY = 1i32;
pub const L1_2us_4us: PCI_EXPRESS_L1_EXIT_LATENCY = 2i32;
pub const L1_32us_64us: PCI_EXPRESS_L1_EXIT_LATENCY = 6i32;
pub const L1_4us_8us: PCI_EXPRESS_L1_EXIT_LATENCY = 3i32;
pub const L1_8us_16us: PCI_EXPRESS_L1_EXIT_LATENCY = 4i32;
pub const L1_Above64us: PCI_EXPRESS_L1_EXIT_LATENCY = 7i32;
pub const L1_Below1us: PCI_EXPRESS_L1_EXIT_LATENCY = 0i32;
pub const LOCK_QUEUE_HALTED: u32 = 4u32;
pub const LOCK_QUEUE_HALTED_BIT: u32 = 2u32;
pub const LOCK_QUEUE_OWNER: u32 = 2u32;
pub const LOCK_QUEUE_OWNER_BIT: u32 = 1u32;
pub const LOCK_QUEUE_WAIT: u32 = 1u32;
pub const LOCK_QUEUE_WAIT_BIT: u32 = 0u32;
pub const LONG_2ND_MOST_SIGNIFICANT_BIT: u32 = 2u32;
pub const LONG_3RD_MOST_SIGNIFICANT_BIT: u32 = 1u32;
pub const LONG_LEAST_SIGNIFICANT_BIT: u32 = 0u32;
pub const LONG_MOST_SIGNIFICANT_BIT: u32 = 3u32;
pub const LOWBYTE_MASK: u32 = 255u32;
pub const LOW_LEVEL: u32 = 0u32;
pub const LOW_PRIORITY: u32 = 0u32;
pub const LOW_REALTIME_PRIORITY: u32 = 16u32;
pub const LastDStateTransitionD3cold: D3COLD_LAST_TRANSITION_STATUS = 2i32;
pub const LastDStateTransitionD3hot: D3COLD_LAST_TRANSITION_STATUS = 1i32;
pub const LastDStateTransitionStatusUnknown: D3COLD_LAST_TRANSITION_STATUS = 0i32;
pub const LastDrvRtFlag: DRIVER_RUNTIME_INIT_FLAGS = 2i32;
pub const Latched: KINTERRUPT_MODE = 1i32;
pub const LevelSensitive: KINTERRUPT_MODE = 0i32;
pub const LinePeripheral: CONFIGURATION_TYPE = 35i32;
pub const LocateControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 3i32;
pub const LocationTypeFileSystem: STATE_LOCATION_TYPE = 1i32;
pub const LocationTypeMaximum: STATE_LOCATION_TYPE = 2i32;
pub const LocationTypeRegistry: STATE_LOCATION_TYPE = 0i32;
pub const LoggerEventsLoggedClass: TRACE_INFORMATION_CLASS = 10i32;
pub const LoggerEventsLostClass: TRACE_INFORMATION_CLASS = 8i32;
pub const LowImportance: KDPC_IMPORTANCE = 0i32;
pub const LowPagePriority: MM_PAGE_PRIORITY = 0i32;
pub const LowPoolPriority: EX_POOL_PRIORITY = 0i32;
pub const LowPoolPrioritySpecialPoolOverrun: EX_POOL_PRIORITY = 8i32;
pub const LowPoolPrioritySpecialPoolUnderrun: EX_POOL_PRIORITY = 9i32;
pub const MAXIMUM_DEBUG_BARS: u32 = 6u32;
pub const MAXIMUM_FILENAME_LENGTH: u32 = 256u32;
pub const MAXIMUM_PRIORITY: u32 = 32u32;
pub const MAX_EVENT_COUNTERS: u32 = 31u32;
pub const MCA_EXTREG_V2MAX: u32 = 24u32;
pub const MCE_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe8f56ffe_919c_4cc5_ba88_65abe14913bb);
pub const MDL_ALLOCATED_FIXED_SIZE: u32 = 8u32;
pub const MDL_DESCRIBES_AWE: u32 = 1024u32;
pub const MDL_FREE_EXTRA_PTES: u32 = 512u32;
pub const MDL_INTERNAL: u32 = 32768u32;
pub const MDL_LOCKED_PAGE_TABLES: u32 = 256u32;
pub const MDL_PAGE_CONTENTS_INVARIANT: u32 = 16384u32;
pub const MEMORY_CORRECTABLE_ERROR_SUMMARY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0e36c93e_ca15_4a83_ba8a_cbe80f7f0017);
pub const MEMORY_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa5bc1114_6f64_4ede_b863_3e83ed7c83b1);
pub const MEM_COMMIT: u32 = 4096u32;
pub const MEM_DECOMMIT: u32 = 16384u32;
pub const MEM_EXTENDED_PARAMETER_EC_CODE: u32 = 64u32;
pub const MEM_EXTENDED_PARAMETER_TYPE_BITS: u32 = 8u32;
pub const MEM_LARGE_PAGES: u32 = 536870912u32;
pub const MEM_MAPPED: u32 = 262144u32;
pub const MEM_PRIVATE: u32 = 131072u32;
pub const MEM_RELEASE: u32 = 32768u32;
pub const MEM_RESERVE: u32 = 8192u32;
pub const MEM_RESET: u32 = 524288u32;
pub const MEM_RESET_UNDO: u32 = 16777216u32;
pub const MEM_TOP_DOWN: u32 = 1048576u32;
pub const MM_ADD_PHYSICAL_MEMORY_ALREADY_ZEROED: u32 = 1u32;
pub const MM_ADD_PHYSICAL_MEMORY_HUGE_PAGES_ONLY: u32 = 4u32;
pub const MM_ADD_PHYSICAL_MEMORY_LARGE_PAGES_ONLY: u32 = 2u32;
pub const MM_ALLOCATE_AND_HOT_REMOVE: u32 = 256u32;
pub const MM_ALLOCATE_CONTIGUOUS_MEMORY_FAST_ONLY: u32 = 1u32;
pub const MM_ALLOCATE_FAST_LARGE_PAGES: u32 = 64u32;
pub const MM_ALLOCATE_FROM_LOCAL_NODE_ONLY: u32 = 2u32;
pub const MM_ALLOCATE_FULLY_REQUIRED: u32 = 4u32;
pub const MM_ALLOCATE_NO_WAIT: u32 = 8u32;
pub const MM_ALLOCATE_PREFER_CONTIGUOUS: u32 = 16u32;
pub const MM_ALLOCATE_REQUIRE_CONTIGUOUS_CHUNKS: u32 = 32u32;
pub const MM_ALLOCATE_TRIM_IF_NECESSARY: u32 = 128u32;
pub const MM_ANY_NODE_OK: u32 = 2147483648u32;
pub const MM_COPY_MEMORY_PHYSICAL: u32 = 1u32;
pub const MM_COPY_MEMORY_VIRTUAL: u32 = 2u32;
pub const MM_DONT_ZERO_ALLOCATION: u32 = 1u32;
pub const MM_DUMP_MAP_CACHED: u32 = 1u32;
pub const MM_DUMP_MAP_INVALIDATE: u32 = 2u32;
pub const MM_FREE_MDL_PAGES_ZERO: u32 = 1u32;
pub const MM_GET_CACHE_ATTRIBUTE_IO_SPACE: u32 = 1u32;
pub const MM_GET_PHYSICAL_MEMORY_RANGES_INCLUDE_ALL_PARTITIONS: u32 = 2u32;
pub const MM_GET_PHYSICAL_MEMORY_RANGES_INCLUDE_FILE_ONLY: u32 = 1u32;
pub const MM_MAPPING_ADDRESS_DIVISIBLE: u32 = 1u32;
pub const MM_MAXIMUM_DISK_IO_SIZE: u32 = 65536u32;
pub const MM_PERMANENT_ADDRESS_IS_IO_SPACE: u32 = 1u32;
pub const MM_PROTECT_DRIVER_SECTION_ALLOW_UNLOAD: u32 = 1u32;
pub const MM_PROTECT_DRIVER_SECTION_VALID_FLAGS: u32 = 1u32;
pub const MM_REMOVE_PHYSICAL_MEMORY_BAD_ONLY: u32 = 1u32;
pub const MM_SECURE_EXCLUSIVE: u32 = 1u32;
pub const MM_SECURE_NO_CHANGE: u32 = 2u32;
pub const MM_SECURE_NO_INHERIT: u32 = 8u32;
pub const MM_SECURE_USER_MODE_ONLY: u32 = 4u32;
pub const MM_SYSTEM_SPACE_END: u32 = 4294967295u32;
pub const MM_SYSTEM_VIEW_EXCEPTIONS_FOR_INPAGE_ERRORS: u32 = 1u32;
pub const MPIBus: INTERFACE_TYPE = 10i32;
pub const MPIConfiguration: BUS_DATA_TYPE = 8i32;
pub const MPSABus: INTERFACE_TYPE = 11i32;
pub const MPSAConfiguration: BUS_DATA_TYPE = 9i32;
pub const MRLClosed: PCI_EXPRESS_MRL_STATE = 0i32;
pub const MRLOpen: PCI_EXPRESS_MRL_STATE = 1i32;
pub const MU_TELEMETRY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x85183a8b_9c41_429c_939c_5c3c087ca280);
pub const MapPhysicalAddressTypeContiguousRange: IOMMU_MAP_PHYSICAL_ADDRESS_TYPE = 1i32;
pub const MapPhysicalAddressTypeMax: IOMMU_MAP_PHYSICAL_ADDRESS_TYPE = 3i32;
pub const MapPhysicalAddressTypeMdl: IOMMU_MAP_PHYSICAL_ADDRESS_TYPE = 0i32;
pub const MapPhysicalAddressTypePfn: IOMMU_MAP_PHYSICAL_ADDRESS_TYPE = 2i32;
pub const MaxFaultType: FAULT_INFORMATION_ARM64_TYPE = 7i32;
pub const MaxHardwareCounterType: HARDWARE_COUNTER_TYPE = 1i32;
pub const MaxPayload1024Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 3i32;
pub const MaxPayload128Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 0i32;
pub const MaxPayload2048Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 4i32;
pub const MaxPayload256Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 1i32;
pub const MaxPayload4096Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 5i32;
pub const MaxPayload512Bytes: PCI_EXPRESS_MAX_PAYLOAD_SIZE = 2i32;
pub const MaxRegNtNotifyClass: REG_NOTIFY_CLASS = 51i32;
pub const MaxSubsystemInformationType: SUBSYSTEM_INFORMATION_TYPE = 2i32;
pub const MaxTraceInformationClass: TRACE_INFORMATION_CLASS = 16i32;
pub const MaximumBusDataType: BUS_DATA_TYPE = 12i32;
pub const MaximumDmaSpeed: DMA_SPEED = 5i32;
pub const MaximumDmaWidth: DMA_WIDTH = 5i32;
pub const MaximumInterfaceType: INTERFACE_TYPE = 18i32;
pub const MaximumMode: MODE = 2i32;
pub const MaximumType: CONFIGURATION_TYPE = 41i32;
pub const MaximumWaitReason: KWAIT_REASON = 42i32;
pub const MaximumWorkQueue: WORK_QUEUE_TYPE = 7i32;
pub const MdlMappingNoExecute: u32 = 1073741824u32;
pub const MdlMappingNoWrite: u32 = 2147483648u32;
pub const MdlMappingWithGuardPtes: u32 = 536870912u32;
pub const MediumHighImportance: KDPC_IMPORTANCE = 3i32;
pub const MediumImportance: KDPC_IMPORTANCE = 1i32;
pub const MemDedicatedAttributeMax: MEM_DEDICATED_ATTRIBUTE_TYPE = 4i32;
pub const MemDedicatedAttributeReadBandwidth: MEM_DEDICATED_ATTRIBUTE_TYPE = 0i32;
pub const MemDedicatedAttributeReadLatency: MEM_DEDICATED_ATTRIBUTE_TYPE = 1i32;
pub const MemDedicatedAttributeWriteBandwidth: MEM_DEDICATED_ATTRIBUTE_TYPE = 2i32;
pub const MemDedicatedAttributeWriteLatency: MEM_DEDICATED_ATTRIBUTE_TYPE = 3i32;
pub const MemSectionExtendedParameterInvalidType: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 0i32;
pub const MemSectionExtendedParameterMax: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 4i32;
pub const MemSectionExtendedParameterNumaNode: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 2i32;
pub const MemSectionExtendedParameterSigningLevel: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 3i32;
pub const MemSectionExtendedParameterUserPhysicalFlags: MEM_SECTION_EXTENDED_PARAMETER_TYPE = 1i32;
pub const MicroChannel: INTERFACE_TYPE = 3i32;
pub const MmCached: MEMORY_CACHING_TYPE = 1i32;
pub const MmFrameBufferCached: MEMORY_CACHING_TYPE_ORIG = 2i32;
pub const MmHardwareCoherentCached: MEMORY_CACHING_TYPE = 3i32;
pub const MmLargeSystem: MM_SYSTEMSIZE = 2i32;
pub const MmMaximumCacheType: MEMORY_CACHING_TYPE = 6i32;
pub const MmMaximumRotateDirection: MM_ROTATE_DIRECTION = 4i32;
pub const MmMdlPageContentsDynamic: MM_MDL_PAGE_CONTENTS_STATE = 0i32;
pub const MmMdlPageContentsInvariant: MM_MDL_PAGE_CONTENTS_STATE = 1i32;
pub const MmMdlPageContentsQuery: MM_MDL_PAGE_CONTENTS_STATE = 2i32;
pub const MmMediumSystem: MM_SYSTEMSIZE = 1i32;
pub const MmNonCached: MEMORY_CACHING_TYPE = 0i32;
pub const MmNonCachedUnordered: MEMORY_CACHING_TYPE = 4i32;
pub const MmNotMapped: MEMORY_CACHING_TYPE = -1i32;
pub const MmSmallSystem: MM_SYSTEMSIZE = 0i32;
pub const MmToFrameBuffer: MM_ROTATE_DIRECTION = 0i32;
pub const MmToFrameBufferNoCopy: MM_ROTATE_DIRECTION = 1i32;
pub const MmToRegularMemory: MM_ROTATE_DIRECTION = 2i32;
pub const MmToRegularMemoryNoCopy: MM_ROTATE_DIRECTION = 3i32;
pub const MmUSWCCached: MEMORY_CACHING_TYPE = 5i32;
pub const MmWriteCombined: MEMORY_CACHING_TYPE = 2i32;
pub const ModemPeripheral: CONFIGURATION_TYPE = 28i32;
pub const ModifyAccess: IO_ACCESS_TYPE = 2i32;
pub const MonitorPeripheral: CONFIGURATION_TYPE = 29i32;
pub const MonitorRequestReasonAcDcDisplayBurst: POWER_MONITOR_REQUEST_REASON = 5i32;
pub const MonitorRequestReasonAcDcDisplayBurstSuppressed: POWER_MONITOR_REQUEST_REASON = 28i32;
pub const MonitorRequestReasonBatteryCountChange: POWER_MONITOR_REQUEST_REASON = 16i32;
pub const MonitorRequestReasonBatteryCountChangeSuppressed: POWER_MONITOR_REQUEST_REASON = 49i32;
pub const MonitorRequestReasonBatteryPreCritical: POWER_MONITOR_REQUEST_REASON = 53i32;
pub const MonitorRequestReasonBuiltinPanel: POWER_MONITOR_REQUEST_REASON = 47i32;
pub const MonitorRequestReasonDP: POWER_MONITOR_REQUEST_REASON = 19i32;
pub const MonitorRequestReasonDim: POWER_MONITOR_REQUEST_REASON = 46i32;
pub const MonitorRequestReasonDirectedDrips: POWER_MONITOR_REQUEST_REASON = 45i32;
pub const MonitorRequestReasonDisplayRequiredUnDim: POWER_MONITOR_REQUEST_REASON = 48i32;
pub const MonitorRequestReasonFullWake: POWER_MONITOR_REQUEST_REASON = 9i32;
pub const MonitorRequestReasonGracePeriod: POWER_MONITOR_REQUEST_REASON = 17i32;
pub const MonitorRequestReasonIdleTimeout: POWER_MONITOR_REQUEST_REASON = 12i32;
pub const MonitorRequestReasonLid: POWER_MONITOR_REQUEST_REASON = 15i32;
pub const MonitorRequestReasonMax: POWER_MONITOR_REQUEST_REASON = 55i32;
pub const MonitorRequestReasonNearProximity: POWER_MONITOR_REQUEST_REASON = 22i32;
pub const MonitorRequestReasonPdcSignal: POWER_MONITOR_REQUEST_REASON = 27i32;
pub const MonitorRequestReasonPdcSignalFingerprint: POWER_MONITOR_REQUEST_REASON = 44i32;
pub const MonitorRequestReasonPdcSignalHeyCortana: POWER_MONITOR_REQUEST_REASON = 42i32;
pub const MonitorRequestReasonPdcSignalHolographicShell: POWER_MONITOR_REQUEST_REASON = 43i32;
pub const MonitorRequestReasonPdcSignalSensorsHumanPresence: POWER_MONITOR_REQUEST_REASON = 52i32;
pub const MonitorRequestReasonPdcSignalWindowsMobilePwrNotif: POWER_MONITOR_REQUEST_REASON = 40i32;
pub const MonitorRequestReasonPdcSignalWindowsMobileShell: POWER_MONITOR_REQUEST_REASON = 41i32;
pub const MonitorRequestReasonPnP: POWER_MONITOR_REQUEST_REASON = 18i32;
pub const MonitorRequestReasonPoSetSystemState: POWER_MONITOR_REQUEST_REASON = 7i32;
pub const MonitorRequestReasonPolicyChange: POWER_MONITOR_REQUEST_REASON = 13i32;
pub const MonitorRequestReasonPowerButton: POWER_MONITOR_REQUEST_REASON = 1i32;
pub const MonitorRequestReasonRemoteConnection: POWER_MONITOR_REQUEST_REASON = 2i32;
pub const MonitorRequestReasonResumeModernStandby: POWER_MONITOR_REQUEST_REASON = 50i32;
pub const MonitorRequestReasonResumePdc: POWER_MONITOR_REQUEST_REASON = 24i32;
pub const MonitorRequestReasonResumeS4: POWER_MONITOR_REQUEST_REASON = 25i32;
pub const MonitorRequestReasonScMonitorpower: POWER_MONITOR_REQUEST_REASON = 3i32;
pub const MonitorRequestReasonScreenOffRequest: POWER_MONITOR_REQUEST_REASON = 11i32;
pub const MonitorRequestReasonSessionUnlock: POWER_MONITOR_REQUEST_REASON = 10i32;
pub const MonitorRequestReasonSetThreadExecutionState: POWER_MONITOR_REQUEST_REASON = 8i32;
pub const MonitorRequestReasonSleepButton: POWER_MONITOR_REQUEST_REASON = 14i32;
pub const MonitorRequestReasonSxTransition: POWER_MONITOR_REQUEST_REASON = 20i32;
pub const MonitorRequestReasonSystemIdle: POWER_MONITOR_REQUEST_REASON = 21i32;
pub const MonitorRequestReasonSystemStateEntered: POWER_MONITOR_REQUEST_REASON = 29i32;
pub const MonitorRequestReasonTerminal: POWER_MONITOR_REQUEST_REASON = 26i32;
pub const MonitorRequestReasonTerminalInit: POWER_MONITOR_REQUEST_REASON = 51i32;
pub const MonitorRequestReasonThermalStandby: POWER_MONITOR_REQUEST_REASON = 23i32;
pub const MonitorRequestReasonUnknown: POWER_MONITOR_REQUEST_REASON = 0i32;
pub const MonitorRequestReasonUserDisplayBurst: POWER_MONITOR_REQUEST_REASON = 6i32;
pub const MonitorRequestReasonUserInput: POWER_MONITOR_REQUEST_REASON = 4i32;
pub const MonitorRequestReasonUserInputAccelerometer: POWER_MONITOR_REQUEST_REASON = 35i32;
pub const MonitorRequestReasonUserInputHid: POWER_MONITOR_REQUEST_REASON = 36i32;
pub const MonitorRequestReasonUserInputInitialization: POWER_MONITOR_REQUEST_REASON = 39i32;
pub const MonitorRequestReasonUserInputKeyboard: POWER_MONITOR_REQUEST_REASON = 31i32;
pub const MonitorRequestReasonUserInputMouse: POWER_MONITOR_REQUEST_REASON = 32i32;
pub const MonitorRequestReasonUserInputPen: POWER_MONITOR_REQUEST_REASON = 34i32;
pub const MonitorRequestReasonUserInputPoUserPresent: POWER_MONITOR_REQUEST_REASON = 37i32;
pub const MonitorRequestReasonUserInputSessionSwitch: POWER_MONITOR_REQUEST_REASON = 38i32;
pub const MonitorRequestReasonUserInputTouch: POWER_MONITOR_REQUEST_REASON = 54i32;
pub const MonitorRequestReasonUserInputTouchpad: POWER_MONITOR_REQUEST_REASON = 33i32;
pub const MonitorRequestReasonWinrt: POWER_MONITOR_REQUEST_REASON = 30i32;
pub const MonitorRequestTypeOff: POWER_MONITOR_REQUEST_TYPE = 0i32;
pub const MonitorRequestTypeOnAndPresent: POWER_MONITOR_REQUEST_TYPE = 1i32;
pub const MonitorRequestTypeToggleOn: POWER_MONITOR_REQUEST_TYPE = 2i32;
pub const MultiFunctionAdapter: CONFIGURATION_TYPE = 12i32;
pub const NEC98x86: ALTERNATIVE_ARCHITECTURE_TYPE = 1i32;
pub const NMI_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5bad89ff_b7e6_42c9_814a_cf2485d6e98a);
pub const NMI_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe71254e7_c1b9_4940_ab76_909703a4320f);
pub const NPEM_CONTROL_INTERFACE_CURRENT_VERSION: u32 = 2u32;
pub const NPEM_CONTROL_INTERFACE_VERSION1: u32 = 1u32;
pub const NPEM_CONTROL_INTERFACE_VERSION2: u32 = 2u32;
pub const NT_PAGING_LEVELS: u32 = 2u32;
pub const NX_SUPPORT_POLICY_ALWAYSOFF: u32 = 0u32;
pub const NX_SUPPORT_POLICY_ALWAYSON: u32 = 1u32;
pub const NX_SUPPORT_POLICY_OPTIN: u32 = 2u32;
pub const NX_SUPPORT_POLICY_OPTOUT: u32 = 3u32;
pub const NetworkController: CONFIGURATION_TYPE = 18i32;
pub const NetworkPeripheral: CONFIGURATION_TYPE = 36i32;
pub const NoAspmSupport: PCI_EXPRESS_ASPM_SUPPORT = 0i32;
pub const NormalPagePriority: MM_PAGE_PRIORITY = 16i32;
pub const NormalPoolPriority: EX_POOL_PRIORITY = 16i32;
pub const NormalPoolPrioritySpecialPoolOverrun: EX_POOL_PRIORITY = 24i32;
pub const NormalPoolPrioritySpecialPoolUnderrun: EX_POOL_PRIORITY = 25i32;
pub const NormalWorkQueue: WORK_QUEUE_TYPE = 3i32;
pub const NuBus: INTERFACE_TYPE = 7i32;
pub const NuBusConfiguration: BUS_DATA_TYPE = 6i32;
pub const OBJECT_TYPE_CREATE: u32 = 1u32;
pub const OB_FLT_REGISTRATION_VERSION: u32 = 256u32;
pub const OB_FLT_REGISTRATION_VERSION_0100: u32 = 256u32;
pub const OB_OPERATION_HANDLE_CREATE: u32 = 1u32;
pub const OB_OPERATION_HANDLE_DUPLICATE: u32 = 2u32;
pub const OB_PREOP_SUCCESS: OB_PREOP_CALLBACK_STATUS = 0i32;
pub const OPLOCK_KEY_FLAG_PARENT_KEY: u32 = 1u32;
pub const OPLOCK_KEY_FLAG_TARGET_KEY: u32 = 2u32;
pub const OPLOCK_KEY_VERSION_WIN7: u32 = 1u32;
pub const OPLOCK_KEY_VERSION_WIN8: u32 = 2u32;
pub const OSC_CAPABILITIES_MASKED: u32 = 16u32;
pub const OSC_FIRMWARE_FAILURE: u32 = 2u32;
pub const OSC_UNRECOGNIZED_REVISION: u32 = 8u32;
pub const OSC_UNRECOGNIZED_UUID: u32 = 4u32;
pub const OkControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 2i32;
pub const OtherController: CONFIGURATION_TYPE = 24i32;
pub const OtherPeripheral: CONFIGURATION_TYPE = 34i32;
pub const PAGE_ENCLAVE_NO_CHANGE: u32 = 536870912u32;
pub const PAGE_ENCLAVE_THREAD_CONTROL: u32 = 2147483648u32;
pub const PAGE_ENCLAVE_UNVALIDATED: u32 = 536870912u32;
pub const PAGE_EXECUTE: u32 = 16u32;
pub const PAGE_EXECUTE_READ: u32 = 32u32;
pub const PAGE_EXECUTE_READWRITE: u32 = 64u32;
pub const PAGE_EXECUTE_WRITECOPY: u32 = 128u32;
pub const PAGE_GRAPHICS_COHERENT: u32 = 131072u32;
pub const PAGE_GRAPHICS_EXECUTE: u32 = 16384u32;
pub const PAGE_GRAPHICS_EXECUTE_READ: u32 = 32768u32;
pub const PAGE_GRAPHICS_EXECUTE_READWRITE: u32 = 65536u32;
pub const PAGE_GRAPHICS_NOACCESS: u32 = 2048u32;
pub const PAGE_GRAPHICS_NOCACHE: u32 = 262144u32;
pub const PAGE_GRAPHICS_READONLY: u32 = 4096u32;
pub const PAGE_GRAPHICS_READWRITE: u32 = 8192u32;
pub const PAGE_GUARD: u32 = 256u32;
pub const PAGE_NOACCESS: u32 = 1u32;
pub const PAGE_NOCACHE: u32 = 512u32;
pub const PAGE_READONLY: u32 = 2u32;
pub const PAGE_READWRITE: u32 = 4u32;
pub const PAGE_REVERT_TO_FILE_MAP: u32 = 2147483648u32;
pub const PAGE_SHIFT: i32 = 12i32;
pub const PAGE_SIZE: u32 = 4096u32;
pub const PAGE_TARGETS_INVALID: u32 = 1073741824u32;
pub const PAGE_TARGETS_NO_UPDATE: u32 = 1073741824u32;
pub const PAGE_WRITECOMBINE: u32 = 1024u32;
pub const PAGE_WRITECOPY: u32 = 8u32;
pub const PARKING_TOPOLOGY_POLICY_DISABLED: u32 = 0u32;
pub const PASSIVE_LEVEL: u32 = 0u32;
pub const PCCARD_DEVICE_PCI: u32 = 16u32;
pub const PCCARD_DUP_LEGACY_BASE: u32 = 6u32;
pub const PCCARD_MAP_ERROR: u32 = 1u32;
pub const PCCARD_MAP_ZERO: u32 = 2u32;
pub const PCCARD_NO_CONTROLLERS: u32 = 7u32;
pub const PCCARD_NO_LEGACY_BASE: u32 = 5u32;
pub const PCCARD_NO_PIC: u32 = 4u32;
pub const PCCARD_NO_TIMER: u32 = 3u32;
pub const PCCARD_SCAN_DISABLED: u32 = 1u32;
pub const PCIBus: INTERFACE_TYPE = 5i32;
pub const PCIConfiguration: BUS_DATA_TYPE = 4i32;
pub const PCIEXPRESS_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd995e954_bbc1_430f_ad91_b44dcb3c6f35);
pub const PCIE_CORRECTABLE_ERROR_SUMMARY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe96eca99_53e2_4f52_9be7_d2dbe9508ed0);
pub const PCIXBUS_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc5753963_3b84_4095_bf78_eddad3f9c9dd);
pub const PCIXBUS_ERRTYPE_ADDRESSPARITY: u32 = 6u32;
pub const PCIXBUS_ERRTYPE_BUSTIMEOUT: u32 = 4u32;
pub const PCIXBUS_ERRTYPE_COMMANDPARITY: u32 = 7u32;
pub const PCIXBUS_ERRTYPE_DATAPARITY: u32 = 1u32;
pub const PCIXBUS_ERRTYPE_MASTERABORT: u32 = 3u32;
pub const PCIXBUS_ERRTYPE_MASTERDATAPARITY: u32 = 5u32;
pub const PCIXBUS_ERRTYPE_SYSTEM: u32 = 2u32;
pub const PCIXBUS_ERRTYPE_UNKNOWN: u32 = 0u32;
pub const PCIXDEVICE_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeb5e4685_ca66_4769_b6a2_26068b001326);
pub const PCIX_MODE1_100MHZ: u32 = 2u32;
pub const PCIX_MODE1_133MHZ: u32 = 3u32;
pub const PCIX_MODE1_66MHZ: u32 = 1u32;
pub const PCIX_MODE2_266_100MHZ: u32 = 10u32;
pub const PCIX_MODE2_266_133MHZ: u32 = 11u32;
pub const PCIX_MODE2_266_66MHZ: u32 = 9u32;
pub const PCIX_MODE2_533_100MHZ: u32 = 14u32;
pub const PCIX_MODE2_533_133MHZ: u32 = 15u32;
pub const PCIX_MODE2_533_66MHZ: u32 = 13u32;
pub const PCIX_MODE_CONVENTIONAL_PCI: u32 = 0u32;
pub const PCIX_VERSION_DUAL_MODE_ECC: u32 = 2u32;
pub const PCIX_VERSION_MODE1_ONLY: u32 = 0u32;
pub const PCIX_VERSION_MODE2_ECC: u32 = 1u32;
pub const PCI_ACS_ALLOWED: u32 = 0u32;
pub const PCI_ACS_BLOCKED: u32 = 1u32;
pub const PCI_ACS_REDIRECTED: u32 = 2u32;
pub const PCI_ADDRESS_IO_ADDRESS_MASK: u32 = 4294967292u32;
pub const PCI_ADDRESS_IO_SPACE: u32 = 1u32;
pub const PCI_ADDRESS_MEMORY_ADDRESS_MASK: u32 = 4294967280u32;
pub const PCI_ADDRESS_MEMORY_PREFETCHABLE: u32 = 8u32;
pub const PCI_ADDRESS_MEMORY_TYPE_MASK: u32 = 6u32;
pub const PCI_ADDRESS_ROM_ADDRESS_MASK: u32 = 4294965248u32;
pub const PCI_AGP_RATE_1X: u32 = 1u32;
pub const PCI_AGP_RATE_2X: u32 = 2u32;
pub const PCI_AGP_RATE_4X: u32 = 4u32;
pub const PCI_ATS_INTERFACE_VERSION: u32 = 1u32;
pub const PCI_BRIDGE_TYPE: u32 = 1u32;
pub const PCI_BUS_INTERFACE_STANDARD_VERSION: u32 = 2u32;
pub const PCI_CAPABILITY_ID_ADVANCED_FEATURES: u32 = 19u32;
pub const PCI_CAPABILITY_ID_AGP: u32 = 2u32;
pub const PCI_CAPABILITY_ID_AGP_TARGET: u32 = 14u32;
pub const PCI_CAPABILITY_ID_CPCI_HOTSWAP: u32 = 6u32;
pub const PCI_CAPABILITY_ID_CPCI_RES_CTRL: u32 = 11u32;
pub const PCI_CAPABILITY_ID_DEBUG_PORT: u32 = 10u32;
pub const PCI_CAPABILITY_ID_FPB: u32 = 21u32;
pub const PCI_CAPABILITY_ID_HYPERTRANSPORT: u32 = 8u32;
pub const PCI_CAPABILITY_ID_MSI: u32 = 5u32;
pub const PCI_CAPABILITY_ID_MSIX: u32 = 17u32;
pub const PCI_CAPABILITY_ID_P2P_SSID: u32 = 13u32;
pub const PCI_CAPABILITY_ID_PCIX: u32 = 7u32;
pub const PCI_CAPABILITY_ID_PCI_EXPRESS: u32 = 16u32;
pub const PCI_CAPABILITY_ID_POWER_MANAGEMENT: u32 = 1u32;
pub const PCI_CAPABILITY_ID_SATA_CONFIG: u32 = 18u32;
pub const PCI_CAPABILITY_ID_SECURE: u32 = 15u32;
pub const PCI_CAPABILITY_ID_SHPC: u32 = 12u32;
pub const PCI_CAPABILITY_ID_SLOT_ID: u32 = 4u32;
pub const PCI_CAPABILITY_ID_VENDOR_SPECIFIC: u32 = 9u32;
pub const PCI_CAPABILITY_ID_VPD: u32 = 3u32;
pub const PCI_CARDBUS_BRIDGE_TYPE: u32 = 2u32;
pub const PCI_CLASS_BASE_SYSTEM_DEV: u32 = 8u32;
pub const PCI_CLASS_BRIDGE_DEV: u32 = 6u32;
pub const PCI_CLASS_DATA_ACQ_SIGNAL_PROC: u32 = 17u32;
pub const PCI_CLASS_DISPLAY_CTLR: u32 = 3u32;
pub const PCI_CLASS_DOCKING_STATION: u32 = 10u32;
pub const PCI_CLASS_ENCRYPTION_DECRYPTION: u32 = 16u32;
pub const PCI_CLASS_INPUT_DEV: u32 = 9u32;
pub const PCI_CLASS_INTELLIGENT_IO_CTLR: u32 = 14u32;
pub const PCI_CLASS_MASS_STORAGE_CTLR: u32 = 1u32;
pub const PCI_CLASS_MEMORY_CTLR: u32 = 5u32;
pub const PCI_CLASS_MULTIMEDIA_DEV: u32 = 4u32;
pub const PCI_CLASS_NETWORK_CTLR: u32 = 2u32;
pub const PCI_CLASS_NOT_DEFINED: u32 = 255u32;
pub const PCI_CLASS_PRE_20: u32 = 0u32;
pub const PCI_CLASS_PROCESSOR: u32 = 11u32;
pub const PCI_CLASS_SATELLITE_COMMS_CTLR: u32 = 15u32;
pub const PCI_CLASS_SERIAL_BUS_CTLR: u32 = 12u32;
pub const PCI_CLASS_SIMPLE_COMMS_CTLR: u32 = 7u32;
pub const PCI_CLASS_WIRELESS_CTLR: u32 = 13u32;
pub const PCI_DATA_VERSION: u32 = 1u32;
pub const PCI_DEVICE_PRESENT_INTERFACE_VERSION: u32 = 1u32;
pub const PCI_DEVICE_TYPE: u32 = 0u32;
pub const PCI_DISABLE_LEVEL_INTERRUPT: u32 = 1024u32;
pub const PCI_ENABLE_BUS_MASTER: u32 = 4u32;
pub const PCI_ENABLE_FAST_BACK_TO_BACK: u32 = 512u32;
pub const PCI_ENABLE_IO_SPACE: u32 = 1u32;
pub const PCI_ENABLE_MEMORY_SPACE: u32 = 2u32;
pub const PCI_ENABLE_PARITY: u32 = 64u32;
pub const PCI_ENABLE_SERR: u32 = 256u32;
pub const PCI_ENABLE_SPECIAL_CYCLES: u32 = 8u32;
pub const PCI_ENABLE_VGA_COMPATIBLE_PALETTE: u32 = 32u32;
pub const PCI_ENABLE_WAIT_CYCLE: u32 = 128u32;
pub const PCI_ENABLE_WRITE_AND_INVALIDATE: u32 = 16u32;
pub const PCI_EXPRESS_ACCESS_CONTROL_SERVICES_CAP_ID: u32 = 13u32;
pub const PCI_EXPRESS_ADVANCED_ERROR_REPORTING_CAP_ID: u32 = 1u32;
pub const PCI_EXPRESS_ARI_CAP_ID: u32 = 14u32;
pub const PCI_EXPRESS_ATS_CAP_ID: u32 = 15u32;
pub const PCI_EXPRESS_CONFIGURATION_ACCESS_CORRELATION_CAP_ID: u32 = 12u32;
pub const PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_CAP_ID: u32 = 35u32;
pub const PCI_EXPRESS_DEVICE_SERIAL_NUMBER_CAP_ID: u32 = 3u32;
pub const PCI_EXPRESS_DPA_CAP_ID: u32 = 22u32;
pub const PCI_EXPRESS_DPC_CAP_ID: u32 = 29u32;
pub const PCI_EXPRESS_FRS_QUEUEING_CAP_ID: u32 = 33u32;
pub const PCI_EXPRESS_L1_PM_SS_CAP_ID: u32 = 30u32;
pub const PCI_EXPRESS_LINK_QUIESCENT_INTERFACE_VERSION: u32 = 1u32;
pub const PCI_EXPRESS_LN_REQUESTER_CAP_ID: u32 = 28u32;
pub const PCI_EXPRESS_LTR_CAP_ID: u32 = 24u32;
pub const PCI_EXPRESS_MFVC_CAP_ID: u32 = 8u32;
pub const PCI_EXPRESS_MPCIE_CAP_ID: u32 = 32u32;
pub const PCI_EXPRESS_MULTICAST_CAP_ID: u32 = 18u32;
pub const PCI_EXPRESS_MULTI_ROOT_IO_VIRTUALIZATION_CAP_ID: u32 = 17u32;
pub const PCI_EXPRESS_NPEM_CAP_ID: u32 = 41u32;
pub const PCI_EXPRESS_PAGE_REQUEST_CAP_ID: u32 = 19u32;
pub const PCI_EXPRESS_PASID_CAP_ID: u32 = 27u32;
pub const PCI_EXPRESS_PMUX_CAP_ID: u32 = 26u32;
pub const PCI_EXPRESS_POWER_BUDGETING_CAP_ID: u32 = 4u32;
pub const PCI_EXPRESS_PTM_CAP_ID: u32 = 31u32;
pub const PCI_EXPRESS_RCRB_HEADER_CAP_ID: u32 = 10u32;
pub const PCI_EXPRESS_RC_EVENT_COLLECTOR_ENDPOINT_ASSOCIATION_CAP_ID: u32 = 7u32;
pub const PCI_EXPRESS_RC_INTERNAL_LINK_CONTROL_CAP_ID: u32 = 6u32;
pub const PCI_EXPRESS_RC_LINK_DECLARATION_CAP_ID: u32 = 5u32;
pub const PCI_EXPRESS_READINESS_TIME_REPORTING_CAP_ID: u32 = 34u32;
pub const PCI_EXPRESS_RESERVED_FOR_AMD_CAP_ID: u32 = 20u32;
pub const PCI_EXPRESS_RESIZABLE_BAR_CAP_ID: u32 = 21u32;
pub const PCI_EXPRESS_ROOT_PORT_INTERFACE_VERSION: u32 = 1u32;
pub const PCI_EXPRESS_SECONDARY_PCI_EXPRESS_CAP_ID: u32 = 25u32;
pub const PCI_EXPRESS_SINGLE_ROOT_IO_VIRTUALIZATION_CAP_ID: u32 = 16u32;
pub const PCI_EXPRESS_TPH_REQUESTER_CAP_ID: u32 = 23u32;
pub const PCI_EXPRESS_TPH_ST_LOCATION_MSIX_TABLE: u32 = 2u32;
pub const PCI_EXPRESS_TPH_ST_LOCATION_NONE: u32 = 0u32;
pub const PCI_EXPRESS_TPH_ST_LOCATION_RESERVED: u32 = 3u32;
pub const PCI_EXPRESS_TPH_ST_LOCATION_TPH_CAPABILITY: u32 = 1u32;
pub const PCI_EXPRESS_VC_AND_MFVC_CAP_ID: u32 = 9u32;
pub const PCI_EXPRESS_VENDOR_SPECIFIC_CAP_ID: u32 = 11u32;
pub const PCI_EXPRESS_VIRTUAL_CHANNEL_CAP_ID: u32 = 2u32;
pub const PCI_EXTENDED_CONFIG_LENGTH: u32 = 4096u32;
pub const PCI_INVALID_ALTERNATE_FUNCTION_NUMBER: u32 = 255u32;
pub const PCI_INVALID_VENDORID: u32 = 65535u32;
pub const PCI_MAX_BRIDGE_NUMBER: u32 = 255u32;
pub const PCI_MAX_DEVICES: u32 = 32u32;
pub const PCI_MAX_FUNCTION: u32 = 8u32;
pub const PCI_MAX_SEGMENT_NUMBER: u32 = 65535u32;
pub const PCI_MSIX_TABLE_CONFIG_INTERFACE_VERSION: u32 = 1u32;
pub const PCI_MULTIFUNCTION: u32 = 128u32;
pub const PCI_PROGRAMMING_INTERFACE_MSC_NVM_EXPRESS: u32 = 2u32;
pub const PCI_PTM_TIME_SOURCE_AUX: u32 = 4294967295u32;
pub const PCI_RECOVERY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdd060800_f6e1_4204_ac27_c4bca9568402);
pub const PCI_ROMADDRESS_ENABLED: u32 = 1u32;
pub const PCI_ROOT_BUS_OSC_METHOD_CAPABILITY_REVISION: u32 = 1u32;
pub const PCI_SECURITY_DIRECT_TRANSLATED_P2P: u32 = 4u32;
pub const PCI_SECURITY_ENHANCED: u32 = 2u32;
pub const PCI_SECURITY_FULLY_SUPPORTED: u32 = 1u32;
pub const PCI_SECURITY_GUEST_ASSIGNED: u32 = 1u32;
pub const PCI_SECURITY_INTERFACE_VERSION: u32 = 1u32;
pub const PCI_SECURITY_INTERFACE_VERSION2: u32 = 2u32;
pub const PCI_SECURITY_SRIOV_DIRECT_TRANSLATED_P2P: u32 = 262144u32;
pub const PCI_STATUS_66MHZ_CAPABLE: u32 = 32u32;
pub const PCI_STATUS_CAPABILITIES_LIST: u32 = 16u32;
pub const PCI_STATUS_DATA_PARITY_DETECTED: u32 = 256u32;
pub const PCI_STATUS_DETECTED_PARITY_ERROR: u32 = 32768u32;
pub const PCI_STATUS_DEVSEL: u32 = 1536u32;
pub const PCI_STATUS_FAST_BACK_TO_BACK: u32 = 128u32;
pub const PCI_STATUS_IMMEDIATE_READINESS: u32 = 1u32;
pub const PCI_STATUS_INTERRUPT_PENDING: u32 = 8u32;
pub const PCI_STATUS_RECEIVED_MASTER_ABORT: u32 = 8192u32;
pub const PCI_STATUS_RECEIVED_TARGET_ABORT: u32 = 4096u32;
pub const PCI_STATUS_SIGNALED_SYSTEM_ERROR: u32 = 16384u32;
pub const PCI_STATUS_SIGNALED_TARGET_ABORT: u32 = 2048u32;
pub const PCI_STATUS_UDF_SUPPORTED: u32 = 64u32;
pub const PCI_SUBCLASS_BR_CARDBUS: u32 = 7u32;
pub const PCI_SUBCLASS_BR_EISA: u32 = 2u32;
pub const PCI_SUBCLASS_BR_HOST: u32 = 0u32;
pub const PCI_SUBCLASS_BR_ISA: u32 = 1u32;
pub const PCI_SUBCLASS_BR_MCA: u32 = 3u32;
pub const PCI_SUBCLASS_BR_NUBUS: u32 = 6u32;
pub const PCI_SUBCLASS_BR_OTHER: u32 = 128u32;
pub const PCI_SUBCLASS_BR_PCI_TO_PCI: u32 = 4u32;
pub const PCI_SUBCLASS_BR_PCMCIA: u32 = 5u32;
pub const PCI_SUBCLASS_BR_RACEWAY: u32 = 8u32;
pub const PCI_SUBCLASS_COM_MODEM: u32 = 3u32;
pub const PCI_SUBCLASS_COM_MULTIPORT: u32 = 2u32;
pub const PCI_SUBCLASS_COM_OTHER: u32 = 128u32;
pub const PCI_SUBCLASS_COM_PARALLEL: u32 = 1u32;
pub const PCI_SUBCLASS_COM_SERIAL: u32 = 0u32;
pub const PCI_SUBCLASS_CRYPTO_ENTERTAINMENT: u32 = 16u32;
pub const PCI_SUBCLASS_CRYPTO_NET_COMP: u32 = 0u32;
pub const PCI_SUBCLASS_CRYPTO_OTHER: u32 = 128u32;
pub const PCI_SUBCLASS_DASP_DPIO: u32 = 0u32;
pub const PCI_SUBCLASS_DASP_OTHER: u32 = 128u32;
pub const PCI_SUBCLASS_DOC_GENERIC: u32 = 0u32;
pub const PCI_SUBCLASS_DOC_OTHER: u32 = 128u32;
pub const PCI_SUBCLASS_INP_DIGITIZER: u32 = 1u32;
pub const PCI_SUBCLASS_INP_GAMEPORT: u32 = 4u32;
pub const PCI_SUBCLASS_INP_KEYBOARD: u32 = 0u32;
pub const PCI_SUBCLASS_INP_MOUSE: u32 = 2u32;
pub const PCI_SUBCLASS_INP_OTHER: u32 = 128u32;
pub const PCI_SUBCLASS_INP_SCANNER: u32 = 3u32;
pub const PCI_SUBCLASS_INTIO_I2O: u32 = 0u32;
pub const PCI_SUBCLASS_MEM_FLASH: u32 = 1u32;
pub const PCI_SUBCLASS_MEM_OTHER: u32 = 128u32;
pub const PCI_SUBCLASS_MEM_RAM: u32 = 0u32;
pub const PCI_SUBCLASS_MM_AUDIO_DEV: u32 = 1u32;
pub const PCI_SUBCLASS_MM_OTHER: u32 = 128u32;
pub const PCI_SUBCLASS_MM_TELEPHONY_DEV: u32 = 2u32;
pub const PCI_SUBCLASS_MM_VIDEO_DEV: u32 = 0u32;
pub const PCI_SUBCLASS_MSC_AHCI_CTLR: u32 = 6u32;
pub const PCI_SUBCLASS_MSC_FLOPPY_CTLR: u32 = 2u32;
pub const PCI_SUBCLASS_MSC_IDE_CTLR: u32 = 1u32;
pub const PCI_SUBCLASS_MSC_IPI_CTLR: u32 = 3u32;
pub const PCI_SUBCLASS_MSC_NVM_CTLR: u32 = 8u32;
pub const PCI_SUBCLASS_MSC_OTHER: u32 = 128u32;
pub const PCI_SUBCLASS_MSC_RAID_CTLR: u32 = 4u32;
pub const PCI_SUBCLASS_MSC_SCSI_BUS_CTLR: u32 = 0u32;
pub const PCI_SUBCLASS_NET_ATM_CTLR: u32 = 3u32;
pub const PCI_SUBCLASS_NET_ETHERNET_CTLR: u32 = 0u32;
pub const PCI_SUBCLASS_NET_FDDI_CTLR: u32 = 2u32;
pub const PCI_SUBCLASS_NET_ISDN_CTLR: u32 = 4u32;
pub const PCI_SUBCLASS_NET_OTHER: u32 = 128u32;
pub const PCI_SUBCLASS_NET_TOKEN_RING_CTLR: u32 = 1u32;
pub const PCI_SUBCLASS_PRE_20_NON_VGA: u32 = 0u32;
pub const PCI_SUBCLASS_PRE_20_VGA: u32 = 1u32;
pub const PCI_SUBCLASS_PROC_386: u32 = 0u32;
pub const PCI_SUBCLASS_PROC_486: u32 = 1u32;
pub const PCI_SUBCLASS_PROC_ALPHA: u32 = 16u32;
pub const PCI_SUBCLASS_PROC_COPROCESSOR: u32 = 64u32;
pub const PCI_SUBCLASS_PROC_PENTIUM: u32 = 2u32;
pub const PCI_SUBCLASS_PROC_POWERPC: u32 = 32u32;
pub const PCI_SUBCLASS_SAT_AUDIO: u32 = 2u32;
pub const PCI_SUBCLASS_SAT_DATA: u32 = 4u32;
pub const PCI_SUBCLASS_SAT_TV: u32 = 1u32;
pub const PCI_SUBCLASS_SAT_VOICE: u32 = 3u32;
pub const PCI_SUBCLASS_SB_ACCESS: u32 = 1u32;
pub const PCI_SUBCLASS_SB_FIBRE_CHANNEL: u32 = 4u32;
pub const PCI_SUBCLASS_SB_IEEE1394: u32 = 0u32;
pub const PCI_SUBCLASS_SB_SMBUS: u32 = 5u32;
pub const PCI_SUBCLASS_SB_SSA: u32 = 2u32;
pub const PCI_SUBCLASS_SB_THUNDERBOLT: u32 = 10u32;
pub const PCI_SUBCLASS_SB_USB: u32 = 3u32;
pub const PCI_SUBCLASS_SYS_DMA_CTLR: u32 = 1u32;
pub const PCI_SUBCLASS_SYS_GEN_HOTPLUG_CTLR: u32 = 4u32;
pub const PCI_SUBCLASS_SYS_INTERRUPT_CTLR: u32 = 0u32;
pub const PCI_SUBCLASS_SYS_OTHER: u32 = 128u32;
pub const PCI_SUBCLASS_SYS_RCEC: u32 = 7u32;
pub const PCI_SUBCLASS_SYS_REAL_TIME_CLOCK: u32 = 3u32;
pub const PCI_SUBCLASS_SYS_SDIO_CTRL: u32 = 5u32;
pub const PCI_SUBCLASS_SYS_SYSTEM_TIMER: u32 = 2u32;
pub const PCI_SUBCLASS_VID_OTHER: u32 = 128u32;
pub const PCI_SUBCLASS_VID_VGA_CTLR: u32 = 0u32;
pub const PCI_SUBCLASS_VID_XGA_CTLR: u32 = 1u32;
pub const PCI_SUBCLASS_WIRELESS_CON_IR: u32 = 1u32;
pub const PCI_SUBCLASS_WIRELESS_IRDA: u32 = 0u32;
pub const PCI_SUBCLASS_WIRELESS_OTHER: u32 = 128u32;
pub const PCI_SUBCLASS_WIRELESS_RF: u32 = 16u32;
pub const PCI_SUBLCASS_VID_3D_CTLR: u32 = 2u32;
pub const PCI_TYPE0_ADDRESSES: u32 = 6u32;
pub const PCI_TYPE1_ADDRESSES: u32 = 2u32;
pub const PCI_TYPE2_ADDRESSES: u32 = 5u32;
pub const PCI_TYPE_20BIT: u32 = 2u32;
pub const PCI_TYPE_32BIT: u32 = 0u32;
pub const PCI_TYPE_64BIT: u32 = 4u32;
pub const PCI_USE_CLASS_SUBCLASS: u32 = 8u32;
pub const PCI_USE_LOCAL_BUS: u32 = 32u32;
pub const PCI_USE_LOCAL_DEVICE: u32 = 64u32;
pub const PCI_USE_PROGIF: u32 = 16u32;
pub const PCI_USE_REVISION: u32 = 2u32;
pub const PCI_USE_SUBSYSTEM_IDS: u32 = 1u32;
pub const PCI_USE_VENDEV_IDS: u32 = 4u32;
pub const PCI_WHICHSPACE_CONFIG: u32 = 0u32;
pub const PCI_WHICHSPACE_ROM: u32 = 1382638416u32;
pub const PCIe_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcf93c01f_1a16_4dfc_b8bc_9c4daf67c104);
pub const PCMCIABus: INTERFACE_TYPE = 8i32;
pub const PCMCIAConfiguration: BUS_DATA_TYPE = 7i32;
pub const PCR_BTI_MITIGATION_CSWAP_HVC: u32 = 16u32;
pub const PCR_BTI_MITIGATION_CSWAP_SMC: u32 = 32u32;
pub const PCR_BTI_MITIGATION_NONE: u32 = 0u32;
pub const PCR_BTI_MITIGATION_VBAR_MASK: u32 = 15u32;
pub const PCR_MAJOR_VERSION: u32 = 1u32;
pub const PCR_MINOR_VERSION: u32 = 1u32;
pub const PCW_CURRENT_VERSION: u32 = 512u32;
pub const PCW_VERSION_1: u32 = 256u32;
pub const PCW_VERSION_2: u32 = 512u32;
pub const PDE_BASE: u32 = 3224371200u32;
pub const PDE_PER_PAGE: u32 = 512u32;
pub const PDE_TOP: u32 = 3224375295u32;
pub const PDI_SHIFT: u32 = 21u32;
pub const PEI_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x09a9d5ac_5204_4214_96e5_94992e752bcd);
pub const PFAControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 6i32;
pub const PLUGPLAY_PROPERTY_PERSISTENT: u32 = 1u32;
pub const PLUGPLAY_REGKEY_CURRENT_HWPROFILE: u32 = 4u32;
pub const PLUGPLAY_REGKEY_DEVICE: u32 = 1u32;
pub const PLUGPLAY_REGKEY_DRIVER: u32 = 2u32;
pub const PMCCounter: HARDWARE_COUNTER_TYPE = 0i32;
pub const PMEM_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x81687003_dbfd_4728_9ffd_f0904f97597d);
pub const PNPBus: INTERFACE_TYPE = 15i32;
pub const PNPISABus: INTERFACE_TYPE = 14i32;
pub const PNPISAConfiguration: BUS_DATA_TYPE = 10i32;
pub const PNPNOTIFY_DEVICE_INTERFACE_INCLUDE_EXISTING_INTERFACES: u32 = 1u32;
pub const PNP_DEVICE_ASSIGNED_TO_GUEST: u32 = 256u32;
pub const PNP_DEVICE_DISABLED: u32 = 1u32;
pub const PNP_DEVICE_DISCONNECTED: u32 = 64u32;
pub const PNP_DEVICE_DONT_DISPLAY_IN_UI: u32 = 2u32;
pub const PNP_DEVICE_FAILED: u32 = 4u32;
pub const PNP_DEVICE_NOT_DISABLEABLE: u32 = 32u32;
pub const PNP_DEVICE_REMOVED: u32 = 8u32;
pub const PNP_DEVICE_RESOURCE_REQUIREMENTS_CHANGED: u32 = 16u32;
pub const PNP_DEVICE_RESOURCE_UPDATED: u32 = 128u32;
pub const PNP_EXTENDED_ADDRESS_INTERFACE_VERSION: u32 = 1u32;
pub const PNP_REPLACE_DRIVER_INTERFACE_VERSION: u32 = 1u32;
pub const PNP_REPLACE_HARDWARE_MEMORY_MIRRORING: u32 = 4u32;
pub const PNP_REPLACE_HARDWARE_PAGE_COPY: u32 = 8u32;
pub const PNP_REPLACE_HARDWARE_QUIESCE: u32 = 16u32;
pub const PNP_REPLACE_MEMORY_SUPPORTED: u32 = 1u32;
pub const PNP_REPLACE_PARAMETERS_VERSION: u32 = 2u32;
pub const PNP_REPLACE_PROCESSOR_SUPPORTED: u32 = 2u32;
pub const POOL_COLD_ALLOCATION: u32 = 256u32;
pub const POOL_CREATE_FLG_SECURE_POOL: u32 = 1u32;
pub const POOL_CREATE_FLG_USE_GLOBAL_POOL: u32 = 2u32;
pub const POOL_CREATE_PARAMS_VERSION: u32 = 1u32;
pub const POOL_EXTENDED_PARAMETER_REQUIRED_FIELD_BITS: u32 = 1u32;
pub const POOL_EXTENDED_PARAMETER_TYPE_BITS: u32 = 8u32;
pub const POOL_NX_ALLOCATION: u32 = 512u32;
pub const POOL_NX_OPTIN_AUTO: u32 = 1u32;
pub const POOL_QUOTA_FAIL_INSTEAD_OF_RAISE: u32 = 8u32;
pub const POOL_RAISE_IF_ALLOCATION_FAILURE: u32 = 16u32;
pub const POOL_TAGGING: u32 = 1u32;
pub const POOL_ZEROING_INFORMATION: u32 = 227u32;
pub const POOL_ZERO_ALLOCATION: u32 = 1024u32;
pub const PORT_MAXIMUM_MESSAGE_LENGTH: u32 = 512u32;
pub const POWER_LEVEL: u32 = 30u32;
pub const POWER_PLATFORM_ROLE_V1: u32 = 1u32;
pub const POWER_PLATFORM_ROLE_V2: u32 = 2u32;
pub const POWER_PLATFORM_ROLE_VERSION: u32 = 2u32;
pub const POWER_SETTING_VALUE_VERSION: u32 = 1u32;
pub const POWER_THROTTLING_PROCESS_CURRENT_VERSION: u32 = 1u32;
pub const POWER_THROTTLING_PROCESS_DELAYTIMERS: u32 = 2u32;
pub const POWER_THROTTLING_PROCESS_EXECUTION_SPEED: u32 = 1u32;
pub const POWER_THROTTLING_PROCESS_IGNORE_TIMER_RESOLUTION: u32 = 4u32;
pub const POWER_THROTTLING_THREAD_CURRENT_VERSION: u32 = 1u32;
pub const POWER_THROTTLING_THREAD_EXECUTION_SPEED: u32 = 1u32;
pub const POWER_THROTTLING_THREAD_VALID_FLAGS: u32 = 1u32;
pub const PO_FX_COMPONENT_FLAG_F0_ON_DX: u64 = 1u64;
pub const PO_FX_COMPONENT_FLAG_NO_DEBOUNCE: u64 = 2u64;
pub const PO_FX_DIRECTED_FX_DEFAULT_IDLE_TIMEOUT: u32 = 0u32;
pub const PO_FX_FLAG_ASYNC_ONLY: u32 = 2u32;
pub const PO_FX_FLAG_BLOCKING: u32 = 1u32;
pub const PO_FX_FLAG_PERF_PEP_OPTIONAL: u32 = 1u32;
pub const PO_FX_FLAG_PERF_QUERY_ON_ALL_IDLE_STATES: u32 = 4u32;
pub const PO_FX_FLAG_PERF_QUERY_ON_F0: u32 = 2u32;
pub const PO_FX_UNKNOWN_POWER: u32 = 4294967295u32;
pub const PO_FX_UNKNOWN_TIME: u64 = 18446744073709551615u64;
pub const PO_FX_VERSION: u32 = 1u32;
pub const PO_FX_VERSION_V1: u32 = 1u32;
pub const PO_FX_VERSION_V2: u32 = 2u32;
pub const PO_FX_VERSION_V3: u32 = 3u32;
pub const PO_MEM_BOOT_PHASE: u32 = 65536u32;
pub const PO_MEM_CLONE: u32 = 2u32;
pub const PO_MEM_CL_OR_NCHK: u32 = 4u32;
pub const PO_MEM_DISCARD: u32 = 32768u32;
pub const PO_MEM_PAGE_ADDRESS: u32 = 16384u32;
pub const PO_MEM_PRESERVE: u32 = 1u32;
pub const PPI_SHIFT: u32 = 30u32;
pub const PRIVILEGE_SET_ALL_NECESSARY: u32 = 1u32;
pub const PROCESSOR_FEATURE_MAX: u32 = 64u32;
pub const PROCESSOR_GENERIC_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9876ccad_47b4_4bdb_b65e_16f193c4f3db);
pub const PROCESS_EXCEPTION_PORT_ALL_STATE_BITS: u32 = 3u32;
pub const PROCESS_HANDLE_EXCEPTIONS_ENABLED: u32 = 1u32;
pub const PROCESS_HANDLE_RAISE_UM_EXCEPTION_ON_INVALID_HANDLE_CLOSE_DISABLED: u32 = 0u32;
pub const PROCESS_HANDLE_RAISE_UM_EXCEPTION_ON_INVALID_HANDLE_CLOSE_ENABLED: u32 = 1u32;
pub const PROCESS_HANDLE_TRACING_MAX_STACKS: u32 = 16u32;
pub const PROCESS_LUID_DOSDEVICES_ONLY: u32 = 1u32;
pub const PROFILE_LEVEL: u32 = 27u32;
pub const PROTECTED_POOL: u32 = 0u32;
pub const PS_IMAGE_NOTIFY_CONFLICTING_ARCHITECTURE: u32 = 1u32;
pub const PS_INVALID_SILO_CONTEXT_SLOT: u32 = 4294967295u32;
pub const PTE_BASE: u32 = 3221225472u32;
pub const PTE_PER_PAGE: u32 = 512u32;
pub const PTE_TOP: u32 = 3225419775u32;
pub const PTI_SHIFT: u32 = 12u32;
pub const PageIn: KWAIT_REASON = 2i32;
pub const ParallelController: CONFIGURATION_TYPE = 20i32;
pub const PciAcsBitDisable: PCI_ACS_BIT = 2i32;
pub const PciAcsBitDontCare: PCI_ACS_BIT = 3i32;
pub const PciAcsBitEnable: PCI_ACS_BIT = 1i32;
pub const PciAcsReserved: PCI_ACS_BIT = 0i32;
pub const PciAddressParityError: u16 = 6u16;
pub const PciBusDataParityError: u16 = 1u16;
pub const PciBusMasterAbort: u16 = 3u16;
pub const PciBusSystemError: u16 = 2u16;
pub const PciBusTimeOut: u16 = 4u16;
pub const PciBusUnknownError: u16 = 0u16;
pub const PciCommandParityError: u16 = 7u16;
pub const PciConventional: PCI_HARDWARE_INTERFACE = 0i32;
pub const PciDeviceD3Cold_Reason_Default_State_BitIndex: PCI_DEVICE_D3COLD_STATE_REASON = 8i32;
pub const PciDeviceD3Cold_Reason_INF_BitIndex: PCI_DEVICE_D3COLD_STATE_REASON = 9i32;
pub const PciDeviceD3Cold_Reason_Interface_Api_BitIndex: PCI_DEVICE_D3COLD_STATE_REASON = 10i32;
pub const PciDeviceD3Cold_State_Disabled_BitIndex: PCI_DEVICE_D3COLD_STATE_REASON = 1i32;
pub const PciDeviceD3Cold_State_Disabled_Bridge_HackFlags_BitIndex: PCI_DEVICE_D3COLD_STATE_REASON = 4i32;
pub const PciDeviceD3Cold_State_Enabled_BitIndex: PCI_DEVICE_D3COLD_STATE_REASON = 2i32;
pub const PciDeviceD3Cold_State_ParentRootPortS0WakeSupported_BitIndex: PCI_DEVICE_D3COLD_STATE_REASON = 3i32;
pub const PciExpress: PCI_HARDWARE_INTERFACE = 3i32;
pub const PciExpressASPMLinkSubState_L11_BitIndex: PCI_EXPRESS_LINK_SUBSTATE = 2i32;
pub const PciExpressASPMLinkSubState_L12_BitIndex: PCI_EXPRESS_LINK_SUBSTATE = 3i32;
pub const PciExpressDownstreamSwitchPort: PCI_EXPRESS_DEVICE_TYPE = 6i32;
pub const PciExpressEndpoint: PCI_EXPRESS_DEVICE_TYPE = 0i32;
pub const PciExpressLegacyEndpoint: PCI_EXPRESS_DEVICE_TYPE = 1i32;
pub const PciExpressPciPmLinkSubState_L11_BitIndex: PCI_EXPRESS_LINK_SUBSTATE = 0i32;
pub const PciExpressPciPmLinkSubState_L12_BitIndex: PCI_EXPRESS_LINK_SUBSTATE = 1i32;
pub const PciExpressRootComplexEventCollector: PCI_EXPRESS_DEVICE_TYPE = 10i32;
pub const PciExpressRootComplexIntegratedEndpoint: PCI_EXPRESS_DEVICE_TYPE = 9i32;
pub const PciExpressRootPort: PCI_EXPRESS_DEVICE_TYPE = 4i32;
pub const PciExpressToPciXBridge: PCI_EXPRESS_DEVICE_TYPE = 7i32;
pub const PciExpressUpstreamSwitchPort: PCI_EXPRESS_DEVICE_TYPE = 5i32;
pub const PciMasterDataParityError: u16 = 5u16;
pub const PciXMode1: PCI_HARDWARE_INTERFACE = 1i32;
pub const PciXMode2: PCI_HARDWARE_INTERFACE = 2i32;
pub const PciXToExpressBridge: PCI_EXPRESS_DEVICE_TYPE = 8i32;
pub const PcwCallbackAddCounter: PCW_CALLBACK_TYPE = 0i32;
pub const PcwCallbackCollectData: PCW_CALLBACK_TYPE = 3i32;
pub const PcwCallbackEnumerateInstances: PCW_CALLBACK_TYPE = 2i32;
pub const PcwCallbackRemoveCounter: PCW_CALLBACK_TYPE = 1i32;
pub const PcwRegistrationNone: PCW_REGISTRATION_FLAGS = 0i32;
pub const PcwRegistrationSiloNeutral: PCW_REGISTRATION_FLAGS = 1i32;
pub const PermissionFault: FAULT_INFORMATION_ARM64_TYPE = 4i32;
pub const PlatformLevelDeviceReset: DEVICE_RESET_TYPE = 1i32;
pub const PlatformRoleAppliancePC: POWER_PLATFORM_ROLE = 6i32;
pub const PlatformRoleDesktop: POWER_PLATFORM_ROLE = 1i32;
pub const PlatformRoleEnterpriseServer: POWER_PLATFORM_ROLE = 4i32;
pub const PlatformRoleMaximum: POWER_PLATFORM_ROLE = 9i32;
pub const PlatformRoleMobile: POWER_PLATFORM_ROLE = 2i32;
pub const PlatformRolePerformanceServer: POWER_PLATFORM_ROLE = 7i32;
pub const PlatformRoleSOHOServer: POWER_PLATFORM_ROLE = 5i32;
pub const PlatformRoleSlate: POWER_PLATFORM_ROLE = 8i32;
pub const PlatformRoleUnspecified: POWER_PLATFORM_ROLE = 0i32;
pub const PlatformRoleWorkstation: POWER_PLATFORM_ROLE = 3i32;
pub const PoAc: SYSTEM_POWER_CONDITION = 0i32;
pub const PoConditionMaximum: SYSTEM_POWER_CONDITION = 3i32;
pub const PoDc: SYSTEM_POWER_CONDITION = 1i32;
pub const PoFxPerfStateTypeDiscrete: PO_FX_PERF_STATE_TYPE = 0i32;
pub const PoFxPerfStateTypeMaximum: PO_FX_PERF_STATE_TYPE = 2i32;
pub const PoFxPerfStateTypeRange: PO_FX_PERF_STATE_TYPE = 1i32;
pub const PoFxPerfStateUnitBandwidth: PO_FX_PERF_STATE_UNIT = 2i32;
pub const PoFxPerfStateUnitFrequency: PO_FX_PERF_STATE_UNIT = 1i32;
pub const PoFxPerfStateUnitMaximum: PO_FX_PERF_STATE_UNIT = 3i32;
pub const PoFxPerfStateUnitOther: PO_FX_PERF_STATE_UNIT = 0i32;
pub const PoHot: SYSTEM_POWER_CONDITION = 2i32;
pub const PoThermalRequestActive: PO_THERMAL_REQUEST_TYPE = 1i32;
pub const PoThermalRequestPassive: PO_THERMAL_REQUEST_TYPE = 0i32;
pub const PointerController: CONFIGURATION_TYPE = 21i32;
pub const PointerPeripheral: CONFIGURATION_TYPE = 31i32;
pub const PoolAllocation: KWAIT_REASON = 3i32;
pub const PoolExtendedParameterInvalidType: POOL_EXTENDED_PARAMETER_TYPE = 0i32;
pub const PoolExtendedParameterMax: POOL_EXTENDED_PARAMETER_TYPE = 4i32;
pub const PoolExtendedParameterNumaNode: POOL_EXTENDED_PARAMETER_TYPE = 3i32;
pub const PoolExtendedParameterPriority: POOL_EXTENDED_PARAMETER_TYPE = 1i32;
pub const PoolExtendedParameterSecurePool: POOL_EXTENDED_PARAMETER_TYPE = 2i32;
pub const Pos: BUS_DATA_TYPE = 2i32;
pub const PowerOff: PCI_EXPRESS_POWER_STATE = 1i32;
pub const PowerOn: PCI_EXPRESS_POWER_STATE = 0i32;
pub const PowerRelations: DEVICE_RELATION_TYPE = 2i32;
pub const PrimaryDcache: CONFIGURATION_TYPE = 4i32;
pub const PrimaryIcache: CONFIGURATION_TYPE = 3i32;
pub const PrinterPeripheral: CONFIGURATION_TYPE = 30i32;
pub const ProcessorInternal: INTERFACE_TYPE = 12i32;
pub const Profile2Issue: KPROFILE_SOURCE = 15i32;
pub const Profile3Issue: KPROFILE_SOURCE = 16i32;
pub const Profile4Issue: KPROFILE_SOURCE = 17i32;
pub const ProfileAlignmentFixup: KPROFILE_SOURCE = 1i32;
pub const ProfileBranchInstructions: KPROFILE_SOURCE = 6i32;
pub const ProfileBranchMispredictions: KPROFILE_SOURCE = 11i32;
pub const ProfileCacheMisses: KPROFILE_SOURCE = 10i32;
pub const ProfileDcacheAccesses: KPROFILE_SOURCE = 21i32;
pub const ProfileDcacheMisses: KPROFILE_SOURCE = 8i32;
pub const ProfileFpInstructions: KPROFILE_SOURCE = 13i32;
pub const ProfileIcacheIssues: KPROFILE_SOURCE = 20i32;
pub const ProfileIcacheMisses: KPROFILE_SOURCE = 9i32;
pub const ProfileIntegerInstructions: KPROFILE_SOURCE = 14i32;
pub const ProfileLoadInstructions: KPROFILE_SOURCE = 4i32;
pub const ProfileLoadLinkedIssues: KPROFILE_SOURCE = 23i32;
pub const ProfileMaximum: KPROFILE_SOURCE = 24i32;
pub const ProfileMemoryBarrierCycles: KPROFILE_SOURCE = 22i32;
pub const ProfilePipelineDry: KPROFILE_SOURCE = 3i32;
pub const ProfilePipelineFrozen: KPROFILE_SOURCE = 5i32;
pub const ProfileSpecialInstructions: KPROFILE_SOURCE = 18i32;
pub const ProfileStoreInstructions: KPROFILE_SOURCE = 12i32;
pub const ProfileTime: KPROFILE_SOURCE = 0i32;
pub const ProfileTotalCycles: KPROFILE_SOURCE = 19i32;
pub const ProfileTotalIssues: KPROFILE_SOURCE = 2i32;
pub const ProfileTotalNonissues: KPROFILE_SOURCE = 7i32;
pub const PsCreateProcessNotifySubsystems: PSCREATEPROCESSNOTIFYTYPE = 0i32;
pub const PsCreateThreadNotifyNonSystem: PSCREATETHREADNOTIFYTYPE = 0i32;
pub const PsCreateThreadNotifySubsystems: PSCREATETHREADNOTIFYTYPE = 1i32;
pub const PshedFADiscovery: u32 = 1u32;
pub const PshedFAErrorInfoRetrieval: u32 = 8u32;
pub const PshedFAErrorInjection: u32 = 32u32;
pub const PshedFAErrorRecordPersistence: u32 = 4u32;
pub const PshedFAErrorRecovery: u32 = 16u32;
pub const PshedFAErrorSourceControl: u32 = 2u32;
pub const PshedPiEnableNotifyErrorCreateNotifyEvent: WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS = 1i32;
pub const PshedPiEnableNotifyErrorCreateSystemThread: WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS = 2i32;
pub const PshedPiEnableNotifyErrorMax: WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS = 3i32;
pub const PshedPiErrReadingPcieOverridesBadSignature: PSHED_PI_ERR_READING_PCIE_OVERRIDES = 4i32;
pub const PshedPiErrReadingPcieOverridesBadSize: PSHED_PI_ERR_READING_PCIE_OVERRIDES = 3i32;
pub const PshedPiErrReadingPcieOverridesNoCapOffset: PSHED_PI_ERR_READING_PCIE_OVERRIDES = 5i32;
pub const PshedPiErrReadingPcieOverridesNoErr: PSHED_PI_ERR_READING_PCIE_OVERRIDES = 0i32;
pub const PshedPiErrReadingPcieOverridesNoMemory: PSHED_PI_ERR_READING_PCIE_OVERRIDES = 1i32;
pub const PshedPiErrReadingPcieOverridesNotBinary: PSHED_PI_ERR_READING_PCIE_OVERRIDES = 6i32;
pub const PshedPiErrReadingPcieOverridesQueryErr: PSHED_PI_ERR_READING_PCIE_OVERRIDES = 2i32;
pub const QuerySecurityDescriptor: SECURITY_OPERATION_CODE = 1i32;
pub const RCB128Bytes: PCI_EXPRESS_RCB = 1i32;
pub const RCB64Bytes: PCI_EXPRESS_RCB = 0i32;
pub const RECOVERY_INFO_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc34832a1_02c3_4c52_a9f1_9f1d5d7723fc);
pub const RESOURCE_HASH_TABLE_SIZE: u32 = 64u32;
pub const RESULT_NEGATIVE: u32 = 1u32;
pub const RESULT_POSITIVE: u32 = 2u32;
pub const RESULT_ZERO: u32 = 0u32;
pub const ROOT_CMD_ENABLE_CORRECTABLE_ERROR_REPORTING: u32 = 1u32;
pub const ROOT_CMD_ENABLE_FATAL_ERROR_REPORTING: u32 = 4u32;
pub const ROOT_CMD_ENABLE_NONFATAL_ERROR_REPORTING: u32 = 2u32;
pub const RTL_GUID_STRING_SIZE: u32 = 38u32;
pub const RTL_HASH_ALLOCATED_HEADER: u32 = 1u32;
pub const RTL_HASH_RESERVED_SIGNATURE: u32 = 0u32;
pub const RTL_QUERY_REGISTRY_DELETE: u32 = 64u32;
pub const RTL_QUERY_REGISTRY_DIRECT: u32 = 32u32;
pub const RTL_QUERY_REGISTRY_NOEXPAND: u32 = 16u32;
pub const RTL_QUERY_REGISTRY_NOSTRING: u32 = 128u32;
pub const RTL_QUERY_REGISTRY_NOVALUE: u32 = 8u32;
pub const RTL_QUERY_REGISTRY_REQUIRED: u32 = 4u32;
pub const RTL_QUERY_REGISTRY_SUBKEY: u32 = 1u32;
pub const RTL_QUERY_REGISTRY_TOPKEY: u32 = 2u32;
pub const RTL_QUERY_REGISTRY_TYPECHECK: u32 = 256u32;
pub const RTL_QUERY_REGISTRY_TYPECHECK_SHIFT: u32 = 24u32;
pub const RTL_REGISTRY_ABSOLUTE: u32 = 0u32;
pub const RTL_REGISTRY_CONTROL: u32 = 2u32;
pub const RTL_REGISTRY_DEVICEMAP: u32 = 4u32;
pub const RTL_REGISTRY_HANDLE: u32 = 1073741824u32;
pub const RTL_REGISTRY_MAXIMUM: u32 = 6u32;
pub const RTL_REGISTRY_OPTIONAL: u32 = 2147483648u32;
pub const RTL_REGISTRY_SERVICES: u32 = 1u32;
pub const RTL_REGISTRY_USER: u32 = 5u32;
pub const RTL_REGISTRY_WINDOWS_NT: u32 = 3u32;
pub const RTL_STACK_WALKING_MODE_FRAMES_TO_SKIP_SHIFT: u32 = 8u32;
pub const RandomAccess: IO_ACCESS_MODE = 1i32;
pub const ReadAccess: IO_ACCESS_TYPE = 0i32;
pub const RealModeIrqRoutingTable: CONFIGURATION_TYPE = 39i32;
pub const RealModePCIEnumeration: CONFIGURATION_TYPE = 40i32;
pub const RealTimeWorkQueue: WORK_QUEUE_TYPE = 5i32;
pub const RebuildControl: NPEM_CONTROL_STANDARD_CONTROL_BIT = 5i32;
pub const RegNtCallbackObjectContextCleanup: REG_NOTIFY_CLASS = 40i32;
pub const RegNtDeleteKey: REG_NOTIFY_CLASS = 0i32;
pub const RegNtDeleteValueKey: REG_NOTIFY_CLASS = 2i32;
pub const RegNtEnumerateKey: REG_NOTIFY_CLASS = 5i32;
pub const RegNtEnumerateValueKey: REG_NOTIFY_CLASS = 6i32;
pub const RegNtKeyHandleClose: REG_NOTIFY_CLASS = 14i32;
pub const RegNtPostCreateKey: REG_NOTIFY_CLASS = 11i32;
pub const RegNtPostCreateKeyEx: REG_NOTIFY_CLASS = 27i32;
pub const RegNtPostDeleteKey: REG_NOTIFY_CLASS = 15i32;
pub const RegNtPostDeleteValueKey: REG_NOTIFY_CLASS = 17i32;
pub const RegNtPostEnumerateKey: REG_NOTIFY_CLASS = 20i32;
pub const RegNtPostEnumerateValueKey: REG_NOTIFY_CLASS = 21i32;
pub const RegNtPostFlushKey: REG_NOTIFY_CLASS = 31i32;
pub const RegNtPostKeyHandleClose: REG_NOTIFY_CLASS = 25i32;
pub const RegNtPostLoadKey: REG_NOTIFY_CLASS = 33i32;
pub const RegNtPostOpenKey: REG_NOTIFY_CLASS = 13i32;
pub const RegNtPostOpenKeyEx: REG_NOTIFY_CLASS = 29i32;
pub const RegNtPostQueryKey: REG_NOTIFY_CLASS = 22i32;
pub const RegNtPostQueryKeyName: REG_NOTIFY_CLASS = 48i32;
pub const RegNtPostQueryKeySecurity: REG_NOTIFY_CLASS = 37i32;
pub const RegNtPostQueryMultipleValueKey: REG_NOTIFY_CLASS = 24i32;
pub const RegNtPostQueryValueKey: REG_NOTIFY_CLASS = 23i32;
pub const RegNtPostRenameKey: REG_NOTIFY_CLASS = 19i32;
pub const RegNtPostReplaceKey: REG_NOTIFY_CLASS = 46i32;
pub const RegNtPostRestoreKey: REG_NOTIFY_CLASS = 42i32;
pub const RegNtPostSaveKey: REG_NOTIFY_CLASS = 44i32;
pub const RegNtPostSaveMergedKey: REG_NOTIFY_CLASS = 50i32;
pub const RegNtPostSetInformationKey: REG_NOTIFY_CLASS = 18i32;
pub const RegNtPostSetKeySecurity: REG_NOTIFY_CLASS = 39i32;
pub const RegNtPostSetValueKey: REG_NOTIFY_CLASS = 16i32;
pub const RegNtPostUnLoadKey: REG_NOTIFY_CLASS = 35i32;
pub const RegNtPreCreateKey: REG_NOTIFY_CLASS = 10i32;
pub const RegNtPreCreateKeyEx: REG_NOTIFY_CLASS = 26i32;
pub const RegNtPreDeleteKey: REG_NOTIFY_CLASS = 0i32;
pub const RegNtPreDeleteValueKey: REG_NOTIFY_CLASS = 2i32;
pub const RegNtPreEnumerateKey: REG_NOTIFY_CLASS = 5i32;
pub const RegNtPreEnumerateValueKey: REG_NOTIFY_CLASS = 6i32;
pub const RegNtPreFlushKey: REG_NOTIFY_CLASS = 30i32;
pub const RegNtPreKeyHandleClose: REG_NOTIFY_CLASS = 14i32;
pub const RegNtPreLoadKey: REG_NOTIFY_CLASS = 32i32;
pub const RegNtPreOpenKey: REG_NOTIFY_CLASS = 12i32;
pub const RegNtPreOpenKeyEx: REG_NOTIFY_CLASS = 28i32;
pub const RegNtPreQueryKey: REG_NOTIFY_CLASS = 7i32;
pub const RegNtPreQueryKeyName: REG_NOTIFY_CLASS = 47i32;
pub const RegNtPreQueryKeySecurity: REG_NOTIFY_CLASS = 36i32;
pub const RegNtPreQueryMultipleValueKey: REG_NOTIFY_CLASS = 9i32;
pub const RegNtPreQueryValueKey: REG_NOTIFY_CLASS = 8i32;
pub const RegNtPreRenameKey: REG_NOTIFY_CLASS = 4i32;
pub const RegNtPreReplaceKey: REG_NOTIFY_CLASS = 45i32;
pub const RegNtPreRestoreKey: REG_NOTIFY_CLASS = 41i32;
pub const RegNtPreSaveKey: REG_NOTIFY_CLASS = 43i32;
pub const RegNtPreSaveMergedKey: REG_NOTIFY_CLASS = 49i32;
pub const RegNtPreSetInformationKey: REG_NOTIFY_CLASS = 3i32;
pub const RegNtPreSetKeySecurity: REG_NOTIFY_CLASS = 38i32;
pub const RegNtPreSetValueKey: REG_NOTIFY_CLASS = 1i32;
pub const RegNtPreUnLoadKey: REG_NOTIFY_CLASS = 34i32;
pub const RegNtQueryKey: REG_NOTIFY_CLASS = 7i32;
pub const RegNtQueryMultipleValueKey: REG_NOTIFY_CLASS = 9i32;
pub const RegNtQueryValueKey: REG_NOTIFY_CLASS = 8i32;
pub const RegNtRenameKey: REG_NOTIFY_CLASS = 4i32;
pub const RegNtSetInformationKey: REG_NOTIFY_CLASS = 3i32;
pub const RegNtSetValueKey: REG_NOTIFY_CLASS = 1i32;
pub const RemovalPolicyExpectNoRemoval: DEVICE_REMOVAL_POLICY = 1i32;
pub const RemovalPolicyExpectOrderlyRemoval: DEVICE_REMOVAL_POLICY = 2i32;
pub const RemovalPolicyExpectSurpriseRemoval: DEVICE_REMOVAL_POLICY = 3i32;
pub const RemovalRelations: DEVICE_RELATION_TYPE = 3i32;
pub const ResourceNeverExclusive: u32 = 16u32;
pub const ResourceOwnedExclusive: u32 = 128u32;
pub const ResourceReleaseByOtherThread: u32 = 32u32;
pub const ResourceTypeEventBuffer: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = 4i32;
pub const ResourceTypeExtendedCounterConfiguration: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = 2i32;
pub const ResourceTypeIdenitificationTag: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = 5i32;
pub const ResourceTypeMax: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = 6i32;
pub const ResourceTypeOverflow: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = 3i32;
pub const ResourceTypeRange: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = 1i32;
pub const ResourceTypeSingle: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = 0i32;
pub const ResultNegative: INTERLOCKED_RESULT = 32768i32;
pub const ResultPositive: INTERLOCKED_RESULT = 0i32;
pub const ResultZero: INTERLOCKED_RESULT = 16384i32;
pub const SCI_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe9d59197_94ee_4a4f_8ad8_9b7d8bd93d2e);
pub const SDEV_IDENTIFIER_INTERFACE_VERSION: u32 = 1u32;
pub const SEA_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9a78788a_bbe8_11e4_809e_67611e5d46b0);
pub const SEA_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf5fe48a6_84ce_4c1e_aa64_20c9a53099f1);
pub const SECTION_MAP_EXECUTE: u32 = 8u32;
pub const SECTION_MAP_EXECUTE_EXPLICIT: u32 = 32u32;
pub const SECTION_MAP_READ: u32 = 4u32;
pub const SECTION_MAP_WRITE: u32 = 2u32;
pub const SECTION_QUERY: u32 = 1u32;
pub const SECURE_DRIVER_INTERFACE_VERSION: u32 = 1u32;
pub const SECURE_POOL_FLAGS_FREEABLE: u32 = 1u32;
pub const SECURE_POOL_FLAGS_MODIFIABLE: u32 = 2u32;
pub const SECURE_POOL_FLAGS_NONE: u32 = 0u32;
pub const SECURE_SECTION_ALLOW_PARTIAL_MDL: u32 = 1u32;
pub const SEC_LARGE_PAGES: u32 = 2147483648u32;
pub const SEH_VALIDATION_POLICY_DEFER: u32 = 3u32;
pub const SEH_VALIDATION_POLICY_OFF: u32 = 1u32;
pub const SEH_VALIDATION_POLICY_ON: u32 = 0u32;
pub const SEH_VALIDATION_POLICY_TELEMETRY: u32 = 2u32;
pub const SEI_NOTIFY_TYPE_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5c284c81_b0ae_4e87_a322_b04c85624323);
pub const SEI_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf2a4a152_9c6d_4020_aecf_7695b389251b);
pub const SEMAPHORE_QUERY_STATE: u32 = 1u32;
pub const SE_ASSIGNPRIMARYTOKEN_PRIVILEGE: i32 = 3i32;
pub const SE_AUDIT_PRIVILEGE: i32 = 21i32;
pub const SE_BACKUP_PRIVILEGE: i32 = 17i32;
pub const SE_CHANGE_NOTIFY_PRIVILEGE: i32 = 23i32;
pub const SE_CREATE_GLOBAL_PRIVILEGE: i32 = 30i32;
pub const SE_CREATE_PAGEFILE_PRIVILEGE: i32 = 15i32;
pub const SE_CREATE_PERMANENT_PRIVILEGE: i32 = 16i32;
pub const SE_CREATE_SYMBOLIC_LINK_PRIVILEGE: i32 = 35i32;
pub const SE_CREATE_TOKEN_PRIVILEGE: i32 = 2i32;
pub const SE_DEBUG_PRIVILEGE: i32 = 20i32;
pub const SE_DELEGATE_SESSION_USER_IMPERSONATE_PRIVILEGE: i32 = 36i32;
pub const SE_ENABLE_DELEGATION_PRIVILEGE: i32 = 27i32;
pub const SE_IMPERSONATE_PRIVILEGE: i32 = 29i32;
pub const SE_INCREASE_QUOTA_PRIVILEGE: i32 = 5i32;
pub const SE_INC_BASE_PRIORITY_PRIVILEGE: i32 = 14i32;
pub const SE_INC_WORKING_SET_PRIVILEGE: i32 = 33i32;
pub const SE_LOAD_DRIVER_PRIVILEGE: i32 = 10i32;
pub const SE_LOCK_MEMORY_PRIVILEGE: i32 = 4i32;
pub const SE_MACHINE_ACCOUNT_PRIVILEGE: i32 = 6i32;
pub const SE_MANAGE_VOLUME_PRIVILEGE: i32 = 28i32;
pub const SE_MAX_WELL_KNOWN_PRIVILEGE: i32 = 36i32;
pub const SE_MIN_WELL_KNOWN_PRIVILEGE: i32 = 2i32;
pub const SE_PROF_SINGLE_PROCESS_PRIVILEGE: i32 = 13i32;
pub const SE_RELABEL_PRIVILEGE: i32 = 32i32;
pub const SE_REMOTE_SHUTDOWN_PRIVILEGE: i32 = 24i32;
pub const SE_RESTORE_PRIVILEGE: i32 = 18i32;
pub const SE_SECURITY_PRIVILEGE: i32 = 8i32;
pub const SE_SHUTDOWN_PRIVILEGE: i32 = 19i32;
pub const SE_SYNC_AGENT_PRIVILEGE: i32 = 26i32;
pub const SE_SYSTEMTIME_PRIVILEGE: i32 = 12i32;
pub const SE_SYSTEM_ENVIRONMENT_PRIVILEGE: i32 = 22i32;
pub const SE_SYSTEM_PROFILE_PRIVILEGE: i32 = 11i32;
pub const SE_TAKE_OWNERSHIP_PRIVILEGE: i32 = 9i32;
pub const SE_TCB_PRIVILEGE: i32 = 7i32;
pub const SE_TIME_ZONE_PRIVILEGE: i32 = 34i32;
pub const SE_TRUSTED_CREDMAN_ACCESS_PRIVILEGE: i32 = 31i32;
pub const SE_UNDOCK_PRIVILEGE: i32 = 25i32;
pub const SE_UNSOLICITED_INPUT_PRIVILEGE: i32 = 6i32;
pub const SHARED_GLOBAL_FLAGS_CLEAR_GLOBAL_DATA_FLAG: u32 = 2147483648u32;
pub const SHARED_GLOBAL_FLAGS_CONSOLE_BROKER_ENABLED_V: u32 = 6u32;
pub const SHARED_GLOBAL_FLAGS_DYNAMIC_PROC_ENABLED_V: u32 = 5u32;
pub const SHARED_GLOBAL_FLAGS_ELEVATION_ENABLED_V: u32 = 1u32;
pub const SHARED_GLOBAL_FLAGS_ERROR_PORT_V: u32 = 0u32;
pub const SHARED_GLOBAL_FLAGS_INSTALLER_DETECT_ENABLED_V: u32 = 3u32;
pub const SHARED_GLOBAL_FLAGS_LKG_ENABLED_V: u32 = 4u32;
pub const SHARED_GLOBAL_FLAGS_MULTIUSERS_IN_SESSION_SKU_V: u32 = 9u32;
pub const SHARED_GLOBAL_FLAGS_MULTI_SESSION_SKU_V: u32 = 8u32;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_A73_ERRATA: u32 = 64u32;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_DISABLE_32BIT: u32 = 4u32;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_ENABLED: u32 = 1u32;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_HV_PAGE: u32 = 2u32;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_LFENCE: u32 = 32u32;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_MFENCE: u32 = 16u32;
pub const SHARED_GLOBAL_FLAGS_QPC_BYPASS_USE_RDTSCP: u32 = 128u32;
pub const SHARED_GLOBAL_FLAGS_SECURE_BOOT_ENABLED_V: u32 = 7u32;
pub const SHARED_GLOBAL_FLAGS_SET_GLOBAL_DATA_FLAG: u32 = 1073741824u32;
pub const SHARED_GLOBAL_FLAGS_STATE_SEPARATION_ENABLED_V: u32 = 10u32;
pub const SHARED_GLOBAL_FLAGS_VIRT_ENABLED_V: u32 = 2u32;
pub const SHORT_LEAST_SIGNIFICANT_BIT: u32 = 0u32;
pub const SHORT_MOST_SIGNIFICANT_BIT: u32 = 1u32;
pub const SILO_MONITOR_REGISTRATION_VERSION: u32 = 1u32;
pub const SINGLE_GROUP_LEGACY_API: u32 = 1u32;
pub const SL_ALLOW_RAW_MOUNT: u32 = 1u32;
pub const SL_BYPASS_ACCESS_CHECK: u32 = 1u32;
pub const SL_BYPASS_IO: u32 = 64u32;
pub const SL_CASE_SENSITIVE: u32 = 128u32;
pub const SL_ERROR_RETURNED: u32 = 2u32;
pub const SL_EXCLUSIVE_LOCK: u32 = 2u32;
pub const SL_FAIL_IMMEDIATELY: u32 = 1u32;
pub const SL_FORCE_ACCESS_CHECK: u32 = 1u32;
pub const SL_FORCE_ASYNCHRONOUS: u32 = 1u32;
pub const SL_FORCE_DIRECT_WRITE: u32 = 16u32;
pub const SL_FT_SEQUENTIAL_WRITE: u32 = 8u32;
pub const SL_IGNORE_READONLY_ATTRIBUTE: u32 = 64u32;
pub const SL_INDEX_SPECIFIED: u32 = 4u32;
pub const SL_INFO_FORCE_ACCESS_CHECK: u32 = 1u32;
pub const SL_INFO_IGNORE_READONLY_ATTRIBUTE: u32 = 64u32;
pub const SL_INVOKE_ON_CANCEL: u32 = 32u32;
pub const SL_INVOKE_ON_ERROR: u32 = 128u32;
pub const SL_INVOKE_ON_SUCCESS: u32 = 64u32;
pub const SL_KEY_SPECIFIED: u32 = 1u32;
pub const SL_NO_CURSOR_UPDATE: u32 = 16u32;
pub const SL_OPEN_PAGING_FILE: u32 = 2u32;
pub const SL_OPEN_TARGET_DIRECTORY: u32 = 4u32;
pub const SL_OVERRIDE_VERIFY_VOLUME: u32 = 2u32;
pub const SL_PENDING_RETURNED: u32 = 1u32;
pub const SL_PERSISTENT_MEMORY_FIXED_MAPPING: u32 = 32u32;
pub const SL_QUERY_DIRECTORY_MASK: u32 = 27u32;
pub const SL_READ_ACCESS_GRANTED: u32 = 1u32;
pub const SL_REALTIME_STREAM: u32 = 32u32;
pub const SL_RESTART_SCAN: u32 = 1u32;
pub const SL_RETURN_ON_DISK_ENTRIES_ONLY: u32 = 8u32;
pub const SL_RETURN_SINGLE_ENTRY: u32 = 2u32;
pub const SL_STOP_ON_SYMLINK: u32 = 8u32;
pub const SL_WATCH_TREE: u32 = 1u32;
pub const SL_WRITE_ACCESS_GRANTED: u32 = 4u32;
pub const SL_WRITE_THROUGH: u32 = 4u32;
pub const SOC_SUBSYS_AUDIO_DSP: SOC_SUBSYSTEM_TYPE = 1i32;
pub const SOC_SUBSYS_COMPUTE_DSP: SOC_SUBSYSTEM_TYPE = 4i32;
pub const SOC_SUBSYS_SECURE_PROC: SOC_SUBSYSTEM_TYPE = 5i32;
pub const SOC_SUBSYS_SENSORS: SOC_SUBSYSTEM_TYPE = 3i32;
pub const SOC_SUBSYS_VENDOR_DEFINED: SOC_SUBSYSTEM_TYPE = 65536i32;
pub const SOC_SUBSYS_WIRELESS_MODEM: SOC_SUBSYSTEM_TYPE = 0i32;
pub const SOC_SUBSYS_WIRELSS_CONNECTIVITY: SOC_SUBSYSTEM_TYPE = 2i32;
pub const SSINFO_FLAGS_ALIGNED_DEVICE: u32 = 1u32;
pub const SSINFO_FLAGS_BYTE_ADDRESSABLE: u32 = 16u32;
pub const SSINFO_FLAGS_NO_SEEK_PENALTY: u32 = 4u32;
pub const SSINFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE: u32 = 2u32;
pub const SSINFO_FLAGS_TRIM_ENABLED: u32 = 8u32;
pub const SSINFO_OFFSET_UNKNOWN: u32 = 4294967295u32;
pub const SYMBOLIC_LINK_QUERY: u32 = 1u32;
pub const SYMBOLIC_LINK_SET: u32 = 2u32;
pub const SYSTEM_CALL_INT_2E: u32 = 1u32;
pub const SYSTEM_CALL_SYSCALL: u32 = 0u32;
pub const ScsiAdapter: CONFIGURATION_TYPE = 10i32;
pub const SeImageTypeDriver: SE_IMAGE_TYPE = 1i32;
pub const SeImageTypeDynamicCodeFile: SE_IMAGE_TYPE = 3i32;
pub const SeImageTypeElamDriver: SE_IMAGE_TYPE = 0i32;
pub const SeImageTypeMax: SE_IMAGE_TYPE = 4i32;
pub const SeImageTypePlatformSecureFile: SE_IMAGE_TYPE = 2i32;
pub const SeImageVerificationCallbackInformational: SE_IMAGE_VERIFICATION_CALLBACK_TYPE = 0i32;
pub const SecondaryCache: CONFIGURATION_TYPE = 7i32;
pub const SecondaryDcache: CONFIGURATION_TYPE = 6i32;
pub const SecondaryIcache: CONFIGURATION_TYPE = 5i32;
pub const SequentialAccess: IO_ACCESS_MODE = 0i32;
pub const SerialController: CONFIGURATION_TYPE = 17i32;
pub const SetSecurityDescriptor: SECURITY_OPERATION_CODE = 0i32;
pub const SgiInternalConfiguration: BUS_DATA_TYPE = 11i32;
pub const SharedInterruptTime: u32 = 4292804616u32;
pub const SharedSystemTime: u32 = 4292804628u32;
pub const SharedTickCount: u32 = 4292805408u32;
pub const SingleBusRelations: DEVICE_RELATION_TYPE = 5i32;
pub const SlotEmpty: PCI_EXPRESS_CARD_PRESENCE = 0i32;
pub const StandardDesign: ALTERNATIVE_ARCHITECTURE_TYPE = 0i32;
pub const StopCompletion: IO_COMPLETION_ROUTINE_RESULT = -1073741802i32;
pub const SubsystemInformationTypeWSL: SUBSYSTEM_INFORMATION_TYPE = 1i32;
pub const SubsystemInformationTypeWin32: SUBSYSTEM_INFORMATION_TYPE = 0i32;
pub const SuperCriticalWorkQueue: WORK_QUEUE_TYPE = 6i32;
pub const Suspended: KWAIT_REASON = 5i32;
pub const SystemFirmwareTable_Enumerate: SYSTEM_FIRMWARE_TABLE_ACTION = 0i32;
pub const SystemFirmwareTable_Get: SYSTEM_FIRMWARE_TABLE_ACTION = 1i32;
pub const SystemMemory: CONFIGURATION_TYPE = 37i32;
pub const SystemMemoryPartitionDedicatedMemoryInformation: PARTITION_INFORMATION_CLASS = 9i32;
pub const SystemMemoryPartitionInformation: PARTITION_INFORMATION_CLASS = 0i32;
pub const SystemMemoryPartitionOpenDedicatedMemory: PARTITION_INFORMATION_CLASS = 10i32;
pub const SystemPowerState: POWER_STATE_TYPE = 0i32;
pub const THREAD_ALERT: u32 = 4u32;
pub const THREAD_CSWITCH_PMU_DISABLE: u32 = 0u32;
pub const THREAD_CSWITCH_PMU_ENABLE: u32 = 1u32;
pub const THREAD_GET_CONTEXT: u32 = 8u32;
pub const THREAD_WAIT_OBJECTS: u32 = 3u32;
pub const TIMER_EXPIRED_INDEX_BITS: u32 = 6u32;
pub const TIMER_PROCESSOR_INDEX_BITS: u32 = 5u32;
pub const TIMER_TOLERABLE_DELAY_BITS: u32 = 6u32;
pub const TREE_CONNECT_NO_CLIENT_BUFFERING: u32 = 8u32;
pub const TREE_CONNECT_WRITE_THROUGH: u32 = 2u32;
pub const TXF_MINIVERSION_DEFAULT_VIEW: u32 = 65534u32;
pub const TableEmptyTree: TABLE_SEARCH_RESULT = 0i32;
pub const TableFoundNode: TABLE_SEARCH_RESULT = 1i32;
pub const TableInsertAsLeft: TABLE_SEARCH_RESULT = 2i32;
pub const TableInsertAsRight: TABLE_SEARCH_RESULT = 3i32;
pub const TapeController: CONFIGURATION_TYPE = 14i32;
pub const TapePeripheral: CONFIGURATION_TYPE = 27i32;
pub const TargetDeviceRelation: DEVICE_RELATION_TYPE = 4i32;
pub const TcAdapter: CONFIGURATION_TYPE = 9i32;
pub const TerminalPeripheral: CONFIGURATION_TYPE = 33i32;
pub const TlbMatchConflict: FAULT_INFORMATION_ARM64_TYPE = 2i32;
pub const TraceEnableFlagsClass: TRACE_INFORMATION_CLASS = 2i32;
pub const TraceEnableLevelClass: TRACE_INFORMATION_CLASS = 3i32;
pub const TraceHandleByNameClass: TRACE_INFORMATION_CLASS = 7i32;
pub const TraceHandleClass: TRACE_INFORMATION_CLASS = 1i32;
pub const TraceIdClass: TRACE_INFORMATION_CLASS = 0i32;
pub const TraceInformationClassReserved1: TRACE_INFORMATION_CLASS = 12i32;
pub const TraceInformationClassReserved2: TRACE_INFORMATION_CLASS = 14i32;
pub const TraceSessionSettingsClass: TRACE_INFORMATION_CLASS = 9i32;
pub const TranslateChildToParent: RESOURCE_TRANSLATION_DIRECTION = 0i32;
pub const TranslateParentToChild: RESOURCE_TRANSLATION_DIRECTION = 1i32;
pub const TranslationFault: FAULT_INFORMATION_ARM64_TYPE = 6i32;
pub const TransportRelations: DEVICE_RELATION_TYPE = 6i32;
pub const TurboChannel: INTERFACE_TYPE = 4i32;
pub const TypeA: DMA_SPEED = 1i32;
pub const TypeB: DMA_SPEED = 2i32;
pub const TypeC: DMA_SPEED = 3i32;
pub const TypeF: DMA_SPEED = 4i32;
pub const UADDRESS_BASE: u32 = 0u32;
pub const UnsupportedUpstreamTransaction: FAULT_INFORMATION_ARM64_TYPE = 0i32;
pub const UserMode: MODE = 1i32;
pub const UserNotPresent: POWER_USER_PRESENCE_TYPE = 0i32;
pub const UserPresent: POWER_USER_PRESENCE_TYPE = 1i32;
pub const UserRequest: KWAIT_REASON = 6i32;
pub const UserUnknown: POWER_USER_PRESENCE_TYPE = 255i32;
pub const VMEBus: INTERFACE_TYPE = 6i32;
pub const VMEConfiguration: BUS_DATA_TYPE = 5i32;
pub const VPB_DIRECT_WRITES_ALLOWED: u32 = 32u32;
pub const VPB_DISMOUNTING: u32 = 128u32;
pub const VPB_FLAGS_BYPASSIO_BLOCKED: u32 = 64u32;
pub const VPB_LOCKED: u32 = 2u32;
pub const VPB_MOUNTED: u32 = 1u32;
pub const VPB_PERSISTENT: u32 = 4u32;
pub const VPB_RAW_MOUNT: u32 = 16u32;
pub const VPB_REMOVE_PENDING: u32 = 8u32;
pub const Vmcs: INTERFACE_TYPE = 16i32;
pub const WCS_RAS_REGISTER_NAME_MAX_LENGTH: u32 = 32u32;
pub const WDM_MAJORVERSION: u32 = 6u32;
pub const WDM_MINORVERSION: u32 = 0u32;
pub const WHEA_AMD_EXT_REG_NUM: u32 = 10u32;
pub const WHEA_BUSCHECK_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x1cf3f8b3_c5b1_49a2_aa59_5eef92ffa63c);
pub const WHEA_CACHECHECK_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa55701f5_e3ef_43de_ac72_249b573fad2c);
pub const WHEA_DEVICE_ERROR_SUMMARY_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x990b31e9_541a_4db0_a42f_837d344f6923);
pub const WHEA_DPC_CAPABILITY_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xec49534b_30e7_4358_972f_eca6958fae3b);
pub const WHEA_ERROR_LOG_ENTRY_VERSION: u32 = 1u32;
pub const WHEA_ERROR_PACKET_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe71254e9_c1b9_4940_ab76_909703a4320f);
pub const WHEA_ERROR_PACKET_V1_VERSION: u32 = 2u32;
pub const WHEA_ERROR_PACKET_V2_VERSION: u32 = 3u32;
pub const WHEA_ERROR_PACKET_VERSION: u32 = 3u32;
pub const WHEA_ERROR_PKT_VERSION: u32 = 3u32;
pub const WHEA_ERROR_RECORD_FLAGS_DEVICE_DRIVER: u32 = 8u32;
pub const WHEA_ERROR_RECORD_FLAGS_PREVIOUSERROR: u32 = 2u32;
pub const WHEA_ERROR_RECORD_FLAGS_RECOVERED: u32 = 1u32;
pub const WHEA_ERROR_RECORD_FLAGS_SIMULATED: u32 = 4u32;
pub const WHEA_ERROR_RECORD_REVISION: u32 = 528u32;
pub const WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_REVISION: u32 = 768u32;
pub const WHEA_ERROR_RECORD_SIGNATURE_END: u32 = 4294967295u32;
pub const WHEA_ERROR_RECORD_VALID_PARTITIONID: u32 = 4u32;
pub const WHEA_ERROR_RECORD_VALID_PLATFORMID: u32 = 1u32;
pub const WHEA_ERROR_RECORD_VALID_TIMESTAMP: u32 = 2u32;
pub const WHEA_ERROR_TEXT_LEN: u32 = 32u32;
pub const WHEA_ERR_SRC_OVERRIDE_FLAG: u32 = 1u32;
pub const WHEA_FIRMWARE_RECORD_TYPE_IPFSAL: u32 = 0u32;
pub const WHEA_GENERIC_ENTRY_TEXT_LEN: u32 = 20u32;
pub const WHEA_GENERIC_ENTRY_V2_VERSION: u32 = 768u32;
pub const WHEA_GENERIC_ENTRY_VERSION: u32 = 768u32;
pub const WHEA_INVALID_ERR_SRC_ID: u32 = 0u32;
pub const WHEA_IN_USE_PAGE_NOTIFY_FLAG_NOTIFYALL: u32 = 64u32;
pub const WHEA_IN_USE_PAGE_NOTIFY_FLAG_PAGEOFFLINED: u32 = 128u32;
pub const WHEA_IN_USE_PAGE_NOTIFY_FLAG_PLATFORMDIRECTED: u32 = 1u32;
pub const WHEA_MAX_LOG_DATA_LEN: u32 = 36u32;
pub const WHEA_MEMERRTYPE_INVALIDADDRESS: u32 = 10u32;
pub const WHEA_MEMERRTYPE_MASTERABORT: u32 = 6u32;
pub const WHEA_MEMERRTYPE_MEMORYSPARING: u32 = 12u32;
pub const WHEA_MEMERRTYPE_MIRRORBROKEN: u32 = 11u32;
pub const WHEA_MEMERRTYPE_MULTIBITECC: u32 = 3u32;
pub const WHEA_MEMERRTYPE_MULTISYMCHIPKILL: u32 = 5u32;
pub const WHEA_MEMERRTYPE_NOERROR: u32 = 1u32;
pub const WHEA_MEMERRTYPE_PARITYERROR: u32 = 8u32;
pub const WHEA_MEMERRTYPE_SINGLEBITECC: u32 = 2u32;
pub const WHEA_MEMERRTYPE_SINGLESYMCHIPKILL: u32 = 4u32;
pub const WHEA_MEMERRTYPE_TARGETABORT: u32 = 7u32;
pub const WHEA_MEMERRTYPE_UNKNOWN: u32 = 0u32;
pub const WHEA_MEMERRTYPE_WATCHDOGTIMEOUT: u32 = 9u32;
pub const WHEA_MSCHECK_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x48ab7f57_dc34_4f6c_a7d3_b0b5b0a74314);
pub const WHEA_PCIE_CORRECTABLE_ERROR_SECTION_COUNT_SIZE: u32 = 32u32;
pub const WHEA_PLUGIN_REGISTRATION_PACKET_V1: u32 = 65536u32;
pub const WHEA_PLUGIN_REGISTRATION_PACKET_V2: u32 = 131072u32;
pub const WHEA_PLUGIN_REGISTRATION_PACKET_VERSION: u32 = 131072u32;
pub const WHEA_PMEM_ERROR_SECTION_LOCATION_INFO_SIZE: u32 = 64u32;
pub const WHEA_PMEM_ERROR_SECTION_MAX_PAGES: u32 = 50u32;
pub const WHEA_RECORD_CREATOR_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcf07c4bd_b789_4e18_b3c4_1f732cb57131);
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_CONTAINMENTWRN: u32 = 2u32;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_FRU_TEXT_BY_PLUGIN: u32 = 128u32;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_LATENTERROR: u32 = 32u32;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_PRIMARY: u32 = 1u32;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_PROPAGATED: u32 = 64u32;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_RESET: u32 = 4u32;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_RESOURCENA: u32 = 16u32;
pub const WHEA_SECTION_DESCRIPTOR_FLAGS_THRESHOLDEXCEEDED: u32 = 8u32;
pub const WHEA_SECTION_DESCRIPTOR_REVISION: u32 = 768u32;
pub const WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_VERSION: u32 = 1u32;
pub const WHEA_TLBCHECK_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xfc06b535_5e1f_4562_9f25_0a3b9adb63c3);
pub const WHEA_WRITE_FLAG_DUMMY: u32 = 1u32;
pub const WHEA_XPF_MCA_EXTREG_MAX_COUNT: u32 = 24u32;
pub const WHEA_XPF_MCA_SECTION_VERSION: u32 = 3u32;
pub const WHEA_XPF_MCA_SECTION_VERSION_2: u32 = 2u32;
pub const WHEA_XPF_MCA_SECTION_VERSION_3: u32 = 3u32;
pub const WMIREGISTER: u32 = 0u32;
pub const WMIREG_ACTION_BLOCK_IRPS: u32 = 5u32;
pub const WMIREG_ACTION_DEREGISTER: u32 = 2u32;
pub const WMIREG_ACTION_REGISTER: u32 = 1u32;
pub const WMIREG_ACTION_REREGISTER: u32 = 3u32;
pub const WMIREG_ACTION_UPDATE_GUIDS: u32 = 4u32;
pub const WMIUPDATE: u32 = 1u32;
pub const WdfNotifyRoutinesClass: TRACE_INFORMATION_CLASS = 15i32;
pub const WheaCpuVendorAmd: WHEA_CPU_VENDOR = 2i32;
pub const WheaCpuVendorIntel: WHEA_CPU_VENDOR = 1i32;
pub const WheaCpuVendorOther: WHEA_CPU_VENDOR = 0i32;
pub const WheaDataFormatGeneric: WHEA_ERROR_PACKET_DATA_FORMAT = 7i32;
pub const WheaDataFormatIPFSalRecord: WHEA_ERROR_PACKET_DATA_FORMAT = 0i32;
pub const WheaDataFormatMax: WHEA_ERROR_PACKET_DATA_FORMAT = 8i32;
pub const WheaDataFormatMemory: WHEA_ERROR_PACKET_DATA_FORMAT = 2i32;
pub const WheaDataFormatNMIPort: WHEA_ERROR_PACKET_DATA_FORMAT = 4i32;
pub const WheaDataFormatPCIExpress: WHEA_ERROR_PACKET_DATA_FORMAT = 3i32;
pub const WheaDataFormatPCIXBus: WHEA_ERROR_PACKET_DATA_FORMAT = 5i32;
pub const WheaDataFormatPCIXDevice: WHEA_ERROR_PACKET_DATA_FORMAT = 6i32;
pub const WheaDataFormatXPFMCA: WHEA_ERROR_PACKET_DATA_FORMAT = 1i32;
pub const WheaErrSevCorrected: WHEA_ERROR_SEVERITY = 2i32;
pub const WheaErrSevFatal: WHEA_ERROR_SEVERITY = 1i32;
pub const WheaErrSevInformational: WHEA_ERROR_SEVERITY = 3i32;
pub const WheaErrSevRecoverable: WHEA_ERROR_SEVERITY = 0i32;
pub const WheaErrTypeGeneric: WHEA_ERROR_TYPE = 6i32;
pub const WheaErrTypeMemory: WHEA_ERROR_TYPE = 1i32;
pub const WheaErrTypeNMI: WHEA_ERROR_TYPE = 3i32;
pub const WheaErrTypePCIExpress: WHEA_ERROR_TYPE = 2i32;
pub const WheaErrTypePCIXBus: WHEA_ERROR_TYPE = 4i32;
pub const WheaErrTypePCIXDevice: WHEA_ERROR_TYPE = 5i32;
pub const WheaErrTypePmem: WHEA_ERROR_TYPE = 7i32;
pub const WheaErrTypeProcessor: WHEA_ERROR_TYPE = 0i32;
pub const WheaEventBugCheckRecoveryEntry: WHEA_BUGCHECK_RECOVERY_LOG_TYPE = 0i32;
pub const WheaEventBugCheckRecoveryMax: WHEA_BUGCHECK_RECOVERY_LOG_TYPE = 2i32;
pub const WheaEventBugCheckRecoveryReturn: WHEA_BUGCHECK_RECOVERY_LOG_TYPE = 1i32;
pub const WheaEventLogAzccRootBusList: WHEA_EVENT_LOG_ENTRY_ID = -2147483617i32;
pub const WheaEventLogAzccRootBusPoisonSet: WHEA_EVENT_LOG_ENTRY_ID = -2147483602i32;
pub const WheaEventLogAzccRootBusSearchErr: WHEA_EVENT_LOG_ENTRY_ID = -2147483618i32;
pub const WheaEventLogCmciFinalRestart: WHEA_EVENT_LOG_ENTRY_ID = -2147483620i32;
pub const WheaEventLogCmciRestart: WHEA_EVENT_LOG_ENTRY_ID = -2147483621i32;
pub const WheaEventLogEntryEarlyError: WHEA_EVENT_LOG_ENTRY_ID = -2147483594i32;
pub const WheaEventLogEntryEtwOverFlow: WHEA_EVENT_LOG_ENTRY_ID = -2147483619i32;
pub const WheaEventLogEntryIdAcpiTimeOut: WHEA_EVENT_LOG_ENTRY_ID = -2147483622i32;
pub const WheaEventLogEntryIdAddRemoveErrorSource: WHEA_EVENT_LOG_ENTRY_ID = -2147483636i32;
pub const WheaEventLogEntryIdAerNotGrantedToOs: WHEA_EVENT_LOG_ENTRY_ID = -2147483624i32;
pub const WheaEventLogEntryIdAttemptErrorRecovery: WHEA_EVENT_LOG_ENTRY_ID = -2147483634i32;
pub const WheaEventLogEntryIdBadHestNotifyData: WHEA_EVENT_LOG_ENTRY_ID = -2147483565i32;
pub const WheaEventLogEntryIdBadPageLimitReached: WHEA_EVENT_LOG_ENTRY_ID = -2147483596i32;
pub const WheaEventLogEntryIdClearedPoison: WHEA_EVENT_LOG_ENTRY_ID = -2147483630i32;
pub const WheaEventLogEntryIdCmcPollingTimeout: WHEA_EVENT_LOG_ENTRY_ID = -2147483647i32;
pub const WheaEventLogEntryIdCmcSwitchToPolling: WHEA_EVENT_LOG_ENTRY_ID = -2147483645i32;
pub const WheaEventLogEntryIdCmciImplPresent: WHEA_EVENT_LOG_ENTRY_ID = -2147483608i32;
pub const WheaEventLogEntryIdCmciInitError: WHEA_EVENT_LOG_ENTRY_ID = -2147483607i32;
pub const WheaEventLogEntryIdCpuBusesInitFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483571i32;
pub const WheaEventLogEntryIdCpusFrozen: WHEA_EVENT_LOG_ENTRY_ID = -2147483552i32;
pub const WheaEventLogEntryIdCpusFrozenNoCrashDump: WHEA_EVENT_LOG_ENTRY_ID = -2147483551i32;
pub const WheaEventLogEntryIdCreateGenericRecord: WHEA_EVENT_LOG_ENTRY_ID = -2147483627i32;
pub const WheaEventLogEntryIdDefectListCorrupt: WHEA_EVENT_LOG_ENTRY_ID = -2147483566i32;
pub const WheaEventLogEntryIdDefectListFull: WHEA_EVENT_LOG_ENTRY_ID = -2147483568i32;
pub const WheaEventLogEntryIdDefectListUEFIVarFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483567i32;
pub const WheaEventLogEntryIdDeviceDriver: WHEA_EVENT_LOG_ENTRY_ID = -2147483609i32;
pub const WheaEventLogEntryIdDroppedCorrectedError: WHEA_EVENT_LOG_ENTRY_ID = -2147483644i32;
pub const WheaEventLogEntryIdDrvErrSrcInvalid: WHEA_EVENT_LOG_ENTRY_ID = -2147483605i32;
pub const WheaEventLogEntryIdDrvHandleBusy: WHEA_EVENT_LOG_ENTRY_ID = -2147483604i32;
pub const WheaEventLogEntryIdEnableKeyNotifFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483580i32;
pub const WheaEventLogEntryIdErrDimmInfoMismatch: WHEA_EVENT_LOG_ENTRY_ID = -2147483600i32;
pub const WheaEventLogEntryIdErrSrcArrayInvalid: WHEA_EVENT_LOG_ENTRY_ID = -2147483623i32;
pub const WheaEventLogEntryIdErrSrcInvalid: WHEA_EVENT_LOG_ENTRY_ID = -2147483616i32;
pub const WheaEventLogEntryIdErrorRecord: WHEA_EVENT_LOG_ENTRY_ID = -2147483626i32;
pub const WheaEventLogEntryIdErrorRecordLimit: WHEA_EVENT_LOG_ENTRY_ID = -2147483625i32;
pub const WheaEventLogEntryIdFailedAddToDefectList: WHEA_EVENT_LOG_ENTRY_ID = -2147483569i32;
pub const WheaEventLogEntryIdGenericErrMemMap: WHEA_EVENT_LOG_ENTRY_ID = -2147483615i32;
pub const WheaEventLogEntryIdKeyNotificationFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483579i32;
pub const WheaEventLogEntryIdMcaErrorCleared: WHEA_EVENT_LOG_ENTRY_ID = -2147483631i32;
pub const WheaEventLogEntryIdMcaFoundErrorInBank: WHEA_EVENT_LOG_ENTRY_ID = -2147483633i32;
pub const WheaEventLogEntryIdMcaStuckErrorCheck: WHEA_EVENT_LOG_ENTRY_ID = -2147483632i32;
pub const WheaEventLogEntryIdMemoryAddDevice: WHEA_EVENT_LOG_ENTRY_ID = -2147483575i32;
pub const WheaEventLogEntryIdMemoryRemoveDevice: WHEA_EVENT_LOG_ENTRY_ID = -2147483574i32;
pub const WheaEventLogEntryIdMemorySummaryFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483573i32;
pub const WheaEventLogEntryIdOscCapabilities: WHEA_EVENT_LOG_ENTRY_ID = -2147483638i32;
pub const WheaEventLogEntryIdPFAMemoryOfflined: WHEA_EVENT_LOG_ENTRY_ID = -2147483642i32;
pub const WheaEventLogEntryIdPFAMemoryPolicy: WHEA_EVENT_LOG_ENTRY_ID = -2147483640i32;
pub const WheaEventLogEntryIdPFAMemoryRemoveMonitor: WHEA_EVENT_LOG_ENTRY_ID = -2147483641i32;
pub const WheaEventLogEntryIdPcieAddDevice: WHEA_EVENT_LOG_ENTRY_ID = -2147483577i32;
pub const WheaEventLogEntryIdPcieConfigInfo: WHEA_EVENT_LOG_ENTRY_ID = -2147483591i32;
pub const WheaEventLogEntryIdPcieDpcError: WHEA_EVENT_LOG_ENTRY_ID = -2147483572i32;
pub const WheaEventLogEntryIdPcieOverrideInfo: WHEA_EVENT_LOG_ENTRY_ID = -2147483593i32;
pub const WheaEventLogEntryIdPcieRemoveDevice: WHEA_EVENT_LOG_ENTRY_ID = -2147483578i32;
pub const WheaEventLogEntryIdPcieSpuriousErrSource: WHEA_EVENT_LOG_ENTRY_ID = -2147483576i32;
pub const WheaEventLogEntryIdPcieSummaryFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483584i32;
pub const WheaEventLogEntryIdProcessEINJ: WHEA_EVENT_LOG_ENTRY_ID = -2147483629i32;
pub const WheaEventLogEntryIdProcessHEST: WHEA_EVENT_LOG_ENTRY_ID = -2147483628i32;
pub const WheaEventLogEntryIdPshedCallbackCollision: WHEA_EVENT_LOG_ENTRY_ID = -2147483614i32;
pub const WheaEventLogEntryIdPshedInjectError: WHEA_EVENT_LOG_ENTRY_ID = -2147483639i32;
pub const WheaEventLogEntryIdPshedPiTraceLog: WHEA_EVENT_LOG_ENTRY_ID = -2147221488i32;
pub const WheaEventLogEntryIdPshedPluginInitFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483570i32;
pub const WheaEventLogEntryIdPshedPluginLoad: WHEA_EVENT_LOG_ENTRY_ID = -2147483612i32;
pub const WheaEventLogEntryIdPshedPluginRegister: WHEA_EVENT_LOG_ENTRY_ID = -2147483637i32;
pub const WheaEventLogEntryIdPshedPluginSupported: WHEA_EVENT_LOG_ENTRY_ID = -2147483610i32;
pub const WheaEventLogEntryIdPshedPluginUnload: WHEA_EVENT_LOG_ENTRY_ID = -2147483611i32;
pub const WheaEventLogEntryIdReadPcieOverridesErr: WHEA_EVENT_LOG_ENTRY_ID = -2147483592i32;
pub const WheaEventLogEntryIdRowFailure: WHEA_EVENT_LOG_ENTRY_ID = -2147483561i32;
pub const WheaEventLogEntryIdSELBugCheckInfo: WHEA_EVENT_LOG_ENTRY_ID = -2147483601i32;
pub const WheaEventLogEntryIdSELBugCheckProgress: WHEA_EVENT_LOG_ENTRY_ID = -2147483613i32;
pub const WheaEventLogEntryIdSELBugCheckRecovery: WHEA_EVENT_LOG_ENTRY_ID = -2147483606i32;
pub const WheaEventLogEntryIdSrasTableEntries: WHEA_EVENT_LOG_ENTRY_ID = -2147483562i32;
pub const WheaEventLogEntryIdSrasTableError: WHEA_EVENT_LOG_ENTRY_ID = -2147483563i32;
pub const WheaEventLogEntryIdSrasTableNotFound: WHEA_EVENT_LOG_ENTRY_ID = -2147483564i32;
pub const WheaEventLogEntryIdStartedReportHwError: WHEA_EVENT_LOG_ENTRY_ID = -2147483643i32;
pub const WheaEventLogEntryIdThrottleAddErrSrcFailed: WHEA_EVENT_LOG_ENTRY_ID = -2147483582i32;
pub const WheaEventLogEntryIdThrottleRegCorrupt: WHEA_EVENT_LOG_ENTRY_ID = -2147483583i32;
pub const WheaEventLogEntryIdThrottleRegDataIgnored: WHEA_EVENT_LOG_ENTRY_ID = -2147483581i32;
pub const WheaEventLogEntryIdWheaHeartbeat: WHEA_EVENT_LOG_ENTRY_ID = -2147483603i32;
pub const WheaEventLogEntryIdWheaInit: WHEA_EVENT_LOG_ENTRY_ID = -2147483646i32;
pub const WheaEventLogEntryIdWorkQueueItem: WHEA_EVENT_LOG_ENTRY_ID = -2147483635i32;
pub const WheaEventLogEntryIdeDpcEnabled: WHEA_EVENT_LOG_ENTRY_ID = -2147483599i32;
pub const WheaEventLogEntryPageOfflineDone: WHEA_EVENT_LOG_ENTRY_ID = -2147483598i32;
pub const WheaEventLogEntryPageOfflinePendMax: WHEA_EVENT_LOG_ENTRY_ID = -2147483597i32;
pub const WheaEventLogEntrySrarDetail: WHEA_EVENT_LOG_ENTRY_ID = -2147483595i32;
pub const WheaEventLogEntryTypeError: WHEA_EVENT_LOG_ENTRY_TYPE = 2i32;
pub const WheaEventLogEntryTypeInformational: WHEA_EVENT_LOG_ENTRY_TYPE = 0i32;
pub const WheaEventLogEntryTypeWarning: WHEA_EVENT_LOG_ENTRY_TYPE = 1i32;
pub const WheaMemoryThrottle: WHEA_THROTTLE_TYPE = 1i32;
pub const WheaPciExpressDownstreamSwitchPort: WHEA_PCIEXPRESS_DEVICE_TYPE = 6i32;
pub const WheaPciExpressEndpoint: WHEA_PCIEXPRESS_DEVICE_TYPE = 0i32;
pub const WheaPciExpressLegacyEndpoint: WHEA_PCIEXPRESS_DEVICE_TYPE = 1i32;
pub const WheaPciExpressRootComplexEventCollector: WHEA_PCIEXPRESS_DEVICE_TYPE = 10i32;
pub const WheaPciExpressRootComplexIntegratedEndpoint: WHEA_PCIEXPRESS_DEVICE_TYPE = 9i32;
pub const WheaPciExpressRootPort: WHEA_PCIEXPRESS_DEVICE_TYPE = 4i32;
pub const WheaPciExpressToPciXBridge: WHEA_PCIEXPRESS_DEVICE_TYPE = 7i32;
pub const WheaPciExpressUpstreamSwitchPort: WHEA_PCIEXPRESS_DEVICE_TYPE = 5i32;
pub const WheaPciREcoveryStatusUnknown: WHEA_PCI_RECOVERY_STATUS = 0i32;
pub const WheaPciRecoverySignalAer: WHEA_PCI_RECOVERY_SIGNAL = 1i32;
pub const WheaPciRecoverySignalDpc: WHEA_PCI_RECOVERY_SIGNAL = 2i32;
pub const WheaPciRecoverySignalUnknown: WHEA_PCI_RECOVERY_SIGNAL = 0i32;
pub const WheaPciRecoveryStatusBusNotFound: WHEA_PCI_RECOVERY_STATUS = 6i32;
pub const WheaPciRecoveryStatusComplexTree: WHEA_PCI_RECOVERY_STATUS = 5i32;
pub const WheaPciRecoveryStatusLinkDisableTimeout: WHEA_PCI_RECOVERY_STATUS = 2i32;
pub const WheaPciRecoveryStatusLinkEnableTimeout: WHEA_PCI_RECOVERY_STATUS = 3i32;
pub const WheaPciRecoveryStatusNoError: WHEA_PCI_RECOVERY_STATUS = 1i32;
pub const WheaPciRecoveryStatusRpBusyTimeout: WHEA_PCI_RECOVERY_STATUS = 4i32;
pub const WheaPciXToExpressBridge: WHEA_PCIEXPRESS_DEVICE_TYPE = 8i32;
pub const WheaPcieThrottle: WHEA_THROTTLE_TYPE = 0i32;
pub const WheaPfaRemoveCapacity: WHEA_PFA_REMOVE_TRIGGER = 3i32;
pub const WheaPfaRemoveErrorThreshold: WHEA_PFA_REMOVE_TRIGGER = 1i32;
pub const WheaPfaRemoveTimeout: WHEA_PFA_REMOVE_TRIGGER = 2i32;
pub const WheaRawDataFormatAMD64MCA: WHEA_RAW_DATA_FORMAT = 3i32;
pub const WheaRawDataFormatGeneric: WHEA_RAW_DATA_FORMAT = 9i32;
pub const WheaRawDataFormatIA32MCA: WHEA_RAW_DATA_FORMAT = 1i32;
pub const WheaRawDataFormatIPFSalRecord: WHEA_RAW_DATA_FORMAT = 0i32;
pub const WheaRawDataFormatIntel64MCA: WHEA_RAW_DATA_FORMAT = 2i32;
pub const WheaRawDataFormatMax: WHEA_RAW_DATA_FORMAT = 10i32;
pub const WheaRawDataFormatMemory: WHEA_RAW_DATA_FORMAT = 4i32;
pub const WheaRawDataFormatNMIPort: WHEA_RAW_DATA_FORMAT = 6i32;
pub const WheaRawDataFormatPCIExpress: WHEA_RAW_DATA_FORMAT = 5i32;
pub const WheaRawDataFormatPCIXBus: WHEA_RAW_DATA_FORMAT = 7i32;
pub const WheaRawDataFormatPCIXDevice: WHEA_RAW_DATA_FORMAT = 8i32;
pub const WheaRecoveryContextErrorTypeMax: WHEA_RECOVERY_CONTEXT_ERROR_TYPE = 3i32;
pub const WheaRecoveryContextErrorTypeMemory: WHEA_RECOVERY_CONTEXT_ERROR_TYPE = 1i32;
pub const WheaRecoveryContextErrorTypePmem: WHEA_RECOVERY_CONTEXT_ERROR_TYPE = 2i32;
pub const WheaRecoveryFailureReasonFarNotValid: WHEA_RECOVERY_FAILURE_REASON = 17i32;
pub const WheaRecoveryFailureReasonHighIrql: WHEA_RECOVERY_FAILURE_REASON = 10i32;
pub const WheaRecoveryFailureReasonInsufficientAltContextWrappers: WHEA_RECOVERY_FAILURE_REASON = 11i32;
pub const WheaRecoveryFailureReasonInterruptsDisabled: WHEA_RECOVERY_FAILURE_REASON = 12i32;
pub const WheaRecoveryFailureReasonInvalidAddressMode: WHEA_RECOVERY_FAILURE_REASON = 9i32;
pub const WheaRecoveryFailureReasonKernelCouldNotMarkMemoryBad: WHEA_RECOVERY_FAILURE_REASON = 1i32;
pub const WheaRecoveryFailureReasonKernelMarkMemoryBadTimedOut: WHEA_RECOVERY_FAILURE_REASON = 2i32;
pub const WheaRecoveryFailureReasonKernelWillPageFaultBCAtCurrentIrql: WHEA_RECOVERY_FAILURE_REASON = 16i32;
pub const WheaRecoveryFailureReasonMax: WHEA_RECOVERY_FAILURE_REASON = 18i32;
pub const WheaRecoveryFailureReasonMiscOrAddrNotValid: WHEA_RECOVERY_FAILURE_REASON = 8i32;
pub const WheaRecoveryFailureReasonNoRecoveryContext: WHEA_RECOVERY_FAILURE_REASON = 3i32;
pub const WheaRecoveryFailureReasonNotContinuable: WHEA_RECOVERY_FAILURE_REASON = 4i32;
pub const WheaRecoveryFailureReasonNotSupported: WHEA_RECOVERY_FAILURE_REASON = 7i32;
pub const WheaRecoveryFailureReasonOverflow: WHEA_RECOVERY_FAILURE_REASON = 6i32;
pub const WheaRecoveryFailureReasonPcc: WHEA_RECOVERY_FAILURE_REASON = 5i32;
pub const WheaRecoveryFailureReasonStackOverflow: WHEA_RECOVERY_FAILURE_REASON = 14i32;
pub const WheaRecoveryFailureReasonSwapBusy: WHEA_RECOVERY_FAILURE_REASON = 13i32;
pub const WheaRecoveryFailureReasonUnexpectedFailure: WHEA_RECOVERY_FAILURE_REASON = 15i32;
pub const WheaRecoveryTypeActionOptional: WHEA_RECOVERY_TYPE = 2i32;
pub const WheaRecoveryTypeActionRequired: WHEA_RECOVERY_TYPE = 1i32;
pub const WheaRecoveryTypeMax: WHEA_RECOVERY_TYPE = 3i32;
pub const WheapDpcErrBusNotFound: WHEAP_DPC_ERROR_EVENT_TYPE = 1i32;
pub const WheapDpcErrDeviceIdBad: WHEAP_DPC_ERROR_EVENT_TYPE = 3i32;
pub const WheapDpcErrDpcedSubtree: WHEAP_DPC_ERROR_EVENT_TYPE = 2i32;
pub const WheapDpcErrNoChildren: WHEAP_DPC_ERROR_EVENT_TYPE = 5i32;
pub const WheapDpcErrNoErr: WHEAP_DPC_ERROR_EVENT_TYPE = 0i32;
pub const WheapDpcErrResetFailed: WHEAP_DPC_ERROR_EVENT_TYPE = 4i32;
pub const WheapPfaOfflinePredictiveFailure: WHEAP_PFA_OFFLINE_DECISION_TYPE = 1i32;
pub const WheapPfaOfflineUncorrectedError: WHEAP_PFA_OFFLINE_DECISION_TYPE = 2i32;
pub const Width16Bits: DMA_WIDTH = 1i32;
pub const Width32Bits: DMA_WIDTH = 2i32;
pub const Width64Bits: DMA_WIDTH = 3i32;
pub const Width8Bits: DMA_WIDTH = 0i32;
pub const WidthNoWrap: DMA_WIDTH = 4i32;
pub const WormController: CONFIGURATION_TYPE = 16i32;
pub const WrAlertByThreadId: KWAIT_REASON = 37i32;
pub const WrCalloutStack: KWAIT_REASON = 25i32;
pub const WrCpuRateControl: KWAIT_REASON = 24i32;
pub const WrDeferredPreempt: KWAIT_REASON = 38i32;
pub const WrDelayExecution: KWAIT_REASON = 11i32;
pub const WrDispatchInt: KWAIT_REASON = 31i32;
pub const WrExecutive: KWAIT_REASON = 7i32;
pub const WrFastMutex: KWAIT_REASON = 34i32;
pub const WrFreePage: KWAIT_REASON = 8i32;
pub const WrGuardedMutex: KWAIT_REASON = 35i32;
pub const WrIoRing: KWAIT_REASON = 40i32;
pub const WrKernel: KWAIT_REASON = 26i32;
pub const WrKeyedEvent: KWAIT_REASON = 21i32;
pub const WrLpcReceive: KWAIT_REASON = 16i32;
pub const WrLpcReply: KWAIT_REASON = 17i32;
pub const WrMdlCache: KWAIT_REASON = 41i32;
pub const WrMutex: KWAIT_REASON = 29i32;
pub const WrPageIn: KWAIT_REASON = 9i32;
pub const WrPageOut: KWAIT_REASON = 19i32;
pub const WrPhysicalFault: KWAIT_REASON = 39i32;
pub const WrPoolAllocation: KWAIT_REASON = 10i32;
pub const WrPreempted: KWAIT_REASON = 32i32;
pub const WrProcessInSwap: KWAIT_REASON = 23i32;
pub const WrPushLock: KWAIT_REASON = 28i32;
pub const WrQuantumEnd: KWAIT_REASON = 30i32;
pub const WrQueue: KWAIT_REASON = 15i32;
pub const WrRendezvous: KWAIT_REASON = 20i32;
pub const WrResource: KWAIT_REASON = 27i32;
pub const WrRundown: KWAIT_REASON = 36i32;
pub const WrSpare0: KWAIT_REASON = 14i32;
pub const WrSuspended: KWAIT_REASON = 12i32;
pub const WrTerminated: KWAIT_REASON = 22i32;
pub const WrUserRequest: KWAIT_REASON = 13i32;
pub const WrVirtualMemory: KWAIT_REASON = 18i32;
pub const WrYieldExecution: KWAIT_REASON = 33i32;
pub const WriteAccess: IO_ACCESS_TYPE = 1i32;
pub const XPF_BUS_CHECK_ADDRESS_IO: u32 = 2u32;
pub const XPF_BUS_CHECK_ADDRESS_MEMORY: u32 = 0u32;
pub const XPF_BUS_CHECK_ADDRESS_OTHER: u32 = 3u32;
pub const XPF_BUS_CHECK_ADDRESS_RESERVED: u32 = 1u32;
pub const XPF_BUS_CHECK_OPERATION_DATAREAD: u32 = 3u32;
pub const XPF_BUS_CHECK_OPERATION_DATAWRITE: u32 = 4u32;
pub const XPF_BUS_CHECK_OPERATION_GENERIC: u32 = 0u32;
pub const XPF_BUS_CHECK_OPERATION_GENREAD: u32 = 1u32;
pub const XPF_BUS_CHECK_OPERATION_GENWRITE: u32 = 2u32;
pub const XPF_BUS_CHECK_OPERATION_INSTRUCTIONFETCH: u32 = 5u32;
pub const XPF_BUS_CHECK_OPERATION_PREFETCH: u32 = 6u32;
pub const XPF_BUS_CHECK_PARTICIPATION_GENERIC: u32 = 3u32;
pub const XPF_BUS_CHECK_PARTICIPATION_PROCOBSERVED: u32 = 2u32;
pub const XPF_BUS_CHECK_PARTICIPATION_PROCORIGINATED: u32 = 0u32;
pub const XPF_BUS_CHECK_PARTICIPATION_PROCRESPONDED: u32 = 1u32;
pub const XPF_BUS_CHECK_TRANSACTIONTYPE_DATAACCESS: u32 = 1u32;
pub const XPF_BUS_CHECK_TRANSACTIONTYPE_GENERIC: u32 = 2u32;
pub const XPF_BUS_CHECK_TRANSACTIONTYPE_INSTRUCTION: u32 = 0u32;
pub const XPF_CACHE_CHECK_OPERATION_DATAREAD: u32 = 3u32;
pub const XPF_CACHE_CHECK_OPERATION_DATAWRITE: u32 = 4u32;
pub const XPF_CACHE_CHECK_OPERATION_EVICTION: u32 = 7u32;
pub const XPF_CACHE_CHECK_OPERATION_GENERIC: u32 = 0u32;
pub const XPF_CACHE_CHECK_OPERATION_GENREAD: u32 = 1u32;
pub const XPF_CACHE_CHECK_OPERATION_GENWRITE: u32 = 2u32;
pub const XPF_CACHE_CHECK_OPERATION_INSTRUCTIONFETCH: u32 = 5u32;
pub const XPF_CACHE_CHECK_OPERATION_PREFETCH: u32 = 6u32;
pub const XPF_CACHE_CHECK_OPERATION_SNOOP: u32 = 8u32;
pub const XPF_CACHE_CHECK_TRANSACTIONTYPE_DATAACCESS: u32 = 1u32;
pub const XPF_CACHE_CHECK_TRANSACTIONTYPE_GENERIC: u32 = 2u32;
pub const XPF_CACHE_CHECK_TRANSACTIONTYPE_INSTRUCTION: u32 = 0u32;
pub const XPF_CONTEXT_INFO_32BITCONTEXT: u32 = 2u32;
pub const XPF_CONTEXT_INFO_32BITDEBUGREGS: u32 = 5u32;
pub const XPF_CONTEXT_INFO_64BITCONTEXT: u32 = 3u32;
pub const XPF_CONTEXT_INFO_64BITDEBUGREGS: u32 = 6u32;
pub const XPF_CONTEXT_INFO_FXSAVE: u32 = 4u32;
pub const XPF_CONTEXT_INFO_MMREGISTERS: u32 = 7u32;
pub const XPF_CONTEXT_INFO_MSRREGISTERS: u32 = 1u32;
pub const XPF_CONTEXT_INFO_UNCLASSIFIEDDATA: u32 = 0u32;
pub const XPF_MCA_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x8a1e1d01_42f9_4557_9c33_565e5cc3f7e8);
pub const XPF_MS_CHECK_ERRORTYPE_EXTERNAL: u32 = 3u32;
pub const XPF_MS_CHECK_ERRORTYPE_FRC: u32 = 4u32;
pub const XPF_MS_CHECK_ERRORTYPE_INTERNALUNCLASSIFIED: u32 = 5u32;
pub const XPF_MS_CHECK_ERRORTYPE_MCROMPARITY: u32 = 2u32;
pub const XPF_MS_CHECK_ERRORTYPE_NOERROR: u32 = 0u32;
pub const XPF_MS_CHECK_ERRORTYPE_UNCLASSIFIED: u32 = 1u32;
pub const XPF_PROCESSOR_ERROR_SECTION_GUID: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xdc3ea0b0_a144_4797_b95b_53fa242b6e1d);
pub const XPF_TLB_CHECK_OPERATION_DATAREAD: u32 = 3u32;
pub const XPF_TLB_CHECK_OPERATION_DATAWRITE: u32 = 4u32;
pub const XPF_TLB_CHECK_OPERATION_GENERIC: u32 = 0u32;
pub const XPF_TLB_CHECK_OPERATION_GENREAD: u32 = 1u32;
pub const XPF_TLB_CHECK_OPERATION_GENWRITE: u32 = 2u32;
pub const XPF_TLB_CHECK_OPERATION_INSTRUCTIONFETCH: u32 = 5u32;
pub const XPF_TLB_CHECK_OPERATION_PREFETCH: u32 = 6u32;
pub const XPF_TLB_CHECK_TRANSACTIONTYPE_DATAACCESS: u32 = 1u32;
pub const XPF_TLB_CHECK_TRANSACTIONTYPE_GENERIC: u32 = 2u32;
pub const XPF_TLB_CHECK_TRANSACTIONTYPE_INSTRUCTION: u32 = 0u32;
pub const _STRSAFE_USE_SECURE_CRT: u32 = 0u32;
pub const __guid_type: u32 = 8192u32;
pub const __multiString_type: u32 = 16384u32;
pub const __string_type: u32 = 4096u32;
pub type ALTERNATIVE_ARCHITECTURE_TYPE = i32;
pub type ARBITER_ACTION = i32;
pub type ARBITER_REQUEST_SOURCE = i32;
pub type ARBITER_RESULT = i32;
pub type BDCB_CALLBACK_TYPE = i32;
pub type BDCB_CLASSIFICATION = i32;
pub type BDCB_STATUS_UPDATE_TYPE = i32;
pub type BOUND_CALLBACK_STATUS = i32;
pub type BUS_DATA_TYPE = i32;
pub type BUS_QUERY_ID_TYPE = i32;
pub type CM_SHARE_DISPOSITION = i32;
pub type CONFIGURATION_TYPE = i32;
pub type CREATE_FILE_TYPE = i32;
pub type D3COLD_LAST_TRANSITION_STATUS = i32;
pub type DEVICE_DIRECTORY_TYPE = i32;
pub type DEVICE_INSTALL_STATE = i32;
pub type DEVICE_REGISTRY_PROPERTY = i32;
pub type DEVICE_RELATION_TYPE = i32;
pub type DEVICE_REMOVAL_POLICY = i32;
pub type DEVICE_RESET_TYPE = i32;
pub type DEVICE_TEXT_TYPE = i32;
pub type DEVICE_USAGE_NOTIFICATION_TYPE = i32;
pub type DEVICE_WAKE_DEPTH = i32;
pub type DIRECTORY_NOTIFY_INFORMATION_CLASS = i32;
pub type DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_ACCESS_TYPE = i32;
pub type DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_TYPE = i32;
pub type DMA_COMPLETION_STATUS = i32;
pub type DMA_SPEED = i32;
pub type DMA_WIDTH = i32;
pub type DOMAIN_CONFIGURATION_ARCH = i32;
pub type DRIVER_DIRECTORY_TYPE = i32;
pub type DRIVER_REGKEY_TYPE = i32;
pub type DRIVER_RUNTIME_INIT_FLAGS = i32;
pub type EXTENDED_AGP_REGISTER = i32;
pub type EX_POOL_PRIORITY = i32;
pub type FAULT_INFORMATION_ARCH = i32;
pub type FAULT_INFORMATION_ARM64_TYPE = i32;
pub type HAL_APIC_DESTINATION_MODE = i32;
pub type HAL_DISPLAY_BIOS_INFORMATION = i32;
pub type HAL_DMA_CRASH_DUMP_REGISTER_TYPE = i32;
pub type HAL_QUERY_INFORMATION_CLASS = i32;
pub type HAL_SET_INFORMATION_CLASS = i32;
pub type HARDWARE_COUNTER_TYPE = i32;
pub type INTEL_CACHE_TYPE = i32;
pub type INTERFACE_TYPE = i32;
pub type INTERLOCKED_RESULT = i32;
pub type IOMMU_DEVICE_CREATION_CONFIGURATION_TYPE = i32;
pub type IOMMU_DMA_DOMAIN_TYPE = i32;
pub type IOMMU_DMA_LOGICAL_ALLOCATOR_TYPE = i32;
pub type IOMMU_MAP_PHYSICAL_ADDRESS_TYPE = i32;
pub type IO_ACCESS_MODE = i32;
pub type IO_ACCESS_TYPE = i32;
pub type IO_ALLOCATION_ACTION = i32;
pub type IO_COMPLETION_ROUTINE_RESULT = i32;
pub type IO_CONTAINER_INFORMATION_CLASS = i32;
pub type IO_CONTAINER_NOTIFICATION_CLASS = i32;
pub type IO_NOTIFICATION_EVENT_CATEGORY = i32;
pub type IO_PAGING_PRIORITY = i32;
pub type IO_QUERY_DEVICE_DATA_FORMAT = i32;
pub type IO_SESSION_EVENT = i32;
pub type IO_SESSION_STATE = i32;
pub type IRQ_DEVICE_POLICY = i32;
pub type IRQ_GROUP_POLICY = i32;
pub type IRQ_PRIORITY = i32;
pub type KBUGCHECK_BUFFER_DUMP_STATE = i32;
pub type KBUGCHECK_CALLBACK_REASON = i32;
pub type KBUGCHECK_DUMP_IO_TYPE = i32;
pub type KDPC_IMPORTANCE = i32;
pub type KD_CALLBACK_ACTION = i32;
pub type KD_NAMESPACE_ENUM = i32;
pub type KD_OPTION = i32;
pub type KE_PROCESSOR_CHANGE_NOTIFY_STATE = i32;
pub type KINTERRUPT_MODE = i32;
pub type KINTERRUPT_POLARITY = i32;
pub type KPROFILE_SOURCE = i32;
pub type KWAIT_REASON = i32;
pub type LOCK_OPERATION = i32;
pub type MCA_EXCEPTION_TYPE = i32;
pub type MEMORY_CACHING_TYPE = i32;
pub type MEMORY_CACHING_TYPE_ORIG = i32;
pub type MEM_DEDICATED_ATTRIBUTE_TYPE = i32;
pub type MEM_SECTION_EXTENDED_PARAMETER_TYPE = i32;
pub type MM_MDL_PAGE_CONTENTS_STATE = i32;
pub type MM_PAGE_PRIORITY = i32;
pub type MM_ROTATE_DIRECTION = i32;
pub type MM_SYSTEMSIZE = i32;
pub type MODE = i32;
pub type NPEM_CONTROL_STANDARD_CONTROL_BIT = i32;
pub type OB_PREOP_CALLBACK_STATUS = i32;
pub type PARTITION_INFORMATION_CLASS = i32;
pub type PCI_ACS_BIT = i32;
pub type PCI_BUS_WIDTH = i32;
pub type PCI_DEVICE_D3COLD_STATE_REASON = i32;
pub type PCI_EXPRESS_ASPM_CONTROL = i32;
pub type PCI_EXPRESS_ASPM_SUPPORT = i32;
pub type PCI_EXPRESS_CARD_PRESENCE = i32;
pub type PCI_EXPRESS_DEVICE_TYPE = i32;
pub type PCI_EXPRESS_INDICATOR_STATE = i32;
pub type PCI_EXPRESS_L0s_EXIT_LATENCY = i32;
pub type PCI_EXPRESS_L1_EXIT_LATENCY = i32;
pub type PCI_EXPRESS_LINK_SUBSTATE = i32;
pub type PCI_EXPRESS_MAX_PAYLOAD_SIZE = i32;
pub type PCI_EXPRESS_MRL_STATE = i32;
pub type PCI_EXPRESS_POWER_STATE = i32;
pub type PCI_EXPRESS_RCB = i32;
pub type PCI_HARDWARE_INTERFACE = i32;
pub type PCW_CALLBACK_TYPE = i32;
pub type PCW_REGISTRATION_FLAGS = i32;
pub type PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE = i32;
pub type POOL_EXTENDED_PARAMETER_TYPE = i32;
pub type POWER_MONITOR_REQUEST_REASON = i32;
pub type POWER_MONITOR_REQUEST_TYPE = i32;
pub type POWER_PLATFORM_ROLE = i32;
pub type POWER_STATE_TYPE = i32;
pub type POWER_USER_PRESENCE_TYPE = i32;
pub type PO_FX_PERF_STATE_TYPE = i32;
pub type PO_FX_PERF_STATE_UNIT = i32;
pub type PO_THERMAL_REQUEST_TYPE = i32;
pub type PSCREATEPROCESSNOTIFYTYPE = i32;
pub type PSCREATETHREADNOTIFYTYPE = i32;
pub type PSHED_PI_ERR_READING_PCIE_OVERRIDES = i32;
pub type REG_NOTIFY_CLASS = i32;
pub type RESOURCE_TRANSLATION_DIRECTION = i32;
pub type RTL_GENERIC_COMPARE_RESULTS = i32;
pub type SECURITY_OPERATION_CODE = i32;
pub type SE_IMAGE_TYPE = i32;
pub type SE_IMAGE_VERIFICATION_CALLBACK_TYPE = i32;
pub type SOC_SUBSYSTEM_TYPE = i32;
pub type STATE_LOCATION_TYPE = i32;
pub type SUBSYSTEM_INFORMATION_TYPE = i32;
pub type SYSTEM_FIRMWARE_TABLE_ACTION = i32;
pub type SYSTEM_POWER_CONDITION = i32;
pub type TABLE_SEARCH_RESULT = i32;
pub type TRACE_INFORMATION_CLASS = i32;
pub type WHEAP_DPC_ERROR_EVENT_TYPE = i32;
pub type WHEAP_PFA_OFFLINE_DECISION_TYPE = i32;
pub type WHEA_BUGCHECK_RECOVERY_LOG_TYPE = i32;
pub type WHEA_CPU_VENDOR = i32;
pub type WHEA_ERROR_PACKET_DATA_FORMAT = i32;
pub type WHEA_ERROR_SEVERITY = i32;
pub type WHEA_ERROR_TYPE = i32;
pub type WHEA_EVENT_LOG_ENTRY_ID = i32;
pub type WHEA_EVENT_LOG_ENTRY_TYPE = i32;
pub type WHEA_PCIEXPRESS_DEVICE_TYPE = i32;
pub type WHEA_PCI_RECOVERY_SIGNAL = i32;
pub type WHEA_PCI_RECOVERY_STATUS = i32;
pub type WHEA_PFA_REMOVE_TRIGGER = i32;
pub type WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS = i32;
pub type WHEA_RAW_DATA_FORMAT = i32;
pub type WHEA_RECOVERY_CONTEXT_ERROR_TYPE = i32;
pub type WHEA_RECOVERY_FAILURE_REASON = i32;
pub type WHEA_RECOVERY_TYPE = i32;
pub type WHEA_THROTTLE_TYPE = i32;
pub type WORK_QUEUE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ACPI_DEBUGGING_DEVICE_IN_USE {
    pub NameSpacePathLength: u32,
    pub NameSpacePath: [u16; 1],
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct ACPI_INTERFACE_STANDARD {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub GpeConnectVector: PGPE_CONNECT_VECTOR,
    pub GpeDisconnectVector: PGPE_DISCONNECT_VECTOR,
    pub GpeEnableEvent: PGPE_ENABLE_EVENT,
    pub GpeDisableEvent: PGPE_DISABLE_EVENT,
    pub GpeClearStatus: PGPE_CLEAR_STATUS,
    pub RegisterForDeviceNotifications: PREGISTER_FOR_DEVICE_NOTIFICATIONS,
    pub UnregisterForDeviceNotifications: PUNREGISTER_FOR_DEVICE_NOTIFICATIONS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ACPI_INTERFACE_STANDARD2 {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub GpeConnectVector: PGPE_CONNECT_VECTOR2,
    pub GpeDisconnectVector: PGPE_DISCONNECT_VECTOR2,
    pub GpeEnableEvent: PGPE_ENABLE_EVENT2,
    pub GpeDisableEvent: PGPE_DISABLE_EVENT2,
    pub GpeClearStatus: PGPE_CLEAR_STATUS2,
    pub RegisterForDeviceNotifications: PREGISTER_FOR_DEVICE_NOTIFICATIONS2,
    pub UnregisterForDeviceNotifications: PUNREGISTER_FOR_DEVICE_NOTIFICATIONS2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AGP_TARGET_BUS_INTERFACE_STANDARD {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SetBusData: PGET_SET_DEVICE_DATA,
    pub GetBusData: PGET_SET_DEVICE_DATA,
    pub CapabilityID: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union AMD_L1_CACHE_INFO {
    pub Ulong: u32,
    pub Anonymous: AMD_L1_CACHE_INFO_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AMD_L1_CACHE_INFO_0 {
    pub LineSize: u8,
    pub LinesPerTag: u8,
    pub Associativity: u8,
    pub Size: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union AMD_L2_CACHE_INFO {
    pub Ulong: u32,
    pub Anonymous: AMD_L2_CACHE_INFO_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AMD_L2_CACHE_INFO_0 {
    pub LineSize: u8,
    pub _bitfield: u8,
    pub Size: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union AMD_L3_CACHE_INFO {
    pub Ulong: u32,
    pub Anonymous: AMD_L3_CACHE_INFO_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct AMD_L3_CACHE_INFO_0 {
    pub LineSize: u8,
    pub _bitfield1: u8,
    pub _bitfield2: u16,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct ARBITER_ADD_RESERVED_PARAMETERS {
    pub ReserveDevice: *mut super::super::Foundation::DEVICE_OBJECT,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct ARBITER_BOOT_ALLOCATION_PARAMETERS {
    pub ArbitrationList: *mut super::super::super::Win32::System::Kernel::LIST_ENTRY,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct ARBITER_CONFLICT_INFO {
    pub OwningObject: *mut super::super::Foundation::DEVICE_OBJECT,
    pub Start: u64,
    pub End: u64,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct ARBITER_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub ArbiterHandler: PARBITER_HANDLER,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct ARBITER_LIST_ENTRY {
    pub ListEntry: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub AlternativeCount: u32,
    pub Alternatives: *mut IO_RESOURCE_DESCRIPTOR,
    pub PhysicalDeviceObject: *mut super::super::Foundation::DEVICE_OBJECT,
    pub RequestSource: ARBITER_REQUEST_SOURCE,
    pub Flags: u32,
    pub WorkSpace: isize,
    pub InterfaceType: INTERFACE_TYPE,
    pub SlotNumber: u32,
    pub BusNumber: u32,
    pub Assignment: *mut CM_PARTIAL_RESOURCE_DESCRIPTOR,
    pub SelectedAlternative: *mut IO_RESOURCE_DESCRIPTOR,
    pub Result: ARBITER_RESULT,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct ARBITER_PARAMETERS {
    pub Parameters: ARBITER_PARAMETERS_0,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union ARBITER_PARAMETERS_0 {
    pub TestAllocation: ARBITER_TEST_ALLOCATION_PARAMETERS,
    pub RetestAllocation: ARBITER_RETEST_ALLOCATION_PARAMETERS,
    pub BootAllocation: ARBITER_BOOT_ALLOCATION_PARAMETERS,
    pub QueryAllocatedResources: ARBITER_QUERY_ALLOCATED_RESOURCES_PARAMETERS,
    pub QueryConflict: ARBITER_QUERY_CONFLICT_PARAMETERS,
    pub QueryArbitrate: ARBITER_QUERY_ARBITRATE_PARAMETERS,
    pub AddReserved: ARBITER_ADD_RESERVED_PARAMETERS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ARBITER_QUERY_ALLOCATED_RESOURCES_PARAMETERS {
    pub AllocatedResources: *mut *mut CM_PARTIAL_RESOURCE_LIST,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct ARBITER_QUERY_ARBITRATE_PARAMETERS {
    pub ArbitrationList: *mut super::super::super::Win32::System::Kernel::LIST_ENTRY,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct ARBITER_QUERY_CONFLICT_PARAMETERS {
    pub PhysicalDeviceObject: *mut super::super::Foundation::DEVICE_OBJECT,
    pub ConflictingResource: *mut IO_RESOURCE_DESCRIPTOR,
    pub ConflictCount: *mut u32,
    pub Conflicts: *mut *mut ARBITER_CONFLICT_INFO,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct ARBITER_RETEST_ALLOCATION_PARAMETERS {
    pub ArbitrationList: *mut super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub AllocateFromCount: u32,
    pub AllocateFrom: *mut CM_PARTIAL_RESOURCE_DESCRIPTOR,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct ARBITER_TEST_ALLOCATION_PARAMETERS {
    pub ArbitrationList: *mut super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub AllocateFromCount: u32,
    pub AllocateFrom: *mut CM_PARTIAL_RESOURCE_DESCRIPTOR,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct ARM64_NT_CONTEXT {
    pub ContextFlags: u32,
    pub Cpsr: u32,
    pub Anonymous: ARM64_NT_CONTEXT_0,
    pub Sp: u64,
    pub Pc: u64,
    pub V: [super::super::super::Win32::System::Diagnostics::Debug::ARM64_NT_NEON128; 32],
    pub Fpcr: u32,
    pub Fpsr: u32,
    pub Bcr: [u32; 8],
    pub Bvr: [u64; 8],
    pub Wcr: [u32; 2],
    pub Wvr: [u64; 2],
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub union ARM64_NT_CONTEXT_0 {
    pub Anonymous: ARM64_NT_CONTEXT_0_0,
    pub X: [u64; 31],
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct ARM64_NT_CONTEXT_0_0 {
    pub X0: u64,
    pub X1: u64,
    pub X2: u64,
    pub X3: u64,
    pub X4: u64,
    pub X5: u64,
    pub X6: u64,
    pub X7: u64,
    pub X8: u64,
    pub X9: u64,
    pub X10: u64,
    pub X11: u64,
    pub X12: u64,
    pub X13: u64,
    pub X14: u64,
    pub X15: u64,
    pub X16: u64,
    pub X17: u64,
    pub X18: u64,
    pub X19: u64,
    pub X20: u64,
    pub X21: u64,
    pub X22: u64,
    pub X23: u64,
    pub X24: u64,
    pub X25: u64,
    pub X26: u64,
    pub X27: u64,
    pub X28: u64,
    pub Fp: u64,
    pub Lr: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BDCB_IMAGE_INFORMATION {
    pub Classification: BDCB_CLASSIFICATION,
    pub ImageFlags: u32,
    pub ImageName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub RegistryPath: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub CertificatePublisher: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub CertificateIssuer: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ImageHash: *mut core::ffi::c_void,
    pub CertificateThumbprint: *mut core::ffi::c_void,
    pub ImageHashAlgorithm: u32,
    pub ThumbprintHashAlgorithm: u32,
    pub ImageHashLength: u32,
    pub CertificateThumbprintLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BDCB_STATUS_UPDATE_CONTEXT {
    pub StatusType: BDCB_STATUS_UPDATE_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BOOTDISK_INFORMATION {
    pub BootPartitionOffset: i64,
    pub SystemPartitionOffset: i64,
    pub BootDeviceSignature: u32,
    pub SystemDeviceSignature: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BOOTDISK_INFORMATION_EX {
    pub BootPartitionOffset: i64,
    pub SystemPartitionOffset: i64,
    pub BootDeviceSignature: u32,
    pub SystemDeviceSignature: u32,
    pub BootDeviceGuid: windows_sys::core::GUID,
    pub SystemDeviceGuid: windows_sys::core::GUID,
    pub BootDeviceIsGpt: super::super::super::Win32::Foundation::BOOLEAN,
    pub SystemDeviceIsGpt: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BOOTDISK_INFORMATION_LITE {
    pub NumberEntries: u32,
    pub Entries: [LOADER_PARTITION_INFORMATION_EX; 1],
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct BUS_INTERFACE_STANDARD {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub TranslateBusAddress: PTRANSLATE_BUS_ADDRESS,
    pub GetDmaAdapter: PGET_DMA_ADAPTER,
    pub SetBusData: PGET_SET_DEVICE_DATA,
    pub GetBusData: PGET_SET_DEVICE_DATA,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BUS_RESOURCE_UPDATE_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub GetUpdatedBusResource: PGET_UPDATED_BUS_RESOURCE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union BUS_SPECIFIC_RESET_FLAGS {
    pub u: BUS_SPECIFIC_RESET_FLAGS_0,
    pub AsUlonglong: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct BUS_SPECIFIC_RESET_FLAGS_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct CLFS_MGMT_CLIENT_REGISTRATION {
    pub Version: u32,
    pub AdvanceTailCallback: PCLFS_CLIENT_ADVANCE_TAIL_CALLBACK,
    pub AdvanceTailCallbackData: *mut core::ffi::c_void,
    pub LogGrowthCompleteCallback: PCLFS_CLIENT_LFF_HANDLER_COMPLETE_CALLBACK,
    pub LogGrowthCompleteCallbackData: *mut core::ffi::c_void,
    pub LogUnpinnedCallback: PCLFS_CLIENT_LOG_UNPINNED_CALLBACK,
    pub LogUnpinnedCallbackData: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct CMC_DRIVER_INFO {
    pub ExceptionCallback: PDRIVER_CMC_EXCEPTION_CALLBACK,
    pub DpcCallback: super::super::Foundation::PKDEFERRED_ROUTINE,
    pub DeviceContext: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_COMPONENT_INFORMATION {
    pub Flags: DEVICE_FLAGS,
    pub Version: u32,
    pub Key: u32,
    pub AffinityMask: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_DISK_GEOMETRY_DEVICE_DATA {
    pub BytesPerSector: u32,
    pub NumberOfCylinders: u32,
    pub SectorsPerTrack: u32,
    pub NumberOfHeads: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CM_EISA_FUNCTION_INFORMATION {
    pub CompressedId: u32,
    pub IdSlotFlags1: u8,
    pub IdSlotFlags2: u8,
    pub MinorRevision: u8,
    pub MajorRevision: u8,
    pub Selections: [u8; 26],
    pub FunctionFlags: u8,
    pub TypeString: [u8; 80],
    pub EisaMemory: [EISA_MEMORY_CONFIGURATION; 9],
    pub EisaIrq: [EISA_IRQ_CONFIGURATION; 7],
    pub EisaDma: [EISA_DMA_CONFIGURATION; 4],
    pub EisaPort: [EISA_PORT_CONFIGURATION; 20],
    pub InitializationData: [u8; 60],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CM_EISA_SLOT_INFORMATION {
    pub ReturnCode: u8,
    pub ReturnFlags: u8,
    pub MajorRevision: u8,
    pub MinorRevision: u8,
    pub Checksum: u16,
    pub NumberFunctions: u8,
    pub FunctionInformation: u8,
    pub CompressedId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_FLOPPY_DEVICE_DATA {
    pub Version: u16,
    pub Revision: u16,
    pub Size: [i8; 8],
    pub MaxDensity: u32,
    pub MountDensity: u32,
    pub StepRateHeadUnloadTime: u8,
    pub HeadLoadTime: u8,
    pub MotorOffTime: u8,
    pub SectorLengthCode: u8,
    pub SectorPerTrack: u8,
    pub ReadWriteGapLength: u8,
    pub DataTransferLength: u8,
    pub FormatGapLength: u8,
    pub FormatFillCharacter: u8,
    pub HeadSettleTime: u8,
    pub MotorSettleTime: u8,
    pub MaximumTrackValue: u8,
    pub DataTransferRate: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_FULL_RESOURCE_DESCRIPTOR {
    pub InterfaceType: INTERFACE_TYPE,
    pub BusNumber: u32,
    pub PartialResourceList: CM_PARTIAL_RESOURCE_LIST,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CM_INT13_DRIVE_PARAMETER {
    pub DriveSelect: u16,
    pub MaxCylinders: u32,
    pub SectorsPerTrack: u16,
    pub MaxHeads: u16,
    pub NumberDrives: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_KEYBOARD_DEVICE_DATA {
    pub Version: u16,
    pub Revision: u16,
    pub Type: u8,
    pub Subtype: u8,
    pub KeyboardFlags: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CM_MCA_POS_DATA {
    pub AdapterId: u16,
    pub PosData1: u8,
    pub PosData2: u8,
    pub PosData3: u8,
    pub PosData4: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_MONITOR_DEVICE_DATA {
    pub Version: u16,
    pub Revision: u16,
    pub HorizontalScreenSize: u16,
    pub VerticalScreenSize: u16,
    pub HorizontalResolution: u16,
    pub VerticalResolution: u16,
    pub HorizontalDisplayTimeLow: u16,
    pub HorizontalDisplayTime: u16,
    pub HorizontalDisplayTimeHigh: u16,
    pub HorizontalBackPorchLow: u16,
    pub HorizontalBackPorch: u16,
    pub HorizontalBackPorchHigh: u16,
    pub HorizontalFrontPorchLow: u16,
    pub HorizontalFrontPorch: u16,
    pub HorizontalFrontPorchHigh: u16,
    pub HorizontalSyncLow: u16,
    pub HorizontalSync: u16,
    pub HorizontalSyncHigh: u16,
    pub VerticalBackPorchLow: u16,
    pub VerticalBackPorch: u16,
    pub VerticalBackPorchHigh: u16,
    pub VerticalFrontPorchLow: u16,
    pub VerticalFrontPorch: u16,
    pub VerticalFrontPorchHigh: u16,
    pub VerticalSyncLow: u16,
    pub VerticalSync: u16,
    pub VerticalSyncHigh: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR {
    pub Type: u8,
    pub ShareDisposition: u8,
    pub Flags: u16,
    pub u: CM_PARTIAL_RESOURCE_DESCRIPTOR_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CM_PARTIAL_RESOURCE_DESCRIPTOR_0 {
    pub Generic: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_6,
    pub Port: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_13,
    pub Interrupt: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_7,
    pub MessageInterrupt: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_12,
    pub Memory: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_11,
    pub Dma: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_5,
    pub DmaV3: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_4,
    pub DevicePrivate: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_2,
    pub BusNumber: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_0,
    pub DeviceSpecificData: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3,
    pub Memory40: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_8,
    pub Memory48: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_9,
    pub Memory64: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_10,
    pub Connection: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_1,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_0 {
    pub Start: u32,
    pub Length: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_1 {
    pub Class: u8,
    pub Type: u8,
    pub Reserved1: u8,
    pub Reserved2: u8,
    pub IdLowPart: u32,
    pub IdHighPart: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_2 {
    pub Data: [u32; 3],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_3 {
    pub DataSize: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_4 {
    pub Channel: u32,
    pub RequestLine: u32,
    pub TransferWidth: u8,
    pub Reserved1: u8,
    pub Reserved2: u8,
    pub Reserved3: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_5 {
    pub Channel: u32,
    pub Port: u32,
    pub Reserved1: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_6 {
    pub Start: i64,
    pub Length: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_7 {
    pub Level: u32,
    pub Vector: u32,
    pub Affinity: usize,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_8 {
    pub Start: i64,
    pub Length40: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_9 {
    pub Start: i64,
    pub Length48: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_10 {
    pub Start: i64,
    pub Length64: u32,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_11 {
    pub Start: i64,
    pub Length: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_12 {
    pub Anonymous: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_12_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union CM_PARTIAL_RESOURCE_DESCRIPTOR_0_12_0 {
    pub Raw: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_12_0_0,
    pub Translated: CM_PARTIAL_RESOURCE_DESCRIPTOR_0_12_0_1,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_12_0_0 {
    pub Reserved: u16,
    pub MessageCount: u16,
    pub Vector: u32,
    pub Affinity: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_12_0_1 {
    pub Level: u32,
    pub Vector: u32,
    pub Affinity: usize,
}
#[repr(C, packed(4))]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_DESCRIPTOR_0_13 {
    pub Start: i64,
    pub Length: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_PARTIAL_RESOURCE_LIST {
    pub Version: u16,
    pub Revision: u16,
    pub Count: u32,
    pub PartialDescriptors: [CM_PARTIAL_RESOURCE_DESCRIPTOR; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_PCCARD_DEVICE_DATA {
    pub Flags: u8,
    pub ErrorCode: u8,
    pub Reserved: u16,
    pub BusData: u32,
    pub DeviceId: u32,
    pub LegacyBaseAddress: u32,
    pub IRQMap: [u8; 16],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CM_PNP_BIOS_DEVICE_NODE {
    pub Size: u16,
    pub Node: u8,
    pub ProductId: u32,
    pub DeviceType: [u8; 3],
    pub DeviceAttributes: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct CM_PNP_BIOS_INSTALLATION_CHECK {
    pub Signature: [u8; 4],
    pub Revision: u8,
    pub Length: u8,
    pub ControlField: u16,
    pub Checksum: u8,
    pub EventFlagAddress: u32,
    pub RealModeEntryOffset: u16,
    pub RealModeEntrySegment: u16,
    pub ProtectedModeEntryOffset: u16,
    pub ProtectedModeCodeBaseAddress: u32,
    pub OemDeviceId: u32,
    pub RealModeDataBaseAddress: u16,
    pub ProtectedModeDataBaseAddress: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Power")]
#[derive(Clone, Copy)]
pub struct CM_POWER_DATA {
    pub PD_Size: u32,
    pub PD_MostRecentPowerState: super::super::super::Win32::System::Power::DEVICE_POWER_STATE,
    pub PD_Capabilities: super::super::super::Win32::System::Power::DEVICE_POWER_CAPABILITIES,
    pub PD_D1Latency: u32,
    pub PD_D2Latency: u32,
    pub PD_D3Latency: u32,
    pub PD_PowerStateMapping: [super::super::super::Win32::System::Power::DEVICE_POWER_STATE; 7],
    pub PD_DeepestSystemWake: super::super::super::Win32::System::Power::SYSTEM_POWER_STATE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_RESOURCE_LIST {
    pub Count: u32,
    pub List: [CM_FULL_RESOURCE_DESCRIPTOR; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_ROM_BLOCK {
    pub Address: u32,
    pub Size: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_SCSI_DEVICE_DATA {
    pub Version: u16,
    pub Revision: u16,
    pub HostIdentifier: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_SERIAL_DEVICE_DATA {
    pub Version: u16,
    pub Revision: u16,
    pub BaudClock: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_SONIC_DEVICE_DATA {
    pub Version: u16,
    pub Revision: u16,
    pub DataConfigurationRegister: u16,
    pub EthernetAddress: [u8; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CM_VIDEO_DEVICE_DATA {
    pub Version: u16,
    pub Revision: u16,
    pub VideoClock: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CONFIGURATION_INFORMATION {
    pub DiskCount: u32,
    pub FloppyCount: u32,
    pub CdRomCount: u32,
    pub TapeCount: u32,
    pub ScsiPortCount: u32,
    pub SerialCount: u32,
    pub ParallelCount: u32,
    pub AtDiskPrimaryAddressClaimed: super::super::super::Win32::Foundation::BOOLEAN,
    pub AtDiskSecondaryAddressClaimed: super::super::super::Win32::Foundation::BOOLEAN,
    pub Version: u32,
    pub MediumChangerCount: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct CONTROLLER_OBJECT {
    pub Type: i16,
    pub Size: i16,
    pub ControllerExtension: *mut core::ffi::c_void,
    pub DeviceWaitQueue: super::super::Foundation::KDEVICE_QUEUE,
    pub Spare1: u32,
    pub Spare2: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COUNTED_REASON_CONTEXT {
    pub Version: u32,
    pub Flags: u32,
    pub Anonymous: COUNTED_REASON_CONTEXT_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union COUNTED_REASON_CONTEXT_0 {
    pub Anonymous: COUNTED_REASON_CONTEXT_0_0,
    pub SimpleString: super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COUNTED_REASON_CONTEXT_0_0 {
    pub ResourceFileName: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ResourceReasonId: u16,
    pub StringCount: u32,
    pub ReasonStrings: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct CPE_DRIVER_INFO {
    pub ExceptionCallback: PDRIVER_CPE_EXCEPTION_CALLBACK,
    pub DpcCallback: super::super::Foundation::PKDEFERRED_ROUTINE,
    pub DeviceContext: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CRASHDUMP_FUNCTIONS_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub PowerOn: PCRASHDUMP_POWER_ON,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct CREATE_USER_PROCESS_ECP_CONTEXT {
    pub Size: u16,
    pub Reserved: u16,
    pub AccessToken: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3COLD_AUX_POWER_AND_TIMING_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub RequestCorePowerRail: PD3COLD_REQUEST_CORE_POWER_RAIL,
    pub RequestAuxPower: PD3COLD_REQUEST_AUX_POWER,
    pub RequestPerstDelay: PD3COLD_REQUEST_PERST_DELAY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct D3COLD_SUPPORT_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SetD3ColdSupport: PSET_D3COLD_SUPPORT,
    pub GetIdleWakeInfo: PGET_IDLE_WAKE_INFO,
    pub GetD3ColdCapability: PGET_D3COLD_CAPABILITY,
    pub GetBusDriverD3ColdSupport: PGET_D3COLD_CAPABILITY,
    pub GetLastTransitionStatus: PGET_D3COLD_LAST_TRANSITION_STATUS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUGGING_DEVICE_IN_USE {
    pub NameSpace: KD_NAMESPACE_ENUM,
    pub StructureLength: u32,
    pub Anonymous: DEBUGGING_DEVICE_IN_USE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEBUGGING_DEVICE_IN_USE_0 {
    pub AcpiDevice: ACPI_DEBUGGING_DEVICE_IN_USE,
    pub PciDevice: PCI_DEBUGGING_DEVICE_IN_USE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUGGING_DEVICE_IN_USE_INFORMATION {
    pub DeviceCount: u32,
    pub Device: [DEBUGGING_DEVICE_IN_USE; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_DEVICE_ADDRESS {
    pub Type: u8,
    pub Valid: super::super::super::Win32::Foundation::BOOLEAN,
    pub Anonymous: DEBUG_DEVICE_ADDRESS_0,
    pub TranslatedAddress: *mut u8,
    pub Length: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEBUG_DEVICE_ADDRESS_0 {
    pub Reserved: [u8; 2],
    pub Anonymous: DEBUG_DEVICE_ADDRESS_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_DEVICE_ADDRESS_0_0 {
    pub BitWidth: u8,
    pub AccessSize: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_DEVICE_DESCRIPTOR {
    pub Bus: u32,
    pub Slot: u32,
    pub Segment: u16,
    pub VendorID: u16,
    pub DeviceID: u16,
    pub BaseClass: u8,
    pub SubClass: u8,
    pub ProgIf: u8,
    pub Anonymous: DEBUG_DEVICE_DESCRIPTOR_0,
    pub Initialized: super::super::super::Win32::Foundation::BOOLEAN,
    pub Configured: super::super::super::Win32::Foundation::BOOLEAN,
    pub BaseAddress: [DEBUG_DEVICE_ADDRESS; 6],
    pub Memory: DEBUG_MEMORY_REQUIREMENTS,
    pub Dbg2TableIndex: u32,
    pub PortType: u16,
    pub PortSubtype: u16,
    pub OemData: *mut core::ffi::c_void,
    pub OemDataLength: u32,
    pub NameSpace: KD_NAMESPACE_ENUM,
    pub NameSpacePath: windows_sys::core::PWSTR,
    pub NameSpacePathLength: u32,
    pub TransportType: u32,
    pub TransportData: DEBUG_TRANSPORT_DATA,
    pub EfiIoMmuData: DEBUG_EFI_IOMMU_DATA,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEBUG_DEVICE_DESCRIPTOR_0 {
    pub Flags: u8,
    pub Anonymous: DEBUG_DEVICE_DESCRIPTOR_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_DEVICE_DESCRIPTOR_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_EFI_IOMMU_DATA {
    pub PciIoProtocolHandle: *mut core::ffi::c_void,
    pub Mapping: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_MEMORY_REQUIREMENTS {
    pub Start: i64,
    pub MaxEnd: i64,
    pub VirtualAddress: *mut core::ffi::c_void,
    pub Length: u32,
    pub Cached: super::super::super::Win32::Foundation::BOOLEAN,
    pub Aligned: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEBUG_TRANSPORT_DATA {
    pub HwContextSize: u32,
    pub SharedVisibleDataSize: u32,
    pub UseSerialFraming: super::super::super::Win32::Foundation::BOOLEAN,
    pub ValidUSBCoreId: super::super::super::Win32::Foundation::BOOLEAN,
    pub USBCoreId: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_BUS_SPECIFIC_RESET_INFO {
    pub BusTypeGuid: windows_sys::core::GUID,
    pub ResetTypeSupported: DEVICE_BUS_SPECIFIC_RESET_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEVICE_BUS_SPECIFIC_RESET_TYPE {
    pub Pci: DEVICE_BUS_SPECIFIC_RESET_TYPE_1,
    pub Acpi: DEVICE_BUS_SPECIFIC_RESET_TYPE_0,
    pub AsULONGLONG: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_BUS_SPECIFIC_RESET_TYPE_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_BUS_SPECIFIC_RESET_TYPE_1 {
    pub _bitfield: u64,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Power")]
#[derive(Clone, Copy)]
pub struct DEVICE_CAPABILITIES {
    pub Size: u16,
    pub Version: u16,
    pub _bitfield: u32,
    pub Address: u32,
    pub UINumber: u32,
    pub DeviceState: [super::super::super::Win32::System::Power::DEVICE_POWER_STATE; 7],
    pub SystemWake: super::super::super::Win32::System::Power::SYSTEM_POWER_STATE,
    pub DeviceWake: super::super::super::Win32::System::Power::DEVICE_POWER_STATE,
    pub D1Latency: u32,
    pub D2Latency: u32,
    pub D3Latency: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_DESCRIPTION {
    pub Version: u32,
    pub Master: super::super::super::Win32::Foundation::BOOLEAN,
    pub ScatterGather: super::super::super::Win32::Foundation::BOOLEAN,
    pub DemandMode: super::super::super::Win32::Foundation::BOOLEAN,
    pub AutoInitialize: super::super::super::Win32::Foundation::BOOLEAN,
    pub Dma32BitAddresses: super::super::super::Win32::Foundation::BOOLEAN,
    pub IgnoreCount: super::super::super::Win32::Foundation::BOOLEAN,
    pub Reserved1: super::super::super::Win32::Foundation::BOOLEAN,
    pub Dma64BitAddresses: super::super::super::Win32::Foundation::BOOLEAN,
    pub BusNumber: u32,
    pub DmaChannel: u32,
    pub InterfaceType: INTERFACE_TYPE,
    pub DmaWidth: DMA_WIDTH,
    pub DmaSpeed: DMA_SPEED,
    pub MaximumLength: u32,
    pub DmaPort: u32,
    pub DmaAddressWidth: u32,
    pub DmaControllerInstance: u32,
    pub DmaRequestLine: u32,
    pub DeviceAddress: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_FAULT_CONFIGURATION {
    pub FaultHandler: PIOMMU_DEVICE_FAULT_HANDLER,
    pub FaultContext: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_FLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_INTERFACE_CHANGE_NOTIFICATION {
    pub Version: u16,
    pub Size: u16,
    pub Event: windows_sys::core::GUID,
    pub InterfaceClassGuid: windows_sys::core::GUID,
    pub SymbolicLinkName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct DEVICE_RELATIONS {
    pub Count: u32,
    pub Objects: [*mut super::super::Foundation::DEVICE_OBJECT; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_RESET_INTERFACE_STANDARD {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub DeviceReset: PDEVICE_RESET_HANDLER,
    pub SupportedResetTypes: u32,
    pub Reserved: *mut core::ffi::c_void,
    pub QueryBusSpecificResetInfo: PDEVICE_QUERY_BUS_SPECIFIC_RESET_HANDLER,
    pub DeviceBusSpecificReset: PDEVICE_BUS_SPECIFIC_RESET_HANDLER,
    pub GetDeviceResetStatus: PGET_DEVICE_RESET_STATUS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DEVICE_RESET_STATUS_FLAGS {
    pub u: DEVICE_RESET_STATUS_FLAGS_0,
    pub AsUlonglong: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DEVICE_RESET_STATUS_FLAGS_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISK_SIGNATURE {
    pub PartitionStyle: u32,
    pub Anonymous: DISK_SIGNATURE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DISK_SIGNATURE_0 {
    pub Mbr: DISK_SIGNATURE_0_1,
    pub Gpt: DISK_SIGNATURE_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISK_SIGNATURE_0_0 {
    pub DiskId: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DISK_SIGNATURE_0_1 {
    pub Signature: u32,
    pub CheckSum: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct DMA_ADAPTER {
    pub Version: u16,
    pub Size: u16,
    pub DmaOperations: *mut DMA_OPERATIONS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_ADAPTER_INFO {
    pub Version: u32,
    pub Anonymous: DMA_ADAPTER_INFO_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DMA_ADAPTER_INFO_0 {
    pub V1: DMA_ADAPTER_INFO_V1,
    pub Crashdump: DMA_ADAPTER_INFO_CRASHDUMP,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_ADAPTER_INFO_CRASHDUMP {
    pub DeviceDescription: DEVICE_DESCRIPTION,
    pub DeviceIdSize: usize,
    pub DeviceId: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_ADAPTER_INFO_V1 {
    pub ReadDmaCounterAvailable: u32,
    pub ScatterGatherLimit: u32,
    pub DmaAddressWidth: u32,
    pub Flags: u32,
    pub MinimumTransferUnit: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION {
    pub ConfigType: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_TYPE,
    pub Anonymous: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_0 {
    pub LogicalAddressLimits: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_0_0,
    pub SubSection: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_0_1,
    pub HardwareAccessType: DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_ACCESS_TYPE,
    pub Reserved: [u64; 4],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_0_0 {
    pub MinimumAddress: i64,
    pub MaximumAddress: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION_0_1 {
    pub Offset: u64,
    pub Length: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_CONFIGURATION_BYTE0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_CONFIGURATION_BYTE1 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_IOMMU_INTERFACE {
    pub Version: u32,
    pub CreateDomain: PIOMMU_DOMAIN_CREATE,
    pub DeleteDomain: PIOMMU_DOMAIN_DELETE,
    pub AttachDevice: PIOMMU_DOMAIN_ATTACH_DEVICE,
    pub DetachDevice: PIOMMU_DOMAIN_DETACH_DEVICE,
    pub FlushDomain: PIOMMU_FLUSH_DOMAIN,
    pub FlushDomainByVaList: PIOMMU_FLUSH_DOMAIN_VA_LIST,
    pub QueryInputMappings: PIOMMU_QUERY_INPUT_MAPPINGS,
    pub MapLogicalRange: PIOMMU_MAP_LOGICAL_RANGE,
    pub UnmapLogicalRange: PIOMMU_UNMAP_LOGICAL_RANGE,
    pub MapIdentityRange: PIOMMU_MAP_IDENTITY_RANGE,
    pub UnmapIdentityRange: PIOMMU_UNMAP_IDENTITY_RANGE,
    pub SetDeviceFaultReporting: PIOMMU_SET_DEVICE_FAULT_REPORTING,
    pub ConfigureDomain: PIOMMU_DOMAIN_CONFIGURE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_IOMMU_INTERFACE_EX {
    pub Size: usize,
    pub Version: u32,
    pub Anonymous: DMA_IOMMU_INTERFACE_EX_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DMA_IOMMU_INTERFACE_EX_0 {
    pub V1: DMA_IOMMU_INTERFACE_V1,
    pub V2: DMA_IOMMU_INTERFACE_V2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_IOMMU_INTERFACE_V1 {
    pub CreateDomain: PIOMMU_DOMAIN_CREATE,
    pub DeleteDomain: PIOMMU_DOMAIN_DELETE,
    pub AttachDevice: PIOMMU_DOMAIN_ATTACH_DEVICE,
    pub DetachDevice: PIOMMU_DOMAIN_DETACH_DEVICE,
    pub FlushDomain: PIOMMU_FLUSH_DOMAIN,
    pub FlushDomainByVaList: PIOMMU_FLUSH_DOMAIN_VA_LIST,
    pub QueryInputMappings: PIOMMU_QUERY_INPUT_MAPPINGS,
    pub MapLogicalRange: PIOMMU_MAP_LOGICAL_RANGE,
    pub UnmapLogicalRange: PIOMMU_UNMAP_LOGICAL_RANGE,
    pub MapIdentityRange: PIOMMU_MAP_IDENTITY_RANGE,
    pub UnmapIdentityRange: PIOMMU_UNMAP_IDENTITY_RANGE,
    pub SetDeviceFaultReporting: PIOMMU_SET_DEVICE_FAULT_REPORTING,
    pub ConfigureDomain: PIOMMU_DOMAIN_CONFIGURE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_IOMMU_INTERFACE_V2 {
    pub CreateDomainEx: PIOMMU_DOMAIN_CREATE_EX,
    pub DeleteDomain: PIOMMU_DOMAIN_DELETE,
    pub AttachDeviceEx: PIOMMU_DOMAIN_ATTACH_DEVICE_EX,
    pub DetachDeviceEx: PIOMMU_DOMAIN_DETACH_DEVICE_EX,
    pub FlushDomain: PIOMMU_FLUSH_DOMAIN,
    pub FlushDomainByVaList: PIOMMU_FLUSH_DOMAIN_VA_LIST,
    pub QueryInputMappings: PIOMMU_QUERY_INPUT_MAPPINGS,
    pub MapLogicalRangeEx: PIOMMU_MAP_LOGICAL_RANGE_EX,
    pub UnmapLogicalRange: PIOMMU_UNMAP_LOGICAL_RANGE,
    pub MapIdentityRangeEx: PIOMMU_MAP_IDENTITY_RANGE_EX,
    pub UnmapIdentityRangeEx: PIOMMU_UNMAP_IDENTITY_RANGE_EX,
    pub SetDeviceFaultReportingEx: PIOMMU_SET_DEVICE_FAULT_REPORTING_EX,
    pub ConfigureDomain: PIOMMU_DOMAIN_CONFIGURE,
    pub QueryAvailableDomainTypes: PIOMMU_DEVICE_QUERY_DOMAIN_TYPES,
    pub RegisterInterfaceStateChangeCallback: PIOMMU_REGISTER_INTERFACE_STATE_CHANGE_CALLBACK,
    pub UnregisterInterfaceStateChangeCallback: PIOMMU_UNREGISTER_INTERFACE_STATE_CHANGE_CALLBACK,
    pub ReserveLogicalAddressRange: PIOMMU_RESERVE_LOGICAL_ADDRESS_RANGE,
    pub FreeReservedLogicalAddressRange: PIOMMU_FREE_RESERVED_LOGICAL_ADDRESS_RANGE,
    pub MapReservedLogicalRange: PIOMMU_MAP_RESERVED_LOGICAL_RANGE,
    pub UnmapReservedLogicalRange: PIOMMU_UNMAP_RESERVED_LOGICAL_RANGE,
    pub CreateDevice: PIOMMU_DEVICE_CREATE,
    pub DeleteDevice: PIOMMU_DEVICE_DELETE,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct DMA_OPERATIONS {
    pub Size: u32,
    pub PutDmaAdapter: PPUT_DMA_ADAPTER,
    pub AllocateCommonBuffer: PALLOCATE_COMMON_BUFFER,
    pub FreeCommonBuffer: PFREE_COMMON_BUFFER,
    pub AllocateAdapterChannel: PALLOCATE_ADAPTER_CHANNEL,
    pub FlushAdapterBuffers: PFLUSH_ADAPTER_BUFFERS,
    pub FreeAdapterChannel: PFREE_ADAPTER_CHANNEL,
    pub FreeMapRegisters: PFREE_MAP_REGISTERS,
    pub MapTransfer: PMAP_TRANSFER,
    pub GetDmaAlignment: PGET_DMA_ALIGNMENT,
    pub ReadDmaCounter: PREAD_DMA_COUNTER,
    pub GetScatterGatherList: PGET_SCATTER_GATHER_LIST,
    pub PutScatterGatherList: PPUT_SCATTER_GATHER_LIST,
    pub CalculateScatterGatherList: PCALCULATE_SCATTER_GATHER_LIST_SIZE,
    pub BuildScatterGatherList: PBUILD_SCATTER_GATHER_LIST,
    pub BuildMdlFromScatterGatherList: PBUILD_MDL_FROM_SCATTER_GATHER_LIST,
    pub GetDmaAdapterInfo: PGET_DMA_ADAPTER_INFO,
    pub GetDmaTransferInfo: PGET_DMA_TRANSFER_INFO,
    pub InitializeDmaTransferContext: PINITIALIZE_DMA_TRANSFER_CONTEXT,
    pub AllocateCommonBufferEx: PALLOCATE_COMMON_BUFFER_EX,
    pub AllocateAdapterChannelEx: PALLOCATE_ADAPTER_CHANNEL_EX,
    pub ConfigureAdapterChannel: PCONFIGURE_ADAPTER_CHANNEL,
    pub CancelAdapterChannel: PCANCEL_ADAPTER_CHANNEL,
    pub MapTransferEx: PMAP_TRANSFER_EX,
    pub GetScatterGatherListEx: PGET_SCATTER_GATHER_LIST_EX,
    pub BuildScatterGatherListEx: PBUILD_SCATTER_GATHER_LIST_EX,
    pub FlushAdapterBuffersEx: PFLUSH_ADAPTER_BUFFERS_EX,
    pub FreeAdapterObject: PFREE_ADAPTER_OBJECT,
    pub CancelMappedTransfer: PCANCEL_MAPPED_TRANSFER,
    pub AllocateDomainCommonBuffer: PALLOCATE_DOMAIN_COMMON_BUFFER,
    pub FlushDmaBuffer: PFLUSH_DMA_BUFFER,
    pub JoinDmaDomain: PJOIN_DMA_DOMAIN,
    pub LeaveDmaDomain: PLEAVE_DMA_DOMAIN,
    pub GetDmaDomain: PGET_DMA_DOMAIN,
    pub AllocateCommonBufferWithBounds: PALLOCATE_COMMON_BUFFER_WITH_BOUNDS,
    pub AllocateCommonBufferVector: PALLOCATE_COMMON_BUFFER_VECTOR,
    pub GetCommonBufferFromVectorByIndex: PGET_COMMON_BUFFER_FROM_VECTOR_BY_INDEX,
    pub FreeCommonBufferFromVector: PFREE_COMMON_BUFFER_FROM_VECTOR,
    pub FreeCommonBufferVector: PFREE_COMMON_BUFFER_VECTOR,
    pub CreateCommonBufferFromMdl: PCREATE_COMMON_BUFFER_FROM_MDL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_TRANSFER_INFO {
    pub Version: u32,
    pub Anonymous: DMA_TRANSFER_INFO_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DMA_TRANSFER_INFO_0 {
    pub V1: DMA_TRANSFER_INFO_V1,
    pub V2: DMA_TRANSFER_INFO_V2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_TRANSFER_INFO_V1 {
    pub MapRegisterCount: u32,
    pub ScatterGatherElementCount: u32,
    pub ScatterGatherListSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DMA_TRANSFER_INFO_V2 {
    pub MapRegisterCount: u32,
    pub ScatterGatherElementCount: u32,
    pub ScatterGatherListSize: u32,
    pub LogicalPageCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOMAIN_CONFIGURATION {
    pub Type: DOMAIN_CONFIGURATION_ARCH,
    pub Anonymous: DOMAIN_CONFIGURATION_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union DOMAIN_CONFIGURATION_0 {
    pub Arm64: DOMAIN_CONFIGURATION_ARM64,
    pub X64: DOMAIN_CONFIGURATION_X64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOMAIN_CONFIGURATION_ARM64 {
    pub Ttbr0: i64,
    pub Ttbr1: i64,
    pub Mair0: u32,
    pub Mair1: u32,
    pub InputSize0: u8,
    pub InputSize1: u8,
    pub CoherentTableWalks: super::super::super::Win32::Foundation::BOOLEAN,
    pub TranslationEnabled: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOMAIN_CONFIGURATION_X64 {
    pub FirstLevelPageTableRoot: i64,
    pub TranslationEnabled: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DPC_WATCHDOG_GLOBAL_TRIAGE_BLOCK {
    pub Signature: u32,
    pub Revision: u16,
    pub Size: u16,
    pub DpcWatchdogProfileOffset: u16,
    pub DpcWatchdogProfileLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DRIVER_VERIFIER_THUNK_PAIRS {
    pub PristineRoutine: PDRIVER_VERIFIER_THUNK_ROUTINE,
    pub NewRoutine: PDRIVER_VERIFIER_THUNK_ROUTINE,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct EFI_ACPI_RAS_SIGNAL_TABLE {
    pub Header: WHEA_ACPI_HEADER,
    pub NumberRecord: u32,
    pub Entries: [SIGNAL_REG_VALUE; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EISA_DMA_CONFIGURATION {
    pub ConfigurationByte0: DMA_CONFIGURATION_BYTE0,
    pub ConfigurationByte1: DMA_CONFIGURATION_BYTE1,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EISA_IRQ_CONFIGURATION {
    pub ConfigurationByte: EISA_IRQ_DESCRIPTOR,
    pub Reserved: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EISA_IRQ_DESCRIPTOR {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct EISA_MEMORY_CONFIGURATION {
    pub ConfigurationByte: EISA_MEMORY_TYPE,
    pub DataSize: u8,
    pub AddressLowWord: u16,
    pub AddressHighByte: u8,
    pub MemorySize: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EISA_MEMORY_TYPE {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct EISA_PORT_CONFIGURATION {
    pub Configuration: EISA_PORT_DESCRIPTOR,
    pub PortAddress: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EISA_PORT_DESCRIPTOR {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ETW_TRACE_SESSION_SETTINGS {
    pub Version: u32,
    pub BufferSize: u32,
    pub MinimumBuffers: u32,
    pub MaximumBuffers: u32,
    pub LoggerMode: u32,
    pub FlushTimer: u32,
    pub FlushThreshold: u32,
    pub ClockType: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXTENDED_CREATE_INFORMATION {
    pub ExtendedCreateFlags: i64,
    pub EaBuffer: *mut core::ffi::c_void,
    pub EaLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXTENDED_CREATE_INFORMATION_32 {
    pub ExtendedCreateFlags: i64,
    pub EaBuffer: *mut core::ffi::c_void,
    pub EaLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EXT_DELETE_PARAMETERS {
    pub Version: u32,
    pub Reserved: u32,
    pub DeleteCallback: PEXT_DELETE_CALLBACK,
    pub DeleteContext: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EX_RUNDOWN_REF {
    pub Anonymous: EX_RUNDOWN_REF_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EX_RUNDOWN_REF_0 {
    pub Count: usize,
    pub Ptr: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FAULT_INFORMATION {
    pub Type: FAULT_INFORMATION_ARCH,
    pub IsStage1: super::super::super::Win32::Foundation::BOOLEAN,
    pub Anonymous: FAULT_INFORMATION_0,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union FAULT_INFORMATION_0 {
    pub Arm64: FAULT_INFORMATION_ARM64,
    pub X64: FAULT_INFORMATION_X64,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct FAULT_INFORMATION_ARM64 {
    pub DomainHandle: *mut core::ffi::c_void,
    pub FaultAddress: *mut core::ffi::c_void,
    pub PhysicalDeviceObject: *mut super::super::Foundation::DEVICE_OBJECT,
    pub InputMappingId: u32,
    pub Flags: FAULT_INFORMATION_ARM64_FLAGS,
    pub Type: FAULT_INFORMATION_ARM64_TYPE,
    pub IommuBaseAddress: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAULT_INFORMATION_ARM64_FLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAULT_INFORMATION_X64 {
    pub DomainHandle: *mut core::ffi::c_void,
    pub FaultAddress: *mut core::ffi::c_void,
    pub Flags: FAULT_INFORMATION_X64_FLAGS,
    pub Type: FAULT_INFORMATION_ARM64_TYPE,
    pub IommuBaseAddress: u64,
    pub PciSegment: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FAULT_INFORMATION_X64_FLAGS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_ATTRIBUTE_TAG_INFORMATION {
    pub FileAttributes: u32,
    pub ReparseTag: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_END_OF_FILE_INFORMATION {
    pub EndOfFile: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_FS_DEVICE_INFORMATION {
    pub DeviceType: u32,
    pub Characteristics: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_FS_FULL_SIZE_INFORMATION {
    pub TotalAllocationUnits: i64,
    pub CallerAvailableAllocationUnits: i64,
    pub ActualAvailableAllocationUnits: i64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_FS_FULL_SIZE_INFORMATION_EX {
    pub ActualTotalAllocationUnits: u64,
    pub ActualAvailableAllocationUnits: u64,
    pub ActualPoolUnavailableAllocationUnits: u64,
    pub CallerTotalAllocationUnits: u64,
    pub CallerAvailableAllocationUnits: u64,
    pub CallerPoolUnavailableAllocationUnits: u64,
    pub UsedAllocationUnits: u64,
    pub TotalReservedAllocationUnits: u64,
    pub VolumeStorageReserveAllocationUnits: u64,
    pub AvailableCommittedAllocationUnits: u64,
    pub PoolAvailableAllocationUnits: u64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_FS_LABEL_INFORMATION {
    pub VolumeLabelLength: u32,
    pub VolumeLabel: [u16; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_FS_METADATA_SIZE_INFORMATION {
    pub TotalMetadataAllocationUnits: i64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_FS_OBJECTID_INFORMATION {
    pub ObjectId: [u8; 16],
    pub ExtendedInfo: [u8; 48],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_FS_SIZE_INFORMATION {
    pub TotalAllocationUnits: i64,
    pub AvailableAllocationUnits: i64,
    pub SectorsPerAllocationUnit: u32,
    pub BytesPerSector: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_FS_VOLUME_INFORMATION {
    pub VolumeCreationTime: i64,
    pub VolumeSerialNumber: u32,
    pub VolumeLabelLength: u32,
    pub SupportsObjects: super::super::super::Win32::Foundation::BOOLEAN,
    pub VolumeLabel: [u16; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_IOSTATUSBLOCK_RANGE_INFORMATION {
    pub IoStatusBlockRange: *mut u8,
    pub Length: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_IO_COMPLETION_NOTIFICATION_INFORMATION {
    pub Flags: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct FILE_IO_PRIORITY_HINT_INFORMATION {
    pub PriorityHint: super::super::Foundation::IO_PRIORITY_HINT,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct FILE_IO_PRIORITY_HINT_INFORMATION_EX {
    pub PriorityHint: super::super::Foundation::IO_PRIORITY_HINT,
    pub BoostOutstanding: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_IS_REMOTE_DEVICE_INFORMATION {
    pub IsRemote: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_MEMORY_PARTITION_INFORMATION {
    pub OwnerPartitionHandle: usize,
    pub Flags: FILE_MEMORY_PARTITION_INFORMATION_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_MEMORY_PARTITION_INFORMATION_0 {
    pub Anonymous: FILE_MEMORY_PARTITION_INFORMATION_0_0,
    pub AllFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_MEMORY_PARTITION_INFORMATION_0_0 {
    pub NoCrossPartitionAccess: u8,
    pub Spare: [u8; 3],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_NUMA_NODE_INFORMATION {
    pub NodeNumber: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_PROCESS_IDS_USING_FILE_INFORMATION {
    pub NumberOfProcessIdsInList: u32,
    pub ProcessIdList: [usize; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_SFIO_RESERVE_INFORMATION {
    pub RequestsPerPeriod: u32,
    pub Period: u32,
    pub RetryFailures: super::super::super::Win32::Foundation::BOOLEAN,
    pub Discardable: super::super::super::Win32::Foundation::BOOLEAN,
    pub RequestSize: u32,
    pub NumOutstandingRequests: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_SFIO_VOLUME_INFORMATION {
    pub MaximumRequestsPerPeriod: u32,
    pub MinimumPeriod: u32,
    pub MinimumTransferSize: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_STANDARD_INFORMATION_EX {
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub NumberOfLinks: u32,
    pub DeletePending: super::super::super::Win32::Foundation::BOOLEAN,
    pub Directory: super::super::super::Win32::Foundation::BOOLEAN,
    pub AlternateStream: super::super::super::Win32::Foundation::BOOLEAN,
    pub MetadataAttribute: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_VALID_DATA_LENGTH_INFORMATION {
    pub ValidDataLength: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FLOATING_SAVE_AREA {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub TagWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub RegisterArea: [u8; 80],
    pub Spare0: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FPGA_CONTROL_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub BusScan: PFPGA_BUS_SCAN,
    pub ControlLink: PFPGA_CONTROL_LINK,
    pub ControlConfigSpace: PFPGA_CONTROL_CONFIG_SPACE,
    pub ControlErrorReporting: PFPGA_CONTROL_ERROR_REPORTING,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FUNCTION_LEVEL_DEVICE_RESET_PARAMETERS {
    pub Size: u32,
    pub DeviceResetCompletion: PDEVICE_RESET_COMPLETION,
    pub CompletionContext: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HAL_AMLI_BAD_IO_ADDRESS_LIST {
    pub BadAddrBegin: u32,
    pub BadAddrSize: u32,
    pub OSVersionTrigger: u32,
    pub IOHandler: PHALIOREADWRITEHANDLER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HAL_BUS_INFORMATION {
    pub BusType: INTERFACE_TYPE,
    pub ConfigurationType: BUS_DATA_TYPE,
    pub BusNumber: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct HAL_CALLBACKS {
    pub SetSystemInformation: super::super::Foundation::PCALLBACK_OBJECT,
    pub BusCheck: super::super::Foundation::PCALLBACK_OBJECT,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Ioctl", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct HAL_DISPATCH {
    pub Version: u32,
    pub HalQuerySystemInformation: pHalQuerySystemInformation,
    pub HalSetSystemInformation: pHalSetSystemInformation,
    pub HalQueryBusSlots: pHalQueryBusSlots,
    pub Spare1: u32,
    pub HalExamineMBR: pHalExamineMBR,
    pub HalIoReadPartitionTable: pHalIoReadPartitionTable,
    pub HalIoSetPartitionInformation: pHalIoSetPartitionInformation,
    pub HalIoWritePartitionTable: pHalIoWritePartitionTable,
    pub HalReferenceHandlerForBus: pHalHandlerForBus,
    pub HalReferenceBusHandler: pHalReferenceBusHandler,
    pub HalDereferenceBusHandler: pHalReferenceBusHandler,
    pub HalInitPnpDriver: pHalInitPnpDriver,
    pub HalInitPowerManagement: pHalInitPowerManagement,
    pub HalGetDmaAdapter: pHalGetDmaAdapter,
    pub HalGetInterruptTranslator: pHalGetInterruptTranslator,
    pub HalStartMirroring: pHalStartMirroring,
    pub HalEndMirroring: pHalEndMirroring,
    pub HalMirrorPhysicalMemory: pHalMirrorPhysicalMemory,
    pub HalEndOfBoot: pHalEndOfBoot,
    pub HalMirrorVerify: pHalMirrorVerify,
    pub HalGetCachedAcpiTable: pHalGetAcpiTable,
    pub HalSetPciErrorHandlerCallback: pHalSetPciErrorHandlerCallback,
    pub HalGetPrmCache: pHalGetPrmCache,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HAL_ERROR_INFO {
    pub Version: u32,
    pub InitMaxSize: u32,
    pub McaMaxSize: u32,
    pub McaPreviousEventsCount: u32,
    pub McaCorrectedEventsCount: u32,
    pub McaKernelDeliveryFails: u32,
    pub McaDriverDpcQueueFails: u32,
    pub McaReserved: u32,
    pub CmcMaxSize: u32,
    pub CmcPollingInterval: u32,
    pub CmcInterruptsCount: u32,
    pub CmcKernelDeliveryFails: u32,
    pub CmcDriverDpcQueueFails: u32,
    pub CmcGetStateFails: u32,
    pub CmcClearStateFails: u32,
    pub CmcReserved: u32,
    pub CmcLogId: u64,
    pub CpeMaxSize: u32,
    pub CpePollingInterval: u32,
    pub CpeInterruptsCount: u32,
    pub CpeKernelDeliveryFails: u32,
    pub CpeDriverDpcQueueFails: u32,
    pub CpeGetStateFails: u32,
    pub CpeClearStateFails: u32,
    pub CpeInterruptSources: u32,
    pub CpeLogId: u64,
    pub KernelReserved: [u64; 4],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HAL_MCA_INTERFACE {
    pub Lock: PHALMCAINTERFACELOCK,
    pub Unlock: PHALMCAINTERFACEUNLOCK,
    pub ReadRegister: PHALMCAINTERFACEREADREGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HAL_PLATFORM_INFORMATION {
    pub PlatformFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HAL_POWER_INFORMATION {
    pub TBD: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HAL_PROCESSOR_FEATURE {
    pub UsableFeatureBits: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HAL_PROCESSOR_SPEED_INFORMATION {
    pub ProcessorSpeed: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HARDWARE_COUNTER {
    pub Type: HARDWARE_COUNTER_TYPE,
    pub Reserved: u32,
    pub Index: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HWPROFILE_CHANGE_NOTIFICATION {
    pub Version: u16,
    pub Size: u16,
    pub Event: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_INFO {
    pub Anonymous: IMAGE_INFO_0,
    pub ImageBase: *mut core::ffi::c_void,
    pub ImageSelector: u32,
    pub ImageSize: usize,
    pub ImageSectionNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IMAGE_INFO_0 {
    pub Properties: u32,
    pub Anonymous: IMAGE_INFO_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IMAGE_INFO_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IMAGE_INFO_EX {
    pub Size: usize,
    pub ImageInfo: IMAGE_INFO,
    pub FileObject: *mut super::super::Foundation::FILE_OBJECT,
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct INITIAL_PRIVILEGE_SET {
    pub PrivilegeCount: u32,
    pub Control: u32,
    pub Privilege: [super::super::super::Win32::Security::LUID_AND_ATTRIBUTES; 3],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INPUT_MAPPING_ELEMENT {
    pub InputMappingId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INTEL_CACHE_INFO_EAX {
    pub Ulong: u32,
    pub Anonymous: INTEL_CACHE_INFO_EAX_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INTEL_CACHE_INFO_EAX_0 {
    pub _bitfield: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union INTEL_CACHE_INFO_EBX {
    pub Ulong: u32,
    pub Anonymous: INTEL_CACHE_INFO_EBX_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INTEL_CACHE_INFO_EBX_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct IOMMU_DEVICE_CREATION_CONFIGURATION {
    pub NextConfiguration: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub ConfigType: IOMMU_DEVICE_CREATION_CONFIGURATION_TYPE,
    pub Anonymous: IOMMU_DEVICE_CREATION_CONFIGURATION_0,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union IOMMU_DEVICE_CREATION_CONFIGURATION_0 {
    pub Acpi: IOMMU_DEVICE_CREATION_CONFIGURATION_ACPI,
    pub DeviceId: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IOMMU_DEVICE_CREATION_CONFIGURATION_ACPI {
    pub InputMappingBase: u32,
    pub MappingsCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IOMMU_DMA_DOMAIN_CREATION_FLAGS {
    pub Anonymous: IOMMU_DMA_DOMAIN_CREATION_FLAGS_0,
    pub AsUlonglong: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IOMMU_DMA_DOMAIN_CREATION_FLAGS_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IOMMU_DMA_LOGICAL_ADDRESS_TOKEN {
    pub LogicalAddressBase: u64,
    pub Size: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IOMMU_DMA_LOGICAL_ADDRESS_TOKEN_MAPPED_SEGMENT {
    pub OwningToken: *mut IOMMU_DMA_LOGICAL_ADDRESS_TOKEN,
    pub Offset: usize,
    pub Size: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG {
    pub LogicalAllocatorType: IOMMU_DMA_LOGICAL_ALLOCATOR_TYPE,
    pub Anonymous: IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG_0 {
    pub BuddyAllocatorConfig: IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG_0_0 {
    pub AddressWidth: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IOMMU_DMA_RESERVED_REGION {
    pub RegionNext: *mut IOMMU_DMA_RESERVED_REGION,
    pub Base: u64,
    pub NumberOfPages: usize,
    pub ShouldMap: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IOMMU_INTERFACE_STATE_CHANGE {
    pub PresentFields: IOMMU_INTERFACE_STATE_CHANGE_FIELDS,
    pub AvailableDomainTypes: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IOMMU_INTERFACE_STATE_CHANGE_FIELDS {
    pub Anonymous: IOMMU_INTERFACE_STATE_CHANGE_FIELDS_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IOMMU_INTERFACE_STATE_CHANGE_FIELDS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct IOMMU_MAP_PHYSICAL_ADDRESS {
    pub MapType: IOMMU_MAP_PHYSICAL_ADDRESS_TYPE,
    pub Anonymous: IOMMU_MAP_PHYSICAL_ADDRESS_0,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub union IOMMU_MAP_PHYSICAL_ADDRESS_0 {
    pub Mdl: IOMMU_MAP_PHYSICAL_ADDRESS_0_1,
    pub ContiguousRange: IOMMU_MAP_PHYSICAL_ADDRESS_0_0,
    pub PfnArray: IOMMU_MAP_PHYSICAL_ADDRESS_0_2,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct IOMMU_MAP_PHYSICAL_ADDRESS_0_0 {
    pub Base: i64,
    pub Size: usize,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct IOMMU_MAP_PHYSICAL_ADDRESS_0_1 {
    pub Mdl: *mut super::super::Foundation::MDL,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct IOMMU_MAP_PHYSICAL_ADDRESS_0_2 {
    pub PageFrame: *mut u32,
    pub NumberOfPages: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_ATTRIBUTION_INFORMATION {
    pub Version: u32,
    pub Flags: IO_ATTRIBUTION_INFORMATION_0,
    pub Length: u32,
    pub ServiceStartTime: u64,
    pub CurrentTime: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IO_ATTRIBUTION_INFORMATION_0 {
    pub Anonymous: IO_ATTRIBUTION_INFORMATION_0_0,
    pub AllFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_ATTRIBUTION_INFORMATION_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_CONNECT_INTERRUPT_FULLY_SPECIFIED_PARAMETERS {
    pub PhysicalDeviceObject: *mut super::super::Foundation::DEVICE_OBJECT,
    pub InterruptObject: *mut super::super::Foundation::PKINTERRUPT,
    pub ServiceRoutine: PKSERVICE_ROUTINE,
    pub ServiceContext: *mut core::ffi::c_void,
    pub SpinLock: *mut usize,
    pub SynchronizeIrql: u8,
    pub FloatingSave: super::super::super::Win32::Foundation::BOOLEAN,
    pub ShareVector: super::super::super::Win32::Foundation::BOOLEAN,
    pub Vector: u32,
    pub Irql: u8,
    pub InterruptMode: KINTERRUPT_MODE,
    pub ProcessorEnableMask: usize,
    pub Group: u16,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_CONNECT_INTERRUPT_LINE_BASED_PARAMETERS {
    pub PhysicalDeviceObject: *mut super::super::Foundation::DEVICE_OBJECT,
    pub InterruptObject: *mut super::super::Foundation::PKINTERRUPT,
    pub ServiceRoutine: PKSERVICE_ROUTINE,
    pub ServiceContext: *mut core::ffi::c_void,
    pub SpinLock: *mut usize,
    pub SynchronizeIrql: u8,
    pub FloatingSave: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_CONNECT_INTERRUPT_MESSAGE_BASED_PARAMETERS {
    pub PhysicalDeviceObject: *mut super::super::Foundation::DEVICE_OBJECT,
    pub ConnectionContext: IO_CONNECT_INTERRUPT_MESSAGE_BASED_PARAMETERS_0,
    pub MessageServiceRoutine: PKMESSAGE_SERVICE_ROUTINE,
    pub ServiceContext: *mut core::ffi::c_void,
    pub SpinLock: *mut usize,
    pub SynchronizeIrql: u8,
    pub FloatingSave: super::super::super::Win32::Foundation::BOOLEAN,
    pub FallBackServiceRoutine: PKSERVICE_ROUTINE,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union IO_CONNECT_INTERRUPT_MESSAGE_BASED_PARAMETERS_0 {
    pub Generic: *mut *mut core::ffi::c_void,
    pub InterruptMessageTable: *mut *mut IO_INTERRUPT_MESSAGE_INFO,
    pub InterruptObject: *mut super::super::Foundation::PKINTERRUPT,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_CONNECT_INTERRUPT_PARAMETERS {
    pub Version: u32,
    pub Anonymous: IO_CONNECT_INTERRUPT_PARAMETERS_0,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union IO_CONNECT_INTERRUPT_PARAMETERS_0 {
    pub FullySpecified: IO_CONNECT_INTERRUPT_FULLY_SPECIFIED_PARAMETERS,
    pub LineBased: IO_CONNECT_INTERRUPT_LINE_BASED_PARAMETERS,
    pub MessageBased: IO_CONNECT_INTERRUPT_MESSAGE_BASED_PARAMETERS,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_CSQ {
    pub Type: u32,
    pub CsqInsertIrp: PIO_CSQ_INSERT_IRP,
    pub CsqRemoveIrp: PIO_CSQ_REMOVE_IRP,
    pub CsqPeekNextIrp: PIO_CSQ_PEEK_NEXT_IRP,
    pub CsqAcquireLock: PIO_CSQ_ACQUIRE_LOCK,
    pub CsqReleaseLock: PIO_CSQ_RELEASE_LOCK,
    pub CsqCompleteCanceledIrp: PIO_CSQ_COMPLETE_CANCELED_IRP,
    pub ReservePointer: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_CSQ_IRP_CONTEXT {
    pub Type: u32,
    pub Irp: *mut super::super::Foundation::IRP,
    pub Csq: *mut IO_CSQ,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct IO_DISCONNECT_INTERRUPT_PARAMETERS {
    pub Version: u32,
    pub ConnectionContext: IO_DISCONNECT_INTERRUPT_PARAMETERS_0,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub union IO_DISCONNECT_INTERRUPT_PARAMETERS_0 {
    pub Generic: *mut core::ffi::c_void,
    pub InterruptObject: super::super::Foundation::PKINTERRUPT,
    pub InterruptMessageTable: *mut IO_INTERRUPT_MESSAGE_INFO,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct IO_DRIVER_CREATE_CONTEXT {
    pub Size: i16,
    pub ExtraCreateParameter: *mut isize,
    pub DeviceObjectHint: *mut core::ffi::c_void,
    pub TxnParameters: *mut TXN_PARAMETER_BLOCK,
    pub SiloContext: super::super::Foundation::PESILO,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_ERROR_LOG_MESSAGE {
    pub Type: u16,
    pub Size: u16,
    pub DriverNameLength: u16,
    pub TimeStamp: i64,
    pub DriverNameOffset: u32,
    pub EntryData: IO_ERROR_LOG_PACKET,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_ERROR_LOG_PACKET {
    pub MajorFunctionCode: u8,
    pub RetryCount: u8,
    pub DumpDataSize: u16,
    pub NumberOfStrings: u16,
    pub StringOffset: u16,
    pub EventCategory: u16,
    pub ErrorCode: super::super::super::Win32::Foundation::NTSTATUS,
    pub UniqueErrorValue: u32,
    pub FinalStatus: super::super::super::Win32::Foundation::NTSTATUS,
    pub SequenceNumber: u32,
    pub IoControlCode: u32,
    pub DeviceOffset: i64,
    pub DumpData: [u32; 1],
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct IO_FOEXT_SHADOW_FILE {
    pub BackingFileObject: *mut super::super::Foundation::FILE_OBJECT,
    pub BackingFltInstance: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct IO_FOEXT_SILO_PARAMETERS {
    pub Length: u32,
    pub Anonymous: IO_FOEXT_SILO_PARAMETERS_0,
    pub SiloContext: super::super::Foundation::PESILO,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub union IO_FOEXT_SILO_PARAMETERS_0 {
    pub Anonymous: IO_FOEXT_SILO_PARAMETERS_0_0,
    pub Flags: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct IO_FOEXT_SILO_PARAMETERS_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct IO_INTERRUPT_MESSAGE_INFO {
    pub UnifiedIrql: u8,
    pub MessageCount: u32,
    pub MessageInfo: [IO_INTERRUPT_MESSAGE_INFO_ENTRY; 1],
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct IO_INTERRUPT_MESSAGE_INFO_ENTRY {
    pub MessageAddress: i64,
    pub TargetProcessorSet: usize,
    pub InterruptObject: super::super::Foundation::PKINTERRUPT,
    pub MessageData: u32,
    pub Vector: u32,
    pub Irql: u8,
    pub Mode: KINTERRUPT_MODE,
    pub Polarity: KINTERRUPT_POLARITY,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct IO_REMOVE_LOCK {
    pub Common: IO_REMOVE_LOCK_COMMON_BLOCK,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct IO_REMOVE_LOCK_COMMON_BLOCK {
    pub Removed: super::super::super::Win32::Foundation::BOOLEAN,
    pub Reserved: [super::super::super::Win32::Foundation::BOOLEAN; 3],
    pub IoCount: i32,
    pub RemoveEvent: super::super::Foundation::KEVENT,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct IO_REMOVE_LOCK_DBG_BLOCK {
    pub Signature: i32,
    pub HighWatermark: u32,
    pub MaxLockedTicks: i64,
    pub AllocateTag: i32,
    pub LockList: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub Spin: usize,
    pub LowMemoryCount: i32,
    pub Reserved1: [u32; 4],
    pub Reserved2: *mut core::ffi::c_void,
    pub Blocks: super::super::Foundation::PIO_REMOVE_LOCK_TRACKING_BLOCK,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct IO_REPORT_INTERRUPT_ACTIVE_STATE_PARAMETERS {
    pub Version: u32,
    pub ConnectionContext: IO_REPORT_INTERRUPT_ACTIVE_STATE_PARAMETERS_0,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub union IO_REPORT_INTERRUPT_ACTIVE_STATE_PARAMETERS_0 {
    pub Generic: *mut core::ffi::c_void,
    pub InterruptObject: super::super::Foundation::PKINTERRUPT,
    pub InterruptMessageTable: *mut IO_INTERRUPT_MESSAGE_INFO,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR {
    pub Option: u8,
    pub Type: u8,
    pub ShareDisposition: u8,
    pub Spare1: u8,
    pub Flags: u16,
    pub Spare2: u16,
    pub u: IO_RESOURCE_DESCRIPTOR_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IO_RESOURCE_DESCRIPTOR_0 {
    pub Port: IO_RESOURCE_DESCRIPTOR_0_12,
    pub Memory: IO_RESOURCE_DESCRIPTOR_0_11,
    pub Interrupt: IO_RESOURCE_DESCRIPTOR_0_7,
    pub Dma: IO_RESOURCE_DESCRIPTOR_0_5,
    pub DmaV3: IO_RESOURCE_DESCRIPTOR_0_4,
    pub Generic: IO_RESOURCE_DESCRIPTOR_0_6,
    pub DevicePrivate: IO_RESOURCE_DESCRIPTOR_0_3,
    pub BusNumber: IO_RESOURCE_DESCRIPTOR_0_0,
    pub ConfigData: IO_RESOURCE_DESCRIPTOR_0_1,
    pub Memory40: IO_RESOURCE_DESCRIPTOR_0_8,
    pub Memory48: IO_RESOURCE_DESCRIPTOR_0_9,
    pub Memory64: IO_RESOURCE_DESCRIPTOR_0_10,
    pub Connection: IO_RESOURCE_DESCRIPTOR_0_2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR_0_0 {
    pub Length: u32,
    pub MinBusNumber: u32,
    pub MaxBusNumber: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR_0_1 {
    pub Priority: u32,
    pub Reserved1: u32,
    pub Reserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR_0_2 {
    pub Class: u8,
    pub Type: u8,
    pub Reserved1: u8,
    pub Reserved2: u8,
    pub IdLowPart: u32,
    pub IdHighPart: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR_0_3 {
    pub Data: [u32; 3],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR_0_4 {
    pub RequestLine: u32,
    pub Reserved: u32,
    pub Channel: u32,
    pub TransferWidth: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR_0_5 {
    pub MinimumChannel: u32,
    pub MaximumChannel: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR_0_6 {
    pub Length: u32,
    pub Alignment: u32,
    pub MinimumAddress: i64,
    pub MaximumAddress: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR_0_7 {
    pub MinimumVector: u32,
    pub MaximumVector: u32,
    pub AffinityPolicy: IRQ_DEVICE_POLICY,
    pub PriorityPolicy: IRQ_PRIORITY,
    pub TargetedProcessors: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR_0_8 {
    pub Length40: u32,
    pub Alignment40: u32,
    pub MinimumAddress: i64,
    pub MaximumAddress: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR_0_9 {
    pub Length48: u32,
    pub Alignment48: u32,
    pub MinimumAddress: i64,
    pub MaximumAddress: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR_0_10 {
    pub Length64: u32,
    pub Alignment64: u32,
    pub MinimumAddress: i64,
    pub MaximumAddress: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR_0_11 {
    pub Length: u32,
    pub Alignment: u32,
    pub MinimumAddress: i64,
    pub MaximumAddress: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_DESCRIPTOR_0_12 {
    pub Length: u32,
    pub Alignment: u32,
    pub MinimumAddress: i64,
    pub MaximumAddress: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_LIST {
    pub Version: u16,
    pub Revision: u16,
    pub Count: u32,
    pub Descriptors: [IO_RESOURCE_DESCRIPTOR; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_RESOURCE_REQUIREMENTS_LIST {
    pub ListSize: u32,
    pub InterfaceType: INTERFACE_TYPE,
    pub BusNumber: u32,
    pub SlotNumber: u32,
    pub Reserved: [u32; 3],
    pub AlternativeLists: u32,
    pub List: [IO_RESOURCE_LIST; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_SESSION_CONNECT_INFO {
    pub SessionId: u32,
    pub LocalSession: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_SESSION_STATE_INFORMATION {
    pub SessionId: u32,
    pub SessionState: IO_SESSION_STATE,
    pub LocalSession: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_SESSION_STATE_NOTIFICATION {
    pub Size: u32,
    pub Flags: u32,
    pub IoObject: *mut core::ffi::c_void,
    pub EventMask: u32,
    pub Context: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_STATUS_BLOCK32 {
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
    pub Information: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct IO_STATUS_BLOCK64 {
    pub Anonymous: IO_STATUS_BLOCK64_0,
    pub Information: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union IO_STATUS_BLOCK64_0 {
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
    pub Pointer: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KADDRESS_RANGE {
    pub Address: *mut core::ffi::c_void,
    pub Size: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KADDRESS_RANGE_DESCRIPTOR {
    pub AddressRanges: *const KADDRESS_RANGE,
    pub AddressRangeCount: usize,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct KAPC {
    pub Type: u8,
    pub AllFlags: u8,
    pub Size: u8,
    pub SpareByte1: u8,
    pub SpareLong0: u32,
    pub Thread: *mut isize,
    pub ApcListEntry: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub Reserved: [*mut core::ffi::c_void; 3],
    pub NormalContext: *mut core::ffi::c_void,
    pub SystemArgument1: *mut core::ffi::c_void,
    pub SystemArgument2: *mut core::ffi::c_void,
    pub ApcStateIndex: i8,
    pub ApcMode: i8,
    pub Inserted: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KBUGCHECK_ADD_PAGES {
    pub Context: *mut core::ffi::c_void,
    pub Flags: u32,
    pub BugCheckCode: u32,
    pub Address: usize,
    pub Count: usize,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct KBUGCHECK_CALLBACK_RECORD {
    pub Entry: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub CallbackRoutine: PKBUGCHECK_CALLBACK_ROUTINE,
    pub Buffer: *mut core::ffi::c_void,
    pub Length: u32,
    pub Component: *mut u8,
    pub Checksum: usize,
    pub State: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KBUGCHECK_DUMP_IO {
    pub Offset: u64,
    pub Buffer: *mut core::ffi::c_void,
    pub BufferLength: u32,
    pub Type: KBUGCHECK_DUMP_IO_TYPE,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct KBUGCHECK_REASON_CALLBACK_RECORD {
    pub Entry: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub CallbackRoutine: PKBUGCHECK_REASON_CALLBACK_ROUTINE,
    pub Component: *mut u8,
    pub Checksum: usize,
    pub Reason: KBUGCHECK_CALLBACK_REASON,
    pub State: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KBUGCHECK_REMOVE_PAGES {
    pub Context: *mut core::ffi::c_void,
    pub Flags: u32,
    pub BugCheckCode: u32,
    pub Address: usize,
    pub Count: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KBUGCHECK_SECONDARY_DUMP_DATA {
    pub InBuffer: *mut core::ffi::c_void,
    pub InBufferLength: u32,
    pub MaximumAllowed: u32,
    pub Guid: windows_sys::core::GUID,
    pub OutBuffer: *mut core::ffi::c_void,
    pub OutBufferLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KBUGCHECK_SECONDARY_DUMP_DATA_EX {
    pub InBuffer: *mut core::ffi::c_void,
    pub InBufferLength: u32,
    pub MaximumAllowed: u32,
    pub Guid: windows_sys::core::GUID,
    pub OutBuffer: *mut core::ffi::c_void,
    pub OutBufferLength: u32,
    pub Context: *mut core::ffi::c_void,
    pub Flags: u32,
    pub DumpType: u32,
    pub BugCheckCode: u32,
    pub BugCheckParameter1: usize,
    pub BugCheckParameter2: usize,
    pub BugCheckParameter3: usize,
    pub BugCheckParameter4: usize,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct KBUGCHECK_TRIAGE_DUMP_DATA {
    pub DataArray: *mut KTRIAGE_DUMP_DATA_ARRAY,
    pub Flags: u32,
    pub MaxVirtMemSize: u32,
    pub BugCheckCode: u32,
    pub BugCheckParameter1: usize,
    pub BugCheckParameter2: usize,
    pub BugCheckParameter3: usize,
    pub BugCheckParameter4: usize,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct KDEVICE_QUEUE_ENTRY {
    pub DeviceListEntry: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub SortKey: u32,
    pub Inserted: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KDPC_WATCHDOG_INFORMATION {
    pub DpcTimeLimit: u32,
    pub DpcTimeCount: u32,
    pub DpcWatchdogLimit: u32,
    pub DpcWatchdogCount: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KERNEL_SOFT_RESTART_NOTIFICATION {
    pub Version: u16,
    pub Size: u16,
    pub Event: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KERNEL_USER_TIMES {
    pub CreateTime: i64,
    pub ExitTime: i64,
    pub KernelTime: i64,
    pub UserTime: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_BASIC_INFORMATION {
    pub LastWriteTime: i64,
    pub TitleIndex: u32,
    pub NameLength: u32,
    pub Name: [u16; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_CACHED_INFORMATION {
    pub LastWriteTime: i64,
    pub TitleIndex: u32,
    pub SubKeys: u32,
    pub MaxNameLen: u32,
    pub Values: u32,
    pub MaxValueNameLen: u32,
    pub MaxValueDataLen: u32,
    pub NameLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_CONTROL_FLAGS_INFORMATION {
    pub ControlFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_FULL_INFORMATION {
    pub LastWriteTime: i64,
    pub TitleIndex: u32,
    pub ClassOffset: u32,
    pub ClassLength: u32,
    pub SubKeys: u32,
    pub MaxNameLen: u32,
    pub MaxClassLen: u32,
    pub Values: u32,
    pub MaxValueNameLen: u32,
    pub MaxValueDataLen: u32,
    pub Class: [u16; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_LAYER_INFORMATION {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_NAME_INFORMATION {
    pub NameLength: u32,
    pub Name: [u16; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_NODE_INFORMATION {
    pub LastWriteTime: i64,
    pub TitleIndex: u32,
    pub ClassOffset: u32,
    pub ClassLength: u32,
    pub NameLength: u32,
    pub Name: [u16; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_SET_VIRTUALIZATION_INFORMATION {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_TRUST_INFORMATION {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_VALUE_BASIC_INFORMATION {
    pub TitleIndex: u32,
    pub Type: u32,
    pub NameLength: u32,
    pub Name: [u16; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_VALUE_FULL_INFORMATION {
    pub TitleIndex: u32,
    pub Type: u32,
    pub DataOffset: u32,
    pub DataLength: u32,
    pub NameLength: u32,
    pub Name: [u16; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_VALUE_LAYER_INFORMATION {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_VALUE_PARTIAL_INFORMATION {
    pub TitleIndex: u32,
    pub Type: u32,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_VALUE_PARTIAL_INFORMATION_ALIGN64 {
    pub Type: u32,
    pub DataLength: u32,
    pub Data: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_VIRTUALIZATION_INFORMATION {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_WOW64_FLAGS_INFORMATION {
    pub UserFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KEY_WRITE_TIME_INFORMATION {
    pub LastWriteTime: i64,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct KE_PROCESSOR_CHANGE_NOTIFY_CONTEXT {
    pub State: KE_PROCESSOR_CHANGE_NOTIFY_STATE,
    pub NtNumber: u32,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
    pub ProcNumber: super::super::super::Win32::System::Kernel::PROCESSOR_NUMBER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KFLOATING_SAVE {
    pub ControlWord: u32,
    pub StatusWord: u32,
    pub ErrorOffset: u32,
    pub ErrorSelector: u32,
    pub DataOffset: u32,
    pub DataSelector: u32,
    pub Spare0: u32,
    pub Spare1: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct KGATE {
    pub Header: super::super::Foundation::DISPATCHER_HEADER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KLOCK_QUEUE_HANDLE {
    pub LockQueue: KSPIN_LOCK_QUEUE,
    pub OldIrql: u8,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct KSEMAPHORE {
    pub Header: super::super::Foundation::DISPATCHER_HEADER,
    pub Limit: i32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSPIN_LOCK_QUEUE {
    pub Next: *mut KSPIN_LOCK_QUEUE,
    pub Lock: *mut usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KSYSTEM_TIME {
    pub LowPart: u32,
    pub High1Time: i32,
    pub High2Time: i32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct KTIMER {
    pub Header: super::super::Foundation::DISPATCHER_HEADER,
    pub DueTime: u64,
    pub TimerListEntry: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub Dpc: *mut super::super::Foundation::KDPC,
    pub Period: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct KTRIAGE_DUMP_DATA_ARRAY {
    pub List: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub NumBlocksUsed: u32,
    pub NumBlocksTotal: u32,
    pub DataSize: u32,
    pub MaxDataSize: u32,
    pub ComponentNameBufferLength: u32,
    pub ComponentName: *mut u8,
    pub Blocks: [KADDRESS_RANGE; 1],
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct KUSER_SHARED_DATA {
    pub TickCountLowDeprecated: u32,
    pub TickCountMultiplier: u32,
    pub InterruptTime: KSYSTEM_TIME,
    pub SystemTime: KSYSTEM_TIME,
    pub TimeZoneBias: KSYSTEM_TIME,
    pub ImageNumberLow: u16,
    pub ImageNumberHigh: u16,
    pub NtSystemRoot: [u16; 260],
    pub MaxStackTraceDepth: u32,
    pub CryptoExponent: u32,
    pub TimeZoneId: u32,
    pub LargePageMinimum: u32,
    pub AitSamplingValue: u32,
    pub AppCompatFlag: u32,
    pub RNGSeedVersion: u64,
    pub GlobalValidationRunlevel: u32,
    pub TimeZoneBiasStamp: i32,
    pub NtBuildNumber: u32,
    pub NtProductType: super::super::super::Win32::System::Kernel::NT_PRODUCT_TYPE,
    pub ProductTypeIsValid: super::super::super::Win32::Foundation::BOOLEAN,
    pub Reserved0: [super::super::super::Win32::Foundation::BOOLEAN; 1],
    pub NativeProcessorArchitecture: u16,
    pub NtMajorVersion: u32,
    pub NtMinorVersion: u32,
    pub ProcessorFeatures: [super::super::super::Win32::Foundation::BOOLEAN; 64],
    pub Reserved1: u32,
    pub Reserved3: u32,
    pub TimeSlip: u32,
    pub AlternativeArchitecture: ALTERNATIVE_ARCHITECTURE_TYPE,
    pub BootId: u32,
    pub SystemExpirationDate: i64,
    pub SuiteMask: u32,
    pub KdDebuggerEnabled: super::super::super::Win32::Foundation::BOOLEAN,
    pub Anonymous1: KUSER_SHARED_DATA_0,
    pub CyclesPerYield: u16,
    pub ActiveConsoleId: u32,
    pub DismountCount: u32,
    pub ComPlusPackage: u32,
    pub LastSystemRITEventTickCount: u32,
    pub NumberOfPhysicalPages: u32,
    pub SafeBootMode: super::super::super::Win32::Foundation::BOOLEAN,
    pub Anonymous2: KUSER_SHARED_DATA_1,
    pub Reserved12: [u8; 2],
    pub Anonymous3: KUSER_SHARED_DATA_2,
    pub DataFlagsPad: [u32; 1],
    pub TestRetInstruction: u64,
    pub QpcFrequency: i64,
    pub SystemCall: u32,
    pub Reserved2: u32,
    pub SystemCallPad: [u64; 2],
    pub Anonymous4: KUSER_SHARED_DATA_3,
    pub Cookie: u32,
    pub CookiePad: [u32; 1],
    pub ConsoleSessionForegroundProcessId: i64,
    pub TimeUpdateLock: u64,
    pub BaselineSystemTimeQpc: u64,
    pub BaselineInterruptTimeQpc: u64,
    pub QpcSystemTimeIncrement: u64,
    pub QpcInterruptTimeIncrement: u64,
    pub QpcSystemTimeIncrementShift: u8,
    pub QpcInterruptTimeIncrementShift: u8,
    pub UnparkedProcessorCount: u16,
    pub EnclaveFeatureMask: [u32; 4],
    pub TelemetryCoverageRound: u32,
    pub UserModeGlobalLogger: [u16; 16],
    pub ImageFileExecutionOptions: u32,
    pub LangGenerationCount: u32,
    pub Reserved4: u64,
    pub InterruptTimeBias: u64,
    pub QpcBias: u64,
    pub ActiveProcessorCount: u32,
    pub ActiveGroupCount: u8,
    pub Reserved9: u8,
    pub Anonymous5: KUSER_SHARED_DATA_4,
    pub TimeZoneBiasEffectiveStart: i64,
    pub TimeZoneBiasEffectiveEnd: i64,
    pub XState: super::super::super::Win32::System::Diagnostics::Debug::XSTATE_CONFIGURATION,
    pub FeatureConfigurationChangeStamp: KSYSTEM_TIME,
    pub Spare: u32,
    pub UserPointerAuthMask: u64,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub union KUSER_SHARED_DATA_0 {
    pub MitigationPolicies: u8,
    pub Anonymous: KUSER_SHARED_DATA_0_0,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct KUSER_SHARED_DATA_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub union KUSER_SHARED_DATA_1 {
    pub VirtualizationFlags: u8,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub union KUSER_SHARED_DATA_2 {
    pub SharedDataFlags: u32,
    pub Anonymous: KUSER_SHARED_DATA_2_0,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct KUSER_SHARED_DATA_2_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub union KUSER_SHARED_DATA_3 {
    pub TickCount: KSYSTEM_TIME,
    pub TickCountQuad: u64,
    pub Anonymous: KUSER_SHARED_DATA_3_0,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct KUSER_SHARED_DATA_3_0 {
    pub ReservedTickCountOverlay: [u32; 3],
    pub TickCountPad: [u32; 1],
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub union KUSER_SHARED_DATA_4 {
    pub QpcData: u16,
    pub Anonymous: KUSER_SHARED_DATA_4_0,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Diagnostics_Debug", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct KUSER_SHARED_DATA_4_0 {
    pub QpcBypassEnabled: u8,
    pub QpcShift: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct KWAIT_CHAIN {
    pub Head: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LEGACY_BUS_INFORMATION {
    pub BusTypeGuid: windows_sys::core::GUID,
    pub LegacyBusType: INTERFACE_TYPE,
    pub BusNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LINK_SHARE_ACCESS {
    pub OpenCount: u32,
    pub Deleters: u32,
    pub SharedDelete: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct LOADER_PARTITION_INFORMATION_EX {
    pub PartitionStyle: u32,
    pub PartitionNumber: u32,
    pub Anonymous: LOADER_PARTITION_INFORMATION_EX_0,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union LOADER_PARTITION_INFORMATION_EX_0 {
    pub Signature: u32,
    pub DeviceId: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MAILSLOT_CREATE_PARAMETERS {
    pub MailslotQuota: u32,
    pub MaximumMessageSize: u32,
    pub ReadTimeout: i64,
    pub TimeoutSpecified: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MAP_REGISTER_ENTRY {
    pub MapRegister: *mut core::ffi::c_void,
    pub WriteToDevice: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct MCA_DRIVER_INFO {
    pub ExceptionCallback: isize,
    pub DpcCallback: super::super::Foundation::PKDEFERRED_ROUTINE,
    pub DeviceContext: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MCA_EXCEPTION {
    pub VersionNumber: u32,
    pub ExceptionType: MCA_EXCEPTION_TYPE,
    pub TimeStamp: i64,
    pub ProcessorNumber: u32,
    pub Reserved1: u32,
    pub u: MCA_EXCEPTION_0,
    pub ExtCnt: u32,
    pub Reserved3: u32,
    pub ExtReg: [u64; 24],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MCA_EXCEPTION_0 {
    pub Mca: MCA_EXCEPTION_0_0,
    pub Mce: MCA_EXCEPTION_0_1,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MCA_EXCEPTION_0_0 {
    pub BankNumber: u8,
    pub Reserved2: [u8; 7],
    pub Status: MCI_STATS,
    pub Address: MCI_ADDR,
    pub Misc: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MCA_EXCEPTION_0_1 {
    pub Address: u64,
    pub Type: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MCG_CAP {
    pub Anonymous: MCG_CAP_0,
    pub QuadPart: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCG_CAP_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MCG_STATUS {
    pub Anonymous: MCG_STATUS_0,
    pub QuadPart: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCG_STATUS_0 {
    pub _bitfield: u32,
    pub Reserved2: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MCI_ADDR {
    pub Anonymous: MCI_ADDR_0,
    pub QuadPart: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MCI_ADDR_0 {
    pub Address: u32,
    pub Reserved: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MCI_STATS {
    pub MciStats: MCI_STATS_0,
    pub QuadPart: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MCI_STATS_0 {
    pub McaCod: u16,
    pub MsCod: u16,
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union MCI_STATUS {
    pub CommonBits: MCI_STATUS_BITS_COMMON,
    pub AmdBits: MCI_STATUS_AMD_BITS,
    pub IntelBits: MCI_STATUS_INTEL_BITS,
    pub QuadPart: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_STATUS_AMD_BITS {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_STATUS_BITS_COMMON {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MCI_STATUS_INTEL_BITS {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MEMORY_PARTITION_DEDICATED_MEMORY_OPEN_INFORMATION {
    pub DedicatedMemoryTypeId: u64,
    pub HandleAttributes: u32,
    pub DesiredAccess: u32,
    pub DedicatedMemoryPartitionHandle: super::super::super::Win32::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MM_COPY_ADDRESS {
    pub Anonymous: MM_COPY_ADDRESS_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union MM_COPY_ADDRESS_0 {
    pub VirtualAddress: *mut core::ffi::c_void,
    pub PhysicalAddress: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MM_PHYSICAL_ADDRESS_LIST {
    pub PhysicalAddress: i64,
    pub NumberOfBytes: usize,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct MU_TELEMETRY_SECTION {
    pub ComponentID: windows_sys::core::GUID,
    pub SubComponentID: windows_sys::core::GUID,
    pub Reserved: u32,
    pub ErrorStatusValue: u32,
    pub AdditionalInfo1: u64,
    pub AdditionalInfo2: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NAMED_PIPE_CREATE_PARAMETERS {
    pub NamedPipeType: u32,
    pub ReadMode: u32,
    pub CompletionMode: u32,
    pub MaximumInstances: u32,
    pub InboundQuota: u32,
    pub OutboundQuota: u32,
    pub DefaultTimeout: i64,
    pub TimeoutSpecified: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NPEM_CAPABILITY_STANDARD {
    pub Anonymous: NPEM_CAPABILITY_STANDARD_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NPEM_CAPABILITY_STANDARD_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NPEM_CONTROL_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SetNpemSupportState: PNPEM_CONTROL_ENABLE_DISABLE,
    pub QueryStandardCapabilities: PNPEM_CONTROL_QUERY_STANDARD_CAPABILITIES,
    pub SetStandardControl: PNPEM_CONTROL_SET_STANDARD_CONTROL,
    pub QueryNpemControl: PNPEM_CONTROL_QUERY_CONTROL,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NT_TIB32 {
    pub ExceptionList: u32,
    pub StackBase: u32,
    pub StackLimit: u32,
    pub SubSystemTib: u32,
    pub Anonymous: NT_TIB32_0,
    pub ArbitraryUserPointer: u32,
    pub Self_: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union NT_TIB32_0 {
    pub FiberData: u32,
    pub Version: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OBJECT_HANDLE_INFORMATION {
    pub HandleAttributes: u32,
    pub GrantedAccess: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct OB_CALLBACK_REGISTRATION {
    pub Version: u16,
    pub OperationRegistrationCount: u16,
    pub Altitude: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub RegistrationContext: *mut core::ffi::c_void,
    pub OperationRegistration: *mut OB_OPERATION_REGISTRATION,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct OB_OPERATION_REGISTRATION {
    pub ObjectType: *mut super::super::Foundation::POBJECT_TYPE,
    pub Operations: u32,
    pub PreOperation: POB_PRE_OPERATION_CALLBACK,
    pub PostOperation: POB_POST_OPERATION_CALLBACK,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OB_POST_CREATE_HANDLE_INFORMATION {
    pub GrantedAccess: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OB_POST_DUPLICATE_HANDLE_INFORMATION {
    pub GrantedAccess: u32,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct OB_POST_OPERATION_INFORMATION {
    pub Operation: u32,
    pub Anonymous: OB_POST_OPERATION_INFORMATION_0,
    pub Object: *mut core::ffi::c_void,
    pub ObjectType: super::super::Foundation::POBJECT_TYPE,
    pub CallContext: *mut core::ffi::c_void,
    pub ReturnStatus: super::super::super::Win32::Foundation::NTSTATUS,
    pub Parameters: *mut OB_POST_OPERATION_PARAMETERS,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub union OB_POST_OPERATION_INFORMATION_0 {
    pub Flags: u32,
    pub Anonymous: OB_POST_OPERATION_INFORMATION_0_0,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct OB_POST_OPERATION_INFORMATION_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union OB_POST_OPERATION_PARAMETERS {
    pub CreateHandleInformation: OB_POST_CREATE_HANDLE_INFORMATION,
    pub DuplicateHandleInformation: OB_POST_DUPLICATE_HANDLE_INFORMATION,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OB_PRE_CREATE_HANDLE_INFORMATION {
    pub DesiredAccess: u32,
    pub OriginalDesiredAccess: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OB_PRE_DUPLICATE_HANDLE_INFORMATION {
    pub DesiredAccess: u32,
    pub OriginalDesiredAccess: u32,
    pub SourceProcess: *mut core::ffi::c_void,
    pub TargetProcess: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct OB_PRE_OPERATION_INFORMATION {
    pub Operation: u32,
    pub Anonymous: OB_PRE_OPERATION_INFORMATION_0,
    pub Object: *mut core::ffi::c_void,
    pub ObjectType: super::super::Foundation::POBJECT_TYPE,
    pub CallContext: *mut core::ffi::c_void,
    pub Parameters: *mut OB_PRE_OPERATION_PARAMETERS,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub union OB_PRE_OPERATION_INFORMATION_0 {
    pub Flags: u32,
    pub Anonymous: OB_PRE_OPERATION_INFORMATION_0_0,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct OB_PRE_OPERATION_INFORMATION_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union OB_PRE_OPERATION_PARAMETERS {
    pub CreateHandleInformation: OB_PRE_CREATE_HANDLE_INFORMATION,
    pub DuplicateHandleInformation: OB_PRE_DUPLICATE_HANDLE_INFORMATION,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PAGE_PRIORITY_INFORMATION {
    pub PagePriority: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCIBUSDATA {
    pub Tag: u32,
    pub Version: u32,
    pub ReadConfig: PciReadWriteConfig,
    pub WriteConfig: PciReadWriteConfig,
    pub Pin2Line: PciPin2Line,
    pub Line2Pin: PciLine2Pin,
    pub ParentSlot: PCI_SLOT_NUMBER,
    pub Reserved: [*mut core::ffi::c_void; 4],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCIX_BRIDGE_CAPABILITY {
    pub Header: PCI_CAPABILITIES_HEADER,
    pub SecondaryStatus: PCIX_BRIDGE_CAPABILITY_2,
    pub BridgeStatus: PCIX_BRIDGE_CAPABILITY_0,
    pub UpstreamSplitTransactionCapacity: u16,
    pub UpstreamSplitTransactionLimit: u16,
    pub DownstreamSplitTransactionCapacity: u16,
    pub DownstreamSplitTransactionLimit: u16,
    pub EccControlStatus: PCIX_BRIDGE_CAPABILITY_1,
    pub EccFirstAddress: u32,
    pub EccSecondAddress: u32,
    pub EccAttribute: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCIX_BRIDGE_CAPABILITY_0 {
    pub Anonymous: PCIX_BRIDGE_CAPABILITY_0_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCIX_BRIDGE_CAPABILITY_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCIX_BRIDGE_CAPABILITY_1 {
    pub Anonymous: PCIX_BRIDGE_CAPABILITY_1_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCIX_BRIDGE_CAPABILITY_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCIX_BRIDGE_CAPABILITY_2 {
    pub Anonymous: PCIX_BRIDGE_CAPABILITY_2_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCIX_BRIDGE_CAPABILITY_2_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_ADVANCED_FEATURES_CAPABILITY {
    pub Header: PCI_CAPABILITIES_HEADER,
    pub Length: u8,
    pub Capabilities: PCI_ADVANCED_FEATURES_CAPABILITY_0,
    pub Control: PCI_ADVANCED_FEATURES_CAPABILITY_1,
    pub Status: PCI_ADVANCED_FEATURES_CAPABILITY_2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_ADVANCED_FEATURES_CAPABILITY_0 {
    pub Anonymous: PCI_ADVANCED_FEATURES_CAPABILITY_0_0,
    pub AsUCHAR: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_ADVANCED_FEATURES_CAPABILITY_0_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_ADVANCED_FEATURES_CAPABILITY_1 {
    pub Anonymous: PCI_ADVANCED_FEATURES_CAPABILITY_1_0,
    pub AsUCHAR: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_ADVANCED_FEATURES_CAPABILITY_1_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_ADVANCED_FEATURES_CAPABILITY_2 {
    pub Anonymous: PCI_ADVANCED_FEATURES_CAPABILITY_2_0,
    pub AsUCHAR: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_ADVANCED_FEATURES_CAPABILITY_2_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_AGP_APERTURE_PAGE_SIZE {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_AGP_CAPABILITY {
    pub Header: PCI_CAPABILITIES_HEADER,
    pub _bitfield: u16,
    pub AGPStatus: PCI_AGP_CAPABILITY_1,
    pub AGPCommand: PCI_AGP_CAPABILITY_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_AGP_CAPABILITY_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_AGP_CAPABILITY_1 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_AGP_CONTROL {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_AGP_EXTENDED_CAPABILITY {
    pub IsochStatus: PCI_AGP_ISOCH_STATUS,
    pub AgpControl: PCI_AGP_CONTROL,
    pub ApertureSize: u16,
    pub AperturePageSize: PCI_AGP_APERTURE_PAGE_SIZE,
    pub GartLow: u32,
    pub GartHigh: u32,
    pub IsochCommand: PCI_AGP_ISOCH_COMMAND,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_AGP_ISOCH_COMMAND {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_AGP_ISOCH_STATUS {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_ATS_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SetAddressTranslationServices: PPCI_SET_ATS,
    pub InvalidateQueueDepth: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_BUS_INTERFACE_STANDARD {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub ReadConfig: PPCI_READ_WRITE_CONFIG,
    pub WriteConfig: PPCI_READ_WRITE_CONFIG,
    pub PinToLine: PPCI_PIN_TO_LINE,
    pub LineToPin: PPCI_LINE_TO_PIN,
    pub RootBusCapability: PPCI_ROOT_BUS_CAPABILITY,
    pub ExpressWakeControl: PPCI_EXPRESS_WAKE_CONTROL,
    pub PrepareMultistageResume: PPCI_PREPARE_MULTISTAGE_RESUME,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_CAPABILITIES_HEADER {
    pub CapabilityID: u8,
    pub Next: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_COMMON_CONFIG {
    pub Base: PCI_COMMON_HEADER,
    pub DeviceSpecific: [u8; 192],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_COMMON_HEADER {
    pub VendorID: u16,
    pub DeviceID: u16,
    pub Command: u16,
    pub Status: u16,
    pub RevisionID: u8,
    pub ProgIf: u8,
    pub SubClass: u8,
    pub BaseClass: u8,
    pub CacheLineSize: u8,
    pub LatencyTimer: u8,
    pub HeaderType: u8,
    pub BIST: u8,
    pub u: PCI_COMMON_HEADER_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_COMMON_HEADER_0 {
    pub type0: PCI_COMMON_HEADER_0_0,
    pub type1: PCI_COMMON_HEADER_0_1,
    pub type2: PCI_COMMON_HEADER_0_2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_COMMON_HEADER_0_0 {
    pub BaseAddresses: [u32; 6],
    pub CIS: u32,
    pub SubVendorID: u16,
    pub SubSystemID: u16,
    pub ROMBaseAddress: u32,
    pub CapabilitiesPtr: u8,
    pub Reserved1: [u8; 3],
    pub Reserved2: u32,
    pub InterruptLine: u8,
    pub InterruptPin: u8,
    pub MinimumGrant: u8,
    pub MaximumLatency: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_COMMON_HEADER_0_1 {
    pub BaseAddresses: [u32; 2],
    pub PrimaryBus: u8,
    pub SecondaryBus: u8,
    pub SubordinateBus: u8,
    pub SecondaryLatency: u8,
    pub IOBase: u8,
    pub IOLimit: u8,
    pub SecondaryStatus: u16,
    pub MemoryBase: u16,
    pub MemoryLimit: u16,
    pub PrefetchBase: u16,
    pub PrefetchLimit: u16,
    pub PrefetchBaseUpper32: u32,
    pub PrefetchLimitUpper32: u32,
    pub IOBaseUpper16: u16,
    pub IOLimitUpper16: u16,
    pub CapabilitiesPtr: u8,
    pub Reserved1: [u8; 3],
    pub ROMBaseAddress: u32,
    pub InterruptLine: u8,
    pub InterruptPin: u8,
    pub BridgeControl: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_COMMON_HEADER_0_2 {
    pub SocketRegistersBaseAddress: u32,
    pub CapabilitiesPtr: u8,
    pub Reserved: u8,
    pub SecondaryStatus: u16,
    pub PrimaryBus: u8,
    pub SecondaryBus: u8,
    pub SubordinateBus: u8,
    pub SecondaryLatency: u8,
    pub Range: [PCI_COMMON_HEADER_0_2_0; 4],
    pub InterruptLine: u8,
    pub InterruptPin: u8,
    pub BridgeControl: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_COMMON_HEADER_0_2_0 {
    pub Base: u32,
    pub Limit: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_DEBUGGING_DEVICE_IN_USE {
    pub Segment: u16,
    pub Bus: u32,
    pub Slot: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_DEVICE_PRESENCE_PARAMETERS {
    pub Size: u32,
    pub Flags: u32,
    pub VendorID: u16,
    pub DeviceID: u16,
    pub RevisionID: u8,
    pub SubVendorID: u16,
    pub SubSystemID: u16,
    pub BaseClass: u8,
    pub SubClass: u8,
    pub ProgIf: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_DEVICE_PRESENT_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub IsDevicePresent: PPCI_IS_DEVICE_PRESENT,
    pub IsDevicePresentEx: PPCI_IS_DEVICE_PRESENT_EX,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ACS_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capability: PCI_EXPRESS_ACS_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_ACS_CONTROL,
    pub EgressControl: [u32; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ACS_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_ACS_CAPABILITY_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ACS_CAPABILITY_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ACS_CONTROL {
    pub Anonymous: PCI_EXPRESS_ACS_CONTROL_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ACS_CONTROL_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_AER_CAPABILITIES {
    pub Anonymous: PCI_EXPRESS_AER_CAPABILITIES_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_AER_CAPABILITIES_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_AER_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub UncorrectableErrorStatus: PCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS,
    pub UncorrectableErrorMask: PCI_EXPRESS_UNCORRECTABLE_ERROR_MASK,
    pub UncorrectableErrorSeverity: PCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY,
    pub CorrectableErrorStatus: PCI_EXPRESS_CORRECTABLE_ERROR_STATUS,
    pub CorrectableErrorMask: PCI_EXPRESS_CORRECTABLE_ERROR_MASK,
    pub CapabilitiesAndControl: PCI_EXPRESS_AER_CAPABILITIES,
    pub HeaderLog: [u32; 4],
    pub SecUncorrectableErrorStatus: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_STATUS,
    pub SecUncorrectableErrorMask: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_MASK,
    pub SecUncorrectableErrorSeverity: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_SEVERITY,
    pub SecCapabilitiesAndControl: PCI_EXPRESS_SEC_AER_CAPABILITIES,
    pub SecHeaderLog: [u32; 4],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ARI_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capability: PCI_EXPRESS_ARI_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_ARI_CONTROL_REGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ARI_CAPABILITY_REGISTER {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ARI_CONTROL_REGISTER {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ATS_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capability: PCI_EXPRESS_ATS_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_ATS_CONTROL_REGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ATS_CAPABILITY_REGISTER {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ATS_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_ATS_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ATS_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_BRIDGE_AER_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub UncorrectableErrorStatus: PCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS,
    pub UncorrectableErrorMask: PCI_EXPRESS_UNCORRECTABLE_ERROR_MASK,
    pub UncorrectableErrorSeverity: PCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY,
    pub CorrectableErrorStatus: PCI_EXPRESS_CORRECTABLE_ERROR_STATUS,
    pub CorrectableErrorMask: PCI_EXPRESS_CORRECTABLE_ERROR_MASK,
    pub CapabilitiesAndControl: PCI_EXPRESS_AER_CAPABILITIES,
    pub HeaderLog: [u32; 4],
    pub SecUncorrectableErrorStatus: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_STATUS,
    pub SecUncorrectableErrorMask: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_MASK,
    pub SecUncorrectableErrorSeverity: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_SEVERITY,
    pub SecCapabilitiesAndControl: PCI_EXPRESS_SEC_AER_CAPABILITIES,
    pub SecHeaderLog: [u32; 4],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_CAPABILITIES_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CAPABILITY {
    pub Header: PCI_CAPABILITIES_HEADER,
    pub ExpressCapabilities: PCI_EXPRESS_CAPABILITIES_REGISTER,
    pub DeviceCapabilities: PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER,
    pub DeviceControl: PCI_EXPRESS_DEVICE_CONTROL_REGISTER,
    pub DeviceStatus: PCI_EXPRESS_DEVICE_STATUS_REGISTER,
    pub LinkCapabilities: PCI_EXPRESS_LINK_CAPABILITIES_REGISTER,
    pub LinkControl: PCI_EXPRESS_LINK_CONTROL_REGISTER,
    pub LinkStatus: PCI_EXPRESS_LINK_STATUS_REGISTER,
    pub SlotCapabilities: PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER,
    pub SlotControl: PCI_EXPRESS_SLOT_CONTROL_REGISTER,
    pub SlotStatus: PCI_EXPRESS_SLOT_STATUS_REGISTER,
    pub RootControl: PCI_EXPRESS_ROOT_CONTROL_REGISTER,
    pub RootCapabilities: PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER,
    pub RootStatus: PCI_EXPRESS_ROOT_STATUS_REGISTER,
    pub DeviceCapabilities2: PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER,
    pub DeviceControl2: PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER,
    pub DeviceStatus2: PCI_EXPRESS_DEVICE_STATUS_2_REGISTER,
    pub LinkCapabilities2: PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER,
    pub LinkControl2: PCI_EXPRESS_LINK_CONTROL_2_REGISTER,
    pub LinkStatus2: PCI_EXPRESS_LINK_STATUS_2_REGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CORRECTABLE_ERROR_MASK {
    pub Anonymous: PCI_EXPRESS_CORRECTABLE_ERROR_MASK_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CORRECTABLE_ERROR_MASK_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CORRECTABLE_ERROR_STATUS {
    pub Anonymous: PCI_EXPRESS_CORRECTABLE_ERROR_STATUS_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CORRECTABLE_ERROR_STATUS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CXL_DVSEC_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DvsecHeader1: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1,
    pub DvsecHeader2: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2,
    pub Reserved: [u8; 46],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11 {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CXL_DVSEC_CAPABILITY_V11 {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DvsecHeader1: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1,
    pub DvsecHeader2: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2,
    pub Capability: PCI_EXPRESS_CXL_DVSEC_CAPABILITY_REGISTER_V11,
    pub Control: PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER,
    pub Status: PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER,
    pub Control2: u16,
    pub Status2: u16,
    pub Lock: PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER,
    pub Reserved: u16,
    pub Range1SizeHigh: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_HIGH_REGISTER,
    pub Range1SizeLow: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11,
    pub Range1BaseHigh: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_HIGH_REGISTER,
    pub Range1BaseLow: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER,
    pub Range2SizeHigh: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_HIGH_REGISTER,
    pub Range2SizeLow: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11,
    pub Range2BaseHigh: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_HIGH_REGISTER,
    pub Range2BaseLow: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CXL_DVSEC_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CXL_DVSEC_LOCK_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_HIGH_REGISTER {
    pub MemBaseHigh: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CXL_DVSEC_RANGE_BASE_LOW_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_HIGH_REGISTER {
    pub MemSizeHigh: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11 {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CXL_DVSEC_RANGE_SIZE_LOW_REGISTER_V11_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_CXL_DVSEC_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DvsecHeader1: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1,
    pub DvsecHeader2: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2,
    pub DvsecRegisters: [u16; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1 {
    pub Anonymous: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2 {
    pub Anonymous: PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DESIGNATED_VENDOR_SPECIFIC_HEADER_2_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DEVICE_CAPABILITIES_2_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DEVICE_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DEVICE_CONTROL_2_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_CONTROL_REGISTER {
    pub Anonymous1: PCI_EXPRESS_DEVICE_CONTROL_REGISTER_0,
    pub Anonymous2: PCI_EXPRESS_DEVICE_CONTROL_REGISTER_1,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DEVICE_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DEVICE_CONTROL_REGISTER_1 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_STATUS_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_STATUS_2_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DEVICE_STATUS_2_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DEVICE_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_DEVICE_STATUS_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DEVICE_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub DpcCapabilities: PCI_EXPRESS_DPC_CAPS_REGISTER,
    pub DpcControl: PCI_EXPRESS_DPC_CONTROL_REGISTER,
    pub DpcStatus: PCI_EXPRESS_DPC_STATUS_REGISTER,
    pub DpcErrSrcId: PCI_EXPRESS_DPC_ERROR_SOURCE_ID,
    pub RpPioStatus: PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER,
    pub RpPioMask: PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER,
    pub RpPioSeverity: PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER,
    pub RpPioSysError: PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER,
    pub RpPioException: PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER,
    pub RpPioHeaderLog: PCI_EXPRESS_DPC_RP_PIO_HEADERLOG_REGISTER,
    pub RpPioImpSpecLog: PCI_EXPRESS_DPC_RP_PIO_IMPSPECLOG_REGISTER,
    pub RpPioPrefixLog: PCI_EXPRESS_DPC_RP_PIO_TLPPREFIXLOG_REGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_CAPS_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_CAPS_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_CAPS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_ERROR_SOURCE_ID {
    pub Anonymous: PCI_EXPRESS_DPC_ERROR_SOURCE_ID_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_ERROR_SOURCE_ID_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_RP_PIO_EXCEPTION_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_RP_PIO_HEADERLOG_REGISTER {
    pub PioHeaderLogRegister: [u32; 4],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_IMPSPECLOG_REGISTER {
    pub PioImpSpecLog: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_RP_PIO_MASK_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_RP_PIO_SEVERITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_RP_PIO_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_RP_PIO_SYSERR_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_RP_PIO_TLPPREFIXLOG_REGISTER {
    pub PioTlpPrefixLogRegister: [u32; 4],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_DPC_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_DPC_STATUS_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_DPC_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER {
    pub CapabilityID: u16,
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ERROR_SOURCE_ID {
    pub Anonymous: PCI_EXPRESS_ERROR_SOURCE_ID_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ERROR_SOURCE_ID_0 {
    pub _bitfield1: u16,
    pub _bitfield2: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_EVENT_COLLECTOR_ENDPOINT_ASSOCIATION_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub AssociationBitmap: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_L1_PM_SS_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub L1PmSsCapabilities: PCI_EXPRESS_L1_PM_SS_CAPABILITIES_REGISTER,
    pub L1PmSsControl1: PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER,
    pub L1PmSsControl2: PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER {
    pub Anonymous: PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_L1_PM_SS_CONTROL_1_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_L1_PM_SS_CONTROL_2_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LANE_ERROR_STATUS {
    pub LaneBitmap: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LINK_CAPABILITIES_2_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_CAPABILITIES_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LINK_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LINK_CONTROL3 {
    pub Anonymous: PCI_EXPRESS_LINK_CONTROL3_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CONTROL3_0 {
    pub Anonymous: PCI_EXPRESS_LINK_CONTROL3_0_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LINK_CONTROL3_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CONTROL_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_CONTROL_2_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LINK_CONTROL_2_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LINK_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LINK_QUIESCENT_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub PciExpressEnterLinkQuiescentMode: PPCI_EXPRESS_ENTER_LINK_QUIESCENT_MODE,
    pub PciExpressExitLinkQuiescentMode: PPCI_EXPRESS_EXIT_LINK_QUIESCENT_MODE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_STATUS_2_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_STATUS_2_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LINK_STATUS_2_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LINK_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_LINK_STATUS_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LINK_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LTR_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Latency: PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER {
    pub Anonymous: PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_LTR_MAX_LATENCY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_NPEM_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capability: PCI_EXPRESS_NPEM_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_NPEM_CONTROL_REGISTER,
    pub Status: PCI_EXPRESS_NPEM_STATUS_REGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_NPEM_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_NPEM_CAPABILITY_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_NPEM_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_NPEM_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_NPEM_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_NPEM_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_NPEM_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_NPEM_STATUS_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_NPEM_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_PASID_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capability: PCI_EXPRESS_PASID_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_PASID_CONTROL_REGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PASID_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_PASID_CAPABILITY_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_PASID_CAPABILITY_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PASID_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_PASID_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_PASID_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PME_REQUESTOR_ID {
    pub Anonymous: PCI_EXPRESS_PME_REQUESTOR_ID_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_PME_REQUESTOR_ID_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_PRI_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Control: PCI_EXPRESS_PRI_CONTROL_REGISTER,
    pub Status: PCI_EXPRESS_PRI_STATUS_REGISTER,
    pub PRCapacity: u32,
    pub PRAllocation: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PRI_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_PRI_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_PRI_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PRI_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_PRI_STATUS_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_PRI_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_PTM_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub PtmCapability: PCI_EXPRESS_PTM_CAPABILITY_REGISTER,
    pub PtmControl: PCI_EXPRESS_PTM_CONTROL_REGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PTM_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_PTM_CAPABILITY_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_PTM_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_PTM_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_PTM_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_PTM_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Entry: [PCI_EXPRESS_RESIZABLE_BAR_ENTRY; 6],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_RESIZABLE_BAR_ENTRY {
    pub Capability: PCI_EXPRESS_RESIZABLE_BAR_CAPABILITY_REGISTER,
    pub Control: PCI_EXPRESS_RESIZABLE_BAR_CONTROL_REGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ROOTPORT_AER_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub UncorrectableErrorStatus: PCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS,
    pub UncorrectableErrorMask: PCI_EXPRESS_UNCORRECTABLE_ERROR_MASK,
    pub UncorrectableErrorSeverity: PCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY,
    pub CorrectableErrorStatus: PCI_EXPRESS_CORRECTABLE_ERROR_STATUS,
    pub CorrectableErrorMask: PCI_EXPRESS_CORRECTABLE_ERROR_MASK,
    pub CapabilitiesAndControl: PCI_EXPRESS_AER_CAPABILITIES,
    pub HeaderLog: [u32; 4],
    pub RootErrorCommand: PCI_EXPRESS_ROOT_ERROR_COMMAND,
    pub RootErrorStatus: PCI_EXPRESS_ROOT_ERROR_STATUS,
    pub ErrorSourceId: PCI_EXPRESS_ERROR_SOURCE_ID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ROOT_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ROOT_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_ROOT_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ROOT_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ROOT_ERROR_COMMAND {
    pub Anonymous: PCI_EXPRESS_ROOT_ERROR_COMMAND_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ROOT_ERROR_COMMAND_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ROOT_ERROR_STATUS {
    pub Anonymous: PCI_EXPRESS_ROOT_ERROR_STATUS_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ROOT_ERROR_STATUS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ROOT_PORT_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub ReadConfigSpace: PPCI_EXPRESS_ROOT_PORT_READ_CONFIG_SPACE,
    pub WriteConfigSpace: PPCI_EXPRESS_ROOT_PORT_WRITE_CONFIG_SPACE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_ROOT_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_ROOT_STATUS_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_ROOT_STATUS_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SECONDARY_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub LinkControl3: PCI_EXPRESS_LINK_CONTROL3,
    pub LaneErrorStatus: PCI_EXPRESS_LANE_ERROR_STATUS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SEC_AER_CAPABILITIES {
    pub Anonymous: PCI_EXPRESS_SEC_AER_CAPABILITIES_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SEC_AER_CAPABILITIES_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_MASK {
    pub Anonymous: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_MASK_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_MASK_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_SEVERITY {
    pub Anonymous: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_SEVERITY_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_SEVERITY_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_STATUS {
    pub Anonymous: PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_STATUS_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SEC_UNCORRECTABLE_ERROR_STATUS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SERIAL_NUMBER_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub LowSerialNumber: u32,
    pub HighSerialNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SLOT_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SLOT_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_SLOT_CONTROL_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SLOT_CONTROL_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SLOT_STATUS_REGISTER {
    pub Anonymous: PCI_EXPRESS_SLOT_STATUS_REGISTER_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SLOT_STATUS_REGISTER_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SRIOV_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub SRIOVCapabilities: PCI_EXPRESS_SRIOV_CAPS,
    pub SRIOVControl: PCI_EXPRESS_SRIOV_CONTROL,
    pub SRIOVStatus: PCI_EXPRESS_SRIOV_STATUS,
    pub InitialVFs: u16,
    pub TotalVFs: u16,
    pub NumVFs: u16,
    pub FunctionDependencyLink: u8,
    pub RsvdP1: u8,
    pub FirstVFOffset: u16,
    pub VFStride: u16,
    pub RsvdP2: u16,
    pub VFDeviceId: u16,
    pub SupportedPageSizes: u32,
    pub SystemPageSize: u32,
    pub BaseAddresses: [u32; 6],
    pub VFMigrationStateArrayOffset: PCI_EXPRESS_SRIOV_MIGRATION_STATE_ARRAY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SRIOV_CAPS {
    pub Anonymous: PCI_EXPRESS_SRIOV_CAPS_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SRIOV_CAPS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SRIOV_CONTROL {
    pub Anonymous: PCI_EXPRESS_SRIOV_CONTROL_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SRIOV_CONTROL_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SRIOV_MIGRATION_STATE_ARRAY {
    pub Anonymous: PCI_EXPRESS_SRIOV_MIGRATION_STATE_ARRAY_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SRIOV_MIGRATION_STATE_ARRAY_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_SRIOV_STATUS {
    pub Anonymous: PCI_EXPRESS_SRIOV_STATUS_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_SRIOV_STATUS_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_TPH_REQUESTER_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub RequesterCapability: PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER,
    pub RequesterControl: PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER {
    pub Anonymous: PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_TPH_REQUESTER_CAPABILITY_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER {
    pub Anonymous: PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_TPH_REQUESTER_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_TPH_ST_TABLE_ENTRY {
    pub Anonymous: PCI_EXPRESS_TPH_ST_TABLE_ENTRY_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_TPH_ST_TABLE_ENTRY_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_UNCORRECTABLE_ERROR_MASK {
    pub Anonymous: PCI_EXPRESS_UNCORRECTABLE_ERROR_MASK_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_UNCORRECTABLE_ERROR_MASK_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY {
    pub Anonymous: PCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_UNCORRECTABLE_ERROR_SEVERITY_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS {
    pub Anonymous: PCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_UNCORRECTABLE_ERROR_STATUS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_VENDOR_SPECIFIC_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub VsecId: u16,
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_EXPRESS_VIRTUAL_CHANNEL_CAPABILITY {
    pub Header: PCI_EXPRESS_ENHANCED_CAPABILITY_HEADER,
    pub Capabilities1: VIRTUAL_CHANNEL_CAPABILITIES1,
    pub Capabilities2: VIRTUAL_CHANNEL_CAPABILITIES2,
    pub Control: VIRTUAL_CHANNEL_CONTROL,
    pub Status: VIRTUAL_CHANNEL_STATUS,
    pub Resource: [VIRTUAL_RESOURCE; 8],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_FIRMWARE_BUS_CAPS {
    pub Type: u16,
    pub Length: u16,
    pub Anonymous: PCI_FIRMWARE_BUS_CAPS_0,
    pub CurrentSpeedAndMode: u8,
    pub SupportedSpeedsAndModesLowByte: u8,
    pub SupportedSpeedsAndModesHighByte: u8,
    pub Voltage: u8,
    pub Reserved2: [u8; 7],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_FIRMWARE_BUS_CAPS_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_FIRMWARE_BUS_CAPS_RETURN_BUFFER {
    pub Version: u16,
    pub Status: u16,
    pub Length: u32,
    pub Caps: PCI_FIRMWARE_BUS_CAPS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_FPB_CAPABILITIES_REGISTER {
    pub Anonymous: PCI_FPB_CAPABILITIES_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_FPB_CAPABILITIES_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_FPB_CAPABILITY {
    pub Header: PCI_FPB_CAPABILITY_HEADER,
    pub CapabilitiesRegister: PCI_FPB_CAPABILITIES_REGISTER,
    pub RidVectorControl1Register: PCI_FPB_RID_VECTOR_CONTROL1_REGISTER,
    pub RidVectorControl2Register: PCI_FPB_RID_VECTOR_CONTROL2_REGISTER,
    pub MemLowVectorControlRegister: PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER,
    pub MemHighVectorControl1Register: PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER,
    pub MemHighVectorControl2Register: PCI_FPB_MEM_HIGH_VECTOR_CONTROL2_REGISTER,
    pub VectorAccessControlRegister: PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER,
    pub VectorAccessDataRegister: PCI_FPB_VECTOR_ACCESS_DATA_REGISTER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_FPB_CAPABILITY_HEADER {
    pub Header: PCI_CAPABILITIES_HEADER,
    pub Reserved: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER {
    pub Anonymous: PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_FPB_MEM_HIGH_VECTOR_CONTROL1_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_FPB_MEM_HIGH_VECTOR_CONTROL2_REGISTER {
    pub MemHighVectorStartUpper: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER {
    pub Anonymous: PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_FPB_MEM_LOW_VECTOR_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_FPB_RID_VECTOR_CONTROL1_REGISTER {
    pub Anonymous: PCI_FPB_RID_VECTOR_CONTROL1_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_FPB_RID_VECTOR_CONTROL1_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_FPB_RID_VECTOR_CONTROL2_REGISTER {
    pub Anonymous: PCI_FPB_RID_VECTOR_CONTROL2_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_FPB_RID_VECTOR_CONTROL2_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER {
    pub Anonymous: PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_FPB_VECTOR_ACCESS_CONTROL_REGISTER_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_FPB_VECTOR_ACCESS_DATA_REGISTER {
    pub VectorAccessData: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_MSIX_TABLE_CONFIG_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SetTableEntry: PPCI_MSIX_SET_ENTRY,
    pub MaskTableEntry: PPCI_MSIX_MASKUNMASK_ENTRY,
    pub UnmaskTableEntry: PPCI_MSIX_MASKUNMASK_ENTRY,
    pub GetTableEntry: PPCI_MSIX_GET_ENTRY,
    pub GetTableSize: PPCI_MSIX_GET_TABLE_SIZE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_PMC {
    pub _bitfield: u8,
    pub Support: PCI_PMC_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_PMC_0 {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_PMCSR {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_PMCSR_BSE {
    pub _bitfield: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_PM_CAPABILITY {
    pub Header: PCI_CAPABILITIES_HEADER,
    pub PMC: PCI_PM_CAPABILITY_2,
    pub PMCSR: PCI_PM_CAPABILITY_1,
    pub PMCSR_BSE: PCI_PM_CAPABILITY_0,
    pub Data: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_PM_CAPABILITY_0 {
    pub BridgeSupport: PCI_PMCSR_BSE,
    pub AsUCHAR: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_PM_CAPABILITY_1 {
    pub ControlStatus: PCI_PMCSR,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_PM_CAPABILITY_2 {
    pub Capabilities: PCI_PMC,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_ROOT_BUS_HARDWARE_CAPABILITY {
    pub SecondaryInterface: PCI_HARDWARE_INTERFACE,
    pub Anonymous: PCI_ROOT_BUS_HARDWARE_CAPABILITY_0,
    pub OscFeatureSupport: PCI_ROOT_BUS_OSC_SUPPORT_FIELD,
    pub OscControlRequest: PCI_ROOT_BUS_OSC_CONTROL_FIELD,
    pub OscControlGranted: PCI_ROOT_BUS_OSC_CONTROL_FIELD,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_ROOT_BUS_HARDWARE_CAPABILITY_0 {
    pub BusCapabilitiesFound: super::super::super::Win32::Foundation::BOOLEAN,
    pub CurrentSpeedAndMode: u32,
    pub SupportedSpeedsAndModes: u32,
    pub DeviceIDMessagingCapable: super::super::super::Win32::Foundation::BOOLEAN,
    pub SecondaryBusWidth: PCI_BUS_WIDTH,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_ROOT_BUS_OSC_CONTROL_FIELD {
    pub u: PCI_ROOT_BUS_OSC_CONTROL_FIELD_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_ROOT_BUS_OSC_CONTROL_FIELD_0 {
    pub Anonymous: PCI_ROOT_BUS_OSC_CONTROL_FIELD_0_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_ROOT_BUS_OSC_CONTROL_FIELD_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_ROOT_BUS_OSC_SUPPORT_FIELD {
    pub u: PCI_ROOT_BUS_OSC_SUPPORT_FIELD_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_ROOT_BUS_OSC_SUPPORT_FIELD_0 {
    pub Anonymous: PCI_ROOT_BUS_OSC_SUPPORT_FIELD_0_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_ROOT_BUS_OSC_SUPPORT_FIELD_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_SECURITY_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SetAccessControlServices: PPCI_SET_ACS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_SECURITY_INTERFACE2 {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub Flags: u32,
    pub SupportedScenarios: u32,
    pub SetAccessControlServices: PPCI_SET_ACS2,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_SEGMENT_BUS_NUMBER {
    pub u: PCI_SEGMENT_BUS_NUMBER_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_SEGMENT_BUS_NUMBER_0 {
    pub bits: PCI_SEGMENT_BUS_NUMBER_0_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_SEGMENT_BUS_NUMBER_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_SLOT_NUMBER {
    pub u: PCI_SLOT_NUMBER_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_SLOT_NUMBER_0 {
    pub bits: PCI_SLOT_NUMBER_0_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_SLOT_NUMBER_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_SUBSYSTEM_IDS_CAPABILITY {
    pub Header: PCI_CAPABILITIES_HEADER,
    pub Reserved: u16,
    pub SubVendorID: u16,
    pub SubSystemID: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_VENDOR_SPECIFIC_CAPABILITY {
    pub Header: PCI_CAPABILITIES_HEADER,
    pub VscLength: u8,
    pub VendorSpecific: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_VIRTUALIZATION_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SetVirtualFunctionData: PSET_VIRTUAL_DEVICE_DATA,
    pub GetVirtualFunctionData: PGET_VIRTUAL_DEVICE_DATA,
    pub GetLocation: PGET_VIRTUAL_DEVICE_LOCATION,
    pub GetResources: PGET_VIRTUAL_DEVICE_RESOURCES,
    pub EnableVirtualization: PENABLE_VIRTUALIZATION,
    pub GetVirtualFunctionProbedBars: PGET_VIRTUAL_FUNCTION_PROBED_BARS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_X_CAPABILITY {
    pub Header: PCI_CAPABILITIES_HEADER,
    pub Command: PCI_X_CAPABILITY_0,
    pub Status: PCI_X_CAPABILITY_1,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_X_CAPABILITY_0 {
    pub bits: PCI_X_CAPABILITY_0_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_X_CAPABILITY_0_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PCI_X_CAPABILITY_1 {
    pub bits: PCI_X_CAPABILITY_1_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCI_X_CAPABILITY_1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub union PCW_CALLBACK_INFORMATION {
    pub AddCounter: PCW_COUNTER_INFORMATION,
    pub RemoveCounter: PCW_COUNTER_INFORMATION,
    pub EnumerateInstances: PCW_MASK_INFORMATION,
    pub CollectData: PCW_MASK_INFORMATION,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCW_COUNTER_DESCRIPTOR {
    pub Id: u16,
    pub StructIndex: u16,
    pub Offset: u16,
    pub Size: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCW_COUNTER_INFORMATION {
    pub CounterMask: u64,
    pub InstanceMask: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCW_DATA {
    pub Data: *const core::ffi::c_void,
    pub Size: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct PCW_MASK_INFORMATION {
    pub CounterMask: u64,
    pub InstanceMask: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub InstanceId: u32,
    pub CollectMultiple: super::super::super::Win32::Foundation::BOOLEAN,
    pub Buffer: super::super::Foundation::PPCW_BUFFER,
    pub CancelEvent: *mut super::super::Foundation::KEVENT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PCW_REGISTRATION_INFORMATION {
    pub Version: u32,
    pub Name: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub CounterCount: u32,
    pub Counters: *mut PCW_COUNTER_DESCRIPTOR,
    pub Callback: PPCW_CALLBACK,
    pub CallbackContext: *mut core::ffi::c_void,
    pub Flags: PCW_REGISTRATION_FLAGS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PHYSICAL_COUNTER_EVENT_BUFFER_CONFIGURATION {
    pub OverflowHandler: PPHYSICAL_COUNTER_EVENT_BUFFER_OVERFLOW_HANDLER,
    pub CustomEventBufferEntrySize: u32,
    pub EventThreshold: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR {
    pub Type: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_TYPE,
    pub Flags: u32,
    pub u: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_0 {
    pub CounterIndex: u32,
    pub Range: PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_0_0,
    pub OverflowHandler: PPHYSICAL_COUNTER_OVERFLOW_HANDLER,
    pub EventBufferConfiguration: PHYSICAL_COUNTER_EVENT_BUFFER_CONFIGURATION,
    pub IdentificationTag: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR_0_0 {
    pub Begin: u32,
    pub End: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PHYSICAL_COUNTER_RESOURCE_LIST {
    pub Count: u32,
    pub Descriptors: [PHYSICAL_COUNTER_RESOURCE_DESCRIPTOR; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PHYSICAL_MEMORY_RANGE {
    pub BaseAddress: i64,
    pub NumberOfBytes: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PLUGPLAY_NOTIFICATION_HEADER {
    pub Version: u16,
    pub Size: u16,
    pub Event: windows_sys::core::GUID,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PM_DISPATCH_TABLE {
    pub Signature: u32,
    pub Version: u32,
    pub Function: [*mut core::ffi::c_void; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PNP_BUS_INFORMATION {
    pub BusTypeGuid: windows_sys::core::GUID,
    pub LegacyBusType: INTERFACE_TYPE,
    pub BusNumber: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PNP_EXTENDED_ADDRESS_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub QueryExtendedAddress: PQUERYEXTENDEDADDRESS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PNP_LOCATION_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub GetLocationString: PGET_LOCATION_STRING,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PNP_REPLACE_DRIVER_INTERFACE {
    pub Size: u32,
    pub Version: u32,
    pub Flags: u32,
    pub Unload: PREPLACE_UNLOAD,
    pub BeginReplace: PREPLACE_BEGIN,
    pub EndReplace: PREPLACE_END,
    pub MirrorPhysicalMemory: PREPLACE_MIRROR_PHYSICAL_MEMORY,
    pub SetProcessorId: PREPLACE_SET_PROCESSOR_ID,
    pub Swap: PREPLACE_SWAP,
    pub InitiateHardwareMirror: PREPLACE_INITIATE_HARDWARE_MIRROR,
    pub MirrorPlatformMemory: PREPLACE_MIRROR_PLATFORM_MEMORY,
    pub GetMemoryDestination: PREPLACE_GET_MEMORY_DESTINATION,
    pub EnableDisableHardwareQuiesce: PREPLACE_ENABLE_DISABLE_HARDWARE_QUIESCE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PNP_REPLACE_MEMORY_LIST {
    pub AllocatedCount: u32,
    pub Count: u32,
    pub TotalLength: u64,
    pub Ranges: [PNP_REPLACE_MEMORY_LIST_0; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PNP_REPLACE_MEMORY_LIST_0 {
    pub Address: i64,
    pub Length: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PNP_REPLACE_PARAMETERS {
    pub Size: u32,
    pub Version: u32,
    pub Target: u64,
    pub Spare: u64,
    pub TargetProcessors: *mut PNP_REPLACE_PROCESSOR_LIST,
    pub SpareProcessors: *mut PNP_REPLACE_PROCESSOR_LIST,
    pub TargetMemory: *mut PNP_REPLACE_MEMORY_LIST,
    pub SpareMemory: *mut PNP_REPLACE_MEMORY_LIST,
    pub MapMemory: PREPLACE_MAP_MEMORY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PNP_REPLACE_PROCESSOR_LIST {
    pub Affinity: *mut usize,
    pub GroupCount: u32,
    pub AllocatedCount: u32,
    pub Count: u32,
    pub ApicIds: [u32; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PNP_REPLACE_PROCESSOR_LIST_V1 {
    pub AffinityMask: usize,
    pub AllocatedCount: u32,
    pub Count: u32,
    pub ApicIds: [u32; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POOLED_USAGE_AND_LIMITS {
    pub PeakPagedPoolUsage: usize,
    pub PagedPoolUsage: usize,
    pub PagedPoolLimit: usize,
    pub PeakNonPagedPoolUsage: usize,
    pub NonPagedPoolUsage: usize,
    pub NonPagedPoolLimit: usize,
    pub PeakPagefileUsage: usize,
    pub PagefileUsage: usize,
    pub PagefileLimit: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POOL_CREATE_EXTENDED_PARAMS {
    pub Version: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POOL_EXTENDED_PARAMETER {
    pub Anonymous1: POOL_EXTENDED_PARAMETER_0,
    pub Anonymous2: POOL_EXTENDED_PARAMETER_1,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POOL_EXTENDED_PARAMETER_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union POOL_EXTENDED_PARAMETER_1 {
    pub Reserved2: u64,
    pub Reserved3: *mut core::ffi::c_void,
    pub Priority: EX_POOL_PRIORITY,
    pub SecurePoolParams: *mut POOL_EXTENDED_PARAMS_SECURE_POOL,
    pub PreferredNode: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POOL_EXTENDED_PARAMS_SECURE_POOL {
    pub SecurePoolHandle: super::super::super::Win32::Foundation::HANDLE,
    pub Buffer: *mut core::ffi::c_void,
    pub Cookie: usize,
    pub SecurePoolFlags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POWER_MONITOR_INVOCATION {
    pub Console: super::super::super::Win32::Foundation::BOOLEAN,
    pub RequestReason: POWER_MONITOR_REQUEST_REASON,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POWER_PLATFORM_INFORMATION {
    pub AoAc: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POWER_SEQUENCE {
    pub SequenceD1: u32,
    pub SequenceD2: u32,
    pub SequenceD3: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POWER_SESSION_CONNECT {
    pub Connected: super::super::super::Win32::Foundation::BOOLEAN,
    pub Console: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POWER_SESSION_RIT_STATE {
    pub Active: super::super::super::Win32::Foundation::BOOLEAN,
    pub LastInputTime: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POWER_SESSION_TIMEOUTS {
    pub InputTimeout: u32,
    pub DisplayTimeout: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POWER_SESSION_WINLOGON {
    pub SessionId: u32,
    pub Console: super::super::super::Win32::Foundation::BOOLEAN,
    pub Locked: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Power")]
#[derive(Clone, Copy)]
pub union POWER_STATE {
    pub SystemState: super::super::super::Win32::System::Power::SYSTEM_POWER_STATE,
    pub DeviceState: super::super::super::Win32::System::Power::DEVICE_POWER_STATE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POWER_THROTTLING_PROCESS_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct POWER_THROTTLING_THREAD_STATE {
    pub Version: u32,
    pub ControlMask: u32,
    pub StateMask: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PO_FX_COMPONENT_IDLE_STATE {
    pub TransitionLatency: u64,
    pub ResidencyRequirement: u64,
    pub NominalPower: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PO_FX_COMPONENT_PERF_INFO {
    pub PerfStateSetsCount: u32,
    pub PerfStateSets: [PO_FX_COMPONENT_PERF_SET; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PO_FX_COMPONENT_PERF_SET {
    pub Name: super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Flags: u64,
    pub Unit: PO_FX_PERF_STATE_UNIT,
    pub Type: PO_FX_PERF_STATE_TYPE,
    pub Anonymous: PO_FX_COMPONENT_PERF_SET_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PO_FX_COMPONENT_PERF_SET_0 {
    pub Discrete: PO_FX_COMPONENT_PERF_SET_0_0,
    pub Range: PO_FX_COMPONENT_PERF_SET_0_1,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PO_FX_COMPONENT_PERF_SET_0_0 {
    pub Count: u32,
    pub States: *mut PO_FX_PERF_STATE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PO_FX_COMPONENT_PERF_SET_0_1 {
    pub Minimum: u64,
    pub Maximum: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PO_FX_COMPONENT_V1 {
    pub Id: windows_sys::core::GUID,
    pub IdleStateCount: u32,
    pub DeepestWakeableIdleState: u32,
    pub IdleStates: *mut PO_FX_COMPONENT_IDLE_STATE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PO_FX_COMPONENT_V2 {
    pub Id: windows_sys::core::GUID,
    pub Flags: u64,
    pub DeepestWakeableIdleState: u32,
    pub IdleStateCount: u32,
    pub IdleStates: *mut PO_FX_COMPONENT_IDLE_STATE,
    pub ProviderCount: u32,
    pub Providers: *mut u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PO_FX_DEVICE_V1 {
    pub Version: u32,
    pub ComponentCount: u32,
    pub ComponentActiveConditionCallback: PPO_FX_COMPONENT_ACTIVE_CONDITION_CALLBACK,
    pub ComponentIdleConditionCallback: PPO_FX_COMPONENT_IDLE_CONDITION_CALLBACK,
    pub ComponentIdleStateCallback: PPO_FX_COMPONENT_IDLE_STATE_CALLBACK,
    pub DevicePowerRequiredCallback: PPO_FX_DEVICE_POWER_REQUIRED_CALLBACK,
    pub DevicePowerNotRequiredCallback: PPO_FX_DEVICE_POWER_NOT_REQUIRED_CALLBACK,
    pub PowerControlCallback: PPO_FX_POWER_CONTROL_CALLBACK,
    pub DeviceContext: *mut core::ffi::c_void,
    pub Components: [PO_FX_COMPONENT_V1; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PO_FX_DEVICE_V2 {
    pub Version: u32,
    pub Flags: u64,
    pub ComponentActiveConditionCallback: PPO_FX_COMPONENT_ACTIVE_CONDITION_CALLBACK,
    pub ComponentIdleConditionCallback: PPO_FX_COMPONENT_IDLE_CONDITION_CALLBACK,
    pub ComponentIdleStateCallback: PPO_FX_COMPONENT_IDLE_STATE_CALLBACK,
    pub DevicePowerRequiredCallback: PPO_FX_DEVICE_POWER_REQUIRED_CALLBACK,
    pub DevicePowerNotRequiredCallback: PPO_FX_DEVICE_POWER_NOT_REQUIRED_CALLBACK,
    pub PowerControlCallback: PPO_FX_POWER_CONTROL_CALLBACK,
    pub DeviceContext: *mut core::ffi::c_void,
    pub ComponentCount: u32,
    pub Components: [PO_FX_COMPONENT_V2; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PO_FX_DEVICE_V3 {
    pub Version: u32,
    pub Flags: u64,
    pub ComponentActiveConditionCallback: PPO_FX_COMPONENT_ACTIVE_CONDITION_CALLBACK,
    pub ComponentIdleConditionCallback: PPO_FX_COMPONENT_IDLE_CONDITION_CALLBACK,
    pub ComponentIdleStateCallback: PPO_FX_COMPONENT_IDLE_STATE_CALLBACK,
    pub DevicePowerRequiredCallback: PPO_FX_DEVICE_POWER_REQUIRED_CALLBACK,
    pub DevicePowerNotRequiredCallback: PPO_FX_DEVICE_POWER_NOT_REQUIRED_CALLBACK,
    pub PowerControlCallback: PPO_FX_POWER_CONTROL_CALLBACK,
    pub DirectedPowerUpCallback: PPO_FX_DIRECTED_POWER_UP_CALLBACK,
    pub DirectedPowerDownCallback: PPO_FX_DIRECTED_POWER_DOWN_CALLBACK,
    pub DirectedFxTimeoutInSeconds: u32,
    pub DeviceContext: *mut core::ffi::c_void,
    pub ComponentCount: u32,
    pub Components: [PO_FX_COMPONENT_V2; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PO_FX_PERF_STATE {
    pub Value: u64,
    pub Context: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PO_FX_PERF_STATE_CHANGE {
    pub Set: u32,
    pub Anonymous: PO_FX_PERF_STATE_CHANGE_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PO_FX_PERF_STATE_CHANGE_0 {
    pub StateIndex: u32,
    pub StateValue: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_ACCESS_TOKEN {
    pub Token: super::super::super::Win32::Foundation::HANDLE,
    pub Thread: super::super::super::Win32::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION {
    pub Anonymous: PROCESS_DEVICEMAP_INFORMATION_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_DEVICEMAP_INFORMATION_0 {
    pub Set: PROCESS_DEVICEMAP_INFORMATION_0_1,
    pub Query: PROCESS_DEVICEMAP_INFORMATION_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION_0_0 {
    pub DriveMap: u32,
    pub DriveType: [u8; 32],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION_0_1 {
    pub DirectoryHandle: super::super::super::Win32::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION_EX {
    pub Anonymous: PROCESS_DEVICEMAP_INFORMATION_EX_0,
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_DEVICEMAP_INFORMATION_EX_0 {
    pub Set: PROCESS_DEVICEMAP_INFORMATION_EX_0_1,
    pub Query: PROCESS_DEVICEMAP_INFORMATION_EX_0_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION_EX_0_0 {
    pub DriveMap: u32,
    pub DriveType: [u8; 32],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_DEVICEMAP_INFORMATION_EX_0_1 {
    pub DirectoryHandle: super::super::super::Win32::Foundation::HANDLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_EXCEPTION_PORT {
    pub ExceptionPortHandle: super::super::super::Win32::Foundation::HANDLE,
    pub StateFlags: u32,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
#[derive(Clone, Copy)]
pub struct PROCESS_EXTENDED_BASIC_INFORMATION {
    pub Size: usize,
    pub BasicInfo: super::super::super::Win32::System::Threading::PROCESS_BASIC_INFORMATION,
    pub Anonymous: PROCESS_EXTENDED_BASIC_INFORMATION_0,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
#[derive(Clone, Copy)]
pub union PROCESS_EXTENDED_BASIC_INFORMATION_0 {
    pub Flags: u32,
    pub Anonymous: PROCESS_EXTENDED_BASIC_INFORMATION_0_0,
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Kernel", feature = "Win32_System_Threading"))]
#[derive(Clone, Copy)]
pub struct PROCESS_EXTENDED_BASIC_INFORMATION_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_HANDLE_TRACING_ENABLE {
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_HANDLE_TRACING_ENABLE_EX {
    pub Flags: u32,
    pub TotalSlots: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[derive(Clone, Copy)]
pub struct PROCESS_HANDLE_TRACING_ENTRY {
    pub Handle: super::super::super::Win32::Foundation::HANDLE,
    pub ClientId: super::super::super::Win32::System::WindowsProgramming::CLIENT_ID,
    pub Type: u32,
    pub Stacks: [*mut core::ffi::c_void; 16],
}
#[repr(C)]
#[cfg(feature = "Win32_System_WindowsProgramming")]
#[derive(Clone, Copy)]
pub struct PROCESS_HANDLE_TRACING_QUERY {
    pub Handle: super::super::super::Win32::Foundation::HANDLE,
    pub TotalTraces: u32,
    pub HandleTrace: [PROCESS_HANDLE_TRACING_ENTRY; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_KEEPALIVE_COUNT_INFORMATION {
    pub WakeCount: u32,
    pub NoWakeCount: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_MEMBERSHIP_INFORMATION {
    pub ServerSiloId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_REVOKE_FILE_HANDLES_INFORMATION {
    pub TargetDevicePath: super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_SESSION_INFORMATION {
    pub SessionId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_SYSCALL_PROVIDER_INFORMATION {
    pub ProviderId: windows_sys::core::GUID,
    pub Level: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_WS_WATCH_INFORMATION {
    pub FaultingPc: *mut core::ffi::c_void,
    pub FaultingVa: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
#[derive(Clone, Copy)]
pub struct PS_CREATE_NOTIFY_INFO {
    pub Size: usize,
    pub Anonymous: PS_CREATE_NOTIFY_INFO_0,
    pub ParentProcessId: super::super::super::Win32::Foundation::HANDLE,
    pub CreatingThreadId: super::super::super::Win32::System::WindowsProgramming::CLIENT_ID,
    pub FileObject: *mut super::super::Foundation::FILE_OBJECT,
    pub ImageFileName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub CommandLine: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub CreationStatus: super::super::super::Win32::Foundation::NTSTATUS,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
#[derive(Clone, Copy)]
pub union PS_CREATE_NOTIFY_INFO_0 {
    pub Flags: u32,
    pub Anonymous: PS_CREATE_NOTIFY_INFO_0_0,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
#[derive(Clone, Copy)]
pub struct PS_CREATE_NOTIFY_INFO_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PTM_CONTROL_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub QueryGranularity: PPTM_DEVICE_QUERY_GRANULARITY,
    pub QueryTimeSource: PPTM_DEVICE_QUERY_TIME_SOURCE,
    pub Enable: PPTM_DEVICE_ENABLE,
    pub Disable: PPTM_DEVICE_DISABLE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REENUMERATE_SELF_INTERFACE_STANDARD {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub SurpriseRemoveAndReenumerateSelf: PREENUMERATE_SELF,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_CALLBACK_CONTEXT_CLEANUP_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_CREATE_KEY_INFORMATION {
    pub CompleteName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub RootObject: *mut core::ffi::c_void,
    pub ObjectType: *mut core::ffi::c_void,
    pub CreateOptions: u32,
    pub Class: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub SecurityQualityOfService: *mut core::ffi::c_void,
    pub DesiredAccess: u32,
    pub GrantedAccess: u32,
    pub Disposition: *mut u32,
    pub ResultObject: *mut *mut core::ffi::c_void,
    pub CallContext: *mut core::ffi::c_void,
    pub RootObjectContext: *mut core::ffi::c_void,
    pub Transaction: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_CREATE_KEY_INFORMATION_V1 {
    pub CompleteName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub RootObject: *mut core::ffi::c_void,
    pub ObjectType: *mut core::ffi::c_void,
    pub Options: u32,
    pub Class: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub SecurityQualityOfService: *mut core::ffi::c_void,
    pub DesiredAccess: u32,
    pub GrantedAccess: u32,
    pub Disposition: *mut u32,
    pub ResultObject: *mut *mut core::ffi::c_void,
    pub CallContext: *mut core::ffi::c_void,
    pub RootObjectContext: *mut core::ffi::c_void,
    pub Transaction: *mut core::ffi::c_void,
    pub Version: usize,
    pub RemainingName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Wow64Flags: u32,
    pub Attributes: u32,
    pub CheckAccessMode: i8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_DELETE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_DELETE_VALUE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub ValueName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_KEY_HANDLE_CLOSE_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_LOAD_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub KeyName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub SourceFile: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Flags: u32,
    pub TrustClassObject: *mut core::ffi::c_void,
    pub UserEvent: *mut core::ffi::c_void,
    pub DesiredAccess: u32,
    pub RootHandle: *mut super::super::super::Win32::Foundation::HANDLE,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_LOAD_KEY_INFORMATION_V2 {
    pub Object: *mut core::ffi::c_void,
    pub KeyName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub SourceFile: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Flags: u32,
    pub TrustClassObject: *mut core::ffi::c_void,
    pub UserEvent: *mut core::ffi::c_void,
    pub DesiredAccess: u32,
    pub RootHandle: *mut super::super::super::Win32::Foundation::HANDLE,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Version: usize,
    pub FileAccessToken: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_POST_CREATE_KEY_INFORMATION {
    pub CompleteName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub Object: *mut core::ffi::c_void,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_POST_OPERATION_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
    pub PreInformation: *mut core::ffi::c_void,
    pub ReturnStatus: super::super::super::Win32::Foundation::NTSTATUS,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_PRE_CREATE_KEY_INFORMATION {
    pub CompleteName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct REG_QUERY_KEY_NAME {
    pub Object: *mut core::ffi::c_void,
    pub ObjectNameInfo: *mut super::super::Foundation::OBJECT_NAME_INFORMATION,
    pub Length: u32,
    pub ReturnLength: *mut u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct REG_QUERY_KEY_SECURITY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub SecurityInformation: *mut u32,
    pub SecurityDescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR,
    pub Length: *mut u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_RENAME_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub NewName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_REPLACE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub OldFileName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub NewFileName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_RESTORE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub FileHandle: super::super::super::Win32::Foundation::HANDLE,
    pub Flags: u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_SAVE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub FileHandle: super::super::super::Win32::Foundation::HANDLE,
    pub Format: u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_SAVE_MERGED_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub FileHandle: super::super::super::Win32::Foundation::HANDLE,
    pub HighKeyObject: *mut core::ffi::c_void,
    pub LowKeyObject: *mut core::ffi::c_void,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct REG_SET_KEY_SECURITY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub SecurityInformation: *mut u32,
    pub SecurityDescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_SET_VALUE_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub ValueName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub TitleIndex: u32,
    pub Type: u32,
    pub Data: *mut core::ffi::c_void,
    pub DataSize: u32,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct REG_UNLOAD_KEY_INFORMATION {
    pub Object: *mut core::ffi::c_void,
    pub UserEvent: *mut core::ffi::c_void,
    pub CallContext: *mut core::ffi::c_void,
    pub ObjectContext: *mut core::ffi::c_void,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct RESOURCE_HASH_ENTRY {
    pub ListEntry: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub Address: *mut core::ffi::c_void,
    pub ContentionCount: u32,
    pub Number: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct RESOURCE_PERFORMANCE_DATA {
    pub ActiveResourceCount: u32,
    pub TotalResourceCount: u32,
    pub ExclusiveAcquire: u32,
    pub SharedFirstLevel: u32,
    pub SharedSecondLevel: u32,
    pub StarveFirstLevel: u32,
    pub StarveSecondLevel: u32,
    pub WaitForExclusive: u32,
    pub OwnerTableExpands: u32,
    pub MaximumTableExpand: u32,
    pub HashTable: [super::super::super::Win32::System::Kernel::LIST_ENTRY; 64],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTL_AVL_TABLE {
    pub BalancedRoot: RTL_BALANCED_LINKS,
    pub OrderedPointer: *mut core::ffi::c_void,
    pub WhichOrderedElement: u32,
    pub NumberGenericTableElements: u32,
    pub DepthOfTree: u32,
    pub RestartKey: *mut RTL_BALANCED_LINKS,
    pub DeleteCount: u32,
    pub CompareRoutine: PRTL_AVL_COMPARE_ROUTINE,
    pub AllocateRoutine: PRTL_AVL_ALLOCATE_ROUTINE,
    pub FreeRoutine: PRTL_AVL_FREE_ROUTINE,
    pub TableContext: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTL_BALANCED_LINKS {
    pub Parent: *mut RTL_BALANCED_LINKS,
    pub LeftChild: *mut RTL_BALANCED_LINKS,
    pub RightChild: *mut RTL_BALANCED_LINKS,
    pub Balance: i8,
    pub Reserved: [u8; 3],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTL_BITMAP {
    pub SizeOfBitMap: u32,
    pub Buffer: *mut u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTL_BITMAP_RUN {
    pub StartingIndex: u32,
    pub NumberOfBits: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTL_DYNAMIC_HASH_TABLE {
    pub Flags: u32,
    pub Shift: u32,
    pub TableSize: u32,
    pub Pivot: u32,
    pub DivisorMask: u32,
    pub NumEntries: u32,
    pub NonEmptyBuckets: u32,
    pub NumEnumerators: u32,
    pub Directory: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct RTL_DYNAMIC_HASH_TABLE_CONTEXT {
    pub ChainHead: *mut super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub PrevLinkage: *mut super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub Signature: usize,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct RTL_DYNAMIC_HASH_TABLE_ENTRY {
    pub Linkage: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub Signature: usize,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct RTL_DYNAMIC_HASH_TABLE_ENUMERATOR {
    pub Anonymous: RTL_DYNAMIC_HASH_TABLE_ENUMERATOR_0,
    pub ChainHead: *mut super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub BucketIndex: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub union RTL_DYNAMIC_HASH_TABLE_ENUMERATOR_0 {
    pub HashEntry: RTL_DYNAMIC_HASH_TABLE_ENTRY,
    pub CurEntry: *mut super::super::super::Win32::System::Kernel::LIST_ENTRY,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
#[derive(Clone, Copy)]
pub struct RTL_GENERIC_TABLE {
    pub TableRoot: *mut super::super::Foundation::RTL_SPLAY_LINKS,
    pub InsertOrderList: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub OrderedPointer: *mut super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub WhichOrderedElement: u32,
    pub NumberGenericTableElements: u32,
    pub CompareRoutine: PRTL_GENERIC_COMPARE_ROUTINE,
    pub AllocateRoutine: PRTL_GENERIC_ALLOCATE_ROUTINE,
    pub FreeRoutine: PRTL_GENERIC_FREE_ROUTINE,
    pub TableContext: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct RTL_QUERY_REGISTRY_TABLE {
    pub QueryRoutine: PRTL_QUERY_REGISTRY_ROUTINE,
    pub Flags: u32,
    pub Name: windows_sys::core::PWSTR,
    pub EntryContext: *mut core::ffi::c_void,
    pub DefaultType: u32,
    pub DefaultData: *mut core::ffi::c_void,
    pub DefaultLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCATTER_GATHER_ELEMENT {
    pub Address: i64,
    pub Length: u32,
    pub Reserved: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SCATTER_GATHER_LIST {
    pub NumberOfElements: u32,
    pub Reserved: usize,
    pub Elements: [SCATTER_GATHER_ELEMENT; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SDEV_IDENTIFIER_INTERFACE {
    pub InterfaceHeader: INTERFACE,
    pub GetIdentifier: PGET_SDEV_IDENTIFIER,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct SECURE_DRIVER_INTERFACE {
    pub InterfaceHeader: INTERFACE,
    pub ProcessReference: PSECURE_DRIVER_PROCESS_REFERENCE,
    pub ProcessDereference: PSECURE_DRIVER_PROCESS_DEREFERENCE,
    pub Reserved: u32,
}
pub type SECURITY_CONTEXT_TRACKING_MODE = u8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SHARE_ACCESS {
    pub OpenCount: u32,
    pub Readers: u32,
    pub Writers: u32,
    pub Deleters: u32,
    pub SharedRead: u32,
    pub SharedWrite: u32,
    pub SharedDelete: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct SIGNAL_REG_VALUE {
    pub RegName: [u8; 32],
    pub MsrAddr: u32,
    pub Value: u64,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub struct SILO_MONITOR_REGISTRATION {
    pub Version: u8,
    pub MonitorHost: super::super::super::Win32::Foundation::BOOLEAN,
    pub MonitorExistingSilos: super::super::super::Win32::Foundation::BOOLEAN,
    pub Reserved: [u8; 5],
    pub Anonymous: SILO_MONITOR_REGISTRATION_0,
    pub CreateCallback: SILO_MONITOR_CREATE_CALLBACK,
    pub TerminateCallback: SILO_MONITOR_TERMINATE_CALLBACK,
}
#[repr(C)]
#[cfg(feature = "Wdk_Foundation")]
#[derive(Clone, Copy)]
pub union SILO_MONITOR_REGISTRATION_0 {
    pub DriverObjectName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
    pub ComponentName: *mut super::super::super::Win32::Foundation::UNICODE_STRING,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SOC_SUBSYSTEM_FAILURE_DETAILS {
    pub SubsysType: SOC_SUBSYSTEM_TYPE,
    pub FirmwareVersion: u64,
    pub HardwareVersion: u64,
    pub UnifiedFailureRegionSize: u32,
    pub UnifiedFailureRegion: [i8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_FIRMWARE_TABLE_HANDLER {
    pub ProviderSignature: u32,
    pub Register: super::super::super::Win32::Foundation::BOOLEAN,
    pub FirmwareTableHandler: PFNFTH,
    pub DriverObject: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_FIRMWARE_TABLE_INFORMATION {
    pub ProviderSignature: u32,
    pub Action: SYSTEM_FIRMWARE_TABLE_ACTION,
    pub TableID: u32,
    pub TableBufferLength: u32,
    pub TableBuffer: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_POWER_STATE_CONTEXT {
    pub Anonymous: SYSTEM_POWER_STATE_CONTEXT_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union SYSTEM_POWER_STATE_CONTEXT_0 {
    pub Anonymous: SYSTEM_POWER_STATE_CONTEXT_0_0,
    pub ContextAsUlong: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct SYSTEM_POWER_STATE_CONTEXT_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct TARGET_DEVICE_REMOVAL_NOTIFICATION {
    pub Version: u16,
    pub Size: u16,
    pub Event: windows_sys::core::GUID,
    pub FileObject: *mut super::super::Foundation::FILE_OBJECT,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TIMER_SET_COALESCABLE_TIMER_INFO {
    pub DueTime: i64,
    pub TimerApcRoutine: PTIMER_APC_ROUTINE,
    pub TimerContext: *mut core::ffi::c_void,
    pub WakeContext: *mut COUNTED_REASON_CONTEXT,
    pub Period: u32,
    pub TolerableDelay: u32,
    pub PreviousState: *mut super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TIME_FIELDS {
    pub Year: i16,
    pub Month: i16,
    pub Day: i16,
    pub Hour: i16,
    pub Minute: i16,
    pub Second: i16,
    pub Milliseconds: i16,
    pub Weekday: i16,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct TRANSLATOR_INTERFACE {
    pub Size: u16,
    pub Version: u16,
    pub Context: *mut core::ffi::c_void,
    pub InterfaceReference: PINTERFACE_REFERENCE,
    pub InterfaceDereference: PINTERFACE_DEREFERENCE,
    pub TranslateResources: PTRANSLATE_RESOURCE_HANDLER,
    pub TranslateResourceRequirements: PTRANSLATE_RESOURCE_REQUIREMENTS_HANDLER,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TXN_PARAMETER_BLOCK {
    pub Length: u16,
    pub TxFsContext: u16,
    pub TransactionObject: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIRTUAL_CHANNEL_CAPABILITIES1 {
    pub Anonymous: VIRTUAL_CHANNEL_CAPABILITIES1_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIRTUAL_CHANNEL_CAPABILITIES1_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIRTUAL_CHANNEL_CAPABILITIES2 {
    pub Anonymous: VIRTUAL_CHANNEL_CAPABILITIES2_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIRTUAL_CHANNEL_CAPABILITIES2_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIRTUAL_CHANNEL_CONTROL {
    pub Anonymous: VIRTUAL_CHANNEL_CONTROL_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIRTUAL_CHANNEL_CONTROL_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIRTUAL_CHANNEL_STATUS {
    pub Anonymous: VIRTUAL_CHANNEL_STATUS_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIRTUAL_CHANNEL_STATUS_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIRTUAL_RESOURCE {
    pub Capability: VIRTUAL_RESOURCE_CAPABILITY,
    pub Control: VIRTUAL_RESOURCE_CONTROL,
    pub RsvdP: u16,
    pub Status: VIRTUAL_RESOURCE_STATUS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIRTUAL_RESOURCE_CAPABILITY {
    pub Anonymous: VIRTUAL_RESOURCE_CAPABILITY_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIRTUAL_RESOURCE_CAPABILITY_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIRTUAL_RESOURCE_CONTROL {
    pub Anonymous: VIRTUAL_RESOURCE_CONTROL_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIRTUAL_RESOURCE_CONTROL_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union VIRTUAL_RESOURCE_STATUS {
    pub Anonymous: VIRTUAL_RESOURCE_STATUS_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VIRTUAL_RESOURCE_STATUS_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VM_COUNTERS {
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VM_COUNTERS_EX {
    pub PeakVirtualSize: usize,
    pub VirtualSize: usize,
    pub PageFaultCount: u32,
    pub PeakWorkingSetSize: usize,
    pub WorkingSetSize: usize,
    pub QuotaPeakPagedPoolUsage: usize,
    pub QuotaPagedPoolUsage: usize,
    pub QuotaPeakNonPagedPoolUsage: usize,
    pub QuotaNonPagedPoolUsage: usize,
    pub PagefileUsage: usize,
    pub PeakPagefileUsage: usize,
    pub PrivateUsage: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct VM_COUNTERS_EX2 {
    pub CountersEx: VM_COUNTERS_EX,
    pub PrivateWorkingSetSize: usize,
    pub SharedCommitUsage: u64,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct WAIT_CONTEXT_BLOCK {
    pub Anonymous: WAIT_CONTEXT_BLOCK_0,
    pub DeviceRoutine: super::super::Foundation::DRIVER_CONTROL,
    pub DeviceContext: *mut core::ffi::c_void,
    pub NumberOfMapRegisters: u32,
    pub DeviceObject: *mut core::ffi::c_void,
    pub CurrentIrp: *mut core::ffi::c_void,
    pub BufferChainingDpc: *mut super::super::Foundation::KDPC,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub union WAIT_CONTEXT_BLOCK_0 {
    pub WaitQueueEntry: KDEVICE_QUEUE_ENTRY,
    pub Anonymous: WAIT_CONTEXT_BLOCK_0_0,
}
#[repr(C)]
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
#[derive(Clone, Copy)]
pub struct WAIT_CONTEXT_BLOCK_0_0 {
    pub DmaWaitEntry: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub NumberOfChannels: u32,
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA128A {
    pub Low: u64,
    pub High: i64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_ACPI_TIMEOUT_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub TableType: [i8; 32],
    pub TableRequest: [i8; 32],
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct WHEAP_ADD_REMOVE_ERROR_SOURCE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Descriptor: super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_DESCRIPTOR,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
    pub IsRemove: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_ATTEMPT_RECOVERY_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorHeader: WHEA_ERROR_RECORD_HEADER,
    pub ArchitecturalRecovery: super::super::super::Win32::Foundation::BOOLEAN,
    pub PshedRecovery: super::super::super::Win32::Foundation::BOOLEAN,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct WHEAP_BAD_HEST_NOTIFY_DATA_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub SourceId: u16,
    pub Reserved: u16,
    pub NotifyDesc: super::super::super::Win32::System::Diagnostics::Debug::WHEA_NOTIFICATION_DESCRIPTOR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_CLEARED_POISON_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PhysicalAddress: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_CMCI_IMPLEMENTED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub CmciAvailable: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_CMCI_INITERR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Msr: u64,
    pub Type: u32,
    pub Bank: u32,
    pub EpIndex: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_CMCI_RESTART_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub CmciRestoreAttempts: u32,
    pub MaxCmciRestoreLimit: u32,
    pub MaxCorrectedErrorsFound: u32,
    pub MaxCorrectedErrorLimit: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_CREATE_GENERIC_RECORD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Error: [i8; 32],
    pub EntryCount: u32,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct WHEAP_DEFERRED_EVENT {
    pub ListEntry: super::super::super::Win32::System::Kernel::LIST_ENTRY,
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_DEVICE_DRV_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Function: [i8; 32],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_DPC_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrType: WHEAP_DPC_ERROR_EVENT_TYPE,
    pub Bus: u32,
    pub Device: u32,
    pub Function: u32,
    pub DeviceId: u16,
    pub VendorId: u16,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct WHEAP_DROPPED_CORRECTED_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorSourceType: super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_TYPE,
    pub ErrorSourceId: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_EDPC_ENABLED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub eDPCEnabled: super::super::super::Win32::Foundation::BOOLEAN,
    pub eDPCRecovEnabled: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_ERROR_CLEARED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub EpIndex: u32,
    pub Bank: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_ERROR_RECORD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Record: *mut WHEA_ERROR_RECORD,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_ERR_SRC_ARRAY_INVALID_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorSourceCount: u32,
    pub ReportedLength: u32,
    pub ExpectedLength: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct WHEAP_ERR_SRC_INVALID_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrDescriptor: super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_DESCRIPTOR,
    pub Error: [i8; 32],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_FOUND_ERROR_IN_BANK_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub EpIndex: u32,
    pub Bank: u32,
    pub MciStatus: u64,
    pub ErrorType: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_GENERIC_ERR_MEM_MAP_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub MapReason: [i8; 32],
    pub PhysicalAddress: u64,
    pub Length: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_OSC_IMPLEMENTED {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub OscImplemented: super::super::super::Win32::Foundation::BOOLEAN,
    pub DebugChecked: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PCIE_CONFIG_INFO {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Segment: u32,
    pub Bus: u32,
    pub Device: u32,
    pub Function: u32,
    pub Offset: u32,
    pub Length: u32,
    pub Value: u64,
    pub Succeeded: u8,
    pub Reserved: [u8; 3],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PCIE_OVERRIDE_INFO {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Segment: u32,
    pub Bus: u32,
    pub Device: u32,
    pub Function: u32,
    pub ValidBits: u8,
    pub Reserved: [u8; 3],
    pub UncorrectableErrorMask: u32,
    pub UncorrectableErrorSeverity: u32,
    pub CorrectableErrorMask: u32,
    pub CapAndControl: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PCIE_READ_OVERRIDES_ERR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub FailureReason: u32,
    pub FailureStatus: super::super::super::Win32::Foundation::NTSTATUS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PFA_MEMORY_OFFLINED {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub DecisionType: WHEAP_PFA_OFFLINE_DECISION_TYPE,
    pub ImmediateSuccess: super::super::super::Win32::Foundation::BOOLEAN,
    pub Page: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PFA_MEMORY_POLICY {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RegistryKeysPresent: u32,
    pub DisableOffline: super::super::super::Win32::Foundation::BOOLEAN,
    pub PersistOffline: super::super::super::Win32::Foundation::BOOLEAN,
    pub PfaDisabled: super::super::super::Win32::Foundation::BOOLEAN,
    pub PageCount: u32,
    pub ErrorThreshold: u32,
    pub TimeOut: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PFA_MEMORY_REMOVE_MONITOR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RemoveTrigger: WHEA_PFA_REMOVE_TRIGGER,
    pub TimeInList: u32,
    pub ErrorCount: u32,
    pub Page: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_PLUGIN_DEFECT_LIST_CORRUPT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_PLUGIN_DEFECT_LIST_FULL_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_PLUGIN_DEFECT_LIST_UEFI_VAR_FAILED {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEAP_PLUGIN_PFA_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub NoFurtherPfa: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PROCESS_EINJ_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Error: [i8; 32],
    pub InjectionActionTableValid: super::super::super::Win32::Foundation::BOOLEAN,
    pub BeginInjectionInstructionCount: u32,
    pub GetTriggerErrorActionTableInstructionCount: u32,
    pub SetErrorTypeInstructionCount: u32,
    pub GetErrorTypeInstructionCount: u32,
    pub EndOperationInstructionCount: u32,
    pub ExecuteOperationInstructionCount: u32,
    pub CheckBusyStatusInstructionCount: u32,
    pub GetCommandStatusInstructionCount: u32,
    pub SetErrorTypeWithAddressInstructionCount: u32,
    pub GetExecuteOperationTimingsInstructionCount: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PROCESS_HEST_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Error: [i8; 32],
    pub EntryType: [i8; 32],
    pub EntryIndex: u32,
    pub HestValid: super::super::super::Win32::Foundation::BOOLEAN,
    pub CmcCount: u32,
    pub MceCount: u32,
    pub NmiCount: u32,
    pub AerRootCount: u32,
    pub AerBridgeCount: u32,
    pub AerEndPointCount: u32,
    pub GenericV1Count: u32,
    pub GenericV2Count: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PSHED_INJECT_ERROR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorType: u32,
    pub Parameter1: u64,
    pub Parameter2: u64,
    pub Parameter3: u64,
    pub Parameter4: u64,
    pub InjectionStatus: super::super::super::Win32::Foundation::NTSTATUS,
    pub InjectionAttempted: super::super::super::Win32::Foundation::BOOLEAN,
    pub InjectionByPlugin: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_PSHED_PLUGIN_REGISTER {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Version: u32,
    pub Length: u32,
    pub FunctionalAreaMask: u32,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_ROW_FAILURE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub LowOrderPage: u32,
    pub HighOrderPage: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_SPURIOUS_AER_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub ErrorHandlerType: u32,
    pub SpuriousErrorSourceId: u32,
    pub RootErrorCommand: u32,
    pub RootErrorStatus: u32,
    pub DeviceAssociationBitmap: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct WHEAP_STARTED_REPORT_HW_ERROR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ErrorPacket: *mut WHEA_ERROR_PACKET_V2,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEAP_STUCK_ERROR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub EpIndex: u32,
    pub Bank: u32,
    pub MciStatus: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ACPI_HEADER {
    pub Signature: u32,
    pub Length: u32,
    pub Revision: u8,
    pub Checksum: u8,
    pub OemId: [u8; 6],
    pub OemTableId: u64,
    pub OemRevision: u32,
    pub CreatorId: u32,
    pub CreatorRevision: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_AMD_EXTENDED_REGISTERS {
    pub IPID: u64,
    pub SYND: u64,
    pub CONFIG: u64,
    pub DESTAT: u64,
    pub DEADDR: u64,
    pub MISC1: u64,
    pub MISC2: u64,
    pub MISC3: u64,
    pub MISC4: u64,
    pub RasCap: u64,
    pub Reserved: [u64; 14],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARMV8_AARCH32_GPRS {
    pub R0: u32,
    pub R1: u32,
    pub R2: u32,
    pub R3: u32,
    pub R4: u32,
    pub R5: u32,
    pub R6: u32,
    pub R7: u32,
    pub R8: u32,
    pub R9: u32,
    pub R10: u32,
    pub R11: u32,
    pub R12: u32,
    pub R13: u32,
    pub R14: u32,
    pub R15: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARMV8_AARCH64_EL3_CSR {
    pub ELR_EL3: u64,
    pub ESR_EL3: u64,
    pub FAR_EL3: u64,
    pub MAIR_EL3: u64,
    pub SCTLR_EL3: u64,
    pub SP_EL3: u64,
    pub SPSR_EL3: u64,
    pub TCR_EL3: u64,
    pub TPIDR_EL3: u64,
    pub TTBR0_EL3: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARMV8_AARCH64_GPRS {
    pub X0: u64,
    pub X1: u64,
    pub X2: u64,
    pub X3: u64,
    pub X4: u64,
    pub X5: u64,
    pub X6: u64,
    pub X7: u64,
    pub X8: u64,
    pub X9: u64,
    pub X10: u64,
    pub X11: u64,
    pub X12: u64,
    pub X13: u64,
    pub X14: u64,
    pub X15: u64,
    pub X16: u64,
    pub X17: u64,
    pub X18: u64,
    pub X19: u64,
    pub X20: u64,
    pub X21: u64,
    pub X22: u64,
    pub X23: u64,
    pub X24: u64,
    pub X25: u64,
    pub X26: u64,
    pub X27: u64,
    pub X28: u64,
    pub X29: u64,
    pub X30: u64,
    pub SP: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_AARCH32_EL1_CSR {
    pub DFAR: u32,
    pub DFSR: u32,
    pub IFAR: u32,
    pub ISR: u32,
    pub MAIR0: u32,
    pub MAIR1: u32,
    pub MIDR: u32,
    pub MPIDR: u32,
    pub NMRR: u32,
    pub PRRR: u32,
    pub SCTLR: u32,
    pub SPSR: u32,
    pub SPSR_abt: u32,
    pub SPSR_fiq: u32,
    pub SPSR_irq: u32,
    pub SPSR_svc: u32,
    pub SPSR_und: u32,
    pub TPIDRPRW: u32,
    pub TPIDRURO: u32,
    pub TPIDRURW: u32,
    pub TTBCR: u32,
    pub TTBR0: u32,
    pub TTBR1: u32,
    pub DACR: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_AARCH32_EL2_CSR {
    pub ELR_hyp: u32,
    pub HAMAIR0: u32,
    pub HAMAIR1: u32,
    pub HCR: u32,
    pub HCR2: u32,
    pub HDFAR: u32,
    pub HIFAR: u32,
    pub HPFAR: u32,
    pub HSR: u32,
    pub HTCR: u32,
    pub HTPIDR: u32,
    pub HTTBR: u32,
    pub SPSR_hyp: u32,
    pub VTCR: u32,
    pub VTTBR: u32,
    pub DACR32_EL2: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_AARCH32_SECURE_CSR {
    pub SCTLR: u32,
    pub SPSR_mon: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_AARCH64_EL1_CSR {
    pub ELR_EL1: u64,
    pub ESR_EL2: u64,
    pub FAR_EL1: u64,
    pub ISR_EL1: u64,
    pub MAIR_EL1: u64,
    pub MIDR_EL1: u64,
    pub MPIDR_EL1: u64,
    pub SCTLR_EL1: u64,
    pub SP_EL0: u64,
    pub SP_EL1: u64,
    pub SPSR_EL1: u64,
    pub TCR_EL1: u64,
    pub TPIDR_EL0: u64,
    pub TPIDR_EL1: u64,
    pub TPIDRRO_EL0: u64,
    pub TTBR0_EL1: u64,
    pub TTBR1_EL1: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_AARCH64_EL2_CSR {
    pub ELR_EL2: u64,
    pub ESR_EL2: u64,
    pub FAR_EL2: u64,
    pub HACR_EL2: u64,
    pub HCR_EL2: u64,
    pub HPFAR_EL2: u64,
    pub MAIR_EL2: u64,
    pub SCTLR_EL2: u64,
    pub SP_EL2: u64,
    pub SPSR_EL2: u64,
    pub TCR_EL2: u64,
    pub TPIDR_EL2: u64,
    pub TTBR0_EL2: u64,
    pub VTCR_EL2: u64,
    pub VTTBR_EL2: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_BUS_ERROR {
    pub ValidationBit: WHEA_ARM_BUS_ERROR_VALID_BITS,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u8,
    pub _bitfield4: u16,
    pub _bitfield5: u8,
    pub _bitfield6: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ARM_BUS_ERROR_VALID_BITS {
    pub Anonymous: WHEA_ARM_BUS_ERROR_VALID_BITS_0,
    pub AsUSHORT: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_BUS_ERROR_VALID_BITS_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_CACHE_ERROR {
    pub ValidationBit: WHEA_ARM_CACHE_ERROR_VALID_BITS,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ARM_CACHE_ERROR_VALID_BITS {
    pub Anonymous: WHEA_ARM_CACHE_ERROR_VALID_BITS_0,
    pub AsUSHORT: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_CACHE_ERROR_VALID_BITS_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_MISC_CSR {
    pub MRSEncoding: u16,
    pub Value: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ARM_PROCESSOR_ERROR {
    pub CacheError: WHEA_ARM_CACHE_ERROR,
    pub TlbError: WHEA_ARM_TLB_ERROR,
    pub BusError: WHEA_ARM_BUS_ERROR,
    pub AsULONGLONG: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER {
    pub Version: u16,
    pub RegisterContextType: u16,
    pub RegisterArraySize: u32,
    pub RegisterArray: [u8; 1],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS {
    pub Anonymous: WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS_0,
    pub AsULONG: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_PROCESSOR_ERROR_CONTEXT_INFORMATION_HEADER_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_PROCESSOR_ERROR_INFORMATION {
    pub Version: u8,
    pub Length: u8,
    pub ValidationBit: WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS,
    pub Type: u8,
    pub MultipleError: u16,
    pub Flags: u8,
    pub ErrorInformation: u64,
    pub VirtualFaultAddress: u64,
    pub PhysicalFaultAddress: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS {
    pub Anonymous: WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS_0,
    pub AsUSHORT: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_PROCESSOR_ERROR_INFORMATION_VALID_BITS_0 {
    pub _bitfield: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_PROCESSOR_ERROR_SECTION {
    pub ValidBits: WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS,
    pub ErrorInformationStructures: u16,
    pub ContextInformationStructures: u16,
    pub SectionLength: u32,
    pub ErrorAffinityLevel: u8,
    pub Reserved: [u8; 3],
    pub MPIDR_EL1: u64,
    pub MIDR_EL1: u64,
    pub RunningState: u32,
    pub PSCIState: u32,
    pub Data: [u8; 1],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS {
    pub Anonymous: WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS_0,
    pub AsULONG: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_PROCESSOR_ERROR_SECTION_VALID_BITS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_TLB_ERROR {
    pub ValidationBit: WHEA_ARM_TLB_ERROR_VALID_BITS,
    pub _bitfield1: u8,
    pub _bitfield2: u8,
    pub _bitfield3: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ARM_TLB_ERROR_VALID_BITS {
    pub Anonymous: WHEA_ARM_TLB_ERROR_VALID_BITS_0,
    pub AsUSHORT: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ARM_TLB_ERROR_VALID_BITS_0 {
    pub _bitfield: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_AZCC_ROOT_BUS_ERR_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub MaxBusCountPassed: super::super::super::Win32::Foundation::BOOLEAN,
    pub InvalidBusMSR: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_AZCC_ROOT_BUS_LIST_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RootBusCount: u32,
    pub RootBuses: [u32; 8],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_AZCC_SET_POISON_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Bus: u32,
    pub ReadSuccess: super::super::super::Win32::Foundation::BOOLEAN,
    pub WriteSuccess: super::super::super::Win32::Foundation::BOOLEAN,
    pub IsEnable: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_INJECTION_CAPABILITIES {
    pub Anonymous: WHEA_ERROR_INJECTION_CAPABILITIES_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_INJECTION_CAPABILITIES_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_PACKET_FLAGS {
    pub Anonymous: WHEA_ERROR_PACKET_FLAGS_0,
    pub AsULONG: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_PACKET_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_PACKET_V1 {
    pub Signature: u32,
    pub Flags: WHEA_ERROR_PACKET_FLAGS,
    pub Size: u32,
    pub RawDataLength: u32,
    pub Reserved1: u64,
    pub Context: u64,
    pub ErrorType: WHEA_ERROR_TYPE,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub ErrorSourceId: u32,
    pub ErrorSourceType: super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_TYPE,
    pub Reserved2: u32,
    pub Version: u32,
    pub Cpu: u64,
    pub u: WHEA_ERROR_PACKET_V1_0,
    pub RawDataFormat: WHEA_RAW_DATA_FORMAT,
    pub RawDataOffset: u32,
    pub RawData: [u8; 1],
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_PACKET_V1_0 {
    pub ProcessorError: WHEA_PROCESSOR_GENERIC_ERROR_SECTION,
    pub MemoryError: WHEA_MEMORY_ERROR_SECTION,
    pub NmiError: WHEA_NMI_ERROR_SECTION,
    pub PciExpressError: WHEA_PCIEXPRESS_ERROR_SECTION,
    pub PciXBusError: WHEA_PCIXBUS_ERROR_SECTION,
    pub PciXDeviceError: WHEA_PCIXDEVICE_ERROR_SECTION,
    pub PmemError: WHEA_PMEM_ERROR_SECTION,
}
#[repr(C, packed(1))]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_PACKET_V2 {
    pub Signature: u32,
    pub Version: u32,
    pub Length: u32,
    pub Flags: WHEA_ERROR_PACKET_FLAGS,
    pub ErrorType: WHEA_ERROR_TYPE,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub ErrorSourceId: u32,
    pub ErrorSourceType: super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_TYPE,
    pub NotifyType: windows_sys::core::GUID,
    pub Context: u64,
    pub DataFormat: WHEA_ERROR_PACKET_DATA_FORMAT,
    pub Reserved1: u32,
    pub DataOffset: u32,
    pub DataLength: u32,
    pub PshedDataOffset: u32,
    pub PshedDataLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD {
    pub Header: WHEA_ERROR_RECORD_HEADER,
    pub SectionDescriptor: [WHEA_ERROR_RECORD_SECTION_DESCRIPTOR; 1],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD_HEADER {
    pub Signature: u32,
    pub Revision: WHEA_REVISION,
    pub SignatureEnd: u32,
    pub SectionCount: u16,
    pub Severity: WHEA_ERROR_SEVERITY,
    pub ValidBits: WHEA_ERROR_RECORD_HEADER_VALIDBITS,
    pub Length: u32,
    pub Timestamp: WHEA_TIMESTAMP,
    pub PlatformId: windows_sys::core::GUID,
    pub PartitionId: windows_sys::core::GUID,
    pub CreatorId: windows_sys::core::GUID,
    pub NotifyType: windows_sys::core::GUID,
    pub RecordId: u64,
    pub Flags: WHEA_ERROR_RECORD_HEADER_FLAGS,
    pub PersistenceInfo: WHEA_PERSISTENCE_INFO,
    pub Anonymous: WHEA_ERROR_RECORD_HEADER_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_HEADER_0 {
    pub Anonymous: WHEA_ERROR_RECORD_HEADER_0_0,
    pub Reserved: [u8; 12],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD_HEADER_0_0 {
    pub OsBuildNumber: u32,
    pub Reserved2: [u8; 8],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_HEADER_FLAGS {
    pub Anonymous: WHEA_ERROR_RECORD_HEADER_FLAGS_0,
    pub AsULONG: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD_HEADER_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_HEADER_VALIDBITS {
    pub Anonymous: WHEA_ERROR_RECORD_HEADER_VALIDBITS_0,
    pub AsULONG: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD_HEADER_VALIDBITS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD_SECTION_DESCRIPTOR {
    pub SectionOffset: u32,
    pub SectionLength: u32,
    pub Revision: WHEA_REVISION,
    pub ValidBits: WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS,
    pub Reserved: u8,
    pub Flags: WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS,
    pub SectionType: windows_sys::core::GUID,
    pub FRUId: windows_sys::core::GUID,
    pub SectionSeverity: WHEA_ERROR_SEVERITY,
    pub FRUText: [i8; 20],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS {
    pub Anonymous: WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS_0,
    pub AsULONG: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS {
    pub Anonymous: WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS_0,
    pub AsUCHAR: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECORD_SECTION_DESCRIPTOR_VALIDBITS_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_RECOVERY_INFO_SECTION {
    pub RecoveryKernel: super::super::super::Win32::Foundation::BOOLEAN,
    pub RecoveryAction: WHEA_RECOVERY_ACTION,
    pub RecoveryType: WHEA_RECOVERY_TYPE,
    pub Irql: u8,
    pub RecoverySucceeded: super::super::super::Win32::Foundation::BOOLEAN,
    pub FailureReason: WHEA_RECOVERY_FAILURE_REASON,
    pub ProcessName: [i8; 20],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_SOURCE_CONFIGURATION {
    pub Flags: u32,
    pub Correct: WHEA_ERROR_SOURCE_CORRECT,
    pub Initialize: WHEA_ERROR_SOURCE_INITIALIZE,
    pub CreateRecord: WHEA_ERROR_SOURCE_CREATE_RECORD,
    pub Recover: WHEA_ERROR_SOURCE_RECOVER,
    pub Uninitialize: WHEA_ERROR_SOURCE_UNINITIALIZE,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_SOURCE_OVERRIDE_SETTINGS {
    pub Type: super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_TYPE,
    pub MaxRawDataLength: u32,
    pub NumRecordsToPreallocate: u32,
    pub MaxSectionsPerRecord: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_ERROR_STATUS {
    pub ErrorStatus: u64,
    pub Anonymous: WHEA_ERROR_STATUS_0,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ERROR_STATUS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_ETW_OVERFLOW_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RecordId: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_EVENT_LOG_ENTRY {
    pub Header: WHEA_EVENT_LOG_ENTRY_HEADER,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_EVENT_LOG_ENTRY_FLAGS {
    pub Anonymous: WHEA_EVENT_LOG_ENTRY_FLAGS_0,
    pub AsULONG: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_EVENT_LOG_ENTRY_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_EVENT_LOG_ENTRY_HEADER {
    pub Signature: u32,
    pub Version: u32,
    pub Length: u32,
    pub Type: WHEA_EVENT_LOG_ENTRY_TYPE,
    pub OwnerTag: u32,
    pub Id: WHEA_EVENT_LOG_ENTRY_ID,
    pub Flags: WHEA_EVENT_LOG_ENTRY_FLAGS,
    pub PayloadLength: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_FAILED_ADD_DEFECT_LIST_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_FIRMWARE_ERROR_RECORD_REFERENCE {
    pub Type: u8,
    pub Reserved: [u8; 7],
    pub FirmwareRecordId: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_GENERIC_ERROR {
    pub BlockStatus: WHEA_GENERIC_ERROR_BLOCKSTATUS,
    pub RawDataOffset: u32,
    pub RawDataLength: u32,
    pub DataLength: u32,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub Data: [u8; 1],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_GENERIC_ERROR_BLOCKSTATUS {
    pub Anonymous: WHEA_GENERIC_ERROR_BLOCKSTATUS_0,
    pub AsULONG: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_GENERIC_ERROR_BLOCKSTATUS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_GENERIC_ERROR_DATA_ENTRY_V1 {
    pub SectionType: windows_sys::core::GUID,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub Revision: WHEA_REVISION,
    pub ValidBits: u8,
    pub Flags: u8,
    pub ErrorDataLength: u32,
    pub FRUId: windows_sys::core::GUID,
    pub FRUText: [u8; 20],
    pub Data: [u8; 1],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_GENERIC_ERROR_DATA_ENTRY_V2 {
    pub SectionType: windows_sys::core::GUID,
    pub ErrorSeverity: WHEA_ERROR_SEVERITY,
    pub Revision: WHEA_REVISION,
    pub ValidBits: u8,
    pub Flags: u8,
    pub ErrorDataLength: u32,
    pub FRUId: windows_sys::core::GUID,
    pub FRUText: [u8; 20],
    pub Timestamp: WHEA_TIMESTAMP,
    pub Data: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_IN_USE_PAGE_NOTIFY_FLAGS {
    pub Bits: WHEA_IN_USE_PAGE_NOTIFY_FLAGS_0,
    pub AsUCHAR: u8,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_IN_USE_PAGE_NOTIFY_FLAGS_0 {
    pub _bitfield: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_CORRECTABLE_ERROR_DATA {
    pub ValidBits: WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS,
    pub SocketId: u32,
    pub ChannelId: u32,
    pub DimmSlot: u32,
    pub CorrectableErrorCount: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_CORRECTABLE_ERROR_HEADER {
    pub Version: u16,
    pub Count: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_CORRECTABLE_ERROR_SECTION {
    pub Header: WHEA_MEMORY_CORRECTABLE_ERROR_HEADER,
    pub Data: [WHEA_MEMORY_CORRECTABLE_ERROR_DATA; 1],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_CORRECTABLE_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_ERROR_SECTION {
    pub ValidBits: WHEA_MEMORY_ERROR_SECTION_VALIDBITS,
    pub ErrorStatus: WHEA_ERROR_STATUS,
    pub PhysicalAddress: u64,
    pub PhysicalAddressMask: u64,
    pub Node: u16,
    pub Card: u16,
    pub Module: u16,
    pub Bank: u16,
    pub Device: u16,
    pub Row: u16,
    pub Column: u16,
    pub BitPosition: u16,
    pub RequesterId: u64,
    pub ResponderId: u64,
    pub TargetId: u64,
    pub ErrorType: u8,
    pub Extended: u8,
    pub RankNumber: u16,
    pub CardHandle: u16,
    pub ModuleHandle: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_MEMORY_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_MEMORY_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_MEMORY_THROTTLE_SUMMARY_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_MSR_DUMP_SECTION {
    pub MsrDumpBuffer: u8,
    pub MsrDumpLength: u32,
    pub MsrDumpData: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_NMI_ERROR_SECTION {
    pub Data: [u8; 8],
    pub Flags: WHEA_NMI_ERROR_SECTION_FLAGS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_NMI_ERROR_SECTION_FLAGS {
    pub Anonymous: WHEA_NMI_ERROR_SECTION_FLAGS_0,
    pub AsULONG: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_NMI_ERROR_SECTION_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_OFFLINE_DONE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Address: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_PACKET_LOG_DATA {
    pub LogData: [u8; 36],
    pub ExtraBytes: [u8; 36],
    pub BcParam3: usize,
    pub BcParam4: usize,
    pub LogDataLength: u32,
    pub LogTag: u16,
    pub Reserved: u16,
    pub Flags: WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS {
    pub Anonymous: WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS_0,
    pub AsULONG: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS_0 {
    pub BridgeSecondaryStatus: u16,
    pub BridgeControl: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIEXPRESS_COMMAND_STATUS {
    pub Anonymous: WHEA_PCIEXPRESS_COMMAND_STATUS_0,
    pub AsULONG: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIEXPRESS_COMMAND_STATUS_0 {
    pub Command: u16,
    pub Status: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIEXPRESS_DEVICE_ID {
    pub VendorID: u16,
    pub DeviceID: u16,
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub _bitfield3: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIEXPRESS_ERROR_SECTION {
    pub ValidBits: WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS,
    pub PortType: WHEA_PCIEXPRESS_DEVICE_TYPE,
    pub Version: WHEA_PCIEXPRESS_VERSION,
    pub CommandStatus: WHEA_PCIEXPRESS_COMMAND_STATUS,
    pub Reserved: u32,
    pub DeviceId: WHEA_PCIEXPRESS_DEVICE_ID,
    pub DeviceSerialNumber: u64,
    pub BridgeControlStatus: WHEA_PCIEXPRESS_BRIDGE_CONTROL_STATUS,
    pub ExpressCapability: [u8; 60],
    pub AerInfo: [u8; 96],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIEXPRESS_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIEXPRESS_VERSION {
    pub Anonymous: WHEA_PCIEXPRESS_VERSION_0,
    pub AsULONG: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIEXPRESS_VERSION_0 {
    pub MinorVersion: u8,
    pub MajorVersion: u8,
    pub Reserved: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIE_ADDRESS {
    pub Segment: u32,
    pub Bus: u32,
    pub Device: u32,
    pub Function: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIE_CORRECTABLE_ERROR_DEVICES {
    pub ValidBits: WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS,
    pub Address: WHEA_PCIE_ADDRESS,
    pub Mask: u32,
    pub CorrectableErrorCount: [u32; 32],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS {
    pub Anonymous: WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS_0,
    pub ValidBits: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIE_CORRECTABLE_ERROR_DEVICES_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_PCIE_CORRECTABLE_ERROR_SECTION {
    pub Header: WHEA_PCIE_CORRECTABLE_ERROR_SECTION_HEADER,
    pub Devices: [WHEA_PCIE_CORRECTABLE_ERROR_DEVICES; 1],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIE_CORRECTABLE_ERROR_SECTION_HEADER {
    pub Version: u16,
    pub Count: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIXBUS_COMMAND {
    pub Anonymous: WHEA_PCIXBUS_COMMAND_0,
    pub AsULONGLONG: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIXBUS_COMMAND_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIXBUS_ERROR_SECTION {
    pub ValidBits: WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS,
    pub ErrorStatus: WHEA_ERROR_STATUS,
    pub ErrorType: u16,
    pub BusId: WHEA_PCIXBUS_ID,
    pub Reserved: u32,
    pub BusAddress: u64,
    pub BusData: u64,
    pub BusCommand: WHEA_PCIXBUS_COMMAND,
    pub RequesterId: u64,
    pub CompleterId: u64,
    pub TargetId: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIXBUS_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIXBUS_ID {
    pub Anonymous: WHEA_PCIXBUS_ID_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_PCIXBUS_ID_0 {
    pub BusNumber: u8,
    pub BusSegment: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIXDEVICE_ERROR_SECTION {
    pub ValidBits: WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS,
    pub ErrorStatus: WHEA_ERROR_STATUS,
    pub IdInfo: WHEA_PCIXDEVICE_ID,
    pub MemoryNumber: u32,
    pub IoNumber: u32,
    pub RegisterDataPairs: [WHEA_PCIXDEVICE_REGISTER_PAIR; 1],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIXDEVICE_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIXDEVICE_ID {
    pub VendorId: u16,
    pub DeviceId: u16,
    pub _bitfield1: u32,
    pub _bitfield2: u32,
    pub Reserved2: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PCIXDEVICE_REGISTER_PAIR {
    pub Register: u64,
    pub Data: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_PCI_RECOVERY_SECTION {
    pub SignalType: u8,
    pub RecoveryAttempted: super::super::super::Win32::Foundation::BOOLEAN,
    pub RecoveryStatus: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PERSISTENCE_INFO {
    pub Anonymous: WHEA_PERSISTENCE_INFO_0,
    pub AsULONGLONG: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PERSISTENCE_INFO_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PMEM_ERROR_SECTION {
    pub ValidBits: WHEA_PMEM_ERROR_SECTION_VALIDBITS,
    pub LocationInfo: [u8; 64],
    pub ErrorStatus: WHEA_ERROR_STATUS,
    pub NFITHandle: u32,
    pub PageRangeCount: u32,
    pub PageRange: [WHEA_PMEM_PAGE_RANGE; 1],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PMEM_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PMEM_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PMEM_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PMEM_PAGE_RANGE {
    pub StartingPfn: u64,
    pub PageCount: u64,
    pub MarkedBadBitmap: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PROCESSOR_FAMILY_INFO {
    pub Anonymous: WHEA_PROCESSOR_FAMILY_INFO_0,
    pub AsULONGLONG: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PROCESSOR_FAMILY_INFO_0 {
    pub _bitfield: u32,
    pub NativeModelId: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PROCESSOR_GENERIC_ERROR_SECTION {
    pub ValidBits: WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS,
    pub ProcessorType: u8,
    pub InstructionSet: u8,
    pub ErrorType: u8,
    pub Operation: u8,
    pub Flags: u8,
    pub Level: u8,
    pub Reserved: u16,
    pub CPUVersion: u64,
    pub CPUBrandString: [u8; 128],
    pub ProcessorId: u64,
    pub TargetAddress: u64,
    pub RequesterId: u64,
    pub ResponderId: u64,
    pub InstructionPointer: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PROCESSOR_GENERIC_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PI_CPU_BUSES_INIT_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PI_TRACE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Buffer: [i8; 256],
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_CALLBACKS {
    pub GetAllErrorSources: PSHED_PI_GET_ALL_ERROR_SOURCES,
    pub Reserved: *mut core::ffi::c_void,
    pub GetErrorSourceInfo: PSHED_PI_GET_ERROR_SOURCE_INFO,
    pub SetErrorSourceInfo: PSHED_PI_SET_ERROR_SOURCE_INFO,
    pub EnableErrorSource: PSHED_PI_ENABLE_ERROR_SOURCE,
    pub DisableErrorSource: PSHED_PI_DISABLE_ERROR_SOURCE,
    pub WriteErrorRecord: PSHED_PI_WRITE_ERROR_RECORD,
    pub ReadErrorRecord: PSHED_PI_READ_ERROR_RECORD,
    pub ClearErrorRecord: PSHED_PI_CLEAR_ERROR_RECORD,
    pub RetrieveErrorInfo: PSHED_PI_RETRIEVE_ERROR_INFO,
    pub FinalizeErrorRecord: PSHED_PI_FINALIZE_ERROR_RECORD,
    pub ClearErrorStatus: PSHED_PI_CLEAR_ERROR_STATUS,
    pub AttemptRecovery: PSHED_PI_ATTEMPT_ERROR_RECOVERY,
    pub GetInjectionCapabilities: PSHED_PI_GET_INJECTION_CAPABILITIES,
    pub InjectError: PSHED_PI_INJECT_ERROR,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_DIMM_MISMATCH {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub FirmwareBank: u16,
    pub FirmwareCol: u16,
    pub FirmwareRow: u16,
    pub RetryRdBank: u16,
    pub RetryRdCol: u16,
    pub RetryRdRow: u16,
    pub TaBank: u16,
    pub TaCol: u16,
    pub TaRow: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub EnableError: WHEA_PSHED_PLUGIN_ENABLE_NOTIFY_ERRORS,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_HEARTBEAT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_INIT_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_LOAD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PluginName: [u16; 32],
    pub MajorVersion: u32,
    pub MinorVersion: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_PLATFORM_SUPPORT_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PluginName: [u16; 32],
    pub Supported: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_REGISTRATION_PACKET_V1 {
    pub Length: u32,
    pub Version: u32,
    pub Context: *mut core::ffi::c_void,
    pub FunctionalAreaMask: u32,
    pub Reserved: u32,
    pub Callbacks: WHEA_PSHED_PLUGIN_CALLBACKS,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_REGISTRATION_PACKET_V2 {
    pub Length: u32,
    pub Version: u32,
    pub Context: *mut core::ffi::c_void,
    pub FunctionalAreaMask: u32,
    pub Reserved: u32,
    pub Callbacks: WHEA_PSHED_PLUGIN_CALLBACKS,
    pub PluginHandle: *mut core::ffi::c_void,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_PSHED_PLUGIN_UNLOAD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub PluginName: [u16; 32],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_RECOVERY_ACTION {
    pub Anonymous: WHEA_RECOVERY_ACTION_0,
    pub AsULONG: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_RECOVERY_ACTION_0 {
    pub _bitfield1: u32,
    pub _bitfield2: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_RECOVERY_CONTEXT {
    pub Anonymous: WHEA_RECOVERY_CONTEXT_0,
    pub PartitionId: u64,
    pub VpIndex: u32,
    pub ErrorType: WHEA_RECOVERY_CONTEXT_ERROR_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_RECOVERY_CONTEXT_0 {
    pub MemoryError: WHEA_RECOVERY_CONTEXT_0_0,
    pub PmemError: WHEA_RECOVERY_CONTEXT_0_1,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_RECOVERY_CONTEXT_0_0 {
    pub Address: usize,
    pub Consumed: super::super::super::Win32::Foundation::BOOLEAN,
    pub ErrorCode: u16,
    pub ErrorIpValid: super::super::super::Win32::Foundation::BOOLEAN,
    pub RestartIpValid: super::super::super::Win32::Foundation::BOOLEAN,
    pub ClearPoison: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_RECOVERY_CONTEXT_0_1 {
    pub PmemErrInfo: usize,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_REGISTER_KEY_NOTIFICATION_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS {
    pub Anonymous: WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS_0,
    pub AsULONG: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_REPORT_HW_ERROR_DEVICE_DRIVER_FLAGS_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_REVISION {
    pub Anonymous: WHEA_REVISION_0,
    pub AsUSHORT: u16,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_REVISION_0 {
    pub MinorRevision: u8,
    pub MajorRevision: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_SEA_SECTION {
    pub Esr: u32,
    pub Far: u64,
    pub Par: u64,
    pub WasKernel: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_SEI_SECTION {
    pub Esr: u32,
    pub Far: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_PROGRESS {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub BugCheckCode: u32,
    pub BugCheckProgressSummary: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_MULTIPLE_BUGCHECK_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub IsBugcheckOwner: super::super::super::Win32::Foundation::BOOLEAN,
    pub RecursionCount: u8,
    pub IsBugcheckRecoveryOwner: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Success: super::super::super::Win32::Foundation::BOOLEAN,
    pub Version: u8,
    pub EntryCount: u16,
    pub Data: WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT_0,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE1_EVENT_0 {
    pub DumpPolicy: u8,
    pub Reserved: [u8; 3],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_PHASE2_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub BootId: u32,
    pub Success: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_SEL_BUGCHECK_RECOVERY_STATUS_START_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub StartingIrql: u8,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_SRAR_DETAIL_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub RecoveryContextFlags: u32,
    pub RecoveryContextPa: u64,
    pub PageOfflineStatus: super::super::super::Win32::Foundation::NTSTATUS,
    pub KernelConsumerError: super::super::super::Win32::Foundation::BOOLEAN,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_SRAS_TABLE_ENTRIES_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub LogNumber: u32,
    pub NumberSignals: u32,
    pub Data: [u8; 1],
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_SRAS_TABLE_ERROR {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_SRAS_TABLE_NOT_FOUND {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_ADD_ERR_SRC_FAILED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_MEMORY_ADD_OR_REMOVE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub SocketId: u32,
    pub ChannelId: u32,
    pub DimmSlot: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_PCIE_ADD_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Address: WHEA_PCIE_ADDRESS,
    pub Mask: u32,
    pub Updated: super::super::super::Win32::Foundation::BOOLEAN,
    pub Status: super::super::super::Win32::Foundation::NTSTATUS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_PCIE_REMOVE_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub Address: WHEA_PCIE_ADDRESS,
    pub Mask: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_REGISTRY_CORRUPT_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ThrottleType: WHEA_THROTTLE_TYPE,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_THROTTLE_REG_DATA_IGNORED_EVENT {
    pub WheaEventLogEntry: WHEA_EVENT_LOG_ENTRY,
    pub ThrottleType: WHEA_THROTTLE_TYPE,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_TIMESTAMP {
    pub Anonymous: WHEA_TIMESTAMP_0,
    pub AsLARGE_INTEGER: i64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_TIMESTAMP_0 {
    pub _bitfield: u64,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WHEA_X64_REGISTER_STATE {
    pub Rax: u64,
    pub Rbx: u64,
    pub Rcx: u64,
    pub Rdx: u64,
    pub Rsi: u64,
    pub Rdi: u64,
    pub Rbp: u64,
    pub Rsp: u64,
    pub R8: u64,
    pub R9: u64,
    pub R10: u64,
    pub R11: u64,
    pub R12: u64,
    pub R13: u64,
    pub R14: u64,
    pub R15: u64,
    pub Cs: u16,
    pub Ds: u16,
    pub Ss: u16,
    pub Es: u16,
    pub Fs: u16,
    pub Gs: u16,
    pub Reserved: u32,
    pub Rflags: u64,
    pub Eip: u64,
    pub Cr0: u64,
    pub Cr1: u64,
    pub Cr2: u64,
    pub Cr3: u64,
    pub Cr4: u64,
    pub Cr8: u64,
    pub Gdtr: WHEA128A,
    pub Idtr: WHEA128A,
    pub Ldtr: u16,
    pub Tr: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_X86_REGISTER_STATE {
    pub Eax: u32,
    pub Ebx: u32,
    pub Ecx: u32,
    pub Edx: u32,
    pub Esi: u32,
    pub Edi: u32,
    pub Ebp: u32,
    pub Esp: u32,
    pub Cs: u16,
    pub Ds: u16,
    pub Ss: u16,
    pub Es: u16,
    pub Fs: u16,
    pub Gs: u16,
    pub Eflags: u32,
    pub Eip: u32,
    pub Cr0: u32,
    pub Cr1: u32,
    pub Cr2: u32,
    pub Cr3: u32,
    pub Cr4: u32,
    pub Gdtr: u64,
    pub Idtr: u64,
    pub Ldtr: u16,
    pub Tr: u16,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_BUS_CHECK {
    pub Anonymous: WHEA_XPF_BUS_CHECK_0,
    pub XpfBusCheck: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_BUS_CHECK_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_CACHE_CHECK {
    pub Anonymous: WHEA_XPF_CACHE_CHECK_0,
    pub XpfCacheCheck: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_CACHE_CHECK_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_CONTEXT_INFO {
    pub RegisterContextType: u16,
    pub RegisterDataSize: u16,
    pub MSRAddress: u32,
    pub MmRegisterAddress: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_MCA_SECTION {
    pub VersionNumber: u32,
    pub CpuVendor: WHEA_CPU_VENDOR,
    pub Timestamp: i64,
    pub ProcessorNumber: u32,
    pub GlobalStatus: MCG_STATUS,
    pub InstructionPointer: u64,
    pub BankNumber: u32,
    pub Status: MCI_STATUS,
    pub Address: u64,
    pub Misc: u64,
    pub ExtendedRegisterCount: u32,
    pub ApicId: u32,
    pub Anonymous: WHEA_XPF_MCA_SECTION_0,
    pub GlobalCapability: MCG_CAP,
    pub RecoveryInfo: XPF_RECOVERY_INFO,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_MCA_SECTION_0 {
    pub ExtendedRegisters: [u64; 24],
    pub AMDExtendedRegisters: WHEA_AMD_EXTENDED_REGISTERS,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_MS_CHECK {
    pub Anonymous: WHEA_XPF_MS_CHECK_0,
    pub XpfMsCheck: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_MS_CHECK_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_PROCESSOR_ERROR_SECTION {
    pub ValidBits: WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS,
    pub LocalAPICId: u64,
    pub CpuId: [u8; 48],
    pub VariableInfo: [u8; 1],
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS {
    pub Anonymous: WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS_0,
    pub ValidBits: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_PROCESSOR_ERROR_SECTION_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_PROCINFO {
    pub CheckInfoId: windows_sys::core::GUID,
    pub ValidBits: WHEA_XPF_PROCINFO_VALIDBITS,
    pub CheckInfo: WHEA_XPF_PROCINFO_0,
    pub TargetId: u64,
    pub RequesterId: u64,
    pub ResponderId: u64,
    pub InstructionPointer: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_PROCINFO_0 {
    pub CacheCheck: WHEA_XPF_CACHE_CHECK,
    pub TlbCheck: WHEA_XPF_TLB_CHECK,
    pub BusCheck: WHEA_XPF_BUS_CHECK,
    pub MsCheck: WHEA_XPF_MS_CHECK,
    pub AsULONGLONG: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_PROCINFO_VALIDBITS {
    pub Anonymous: WHEA_XPF_PROCINFO_VALIDBITS_0,
    pub ValidBits: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_PROCINFO_VALIDBITS_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub union WHEA_XPF_TLB_CHECK {
    pub Anonymous: WHEA_XPF_TLB_CHECK_0,
    pub XpfTLBCheck: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct WHEA_XPF_TLB_CHECK_0 {
    pub _bitfield: u64,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XPF_RECOVERY_INFO {
    pub FailureReason: XPF_RECOVERY_INFO_1,
    pub Action: XPF_RECOVERY_INFO_0,
    pub ActionRequired: super::super::super::Win32::Foundation::BOOLEAN,
    pub RecoverySucceeded: super::super::super::Win32::Foundation::BOOLEAN,
    pub RecoveryKernel: super::super::super::Win32::Foundation::BOOLEAN,
    pub Reserved: u8,
    pub Reserved2: u16,
    pub Reserved3: u16,
    pub Reserved4: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XPF_RECOVERY_INFO_0 {
    pub _bitfield: u32,
}
#[repr(C, packed(1))]
#[derive(Clone, Copy)]
pub struct XPF_RECOVERY_INFO_1 {
    pub _bitfield: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct XSAVE_FORMAT {
    pub ControlWord: u16,
    pub StatusWord: u16,
    pub TagWord: u8,
    pub Reserved1: u8,
    pub ErrorOpcode: u16,
    pub ErrorOffset: u32,
    pub ErrorSelector: u16,
    pub Reserved2: u16,
    pub DataOffset: u32,
    pub DataSelector: u16,
    pub Reserved3: u16,
    pub MxCsr: u32,
    pub MxCsr_Mask: u32,
    pub FloatRegisters: [super::super::super::Win32::System::Diagnostics::Debug::M128A; 8],
    pub XmmRegisters: [super::super::super::Win32::System::Diagnostics::Debug::M128A; 8],
    pub Reserved4: [u8; 224],
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct XSTATE_CONTEXT {
    pub Mask: u64,
    pub Length: u32,
    pub Reserved1: u32,
    pub Area: *mut super::super::super::Win32::System::Diagnostics::Debug::XSAVE_AREA,
    pub Reserved2: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub Reserved3: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct XSTATE_SAVE {
    pub Anonymous: XSTATE_SAVE_0,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub union XSTATE_SAVE_0 {
    pub Anonymous: XSTATE_SAVE_0_0,
    pub XStateContext: XSTATE_CONTEXT,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
#[derive(Clone, Copy)]
pub struct XSTATE_SAVE_0_0 {
    pub Reserved1: i64,
    pub Reserved2: u32,
    pub Prev: *mut XSTATE_SAVE,
    pub Reserved3: *mut super::super::super::Win32::System::Diagnostics::Debug::XSAVE_AREA,
    pub Thread: *mut isize,
    pub Reserved4: *mut core::ffi::c_void,
    pub Level: u8,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct ZONE_HEADER {
    pub FreeList: super::super::super::Win32::System::Kernel::SINGLE_LIST_ENTRY,
    pub SegmentList: super::super::super::Win32::System::Kernel::SINGLE_LIST_ENTRY,
    pub BlockSize: u32,
    pub TotalSegmentSize: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_System_Kernel")]
#[derive(Clone, Copy)]
pub struct ZONE_SEGMENT_HEADER {
    pub SegmentList: super::super::super::Win32::System::Kernel::SINGLE_LIST_ENTRY,
    pub Reserved: *mut core::ffi::c_void,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct _EXT_SET_PARAMETERS_V0 {
    pub Version: u32,
    pub Reserved: u32,
    pub NoWakeTolerance: i64,
}
#[cfg(feature = "Wdk_Foundation")]
pub type ALLOCATE_FUNCTION = Option<unsafe extern "system" fn(pooltype: super::super::Foundation::POOL_TYPE, numberofbytes: usize, tag: u32) -> *mut core::ffi::c_void>;
pub type BOOT_DRIVER_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(callbackcontext: *const core::ffi::c_void, classification: BDCB_CALLBACK_TYPE, imageinformation: *mut BDCB_IMAGE_INFORMATION)>;
pub type BOUND_CALLBACK = Option<unsafe extern "system" fn() -> BOUND_CALLBACK_STATUS>;
pub type D3COLD_REQUEST_AUX_POWER = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, auxpowerinmilliwatts: u32, retryinseconds: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type D3COLD_REQUEST_CORE_POWER_RAIL = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, corepowerrailneeded: super::super::super::Win32::Foundation::BOOLEAN)>;
pub type D3COLD_REQUEST_PERST_DELAY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, delayinmicroseconds: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type DEVICE_BUS_SPECIFIC_RESET_HANDLER = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void, bustype: *const windows_sys::core::GUID, resettypeselected: DEVICE_BUS_SPECIFIC_RESET_TYPE, flags: *const BUS_SPECIFIC_RESET_FLAGS, resetparameters: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type DEVICE_CHANGE_COMPLETE_CALLBACK = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
pub type DEVICE_QUERY_BUS_SPECIFIC_RESET_HANDLER = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void, resetinfocount: *mut u32, resetinfosupported: *mut DEVICE_BUS_SPECIFIC_RESET_INFO) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type DEVICE_RESET_COMPLETION = Option<unsafe extern "system" fn(status: super::super::super::Win32::Foundation::NTSTATUS, context: *mut core::ffi::c_void)>;
pub type DEVICE_RESET_HANDLER = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void, resettype: DEVICE_RESET_TYPE, flags: u32, resetparameters: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DMA_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const super::super::Foundation::DEVICE_OBJECT, completioncontext: *const core::ffi::c_void, status: DMA_COMPLETION_STATUS)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type DRIVER_LIST_CONTROL = Option<unsafe extern "system" fn(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, irp: *const super::super::Foundation::IRP, scattergather: *const SCATTER_GATHER_LIST, context: *const core::ffi::c_void)>;
pub type ENABLE_VIRTUALIZATION = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, numvfs: u16, enablevfmigration: super::super::super::Win32::Foundation::BOOLEAN, enablemigrationinterrupt: super::super::super::Win32::Foundation::BOOLEAN, enablevirtualization: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Diagnostics_Etw")]
pub type ETWENABLECALLBACK = Option<unsafe extern "system" fn(sourceid: *const windows_sys::core::GUID, controlcode: u32, level: u8, matchanykeyword: u64, matchallkeyword: u64, filterdata: *const super::super::super::Win32::System::Diagnostics::Etw::EVENT_FILTER_DESCRIPTOR, callbackcontext: *mut core::ffi::c_void)>;
pub type EXPAND_STACK_CALLOUT = Option<unsafe extern "system" fn(parameter: *const core::ffi::c_void)>;
#[cfg(feature = "Wdk_Foundation")]
pub type EXT_CALLBACK = Option<unsafe extern "system" fn(timer: super::super::Foundation::PEX_TIMER, context: *const core::ffi::c_void)>;
pub type EXT_DELETE_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type EX_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(callbackcontext: *const core::ffi::c_void, argument1: *const core::ffi::c_void, argument2: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type FPGA_BUS_SCAN = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type FPGA_CONTROL_CONFIG_SPACE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, enable: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type FPGA_CONTROL_ERROR_REPORTING = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, uncorrectablemask: u32, correctablemask: u32, disableerrorreporting: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type FPGA_CONTROL_LINK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, enable: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type FREE_FUNCTION = Option<unsafe extern "system" fn(buffer: *const core::ffi::c_void)>;
pub type FWMI_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(wnode: *mut core::ffi::c_void, context: *mut core::ffi::c_void)>;
pub type GET_D3COLD_CAPABILITY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, d3coldsupported: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type GET_D3COLD_LAST_TRANSITION_STATUS = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, lasttransitionstatus: *mut D3COLD_LAST_TRANSITION_STATUS)>;
pub type GET_DEVICE_RESET_STATUS = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void, isresetting: *mut super::super::super::Win32::Foundation::BOOLEAN, resettypeselected: *mut DEVICE_BUS_SPECIFIC_RESET_TYPE, flags: *mut DEVICE_RESET_STATUS_FLAGS) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type GET_DMA_ADAPTER = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, devicedescriptor: *const DEVICE_DESCRIPTION, numberofmapregisters: *mut u32) -> *mut DMA_ADAPTER>;
#[cfg(feature = "Win32_System_Power")]
pub type GET_IDLE_WAKE_INFO = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, systempowerstate: super::super::super::Win32::System::Power::SYSTEM_POWER_STATE, deepestwakeabledstate: *mut DEVICE_WAKE_DEPTH) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type GET_SDEV_IDENTIFIER = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void) -> u64>;
pub type GET_SET_DEVICE_DATA = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, datatype: u32, buffer: *mut core::ffi::c_void, offset: u32, length: u32) -> u32>;
pub type GET_UPDATED_BUS_RESOURCE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, updatedresourcelist: *mut *mut CM_RESOURCE_LIST, updatedtranslatedresourcelist: *mut *mut CM_RESOURCE_LIST) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type GET_VIRTUAL_DEVICE_DATA = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, virtualfunction: u16, buffer: *mut core::ffi::c_void, offset: u32, length: u32) -> u32>;
pub type GET_VIRTUAL_DEVICE_LOCATION = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, virtualfunction: u16, segmentnumber: *mut u16, busnumber: *mut u8, functionnumber: *mut u8) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type GET_VIRTUAL_DEVICE_RESOURCES = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, capturedbusnumbers: *mut u8)>;
pub type GET_VIRTUAL_FUNCTION_PROBED_BARS = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, baseregistervalues: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type HVL_WHEA_ERROR_NOTIFICATION = Option<unsafe extern "system" fn(recoverycontext: *const WHEA_RECOVERY_CONTEXT, platformdirected: super::super::super::Win32::Foundation::BOOLEAN, poisoned: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IOMMU_DEVICE_CREATE = Option<unsafe extern "system" fn(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, deviceconfig: *const IOMMU_DEVICE_CREATION_CONFIGURATION, dmadeviceout: *mut *mut super::super::Foundation::IOMMU_DMA_DEVICE) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_DEVICE_DELETE = Option<unsafe extern "system" fn(dmadevice: *const super::super::Foundation::IOMMU_DMA_DEVICE) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IOMMU_DEVICE_FAULT_HANDLER = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, faultinformation: *mut FAULT_INFORMATION)>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_DEVICE_QUERY_DOMAIN_TYPES = Option<unsafe extern "system" fn(dmadevice: *const super::super::Foundation::IOMMU_DMA_DEVICE, availabledomains: *mut u32)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IOMMU_DOMAIN_ATTACH_DEVICE = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN, physicaldeviceobject: *const super::super::Foundation::DEVICE_OBJECT, inputmappingidbase: u32, mappingcount: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_DOMAIN_ATTACH_DEVICE_EX = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN, dmadevice: *const super::super::Foundation::IOMMU_DMA_DEVICE) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_DOMAIN_CONFIGURE = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN, configuration: *const DOMAIN_CONFIGURATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_DOMAIN_CREATE = Option<unsafe extern "system" fn(osmanagedpagetable: super::super::super::Win32::Foundation::BOOLEAN, domainout: *mut *mut super::super::Foundation::IOMMU_DMA_DOMAIN) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_DOMAIN_CREATE_EX = Option<unsafe extern "system" fn(domaintype: IOMMU_DMA_DOMAIN_TYPE, flags: IOMMU_DMA_DOMAIN_CREATION_FLAGS, logicalallocatorconfig: *const IOMMU_DMA_LOGICAL_ALLOCATOR_CONFIG, reservedregions: *const IOMMU_DMA_RESERVED_REGION, domainout: *mut *mut super::super::Foundation::IOMMU_DMA_DOMAIN) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_DOMAIN_DELETE = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IOMMU_DOMAIN_DETACH_DEVICE = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN, physicaldeviceobject: *const super::super::Foundation::DEVICE_OBJECT, inputmappingid: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_DOMAIN_DETACH_DEVICE_EX = Option<unsafe extern "system" fn(dmadevice: *const super::super::Foundation::IOMMU_DMA_DEVICE) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_FLUSH_DOMAIN = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_FLUSH_DOMAIN_VA_LIST = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN, lastlevel: super::super::super::Win32::Foundation::BOOLEAN, number: u32, valist: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type IOMMU_FREE_RESERVED_LOGICAL_ADDRESS_RANGE = Option<unsafe extern "system" fn(logicaladdresstoken: *const IOMMU_DMA_LOGICAL_ADDRESS_TOKEN) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type IOMMU_INTERFACE_STATE_CHANGE_CALLBACK = Option<unsafe extern "system" fn(statechange: *const IOMMU_INTERFACE_STATE_CHANGE, context: *const core::ffi::c_void)>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_MAP_IDENTITY_RANGE = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN, permissions: u32, mdl: *const super::super::Foundation::MDL) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_MAP_IDENTITY_RANGE_EX = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN, permissions: u32, physicaladdresstomap: *const IOMMU_MAP_PHYSICAL_ADDRESS) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_MAP_LOGICAL_RANGE = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN, permissions: u32, mdl: *const super::super::Foundation::MDL, logicaladdress: u64) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_MAP_LOGICAL_RANGE_EX = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN, permissions: u32, physicaladdresstomap: *const IOMMU_MAP_PHYSICAL_ADDRESS, explicitlogicaladdress: *const u64, minlogicaladdress: *const u64, maxlogicaladdress: *const u64, logicaladdressout: *mut u64) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_MAP_RESERVED_LOGICAL_RANGE = Option<unsafe extern "system" fn(logicaladdresstoken: *mut IOMMU_DMA_LOGICAL_ADDRESS_TOKEN, offset: usize, permissions: u32, physicaladdresstomap: *const IOMMU_MAP_PHYSICAL_ADDRESS, mappedsegment: *mut IOMMU_DMA_LOGICAL_ADDRESS_TOKEN_MAPPED_SEGMENT) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IOMMU_QUERY_INPUT_MAPPINGS = Option<unsafe extern "system" fn(physicaldeviceobject: *const super::super::Foundation::DEVICE_OBJECT, buffer: *mut INPUT_MAPPING_ELEMENT, bufferlength: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_REGISTER_INTERFACE_STATE_CHANGE_CALLBACK = Option<unsafe extern "system" fn(statechangecallback: PIOMMU_INTERFACE_STATE_CHANGE_CALLBACK, context: *const core::ffi::c_void, dmadevice: *const super::super::Foundation::IOMMU_DMA_DEVICE, statefields: *const IOMMU_INTERFACE_STATE_CHANGE_FIELDS) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_RESERVE_LOGICAL_ADDRESS_RANGE = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN, size: usize, explicitlogicaladdress: *const u64, minlogicaladdress: *const u64, maxlogicaladdress: *const u64, logicaladdresstoken: *mut *mut IOMMU_DMA_LOGICAL_ADDRESS_TOKEN) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IOMMU_SET_DEVICE_FAULT_REPORTING = Option<unsafe extern "system" fn(physicaldeviceobject: *const super::super::Foundation::DEVICE_OBJECT, inputmappingidbase: u32, enable: super::super::super::Win32::Foundation::BOOLEAN, faultconfig: *const DEVICE_FAULT_CONFIGURATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_SET_DEVICE_FAULT_REPORTING_EX = Option<unsafe extern "system" fn(dmadevice: *const super::super::Foundation::IOMMU_DMA_DEVICE, inputmappingidbase: u32, enable: super::super::super::Win32::Foundation::BOOLEAN, faultconfig: *const DEVICE_FAULT_CONFIGURATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_UNMAP_IDENTITY_RANGE = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN, mdl: *const super::super::Foundation::MDL) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_UNMAP_IDENTITY_RANGE_EX = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN, mappedphysicaladdress: *const IOMMU_MAP_PHYSICAL_ADDRESS) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_UNMAP_LOGICAL_RANGE = Option<unsafe extern "system" fn(domain: *const super::super::Foundation::IOMMU_DMA_DOMAIN, logicaladdress: u64, numberofpages: u64) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type IOMMU_UNMAP_RESERVED_LOGICAL_RANGE = Option<unsafe extern "system" fn(mappedsegment: *mut IOMMU_DMA_LOGICAL_ADDRESS_TOKEN_MAPPED_SEGMENT) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type IOMMU_UNREGISTER_INTERFACE_STATE_CHANGE_CALLBACK = Option<unsafe extern "system" fn(statechangecallback: PIOMMU_INTERFACE_STATE_CHANGE_CALLBACK, dmadevice: *const super::super::Foundation::IOMMU_DMA_DEVICE) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IO_COMPLETION_ROUTINE = Option<unsafe extern "system" fn(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, irp: *const super::super::Foundation::IRP, context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IO_CSQ_ACQUIRE_LOCK = Option<unsafe extern "system" fn(csq: *const IO_CSQ, irql: *mut u8)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IO_CSQ_COMPLETE_CANCELED_IRP = Option<unsafe extern "system" fn(csq: *const IO_CSQ, irp: *const super::super::Foundation::IRP)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IO_CSQ_INSERT_IRP = Option<unsafe extern "system" fn(csq: *const IO_CSQ, irp: *const super::super::Foundation::IRP)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IO_CSQ_INSERT_IRP_EX = Option<unsafe extern "system" fn(csq: *const IO_CSQ, irp: *const super::super::Foundation::IRP, insertcontext: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IO_CSQ_PEEK_NEXT_IRP = Option<unsafe extern "system" fn(csq: *const IO_CSQ, irp: *const super::super::Foundation::IRP, peekcontext: *const core::ffi::c_void) -> *mut super::super::Foundation::IRP>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IO_CSQ_RELEASE_LOCK = Option<unsafe extern "system" fn(csq: *const IO_CSQ, irql: u8)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IO_CSQ_REMOVE_IRP = Option<unsafe extern "system" fn(csq: *const IO_CSQ, irp: *const super::super::Foundation::IRP)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IO_DPC_ROUTINE = Option<unsafe extern "system" fn(dpc: *const super::super::Foundation::KDPC, deviceobject: *const super::super::Foundation::DEVICE_OBJECT, irp: *mut super::super::Foundation::IRP, context: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IO_PERSISTED_MEMORY_ENUMERATION_CALLBACK = Option<unsafe extern "system" fn(driverobject: *const super::super::Foundation::DRIVER_OBJECT, physicaldeviceobject: *const super::super::Foundation::DEVICE_OBJECT, physicaldeviceid: *const super::super::super::Win32::Foundation::UNICODE_STRING, datatag: *const u16, dataversion: *const u32, context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type IO_SESSION_NOTIFICATION_FUNCTION = Option<unsafe extern "system" fn(sessionobject: *const core::ffi::c_void, ioobject: *const core::ffi::c_void, event: u32, context: *const core::ffi::c_void, notificationpayload: *const core::ffi::c_void, payloadlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IO_TIMER_ROUTINE = Option<unsafe extern "system" fn(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, context: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type IO_WORKITEM_ROUTINE = Option<unsafe extern "system" fn(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, context: *const core::ffi::c_void)>;
#[cfg(feature = "Wdk_Foundation")]
pub type IO_WORKITEM_ROUTINE_EX = Option<unsafe extern "system" fn(ioobject: *const core::ffi::c_void, context: *const core::ffi::c_void, ioworkitem: super::super::Foundation::PIO_WORKITEM)>;
pub type KBUGCHECK_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(buffer: *mut core::ffi::c_void, length: u32)>;
#[cfg(feature = "Win32_System_Kernel")]
pub type KBUGCHECK_REASON_CALLBACK_ROUTINE = Option<unsafe extern "system" fn(reason: KBUGCHECK_CALLBACK_REASON, record: *const KBUGCHECK_REASON_CALLBACK_RECORD, reasonspecificdata: *mut core::ffi::c_void, reasonspecificdatalength: u32)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
pub type KDEFERRED_ROUTINE = Option<unsafe extern "system" fn(dpc: *const super::super::Foundation::KDPC, deferredcontext: *const core::ffi::c_void, systemargument1: *const core::ffi::c_void, systemargument2: *const core::ffi::c_void)>;
pub type KIPI_BROADCAST_WORKER = Option<unsafe extern "system" fn(argument: usize) -> usize>;
pub type KMESSAGE_SERVICE_ROUTINE = Option<unsafe extern "system" fn(interrupt: *const isize, servicecontext: *const core::ffi::c_void, messageid: u32) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type KSERVICE_ROUTINE = Option<unsafe extern "system" fn(interrupt: *const isize, servicecontext: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type KSTART_ROUTINE = Option<unsafe extern "system" fn(startcontext: *const core::ffi::c_void)>;
pub type KSYNCHRONIZE_ROUTINE = Option<unsafe extern "system" fn(synchronizecontext: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type MM_MDL_ROUTINE = Option<unsafe extern "system" fn(drivercontext: *const core::ffi::c_void, mappedva: *const core::ffi::c_void)>;
pub type NMI_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, handled: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type NPEM_CONTROL_ENABLE_DISABLE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, enablenpem: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type NPEM_CONTROL_QUERY_CONTROL = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> u32>;
pub type NPEM_CONTROL_QUERY_STANDARD_CAPABILITIES = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, standardcapabilities: *mut NPEM_CAPABILITY_STANDARD) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type NPEM_CONTROL_SET_STANDARD_CONTROL = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, standardcontrol: NPEM_CONTROL_STANDARD_CONTROL_BIT, set: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_Security")]
pub type NTFS_DEREF_EXPORTED_SECURITY_DESCRIPTOR = Option<unsafe extern "system" fn(vcb: *const core::ffi::c_void, securitydescriptor: super::super::super::Win32::Security::PSECURITY_DESCRIPTOR)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PALLOCATE_ADAPTER_CHANNEL = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const super::super::Foundation::DEVICE_OBJECT, numberofmapregisters: u32, executionroutine: super::super::Foundation::DRIVER_CONTROL, context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PALLOCATE_ADAPTER_CHANNEL_EX = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const super::super::Foundation::DEVICE_OBJECT, dmatransfercontext: *const core::ffi::c_void, numberofmapregisters: u32, flags: u32, executionroutine: super::super::Foundation::DRIVER_CONTROL, executioncontext: *const core::ffi::c_void, mapregisterbase: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PALLOCATE_COMMON_BUFFER = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, length: u32, logicaladdress: *mut i64, cacheenabled: super::super::super::Win32::Foundation::BOOLEAN) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PALLOCATE_COMMON_BUFFER_EX = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, maximumaddress: *const i64, length: u32, logicaladdress: *mut i64, cacheenabled: super::super::super::Win32::Foundation::BOOLEAN, preferrednode: u32) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PALLOCATE_COMMON_BUFFER_VECTOR = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, lowaddress: i64, highaddress: i64, cachetype: MEMORY_CACHING_TYPE, idealnode: u32, flags: u32, numberofelements: u32, sizeofelements: u64, vectorout: *mut *mut super::super::Foundation::DMA_COMMON_BUFFER_VECTOR) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PALLOCATE_COMMON_BUFFER_WITH_BOUNDS = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, minimumaddress: *const i64, maximumaddress: *const i64, length: u32, flags: u32, cachetype: *const MEMORY_CACHING_TYPE, preferrednode: u32, logicaladdress: *mut i64) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PALLOCATE_DOMAIN_COMMON_BUFFER = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, domainhandle: super::super::super::Win32::Foundation::HANDLE, maximumaddress: *const i64, length: u32, flags: u32, cachetype: *const MEMORY_CACHING_TYPE, preferrednode: u32, logicaladdress: *mut i64, virtualaddress: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PALLOCATE_FUNCTION = Option<unsafe extern "system" fn() -> *mut core::ffi::c_void>;
pub type PALLOCATE_FUNCTION_EX = Option<unsafe extern "system" fn() -> *mut core::ffi::c_void>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PARBITER_HANDLER = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, action: ARBITER_ACTION, parameters: *mut ARBITER_PARAMETERS) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PBOOT_DRIVER_CALLBACK_FUNCTION = Option<unsafe extern "system" fn()>;
pub type PBOUND_CALLBACK = Option<unsafe extern "system" fn() -> BOUND_CALLBACK_STATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PBUILD_MDL_FROM_SCATTER_GATHER_LIST = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, scattergather: *const SCATTER_GATHER_LIST, originalmdl: *const super::super::Foundation::MDL, targetmdl: *mut *mut super::super::Foundation::MDL) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PBUILD_SCATTER_GATHER_LIST = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const super::super::Foundation::DEVICE_OBJECT, mdl: *const super::super::Foundation::MDL, currentva: *const core::ffi::c_void, length: u32, executionroutine: DRIVER_LIST_CONTROL, context: *const core::ffi::c_void, writetodevice: super::super::super::Win32::Foundation::BOOLEAN, scattergatherbuffer: *const core::ffi::c_void, scattergatherlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PBUILD_SCATTER_GATHER_LIST_EX = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const super::super::Foundation::DEVICE_OBJECT, dmatransfercontext: *const core::ffi::c_void, mdl: *const super::super::Foundation::MDL, offset: u64, length: u32, flags: u32, executionroutine: DRIVER_LIST_CONTROL, context: *const core::ffi::c_void, writetodevice: super::super::super::Win32::Foundation::BOOLEAN, scattergatherbuffer: *const core::ffi::c_void, scattergatherlength: u32, dmacompletionroutine: PDMA_COMPLETION_ROUTINE, completioncontext: *const core::ffi::c_void, scattergatherlist: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PCALCULATE_SCATTER_GATHER_LIST_SIZE = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const super::super::Foundation::MDL, currentva: *const core::ffi::c_void, length: u32, scattergatherlistsize: *mut u32, pnumberofmapregisters: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PCALLBACK_FUNCTION = Option<unsafe extern "system" fn()>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PCANCEL_ADAPTER_CHANNEL = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const super::super::Foundation::DEVICE_OBJECT, dmatransfercontext: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PCANCEL_MAPPED_TRANSFER = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, dmatransfercontext: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PCI_ERROR_HANDLER_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PCI_EXPRESS_ENTER_LINK_QUIESCENT_MODE = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PCI_EXPRESS_EXIT_LINK_QUIESCENT_MODE = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PCI_EXPRESS_WAKE_CONTROL = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, enablewake: super::super::super::Win32::Foundation::BOOLEAN)>;
pub type PCI_IS_DEVICE_PRESENT = Option<unsafe extern "system" fn(vendorid: u16, deviceid: u16, revisionid: u8, subvendorid: u16, subsystemid: u16, flags: u32) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PCI_IS_DEVICE_PRESENT_EX = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, parameters: *const PCI_DEVICE_PRESENCE_PARAMETERS) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PCI_LINE_TO_PIN = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, pcinewdata: *const PCI_COMMON_CONFIG, pciolddata: *const PCI_COMMON_CONFIG)>;
pub type PCI_MSIX_GET_ENTRY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, tableentry: u32, messagenumber: *mut u32, masked: *mut super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PCI_MSIX_GET_TABLE_SIZE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, tablesize: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PCI_MSIX_MASKUNMASK_ENTRY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, tableentry: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PCI_MSIX_SET_ENTRY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, tableentry: u32, messagenumber: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PCI_PIN_TO_LINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, pcidata: *const PCI_COMMON_CONFIG)>;
pub type PCI_PREPARE_MULTISTAGE_RESUME = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type PCI_READ_WRITE_CONFIG = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, busoffset: u32, slot: u32, buffer: *const core::ffi::c_void, offset: u32, length: u32) -> u32>;
pub type PCI_ROOT_BUS_CAPABILITY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, hardwarecapability: *mut PCI_ROOT_BUS_HARDWARE_CAPABILITY)>;
pub type PCI_SET_ACS = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, enablesourcevalidation: PCI_ACS_BIT, enabletranslationblocking: PCI_ACS_BIT, enablep2prequestredirect: PCI_ACS_BIT, enablecompletionredirect: PCI_ACS_BIT, enableupstreamforwarding: PCI_ACS_BIT, enableegresscontrol: PCI_ACS_BIT, enabledirecttranslatedp2p: PCI_ACS_BIT) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PCI_SET_ACS2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, scenariostomodify: u32, scenariostate: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PCI_SET_ATS = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, enableats: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_Storage_FileSystem", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PCLFS_CLIENT_ADVANCE_TAIL_CALLBACK = Option<unsafe extern "system" fn(logfile: *const super::super::Foundation::FILE_OBJECT, targetlsn: *const super::super::super::Win32::Storage::FileSystem::CLS_LSN, clientdata: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PCLFS_CLIENT_LFF_HANDLER_COMPLETE_CALLBACK = Option<unsafe extern "system" fn(logfile: *const super::super::Foundation::FILE_OBJECT, operationstatus: super::super::super::Win32::Foundation::NTSTATUS, logispinned: super::super::super::Win32::Foundation::BOOLEAN, clientdata: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PCLFS_CLIENT_LOG_UNPINNED_CALLBACK = Option<unsafe extern "system" fn(logfile: *const super::super::Foundation::FILE_OBJECT, clientdata: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PCLFS_SET_LOG_SIZE_COMPLETE_CALLBACK = Option<unsafe extern "system" fn(logfile: *const super::super::Foundation::FILE_OBJECT, operationstatus: super::super::super::Win32::Foundation::NTSTATUS, clientdata: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PCONFIGURE_ADAPTER_CHANNEL = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, functionnumber: u32, context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PCRASHDUMP_POWER_ON = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PCREATE_COMMON_BUFFER_FROM_MDL = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const super::super::Foundation::MDL, extendedconfigs: *const DMA_COMMON_BUFFER_EXTENDED_CONFIGURATION, extendedconfigscount: u32, logicaladdress: *mut i64) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PCREATE_PROCESS_NOTIFY_ROUTINE = Option<unsafe extern "system" fn(parentid: super::super::super::Win32::Foundation::HANDLE, processid: super::super::super::Win32::Foundation::HANDLE, create: super::super::super::Win32::Foundation::BOOLEAN)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power", feature = "Win32_System_WindowsProgramming"))]
pub type PCREATE_PROCESS_NOTIFY_ROUTINE_EX = Option<unsafe extern "system" fn(process: super::super::Foundation::PEPROCESS, processid: super::super::super::Win32::Foundation::HANDLE, createinfo: *mut PS_CREATE_NOTIFY_INFO)>;
pub type PCREATE_THREAD_NOTIFY_ROUTINE = Option<unsafe extern "system" fn(processid: super::super::super::Win32::Foundation::HANDLE, threadid: super::super::super::Win32::Foundation::HANDLE, create: super::super::super::Win32::Foundation::BOOLEAN)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
pub type PCW_CALLBACK = Option<unsafe extern "system" fn(r#type: PCW_CALLBACK_TYPE, info: *const PCW_CALLBACK_INFORMATION, context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PD3COLD_REQUEST_AUX_POWER = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PD3COLD_REQUEST_CORE_POWER_RAIL = Option<unsafe extern "system" fn()>;
pub type PD3COLD_REQUEST_PERST_DELAY = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PDEBUG_DEVICE_FOUND_FUNCTION = Option<unsafe extern "system" fn(device: *mut DEBUG_DEVICE_DESCRIPTOR) -> KD_CALLBACK_ACTION>;
#[cfg(feature = "Win32_System_Kernel")]
pub type PDEBUG_PRINT_CALLBACK = Option<unsafe extern "system" fn(output: *const super::super::super::Win32::System::Kernel::STRING, componentid: u32, level: u32)>;
pub type PDEVICE_BUS_SPECIFIC_RESET_HANDLER = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PDEVICE_CHANGE_COMPLETE_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PDEVICE_NOTIFY_CALLBACK = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: u32)>;
pub type PDEVICE_NOTIFY_CALLBACK2 = Option<unsafe extern "system" fn(notificationcontext: *mut core::ffi::c_void, notifycode: u32)>;
pub type PDEVICE_QUERY_BUS_SPECIFIC_RESET_HANDLER = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PDEVICE_RESET_COMPLETION = Option<unsafe extern "system" fn()>;
pub type PDEVICE_RESET_HANDLER = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PDMA_COMPLETION_ROUTINE = Option<unsafe extern "system" fn()>;
pub type PDRIVER_CMC_EXCEPTION_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, cmclog: *const MCA_EXCEPTION)>;
pub type PDRIVER_CPE_EXCEPTION_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, cmclog: *const MCA_EXCEPTION)>;
pub type PDRIVER_EXCPTN_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, banklog: *const MCA_EXCEPTION)>;
pub type PDRIVER_VERIFIER_THUNK_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> usize>;
pub type PENABLE_VIRTUALIZATION = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PETWENABLECALLBACK = Option<unsafe extern "system" fn()>;
pub type PEXPAND_STACK_CALLOUT = Option<unsafe extern "system" fn()>;
pub type PEXT_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PEXT_DELETE_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PEX_CALLBACK_FUNCTION = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLUSH_ADAPTER_BUFFERS = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const super::super::Foundation::MDL, mapregisterbase: *const core::ffi::c_void, currentva: *const core::ffi::c_void, length: u32, writetodevice: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLUSH_ADAPTER_BUFFERS_EX = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const super::super::Foundation::MDL, mapregisterbase: *const core::ffi::c_void, offset: u64, length: u32, writetodevice: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFLUSH_DMA_BUFFER = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const super::super::Foundation::MDL, readoperation: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFNFTH = Option<unsafe extern "system" fn(systemfirmwaretableinfo: *mut SYSTEM_FIRMWARE_TABLE_INFORMATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFN_IN_USE_PAGE_OFFLINE_NOTIFY = Option<unsafe extern "system" fn(page: u32, flags: super::super::super::Win32::Foundation::BOOLEAN, poisoned: super::super::super::Win32::Foundation::BOOLEAN, context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PFN_NT_COMMIT_TRANSACTION = Option<unsafe extern "system" fn(transactionhandle: super::super::super::Win32::Foundation::HANDLE, wait: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type PFN_NT_CREATE_TRANSACTION = Option<unsafe extern "system" fn(transactionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, uow: *const windows_sys::core::GUID, tmhandle: super::super::super::Win32::Foundation::HANDLE, createoptions: u32, isolationlevel: u32, isolationflags: u32, timeout: *const i64, description: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type PFN_NT_OPEN_TRANSACTION = Option<unsafe extern "system" fn(transactionhandle: *mut super::super::super::Win32::Foundation::HANDLE, desiredaccess: u32, objectattributes: *const super::super::Foundation::OBJECT_ATTRIBUTES, uow: *const windows_sys::core::GUID, tmhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_SystemServices")]
pub type PFN_NT_QUERY_INFORMATION_TRANSACTION = Option<unsafe extern "system" fn(transactionhandle: super::super::super::Win32::Foundation::HANDLE, transactioninformationclass: super::super::super::Win32::System::SystemServices::TRANSACTION_INFORMATION_CLASS, transactioninformation: *mut core::ffi::c_void, transactioninformationlength: u32, returnlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFN_NT_ROLLBACK_TRANSACTION = Option<unsafe extern "system" fn(transactionhandle: super::super::super::Win32::Foundation::HANDLE, wait: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_SystemServices")]
pub type PFN_NT_SET_INFORMATION_TRANSACTION = Option<unsafe extern "system" fn(transactionhandle: super::super::super::Win32::Foundation::HANDLE, transactioninformationclass: super::super::super::Win32::System::SystemServices::TRANSACTION_INFORMATION_CLASS, transactioninformation: *const core::ffi::c_void, transactioninformationlength: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFN_RTL_IS_NTDDI_VERSION_AVAILABLE = Option<unsafe extern "system" fn(version: u32) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PFN_RTL_IS_SERVICE_PACK_VERSION_INSTALLED = Option<unsafe extern "system" fn(version: u32) -> super::super::super::Win32::Foundation::BOOLEAN>;
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type PFN_WHEA_HIGH_IRQL_LOG_SEL_EVENT_HANDLER = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, osselrecord: *const super::super::super::Win32::System::Diagnostics::Debug::IPMI_OS_SEL_RECORD) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFPGA_BUS_SCAN = Option<unsafe extern "system" fn()>;
pub type PFPGA_CONTROL_CONFIG_SPACE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFPGA_CONTROL_ERROR_REPORTING = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PFPGA_CONTROL_LINK = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFREE_ADAPTER_CHANNEL = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFREE_ADAPTER_OBJECT = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, allocationaction: IO_ALLOCATION_ACTION)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFREE_COMMON_BUFFER = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, length: u32, logicaladdress: i64, virtualaddress: *const core::ffi::c_void, cacheenabled: super::super::super::Win32::Foundation::BOOLEAN)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFREE_COMMON_BUFFER_FROM_VECTOR = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, vector: *const super::super::Foundation::DMA_COMMON_BUFFER_VECTOR, index: u32)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFREE_COMMON_BUFFER_VECTOR = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, vector: *const super::super::Foundation::DMA_COMMON_BUFFER_VECTOR)>;
pub type PFREE_FUNCTION_EX = Option<unsafe extern "system" fn()>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PFREE_MAP_REGISTERS = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mapregisterbase: *mut core::ffi::c_void, numberofmapregisters: u32)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PGET_COMMON_BUFFER_FROM_VECTOR_BY_INDEX = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, vector: *const super::super::Foundation::DMA_COMMON_BUFFER_VECTOR, index: u32, virtualaddressout: *mut *mut core::ffi::c_void, logicaladdressout: *mut i64)>;
pub type PGET_D3COLD_CAPABILITY = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PGET_D3COLD_LAST_TRANSITION_STATUS = Option<unsafe extern "system" fn()>;
pub type PGET_DEVICE_RESET_STATUS = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PGET_DMA_ADAPTER = Option<unsafe extern "system" fn() -> *mut DMA_ADAPTER>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PGET_DMA_ADAPTER_INFO = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, adapterinfo: *mut DMA_ADAPTER_INFO) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PGET_DMA_ALIGNMENT = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER) -> u32>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PGET_DMA_DOMAIN = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER) -> super::super::super::Win32::Foundation::HANDLE>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PGET_DMA_TRANSFER_INFO = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const super::super::Foundation::MDL, offset: u64, length: u32, writeonly: super::super::super::Win32::Foundation::BOOLEAN, transferinfo: *mut DMA_TRANSFER_INFO) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PGET_IDLE_WAKE_INFO = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PGET_LOCATION_STRING = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, locationstrings: *mut windows_sys::core::PWSTR) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PGET_SCATTER_GATHER_LIST = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const super::super::Foundation::DEVICE_OBJECT, mdl: *const super::super::Foundation::MDL, currentva: *const core::ffi::c_void, length: u32, executionroutine: DRIVER_LIST_CONTROL, context: *const core::ffi::c_void, writetodevice: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PGET_SCATTER_GATHER_LIST_EX = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, deviceobject: *const super::super::Foundation::DEVICE_OBJECT, dmatransfercontext: *const core::ffi::c_void, mdl: *const super::super::Foundation::MDL, offset: u64, length: u32, flags: u32, executionroutine: DRIVER_LIST_CONTROL, context: *const core::ffi::c_void, writetodevice: super::super::super::Win32::Foundation::BOOLEAN, dmacompletionroutine: PDMA_COMPLETION_ROUTINE, completioncontext: *const core::ffi::c_void, scattergatherlist: *mut *mut SCATTER_GATHER_LIST) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PGET_SDEV_IDENTIFIER = Option<unsafe extern "system" fn() -> u64>;
pub type PGET_SET_DEVICE_DATA = Option<unsafe extern "system" fn() -> u32>;
pub type PGET_UPDATED_BUS_RESOURCE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PGET_VIRTUAL_DEVICE_DATA = Option<unsafe extern "system" fn() -> u32>;
pub type PGET_VIRTUAL_DEVICE_LOCATION = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PGET_VIRTUAL_DEVICE_RESOURCES = Option<unsafe extern "system" fn()>;
pub type PGET_VIRTUAL_FUNCTION_PROBED_BARS = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PGPE_CLEAR_STATUS = Option<unsafe extern "system" fn(param0: *mut super::super::Foundation::DEVICE_OBJECT, param1: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PGPE_CLEAR_STATUS2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, objectcontext: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PGPE_CONNECT_VECTOR = Option<unsafe extern "system" fn(param0: *mut super::super::Foundation::DEVICE_OBJECT, param1: u32, param2: KINTERRUPT_MODE, param3: super::super::super::Win32::Foundation::BOOLEAN, param4: PGPE_SERVICE_ROUTINE, param5: *mut core::ffi::c_void, param6: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PGPE_CONNECT_VECTOR2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, gpenumber: u32, mode: KINTERRUPT_MODE, shareable: super::super::super::Win32::Foundation::BOOLEAN, serviceroutine: PGPE_SERVICE_ROUTINE, servicecontext: *mut core::ffi::c_void, objectcontext: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PGPE_DISABLE_EVENT = Option<unsafe extern "system" fn(param0: *mut super::super::Foundation::DEVICE_OBJECT, param1: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PGPE_DISABLE_EVENT2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, objectcontext: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PGPE_DISCONNECT_VECTOR = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PGPE_DISCONNECT_VECTOR2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, objectcontext: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PGPE_ENABLE_EVENT = Option<unsafe extern "system" fn(param0: *mut super::super::Foundation::DEVICE_OBJECT, param1: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PGPE_ENABLE_EVENT2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, objectcontext: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PGPE_SERVICE_ROUTINE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PGPE_SERVICE_ROUTINE2 = Option<unsafe extern "system" fn(objectcontext: *mut core::ffi::c_void, servicecontext: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PHALIOREADWRITEHANDLER = Option<unsafe extern "system" fn(fread: super::super::super::Win32::Foundation::BOOLEAN, dwaddr: u32, dwsize: u32, pdwdata: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PHALMCAINTERFACELOCK = Option<unsafe extern "system" fn()>;
pub type PHALMCAINTERFACEREADREGISTER = Option<unsafe extern "system" fn(banknumber: u8, exception: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PHALMCAINTERFACEUNLOCK = Option<unsafe extern "system" fn()>;
pub type PHAL_RESET_DISPLAY_PARAMETERS = Option<unsafe extern "system" fn(columns: u32, rows: u32) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PHVL_WHEA_ERROR_NOTIFICATION = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PINITIALIZE_DMA_TRANSFER_CONTEXT = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, dmatransfercontext: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PINTERFACE_DEREFERENCE = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
pub type PINTERFACE_REFERENCE = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
pub type PIOMMU_DEVICE_CREATE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_DEVICE_DELETE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_DEVICE_FAULT_HANDLER = Option<unsafe extern "system" fn()>;
pub type PIOMMU_DEVICE_QUERY_DOMAIN_TYPES = Option<unsafe extern "system" fn()>;
pub type PIOMMU_DOMAIN_ATTACH_DEVICE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_DOMAIN_ATTACH_DEVICE_EX = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_DOMAIN_CONFIGURE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_DOMAIN_CREATE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_DOMAIN_CREATE_EX = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_DOMAIN_DELETE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_DOMAIN_DETACH_DEVICE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_DOMAIN_DETACH_DEVICE_EX = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_FLUSH_DOMAIN = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_FLUSH_DOMAIN_VA_LIST = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_FREE_RESERVED_LOGICAL_ADDRESS_RANGE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_INTERFACE_STATE_CHANGE_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PIOMMU_MAP_IDENTITY_RANGE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_MAP_IDENTITY_RANGE_EX = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_MAP_LOGICAL_RANGE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_MAP_LOGICAL_RANGE_EX = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_MAP_RESERVED_LOGICAL_RANGE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_QUERY_INPUT_MAPPINGS = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_REGISTER_INTERFACE_STATE_CHANGE_CALLBACK = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_RESERVE_LOGICAL_ADDRESS_RANGE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_SET_DEVICE_FAULT_REPORTING = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_SET_DEVICE_FAULT_REPORTING_EX = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_UNMAP_IDENTITY_RANGE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_UNMAP_IDENTITY_RANGE_EX = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_UNMAP_LOGICAL_RANGE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_UNMAP_RESERVED_LOGICAL_RANGE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIOMMU_UNREGISTER_INTERFACE_STATE_CHANGE_CALLBACK = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIO_CONTAINER_NOTIFICATION_FUNCTION = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIO_CSQ_ACQUIRE_LOCK = Option<unsafe extern "system" fn()>;
pub type PIO_CSQ_COMPLETE_CANCELED_IRP = Option<unsafe extern "system" fn()>;
pub type PIO_CSQ_INSERT_IRP = Option<unsafe extern "system" fn()>;
pub type PIO_CSQ_INSERT_IRP_EX = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PIO_CSQ_PEEK_NEXT_IRP = Option<unsafe extern "system" fn() -> *mut super::super::Foundation::IRP>;
pub type PIO_CSQ_RELEASE_LOCK = Option<unsafe extern "system" fn()>;
pub type PIO_CSQ_REMOVE_IRP = Option<unsafe extern "system" fn()>;
pub type PIO_DEVICE_EJECT_CALLBACK = Option<unsafe extern "system" fn(status: super::super::super::Win32::Foundation::NTSTATUS, context: *mut core::ffi::c_void)>;
pub type PIO_DPC_ROUTINE = Option<unsafe extern "system" fn()>;
pub type PIO_PERSISTED_MEMORY_ENUMERATION_CALLBACK = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIO_QUERY_DEVICE_ROUTINE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, pathname: *const super::super::super::Win32::Foundation::UNICODE_STRING, bustype: INTERFACE_TYPE, busnumber: u32, businformation: *const *const KEY_VALUE_FULL_INFORMATION, controllertype: CONFIGURATION_TYPE, controllernumber: u32, controllerinformation: *const *const KEY_VALUE_FULL_INFORMATION, peripheraltype: CONFIGURATION_TYPE, peripheralnumber: u32, peripheralinformation: *const *const KEY_VALUE_FULL_INFORMATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIO_SESSION_NOTIFICATION_FUNCTION = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PIO_TIMER_ROUTINE = Option<unsafe extern "system" fn()>;
pub type PIO_WORKITEM_ROUTINE = Option<unsafe extern "system" fn()>;
pub type PIO_WORKITEM_ROUTINE_EX = Option<unsafe extern "system" fn()>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PJOIN_DMA_DOMAIN = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, domainhandle: super::super::super::Win32::Foundation::HANDLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PKBUGCHECK_CALLBACK_ROUTINE = Option<unsafe extern "system" fn()>;
pub type PKBUGCHECK_REASON_CALLBACK_ROUTINE = Option<unsafe extern "system" fn()>;
pub type PKIPI_BROADCAST_WORKER = Option<unsafe extern "system" fn() -> usize>;
pub type PKMESSAGE_SERVICE_ROUTINE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PKSERVICE_ROUTINE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PKSTART_ROUTINE = Option<unsafe extern "system" fn()>;
pub type PKSYNCHRONIZE_ROUTINE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PLEAVE_DMA_DOMAIN = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PLOAD_IMAGE_NOTIFY_ROUTINE = Option<unsafe extern "system" fn(fullimagename: *const super::super::super::Win32::Foundation::UNICODE_STRING, processid: super::super::super::Win32::Foundation::HANDLE, imageinfo: *const IMAGE_INFO)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PMAP_TRANSFER = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const super::super::Foundation::MDL, mapregisterbase: *const core::ffi::c_void, currentva: *const core::ffi::c_void, length: *mut u32, writetodevice: super::super::super::Win32::Foundation::BOOLEAN) -> i64>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PMAP_TRANSFER_EX = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, mdl: *const super::super::Foundation::MDL, mapregisterbase: *const core::ffi::c_void, offset: u64, deviceoffset: u32, length: *mut u32, writetodevice: super::super::super::Win32::Foundation::BOOLEAN, scattergatherbuffer: *mut SCATTER_GATHER_LIST, scattergatherbufferlength: u32, dmacompletionroutine: PDMA_COMPLETION_ROUTINE, completioncontext: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PMM_DLL_INITIALIZE = Option<unsafe extern "system" fn(registrypath: *const super::super::super::Win32::Foundation::UNICODE_STRING) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PMM_DLL_UNLOAD = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PMM_GET_SYSTEM_ROUTINE_ADDRESS_EX = Option<unsafe extern "system" fn(modulename: *const super::super::super::Win32::Foundation::UNICODE_STRING, functionname: windows_sys::core::PCSTR) -> *mut core::ffi::c_void>;
pub type PMM_MDL_ROUTINE = Option<unsafe extern "system" fn()>;
#[cfg(feature = "Wdk_Foundation")]
pub type PMM_ROTATE_COPY_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(destinationmdl: *const super::super::Foundation::MDL, sourcemdl: *const super::super::Foundation::MDL, context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PNMI_CALLBACK = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PNPEM_CONTROL_ENABLE_DISABLE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PNPEM_CONTROL_QUERY_CONTROL = Option<unsafe extern "system" fn() -> u32>;
pub type PNPEM_CONTROL_QUERY_STANDARD_CAPABILITIES = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PNPEM_CONTROL_SET_STANDARD_CONTROL = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PNTFS_DEREF_EXPORTED_SECURITY_DESCRIPTOR = Option<unsafe extern "system" fn()>;
#[cfg(feature = "Wdk_Foundation")]
pub type POB_POST_OPERATION_CALLBACK = Option<unsafe extern "system" fn(registrationcontext: *const core::ffi::c_void, operationinformation: *const OB_POST_OPERATION_INFORMATION)>;
#[cfg(feature = "Wdk_Foundation")]
pub type POB_PRE_OPERATION_CALLBACK = Option<unsafe extern "system" fn(registrationcontext: *const core::ffi::c_void, operationinformation: *mut OB_PRE_OPERATION_INFORMATION) -> OB_PREOP_CALLBACK_STATUS>;
pub type POWER_SETTING_CALLBACK = Option<unsafe extern "system" fn(settingguid: *const windows_sys::core::GUID, value: *const core::ffi::c_void, valuelength: u32, context: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PO_FX_COMPONENT_ACTIVE_CONDITION_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, component: u32)>;
pub type PO_FX_COMPONENT_CRITICAL_TRANSITION_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, component: u32, active: super::super::super::Win32::Foundation::BOOLEAN)>;
pub type PO_FX_COMPONENT_IDLE_CONDITION_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, component: u32)>;
pub type PO_FX_COMPONENT_IDLE_STATE_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, component: u32, state: u32)>;
pub type PO_FX_COMPONENT_PERF_STATE_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, component: u32, succeeded: super::super::super::Win32::Foundation::BOOLEAN, requestcontext: *const core::ffi::c_void)>;
pub type PO_FX_DEVICE_POWER_NOT_REQUIRED_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type PO_FX_DEVICE_POWER_REQUIRED_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type PO_FX_DIRECTED_POWER_DOWN_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, flags: u32)>;
pub type PO_FX_DIRECTED_POWER_UP_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, flags: u32)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PO_FX_DRIPS_WATCHDOG_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, physicaldeviceobject: *const super::super::Foundation::DEVICE_OBJECT, uniqueid: u32)>;
pub type PO_FX_POWER_CONTROL_CALLBACK = Option<unsafe extern "system" fn(devicecontext: *const core::ffi::c_void, powercontrolcode: *const windows_sys::core::GUID, inbuffer: *const core::ffi::c_void, inbuffersize: usize, outbuffer: *mut core::ffi::c_void, outbuffersize: usize, bytesreturned: *mut usize) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPCI_EXPRESS_ENTER_LINK_QUIESCENT_MODE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPCI_EXPRESS_EXIT_LINK_QUIESCENT_MODE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPCI_EXPRESS_ROOT_PORT_READ_CONFIG_SPACE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, buffer: *mut core::ffi::c_void, offset: u32, length: u32) -> u32>;
pub type PPCI_EXPRESS_ROOT_PORT_WRITE_CONFIG_SPACE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, buffer: *const core::ffi::c_void, offset: u32, length: u32) -> u32>;
pub type PPCI_EXPRESS_WAKE_CONTROL = Option<unsafe extern "system" fn()>;
pub type PPCI_IS_DEVICE_PRESENT = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PPCI_IS_DEVICE_PRESENT_EX = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type PPCI_LINE_TO_PIN = Option<unsafe extern "system" fn()>;
pub type PPCI_MSIX_GET_ENTRY = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPCI_MSIX_GET_TABLE_SIZE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPCI_MSIX_MASKUNMASK_ENTRY = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPCI_MSIX_SET_ENTRY = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPCI_PIN_TO_LINE = Option<unsafe extern "system" fn()>;
pub type PPCI_PREPARE_MULTISTAGE_RESUME = Option<unsafe extern "system" fn()>;
pub type PPCI_READ_WRITE_CONFIG = Option<unsafe extern "system" fn() -> u32>;
pub type PPCI_ROOT_BUS_CAPABILITY = Option<unsafe extern "system" fn()>;
pub type PPCI_SET_ACS = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPCI_SET_ACS2 = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPCI_SET_ATS = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPCW_CALLBACK = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPHYSICAL_COUNTER_EVENT_BUFFER_OVERFLOW_HANDLER = Option<unsafe extern "system" fn(eventbuffer: *const core::ffi::c_void, entrysize: usize, numberofentries: usize, owninghandle: super::super::super::Win32::Foundation::HANDLE)>;
pub type PPHYSICAL_COUNTER_OVERFLOW_HANDLER = Option<unsafe extern "system" fn(overflowbits: u64, owninghandle: super::super::super::Win32::Foundation::HANDLE)>;
pub type PPOWER_SETTING_CALLBACK = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPO_FX_COMPONENT_ACTIVE_CONDITION_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PPO_FX_COMPONENT_CRITICAL_TRANSITION_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PPO_FX_COMPONENT_IDLE_CONDITION_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PPO_FX_COMPONENT_IDLE_STATE_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PPO_FX_COMPONENT_PERF_STATE_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PPO_FX_DEVICE_POWER_NOT_REQUIRED_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PPO_FX_DEVICE_POWER_REQUIRED_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PPO_FX_DIRECTED_POWER_DOWN_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PPO_FX_DIRECTED_POWER_UP_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PPO_FX_DRIPS_WATCHDOG_CALLBACK = Option<unsafe extern "system" fn()>;
pub type PPO_FX_POWER_CONTROL_CALLBACK = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPROCESSOR_CALLBACK_FUNCTION = Option<unsafe extern "system" fn()>;
pub type PPROCESSOR_HALT_ROUTINE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPTM_DEVICE_DISABLE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPTM_DEVICE_ENABLE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPTM_DEVICE_QUERY_GRANULARITY = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PPTM_DEVICE_QUERY_TIME_SOURCE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PPUT_DMA_ADAPTER = Option<unsafe extern "system" fn(dmaadapter: *mut DMA_ADAPTER)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PPUT_SCATTER_GATHER_LIST = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER, scattergather: *const SCATTER_GATHER_LIST, writetodevice: super::super::super::Win32::Foundation::BOOLEAN)>;
pub type PQUERYEXTENDEDADDRESS = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, extendedaddress: *mut u64)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PREAD_DMA_COUNTER = Option<unsafe extern "system" fn(dmaadapter: *const DMA_ADAPTER) -> u32>;
pub type PREENUMERATE_SELF = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PREGISTER_FOR_DEVICE_NOTIFICATIONS = Option<unsafe extern "system" fn(param0: *mut super::super::Foundation::DEVICE_OBJECT, param1: PDEVICE_NOTIFY_CALLBACK, param2: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PREGISTER_FOR_DEVICE_NOTIFICATIONS2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, notificationhandler: PDEVICE_NOTIFY_CALLBACK2, notificationcontext: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PREPLACE_BEGIN = Option<unsafe extern "system" fn(parameters: *const PNP_REPLACE_PARAMETERS, context: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PREPLACE_DRIVER_INIT = Option<unsafe extern "system" fn(interface: *mut PNP_REPLACE_DRIVER_INTERFACE, unused: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PREPLACE_ENABLE_DISABLE_HARDWARE_QUIESCE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, enable: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PREPLACE_END = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PREPLACE_GET_MEMORY_DESTINATION = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sourceaddress: i64, destinationaddress: *mut i64) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PREPLACE_INITIATE_HARDWARE_MIRROR = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PREPLACE_MAP_MEMORY = Option<unsafe extern "system" fn(targetphysicaladdress: i64, sparephysicaladdress: i64, numberofbytes: *mut i64, targetaddress: *mut *mut core::ffi::c_void, spareaddress: *mut *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PREPLACE_MIRROR_PHYSICAL_MEMORY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, physicaladdress: i64, bytecount: i64) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PREPLACE_MIRROR_PLATFORM_MEMORY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PREPLACE_SET_PROCESSOR_ID = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, apicid: u32, target: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PREPLACE_SWAP = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PREPLACE_UNLOAD = Option<unsafe extern "system" fn()>;
pub type PREQUEST_POWER_COMPLETE = Option<unsafe extern "system" fn()>;
#[cfg(feature = "Win32_System_Kernel")]
pub type PROCESSOR_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(callbackcontext: *const core::ffi::c_void, changecontext: *const KE_PROCESSOR_CHANGE_NOTIFY_CONTEXT, operationstatus: *mut i32)>;
pub type PROCESSOR_HALT_ROUTINE = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PRTL_AVL_ALLOCATE_ROUTINE = Option<unsafe extern "system" fn() -> *mut core::ffi::c_void>;
pub type PRTL_AVL_COMPARE_ROUTINE = Option<unsafe extern "system" fn() -> RTL_GENERIC_COMPARE_RESULTS>;
pub type PRTL_AVL_FREE_ROUTINE = Option<unsafe extern "system" fn()>;
pub type PRTL_AVL_MATCH_FUNCTION = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PRTL_GENERIC_ALLOCATE_ROUTINE = Option<unsafe extern "system" fn() -> *mut core::ffi::c_void>;
pub type PRTL_GENERIC_COMPARE_ROUTINE = Option<unsafe extern "system" fn() -> RTL_GENERIC_COMPARE_RESULTS>;
pub type PRTL_GENERIC_FREE_ROUTINE = Option<unsafe extern "system" fn()>;
pub type PRTL_QUERY_REGISTRY_ROUTINE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PRTL_RUN_ONCE_INIT_FN = Option<unsafe extern "system" fn() -> u32>;
pub type PSECURE_DRIVER_PROCESS_DEREFERENCE = Option<unsafe extern "system" fn()>;
#[cfg(feature = "Wdk_Foundation")]
pub type PSECURE_DRIVER_PROCESS_REFERENCE = Option<unsafe extern "system" fn() -> super::super::Foundation::PEPROCESS>;
pub type PSET_D3COLD_SUPPORT = Option<unsafe extern "system" fn()>;
pub type PSET_VIRTUAL_DEVICE_DATA = Option<unsafe extern "system" fn() -> u32>;
pub type PSE_IMAGE_VERIFICATION_CALLBACK_FUNCTION = Option<unsafe extern "system" fn()>;
pub type PSHED_PI_ATTEMPT_ERROR_RECOVERY = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, bufferlength: u32, errorrecord: *const WHEA_ERROR_RECORD) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PSHED_PI_CLEAR_ERROR_RECORD = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, flags: u32, errorrecordid: u64) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type PSHED_PI_CLEAR_ERROR_STATUS = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errorsource: *const super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_DESCRIPTOR, bufferlength: u32, errorrecord: *const WHEA_ERROR_RECORD) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type PSHED_PI_DISABLE_ERROR_SOURCE = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errorsource: *const super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type PSHED_PI_ENABLE_ERROR_SOURCE = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errorsource: *const super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type PSHED_PI_FINALIZE_ERROR_RECORD = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errorsource: *const super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_DESCRIPTOR, bufferlength: u32, errorrecord: *mut WHEA_ERROR_RECORD) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type PSHED_PI_GET_ALL_ERROR_SOURCES = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, count: *mut u32, errorsrcs: *mut *mut super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_DESCRIPTOR, length: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type PSHED_PI_GET_ERROR_SOURCE_INFO = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errorsource: *mut super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PSHED_PI_GET_INJECTION_CAPABILITIES = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, capabilities: *mut WHEA_ERROR_INJECTION_CAPABILITIES) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PSHED_PI_INJECT_ERROR = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errortype: u64, parameter1: u64, parameter2: u64, parameter3: u64, parameter4: u64) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PSHED_PI_READ_ERROR_RECORD = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, flags: u32, errorrecordid: u64, nexterrorrecordid: *mut u64, recordlength: *mut u32, errorrecord: *mut WHEA_ERROR_RECORD) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type PSHED_PI_RETRIEVE_ERROR_INFO = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errorsource: *const super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_DESCRIPTOR, bufferlength: u64, packet: *mut WHEA_ERROR_PACKET_V2) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type PSHED_PI_SET_ERROR_SOURCE_INFO = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, errorsource: *const super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PSHED_PI_WRITE_ERROR_RECORD = Option<unsafe extern "system" fn(plugincontext: *mut core::ffi::c_void, flags: u32, recordlength: u32, errorrecord: *const WHEA_ERROR_RECORD) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PTIMER_APC_ROUTINE = Option<unsafe extern "system" fn(timercontext: *const core::ffi::c_void, timerlowvalue: u32, timerhighvalue: i32)>;
pub type PTM_DEVICE_DISABLE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PTM_DEVICE_ENABLE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PTM_DEVICE_QUERY_GRANULARITY = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, granularity: *mut u8) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PTM_DEVICE_QUERY_TIME_SOURCE = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, timesource: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PTM_PROPAGATE_ROUTINE = Option<unsafe extern "system" fn(propagationcookie: *const core::ffi::c_void, callbackdata: *const core::ffi::c_void, propagationstatus: super::super::super::Win32::Foundation::NTSTATUS, transactionguid: windows_sys::core::GUID) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type PTM_RM_NOTIFICATION = Option<unsafe extern "system" fn(enlistmentobject: *const super::super::Foundation::KENLISTMENT, rmcontext: *const core::ffi::c_void, transactioncontext: *const core::ffi::c_void, transactionnotification: u32, tmvirtualclock: *mut i64, argumentlength: u32, argument: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type PTRANSLATE_BUS_ADDRESS = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PTRANSLATE_RESOURCE_HANDLER = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, source: *const CM_PARTIAL_RESOURCE_DESCRIPTOR, direction: RESOURCE_TRANSLATION_DIRECTION, alternativescount: u32, alternatives: *const IO_RESOURCE_DESCRIPTOR, physicaldeviceobject: *const super::super::Foundation::DEVICE_OBJECT, target: *mut CM_PARTIAL_RESOURCE_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PTRANSLATE_RESOURCE_REQUIREMENTS_HANDLER = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, source: *const IO_RESOURCE_DESCRIPTOR, physicaldeviceobject: *const super::super::Foundation::DEVICE_OBJECT, targetcount: *mut u32, target: *mut *mut IO_RESOURCE_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type PUNREGISTER_FOR_DEVICE_NOTIFICATIONS = Option<unsafe extern "system" fn(param0: *mut super::super::Foundation::DEVICE_OBJECT, param1: PDEVICE_NOTIFY_CALLBACK)>;
pub type PUNREGISTER_FOR_DEVICE_NOTIFICATIONS2 = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
pub type PciLine2Pin = Option<unsafe extern "system" fn(bushandler: *const isize, roothandler: *const isize, slotnumber: PCI_SLOT_NUMBER, pcinewdata: *const PCI_COMMON_CONFIG, pciolddata: *const PCI_COMMON_CONFIG)>;
pub type PciPin2Line = Option<unsafe extern "system" fn(bushandler: *const isize, roothandler: *const isize, slotnumber: PCI_SLOT_NUMBER, pcidata: *const PCI_COMMON_CONFIG)>;
pub type PciReadWriteConfig = Option<unsafe extern "system" fn(bushandler: *const isize, slot: PCI_SLOT_NUMBER, buffer: *const core::ffi::c_void, offset: u32, length: u32)>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type REQUEST_POWER_COMPLETE = Option<unsafe extern "system" fn(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, minorfunction: u8, powerstate: POWER_STATE, context: *const core::ffi::c_void, iostatus: *const super::super::super::Win32::System::IO::IO_STATUS_BLOCK)>;
pub type RTL_AVL_ALLOCATE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, bytesize: u32) -> *mut core::ffi::c_void>;
pub type RTL_AVL_COMPARE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, firststruct: *const core::ffi::c_void, secondstruct: *const core::ffi::c_void) -> RTL_GENERIC_COMPARE_RESULTS>;
pub type RTL_AVL_FREE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, buffer: *const core::ffi::c_void)>;
pub type RTL_AVL_MATCH_FUNCTION = Option<unsafe extern "system" fn(table: *const RTL_AVL_TABLE, userdata: *const core::ffi::c_void, matchdata: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
pub type RTL_GENERIC_ALLOCATE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, bytesize: u32) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
pub type RTL_GENERIC_COMPARE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, firststruct: *const core::ffi::c_void, secondstruct: *const core::ffi::c_void) -> RTL_GENERIC_COMPARE_RESULTS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Win32_System_Kernel"))]
pub type RTL_GENERIC_FREE_ROUTINE = Option<unsafe extern "system" fn(table: *const RTL_GENERIC_TABLE, buffer: *const core::ffi::c_void)>;
pub type RTL_QUERY_REGISTRY_ROUTINE = Option<unsafe extern "system" fn(valuename: windows_sys::core::PCWSTR, valuetype: u32, valuedata: *const core::ffi::c_void, valuelength: u32, context: *const core::ffi::c_void, entrycontext: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Threading")]
pub type RTL_RUN_ONCE_INIT_FN = Option<unsafe extern "system" fn(runonce: *mut super::super::super::Win32::System::Threading::INIT_ONCE, parameter: *mut core::ffi::c_void, context: *mut *mut core::ffi::c_void) -> u32>;
#[cfg(feature = "Wdk_Foundation")]
pub type SECURE_DRIVER_PROCESS_DEREFERENCE = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void, process: super::super::Foundation::PEPROCESS)>;
#[cfg(feature = "Wdk_Foundation")]
pub type SECURE_DRIVER_PROCESS_REFERENCE = Option<unsafe extern "system" fn(interfacecontext: *const core::ffi::c_void) -> super::super::Foundation::PEPROCESS>;
pub type SET_D3COLD_SUPPORT = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, d3coldsupport: super::super::super::Win32::Foundation::BOOLEAN)>;
pub type SET_VIRTUAL_DEVICE_DATA = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, virtualfunction: u16, buffer: *const core::ffi::c_void, offset: u32, length: u32) -> u32>;
pub type SE_IMAGE_VERIFICATION_CALLBACK_FUNCTION = Option<unsafe extern "system" fn(callbackcontext: *const core::ffi::c_void, imagetype: SE_IMAGE_TYPE, imageinformation: *mut BDCB_IMAGE_INFORMATION)>;
pub type SILO_CONTEXT_CLEANUP_CALLBACK = Option<unsafe extern "system" fn(silocontext: *const core::ffi::c_void)>;
#[cfg(feature = "Wdk_Foundation")]
pub type SILO_MONITOR_CREATE_CALLBACK = Option<unsafe extern "system" fn(silo: super::super::Foundation::PESILO) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type SILO_MONITOR_TERMINATE_CALLBACK = Option<unsafe extern "system" fn(silo: super::super::Foundation::PESILO)>;
pub type TRANSLATE_BUS_ADDRESS = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, busaddress: i64, length: u32, addressspace: *mut u32, translatedaddress: *mut i64) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type WHEA_ERROR_SOURCE_CORRECT = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type WHEA_ERROR_SOURCE_CREATE_RECORD = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type WHEA_ERROR_SOURCE_INITIALIZE = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type WHEA_ERROR_SOURCE_RECOVER = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type WHEA_ERROR_SOURCE_UNINITIALIZE = Option<unsafe extern "system" fn()>;
pub type WHEA_SIGNAL_HANDLER_OVERRIDE_CALLBACK = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type WMI_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn()>;
pub type WORKER_THREAD_ROUTINE = Option<unsafe extern "system" fn(parameter: *const core::ffi::c_void)>;
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type _WHEA_ERROR_SOURCE_CORRECT = Option<unsafe extern "system" fn(errorsource: *mut super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_DESCRIPTOR, maximumsectionlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type _WHEA_ERROR_SOURCE_CREATE_RECORD = Option<unsafe extern "system" fn(errorsource: *mut super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_DESCRIPTOR, errorpacket: *mut WHEA_ERROR_PACKET_V2, errorrecord: *mut WHEA_ERROR_RECORD, buffersize: u32, context: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Diagnostics_Debug")]
pub type _WHEA_ERROR_SOURCE_INITIALIZE = Option<unsafe extern "system" fn(phase: u32, errorsource: *mut super::super::super::Win32::System::Diagnostics::Debug::WHEA_ERROR_SOURCE_DESCRIPTOR, context: *mut core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type _WHEA_ERROR_SOURCE_RECOVER = Option<unsafe extern "system" fn(recoverycontext: *mut core::ffi::c_void, severity: *mut WHEA_ERROR_SEVERITY) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type _WHEA_ERROR_SOURCE_UNINITIALIZE = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
pub type _WHEA_SIGNAL_HANDLER_OVERRIDE_CALLBACK = Option<unsafe extern "system" fn(context: usize) -> super::super::super::Win32::Foundation::BOOLEAN>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type pHalAssignSlotResources = Option<unsafe extern "system" fn(registrypath: *const super::super::super::Win32::Foundation::UNICODE_STRING, driverclassname: *const super::super::super::Win32::Foundation::UNICODE_STRING, driverobject: *const super::super::Foundation::DRIVER_OBJECT, deviceobject: *const super::super::Foundation::DEVICE_OBJECT, bustype: INTERFACE_TYPE, busnumber: u32, slotnumber: u32, allocatedresources: *mut *mut CM_RESOURCE_LIST) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type pHalEndMirroring = Option<unsafe extern "system" fn(passnumber: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type pHalEndOfBoot = Option<unsafe extern "system" fn()>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type pHalExamineMBR = Option<unsafe extern "system" fn(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, sectorsize: u32, mbrtypeidentifier: u32, buffer: *mut *mut core::ffi::c_void)>;
pub type pHalFindBusAddressTranslation = Option<unsafe extern "system" fn(busaddress: i64, addressspace: *mut u32, translatedaddress: *mut i64, context: *mut usize, nextbus: super::super::super::Win32::Foundation::BOOLEAN) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type pHalGetAcpiTable = Option<unsafe extern "system" fn(signature: u32, oemid: windows_sys::core::PCSTR, oemtableid: windows_sys::core::PCSTR) -> *mut core::ffi::c_void>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type pHalGetDmaAdapter = Option<unsafe extern "system" fn(physicaldeviceobject: *const core::ffi::c_void, devicedescriptor: *const DEVICE_DESCRIPTION, numberofmapregisters: *mut u32) -> *mut DMA_ADAPTER>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type pHalGetInterruptTranslator = Option<unsafe extern "system" fn(parentinterfacetype: INTERFACE_TYPE, parentbusnumber: u32, bridgeinterfacetype: INTERFACE_TYPE, size: u16, version: u16, translator: *mut TRANSLATOR_INTERFACE, bridgebusnumber: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Win32_System_Kernel")]
pub type pHalGetPrmCache = Option<unsafe extern "system" fn(firmwarelist: *mut *mut super::super::super::Win32::System::Kernel::LIST_ENTRY, updatelist: *mut *mut super::super::super::Win32::System::Kernel::LIST_ENTRY)>;
pub type pHalHaltSystem = Option<unsafe extern "system" fn()>;
#[cfg(feature = "Wdk_Foundation")]
pub type pHalHandlerForBus = Option<unsafe extern "system" fn(interfacetype: INTERFACE_TYPE, busnumber: u32) -> super::super::Foundation::PBUS_HANDLER>;
pub type pHalInitPnpDriver = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type pHalInitPowerManagement = Option<unsafe extern "system" fn(pmdriverdispatchtable: *const PM_DISPATCH_TABLE, pmhaldispatchtable: *mut *mut PM_DISPATCH_TABLE) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Ioctl", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type pHalIoReadPartitionTable = Option<unsafe extern "system" fn(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, sectorsize: u32, returnrecognizedpartitions: super::super::super::Win32::Foundation::BOOLEAN, partitionbuffer: *mut *mut super::super::super::Win32::System::Ioctl::DRIVE_LAYOUT_INFORMATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type pHalIoSetPartitionInformation = Option<unsafe extern "system" fn(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, sectorsize: u32, partitionnumber: u32, partitiontype: u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(all(feature = "Wdk_Foundation", feature = "Wdk_Storage_FileSystem", feature = "Win32_Security", feature = "Win32_System_IO", feature = "Win32_System_Ioctl", feature = "Win32_System_Kernel", feature = "Win32_System_Power"))]
pub type pHalIoWritePartitionTable = Option<unsafe extern "system" fn(deviceobject: *const super::super::Foundation::DEVICE_OBJECT, sectorsize: u32, sectorspertrack: u32, numberofheads: u32, partitionbuffer: *const super::super::super::Win32::System::Ioctl::DRIVE_LAYOUT_INFORMATION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type pHalMirrorPhysicalMemory = Option<unsafe extern "system" fn(physicaladdress: i64, numberofbytes: i64) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type pHalMirrorVerify = Option<unsafe extern "system" fn(physicaladdress: i64, numberofbytes: i64) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type pHalQueryBusSlots = Option<unsafe extern "system" fn(bushandler: super::super::Foundation::PBUS_HANDLER, buffersize: u32, slotnumbers: *mut u32, returnedlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type pHalQuerySystemInformation = Option<unsafe extern "system" fn(informationclass: HAL_QUERY_INFORMATION_CLASS, buffersize: u32, buffer: *mut core::ffi::c_void, returnedlength: *mut u32) -> super::super::super::Win32::Foundation::NTSTATUS>;
#[cfg(feature = "Wdk_Foundation")]
pub type pHalReferenceBusHandler = Option<unsafe extern "system" fn(bushandler: super::super::Foundation::PBUS_HANDLER)>;
pub type pHalResetDisplay = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type pHalSetPciErrorHandlerCallback = Option<unsafe extern "system" fn(callback: PCI_ERROR_HANDLER_CALLBACK)>;
pub type pHalSetSystemInformation = Option<unsafe extern "system" fn(informationclass: HAL_SET_INFORMATION_CLASS, buffersize: u32, buffer: *const core::ffi::c_void) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type pHalStartMirroring = Option<unsafe extern "system" fn() -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type pHalTranslateBusAddress = Option<unsafe extern "system" fn(interfacetype: INTERFACE_TYPE, busnumber: u32, busaddress: i64, addressspace: *mut u32, translatedaddress: *mut i64) -> super::super::super::Win32::Foundation::BOOLEAN>;
pub type pHalVectorToIDTEntry = Option<unsafe extern "system" fn(vector: u32) -> u8>;
pub type pKdCheckPowerButton = Option<unsafe extern "system" fn()>;
pub type pKdEnumerateDebuggingDevices = Option<unsafe extern "system" fn(loaderblock: *const core::ffi::c_void, device: *mut DEBUG_DEVICE_DESCRIPTOR, callback: PDEBUG_DEVICE_FOUND_FUNCTION) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type pKdGetAcpiTablePhase0 = Option<unsafe extern "system" fn(loaderblock: *const isize, signature: u32) -> *mut core::ffi::c_void>;
pub type pKdGetPciDataByOffset = Option<unsafe extern "system" fn(busnumber: u32, slotnumber: u32, buffer: *mut core::ffi::c_void, offset: u32, length: u32) -> u32>;
pub type pKdMapPhysicalMemory64 = Option<unsafe extern "system" fn(physicaladdress: i64, numberpages: u32, flushcurrenttlb: super::super::super::Win32::Foundation::BOOLEAN) -> *mut core::ffi::c_void>;
pub type pKdReleaseIntegratedDeviceForDebugging = Option<unsafe extern "system" fn(integrateddevice: *mut DEBUG_DEVICE_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type pKdReleasePciDeviceForDebugging = Option<unsafe extern "system" fn(pcidevice: *mut DEBUG_DEVICE_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type pKdSetPciDataByOffset = Option<unsafe extern "system" fn(busnumber: u32, slotnumber: u32, buffer: *const core::ffi::c_void, offset: u32, length: u32) -> u32>;
pub type pKdSetupIntegratedDeviceForDebugging = Option<unsafe extern "system" fn(loaderblock: *const core::ffi::c_void, integrateddevice: *mut DEBUG_DEVICE_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type pKdSetupPciDeviceForDebugging = Option<unsafe extern "system" fn(loaderblock: *const core::ffi::c_void, pcidevice: *mut DEBUG_DEVICE_DESCRIPTOR) -> super::super::super::Win32::Foundation::NTSTATUS>;
pub type pKdUnmapVirtualAddress = Option<unsafe extern "system" fn(virtualaddress: *const core::ffi::c_void, numberpages: u32, flushcurrenttlb: super::super::super::Win32::Foundation::BOOLEAN)>;
