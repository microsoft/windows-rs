#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerBeginSession(dwflags: u32, eaptype: EAP_METHOD_TYPE, pattributearray: *const EAP_ATTRIBUTES, htokenimpersonateuser: super::super::Foundation::HANDLE, dwsizeofconnectiondata: u32, pconnectiondata: *const u8, dwsizeofuserdata: u32, puserdata: *const u8, dwmaxsendpacketsize: u32, pconnectionid: *const ::windows_sys::core::GUID, func: NotificationHandler, pcontextdata: *mut ::core::ffi::c_void, psessionid: *mut u32, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerClearConnection(pconnectionid: *mut ::windows_sys::core::GUID, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Data_Xml_MsXml`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub fn EapHostPeerConfigBlob2Xml(dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, dwsizeofconfigin: u32, pconfigin: *const u8, ppconfigdoc: *mut super::super::Data::Xml::MsXml::IXMLDOMDocument2, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Data_Xml_MsXml`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub fn EapHostPeerConfigXml2Blob(dwflags: u32, pconfigdoc: super::super::Data::Xml::MsXml::IXMLDOMNode, pdwsizeofconfigout: *mut u32, ppconfigout: *mut *mut u8, peapmethodtype: *mut EAP_METHOD_TYPE, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Data_Xml_MsXml`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub fn EapHostPeerCredentialsXml2Blob(dwflags: u32, pcredentialsdoc: super::super::Data::Xml::MsXml::IXMLDOMNode, dwsizeofconfigin: u32, pconfigin: *const u8, pdwsizeofcredentialsout: *mut u32, ppcredentialsout: *mut *mut u8, peapmethodtype: *mut EAP_METHOD_TYPE, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerEndSession(sessionhandle: u32, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerFreeEapError(peaperror: *mut EAP_ERROR);
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerFreeErrorMemory(peaperror: *mut EAP_ERROR);
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
    pub fn EapHostPeerFreeMemory(pdata: *mut u8);
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
    pub fn EapHostPeerFreeRuntimeMemory(pdata: *mut u8);
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetAuthStatus(sessionhandle: u32, authparam: EapHostPeerAuthParams, pcbauthdata: *mut u32, ppauthdata: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetDataToUnplumbCredentials(pconnectionidthatlastsavedcreds: *mut ::windows_sys::core::GUID, phcredentialimpersonationtoken: *mut isize, sessionhandle: u32, ppeaperror: *mut *mut EAP_ERROR, fsavetocredman: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetEncryptedPassword(dwsizeofpassword: u32, szpassword: super::super::Foundation::PWSTR, ppszencpassword: *mut super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetIdentity(dwversion: u32, dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, dwsizeofconnectiondata: u32, pconnectiondata: *const u8, dwsizeofuserdata: u32, puserdata: *const u8, htokenimpersonateuser: super::super::Foundation::HANDLE, pfinvokeui: *mut super::super::Foundation::BOOL, pdwsizeofuserdataout: *mut u32, ppuserdataout: *mut *mut u8, ppwszidentity: *mut super::super::Foundation::PWSTR, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetMethodProperties(dwversion: u32, dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, huserimpersonationtoken: super::super::Foundation::HANDLE, dweapconndatasize: u32, pbeapconndata: *const u8, dwuserdatasize: u32, pbuserdata: *const u8, pmethodpropertyarray: *mut EAP_METHOD_PROPERTY_ARRAY, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetMethods(peapmethodinfoarray: *mut EAP_METHOD_INFO_ARRAY, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetResponseAttributes(sessionhandle: u32, pattribs: *mut EAP_ATTRIBUTES, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetResult(sessionhandle: u32, reason: EapHostPeerMethodResultReason, ppresult: *mut EapHostPeerMethodResult, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetSendPacket(sessionhandle: u32, pcbsendpacket: *mut u32, ppsendpacket: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetUIContext(sessionhandle: u32, pdwsizeofuicontextdata: *mut u32, ppuicontextdata: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
    pub fn EapHostPeerInitialize() -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerInvokeConfigUI(hwndparent: super::super::Foundation::HWND, dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, dwsizeofconfigin: u32, pconfigin: *const u8, pdwsizeofconfigout: *mut u32, ppconfigout: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerInvokeIdentityUI(dwversion: u32, eapmethodtype: EAP_METHOD_TYPE, dwflags: u32, hwndparent: super::super::Foundation::HWND, dwsizeofconnectiondata: u32, pconnectiondata: *const u8, dwsizeofuserdata: u32, puserdata: *const u8, pdwsizeofuserdataout: *mut u32, ppuserdataout: *mut *mut u8, ppwszidentity: *mut super::super::Foundation::PWSTR, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerInvokeInteractiveUI(hwndparent: super::super::Foundation::HWND, dwsizeofuicontextdata: u32, puicontextdata: *const u8, pdwsizeofdatafrominteractiveui: *mut u32, ppdatafrominteractiveui: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerProcessReceivedPacket(sessionhandle: u32, cbreceivepacket: u32, preceivepacket: *const u8, peapoutput: *mut EapHostPeerResponseAction, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerQueryCredentialInputFields(huserimpersonationtoken: super::super::Foundation::HANDLE, eapmethodtype: EAP_METHOD_TYPE, dwflags: u32, dweapconndatasize: u32, pbeapconndata: *const u8, peapconfiginputfieldarray: *mut EAP_CONFIG_INPUT_FIELD_ARRAY, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerQueryInteractiveUIInputFields(dwversion: u32, dwflags: u32, dwsizeofuicontextdata: u32, puicontextdata: *const u8, peapinteractiveuidata: *mut EAP_INTERACTIVE_UI_DATA, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerQueryUIBlobFromInteractiveUIInputFields(dwversion: u32, dwflags: u32, dwsizeofuicontextdata: u32, puicontextdata: *const u8, peapinteractiveuidata: *const EAP_INTERACTIVE_UI_DATA, pdwsizeofdatafrominteractiveui: *mut u32, ppdatafrominteractiveui: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR, ppvreserved: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerQueryUserBlobFromCredentialInputFields(huserimpersonationtoken: super::super::Foundation::HANDLE, eapmethodtype: EAP_METHOD_TYPE, dwflags: u32, dweapconndatasize: u32, pbeapconndata: *const u8, peapconfiginputfieldarray: *const EAP_CONFIG_INPUT_FIELD_ARRAY, pdwuserblobsize: *mut u32, ppbuserblob: *mut *mut u8, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerSetResponseAttributes(sessionhandle: u32, pattribs: *const EAP_ATTRIBUTES, peapoutput: *mut EapHostPeerResponseAction, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerSetUIContext(sessionhandle: u32, dwsizeofuicontextdata: u32, puicontextdata: *const u8, peapoutput: *mut EapHostPeerResponseAction, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
    pub fn EapHostPeerUninitialize();
}
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const CERTIFICATE_HASH_LENGTH: u32 = 20u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAPCODE_Failure: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAPCODE_Request: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAPCODE_Response: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAPCODE_Success: u32 = 3u32;
pub struct EAPHOST_AUTH_INFO(i32);
pub struct EAPHOST_AUTH_STATUS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EAPHOST_IDENTITY_UI_PARAMS(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EAPHOST_INTERACTIVE_UI_PARAMS(i32);
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAPHOST_METHOD_API_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAPHOST_PEER_API_VERSION: u32 = 1u32;
pub struct EAP_ATTRIBUTE(i32);
pub struct EAP_ATTRIBUTES(i32);
pub struct EAP_ATTRIBUTE_TYPE(i32);
pub struct EAP_AUTHENTICATOR_METHOD_ROUTINES(i32);
pub struct EAP_AUTHENTICATOR_SEND_TIMEOUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_CONFIG_INPUT_FIELD_ARRAY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_CONFIG_INPUT_FIELD_DATA(i32);
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_CONFIG_INPUT_FIELD_PROPS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_CONFIG_INPUT_FIELD_PROPS_NON_DISPLAYABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_CONFIG_INPUT_FIELD_PROPS_NON_PERSIST: u32 = 2u32;
pub struct EAP_CONFIG_INPUT_FIELD_TYPE(i32);
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_CREDENTIAL_VERSION: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_CRED_EXPIRY_REQ(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_ERROR(i32);
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_AUTHENTICATION_FAILED: u32 = 2151809045u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_CERT_STORE_INACCESSIBLE: u32 = 2151809040u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_EAPHOST_EAPQEC_INACCESSIBLE: u32 = 2151809043u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_EAPHOST_FIRST: i32 = -2143158272i32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_EAPHOST_IDENTITY_UNKNOWN: u32 = 2151809044u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_EAPHOST_LAST: i32 = -2143158017i32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_EAPHOST_METHOD_INVALID_PACKET: u32 = 2151809047u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_EAPHOST_METHOD_NOT_INSTALLED: u32 = 2151809041u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_EAPHOST_METHOD_OPERATION_NOT_SUPPORTED: u32 = 2151809056u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_EAPHOST_REMOTE_INVALID_PACKET: u32 = 2151809048u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_EAPHOST_THIRDPARTY_METHOD_HOST_RESET: u32 = 2151809042u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_EAPHOST_XML_MALFORMED: u32 = 2151809049u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_METHOD_CONFIG_DOES_NOT_SUPPORT_SSO: u32 = 2151809050u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_NO_SMART_CARD_READER: u32 = 2151809299u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_SERVER_CERT_EXPIRED: u32 = 2151809538u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_SERVER_CERT_INVALID: u32 = 2151809537u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_SERVER_CERT_NOT_FOUND: u32 = 2151809536u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_SERVER_CERT_OTHER_ERROR: u32 = 2151809540u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_SERVER_CERT_REVOKED: u32 = 2151809539u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_SERVER_FIRST: i32 = -2143157760i32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_SERVER_LAST: i32 = -2143157505i32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_SERVER_ROOT_CERT_FIRST: i32 = -2143157248i32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_SERVER_ROOT_CERT_INVALID: u32 = 2151810049u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_SERVER_ROOT_CERT_LAST: i32 = -2143156993i32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_SERVER_ROOT_CERT_NAME_REQUIRED: u32 = 2151810054u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_SERVER_ROOT_CERT_NOT_FOUND: u32 = 2151810048u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_SIM_NOT_VALID: u32 = 2151810304u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_CERT_EXPIRED: u32 = 2151809282u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_CERT_INVALID: u32 = 2151809281u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_CERT_NOT_FOUND: u32 = 2151809280u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_CERT_OTHER_ERROR: u32 = 2151809284u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_CERT_REJECTED: u32 = 2151809285u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_CERT_REVOKED: u32 = 2151809283u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_CREDENTIALS_REJECTED: u32 = 2151809297u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_FIRST: i32 = -2143158016i32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_LAST: i32 = -2143157761i32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_NAME_PASSWORD_REJECTED: u32 = 2151809298u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_ROOT_CERT_EXPIRED: u32 = 2151809794u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_ROOT_CERT_FIRST: i32 = -2143157504i32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_ROOT_CERT_INVALID: u32 = 2151809793u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_ROOT_CERT_LAST: i32 = -2143157249i32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_E_USER_ROOT_CERT_NOT_FOUND: u32 = 2151809792u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_CONFG_READONLY: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_FULL_AUTH: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_GUEST_ACCESS: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_LOGON: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_MACHINE_AUTH: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_NON_INTERACTIVE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_ONLY_EAP_TLS: u32 = 16777216u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_PREFER_ALT_CREDENTIALS: u32 = 8192u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_PREVIEW: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_PRE_LOGON: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_RESUME_FROM_HIBERNATE: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_Reserved1: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_Reserved2: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_Reserved3: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_Reserved4: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_Reserved5: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_Reserved6: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_Reserved7: u32 = 16384u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_Reserved8: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_Reserved9: u32 = 4194304u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_SERVER_VALIDATION_REQUIRED: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_SUPRESS_UI: u32 = 65536u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_USER_AUTH: u32 = 262144u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_FLAG_VPN: u32 = 8388608u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_GROUP_MASK: i32 = 65280i32;
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_INTERACTIVE_UI_DATA(i32);
pub struct EAP_INTERACTIVE_UI_DATA_TYPE(i32);
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_INTERACTIVE_UI_DATA_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_INVALID_PACKET: u32 = 2151809048u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_I_EAPHOST_EAP_NEGOTIATION_FAILED: u32 = 1078067222u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_I_EAPHOST_FIRST: i32 = -2143158272i32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_I_EAPHOST_LAST: i32 = -2143158017i32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_I_USER_ACCOUNT_OTHER_ERROR: u32 = 1078067472u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_I_USER_FIRST: i32 = 1078067456i32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_I_USER_LAST: i32 = 1078067711i32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_METHOD_AUTHENTICATOR_CONFIG_IS_IDENTITY_PRIVACY: u32 = 1u32;
pub struct EAP_METHOD_AUTHENTICATOR_RESPONSE_ACTION(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_AUTHENTICATOR_RESULT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_INFO_ARRAY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_INFO_ARRAY_EX(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_INFO_EX(i32);
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_METHOD_INVALID_PACKET: u32 = 2151809047u32;
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_PROPERTY(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_PROPERTY_ARRAY(i32);
pub struct EAP_METHOD_PROPERTY_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_PROPERTY_VALUE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_METHOD_PROPERTY_VALUE_BOOL(i32);
pub struct EAP_METHOD_PROPERTY_VALUE_DWORD(i32);
pub struct EAP_METHOD_PROPERTY_VALUE_STRING(i32);
pub struct EAP_METHOD_PROPERTY_VALUE_TYPE(i32);
pub struct EAP_METHOD_TYPE(i32);
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_PEER_FLAG_GUEST_ACCESS: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_PEER_FLAG_HEALTH_STATE_CHANGE: u32 = 32768u32;
pub struct EAP_PEER_METHOD_ROUTINES(i32);
pub struct EAP_TYPE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EAP_UI_DATA_FORMAT(i32);
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_UI_INPUT_FIELD_PROPS_DEFAULT: u32 = 0u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_UI_INPUT_FIELD_PROPS_NON_DISPLAYABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_UI_INPUT_FIELD_PROPS_NON_PERSIST: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const EAP_UI_INPUT_FIELD_PROPS_READ_ONLY: u32 = 4u32;
#[cfg(feature = "Win32_Foundation")]
pub struct EapCertificateCredential(i32);
pub struct EapCode(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EapCredential(i32);
pub struct EapCredentialType(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EapCredentialTypeData(i32);
pub struct EapHostPeerAuthParams(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EapHostPeerMethodResult(i32);
pub struct EapHostPeerMethodResultReason(i32);
pub struct EapHostPeerResponseAction(i32);
pub struct EapPacket(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EapPeerMethodOutput(i32);
pub struct EapPeerMethodResponseAction(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EapPeerMethodResult(i32);
pub struct EapPeerMethodResultReason(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EapSimCredential(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct EapUsernamePasswordCredential(i32);
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const FACILITY_EAP_MESSAGE: u32 = 2114u32;
pub const GUID_EapHost_Cause_CertStoreInaccessible: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 4] };
pub const GUID_EapHost_Cause_EapNegotiationFailed: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 28] };
pub const GUID_EapHost_Cause_EapQecInaccessible: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 3, 18] };
pub const GUID_EapHost_Cause_Generic_AuthFailure: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 1, 4] };
pub const GUID_EapHost_Cause_IdentityUnknown: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 2, 4] };
pub const GUID_EapHost_Cause_MethodDLLNotFound: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 1] };
pub const GUID_EapHost_Cause_MethodDoesNotSupportOperation: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 30] };
pub const GUID_EapHost_Cause_Method_Config_Does_Not_Support_Sso: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 3659054386, data2: 79, data3: 16890, data4: [174, 8, 11, 200, 94, 88, 69, 172] };
pub const GUID_EapHost_Cause_No_SmartCardReader_Found: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 43] };
pub const GUID_EapHost_Cause_Server_CertExpired: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 5] };
pub const GUID_EapHost_Cause_Server_CertInvalid: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 6] };
pub const GUID_EapHost_Cause_Server_CertNotFound: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 7] };
pub const GUID_EapHost_Cause_Server_CertOtherError: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 1, 8] };
pub const GUID_EapHost_Cause_Server_CertRevoked: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 8] };
pub const GUID_EapHost_Cause_Server_Root_CertNameRequired: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 18] };
pub const GUID_EapHost_Cause_Server_Root_CertNotFound: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 1, 18] };
pub const GUID_EapHost_Cause_SimNotValid: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 3, 4] };
pub const GUID_EapHost_Cause_ThirdPartyMethod_Host_Reset: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 2, 18] };
pub const GUID_EapHost_Cause_User_Account_OtherProblem: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 1, 14] };
pub const GUID_EapHost_Cause_User_CertExpired: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 9] };
pub const GUID_EapHost_Cause_User_CertInvalid: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 10] };
pub const GUID_EapHost_Cause_User_CertNotFound: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 11] };
pub const GUID_EapHost_Cause_User_CertOtherError: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 12] };
pub const GUID_EapHost_Cause_User_CertRejected: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 13] };
pub const GUID_EapHost_Cause_User_CertRevoked: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 14] };
pub const GUID_EapHost_Cause_User_CredsRejected: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 2, 14] };
pub const GUID_EapHost_Cause_User_Root_CertExpired: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 15] };
pub const GUID_EapHost_Cause_User_Root_CertInvalid: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 16] };
pub const GUID_EapHost_Cause_User_Root_CertNotFound: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 17] };
pub const GUID_EapHost_Cause_XmlMalformed: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 29] };
pub const GUID_EapHost_Default: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 0, data2: 0, data3: 0, data4: [0, 0, 0, 0, 0, 0, 0, 0] };
pub const GUID_EapHost_Help_ObtainingCerts: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 4113952419,
    data2: 7133,
    data3: 18122,
    data4: [162, 252, 166, 101, 89, 57, 183, 232],
};
pub const GUID_EapHost_Help_Troubleshooting: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 858815183, data2: 1688, data3: 16826, data4: [176, 20, 234, 10, 46, 184, 208, 168] };
pub const GUID_EapHost_Repair_ContactAdmin_AuthFailure: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 31] };
pub const GUID_EapHost_Repair_ContactAdmin_CertNameAbsent: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 41] };
pub const GUID_EapHost_Repair_ContactAdmin_CertStoreInaccessible: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 36] };
pub const GUID_EapHost_Repair_ContactAdmin_IdentityUnknown: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 32] };
pub const GUID_EapHost_Repair_ContactAdmin_InvalidUserAccount: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 37] };
pub const GUID_EapHost_Repair_ContactAdmin_InvalidUserCert: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 44] };
pub const GUID_EapHost_Repair_ContactAdmin_MethodNotFound: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 34] };
pub const GUID_EapHost_Repair_ContactAdmin_NegotiationFailed: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 33] };
pub const GUID_EapHost_Repair_ContactAdmin_NoSmartCardReader: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 42] };
pub const GUID_EapHost_Repair_ContactAdmin_RootCertInvalid: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 38] };
pub const GUID_EapHost_Repair_ContactAdmin_RootCertNotFound: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 39] };
pub const GUID_EapHost_Repair_ContactAdmin_RootExpired: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 40] };
pub const GUID_EapHost_Repair_ContactSysadmin: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 2] };
pub const GUID_EapHost_Repair_Method_Not_Support_Sso: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 45] };
pub const GUID_EapHost_Repair_No_ValidSim_Found: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 46] };
pub const GUID_EapHost_Repair_RestartNap: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 35] };
pub const GUID_EapHost_Repair_Retry_Authentication: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 1, 27] };
pub const GUID_EapHost_Repair_Server_ClientSelectServerCert: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 24] };
pub const GUID_EapHost_Repair_User_AuthFailure: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 25] };
pub const GUID_EapHost_Repair_User_GetNewCert: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 26] };
pub const GUID_EapHost_Repair_User_SelectValidCert: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 2517826663, data2: 24912, data3: 16905, data4: [168, 94, 168, 216, 0, 0, 0, 27] };
pub struct IAccountingProviderConfig(pub *mut ::core::ffi::c_void);
pub struct IAuthenticationProviderConfig(pub *mut ::core::ffi::c_void);
pub struct IEAPProviderConfig(pub *mut ::core::ffi::c_void);
pub struct IEAPProviderConfig2(pub *mut ::core::ffi::c_void);
pub struct IEAPProviderConfig3(pub *mut ::core::ffi::c_void);
pub struct IRouterProtocolConfig(pub *mut ::core::ffi::c_void);
pub struct ISOLATION_STATE(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct LEGACY_IDENTITY_UI_PARAMS(i32);
pub struct LEGACY_INTERACTIVE_UI_PARAMS(i32);
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const MAXEAPCODE: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const MAX_EAP_CONFIG_INPUT_FIELD_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const MAX_EAP_CONFIG_INPUT_FIELD_VALUE_LENGTH: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const NCRYPT_PIN_CACHE_PIN_BYTE_LENGTH: u32 = 90u32;
#[cfg(feature = "Win32_Foundation")]
pub struct NgcTicketContext(i32);
pub struct NotificationHandler(i32);
pub struct PPP_EAP_ACTION(i32);
pub struct PPP_EAP_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PPP_EAP_INPUT(i32);
#[cfg(feature = "Win32_Foundation")]
pub struct PPP_EAP_OUTPUT(i32);
pub struct PPP_EAP_PACKET(i32);
pub struct RAS_AUTH_ATTRIBUTE(i32);
pub struct RAS_AUTH_ATTRIBUTE_TYPE(i32);
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_8021X_AUTH: u32 = 128u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_ALTERNATIVE_USER_DB: u32 = 2048u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_CONFG_READONLY: u32 = 524288u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_FIRST_LINK: u32 = 16u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_GUEST_ACCESS: u32 = 64u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_HOSTED_IN_PEAP: u32 = 256u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_LOGON: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_MACHINE_AUTH: u32 = 32u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_NON_INTERACTIVE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_PEAP_FORCE_FULL_AUTH: u32 = 4096u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_PEAP_UPFRONT: u32 = 1024u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_PREVIEW: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_PRE_LOGON: u32 = 131072u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_RESERVED: u32 = 1048576u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_RESUME_FROM_HIBERNATE: u32 = 512u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_ROUTER: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_SAVE_CREDMAN: u32 = 2097152u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_FLAG_SERVER_VALIDATION_REQUIRED: u32 = 33554432u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_ROLE_AUTHENTICATEE: u32 = 2u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_ROLE_AUTHENTICATOR: u32 = 1u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_ROLE_EXCLUDE_IN_EAP: u32 = 4u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_ROLE_EXCLUDE_IN_PEAP: u32 = 8u32;
#[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
pub const RAS_EAP_ROLE_EXCLUDE_IN_VPN: u32 = 16u32;
