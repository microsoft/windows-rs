#[inline]
pub unsafe fn MQADsPathToFormatName<P0>(lpwcsadspath: P0, lpwcsformatname: windows_core::PWSTR, lpdwformatnamelength: *mut u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQADsPathToFormatName(lpwcsadspath : windows_core::PCWSTR, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    unsafe { MQADsPathToFormatName(lpwcsadspath.param().abi(), lpwcsformatname, lpdwformatnamelength as _) }
}
#[cfg(feature = "transact")]
#[inline]
pub unsafe fn MQBeginTransaction() -> windows_core::Result<super::ITransaction> {
    windows_core::link!("mqrt.dll" "system" fn MQBeginTransaction(pptransaction : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MQBeginTransaction(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MQCloseCursor(hcursor: super::HANDLE) -> windows_core::HRESULT {
    windows_core::link!("mqrt.dll" "system" fn MQCloseCursor(hcursor : super::HANDLE) -> windows_core::HRESULT);
    unsafe { MQCloseCursor(hcursor) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MQCloseQueue(hqueue: QUEUEHANDLE) -> windows_core::HRESULT {
    windows_core::link!("mqrt.dll" "system" fn MQCloseQueue(hqueue : QUEUEHANDLE) -> windows_core::HRESULT);
    unsafe { MQCloseQueue(hqueue) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MQCreateCursor(hqueue: QUEUEHANDLE) -> windows_core::Result<super::HANDLE> {
    windows_core::link!("mqrt.dll" "system" fn MQCreateCursor(hqueue : QUEUEHANDLE, phcursor : *mut super::HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MQCreateCursor(hqueue, &mut result__).map(|| result__)
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MQCreateQueue(psecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>, pqueueprops: *mut MQQUEUEPROPS, lpwcsformatname: Option<windows_core::PWSTR>, lpdwformatnamelength: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("mqrt.dll" "system" fn MQCreateQueue(psecuritydescriptor : super::PSECURITY_DESCRIPTOR, pqueueprops : *mut MQQUEUEPROPS, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    unsafe { MQCreateQueue(psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _, pqueueprops as _, lpwcsformatname.unwrap_or(core::mem::zeroed()) as _, lpdwformatnamelength as _) }
}
#[inline]
pub unsafe fn MQDeleteQueue<P0>(lpwcsformatname: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQDeleteQueue(lpwcsformatname : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { MQDeleteQueue(lpwcsformatname.param().abi()) }
}
#[inline]
pub unsafe fn MQFreeMemory(pvmemory: *const core::ffi::c_void) {
    windows_core::link!("mqrt.dll" "system" fn MQFreeMemory(pvmemory : *const core::ffi::c_void));
    unsafe { MQFreeMemory(pvmemory) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MQFreeSecurityContext(hsecuritycontext: super::HANDLE) {
    windows_core::link!("mqrt.dll" "system" fn MQFreeSecurityContext(hsecuritycontext : super::HANDLE));
    unsafe { MQFreeSecurityContext(hsecuritycontext) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MQGetMachineProperties<P0>(lpwcsmachinename: P0, pguidmachineid: Option<*const windows_core::GUID>, pqmprops: *mut MQQMPROPS) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQGetMachineProperties(lpwcsmachinename : windows_core::PCWSTR, pguidmachineid : *const windows_core::GUID, pqmprops : *mut MQQMPROPS) -> windows_core::HRESULT);
    unsafe { MQGetMachineProperties(lpwcsmachinename.param().abi(), pguidmachineid.unwrap_or(core::mem::zeroed()) as _, pqmprops as _) }
}
#[cfg(all(feature = "minwinbase", feature = "winnt"))]
#[inline]
pub unsafe fn MQGetOverlappedResult(lpoverlapped: *const super::OVERLAPPED) -> windows_core::HRESULT {
    windows_core::link!("mqrt.dll" "system" fn MQGetOverlappedResult(lpoverlapped : *const super::OVERLAPPED) -> windows_core::HRESULT);
    unsafe { MQGetOverlappedResult(lpoverlapped) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MQGetPrivateComputerInformation<P0>(lpwcscomputername: P0, pprivateprops: *mut MQPRIVATEPROPS) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQGetPrivateComputerInformation(lpwcscomputername : windows_core::PCWSTR, pprivateprops : *mut MQPRIVATEPROPS) -> windows_core::HRESULT);
    unsafe { MQGetPrivateComputerInformation(lpwcscomputername.param().abi(), pprivateprops as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MQGetQueueProperties<P0>(lpwcsformatname: P0, pqueueprops: *mut MQQUEUEPROPS) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQGetQueueProperties(lpwcsformatname : windows_core::PCWSTR, pqueueprops : *mut MQQUEUEPROPS) -> windows_core::HRESULT);
    unsafe { MQGetQueueProperties(lpwcsformatname.param().abi(), pqueueprops as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MQGetQueueSecurity<P0>(lpwcsformatname: P0, requestedinformation: super::SECURITY_INFORMATION, psecuritydescriptor: super::PSECURITY_DESCRIPTOR, nlength: u32, lpnlengthneeded: *mut u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQGetQueueSecurity(lpwcsformatname : windows_core::PCWSTR, requestedinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_core::HRESULT);
    unsafe { MQGetQueueSecurity(lpwcsformatname.param().abi(), requestedinformation, psecuritydescriptor as _, nlength, lpnlengthneeded as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MQGetSecurityContext(lpcertbuffer: Option<*const core::ffi::c_void>, dwcertbufferlength: u32) -> windows_core::Result<super::HANDLE> {
    windows_core::link!("mqrt.dll" "system" fn MQGetSecurityContext(lpcertbuffer : *const core::ffi::c_void, dwcertbufferlength : u32, phsecuritycontext : *mut super::HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MQGetSecurityContext(lpcertbuffer.unwrap_or(core::mem::zeroed()) as _, dwcertbufferlength, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MQGetSecurityContextEx(lpcertbuffer: Option<*const core::ffi::c_void>, dwcertbufferlength: u32) -> windows_core::Result<super::HANDLE> {
    windows_core::link!("mqrt.dll" "system" fn MQGetSecurityContextEx(lpcertbuffer : *const core::ffi::c_void, dwcertbufferlength : u32, phsecuritycontext : *mut super::HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MQGetSecurityContextEx(lpcertbuffer.unwrap_or(core::mem::zeroed()) as _, dwcertbufferlength, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MQHandleToFormatName(hqueue: QUEUEHANDLE, lpwcsformatname: windows_core::PWSTR, lpdwformatnamelength: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("mqrt.dll" "system" fn MQHandleToFormatName(hqueue : QUEUEHANDLE, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    unsafe { MQHandleToFormatName(hqueue, lpwcsformatname, lpdwformatnamelength as _) }
}
#[inline]
pub unsafe fn MQInstanceToFormatName(pguid: *const windows_core::GUID, lpwcsformatname: windows_core::PWSTR, lpdwformatnamelength: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("mqrt.dll" "system" fn MQInstanceToFormatName(pguid : *const windows_core::GUID, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    unsafe { MQInstanceToFormatName(pguid, lpwcsformatname, lpdwformatnamelength as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MQLocateBegin<P0>(lpwcscontext: P0, prestriction: Option<*const MQRESTRICTION>, pcolumns: *const MQCOLUMNSET, psort: *const MQSORTSET) -> windows_core::Result<super::HANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQLocateBegin(lpwcscontext : windows_core::PCWSTR, prestriction : *const MQRESTRICTION, pcolumns : *const MQCOLUMNSET, psort : *const MQSORTSET, phenum : *mut super::HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MQLocateBegin(lpwcscontext.param().abi(), prestriction.unwrap_or(core::mem::zeroed()) as _, pcolumns, psort, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MQLocateEnd(henum: super::HANDLE) -> windows_core::HRESULT {
    windows_core::link!("mqrt.dll" "system" fn MQLocateEnd(henum : super::HANDLE) -> windows_core::HRESULT);
    unsafe { MQLocateEnd(henum) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MQLocateNext(henum: super::HANDLE, pcprops: *mut u32, apropvar: *mut MQPROPVARIANT) -> windows_core::HRESULT {
    windows_core::link!("mqrt.dll" "system" fn MQLocateNext(henum : super::HANDLE, pcprops : *mut u32, apropvar : *mut MQPROPVARIANT) -> windows_core::HRESULT);
    unsafe { MQLocateNext(henum, pcprops as _, apropvar) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MQMarkMessageRejected(hqueue: super::HANDLE, ulllookupid: u64) -> windows_core::HRESULT {
    windows_core::link!("mqrt.dll" "system" fn MQMarkMessageRejected(hqueue : super::HANDLE, ulllookupid : u64) -> windows_core::HRESULT);
    unsafe { MQMarkMessageRejected(hqueue, ulllookupid) }
}
#[inline]
pub unsafe fn MQMgmtAction<P0, P1, P2>(pcomputername: P0, pobjectname: P1, paction: P2) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQMgmtAction(pcomputername : windows_core::PCWSTR, pobjectname : windows_core::PCWSTR, paction : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { MQMgmtAction(pcomputername.param().abi(), pobjectname.param().abi(), paction.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MQMgmtGetInfo<P0, P1>(pcomputername: P0, pobjectname: P1, pmgmtprops: *mut MQMGMTPROPS) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQMgmtGetInfo(pcomputername : windows_core::PCWSTR, pobjectname : windows_core::PCWSTR, pmgmtprops : *mut MQMGMTPROPS) -> windows_core::HRESULT);
    unsafe { MQMgmtGetInfo(pcomputername.param().abi(), pobjectname.param().abi(), pmgmtprops as _) }
}
#[cfg(all(feature = "transact", feature = "winnt"))]
#[inline]
pub unsafe fn MQMoveMessage<P3>(hsourcequeue: QUEUEHANDLE, hdestinationqueue: QUEUEHANDLE, ulllookupid: u64, ptransaction: P3) -> windows_core::HRESULT
where
    P3: windows_core::Param<super::ITransaction>,
{
    windows_core::link!("mqrt.dll" "system" fn MQMoveMessage(hsourcequeue : QUEUEHANDLE, hdestinationqueue : QUEUEHANDLE, ulllookupid : u64, ptransaction : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MQMoveMessage(hsourcequeue, hdestinationqueue, ulllookupid, ptransaction.param().abi()) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MQOpenQueue<P0>(lpwcsformatname: P0, dwaccess: u32, dwsharemode: u32) -> windows_core::Result<QUEUEHANDLE>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQOpenQueue(lpwcsformatname : windows_core::PCWSTR, dwaccess : u32, dwsharemode : u32, phqueue : *mut QUEUEHANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        MQOpenQueue(lpwcsformatname.param().abi(), dwaccess, dwsharemode, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn MQPathNameToFormatName<P0>(lpwcspathname: P0, lpwcsformatname: windows_core::PWSTR, lpdwformatnamelength: *mut u32) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQPathNameToFormatName(lpwcspathname : windows_core::PCWSTR, lpwcsformatname : windows_core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_core::HRESULT);
    unsafe { MQPathNameToFormatName(lpwcspathname.param().abi(), lpwcsformatname, lpdwformatnamelength as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MQPurgeQueue(hqueue: QUEUEHANDLE) -> windows_core::HRESULT {
    windows_core::link!("mqrt.dll" "system" fn MQPurgeQueue(hqueue : QUEUEHANDLE) -> windows_core::HRESULT);
    unsafe { MQPurgeQueue(hqueue) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "transact", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MQReceiveMessage<P7>(hsource: QUEUEHANDLE, dwtimeout: u32, dwaction: u32, pmessageprops: Option<*mut MQMSGPROPS>, lpoverlapped: Option<*mut super::OVERLAPPED>, fnreceivecallback: PMQRECEIVECALLBACK, hcursor: Option<super::HANDLE>, ptransaction: P7) -> windows_core::HRESULT
where
    P7: windows_core::Param<super::ITransaction>,
{
    windows_core::link!("mqrt.dll" "system" fn MQReceiveMessage(hsource : QUEUEHANDLE, dwtimeout : u32, dwaction : u32, pmessageprops : *mut MQMSGPROPS, lpoverlapped : *mut super::OVERLAPPED, fnreceivecallback : PMQRECEIVECALLBACK, hcursor : super::HANDLE, ptransaction : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MQReceiveMessage(hsource, dwtimeout, dwaction, pmessageprops.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, fnreceivecallback, hcursor.unwrap_or(core::mem::zeroed()) as _, ptransaction.param().abi()) }
}
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "transact", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MQReceiveMessageByLookupId<P6>(hsource: QUEUEHANDLE, ulllookupid: u64, dwlookupaction: u32, pmessageprops: Option<*mut MQMSGPROPS>, lpoverlapped: Option<*mut super::OVERLAPPED>, fnreceivecallback: PMQRECEIVECALLBACK, ptransaction: P6) -> windows_core::HRESULT
where
    P6: windows_core::Param<super::ITransaction>,
{
    windows_core::link!("mqrt.dll" "system" fn MQReceiveMessageByLookupId(hsource : QUEUEHANDLE, ulllookupid : u64, dwlookupaction : u32, pmessageprops : *mut MQMSGPROPS, lpoverlapped : *mut super::OVERLAPPED, fnreceivecallback : PMQRECEIVECALLBACK, ptransaction : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MQReceiveMessageByLookupId(hsource, ulllookupid, dwlookupaction, pmessageprops.unwrap_or(core::mem::zeroed()) as _, lpoverlapped.unwrap_or(core::mem::zeroed()) as _, fnreceivecallback, ptransaction.param().abi()) }
}
#[inline]
pub unsafe fn MQRegisterCertificate(dwflags: u32, lpcertbuffer: *const core::ffi::c_void, dwcertbufferlength: u32) -> windows_core::HRESULT {
    windows_core::link!("mqrt.dll" "system" fn MQRegisterCertificate(dwflags : u32, lpcertbuffer : *const core::ffi::c_void, dwcertbufferlength : u32) -> windows_core::HRESULT);
    unsafe { MQRegisterCertificate(dwflags, lpcertbuffer, dwcertbufferlength) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "transact", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MQSendMessage<P2>(hdestinationqueue: QUEUEHANDLE, pmessageprops: *const MQMSGPROPS, ptransaction: P2) -> windows_core::HRESULT
where
    P2: windows_core::Param<super::ITransaction>,
{
    windows_core::link!("mqrt.dll" "system" fn MQSendMessage(hdestinationqueue : QUEUEHANDLE, pmessageprops : *const MQMSGPROPS, ptransaction : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MQSendMessage(hdestinationqueue, pmessageprops, ptransaction.param().abi()) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn MQSetQueueProperties<P0>(lpwcsformatname: P0, pqueueprops: *mut MQQUEUEPROPS) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQSetQueueProperties(lpwcsformatname : windows_core::PCWSTR, pqueueprops : *mut MQQUEUEPROPS) -> windows_core::HRESULT);
    unsafe { MQSetQueueProperties(lpwcsformatname.param().abi(), pqueueprops as _) }
}
#[cfg(feature = "winnt")]
#[inline]
pub unsafe fn MQSetQueueSecurity<P0>(lpwcsformatname: P0, securityinformation: super::SECURITY_INFORMATION, psecuritydescriptor: Option<super::PSECURITY_DESCRIPTOR>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("mqrt.dll" "system" fn MQSetQueueSecurity(lpwcsformatname : windows_core::PCWSTR, securityinformation : super::SECURITY_INFORMATION, psecuritydescriptor : super::PSECURITY_DESCRIPTOR) -> windows_core::HRESULT);
    unsafe { MQSetQueueSecurity(lpwcsformatname.param().abi(), securityinformation, psecuritydescriptor.unwrap_or(core::mem::zeroed()) as _) }
}
pub const LONG_LIVED: u32 = 4294967294;
pub const MACHINE_ACTION_CONNECT: windows_core::PCWSTR = windows_core::w!("CONNECT");
pub const MACHINE_ACTION_DISCONNECT: windows_core::PCWSTR = windows_core::w!("DISCONNECT");
pub const MACHINE_ACTION_TIDY: windows_core::PCWSTR = windows_core::w!("TIDY");
#[cfg(feature = "wtypes")]
pub type MGMTPROPID = super::PROPID;
pub const MGMT_QUEUE_CORRECT_TYPE: windows_core::PCWSTR = windows_core::w!("YES");
pub const MGMT_QUEUE_FOREIGN_TYPE: windows_core::PCWSTR = windows_core::w!("YES");
pub const MGMT_QUEUE_INCORRECT_TYPE: windows_core::PCWSTR = windows_core::w!("NO");
pub const MGMT_QUEUE_LOCAL_LOCATION: windows_core::PCWSTR = windows_core::w!("LOCAL");
pub const MGMT_QUEUE_NOT_FOREIGN_TYPE: windows_core::PCWSTR = windows_core::w!("NO");
pub const MGMT_QUEUE_NOT_TRANSACTIONAL_TYPE: windows_core::PCWSTR = windows_core::w!("NO");
pub const MGMT_QUEUE_REMOTE_LOCATION: windows_core::PCWSTR = windows_core::w!("REMOTE");
pub const MGMT_QUEUE_STATE_CONNECTED: windows_core::PCWSTR = windows_core::w!("CONNECTED");
pub const MGMT_QUEUE_STATE_DISCONNECTED: windows_core::PCWSTR = windows_core::w!("DISCONNECTED");
pub const MGMT_QUEUE_STATE_DISCONNECTING: windows_core::PCWSTR = windows_core::w!("DISCONNECTING");
pub const MGMT_QUEUE_STATE_LOCAL: windows_core::PCWSTR = windows_core::w!("LOCAL CONNECTION");
pub const MGMT_QUEUE_STATE_LOCKED: windows_core::PCWSTR = windows_core::w!("LOCKED");
pub const MGMT_QUEUE_STATE_NEED_VALIDATE: windows_core::PCWSTR = windows_core::w!("NEED VALIDATION");
pub const MGMT_QUEUE_STATE_NONACTIVE: windows_core::PCWSTR = windows_core::w!("INACTIVE");
pub const MGMT_QUEUE_STATE_ONHOLD: windows_core::PCWSTR = windows_core::w!("ONHOLD");
pub const MGMT_QUEUE_STATE_WAITING: windows_core::PCWSTR = windows_core::w!("WAITING");
pub const MGMT_QUEUE_TRANSACTIONAL_TYPE: windows_core::PCWSTR = windows_core::w!("YES");
pub const MGMT_QUEUE_TYPE_CONNECTOR: windows_core::PCWSTR = windows_core::w!("CONNECTOR");
pub const MGMT_QUEUE_TYPE_MACHINE: windows_core::PCWSTR = windows_core::w!("MACHINE");
pub const MGMT_QUEUE_TYPE_MULTICAST: windows_core::PCWSTR = windows_core::w!("MULTICAST");
pub const MGMT_QUEUE_TYPE_PRIVATE: windows_core::PCWSTR = windows_core::w!("PRIVATE");
pub const MGMT_QUEUE_TYPE_PUBLIC: windows_core::PCWSTR = windows_core::w!("PUBLIC");
pub const MGMT_QUEUE_UNKNOWN_TYPE: windows_core::PCWSTR = windows_core::w!("UNKNOWN");
pub const MO_MACHINE_TOKEN: windows_core::PCWSTR = windows_core::w!("MACHINE");
pub const MO_QUEUE_TOKEN: windows_core::PCWSTR = windows_core::w!("QUEUE");
pub const MQCERT_REGISTER_ALWAYS: i32 = 1;
pub const MQCERT_REGISTER_IF_NOT_EXIST: i32 = 2;
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQCOLUMNSET {
    pub cCol: u32,
    pub aCol: *mut super::PROPID,
}
pub const MQCONN_BIND_SOCKET_FAILURE: MQConnectionState = -2147483645;
pub const MQCONN_CONNECT_SOCKET_FAILURE: MQConnectionState = -2147483644;
pub const MQCONN_CREATE_SOCKET_FAILURE: MQConnectionState = -2147483646;
pub const MQCONN_ESTABLISH_PACKET_RECEIVED: MQConnectionState = 1;
pub const MQCONN_INVALID_SERVER_CERT: MQConnectionState = -2147483639;
pub const MQCONN_LIMIT_REACHED: MQConnectionState = -2147483638;
pub const MQCONN_NAME_RESOLUTION_FAILURE: MQConnectionState = -2147483640;
pub const MQCONN_NOFAILURE: MQConnectionState = 0;
pub const MQCONN_NOT_READY: MQConnectionState = -2147483641;
pub const MQCONN_OUT_OF_MEMORY: MQConnectionState = -2147483635;
pub const MQCONN_PING_FAILURE: MQConnectionState = -2147483647;
pub const MQCONN_READY: MQConnectionState = 2;
pub const MQCONN_REFUSED_BY_OTHER_SIDE: MQConnectionState = -2147483637;
pub const MQCONN_ROUTING_FAILURE: MQConnectionState = -2147483636;
pub const MQCONN_SEND_FAILURE: MQConnectionState = -2147483642;
pub const MQCONN_TCP_NOT_ENABLED: MQConnectionState = -2147483643;
pub const MQCONN_UNKNOWN_FAILURE: MQConnectionState = -2147483648;
pub type MQConnectionState = i32;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMGMTPROPS {
    pub cProp: u32,
    pub aPropID: *mut MGMTPROPID,
    pub aPropVar: *mut MQPROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQMSGPROPS {
    pub cProp: u32,
    pub aPropID: *mut MSGPROPID,
    pub aPropVar: *mut MQPROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
pub const MQMSG_ACKNOWLEDGMENT_FULL_REACH_QUEUE: u32 = 5;
pub const MQMSG_ACKNOWLEDGMENT_FULL_RECEIVE: u32 = 14;
pub const MQMSG_ACKNOWLEDGMENT_NACK_REACH_QUEUE: u32 = 4;
pub const MQMSG_ACKNOWLEDGMENT_NACK_RECEIVE: u32 = 12;
pub const MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL: i32 = 4;
pub const MQMSG_ACKNOWLEDGMENT_NEG_RECEIVE: i32 = 8;
pub const MQMSG_ACKNOWLEDGMENT_NONE: i32 = 0;
pub const MQMSG_ACKNOWLEDGMENT_POS_ARRIVAL: i32 = 1;
pub const MQMSG_ACKNOWLEDGMENT_POS_RECEIVE: i32 = 2;
pub const MQMSG_AUTHENTICATED_QM_MESSAGE: i32 = 11;
pub const MQMSG_AUTHENTICATED_SIG10: i32 = 1;
pub const MQMSG_AUTHENTICATED_SIG20: i32 = 3;
pub const MQMSG_AUTHENTICATED_SIG30: i32 = 5;
pub const MQMSG_AUTHENTICATED_SIGXML: i32 = 9;
pub const MQMSG_AUTHENTICATION_NOT_REQUESTED: i32 = 0;
pub const MQMSG_AUTHENTICATION_REQUESTED: i32 = 1;
pub const MQMSG_AUTHENTICATION_REQUESTED_EX: i32 = 3;
pub const MQMSG_AUTH_LEVEL_ALWAYS: i32 = 1;
pub const MQMSG_AUTH_LEVEL_MSMQ10: i32 = 2;
pub const MQMSG_AUTH_LEVEL_MSMQ20: i32 = 4;
pub const MQMSG_AUTH_LEVEL_NONE: i32 = 0;
pub const MQMSG_AUTH_LEVEL_SIG10: i32 = 2;
pub const MQMSG_AUTH_LEVEL_SIG20: i32 = 4;
pub const MQMSG_AUTH_LEVEL_SIG30: i32 = 8;
pub const MQMSG_CLASS_ACK_REACH_QUEUE: u32 = 2;
pub const MQMSG_CLASS_ACK_RECEIVE: u32 = 16384;
pub const MQMSG_CLASS_NACK_ACCESS_DENIED: u32 = 32772;
pub const MQMSG_CLASS_NACK_BAD_DST_Q: u32 = 32768;
pub const MQMSG_CLASS_NACK_BAD_ENCRYPTION: u32 = 32775;
pub const MQMSG_CLASS_NACK_BAD_SIGNATURE: u32 = 32774;
pub const MQMSG_CLASS_NACK_COULD_NOT_ENCRYPT: u32 = 32776;
pub const MQMSG_CLASS_NACK_HOP_COUNT_EXCEEDED: u32 = 32773;
pub const MQMSG_CLASS_NACK_MESSAGE_TOO_LARGE: u32 = 32781;
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_MSG: u32 = 32778;
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_Q: u32 = 32777;
pub const MQMSG_CLASS_NACK_PURGED: u32 = 32769;
pub const MQMSG_CLASS_NACK_Q_DELETED: u32 = 49152;
pub const MQMSG_CLASS_NACK_Q_EXCEED_QUOTA: u32 = 32771;
pub const MQMSG_CLASS_NACK_Q_PURGED: u32 = 49153;
pub const MQMSG_CLASS_NACK_REACH_QUEUE_TIMEOUT: u32 = 32770;
pub const MQMSG_CLASS_NACK_RECEIVE_REJECTED: u32 = 49156;
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT: u32 = 49154;
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT_AT_SENDER: u32 = 49155;
pub const MQMSG_CLASS_NACK_SOURCE_COMPUTER_GUID_CHANGED: u32 = 32780;
pub const MQMSG_CLASS_NACK_UNSUPPORTED_CRYPTO_PROVIDER: u32 = 32779;
pub const MQMSG_CLASS_NORMAL: u32 = 0;
pub const MQMSG_CLASS_REPORT: u32 = 1;
pub const MQMSG_DEADLETTER: i32 = 1;
pub const MQMSG_DELIVERY_EXPRESS: i32 = 0;
pub const MQMSG_DELIVERY_RECOVERABLE: i32 = 1;
pub const MQMSG_FIRST_IN_XACT: i32 = 1;
pub const MQMSG_JOURNAL: i32 = 2;
pub const MQMSG_JOURNAL_NONE: i32 = 0;
pub const MQMSG_LAST_IN_XACT: i32 = 1;
pub const MQMSG_NOT_FIRST_IN_XACT: i32 = 0;
pub const MQMSG_NOT_LAST_IN_XACT: i32 = 0;
pub const MQMSG_PRIV_LEVEL_BODY_AES: i32 = 5;
pub const MQMSG_PRIV_LEVEL_BODY_BASE: i32 = 1;
pub const MQMSG_PRIV_LEVEL_BODY_ENHANCED: i32 = 3;
pub const MQMSG_PRIV_LEVEL_NONE: i32 = 0;
pub const MQMSG_SENDERID_TYPE_NONE: i32 = 0;
pub const MQMSG_SENDERID_TYPE_SID: i32 = 1;
pub const MQMSG_SEND_ROUTE_TO_REPORT_QUEUE: i32 = 1;
pub const MQMSG_TRACE_NONE: i32 = 0;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQPRIVATEPROPS {
    pub cProp: u32,
    pub aPropID: *mut QMPROPID,
    pub aPropVar: *mut MQPROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub struct MQPROPERTYRESTRICTION {
    pub rel: u32,
    pub prop: super::PROPID,
    pub prval: MQPROPVARIANT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for MQPROPERTYRESTRICTION {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
impl Default for MQPROPERTYRESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub type MQPROPVARIANT = tagMQPROPVARIANT;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQQMPROPS {
    pub cProp: u32,
    pub aPropID: *mut QMPROPID,
    pub aPropVar: *mut MQPROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQQUEUEPROPS {
    pub cProp: u32,
    pub aPropID: *mut QUEUEPROPID,
    pub aPropVar: *mut MQPROPVARIANT,
    pub aStatus: *mut windows_core::HRESULT,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQRESTRICTION {
    pub cRes: u32,
    pub paPropRes: *mut MQPROPERTYRESTRICTION,
}
pub const MQSEC_CHANGE_QUEUE_PERMISSIONS: i32 = 262144;
pub const MQSEC_DELETE_JOURNAL_MESSAGE: i32 = 8;
pub const MQSEC_DELETE_MESSAGE: i32 = 1;
pub const MQSEC_DELETE_QUEUE: i32 = 65536;
pub const MQSEC_GET_QUEUE_PERMISSIONS: i32 = 131072;
pub const MQSEC_GET_QUEUE_PROPERTIES: i32 = 32;
pub const MQSEC_PEEK_MESSAGE: i32 = 2;
pub const MQSEC_QUEUE_GENERIC_ALL: i32 = 983103;
pub const MQSEC_QUEUE_GENERIC_EXECUTE: i32 = 0;
pub const MQSEC_QUEUE_GENERIC_READ: i32 = 131115;
pub const MQSEC_QUEUE_GENERIC_WRITE: i32 = 131108;
pub const MQSEC_RECEIVE_JOURNAL_MESSAGE: i32 = 10;
pub const MQSEC_RECEIVE_MESSAGE: i32 = 3;
pub const MQSEC_SET_QUEUE_PROPERTIES: i32 = 16;
pub const MQSEC_TAKE_QUEUE_OWNERSHIP: i32 = 524288;
pub const MQSEC_WRITE_MESSAGE: i32 = 4;
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQSORTKEY {
    pub propColumn: super::PROPID,
    pub dwOrder: u32,
}
#[repr(C)]
#[cfg(feature = "wtypes")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MQSORTSET {
    pub cCol: u32,
    pub aCol: *mut MQSORTKEY,
}
pub const MQ_ACTION_PEEK_CURRENT: u32 = 2147483648;
pub const MQ_ACTION_PEEK_NEXT: u32 = 2147483649;
pub const MQ_ACTION_RECEIVE: i32 = 0;
pub const MQ_ADMIN_ACCESS: i32 = 128;
pub const MQ_AUTHENTICATE: u32 = 1;
pub const MQ_AUTHENTICATE_NONE: u32 = 0;
pub const MQ_CORRUPTED_QUEUE_WAS_DELETED: windows_core::HRESULT = windows_core::HRESULT(0xC00E0068_u32 as _);
pub const MQ_DENY_NONE: i32 = 0;
pub const MQ_DENY_RECEIVE_SHARE: i32 = 1;
pub const MQ_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC00E0001_u32 as _);
pub const MQ_ERROR_ACCESS_DENIED: windows_core::HRESULT = windows_core::HRESULT(0xC00E0025_u32 as _);
pub const MQ_ERROR_BAD_SECURITY_CONTEXT: windows_core::HRESULT = windows_core::HRESULT(0xC00E0035_u32 as _);
pub const MQ_ERROR_BAD_XML_FORMAT: windows_core::HRESULT = windows_core::HRESULT(0xC00E0092_u32 as _);
pub const MQ_ERROR_BUFFER_OVERFLOW: windows_core::HRESULT = windows_core::HRESULT(0xC00E001A_u32 as _);
pub const MQ_ERROR_CANNOT_CREATE_CERT_STORE: windows_core::HRESULT = windows_core::HRESULT(0xC00E006F_u32 as _);
pub const MQ_ERROR_CANNOT_CREATE_HASH_EX: windows_core::HRESULT = windows_core::HRESULT(0xC00E0081_u32 as _);
pub const MQ_ERROR_CANNOT_CREATE_ON_GC: windows_core::HRESULT = windows_core::HRESULT(0xC00E0077_u32 as _);
pub const MQ_ERROR_CANNOT_CREATE_PSC_OBJECTS: windows_core::HRESULT = windows_core::HRESULT(0xC00E0095_u32 as _);
pub const MQ_ERROR_CANNOT_DELETE_PSC_OBJECTS: windows_core::HRESULT = windows_core::HRESULT(0xC00E0083_u32 as _);
pub const MQ_ERROR_CANNOT_GET_DN: windows_core::HRESULT = windows_core::HRESULT(0xC00E007E_u32 as _);
pub const MQ_ERROR_CANNOT_GRANT_ADD_GUID: windows_core::HRESULT = windows_core::HRESULT(0xC00E0072_u32 as _);
pub const MQ_ERROR_CANNOT_HASH_DATA_EX: windows_core::HRESULT = windows_core::HRESULT(0xC00E007F_u32 as _);
pub const MQ_ERROR_CANNOT_IMPERSONATE_CLIENT: windows_core::HRESULT = windows_core::HRESULT(0xC00E0024_u32 as _);
pub const MQ_ERROR_CANNOT_JOIN_DOMAIN: windows_core::HRESULT = windows_core::HRESULT(0xC00E0076_u32 as _);
pub const MQ_ERROR_CANNOT_LOAD_MQAD: windows_core::HRESULT = windows_core::HRESULT(0xC00E0085_u32 as _);
pub const MQ_ERROR_CANNOT_LOAD_MQDSSRV: windows_core::HRESULT = windows_core::HRESULT(0xC00E0086_u32 as _);
pub const MQ_ERROR_CANNOT_LOAD_MSMQOCM: windows_core::HRESULT = windows_core::HRESULT(0xC00E0073_u32 as _);
pub const MQ_ERROR_CANNOT_OPEN_CERT_STORE: windows_core::HRESULT = windows_core::HRESULT(0xC00E0070_u32 as _);
pub const MQ_ERROR_CANNOT_SET_CRYPTO_SEC_DESCR: windows_core::HRESULT = windows_core::HRESULT(0xC00E006C_u32 as _);
pub const MQ_ERROR_CANNOT_SIGN_DATA_EX: windows_core::HRESULT = windows_core::HRESULT(0xC00E0080_u32 as _);
pub const MQ_ERROR_CANNOT_UPDATE_PSC_OBJECTS: windows_core::HRESULT = windows_core::HRESULT(0xC00E0096_u32 as _);
pub const MQ_ERROR_CANT_RESOLVE_SITES: windows_core::HRESULT = windows_core::HRESULT(0xC00E0089_u32 as _);
pub const MQ_ERROR_CERTIFICATE_NOT_PROVIDED: windows_core::HRESULT = windows_core::HRESULT(0xC00E006D_u32 as _);
pub const MQ_ERROR_COMPUTER_DOES_NOT_SUPPORT_ENCRYPTION: windows_core::HRESULT = windows_core::HRESULT(0xC00E0033_u32 as _);
pub const MQ_ERROR_CORRUPTED_INTERNAL_CERTIFICATE: windows_core::HRESULT = windows_core::HRESULT(0xC00E002D_u32 as _);
pub const MQ_ERROR_CORRUPTED_PERSONAL_CERT_STORE: windows_core::HRESULT = windows_core::HRESULT(0xC00E0031_u32 as _);
pub const MQ_ERROR_CORRUPTED_SECURITY_DATA: windows_core::HRESULT = windows_core::HRESULT(0xC00E0030_u32 as _);
pub const MQ_ERROR_COULD_NOT_GET_ACCOUNT_INFO: windows_core::HRESULT = windows_core::HRESULT(0xC00E0037_u32 as _);
pub const MQ_ERROR_COULD_NOT_GET_USER_SID: windows_core::HRESULT = windows_core::HRESULT(0xC00E0036_u32 as _);
pub const MQ_ERROR_DELETE_CN_IN_USE: windows_core::HRESULT = windows_core::HRESULT(0xC00E0048_u32 as _);
pub const MQ_ERROR_DEPEND_WKS_LICENSE_OVERFLOW: windows_core::HRESULT = windows_core::HRESULT(0xC00E0067_u32 as _);
pub const MQ_ERROR_DS_BIND_ROOT_FOREST: windows_core::HRESULT = windows_core::HRESULT(0xC00E008F_u32 as _);
pub const MQ_ERROR_DS_ERROR: windows_core::HRESULT = windows_core::HRESULT(0xC00E0043_u32 as _);
pub const MQ_ERROR_DS_IS_FULL: windows_core::HRESULT = windows_core::HRESULT(0xC00E0042_u32 as _);
pub const MQ_ERROR_DS_LOCAL_USER: windows_core::HRESULT = windows_core::HRESULT(0xC00E0090_u32 as _);
pub const MQ_ERROR_DTC_CONNECT: windows_core::HRESULT = windows_core::HRESULT(0xC00E004C_u32 as _);
pub const MQ_ERROR_ENCRYPTION_PROVIDER_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC00E006B_u32 as _);
pub const MQ_ERROR_FAIL_VERIFY_SIGNATURE_EX: windows_core::HRESULT = windows_core::HRESULT(0xC00E0082_u32 as _);
pub const MQ_ERROR_FORMATNAME_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0xC00E001F_u32 as _);
pub const MQ_ERROR_GC_NEEDED: windows_core::HRESULT = windows_core::HRESULT(0xC00E008E_u32 as _);
pub const MQ_ERROR_GUID_NOT_MATCHING: windows_core::HRESULT = windows_core::HRESULT(0xC00E0078_u32 as _);
pub const MQ_ERROR_ILLEGAL_CONTEXT: windows_core::HRESULT = windows_core::HRESULT(0xC00E005B_u32 as _);
pub const MQ_ERROR_ILLEGAL_CURSOR_ACTION: windows_core::HRESULT = windows_core::HRESULT(0xC00E001C_u32 as _);
pub const MQ_ERROR_ILLEGAL_ENTERPRISE_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0xC00E0071_u32 as _);
pub const MQ_ERROR_ILLEGAL_FORMATNAME: windows_core::HRESULT = windows_core::HRESULT(0xC00E001E_u32 as _);
pub const MQ_ERROR_ILLEGAL_MQCOLUMNS: windows_core::HRESULT = windows_core::HRESULT(0xC00E0038_u32 as _);
pub const MQ_ERROR_ILLEGAL_MQPRIVATEPROPS: windows_core::HRESULT = windows_core::HRESULT(0xC00E007B_u32 as _);
pub const MQ_ERROR_ILLEGAL_MQQMPROPS: windows_core::HRESULT = windows_core::HRESULT(0xC00E0041_u32 as _);
pub const MQ_ERROR_ILLEGAL_MQQUEUEPROPS: windows_core::HRESULT = windows_core::HRESULT(0xC00E003D_u32 as _);
pub const MQ_ERROR_ILLEGAL_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0xC00E0064_u32 as _);
pub const MQ_ERROR_ILLEGAL_PROPERTY_SIZE: windows_core::HRESULT = windows_core::HRESULT(0xC00E003B_u32 as _);
pub const MQ_ERROR_ILLEGAL_PROPERTY_VALUE: windows_core::HRESULT = windows_core::HRESULT(0xC00E0018_u32 as _);
pub const MQ_ERROR_ILLEGAL_PROPERTY_VT: windows_core::HRESULT = windows_core::HRESULT(0xC00E0019_u32 as _);
pub const MQ_ERROR_ILLEGAL_PROPID: windows_core::HRESULT = windows_core::HRESULT(0xC00E0039_u32 as _);
pub const MQ_ERROR_ILLEGAL_QUEUE_PATHNAME: windows_core::HRESULT = windows_core::HRESULT(0xC00E0014_u32 as _);
pub const MQ_ERROR_ILLEGAL_RELATION: windows_core::HRESULT = windows_core::HRESULT(0xC00E003A_u32 as _);
pub const MQ_ERROR_ILLEGAL_RESTRICTION_PROPID: windows_core::HRESULT = windows_core::HRESULT(0xC00E003C_u32 as _);
pub const MQ_ERROR_ILLEGAL_SECURITY_DESCRIPTOR: windows_core::HRESULT = windows_core::HRESULT(0xC00E0021_u32 as _);
pub const MQ_ERROR_ILLEGAL_SORT: windows_core::HRESULT = windows_core::HRESULT(0xC00E0010_u32 as _);
pub const MQ_ERROR_ILLEGAL_SORT_PROPID: windows_core::HRESULT = windows_core::HRESULT(0xC00E005C_u32 as _);
pub const MQ_ERROR_ILLEGAL_USER: windows_core::HRESULT = windows_core::HRESULT(0xC00E0011_u32 as _);
pub const MQ_ERROR_INSUFFICIENT_PROPERTIES: windows_core::HRESULT = windows_core::HRESULT(0xC00E003F_u32 as _);
pub const MQ_ERROR_INSUFFICIENT_RESOURCES: windows_core::HRESULT = windows_core::HRESULT(0xC00E0027_u32 as _);
pub const MQ_ERROR_INTERNAL_USER_CERT_EXIST: windows_core::HRESULT = windows_core::HRESULT(0xC00E002E_u32 as _);
pub const MQ_ERROR_INVALID_CERTIFICATE: windows_core::HRESULT = windows_core::HRESULT(0xC00E002C_u32 as _);
pub const MQ_ERROR_INVALID_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0xC00E0007_u32 as _);
pub const MQ_ERROR_INVALID_OWNER: windows_core::HRESULT = windows_core::HRESULT(0xC00E0044_u32 as _);
pub const MQ_ERROR_INVALID_PARAMETER: windows_core::HRESULT = windows_core::HRESULT(0xC00E0006_u32 as _);
pub const MQ_ERROR_IO_TIMEOUT: windows_core::HRESULT = windows_core::HRESULT(0xC00E001B_u32 as _);
pub const MQ_ERROR_LABEL_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0xC00E005E_u32 as _);
pub const MQ_ERROR_LABEL_TOO_LONG: windows_core::HRESULT = windows_core::HRESULT(0xC00E005D_u32 as _);
pub const MQ_ERROR_MACHINE_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0xC00E0040_u32 as _);
pub const MQ_ERROR_MACHINE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC00E000D_u32 as _);
pub const MQ_ERROR_MESSAGE_ALREADY_RECEIVED: windows_core::HRESULT = windows_core::HRESULT(0xC00E001D_u32 as _);
pub const MQ_ERROR_MESSAGE_LOCKED_UNDER_TRANSACTION: windows_core::HRESULT = windows_core::HRESULT(0xC00E009C_u32 as _);
pub const MQ_ERROR_MESSAGE_NOT_AUTHENTICATED: windows_core::HRESULT = windows_core::HRESULT(0xC00E009B_u32 as _);
pub const MQ_ERROR_MESSAGE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC00E0088_u32 as _);
pub const MQ_ERROR_MESSAGE_STORAGE_FAILED: windows_core::HRESULT = windows_core::HRESULT(0xC00E002A_u32 as _);
pub const MQ_ERROR_MISSING_CONNECTOR_TYPE: windows_core::HRESULT = windows_core::HRESULT(0xC00E0055_u32 as _);
pub const MQ_ERROR_MQIS_READONLY_MODE: windows_core::HRESULT = windows_core::HRESULT(0xC00E0060_u32 as _);
pub const MQ_ERROR_MQIS_SERVER_EMPTY: windows_core::HRESULT = windows_core::HRESULT(0xC00E005F_u32 as _);
pub const MQ_ERROR_MULTI_SORT_KEYS: windows_core::HRESULT = windows_core::HRESULT(0xC00E008D_u32 as _);
pub const MQ_ERROR_NOT_A_CORRECT_OBJECT_CLASS: windows_core::HRESULT = windows_core::HRESULT(0xC00E008C_u32 as _);
pub const MQ_ERROR_NOT_SUPPORTED_BY_DEPENDENT_CLIENTS: windows_core::HRESULT = windows_core::HRESULT(0xC00E008A_u32 as _);
pub const MQ_ERROR_NO_DS: windows_core::HRESULT = windows_core::HRESULT(0xC00E0013_u32 as _);
pub const MQ_ERROR_NO_ENTRY_POINT_MSMQOCM: windows_core::HRESULT = windows_core::HRESULT(0xC00E0074_u32 as _);
pub const MQ_ERROR_NO_GC_IN_DOMAIN: windows_core::HRESULT = windows_core::HRESULT(0xC00E007C_u32 as _);
pub const MQ_ERROR_NO_INTERNAL_USER_CERT: windows_core::HRESULT = windows_core::HRESULT(0xC00E002F_u32 as _);
pub const MQ_ERROR_NO_MQUSER_OU: windows_core::HRESULT = windows_core::HRESULT(0xC00E0084_u32 as _);
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_DC: windows_core::HRESULT = windows_core::HRESULT(0xC00E0075_u32 as _);
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_GC: windows_core::HRESULT = windows_core::HRESULT(0xC00E007D_u32 as _);
pub const MQ_ERROR_NO_RESPONSE_FROM_OBJECT_SERVER: windows_core::HRESULT = windows_core::HRESULT(0xC00E0049_u32 as _);
pub const MQ_ERROR_OBJECT_SERVER_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC00E004A_u32 as _);
pub const MQ_ERROR_OPERATION_CANCELLED: windows_core::HRESULT = windows_core::HRESULT(0xC00E0008_u32 as _);
pub const MQ_ERROR_OPERATION_NOT_SUPPORTED_BY_REMOTE_COMPUTER: windows_core::HRESULT = windows_core::HRESULT(0xC00E008B_u32 as _);
pub const MQ_ERROR_PRIVILEGE_NOT_HELD: windows_core::HRESULT = windows_core::HRESULT(0xC00E0026_u32 as _);
pub const MQ_ERROR_PROPERTIES_CONFLICT: windows_core::HRESULT = windows_core::HRESULT(0xC00E0087_u32 as _);
pub const MQ_ERROR_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0xC00E0002_u32 as _);
pub const MQ_ERROR_PROPERTY_NOTALLOWED: windows_core::HRESULT = windows_core::HRESULT(0xC00E003E_u32 as _);
pub const MQ_ERROR_PROV_NAME_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0xC00E0063_u32 as _);
pub const MQ_ERROR_PUBLIC_KEY_DOES_NOT_EXIST: windows_core::HRESULT = windows_core::HRESULT(0xC00E007A_u32 as _);
pub const MQ_ERROR_PUBLIC_KEY_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC00E0079_u32 as _);
pub const MQ_ERROR_QUEUE_DELETED: windows_core::HRESULT = windows_core::HRESULT(0xC00E005A_u32 as _);
pub const MQ_ERROR_QUEUE_EXISTS: windows_core::HRESULT = windows_core::HRESULT(0xC00E0005_u32 as _);
pub const MQ_ERROR_QUEUE_NOT_ACTIVE: windows_core::HRESULT = windows_core::HRESULT(0xC00E0004_u32 as _);
pub const MQ_ERROR_QUEUE_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC00E004B_u32 as _);
pub const MQ_ERROR_QUEUE_NOT_FOUND: windows_core::HRESULT = windows_core::HRESULT(0xC00E0003_u32 as _);
pub const MQ_ERROR_Q_ADS_PROPERTY_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC00E0091_u32 as _);
pub const MQ_ERROR_Q_DNS_PROPERTY_NOT_SUPPORTED: windows_core::HRESULT = windows_core::HRESULT(0xC00E006E_u32 as _);
pub const MQ_ERROR_REMOTE_MACHINE_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC00E0069_u32 as _);
pub const MQ_ERROR_RESOLVE_ADDRESS: windows_core::HRESULT = windows_core::HRESULT(0xC00E0099_u32 as _);
pub const MQ_ERROR_RESULT_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0xC00E0046_u32 as _);
pub const MQ_ERROR_SECURITY_DESCRIPTOR_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0xC00E0023_u32 as _);
pub const MQ_ERROR_SENDERID_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0xC00E0022_u32 as _);
pub const MQ_ERROR_SENDER_CERT_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0xC00E002B_u32 as _);
pub const MQ_ERROR_SERVICE_NOT_AVAILABLE: windows_core::HRESULT = windows_core::HRESULT(0xC00E000B_u32 as _);
pub const MQ_ERROR_SHARING_VIOLATION: windows_core::HRESULT = windows_core::HRESULT(0xC00E0009_u32 as _);
pub const MQ_ERROR_SIGNATURE_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0xC00E0062_u32 as _);
pub const MQ_ERROR_STALE_HANDLE: windows_core::HRESULT = windows_core::HRESULT(0xC00E0056_u32 as _);
pub const MQ_ERROR_SYMM_KEY_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0xC00E0061_u32 as _);
pub const MQ_ERROR_TOO_MANY_PROPERTIES: windows_core::HRESULT = windows_core::HRESULT(0xC00E009A_u32 as _);
pub const MQ_ERROR_TRANSACTION_ENLIST: windows_core::HRESULT = windows_core::HRESULT(0xC00E0058_u32 as _);
pub const MQ_ERROR_TRANSACTION_IMPORT: windows_core::HRESULT = windows_core::HRESULT(0xC00E004E_u32 as _);
pub const MQ_ERROR_TRANSACTION_SEQUENCE: windows_core::HRESULT = windows_core::HRESULT(0xC00E0051_u32 as _);
pub const MQ_ERROR_TRANSACTION_USAGE: windows_core::HRESULT = windows_core::HRESULT(0xC00E0050_u32 as _);
pub const MQ_ERROR_UNINITIALIZED_OBJECT: windows_core::HRESULT = windows_core::HRESULT(0xC00E0094_u32 as _);
pub const MQ_ERROR_UNSUPPORTED_ACCESS_MODE: windows_core::HRESULT = windows_core::HRESULT(0xC00E0045_u32 as _);
pub const MQ_ERROR_UNSUPPORTED_CLASS: windows_core::HRESULT = windows_core::HRESULT(0xC00E0093_u32 as _);
pub const MQ_ERROR_UNSUPPORTED_FORMATNAME_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0xC00E0020_u32 as _);
pub const MQ_ERROR_UNSUPPORTED_OPERATION: windows_core::HRESULT = windows_core::HRESULT(0xC00E006A_u32 as _);
pub const MQ_ERROR_USER_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0xC00E0028_u32 as _);
pub const MQ_ERROR_WKS_CANT_SERVE_CLIENT: windows_core::HRESULT = windows_core::HRESULT(0xC00E0066_u32 as _);
pub const MQ_ERROR_WRITE_NOT_ALLOWED: windows_core::HRESULT = windows_core::HRESULT(0xC00E0065_u32 as _);
pub const MQ_INFORMATION_DUPLICATE_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x400E0005_u32 as _);
pub const MQ_INFORMATION_FORMATNAME_BUFFER_TOO_SMALL: windows_core::HRESULT = windows_core::HRESULT(0x400E0009_u32 as _);
pub const MQ_INFORMATION_ILLEGAL_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x400E0002_u32 as _);
pub const MQ_INFORMATION_INTERNAL_USER_CERT_EXIST: windows_core::HRESULT = windows_core::HRESULT(0x400E000A_u32 as _);
pub const MQ_INFORMATION_OPERATION_PENDING: windows_core::HRESULT = windows_core::HRESULT(0x400E0006_u32 as _);
pub const MQ_INFORMATION_OWNER_IGNORED: windows_core::HRESULT = windows_core::HRESULT(0x400E000B_u32 as _);
pub const MQ_INFORMATION_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x400E0001_u32 as _);
pub const MQ_INFORMATION_PROPERTY_IGNORED: windows_core::HRESULT = windows_core::HRESULT(0x400E0003_u32 as _);
pub const MQ_INFORMATION_UNSUPPORTED_PROPERTY: windows_core::HRESULT = windows_core::HRESULT(0x400E0004_u32 as _);
pub const MQ_JOURNAL: u32 = 1;
pub const MQ_JOURNAL_NONE: u32 = 0;
pub const MQ_LOOKUP_PEEK_CURRENT: i32 = 1073741840;
pub const MQ_LOOKUP_PEEK_FIRST: i32 = 1073741844;
pub const MQ_LOOKUP_PEEK_LAST: i32 = 1073741848;
pub const MQ_LOOKUP_PEEK_NEXT: i32 = 1073741841;
pub const MQ_LOOKUP_PEEK_PREV: i32 = 1073741842;
pub const MQ_LOOKUP_RECEIVE_ALLOW_PEEK: i32 = 1073742112;
pub const MQ_LOOKUP_RECEIVE_CURRENT: i32 = 1073741856;
pub const MQ_LOOKUP_RECEIVE_FIRST: i32 = 1073741860;
pub const MQ_LOOKUP_RECEIVE_LAST: i32 = 1073741864;
pub const MQ_LOOKUP_RECEIVE_NEXT: i32 = 1073741857;
pub const MQ_LOOKUP_RECEIVE_PREV: i32 = 1073741858;
pub const MQ_MAX_MSG_LABEL_LEN: i32 = 250;
pub const MQ_MAX_PRIORITY: i32 = 7;
pub const MQ_MAX_Q_LABEL_LEN: i32 = 124;
pub const MQ_MAX_Q_NAME_LEN: i32 = 124;
pub const MQ_MIN_PRIORITY: i32 = 0;
pub const MQ_MOVE_ACCESS: i32 = 4;
pub const MQ_NO_TRANSACTION: i32 = 0;
pub const MQ_OK: windows_core::HRESULT = windows_core::HRESULT(0x0_u32 as _);
pub const MQ_PEEK_ACCESS: i32 = 32;
pub const MQ_PRIV_LEVEL_BODY: u32 = 2;
pub const MQ_PRIV_LEVEL_NONE: u32 = 0;
pub const MQ_PRIV_LEVEL_OPTIONAL: u32 = 1;
pub const MQ_RECEIVE_ACCESS: i32 = 1;
pub const MQ_SEND_ACCESS: i32 = 2;
pub const MQ_TRANSACTIONAL: u32 = 1;
pub const MQ_TRANSACTIONAL_NONE: u32 = 0;
#[cfg(feature = "wtypes")]
pub type MSGPROPID = super::PROPID;
pub const MSMQ_CONNECTED: windows_core::PCWSTR = windows_core::w!("CONNECTED");
pub const MSMQ_DISCONNECTED: windows_core::PCWSTR = windows_core::w!("DISCONNECTED");
#[cfg(all(feature = "minwinbase", feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub type PMQRECEIVECALLBACK = Option<unsafe extern "system" fn(hrstatus: windows_core::HRESULT, hsource: QUEUEHANDLE, dwtimeout: u32, dwaction: u32, pmessageprops: *mut MQMSGPROPS, lpoverlapped: *mut super::OVERLAPPED, hcursor: super::HANDLE)>;
pub const PREQ: i32 = 4;
pub const PRGE: i32 = 3;
pub const PRGT: i32 = 2;
pub const PRLE: i32 = 1;
pub const PRLT: i32 = 0;
pub const PRNE: i32 = 5;
pub const PROPID_MGMT_MSMQ_ACTIVEQUEUES: i32 = 1;
pub const PROPID_MGMT_MSMQ_BASE: i32 = 0;
pub const PROPID_MGMT_MSMQ_BYTES_IN_ALL_QUEUES: i32 = 6;
pub const PROPID_MGMT_MSMQ_CONNECTED: i32 = 4;
pub const PROPID_MGMT_MSMQ_DSSERVER: i32 = 3;
pub const PROPID_MGMT_MSMQ_PRIVATEQ: i32 = 2;
pub const PROPID_MGMT_MSMQ_TYPE: i32 = 5;
pub const PROPID_MGMT_QUEUE_BASE: i32 = 0;
pub const PROPID_MGMT_QUEUE_BYTES_IN_JOURNAL: i32 = 10;
pub const PROPID_MGMT_QUEUE_BYTES_IN_QUEUE: i32 = 8;
pub const PROPID_MGMT_QUEUE_CONNECTION_HISTORY: i32 = 25;
pub const PROPID_MGMT_QUEUE_EOD_FIRST_NON_ACK: i32 = 16;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK: i32 = 13;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_COUNT: i32 = 15;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_TIME: i32 = 14;
pub const PROPID_MGMT_QUEUE_EOD_LAST_NON_ACK: i32 = 17;
pub const PROPID_MGMT_QUEUE_EOD_NEXT_SEQ: i32 = 18;
pub const PROPID_MGMT_QUEUE_EOD_NO_ACK_COUNT: i32 = 20;
pub const PROPID_MGMT_QUEUE_EOD_NO_READ_COUNT: i32 = 19;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_COUNT: i32 = 23;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_INTERVAL: i32 = 22;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_TIME: i32 = 21;
pub const PROPID_MGMT_QUEUE_EOD_SOURCE_INFO: i32 = 24;
pub const PROPID_MGMT_QUEUE_FOREIGN: i32 = 6;
pub const PROPID_MGMT_QUEUE_FORMATNAME: i32 = 2;
pub const PROPID_MGMT_QUEUE_JOURNAL_MESSAGE_COUNT: i32 = 9;
pub const PROPID_MGMT_QUEUE_JOURNAL_USED_QUOTA: i32 = 10;
pub const PROPID_MGMT_QUEUE_LOCATION: i32 = 4;
pub const PROPID_MGMT_QUEUE_MESSAGE_COUNT: i32 = 7;
pub const PROPID_MGMT_QUEUE_NEXTHOPS: i32 = 12;
pub const PROPID_MGMT_QUEUE_PATHNAME: i32 = 1;
pub const PROPID_MGMT_QUEUE_STATE: i32 = 11;
pub const PROPID_MGMT_QUEUE_SUBQUEUE_COUNT: i32 = 26;
pub const PROPID_MGMT_QUEUE_SUBQUEUE_NAMES: i32 = 27;
pub const PROPID_MGMT_QUEUE_TYPE: i32 = 3;
pub const PROPID_MGMT_QUEUE_USED_QUOTA: i32 = 8;
pub const PROPID_MGMT_QUEUE_XACT: i32 = 5;
pub const PROPID_M_ABORT_COUNT: i32 = 69;
pub const PROPID_M_ACKNOWLEDGE: i32 = 6;
pub const PROPID_M_ADMIN_QUEUE: i32 = 17;
pub const PROPID_M_ADMIN_QUEUE_LEN: i32 = 18;
pub const PROPID_M_APPSPECIFIC: i32 = 8;
pub const PROPID_M_ARRIVEDTIME: i32 = 32;
pub const PROPID_M_AUTHENTICATED: i32 = 25;
pub const PROPID_M_AUTHENTICATED_EX: i32 = 53;
pub const PROPID_M_AUTH_LEVEL: i32 = 24;
pub const PROPID_M_BASE: i32 = 0;
pub const PROPID_M_BODY: i32 = 9;
pub const PROPID_M_BODY_SIZE: i32 = 10;
pub const PROPID_M_BODY_TYPE: i32 = 42;
pub const PROPID_M_CLASS: i32 = 1;
pub const PROPID_M_COMPOUND_MESSAGE: i32 = 63;
pub const PROPID_M_COMPOUND_MESSAGE_SIZE: i32 = 64;
pub const PROPID_M_CONNECTOR_TYPE: i32 = 38;
pub const PROPID_M_CORRELATIONID: i32 = 3;
pub const PROPID_M_CORRELATIONID_SIZE: i32 = 20;
pub const PROPID_M_DEADLETTER_QUEUE: i32 = 67;
pub const PROPID_M_DEADLETTER_QUEUE_LEN: i32 = 68;
pub const PROPID_M_DELIVERY: i32 = 5;
pub const PROPID_M_DEST_FORMAT_NAME: i32 = 58;
pub const PROPID_M_DEST_FORMAT_NAME_LEN: i32 = 59;
pub const PROPID_M_DEST_QUEUE: i32 = 33;
pub const PROPID_M_DEST_QUEUE_LEN: i32 = 34;
pub const PROPID_M_DEST_SYMM_KEY: i32 = 43;
pub const PROPID_M_DEST_SYMM_KEY_LEN: i32 = 44;
pub const PROPID_M_ENCRYPTION_ALG: i32 = 27;
pub const PROPID_M_EXTENSION: i32 = 35;
pub const PROPID_M_EXTENSION_LEN: i32 = 36;
pub const PROPID_M_FIRST_IN_XACT: i32 = 50;
pub const PROPID_M_HASH_ALG: i32 = 26;
pub const PROPID_M_JOURNAL: i32 = 7;
pub const PROPID_M_LABEL: i32 = 11;
pub const PROPID_M_LABEL_LEN: i32 = 12;
pub const PROPID_M_LAST_IN_XACT: i32 = 51;
pub const PROPID_M_LAST_MOVE_TIME: i32 = 75;
pub const PROPID_M_LOOKUPID: i32 = 60;
pub const PROPID_M_MOVE_COUNT: i32 = 70;
pub const PROPID_M_MSGID: i32 = 2;
pub const PROPID_M_MSGID_SIZE: i32 = 20;
pub const PROPID_M_PRIORITY: i32 = 4;
pub const PROPID_M_PRIV_LEVEL: i32 = 23;
pub const PROPID_M_PROV_NAME: i32 = 48;
pub const PROPID_M_PROV_NAME_LEN: i32 = 49;
pub const PROPID_M_PROV_TYPE: i32 = 47;
pub const PROPID_M_RESP_FORMAT_NAME: i32 = 54;
pub const PROPID_M_RESP_FORMAT_NAME_LEN: i32 = 55;
pub const PROPID_M_RESP_QUEUE: i32 = 15;
pub const PROPID_M_RESP_QUEUE_LEN: i32 = 16;
pub const PROPID_M_SECURITY_CONTEXT: i32 = 37;
pub const PROPID_M_SENDERID: i32 = 20;
pub const PROPID_M_SENDERID_LEN: i32 = 21;
pub const PROPID_M_SENDERID_TYPE: i32 = 22;
pub const PROPID_M_SENDER_CERT: i32 = 28;
pub const PROPID_M_SENDER_CERT_LEN: i32 = 29;
pub const PROPID_M_SENTTIME: i32 = 31;
pub const PROPID_M_SIGNATURE: i32 = 45;
pub const PROPID_M_SIGNATURE_LEN: i32 = 46;
pub const PROPID_M_SOAP_BODY: i32 = 66;
pub const PROPID_M_SOAP_ENVELOPE: i32 = 61;
pub const PROPID_M_SOAP_ENVELOPE_LEN: i32 = 62;
pub const PROPID_M_SOAP_HEADER: i32 = 65;
pub const PROPID_M_SRC_MACHINE_ID: i32 = 30;
pub const PROPID_M_TIME_TO_BE_RECEIVED: i32 = 14;
pub const PROPID_M_TIME_TO_REACH_QUEUE: i32 = 13;
pub const PROPID_M_TRACE: i32 = 41;
pub const PROPID_M_VERSION: i32 = 19;
pub const PROPID_M_XACTID: i32 = 52;
pub const PROPID_M_XACTID_SIZE: i32 = 20;
pub const PROPID_M_XACT_STATUS_QUEUE: i32 = 39;
pub const PROPID_M_XACT_STATUS_QUEUE_LEN: i32 = 40;
pub const PROPID_PC_BASE: i32 = 5800;
pub const PROPID_PC_DS_ENABLED: i32 = 5802;
pub const PROPID_PC_VERSION: i32 = 5801;
pub const PROPID_QM_BASE: i32 = 200;
pub const PROPID_QM_CONNECTION: i32 = 204;
pub const PROPID_QM_ENCRYPTION_PK: i32 = 205;
pub const PROPID_QM_ENCRYPTION_PK_AES: i32 = 244;
pub const PROPID_QM_ENCRYPTION_PK_BASE: i32 = 231;
pub const PROPID_QM_ENCRYPTION_PK_ENHANCED: i32 = 232;
pub const PROPID_QM_MACHINE_ID: i32 = 202;
pub const PROPID_QM_PATHNAME: i32 = 203;
pub const PROPID_QM_PATHNAME_DNS: i32 = 233;
pub const PROPID_QM_SITE_ID: i32 = 201;
pub const PROPID_Q_ADS_PATH: i32 = 126;
pub const PROPID_Q_AUTHENTICATE: i32 = 111;
pub const PROPID_Q_BASE: i32 = 100;
pub const PROPID_Q_BASEPRIORITY: i32 = 106;
pub const PROPID_Q_CREATE_TIME: i32 = 109;
pub const PROPID_Q_INSTANCE: i32 = 101;
pub const PROPID_Q_JOURNAL: i32 = 104;
pub const PROPID_Q_JOURNAL_QUOTA: i32 = 107;
pub const PROPID_Q_LABEL: i32 = 108;
pub const PROPID_Q_MODIFY_TIME: i32 = 110;
pub const PROPID_Q_MULTICAST_ADDRESS: i32 = 125;
pub const PROPID_Q_PATHNAME: i32 = 103;
pub const PROPID_Q_PATHNAME_DNS: i32 = 124;
pub const PROPID_Q_PRIV_LEVEL: i32 = 112;
pub const PROPID_Q_QUOTA: i32 = 105;
pub const PROPID_Q_TRANSACTION: i32 = 113;
pub const PROPID_Q_TYPE: i32 = 102;
#[cfg(feature = "wtypes")]
pub type QMPROPID = super::PROPID;
pub const QUERY_SORTASCEND: i32 = 0;
pub const QUERY_SORTDESCEND: i32 = 1;
#[cfg(feature = "winnt")]
pub type QUEUEHANDLE = super::HANDLE;
#[cfg(feature = "wtypes")]
pub type QUEUEPROPID = super::PROPID;
pub const QUEUE_ACTION_EOD_RESEND: windows_core::PCWSTR = windows_core::w!("EOD_RESEND");
pub const QUEUE_ACTION_PAUSE: windows_core::PCWSTR = windows_core::w!("PAUSE");
pub const QUEUE_ACTION_RESUME: windows_core::PCWSTR = windows_core::w!("RESUME");
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SEQUENCE_INFO {
    pub SeqID: i64,
    pub SeqNo: u32,
    pub PrevNo: u32,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "objidl", feature = "objidlbase", feature = "propidlbase", feature = "wtypes", feature = "wtypesbase"))]
pub type tagMQPROPVARIANT = super::PROPVARIANT;
