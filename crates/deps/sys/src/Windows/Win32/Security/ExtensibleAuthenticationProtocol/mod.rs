#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerBeginSession();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerClearConnection();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Data_Xml_MsXml`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub fn EapHostPeerConfigBlob2Xml();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Data_Xml_MsXml`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub fn EapHostPeerConfigXml2Blob();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Data_Xml_MsXml`, `Win32_Foundation`*"]
    #[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation"))]
    pub fn EapHostPeerCredentialsXml2Blob();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerEndSession();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerFreeEapError();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerFreeErrorMemory();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
    pub fn EapHostPeerFreeMemory();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
    pub fn EapHostPeerFreeRuntimeMemory();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetAuthStatus();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetDataToUnplumbCredentials();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetEncryptedPassword();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetIdentity();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetMethodProperties();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetMethods();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetResponseAttributes();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetResult();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetSendPacket();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerGetUIContext();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
    pub fn EapHostPeerInitialize();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerInvokeConfigUI();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerInvokeIdentityUI();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerInvokeInteractiveUI();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerProcessReceivedPacket();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerQueryCredentialInputFields();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerQueryInteractiveUIInputFields();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerQueryUIBlobFromInteractiveUIInputFields();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerQueryUserBlobFromCredentialInputFields();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerSetResponseAttributes();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EapHostPeerSetUIContext();
    #[doc = "*Required features: `Win32_Security_ExtensibleAuthenticationProtocol`*"]
    pub fn EapHostPeerUninitialize();
}
