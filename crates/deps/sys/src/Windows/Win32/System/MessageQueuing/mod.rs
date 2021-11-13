#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[link(name = "windows")]
extern "system" {}
#[repr(transparent)]
pub struct FOREIGN_STATUS(pub i32);
pub const MQ_STATUS_FOREIGN: FOREIGN_STATUS = FOREIGN_STATUS(0i32);
pub const MQ_STATUS_NOT_FOREIGN: FOREIGN_STATUS = FOREIGN_STATUS(1i32);
pub const MQ_STATUS_UNKNOWN: FOREIGN_STATUS = FOREIGN_STATUS(2i32);
impl ::core::marker::Copy for FOREIGN_STATUS {}
impl ::core::clone::Clone for FOREIGN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQApplication(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQApplication {}
impl ::core::clone::Clone for IMSMQApplication {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQApplication2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQApplication2 {}
impl ::core::clone::Clone for IMSMQApplication2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQApplication3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQApplication3 {}
impl ::core::clone::Clone for IMSMQApplication3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQCollection {}
impl ::core::clone::Clone for IMSMQCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQCoordinatedTransactionDispenser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQCoordinatedTransactionDispenser {}
impl ::core::clone::Clone for IMSMQCoordinatedTransactionDispenser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQCoordinatedTransactionDispenser2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQCoordinatedTransactionDispenser2 {}
impl ::core::clone::Clone for IMSMQCoordinatedTransactionDispenser2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQCoordinatedTransactionDispenser3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQCoordinatedTransactionDispenser3 {}
impl ::core::clone::Clone for IMSMQCoordinatedTransactionDispenser3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQDestination(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQDestination {}
impl ::core::clone::Clone for IMSMQDestination {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQEvent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQEvent {}
impl ::core::clone::Clone for IMSMQEvent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQEvent2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQEvent2 {}
impl ::core::clone::Clone for IMSMQEvent2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQEvent3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQEvent3 {}
impl ::core::clone::Clone for IMSMQEvent3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQManagement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQManagement {}
impl ::core::clone::Clone for IMSMQManagement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQMessage(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQMessage {}
impl ::core::clone::Clone for IMSMQMessage {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQMessage2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQMessage2 {}
impl ::core::clone::Clone for IMSMQMessage2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQMessage3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQMessage3 {}
impl ::core::clone::Clone for IMSMQMessage3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQMessage4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQMessage4 {}
impl ::core::clone::Clone for IMSMQMessage4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQOutgoingQueueManagement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQOutgoingQueueManagement {}
impl ::core::clone::Clone for IMSMQOutgoingQueueManagement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQPrivateDestination(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQPrivateDestination {}
impl ::core::clone::Clone for IMSMQPrivateDestination {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQPrivateEvent(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQPrivateEvent {}
impl ::core::clone::Clone for IMSMQPrivateEvent {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQuery(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQuery {}
impl ::core::clone::Clone for IMSMQQuery {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQuery2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQuery2 {}
impl ::core::clone::Clone for IMSMQQuery2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQuery3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQuery3 {}
impl ::core::clone::Clone for IMSMQQuery3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQuery4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQuery4 {}
impl ::core::clone::Clone for IMSMQQuery4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQueue(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQueue {}
impl ::core::clone::Clone for IMSMQQueue {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQueue2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQueue2 {}
impl ::core::clone::Clone for IMSMQQueue2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQueue3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQueue3 {}
impl ::core::clone::Clone for IMSMQQueue3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQueue4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQueue4 {}
impl ::core::clone::Clone for IMSMQQueue4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQueueInfo(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQueueInfo {}
impl ::core::clone::Clone for IMSMQQueueInfo {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQueueInfo2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQueueInfo2 {}
impl ::core::clone::Clone for IMSMQQueueInfo2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQueueInfo3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQueueInfo3 {}
impl ::core::clone::Clone for IMSMQQueueInfo3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQueueInfo4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQueueInfo4 {}
impl ::core::clone::Clone for IMSMQQueueInfo4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQueueInfos(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQueueInfos {}
impl ::core::clone::Clone for IMSMQQueueInfos {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQueueInfos2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQueueInfos2 {}
impl ::core::clone::Clone for IMSMQQueueInfos2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQueueInfos3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQueueInfos3 {}
impl ::core::clone::Clone for IMSMQQueueInfos3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQueueInfos4(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQueueInfos4 {}
impl ::core::clone::Clone for IMSMQQueueInfos4 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQQueueManagement(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQQueueManagement {}
impl ::core::clone::Clone for IMSMQQueueManagement {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQTransaction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQTransaction {}
impl ::core::clone::Clone for IMSMQTransaction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQTransaction2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQTransaction2 {}
impl ::core::clone::Clone for IMSMQTransaction2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQTransaction3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQTransaction3 {}
impl ::core::clone::Clone for IMSMQTransaction3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQTransactionDispenser(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQTransactionDispenser {}
impl ::core::clone::Clone for IMSMQTransactionDispenser {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQTransactionDispenser2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQTransactionDispenser2 {}
impl ::core::clone::Clone for IMSMQTransactionDispenser2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IMSMQTransactionDispenser3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IMSMQTransactionDispenser3 {}
impl ::core::clone::Clone for IMSMQTransactionDispenser3 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const LONG_LIVED: u32 = 4294967294u32;
#[repr(transparent)]
pub struct MQACCESS(pub i32);
pub const MQ_RECEIVE_ACCESS: MQACCESS = MQACCESS(1i32);
pub const MQ_SEND_ACCESS: MQACCESS = MQACCESS(2i32);
pub const MQ_PEEK_ACCESS: MQACCESS = MQACCESS(32i32);
pub const MQ_ADMIN_ACCESS: MQACCESS = MQACCESS(128i32);
impl ::core::marker::Copy for MQACCESS {}
impl ::core::clone::Clone for MQACCESS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQAUTHENTICATE(pub i32);
pub const MQ_AUTHENTICATE_NONE: MQAUTHENTICATE = MQAUTHENTICATE(0i32);
pub const MQ_AUTHENTICATE: MQAUTHENTICATE = MQAUTHENTICATE(1i32);
impl ::core::marker::Copy for MQAUTHENTICATE {}
impl ::core::clone::Clone for MQAUTHENTICATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQCALG(pub i32);
pub const MQMSG_CALG_MD2: MQCALG = MQCALG(32769i32);
pub const MQMSG_CALG_MD4: MQCALG = MQCALG(32770i32);
pub const MQMSG_CALG_MD5: MQCALG = MQCALG(32771i32);
pub const MQMSG_CALG_SHA: MQCALG = MQCALG(32772i32);
pub const MQMSG_CALG_SHA1: MQCALG = MQCALG(32772i32);
pub const MQMSG_CALG_MAC: MQCALG = MQCALG(32773i32);
pub const MQMSG_CALG_RSA_SIGN: MQCALG = MQCALG(9216i32);
pub const MQMSG_CALG_DSS_SIGN: MQCALG = MQCALG(8704i32);
pub const MQMSG_CALG_RSA_KEYX: MQCALG = MQCALG(41984i32);
pub const MQMSG_CALG_DES: MQCALG = MQCALG(26113i32);
pub const MQMSG_CALG_RC2: MQCALG = MQCALG(26114i32);
pub const MQMSG_CALG_RC4: MQCALG = MQCALG(26625i32);
pub const MQMSG_CALG_SEAL: MQCALG = MQCALG(26626i32);
impl ::core::marker::Copy for MQCALG {}
impl ::core::clone::Clone for MQCALG {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQCERT_REGISTER(pub i32);
pub const MQCERT_REGISTER_ALWAYS: MQCERT_REGISTER = MQCERT_REGISTER(1i32);
pub const MQCERT_REGISTER_IF_NOT_EXIST: MQCERT_REGISTER = MQCERT_REGISTER(2i32);
impl ::core::marker::Copy for MQCERT_REGISTER {}
impl ::core::clone::Clone for MQCERT_REGISTER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQDEFAULT(pub i32);
pub const DEFAULT_M_PRIORITY: MQDEFAULT = MQDEFAULT(3i32);
pub const DEFAULT_M_DELIVERY: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_ACKNOWLEDGE: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_JOURNAL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_APPSPECIFIC: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_PRIV_LEVEL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_AUTH_LEVEL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_M_SENDERID_TYPE: MQDEFAULT = MQDEFAULT(1i32);
pub const DEFAULT_Q_JOURNAL: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_BASEPRIORITY: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_QUOTA: MQDEFAULT = MQDEFAULT(-1i32);
pub const DEFAULT_Q_JOURNAL_QUOTA: MQDEFAULT = MQDEFAULT(-1i32);
pub const DEFAULT_Q_TRANSACTION: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_AUTHENTICATE: MQDEFAULT = MQDEFAULT(0i32);
pub const DEFAULT_Q_PRIV_LEVEL: MQDEFAULT = MQDEFAULT(1i32);
pub const DEFAULT_M_LOOKUPID: MQDEFAULT = MQDEFAULT(0i32);
impl ::core::marker::Copy for MQDEFAULT {}
impl ::core::clone::Clone for MQDEFAULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQERROR(pub i32);
pub const MQ_ERROR: MQERROR = MQERROR(-1072824319i32);
pub const MQ_ERROR_PROPERTY: MQERROR = MQERROR(-1072824318i32);
pub const MQ_ERROR_QUEUE_NOT_FOUND: MQERROR = MQERROR(-1072824317i32);
pub const MQ_ERROR_QUEUE_NOT_ACTIVE: MQERROR = MQERROR(-1072824316i32);
pub const MQ_ERROR_QUEUE_EXISTS: MQERROR = MQERROR(-1072824315i32);
pub const MQ_ERROR_INVALID_PARAMETER: MQERROR = MQERROR(-1072824314i32);
pub const MQ_ERROR_INVALID_HANDLE: MQERROR = MQERROR(-1072824313i32);
pub const MQ_ERROR_OPERATION_CANCELLED: MQERROR = MQERROR(-1072824312i32);
pub const MQ_ERROR_SHARING_VIOLATION: MQERROR = MQERROR(-1072824311i32);
pub const MQ_ERROR_SERVICE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824309i32);
pub const MQ_ERROR_MACHINE_NOT_FOUND: MQERROR = MQERROR(-1072824307i32);
pub const MQ_ERROR_ILLEGAL_SORT: MQERROR = MQERROR(-1072824304i32);
pub const MQ_ERROR_ILLEGAL_USER: MQERROR = MQERROR(-1072824303i32);
pub const MQ_ERROR_NO_DS: MQERROR = MQERROR(-1072824301i32);
pub const MQ_ERROR_ILLEGAL_QUEUE_PATHNAME: MQERROR = MQERROR(-1072824300i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_VALUE: MQERROR = MQERROR(-1072824296i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_VT: MQERROR = MQERROR(-1072824295i32);
pub const MQ_ERROR_BUFFER_OVERFLOW: MQERROR = MQERROR(-1072824294i32);
pub const MQ_ERROR_IO_TIMEOUT: MQERROR = MQERROR(-1072824293i32);
pub const MQ_ERROR_ILLEGAL_CURSOR_ACTION: MQERROR = MQERROR(-1072824292i32);
pub const MQ_ERROR_MESSAGE_ALREADY_RECEIVED: MQERROR = MQERROR(-1072824291i32);
pub const MQ_ERROR_ILLEGAL_FORMATNAME: MQERROR = MQERROR(-1072824290i32);
pub const MQ_ERROR_FORMATNAME_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824289i32);
pub const MQ_ERROR_UNSUPPORTED_FORMATNAME_OPERATION: MQERROR = MQERROR(-1072824288i32);
pub const MQ_ERROR_ILLEGAL_SECURITY_DESCRIPTOR: MQERROR = MQERROR(-1072824287i32);
pub const MQ_ERROR_SENDERID_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824286i32);
pub const MQ_ERROR_SECURITY_DESCRIPTOR_TOO_SMALL: MQERROR = MQERROR(-1072824285i32);
pub const MQ_ERROR_CANNOT_IMPERSONATE_CLIENT: MQERROR = MQERROR(-1072824284i32);
pub const MQ_ERROR_ACCESS_DENIED: MQERROR = MQERROR(-1072824283i32);
pub const MQ_ERROR_PRIVILEGE_NOT_HELD: MQERROR = MQERROR(-1072824282i32);
pub const MQ_ERROR_INSUFFICIENT_RESOURCES: MQERROR = MQERROR(-1072824281i32);
pub const MQ_ERROR_USER_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824280i32);
pub const MQ_ERROR_MESSAGE_STORAGE_FAILED: MQERROR = MQERROR(-1072824278i32);
pub const MQ_ERROR_SENDER_CERT_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824277i32);
pub const MQ_ERROR_INVALID_CERTIFICATE: MQERROR = MQERROR(-1072824276i32);
pub const MQ_ERROR_CORRUPTED_INTERNAL_CERTIFICATE: MQERROR = MQERROR(-1072824275i32);
pub const MQ_ERROR_INTERNAL_USER_CERT_EXIST: MQERROR = MQERROR(-1072824274i32);
pub const MQ_ERROR_NO_INTERNAL_USER_CERT: MQERROR = MQERROR(-1072824273i32);
pub const MQ_ERROR_CORRUPTED_SECURITY_DATA: MQERROR = MQERROR(-1072824272i32);
pub const MQ_ERROR_CORRUPTED_PERSONAL_CERT_STORE: MQERROR = MQERROR(-1072824271i32);
pub const MQ_ERROR_COMPUTER_DOES_NOT_SUPPORT_ENCRYPTION: MQERROR = MQERROR(-1072824269i32);
pub const MQ_ERROR_BAD_SECURITY_CONTEXT: MQERROR = MQERROR(-1072824267i32);
pub const MQ_ERROR_COULD_NOT_GET_USER_SID: MQERROR = MQERROR(-1072824266i32);
pub const MQ_ERROR_COULD_NOT_GET_ACCOUNT_INFO: MQERROR = MQERROR(-1072824265i32);
pub const MQ_ERROR_ILLEGAL_MQCOLUMNS: MQERROR = MQERROR(-1072824264i32);
pub const MQ_ERROR_ILLEGAL_PROPID: MQERROR = MQERROR(-1072824263i32);
pub const MQ_ERROR_ILLEGAL_RELATION: MQERROR = MQERROR(-1072824262i32);
pub const MQ_ERROR_ILLEGAL_PROPERTY_SIZE: MQERROR = MQERROR(-1072824261i32);
pub const MQ_ERROR_ILLEGAL_RESTRICTION_PROPID: MQERROR = MQERROR(-1072824260i32);
pub const MQ_ERROR_ILLEGAL_MQQUEUEPROPS: MQERROR = MQERROR(-1072824259i32);
pub const MQ_ERROR_PROPERTY_NOTALLOWED: MQERROR = MQERROR(-1072824258i32);
pub const MQ_ERROR_INSUFFICIENT_PROPERTIES: MQERROR = MQERROR(-1072824257i32);
pub const MQ_ERROR_MACHINE_EXISTS: MQERROR = MQERROR(-1072824256i32);
pub const MQ_ERROR_ILLEGAL_MQQMPROPS: MQERROR = MQERROR(-1072824255i32);
pub const MQ_ERROR_DS_IS_FULL: MQERROR = MQERROR(-1072824254i32);
pub const MQ_ERROR_DS_ERROR: MQERROR = MQERROR(-1072824253i32);
pub const MQ_ERROR_INVALID_OWNER: MQERROR = MQERROR(-1072824252i32);
pub const MQ_ERROR_UNSUPPORTED_ACCESS_MODE: MQERROR = MQERROR(-1072824251i32);
pub const MQ_ERROR_RESULT_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824250i32);
pub const MQ_ERROR_DELETE_CN_IN_USE: MQERROR = MQERROR(-1072824248i32);
pub const MQ_ERROR_NO_RESPONSE_FROM_OBJECT_SERVER: MQERROR = MQERROR(-1072824247i32);
pub const MQ_ERROR_OBJECT_SERVER_NOT_AVAILABLE: MQERROR = MQERROR(-1072824246i32);
pub const MQ_ERROR_QUEUE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824245i32);
pub const MQ_ERROR_DTC_CONNECT: MQERROR = MQERROR(-1072824244i32);
pub const MQ_ERROR_TRANSACTION_IMPORT: MQERROR = MQERROR(-1072824242i32);
pub const MQ_ERROR_TRANSACTION_USAGE: MQERROR = MQERROR(-1072824240i32);
pub const MQ_ERROR_TRANSACTION_SEQUENCE: MQERROR = MQERROR(-1072824239i32);
pub const MQ_ERROR_MISSING_CONNECTOR_TYPE: MQERROR = MQERROR(-1072824235i32);
pub const MQ_ERROR_STALE_HANDLE: MQERROR = MQERROR(-1072824234i32);
pub const MQ_ERROR_TRANSACTION_ENLIST: MQERROR = MQERROR(-1072824232i32);
pub const MQ_ERROR_QUEUE_DELETED: MQERROR = MQERROR(-1072824230i32);
pub const MQ_ERROR_ILLEGAL_CONTEXT: MQERROR = MQERROR(-1072824229i32);
pub const MQ_ERROR_ILLEGAL_SORT_PROPID: MQERROR = MQERROR(-1072824228i32);
pub const MQ_ERROR_LABEL_TOO_LONG: MQERROR = MQERROR(-1072824227i32);
pub const MQ_ERROR_LABEL_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824226i32);
pub const MQ_ERROR_MQIS_SERVER_EMPTY: MQERROR = MQERROR(-1072824225i32);
pub const MQ_ERROR_MQIS_READONLY_MODE: MQERROR = MQERROR(-1072824224i32);
pub const MQ_ERROR_SYMM_KEY_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824223i32);
pub const MQ_ERROR_SIGNATURE_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824222i32);
pub const MQ_ERROR_PROV_NAME_BUFFER_TOO_SMALL: MQERROR = MQERROR(-1072824221i32);
pub const MQ_ERROR_ILLEGAL_OPERATION: MQERROR = MQERROR(-1072824220i32);
pub const MQ_ERROR_WRITE_NOT_ALLOWED: MQERROR = MQERROR(-1072824219i32);
pub const MQ_ERROR_WKS_CANT_SERVE_CLIENT: MQERROR = MQERROR(-1072824218i32);
pub const MQ_ERROR_DEPEND_WKS_LICENSE_OVERFLOW: MQERROR = MQERROR(-1072824217i32);
pub const MQ_CORRUPTED_QUEUE_WAS_DELETED: MQERROR = MQERROR(-1072824216i32);
pub const MQ_ERROR_REMOTE_MACHINE_NOT_AVAILABLE: MQERROR = MQERROR(-1072824215i32);
pub const MQ_ERROR_UNSUPPORTED_OPERATION: MQERROR = MQERROR(-1072824214i32);
pub const MQ_ERROR_ENCRYPTION_PROVIDER_NOT_SUPPORTED: MQERROR = MQERROR(-1072824213i32);
pub const MQ_ERROR_CANNOT_SET_CRYPTO_SEC_DESCR: MQERROR = MQERROR(-1072824212i32);
pub const MQ_ERROR_CERTIFICATE_NOT_PROVIDED: MQERROR = MQERROR(-1072824211i32);
pub const MQ_ERROR_Q_DNS_PROPERTY_NOT_SUPPORTED: MQERROR = MQERROR(-1072824210i32);
pub const MQ_ERROR_CANT_CREATE_CERT_STORE: MQERROR = MQERROR(-1072824209i32);
pub const MQ_ERROR_CANNOT_CREATE_CERT_STORE: MQERROR = MQERROR(-1072824209i32);
pub const MQ_ERROR_CANT_OPEN_CERT_STORE: MQERROR = MQERROR(-1072824208i32);
pub const MQ_ERROR_CANNOT_OPEN_CERT_STORE: MQERROR = MQERROR(-1072824208i32);
pub const MQ_ERROR_ILLEGAL_ENTERPRISE_OPERATION: MQERROR = MQERROR(-1072824207i32);
pub const MQ_ERROR_CANNOT_GRANT_ADD_GUID: MQERROR = MQERROR(-1072824206i32);
pub const MQ_ERROR_CANNOT_LOAD_MSMQOCM: MQERROR = MQERROR(-1072824205i32);
pub const MQ_ERROR_NO_ENTRY_POINT_MSMQOCM: MQERROR = MQERROR(-1072824204i32);
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_DC: MQERROR = MQERROR(-1072824203i32);
pub const MQ_ERROR_CANNOT_JOIN_DOMAIN: MQERROR = MQERROR(-1072824202i32);
pub const MQ_ERROR_CANNOT_CREATE_ON_GC: MQERROR = MQERROR(-1072824201i32);
pub const MQ_ERROR_GUID_NOT_MATCHING: MQERROR = MQERROR(-1072824200i32);
pub const MQ_ERROR_PUBLIC_KEY_NOT_FOUND: MQERROR = MQERROR(-1072824199i32);
pub const MQ_ERROR_PUBLIC_KEY_DOES_NOT_EXIST: MQERROR = MQERROR(-1072824198i32);
pub const MQ_ERROR_ILLEGAL_MQPRIVATEPROPS: MQERROR = MQERROR(-1072824197i32);
pub const MQ_ERROR_NO_GC_IN_DOMAIN: MQERROR = MQERROR(-1072824196i32);
pub const MQ_ERROR_NO_MSMQ_SERVERS_ON_GC: MQERROR = MQERROR(-1072824195i32);
pub const MQ_ERROR_CANNOT_GET_DN: MQERROR = MQERROR(-1072824194i32);
pub const MQ_ERROR_CANNOT_HASH_DATA_EX: MQERROR = MQERROR(-1072824193i32);
pub const MQ_ERROR_CANNOT_SIGN_DATA_EX: MQERROR = MQERROR(-1072824192i32);
pub const MQ_ERROR_CANNOT_CREATE_HASH_EX: MQERROR = MQERROR(-1072824191i32);
pub const MQ_ERROR_FAIL_VERIFY_SIGNATURE_EX: MQERROR = MQERROR(-1072824190i32);
pub const MQ_ERROR_CANNOT_DELETE_PSC_OBJECTS: MQERROR = MQERROR(-1072824189i32);
pub const MQ_ERROR_NO_MQUSER_OU: MQERROR = MQERROR(-1072824188i32);
pub const MQ_ERROR_CANNOT_LOAD_MQAD: MQERROR = MQERROR(-1072824187i32);
pub const MQ_ERROR_CANNOT_LOAD_MQDSSRV: MQERROR = MQERROR(-1072824186i32);
pub const MQ_ERROR_PROPERTIES_CONFLICT: MQERROR = MQERROR(-1072824185i32);
pub const MQ_ERROR_MESSAGE_NOT_FOUND: MQERROR = MQERROR(-1072824184i32);
pub const MQ_ERROR_CANT_RESOLVE_SITES: MQERROR = MQERROR(-1072824183i32);
pub const MQ_ERROR_NOT_SUPPORTED_BY_DEPENDENT_CLIENTS: MQERROR = MQERROR(-1072824182i32);
pub const MQ_ERROR_OPERATION_NOT_SUPPORTED_BY_REMOTE_COMPUTER: MQERROR = MQERROR(-1072824181i32);
pub const MQ_ERROR_NOT_A_CORRECT_OBJECT_CLASS: MQERROR = MQERROR(-1072824180i32);
pub const MQ_ERROR_MULTI_SORT_KEYS: MQERROR = MQERROR(-1072824179i32);
pub const MQ_ERROR_GC_NEEDED: MQERROR = MQERROR(-1072824178i32);
pub const MQ_ERROR_DS_BIND_ROOT_FOREST: MQERROR = MQERROR(-1072824177i32);
pub const MQ_ERROR_DS_LOCAL_USER: MQERROR = MQERROR(-1072824176i32);
pub const MQ_ERROR_Q_ADS_PROPERTY_NOT_SUPPORTED: MQERROR = MQERROR(-1072824175i32);
pub const MQ_ERROR_BAD_XML_FORMAT: MQERROR = MQERROR(-1072824174i32);
pub const MQ_ERROR_UNSUPPORTED_CLASS: MQERROR = MQERROR(-1072824173i32);
pub const MQ_ERROR_UNINITIALIZED_OBJECT: MQERROR = MQERROR(-1072824172i32);
pub const MQ_ERROR_CANNOT_CREATE_PSC_OBJECTS: MQERROR = MQERROR(-1072824171i32);
pub const MQ_ERROR_CANNOT_UPDATE_PSC_OBJECTS: MQERROR = MQERROR(-1072824170i32);
impl ::core::marker::Copy for MQERROR {}
impl ::core::clone::Clone for MQERROR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQJOURNAL(pub i32);
pub const MQ_JOURNAL_NONE: MQJOURNAL = MQJOURNAL(0i32);
pub const MQ_JOURNAL: MQJOURNAL = MQJOURNAL(1i32);
impl ::core::marker::Copy for MQJOURNAL {}
impl ::core::clone::Clone for MQJOURNAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQMAX(pub i32);
pub const MQ_MAX_Q_NAME_LEN: MQMAX = MQMAX(124i32);
pub const MQ_MAX_Q_LABEL_LEN: MQMAX = MQMAX(124i32);
impl ::core::marker::Copy for MQMAX {}
impl ::core::clone::Clone for MQMAX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQMSGACKNOWLEDGEMENT(pub i32);
pub const MQMSG_ACKNOWLEDGMENT_NONE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(0i32);
pub const MQMSG_ACKNOWLEDGMENT_POS_ARRIVAL: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(1i32);
pub const MQMSG_ACKNOWLEDGMENT_POS_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(2i32);
pub const MQMSG_ACKNOWLEDGMENT_NEG_ARRIVAL: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(4i32);
pub const MQMSG_ACKNOWLEDGMENT_NEG_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(8i32);
pub const MQMSG_ACKNOWLEDGMENT_NACK_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(4i32);
pub const MQMSG_ACKNOWLEDGMENT_FULL_REACH_QUEUE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(5i32);
pub const MQMSG_ACKNOWLEDGMENT_NACK_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(12i32);
pub const MQMSG_ACKNOWLEDGMENT_FULL_RECEIVE: MQMSGACKNOWLEDGEMENT = MQMSGACKNOWLEDGEMENT(14i32);
impl ::core::marker::Copy for MQMSGACKNOWLEDGEMENT {}
impl ::core::clone::Clone for MQMSGACKNOWLEDGEMENT {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQMSGAUTHENTICATION(pub i32);
pub const MQMSG_AUTHENTICATION_NOT_REQUESTED: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(0i32);
pub const MQMSG_AUTHENTICATION_REQUESTED: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(1i32);
pub const MQMSG_AUTHENTICATED_SIG10: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(1i32);
pub const MQMSG_AUTHENTICATION_REQUESTED_EX: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(3i32);
pub const MQMSG_AUTHENTICATED_SIG20: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(3i32);
pub const MQMSG_AUTHENTICATED_SIG30: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(5i32);
pub const MQMSG_AUTHENTICATED_SIGXML: MQMSGAUTHENTICATION = MQMSGAUTHENTICATION(9i32);
impl ::core::marker::Copy for MQMSGAUTHENTICATION {}
impl ::core::clone::Clone for MQMSGAUTHENTICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQMSGAUTHLEVEL(pub i32);
pub const MQMSG_AUTH_LEVEL_NONE: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(0i32);
pub const MQMSG_AUTH_LEVEL_ALWAYS: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(1i32);
pub const MQMSG_AUTH_LEVEL_MSMQ10: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(2i32);
pub const MQMSG_AUTH_LEVEL_SIG10: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(2i32);
pub const MQMSG_AUTH_LEVEL_MSMQ20: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(4i32);
pub const MQMSG_AUTH_LEVEL_SIG20: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(4i32);
pub const MQMSG_AUTH_LEVEL_SIG30: MQMSGAUTHLEVEL = MQMSGAUTHLEVEL(8i32);
impl ::core::marker::Copy for MQMSGAUTHLEVEL {}
impl ::core::clone::Clone for MQMSGAUTHLEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQMSGCLASS(pub i32);
pub const MQMSG_CLASS_NORMAL: MQMSGCLASS = MQMSGCLASS(0i32);
pub const MQMSG_CLASS_REPORT: MQMSGCLASS = MQMSGCLASS(1i32);
pub const MQMSG_CLASS_ACK_REACH_QUEUE: MQMSGCLASS = MQMSGCLASS(2i32);
pub const MQMSG_CLASS_ACK_RECEIVE: MQMSGCLASS = MQMSGCLASS(16384i32);
pub const MQMSG_CLASS_NACK_BAD_DST_Q: MQMSGCLASS = MQMSGCLASS(32768i32);
pub const MQMSG_CLASS_NACK_PURGED: MQMSGCLASS = MQMSGCLASS(32769i32);
pub const MQMSG_CLASS_NACK_REACH_QUEUE_TIMEOUT: MQMSGCLASS = MQMSGCLASS(32770i32);
pub const MQMSG_CLASS_NACK_Q_EXCEED_QUOTA: MQMSGCLASS = MQMSGCLASS(32771i32);
pub const MQMSG_CLASS_NACK_ACCESS_DENIED: MQMSGCLASS = MQMSGCLASS(32772i32);
pub const MQMSG_CLASS_NACK_HOP_COUNT_EXCEEDED: MQMSGCLASS = MQMSGCLASS(32773i32);
pub const MQMSG_CLASS_NACK_BAD_SIGNATURE: MQMSGCLASS = MQMSGCLASS(32774i32);
pub const MQMSG_CLASS_NACK_BAD_ENCRYPTION: MQMSGCLASS = MQMSGCLASS(32775i32);
pub const MQMSG_CLASS_NACK_COULD_NOT_ENCRYPT: MQMSGCLASS = MQMSGCLASS(32776i32);
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_Q: MQMSGCLASS = MQMSGCLASS(32777i32);
pub const MQMSG_CLASS_NACK_NOT_TRANSACTIONAL_MSG: MQMSGCLASS = MQMSGCLASS(32778i32);
pub const MQMSG_CLASS_NACK_UNSUPPORTED_CRYPTO_PROVIDER: MQMSGCLASS = MQMSGCLASS(32779i32);
pub const MQMSG_CLASS_NACK_SOURCE_COMPUTER_GUID_CHANGED: MQMSGCLASS = MQMSGCLASS(32780i32);
pub const MQMSG_CLASS_NACK_Q_DELETED: MQMSGCLASS = MQMSGCLASS(49152i32);
pub const MQMSG_CLASS_NACK_Q_PURGED: MQMSGCLASS = MQMSGCLASS(49153i32);
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT: MQMSGCLASS = MQMSGCLASS(49154i32);
pub const MQMSG_CLASS_NACK_RECEIVE_TIMEOUT_AT_SENDER: MQMSGCLASS = MQMSGCLASS(49155i32);
impl ::core::marker::Copy for MQMSGCLASS {}
impl ::core::clone::Clone for MQMSGCLASS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQMSGCURSOR(pub i32);
pub const MQMSG_FIRST: MQMSGCURSOR = MQMSGCURSOR(0i32);
pub const MQMSG_CURRENT: MQMSGCURSOR = MQMSGCURSOR(1i32);
pub const MQMSG_NEXT: MQMSGCURSOR = MQMSGCURSOR(2i32);
impl ::core::marker::Copy for MQMSGCURSOR {}
impl ::core::clone::Clone for MQMSGCURSOR {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQMSGDELIVERY(pub i32);
pub const MQMSG_DELIVERY_EXPRESS: MQMSGDELIVERY = MQMSGDELIVERY(0i32);
pub const MQMSG_DELIVERY_RECOVERABLE: MQMSGDELIVERY = MQMSGDELIVERY(1i32);
impl ::core::marker::Copy for MQMSGDELIVERY {}
impl ::core::clone::Clone for MQMSGDELIVERY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQMSGIDSIZE(pub i32);
pub const MQMSG_MSGID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
pub const MQMSG_CORRELATIONID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
pub const MQMSG_XACTID_SIZE: MQMSGIDSIZE = MQMSGIDSIZE(20i32);
impl ::core::marker::Copy for MQMSGIDSIZE {}
impl ::core::clone::Clone for MQMSGIDSIZE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQMSGJOURNAL(pub i32);
pub const MQMSG_JOURNAL_NONE: MQMSGJOURNAL = MQMSGJOURNAL(0i32);
pub const MQMSG_DEADLETTER: MQMSGJOURNAL = MQMSGJOURNAL(1i32);
pub const MQMSG_JOURNAL: MQMSGJOURNAL = MQMSGJOURNAL(2i32);
impl ::core::marker::Copy for MQMSGJOURNAL {}
impl ::core::clone::Clone for MQMSGJOURNAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQMSGMAX(pub i32);
pub const MQ_MAX_MSG_LABEL_LEN: MQMSGMAX = MQMSGMAX(249i32);
impl ::core::marker::Copy for MQMSGMAX {}
impl ::core::clone::Clone for MQMSGMAX {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQMSGPRIVLEVEL(pub i32);
pub const MQMSG_PRIV_LEVEL_NONE: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(0i32);
pub const MQMSG_PRIV_LEVEL_BODY_BASE: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(1i32);
pub const MQMSG_PRIV_LEVEL_BODY_ENHANCED: MQMSGPRIVLEVEL = MQMSGPRIVLEVEL(3i32);
impl ::core::marker::Copy for MQMSGPRIVLEVEL {}
impl ::core::clone::Clone for MQMSGPRIVLEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQMSGSENDERIDTYPE(pub i32);
pub const MQMSG_SENDERID_TYPE_NONE: MQMSGSENDERIDTYPE = MQMSGSENDERIDTYPE(0i32);
pub const MQMSG_SENDERID_TYPE_SID: MQMSGSENDERIDTYPE = MQMSGSENDERIDTYPE(1i32);
impl ::core::marker::Copy for MQMSGSENDERIDTYPE {}
impl ::core::clone::Clone for MQMSGSENDERIDTYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQMSGTRACE(pub i32);
pub const MQMSG_TRACE_NONE: MQMSGTRACE = MQMSGTRACE(0i32);
pub const MQMSG_SEND_ROUTE_TO_REPORT_QUEUE: MQMSGTRACE = MQMSGTRACE(1i32);
impl ::core::marker::Copy for MQMSGTRACE {}
impl ::core::clone::Clone for MQMSGTRACE {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MQMSG_AUTHENTICATED_QM_MESSAGE: u32 = 11u32;
pub const MQMSG_FIRST_IN_XACT: u32 = 1u32;
pub const MQMSG_LAST_IN_XACT: u32 = 1u32;
pub const MQMSG_NOT_FIRST_IN_XACT: u32 = 0u32;
pub const MQMSG_NOT_LAST_IN_XACT: u32 = 0u32;
pub const MQMSG_PRIV_LEVEL_BODY_AES: u32 = 5u32;
#[repr(transparent)]
pub struct MQPRIORITY(pub i32);
pub const MQ_MIN_PRIORITY: MQPRIORITY = MQPRIORITY(0i32);
pub const MQ_MAX_PRIORITY: MQPRIORITY = MQPRIORITY(7i32);
impl ::core::marker::Copy for MQPRIORITY {}
impl ::core::clone::Clone for MQPRIORITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQPRIVLEVEL(pub i32);
pub const MQ_PRIV_LEVEL_NONE: MQPRIVLEVEL = MQPRIVLEVEL(0i32);
pub const MQ_PRIV_LEVEL_OPTIONAL: MQPRIVLEVEL = MQPRIVLEVEL(1i32);
pub const MQ_PRIV_LEVEL_BODY: MQPRIVLEVEL = MQPRIVLEVEL(2i32);
impl ::core::marker::Copy for MQPRIVLEVEL {}
impl ::core::clone::Clone for MQPRIVLEVEL {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MQSEC_CHANGE_QUEUE_PERMISSIONS: u32 = 262144u32;
pub const MQSEC_DELETE_JOURNAL_MESSAGE: u32 = 8u32;
pub const MQSEC_DELETE_MESSAGE: u32 = 1u32;
pub const MQSEC_DELETE_QUEUE: u32 = 65536u32;
pub const MQSEC_GET_QUEUE_PROPERTIES: u32 = 32u32;
pub const MQSEC_PEEK_MESSAGE: u32 = 2u32;
pub const MQSEC_QUEUE_GENERIC_EXECUTE: u32 = 0u32;
pub const MQSEC_SET_QUEUE_PROPERTIES: u32 = 16u32;
pub const MQSEC_TAKE_QUEUE_OWNERSHIP: u32 = 524288u32;
pub const MQSEC_WRITE_MESSAGE: u32 = 4u32;
#[repr(transparent)]
pub struct MQSHARE(pub i32);
pub const MQ_DENY_NONE: MQSHARE = MQSHARE(0i32);
pub const MQ_DENY_RECEIVE_SHARE: MQSHARE = MQSHARE(1i32);
impl ::core::marker::Copy for MQSHARE {}
impl ::core::clone::Clone for MQSHARE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQTRANSACTION(pub i32);
pub const MQ_NO_TRANSACTION: MQTRANSACTION = MQTRANSACTION(0i32);
pub const MQ_MTS_TRANSACTION: MQTRANSACTION = MQTRANSACTION(1i32);
pub const MQ_XA_TRANSACTION: MQTRANSACTION = MQTRANSACTION(2i32);
pub const MQ_SINGLE_MESSAGE: MQTRANSACTION = MQTRANSACTION(3i32);
impl ::core::marker::Copy for MQTRANSACTION {}
impl ::core::clone::Clone for MQTRANSACTION {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQTRANSACTIONAL(pub i32);
pub const MQ_TRANSACTIONAL_NONE: MQTRANSACTIONAL = MQTRANSACTIONAL(0i32);
pub const MQ_TRANSACTIONAL: MQTRANSACTIONAL = MQTRANSACTIONAL(1i32);
impl ::core::marker::Copy for MQTRANSACTIONAL {}
impl ::core::clone::Clone for MQTRANSACTIONAL {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct MQWARNING(pub i32);
pub const MQ_INFORMATION_PROPERTY: MQWARNING = MQWARNING(1074659329i32);
pub const MQ_INFORMATION_ILLEGAL_PROPERTY: MQWARNING = MQWARNING(1074659330i32);
pub const MQ_INFORMATION_PROPERTY_IGNORED: MQWARNING = MQWARNING(1074659331i32);
pub const MQ_INFORMATION_UNSUPPORTED_PROPERTY: MQWARNING = MQWARNING(1074659332i32);
pub const MQ_INFORMATION_DUPLICATE_PROPERTY: MQWARNING = MQWARNING(1074659333i32);
pub const MQ_INFORMATION_OPERATION_PENDING: MQWARNING = MQWARNING(1074659334i32);
pub const MQ_INFORMATION_FORMATNAME_BUFFER_TOO_SMALL: MQWARNING = MQWARNING(1074659337i32);
pub const MQ_INFORMATION_INTERNAL_USER_CERT_EXIST: MQWARNING = MQWARNING(1074659338i32);
pub const MQ_INFORMATION_OWNER_IGNORED: MQWARNING = MQWARNING(1074659339i32);
impl ::core::marker::Copy for MQWARNING {}
impl ::core::clone::Clone for MQWARNING {
    fn clone(&self) -> Self {
        *self
    }
}
pub const MQ_ACTION_PEEK_CURRENT: u32 = 2147483648u32;
pub const MQ_ACTION_PEEK_NEXT: u32 = 2147483649u32;
pub const MQ_ACTION_RECEIVE: u32 = 0u32;
pub const MQ_ERROR_MESSAGE_LOCKED_UNDER_TRANSACTION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1072824164i32 as _);
pub const MQ_ERROR_MESSAGE_NOT_AUTHENTICATED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1072824165i32 as _);
pub const MQ_ERROR_RESOLVE_ADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1072824167i32 as _);
pub const MQ_ERROR_TOO_MANY_PROPERTIES: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1072824166i32 as _);
pub const MQ_LOOKUP_PEEK_CURRENT: u32 = 1073741840u32;
pub const MQ_LOOKUP_PEEK_FIRST: u32 = 1073741844u32;
pub const MQ_LOOKUP_PEEK_LAST: u32 = 1073741848u32;
pub const MQ_LOOKUP_PEEK_NEXT: u32 = 1073741841u32;
pub const MQ_LOOKUP_PEEK_PREV: u32 = 1073741842u32;
pub const MQ_LOOKUP_RECEIVE_ALLOW_PEEK: u32 = 1073742112u32;
pub const MQ_LOOKUP_RECEIVE_CURRENT: u32 = 1073741856u32;
pub const MQ_LOOKUP_RECEIVE_FIRST: u32 = 1073741860u32;
pub const MQ_LOOKUP_RECEIVE_LAST: u32 = 1073741864u32;
pub const MQ_LOOKUP_RECEIVE_NEXT: u32 = 1073741857u32;
pub const MQ_LOOKUP_RECEIVE_PREV: u32 = 1073741858u32;
pub const MQ_MOVE_ACCESS: u32 = 4u32;
pub const MQ_OK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(0i32 as _);
pub const MSMQApplication: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183622, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQCollection: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 4146827313, data2: 12044, data3: 17384, data4: [146, 78, 230, 5, 44, 220, 73, 63] };
pub const MSMQCoordinatedTransactionDispenser: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183618, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQDestination: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3953748760, data2: 8552, data3: 4563, data4: [137, 140, 0, 224, 44, 7, 79, 107] };
pub const MSMQEvent: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183610, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQManagement: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 969840382, data2: 62661, data3: 17540, data4: [161, 67, 76, 45, 93, 50, 66, 41] };
pub const MSMQMessage: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183605, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQOutgoingQueueManagement: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 25706524, data2: 9338, data3: 20461, data4: [153, 198, 191, 20, 17, 157, 112, 85] };
pub const MSMQQuery: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183603, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQQueue: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183609, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQQueueInfo: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183612, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQQueueInfos: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183614, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQQueueManagement: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 867618942,
    data2: 62077,
    data3: 17146,
    data4: [178, 215, 191, 130, 225, 30, 147, 116],
};
pub const MSMQTransaction: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183616, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const MSMQTransactionDispenser: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3621183620, data2: 56525, data3: 4560, data4: [170, 75, 0, 96, 151, 13, 235, 174] };
pub const PREQ: u32 = 4u32;
pub const PRGE: u32 = 3u32;
pub const PRGT: u32 = 2u32;
pub const PRLE: u32 = 1u32;
pub const PRLT: u32 = 0u32;
pub const PRNE: u32 = 5u32;
pub const PROPID_MGMT_MSMQ_ACTIVEQUEUES: u32 = 1u32;
pub const PROPID_MGMT_MSMQ_BASE: u32 = 0u32;
pub const PROPID_MGMT_MSMQ_BYTES_IN_ALL_QUEUES: u32 = 6u32;
pub const PROPID_MGMT_MSMQ_CONNECTED: u32 = 4u32;
pub const PROPID_MGMT_MSMQ_DSSERVER: u32 = 3u32;
pub const PROPID_MGMT_MSMQ_PRIVATEQ: u32 = 2u32;
pub const PROPID_MGMT_MSMQ_TYPE: u32 = 5u32;
pub const PROPID_MGMT_QUEUE_BASE: u32 = 0u32;
pub const PROPID_MGMT_QUEUE_BYTES_IN_JOURNAL: u32 = 10u32;
pub const PROPID_MGMT_QUEUE_BYTES_IN_QUEUE: u32 = 8u32;
pub const PROPID_MGMT_QUEUE_CONNECTION_HISTORY: u32 = 25u32;
pub const PROPID_MGMT_QUEUE_EOD_FIRST_NON_ACK: u32 = 16u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK: u32 = 13u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_COUNT: u32 = 15u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_ACK_TIME: u32 = 14u32;
pub const PROPID_MGMT_QUEUE_EOD_LAST_NON_ACK: u32 = 17u32;
pub const PROPID_MGMT_QUEUE_EOD_NEXT_SEQ: u32 = 18u32;
pub const PROPID_MGMT_QUEUE_EOD_NO_ACK_COUNT: u32 = 20u32;
pub const PROPID_MGMT_QUEUE_EOD_NO_READ_COUNT: u32 = 19u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_COUNT: u32 = 23u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_INTERVAL: u32 = 22u32;
pub const PROPID_MGMT_QUEUE_EOD_RESEND_TIME: u32 = 21u32;
pub const PROPID_MGMT_QUEUE_EOD_SOURCE_INFO: u32 = 24u32;
pub const PROPID_MGMT_QUEUE_FOREIGN: u32 = 6u32;
pub const PROPID_MGMT_QUEUE_FORMATNAME: u32 = 2u32;
pub const PROPID_MGMT_QUEUE_JOURNAL_MESSAGE_COUNT: u32 = 9u32;
pub const PROPID_MGMT_QUEUE_JOURNAL_USED_QUOTA: u32 = 10u32;
pub const PROPID_MGMT_QUEUE_LOCATION: u32 = 4u32;
pub const PROPID_MGMT_QUEUE_MESSAGE_COUNT: u32 = 7u32;
pub const PROPID_MGMT_QUEUE_NEXTHOPS: u32 = 12u32;
pub const PROPID_MGMT_QUEUE_PATHNAME: u32 = 1u32;
pub const PROPID_MGMT_QUEUE_STATE: u32 = 11u32;
pub const PROPID_MGMT_QUEUE_SUBQUEUE_COUNT: u32 = 26u32;
pub const PROPID_MGMT_QUEUE_SUBQUEUE_NAMES: u32 = 27u32;
pub const PROPID_MGMT_QUEUE_TYPE: u32 = 3u32;
pub const PROPID_MGMT_QUEUE_USED_QUOTA: u32 = 8u32;
pub const PROPID_MGMT_QUEUE_XACT: u32 = 5u32;
pub const PROPID_M_ABORT_COUNT: u32 = 69u32;
pub const PROPID_M_ACKNOWLEDGE: u32 = 6u32;
pub const PROPID_M_ADMIN_QUEUE: u32 = 17u32;
pub const PROPID_M_ADMIN_QUEUE_LEN: u32 = 18u32;
pub const PROPID_M_APPSPECIFIC: u32 = 8u32;
pub const PROPID_M_ARRIVEDTIME: u32 = 32u32;
pub const PROPID_M_AUTHENTICATED: u32 = 25u32;
pub const PROPID_M_AUTHENTICATED_EX: u32 = 53u32;
pub const PROPID_M_AUTH_LEVEL: u32 = 24u32;
pub const PROPID_M_BASE: u32 = 0u32;
pub const PROPID_M_BODY: u32 = 9u32;
pub const PROPID_M_BODY_SIZE: u32 = 10u32;
pub const PROPID_M_BODY_TYPE: u32 = 42u32;
pub const PROPID_M_CLASS: u32 = 1u32;
pub const PROPID_M_COMPOUND_MESSAGE: u32 = 63u32;
pub const PROPID_M_COMPOUND_MESSAGE_SIZE: u32 = 64u32;
pub const PROPID_M_CONNECTOR_TYPE: u32 = 38u32;
pub const PROPID_M_CORRELATIONID: u32 = 3u32;
pub const PROPID_M_CORRELATIONID_SIZE: u32 = 20u32;
pub const PROPID_M_DEADLETTER_QUEUE: u32 = 67u32;
pub const PROPID_M_DEADLETTER_QUEUE_LEN: u32 = 68u32;
pub const PROPID_M_DELIVERY: u32 = 5u32;
pub const PROPID_M_DEST_FORMAT_NAME: u32 = 58u32;
pub const PROPID_M_DEST_FORMAT_NAME_LEN: u32 = 59u32;
pub const PROPID_M_DEST_QUEUE: u32 = 33u32;
pub const PROPID_M_DEST_QUEUE_LEN: u32 = 34u32;
pub const PROPID_M_DEST_SYMM_KEY: u32 = 43u32;
pub const PROPID_M_DEST_SYMM_KEY_LEN: u32 = 44u32;
pub const PROPID_M_ENCRYPTION_ALG: u32 = 27u32;
pub const PROPID_M_EXTENSION: u32 = 35u32;
pub const PROPID_M_EXTENSION_LEN: u32 = 36u32;
pub const PROPID_M_FIRST_IN_XACT: u32 = 50u32;
pub const PROPID_M_HASH_ALG: u32 = 26u32;
pub const PROPID_M_JOURNAL: u32 = 7u32;
pub const PROPID_M_LABEL: u32 = 11u32;
pub const PROPID_M_LABEL_LEN: u32 = 12u32;
pub const PROPID_M_LAST_IN_XACT: u32 = 51u32;
pub const PROPID_M_LAST_MOVE_TIME: u32 = 75u32;
pub const PROPID_M_LOOKUPID: u32 = 60u32;
pub const PROPID_M_MOVE_COUNT: u32 = 70u32;
pub const PROPID_M_MSGID: u32 = 2u32;
pub const PROPID_M_MSGID_SIZE: u32 = 20u32;
pub const PROPID_M_PRIORITY: u32 = 4u32;
pub const PROPID_M_PRIV_LEVEL: u32 = 23u32;
pub const PROPID_M_PROV_NAME: u32 = 48u32;
pub const PROPID_M_PROV_NAME_LEN: u32 = 49u32;
pub const PROPID_M_PROV_TYPE: u32 = 47u32;
pub const PROPID_M_RESP_FORMAT_NAME: u32 = 54u32;
pub const PROPID_M_RESP_FORMAT_NAME_LEN: u32 = 55u32;
pub const PROPID_M_RESP_QUEUE: u32 = 15u32;
pub const PROPID_M_RESP_QUEUE_LEN: u32 = 16u32;
pub const PROPID_M_SECURITY_CONTEXT: u32 = 37u32;
pub const PROPID_M_SENDERID: u32 = 20u32;
pub const PROPID_M_SENDERID_LEN: u32 = 21u32;
pub const PROPID_M_SENDERID_TYPE: u32 = 22u32;
pub const PROPID_M_SENDER_CERT: u32 = 28u32;
pub const PROPID_M_SENDER_CERT_LEN: u32 = 29u32;
pub const PROPID_M_SENTTIME: u32 = 31u32;
pub const PROPID_M_SIGNATURE: u32 = 45u32;
pub const PROPID_M_SIGNATURE_LEN: u32 = 46u32;
pub const PROPID_M_SOAP_BODY: u32 = 66u32;
pub const PROPID_M_SOAP_ENVELOPE: u32 = 61u32;
pub const PROPID_M_SOAP_ENVELOPE_LEN: u32 = 62u32;
pub const PROPID_M_SOAP_HEADER: u32 = 65u32;
pub const PROPID_M_SRC_MACHINE_ID: u32 = 30u32;
pub const PROPID_M_TIME_TO_BE_RECEIVED: u32 = 14u32;
pub const PROPID_M_TIME_TO_REACH_QUEUE: u32 = 13u32;
pub const PROPID_M_TRACE: u32 = 41u32;
pub const PROPID_M_VERSION: u32 = 19u32;
pub const PROPID_M_XACTID: u32 = 52u32;
pub const PROPID_M_XACTID_SIZE: u32 = 20u32;
pub const PROPID_M_XACT_STATUS_QUEUE: u32 = 39u32;
pub const PROPID_M_XACT_STATUS_QUEUE_LEN: u32 = 40u32;
pub const PROPID_PC_BASE: u32 = 5800u32;
pub const PROPID_PC_DS_ENABLED: u32 = 5802u32;
pub const PROPID_PC_VERSION: u32 = 5801u32;
pub const PROPID_QM_BASE: u32 = 200u32;
pub const PROPID_QM_CONNECTION: u32 = 204u32;
pub const PROPID_QM_ENCRYPTION_PK: u32 = 205u32;
pub const PROPID_QM_ENCRYPTION_PK_AES: u32 = 244u32;
pub const PROPID_QM_ENCRYPTION_PK_BASE: u32 = 231u32;
pub const PROPID_QM_ENCRYPTION_PK_ENHANCED: u32 = 232u32;
pub const PROPID_QM_MACHINE_ID: u32 = 202u32;
pub const PROPID_QM_PATHNAME: u32 = 203u32;
pub const PROPID_QM_PATHNAME_DNS: u32 = 233u32;
pub const PROPID_QM_SITE_ID: u32 = 201u32;
pub const PROPID_Q_ADS_PATH: u32 = 126u32;
pub const PROPID_Q_AUTHENTICATE: u32 = 111u32;
pub const PROPID_Q_BASE: u32 = 100u32;
pub const PROPID_Q_BASEPRIORITY: u32 = 106u32;
pub const PROPID_Q_CREATE_TIME: u32 = 109u32;
pub const PROPID_Q_INSTANCE: u32 = 101u32;
pub const PROPID_Q_JOURNAL: u32 = 104u32;
pub const PROPID_Q_JOURNAL_QUOTA: u32 = 107u32;
pub const PROPID_Q_LABEL: u32 = 108u32;
pub const PROPID_Q_MODIFY_TIME: u32 = 110u32;
pub const PROPID_Q_MULTICAST_ADDRESS: u32 = 125u32;
pub const PROPID_Q_PATHNAME: u32 = 103u32;
pub const PROPID_Q_PATHNAME_DNS: u32 = 124u32;
pub const PROPID_Q_PRIV_LEVEL: u32 = 112u32;
pub const PROPID_Q_QUOTA: u32 = 105u32;
pub const PROPID_Q_TRANSACTION: u32 = 113u32;
pub const PROPID_Q_TYPE: u32 = 102u32;
pub const QUERY_SORTASCEND: u32 = 0u32;
pub const QUERY_SORTDESCEND: u32 = 1u32;
#[repr(transparent)]
pub struct QUEUE_STATE(pub i32);
pub const MQ_QUEUE_STATE_LOCAL_CONNECTION: QUEUE_STATE = QUEUE_STATE(0i32);
pub const MQ_QUEUE_STATE_DISCONNECTED: QUEUE_STATE = QUEUE_STATE(1i32);
pub const MQ_QUEUE_STATE_WAITING: QUEUE_STATE = QUEUE_STATE(2i32);
pub const MQ_QUEUE_STATE_NEEDVALIDATE: QUEUE_STATE = QUEUE_STATE(3i32);
pub const MQ_QUEUE_STATE_ONHOLD: QUEUE_STATE = QUEUE_STATE(4i32);
pub const MQ_QUEUE_STATE_NONACTIVE: QUEUE_STATE = QUEUE_STATE(5i32);
pub const MQ_QUEUE_STATE_CONNECTED: QUEUE_STATE = QUEUE_STATE(6i32);
pub const MQ_QUEUE_STATE_DISCONNECTING: QUEUE_STATE = QUEUE_STATE(7i32);
pub const MQ_QUEUE_STATE_LOCKED: QUEUE_STATE = QUEUE_STATE(8i32);
impl ::core::marker::Copy for QUEUE_STATE {}
impl ::core::clone::Clone for QUEUE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct QUEUE_TYPE(pub i32);
pub const MQ_TYPE_PUBLIC: QUEUE_TYPE = QUEUE_TYPE(0i32);
pub const MQ_TYPE_PRIVATE: QUEUE_TYPE = QUEUE_TYPE(1i32);
pub const MQ_TYPE_MACHINE: QUEUE_TYPE = QUEUE_TYPE(2i32);
pub const MQ_TYPE_CONNECTOR: QUEUE_TYPE = QUEUE_TYPE(3i32);
pub const MQ_TYPE_MULTICAST: QUEUE_TYPE = QUEUE_TYPE(4i32);
impl ::core::marker::Copy for QUEUE_TYPE {}
impl ::core::clone::Clone for QUEUE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct RELOPS(pub i32);
pub const REL_NOP: RELOPS = RELOPS(0i32);
pub const REL_EQ: RELOPS = RELOPS(1i32);
pub const REL_NEQ: RELOPS = RELOPS(2i32);
pub const REL_LT: RELOPS = RELOPS(3i32);
pub const REL_GT: RELOPS = RELOPS(4i32);
pub const REL_LE: RELOPS = RELOPS(5i32);
pub const REL_GE: RELOPS = RELOPS(6i32);
impl ::core::marker::Copy for RELOPS {}
impl ::core::clone::Clone for RELOPS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct XACT_STATUS(pub i32);
pub const MQ_XACT_STATUS_XACT: XACT_STATUS = XACT_STATUS(0i32);
pub const MQ_XACT_STATUS_NOT_XACT: XACT_STATUS = XACT_STATUS(1i32);
pub const MQ_XACT_STATUS_UNKNOWN: XACT_STATUS = XACT_STATUS(2i32);
impl ::core::marker::Copy for XACT_STATUS {}
impl ::core::clone::Clone for XACT_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct _DMSMQEventEvents(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for _DMSMQEventEvents {}
impl ::core::clone::Clone for _DMSMQEventEvents {
    fn clone(&self) -> Self {
        *self
    }
}
