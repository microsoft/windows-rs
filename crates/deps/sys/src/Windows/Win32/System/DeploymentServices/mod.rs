#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeAsyncRecvDone();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpAppendOption();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpAppendOptionRaw();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpGetOptionValue();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpGetVendorOptionValue();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpInitialize();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeDhcpIsValid();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpv6AppendOption();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpv6AppendOptionRaw();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpv6CreateRelayRepl();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpv6GetOptionValue();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpv6GetVendorOptionValue();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpv6Initialize();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeDhcpv6IsValid();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpv6ParseRelayForw();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeGetServerInfo();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeGetServerInfoEx();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxePacketAllocate();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxePacketFree();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderEnumClose();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderEnumFirst();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderEnumNext();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderFreeInfo();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderQueryIndex();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn PxeProviderRegister();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderSetAttribute();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderUnRegister();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeRegisterCallback();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeSendReply();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeTrace();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeTraceV();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpAddOption();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpCloseHandle();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpGetOptionBuffer();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpInitialize();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpParseInitialize();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpParseInitializev6();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpQueryOption();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliAuthorizeSession();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliCancelTransfer();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliClose();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliCreateSession();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliFindFirstImage();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliFindNextImage();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliFreeStringArray();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetDriverQueryXml();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetEnumerationFlags();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageArchitecture();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageDescription();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageFiles();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageGroup();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageHalName();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageHandleFromFindHandle();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageHandleFromTransferHandle();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageIndex();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageLanguage();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageLanguages();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageLastModifiedTime();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageName();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageNamespace();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageParameter();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImagePath();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageSize();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageType();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageVersion();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetTransferSize();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliInitializeLog();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliLog();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliObtainDriverPackages();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliObtainDriverPackagesEx();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliRegisterTrace();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn WdsCliSetTransferBufferSize();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliTransferFile();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliTransferImage();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliWaitForTransfer();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn WdsTransportClientAddRefBuffer();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCancelSession();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCancelSessionEx();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCloseSession();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCompleteReceive();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn WdsTransportClientInitialize();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientInitializeSession();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientQueryStatus();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientRegisterCallback();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn WdsTransportClientReleaseBuffer();
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn WdsTransportClientShutdown();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientStartSession();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientWaitForCompletion();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerAllocateBuffer();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerCompleteRead();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerFreeBuffer();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerRegisterCallback();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerTrace();
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerTraceV();
}
