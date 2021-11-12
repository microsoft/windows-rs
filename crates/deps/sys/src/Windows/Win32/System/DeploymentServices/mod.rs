#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeAsyncRecvDone(hclientrequest: super::super::Foundation::HANDLE, action: u32) -> u32;
    pub fn PxeDhcpAppendOption(preplypacket: *mut ::core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32, boption: u8, boptionlen: u8, pvalue: *const ::core::ffi::c_void) -> u32;
    pub fn PxeDhcpAppendOptionRaw(preplypacket: *mut ::core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32, ubufferlen: u16, pbuffer: *const ::core::ffi::c_void) -> u32;
    pub fn PxeDhcpGetOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, uinstance: u32, boption: u8, pboptionlen: *mut u8, ppoptionvalue: *mut *mut ::core::ffi::c_void) -> u32;
    pub fn PxeDhcpGetVendorOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, boption: u8, uinstance: u32, pboptionlen: *mut u8, ppoptionvalue: *mut *mut ::core::ffi::c_void) -> u32;
    pub fn PxeDhcpInitialize(precvpacket: *const ::core::ffi::c_void, urecvpacketlen: u32, preplypacket: *mut ::core::ffi::c_void, umaxreplypacketlen: u32, pureplypacketlen: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeDhcpIsValid(ppacket: *const ::core::ffi::c_void, upacketlen: u32, brequestpacket: super::super::Foundation::BOOL, pbpxeoptionpresent: *mut super::super::Foundation::BOOL) -> u32;
    pub fn PxeDhcpv6AppendOption(preply: *mut ::core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32, woptiontype: u16, cboption: u16, poption: *const ::core::ffi::c_void) -> u32;
    pub fn PxeDhcpv6AppendOptionRaw(preply: *mut ::core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32, cbbuffer: u16, pbuffer: *const ::core::ffi::c_void) -> u32;
    pub fn PxeDhcpv6CreateRelayRepl(prelaymessages: *const PXE_DHCPV6_NESTED_RELAY_MESSAGE, nrelaymessages: u32, pinnerpacket: *const u8, cbinnerpacket: u32, preplybuffer: *mut ::core::ffi::c_void, cbreplybuffer: u32, pcbreplybuffer: *mut u32) -> u32;
    pub fn PxeDhcpv6GetOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, uinstance: u32, woption: u16, pwoptionlen: *mut u16, ppoptionvalue: *mut *mut ::core::ffi::c_void) -> u32;
    pub fn PxeDhcpv6GetVendorOptionValue(ppacket: *const ::core::ffi::c_void, upacketlen: u32, dwenterprisenumber: u32, woption: u16, uinstance: u32, pwoptionlen: *mut u16, ppoptionvalue: *mut *mut ::core::ffi::c_void) -> u32;
    pub fn PxeDhcpv6Initialize(prequest: *const ::core::ffi::c_void, cbrequest: u32, preply: *mut ::core::ffi::c_void, cbreply: u32, pcbreplyused: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeDhcpv6IsValid(ppacket: *const ::core::ffi::c_void, upacketlen: u32, brequestpacket: super::super::Foundation::BOOL, pbpxeoptionpresent: *mut super::super::Foundation::BOOL) -> u32;
    pub fn PxeDhcpv6ParseRelayForw(prelayforwpacket: *const ::core::ffi::c_void, urelayforwpacketlen: u32, prelaymessages: *mut PXE_DHCPV6_NESTED_RELAY_MESSAGE, nrelaymessages: u32, pnrelaymessages: *mut u32, ppinnerpacket: *mut *mut u8, pcbinnerpacket: *mut u32) -> u32;
    pub fn PxeGetServerInfo(uinfotype: u32, pbuffer: *mut ::core::ffi::c_void, ubufferlen: u32) -> u32;
    pub fn PxeGetServerInfoEx(uinfotype: u32, pbuffer: *mut ::core::ffi::c_void, ubufferlen: u32, pubufferused: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxePacketAllocate(hprovider: super::super::Foundation::HANDLE, hclientrequest: super::super::Foundation::HANDLE, usize: u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxePacketFree(hprovider: super::super::Foundation::HANDLE, hclientrequest: super::super::Foundation::HANDLE, ppacket: *const ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderEnumClose(henum: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderEnumFirst(phenum: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderEnumNext(henum: super::super::Foundation::HANDLE, ppprovider: *mut *mut PXE_PROVIDER) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderFreeInfo(pprovider: *const PXE_PROVIDER) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderQueryIndex(pszprovidername: super::super::Foundation::PWSTR, puindex: *mut u32) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn PxeProviderRegister(pszprovidername: super::super::Foundation::PWSTR, pszmodulepath: super::super::Foundation::PWSTR, index: u32, biscritical: super::super::Foundation::BOOL, phproviderkey: *mut super::Registry::HKEY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderSetAttribute(hprovider: super::super::Foundation::HANDLE, attribute: u32, pparameterbuffer: *const ::core::ffi::c_void, uparamlen: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeProviderUnRegister(pszprovidername: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeRegisterCallback(hprovider: super::super::Foundation::HANDLE, callbacktype: u32, pcallbackfunction: *const ::core::ffi::c_void, pcontext: *const ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeSendReply(hclientrequest: super::super::Foundation::HANDLE, ppacket: *const ::core::ffi::c_void, upacketlen: u32, paddress: *const PXE_ADDRESS) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeTrace(hprovider: super::super::Foundation::HANDLE, severity: u32, pszformat: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn PxeTraceV(hprovider: super::super::Foundation::HANDLE, severity: u32, pszformat: super::super::Foundation::PWSTR, params: *const i8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpAddOption(hhandle: super::super::Foundation::HANDLE, uoption: u32, uvaluelen: u32, pvalue: *const ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpCloseHandle(hhandle: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpGetOptionBuffer(hhandle: super::super::Foundation::HANDLE, ubufferlen: u32, pbuffer: *mut ::core::ffi::c_void, pubytes: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpInitialize(bpackettype: u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpParseInitialize(ppacket: *const ::core::ffi::c_void, upacketlen: u32, pbpackettype: *mut u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpParseInitializev6(ppacket: *const ::core::ffi::c_void, upacketlen: u32, pbpackettype: *mut u8, phhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsBpQueryOption(hhandle: super::super::Foundation::HANDLE, uoption: u32, uvaluelen: u32, pvalue: *mut ::core::ffi::c_void, pubytes: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliAuthorizeSession(hsession: super::super::Foundation::HANDLE, pcred: *const WDS_CLI_CRED) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliCancelTransfer(htransfer: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliClose(handle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliCreateSession(pwszserver: super::super::Foundation::PWSTR, pcred: *const WDS_CLI_CRED, phsession: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliFindFirstImage(hsession: super::super::Foundation::HANDLE, phfindhandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliFindNextImage(handle: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliFreeStringArray(ppwszarray: *mut super::super::Foundation::PWSTR, ulcount: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetDriverQueryXml(pwszwindirpath: super::super::Foundation::PWSTR, ppwszdriverquery: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetEnumerationFlags(handle: super::super::Foundation::HANDLE, pdwflags: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageArchitecture(hifh: super::super::Foundation::HANDLE, pdwvalue: *mut CPU_ARCHITECTURE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageDescription(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageFiles(hifh: super::super::Foundation::HANDLE, pppwszfiles: *mut *mut super::super::Foundation::PWSTR, pdwcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageGroup(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageHalName(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageHandleFromFindHandle(findhandle: super::super::Foundation::HANDLE, phimagehandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageHandleFromTransferHandle(htransfer: super::super::Foundation::HANDLE, phimagehandle: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageIndex(hifh: super::super::Foundation::HANDLE, pdwvalue: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageLanguage(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageLanguages(hifh: super::super::Foundation::HANDLE, pppszvalues: *mut *mut *mut i8, pdwnumvalues: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageLastModifiedTime(hifh: super::super::Foundation::HANDLE, ppsystimevalue: *mut *mut super::super::Foundation::SYSTEMTIME) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageName(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageNamespace(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageParameter(hifh: super::super::Foundation::HANDLE, paramtype: WDS_CLI_IMAGE_PARAM_TYPE, presponse: *mut ::core::ffi::c_void, uresponselen: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImagePath(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageSize(hifh: super::super::Foundation::HANDLE, pullvalue: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageType(hifh: super::super::Foundation::HANDLE, pimagetype: *mut WDS_CLI_IMAGE_TYPE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetImageVersion(hifh: super::super::Foundation::HANDLE, ppwszvalue: *mut super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliGetTransferSize(hifh: super::super::Foundation::HANDLE, pullvalue: *mut u64) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliInitializeLog(hsession: super::super::Foundation::HANDLE, ulclientarchitecture: CPU_ARCHITECTURE, pwszclientid: super::super::Foundation::PWSTR, pwszclientaddress: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliLog(hsession: super::super::Foundation::HANDLE, ulloglevel: u32, ulmessagecode: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliObtainDriverPackages(himage: super::super::Foundation::HANDLE, ppwszservername: *mut super::super::Foundation::PWSTR, pppwszdriverpackages: *mut *mut super::super::Foundation::PWSTR, pulcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliObtainDriverPackagesEx(hsession: super::super::Foundation::HANDLE, pwszmachineinfo: super::super::Foundation::PWSTR, ppwszservername: *mut super::super::Foundation::PWSTR, pppwszdriverpackages: *mut *mut super::super::Foundation::PWSTR, pulcount: *mut u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliRegisterTrace(pfn: PFN_WdsCliTraceFunction) -> ::windows_sys::core::HRESULT;
    pub fn WdsCliSetTransferBufferSize(ulsizeinbytes: u32);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliTransferFile(pwszserver: super::super::Foundation::PWSTR, pwsznamespace: super::super::Foundation::PWSTR, pwszremotefilepath: super::super::Foundation::PWSTR, pwszlocalfilepath: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: u32, pfnwdsclicallback: PFN_WdsCliCallback, pvuserdata: *const ::core::ffi::c_void, phtransfer: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliTransferImage(himage: super::super::Foundation::HANDLE, pwszlocalpath: super::super::Foundation::PWSTR, dwflags: u32, dwreserved: u32, pfnwdsclicallback: PFN_WdsCliCallback, pvuserdata: *const ::core::ffi::c_void, phtransfer: *mut super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsCliWaitForTransfer(htransfer: super::super::Foundation::HANDLE) -> ::windows_sys::core::HRESULT;
    pub fn WdsTransportClientAddRefBuffer(pvbuffer: *const ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCancelSession(hsessionkey: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCancelSessionEx(hsessionkey: super::super::Foundation::HANDLE, dwerrorcode: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCloseSession(hsessionkey: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientCompleteReceive(hsessionkey: super::super::Foundation::HANDLE, ulsize: u32, pulloffset: *const u64) -> u32;
    pub fn WdsTransportClientInitialize() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientInitializeSession(psessionrequest: *const WDS_TRANSPORTCLIENT_REQUEST, pcallerdata: *const ::core::ffi::c_void, hsessionkey: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientQueryStatus(hsessionkey: super::super::Foundation::HANDLE, pustatus: *mut u32, puerrorcode: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientRegisterCallback(hsessionkey: super::super::Foundation::HANDLE, callbackid: TRANSPORTCLIENT_CALLBACK_ID, pfncallback: *const ::core::ffi::c_void) -> u32;
    pub fn WdsTransportClientReleaseBuffer(pvbuffer: *const ::core::ffi::c_void) -> u32;
    pub fn WdsTransportClientShutdown() -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientStartSession(hsessionkey: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportClientWaitForCompletion(hsessionkey: super::super::Foundation::HANDLE, utimeout: u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerAllocateBuffer(hprovider: super::super::Foundation::HANDLE, ulbuffersize: u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerCompleteRead(hprovider: super::super::Foundation::HANDLE, ulbytesread: u32, pvuserdata: *const ::core::ffi::c_void, hreadresult: ::windows_sys::core::HRESULT) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerFreeBuffer(hprovider: super::super::Foundation::HANDLE, pvbuffer: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerRegisterCallback(hprovider: super::super::Foundation::HANDLE, callbackid: TRANSPORTPROVIDER_CALLBACK_ID, pfncallback: *const ::core::ffi::c_void) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerTrace(hprovider: super::super::Foundation::HANDLE, severity: u32, pwszformat: super::super::Foundation::PWSTR) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WdsTransportServerTraceV(hprovider: super::super::Foundation::HANDLE, severity: u32, pwszformat: super::super::Foundation::PWSTR, params: *const i8) -> ::windows_sys::core::HRESULT;
}
#[repr(C)]
pub struct CPU_ARCHITECTURE(i32);
pub const EVT_WDSMCS_E_CP_CALLBACKS_NOT_REG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801324i32 as _);
pub const EVT_WDSMCS_E_CP_CLOSE_INSTANCE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801320i32 as _);
pub const EVT_WDSMCS_E_CP_DLL_LOAD_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801328i32 as _);
pub const EVT_WDSMCS_E_CP_DLL_LOAD_FAILED_CRITICAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801317i32 as _);
pub const EVT_WDSMCS_E_CP_INCOMPATIBLE_SERVER_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801325i32 as _);
pub const EVT_WDSMCS_E_CP_INIT_FUNC_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801326i32 as _);
pub const EVT_WDSMCS_E_CP_INIT_FUNC_MISSING: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801327i32 as _);
pub const EVT_WDSMCS_E_CP_MEMORY_LEAK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801322i32 as _);
pub const EVT_WDSMCS_E_CP_OPEN_CONTENT_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801319i32 as _);
pub const EVT_WDSMCS_E_CP_OPEN_INSTANCE_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801321i32 as _);
pub const EVT_WDSMCS_E_CP_SHUTDOWN_FUNC_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801323i32 as _);
pub const EVT_WDSMCS_E_DUPLICATE_MULTICAST_ADDR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801406i32 as _);
pub const EVT_WDSMCS_E_NON_WDS_DUPLICATE_MULTICAST_ADDR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801405i32 as _);
pub const EVT_WDSMCS_E_NSREG_CONTENT_PROVIDER_NOT_REG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801151i32 as _);
pub const EVT_WDSMCS_E_NSREG_FAILURE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801149i32 as _);
pub const EVT_WDSMCS_E_NSREG_NAMESPACE_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801150i32 as _);
pub const EVT_WDSMCS_E_NSREG_START_TIME_IN_PAST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801152i32 as _);
pub const EVT_WDSMCS_E_PARAMETERS_READ_FAILED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801407i32 as _);
pub const EVT_WDSMCS_S_PARAMETERS_READ: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1092682240i32 as _);
pub const EVT_WDSMCS_W_CP_DLL_LOAD_FAILED_NOT_CRITICAL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-2128543142i32 as _);
pub const FACILITY_WDSMCCLIENT: u32 = 290u32;
pub const FACILITY_WDSMCSERVER: u32 = 289u32;
pub const FACILITY_WDSTPTMGMT: u32 = 272u32;
#[repr(transparent)]
pub struct IWdsTransportCacheable(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportConfigurationManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportConfigurationManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportContent(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportContentProvider(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportDiagnosticsPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportMulticastSessionPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportNamespace(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportNamespaceAutoCast(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportNamespaceManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportNamespaceScheduledCast(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportNamespaceScheduledCastAutoStart(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportNamespaceScheduledCastManualStart(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportServer(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportServer2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportServicePolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportServicePolicy2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportSession(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportSetupManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportSetupManager2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportTftpClient(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IWdsTransportTftpManager(pub *mut ::core::ffi::c_void);
pub const MC_SERVER_CURRENT_VERSION: u32 = 1u32;
#[repr(C)]
pub struct PFN_WDS_CLI_CALLBACK_MESSAGE_ID(i32);
#[repr(C)]
pub struct PFN_WdsCliCallback(i32);
#[repr(C)]
pub struct PFN_WdsCliTraceFunction(i32);
#[repr(C)]
pub struct PFN_WdsTransportClientReceiveContents(i32);
#[repr(C)]
pub struct PFN_WdsTransportClientReceiveMetadata(i32);
#[repr(C)]
pub struct PFN_WdsTransportClientSessionComplete(i32);
#[repr(C)]
pub struct PFN_WdsTransportClientSessionNegotiate(i32);
#[repr(C)]
pub struct PFN_WdsTransportClientSessionStart(i32);
#[repr(C)]
pub struct PFN_WdsTransportClientSessionStartEx(i32);
#[repr(C)]
pub struct PXE_ADDRESS(i32);
pub const PXE_ADDR_BROADCAST: u32 = 1u32;
pub const PXE_ADDR_USE_ADDR: u32 = 4u32;
pub const PXE_ADDR_USE_DHCP_RULES: u32 = 8u32;
pub const PXE_ADDR_USE_PORT: u32 = 2u32;
pub const PXE_BA_CUSTOM: u32 = 2u32;
pub const PXE_BA_IGNORE: u32 = 3u32;
pub const PXE_BA_NBP: u32 = 1u32;
pub const PXE_BA_REJECTED: u32 = 4u32;
pub const PXE_CALLBACK_MAX: u32 = 3u32;
pub const PXE_CALLBACK_RECV_REQUEST: u32 = 0u32;
pub const PXE_CALLBACK_SERVICE_CONTROL: u32 = 2u32;
pub const PXE_CALLBACK_SHUTDOWN: u32 = 1u32;
pub const PXE_DHCPV6_CLIENT_PORT: u32 = 546u32;
#[repr(C)]
pub struct PXE_DHCPV6_MESSAGE(i32);
#[repr(C)]
pub struct PXE_DHCPV6_MESSAGE_HEADER(i32);
#[repr(C)]
pub struct PXE_DHCPV6_NESTED_RELAY_MESSAGE(i32);
#[repr(C)]
pub struct PXE_DHCPV6_OPTION(i32);
pub const PXE_DHCPV6_RELAY_HOP_COUNT_LIMIT: u32 = 32u32;
#[repr(C)]
pub struct PXE_DHCPV6_RELAY_MESSAGE(i32);
pub const PXE_DHCPV6_SERVER_PORT: u32 = 547u32;
pub const PXE_DHCP_CLIENT_PORT: u32 = 68u32;
pub const PXE_DHCP_FILE_SIZE: u32 = 128u32;
pub const PXE_DHCP_HWAADR_SIZE: u32 = 16u32;
pub const PXE_DHCP_MAGIC_COOKIE_SIZE: u32 = 4u32;
#[repr(C)]
pub struct PXE_DHCP_MESSAGE(i32);
#[repr(C)]
pub struct PXE_DHCP_OPTION(i32);
pub const PXE_DHCP_SERVER_PORT: u32 = 67u32;
pub const PXE_DHCP_SERVER_SIZE: u32 = 64u32;
pub const PXE_GSI_SERVER_DUID: u32 = 2u32;
pub const PXE_GSI_TRACE_ENABLED: u32 = 1u32;
pub const PXE_MAX_ADDRESS: u32 = 16u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct PXE_PROVIDER(i32);
pub const PXE_PROV_ATTR_FILTER: u32 = 0u32;
pub const PXE_PROV_ATTR_FILTER_IPV6: u32 = 1u32;
pub const PXE_PROV_ATTR_IPV6_CAPABLE: u32 = 2u32;
pub const PXE_PROV_FILTER_ALL: u32 = 0u32;
pub const PXE_PROV_FILTER_DHCP_ONLY: u32 = 1u32;
pub const PXE_PROV_FILTER_PXE_ONLY: u32 = 2u32;
pub const PXE_REG_INDEX_BOTTOM: u32 = 4294967295u32;
pub const PXE_REG_INDEX_TOP: u32 = 0u32;
pub const PXE_SERVER_PORT: u32 = 4011u32;
pub const PXE_TRACE_ERROR: u32 = 524288u32;
pub const PXE_TRACE_FATAL: u32 = 1048576u32;
pub const PXE_TRACE_INFO: u32 = 131072u32;
pub const PXE_TRACE_VERBOSE: u32 = 65536u32;
pub const PXE_TRACE_WARNING: u32 = 262144u32;
#[repr(C)]
pub struct TRANSPORTCLIENT_CALLBACK_ID(i32);
#[repr(C)]
pub struct TRANSPORTCLIENT_SESSION_INFO(i32);
#[repr(C)]
pub struct TRANSPORTPROVIDER_CALLBACK_ID(i32);
pub const TRANSPORTPROVIDER_CURRENT_VERSION: u32 = 1u32;
pub const WDSBP_OPTVAL_ACTION_ABORT: u32 = 5u32;
pub const WDSBP_OPTVAL_ACTION_APPROVAL: u32 = 1u32;
pub const WDSBP_OPTVAL_ACTION_REFERRAL: u32 = 3u32;
pub const WDSBP_OPTVAL_NBP_VER_7: u32 = 1792u32;
pub const WDSBP_OPTVAL_NBP_VER_8: u32 = 2048u32;
pub const WDSBP_OPTVAL_PXE_PROMPT_NOPROMPT: u32 = 2u32;
pub const WDSBP_OPTVAL_PXE_PROMPT_OPTIN: u32 = 1u32;
pub const WDSBP_OPTVAL_PXE_PROMPT_OPTOUT: u32 = 3u32;
pub const WDSBP_OPT_TYPE_BYTE: u32 = 1u32;
pub const WDSBP_OPT_TYPE_IP4: u32 = 6u32;
pub const WDSBP_OPT_TYPE_IP6: u32 = 7u32;
pub const WDSBP_OPT_TYPE_NONE: u32 = 0u32;
pub const WDSBP_OPT_TYPE_STR: u32 = 5u32;
pub const WDSBP_OPT_TYPE_ULONG: u32 = 3u32;
pub const WDSBP_OPT_TYPE_USHORT: u32 = 2u32;
pub const WDSBP_OPT_TYPE_WSTR: u32 = 4u32;
pub const WDSBP_PK_TYPE_BCD: u32 = 4u32;
pub const WDSBP_PK_TYPE_DHCP: u32 = 1u32;
pub const WDSBP_PK_TYPE_DHCPV6: u32 = 8u32;
pub const WDSBP_PK_TYPE_WDSNBP: u32 = 2u32;
pub const WDSMCCLIENT_CATEGORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2i32 as _);
pub const WDSMCSERVER_CATEGORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1i32 as _);
pub const WDSMCS_E_CLIENT_DOESNOT_SUPPORT_SECURITY_MODE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801648i32 as _);
pub const WDSMCS_E_CLIENT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801660i32 as _);
pub const WDSMCS_E_CONTENT_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801661i32 as _);
pub const WDSMCS_E_CONTENT_PROVIDER_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801658i32 as _);
pub const WDSMCS_E_INCOMPATIBLE_VERSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801662i32 as _);
pub const WDSMCS_E_NAMESPACE_ALREADY_EXISTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801657i32 as _);
pub const WDSMCS_E_NAMESPACE_ALREADY_STARTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801655i32 as _);
pub const WDSMCS_E_NAMESPACE_NOT_FOUND: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801659i32 as _);
pub const WDSMCS_E_NAMESPACE_SHUTDOWN_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801656i32 as _);
pub const WDSMCS_E_NS_START_FAILED_NO_CLIENTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801654i32 as _);
pub const WDSMCS_E_PACKET_HAS_SECURITY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801650i32 as _);
pub const WDSMCS_E_PACKET_NOT_CHECKSUMED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801649i32 as _);
pub const WDSMCS_E_PACKET_NOT_HASHED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801652i32 as _);
pub const WDSMCS_E_PACKET_NOT_SIGNED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801651i32 as _);
pub const WDSMCS_E_REQCALLBACKS_NOT_REG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801663i32 as _);
pub const WDSMCS_E_SESSION_SHUTDOWN_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801664i32 as _);
pub const WDSMCS_E_START_TIME_IN_PAST: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054801653i32 as _);
pub const WDSTPC_E_ALREADY_COMPLETED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054735615i32 as _);
pub const WDSTPC_E_ALREADY_IN_LOWEST_SESSION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054735606i32 as _);
pub const WDSTPC_E_ALREADY_IN_PROGRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054735614i32 as _);
pub const WDSTPC_E_CALLBACKS_NOT_REG: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054735616i32 as _);
pub const WDSTPC_E_CLIENT_DEMOTE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054735605i32 as _);
pub const WDSTPC_E_KICKED_FAIL: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054735609i32 as _);
pub const WDSTPC_E_KICKED_FALLBACK: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054735610i32 as _);
pub const WDSTPC_E_KICKED_POLICY_NOT_MET: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054735611i32 as _);
pub const WDSTPC_E_KICKED_UNKNOWN: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054735608i32 as _);
pub const WDSTPC_E_MULTISTREAM_NOT_ENABLED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054735607i32 as _);
pub const WDSTPC_E_NOT_INITIALIZED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054735612i32 as _);
pub const WDSTPC_E_NO_IP4_INTERFACE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054735604i32 as _);
pub const WDSTPC_E_UNKNOWN_ERROR: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054735613i32 as _);
pub const WDSTPTC_E_WIM_APPLY_REQUIRES_REFERENCE_IMAGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1054735603i32 as _);
pub const WDSTPTMGMT_CATEGORY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(1i32 as _);
pub const WDSTPTMGMT_E_CANNOT_REFRESH_DIRTY_OBJECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915761i32 as _);
pub const WDSTPTMGMT_E_CANNOT_REINITIALIZE_OBJECT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915767i32 as _);
pub const WDSTPTMGMT_E_CONTENT_PROVIDER_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915773i32 as _);
pub const WDSTPTMGMT_E_CONTENT_PROVIDER_NOT_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915772i32 as _);
pub const WDSTPTMGMT_E_INVALID_AUTO_DISCONNECT_THRESHOLD: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915748i32 as _);
pub const WDSTPTMGMT_E_INVALID_CLASS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915774i32 as _);
pub const WDSTPTMGMT_E_INVALID_CONTENT_PROVIDER_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915771i32 as _);
pub const WDSTPTMGMT_E_INVALID_DIAGNOSTICS_COMPONENTS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915762i32 as _);
pub const WDSTPTMGMT_E_INVALID_IPV4_MULTICAST_ADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915753i32 as _);
pub const WDSTPTMGMT_E_INVALID_IPV6_MULTICAST_ADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915752i32 as _);
pub const WDSTPTMGMT_E_INVALID_IPV6_MULTICAST_ADDRESS_SOURCE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915750i32 as _);
pub const WDSTPTMGMT_E_INVALID_IP_ADDRESS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915754i32 as _);
pub const WDSTPTMGMT_E_INVALID_MULTISTREAM_STREAM_COUNT: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915749i32 as _);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_DATA: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915765i32 as _);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_NAME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915766i32 as _);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_START_PARAMETERS: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915758i32 as _);
pub const WDSTPTMGMT_E_INVALID_NAMESPACE_START_TIME: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915763i32 as _);
pub const WDSTPTMGMT_E_INVALID_OPERATION: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915775i32 as _);
pub const WDSTPTMGMT_E_INVALID_PROPERTY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915776i32 as _);
pub const WDSTPTMGMT_E_INVALID_SERVICE_IP_ADDRESS_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915760i32 as _);
pub const WDSTPTMGMT_E_INVALID_SERVICE_PORT_RANGE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915759i32 as _);
pub const WDSTPTMGMT_E_INVALID_SLOW_CLIENT_HANDLING_TYPE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915746i32 as _);
pub const WDSTPTMGMT_E_INVALID_TFTP_MAX_BLOCKSIZE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915741i32 as _);
pub const WDSTPTMGMT_E_IPV6_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915751i32 as _);
pub const WDSTPTMGMT_E_MULTICAST_SESSION_POLICY_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915747i32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_ALREADY_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915769i32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_NOT_ON_SERVER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915756i32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_NOT_REGISTERED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915768i32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_READ_ONLY: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915764i32 as _);
pub const WDSTPTMGMT_E_NAMESPACE_REMOVED_FROM_SERVER: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915755i32 as _);
pub const WDSTPTMGMT_E_NETWORK_PROFILES_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915745i32 as _);
pub const WDSTPTMGMT_E_TFTP_MAX_BLOCKSIZE_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915743i32 as _);
pub const WDSTPTMGMT_E_TFTP_VAR_WINDOW_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915742i32 as _);
pub const WDSTPTMGMT_E_TRANSPORT_SERVER_ROLE_NOT_CONFIGURED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915770i32 as _);
pub const WDSTPTMGMT_E_TRANSPORT_SERVER_UNAVAILABLE: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915757i32 as _);
pub const WDSTPTMGMT_E_UDP_PORT_POLICY_NOT_SUPPORTED: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(-1055915744i32 as _);
#[repr(C)]
pub struct WDSTRANSPORT_DIAGNOSTICS_COMPONENT_FLAGS(i32);
#[repr(C)]
pub struct WDSTRANSPORT_DISCONNECT_TYPE(i32);
#[repr(C)]
pub struct WDSTRANSPORT_FEATURE_FLAGS(i32);
#[repr(C)]
pub struct WDSTRANSPORT_IP_ADDRESS_SOURCE_TYPE(i32);
#[repr(C)]
pub struct WDSTRANSPORT_IP_ADDRESS_TYPE(i32);
#[repr(C)]
pub struct WDSTRANSPORT_NAMESPACE_TYPE(i32);
#[repr(C)]
pub struct WDSTRANSPORT_NETWORK_PROFILE_TYPE(i32);
#[repr(C)]
pub struct WDSTRANSPORT_PROTOCOL_FLAGS(i32);
pub const WDSTRANSPORT_RESOURCE_UTILIZATION_UNKNOWN: u32 = 255u32;
#[repr(C)]
pub struct WDSTRANSPORT_SERVICE_NOTIFICATION(i32);
#[repr(C)]
pub struct WDSTRANSPORT_SLOW_CLIENT_HANDLING_TYPE(i32);
#[repr(C)]
pub struct WDSTRANSPORT_TFTP_CAPABILITY(i32);
#[repr(C)]
pub struct WDSTRANSPORT_UDP_PORT_POLICY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WDS_CLI_CRED(i32);
#[repr(C)]
pub struct WDS_CLI_FIRMWARE_TYPE(i32);
#[repr(C)]
pub struct WDS_CLI_IMAGE_PARAM_TYPE(i32);
#[repr(C)]
pub struct WDS_CLI_IMAGE_TYPE(i32);
pub const WDS_CLI_NO_SPARSE_FILE: u32 = 2u32;
pub const WDS_CLI_TRANSFER_ASYNCHRONOUS: u32 = 1u32;
pub const WDS_LOG_LEVEL_DISABLED: i32 = 0i32;
pub const WDS_LOG_LEVEL_ERROR: i32 = 1i32;
pub const WDS_LOG_LEVEL_INFO: i32 = 3i32;
pub const WDS_LOG_LEVEL_WARNING: i32 = 2i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_FINISHED: i32 = 6i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_FINISHED_2: i32 = 16i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_STARTED: i32 = 5i32;
pub const WDS_LOG_TYPE_CLIENT_APPLY_STARTED_2: i32 = 15i32;
pub const WDS_LOG_TYPE_CLIENT_DOMAINJOINERROR: i32 = 12i32;
pub const WDS_LOG_TYPE_CLIENT_DOMAINJOINERROR_2: i32 = 17i32;
pub const WDS_LOG_TYPE_CLIENT_DRIVER_PACKAGE_NOT_ACCESSIBLE: i32 = 18i32;
pub const WDS_LOG_TYPE_CLIENT_ERROR: i32 = 1i32;
pub const WDS_LOG_TYPE_CLIENT_FINISHED: i32 = 3i32;
pub const WDS_LOG_TYPE_CLIENT_GENERIC_MESSAGE: i32 = 7i32;
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED: i32 = 4i32;
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED2: i32 = 22i32;
pub const WDS_LOG_TYPE_CLIENT_IMAGE_SELECTED3: i32 = 23i32;
pub const WDS_LOG_TYPE_CLIENT_MAX_CODE: i32 = 24i32;
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_END: i32 = 20i32;
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_FAILURE: i32 = 21i32;
pub const WDS_LOG_TYPE_CLIENT_OFFLINE_DRIVER_INJECTION_START: i32 = 19i32;
pub const WDS_LOG_TYPE_CLIENT_POST_ACTIONS_END: i32 = 14i32;
pub const WDS_LOG_TYPE_CLIENT_POST_ACTIONS_START: i32 = 13i32;
pub const WDS_LOG_TYPE_CLIENT_STARTED: i32 = 2i32;
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_DOWNGRADE: i32 = 11i32;
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_END: i32 = 10i32;
pub const WDS_LOG_TYPE_CLIENT_TRANSFER_START: i32 = 9i32;
pub const WDS_LOG_TYPE_CLIENT_UNATTEND_MODE: i32 = 8i32;
pub const WDS_MC_TRACE_ERROR: u32 = 524288u32;
pub const WDS_MC_TRACE_FATAL: u32 = 1048576u32;
pub const WDS_MC_TRACE_INFO: u32 = 131072u32;
pub const WDS_MC_TRACE_VERBOSE: u32 = 65536u32;
pub const WDS_MC_TRACE_WARNING: u32 = 262144u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WDS_TRANSPORTCLIENT_CALLBACKS(i32);
pub const WDS_TRANSPORTCLIENT_CURRENT_API_VERSION: u32 = 1u32;
pub const WDS_TRANSPORTCLIENT_NO_CACHE: u32 = 0u32;
pub const WDS_TRANSPORTCLIENT_PROTOCOL_MULTICAST: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WDS_TRANSPORTCLIENT_REQUEST(i32);
#[repr(C)]
pub struct WDS_TRANSPORTCLIENT_REQUEST_AUTH_LEVEL(i32);
pub const WDS_TRANSPORTCLIENT_STATUS_FAILURE: u32 = 3u32;
pub const WDS_TRANSPORTCLIENT_STATUS_IN_PROGRESS: u32 = 1u32;
pub const WDS_TRANSPORTCLIENT_STATUS_SUCCESS: u32 = 2u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
#[repr(C)]
pub struct WDS_TRANSPORTPROVIDER_INIT_PARAMS(i32);
#[repr(C)]
pub struct WDS_TRANSPORTPROVIDER_SETTINGS(i32);
pub const WdsCliFlagEnumFilterFirmware: i32 = 2i32;
pub const WdsCliFlagEnumFilterVersion: i32 = 1i32;
#[repr(C)]
pub struct WdsTransportCacheable(i32);
#[repr(C)]
pub struct WdsTransportClient(i32);
#[repr(C)]
pub struct WdsTransportCollection(i32);
#[repr(C)]
pub struct WdsTransportConfigurationManager(i32);
#[repr(C)]
pub struct WdsTransportContent(i32);
#[repr(C)]
pub struct WdsTransportContentProvider(i32);
#[repr(C)]
pub struct WdsTransportDiagnosticsPolicy(i32);
#[repr(C)]
pub struct WdsTransportManager(i32);
#[repr(C)]
pub struct WdsTransportMulticastSessionPolicy(i32);
#[repr(C)]
pub struct WdsTransportNamespace(i32);
#[repr(C)]
pub struct WdsTransportNamespaceAutoCast(i32);
#[repr(C)]
pub struct WdsTransportNamespaceManager(i32);
#[repr(C)]
pub struct WdsTransportNamespaceScheduledCast(i32);
#[repr(C)]
pub struct WdsTransportNamespaceScheduledCastAutoStart(i32);
#[repr(C)]
pub struct WdsTransportNamespaceScheduledCastManualStart(i32);
#[repr(C)]
pub struct WdsTransportServer(i32);
#[repr(C)]
pub struct WdsTransportServicePolicy(i32);
#[repr(C)]
pub struct WdsTransportSession(i32);
#[repr(C)]
pub struct WdsTransportSetupManager(i32);
#[repr(C)]
pub struct WdsTransportTftpClient(i32);
#[repr(C)]
pub struct WdsTransportTftpManager(i32);
