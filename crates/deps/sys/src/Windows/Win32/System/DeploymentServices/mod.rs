#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeAsyncRecvDone(hclientrequest: super::super::Foundation::HANDLE, action: u32) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpAppendOption(preplypacket: *mut ::core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32, boption: u8, boptionlen: u8, pvalue: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpAppendOptionRaw(preplypacket: *mut ::core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32, ubufferlen: u16, pbuffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpGetOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, uinstance: u32, boption: u8, pboptionlen: *mut u8, ppoptionvalue: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpGetVendorOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, boption: u8, uinstance: u32, pboptionlen: *mut u8, ppoptionvalue: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpInitialize(precvpacket: *const ::core::ffi::c_void, urecvpacketlen: u32, preplypacket: *mut ::core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeDhcpIsValid(ppacket: *const ::core::ffi::c_void, upacketlen: u32, brequestpacket: super::super::Foundation::BOOL, pbpxeoptionpresent: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpv6AppendOption(preply: *mut ::core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32, woptiontype: u16, cboption: u16, poption: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpv6AppendOptionRaw(preply: *mut ::core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32, cbbuffer: u16, pbuffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpv6CreateRelayRepl(prelaymessages: *const PXE_DHCPV6_NESTED_RELAY_MESSAGE, nrelaymessages: u32, pinnerpacket: *const u8, cbinnerpacket: u32, preplybuffer: *mut ::core::ffi::c_void, cbreplybuffer: u32, pcbreplybuffer: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpv6GetOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, uinstance: u32, woption: u16, pwoptionlen: *mut u16, ppoptionvalue: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpv6GetVendorOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, dwenterprisenumber: u32, woption: u16, uinstance: u32, pwoptionlen: *mut u16, ppoptionvalue: *mut *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpv6Initialize(prequest: *const ::core::ffi::c_void, cbrequest: u32, preply: *mut ::core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeDhcpv6IsValid(ppacket: *const ::core::ffi::c_void, upacketlen: u32, brequestpacket: super::super::Foundation::BOOL, pbpxeoptionpresent: *mut super::super::Foundation::BOOL) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeDhcpv6ParseRelayForw(prelayforwpacket: *const ::core::ffi::c_void, urelayforwpacketlen: u32, prelaymessages: *mut PXE_DHCPV6_NESTED_RELAY_MESSAGE, nrelaymessages: u32, pnrelaymessages: *mut u32, ppinnerpacket: *mut *mut u8, pcbinnerpacket: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeGetServerInfo(uinfotype: u32, pbuffer: *mut ::core::ffi::c_void, ubufferlen: u32) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn PxeGetServerInfoEx(uinfotype: u32, pbuffer: *mut ::core::ffi::c_void, ubufferlen: u32, pubufferused: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxePacketAllocate(hprovider: super::super::Foundation::HANDLE, hclientrequest: super::super::Foundation::HANDLE, usize: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxePacketFree(hprovider: super::super::Foundation::HANDLE, hclientrequest: super::super::Foundation::HANDLE, ppacket: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderEnumClose(henum: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderEnumFirst(phenum: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderEnumNext(henum: super::super::Foundation::HANDLE, ppprovider: *mut *mut PXE_PROVIDER) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderFreeInfo(pprovider: *const PXE_PROVIDER) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderQueryIndex(pszprovidername: super::super::Foundation::PWSTR, puindex: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn PxeProviderRegister(pszprovidername: super::super::Foundation::PWSTR, pszmodulepath: super::super::Foundation::PWSTR, index: u32, biscritical: super::super::Foundation::BOOL, phproviderkey: *mut super::Registry::HKEY) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderSetAttribute(hprovider: super::super::Foundation::HANDLE, attribute: u32, pparameterbuffer: *const ::core::ffi::c_void, uparamlen: u32) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderUnRegister(pszprovidername: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeRegisterCallback(hprovider: super::super::Foundation::HANDLE, callbacktype: u32, pcallbackfunction: *const ::core::ffi::c_void, pcontext: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeSendReply(hclientrequest: super::super::Foundation::HANDLE, ppacket: *const ::core::ffi::c_void, upacketlen: u32, paddress: *const PXE_ADDRESS) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeTrace(hprovider: super::super::Foundation::HANDLE, severity: u32, pszformat: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeTraceV(hprovider: super::super::Foundation::HANDLE, severity: u32, pszformat: super::super::Foundation::PWSTR, params: *const i8) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpAddOption(hhandle: super::super::Foundation::HANDLE, uoption: u32, uvaluelen: u32, pvalue: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpCloseHandle(hhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpGetOptionBuffer(hhandle: super::super::Foundation::HANDLE, ubufferlen: u32, pbuffer: *mut ::core::ffi::c_void, pubytes: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpInitialize(bpackettype: u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpParseInitialize(ppacket: *const ::core::ffi::c_void, upacketlen: u32, pbpackettype: *mut u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpParseInitializev6(ppacket: *const ::core::ffi::c_void, upacketlen: u32, pbpackettype: *mut u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpQueryOption(hhandle: super::super::Foundation::HANDLE, uoption: u32, uvaluelen: u32, pvalue: *mut ::core::ffi::c_void, pubytes: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliAuthorizeSession(hsession: super::super::Foundation::HANDLE, pcred: *const WDS_CLI_CRED) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliCancelTransfer(htransfer: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliClose(handle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliCreateSession(pwszserver: super::super::Foundation::PWSTR, pcred: *const WDS_CLI_CRED, phsession: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliFindFirstImage(hsession: super::super::Foundation::HANDLE, phfindhandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliFindNextImage(handle: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliFreeStringArray(ppwszarray: *mut super::super::Foundation::PWSTR, ulcount: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetDriverQueryXml(pwszwindirpath: super::super::Foundation::PWSTR, ppwszdriverquery: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetEnumerationFlags(handle: super::super::Foundation::HANDLE, pdwflags: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageArchitecture(hifh: super::super::Foundation::HANDLE, pdwvalue: *mut CPU_ARCHITECTURE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageDescription(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageFiles(hifh: super::super::Foundation::HANDLE, pppwszfiles: *mut *mut super::super::Foundation::PWSTR, pdwcount: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageGroup(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageHalName(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageHandleFromFindHandle(findhandle: super::super::Foundation::HANDLE, phimagehandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageHandleFromTransferHandle(htransfer: super::super::Foundation::HANDLE, phimagehandle: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageIndex(hifh: super::super::Foundation::HANDLE, pdwvalue: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageLanguage(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageLanguages(hifh: super::super::Foundation::HANDLE, pppszvalues: *mut *mut *mut i8, pdwnumvalues: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageLastModifiedTime(hifh: super::super::Foundation::HANDLE, ppsystimevalue: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageName(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageNamespace(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageParameter(hifh: super::super::Foundation::HANDLE, paramtype: WDS_CLI_IMAGE_PARAM_TYPE, presponse: *mut ::core::ffi::c_void, uresponselen: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImagePath(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageSize(hifh: super::super::Foundation::HANDLE, pullvalue: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageType(hifh: super::super::Foundation::HANDLE, pimagetype: *mut WDS_CLI_IMAGE_TYPE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageVersion(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetTransferSize(hifh: super::super::Foundation::HANDLE, pullvalue: *mut u64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliInitializeLog(hsession: super::super::Foundation::HANDLE, ulclientarchitecture: CPU_ARCHITECTURE, pwszclientid: super::super::Foundation::PWSTR, pwszclientaddress: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliLog(hsession: super::super::Foundation::HANDLE, ulloglevel: u32, ulmessagecode: u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliObtainDriverPackages(himage: super::super::Foundation::HANDLE, ppwszservername: *mut super::super::Foundation::PWSTR, pppwszdriverpackages: *mut *mut super::super::Foundation::PWSTR, pulcount: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliObtainDriverPackagesEx(hsession: super::super::Foundation::HANDLE, pwszmachineinfo: super::super::Foundation::PWSTR, ppwszservername: *mut super::super::Foundation::PWSTR, pppwszdriverpackages: *mut *mut super::super::Foundation::PWSTR, pulcount: *mut u32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliRegisterTrace(pfn: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn WdsCliSetTransferBufferSize(ulsizeinbytes: u32);
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliTransferFile(pwszserver: super::super::Foundation::PWSTR, pwsznamespace: super::super::Foundation::PWSTR, pwszremotefilepath: super::super::Foundation::PWSTR, pwszlocalfilepath: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: u32, pfnwdsclicallback: ::windows::runtime::RawPtr, pvuserdata: *const ::core::ffi::c_void, phtransfer: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliTransferImage(himage: super::super::Foundation::HANDLE, pwszlocalpath: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: u32, pfnwdsclicallback: ::windows::runtime::RawPtr, pvuserdata: *const ::core::ffi::c_void, phtransfer: *mut super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliWaitForTransfer(htransfer: super::super::Foundation::HANDLE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn WdsTransportClientAddRefBuffer(pvbuffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCancelSession(hsessionkey: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCancelSessionEx(hsessionkey: super::super::Foundation::HANDLE, dwerrorcode: u32) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCloseSession(hsessionkey: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCompleteReceive(hsessionkey: super::super::Foundation::HANDLE, ulsize: u32, pulloffset: *const u64) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn WdsTransportClientInitialize() -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientInitializeSession(psessionrequest: *const WDS_TRANSPORTCLIENT_REQUEST, pcallerdata: *const ::core::ffi::c_void, hsessionkey: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientQueryStatus(hsessionkey: super::super::Foundation::HANDLE, pustatus: *mut u32, puerrorcode: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientRegisterCallback(hsessionkey: super::super::Foundation::HANDLE, callbackid: TRANSPORTCLIENT_CALLBACK_ID, pfncallback: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn WdsTransportClientReleaseBuffer(pvbuffer: *const ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`*"]
    pub fn WdsTransportClientShutdown() -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientStartSession(hsessionkey: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientWaitForCompletion(hsessionkey: super::super::Foundation::HANDLE, utimeout: u32) -> u32;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerAllocateBuffer(hprovider: super::super::Foundation::HANDLE, ulbuffersize: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerCompleteRead(hprovider: super::super::Foundation::HANDLE, ulbytesread: u32, pvuserdata: *const ::core::ffi::c_void, hreadresult: ::windows::runtime::HRESULT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerFreeBuffer(hprovider: super::super::Foundation::HANDLE, pvbuffer: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerRegisterCallback(hprovider: super::super::Foundation::HANDLE, callbackid: TRANSPORTPROVIDER_CALLBACK_ID, pfncallback: *const ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerTrace(hprovider: super::super::Foundation::HANDLE, severity: u32, pwszformat: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_System_DeploymentServices`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerTraceV(hprovider: super::super::Foundation::HANDLE, severity: u32, pwszformat: super::super::Foundation::PWSTR, params: *const i8) -> ::windows::runtime::HRESULT;
}
