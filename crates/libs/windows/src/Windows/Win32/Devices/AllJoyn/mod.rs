pub const AJ_IFC_SECURITY_INHERIT: alljoyn_interfacedescription_securitypolicy = 0i32;
pub const AJ_IFC_SECURITY_OFF: alljoyn_interfacedescription_securitypolicy = 2i32;
pub const AJ_IFC_SECURITY_REQUIRED: alljoyn_interfacedescription_securitypolicy = 1i32;
pub const ALLJOYN_ARRAY: alljoyn_typeid = 97i32;
pub const ALLJOYN_BIG_ENDIAN: u8 = 66u8;
pub const ALLJOYN_BOOLEAN: alljoyn_typeid = 98i32;
pub const ALLJOYN_BOOLEAN_ARRAY: alljoyn_typeid = 25185i32;
pub const ALLJOYN_BYTE: alljoyn_typeid = 121i32;
pub const ALLJOYN_BYTE_ARRAY: alljoyn_typeid = 31073i32;
pub const ALLJOYN_CRED_CERT_CHAIN: u16 = 4u16;
pub const ALLJOYN_CRED_EXPIRATION: u16 = 32u16;
pub const ALLJOYN_CRED_LOGON_ENTRY: u16 = 16u16;
pub const ALLJOYN_CRED_NEW_PASSWORD: u16 = 4097u16;
pub const ALLJOYN_CRED_ONE_TIME_PWD: u16 = 8193u16;
pub const ALLJOYN_CRED_PASSWORD: u16 = 1u16;
pub const ALLJOYN_CRED_PRIVATE_KEY: u16 = 8u16;
pub const ALLJOYN_CRED_USER_NAME: u16 = 2u16;
pub const ALLJOYN_DICT_ENTRY: alljoyn_typeid = 101i32;
pub const ALLJOYN_DICT_ENTRY_CLOSE: alljoyn_typeid = 125i32;
pub const ALLJOYN_DICT_ENTRY_OPEN: alljoyn_typeid = 123i32;
pub const ALLJOYN_DISCONNECTED: u32 = 4u32;
pub const ALLJOYN_DOUBLE: alljoyn_typeid = 100i32;
pub const ALLJOYN_DOUBLE_ARRAY: alljoyn_typeid = 25697i32;
pub const ALLJOYN_HANDLE: alljoyn_typeid = 104i32;
pub const ALLJOYN_INT16: alljoyn_typeid = 110i32;
pub const ALLJOYN_INT16_ARRAY: alljoyn_typeid = 28257i32;
pub const ALLJOYN_INT32: alljoyn_typeid = 105i32;
pub const ALLJOYN_INT32_ARRAY: alljoyn_typeid = 26977i32;
pub const ALLJOYN_INT64: alljoyn_typeid = 120i32;
pub const ALLJOYN_INT64_ARRAY: alljoyn_typeid = 30817i32;
pub const ALLJOYN_INVALID: alljoyn_typeid = 0i32;
pub const ALLJOYN_LITTLE_ENDIAN: u8 = 108u8;
pub const ALLJOYN_MEMBER_ANNOTATE_DEPRECATED: u8 = 2u8;
pub const ALLJOYN_MEMBER_ANNOTATE_GLOBAL_BROADCAST: u8 = 32u8;
pub const ALLJOYN_MEMBER_ANNOTATE_NO_REPLY: u8 = 1u8;
pub const ALLJOYN_MEMBER_ANNOTATE_SESSIONCAST: u8 = 4u8;
pub const ALLJOYN_MEMBER_ANNOTATE_SESSIONLESS: u8 = 8u8;
pub const ALLJOYN_MEMBER_ANNOTATE_UNICAST: u8 = 16u8;
pub const ALLJOYN_MESSAGE_DEFAULT_TIMEOUT: u32 = 25000u32;
pub const ALLJOYN_MESSAGE_ERROR: alljoyn_messagetype = 3i32;
pub const ALLJOYN_MESSAGE_FLAG_ALLOW_REMOTE_MSG: u32 = 4u32;
pub const ALLJOYN_MESSAGE_FLAG_AUTO_START: u32 = 2u32;
pub const ALLJOYN_MESSAGE_FLAG_ENCRYPTED: u32 = 128u32;
pub const ALLJOYN_MESSAGE_FLAG_GLOBAL_BROADCAST: u32 = 32u32;
pub const ALLJOYN_MESSAGE_FLAG_NO_REPLY_EXPECTED: u32 = 1u32;
pub const ALLJOYN_MESSAGE_FLAG_SESSIONLESS: u32 = 16u32;
pub const ALLJOYN_MESSAGE_INVALID: alljoyn_messagetype = 0i32;
pub const ALLJOYN_MESSAGE_METHOD_CALL: alljoyn_messagetype = 1i32;
pub const ALLJOYN_MESSAGE_METHOD_RET: alljoyn_messagetype = 2i32;
pub const ALLJOYN_MESSAGE_SIGNAL: alljoyn_messagetype = 4i32;
pub const ALLJOYN_NAMED_PIPE_CONNECT_SPEC: windows_core::PCWSTR = windows_core::w!("npipe:");
pub const ALLJOYN_OBJECT_PATH: alljoyn_typeid = 111i32;
pub const ALLJOYN_PROP_ACCESS_READ: u8 = 1u8;
pub const ALLJOYN_PROP_ACCESS_RW: u8 = 3u8;
pub const ALLJOYN_PROP_ACCESS_WRITE: u8 = 2u8;
pub const ALLJOYN_PROXIMITY_ANY: u32 = 255u32;
pub const ALLJOYN_PROXIMITY_NETWORK: u32 = 2u32;
pub const ALLJOYN_PROXIMITY_PHYSICAL: u32 = 1u32;
pub const ALLJOYN_READ_READY: u32 = 1u32;
pub const ALLJOYN_SESSIONLOST_INVALID: alljoyn_sessionlostreason = 0i32;
pub const ALLJOYN_SESSIONLOST_LINK_TIMEOUT: alljoyn_sessionlostreason = 4i32;
pub const ALLJOYN_SESSIONLOST_REASON_OTHER: alljoyn_sessionlostreason = 5i32;
pub const ALLJOYN_SESSIONLOST_REMOTE_END_CLOSED_ABRUPTLY: alljoyn_sessionlostreason = 2i32;
pub const ALLJOYN_SESSIONLOST_REMOTE_END_LEFT_SESSION: alljoyn_sessionlostreason = 1i32;
pub const ALLJOYN_SESSIONLOST_REMOVED_BY_BINDER: alljoyn_sessionlostreason = 3i32;
pub const ALLJOYN_SIGNATURE: alljoyn_typeid = 103i32;
pub const ALLJOYN_STRING: alljoyn_typeid = 115i32;
pub const ALLJOYN_STRUCT: alljoyn_typeid = 114i32;
pub const ALLJOYN_STRUCT_CLOSE: alljoyn_typeid = 41i32;
pub const ALLJOYN_STRUCT_OPEN: alljoyn_typeid = 40i32;
pub const ALLJOYN_TRAFFIC_TYPE_MESSAGES: u32 = 1u32;
pub const ALLJOYN_TRAFFIC_TYPE_RAW_RELIABLE: u32 = 4u32;
pub const ALLJOYN_TRAFFIC_TYPE_RAW_UNRELIABLE: u32 = 2u32;
pub const ALLJOYN_UINT16: alljoyn_typeid = 113i32;
pub const ALLJOYN_UINT16_ARRAY: alljoyn_typeid = 29025i32;
pub const ALLJOYN_UINT32: alljoyn_typeid = 117i32;
pub const ALLJOYN_UINT32_ARRAY: alljoyn_typeid = 30049i32;
pub const ALLJOYN_UINT64: alljoyn_typeid = 116i32;
pub const ALLJOYN_UINT64_ARRAY: alljoyn_typeid = 29793i32;
pub const ALLJOYN_VARIANT: alljoyn_typeid = 118i32;
pub const ALLJOYN_WILDCARD: alljoyn_typeid = 42i32;
pub const ALLJOYN_WRITE_READY: u32 = 2u32;
pub const ANNOUNCED: alljoyn_about_announceflag = 1i32;
pub const CAPABLE_ECDHE_ECDSA: alljoyn_claimcapability_masks = 4i32;
pub const CAPABLE_ECDHE_NULL: alljoyn_claimcapability_masks = 1i32;
pub const CAPABLE_ECDHE_SPEKE: alljoyn_claimcapability_masks = 8i32;
pub const CLAIMABLE: alljoyn_applicationstate = 1i32;
pub const CLAIMED: alljoyn_applicationstate = 2i32;
pub const ER_ABOUT_ABOUTDATA_MISSING_REQUIRED_FIELD: QStatus = 37157i32;
pub const ER_ABOUT_DEFAULT_LANGUAGE_NOT_SPECIFIED: QStatus = 37155i32;
pub const ER_ABOUT_FIELD_ALREADY_SPECIFIED: QStatus = 37147i32;
pub const ER_ABOUT_INVALID_ABOUTDATA_FIELD_APPID_SIZE: QStatus = 37163i32;
pub const ER_ABOUT_INVALID_ABOUTDATA_FIELD_VALUE: QStatus = 37162i32;
pub const ER_ABOUT_INVALID_ABOUTDATA_LISTENER: QStatus = 37158i32;
pub const ER_ABOUT_SESSIONPORT_NOT_BOUND: QStatus = 37156i32;
pub const ER_ALERTED_THREAD: QStatus = 4098i32;
pub const ER_ALLJOYN_ACCESS_PERMISSION_ERROR: QStatus = 37028i32;
pub const ER_ALLJOYN_ACCESS_PERMISSION_WARNING: QStatus = 37027i32;
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_ALREADY_ADVERTISING: QStatus = 37005i32;
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_FAILED: QStatus = 37006i32;
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_TRANSPORT_NOT_AVAILABLE: QStatus = 37004i32;
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_ALREADY_EXISTS: QStatus = 36992i32;
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_FAILED: QStatus = 36993i32;
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_INVALID_OPTS: QStatus = 37018i32;
pub const ER_ALLJOYN_CANCELADVERTISENAME_REPLY_FAILED: QStatus = 37008i32;
pub const ER_ALLJOYN_CANCELFINDADVERTISEDNAME_REPLY_FAILED: QStatus = 37013i32;
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_ALREADY_DISCOVERING: QStatus = 37010i32;
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_FAILED: QStatus = 37011i32;
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_TRANSPORT_NOT_AVAILABLE: QStatus = 37009i32;
pub const ER_ALLJOYN_JOINSESSION_REPLY_ALREADY_JOINED: QStatus = 37019i32;
pub const ER_ALLJOYN_JOINSESSION_REPLY_BAD_SESSION_OPTS: QStatus = 36999i32;
pub const ER_ALLJOYN_JOINSESSION_REPLY_CONNECT_FAILED: QStatus = 36997i32;
pub const ER_ALLJOYN_JOINSESSION_REPLY_FAILED: QStatus = 37000i32;
pub const ER_ALLJOYN_JOINSESSION_REPLY_NO_SESSION: QStatus = 36995i32;
pub const ER_ALLJOYN_JOINSESSION_REPLY_REJECTED: QStatus = 36998i32;
pub const ER_ALLJOYN_JOINSESSION_REPLY_UNREACHABLE: QStatus = 36996i32;
pub const ER_ALLJOYN_LEAVESESSION_REPLY_FAILED: QStatus = 37003i32;
pub const ER_ALLJOYN_LEAVESESSION_REPLY_NO_SESSION: QStatus = 37002i32;
pub const ER_ALLJOYN_ONAPPRESUME_REPLY_FAILED: QStatus = 37100i32;
pub const ER_ALLJOYN_ONAPPRESUME_REPLY_UNSUPPORTED: QStatus = 37101i32;
pub const ER_ALLJOYN_ONAPPSUSPEND_REPLY_FAILED: QStatus = 37098i32;
pub const ER_ALLJOYN_ONAPPSUSPEND_REPLY_UNSUPPORTED: QStatus = 37099i32;
pub const ER_ALLJOYN_PING_FAILED: QStatus = 37111i32;
pub const ER_ALLJOYN_PING_REPLY_FAILED: QStatus = 37143i32;
pub const ER_ALLJOYN_PING_REPLY_INCOMPATIBLE_REMOTE_ROUTING_NODE: QStatus = 37140i32;
pub const ER_ALLJOYN_PING_REPLY_IN_PROGRESS: QStatus = 37145i32;
pub const ER_ALLJOYN_PING_REPLY_TIMEOUT: QStatus = 37141i32;
pub const ER_ALLJOYN_PING_REPLY_UNKNOWN_NAME: QStatus = 37142i32;
pub const ER_ALLJOYN_PING_REPLY_UNREACHABLE: QStatus = 37112i32;
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_INCOMPATIBLE_REMOTE_DAEMON: QStatus = 37107i32;
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_BINDER: QStatus = 37104i32;
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_FOUND: QStatus = 37106i32;
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_MULTIPOINT: QStatus = 37105i32;
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_REPLY_FAILED: QStatus = 37108i32;
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_REPLY_NO_SESSION: QStatus = 37103i32;
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_FAILED: QStatus = 37026i32;
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_NOT_SUPPORTED: QStatus = 37024i32;
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_NO_DEST_SUPPORT: QStatus = 37025i32;
pub const ER_ALLJOYN_UNBINDSESSIONPORT_REPLY_BAD_PORT: QStatus = 37016i32;
pub const ER_ALLJOYN_UNBINDSESSIONPORT_REPLY_FAILED: QStatus = 37017i32;
pub const ER_APPLICATION_STATE_LISTENER_ALREADY_EXISTS: QStatus = 37184i32;
pub const ER_APPLICATION_STATE_LISTENER_NO_SUCH_LISTENER: QStatus = 37185i32;
pub const ER_ARDP_BACKPRESSURE: QStatus = 37122i32;
pub const ER_ARDP_DISCONNECTING: QStatus = 37139i32;
pub const ER_ARDP_INVALID_CONNECTION: QStatus = 37135i32;
pub const ER_ARDP_INVALID_RESPONSE: QStatus = 37134i32;
pub const ER_ARDP_INVALID_STATE: QStatus = 37124i32;
pub const ER_ARDP_PERSIST_TIMEOUT: QStatus = 37126i32;
pub const ER_ARDP_PROBE_TIMEOUT: QStatus = 37127i32;
pub const ER_ARDP_REMOTE_CONNECTION_RESET: QStatus = 37128i32;
pub const ER_ARDP_TTL_EXPIRED: QStatus = 37125i32;
pub const ER_ARDP_VERSION_NOT_SUPPORTED: QStatus = 37151i32;
pub const ER_ARDP_WRITE_BLOCKED: QStatus = 37153i32;
pub const ER_AUTH_FAIL: QStatus = 4100i32;
pub const ER_AUTH_USER_REJECT: QStatus = 4101i32;
pub const ER_BAD_ARG_1: QStatus = 12i32;
pub const ER_BAD_ARG_2: QStatus = 13i32;
pub const ER_BAD_ARG_3: QStatus = 14i32;
pub const ER_BAD_ARG_4: QStatus = 15i32;
pub const ER_BAD_ARG_5: QStatus = 16i32;
pub const ER_BAD_ARG_6: QStatus = 17i32;
pub const ER_BAD_ARG_7: QStatus = 18i32;
pub const ER_BAD_ARG_8: QStatus = 19i32;
pub const ER_BAD_ARG_COUNT: QStatus = 28i32;
pub const ER_BAD_HOSTNAME: QStatus = 4112i32;
pub const ER_BAD_STRING_ENCODING: QStatus = 4120i32;
pub const ER_BAD_TRANSPORT_MASK: QStatus = 37088i32;
pub const ER_BUFFER_TOO_SMALL: QStatus = 3i32;
pub const ER_BUS_ALREADY_CONNECTED: QStatus = 36932i32;
pub const ER_BUS_ALREADY_LISTENING: QStatus = 36934i32;
pub const ER_BUS_ANNOTATION_ALREADY_EXISTS: QStatus = 37082i32;
pub const ER_BUS_AUTHENTICATION_PENDING: QStatus = 37031i32;
pub const ER_BUS_BAD_BODY_LEN: QStatus = 36879i32;
pub const ER_BUS_BAD_BUS_NAME: QStatus = 36874i32;
pub const ER_BUS_BAD_CHILD_PATH: QStatus = 36927i32;
pub const ER_BUS_BAD_ERROR_NAME: QStatus = 36873i32;
pub const ER_BUS_BAD_HDR_FLAGS: QStatus = 36878i32;
pub const ER_BUS_BAD_HEADER_FIELD: QStatus = 36868i32;
pub const ER_BUS_BAD_HEADER_LEN: QStatus = 36880i32;
pub const ER_BUS_BAD_INTERFACE_NAME: QStatus = 36872i32;
pub const ER_BUS_BAD_LENGTH: QStatus = 36876i32;
pub const ER_BUS_BAD_MEMBER_NAME: QStatus = 36871i32;
pub const ER_BUS_BAD_OBJ_PATH: QStatus = 36870i32;
pub const ER_BUS_BAD_SENDER_ID: QStatus = 36908i32;
pub const ER_BUS_BAD_SEND_PARAMETER: QStatus = 36906i32;
pub const ER_BUS_BAD_SESSION_OPTS: QStatus = 36980i32;
pub const ER_BUS_BAD_SIGNATURE: QStatus = 36869i32;
pub const ER_BUS_BAD_TRANSPORT_ARGS: QStatus = 36903i32;
pub const ER_BUS_BAD_VALUE: QStatus = 36877i32;
pub const ER_BUS_BAD_VALUE_TYPE: QStatus = 36867i32;
pub const ER_BUS_BAD_XML: QStatus = 36926i32;
pub const ER_BUS_BLOCKING_CALL_NOT_ALLOWED: QStatus = 36960i32;
pub const ER_BUS_BUS_ALREADY_STARTED: QStatus = 36939i32;
pub const ER_BUS_BUS_NOT_STARTED: QStatus = 36940i32;
pub const ER_BUS_CANNOT_ADD_HANDLER: QStatus = 36965i32;
pub const ER_BUS_CANNOT_ADD_INTERFACE: QStatus = 36964i32;
pub const ER_BUS_CANNOT_EXPAND_MESSAGE: QStatus = 36930i32;
pub const ER_BUS_CONNECTION_REJECTED: QStatus = 36981i32;
pub const ER_BUS_CONNECT_FAILED: QStatus = 36913i32;
pub const ER_BUS_CORRUPT_KEYSTORE: QStatus = 36952i32;
pub const ER_BUS_DESCRIPTION_ALREADY_EXISTS: QStatus = 37188i32;
pub const ER_BUS_DESTINATION_NOT_AUTHENTICATED: QStatus = 37029i32;
pub const ER_BUS_ELEMENT_NOT_FOUND: QStatus = 36976i32;
pub const ER_BUS_EMPTY_MESSAGE: QStatus = 36910i32;
pub const ER_BUS_ENDPOINT_CLOSING: QStatus = 36920i32;
pub const ER_BUS_ENDPOINT_REDIRECTED: QStatus = 37030i32;
pub const ER_BUS_ERRORS: QStatus = 36864i32;
pub const ER_BUS_ERROR_NAME_MISSING: QStatus = 36890i32;
pub const ER_BUS_ERROR_RESPONSE: QStatus = 36925i32;
pub const ER_BUS_ESTABLISH_FAILED: QStatus = 36884i32;
pub const ER_BUS_HANDLES_MISMATCH: QStatus = 36973i32;
pub const ER_BUS_HANDLES_NOT_ENABLED: QStatus = 36972i32;
pub const ER_BUS_HDR_EXPANSION_INVALID: QStatus = 36946i32;
pub const ER_BUS_IFACE_ALREADY_EXISTS: QStatus = 36924i32;
pub const ER_BUS_INCOMPATIBLE_DAEMON: QStatus = 37094i32;
pub const ER_BUS_INTERFACE_ACTIVATED: QStatus = 37015i32;
pub const ER_BUS_INTERFACE_MISMATCH: QStatus = 36921i32;
pub const ER_BUS_INTERFACE_MISSING: QStatus = 36886i32;
pub const ER_BUS_INTERFACE_NO_SUCH_MEMBER: QStatus = 36891i32;
pub const ER_BUS_INVALID_AUTH_MECHANISM: QStatus = 36958i32;
pub const ER_BUS_INVALID_HEADER_CHECKSUM: QStatus = 36942i32;
pub const ER_BUS_INVALID_HEADER_SERIAL: QStatus = 36944i32;
pub const ER_BUS_KEYBLOB_OP_INVALID: QStatus = 36941i32;
pub const ER_BUS_KEYSTORE_NOT_LOADED: QStatus = 36966i32;
pub const ER_BUS_KEYSTORE_VERSION_MISMATCH: QStatus = 36959i32;
pub const ER_BUS_KEY_EXPIRED: QStatus = 36951i32;
pub const ER_BUS_KEY_STORE_NOT_LOADED: QStatus = 36937i32;
pub const ER_BUS_KEY_UNAVAILABLE: QStatus = 36935i32;
pub const ER_BUS_LISTENER_ALREADY_SET: QStatus = 37022i32;
pub const ER_BUS_MATCH_RULE_NOT_FOUND: QStatus = 37110i32;
pub const ER_BUS_MEMBER_ALREADY_EXISTS: QStatus = 36922i32;
pub const ER_BUS_MEMBER_MISSING: QStatus = 36888i32;
pub const ER_BUS_MEMBER_NO_SUCH_SIGNATURE: QStatus = 36896i32;
pub const ER_BUS_MESSAGE_DECRYPTION_FAILED: QStatus = 36949i32;
pub const ER_BUS_MESSAGE_NOT_ENCRYPTED: QStatus = 36943i32;
pub const ER_BUS_METHOD_CALL_ABORTED: QStatus = 36963i32;
pub const ER_BUS_MISSING_COMPRESSION_TOKEN: QStatus = 36947i32;
pub const ER_BUS_NAME_TOO_LONG: QStatus = 36875i32;
pub const ER_BUS_NOT_ALLOWED: QStatus = 36918i32;
pub const ER_BUS_NOT_AUTHENTICATING: QStatus = 36915i32;
pub const ER_BUS_NOT_AUTHORIZED: QStatus = 37032i32;
pub const ER_BUS_NOT_A_COMPLETE_TYPE: QStatus = 36954i32;
pub const ER_BUS_NOT_A_DICTIONARY: QStatus = 36977i32;
pub const ER_BUS_NOT_COMPRESSED: QStatus = 36931i32;
pub const ER_BUS_NOT_CONNECTED: QStatus = 36933i32;
pub const ER_BUS_NOT_NUL_TERMINATED: QStatus = 36897i32;
pub const ER_BUS_NOT_OWNER: QStatus = 36911i32;
pub const ER_BUS_NO_AUTHENTICATION_MECHANISM: QStatus = 36938i32;
pub const ER_BUS_NO_CALL_FOR_REPLY: QStatus = 36953i32;
pub const ER_BUS_NO_ENDPOINT: QStatus = 36905i32;
pub const ER_BUS_NO_LISTENER: QStatus = 36916i32;
pub const ER_BUS_NO_PEER_GUID: QStatus = 36948i32;
pub const ER_BUS_NO_ROUTE: QStatus = 36904i32;
pub const ER_BUS_NO_SESSION: QStatus = 36975i32;
pub const ER_BUS_NO_SUCH_ANNOTATION: QStatus = 37081i32;
pub const ER_BUS_NO_SUCH_HANDLE: QStatus = 36971i32;
pub const ER_BUS_NO_SUCH_INTERFACE: QStatus = 36895i32;
pub const ER_BUS_NO_SUCH_MESSAGE: QStatus = 37102i32;
pub const ER_BUS_NO_SUCH_OBJECT: QStatus = 36892i32;
pub const ER_BUS_NO_SUCH_PROPERTY: QStatus = 36898i32;
pub const ER_BUS_NO_SUCH_SERVICE: QStatus = 36956i32;
pub const ER_BUS_NO_TRANSPORTS: QStatus = 36902i32;
pub const ER_BUS_OBJECT_NOT_REGISTERED: QStatus = 37091i32;
pub const ER_BUS_OBJECT_NO_SUCH_INTERFACE: QStatus = 36894i32;
pub const ER_BUS_OBJECT_NO_SUCH_MEMBER: QStatus = 36893i32;
pub const ER_BUS_OBJ_ALREADY_EXISTS: QStatus = 36928i32;
pub const ER_BUS_OBJ_NOT_FOUND: QStatus = 36929i32;
pub const ER_BUS_PATH_MISSING: QStatus = 36887i32;
pub const ER_BUS_PEER_AUTH_VERSION_MISMATCH: QStatus = 37023i32;
pub const ER_BUS_PING_GROUP_NOT_FOUND: QStatus = 37159i32;
pub const ER_BUS_POLICY_VIOLATION: QStatus = 36955i32;
pub const ER_BUS_PROPERTY_ACCESS_DENIED: QStatus = 36901i32;
pub const ER_BUS_PROPERTY_ALREADY_EXISTS: QStatus = 36923i32;
pub const ER_BUS_PROPERTY_VALUE_NOT_SET: QStatus = 36900i32;
pub const ER_BUS_READ_ERROR: QStatus = 36865i32;
pub const ER_BUS_REMOVED_BY_BINDER: QStatus = 37109i32;
pub const ER_BUS_REMOVED_BY_BINDER_SELF: QStatus = 37160i32;
pub const ER_BUS_REPLY_IS_ERROR_MESSAGE: QStatus = 36914i32;
pub const ER_BUS_REPLY_SERIAL_MISSING: QStatus = 36889i32;
pub const ER_BUS_SECURITY_FATAL: QStatus = 36950i32;
pub const ER_BUS_SECURITY_NOT_ENABLED: QStatus = 37021i32;
pub const ER_BUS_SELF_CONNECT: QStatus = 37020i32;
pub const ER_BUS_SET_PROPERTY_REJECTED: QStatus = 36912i32;
pub const ER_BUS_SET_WRONG_SIGNATURE: QStatus = 36899i32;
pub const ER_BUS_SIGNATURE_MISMATCH: QStatus = 36961i32;
pub const ER_BUS_STOPPING: QStatus = 36962i32;
pub const ER_BUS_TIME_TO_LIVE_EXPIRED: QStatus = 36945i32;
pub const ER_BUS_TRANSPORT_ACCESS_DENIED: QStatus = 37164i32;
pub const ER_BUS_TRANSPORT_NOT_AVAILABLE: QStatus = 36957i32;
pub const ER_BUS_TRANSPORT_NOT_STARTED: QStatus = 36909i32;
pub const ER_BUS_TRUNCATED: QStatus = 36936i32;
pub const ER_BUS_UNEXPECTED_DISPOSITION: QStatus = 37014i32;
pub const ER_BUS_UNEXPECTED_SIGNATURE: QStatus = 36885i32;
pub const ER_BUS_UNKNOWN_INTERFACE: QStatus = 36883i32;
pub const ER_BUS_UNKNOWN_PATH: QStatus = 36882i32;
pub const ER_BUS_UNKNOWN_SERIAL: QStatus = 36881i32;
pub const ER_BUS_UNMATCHED_REPLY_SERIAL: QStatus = 36907i32;
pub const ER_BUS_WAIT_FAILED: QStatus = 36978i32;
pub const ER_BUS_WRITE_ERROR: QStatus = 36866i32;
pub const ER_BUS_WRITE_QUEUE_FULL: QStatus = 36919i32;
pub const ER_CERTIFICATE_NOT_FOUND: QStatus = 37166i32;
pub const ER_COMMON_ERRORS: QStatus = 4096i32;
pub const ER_CONNECTION_LIMIT_EXCEEDED: QStatus = 37152i32;
pub const ER_CONN_REFUSED: QStatus = 27i32;
pub const ER_CORRUPT_KEYBLOB: QStatus = 4115i32;
pub const ER_CRYPTO_ERROR: QStatus = 4109i32;
pub const ER_CRYPTO_HASH_UNINITIALIZED: QStatus = 4123i32;
pub const ER_CRYPTO_ILLEGAL_PARAMETERS: QStatus = 4122i32;
pub const ER_CRYPTO_INSUFFICIENT_SECURITY: QStatus = 4121i32;
pub const ER_CRYPTO_KEY_UNAVAILABLE: QStatus = 4111i32;
pub const ER_CRYPTO_KEY_UNUSABLE: QStatus = 4113i32;
pub const ER_CRYPTO_TRUNCATED: QStatus = 4110i32;
pub const ER_DBUS_RELEASE_NAME_REPLY_NON_EXISTENT: QStatus = 36987i32;
pub const ER_DBUS_RELEASE_NAME_REPLY_NOT_OWNER: QStatus = 36988i32;
pub const ER_DBUS_RELEASE_NAME_REPLY_RELEASED: QStatus = 36986i32;
pub const ER_DBUS_REQUEST_NAME_REPLY_ALREADY_OWNER: QStatus = 36985i32;
pub const ER_DBUS_REQUEST_NAME_REPLY_EXISTS: QStatus = 36984i32;
pub const ER_DBUS_REQUEST_NAME_REPLY_IN_QUEUE: QStatus = 36983i32;
pub const ER_DBUS_REQUEST_NAME_REPLY_PRIMARY_OWNER: QStatus = 36982i32;
pub const ER_DBUS_START_REPLY_ALREADY_RUNNING: QStatus = 36990i32;
pub const ER_DEADLOCK: QStatus = 31i32;
pub const ER_DEAD_THREAD: QStatus = 4117i32;
pub const ER_DIGEST_MISMATCH: QStatus = 37170i32;
pub const ER_DUPLICATE_CERTIFICATE: QStatus = 37167i32;
pub const ER_DUPLICATE_KEY: QStatus = 37171i32;
pub const ER_EMPTY_KEY_BLOB: QStatus = 4114i32;
pub const ER_END_OF_DATA: QStatus = 26i32;
pub const ER_EOF: QStatus = 30i32;
pub const ER_EXTERNAL_THREAD: QStatus = 4108i32;
pub const ER_FAIL: QStatus = 1i32;
pub const ER_FEATURE_NOT_AVAILABLE: QStatus = 37177i32;
pub const ER_INIT_FAILED: QStatus = 7i32;
pub const ER_INVALID_ADDRESS: QStatus = 20i32;
pub const ER_INVALID_APPLICATION_STATE: QStatus = 37176i32;
pub const ER_INVALID_CERTIFICATE: QStatus = 37165i32;
pub const ER_INVALID_CERTIFICATE_USAGE: QStatus = 37182i32;
pub const ER_INVALID_CERT_CHAIN: QStatus = 37174i32;
pub const ER_INVALID_CONFIG: QStatus = 37161i32;
pub const ER_INVALID_DATA: QStatus = 21i32;
pub const ER_INVALID_GUID: QStatus = 4126i32;
pub const ER_INVALID_HTTP_METHOD_USED_FOR_RENDEZVOUS_SERVER_INTERFACE_MESSAGE: QStatus = 37075i32;
pub const ER_INVALID_KEY_ENCODING: QStatus = 4116i32;
pub const ER_INVALID_ON_DEMAND_CONNECTION_MESSAGE_RESPONSE: QStatus = 37074i32;
pub const ER_INVALID_PERSISTENT_CONNECTION_MESSAGE_RESPONSE: QStatus = 37073i32;
pub const ER_INVALID_RENDEZVOUS_SERVER_INTERFACE_MESSAGE: QStatus = 37072i32;
pub const ER_INVALID_SIGNAL_EMISSION_TYPE: QStatus = 37183i32;
pub const ER_INVALID_STREAM: QStatus = 4129i32;
pub const ER_IODISPATCH_STOPPING: QStatus = 4131i32;
pub const ER_KEY_STORE_ALREADY_INITIALIZED: QStatus = 37178i32;
pub const ER_KEY_STORE_ID_NOT_YET_SET: QStatus = 37179i32;
pub const ER_LANGUAGE_NOT_SUPPORTED: QStatus = 37146i32;
pub const ER_MANAGEMENT_ALREADY_STARTED: QStatus = 37186i32;
pub const ER_MANAGEMENT_NOT_STARTED: QStatus = 37187i32;
pub const ER_MANIFEST_NOT_FOUND: QStatus = 37173i32;
pub const ER_MANIFEST_REJECTED: QStatus = 37181i32;
pub const ER_MISSING_DIGEST_IN_CERTIFICATE: QStatus = 37169i32;
pub const ER_NONE: QStatus = 65535i32;
pub const ER_NOT_CONN: QStatus = 4141i32;
pub const ER_NOT_CONNECTED_TO_RENDEZVOUS_SERVER: QStatus = 37070i32;
pub const ER_NOT_IMPLEMENTED: QStatus = 9i32;
pub const ER_NO_COMMON_TRUST: QStatus = 37172i32;
pub const ER_NO_SUCH_ALARM: QStatus = 4102i32;
pub const ER_NO_SUCH_DEVICE: QStatus = 37084i32;
pub const ER_NO_TRUST_ANCHOR: QStatus = 37175i32;
pub const ER_OK: QStatus = 0i32;
pub const ER_OPEN_FAILED: QStatus = 24i32;
pub const ER_OS_ERROR: QStatus = 4i32;
pub const ER_OUT_OF_MEMORY: QStatus = 5i32;
pub const ER_P2P: QStatus = 37085i32;
pub const ER_P2P_BUSY: QStatus = 37093i32;
pub const ER_P2P_DISABLED: QStatus = 37092i32;
pub const ER_P2P_FORBIDDEN: QStatus = 37097i32;
pub const ER_P2P_NOT_CONNECTED: QStatus = 37087i32;
pub const ER_P2P_NO_GO: QStatus = 37095i32;
pub const ER_P2P_NO_STA: QStatus = 37096i32;
pub const ER_P2P_TIMEOUT: QStatus = 37086i32;
pub const ER_PACKET_BAD_CRC: QStatus = 37039i32;
pub const ER_PACKET_BAD_FORMAT: QStatus = 37034i32;
pub const ER_PACKET_BAD_PARAMETER: QStatus = 37038i32;
pub const ER_PACKET_BUS_NO_SUCH_CHANNEL: QStatus = 37033i32;
pub const ER_PACKET_CHANNEL_FAIL: QStatus = 37036i32;
pub const ER_PACKET_CONNECT_TIMEOUT: QStatus = 37035i32;
pub const ER_PACKET_TOO_LARGE: QStatus = 37037i32;
pub const ER_PARSE_ERROR: QStatus = 25i32;
pub const ER_PERMISSION_DENIED: QStatus = 37154i32;
pub const ER_POLICY_NOT_NEWER: QStatus = 37180i32;
pub const ER_PROXIMITY_CONNECTION_ESTABLISH_FAIL: QStatus = 37089i32;
pub const ER_PROXIMITY_NO_PEERS_FOUND: QStatus = 37090i32;
pub const ER_READ_ERROR: QStatus = 22i32;
pub const ER_RENDEZVOUS_SERVER_DEACTIVATED_USER: QStatus = 37067i32;
pub const ER_RENDEZVOUS_SERVER_ERR401_UNAUTHORIZED_REQUEST: QStatus = 37078i32;
pub const ER_RENDEZVOUS_SERVER_ERR500_INTERNAL_ERROR: QStatus = 37076i32;
pub const ER_RENDEZVOUS_SERVER_ERR503_STATUS_UNAVAILABLE: QStatus = 37077i32;
pub const ER_RENDEZVOUS_SERVER_ROOT_CERTIFICATE_UNINITIALIZED: QStatus = 37080i32;
pub const ER_RENDEZVOUS_SERVER_UNKNOWN_USER: QStatus = 37068i32;
pub const ER_RENDEZVOUS_SERVER_UNRECOVERABLE_ERROR: QStatus = 37079i32;
pub const ER_SLAP_CRC_ERROR: QStatus = 4137i32;
pub const ER_SLAP_ERROR: QStatus = 4138i32;
pub const ER_SLAP_HDR_CHECKSUM_ERROR: QStatus = 4133i32;
pub const ER_SLAP_INVALID_PACKET_LEN: QStatus = 4132i32;
pub const ER_SLAP_INVALID_PACKET_TYPE: QStatus = 4134i32;
pub const ER_SLAP_LEN_MISMATCH: QStatus = 4135i32;
pub const ER_SLAP_OTHER_END_CLOSED: QStatus = 4139i32;
pub const ER_SLAP_PACKET_TYPE_MISMATCH: QStatus = 4136i32;
pub const ER_SOCKET_BIND_ERROR: QStatus = 6i32;
pub const ER_SOCK_CLOSING: QStatus = 37083i32;
pub const ER_SOCK_OTHER_END_CLOSED: QStatus = 11i32;
pub const ER_SSL_CONNECT: QStatus = 4106i32;
pub const ER_SSL_ERRORS: QStatus = 4104i32;
pub const ER_SSL_INIT: QStatus = 4105i32;
pub const ER_SSL_VERIFY: QStatus = 4107i32;
pub const ER_STOPPING_THREAD: QStatus = 4097i32;
pub const ER_TCP_MAX_UNTRUSTED: QStatus = 37144i32;
pub const ER_THREADPOOL_EXHAUSTED: QStatus = 4127i32;
pub const ER_THREADPOOL_STOPPING: QStatus = 4128i32;
pub const ER_THREAD_NO_WAIT: QStatus = 4124i32;
pub const ER_THREAD_RUNNING: QStatus = 4118i32;
pub const ER_THREAD_STOPPING: QStatus = 4119i32;
pub const ER_TIMEOUT: QStatus = 10i32;
pub const ER_TIMER_EXITING: QStatus = 4125i32;
pub const ER_TIMER_FALLBEHIND: QStatus = 4103i32;
pub const ER_TIMER_FULL: QStatus = 4130i32;
pub const ER_TIMER_NOT_ALLOWED: QStatus = 4140i32;
pub const ER_UDP_BACKPRESSURE: QStatus = 37123i32;
pub const ER_UDP_BUSHELLO: QStatus = 37129i32;
pub const ER_UDP_DEMUX_NO_ENDPOINT: QStatus = 37114i32;
pub const ER_UDP_DISCONNECT: QStatus = 37118i32;
pub const ER_UDP_EARLY_EXIT: QStatus = 37137i32;
pub const ER_UDP_ENDPOINT_NOT_STARTED: QStatus = 37149i32;
pub const ER_UDP_ENDPOINT_REMOVED: QStatus = 37150i32;
pub const ER_UDP_ENDPOINT_STALLED: QStatus = 37133i32;
pub const ER_UDP_INVALID: QStatus = 37131i32;
pub const ER_UDP_LOCAL_DISCONNECT: QStatus = 37136i32;
pub const ER_UDP_LOCAL_DISCONNECT_FAIL: QStatus = 37138i32;
pub const ER_UDP_MESSAGE: QStatus = 37130i32;
pub const ER_UDP_MSG_TOO_LONG: QStatus = 37113i32;
pub const ER_UDP_NOT_DISCONNECTED: QStatus = 37148i32;
pub const ER_UDP_NOT_IMPLEMENTED: QStatus = 37119i32;
pub const ER_UDP_NO_LISTENER: QStatus = 37120i32;
pub const ER_UDP_NO_NETWORK: QStatus = 37115i32;
pub const ER_UDP_STOPPING: QStatus = 37121i32;
pub const ER_UDP_UNEXPECTED_FLOW: QStatus = 37117i32;
pub const ER_UDP_UNEXPECTED_LENGTH: QStatus = 37116i32;
pub const ER_UDP_UNSUPPORTED: QStatus = 37132i32;
pub const ER_UNABLE_TO_CONNECT_TO_RENDEZVOUS_SERVER: QStatus = 37069i32;
pub const ER_UNABLE_TO_SEND_MESSAGE_TO_RENDEZVOUS_SERVER: QStatus = 37071i32;
pub const ER_UNKNOWN_CERTIFICATE: QStatus = 37168i32;
pub const ER_UTF_CONVERSION_FAILED: QStatus = 2i32;
pub const ER_WARNING: QStatus = 29i32;
pub const ER_WOULDBLOCK: QStatus = 8i32;
pub const ER_WRITE_ERROR: QStatus = 23i32;
pub const ER_XML_ACLS_MISSING: QStatus = 8211i32;
pub const ER_XML_ACL_ALL_TYPE_PEER_WITH_OTHERS: QStatus = 8207i32;
pub const ER_XML_ACL_PEERS_MISSING: QStatus = 8212i32;
pub const ER_XML_ACL_PEER_NOT_UNIQUE: QStatus = 8209i32;
pub const ER_XML_ACL_PEER_PUBLIC_KEY_SET: QStatus = 8210i32;
pub const ER_XML_ANNOTATION_NOT_UNIQUE: QStatus = 8222i32;
pub const ER_XML_CONVERTER_ERROR: QStatus = 8192i32;
pub const ER_XML_INTERFACE_MEMBERS_MISSING: QStatus = 8194i32;
pub const ER_XML_INTERFACE_NAME_NOT_UNIQUE: QStatus = 8219i32;
pub const ER_XML_INVALID_ACL_PEER_CHILDREN_COUNT: QStatus = 8206i32;
pub const ER_XML_INVALID_ACL_PEER_PUBLIC_KEY: QStatus = 8208i32;
pub const ER_XML_INVALID_ACL_PEER_TYPE: QStatus = 8205i32;
pub const ER_XML_INVALID_ANNOTATIONS_COUNT: QStatus = 8198i32;
pub const ER_XML_INVALID_ATTRIBUTE_VALUE: QStatus = 8200i32;
pub const ER_XML_INVALID_BASE64: QStatus = 8218i32;
pub const ER_XML_INVALID_ELEMENT_CHILDREN_COUNT: QStatus = 8202i32;
pub const ER_XML_INVALID_ELEMENT_NAME: QStatus = 8199i32;
pub const ER_XML_INVALID_INTERFACE_NAME: QStatus = 8214i32;
pub const ER_XML_INVALID_MANIFEST_VERSION: QStatus = 8216i32;
pub const ER_XML_INVALID_MEMBER_ACTION: QStatus = 8196i32;
pub const ER_XML_INVALID_MEMBER_NAME: QStatus = 8215i32;
pub const ER_XML_INVALID_MEMBER_TYPE: QStatus = 8195i32;
pub const ER_XML_INVALID_OBJECT_PATH: QStatus = 8213i32;
pub const ER_XML_INVALID_OID: QStatus = 8217i32;
pub const ER_XML_INVALID_POLICY_SERIAL_NUMBER: QStatus = 8204i32;
pub const ER_XML_INVALID_POLICY_VERSION: QStatus = 8203i32;
pub const ER_XML_INVALID_RULES_COUNT: QStatus = 8193i32;
pub const ER_XML_INVALID_SECURITY_LEVEL_ANNOTATION_VALUE: QStatus = 8201i32;
pub const ER_XML_MALFORMED: QStatus = 4099i32;
pub const ER_XML_MEMBER_DENY_ACTION_WITH_OTHER: QStatus = 8197i32;
pub const ER_XML_MEMBER_NAME_NOT_UNIQUE: QStatus = 8220i32;
pub const ER_XML_OBJECT_PATH_NOT_UNIQUE: QStatus = 8221i32;
pub const NEED_UPDATE: alljoyn_applicationstate = 3i32;
pub const NOT_CLAIMABLE: alljoyn_applicationstate = 0i32;
pub const PASSWORD_GENERATED_BY_APPLICATION: alljoyn_claimcapabilityadditionalinfo_masks = 2i32;
pub const PASSWORD_GENERATED_BY_SECURITY_MANAGER: alljoyn_claimcapabilityadditionalinfo_masks = 1i32;
pub const QCC_FALSE: u32 = 0u32;
pub const QCC_TRUE: u32 = 1u32;
pub const UNANNOUNCED: alljoyn_about_announceflag = 0i32;
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct QStatus(pub i32);
impl windows_core::TypeKind for QStatus {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct alljoyn_about_announceflag(pub i32);
impl windows_core::TypeKind for alljoyn_about_announceflag {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct alljoyn_applicationstate(pub i32);
impl windows_core::TypeKind for alljoyn_applicationstate {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct alljoyn_claimcapability_masks(pub i32);
impl windows_core::TypeKind for alljoyn_claimcapability_masks {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct alljoyn_claimcapabilityadditionalinfo_masks(pub i32);
impl windows_core::TypeKind for alljoyn_claimcapabilityadditionalinfo_masks {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct alljoyn_interfacedescription_securitypolicy(pub i32);
impl windows_core::TypeKind for alljoyn_interfacedescription_securitypolicy {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct alljoyn_messagetype(pub i32);
impl windows_core::TypeKind for alljoyn_messagetype {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct alljoyn_sessionlostreason(pub i32);
impl windows_core::TypeKind for alljoyn_sessionlostreason {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct alljoyn_typeid(pub i32);
impl windows_core::TypeKind for alljoyn_typeid {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_aboutdatalistener_callbacks {
    pub about_datalistener_getaboutdata: alljoyn_aboutdatalistener_getaboutdata_ptr,
    pub about_datalistener_getannouncedaboutdata: alljoyn_aboutdatalistener_getannouncedaboutdata_ptr,
}
impl Default for alljoyn_aboutdatalistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_aboutdatalistener_callbacks {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_aboutlistener_callback {
    pub about_listener_announced: alljoyn_about_announced_ptr,
}
impl Default for alljoyn_aboutlistener_callback {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_aboutlistener_callback {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_applicationstatelistener_callbacks {
    pub state: alljoyn_applicationstatelistener_state_ptr,
}
impl Default for alljoyn_applicationstatelistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_applicationstatelistener_callbacks {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_authlistener_callbacks {
    pub request_credentials: alljoyn_authlistener_requestcredentials_ptr,
    pub verify_credentials: alljoyn_authlistener_verifycredentials_ptr,
    pub security_violation: alljoyn_authlistener_securityviolation_ptr,
    pub authentication_complete: alljoyn_authlistener_authenticationcomplete_ptr,
}
impl Default for alljoyn_authlistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_authlistener_callbacks {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_authlistenerasync_callbacks {
    pub request_credentials: alljoyn_authlistener_requestcredentialsasync_ptr,
    pub verify_credentials: alljoyn_authlistener_verifycredentialsasync_ptr,
    pub security_violation: alljoyn_authlistener_securityviolation_ptr,
    pub authentication_complete: alljoyn_authlistener_authenticationcomplete_ptr,
}
impl Default for alljoyn_authlistenerasync_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_authlistenerasync_callbacks {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_buslistener_callbacks {
    pub listener_registered: alljoyn_buslistener_listener_registered_ptr,
    pub listener_unregistered: alljoyn_buslistener_listener_unregistered_ptr,
    pub found_advertised_name: alljoyn_buslistener_found_advertised_name_ptr,
    pub lost_advertised_name: alljoyn_buslistener_lost_advertised_name_ptr,
    pub name_owner_changed: alljoyn_buslistener_name_owner_changed_ptr,
    pub bus_stopping: alljoyn_buslistener_bus_stopping_ptr,
    pub bus_disconnected: alljoyn_buslistener_bus_disconnected_ptr,
    pub property_changed: alljoyn_buslistener_bus_prop_changed_ptr,
}
impl Default for alljoyn_buslistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_buslistener_callbacks {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_busobject_callbacks {
    pub property_get: alljoyn_busobject_prop_get_ptr,
    pub property_set: alljoyn_busobject_prop_set_ptr,
    pub object_registered: alljoyn_busobject_object_registration_ptr,
    pub object_unregistered: alljoyn_busobject_object_registration_ptr,
}
impl Default for alljoyn_busobject_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_busobject_callbacks {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_busobject_methodentry {
    pub member: *const alljoyn_interfacedescription_member,
    pub method_handler: alljoyn_messagereceiver_methodhandler_ptr,
}
impl Default for alljoyn_busobject_methodentry {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_busobject_methodentry {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_certificateid {
    pub serial: *mut u8,
    pub serialLen: usize,
    pub issuerPublicKey: *mut i8,
    pub issuerAki: *mut u8,
    pub issuerAkiLen: usize,
}
impl Default for alljoyn_certificateid {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_certificateid {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_certificateidarray {
    pub count: usize,
    pub ids: *mut alljoyn_certificateid,
}
impl Default for alljoyn_certificateidarray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_certificateidarray {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_interfacedescription_member {
    pub iface: alljoyn_interfacedescription,
    pub memberType: alljoyn_messagetype,
    pub name: windows_core::PCSTR,
    pub signature: windows_core::PCSTR,
    pub returnSignature: windows_core::PCSTR,
    pub argNames: windows_core::PCSTR,
    pub internal_member: *const core::ffi::c_void,
}
impl Default for alljoyn_interfacedescription_member {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_interfacedescription_member {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_interfacedescription_property {
    pub name: windows_core::PCSTR,
    pub signature: windows_core::PCSTR,
    pub access: u8,
    pub internal_property: *const core::ffi::c_void,
}
impl Default for alljoyn_interfacedescription_property {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_interfacedescription_property {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_keystorelistener_callbacks {
    pub load_request: alljoyn_keystorelistener_loadrequest_ptr,
    pub store_request: alljoyn_keystorelistener_storerequest_ptr,
}
impl Default for alljoyn_keystorelistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_keystorelistener_callbacks {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_keystorelistener_with_synchronization_callbacks {
    pub load_request: alljoyn_keystorelistener_loadrequest_ptr,
    pub store_request: alljoyn_keystorelistener_storerequest_ptr,
    pub acquire_exclusive_lock: alljoyn_keystorelistener_acquireexclusivelock_ptr,
    pub release_exclusive_lock: alljoyn_keystorelistener_releaseexclusivelock_ptr,
}
impl Default for alljoyn_keystorelistener_with_synchronization_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_keystorelistener_with_synchronization_callbacks {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_manifestarray {
    pub count: usize,
    pub xmls: *mut *mut i8,
}
impl Default for alljoyn_manifestarray {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_manifestarray {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_observerlistener_callback {
    pub object_discovered: alljoyn_observer_object_discovered_ptr,
    pub object_lost: alljoyn_observer_object_lost_ptr,
}
impl Default for alljoyn_observerlistener_callback {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_observerlistener_callback {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_permissionconfigurationlistener_callbacks {
    pub factory_reset: alljoyn_permissionconfigurationlistener_factoryreset_ptr,
    pub policy_changed: alljoyn_permissionconfigurationlistener_policychanged_ptr,
    pub start_management: alljoyn_permissionconfigurationlistener_startmanagement_ptr,
    pub end_management: alljoyn_permissionconfigurationlistener_endmanagement_ptr,
}
impl Default for alljoyn_permissionconfigurationlistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_permissionconfigurationlistener_callbacks {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_pinglistener_callback {
    pub destination_found: alljoyn_autopinger_destination_found_ptr,
    pub destination_lost: alljoyn_autopinger_destination_lost_ptr,
}
impl Default for alljoyn_pinglistener_callback {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_pinglistener_callback {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_sessionlistener_callbacks {
    pub session_lost: alljoyn_sessionlistener_sessionlost_ptr,
    pub session_member_added: alljoyn_sessionlistener_sessionmemberadded_ptr,
    pub session_member_removed: alljoyn_sessionlistener_sessionmemberremoved_ptr,
}
impl Default for alljoyn_sessionlistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_sessionlistener_callbacks {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct alljoyn_sessionportlistener_callbacks {
    pub accept_session_joiner: alljoyn_sessionportlistener_acceptsessionjoiner_ptr,
    pub session_joined: alljoyn_sessionportlistener_sessionjoined_ptr,
}
impl Default for alljoyn_sessionportlistener_callbacks {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for alljoyn_sessionportlistener_callbacks {
    type TypeKind = windows_core::CloneType;
}
pub type alljoyn_about_announced_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, busname: windows_core::PCSTR, version: u16, port: u16, objectdescriptionarg: alljoyn_msgarg, aboutdataarg: alljoyn_msgarg)>;
pub type alljoyn_aboutdatalistener_getaboutdata_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, msgarg: alljoyn_msgarg, language: windows_core::PCSTR) -> QStatus>;
pub type alljoyn_aboutdatalistener_getannouncedaboutdata_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, msgarg: alljoyn_msgarg) -> QStatus>;
pub type alljoyn_applicationstatelistener_state_ptr = Option<unsafe extern "system" fn(busname: *mut i8, publickey: *mut i8, applicationstate: alljoyn_applicationstate, context: *mut core::ffi::c_void)>;
pub type alljoyn_authlistener_authenticationcomplete_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, success: i32)>;
pub type alljoyn_authlistener_requestcredentials_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, authcount: u16, username: windows_core::PCSTR, credmask: u16, credentials: alljoyn_credentials) -> i32>;
pub type alljoyn_authlistener_requestcredentialsasync_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_authlistener, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, authcount: u16, username: windows_core::PCSTR, credmask: u16, authcontext: *mut core::ffi::c_void) -> QStatus>;
pub type alljoyn_authlistener_securityviolation_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, status: QStatus, msg: alljoyn_message)>;
pub type alljoyn_authlistener_verifycredentials_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, credentials: alljoyn_credentials) -> i32>;
pub type alljoyn_authlistener_verifycredentialsasync_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_authlistener, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, credentials: alljoyn_credentials, authcontext: *mut core::ffi::c_void) -> QStatus>;
pub type alljoyn_autopinger_destination_found_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, group: windows_core::PCSTR, destination: windows_core::PCSTR)>;
pub type alljoyn_autopinger_destination_lost_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, group: windows_core::PCSTR, destination: windows_core::PCSTR)>;
pub type alljoyn_busattachment_joinsessioncb_ptr = Option<unsafe extern "system" fn(status: QStatus, sessionid: u32, opts: alljoyn_sessionopts, context: *mut core::ffi::c_void)>;
pub type alljoyn_busattachment_setlinktimeoutcb_ptr = Option<unsafe extern "system" fn(status: QStatus, timeout: u32, context: *mut core::ffi::c_void)>;
pub type alljoyn_buslistener_bus_disconnected_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_buslistener_bus_prop_changed_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, prop_name: windows_core::PCSTR, prop_value: alljoyn_msgarg)>;
pub type alljoyn_buslistener_bus_stopping_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_buslistener_found_advertised_name_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, name: windows_core::PCSTR, transport: u16, nameprefix: windows_core::PCSTR)>;
pub type alljoyn_buslistener_listener_registered_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, bus: alljoyn_busattachment)>;
pub type alljoyn_buslistener_listener_unregistered_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_buslistener_lost_advertised_name_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, name: windows_core::PCSTR, transport: u16, nameprefix: windows_core::PCSTR)>;
pub type alljoyn_buslistener_name_owner_changed_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, busname: windows_core::PCSTR, previousowner: windows_core::PCSTR, newowner: windows_core::PCSTR)>;
pub type alljoyn_busobject_object_registration_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_busobject_prop_get_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, ifcname: windows_core::PCSTR, propname: windows_core::PCSTR, val: alljoyn_msgarg) -> QStatus>;
pub type alljoyn_busobject_prop_set_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, ifcname: windows_core::PCSTR, propname: windows_core::PCSTR, val: alljoyn_msgarg) -> QStatus>;
pub type alljoyn_interfacedescription_translation_callback_ptr = Option<unsafe extern "system" fn(sourcelanguage: windows_core::PCSTR, targetlanguage: windows_core::PCSTR, sourcetext: windows_core::PCSTR) -> windows_core::PCSTR>;
pub type alljoyn_keystorelistener_acquireexclusivelock_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_keystorelistener) -> QStatus>;
pub type alljoyn_keystorelistener_loadrequest_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_keystorelistener, keystore: alljoyn_keystore) -> QStatus>;
pub type alljoyn_keystorelistener_releaseexclusivelock_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_keystorelistener)>;
pub type alljoyn_keystorelistener_storerequest_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_keystorelistener, keystore: alljoyn_keystore) -> QStatus>;
pub type alljoyn_messagereceiver_methodhandler_ptr = Option<unsafe extern "system" fn(bus: alljoyn_busobject, member: *const alljoyn_interfacedescription_member, message: alljoyn_message)>;
pub type alljoyn_messagereceiver_replyhandler_ptr = Option<unsafe extern "system" fn(message: alljoyn_message, context: *mut core::ffi::c_void)>;
pub type alljoyn_messagereceiver_signalhandler_ptr = Option<unsafe extern "system" fn(member: *const alljoyn_interfacedescription_member, srcpath: windows_core::PCSTR, message: alljoyn_message)>;
pub type alljoyn_observer_object_discovered_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, proxyref: alljoyn_proxybusobject_ref)>;
pub type alljoyn_observer_object_lost_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, proxyref: alljoyn_proxybusobject_ref)>;
pub type alljoyn_permissionconfigurationlistener_endmanagement_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_permissionconfigurationlistener_factoryreset_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> QStatus>;
pub type alljoyn_permissionconfigurationlistener_policychanged_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_permissionconfigurationlistener_startmanagement_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_getallpropertiescb_ptr = Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, values: alljoyn_msgarg, context: *mut core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_getpropertycb_ptr = Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, value: alljoyn_msgarg, context: *mut core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_introspectcb_ptr = Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, context: *mut core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_propertieschanged_ptr = Option<unsafe extern "system" fn(obj: alljoyn_proxybusobject, ifacename: windows_core::PCSTR, changed: alljoyn_msgarg, invalidated: alljoyn_msgarg, context: *mut core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_setpropertycb_ptr = Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, context: *mut core::ffi::c_void)>;
pub type alljoyn_sessionlistener_sessionlost_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionid: u32, reason: alljoyn_sessionlostreason)>;
pub type alljoyn_sessionlistener_sessionmemberadded_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionid: u32, uniquename: windows_core::PCSTR)>;
pub type alljoyn_sessionlistener_sessionmemberremoved_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionid: u32, uniquename: windows_core::PCSTR)>;
pub type alljoyn_sessionportlistener_acceptsessionjoiner_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionport: u16, joiner: windows_core::PCSTR, opts: alljoyn_sessionopts) -> i32>;
pub type alljoyn_sessionportlistener_sessionjoined_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionport: u16, id: u32, joiner: windows_core::PCSTR)>;
