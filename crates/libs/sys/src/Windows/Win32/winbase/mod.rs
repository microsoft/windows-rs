#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AccessCheckAndAuditAlarmA(subsystemname : windows_sys::core::PCSTR, handleid : *const core::ffi::c_void, objecttypename : windows_sys::core::PCSTR, objectname : windows_sys::core::PCSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, desiredaccess : u32, genericmapping : *const super::GENERIC_MAPPING, objectcreation : windows_sys::core::BOOL, grantedaccess : *mut u32, accessstatus : *mut windows_sys::core::BOOL, pfgenerateonclose : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AccessCheckByTypeAndAuditAlarmA(subsystemname : windows_sys::core::PCSTR, handleid : *const core::ffi::c_void, objecttypename : windows_sys::core::PCSTR, objectname : windows_sys::core::PCSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : u32, audittype : super::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut super::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::GENERIC_MAPPING, objectcreation : windows_sys::core::BOOL, grantedaccess : *mut u32, accessstatus : *mut windows_sys::core::BOOL, pfgenerateonclose : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmA(subsystemname : windows_sys::core::PCSTR, handleid : *const core::ffi::c_void, objecttypename : windows_sys::core::PCSTR, objectname : windows_sys::core::PCSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : u32, audittype : super::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut super::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::GENERIC_MAPPING, objectcreation : windows_sys::core::BOOL, grantedaccess : *mut u32, accessstatuslist : *mut u32, pfgenerateonclose : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmByHandleA(subsystemname : windows_sys::core::PCSTR, handleid : *const core::ffi::c_void, clienttoken : super::HANDLE, objecttypename : windows_sys::core::PCSTR, objectname : windows_sys::core::PCSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : u32, audittype : super::AUDIT_EVENT_TYPE, flags : u32, objecttypelist : *mut super::OBJECT_TYPE_LIST, objecttypelistlength : u32, genericmapping : *const super::GENERIC_MAPPING, objectcreation : windows_sys::core::BOOL, grantedaccess : *mut u32, accessstatuslist : *mut u32, pfgenerateonclose : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ActivateActCtx(hactctx : super::HANDLE, lpcookie : *mut usize) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn AddAtomA(lpstring : windows_sys::core::PCSTR) -> super::ATOM);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn AddAtomW(lpstring : windows_sys::core::PCWSTR) -> super::ATOM);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn AddConditionalAce(pacl : *mut super::ACL, dwacerevision : u32, aceflags : u32, acetype : u8, accessmask : u32, psid : super::PSID, conditionstr : *const u16, returnlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AddIntegrityLabelToBoundaryDescriptor(boundarydescriptor : *mut super::HANDLE, integritylabel : super::PSID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AddRefActCtx(hactctx : super::HANDLE));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn AddSecureMemoryCacheCallback(pfncallback : super::PSECURE_MEMORY_CACHE_CALLBACK) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ApplicationRecoveryFinished(bsuccess : windows_sys::core::BOOL));
windows_link::link!("kernel32.dll" "system" fn ApplicationRecoveryInProgress(pbcancelled : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn BackupEventLogA(heventlog : super::HANDLE, lpbackupfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn BackupEventLogW(heventlog : super::HANDLE, lpbackupfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn BackupRead(hfile : super::HANDLE, lpbuffer : *mut u8, nnumberofbytestoread : u32, lpnumberofbytesread : *mut u32, babort : windows_sys::core::BOOL, bprocesssecurity : windows_sys::core::BOOL, lpcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn BackupSeek(hfile : super::HANDLE, dwlowbytestoseek : u32, dwhighbytestoseek : u32, lpdwlowbyteseeked : *mut u32, lpdwhighbyteseeked : *mut u32, lpcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn BackupWrite(hfile : super::HANDLE, lpbuffer : *const u8, nnumberofbytestowrite : u32, lpnumberofbyteswritten : *mut u32, babort : windows_sys::core::BOOL, bprocesssecurity : windows_sys::core::BOOL, lpcontext : *mut *mut core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn BeginUpdateResourceA(pfilename : windows_sys::core::PCSTR, bdeleteexistingresources : windows_sys::core::BOOL) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn BeginUpdateResourceW(pfilename : windows_sys::core::PCWSTR, bdeleteexistingresources : windows_sys::core::BOOL) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn BindIoCompletionCallback(filehandle : super::HANDLE, function : super::LPOVERLAPPED_COMPLETION_ROUTINE, flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn BuildCommDCBA(lpdef : windows_sys::core::PCSTR, lpdcb : *mut DCB) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn BuildCommDCBAndTimeoutsA(lpdef : windows_sys::core::PCSTR, lpdcb : *mut DCB, lpcommtimeouts : *mut COMMTIMEOUTS) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn BuildCommDCBAndTimeoutsW(lpdef : windows_sys::core::PCWSTR, lpdcb : *mut DCB, lpcommtimeouts : *mut COMMTIMEOUTS) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn BuildCommDCBW(lpdef : windows_sys::core::PCWSTR, lpdcb : *mut DCB) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn CallNamedPipeA(lpnamedpipename : windows_sys::core::PCSTR, lpinbuffer : *const core::ffi::c_void, ninbuffersize : u32, lpoutbuffer : *mut core::ffi::c_void, noutbuffersize : u32, lpbytesread : *mut u32, ntimeout : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelDeviceWakeupRequest(hdevice : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CancelTimerQueueTimer(timerqueue : super::HANDLE, timer : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn CheckNameLegalDOS8Dot3A(lpname : windows_sys::core::PCSTR, lpoemname : windows_sys::core::PSTR, oemnamesize : u32, pbnamecontainsspaces : *mut windows_sys::core::BOOL, pbnamelegal : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn CheckNameLegalDOS8Dot3W(lpname : windows_sys::core::PCWSTR, lpoemname : windows_sys::core::PSTR, oemnamesize : u32, pbnamecontainsspaces : *mut windows_sys::core::BOOL, pbnamelegal : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ClearCommBreak(hfile : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ClearCommError(hfile : super::HANDLE, lperrors : *mut u32, lpstat : *mut COMSTAT) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ClearEventLogA(heventlog : super::HANDLE, lpbackupfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ClearEventLogW(heventlog : super::HANDLE, lpbackupfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn CloseEncryptedFileRaw(pvcontext : *const core::ffi::c_void));
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn CloseEventLog(heventlog : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("kernel32.dll" "system" fn CommConfigDialogA(lpszname : windows_sys::core::PCSTR, hwnd : super::HWND, lpcc : *mut COMMCONFIG) -> windows_sys::core::BOOL);
#[cfg(feature = "windef")]
windows_link::link!("kernel32.dll" "system" fn CommConfigDialogW(lpszname : windows_sys::core::PCWSTR, hwnd : super::HWND, lpcc : *mut COMMCONFIG) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ConvertFiberToThread() -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ConvertThreadToFiber(lpparameter : *const core::ffi::c_void) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn ConvertThreadToFiberEx(lpparameter : *const core::ffi::c_void, dwflags : u32) -> *mut core::ffi::c_void);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CopyContext(destination : *mut super::CONTEXT, contextflags : u32, source : *const super::CONTEXT) -> windows_sys::core::BOOL);
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CopyContext(destination : *mut super::ARM64_NT_CONTEXT, contextflags : u32, source : *const super::ARM64_NT_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CopyFile2(pwszexistingfilename : windows_sys::core::PCWSTR, pwsznewfilename : windows_sys::core::PCWSTR, pextendedparameters : *const COPYFILE2_EXTENDED_PARAMETERS) -> windows_sys::core::HRESULT);
windows_link::link!("kernel32.dll" "system" fn CopyFileA(lpexistingfilename : windows_sys::core::PCSTR, lpnewfilename : windows_sys::core::PCSTR, bfailifexists : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CopyFileExA(lpexistingfilename : windows_sys::core::PCSTR, lpnewfilename : windows_sys::core::PCSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, pbcancel : *mut windows_sys::core::BOOL, dwcopyflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CopyFileExW(lpexistingfilename : windows_sys::core::PCWSTR, lpnewfilename : windows_sys::core::PCWSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, pbcancel : *mut windows_sys::core::BOOL, dwcopyflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CopyFileTransactedA(lpexistingfilename : windows_sys::core::PCSTR, lpnewfilename : windows_sys::core::PCSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, pbcancel : *const windows_sys::core::BOOL, dwcopyflags : u32, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CopyFileTransactedW(lpexistingfilename : windows_sys::core::PCWSTR, lpnewfilename : windows_sys::core::PCWSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, pbcancel : *const windows_sys::core::BOOL, dwcopyflags : u32, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn CopyFileW(lpexistingfilename : windows_sys::core::PCWSTR, lpnewfilename : windows_sys::core::PCWSTR, bfailifexists : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateActCtxA(pactctx : *const ACTCTXA) -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateActCtxW(pactctx : *const ACTCTXW) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateBoundaryDescriptorA(name : windows_sys::core::PCSTR, flags : u32) -> super::HANDLE);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn CreateDirectoryExA(lptemplatedirectory : windows_sys::core::PCSTR, lpnewdirectory : windows_sys::core::PCSTR, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn CreateDirectoryExW(lptemplatedirectory : windows_sys::core::PCWSTR, lpnewdirectory : windows_sys::core::PCWSTR, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateDirectoryTransactedA(lptemplatedirectory : windows_sys::core::PCSTR, lpnewdirectory : windows_sys::core::PCSTR, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateDirectoryTransactedW(lptemplatedirectory : windows_sys::core::PCWSTR, lpnewdirectory : windows_sys::core::PCWSTR, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn CreateFiber(dwstacksize : usize, lpstartaddress : LPFIBER_START_ROUTINE, lpparameter : *const core::ffi::c_void) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn CreateFiberEx(dwstackcommitsize : usize, dwstackreservesize : usize, dwflags : u32, lpstartaddress : LPFIBER_START_ROUTINE, lpparameter : *const core::ffi::c_void) -> *mut core::ffi::c_void);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFileMappingA(hfile : super::HANDLE, lpfilemappingattributes : *const super::SECURITY_ATTRIBUTES, flprotect : u32, dwmaximumsizehigh : u32, dwmaximumsizelow : u32, lpname : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFileMappingNumaA(hfile : super::HANDLE, lpfilemappingattributes : *const super::SECURITY_ATTRIBUTES, flprotect : u32, dwmaximumsizehigh : u32, dwmaximumsizelow : u32, lpname : windows_sys::core::PCSTR, nndpreferred : u32) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFileTransactedA(lpfilename : windows_sys::core::PCSTR, dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, dwcreationdisposition : u32, dwflagsandattributes : u32, htemplatefile : super::HANDLE, htransaction : super::HANDLE, pusminiversion : *const u16, lpextendedparameter : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateFileTransactedW(lpfilename : windows_sys::core::PCWSTR, dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, dwcreationdisposition : u32, dwflagsandattributes : u32, htemplatefile : super::HANDLE, htransaction : super::HANDLE, pusminiversion : *const u16, lpextendedparameter : *const core::ffi::c_void) -> super::HANDLE);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn CreateHardLinkA(lpfilename : windows_sys::core::PCSTR, lpexistingfilename : windows_sys::core::PCSTR, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateHardLinkTransactedA(lpfilename : windows_sys::core::PCSTR, lpexistingfilename : windows_sys::core::PCSTR, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateHardLinkTransactedW(lpfilename : windows_sys::core::PCWSTR, lpexistingfilename : windows_sys::core::PCWSTR, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn CreateHardLinkW(lpfilename : windows_sys::core::PCWSTR, lpexistingfilename : windows_sys::core::PCWSTR, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateJobObjectA(lpjobattributes : *const super::SECURITY_ATTRIBUTES, lpname : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateJobSet(numjob : u32, userjobset : *const super::JOB_SET_ARRAY, flags : u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateMailslotA(lpname : windows_sys::core::PCSTR, nmaxmessagesize : u32, lreadtimeout : u32, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateMailslotW(lpname : windows_sys::core::PCWSTR, nmaxmessagesize : u32, lreadtimeout : u32, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateNamedPipeA(lpname : windows_sys::core::PCSTR, dwopenmode : u32, dwpipemode : u32, nmaxinstances : u32, noutbuffersize : u32, ninbuffersize : u32, ndefaulttimeout : u32, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreatePrivateNamespaceA(lpprivatenamespaceattributes : *const super::SECURITY_ATTRIBUTES, lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(all(feature = "minwindef", feature = "processthreadsapi", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn CreateProcessWithLogonW(lpusername : windows_sys::core::PCWSTR, lpdomain : windows_sys::core::PCWSTR, lppassword : windows_sys::core::PCWSTR, dwlogonflags : u32, lpapplicationname : windows_sys::core::PCWSTR, lpcommandline : windows_sys::core::PWSTR, dwcreationflags : u32, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_sys::core::PCWSTR, lpstartupinfo : *const super::STARTUPINFOW, lpprocessinformation : *mut super::PROCESS_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "processthreadsapi", feature = "winnt"))]
windows_link::link!("advapi32.dll" "system" fn CreateProcessWithTokenW(htoken : super::HANDLE, dwlogonflags : u32, lpapplicationname : windows_sys::core::PCWSTR, lpcommandline : windows_sys::core::PWSTR, dwcreationflags : u32, lpenvironment : *const core::ffi::c_void, lpcurrentdirectory : windows_sys::core::PCWSTR, lpstartupinfo : *const super::STARTUPINFOW, lpprocessinformation : *mut super::PROCESS_INFORMATION) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateSemaphoreA(lpsemaphoreattributes : *const super::SECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateSemaphoreExA(lpsemaphoreattributes : *const super::SECURITY_ATTRIBUTES, linitialcount : i32, lmaximumcount : i32, lpname : windows_sys::core::PCSTR, dwflags : u32, dwdesiredaccess : u32) -> super::HANDLE);
windows_link::link!("kernel32.dll" "system" fn CreateSymbolicLinkA(lpsymlinkfilename : windows_sys::core::PCSTR, lptargetfilename : windows_sys::core::PCSTR, dwflags : u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateSymbolicLinkTransactedA(lpsymlinkfilename : windows_sys::core::PCSTR, lptargetfilename : windows_sys::core::PCSTR, dwflags : u32, htransaction : super::HANDLE) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateSymbolicLinkTransactedW(lpsymlinkfilename : windows_sys::core::PCWSTR, lptargetfilename : windows_sys::core::PCWSTR, dwflags : u32, htransaction : super::HANDLE) -> bool);
windows_link::link!("kernel32.dll" "system" fn CreateSymbolicLinkW(lpsymlinkfilename : windows_sys::core::PCWSTR, lptargetfilename : windows_sys::core::PCWSTR, dwflags : u32) -> bool);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn CreateTapePartition(hdevice : super::HANDLE, dwpartitionmethod : u32, dwcount : u32, dwsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn CreateUmsCompletionList(umscompletionlist : *mut PUMS_COMPLETION_LIST) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn CreateUmsThreadContext(lpumsthread : *mut PUMS_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateWaitableTimerA(lptimerattributes : *const super::SECURITY_ATTRIBUTES, bmanualreset : windows_sys::core::BOOL, lptimername : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn CreateWaitableTimerExA(lptimerattributes : *const super::SECURITY_ATTRIBUTES, lptimername : windows_sys::core::PCSTR, dwflags : u32, dwdesiredaccess : u32) -> super::HANDLE);
windows_link::link!("kernel32.dll" "system" fn DeactivateActCtx(dwflags : u32, ulcookie : usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DebugBreakProcess(process : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DebugSetProcessKillOnExit(killonexit : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn DecryptFileA(lpfilename : windows_sys::core::PCSTR, dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn DecryptFileW(lpfilename : windows_sys::core::PCWSTR, dwreserved : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DefineDosDeviceA(dwflags : u32, lpdevicename : windows_sys::core::PCSTR, lptargetpath : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn DeleteAtom(natom : super::ATOM) -> super::ATOM);
windows_link::link!("kernel32.dll" "system" fn DeleteFiber(lpfiber : *const core::ffi::c_void));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteFileTransactedA(lpfilename : windows_sys::core::PCSTR, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DeleteFileTransactedW(lpfilename : windows_sys::core::PCWSTR, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DeleteUmsCompletionList(umscompletionlist : PUMS_COMPLETION_LIST) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DeleteUmsThreadContext(umsthread : PUMS_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DeleteVolumeMountPointA(lpszvolumemountpoint : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DequeueUmsCompletionListItems(umscompletionlist : PUMS_COMPLETION_LIST, waittimeout : u32, umsthreadlist : *mut PUMS_CONTEXT) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn DeregisterEventSource(heventlog : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn DisableThreadProfiling(performancedatahandle : super::HANDLE) -> u32);
windows_link::link!("kernel32.dll" "system" fn DnsHostnameToComputerNameA(hostname : windows_sys::core::PCSTR, computername : windows_sys::core::PSTR, nsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn DnsHostnameToComputerNameW(hostname : windows_sys::core::PCWSTR, computername : windows_sys::core::PWSTR, nsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn DosDateTimeToFileTime(wfatdate : u16, wfattime : u16, lpfiletime : *mut super::FILETIME) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn EnableProcessOptionalXStateFeatures(features : u64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnableThreadProfiling(threadhandle : super::HANDLE, flags : u32, hardwarecounters : u64, performancedatahandle : *mut super::HANDLE) -> u32);
windows_link::link!("advapi32.dll" "system" fn EncryptFileA(lpfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn EncryptFileW(lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EndUpdateResourceA(hupdate : super::HANDLE, fdiscard : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EndUpdateResourceW(hupdate : super::HANDLE, fdiscard : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EnterUmsSchedulingMode(schedulerstartupinfo : *const UMS_SCHEDULER_STARTUP_INFO) -> windows_sys::core::BOOL);
#[cfg(all(feature = "libloaderapi", feature = "minwindef"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceLanguagesA(hmodule : super::HMODULE, lptype : windows_sys::core::PCSTR, lpname : windows_sys::core::PCSTR, lpenumfunc : super::ENUMRESLANGPROCA, lparam : isize) -> windows_sys::core::BOOL);
#[cfg(all(feature = "libloaderapi", feature = "minwindef"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceLanguagesW(hmodule : super::HMODULE, lptype : windows_sys::core::PCWSTR, lpname : windows_sys::core::PCWSTR, lpenumfunc : super::ENUMRESLANGPROCW, lparam : isize) -> windows_sys::core::BOOL);
#[cfg(all(feature = "libloaderapi", feature = "minwindef"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceTypesA(hmodule : super::HMODULE, lpenumfunc : super::ENUMRESTYPEPROCA, lparam : isize) -> windows_sys::core::BOOL);
#[cfg(all(feature = "libloaderapi", feature = "minwindef"))]
windows_link::link!("kernel32.dll" "system" fn EnumResourceTypesW(hmodule : super::HMODULE, lpenumfunc : super::ENUMRESTYPEPROCW, lparam : isize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EraseTape(hdevice : super::HANDLE, dwerasetype : u32, bimmediate : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn EscapeCommFunction(hfile : super::HANDLE, dwfunc : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ExecuteUmsThread(umsthread : PUMS_CONTEXT) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn FatalExit(exitcode : i32));
windows_link::link!("advapi32.dll" "system" fn FileEncryptionStatusA(lpfilename : windows_sys::core::PCSTR, lpstatus : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn FileEncryptionStatusW(lpfilename : windows_sys::core::PCWSTR, lpstatus : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn FileTimeToDosDateTime(lpfiletime : *const super::FILETIME, lpfatdate : *mut u16, lpfattime : *mut u16) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindActCtxSectionGuid(dwflags : u32, lpextensionguid : *const windows_sys::core::GUID, ulsectionid : u32, lpguidtofind : *const windows_sys::core::GUID, returneddata : *mut ACTCTX_SECTION_KEYED_DATA) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindActCtxSectionStringA(dwflags : u32, lpextensionguid : *const windows_sys::core::GUID, ulsectionid : u32, lpstringtofind : windows_sys::core::PCSTR, returneddata : *mut ACTCTX_SECTION_KEYED_DATA) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindActCtxSectionStringW(dwflags : u32, lpextensionguid : *const windows_sys::core::GUID, ulsectionid : u32, lpstringtofind : windows_sys::core::PCWSTR, returneddata : *mut ACTCTX_SECTION_KEYED_DATA) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn FindAtomA(lpstring : windows_sys::core::PCSTR) -> super::ATOM);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn FindAtomW(lpstring : windows_sys::core::PCWSTR) -> super::ATOM);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindFirstFileNameTransactedW(lpfilename : windows_sys::core::PCWSTR, dwflags : u32, stringlength : *mut u32, linkname : windows_sys::core::PWSTR, htransaction : super::HANDLE) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstFileTransactedA(lpfilename : windows_sys::core::PCSTR, finfolevelid : super::FINDEX_INFO_LEVELS, lpfindfiledata : *mut core::ffi::c_void, fsearchop : super::FINDEX_SEARCH_OPS, lpsearchfilter : *const core::ffi::c_void, dwadditionalflags : u32, htransaction : super::HANDLE) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstFileTransactedW(lpfilename : windows_sys::core::PCWSTR, finfolevelid : super::FINDEX_INFO_LEVELS, lpfindfiledata : *mut core::ffi::c_void, fsearchop : super::FINDEX_SEARCH_OPS, lpsearchfilter : *const core::ffi::c_void, dwadditionalflags : u32, htransaction : super::HANDLE) -> super::HANDLE);
#[cfg(all(feature = "fileapi", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn FindFirstStreamTransactedW(lpfilename : windows_sys::core::PCWSTR, infolevel : super::STREAM_INFO_LEVELS, lpfindstreamdata : *mut core::ffi::c_void, dwflags : u32, htransaction : super::HANDLE) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindFirstVolumeA(lpszvolumename : windows_sys::core::PSTR, cchbufferlength : u32) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindFirstVolumeMountPointA(lpszrootpathname : windows_sys::core::PCSTR, lpszvolumemountpoint : windows_sys::core::PSTR, cchbufferlength : u32) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindFirstVolumeMountPointW(lpszrootpathname : windows_sys::core::PCWSTR, lpszvolumemountpoint : windows_sys::core::PWSTR, cchbufferlength : u32) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindNextVolumeA(hfindvolume : super::HANDLE, lpszvolumename : windows_sys::core::PSTR, cchbufferlength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindNextVolumeMountPointA(hfindvolumemountpoint : super::HANDLE, lpszvolumemountpoint : windows_sys::core::PSTR, cchbufferlength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindNextVolumeMountPointW(hfindvolumemountpoint : super::HANDLE, lpszvolumemountpoint : windows_sys::core::PWSTR, cchbufferlength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn FindResourceA(hmodule : super::HMODULE, lpname : windows_sys::core::PCSTR, lptype : windows_sys::core::PCSTR) -> super::HRSRC);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn FindResourceExA(hmodule : super::HMODULE, lptype : windows_sys::core::PCSTR, lpname : windows_sys::core::PCSTR, wlanguage : u16) -> super::HRSRC);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn FindVolumeMountPointClose(hfindvolumemountpoint : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "vadefs")]
windows_link::link!("kernel32.dll" "system" fn FormatMessageA(dwflags : u32, lpsource : *const core::ffi::c_void, dwmessageid : u32, dwlanguageid : u32, lpbuffer : windows_sys::core::PCSTR, nsize : u32, arguments : *const super::va_list) -> u32);
#[cfg(feature = "vadefs")]
windows_link::link!("kernel32.dll" "system" fn FormatMessageW(dwflags : u32, lpsource : *const core::ffi::c_void, dwmessageid : u32, dwlanguageid : u32, lpbuffer : windows_sys::core::PCWSTR, nsize : u32, arguments : *const super::va_list) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetActiveProcessorCount(groupnumber : u16) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetActiveProcessorGroupCount() -> u16);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetApplicationRecoveryCallback(hprocess : super::HANDLE, precoverycallback : *mut APPLICATION_RECOVERY_CALLBACK, ppvparameter : *mut *mut core::ffi::c_void, pdwpinginterval : *mut u32, pdwflags : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetApplicationRestartSettings(hprocess : super::HANDLE, pwzcommandline : windows_sys::core::PWSTR, pcchsize : *mut u32, pdwflags : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetAtomNameA(natom : super::ATOM, lpbuffer : windows_sys::core::PSTR, nsize : i32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GetAtomNameW(natom : super::ATOM, lpbuffer : windows_sys::core::PWSTR, nsize : i32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetBinaryTypeA(lpapplicationname : windows_sys::core::PCSTR, lpbinarytype : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetBinaryTypeW(lpapplicationname : windows_sys::core::PCWSTR, lpbinarytype : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCommConfig(hcommdev : super::HANDLE, lpcc : *mut COMMCONFIG, lpdwsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCommMask(hfile : super::HANDLE, lpevtmask : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCommModemStatus(hfile : super::HANDLE, lpmodemstat : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-core-comm-l1-1-2.dll" "system" fn GetCommPorts(lpportnumbers : *mut u32, uportnumberscount : u32, puportnumbersfound : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCommProperties(hfile : super::HANDLE, lpcommprop : *mut COMMPROP) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCommState(hfile : super::HANDLE, lpdcb : *mut DCB) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCommTimeouts(hfile : super::HANDLE, lpcommtimeouts : *mut COMMTIMEOUTS) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCompressedFileSizeTransactedA(lpfilename : windows_sys::core::PCSTR, lpfilesizehigh : *mut u32, htransaction : super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCompressedFileSizeTransactedW(lpfilename : windows_sys::core::PCWSTR, lpfilesizehigh : *mut u32, htransaction : super::HANDLE) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetComputerNameA(lpbuffer : windows_sys::core::PSTR, nsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetComputerNameW(lpbuffer : windows_sys::core::PWSTR, nsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetCurrentActCtx(lphactctx : *mut super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn GetCurrentHwProfileA(lphwprofileinfo : *mut HW_PROFILE_INFOA) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn GetCurrentHwProfileW(lphwprofileinfo : *mut HW_PROFILE_INFOW) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetCurrentUmsThread() -> PUMS_CONTEXT);
windows_link::link!("kernel32.dll" "system" fn GetDefaultCommConfigA(lpszname : windows_sys::core::PCSTR, lpcc : *mut COMMCONFIG, lpdwsize : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetDefaultCommConfigW(lpszname : windows_sys::core::PCWSTR, lpcc : *mut COMMCONFIG, lpdwsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetDevicePowerState(hdevice : super::HANDLE, pfon : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetDllDirectoryA(nbufferlength : u32, lpbuffer : windows_sys::core::PSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetDllDirectoryW(nbufferlength : u32, lpbuffer : windows_sys::core::PWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetEnabledXStateFeatures() -> u64);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetEventLogInformation(heventlog : super::HANDLE, dwinfolevel : u32, lpbuffer : *mut core::ffi::c_void, cbbufsize : u32, pcbbytesneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetFileAttributesTransactedA(lpfilename : windows_sys::core::PCSTR, finfolevelid : super::GET_FILEEX_INFO_LEVELS, lpfileinformation : *mut core::ffi::c_void, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetFileAttributesTransactedW(lpfilename : windows_sys::core::PCWSTR, finfolevelid : super::GET_FILEEX_INFO_LEVELS, lpfileinformation : *mut core::ffi::c_void, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetFileBandwidthReservation(hfile : super::HANDLE, lpperiodmilliseconds : *mut u32, lpbytesperperiod : *mut u32, pdiscardable : *mut windows_sys::core::BOOL, lptransfersize : *mut u32, lpnumoutstandingrequests : *mut u32) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetFileInformationByHandleEx(hfile : super::HANDLE, fileinformationclass : super::FILE_INFO_BY_HANDLE_CLASS, lpfileinformation : *mut core::ffi::c_void, dwbuffersize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "minwinbase")]
windows_link::link!("kernel32.dll" "system" fn GetFileInformationByName(filename : windows_sys::core::PCWSTR, fileinformationclass : super::FILE_INFO_BY_NAME_CLASS, fileinfobuffer : *mut core::ffi::c_void, fileinfobuffersize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetFileSecurityA(lpfilename : windows_sys::core::PCSTR, requestedinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableA(lpname : windows_sys::core::PCSTR, lpguid : windows_sys::core::PCSTR, pbuffer : *mut core::ffi::c_void, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableExA(lpname : windows_sys::core::PCSTR, lpguid : windows_sys::core::PCSTR, pbuffer : *mut core::ffi::c_void, nsize : u32, pdwattribubutes : *mut u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableExW(lpname : windows_sys::core::PCWSTR, lpguid : windows_sys::core::PCWSTR, pbuffer : *mut core::ffi::c_void, nsize : u32, pdwattribubutes : *mut u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableW(lpname : windows_sys::core::PCWSTR, lpguid : windows_sys::core::PCWSTR, pbuffer : *mut core::ffi::c_void, nsize : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetFirmwareType(firmwaretype : *mut super::FIRMWARE_TYPE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetFullPathNameTransactedA(lpfilename : windows_sys::core::PCSTR, nbufferlength : u32, lpbuffer : windows_sys::core::PSTR, lpfilepart : *mut windows_sys::core::PSTR, htransaction : super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetFullPathNameTransactedW(lpfilename : windows_sys::core::PCWSTR, nbufferlength : u32, lpbuffer : windows_sys::core::PWSTR, lpfilepart : *mut windows_sys::core::PWSTR, htransaction : super::HANDLE) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetLogicalDriveStringsA(nbufferlength : u32, lpbuffer : windows_sys::core::PSTR) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetLongPathNameTransactedA(lpszshortpath : windows_sys::core::PCSTR, lpszlongpath : windows_sys::core::PSTR, cchbuffer : u32, htransaction : super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetLongPathNameTransactedW(lpszshortpath : windows_sys::core::PCWSTR, lpszlongpath : windows_sys::core::PWSTR, cchbuffer : u32, htransaction : super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetMailslotInfo(hmailslot : super::HANDLE, lpmaxmessagesize : *mut u32, lpnextsize : *mut u32, lpmessagecount : *mut u32, lpreadtimeout : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetMaximumProcessorCount(groupnumber : u16) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetMaximumProcessorGroupCount() -> u16);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameA(pipe : super::HANDLE, clientcomputername : windows_sys::core::PSTR, clientcomputernamelength : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeClientProcessId(pipe : super::HANDLE, clientprocessid : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeClientSessionId(pipe : super::HANDLE, clientsessionid : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateA(hnamedpipe : super::HANDLE, lpstate : *mut u32, lpcurinstances : *mut u32, lpmaxcollectioncount : *mut u32, lpcollectdatatimeout : *mut u32, lpusername : windows_sys::core::PSTR, nmaxusernamesize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeServerProcessId(pipe : super::HANDLE, serverprocessid : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNamedPipeServerSessionId(pipe : super::HANDLE, serversessionid : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetNextUmsListItem(umscontext : PUMS_CONTEXT) -> PUMS_CONTEXT);
windows_link::link!("kernel32.dll" "system" fn GetNumaAvailableMemoryNode(node : u8, availablebytes : *mut u64) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetNumaAvailableMemoryNodeEx(node : u16, availablebytes : *mut u64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNumaNodeNumberFromHandle(hfile : super::HANDLE, nodenumber : *mut u16) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMask(node : u8, processormask : *mut u64) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetNumaProcessorNode(processor : u8, nodenumber : *mut u8) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetNumaProcessorNodeEx(processor : *const super::PROCESSOR_NUMBER, nodenumber : *mut u16) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetNumaProximityNode(proximityid : u32, nodenumber : *mut u8) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetNumberOfEventLogRecords(heventlog : super::HANDLE, numberofrecords : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn GetOldestEventLogRecord(heventlog : super::HANDLE, oldestrecord : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetPrivateProfileIntA(lpappname : windows_sys::core::PCSTR, lpkeyname : windows_sys::core::PCSTR, ndefault : i32, lpfilename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetPrivateProfileIntW(lpappname : windows_sys::core::PCWSTR, lpkeyname : windows_sys::core::PCWSTR, ndefault : i32, lpfilename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetPrivateProfileSectionA(lpappname : windows_sys::core::PCSTR, lpreturnedstring : windows_sys::core::PSTR, nsize : u32, lpfilename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetPrivateProfileSectionNamesA(lpszreturnbuffer : windows_sys::core::PSTR, nsize : u32, lpfilename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetPrivateProfileSectionNamesW(lpszreturnbuffer : windows_sys::core::PWSTR, nsize : u32, lpfilename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetPrivateProfileSectionW(lpappname : windows_sys::core::PCWSTR, lpreturnedstring : windows_sys::core::PWSTR, nsize : u32, lpfilename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetPrivateProfileStringA(lpappname : windows_sys::core::PCSTR, lpkeyname : windows_sys::core::PCSTR, lpdefault : windows_sys::core::PCSTR, lpreturnedstring : windows_sys::core::PSTR, nsize : u32, lpfilename : windows_sys::core::PCSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetPrivateProfileStringW(lpappname : windows_sys::core::PCWSTR, lpkeyname : windows_sys::core::PCWSTR, lpdefault : windows_sys::core::PCWSTR, lpreturnedstring : windows_sys::core::PWSTR, nsize : u32, lpfilename : windows_sys::core::PCWSTR) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetPrivateProfileStructA(lpszsection : windows_sys::core::PCSTR, lpszkey : windows_sys::core::PCSTR, lpstruct : *mut core::ffi::c_void, usizestruct : u32, szfile : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetPrivateProfileStructW(lpszsection : windows_sys::core::PCWSTR, lpszkey : windows_sys::core::PCWSTR, lpstruct : *mut core::ffi::c_void, usizestruct : u32, szfile : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessAffinityMask(hprocess : super::HANDLE, lpprocessaffinitymask : *mut usize, lpsystemaffinitymask : *mut usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessDEPPolicy(hprocess : super::HANDLE, lpflags : *mut u32, lppermanent : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetProcessIoCounters(hprocess : super::HANDLE, lpiocounters : *mut super::IO_COUNTERS) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetProfileIntA(lpappname : windows_sys::core::PCSTR, lpkeyname : windows_sys::core::PCSTR, ndefault : i32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetProfileIntW(lpappname : windows_sys::core::PCWSTR, lpkeyname : windows_sys::core::PCWSTR, ndefault : i32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetProfileSectionA(lpappname : windows_sys::core::PCSTR, lpreturnedstring : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetProfileSectionW(lpappname : windows_sys::core::PCWSTR, lpreturnedstring : windows_sys::core::PWSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetProfileStringA(lpappname : windows_sys::core::PCSTR, lpkeyname : windows_sys::core::PCSTR, lpdefault : windows_sys::core::PCSTR, lpreturnedstring : windows_sys::core::PSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetProfileStringW(lpappname : windows_sys::core::PCWSTR, lpkeyname : windows_sys::core::PCWSTR, lpdefault : windows_sys::core::PCWSTR, lpreturnedstring : windows_sys::core::PWSTR, nsize : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetShortPathNameA(lpszlongpath : windows_sys::core::PCSTR, lpszshortpath : windows_sys::core::PSTR, cchbuffer : u32) -> u32);
#[cfg(all(feature = "minwindef", feature = "processthreadsapi", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GetStartupInfoA(lpstartupinfo : *mut super::STARTUPINFOA));
windows_link::link!("kernel32.dll" "system" fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE);
windows_link::link!("kernel32.dll" "system" fn GetSystemPowerStatus(lpsystempowerstatus : *mut SYSTEM_POWER_STATUS) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetSystemRegistryQuota(pdwquotaallowed : *mut u32, pdwquotaused : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetTapeParameters(hdevice : super::HANDLE, dwoperation : u32, lpdwsize : *mut u32, lptapeinformation : *mut core::ffi::c_void) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetTapePosition(hdevice : super::HANDLE, dwpositiontype : u32, lpdwpartition : *mut u32, lpdwoffsetlow : *mut u32, lpdwoffsethigh : *mut u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetTapeStatus(hdevice : super::HANDLE) -> u32);
windows_link::link!("kernel32.dll" "system" fn GetThreadEnabledXStateFeatures() -> u64);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetThreadSelectorEntry(hthread : super::HANDLE, dwselector : u32, lpselectorentry : LPLDT_ENTRY) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetUmsCompletionListEvent(umscompletionlist : PUMS_COMPLETION_LIST, umscompletionevent : *mut super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetUmsSystemThreadInformation(threadhandle : super::HANDLE, systemthreadinfo : *mut UMS_SYSTEM_THREAD_INFORMATION) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn GetUserNameA(lpbuffer : windows_sys::core::PSTR, pcbbuffer : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn GetUserNameW(lpbuffer : windows_sys::core::PWSTR, pcbbuffer : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetVolumeNameForVolumeMountPointA(lpszvolumemountpoint : windows_sys::core::PCSTR, lpszvolumename : windows_sys::core::PSTR, cchbufferlength : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetVolumePathNameA(lpszfilename : windows_sys::core::PCSTR, lpszvolumepathname : windows_sys::core::PSTR, cchbufferlength : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn GetVolumePathNamesForVolumeNameA(lpszvolumename : windows_sys::core::PCSTR, lpszvolumepathnames : *mut i8, cchbufferlength : u32, lpcchreturnlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetXStateFeaturesMask(context : *const super::CONTEXT, featuremask : *mut u64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn GetXStateFeaturesMask(context : *const super::ARM64_NT_CONTEXT, featuremask : *mut u64) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GlobalAddAtomA(lpstring : windows_sys::core::PCSTR) -> super::ATOM);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GlobalAddAtomExA(lpstring : windows_sys::core::PCSTR, flags : u32) -> super::ATOM);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GlobalAddAtomExW(lpstring : windows_sys::core::PCWSTR, flags : u32) -> super::ATOM);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GlobalAddAtomW(lpstring : windows_sys::core::PCWSTR) -> super::ATOM);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GlobalAlloc(uflags : u32, dwbytes : usize) -> super::HGLOBAL);
windows_link::link!("kernel32.dll" "system" fn GlobalCompact(dwminfree : u32) -> usize);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GlobalDeleteAtom(natom : super::ATOM) -> super::ATOM);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GlobalFindAtomA(lpstring : windows_sys::core::PCSTR) -> super::ATOM);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GlobalFindAtomW(lpstring : windows_sys::core::PCWSTR) -> super::ATOM);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GlobalFix(hmem : super::HGLOBAL));
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GlobalFlags(hmem : super::HGLOBAL) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GlobalFree(hmem : super::HGLOBAL) -> super::HGLOBAL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GlobalGetAtomNameA(natom : super::ATOM, lpbuffer : windows_sys::core::PSTR, nsize : i32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn GlobalGetAtomNameW(natom : super::ATOM, lpbuffer : windows_sys::core::PWSTR, nsize : i32) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GlobalHandle(pmem : *const core::ffi::c_void) -> super::HGLOBAL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GlobalLock(hmem : super::HGLOBAL) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn GlobalMemoryStatus(lpbuffer : *mut MEMORYSTATUS));
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GlobalReAlloc(hmem : super::HGLOBAL, dwbytes : usize, uflags : u32) -> super::HGLOBAL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GlobalSize(hmem : super::HGLOBAL) -> usize);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GlobalUnWire(hmem : super::HGLOBAL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GlobalUnfix(hmem : super::HGLOBAL));
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GlobalUnlock(hmem : super::HGLOBAL) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn GlobalWire(hmem : super::HGLOBAL) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn InitAtomTable(nsize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InitializeContext(buffer : *mut core::ffi::c_void, contextflags : u32, context : *mut super::PCONTEXT, contextlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn InitializeContext2(buffer : *mut core::ffi::c_void, contextflags : u32, context : *mut super::PCONTEXT, contextlength : *mut u32, xstatecompactionmask : u64) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn IsBadCodePtr(lpfn : super::FARPROC) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsBadHugeReadPtr(lp : *const core::ffi::c_void, ucb : usize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsBadHugeWritePtr(lp : *const core::ffi::c_void, ucb : usize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsBadReadPtr(lp : *const core::ffi::c_void, ucb : usize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsBadStringPtrA(lpsz : windows_sys::core::PCSTR, ucchmax : usize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsBadStringPtrW(lpsz : windows_sys::core::PCWSTR, ucchmax : usize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsBadWritePtr(lp : *const core::ffi::c_void, ucb : usize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsNativeVhdBoot(nativevhdboot : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn IsSystemResumeAutomatic() -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn IsTextUnicode(lpv : *const core::ffi::c_void, isize : i32, lpiresult : *mut i32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn IsTokenUntrusted(tokenhandle : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn LoadModule(lpmodulename : windows_sys::core::PCSTR, lpparameterblock : *const core::ffi::c_void) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn LoadPackagedLibrary(lpwlibfilename : windows_sys::core::PCWSTR, reserved : u32) -> super::HMODULE);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LocalAlloc(uflags : u32, ubytes : usize) -> super::HLOCAL);
windows_link::link!("kernel32.dll" "system" fn LocalCompact(uminfree : u32) -> usize);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LocalFlags(hmem : super::HLOCAL) -> u32);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LocalFree(hmem : super::HLOCAL) -> super::HLOCAL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LocalHandle(pmem : *const core::ffi::c_void) -> super::HLOCAL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LocalLock(hmem : super::HLOCAL) -> *mut core::ffi::c_void);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LocalReAlloc(hmem : super::HLOCAL, ubytes : usize, uflags : u32) -> super::HLOCAL);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LocalShrink(hmem : super::HLOCAL, cbnewsize : u32) -> usize);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LocalSize(hmem : super::HLOCAL) -> usize);
#[cfg(all(feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn LocalUnlock(hmem : super::HLOCAL) -> windows_sys::core::BOOL);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn LocateXStateFeature(context : *const super::CONTEXT, featureid : u32, length : *mut u32) -> *mut core::ffi::c_void);
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn LocateXStateFeature(context : *const super::ARM64_NT_CONTEXT, featureid : u32, length : *mut u32) -> *mut core::ffi::c_void);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn LogonUserA(lpszusername : windows_sys::core::PCSTR, lpszdomain : windows_sys::core::PCSTR, lpszpassword : windows_sys::core::PCSTR, dwlogontype : u32, dwlogonprovider : u32, phtoken : *mut super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn LogonUserExA(lpszusername : windows_sys::core::PCSTR, lpszdomain : windows_sys::core::PCSTR, lpszpassword : windows_sys::core::PCSTR, dwlogontype : u32, dwlogonprovider : u32, phtoken : *mut super::HANDLE, pplogonsid : *mut super::PSID, ppprofilebuffer : *mut *mut core::ffi::c_void, pdwprofilelength : *mut u32, pquotalimits : *mut super::QUOTA_LIMITS) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn LogonUserExW(lpszusername : windows_sys::core::PCWSTR, lpszdomain : windows_sys::core::PCWSTR, lpszpassword : windows_sys::core::PCWSTR, dwlogontype : u32, dwlogonprovider : u32, phtoken : *mut super::HANDLE, pplogonsid : *mut super::PSID, ppprofilebuffer : *mut *mut core::ffi::c_void, pdwprofilelength : *mut u32, pquotalimits : *mut super::QUOTA_LIMITS) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn LogonUserW(lpszusername : windows_sys::core::PCWSTR, lpszdomain : windows_sys::core::PCWSTR, lpszpassword : windows_sys::core::PCWSTR, dwlogontype : u32, dwlogonprovider : u32, phtoken : *mut super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn LookupAccountNameA(lpsystemname : windows_sys::core::PCSTR, lpaccountname : windows_sys::core::PCSTR, sid : super::PSID, cbsid : *mut u32, referenceddomainname : windows_sys::core::PSTR, cchreferenceddomainname : *mut u32, peuse : *mut super::SID_NAME_USE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn LookupAccountNameW(lpsystemname : windows_sys::core::PCWSTR, lpaccountname : windows_sys::core::PCWSTR, sid : super::PSID, cbsid : *mut u32, referenceddomainname : windows_sys::core::PWSTR, cchreferenceddomainname : *mut u32, peuse : *mut super::SID_NAME_USE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn LookupAccountSidA(lpsystemname : windows_sys::core::PCSTR, sid : super::PSID, name : windows_sys::core::PSTR, cchname : *mut u32, referenceddomainname : windows_sys::core::PSTR, cchreferenceddomainname : *mut u32, peuse : *mut super::SID_NAME_USE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn LookupAccountSidW(lpsystemname : windows_sys::core::PCWSTR, sid : super::PSID, name : windows_sys::core::PWSTR, cchname : *mut u32, referenceddomainname : windows_sys::core::PWSTR, cchreferenceddomainname : *mut u32, peuse : *mut super::SID_NAME_USE) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn LookupPrivilegeDisplayNameA(lpsystemname : windows_sys::core::PCSTR, lpname : windows_sys::core::PCSTR, lpdisplayname : windows_sys::core::PSTR, cchdisplayname : *mut u32, lplanguageid : *mut u32) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn LookupPrivilegeDisplayNameW(lpsystemname : windows_sys::core::PCWSTR, lpname : windows_sys::core::PCWSTR, lpdisplayname : windows_sys::core::PWSTR, cchdisplayname : *mut u32, lplanguageid : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn LookupPrivilegeNameA(lpsystemname : windows_sys::core::PCSTR, lpluid : *const super::LUID, lpname : windows_sys::core::PSTR, cchname : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn LookupPrivilegeNameW(lpsystemname : windows_sys::core::PCWSTR, lpluid : *const super::LUID, lpname : windows_sys::core::PWSTR, cchname : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn LookupPrivilegeValueA(lpsystemname : windows_sys::core::PCSTR, lpname : windows_sys::core::PCSTR, lpluid : *mut super::LUID) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn LookupPrivilegeValueW(lpsystemname : windows_sys::core::PCWSTR, lpname : windows_sys::core::PCWSTR, lpluid : *mut super::LUID) -> windows_sys::core::BOOL);
#[cfg(target_arch = "x86")]
windows_link::link!("kernel32.dll" "system" fn MapUserPhysicalPagesScatter(virtualaddresses : *const *const core::ffi::c_void, numberofpages : usize, pagearray : *const u32) -> windows_sys::core::BOOL);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
windows_link::link!("kernel32.dll" "system" fn MapUserPhysicalPagesScatter(virtualaddresses : *const *const core::ffi::c_void, numberofpages : usize, pagearray : *const u64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn MapViewOfFileExNuma(hfilemappingobject : super::HANDLE, dwdesiredaccess : u32, dwfileoffsethigh : u32, dwfileoffsetlow : u32, dwnumberofbytestomap : usize, lpbaseaddress : *const core::ffi::c_void, nndpreferred : u32) -> *mut core::ffi::c_void);
windows_link::link!("kernel32.dll" "system" fn MoveFileA(lpexistingfilename : windows_sys::core::PCSTR, lpnewfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn MoveFileExA(lpexistingfilename : windows_sys::core::PCSTR, lpnewfilename : windows_sys::core::PCSTR, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn MoveFileExW(lpexistingfilename : windows_sys::core::PCWSTR, lpnewfilename : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn MoveFileTransactedA(lpexistingfilename : windows_sys::core::PCSTR, lpnewfilename : windows_sys::core::PCSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, dwflags : u32, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn MoveFileTransactedW(lpexistingfilename : windows_sys::core::PCWSTR, lpnewfilename : windows_sys::core::PCWSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, dwflags : u32, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn MoveFileW(lpexistingfilename : windows_sys::core::PCWSTR, lpnewfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn MoveFileWithProgressA(lpexistingfilename : windows_sys::core::PCSTR, lpnewfilename : windows_sys::core::PCSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn MoveFileWithProgressW(lpexistingfilename : windows_sys::core::PCWSTR, lpnewfilename : windows_sys::core::PCWSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : *const core::ffi::c_void, dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn MulDiv(nnumber : i32, nnumerator : i32, ndenominator : i32) -> i32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn NotifyChangeEventLog(heventlog : super::HANDLE, hevent : super::HANDLE) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn ObjectCloseAuditAlarmA(subsystemname : windows_sys::core::PCSTR, handleid : *const core::ffi::c_void, generateonclose : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn ObjectDeleteAuditAlarmA(subsystemname : windows_sys::core::PCSTR, handleid : *const core::ffi::c_void, generateonclose : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ObjectOpenAuditAlarmA(subsystemname : windows_sys::core::PCSTR, handleid : *const core::ffi::c_void, objecttypename : windows_sys::core::PCSTR, objectname : windows_sys::core::PCSTR, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, clienttoken : super::HANDLE, desiredaccess : u32, grantedaccess : u32, privileges : *const super::PRIVILEGE_SET, objectcreation : windows_sys::core::BOOL, accessgranted : windows_sys::core::BOOL, generateonclose : *mut windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ObjectPrivilegeAuditAlarmA(subsystemname : windows_sys::core::PCSTR, handleid : *const core::ffi::c_void, clienttoken : super::HANDLE, desiredaccess : u32, privileges : *const super::PRIVILEGE_SET, accessgranted : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn OpenBackupEventLogA(lpuncservername : windows_sys::core::PCSTR, lpfilename : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn OpenBackupEventLogW(lpuncservername : windows_sys::core::PCWSTR, lpfilename : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-core-comm-l1-1-1.dll" "system" fn OpenCommPort(uportnumber : u32, dwdesiredaccess : u32, dwflagsandattributes : u32) -> super::HANDLE);
windows_link::link!("advapi32.dll" "system" fn OpenEncryptedFileRawA(lpfilename : windows_sys::core::PCSTR, ulflags : u32, pvcontext : *mut *mut core::ffi::c_void) -> u32);
windows_link::link!("advapi32.dll" "system" fn OpenEncryptedFileRawW(lpfilename : windows_sys::core::PCWSTR, ulflags : u32, pvcontext : *mut *mut core::ffi::c_void) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn OpenEventLogA(lpuncservername : windows_sys::core::PCSTR, lpsourcename : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn OpenEventLogW(lpuncservername : windows_sys::core::PCWSTR, lpsourcename : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn OpenFile(lpfilename : windows_sys::core::PCSTR, lpreopenbuff : *mut OFSTRUCT, ustyle : u32) -> super::HFILE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn OpenFileById(hvolumehint : super::HANDLE, lpfileid : *const FILE_ID_DESCRIPTOR, dwdesiredaccess : u32, dwsharemode : u32, lpsecurityattributes : *const super::SECURITY_ATTRIBUTES, dwflagsandattributes : u32) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenFileMappingA(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenJobObjectA(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenMutexA(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenPrivateNamespaceA(lpboundarydescriptor : *const core::ffi::c_void, lpaliasprefix : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenSemaphoreA(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lpname : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn OpenWaitableTimerA(dwdesiredaccess : u32, binherithandle : windows_sys::core::BOOL, lptimername : windows_sys::core::PCSTR) -> super::HANDLE);
windows_link::link!("advapi32.dll" "system" fn OperationEnd(operationendparams : *const OPERATION_END_PARAMETERS) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn OperationStart(operationstartparams : *const OPERATION_START_PARAMETERS) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn PowerClearRequest(powerrequest : super::HANDLE, requesttype : super::POWER_REQUEST_TYPE) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn PowerCreateRequest(context : *const super::REASON_CONTEXT) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn PowerSetRequest(powerrequest : super::HANDLE, requesttype : super::POWER_REQUEST_TYPE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn PrepareTape(hdevice : super::HANDLE, dwoperation : u32, bimmediate : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn PrivilegedServiceAuditAlarmA(subsystemname : windows_sys::core::PCSTR, servicename : windows_sys::core::PCSTR, clienttoken : super::HANDLE, privileges : *const super::PRIVILEGE_SET, accessgranted : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn PulseEvent(hevent : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn PurgeComm(hfile : super::HANDLE, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryActCtxSettingsW(dwflags : u32, hactctx : super::HANDLE, settingsnamespace : windows_sys::core::PCWSTR, settingname : windows_sys::core::PCWSTR, pvbuffer : windows_sys::core::PWSTR, dwbuffer : usize, pdwwrittenorrequired : *mut usize) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryActCtxW(dwflags : u32, hactctx : super::HANDLE, pvsubinstance : *const core::ffi::c_void, ulinfoclass : u32, pvbuffer : *mut core::ffi::c_void, cbbuffer : usize, pcbwrittenorrequired : *mut usize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn QueryDosDeviceA(lpdevicename : windows_sys::core::PCSTR, lptargetpath : windows_sys::core::PSTR, ucchmax : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryFullProcessImageNameA(hprocess : super::HANDLE, dwflags : u32, lpexename : windows_sys::core::PSTR, lpdwsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryFullProcessImageNameW(hprocess : super::HANDLE, dwflags : u32, lpexename : windows_sys::core::PWSTR, lpdwsize : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryThreadProfiling(threadhandle : super::HANDLE, enabled : *mut bool) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn QueryUmsThreadInformation(umsthread : PUMS_CONTEXT, umsthreadinfoclass : UMS_THREAD_INFO_CLASS, umsthreadinformation : *mut core::ffi::c_void, umsthreadinformationlength : u32, returnlength : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-core-backgroundtask-l1-1-0.dll" "system" fn RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig : *const super::CUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReOpenFile(horiginalfile : super::HANDLE, dwdesiredaccess : u32, dwsharemode : u32, dwflagsandattributes : u32) -> super::HANDLE);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadDirectoryChangesExW(hdirectory : super::HANDLE, lpbuffer : *mut core::ffi::c_void, nbufferlength : u32, bwatchsubtree : windows_sys::core::BOOL, dwnotifyfilter : u32, lpbytesreturned : *mut u32, lpoverlapped : *mut super::OVERLAPPED, lpcompletionroutine : super::LPOVERLAPPED_COMPLETION_ROUTINE, readdirectorynotifyinformationclass : super::READ_DIRECTORY_NOTIFY_INFORMATION_CLASS) -> windows_sys::core::BOOL);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn ReadDirectoryChangesW(hdirectory : super::HANDLE, lpbuffer : *mut core::ffi::c_void, nbufferlength : u32, bwatchsubtree : windows_sys::core::BOOL, dwnotifyfilter : u32, lpbytesreturned : *mut u32, lpoverlapped : *mut super::OVERLAPPED, lpcompletionroutine : super::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn ReadEncryptedFileRaw(pfexportcallback : PFE_EXPORT_FUNC, pvcallbackcontext : *const core::ffi::c_void, pvcontext : *const core::ffi::c_void) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ReadEventLogA(heventlog : super::HANDLE, dwreadflags : u32, dwrecordoffset : u32, lpbuffer : *mut core::ffi::c_void, nnumberofbytestoread : u32, pnbytesread : *mut u32, pnminnumberofbytesneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ReadEventLogW(heventlog : super::HANDLE, dwreadflags : u32, dwrecordoffset : u32, lpbuffer : *mut core::ffi::c_void, nnumberofbytestoread : u32, pnbytesread : *mut u32, pnminnumberofbytesneeded : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReadThreadProfilingData(performancedatahandle : super::HANDLE, flags : u32, performancedata : *mut super::PERFORMANCE_DATA) -> u32);
windows_link::link!("kernel32.dll" "system" fn RegisterApplicationRecoveryCallback(precoveycallback : APPLICATION_RECOVERY_CALLBACK, pvparameter : *const core::ffi::c_void, dwpinginterval : u32, dwflags : u32) -> windows_sys::core::HRESULT);
windows_link::link!("kernel32.dll" "system" fn RegisterApplicationRestart(pwzcommandline : windows_sys::core::PCWSTR, dwflags : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn RegisterEventSourceA(lpuncservername : windows_sys::core::PCSTR, lpsourcename : windows_sys::core::PCSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn RegisterEventSourceW(lpuncservername : windows_sys::core::PCWSTR, lpsourcename : windows_sys::core::PCWSTR) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn RegisterWaitForSingleObject(phnewwaitobject : *mut super::HANDLE, hobject : super::HANDLE, callback : super::WAITORTIMERCALLBACK, context : *const core::ffi::c_void, dwmilliseconds : u32, dwflags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ReleaseActCtx(hactctx : super::HANDLE));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn RemoveDirectoryTransactedA(lppathname : windows_sys::core::PCSTR, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn RemoveDirectoryTransactedW(lppathname : windows_sys::core::PCWSTR, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn RemoveSecureMemoryCacheCallback(pfncallback : super::PSECURE_MEMORY_CACHE_CALLBACK) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ReplaceFileA(lpreplacedfilename : windows_sys::core::PCSTR, lpreplacementfilename : windows_sys::core::PCSTR, lpbackupfilename : windows_sys::core::PCSTR, dwreplaceflags : u32, lpexclude : *const core::ffi::c_void, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ReplaceFileW(lpreplacedfilename : windows_sys::core::PCWSTR, lpreplacementfilename : windows_sys::core::PCWSTR, lpbackupfilename : windows_sys::core::PCWSTR, dwreplaceflags : u32, lpexclude : *const core::ffi::c_void, lpreserved : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn ReplacePartitionUnit(targetpartition : windows_sys::core::PCWSTR, sparepartition : windows_sys::core::PCWSTR, flags : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ReportEventA(heventlog : super::HANDLE, wtype : u16, wcategory : u16, dweventid : u32, lpusersid : super::PSID, wnumstrings : u16, dwdatasize : u32, lpstrings : *const windows_sys::core::PCSTR, lprawdata : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn ReportEventW(heventlog : super::HANDLE, wtype : u16, wcategory : u16, dweventid : u32, lpusersid : super::PSID, wnumstrings : u16, dwdatasize : u32, lpstrings : *const windows_sys::core::PCWSTR, lprawdata : *const core::ffi::c_void) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn RequestDeviceWakeup(hdevice : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn RequestWakeupLatency(latency : super::LATENCY_TIME) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetCommBreak(hfile : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetCommConfig(hcommdev : super::HANDLE, lpcc : *const COMMCONFIG, dwsize : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetCommMask(hfile : super::HANDLE, dwevtmask : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetCommState(hfile : super::HANDLE, lpdcb : *const DCB) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetCommTimeouts(hfile : super::HANDLE, lpcommtimeouts : *const COMMTIMEOUTS) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetDefaultCommConfigA(lpszname : windows_sys::core::PCSTR, lpcc : *const COMMCONFIG, dwsize : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetDefaultCommConfigW(lpszname : windows_sys::core::PCWSTR, lpcc : *const COMMCONFIG, dwsize : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetDllDirectoryA(lppathname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetDllDirectoryW(lppathname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetEnvironmentStringsA(newenvironment : *const i8) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetFileAttributesTransactedA(lpfilename : windows_sys::core::PCSTR, dwfileattributes : u32, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetFileAttributesTransactedW(lpfilename : windows_sys::core::PCWSTR, dwfileattributes : u32, htransaction : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetFileBandwidthReservation(hfile : super::HANDLE, nperiodmilliseconds : u32, nbytesperperiod : u32, bdiscardable : windows_sys::core::BOOL, lptransfersize : *mut u32, lpnumoutstandingrequests : *mut u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetFileCompletionNotificationModes(filehandle : super::HANDLE, flags : u8) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("advapi32.dll" "system" fn SetFileSecurityA(lpfilename : windows_sys::core::PCSTR, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetFileShortNameA(hfile : super::HANDLE, lpshortname : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetFileShortNameW(hfile : super::HANDLE, lpshortname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableA(lpname : windows_sys::core::PCSTR, lpguid : windows_sys::core::PCSTR, pvalue : *const core::ffi::c_void, nsize : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableExA(lpname : windows_sys::core::PCSTR, lpguid : windows_sys::core::PCSTR, pvalue : *const core::ffi::c_void, nsize : u32, dwattributes : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableExW(lpname : windows_sys::core::PCWSTR, lpguid : windows_sys::core::PCWSTR, pvalue : *const core::ffi::c_void, nsize : u32, dwattributes : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableW(lpname : windows_sys::core::PCWSTR, lpguid : windows_sys::core::PCWSTR, pvalue : *const core::ffi::c_void, nsize : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetHandleCount(unumber : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetMailslotInfo(hmailslot : super::HANDLE, lreadtimeout : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetMessageWaitingIndicator(hmsgindicator : super::HANDLE, ulmsgcount : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetProcessAffinityMask(hprocess : super::HANDLE, dwprocessaffinitymask : usize) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetProcessDEPPolicy(dwflags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetSearchPathMode(flags : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetSystemPowerState(fsuspend : windows_sys::core::BOOL, fforce : windows_sys::core::BOOL) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetTapeParameters(hdevice : super::HANDLE, dwoperation : u32, lptapeinformation : *const core::ffi::c_void) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetTapePosition(hdevice : super::HANDLE, dwpositionmethod : u32, dwpartition : u32, dwoffsetlow : u32, dwoffsethigh : u32, bimmediate : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadAffinityMask(hthread : super::HANDLE, dwthreadaffinitymask : usize) -> usize);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetThreadExecutionState(esflags : super::EXECUTION_STATE) -> super::EXECUTION_STATE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetTimerQueueTimer(timerqueue : super::HANDLE, callback : super::WAITORTIMERCALLBACK, parameter : *const core::ffi::c_void, duetime : u32, period : u32, preferio : windows_sys::core::BOOL) -> super::HANDLE);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetUmsThreadInformation(umsthread : PUMS_CONTEXT, umsthreadinfoclass : UMS_THREAD_INFO_CLASS, umsthreadinformation : *const core::ffi::c_void, umsthreadinformationlength : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetVolumeLabelA(lprootpathname : windows_sys::core::PCSTR, lpvolumename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetVolumeLabelW(lprootpathname : windows_sys::core::PCWSTR, lpvolumename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetVolumeMountPointA(lpszvolumemountpoint : windows_sys::core::PCSTR, lpszvolumename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SetVolumeMountPointW(lpszvolumemountpoint : windows_sys::core::PCWSTR, lpszvolumename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetXStateFeaturesMask(context : *mut super::CONTEXT, featuremask : u64) -> windows_sys::core::BOOL);
#[cfg(target_arch = "aarch64")]
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetXStateFeaturesMask(context : *mut super::ARM64_NT_CONTEXT, featuremask : u64) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn SetupComm(hfile : super::HANDLE, dwinqueue : u32, dwoutqueue : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn SwitchToFiber(lpfiber : *const core::ffi::c_void));
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn TransmitCommChar(hfile : super::HANDLE, cchar : i8) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn UmsThreadYield(schedulerparam : *const core::ffi::c_void) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn UnregisterApplicationRecoveryCallback() -> windows_sys::core::HRESULT);
windows_link::link!("kernel32.dll" "system" fn UnregisterApplicationRestart() -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn UnregisterWait(waithandle : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn UpdateResourceA(hupdate : super::HANDLE, lptype : windows_sys::core::PCSTR, lpname : windows_sys::core::PCSTR, wlanguage : u16, lpdata : *const core::ffi::c_void, cb : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn UpdateResourceW(hupdate : super::HANDLE, lptype : windows_sys::core::PCWSTR, lpname : windows_sys::core::PCWSTR, wlanguage : u16, lpdata : *const core::ffi::c_void, cb : u32) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn VerifyVersionInfoA(lpversioninformation : *mut super::OSVERSIONINFOEXA, dwtypemask : u32, dwlconditionmask : super::DWORDLONG) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn VerifyVersionInfoW(lpversioninformation : *mut super::OSVERSIONINFOEXW, dwtypemask : u32, dwlconditionmask : super::DWORDLONG) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WTSGetActiveConsoleSessionId() -> u32);
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
windows_link::link!("kernel32.dll" "system" fn WaitCommEvent(hfile : super::HANDLE, lpevtmask : *mut u32, lpoverlapped : *mut super::OVERLAPPED) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WaitNamedPipeA(lpnamedpipename : windows_sys::core::PCSTR, ntimeout : u32) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WinExec(lpcmdline : windows_sys::core::PCSTR, ucmdshow : u32) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn Wow64GetThreadSelectorEntry(hthread : super::HANDLE, dwselector : u32, lpselectorentry : *mut super::WOW64_LDT_ENTRY) -> windows_sys::core::BOOL);
windows_link::link!("advapi32.dll" "system" fn WriteEncryptedFileRaw(pfimportcallback : PFE_IMPORT_FUNC, pvcallbackcontext : *const core::ffi::c_void, pvcontext : *const core::ffi::c_void) -> u32);
windows_link::link!("kernel32.dll" "system" fn WritePrivateProfileSectionA(lpappname : windows_sys::core::PCSTR, lpstring : windows_sys::core::PCSTR, lpfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WritePrivateProfileSectionW(lpappname : windows_sys::core::PCWSTR, lpstring : windows_sys::core::PCWSTR, lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WritePrivateProfileStringA(lpappname : windows_sys::core::PCSTR, lpkeyname : windows_sys::core::PCSTR, lpstring : windows_sys::core::PCSTR, lpfilename : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WritePrivateProfileStringW(lpappname : windows_sys::core::PCWSTR, lpkeyname : windows_sys::core::PCWSTR, lpstring : windows_sys::core::PCWSTR, lpfilename : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WritePrivateProfileStructA(lpszsection : windows_sys::core::PCSTR, lpszkey : windows_sys::core::PCSTR, lpstruct : *const core::ffi::c_void, usizestruct : u32, szfile : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WritePrivateProfileStructW(lpszsection : windows_sys::core::PCWSTR, lpszkey : windows_sys::core::PCWSTR, lpstruct : *const core::ffi::c_void, usizestruct : u32, szfile : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WriteProfileSectionA(lpappname : windows_sys::core::PCSTR, lpstring : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WriteProfileSectionW(lpappname : windows_sys::core::PCWSTR, lpstring : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WriteProfileStringA(lpappname : windows_sys::core::PCSTR, lpkeyname : windows_sys::core::PCSTR, lpstring : windows_sys::core::PCSTR) -> windows_sys::core::BOOL);
windows_link::link!("kernel32.dll" "system" fn WriteProfileStringW(lpappname : windows_sys::core::PCWSTR, lpkeyname : windows_sys::core::PCWSTR, lpstring : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn WriteTapemark(hdevice : super::HANDLE, dwtapemarktype : u32, dwtapemarkcount : u32, bimmediate : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("kernel32.dll" "system" fn ZombifyActCtx(hactctx : super::HANDLE) -> windows_sys::core::BOOL);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn _hread(hfile : super::HFILE, lpbuffer : *mut core::ffi::c_void, lbytes : i32) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn _hwrite(hfile : super::HFILE, lpbuffer : *const i8, lbytes : i32) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn _lclose(hfile : super::HFILE) -> super::HFILE);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn _lcreat(lppathname : windows_sys::core::PCSTR, iattribute : i32) -> super::HFILE);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn _llseek(hfile : super::HFILE, loffset : i32, iorigin : i32) -> i32);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn _lopen(lppathname : windows_sys::core::PCSTR, ireadwrite : i32) -> super::HFILE);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn _lread(hfile : super::HFILE, lpbuffer : *mut core::ffi::c_void, ubytes : u32) -> u32);
#[cfg(feature = "minwindef")]
windows_link::link!("kernel32.dll" "system" fn _lwrite(hfile : super::HFILE, lpbuffer : *const i8, ubytes : u32) -> u32);
windows_link::link!("kernel32.dll" "system" fn lstrcatA(lpstring1 : windows_sys::core::PSTR, lpstring2 : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("kernel32.dll" "system" fn lstrcatW(lpstring1 : windows_sys::core::PWSTR, lpstring2 : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
windows_link::link!("kernel32.dll" "system" fn lstrcmpA(lpstring1 : windows_sys::core::PCSTR, lpstring2 : windows_sys::core::PCSTR) -> i32);
windows_link::link!("kernel32.dll" "system" fn lstrcmpW(lpstring1 : windows_sys::core::PCWSTR, lpstring2 : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("kernel32.dll" "system" fn lstrcmpiA(lpstring1 : windows_sys::core::PCSTR, lpstring2 : windows_sys::core::PCSTR) -> i32);
windows_link::link!("kernel32.dll" "system" fn lstrcmpiW(lpstring1 : windows_sys::core::PCWSTR, lpstring2 : windows_sys::core::PCWSTR) -> i32);
windows_link::link!("kernel32.dll" "system" fn lstrcpyA(lpstring1 : windows_sys::core::PSTR, lpstring2 : windows_sys::core::PCSTR) -> windows_sys::core::PSTR);
windows_link::link!("kernel32.dll" "system" fn lstrcpyW(lpstring1 : windows_sys::core::PWSTR, lpstring2 : windows_sys::core::PCWSTR) -> windows_sys::core::PWSTR);
windows_link::link!("kernel32.dll" "system" fn lstrcpynA(lpstring1 : windows_sys::core::PSTR, lpstring2 : windows_sys::core::PCSTR, imaxlength : i32) -> windows_sys::core::PSTR);
windows_link::link!("kernel32.dll" "system" fn lstrcpynW(lpstring1 : windows_sys::core::PWSTR, lpstring2 : windows_sys::core::PCWSTR, imaxlength : i32) -> windows_sys::core::PWSTR);
windows_link::link!("kernel32.dll" "system" fn lstrlenA(lpstring : windows_sys::core::PCSTR) -> i32);
windows_link::link!("kernel32.dll" "system" fn lstrlenW(lpstring : windows_sys::core::PCWSTR) -> i32);
pub const ABOVE_NORMAL_PRIORITY_CLASS: u32 = 32768;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type ACTCTX = ACTCTXA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ACTCTXA {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: windows_sys::core::PCSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: super::LANGID,
    pub lpAssemblyDirectory: windows_sys::core::PCSTR,
    pub lpResourceName: windows_sys::core::PCSTR,
    pub lpApplicationName: windows_sys::core::PCSTR,
    pub hModule: super::HMODULE,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for ACTCTXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct ACTCTXW {
    pub cbSize: u32,
    pub dwFlags: u32,
    pub lpSource: windows_sys::core::PCWSTR,
    pub wProcessorArchitecture: u16,
    pub wLangId: super::LANGID,
    pub lpAssemblyDirectory: windows_sys::core::PCWSTR,
    pub lpResourceName: windows_sys::core::PCWSTR,
    pub lpApplicationName: windows_sys::core::PCWSTR,
    pub hModule: super::HMODULE,
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
impl Default for ACTCTXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACTCTX_FLAG_APPLICATION_NAME_VALID: u32 = 32;
pub const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: u32 = 4;
pub const ACTCTX_FLAG_HMODULE_VALID: u32 = 128;
pub const ACTCTX_FLAG_LANGID_VALID: u32 = 2;
pub const ACTCTX_FLAG_PROCESSOR_ARCHITECTURE_VALID: u32 = 1;
pub const ACTCTX_FLAG_RESOURCE_NAME_VALID: u32 = 8;
pub const ACTCTX_FLAG_SET_PROCESS_DEFAULT: u32 = 16;
pub const ACTCTX_FLAG_SOURCE_IS_ASSEMBLYREF: u32 = 64;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct ACTCTX_SECTION_KEYED_DATA {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut core::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut core::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::HANDLE,
    pub ulAssemblyRosterIndex: u32,
    pub ulFlags: u32,
    pub AssemblyMetadata: ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA,
}
#[cfg(feature = "winnt")]
impl Default for ACTCTX_SECTION_KEYED_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct ACTCTX_SECTION_KEYED_DATA_2600 {
    pub cbSize: u32,
    pub ulDataFormatVersion: u32,
    pub lpData: *mut core::ffi::c_void,
    pub ulLength: u32,
    pub lpSectionGlobalData: *mut core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
    pub lpSectionBase: *mut core::ffi::c_void,
    pub ulSectionTotalLength: u32,
    pub hActCtx: super::HANDLE,
    pub ulAssemblyRosterIndex: u32,
}
#[cfg(feature = "winnt")]
impl Default for ACTCTX_SECTION_KEYED_DATA_2600 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    pub lpInformation: *mut core::ffi::c_void,
    pub lpSectionBase: *mut core::ffi::c_void,
    pub ulSectionLength: u32,
    pub lpSectionGlobalDataBase: *mut core::ffi::c_void,
    pub ulSectionGlobalDataLength: u32,
}
impl Default for ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct ACTIVATION_CONTEXT_BASIC_INFORMATION {
    pub hActCtx: super::HANDLE,
    pub dwFlags: u32,
}
#[cfg(feature = "winnt")]
impl Default for ACTIVATION_CONTEXT_BASIC_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ACTIVATION_CONTEXT_BASIC_INFORMATION_DEFINED: u32 = 1;
pub const AC_LINE_BACKUP_POWER: u32 = 2;
pub const AC_LINE_OFFLINE: u32 = 0;
pub const AC_LINE_ONLINE: u32 = 1;
pub const AC_LINE_UNKNOWN: u32 = 255;
pub type APPLICATION_RECOVERY_CALLBACK = Option<unsafe extern "system" fn(pvparameter: *mut core::ffi::c_void) -> u32>;
pub const ATOM_FLAG_GLOBAL: u32 = 2;
pub const BACKUP_ALTERNATE_DATA: u32 = 4;
pub const BACKUP_DATA: u32 = 1;
pub const BACKUP_EA_DATA: u32 = 2;
pub const BACKUP_GHOSTED_FILE_EXTENTS: u32 = 11;
pub const BACKUP_INVALID: u32 = 0;
pub const BACKUP_LINK: u32 = 5;
pub const BACKUP_OBJECT_ID: u32 = 7;
pub const BACKUP_PROPERTY_DATA: u32 = 6;
pub const BACKUP_REPARSE_DATA: u32 = 8;
pub const BACKUP_SECURITY_DATA: u32 = 3;
pub const BACKUP_SPARSE_BLOCK: u32 = 9;
pub const BACKUP_TXFS_DATA: u32 = 10;
pub const BASE_SEARCH_PATH_DISABLE_SAFE_SEARCHMODE: u32 = 65536;
pub const BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE: u32 = 1;
pub const BASE_SEARCH_PATH_INVALID_FLAGS: i32 = -98306;
pub const BASE_SEARCH_PATH_PERMANENT: u32 = 32768;
pub const BATTERY_FLAG_CHARGING: u32 = 8;
pub const BATTERY_FLAG_CRITICAL: u32 = 4;
pub const BATTERY_FLAG_HIGH: u32 = 1;
pub const BATTERY_FLAG_LOW: u32 = 2;
pub const BATTERY_FLAG_NO_BATTERY: u32 = 128;
pub const BATTERY_FLAG_UNKNOWN: u32 = 255;
pub const BATTERY_LIFE_UNKNOWN: u32 = 4294967295;
pub const BATTERY_PERCENTAGE_UNKNOWN: u32 = 255;
pub const BAUD_075: u32 = 1;
pub const BAUD_110: u32 = 2;
pub const BAUD_115200: u32 = 131072;
pub const BAUD_1200: u32 = 64;
pub const BAUD_128K: u32 = 65536;
pub const BAUD_134_5: u32 = 4;
pub const BAUD_14400: u32 = 4096;
pub const BAUD_150: u32 = 8;
pub const BAUD_1800: u32 = 128;
pub const BAUD_19200: u32 = 8192;
pub const BAUD_2400: u32 = 256;
pub const BAUD_300: u32 = 16;
pub const BAUD_38400: u32 = 16384;
pub const BAUD_4800: u32 = 512;
pub const BAUD_56K: u32 = 32768;
pub const BAUD_57600: u32 = 262144;
pub const BAUD_600: u32 = 32;
pub const BAUD_7200: u32 = 1024;
pub const BAUD_9600: u32 = 2048;
pub const BAUD_USER: u32 = 268435456;
pub const BELOW_NORMAL_PRIORITY_CLASS: u32 = 16384;
pub const CALLBACK_CHUNK_FINISHED: u32 = 0;
pub const CALLBACK_STREAM_SWITCH: u32 = 1;
pub const CBR_110: u32 = 110;
pub const CBR_115200: u32 = 115200;
pub const CBR_1200: u32 = 1200;
pub const CBR_128000: u32 = 128000;
pub const CBR_14400: u32 = 14400;
pub const CBR_19200: u32 = 19200;
pub const CBR_2400: u32 = 2400;
pub const CBR_256000: u32 = 256000;
pub const CBR_300: u32 = 300;
pub const CBR_38400: u32 = 38400;
pub const CBR_4800: u32 = 4800;
pub const CBR_56000: u32 = 56000;
pub const CBR_57600: u32 = 57600;
pub const CBR_600: u32 = 600;
pub const CBR_9600: u32 = 9600;
pub const CE_BREAK: u32 = 16;
pub const CE_DNS: u32 = 2048;
pub const CE_FRAME: u32 = 8;
pub const CE_IOE: u32 = 1024;
pub const CE_MODE: u32 = 32768;
pub const CE_OOP: u32 = 4096;
pub const CE_OVERRUN: u32 = 2;
pub const CE_PTO: u32 = 512;
pub const CE_RXOVER: u32 = 1;
pub const CE_RXPARITY: u32 = 4;
pub const CE_TXFULL: u32 = 256;
pub const CLRBREAK: u32 = 9;
pub const CLRDTR: u32 = 6;
pub const CLRRTS: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COMMCONFIG {
    pub dwSize: u32,
    pub wVersion: u16,
    pub wReserved: u16,
    pub dcb: DCB,
    pub dwProviderSubType: u32,
    pub dwProviderOffset: u32,
    pub dwProviderSize: u32,
    pub wcProviderData: [u16; 1],
}
impl Default for COMMCONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct COMMPROP {
    pub wPacketLength: u16,
    pub wPacketVersion: u16,
    pub dwServiceMask: u32,
    pub dwReserved1: u32,
    pub dwMaxTxQueue: u32,
    pub dwMaxRxQueue: u32,
    pub dwMaxBaud: u32,
    pub dwProvSubType: u32,
    pub dwProvCapabilities: u32,
    pub dwSettableParams: u32,
    pub dwSettableBaud: u32,
    pub wSettableData: u16,
    pub wSettableStopParity: u16,
    pub dwCurrentTxQueue: u32,
    pub dwCurrentRxQueue: u32,
    pub dwProvSpec1: u32,
    pub dwProvSpec2: u32,
    pub wcProvChar: [u16; 1],
}
impl Default for COMMPROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COMMPROP_INITIALIZED: u32 = 3879531822;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct COMMTIMEOUTS {
    pub ReadIntervalTimeout: u32,
    pub ReadTotalTimeoutMultiplier: u32,
    pub ReadTotalTimeoutConstant: u32,
    pub WriteTotalTimeoutMultiplier: u32,
    pub WriteTotalTimeoutConstant: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct COMSTAT {
    pub _bitfield: u32,
    pub cbInQue: u32,
    pub cbOutQue: u32,
}
pub const COPYFILE2_CALLBACK_CHUNK_FINISHED: COPYFILE2_MESSAGE_TYPE = 2;
pub const COPYFILE2_CALLBACK_CHUNK_STARTED: COPYFILE2_MESSAGE_TYPE = 1;
pub const COPYFILE2_CALLBACK_ERROR: COPYFILE2_MESSAGE_TYPE = 6;
pub const COPYFILE2_CALLBACK_MAX: COPYFILE2_MESSAGE_TYPE = 9;
pub const COPYFILE2_CALLBACK_NONE: COPYFILE2_MESSAGE_TYPE = 0;
pub const COPYFILE2_CALLBACK_POLL_CONTINUE: COPYFILE2_MESSAGE_TYPE = 5;
pub const COPYFILE2_CALLBACK_SPARSE_CHUNK_FINISHED: COPYFILE2_MESSAGE_TYPE = 8;
pub const COPYFILE2_CALLBACK_SPARSE_CHUNK_STARTED: COPYFILE2_MESSAGE_TYPE = 7;
pub const COPYFILE2_CALLBACK_STREAM_FINISHED: COPYFILE2_MESSAGE_TYPE = 4;
pub const COPYFILE2_CALLBACK_STREAM_STARTED: COPYFILE2_MESSAGE_TYPE = 3;
pub type COPYFILE2_COPY_PHASE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct COPYFILE2_CREATE_OPLOCK_KEYS {
    pub ParentOplockKey: windows_sys::core::GUID,
    pub TargetOplockKey: windows_sys::core::GUID,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct COPYFILE2_EXTENDED_PARAMETERS {
    pub dwSize: u32,
    pub dwCopyFlags: u32,
    pub pfCancel: *mut windows_sys::core::BOOL,
    pub pProgressRoutine: PCOPYFILE2_PROGRESS_ROUTINE,
    pub pvCallbackContext: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for COPYFILE2_EXTENDED_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct COPYFILE2_EXTENDED_PARAMETERS_V2 {
    pub dwSize: u32,
    pub dwCopyFlags: u32,
    pub pfCancel: *mut windows_sys::core::BOOL,
    pub pProgressRoutine: PCOPYFILE2_PROGRESS_ROUTINE,
    pub pvCallbackContext: *mut core::ffi::c_void,
    pub dwCopyFlagsV2: u32,
    pub ioDesiredSize: u32,
    pub ioDesiredRate: u32,
    pub pProgressRoutineOld: LPPROGRESS_ROUTINE,
    pub SourceOplockKeys: PCOPYFILE2_CREATE_OPLOCK_KEYS,
    pub reserved: [*mut core::ffi::c_void; 6],
}
#[cfg(feature = "winnt")]
impl Default for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COPYFILE2_IO_CYCLE_SIZE_MAX: u32 = 1073741824;
pub const COPYFILE2_IO_CYCLE_SIZE_MIN: u32 = 4096;
pub const COPYFILE2_IO_RATE_MIN: u32 = 512;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct COPYFILE2_MESSAGE {
    pub Type: COPYFILE2_MESSAGE_TYPE,
    pub dwPadding: u32,
    pub Info: COPYFILE2_MESSAGE_0,
}
#[cfg(feature = "winnt")]
impl Default for COPYFILE2_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union COPYFILE2_MESSAGE_0 {
    pub ChunkStarted: COPYFILE2_MESSAGE_0_0,
    pub ChunkFinished: COPYFILE2_MESSAGE_0_1,
    pub StreamStarted: COPYFILE2_MESSAGE_0_2,
    pub StreamFinished: COPYFILE2_MESSAGE_0_3,
    pub PollContinue: COPYFILE2_MESSAGE_0_4,
    pub Error: COPYFILE2_MESSAGE_0_5,
    pub SparseChunkStatus: COPYFILE2_MESSAGE_0_6,
}
#[cfg(feature = "winnt")]
impl Default for COPYFILE2_MESSAGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct COPYFILE2_MESSAGE_0_0 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::HANDLE,
    pub hDestinationFile: super::HANDLE,
    pub uliChunkNumber: u64,
    pub uliChunkSize: u64,
    pub uliStreamSize: u64,
    pub uliTotalFileSize: u64,
}
#[cfg(feature = "winnt")]
impl Default for COPYFILE2_MESSAGE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct COPYFILE2_MESSAGE_0_1 {
    pub dwStreamNumber: u32,
    pub dwFlags: u32,
    pub hSourceFile: super::HANDLE,
    pub hDestinationFile: super::HANDLE,
    pub uliChunkNumber: u64,
    pub uliChunkSize: u64,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
#[cfg(feature = "winnt")]
impl Default for COPYFILE2_MESSAGE_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct COPYFILE2_MESSAGE_0_2 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::HANDLE,
    pub hDestinationFile: super::HANDLE,
    pub uliStreamSize: u64,
    pub uliTotalFileSize: u64,
}
#[cfg(feature = "winnt")]
impl Default for COPYFILE2_MESSAGE_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct COPYFILE2_MESSAGE_0_3 {
    pub dwStreamNumber: u32,
    pub dwReserved: u32,
    pub hSourceFile: super::HANDLE,
    pub hDestinationFile: super::HANDLE,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
#[cfg(feature = "winnt")]
impl Default for COPYFILE2_MESSAGE_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct COPYFILE2_MESSAGE_0_4 {
    pub dwReserved: u32,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct COPYFILE2_MESSAGE_0_5 {
    pub CopyPhase: COPYFILE2_COPY_PHASE,
    pub dwStreamNumber: u32,
    pub hrFailure: windows_sys::core::HRESULT,
    pub dwReserved: u32,
    pub uliChunkNumber: u64,
    pub uliStreamSize: u64,
    pub uliStreamBytesTransferred: u64,
    pub uliTotalFileSize: u64,
    pub uliTotalBytesTransferred: u64,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct COPYFILE2_MESSAGE_0_6 {
    pub uliChunkNumber: u64,
    pub uliChunkSize: u64,
    pub uliChunkOffset: u64,
}
pub type COPYFILE2_MESSAGE_ACTION = i32;
pub const COPYFILE2_MESSAGE_COPY_OFFLOAD: u32 = 1;
pub type COPYFILE2_MESSAGE_TYPE = i32;
pub const COPYFILE2_PHASE_MAX: COPYFILE2_COPY_PHASE = 7;
pub const COPYFILE2_PHASE_NAMEGRAFT_COPY: COPYFILE2_COPY_PHASE = 6;
pub const COPYFILE2_PHASE_NONE: COPYFILE2_COPY_PHASE = 0;
pub const COPYFILE2_PHASE_PREPARE_DEST: COPYFILE2_COPY_PHASE = 2;
pub const COPYFILE2_PHASE_PREPARE_SOURCE: COPYFILE2_COPY_PHASE = 1;
pub const COPYFILE2_PHASE_READ_SOURCE: COPYFILE2_COPY_PHASE = 3;
pub const COPYFILE2_PHASE_SERVER_COPY: COPYFILE2_COPY_PHASE = 5;
pub const COPYFILE2_PHASE_WRITE_DESTINATION: COPYFILE2_COPY_PHASE = 4;
pub const COPYFILE2_PROGRESS_CANCEL: COPYFILE2_MESSAGE_ACTION = 1;
pub const COPYFILE2_PROGRESS_CONTINUE: COPYFILE2_MESSAGE_ACTION = 0;
pub const COPYFILE2_PROGRESS_PAUSE: COPYFILE2_MESSAGE_ACTION = 4;
pub const COPYFILE2_PROGRESS_QUIET: COPYFILE2_MESSAGE_ACTION = 3;
pub const COPYFILE2_PROGRESS_STOP: COPYFILE2_MESSAGE_ACTION = 2;
pub const COPY_FILE2_V2_DISABLE_BLOCK_CLONING: u32 = 2;
pub const COPY_FILE2_V2_DONT_COPY_JUNCTIONS: u32 = 1;
pub const COPY_FILE2_V2_VALID_FLAGS: u32 = 3;
pub const COPY_FILE_ALLOW_DECRYPTED_DESTINATION: u32 = 8;
pub const COPY_FILE_COPY_SYMLINK: u32 = 2048;
pub const COPY_FILE_DIRECTORY: u32 = 128;
pub const COPY_FILE_DISABLE_PRE_ALLOCATION: u32 = 67108864;
pub const COPY_FILE_DISABLE_SPARSE_COPY: u32 = 2147483648;
pub const COPY_FILE_DONT_REQUEST_DEST_WRITE_DAC: u32 = 33554432;
pub const COPY_FILE_ENABLE_LOW_FREE_SPACE_MODE: u32 = 134217728;
pub const COPY_FILE_ENABLE_SPARSE_COPY: u32 = 536870912;
pub const COPY_FILE_FAIL_IF_EXISTS: u32 = 1;
pub const COPY_FILE_IGNORE_EDP_BLOCK: u32 = 4194304;
pub const COPY_FILE_IGNORE_SOURCE_ENCRYPTION: u32 = 8388608;
pub const COPY_FILE_NO_BUFFERING: u32 = 4096;
pub const COPY_FILE_NO_OFFLOAD: u32 = 262144;
pub const COPY_FILE_OPEN_AND_COPY_REPARSE_POINT: u32 = 2097152;
pub const COPY_FILE_OPEN_SOURCE_FOR_WRITE: u32 = 4;
pub const COPY_FILE_REQUEST_COMPRESSED_TRAFFIC: u32 = 268435456;
pub const COPY_FILE_REQUEST_SECURITY_PRIVILEGES: u32 = 8192;
pub const COPY_FILE_RESTARTABLE: u32 = 2;
pub const COPY_FILE_RESUME_FROM_PAUSE: u32 = 16384;
pub const COPY_FILE_SKIP_ALTERNATE_STREAMS: u32 = 32768;
pub const CREATE_BREAKAWAY_FROM_JOB: u32 = 16777216;
pub const CREATE_DEFAULT_ERROR_MODE: u32 = 67108864;
pub const CREATE_FORCEDOS: u32 = 8192;
pub const CREATE_FOR_DIR: u32 = 2;
pub const CREATE_FOR_IMPORT: u32 = 1;
pub const CREATE_IGNORE_SYSTEM_DEFAULT: u32 = 2147483648;
pub const CREATE_NEW_CONSOLE: u32 = 16;
pub const CREATE_NEW_PROCESS_GROUP: u32 = 512;
pub const CREATE_NO_WINDOW: u32 = 134217728;
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: u32 = 33554432;
pub const CREATE_PROTECTED_PROCESS: u32 = 262144;
pub const CREATE_SECURE_PROCESS: u32 = 4194304;
pub const CREATE_SEPARATE_WOW_VDM: u32 = 2048;
pub const CREATE_SHARED_WOW_VDM: u32 = 4096;
pub const CREATE_SUSPENDED: u32 = 4;
pub const CREATE_UNICODE_ENVIRONMENT: u32 = 1024;
pub const CRITICAL_SECTION_NO_DEBUG_INFO: u32 = 16777216;
pub const DATABITS_16: u16 = 16;
pub const DATABITS_16X: u16 = 32;
pub const DATABITS_5: u16 = 1;
pub const DATABITS_6: u16 = 2;
pub const DATABITS_7: u16 = 4;
pub const DATABITS_8: u16 = 8;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DCB {
    pub DCBlength: u32,
    pub BaudRate: u32,
    pub _bitfield: u32,
    pub wReserved: u16,
    pub XonLim: u16,
    pub XoffLim: u16,
    pub ByteSize: u8,
    pub Parity: u8,
    pub StopBits: u8,
    pub XonChar: i8,
    pub XoffChar: i8,
    pub ErrorChar: i8,
    pub EofChar: i8,
    pub EvtChar: i8,
    pub wReserved1: u16,
}
pub const DDD_EXACT_MATCH_ON_REMOVE: u32 = 4;
pub const DDD_LUID_BROADCAST_DRIVE: u32 = 16;
pub const DDD_NO_BROADCAST_SYSTEM: u32 = 8;
pub const DDD_RAW_TARGET_PATH: u32 = 1;
pub const DDD_REMOVE_DEFINITION: u32 = 2;
pub const DEACTIVATE_ACTCTX_FLAG_FORCE_EARLY_DEACTIVATION: u32 = 1;
pub const DEBUG_ONLY_THIS_PROCESS: u32 = 2;
pub const DEBUG_PROCESS: u32 = 1;
pub const DEPPolicyAlwaysOff: DEP_SYSTEM_POLICY_TYPE = 0;
pub const DEPPolicyAlwaysOn: DEP_SYSTEM_POLICY_TYPE = 1;
pub const DEPPolicyOptIn: DEP_SYSTEM_POLICY_TYPE = 2;
pub const DEPPolicyOptOut: DEP_SYSTEM_POLICY_TYPE = 3;
pub const DEPTotalPolicyCount: DEP_SYSTEM_POLICY_TYPE = 4;
pub type DEP_SYSTEM_POLICY_TYPE = i32;
pub const DETACHED_PROCESS: u32 = 8;
pub const DOCKINFO_DOCKED: u32 = 2;
pub const DOCKINFO_UNDOCKED: u32 = 1;
pub const DOCKINFO_USER_DOCKED: u32 = 6;
pub const DOCKINFO_USER_SUPPLIED: u32 = 4;
pub const DOCKINFO_USER_UNDOCKED: u32 = 5;
pub const DRIVE_CDROM: u32 = 5;
pub const DRIVE_FIXED: u32 = 3;
pub const DRIVE_NO_ROOT_DIR: u32 = 1;
pub const DRIVE_RAMDISK: u32 = 6;
pub const DRIVE_REMOTE: u32 = 4;
pub const DRIVE_REMOVABLE: u32 = 2;
pub const DRIVE_UNKNOWN: u32 = 0;
pub const DTR_CONTROL_DISABLE: u32 = 0;
pub const DTR_CONTROL_ENABLE: u32 = 1;
pub const DTR_CONTROL_HANDSHAKE: u32 = 2;
pub const EFSRPC_SECURE_ONLY: u32 = 8;
pub const EFS_DROP_ALTERNATE_STREAMS: u32 = 16;
pub const EFS_USE_RECOVERY_KEYS: u32 = 1;
pub const EVENPARITY: u32 = 2;
pub const EVENTLOG_FULL_INFO: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct EVENTLOG_FULL_INFORMATION {
    pub dwFull: u32,
}
pub const EV_BREAK: u32 = 64;
pub const EV_CTS: u32 = 8;
pub const EV_DSR: u32 = 16;
pub const EV_ERR: u32 = 128;
pub const EV_EVENT1: u32 = 2048;
pub const EV_EVENT2: u32 = 4096;
pub const EV_PERR: u32 = 512;
pub const EV_RING: u32 = 256;
pub const EV_RLSD: u32 = 32;
pub const EV_RX80FULL: u32 = 1024;
pub const EV_RXCHAR: u32 = 1;
pub const EV_RXFLAG: u32 = 2;
pub const EV_TXEMPTY: u32 = 4;
pub const EXTENDED_STARTUPINFO_PRESENT: u32 = 524288;
pub const ExtendedFileIdType: FILE_ID_TYPE = 2;
pub const FAIL_FAST_GENERATE_EXCEPTION_ADDRESS: u32 = 1;
pub const FAIL_FAST_NO_HARD_ERROR_DLG: u32 = 2;
pub const FIBER_FLAG_FLOAT_SWITCH: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_ALIGNMENT_INFO {
    pub AlignmentRequirement: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_ALLOCATION_INFO {
    pub AllocationSize: i64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_ATTRIBUTE_TAG_INFO {
    pub FileAttributes: u32,
    pub ReparseTag: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_BASIC_INFO {
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub FileAttributes: u32,
}
pub const FILE_BEGIN: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_CASE_SENSITIVE_INFO {
    pub Flags: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_COMPRESSION_INFO {
    pub CompressedFileSize: i64,
    pub CompressionFormat: u16,
    pub CompressionUnitShift: u8,
    pub ChunkShift: u8,
    pub ClusterShift: u8,
    pub Reserved: [u8; 3],
}
impl Default for FILE_COMPRESSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_CURRENT: u32 = 1;
pub const FILE_DIR_DISALLOWED: u32 = 9;
pub const FILE_DISPOSITION_FLAG_DELETE: u32 = 1;
pub const FILE_DISPOSITION_FLAG_DO_NOT_DELETE: u32 = 0;
pub const FILE_DISPOSITION_FLAG_FORCE_IMAGE_SECTION_CHECK: u32 = 4;
pub const FILE_DISPOSITION_FLAG_IGNORE_READONLY_ATTRIBUTE: u32 = 16;
pub const FILE_DISPOSITION_FLAG_ON_CLOSE: u32 = 8;
pub const FILE_DISPOSITION_FLAG_POSIX_SEMANTICS: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_DISPOSITION_INFO {
    pub DeleteFileA: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_DISPOSITION_INFO_EX {
    pub Flags: u32,
}
pub const FILE_ENCRYPTABLE: u32 = 0;
pub const FILE_END: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_END_OF_FILE_INFO {
    pub EndOfFile: i64,
}
pub const FILE_FLAG_BACKUP_SEMANTICS: u32 = 33554432;
pub const FILE_FLAG_DELETE_ON_CLOSE: u32 = 67108864;
pub const FILE_FLAG_DISALLOW_PATH_REDIRECTS: u32 = 65536;
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: u32 = 524288;
pub const FILE_FLAG_IGNORE_IMPERSONATED_DEVICEMAP: u32 = 131072;
pub const FILE_FLAG_NO_BUFFERING: u32 = 536870912;
pub const FILE_FLAG_OPEN_NO_RECALL: u32 = 1048576;
pub const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 2097152;
pub const FILE_FLAG_OPEN_REQUIRING_OPLOCK: u32 = 262144;
pub const FILE_FLAG_OVERLAPPED: u32 = 1073741824;
pub const FILE_FLAG_POSIX_SEMANTICS: u32 = 16777216;
pub const FILE_FLAG_RANDOM_ACCESS: u32 = 268435456;
pub const FILE_FLAG_SEQUENTIAL_SCAN: u32 = 134217728;
pub const FILE_FLAG_SESSION_AWARE: u32 = 8388608;
pub const FILE_FLAG_WRITE_THROUGH: u32 = 2147483648;
pub const FILE_FLUSH_DATA: FILE_FLUSH_MODE = 1;
pub const FILE_FLUSH_DEFAULT: FILE_FLUSH_MODE = 0;
pub const FILE_FLUSH_MIN_METADATA: FILE_FLUSH_MODE = 2;
pub type FILE_FLUSH_MODE = i32;
pub const FILE_FLUSH_NO_SYNC: FILE_FLUSH_MODE = 3;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_FULL_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub FileName: [u16; 1],
}
impl Default for FILE_FULL_DIR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_ID_BOTH_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ShortNameLength: super::CCHAR,
    pub ShortName: [u16; 12],
    pub FileId: i64,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_ID_BOTH_DIR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_ID_DESCRIPTOR {
    pub dwSize: u32,
    pub Type: FILE_ID_TYPE,
    pub Anonymous: FILE_ID_DESCRIPTOR_0,
}
#[cfg(feature = "winnt")]
impl Default for FILE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union FILE_ID_DESCRIPTOR_0 {
    pub FileId: i64,
    pub ObjectId: windows_sys::core::GUID,
    pub ExtendedFileId: super::FILE_ID_128,
}
#[cfg(feature = "winnt")]
impl Default for FILE_ID_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_ID_EXTD_DIR_INFO {
    pub NextEntryOffset: u32,
    pub FileIndex: u32,
    pub CreationTime: i64,
    pub LastAccessTime: i64,
    pub LastWriteTime: i64,
    pub ChangeTime: i64,
    pub EndOfFile: i64,
    pub AllocationSize: i64,
    pub FileAttributes: u32,
    pub FileNameLength: u32,
    pub EaSize: u32,
    pub ReparsePointTag: u32,
    pub FileId: super::FILE_ID_128,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_ID_EXTD_DIR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct FILE_ID_INFO {
    pub VolumeSerialNumber: u64,
    pub FileId: super::FILE_ID_128,
}
pub type FILE_ID_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_IO_PRIORITY_HINT_INFO {
    pub PriorityHint: PRIORITY_HINT,
}
pub const FILE_IS_ENCRYPTED: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_NAME_INFO {
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
impl Default for FILE_NAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_NAME_NORMALIZED: u32 = 0;
pub const FILE_NAME_OPENED: u32 = 8;
pub const FILE_READ_ONLY: u32 = 8;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_REMOTE_PROTOCOL_INFO {
    pub StructureVersion: u16,
    pub StructureSize: u16,
    pub Protocol: u32,
    pub ProtocolMajorVersion: u16,
    pub ProtocolMinorVersion: u16,
    pub ProtocolRevision: u16,
    pub Reserved: u16,
    pub Flags: u32,
    pub GenericReserved: FILE_REMOTE_PROTOCOL_INFO_0,
    pub ProtocolSpecific: FILE_REMOTE_PROTOCOL_INFO_1,
}
impl Default for FILE_REMOTE_PROTOCOL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_REMOTE_PROTOCOL_INFO_0 {
    pub Reserved: [u32; 8],
}
impl Default for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union FILE_REMOTE_PROTOCOL_INFO_1 {
    pub Smb2: FILE_REMOTE_PROTOCOL_INFO_1_0,
    pub Reserved: [u32; 16],
}
impl Default for FILE_REMOTE_PROTOCOL_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0 {
    pub Server: FILE_REMOTE_PROTOCOL_INFO_1_0_0,
    pub Share: FILE_REMOTE_PROTOCOL_INFO_1_0_1,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    pub Capabilities: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    pub Capabilities: u32,
    pub ShareFlags: u32,
}
pub const FILE_RENAME_FLAG_POSIX_SEMANTICS: u32 = 2;
pub const FILE_RENAME_FLAG_REPLACE_IF_EXISTS: u32 = 1;
pub const FILE_RENAME_FLAG_SUPPRESS_PIN_STATE_INHERITANCE: u32 = 4;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct FILE_RENAME_INFO {
    pub Anonymous: FILE_RENAME_INFO_0,
    pub RootDirectory: super::HANDLE,
    pub FileNameLength: u32,
    pub FileName: [u16; 1],
}
#[cfg(feature = "winnt")]
impl Default for FILE_RENAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union FILE_RENAME_INFO_0 {
    pub ReplaceIfExists: bool,
    pub Flags: u32,
}
#[cfg(feature = "winnt")]
impl Default for FILE_RENAME_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_ROOT_DIR: u32 = 3;
pub const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: u32 = 1;
pub const FILE_SKIP_SET_EVENT_ON_HANDLE: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_STANDARD_INFO {
    pub AllocationSize: i64,
    pub EndOfFile: i64,
    pub NumberOfLinks: u32,
    pub DeletePending: bool,
    pub Directory: bool,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FILE_STORAGE_INFO {
    pub LogicalBytesPerSector: u32,
    pub PhysicalBytesPerSectorForAtomicity: u32,
    pub PhysicalBytesPerSectorForPerformance: u32,
    pub FileSystemEffectivePhysicalBytesPerSectorForAtomicity: u32,
    pub Flags: u32,
    pub ByteOffsetForSectorAlignment: u32,
    pub ByteOffsetForPartitionAlignment: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FILE_STREAM_INFO {
    pub NextEntryOffset: u32,
    pub StreamNameLength: u32,
    pub StreamSize: i64,
    pub StreamAllocationSize: i64,
    pub StreamName: [u16; 1],
}
impl Default for FILE_STREAM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_SYSTEM_ATTR: u32 = 2;
pub const FILE_SYSTEM_DIR: u32 = 4;
pub const FILE_SYSTEM_NOT_SUPPORT: u32 = 6;
pub const FILE_TYPE_CHAR: u32 = 2;
pub const FILE_TYPE_DISK: u32 = 1;
pub const FILE_TYPE_PIPE: u32 = 3;
pub const FILE_TYPE_REMOTE: u32 = 32768;
pub const FILE_TYPE_UNKNOWN: u32 = 0;
pub const FILE_UNKNOWN: u32 = 5;
pub const FILE_USER_DISALLOWED: u32 = 7;
pub type FILE_WRITE_FLAGS = u32;
pub const FILE_WRITE_FLAGS_NONE: FILE_WRITE_FLAGS = 0;
pub const FILE_WRITE_FLAGS_WRITE_THROUGH: FILE_WRITE_FLAGS = 1;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_ASSEMBLY_METADATA: u32 = 4;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_FLAGS: u32 = 2;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_HACTCTX: u32 = 1;
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: u32 = 256;
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: u32 = 8192;
pub const FORMAT_MESSAGE_FROM_HMODULE: u32 = 2048;
pub const FORMAT_MESSAGE_FROM_STRING: u32 = 1024;
pub const FORMAT_MESSAGE_FROM_SYSTEM: u32 = 4096;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: u32 = 512;
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: u32 = 255;
pub const FS_CASE_IS_PRESERVED: u32 = 2;
pub const FS_CASE_SENSITIVE: u32 = 1;
pub const FS_FILE_COMPRESSION: u32 = 16;
pub const FS_FILE_ENCRYPTION: u32 = 131072;
pub const FS_PERSISTENT_ACLS: u32 = 8;
pub const FS_UNICODE_STORED_ON_DISK: u32 = 4;
pub const FS_VOL_IS_COMPRESSED: u32 = 32768;
pub const FileIdType: FILE_ID_TYPE = 0;
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_A: windows_sys::core::PCSTR = windows_sys::core::s!("GetSystemWow64DirectoryA");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_W: windows_sys::core::PCWSTR = windows_sys::core::w!("GetSystemWow64DirectoryA");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_A: windows_sys::core::PCSTR = windows_sys::core::s!("GetSystemWow64DirectoryW");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_W: windows_sys::core::PCWSTR = windows_sys::core::w!("GetSystemWow64DirectoryW");
pub const GET_TAPE_DRIVE_INFORMATION: u32 = 1;
pub const GET_TAPE_MEDIA_INFORMATION: u32 = 0;
pub const GHND: u32 = 66;
pub const GMEM_DDESHARE: u32 = 8192;
pub const GMEM_DISCARDABLE: u32 = 256;
pub const GMEM_DISCARDED: u32 = 16384;
pub const GMEM_FIXED: u32 = 0;
pub const GMEM_INVALID_HANDLE: u32 = 32768;
pub const GMEM_LOCKCOUNT: u32 = 255;
pub const GMEM_LOWER: u32 = 4096;
pub const GMEM_MODIFY: u32 = 128;
pub const GMEM_MOVEABLE: u32 = 2;
pub const GMEM_NOCOMPACT: u32 = 16;
pub const GMEM_NODISCARD: u32 = 32;
pub const GMEM_NOTIFY: u32 = 16384;
pub const GMEM_NOT_BANKED: u32 = 4096;
pub const GMEM_SHARE: u32 = 8192;
pub const GMEM_VALID_FLAGS: u32 = 32626;
pub const GMEM_ZEROINIT: u32 = 64;
pub const GPTR: u32 = 64;
pub const HANDLE_FLAG_INHERIT: u32 = 1;
pub const HANDLE_FLAG_PROTECT_FROM_CLOSE: u32 = 2;
pub const HIGH_PRIORITY_CLASS: u32 = 128;
pub const HINSTANCE_ERROR: u32 = 32;
pub const HW_PROFILE_GUIDLEN: u32 = 39;
pub type HW_PROFILE_INFO = HW_PROFILE_INFOA;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HW_PROFILE_INFOA {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [i8; 39],
    pub szHwProfileName: [i8; 80],
}
impl Default for HW_PROFILE_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct HW_PROFILE_INFOW {
    pub dwDockInfo: u32,
    pub szHwProfileGuid: [u16; 39],
    pub szHwProfileName: [u16; 80],
}
impl Default for HW_PROFILE_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IDLE_PRIORITY_CLASS: u32 = 64;
pub const IE_BADID: i32 = -1;
pub const IE_BAUDRATE: i32 = -12;
pub const IE_BYTESIZE: i32 = -11;
pub const IE_DEFAULT: i32 = -5;
pub const IE_HARDWARE: i32 = -10;
pub const IE_MEMORY: i32 = -4;
pub const IE_NOPEN: i32 = -3;
pub const IE_OPEN: i32 = -2;
pub const IGNORE: u32 = 0;
pub const INFINITE: u32 = 4294967295;
pub const INHERIT_CALLER_PRIORITY: u32 = 131072;
pub const INHERIT_PARENT_AFFINITY: u32 = 65536;
#[cfg(feature = "minwindef")]
pub const INVALID_ATOM: super::ATOM = 0;
pub const IoPriorityHintLow: PRIORITY_HINT = 1;
pub const IoPriorityHintNormal: PRIORITY_HINT = 2;
pub const IoPriorityHintVeryLow: PRIORITY_HINT = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct JIT_DEBUG_INFO {
    pub dwSize: u32,
    pub dwProcessorArchitecture: u32,
    pub dwThreadID: u32,
    pub dwReserved0: u32,
    pub lpExceptionAddress: u64,
    pub lpExceptionRecord: u64,
    pub lpContextRecord: u64,
}
pub type JIT_DEBUG_INFO32 = JIT_DEBUG_INFO;
pub type JIT_DEBUG_INFO64 = JIT_DEBUG_INFO;
pub const LOGON32_LOGON_BATCH: u32 = 4;
pub const LOGON32_LOGON_INTERACTIVE: u32 = 2;
pub const LOGON32_LOGON_NETWORK: u32 = 3;
pub const LOGON32_LOGON_NETWORK_CLEARTEXT: u32 = 8;
pub const LOGON32_LOGON_NEW_CREDENTIALS: u32 = 9;
pub const LOGON32_LOGON_SERVICE: u32 = 5;
pub const LOGON32_LOGON_UNLOCK: u32 = 7;
pub const LOGON32_PROVIDER_DEFAULT: u32 = 0;
pub const LOGON32_PROVIDER_VIRTUAL: u32 = 4;
pub const LOGON32_PROVIDER_WINNT35: u32 = 1;
pub const LOGON32_PROVIDER_WINNT40: u32 = 2;
pub const LOGON32_PROVIDER_WINNT50: u32 = 3;
pub const LOGON_NETCREDENTIALS_ONLY: u32 = 2;
pub const LOGON_WITH_PROFILE: u32 = 1;
pub const LOGON_ZERO_PASSWORD_BUFFER: u32 = 2147483648;
pub type LPCOMMCONFIG = *mut COMMCONFIG;
pub type LPCOMMPROP = *mut COMMPROP;
pub type LPCOMMTIMEOUTS = *mut COMMTIMEOUTS;
pub type LPCOMSTAT = *mut COMSTAT;
pub type LPDCB = *mut DCB;
pub type LPEVENTLOG_FULL_INFORMATION = *mut EVENTLOG_FULL_INFORMATION;
#[cfg(feature = "winnt")]
pub type LPEXCEPTION_POINTERS = super::PEXCEPTION_POINTERS;
#[cfg(feature = "winnt")]
pub type LPEXCEPTION_RECORD = super::PEXCEPTION_RECORD;
pub type LPFIBER_START_ROUTINE = PFIBER_START_ROUTINE;
#[cfg(feature = "winnt")]
pub type LPFILE_ID_DESCRIPTOR = *mut FILE_ID_DESCRIPTOR;
pub type LPHW_PROFILE_INFO = LPHW_PROFILE_INFOA;
pub type LPHW_PROFILE_INFOA = *mut HW_PROFILE_INFOA;
pub type LPHW_PROFILE_INFOW = *mut HW_PROFILE_INFOW;
pub type LPJIT_DEBUG_INFO = *mut JIT_DEBUG_INFO;
pub type LPJIT_DEBUG_INFO32 = *mut JIT_DEBUG_INFO;
pub type LPJIT_DEBUG_INFO64 = *mut JIT_DEBUG_INFO;
#[cfg(target_arch = "x86")]
#[cfg(feature = "winnt")]
pub type LPLDT_ENTRY = super::PLDT_ENTRY;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type LPLDT_ENTRY = *mut core::ffi::c_void;
pub type LPMEMORYSTATUS = *mut MEMORYSTATUS;
pub type LPOFSTRUCT = *mut OFSTRUCT;
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
pub type LPPOWER_REQUEST_CONTEXT = *mut super::REASON_CONTEXT;
#[cfg(feature = "winnt")]
pub type LPPROGRESS_ROUTINE = Option<unsafe extern "system" fn(totalfilesize: i64, totalbytestransferred: i64, streamsize: i64, streambytestransferred: i64, dwstreamnumber: u32, dwcallbackreason: u32, hsourcefile: super::HANDLE, hdestinationfile: super::HANDLE, lpdata: *const core::ffi::c_void) -> u32>;
#[cfg(all(feature = "minwindef", feature = "processthreadsapi", feature = "winnt"))]
pub type LPSTARTUPINFOEX = LPSTARTUPINFOEXA;
#[cfg(all(feature = "minwindef", feature = "processthreadsapi", feature = "winnt"))]
pub type LPSTARTUPINFOEXA = *mut STARTUPINFOEXA;
#[cfg(all(feature = "minwindef", feature = "processthreadsapi", feature = "winnt"))]
pub type LPSTARTUPINFOEXW = *mut STARTUPINFOEXW;
pub type LPSYSTEM_POWER_STATUS = *mut SYSTEM_POWER_STATUS;
pub const LPTx: u32 = 128;
pub type LPWIN32_STREAM_ID = *mut WIN32_STREAM_ID;
pub const MARKPARITY: u32 = 3;
pub const MAXINTATOM: u32 = 49152;
pub const MAX_COMPUTERNAME_LENGTH: u32 = 15;
pub const MAX_PROFILE_LEN: u32 = 80;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MEMORYSTATUS {
    pub dwLength: u32,
    pub dwMemoryLoad: u32,
    pub dwTotalPhys: usize,
    pub dwAvailPhys: usize,
    pub dwTotalPageFile: usize,
    pub dwAvailPageFile: usize,
    pub dwTotalVirtual: usize,
    pub dwAvailVirtual: usize,
}
pub const MICROSOFT_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 0;
pub const MICROSOFT_WINDOWS_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: u32 = 1;
pub const MOVEFILE_COPY_ALLOWED: u32 = 2;
pub const MOVEFILE_CREATE_HARDLINK: u32 = 16;
pub const MOVEFILE_DELAY_UNTIL_REBOOT: u32 = 4;
pub const MOVEFILE_FAIL_IF_NOT_TRACKABLE: u32 = 32;
pub const MOVEFILE_REPLACE_EXISTING: u32 = 1;
pub const MOVEFILE_WRITE_THROUGH: u32 = 8;
pub const MS_CTS_ON: u32 = 16;
pub const MS_DSR_ON: u32 = 32;
pub const MS_RING_ON: u32 = 64;
pub const MS_RLSD_ON: u32 = 128;
pub const MaximumFileIdType: FILE_ID_TYPE = 3;
pub const MaximumIoPriorityHintType: PRIORITY_HINT = 3;
pub const NMPWAIT_NOWAIT: u32 = 1;
pub const NMPWAIT_USE_DEFAULT_WAIT: u32 = 0;
pub const NMPWAIT_WAIT_FOREVER: u32 = 4294967295;
pub const NOPARITY: u32 = 0;
pub const NORMAL_PRIORITY_CLASS: u32 = 32;
pub const ODDPARITY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct OFSTRUCT {
    pub cBytes: u8,
    pub fFixedDisk: u8,
    pub nErrCode: u16,
    pub Reserved1: u16,
    pub Reserved2: u16,
    pub szPathName: [i8; 128],
}
impl Default for OFSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OFS_MAXPATHNAME: u32 = 128;
pub const OF_CANCEL: u32 = 2048;
pub const OF_CREATE: u32 = 4096;
pub const OF_DELETE: u32 = 512;
pub const OF_EXIST: u32 = 16384;
pub const OF_PARSE: u32 = 256;
pub const OF_PROMPT: u32 = 8192;
pub const OF_READ: u32 = 0;
pub const OF_READWRITE: u32 = 2;
pub const OF_REOPEN: u32 = 32768;
pub const OF_SHARE_COMPAT: u32 = 0;
pub const OF_SHARE_DENY_NONE: u32 = 64;
pub const OF_SHARE_DENY_READ: u32 = 48;
pub const OF_SHARE_DENY_WRITE: u32 = 32;
pub const OF_SHARE_EXCLUSIVE: u32 = 16;
pub const OF_VERIFY: u32 = 1024;
pub const OF_WRITE: u32 = 1;
pub const ONE5STOPBITS: u32 = 1;
pub const ONESTOPBIT: u32 = 0;
pub const OPERATION_API_VERSION: u32 = 1;
pub const OPERATION_END_DISCARD: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OPERATION_END_PARAMETERS {
    pub Version: u32,
    pub OperationId: OPERATION_ID,
    pub Flags: u32,
}
pub type OPERATION_ID = u32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct OPERATION_START_PARAMETERS {
    pub Version: u32,
    pub OperationId: OPERATION_ID,
    pub Flags: u32,
}
pub const OPERATION_START_TRACE_CURRENT_THREAD: u32 = 1;
pub const OVERWRITE_HIDDEN: u32 = 4;
pub const ObjectIdType: FILE_ID_TYPE = 1;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PACTCTX = PACTCTXA;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PACTCTXA = *mut ACTCTXA;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PACTCTXW = *mut ACTCTXW;
#[cfg(feature = "winnt")]
pub type PACTCTX_SECTION_KEYED_DATA = *mut ACTCTX_SECTION_KEYED_DATA;
#[cfg(feature = "winnt")]
pub type PACTCTX_SECTION_KEYED_DATA_2600 = *mut ACTCTX_SECTION_KEYED_DATA_2600;
pub type PACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA = *mut ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA;
#[cfg(feature = "winnt")]
pub type PACTIVATION_CONTEXT_BASIC_INFORMATION = *mut ACTIVATION_CONTEXT_BASIC_INFORMATION;
pub const PARITY_EVEN: u16 = 1024;
pub const PARITY_MARK: u16 = 2048;
pub const PARITY_NONE: u16 = 256;
pub const PARITY_ODD: u16 = 512;
pub const PARITY_SPACE: u16 = 4096;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PCACTCTX = PCACTCTXA;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PCACTCTXA = *const ACTCTXA;
#[cfg(all(feature = "minwindef", feature = "winnt"))]
pub type PCACTCTXW = *const ACTCTXW;
#[cfg(feature = "winnt")]
pub type PCACTCTX_SECTION_KEYED_DATA = *const ACTCTX_SECTION_KEYED_DATA;
#[cfg(feature = "winnt")]
pub type PCACTCTX_SECTION_KEYED_DATA_2600 = *const ACTCTX_SECTION_KEYED_DATA_2600;
pub type PCACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA = *const ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA;
#[cfg(feature = "winnt")]
pub type PCACTIVATION_CONTEXT_BASIC_INFORMATION = *const ACTIVATION_CONTEXT_BASIC_INFORMATION;
pub const PCF_16BITMODE: u32 = 512;
pub const PCF_DTRDSR: u32 = 1;
pub const PCF_INTTIMEOUTS: u32 = 128;
pub const PCF_PARITY_CHECK: u32 = 8;
pub const PCF_RLSD: u32 = 4;
pub const PCF_RTSCTS: u32 = 2;
pub const PCF_SETXCHAR: u32 = 32;
pub const PCF_SPECIALCHARS: u32 = 256;
pub const PCF_TOTALTIMEOUTS: u32 = 64;
pub const PCF_XONXOFF: u32 = 16;
pub type PCOPYFILE2_CREATE_OPLOCK_KEYS = *mut COPYFILE2_CREATE_OPLOCK_KEYS;
#[cfg(feature = "winnt")]
pub type PCOPYFILE2_PROGRESS_ROUTINE = Option<unsafe extern "system" fn(pmessage: *const COPYFILE2_MESSAGE, pvcallbackcontext: *const core::ffi::c_void) -> COPYFILE2_MESSAGE_ACTION>;
pub type PFE_EXPORT_FUNC = Option<unsafe extern "system" fn(pbdata: *const u8, pvcallbackcontext: *const core::ffi::c_void, ullength: u32) -> u32>;
pub type PFE_IMPORT_FUNC = Option<unsafe extern "system" fn(pbdata: *mut u8, pvcallbackcontext: *const core::ffi::c_void, ullength: *mut u32) -> u32>;
pub type PFIBER_CALLOUT_ROUTINE = Option<unsafe extern "system" fn(lpparameter: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
pub type PFIBER_START_ROUTINE = Option<unsafe extern "system" fn(lpfiberparameter: *mut core::ffi::c_void)>;
pub type PFILE_ALIGNMENT_INFO = *mut FILE_ALIGNMENT_INFO;
pub type PFILE_ALLOCATION_INFO = *mut FILE_ALLOCATION_INFO;
pub type PFILE_ATTRIBUTE_TAG_INFO = *mut FILE_ATTRIBUTE_TAG_INFO;
pub type PFILE_BASIC_INFO = *mut FILE_BASIC_INFO;
pub type PFILE_CASE_SENSITIVE_INFO = *mut FILE_CASE_SENSITIVE_INFO;
pub type PFILE_COMPRESSION_INFO = *mut FILE_COMPRESSION_INFO;
pub type PFILE_DISPOSITION_INFO = *mut FILE_DISPOSITION_INFO;
pub type PFILE_DISPOSITION_INFO_EX = *mut FILE_DISPOSITION_INFO_EX;
pub type PFILE_END_OF_FILE_INFO = *mut FILE_END_OF_FILE_INFO;
pub type PFILE_FULL_DIR_INFO = *mut FILE_FULL_DIR_INFO;
#[cfg(feature = "winnt")]
pub type PFILE_ID_BOTH_DIR_INFO = *mut FILE_ID_BOTH_DIR_INFO;
#[cfg(feature = "winnt")]
pub type PFILE_ID_EXTD_DIR_INFO = *mut FILE_ID_EXTD_DIR_INFO;
#[cfg(feature = "winnt")]
pub type PFILE_ID_INFO = *mut FILE_ID_INFO;
pub type PFILE_ID_TYPE = *mut FILE_ID_TYPE;
pub type PFILE_IO_PRIORITY_HINT_INFO = *mut FILE_IO_PRIORITY_HINT_INFO;
pub type PFILE_NAME_INFO = *mut FILE_NAME_INFO;
pub type PFILE_REMOTE_PROTOCOL_INFO = *mut FILE_REMOTE_PROTOCOL_INFO;
#[cfg(feature = "winnt")]
pub type PFILE_RENAME_INFO = *mut FILE_RENAME_INFO;
pub type PFILE_STANDARD_INFO = *mut FILE_STANDARD_INFO;
pub type PFILE_STORAGE_INFO = *mut FILE_STORAGE_INFO;
pub type PFILE_STREAM_INFO = *mut FILE_STREAM_INFO;
pub type PGET_SYSTEM_WOW64_DIRECTORY_A = Option<unsafe extern "system" fn(lpbuffer: windows_sys::core::PSTR, usize: u32) -> u32>;
pub type PGET_SYSTEM_WOW64_DIRECTORY_W = Option<unsafe extern "system" fn(lpbuffer: windows_sys::core::PWSTR, usize: u32) -> u32>;
pub const PIPE_ACCEPT_REMOTE_CLIENTS: u32 = 0;
pub const PIPE_ACCESS_DUPLEX: u32 = 3;
pub const PIPE_ACCESS_INBOUND: u32 = 1;
pub const PIPE_ACCESS_OUTBOUND: u32 = 2;
pub const PIPE_CLIENT_END: u32 = 0;
pub const PIPE_NOWAIT: u32 = 1;
pub const PIPE_READMODE_BYTE: u32 = 0;
pub const PIPE_READMODE_MESSAGE: u32 = 2;
pub const PIPE_REJECT_REMOTE_CLIENTS: u32 = 8;
pub const PIPE_SERVER_END: u32 = 1;
pub const PIPE_TYPE_BYTE: u32 = 0;
pub const PIPE_TYPE_MESSAGE: u32 = 4;
pub const PIPE_UNLIMITED_INSTANCES: u32 = 255;
pub const PIPE_WAIT: u32 = 0;
pub type POFSTRUCT = *mut OFSTRUCT;
pub type POPERATION_END_PARAMETERS = *mut OPERATION_END_PARAMETERS;
pub type POPERATION_START_PARAMETERS = *mut OPERATION_START_PARAMETERS;
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
pub type POWER_REQUEST_CONTEXT = super::REASON_CONTEXT;
#[cfg(all(feature = "minwinbase", feature = "minwindef"))]
pub type PPOWER_REQUEST_CONTEXT = *mut super::REASON_CONTEXT;
pub type PPROCESS_CREATION_SME_VECTOR_LENGTH = *mut PROCESS_CREATION_SME_VECTOR_LENGTH;
pub type PPROCESS_CREATION_SVE_VECTOR_LENGTH = *mut PROCESS_CREATION_SVE_VECTOR_LENGTH;
#[cfg(feature = "winnt")]
pub type PQUERYACTCTXW_FUNC = Option<unsafe extern "system" fn(dwflags: u32, hactctx: super::HANDLE, pvsubinstance: *const core::ffi::c_void, ulinfoclass: u32, pvbuffer: *mut core::ffi::c_void, cbbuffer: usize, pcbwrittenorrequired: *mut usize) -> windows_sys::core::BOOL>;
pub type PRIORITY_HINT = i32;
pub const PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT: u32 = 1;
pub const PROCESS_CREATION_CHILD_PROCESS_OVERRIDE: u32 = 2;
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED: u32 = 1;
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED_UNLESS_SECURE: u32 = 4;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_DISABLE_PROCESS_TREE: u32 = 2;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_ENABLE_PROCESS_TREE: u32 = 1;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_OVERRIDE: u32 = 4;
pub const PROCESS_CREATION_DESKTOP_APP_TRUSTED_LAUNCH_ALLOW_WINDOWS_HOST: u32 = 8;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_BLOCK_NON_CET_BINARIES_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_BLOCK_NON_CET_BINARIES_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_BLOCK_NON_CET_BINARIES_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_BLOCK_NON_CET_BINARIES_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_BLOCK_NON_CET_BINARIES_RESERVED: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_CET_USER_SHADOW_STACKS_ALWAYS_OFF: u32 = 536870912;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_CET_USER_SHADOW_STACKS_ALWAYS_ON: u32 = 268435456;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_CET_USER_SHADOW_STACKS_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_CET_USER_SHADOW_STACKS_MASK: u32 = 805306368;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_CET_USER_SHADOW_STACKS_RESERVED: u32 = 805306368;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_RESERVED: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_XTENDED_CONTROL_FLOW_GUARD_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_XTENDED_CONTROL_FLOW_GUARD_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_XTENDED_CONTROL_FLOW_GUARD_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_XTENDED_CONTROL_FLOW_GUARD_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_XTENDED_CONTROL_FLOW_GUARD_RESERVED: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_ALLOW_DOWNGRADE_DYNAMIC_CODE_POLICY_ALWAYS_OFF: u32 = 2097152;
pub const PROCESS_CREATION_MITIGATION_POLICY2_ALLOW_DOWNGRADE_DYNAMIC_CODE_POLICY_ALWAYS_ON: u32 = 1048576;
pub const PROCESS_CREATION_MITIGATION_POLICY2_ALLOW_DOWNGRADE_DYNAMIC_CODE_POLICY_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_ALLOW_DOWNGRADE_DYNAMIC_CODE_POLICY_MASK: u32 = 3145728;
pub const PROCESS_CREATION_MITIGATION_POLICY2_ALLOW_DOWNGRADE_DYNAMIC_CODE_POLICY_RESERVED: u32 = 3145728;
pub const PROCESS_CREATION_MITIGATION_POLICY2_BLOCK_NON_CET_BINARIES_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_BLOCK_NON_CET_BINARIES_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_BLOCK_NON_CET_BINARIES_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_BLOCK_NON_CET_BINARIES_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_BLOCK_NON_CET_BINARIES_NON_EHCONT: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_DYNAMIC_APIS_OUT_OF_PROC_ONLY_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_DYNAMIC_APIS_OUT_OF_PROC_ONLY_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_DYNAMIC_APIS_OUT_OF_PROC_ONLY_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_DYNAMIC_APIS_OUT_OF_PROC_ONLY_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_DYNAMIC_APIS_OUT_OF_PROC_ONLY_RESERVED: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_USER_SHADOW_STACKS_ALWAYS_OFF: u32 = 536870912;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_USER_SHADOW_STACKS_ALWAYS_ON: u32 = 268435456;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_USER_SHADOW_STACKS_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_USER_SHADOW_STACKS_MASK: u32 = 805306368;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_USER_SHADOW_STACKS_STRICT_MODE: u32 = 805306368;
pub const PROCESS_CREATION_MITIGATION_POLICY2_FSCTL_SYSTEM_CALL_DISABLE_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_FSCTL_SYSTEM_CALL_DISABLE_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_FSCTL_SYSTEM_CALL_DISABLE_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_FSCTL_SYSTEM_CALL_DISABLE_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_FSCTL_SYSTEM_CALL_DISABLE_RESERVED: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_LOADER_INTEGRITY_CONTINUITY_ALWAYS_OFF: u32 = 32;
pub const PROCESS_CREATION_MITIGATION_POLICY2_LOADER_INTEGRITY_CONTINUITY_ALWAYS_ON: u32 = 16;
pub const PROCESS_CREATION_MITIGATION_POLICY2_LOADER_INTEGRITY_CONTINUITY_AUDIT: u32 = 48;
pub const PROCESS_CREATION_MITIGATION_POLICY2_LOADER_INTEGRITY_CONTINUITY_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_LOADER_INTEGRITY_CONTINUITY_MASK: u32 = 48;
pub const PROCESS_CREATION_MITIGATION_POLICY2_MODULE_TAMPERING_PROTECTION_ALWAYS_OFF: u32 = 8192;
pub const PROCESS_CREATION_MITIGATION_POLICY2_MODULE_TAMPERING_PROTECTION_ALWAYS_ON: u32 = 4096;
pub const PROCESS_CREATION_MITIGATION_POLICY2_MODULE_TAMPERING_PROTECTION_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_MODULE_TAMPERING_PROTECTION_MASK: u32 = 12288;
pub const PROCESS_CREATION_MITIGATION_POLICY2_MODULE_TAMPERING_PROTECTION_NOINHERIT: u32 = 12288;
pub const PROCESS_CREATION_MITIGATION_POLICY2_POINTER_AUTH_USER_IP_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_POINTER_AUTH_USER_IP_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_POINTER_AUTH_USER_IP_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_POINTER_AUTH_USER_IP_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_POINTER_AUTH_USER_IP_RESERVED: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_CORE_SHARING_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_CORE_SHARING_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_CORE_SHARING_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_CORE_SHARING_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_CORE_SHARING_RESERVED: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_INDIRECT_BRANCH_PREDICTION_ALWAYS_OFF: u32 = 131072;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_INDIRECT_BRANCH_PREDICTION_ALWAYS_ON: u32 = 65536;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_INDIRECT_BRANCH_PREDICTION_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_INDIRECT_BRANCH_PREDICTION_MASK: u32 = 196608;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_INDIRECT_BRANCH_PREDICTION_RESERVED: u32 = 196608;
pub const PROCESS_CREATION_MITIGATION_POLICY2_SPECULATIVE_STORE_BYPASS_DISABLE_ALWAYS_OFF: u32 = 33554432;
pub const PROCESS_CREATION_MITIGATION_POLICY2_SPECULATIVE_STORE_BYPASS_DISABLE_ALWAYS_ON: u32 = 16777216;
pub const PROCESS_CREATION_MITIGATION_POLICY2_SPECULATIVE_STORE_BYPASS_DISABLE_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_SPECULATIVE_STORE_BYPASS_DISABLE_MASK: u32 = 50331648;
pub const PROCESS_CREATION_MITIGATION_POLICY2_SPECULATIVE_STORE_BYPASS_DISABLE_RESERVED: u32 = 50331648;
pub const PROCESS_CREATION_MITIGATION_POLICY2_STRICT_CONTROL_FLOW_GUARD_ALWAYS_OFF: u32 = 512;
pub const PROCESS_CREATION_MITIGATION_POLICY2_STRICT_CONTROL_FLOW_GUARD_ALWAYS_ON: u32 = 256;
pub const PROCESS_CREATION_MITIGATION_POLICY2_STRICT_CONTROL_FLOW_GUARD_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_STRICT_CONTROL_FLOW_GUARD_MASK: u32 = 768;
pub const PROCESS_CREATION_MITIGATION_POLICY2_STRICT_CONTROL_FLOW_GUARD_RESERVED: u32 = 768;
pub const PROCESS_CREATION_MITIGATION_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_RELAXED_MODE: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_XTENDED_CONTROL_FLOW_GUARD_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_XTENDED_CONTROL_FLOW_GUARD_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_XTENDED_CONTROL_FLOW_GUARD_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_XTENDED_CONTROL_FLOW_GUARD_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_XTENDED_CONTROL_FLOW_GUARD_RESERVED: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_AUDIT_NONSYSTEM_FONTS: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALLOW_STORE: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_ALWAYS_OFF: u32 = 131072;
pub const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_ALWAYS_ON: u32 = 65536;
pub const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_MASK: u32 = 196608;
pub const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_RESERVED: u32 = 196608;
pub const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_EXPORT_SUPPRESSION: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ATL_THUNK_ENABLE: u32 = 2;
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ENABLE: u32 = 1;
pub const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_RESERVED: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_OFF: u32 = 512;
pub const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_ON: u32 = 256;
pub const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_ON_REQ_RELOCS: u32 = 768;
pub const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_MASK: u32 = 768;
pub const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_ALWAYS_OFF: u32 = 8192;
pub const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_ALWAYS_ON: u32 = 4096;
pub const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_MASK: u32 = 12288;
pub const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_RESERVED: u32 = 12288;
pub const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_ALWAYS_OFF: u32 = 2097152;
pub const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_ALWAYS_ON: u32 = 1048576;
pub const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_MASK: u32 = 3145728;
pub const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_RESERVED: u32 = 3145728;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_RESERVED: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_RESERVED: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_RESERVED: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_OFF: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_ON: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_ON_ALLOW_OPT_OUT: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_MASK: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_SEHOP_ENABLE: u32 = 4;
pub const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_ALWAYS_OFF: u32 = 33554432;
pub const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_ALWAYS_ON: u32 = 16777216;
pub const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_MASK: u32 = 50331648;
pub const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_RESERVED: u32 = 50331648;
pub const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_ALWAYS_OFF: u32 = 536870912;
pub const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_ALWAYS_ON: u32 = 268435456;
pub const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_DEFER: u32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_MASK: u32 = 805306368;
pub const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_RESERVED: u32 = 805306368;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PROCESS_CREATION_SME_VECTOR_LENGTH {
    pub Anonymous: PROCESS_CREATION_SME_VECTOR_LENGTH_0,
    pub Reserved: u32,
}
impl Default for PROCESS_CREATION_SME_VECTOR_LENGTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_CREATION_SME_VECTOR_LENGTH_0 {
    pub Data: u32,
    pub Anonymous: PROCESS_CREATION_SME_VECTOR_LENGTH_0_0,
}
impl Default for PROCESS_CREATION_SME_VECTOR_LENGTH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_CREATION_SME_VECTOR_LENGTH_0_0 {
    pub _bitfield: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union PROCESS_CREATION_SVE_VECTOR_LENGTH {
    pub Data: u32,
    pub Anonymous: PROCESS_CREATION_SVE_VECTOR_LENGTH_0,
}
impl Default for PROCESS_CREATION_SVE_VECTOR_LENGTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct PROCESS_CREATION_SVE_VECTOR_LENGTH_0 {
    pub _bitfield: u32,
}
pub const PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION: u32 = 2;
pub const PROCESS_DEP_ENABLE: u32 = 1;
pub const PROCESS_MODE_BACKGROUND_BEGIN: u32 = 1048576;
pub const PROCESS_MODE_BACKGROUND_END: u32 = 2097152;
pub const PROCESS_NAME_NATIVE: u32 = 1;
pub const PROC_THREAD_ATTRIBUTE_ADDITIVE: u32 = 262144;
pub const PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY: u32 = 131087;
pub const PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY: u32 = 131086;
pub const PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER: u32 = 131098;
pub const PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY: u32 = 131090;
pub const PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES: u32 = 196635;
pub const PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY: u32 = 196611;
pub const PROC_THREAD_ATTRIBUTE_HANDLE_LIST: u32 = 131074;
pub const PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR: u32 = 196613;
pub const PROC_THREAD_ATTRIBUTE_INPUT: u32 = 131072;
pub const PROC_THREAD_ATTRIBUTE_JOB_LIST: u32 = 131085;
pub const PROC_THREAD_ATTRIBUTE_MACHINE_TYPE: u32 = 131097;
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_AUDIT_POLICY: u32 = 131096;
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY: u32 = 131079;
pub type PROC_THREAD_ATTRIBUTE_NUM = i32;
pub const PROC_THREAD_ATTRIBUTE_NUMBER: u32 = 65535;
pub const PROC_THREAD_ATTRIBUTE_PARENT_PROCESS: u32 = 131072;
pub const PROC_THREAD_ATTRIBUTE_PREFERRED_NODE: u32 = 131076;
pub const PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL: u32 = 131083;
pub const PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE: u32 = 131094;
pub const PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES: u32 = 131081;
pub const PROC_THREAD_ATTRIBUTE_SME_VECTOR_LENGTH: u32 = 131103;
pub const PROC_THREAD_ATTRIBUTE_SVE_VECTOR_LENGTH: u32 = 131102;
pub const PROC_THREAD_ATTRIBUTE_THREAD: u32 = 65536;
pub const PROC_THREAD_ATTRIBUTE_TRUSTED_APP: u32 = 131101;
pub const PROC_THREAD_ATTRIBUTE_UMS_THREAD: u32 = 196614;
pub const PROC_THREAD_ATTRIBUTE_WIN32K_FILTER: u32 = 131088;
pub const PROFILE_KERNEL: u32 = 536870912;
pub const PROFILE_SERVER: u32 = 1073741824;
pub const PROFILE_USER: u32 = 268435456;
pub const PROGRESS_CANCEL: u32 = 1;
pub const PROGRESS_CONTINUE: u32 = 0;
pub const PROGRESS_QUIET: u32 = 3;
pub const PROGRESS_STOP: u32 = 2;
pub const PROTECTION_LEVEL_ANTIMALWARE_LIGHT: u32 = 3;
pub const PROTECTION_LEVEL_AUTHENTICODE: u32 = 7;
pub const PROTECTION_LEVEL_CODEGEN_LIGHT: u32 = 6;
pub const PROTECTION_LEVEL_LSA_LIGHT: u32 = 4;
pub const PROTECTION_LEVEL_NONE: u32 = 4294967294;
pub const PROTECTION_LEVEL_PPL_APP: u32 = 8;
pub const PROTECTION_LEVEL_SAME: u32 = 4294967295;
pub const PROTECTION_LEVEL_WINDOWS: u32 = 1;
pub const PROTECTION_LEVEL_WINDOWS_LIGHT: u32 = 2;
pub const PROTECTION_LEVEL_WINTCB: u32 = 5;
pub const PROTECTION_LEVEL_WINTCB_LIGHT: u32 = 0;
pub const PST_FAX: u32 = 33;
pub const PST_LAT: u32 = 257;
pub const PST_MODEM: u32 = 6;
pub const PST_NETWORK_BRIDGE: u32 = 256;
pub const PST_PARALLELPORT: u32 = 2;
pub const PST_RS232: u32 = 1;
pub const PST_RS422: u32 = 3;
pub const PST_RS423: u32 = 4;
pub const PST_RS449: u32 = 5;
pub const PST_SCANNER: u32 = 34;
pub const PST_TCPIP_TELNET: u32 = 258;
pub const PST_UNSPECIFIED: u32 = 0;
pub const PST_X25: u32 = 259;
pub type PUMS_COMPLETION_LIST = *mut core::ffi::c_void;
pub type PUMS_CONTEXT = *mut core::ffi::c_void;
#[cfg(feature = "winnt")]
pub type PUMS_SCHEDULER_ENTRY_POINT = super::PRTL_UMS_SCHEDULER_ENTRY_POINT;
#[cfg(feature = "winnt")]
pub type PUMS_SCHEDULER_STARTUP_INFO = *mut UMS_SCHEDULER_STARTUP_INFO;
pub type PUMS_SYSTEM_THREAD_INFORMATION = *mut UMS_SYSTEM_THREAD_INFORMATION;
#[cfg(feature = "winnt")]
pub type PUMS_THREAD_INFO_CLASS = *mut super::RTL_UMS_THREAD_INFO_CLASS;
pub const PURGE_RXABORT: u32 = 2;
pub const PURGE_RXCLEAR: u32 = 8;
pub const PURGE_TXABORT: u32 = 1;
pub const PURGE_TXCLEAR: u32 = 4;
pub const ProcThreadAttributeAllApplicationPackagesPolicy: PROC_THREAD_ATTRIBUTE_NUM = 15;
pub const ProcThreadAttributeChildProcessPolicy: PROC_THREAD_ATTRIBUTE_NUM = 14;
pub const ProcThreadAttributeComponentFilter: PROC_THREAD_ATTRIBUTE_NUM = 26;
pub const ProcThreadAttributeDesktopAppPolicy: PROC_THREAD_ATTRIBUTE_NUM = 18;
pub const ProcThreadAttributeEnableOptionalXStateFeatures: PROC_THREAD_ATTRIBUTE_NUM = 27;
pub const ProcThreadAttributeGroupAffinity: PROC_THREAD_ATTRIBUTE_NUM = 3;
pub const ProcThreadAttributeHandleList: PROC_THREAD_ATTRIBUTE_NUM = 2;
pub const ProcThreadAttributeIdealProcessor: PROC_THREAD_ATTRIBUTE_NUM = 5;
pub const ProcThreadAttributeJobList: PROC_THREAD_ATTRIBUTE_NUM = 13;
pub const ProcThreadAttributeMachineType: PROC_THREAD_ATTRIBUTE_NUM = 25;
pub const ProcThreadAttributeMitigationAuditPolicy: PROC_THREAD_ATTRIBUTE_NUM = 24;
pub const ProcThreadAttributeMitigationPolicy: PROC_THREAD_ATTRIBUTE_NUM = 7;
pub const ProcThreadAttributeParentProcess: PROC_THREAD_ATTRIBUTE_NUM = 0;
pub const ProcThreadAttributePreferredNode: PROC_THREAD_ATTRIBUTE_NUM = 4;
pub const ProcThreadAttributeProtectionLevel: PROC_THREAD_ATTRIBUTE_NUM = 11;
pub const ProcThreadAttributePseudoConsole: PROC_THREAD_ATTRIBUTE_NUM = 22;
pub const ProcThreadAttributeSafeOpenPromptOriginClaim: PROC_THREAD_ATTRIBUTE_NUM = 17;
pub const ProcThreadAttributeSecurityCapabilities: PROC_THREAD_ATTRIBUTE_NUM = 9;
pub const ProcThreadAttributeSmeVectorLength: PROC_THREAD_ATTRIBUTE_NUM = 31;
pub const ProcThreadAttributeSveVectorLength: PROC_THREAD_ATTRIBUTE_NUM = 30;
pub const ProcThreadAttributeTrustedApp: PROC_THREAD_ATTRIBUTE_NUM = 29;
pub const ProcThreadAttributeUmsThread: PROC_THREAD_ATTRIBUTE_NUM = 6;
pub const ProcThreadAttributeWin32kFilter: PROC_THREAD_ATTRIBUTE_NUM = 16;
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_ADDRESS: u32 = 16;
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_HMODULE: u32 = 8;
pub const QUERY_ACTCTX_FLAG_NO_ADDREF: u32 = 2147483648;
pub const QUERY_ACTCTX_FLAG_USE_ACTIVE_ACTCTX: u32 = 4;
pub const REALTIME_PRIORITY_CLASS: u32 = 256;
pub const RECOVERY_DEFAULT_PING_INTERVAL: u32 = 5000;
pub const RECOVERY_MAX_PING_INTERVAL: u32 = 300000;
pub const REMOTE_PROTOCOL_INFO_FLAG_LOOPBACK: u32 = 1;
pub const REMOTE_PROTOCOL_INFO_FLAG_OFFLINE: u32 = 2;
pub const REMOTE_PROTOCOL_INFO_FLAG_PERSISTENT_HANDLE: u32 = 4;
pub const REPLACEFILE_IGNORE_ACL_ERRORS: u32 = 4;
pub const REPLACEFILE_IGNORE_MERGE_ERRORS: u32 = 2;
pub const REPLACEFILE_WRITE_THROUGH: u32 = 1;
pub const RESETDEV: u32 = 7;
pub const RESTART_MAX_CMD_LINE: u32 = 1024;
pub const RESTART_NO_CRASH: u32 = 1;
pub const RESTART_NO_HANG: u32 = 2;
pub const RESTART_NO_PATCH: u32 = 4;
pub const RESTART_NO_REBOOT: u32 = 8;
pub const RPI_FLAG_SMB2_SHARECAP_CLUSTER: u32 = 64;
pub const RPI_FLAG_SMB2_SHARECAP_CONTINUOUS_AVAILABILITY: u32 = 16;
pub const RPI_FLAG_SMB2_SHARECAP_DFS: u32 = 8;
pub const RPI_FLAG_SMB2_SHARECAP_SCALEOUT: u32 = 32;
pub const RPI_FLAG_SMB2_SHARECAP_TIMEWARP: u32 = 2;
pub const RPI_SMB2_FLAG_SERVERCAP_DFS: u32 = 1;
pub const RPI_SMB2_FLAG_SERVERCAP_DIRECTORY_LEASING: u32 = 32;
pub const RPI_SMB2_FLAG_SERVERCAP_LARGEMTU: u32 = 4;
pub const RPI_SMB2_FLAG_SERVERCAP_LEASING: u32 = 2;
pub const RPI_SMB2_FLAG_SERVERCAP_MULTICHANNEL: u32 = 8;
pub const RPI_SMB2_FLAG_SERVERCAP_PERSISTENT_HANDLES: u32 = 16;
pub const RPI_SMB2_SHAREFLAG_COMPRESS_DATA: u32 = 2;
pub const RPI_SMB2_SHAREFLAG_ENCRYPT_DATA: u32 = 1;
pub const RTS_CONTROL_DISABLE: u32 = 0;
pub const RTS_CONTROL_ENABLE: u32 = 1;
pub const RTS_CONTROL_HANDSHAKE: u32 = 2;
pub const RTS_CONTROL_TOGGLE: u32 = 3;
pub const SCS_32BIT_BINARY: u32 = 0;
pub const SCS_64BIT_BINARY: u32 = 6;
pub const SCS_DOS_BINARY: u32 = 1;
pub const SCS_OS216_BINARY: u32 = 5;
pub const SCS_PIF_BINARY: u32 = 3;
pub const SCS_POSIX_BINARY: u32 = 4;
#[cfg(target_arch = "x86")]
pub const SCS_THIS_PLATFORM_BINARY: u32 = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SCS_THIS_PLATFORM_BINARY: u32 = 6;
pub const SCS_WOW_BINARY: u32 = 2;
pub const SECURITY_ANONYMOUS: u32 = 0;
pub const SECURITY_CONTEXT_TRACKING: u32 = 262144;
pub const SECURITY_DELEGATION: u32 = 196608;
pub const SECURITY_EFFECTIVE_ONLY: u32 = 524288;
pub const SECURITY_IDENTIFICATION: u32 = 65536;
pub const SECURITY_IMPERSONATION: u32 = 131072;
pub const SECURITY_SQOS_PRESENT: u32 = 1048576;
pub const SECURITY_VALID_SQOS_FLAGS: u32 = 2031616;
pub const SEM_FAILCRITICALERRORS: u32 = 1;
pub const SEM_NOALIGNMENTFAULTEXCEPT: u32 = 4;
pub const SEM_NOGPFAULTERRORBOX: u32 = 2;
pub const SEM_NOOPENFILEERRORBOX: u32 = 32768;
pub const SETBREAK: u32 = 8;
pub const SETDTR: u32 = 5;
pub const SETRTS: u32 = 3;
pub const SETXOFF: u32 = 1;
pub const SETXON: u32 = 2;
pub const SET_TAPE_DRIVE_INFORMATION: u32 = 1;
pub const SET_TAPE_MEDIA_INFORMATION: u32 = 0;
pub const SHUTDOWN_NORETRY: u32 = 1;
pub const SPACEPARITY: u32 = 4;
pub const SP_BAUD: u32 = 2;
pub const SP_DATABITS: u32 = 4;
pub const SP_HANDSHAKING: u32 = 16;
pub const SP_PARITY: u32 = 1;
pub const SP_PARITY_CHECK: u32 = 32;
pub const SP_RLSD: u32 = 64;
pub const SP_SERIALCOMM: u32 = 1;
pub const SP_STOPBITS: u32 = 8;
pub const STACK_SIZE_PARAM_IS_A_RESERVATION: u32 = 65536;
pub const STARTF_FORCEOFFFEEDBACK: u32 = 128;
pub const STARTF_FORCEONFEEDBACK: u32 = 64;
pub const STARTF_HOLOGRAPHIC: u32 = 262144;
pub const STARTF_PREVENTPINNING: u32 = 8192;
pub const STARTF_RUNFULLSCREEN: u32 = 32;
pub const STARTF_TITLEISAPPID: u32 = 4096;
pub const STARTF_TITLEISLINKNAME: u32 = 2048;
pub const STARTF_UNTRUSTEDSOURCE: u32 = 32768;
pub const STARTF_USECOUNTCHARS: u32 = 8;
pub const STARTF_USEFILLATTRIBUTE: u32 = 16;
pub const STARTF_USEHOTKEY: u32 = 512;
pub const STARTF_USEPOSITION: u32 = 4;
pub const STARTF_USESHOWWINDOW: u32 = 1;
pub const STARTF_USESIZE: u32 = 2;
pub const STARTF_USESTDHANDLES: u32 = 256;
#[cfg(all(feature = "minwindef", feature = "processthreadsapi", feature = "winnt"))]
pub type STARTUPINFOEX = STARTUPINFOEXA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "processthreadsapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STARTUPINFOEXA {
    pub StartupInfo: super::STARTUPINFOA,
    pub lpAttributeList: super::LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[cfg(all(feature = "minwindef", feature = "processthreadsapi", feature = "winnt"))]
impl Default for STARTUPINFOEXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "processthreadsapi", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct STARTUPINFOEXW {
    pub StartupInfo: super::STARTUPINFOW,
    pub lpAttributeList: super::LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[cfg(all(feature = "minwindef", feature = "processthreadsapi", feature = "winnt"))]
impl Default for STARTUPINFOEXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const STD_ERROR_HANDLE: u32 = 4294967284;
pub const STD_INPUT_HANDLE: u32 = 4294967286;
pub const STD_OUTPUT_HANDLE: u32 = 4294967285;
pub const STOPBITS_10: u16 = 1;
pub const STOPBITS_15: u16 = 2;
pub const STOPBITS_20: u16 = 4;
pub const STORAGE_INFO_FLAGS_ALIGNED_DEVICE: u32 = 1;
pub const STORAGE_INFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE: u32 = 2;
pub const STORAGE_INFO_OFFSET_UNKNOWN: u32 = 4294967295;
pub const STREAM_CONTAINS_GHOSTED_FILE_EXTENTS: u32 = 16;
pub const STREAM_CONTAINS_PROPERTIES: u32 = 4;
pub const STREAM_CONTAINS_SECURITY: u32 = 2;
pub const STREAM_MODIFIED_WHEN_READ: u32 = 1;
pub const STREAM_NORMAL_ATTRIBUTE: u32 = 0;
pub const STREAM_SPARSE_ATTRIBUTE: u32 = 8;
pub const SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE: u32 = 2;
pub const SYMBOLIC_LINK_FLAG_DIRECTORY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SYSTEM_POWER_STATUS {
    pub ACLineStatus: u8,
    pub BatteryFlag: u8,
    pub BatteryLifePercent: u8,
    pub SystemStatusFlag: u8,
    pub BatteryLifeTime: u32,
    pub BatteryFullLifeTime: u32,
}
pub const SYSTEM_STATUS_FLAG_POWER_SAVING_ON: u32 = 1;
pub const S_ALLTHRESHOLD: u32 = 2;
pub const S_LEGATO: u32 = 1;
pub const S_NORMAL: u32 = 0;
pub const S_PERIOD1024: u32 = 1;
pub const S_PERIOD2048: u32 = 2;
pub const S_PERIOD512: u32 = 0;
pub const S_PERIODVOICE: u32 = 3;
pub const S_QUEUEEMPTY: u32 = 0;
pub const S_SERBDNT: i32 = -5;
pub const S_SERDCC: i32 = -7;
pub const S_SERDDR: i32 = -14;
pub const S_SERDFQ: i32 = -13;
pub const S_SERDLN: i32 = -6;
pub const S_SERDMD: i32 = -10;
pub const S_SERDPT: i32 = -12;
pub const S_SERDSH: i32 = -11;
pub const S_SERDSR: i32 = -15;
pub const S_SERDST: i32 = -16;
pub const S_SERDTP: i32 = -8;
pub const S_SERDVL: i32 = -9;
pub const S_SERDVNA: i32 = -1;
pub const S_SERMACT: i32 = -3;
pub const S_SEROFM: i32 = -2;
pub const S_SERQFUL: i32 = -4;
pub const S_STACCATO: u32 = 2;
pub const S_THRESHOLD: u32 = 1;
pub const S_WHITE1024: u32 = 5;
pub const S_WHITE2048: u32 = 6;
pub const S_WHITE512: u32 = 4;
pub const S_WHITEVOICE: u32 = 7;
pub const TC_GP_TRAP: u32 = 2;
pub const TC_HARDERR: u32 = 1;
pub const TC_NORMAL: u32 = 0;
pub const TC_SIGNAL: u32 = 3;
pub const THREAD_MODE_BACKGROUND_BEGIN: u32 = 65536;
pub const THREAD_MODE_BACKGROUND_END: u32 = 131072;
pub const THREAD_PRIORITY_ABOVE_NORMAL: u32 = 1;
pub const THREAD_PRIORITY_BELOW_NORMAL: i32 = -1;
pub const THREAD_PRIORITY_ERROR_RETURN: u32 = 2147483647;
pub const THREAD_PRIORITY_HIGHEST: u32 = 2;
pub const THREAD_PRIORITY_IDLE: i32 = -15;
pub const THREAD_PRIORITY_LOWEST: i32 = -2;
pub const THREAD_PRIORITY_NORMAL: u32 = 0;
pub const THREAD_PRIORITY_TIME_CRITICAL: u32 = 15;
pub const TWOSTOPBITS: u32 = 2;
#[cfg(feature = "winnt")]
pub type UMS_SCHEDULER_REASON = super::RTL_UMS_SCHEDULER_REASON;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct UMS_SCHEDULER_STARTUP_INFO {
    pub UmsVersion: u32,
    pub CompletionList: PUMS_COMPLETION_LIST,
    pub SchedulerProc: PUMS_SCHEDULER_ENTRY_POINT,
    pub SchedulerParam: *mut core::ffi::c_void,
}
#[cfg(feature = "winnt")]
impl Default for UMS_SCHEDULER_STARTUP_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UMS_SYSTEM_THREAD_INFORMATION {
    pub UmsVersion: u32,
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0,
}
impl Default for UMS_SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union UMS_SYSTEM_THREAD_INFORMATION_0 {
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0_0,
    pub ThreadUmsFlags: u32,
}
impl Default for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "winnt")]
pub type UMS_THREAD_INFO_CLASS = super::RTL_UMS_THREAD_INFO_CLASS;
pub const UMS_VERSION: u32 = 256;
pub const VOLUME_NAME_DOS: u32 = 0;
pub const VOLUME_NAME_GUID: u32 = 1;
pub const VOLUME_NAME_NONE: u32 = 4;
pub const VOLUME_NAME_NT: u32 = 2;
pub const WAIT_ABANDONED: u32 = 128;
pub const WAIT_ABANDONED_0: u32 = 128;
pub const WAIT_FAILED: u32 = 4294967295;
pub const WAIT_IO_COMPLETION: u32 = 192;
pub const WAIT_OBJECT_0: u32 = 0;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WIN32_STREAM_ID {
    pub dwStreamId: u32,
    pub dwStreamAttributes: u32,
    pub Size: i64,
    pub dwStreamNameSize: u32,
    pub cStreamName: [u16; 1],
}
impl Default for WIN32_STREAM_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
