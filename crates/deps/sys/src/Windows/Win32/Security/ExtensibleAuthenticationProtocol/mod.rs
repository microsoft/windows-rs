#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerBeginSession(dwflags: u32, eaptype: EAP_METHOD_TYPE, pattributearray: *const EAP_ATTRIBUTES, htokenimpersonateuser: super::super::Foundation::HANDLE, dwsizeofconnectiondata: u32, pconnectiondata: *const u8, dwsizeofuserdata: u32, puserdata: *const u8, dwmaxsendpacketsize: u32, pconnectionid: *const ::windows::runtime::GUID, func: ::windows::runtime::RawPtr, pcontextdata: *mut ::core::ffi::c_void, psessionid: *mut u32, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerClearConnection(pconnectionid: *mut ::windows::runtime::GUID, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Data_Xml_MsXml`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub fn EapHostPeerConfigBlob2Xml(dwflags: u32, eapmethodtype: EAP_METHOD_TYPE, dwsizeofconfigin: u32, pconfigin: *const u8, ppconfigdoc: *mut ::windows::runtime::RawPtr, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Data_Xml_MsXml`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub fn EapHostPeerConfigXml2Blob(dwflags: u32, pconfigdoc: ::windows::runtime::RawPtr, pdwsizeofconfigout: *mut u32, ppconfigout: *mut *mut u8, peapmethodtype: *mut EAP_METHOD_TYPE, ppeaperror: *mut *mut EAP_ERROR) -> u32;
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Data_Xml_MsXml`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub fn EapHostPeerCredentialsXml2Blob(dwflags: u32, pcredentialsdoc: ::windows::runtime::RawPtr, dwsizeofconfigin: u32, pconfigin: *const u8, pdwsizeofcredentialsout: *mut u32, ppcredentialsout: *mut *mut u8, peapmethodtype: *mut EAP_METHOD_TYPE, ppeaperror: *mut *mut EAP_ERROR) -> u32;
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
    pub fn EapHostPeerGetDataToUnplumbCredentials(pconnectionidthatlastsavedcreds: *mut ::windows::runtime::GUID, phcredentialimpersonationtoken: *mut isize, sessionhandle: u32, ppeaperror: *mut *mut EAP_ERROR, fsavetocredman: *mut super::super::Foundation::BOOL) -> u32;
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
