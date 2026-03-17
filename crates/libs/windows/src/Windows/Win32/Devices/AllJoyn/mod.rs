pub const AJ_IFC_SECURITY_INHERIT: alljoyn_interfacedescription_securitypolicy = alljoyn_interfacedescription_securitypolicy(0i32);
pub const AJ_IFC_SECURITY_OFF: alljoyn_interfacedescription_securitypolicy = alljoyn_interfacedescription_securitypolicy(2i32);
pub const AJ_IFC_SECURITY_REQUIRED: alljoyn_interfacedescription_securitypolicy = alljoyn_interfacedescription_securitypolicy(1i32);
pub const ALLJOYN_ARRAY: alljoyn_typeid = alljoyn_typeid(97i32);
pub const ALLJOYN_BIG_ENDIAN: u8 = 66u8;
pub const ALLJOYN_BOOLEAN: alljoyn_typeid = alljoyn_typeid(98i32);
pub const ALLJOYN_BOOLEAN_ARRAY: alljoyn_typeid = alljoyn_typeid(25185i32);
pub const ALLJOYN_BYTE: alljoyn_typeid = alljoyn_typeid(121i32);
pub const ALLJOYN_BYTE_ARRAY: alljoyn_typeid = alljoyn_typeid(31073i32);
pub const ALLJOYN_CRED_CERT_CHAIN: u16 = 4u16;
pub const ALLJOYN_CRED_EXPIRATION: u16 = 32u16;
pub const ALLJOYN_CRED_LOGON_ENTRY: u16 = 16u16;
pub const ALLJOYN_CRED_NEW_PASSWORD: u16 = 4097u16;
pub const ALLJOYN_CRED_ONE_TIME_PWD: u16 = 8193u16;
pub const ALLJOYN_CRED_PASSWORD: u16 = 1u16;
pub const ALLJOYN_CRED_PRIVATE_KEY: u16 = 8u16;
pub const ALLJOYN_CRED_USER_NAME: u16 = 2u16;
pub const ALLJOYN_DICT_ENTRY: alljoyn_typeid = alljoyn_typeid(101i32);
pub const ALLJOYN_DICT_ENTRY_CLOSE: alljoyn_typeid = alljoyn_typeid(125i32);
pub const ALLJOYN_DICT_ENTRY_OPEN: alljoyn_typeid = alljoyn_typeid(123i32);
pub const ALLJOYN_DISCONNECTED: u32 = 4u32;
pub const ALLJOYN_DOUBLE: alljoyn_typeid = alljoyn_typeid(100i32);
pub const ALLJOYN_DOUBLE_ARRAY: alljoyn_typeid = alljoyn_typeid(25697i32);
pub const ALLJOYN_HANDLE: alljoyn_typeid = alljoyn_typeid(104i32);
pub const ALLJOYN_INT16: alljoyn_typeid = alljoyn_typeid(110i32);
pub const ALLJOYN_INT16_ARRAY: alljoyn_typeid = alljoyn_typeid(28257i32);
pub const ALLJOYN_INT32: alljoyn_typeid = alljoyn_typeid(105i32);
pub const ALLJOYN_INT32_ARRAY: alljoyn_typeid = alljoyn_typeid(26977i32);
pub const ALLJOYN_INT64: alljoyn_typeid = alljoyn_typeid(120i32);
pub const ALLJOYN_INT64_ARRAY: alljoyn_typeid = alljoyn_typeid(30817i32);
pub const ALLJOYN_INVALID: alljoyn_typeid = alljoyn_typeid(0i32);
pub const ALLJOYN_LITTLE_ENDIAN: u8 = 108u8;
pub const ALLJOYN_MEMBER_ANNOTATE_DEPRECATED: u8 = 2u8;
pub const ALLJOYN_MEMBER_ANNOTATE_GLOBAL_BROADCAST: u8 = 32u8;
pub const ALLJOYN_MEMBER_ANNOTATE_NO_REPLY: u8 = 1u8;
pub const ALLJOYN_MEMBER_ANNOTATE_SESSIONCAST: u8 = 4u8;
pub const ALLJOYN_MEMBER_ANNOTATE_SESSIONLESS: u8 = 8u8;
pub const ALLJOYN_MEMBER_ANNOTATE_UNICAST: u8 = 16u8;
pub const ALLJOYN_MESSAGE_DEFAULT_TIMEOUT: u32 = 25000u32;
pub const ALLJOYN_MESSAGE_ERROR: alljoyn_messagetype = alljoyn_messagetype(3i32);
pub const ALLJOYN_MESSAGE_FLAG_ALLOW_REMOTE_MSG: u32 = 4u32;
pub const ALLJOYN_MESSAGE_FLAG_AUTO_START: u32 = 2u32;
pub const ALLJOYN_MESSAGE_FLAG_ENCRYPTED: u32 = 128u32;
pub const ALLJOYN_MESSAGE_FLAG_GLOBAL_BROADCAST: u32 = 32u32;
pub const ALLJOYN_MESSAGE_FLAG_NO_REPLY_EXPECTED: u32 = 1u32;
pub const ALLJOYN_MESSAGE_FLAG_SESSIONLESS: u32 = 16u32;
pub const ALLJOYN_MESSAGE_INVALID: alljoyn_messagetype = alljoyn_messagetype(0i32);
pub const ALLJOYN_MESSAGE_METHOD_CALL: alljoyn_messagetype = alljoyn_messagetype(1i32);
pub const ALLJOYN_MESSAGE_METHOD_RET: alljoyn_messagetype = alljoyn_messagetype(2i32);
pub const ALLJOYN_MESSAGE_SIGNAL: alljoyn_messagetype = alljoyn_messagetype(4i32);
pub const ALLJOYN_NAMED_PIPE_CONNECT_SPEC: windows_core::PCWSTR = windows_core::w!("npipe:");
pub const ALLJOYN_OBJECT_PATH: alljoyn_typeid = alljoyn_typeid(111i32);
pub const ALLJOYN_PROP_ACCESS_READ: u8 = 1u8;
pub const ALLJOYN_PROP_ACCESS_RW: u8 = 3u8;
pub const ALLJOYN_PROP_ACCESS_WRITE: u8 = 2u8;
pub const ALLJOYN_PROXIMITY_ANY: u32 = 255u32;
pub const ALLJOYN_PROXIMITY_NETWORK: u32 = 2u32;
pub const ALLJOYN_PROXIMITY_PHYSICAL: u32 = 1u32;
pub const ALLJOYN_READ_READY: u32 = 1u32;
pub const ALLJOYN_SESSIONLOST_INVALID: alljoyn_sessionlostreason = alljoyn_sessionlostreason(0i32);
pub const ALLJOYN_SESSIONLOST_LINK_TIMEOUT: alljoyn_sessionlostreason = alljoyn_sessionlostreason(4i32);
pub const ALLJOYN_SESSIONLOST_REASON_OTHER: alljoyn_sessionlostreason = alljoyn_sessionlostreason(5i32);
pub const ALLJOYN_SESSIONLOST_REMOTE_END_CLOSED_ABRUPTLY: alljoyn_sessionlostreason = alljoyn_sessionlostreason(2i32);
pub const ALLJOYN_SESSIONLOST_REMOTE_END_LEFT_SESSION: alljoyn_sessionlostreason = alljoyn_sessionlostreason(1i32);
pub const ALLJOYN_SESSIONLOST_REMOVED_BY_BINDER: alljoyn_sessionlostreason = alljoyn_sessionlostreason(3i32);
pub const ALLJOYN_SIGNATURE: alljoyn_typeid = alljoyn_typeid(103i32);
pub const ALLJOYN_STRING: alljoyn_typeid = alljoyn_typeid(115i32);
pub const ALLJOYN_STRUCT: alljoyn_typeid = alljoyn_typeid(114i32);
pub const ALLJOYN_STRUCT_CLOSE: alljoyn_typeid = alljoyn_typeid(41i32);
pub const ALLJOYN_STRUCT_OPEN: alljoyn_typeid = alljoyn_typeid(40i32);
pub const ALLJOYN_TRAFFIC_TYPE_MESSAGES: u32 = 1u32;
pub const ALLJOYN_TRAFFIC_TYPE_RAW_RELIABLE: u32 = 4u32;
pub const ALLJOYN_TRAFFIC_TYPE_RAW_UNRELIABLE: u32 = 2u32;
pub const ALLJOYN_UINT16: alljoyn_typeid = alljoyn_typeid(113i32);
pub const ALLJOYN_UINT16_ARRAY: alljoyn_typeid = alljoyn_typeid(29025i32);
pub const ALLJOYN_UINT32: alljoyn_typeid = alljoyn_typeid(117i32);
pub const ALLJOYN_UINT32_ARRAY: alljoyn_typeid = alljoyn_typeid(30049i32);
pub const ALLJOYN_UINT64: alljoyn_typeid = alljoyn_typeid(116i32);
pub const ALLJOYN_UINT64_ARRAY: alljoyn_typeid = alljoyn_typeid(29793i32);
pub const ALLJOYN_VARIANT: alljoyn_typeid = alljoyn_typeid(118i32);
pub const ALLJOYN_WILDCARD: alljoyn_typeid = alljoyn_typeid(42i32);
pub const ALLJOYN_WRITE_READY: u32 = 2u32;
pub const ANNOUNCED: alljoyn_about_announceflag = alljoyn_about_announceflag(1i32);
pub const CAPABLE_ECDHE_ECDSA: alljoyn_claimcapability_masks = alljoyn_claimcapability_masks(4i32);
pub const CAPABLE_ECDHE_NULL: alljoyn_claimcapability_masks = alljoyn_claimcapability_masks(1i32);
pub const CAPABLE_ECDHE_SPEKE: alljoyn_claimcapability_masks = alljoyn_claimcapability_masks(8i32);
pub const CLAIMABLE: alljoyn_applicationstate = alljoyn_applicationstate(1i32);
pub const CLAIMED: alljoyn_applicationstate = alljoyn_applicationstate(2i32);
pub const ER_ABOUT_ABOUTDATA_MISSING_REQUIRED_FIELD: QStatus = QStatus(37157i32);
pub const ER_ABOUT_DEFAULT_LANGUAGE_NOT_SPECIFIED: QStatus = QStatus(37155i32);
pub const ER_ABOUT_FIELD_ALREADY_SPECIFIED: QStatus = QStatus(37147i32);
pub const ER_ABOUT_INVALID_ABOUTDATA_FIELD_APPID_SIZE: QStatus = QStatus(37163i32);
pub const ER_ABOUT_INVALID_ABOUTDATA_FIELD_VALUE: QStatus = QStatus(37162i32);
pub const ER_ABOUT_INVALID_ABOUTDATA_LISTENER: QStatus = QStatus(37158i32);
pub const ER_ABOUT_SESSIONPORT_NOT_BOUND: QStatus = QStatus(37156i32);
pub const ER_ALERTED_THREAD: QStatus = QStatus(4098i32);
pub const ER_ALLJOYN_ACCESS_PERMISSION_ERROR: QStatus = QStatus(37028i32);
pub const ER_ALLJOYN_ACCESS_PERMISSION_WARNING: QStatus = QStatus(37027i32);
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_ALREADY_ADVERTISING: QStatus = QStatus(37005i32);
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_FAILED: QStatus = QStatus(37006i32);
pub const ER_ALLJOYN_ADVERTISENAME_REPLY_TRANSPORT_NOT_AVAILABLE: QStatus = QStatus(37004i32);
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_ALREADY_EXISTS: QStatus = QStatus(36992i32);
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_FAILED: QStatus = QStatus(36993i32);
pub const ER_ALLJOYN_BINDSESSIONPORT_REPLY_INVALID_OPTS: QStatus = QStatus(37018i32);
pub const ER_ALLJOYN_CANCELADVERTISENAME_REPLY_FAILED: QStatus = QStatus(37008i32);
pub const ER_ALLJOYN_CANCELFINDADVERTISEDNAME_REPLY_FAILED: QStatus = QStatus(37013i32);
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_ALREADY_DISCOVERING: QStatus = QStatus(37010i32);
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_FAILED: QStatus = QStatus(37011i32);
pub const ER_ALLJOYN_FINDADVERTISEDNAME_REPLY_TRANSPORT_NOT_AVAILABLE: QStatus = QStatus(37009i32);
pub const ER_ALLJOYN_JOINSESSION_REPLY_ALREADY_JOINED: QStatus = QStatus(37019i32);
pub const ER_ALLJOYN_JOINSESSION_REPLY_BAD_SESSION_OPTS: QStatus = QStatus(36999i32);
pub const ER_ALLJOYN_JOINSESSION_REPLY_CONNECT_FAILED: QStatus = QStatus(36997i32);
pub const ER_ALLJOYN_JOINSESSION_REPLY_FAILED: QStatus = QStatus(37000i32);
pub const ER_ALLJOYN_JOINSESSION_REPLY_NO_SESSION: QStatus = QStatus(36995i32);
pub const ER_ALLJOYN_JOINSESSION_REPLY_REJECTED: QStatus = QStatus(36998i32);
pub const ER_ALLJOYN_JOINSESSION_REPLY_UNREACHABLE: QStatus = QStatus(36996i32);
pub const ER_ALLJOYN_LEAVESESSION_REPLY_FAILED: QStatus = QStatus(37003i32);
pub const ER_ALLJOYN_LEAVESESSION_REPLY_NO_SESSION: QStatus = QStatus(37002i32);
pub const ER_ALLJOYN_ONAPPRESUME_REPLY_FAILED: QStatus = QStatus(37100i32);
pub const ER_ALLJOYN_ONAPPRESUME_REPLY_UNSUPPORTED: QStatus = QStatus(37101i32);
pub const ER_ALLJOYN_ONAPPSUSPEND_REPLY_FAILED: QStatus = QStatus(37098i32);
pub const ER_ALLJOYN_ONAPPSUSPEND_REPLY_UNSUPPORTED: QStatus = QStatus(37099i32);
pub const ER_ALLJOYN_PING_FAILED: QStatus = QStatus(37111i32);
pub const ER_ALLJOYN_PING_REPLY_FAILED: QStatus = QStatus(37143i32);
pub const ER_ALLJOYN_PING_REPLY_INCOMPATIBLE_REMOTE_ROUTING_NODE: QStatus = QStatus(37140i32);
pub const ER_ALLJOYN_PING_REPLY_IN_PROGRESS: QStatus = QStatus(37145i32);
pub const ER_ALLJOYN_PING_REPLY_TIMEOUT: QStatus = QStatus(37141i32);
pub const ER_ALLJOYN_PING_REPLY_UNKNOWN_NAME: QStatus = QStatus(37142i32);
pub const ER_ALLJOYN_PING_REPLY_UNREACHABLE: QStatus = QStatus(37112i32);
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_INCOMPATIBLE_REMOTE_DAEMON: QStatus = QStatus(37107i32);
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_BINDER: QStatus = QStatus(37104i32);
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_FOUND: QStatus = QStatus(37106i32);
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_NOT_MULTIPOINT: QStatus = QStatus(37105i32);
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_REPLY_FAILED: QStatus = QStatus(37108i32);
pub const ER_ALLJOYN_REMOVESESSIONMEMBER_REPLY_NO_SESSION: QStatus = QStatus(37103i32);
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_FAILED: QStatus = QStatus(37026i32);
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_NOT_SUPPORTED: QStatus = QStatus(37024i32);
pub const ER_ALLJOYN_SETLINKTIMEOUT_REPLY_NO_DEST_SUPPORT: QStatus = QStatus(37025i32);
pub const ER_ALLJOYN_UNBINDSESSIONPORT_REPLY_BAD_PORT: QStatus = QStatus(37016i32);
pub const ER_ALLJOYN_UNBINDSESSIONPORT_REPLY_FAILED: QStatus = QStatus(37017i32);
pub const ER_APPLICATION_STATE_LISTENER_ALREADY_EXISTS: QStatus = QStatus(37184i32);
pub const ER_APPLICATION_STATE_LISTENER_NO_SUCH_LISTENER: QStatus = QStatus(37185i32);
pub const ER_ARDP_BACKPRESSURE: QStatus = QStatus(37122i32);
pub const ER_ARDP_DISCONNECTING: QStatus = QStatus(37139i32);
pub const ER_ARDP_INVALID_CONNECTION: QStatus = QStatus(37135i32);
pub const ER_ARDP_INVALID_RESPONSE: QStatus = QStatus(37134i32);
pub const ER_ARDP_INVALID_STATE: QStatus = QStatus(37124i32);
pub const ER_ARDP_PERSIST_TIMEOUT: QStatus = QStatus(37126i32);
pub const ER_ARDP_PROBE_TIMEOUT: QStatus = QStatus(37127i32);
pub const ER_ARDP_REMOTE_CONNECTION_RESET: QStatus = QStatus(37128i32);
pub const ER_ARDP_TTL_EXPIRED: QStatus = QStatus(37125i32);
pub const ER_ARDP_VERSION_NOT_SUPPORTED: QStatus = QStatus(37151i32);
pub const ER_ARDP_WRITE_BLOCKED: QStatus = QStatus(37153i32);
pub const ER_AUTH_FAIL: QStatus = QStatus(4100i32);
pub const ER_AUTH_USER_REJECT: QStatus = QStatus(4101i32);
pub const ER_BAD_ARG_1: QStatus = QStatus(12i32);
pub const ER_BAD_ARG_2: QStatus = QStatus(13i32);
pub const ER_BAD_ARG_3: QStatus = QStatus(14i32);
pub const ER_BAD_ARG_4: QStatus = QStatus(15i32);
pub const ER_BAD_ARG_5: QStatus = QStatus(16i32);
pub const ER_BAD_ARG_6: QStatus = QStatus(17i32);
pub const ER_BAD_ARG_7: QStatus = QStatus(18i32);
pub const ER_BAD_ARG_8: QStatus = QStatus(19i32);
pub const ER_BAD_ARG_COUNT: QStatus = QStatus(28i32);
pub const ER_BAD_HOSTNAME: QStatus = QStatus(4112i32);
pub const ER_BAD_STRING_ENCODING: QStatus = QStatus(4120i32);
pub const ER_BAD_TRANSPORT_MASK: QStatus = QStatus(37088i32);
pub const ER_BUFFER_TOO_SMALL: QStatus = QStatus(3i32);
pub const ER_BUS_ALREADY_CONNECTED: QStatus = QStatus(36932i32);
pub const ER_BUS_ALREADY_LISTENING: QStatus = QStatus(36934i32);
pub const ER_BUS_ANNOTATION_ALREADY_EXISTS: QStatus = QStatus(37082i32);
pub const ER_BUS_AUTHENTICATION_PENDING: QStatus = QStatus(37031i32);
pub const ER_BUS_BAD_BODY_LEN: QStatus = QStatus(36879i32);
pub const ER_BUS_BAD_BUS_NAME: QStatus = QStatus(36874i32);
pub const ER_BUS_BAD_CHILD_PATH: QStatus = QStatus(36927i32);
pub const ER_BUS_BAD_ERROR_NAME: QStatus = QStatus(36873i32);
pub const ER_BUS_BAD_HDR_FLAGS: QStatus = QStatus(36878i32);
pub const ER_BUS_BAD_HEADER_FIELD: QStatus = QStatus(36868i32);
pub const ER_BUS_BAD_HEADER_LEN: QStatus = QStatus(36880i32);
pub const ER_BUS_BAD_INTERFACE_NAME: QStatus = QStatus(36872i32);
pub const ER_BUS_BAD_LENGTH: QStatus = QStatus(36876i32);
pub const ER_BUS_BAD_MEMBER_NAME: QStatus = QStatus(36871i32);
pub const ER_BUS_BAD_OBJ_PATH: QStatus = QStatus(36870i32);
pub const ER_BUS_BAD_SENDER_ID: QStatus = QStatus(36908i32);
pub const ER_BUS_BAD_SEND_PARAMETER: QStatus = QStatus(36906i32);
pub const ER_BUS_BAD_SESSION_OPTS: QStatus = QStatus(36980i32);
pub const ER_BUS_BAD_SIGNATURE: QStatus = QStatus(36869i32);
pub const ER_BUS_BAD_TRANSPORT_ARGS: QStatus = QStatus(36903i32);
pub const ER_BUS_BAD_VALUE: QStatus = QStatus(36877i32);
pub const ER_BUS_BAD_VALUE_TYPE: QStatus = QStatus(36867i32);
pub const ER_BUS_BAD_XML: QStatus = QStatus(36926i32);
pub const ER_BUS_BLOCKING_CALL_NOT_ALLOWED: QStatus = QStatus(36960i32);
pub const ER_BUS_BUS_ALREADY_STARTED: QStatus = QStatus(36939i32);
pub const ER_BUS_BUS_NOT_STARTED: QStatus = QStatus(36940i32);
pub const ER_BUS_CANNOT_ADD_HANDLER: QStatus = QStatus(36965i32);
pub const ER_BUS_CANNOT_ADD_INTERFACE: QStatus = QStatus(36964i32);
pub const ER_BUS_CANNOT_EXPAND_MESSAGE: QStatus = QStatus(36930i32);
pub const ER_BUS_CONNECTION_REJECTED: QStatus = QStatus(36981i32);
pub const ER_BUS_CONNECT_FAILED: QStatus = QStatus(36913i32);
pub const ER_BUS_CORRUPT_KEYSTORE: QStatus = QStatus(36952i32);
pub const ER_BUS_DESCRIPTION_ALREADY_EXISTS: QStatus = QStatus(37188i32);
pub const ER_BUS_DESTINATION_NOT_AUTHENTICATED: QStatus = QStatus(37029i32);
pub const ER_BUS_ELEMENT_NOT_FOUND: QStatus = QStatus(36976i32);
pub const ER_BUS_EMPTY_MESSAGE: QStatus = QStatus(36910i32);
pub const ER_BUS_ENDPOINT_CLOSING: QStatus = QStatus(36920i32);
pub const ER_BUS_ENDPOINT_REDIRECTED: QStatus = QStatus(37030i32);
pub const ER_BUS_ERRORS: QStatus = QStatus(36864i32);
pub const ER_BUS_ERROR_NAME_MISSING: QStatus = QStatus(36890i32);
pub const ER_BUS_ERROR_RESPONSE: QStatus = QStatus(36925i32);
pub const ER_BUS_ESTABLISH_FAILED: QStatus = QStatus(36884i32);
pub const ER_BUS_HANDLES_MISMATCH: QStatus = QStatus(36973i32);
pub const ER_BUS_HANDLES_NOT_ENABLED: QStatus = QStatus(36972i32);
pub const ER_BUS_HDR_EXPANSION_INVALID: QStatus = QStatus(36946i32);
pub const ER_BUS_IFACE_ALREADY_EXISTS: QStatus = QStatus(36924i32);
pub const ER_BUS_INCOMPATIBLE_DAEMON: QStatus = QStatus(37094i32);
pub const ER_BUS_INTERFACE_ACTIVATED: QStatus = QStatus(37015i32);
pub const ER_BUS_INTERFACE_MISMATCH: QStatus = QStatus(36921i32);
pub const ER_BUS_INTERFACE_MISSING: QStatus = QStatus(36886i32);
pub const ER_BUS_INTERFACE_NO_SUCH_MEMBER: QStatus = QStatus(36891i32);
pub const ER_BUS_INVALID_AUTH_MECHANISM: QStatus = QStatus(36958i32);
pub const ER_BUS_INVALID_HEADER_CHECKSUM: QStatus = QStatus(36942i32);
pub const ER_BUS_INVALID_HEADER_SERIAL: QStatus = QStatus(36944i32);
pub const ER_BUS_KEYBLOB_OP_INVALID: QStatus = QStatus(36941i32);
pub const ER_BUS_KEYSTORE_NOT_LOADED: QStatus = QStatus(36966i32);
pub const ER_BUS_KEYSTORE_VERSION_MISMATCH: QStatus = QStatus(36959i32);
pub const ER_BUS_KEY_EXPIRED: QStatus = QStatus(36951i32);
pub const ER_BUS_KEY_STORE_NOT_LOADED: QStatus = QStatus(36937i32);
pub const ER_BUS_KEY_UNAVAILABLE: QStatus = QStatus(36935i32);
pub const ER_BUS_LISTENER_ALREADY_SET: QStatus = QStatus(37022i32);
pub const ER_BUS_MATCH_RULE_NOT_FOUND: QStatus = QStatus(37110i32);
pub const ER_BUS_MEMBER_ALREADY_EXISTS: QStatus = QStatus(36922i32);
pub const ER_BUS_MEMBER_MISSING: QStatus = QStatus(36888i32);
pub const ER_BUS_MEMBER_NO_SUCH_SIGNATURE: QStatus = QStatus(36896i32);
pub const ER_BUS_MESSAGE_DECRYPTION_FAILED: QStatus = QStatus(36949i32);
pub const ER_BUS_MESSAGE_NOT_ENCRYPTED: QStatus = QStatus(36943i32);
pub const ER_BUS_METHOD_CALL_ABORTED: QStatus = QStatus(36963i32);
pub const ER_BUS_MISSING_COMPRESSION_TOKEN: QStatus = QStatus(36947i32);
pub const ER_BUS_NAME_TOO_LONG: QStatus = QStatus(36875i32);
pub const ER_BUS_NOT_ALLOWED: QStatus = QStatus(36918i32);
pub const ER_BUS_NOT_AUTHENTICATING: QStatus = QStatus(36915i32);
pub const ER_BUS_NOT_AUTHORIZED: QStatus = QStatus(37032i32);
pub const ER_BUS_NOT_A_COMPLETE_TYPE: QStatus = QStatus(36954i32);
pub const ER_BUS_NOT_A_DICTIONARY: QStatus = QStatus(36977i32);
pub const ER_BUS_NOT_COMPRESSED: QStatus = QStatus(36931i32);
pub const ER_BUS_NOT_CONNECTED: QStatus = QStatus(36933i32);
pub const ER_BUS_NOT_NUL_TERMINATED: QStatus = QStatus(36897i32);
pub const ER_BUS_NOT_OWNER: QStatus = QStatus(36911i32);
pub const ER_BUS_NO_AUTHENTICATION_MECHANISM: QStatus = QStatus(36938i32);
pub const ER_BUS_NO_CALL_FOR_REPLY: QStatus = QStatus(36953i32);
pub const ER_BUS_NO_ENDPOINT: QStatus = QStatus(36905i32);
pub const ER_BUS_NO_LISTENER: QStatus = QStatus(36916i32);
pub const ER_BUS_NO_PEER_GUID: QStatus = QStatus(36948i32);
pub const ER_BUS_NO_ROUTE: QStatus = QStatus(36904i32);
pub const ER_BUS_NO_SESSION: QStatus = QStatus(36975i32);
pub const ER_BUS_NO_SUCH_ANNOTATION: QStatus = QStatus(37081i32);
pub const ER_BUS_NO_SUCH_HANDLE: QStatus = QStatus(36971i32);
pub const ER_BUS_NO_SUCH_INTERFACE: QStatus = QStatus(36895i32);
pub const ER_BUS_NO_SUCH_MESSAGE: QStatus = QStatus(37102i32);
pub const ER_BUS_NO_SUCH_OBJECT: QStatus = QStatus(36892i32);
pub const ER_BUS_NO_SUCH_PROPERTY: QStatus = QStatus(36898i32);
pub const ER_BUS_NO_SUCH_SERVICE: QStatus = QStatus(36956i32);
pub const ER_BUS_NO_TRANSPORTS: QStatus = QStatus(36902i32);
pub const ER_BUS_OBJECT_NOT_REGISTERED: QStatus = QStatus(37091i32);
pub const ER_BUS_OBJECT_NO_SUCH_INTERFACE: QStatus = QStatus(36894i32);
pub const ER_BUS_OBJECT_NO_SUCH_MEMBER: QStatus = QStatus(36893i32);
pub const ER_BUS_OBJ_ALREADY_EXISTS: QStatus = QStatus(36928i32);
pub const ER_BUS_OBJ_NOT_FOUND: QStatus = QStatus(36929i32);
pub const ER_BUS_PATH_MISSING: QStatus = QStatus(36887i32);
pub const ER_BUS_PEER_AUTH_VERSION_MISMATCH: QStatus = QStatus(37023i32);
pub const ER_BUS_PING_GROUP_NOT_FOUND: QStatus = QStatus(37159i32);
pub const ER_BUS_POLICY_VIOLATION: QStatus = QStatus(36955i32);
pub const ER_BUS_PROPERTY_ACCESS_DENIED: QStatus = QStatus(36901i32);
pub const ER_BUS_PROPERTY_ALREADY_EXISTS: QStatus = QStatus(36923i32);
pub const ER_BUS_PROPERTY_VALUE_NOT_SET: QStatus = QStatus(36900i32);
pub const ER_BUS_READ_ERROR: QStatus = QStatus(36865i32);
pub const ER_BUS_REMOVED_BY_BINDER: QStatus = QStatus(37109i32);
pub const ER_BUS_REMOVED_BY_BINDER_SELF: QStatus = QStatus(37160i32);
pub const ER_BUS_REPLY_IS_ERROR_MESSAGE: QStatus = QStatus(36914i32);
pub const ER_BUS_REPLY_SERIAL_MISSING: QStatus = QStatus(36889i32);
pub const ER_BUS_SECURITY_FATAL: QStatus = QStatus(36950i32);
pub const ER_BUS_SECURITY_NOT_ENABLED: QStatus = QStatus(37021i32);
pub const ER_BUS_SELF_CONNECT: QStatus = QStatus(37020i32);
pub const ER_BUS_SET_PROPERTY_REJECTED: QStatus = QStatus(36912i32);
pub const ER_BUS_SET_WRONG_SIGNATURE: QStatus = QStatus(36899i32);
pub const ER_BUS_SIGNATURE_MISMATCH: QStatus = QStatus(36961i32);
pub const ER_BUS_STOPPING: QStatus = QStatus(36962i32);
pub const ER_BUS_TIME_TO_LIVE_EXPIRED: QStatus = QStatus(36945i32);
pub const ER_BUS_TRANSPORT_ACCESS_DENIED: QStatus = QStatus(37164i32);
pub const ER_BUS_TRANSPORT_NOT_AVAILABLE: QStatus = QStatus(36957i32);
pub const ER_BUS_TRANSPORT_NOT_STARTED: QStatus = QStatus(36909i32);
pub const ER_BUS_TRUNCATED: QStatus = QStatus(36936i32);
pub const ER_BUS_UNEXPECTED_DISPOSITION: QStatus = QStatus(37014i32);
pub const ER_BUS_UNEXPECTED_SIGNATURE: QStatus = QStatus(36885i32);
pub const ER_BUS_UNKNOWN_INTERFACE: QStatus = QStatus(36883i32);
pub const ER_BUS_UNKNOWN_PATH: QStatus = QStatus(36882i32);
pub const ER_BUS_UNKNOWN_SERIAL: QStatus = QStatus(36881i32);
pub const ER_BUS_UNMATCHED_REPLY_SERIAL: QStatus = QStatus(36907i32);
pub const ER_BUS_WAIT_FAILED: QStatus = QStatus(36978i32);
pub const ER_BUS_WRITE_ERROR: QStatus = QStatus(36866i32);
pub const ER_BUS_WRITE_QUEUE_FULL: QStatus = QStatus(36919i32);
pub const ER_CERTIFICATE_NOT_FOUND: QStatus = QStatus(37166i32);
pub const ER_COMMON_ERRORS: QStatus = QStatus(4096i32);
pub const ER_CONNECTION_LIMIT_EXCEEDED: QStatus = QStatus(37152i32);
pub const ER_CONN_REFUSED: QStatus = QStatus(27i32);
pub const ER_CORRUPT_KEYBLOB: QStatus = QStatus(4115i32);
pub const ER_CRYPTO_ERROR: QStatus = QStatus(4109i32);
pub const ER_CRYPTO_HASH_UNINITIALIZED: QStatus = QStatus(4123i32);
pub const ER_CRYPTO_ILLEGAL_PARAMETERS: QStatus = QStatus(4122i32);
pub const ER_CRYPTO_INSUFFICIENT_SECURITY: QStatus = QStatus(4121i32);
pub const ER_CRYPTO_KEY_UNAVAILABLE: QStatus = QStatus(4111i32);
pub const ER_CRYPTO_KEY_UNUSABLE: QStatus = QStatus(4113i32);
pub const ER_CRYPTO_TRUNCATED: QStatus = QStatus(4110i32);
pub const ER_DBUS_RELEASE_NAME_REPLY_NON_EXISTENT: QStatus = QStatus(36987i32);
pub const ER_DBUS_RELEASE_NAME_REPLY_NOT_OWNER: QStatus = QStatus(36988i32);
pub const ER_DBUS_RELEASE_NAME_REPLY_RELEASED: QStatus = QStatus(36986i32);
pub const ER_DBUS_REQUEST_NAME_REPLY_ALREADY_OWNER: QStatus = QStatus(36985i32);
pub const ER_DBUS_REQUEST_NAME_REPLY_EXISTS: QStatus = QStatus(36984i32);
pub const ER_DBUS_REQUEST_NAME_REPLY_IN_QUEUE: QStatus = QStatus(36983i32);
pub const ER_DBUS_REQUEST_NAME_REPLY_PRIMARY_OWNER: QStatus = QStatus(36982i32);
pub const ER_DBUS_START_REPLY_ALREADY_RUNNING: QStatus = QStatus(36990i32);
pub const ER_DEADLOCK: QStatus = QStatus(31i32);
pub const ER_DEAD_THREAD: QStatus = QStatus(4117i32);
pub const ER_DIGEST_MISMATCH: QStatus = QStatus(37170i32);
pub const ER_DUPLICATE_CERTIFICATE: QStatus = QStatus(37167i32);
pub const ER_DUPLICATE_KEY: QStatus = QStatus(37171i32);
pub const ER_EMPTY_KEY_BLOB: QStatus = QStatus(4114i32);
pub const ER_END_OF_DATA: QStatus = QStatus(26i32);
pub const ER_EOF: QStatus = QStatus(30i32);
pub const ER_EXTERNAL_THREAD: QStatus = QStatus(4108i32);
pub const ER_FAIL: QStatus = QStatus(1i32);
pub const ER_FEATURE_NOT_AVAILABLE: QStatus = QStatus(37177i32);
pub const ER_INIT_FAILED: QStatus = QStatus(7i32);
pub const ER_INVALID_ADDRESS: QStatus = QStatus(20i32);
pub const ER_INVALID_APPLICATION_STATE: QStatus = QStatus(37176i32);
pub const ER_INVALID_CERTIFICATE: QStatus = QStatus(37165i32);
pub const ER_INVALID_CERTIFICATE_USAGE: QStatus = QStatus(37182i32);
pub const ER_INVALID_CERT_CHAIN: QStatus = QStatus(37174i32);
pub const ER_INVALID_CONFIG: QStatus = QStatus(37161i32);
pub const ER_INVALID_DATA: QStatus = QStatus(21i32);
pub const ER_INVALID_GUID: QStatus = QStatus(4126i32);
pub const ER_INVALID_HTTP_METHOD_USED_FOR_RENDEZVOUS_SERVER_INTERFACE_MESSAGE: QStatus = QStatus(37075i32);
pub const ER_INVALID_KEY_ENCODING: QStatus = QStatus(4116i32);
pub const ER_INVALID_ON_DEMAND_CONNECTION_MESSAGE_RESPONSE: QStatus = QStatus(37074i32);
pub const ER_INVALID_PERSISTENT_CONNECTION_MESSAGE_RESPONSE: QStatus = QStatus(37073i32);
pub const ER_INVALID_RENDEZVOUS_SERVER_INTERFACE_MESSAGE: QStatus = QStatus(37072i32);
pub const ER_INVALID_SIGNAL_EMISSION_TYPE: QStatus = QStatus(37183i32);
pub const ER_INVALID_STREAM: QStatus = QStatus(4129i32);
pub const ER_IODISPATCH_STOPPING: QStatus = QStatus(4131i32);
pub const ER_KEY_STORE_ALREADY_INITIALIZED: QStatus = QStatus(37178i32);
pub const ER_KEY_STORE_ID_NOT_YET_SET: QStatus = QStatus(37179i32);
pub const ER_LANGUAGE_NOT_SUPPORTED: QStatus = QStatus(37146i32);
pub const ER_MANAGEMENT_ALREADY_STARTED: QStatus = QStatus(37186i32);
pub const ER_MANAGEMENT_NOT_STARTED: QStatus = QStatus(37187i32);
pub const ER_MANIFEST_NOT_FOUND: QStatus = QStatus(37173i32);
pub const ER_MANIFEST_REJECTED: QStatus = QStatus(37181i32);
pub const ER_MISSING_DIGEST_IN_CERTIFICATE: QStatus = QStatus(37169i32);
pub const ER_NONE: QStatus = QStatus(65535i32);
pub const ER_NOT_CONN: QStatus = QStatus(4141i32);
pub const ER_NOT_CONNECTED_TO_RENDEZVOUS_SERVER: QStatus = QStatus(37070i32);
pub const ER_NOT_IMPLEMENTED: QStatus = QStatus(9i32);
pub const ER_NO_COMMON_TRUST: QStatus = QStatus(37172i32);
pub const ER_NO_SUCH_ALARM: QStatus = QStatus(4102i32);
pub const ER_NO_SUCH_DEVICE: QStatus = QStatus(37084i32);
pub const ER_NO_TRUST_ANCHOR: QStatus = QStatus(37175i32);
pub const ER_OK: QStatus = QStatus(0i32);
pub const ER_OPEN_FAILED: QStatus = QStatus(24i32);
pub const ER_OS_ERROR: QStatus = QStatus(4i32);
pub const ER_OUT_OF_MEMORY: QStatus = QStatus(5i32);
pub const ER_P2P: QStatus = QStatus(37085i32);
pub const ER_P2P_BUSY: QStatus = QStatus(37093i32);
pub const ER_P2P_DISABLED: QStatus = QStatus(37092i32);
pub const ER_P2P_FORBIDDEN: QStatus = QStatus(37097i32);
pub const ER_P2P_NOT_CONNECTED: QStatus = QStatus(37087i32);
pub const ER_P2P_NO_GO: QStatus = QStatus(37095i32);
pub const ER_P2P_NO_STA: QStatus = QStatus(37096i32);
pub const ER_P2P_TIMEOUT: QStatus = QStatus(37086i32);
pub const ER_PACKET_BAD_CRC: QStatus = QStatus(37039i32);
pub const ER_PACKET_BAD_FORMAT: QStatus = QStatus(37034i32);
pub const ER_PACKET_BAD_PARAMETER: QStatus = QStatus(37038i32);
pub const ER_PACKET_BUS_NO_SUCH_CHANNEL: QStatus = QStatus(37033i32);
pub const ER_PACKET_CHANNEL_FAIL: QStatus = QStatus(37036i32);
pub const ER_PACKET_CONNECT_TIMEOUT: QStatus = QStatus(37035i32);
pub const ER_PACKET_TOO_LARGE: QStatus = QStatus(37037i32);
pub const ER_PARSE_ERROR: QStatus = QStatus(25i32);
pub const ER_PERMISSION_DENIED: QStatus = QStatus(37154i32);
pub const ER_POLICY_NOT_NEWER: QStatus = QStatus(37180i32);
pub const ER_PROXIMITY_CONNECTION_ESTABLISH_FAIL: QStatus = QStatus(37089i32);
pub const ER_PROXIMITY_NO_PEERS_FOUND: QStatus = QStatus(37090i32);
pub const ER_READ_ERROR: QStatus = QStatus(22i32);
pub const ER_RENDEZVOUS_SERVER_DEACTIVATED_USER: QStatus = QStatus(37067i32);
pub const ER_RENDEZVOUS_SERVER_ERR401_UNAUTHORIZED_REQUEST: QStatus = QStatus(37078i32);
pub const ER_RENDEZVOUS_SERVER_ERR500_INTERNAL_ERROR: QStatus = QStatus(37076i32);
pub const ER_RENDEZVOUS_SERVER_ERR503_STATUS_UNAVAILABLE: QStatus = QStatus(37077i32);
pub const ER_RENDEZVOUS_SERVER_ROOT_CERTIFICATE_UNINITIALIZED: QStatus = QStatus(37080i32);
pub const ER_RENDEZVOUS_SERVER_UNKNOWN_USER: QStatus = QStatus(37068i32);
pub const ER_RENDEZVOUS_SERVER_UNRECOVERABLE_ERROR: QStatus = QStatus(37079i32);
pub const ER_SLAP_CRC_ERROR: QStatus = QStatus(4137i32);
pub const ER_SLAP_ERROR: QStatus = QStatus(4138i32);
pub const ER_SLAP_HDR_CHECKSUM_ERROR: QStatus = QStatus(4133i32);
pub const ER_SLAP_INVALID_PACKET_LEN: QStatus = QStatus(4132i32);
pub const ER_SLAP_INVALID_PACKET_TYPE: QStatus = QStatus(4134i32);
pub const ER_SLAP_LEN_MISMATCH: QStatus = QStatus(4135i32);
pub const ER_SLAP_OTHER_END_CLOSED: QStatus = QStatus(4139i32);
pub const ER_SLAP_PACKET_TYPE_MISMATCH: QStatus = QStatus(4136i32);
pub const ER_SOCKET_BIND_ERROR: QStatus = QStatus(6i32);
pub const ER_SOCK_CLOSING: QStatus = QStatus(37083i32);
pub const ER_SOCK_OTHER_END_CLOSED: QStatus = QStatus(11i32);
pub const ER_SSL_CONNECT: QStatus = QStatus(4106i32);
pub const ER_SSL_ERRORS: QStatus = QStatus(4104i32);
pub const ER_SSL_INIT: QStatus = QStatus(4105i32);
pub const ER_SSL_VERIFY: QStatus = QStatus(4107i32);
pub const ER_STOPPING_THREAD: QStatus = QStatus(4097i32);
pub const ER_TCP_MAX_UNTRUSTED: QStatus = QStatus(37144i32);
pub const ER_THREADPOOL_EXHAUSTED: QStatus = QStatus(4127i32);
pub const ER_THREADPOOL_STOPPING: QStatus = QStatus(4128i32);
pub const ER_THREAD_NO_WAIT: QStatus = QStatus(4124i32);
pub const ER_THREAD_RUNNING: QStatus = QStatus(4118i32);
pub const ER_THREAD_STOPPING: QStatus = QStatus(4119i32);
pub const ER_TIMEOUT: QStatus = QStatus(10i32);
pub const ER_TIMER_EXITING: QStatus = QStatus(4125i32);
pub const ER_TIMER_FALLBEHIND: QStatus = QStatus(4103i32);
pub const ER_TIMER_FULL: QStatus = QStatus(4130i32);
pub const ER_TIMER_NOT_ALLOWED: QStatus = QStatus(4140i32);
pub const ER_UDP_BACKPRESSURE: QStatus = QStatus(37123i32);
pub const ER_UDP_BUSHELLO: QStatus = QStatus(37129i32);
pub const ER_UDP_DEMUX_NO_ENDPOINT: QStatus = QStatus(37114i32);
pub const ER_UDP_DISCONNECT: QStatus = QStatus(37118i32);
pub const ER_UDP_EARLY_EXIT: QStatus = QStatus(37137i32);
pub const ER_UDP_ENDPOINT_NOT_STARTED: QStatus = QStatus(37149i32);
pub const ER_UDP_ENDPOINT_REMOVED: QStatus = QStatus(37150i32);
pub const ER_UDP_ENDPOINT_STALLED: QStatus = QStatus(37133i32);
pub const ER_UDP_INVALID: QStatus = QStatus(37131i32);
pub const ER_UDP_LOCAL_DISCONNECT: QStatus = QStatus(37136i32);
pub const ER_UDP_LOCAL_DISCONNECT_FAIL: QStatus = QStatus(37138i32);
pub const ER_UDP_MESSAGE: QStatus = QStatus(37130i32);
pub const ER_UDP_MSG_TOO_LONG: QStatus = QStatus(37113i32);
pub const ER_UDP_NOT_DISCONNECTED: QStatus = QStatus(37148i32);
pub const ER_UDP_NOT_IMPLEMENTED: QStatus = QStatus(37119i32);
pub const ER_UDP_NO_LISTENER: QStatus = QStatus(37120i32);
pub const ER_UDP_NO_NETWORK: QStatus = QStatus(37115i32);
pub const ER_UDP_STOPPING: QStatus = QStatus(37121i32);
pub const ER_UDP_UNEXPECTED_FLOW: QStatus = QStatus(37117i32);
pub const ER_UDP_UNEXPECTED_LENGTH: QStatus = QStatus(37116i32);
pub const ER_UDP_UNSUPPORTED: QStatus = QStatus(37132i32);
pub const ER_UNABLE_TO_CONNECT_TO_RENDEZVOUS_SERVER: QStatus = QStatus(37069i32);
pub const ER_UNABLE_TO_SEND_MESSAGE_TO_RENDEZVOUS_SERVER: QStatus = QStatus(37071i32);
pub const ER_UNKNOWN_CERTIFICATE: QStatus = QStatus(37168i32);
pub const ER_UTF_CONVERSION_FAILED: QStatus = QStatus(2i32);
pub const ER_WARNING: QStatus = QStatus(29i32);
pub const ER_WOULDBLOCK: QStatus = QStatus(8i32);
pub const ER_WRITE_ERROR: QStatus = QStatus(23i32);
pub const ER_XML_ACLS_MISSING: QStatus = QStatus(8211i32);
pub const ER_XML_ACL_ALL_TYPE_PEER_WITH_OTHERS: QStatus = QStatus(8207i32);
pub const ER_XML_ACL_PEERS_MISSING: QStatus = QStatus(8212i32);
pub const ER_XML_ACL_PEER_NOT_UNIQUE: QStatus = QStatus(8209i32);
pub const ER_XML_ACL_PEER_PUBLIC_KEY_SET: QStatus = QStatus(8210i32);
pub const ER_XML_ANNOTATION_NOT_UNIQUE: QStatus = QStatus(8222i32);
pub const ER_XML_CONVERTER_ERROR: QStatus = QStatus(8192i32);
pub const ER_XML_INTERFACE_MEMBERS_MISSING: QStatus = QStatus(8194i32);
pub const ER_XML_INTERFACE_NAME_NOT_UNIQUE: QStatus = QStatus(8219i32);
pub const ER_XML_INVALID_ACL_PEER_CHILDREN_COUNT: QStatus = QStatus(8206i32);
pub const ER_XML_INVALID_ACL_PEER_PUBLIC_KEY: QStatus = QStatus(8208i32);
pub const ER_XML_INVALID_ACL_PEER_TYPE: QStatus = QStatus(8205i32);
pub const ER_XML_INVALID_ANNOTATIONS_COUNT: QStatus = QStatus(8198i32);
pub const ER_XML_INVALID_ATTRIBUTE_VALUE: QStatus = QStatus(8200i32);
pub const ER_XML_INVALID_BASE64: QStatus = QStatus(8218i32);
pub const ER_XML_INVALID_ELEMENT_CHILDREN_COUNT: QStatus = QStatus(8202i32);
pub const ER_XML_INVALID_ELEMENT_NAME: QStatus = QStatus(8199i32);
pub const ER_XML_INVALID_INTERFACE_NAME: QStatus = QStatus(8214i32);
pub const ER_XML_INVALID_MANIFEST_VERSION: QStatus = QStatus(8216i32);
pub const ER_XML_INVALID_MEMBER_ACTION: QStatus = QStatus(8196i32);
pub const ER_XML_INVALID_MEMBER_NAME: QStatus = QStatus(8215i32);
pub const ER_XML_INVALID_MEMBER_TYPE: QStatus = QStatus(8195i32);
pub const ER_XML_INVALID_OBJECT_PATH: QStatus = QStatus(8213i32);
pub const ER_XML_INVALID_OID: QStatus = QStatus(8217i32);
pub const ER_XML_INVALID_POLICY_SERIAL_NUMBER: QStatus = QStatus(8204i32);
pub const ER_XML_INVALID_POLICY_VERSION: QStatus = QStatus(8203i32);
pub const ER_XML_INVALID_RULES_COUNT: QStatus = QStatus(8193i32);
pub const ER_XML_INVALID_SECURITY_LEVEL_ANNOTATION_VALUE: QStatus = QStatus(8201i32);
pub const ER_XML_MALFORMED: QStatus = QStatus(4099i32);
pub const ER_XML_MEMBER_DENY_ACTION_WITH_OTHER: QStatus = QStatus(8197i32);
pub const ER_XML_MEMBER_NAME_NOT_UNIQUE: QStatus = QStatus(8220i32);
pub const ER_XML_OBJECT_PATH_NOT_UNIQUE: QStatus = QStatus(8221i32);
pub const NEED_UPDATE: alljoyn_applicationstate = alljoyn_applicationstate(3i32);
pub const NOT_CLAIMABLE: alljoyn_applicationstate = alljoyn_applicationstate(0i32);
pub const PASSWORD_GENERATED_BY_APPLICATION: alljoyn_claimcapabilityadditionalinfo_masks = alljoyn_claimcapabilityadditionalinfo_masks(2i32);
pub const PASSWORD_GENERATED_BY_SECURITY_MANAGER: alljoyn_claimcapabilityadditionalinfo_masks = alljoyn_claimcapabilityadditionalinfo_masks(1i32);
pub const QCC_FALSE: u32 = 0u32;
pub const QCC_TRUE: u32 = 1u32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct QStatus(pub i32);
pub const UNANNOUNCED: alljoyn_about_announceflag = alljoyn_about_announceflag(0i32);
pub type alljoyn_about_announced_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, busname: windows_core::PCSTR, version: u16, port: u16, objectdescriptionarg: alljoyn_msgarg, aboutdataarg: alljoyn_msgarg)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_about_announceflag(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_aboutdata(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_aboutdatalistener(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct alljoyn_aboutdatalistener_callbacks {
    pub about_datalistener_getaboutdata: alljoyn_aboutdatalistener_getaboutdata_ptr,
    pub about_datalistener_getannouncedaboutdata: alljoyn_aboutdatalistener_getannouncedaboutdata_ptr,
}
pub type alljoyn_aboutdatalistener_getaboutdata_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, msgarg: alljoyn_msgarg, language: windows_core::PCSTR) -> QStatus>;
pub type alljoyn_aboutdatalistener_getannouncedaboutdata_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, msgarg: alljoyn_msgarg) -> QStatus>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_abouticon(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_abouticonobj(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_abouticonproxy(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_aboutlistener(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct alljoyn_aboutlistener_callback {
    pub about_listener_announced: alljoyn_about_announced_ptr,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_aboutobj(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_aboutobjectdescription(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_aboutproxy(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_applicationstate(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_applicationstatelistener(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct alljoyn_applicationstatelistener_callbacks {
    pub state: alljoyn_applicationstatelistener_state_ptr,
}
pub type alljoyn_applicationstatelistener_state_ptr = Option<unsafe extern "system" fn(busname: *mut i8, publickey: *mut i8, applicationstate: alljoyn_applicationstate, context: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_authlistener(pub isize);
pub type alljoyn_authlistener_authenticationcomplete_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, success: i32)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct alljoyn_authlistener_callbacks {
    pub request_credentials: alljoyn_authlistener_requestcredentials_ptr,
    pub verify_credentials: alljoyn_authlistener_verifycredentials_ptr,
    pub security_violation: alljoyn_authlistener_securityviolation_ptr,
    pub authentication_complete: alljoyn_authlistener_authenticationcomplete_ptr,
}
pub type alljoyn_authlistener_requestcredentials_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, authcount: u16, username: windows_core::PCSTR, credmask: u16, credentials: alljoyn_credentials) -> i32>;
pub type alljoyn_authlistener_requestcredentialsasync_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_authlistener, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, authcount: u16, username: windows_core::PCSTR, credmask: u16, authcontext: *mut core::ffi::c_void) -> QStatus>;
pub type alljoyn_authlistener_securityviolation_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, status: QStatus, msg: alljoyn_message)>;
pub type alljoyn_authlistener_verifycredentials_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, credentials: alljoyn_credentials) -> i32>;
pub type alljoyn_authlistener_verifycredentialsasync_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_authlistener, authmechanism: windows_core::PCSTR, peername: windows_core::PCSTR, credentials: alljoyn_credentials, authcontext: *mut core::ffi::c_void) -> QStatus>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct alljoyn_authlistenerasync_callbacks {
    pub request_credentials: alljoyn_authlistener_requestcredentialsasync_ptr,
    pub verify_credentials: alljoyn_authlistener_verifycredentialsasync_ptr,
    pub security_violation: alljoyn_authlistener_securityviolation_ptr,
    pub authentication_complete: alljoyn_authlistener_authenticationcomplete_ptr,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_autopinger(pub isize);
pub type alljoyn_autopinger_destination_found_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, group: windows_core::PCSTR, destination: windows_core::PCSTR)>;
pub type alljoyn_autopinger_destination_lost_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, group: windows_core::PCSTR, destination: windows_core::PCSTR)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_busattachment(pub isize);
pub type alljoyn_busattachment_joinsessioncb_ptr = Option<unsafe extern "system" fn(status: QStatus, sessionid: u32, opts: alljoyn_sessionopts, context: *mut core::ffi::c_void)>;
pub type alljoyn_busattachment_setlinktimeoutcb_ptr = Option<unsafe extern "system" fn(status: QStatus, timeout: u32, context: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_buslistener(pub isize);
pub type alljoyn_buslistener_bus_disconnected_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_buslistener_bus_prop_changed_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, prop_name: windows_core::PCSTR, prop_value: alljoyn_msgarg)>;
pub type alljoyn_buslistener_bus_stopping_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
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
pub type alljoyn_buslistener_found_advertised_name_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, name: windows_core::PCSTR, transport: u16, nameprefix: windows_core::PCSTR)>;
pub type alljoyn_buslistener_listener_registered_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, bus: alljoyn_busattachment)>;
pub type alljoyn_buslistener_listener_unregistered_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_buslistener_lost_advertised_name_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, name: windows_core::PCSTR, transport: u16, nameprefix: windows_core::PCSTR)>;
pub type alljoyn_buslistener_name_owner_changed_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, busname: windows_core::PCSTR, previousowner: windows_core::PCSTR, newowner: windows_core::PCSTR)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_busobject(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct alljoyn_busobject_callbacks {
    pub property_get: alljoyn_busobject_prop_get_ptr,
    pub property_set: alljoyn_busobject_prop_set_ptr,
    pub object_registered: alljoyn_busobject_object_registration_ptr,
    pub object_unregistered: alljoyn_busobject_object_registration_ptr,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct alljoyn_busobject_methodentry {
    pub member: *const alljoyn_interfacedescription_member,
    pub method_handler: alljoyn_messagereceiver_methodhandler_ptr,
}
impl Default for alljoyn_busobject_methodentry {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type alljoyn_busobject_object_registration_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_busobject_prop_get_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, ifcname: windows_core::PCSTR, propname: windows_core::PCSTR, val: alljoyn_msgarg) -> QStatus>;
pub type alljoyn_busobject_prop_set_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, ifcname: windows_core::PCSTR, propname: windows_core::PCSTR, val: alljoyn_msgarg) -> QStatus>;
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_claimcapability_masks(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_claimcapabilityadditionalinfo_masks(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_credentials(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_interfacedescription(pub isize);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_interfacedescription_securitypolicy(pub i32);
pub type alljoyn_interfacedescription_translation_callback_ptr = Option<unsafe extern "system" fn(sourcelanguage: windows_core::PCSTR, targetlanguage: windows_core::PCSTR, sourcetext: windows_core::PCSTR) -> windows_core::PCSTR>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_keystore(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_keystorelistener(pub isize);
pub type alljoyn_keystorelistener_acquireexclusivelock_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_keystorelistener) -> QStatus>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct alljoyn_keystorelistener_callbacks {
    pub load_request: alljoyn_keystorelistener_loadrequest_ptr,
    pub store_request: alljoyn_keystorelistener_storerequest_ptr,
}
pub type alljoyn_keystorelistener_loadrequest_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_keystorelistener, keystore: alljoyn_keystore) -> QStatus>;
pub type alljoyn_keystorelistener_releaseexclusivelock_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_keystorelistener)>;
pub type alljoyn_keystorelistener_storerequest_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, listener: alljoyn_keystorelistener, keystore: alljoyn_keystore) -> QStatus>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct alljoyn_keystorelistener_with_synchronization_callbacks {
    pub load_request: alljoyn_keystorelistener_loadrequest_ptr,
    pub store_request: alljoyn_keystorelistener_storerequest_ptr,
    pub acquire_exclusive_lock: alljoyn_keystorelistener_acquireexclusivelock_ptr,
    pub release_exclusive_lock: alljoyn_keystorelistener_releaseexclusivelock_ptr,
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_message(pub isize);
pub type alljoyn_messagereceiver_methodhandler_ptr = Option<unsafe extern "system" fn(bus: alljoyn_busobject, member: *const alljoyn_interfacedescription_member, message: alljoyn_message)>;
pub type alljoyn_messagereceiver_replyhandler_ptr = Option<unsafe extern "system" fn(message: alljoyn_message, context: *mut core::ffi::c_void)>;
pub type alljoyn_messagereceiver_signalhandler_ptr = Option<unsafe extern "system" fn(member: *const alljoyn_interfacedescription_member, srcpath: windows_core::PCSTR, message: alljoyn_message)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_messagetype(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_msgarg(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_observer(pub isize);
pub type alljoyn_observer_object_discovered_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, proxyref: alljoyn_proxybusobject_ref)>;
pub type alljoyn_observer_object_lost_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, proxyref: alljoyn_proxybusobject_ref)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_observerlistener(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct alljoyn_observerlistener_callback {
    pub object_discovered: alljoyn_observer_object_discovered_ptr,
    pub object_lost: alljoyn_observer_object_lost_ptr,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_permissionconfigurationlistener(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct alljoyn_permissionconfigurationlistener_callbacks {
    pub factory_reset: alljoyn_permissionconfigurationlistener_factoryreset_ptr,
    pub policy_changed: alljoyn_permissionconfigurationlistener_policychanged_ptr,
    pub start_management: alljoyn_permissionconfigurationlistener_startmanagement_ptr,
    pub end_management: alljoyn_permissionconfigurationlistener_endmanagement_ptr,
}
pub type alljoyn_permissionconfigurationlistener_endmanagement_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_permissionconfigurationlistener_factoryreset_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void) -> QStatus>;
pub type alljoyn_permissionconfigurationlistener_policychanged_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
pub type alljoyn_permissionconfigurationlistener_startmanagement_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_permissionconfigurator(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_pinglistener(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct alljoyn_pinglistener_callback {
    pub destination_found: alljoyn_autopinger_destination_found_ptr,
    pub destination_lost: alljoyn_autopinger_destination_lost_ptr,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_proxybusobject(pub isize);
pub type alljoyn_proxybusobject_listener_getallpropertiescb_ptr = Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, values: alljoyn_msgarg, context: *mut core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_getpropertycb_ptr = Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, value: alljoyn_msgarg, context: *mut core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_introspectcb_ptr = Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, context: *mut core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_propertieschanged_ptr = Option<unsafe extern "system" fn(obj: alljoyn_proxybusobject, ifacename: windows_core::PCSTR, changed: alljoyn_msgarg, invalidated: alljoyn_msgarg, context: *mut core::ffi::c_void)>;
pub type alljoyn_proxybusobject_listener_setpropertycb_ptr = Option<unsafe extern "system" fn(status: QStatus, obj: alljoyn_proxybusobject, context: *mut core::ffi::c_void)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_proxybusobject_ref(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_securityapplicationproxy(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_sessionlistener(pub isize);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct alljoyn_sessionlistener_callbacks {
    pub session_lost: alljoyn_sessionlistener_sessionlost_ptr,
    pub session_member_added: alljoyn_sessionlistener_sessionmemberadded_ptr,
    pub session_member_removed: alljoyn_sessionlistener_sessionmemberremoved_ptr,
}
pub type alljoyn_sessionlistener_sessionlost_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionid: u32, reason: alljoyn_sessionlostreason)>;
pub type alljoyn_sessionlistener_sessionmemberadded_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionid: u32, uniquename: windows_core::PCSTR)>;
pub type alljoyn_sessionlistener_sessionmemberremoved_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionid: u32, uniquename: windows_core::PCSTR)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_sessionlostreason(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_sessionopts(pub isize);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct alljoyn_sessionportlistener(pub isize);
pub type alljoyn_sessionportlistener_acceptsessionjoiner_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionport: u16, joiner: windows_core::PCSTR, opts: alljoyn_sessionopts) -> i32>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct alljoyn_sessionportlistener_callbacks {
    pub accept_session_joiner: alljoyn_sessionportlistener_acceptsessionjoiner_ptr,
    pub session_joined: alljoyn_sessionportlistener_sessionjoined_ptr,
}
pub type alljoyn_sessionportlistener_sessionjoined_ptr = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, sessionport: u16, id: u32, joiner: windows_core::PCSTR)>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct alljoyn_typeid(pub i32);
