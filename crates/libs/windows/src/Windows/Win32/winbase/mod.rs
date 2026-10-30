#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AccessCheckAndAuditAlarmA<P0, P2, P3>(subsystemname: P0, handleid: Option<super::LPVOID>, objecttypename: P2, objectname: P3, securitydescriptor: super::PSECURITY_DESCRIPTOR, desiredaccess: super::DWORD, genericmapping: super::PGENERIC_MAPPING, objectcreation: bool, grantedaccess: super::LPDWORD, accessstatus: super::LPBOOL, pfgenerateonclose: super::LPBOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AccessCheckAndAuditAlarmA(subsystemname : windows_core::PCSTR, handleid : super::LPVOID, objecttypename : windows_core::PCSTR, objectname : windows_core::PCSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, desiredaccess : super::DWORD, genericmapping : super::PGENERIC_MAPPING, objectcreation : windows_core::BOOL, grantedaccess : super::LPDWORD, accessstatus : super::LPBOOL, pfgenerateonclose : super::LPBOOL) -> windows_core::BOOL);
    unsafe { AccessCheckAndAuditAlarmA(subsystemname.param().abi(), handleid.unwrap_or(core::mem::zeroed()) as _, objecttypename.param().abi(), objectname.param().abi(), securitydescriptor, desiredaccess, genericmapping, objectcreation.into(), grantedaccess as _, accessstatus as _, pfgenerateonclose as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AccessCheckByTypeAndAuditAlarmA<P0, P2, P3>(subsystemname: P0, handleid: super::LPVOID, objecttypename: P2, objectname: P3, securitydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, desiredaccess: super::DWORD, audittype: super::AUDIT_EVENT_TYPE, flags: super::DWORD, mut objecttypelist: Option<&mut [super::OBJECT_TYPE_LIST]>, genericmapping: super::PGENERIC_MAPPING, objectcreation: bool, grantedaccess: super::LPDWORD, accessstatus: super::LPBOOL, pfgenerateonclose: super::LPBOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AccessCheckByTypeAndAuditAlarmA(subsystemname : windows_core::PCSTR, handleid : super::LPVOID, objecttypename : windows_core::PCSTR, objectname : windows_core::PCSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : super::DWORD, audittype : super::AUDIT_EVENT_TYPE, flags : super::DWORD, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : super::DWORD, genericmapping : super::PGENERIC_MAPPING, objectcreation : windows_core::BOOL, grantedaccess : super::LPDWORD, accessstatus : super::LPBOOL, pfgenerateonclose : super::LPBOOL) -> windows_core::BOOL);
    unsafe {
        AccessCheckByTypeAndAuditAlarmA(
            subsystemname.param().abi(),
            handleid,
            objecttypename.param().abi(),
            objectname.param().abi(),
            securitydescriptor,
            principalselfsid.unwrap_or(core::mem::zeroed()) as _,
            desiredaccess,
            audittype,
            flags,
            core::mem::transmute(objecttypelist.as_mut().map_or(core::ptr::null_mut(), |slice| slice.as_mut_ptr())),
            objecttypelist.as_deref().map_or(0, |slice| slice.len().try_into().unwrap()),
            genericmapping,
            objectcreation.into(),
            grantedaccess as _,
            accessstatus as _,
            pfgenerateonclose as _,
        )
    }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmA<P0, P2, P3>(subsystemname: P0, handleid: super::LPVOID, objecttypename: P2, objectname: P3, securitydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, desiredaccess: super::DWORD, audittype: super::AUDIT_EVENT_TYPE, flags: super::DWORD, objecttypelist: Option<super::POBJECT_TYPE_LIST>, objecttypelistlength: super::DWORD, genericmapping: super::PGENERIC_MAPPING, objectcreation: bool, grantedaccess: super::LPDWORD, accessstatuslist: super::LPDWORD, pfgenerateonclose: super::LPBOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmA(subsystemname : windows_core::PCSTR, handleid : super::LPVOID, objecttypename : windows_core::PCSTR, objectname : windows_core::PCSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : super::DWORD, audittype : super::AUDIT_EVENT_TYPE, flags : super::DWORD, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : super::DWORD, genericmapping : super::PGENERIC_MAPPING, objectcreation : windows_core::BOOL, grantedaccess : super::LPDWORD, accessstatuslist : super::LPDWORD, pfgenerateonclose : super::LPBOOL) -> windows_core::BOOL);
    unsafe { AccessCheckByTypeResultListAndAuditAlarmA(subsystemname.param().abi(), handleid, objecttypename.param().abi(), objectname.param().abi(), securitydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, desiredaccess, audittype, flags, objecttypelist.unwrap_or(core::mem::zeroed()) as _, objecttypelistlength, genericmapping, objectcreation.into(), grantedaccess as _, accessstatuslist as _, pfgenerateonclose as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AccessCheckByTypeResultListAndAuditAlarmByHandleA<P0, P3, P4>(subsystemname: P0, handleid: super::LPVOID, clienttoken: super::HANDLE, objecttypename: P3, objectname: P4, securitydescriptor: super::PSECURITY_DESCRIPTOR, principalselfsid: Option<super::PSID>, desiredaccess: super::DWORD, audittype: super::AUDIT_EVENT_TYPE, flags: super::DWORD, objecttypelist: Option<super::POBJECT_TYPE_LIST>, objecttypelistlength: super::DWORD, genericmapping: super::PGENERIC_MAPPING, objectcreation: bool, grantedaccess: super::LPDWORD, accessstatuslist: super::LPDWORD, pfgenerateonclose: super::LPBOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn AccessCheckByTypeResultListAndAuditAlarmByHandleA(subsystemname : windows_core::PCSTR, handleid : super::LPVOID, clienttoken : super::HANDLE, objecttypename : windows_core::PCSTR, objectname : windows_core::PCSTR, securitydescriptor : super::PSECURITY_DESCRIPTOR, principalselfsid : super::PSID, desiredaccess : super::DWORD, audittype : super::AUDIT_EVENT_TYPE, flags : super::DWORD, objecttypelist : super::POBJECT_TYPE_LIST, objecttypelistlength : super::DWORD, genericmapping : super::PGENERIC_MAPPING, objectcreation : windows_core::BOOL, grantedaccess : super::LPDWORD, accessstatuslist : super::LPDWORD, pfgenerateonclose : super::LPBOOL) -> windows_core::BOOL);
    unsafe { AccessCheckByTypeResultListAndAuditAlarmByHandleA(subsystemname.param().abi(), handleid, clienttoken, objecttypename.param().abi(), objectname.param().abi(), securitydescriptor, principalselfsid.unwrap_or(core::mem::zeroed()) as _, desiredaccess, audittype, flags, objecttypelist.unwrap_or(core::mem::zeroed()) as _, objecttypelistlength, genericmapping, objectcreation.into(), grantedaccess as _, accessstatuslist as _, pfgenerateonclose as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ActivateActCtx(hactctx: Option<super::HANDLE>, lpcookie: *mut super::ULONG_PTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ActivateActCtx(hactctx : super::HANDLE, lpcookie : *mut super::ULONG_PTR) -> windows_core::BOOL);
    unsafe { ActivateActCtx(hactctx.unwrap_or(core::mem::zeroed()) as _, lpcookie as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn AddAtomA<P0>(lpstring: P0) -> super::ATOM
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn AddAtomA(lpstring : windows_core::PCSTR) -> super::ATOM);
    unsafe { AddAtomA(lpstring.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn AddAtomW<P0>(lpstring: P0) -> super::ATOM
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn AddAtomW(lpstring : windows_core::PCWSTR) -> super::ATOM);
    unsafe { AddAtomW(lpstring.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddConditionalAce(pacl: super::PACL, dwacerevision: super::DWORD, aceflags: super::DWORD, acetype: super::UCHAR, accessmask: super::DWORD, psid: super::PSID, conditionstr: super::PWCHAR, returnlength: *mut super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn AddConditionalAce(pacl : super::PACL, dwacerevision : super::DWORD, aceflags : super::DWORD, acetype : super::UCHAR, accessmask : super::DWORD, psid : super::PSID, conditionstr : super::PWCHAR, returnlength : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { AddConditionalAce(pacl as _, dwacerevision, aceflags, acetype, accessmask, psid, conditionstr, returnlength as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddIntegrityLabelToBoundaryDescriptor(boundarydescriptor: *mut super::HANDLE, integritylabel: super::PSID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AddIntegrityLabelToBoundaryDescriptor(boundarydescriptor : *mut super::HANDLE, integritylabel : super::PSID) -> windows_core::BOOL);
    unsafe { AddIntegrityLabelToBoundaryDescriptor(boundarydescriptor as _, integritylabel) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn AddRefActCtx(hactctx: super::HANDLE) {
    windows_core::link!("kernel32.dll" "system" fn AddRefActCtx(hactctx : super::HANDLE));
    unsafe { AddRefActCtx(hactctx as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn AddSecureMemoryCacheCallback(pfncallback: super::PSECURE_MEMORY_CACHE_CALLBACK) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn AddSecureMemoryCacheCallback(pfncallback : super::PSECURE_MEMORY_CACHE_CALLBACK) -> windows_core::BOOL);
    unsafe { AddSecureMemoryCacheCallback(pfncallback) }
}
#[inline]
pub unsafe fn ApplicationRecoveryFinished(bsuccess: bool) {
    windows_core::link!("kernel32.dll" "system" fn ApplicationRecoveryFinished(bsuccess : windows_core::BOOL));
    unsafe { ApplicationRecoveryFinished(bsuccess.into()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ApplicationRecoveryInProgress(pbcancelled: super::PBOOL) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn ApplicationRecoveryInProgress(pbcancelled : super::PBOOL) -> windows_core::HRESULT);
    unsafe { ApplicationRecoveryInProgress(pbcancelled as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn BackupEventLogA<P1>(heventlog: super::HANDLE, lpbackupfilename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BackupEventLogA(heventlog : super::HANDLE, lpbackupfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { BackupEventLogA(heventlog, lpbackupfilename.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn BackupEventLogW<P1>(heventlog: super::HANDLE, lpbackupfilename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn BackupEventLogW(heventlog : super::HANDLE, lpbackupfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { BackupEventLogW(heventlog, lpbackupfilename.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BackupRead(hfile: super::HANDLE, lpbuffer: super::LPBYTE, nnumberofbytestoread: super::DWORD, lpnumberofbytesread: super::LPDWORD, babort: bool, bprocesssecurity: bool, lpcontext: *mut super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn BackupRead(hfile : super::HANDLE, lpbuffer : super::LPBYTE, nnumberofbytestoread : super::DWORD, lpnumberofbytesread : super::LPDWORD, babort : windows_core::BOOL, bprocesssecurity : windows_core::BOOL, lpcontext : *mut super::LPVOID) -> windows_core::BOOL);
    unsafe { BackupRead(hfile, lpbuffer as _, nnumberofbytestoread, lpnumberofbytesread as _, babort.into(), bprocesssecurity.into(), lpcontext as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BackupSeek(hfile: super::HANDLE, dwlowbytestoseek: super::DWORD, dwhighbytestoseek: super::DWORD, lpdwlowbyteseeked: super::LPDWORD, lpdwhighbyteseeked: super::LPDWORD, lpcontext: *mut super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn BackupSeek(hfile : super::HANDLE, dwlowbytestoseek : super::DWORD, dwhighbytestoseek : super::DWORD, lpdwlowbyteseeked : super::LPDWORD, lpdwhighbyteseeked : super::LPDWORD, lpcontext : *mut super::LPVOID) -> windows_core::BOOL);
    unsafe { BackupSeek(hfile, dwlowbytestoseek, dwhighbytestoseek, lpdwlowbyteseeked as _, lpdwhighbyteseeked as _, lpcontext as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BackupWrite(hfile: super::HANDLE, lpbuffer: &[u8], lpnumberofbyteswritten: super::LPDWORD, babort: bool, bprocesssecurity: bool, lpcontext: *mut super::LPVOID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn BackupWrite(hfile : super::HANDLE, lpbuffer : super::LPBYTE, nnumberofbytestowrite : super::DWORD, lpnumberofbyteswritten : super::LPDWORD, babort : windows_core::BOOL, bprocesssecurity : windows_core::BOOL, lpcontext : *mut super::LPVOID) -> windows_core::BOOL);
    unsafe { BackupWrite(hfile, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap(), lpnumberofbyteswritten as _, babort.into(), bprocesssecurity.into(), lpcontext as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn BeginUpdateResourceA<P0>(pfilename: P0, bdeleteexistingresources: bool) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn BeginUpdateResourceA(pfilename : windows_core::PCSTR, bdeleteexistingresources : windows_core::BOOL) -> super::HANDLE);
    unsafe { BeginUpdateResourceA(pfilename.param().abi(), bdeleteexistingresources.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn BeginUpdateResourceW<P0>(pfilename: P0, bdeleteexistingresources: bool) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn BeginUpdateResourceW(pfilename : windows_core::PCWSTR, bdeleteexistingresources : windows_core::BOOL) -> super::HANDLE);
    unsafe { BeginUpdateResourceW(pfilename.param().abi(), bdeleteexistingresources.into()) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn BindIoCompletionCallback(filehandle: super::HANDLE, function: super::LPOVERLAPPED_COMPLETION_ROUTINE, flags: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn BindIoCompletionCallback(filehandle : super::HANDLE, function : super::LPOVERLAPPED_COMPLETION_ROUTINE, flags : super::ULONG) -> windows_core::BOOL);
    unsafe { BindIoCompletionCallback(filehandle, function, flags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BuildCommDCBA<P0>(lpdef: P0, lpdcb: LPDCB) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn BuildCommDCBA(lpdef : windows_core::PCSTR, lpdcb : LPDCB) -> windows_core::BOOL);
    unsafe { BuildCommDCBA(lpdef.param().abi(), lpdcb as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BuildCommDCBAndTimeoutsA<P0>(lpdef: P0, lpdcb: LPDCB, lpcommtimeouts: LPCOMMTIMEOUTS) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn BuildCommDCBAndTimeoutsA(lpdef : windows_core::PCSTR, lpdcb : LPDCB, lpcommtimeouts : LPCOMMTIMEOUTS) -> windows_core::BOOL);
    unsafe { BuildCommDCBAndTimeoutsA(lpdef.param().abi(), lpdcb as _, lpcommtimeouts as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BuildCommDCBAndTimeoutsW<P0>(lpdef: P0, lpdcb: LPDCB, lpcommtimeouts: LPCOMMTIMEOUTS) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn BuildCommDCBAndTimeoutsW(lpdef : windows_core::PCWSTR, lpdcb : LPDCB, lpcommtimeouts : LPCOMMTIMEOUTS) -> windows_core::BOOL);
    unsafe { BuildCommDCBAndTimeoutsW(lpdef.param().abi(), lpdcb as _, lpcommtimeouts as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn BuildCommDCBW<P0>(lpdef: P0, lpdcb: LPDCB) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn BuildCommDCBW(lpdef : windows_core::PCWSTR, lpdcb : LPDCB) -> windows_core::BOOL);
    unsafe { BuildCommDCBW(lpdef.param().abi(), lpdcb as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CallNamedPipeA<P0>(lpnamedpipename: P0, lpinbuffer: Option<super::LPVOID>, ninbuffersize: super::DWORD, lpoutbuffer: Option<super::LPVOID>, noutbuffersize: super::DWORD, lpbytesread: super::LPDWORD, ntimeout: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CallNamedPipeA(lpnamedpipename : windows_core::PCSTR, lpinbuffer : super::LPVOID, ninbuffersize : super::DWORD, lpoutbuffer : super::LPVOID, noutbuffersize : super::DWORD, lpbytesread : super::LPDWORD, ntimeout : super::DWORD) -> windows_core::BOOL);
    unsafe { CallNamedPipeA(lpnamedpipename.param().abi(), lpinbuffer.unwrap_or(core::mem::zeroed()) as _, ninbuffersize, lpoutbuffer.unwrap_or(core::mem::zeroed()) as _, noutbuffersize, lpbytesread as _, ntimeout) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CancelDeviceWakeupRequest(hdevice: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CancelDeviceWakeupRequest(hdevice : super::HANDLE) -> windows_core::BOOL);
    unsafe { CancelDeviceWakeupRequest(hdevice) }
}
#[must_use]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CancelTimerQueueTimer(timerqueue: Option<super::HANDLE>, timer: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CancelTimerQueueTimer(timerqueue : super::HANDLE, timer : super::HANDLE) -> windows_core::BOOL);
    unsafe { CancelTimerQueueTimer(timerqueue.unwrap_or(core::mem::zeroed()) as _, timer) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CaptureStackBackTrace(framestoskip: super::DWORD, framestocapture: super::DWORD, backtrace: *mut super::PVOID, backtracehash: Option<super::PDWORD>) -> super::WORD {
    windows_core::link!("kernel32.dll" "system" "RtlCaptureStackBackTrace" fn CaptureStackBackTrace(framestoskip : super::DWORD, framestocapture : super::DWORD, backtrace : *mut super::PVOID, backtracehash : super::PDWORD) -> super::WORD);
    unsafe { CaptureStackBackTrace(framestoskip, framestocapture, backtrace as _, backtracehash.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CheckNameLegalDOS8Dot3A<P0>(lpname: P0, lpoemname: Option<windows_core::PSTR>, oemnamesize: super::DWORD, pbnamecontainsspaces: Option<super::PBOOL>, pbnamelegal: super::PBOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CheckNameLegalDOS8Dot3A(lpname : windows_core::PCSTR, lpoemname : windows_core::PSTR, oemnamesize : super::DWORD, pbnamecontainsspaces : super::PBOOL, pbnamelegal : super::PBOOL) -> windows_core::BOOL);
    unsafe { CheckNameLegalDOS8Dot3A(lpname.param().abi(), lpoemname.unwrap_or(core::mem::zeroed()) as _, oemnamesize, pbnamecontainsspaces.unwrap_or(core::mem::zeroed()) as _, pbnamelegal as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CheckNameLegalDOS8Dot3W<P0>(lpname: P0, lpoemname: Option<windows_core::PSTR>, oemnamesize: super::DWORD, pbnamecontainsspaces: Option<super::PBOOL>, pbnamelegal: super::PBOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CheckNameLegalDOS8Dot3W(lpname : windows_core::PCWSTR, lpoemname : windows_core::PSTR, oemnamesize : super::DWORD, pbnamecontainsspaces : super::PBOOL, pbnamelegal : super::PBOOL) -> windows_core::BOOL);
    unsafe { CheckNameLegalDOS8Dot3W(lpname.param().abi(), lpoemname.unwrap_or(core::mem::zeroed()) as _, oemnamesize, pbnamecontainsspaces.unwrap_or(core::mem::zeroed()) as _, pbnamelegal as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ClearCommBreak(hfile: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ClearCommBreak(hfile : super::HANDLE) -> windows_core::BOOL);
    unsafe { ClearCommBreak(hfile) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ClearCommError(hfile: super::HANDLE, lperrors: Option<super::LPDWORD>, lpstat: Option<LPCOMSTAT>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ClearCommError(hfile : super::HANDLE, lperrors : super::LPDWORD, lpstat : LPCOMSTAT) -> windows_core::BOOL);
    unsafe { ClearCommError(hfile, lperrors.unwrap_or(core::mem::zeroed()) as _, lpstat.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ClearEventLogA<P1>(heventlog: super::HANDLE, lpbackupfilename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ClearEventLogA(heventlog : super::HANDLE, lpbackupfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { ClearEventLogA(heventlog, lpbackupfilename.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ClearEventLogW<P1>(heventlog: super::HANDLE, lpbackupfilename: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ClearEventLogW(heventlog : super::HANDLE, lpbackupfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { ClearEventLogW(heventlog, lpbackupfilename.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CloseEncryptedFileRaw(pvcontext: super::PVOID) {
    windows_core::link!("advapi32.dll" "system" fn CloseEncryptedFileRaw(pvcontext : super::PVOID));
    unsafe { CloseEncryptedFileRaw(pvcontext) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn CloseEventLog(heventlog: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn CloseEventLog(heventlog : super::HANDLE) -> windows_core::BOOL);
    unsafe { CloseEventLog(heventlog) }
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn CommConfigDialogA<P0>(lpszname: P0, hwnd: Option<super::HWND>, lpcc: LPCOMMCONFIG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CommConfigDialogA(lpszname : windows_core::PCSTR, hwnd : super::HWND, lpcc : LPCOMMCONFIG) -> windows_core::BOOL);
    unsafe { CommConfigDialogA(lpszname.param().abi(), hwnd.unwrap_or(core::mem::zeroed()) as _, lpcc as _) }
}
#[cfg(all(feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn CommConfigDialogW<P0>(lpszname: P0, hwnd: Option<super::HWND>, lpcc: LPCOMMCONFIG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CommConfigDialogW(lpszname : windows_core::PCWSTR, hwnd : super::HWND, lpcc : LPCOMMCONFIG) -> windows_core::BOOL);
    unsafe { CommConfigDialogW(lpszname.param().abi(), hwnd.unwrap_or(core::mem::zeroed()) as _, lpcc as _) }
}
#[inline]
pub unsafe fn ConvertFiberToThread() -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ConvertFiberToThread() -> windows_core::BOOL);
    unsafe { ConvertFiberToThread() }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ConvertThreadToFiber(lpparameter: Option<super::LPVOID>) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn ConvertThreadToFiber(lpparameter : super::LPVOID) -> super::LPVOID);
    unsafe { ConvertThreadToFiber(lpparameter.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn ConvertThreadToFiberEx(lpparameter: Option<super::LPVOID>, dwflags: super::DWORD) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn ConvertThreadToFiberEx(lpparameter : super::LPVOID, dwflags : super::DWORD) -> super::LPVOID);
    unsafe { ConvertThreadToFiberEx(lpparameter.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[must_use]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CopyContext(destination: super::PCONTEXT, contextflags: super::DWORD, source: super::PCONTEXT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CopyContext(destination : super::PCONTEXT, contextflags : super::DWORD, source : super::PCONTEXT) -> windows_core::BOOL);
    unsafe { CopyContext(destination as _, contextflags, source) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CopyFile2<P0, P1>(pwszexistingfilename: P0, pwsznewfilename: P1, pextendedparameters: Option<*const COPYFILE2_EXTENDED_PARAMETERS>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CopyFile2(pwszexistingfilename : windows_core::PCWSTR, pwsznewfilename : windows_core::PCWSTR, pextendedparameters : *const COPYFILE2_EXTENDED_PARAMETERS) -> windows_core::HRESULT);
    unsafe { CopyFile2(pwszexistingfilename.param().abi(), pwsznewfilename.param().abi(), pextendedparameters.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn CopyFileA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, bfailifexists: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CopyFileA(lpexistingfilename : windows_core::PCSTR, lpnewfilename : windows_core::PCSTR, bfailifexists : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CopyFileA(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), bfailifexists.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CopyFileExA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<super::LPVOID>, pbcancel: Option<super::LPBOOL>, dwcopyflags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CopyFileExA(lpexistingfilename : windows_core::PCSTR, lpnewfilename : windows_core::PCSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : super::LPVOID, pbcancel : super::LPBOOL, dwcopyflags : super::DWORD) -> windows_core::BOOL);
    unsafe { CopyFileExA(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, lpdata.unwrap_or(core::mem::zeroed()) as _, pbcancel.unwrap_or(core::mem::zeroed()) as _, dwcopyflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CopyFileExW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<super::LPVOID>, pbcancel: Option<super::LPBOOL>, dwcopyflags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CopyFileExW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : super::LPVOID, pbcancel : super::LPBOOL, dwcopyflags : super::DWORD) -> windows_core::BOOL);
    unsafe { CopyFileExW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, lpdata.unwrap_or(core::mem::zeroed()) as _, pbcancel.unwrap_or(core::mem::zeroed()) as _, dwcopyflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CopyFileTransactedA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<super::LPVOID>, pbcancel: Option<super::LPBOOL>, dwcopyflags: super::DWORD, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CopyFileTransactedA(lpexistingfilename : windows_core::PCSTR, lpnewfilename : windows_core::PCSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : super::LPVOID, pbcancel : super::LPBOOL, dwcopyflags : super::DWORD, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { CopyFileTransactedA(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, lpdata.unwrap_or(core::mem::zeroed()) as _, pbcancel.unwrap_or(core::mem::zeroed()) as _, dwcopyflags, htransaction) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CopyFileTransactedW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<super::LPVOID>, pbcancel: Option<super::LPBOOL>, dwcopyflags: super::DWORD, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CopyFileTransactedW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : super::LPVOID, pbcancel : super::LPBOOL, dwcopyflags : super::DWORD, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { CopyFileTransactedW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, lpdata.unwrap_or(core::mem::zeroed()) as _, pbcancel.unwrap_or(core::mem::zeroed()) as _, dwcopyflags, htransaction) }
}
#[inline]
pub unsafe fn CopyFileW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, bfailifexists: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CopyFileW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, bfailifexists : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { CopyFileW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), bfailifexists.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateActCtxA(pactctx: PCACTCTXA) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateActCtxA(pactctx : PCACTCTXA) -> super::HANDLE);
    unsafe { CreateActCtxA(pactctx) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateActCtxW(pactctx: PCACTCTXW) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn CreateActCtxW(pactctx : PCACTCTXW) -> super::HANDLE);
    unsafe { CreateActCtxW(pactctx) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateBoundaryDescriptorA<P0>(name: P0, flags: super::ULONG) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateBoundaryDescriptorA(name : windows_core::PCSTR, flags : super::ULONG) -> super::HANDLE);
    unsafe { CreateBoundaryDescriptorA(name.param().abi(), flags) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CreateDirectoryExA<P0, P1>(lptemplatedirectory: P0, lpnewdirectory: P1, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateDirectoryExA(lptemplatedirectory : windows_core::PCSTR, lpnewdirectory : windows_core::PCSTR, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> windows_core::BOOL);
    unsafe { CreateDirectoryExA(lptemplatedirectory.param().abi(), lpnewdirectory.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CreateDirectoryExW<P0, P1>(lptemplatedirectory: P0, lpnewdirectory: P1, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateDirectoryExW(lptemplatedirectory : windows_core::PCWSTR, lpnewdirectory : windows_core::PCWSTR, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> windows_core::BOOL);
    unsafe { CreateDirectoryExW(lptemplatedirectory.param().abi(), lpnewdirectory.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateDirectoryTransactedA<P0, P1>(lptemplatedirectory: P0, lpnewdirectory: P1, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateDirectoryTransactedA(lptemplatedirectory : windows_core::PCSTR, lpnewdirectory : windows_core::PCSTR, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { CreateDirectoryTransactedA(lptemplatedirectory.param().abi(), lpnewdirectory.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, htransaction) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateDirectoryTransactedW<P0, P1>(lptemplatedirectory: P0, lpnewdirectory: P1, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateDirectoryTransactedW(lptemplatedirectory : windows_core::PCWSTR, lpnewdirectory : windows_core::PCWSTR, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { CreateDirectoryTransactedW(lptemplatedirectory.param().abi(), lpnewdirectory.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, htransaction) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CreateFiber(dwstacksize: super::SIZE_T, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: Option<super::LPVOID>) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn CreateFiber(dwstacksize : super::SIZE_T, lpstartaddress : LPFIBER_START_ROUTINE, lpparameter : super::LPVOID) -> super::LPVOID);
    unsafe { CreateFiber(dwstacksize, lpstartaddress, lpparameter.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CreateFiberEx(dwstackcommitsize: super::SIZE_T, dwstackreservesize: super::SIZE_T, dwflags: super::DWORD, lpstartaddress: LPFIBER_START_ROUTINE, lpparameter: Option<super::LPVOID>) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn CreateFiberEx(dwstackcommitsize : super::SIZE_T, dwstackreservesize : super::SIZE_T, dwflags : super::DWORD, lpstartaddress : LPFIBER_START_ROUTINE, lpparameter : super::LPVOID) -> super::LPVOID);
    unsafe { CreateFiberEx(dwstackcommitsize, dwstackreservesize, dwflags, lpstartaddress, lpparameter.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileMappingA<P5>(hfile: super::HANDLE, lpfilemappingattributes: Option<super::LPSECURITY_ATTRIBUTES>, flprotect: super::DWORD, dwmaximumsizehigh: super::DWORD, dwmaximumsizelow: super::DWORD, lpname: P5) -> super::HANDLE
where
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileMappingA(hfile : super::HANDLE, lpfilemappingattributes : super::LPSECURITY_ATTRIBUTES, flprotect : super::DWORD, dwmaximumsizehigh : super::DWORD, dwmaximumsizelow : super::DWORD, lpname : windows_core::PCSTR) -> super::HANDLE);
    unsafe { CreateFileMappingA(hfile, lpfilemappingattributes.unwrap_or(core::mem::zeroed()) as _, flprotect, dwmaximumsizehigh, dwmaximumsizelow, lpname.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileMappingNumaA<P5>(hfile: super::HANDLE, lpfilemappingattributes: Option<super::LPSECURITY_ATTRIBUTES>, flprotect: super::DWORD, dwmaximumsizehigh: super::DWORD, dwmaximumsizelow: super::DWORD, lpname: P5, nndpreferred: super::DWORD) -> super::HANDLE
where
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileMappingNumaA(hfile : super::HANDLE, lpfilemappingattributes : super::LPSECURITY_ATTRIBUTES, flprotect : super::DWORD, dwmaximumsizehigh : super::DWORD, dwmaximumsizelow : super::DWORD, lpname : windows_core::PCSTR, nndpreferred : super::DWORD) -> super::HANDLE);
    unsafe { CreateFileMappingNumaA(hfile, lpfilemappingattributes.unwrap_or(core::mem::zeroed()) as _, flprotect, dwmaximumsizehigh, dwmaximumsizelow, lpname.param().abi(), nndpreferred) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileTransactedA<P0>(lpfilename: P0, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>, dwcreationdisposition: super::DWORD, dwflagsandattributes: super::DWORD, htemplatefile: Option<super::HANDLE>, htransaction: super::HANDLE, pusminiversion: Option<super::PUSHORT>, lpextendedparameter: Option<super::PVOID>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileTransactedA(lpfilename : windows_core::PCSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES, dwcreationdisposition : super::DWORD, dwflagsandattributes : super::DWORD, htemplatefile : super::HANDLE, htransaction : super::HANDLE, pusminiversion : super::PUSHORT, lpextendedparameter : super::PVOID) -> super::HANDLE);
    unsafe { CreateFileTransactedA(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, dwcreationdisposition, dwflagsandattributes, htemplatefile.unwrap_or(core::mem::zeroed()) as _, htransaction, pusminiversion.unwrap_or(core::mem::zeroed()) as _, lpextendedparameter.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateFileTransactedW<P0>(lpfilename: P0, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>, dwcreationdisposition: super::DWORD, dwflagsandattributes: super::DWORD, htemplatefile: Option<super::HANDLE>, htransaction: super::HANDLE, pusminiversion: Option<super::PUSHORT>, lpextendedparameter: Option<super::PVOID>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateFileTransactedW(lpfilename : windows_core::PCWSTR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES, dwcreationdisposition : super::DWORD, dwflagsandattributes : super::DWORD, htemplatefile : super::HANDLE, htransaction : super::HANDLE, pusminiversion : super::PUSHORT, lpextendedparameter : super::PVOID) -> super::HANDLE);
    unsafe { CreateFileTransactedW(lpfilename.param().abi(), dwdesiredaccess, dwsharemode, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, dwcreationdisposition, dwflagsandattributes, htemplatefile.unwrap_or(core::mem::zeroed()) as _, htransaction, pusminiversion.unwrap_or(core::mem::zeroed()) as _, lpextendedparameter.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CreateHardLinkA<P0, P1>(lpfilename: P0, lpexistingfilename: P1, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateHardLinkA(lpfilename : windows_core::PCSTR, lpexistingfilename : windows_core::PCSTR, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> windows_core::BOOL);
    unsafe { CreateHardLinkA(lpfilename.param().abi(), lpexistingfilename.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateHardLinkTransactedA<P0, P1>(lpfilename: P0, lpexistingfilename: P1, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateHardLinkTransactedA(lpfilename : windows_core::PCSTR, lpexistingfilename : windows_core::PCSTR, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { CreateHardLinkTransactedA(lpfilename.param().abi(), lpexistingfilename.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, htransaction) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateHardLinkTransactedW<P0, P1>(lpfilename: P0, lpexistingfilename: P1, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateHardLinkTransactedW(lpfilename : windows_core::PCWSTR, lpexistingfilename : windows_core::PCWSTR, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { CreateHardLinkTransactedW(lpfilename.param().abi(), lpexistingfilename.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, htransaction) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn CreateHardLinkW<P0, P1>(lpfilename: P0, lpexistingfilename: P1, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateHardLinkW(lpfilename : windows_core::PCWSTR, lpexistingfilename : windows_core::PCWSTR, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> windows_core::BOOL);
    unsafe { CreateHardLinkW(lpfilename.param().abi(), lpexistingfilename.param().abi(), lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateJobObjectA<P1>(lpjobattributes: Option<super::LPSECURITY_ATTRIBUTES>, lpname: P1) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateJobObjectA(lpjobattributes : super::LPSECURITY_ATTRIBUTES, lpname : windows_core::PCSTR) -> super::HANDLE);
    unsafe { CreateJobObjectA(lpjobattributes.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateJobSet(userjobset: &[super::JOB_SET_ARRAY], flags: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CreateJobSet(numjob : super::ULONG, userjobset : super::PJOB_SET_ARRAY, flags : super::ULONG) -> windows_core::BOOL);
    unsafe { CreateJobSet(userjobset.len().try_into().unwrap(), core::mem::transmute(userjobset.as_ptr()), flags) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateMailslotA<P0>(lpname: P0, nmaxmessagesize: super::DWORD, lreadtimeout: super::DWORD, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateMailslotA(lpname : windows_core::PCSTR, nmaxmessagesize : super::DWORD, lreadtimeout : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> super::HANDLE);
    unsafe { CreateMailslotA(lpname.param().abi(), nmaxmessagesize, lreadtimeout, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateMailslotW<P0>(lpname: P0, nmaxmessagesize: super::DWORD, lreadtimeout: super::DWORD, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateMailslotW(lpname : windows_core::PCWSTR, nmaxmessagesize : super::DWORD, lreadtimeout : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> super::HANDLE);
    unsafe { CreateMailslotW(lpname.param().abi(), nmaxmessagesize, lreadtimeout, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateNamedPipeA<P0>(lpname: P0, dwopenmode: super::DWORD, dwpipemode: super::DWORD, nmaxinstances: super::DWORD, noutbuffersize: super::DWORD, ninbuffersize: super::DWORD, ndefaulttimeout: super::DWORD, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateNamedPipeA(lpname : windows_core::PCSTR, dwopenmode : super::DWORD, dwpipemode : super::DWORD, nmaxinstances : super::DWORD, noutbuffersize : super::DWORD, ninbuffersize : super::DWORD, ndefaulttimeout : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES) -> super::HANDLE);
    unsafe { CreateNamedPipeA(lpname.param().abi(), dwopenmode, dwpipemode, nmaxinstances, noutbuffersize, ninbuffersize, ndefaulttimeout, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreatePrivateNamespaceA<P2>(lpprivatenamespaceattributes: Option<super::LPSECURITY_ATTRIBUTES>, lpboundarydescriptor: super::LPVOID, lpaliasprefix: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreatePrivateNamespaceA(lpprivatenamespaceattributes : super::LPSECURITY_ATTRIBUTES, lpboundarydescriptor : super::LPVOID, lpaliasprefix : windows_core::PCSTR) -> super::HANDLE);
    unsafe { CreatePrivateNamespaceA(lpprivatenamespaceattributes.unwrap_or(core::mem::zeroed()) as _, lpboundarydescriptor, lpaliasprefix.param().abi()) }
}
#[must_use]
#[cfg(all(feature = "minwindef", feature = "processthreadsapi", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateProcessWithLogonW<P0, P1, P2, P4, P8>(lpusername: P0, lpdomain: P1, lppassword: P2, dwlogonflags: super::DWORD, lpapplicationname: P4, lpcommandline: Option<windows_core::PWSTR>, dwcreationflags: super::DWORD, lpenvironment: Option<super::LPVOID>, lpcurrentdirectory: P8, lpstartupinfo: super::LPSTARTUPINFOW, lpprocessinformation: super::LPPROCESS_INFORMATION) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
    P8: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CreateProcessWithLogonW(lpusername : windows_core::PCWSTR, lpdomain : windows_core::PCWSTR, lppassword : windows_core::PCWSTR, dwlogonflags : super::DWORD, lpapplicationname : windows_core::PCWSTR, lpcommandline : windows_core::PWSTR, dwcreationflags : super::DWORD, lpenvironment : super::LPVOID, lpcurrentdirectory : windows_core::PCWSTR, lpstartupinfo : super::LPSTARTUPINFOW, lpprocessinformation : super::LPPROCESS_INFORMATION) -> windows_core::BOOL);
    unsafe { CreateProcessWithLogonW(lpusername.param().abi(), lpdomain.param().abi(), lppassword.param().abi(), dwlogonflags, lpapplicationname.param().abi(), lpcommandline.unwrap_or(core::mem::zeroed()) as _, dwcreationflags, lpenvironment.unwrap_or(core::mem::zeroed()) as _, lpcurrentdirectory.param().abi(), lpstartupinfo, lpprocessinformation as _) }
}
#[must_use]
#[cfg(all(feature = "minwindef", feature = "processthreadsapi", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateProcessWithTokenW<P2, P6>(htoken: super::HANDLE, dwlogonflags: super::DWORD, lpapplicationname: P2, lpcommandline: Option<windows_core::PWSTR>, dwcreationflags: super::DWORD, lpenvironment: Option<super::LPVOID>, lpcurrentdirectory: P6, lpstartupinfo: super::LPSTARTUPINFOW, lpprocessinformation: super::LPPROCESS_INFORMATION) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P6: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn CreateProcessWithTokenW(htoken : super::HANDLE, dwlogonflags : super::DWORD, lpapplicationname : windows_core::PCWSTR, lpcommandline : windows_core::PWSTR, dwcreationflags : super::DWORD, lpenvironment : super::LPVOID, lpcurrentdirectory : windows_core::PCWSTR, lpstartupinfo : super::LPSTARTUPINFOW, lpprocessinformation : super::LPPROCESS_INFORMATION) -> windows_core::BOOL);
    unsafe { CreateProcessWithTokenW(htoken, dwlogonflags, lpapplicationname.param().abi(), lpcommandline.unwrap_or(core::mem::zeroed()) as _, dwcreationflags, lpenvironment.unwrap_or(core::mem::zeroed()) as _, lpcurrentdirectory.param().abi(), lpstartupinfo, lpprocessinformation as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateSemaphoreA<P3>(lpsemaphoreattributes: Option<super::LPSECURITY_ATTRIBUTES>, linitialcount: super::LONG, lmaximumcount: super::LONG, lpname: P3) -> super::HANDLE
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateSemaphoreA(lpsemaphoreattributes : super::LPSECURITY_ATTRIBUTES, linitialcount : super::LONG, lmaximumcount : super::LONG, lpname : windows_core::PCSTR) -> super::HANDLE);
    unsafe { CreateSemaphoreA(lpsemaphoreattributes.unwrap_or(core::mem::zeroed()) as _, linitialcount, lmaximumcount, lpname.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateSemaphoreExA<P3>(lpsemaphoreattributes: Option<super::LPSECURITY_ATTRIBUTES>, linitialcount: super::LONG, lmaximumcount: super::LONG, lpname: P3, dwflags: Option<super::DWORD>, dwdesiredaccess: super::DWORD) -> super::HANDLE
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateSemaphoreExA(lpsemaphoreattributes : super::LPSECURITY_ATTRIBUTES, linitialcount : super::LONG, lmaximumcount : super::LONG, lpname : windows_core::PCSTR, dwflags : super::DWORD, dwdesiredaccess : super::DWORD) -> super::HANDLE);
    unsafe { CreateSemaphoreExA(lpsemaphoreattributes.unwrap_or(core::mem::zeroed()) as _, linitialcount, lmaximumcount, lpname.param().abi(), dwflags.unwrap_or(core::mem::zeroed()) as _, dwdesiredaccess) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CreateSymbolicLinkA<P0, P1>(lpsymlinkfilename: P0, lptargetfilename: P1, dwflags: super::DWORD) -> super::BOOLEAN
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateSymbolicLinkA(lpsymlinkfilename : windows_core::PCSTR, lptargetfilename : windows_core::PCSTR, dwflags : super::DWORD) -> super::BOOLEAN);
    unsafe { CreateSymbolicLinkA(lpsymlinkfilename.param().abi(), lptargetfilename.param().abi(), dwflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateSymbolicLinkTransactedA<P0, P1>(lpsymlinkfilename: P0, lptargetfilename: P1, dwflags: super::DWORD, htransaction: super::HANDLE) -> super::BOOLEAN
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateSymbolicLinkTransactedA(lpsymlinkfilename : windows_core::PCSTR, lptargetfilename : windows_core::PCSTR, dwflags : super::DWORD, htransaction : super::HANDLE) -> super::BOOLEAN);
    unsafe { CreateSymbolicLinkTransactedA(lpsymlinkfilename.param().abi(), lptargetfilename.param().abi(), dwflags, htransaction) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateSymbolicLinkTransactedW<P0, P1>(lpsymlinkfilename: P0, lptargetfilename: P1, dwflags: super::DWORD, htransaction: super::HANDLE) -> super::BOOLEAN
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateSymbolicLinkTransactedW(lpsymlinkfilename : windows_core::PCWSTR, lptargetfilename : windows_core::PCWSTR, dwflags : super::DWORD, htransaction : super::HANDLE) -> super::BOOLEAN);
    unsafe { CreateSymbolicLinkTransactedW(lpsymlinkfilename.param().abi(), lptargetfilename.param().abi(), dwflags, htransaction) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn CreateSymbolicLinkW<P0, P1>(lpsymlinkfilename: P0, lptargetfilename: P1, dwflags: super::DWORD) -> super::BOOLEAN
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateSymbolicLinkW(lpsymlinkfilename : windows_core::PCWSTR, lptargetfilename : windows_core::PCWSTR, dwflags : super::DWORD) -> super::BOOLEAN);
    unsafe { CreateSymbolicLinkW(lpsymlinkfilename.param().abi(), lptargetfilename.param().abi(), dwflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateTapePartition(hdevice: super::HANDLE, dwpartitionmethod: super::DWORD, dwcount: super::DWORD, dwsize: super::DWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn CreateTapePartition(hdevice : super::HANDLE, dwpartitionmethod : super::DWORD, dwcount : super::DWORD, dwsize : super::DWORD) -> super::DWORD);
    unsafe { CreateTapePartition(hdevice, dwpartitionmethod, dwcount, dwsize) }
}
#[must_use]
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn CreateUmsCompletionList(umscompletionlist: *mut PUMS_COMPLETION_LIST) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CreateUmsCompletionList(umscompletionlist : *mut PUMS_COMPLETION_LIST) -> windows_core::BOOL);
    unsafe { CreateUmsCompletionList(umscompletionlist as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn CreateUmsThreadContext(lpumsthread: *mut PUMS_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn CreateUmsThreadContext(lpumsthread : *mut PUMS_CONTEXT) -> windows_core::BOOL);
    unsafe { CreateUmsThreadContext(lpumsthread as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateWaitableTimerA<P2>(lptimerattributes: Option<super::LPSECURITY_ATTRIBUTES>, bmanualreset: bool, lptimername: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateWaitableTimerA(lptimerattributes : super::LPSECURITY_ATTRIBUTES, bmanualreset : windows_core::BOOL, lptimername : windows_core::PCSTR) -> super::HANDLE);
    unsafe { CreateWaitableTimerA(lptimerattributes.unwrap_or(core::mem::zeroed()) as _, bmanualreset.into(), lptimername.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn CreateWaitableTimerExA<P1>(lptimerattributes: Option<super::LPSECURITY_ATTRIBUTES>, lptimername: P1, dwflags: super::DWORD, dwdesiredaccess: super::DWORD) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn CreateWaitableTimerExA(lptimerattributes : super::LPSECURITY_ATTRIBUTES, lptimername : windows_core::PCSTR, dwflags : super::DWORD, dwdesiredaccess : super::DWORD) -> super::HANDLE);
    unsafe { CreateWaitableTimerExA(lptimerattributes.unwrap_or(core::mem::zeroed()) as _, lptimername.param().abi(), dwflags, dwdesiredaccess) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DeactivateActCtx(dwflags: super::DWORD, ulcookie: super::ULONG_PTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DeactivateActCtx(dwflags : super::DWORD, ulcookie : super::ULONG_PTR) -> windows_core::BOOL);
    unsafe { DeactivateActCtx(dwflags, ulcookie) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DebugBreakProcess(process: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DebugBreakProcess(process : super::HANDLE) -> windows_core::BOOL);
    unsafe { DebugBreakProcess(process) }
}
#[inline]
pub unsafe fn DebugSetProcessKillOnExit(killonexit: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DebugSetProcessKillOnExit(killonexit : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { DebugSetProcessKillOnExit(killonexit.into()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DecryptFileA<P0>(lpfilename: P0, dwreserved: Option<super::DWORD>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn DecryptFileA(lpfilename : windows_core::PCSTR, dwreserved : super::DWORD) -> windows_core::BOOL);
    unsafe { DecryptFileA(lpfilename.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DecryptFileW<P0>(lpfilename: P0, dwreserved: Option<super::DWORD>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn DecryptFileW(lpfilename : windows_core::PCWSTR, dwreserved : super::DWORD) -> windows_core::BOOL);
    unsafe { DecryptFileW(lpfilename.param().abi(), dwreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DefineDosDeviceA<P1, P2>(dwflags: super::DWORD, lpdevicename: P1, lptargetpath: P2) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DefineDosDeviceA(dwflags : super::DWORD, lpdevicename : windows_core::PCSTR, lptargetpath : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DefineDosDeviceA(dwflags, lpdevicename.param().abi(), lptargetpath.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DeleteAtom(natom: super::ATOM) -> super::ATOM {
    windows_core::link!("kernel32.dll" "system" fn DeleteAtom(natom : super::ATOM) -> super::ATOM);
    unsafe { DeleteAtom(natom) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DeleteFiber(lpfiber: super::LPVOID) {
    windows_core::link!("kernel32.dll" "system" fn DeleteFiber(lpfiber : super::LPVOID));
    unsafe { DeleteFiber(lpfiber) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteFileTransactedA<P0>(lpfilename: P0, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DeleteFileTransactedA(lpfilename : windows_core::PCSTR, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { DeleteFileTransactedA(lpfilename.param().abi(), htransaction) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeleteFileTransactedW<P0>(lpfilename: P0, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DeleteFileTransactedW(lpfilename : windows_core::PCWSTR, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { DeleteFileTransactedW(lpfilename.param().abi(), htransaction) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn DeleteUmsCompletionList(umscompletionlist: PUMS_COMPLETION_LIST) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DeleteUmsCompletionList(umscompletionlist : PUMS_COMPLETION_LIST) -> windows_core::BOOL);
    unsafe { DeleteUmsCompletionList(umscompletionlist) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn DeleteUmsThreadContext(umsthread: PUMS_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DeleteUmsThreadContext(umsthread : PUMS_CONTEXT) -> windows_core::BOOL);
    unsafe { DeleteUmsThreadContext(umsthread) }
}
#[inline]
pub unsafe fn DeleteVolumeMountPointA<P0>(lpszvolumemountpoint: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DeleteVolumeMountPointA(lpszvolumemountpoint : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { DeleteVolumeMountPointA(lpszvolumemountpoint.param().abi()) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn DequeueUmsCompletionListItems(umscompletionlist: PUMS_COMPLETION_LIST, waittimeout: super::DWORD, umsthreadlist: *mut PUMS_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DequeueUmsCompletionListItems(umscompletionlist : PUMS_COMPLETION_LIST, waittimeout : super::DWORD, umsthreadlist : *mut PUMS_CONTEXT) -> windows_core::BOOL);
    unsafe { DequeueUmsCompletionListItems(umscompletionlist, waittimeout, umsthreadlist as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn DeregisterEventSource(heventlog: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn DeregisterEventSource(heventlog : super::HANDLE) -> windows_core::BOOL);
    unsafe { DeregisterEventSource(heventlog) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn DisableThreadProfiling(performancedatahandle: super::HANDLE) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn DisableThreadProfiling(performancedatahandle : super::HANDLE) -> super::DWORD);
    unsafe { DisableThreadProfiling(performancedatahandle) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsHostnameToComputerNameA<P0>(hostname: P0, computername: Option<windows_core::PSTR>, nsize: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DnsHostnameToComputerNameA(hostname : windows_core::PCSTR, computername : windows_core::PSTR, nsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { DnsHostnameToComputerNameA(hostname.param().abi(), computername.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn DnsHostnameToComputerNameW<P0>(hostname: P0, computername: Option<windows_core::PWSTR>, nsize: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn DnsHostnameToComputerNameW(hostname : windows_core::PCWSTR, computername : windows_core::PWSTR, nsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { DnsHostnameToComputerNameW(hostname.param().abi(), computername.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn DosDateTimeToFileTime(wfatdate: super::WORD, wfattime: super::WORD, lpfiletime: super::LPFILETIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn DosDateTimeToFileTime(wfatdate : super::WORD, wfattime : super::WORD, lpfiletime : super::LPFILETIME) -> windows_core::BOOL);
    unsafe { DosDateTimeToFileTime(wfatdate, wfattime, lpfiletime as _) }
}
#[must_use]
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn EnableProcessOptionalXStateFeatures(features: super::DWORD64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EnableProcessOptionalXStateFeatures(features : super::DWORD64) -> windows_core::BOOL);
    unsafe { EnableProcessOptionalXStateFeatures(features) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnableThreadProfiling(threadhandle: super::HANDLE, flags: super::DWORD, hardwarecounters: super::DWORD64, performancedatahandle: *mut super::HANDLE) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn EnableThreadProfiling(threadhandle : super::HANDLE, flags : super::DWORD, hardwarecounters : super::DWORD64, performancedatahandle : *mut super::HANDLE) -> super::DWORD);
    unsafe { EnableThreadProfiling(threadhandle, flags, hardwarecounters, performancedatahandle as _) }
}
#[inline]
pub unsafe fn EncryptFileA<P0>(lpfilename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn EncryptFileA(lpfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { EncryptFileA(lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn EncryptFileW<P0>(lpfilename: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn EncryptFileW(lpfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { EncryptFileW(lpfilename.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EndUpdateResourceA(hupdate: super::HANDLE, fdiscard: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EndUpdateResourceA(hupdate : super::HANDLE, fdiscard : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { EndUpdateResourceA(hupdate, fdiscard.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn EndUpdateResourceW(hupdate: super::HANDLE, fdiscard: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EndUpdateResourceW(hupdate : super::HANDLE, fdiscard : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { EndUpdateResourceW(hupdate, fdiscard.into()) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EnterUmsSchedulingMode(schedulerstartupinfo: PUMS_SCHEDULER_STARTUP_INFO) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EnterUmsSchedulingMode(schedulerstartupinfo : PUMS_SCHEDULER_STARTUP_INFO) -> windows_core::BOOL);
    unsafe { EnterUmsSchedulingMode(schedulerstartupinfo) }
}
#[cfg(all(feature = "libloaderapi", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn EnumResourceLanguagesA<P1, P2>(hmodule: Option<super::HMODULE>, lptype: P1, lpname: P2, lpenumfunc: super::ENUMRESLANGPROCA, lparam: super::LONG_PTR) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceLanguagesA(hmodule : super::HMODULE, lptype : windows_core::PCSTR, lpname : windows_core::PCSTR, lpenumfunc : super::ENUMRESLANGPROCA, lparam : super::LONG_PTR) -> windows_core::BOOL);
    unsafe { EnumResourceLanguagesA(hmodule.unwrap_or(core::mem::zeroed()) as _, lptype.param().abi(), lpname.param().abi(), lpenumfunc, lparam) }
}
#[cfg(all(feature = "libloaderapi", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn EnumResourceLanguagesW<P1, P2>(hmodule: Option<super::HMODULE>, lptype: P1, lpname: P2, lpenumfunc: super::ENUMRESLANGPROCW, lparam: super::LONG_PTR) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn EnumResourceLanguagesW(hmodule : super::HMODULE, lptype : windows_core::PCWSTR, lpname : windows_core::PCWSTR, lpenumfunc : super::ENUMRESLANGPROCW, lparam : super::LONG_PTR) -> windows_core::BOOL);
    unsafe { EnumResourceLanguagesW(hmodule.unwrap_or(core::mem::zeroed()) as _, lptype.param().abi(), lpname.param().abi(), lpenumfunc, lparam) }
}
#[cfg(all(feature = "libloaderapi", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn EnumResourceTypesA(hmodule: Option<super::HMODULE>, lpenumfunc: super::ENUMRESTYPEPROCA, lparam: super::LONG_PTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EnumResourceTypesA(hmodule : super::HMODULE, lpenumfunc : super::ENUMRESTYPEPROCA, lparam : super::LONG_PTR) -> windows_core::BOOL);
    unsafe { EnumResourceTypesA(hmodule.unwrap_or(core::mem::zeroed()) as _, lpenumfunc, lparam) }
}
#[cfg(all(feature = "libloaderapi", feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn EnumResourceTypesW(hmodule: Option<super::HMODULE>, lpenumfunc: super::ENUMRESTYPEPROCW, lparam: super::LONG_PTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EnumResourceTypesW(hmodule : super::HMODULE, lpenumfunc : super::ENUMRESTYPEPROCW, lparam : super::LONG_PTR) -> windows_core::BOOL);
    unsafe { EnumResourceTypesW(hmodule.unwrap_or(core::mem::zeroed()) as _, lpenumfunc, lparam) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EraseTape(hdevice: super::HANDLE, dwerasetype: super::DWORD, bimmediate: bool) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn EraseTape(hdevice : super::HANDLE, dwerasetype : super::DWORD, bimmediate : windows_core::BOOL) -> super::DWORD);
    unsafe { EraseTape(hdevice, dwerasetype, bimmediate.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn EscapeCommFunction(hfile: super::HANDLE, dwfunc: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn EscapeCommFunction(hfile : super::HANDLE, dwfunc : super::DWORD) -> windows_core::BOOL);
    unsafe { EscapeCommFunction(hfile, dwfunc) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn ExecuteUmsThread(umsthread: PUMS_CONTEXT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ExecuteUmsThread(umsthread : PUMS_CONTEXT) -> windows_core::BOOL);
    unsafe { ExecuteUmsThread(umsthread as _) }
}
#[inline]
pub unsafe fn FatalExit(exitcode: i32) {
    windows_core::link!("kernel32.dll" "system" fn FatalExit(exitcode : i32));
    unsafe { FatalExit(exitcode) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FileEncryptionStatusA<P0>(lpfilename: P0, lpstatus: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn FileEncryptionStatusA(lpfilename : windows_core::PCSTR, lpstatus : super::LPDWORD) -> windows_core::BOOL);
    unsafe { FileEncryptionStatusA(lpfilename.param().abi(), lpstatus as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FileEncryptionStatusW<P0>(lpfilename: P0, lpstatus: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn FileEncryptionStatusW(lpfilename : windows_core::PCWSTR, lpstatus : super::LPDWORD) -> windows_core::BOOL);
    unsafe { FileEncryptionStatusW(lpfilename.param().abi(), lpstatus as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn FileTimeToDosDateTime(lpfiletime: *const super::FILETIME, lpfatdate: super::LPWORD, lpfattime: super::LPWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FileTimeToDosDateTime(lpfiletime : *const super::FILETIME, lpfatdate : super::LPWORD, lpfattime : super::LPWORD) -> windows_core::BOOL);
    unsafe { FileTimeToDosDateTime(lpfiletime, lpfatdate as _, lpfattime as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindActCtxSectionGuid(dwflags: super::DWORD, lpextensionguid: Option<*const windows_core::GUID>, ulsectionid: super::ULONG, lpguidtofind: Option<*const windows_core::GUID>, returneddata: PACTCTX_SECTION_KEYED_DATA) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindActCtxSectionGuid(dwflags : super::DWORD, lpextensionguid : *const windows_core::GUID, ulsectionid : super::ULONG, lpguidtofind : *const windows_core::GUID, returneddata : PACTCTX_SECTION_KEYED_DATA) -> windows_core::BOOL);
    unsafe { FindActCtxSectionGuid(dwflags, lpextensionguid.unwrap_or(core::mem::zeroed()) as _, ulsectionid, lpguidtofind.unwrap_or(core::mem::zeroed()) as _, returneddata as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindActCtxSectionStringA<P3>(dwflags: super::DWORD, lpextensionguid: Option<*const windows_core::GUID>, ulsectionid: super::ULONG, lpstringtofind: P3, returneddata: PACTCTX_SECTION_KEYED_DATA) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindActCtxSectionStringA(dwflags : super::DWORD, lpextensionguid : *const windows_core::GUID, ulsectionid : super::ULONG, lpstringtofind : windows_core::PCSTR, returneddata : PACTCTX_SECTION_KEYED_DATA) -> windows_core::BOOL);
    unsafe { FindActCtxSectionStringA(dwflags, lpextensionguid.unwrap_or(core::mem::zeroed()) as _, ulsectionid, lpstringtofind.param().abi(), returneddata as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindActCtxSectionStringW<P3>(dwflags: super::DWORD, lpextensionguid: Option<*const windows_core::GUID>, ulsectionid: super::ULONG, lpstringtofind: P3, returneddata: PACTCTX_SECTION_KEYED_DATA) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindActCtxSectionStringW(dwflags : super::DWORD, lpextensionguid : *const windows_core::GUID, ulsectionid : super::ULONG, lpstringtofind : windows_core::PCWSTR, returneddata : PACTCTX_SECTION_KEYED_DATA) -> windows_core::BOOL);
    unsafe { FindActCtxSectionStringW(dwflags, lpextensionguid.unwrap_or(core::mem::zeroed()) as _, ulsectionid, lpstringtofind.param().abi(), returneddata as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FindAtomA<P0>(lpstring: P0) -> super::ATOM
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindAtomA(lpstring : windows_core::PCSTR) -> super::ATOM);
    unsafe { FindAtomA(lpstring.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FindAtomW<P0>(lpstring: P0) -> super::ATOM
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindAtomW(lpstring : windows_core::PCWSTR) -> super::ATOM);
    unsafe { FindAtomW(lpstring.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstFileNameTransactedW<P0>(lpfilename: P0, dwflags: super::DWORD, stringlength: super::LPDWORD, linkname: windows_core::PWSTR, htransaction: Option<super::HANDLE>) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstFileNameTransactedW(lpfilename : windows_core::PCWSTR, dwflags : super::DWORD, stringlength : super::LPDWORD, linkname : windows_core::PWSTR, htransaction : super::HANDLE) -> super::HANDLE);
    unsafe { FindFirstFileNameTransactedW(lpfilename.param().abi(), dwflags, stringlength as _, linkname, htransaction.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstFileTransactedA<P0>(lpfilename: P0, finfolevelid: super::FINDEX_INFO_LEVELS, lpfindfiledata: super::LPVOID, fsearchop: super::FINDEX_SEARCH_OPS, lpsearchfilter: Option<super::LPVOID>, dwadditionalflags: super::DWORD, htransaction: super::HANDLE) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstFileTransactedA(lpfilename : windows_core::PCSTR, finfolevelid : super::FINDEX_INFO_LEVELS, lpfindfiledata : super::LPVOID, fsearchop : super::FINDEX_SEARCH_OPS, lpsearchfilter : super::LPVOID, dwadditionalflags : super::DWORD, htransaction : super::HANDLE) -> super::HANDLE);
    unsafe { FindFirstFileTransactedA(lpfilename.param().abi(), finfolevelid, lpfindfiledata as _, fsearchop, lpsearchfilter.unwrap_or(core::mem::zeroed()) as _, dwadditionalflags, htransaction) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstFileTransactedW<P0>(lpfilename: P0, finfolevelid: super::FINDEX_INFO_LEVELS, lpfindfiledata: super::LPVOID, fsearchop: super::FINDEX_SEARCH_OPS, lpsearchfilter: Option<super::LPVOID>, dwadditionalflags: super::DWORD, htransaction: super::HANDLE) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstFileTransactedW(lpfilename : windows_core::PCWSTR, finfolevelid : super::FINDEX_INFO_LEVELS, lpfindfiledata : super::LPVOID, fsearchop : super::FINDEX_SEARCH_OPS, lpsearchfilter : super::LPVOID, dwadditionalflags : super::DWORD, htransaction : super::HANDLE) -> super::HANDLE);
    unsafe { FindFirstFileTransactedW(lpfilename.param().abi(), finfolevelid, lpfindfiledata as _, fsearchop, lpsearchfilter.unwrap_or(core::mem::zeroed()) as _, dwadditionalflags, htransaction) }
}
#[cfg(all(feature = "fileapi", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstStreamTransactedW<P0>(lpfilename: P0, infolevel: super::STREAM_INFO_LEVELS, lpfindstreamdata: super::LPVOID, dwflags: Option<super::DWORD>, htransaction: super::HANDLE) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstStreamTransactedW(lpfilename : windows_core::PCWSTR, infolevel : super::STREAM_INFO_LEVELS, lpfindstreamdata : super::LPVOID, dwflags : super::DWORD, htransaction : super::HANDLE) -> super::HANDLE);
    unsafe { FindFirstStreamTransactedW(lpfilename.param().abi(), infolevel, lpfindstreamdata as _, dwflags.unwrap_or(core::mem::zeroed()) as _, htransaction) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstVolumeA(lpszvolumename: windows_core::PSTR, cchbufferlength: super::DWORD) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn FindFirstVolumeA(lpszvolumename : windows_core::PSTR, cchbufferlength : super::DWORD) -> super::HANDLE);
    unsafe { FindFirstVolumeA(lpszvolumename, cchbufferlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstVolumeMountPointA<P0>(lpszrootpathname: P0, lpszvolumemountpoint: windows_core::PSTR, cchbufferlength: super::DWORD) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstVolumeMountPointA(lpszrootpathname : windows_core::PCSTR, lpszvolumemountpoint : windows_core::PSTR, cchbufferlength : super::DWORD) -> super::HANDLE);
    unsafe { FindFirstVolumeMountPointA(lpszrootpathname.param().abi(), lpszvolumemountpoint, cchbufferlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindFirstVolumeMountPointW<P0>(lpszrootpathname: P0, lpszvolumemountpoint: windows_core::PWSTR, cchbufferlength: super::DWORD) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindFirstVolumeMountPointW(lpszrootpathname : windows_core::PCWSTR, lpszvolumemountpoint : windows_core::PWSTR, cchbufferlength : super::DWORD) -> super::HANDLE);
    unsafe { FindFirstVolumeMountPointW(lpszrootpathname.param().abi(), lpszvolumemountpoint, cchbufferlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindNextVolumeA(hfindvolume: super::HANDLE, lpszvolumename: windows_core::PSTR, cchbufferlength: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextVolumeA(hfindvolume : super::HANDLE, lpszvolumename : windows_core::PSTR, cchbufferlength : super::DWORD) -> windows_core::BOOL);
    unsafe { FindNextVolumeA(hfindvolume as _, lpszvolumename, cchbufferlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindNextVolumeMountPointA(hfindvolumemountpoint: super::HANDLE, lpszvolumemountpoint: windows_core::PSTR, cchbufferlength: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextVolumeMountPointA(hfindvolumemountpoint : super::HANDLE, lpszvolumemountpoint : windows_core::PSTR, cchbufferlength : super::DWORD) -> windows_core::BOOL);
    unsafe { FindNextVolumeMountPointA(hfindvolumemountpoint, lpszvolumemountpoint, cchbufferlength) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn FindNextVolumeMountPointW(hfindvolumemountpoint: super::HANDLE, lpszvolumemountpoint: windows_core::PWSTR, cchbufferlength: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindNextVolumeMountPointW(hfindvolumemountpoint : super::HANDLE, lpszvolumemountpoint : windows_core::PWSTR, cchbufferlength : super::DWORD) -> windows_core::BOOL);
    unsafe { FindNextVolumeMountPointW(hfindvolumemountpoint, lpszvolumemountpoint, cchbufferlength) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn FindResourceA<P1, P2>(hmodule: Option<super::HMODULE>, lpname: P1, lptype: P2) -> super::HRSRC
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindResourceA(hmodule : super::HMODULE, lpname : windows_core::PCSTR, lptype : windows_core::PCSTR) -> super::HRSRC);
    unsafe { FindResourceA(hmodule.unwrap_or(core::mem::zeroed()) as _, lpname.param().abi(), lptype.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn FindResourceExA<P1, P2>(hmodule: Option<super::HMODULE>, lptype: P1, lpname: P2, wlanguage: super::WORD) -> super::HRSRC
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn FindResourceExA(hmodule : super::HMODULE, lptype : windows_core::PCSTR, lpname : windows_core::PCSTR, wlanguage : super::WORD) -> super::HRSRC);
    unsafe { FindResourceExA(hmodule.unwrap_or(core::mem::zeroed()) as _, lptype.param().abi(), lpname.param().abi(), wlanguage) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn FindVolumeMountPointClose(hfindvolumemountpoint: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn FindVolumeMountPointClose(hfindvolumemountpoint : super::HANDLE) -> windows_core::BOOL);
    unsafe { FindVolumeMountPointClose(hfindvolumemountpoint) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn FormatMessageA(dwflags: super::DWORD, lpsource: Option<super::LPCVOID>, dwmessageid: super::DWORD, dwlanguageid: super::DWORD, lpbuffer: windows_core::PSTR, nsize: super::DWORD, arguments: Option<*const super::va_list>) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn FormatMessageA(dwflags : super::DWORD, lpsource : super::LPCVOID, dwmessageid : super::DWORD, dwlanguageid : super::DWORD, lpbuffer : windows_core::PSTR, nsize : super::DWORD, arguments : *const super::va_list) -> super::DWORD);
    unsafe { FormatMessageA(dwflags, lpsource.unwrap_or(core::mem::zeroed()) as _, dwmessageid, dwlanguageid, lpbuffer, nsize, arguments.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn FormatMessageW(dwflags: super::DWORD, lpsource: Option<super::LPCVOID>, dwmessageid: super::DWORD, dwlanguageid: super::DWORD, lpbuffer: windows_core::PWSTR, nsize: super::DWORD, arguments: Option<*const super::va_list>) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn FormatMessageW(dwflags : super::DWORD, lpsource : super::LPCVOID, dwmessageid : super::DWORD, dwlanguageid : super::DWORD, lpbuffer : windows_core::PWSTR, nsize : super::DWORD, arguments : *const super::va_list) -> super::DWORD);
    unsafe { FormatMessageW(dwflags, lpsource.unwrap_or(core::mem::zeroed()) as _, dwmessageid, dwlanguageid, lpbuffer, nsize, arguments.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetActiveProcessorCount(groupnumber: super::WORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetActiveProcessorCount(groupnumber : super::WORD) -> super::DWORD);
    unsafe { GetActiveProcessorCount(groupnumber) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetActiveProcessorGroupCount() -> super::WORD {
    windows_core::link!("kernel32.dll" "system" fn GetActiveProcessorGroupCount() -> super::WORD);
    unsafe { GetActiveProcessorGroupCount() }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetApplicationRecoveryCallback(hprocess: super::HANDLE, precoverycallback: *mut APPLICATION_RECOVERY_CALLBACK, ppvparameter: Option<*mut super::PVOID>, pdwpinginterval: Option<super::PDWORD>, pdwflags: Option<super::PDWORD>) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn GetApplicationRecoveryCallback(hprocess : super::HANDLE, precoverycallback : *mut APPLICATION_RECOVERY_CALLBACK, ppvparameter : *mut super::PVOID, pdwpinginterval : super::PDWORD, pdwflags : super::PDWORD) -> windows_core::HRESULT);
    unsafe { GetApplicationRecoveryCallback(hprocess, precoverycallback as _, ppvparameter.unwrap_or(core::mem::zeroed()) as _, pdwpinginterval.unwrap_or(core::mem::zeroed()) as _, pdwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetApplicationRestartSettings(hprocess: super::HANDLE, pwzcommandline: Option<windows_core::PWSTR>, pcchsize: super::PDWORD, pdwflags: Option<super::PDWORD>) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn GetApplicationRestartSettings(hprocess : super::HANDLE, pwzcommandline : windows_core::PWSTR, pcchsize : super::PDWORD, pdwflags : super::PDWORD) -> windows_core::HRESULT);
    unsafe { GetApplicationRestartSettings(hprocess, pwzcommandline.unwrap_or(core::mem::zeroed()) as _, pcchsize as _, pdwflags.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetAtomNameA(natom: super::ATOM, lpbuffer: windows_core::PSTR, nsize: i32) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn GetAtomNameA(natom : super::ATOM, lpbuffer : windows_core::PSTR, nsize : i32) -> super::UINT);
    unsafe { GetAtomNameA(natom, lpbuffer, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetAtomNameW(natom: super::ATOM, lpbuffer: windows_core::PWSTR, nsize: i32) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn GetAtomNameW(natom : super::ATOM, lpbuffer : windows_core::PWSTR, nsize : i32) -> super::UINT);
    unsafe { GetAtomNameW(natom, lpbuffer, nsize) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetBinaryTypeA<P0>(lpapplicationname: P0, lpbinarytype: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetBinaryTypeA(lpapplicationname : windows_core::PCSTR, lpbinarytype : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetBinaryTypeA(lpapplicationname.param().abi(), lpbinarytype as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetBinaryTypeW<P0>(lpapplicationname: P0, lpbinarytype: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetBinaryTypeW(lpapplicationname : windows_core::PCWSTR, lpbinarytype : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetBinaryTypeW(lpapplicationname.param().abi(), lpbinarytype as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetCommConfig(hcommdev: super::HANDLE, lpcc: Option<LPCOMMCONFIG>, lpdwsize: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetCommConfig(hcommdev : super::HANDLE, lpcc : LPCOMMCONFIG, lpdwsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetCommConfig(hcommdev, lpcc.unwrap_or(core::mem::zeroed()) as _, lpdwsize as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetCommMask(hfile: super::HANDLE, lpevtmask: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetCommMask(hfile : super::HANDLE, lpevtmask : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetCommMask(hfile, lpevtmask as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetCommModemStatus(hfile: super::HANDLE, lpmodemstat: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetCommModemStatus(hfile : super::HANDLE, lpmodemstat : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetCommModemStatus(hfile, lpmodemstat as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetCommPorts(lpportnumbers: super::PULONG, uportnumberscount: super::ULONG, puportnumbersfound: super::PULONG) -> super::ULONG {
    windows_core::link!("api-ms-win-core-comm-l1-1-2.dll" "system" fn GetCommPorts(lpportnumbers : super::PULONG, uportnumberscount : super::ULONG, puportnumbersfound : super::PULONG) -> super::ULONG);
    unsafe { GetCommPorts(lpportnumbers as _, uportnumberscount, puportnumbersfound as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetCommProperties(hfile: super::HANDLE, lpcommprop: LPCOMMPROP) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetCommProperties(hfile : super::HANDLE, lpcommprop : LPCOMMPROP) -> windows_core::BOOL);
    unsafe { GetCommProperties(hfile, lpcommprop as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetCommState(hfile: super::HANDLE, lpdcb: LPDCB) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetCommState(hfile : super::HANDLE, lpdcb : LPDCB) -> windows_core::BOOL);
    unsafe { GetCommState(hfile, lpdcb as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetCommTimeouts(hfile: super::HANDLE, lpcommtimeouts: LPCOMMTIMEOUTS) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetCommTimeouts(hfile : super::HANDLE, lpcommtimeouts : LPCOMMTIMEOUTS) -> windows_core::BOOL);
    unsafe { GetCommTimeouts(hfile, lpcommtimeouts as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetCompressedFileSizeTransactedA<P0>(lpfilename: P0, lpfilesizehigh: Option<super::LPDWORD>, htransaction: super::HANDLE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetCompressedFileSizeTransactedA(lpfilename : windows_core::PCSTR, lpfilesizehigh : super::LPDWORD, htransaction : super::HANDLE) -> super::DWORD);
    unsafe { GetCompressedFileSizeTransactedA(lpfilename.param().abi(), lpfilesizehigh.unwrap_or(core::mem::zeroed()) as _, htransaction) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetCompressedFileSizeTransactedW<P0>(lpfilename: P0, lpfilesizehigh: Option<super::LPDWORD>, htransaction: super::HANDLE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetCompressedFileSizeTransactedW(lpfilename : windows_core::PCWSTR, lpfilesizehigh : super::LPDWORD, htransaction : super::HANDLE) -> super::DWORD);
    unsafe { GetCompressedFileSizeTransactedW(lpfilename.param().abi(), lpfilesizehigh.unwrap_or(core::mem::zeroed()) as _, htransaction) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetComputerNameA(lpbuffer: Option<windows_core::PSTR>, nsize: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetComputerNameA(lpbuffer : windows_core::PSTR, nsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetComputerNameA(lpbuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetComputerNameW(lpbuffer: Option<windows_core::PWSTR>, nsize: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetComputerNameW(lpbuffer : windows_core::PWSTR, nsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetComputerNameW(lpbuffer.unwrap_or(core::mem::zeroed()) as _, nsize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetCurrentActCtx(lphactctx: *mut super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentActCtx(lphactctx : *mut super::HANDLE) -> windows_core::BOOL);
    unsafe { GetCurrentActCtx(lphactctx as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetCurrentHwProfileA(lphwprofileinfo: LPHW_PROFILE_INFOA) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetCurrentHwProfileA(lphwprofileinfo : LPHW_PROFILE_INFOA) -> windows_core::BOOL);
    unsafe { GetCurrentHwProfileA(lphwprofileinfo as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetCurrentHwProfileW(lphwprofileinfo: LPHW_PROFILE_INFOW) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetCurrentHwProfileW(lphwprofileinfo : LPHW_PROFILE_INFOW) -> windows_core::BOOL);
    unsafe { GetCurrentHwProfileW(lphwprofileinfo as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn GetCurrentUmsThread() -> PUMS_CONTEXT {
    windows_core::link!("kernel32.dll" "system" fn GetCurrentUmsThread() -> PUMS_CONTEXT);
    unsafe { GetCurrentUmsThread() }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetDefaultCommConfigA<P0>(lpszname: P0, lpcc: LPCOMMCONFIG, lpdwsize: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDefaultCommConfigA(lpszname : windows_core::PCSTR, lpcc : LPCOMMCONFIG, lpdwsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetDefaultCommConfigA(lpszname.param().abi(), lpcc as _, lpdwsize as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetDefaultCommConfigW<P0>(lpszname: P0, lpcc: LPCOMMCONFIG, lpdwsize: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetDefaultCommConfigW(lpszname : windows_core::PCWSTR, lpcc : LPCOMMCONFIG, lpdwsize : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetDefaultCommConfigW(lpszname.param().abi(), lpcc as _, lpdwsize as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetDevicePowerState(hdevice: super::HANDLE, pfon: *mut windows_core::BOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetDevicePowerState(hdevice : super::HANDLE, pfon : *mut windows_core::BOOL) -> windows_core::BOOL);
    unsafe { GetDevicePowerState(hdevice, pfon as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetDllDirectoryA(nbufferlength: super::DWORD, lpbuffer: Option<windows_core::PSTR>) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetDllDirectoryA(nbufferlength : super::DWORD, lpbuffer : windows_core::PSTR) -> super::DWORD);
    unsafe { GetDllDirectoryA(nbufferlength, lpbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetDllDirectoryW(nbufferlength: super::DWORD, lpbuffer: Option<windows_core::PWSTR>) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetDllDirectoryW(nbufferlength : super::DWORD, lpbuffer : windows_core::PWSTR) -> super::DWORD);
    unsafe { GetDllDirectoryW(nbufferlength, lpbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetEnabledXStateFeatures() -> super::DWORD64 {
    windows_core::link!("kernel32.dll" "system" fn GetEnabledXStateFeatures() -> super::DWORD64);
    unsafe { GetEnabledXStateFeatures() }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetEventLogInformation(heventlog: super::HANDLE, dwinfolevel: super::DWORD, lpbuffer: super::LPVOID, cbbufsize: super::DWORD, pcbbytesneeded: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetEventLogInformation(heventlog : super::HANDLE, dwinfolevel : super::DWORD, lpbuffer : super::LPVOID, cbbufsize : super::DWORD, pcbbytesneeded : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetEventLogInformation(heventlog, dwinfolevel, lpbuffer as _, cbbufsize, pcbbytesneeded as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetFileAttributesTransactedA<P0>(lpfilename: P0, finfolevelid: super::GET_FILEEX_INFO_LEVELS, lpfileinformation: super::LPVOID, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFileAttributesTransactedA(lpfilename : windows_core::PCSTR, finfolevelid : super::GET_FILEEX_INFO_LEVELS, lpfileinformation : super::LPVOID, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { GetFileAttributesTransactedA(lpfilename.param().abi(), finfolevelid, lpfileinformation as _, htransaction) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetFileAttributesTransactedW<P0>(lpfilename: P0, finfolevelid: super::GET_FILEEX_INFO_LEVELS, lpfileinformation: super::LPVOID, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFileAttributesTransactedW(lpfilename : windows_core::PCWSTR, finfolevelid : super::GET_FILEEX_INFO_LEVELS, lpfileinformation : super::LPVOID, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { GetFileAttributesTransactedW(lpfilename.param().abi(), finfolevelid, lpfileinformation as _, htransaction) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetFileBandwidthReservation(hfile: super::HANDLE, lpperiodmilliseconds: super::LPDWORD, lpbytesperperiod: super::LPDWORD, pdiscardable: super::LPBOOL, lptransfersize: super::LPDWORD, lpnumoutstandingrequests: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetFileBandwidthReservation(hfile : super::HANDLE, lpperiodmilliseconds : super::LPDWORD, lpbytesperperiod : super::LPDWORD, pdiscardable : super::LPBOOL, lptransfersize : super::LPDWORD, lpnumoutstandingrequests : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetFileBandwidthReservation(hfile, lpperiodmilliseconds as _, lpbytesperperiod as _, pdiscardable as _, lptransfersize as _, lpnumoutstandingrequests as _) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFileInformationByHandleEx(hfile: super::HANDLE, fileinformationclass: super::FILE_INFO_BY_HANDLE_CLASS, lpfileinformation: super::LPVOID, dwbuffersize: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetFileInformationByHandleEx(hfile : super::HANDLE, fileinformationclass : super::FILE_INFO_BY_HANDLE_CLASS, lpfileinformation : super::LPVOID, dwbuffersize : super::DWORD) -> windows_core::BOOL);
    unsafe { GetFileInformationByHandleEx(hfile, fileinformationclass, lpfileinformation as _, dwbuffersize) }
}
#[cfg(all(feature = "minwinbase", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFileInformationByName<P0>(filename: P0, fileinformationclass: super::FILE_INFO_BY_NAME_CLASS, fileinfobuffer: super::PVOID, fileinfobuffersize: super::ULONG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFileInformationByName(filename : windows_core::PCWSTR, fileinformationclass : super::FILE_INFO_BY_NAME_CLASS, fileinfobuffer : super::PVOID, fileinfobuffersize : super::ULONG) -> windows_core::BOOL);
    unsafe { GetFileInformationByName(filename.param().abi(), fileinformationclass, fileinfobuffer as _, fileinfobuffersize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFileSecurityA<P0>(lpfilename: P0, requestedinformation: super::SECURITY_INFORMATION, psecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, nlength: super::DWORD, lpnlengthneeded: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn GetFileSecurityA(lpfilename : windows_core::PCSTR, requestedinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, nlength : super::DWORD, lpnlengthneeded : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetFileSecurityA(lpfilename.param().abi(), requestedinformation, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, nlength, lpnlengthneeded as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableA<P0, P1>(lpname: P0, lpguid: P1, pbuffer: Option<super::PVOID>, nsize: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableA(lpname : windows_core::PCSTR, lpguid : windows_core::PCSTR, pbuffer : super::PVOID, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetFirmwareEnvironmentVariableA(lpname.param().abi(), lpguid.param().abi(), pbuffer.unwrap_or(core::mem::zeroed()) as _, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableExA<P0, P1>(lpname: P0, lpguid: P1, pbuffer: Option<super::PVOID>, nsize: super::DWORD, pdwattribubutes: Option<super::PDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableExA(lpname : windows_core::PCSTR, lpguid : windows_core::PCSTR, pbuffer : super::PVOID, nsize : super::DWORD, pdwattribubutes : super::PDWORD) -> super::DWORD);
    unsafe { GetFirmwareEnvironmentVariableExA(lpname.param().abi(), lpguid.param().abi(), pbuffer.unwrap_or(core::mem::zeroed()) as _, nsize, pdwattribubutes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableExW<P0, P1>(lpname: P0, lpguid: P1, pbuffer: Option<super::PVOID>, nsize: super::DWORD, pdwattribubutes: Option<super::PDWORD>) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableExW(lpname : windows_core::PCWSTR, lpguid : windows_core::PCWSTR, pbuffer : super::PVOID, nsize : super::DWORD, pdwattribubutes : super::PDWORD) -> super::DWORD);
    unsafe { GetFirmwareEnvironmentVariableExW(lpname.param().abi(), lpguid.param().abi(), pbuffer.unwrap_or(core::mem::zeroed()) as _, nsize, pdwattribubutes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFirmwareEnvironmentVariableW<P0, P1>(lpname: P0, lpguid: P1, pbuffer: Option<super::PVOID>, nsize: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFirmwareEnvironmentVariableW(lpname : windows_core::PCWSTR, lpguid : windows_core::PCWSTR, pbuffer : super::PVOID, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetFirmwareEnvironmentVariableW(lpname.param().abi(), lpguid.param().abi(), pbuffer.unwrap_or(core::mem::zeroed()) as _, nsize) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetFirmwareType(firmwaretype: super::PFIRMWARE_TYPE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetFirmwareType(firmwaretype : super::PFIRMWARE_TYPE) -> windows_core::BOOL);
    unsafe { GetFirmwareType(firmwaretype as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFullPathNameTransactedA<P0>(lpfilename: P0, nbufferlength: super::DWORD, lpbuffer: Option<windows_core::PSTR>, lpfilepart: Option<*mut windows_core::PSTR>, htransaction: super::HANDLE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFullPathNameTransactedA(lpfilename : windows_core::PCSTR, nbufferlength : super::DWORD, lpbuffer : windows_core::PSTR, lpfilepart : *mut windows_core::PSTR, htransaction : super::HANDLE) -> super::DWORD);
    unsafe { GetFullPathNameTransactedA(lpfilename.param().abi(), nbufferlength, lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpfilepart.unwrap_or(core::mem::zeroed()) as _, htransaction) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetFullPathNameTransactedW<P0>(lpfilename: P0, nbufferlength: super::DWORD, lpbuffer: Option<windows_core::PWSTR>, lpfilepart: Option<*mut windows_core::PWSTR>, htransaction: super::HANDLE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetFullPathNameTransactedW(lpfilename : windows_core::PCWSTR, nbufferlength : super::DWORD, lpbuffer : windows_core::PWSTR, lpfilepart : *mut windows_core::PWSTR, htransaction : super::HANDLE) -> super::DWORD);
    unsafe { GetFullPathNameTransactedW(lpfilename.param().abi(), nbufferlength, lpbuffer.unwrap_or(core::mem::zeroed()) as _, lpfilepart.unwrap_or(core::mem::zeroed()) as _, htransaction) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetLogicalDriveStringsA(nbufferlength: super::DWORD, lpbuffer: Option<windows_core::PSTR>) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetLogicalDriveStringsA(nbufferlength : super::DWORD, lpbuffer : windows_core::PSTR) -> super::DWORD);
    unsafe { GetLogicalDriveStringsA(nbufferlength, lpbuffer.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetLongPathNameTransactedA<P0>(lpszshortpath: P0, lpszlongpath: Option<windows_core::PSTR>, cchbuffer: super::DWORD, htransaction: super::HANDLE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetLongPathNameTransactedA(lpszshortpath : windows_core::PCSTR, lpszlongpath : windows_core::PSTR, cchbuffer : super::DWORD, htransaction : super::HANDLE) -> super::DWORD);
    unsafe { GetLongPathNameTransactedA(lpszshortpath.param().abi(), lpszlongpath.unwrap_or(core::mem::zeroed()) as _, cchbuffer, htransaction) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetLongPathNameTransactedW<P0>(lpszshortpath: P0, lpszlongpath: Option<windows_core::PWSTR>, cchbuffer: super::DWORD, htransaction: super::HANDLE) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetLongPathNameTransactedW(lpszshortpath : windows_core::PCWSTR, lpszlongpath : windows_core::PWSTR, cchbuffer : super::DWORD, htransaction : super::HANDLE) -> super::DWORD);
    unsafe { GetLongPathNameTransactedW(lpszshortpath.param().abi(), lpszlongpath.unwrap_or(core::mem::zeroed()) as _, cchbuffer, htransaction) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetMailslotInfo(hmailslot: super::HANDLE, lpmaxmessagesize: Option<super::LPDWORD>, lpnextsize: Option<super::LPDWORD>, lpmessagecount: Option<super::LPDWORD>, lpreadtimeout: Option<super::LPDWORD>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetMailslotInfo(hmailslot : super::HANDLE, lpmaxmessagesize : super::LPDWORD, lpnextsize : super::LPDWORD, lpmessagecount : super::LPDWORD, lpreadtimeout : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetMailslotInfo(hmailslot, lpmaxmessagesize.unwrap_or(core::mem::zeroed()) as _, lpnextsize.unwrap_or(core::mem::zeroed()) as _, lpmessagecount.unwrap_or(core::mem::zeroed()) as _, lpreadtimeout.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetMaximumProcessorCount(groupnumber: super::WORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetMaximumProcessorCount(groupnumber : super::WORD) -> super::DWORD);
    unsafe { GetMaximumProcessorCount(groupnumber) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetMaximumProcessorGroupCount() -> super::WORD {
    windows_core::link!("kernel32.dll" "system" fn GetMaximumProcessorGroupCount() -> super::WORD);
    unsafe { GetMaximumProcessorGroupCount() }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetNamedPipeClientComputerNameA(pipe: super::HANDLE, clientcomputername: windows_core::PSTR, clientcomputernamelength: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeClientComputerNameA(pipe : super::HANDLE, clientcomputername : windows_core::PSTR, clientcomputernamelength : super::ULONG) -> windows_core::BOOL);
    unsafe { GetNamedPipeClientComputerNameA(pipe, clientcomputername, clientcomputernamelength) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetNamedPipeClientProcessId(pipe: super::HANDLE, clientprocessid: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeClientProcessId(pipe : super::HANDLE, clientprocessid : super::PULONG) -> windows_core::BOOL);
    unsafe { GetNamedPipeClientProcessId(pipe, clientprocessid as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetNamedPipeClientSessionId(pipe: super::HANDLE, clientsessionid: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeClientSessionId(pipe : super::HANDLE, clientsessionid : super::PULONG) -> windows_core::BOOL);
    unsafe { GetNamedPipeClientSessionId(pipe, clientsessionid as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetNamedPipeHandleStateA(hnamedpipe: super::HANDLE, lpstate: Option<super::LPDWORD>, lpcurinstances: Option<super::LPDWORD>, lpmaxcollectioncount: Option<super::LPDWORD>, lpcollectdatatimeout: Option<super::LPDWORD>, lpusername: Option<windows_core::PSTR>, nmaxusernamesize: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeHandleStateA(hnamedpipe : super::HANDLE, lpstate : super::LPDWORD, lpcurinstances : super::LPDWORD, lpmaxcollectioncount : super::LPDWORD, lpcollectdatatimeout : super::LPDWORD, lpusername : windows_core::PSTR, nmaxusernamesize : super::DWORD) -> windows_core::BOOL);
    unsafe { GetNamedPipeHandleStateA(hnamedpipe, lpstate.unwrap_or(core::mem::zeroed()) as _, lpcurinstances.unwrap_or(core::mem::zeroed()) as _, lpmaxcollectioncount.unwrap_or(core::mem::zeroed()) as _, lpcollectdatatimeout.unwrap_or(core::mem::zeroed()) as _, lpusername.unwrap_or(core::mem::zeroed()) as _, nmaxusernamesize) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetNamedPipeServerProcessId(pipe: super::HANDLE, serverprocessid: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeServerProcessId(pipe : super::HANDLE, serverprocessid : super::PULONG) -> windows_core::BOOL);
    unsafe { GetNamedPipeServerProcessId(pipe, serverprocessid as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetNamedPipeServerSessionId(pipe: super::HANDLE, serversessionid: super::PULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNamedPipeServerSessionId(pipe : super::HANDLE, serversessionid : super::PULONG) -> windows_core::BOOL);
    unsafe { GetNamedPipeServerSessionId(pipe, serversessionid as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[inline]
pub unsafe fn GetNextUmsListItem(umscontext: PUMS_CONTEXT) -> PUMS_CONTEXT {
    windows_core::link!("kernel32.dll" "system" fn GetNextUmsListItem(umscontext : PUMS_CONTEXT) -> PUMS_CONTEXT);
    unsafe { GetNextUmsListItem(umscontext as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetNumaAvailableMemoryNode(node: super::UCHAR, availablebytes: super::PULONGLONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaAvailableMemoryNode(node : super::UCHAR, availablebytes : super::PULONGLONG) -> windows_core::BOOL);
    unsafe { GetNumaAvailableMemoryNode(node, availablebytes as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetNumaAvailableMemoryNodeEx(node: super::USHORT, availablebytes: super::PULONGLONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaAvailableMemoryNodeEx(node : super::USHORT, availablebytes : super::PULONGLONG) -> windows_core::BOOL);
    unsafe { GetNumaAvailableMemoryNodeEx(node, availablebytes as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetNumaNodeNumberFromHandle(hfile: super::HANDLE, nodenumber: super::PUSHORT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaNodeNumberFromHandle(hfile : super::HANDLE, nodenumber : super::PUSHORT) -> windows_core::BOOL);
    unsafe { GetNumaNodeNumberFromHandle(hfile, nodenumber as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetNumaNodeProcessorMask(node: super::UCHAR, processormask: super::PULONGLONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaNodeProcessorMask(node : super::UCHAR, processormask : super::PULONGLONG) -> windows_core::BOOL);
    unsafe { GetNumaNodeProcessorMask(node, processormask as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetNumaProcessorNode(processor: super::UCHAR, nodenumber: super::PUCHAR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaProcessorNode(processor : super::UCHAR, nodenumber : super::PUCHAR) -> windows_core::BOOL);
    unsafe { GetNumaProcessorNode(processor, nodenumber as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetNumaProcessorNodeEx(processor: super::PPROCESSOR_NUMBER, nodenumber: super::PUSHORT) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaProcessorNodeEx(processor : super::PPROCESSOR_NUMBER, nodenumber : super::PUSHORT) -> windows_core::BOOL);
    unsafe { GetNumaProcessorNodeEx(processor, nodenumber as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetNumaProximityNode(proximityid: super::ULONG, nodenumber: super::PUCHAR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetNumaProximityNode(proximityid : super::ULONG, nodenumber : super::PUCHAR) -> windows_core::BOOL);
    unsafe { GetNumaProximityNode(proximityid, nodenumber as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetNumberOfEventLogRecords(heventlog: super::HANDLE, numberofrecords: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetNumberOfEventLogRecords(heventlog : super::HANDLE, numberofrecords : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetNumberOfEventLogRecords(heventlog, numberofrecords as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetOldestEventLogRecord(heventlog: super::HANDLE, oldestrecord: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetOldestEventLogRecord(heventlog : super::HANDLE, oldestrecord : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetOldestEventLogRecord(heventlog, oldestrecord as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPrivateProfileIntA<P0, P1, P3>(lpappname: P0, lpkeyname: P1, ndefault: super::INT, lpfilename: P3) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileIntA(lpappname : windows_core::PCSTR, lpkeyname : windows_core::PCSTR, ndefault : super::INT, lpfilename : windows_core::PCSTR) -> super::UINT);
    unsafe { GetPrivateProfileIntA(lpappname.param().abi(), lpkeyname.param().abi(), ndefault, lpfilename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPrivateProfileIntW<P0, P1, P3>(lpappname: P0, lpkeyname: P1, ndefault: super::INT, lpfilename: P3) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileIntW(lpappname : windows_core::PCWSTR, lpkeyname : windows_core::PCWSTR, ndefault : super::INT, lpfilename : windows_core::PCWSTR) -> super::UINT);
    unsafe { GetPrivateProfileIntW(lpappname.param().abi(), lpkeyname.param().abi(), ndefault, lpfilename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPrivateProfileSectionA<P0, P3>(lpappname: P0, lpreturnedstring: Option<windows_core::PSTR>, nsize: super::DWORD, lpfilename: P3) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileSectionA(lpappname : windows_core::PCSTR, lpreturnedstring : windows_core::PSTR, nsize : super::DWORD, lpfilename : windows_core::PCSTR) -> super::DWORD);
    unsafe { GetPrivateProfileSectionA(lpappname.param().abi(), lpreturnedstring.unwrap_or(core::mem::zeroed()) as _, nsize, lpfilename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPrivateProfileSectionNamesA<P2>(lpszreturnbuffer: Option<windows_core::PSTR>, nsize: super::DWORD, lpfilename: P2) -> super::DWORD
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileSectionNamesA(lpszreturnbuffer : windows_core::PSTR, nsize : super::DWORD, lpfilename : windows_core::PCSTR) -> super::DWORD);
    unsafe { GetPrivateProfileSectionNamesA(lpszreturnbuffer.unwrap_or(core::mem::zeroed()) as _, nsize, lpfilename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPrivateProfileSectionNamesW<P2>(lpszreturnbuffer: Option<windows_core::PWSTR>, nsize: super::DWORD, lpfilename: P2) -> super::DWORD
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileSectionNamesW(lpszreturnbuffer : windows_core::PWSTR, nsize : super::DWORD, lpfilename : windows_core::PCWSTR) -> super::DWORD);
    unsafe { GetPrivateProfileSectionNamesW(lpszreturnbuffer.unwrap_or(core::mem::zeroed()) as _, nsize, lpfilename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPrivateProfileSectionW<P0, P3>(lpappname: P0, lpreturnedstring: Option<windows_core::PWSTR>, nsize: super::DWORD, lpfilename: P3) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileSectionW(lpappname : windows_core::PCWSTR, lpreturnedstring : windows_core::PWSTR, nsize : super::DWORD, lpfilename : windows_core::PCWSTR) -> super::DWORD);
    unsafe { GetPrivateProfileSectionW(lpappname.param().abi(), lpreturnedstring.unwrap_or(core::mem::zeroed()) as _, nsize, lpfilename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPrivateProfileStringA<P0, P1, P2, P5>(lpappname: P0, lpkeyname: P1, lpdefault: P2, lpreturnedstring: Option<windows_core::PSTR>, nsize: super::DWORD, lpfilename: P5) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P5: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileStringA(lpappname : windows_core::PCSTR, lpkeyname : windows_core::PCSTR, lpdefault : windows_core::PCSTR, lpreturnedstring : windows_core::PSTR, nsize : super::DWORD, lpfilename : windows_core::PCSTR) -> super::DWORD);
    unsafe { GetPrivateProfileStringA(lpappname.param().abi(), lpkeyname.param().abi(), lpdefault.param().abi(), lpreturnedstring.unwrap_or(core::mem::zeroed()) as _, nsize, lpfilename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetPrivateProfileStringW<P0, P1, P2, P5>(lpappname: P0, lpkeyname: P1, lpdefault: P2, lpreturnedstring: Option<windows_core::PWSTR>, nsize: super::DWORD, lpfilename: P5) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileStringW(lpappname : windows_core::PCWSTR, lpkeyname : windows_core::PCWSTR, lpdefault : windows_core::PCWSTR, lpreturnedstring : windows_core::PWSTR, nsize : super::DWORD, lpfilename : windows_core::PCWSTR) -> super::DWORD);
    unsafe { GetPrivateProfileStringW(lpappname.param().abi(), lpkeyname.param().abi(), lpdefault.param().abi(), lpreturnedstring.unwrap_or(core::mem::zeroed()) as _, nsize, lpfilename.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetPrivateProfileStructA<P0, P1, P4>(lpszsection: P0, lpszkey: P1, lpstruct: Option<super::LPVOID>, usizestruct: super::UINT, szfile: P4) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileStructA(lpszsection : windows_core::PCSTR, lpszkey : windows_core::PCSTR, lpstruct : super::LPVOID, usizestruct : super::UINT, szfile : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { GetPrivateProfileStructA(lpszsection.param().abi(), lpszkey.param().abi(), lpstruct.unwrap_or(core::mem::zeroed()) as _, usizestruct, szfile.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GetPrivateProfileStructW<P0, P1, P4>(lpszsection: P0, lpszkey: P1, lpstruct: Option<super::LPVOID>, usizestruct: super::UINT, szfile: P4) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetPrivateProfileStructW(lpszsection : windows_core::PCWSTR, lpszkey : windows_core::PCWSTR, lpstruct : super::LPVOID, usizestruct : super::UINT, szfile : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { GetPrivateProfileStructW(lpszsection.param().abi(), lpszkey.param().abi(), lpstruct.unwrap_or(core::mem::zeroed()) as _, usizestruct, szfile.param().abi()) }
}
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessAffinityMask(hprocess: super::HANDLE, lpprocessaffinitymask: super::PDWORD_PTR, lpsystemaffinitymask: super::PDWORD_PTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessAffinityMask(hprocess : super::HANDLE, lpprocessaffinitymask : super::PDWORD_PTR, lpsystemaffinitymask : super::PDWORD_PTR) -> windows_core::BOOL);
    unsafe { GetProcessAffinityMask(hprocess, lpprocessaffinitymask as _, lpsystemaffinitymask as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessDEPPolicy(hprocess: super::HANDLE, lpflags: super::LPDWORD, lppermanent: super::PBOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessDEPPolicy(hprocess : super::HANDLE, lpflags : super::LPDWORD, lppermanent : super::PBOOL) -> windows_core::BOOL);
    unsafe { GetProcessDEPPolicy(hprocess, lpflags as _, lppermanent as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetProcessIoCounters(hprocess: super::HANDLE, lpiocounters: super::PIO_COUNTERS) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetProcessIoCounters(hprocess : super::HANDLE, lpiocounters : super::PIO_COUNTERS) -> windows_core::BOOL);
    unsafe { GetProcessIoCounters(hprocess, lpiocounters as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetProfileIntA<P0, P1>(lpappname: P0, lpkeyname: P1, ndefault: super::INT) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProfileIntA(lpappname : windows_core::PCSTR, lpkeyname : windows_core::PCSTR, ndefault : super::INT) -> super::UINT);
    unsafe { GetProfileIntA(lpappname.param().abi(), lpkeyname.param().abi(), ndefault) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetProfileIntW<P0, P1>(lpappname: P0, lpkeyname: P1, ndefault: super::INT) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProfileIntW(lpappname : windows_core::PCWSTR, lpkeyname : windows_core::PCWSTR, ndefault : super::INT) -> super::UINT);
    unsafe { GetProfileIntW(lpappname.param().abi(), lpkeyname.param().abi(), ndefault) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetProfileSectionA<P0>(lpappname: P0, lpreturnedstring: Option<windows_core::PSTR>, nsize: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProfileSectionA(lpappname : windows_core::PCSTR, lpreturnedstring : windows_core::PSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetProfileSectionA(lpappname.param().abi(), lpreturnedstring.unwrap_or(core::mem::zeroed()) as _, nsize) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetProfileSectionW<P0>(lpappname: P0, lpreturnedstring: Option<windows_core::PWSTR>, nsize: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProfileSectionW(lpappname : windows_core::PCWSTR, lpreturnedstring : windows_core::PWSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetProfileSectionW(lpappname.param().abi(), lpreturnedstring.unwrap_or(core::mem::zeroed()) as _, nsize) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetProfileStringA<P0, P1, P2>(lpappname: P0, lpkeyname: P1, lpdefault: P2, lpreturnedstring: Option<windows_core::PSTR>, nsize: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProfileStringA(lpappname : windows_core::PCSTR, lpkeyname : windows_core::PCSTR, lpdefault : windows_core::PCSTR, lpreturnedstring : windows_core::PSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetProfileStringA(lpappname.param().abi(), lpkeyname.param().abi(), lpdefault.param().abi(), lpreturnedstring.unwrap_or(core::mem::zeroed()) as _, nsize) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetProfileStringW<P0, P1, P2>(lpappname: P0, lpkeyname: P1, lpdefault: P2, lpreturnedstring: Option<windows_core::PWSTR>, nsize: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetProfileStringW(lpappname : windows_core::PCWSTR, lpkeyname : windows_core::PCWSTR, lpdefault : windows_core::PCWSTR, lpreturnedstring : windows_core::PWSTR, nsize : super::DWORD) -> super::DWORD);
    unsafe { GetProfileStringW(lpappname.param().abi(), lpkeyname.param().abi(), lpdefault.param().abi(), lpreturnedstring.unwrap_or(core::mem::zeroed()) as _, nsize) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetShortPathNameA<P0>(lpszlongpath: P0, lpszshortpath: Option<windows_core::PSTR>, cchbuffer: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetShortPathNameA(lpszlongpath : windows_core::PCSTR, lpszshortpath : windows_core::PSTR, cchbuffer : super::DWORD) -> super::DWORD);
    unsafe { GetShortPathNameA(lpszlongpath.param().abi(), lpszshortpath.unwrap_or(core::mem::zeroed()) as _, cchbuffer) }
}
#[cfg(all(feature = "processthreadsapi", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetStartupInfoA(lpstartupinfo: super::LPSTARTUPINFOA) {
    windows_core::link!("kernel32.dll" "system" fn GetStartupInfoA(lpstartupinfo : super::LPSTARTUPINFOA));
    unsafe { GetStartupInfoA(lpstartupinfo as _) }
}
#[inline]
pub unsafe fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE {
    windows_core::link!("kernel32.dll" "system" fn GetSystemDEPPolicy() -> DEP_SYSTEM_POLICY_TYPE);
    unsafe { GetSystemDEPPolicy() }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetSystemPowerStatus(lpsystempowerstatus: LPSYSTEM_POWER_STATUS) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetSystemPowerStatus(lpsystempowerstatus : LPSYSTEM_POWER_STATUS) -> windows_core::BOOL);
    unsafe { GetSystemPowerStatus(lpsystempowerstatus as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetSystemRegistryQuota(pdwquotaallowed: Option<super::PDWORD>, pdwquotaused: Option<super::PDWORD>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetSystemRegistryQuota(pdwquotaallowed : super::PDWORD, pdwquotaused : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetSystemRegistryQuota(pdwquotaallowed.unwrap_or(core::mem::zeroed()) as _, pdwquotaused.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetTapeParameters(hdevice: super::HANDLE, dwoperation: super::DWORD, lpdwsize: super::LPDWORD, lptapeinformation: super::LPVOID) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetTapeParameters(hdevice : super::HANDLE, dwoperation : super::DWORD, lpdwsize : super::LPDWORD, lptapeinformation : super::LPVOID) -> super::DWORD);
    unsafe { GetTapeParameters(hdevice, dwoperation, lpdwsize as _, lptapeinformation as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetTapePosition(hdevice: super::HANDLE, dwpositiontype: super::DWORD, lpdwpartition: super::LPDWORD, lpdwoffsetlow: super::LPDWORD, lpdwoffsethigh: super::LPDWORD) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetTapePosition(hdevice : super::HANDLE, dwpositiontype : super::DWORD, lpdwpartition : super::LPDWORD, lpdwoffsetlow : super::LPDWORD, lpdwoffsethigh : super::LPDWORD) -> super::DWORD);
    unsafe { GetTapePosition(hdevice, dwpositiontype, lpdwpartition as _, lpdwoffsetlow as _, lpdwoffsethigh as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetTapeStatus(hdevice: super::HANDLE) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn GetTapeStatus(hdevice : super::HANDLE) -> super::DWORD);
    unsafe { GetTapeStatus(hdevice) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetThreadEnabledXStateFeatures() -> super::DWORD64 {
    windows_core::link!("kernel32.dll" "system" fn GetThreadEnabledXStateFeatures() -> super::DWORD64);
    unsafe { GetThreadEnabledXStateFeatures() }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetThreadSelectorEntry(hthread: super::HANDLE, dwselector: super::DWORD, lpselectorentry: LPLDT_ENTRY) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetThreadSelectorEntry(hthread : super::HANDLE, dwselector : super::DWORD, lpselectorentry : LPLDT_ENTRY) -> windows_core::BOOL);
    unsafe { GetThreadSelectorEntry(hthread, dwselector, lpselectorentry as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn GetUmsCompletionListEvent(umscompletionlist: PUMS_COMPLETION_LIST, umscompletionevent: super::PHANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetUmsCompletionListEvent(umscompletionlist : PUMS_COMPLETION_LIST, umscompletionevent : super::PHANDLE) -> windows_core::BOOL);
    unsafe { GetUmsCompletionListEvent(umscompletionlist, umscompletionevent as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetUmsSystemThreadInformation(threadhandle: super::HANDLE, systemthreadinfo: PUMS_SYSTEM_THREAD_INFORMATION) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetUmsSystemThreadInformation(threadhandle : super::HANDLE, systemthreadinfo : PUMS_SYSTEM_THREAD_INFORMATION) -> windows_core::BOOL);
    unsafe { GetUmsSystemThreadInformation(threadhandle, systemthreadinfo as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetUserNameA(lpbuffer: Option<windows_core::PSTR>, pcbbuffer: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetUserNameA(lpbuffer : windows_core::PSTR, pcbbuffer : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetUserNameA(lpbuffer.unwrap_or(core::mem::zeroed()) as _, pcbbuffer as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GetUserNameW(lpbuffer: Option<windows_core::PWSTR>, pcbbuffer: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn GetUserNameW(lpbuffer : windows_core::PWSTR, pcbbuffer : super::LPDWORD) -> windows_core::BOOL);
    unsafe { GetUserNameW(lpbuffer.unwrap_or(core::mem::zeroed()) as _, pcbbuffer as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetVolumeNameForVolumeMountPointA<P0>(lpszvolumemountpoint: P0, lpszvolumename: windows_core::PSTR, cchbufferlength: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetVolumeNameForVolumeMountPointA(lpszvolumemountpoint : windows_core::PCSTR, lpszvolumename : windows_core::PSTR, cchbufferlength : super::DWORD) -> windows_core::BOOL);
    unsafe { GetVolumeNameForVolumeMountPointA(lpszvolumemountpoint.param().abi(), lpszvolumename, cchbufferlength) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GetVolumePathNameA<P0>(lpszfilename: P0, lpszvolumepathname: windows_core::PSTR, cchbufferlength: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetVolumePathNameA(lpszfilename : windows_core::PCSTR, lpszvolumepathname : windows_core::PSTR, cchbufferlength : super::DWORD) -> windows_core::BOOL);
    unsafe { GetVolumePathNameA(lpszfilename.param().abi(), lpszvolumepathname, cchbufferlength) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetVolumePathNamesForVolumeNameA<P0>(lpszvolumename: P0, lpszvolumepathnames: Option<super::LPCH>, cchbufferlength: super::DWORD, lpcchreturnlength: super::PDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GetVolumePathNamesForVolumeNameA(lpszvolumename : windows_core::PCSTR, lpszvolumepathnames : super::LPCH, cchbufferlength : super::DWORD, lpcchreturnlength : super::PDWORD) -> windows_core::BOOL);
    unsafe { GetVolumePathNamesForVolumeNameA(lpszvolumename.param().abi(), lpszvolumepathnames.unwrap_or(core::mem::zeroed()) as _, cchbufferlength, lpcchreturnlength as _) }
}
#[must_use]
#[cfg(all(feature = "basetsd", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn GetXStateFeaturesMask(context: super::PCONTEXT, featuremask: super::PDWORD64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GetXStateFeaturesMask(context : super::PCONTEXT, featuremask : super::PDWORD64) -> windows_core::BOOL);
    unsafe { GetXStateFeaturesMask(context, featuremask as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GlobalAddAtomA<P0>(lpstring: P0) -> super::ATOM
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GlobalAddAtomA(lpstring : windows_core::PCSTR) -> super::ATOM);
    unsafe { GlobalAddAtomA(lpstring.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GlobalAddAtomExA<P0>(lpstring: P0, flags: super::DWORD) -> super::ATOM
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GlobalAddAtomExA(lpstring : windows_core::PCSTR, flags : super::DWORD) -> super::ATOM);
    unsafe { GlobalAddAtomExA(lpstring.param().abi(), flags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GlobalAddAtomExW<P0>(lpstring: P0, flags: super::DWORD) -> super::ATOM
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GlobalAddAtomExW(lpstring : windows_core::PCWSTR, flags : super::DWORD) -> super::ATOM);
    unsafe { GlobalAddAtomExW(lpstring.param().abi(), flags) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GlobalAddAtomW<P0>(lpstring: P0) -> super::ATOM
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GlobalAddAtomW(lpstring : windows_core::PCWSTR) -> super::ATOM);
    unsafe { GlobalAddAtomW(lpstring.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GlobalAlloc(uflags: super::UINT, dwbytes: super::SIZE_T) -> super::HGLOBAL {
    windows_core::link!("kernel32.dll" "system" fn GlobalAlloc(uflags : super::UINT, dwbytes : super::SIZE_T) -> super::HGLOBAL);
    unsafe { GlobalAlloc(uflags, dwbytes) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GlobalCompact(dwminfree: super::DWORD) -> super::SIZE_T {
    windows_core::link!("kernel32.dll" "system" fn GlobalCompact(dwminfree : super::DWORD) -> super::SIZE_T);
    unsafe { GlobalCompact(dwminfree) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GlobalDeleteAtom(natom: super::ATOM) -> super::ATOM {
    windows_core::link!("kernel32.dll" "system" fn GlobalDeleteAtom(natom : super::ATOM) -> super::ATOM);
    unsafe { GlobalDeleteAtom(natom) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GlobalFindAtomA<P0>(lpstring: P0) -> super::ATOM
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GlobalFindAtomA(lpstring : windows_core::PCSTR) -> super::ATOM);
    unsafe { GlobalFindAtomA(lpstring.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GlobalFindAtomW<P0>(lpstring: P0) -> super::ATOM
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn GlobalFindAtomW(lpstring : windows_core::PCWSTR) -> super::ATOM);
    unsafe { GlobalFindAtomW(lpstring.param().abi()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GlobalFix(hmem: super::HGLOBAL) {
    windows_core::link!("kernel32.dll" "system" fn GlobalFix(hmem : super::HGLOBAL));
    unsafe { GlobalFix(hmem) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GlobalFlags(hmem: super::HGLOBAL) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn GlobalFlags(hmem : super::HGLOBAL) -> super::UINT);
    unsafe { GlobalFlags(hmem) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GlobalFree(hmem: super::HGLOBAL) -> super::HGLOBAL {
    windows_core::link!("kernel32.dll" "system" fn GlobalFree(hmem : super::HGLOBAL) -> super::HGLOBAL);
    unsafe { GlobalFree(hmem as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GlobalGetAtomNameA(natom: super::ATOM, lpbuffer: windows_core::PSTR, nsize: i32) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn GlobalGetAtomNameA(natom : super::ATOM, lpbuffer : windows_core::PSTR, nsize : i32) -> super::UINT);
    unsafe { GlobalGetAtomNameA(natom, lpbuffer, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GlobalGetAtomNameW(natom: super::ATOM, lpbuffer: windows_core::PWSTR, nsize: i32) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn GlobalGetAtomNameW(natom : super::ATOM, lpbuffer : windows_core::PWSTR, nsize : i32) -> super::UINT);
    unsafe { GlobalGetAtomNameW(natom, lpbuffer, nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GlobalHandle(pmem: super::LPCVOID) -> super::HGLOBAL {
    windows_core::link!("kernel32.dll" "system" fn GlobalHandle(pmem : super::LPCVOID) -> super::HGLOBAL);
    unsafe { GlobalHandle(pmem) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GlobalLock(hmem: super::HGLOBAL) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn GlobalLock(hmem : super::HGLOBAL) -> super::LPVOID);
    unsafe { GlobalLock(hmem) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn GlobalMemoryStatus(lpbuffer: LPMEMORYSTATUS) {
    windows_core::link!("kernel32.dll" "system" fn GlobalMemoryStatus(lpbuffer : LPMEMORYSTATUS));
    unsafe { GlobalMemoryStatus(lpbuffer as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GlobalReAlloc(hmem: super::HGLOBAL, dwbytes: super::SIZE_T, uflags: super::UINT) -> super::HGLOBAL {
    windows_core::link!("kernel32.dll" "system" fn GlobalReAlloc(hmem : super::HGLOBAL, dwbytes : super::SIZE_T, uflags : super::UINT) -> super::HGLOBAL);
    unsafe { GlobalReAlloc(hmem as _, dwbytes, uflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn GlobalSize(hmem: super::HGLOBAL) -> super::SIZE_T {
    windows_core::link!("kernel32.dll" "system" fn GlobalSize(hmem : super::HGLOBAL) -> super::SIZE_T);
    unsafe { GlobalSize(hmem) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GlobalUnWire(hmem: super::HGLOBAL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GlobalUnWire(hmem : super::HGLOBAL) -> windows_core::BOOL);
    unsafe { GlobalUnWire(hmem) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GlobalUnfix(hmem: super::HGLOBAL) {
    windows_core::link!("kernel32.dll" "system" fn GlobalUnfix(hmem : super::HGLOBAL));
    unsafe { GlobalUnfix(hmem) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GlobalUnlock(hmem: super::HGLOBAL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn GlobalUnlock(hmem : super::HGLOBAL) -> windows_core::BOOL);
    unsafe { GlobalUnlock(hmem) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn GlobalWire(hmem: super::HGLOBAL) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn GlobalWire(hmem : super::HGLOBAL) -> super::LPVOID);
    unsafe { GlobalWire(hmem) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn InitAtomTable(nsize: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitAtomTable(nsize : super::DWORD) -> windows_core::BOOL);
    unsafe { InitAtomTable(nsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InitializeContext(buffer: Option<super::PVOID>, contextflags: super::DWORD, context: *mut super::PCONTEXT, contextlength: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitializeContext(buffer : super::PVOID, contextflags : super::DWORD, context : *mut super::PCONTEXT, contextlength : super::PDWORD) -> windows_core::BOOL);
    unsafe { InitializeContext(buffer.unwrap_or(core::mem::zeroed()) as _, contextflags, context as _, contextlength as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn InitializeContext2(buffer: Option<super::PVOID>, contextflags: super::DWORD, context: *mut super::PCONTEXT, contextlength: super::PDWORD, xstatecompactionmask: super::ULONG64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn InitializeContext2(buffer : super::PVOID, contextflags : super::DWORD, context : *mut super::PCONTEXT, contextlength : super::PDWORD, xstatecompactionmask : super::ULONG64) -> windows_core::BOOL);
    unsafe { InitializeContext2(buffer.unwrap_or(core::mem::zeroed()) as _, contextflags, context as _, contextlength as _, xstatecompactionmask) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn IsBadCodePtr(lpfn: super::FARPROC) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsBadCodePtr(lpfn : super::FARPROC) -> windows_core::BOOL);
    unsafe { IsBadCodePtr(lpfn) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn IsBadHugeReadPtr(lp: Option<*const core::ffi::c_void>, ucb: super::UINT_PTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsBadHugeReadPtr(lp : *const core::ffi::c_void, ucb : super::UINT_PTR) -> windows_core::BOOL);
    unsafe { IsBadHugeReadPtr(lp.unwrap_or(core::mem::zeroed()) as _, ucb) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn IsBadHugeWritePtr(lp: Option<super::LPVOID>, ucb: super::UINT_PTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsBadHugeWritePtr(lp : super::LPVOID, ucb : super::UINT_PTR) -> windows_core::BOOL);
    unsafe { IsBadHugeWritePtr(lp.unwrap_or(core::mem::zeroed()) as _, ucb) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn IsBadReadPtr(lp: Option<*const core::ffi::c_void>, ucb: super::UINT_PTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsBadReadPtr(lp : *const core::ffi::c_void, ucb : super::UINT_PTR) -> windows_core::BOOL);
    unsafe { IsBadReadPtr(lp.unwrap_or(core::mem::zeroed()) as _, ucb) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn IsBadStringPtrA<P0>(lpsz: P0, ucchmax: super::UINT_PTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn IsBadStringPtrA(lpsz : windows_core::PCSTR, ucchmax : super::UINT_PTR) -> windows_core::BOOL);
    unsafe { IsBadStringPtrA(lpsz.param().abi(), ucchmax) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn IsBadStringPtrW<P0>(lpsz: P0, ucchmax: super::UINT_PTR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn IsBadStringPtrW(lpsz : windows_core::PCWSTR, ucchmax : super::UINT_PTR) -> windows_core::BOOL);
    unsafe { IsBadStringPtrW(lpsz.param().abi(), ucchmax) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn IsBadWritePtr(lp: Option<super::LPVOID>, ucb: super::UINT_PTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsBadWritePtr(lp : super::LPVOID, ucb : super::UINT_PTR) -> windows_core::BOOL);
    unsafe { IsBadWritePtr(lp.unwrap_or(core::mem::zeroed()) as _, ucb) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn IsNativeVhdBoot(nativevhdboot: super::PBOOL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsNativeVhdBoot(nativevhdboot : super::PBOOL) -> windows_core::BOOL);
    unsafe { IsNativeVhdBoot(nativevhdboot as _) }
}
#[inline]
pub unsafe fn IsSystemResumeAutomatic() -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn IsSystemResumeAutomatic() -> windows_core::BOOL);
    unsafe { IsSystemResumeAutomatic() }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn IsTextUnicode(lpv: *const core::ffi::c_void, isize: i32, lpiresult: Option<super::LPINT>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn IsTextUnicode(lpv : *const core::ffi::c_void, isize : i32, lpiresult : super::LPINT) -> windows_core::BOOL);
    unsafe { IsTextUnicode(lpv, isize, lpiresult.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn IsTokenUntrusted(tokenhandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn IsTokenUntrusted(tokenhandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { IsTokenUntrusted(tokenhandle) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn LoadModule<P0>(lpmodulename: P0, lpparameterblock: super::LPVOID) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadModule(lpmodulename : windows_core::PCSTR, lpparameterblock : super::LPVOID) -> super::DWORD);
    unsafe { LoadModule(lpmodulename.param().abi(), lpparameterblock) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn LoadPackagedLibrary<P0>(lpwlibfilename: P0, reserved: Option<super::DWORD>) -> super::HMODULE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn LoadPackagedLibrary(lpwlibfilename : windows_core::PCWSTR, reserved : super::DWORD) -> super::HMODULE);
    unsafe { LoadPackagedLibrary(lpwlibfilename.param().abi(), reserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn LocalAlloc(uflags: super::UINT, ubytes: super::SIZE_T) -> super::HLOCAL {
    windows_core::link!("kernel32.dll" "system" fn LocalAlloc(uflags : super::UINT, ubytes : super::SIZE_T) -> super::HLOCAL);
    unsafe { LocalAlloc(uflags, ubytes) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn LocalCompact(uminfree: super::UINT) -> super::SIZE_T {
    windows_core::link!("kernel32.dll" "system" fn LocalCompact(uminfree : super::UINT) -> super::SIZE_T);
    unsafe { LocalCompact(uminfree) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn LocalFlags(hmem: super::HLOCAL) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn LocalFlags(hmem : super::HLOCAL) -> super::UINT);
    unsafe { LocalFlags(hmem) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LocalFree(hmem: super::HLOCAL) -> super::HLOCAL {
    windows_core::link!("kernel32.dll" "system" fn LocalFree(hmem : super::HLOCAL) -> super::HLOCAL);
    unsafe { LocalFree(hmem as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn LocalHandle(pmem: super::LPCVOID) -> super::HLOCAL {
    windows_core::link!("kernel32.dll" "system" fn LocalHandle(pmem : super::LPCVOID) -> super::HLOCAL);
    unsafe { LocalHandle(pmem) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LocalLock(hmem: super::HLOCAL) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn LocalLock(hmem : super::HLOCAL) -> super::LPVOID);
    unsafe { LocalLock(hmem) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn LocalReAlloc(hmem: super::HLOCAL, ubytes: super::SIZE_T, uflags: super::UINT) -> super::HLOCAL {
    windows_core::link!("kernel32.dll" "system" fn LocalReAlloc(hmem : super::HLOCAL, ubytes : super::SIZE_T, uflags : super::UINT) -> super::HLOCAL);
    unsafe { LocalReAlloc(hmem as _, ubytes, uflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn LocalShrink(hmem: super::HLOCAL, cbnewsize: super::UINT) -> super::SIZE_T {
    windows_core::link!("kernel32.dll" "system" fn LocalShrink(hmem : super::HLOCAL, cbnewsize : super::UINT) -> super::SIZE_T);
    unsafe { LocalShrink(hmem, cbnewsize) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn LocalSize(hmem: super::HLOCAL) -> super::SIZE_T {
    windows_core::link!("kernel32.dll" "system" fn LocalSize(hmem : super::HLOCAL) -> super::SIZE_T);
    unsafe { LocalSize(hmem) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LocalUnlock(hmem: super::HLOCAL) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn LocalUnlock(hmem : super::HLOCAL) -> windows_core::BOOL);
    unsafe { LocalUnlock(hmem) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LocateXStateFeature(context: super::PCONTEXT, featureid: super::DWORD, length: Option<super::PDWORD>) -> super::PVOID {
    windows_core::link!("kernel32.dll" "system" fn LocateXStateFeature(context : super::PCONTEXT, featureid : super::DWORD, length : super::PDWORD) -> super::PVOID);
    unsafe { LocateXStateFeature(context, featureid, length.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LogonUserA<P0, P1, P2>(lpszusername: P0, lpszdomain: P1, lpszpassword: P2, dwlogontype: super::DWORD, dwlogonprovider: super::DWORD, phtoken: super::PHANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LogonUserA(lpszusername : windows_core::PCSTR, lpszdomain : windows_core::PCSTR, lpszpassword : windows_core::PCSTR, dwlogontype : super::DWORD, dwlogonprovider : super::DWORD, phtoken : super::PHANDLE) -> windows_core::BOOL);
    unsafe { LogonUserA(lpszusername.param().abi(), lpszdomain.param().abi(), lpszpassword.param().abi(), dwlogontype, dwlogonprovider, phtoken as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LogonUserExA<P0, P1, P2>(lpszusername: P0, lpszdomain: P1, lpszpassword: P2, dwlogontype: super::DWORD, dwlogonprovider: super::DWORD, phtoken: Option<super::PHANDLE>, pplogonsid: Option<*mut super::PSID>, ppprofilebuffer: Option<*mut super::PVOID>, pdwprofilelength: Option<super::LPDWORD>, pquotalimits: Option<super::PQUOTA_LIMITS>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LogonUserExA(lpszusername : windows_core::PCSTR, lpszdomain : windows_core::PCSTR, lpszpassword : windows_core::PCSTR, dwlogontype : super::DWORD, dwlogonprovider : super::DWORD, phtoken : super::PHANDLE, pplogonsid : *mut super::PSID, ppprofilebuffer : *mut super::PVOID, pdwprofilelength : super::LPDWORD, pquotalimits : super::PQUOTA_LIMITS) -> windows_core::BOOL);
    unsafe { LogonUserExA(lpszusername.param().abi(), lpszdomain.param().abi(), lpszpassword.param().abi(), dwlogontype, dwlogonprovider, phtoken.unwrap_or(core::mem::zeroed()) as _, pplogonsid.unwrap_or(core::mem::zeroed()) as _, ppprofilebuffer.unwrap_or(core::mem::zeroed()) as _, pdwprofilelength.unwrap_or(core::mem::zeroed()) as _, pquotalimits.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LogonUserExW<P0, P1, P2>(lpszusername: P0, lpszdomain: P1, lpszpassword: P2, dwlogontype: super::DWORD, dwlogonprovider: super::DWORD, phtoken: Option<super::PHANDLE>, pplogonsid: Option<*mut super::PSID>, ppprofilebuffer: Option<*mut super::PVOID>, pdwprofilelength: Option<super::LPDWORD>, pquotalimits: Option<super::PQUOTA_LIMITS>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LogonUserExW(lpszusername : windows_core::PCWSTR, lpszdomain : windows_core::PCWSTR, lpszpassword : windows_core::PCWSTR, dwlogontype : super::DWORD, dwlogonprovider : super::DWORD, phtoken : super::PHANDLE, pplogonsid : *mut super::PSID, ppprofilebuffer : *mut super::PVOID, pdwprofilelength : super::LPDWORD, pquotalimits : super::PQUOTA_LIMITS) -> windows_core::BOOL);
    unsafe { LogonUserExW(lpszusername.param().abi(), lpszdomain.param().abi(), lpszpassword.param().abi(), dwlogontype, dwlogonprovider, phtoken.unwrap_or(core::mem::zeroed()) as _, pplogonsid.unwrap_or(core::mem::zeroed()) as _, ppprofilebuffer.unwrap_or(core::mem::zeroed()) as _, pdwprofilelength.unwrap_or(core::mem::zeroed()) as _, pquotalimits.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LogonUserW<P0, P1, P2>(lpszusername: P0, lpszdomain: P1, lpszpassword: P2, dwlogontype: super::DWORD, dwlogonprovider: super::DWORD, phtoken: super::PHANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LogonUserW(lpszusername : windows_core::PCWSTR, lpszdomain : windows_core::PCWSTR, lpszpassword : windows_core::PCWSTR, dwlogontype : super::DWORD, dwlogonprovider : super::DWORD, phtoken : super::PHANDLE) -> windows_core::BOOL);
    unsafe { LogonUserW(lpszusername.param().abi(), lpszdomain.param().abi(), lpszpassword.param().abi(), dwlogontype, dwlogonprovider, phtoken as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn LookupAccountNameA<P0, P1>(lpsystemname: P0, lpaccountname: P1, sid: Option<super::PSID>, cbsid: super::LPDWORD, referenceddomainname: Option<windows_core::PSTR>, cchreferenceddomainname: super::LPDWORD, peuse: super::PSID_NAME_USE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LookupAccountNameA(lpsystemname : windows_core::PCSTR, lpaccountname : windows_core::PCSTR, sid : super::PSID, cbsid : super::LPDWORD, referenceddomainname : windows_core::PSTR, cchreferenceddomainname : super::LPDWORD, peuse : super::PSID_NAME_USE) -> windows_core::BOOL);
    unsafe { LookupAccountNameA(lpsystemname.param().abi(), lpaccountname.param().abi(), sid.unwrap_or(core::mem::zeroed()) as _, cbsid as _, referenceddomainname.unwrap_or(core::mem::zeroed()) as _, cchreferenceddomainname as _, peuse as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn LookupAccountNameW<P0, P1>(lpsystemname: P0, lpaccountname: P1, sid: Option<super::PSID>, cbsid: super::LPDWORD, referenceddomainname: Option<windows_core::PWSTR>, cchreferenceddomainname: super::LPDWORD, peuse: super::PSID_NAME_USE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LookupAccountNameW(lpsystemname : windows_core::PCWSTR, lpaccountname : windows_core::PCWSTR, sid : super::PSID, cbsid : super::LPDWORD, referenceddomainname : windows_core::PWSTR, cchreferenceddomainname : super::LPDWORD, peuse : super::PSID_NAME_USE) -> windows_core::BOOL);
    unsafe { LookupAccountNameW(lpsystemname.param().abi(), lpaccountname.param().abi(), sid.unwrap_or(core::mem::zeroed()) as _, cbsid as _, referenceddomainname.unwrap_or(core::mem::zeroed()) as _, cchreferenceddomainname as _, peuse as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn LookupAccountSidA<P0>(lpsystemname: P0, sid: super::PSID, name: Option<windows_core::PSTR>, cchname: super::LPDWORD, referenceddomainname: Option<windows_core::PSTR>, cchreferenceddomainname: super::LPDWORD, peuse: super::PSID_NAME_USE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LookupAccountSidA(lpsystemname : windows_core::PCSTR, sid : super::PSID, name : windows_core::PSTR, cchname : super::LPDWORD, referenceddomainname : windows_core::PSTR, cchreferenceddomainname : super::LPDWORD, peuse : super::PSID_NAME_USE) -> windows_core::BOOL);
    unsafe { LookupAccountSidA(lpsystemname.param().abi(), sid, name.unwrap_or(core::mem::zeroed()) as _, cchname as _, referenceddomainname.unwrap_or(core::mem::zeroed()) as _, cchreferenceddomainname as _, peuse as _) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn LookupAccountSidW<P0>(lpsystemname: P0, sid: super::PSID, name: Option<windows_core::PWSTR>, cchname: super::LPDWORD, referenceddomainname: Option<windows_core::PWSTR>, cchreferenceddomainname: super::LPDWORD, peuse: super::PSID_NAME_USE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LookupAccountSidW(lpsystemname : windows_core::PCWSTR, sid : super::PSID, name : windows_core::PWSTR, cchname : super::LPDWORD, referenceddomainname : windows_core::PWSTR, cchreferenceddomainname : super::LPDWORD, peuse : super::PSID_NAME_USE) -> windows_core::BOOL);
    unsafe { LookupAccountSidW(lpsystemname.param().abi(), sid, name.unwrap_or(core::mem::zeroed()) as _, cchname as _, referenceddomainname.unwrap_or(core::mem::zeroed()) as _, cchreferenceddomainname as _, peuse as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LookupPrivilegeDisplayNameA<P0, P1>(lpsystemname: P0, lpname: P1, lpdisplayname: Option<windows_core::PSTR>, cchdisplayname: super::LPDWORD, lplanguageid: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LookupPrivilegeDisplayNameA(lpsystemname : windows_core::PCSTR, lpname : windows_core::PCSTR, lpdisplayname : windows_core::PSTR, cchdisplayname : super::LPDWORD, lplanguageid : super::LPDWORD) -> windows_core::BOOL);
    unsafe { LookupPrivilegeDisplayNameA(lpsystemname.param().abi(), lpname.param().abi(), lpdisplayname.unwrap_or(core::mem::zeroed()) as _, cchdisplayname as _, lplanguageid as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn LookupPrivilegeDisplayNameW<P0, P1>(lpsystemname: P0, lpname: P1, lpdisplayname: Option<windows_core::PWSTR>, cchdisplayname: super::LPDWORD, lplanguageid: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LookupPrivilegeDisplayNameW(lpsystemname : windows_core::PCWSTR, lpname : windows_core::PCWSTR, lpdisplayname : windows_core::PWSTR, cchdisplayname : super::LPDWORD, lplanguageid : super::LPDWORD) -> windows_core::BOOL);
    unsafe { LookupPrivilegeDisplayNameW(lpsystemname.param().abi(), lpname.param().abi(), lpdisplayname.unwrap_or(core::mem::zeroed()) as _, cchdisplayname as _, lplanguageid as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LookupPrivilegeNameA<P0>(lpsystemname: P0, lpluid: super::PLUID, lpname: Option<windows_core::PSTR>, cchname: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LookupPrivilegeNameA(lpsystemname : windows_core::PCSTR, lpluid : super::PLUID, lpname : windows_core::PSTR, cchname : super::LPDWORD) -> windows_core::BOOL);
    unsafe { LookupPrivilegeNameA(lpsystemname.param().abi(), lpluid, lpname.unwrap_or(core::mem::zeroed()) as _, cchname as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LookupPrivilegeNameW<P0>(lpsystemname: P0, lpluid: super::PLUID, lpname: Option<windows_core::PWSTR>, cchname: super::LPDWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LookupPrivilegeNameW(lpsystemname : windows_core::PCWSTR, lpluid : super::PLUID, lpname : windows_core::PWSTR, cchname : super::LPDWORD) -> windows_core::BOOL);
    unsafe { LookupPrivilegeNameW(lpsystemname.param().abi(), lpluid, lpname.unwrap_or(core::mem::zeroed()) as _, cchname as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LookupPrivilegeValueA<P0, P1>(lpsystemname: P0, lpname: P1, lpluid: super::PLUID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LookupPrivilegeValueA(lpsystemname : windows_core::PCSTR, lpname : windows_core::PCSTR, lpluid : super::PLUID) -> windows_core::BOOL);
    unsafe { LookupPrivilegeValueA(lpsystemname.param().abi(), lpname.param().abi(), lpluid as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn LookupPrivilegeValueW<P0, P1>(lpsystemname: P0, lpname: P1, lpluid: super::PLUID) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn LookupPrivilegeValueW(lpsystemname : windows_core::PCWSTR, lpname : windows_core::PCWSTR, lpluid : super::PLUID) -> windows_core::BOOL);
    unsafe { LookupPrivilegeValueW(lpsystemname.param().abi(), lpname.param().abi(), lpluid as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MapUserPhysicalPagesScatter(virtualaddresses: *const super::PVOID, numberofpages: super::ULONG_PTR, pagearray: Option<super::PULONG_PTR>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn MapUserPhysicalPagesScatter(virtualaddresses : *const super::PVOID, numberofpages : super::ULONG_PTR, pagearray : super::PULONG_PTR) -> windows_core::BOOL);
    unsafe { MapUserPhysicalPagesScatter(virtualaddresses, numberofpages, pagearray.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MapViewOfFileExNuma(hfilemappingobject: super::HANDLE, dwdesiredaccess: super::DWORD, dwfileoffsethigh: super::DWORD, dwfileoffsetlow: super::DWORD, dwnumberofbytestomap: super::SIZE_T, lpbaseaddress: Option<super::LPVOID>, nndpreferred: super::DWORD) -> super::LPVOID {
    windows_core::link!("kernel32.dll" "system" fn MapViewOfFileExNuma(hfilemappingobject : super::HANDLE, dwdesiredaccess : super::DWORD, dwfileoffsethigh : super::DWORD, dwfileoffsetlow : super::DWORD, dwnumberofbytestomap : super::SIZE_T, lpbaseaddress : super::LPVOID, nndpreferred : super::DWORD) -> super::LPVOID);
    unsafe { MapViewOfFileExNuma(hfilemappingobject, dwdesiredaccess, dwfileoffsethigh, dwfileoffsetlow, dwnumberofbytestomap, lpbaseaddress.unwrap_or(core::mem::zeroed()) as _, nndpreferred) }
}
#[inline]
pub unsafe fn MoveFileA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn MoveFileA(lpexistingfilename : windows_core::PCSTR, lpnewfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { MoveFileA(lpexistingfilename.param().abi(), lpnewfilename.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MoveFileExA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, dwflags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn MoveFileExA(lpexistingfilename : windows_core::PCSTR, lpnewfilename : windows_core::PCSTR, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { MoveFileExA(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn MoveFileExW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, dwflags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn MoveFileExW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { MoveFileExW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MoveFileTransactedA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<super::LPVOID>, dwflags: super::DWORD, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn MoveFileTransactedA(lpexistingfilename : windows_core::PCSTR, lpnewfilename : windows_core::PCSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : super::LPVOID, dwflags : super::DWORD, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { MoveFileTransactedA(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, lpdata.unwrap_or(core::mem::zeroed()) as _, dwflags, htransaction) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MoveFileTransactedW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<super::LPVOID>, dwflags: super::DWORD, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn MoveFileTransactedW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : super::LPVOID, dwflags : super::DWORD, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { MoveFileTransactedW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, lpdata.unwrap_or(core::mem::zeroed()) as _, dwflags, htransaction) }
}
#[inline]
pub unsafe fn MoveFileW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn MoveFileW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { MoveFileW(lpexistingfilename.param().abi(), lpnewfilename.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MoveFileWithProgressA<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<super::LPVOID>, dwflags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn MoveFileWithProgressA(lpexistingfilename : windows_core::PCSTR, lpnewfilename : windows_core::PCSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : super::LPVOID, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { MoveFileWithProgressA(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, lpdata.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn MoveFileWithProgressW<P0, P1>(lpexistingfilename: P0, lpnewfilename: P1, lpprogressroutine: LPPROGRESS_ROUTINE, lpdata: Option<super::LPVOID>, dwflags: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn MoveFileWithProgressW(lpexistingfilename : windows_core::PCWSTR, lpnewfilename : windows_core::PCWSTR, lpprogressroutine : LPPROGRESS_ROUTINE, lpdata : super::LPVOID, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { MoveFileWithProgressW(lpexistingfilename.param().abi(), lpnewfilename.param().abi(), lpprogressroutine, lpdata.unwrap_or(core::mem::zeroed()) as _, dwflags) }
}
#[inline]
pub unsafe fn MulDiv(nnumber: i32, nnumerator: i32, ndenominator: i32) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn MulDiv(nnumber : i32, nnumerator : i32, ndenominator : i32) -> i32);
    unsafe { MulDiv(nnumber, nnumerator, ndenominator) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn NotifyChangeEventLog(heventlog: super::HANDLE, hevent: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn NotifyChangeEventLog(heventlog : super::HANDLE, hevent : super::HANDLE) -> windows_core::BOOL);
    unsafe { NotifyChangeEventLog(heventlog, hevent) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ObjectCloseAuditAlarmA<P0>(subsystemname: P0, handleid: super::LPVOID, generateonclose: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ObjectCloseAuditAlarmA(subsystemname : windows_core::PCSTR, handleid : super::LPVOID, generateonclose : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ObjectCloseAuditAlarmA(subsystemname.param().abi(), handleid, generateonclose.into()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn ObjectDeleteAuditAlarmA<P0>(subsystemname: P0, handleid: super::LPVOID, generateonclose: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ObjectDeleteAuditAlarmA(subsystemname : windows_core::PCSTR, handleid : super::LPVOID, generateonclose : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ObjectDeleteAuditAlarmA(subsystemname.param().abi(), handleid, generateonclose.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ObjectOpenAuditAlarmA<P0, P2, P3>(subsystemname: P0, handleid: super::LPVOID, objecttypename: P2, objectname: P3, psecuritydescriptor: super::PSECURITY_DESCRIPTOR, clienttoken: super::HANDLE, desiredaccess: super::DWORD, grantedaccess: super::DWORD, privileges: Option<super::PPRIVILEGE_SET>, objectcreation: bool, accessgranted: bool, generateonclose: super::LPBOOL) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ObjectOpenAuditAlarmA(subsystemname : windows_core::PCSTR, handleid : super::LPVOID, objecttypename : windows_core::PCSTR, objectname : windows_core::PCSTR, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, clienttoken : super::HANDLE, desiredaccess : super::DWORD, grantedaccess : super::DWORD, privileges : super::PPRIVILEGE_SET, objectcreation : windows_core::BOOL, accessgranted : windows_core::BOOL, generateonclose : super::LPBOOL) -> windows_core::BOOL);
    unsafe { ObjectOpenAuditAlarmA(subsystemname.param().abi(), handleid, objecttypename.param().abi(), objectname.param().abi(), psecuritydescriptor, clienttoken, desiredaccess, grantedaccess, privileges.unwrap_or(core::mem::zeroed()) as _, objectcreation.into(), accessgranted.into(), generateonclose as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ObjectPrivilegeAuditAlarmA<P0>(subsystemname: P0, handleid: super::LPVOID, clienttoken: super::HANDLE, desiredaccess: super::DWORD, privileges: super::PPRIVILEGE_SET, accessgranted: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn ObjectPrivilegeAuditAlarmA(subsystemname : windows_core::PCSTR, handleid : super::LPVOID, clienttoken : super::HANDLE, desiredaccess : super::DWORD, privileges : super::PPRIVILEGE_SET, accessgranted : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { ObjectPrivilegeAuditAlarmA(subsystemname.param().abi(), handleid, clienttoken, desiredaccess, privileges, accessgranted.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenBackupEventLogA<P0, P1>(lpuncservername: P0, lpfilename: P1) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn OpenBackupEventLogA(lpuncservername : windows_core::PCSTR, lpfilename : windows_core::PCSTR) -> super::HANDLE);
    unsafe { OpenBackupEventLogA(lpuncservername.param().abi(), lpfilename.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenBackupEventLogW<P0, P1>(lpuncservername: P0, lpfilename: P1) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn OpenBackupEventLogW(lpuncservername : windows_core::PCWSTR, lpfilename : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { OpenBackupEventLogW(lpuncservername.param().abi(), lpfilename.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenCommPort(uportnumber: super::ULONG, dwdesiredaccess: super::DWORD, dwflagsandattributes: super::DWORD) -> super::HANDLE {
    windows_core::link!("api-ms-win-core-comm-l1-1-1.dll" "system" fn OpenCommPort(uportnumber : super::ULONG, dwdesiredaccess : super::DWORD, dwflagsandattributes : super::DWORD) -> super::HANDLE);
    unsafe { OpenCommPort(uportnumber, dwdesiredaccess, dwflagsandattributes) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenEncryptedFileRawA<P0>(lpfilename: P0, ulflags: super::ULONG, pvcontext: *mut super::PVOID) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn OpenEncryptedFileRawA(lpfilename : windows_core::PCSTR, ulflags : super::ULONG, pvcontext : *mut super::PVOID) -> super::DWORD);
    unsafe { OpenEncryptedFileRawA(lpfilename.param().abi(), ulflags, pvcontext as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenEncryptedFileRawW<P0>(lpfilename: P0, ulflags: super::ULONG, pvcontext: *mut super::PVOID) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn OpenEncryptedFileRawW(lpfilename : windows_core::PCWSTR, ulflags : super::ULONG, pvcontext : *mut super::PVOID) -> super::DWORD);
    unsafe { OpenEncryptedFileRawW(lpfilename.param().abi(), ulflags, pvcontext as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenEventLogA<P0, P1>(lpuncservername: P0, lpsourcename: P1) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn OpenEventLogA(lpuncservername : windows_core::PCSTR, lpsourcename : windows_core::PCSTR) -> super::HANDLE);
    unsafe { OpenEventLogA(lpuncservername.param().abi(), lpsourcename.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn OpenEventLogW<P0, P1>(lpuncservername: P0, lpsourcename: P1) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn OpenEventLogW(lpuncservername : windows_core::PCWSTR, lpsourcename : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { OpenEventLogW(lpuncservername.param().abi(), lpsourcename.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn OpenFile<P0>(lpfilename: P0, lpreopenbuff: LPOFSTRUCT, ustyle: super::UINT) -> super::HFILE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenFile(lpfilename : windows_core::PCSTR, lpreopenbuff : LPOFSTRUCT, ustyle : super::UINT) -> super::HFILE);
    unsafe { OpenFile(lpfilename.param().abi(), lpreopenbuff as _, ustyle) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenFileById(hvolumehint: super::HANDLE, lpfileid: LPFILE_ID_DESCRIPTOR, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD, lpsecurityattributes: Option<super::LPSECURITY_ATTRIBUTES>, dwflagsandattributes: super::DWORD) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn OpenFileById(hvolumehint : super::HANDLE, lpfileid : LPFILE_ID_DESCRIPTOR, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, lpsecurityattributes : super::LPSECURITY_ATTRIBUTES, dwflagsandattributes : super::DWORD) -> super::HANDLE);
    unsafe { OpenFileById(hvolumehint, lpfileid, dwdesiredaccess, dwsharemode, lpsecurityattributes.unwrap_or(core::mem::zeroed()) as _, dwflagsandattributes) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenFileMappingA<P2>(dwdesiredaccess: super::DWORD, binherithandle: bool, lpname: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenFileMappingA(dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, lpname : windows_core::PCSTR) -> super::HANDLE);
    unsafe { OpenFileMappingA(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenJobObjectA<P2>(dwdesiredaccess: super::DWORD, binherithandle: bool, lpname: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenJobObjectA(dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, lpname : windows_core::PCSTR) -> super::HANDLE);
    unsafe { OpenJobObjectA(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenMutexA<P2>(dwdesiredaccess: super::DWORD, binherithandle: bool, lpname: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenMutexA(dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, lpname : windows_core::PCSTR) -> super::HANDLE);
    unsafe { OpenMutexA(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn OpenPrivateNamespaceA<P1>(lpboundarydescriptor: super::LPVOID, lpaliasprefix: P1) -> super::HANDLE
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenPrivateNamespaceA(lpboundarydescriptor : super::LPVOID, lpaliasprefix : windows_core::PCSTR) -> super::HANDLE);
    unsafe { OpenPrivateNamespaceA(lpboundarydescriptor, lpaliasprefix.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenSemaphoreA<P2>(dwdesiredaccess: super::DWORD, binherithandle: bool, lpname: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenSemaphoreA(dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, lpname : windows_core::PCSTR) -> super::HANDLE);
    unsafe { OpenSemaphoreA(dwdesiredaccess, binherithandle.into(), lpname.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn OpenWaitableTimerA<P2>(dwdesiredaccess: super::DWORD, binherithandle: bool, lptimername: P2) -> super::HANDLE
where
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn OpenWaitableTimerA(dwdesiredaccess : super::DWORD, binherithandle : windows_core::BOOL, lptimername : windows_core::PCSTR) -> super::HANDLE);
    unsafe { OpenWaitableTimerA(dwdesiredaccess, binherithandle.into(), lptimername.param().abi()) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn OperationEnd(operationendparams: *const OPERATION_END_PARAMETERS) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn OperationEnd(operationendparams : *const OPERATION_END_PARAMETERS) -> windows_core::BOOL);
    unsafe { OperationEnd(operationendparams) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn OperationStart(operationstartparams: *const OPERATION_START_PARAMETERS) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn OperationStart(operationstartparams : *const OPERATION_START_PARAMETERS) -> windows_core::BOOL);
    unsafe { OperationStart(operationstartparams) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PowerClearRequest(powerrequest: super::HANDLE, requesttype: super::POWER_REQUEST_TYPE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn PowerClearRequest(powerrequest : super::HANDLE, requesttype : super::POWER_REQUEST_TYPE) -> windows_core::BOOL);
    unsafe { PowerClearRequest(powerrequest, requesttype) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PowerCreateRequest(context: super::PREASON_CONTEXT) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn PowerCreateRequest(context : super::PREASON_CONTEXT) -> super::HANDLE);
    unsafe { PowerCreateRequest(context) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PowerSetRequest(powerrequest: super::HANDLE, requesttype: super::POWER_REQUEST_TYPE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn PowerSetRequest(powerrequest : super::HANDLE, requesttype : super::POWER_REQUEST_TYPE) -> windows_core::BOOL);
    unsafe { PowerSetRequest(powerrequest, requesttype) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PrepareTape(hdevice: super::HANDLE, dwoperation: super::DWORD, bimmediate: bool) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn PrepareTape(hdevice : super::HANDLE, dwoperation : super::DWORD, bimmediate : windows_core::BOOL) -> super::DWORD);
    unsafe { PrepareTape(hdevice, dwoperation, bimmediate.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PrivilegedServiceAuditAlarmA<P0, P1>(subsystemname: P0, servicename: P1, clienttoken: super::HANDLE, privileges: super::PPRIVILEGE_SET, accessgranted: bool) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn PrivilegedServiceAuditAlarmA(subsystemname : windows_core::PCSTR, servicename : windows_core::PCSTR, clienttoken : super::HANDLE, privileges : super::PPRIVILEGE_SET, accessgranted : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { PrivilegedServiceAuditAlarmA(subsystemname.param().abi(), servicename.param().abi(), clienttoken, privileges, accessgranted.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn PulseEvent(hevent: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn PulseEvent(hevent : super::HANDLE) -> windows_core::BOOL);
    unsafe { PulseEvent(hevent) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn PurgeComm(hfile: super::HANDLE, dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn PurgeComm(hfile : super::HANDLE, dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { PurgeComm(hfile, dwflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryActCtxSettingsW<P2, P3>(dwflags: Option<super::DWORD>, hactctx: Option<super::HANDLE>, settingsnamespace: P2, settingname: P3, pvbuffer: Option<windows_core::PWSTR>, dwbuffer: super::SIZE_T, pdwwrittenorrequired: Option<*mut super::SIZE_T>) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn QueryActCtxSettingsW(dwflags : super::DWORD, hactctx : super::HANDLE, settingsnamespace : windows_core::PCWSTR, settingname : windows_core::PCWSTR, pvbuffer : windows_core::PWSTR, dwbuffer : super::SIZE_T, pdwwrittenorrequired : *mut super::SIZE_T) -> windows_core::BOOL);
    unsafe { QueryActCtxSettingsW(dwflags.unwrap_or(core::mem::zeroed()) as _, hactctx.unwrap_or(core::mem::zeroed()) as _, settingsnamespace.param().abi(), settingname.param().abi(), pvbuffer.unwrap_or(core::mem::zeroed()) as _, dwbuffer, pdwwrittenorrequired.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryActCtxW(dwflags: super::DWORD, hactctx: super::HANDLE, pvsubinstance: Option<super::PVOID>, ulinfoclass: super::ULONG, pvbuffer: Option<super::PVOID>, cbbuffer: super::SIZE_T, pcbwrittenorrequired: Option<*mut super::SIZE_T>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryActCtxW(dwflags : super::DWORD, hactctx : super::HANDLE, pvsubinstance : super::PVOID, ulinfoclass : super::ULONG, pvbuffer : super::PVOID, cbbuffer : super::SIZE_T, pcbwrittenorrequired : *mut super::SIZE_T) -> windows_core::BOOL);
    unsafe { QueryActCtxW(dwflags, hactctx, pvsubinstance.unwrap_or(core::mem::zeroed()) as _, ulinfoclass, pvbuffer.unwrap_or(core::mem::zeroed()) as _, cbbuffer, pcbwrittenorrequired.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn QueryDosDeviceA<P0>(lpdevicename: P0, lptargetpath: Option<windows_core::PSTR>, ucchmax: super::DWORD) -> super::DWORD
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn QueryDosDeviceA(lpdevicename : windows_core::PCSTR, lptargetpath : windows_core::PSTR, ucchmax : super::DWORD) -> super::DWORD);
    unsafe { QueryDosDeviceA(lpdevicename.param().abi(), lptargetpath.unwrap_or(core::mem::zeroed()) as _, ucchmax) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryFullProcessImageNameA(hprocess: super::HANDLE, dwflags: super::DWORD, lpexename: windows_core::PSTR, lpdwsize: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryFullProcessImageNameA(hprocess : super::HANDLE, dwflags : super::DWORD, lpexename : windows_core::PSTR, lpdwsize : super::PDWORD) -> windows_core::BOOL);
    unsafe { QueryFullProcessImageNameA(hprocess, dwflags, lpexename, lpdwsize as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryFullProcessImageNameW(hprocess: super::HANDLE, dwflags: super::DWORD, lpexename: windows_core::PWSTR, lpdwsize: super::PDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryFullProcessImageNameW(hprocess : super::HANDLE, dwflags : super::DWORD, lpexename : windows_core::PWSTR, lpdwsize : super::PDWORD) -> windows_core::BOOL);
    unsafe { QueryFullProcessImageNameW(hprocess, dwflags, lpexename, lpdwsize as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryThreadProfiling(threadhandle: super::HANDLE, enabled: super::PBOOLEAN) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn QueryThreadProfiling(threadhandle : super::HANDLE, enabled : super::PBOOLEAN) -> super::DWORD);
    unsafe { QueryThreadProfiling(threadhandle, enabled as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn QueryUmsThreadInformation(umsthread: PUMS_CONTEXT, umsthreadinfoclass: UMS_THREAD_INFO_CLASS, umsthreadinformation: super::PVOID, umsthreadinformationlength: super::ULONG, returnlength: Option<super::PULONG>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn QueryUmsThreadInformation(umsthread : PUMS_CONTEXT, umsthreadinfoclass : UMS_THREAD_INFO_CLASS, umsthreadinformation : super::PVOID, umsthreadinformationlength : super::ULONG, returnlength : super::PULONG) -> windows_core::BOOL);
    unsafe { QueryUmsThreadInformation(umsthread, umsthreadinfoclass, umsthreadinformation as _, umsthreadinformationlength, returnlength.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig: super::PCUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> super::DWORD {
    windows_core::link!("api-ms-win-core-backgroundtask-l1-1-0.dll" "system" fn RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig : super::PCUSTOM_SYSTEM_EVENT_TRIGGER_CONFIG) -> super::DWORD);
    unsafe { RaiseCustomSystemEventTrigger(customsystemeventtriggerconfig) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReOpenFile(horiginalfile: super::HANDLE, dwdesiredaccess: super::DWORD, dwsharemode: super::DWORD, dwflagsandattributes: super::DWORD) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn ReOpenFile(horiginalfile : super::HANDLE, dwdesiredaccess : super::DWORD, dwsharemode : super::DWORD, dwflagsandattributes : super::DWORD) -> super::HANDLE);
    unsafe { ReOpenFile(horiginalfile, dwdesiredaccess, dwsharemode, dwflagsandattributes) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadDirectoryChangesExW(hdirectory: super::HANDLE, lpbuffer: super::LPVOID, nbufferlength: super::DWORD, bwatchsubtree: bool, dwnotifyfilter: super::DWORD, lpbytesreturned: Option<super::LPDWORD>, lpoverlapped: Option<super::LPOVERLAPPED>, lpcompletionroutine: super::LPOVERLAPPED_COMPLETION_ROUTINE, readdirectorynotifyinformationclass: super::READ_DIRECTORY_NOTIFY_INFORMATION_CLASS) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadDirectoryChangesExW(hdirectory : super::HANDLE, lpbuffer : super::LPVOID, nbufferlength : super::DWORD, bwatchsubtree : windows_core::BOOL, dwnotifyfilter : super::DWORD, lpbytesreturned : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED, lpcompletionroutine : super::LPOVERLAPPED_COMPLETION_ROUTINE, readdirectorynotifyinformationclass : super::READ_DIRECTORY_NOTIFY_INFORMATION_CLASS) -> windows_core::BOOL);
    unsafe { ReadDirectoryChangesExW(hdirectory, lpbuffer as _, nbufferlength, bwatchsubtree.into(), dwnotifyfilter, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine, readdirectorynotifyinformationclass) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadDirectoryChangesW(hdirectory: super::HANDLE, lpbuffer: super::LPVOID, nbufferlength: super::DWORD, bwatchsubtree: bool, dwnotifyfilter: super::DWORD, lpbytesreturned: Option<super::LPDWORD>, lpoverlapped: Option<super::LPOVERLAPPED>, lpcompletionroutine: super::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ReadDirectoryChangesW(hdirectory : super::HANDLE, lpbuffer : super::LPVOID, nbufferlength : super::DWORD, bwatchsubtree : windows_core::BOOL, dwnotifyfilter : super::DWORD, lpbytesreturned : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED, lpcompletionroutine : super::LPOVERLAPPED_COMPLETION_ROUTINE) -> windows_core::BOOL);
    unsafe { ReadDirectoryChangesW(hdirectory, lpbuffer as _, nbufferlength, bwatchsubtree.into(), dwnotifyfilter, lpbytesreturned.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, lpcompletionroutine) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadEncryptedFileRaw(pfexportcallback: PFE_EXPORT_FUNC, pvcallbackcontext: Option<super::PVOID>, pvcontext: super::PVOID) -> super::DWORD {
    windows_core::link!("advapi32.dll" "system" fn ReadEncryptedFileRaw(pfexportcallback : PFE_EXPORT_FUNC, pvcallbackcontext : super::PVOID, pvcontext : super::PVOID) -> super::DWORD);
    unsafe { ReadEncryptedFileRaw(pfexportcallback, pvcallbackcontext.unwrap_or(core::mem::zeroed()) as _, pvcontext) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadEventLogA(heventlog: super::HANDLE, dwreadflags: super::DWORD, dwrecordoffset: super::DWORD, lpbuffer: super::LPVOID, nnumberofbytestoread: super::DWORD, pnbytesread: *mut super::DWORD, pnminnumberofbytesneeded: *mut super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ReadEventLogA(heventlog : super::HANDLE, dwreadflags : super::DWORD, dwrecordoffset : super::DWORD, lpbuffer : super::LPVOID, nnumberofbytestoread : super::DWORD, pnbytesread : *mut super::DWORD, pnminnumberofbytesneeded : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { ReadEventLogA(heventlog, dwreadflags, dwrecordoffset, lpbuffer as _, nnumberofbytestoread, pnbytesread as _, pnminnumberofbytesneeded as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadEventLogW(heventlog: super::HANDLE, dwreadflags: super::DWORD, dwrecordoffset: super::DWORD, lpbuffer: super::LPVOID, nnumberofbytestoread: super::DWORD, pnbytesread: *mut super::DWORD, pnminnumberofbytesneeded: *mut super::DWORD) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ReadEventLogW(heventlog : super::HANDLE, dwreadflags : super::DWORD, dwrecordoffset : super::DWORD, lpbuffer : super::LPVOID, nnumberofbytestoread : super::DWORD, pnbytesread : *mut super::DWORD, pnminnumberofbytesneeded : *mut super::DWORD) -> windows_core::BOOL);
    unsafe { ReadEventLogW(heventlog, dwreadflags, dwrecordoffset, lpbuffer as _, nnumberofbytestoread, pnbytesread as _, pnminnumberofbytesneeded as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReadThreadProfilingData(performancedatahandle: super::HANDLE, flags: super::DWORD, performancedata: super::PPERFORMANCE_DATA) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn ReadThreadProfilingData(performancedatahandle : super::HANDLE, flags : super::DWORD, performancedata : super::PPERFORMANCE_DATA) -> super::DWORD);
    unsafe { ReadThreadProfilingData(performancedatahandle, flags, performancedata as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RegisterApplicationRecoveryCallback(precoveycallback: APPLICATION_RECOVERY_CALLBACK, pvparameter: Option<super::PVOID>, dwpinginterval: super::DWORD, dwflags: super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn RegisterApplicationRecoveryCallback(precoveycallback : APPLICATION_RECOVERY_CALLBACK, pvparameter : super::PVOID, dwpinginterval : super::DWORD, dwflags : super::DWORD) -> windows_core::HRESULT);
    unsafe { RegisterApplicationRecoveryCallback(precoveycallback, pvparameter.unwrap_or(core::mem::zeroed()) as _, dwpinginterval, dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RegisterApplicationRestart<P0>(pwzcommandline: P0, dwflags: super::DWORD) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn RegisterApplicationRestart(pwzcommandline : windows_core::PCWSTR, dwflags : super::DWORD) -> windows_core::HRESULT);
    unsafe { RegisterApplicationRestart(pwzcommandline.param().abi(), dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RegisterEventSourceA<P0, P1>(lpuncservername: P0, lpsourcename: P1) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegisterEventSourceA(lpuncservername : windows_core::PCSTR, lpsourcename : windows_core::PCSTR) -> super::HANDLE);
    unsafe { RegisterEventSourceA(lpuncservername.param().abi(), lpsourcename.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RegisterEventSourceW<P0, P1>(lpuncservername: P0, lpsourcename: P1) -> super::HANDLE
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn RegisterEventSourceW(lpuncservername : windows_core::PCWSTR, lpsourcename : windows_core::PCWSTR) -> super::HANDLE);
    unsafe { RegisterEventSourceW(lpuncservername.param().abi(), lpsourcename.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RegisterWaitForSingleObject(phnewwaitobject: super::PHANDLE, hobject: super::HANDLE, callback: super::WAITORTIMERCALLBACK, context: Option<super::PVOID>, dwmilliseconds: super::ULONG, dwflags: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn RegisterWaitForSingleObject(phnewwaitobject : super::PHANDLE, hobject : super::HANDLE, callback : super::WAITORTIMERCALLBACK, context : super::PVOID, dwmilliseconds : super::ULONG, dwflags : super::ULONG) -> windows_core::BOOL);
    unsafe { RegisterWaitForSingleObject(phnewwaitobject as _, hobject, callback, context.unwrap_or(core::mem::zeroed()) as _, dwmilliseconds, dwflags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ReleaseActCtx(hactctx: super::HANDLE) {
    windows_core::link!("kernel32.dll" "system" fn ReleaseActCtx(hactctx : super::HANDLE));
    unsafe { ReleaseActCtx(hactctx as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RemoveDirectoryTransactedA<P0>(lppathname: P0, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn RemoveDirectoryTransactedA(lppathname : windows_core::PCSTR, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { RemoveDirectoryTransactedA(lppathname.param().abi(), htransaction) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RemoveDirectoryTransactedW<P0>(lppathname: P0, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn RemoveDirectoryTransactedW(lppathname : windows_core::PCWSTR, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { RemoveDirectoryTransactedW(lppathname.param().abi(), htransaction) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RemoveSecureMemoryCacheCallback(pfncallback: super::PSECURE_MEMORY_CACHE_CALLBACK) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn RemoveSecureMemoryCacheCallback(pfncallback : super::PSECURE_MEMORY_CACHE_CALLBACK) -> windows_core::BOOL);
    unsafe { RemoveSecureMemoryCacheCallback(pfncallback) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn ReplaceFileA<P0, P1, P2>(lpreplacedfilename: P0, lpreplacementfilename: P1, lpbackupfilename: P2, dwreplaceflags: super::DWORD, lpexclude: Option<super::LPVOID>, lpreserved: Option<super::LPVOID>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn ReplaceFileA(lpreplacedfilename : windows_core::PCSTR, lpreplacementfilename : windows_core::PCSTR, lpbackupfilename : windows_core::PCSTR, dwreplaceflags : super::DWORD, lpexclude : super::LPVOID, lpreserved : super::LPVOID) -> windows_core::BOOL);
    unsafe { ReplaceFileA(lpreplacedfilename.param().abi(), lpreplacementfilename.param().abi(), lpbackupfilename.param().abi(), dwreplaceflags, lpexclude.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn ReplaceFileW<P0, P1, P2>(lpreplacedfilename: P0, lpreplacementfilename: P1, lpbackupfilename: P2, dwreplaceflags: super::DWORD, lpexclude: Option<super::LPVOID>, lpreserved: Option<super::LPVOID>) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn ReplaceFileW(lpreplacedfilename : windows_core::PCWSTR, lpreplacementfilename : windows_core::PCWSTR, lpbackupfilename : windows_core::PCWSTR, dwreplaceflags : super::DWORD, lpexclude : super::LPVOID, lpreserved : super::LPVOID) -> windows_core::BOOL);
    unsafe { ReplaceFileW(lpreplacedfilename.param().abi(), lpreplacementfilename.param().abi(), lpbackupfilename.param().abi(), dwreplaceflags, lpexclude.unwrap_or(core::mem::zeroed()) as _, lpreserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn ReplacePartitionUnit<P0, P1>(targetpartition: P0, sparepartition: P1, flags: super::ULONG) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn ReplacePartitionUnit(targetpartition : windows_core::PCWSTR, sparepartition : windows_core::PCWSTR, flags : super::ULONG) -> windows_core::BOOL);
    unsafe { ReplacePartitionUnit(targetpartition.param().abi(), sparepartition.param().abi(), flags) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReportEventA(heventlog: super::HANDLE, wtype: super::WORD, wcategory: super::WORD, dweventid: super::DWORD, lpusersid: Option<super::PSID>, dwdatasize: super::DWORD, lpstrings: Option<&[windows_core::PCSTR]>, lprawdata: Option<super::LPVOID>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ReportEventA(heventlog : super::HANDLE, wtype : super::WORD, wcategory : super::WORD, dweventid : super::DWORD, lpusersid : super::PSID, wnumstrings : super::WORD, dwdatasize : super::DWORD, lpstrings : *const windows_core::PCSTR, lprawdata : super::LPVOID) -> windows_core::BOOL);
    unsafe { ReportEventA(heventlog, wtype, wcategory, dweventid, lpusersid.unwrap_or(core::mem::zeroed()) as _, lpstrings.map_or(0, |slice| slice.len().try_into().unwrap()), dwdatasize, lpstrings.map_or(core::ptr::null(), |slice| slice.as_ptr()), lprawdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn ReportEventW(heventlog: super::HANDLE, wtype: super::WORD, wcategory: super::WORD, dweventid: super::DWORD, lpusersid: Option<super::PSID>, dwdatasize: super::DWORD, lpstrings: Option<&[windows_core::PCWSTR]>, lprawdata: Option<super::LPVOID>) -> windows_core::BOOL {
    windows_core::link!("advapi32.dll" "system" fn ReportEventW(heventlog : super::HANDLE, wtype : super::WORD, wcategory : super::WORD, dweventid : super::DWORD, lpusersid : super::PSID, wnumstrings : super::WORD, dwdatasize : super::DWORD, lpstrings : *const windows_core::PCWSTR, lprawdata : super::LPVOID) -> windows_core::BOOL);
    unsafe { ReportEventW(heventlog, wtype, wcategory, dweventid, lpusersid.unwrap_or(core::mem::zeroed()) as _, lpstrings.map_or(0, |slice| slice.len().try_into().unwrap()), dwdatasize, lpstrings.map_or(core::ptr::null(), |slice| slice.as_ptr()), lprawdata.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RequestDeviceWakeup(hdevice: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn RequestDeviceWakeup(hdevice : super::HANDLE) -> windows_core::BOOL);
    unsafe { RequestDeviceWakeup(hdevice) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn RequestWakeupLatency(latency: super::LATENCY_TIME) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn RequestWakeupLatency(latency : super::LATENCY_TIME) -> windows_core::BOOL);
    unsafe { RequestWakeupLatency(latency) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetCommBreak(hfile: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetCommBreak(hfile : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetCommBreak(hfile) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetCommConfig(hcommdev: super::HANDLE, lpcc: LPCOMMCONFIG, dwsize: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetCommConfig(hcommdev : super::HANDLE, lpcc : LPCOMMCONFIG, dwsize : super::DWORD) -> windows_core::BOOL);
    unsafe { SetCommConfig(hcommdev, lpcc, dwsize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetCommMask(hfile: super::HANDLE, dwevtmask: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetCommMask(hfile : super::HANDLE, dwevtmask : super::DWORD) -> windows_core::BOOL);
    unsafe { SetCommMask(hfile, dwevtmask) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetCommState(hfile: super::HANDLE, lpdcb: LPDCB) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetCommState(hfile : super::HANDLE, lpdcb : LPDCB) -> windows_core::BOOL);
    unsafe { SetCommState(hfile, lpdcb) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetCommTimeouts(hfile: super::HANDLE, lpcommtimeouts: LPCOMMTIMEOUTS) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetCommTimeouts(hfile : super::HANDLE, lpcommtimeouts : LPCOMMTIMEOUTS) -> windows_core::BOOL);
    unsafe { SetCommTimeouts(hfile, lpcommtimeouts) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetDefaultCommConfigA<P0>(lpszname: P0, lpcc: LPCOMMCONFIG, dwsize: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetDefaultCommConfigA(lpszname : windows_core::PCSTR, lpcc : LPCOMMCONFIG, dwsize : super::DWORD) -> windows_core::BOOL);
    unsafe { SetDefaultCommConfigA(lpszname.param().abi(), lpcc, dwsize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetDefaultCommConfigW<P0>(lpszname: P0, lpcc: LPCOMMCONFIG, dwsize: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetDefaultCommConfigW(lpszname : windows_core::PCWSTR, lpcc : LPCOMMCONFIG, dwsize : super::DWORD) -> windows_core::BOOL);
    unsafe { SetDefaultCommConfigW(lpszname.param().abi(), lpcc, dwsize) }
}
#[inline]
pub unsafe fn SetDllDirectoryA<P0>(lppathname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetDllDirectoryA(lppathname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetDllDirectoryA(lppathname.param().abi()) }
}
#[inline]
pub unsafe fn SetDllDirectoryW<P0>(lppathname: P0) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetDllDirectoryW(lppathname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetDllDirectoryW(lppathname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetEnvironmentStringsA(newenvironment: super::LPCH) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetEnvironmentStringsA(newenvironment : super::LPCH) -> windows_core::BOOL);
    unsafe { SetEnvironmentStringsA(newenvironment) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetFileAttributesTransactedA<P0>(lpfilename: P0, dwfileattributes: super::DWORD, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFileAttributesTransactedA(lpfilename : windows_core::PCSTR, dwfileattributes : super::DWORD, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetFileAttributesTransactedA(lpfilename.param().abi(), dwfileattributes, htransaction) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetFileAttributesTransactedW<P0>(lpfilename: P0, dwfileattributes: super::DWORD, htransaction: super::HANDLE) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFileAttributesTransactedW(lpfilename : windows_core::PCWSTR, dwfileattributes : super::DWORD, htransaction : super::HANDLE) -> windows_core::BOOL);
    unsafe { SetFileAttributesTransactedW(lpfilename.param().abi(), dwfileattributes, htransaction) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetFileBandwidthReservation(hfile: super::HANDLE, nperiodmilliseconds: super::DWORD, nbytesperperiod: super::DWORD, bdiscardable: bool, lptransfersize: super::LPDWORD, lpnumoutstandingrequests: super::LPDWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetFileBandwidthReservation(hfile : super::HANDLE, nperiodmilliseconds : super::DWORD, nbytesperperiod : super::DWORD, bdiscardable : windows_core::BOOL, lptransfersize : super::LPDWORD, lpnumoutstandingrequests : super::LPDWORD) -> windows_core::BOOL);
    unsafe { SetFileBandwidthReservation(hfile, nperiodmilliseconds, nbytesperperiod, bdiscardable.into(), lptransfersize as _, lpnumoutstandingrequests as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetFileCompletionNotificationModes(filehandle: super::HANDLE, flags: super::UCHAR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetFileCompletionNotificationModes(filehandle : super::HANDLE, flags : super::UCHAR) -> windows_core::BOOL);
    unsafe { SetFileCompletionNotificationModes(filehandle, flags) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetFileSecurityA<P0>(lpfilename: P0, securityinformation: super::SECURITY_INFORMATION, psecuritydescriptor: super::PSECURITY_DESCRIPTOR) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("advapi32.dll" "system" fn SetFileSecurityA(lpfilename : windows_core::PCSTR, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> windows_core::BOOL);
    unsafe { SetFileSecurityA(lpfilename.param().abi(), securityinformation, psecuritydescriptor) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetFileShortNameA<P1>(hfile: super::HANDLE, lpshortname: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFileShortNameA(hfile : super::HANDLE, lpshortname : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetFileShortNameA(hfile, lpshortname.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetFileShortNameW<P1>(hfile: super::HANDLE, lpshortname: P1) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFileShortNameW(hfile : super::HANDLE, lpshortname : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetFileShortNameW(hfile, lpshortname.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableA<P0, P1>(lpname: P0, lpguid: P1, pvalue: Option<super::PVOID>, nsize: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableA(lpname : windows_core::PCSTR, lpguid : windows_core::PCSTR, pvalue : super::PVOID, nsize : super::DWORD) -> windows_core::BOOL);
    unsafe { SetFirmwareEnvironmentVariableA(lpname.param().abi(), lpguid.param().abi(), pvalue.unwrap_or(core::mem::zeroed()) as _, nsize) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableExA<P0, P1>(lpname: P0, lpguid: P1, pvalue: Option<super::PVOID>, nsize: super::DWORD, dwattributes: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableExA(lpname : windows_core::PCSTR, lpguid : windows_core::PCSTR, pvalue : super::PVOID, nsize : super::DWORD, dwattributes : super::DWORD) -> windows_core::BOOL);
    unsafe { SetFirmwareEnvironmentVariableExA(lpname.param().abi(), lpguid.param().abi(), pvalue.unwrap_or(core::mem::zeroed()) as _, nsize, dwattributes) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableExW<P0, P1>(lpname: P0, lpguid: P1, pvalue: Option<super::PVOID>, nsize: super::DWORD, dwattributes: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableExW(lpname : windows_core::PCWSTR, lpguid : windows_core::PCWSTR, pvalue : super::PVOID, nsize : super::DWORD, dwattributes : super::DWORD) -> windows_core::BOOL);
    unsafe { SetFirmwareEnvironmentVariableExW(lpname.param().abi(), lpguid.param().abi(), pvalue.unwrap_or(core::mem::zeroed()) as _, nsize, dwattributes) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetFirmwareEnvironmentVariableW<P0, P1>(lpname: P0, lpguid: P1, pvalue: Option<super::PVOID>, nsize: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetFirmwareEnvironmentVariableW(lpname : windows_core::PCWSTR, lpguid : windows_core::PCWSTR, pvalue : super::PVOID, nsize : super::DWORD) -> windows_core::BOOL);
    unsafe { SetFirmwareEnvironmentVariableW(lpname.param().abi(), lpguid.param().abi(), pvalue.unwrap_or(core::mem::zeroed()) as _, nsize) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetHandleCount(unumber: super::UINT) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn SetHandleCount(unumber : super::UINT) -> super::UINT);
    unsafe { SetHandleCount(unumber) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetMailslotInfo(hmailslot: super::HANDLE, lreadtimeout: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetMailslotInfo(hmailslot : super::HANDLE, lreadtimeout : super::DWORD) -> windows_core::BOOL);
    unsafe { SetMailslotInfo(hmailslot, lreadtimeout) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetMessageWaitingIndicator(hmsgindicator: super::HANDLE, ulmsgcount: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetMessageWaitingIndicator(hmsgindicator : super::HANDLE, ulmsgcount : super::ULONG) -> windows_core::BOOL);
    unsafe { SetMessageWaitingIndicator(hmsgindicator, ulmsgcount) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetProcessAffinityMask(hprocess: super::HANDLE, dwprocessaffinitymask: super::DWORD_PTR) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessAffinityMask(hprocess : super::HANDLE, dwprocessaffinitymask : super::DWORD_PTR) -> windows_core::BOOL);
    unsafe { SetProcessAffinityMask(hprocess, dwprocessaffinitymask) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetProcessDEPPolicy(dwflags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetProcessDEPPolicy(dwflags : super::DWORD) -> windows_core::BOOL);
    unsafe { SetProcessDEPPolicy(dwflags) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn SetSearchPathMode(flags: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetSearchPathMode(flags : super::DWORD) -> windows_core::BOOL);
    unsafe { SetSearchPathMode(flags) }
}
#[inline]
pub unsafe fn SetSystemPowerState(fsuspend: bool, fforce: bool) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetSystemPowerState(fsuspend : windows_core::BOOL, fforce : windows_core::BOOL) -> windows_core::BOOL);
    unsafe { SetSystemPowerState(fsuspend.into(), fforce.into()) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetTapeParameters(hdevice: super::HANDLE, dwoperation: super::DWORD, lptapeinformation: super::LPVOID) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn SetTapeParameters(hdevice : super::HANDLE, dwoperation : super::DWORD, lptapeinformation : super::LPVOID) -> super::DWORD);
    unsafe { SetTapeParameters(hdevice, dwoperation, lptapeinformation) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetTapePosition(hdevice: super::HANDLE, dwpositionmethod: super::DWORD, dwpartition: super::DWORD, dwoffsetlow: super::DWORD, dwoffsethigh: super::DWORD, bimmediate: bool) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn SetTapePosition(hdevice : super::HANDLE, dwpositionmethod : super::DWORD, dwpartition : super::DWORD, dwoffsetlow : super::DWORD, dwoffsethigh : super::DWORD, bimmediate : windows_core::BOOL) -> super::DWORD);
    unsafe { SetTapePosition(hdevice, dwpositionmethod, dwpartition, dwoffsetlow, dwoffsethigh, bimmediate.into()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetThreadAffinityMask(hthread: super::HANDLE, dwthreadaffinitymask: super::DWORD_PTR) -> super::DWORD_PTR {
    windows_core::link!("kernel32.dll" "system" fn SetThreadAffinityMask(hthread : super::HANDLE, dwthreadaffinitymask : super::DWORD_PTR) -> super::DWORD_PTR);
    unsafe { SetThreadAffinityMask(hthread, dwthreadaffinitymask) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn SetThreadExecutionState(esflags: super::EXECUTION_STATE) -> super::EXECUTION_STATE {
    windows_core::link!("kernel32.dll" "system" fn SetThreadExecutionState(esflags : super::EXECUTION_STATE) -> super::EXECUTION_STATE);
    unsafe { SetThreadExecutionState(esflags) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetTimerQueueTimer(timerqueue: Option<super::HANDLE>, callback: super::WAITORTIMERCALLBACK, parameter: Option<super::PVOID>, duetime: super::DWORD, period: super::DWORD, preferio: bool) -> super::HANDLE {
    windows_core::link!("kernel32.dll" "system" fn SetTimerQueueTimer(timerqueue : super::HANDLE, callback : super::WAITORTIMERCALLBACK, parameter : super::PVOID, duetime : super::DWORD, period : super::DWORD, preferio : windows_core::BOOL) -> super::HANDLE);
    unsafe { SetTimerQueueTimer(timerqueue.unwrap_or(core::mem::zeroed()) as _, callback, parameter.unwrap_or(core::mem::zeroed()) as _, duetime, period, preferio.into()) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetUmsThreadInformation(umsthread: PUMS_CONTEXT, umsthreadinfoclass: UMS_THREAD_INFO_CLASS, umsthreadinformation: super::PVOID, umsthreadinformationlength: super::ULONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetUmsThreadInformation(umsthread : PUMS_CONTEXT, umsthreadinfoclass : UMS_THREAD_INFO_CLASS, umsthreadinformation : super::PVOID, umsthreadinformationlength : super::ULONG) -> windows_core::BOOL);
    unsafe { SetUmsThreadInformation(umsthread, umsthreadinfoclass, umsthreadinformation, umsthreadinformationlength) }
}
#[inline]
pub unsafe fn SetVolumeLabelA<P0, P1>(lprootpathname: P0, lpvolumename: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetVolumeLabelA(lprootpathname : windows_core::PCSTR, lpvolumename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetVolumeLabelA(lprootpathname.param().abi(), lpvolumename.param().abi()) }
}
#[inline]
pub unsafe fn SetVolumeLabelW<P0, P1>(lprootpathname: P0, lpvolumename: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetVolumeLabelW(lprootpathname : windows_core::PCWSTR, lpvolumename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetVolumeLabelW(lprootpathname.param().abi(), lpvolumename.param().abi()) }
}
#[inline]
pub unsafe fn SetVolumeMountPointA<P0, P1>(lpszvolumemountpoint: P0, lpszvolumename: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetVolumeMountPointA(lpszvolumemountpoint : windows_core::PCSTR, lpszvolumename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { SetVolumeMountPointA(lpszvolumemountpoint.param().abi(), lpszvolumename.param().abi()) }
}
#[inline]
pub unsafe fn SetVolumeMountPointW<P0, P1>(lpszvolumemountpoint: P0, lpszvolumename: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn SetVolumeMountPointW(lpszvolumemountpoint : windows_core::PCWSTR, lpszvolumename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { SetVolumeMountPointW(lpszvolumemountpoint.param().abi(), lpszvolumename.param().abi()) }
}
#[must_use]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetXStateFeaturesMask(context: super::PCONTEXT, featuremask: super::DWORD64) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetXStateFeaturesMask(context : super::PCONTEXT, featuremask : super::DWORD64) -> windows_core::BOOL);
    unsafe { SetXStateFeaturesMask(context as _, featuremask) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn SetupComm(hfile: super::HANDLE, dwinqueue: super::DWORD, dwoutqueue: super::DWORD) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn SetupComm(hfile : super::HANDLE, dwinqueue : super::DWORD, dwoutqueue : super::DWORD) -> windows_core::BOOL);
    unsafe { SetupComm(hfile, dwinqueue, dwoutqueue) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn SwitchToFiber(lpfiber: super::LPVOID) {
    windows_core::link!("kernel32.dll" "system" fn SwitchToFiber(lpfiber : super::LPVOID));
    unsafe { SwitchToFiber(lpfiber) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn TransmitCommChar(hfile: super::HANDLE, cchar: i8) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn TransmitCommChar(hfile : super::HANDLE, cchar : i8) -> windows_core::BOOL);
    unsafe { TransmitCommChar(hfile, cchar) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn UmsThreadYield(schedulerparam: super::PVOID) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UmsThreadYield(schedulerparam : super::PVOID) -> windows_core::BOOL);
    unsafe { UmsThreadYield(schedulerparam) }
}
#[inline]
pub unsafe fn UnregisterApplicationRecoveryCallback() -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn UnregisterApplicationRecoveryCallback() -> windows_core::HRESULT);
    unsafe { UnregisterApplicationRecoveryCallback() }
}
#[inline]
pub unsafe fn UnregisterApplicationRestart() -> windows_core::HRESULT {
    windows_core::link!("kernel32.dll" "system" fn UnregisterApplicationRestart() -> windows_core::HRESULT);
    unsafe { UnregisterApplicationRestart() }
}
#[must_use]
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn UnregisterWait(waithandle: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn UnregisterWait(waithandle : super::HANDLE) -> windows_core::BOOL);
    unsafe { UnregisterWait(waithandle) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn UpdateResourceA<P1, P2>(hupdate: super::HANDLE, lptype: P1, lpname: P2, wlanguage: super::WORD, lpdata: Option<super::LPVOID>, cb: super::DWORD) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn UpdateResourceA(hupdate : super::HANDLE, lptype : windows_core::PCSTR, lpname : windows_core::PCSTR, wlanguage : super::WORD, lpdata : super::LPVOID, cb : super::DWORD) -> windows_core::BOOL);
    unsafe { UpdateResourceA(hupdate, lptype.param().abi(), lpname.param().abi(), wlanguage, lpdata.unwrap_or(core::mem::zeroed()) as _, cb) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn UpdateResourceW<P1, P2>(hupdate: super::HANDLE, lptype: P1, lpname: P2, wlanguage: super::WORD, lpdata: Option<super::LPVOID>, cb: super::DWORD) -> windows_core::BOOL
where
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn UpdateResourceW(hupdate : super::HANDLE, lptype : windows_core::PCWSTR, lpname : windows_core::PCWSTR, wlanguage : super::WORD, lpdata : super::LPVOID, cb : super::DWORD) -> windows_core::BOOL);
    unsafe { UpdateResourceW(hupdate, lptype.param().abi(), lpname.param().abi(), wlanguage, lpdata.unwrap_or(core::mem::zeroed()) as _, cb) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn VerifyVersionInfoA(lpversioninformation: super::LPOSVERSIONINFOEXA, dwtypemask: super::DWORD, dwlconditionmask: super::DWORDLONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VerifyVersionInfoA(lpversioninformation : super::LPOSVERSIONINFOEXA, dwtypemask : super::DWORD, dwlconditionmask : super::DWORDLONG) -> windows_core::BOOL);
    unsafe { VerifyVersionInfoA(lpversioninformation as _, dwtypemask, dwlconditionmask) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn VerifyVersionInfoW(lpversioninformation: super::LPOSVERSIONINFOEXW, dwtypemask: super::DWORD, dwlconditionmask: super::DWORDLONG) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn VerifyVersionInfoW(lpversioninformation : super::LPOSVERSIONINFOEXW, dwtypemask : super::DWORD, dwlconditionmask : super::DWORDLONG) -> windows_core::BOOL);
    unsafe { VerifyVersionInfoW(lpversioninformation as _, dwtypemask, dwlconditionmask) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WTSGetActiveConsoleSessionId() -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn WTSGetActiveConsoleSessionId() -> super::DWORD);
    unsafe { WTSGetActiveConsoleSessionId() }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WaitCommEvent(hfile: super::HANDLE, lpevtmask: super::LPDWORD, lpoverlapped: Option<super::LPOVERLAPPED>) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn WaitCommEvent(hfile : super::HANDLE, lpevtmask : super::LPDWORD, lpoverlapped : super::LPOVERLAPPED) -> windows_core::BOOL);
    unsafe { WaitCommEvent(hfile, lpevtmask as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WaitNamedPipeA<P0>(lpnamedpipename: P0, ntimeout: super::DWORD) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WaitNamedPipeA(lpnamedpipename : windows_core::PCSTR, ntimeout : super::DWORD) -> windows_core::BOOL);
    unsafe { WaitNamedPipeA(lpnamedpipename.param().abi(), ntimeout) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn WinExec<P0>(lpcmdline: P0, ucmdshow: super::UINT) -> super::UINT
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WinExec(lpcmdline : windows_core::PCSTR, ucmdshow : super::UINT) -> super::UINT);
    unsafe { WinExec(lpcmdline.param().abi(), ucmdshow) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn Wow64GetThreadSelectorEntry(hthread: super::HANDLE, dwselector: super::DWORD, lpselectorentry: super::PWOW64_LDT_ENTRY) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn Wow64GetThreadSelectorEntry(hthread : super::HANDLE, dwselector : super::DWORD, lpselectorentry : super::PWOW64_LDT_ENTRY) -> windows_core::BOOL);
    unsafe { Wow64GetThreadSelectorEntry(hthread, dwselector, lpselectorentry as _) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WriteEncryptedFileRaw(pfimportcallback: PFE_IMPORT_FUNC, pvcallbackcontext: Option<super::PVOID>, pvcontext: super::PVOID) -> super::DWORD {
    windows_core::link!("advapi32.dll" "system" fn WriteEncryptedFileRaw(pfimportcallback : PFE_IMPORT_FUNC, pvcallbackcontext : super::PVOID, pvcontext : super::PVOID) -> super::DWORD);
    unsafe { WriteEncryptedFileRaw(pfimportcallback, pvcallbackcontext.unwrap_or(core::mem::zeroed()) as _, pvcontext) }
}
#[inline]
pub unsafe fn WritePrivateProfileSectionA<P0, P1, P2>(lpappname: P0, lpstring: P1, lpfilename: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WritePrivateProfileSectionA(lpappname : windows_core::PCSTR, lpstring : windows_core::PCSTR, lpfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { WritePrivateProfileSectionA(lpappname.param().abi(), lpstring.param().abi(), lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn WritePrivateProfileSectionW<P0, P1, P2>(lpappname: P0, lpstring: P1, lpfilename: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WritePrivateProfileSectionW(lpappname : windows_core::PCWSTR, lpstring : windows_core::PCWSTR, lpfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { WritePrivateProfileSectionW(lpappname.param().abi(), lpstring.param().abi(), lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn WritePrivateProfileStringA<P0, P1, P2, P3>(lpappname: P0, lpkeyname: P1, lpstring: P2, lpfilename: P3) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
    P3: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WritePrivateProfileStringA(lpappname : windows_core::PCSTR, lpkeyname : windows_core::PCSTR, lpstring : windows_core::PCSTR, lpfilename : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { WritePrivateProfileStringA(lpappname.param().abi(), lpkeyname.param().abi(), lpstring.param().abi(), lpfilename.param().abi()) }
}
#[inline]
pub unsafe fn WritePrivateProfileStringW<P0, P1, P2, P3>(lpappname: P0, lpkeyname: P1, lpstring: P2, lpfilename: P3) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WritePrivateProfileStringW(lpappname : windows_core::PCWSTR, lpkeyname : windows_core::PCWSTR, lpstring : windows_core::PCWSTR, lpfilename : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { WritePrivateProfileStringW(lpappname.param().abi(), lpkeyname.param().abi(), lpstring.param().abi(), lpfilename.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WritePrivateProfileStructA<P0, P1, P4>(lpszsection: P0, lpszkey: P1, lpstruct: Option<super::LPVOID>, usizestruct: super::UINT, szfile: P4) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P4: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WritePrivateProfileStructA(lpszsection : windows_core::PCSTR, lpszkey : windows_core::PCSTR, lpstruct : super::LPVOID, usizestruct : super::UINT, szfile : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { WritePrivateProfileStructA(lpszsection.param().abi(), lpszkey.param().abi(), lpstruct.unwrap_or(core::mem::zeroed()) as _, usizestruct, szfile.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn WritePrivateProfileStructW<P0, P1, P4>(lpszsection: P0, lpszkey: P1, lpstruct: Option<super::LPVOID>, usizestruct: super::UINT, szfile: P4) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P4: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WritePrivateProfileStructW(lpszsection : windows_core::PCWSTR, lpszkey : windows_core::PCWSTR, lpstruct : super::LPVOID, usizestruct : super::UINT, szfile : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { WritePrivateProfileStructW(lpszsection.param().abi(), lpszkey.param().abi(), lpstruct.unwrap_or(core::mem::zeroed()) as _, usizestruct, szfile.param().abi()) }
}
#[inline]
pub unsafe fn WriteProfileSectionA<P0, P1>(lpappname: P0, lpstring: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WriteProfileSectionA(lpappname : windows_core::PCSTR, lpstring : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { WriteProfileSectionA(lpappname.param().abi(), lpstring.param().abi()) }
}
#[inline]
pub unsafe fn WriteProfileSectionW<P0, P1>(lpappname: P0, lpstring: P1) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WriteProfileSectionW(lpappname : windows_core::PCWSTR, lpstring : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { WriteProfileSectionW(lpappname.param().abi(), lpstring.param().abi()) }
}
#[inline]
pub unsafe fn WriteProfileStringA<P0, P1, P2>(lpappname: P0, lpkeyname: P1, lpstring: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
    P2: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WriteProfileStringA(lpappname : windows_core::PCSTR, lpkeyname : windows_core::PCSTR, lpstring : windows_core::PCSTR) -> windows_core::BOOL);
    unsafe { WriteProfileStringA(lpappname.param().abi(), lpkeyname.param().abi(), lpstring.param().abi()) }
}
#[inline]
pub unsafe fn WriteProfileStringW<P0, P1, P2>(lpappname: P0, lpkeyname: P1, lpstring: P2) -> windows_core::BOOL
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn WriteProfileStringW(lpappname : windows_core::PCWSTR, lpkeyname : windows_core::PCWSTR, lpstring : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { WriteProfileStringW(lpappname.param().abi(), lpkeyname.param().abi(), lpstring.param().abi()) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn WriteTapemark(hdevice: super::HANDLE, dwtapemarktype: super::DWORD, dwtapemarkcount: super::DWORD, bimmediate: bool) -> super::DWORD {
    windows_core::link!("kernel32.dll" "system" fn WriteTapemark(hdevice : super::HANDLE, dwtapemarktype : super::DWORD, dwtapemarkcount : super::DWORD, bimmediate : windows_core::BOOL) -> super::DWORD);
    unsafe { WriteTapemark(hdevice, dwtapemarktype, dwtapemarkcount, bimmediate.into()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn ZombifyActCtx(hactctx: super::HANDLE) -> windows_core::BOOL {
    windows_core::link!("kernel32.dll" "system" fn ZombifyActCtx(hactctx : super::HANDLE) -> windows_core::BOOL);
    unsafe { ZombifyActCtx(hactctx as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn _hread(hfile: super::HFILE, lpbuffer: super::LPVOID, lbytes: i32) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn _hread(hfile : super::HFILE, lpbuffer : super::LPVOID, lbytes : i32) -> i32);
    unsafe { _hread(hfile, lpbuffer as _, lbytes) }
}
#[cfg(all(feature = "minwindef", feature = "winnt"))]
#[inline]
pub unsafe fn _hwrite(hfile: super::HFILE, lpbuffer: &[u8]) -> i32 {
    windows_core::link!("kernel32.dll" "system" fn _hwrite(hfile : super::HFILE, lpbuffer : super::LPCCH, lbytes : i32) -> i32);
    unsafe { _hwrite(hfile, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap()) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn _lclose(hfile: super::HFILE) -> super::HFILE {
    windows_core::link!("kernel32.dll" "system" fn _lclose(hfile : super::HFILE) -> super::HFILE);
    unsafe { _lclose(hfile) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn _lcreat<P0>(lppathname: P0, iattribute: i32) -> super::HFILE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn _lcreat(lppathname : windows_core::PCSTR, iattribute : i32) -> super::HFILE);
    unsafe { _lcreat(lppathname.param().abi(), iattribute) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn _llseek(hfile: super::HFILE, loffset: super::LONG, iorigin: i32) -> super::LONG {
    windows_core::link!("kernel32.dll" "system" fn _llseek(hfile : super::HFILE, loffset : super::LONG, iorigin : i32) -> super::LONG);
    unsafe { _llseek(hfile, loffset, iorigin) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn _lopen<P0>(lppathname: P0, ireadwrite: i32) -> super::HFILE
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn _lopen(lppathname : windows_core::PCSTR, ireadwrite : i32) -> super::HFILE);
    unsafe { _lopen(lppathname.param().abi(), ireadwrite) }
}
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[inline]
pub unsafe fn _lread(hfile: super::HFILE, lpbuffer: super::LPVOID, ubytes: super::UINT) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn _lread(hfile : super::HFILE, lpbuffer : super::LPVOID, ubytes : super::UINT) -> super::UINT);
    unsafe { _lread(hfile, lpbuffer as _, ubytes) }
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn _lwrite(hfile: super::HFILE, lpbuffer: &[u8]) -> super::UINT {
    windows_core::link!("kernel32.dll" "system" fn _lwrite(hfile : super::HFILE, lpbuffer : super::LPCCH, ubytes : super::UINT) -> super::UINT);
    unsafe { _lwrite(hfile, core::mem::transmute(lpbuffer.as_ptr()), lpbuffer.len().try_into().unwrap()) }
}
#[inline]
pub unsafe fn lstrcatA<P1>(lpstring1: windows_core::PSTR, lpstring2: P1) -> windows_core::PSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcatA(lpstring1 : windows_core::PSTR, lpstring2 : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { lstrcatA(lpstring1, lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcatW<P1>(lpstring1: windows_core::PWSTR, lpstring2: P1) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcatW(lpstring1 : windows_core::PWSTR, lpstring2 : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { lstrcatW(lpstring1, lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcmpA<P0, P1>(lpstring1: P0, lpstring2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcmpA(lpstring1 : windows_core::PCSTR, lpstring2 : windows_core::PCSTR) -> i32);
    unsafe { lstrcmpA(lpstring1.param().abi(), lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcmpW<P0, P1>(lpstring1: P0, lpstring2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcmpW(lpstring1 : windows_core::PCWSTR, lpstring2 : windows_core::PCWSTR) -> i32);
    unsafe { lstrcmpW(lpstring1.param().abi(), lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcmpiA<P0, P1>(lpstring1: P0, lpstring2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcmpiA(lpstring1 : windows_core::PCSTR, lpstring2 : windows_core::PCSTR) -> i32);
    unsafe { lstrcmpiA(lpstring1.param().abi(), lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcmpiW<P0, P1>(lpstring1: P0, lpstring2: P1) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcmpiW(lpstring1 : windows_core::PCWSTR, lpstring2 : windows_core::PCWSTR) -> i32);
    unsafe { lstrcmpiW(lpstring1.param().abi(), lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcpyA<P1>(lpstring1: windows_core::PSTR, lpstring2: P1) -> windows_core::PSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcpyA(lpstring1 : windows_core::PSTR, lpstring2 : windows_core::PCSTR) -> windows_core::PSTR);
    unsafe { lstrcpyA(lpstring1, lpstring2.param().abi()) }
}
#[inline]
pub unsafe fn lstrcpyW<P1>(lpstring1: windows_core::PWSTR, lpstring2: P1) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcpyW(lpstring1 : windows_core::PWSTR, lpstring2 : windows_core::PCWSTR) -> windows_core::PWSTR);
    unsafe { lstrcpyW(lpstring1, lpstring2.param().abi()) }
}
#[must_use]
#[inline]
pub unsafe fn lstrcpynA<P1>(lpstring1: windows_core::PSTR, lpstring2: P1, imaxlength: i32) -> windows_core::PSTR
where
    P1: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcpynA(lpstring1 : windows_core::PSTR, lpstring2 : windows_core::PCSTR, imaxlength : i32) -> windows_core::PSTR);
    unsafe { lstrcpynA(lpstring1, lpstring2.param().abi(), imaxlength) }
}
#[must_use]
#[inline]
pub unsafe fn lstrcpynW<P1>(lpstring1: windows_core::PWSTR, lpstring2: P1, imaxlength: i32) -> windows_core::PWSTR
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrcpynW(lpstring1 : windows_core::PWSTR, lpstring2 : windows_core::PCWSTR, imaxlength : i32) -> windows_core::PWSTR);
    unsafe { lstrcpynW(lpstring1, lpstring2.param().abi(), imaxlength) }
}
#[inline]
pub unsafe fn lstrlenA<P0>(lpstring: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrlenA(lpstring : windows_core::PCSTR) -> i32);
    unsafe { lstrlenA(lpstring.param().abi()) }
}
#[inline]
pub unsafe fn lstrlenW<P0>(lpstring: P0) -> i32
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("kernel32.dll" "system" fn lstrlenW(lpstring : windows_core::PCWSTR) -> i32);
    unsafe { lstrlenW(lpstring.param().abi()) }
}
pub const ABOVE_NORMAL_PRIORITY_CLASS: i32 = 32768;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type ACTCTX = ACTCTXA;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACTCTXA {
    pub cbSize: super::ULONG,
    pub dwFlags: super::DWORD,
    pub lpSource: windows_core::PCSTR,
    pub wProcessorArchitecture: super::USHORT,
    pub wLangId: super::LANGID,
    pub lpAssemblyDirectory: windows_core::PCSTR,
    pub lpResourceName: windows_core::PCSTR,
    pub lpApplicationName: windows_core::PCSTR,
    pub hModule: super::HMODULE,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACTCTXW {
    pub cbSize: super::ULONG,
    pub dwFlags: super::DWORD,
    pub lpSource: windows_core::PCWSTR,
    pub wProcessorArchitecture: super::USHORT,
    pub wLangId: super::LANGID,
    pub lpAssemblyDirectory: windows_core::PCWSTR,
    pub lpResourceName: windows_core::PCWSTR,
    pub lpApplicationName: windows_core::PCWSTR,
    pub hModule: super::HMODULE,
}
pub const ACTCTX_FLAG_APPLICATION_NAME_VALID: i32 = 32;
pub const ACTCTX_FLAG_ASSEMBLY_DIRECTORY_VALID: i32 = 4;
pub const ACTCTX_FLAG_HMODULE_VALID: i32 = 128;
pub const ACTCTX_FLAG_LANGID_VALID: i32 = 2;
pub const ACTCTX_FLAG_PROCESSOR_ARCHITECTURE_VALID: i32 = 1;
pub const ACTCTX_FLAG_RESOURCE_NAME_VALID: i32 = 8;
pub const ACTCTX_FLAG_SET_PROCESS_DEFAULT: i32 = 16;
pub const ACTCTX_FLAG_SOURCE_IS_ASSEMBLYREF: i32 = 64;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACTCTX_SECTION_KEYED_DATA {
    pub cbSize: super::ULONG,
    pub ulDataFormatVersion: super::ULONG,
    pub lpData: super::PVOID,
    pub ulLength: super::ULONG,
    pub lpSectionGlobalData: super::PVOID,
    pub ulSectionGlobalDataLength: super::ULONG,
    pub lpSectionBase: super::PVOID,
    pub ulSectionTotalLength: super::ULONG,
    pub hActCtx: super::HANDLE,
    pub ulAssemblyRosterIndex: super::ULONG,
    pub ulFlags: super::ULONG,
    pub AssemblyMetadata: ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACTCTX_SECTION_KEYED_DATA_2600 {
    pub cbSize: super::ULONG,
    pub ulDataFormatVersion: super::ULONG,
    pub lpData: super::PVOID,
    pub ulLength: super::ULONG,
    pub lpSectionGlobalData: super::PVOID,
    pub ulSectionGlobalDataLength: super::ULONG,
    pub lpSectionBase: super::PVOID,
    pub ulSectionTotalLength: super::ULONG,
    pub hActCtx: super::HANDLE,
    pub ulAssemblyRosterIndex: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA {
    pub lpInformation: super::PVOID,
    pub lpSectionBase: super::PVOID,
    pub ulSectionLength: super::ULONG,
    pub lpSectionGlobalDataBase: super::PVOID,
    pub ulSectionGlobalDataLength: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ACTIVATION_CONTEXT_BASIC_INFORMATION {
    pub hActCtx: super::HANDLE,
    pub dwFlags: super::DWORD,
}
pub const ACTIVATION_CONTEXT_BASIC_INFORMATION_DEFINED: i32 = 1;
pub const AC_LINE_BACKUP_POWER: i32 = 2;
pub const AC_LINE_OFFLINE: i32 = 0;
pub const AC_LINE_ONLINE: i32 = 1;
pub const AC_LINE_UNKNOWN: i32 = 255;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type APPLICATION_RECOVERY_CALLBACK = Option<unsafe extern "system" fn(pvparameter: super::PVOID) -> super::DWORD>;
pub const ATOM_FLAG_GLOBAL: i32 = 2;
pub const BACKUP_ALTERNATE_DATA: i32 = 4;
pub const BACKUP_DATA: i32 = 1;
pub const BACKUP_EA_DATA: i32 = 2;
pub const BACKUP_GHOSTED_FILE_EXTENTS: i32 = 11;
pub const BACKUP_INVALID: i32 = 0;
pub const BACKUP_LINK: i32 = 5;
pub const BACKUP_OBJECT_ID: i32 = 7;
pub const BACKUP_PROPERTY_DATA: i32 = 6;
pub const BACKUP_REPARSE_DATA: i32 = 8;
pub const BACKUP_SECURITY_DATA: i32 = 3;
pub const BACKUP_SPARSE_BLOCK: i32 = 9;
pub const BACKUP_TXFS_DATA: i32 = 10;
pub const BASE_SEARCH_PATH_DISABLE_SAFE_SEARCHMODE: i32 = 65536;
pub const BASE_SEARCH_PATH_ENABLE_SAFE_SEARCHMODE: i32 = 1;
pub const BASE_SEARCH_PATH_INVALID_FLAGS: i32 = -98306;
pub const BASE_SEARCH_PATH_PERMANENT: i32 = 32768;
pub const BATTERY_FLAG_CHARGING: i32 = 8;
pub const BATTERY_FLAG_CRITICAL: i32 = 4;
pub const BATTERY_FLAG_HIGH: i32 = 1;
pub const BATTERY_FLAG_LOW: i32 = 2;
pub const BATTERY_FLAG_NO_BATTERY: i32 = 128;
pub const BATTERY_FLAG_UNKNOWN: i32 = 255;
pub const BATTERY_LIFE_UNKNOWN: u32 = 4294967295;
pub const BATTERY_PERCENTAGE_UNKNOWN: i32 = 255;
#[cfg(feature = "shared")]
pub const BAUD_075: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const BAUD_110: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const BAUD_115200: super::DWORD = 131072;
#[cfg(feature = "shared")]
pub const BAUD_1200: super::DWORD = 64;
#[cfg(feature = "shared")]
pub const BAUD_128K: super::DWORD = 65536;
#[cfg(feature = "shared")]
pub const BAUD_134_5: super::DWORD = 4;
#[cfg(feature = "shared")]
pub const BAUD_14400: super::DWORD = 4096;
#[cfg(feature = "shared")]
pub const BAUD_150: super::DWORD = 8;
#[cfg(feature = "shared")]
pub const BAUD_1800: super::DWORD = 128;
#[cfg(feature = "shared")]
pub const BAUD_19200: super::DWORD = 8192;
#[cfg(feature = "shared")]
pub const BAUD_2400: super::DWORD = 256;
#[cfg(feature = "shared")]
pub const BAUD_300: super::DWORD = 16;
#[cfg(feature = "shared")]
pub const BAUD_38400: super::DWORD = 16384;
#[cfg(feature = "shared")]
pub const BAUD_4800: super::DWORD = 512;
#[cfg(feature = "shared")]
pub const BAUD_56K: super::DWORD = 32768;
#[cfg(feature = "shared")]
pub const BAUD_57600: super::DWORD = 262144;
#[cfg(feature = "shared")]
pub const BAUD_600: super::DWORD = 32;
#[cfg(feature = "shared")]
pub const BAUD_7200: super::DWORD = 1024;
#[cfg(feature = "shared")]
pub const BAUD_9600: super::DWORD = 2048;
#[cfg(feature = "shared")]
pub const BAUD_USER: super::DWORD = 268435456;
pub const BELOW_NORMAL_PRIORITY_CLASS: i32 = 16384;
pub const CALLBACK_CHUNK_FINISHED: i32 = 0;
pub const CALLBACK_STREAM_SWITCH: i32 = 1;
pub const CBR_110: i32 = 110;
pub const CBR_115200: i32 = 115200;
pub const CBR_1200: i32 = 1200;
pub const CBR_128000: i32 = 128000;
pub const CBR_14400: i32 = 14400;
pub const CBR_19200: i32 = 19200;
pub const CBR_2400: i32 = 2400;
pub const CBR_256000: i32 = 256000;
pub const CBR_300: i32 = 300;
pub const CBR_38400: i32 = 38400;
pub const CBR_4800: i32 = 4800;
pub const CBR_56000: i32 = 56000;
pub const CBR_57600: i32 = 57600;
pub const CBR_600: i32 = 600;
pub const CBR_9600: i32 = 9600;
pub const CE_BREAK: i32 = 16;
pub const CE_DNS: i32 = 2048;
pub const CE_FRAME: i32 = 8;
pub const CE_IOE: i32 = 1024;
pub const CE_MODE: i32 = 32768;
pub const CE_OOP: i32 = 4096;
pub const CE_OVERRUN: i32 = 2;
pub const CE_PTO: i32 = 512;
pub const CE_RXOVER: i32 = 1;
pub const CE_RXPARITY: i32 = 4;
pub const CE_TXFULL: i32 = 256;
pub const CLRBREAK: i32 = 9;
pub const CLRDTR: i32 = 6;
pub const CLRRTS: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COMMCONFIG {
    pub dwSize: super::DWORD,
    pub wVersion: super::WORD,
    pub wReserved: super::WORD,
    pub dcb: DCB,
    pub dwProviderSubType: super::DWORD,
    pub dwProviderOffset: super::DWORD,
    pub dwProviderSize: super::DWORD,
    pub wcProviderData: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for COMMCONFIG {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct COMMPROP {
    pub wPacketLength: super::WORD,
    pub wPacketVersion: super::WORD,
    pub dwServiceMask: super::DWORD,
    pub dwReserved1: super::DWORD,
    pub dwMaxTxQueue: super::DWORD,
    pub dwMaxRxQueue: super::DWORD,
    pub dwMaxBaud: super::DWORD,
    pub dwProvSubType: super::DWORD,
    pub dwProvCapabilities: super::DWORD,
    pub dwSettableParams: super::DWORD,
    pub dwSettableBaud: super::DWORD,
    pub wSettableData: super::WORD,
    pub wSettableStopParity: super::WORD,
    pub dwCurrentTxQueue: super::DWORD,
    pub dwCurrentRxQueue: super::DWORD,
    pub dwProvSpec1: super::DWORD,
    pub dwProvSpec2: super::DWORD,
    pub wcProvChar: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for COMMPROP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "shared")]
pub const COMMPROP_INITIALIZED: super::DWORD = 3879531822;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMMTIMEOUTS {
    pub ReadIntervalTimeout: super::DWORD,
    pub ReadTotalTimeoutMultiplier: super::DWORD,
    pub ReadTotalTimeoutConstant: super::DWORD,
    pub WriteTotalTimeoutMultiplier: super::DWORD,
    pub WriteTotalTimeoutConstant: super::DWORD,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COMSTAT {
    pub _bitfield: u32,
    pub cbInQue: super::DWORD,
    pub cbOutQue: super::DWORD,
}
#[cfg(feature = "shared")]
impl COMSTAT {
    pub fn fCtsHold(&self) -> super::DWORD {
        (self._bitfield) & 1
    }
    pub fn set_fCtsHold(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn fDsrHold(&self) -> super::DWORD {
        (self._bitfield >> 1) & 1
    }
    pub fn set_fDsrHold(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn fRlsdHold(&self) -> super::DWORD {
        (self._bitfield >> 2) & 1
    }
    pub fn set_fRlsdHold(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn fXoffHold(&self) -> super::DWORD {
        (self._bitfield >> 3) & 1
    }
    pub fn set_fXoffHold(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn fXoffSent(&self) -> super::DWORD {
        (self._bitfield >> 4) & 1
    }
    pub fn set_fXoffSent(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 4)) | ((value & 1) << 4);
    }
    pub fn fEof(&self) -> super::DWORD {
        (self._bitfield >> 5) & 1
    }
    pub fn set_fEof(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 5)) | ((value & 1) << 5);
    }
    pub fn fTxim(&self) -> super::DWORD {
        (self._bitfield >> 6) & 1
    }
    pub fn set_fTxim(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn fReserved(&self) -> super::DWORD {
        self._bitfield >> 7
    }
    pub fn set_fReserved(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(33554431 << 7)) | ((value & 33554431) << 7);
    }
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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COPYFILE2_CREATE_OPLOCK_KEYS {
    pub ParentOplockKey: windows_core::GUID,
    pub TargetOplockKey: windows_core::GUID,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct COPYFILE2_EXTENDED_PARAMETERS {
    pub dwSize: super::DWORD,
    pub dwCopyFlags: super::DWORD,
    pub pfCancel: *mut windows_core::BOOL,
    pub pProgressRoutine: PCOPYFILE2_PROGRESS_ROUTINE,
    pub pvCallbackContext: super::PVOID,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug)]
pub struct COPYFILE2_EXTENDED_PARAMETERS_V2 {
    pub dwSize: super::DWORD,
    pub dwCopyFlags: super::DWORD,
    pub pfCancel: *mut windows_core::BOOL,
    pub pProgressRoutine: PCOPYFILE2_PROGRESS_ROUTINE,
    pub pvCallbackContext: super::PVOID,
    pub dwCopyFlagsV2: super::DWORD,
    pub ioDesiredSize: super::ULONG,
    pub ioDesiredRate: super::ULONG,
    pub pProgressRoutineOld: LPPROGRESS_ROUTINE,
    pub SourceOplockKeys: PCOPYFILE2_CREATE_OPLOCK_KEYS,
    pub reserved: [super::PVOID; 6],
}
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
impl Default for COPYFILE2_EXTENDED_PARAMETERS_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const COPYFILE2_IO_CYCLE_SIZE_MAX: i32 = 1073741824;
pub const COPYFILE2_IO_CYCLE_SIZE_MIN: i32 = 4096;
pub const COPYFILE2_IO_RATE_MIN: i32 = 512;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct COPYFILE2_MESSAGE {
    pub Type: COPYFILE2_MESSAGE_TYPE,
    pub dwPadding: super::DWORD,
    pub Info: COPYFILE2_MESSAGE_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for COPYFILE2_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
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
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for COPYFILE2_MESSAGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct COPYFILE2_MESSAGE_0_0 {
    pub dwStreamNumber: super::DWORD,
    pub dwReserved: super::DWORD,
    pub hSourceFile: super::HANDLE,
    pub hDestinationFile: super::HANDLE,
    pub uliChunkNumber: super::ULARGE_INTEGER,
    pub uliChunkSize: super::ULARGE_INTEGER,
    pub uliStreamSize: super::ULARGE_INTEGER,
    pub uliTotalFileSize: super::ULARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for COPYFILE2_MESSAGE_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct COPYFILE2_MESSAGE_0_1 {
    pub dwStreamNumber: super::DWORD,
    pub dwFlags: super::DWORD,
    pub hSourceFile: super::HANDLE,
    pub hDestinationFile: super::HANDLE,
    pub uliChunkNumber: super::ULARGE_INTEGER,
    pub uliChunkSize: super::ULARGE_INTEGER,
    pub uliStreamSize: super::ULARGE_INTEGER,
    pub uliStreamBytesTransferred: super::ULARGE_INTEGER,
    pub uliTotalFileSize: super::ULARGE_INTEGER,
    pub uliTotalBytesTransferred: super::ULARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for COPYFILE2_MESSAGE_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct COPYFILE2_MESSAGE_0_2 {
    pub dwStreamNumber: super::DWORD,
    pub dwReserved: super::DWORD,
    pub hSourceFile: super::HANDLE,
    pub hDestinationFile: super::HANDLE,
    pub uliStreamSize: super::ULARGE_INTEGER,
    pub uliTotalFileSize: super::ULARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for COPYFILE2_MESSAGE_0_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct COPYFILE2_MESSAGE_0_3 {
    pub dwStreamNumber: super::DWORD,
    pub dwReserved: super::DWORD,
    pub hSourceFile: super::HANDLE,
    pub hDestinationFile: super::HANDLE,
    pub uliStreamSize: super::ULARGE_INTEGER,
    pub uliStreamBytesTransferred: super::ULARGE_INTEGER,
    pub uliTotalFileSize: super::ULARGE_INTEGER,
    pub uliTotalBytesTransferred: super::ULARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for COPYFILE2_MESSAGE_0_3 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct COPYFILE2_MESSAGE_0_4 {
    pub dwReserved: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct COPYFILE2_MESSAGE_0_5 {
    pub CopyPhase: COPYFILE2_COPY_PHASE,
    pub dwStreamNumber: super::DWORD,
    pub hrFailure: windows_core::HRESULT,
    pub dwReserved: super::DWORD,
    pub uliChunkNumber: super::ULARGE_INTEGER,
    pub uliStreamSize: super::ULARGE_INTEGER,
    pub uliStreamBytesTransferred: super::ULARGE_INTEGER,
    pub uliTotalFileSize: super::ULARGE_INTEGER,
    pub uliTotalBytesTransferred: super::ULARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for COPYFILE2_MESSAGE_0_5 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct COPYFILE2_MESSAGE_0_6 {
    pub uliChunkNumber: super::ULARGE_INTEGER,
    pub uliChunkSize: super::ULARGE_INTEGER,
    pub uliChunkOffset: super::ULARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for COPYFILE2_MESSAGE_0_6 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type COPYFILE2_MESSAGE_ACTION = i32;
pub const COPYFILE2_MESSAGE_COPY_OFFLOAD: i32 = 1;
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
pub const COPY_FILE2_V2_DISABLE_BLOCK_CLONING: i32 = 2;
pub const COPY_FILE2_V2_DONT_COPY_JUNCTIONS: i32 = 1;
pub const COPY_FILE2_V2_VALID_FLAGS: i32 = 3;
pub const COPY_FILE_ALLOW_DECRYPTED_DESTINATION: i32 = 8;
pub const COPY_FILE_COPY_SYMLINK: i32 = 2048;
pub const COPY_FILE_DIRECTORY: i32 = 128;
pub const COPY_FILE_DISABLE_PRE_ALLOCATION: i32 = 67108864;
pub const COPY_FILE_DISABLE_SPARSE_COPY: u32 = 2147483648;
pub const COPY_FILE_DONT_REQUEST_DEST_WRITE_DAC: i32 = 33554432;
pub const COPY_FILE_ENABLE_LOW_FREE_SPACE_MODE: i32 = 134217728;
pub const COPY_FILE_ENABLE_SPARSE_COPY: i32 = 536870912;
pub const COPY_FILE_FAIL_IF_EXISTS: i32 = 1;
pub const COPY_FILE_IGNORE_EDP_BLOCK: i32 = 4194304;
pub const COPY_FILE_IGNORE_SOURCE_ENCRYPTION: i32 = 8388608;
pub const COPY_FILE_NO_BUFFERING: i32 = 4096;
pub const COPY_FILE_NO_OFFLOAD: i32 = 262144;
pub const COPY_FILE_OPEN_AND_COPY_REPARSE_POINT: i32 = 2097152;
pub const COPY_FILE_OPEN_SOURCE_FOR_WRITE: i32 = 4;
pub const COPY_FILE_REQUEST_COMPRESSED_TRAFFIC: i32 = 268435456;
pub const COPY_FILE_REQUEST_SECURITY_PRIVILEGES: i32 = 8192;
pub const COPY_FILE_RESTARTABLE: i32 = 2;
pub const COPY_FILE_RESUME_FROM_PAUSE: i32 = 16384;
pub const COPY_FILE_SKIP_ALTERNATE_STREAMS: i32 = 32768;
pub const CREATE_BREAKAWAY_FROM_JOB: i32 = 16777216;
pub const CREATE_DEFAULT_ERROR_MODE: i32 = 67108864;
pub const CREATE_FORCEDOS: i32 = 8192;
pub const CREATE_FOR_DIR: i32 = 2;
pub const CREATE_FOR_IMPORT: i32 = 1;
pub const CREATE_IGNORE_SYSTEM_DEFAULT: u32 = 2147483648;
pub const CREATE_NEW_CONSOLE: i32 = 16;
pub const CREATE_NEW_PROCESS_GROUP: i32 = 512;
pub const CREATE_NO_WINDOW: i32 = 134217728;
pub const CREATE_PRESERVE_CODE_AUTHZ_LEVEL: i32 = 33554432;
pub const CREATE_PROTECTED_PROCESS: i32 = 262144;
pub const CREATE_SECURE_PROCESS: i32 = 4194304;
pub const CREATE_SEPARATE_WOW_VDM: i32 = 2048;
pub const CREATE_SHARED_WOW_VDM: i32 = 4096;
pub const CREATE_SUSPENDED: i32 = 4;
pub const CREATE_UNICODE_ENVIRONMENT: i32 = 1024;
pub const CRITICAL_SECTION_NO_DEBUG_INFO: i32 = 16777216;
#[cfg(feature = "shared")]
pub const DATABITS_16: super::WORD = 16;
#[cfg(feature = "shared")]
pub const DATABITS_16X: super::WORD = 32;
#[cfg(feature = "shared")]
pub const DATABITS_5: super::WORD = 1;
#[cfg(feature = "shared")]
pub const DATABITS_6: super::WORD = 2;
#[cfg(feature = "shared")]
pub const DATABITS_7: super::WORD = 4;
#[cfg(feature = "shared")]
pub const DATABITS_8: super::WORD = 8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DCB {
    pub DCBlength: super::DWORD,
    pub BaudRate: super::DWORD,
    pub _bitfield: u32,
    pub wReserved: super::WORD,
    pub XonLim: super::WORD,
    pub XoffLim: super::WORD,
    pub ByteSize: super::BYTE,
    pub Parity: super::BYTE,
    pub StopBits: super::BYTE,
    pub XonChar: i8,
    pub XoffChar: i8,
    pub ErrorChar: i8,
    pub EofChar: i8,
    pub EvtChar: i8,
    pub wReserved1: super::WORD,
}
#[cfg(feature = "shared")]
impl DCB {
    pub fn fBinary(&self) -> super::DWORD {
        (self._bitfield) & 1
    }
    pub fn set_fBinary(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn fParity(&self) -> super::DWORD {
        (self._bitfield >> 1) & 1
    }
    pub fn set_fParity(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
    pub fn fOutxCtsFlow(&self) -> super::DWORD {
        (self._bitfield >> 2) & 1
    }
    pub fn set_fOutxCtsFlow(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 2)) | ((value & 1) << 2);
    }
    pub fn fOutxDsrFlow(&self) -> super::DWORD {
        (self._bitfield >> 3) & 1
    }
    pub fn set_fOutxDsrFlow(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 3)) | ((value & 1) << 3);
    }
    pub fn fDtrControl(&self) -> super::DWORD {
        (self._bitfield >> 4) & 3
    }
    pub fn set_fDtrControl(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(3 << 4)) | ((value & 3) << 4);
    }
    pub fn fDsrSensitivity(&self) -> super::DWORD {
        (self._bitfield >> 6) & 1
    }
    pub fn set_fDsrSensitivity(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 6)) | ((value & 1) << 6);
    }
    pub fn fTXContinueOnXoff(&self) -> super::DWORD {
        (self._bitfield >> 7) & 1
    }
    pub fn set_fTXContinueOnXoff(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 7)) | ((value & 1) << 7);
    }
    pub fn fOutX(&self) -> super::DWORD {
        (self._bitfield >> 8) & 1
    }
    pub fn set_fOutX(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 8)) | ((value & 1) << 8);
    }
    pub fn fInX(&self) -> super::DWORD {
        (self._bitfield >> 9) & 1
    }
    pub fn set_fInX(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 9)) | ((value & 1) << 9);
    }
    pub fn fErrorChar(&self) -> super::DWORD {
        (self._bitfield >> 10) & 1
    }
    pub fn set_fErrorChar(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 10)) | ((value & 1) << 10);
    }
    pub fn fNull(&self) -> super::DWORD {
        (self._bitfield >> 11) & 1
    }
    pub fn set_fNull(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 11)) | ((value & 1) << 11);
    }
    pub fn fRtsControl(&self) -> super::DWORD {
        (self._bitfield >> 12) & 3
    }
    pub fn set_fRtsControl(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(3 << 12)) | ((value & 3) << 12);
    }
    pub fn fAbortOnError(&self) -> super::DWORD {
        (self._bitfield >> 14) & 1
    }
    pub fn set_fAbortOnError(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(1 << 14)) | ((value & 1) << 14);
    }
    pub fn fDummy2(&self) -> super::DWORD {
        self._bitfield >> 15
    }
    pub fn set_fDummy2(&mut self, value: super::DWORD) {
        self._bitfield = (self._bitfield & !(131071 << 15)) | ((value & 131071) << 15);
    }
}
pub const DDD_EXACT_MATCH_ON_REMOVE: i32 = 4;
pub const DDD_LUID_BROADCAST_DRIVE: i32 = 16;
pub const DDD_NO_BROADCAST_SYSTEM: i32 = 8;
pub const DDD_RAW_TARGET_PATH: i32 = 1;
pub const DDD_REMOVE_DEFINITION: i32 = 2;
pub const DEACTIVATE_ACTCTX_FLAG_FORCE_EARLY_DEACTIVATION: i32 = 1;
pub const DEBUG_ONLY_THIS_PROCESS: i32 = 2;
pub const DEBUG_PROCESS: i32 = 1;
pub const DEPPolicyAlwaysOff: DEP_SYSTEM_POLICY_TYPE = 0;
pub const DEPPolicyAlwaysOn: DEP_SYSTEM_POLICY_TYPE = 1;
pub const DEPPolicyOptIn: DEP_SYSTEM_POLICY_TYPE = 2;
pub const DEPPolicyOptOut: DEP_SYSTEM_POLICY_TYPE = 3;
pub const DEPTotalPolicyCount: DEP_SYSTEM_POLICY_TYPE = 4;
pub type DEP_SYSTEM_POLICY_TYPE = i32;
pub const DETACHED_PROCESS: i32 = 8;
pub const DOCKINFO_DOCKED: i32 = 2;
pub const DOCKINFO_UNDOCKED: i32 = 1;
pub const DOCKINFO_USER_DOCKED: i32 = 6;
pub const DOCKINFO_USER_SUPPLIED: i32 = 4;
pub const DOCKINFO_USER_UNDOCKED: i32 = 5;
pub const DRIVE_CDROM: i32 = 5;
pub const DRIVE_FIXED: i32 = 3;
pub const DRIVE_NO_ROOT_DIR: i32 = 1;
pub const DRIVE_RAMDISK: i32 = 6;
pub const DRIVE_REMOTE: i32 = 4;
pub const DRIVE_REMOVABLE: i32 = 2;
pub const DRIVE_UNKNOWN: i32 = 0;
pub const DTR_CONTROL_DISABLE: i32 = 0;
pub const DTR_CONTROL_ENABLE: i32 = 1;
pub const DTR_CONTROL_HANDSHAKE: i32 = 2;
pub const EFSRPC_SECURE_ONLY: i32 = 8;
pub const EFS_DROP_ALTERNATE_STREAMS: i32 = 16;
pub const EFS_USE_RECOVERY_KEYS: i32 = 1;
pub const EVENPARITY: i32 = 2;
pub const EVENTLOG_FULL_INFO: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EVENTLOG_FULL_INFORMATION {
    pub dwFull: super::DWORD,
}
pub const EV_BREAK: i32 = 64;
pub const EV_CTS: i32 = 8;
pub const EV_DSR: i32 = 16;
pub const EV_ERR: i32 = 128;
pub const EV_EVENT1: i32 = 2048;
pub const EV_EVENT2: i32 = 4096;
pub const EV_PERR: i32 = 512;
pub const EV_RING: i32 = 256;
pub const EV_RLSD: i32 = 32;
pub const EV_RX80FULL: i32 = 1024;
pub const EV_RXCHAR: i32 = 1;
pub const EV_RXFLAG: i32 = 2;
pub const EV_TXEMPTY: i32 = 4;
pub const EXTENDED_STARTUPINFO_PRESENT: i32 = 524288;
pub const ExtendedFileIdType: FILE_ID_TYPE = 2;
pub const FAIL_FAST_GENERATE_EXCEPTION_ADDRESS: i32 = 1;
pub const FAIL_FAST_NO_HARD_ERROR_DLG: i32 = 2;
pub const FIBER_FLAG_FLOAT_SWITCH: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_ALIGNMENT_INFO {
    pub AlignmentRequirement: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_ALLOCATION_INFO {
    pub AllocationSize: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_ALLOCATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_ATTRIBUTE_TAG_INFO {
    pub FileAttributes: super::DWORD,
    pub ReparseTag: super::DWORD,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_BASIC_INFO {
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub FileAttributes: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_BASIC_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_BEGIN: i32 = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_CASE_SENSITIVE_INFO {
    pub Flags: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_COMPRESSION_INFO {
    pub CompressedFileSize: super::LARGE_INTEGER,
    pub CompressionFormat: super::WORD,
    pub CompressionUnitShift: super::UCHAR,
    pub ChunkShift: super::UCHAR,
    pub ClusterShift: super::UCHAR,
    pub Reserved: [super::UCHAR; 3],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_COMPRESSION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_CURRENT: i32 = 1;
pub const FILE_DIR_DISALLOWED: i32 = 9;
pub const FILE_DISPOSITION_FLAG_DELETE: i32 = 1;
pub const FILE_DISPOSITION_FLAG_DO_NOT_DELETE: i32 = 0;
pub const FILE_DISPOSITION_FLAG_FORCE_IMAGE_SECTION_CHECK: i32 = 4;
pub const FILE_DISPOSITION_FLAG_IGNORE_READONLY_ATTRIBUTE: i32 = 16;
pub const FILE_DISPOSITION_FLAG_ON_CLOSE: i32 = 8;
pub const FILE_DISPOSITION_FLAG_POSIX_SEMANTICS: i32 = 2;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_DISPOSITION_INFO {
    pub DeleteFile: super::BOOLEAN,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_DISPOSITION_INFO_EX {
    pub Flags: super::DWORD,
}
pub const FILE_ENCRYPTABLE: i32 = 0;
pub const FILE_END: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_END_OF_FILE_INFO {
    pub EndOfFile: super::LARGE_INTEGER,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_END_OF_FILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_FLAG_BACKUP_SEMANTICS: i32 = 33554432;
pub const FILE_FLAG_DELETE_ON_CLOSE: i32 = 67108864;
pub const FILE_FLAG_DISALLOW_PATH_REDIRECTS: i32 = 65536;
pub const FILE_FLAG_FIRST_PIPE_INSTANCE: i32 = 524288;
pub const FILE_FLAG_IGNORE_IMPERSONATED_DEVICEMAP: i32 = 131072;
pub const FILE_FLAG_NO_BUFFERING: i32 = 536870912;
pub const FILE_FLAG_OPEN_NO_RECALL: i32 = 1048576;
pub const FILE_FLAG_OPEN_REPARSE_POINT: i32 = 2097152;
pub const FILE_FLAG_OPEN_REQUIRING_OPLOCK: i32 = 262144;
pub const FILE_FLAG_OVERLAPPED: i32 = 1073741824;
pub const FILE_FLAG_POSIX_SEMANTICS: i32 = 16777216;
pub const FILE_FLAG_RANDOM_ACCESS: i32 = 268435456;
pub const FILE_FLAG_SEQUENTIAL_SCAN: i32 = 134217728;
pub const FILE_FLAG_SESSION_AWARE: i32 = 8388608;
pub const FILE_FLAG_WRITE_THROUGH: u32 = 2147483648;
pub const FILE_FLUSH_DATA: FILE_FLUSH_MODE = 1;
pub const FILE_FLUSH_DEFAULT: FILE_FLUSH_MODE = 0;
pub const FILE_FLUSH_MIN_METADATA: FILE_FLUSH_MODE = 2;
pub type FILE_FLUSH_MODE = i32;
pub const FILE_FLUSH_NO_SYNC: FILE_FLUSH_MODE = 3;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_FULL_DIR_INFO {
    pub NextEntryOffset: super::ULONG,
    pub FileIndex: super::ULONG,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: super::ULONG,
    pub FileNameLength: super::ULONG,
    pub EaSize: super::ULONG,
    pub FileName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_FULL_DIR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_ID_BOTH_DIR_INFO {
    pub NextEntryOffset: super::DWORD,
    pub FileIndex: super::DWORD,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: super::DWORD,
    pub FileNameLength: super::DWORD,
    pub EaSize: super::DWORD,
    pub ShortNameLength: super::CCHAR,
    pub ShortName: [super::WCHAR; 12],
    pub FileId: super::LARGE_INTEGER,
    pub FileName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_ID_BOTH_DIR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_ID_DESCRIPTOR {
    pub dwSize: super::DWORD,
    pub Type: FILE_ID_TYPE,
    pub Anonymous: FILE_ID_DESCRIPTOR_0,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_ID_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FILE_ID_DESCRIPTOR_0 {
    pub FileId: super::LARGE_INTEGER,
    pub ObjectId: windows_core::GUID,
    pub ExtendedFileId: super::FILE_ID_128,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_ID_DESCRIPTOR_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_ID_EXTD_DIR_INFO {
    pub NextEntryOffset: super::ULONG,
    pub FileIndex: super::ULONG,
    pub CreationTime: super::LARGE_INTEGER,
    pub LastAccessTime: super::LARGE_INTEGER,
    pub LastWriteTime: super::LARGE_INTEGER,
    pub ChangeTime: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub AllocationSize: super::LARGE_INTEGER,
    pub FileAttributes: super::ULONG,
    pub FileNameLength: super::ULONG,
    pub EaSize: super::ULONG,
    pub ReparsePointTag: super::ULONG,
    pub FileId: super::FILE_ID_128,
    pub FileName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_ID_EXTD_DIR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_ID_INFO {
    pub VolumeSerialNumber: super::ULONGLONG,
    pub FileId: super::FILE_ID_128,
}
pub type FILE_ID_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_IO_PRIORITY_HINT_INFO {
    pub PriorityHint: PRIORITY_HINT,
}
pub const FILE_IS_ENCRYPTED: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_NAME_INFO {
    pub FileNameLength: super::DWORD,
    pub FileName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_NAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_NAME_NORMALIZED: i32 = 0;
pub const FILE_NAME_OPENED: i32 = 8;
pub const FILE_READ_ONLY: i32 = 8;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct FILE_REMOTE_PROTOCOL_INFO {
    pub StructureVersion: super::USHORT,
    pub StructureSize: super::USHORT,
    pub Protocol: super::ULONG,
    pub ProtocolMajorVersion: super::USHORT,
    pub ProtocolMinorVersion: super::USHORT,
    pub ProtocolRevision: super::USHORT,
    pub Reserved: super::USHORT,
    pub Flags: super::ULONG,
    pub GenericReserved: FILE_REMOTE_PROTOCOL_INFO_0,
    pub ProtocolSpecific: FILE_REMOTE_PROTOCOL_INFO_1,
}
#[cfg(feature = "shared")]
impl Default for FILE_REMOTE_PROTOCOL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFO_0 {
    pub Reserved: [super::ULONG; 8],
}
#[cfg(feature = "shared")]
impl Default for FILE_REMOTE_PROTOCOL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union FILE_REMOTE_PROTOCOL_INFO_1 {
    pub Smb2: FILE_REMOTE_PROTOCOL_INFO_1_0,
    pub Reserved: [super::ULONG; 16],
}
#[cfg(feature = "shared")]
impl Default for FILE_REMOTE_PROTOCOL_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0 {
    pub Server: FILE_REMOTE_PROTOCOL_INFO_1_0_0,
    pub Share: FILE_REMOTE_PROTOCOL_INFO_1_0_1,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0_0 {
    pub Capabilities: super::ULONG,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_REMOTE_PROTOCOL_INFO_1_0_1 {
    pub Capabilities: super::ULONG,
    pub ShareFlags: super::ULONG,
}
pub const FILE_RENAME_FLAG_POSIX_SEMANTICS: i32 = 2;
pub const FILE_RENAME_FLAG_REPLACE_IF_EXISTS: i32 = 1;
pub const FILE_RENAME_FLAG_SUPPRESS_PIN_STATE_INHERITANCE: i32 = 4;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_RENAME_INFO {
    pub Anonymous: FILE_RENAME_INFO_0,
    pub RootDirectory: super::HANDLE,
    pub FileNameLength: super::DWORD,
    pub FileName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_RENAME_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub union FILE_RENAME_INFO_0 {
    pub ReplaceIfExists: super::BOOLEAN,
    pub Flags: super::DWORD,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_RENAME_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_ROOT_DIR: i32 = 3;
pub const FILE_SKIP_COMPLETION_PORT_ON_SUCCESS: i32 = 1;
pub const FILE_SKIP_SET_EVENT_ON_HANDLE: i32 = 2;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_STANDARD_INFO {
    pub AllocationSize: super::LARGE_INTEGER,
    pub EndOfFile: super::LARGE_INTEGER,
    pub NumberOfLinks: super::DWORD,
    pub DeletePending: super::BOOLEAN,
    pub Directory: super::BOOLEAN,
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_STANDARD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct FILE_STORAGE_INFO {
    pub LogicalBytesPerSector: super::ULONG,
    pub PhysicalBytesPerSectorForAtomicity: super::ULONG,
    pub PhysicalBytesPerSectorForPerformance: super::ULONG,
    pub FileSystemEffectivePhysicalBytesPerSectorForAtomicity: super::ULONG,
    pub Flags: super::ULONG,
    pub ByteOffsetForSectorAlignment: super::ULONG,
    pub ByteOffsetForPartitionAlignment: super::ULONG,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct FILE_STREAM_INFO {
    pub NextEntryOffset: super::DWORD,
    pub StreamNameLength: super::DWORD,
    pub StreamSize: super::LARGE_INTEGER,
    pub StreamAllocationSize: super::LARGE_INTEGER,
    pub StreamName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for FILE_STREAM_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FILE_SYSTEM_ATTR: i32 = 2;
pub const FILE_SYSTEM_DIR: i32 = 4;
pub const FILE_SYSTEM_NOT_SUPPORT: i32 = 6;
pub const FILE_TYPE_CHAR: i32 = 2;
pub const FILE_TYPE_DISK: i32 = 1;
pub const FILE_TYPE_PIPE: i32 = 3;
pub const FILE_TYPE_REMOTE: i32 = 32768;
pub const FILE_TYPE_UNKNOWN: i32 = 0;
pub const FILE_UNKNOWN: i32 = 5;
pub const FILE_USER_DISALLOWED: i32 = 7;
pub type FILE_WRITE_FLAGS = i32;
pub const FILE_WRITE_FLAGS_NONE: FILE_WRITE_FLAGS = 0;
pub const FILE_WRITE_FLAGS_WRITE_THROUGH: FILE_WRITE_FLAGS = 1;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_ASSEMBLY_METADATA: i32 = 4;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_FLAGS: i32 = 2;
pub const FIND_ACTCTX_SECTION_KEY_RETURN_HACTCTX: i32 = 1;
pub const FORMAT_MESSAGE_ALLOCATE_BUFFER: i32 = 256;
pub const FORMAT_MESSAGE_ARGUMENT_ARRAY: i32 = 8192;
pub const FORMAT_MESSAGE_FROM_HMODULE: i32 = 2048;
pub const FORMAT_MESSAGE_FROM_STRING: i32 = 1024;
pub const FORMAT_MESSAGE_FROM_SYSTEM: i32 = 4096;
pub const FORMAT_MESSAGE_IGNORE_INSERTS: i32 = 512;
pub const FORMAT_MESSAGE_MAX_WIDTH_MASK: i32 = 255;
pub const FS_CASE_IS_PRESERVED: i32 = 2;
pub const FS_CASE_SENSITIVE: i32 = 1;
pub const FS_FILE_COMPRESSION: i32 = 16;
pub const FS_FILE_ENCRYPTION: i32 = 131072;
pub const FS_PERSISTENT_ACLS: i32 = 8;
pub const FS_UNICODE_STORED_ON_DISK: i32 = 4;
pub const FS_VOL_IS_COMPRESSED: i32 = 32768;
pub const FileIdType: FILE_ID_TYPE = 0;
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_A: windows_core::PCSTR = windows_core::s!("GetSystemWow64DirectoryA");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_T: windows_core::PCSTR = windows_core::s!("GetSystemWow64DirectoryA");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_A_W: windows_core::PCWSTR = windows_core::w!("GetSystemWow64DirectoryA");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_A: windows_core::PCSTR = windows_core::s!("GetSystemWow64DirectoryA");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_T: windows_core::PCSTR = windows_core::s!("GetSystemWow64DirectoryA");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_T_W: windows_core::PCWSTR = windows_core::w!("GetSystemWow64DirectoryA");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_A: windows_core::PCSTR = windows_core::s!("GetSystemWow64DirectoryW");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_T: windows_core::PCSTR = windows_core::s!("GetSystemWow64DirectoryW");
pub const GET_SYSTEM_WOW64_DIRECTORY_NAME_W_W: windows_core::PCWSTR = windows_core::w!("GetSystemWow64DirectoryW");
pub const GET_TAPE_DRIVE_INFORMATION: i32 = 1;
pub const GET_TAPE_MEDIA_INFORMATION: i32 = 0;
pub const GHND: i32 = 66;
pub const GMEM_DDESHARE: i32 = 8192;
pub const GMEM_DISCARDABLE: i32 = 256;
pub const GMEM_DISCARDED: i32 = 16384;
pub const GMEM_FIXED: i32 = 0;
pub const GMEM_INVALID_HANDLE: i32 = 32768;
pub const GMEM_LOCKCOUNT: i32 = 255;
pub const GMEM_LOWER: i32 = 4096;
pub const GMEM_MODIFY: i32 = 128;
pub const GMEM_MOVEABLE: i32 = 2;
pub const GMEM_NOCOMPACT: i32 = 16;
pub const GMEM_NODISCARD: i32 = 32;
pub const GMEM_NOTIFY: i32 = 16384;
pub const GMEM_NOT_BANKED: i32 = 4096;
pub const GMEM_SHARE: i32 = 8192;
pub const GMEM_VALID_FLAGS: i32 = 32626;
pub const GMEM_ZEROINIT: i32 = 64;
pub const GPTR: i32 = 64;
pub const HANDLE_FLAG_INHERIT: i32 = 1;
pub const HANDLE_FLAG_PROTECT_FROM_CLOSE: i32 = 2;
pub const HIGH_PRIORITY_CLASS: i32 = 128;
pub const HINSTANCE_ERROR: i32 = 32;
pub const HW_PROFILE_GUIDLEN: i32 = 39;
#[cfg(feature = "shared")]
pub type HW_PROFILE_INFO = HW_PROFILE_INFOA;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HW_PROFILE_INFOA {
    pub dwDockInfo: super::DWORD,
    pub szHwProfileGuid: [super::CHAR; 39],
    pub szHwProfileName: [super::CHAR; 80],
}
#[cfg(feature = "shared")]
impl Default for HW_PROFILE_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HW_PROFILE_INFOW {
    pub dwDockInfo: super::DWORD,
    pub szHwProfileGuid: [super::WCHAR; 39],
    pub szHwProfileName: [super::WCHAR; 80],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for HW_PROFILE_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const IDLE_PRIORITY_CLASS: i32 = 64;
pub const IE_BADID: i32 = -1;
pub const IE_BAUDRATE: i32 = -12;
pub const IE_BYTESIZE: i32 = -11;
pub const IE_DEFAULT: i32 = -5;
pub const IE_HARDWARE: i32 = -10;
pub const IE_MEMORY: i32 = -4;
pub const IE_NOPEN: i32 = -3;
pub const IE_OPEN: i32 = -2;
pub const IGNORE: i32 = 0;
pub const INFINITE: u32 = 4294967295;
pub const INHERIT_CALLER_PRIORITY: i32 = 131072;
pub const INHERIT_PARENT_AFFINITY: i32 = 65536;
#[cfg(feature = "minwindef")]
pub const INVALID_ATOM: super::ATOM = 0;
pub const IoPriorityHintLow: PRIORITY_HINT = 1;
pub const IoPriorityHintNormal: PRIORITY_HINT = 2;
pub const IoPriorityHintVeryLow: PRIORITY_HINT = 0;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JIT_DEBUG_INFO {
    pub dwSize: super::DWORD,
    pub dwProcessorArchitecture: super::DWORD,
    pub dwThreadID: super::DWORD,
    pub dwReserved0: super::DWORD,
    pub lpExceptionAddress: super::ULONG64,
    pub lpExceptionRecord: super::ULONG64,
    pub lpContextRecord: super::ULONG64,
}
#[cfg(feature = "shared")]
pub type JIT_DEBUG_INFO32 = JIT_DEBUG_INFO;
#[cfg(feature = "shared")]
pub type JIT_DEBUG_INFO64 = JIT_DEBUG_INFO;
pub const LOGON32_LOGON_BATCH: i32 = 4;
pub const LOGON32_LOGON_INTERACTIVE: i32 = 2;
pub const LOGON32_LOGON_NETWORK: i32 = 3;
pub const LOGON32_LOGON_NETWORK_CLEARTEXT: i32 = 8;
pub const LOGON32_LOGON_NEW_CREDENTIALS: i32 = 9;
pub const LOGON32_LOGON_SERVICE: i32 = 5;
pub const LOGON32_LOGON_UNLOCK: i32 = 7;
pub const LOGON32_PROVIDER_DEFAULT: i32 = 0;
pub const LOGON32_PROVIDER_VIRTUAL: i32 = 4;
pub const LOGON32_PROVIDER_WINNT35: i32 = 1;
pub const LOGON32_PROVIDER_WINNT40: i32 = 2;
pub const LOGON32_PROVIDER_WINNT50: i32 = 3;
pub const LOGON_NETCREDENTIALS_ONLY: i32 = 2;
pub const LOGON_WITH_PROFILE: i32 = 1;
pub const LOGON_ZERO_PASSWORD_BUFFER: u32 = 2147483648;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPCOMMCONFIG = *mut COMMCONFIG;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPCOMMPROP = *mut COMMPROP;
#[cfg(feature = "shared")]
pub type LPCOMMTIMEOUTS = *mut COMMTIMEOUTS;
#[cfg(feature = "shared")]
pub type LPCOMSTAT = *mut COMSTAT;
#[cfg(feature = "shared")]
pub type LPDCB = *mut DCB;
#[cfg(feature = "shared")]
pub type LPEVENTLOG_FULL_INFORMATION = *mut EVENTLOG_FULL_INFORMATION;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPEXCEPTION_POINTERS = *mut super::EXCEPTION_POINTERS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPEXCEPTION_RECORD = *mut super::EXCEPTION_RECORD;
#[cfg(feature = "minwindef")]
pub type LPFIBER_START_ROUTINE = Option<unsafe extern "system" fn(lpfiberparameter: super::LPVOID)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPFILE_ID_DESCRIPTOR = *mut FILE_ID_DESCRIPTOR;
#[cfg(feature = "shared")]
pub type LPHW_PROFILE_INFO = *mut HW_PROFILE_INFOA;
#[cfg(feature = "shared")]
pub type LPHW_PROFILE_INFOA = *mut HW_PROFILE_INFOA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPHW_PROFILE_INFOW = *mut HW_PROFILE_INFOW;
#[cfg(feature = "shared")]
pub type LPJIT_DEBUG_INFO = *mut JIT_DEBUG_INFO;
#[cfg(feature = "shared")]
pub type LPJIT_DEBUG_INFO32 = *mut JIT_DEBUG_INFO;
#[cfg(feature = "shared")]
pub type LPJIT_DEBUG_INFO64 = *mut JIT_DEBUG_INFO;
#[cfg(target_arch = "x86")]
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPLDT_ENTRY = *mut super::LDT_ENTRY;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub type LPLDT_ENTRY = *mut core::ffi::c_void;
#[cfg(feature = "shared")]
pub type LPMEMORYSTATUS = *mut MEMORYSTATUS;
#[cfg(feature = "shared")]
pub type LPOFSTRUCT = *mut OFSTRUCT;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
pub type LPPOWER_REQUEST_CONTEXT = *mut super::REASON_CONTEXT;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type LPPROGRESS_ROUTINE = Option<unsafe extern "system" fn(totalfilesize: super::LARGE_INTEGER, totalbytestransferred: super::LARGE_INTEGER, streamsize: super::LARGE_INTEGER, streambytestransferred: super::LARGE_INTEGER, dwstreamnumber: super::DWORD, dwcallbackreason: super::DWORD, hsourcefile: super::HANDLE, hdestinationfile: super::HANDLE, lpdata: super::LPVOID) -> super::DWORD>;
#[cfg(all(feature = "processthreadsapi", feature = "shared", feature = "winnt"))]
pub type LPSTARTUPINFOEX = *mut STARTUPINFOEXA;
#[cfg(all(feature = "processthreadsapi", feature = "shared", feature = "winnt"))]
pub type LPSTARTUPINFOEXA = *mut STARTUPINFOEXA;
#[cfg(all(feature = "processthreadsapi", feature = "shared", feature = "winnt"))]
pub type LPSTARTUPINFOEXW = *mut STARTUPINFOEXW;
#[cfg(feature = "shared")]
pub type LPSYSTEM_POWER_STATUS = *mut SYSTEM_POWER_STATUS;
pub const LPTx: i32 = 128;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type LPWIN32_STREAM_ID = *mut WIN32_STREAM_ID;
pub const MARKPARITY: i32 = 3;
pub const MAXINTATOM: i32 = 49152;
pub const MAX_COMPUTERNAME_LENGTH: i32 = 15;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MEMORYSTATUS {
    pub dwLength: super::DWORD,
    pub dwMemoryLoad: super::DWORD,
    pub dwTotalPhys: super::SIZE_T,
    pub dwAvailPhys: super::SIZE_T,
    pub dwTotalPageFile: super::SIZE_T,
    pub dwAvailPageFile: super::SIZE_T,
    pub dwTotalVirtual: super::SIZE_T,
    pub dwAvailVirtual: super::SIZE_T,
}
pub const MICROSOFT_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: i32 = 0;
pub const MICROSOFT_WINDOWS_WINBASE_H_DEFINE_INTERLOCKED_CPLUSPLUS_OVERLOADS: i32 = 1;
pub const MOVEFILE_COPY_ALLOWED: i32 = 2;
pub const MOVEFILE_CREATE_HARDLINK: i32 = 16;
pub const MOVEFILE_DELAY_UNTIL_REBOOT: i32 = 4;
pub const MOVEFILE_FAIL_IF_NOT_TRACKABLE: i32 = 32;
pub const MOVEFILE_REPLACE_EXISTING: i32 = 1;
pub const MOVEFILE_WRITE_THROUGH: i32 = 8;
#[cfg(feature = "shared")]
pub const MS_CTS_ON: super::DWORD = 16;
#[cfg(feature = "shared")]
pub const MS_DSR_ON: super::DWORD = 32;
#[cfg(feature = "shared")]
pub const MS_RING_ON: super::DWORD = 64;
#[cfg(feature = "shared")]
pub const MS_RLSD_ON: super::DWORD = 128;
pub const MaximumFileIdType: FILE_ID_TYPE = 3;
pub const MaximumIoPriorityHintType: PRIORITY_HINT = 3;
pub const NMPWAIT_NOWAIT: i32 = 1;
pub const NMPWAIT_USE_DEFAULT_WAIT: i32 = 0;
pub const NMPWAIT_WAIT_FOREVER: u32 = 4294967295;
pub const NOPARITY: i32 = 0;
pub const NORMAL_PRIORITY_CLASS: i32 = 32;
pub const ODDPARITY: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OFSTRUCT {
    pub cBytes: super::BYTE,
    pub fFixedDisk: super::BYTE,
    pub nErrCode: super::WORD,
    pub Reserved1: super::WORD,
    pub Reserved2: super::WORD,
    pub szPathName: [super::CHAR; 128],
}
#[cfg(feature = "shared")]
impl Default for OFSTRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const OFS_MAXPATHNAME: i32 = 128;
pub const OF_CANCEL: i32 = 2048;
pub const OF_CREATE: i32 = 4096;
pub const OF_DELETE: i32 = 512;
pub const OF_EXIST: i32 = 16384;
pub const OF_PARSE: i32 = 256;
pub const OF_PROMPT: i32 = 8192;
pub const OF_READ: i32 = 0;
pub const OF_READWRITE: i32 = 2;
pub const OF_REOPEN: i32 = 32768;
pub const OF_SHARE_COMPAT: i32 = 0;
pub const OF_SHARE_DENY_NONE: i32 = 64;
pub const OF_SHARE_DENY_READ: i32 = 48;
pub const OF_SHARE_DENY_WRITE: i32 = 32;
pub const OF_SHARE_EXCLUSIVE: i32 = 16;
pub const OF_VERIFY: i32 = 1024;
pub const OF_WRITE: i32 = 1;
pub const ONE5STOPBITS: i32 = 1;
pub const ONESTOPBIT: i32 = 0;
pub const OPERATION_API_VERSION: i32 = 1;
pub const OPERATION_END_DISCARD: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPERATION_END_PARAMETERS {
    pub Version: super::ULONG,
    pub OperationId: OPERATION_ID,
    pub Flags: super::ULONG,
}
pub type OPERATION_ID = u32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct OPERATION_START_PARAMETERS {
    pub Version: super::ULONG,
    pub OperationId: OPERATION_ID,
    pub Flags: super::ULONG,
}
pub const OPERATION_START_TRACE_CURRENT_THREAD: i32 = 1;
pub const OVERWRITE_HIDDEN: i32 = 4;
pub const ObjectIdType: FILE_ID_TYPE = 1;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PACTCTX = *mut ACTCTXA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PACTCTXA = *mut ACTCTXA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PACTCTXW = *mut ACTCTXW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PACTCTX_SECTION_KEYED_DATA = *mut ACTCTX_SECTION_KEYED_DATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PACTCTX_SECTION_KEYED_DATA_2600 = *mut ACTCTX_SECTION_KEYED_DATA_2600;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA = *mut ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PACTIVATION_CONTEXT_BASIC_INFORMATION = *mut ACTIVATION_CONTEXT_BASIC_INFORMATION;
#[cfg(feature = "shared")]
pub const PARITY_EVEN: super::WORD = 1024;
#[cfg(feature = "shared")]
pub const PARITY_MARK: super::WORD = 2048;
#[cfg(feature = "shared")]
pub const PARITY_NONE: super::WORD = 256;
#[cfg(feature = "shared")]
pub const PARITY_ODD: super::WORD = 512;
#[cfg(feature = "shared")]
pub const PARITY_SPACE: super::WORD = 4096;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCACTCTX = *const ACTCTXA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCACTCTXA = *const ACTCTXA;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PCACTCTXW = *const ACTCTXW;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCACTCTX_SECTION_KEYED_DATA = *const ACTCTX_SECTION_KEYED_DATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCACTCTX_SECTION_KEYED_DATA_2600 = *const ACTCTX_SECTION_KEYED_DATA_2600;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA = *const ACTCTX_SECTION_KEYED_DATA_ASSEMBLY_METADATA;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCACTIVATION_CONTEXT_BASIC_INFORMATION = *const ACTIVATION_CONTEXT_BASIC_INFORMATION;
#[cfg(feature = "shared")]
pub const PCF_16BITMODE: super::DWORD = 512;
#[cfg(feature = "shared")]
pub const PCF_DTRDSR: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const PCF_INTTIMEOUTS: super::DWORD = 128;
#[cfg(feature = "shared")]
pub const PCF_PARITY_CHECK: super::DWORD = 8;
#[cfg(feature = "shared")]
pub const PCF_RLSD: super::DWORD = 4;
#[cfg(feature = "shared")]
pub const PCF_RTSCTS: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const PCF_SETXCHAR: super::DWORD = 32;
#[cfg(feature = "shared")]
pub const PCF_SPECIALCHARS: super::DWORD = 256;
#[cfg(feature = "shared")]
pub const PCF_TOTALTIMEOUTS: super::DWORD = 64;
#[cfg(feature = "shared")]
pub const PCF_XONXOFF: super::DWORD = 16;
pub type PCOPYFILE2_CREATE_OPLOCK_KEYS = *mut COPYFILE2_CREATE_OPLOCK_KEYS;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PCOPYFILE2_PROGRESS_ROUTINE = Option<unsafe extern "system" fn(pmessage: *const COPYFILE2_MESSAGE, pvcallbackcontext: super::PVOID) -> COPYFILE2_MESSAGE_ACTION>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFE_EXPORT_FUNC = Option<unsafe extern "system" fn(pbdata: super::PBYTE, pvcallbackcontext: super::PVOID, ullength: super::ULONG) -> super::DWORD>;
#[cfg(all(feature = "minwindef", feature = "shared", feature = "winnt"))]
pub type PFE_IMPORT_FUNC = Option<unsafe extern "system" fn(pbdata: super::PBYTE, pvcallbackcontext: super::PVOID, ullength: super::PULONG) -> super::DWORD>;
#[cfg(feature = "minwindef")]
pub type PFIBER_CALLOUT_ROUTINE = Option<unsafe extern "system" fn(lpparameter: super::LPVOID) -> super::LPVOID>;
#[cfg(feature = "minwindef")]
pub type PFIBER_START_ROUTINE = Option<unsafe extern "system" fn(lpfiberparameter: super::LPVOID)>;
#[cfg(feature = "shared")]
pub type PFILE_ALIGNMENT_INFO = *mut FILE_ALIGNMENT_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_ALLOCATION_INFO = *mut FILE_ALLOCATION_INFO;
#[cfg(feature = "shared")]
pub type PFILE_ATTRIBUTE_TAG_INFO = *mut FILE_ATTRIBUTE_TAG_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_BASIC_INFO = *mut FILE_BASIC_INFO;
#[cfg(feature = "shared")]
pub type PFILE_CASE_SENSITIVE_INFO = *mut FILE_CASE_SENSITIVE_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_COMPRESSION_INFO = *mut FILE_COMPRESSION_INFO;
#[cfg(feature = "shared")]
pub type PFILE_DISPOSITION_INFO = *mut FILE_DISPOSITION_INFO;
#[cfg(feature = "shared")]
pub type PFILE_DISPOSITION_INFO_EX = *mut FILE_DISPOSITION_INFO_EX;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_END_OF_FILE_INFO = *mut FILE_END_OF_FILE_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_FULL_DIR_INFO = *mut FILE_FULL_DIR_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_ID_BOTH_DIR_INFO = *mut FILE_ID_BOTH_DIR_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_ID_EXTD_DIR_INFO = *mut FILE_ID_EXTD_DIR_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_ID_INFO = *mut FILE_ID_INFO;
pub type PFILE_ID_TYPE = *mut FILE_ID_TYPE;
pub type PFILE_IO_PRIORITY_HINT_INFO = *mut FILE_IO_PRIORITY_HINT_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_NAME_INFO = *mut FILE_NAME_INFO;
#[cfg(feature = "shared")]
pub type PFILE_REMOTE_PROTOCOL_INFO = *mut FILE_REMOTE_PROTOCOL_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_RENAME_INFO = *mut FILE_RENAME_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_STANDARD_INFO = *mut FILE_STANDARD_INFO;
#[cfg(feature = "shared")]
pub type PFILE_STORAGE_INFO = *mut FILE_STORAGE_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PFILE_STREAM_INFO = *mut FILE_STREAM_INFO;
#[cfg(feature = "shared")]
pub type PGET_SYSTEM_WOW64_DIRECTORY_A = Option<unsafe extern "system" fn(lpbuffer: windows_core::PSTR, usize: super::UINT) -> super::UINT>;
#[cfg(feature = "shared")]
pub type PGET_SYSTEM_WOW64_DIRECTORY_W = Option<unsafe extern "system" fn(lpbuffer: windows_core::PWSTR, usize: super::UINT) -> super::UINT>;
pub const PIPE_ACCEPT_REMOTE_CLIENTS: i32 = 0;
pub const PIPE_ACCESS_DUPLEX: i32 = 3;
pub const PIPE_ACCESS_INBOUND: i32 = 1;
pub const PIPE_ACCESS_OUTBOUND: i32 = 2;
pub const PIPE_CLIENT_END: i32 = 0;
pub const PIPE_NOWAIT: i32 = 1;
pub const PIPE_READMODE_BYTE: i32 = 0;
pub const PIPE_READMODE_MESSAGE: i32 = 2;
pub const PIPE_REJECT_REMOTE_CLIENTS: i32 = 8;
pub const PIPE_SERVER_END: i32 = 1;
pub const PIPE_TYPE_BYTE: i32 = 0;
pub const PIPE_TYPE_MESSAGE: i32 = 4;
pub const PIPE_UNLIMITED_INSTANCES: i32 = 255;
pub const PIPE_WAIT: i32 = 0;
#[cfg(feature = "shared")]
pub type POFSTRUCT = *mut OFSTRUCT;
#[cfg(feature = "shared")]
pub type POPERATION_END_PARAMETERS = *mut OPERATION_END_PARAMETERS;
#[cfg(feature = "shared")]
pub type POPERATION_START_PARAMETERS = *mut OPERATION_START_PARAMETERS;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
pub type POWER_REQUEST_CONTEXT = super::REASON_CONTEXT;
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "shared"))]
pub type PPOWER_REQUEST_CONTEXT = *mut super::REASON_CONTEXT;
#[cfg(feature = "shared")]
pub type PPROCESS_CREATION_SME_VECTOR_LENGTH = *mut PROCESS_CREATION_SME_VECTOR_LENGTH;
#[cfg(feature = "shared")]
pub type PPROCESS_CREATION_SVE_VECTOR_LENGTH = *mut PROCESS_CREATION_SVE_VECTOR_LENGTH;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PQUERYACTCTXW_FUNC = Option<unsafe extern "system" fn(dwflags: super::DWORD, hactctx: super::HANDLE, pvsubinstance: super::PVOID, ulinfoclass: super::ULONG, pvbuffer: super::PVOID, cbbuffer: super::SIZE_T, pcbwrittenorrequired: *mut super::SIZE_T) -> windows_core::BOOL>;
pub type PRIORITY_HINT = i32;
pub const PROCESS_CREATION_ALL_APPLICATION_PACKAGES_OPT_OUT: i32 = 1;
pub const PROCESS_CREATION_CHILD_PROCESS_OVERRIDE: i32 = 2;
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED: i32 = 1;
pub const PROCESS_CREATION_CHILD_PROCESS_RESTRICTED_UNLESS_SECURE: i32 = 4;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_DISABLE_PROCESS_TREE: i32 = 2;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_ENABLE_PROCESS_TREE: i32 = 1;
pub const PROCESS_CREATION_DESKTOP_APP_BREAKAWAY_OVERRIDE: i32 = 4;
pub const PROCESS_CREATION_DESKTOP_APP_TRUSTED_LAUNCH_ALLOW_WINDOWS_HOST: i32 = 8;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_BLOCK_NON_CET_BINARIES_ALWAYS_OFF: u64 = 137438953472;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_BLOCK_NON_CET_BINARIES_ALWAYS_ON: u64 = 68719476736;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_BLOCK_NON_CET_BINARIES_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_BLOCK_NON_CET_BINARIES_MASK: u64 = 206158430208;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_BLOCK_NON_CET_BINARIES_RESERVED: u64 = 206158430208;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_CET_USER_SHADOW_STACKS_ALWAYS_OFF: u64 = 536870912;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_CET_USER_SHADOW_STACKS_ALWAYS_ON: u64 = 268435456;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_CET_USER_SHADOW_STACKS_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_CET_USER_SHADOW_STACKS_MASK: u64 = 805306368;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_CET_USER_SHADOW_STACKS_RESERVED: u64 = 805306368;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_ALWAYS_OFF: u64 = 8589934592;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_ALWAYS_ON: u64 = 4294967296;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_MASK: u64 = 12884901888;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_RESERVED: u64 = 12884901888;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_XTENDED_CONTROL_FLOW_GUARD_ALWAYS_OFF: u64 = 2199023255552;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_XTENDED_CONTROL_FLOW_GUARD_ALWAYS_ON: u64 = 1099511627776;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_XTENDED_CONTROL_FLOW_GUARD_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_XTENDED_CONTROL_FLOW_GUARD_MASK: u64 = 3298534883328;
pub const PROCESS_CREATION_MITIGATION_AUDIT_POLICY2_XTENDED_CONTROL_FLOW_GUARD_RESERVED: u64 = 3298534883328;
pub const PROCESS_CREATION_MITIGATION_POLICY2_ALLOW_DOWNGRADE_DYNAMIC_CODE_POLICY_ALWAYS_OFF: u64 = 2097152;
pub const PROCESS_CREATION_MITIGATION_POLICY2_ALLOW_DOWNGRADE_DYNAMIC_CODE_POLICY_ALWAYS_ON: u64 = 1048576;
pub const PROCESS_CREATION_MITIGATION_POLICY2_ALLOW_DOWNGRADE_DYNAMIC_CODE_POLICY_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_ALLOW_DOWNGRADE_DYNAMIC_CODE_POLICY_MASK: u64 = 3145728;
pub const PROCESS_CREATION_MITIGATION_POLICY2_ALLOW_DOWNGRADE_DYNAMIC_CODE_POLICY_RESERVED: u64 = 3145728;
pub const PROCESS_CREATION_MITIGATION_POLICY2_BLOCK_NON_CET_BINARIES_ALWAYS_OFF: u64 = 137438953472;
pub const PROCESS_CREATION_MITIGATION_POLICY2_BLOCK_NON_CET_BINARIES_ALWAYS_ON: u64 = 68719476736;
pub const PROCESS_CREATION_MITIGATION_POLICY2_BLOCK_NON_CET_BINARIES_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_BLOCK_NON_CET_BINARIES_MASK: u64 = 206158430208;
pub const PROCESS_CREATION_MITIGATION_POLICY2_BLOCK_NON_CET_BINARIES_NON_EHCONT: u64 = 206158430208;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_DYNAMIC_APIS_OUT_OF_PROC_ONLY_ALWAYS_OFF: u64 = 562949953421312;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_DYNAMIC_APIS_OUT_OF_PROC_ONLY_ALWAYS_ON: u64 = 281474976710656;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_DYNAMIC_APIS_OUT_OF_PROC_ONLY_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_DYNAMIC_APIS_OUT_OF_PROC_ONLY_MASK: u64 = 844424930131968;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_DYNAMIC_APIS_OUT_OF_PROC_ONLY_RESERVED: u64 = 844424930131968;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_USER_SHADOW_STACKS_ALWAYS_OFF: u64 = 536870912;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_USER_SHADOW_STACKS_ALWAYS_ON: u64 = 268435456;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_USER_SHADOW_STACKS_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_USER_SHADOW_STACKS_MASK: u64 = 805306368;
pub const PROCESS_CREATION_MITIGATION_POLICY2_CET_USER_SHADOW_STACKS_STRICT_MODE: u64 = 805306368;
pub const PROCESS_CREATION_MITIGATION_POLICY2_FSCTL_SYSTEM_CALL_DISABLE_ALWAYS_OFF: u64 = 144115188075855872;
pub const PROCESS_CREATION_MITIGATION_POLICY2_FSCTL_SYSTEM_CALL_DISABLE_ALWAYS_ON: u64 = 72057594037927936;
pub const PROCESS_CREATION_MITIGATION_POLICY2_FSCTL_SYSTEM_CALL_DISABLE_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_FSCTL_SYSTEM_CALL_DISABLE_MASK: u64 = 216172782113783808;
pub const PROCESS_CREATION_MITIGATION_POLICY2_FSCTL_SYSTEM_CALL_DISABLE_RESERVED: u64 = 216172782113783808;
pub const PROCESS_CREATION_MITIGATION_POLICY2_LOADER_INTEGRITY_CONTINUITY_ALWAYS_OFF: u64 = 32;
pub const PROCESS_CREATION_MITIGATION_POLICY2_LOADER_INTEGRITY_CONTINUITY_ALWAYS_ON: u64 = 16;
pub const PROCESS_CREATION_MITIGATION_POLICY2_LOADER_INTEGRITY_CONTINUITY_AUDIT: u64 = 48;
pub const PROCESS_CREATION_MITIGATION_POLICY2_LOADER_INTEGRITY_CONTINUITY_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_LOADER_INTEGRITY_CONTINUITY_MASK: u64 = 48;
pub const PROCESS_CREATION_MITIGATION_POLICY2_MODULE_TAMPERING_PROTECTION_ALWAYS_OFF: u64 = 8192;
pub const PROCESS_CREATION_MITIGATION_POLICY2_MODULE_TAMPERING_PROTECTION_ALWAYS_ON: u64 = 4096;
pub const PROCESS_CREATION_MITIGATION_POLICY2_MODULE_TAMPERING_PROTECTION_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_MODULE_TAMPERING_PROTECTION_MASK: u64 = 12288;
pub const PROCESS_CREATION_MITIGATION_POLICY2_MODULE_TAMPERING_PROTECTION_NOINHERIT: u64 = 12288;
pub const PROCESS_CREATION_MITIGATION_POLICY2_POINTER_AUTH_USER_IP_ALWAYS_OFF: u64 = 35184372088832;
pub const PROCESS_CREATION_MITIGATION_POLICY2_POINTER_AUTH_USER_IP_ALWAYS_ON: u64 = 17592186044416;
pub const PROCESS_CREATION_MITIGATION_POLICY2_POINTER_AUTH_USER_IP_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_POINTER_AUTH_USER_IP_MASK: u64 = 52776558133248;
pub const PROCESS_CREATION_MITIGATION_POLICY2_POINTER_AUTH_USER_IP_RESERVED: u64 = 52776558133248;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_CORE_SHARING_ALWAYS_OFF: u64 = 9007199254740992;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_CORE_SHARING_ALWAYS_ON: u64 = 4503599627370496;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_CORE_SHARING_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_CORE_SHARING_MASK: u64 = 13510798882111488;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_CORE_SHARING_RESERVED: u64 = 13510798882111488;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_INDIRECT_BRANCH_PREDICTION_ALWAYS_OFF: u64 = 131072;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_INDIRECT_BRANCH_PREDICTION_ALWAYS_ON: u64 = 65536;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_INDIRECT_BRANCH_PREDICTION_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_INDIRECT_BRANCH_PREDICTION_MASK: u64 = 196608;
pub const PROCESS_CREATION_MITIGATION_POLICY2_RESTRICT_INDIRECT_BRANCH_PREDICTION_RESERVED: u64 = 196608;
pub const PROCESS_CREATION_MITIGATION_POLICY2_SPECULATIVE_STORE_BYPASS_DISABLE_ALWAYS_OFF: u64 = 33554432;
pub const PROCESS_CREATION_MITIGATION_POLICY2_SPECULATIVE_STORE_BYPASS_DISABLE_ALWAYS_ON: u64 = 16777216;
pub const PROCESS_CREATION_MITIGATION_POLICY2_SPECULATIVE_STORE_BYPASS_DISABLE_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_SPECULATIVE_STORE_BYPASS_DISABLE_MASK: u64 = 50331648;
pub const PROCESS_CREATION_MITIGATION_POLICY2_SPECULATIVE_STORE_BYPASS_DISABLE_RESERVED: u64 = 50331648;
pub const PROCESS_CREATION_MITIGATION_POLICY2_STRICT_CONTROL_FLOW_GUARD_ALWAYS_OFF: u64 = 512;
pub const PROCESS_CREATION_MITIGATION_POLICY2_STRICT_CONTROL_FLOW_GUARD_ALWAYS_ON: u64 = 256;
pub const PROCESS_CREATION_MITIGATION_POLICY2_STRICT_CONTROL_FLOW_GUARD_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_STRICT_CONTROL_FLOW_GUARD_MASK: u64 = 768;
pub const PROCESS_CREATION_MITIGATION_POLICY2_STRICT_CONTROL_FLOW_GUARD_RESERVED: u64 = 768;
pub const PROCESS_CREATION_MITIGATION_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_ALWAYS_OFF: u64 = 8589934592;
pub const PROCESS_CREATION_MITIGATION_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_ALWAYS_ON: u64 = 4294967296;
pub const PROCESS_CREATION_MITIGATION_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_MASK: u64 = 12884901888;
pub const PROCESS_CREATION_MITIGATION_POLICY2_USER_CET_SET_CONTEXT_IP_VALIDATION_RELAXED_MODE: u64 = 12884901888;
pub const PROCESS_CREATION_MITIGATION_POLICY2_XTENDED_CONTROL_FLOW_GUARD_ALWAYS_OFF: u64 = 2199023255552;
pub const PROCESS_CREATION_MITIGATION_POLICY2_XTENDED_CONTROL_FLOW_GUARD_ALWAYS_ON: u64 = 1099511627776;
pub const PROCESS_CREATION_MITIGATION_POLICY2_XTENDED_CONTROL_FLOW_GUARD_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY2_XTENDED_CONTROL_FLOW_GUARD_MASK: u64 = 3298534883328;
pub const PROCESS_CREATION_MITIGATION_POLICY2_XTENDED_CONTROL_FLOW_GUARD_RESERVED: u64 = 3298534883328;
pub const PROCESS_CREATION_MITIGATION_POLICY_AUDIT_NONSYSTEM_FONTS: u64 = 844424930131968;
pub const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALLOW_STORE: u64 = 52776558133248;
pub const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALWAYS_OFF: u64 = 35184372088832;
pub const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_ALWAYS_ON: u64 = 17592186044416;
pub const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_BLOCK_NON_MICROSOFT_BINARIES_MASK: u64 = 52776558133248;
pub const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_ALWAYS_OFF: i32 = 131072;
pub const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_ALWAYS_ON: i32 = 65536;
pub const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_DEFER: i32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_MASK: i32 = 196608;
pub const PROCESS_CREATION_MITIGATION_POLICY_BOTTOM_UP_ASLR_RESERVED: i32 = 196608;
pub const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_ALWAYS_OFF: u64 = 2199023255552;
pub const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_ALWAYS_ON: u64 = 1099511627776;
pub const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_EXPORT_SUPPRESSION: u64 = 3298534883328;
pub const PROCESS_CREATION_MITIGATION_POLICY_CONTROL_FLOW_GUARD_MASK: u64 = 3298534883328;
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ATL_THUNK_ENABLE: i32 = 2;
pub const PROCESS_CREATION_MITIGATION_POLICY_DEP_ENABLE: i32 = 1;
pub const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_ALWAYS_OFF: u64 = 8589934592;
pub const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_ALWAYS_ON: u64 = 4294967296;
pub const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_MASK: u64 = 12884901888;
pub const PROCESS_CREATION_MITIGATION_POLICY_EXTENSION_POINT_DISABLE_RESERVED: u64 = 12884901888;
pub const PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_ALWAYS_OFF: u64 = 562949953421312;
pub const PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_ALWAYS_ON: u64 = 281474976710656;
pub const PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_FONT_DISABLE_MASK: u64 = 844424930131968;
pub const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_OFF: i32 = 512;
pub const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_ON: i32 = 256;
pub const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_ALWAYS_ON_REQ_RELOCS: i32 = 768;
pub const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_DEFER: i32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_FORCE_RELOCATE_IMAGES_MASK: i32 = 768;
pub const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_ALWAYS_OFF: i32 = 8192;
pub const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_ALWAYS_ON: i32 = 4096;
pub const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_DEFER: i32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_MASK: i32 = 12288;
pub const PROCESS_CREATION_MITIGATION_POLICY_HEAP_TERMINATE_RESERVED: i32 = 12288;
pub const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_ALWAYS_OFF: i32 = 2097152;
pub const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_ALWAYS_ON: i32 = 1048576;
pub const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_DEFER: i32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_MASK: i32 = 3145728;
pub const PROCESS_CREATION_MITIGATION_POLICY_HIGH_ENTROPY_ASLR_RESERVED: i32 = 3145728;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_ALWAYS_OFF: u64 = 144115188075855872;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_ALWAYS_ON: u64 = 72057594037927936;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_MASK: u64 = 216172782113783808;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_LOW_LABEL_RESERVED: u64 = 216172782113783808;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_ALWAYS_OFF: u64 = 9007199254740992;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_ALWAYS_ON: u64 = 4503599627370496;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_MASK: u64 = 13510798882111488;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_NO_REMOTE_RESERVED: u64 = 13510798882111488;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_ALWAYS_OFF: u64 = 2305843009213693952;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_ALWAYS_ON: u64 = 1152921504606846976;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_MASK: u64 = 3458764513820540928;
pub const PROCESS_CREATION_MITIGATION_POLICY_IMAGE_LOAD_PREFER_SYSTEM32_RESERVED: u64 = 3458764513820540928;
pub const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_OFF: u64 = 137438953472;
pub const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_ON: u64 = 68719476736;
pub const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_ALWAYS_ON_ALLOW_OPT_OUT: u64 = 206158430208;
pub const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_DEFER: u64 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_PROHIBIT_DYNAMIC_CODE_MASK: u64 = 206158430208;
pub const PROCESS_CREATION_MITIGATION_POLICY_SEHOP_ENABLE: i32 = 4;
pub const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_ALWAYS_OFF: i32 = 33554432;
pub const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_ALWAYS_ON: i32 = 16777216;
pub const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_DEFER: i32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_MASK: i32 = 50331648;
pub const PROCESS_CREATION_MITIGATION_POLICY_STRICT_HANDLE_CHECKS_RESERVED: i32 = 50331648;
pub const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_ALWAYS_OFF: i32 = 536870912;
pub const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_ALWAYS_ON: i32 = 268435456;
pub const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_DEFER: i32 = 0;
pub const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_MASK: i32 = 805306368;
pub const PROCESS_CREATION_MITIGATION_POLICY_WIN32K_SYSTEM_CALL_DISABLE_RESERVED: i32 = 805306368;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct PROCESS_CREATION_SME_VECTOR_LENGTH {
    pub Anonymous: PROCESS_CREATION_SME_VECTOR_LENGTH_0,
    pub Reserved: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for PROCESS_CREATION_SME_VECTOR_LENGTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PROCESS_CREATION_SME_VECTOR_LENGTH_0 {
    pub Data: super::ULONG,
    pub Anonymous: PROCESS_CREATION_SME_VECTOR_LENGTH_0_0,
}
#[cfg(feature = "shared")]
impl Default for PROCESS_CREATION_SME_VECTOR_LENGTH_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_CREATION_SME_VECTOR_LENGTH_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl PROCESS_CREATION_SME_VECTOR_LENGTH_0_0 {
    pub fn VectorLength(&self) -> super::ULONG {
        (self._bitfield) & 16777215
    }
    pub fn set_VectorLength(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !16777215) | (value & 16777215);
    }
    pub fn FlagsReserved(&self) -> super::ULONG {
        self._bitfield >> 24
    }
    pub fn set_FlagsReserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union PROCESS_CREATION_SVE_VECTOR_LENGTH {
    pub Data: super::ULONG,
    pub Anonymous: PROCESS_CREATION_SVE_VECTOR_LENGTH_0,
}
#[cfg(feature = "shared")]
impl Default for PROCESS_CREATION_SVE_VECTOR_LENGTH {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PROCESS_CREATION_SVE_VECTOR_LENGTH_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl PROCESS_CREATION_SVE_VECTOR_LENGTH_0 {
    pub fn VectorLength(&self) -> super::ULONG {
        (self._bitfield) & 16777215
    }
    pub fn set_VectorLength(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !16777215) | (value & 16777215);
    }
    pub fn FlagsReserved(&self) -> super::ULONG {
        self._bitfield >> 24
    }
    pub fn set_FlagsReserved(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(255 << 24)) | ((value & 255) << 24);
    }
}
pub const PROCESS_DEP_DISABLE_ATL_THUNK_EMULATION: i32 = 2;
pub const PROCESS_DEP_ENABLE: i32 = 1;
pub const PROCESS_MODE_BACKGROUND_BEGIN: i32 = 1048576;
pub const PROCESS_MODE_BACKGROUND_END: i32 = 2097152;
pub const PROCESS_NAME_NATIVE: i32 = 1;
pub const PROC_THREAD_ATTRIBUTE_ADDITIVE: i32 = 262144;
pub const PROC_THREAD_ATTRIBUTE_ALL_APPLICATION_PACKAGES_POLICY: i32 = 131087;
pub const PROC_THREAD_ATTRIBUTE_CHILD_PROCESS_POLICY: i32 = 131086;
pub const PROC_THREAD_ATTRIBUTE_COMPONENT_FILTER: i32 = 131098;
pub const PROC_THREAD_ATTRIBUTE_DESKTOP_APP_POLICY: i32 = 131090;
pub const PROC_THREAD_ATTRIBUTE_ENABLE_OPTIONAL_XSTATE_FEATURES: i32 = 196635;
pub const PROC_THREAD_ATTRIBUTE_GROUP_AFFINITY: i32 = 196611;
pub const PROC_THREAD_ATTRIBUTE_HANDLE_LIST: i32 = 131074;
pub const PROC_THREAD_ATTRIBUTE_IDEAL_PROCESSOR: i32 = 196613;
pub const PROC_THREAD_ATTRIBUTE_INPUT: i32 = 131072;
pub const PROC_THREAD_ATTRIBUTE_JOB_LIST: i32 = 131085;
pub const PROC_THREAD_ATTRIBUTE_MACHINE_TYPE: i32 = 131097;
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_AUDIT_POLICY: i32 = 131096;
pub const PROC_THREAD_ATTRIBUTE_MITIGATION_POLICY: i32 = 131079;
pub type PROC_THREAD_ATTRIBUTE_NUM = i32;
pub const PROC_THREAD_ATTRIBUTE_NUMBER: i32 = 65535;
pub const PROC_THREAD_ATTRIBUTE_PARENT_PROCESS: i32 = 131072;
pub const PROC_THREAD_ATTRIBUTE_PREFERRED_NODE: i32 = 131076;
pub const PROC_THREAD_ATTRIBUTE_PROTECTION_LEVEL: i32 = 131083;
pub const PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE: i32 = 131094;
pub const PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES: i32 = 131081;
pub const PROC_THREAD_ATTRIBUTE_SME_VECTOR_LENGTH: i32 = 131103;
pub const PROC_THREAD_ATTRIBUTE_SVE_VECTOR_LENGTH: i32 = 131102;
pub const PROC_THREAD_ATTRIBUTE_THREAD: i32 = 65536;
pub const PROC_THREAD_ATTRIBUTE_TRUSTED_APP: i32 = 131101;
pub const PROC_THREAD_ATTRIBUTE_UMS_THREAD: i32 = 196614;
pub const PROC_THREAD_ATTRIBUTE_WIN32K_FILTER: i32 = 131088;
pub const PROFILE_KERNEL: i32 = 536870912;
pub const PROFILE_SERVER: i32 = 1073741824;
pub const PROFILE_USER: i32 = 268435456;
pub const PROGRESS_CANCEL: i32 = 1;
pub const PROGRESS_CONTINUE: i32 = 0;
pub const PROGRESS_QUIET: i32 = 3;
pub const PROGRESS_STOP: i32 = 2;
pub const PROTECTION_LEVEL_ANTIMALWARE_LIGHT: i32 = 3;
pub const PROTECTION_LEVEL_AUTHENTICODE: i32 = 7;
pub const PROTECTION_LEVEL_CODEGEN_LIGHT: i32 = 6;
pub const PROTECTION_LEVEL_LSA_LIGHT: i32 = 4;
pub const PROTECTION_LEVEL_NONE: u32 = 4294967294;
pub const PROTECTION_LEVEL_PPL_APP: i32 = 8;
pub const PROTECTION_LEVEL_SAME: u32 = 4294967295;
pub const PROTECTION_LEVEL_WINDOWS: i32 = 1;
pub const PROTECTION_LEVEL_WINDOWS_LIGHT: i32 = 2;
pub const PROTECTION_LEVEL_WINTCB: i32 = 5;
pub const PROTECTION_LEVEL_WINTCB_LIGHT: i32 = 0;
#[cfg(feature = "shared")]
pub const PST_FAX: super::DWORD = 33;
#[cfg(feature = "shared")]
pub const PST_LAT: super::DWORD = 257;
#[cfg(feature = "shared")]
pub const PST_MODEM: super::DWORD = 6;
#[cfg(feature = "shared")]
pub const PST_NETWORK_BRIDGE: super::DWORD = 256;
#[cfg(feature = "shared")]
pub const PST_PARALLELPORT: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const PST_RS232: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const PST_RS422: super::DWORD = 3;
#[cfg(feature = "shared")]
pub const PST_RS423: super::DWORD = 4;
#[cfg(feature = "shared")]
pub const PST_RS449: super::DWORD = 5;
#[cfg(feature = "shared")]
pub const PST_SCANNER: super::DWORD = 34;
#[cfg(feature = "shared")]
pub const PST_TCPIP_TELNET: super::DWORD = 258;
#[cfg(feature = "shared")]
pub const PST_UNSPECIFIED: super::DWORD = 0;
#[cfg(feature = "shared")]
pub const PST_X25: super::DWORD = 259;
pub type PUMS_COMPLETION_LIST = *mut core::ffi::c_void;
pub type PUMS_CONTEXT = *mut core::ffi::c_void;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUMS_SCHEDULER_ENTRY_POINT = Option<unsafe extern "system" fn(reason: super::RTL_UMS_SCHEDULER_REASON, activationpayload: super::ULONG_PTR, schedulerparam: super::PVOID)>;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PUMS_SCHEDULER_STARTUP_INFO = *mut UMS_SCHEDULER_STARTUP_INFO;
#[cfg(feature = "shared")]
pub type PUMS_SYSTEM_THREAD_INFORMATION = *mut UMS_SYSTEM_THREAD_INFORMATION;
#[cfg(feature = "winnt")]
pub type PUMS_THREAD_INFO_CLASS = *mut super::RTL_UMS_THREAD_INFO_CLASS;
pub const PURGE_RXABORT: i32 = 2;
pub const PURGE_RXCLEAR: i32 = 8;
pub const PURGE_TXABORT: i32 = 1;
pub const PURGE_TXCLEAR: i32 = 4;
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
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_ADDRESS: i32 = 16;
pub const QUERY_ACTCTX_FLAG_ACTCTX_IS_HMODULE: i32 = 8;
pub const QUERY_ACTCTX_FLAG_NO_ADDREF: u32 = 2147483648;
pub const QUERY_ACTCTX_FLAG_USE_ACTIVE_ACTCTX: i32 = 4;
pub const REALTIME_PRIORITY_CLASS: i32 = 256;
pub const RECOVERY_DEFAULT_PING_INTERVAL: i32 = 5000;
pub const RECOVERY_MAX_PING_INTERVAL: i32 = 300000;
pub const REMOTE_PROTOCOL_INFO_FLAG_LOOPBACK: i32 = 1;
pub const REMOTE_PROTOCOL_INFO_FLAG_OFFLINE: i32 = 2;
pub const REMOTE_PROTOCOL_INFO_FLAG_PERSISTENT_HANDLE: i32 = 4;
pub const REPLACEFILE_IGNORE_ACL_ERRORS: i32 = 4;
pub const REPLACEFILE_IGNORE_MERGE_ERRORS: i32 = 2;
pub const REPLACEFILE_WRITE_THROUGH: i32 = 1;
pub const RESETDEV: i32 = 7;
pub const RESTART_MAX_CMD_LINE: i32 = 1024;
pub const RESTART_NO_CRASH: i32 = 1;
pub const RESTART_NO_HANG: i32 = 2;
pub const RESTART_NO_PATCH: i32 = 4;
pub const RESTART_NO_REBOOT: i32 = 8;
pub const RPI_FLAG_SMB2_SHARECAP_CLUSTER: i32 = 64;
pub const RPI_FLAG_SMB2_SHARECAP_CONTINUOUS_AVAILABILITY: i32 = 16;
pub const RPI_FLAG_SMB2_SHARECAP_DFS: i32 = 8;
pub const RPI_FLAG_SMB2_SHARECAP_SCALEOUT: i32 = 32;
pub const RPI_FLAG_SMB2_SHARECAP_TIMEWARP: i32 = 2;
pub const RPI_SMB2_FLAG_SERVERCAP_DFS: i32 = 1;
pub const RPI_SMB2_FLAG_SERVERCAP_DIRECTORY_LEASING: i32 = 32;
pub const RPI_SMB2_FLAG_SERVERCAP_LARGEMTU: i32 = 4;
pub const RPI_SMB2_FLAG_SERVERCAP_LEASING: i32 = 2;
pub const RPI_SMB2_FLAG_SERVERCAP_MULTICHANNEL: i32 = 8;
pub const RPI_SMB2_FLAG_SERVERCAP_PERSISTENT_HANDLES: i32 = 16;
pub const RPI_SMB2_SHAREFLAG_COMPRESS_DATA: i32 = 2;
pub const RPI_SMB2_SHAREFLAG_ENCRYPT_DATA: i32 = 1;
pub const RTS_CONTROL_DISABLE: i32 = 0;
pub const RTS_CONTROL_ENABLE: i32 = 1;
pub const RTS_CONTROL_HANDSHAKE: i32 = 2;
pub const RTS_CONTROL_TOGGLE: i32 = 3;
pub const SCS_32BIT_BINARY: i32 = 0;
pub const SCS_64BIT_BINARY: i32 = 6;
pub const SCS_DOS_BINARY: i32 = 1;
pub const SCS_OS216_BINARY: i32 = 5;
pub const SCS_PIF_BINARY: i32 = 3;
pub const SCS_POSIX_BINARY: i32 = 4;
#[cfg(target_arch = "x86")]
pub const SCS_THIS_PLATFORM_BINARY: i32 = 0;
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
pub const SCS_THIS_PLATFORM_BINARY: i32 = 6;
pub const SCS_WOW_BINARY: i32 = 2;
pub const SECURITY_ANONYMOUS: i32 = 0;
pub const SECURITY_CONTEXT_TRACKING: i32 = 262144;
pub const SECURITY_DELEGATION: i32 = 196608;
pub const SECURITY_EFFECTIVE_ONLY: i32 = 524288;
pub const SECURITY_IDENTIFICATION: i32 = 65536;
pub const SECURITY_IMPERSONATION: i32 = 131072;
pub const SECURITY_SQOS_PRESENT: i32 = 1048576;
pub const SECURITY_VALID_SQOS_FLAGS: i32 = 2031616;
pub const SEM_FAILCRITICALERRORS: i32 = 1;
pub const SEM_NOALIGNMENTFAULTEXCEPT: i32 = 4;
pub const SEM_NOGPFAULTERRORBOX: i32 = 2;
pub const SEM_NOOPENFILEERRORBOX: i32 = 32768;
pub const SETBREAK: i32 = 8;
pub const SETDTR: i32 = 5;
pub const SETRTS: i32 = 3;
pub const SETXOFF: i32 = 1;
pub const SETXON: i32 = 2;
pub const SET_TAPE_DRIVE_INFORMATION: i32 = 1;
pub const SET_TAPE_MEDIA_INFORMATION: i32 = 0;
pub const SHUTDOWN_NORETRY: i32 = 1;
pub const SPACEPARITY: i32 = 4;
#[cfg(feature = "shared")]
pub const SP_BAUD: super::DWORD = 2;
#[cfg(feature = "shared")]
pub const SP_DATABITS: super::DWORD = 4;
#[cfg(feature = "shared")]
pub const SP_HANDSHAKING: super::DWORD = 16;
#[cfg(feature = "shared")]
pub const SP_PARITY: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const SP_PARITY_CHECK: super::DWORD = 32;
#[cfg(feature = "shared")]
pub const SP_RLSD: super::DWORD = 64;
#[cfg(feature = "shared")]
pub const SP_SERIALCOMM: super::DWORD = 1;
#[cfg(feature = "shared")]
pub const SP_STOPBITS: super::DWORD = 8;
pub const STACK_SIZE_PARAM_IS_A_RESERVATION: i32 = 65536;
pub const STARTF_FORCEOFFFEEDBACK: i32 = 128;
pub const STARTF_FORCEONFEEDBACK: i32 = 64;
pub const STARTF_HOLOGRAPHIC: i32 = 262144;
pub const STARTF_PREVENTPINNING: i32 = 8192;
pub const STARTF_RUNFULLSCREEN: i32 = 32;
pub const STARTF_TITLEISAPPID: i32 = 4096;
pub const STARTF_TITLEISLINKNAME: i32 = 2048;
pub const STARTF_UNTRUSTEDSOURCE: i32 = 32768;
pub const STARTF_USECOUNTCHARS: i32 = 8;
pub const STARTF_USEFILLATTRIBUTE: i32 = 16;
pub const STARTF_USEHOTKEY: i32 = 512;
pub const STARTF_USEPOSITION: i32 = 4;
pub const STARTF_USESHOWWINDOW: i32 = 1;
pub const STARTF_USESIZE: i32 = 2;
pub const STARTF_USESTDHANDLES: i32 = 256;
#[cfg(all(feature = "processthreadsapi", feature = "shared", feature = "winnt"))]
pub type STARTUPINFOEX = STARTUPINFOEXA;
#[repr(C)]
#[cfg(all(feature = "processthreadsapi", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STARTUPINFOEXA {
    pub StartupInfo: super::STARTUPINFOA,
    pub lpAttributeList: super::LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[repr(C)]
#[cfg(all(feature = "processthreadsapi", feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct STARTUPINFOEXW {
    pub StartupInfo: super::STARTUPINFOW,
    pub lpAttributeList: super::LPPROC_THREAD_ATTRIBUTE_LIST,
}
#[cfg(feature = "shared")]
pub const STD_ERROR_HANDLE: super::DWORD = 4294967284;
#[cfg(feature = "shared")]
pub const STD_INPUT_HANDLE: super::DWORD = 4294967286;
#[cfg(feature = "shared")]
pub const STD_OUTPUT_HANDLE: super::DWORD = 4294967285;
#[cfg(feature = "shared")]
pub const STOPBITS_10: super::WORD = 1;
#[cfg(feature = "shared")]
pub const STOPBITS_15: super::WORD = 2;
#[cfg(feature = "shared")]
pub const STOPBITS_20: super::WORD = 4;
pub const STORAGE_INFO_FLAGS_ALIGNED_DEVICE: i32 = 1;
pub const STORAGE_INFO_FLAGS_PARTITION_ALIGNED_ON_DEVICE: i32 = 2;
pub const STORAGE_INFO_OFFSET_UNKNOWN: u32 = 4294967295;
pub const STREAM_CONTAINS_GHOSTED_FILE_EXTENTS: i32 = 16;
pub const STREAM_CONTAINS_PROPERTIES: i32 = 4;
pub const STREAM_CONTAINS_SECURITY: i32 = 2;
pub const STREAM_MODIFIED_WHEN_READ: i32 = 1;
pub const STREAM_NORMAL_ATTRIBUTE: i32 = 0;
pub const STREAM_SPARSE_ATTRIBUTE: i32 = 8;
pub const SYMBOLIC_LINK_FLAG_ALLOW_UNPRIVILEGED_CREATE: i32 = 2;
pub const SYMBOLIC_LINK_FLAG_DIRECTORY: i32 = 1;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SYSTEM_POWER_STATUS {
    pub ACLineStatus: super::BYTE,
    pub BatteryFlag: super::BYTE,
    pub BatteryLifePercent: super::BYTE,
    pub SystemStatusFlag: super::BYTE,
    pub BatteryLifeTime: super::DWORD,
    pub BatteryFullLifeTime: super::DWORD,
}
pub const SYSTEM_STATUS_FLAG_POWER_SAVING_ON: i32 = 1;
pub const S_ALLTHRESHOLD: i32 = 2;
pub const S_LEGATO: i32 = 1;
pub const S_NORMAL: i32 = 0;
pub const S_PERIOD1024: i32 = 1;
pub const S_PERIOD2048: i32 = 2;
pub const S_PERIOD512: i32 = 0;
pub const S_PERIODVOICE: i32 = 3;
pub const S_QUEUEEMPTY: i32 = 0;
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
pub const S_STACCATO: i32 = 2;
pub const S_THRESHOLD: i32 = 1;
pub const S_WHITE1024: i32 = 5;
pub const S_WHITE2048: i32 = 6;
pub const S_WHITE512: i32 = 4;
pub const S_WHITEVOICE: i32 = 7;
pub const TC_GP_TRAP: i32 = 2;
pub const TC_HARDERR: i32 = 1;
pub const TC_NORMAL: i32 = 0;
pub const TC_SIGNAL: i32 = 3;
pub const THREAD_MODE_BACKGROUND_BEGIN: i32 = 65536;
pub const THREAD_MODE_BACKGROUND_END: i32 = 131072;
pub const THREAD_PRIORITY_ABOVE_NORMAL: i32 = 1;
pub const THREAD_PRIORITY_BELOW_NORMAL: i32 = -1;
pub const THREAD_PRIORITY_ERROR_RETURN: i32 = 2147483647;
pub const THREAD_PRIORITY_HIGHEST: i32 = 2;
pub const THREAD_PRIORITY_IDLE: i32 = -15;
pub const THREAD_PRIORITY_LOWEST: i32 = -2;
pub const THREAD_PRIORITY_NORMAL: i32 = 0;
pub const THREAD_PRIORITY_TIME_CRITICAL: i32 = 15;
pub const TWOSTOPBITS: i32 = 2;
#[cfg(feature = "winnt")]
pub type UMS_SCHEDULER_REASON = super::RTL_UMS_SCHEDULER_REASON;
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default)]
pub struct UMS_SCHEDULER_STARTUP_INFO {
    pub UmsVersion: super::ULONG,
    pub CompletionList: PUMS_COMPLETION_LIST,
    pub SchedulerProc: PUMS_SCHEDULER_ENTRY_POINT,
    pub SchedulerParam: super::PVOID,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub struct UMS_SYSTEM_THREAD_INFORMATION {
    pub UmsVersion: super::ULONG,
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0,
}
#[cfg(feature = "shared")]
impl Default for UMS_SYSTEM_THREAD_INFORMATION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy)]
pub union UMS_SYSTEM_THREAD_INFORMATION_0 {
    pub Anonymous: UMS_SYSTEM_THREAD_INFORMATION_0_0,
    pub ThreadUmsFlags: super::ULONG,
}
#[cfg(feature = "shared")]
impl Default for UMS_SYSTEM_THREAD_INFORMATION_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    pub _bitfield: u32,
}
#[cfg(feature = "shared")]
impl UMS_SYSTEM_THREAD_INFORMATION_0_0 {
    pub fn IsUmsSchedulerThread(&self) -> super::ULONG {
        (self._bitfield) & 1
    }
    pub fn set_IsUmsSchedulerThread(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !1) | (value & 1);
    }
    pub fn IsUmsWorkerThread(&self) -> super::ULONG {
        (self._bitfield >> 1) & 1
    }
    pub fn set_IsUmsWorkerThread(&mut self, value: super::ULONG) {
        self._bitfield = (self._bitfield & !(1 << 1)) | ((value & 1) << 1);
    }
}
#[cfg(feature = "winnt")]
pub type UMS_THREAD_INFO_CLASS = super::RTL_UMS_THREAD_INFO_CLASS;
pub const UMS_VERSION: i32 = 256;
pub const VOLUME_NAME_DOS: i32 = 0;
pub const VOLUME_NAME_GUID: i32 = 1;
pub const VOLUME_NAME_NONE: i32 = 4;
pub const VOLUME_NAME_NT: i32 = 2;
pub const WAIT_ABANDONED: windows_core::NTSTATUS = windows_core::NTSTATUS(0x80_u32 as _);
pub const WAIT_ABANDONED_0: windows_core::NTSTATUS = windows_core::NTSTATUS(0x80_u32 as _);
#[cfg(feature = "shared")]
pub const WAIT_FAILED: super::DWORD = 4294967295;
pub const WAIT_IO_COMPLETION: windows_core::NTSTATUS = windows_core::NTSTATUS(0xC0_u32 as _);
pub const WAIT_OBJECT_0: windows_core::NTSTATUS = windows_core::NTSTATUS(0x0_u32 as _);
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WIN32_STREAM_ID {
    pub dwStreamId: super::DWORD,
    pub dwStreamAttributes: super::DWORD,
    pub Size: super::LARGE_INTEGER,
    pub dwStreamNameSize: super::DWORD,
    pub cStreamName: [super::WCHAR; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WIN32_STREAM_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
