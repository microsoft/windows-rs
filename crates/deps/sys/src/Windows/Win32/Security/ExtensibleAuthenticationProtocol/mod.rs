#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    fn EapHostPeerBeginSession();
    fn EapHostPeerClearConnection();
    fn EapHostPeerConfigBlob2Xml();
    fn EapHostPeerConfigXml2Blob();
    fn EapHostPeerCredentialsXml2Blob();
    fn EapHostPeerEndSession();
    fn EapHostPeerFreeEapError();
    fn EapHostPeerFreeErrorMemory();
    fn EapHostPeerFreeMemory();
    fn EapHostPeerFreeRuntimeMemory();
    fn EapHostPeerGetAuthStatus();
    fn EapHostPeerGetDataToUnplumbCredentials();
    fn EapHostPeerGetEncryptedPassword();
    fn EapHostPeerGetIdentity();
    fn EapHostPeerGetMethodProperties();
    fn EapHostPeerGetMethods();
    fn EapHostPeerGetResponseAttributes();
    fn EapHostPeerGetResult();
    fn EapHostPeerGetSendPacket();
    fn EapHostPeerGetUIContext();
    fn EapHostPeerInitialize();
    fn EapHostPeerInvokeConfigUI();
    fn EapHostPeerInvokeIdentityUI();
    fn EapHostPeerInvokeInteractiveUI();
    fn EapHostPeerProcessReceivedPacket();
    fn EapHostPeerQueryCredentialInputFields();
    fn EapHostPeerQueryInteractiveUIInputFields();
    fn EapHostPeerQueryUIBlobFromInteractiveUIInputFields();
    fn EapHostPeerQueryUserBlobFromCredentialInputFields();
    fn EapHostPeerSetResponseAttributes();
    fn EapHostPeerSetUIContext();
    fn EapHostPeerUninitialize();
}
