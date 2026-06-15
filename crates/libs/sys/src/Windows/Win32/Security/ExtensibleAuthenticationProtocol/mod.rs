windows_link::link!("eappprxy.dll" "system" fn EapHostPeerBeginSession(dwflags : u32, eaptype : EAP_METHOD_TYPE, pattributearray : *const EAP_ATTRIBUTES, htokenimpersonateuser : super::super::Foundation::HANDLE, dwsizeofconnectiondata : u32, pconnectiondata : *const u8, dwsizeofuserdata : u32, puserdata : *const u8, dwmaxsendpacketsize : u32, pconnectionid : *const windows_sys::core::GUID, func : NotificationHandler, pcontextdata : *const core::ffi::c_void, psessionid : *mut u32, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerClearConnection(pconnectionid : *const windows_sys::core::GUID, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerEndSession(sessionhandle : u32, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerFreeEapError(peaperror : *const EAP_ERROR));
windows_link::link!("eappcfg.dll" "system" fn EapHostPeerFreeErrorMemory(peaperror : *mut EAP_ERROR));
windows_link::link!("eappcfg.dll" "system" fn EapHostPeerFreeMemory(pdata : *mut u8));
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerFreeRuntimeMemory(pdata : *const u8));
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerGetAuthStatus(sessionhandle : u32, authparam : EapHostPeerAuthParams, pcbauthdata : *mut u32, ppauthdata : *mut *mut u8, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerGetDataToUnplumbCredentials(pconnectionidthatlastsavedcreds : *mut windows_sys::core::GUID, phcredentialimpersonationtoken : *mut isize, sessionhandle : u32, ppeaperror : *mut *mut EAP_ERROR, fsavetocredman : *mut windows_sys::core::BOOL) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerGetEncryptedPassword(dwsizeofpassword : u32, szpassword : windows_sys::core::PCWSTR, ppszencpassword : *mut windows_sys::core::PWSTR) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerGetIdentity(dwversion : u32, dwflags : u32, eapmethodtype : EAP_METHOD_TYPE, dwsizeofconnectiondata : u32, pconnectiondata : *const u8, dwsizeofuserdata : u32, puserdata : *const u8, htokenimpersonateuser : super::super::Foundation::HANDLE, pfinvokeui : *mut windows_sys::core::BOOL, pdwsizeofuserdataout : *mut u32, ppuserdataout : *mut *mut u8, ppwszidentity : *mut windows_sys::core::PWSTR, ppeaperror : *mut *mut EAP_ERROR, ppvreserved : *mut *mut u8) -> u32);
windows_link::link!("eappcfg.dll" "system" fn EapHostPeerGetMethodProperties(dwversion : u32, dwflags : u32, eapmethodtype : EAP_METHOD_TYPE, huserimpersonationtoken : super::super::Foundation::HANDLE, dweapconndatasize : u32, pbeapconndata : *const u8, dwuserdatasize : u32, pbuserdata : *const u8, pmethodpropertyarray : *mut EAP_METHOD_PROPERTY_ARRAY, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappcfg.dll" "system" fn EapHostPeerGetMethods(peapmethodinfoarray : *mut EAP_METHOD_INFO_ARRAY, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerGetResponseAttributes(sessionhandle : u32, pattribs : *mut EAP_ATTRIBUTES, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerGetResult(sessionhandle : u32, reason : EapHostPeerMethodResultReason, ppresult : *mut EapHostPeerMethodResult, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerGetSendPacket(sessionhandle : u32, pcbsendpacket : *mut u32, ppsendpacket : *mut *mut u8, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerGetUIContext(sessionhandle : u32, pdwsizeofuicontextdata : *mut u32, ppuicontextdata : *mut *mut u8, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerInitialize() -> u32);
windows_link::link!("eappcfg.dll" "system" fn EapHostPeerInvokeConfigUI(hwndparent : super::super::Foundation::HWND, dwflags : u32, eapmethodtype : EAP_METHOD_TYPE, dwsizeofconfigin : u32, pconfigin : *const u8, pdwsizeofconfigout : *mut u32, ppconfigout : *mut *mut u8, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappcfg.dll" "system" fn EapHostPeerInvokeIdentityUI(dwversion : u32, eapmethodtype : EAP_METHOD_TYPE, dwflags : u32, hwndparent : super::super::Foundation::HWND, dwsizeofconnectiondata : u32, pconnectiondata : *const u8, dwsizeofuserdata : u32, puserdata : *const u8, pdwsizeofuserdataout : *mut u32, ppuserdataout : *mut *mut u8, ppwszidentity : *mut windows_sys::core::PWSTR, ppeaperror : *mut *mut EAP_ERROR, ppvreserved : *mut *mut core::ffi::c_void) -> u32);
windows_link::link!("eappcfg.dll" "system" fn EapHostPeerInvokeInteractiveUI(hwndparent : super::super::Foundation::HWND, dwsizeofuicontextdata : u32, puicontextdata : *const u8, pdwsizeofdatafrominteractiveui : *mut u32, ppdatafrominteractiveui : *mut *mut u8, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerProcessReceivedPacket(sessionhandle : u32, cbreceivepacket : u32, preceivepacket : *const u8, peapoutput : *mut EapHostPeerResponseAction, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappcfg.dll" "system" fn EapHostPeerQueryCredentialInputFields(huserimpersonationtoken : super::super::Foundation::HANDLE, eapmethodtype : EAP_METHOD_TYPE, dwflags : u32, dweapconndatasize : u32, pbeapconndata : *const u8, peapconfiginputfieldarray : *mut EAP_CONFIG_INPUT_FIELD_ARRAY, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappcfg.dll" "system" fn EapHostPeerQueryInteractiveUIInputFields(dwversion : u32, dwflags : u32, dwsizeofuicontextdata : u32, puicontextdata : *const u8, peapinteractiveuidata : *mut EAP_INTERACTIVE_UI_DATA, ppeaperror : *mut *mut EAP_ERROR, ppvreserved : *mut *mut core::ffi::c_void) -> u32);
windows_link::link!("eappcfg.dll" "system" fn EapHostPeerQueryUIBlobFromInteractiveUIInputFields(dwversion : u32, dwflags : u32, dwsizeofuicontextdata : u32, puicontextdata : *const u8, peapinteractiveuidata : *const EAP_INTERACTIVE_UI_DATA, pdwsizeofdatafrominteractiveui : *mut u32, ppdatafrominteractiveui : *mut *mut u8, ppeaperror : *mut *mut EAP_ERROR, ppvreserved : *mut *mut core::ffi::c_void) -> u32);
windows_link::link!("eappcfg.dll" "system" fn EapHostPeerQueryUserBlobFromCredentialInputFields(huserimpersonationtoken : super::super::Foundation::HANDLE, eapmethodtype : EAP_METHOD_TYPE, dwflags : u32, dweapconndatasize : u32, pbeapconndata : *const u8, peapconfiginputfieldarray : *const EAP_CONFIG_INPUT_FIELD_ARRAY, pdwuserblobsize : *mut u32, ppbuserblob : *mut *mut u8, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerSetResponseAttributes(sessionhandle : u32, pattribs : *const EAP_ATTRIBUTES, peapoutput : *mut EapHostPeerResponseAction, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerSetUIContext(sessionhandle : u32, dwsizeofuicontextdata : u32, puicontextdata : *const u8, peapoutput : *mut EapHostPeerResponseAction, ppeaperror : *mut *mut EAP_ERROR) -> u32);
windows_link::link!("eappprxy.dll" "system" fn EapHostPeerUninitialize());
pub const CERTIFICATE_HASH_LENGTH: u32 = 20;
pub const EAPACTION_Authenticate: PPP_EAP_ACTION = 1;
pub const EAPACTION_Done: PPP_EAP_ACTION = 2;
pub const EAPACTION_IndicateIdentity: PPP_EAP_ACTION = 8;
pub const EAPACTION_IndicateTLV: PPP_EAP_ACTION = 7;
pub const EAPACTION_NoAction: PPP_EAP_ACTION = 0;
pub const EAPACTION_Send: PPP_EAP_ACTION = 4;
pub const EAPACTION_SendAndDone: PPP_EAP_ACTION = 3;
pub const EAPACTION_SendWithTimeout: PPP_EAP_ACTION = 5;
pub const EAPACTION_SendWithTimeoutInteractive: PPP_EAP_ACTION = 6;
pub const EAPCODE_Failure: u32 = 4;
pub const EAPCODE_Request: u32 = 1;
pub const EAPCODE_Response: u32 = 2;
pub const EAPCODE_Success: u32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EAPHOST_AUTH_INFO {
    pub status: EAPHOST_AUTH_STATUS,
    pub dwErrorCode: u32,
    pub dwReasonCode: u32,
}
pub type EAPHOST_AUTH_STATUS = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAPHOST_IDENTITY_UI_PARAMS {
    pub eapMethodType: EAP_METHOD_TYPE,
    pub dwFlags: u32,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub dwSizeofUserDataOut: u32,
    pub pUserDataOut: *mut u8,
    pub pwszIdentity: windows_sys::core::PWSTR,
    pub dwError: u32,
    pub pEapError: *mut EAP_ERROR,
}
impl Default for EAPHOST_IDENTITY_UI_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAPHOST_INTERACTIVE_UI_PARAMS {
    pub dwSizeofContextData: u32,
    pub pContextData: *mut u8,
    pub dwSizeofInteractiveUIData: u32,
    pub pInteractiveUIData: *mut u8,
    pub dwError: u32,
    pub pEapError: *mut EAP_ERROR,
}
impl Default for EAPHOST_INTERACTIVE_UI_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EAPHOST_METHOD_API_VERSION: u32 = 1;
pub const EAPHOST_PEER_API_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_ATTRIBUTE {
    pub eaType: EAP_ATTRIBUTE_TYPE,
    pub dwLength: u32,
    pub pValue: *mut u8,
}
impl Default for EAP_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_ATTRIBUTES {
    pub dwNumberOfAttributes: u32,
    pub pAttribs: *mut EAP_ATTRIBUTE,
}
impl Default for EAP_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EAP_ATTRIBUTE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_AUTHENTICATOR_METHOD_ROUTINES {
    pub dwSizeInBytes: u32,
    pub pEapType: *mut EAP_METHOD_TYPE,
    pub EapMethodAuthenticatorInitialize: isize,
    pub EapMethodAuthenticatorBeginSession: isize,
    pub EapMethodAuthenticatorUpdateInnerMethodParams: isize,
    pub EapMethodAuthenticatorReceivePacket: isize,
    pub EapMethodAuthenticatorSendPacket: isize,
    pub EapMethodAuthenticatorGetAttributes: isize,
    pub EapMethodAuthenticatorSetAttributes: isize,
    pub EapMethodAuthenticatorGetResult: isize,
    pub EapMethodAuthenticatorEndSession: isize,
    pub EapMethodAuthenticatorShutdown: isize,
}
impl Default for EAP_AUTHENTICATOR_METHOD_ROUTINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EAP_AUTHENTICATOR_SEND_TIMEOUT = i32;
pub const EAP_AUTHENTICATOR_SEND_TIMEOUT_BASIC: EAP_AUTHENTICATOR_SEND_TIMEOUT = 1;
pub const EAP_AUTHENTICATOR_SEND_TIMEOUT_INTERACTIVE: EAP_AUTHENTICATOR_SEND_TIMEOUT = 2;
pub const EAP_AUTHENTICATOR_SEND_TIMEOUT_NONE: EAP_AUTHENTICATOR_SEND_TIMEOUT = 0;
pub const EAP_AUTHENTICATOR_VALUENAME_CONFIGUI: windows_sys::core::PCWSTR = windows_sys::core::w!("AuthenticatorConfigUIPath");
pub const EAP_AUTHENTICATOR_VALUENAME_DLL_PATH: windows_sys::core::PCWSTR = windows_sys::core::w!("AuthenticatorDllPath");
pub const EAP_AUTHENTICATOR_VALUENAME_FRIENDLY_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("AuthenticatorFriendlyName");
pub const EAP_AUTHENTICATOR_VALUENAME_PROPERTIES: windows_sys::core::PCWSTR = windows_sys::core::w!("Properties");
pub const EAP_CERTIFICATE_CREDENTIAL: EapCredentialType = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_CONFIG_INPUT_FIELD_ARRAY {
    pub dwVersion: u32,
    pub dwNumberOfFields: u32,
    pub pFields: *mut EAP_CONFIG_INPUT_FIELD_DATA,
}
impl Default for EAP_CONFIG_INPUT_FIELD_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_CONFIG_INPUT_FIELD_DATA {
    pub dwSize: u32,
    pub Type: EAP_CONFIG_INPUT_FIELD_TYPE,
    pub dwFlagProps: u32,
    pub pwszLabel: windows_sys::core::PWSTR,
    pub pwszData: windows_sys::core::PWSTR,
    pub dwMinDataLength: u32,
    pub dwMaxDataLength: u32,
}
impl Default for EAP_CONFIG_INPUT_FIELD_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EAP_CONFIG_INPUT_FIELD_PROPS_DEFAULT: u32 = 0;
pub const EAP_CONFIG_INPUT_FIELD_PROPS_NON_DISPLAYABLE: u32 = 1;
pub const EAP_CONFIG_INPUT_FIELD_PROPS_NON_PERSIST: u32 = 2;
pub type EAP_CONFIG_INPUT_FIELD_TYPE = i32;
pub const EAP_CREDENTIAL_VERSION: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EAP_CRED_EXPIRY_REQ {
    pub curCreds: EAP_CONFIG_INPUT_FIELD_ARRAY,
    pub newCreds: EAP_CONFIG_INPUT_FIELD_ARRAY,
}
pub const EAP_EMPTY_CREDENTIAL: EapCredentialType = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_ERROR {
    pub dwWinError: u32,
    pub r#type: EAP_METHOD_TYPE,
    pub dwReasonCode: u32,
    pub rootCauseGuid: windows_sys::core::GUID,
    pub repairGuid: windows_sys::core::GUID,
    pub helpLinkGuid: windows_sys::core::GUID,
    pub pRootCauseString: windows_sys::core::PWSTR,
    pub pRepairString: windows_sys::core::PWSTR,
}
impl Default for EAP_ERROR {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EAP_E_AUTHENTICATION_FAILED: u32 = 2151809045;
pub const EAP_E_CERT_STORE_INACCESSIBLE: u32 = 2151809040;
pub const EAP_E_EAPHOST_EAPQEC_INACCESSIBLE: u32 = 2151809043;
pub const EAP_E_EAPHOST_FIRST: i32 = -2143158272;
pub const EAP_E_EAPHOST_IDENTITY_UNKNOWN: u32 = 2151809044;
pub const EAP_E_EAPHOST_LAST: i32 = -2143158017;
pub const EAP_E_EAPHOST_METHOD_INVALID_PACKET: u32 = 2151809047;
pub const EAP_E_EAPHOST_METHOD_NOT_INSTALLED: u32 = 2151809041;
pub const EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED: u32 = 2151809056;
pub const EAP_E_EAPHOST_REMOTE_INVALID_PACKET: u32 = 2151809048;
pub const EAP_E_EAPHOST_THIRDPARTY_METHOD_HOST_RESET: u32 = 2151809042;
pub const EAP_E_EAPHOST_XML_MALFORMED: u32 = 2151809049;
pub const EAP_E_METHOD_CONFIG_DOES_NOT_SUPPORT_SSO: u32 = 2151809050;
pub const EAP_E_NO_SMART_CARD_READER: u32 = 2151809299;
pub const EAP_E_SERVER_CERT_EXPIRED: u32 = 2151809538;
pub const EAP_E_SERVER_CERT_INVALID: u32 = 2151809537;
pub const EAP_E_SERVER_CERT_NOT_FOUND: u32 = 2151809536;
pub const EAP_E_SERVER_CERT_OTHER_ERROR: u32 = 2151809540;
pub const EAP_E_SERVER_CERT_REVOKED: u32 = 2151809539;
pub const EAP_E_SERVER_FIRST: i32 = -2143157760;
pub const EAP_E_SERVER_LAST: i32 = -2143157505;
pub const EAP_E_SERVER_ROOT_CERT_FIRST: i32 = -2143157248;
pub const EAP_E_SERVER_ROOT_CERT_INVALID: u32 = 2151810049;
pub const EAP_E_SERVER_ROOT_CERT_LAST: i32 = -2143156993;
pub const EAP_E_SERVER_ROOT_CERT_NAME_REQUIRED: u32 = 2151810054;
pub const EAP_E_SERVER_ROOT_CERT_NOT_FOUND: u32 = 2151810048;
pub const EAP_E_SIM_NOT_VALID: u32 = 2151810304;
pub const EAP_E_USER_CERT_EXPIRED: u32 = 2151809282;
pub const EAP_E_USER_CERT_INVALID: u32 = 2151809281;
pub const EAP_E_USER_CERT_NOT_FOUND: u32 = 2151809280;
pub const EAP_E_USER_CERT_OTHER_ERROR: u32 = 2151809284;
pub const EAP_E_USER_CERT_REJECTED: u32 = 2151809285;
pub const EAP_E_USER_CERT_REVOKED: u32 = 2151809283;
pub const EAP_E_USER_CREDENTIALS_REJECTED: u32 = 2151809297;
pub const EAP_E_USER_FIRST: i32 = -2143158016;
pub const EAP_E_USER_LAST: i32 = -2143157761;
pub const EAP_E_USER_NAME_PASSWORD_REJECTED: u32 = 2151809298;
pub const EAP_E_USER_ROOT_CERT_EXPIRED: u32 = 2151809794;
pub const EAP_E_USER_ROOT_CERT_FIRST: i32 = -2143157504;
pub const EAP_E_USER_ROOT_CERT_INVALID: u32 = 2151809793;
pub const EAP_E_USER_ROOT_CERT_LAST: i32 = -2143157249;
pub const EAP_E_USER_ROOT_CERT_NOT_FOUND: u32 = 2151809792;
pub const EAP_FLAG_CONFG_READONLY: u32 = 524288;
pub const EAP_FLAG_FULL_AUTH: u32 = 4096;
pub const EAP_FLAG_GUEST_ACCESS: u32 = 64;
pub const EAP_FLAG_LOGON: u32 = 4;
pub const EAP_FLAG_MACHINE_AUTH: u32 = 32;
pub const EAP_FLAG_NON_INTERACTIVE: u32 = 2;
pub const EAP_FLAG_ONLY_EAP_TLS: u32 = 16777216;
pub const EAP_FLAG_PREFER_ALT_CREDENTIALS: u32 = 8192;
pub const EAP_FLAG_PREVIEW: u32 = 8;
pub const EAP_FLAG_PRE_LOGON: u32 = 131072;
pub const EAP_FLAG_RESUME_FROM_HIBERNATE: u32 = 512;
pub const EAP_FLAG_Reserved1: u32 = 1;
pub const EAP_FLAG_Reserved2: u32 = 16;
pub const EAP_FLAG_Reserved3: u32 = 128;
pub const EAP_FLAG_Reserved4: u32 = 256;
pub const EAP_FLAG_Reserved5: u32 = 1024;
pub const EAP_FLAG_Reserved6: u32 = 2048;
pub const EAP_FLAG_Reserved7: u32 = 16384;
pub const EAP_FLAG_Reserved8: u32 = 1048576;
pub const EAP_FLAG_Reserved9: u32 = 4194304;
pub const EAP_FLAG_SERVER_VALIDATION_REQUIRED: u32 = 33554432;
pub const EAP_FLAG_SUPRESS_UI: u32 = 65536;
pub const EAP_FLAG_USER_AUTH: u32 = 262144;
pub const EAP_FLAG_VPN: u32 = 8388608;
pub const EAP_GROUP_MASK: i32 = 65280;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EAP_INTERACTIVE_UI_DATA {
    pub dwVersion: u32,
    pub dwSize: u32,
    pub dwDataType: EAP_INTERACTIVE_UI_DATA_TYPE,
    pub cbUiData: u32,
    pub pbUiData: EAP_UI_DATA_FORMAT,
}
impl Default for EAP_INTERACTIVE_UI_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EAP_INTERACTIVE_UI_DATA_TYPE = i32;
pub const EAP_INTERACTIVE_UI_DATA_VERSION: u32 = 1;
pub const EAP_INVALID_PACKET: u32 = 2151809048;
pub const EAP_I_EAPHOST_EAP_NEGOTIATION_FAILED: u32 = 1078067222;
pub const EAP_I_EAPHOST_FIRST: i32 = -2143158272;
pub const EAP_I_EAPHOST_LAST: i32 = -2143158017;
pub const EAP_I_USER_ACCOUNT_OTHER_ERROR: u32 = 1078067472;
pub const EAP_I_USER_FIRST: i32 = 1078067456;
pub const EAP_I_USER_LAST: i32 = 1078067711;
pub const EAP_METHOD_AUTHENTICATOR_CONFIG_IS_IDENTITY_PRIVACY: u32 = 1;
pub type EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = i32;
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_AUTHENTICATE: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = 4;
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_DISCARD: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = 0;
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_HANDLE_IDENTITY: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = 5;
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_RESPOND: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = 3;
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_RESULT: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = 2;
pub const EAP_METHOD_AUTHENTICATOR_RESPONSE_SEND: EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_METHOD_AUTHENTICATOR_RESULT {
    pub fIsSuccess: windows_sys::core::BOOL,
    pub dwFailureReason: u32,
    pub pAuthAttribs: *mut EAP_ATTRIBUTES,
}
impl Default for EAP_METHOD_AUTHENTICATOR_RESULT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_METHOD_INFO {
    pub eaptype: EAP_METHOD_TYPE,
    pub pwszAuthorName: windows_sys::core::PWSTR,
    pub pwszFriendlyName: windows_sys::core::PWSTR,
    pub eapProperties: u32,
    pub pInnerMethodInfo: *mut EAP_METHOD_INFO,
}
impl Default for EAP_METHOD_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_METHOD_INFO_ARRAY {
    pub dwNumberOfMethods: u32,
    pub pEapMethods: *mut EAP_METHOD_INFO,
}
impl Default for EAP_METHOD_INFO_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_METHOD_INFO_ARRAY_EX {
    pub dwNumberOfMethods: u32,
    pub pEapMethods: *mut EAP_METHOD_INFO_EX,
}
impl Default for EAP_METHOD_INFO_ARRAY_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_METHOD_INFO_EX {
    pub eaptype: EAP_METHOD_TYPE,
    pub pwszAuthorName: windows_sys::core::PWSTR,
    pub pwszFriendlyName: windows_sys::core::PWSTR,
    pub eapProperties: u32,
    pub pInnerMethodInfoArray: *mut EAP_METHOD_INFO_ARRAY_EX,
}
impl Default for EAP_METHOD_INFO_EX {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EAP_METHOD_INVALID_PACKET: u32 = 2151809047;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EAP_METHOD_PROPERTY {
    pub eapMethodPropertyType: EAP_METHOD_PROPERTY_TYPE,
    pub eapMethodPropertyValueType: EAP_METHOD_PROPERTY_VALUE_TYPE,
    pub eapMethodPropertyValue: EAP_METHOD_PROPERTY_VALUE,
}
impl Default for EAP_METHOD_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_METHOD_PROPERTY_ARRAY {
    pub dwNumberOfProperties: u32,
    pub pMethodProperty: *mut EAP_METHOD_PROPERTY,
}
impl Default for EAP_METHOD_PROPERTY_ARRAY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EAP_METHOD_PROPERTY_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union EAP_METHOD_PROPERTY_VALUE {
    pub empvBool: EAP_METHOD_PROPERTY_VALUE_BOOL,
    pub empvDword: EAP_METHOD_PROPERTY_VALUE_DWORD,
    pub empvString: EAP_METHOD_PROPERTY_VALUE_STRING,
}
impl Default for EAP_METHOD_PROPERTY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EAP_METHOD_PROPERTY_VALUE_BOOL {
    pub length: u32,
    pub value: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EAP_METHOD_PROPERTY_VALUE_DWORD {
    pub length: u32,
    pub value: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_METHOD_PROPERTY_VALUE_STRING {
    pub length: u32,
    pub value: *mut u8,
}
impl Default for EAP_METHOD_PROPERTY_VALUE_STRING {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EAP_METHOD_PROPERTY_VALUE_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EAP_METHOD_TYPE {
    pub eapType: EAP_TYPE,
    pub dwAuthorId: u32,
}
pub const EAP_PEER_FLAG_GUEST_ACCESS: u32 = 64;
pub const EAP_PEER_FLAG_HEALTH_STATE_CHANGE: u32 = 32768;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EAP_PEER_METHOD_ROUTINES {
    pub dwVersion: u32,
    pub pEapType: *mut EAP_TYPE,
    pub EapPeerInitialize: isize,
    pub EapPeerGetIdentity: isize,
    pub EapPeerBeginSession: isize,
    pub EapPeerSetCredentials: isize,
    pub EapPeerProcessRequestPacket: isize,
    pub EapPeerGetResponsePacket: isize,
    pub EapPeerGetResult: isize,
    pub EapPeerGetUIContext: isize,
    pub EapPeerSetUIContext: isize,
    pub EapPeerGetResponseAttributes: isize,
    pub EapPeerSetResponseAttributes: isize,
    pub EapPeerEndSession: isize,
    pub EapPeerShutdown: isize,
}
impl Default for EAP_PEER_METHOD_ROUTINES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EAP_PEER_VALUENAME_CONFIGUI: windows_sys::core::PCWSTR = windows_sys::core::w!("PeerConfigUIPath");
pub const EAP_PEER_VALUENAME_DLL_PATH: windows_sys::core::PCWSTR = windows_sys::core::w!("PeerDllPath");
pub const EAP_PEER_VALUENAME_FRIENDLY_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("PeerFriendlyName");
pub const EAP_PEER_VALUENAME_IDENTITY: windows_sys::core::PCWSTR = windows_sys::core::w!("PeerIdentityPath");
pub const EAP_PEER_VALUENAME_INTERACTIVEUI: windows_sys::core::PCWSTR = windows_sys::core::w!("PeerInteractiveUIPath");
pub const EAP_PEER_VALUENAME_INVOKE_NAMEDLG: windows_sys::core::PCWSTR = windows_sys::core::w!("PeerInvokeUsernameDialog");
pub const EAP_PEER_VALUENAME_INVOKE_PWDDLG: windows_sys::core::PCWSTR = windows_sys::core::w!("PeerInvokePasswordDialog");
pub const EAP_PEER_VALUENAME_PROPERTIES: windows_sys::core::PCWSTR = windows_sys::core::w!("Properties");
pub const EAP_PEER_VALUENAME_REQUIRE_CONFIGUI: windows_sys::core::PCWSTR = windows_sys::core::w!("PeerRequireConfigUI");
pub const EAP_REGISTRY_LOCATION: windows_sys::core::PCWSTR = windows_sys::core::w!("System\\CurrentControlSet\\Services\\EapHost\\Methods");
pub const EAP_SIM_CREDENTIAL: EapCredentialType = 4;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EAP_TYPE {
    pub r#type: u8,
    pub dwVendorId: u32,
    pub dwVendorType: u32,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub union EAP_UI_DATA_FORMAT {
    pub credData: *mut EAP_CONFIG_INPUT_FIELD_ARRAY,
    pub credExpiryData: *mut EAP_CRED_EXPIRY_REQ,
    pub credLogonData: *mut EAP_CONFIG_INPUT_FIELD_ARRAY,
}
impl Default for EAP_UI_DATA_FORMAT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EAP_UI_INPUT_FIELD_PROPS_DEFAULT: u32 = 0;
pub const EAP_UI_INPUT_FIELD_PROPS_NON_DISPLAYABLE: u32 = 1;
pub const EAP_UI_INPUT_FIELD_PROPS_NON_PERSIST: u32 = 2;
pub const EAP_UI_INPUT_FIELD_PROPS_READ_ONLY: u32 = 4;
pub const EAP_USERNAME_PASSWORD_CREDENTIAL: EapCredentialType = 1;
pub const EAP_VALUENAME_PROPERTIES: windows_sys::core::PCWSTR = windows_sys::core::w!("Properties");
pub const EAP_WINLOGON_CREDENTIAL: EapCredentialType = 2;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EapCertificateCredential {
    pub certHash: [u8; 20],
    pub password: windows_sys::core::PWSTR,
}
impl Default for EapCertificateCredential {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EapCode = i32;
pub const EapCodeFailure: EapCode = 4;
pub const EapCodeMaximum: EapCode = 4;
pub const EapCodeMinimum: EapCode = 1;
pub const EapCodeRequest: EapCode = 1;
pub const EapCodeResponse: EapCode = 2;
pub const EapCodeSuccess: EapCode = 3;
pub const EapConfigInputEdit: EAP_CONFIG_INPUT_FIELD_TYPE = 6;
pub const EapConfigInputNetworkPassword: EAP_CONFIG_INPUT_FIELD_TYPE = 3;
pub const EapConfigInputNetworkUsername: EAP_CONFIG_INPUT_FIELD_TYPE = 2;
pub const EapConfigInputPSK: EAP_CONFIG_INPUT_FIELD_TYPE = 5;
pub const EapConfigInputPassword: EAP_CONFIG_INPUT_FIELD_TYPE = 1;
pub const EapConfigInputPin: EAP_CONFIG_INPUT_FIELD_TYPE = 4;
pub const EapConfigInputUsername: EAP_CONFIG_INPUT_FIELD_TYPE = 0;
pub const EapConfigSmartCardError: EAP_CONFIG_INPUT_FIELD_TYPE = 8;
pub const EapConfigSmartCardUsername: EAP_CONFIG_INPUT_FIELD_TYPE = 7;
pub const EapCredExpiryReq: EAP_INTERACTIVE_UI_DATA_TYPE = 2;
pub const EapCredExpiryResp: EAP_INTERACTIVE_UI_DATA_TYPE = 3;
pub const EapCredLogonReq: EAP_INTERACTIVE_UI_DATA_TYPE = 4;
pub const EapCredLogonResp: EAP_INTERACTIVE_UI_DATA_TYPE = 5;
pub const EapCredReq: EAP_INTERACTIVE_UI_DATA_TYPE = 0;
pub const EapCredResp: EAP_INTERACTIVE_UI_DATA_TYPE = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct EapCredential {
    pub credType: EapCredentialType,
    pub credData: EapCredentialTypeData,
}
impl Default for EapCredential {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type EapCredentialType = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub union EapCredentialTypeData {
    pub username_password: EapUsernamePasswordCredential,
    pub certificate: EapCertificateCredential,
    pub sim: EapSimCredential,
}
impl Default for EapCredentialTypeData {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EapHostAuthFailed: EAPHOST_AUTH_STATUS = 6;
pub const EapHostAuthIdentityExchange: EAPHOST_AUTH_STATUS = 2;
pub const EapHostAuthInProgress: EAPHOST_AUTH_STATUS = 4;
pub const EapHostAuthNegotiatingType: EAPHOST_AUTH_STATUS = 3;
pub const EapHostAuthNotStarted: EAPHOST_AUTH_STATUS = 1;
pub const EapHostAuthSucceeded: EAPHOST_AUTH_STATUS = 5;
pub const EapHostInvalidSession: EAPHOST_AUTH_STATUS = 0;
pub const EapHostNapInfo: EapHostPeerAuthParams = 4;
pub type EapHostPeerAuthParams = i32;
pub const EapHostPeerAuthStatus: EapHostPeerAuthParams = 1;
pub const EapHostPeerIdentity: EapHostPeerAuthParams = 2;
pub const EapHostPeerIdentityExtendedInfo: EapHostPeerAuthParams = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EapHostPeerMethodResult {
    pub fIsSuccess: windows_sys::core::BOOL,
    pub dwFailureReasonCode: u32,
    pub fSaveConnectionData: windows_sys::core::BOOL,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub fSaveUserData: windows_sys::core::BOOL,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub pAttribArray: *mut EAP_ATTRIBUTES,
    pub isolationState: ISOLATION_STATE,
    pub pEapMethodInfo: *mut EAP_METHOD_INFO,
    pub pEapError: *mut EAP_ERROR,
}
impl Default for EapHostPeerMethodResult {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EapHostPeerMethodResultAltSuccessReceived: EapHostPeerMethodResultReason = 1;
pub const EapHostPeerMethodResultFromMethod: EapHostPeerMethodResultReason = 3;
pub type EapHostPeerMethodResultReason = i32;
pub const EapHostPeerMethodResultTimeout: EapHostPeerMethodResultReason = 2;
pub type EapHostPeerResponseAction = i32;
pub const EapHostPeerResponseDiscard: EapHostPeerResponseAction = 0;
pub const EapHostPeerResponseInvokeUi: EapHostPeerResponseAction = 3;
pub const EapHostPeerResponseNone: EapHostPeerResponseAction = 6;
pub const EapHostPeerResponseRespond: EapHostPeerResponseAction = 4;
pub const EapHostPeerResponseResult: EapHostPeerResponseAction = 2;
pub const EapHostPeerResponseSend: EapHostPeerResponseAction = 1;
pub const EapHostPeerResponseStartAuthentication: EapHostPeerResponseAction = 5;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EapPacket {
    pub Code: u8,
    pub Id: u8,
    pub Length: [u8; 2],
    pub Data: [u8; 1],
}
impl Default for EapPacket {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct EapPeerMethodOutput {
    pub action: EapPeerMethodResponseAction,
    pub fAllowNotifications: windows_sys::core::BOOL,
}
pub type EapPeerMethodResponseAction = i32;
pub const EapPeerMethodResponseActionDiscard: EapPeerMethodResponseAction = 0;
pub const EapPeerMethodResponseActionInvokeUI: EapPeerMethodResponseAction = 3;
pub const EapPeerMethodResponseActionNone: EapPeerMethodResponseAction = 5;
pub const EapPeerMethodResponseActionRespond: EapPeerMethodResponseAction = 4;
pub const EapPeerMethodResponseActionResult: EapPeerMethodResponseAction = 2;
pub const EapPeerMethodResponseActionSend: EapPeerMethodResponseAction = 1;
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EapPeerMethodResult {
    pub fIsSuccess: windows_sys::core::BOOL,
    pub dwFailureReasonCode: u32,
    pub fSaveConnectionData: windows_sys::core::BOOL,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub fSaveUserData: windows_sys::core::BOOL,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub pAttribArray: *mut EAP_ATTRIBUTES,
    pub pEapError: *mut EAP_ERROR,
    pub pNgcKerbTicket: *mut NgcTicketContext,
    pub fSaveToCredMan: windows_sys::core::BOOL,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for EapPeerMethodResult {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EapPeerMethodResultFailure: EapPeerMethodResultReason = 3;
pub type EapPeerMethodResultReason = i32;
pub const EapPeerMethodResultSuccess: EapPeerMethodResultReason = 2;
pub const EapPeerMethodResultUnknown: EapPeerMethodResultReason = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EapSimCredential {
    pub iccID: windows_sys::core::PWSTR,
}
impl Default for EapSimCredential {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EapUsernamePasswordCredential {
    pub username: windows_sys::core::PWSTR,
    pub password: windows_sys::core::PWSTR,
}
impl Default for EapUsernamePasswordCredential {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const FACILITY_EAP_MESSAGE: u32 = 2114;
pub const GUID_EapHost_Cause_CertStoreInaccessible: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000004);
pub const GUID_EapHost_Cause_EapNegotiationFailed: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000001c);
pub const GUID_EapHost_Cause_EapQecInaccessible: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000312);
pub const GUID_EapHost_Cause_Generic_AuthFailure: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000104);
pub const GUID_EapHost_Cause_IdentityUnknown: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000204);
pub const GUID_EapHost_Cause_MethodDLLNotFound: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000001);
pub const GUID_EapHost_Cause_MethodDoesNotSupportOperation: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000001e);
pub const GUID_EapHost_Cause_Method_Config_Does_Not_Support_Sso: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xda18bd32_004f_41fa_ae08_0bc85e5845ac);
pub const GUID_EapHost_Cause_No_SmartCardReader_Found: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000002b);
pub const GUID_EapHost_Cause_Server_CertExpired: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000005);
pub const GUID_EapHost_Cause_Server_CertInvalid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000006);
pub const GUID_EapHost_Cause_Server_CertNotFound: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000007);
pub const GUID_EapHost_Cause_Server_CertOtherError: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000108);
pub const GUID_EapHost_Cause_Server_CertRevoked: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000008);
pub const GUID_EapHost_Cause_Server_Root_CertNameRequired: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000012);
pub const GUID_EapHost_Cause_Server_Root_CertNotFound: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000112);
pub const GUID_EapHost_Cause_SimNotValid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000304);
pub const GUID_EapHost_Cause_ThirdPartyMethod_Host_Reset: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000212);
pub const GUID_EapHost_Cause_User_Account_OtherProblem: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000010e);
pub const GUID_EapHost_Cause_User_CertExpired: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000009);
pub const GUID_EapHost_Cause_User_CertInvalid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000000a);
pub const GUID_EapHost_Cause_User_CertNotFound: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000000b);
pub const GUID_EapHost_Cause_User_CertOtherError: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000000c);
pub const GUID_EapHost_Cause_User_CertRejected: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000000d);
pub const GUID_EapHost_Cause_User_CertRevoked: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000000e);
pub const GUID_EapHost_Cause_User_CredsRejected: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000020e);
pub const GUID_EapHost_Cause_User_Root_CertExpired: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000000f);
pub const GUID_EapHost_Cause_User_Root_CertInvalid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000010);
pub const GUID_EapHost_Cause_User_Root_CertNotFound: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000011);
pub const GUID_EapHost_Cause_XmlMalformed: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000001d);
pub const GUID_EapHost_Default: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x00000000_0000_0000_0000_000000000000);
pub const GUID_EapHost_Help_ObtainingCerts: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xf535eea3_1bdd_46ca_a2fc_a6655939b7e8);
pub const GUID_EapHost_Help_Troubleshooting: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x33307acf_0698_41ba_b014_ea0a2eb8d0a8);
pub const GUID_EapHost_Repair_ContactAdmin_AuthFailure: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000001f);
pub const GUID_EapHost_Repair_ContactAdmin_CertNameAbsent: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000029);
pub const GUID_EapHost_Repair_ContactAdmin_CertStoreInaccessible: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000024);
pub const GUID_EapHost_Repair_ContactAdmin_IdentityUnknown: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000020);
pub const GUID_EapHost_Repair_ContactAdmin_InvalidUserAccount: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000025);
pub const GUID_EapHost_Repair_ContactAdmin_InvalidUserCert: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000002c);
pub const GUID_EapHost_Repair_ContactAdmin_MethodNotFound: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000022);
pub const GUID_EapHost_Repair_ContactAdmin_NegotiationFailed: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000021);
pub const GUID_EapHost_Repair_ContactAdmin_NoSmartCardReader: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000002a);
pub const GUID_EapHost_Repair_ContactAdmin_RootCertInvalid: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000026);
pub const GUID_EapHost_Repair_ContactAdmin_RootCertNotFound: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000027);
pub const GUID_EapHost_Repair_ContactAdmin_RootExpired: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000028);
pub const GUID_EapHost_Repair_ContactSysadmin: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000002);
pub const GUID_EapHost_Repair_Method_Not_Support_Sso: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000002d);
pub const GUID_EapHost_Repair_No_ValidSim_Found: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000002e);
pub const GUID_EapHost_Repair_RestartNap: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000023);
pub const GUID_EapHost_Repair_Retry_Authentication: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000011b);
pub const GUID_EapHost_Repair_Server_ClientSelectServerCert: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000018);
pub const GUID_EapHost_Repair_User_AuthFailure: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d800000019);
pub const GUID_EapHost_Repair_User_GetNewCert: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000001a);
pub const GUID_EapHost_Repair_User_SelectValidCert: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9612fc67_6150_4209_a85e_a8d80000001b);
pub type ISOLATION_STATE = i32;
pub const ISOLATION_STATE_IN_PROBATION: ISOLATION_STATE = 2;
pub const ISOLATION_STATE_NOT_RESTRICTED: ISOLATION_STATE = 1;
pub const ISOLATION_STATE_RESTRICTED_ACCESS: ISOLATION_STATE = 3;
pub const ISOLATION_STATE_UNKNOWN: ISOLATION_STATE = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LEGACY_IDENTITY_UI_PARAMS {
    pub eapType: u32,
    pub dwFlags: u32,
    pub dwSizeofConnectionData: u32,
    pub pConnectionData: *mut u8,
    pub dwSizeofUserData: u32,
    pub pUserData: *mut u8,
    pub dwSizeofUserDataOut: u32,
    pub pUserDataOut: *mut u8,
    pub pwszIdentity: windows_sys::core::PWSTR,
    pub dwError: u32,
}
impl Default for LEGACY_IDENTITY_UI_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LEGACY_INTERACTIVE_UI_PARAMS {
    pub eapType: u32,
    pub dwSizeofContextData: u32,
    pub pContextData: *mut u8,
    pub dwSizeofInteractiveUIData: u32,
    pub pInteractiveUIData: *mut u8,
    pub dwError: u32,
}
impl Default for LEGACY_INTERACTIVE_UI_PARAMS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const MAXEAPCODE: u32 = 4;
pub const MAX_EAP_CONFIG_INPUT_FIELD_LENGTH: u32 = 256;
pub const MAX_EAP_CONFIG_INPUT_FIELD_VALUE_LENGTH: u32 = 1024;
pub const NCRYPT_PIN_CACHE_PIN_BYTE_LENGTH: u32 = 90;
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NgcTicketContext {
    pub wszTicket: [u16; 45],
    pub hKey: super::Cryptography::NCRYPT_KEY_HANDLE,
    pub hImpersonateToken: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for NgcTicketContext {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NotificationHandler = Option<unsafe extern "system" fn(connectionid: windows_sys::core::GUID, pcontextdata: *const core::ffi::c_void)>;
pub type PPP_EAP_ACTION = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PPP_EAP_INFO {
    pub dwSizeInBytes: u32,
    pub dwEapTypeId: u32,
    pub RasEapInitialize: isize,
    pub RasEapBegin: isize,
    pub RasEapEnd: isize,
    pub RasEapMakeMessage: isize,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_EAP_INPUT {
    pub dwSizeInBytes: u32,
    pub fFlags: u32,
    pub fAuthenticator: windows_sys::core::BOOL,
    pub pwszIdentity: windows_sys::core::PWSTR,
    pub pwszPassword: windows_sys::core::PWSTR,
    pub bInitialId: u8,
    pub pUserAttributes: *mut RAS_AUTH_ATTRIBUTE,
    pub fAuthenticationComplete: windows_sys::core::BOOL,
    pub dwAuthResultCode: u32,
    pub hTokenImpersonateUser: super::super::Foundation::HANDLE,
    pub fSuccessPacketReceived: windows_sys::core::BOOL,
    pub fDataReceivedFromInteractiveUI: windows_sys::core::BOOL,
    pub pDataFromInteractiveUI: *mut u8,
    pub dwSizeOfDataFromInteractiveUI: u32,
    pub pConnectionData: *mut u8,
    pub dwSizeOfConnectionData: u32,
    pub pUserData: *mut u8,
    pub dwSizeOfUserData: u32,
    pub hReserved: super::super::Foundation::HANDLE,
    pub guidConnectionId: windows_sys::core::GUID,
    pub isVpn: windows_sys::core::BOOL,
}
impl Default for PPP_EAP_INPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security_Cryptography")]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_EAP_OUTPUT {
    pub dwSizeInBytes: u32,
    pub Action: PPP_EAP_ACTION,
    pub dwAuthResultCode: u32,
    pub pUserAttributes: *mut RAS_AUTH_ATTRIBUTE,
    pub fInvokeInteractiveUI: windows_sys::core::BOOL,
    pub pUIContextData: *mut u8,
    pub dwSizeOfUIContextData: u32,
    pub fSaveConnectionData: windows_sys::core::BOOL,
    pub pConnectionData: *mut u8,
    pub dwSizeOfConnectionData: u32,
    pub fSaveUserData: windows_sys::core::BOOL,
    pub pUserData: *mut u8,
    pub dwSizeOfUserData: u32,
    pub pNgcKerbTicket: *mut NgcTicketContext,
    pub fSaveToCredMan: windows_sys::core::BOOL,
}
#[cfg(feature = "Win32_Security_Cryptography")]
impl Default for PPP_EAP_OUTPUT {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PPP_EAP_PACKET {
    pub Code: u8,
    pub Id: u8,
    pub Length: [u8; 2],
    pub Data: [u8; 1],
}
impl Default for PPP_EAP_PACKET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RAS_AUTH_ATTRIBUTE {
    pub raaType: RAS_AUTH_ATTRIBUTE_TYPE,
    pub dwLength: u32,
    pub Value: *mut core::ffi::c_void,
}
impl Default for RAS_AUTH_ATTRIBUTE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RAS_AUTH_ATTRIBUTE_TYPE = i32;
pub const RAS_EAP_FLAG_8021X_AUTH: u32 = 128;
pub const RAS_EAP_FLAG_ALTERNATIVE_USER_DB: u32 = 2048;
pub const RAS_EAP_FLAG_CONFG_READONLY: u32 = 524288;
pub const RAS_EAP_FLAG_FIRST_LINK: u32 = 16;
pub const RAS_EAP_FLAG_GUEST_ACCESS: u32 = 64;
pub const RAS_EAP_FLAG_HOSTED_IN_PEAP: u32 = 256;
pub const RAS_EAP_FLAG_LOGON: u32 = 4;
pub const RAS_EAP_FLAG_MACHINE_AUTH: u32 = 32;
pub const RAS_EAP_FLAG_NON_INTERACTIVE: u32 = 2;
pub const RAS_EAP_FLAG_PEAP_FORCE_FULL_AUTH: u32 = 4096;
pub const RAS_EAP_FLAG_PEAP_UPFRONT: u32 = 1024;
pub const RAS_EAP_FLAG_PREVIEW: u32 = 8;
pub const RAS_EAP_FLAG_PRE_LOGON: u32 = 131072;
pub const RAS_EAP_FLAG_RESERVED: u32 = 1048576;
pub const RAS_EAP_FLAG_RESUME_FROM_HIBERNATE: u32 = 512;
pub const RAS_EAP_FLAG_ROUTER: u32 = 1;
pub const RAS_EAP_FLAG_SAVE_CREDMAN: u32 = 2097152;
pub const RAS_EAP_FLAG_SERVER_VALIDATION_REQUIRED: u32 = 33554432;
pub const RAS_EAP_REGISTRY_LOCATION: windows_sys::core::PCWSTR = windows_sys::core::w!("System\\CurrentControlSet\\Services\\Rasman\\PPP\\EAP");
pub const RAS_EAP_ROLE_AUTHENTICATEE: u32 = 2;
pub const RAS_EAP_ROLE_AUTHENTICATOR: u32 = 1;
pub const RAS_EAP_ROLE_EXCLUDE_IN_EAP: u32 = 4;
pub const RAS_EAP_ROLE_EXCLUDE_IN_PEAP: u32 = 8;
pub const RAS_EAP_ROLE_EXCLUDE_IN_VPN: u32 = 16;
pub const RAS_EAP_VALUENAME_CONFIGUI: windows_sys::core::PCWSTR = windows_sys::core::w!("ConfigUIPath");
pub const RAS_EAP_VALUENAME_CONFIG_CLSID: windows_sys::core::PCWSTR = windows_sys::core::w!("ConfigCLSID");
pub const RAS_EAP_VALUENAME_DEFAULT_DATA: windows_sys::core::PCWSTR = windows_sys::core::w!("ConfigData");
pub const RAS_EAP_VALUENAME_ENCRYPTION: windows_sys::core::PCWSTR = windows_sys::core::w!("MPPEEncryptionSupported");
pub const RAS_EAP_VALUENAME_FILTER_INNERMETHODS: windows_sys::core::PCWSTR = windows_sys::core::w!("FilterInnerMethods");
pub const RAS_EAP_VALUENAME_FRIENDLY_NAME: windows_sys::core::PCWSTR = windows_sys::core::w!("FriendlyName");
pub const RAS_EAP_VALUENAME_IDENTITY: windows_sys::core::PCWSTR = windows_sys::core::w!("IdentityPath");
pub const RAS_EAP_VALUENAME_INTERACTIVEUI: windows_sys::core::PCWSTR = windows_sys::core::w!("InteractiveUIPath");
pub const RAS_EAP_VALUENAME_INVOKE_NAMEDLG: windows_sys::core::PCWSTR = windows_sys::core::w!("InvokeUsernameDialog");
pub const RAS_EAP_VALUENAME_INVOKE_PWDDLG: windows_sys::core::PCWSTR = windows_sys::core::w!("InvokePasswordDialog");
pub const RAS_EAP_VALUENAME_ISTUNNEL_METHOD: windows_sys::core::PCWSTR = windows_sys::core::w!("IsTunnelMethod");
pub const RAS_EAP_VALUENAME_PATH: windows_sys::core::PCWSTR = windows_sys::core::w!("Path");
pub const RAS_EAP_VALUENAME_PER_POLICY_CONFIG: windows_sys::core::PCWSTR = windows_sys::core::w!("PerPolicyConfig");
pub const RAS_EAP_VALUENAME_REQUIRE_CONFIGUI: windows_sys::core::PCWSTR = windows_sys::core::w!("RequireConfigUI");
pub const RAS_EAP_VALUENAME_ROLES_SUPPORTED: windows_sys::core::PCWSTR = windows_sys::core::w!("RolesSupported");
pub const RAS_EAP_VALUENAME_STANDALONE_SUPPORTED: windows_sys::core::PCWSTR = windows_sys::core::w!("StandaloneSupported");
pub const eapPropCertifiedMethod: u32 = 4194304;
pub const eapPropChannelBinding: u32 = 65536;
pub const eapPropCipherSuiteNegotiation: u32 = 1;
pub const eapPropConfidentiality: u32 = 16;
pub const eapPropCryptoBinding: u32 = 8192;
pub const eapPropDictionaryAttackResistance: u32 = 2048;
pub const eapPropFastReconnect: u32 = 4096;
pub const eapPropFragmentation: u32 = 32768;
pub const eapPropHiddenMethod: u32 = 8388608;
pub const eapPropIdentityPrivacy: u32 = 67108864;
pub const eapPropIntegrity: u32 = 4;
pub const eapPropKeyDerivation: u32 = 32;
pub const eapPropKeyStrength1024: u32 = 1024;
pub const eapPropKeyStrength128: u32 = 128;
pub const eapPropKeyStrength256: u32 = 256;
pub const eapPropKeyStrength512: u32 = 512;
pub const eapPropKeyStrength64: u32 = 64;
pub const eapPropMachineAuth: u32 = 16777216;
pub const eapPropMethodChaining: u32 = 134217728;
pub const eapPropMppeEncryption: u32 = 524288;
pub const eapPropMutualAuth: u32 = 2;
pub const eapPropNap: u32 = 131072;
pub const eapPropReplayProtection: u32 = 8;
pub const eapPropReserved: u32 = 2147483648;
pub const eapPropSessionIndependence: u32 = 16384;
pub const eapPropSharedStateEquivalence: u32 = 268435456;
pub const eapPropStandalone: u32 = 262144;
pub const eapPropSupportsConfig: u32 = 2097152;
pub const eapPropTunnelMethod: u32 = 1048576;
pub const eapPropUserAuth: u32 = 33554432;
pub const eatARAPChallengeResponse: EAP_ATTRIBUTE_TYPE = 84;
pub const eatARAPFeatures: EAP_ATTRIBUTE_TYPE = 71;
pub const eatARAPGuestLogon: EAP_ATTRIBUTE_TYPE = 8096;
pub const eatARAPPassword: EAP_ATTRIBUTE_TYPE = 70;
pub const eatARAPSecurity: EAP_ATTRIBUTE_TYPE = 73;
pub const eatARAPSecurityData: EAP_ATTRIBUTE_TYPE = 74;
pub const eatARAPZoneAccess: EAP_ATTRIBUTE_TYPE = 72;
pub const eatAcctAuthentic: EAP_ATTRIBUTE_TYPE = 45;
pub const eatAcctDelayTime: EAP_ATTRIBUTE_TYPE = 41;
pub const eatAcctEventTimeStamp: EAP_ATTRIBUTE_TYPE = 55;
pub const eatAcctInputOctets: EAP_ATTRIBUTE_TYPE = 42;
pub const eatAcctInputPackets: EAP_ATTRIBUTE_TYPE = 47;
pub const eatAcctInterimInterval: EAP_ATTRIBUTE_TYPE = 85;
pub const eatAcctLinkCount: EAP_ATTRIBUTE_TYPE = 51;
pub const eatAcctMultiSessionId: EAP_ATTRIBUTE_TYPE = 50;
pub const eatAcctOutputOctets: EAP_ATTRIBUTE_TYPE = 43;
pub const eatAcctOutputPackets: EAP_ATTRIBUTE_TYPE = 48;
pub const eatAcctSessionId: EAP_ATTRIBUTE_TYPE = 44;
pub const eatAcctSessionTime: EAP_ATTRIBUTE_TYPE = 46;
pub const eatAcctStatusType: EAP_ATTRIBUTE_TYPE = 40;
pub const eatAcctTerminateCause: EAP_ATTRIBUTE_TYPE = 49;
pub const eatCallbackId: EAP_ATTRIBUTE_TYPE = 20;
pub const eatCallbackNumber: EAP_ATTRIBUTE_TYPE = 19;
pub const eatCalledStationId: EAP_ATTRIBUTE_TYPE = 30;
pub const eatCallingStationId: EAP_ATTRIBUTE_TYPE = 31;
pub const eatCertificateOID: EAP_ATTRIBUTE_TYPE = 8097;
pub const eatCertificateThumbprint: EAP_ATTRIBUTE_TYPE = 8250;
pub const eatClass: EAP_ATTRIBUTE_TYPE = 25;
pub const eatClearTextPassword: EAP_ATTRIBUTE_TYPE = 8107;
pub const eatConfigurationToken: EAP_ATTRIBUTE_TYPE = 78;
pub const eatConnectInfo: EAP_ATTRIBUTE_TYPE = 77;
pub const eatCredentialsChanged: EAP_ATTRIBUTE_TYPE = 8103;
pub const eatEAPConfiguration: EAP_ATTRIBUTE_TYPE = 8098;
pub const eatEAPMessage: EAP_ATTRIBUTE_TYPE = 79;
pub const eatEAPTLV: EAP_ATTRIBUTE_TYPE = 8102;
pub const eatEMSK: EAP_ATTRIBUTE_TYPE = 9003;
pub const eatFastRoamedSession: EAP_ATTRIBUTE_TYPE = 8100;
pub const eatFilterId: EAP_ATTRIBUTE_TYPE = 11;
pub const eatFramedAppleTalkLink: EAP_ATTRIBUTE_TYPE = 37;
pub const eatFramedAppleTalkNetwork: EAP_ATTRIBUTE_TYPE = 38;
pub const eatFramedAppleTalkZone: EAP_ATTRIBUTE_TYPE = 39;
pub const eatFramedCompression: EAP_ATTRIBUTE_TYPE = 13;
pub const eatFramedIPAddress: EAP_ATTRIBUTE_TYPE = 8;
pub const eatFramedIPNetmask: EAP_ATTRIBUTE_TYPE = 9;
pub const eatFramedIPXNetwork: EAP_ATTRIBUTE_TYPE = 23;
pub const eatFramedIPv6Pool: EAP_ATTRIBUTE_TYPE = 100;
pub const eatFramedIPv6Prefix: EAP_ATTRIBUTE_TYPE = 97;
pub const eatFramedIPv6Route: EAP_ATTRIBUTE_TYPE = 99;
pub const eatFramedInterfaceId: EAP_ATTRIBUTE_TYPE = 96;
pub const eatFramedMTU: EAP_ATTRIBUTE_TYPE = 12;
pub const eatFramedProtocol: EAP_ATTRIBUTE_TYPE = 7;
pub const eatFramedRoute: EAP_ATTRIBUTE_TYPE = 22;
pub const eatFramedRouting: EAP_ATTRIBUTE_TYPE = 10;
pub const eatIdleTimeout: EAP_ATTRIBUTE_TYPE = 28;
pub const eatInnerEapMethodType: EAP_ATTRIBUTE_TYPE = 8104;
pub const eatLoginIPHost: EAP_ATTRIBUTE_TYPE = 14;
pub const eatLoginIPv6Host: EAP_ATTRIBUTE_TYPE = 98;
pub const eatLoginLATGroup: EAP_ATTRIBUTE_TYPE = 36;
pub const eatLoginLATNode: EAP_ATTRIBUTE_TYPE = 35;
pub const eatLoginLATPort: EAP_ATTRIBUTE_TYPE = 63;
pub const eatLoginLATService: EAP_ATTRIBUTE_TYPE = 34;
pub const eatLoginService: EAP_ATTRIBUTE_TYPE = 15;
pub const eatLoginTCPPort: EAP_ATTRIBUTE_TYPE = 16;
pub const eatMD5CHAPChallenge: EAP_ATTRIBUTE_TYPE = 60;
pub const eatMD5CHAPPassword: EAP_ATTRIBUTE_TYPE = 3;
pub const eatMethodId: EAP_ATTRIBUTE_TYPE = 9002;
pub const eatMinimum: EAP_ATTRIBUTE_TYPE = 0;
pub const eatNASIPAddress: EAP_ATTRIBUTE_TYPE = 4;
pub const eatNASIPv6Address: EAP_ATTRIBUTE_TYPE = 95;
pub const eatNASIdentifier: EAP_ATTRIBUTE_TYPE = 32;
pub const eatNASPort: EAP_ATTRIBUTE_TYPE = 5;
pub const eatNASPortType: EAP_ATTRIBUTE_TYPE = 61;
pub const eatPEAPEmbeddedEAPTypeId: EAP_ATTRIBUTE_TYPE = 8099;
pub const eatPEAPFastRoamedSession: EAP_ATTRIBUTE_TYPE = 8100;
pub const eatPasswordRetry: EAP_ATTRIBUTE_TYPE = 75;
pub const eatPeerId: EAP_ATTRIBUTE_TYPE = 9000;
pub const eatPortLimit: EAP_ATTRIBUTE_TYPE = 62;
pub const eatPrompt: EAP_ATTRIBUTE_TYPE = 76;
pub const eatProxyState: EAP_ATTRIBUTE_TYPE = 33;
pub const eatQuarantineSoH: EAP_ATTRIBUTE_TYPE = 8150;
pub const eatReplyMessage: EAP_ATTRIBUTE_TYPE = 18;
pub const eatReserved: EAP_ATTRIBUTE_TYPE = -1;
pub const eatServerId: EAP_ATTRIBUTE_TYPE = 9001;
pub const eatServiceType: EAP_ATTRIBUTE_TYPE = 6;
pub const eatSessionId: EAP_ATTRIBUTE_TYPE = 9004;
pub const eatSessionTimeout: EAP_ATTRIBUTE_TYPE = 27;
pub const eatSignature: EAP_ATTRIBUTE_TYPE = 80;
pub const eatState: EAP_ATTRIBUTE_TYPE = 24;
pub const eatTerminationAction: EAP_ATTRIBUTE_TYPE = 29;
pub const eatTunnelClientEndpoint: EAP_ATTRIBUTE_TYPE = 66;
pub const eatTunnelMediumType: EAP_ATTRIBUTE_TYPE = 65;
pub const eatTunnelServerEndpoint: EAP_ATTRIBUTE_TYPE = 67;
pub const eatTunnelType: EAP_ATTRIBUTE_TYPE = 64;
pub const eatUnassigned17: EAP_ATTRIBUTE_TYPE = 17;
pub const eatUnassigned21: EAP_ATTRIBUTE_TYPE = 21;
pub const eatUserName: EAP_ATTRIBUTE_TYPE = 1;
pub const eatUserPassword: EAP_ATTRIBUTE_TYPE = 2;
pub const eatVendorSpecific: EAP_ATTRIBUTE_TYPE = 26;
pub const emptLegacyMethodPropertyFlag: EAP_METHOD_PROPERTY_TYPE = 31;
pub const emptPropCertifiedMethod: EAP_METHOD_PROPERTY_TYPE = 22;
pub const emptPropChannelBinding: EAP_METHOD_PROPERTY_TYPE = 16;
pub const emptPropCipherSuiteNegotiation: EAP_METHOD_PROPERTY_TYPE = 0;
pub const emptPropConfidentiality: EAP_METHOD_PROPERTY_TYPE = 4;
pub const emptPropCryptoBinding: EAP_METHOD_PROPERTY_TYPE = 13;
pub const emptPropDictionaryAttackResistance: EAP_METHOD_PROPERTY_TYPE = 11;
pub const emptPropFastReconnect: EAP_METHOD_PROPERTY_TYPE = 12;
pub const emptPropFragmentation: EAP_METHOD_PROPERTY_TYPE = 15;
pub const emptPropHiddenMethod: EAP_METHOD_PROPERTY_TYPE = 23;
pub const emptPropIdentityPrivacy: EAP_METHOD_PROPERTY_TYPE = 26;
pub const emptPropIntegrity: EAP_METHOD_PROPERTY_TYPE = 2;
pub const emptPropKeyDerivation: EAP_METHOD_PROPERTY_TYPE = 5;
pub const emptPropKeyStrength1024: EAP_METHOD_PROPERTY_TYPE = 10;
pub const emptPropKeyStrength128: EAP_METHOD_PROPERTY_TYPE = 7;
pub const emptPropKeyStrength256: EAP_METHOD_PROPERTY_TYPE = 8;
pub const emptPropKeyStrength512: EAP_METHOD_PROPERTY_TYPE = 9;
pub const emptPropKeyStrength64: EAP_METHOD_PROPERTY_TYPE = 6;
pub const emptPropMachineAuth: EAP_METHOD_PROPERTY_TYPE = 24;
pub const emptPropMethodChaining: EAP_METHOD_PROPERTY_TYPE = 27;
pub const emptPropMppeEncryption: EAP_METHOD_PROPERTY_TYPE = 19;
pub const emptPropMutualAuth: EAP_METHOD_PROPERTY_TYPE = 1;
pub const emptPropNap: EAP_METHOD_PROPERTY_TYPE = 17;
pub const emptPropReplayProtection: EAP_METHOD_PROPERTY_TYPE = 3;
pub const emptPropSessionIndependence: EAP_METHOD_PROPERTY_TYPE = 14;
pub const emptPropSharedStateEquivalence: EAP_METHOD_PROPERTY_TYPE = 28;
pub const emptPropStandalone: EAP_METHOD_PROPERTY_TYPE = 18;
pub const emptPropSupportsConfig: EAP_METHOD_PROPERTY_TYPE = 21;
pub const emptPropTunnelMethod: EAP_METHOD_PROPERTY_TYPE = 20;
pub const emptPropUserAuth: EAP_METHOD_PROPERTY_TYPE = 25;
pub const emptPropVendorSpecific: EAP_METHOD_PROPERTY_TYPE = 255;
pub const empvtBool: EAP_METHOD_PROPERTY_VALUE_TYPE = 0;
pub const empvtDword: EAP_METHOD_PROPERTY_VALUE_TYPE = 1;
pub const empvtString: EAP_METHOD_PROPERTY_VALUE_TYPE = 2;
pub const raatARAPChallenge: u32 = 33;
pub const raatARAPChallengeResponse: RAS_AUTH_ATTRIBUTE_TYPE = 84;
pub const raatARAPFeatures: RAS_AUTH_ATTRIBUTE_TYPE = 71;
pub const raatARAPGuestLogon: RAS_AUTH_ATTRIBUTE_TYPE = 8096;
pub const raatARAPNewPassword: u32 = 20;
pub const raatARAPOldPassword: u32 = 19;
pub const raatARAPPassword: RAS_AUTH_ATTRIBUTE_TYPE = 70;
pub const raatARAPPasswordChangeReason: u32 = 21;
pub const raatARAPSecurity: RAS_AUTH_ATTRIBUTE_TYPE = 73;
pub const raatARAPSecurityData: RAS_AUTH_ATTRIBUTE_TYPE = 74;
pub const raatARAPZoneAccess: RAS_AUTH_ATTRIBUTE_TYPE = 72;
pub const raatAcctAuthentic: RAS_AUTH_ATTRIBUTE_TYPE = 45;
pub const raatAcctDelayTime: RAS_AUTH_ATTRIBUTE_TYPE = 41;
pub const raatAcctEventTimeStamp: RAS_AUTH_ATTRIBUTE_TYPE = 55;
pub const raatAcctInputOctets: RAS_AUTH_ATTRIBUTE_TYPE = 42;
pub const raatAcctInputPackets: RAS_AUTH_ATTRIBUTE_TYPE = 47;
pub const raatAcctInterimInterval: RAS_AUTH_ATTRIBUTE_TYPE = 85;
pub const raatAcctLinkCount: RAS_AUTH_ATTRIBUTE_TYPE = 51;
pub const raatAcctMultiSessionId: RAS_AUTH_ATTRIBUTE_TYPE = 50;
pub const raatAcctOutputOctets: RAS_AUTH_ATTRIBUTE_TYPE = 43;
pub const raatAcctOutputPackets: RAS_AUTH_ATTRIBUTE_TYPE = 48;
pub const raatAcctSessionId: RAS_AUTH_ATTRIBUTE_TYPE = 44;
pub const raatAcctSessionTime: RAS_AUTH_ATTRIBUTE_TYPE = 46;
pub const raatAcctStatusType: RAS_AUTH_ATTRIBUTE_TYPE = 40;
pub const raatAcctTerminateCause: RAS_AUTH_ATTRIBUTE_TYPE = 49;
pub const raatCallbackId: RAS_AUTH_ATTRIBUTE_TYPE = 20;
pub const raatCallbackNumber: RAS_AUTH_ATTRIBUTE_TYPE = 19;
pub const raatCalledStationId: RAS_AUTH_ATTRIBUTE_TYPE = 30;
pub const raatCallingStationId: RAS_AUTH_ATTRIBUTE_TYPE = 31;
pub const raatCertificateOID: RAS_AUTH_ATTRIBUTE_TYPE = 8097;
pub const raatCertificateThumbprint: RAS_AUTH_ATTRIBUTE_TYPE = 8250;
pub const raatClass: RAS_AUTH_ATTRIBUTE_TYPE = 25;
pub const raatConfigurationToken: RAS_AUTH_ATTRIBUTE_TYPE = 78;
pub const raatConnectInfo: RAS_AUTH_ATTRIBUTE_TYPE = 77;
pub const raatCredentialsChanged: RAS_AUTH_ATTRIBUTE_TYPE = 8103;
pub const raatEAPConfiguration: RAS_AUTH_ATTRIBUTE_TYPE = 8098;
pub const raatEAPMessage: RAS_AUTH_ATTRIBUTE_TYPE = 79;
pub const raatEAPTLV: RAS_AUTH_ATTRIBUTE_TYPE = 8102;
pub const raatEMSK: RAS_AUTH_ATTRIBUTE_TYPE = 9003;
pub const raatFastRoamedSession: RAS_AUTH_ATTRIBUTE_TYPE = 8100;
pub const raatFilterId: RAS_AUTH_ATTRIBUTE_TYPE = 11;
pub const raatFramedAppleTalkLink: RAS_AUTH_ATTRIBUTE_TYPE = 37;
pub const raatFramedAppleTalkNetwork: RAS_AUTH_ATTRIBUTE_TYPE = 38;
pub const raatFramedAppleTalkZone: RAS_AUTH_ATTRIBUTE_TYPE = 39;
pub const raatFramedCompression: RAS_AUTH_ATTRIBUTE_TYPE = 13;
pub const raatFramedIPAddress: RAS_AUTH_ATTRIBUTE_TYPE = 8;
pub const raatFramedIPNetmask: RAS_AUTH_ATTRIBUTE_TYPE = 9;
pub const raatFramedIPXNetwork: RAS_AUTH_ATTRIBUTE_TYPE = 23;
pub const raatFramedIPv6Pool: RAS_AUTH_ATTRIBUTE_TYPE = 100;
pub const raatFramedIPv6Prefix: RAS_AUTH_ATTRIBUTE_TYPE = 97;
pub const raatFramedIPv6Route: RAS_AUTH_ATTRIBUTE_TYPE = 99;
pub const raatFramedInterfaceId: RAS_AUTH_ATTRIBUTE_TYPE = 96;
pub const raatFramedMTU: RAS_AUTH_ATTRIBUTE_TYPE = 12;
pub const raatFramedProtocol: RAS_AUTH_ATTRIBUTE_TYPE = 7;
pub const raatFramedRoute: RAS_AUTH_ATTRIBUTE_TYPE = 22;
pub const raatFramedRouting: RAS_AUTH_ATTRIBUTE_TYPE = 10;
pub const raatIdleTimeout: RAS_AUTH_ATTRIBUTE_TYPE = 28;
pub const raatInnerEAPTypeId: RAS_AUTH_ATTRIBUTE_TYPE = 8099;
pub const raatLoginIPHost: RAS_AUTH_ATTRIBUTE_TYPE = 14;
pub const raatLoginIPv6Host: RAS_AUTH_ATTRIBUTE_TYPE = 98;
pub const raatLoginLATGroup: RAS_AUTH_ATTRIBUTE_TYPE = 36;
pub const raatLoginLATNode: RAS_AUTH_ATTRIBUTE_TYPE = 35;
pub const raatLoginLATPort: RAS_AUTH_ATTRIBUTE_TYPE = 63;
pub const raatLoginLATService: RAS_AUTH_ATTRIBUTE_TYPE = 34;
pub const raatLoginService: RAS_AUTH_ATTRIBUTE_TYPE = 15;
pub const raatLoginTCPPort: RAS_AUTH_ATTRIBUTE_TYPE = 16;
pub const raatMD5CHAPChallenge: RAS_AUTH_ATTRIBUTE_TYPE = 60;
pub const raatMD5CHAPPassword: RAS_AUTH_ATTRIBUTE_TYPE = 3;
pub const raatMethodId: RAS_AUTH_ATTRIBUTE_TYPE = 9002;
pub const raatMinimum: RAS_AUTH_ATTRIBUTE_TYPE = 0;
pub const raatNASIPAddress: RAS_AUTH_ATTRIBUTE_TYPE = 4;
pub const raatNASIPv6Address: RAS_AUTH_ATTRIBUTE_TYPE = 95;
pub const raatNASIdentifier: RAS_AUTH_ATTRIBUTE_TYPE = 32;
pub const raatNASPort: RAS_AUTH_ATTRIBUTE_TYPE = 5;
pub const raatNASPortType: RAS_AUTH_ATTRIBUTE_TYPE = 61;
pub const raatPEAPEmbeddedEAPTypeId: RAS_AUTH_ATTRIBUTE_TYPE = 8099;
pub const raatPEAPFastRoamedSession: RAS_AUTH_ATTRIBUTE_TYPE = 8100;
pub const raatPasswordRetry: RAS_AUTH_ATTRIBUTE_TYPE = 75;
pub const raatPeerId: RAS_AUTH_ATTRIBUTE_TYPE = 9000;
pub const raatPortLimit: RAS_AUTH_ATTRIBUTE_TYPE = 62;
pub const raatPrompt: RAS_AUTH_ATTRIBUTE_TYPE = 76;
pub const raatProxyState: RAS_AUTH_ATTRIBUTE_TYPE = 33;
pub const raatReplyMessage: RAS_AUTH_ATTRIBUTE_TYPE = 18;
pub const raatReserved: RAS_AUTH_ATTRIBUTE_TYPE = -1;
pub const raatServerId: RAS_AUTH_ATTRIBUTE_TYPE = 9001;
pub const raatServiceType: RAS_AUTH_ATTRIBUTE_TYPE = 6;
pub const raatSessionId: RAS_AUTH_ATTRIBUTE_TYPE = 9004;
pub const raatSessionTimeout: RAS_AUTH_ATTRIBUTE_TYPE = 27;
pub const raatSignature: RAS_AUTH_ATTRIBUTE_TYPE = 80;
pub const raatState: RAS_AUTH_ATTRIBUTE_TYPE = 24;
pub const raatTerminationAction: RAS_AUTH_ATTRIBUTE_TYPE = 29;
pub const raatTunnelClientEndpoint: RAS_AUTH_ATTRIBUTE_TYPE = 66;
pub const raatTunnelMediumType: RAS_AUTH_ATTRIBUTE_TYPE = 65;
pub const raatTunnelServerEndpoint: RAS_AUTH_ATTRIBUTE_TYPE = 67;
pub const raatTunnelType: RAS_AUTH_ATTRIBUTE_TYPE = 64;
pub const raatUnassigned17: RAS_AUTH_ATTRIBUTE_TYPE = 17;
pub const raatUnassigned21: RAS_AUTH_ATTRIBUTE_TYPE = 21;
pub const raatUserName: RAS_AUTH_ATTRIBUTE_TYPE = 1;
pub const raatUserPassword: RAS_AUTH_ATTRIBUTE_TYPE = 2;
pub const raatVendorSpecific: RAS_AUTH_ATTRIBUTE_TYPE = 26;
