#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {}
#[repr(C)]
pub struct FOREIGN_STATUS(i32);
#[repr(transparent)]
pub struct IMSMQApplication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQApplication2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQApplication3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQCoordinatedTransactionDispenser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQCoordinatedTransactionDispenser2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQCoordinatedTransactionDispenser3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQDestination(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQEvent2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQEvent3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQManagement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQMessage(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQMessage2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQMessage3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQMessage4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQOutgoingQueueManagement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQPrivateDestination(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQPrivateEvent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQuery(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQuery2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQuery3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQuery4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQueue(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQueue2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQueue3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQueue4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQueueInfo(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQueueInfo2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQueueInfo3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQueueInfo4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQueueInfos(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQueueInfos2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQueueInfos3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQueueInfos4(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQQueueManagement(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQTransaction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQTransaction2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQTransaction3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQTransactionDispenser(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQTransactionDispenser2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IMSMQTransactionDispenser3(pub *mut ::core::ffi::c_void);
pub const LONG_LIVED: u32 = 4294967294u32;
#[repr(C)]
pub struct MQACCESS(i32);
#[repr(C)]
pub struct MQAUTHENTICATE(i32);
#[repr(C)]
pub struct MQCALG(i32);
#[repr(C)]
pub struct MQCERT_REGISTER(i32);
#[repr(C)]
pub struct MQDEFAULT(i32);
#[repr(C)]
pub struct MQERROR(i32);
#[repr(C)]
pub struct MQJOURNAL(i32);
#[repr(C)]
pub struct MQMAX(i32);
#[repr(C)]
pub struct MQMSGACKNOWLEDGEMENT(i32);
#[repr(C)]
pub struct MQMSGAUTHENTICATION(i32);
#[repr(C)]
pub struct MQMSGAUTHLEVEL(i32);
#[repr(C)]
pub struct MQMSGCLASS(i32);
#[repr(C)]
pub struct MQMSGCURSOR(i32);
#[repr(C)]
pub struct MQMSGDELIVERY(i32);
#[repr(C)]
pub struct MQMSGIDSIZE(i32);
#[repr(C)]
pub struct MQMSGJOURNAL(i32);
#[repr(C)]
pub struct MQMSGMAX(i32);
#[repr(C)]
pub struct MQMSGPRIVLEVEL(i32);
#[repr(C)]
pub struct MQMSGSENDERIDTYPE(i32);
#[repr(C)]
pub struct MQMSGTRACE(i32);
pub const MQMSG_AUTHENTICATED_QM_MESSAGE: u32 = 11u32;
pub const MQMSG_FIRST_IN_XACT: u32 = 1u32;
pub const MQMSG_LAST_IN_XACT: u32 = 1u32;
pub const MQMSG_NOT_FIRST_IN_XACT: u32 = 0u32;
pub const MQMSG_NOT_LAST_IN_XACT: u32 = 0u32;
pub const MQMSG_PRIV_LEVEL_BODY_AES: u32 = 5u32;
#[repr(C)]
pub struct MQPRIORITY(i32);
#[repr(C)]
pub struct MQPRIVLEVEL(i32);
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
#[repr(C)]
pub struct MQSHARE(i32);
#[repr(C)]
pub struct MQTRANSACTION(i32);
#[repr(C)]
pub struct MQTRANSACTIONAL(i32);
#[repr(C)]
pub struct MQWARNING(i32);
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
#[repr(C)]
pub struct MSMQApplication(i32);
#[repr(C)]
pub struct MSMQCollection(i32);
#[repr(C)]
pub struct MSMQCoordinatedTransactionDispenser(i32);
#[repr(C)]
pub struct MSMQDestination(i32);
#[repr(C)]
pub struct MSMQEvent(i32);
#[repr(C)]
pub struct MSMQManagement(i32);
#[repr(C)]
pub struct MSMQMessage(i32);
#[repr(C)]
pub struct MSMQOutgoingQueueManagement(i32);
#[repr(C)]
pub struct MSMQQuery(i32);
#[repr(C)]
pub struct MSMQQueue(i32);
#[repr(C)]
pub struct MSMQQueueInfo(i32);
#[repr(C)]
pub struct MSMQQueueInfos(i32);
#[repr(C)]
pub struct MSMQQueueManagement(i32);
#[repr(C)]
pub struct MSMQTransaction(i32);
#[repr(C)]
pub struct MSMQTransactionDispenser(i32);
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
#[repr(C)]
pub struct QUEUE_STATE(i32);
#[repr(C)]
pub struct QUEUE_TYPE(i32);
#[repr(C)]
pub struct RELOPS(i32);
#[repr(C)]
pub struct XACT_STATUS(i32);
#[repr(transparent)]
pub struct _DMSMQEventEvents(pub *mut ::core::ffi::c_void);
