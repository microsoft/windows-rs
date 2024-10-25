pub const BinaryBlobCredential: CRED_MARSHAL_TYPE = 3i32;
pub const BinaryBlobForSystem: CRED_MARSHAL_TYPE = 5i32;
pub const CERT_HASH_LENGTH: u32 = 20u32;
pub const CREDSSP_CRED_EX_VERSION: u32 = 0u32;
pub const CREDSSP_FLAG_REDIRECT: u32 = 1u32;
pub const CREDSSP_NAME: windows_core::PCWSTR = windows_core::w!("CREDSSP");
pub const CREDSSP_SERVER_AUTH_CERTIFICATE: u32 = 2u32;
pub const CREDSSP_SERVER_AUTH_LOOPBACK: u32 = 4u32;
pub const CREDSSP_SERVER_AUTH_NEGOTIATE: u32 = 1u32;
pub const CREDUIWIN_AUTHPACKAGE_ONLY: CREDUIWIN_FLAGS = 16u32;
pub const CREDUIWIN_CHECKBOX: CREDUIWIN_FLAGS = 2u32;
pub const CREDUIWIN_DOWNLEVEL_HELLO_AS_SMART_CARD: u32 = 2147483648u32;
pub const CREDUIWIN_ENUMERATE_ADMINS: CREDUIWIN_FLAGS = 256u32;
pub const CREDUIWIN_ENUMERATE_CURRENT_USER: CREDUIWIN_FLAGS = 512u32;
pub const CREDUIWIN_GENERIC: CREDUIWIN_FLAGS = 1u32;
pub const CREDUIWIN_IGNORE_CLOUDAUTHORITY_NAME: u32 = 262144u32;
pub const CREDUIWIN_IN_CRED_ONLY: CREDUIWIN_FLAGS = 32u32;
pub const CREDUIWIN_PACK_32_WOW: CREDUIWIN_FLAGS = 268435456u32;
pub const CREDUIWIN_PREPROMPTING: CREDUIWIN_FLAGS = 8192u32;
pub const CREDUIWIN_SECURE_PROMPT: CREDUIWIN_FLAGS = 4096u32;
pub const CREDUI_FLAGS_ALWAYS_SHOW_UI: CREDUI_FLAGS = 128u32;
pub const CREDUI_FLAGS_COMPLETE_USERNAME: CREDUI_FLAGS = 2048u32;
pub const CREDUI_FLAGS_DO_NOT_PERSIST: CREDUI_FLAGS = 2u32;
pub const CREDUI_FLAGS_EXCLUDE_CERTIFICATES: CREDUI_FLAGS = 8u32;
pub const CREDUI_FLAGS_EXPECT_CONFIRMATION: CREDUI_FLAGS = 131072u32;
pub const CREDUI_FLAGS_GENERIC_CREDENTIALS: CREDUI_FLAGS = 262144u32;
pub const CREDUI_FLAGS_INCORRECT_PASSWORD: CREDUI_FLAGS = 1u32;
pub const CREDUI_FLAGS_KEEP_USERNAME: CREDUI_FLAGS = 1048576u32;
pub const CREDUI_FLAGS_PASSWORD_ONLY_OK: CREDUI_FLAGS = 512u32;
pub const CREDUI_FLAGS_PERSIST: CREDUI_FLAGS = 4096u32;
pub const CREDUI_FLAGS_REQUEST_ADMINISTRATOR: CREDUI_FLAGS = 4u32;
pub const CREDUI_FLAGS_REQUIRE_CERTIFICATE: CREDUI_FLAGS = 16u32;
pub const CREDUI_FLAGS_REQUIRE_SMARTCARD: CREDUI_FLAGS = 256u32;
pub const CREDUI_FLAGS_SERVER_CREDENTIAL: CREDUI_FLAGS = 16384u32;
pub const CREDUI_FLAGS_SHOW_SAVE_CHECK_BOX: CREDUI_FLAGS = 64u32;
pub const CREDUI_FLAGS_USERNAME_TARGET_CREDENTIALS: CREDUI_FLAGS = 524288u32;
pub const CREDUI_FLAGS_VALIDATE_USERNAME: CREDUI_FLAGS = 1024u32;
pub const CREDUI_MAX_CAPTION_LENGTH: u32 = 128u32;
pub const CREDUI_MAX_DOMAIN_TARGET_LENGTH: u32 = 337u32;
pub const CREDUI_MAX_GENERIC_TARGET_LENGTH: u32 = 32767u32;
pub const CREDUI_MAX_MESSAGE_LENGTH: u32 = 1024u32;
pub const CREDUI_MAX_USERNAME_LENGTH: u32 = 513u32;
pub const CRED_ALLOW_NAME_RESOLUTION: u32 = 1u32;
pub const CRED_CACHE_TARGET_INFORMATION: u32 = 1u32;
pub const CRED_ENUMERATE_ALL_CREDENTIALS: CRED_ENUMERATE_FLAGS = 1u32;
pub const CRED_FLAGS_NGC_CERT: CRED_FLAGS = 128u32;
pub const CRED_FLAGS_OWF_CRED_BLOB: CRED_FLAGS = 8u32;
pub const CRED_FLAGS_PASSWORD_FOR_CERT: CRED_FLAGS = 1u32;
pub const CRED_FLAGS_PROMPT_NOW: CRED_FLAGS = 2u32;
pub const CRED_FLAGS_REQUIRE_CONFIRMATION: CRED_FLAGS = 16u32;
pub const CRED_FLAGS_USERNAME_TARGET: CRED_FLAGS = 4u32;
pub const CRED_FLAGS_VALID_FLAGS: CRED_FLAGS = 61695u32;
pub const CRED_FLAGS_VALID_INPUT_FLAGS: CRED_FLAGS = 61599u32;
pub const CRED_FLAGS_VSM_PROTECTED: CRED_FLAGS = 64u32;
pub const CRED_FLAGS_WILDCARD_MATCH: CRED_FLAGS = 32u32;
pub const CRED_LOGON_TYPES_MASK: u32 = 61440u32;
pub const CRED_MAX_ATTRIBUTES: u32 = 64u32;
pub const CRED_MAX_CREDENTIAL_BLOB_SIZE: u32 = 2560u32;
pub const CRED_MAX_DOMAIN_TARGET_NAME_LENGTH: u32 = 337u32;
pub const CRED_MAX_GENERIC_TARGET_NAME_LENGTH: u32 = 32767u32;
pub const CRED_MAX_STRING_LENGTH: u32 = 256u32;
pub const CRED_MAX_TARGETNAME_ATTRIBUTE_LENGTH: u32 = 256u32;
pub const CRED_MAX_TARGETNAME_NAMESPACE_LENGTH: u32 = 256u32;
pub const CRED_MAX_USERNAME_LENGTH: u32 = 513u32;
pub const CRED_MAX_VALUE_SIZE: u32 = 256u32;
pub const CRED_PACK_GENERIC_CREDENTIALS: CRED_PACK_FLAGS = 4u32;
pub const CRED_PACK_ID_PROVIDER_CREDENTIALS: CRED_PACK_FLAGS = 8u32;
pub const CRED_PACK_PROTECTED_CREDENTIALS: CRED_PACK_FLAGS = 1u32;
pub const CRED_PACK_WOW_BUFFER: CRED_PACK_FLAGS = 2u32;
pub const CRED_PERSIST_ENTERPRISE: CRED_PERSIST = 3u32;
pub const CRED_PERSIST_LOCAL_MACHINE: CRED_PERSIST = 2u32;
pub const CRED_PERSIST_NONE: CRED_PERSIST = 0u32;
pub const CRED_PERSIST_SESSION: CRED_PERSIST = 1u32;
pub const CRED_PRESERVE_CREDENTIAL_BLOB: u32 = 1u32;
pub const CRED_PROTECT_AS_SELF: u32 = 1u32;
pub const CRED_PROTECT_TO_SYSTEM: u32 = 2u32;
pub const CRED_SESSION_WILDCARD_NAME: windows_core::PCWSTR = windows_core::w!("*Session");
pub const CRED_SESSION_WILDCARD_NAME_A: windows_core::PCSTR = windows_core::s!("*Session");
pub const CRED_SESSION_WILDCARD_NAME_W: windows_core::PCWSTR = windows_core::w!("*Session");
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH: windows_core::PCWSTR = windows_core::w!("batch");
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH_A: windows_core::PCSTR = windows_core::s!("batch");
pub const CRED_TARGETNAME_ATTRIBUTE_BATCH_W: windows_core::PCWSTR = windows_core::w!("batch");
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE: windows_core::PCWSTR = windows_core::w!("cachedinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_A: windows_core::PCSTR = windows_core::s!("cachedinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_CACHEDINTERACTIVE_W: windows_core::PCWSTR = windows_core::w!("cachedinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE: windows_core::PCWSTR = windows_core::w!("interactive");
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_A: windows_core::PCSTR = windows_core::s!("interactive");
pub const CRED_TARGETNAME_ATTRIBUTE_INTERACTIVE_W: windows_core::PCWSTR = windows_core::w!("interactive");
pub const CRED_TARGETNAME_ATTRIBUTE_NAME: windows_core::PCWSTR = windows_core::w!("name");
pub const CRED_TARGETNAME_ATTRIBUTE_NAME_A: windows_core::PCSTR = windows_core::s!("name");
pub const CRED_TARGETNAME_ATTRIBUTE_NAME_W: windows_core::PCWSTR = windows_core::w!("name");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK: windows_core::PCWSTR = windows_core::w!("network");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT: windows_core::PCWSTR = windows_core::w!("networkcleartext");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_A: windows_core::PCSTR = windows_core::s!("networkcleartext");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORKCLEARTEXT_W: windows_core::PCWSTR = windows_core::w!("networkcleartext");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK_A: windows_core::PCSTR = windows_core::s!("network");
pub const CRED_TARGETNAME_ATTRIBUTE_NETWORK_W: windows_core::PCWSTR = windows_core::w!("network");
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE: windows_core::PCWSTR = windows_core::w!("remoteinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_A: windows_core::PCSTR = windows_core::s!("remoteinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_REMOTEINTERACTIVE_W: windows_core::PCWSTR = windows_core::w!("remoteinteractive");
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE: windows_core::PCWSTR = windows_core::w!("service");
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE_A: windows_core::PCSTR = windows_core::s!("service");
pub const CRED_TARGETNAME_ATTRIBUTE_SERVICE_W: windows_core::PCWSTR = windows_core::w!("service");
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET: windows_core::PCWSTR = windows_core::w!("target");
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET_A: windows_core::PCSTR = windows_core::s!("target");
pub const CRED_TARGETNAME_ATTRIBUTE_TARGET_W: windows_core::PCWSTR = windows_core::w!("target");
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE: windows_core::PCWSTR = windows_core::w!("Domain");
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE_A: windows_core::PCSTR = windows_core::s!("Domain");
pub const CRED_TARGETNAME_DOMAIN_NAMESPACE_W: windows_core::PCWSTR = windows_core::w!("Domain");
pub const CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_A: windows_core::PCSTR = windows_core::s!("LegacyGeneric");
pub const CRED_TARGETNAME_LEGACYGENERIC_NAMESPACE_W: windows_core::PCWSTR = windows_core::w!("LegacyGeneric");
pub const CRED_TI_CREATE_EXPLICIT_CRED: u32 = 16u32;
pub const CRED_TI_DNSTREE_IS_DFS_SERVER: u32 = 64u32;
pub const CRED_TI_DOMAIN_FORMAT_UNKNOWN: u32 = 2u32;
pub const CRED_TI_ONLY_PASSWORD_REQUIRED: u32 = 4u32;
pub const CRED_TI_SERVER_FORMAT_UNKNOWN: u32 = 1u32;
pub const CRED_TI_USERNAME_TARGET: u32 = 8u32;
pub const CRED_TI_VALID_FLAGS: u32 = 61567u32;
pub const CRED_TI_WORKGROUP_MEMBER: u32 = 32u32;
pub const CRED_TYPE_DOMAIN_CERTIFICATE: CRED_TYPE = 3u32;
pub const CRED_TYPE_DOMAIN_EXTENDED: CRED_TYPE = 6u32;
pub const CRED_TYPE_DOMAIN_PASSWORD: CRED_TYPE = 2u32;
pub const CRED_TYPE_DOMAIN_VISIBLE_PASSWORD: CRED_TYPE = 4u32;
pub const CRED_TYPE_GENERIC: CRED_TYPE = 1u32;
pub const CRED_TYPE_GENERIC_CERTIFICATE: CRED_TYPE = 5u32;
pub const CRED_TYPE_MAXIMUM: CRED_TYPE = 7u32;
pub const CRED_TYPE_MAXIMUM_EX: CRED_TYPE = 1007u32;
pub const CRED_UNPROTECT_ALLOW_TO_SYSTEM: u32 = 2u32;
pub const CRED_UNPROTECT_AS_SELF: u32 = 1u32;
pub const CertCredential: CRED_MARSHAL_TYPE = 1i32;
pub const CredForSystemProtection: CRED_PROTECTION_TYPE = 3i32;
pub const CredTrustedProtection: CRED_PROTECTION_TYPE = 2i32;
pub const CredUnprotected: CRED_PROTECTION_TYPE = 0i32;
pub const CredUserProtection: CRED_PROTECTION_TYPE = 1i32;
pub const CredsspCertificateCreds: CREDSPP_SUBMIT_TYPE = 13i32;
pub const CredsspCredEx: CREDSPP_SUBMIT_TYPE = 100i32;
pub const CredsspPasswordCreds: CREDSPP_SUBMIT_TYPE = 2i32;
pub const CredsspSchannelCreds: CREDSPP_SUBMIT_TYPE = 4i32;
pub const CredsspSubmitBufferBoth: CREDSPP_SUBMIT_TYPE = 50i32;
pub const CredsspSubmitBufferBothOld: CREDSPP_SUBMIT_TYPE = 51i32;
pub const FILE_DEVICE_SMARTCARD: u32 = 49u32;
pub const GUID_DEVINTERFACE_SMARTCARD_READER: windows_core::GUID = windows_core::GUID::from_u128(0x50dd5230_ba8a_11d1_bf5d_0000f805f530);
pub const KeyCredentialManagerOperationErrorStateCertificateFailure: KeyCredentialManagerOperationErrorStates = 4i32;
pub const KeyCredentialManagerOperationErrorStateDeviceJoinFailure: KeyCredentialManagerOperationErrorStates = 1i32;
pub const KeyCredentialManagerOperationErrorStateHardwareFailure: KeyCredentialManagerOperationErrorStates = 32i32;
pub const KeyCredentialManagerOperationErrorStateNone: KeyCredentialManagerOperationErrorStates = 0i32;
pub const KeyCredentialManagerOperationErrorStatePinExistsFailure: KeyCredentialManagerOperationErrorStates = 64i32;
pub const KeyCredentialManagerOperationErrorStatePolicyFailure: KeyCredentialManagerOperationErrorStates = 16i32;
pub const KeyCredentialManagerOperationErrorStateRemoteSessionFailure: KeyCredentialManagerOperationErrorStates = 8i32;
pub const KeyCredentialManagerOperationErrorStateTokenFailure: KeyCredentialManagerOperationErrorStates = 2i32;
pub const KeyCredentialManagerPinChange: KeyCredentialManagerOperationType = 1i32;
pub const KeyCredentialManagerPinReset: KeyCredentialManagerOperationType = 2i32;
pub const KeyCredentialManagerProvisioning: KeyCredentialManagerOperationType = 0i32;
pub const MAXIMUM_ATTR_STRING_LENGTH: u32 = 32u32;
pub const MAXIMUM_SMARTCARD_READERS: u32 = 10u32;
pub const RSR_MATCH_TYPE_ALL_CARDS: READER_SEL_REQUEST_MATCH_TYPE = 3i32;
pub const RSR_MATCH_TYPE_READER_AND_CONTAINER: READER_SEL_REQUEST_MATCH_TYPE = 1i32;
pub const RSR_MATCH_TYPE_SERIAL_NUMBER: READER_SEL_REQUEST_MATCH_TYPE = 2i32;
pub const SCARD_ABSENT: u32 = 1u32;
pub const SCARD_ALL_READERS: windows_core::PCWSTR = windows_core::w!("SCard$AllReaders\u{0}00");
pub const SCARD_ATR_LENGTH: u32 = 33u32;
pub const SCARD_AUDIT_CHV_FAILURE: u32 = 0u32;
pub const SCARD_AUDIT_CHV_SUCCESS: u32 = 1u32;
pub const SCARD_CLASS_COMMUNICATIONS: u32 = 2u32;
pub const SCARD_CLASS_ICC_STATE: u32 = 9u32;
pub const SCARD_CLASS_IFD_PROTOCOL: u32 = 8u32;
pub const SCARD_CLASS_MECHANICAL: u32 = 6u32;
pub const SCARD_CLASS_PERF: u32 = 32766u32;
pub const SCARD_CLASS_POWER_MGMT: u32 = 4u32;
pub const SCARD_CLASS_PROTOCOL: u32 = 3u32;
pub const SCARD_CLASS_SECURITY: u32 = 5u32;
pub const SCARD_CLASS_SYSTEM: u32 = 32767u32;
pub const SCARD_CLASS_VENDOR_DEFINED: u32 = 7u32;
pub const SCARD_CLASS_VENDOR_INFO: u32 = 1u32;
pub const SCARD_COLD_RESET: u32 = 1u32;
pub const SCARD_DEFAULT_READERS: windows_core::PCWSTR = windows_core::w!("SCard$DefaultReaders\u{0}00");
pub const SCARD_EJECT_CARD: u32 = 3u32;
pub const SCARD_LEAVE_CARD: u32 = 0u32;
pub const SCARD_LOCAL_READERS: windows_core::PCWSTR = windows_core::w!("SCard$LocalReaders\u{0}00");
pub const SCARD_NEGOTIABLE: u32 = 5u32;
pub const SCARD_POWERED: u32 = 4u32;
pub const SCARD_POWER_DOWN: u32 = 0u32;
pub const SCARD_PRESENT: u32 = 2u32;
pub const SCARD_PROTOCOL_DEFAULT: u32 = 2147483648u32;
pub const SCARD_PROTOCOL_OPTIMAL: u32 = 0u32;
pub const SCARD_PROTOCOL_RAW: u32 = 65536u32;
pub const SCARD_PROTOCOL_T0: u32 = 1u32;
pub const SCARD_PROTOCOL_T1: u32 = 2u32;
pub const SCARD_PROTOCOL_UNDEFINED: u32 = 0u32;
pub const SCARD_PROVIDER_CSP: u32 = 2u32;
pub const SCARD_PROVIDER_KSP: u32 = 3u32;
pub const SCARD_PROVIDER_PRIMARY: u32 = 1u32;
pub const SCARD_READER_CONFISCATES: u32 = 4u32;
pub const SCARD_READER_CONTACTLESS: u32 = 8u32;
pub const SCARD_READER_EJECTS: u32 = 2u32;
pub const SCARD_READER_SWALLOWS: u32 = 1u32;
pub const SCARD_READER_TYPE_EMBEDDEDSE: u32 = 2048u32;
pub const SCARD_READER_TYPE_IDE: u32 = 16u32;
pub const SCARD_READER_TYPE_KEYBOARD: u32 = 4u32;
pub const SCARD_READER_TYPE_NFC: u32 = 256u32;
pub const SCARD_READER_TYPE_NGC: u32 = 1024u32;
pub const SCARD_READER_TYPE_PARALELL: u32 = 2u32;
pub const SCARD_READER_TYPE_PCMCIA: u32 = 64u32;
pub const SCARD_READER_TYPE_SCSI: u32 = 8u32;
pub const SCARD_READER_TYPE_SERIAL: u32 = 1u32;
pub const SCARD_READER_TYPE_TPM: u32 = 128u32;
pub const SCARD_READER_TYPE_UICC: u32 = 512u32;
pub const SCARD_READER_TYPE_USB: u32 = 32u32;
pub const SCARD_READER_TYPE_VENDOR: u32 = 240u32;
pub const SCARD_RESET_CARD: u32 = 1u32;
pub const SCARD_SCOPE_SYSTEM: SCARD_SCOPE = 2u32;
pub const SCARD_SCOPE_TERMINAL: u32 = 1u32;
pub const SCARD_SCOPE_USER: SCARD_SCOPE = 0u32;
pub const SCARD_SHARE_DIRECT: u32 = 3u32;
pub const SCARD_SHARE_EXCLUSIVE: u32 = 1u32;
pub const SCARD_SHARE_SHARED: u32 = 2u32;
pub const SCARD_SPECIFIC: u32 = 6u32;
pub const SCARD_STATE_ATRMATCH: SCARD_STATE = 64u32;
pub const SCARD_STATE_CHANGED: SCARD_STATE = 2u32;
pub const SCARD_STATE_EMPTY: SCARD_STATE = 16u32;
pub const SCARD_STATE_EXCLUSIVE: SCARD_STATE = 128u32;
pub const SCARD_STATE_IGNORE: SCARD_STATE = 1u32;
pub const SCARD_STATE_INUSE: SCARD_STATE = 256u32;
pub const SCARD_STATE_MUTE: SCARD_STATE = 512u32;
pub const SCARD_STATE_PRESENT: SCARD_STATE = 32u32;
pub const SCARD_STATE_UNAVAILABLE: SCARD_STATE = 8u32;
pub const SCARD_STATE_UNAWARE: SCARD_STATE = 0u32;
pub const SCARD_STATE_UNKNOWN: SCARD_STATE = 4u32;
pub const SCARD_STATE_UNPOWERED: u32 = 1024u32;
pub const SCARD_SWALLOWED: u32 = 3u32;
pub const SCARD_SYSTEM_READERS: windows_core::PCWSTR = windows_core::w!("SCard$SystemReaders\u{0}00");
pub const SCARD_T0_CMD_LENGTH: u32 = 5u32;
pub const SCARD_T0_HEADER_LENGTH: u32 = 7u32;
pub const SCARD_T1_EPILOGUE_LENGTH: u32 = 2u32;
pub const SCARD_T1_EPILOGUE_LENGTH_LRC: u32 = 1u32;
pub const SCARD_T1_MAX_IFS: u32 = 254u32;
pub const SCARD_T1_PROLOGUE_LENGTH: u32 = 3u32;
pub const SCARD_UNKNOWN: u32 = 0u32;
pub const SCARD_UNPOWER_CARD: u32 = 2u32;
pub const SCARD_WARM_RESET: u32 = 2u32;
pub const SCERR_NOCARDNAME: u32 = 16384u32;
pub const SCERR_NOGUIDS: u32 = 32768u32;
pub const SC_DLG_FORCE_UI: u32 = 4u32;
pub const SC_DLG_MINIMAL_UI: u32 = 1u32;
pub const SC_DLG_NO_UI: u32 = 2u32;
pub const SECPKG_ALT_ATTR: u32 = 2147483648u32;
pub const SECPKG_ATTR_C_FULL_IDENT_TOKEN: u32 = 2147483781u32;
pub const STATUS_ACCOUNT_DISABLED: super::super::Foundation::NTSTATUS = 0xC0000072_u32 as _;
pub const STATUS_ACCOUNT_EXPIRED: super::super::Foundation::NTSTATUS = 0xC0000193_u32 as _;
pub const STATUS_ACCOUNT_LOCKED_OUT: super::super::Foundation::NTSTATUS = 0xC0000234_u32 as _;
pub const STATUS_ACCOUNT_RESTRICTION: super::super::Foundation::NTSTATUS = 0xC000006E_u32 as _;
pub const STATUS_AUTHENTICATION_FIREWALL_FAILED: super::super::Foundation::NTSTATUS = 0xC0000413_u32 as _;
pub const STATUS_DOWNGRADE_DETECTED: super::super::Foundation::NTSTATUS = 0xC0000388_u32 as _;
pub const STATUS_LOGON_FAILURE: super::super::Foundation::NTSTATUS = 0xC000006D_u32 as _;
pub const STATUS_LOGON_TYPE_NOT_GRANTED: super::super::Foundation::NTSTATUS = 0xC000015B_u32 as _;
pub const STATUS_NO_SUCH_LOGON_SESSION: super::super::Foundation::NTSTATUS = 0xC000005F_u32 as _;
pub const STATUS_NO_SUCH_USER: super::super::Foundation::NTSTATUS = 0xC0000064_u32 as _;
pub const STATUS_PASSWORD_EXPIRED: super::super::Foundation::NTSTATUS = 0xC0000071_u32 as _;
pub const STATUS_PASSWORD_MUST_CHANGE: super::super::Foundation::NTSTATUS = 0xC0000224_u32 as _;
pub const STATUS_WRONG_PASSWORD: super::super::Foundation::NTSTATUS = 0xC000006A_u32 as _;
pub const TS_SSP_NAME: windows_core::PCWSTR = windows_core::w!("TSSSP");
pub const TS_SSP_NAME_A: windows_core::PCSTR = windows_core::s!("TSSSP");
pub const UsernameForPackedCredentials: CRED_MARSHAL_TYPE = 4i32;
pub const UsernameTargetCredential: CRED_MARSHAL_TYPE = 2i32;
pub const szOID_TS_KP_TS_SERVER_AUTH: windows_core::PCSTR = windows_core::s!("1.3.6.1.4.1.311.54.1.2");
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CREDSPP_SUBMIT_TYPE(pub i32);
impl windows_core::TypeKind for CREDSPP_SUBMIT_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CREDUIWIN_FLAGS(pub u32);
impl windows_core::TypeKind for CREDUIWIN_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CREDUI_FLAGS(pub u32);
impl windows_core::TypeKind for CREDUI_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CRED_ENUMERATE_FLAGS(pub u32);
impl windows_core::TypeKind for CRED_ENUMERATE_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CRED_FLAGS(pub u32);
impl windows_core::TypeKind for CRED_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CRED_MARSHAL_TYPE(pub i32);
impl windows_core::TypeKind for CRED_MARSHAL_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CRED_PACK_FLAGS(pub u32);
impl windows_core::TypeKind for CRED_PACK_FLAGS {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CRED_PERSIST(pub u32);
impl windows_core::TypeKind for CRED_PERSIST {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CRED_PROTECTION_TYPE(pub i32);
impl windows_core::TypeKind for CRED_PROTECTION_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CRED_TYPE(pub u32);
impl windows_core::TypeKind for CRED_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyCredentialManagerOperationErrorStates(pub i32);
impl windows_core::TypeKind for KeyCredentialManagerOperationErrorStates {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KeyCredentialManagerOperationType(pub i32);
impl windows_core::TypeKind for KeyCredentialManagerOperationType {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct READER_SEL_REQUEST_MATCH_TYPE(pub i32);
impl windows_core::TypeKind for READER_SEL_REQUEST_MATCH_TYPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SCARD_SCOPE(pub u32);
impl windows_core::TypeKind for SCARD_SCOPE {
    type TypeKind = windows_core::CopyType;
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SCARD_STATE(pub u32);
impl windows_core::TypeKind for SCARD_STATE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BINARY_BLOB_CREDENTIAL_INFO {
    pub cbBlob: u32,
    pub pbBlob: *mut u8,
}
impl Default for BINARY_BLOB_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for BINARY_BLOB_CREDENTIAL_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CERT_CREDENTIAL_INFO {
    pub cbSize: u32,
    pub rgbHashOfCert: [u8; 20],
}
impl Default for CERT_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CERT_CREDENTIAL_INFO {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDENTIALA {
    pub Flags: CRED_FLAGS,
    pub Type: CRED_TYPE,
    pub TargetName: windows_core::PSTR,
    pub Comment: windows_core::PSTR,
    pub LastWritten: super::super::Foundation::FILETIME,
    pub CredentialBlobSize: u32,
    pub CredentialBlob: *mut u8,
    pub Persist: CRED_PERSIST,
    pub AttributeCount: u32,
    pub Attributes: *mut CREDENTIAL_ATTRIBUTEA,
    pub TargetAlias: windows_core::PSTR,
    pub UserName: windows_core::PSTR,
}
impl Default for CREDENTIALA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CREDENTIALA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDENTIALW {
    pub Flags: CRED_FLAGS,
    pub Type: CRED_TYPE,
    pub TargetName: windows_core::PWSTR,
    pub Comment: windows_core::PWSTR,
    pub LastWritten: super::super::Foundation::FILETIME,
    pub CredentialBlobSize: u32,
    pub CredentialBlob: *mut u8,
    pub Persist: CRED_PERSIST,
    pub AttributeCount: u32,
    pub Attributes: *mut CREDENTIAL_ATTRIBUTEW,
    pub TargetAlias: windows_core::PWSTR,
    pub UserName: windows_core::PWSTR,
}
impl Default for CREDENTIALW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CREDENTIALW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDENTIAL_ATTRIBUTEA {
    pub Keyword: windows_core::PSTR,
    pub Flags: u32,
    pub ValueSize: u32,
    pub Value: *mut u8,
}
impl Default for CREDENTIAL_ATTRIBUTEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CREDENTIAL_ATTRIBUTEA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDENTIAL_ATTRIBUTEW {
    pub Keyword: windows_core::PWSTR,
    pub Flags: u32,
    pub ValueSize: u32,
    pub Value: *mut u8,
}
impl Default for CREDENTIAL_ATTRIBUTEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CREDENTIAL_ATTRIBUTEW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDENTIAL_TARGET_INFORMATIONA {
    pub TargetName: windows_core::PSTR,
    pub NetbiosServerName: windows_core::PSTR,
    pub DnsServerName: windows_core::PSTR,
    pub NetbiosDomainName: windows_core::PSTR,
    pub DnsDomainName: windows_core::PSTR,
    pub DnsTreeName: windows_core::PSTR,
    pub PackageName: windows_core::PSTR,
    pub Flags: u32,
    pub CredTypeCount: u32,
    pub CredTypes: *mut u32,
}
impl Default for CREDENTIAL_TARGET_INFORMATIONA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CREDENTIAL_TARGET_INFORMATIONA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDENTIAL_TARGET_INFORMATIONW {
    pub TargetName: windows_core::PWSTR,
    pub NetbiosServerName: windows_core::PWSTR,
    pub DnsServerName: windows_core::PWSTR,
    pub NetbiosDomainName: windows_core::PWSTR,
    pub DnsDomainName: windows_core::PWSTR,
    pub DnsTreeName: windows_core::PWSTR,
    pub PackageName: windows_core::PWSTR,
    pub Flags: u32,
    pub CredTypeCount: u32,
    pub CredTypes: *mut u32,
}
impl Default for CREDENTIAL_TARGET_INFORMATIONW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CREDENTIAL_TARGET_INFORMATIONW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDSSP_CRED {
    pub Type: CREDSPP_SUBMIT_TYPE,
    pub pSchannelCred: *mut core::ffi::c_void,
    pub pSpnegoCred: *mut core::ffi::c_void,
}
impl Default for CREDSSP_CRED {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CREDSSP_CRED {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDSSP_CRED_EX {
    pub Type: CREDSPP_SUBMIT_TYPE,
    pub Version: u32,
    pub Flags: u32,
    pub Reserved: u32,
    pub Cred: CREDSSP_CRED,
}
impl Default for CREDSSP_CRED_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for CREDSSP_CRED_EX {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDUI_INFOA {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pszMessageText: windows_core::PCSTR,
    pub pszCaptionText: windows_core::PCSTR,
    pub hbmBanner: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CREDUI_INFOA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for CREDUI_INFOA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Gdi")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CREDUI_INFOW {
    pub cbSize: u32,
    pub hwndParent: super::super::Foundation::HWND,
    pub pszMessageText: windows_core::PCWSTR,
    pub pszCaptionText: windows_core::PCWSTR,
    pub hbmBanner: super::super::Graphics::Gdi::HBITMAP,
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl Default for CREDUI_INFOW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Gdi")]
impl windows_core::TypeKind for CREDUI_INFOW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KeyCredentialManagerInfo {
    pub containerId: windows_core::GUID,
}
impl Default for KeyCredentialManagerInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for KeyCredentialManagerInfo {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPENCARDNAMEA {
    pub dwStructSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hSCardContext: usize,
    pub lpstrGroupNames: windows_core::PSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: windows_core::PSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: *const windows_core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: windows_core::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_core::PSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: windows_core::PCSTR,
    pub dwFlags: u32,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub dwActiveProtocol: u32,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: usize,
}
impl Default for OPENCARDNAMEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for OPENCARDNAMEA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPENCARDNAMEW {
    pub dwStructSize: u32,
    pub hwndOwner: super::super::Foundation::HWND,
    pub hSCardContext: usize,
    pub lpstrGroupNames: windows_core::PWSTR,
    pub nMaxGroupNames: u32,
    pub lpstrCardNames: windows_core::PWSTR,
    pub nMaxCardNames: u32,
    pub rgguidInterfaces: *const windows_core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrRdr: windows_core::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_core::PWSTR,
    pub nMaxCard: u32,
    pub lpstrTitle: windows_core::PCWSTR,
    pub dwFlags: u32,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub dwActiveProtocol: u32,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub hCardHandle: usize,
}
impl Default for OPENCARDNAMEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for OPENCARDNAMEW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPENCARDNAME_EXA {
    pub dwStructSize: u32,
    pub hSCardContext: usize,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub lpstrTitle: windows_core::PCSTR,
    pub lpstrSearchDesc: windows_core::PCSTR,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pOpenCardSearchCriteria: *mut OPENCARD_SEARCH_CRITERIAA,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: windows_core::PSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_core::PSTR,
    pub nMaxCard: u32,
    pub dwActiveProtocol: u32,
    pub hCardHandle: usize,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for OPENCARDNAME_EXA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for OPENCARDNAME_EXA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPENCARDNAME_EXW {
    pub dwStructSize: u32,
    pub hSCardContext: usize,
    pub hwndOwner: super::super::Foundation::HWND,
    pub dwFlags: u32,
    pub lpstrTitle: windows_core::PCWSTR,
    pub lpstrSearchDesc: windows_core::PCWSTR,
    pub hIcon: super::super::UI::WindowsAndMessaging::HICON,
    pub pOpenCardSearchCriteria: *mut OPENCARD_SEARCH_CRITERIAW,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub lpstrRdr: windows_core::PWSTR,
    pub nMaxRdr: u32,
    pub lpstrCard: windows_core::PWSTR,
    pub nMaxCard: u32,
    pub dwActiveProtocol: u32,
    pub hCardHandle: usize,
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl Default for OPENCARDNAME_EXW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_UI_WindowsAndMessaging")]
impl windows_core::TypeKind for OPENCARDNAME_EXW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPENCARD_SEARCH_CRITERIAA {
    pub dwStructSize: u32,
    pub lpstrGroupNames: windows_core::PSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: *const windows_core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: windows_core::PSTR,
    pub nMaxCardNames: u32,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCA,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
}
impl Default for OPENCARD_SEARCH_CRITERIAA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for OPENCARD_SEARCH_CRITERIAA {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OPENCARD_SEARCH_CRITERIAW {
    pub dwStructSize: u32,
    pub lpstrGroupNames: windows_core::PWSTR,
    pub nMaxGroupNames: u32,
    pub rgguidInterfaces: *const windows_core::GUID,
    pub cguidInterfaces: u32,
    pub lpstrCardNames: windows_core::PWSTR,
    pub nMaxCardNames: u32,
    pub lpfnCheck: LPOCNCHKPROC,
    pub lpfnConnect: LPOCNCONNPROCW,
    pub lpfnDisconnect: LPOCNDSCPROC,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
}
impl Default for OPENCARD_SEARCH_CRITERIAW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for OPENCARD_SEARCH_CRITERIAW {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct READER_SEL_REQUEST {
    pub dwShareMode: u32,
    pub dwPreferredProtocols: u32,
    pub MatchType: READER_SEL_REQUEST_MATCH_TYPE,
    pub Anonymous: READER_SEL_REQUEST_0,
}
impl Default for READER_SEL_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for READER_SEL_REQUEST {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union READER_SEL_REQUEST_0 {
    pub ReaderAndContainerParameter: READER_SEL_REQUEST_0_0,
    pub SerialNumberParameter: READER_SEL_REQUEST_0_1,
}
impl Default for READER_SEL_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for READER_SEL_REQUEST_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct READER_SEL_REQUEST_0_0 {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbContainerNameOffset: u32,
    pub cchContainerNameLength: u32,
    pub dwDesiredCardModuleVersion: u32,
    pub dwCspFlags: u32,
}
impl Default for READER_SEL_REQUEST_0_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for READER_SEL_REQUEST_0_0 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct READER_SEL_REQUEST_0_1 {
    pub cbSerialNumberOffset: u32,
    pub cbSerialNumberLength: u32,
    pub dwDesiredCardModuleVersion: u32,
}
impl Default for READER_SEL_REQUEST_0_1 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for READER_SEL_REQUEST_0_1 {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct READER_SEL_RESPONSE {
    pub cbReaderNameOffset: u32,
    pub cchReaderNameLength: u32,
    pub cbCardNameOffset: u32,
    pub cchCardNameLength: u32,
}
impl Default for READER_SEL_RESPONSE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for READER_SEL_RESPONSE {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCARD_ATRMASK {
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
    pub rgbMask: [u8; 36],
}
impl Default for SCARD_ATRMASK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SCARD_ATRMASK {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCARD_IO_REQUEST {
    pub dwProtocol: u32,
    pub cbPciLength: u32,
}
impl Default for SCARD_IO_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SCARD_IO_REQUEST {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCARD_READERSTATEA {
    pub szReader: windows_core::PCSTR,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwCurrentState: SCARD_STATE,
    pub dwEventState: SCARD_STATE,
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
}
impl Default for SCARD_READERSTATEA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SCARD_READERSTATEA {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCARD_READERSTATEW {
    pub szReader: windows_core::PCWSTR,
    pub pvUserData: *mut core::ffi::c_void,
    pub dwCurrentState: SCARD_STATE,
    pub dwEventState: SCARD_STATE,
    pub cbAtr: u32,
    pub rgbAtr: [u8; 36],
}
impl Default for SCARD_READERSTATEW {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SCARD_READERSTATEW {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCARD_T0_COMMAND {
    pub bCla: u8,
    pub bIns: u8,
    pub bP1: u8,
    pub bP2: u8,
    pub bP3: u8,
}
impl Default for SCARD_T0_COMMAND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SCARD_T0_COMMAND {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCARD_T0_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
    pub bSw1: u8,
    pub bSw2: u8,
    pub Anonymous: SCARD_T0_REQUEST_0,
}
impl Default for SCARD_T0_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SCARD_T0_REQUEST {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub union SCARD_T0_REQUEST_0 {
    pub CmdBytes: SCARD_T0_COMMAND,
    pub rgbHeader: [u8; 5],
}
impl Default for SCARD_T0_REQUEST_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SCARD_T0_REQUEST_0 {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SCARD_T1_REQUEST {
    pub ioRequest: SCARD_IO_REQUEST,
}
impl Default for SCARD_T1_REQUEST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SCARD_T1_REQUEST {
    type TypeKind = windows_core::CloneType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecHandle {
    pub dwLower: usize,
    pub dwUpper: usize,
}
impl Default for SecHandle {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SecHandle {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SecPkgContext_ClientCreds {
    pub AuthBufferLen: u32,
    pub AuthBuffer: *mut u8,
}
impl Default for SecPkgContext_ClientCreds {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for SecPkgContext_ClientCreds {
    type TypeKind = windows_core::CopyType;
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct USERNAME_TARGET_CREDENTIAL_INFO {
    pub UserName: windows_core::PWSTR,
}
impl Default for USERNAME_TARGET_CREDENTIAL_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
impl windows_core::TypeKind for USERNAME_TARGET_CREDENTIAL_INFO {
    type TypeKind = windows_core::CopyType;
}
pub type LPOCNCHKPROC = Option<unsafe extern "system" fn(param0: usize, param1: usize, param2: *const core::ffi::c_void) -> super::super::Foundation::BOOL>;
pub type LPOCNCONNPROCA = Option<unsafe extern "system" fn(param0: usize, param1: windows_core::PCSTR, param2: windows_core::PCSTR, param3: *const core::ffi::c_void) -> usize>;
pub type LPOCNCONNPROCW = Option<unsafe extern "system" fn(param0: usize, param1: windows_core::PCWSTR, param2: windows_core::PCWSTR, param3: *const core::ffi::c_void) -> usize>;
pub type LPOCNDSCPROC = Option<unsafe extern "system" fn(param0: usize, param1: usize, param2: *const core::ffi::c_void)>;
