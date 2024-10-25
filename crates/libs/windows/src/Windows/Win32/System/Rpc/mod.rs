pub const DCE_C_ERROR_STRING_LEN: u32 = 256u32;
pub const EEInfoGCCOM: u32 = 11u32;
pub const EEInfoGCFRS: u32 = 12u32;
pub const EEInfoNextRecordsMissing: u32 = 2u32;
pub const EEInfoPreviousRecordsMissing: u32 = 1u32;
pub const EEInfoUseFileTime: u32 = 4u32;
pub const EPT_S_CANT_CREATE: RPC_STATUS = 1899i32;
pub const EPT_S_CANT_PERFORM_OP: RPC_STATUS = 1752i32;
pub const EPT_S_INVALID_ENTRY: RPC_STATUS = 1751i32;
pub const EPT_S_NOT_REGISTERED: RPC_STATUS = 1753i32;
pub const FC_EXPR_CONST32: EXPR_TOKEN = 1i32;
pub const FC_EXPR_CONST64: EXPR_TOKEN = 2i32;
pub const FC_EXPR_END: EXPR_TOKEN = 6i32;
pub const FC_EXPR_ILLEGAL: EXPR_TOKEN = 0i32;
pub const FC_EXPR_NOOP: EXPR_TOKEN = 5i32;
pub const FC_EXPR_OPER: EXPR_TOKEN = 4i32;
pub const FC_EXPR_START: EXPR_TOKEN = 0i32;
pub const FC_EXPR_VAR: EXPR_TOKEN = 3i32;
pub const IDL_CS_IN_PLACE_CONVERT: IDL_CS_CONVERT = 1i32;
pub const IDL_CS_NEW_BUFFER_CONVERT: IDL_CS_CONVERT = 2i32;
pub const IDL_CS_NO_CONVERT: IDL_CS_CONVERT = 0i32;
pub const INVALID_FRAGMENT_ID: u32 = 0u32;
pub const MES_DECODE: MIDL_ES_CODE = 1i32;
pub const MES_DYNAMIC_BUFFER_HANDLE: MIDL_ES_HANDLE_STYLE = 2i32;
pub const MES_ENCODE: MIDL_ES_CODE = 0i32;
pub const MES_ENCODE_NDR64: MIDL_ES_CODE = 2i32;
pub const MES_FIXED_BUFFER_HANDLE: MIDL_ES_HANDLE_STYLE = 1i32;
pub const MES_INCREMENTAL_HANDLE: MIDL_ES_HANDLE_STYLE = 0i32;
pub const MIDL_WINRT_TYPE_SERIALIZATION_INFO_CURRENT_VERSION: i32 = 1i32;
pub const MarshalDirectionMarshal: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = 0i32;
pub const MarshalDirectionUnmarshal: LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION = 1i32;
pub const MaxNumberOfEEInfoParams: u32 = 4u32;
pub const MidlInterceptionInfoVersionOne: i32 = 1i32;
pub const MidlWinrtTypeSerializationInfoVersionOne: i32 = 1i32;
pub const NDR64_FC_AUTO_HANDLE: u32 = 3u32;
pub const NDR64_FC_BIND_GENERIC: u32 = 1u32;
pub const NDR64_FC_BIND_PRIMITIVE: u32 = 2u32;
pub const NDR64_FC_CALLBACK_HANDLE: u32 = 4u32;
pub const NDR64_FC_EXPLICIT_HANDLE: u32 = 0u32;
pub const NDR64_FC_NO_HANDLE: u32 = 5u32;
pub const NDR_CUSTOM_OR_DEFAULT_ALLOCATOR: u32 = 268435456u32;
pub const NDR_DEFAULT_ALLOCATOR: u32 = 536870912u32;
pub const NT351_INTERFACE_SIZE: u32 = 64u32;
pub const PROTOCOL_ADDRESS_CHANGE: RPC_ADDRESS_CHANGE_TYPE = 3i32;
pub const PROTOCOL_LOADED: RPC_ADDRESS_CHANGE_TYPE = 2i32;
pub const PROTOCOL_NOT_LOADED: RPC_ADDRESS_CHANGE_TYPE = 1i32;
pub const PROXY_CALCSIZE: PROXY_PHASE = 0i32;
pub const PROXY_GETBUFFER: PROXY_PHASE = 1i32;
pub const PROXY_MARSHAL: PROXY_PHASE = 2i32;
pub const PROXY_SENDRECEIVE: PROXY_PHASE = 3i32;
pub const PROXY_UNMARSHAL: PROXY_PHASE = 4i32;
pub const RPCFLG_ACCESSIBILITY_BIT1: u32 = 1048576u32;
pub const RPCFLG_ACCESSIBILITY_BIT2: u32 = 2097152u32;
pub const RPCFLG_ACCESS_LOCAL: u32 = 4194304u32;
pub const RPCFLG_ASYNCHRONOUS: u32 = 1073741824u32;
pub const RPCFLG_AUTO_COMPLETE: u32 = 134217728u32;
pub const RPCFLG_HAS_CALLBACK: u32 = 67108864u32;
pub const RPCFLG_HAS_GUARANTEE: u32 = 16u32;
pub const RPCFLG_HAS_MULTI_SYNTAXES: u32 = 33554432u32;
pub const RPCFLG_INPUT_SYNCHRONOUS: u32 = 536870912u32;
pub const RPCFLG_LOCAL_CALL: u32 = 268435456u32;
pub const RPCFLG_MESSAGE: u32 = 16777216u32;
pub const RPCFLG_NDR64_CONTAINS_ARM_LAYOUT: u32 = 67108864u32;
pub const RPCFLG_NON_NDR: u32 = 2147483648u32;
pub const RPCFLG_SENDER_WAITING_FOR_REPLY: u32 = 8388608u32;
pub const RPCFLG_WINRT_REMOTE_ASYNC: u32 = 32u32;
pub const RPCHTTP_RS_ACCESS_1: RPC_HTTP_REDIRECTOR_STAGE = 2i32;
pub const RPCHTTP_RS_ACCESS_2: RPC_HTTP_REDIRECTOR_STAGE = 4i32;
pub const RPCHTTP_RS_INTERFACE: RPC_HTTP_REDIRECTOR_STAGE = 5i32;
pub const RPCHTTP_RS_REDIRECT: RPC_HTTP_REDIRECTOR_STAGE = 1i32;
pub const RPCHTTP_RS_SESSION: RPC_HTTP_REDIRECTOR_STAGE = 3i32;
pub const RPC_BHO_DONTLINGER: RPC_BINDING_HANDLE_OPTIONS_FLAGS = 2u32;
pub const RPC_BHO_EXCLUSIVE_AND_GUARANTEED: u32 = 4u32;
pub const RPC_BHO_NONCAUSAL: RPC_BINDING_HANDLE_OPTIONS_FLAGS = 1u32;
pub const RPC_BHT_OBJECT_UUID_VALID: u32 = 1u32;
pub const RPC_BUFFER_ASYNC: u32 = 32768u32;
pub const RPC_BUFFER_COMPLETE: u32 = 4096u32;
pub const RPC_BUFFER_EXTRA: u32 = 16384u32;
pub const RPC_BUFFER_NONOTIFY: u32 = 65536u32;
pub const RPC_BUFFER_PARTIAL: u32 = 8192u32;
pub const RPC_CALL_ATTRIBUTES_VERSION: u32 = 2u32;
pub const RPC_CALL_STATUS_CANCELLED: u32 = 1u32;
pub const RPC_CALL_STATUS_DISCONNECTED: u32 = 2u32;
pub const RPC_CONTEXT_HANDLE_DEFAULT_FLAGS: u32 = 0u32;
pub const RPC_CONTEXT_HANDLE_DONT_SERIALIZE: u32 = 536870912u32;
pub const RPC_CONTEXT_HANDLE_FLAGS: u32 = 805306368u32;
pub const RPC_CONTEXT_HANDLE_SERIALIZE: u32 = 268435456u32;
pub const RPC_C_AUTHN_CLOUD_AP: u32 = 36u32;
pub const RPC_C_AUTHN_DCE_PRIVATE: u32 = 1u32;
pub const RPC_C_AUTHN_DCE_PUBLIC: u32 = 2u32;
pub const RPC_C_AUTHN_DEC_PUBLIC: u32 = 4u32;
pub const RPC_C_AUTHN_DEFAULT: i32 = -1i32;
pub const RPC_C_AUTHN_DIGEST: u32 = 21u32;
pub const RPC_C_AUTHN_DPA: u32 = 17u32;
pub const RPC_C_AUTHN_GSS_KERBEROS: u32 = 16u32;
pub const RPC_C_AUTHN_GSS_NEGOTIATE: u32 = 9u32;
pub const RPC_C_AUTHN_GSS_SCHANNEL: u32 = 14u32;
pub const RPC_C_AUTHN_INFO_NONE: RPC_C_AUTHN_INFO_TYPE = 0u32;
pub const RPC_C_AUTHN_INFO_TYPE_HTTP: RPC_C_AUTHN_INFO_TYPE = 1u32;
pub const RPC_C_AUTHN_KERNEL: u32 = 20u32;
pub const RPC_C_AUTHN_LIVEXP_SSP: u32 = 35u32;
pub const RPC_C_AUTHN_LIVE_SSP: u32 = 32u32;
pub const RPC_C_AUTHN_MQ: u32 = 100u32;
pub const RPC_C_AUTHN_MSN: u32 = 18u32;
pub const RPC_C_AUTHN_MSONLINE: u32 = 82u32;
pub const RPC_C_AUTHN_NEGO_EXTENDER: u32 = 30u32;
pub const RPC_C_AUTHN_NONE: u32 = 0u32;
pub const RPC_C_AUTHN_PKU2U: u32 = 31u32;
pub const RPC_C_AUTHN_WINNT: u32 = 10u32;
pub const RPC_C_AUTHZ_DCE: u32 = 2u32;
pub const RPC_C_AUTHZ_DEFAULT: u32 = 4294967295u32;
pub const RPC_C_AUTHZ_NAME: u32 = 1u32;
pub const RPC_C_AUTHZ_NONE: u32 = 0u32;
pub const RPC_C_BINDING_DEFAULT_TIMEOUT: u32 = 5u32;
pub const RPC_C_BINDING_INFINITE_TIMEOUT: u32 = 10u32;
pub const RPC_C_BINDING_MAX_TIMEOUT: u32 = 9u32;
pub const RPC_C_BINDING_MIN_TIMEOUT: u32 = 0u32;
pub const RPC_C_BIND_TO_ALL_NICS: u32 = 1u32;
pub const RPC_C_CANCEL_INFINITE_TIMEOUT: i32 = -1i32;
pub const RPC_C_DONT_FAIL: u32 = 4u32;
pub const RPC_C_EP_ALL_ELTS: u32 = 0u32;
pub const RPC_C_EP_MATCH_BY_BOTH: u32 = 3u32;
pub const RPC_C_EP_MATCH_BY_IF: u32 = 1u32;
pub const RPC_C_EP_MATCH_BY_OBJ: u32 = 2u32;
pub const RPC_C_FULL_CERT_CHAIN: u32 = 1u32;
pub const RPC_C_HTTP_AUTHN_SCHEME_BASIC: u32 = 1u32;
pub const RPC_C_HTTP_AUTHN_SCHEME_CERT: u32 = 65536u32;
pub const RPC_C_HTTP_AUTHN_SCHEME_DIGEST: u32 = 8u32;
pub const RPC_C_HTTP_AUTHN_SCHEME_NEGOTIATE: u32 = 16u32;
pub const RPC_C_HTTP_AUTHN_SCHEME_NTLM: u32 = 2u32;
pub const RPC_C_HTTP_AUTHN_SCHEME_PASSPORT: u32 = 4u32;
pub const RPC_C_HTTP_AUTHN_TARGET_PROXY: RPC_C_HTTP_AUTHN_TARGET = 2u32;
pub const RPC_C_HTTP_AUTHN_TARGET_SERVER: RPC_C_HTTP_AUTHN_TARGET = 1u32;
pub const RPC_C_HTTP_FLAG_ENABLE_CERT_REVOCATION_CHECK: RPC_C_HTTP_FLAGS = 16u32;
pub const RPC_C_HTTP_FLAG_IGNORE_CERT_CN_INVALID: RPC_C_HTTP_FLAGS = 8u32;
pub const RPC_C_HTTP_FLAG_USE_FIRST_AUTH_SCHEME: RPC_C_HTTP_FLAGS = 2u32;
pub const RPC_C_HTTP_FLAG_USE_SSL: RPC_C_HTTP_FLAGS = 1u32;
pub const RPC_C_LISTEN_MAX_CALLS_DEFAULT: u32 = 1234u32;
pub const RPC_C_MGMT_INQ_IF_IDS: u32 = 0u32;
pub const RPC_C_MGMT_INQ_PRINC_NAME: u32 = 1u32;
pub const RPC_C_MGMT_INQ_STATS: u32 = 2u32;
pub const RPC_C_MGMT_IS_SERVER_LISTEN: u32 = 3u32;
pub const RPC_C_MGMT_STOP_SERVER_LISTEN: u32 = 4u32;
pub const RPC_C_MQ_AUTHN_LEVEL_NONE: u32 = 0u32;
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_INTEGRITY: u32 = 8u32;
pub const RPC_C_MQ_AUTHN_LEVEL_PKT_PRIVACY: u32 = 16u32;
pub const RPC_C_MQ_CLEAR_ON_OPEN: u32 = 2u32;
pub const RPC_C_MQ_EXPRESS: u32 = 0u32;
pub const RPC_C_MQ_JOURNAL_ALWAYS: u32 = 2u32;
pub const RPC_C_MQ_JOURNAL_DEADLETTER: u32 = 1u32;
pub const RPC_C_MQ_JOURNAL_NONE: u32 = 0u32;
pub const RPC_C_MQ_PERMANENT: u32 = 1u32;
pub const RPC_C_MQ_RECOVERABLE: u32 = 1u32;
pub const RPC_C_MQ_TEMPORARY: u32 = 0u32;
pub const RPC_C_MQ_USE_EXISTING_SECURITY: u32 = 4u32;
pub const RPC_C_NOTIFY_ON_SEND_COMPLETE: u32 = 1u32;
pub const RPC_C_NS_DEFAULT_EXP_AGE: i32 = -1i32;
pub const RPC_C_NS_SYNTAX_DCE: GROUP_NAME_SYNTAX = 3u32;
pub const RPC_C_NS_SYNTAX_DEFAULT: GROUP_NAME_SYNTAX = 0u32;
pub const RPC_C_OPT_ASYNC_BLOCK: u32 = 15u32;
pub const RPC_C_OPT_BINDING_NONCAUSAL: u32 = 9u32;
pub const RPC_C_OPT_CALL_TIMEOUT: u32 = 12u32;
pub const RPC_C_OPT_COOKIE_AUTH: u32 = 7u32;
pub const RPC_C_OPT_DONT_LINGER: u32 = 13u32;
pub const RPC_C_OPT_MAX_OPTIONS: u32 = 12u32;
pub const RPC_C_OPT_MQ_ACKNOWLEDGE: u32 = 4u32;
pub const RPC_C_OPT_MQ_AUTHN_LEVEL: u32 = 6u32;
pub const RPC_C_OPT_MQ_AUTHN_SERVICE: u32 = 5u32;
pub const RPC_C_OPT_MQ_DELIVERY: u32 = 1u32;
pub const RPC_C_OPT_MQ_JOURNAL: u32 = 3u32;
pub const RPC_C_OPT_MQ_PRIORITY: u32 = 2u32;
pub const RPC_C_OPT_MQ_TIME_TO_BE_RECEIVED: u32 = 8u32;
pub const RPC_C_OPT_MQ_TIME_TO_REACH_QUEUE: u32 = 7u32;
pub const RPC_C_OPT_OPTIMIZE_TIME: u32 = 16u32;
pub const RPC_C_OPT_PRIVATE_BREAK_ON_SUSPEND: u32 = 3u32;
pub const RPC_C_OPT_PRIVATE_DO_NOT_DISTURB: u32 = 2u32;
pub const RPC_C_OPT_PRIVATE_SUPPRESS_WAKE: u32 = 1u32;
pub const RPC_C_OPT_RESOURCE_TYPE_UUID: u32 = 8u32;
pub const RPC_C_OPT_SECURITY_CALLBACK: u32 = 10u32;
pub const RPC_C_OPT_SESSION_ID: u32 = 6u32;
pub const RPC_C_OPT_TRANS_SEND_BUFFER_SIZE: u32 = 5u32;
pub const RPC_C_OPT_TRUST_PEER: u32 = 14u32;
pub const RPC_C_OPT_UNIQUE_BINDING: u32 = 11u32;
pub const RPC_C_PARM_BUFFER_LENGTH: u32 = 2u32;
pub const RPC_C_PARM_MAX_PACKET_LENGTH: u32 = 1u32;
pub const RPC_C_PROFILE_ALL_ELT: u32 = 1u32;
pub const RPC_C_PROFILE_ALL_ELTS: u32 = 1u32;
pub const RPC_C_PROFILE_DEFAULT_ELT: u32 = 0u32;
pub const RPC_C_PROFILE_MATCH_BY_BOTH: u32 = 4u32;
pub const RPC_C_PROFILE_MATCH_BY_IF: u32 = 2u32;
pub const RPC_C_PROFILE_MATCH_BY_MBR: u32 = 3u32;
pub const RPC_C_PROTSEQ_MAX_REQS_DEFAULT: u32 = 10u32;
pub const RPC_C_QOS_CAPABILITIES_ANY_AUTHORITY: RPC_C_QOS_CAPABILITIES = 4u32;
pub const RPC_C_QOS_CAPABILITIES_DEFAULT: RPC_C_QOS_CAPABILITIES = 0u32;
pub const RPC_C_QOS_CAPABILITIES_IGNORE_DELEGATE_FAILURE: RPC_C_QOS_CAPABILITIES = 8u32;
pub const RPC_C_QOS_CAPABILITIES_LOCAL_MA_HINT: RPC_C_QOS_CAPABILITIES = 16u32;
pub const RPC_C_QOS_CAPABILITIES_MAKE_FULLSIC: RPC_C_QOS_CAPABILITIES = 2u32;
pub const RPC_C_QOS_CAPABILITIES_MUTUAL_AUTH: RPC_C_QOS_CAPABILITIES = 1u32;
pub const RPC_C_QOS_CAPABILITIES_SCHANNEL_FULL_AUTH_IDENTITY: RPC_C_QOS_CAPABILITIES = 32u32;
pub const RPC_C_QOS_IDENTITY_DYNAMIC: RPC_C_QOS_IDENTITY = 1u32;
pub const RPC_C_QOS_IDENTITY_STATIC: RPC_C_QOS_IDENTITY = 0u32;
pub const RPC_C_RPCHTTP_USE_LOAD_BALANCE: u32 = 8u32;
pub const RPC_C_SECURITY_QOS_VERSION: i32 = 1i32;
pub const RPC_C_SECURITY_QOS_VERSION_1: i32 = 1i32;
pub const RPC_C_SECURITY_QOS_VERSION_2: i32 = 2i32;
pub const RPC_C_SECURITY_QOS_VERSION_3: i32 = 3i32;
pub const RPC_C_SECURITY_QOS_VERSION_4: i32 = 4i32;
pub const RPC_C_SECURITY_QOS_VERSION_5: i32 = 5i32;
pub const RPC_C_STATS_CALLS_IN: u32 = 0u32;
pub const RPC_C_STATS_CALLS_OUT: u32 = 1u32;
pub const RPC_C_STATS_PKTS_IN: u32 = 2u32;
pub const RPC_C_STATS_PKTS_OUT: u32 = 3u32;
pub const RPC_C_TRY_ENFORCE_MAX_CALLS: u32 = 16u32;
pub const RPC_C_USE_INTERNET_PORT: u32 = 1u32;
pub const RPC_C_USE_INTRANET_PORT: u32 = 2u32;
pub const RPC_C_VERS_ALL: u32 = 1u32;
pub const RPC_C_VERS_COMPATIBLE: u32 = 2u32;
pub const RPC_C_VERS_EXACT: u32 = 3u32;
pub const RPC_C_VERS_MAJOR_ONLY: u32 = 4u32;
pub const RPC_C_VERS_UPTO: u32 = 5u32;
pub const RPC_EEINFO_VERSION: u32 = 1u32;
pub const RPC_FLAGS_VALID_BIT: u32 = 32768u32;
pub const RPC_FW_IF_FLAG_DCOM: u32 = 1u32;
pub const RPC_IF_ALLOW_CALLBACKS_WITH_NO_AUTH: u32 = 16u32;
pub const RPC_IF_ALLOW_LOCAL_ONLY: u32 = 32u32;
pub const RPC_IF_ALLOW_SECURE_ONLY: u32 = 8u32;
pub const RPC_IF_ALLOW_UNKNOWN_AUTHORITY: u32 = 4u32;
pub const RPC_IF_ASYNC_CALLBACK: u32 = 256u32;
pub const RPC_IF_AUTOLISTEN: u32 = 1u32;
pub const RPC_IF_OLE: u32 = 2u32;
pub const RPC_IF_SEC_CACHE_PER_PROC: u32 = 128u32;
pub const RPC_IF_SEC_NO_CACHE: u32 = 64u32;
pub const RPC_INTERFACE_HAS_PIPES: u32 = 1u32;
pub const RPC_NCA_FLAGS_BROADCAST: u32 = 2u32;
pub const RPC_NCA_FLAGS_DEFAULT: u32 = 0u32;
pub const RPC_NCA_FLAGS_IDEMPOTENT: u32 = 1u32;
pub const RPC_NCA_FLAGS_MAYBE: u32 = 4u32;
pub const RPC_PROTSEQ_HTTP: u32 = 4u32;
pub const RPC_PROTSEQ_LRPC: u32 = 3u32;
pub const RPC_PROTSEQ_NMP: u32 = 2u32;
pub const RPC_PROTSEQ_TCP: u32 = 1u32;
pub const RPC_PROXY_CONNECTION_TYPE_IN_PROXY: u32 = 0u32;
pub const RPC_PROXY_CONNECTION_TYPE_OUT_PROXY: u32 = 1u32;
pub const RPC_P_ADDR_FORMAT_TCP_IPV4: u32 = 1u32;
pub const RPC_P_ADDR_FORMAT_TCP_IPV6: u32 = 2u32;
pub const RPC_QUERY_CALL_LOCAL_ADDRESS: u32 = 8u32;
pub const RPC_QUERY_CLIENT_ID: u32 = 128u32;
pub const RPC_QUERY_CLIENT_PID: u32 = 16u32;
pub const RPC_QUERY_CLIENT_PRINCIPAL_NAME: u32 = 4u32;
pub const RPC_QUERY_IS_CLIENT_LOCAL: u32 = 32u32;
pub const RPC_QUERY_NO_AUTH_REQUIRED: u32 = 64u32;
pub const RPC_QUERY_SERVER_PRINCIPAL_NAME: u32 = 2u32;
pub const RPC_SYSTEM_HANDLE_FREE_ALL: u32 = 3u32;
pub const RPC_SYSTEM_HANDLE_FREE_ERROR_ON_CLOSE: u32 = 4u32;
pub const RPC_SYSTEM_HANDLE_FREE_RETRIEVED: u32 = 2u32;
pub const RPC_SYSTEM_HANDLE_FREE_UNRETRIEVED: u32 = 1u32;
pub const RPC_S_ACCESS_DENIED: RPC_STATUS = 5i32;
pub const RPC_S_ADDRESS_ERROR: RPC_STATUS = 1768i32;
pub const RPC_S_ALREADY_LISTENING: RPC_STATUS = 1713i32;
pub const RPC_S_ALREADY_REGISTERED: RPC_STATUS = 1711i32;
pub const RPC_S_ASYNC_CALL_PENDING: RPC_STATUS = 997i32;
pub const RPC_S_BINDING_HAS_NO_AUTH: RPC_STATUS = 1746i32;
pub const RPC_S_BINDING_INCOMPLETE: RPC_STATUS = 1819i32;
pub const RPC_S_BUFFER_TOO_SMALL: RPC_STATUS = 122i32;
pub const RPC_S_CALL_CANCELLED: RPC_STATUS = 1818i32;
pub const RPC_S_CALL_FAILED: RPC_STATUS = 1726i32;
pub const RPC_S_CALL_FAILED_DNE: RPC_STATUS = 1727i32;
pub const RPC_S_CALL_IN_PROGRESS: RPC_STATUS = 1791i32;
pub const RPC_S_CANNOT_SUPPORT: RPC_STATUS = 1764i32;
pub const RPC_S_CANT_CREATE_ENDPOINT: RPC_STATUS = 1720i32;
pub const RPC_S_COMM_FAILURE: RPC_STATUS = 1820i32;
pub const RPC_S_COOKIE_AUTH_FAILED: RPC_STATUS = 1833i32;
pub const RPC_S_DO_NOT_DISTURB: RPC_STATUS = 1834i32;
pub const RPC_S_DUPLICATE_ENDPOINT: RPC_STATUS = 1740i32;
pub const RPC_S_ENTRY_ALREADY_EXISTS: RPC_STATUS = 1760i32;
pub const RPC_S_ENTRY_NOT_FOUND: RPC_STATUS = 1761i32;
pub const RPC_S_ENTRY_TYPE_MISMATCH: RPC_STATUS = 1922i32;
pub const RPC_S_FP_DIV_ZERO: RPC_STATUS = 1769i32;
pub const RPC_S_FP_OVERFLOW: RPC_STATUS = 1771i32;
pub const RPC_S_FP_UNDERFLOW: RPC_STATUS = 1770i32;
pub const RPC_S_GROUP_MEMBER_NOT_FOUND: RPC_STATUS = 1898i32;
pub const RPC_S_GRP_ELT_NOT_ADDED: RPC_STATUS = 1928i32;
pub const RPC_S_GRP_ELT_NOT_REMOVED: RPC_STATUS = 1929i32;
pub const RPC_S_INCOMPLETE_NAME: RPC_STATUS = 1755i32;
pub const RPC_S_INTERFACE_NOT_EXPORTED: RPC_STATUS = 1924i32;
pub const RPC_S_INTERFACE_NOT_FOUND: RPC_STATUS = 1759i32;
pub const RPC_S_INTERNAL_ERROR: RPC_STATUS = 1766i32;
pub const RPC_S_INVALID_ARG: RPC_STATUS = 87i32;
pub const RPC_S_INVALID_ASYNC_CALL: RPC_STATUS = 1915i32;
pub const RPC_S_INVALID_ASYNC_HANDLE: RPC_STATUS = 1914i32;
pub const RPC_S_INVALID_AUTH_IDENTITY: RPC_STATUS = 1749i32;
pub const RPC_S_INVALID_BINDING: RPC_STATUS = 1702i32;
pub const RPC_S_INVALID_BOUND: RPC_STATUS = 1734i32;
pub const RPC_S_INVALID_ENDPOINT_FORMAT: RPC_STATUS = 1706i32;
pub const RPC_S_INVALID_LEVEL: RPC_STATUS = 87i32;
pub const RPC_S_INVALID_NAF_ID: RPC_STATUS = 1763i32;
pub const RPC_S_INVALID_NAME_SYNTAX: RPC_STATUS = 1736i32;
pub const RPC_S_INVALID_NETWORK_OPTIONS: RPC_STATUS = 1724i32;
pub const RPC_S_INVALID_NET_ADDR: RPC_STATUS = 1707i32;
pub const RPC_S_INVALID_OBJECT: RPC_STATUS = 1900i32;
pub const RPC_S_INVALID_RPC_PROTSEQ: RPC_STATUS = 1704i32;
pub const RPC_S_INVALID_SECURITY_DESC: RPC_STATUS = 1338i32;
pub const RPC_S_INVALID_STRING_BINDING: RPC_STATUS = 1700i32;
pub const RPC_S_INVALID_STRING_UUID: RPC_STATUS = 1705i32;
pub const RPC_S_INVALID_TAG: RPC_STATUS = 1733i32;
pub const RPC_S_INVALID_TIMEOUT: RPC_STATUS = 1709i32;
pub const RPC_S_INVALID_VERS_OPTION: RPC_STATUS = 1756i32;
pub const RPC_S_MAX_CALLS_TOO_SMALL: RPC_STATUS = 1742i32;
pub const RPC_S_NAME_SERVICE_UNAVAILABLE: RPC_STATUS = 1762i32;
pub const RPC_S_NOTHING_TO_EXPORT: RPC_STATUS = 1754i32;
pub const RPC_S_NOT_ALL_OBJS_EXPORTED: RPC_STATUS = 1923i32;
pub const RPC_S_NOT_ALL_OBJS_UNEXPORTED: RPC_STATUS = 1758i32;
pub const RPC_S_NOT_CANCELLED: RPC_STATUS = 1826i32;
pub const RPC_S_NOT_ENOUGH_QUOTA: RPC_STATUS = 1816i32;
pub const RPC_S_NOT_LISTENING: RPC_STATUS = 1715i32;
pub const RPC_S_NOT_RPC_ERROR: RPC_STATUS = 1823i32;
pub const RPC_S_NO_BINDINGS: RPC_STATUS = 1718i32;
pub const RPC_S_NO_CALL_ACTIVE: RPC_STATUS = 1725i32;
pub const RPC_S_NO_CONTEXT_AVAILABLE: RPC_STATUS = 1765i32;
pub const RPC_S_NO_ENDPOINT_FOUND: RPC_STATUS = 1708i32;
pub const RPC_S_NO_ENTRY_NAME: RPC_STATUS = 1735i32;
pub const RPC_S_NO_INTERFACES: RPC_STATUS = 1817i32;
pub const RPC_S_NO_MORE_BINDINGS: RPC_STATUS = 1806i32;
pub const RPC_S_NO_MORE_MEMBERS: RPC_STATUS = 1757i32;
pub const RPC_S_NO_PRINC_NAME: RPC_STATUS = 1822i32;
pub const RPC_S_NO_PROTSEQS: RPC_STATUS = 1719i32;
pub const RPC_S_NO_PROTSEQS_REGISTERED: RPC_STATUS = 1714i32;
pub const RPC_S_OBJECT_NOT_FOUND: RPC_STATUS = 1710i32;
pub const RPC_S_OK: RPC_STATUS = 0i32;
pub const RPC_S_OUT_OF_MEMORY: RPC_STATUS = 14i32;
pub const RPC_S_OUT_OF_RESOURCES: RPC_STATUS = 1721i32;
pub const RPC_S_OUT_OF_THREADS: RPC_STATUS = 164i32;
pub const RPC_S_PRF_ELT_NOT_ADDED: RPC_STATUS = 1926i32;
pub const RPC_S_PRF_ELT_NOT_REMOVED: RPC_STATUS = 1927i32;
pub const RPC_S_PROCNUM_OUT_OF_RANGE: RPC_STATUS = 1745i32;
pub const RPC_S_PROFILE_NOT_ADDED: RPC_STATUS = 1925i32;
pub const RPC_S_PROTOCOL_ERROR: RPC_STATUS = 1728i32;
pub const RPC_S_PROTSEQ_NOT_FOUND: RPC_STATUS = 1744i32;
pub const RPC_S_PROTSEQ_NOT_SUPPORTED: RPC_STATUS = 1703i32;
pub const RPC_S_PROXY_ACCESS_DENIED: RPC_STATUS = 1729i32;
pub const RPC_S_RUNTIME_UNINITIALIZED: RPC_STATUS = 1i32;
pub const RPC_S_SEC_PKG_ERROR: RPC_STATUS = 1825i32;
pub const RPC_S_SEND_INCOMPLETE: RPC_STATUS = 1913i32;
pub const RPC_S_SERVER_OUT_OF_MEMORY: RPC_STATUS = 1130i32;
pub const RPC_S_SERVER_TOO_BUSY: RPC_STATUS = 1723i32;
pub const RPC_S_SERVER_UNAVAILABLE: RPC_STATUS = 1722i32;
pub const RPC_S_STRING_TOO_LONG: RPC_STATUS = 1743i32;
pub const RPC_S_SYSTEM_HANDLE_COUNT_EXCEEDED: RPC_STATUS = 1835i32;
pub const RPC_S_SYSTEM_HANDLE_TYPE_MISMATCH: RPC_STATUS = 1836i32;
pub const RPC_S_TIMEOUT: RPC_STATUS = 1460i32;
pub const RPC_S_TYPE_ALREADY_REGISTERED: RPC_STATUS = 1712i32;
pub const RPC_S_UNKNOWN_AUTHN_LEVEL: RPC_STATUS = 1748i32;
pub const RPC_S_UNKNOWN_AUTHN_SERVICE: RPC_STATUS = 1747i32;
pub const RPC_S_UNKNOWN_AUTHN_TYPE: RPC_STATUS = 1741i32;
pub const RPC_S_UNKNOWN_AUTHZ_SERVICE: RPC_STATUS = 1750i32;
pub const RPC_S_UNKNOWN_IF: RPC_STATUS = 1717i32;
pub const RPC_S_UNKNOWN_MGR_TYPE: RPC_STATUS = 1716i32;
pub const RPC_S_UNKNOWN_PRINCIPAL: RPC_STATUS = 1332i32;
pub const RPC_S_UNSUPPORTED_AUTHN_LEVEL: RPC_STATUS = 1821i32;
pub const RPC_S_UNSUPPORTED_NAME_SYNTAX: RPC_STATUS = 1737i32;
pub const RPC_S_UNSUPPORTED_TRANS_SYN: RPC_STATUS = 1730i32;
pub const RPC_S_UNSUPPORTED_TYPE: RPC_STATUS = 1732i32;
pub const RPC_S_UUID_LOCAL_ONLY: RPC_STATUS = 1824i32;
pub const RPC_S_UUID_NO_ADDRESS: RPC_STATUS = 1739i32;
pub const RPC_S_WRONG_KIND_OF_BINDING: RPC_STATUS = 1701i32;
pub const RPC_S_ZERO_DIVIDE: RPC_STATUS = 1767i32;
pub const RPC_TYPE_DISCONNECT_EVENT_CONTEXT_HANDLE: u32 = 2147483648u32;
pub const RPC_TYPE_STRICT_CONTEXT_HANDLE: u32 = 1073741824u32;
pub const RpcAttemptedLbsDecisions: RpcPerfCounters = 8i32;
pub const RpcAttemptedLbsMessages: RpcPerfCounters = 10i32;
pub const RpcBackEndConnectionAttempts: RpcPerfCounters = 2i32;
pub const RpcBackEndConnectionFailed: RpcPerfCounters = 3i32;
pub const RpcCallComplete: RPC_ASYNC_EVENT = 0i32;
pub const RpcClientCancel: RPC_ASYNC_EVENT = 4i32;
pub const RpcClientDisconnect: RPC_ASYNC_EVENT = 3i32;
pub const RpcCurrentUniqueUser: RpcPerfCounters = 1i32;
pub const RpcFailedLbsDecisions: RpcPerfCounters = 9i32;
pub const RpcFailedLbsMessages: RpcPerfCounters = 11i32;
pub const RpcIncomingBandwidth: RpcPerfCounters = 6i32;
pub const RpcIncomingConnections: RpcPerfCounters = 5i32;
pub const RpcLastCounter: RpcPerfCounters = 12i32;
pub const RpcNotificationCallCancel: RPC_NOTIFICATIONS = 2i32;
pub const RpcNotificationCallNone: RPC_NOTIFICATIONS = 0i32;
pub const RpcNotificationClientDisconnect: RPC_NOTIFICATIONS = 1i32;
pub const RpcNotificationTypeApc: RPC_NOTIFICATION_TYPES = 2i32;
pub const RpcNotificationTypeCallback: RPC_NOTIFICATION_TYPES = 5i32;
pub const RpcNotificationTypeEvent: RPC_NOTIFICATION_TYPES = 1i32;
pub const RpcNotificationTypeHwnd: RPC_NOTIFICATION_TYPES = 4i32;
pub const RpcNotificationTypeIoc: RPC_NOTIFICATION_TYPES = 3i32;
pub const RpcNotificationTypeNone: RPC_NOTIFICATION_TYPES = 0i32;
pub const RpcOutgoingBandwidth: RpcPerfCounters = 7i32;
pub const RpcReceiveComplete: RPC_ASYNC_EVENT = 2i32;
pub const RpcRequestsPerSecond: RpcPerfCounters = 4i32;
pub const RpcSendComplete: RPC_ASYNC_EVENT = 1i32;
pub const SEC_WINNT_AUTH_IDENTITY_ANSI: SEC_WINNT_AUTH_IDENTITY = 1u32;
pub const SEC_WINNT_AUTH_IDENTITY_UNICODE: SEC_WINNT_AUTH_IDENTITY = 2u32;
pub const STUB_CALL_SERVER: STUB_PHASE = 1i32;
pub const STUB_CALL_SERVER_NO_HRESULT: STUB_PHASE = 3i32;
pub const STUB_MARSHAL: STUB_PHASE = 2i32;
pub const STUB_UNMARSHAL: STUB_PHASE = 0i32;
pub const SYSTEM_HANDLE_COMPOSITION_OBJECT: system_handle_t = 9i32;
pub const SYSTEM_HANDLE_EVENT: system_handle_t = 2i32;
pub const SYSTEM_HANDLE_FILE: system_handle_t = 0i32;
pub const SYSTEM_HANDLE_INVALID: system_handle_t = 255i32;
pub const SYSTEM_HANDLE_JOB: system_handle_t = 11i32;
pub const SYSTEM_HANDLE_MAX: system_handle_t = 12i32;
pub const SYSTEM_HANDLE_MUTEX: system_handle_t = 3i32;
pub const SYSTEM_HANDLE_PIPE: system_handle_t = 12i32;
pub const SYSTEM_HANDLE_PROCESS: system_handle_t = 4i32;
pub const SYSTEM_HANDLE_REG_KEY: system_handle_t = 7i32;
pub const SYSTEM_HANDLE_SECTION: system_handle_t = 6i32;
pub const SYSTEM_HANDLE_SEMAPHORE: system_handle_t = 1i32;
pub const SYSTEM_HANDLE_SOCKET: system_handle_t = 10i32;
pub const SYSTEM_HANDLE_THREAD: system_handle_t = 8i32;
pub const SYSTEM_HANDLE_TOKEN: system_handle_t = 5i32;
pub const TARGET_IS_NT100_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT1012_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT102_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT351_OR_WIN95_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT40_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT50_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT51_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT60_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT61_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT62_OR_LATER: u32 = 1u32;
pub const TARGET_IS_NT63_OR_LATER: u32 = 1u32;
pub const TRANSPORT_TYPE_CN: u32 = 1u32;
pub const TRANSPORT_TYPE_DG: u32 = 2u32;
pub const TRANSPORT_TYPE_LPC: u32 = 4u32;
pub const TRANSPORT_TYPE_WMSG: u32 = 8u32;
pub const USER_CALL_IS_ASYNC: u32 = 256u32;
pub const USER_CALL_NEW_CORRELATION_DESC: u32 = 512u32;
pub const USER_MARSHAL_CB_BUFFER_SIZE: USER_MARSHAL_CB_TYPE = 0i32;
pub const USER_MARSHAL_CB_FREE: USER_MARSHAL_CB_TYPE = 3i32;
pub const USER_MARSHAL_CB_MARSHALL: USER_MARSHAL_CB_TYPE = 1i32;
pub const USER_MARSHAL_CB_UNMARSHALL: USER_MARSHAL_CB_TYPE = 2i32;
pub const USER_MARSHAL_FC_BYTE: u32 = 1u32;
pub const USER_MARSHAL_FC_CHAR: u32 = 2u32;
pub const USER_MARSHAL_FC_DOUBLE: u32 = 12u32;
pub const USER_MARSHAL_FC_FLOAT: u32 = 10u32;
pub const USER_MARSHAL_FC_HYPER: u32 = 11u32;
pub const USER_MARSHAL_FC_LONG: u32 = 8u32;
pub const USER_MARSHAL_FC_SHORT: u32 = 6u32;
pub const USER_MARSHAL_FC_SMALL: u32 = 3u32;
pub const USER_MARSHAL_FC_ULONG: u32 = 9u32;
pub const USER_MARSHAL_FC_USHORT: u32 = 7u32;
pub const USER_MARSHAL_FC_USMALL: u32 = 4u32;
pub const USER_MARSHAL_FC_WCHAR: u32 = 5u32;
pub const XLAT_CLIENT: XLAT_SIDE = 2i32;
pub const XLAT_SERVER: XLAT_SIDE = 1i32;
pub const __RPCPROXY_H_VERSION__: u32 = 477u32;
pub const cbNDRContext: u32 = 20u32;
pub const eeptAnsiString: ExtendedErrorParamTypes = 1i32;
pub const eeptBinary: ExtendedErrorParamTypes = 7i32;
pub const eeptLongVal: ExtendedErrorParamTypes = 3i32;
pub const eeptNone: ExtendedErrorParamTypes = 6i32;
pub const eeptPointerVal: ExtendedErrorParamTypes = 5i32;
pub const eeptShortVal: ExtendedErrorParamTypes = 4i32;
pub const eeptUnicodeString: ExtendedErrorParamTypes = 2i32;
pub const rcclClientUnknownLocality: RpcCallClientLocality = 3i32;
pub const rcclInvalid: RpcCallClientLocality = 0i32;
pub const rcclLocal: RpcCallClientLocality = 1i32;
pub const rcclRemote: RpcCallClientLocality = 2i32;
pub const rctGuaranteed: RpcCallType = 3i32;
pub const rctInvalid: RpcCallType = 0i32;
pub const rctNormal: RpcCallType = 1i32;
pub const rctTraining: RpcCallType = 2i32;
pub const rlafIPv4: RpcLocalAddressFormat = 1i32;
pub const rlafIPv6: RpcLocalAddressFormat = 2i32;
pub const rlafInvalid: RpcLocalAddressFormat = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EXPR_TOKEN(pub i32);
impl windows_core::TypeKind for EXPR_TOKEN {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExtendedErrorParamTypes(pub i32);
impl windows_core::TypeKind for ExtendedErrorParamTypes {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GROUP_NAME_SYNTAX(pub u32);
impl windows_core::TypeKind for GROUP_NAME_SYNTAX {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IDL_CS_CONVERT(pub i32);
impl windows_core::TypeKind for IDL_CS_CONVERT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION(pub i32);
impl windows_core::TypeKind for LRPC_SYSTEM_HANDLE_MARSHAL_DIRECTION {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MIDL_ES_CODE(pub i32);
impl windows_core::TypeKind for MIDL_ES_CODE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MIDL_ES_HANDLE_STYLE(pub i32);
impl windows_core::TypeKind for MIDL_ES_HANDLE_STYLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PROXY_PHASE(pub i32);
impl windows_core::TypeKind for PROXY_PHASE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPC_ADDRESS_CHANGE_TYPE(pub i32);
impl windows_core::TypeKind for RPC_ADDRESS_CHANGE_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPC_ASYNC_EVENT(pub i32);
impl windows_core::TypeKind for RPC_ASYNC_EVENT {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPC_BINDING_HANDLE_OPTIONS_FLAGS(pub u32);
impl windows_core::TypeKind for RPC_BINDING_HANDLE_OPTIONS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPC_C_AUTHN_INFO_TYPE(pub u32);
impl windows_core::TypeKind for RPC_C_AUTHN_INFO_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPC_C_HTTP_AUTHN_TARGET(pub u32);
impl windows_core::TypeKind for RPC_C_HTTP_AUTHN_TARGET {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPC_C_HTTP_FLAGS(pub u32);
impl windows_core::TypeKind for RPC_C_HTTP_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPC_C_QOS_CAPABILITIES(pub u32);
impl windows_core::TypeKind for RPC_C_QOS_CAPABILITIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPC_C_QOS_IDENTITY(pub u32);
impl windows_core::TypeKind for RPC_C_QOS_IDENTITY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPC_HTTP_REDIRECTOR_STAGE(pub i32);
impl windows_core::TypeKind for RPC_HTTP_REDIRECTOR_STAGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPC_NOTIFICATIONS(pub i32);
impl windows_core::TypeKind for RPC_NOTIFICATIONS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPC_NOTIFICATION_TYPES(pub i32);
impl windows_core::TypeKind for RPC_NOTIFICATION_TYPES {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RPC_STATUS(pub i32);
impl windows_core::TypeKind for RPC_STATUS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RpcCallClientLocality(pub i32);
impl windows_core::TypeKind for RpcCallClientLocality {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RpcCallType(pub i32);
impl windows_core::TypeKind for RpcCallType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RpcLocalAddressFormat(pub i32);
impl windows_core::TypeKind for RpcLocalAddressFormat {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RpcPerfCounters(pub i32);
impl windows_core::TypeKind for RpcPerfCounters {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY(pub u32);
impl windows_core::TypeKind for SEC_WINNT_AUTH_IDENTITY {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct STUB_PHASE(pub i32);
impl windows_core::TypeKind for STUB_PHASE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct USER_MARSHAL_CB_TYPE(pub i32);
impl windows_core::TypeKind for USER_MARSHAL_CB_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct XLAT_SIDE(pub i32);
impl windows_core::TypeKind for XLAT_SIDE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct system_handle_t(pub i32);
impl windows_core::TypeKind for system_handle_t {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ARRAY_INFO {
    pub Dimension: i32,
    pub BufferConformanceMark: *mut u32,
    pub BufferVarianceMark: *mut u32,
    pub MaxCountArray: *mut u32,
    pub OffsetArray: *mut u32,
    pub ActualCountArray: *mut u32,
}
impl Default for ARRAY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for ARRAY_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BinaryParam {
    pub Buffer: *mut core::ffi::c_void,
    pub Size: i16,
}
impl Default for BinaryParam {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BinaryParam {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union CLIENT_CALL_RETURN {
    pub Pointer: *mut core::ffi::c_void,
    pub Simple: isize,
}
impl Default for CLIENT_CALL_RETURN {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CLIENT_CALL_RETURN {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct COMM_FAULT_OFFSETS {
    pub CommOffset: i16,
    pub FaultOffset: i16,
}
impl Default for COMM_FAULT_OFFSETS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for COMM_FAULT_OFFSETS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FULL_PTR_XLAT_TABLES {
    pub RefIdToPointer: *mut core::ffi::c_void,
    pub PointerToRefId: *mut core::ffi::c_void,
    pub NextRefId: u32,
    pub XlatSide: XLAT_SIDE,
}
impl Default for FULL_PTR_XLAT_TABLES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for FULL_PTR_XLAT_TABLES {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GENERIC_BINDING_INFO {
    pub pObj: *mut core::ffi::c_void,
    pub Size: u32,
    pub pfnBind: GENERIC_BINDING_ROUTINE,
    pub pfnUnbind: GENERIC_UNBIND_ROUTINE,
}
impl Default for GENERIC_BINDING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GENERIC_BINDING_INFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GENERIC_BINDING_ROUTINE_PAIR {
    pub pfnBind: GENERIC_BINDING_ROUTINE,
    pub pfnUnbind: GENERIC_UNBIND_ROUTINE,
}
impl Default for GENERIC_BINDING_ROUTINE_PAIR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for GENERIC_BINDING_ROUTINE_PAIR {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct I_RpcProxyCallbackInterface {
    pub IsValidMachineFn: I_RpcProxyIsValidMachineFn,
    pub GetClientAddressFn: I_RpcProxyGetClientAddressFn,
    pub GetConnectionTimeoutFn: I_RpcProxyGetConnectionTimeoutFn,
    pub PerformCalloutFn: I_RpcPerformCalloutFn,
    pub FreeCalloutStateFn: I_RpcFreeCalloutStateFn,
    pub GetClientSessionAndResourceUUIDFn: I_RpcProxyGetClientSessionAndResourceUUID,
    pub ProxyFilterIfFn: I_RpcProxyFilterIfFn,
    pub RpcProxyUpdatePerfCounterFn: I_RpcProxyUpdatePerfCounterFn,
    pub RpcProxyUpdatePerfCounterBackendServerFn: I_RpcProxyUpdatePerfCounterBackendServerFn,
}
impl Default for I_RpcProxyCallbackInterface {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for I_RpcProxyCallbackInterface {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MALLOC_FREE_STRUCT {
    pub pfnAllocate: isize,
    pub pfnFree: isize,
}
impl Default for MALLOC_FREE_STRUCT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MALLOC_FREE_STRUCT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_FORMAT_STRING {
    pub Pad: i16,
    pub Format: [u8; 1],
}
impl Default for MIDL_FORMAT_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MIDL_FORMAT_STRING {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_INTERCEPTION_INFO {
    pub Version: u32,
    pub ProcString: *mut u8,
    pub ProcFormatOffsetTable: *const u16,
    pub ProcCount: u32,
    pub TypeString: *mut u8,
}
impl Default for MIDL_INTERCEPTION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MIDL_INTERCEPTION_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_INTERFACE_METHOD_PROPERTIES {
    pub MethodCount: u16,
    pub MethodProperties: *const *const MIDL_METHOD_PROPERTY_MAP,
}
impl Default for MIDL_INTERFACE_METHOD_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MIDL_INTERFACE_METHOD_PROPERTIES {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_METHOD_PROPERTY {
    pub Id: u32,
    pub Value: usize,
}
impl Default for MIDL_METHOD_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MIDL_METHOD_PROPERTY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_METHOD_PROPERTY_MAP {
    pub Count: u32,
    pub Properties: *const MIDL_METHOD_PROPERTY,
}
impl Default for MIDL_METHOD_PROPERTY_MAP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MIDL_METHOD_PROPERTY_MAP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_SERVER_INFO {
    pub pStubDesc: *mut MIDL_STUB_DESC,
    pub DispatchTable: *const SERVER_ROUTINE,
    pub ProcString: *mut u8,
    pub FmtStringOffset: *const u16,
    pub ThunkTable: *const STUB_THUNK,
    pub pTransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub nCount: usize,
    pub pSyntaxInfo: *mut MIDL_SYNTAX_INFO,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MIDL_SERVER_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MIDL_SERVER_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_STUBLESS_PROXY_INFO {
    pub pStubDesc: *mut MIDL_STUB_DESC,
    pub ProcFormatString: *mut u8,
    pub FormatStringOffset: *const u16,
    pub pTransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub nCount: usize,
    pub pSyntaxInfo: *mut MIDL_SYNTAX_INFO,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MIDL_STUBLESS_PROXY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MIDL_STUBLESS_PROXY_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_STUB_DESC {
    pub RpcInterfaceInformation: *mut core::ffi::c_void,
    pub pfnAllocate: PFN_RPC_ALLOCATE,
    pub pfnFree: PFN_RPC_FREE,
    pub IMPLICIT_HANDLE_INFO: MIDL_STUB_DESC_0,
    pub apfnNdrRundownRoutines: *const NDR_RUNDOWN,
    pub aGenericBindingRoutinePairs: *const GENERIC_BINDING_ROUTINE_PAIR,
    pub apfnExprEval: *const EXPR_EVAL,
    pub aXmitQuintuple: *const XMIT_ROUTINE_QUINTUPLE,
    pub pFormatTypes: *const u8,
    pub fCheckBounds: i32,
    pub Version: u32,
    pub pMallocFreeStruct: *mut MALLOC_FREE_STRUCT,
    pub MIDLVersion: i32,
    pub CommFaultOffsets: *const COMM_FAULT_OFFSETS,
    pub aUserMarshalQuadruple: *const USER_MARSHAL_ROUTINE_QUADRUPLE,
    pub NotifyRoutineTable: *const NDR_NOTIFY_ROUTINE,
    pub mFlags: usize,
    pub CsRoutineTables: *const NDR_CS_ROUTINES,
    pub ProxyServerInfo: *mut core::ffi::c_void,
    pub pExprInfo: *const NDR_EXPR_DESC,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MIDL_STUB_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MIDL_STUB_DESC {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union MIDL_STUB_DESC_0 {
    pub pAutoHandle: *mut *mut core::ffi::c_void,
    pub pPrimitiveHandle: *mut *mut core::ffi::c_void,
    pub pGenericBindingInfo: *mut GENERIC_BINDING_INFO,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MIDL_STUB_DESC_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MIDL_STUB_DESC_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_STUB_MESSAGE {
    pub RpcMsg: *mut RPC_MESSAGE,
    pub Buffer: *mut u8,
    pub BufferStart: *mut u8,
    pub BufferEnd: *mut u8,
    pub BufferMark: *mut u8,
    pub BufferLength: u32,
    pub MemorySize: u32,
    pub Memory: *mut u8,
    pub IsClient: u8,
    pub Pad: u8,
    pub uFlags2: u16,
    pub ReuseBuffer: i32,
    pub pAllocAllNodesContext: *mut NDR_ALLOC_ALL_NODES_CONTEXT,
    pub pPointerQueueState: *mut NDR_POINTER_QUEUE_STATE,
    pub IgnoreEmbeddedPointers: i32,
    pub PointerBufferMark: *mut u8,
    pub CorrDespIncrement: u8,
    pub uFlags: u8,
    pub UniquePtrCount: u16,
    pub MaxCount: usize,
    pub Offset: u32,
    pub ActualCount: u32,
    pub pfnAllocate: PFN_RPC_ALLOCATE,
    pub pfnFree: PFN_RPC_FREE,
    pub StackTop: *mut u8,
    pub pPresentedType: *mut u8,
    pub pTransmitType: *mut u8,
    pub SavedHandle: *mut core::ffi::c_void,
    pub StubDesc: *const MIDL_STUB_DESC,
    pub FullPtrXlatTables: *mut FULL_PTR_XLAT_TABLES,
    pub FullPtrRefId: u32,
    pub PointerLength: u32,
    pub _bitfield: i32,
    pub dwDestContext: u32,
    pub pvDestContext: *mut core::ffi::c_void,
    pub SavedContextHandles: *mut *mut NDR_SCONTEXT,
    pub ParamNumber: i32,
    pub pRpcChannelBuffer: Option<super::Com::IRpcChannelBuffer>,
    pub pArrayInfo: *mut ARRAY_INFO,
    pub SizePtrCountArray: *mut u32,
    pub SizePtrOffsetArray: *mut u32,
    pub SizePtrLengthArray: *mut u32,
    pub pArgQueue: *mut core::ffi::c_void,
    pub dwStubPhase: u32,
    pub LowStackMark: *mut core::ffi::c_void,
    pub pAsyncMsg: PNDR_ASYNC_MESSAGE,
    pub pCorrInfo: PNDR_CORRELATION_INFO,
    pub pCorrMemory: *mut u8,
    pub pMemoryList: *mut core::ffi::c_void,
    pub pCSInfo: isize,
    pub ConformanceMark: *mut u8,
    pub VarianceMark: *mut u8,
    pub Unused: isize,
    pub pContext: *mut _NDR_PROC_CONTEXT,
    pub ContextHandleHash: *mut core::ffi::c_void,
    pub pUserMarshalList: *mut core::ffi::c_void,
    pub Reserved51_3: isize,
    pub Reserved51_4: isize,
    pub Reserved51_5: isize,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MIDL_STUB_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MIDL_STUB_MESSAGE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_SYNTAX_INFO {
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub ProcString: *mut u8,
    pub FmtStringOffset: *const u16,
    pub TypeString: *mut u8,
    pub aUserMarshalQuadruple: *const core::ffi::c_void,
    pub pMethodProperties: *const MIDL_INTERFACE_METHOD_PROPERTIES,
    pub pReserved2: usize,
}
impl Default for MIDL_SYNTAX_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MIDL_SYNTAX_INFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_TYPE_PICKLING_INFO {
    pub Version: u32,
    pub Flags: u32,
    pub Reserved: [usize; 3],
}
impl Default for MIDL_TYPE_PICKLING_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for MIDL_TYPE_PICKLING_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    pub Version: u32,
    pub TypeFormatString: *mut u8,
    pub FormatStringSize: u16,
    pub TypeOffset: u16,
    pub StubDesc: *mut MIDL_STUB_DESC,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for MIDL_WINRT_TYPE_SERIALIZATION_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_ARRAY_ELEMENT_INFO {
    pub ElementMemSize: u32,
    pub Element: *mut core::ffi::c_void,
}
impl Default for NDR64_ARRAY_ELEMENT_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_ARRAY_ELEMENT_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_ARRAY_FLAGS {
    pub _bitfield: u8,
}
impl Default for NDR64_ARRAY_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_ARRAY_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union NDR64_BINDINGS {
    pub Primitive: NDR64_BIND_PRIMITIVE,
    pub Generic: NDR64_BIND_GENERIC,
    pub Context: NDR64_BIND_CONTEXT,
}
impl Default for NDR64_BINDINGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_BINDINGS {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_BIND_AND_NOTIFY_EXTENSION {
    pub Binding: NDR64_BIND_CONTEXT,
    pub NotifyIndex: u16,
}
impl Default for NDR64_BIND_AND_NOTIFY_EXTENSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_BIND_AND_NOTIFY_EXTENSION {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_BIND_CONTEXT {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub RoutineIndex: u8,
    pub Ordinal: u8,
}
impl Default for NDR64_BIND_CONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_BIND_CONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_BIND_GENERIC {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub RoutineIndex: u8,
    pub Size: u8,
}
impl Default for NDR64_BIND_GENERIC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_BIND_GENERIC {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_BIND_PRIMITIVE {
    pub HandleType: u8,
    pub Flags: u8,
    pub StackOffset: u16,
    pub Reserved: u16,
}
impl Default for NDR64_BIND_PRIMITIVE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_BIND_PRIMITIVE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub NumberDims: u8,
    pub NumberElements: u32,
    pub Element: *mut core::ffi::c_void,
}
impl Default for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_BOGUS_ARRAY_HEADER_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
    pub OriginalMemberLayout: *mut core::ffi::c_void,
    pub OriginalPointerLayout: *mut core::ffi::c_void,
    pub PointerLayout: *mut core::ffi::c_void,
}
impl Default for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_BOGUS_STRUCTURE_HEADER_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_BUFFER_ALIGN_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Reserved: u16,
    pub Reserved2: u32,
}
impl Default for NDR64_BUFFER_ALIGN_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_BUFFER_ALIGN_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
}
impl Default for NDR64_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_CONFORMANT_STRING_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_CONF_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub ElementSize: u32,
    pub ConfDescriptor: *mut core::ffi::c_void,
}
impl Default for NDR64_CONF_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_CONF_ARRAY_HEADER_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Dimensions: u8,
    pub MemorySize: u32,
    pub OriginalMemberLayout: *mut core::ffi::c_void,
    pub OriginalPointerLayout: *mut core::ffi::c_void,
    pub PointerLayout: *mut core::ffi::c_void,
    pub ConfArrayDescription: *mut core::ffi::c_void,
}
impl Default for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_CONF_BOGUS_STRUCTURE_HEADER_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
    pub ArrayDescription: *mut core::ffi::c_void,
}
impl Default for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_CONF_STRUCTURE_HEADER_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub ElementSize: u32,
    pub ConfDescriptor: *mut core::ffi::c_void,
    pub VarDescriptor: *mut core::ffi::c_void,
}
impl Default for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_CONF_VAR_ARRAY_HEADER_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    pub FixedArrayFormat: NDR64_BOGUS_ARRAY_HEADER_FORMAT,
    pub ConfDescription: *mut core::ffi::c_void,
    pub VarDescription: *mut core::ffi::c_void,
    pub OffsetDescription: *mut core::ffi::c_void,
}
impl Default for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_CONF_VAR_BOGUS_ARRAY_HEADER_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_CONSTANT_IID_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub Guid: windows_core::GUID,
}
impl Default for NDR64_CONSTANT_IID_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_CONSTANT_IID_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_CONTEXT_HANDLE_FLAGS {
    pub _bitfield: u8,
}
impl Default for NDR64_CONTEXT_HANDLE_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_CONTEXT_HANDLE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_CONTEXT_HANDLE_FORMAT {
    pub FormatCode: u8,
    pub ContextFlags: u8,
    pub RundownRoutineIndex: u8,
    pub Ordinal: u8,
}
impl Default for NDR64_CONTEXT_HANDLE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_CONTEXT_HANDLE_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_EMBEDDED_COMPLEX_FORMAT {
    pub FormatCode: u8,
    pub Reserve1: u8,
    pub Reserve2: u16,
    pub Type: *mut core::ffi::c_void,
}
impl Default for NDR64_EMBEDDED_COMPLEX_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_EMBEDDED_COMPLEX_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_ENCAPSULATED_UNION {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: u8,
    pub SwitchType: u8,
    pub MemoryOffset: u32,
    pub MemorySize: u32,
    pub Reserved: u32,
}
impl Default for NDR64_ENCAPSULATED_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_ENCAPSULATED_UNION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_EXPR_CONST32 {
    pub ExprType: u8,
    pub Reserved: u8,
    pub Reserved1: u16,
    pub ConstValue: u32,
}
impl Default for NDR64_EXPR_CONST32 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_EXPR_CONST32 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_EXPR_CONST64 {
    pub ExprType: u8,
    pub Reserved: u8,
    pub Reserved1: u16,
    pub ConstValue: i64,
}
impl Default for NDR64_EXPR_CONST64 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_EXPR_CONST64 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_EXPR_NOOP {
    pub ExprType: u8,
    pub Size: u8,
    pub Reserved: u16,
}
impl Default for NDR64_EXPR_NOOP {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_EXPR_NOOP {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_EXPR_OPERATOR {
    pub ExprType: u8,
    pub Operator: u8,
    pub CastType: u8,
    pub Reserved: u8,
}
impl Default for NDR64_EXPR_OPERATOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_EXPR_OPERATOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_EXPR_VAR {
    pub ExprType: u8,
    pub VarType: u8,
    pub Reserved: u16,
    pub Offset: u32,
}
impl Default for NDR64_EXPR_VAR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_EXPR_VAR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_FIXED_REPEAT_FORMAT {
    pub RepeatFormat: NDR64_REPEAT_FORMAT,
    pub Iterations: u32,
    pub Reserved: u32,
}
impl Default for NDR64_FIXED_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_FIXED_REPEAT_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_FIX_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub TotalSize: u32,
}
impl Default for NDR64_FIX_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_FIX_ARRAY_HEADER_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_IID_FLAGS {
    pub _bitfield: u8,
}
impl Default for NDR64_IID_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_IID_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_IID_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub IIDDescriptor: *mut core::ffi::c_void,
}
impl Default for NDR64_IID_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_IID_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_MEMPAD_FORMAT {
    pub FormatCode: u8,
    pub Reserve1: u8,
    pub MemPad: u16,
    pub Reserved2: u32,
}
impl Default for NDR64_MEMPAD_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_MEMPAD_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_NON_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub TotalSize: u32,
}
impl Default for NDR64_NON_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_NON_CONFORMANT_STRING_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_NON_ENCAPSULATED_UNION {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: u8,
    pub SwitchType: u8,
    pub MemorySize: u32,
    pub Switch: *mut core::ffi::c_void,
    pub Reserved: u32,
}
impl Default for NDR64_NON_ENCAPSULATED_UNION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_NON_ENCAPSULATED_UNION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_NO_REPEAT_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved1: u16,
    pub Reserved2: u32,
}
impl Default for NDR64_NO_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_NO_REPEAT_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_PARAM_FLAGS {
    pub _bitfield: u16,
}
impl Default for NDR64_PARAM_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_PARAM_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_PARAM_FORMAT {
    pub Type: *mut core::ffi::c_void,
    pub Attributes: NDR64_PARAM_FLAGS,
    pub Reserved: u16,
    pub StackOffset: u32,
}
impl Default for NDR64_PARAM_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_PARAM_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_PIPE_FLAGS {
    pub _bitfield: u8,
}
impl Default for NDR64_PIPE_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_PIPE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_PIPE_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Alignment: u8,
    pub Reserved: u8,
    pub Type: *mut core::ffi::c_void,
    pub MemorySize: u32,
    pub BufferSize: u32,
}
impl Default for NDR64_PIPE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_PIPE_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_POINTER_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Reserved: u16,
    pub Pointee: *mut core::ffi::c_void,
}
impl Default for NDR64_POINTER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_POINTER_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    pub Offset: u32,
    pub Reserved: u32,
}
impl Default for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_POINTER_INSTANCE_HEADER_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_POINTER_REPEAT_FLAGS {
    pub _bitfield: u8,
}
impl Default for NDR64_POINTER_REPEAT_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_POINTER_REPEAT_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_PROC_FLAGS {
    pub _bitfield: u32,
}
impl Default for NDR64_PROC_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_PROC_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_PROC_FORMAT {
    pub Flags: u32,
    pub StackSize: u32,
    pub ConstantClientBufferSize: u32,
    pub ConstantServerBufferSize: u32,
    pub RpcFlags: u16,
    pub FloatDoubleMask: u16,
    pub NumberOfParams: u16,
    pub ExtensionSize: u16,
}
impl Default for NDR64_PROC_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_PROC_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_RANGED_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub Reserved: u32,
    pub Min: u64,
    pub Max: u64,
}
impl Default for NDR64_RANGED_STRING_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_RANGED_STRING_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_RANGE_FORMAT {
    pub FormatCode: u8,
    pub RangeType: u8,
    pub Reserved: u16,
    pub MinValue: i64,
    pub MaxValue: i64,
}
impl Default for NDR64_RANGE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_RANGE_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_RANGE_PIPE_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub Alignment: u8,
    pub Reserved: u8,
    pub Type: *mut core::ffi::c_void,
    pub MemorySize: u32,
    pub BufferSize: u32,
    pub MinValue: u32,
    pub MaxValue: u32,
}
impl Default for NDR64_RANGE_PIPE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_RANGE_PIPE_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_REPEAT_FORMAT {
    pub FormatCode: u8,
    pub Flags: NDR64_POINTER_REPEAT_FLAGS,
    pub Reserved: u16,
    pub Increment: u32,
    pub OffsetToArray: u32,
    pub NumberOfPointers: u32,
}
impl Default for NDR64_REPEAT_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_REPEAT_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_RPC_FLAGS {
    pub _bitfield: u16,
}
impl Default for NDR64_RPC_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_RPC_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_SIMPLE_MEMBER_FORMAT {
    pub FormatCode: u8,
    pub Reserved1: u8,
    pub Reserved2: u16,
    pub Reserved3: u32,
}
impl Default for NDR64_SIMPLE_MEMBER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_SIMPLE_MEMBER_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_SIMPLE_REGION_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub RegionSize: u16,
    pub Reserved: u32,
}
impl Default for NDR64_SIMPLE_REGION_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_SIMPLE_REGION_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    pub Header: NDR64_STRING_HEADER_FORMAT,
    pub SizeDescription: *mut core::ffi::c_void,
}
impl Default for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_SIZED_CONFORMANT_STRING_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_STRING_FLAGS {
    pub _bitfield: u8,
}
impl Default for NDR64_STRING_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_STRING_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_STRING_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Flags: NDR64_STRING_FLAGS,
    pub ElementSize: u16,
}
impl Default for NDR64_STRING_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_STRING_HEADER_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_STRUCTURE_FLAGS {
    pub _bitfield: u8,
}
impl Default for NDR64_STRUCTURE_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_STRUCTURE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_STRUCTURE_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_STRUCTURE_FLAGS,
    pub Reserve: u8,
    pub MemorySize: u32,
}
impl Default for NDR64_STRUCTURE_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_STRUCTURE_HEADER_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_SYSTEM_HANDLE_FORMAT {
    pub FormatCode: u8,
    pub HandleType: u8,
    pub DesiredAccess: u32,
}
impl Default for NDR64_SYSTEM_HANDLE_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_SYSTEM_HANDLE_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_TRANSMIT_AS_FLAGS {
    pub _bitfield: u8,
}
impl Default for NDR64_TRANSMIT_AS_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_TRANSMIT_AS_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_TRANSMIT_AS_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub RoutineIndex: u16,
    pub TransmittedTypeWireAlignment: u16,
    pub MemoryAlignment: u16,
    pub PresentedTypeMemorySize: u32,
    pub TransmittedTypeBufferSize: u32,
    pub TransmittedType: *mut core::ffi::c_void,
}
impl Default for NDR64_TRANSMIT_AS_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_TRANSMIT_AS_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    pub FormatCode: u8,
    pub RealFormatCode: u8,
    pub Reserved: u16,
    pub Type: *mut core::ffi::c_void,
    pub CtxtFlags: u32,
    pub CtxtID: u32,
}
impl Default for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_TYPE_STRICT_CONTEXT_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_UNION_ARM {
    pub CaseValue: i64,
    pub Type: *mut core::ffi::c_void,
    pub Reserved: u32,
}
impl Default for NDR64_UNION_ARM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_UNION_ARM {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_UNION_ARM_SELECTOR {
    pub Reserved1: u8,
    pub Alignment: u8,
    pub Reserved2: u16,
    pub Arms: u32,
}
impl Default for NDR64_UNION_ARM_SELECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_UNION_ARM_SELECTOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_USER_MARSHAL_FLAGS {
    pub _bitfield: u8,
}
impl Default for NDR64_USER_MARSHAL_FLAGS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_USER_MARSHAL_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_USER_MARSHAL_FORMAT {
    pub FormatCode: u8,
    pub Flags: u8,
    pub RoutineIndex: u16,
    pub TransmittedTypeWireAlignment: u16,
    pub MemoryAlignment: u16,
    pub UserTypeMemorySize: u32,
    pub TransmittedTypeBufferSize: u32,
    pub TransmittedType: *mut core::ffi::c_void,
}
impl Default for NDR64_USER_MARSHAL_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_USER_MARSHAL_FORMAT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR64_VAR_ARRAY_HEADER_FORMAT {
    pub FormatCode: u8,
    pub Alignment: u8,
    pub Flags: NDR64_ARRAY_FLAGS,
    pub Reserved: u8,
    pub TotalSize: u32,
    pub ElementSize: u32,
    pub VarDescriptor: *mut core::ffi::c_void,
}
impl Default for NDR64_VAR_ARRAY_HEADER_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR64_VAR_ARRAY_HEADER_FORMAT {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR_CS_ROUTINES {
    pub pSizeConvertRoutines: *mut NDR_CS_SIZE_CONVERT_ROUTINES,
    pub pTagGettingRoutines: *mut CS_TAG_GETTING_ROUTINE,
}
impl Default for NDR_CS_ROUTINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR_CS_ROUTINES {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR_CS_SIZE_CONVERT_ROUTINES {
    pub pfnNetSize: CS_TYPE_NET_SIZE_ROUTINE,
    pub pfnToNetCs: CS_TYPE_TO_NETCS_ROUTINE,
    pub pfnLocalSize: CS_TYPE_LOCAL_SIZE_ROUTINE,
    pub pfnFromNetCs: CS_TYPE_FROM_NETCS_ROUTINE,
}
impl Default for NDR_CS_SIZE_CONVERT_ROUTINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR_CS_SIZE_CONVERT_ROUTINES {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR_EXPR_DESC {
    pub pOffset: *const u16,
    pub pFormatExpr: *mut u8,
}
impl Default for NDR_EXPR_DESC {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR_EXPR_DESC {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR_SCONTEXT {
    pub pad: [*mut core::ffi::c_void; 2],
    pub userContext: *mut core::ffi::c_void,
}
impl Default for NDR_SCONTEXT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for NDR_SCONTEXT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR_USER_MARSHAL_INFO {
    pub InformationLevel: u32,
    pub Anonymous: NDR_USER_MARSHAL_INFO_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for NDR_USER_MARSHAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for NDR_USER_MARSHAL_INFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union NDR_USER_MARSHAL_INFO_0 {
    pub Level1: NDR_USER_MARSHAL_INFO_LEVEL1,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for NDR_USER_MARSHAL_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for NDR_USER_MARSHAL_INFO_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NDR_USER_MARSHAL_INFO_LEVEL1 {
    pub Buffer: *mut core::ffi::c_void,
    pub BufferSize: u32,
    pub pfnAllocate: isize,
    pub pfnFree: isize,
    pub pRpcChannelBuffer: Option<super::Com::IRpcChannelBuffer>,
    pub Reserved: [usize; 5],
}
#[cfg(feature = "Win32_System_Com")]
impl Default for NDR_USER_MARSHAL_INFO_LEVEL1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for NDR_USER_MARSHAL_INFO_LEVEL1 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RDR_CALLOUT_STATE {
    pub LastError: RPC_STATUS,
    pub LastEEInfo: *mut core::ffi::c_void,
    pub LastCalledStage: RPC_HTTP_REDIRECTOR_STAGE,
    pub ServerName: *mut u16,
    pub ServerPort: *mut u16,
    pub RemoteUser: *mut u16,
    pub AuthType: *mut u16,
    pub ResourceTypePresent: u8,
    pub SessionIdPresent: u8,
    pub InterfacePresent: u8,
    pub ResourceType: windows_core::GUID,
    pub SessionId: windows_core::GUID,
    pub Interface: RPC_SYNTAX_IDENTIFIER,
    pub CertContext: *mut core::ffi::c_void,
}
impl Default for RDR_CALLOUT_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RDR_CALLOUT_STATE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union RPC_ASYNC_NOTIFICATION_INFO {
    pub APC: RPC_ASYNC_NOTIFICATION_INFO_0,
    pub IOC: RPC_ASYNC_NOTIFICATION_INFO_1,
    pub IntPtr: RPC_ASYNC_NOTIFICATION_INFO_2,
    pub hEvent: super::super::Foundation::HANDLE,
    pub NotificationRoutine: PFN_RPCNOTIFICATION_ROUTINE,
}
#[cfg(feature = "Win32_System_IO")]
impl Default for RPC_ASYNC_NOTIFICATION_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_IO")]
impl windows_core::TypeKind for RPC_ASYNC_NOTIFICATION_INFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ASYNC_NOTIFICATION_INFO_0 {
    pub NotificationRoutine: PFN_RPCNOTIFICATION_ROUTINE,
    pub hThread: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_System_IO")]
impl Default for RPC_ASYNC_NOTIFICATION_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_IO")]
impl windows_core::TypeKind for RPC_ASYNC_NOTIFICATION_INFO_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ASYNC_NOTIFICATION_INFO_1 {
    pub hIOPort: super::super::Foundation::HANDLE,
    pub dwNumberOfBytesTransferred: u32,
    pub dwCompletionKey: usize,
    pub lpOverlapped: *mut super::IO::OVERLAPPED,
}
#[cfg(feature = "Win32_System_IO")]
impl Default for RPC_ASYNC_NOTIFICATION_INFO_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_IO")]
impl windows_core::TypeKind for RPC_ASYNC_NOTIFICATION_INFO_1 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ASYNC_NOTIFICATION_INFO_2 {
    pub hWnd: super::super::Foundation::HWND,
    pub Msg: u32,
}
#[cfg(feature = "Win32_System_IO")]
impl Default for RPC_ASYNC_NOTIFICATION_INFO_2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_IO")]
impl windows_core::TypeKind for RPC_ASYNC_NOTIFICATION_INFO_2 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_IO")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ASYNC_STATE {
    pub Size: u32,
    pub Signature: u32,
    pub Lock: i32,
    pub Flags: u32,
    pub StubInfo: *mut core::ffi::c_void,
    pub UserInfo: *mut core::ffi::c_void,
    pub RuntimeInfo: *mut core::ffi::c_void,
    pub Event: RPC_ASYNC_EVENT,
    pub NotificationType: RPC_NOTIFICATION_TYPES,
    pub u: RPC_ASYNC_NOTIFICATION_INFO,
    pub Reserved: [isize; 4],
}
#[cfg(feature = "Win32_System_IO")]
impl Default for RPC_ASYNC_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_IO")]
impl windows_core::TypeKind for RPC_ASYNC_STATE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_HANDLE_OPTIONS_V1 {
    pub Version: u32,
    pub Flags: RPC_BINDING_HANDLE_OPTIONS_FLAGS,
    pub ComTimeout: u32,
    pub CallTimeout: u32,
}
impl Default for RPC_BINDING_HANDLE_OPTIONS_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_BINDING_HANDLE_OPTIONS_V1 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_A {
    pub Version: u32,
    pub ServerPrincName: *mut u8,
    pub AuthnLevel: u32,
    pub AuthnSvc: u32,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_BINDING_HANDLE_SECURITY_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_BINDING_HANDLE_SECURITY_V1_A {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_HANDLE_SECURITY_V1_W {
    pub Version: u32,
    pub ServerPrincName: *mut u16,
    pub AuthnLevel: u32,
    pub AuthnSvc: u32,
    pub AuthIdentity: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub SecurityQos: *mut RPC_SECURITY_QOS,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_BINDING_HANDLE_SECURITY_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_BINDING_HANDLE_SECURITY_V1_W {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    pub Version: u32,
    pub Flags: u32,
    pub ProtocolSequence: u32,
    pub NetworkAddress: *mut u8,
    pub StringEndpoint: *mut u8,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_A_0,
    pub ObjectUuid: windows_core::GUID,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_BINDING_HANDLE_TEMPLATE_V1_A {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    pub Reserved: *mut u8,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_BINDING_HANDLE_TEMPLATE_V1_A_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    pub Version: u32,
    pub Flags: u32,
    pub ProtocolSequence: u32,
    pub NetworkAddress: *mut u16,
    pub StringEndpoint: *mut u16,
    pub u1: RPC_BINDING_HANDLE_TEMPLATE_V1_W_0,
    pub ObjectUuid: windows_core::GUID,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_BINDING_HANDLE_TEMPLATE_V1_W {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    pub Reserved: *mut u16,
}
impl Default for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_BINDING_HANDLE_TEMPLATE_V1_W_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_BINDING_VECTOR {
    pub Count: u32,
    pub BindingH: [*mut core::ffi::c_void; 1],
}
impl Default for RPC_BINDING_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_BINDING_VECTOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V1_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
}
impl Default for RPC_CALL_ATTRIBUTES_V1_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_CALL_ATTRIBUTES_V1_A {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V1_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
}
impl Default for RPC_CALL_ATTRIBUTES_V1_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_CALL_ATTRIBUTES_V1_W {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V2_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
    pub KernelModeCaller: super::super::Foundation::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: u32,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_core::GUID,
}
impl Default for RPC_CALL_ATTRIBUTES_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_CALL_ATTRIBUTES_V2_A {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V2_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
    pub KernelModeCaller: super::super::Foundation::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: RpcCallClientLocality,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_core::GUID,
}
impl Default for RPC_CALL_ATTRIBUTES_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_CALL_ATTRIBUTES_V2_W {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V3_A {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u8,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u8,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
    pub KernelModeCaller: super::super::Foundation::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: u32,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_core::GUID,
    pub ClientIdentifierBufferLength: u32,
    pub ClientIdentifier: *mut u8,
}
impl Default for RPC_CALL_ATTRIBUTES_V3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_CALL_ATTRIBUTES_V3_A {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_ATTRIBUTES_V3_W {
    pub Version: u32,
    pub Flags: u32,
    pub ServerPrincipalNameBufferLength: u32,
    pub ServerPrincipalName: *mut u16,
    pub ClientPrincipalNameBufferLength: u32,
    pub ClientPrincipalName: *mut u16,
    pub AuthenticationLevel: u32,
    pub AuthenticationService: u32,
    pub NullSession: super::super::Foundation::BOOL,
    pub KernelModeCaller: super::super::Foundation::BOOL,
    pub ProtocolSequence: u32,
    pub IsClientLocal: RpcCallClientLocality,
    pub ClientPID: super::super::Foundation::HANDLE,
    pub CallStatus: u32,
    pub CallType: RpcCallType,
    pub CallLocalAddress: *mut RPC_CALL_LOCAL_ADDRESS_V1,
    pub OpNum: u16,
    pub InterfaceUuid: windows_core::GUID,
    pub ClientIdentifierBufferLength: u32,
    pub ClientIdentifier: *mut u8,
}
impl Default for RPC_CALL_ATTRIBUTES_V3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_CALL_ATTRIBUTES_V3_W {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CALL_LOCAL_ADDRESS_V1 {
    pub Version: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub BufferSize: u32,
    pub AddressFormat: RpcLocalAddressFormat,
}
impl Default for RPC_CALL_LOCAL_ADDRESS_V1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_CALL_LOCAL_ADDRESS_V1 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CLIENT_INFORMATION1 {
    pub UserName: *mut u8,
    pub ComputerName: *mut u8,
    pub Privilege: u16,
    pub AuthFlags: u32,
}
impl Default for RPC_CLIENT_INFORMATION1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_CLIENT_INFORMATION1 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_CLIENT_INTERFACE {
    pub Length: u32,
    pub InterfaceId: RPC_SYNTAX_IDENTIFIER,
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub RpcProtseqEndpointCount: u32,
    pub RpcProtseqEndpoint: *mut RPC_PROTSEQ_ENDPOINT,
    pub Reserved: usize,
    pub InterpreterInfo: *const core::ffi::c_void,
    pub Flags: u32,
}
impl Default for RPC_CLIENT_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_CLIENT_INTERFACE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    pub BufferSize: u32,
    pub Buffer: windows_core::PSTR,
}
impl Default for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_C_OPT_COOKIE_AUTH_DESCRIPTOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_DISPATCH_TABLE {
    pub DispatchTableCount: u32,
    pub DispatchTable: RPC_DISPATCH_FUNCTION,
    pub Reserved: isize,
}
impl Default for RPC_DISPATCH_TABLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_DISPATCH_TABLE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_EE_INFO_PARAM {
    pub ParameterType: ExtendedErrorParamTypes,
    pub u: RPC_EE_INFO_PARAM_0,
}
impl Default for RPC_EE_INFO_PARAM {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_EE_INFO_PARAM {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union RPC_EE_INFO_PARAM_0 {
    pub AnsiString: windows_core::PSTR,
    pub UnicodeString: windows_core::PWSTR,
    pub LVal: i32,
    pub SVal: i16,
    pub PVal: u64,
    pub BVal: BinaryParam,
}
impl Default for RPC_EE_INFO_PARAM_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_EE_INFO_PARAM_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ENDPOINT_TEMPLATEA {
    pub Version: u32,
    pub ProtSeq: windows_core::PSTR,
    pub Endpoint: windows_core::PSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub Backlog: u32,
}
impl Default for RPC_ENDPOINT_TEMPLATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_ENDPOINT_TEMPLATEA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ENDPOINT_TEMPLATEW {
    pub Version: u32,
    pub ProtSeq: windows_core::PWSTR,
    pub Endpoint: windows_core::PWSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
    pub Backlog: u32,
}
impl Default for RPC_ENDPOINT_TEMPLATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_ENDPOINT_TEMPLATEW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_ERROR_ENUM_HANDLE {
    pub Signature: u32,
    pub CurrentPos: *mut core::ffi::c_void,
    pub Head: *mut core::ffi::c_void,
}
impl Default for RPC_ERROR_ENUM_HANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_ERROR_ENUM_HANDLE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_EXTENDED_ERROR_INFO {
    pub Version: u32,
    pub ComputerName: windows_core::PWSTR,
    pub ProcessID: u32,
    pub u: RPC_EXTENDED_ERROR_INFO_0,
    pub GeneratingComponent: u32,
    pub Status: u32,
    pub DetectionLocation: u16,
    pub Flags: u16,
    pub NumberOfParameters: i32,
    pub Parameters: [RPC_EE_INFO_PARAM; 4],
}
impl Default for RPC_EXTENDED_ERROR_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_EXTENDED_ERROR_INFO {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union RPC_EXTENDED_ERROR_INFO_0 {
    pub SystemTime: super::super::Foundation::SYSTEMTIME,
    pub FileTime: super::super::Foundation::FILETIME,
}
impl Default for RPC_EXTENDED_ERROR_INFO_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_EXTENDED_ERROR_INFO_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_HTTP_TRANSPORT_CREDENTIALS_A {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
    pub ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_A,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_A {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
    pub ProxyCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_HTTP_TRANSPORT_CREDENTIALS_V2_W {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    pub TransportCredentials: *mut core::ffi::c_void,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u8,
    pub ProxyCredentials: *mut core::ffi::c_void,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_A {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    pub TransportCredentials: *mut core::ffi::c_void,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
    pub ProxyCredentials: *mut core::ffi::c_void,
    pub NumberOfProxyAuthnSchemes: u32,
    pub ProxyAuthnSchemes: *mut u32,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_HTTP_TRANSPORT_CREDENTIALS_V3_W {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    pub TransportCredentials: *mut SEC_WINNT_AUTH_IDENTITY_W,
    pub Flags: RPC_C_HTTP_FLAGS,
    pub AuthenticationTarget: RPC_C_HTTP_AUTHN_TARGET,
    pub NumberOfAuthnSchemes: u32,
    pub AuthnSchemes: *mut u32,
    pub ServerCertificateSubject: *mut u16,
}
impl Default for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_HTTP_TRANSPORT_CREDENTIALS_W {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_IF_ID {
    pub Uuid: windows_core::GUID,
    pub VersMajor: u16,
    pub VersMinor: u16,
}
impl Default for RPC_IF_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_IF_ID {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_IF_ID_VECTOR {
    pub Count: u32,
    pub IfId: [*mut RPC_IF_ID; 1],
}
impl Default for RPC_IF_ID_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_IF_ID_VECTOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_IMPORT_CONTEXT_P {
    pub LookupContext: *mut core::ffi::c_void,
    pub ProposedHandle: *mut core::ffi::c_void,
    pub Bindings: *mut RPC_BINDING_VECTOR,
}
impl Default for RPC_IMPORT_CONTEXT_P {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_IMPORT_CONTEXT_P {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_INTERFACE_TEMPLATEA {
    pub Version: u32,
    pub IfSpec: *mut core::ffi::c_void,
    pub MgrTypeUuid: *mut windows_core::GUID,
    pub MgrEpv: *mut core::ffi::c_void,
    pub Flags: u32,
    pub MaxCalls: u32,
    pub MaxRpcSize: u32,
    pub IfCallback: RPC_IF_CALLBACK_FN,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: windows_core::PSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
}
impl Default for RPC_INTERFACE_TEMPLATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_INTERFACE_TEMPLATEA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_INTERFACE_TEMPLATEW {
    pub Version: u32,
    pub IfSpec: *mut core::ffi::c_void,
    pub MgrTypeUuid: *mut windows_core::GUID,
    pub MgrEpv: *mut core::ffi::c_void,
    pub Flags: u32,
    pub MaxCalls: u32,
    pub MaxRpcSize: u32,
    pub IfCallback: RPC_IF_CALLBACK_FN,
    pub UuidVector: *mut UUID_VECTOR,
    pub Annotation: windows_core::PWSTR,
    pub SecurityDescriptor: *mut core::ffi::c_void,
}
impl Default for RPC_INTERFACE_TEMPLATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_INTERFACE_TEMPLATEW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_MESSAGE {
    pub Handle: *mut core::ffi::c_void,
    pub DataRepresentation: u32,
    pub Buffer: *mut core::ffi::c_void,
    pub BufferLength: u32,
    pub ProcNum: u32,
    pub TransferSyntax: *mut RPC_SYNTAX_IDENTIFIER,
    pub RpcInterfaceInformation: *mut core::ffi::c_void,
    pub ReservedForRuntime: *mut core::ffi::c_void,
    pub ManagerEpv: *mut core::ffi::c_void,
    pub ImportContext: *mut core::ffi::c_void,
    pub RpcFlags: u32,
}
impl Default for RPC_MESSAGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_MESSAGE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_POLICY {
    pub Length: u32,
    pub EndpointFlags: u32,
    pub NICFlags: u32,
}
impl Default for RPC_POLICY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_POLICY {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_PROTSEQ_ENDPOINT {
    pub RpcProtocolSequence: *mut u8,
    pub Endpoint: *mut u8,
}
impl Default for RPC_PROTSEQ_ENDPOINT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_PROTSEQ_ENDPOINT {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_PROTSEQ_VECTORA {
    pub Count: u32,
    pub Protseq: [*mut u8; 1],
}
impl Default for RPC_PROTSEQ_VECTORA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_PROTSEQ_VECTORA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_PROTSEQ_VECTORW {
    pub Count: u32,
    pub Protseq: [*mut u16; 1],
}
impl Default for RPC_PROTSEQ_VECTORW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_PROTSEQ_VECTORW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_SECURITY_QOS {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_SECURITY_QOS_V2_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V2_A_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V2_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V2_A {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union RPC_SECURITY_QOS_V2_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V2_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V2_A_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_SECURITY_QOS_V2_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V2_W_0,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V2_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V2_W {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union RPC_SECURITY_QOS_V2_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V2_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V2_W_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_SECURITY_QOS_V3_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V3_A_0,
    pub Sid: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V3_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V3_A {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union RPC_SECURITY_QOS_V3_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V3_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V3_A_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_SECURITY_QOS_V3_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V3_W_0,
    pub Sid: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V3_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V3_W {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union RPC_SECURITY_QOS_V3_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V3_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V3_W_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_SECURITY_QOS_V4_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V4_A_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V4_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V4_A {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union RPC_SECURITY_QOS_V4_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V4_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V4_A_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_SECURITY_QOS_V4_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V4_W_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V4_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V4_W {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union RPC_SECURITY_QOS_V4_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V4_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V4_W_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_SECURITY_QOS_V5_A {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V5_A_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
    pub ServerSecurityDescriptor: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V5_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V5_A {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union RPC_SECURITY_QOS_V5_A_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_A,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V5_A_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V5_A_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_SECURITY_QOS_V5_W {
    pub Version: u32,
    pub Capabilities: RPC_C_QOS_CAPABILITIES,
    pub IdentityTracking: RPC_C_QOS_IDENTITY,
    pub ImpersonationType: super::Com::RPC_C_IMP_LEVEL,
    pub AdditionalSecurityInfoType: RPC_C_AUTHN_INFO_TYPE,
    pub u: RPC_SECURITY_QOS_V5_W_0,
    pub Sid: *mut core::ffi::c_void,
    pub EffectiveOnly: u32,
    pub ServerSecurityDescriptor: *mut core::ffi::c_void,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V5_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V5_W {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union RPC_SECURITY_QOS_V5_W_0 {
    pub HttpCredentials: *mut RPC_HTTP_TRANSPORT_CREDENTIALS_W,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for RPC_SECURITY_QOS_V5_W_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for RPC_SECURITY_QOS_V5_W_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_SEC_CONTEXT_KEY_INFO {
    pub EncryptAlgorithm: u32,
    pub KeySize: u32,
    pub SignatureAlgorithm: u32,
}
impl Default for RPC_SEC_CONTEXT_KEY_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_SEC_CONTEXT_KEY_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_SERVER_INTERFACE {
    pub Length: u32,
    pub InterfaceId: RPC_SYNTAX_IDENTIFIER,
    pub TransferSyntax: RPC_SYNTAX_IDENTIFIER,
    pub DispatchTable: *mut RPC_DISPATCH_TABLE,
    pub RpcProtseqEndpointCount: u32,
    pub RpcProtseqEndpoint: *mut RPC_PROTSEQ_ENDPOINT,
    pub DefaultManagerEpv: *mut core::ffi::c_void,
    pub InterpreterInfo: *const core::ffi::c_void,
    pub Flags: u32,
}
impl Default for RPC_SERVER_INTERFACE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_SERVER_INTERFACE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_STATS_VECTOR {
    pub Count: u32,
    pub Stats: [u32; 1],
}
impl Default for RPC_STATS_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_STATS_VECTOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_SYNTAX_IDENTIFIER {
    pub SyntaxGUID: windows_core::GUID,
    pub SyntaxVersion: RPC_VERSION,
}
impl Default for RPC_SYNTAX_IDENTIFIER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_SYNTAX_IDENTIFIER {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_TRANSFER_SYNTAX {
    pub Uuid: windows_core::GUID,
    pub VersMajor: u16,
    pub VersMinor: u16,
}
impl Default for RPC_TRANSFER_SYNTAX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_TRANSFER_SYNTAX {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RPC_VERSION {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
}
impl Default for RPC_VERSION {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for RPC_VERSION {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCONTEXT_QUEUE {
    pub NumberOfObjects: u32,
    pub ArrayOfObjects: *mut *mut NDR_SCONTEXT,
}
impl Default for SCONTEXT_QUEUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SCONTEXT_QUEUE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY_A {
    pub User: *mut u8,
    pub UserLength: u32,
    pub Domain: *mut u8,
    pub DomainLength: u32,
    pub Password: *mut u8,
    pub PasswordLength: u32,
    pub Flags: SEC_WINNT_AUTH_IDENTITY,
}
impl Default for SEC_WINNT_AUTH_IDENTITY_A {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SEC_WINNT_AUTH_IDENTITY_A {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SEC_WINNT_AUTH_IDENTITY_W {
    pub User: *mut u16,
    pub UserLength: u32,
    pub Domain: *mut u16,
    pub DomainLength: u32,
    pub Password: *mut u16,
    pub PasswordLength: u32,
    pub Flags: SEC_WINNT_AUTH_IDENTITY,
}
impl Default for SEC_WINNT_AUTH_IDENTITY_W {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SEC_WINNT_AUTH_IDENTITY_W {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USER_MARSHAL_CB {
    pub Flags: u32,
    pub pStubMsg: *mut MIDL_STUB_MESSAGE,
    pub pReserve: *mut u8,
    pub Signature: u32,
    pub CBType: USER_MARSHAL_CB_TYPE,
    pub pFormat: *mut u8,
    pub pTypeFormat: *mut u8,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for USER_MARSHAL_CB {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for USER_MARSHAL_CB {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USER_MARSHAL_ROUTINE_QUADRUPLE {
    pub pfnBufferSize: USER_MARSHAL_SIZING_ROUTINE,
    pub pfnMarshall: USER_MARSHAL_MARSHALLING_ROUTINE,
    pub pfnUnmarshall: USER_MARSHAL_UNMARSHALLING_ROUTINE,
    pub pfnFree: USER_MARSHAL_FREEING_ROUTINE,
}
impl Default for USER_MARSHAL_ROUTINE_QUADRUPLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for USER_MARSHAL_ROUTINE_QUADRUPLE {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UUID_VECTOR {
    pub Count: u32,
    pub Uuid: [*mut windows_core::GUID; 1],
}
impl Default for UUID_VECTOR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for UUID_VECTOR {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct XMIT_ROUTINE_QUINTUPLE {
    pub pfnTranslateToXmit: XMIT_HELPER_ROUTINE,
    pub pfnTranslateFromXmit: XMIT_HELPER_ROUTINE,
    pub pfnFreeXmit: XMIT_HELPER_ROUTINE,
    pub pfnFreeInst: XMIT_HELPER_ROUTINE,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for XMIT_ROUTINE_QUINTUPLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_System_Com")]
impl windows_core::TypeKind for XMIT_ROUTINE_QUINTUPLE {
    type TypeKind = windows_core::CloneType;
}
pub type CS_TAG_GETTING_ROUTINE = Option<unsafe extern "system" fn(hbinding: *mut core::ffi::c_void, fserverside: i32, pulsendingtag: *mut u32, puldesiredreceivingtag: *mut u32, pulreceivingtag: *mut u32, pstatus: *mut u32)>;
pub type CS_TYPE_FROM_NETCS_ROUTINE = Option<unsafe extern "system" fn(hbinding: *mut core::ffi::c_void, ulnetworkcodeset: u32, pnetworkdata: *mut u8, ulnetworkdatalength: u32, ullocalbuffersize: u32, plocaldata: *mut core::ffi::c_void, pullocaldatalength: *mut u32, pstatus: *mut u32)>;
pub type CS_TYPE_LOCAL_SIZE_ROUTINE = Option<unsafe extern "system" fn(hbinding: *mut core::ffi::c_void, ulnetworkcodeset: u32, ulnetworkbuffersize: u32, conversiontype: *mut IDL_CS_CONVERT, pullocalbuffersize: *mut u32, pstatus: *mut u32)>;
pub type CS_TYPE_NET_SIZE_ROUTINE = Option<unsafe extern "system" fn(hbinding: *mut core::ffi::c_void, ulnetworkcodeset: u32, ullocalbuffersize: u32, conversiontype: *mut IDL_CS_CONVERT, pulnetworkbuffersize: *mut u32, pstatus: *mut u32)>;
pub type CS_TYPE_TO_NETCS_ROUTINE = Option<unsafe extern "system" fn(hbinding: *mut core::ffi::c_void, ulnetworkcodeset: u32, plocaldata: *mut core::ffi::c_void, ullocaldatalength: u32, pnetworkdata: *mut u8, pulnetworkdatalength: *mut u32, pstatus: *mut u32)>;
#[cfg(feature = "Win32_System_Com")]
pub type EXPR_EVAL = Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
pub type GENERIC_BINDING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void) -> *mut core::ffi::c_void>;
pub type GENERIC_UNBIND_ROUTINE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void, param1: *mut u8)>;
pub type I_RpcFreeCalloutStateFn = Option<unsafe extern "system" fn(calloutstate: *mut RDR_CALLOUT_STATE)>;
pub type I_RpcPerformCalloutFn = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, calloutstate: *mut RDR_CALLOUT_STATE, stage: RPC_HTTP_REDIRECTOR_STAGE) -> RPC_STATUS>;
pub type I_RpcProxyFilterIfFn = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, ifuuid: *const windows_core::GUID, ifmajorversion: u16, fallow: *mut i32) -> RPC_STATUS>;
pub type I_RpcProxyGetClientAddressFn = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, buffer: windows_core::PCSTR, bufferlength: *mut u32) -> RPC_STATUS>;
pub type I_RpcProxyGetClientSessionAndResourceUUID = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionidpresent: *mut i32, sessionid: *mut windows_core::GUID, resourceidpresent: *mut i32, resourceid: *mut windows_core::GUID) -> RPC_STATUS>;
pub type I_RpcProxyGetConnectionTimeoutFn = Option<unsafe extern "system" fn(connectiontimeout: *mut u32) -> RPC_STATUS>;
pub type I_RpcProxyIsValidMachineFn = Option<unsafe extern "system" fn(machine: windows_core::PCWSTR, dotmachine: windows_core::PCWSTR, portnumber: u32) -> RPC_STATUS>;
pub type I_RpcProxyUpdatePerfCounterBackendServerFn = Option<unsafe extern "system" fn(machinename: *const u16, isconnectevent: i32)>;
pub type I_RpcProxyUpdatePerfCounterFn = Option<unsafe extern "system" fn(counter: RpcPerfCounters, modifytrend: i32, size: u32)>;
pub type MIDL_ES_ALLOC = Option<unsafe extern "system" fn(state: *mut core::ffi::c_void, pbuffer: *mut *mut i8, psize: *mut u32)>;
pub type MIDL_ES_READ = Option<unsafe extern "system" fn(state: *mut core::ffi::c_void, pbuffer: *mut *mut i8, psize: *mut u32)>;
pub type MIDL_ES_WRITE = Option<unsafe extern "system" fn(state: *mut core::ffi::c_void, buffer: windows_core::PCSTR, size: u32)>;
pub type NDR_NOTIFY2_ROUTINE = Option<unsafe extern "system" fn(flag: u8)>;
pub type NDR_NOTIFY_ROUTINE = Option<unsafe extern "system" fn()>;
pub type NDR_RUNDOWN = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void)>;
#[cfg(feature = "Win32_System_IO")]
pub type PFN_RPCNOTIFICATION_ROUTINE = Option<unsafe extern "system" fn(pasync: *mut RPC_ASYNC_STATE, context: *mut core::ffi::c_void, event: RPC_ASYNC_EVENT)>;
pub type PFN_RPC_ALLOCATE = Option<unsafe extern "system" fn(param0: usize) -> *mut core::ffi::c_void>;
pub type PFN_RPC_FREE = Option<unsafe extern "system" fn(param0: *mut core::ffi::c_void)>;
pub type PRPC_RUNDOWN = Option<unsafe extern "system" fn(associationcontext: *mut core::ffi::c_void)>;
pub type RPCLT_PDU_FILTER_FUNC = Option<unsafe extern "system" fn(buffer: *mut core::ffi::c_void, bufferlength: u32, fdatagram: i32)>;
pub type RPC_ADDRESS_CHANGE_FN = Option<unsafe extern "system" fn(arg: *mut core::ffi::c_void)>;
pub type RPC_AUTH_KEY_RETRIEVAL_FN = Option<unsafe extern "system" fn(arg: *const core::ffi::c_void, serverprincname: windows_core::PCWSTR, keyver: u32, key: *mut *mut core::ffi::c_void, status: *mut RPC_STATUS)>;
pub type RPC_BLOCKING_FN = Option<unsafe extern "system" fn(hwnd: *mut core::ffi::c_void, context: *mut core::ffi::c_void, hsyncevent: *mut core::ffi::c_void) -> RPC_STATUS>;
pub type RPC_CLIENT_ALLOC = Option<unsafe extern "system" fn(size: usize) -> *mut core::ffi::c_void>;
pub type RPC_CLIENT_FREE = Option<unsafe extern "system" fn(ptr: *const core::ffi::c_void)>;
pub type RPC_DISPATCH_FUNCTION = Option<unsafe extern "system" fn(message: *mut RPC_MESSAGE)>;
pub type RPC_FORWARD_FUNCTION = Option<unsafe extern "system" fn(interfaceid: *mut windows_core::GUID, interfaceversion: *mut RPC_VERSION, objectid: *mut windows_core::GUID, rpcpro: *mut u8, ppdestendpoint: *mut *mut core::ffi::c_void) -> RPC_STATUS>;
pub type RPC_HTTP_PROXY_FREE_STRING = Option<unsafe extern "system" fn(string: windows_core::PCWSTR)>;
pub type RPC_IF_CALLBACK_FN = Option<unsafe extern "system" fn(interfaceuuid: *const core::ffi::c_void, context: *const core::ffi::c_void) -> RPC_STATUS>;
pub type RPC_INTERFACE_GROUP_IDLE_CALLBACK_FN = Option<unsafe extern "system" fn(ifgroup: *const core::ffi::c_void, idlecallbackcontext: *const core::ffi::c_void, isgroupidle: u32)>;
pub type RPC_MGMT_AUTHORIZATION_FN = Option<unsafe extern "system" fn(clientbinding: *const core::ffi::c_void, requestedmgmtoperation: u32, status: *mut RPC_STATUS) -> i32>;
pub type RPC_NEW_HTTP_PROXY_CHANNEL = Option<unsafe extern "system" fn(redirectorstage: RPC_HTTP_REDIRECTOR_STAGE, servername: windows_core::PCWSTR, serverport: windows_core::PCWSTR, remoteuser: windows_core::PCWSTR, authtype: windows_core::PCWSTR, resourceuuid: *mut core::ffi::c_void, sessionid: *mut core::ffi::c_void, interface: *const core::ffi::c_void, reserved: *const core::ffi::c_void, flags: u32, newservername: *mut windows_core::PWSTR, newserverport: *mut windows_core::PWSTR) -> RPC_STATUS>;
pub type RPC_OBJECT_INQ_FN = Option<unsafe extern "system" fn(objectuuid: *const windows_core::GUID, typeuuid: *mut windows_core::GUID, status: *mut RPC_STATUS)>;
pub type RPC_SECURITY_CALLBACK_FN = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type RPC_SETFILTER_FUNC = Option<unsafe extern "system" fn(pfnfilter: RPCLT_PDU_FILTER_FUNC)>;
pub type SERVER_ROUTINE = Option<unsafe extern "system" fn() -> i32>;
#[cfg(feature = "Win32_System_Com")]
pub type STUB_THUNK = Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
pub type USER_MARSHAL_FREEING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut core::ffi::c_void)>;
pub type USER_MARSHAL_MARSHALLING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u8, param2: *mut core::ffi::c_void) -> *mut u8>;
pub type USER_MARSHAL_SIZING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: u32, param2: *mut core::ffi::c_void) -> u32>;
pub type USER_MARSHAL_UNMARSHALLING_ROUTINE = Option<unsafe extern "system" fn(param0: *mut u32, param1: *mut u8, param2: *mut core::ffi::c_void) -> *mut u8>;
#[cfg(feature = "Win32_System_Com")]
pub type XMIT_HELPER_ROUTINE = Option<unsafe extern "system" fn(param0: *mut MIDL_STUB_MESSAGE)>;
