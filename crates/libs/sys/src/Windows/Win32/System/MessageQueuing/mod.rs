windows_link::link!("mqrt.dll" "system" fn MQADsPathToFormatName(lpwcsadspath : windows_sys::core::PCWSTR, lpwcsformatname : windows_sys::core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
windows_link::link!("mqrt.dll" "system" fn MQBeginTransaction(pptransaction : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQCloseCursor(hcursor : super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQCloseQueue(hqueue : isize) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQCreateCursor(hqueue : isize, phcursor : *mut super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_Security", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
windows_link::link!("mqrt.dll" "system" fn MQCreateQueue(psecuritydescriptor : super::super::Security::PSECURITY_DESCRIPTOR, pqueueprops : *mut MQQUEUEPROPS, lpwcsformatname : windows_sys::core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQDeleteQueue(lpwcsformatname : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQFreeMemory(pvmemory : *const core::ffi::c_void));
windows_link::link!("mqrt.dll" "system" fn MQFreeSecurityContext(hsecuritycontext : super::super::Foundation::HANDLE));
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
windows_link::link!("mqrt.dll" "system" fn MQGetMachineProperties(lpwcsmachinename : windows_sys::core::PCWSTR, pguidmachineid : *const windows_sys::core::GUID, pqmprops : *mut MQQMPROPS) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_IO")]
windows_link::link!("mqrt.dll" "system" fn MQGetOverlappedResult(lpoverlapped : *const super::IO::OVERLAPPED) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
windows_link::link!("mqrt.dll" "system" fn MQGetPrivateComputerInformation(lpwcscomputername : windows_sys::core::PCWSTR, pprivateprops : *mut MQPRIVATEPROPS) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
windows_link::link!("mqrt.dll" "system" fn MQGetQueueProperties(lpwcsformatname : windows_sys::core::PCWSTR, pqueueprops : *mut MQQUEUEPROPS) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Security")]
windows_link::link!("mqrt.dll" "system" fn MQGetQueueSecurity(lpwcsformatname : windows_sys::core::PCWSTR, requestedinformation : u32, psecuritydescriptor : super::super::Security::PSECURITY_DESCRIPTOR, nlength : u32, lpnlengthneeded : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQGetSecurityContext(lpcertbuffer : *const core::ffi::c_void, dwcertbufferlength : u32, phsecuritycontext : *mut super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQGetSecurityContextEx(lpcertbuffer : *const core::ffi::c_void, dwcertbufferlength : u32, phsecuritycontext : *mut super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQHandleToFormatName(hqueue : isize, lpwcsformatname : windows_sys::core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQInstanceToFormatName(pguid : *const windows_sys::core::GUID, lpwcsformatname : windows_sys::core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
windows_link::link!("mqrt.dll" "system" fn MQLocateBegin(lpwcscontext : windows_sys::core::PCWSTR, prestriction : *const MQRESTRICTION, pcolumns : *const MQCOLUMNSET, psort : *const MQSORTSET, phenum : *mut super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQLocateEnd(henum : super::super::Foundation::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
windows_link::link!("mqrt.dll" "system" fn MQLocateNext(henum : super::super::Foundation::HANDLE, pcprops : *mut u32, apropvar : *mut super::Com::StructuredStorage::PROPVARIANT) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQMarkMessageRejected(hqueue : super::super::Foundation::HANDLE, ulllookupid : u64) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQMgmtAction(pcomputername : windows_sys::core::PCWSTR, pobjectname : windows_sys::core::PCWSTR, paction : windows_sys::core::PCWSTR) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
windows_link::link!("mqrt.dll" "system" fn MQMgmtGetInfo(pcomputername : windows_sys::core::PCWSTR, pobjectname : windows_sys::core::PCWSTR, pmgmtprops : *mut MQMGMTPROPS) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_System_DistributedTransactionCoordinator")]
windows_link::link!("mqrt.dll" "system" fn MQMoveMessage(hsourcequeue : isize, hdestinationqueue : isize, ulllookupid : u64, ptransaction : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQOpenQueue(lpwcsformatname : windows_sys::core::PCWSTR, dwaccess : u32, dwsharemode : u32, phqueue : *mut isize) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQPathNameToFormatName(lpwcspathname : windows_sys::core::PCWSTR, lpwcsformatname : windows_sys::core::PWSTR, lpdwformatnamelength : *mut u32) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQPurgeQueue(hqueue : isize) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_DistributedTransactionCoordinator", feature = "Win32_System_IO", feature = "Win32_System_Variant"))]
windows_link::link!("mqrt.dll" "system" fn MQReceiveMessage(hsource : isize, dwtimeout : u32, dwaction : u32, pmessageprops : *mut MQMSGPROPS, lpoverlapped : *mut super::IO::OVERLAPPED, fnreceivecallback : PMQRECEIVECALLBACK, hcursor : super::super::Foundation::HANDLE, ptransaction : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_DistributedTransactionCoordinator", feature = "Win32_System_IO", feature = "Win32_System_Variant"))]
windows_link::link!("mqrt.dll" "system" fn MQReceiveMessageByLookupId(hsource : isize, ulllookupid : u64, dwlookupaction : u32, pmessageprops : *mut MQMSGPROPS, lpoverlapped : *mut super::IO::OVERLAPPED, fnreceivecallback : PMQRECEIVECALLBACK, ptransaction : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
windows_link::link!("mqrt.dll" "system" fn MQRegisterCertificate(dwflags : u32, lpcertbuffer : *const core::ffi::c_void, dwcertbufferlength : u32) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_DistributedTransactionCoordinator", feature = "Win32_System_Variant"))]
windows_link::link!("mqrt.dll" "system" fn MQSendMessage(hdestinationqueue : isize, pmessageprops : *const MQMSGPROPS, ptransaction : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
windows_link::link!("mqrt.dll" "system" fn MQSetQueueProperties(lpwcsformatname : windows_sys::core::PCWSTR, pqueueprops : *mut MQQUEUEPROPS) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Security")]
windows_link::link!("mqrt.dll" "system" fn MQSetQueueSecurity(lpwcsformatname : windows_sys::core::PCWSTR, securityinformation : super::super::Security::OBJECT_SECURITY_INFORMATION, psecuritydescriptor : super::super::Security::PSECURITY_DESCRIPTOR) -> windows_sys::core::HRESULT);
pub const DEFAULT_M_ACKNOWLEDGE: MQDEFAULT = 0;
pub const DEFAULT_M_APPSPECIFIC: MQDEFAULT = 0;
pub const DEFAULT_M_AUTH_LEVEL: MQDEFAULT = 0;
pub const DEFAULT_M_DELIVERY: MQDEFAULT = 0;
pub const DEFAULT_M_JOURNAL: MQDEFAULT = 0;
pub const DEFAULT_M_LOOKUPID: MQDEFAULT = 0;
pub const DEFAULT_M_PRIORITY: MQDEFAULT = 3;
pub const DEFAULT_M_PRIV_LEVEL: MQDEFAULT = 0;
pub const DEFAULT_M_SENDERID_TYPE: MQDEFAULT = 1;
pub const DEFAULT_Q_AUTHENTICATE: MQDEFAULT = 0;
pub const DEFAULT_Q_BASEPRIORITY: MQDEFAULT = 0;
pub const DEFAULT_Q_JOURNAL: MQDEFAULT = 0;
pub const DEFAULT_Q_JOURNAL_QUOTA: MQDEFAULT = -1;
pub const DEFAULT_Q_PRIV_LEVEL: MQDEFAULT = 1;
pub const DEFAULT_Q_QUOTA: MQDEFAULT = -1;
pub const DEFAULT_Q_TRANSACTION: MQDEFAULT = 0;
pub type FOREIGN_STATUS = i32;
pub const LONG_LIVED: u32 = 4294967294;
pub const MACHINE_ACTION_CONNECT: windows_sys::core::PCWSTR = windows_sys::core::w!("CONNECT");
pub const MACHINE_ACTION_DISCONNECT: windows_sys::core::PCWSTR = windows_sys::core::w!("DISCONNECT");
pub const MACHINE_ACTION_TIDY: windows_sys::core::PCWSTR = windows_sys::core::w!("TIDY");
pub const MGMT_QUEUE_CORRECT_TYPE: windows_sys::core::PCWSTR = windows_sys::core::w!("YES");
pub const MGMT_QUEUE_FOREIGN_TYPE: windows_sys::core::PCWSTR = windows_sys::core::w!("YES");
pub const MGMT_QUEUE_INCORRECT_TYPE: windows_sys::core::PCWSTR = windows_sys::core::w!("NO");
pub const MGMT_QUEUE_LOCAL_LOCATION: windows_sys::core::PCWSTR = windows_sys::core::w!("LOCAL");
pub const MGMT_QUEUE_NOT_FOREIGN_TYPE: windows_sys::core::PCWSTR = windows_sys::core::w!("NO");
pub const MGMT_QUEUE_NOT_TRANSACTIONAL_TYPE: windows_sys::core::PCWSTR = windows_sys::core::w!("NO");
pub const MGMT_QUEUE_REMOTE_LOCATION: windows_sys::core::PCWSTR = windows_sys::core::w!("REMOTE");
pub const MGMT_QUEUE_STATE_CONNECTED: windows_sys::core::PCWSTR = windows_sys::core::w!("CONNECTED");
pub const MGMT_QUEUE_STATE_DISCONNECTED: windows_sys::core::PCWSTR = windows_sys::core::w!("DISCONNECTED");
pub const MGMT_QUEUE_STATE_DISCONNECTING: windows_sys::core::PCWSTR = windows_sys::core::w!("DISCONNECTING");
pub const MGMT_QUEUE_STATE_LOCAL: windows_sys::core::PCWSTR = windows_sys::core::w!("LOCAL CONNECTION");
pub const MGMT_QUEUE_STATE_LOCKED: windows_sys::core::PCWSTR = windows_sys::core::w!("LOCKED");
pub const MGMT_QUEUE_STATE_NEED_VALIDATE: windows_sys::core::PCWSTR = windows_sys::core::w!("NEED VALIDATION");
pub const MGMT_QUEUE_STATE_NONACTIVE: windows_sys::core::PCWSTR = windows_sys::core::w!("INACTIVE");
pub const MGMT_QUEUE_STATE_ONHOLD: windows_sys::core::PCWSTR = windows_sys::core::w!("ONHOLD");
pub const MGMT_QUEUE_STATE_WAITING: windows_sys::core::PCWSTR = windows_sys::core::w!("WAITING");
pub const MGMT_QUEUE_TRANSACTIONAL_TYPE: windows_sys::core::PCWSTR = windows_sys::core::w!("YES");
pub const MGMT_QUEUE_TYPE_CONNECTOR: windows_sys::core::PCWSTR = windows_sys::core::w!("CONNECTOR");
pub const MGMT_QUEUE_TYPE_MACHINE: windows_sys::core::PCWSTR = windows_sys::core::w!("MACHINE");
pub const MGMT_QUEUE_TYPE_MULTICAST: windows_sys::core::PCWSTR = windows_sys::core::w!("MULTICAST");
pub const MGMT_QUEUE_TYPE_PRIVATE: windows_sys::core::PCWSTR = windows_sys::core::w!("PRIVATE");
pub const MGMT_QUEUE_TYPE_PUBLIC: windows_sys::core::PCWSTR = windows_sys::core::w!("PUBLIC");
pub const MGMT_QUEUE_UNKNOWN_TYPE: windows_sys::core::PCWSTR = windows_sys::core::w!("UNKNOWN");
pub const MO_MACHINE_TOKEN: windows_sys::core::PCWSTR = windows_sys::core::w!("MACHINE");
pub const MO_QUEUE_TOKEN: windows_sys::core::PCWSTR = windows_sys::core::w!("QUEUE");
pub type MQACCESS = i32;
pub type MQAUTHENTICATE = i32;
pub type MQCALG = i32;
pub type MQCERT_REGISTER = i32;
pub const MQCERT_REGISTER_ALWAYS: MQCERT_REGISTER = 1;
pub const MQCERT_REGISTER_IF_NOT_EXIST: MQCERT_REGISTER = 2;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MQCOLUMNSET {
    pub cCol: u32,
    pub aCol: *mut u32,
}
impl Default for MQCOLUMNSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
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
pub type MQDEFAULT = i32;
pub type MQERROR = i32;
pub type MQJOURNAL = i32;
pub type MQMAX = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct MQMGMTPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut super::Com::StructuredStorage::PROPVARIANT,
    pub aStatus: *mut windows_sys::core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for MQMGMTPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MQMSGACKNOWLEDGEMENT = i32;
pub type MQMSGAUTHENTICATION = i32;
pub type MQMSGAUTHLEVEL = i32;
pub type MQMSGCLASS = i32;
pub type MQMSGCURSOR = i32;
pub type MQMSGDELIVERY = i32;
pub type MQMSGIDSIZE = i32;
pub type MQMSGJOURNAL = i32;
pub type MQMSGMAX = i32;
pub type MQMSGPRIVLEVEL = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct MQMSGPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut super::Com::StructuredStorage::PROPVARIANT,
    pub aStatus: *mut windows_sys::core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for MQMSGPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MQMSGSENDERIDTYPE = i32;
pub type MQMSGTRACE = i32;
pub const MQMSG_ACKNOWLEDGMENT_FULL_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = 5;
pub const MQMSG_ACKNOWLEDGMENT_FULL_RECEIVE: MQMSGACKNOWLEDGEMENT = 14;
pub const MQMSG_ACKNOWLEDGMENT_NACK_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = 4;
pub const MQMSG_ACKNOWLEDGMENT_NACK_RECEIVE: MQMSGACKNOWLEDGEMENT = 12;
pub const MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL: MQMSGACKNOWLEDGEMENT = 4;
pub const MQMSG_ACKNOWLEDGMENT_NEG_RECEIVE: MQMSGACKNOWLEDGEMENT = 8;
pub const MQMSG_ACKNOWLEDGMENT_NONE: MQMSGACKNOWLEDGEMENT = 0;
pub const MQMSG_ACKNOWLEDGMENT_POS_ARRIVAL: MQMSGACKNOWLEDGEMENT = 1;
pub const MQMSG_ACKNOWLEDGMENT_POS_RECEIVE: MQMSGACKNOWLEDGEMENT = 2;
pub const MQMSG_AUTHENTICATED_QM_MESSAGE: u32 = 11;
pub const MQMSG_AUTHENTICATED_SIG10: MQMSGAUTHENTICATION = 1;
pub const MQMSG_AUTHENTICATED_SIG20: MQMSGAUTHENTICATION = 3;
pub const MQMSG_AUTHENTICATED_SIG30: MQMSGAUTHENTICATION = 5;
pub const MQMSG_AUTHENTICATED_SIGXML: MQMSGAUTHENTICATION = 9;
pub const MQMSG_AUTHENTICATION_NOT_REQUESTED: MQMSGAUTHENTICATION = 0;
pub const MQMSG_AUTHENTICATION_REQUESTED: MQMSGAUTHENTICATION = 1;
pub const MQMSG_AUTHENTICATION_REQUESTED_EX: MQMSGAUTHENTICATION = 3;
pub const MQMSG_AUTH_LEVEL_ALWAYS: MQMSGAUTHLEVEL = 1;
pub const MQMSG_AUTH_LEVEL_MSMQ10: MQMSGAUTHLEVEL = 2;
pub const MQMSG_AUTH_LEVEL_MSMQ20: MQMSGAUTHLEVEL = 4;
pub const MQMSG_AUTH_LEVEL_NONE: MQMSGAUTHLEVEL = 0;
pub const MQMSG_AUTH_LEVEL_SIG10: MQMSGAUTHLEVEL = 2;
pub const MQMSG_AUTH_LEVEL_SIG20: MQMSGAUTHLEVEL = 4;
pub const MQMSG_AUTH_LEVEL_SIG30: MQMSGAUTHLEVEL = 8;
pub const MQMSG_CALG_DES: MQCALG = 26113;
pub const MQMSG_CALG_DSS_SIGN: MQCALG = 8704;
pub const MQMSG_CALG_MAC: MQCALG = 32773;
pub const MQMSG_CALG_MD2: MQCALG = 32769;
pub const MQMSG_CALG_MD4: MQCALG = 32770;
pub const MQMSG_CALG_MD5: MQCALG = 32771;
pub const MQMSG_CALG_RC2: MQCALG = 26114;
pub const MQMSG_CALG_RC4: MQCALG = 26625;
pub const MQMSG_CALG_RSA_KEYX: MQCALG = 41984;
pub const MQMSG_CALG_RSA_SIGN: MQCALG = 9216;
pub const MQMSG_CALG_SEAL: MQCALG = 26626;
pub const MQMSG_CALG_SHA: MQCALG = 32772;
pub const MQMSG_CALG_SHA1: MQCALG = 32772;
pub const MQMSG_CLASS_ACK_REACH_QUEUE: MQMSGCLASS = 2;
pub const MQMSG_CLASS_ACK_RECEIVE: MQMSGCLASS = 16384;
pub const MQMSG_CLASS_NACK_ACCESS_DENIED: MQMSGCLASS = 32772;
pub const MQMSG_CLASS_NACK_BAD_DST_Q: MQMSGCLASS = 32768;
pub const MQMSG_CLASS_NACK_BAD_ENCRYPTION: MQMSGCLASS = 32775;
pub const MQMSG_CLASS_NACK_BAD_SIGNATURE: MQMSGCLASS = 32774;
pub const MQMSG_CLASS_NACK_COULD_NOT_ENCRYPT: MQMSGCLASS = 32776;
pub const MQMSG_CLASS_NACK_HOP_COUNT_EXCEEDED: MQMSGCLASS = 32773;
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_MSG: MQMSGCLASS = 32778;
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_Q: MQMSGCLASS = 32777;
pub const MQMSG_CLASS_NACK_PURGED: MQMSGCLASS = 32769;
pub const MQMSG_CLASS_NACK_Q_DELETED: MQMSGCLASS = 49152;
pub const MQMSG_CLASS_NACK_Q_EXCEED_QUOTA: MQMSGCLASS = 32771;
pub const MQMSG_CLASS_NACK_Q_PURGED: MQMSGCLASS = 49153;
pub const MQMSG_CLASS_NACK_REACH_QUEUE_TIMEOUT: MQMSGCLASS = 32770;
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT: MQMSGCLASS = 49154;
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT_AT_SENDER: MQMSGCLASS = 49155;
pub const MQMSG_CLASS_NACK_SOURCE_COMPUTER_GUID_CHANGED: MQMSGCLASS = 32780;
pub const MQMSG_CLASS_NACK_UNSUPPORTED_CRYPTO_PROVIDER: MQMSGCLASS = 32779;
pub const MQMSG_CLASS_NORMAL: MQMSGCLASS = 0;
pub const MQMSG_CLASS_REPORT: MQMSGCLASS = 1;
pub const MQMSG_CORRELATIONID_SIZE: MQMSGIDSIZE = 20;
pub const MQMSG_CURRENT: MQMSGCURSOR = 1;
pub const MQMSG_DEADLETTER: MQMSGJOURNAL = 1;
pub const MQMSG_DELIVERY_EXPRESS: MQMSGDELIVERY = 0;
pub const MQMSG_DELIVERY_RECOVERABLE: MQMSGDELIVERY = 1;
pub const MQMSG_FIRST: MQMSGCURSOR = 0;
pub const MQMSG_FIRST_IN_XACT: u32 = 1;
pub const MQMSG_JOURNAL: MQMSGJOURNAL = 2;
pub const MQMSG_JOURNAL_NONE: MQMSGJOURNAL = 0;
pub const MQMSG_LAST_IN_XACT: u32 = 1;
pub const MQMSG_MSGID_SIZE: MQMSGIDSIZE = 20;
pub const MQMSG_NEXT: MQMSGCURSOR = 2;
pub const MQMSG_NOT_FIRST_IN_XACT: u32 = 0;
pub const MQMSG_NOT_LAST_IN_XACT: u32 = 0;
pub const MQMSG_PRIV_LEVEL_BODY_AES: u32 = 5;
pub const MQMSG_PRIV_LEVEL_BODY_BASE: MQMSGPRIVLEVEL = 1;
pub const MQMSG_PRIV_LEVEL_BODY_ENHANCED: MQMSGPRIVLEVEL = 3;
pub const MQMSG_PRIV_LEVEL_NONE: MQMSGPRIVLEVEL = 0;
pub const MQMSG_SENDERID_TYPE_NONE: MQMSGSENDERIDTYPE = 0;
pub const MQMSG_SENDERID_TYPE_SID: MQMSGSENDERIDTYPE = 1;
pub const MQMSG_SEND_ROUTE_TO_REPORT_QUEUE: MQMSGTRACE = 1;
pub const MQMSG_TRACE_NONE: MQMSGTRACE = 0;
pub const MQMSG_XACTID_SIZE: MQMSGIDSIZE = 20;
pub type MQPRIORITY = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct MQPRIVATEPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut super::Com::StructuredStorage::PROPVARIANT,
    pub aStatus: *mut windows_sys::core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for MQPRIVATEPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MQPRIVLEVEL = i32;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct MQPROPERTYRESTRICTION {
    pub rel: u32,
    pub prop: u32,
    pub prval: super::Com::StructuredStorage::PROPVARIANT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for MQPROPERTYRESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct MQQMPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut super::Com::StructuredStorage::PROPVARIANT,
    pub aStatus: *mut windows_sys::core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for MQQMPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MQQUEUEACCESSMASK = u32;
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct MQQUEUEPROPS {
    pub cProp: u32,
    pub aPropID: *mut u32,
    pub aPropVar: *mut super::Com::StructuredStorage::PROPVARIANT,
    pub aStatus: *mut windows_sys::core::HRESULT,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for MQQUEUEPROPS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
#[derive(Clone, Copy)]
pub struct MQRESTRICTION {
    pub cRes: u32,
    pub paPropRes: *mut MQPROPERTYRESTRICTION,
}
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_Variant"))]
impl Default for MQRESTRICTION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MQSEC_CHANGE_QUEUE_PERMISSIONS: MQQUEUEACCESSMASK = 262144;
pub const MQSEC_DELETE_JOURNAL_MESSAGE: MQQUEUEACCESSMASK = 8;
pub const MQSEC_DELETE_MESSAGE: MQQUEUEACCESSMASK = 1;
pub const MQSEC_DELETE_QUEUE: MQQUEUEACCESSMASK = 65536;
pub const MQSEC_GET_QUEUE_PERMISSIONS: MQQUEUEACCESSMASK = 131072;
pub const MQSEC_GET_QUEUE_PROPERTIES: MQQUEUEACCESSMASK = 32;
pub const MQSEC_PEEK_MESSAGE: MQQUEUEACCESSMASK = 2;
pub const MQSEC_QUEUE_GENERIC_ALL: MQQUEUEACCESSMASK = 983103;
pub const MQSEC_QUEUE_GENERIC_EXECUTE: MQQUEUEACCESSMASK = 0;
pub const MQSEC_QUEUE_GENERIC_READ: MQQUEUEACCESSMASK = 131115;
pub const MQSEC_QUEUE_GENERIC_WRITE: MQQUEUEACCESSMASK = 131108;
pub const MQSEC_RECEIVE_JOURNAL_MESSAGE: MQQUEUEACCESSMASK = 10;
pub const MQSEC_RECEIVE_MESSAGE: MQQUEUEACCESSMASK = 3;
pub const MQSEC_SET_QUEUE_PROPERTIES: MQQUEUEACCESSMASK = 16;
pub const MQSEC_TAKE_QUEUE_OWNERSHIP: MQQUEUEACCESSMASK = 524288;
pub const MQSEC_WRITE_MESSAGE: MQQUEUEACCESSMASK = 4;
pub type MQSHARE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct MQSORTKEY {
    pub propColumn: u32,
    pub dwOrder: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MQSORTSET {
    pub cCol: u32,
    pub aCol: *mut MQSORTKEY,
}
impl Default for MQSORTSET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type MQTRANSACTION = i32;
pub type MQTRANSACTIONAL = i32;
pub type MQWARNING = i32;
pub const MQ_ACTION_PEEK_CURRENT: u32 = 2147483648;
pub const MQ_ACTION_PEEK_NEXT: u32 = 2147483649;
pub const MQ_ACTION_RECEIVE: u32 = 0;
pub const MQ_ADMIN_ACCESS: MQACCESS = 128;
pub const MQ_AUTHENTICATE: MQAUTHENTICATE = 1;
pub const MQ_AUTHENTICATE_NONE: MQAUTHENTICATE = 0;
pub const MQ_CORRUPTED_QUEUE_WAS_DELETED: MQERROR = -1072824216;
pub const MQ_DENY_NONE: MQSHARE = 0;
pub const MQ_DENY_RECEIVE_SHARE: MQSHARE = 1;
pub const MQ_ERROR: MQERROR = -1072824319;
pub const MQ_ERROR_ACCESS_DENIED: MQERROR = -1072824283;
pub const MQ_ERROR_BAD_SECURITY_CONTEXT: MQERROR = -1072824267;
pub const MQ_ERROR_BAD_XML_FORMAT: MQERROR = -1072824174;
pub const MQ_ERROR_BUFFER_OVERFLOW: MQERROR = -1072824294;
pub const MQ_ERROR_CANNOT_CREATE_CERT_STORE: MQERROR = -1072824209;
pub const MQ_ERROR_CANNOT_CREATE_HASH_EX: MQERROR = -1072824191;
pub const MQ_ERROR_CANNOT_CREATE_ON_GC: MQERROR = -1072824201;
pub const MQ_ERROR_CANNOT_CREATE_PSC_OBJECTS: MQERROR = -1072824171;
pub const MQ_ERROR_CANNOT_DELETE_PSC_OBJECTS: MQERROR = -1072824189;
pub const MQ_ERROR_CANNOT_GET_DN: MQERROR = -1072824194;
pub const MQ_ERROR_CANNOT_GRANT_ADD_GUID: MQERROR = -1072824206;
pub const MQ_ERROR_CANNOT_HASH_DATA_EX: MQERROR = -1072824193;
pub const MQ_ERROR_CANNOT_IMPERSONATE_CLIENT: MQERROR = -1072824284;
pub const MQ_ERROR_CANNOT_JOIN_DOMAIN: MQERROR = -1072824202;
pub const MQ_ERROR_CANNOT_LOAD_MQAD: MQERROR = -1072824187;
pub const MQ_ERROR_CANNOT_LOAD_MQDSSRV: MQERROR = -1072824186;
pub const MQ_ERROR_CANNOT_LOAD_MSMQOCM: MQERROR = -1072824205;
pub const MQ_ERROR_CANNOT_OPEN_CERT_STORE: MQERROR = -1072824208;
pub const MQ_ERROR_CANNOT_SET_CRYPTO_SEC_DESCR: MQERROR = -1072824212;
pub const MQ_ERROR_CANNOT_SIGN_DATA_EX: MQERROR = -1072824192;
pub const MQ_ERROR_CANNOT_UPDATE_PSC_OBJECTS: MQERROR = -1072824170;
pub const MQ_ERROR_CANT_CREATE_CERT_STORE: MQERROR = -1072824209;
pub const MQ_ERROR_CANT_OPEN_CERT_STORE: MQERROR = -1072824208;
pub const MQ_ERROR_CANT_RESOLVE_SITES: MQERROR = -1072824183;
pub const MQ_ERROR_CERTIFICATE_NOT_PROVIDED: MQERROR = -1072824211;
pub const MQ_ERROR_COMPUTER_DOES_NOT_SUPPORT_ENCRYPTION: MQERROR = -1072824269;
pub const MQ_ERROR_CORRUPTED_INTERNAL_CERTIFICATE: MQERROR = -1072824275;
pub const MQ_ERROR_CORRUPTED_PERSONAL_CERT_STORE: MQERROR = -1072824271;
pub const MQ_ERROR_CORRUPTED_SECURITY_DATA: MQERROR = -1072824272;
pub const MQ_ERROR_COULD_NOT_GET_ACCOUNT_INFO: MQERROR = -1072824265;
pub const MQ_ERROR_COULD_NOT_GET_USER_SID: MQERROR = -1072824266;
pub const MQ_ERROR_DELETE_CN_IN_USE: MQERROR = -1072824248;
pub const MQ_ERROR_DEPEND_WKS_LICENSE_OVERFLOW: MQERROR = -1072824217;
pub const MQ_ERROR_DS_BIND_ROOT_FOREST: MQERROR = -1072824177;
pub const MQ_ERROR_DS_ERROR: MQERROR = -1072824253;
pub const MQ_ERROR_DS_IS_FULL: MQERROR = -1072824254;
pub const MQ_ERROR_DS_LOCAL_USER: MQERROR = -1072824176;
pub const MQ_ERROR_DTC_CONNECT: MQERROR = -1072824244;
pub const MQ_ERROR_ENCRYPTION_PROVIDER_NOT_SUPPORTED: MQERROR = -1072824213;
pub const MQ_ERROR_FAIL_VERIFY_SIGNATURE_EX: MQERROR = -1072824190;
pub const MQ_ERROR_FORMATNAME_BUFFER_TOO_SMALL: MQERROR = -1072824289;
pub const MQ_ERROR_GC_NEEDED: MQERROR = -1072824178;
pub const MQ_ERROR_GUID_NOT_MATCHING: MQERROR = -1072824200;
pub const MQ_ERROR_ILLEGAL_CONTEXT: MQERROR = -1072824229;
pub const MQ_ERROR_ILLEGAL_CURSOR_ACTION: MQERROR = -1072824292;
pub const MQ_ERROR_ILLEGAL_ENTERPRISE_OPERATION: MQERROR = -1072824207;
pub const MQ_ERROR_ILLEGAL_FORMATNAME: MQERROR = -1072824290;
pub const MQ_ERROR_ILLEGAL_MQCOLUMNS: MQERROR = -1072824264;
pub const MQ_ERROR_ILLEGAL_MQPRIVATEPROPS: MQERROR = -1072824197;
pub const MQ_ERROR_ILLEGAL_MQQMPROPS: MQERROR = -1072824255;
pub const MQ_ERROR_ILLEGAL_MQQUEUEPROPS: MQERROR = -1072824259;
pub const MQ_ERROR_ILLEGAL_OPERATION: MQERROR = -1072824220;
pub const MQ_ERROR_ILLEGAL_PROPERTY_SIZE: MQERROR = -1072824261;
pub const MQ_ERROR_ILLEGAL_PROPERTY_VALUE: MQERROR = -1072824296;
pub const MQ_ERROR_ILLEGAL_PROPERTY_VT: MQERROR = -1072824295;
pub const MQ_ERROR_ILLEGAL_PROPID: MQERROR = -1072824263;
pub const MQ_ERROR_ILLEGAL_QUEUE_PATHNAME: MQERROR = -1072824300;
pub const MQ_ERROR_ILLEGAL_RELATION: MQERROR = -1072824262;
pub const MQ_ERROR_ILLEGAL_RESTRICTION_PROPID: MQERROR = -1072824260;
pub const MQ_ERROR_ILLEGAL_SECURITY_DESCRIPTOR: MQERROR = -1072824287;
pub const MQ_ERROR_ILLEGAL_SORT: MQERROR = -1072824304;
pub const MQ_ERROR_ILLEGAL_SORT_PROPID: MQERROR = -1072824228;
pub const MQ_ERROR_ILLEGAL_USER: MQERROR = -1072824303;
pub const MQ_ERROR_INSUFFICIENT_PROPERTIES: MQERROR = -1072824257;
pub const MQ_ERROR_INSUFFICIENT_RESOURCES: MQERROR = -1072824281;
pub const MQ_ERROR_INTERNAL_USER_CERT_EXIST: MQERROR = -1072824274;
pub const MQ_ERROR_INVALID_CERTIFICATE: MQERROR = -1072824276;
pub const MQ_ERROR_INVALID_HANDLE: MQERROR = -1072824313;
pub const MQ_ERROR_INVALID_OWNER: MQERROR = -1072824252;
pub const MQ_ERROR_INVALID_PARAMETER: MQERROR = -1072824314;
pub const MQ_ERROR_IO_TIMEOUT: MQERROR = -1072824293;
pub const MQ_ERROR_LABEL_BUFFER_TOO_SMALL: MQERROR = -1072824226;
pub const MQ_ERROR_LABEL_TOO_LONG: MQERROR = -1072824227;
pub const MQ_ERROR_MACHINE_EXISTS: MQERROR = -1072824256;
pub const MQ_ERROR_MACHINE_NOT_FOUND: MQERROR = -1072824307;
pub const MQ_ERROR_MESSAGE_ALREADY_RECEIVED: MQERROR = -1072824291;
pub const MQ_ERROR_MESSAGE_LOCKED_UNDER_TRANSACTION: windows_sys::core::HRESULT = 0xC00E009C_u32 as _;
pub const MQ_ERROR_MESSAGE_NOT_AUTHENTICATED: windows_sys::core::HRESULT = 0xC00E009B_u32 as _;
pub const MQ_ERROR_MESSAGE_NOT_FOUND: MQERROR = -1072824184;
pub const MQ_ERROR_MESSAGE_STORAGE_FAILED: MQERROR = -1072824278;
pub const MQ_ERROR_MISSING_CONNECTOR_TYPE: MQERROR = -1072824235;
pub const MQ_ERROR_MQIS_READONLY_MODE: MQERROR = -1072824224;
pub const MQ_ERROR_MQIS_SERVER_EMPTY: MQERROR = -1072824225;
pub const MQ_ERROR_MULTI_SORT_KEYS: MQERROR = -1072824179;
pub const MQ_ERROR_NOT_A_CORRECT_OBJECT_CLASS: MQERROR = -1072824180;
pub const MQ_ERROR_NOT_SUPPORTED_BY_DEPENDENT_CLIENTS: MQERROR = -1072824182;
pub const MQ_ERROR_NO_DS: MQERROR = -1072824301;
pub const MQ_ERROR_NO_ENTRY_POINT_MSMQOCM: MQERROR = -1072824204;
pub const MQ_ERROR_NO_GC_IN_DOMAIN: MQERROR = -1072824196;
pub const MQ_ERROR_NO_INTERNAL_USER_CERT: MQERROR = -1072824273;
pub const MQ_ERROR_NO_MQUSER_OU: MQERROR = -1072824188;
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_DC: MQERROR = -1072824203;
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_GC: MQERROR = -1072824195;
pub const MQ_ERROR_NO_RESPONSE_FROM_OBJECT_SERVER: MQERROR = -1072824247;
pub const MQ_ERROR_OBJECT_SERVER_NOT_AVAILABLE: MQERROR = -1072824246;
pub const MQ_ERROR_OPERATION_CANCELLED: MQERROR = -1072824312;
pub const MQ_ERROR_OPERATION_NOT_SUPPORTED_BY_REMOTE_COMPUTER: MQERROR = -1072824181;
pub const MQ_ERROR_PRIVILEGE_NOT_HELD: MQERROR = -1072824282;
pub const MQ_ERROR_PROPERTIES_CONFLICT: MQERROR = -1072824185;
pub const MQ_ERROR_PROPERTY: MQERROR = -1072824318;
pub const MQ_ERROR_PROPERTY_NOTALLOWED: MQERROR = -1072824258;
pub const MQ_ERROR_PROV_NAME_BUFFER_TOO_SMALL: MQERROR = -1072824221;
pub const MQ_ERROR_PUBLIC_KEY_DOES_NOT_EXIST: MQERROR = -1072824198;
pub const MQ_ERROR_PUBLIC_KEY_NOT_FOUND: MQERROR = -1072824199;
pub const MQ_ERROR_QUEUE_DELETED: MQERROR = -1072824230;
pub const MQ_ERROR_QUEUE_EXISTS: MQERROR = -1072824315;
pub const MQ_ERROR_QUEUE_NOT_ACTIVE: MQERROR = -1072824316;
pub const MQ_ERROR_QUEUE_NOT_AVAILABLE: MQERROR = -1072824245;
pub const MQ_ERROR_QUEUE_NOT_FOUND: MQERROR = -1072824317;
pub const MQ_ERROR_Q_ADS_PROPERTY_NOT_SUPPORTED: MQERROR = -1072824175;
pub const MQ_ERROR_Q_DNS_PROPERTY_NOT_SUPPORTED: MQERROR = -1072824210;
pub const MQ_ERROR_REMOTE_MACHINE_NOT_AVAILABLE: MQERROR = -1072824215;
pub const MQ_ERROR_RESOLVE_ADDRESS: windows_sys::core::HRESULT = 0xC00E0099_u32 as _;
pub const MQ_ERROR_RESULT_BUFFER_TOO_SMALL: MQERROR = -1072824250;
pub const MQ_ERROR_SECURITY_DESCRIPTOR_TOO_SMALL: MQERROR = -1072824285;
pub const MQ_ERROR_SENDERID_BUFFER_TOO_SMALL: MQERROR = -1072824286;
pub const MQ_ERROR_SENDER_CERT_BUFFER_TOO_SMALL: MQERROR = -1072824277;
pub const MQ_ERROR_SERVICE_NOT_AVAILABLE: MQERROR = -1072824309;
pub const MQ_ERROR_SHARING_VIOLATION: MQERROR = -1072824311;
pub const MQ_ERROR_SIGNATURE_BUFFER_TOO_SMALL: MQERROR = -1072824222;
pub const MQ_ERROR_STALE_HANDLE: MQERROR = -1072824234;
pub const MQ_ERROR_SYMM_KEY_BUFFER_TOO_SMALL: MQERROR = -1072824223;
pub const MQ_ERROR_TOO_MANY_PROPERTIES: windows_sys::core::HRESULT = 0xC00E009A_u32 as _;
pub const MQ_ERROR_TRANSACTION_ENLIST: MQERROR = -1072824232;
pub const MQ_ERROR_TRANSACTION_IMPORT: MQERROR = -1072824242;
pub const MQ_ERROR_TRANSACTION_SEQUENCE: MQERROR = -1072824239;
pub const MQ_ERROR_TRANSACTION_USAGE: MQERROR = -1072824240;
pub const MQ_ERROR_UNINITIALIZED_OBJECT: MQERROR = -1072824172;
pub const MQ_ERROR_UNSUPPORTED_ACCESS_MODE: MQERROR = -1072824251;
pub const MQ_ERROR_UNSUPPORTED_CLASS: MQERROR = -1072824173;
pub const MQ_ERROR_UNSUPPORTED_FORMATNAME_OPERATION: MQERROR = -1072824288;
pub const MQ_ERROR_UNSUPPORTED_OPERATION: MQERROR = -1072824214;
pub const MQ_ERROR_USER_BUFFER_TOO_SMALL: MQERROR = -1072824280;
pub const MQ_ERROR_WKS_CANT_SERVE_CLIENT: MQERROR = -1072824218;
pub const MQ_ERROR_WRITE_NOT_ALLOWED: MQERROR = -1072824219;
pub const MQ_INFORMATION_DUPLICATE_PROPERTY: MQWARNING = 1074659333;
pub const MQ_INFORMATION_FORMATNAME_BUFFER_TOO_SMALL: MQWARNING = 1074659337;
pub const MQ_INFORMATION_ILLEGAL_PROPERTY: MQWARNING = 1074659330;
pub const MQ_INFORMATION_INTERNAL_USER_CERT_EXIST: MQWARNING = 1074659338;
pub const MQ_INFORMATION_OPERATION_PENDING: MQWARNING = 1074659334;
pub const MQ_INFORMATION_OWNER_IGNORED: MQWARNING = 1074659339;
pub const MQ_INFORMATION_PROPERTY: MQWARNING = 1074659329;
pub const MQ_INFORMATION_PROPERTY_IGNORED: MQWARNING = 1074659331;
pub const MQ_INFORMATION_UNSUPPORTED_PROPERTY: MQWARNING = 1074659332;
pub const MQ_JOURNAL: MQJOURNAL = 1;
pub const MQ_JOURNAL_NONE: MQJOURNAL = 0;
pub const MQ_LOOKUP_PEEK_CURRENT: u32 = 1073741840;
pub const MQ_LOOKUP_PEEK_FIRST: u32 = 1073741844;
pub const MQ_LOOKUP_PEEK_LAST: u32 = 1073741848;
pub const MQ_LOOKUP_PEEK_NEXT: u32 = 1073741841;
pub const MQ_LOOKUP_PEEK_PREV: u32 = 1073741842;
pub const MQ_LOOKUP_RECEIVE_ALLOW_PEEK: u32 = 1073742112;
pub const MQ_LOOKUP_RECEIVE_CURRENT: u32 = 1073741856;
pub const MQ_LOOKUP_RECEIVE_FIRST: u32 = 1073741860;
pub const MQ_LOOKUP_RECEIVE_LAST: u32 = 1073741864;
pub const MQ_LOOKUP_RECEIVE_NEXT: u32 = 1073741857;
pub const MQ_LOOKUP_RECEIVE_PREV: u32 = 1073741858;
pub const MQ_MAX_MSG_LABEL_LEN: MQMSGMAX = 249;
pub const MQ_MAX_PRIORITY: MQPRIORITY = 7;
pub const MQ_MAX_Q_LABEL_LEN: MQMAX = 124;
pub const MQ_MAX_Q_NAME_LEN: MQMAX = 124;
pub const MQ_MIN_PRIORITY: MQPRIORITY = 0;
pub const MQ_MOVE_ACCESS: u32 = 4;
pub const MQ_MTS_TRANSACTION: MQTRANSACTION = 1;
pub const MQ_NO_TRANSACTION: MQTRANSACTION = 0;
pub const MQ_OK: windows_sys::core::HRESULT = 0x0_u32 as _;
pub const MQ_PEEK_ACCESS: MQACCESS = 32;
pub const MQ_PRIV_LEVEL_BODY: MQPRIVLEVEL = 2;
pub const MQ_PRIV_LEVEL_NONE: MQPRIVLEVEL = 0;
pub const MQ_PRIV_LEVEL_OPTIONAL: MQPRIVLEVEL = 1;
pub const MQ_QTYPE_REPORT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x55ee8f32_cce9_11cf_b108_0020afd61ce9);
pub const MQ_QTYPE_TEST: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x55ee8f33_cce9_11cf_b108_0020afd61ce9);
pub const MQ_QUEUE_STATE_CONNECTED: QUEUE_STATE = 6;
pub const MQ_QUEUE_STATE_DISCONNECTED: QUEUE_STATE = 1;
pub const MQ_QUEUE_STATE_DISCONNECTING: QUEUE_STATE = 7;
pub const MQ_QUEUE_STATE_LOCAL_CONNECTION: QUEUE_STATE = 0;
pub const MQ_QUEUE_STATE_LOCKED: QUEUE_STATE = 8;
pub const MQ_QUEUE_STATE_NEEDVALIDATE: QUEUE_STATE = 3;
pub const MQ_QUEUE_STATE_NONACTIVE: QUEUE_STATE = 5;
pub const MQ_QUEUE_STATE_ONHOLD: QUEUE_STATE = 4;
pub const MQ_QUEUE_STATE_WAITING: QUEUE_STATE = 2;
pub const MQ_RECEIVE_ACCESS: MQACCESS = 1;
pub const MQ_SEND_ACCESS: MQACCESS = 2;
pub const MQ_SINGLE_MESSAGE: MQTRANSACTION = 3;
pub const MQ_STATUS_FOREIGN: FOREIGN_STATUS = 0;
pub const MQ_STATUS_NOT_FOREIGN: FOREIGN_STATUS = 1;
pub const MQ_STATUS_UNKNOWN: FOREIGN_STATUS = 2;
pub const MQ_TRANSACTIONAL: MQTRANSACTIONAL = 1;
pub const MQ_TRANSACTIONAL_NONE: MQTRANSACTIONAL = 0;
pub const MQ_TYPE_CONNECTOR: QUEUE_TYPE = 3;
pub const MQ_TYPE_MACHINE: QUEUE_TYPE = 2;
pub const MQ_TYPE_MULTICAST: QUEUE_TYPE = 4;
pub const MQ_TYPE_PRIVATE: QUEUE_TYPE = 1;
pub const MQ_TYPE_PUBLIC: QUEUE_TYPE = 0;
pub const MQ_XACT_STATUS_NOT_XACT: XACT_STATUS = 1;
pub const MQ_XACT_STATUS_UNKNOWN: XACT_STATUS = 2;
pub const MQ_XACT_STATUS_XACT: XACT_STATUS = 0;
pub const MQ_XA_TRANSACTION: MQTRANSACTION = 2;
pub const MSMQApplication: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7d6e086_dccd_11d0_aa4b_0060970debae);
pub const MSMQCollection: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf72b9031_2f0c_43e8_924e_e6052cdc493f);
pub const MSMQCoordinatedTransactionDispenser: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7d6e082_dccd_11d0_aa4b_0060970debae);
pub const MSMQDestination: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xeba96b18_2168_11d3_898c_00e02c074f6b);
pub const MSMQEvent: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7d6e07a_dccd_11d0_aa4b_0060970debae);
pub const MSMQManagement: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x39ce96fe_f4c5_4484_a143_4c2d5d324229);
pub const MSMQMessage: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7d6e075_dccd_11d0_aa4b_0060970debae);
pub const MSMQOutgoingQueueManagement: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0188401c_247a_4fed_99c6_bf14119d7055);
pub const MSMQQuery: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7d6e073_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueue: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7d6e079_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7d6e07c_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueInfos: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7d6e07e_dccd_11d0_aa4b_0060970debae);
pub const MSMQQueueManagement: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x33b6d07e_f27d_42fa_b2d7_bf82e11e9374);
pub const MSMQTransaction: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7d6e080_dccd_11d0_aa4b_0060970debae);
pub const MSMQTransactionDispenser: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xd7d6e084_dccd_11d0_aa4b_0060970debae);
pub const MSMQ_CONNECTED: windows_sys::core::PCWSTR = windows_sys::core::w!("CONNECTED");
pub const MSMQ_DISCONNECTED: windows_sys::core::PCWSTR = windows_sys::core::w!("DISCONNECTED");
#[cfg(all(feature = "Win32_System_Com_StructuredStorage", feature = "Win32_System_IO", feature = "Win32_System_Variant"))]
pub type PMQRECEIVECALLBACK = Option<unsafe extern "system" fn(hrstatus: windows_sys::core::HRESULT, hsource: isize, dwtimeout: u32, dwaction: u32, pmessageprops: *mut MQMSGPROPS, lpoverlapped: *mut super::IO::OVERLAPPED, hcursor: super::super::Foundation::HANDLE)>;
pub const PREQ: u32 = 4;
pub const PRGE: u32 = 3;
pub const PRGT: u32 = 2;
pub const PRLE: u32 = 1;
pub const PRLT: u32 = 0;
pub const PRNE: u32 = 5;
pub const PROPID_MGMT_MSMQ_ACTIVEQUEUES: u32 = 1;
pub const PROPID_MGMT_MSMQ_BASE: u32 = 0;
pub const PROPID_MGMT_MSMQ_BYTES_IN_ALL_QUEUES: u32 = 6;
pub const PROPID_MGMT_MSMQ_CONNECTED: u32 = 4;
pub const PROPID_MGMT_MSMQ_DSSERVER: u32 = 3;
pub const PROPID_MGMT_MSMQ_PRIVATEQ: u32 = 2;
pub const PROPID_MGMT_MSMQ_TYPE: u32 = 5;
pub const PROPID_MGMT_QUEUE_BASE: u32 = 0;
pub const PROPID_MGMT_QUEUE_BYTES_IN_JOURNAL: u32 = 10;
pub const PROPID_MGMT_QUEUE_BYTES_IN_QUEUE: u32 = 8;
pub const PROPID_MGMT_QUEUE_CONNECTION_HISTORY: u32 = 25;
pub const PROPID_MGMT_QUEUE_EOD_FIRST_NON_ACK: u32 = 16;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK: u32 = 13;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_COUNT: u32 = 15;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_TIME: u32 = 14;
pub const PROPID_MGMT_QUEUE_EOD_LAST_NON_ACK: u32 = 17;
pub const PROPID_MGMT_QUEUE_EOD_NEXT_SEQ: u32 = 18;
pub const PROPID_MGMT_QUEUE_EOD_NO_ACK_COUNT: u32 = 20;
pub const PROPID_MGMT_QUEUE_EOD_NO_READ_COUNT: u32 = 19;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_COUNT: u32 = 23;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_INTERVAL: u32 = 22;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_TIME: u32 = 21;
pub const PROPID_MGMT_QUEUE_EOD_SOURCE_INFO: u32 = 24;
pub const PROPID_MGMT_QUEUE_FOREIGN: u32 = 6;
pub const PROPID_MGMT_QUEUE_FORMATNAME: u32 = 2;
pub const PROPID_MGMT_QUEUE_JOURNAL_MESSAGE_COUNT: u32 = 9;
pub const PROPID_MGMT_QUEUE_JOURNAL_USED_QUOTA: u32 = 10;
pub const PROPID_MGMT_QUEUE_LOCATION: u32 = 4;
pub const PROPID_MGMT_QUEUE_MESSAGE_COUNT: u32 = 7;
pub const PROPID_MGMT_QUEUE_NEXTHOPS: u32 = 12;
pub const PROPID_MGMT_QUEUE_PATHNAME: u32 = 1;
pub const PROPID_MGMT_QUEUE_STATE: u32 = 11;
pub const PROPID_MGMT_QUEUE_SUBQUEUE_COUNT: u32 = 26;
pub const PROPID_MGMT_QUEUE_SUBQUEUE_NAMES: u32 = 27;
pub const PROPID_MGMT_QUEUE_TYPE: u32 = 3;
pub const PROPID_MGMT_QUEUE_USED_QUOTA: u32 = 8;
pub const PROPID_MGMT_QUEUE_XACT: u32 = 5;
pub const PROPID_M_ABORT_COUNT: u32 = 69;
pub const PROPID_M_ACKNOWLEDGE: u32 = 6;
pub const PROPID_M_ADMIN_QUEUE: u32 = 17;
pub const PROPID_M_ADMIN_QUEUE_LEN: u32 = 18;
pub const PROPID_M_APPSPECIFIC: u32 = 8;
pub const PROPID_M_ARRIVEDTIME: u32 = 32;
pub const PROPID_M_AUTHENTICATED: u32 = 25;
pub const PROPID_M_AUTHENTICATED_EX: u32 = 53;
pub const PROPID_M_AUTH_LEVEL: u32 = 24;
pub const PROPID_M_BASE: u32 = 0;
pub const PROPID_M_BODY: u32 = 9;
pub const PROPID_M_BODY_SIZE: u32 = 10;
pub const PROPID_M_BODY_TYPE: u32 = 42;
pub const PROPID_M_CLASS: u32 = 1;
pub const PROPID_M_COMPOUND_MESSAGE: u32 = 63;
pub const PROPID_M_COMPOUND_MESSAGE_SIZE: u32 = 64;
pub const PROPID_M_CONNECTOR_TYPE: u32 = 38;
pub const PROPID_M_CORRELATIONID: u32 = 3;
pub const PROPID_M_CORRELATIONID_SIZE: u32 = 20;
pub const PROPID_M_DEADLETTER_QUEUE: u32 = 67;
pub const PROPID_M_DEADLETTER_QUEUE_LEN: u32 = 68;
pub const PROPID_M_DELIVERY: u32 = 5;
pub const PROPID_M_DEST_FORMAT_NAME: u32 = 58;
pub const PROPID_M_DEST_FORMAT_NAME_LEN: u32 = 59;
pub const PROPID_M_DEST_QUEUE: u32 = 33;
pub const PROPID_M_DEST_QUEUE_LEN: u32 = 34;
pub const PROPID_M_DEST_SYMM_KEY: u32 = 43;
pub const PROPID_M_DEST_SYMM_KEY_LEN: u32 = 44;
pub const PROPID_M_ENCRYPTION_ALG: u32 = 27;
pub const PROPID_M_EXTENSION: u32 = 35;
pub const PROPID_M_EXTENSION_LEN: u32 = 36;
pub const PROPID_M_FIRST_IN_XACT: u32 = 50;
pub const PROPID_M_HASH_ALG: u32 = 26;
pub const PROPID_M_JOURNAL: u32 = 7;
pub const PROPID_M_LABEL: u32 = 11;
pub const PROPID_M_LABEL_LEN: u32 = 12;
pub const PROPID_M_LAST_IN_XACT: u32 = 51;
pub const PROPID_M_LAST_MOVE_TIME: u32 = 75;
pub const PROPID_M_LOOKUPID: u32 = 60;
pub const PROPID_M_MOVE_COUNT: u32 = 70;
pub const PROPID_M_MSGID: u32 = 2;
pub const PROPID_M_MSGID_SIZE: u32 = 20;
pub const PROPID_M_PRIORITY: u32 = 4;
pub const PROPID_M_PRIV_LEVEL: u32 = 23;
pub const PROPID_M_PROV_NAME: u32 = 48;
pub const PROPID_M_PROV_NAME_LEN: u32 = 49;
pub const PROPID_M_PROV_TYPE: u32 = 47;
pub const PROPID_M_RESP_FORMAT_NAME: u32 = 54;
pub const PROPID_M_RESP_FORMAT_NAME_LEN: u32 = 55;
pub const PROPID_M_RESP_QUEUE: u32 = 15;
pub const PROPID_M_RESP_QUEUE_LEN: u32 = 16;
pub const PROPID_M_SECURITY_CONTEXT: u32 = 37;
pub const PROPID_M_SENDERID: u32 = 20;
pub const PROPID_M_SENDERID_LEN: u32 = 21;
pub const PROPID_M_SENDERID_TYPE: u32 = 22;
pub const PROPID_M_SENDER_CERT: u32 = 28;
pub const PROPID_M_SENDER_CERT_LEN: u32 = 29;
pub const PROPID_M_SENTTIME: u32 = 31;
pub const PROPID_M_SIGNATURE: u32 = 45;
pub const PROPID_M_SIGNATURE_LEN: u32 = 46;
pub const PROPID_M_SOAP_BODY: u32 = 66;
pub const PROPID_M_SOAP_ENVELOPE: u32 = 61;
pub const PROPID_M_SOAP_ENVELOPE_LEN: u32 = 62;
pub const PROPID_M_SOAP_HEADER: u32 = 65;
pub const PROPID_M_SRC_MACHINE_ID: u32 = 30;
pub const PROPID_M_TIME_TO_BE_RECEIVED: u32 = 14;
pub const PROPID_M_TIME_TO_REACH_QUEUE: u32 = 13;
pub const PROPID_M_TRACE: u32 = 41;
pub const PROPID_M_VERSION: u32 = 19;
pub const PROPID_M_XACTID: u32 = 52;
pub const PROPID_M_XACTID_SIZE: u32 = 20;
pub const PROPID_M_XACT_STATUS_QUEUE: u32 = 39;
pub const PROPID_M_XACT_STATUS_QUEUE_LEN: u32 = 40;
pub const PROPID_PC_BASE: u32 = 5800;
pub const PROPID_PC_DS_ENABLED: u32 = 5802;
pub const PROPID_PC_VERSION: u32 = 5801;
pub const PROPID_QM_BASE: u32 = 200;
pub const PROPID_QM_CONNECTION: u32 = 204;
pub const PROPID_QM_ENCRYPTION_PK: u32 = 205;
pub const PROPID_QM_ENCRYPTION_PK_AES: u32 = 244;
pub const PROPID_QM_ENCRYPTION_PK_BASE: u32 = 231;
pub const PROPID_QM_ENCRYPTION_PK_ENHANCED: u32 = 232;
pub const PROPID_QM_MACHINE_ID: u32 = 202;
pub const PROPID_QM_PATHNAME: u32 = 203;
pub const PROPID_QM_PATHNAME_DNS: u32 = 233;
pub const PROPID_QM_SITE_ID: u32 = 201;
pub const PROPID_Q_ADS_PATH: u32 = 126;
pub const PROPID_Q_AUTHENTICATE: u32 = 111;
pub const PROPID_Q_BASE: u32 = 100;
pub const PROPID_Q_BASEPRIORITY: u32 = 106;
pub const PROPID_Q_CREATE_TIME: u32 = 109;
pub const PROPID_Q_INSTANCE: u32 = 101;
pub const PROPID_Q_JOURNAL: u32 = 104;
pub const PROPID_Q_JOURNAL_QUOTA: u32 = 107;
pub const PROPID_Q_LABEL: u32 = 108;
pub const PROPID_Q_MODIFY_TIME: u32 = 110;
pub const PROPID_Q_MULTICAST_ADDRESS: u32 = 125;
pub const PROPID_Q_PATHNAME: u32 = 103;
pub const PROPID_Q_PATHNAME_DNS: u32 = 124;
pub const PROPID_Q_PRIV_LEVEL: u32 = 112;
pub const PROPID_Q_QUOTA: u32 = 105;
pub const PROPID_Q_TRANSACTION: u32 = 113;
pub const PROPID_Q_TYPE: u32 = 102;
pub const QUERY_SORTASCEND: u32 = 0;
pub const QUERY_SORTDESCEND: u32 = 1;
pub const QUEUE_ACTION_EOD_RESEND: windows_sys::core::PCWSTR = windows_sys::core::w!("EOD_RESEND");
pub const QUEUE_ACTION_PAUSE: windows_sys::core::PCWSTR = windows_sys::core::w!("PAUSE");
pub const QUEUE_ACTION_RESUME: windows_sys::core::PCWSTR = windows_sys::core::w!("RESUME");
pub type QUEUE_STATE = i32;
pub type QUEUE_TYPE = i32;
pub type RELOPS = i32;
pub const REL_EQ: RELOPS = 1;
pub const REL_GE: RELOPS = 6;
pub const REL_GT: RELOPS = 4;
pub const REL_LE: RELOPS = 5;
pub const REL_LT: RELOPS = 3;
pub const REL_NEQ: RELOPS = 2;
pub const REL_NOP: RELOPS = 0;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct SEQUENCE_INFO {
    pub SeqID: i64,
    pub SeqNo: u32,
    pub PrevNo: u32,
}
pub type XACT_STATUS = i32;
