#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WFDCancelOpenSession<P0>(hsessionhandle: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WFDCancelOpenSession ( hsessionhandle : super::super::Foundation:: HANDLE ) -> u32 );
    WFDCancelOpenSession(hsessionhandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WFDCloseHandle<P0>(hclienthandle: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WFDCloseHandle ( hclienthandle : super::super::Foundation:: HANDLE ) -> u32 );
    WFDCloseHandle(hclienthandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WFDCloseSession<P0>(hsessionhandle: P0) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WFDCloseSession ( hsessionhandle : super::super::Foundation:: HANDLE ) -> u32 );
    WFDCloseSession(hsessionhandle.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WFDOpenHandle(dwclientversion: u32, pdwnegotiatedversion: *mut u32, phclienthandle: *mut super::super::Foundation::HANDLE) -> u32 {
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WFDOpenHandle ( dwclientversion : u32 , pdwnegotiatedversion : *mut u32 , phclienthandle : *mut super::super::Foundation:: HANDLE ) -> u32 );
    WFDOpenHandle(dwclientversion, pdwnegotiatedversion, phclienthandle)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WFDOpenLegacySession<P0>(hclienthandle: P0, plegacymacaddress: *const *const u8, phsessionhandle: *mut super::super::Foundation::HANDLE, pguidsessioninterface: *mut ::windows::core::GUID) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WFDOpenLegacySession ( hclienthandle : super::super::Foundation:: HANDLE , plegacymacaddress : *const *const u8 , phsessionhandle : *mut super::super::Foundation:: HANDLE , pguidsessioninterface : *mut ::windows::core::GUID ) -> u32 );
    WFDOpenLegacySession(hclienthandle.into_param().abi(), plegacymacaddress, phsessionhandle, pguidsessioninterface)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WFDStartOpenSession<P0>(hclienthandle: P0, pdeviceaddress: *const *const u8, pvcontext: ::core::option::Option<*const ::core::ffi::c_void>, pfncallback: WFD_OPEN_SESSION_COMPLETE_CALLBACK, phsessionhandle: *mut super::super::Foundation::HANDLE) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WFDStartOpenSession ( hclienthandle : super::super::Foundation:: HANDLE , pdeviceaddress : *const *const u8 , pvcontext : *const ::core::ffi::c_void , pfncallback : WFD_OPEN_SESSION_COMPLETE_CALLBACK , phsessionhandle : *mut super::super::Foundation:: HANDLE ) -> u32 );
    WFDStartOpenSession(hclienthandle.into_param().abi(), pdeviceaddress, ::core::mem::transmute(pvcontext.unwrap_or(::std::ptr::null())), pfncallback, phsessionhandle)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[inline]
pub unsafe fn WFDUpdateDeviceVisibility(pdeviceaddress: *const *const u8) -> u32 {
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WFDUpdateDeviceVisibility ( pdeviceaddress : *const *const u8 ) -> u32 );
    WFDUpdateDeviceVisibility(pdeviceaddress)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[inline]
pub unsafe fn WlanAllocateMemory(dwmemorysize: u32) -> *mut ::core::ffi::c_void {
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanAllocateMemory ( dwmemorysize : u32 ) -> *mut ::core::ffi::c_void );
    WlanAllocateMemory(dwmemorysize)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanCloseHandle<P0>(hclienthandle: P0, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanCloseHandle ( hclienthandle : super::super::Foundation:: HANDLE , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanCloseHandle(hclienthandle.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[inline]
pub unsafe fn WlanConnect<P0>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, pconnectionparameters: *const WLAN_CONNECTION_PARAMETERS, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanConnect ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , pconnectionparameters : *const WLAN_CONNECTION_PARAMETERS , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanConnect(hclienthandle.into_param().abi(), pinterfaceguid, pconnectionparameters, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[inline]
pub unsafe fn WlanConnect2<P0>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, pconnectionparameters: *const WLAN_CONNECTION_PARAMETERS_V2, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanConnect2 ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , pconnectionparameters : *const WLAN_CONNECTION_PARAMETERS_V2 , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanConnect2(hclienthandle.into_param().abi(), pinterfaceguid, pconnectionparameters, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanDeleteProfile<P0, P1>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: P1, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanDeleteProfile ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , strprofilename : ::windows::core::PCWSTR , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanDeleteProfile(hclienthandle.into_param().abi(), pinterfaceguid, strprofilename.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanDeviceServiceCommand<P0>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, pdeviceserviceguid: *const ::windows::core::GUID, dwopcode: u32, dwinbuffersize: u32, pinbuffer: ::core::option::Option<*const ::core::ffi::c_void>, dwoutbuffersize: u32, poutbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, pdwbytesreturned: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanDeviceServiceCommand ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , pdeviceserviceguid : *const ::windows::core::GUID , dwopcode : u32 , dwinbuffersize : u32 , pinbuffer : *const ::core::ffi::c_void , dwoutbuffersize : u32 , poutbuffer : *mut ::core::ffi::c_void , pdwbytesreturned : *mut u32 ) -> u32 );
    WlanDeviceServiceCommand(hclienthandle.into_param().abi(), pinterfaceguid, pdeviceserviceguid, dwopcode, dwinbuffersize, ::core::mem::transmute(pinbuffer.unwrap_or(::std::ptr::null())), dwoutbuffersize, ::core::mem::transmute(poutbuffer.unwrap_or(::std::ptr::null_mut())), pdwbytesreturned)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanDisconnect<P0>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanDisconnect ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanDisconnect(hclienthandle.into_param().abi(), pinterfaceguid, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanEnumInterfaces<P0>(hclienthandle: P0, preserved: ::core::option::Option<*const ::core::ffi::c_void>, ppinterfacelist: *mut *mut WLAN_INTERFACE_INFO_LIST) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanEnumInterfaces ( hclienthandle : super::super::Foundation:: HANDLE , preserved : *const ::core::ffi::c_void , ppinterfacelist : *mut *mut WLAN_INTERFACE_INFO_LIST ) -> u32 );
    WlanEnumInterfaces(hclienthandle.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), ppinterfacelist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanExtractPsdIEDataList<P0, P1>(hclienthandle: P0, prawiedata: &[u8], strformat: P1, preserved: ::core::option::Option<*const ::core::ffi::c_void>, pppsdiedatalist: *mut *mut WLAN_RAW_DATA_LIST) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanExtractPsdIEDataList ( hclienthandle : super::super::Foundation:: HANDLE , dwiedatasize : u32 , prawiedata : *const u8 , strformat : ::windows::core::PCWSTR , preserved : *const ::core::ffi::c_void , pppsdiedatalist : *mut *mut WLAN_RAW_DATA_LIST ) -> u32 );
    WlanExtractPsdIEDataList(hclienthandle.into_param().abi(), prawiedata.len() as _, ::core::mem::transmute(prawiedata.as_ptr()), strformat.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), pppsdiedatalist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[inline]
pub unsafe fn WlanFreeMemory(pmemory: *const ::core::ffi::c_void) {
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanFreeMemory ( pmemory : *const ::core::ffi::c_void ) -> ( ) );
    WlanFreeMemory(pmemory)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetAvailableNetworkList<P0>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, dwflags: u32, preserved: ::core::option::Option<*const ::core::ffi::c_void>, ppavailablenetworklist: *mut *mut WLAN_AVAILABLE_NETWORK_LIST) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanGetAvailableNetworkList ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , dwflags : u32 , preserved : *const ::core::ffi::c_void , ppavailablenetworklist : *mut *mut WLAN_AVAILABLE_NETWORK_LIST ) -> u32 );
    WlanGetAvailableNetworkList(hclienthandle.into_param().abi(), pinterfaceguid, dwflags, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), ppavailablenetworklist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetAvailableNetworkList2<P0>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, dwflags: u32, preserved: ::core::option::Option<*const ::core::ffi::c_void>, ppavailablenetworklist: *mut *mut WLAN_AVAILABLE_NETWORK_LIST_V2) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanGetAvailableNetworkList2 ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , dwflags : u32 , preserved : *const ::core::ffi::c_void , ppavailablenetworklist : *mut *mut WLAN_AVAILABLE_NETWORK_LIST_V2 ) -> u32 );
    WlanGetAvailableNetworkList2(hclienthandle.into_param().abi(), pinterfaceguid, dwflags, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), ppavailablenetworklist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetFilterList<P0>(hclienthandle: P0, wlanfilterlisttype: WLAN_FILTER_LIST_TYPE, preserved: ::core::option::Option<*const ::core::ffi::c_void>, ppnetworklist: *mut *mut DOT11_NETWORK_LIST) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanGetFilterList ( hclienthandle : super::super::Foundation:: HANDLE , wlanfilterlisttype : WLAN_FILTER_LIST_TYPE , preserved : *const ::core::ffi::c_void , ppnetworklist : *mut *mut DOT11_NETWORK_LIST ) -> u32 );
    WlanGetFilterList(hclienthandle.into_param().abi(), wlanfilterlisttype, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), ppnetworklist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetInterfaceCapability<P0>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, preserved: ::core::option::Option<*const ::core::ffi::c_void>, ppcapability: *mut *mut WLAN_INTERFACE_CAPABILITY) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanGetInterfaceCapability ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , preserved : *const ::core::ffi::c_void , ppcapability : *mut *mut WLAN_INTERFACE_CAPABILITY ) -> u32 );
    WlanGetInterfaceCapability(hclienthandle.into_param().abi(), pinterfaceguid, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), ppcapability)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetNetworkBssList<P0, P1>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, pdot11ssid: ::core::option::Option<*const DOT11_SSID>, dot11bsstype: DOT11_BSS_TYPE, bsecurityenabled: P1, preserved: ::core::option::Option<*const ::core::ffi::c_void>, ppwlanbsslist: *mut *mut WLAN_BSS_LIST) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanGetNetworkBssList ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , pdot11ssid : *const DOT11_SSID , dot11bsstype : DOT11_BSS_TYPE , bsecurityenabled : super::super::Foundation:: BOOL , preserved : *const ::core::ffi::c_void , ppwlanbsslist : *mut *mut WLAN_BSS_LIST ) -> u32 );
    WlanGetNetworkBssList(hclienthandle.into_param().abi(), pinterfaceguid, ::core::mem::transmute(pdot11ssid.unwrap_or(::std::ptr::null())), dot11bsstype, bsecurityenabled.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), ppwlanbsslist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetProfile<P0, P1>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: P1, preserved: ::core::option::Option<*const ::core::ffi::c_void>, pstrprofilexml: *mut ::windows::core::PWSTR, pdwflags: ::core::option::Option<*mut u32>, pdwgrantedaccess: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanGetProfile ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , strprofilename : ::windows::core::PCWSTR , preserved : *const ::core::ffi::c_void , pstrprofilexml : *mut ::windows::core::PWSTR , pdwflags : *mut u32 , pdwgrantedaccess : *mut u32 ) -> u32 );
    WlanGetProfile(hclienthandle.into_param().abi(), pinterfaceguid, strprofilename.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), pstrprofilexml, ::core::mem::transmute(pdwflags.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pdwgrantedaccess.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetProfileCustomUserData<P0, P1>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: P1, preserved: ::core::option::Option<*const ::core::ffi::c_void>, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanGetProfileCustomUserData ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , strprofilename : ::windows::core::PCWSTR , preserved : *const ::core::ffi::c_void , pdwdatasize : *mut u32 , ppdata : *mut *mut u8 ) -> u32 );
    WlanGetProfileCustomUserData(hclienthandle.into_param().abi(), pinterfaceguid, strprofilename.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), pdwdatasize, ppdata)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetProfileList<P0>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, preserved: ::core::option::Option<*const ::core::ffi::c_void>, ppprofilelist: *mut *mut WLAN_PROFILE_INFO_LIST) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanGetProfileList ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , preserved : *const ::core::ffi::c_void , ppprofilelist : *mut *mut WLAN_PROFILE_INFO_LIST ) -> u32 );
    WlanGetProfileList(hclienthandle.into_param().abi(), pinterfaceguid, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), ppprofilelist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetSecuritySettings<P0>(hclienthandle: P0, securableobject: WLAN_SECURABLE_OBJECT, pvaluetype: ::core::option::Option<*mut WLAN_OPCODE_VALUE_TYPE>, pstrcurrentsddl: *mut ::windows::core::PWSTR, pdwgrantedaccess: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanGetSecuritySettings ( hclienthandle : super::super::Foundation:: HANDLE , securableobject : WLAN_SECURABLE_OBJECT , pvaluetype : *mut WLAN_OPCODE_VALUE_TYPE , pstrcurrentsddl : *mut ::windows::core::PWSTR , pdwgrantedaccess : *mut u32 ) -> u32 );
    WlanGetSecuritySettings(hclienthandle.into_param().abi(), securableobject, ::core::mem::transmute(pvaluetype.unwrap_or(::std::ptr::null_mut())), pstrcurrentsddl, pdwgrantedaccess)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanGetSupportedDeviceServices<P0>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, ppdevsvcguidlist: *mut *mut WLAN_DEVICE_SERVICE_GUID_LIST) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanGetSupportedDeviceServices ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , ppdevsvcguidlist : *mut *mut WLAN_DEVICE_SERVICE_GUID_LIST ) -> u32 );
    WlanGetSupportedDeviceServices(hclienthandle.into_param().abi(), pinterfaceguid, ppdevsvcguidlist)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkForceStart<P0>(hclienthandle: P0, pfailreason: ::core::option::Option<*mut WLAN_HOSTED_NETWORK_REASON>, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanHostedNetworkForceStart ( hclienthandle : super::super::Foundation:: HANDLE , pfailreason : *mut WLAN_HOSTED_NETWORK_REASON , pvreserved : *const ::core::ffi::c_void ) -> u32 );
    WlanHostedNetworkForceStart(hclienthandle.into_param().abi(), ::core::mem::transmute(pfailreason.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkForceStop<P0>(hclienthandle: P0, pfailreason: ::core::option::Option<*mut WLAN_HOSTED_NETWORK_REASON>, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanHostedNetworkForceStop ( hclienthandle : super::super::Foundation:: HANDLE , pfailreason : *mut WLAN_HOSTED_NETWORK_REASON , pvreserved : *const ::core::ffi::c_void ) -> u32 );
    WlanHostedNetworkForceStop(hclienthandle.into_param().abi(), ::core::mem::transmute(pfailreason.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkInitSettings<P0>(hclienthandle: P0, pfailreason: ::core::option::Option<*mut WLAN_HOSTED_NETWORK_REASON>, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanHostedNetworkInitSettings ( hclienthandle : super::super::Foundation:: HANDLE , pfailreason : *mut WLAN_HOSTED_NETWORK_REASON , pvreserved : *const ::core::ffi::c_void ) -> u32 );
    WlanHostedNetworkInitSettings(hclienthandle.into_param().abi(), ::core::mem::transmute(pfailreason.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkQueryProperty<P0>(hclienthandle: P0, opcode: WLAN_HOSTED_NETWORK_OPCODE, pdwdatasize: *mut u32, ppvdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanHostedNetworkQueryProperty ( hclienthandle : super::super::Foundation:: HANDLE , opcode : WLAN_HOSTED_NETWORK_OPCODE , pdwdatasize : *mut u32 , ppvdata : *mut *mut ::core::ffi::c_void , pwlanopcodevaluetype : *mut WLAN_OPCODE_VALUE_TYPE , pvreserved : *const ::core::ffi::c_void ) -> u32 );
    WlanHostedNetworkQueryProperty(hclienthandle.into_param().abi(), opcode, pdwdatasize, ppvdata, pwlanopcodevaluetype, ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkQuerySecondaryKey<P0>(hclienthandle: P0, pdwkeylength: *mut u32, ppuckeydata: *mut *mut u8, pbispassphrase: *mut super::super::Foundation::BOOL, pbpersistent: *mut super::super::Foundation::BOOL, pfailreason: ::core::option::Option<*mut WLAN_HOSTED_NETWORK_REASON>, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanHostedNetworkQuerySecondaryKey ( hclienthandle : super::super::Foundation:: HANDLE , pdwkeylength : *mut u32 , ppuckeydata : *mut *mut u8 , pbispassphrase : *mut super::super::Foundation:: BOOL , pbpersistent : *mut super::super::Foundation:: BOOL , pfailreason : *mut WLAN_HOSTED_NETWORK_REASON , pvreserved : *const ::core::ffi::c_void ) -> u32 );
    WlanHostedNetworkQuerySecondaryKey(hclienthandle.into_param().abi(), pdwkeylength, ppuckeydata, pbispassphrase, pbpersistent, ::core::mem::transmute(pfailreason.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkQueryStatus<P0>(hclienthandle: P0, ppwlanhostednetworkstatus: *mut *mut WLAN_HOSTED_NETWORK_STATUS, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanHostedNetworkQueryStatus ( hclienthandle : super::super::Foundation:: HANDLE , ppwlanhostednetworkstatus : *mut *mut WLAN_HOSTED_NETWORK_STATUS , pvreserved : *const ::core::ffi::c_void ) -> u32 );
    WlanHostedNetworkQueryStatus(hclienthandle.into_param().abi(), ppwlanhostednetworkstatus, ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkRefreshSecuritySettings<P0>(hclienthandle: P0, pfailreason: ::core::option::Option<*mut WLAN_HOSTED_NETWORK_REASON>, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanHostedNetworkRefreshSecuritySettings ( hclienthandle : super::super::Foundation:: HANDLE , pfailreason : *mut WLAN_HOSTED_NETWORK_REASON , pvreserved : *const ::core::ffi::c_void ) -> u32 );
    WlanHostedNetworkRefreshSecuritySettings(hclienthandle.into_param().abi(), ::core::mem::transmute(pfailreason.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkSetProperty<P0>(hclienthandle: P0, opcode: WLAN_HOSTED_NETWORK_OPCODE, dwdatasize: u32, pvdata: *const ::core::ffi::c_void, pfailreason: ::core::option::Option<*mut WLAN_HOSTED_NETWORK_REASON>, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanHostedNetworkSetProperty ( hclienthandle : super::super::Foundation:: HANDLE , opcode : WLAN_HOSTED_NETWORK_OPCODE , dwdatasize : u32 , pvdata : *const ::core::ffi::c_void , pfailreason : *mut WLAN_HOSTED_NETWORK_REASON , pvreserved : *const ::core::ffi::c_void ) -> u32 );
    WlanHostedNetworkSetProperty(hclienthandle.into_param().abi(), opcode, dwdatasize, pvdata, ::core::mem::transmute(pfailreason.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkSetSecondaryKey<P0, P1, P2>(hclienthandle: P0, puckeydata: &[u8], bispassphrase: P1, bpersistent: P2, pfailreason: ::core::option::Option<*mut WLAN_HOSTED_NETWORK_REASON>, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
    P2: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanHostedNetworkSetSecondaryKey ( hclienthandle : super::super::Foundation:: HANDLE , dwkeylength : u32 , puckeydata : *const u8 , bispassphrase : super::super::Foundation:: BOOL , bpersistent : super::super::Foundation:: BOOL , pfailreason : *mut WLAN_HOSTED_NETWORK_REASON , pvreserved : *const ::core::ffi::c_void ) -> u32 );
    WlanHostedNetworkSetSecondaryKey(hclienthandle.into_param().abi(), puckeydata.len() as _, ::core::mem::transmute(puckeydata.as_ptr()), bispassphrase.into_param().abi(), bpersistent.into_param().abi(), ::core::mem::transmute(pfailreason.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkStartUsing<P0>(hclienthandle: P0, pfailreason: ::core::option::Option<*mut WLAN_HOSTED_NETWORK_REASON>, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanHostedNetworkStartUsing ( hclienthandle : super::super::Foundation:: HANDLE , pfailreason : *mut WLAN_HOSTED_NETWORK_REASON , pvreserved : *const ::core::ffi::c_void ) -> u32 );
    WlanHostedNetworkStartUsing(hclienthandle.into_param().abi(), ::core::mem::transmute(pfailreason.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanHostedNetworkStopUsing<P0>(hclienthandle: P0, pfailreason: ::core::option::Option<*mut WLAN_HOSTED_NETWORK_REASON>, pvreserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanHostedNetworkStopUsing ( hclienthandle : super::super::Foundation:: HANDLE , pfailreason : *mut WLAN_HOSTED_NETWORK_REASON , pvreserved : *const ::core::ffi::c_void ) -> u32 );
    WlanHostedNetworkStopUsing(hclienthandle.into_param().abi(), ::core::mem::transmute(pfailreason.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pvreserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanIhvControl<P0>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, r#type: WLAN_IHV_CONTROL_TYPE, dwinbuffersize: u32, pinbuffer: *const ::core::ffi::c_void, dwoutbuffersize: u32, poutbuffer: ::core::option::Option<*mut ::core::ffi::c_void>, pdwbytesreturned: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanIhvControl ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , r#type : WLAN_IHV_CONTROL_TYPE , dwinbuffersize : u32 , pinbuffer : *const ::core::ffi::c_void , dwoutbuffersize : u32 , poutbuffer : *mut ::core::ffi::c_void , pdwbytesreturned : *mut u32 ) -> u32 );
    WlanIhvControl(hclienthandle.into_param().abi(), pinterfaceguid, r#type, dwinbuffersize, pinbuffer, dwoutbuffersize, ::core::mem::transmute(poutbuffer.unwrap_or(::std::ptr::null_mut())), pdwbytesreturned)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanOpenHandle(dwclientversion: u32, preserved: ::core::option::Option<*const ::core::ffi::c_void>, pdwnegotiatedversion: *mut u32, phclienthandle: *mut super::super::Foundation::HANDLE) -> u32 {
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanOpenHandle ( dwclientversion : u32 , preserved : *const ::core::ffi::c_void , pdwnegotiatedversion : *mut u32 , phclienthandle : *mut super::super::Foundation:: HANDLE ) -> u32 );
    WlanOpenHandle(dwclientversion, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), pdwnegotiatedversion, phclienthandle)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanQueryAutoConfigParameter<P0>(hclienthandle: P0, opcode: WLAN_AUTOCONF_OPCODE, preserved: ::core::option::Option<*const ::core::ffi::c_void>, pdwdatasize: *mut u32, ppdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: ::core::option::Option<*mut WLAN_OPCODE_VALUE_TYPE>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanQueryAutoConfigParameter ( hclienthandle : super::super::Foundation:: HANDLE , opcode : WLAN_AUTOCONF_OPCODE , preserved : *const ::core::ffi::c_void , pdwdatasize : *mut u32 , ppdata : *mut *mut ::core::ffi::c_void , pwlanopcodevaluetype : *mut WLAN_OPCODE_VALUE_TYPE ) -> u32 );
    WlanQueryAutoConfigParameter(hclienthandle.into_param().abi(), opcode, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), pdwdatasize, ppdata, ::core::mem::transmute(pwlanopcodevaluetype.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanQueryInterface<P0>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, opcode: WLAN_INTF_OPCODE, preserved: ::core::option::Option<*const ::core::ffi::c_void>, pdwdatasize: *mut u32, ppdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: ::core::option::Option<*mut WLAN_OPCODE_VALUE_TYPE>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanQueryInterface ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , opcode : WLAN_INTF_OPCODE , preserved : *const ::core::ffi::c_void , pdwdatasize : *mut u32 , ppdata : *mut *mut ::core::ffi::c_void , pwlanopcodevaluetype : *mut WLAN_OPCODE_VALUE_TYPE ) -> u32 );
    WlanQueryInterface(hclienthandle.into_param().abi(), pinterfaceguid, opcode, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), pdwdatasize, ppdata, ::core::mem::transmute(pwlanopcodevaluetype.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[inline]
pub unsafe fn WlanReasonCodeToString(dwreasoncode: u32, pstringbuffer: &[u16], preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32 {
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanReasonCodeToString ( dwreasoncode : u32 , dwbuffersize : u32 , pstringbuffer : ::windows::core::PCWSTR , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanReasonCodeToString(dwreasoncode, pstringbuffer.len() as _, ::core::mem::transmute(pstringbuffer.as_ptr()), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanRegisterDeviceServiceNotification<P0>(hclienthandle: P0, pdevsvcguidlist: ::core::option::Option<*const WLAN_DEVICE_SERVICE_GUID_LIST>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanRegisterDeviceServiceNotification ( hclienthandle : super::super::Foundation:: HANDLE , pdevsvcguidlist : *const WLAN_DEVICE_SERVICE_GUID_LIST ) -> u32 );
    WlanRegisterDeviceServiceNotification(hclienthandle.into_param().abi(), ::core::mem::transmute(pdevsvcguidlist.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanRegisterNotification<P0, P1>(hclienthandle: P0, dwnotifsource: u32, bignoreduplicate: P1, funccallback: WLAN_NOTIFICATION_CALLBACK, pcallbackcontext: ::core::option::Option<*const ::core::ffi::c_void>, preserved: ::core::option::Option<*const ::core::ffi::c_void>, pdwprevnotifsource: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanRegisterNotification ( hclienthandle : super::super::Foundation:: HANDLE , dwnotifsource : u32 , bignoreduplicate : super::super::Foundation:: BOOL , funccallback : WLAN_NOTIFICATION_CALLBACK , pcallbackcontext : *const ::core::ffi::c_void , preserved : *const ::core::ffi::c_void , pdwprevnotifsource : *mut u32 ) -> u32 );
    WlanRegisterNotification(hclienthandle.into_param().abi(), dwnotifsource, bignoreduplicate.into_param().abi(), funccallback, ::core::mem::transmute(pcallbackcontext.unwrap_or(::std::ptr::null())), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pdwprevnotifsource.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanRegisterVirtualStationNotification<P0, P1>(hclienthandle: P0, bregister: P1, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanRegisterVirtualStationNotification ( hclienthandle : super::super::Foundation:: HANDLE , bregister : super::super::Foundation:: BOOL , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanRegisterVirtualStationNotification(hclienthandle.into_param().abi(), bregister.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanRenameProfile<P0, P1, P2>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, stroldprofilename: P1, strnewprofilename: P2, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanRenameProfile ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , stroldprofilename : ::windows::core::PCWSTR , strnewprofilename : ::windows::core::PCWSTR , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanRenameProfile(hclienthandle.into_param().abi(), pinterfaceguid, stroldprofilename.into_param().abi(), strnewprofilename.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSaveTemporaryProfile<P0, P1, P2, P3>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: P1, stralluserprofilesecurity: P2, dwflags: u32, boverwrite: P3, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanSaveTemporaryProfile ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , strprofilename : ::windows::core::PCWSTR , stralluserprofilesecurity : ::windows::core::PCWSTR , dwflags : u32 , boverwrite : super::super::Foundation:: BOOL , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanSaveTemporaryProfile(hclienthandle.into_param().abi(), pinterfaceguid, strprofilename.into_param().abi(), stralluserprofilesecurity.into_param().abi(), dwflags, boverwrite.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanScan<P0>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, pdot11ssid: ::core::option::Option<*const DOT11_SSID>, piedata: ::core::option::Option<*const WLAN_RAW_DATA>, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanScan ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , pdot11ssid : *const DOT11_SSID , piedata : *const WLAN_RAW_DATA , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanScan(hclienthandle.into_param().abi(), pinterfaceguid, ::core::mem::transmute(pdot11ssid.unwrap_or(::std::ptr::null())), ::core::mem::transmute(piedata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetAutoConfigParameter<P0>(hclienthandle: P0, opcode: WLAN_AUTOCONF_OPCODE, dwdatasize: u32, pdata: *const ::core::ffi::c_void, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanSetAutoConfigParameter ( hclienthandle : super::super::Foundation:: HANDLE , opcode : WLAN_AUTOCONF_OPCODE , dwdatasize : u32 , pdata : *const ::core::ffi::c_void , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanSetAutoConfigParameter(hclienthandle.into_param().abi(), opcode, dwdatasize, pdata, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetFilterList<P0>(hclienthandle: P0, wlanfilterlisttype: WLAN_FILTER_LIST_TYPE, pnetworklist: ::core::option::Option<*const DOT11_NETWORK_LIST>, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanSetFilterList ( hclienthandle : super::super::Foundation:: HANDLE , wlanfilterlisttype : WLAN_FILTER_LIST_TYPE , pnetworklist : *const DOT11_NETWORK_LIST , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanSetFilterList(hclienthandle.into_param().abi(), wlanfilterlisttype, ::core::mem::transmute(pnetworklist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetInterface<P0>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, opcode: WLAN_INTF_OPCODE, dwdatasize: u32, pdata: *const ::core::ffi::c_void, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanSetInterface ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , opcode : WLAN_INTF_OPCODE , dwdatasize : u32 , pdata : *const ::core::ffi::c_void , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanSetInterface(hclienthandle.into_param().abi(), pinterfaceguid, opcode, dwdatasize, pdata, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetProfile<P0, P1, P2, P3>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, dwflags: u32, strprofilexml: P1, stralluserprofilesecurity: P2, boverwrite: P3, preserved: ::core::option::Option<*const ::core::ffi::c_void>, pdwreasoncode: *mut u32) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::BOOL>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanSetProfile ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , dwflags : u32 , strprofilexml : ::windows::core::PCWSTR , stralluserprofilesecurity : ::windows::core::PCWSTR , boverwrite : super::super::Foundation:: BOOL , preserved : *const ::core::ffi::c_void , pdwreasoncode : *mut u32 ) -> u32 );
    WlanSetProfile(hclienthandle.into_param().abi(), pinterfaceguid, dwflags, strprofilexml.into_param().abi(), stralluserprofilesecurity.into_param().abi(), boverwrite.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), pdwreasoncode)
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetProfileCustomUserData<P0, P1>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: P1, pdata: &[u8], preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanSetProfileCustomUserData ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , strprofilename : ::windows::core::PCWSTR , dwdatasize : u32 , pdata : *const u8 , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanSetProfileCustomUserData(hclienthandle.into_param().abi(), pinterfaceguid, strprofilename.into_param().abi(), pdata.len() as _, ::core::mem::transmute(pdata.as_ptr()), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
#[inline]
pub unsafe fn WlanSetProfileEapUserData<P0, P1>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: P1, eaptype: super::super::Security::ExtensibleAuthenticationProtocol::EAP_METHOD_TYPE, dwflags: WLAN_SET_EAPHOST_FLAGS, pbeapuserdata: ::core::option::Option<&[u8]>, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanSetProfileEapUserData ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , strprofilename : ::windows::core::PCWSTR , eaptype : super::super::Security::ExtensibleAuthenticationProtocol:: EAP_METHOD_TYPE , dwflags : WLAN_SET_EAPHOST_FLAGS , dweapuserdatasize : u32 , pbeapuserdata : *const u8 , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanSetProfileEapUserData(hclienthandle.into_param().abi(), pinterfaceguid, strprofilename.into_param().abi(), ::core::mem::transmute(eaptype), dwflags, pbeapuserdata.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pbeapuserdata.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetProfileEapXmlUserData<P0, P1, P2>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: P1, dwflags: WLAN_SET_EAPHOST_FLAGS, streapxmluserdata: P2, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanSetProfileEapXmlUserData ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , strprofilename : ::windows::core::PCWSTR , dwflags : WLAN_SET_EAPHOST_FLAGS , streapxmluserdata : ::windows::core::PCWSTR , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanSetProfileEapXmlUserData(hclienthandle.into_param().abi(), pinterfaceguid, strprofilename.into_param().abi(), dwflags, streapxmluserdata.into_param().abi(), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetProfileList<P0>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, strprofilenames: &[::windows::core::PCWSTR], preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanSetProfileList ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , dwitems : u32 , strprofilenames : *const ::windows::core::PCWSTR , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanSetProfileList(hclienthandle.into_param().abi(), pinterfaceguid, strprofilenames.len() as _, ::core::mem::transmute(strprofilenames.as_ptr()), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetProfilePosition<P0, P1>(hclienthandle: P0, pinterfaceguid: *const ::windows::core::GUID, strprofilename: P1, dwposition: u32, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanSetProfilePosition ( hclienthandle : super::super::Foundation:: HANDLE , pinterfaceguid : *const ::windows::core::GUID , strprofilename : ::windows::core::PCWSTR , dwposition : u32 , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanSetProfilePosition(hclienthandle.into_param().abi(), pinterfaceguid, strprofilename.into_param().abi(), dwposition, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetPsdIEDataList<P0, P1>(hclienthandle: P0, strformat: P1, ppsdiedatalist: ::core::option::Option<*const WLAN_RAW_DATA_LIST>, preserved: ::core::option::Option<*const ::core::ffi::c_void>) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanSetPsdIEDataList ( hclienthandle : super::super::Foundation:: HANDLE , strformat : ::windows::core::PCWSTR , ppsdiedatalist : *const WLAN_RAW_DATA_LIST , preserved : *const ::core::ffi::c_void ) -> u32 );
    WlanSetPsdIEDataList(hclienthandle.into_param().abi(), strformat.into_param().abi(), ::core::mem::transmute(ppsdiedatalist.unwrap_or(::std::ptr::null())), ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanSetSecuritySettings<P0, P1>(hclienthandle: P0, securableobject: WLAN_SECURABLE_OBJECT, strmodifiedsddl: P1) -> u32
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows_targets::link ! ( "wlanapi.dll""system" fn WlanSetSecuritySettings ( hclienthandle : super::super::Foundation:: HANDLE , securableobject : WLAN_SECURABLE_OBJECT , strmodifiedsddl : ::windows::core::PCWSTR ) -> u32 );
    WlanSetSecuritySettings(hclienthandle.into_param().abi(), securableobject, strmodifiedsddl.into_param().abi())
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WlanUIEditProfile<P0, P1>(dwclientversion: u32, wstrprofilename: P0, pinterfaceguid: *const ::windows::core::GUID, hwnd: P1, wlstartpage: WL_DISPLAY_PAGES, preserved: ::core::option::Option<*const ::core::ffi::c_void>, pwlanreasoncode: ::core::option::Option<*mut u32>) -> u32
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HWND>,
{
    ::windows_targets::link ! ( "wlanui.dll""system" fn WlanUIEditProfile ( dwclientversion : u32 , wstrprofilename : ::windows::core::PCWSTR , pinterfaceguid : *const ::windows::core::GUID , hwnd : super::super::Foundation:: HWND , wlstartpage : WL_DISPLAY_PAGES , preserved : *const ::core::ffi::c_void , pwlanreasoncode : *mut u32 ) -> u32 );
    WlanUIEditProfile(dwclientversion, wstrprofilename.into_param().abi(), pinterfaceguid, hwnd.into_param().abi(), wlstartpage, ::core::mem::transmute(preserved.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pwlanreasoncode.unwrap_or(::std::ptr::null_mut())))
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
pub struct IDot11AdHocInterface(::windows::core::IUnknown);
impl IDot11AdHocInterface {
    pub unsafe fn GetDeviceSignature(&self, psignature: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDeviceSignature)(::windows::core::Interface::as_raw(self), psignature).ok()
    }
    pub unsafe fn GetFriendlyName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetFriendlyName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn IsDot11d(&self, pf11d: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsDot11d)(::windows::core::Interface::as_raw(self), pf11d).ok()
    }
    pub unsafe fn IsAdHocCapable(&self, pfadhoccapable: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsAdHocCapable)(::windows::core::Interface::as_raw(self), pfadhoccapable).ok()
    }
    pub unsafe fn IsRadioOn(&self, pfisradioon: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).IsRadioOn)(::windows::core::Interface::as_raw(self), pfisradioon).ok()
    }
    pub unsafe fn GetActiveNetwork(&self) -> ::windows::core::Result<IDot11AdHocNetwork> {
        let mut result__ = ::windows::core::zeroed::<IDot11AdHocNetwork>();
        (::windows::core::Interface::vtable(self).GetActiveNetwork)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIEnumSecuritySettings(&self) -> ::windows::core::Result<IEnumDot11AdHocSecuritySettings> {
        let mut result__ = ::windows::core::zeroed::<IEnumDot11AdHocSecuritySettings>();
        (::windows::core::Interface::vtable(self).GetIEnumSecuritySettings)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIEnumDot11AdHocNetworks(&self, pfilterguid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumDot11AdHocNetworks> {
        let mut result__ = ::windows::core::zeroed::<IEnumDot11AdHocNetworks>();
        (::windows::core::Interface::vtable(self).GetIEnumDot11AdHocNetworks)(::windows::core::Interface::as_raw(self), pfilterguid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetStatus(&self, pstate: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), pstate).ok()
    }
}
::windows::imp::interface_hierarchy!(IDot11AdHocInterface, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDot11AdHocInterface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocInterface {}
impl ::core::fmt::Debug for IDot11AdHocInterface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDot11AdHocInterface").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDot11AdHocInterface {
    type Vtable = IDot11AdHocInterface_Vtbl;
}
impl ::core::clone::Clone for IDot11AdHocInterface {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDot11AdHocInterface {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc2b_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDot11AdHocInterface_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDeviceSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetFriendlyName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszname: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub IsDot11d: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pf11d: *mut u8) -> ::windows::core::HRESULT,
    pub IsAdHocCapable: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfadhoccapable: *mut u8) -> ::windows::core::HRESULT,
    pub IsRadioOn: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfisradioon: *mut u8) -> ::windows::core::HRESULT,
    pub GetActiveNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppnetwork: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetIEnumSecuritySettings: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetIEnumDot11AdHocNetworks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pfilterguid: *const ::windows::core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pstate: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
pub struct IDot11AdHocInterfaceNotificationSink(::windows::core::IUnknown);
impl IDot11AdHocInterfaceNotificationSink {
    pub unsafe fn OnConnectionStatusChange(&self, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnConnectionStatusChange)(::windows::core::Interface::as_raw(self), estatus).ok()
    }
}
::windows::imp::interface_hierarchy!(IDot11AdHocInterfaceNotificationSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDot11AdHocInterfaceNotificationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocInterfaceNotificationSink {}
impl ::core::fmt::Debug for IDot11AdHocInterfaceNotificationSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDot11AdHocInterfaceNotificationSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDot11AdHocInterfaceNotificationSink {
    type Vtable = IDot11AdHocInterfaceNotificationSink_Vtbl;
}
impl ::core::clone::Clone for IDot11AdHocInterfaceNotificationSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDot11AdHocInterfaceNotificationSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc2f_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDot11AdHocInterfaceNotificationSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnConnectionStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
pub struct IDot11AdHocManager(::windows::core::IUnknown);
impl IDot11AdHocManager {
    pub unsafe fn CreateNetwork<P0, P1, P2, P3>(&self, name: P0, password: P1, geographicalid: i32, pinterface: P2, psecurity: P3, pcontextguid: *const ::windows::core::GUID) -> ::windows::core::Result<IDot11AdHocNetwork>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P2: ::windows::core::IntoParam<IDot11AdHocInterface>,
        P3: ::windows::core::IntoParam<IDot11AdHocSecuritySettings>,
    {
        let mut result__ = ::windows::core::zeroed::<IDot11AdHocNetwork>();
        (::windows::core::Interface::vtable(self).CreateNetwork)(::windows::core::Interface::as_raw(self), name.into_param().abi(), password.into_param().abi(), geographicalid, pinterface.into_param().abi(), psecurity.into_param().abi(), pcontextguid, &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CommitCreatedNetwork<P0, P1, P2>(&self, piadhoc: P0, fsaveprofile: P1, fmakesavedprofileuserspecific: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDot11AdHocNetwork>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
    {
        (::windows::core::Interface::vtable(self).CommitCreatedNetwork)(::windows::core::Interface::as_raw(self), piadhoc.into_param().abi(), fsaveprofile.into_param().abi(), fmakesavedprofileuserspecific.into_param().abi()).ok()
    }
    pub unsafe fn GetIEnumDot11AdHocNetworks(&self, pcontextguid: *const ::windows::core::GUID) -> ::windows::core::Result<IEnumDot11AdHocNetworks> {
        let mut result__ = ::windows::core::zeroed::<IEnumDot11AdHocNetworks>();
        (::windows::core::Interface::vtable(self).GetIEnumDot11AdHocNetworks)(::windows::core::Interface::as_raw(self), pcontextguid, &mut result__).from_abi(result__)
    }
    pub unsafe fn GetIEnumDot11AdHocInterfaces(&self) -> ::windows::core::Result<IEnumDot11AdHocInterfaces> {
        let mut result__ = ::windows::core::zeroed::<IEnumDot11AdHocInterfaces>();
        (::windows::core::Interface::vtable(self).GetIEnumDot11AdHocInterfaces)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetNetwork(&self, networksignature: *const ::windows::core::GUID) -> ::windows::core::Result<IDot11AdHocNetwork> {
        let mut result__ = ::windows::core::zeroed::<IDot11AdHocNetwork>();
        (::windows::core::Interface::vtable(self).GetNetwork)(::windows::core::Interface::as_raw(self), networksignature, &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IDot11AdHocManager, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDot11AdHocManager {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocManager {}
impl ::core::fmt::Debug for IDot11AdHocManager {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDot11AdHocManager").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDot11AdHocManager {
    type Vtable = IDot11AdHocManager_Vtbl;
}
impl ::core::clone::Clone for IDot11AdHocManager {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDot11AdHocManager {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc26_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDot11AdHocManager_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub CreateNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, name: ::windows::core::PCWSTR, password: ::windows::core::PCWSTR, geographicalid: i32, pinterface: *mut ::core::ffi::c_void, psecurity: *mut ::core::ffi::c_void, pcontextguid: *const ::windows::core::GUID, piadhoc: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub CommitCreatedNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piadhoc: *mut ::core::ffi::c_void, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    CommitCreatedNetwork: usize,
    pub GetIEnumDot11AdHocNetworks: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontextguid: *const ::windows::core::GUID, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetIEnumDot11AdHocInterfaces: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetNetwork: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, networksignature: *const ::windows::core::GUID, pnetwork: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
pub struct IDot11AdHocManagerNotificationSink(::windows::core::IUnknown);
impl IDot11AdHocManagerNotificationSink {
    pub unsafe fn OnNetworkAdd<P0>(&self, piadhocnetwork: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDot11AdHocNetwork>,
    {
        (::windows::core::Interface::vtable(self).OnNetworkAdd)(::windows::core::Interface::as_raw(self), piadhocnetwork.into_param().abi()).ok()
    }
    pub unsafe fn OnNetworkRemove(&self, signature: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnNetworkRemove)(::windows::core::Interface::as_raw(self), signature).ok()
    }
    pub unsafe fn OnInterfaceAdd<P0>(&self, piadhocinterface: P0) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<IDot11AdHocInterface>,
    {
        (::windows::core::Interface::vtable(self).OnInterfaceAdd)(::windows::core::Interface::as_raw(self), piadhocinterface.into_param().abi()).ok()
    }
    pub unsafe fn OnInterfaceRemove(&self, signature: *const ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnInterfaceRemove)(::windows::core::Interface::as_raw(self), signature).ok()
    }
}
::windows::imp::interface_hierarchy!(IDot11AdHocManagerNotificationSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDot11AdHocManagerNotificationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocManagerNotificationSink {}
impl ::core::fmt::Debug for IDot11AdHocManagerNotificationSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDot11AdHocManagerNotificationSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDot11AdHocManagerNotificationSink {
    type Vtable = IDot11AdHocManagerNotificationSink_Vtbl;
}
impl ::core::clone::Clone for IDot11AdHocManagerNotificationSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDot11AdHocManagerNotificationSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc27_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDot11AdHocManagerNotificationSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnNetworkAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piadhocnetwork: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnNetworkRemove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signature: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub OnInterfaceAdd: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, piadhocinterface: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub OnInterfaceRemove: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, signature: *const ::windows::core::GUID) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
pub struct IDot11AdHocNetwork(::windows::core::IUnknown);
impl IDot11AdHocNetwork {
    pub unsafe fn GetStatus(&self, estatus: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetStatus)(::windows::core::Interface::as_raw(self), estatus).ok()
    }
    pub unsafe fn GetSSID(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetSSID)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn HasProfile(&self, pf11d: *mut u8) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).HasProfile)(::windows::core::Interface::as_raw(self), pf11d).ok()
    }
    pub unsafe fn GetProfileName(&self) -> ::windows::core::Result<::windows::core::PWSTR> {
        let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
        (::windows::core::Interface::vtable(self).GetProfileName)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn DeleteProfile(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).DeleteProfile)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn GetSignalQuality(&self, pustrengthvalue: *mut u32, pustrengthmax: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSignalQuality)(::windows::core::Interface::as_raw(self), pustrengthvalue, pustrengthmax).ok()
    }
    pub unsafe fn GetSecuritySetting(&self) -> ::windows::core::Result<IDot11AdHocSecuritySettings> {
        let mut result__ = ::windows::core::zeroed::<IDot11AdHocSecuritySettings>();
        (::windows::core::Interface::vtable(self).GetSecuritySetting)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    pub unsafe fn GetContextGuid(&self, pcontextguid: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetContextGuid)(::windows::core::Interface::as_raw(self), pcontextguid).ok()
    }
    pub unsafe fn GetSignature(&self, psignature: *mut ::windows::core::GUID) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetSignature)(::windows::core::Interface::as_raw(self), psignature).ok()
    }
    pub unsafe fn GetInterface(&self) -> ::windows::core::Result<IDot11AdHocInterface> {
        let mut result__ = ::windows::core::zeroed::<IDot11AdHocInterface>();
        (::windows::core::Interface::vtable(self).GetInterface)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Connect<P0, P1, P2>(&self, passphrase: P0, geographicalid: i32, fsaveprofile: P1, fmakesavedprofileuserspecific: P2) -> ::windows::core::Result<()>
    where
        P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
        P1: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
        P2: ::windows::core::IntoParam<super::super::Foundation::BOOLEAN>,
    {
        (::windows::core::Interface::vtable(self).Connect)(::windows::core::Interface::as_raw(self), passphrase.into_param().abi(), geographicalid, fsaveprofile.into_param().abi(), fmakesavedprofileuserspecific.into_param().abi()).ok()
    }
    pub unsafe fn Disconnect(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Disconnect)(::windows::core::Interface::as_raw(self)).ok()
    }
}
::windows::imp::interface_hierarchy!(IDot11AdHocNetwork, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDot11AdHocNetwork {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocNetwork {}
impl ::core::fmt::Debug for IDot11AdHocNetwork {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDot11AdHocNetwork").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDot11AdHocNetwork {
    type Vtable = IDot11AdHocNetwork_Vtbl;
}
impl ::core::clone::Clone for IDot11AdHocNetwork {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDot11AdHocNetwork {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc29_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDot11AdHocNetwork_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetStatus: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, estatus: *mut DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT,
    pub GetSSID: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwssid: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub HasProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pf11d: *mut u8) -> ::windows::core::HRESULT,
    pub GetProfileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppszwprofilename: *mut ::windows::core::PWSTR) -> ::windows::core::HRESULT,
    pub DeleteProfile: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetSignalQuality: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pustrengthvalue: *mut u32, pustrengthmax: *mut u32) -> ::windows::core::HRESULT,
    pub GetSecuritySetting: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padhocsecuritysetting: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub GetContextGuid: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcontextguid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetSignature: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, psignature: *mut ::windows::core::GUID) -> ::windows::core::HRESULT,
    pub GetInterface: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, padhocinterface: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    #[cfg(feature = "Win32_Foundation")]
    pub Connect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, passphrase: ::windows::core::PCWSTR, geographicalid: i32, fsaveprofile: super::super::Foundation::BOOLEAN, fmakesavedprofileuserspecific: super::super::Foundation::BOOLEAN) -> ::windows::core::HRESULT,
    #[cfg(not(feature = "Win32_Foundation"))]
    Connect: usize,
    pub Disconnect: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
pub struct IDot11AdHocNetworkNotificationSink(::windows::core::IUnknown);
impl IDot11AdHocNetworkNotificationSink {
    pub unsafe fn OnStatusChange(&self, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnStatusChange)(::windows::core::Interface::as_raw(self), estatus).ok()
    }
    pub unsafe fn OnConnectFail(&self, efailreason: DOT11_ADHOC_CONNECT_FAIL_REASON) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).OnConnectFail)(::windows::core::Interface::as_raw(self), efailreason).ok()
    }
}
::windows::imp::interface_hierarchy!(IDot11AdHocNetworkNotificationSink, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDot11AdHocNetworkNotificationSink {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocNetworkNotificationSink {}
impl ::core::fmt::Debug for IDot11AdHocNetworkNotificationSink {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDot11AdHocNetworkNotificationSink").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDot11AdHocNetworkNotificationSink {
    type Vtable = IDot11AdHocNetworkNotificationSink_Vtbl;
}
impl ::core::clone::Clone for IDot11AdHocNetworkNotificationSink {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDot11AdHocNetworkNotificationSink {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc2a_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDot11AdHocNetworkNotificationSink_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub OnStatusChange: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, estatus: DOT11_ADHOC_NETWORK_CONNECTION_STATUS) -> ::windows::core::HRESULT,
    pub OnConnectFail: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, efailreason: DOT11_ADHOC_CONNECT_FAIL_REASON) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
pub struct IDot11AdHocSecuritySettings(::windows::core::IUnknown);
impl IDot11AdHocSecuritySettings {
    pub unsafe fn GetDot11AuthAlgorithm(&self, pauth: *mut DOT11_ADHOC_AUTH_ALGORITHM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDot11AuthAlgorithm)(::windows::core::Interface::as_raw(self), pauth).ok()
    }
    pub unsafe fn GetDot11CipherAlgorithm(&self, pcipher: *mut DOT11_ADHOC_CIPHER_ALGORITHM) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).GetDot11CipherAlgorithm)(::windows::core::Interface::as_raw(self), pcipher).ok()
    }
}
::windows::imp::interface_hierarchy!(IDot11AdHocSecuritySettings, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IDot11AdHocSecuritySettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDot11AdHocSecuritySettings {}
impl ::core::fmt::Debug for IDot11AdHocSecuritySettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDot11AdHocSecuritySettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IDot11AdHocSecuritySettings {
    type Vtable = IDot11AdHocSecuritySettings_Vtbl;
}
impl ::core::clone::Clone for IDot11AdHocSecuritySettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IDot11AdHocSecuritySettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc2e_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IDot11AdHocSecuritySettings_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub GetDot11AuthAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pauth: *mut DOT11_ADHOC_AUTH_ALGORITHM) -> ::windows::core::HRESULT,
    pub GetDot11CipherAlgorithm: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, pcipher: *mut DOT11_ADHOC_CIPHER_ALGORITHM) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
pub struct IEnumDot11AdHocInterfaces(::windows::core::IUnknown);
impl IEnumDot11AdHocInterfaces {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IDot11AdHocInterface>], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDot11AdHocInterfaces> {
        let mut result__ = ::windows::core::zeroed::<IEnumDot11AdHocInterfaces>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumDot11AdHocInterfaces, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumDot11AdHocInterfaces {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDot11AdHocInterfaces {}
impl ::core::fmt::Debug for IEnumDot11AdHocInterfaces {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDot11AdHocInterfaces").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumDot11AdHocInterfaces {
    type Vtable = IEnumDot11AdHocInterfaces_Vtbl;
}
impl ::core::clone::Clone for IEnumDot11AdHocInterfaces {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumDot11AdHocInterfaces {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc2c_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDot11AdHocInterfaces_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
pub struct IEnumDot11AdHocNetworks(::windows::core::IUnknown);
impl IEnumDot11AdHocNetworks {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IDot11AdHocNetwork>], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDot11AdHocNetworks> {
        let mut result__ = ::windows::core::zeroed::<IEnumDot11AdHocNetworks>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumDot11AdHocNetworks, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumDot11AdHocNetworks {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDot11AdHocNetworks {}
impl ::core::fmt::Debug for IEnumDot11AdHocNetworks {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDot11AdHocNetworks").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumDot11AdHocNetworks {
    type Vtable = IEnumDot11AdHocNetworks_Vtbl;
}
impl ::core::clone::Clone for IEnumDot11AdHocNetworks {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumDot11AdHocNetworks {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc28_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDot11AdHocNetworks_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
pub struct IEnumDot11AdHocSecuritySettings(::windows::core::IUnknown);
impl IEnumDot11AdHocSecuritySettings {
    pub unsafe fn Next(&self, rgelt: &mut [::core::option::Option<IDot11AdHocSecuritySettings>], pceltfetched: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Next)(::windows::core::Interface::as_raw(self), rgelt.len() as _, ::core::mem::transmute(rgelt.as_ptr()), pceltfetched).ok()
    }
    pub unsafe fn Skip(&self, celt: u32) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Skip)(::windows::core::Interface::as_raw(self), celt).ok()
    }
    pub unsafe fn Reset(&self) -> ::windows::core::Result<()> {
        (::windows::core::Interface::vtable(self).Reset)(::windows::core::Interface::as_raw(self)).ok()
    }
    pub unsafe fn Clone(&self) -> ::windows::core::Result<IEnumDot11AdHocSecuritySettings> {
        let mut result__ = ::windows::core::zeroed::<IEnumDot11AdHocSecuritySettings>();
        (::windows::core::Interface::vtable(self).Clone)(::windows::core::Interface::as_raw(self), &mut result__).from_abi(result__)
    }
}
::windows::imp::interface_hierarchy!(IEnumDot11AdHocSecuritySettings, ::windows::core::IUnknown);
impl ::core::cmp::PartialEq for IEnumDot11AdHocSecuritySettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IEnumDot11AdHocSecuritySettings {}
impl ::core::fmt::Debug for IEnumDot11AdHocSecuritySettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IEnumDot11AdHocSecuritySettings").field(&self.0).finish()
    }
}
unsafe impl ::windows::core::Interface for IEnumDot11AdHocSecuritySettings {
    type Vtable = IEnumDot11AdHocSecuritySettings_Vtbl;
}
impl ::core::clone::Clone for IEnumDot11AdHocSecuritySettings {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows::core::ComInterface for IEnumDot11AdHocSecuritySettings {
    const IID: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x8f10cc2d_cf0d_42a0_acbe_e2de7007384d);
}
#[repr(C)]
#[doc(hidden)]
pub struct IEnumDot11AdHocSecuritySettings_Vtbl {
    pub base__: ::windows::core::IUnknown_Vtbl,
    pub Next: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows::core::HRESULT,
    pub Skip: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT,
    pub Reset: unsafe extern "system" fn(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
    pub Clone: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT,
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_InfraCast_AccessPointBssid: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 19 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_InfraCast_ChallengeAep: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 21 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_InfraCast_DevnodeAep: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 23 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_InfraCast_HostName_ResolutionMode: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 25 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_InfraCast_PinSupported: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 29 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_InfraCast_RtspTcpConnectionParametersSupported: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 30 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_InfraCast_SinkHostName: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 20 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_InfraCast_SinkIpAddress: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 26 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_InfraCast_StreamSecuritySupported: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 18 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_InfraCast_Supported: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 17 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirectServices_AdvertisementId: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 5 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirectServices_RequestServiceInformation: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 7 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirectServices_ServiceAddress: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 2 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirectServices_ServiceConfigMethods: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 6 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirectServices_ServiceInformation: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 4 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirectServices_ServiceName: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x31b37743_7c5e_4005_93e6_e953f92b82e9), pid: 3 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_DeviceAddress: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 1 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_DeviceAddressCopy: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 13 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_FoundWsbService: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 24 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_GroupId: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 4 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_InformationElements: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 12 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_InterfaceAddress: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 2 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_InterfaceGuid: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 3 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_IsConnected: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 5 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_IsDMGCapable: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 22 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_IsLegacyDevice: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 7 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_IsMiracastLCPSupported: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 9 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_IsRecentlyAssociated: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 14 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_IsVisible: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 6 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_LinkQuality: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 28 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_MiracastVersion: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 8 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_Miracast_SessionMgmtControlPort: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 31 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_NoMiracastAutoProject: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 16 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_RtspTcpConnectionParametersSupported: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 32 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_Service_Aeps: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 15 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_Services: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 10 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_SupportedChannelList: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 11 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFiDirect_TransientAssociation: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0x1506935d_e3e7_450f_8637_82233ebe5f6e), pid: 27 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Devices_Properties\"`*"]
#[cfg(feature = "Win32_Devices_Properties")]
pub const DEVPKEY_WiFi_InterfaceGuid: super::super::Devices::Properties::DEVPROPKEY = super::super::Devices::Properties::DEVPROPKEY { fmtid: ::windows::core::GUID::from_u128(0xef1167eb_cbfc_4341_a568_a7c91a68982c), pid: 2 };
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DISCOVERY_FILTER_BITMASK_ANY: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DISCOVERY_FILTER_BITMASK_DEVICE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DISCOVERY_FILTER_BITMASK_GO: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11EXT_PSK_MAX_LENGTH: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADDITIONAL_IE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ANQP_QUERY_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ASSOCIATION_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ASSOCIATION_COMPLETION_PARAMETERS_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ASSOCIATION_INFO_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ASSOCIATION_PARAMS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ASSOCIATION_START_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ASSOC_ERROR_SOURCE_OS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ASSOC_ERROR_SOURCE_OTHER: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ASSOC_ERROR_SOURCE_REMOTE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ASSOC_STATUS_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGORITHM_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_MICHAEL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_CIPHER_PAIR_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AVAILABLE_CHANNEL_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AVAILABLE_FREQUENCY_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_BSSID_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_BSS_ENTRY_BYTE_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CAN_SUSTAIN_AP_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CAN_SUSTAIN_AP_REASON_IHV_END: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CAN_SUSTAIN_AP_REASON_IHV_START: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CAPABILITY_CHANNEL_AGILITY: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CAPABILITY_DSSSOFDM: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CAPABILITY_INFO_CF_POLLABLE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CAPABILITY_INFO_CF_POLL_REQ: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CAPABILITY_INFO_ESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CAPABILITY_INFO_IBSS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CAPABILITY_INFO_PRIVACY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CAPABILITY_PBCC: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CAPABILITY_SHORT_PREAMBLE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CAPABILITY_SHORT_SLOT_TIME: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CCA_MODE_CS_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CCA_MODE_CS_WITH_TIMER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CCA_MODE_ED_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CCA_MODE_ED_and_CS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CCA_MODE_HRCS_AND_ED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGORITHM_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_DEFAULT_KEY_VALUE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_KEY_MAPPING_KEY_VALUE_BYTE_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CONF_ALGO_TKIP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CONF_ALGO_WEP_RC4: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CONNECTION_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CONNECTION_START_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CONNECTION_STATUS_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_COUNTRY_OR_REGION_STRING_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_DATA_RATE_MAPPING_TABLE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_DEVICE_ENTRY_BYTE_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_DISASSOCIATE_PEER_REQUEST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_DISASSOCIATION_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ENCAP_802_1H: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ENCAP_RFC_1042: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXEMPT_ALWAYS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXEMPT_BOTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXEMPT_MULTICAST: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXEMPT_NO_EXEMPTION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXEMPT_ON_KEY_MAPPING_KEY_UNAVAILABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXEMPT_UNICAST: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXTAP_ATTRIBUTES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXTAP_RECV_CONTEXT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXTAP_SEND_CONTEXT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_CERTIFIED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_OID_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_RESERVED: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXTSTA_CAPABILITY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXTSTA_RECV_CONTEXT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_EXTSTA_SEND_CONTEXT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_FLAGS_80211B_CHANNEL_AGILITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_FLAGS_80211B_PBCC: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_FLAGS_80211B_SHORT_PREAMBLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_FLAGS_80211G_BARKER_PREAMBLE_MODE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_FLAGS_80211G_DSSS_OFDM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_FLAGS_80211G_NON_ERP_PRESENT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_FLAGS_80211G_USE_PROTECTION: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_FLAGS_PS_ON: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_FREQUENCY_BANDS_LOWER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_FREQUENCY_BANDS_MIDDLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_FREQUENCY_BANDS_UPPER: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_HESSID_LENGTH: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_HR_CCA_MODE_CS_AND_ED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_HR_CCA_MODE_CS_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_HR_CCA_MODE_CS_WITH_TIMER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_HR_CCA_MODE_ED_ONLY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_HR_CCA_MODE_HRCS_AND_ED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_HW_DEFRAGMENTATION_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_HW_FRAGMENTATION_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_HW_MSDU_AUTH_SUPPORTED_RX: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_HW_MSDU_AUTH_SUPPORTED_TX: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_HW_WEP_SUPPORTED_RX: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_HW_WEP_SUPPORTED_TX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_IBSS_PARAMS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_INCOMING_ASSOC_DECISION_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_INCOMING_ASSOC_DECISION_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_INCOMING_ASSOC_STARTED_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_INVALID_CHANNEL_NUMBER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_LINK_QUALITY_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MAC_ADDRESS_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MAC_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MANUFACTURING_CALLBACK_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MANUFACTURING_TEST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MAX_CHANNEL_HINTS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MAX_NUM_DEFAULT_KEY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MAX_NUM_DEFAULT_KEY_MFP: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MAX_NUM_OF_FRAGMENTS: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MAX_PDU_SIZE: u32 = 2346u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MAX_REQUESTED_SERVICE_INFORMATION_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MIN_PDU_SIZE: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MPDU_MAX_LENGTH_INDICATION_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_NLO_FLAG_SCAN_AT_SYSTEM_RESUME: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_NLO_FLAG_SCAN_ON_AOAC_PLATFORM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_NLO_FLAG_STOP_NLO_INDICATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_OFFLOAD_NETWORK_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_OI_MAX_LENGTH: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_OI_MIN_LENGTH: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_OPERATION_MODE_AP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_OPERATION_MODE_EXTENSIBLE_AP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_OPERATION_MODE_EXTENSIBLE_STATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_OPERATION_MODE_MANUFACTURING: u32 = 1073741824u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_OPERATION_MODE_NETWORK_MONITOR: u32 = 2147483648u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_OPERATION_MODE_STATION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_OPERATION_MODE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_OPERATION_MODE_WFD_CLIENT: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_OPERATION_MODE_WFD_DEVICE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_OPERATION_MODE_WFD_GROUP_OWNER: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_CTRL: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_DATA: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_MGMT: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_BROADCAST_CTRL: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_BROADCAST_DATA: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_BROADCAST_MGMT: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_DIRECTED_CTRL: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_DIRECTED_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_DIRECTED_MGMT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_MULTICAST_CTRL: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_MULTICAST_DATA: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_MULTICAST_MGMT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_PROMISCUOUS_CTRL: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_PROMISCUOUS_DATA: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PACKET_TYPE_PROMISCUOUS_MGMT: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PEER_INFO_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PHY_ATTRIBUTES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PHY_ID_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PHY_STATE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PHY_TYPE_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PMKID_CANDIDATE_LIST_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PMKID_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PORT_STATE_NOTIFICATION_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_POWER_MGMT_AUTO_MODE_ENABLED_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_POWER_MGMT_MODE_STATUS_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_POWER_SAVE_LEVEL_FAST_PSP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_POWER_SAVE_LEVEL_MAX_PSP: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_POWER_SAVING_FAST_PSP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_POWER_SAVING_MAXIMUM_LEVEL: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_POWER_SAVING_MAX_PSP: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_POWER_SAVING_NO_POWER_SAVING: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PRIORITY_CONTENTION: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PRIORITY_CONTENTION_FREE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PRIVACY_EXEMPTION_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PSD_IE_MAX_DATA_SIZE: u32 = 240u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_PSD_IE_MAX_ENTRY_NUMBER: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_QOS_PARAMS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_RATE_SET_MAX_LENGTH: u32 = 126u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_RECV_CONTEXT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_REG_DOMAIN_DOC: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_REG_DOMAIN_ETSI: u32 = 48u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_REG_DOMAIN_FCC: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_REG_DOMAIN_FRANCE: u32 = 50u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_REG_DOMAIN_MKK: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_REG_DOMAIN_OTHER: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_REG_DOMAIN_SPAIN: u32 = 49u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ROAMING_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ROAMING_START_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_SEND_CONTEXT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_SEND_INVITATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_SEND_INVITATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_SERVICE_CLASS_REORDERABLE_MULTICAST: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_SERVICE_CLASS_STRICTLY_ORDERED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_SSID_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_SSID_MAX_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATISTICS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_AP_JOIN_CONFIRM: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_AUTH_FAILED: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_AUTH_NOT_VERIFIED: u32 = 32768u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_AUTH_VERIFIED: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_ENCRYPTION_FAILED: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_EXCESSIVE_DATA_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_GENERATE_AUTH_FAILED: u32 = 16384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_ICV_VERIFIED: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_JOIN_CONFIRM: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_MPDU_MAX_LENGTH_CHANGED: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_PACKET_NOT_REASSEMBLED: u32 = 8192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_PACKET_REASSEMBLED: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_PS_LIFETIME_EXPIRED: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_RESET_CONFIRM: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_RETRY_LIMIT_EXCEEDED: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_SCAN_CONFIRM: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_START_CONFIRM: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_UNAVAILABLE_BSS: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_UNAVAILABLE_PRIORITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_UNAVAILABLE_SERVICE_CLASS: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_UNSUPPORTED_PRIORITY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_UNSUPPORTED_SERVICE_CLASS: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_WEP_KEY_UNAVAILABLE: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STATUS_XMIT_MSDU_TIMER_EXPIRED: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STOP_AP_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STOP_AP_REASON_AP_ACTIVE: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STOP_AP_REASON_CHANNEL_NOT_AVAILABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STOP_AP_REASON_FREQUENCY_NOT_AVAILABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STOP_AP_REASON_IHV_END: u32 = 4294967295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_STOP_AP_REASON_IHV_START: u32 = 4278190080u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_TKIPMIC_FAILURE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_VWIFI_ATTRIBUTES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_VWIFI_COMBINATION_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_VWIFI_COMBINATION_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_VWIFI_COMBINATION_REVISION_3: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_ADDITIONAL_IE_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_APS2_SERVICE_TYPE_MAX_LENGTH: u32 = 21u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_ASP2_INSTANCE_NAME_MAX_LENGTH: u32 = 63u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_ATTRIBUTES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DEVICE_AUTO_AVAILABILITY: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_CONCURRENT_OPERATION: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_CONFIG_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_CLIENT_DISCOVERABILITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_DEVICE_LIMIT: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_INFRASTRUCTURE_MANAGED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_INVITATION_PROCEDURE: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_RESERVED_6: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_RESERVED_7: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_SERVICE_DISCOVERY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DEVICE_HIGH_AVAILABILITY: u32 = 24u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DEVICE_INFO_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DEVICE_LISTEN_CHANNEL_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DEVICE_NOT_DISCOVERABLE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DISCOVER_COMPLETE_MAX_LIST_SIZE: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_DISCOVER_REQUEST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_CROSS_CONNECTION_SUPPORTED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_EAPOL_KEY_IP_ADDRESS_ALLOCATION_SUPPORTED: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_GROUP_LIMIT_REACHED: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_GROUP_OWNER: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_INTRABSS_DISTRIBUTION_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_IN_GROUP_FORMATION: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_PERSISTENT_GROUP: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_PERSISTENT_RECONNECT_SUPPORTED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_RESERVED_7: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_GROUP_JOIN_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_GROUP_START_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_FROM_WLAN_CROSS_CONNECTION_POLICY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_INFRASTRUCTURE_MANAGED_POLICY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_NOT_MANAGED_INFRASTRUCTURE_CAPABLE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_WFD_COEXISTENCE_POLICY: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_MINOR_REASON_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_SERVICE_INFORMATION_MAX_LENGTH: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_SERVICE_NAME_MAX_LENGTH: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_SESSION_INFO_MAX_LENGTH: u32 = 144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_STATUS_FAILED_INCOMPATIBLE_PARAMETERS: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_STATUS_FAILED_INCOMPATIBLE_PROVISIONING_METHOD: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_STATUS_FAILED_INFORMATION_IS_UNAVAILABLE: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_STATUS_FAILED_INVALID_PARAMETERS: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_STATUS_FAILED_LIMIT_REACHED: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_STATUS_FAILED_MATCHING_MAX_INTENT: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_STATUS_FAILED_NO_COMMON_CHANNELS: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_STATUS_FAILED_PREVIOUS_PROTOCOL_ERROR: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_STATUS_FAILED_REJECTED_BY_USER: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_STATUS_FAILED_UNABLE_TO_ACCOMODATE_REQUEST: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_STATUS_FAILED_UNKNOWN_WFD_GROUP: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_STATUS_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WFD_STATUS_SUCCESS_ACCEPTED_BY_USER: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WME_PACKET: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_DEVICE_NAME_MAX_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_MAX_MODEL_NAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_MAX_MODEL_NUMBER_LENGTH: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_MAX_PASSKEY_LENGTH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_VERSION_1_0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_VERSION_2_0: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_Enhanced: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_NoP2PSupported: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_NotSupported: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_SingleFunctionSupported: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_Supported: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_AcsSupport_Missing: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_AcsSupport_NotNeeded: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_AcsSupport_Present: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_BridgeType_PciConventional: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_BridgeType_PciExpressDownstreamSwitchPort: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_BridgeType_PciExpressEventCollector: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_BridgeType_PciExpressRootPort: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_BridgeType_PciExpressToPciXBridge: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_BridgeType_PciExpressTreatedAsPci: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_BridgeType_PciExpressUpstreamSwitchPort: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_BridgeType_PciX: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_BridgeType_PciXToExpressBridge: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_100Mhz: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_133MHZ: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_66Mhz: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_100Mhz: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_133Mhz: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_66Mhz: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_100MHz: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_133MHz: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_66MHz: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_100MHz: u32 = 14u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_133MHz: u32 = 15u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_66MHz: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode_Conventional_Pci: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_Pci_Conventional_33MHz: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_Pci_Conventional_66MHz: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_DeviceType_PciConventional: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_DeviceType_PciExpressEndpoint: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_DeviceType_PciExpressLegacyEndpoint: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_DeviceType_PciExpressRootComplexIntegratedEndpoint: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_DeviceType_PciExpressTreatedAsPci: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_DeviceType_PciX: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_InterruptType_LineBased: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_InterruptType_Msi: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_InterruptType_MsiX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_SriovSupport_DidntGetVfBarSpace: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_SriovSupport_MissingAcs: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_SriovSupport_MissingPfDriver: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_SriovSupport_NoBusResource: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciDevice_SriovSupport_Ok: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_LinkSpeed_Five_Gbps: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_LinkSpeed_TwoAndHalf_Gbps: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_LinkWidth_By_1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_LinkWidth_By_12: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_LinkWidth_By_16: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_LinkWidth_By_2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_LinkWidth_By_32: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_LinkWidth_By_4: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_LinkWidth_By_8: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_1024Bytes: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_128Bytes: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_2048Bytes: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_256Bytes: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_4096Bytes: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_512Bytes: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_Spec_Version_10: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciExpressDevice_Spec_Version_11: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_BusWidth_32Bits: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_BusWidth_64Bits: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_Conventional_33Mhz: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_Conventional_66Mhz: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_100Mhz: u32 = 9u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_133Mhz: u32 = 10u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_66Mhz: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_100Mhz: u32 = 12u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_133Mhz: u32 = 13u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_66Mhz: u32 = 11u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_100Mhz: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_133Mhz: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_66Mhz: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_100Mhz: u32 = 6u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_133Mhz: u32 = 7u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_66Mhz: u32 = 5u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_SecondaryInterface_PciConventional: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_SecondaryInterface_PciExpress: u32 = 3u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_SecondaryInterface_PciXMode1: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_SecondaryInterface_PciXMode2: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_Conventional_33Mhz: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_Conventional_66Mhz: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_133Mhz: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_266Mhz: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_533Mhz: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_66Mhz: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const Dot11AdHocManager: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xdd06a84f_83bd_4d01_8ab9_2389fea0869e);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const GUID_AEPSERVICE_WIFIDIRECT_DEVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xcc29827c_9caf_4928_99a9_18f7c2381389);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const GUID_DEVINTERFACE_ASP_INFRA_DEVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0xff823995_7a72_4c80_8757_c67ee13d1a49);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const GUID_DEVINTERFACE_WIFIDIRECT_DEVICE: ::windows::core::GUID = ::windows::core::GUID::from_u128(0x439b20af_8955_405b_99f0_a62af0c68d43);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const IHV_INIT_FUNCTION_NAME: ::windows::core::PCSTR = ::windows::core::s!("Dot11ExtIhvInitService");
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const IHV_INIT_VS_FUNCTION_NAME: ::windows::core::PCSTR = ::windows::core::s!("Dot11ExtIhvInitVirtualStation");
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const IHV_VERSION_FUNCTION_NAME: ::windows::core::PCSTR = ::windows::core::s!("Dot11ExtIhvGetVersionInfo");
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_CODE_GROUP_SIZE: u32 = 4096u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_CODE_PUBLIC_BEGIN: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_SOURCE_ALL: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_SOURCE_DOT3_AUTO_CONFIG: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_SOURCE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_SOURCE_ONEX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_SOURCE_SECURITY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_SOURCE_WCM: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_SOURCE_WCM_CSP: u32 = 512u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_SOURCE_WFD: u32 = 1024u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_SOURCE_WLAN_ACM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_SOURCE_WLAN_DEVICE_SERVICE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_SOURCE_WLAN_HNWK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_SOURCE_WLAN_IHV: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_SOURCE_WLAN_MSM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_NOTIFICATION_SOURCE_WLAN_SECURITY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_PROFILE_MAX_NAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_DOT11_AC_BASE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_DOT11_MSM_BASE: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_DOT11_SECURITY_BASE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_DOT3_AC_BASE: u32 = 393216u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_DOT3_MSM_BASE: u32 = 458752u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_GEN_BASE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_GROUP_SIZE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_IHV_BASE: u32 = 589824u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_ONEX_BASE: u32 = 327680u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_PROFILE_BASE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_PROFILE_MISSING: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_RESERVED_BASE: u32 = 720896u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_UNKNOWN: u32 = 65537u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const L2_REASON_CODE_WIMAX_BASE: u32 = 655360u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const MAX_NUM_SUPPORTED_RATES: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const MAX_NUM_SUPPORTED_RATES_V2: u32 = 255u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const MS_MAX_PROFILE_NAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const MS_PROFILE_GROUP_POLICY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const MS_PROFILE_USER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const NDIS_PACKET_TYPE_802_11_ALL_MULTICAST_DATA: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const NDIS_PACKET_TYPE_802_11_BROADCAST_DATA: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const NDIS_PACKET_TYPE_802_11_DIRECTED_DATA: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const NDIS_PACKET_TYPE_802_11_MULTICAST_DATA: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const NDIS_PACKET_TYPE_802_11_PROMISCUOUS_DATA: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_AP_JOIN_REQUEST: u32 = 218170205u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_ATIM_WINDOW: u32 = 218170122u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_BEACON_PERIOD: u32 = 218170139u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CCA_MODE_SUPPORTED: u32 = 218170166u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CCA_WATCHDOG_COUNT_MAX: u32 = 218170170u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CCA_WATCHDOG_COUNT_MIN: u32 = 218170172u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CCA_WATCHDOG_TIMER_MAX: u32 = 218170169u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CCA_WATCHDOG_TIMER_MIN: u32 = 218170171u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CFP_MAX_DURATION: u32 = 218170136u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CFP_PERIOD: u32 = 218170135u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CF_POLLABLE: u32 = 218170134u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CHANNEL_AGILITY_ENABLED: u32 = 218170184u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CHANNEL_AGILITY_PRESENT: u32 = 218170183u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_COUNTERS_ENTRY: u32 = 218170149u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_COUNTRY_STRING: u32 = 218170188u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_ADDRESS: u32 = 218171138u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_CCA_MODE: u32 = 218170167u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_CHANNEL: u32 = 218170165u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_CHANNEL_NUMBER: u32 = 218170159u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_DWELL_TIME: u32 = 218170161u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_FREQUENCY: u32 = 218170178u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_INDEX: u32 = 218170164u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_OFFLOAD_CAPABILITY: u32 = 218170113u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_OPERATION_MODE: u32 = 218170120u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_OPTIONAL_CAPABILITY: u32 = 218170131u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_PACKET_FILTER: u32 = 218170121u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_PATTERN: u32 = 218170163u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_PHY_TYPE: u32 = 218170124u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_REG_DOMAIN: u32 = 218170151u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_RX_ANTENNA: u32 = 218170155u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_SET: u32 = 218170162u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_TX_ANTENNA: u32 = 218170153u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_CURRENT_TX_POWER_LEVEL: u32 = 218170157u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_DEFAULT_WEP_OFFLOAD: u32 = 218170116u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_DEFAULT_WEP_UPLOAD: u32 = 218170117u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_DIVERSITY_SELECTION_RX: u32 = 218170176u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_DIVERSITY_SUPPORT: u32 = 218170154u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_DSSS_OFDM_OPTION_ENABLED: u32 = 218170209u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_DSSS_OFDM_OPTION_IMPLEMENTED: u32 = 218170208u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_DTIM_PERIOD: u32 = 218170140u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_ED_THRESHOLD: u32 = 218170168u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_EHCC_CAPABILITY_ENABLED: u32 = 218170193u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_EHCC_CAPABILITY_IMPLEMENTED: u32 = 218170192u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_EHCC_NUMBER_OF_CHANNELS_FAMILY_INDEX: u32 = 218170191u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_EHCC_PRIME_RADIX: u32 = 218170190u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_ERP_PBCC_OPTION_ENABLED: u32 = 218170207u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_ERP_PBCC_OPTION_IMPLEMENTED: u32 = 218170206u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_FRAGMENTATION_THRESHOLD: u32 = 218170146u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_FREQUENCY_BANDS_SUPPORTED: u32 = 218170180u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_HOPPING_PATTERN: u32 = 218170199u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_HOP_ALGORITHM_ADOPTED: u32 = 218170194u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_HOP_MODULUS: u32 = 218170197u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_HOP_OFFSET: u32 = 218170198u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_HOP_TIME: u32 = 218170158u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_HR_CCA_MODE_SUPPORTED: u32 = 218170185u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_JOIN_REQUEST: u32 = 218170125u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_LONG_RETRY_LIMIT: u32 = 218170145u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_MAC_ADDRESS: u32 = 218170142u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_MAXIMUM_LIST_SIZE: u32 = 218171141u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_MAX_DWELL_TIME: u32 = 218170160u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_MAX_MAC_ADDRESS_STATES: u32 = 218170212u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_MAX_RECEIVE_LIFETIME: u32 = 218170148u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_MAX_TRANSMIT_MSDU_LIFETIME: u32 = 218170147u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_MEDIUM_OCCUPANCY_LIMIT: u32 = 218170133u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_MPDU_MAX_LENGTH: u32 = 218170118u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_MULTICAST_LIST: u32 = 218171140u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY: u32 = 218170189u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY_ENABLED: u32 = 218170187u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY_IMPLEMENTED: u32 = 218170186u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_NDIS_START: u32 = 218170112u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_NIC_POWER_STATE: u32 = 218170129u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_NIC_SPECIFIC_EXTENSION: u32 = 218170204u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_NUMBER_OF_HOPPING_SETS: u32 = 218170196u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_OFFLOAD_CAPABILITY: u32 = 218170112u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_OPERATIONAL_RATE_SET: u32 = 218170138u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_OPERATION_MODE_CAPABILITY: u32 = 218170119u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_OPTIONAL_CAPABILITY: u32 = 218170130u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_PBCC_OPTION_IMPLEMENTED: u32 = 218170182u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_PERMANENT_ADDRESS: u32 = 218171139u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_POWER_MGMT_MODE: u32 = 218170137u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_PRIVATE_OIDS_START: u32 = 218171136u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_QOS_TX_DURATION: u32 = 218170219u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_QOS_TX_MEDIUM_TIME: u32 = 218170220u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_QOS_TX_QUEUES_SUPPORTED: u32 = 218170218u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_RANDOM_TABLE_FIELD_NUMBER: u32 = 218170200u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_RANDOM_TABLE_FLAG: u32 = 218170195u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_RECV_SENSITIVITY_LIST: u32 = 218170213u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_REG_DOMAINS_SUPPORT_VALUE: u32 = 218170173u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_RESET_REQUEST: u32 = 218170128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_RF_USAGE: u32 = 218170203u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_RSSI_RANGE: u32 = 218170202u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_RTS_THRESHOLD: u32 = 218170143u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_SCAN_REQUEST: u32 = 218170123u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_SHORT_PREAMBLE_OPTION_IMPLEMENTED: u32 = 218170181u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_SHORT_RETRY_LIMIT: u32 = 218170144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_SHORT_SLOT_TIME_OPTION_ENABLED: u32 = 218170211u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_SHORT_SLOT_TIME_OPTION_IMPLEMENTED: u32 = 218170210u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_START_REQUEST: u32 = 218170126u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_STATION_ID: u32 = 218170132u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_SUPPORTED_DATA_RATES_VALUE: u32 = 218170177u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_SUPPORTED_DSSS_CHANNEL_LIST: u32 = 218170222u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_SUPPORTED_OFDM_FREQUENCY_LIST: u32 = 218170221u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_SUPPORTED_PHY_TYPES: u32 = 218170150u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_SUPPORTED_POWER_LEVELS: u32 = 218170156u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_SUPPORTED_RX_ANTENNA: u32 = 218170175u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_SUPPORTED_TX_ANTENNA: u32 = 218170174u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_TEMP_TYPE: u32 = 218170152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_TI_THRESHOLD: u32 = 218170179u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_UPDATE_IE: u32 = 218170127u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_WEP_ICV_ERROR_COUNT: u32 = 218170141u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_WEP_OFFLOAD: u32 = 218170114u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_WEP_UPLOAD: u32 = 218170115u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_WME_AC_PARAMETERS: u32 = 218170216u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_WME_ENABLED: u32 = 218170215u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_WME_IMPLEMENTED: u32 = 218170214u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_WME_UPDATE_IE: u32 = 218170217u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OID_DOT11_WPA_TSC: u32 = 218170201u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WDIAG_IHV_WLAN_ID_FLAG_SECURITY_ENABLED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WFDSVC_CONNECTION_CAPABILITY_CLIENT: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WFDSVC_CONNECTION_CAPABILITY_GO: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WFDSVC_CONNECTION_CAPABILITY_NEW: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WFD_API_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WFD_API_VERSION_1_0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_API_VERSION: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_API_VERSION_1_0: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_API_VERSION_2_0: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_AVAILABLE_NETWORK_ANQP_SUPPORTED: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_AVAILABLE_NETWORK_AUTO_CONNECT_FAILED: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_AVAILABLE_NETWORK_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_AVAILABLE_NETWORK_CONSOLE_USER_PROFILE: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_AVAILABLE_NETWORK_HAS_PROFILE: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_DOMAIN: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_ENABLED: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_ROAMING: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_AVAILABLE_NETWORK_INCLUDE_ALL_ADHOC_PROFILES: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_AVAILABLE_NETWORK_INCLUDE_ALL_MANUAL_HIDDEN_PROFILES: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_AVAILABLE_NETWORK_INTERWORKING_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_CONNECTION_ADHOC_JOIN_ONLY: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_CONNECTION_EAPOL_PASSTHROUGH: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_CONNECTION_HIDDEN_NETWORK: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_CONNECTION_IGNORE_PRIVACY_BIT: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE_CONNECTION_MODE_AUTO: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE_OVERWRITE_EXISTING: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_MAX_NAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_MAX_PHY_INDEX: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_MAX_PHY_TYPE_NUMBER: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_NOTIFICATION_SOURCE_ACM: u32 = 8u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_NOTIFICATION_SOURCE_ALL: u32 = 65535u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_NOTIFICATION_SOURCE_DEVICE_SERVICE: u32 = 2048u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_NOTIFICATION_SOURCE_HNWK: u32 = 128u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_NOTIFICATION_SOURCE_IHV: u32 = 64u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_NOTIFICATION_SOURCE_MSM: u32 = 16u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_NOTIFICATION_SOURCE_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_NOTIFICATION_SOURCE_ONEX: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_NOTIFICATION_SOURCE_SECURITY: u32 = 32u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_PROFILE_CONNECTION_MODE_AUTO: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_PROFILE_CONNECTION_MODE_SET_BY_CLIENT: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_PROFILE_GET_PLAINTEXT_KEY: u32 = 4u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_PROFILE_GROUP_POLICY: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_PROFILE_USER: u32 = 2u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_AC_BASE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_AC_CONNECT_BASE: u32 = 163840u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_AC_END: u32 = 196607u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_ADHOC_SECURITY_FAILURE: u32 = 229386u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_AP_PROFILE_NOT_ALLOWED: u32 = 163856u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_AP_PROFILE_NOT_ALLOWED_FOR_CLIENT: u32 = 163855u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_AP_STARTING_FAILURE: u32 = 229395u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_ASSOCIATION_FAILURE: u32 = 229378u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_ASSOCIATION_TIMEOUT: u32 = 229379u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_AUTO_AP_PROFILE_NOT_ALLOWED: u32 = 524313u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_AUTO_CONNECTION_NOT_ALLOWED: u32 = 524314u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_AUTO_SWITCH_SET_FOR_ADHOC: u32 = 524304u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_AUTO_SWITCH_SET_FOR_MANUAL_CONNECTION: u32 = 524305u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_BAD_MAX_NUMBER_OF_CLIENTS_FOR_AP: u32 = 524310u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_BASE: u32 = 131072u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_BSS_TYPE_NOT_ALLOWED: u32 = 163845u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_BSS_TYPE_UNMATCH: u32 = 196611u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_CONFLICT_SECURITY: u32 = 524299u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_CONNECT_CALL_FAIL: u32 = 163849u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_DATARATE_UNMATCH: u32 = 196613u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_DISCONNECT_TIMEOUT: u32 = 229391u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_DRIVER_DISCONNECTED: u32 = 229387u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_DRIVER_OPERATION_FAILURE: u32 = 229388u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_GP_DENIED: u32 = 163843u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_HOTSPOT2_PROFILE_DENIED: u32 = 163857u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_HOTSPOT2_PROFILE_NOT_ALLOWED: u32 = 524315u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_IHV_CONNECTIVITY_NOT_SUPPORTED: u32 = 524309u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_IHV_NOT_AVAILABLE: u32 = 229389u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_IHV_NOT_RESPONDING: u32 = 229390u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_IHV_OUI_MISMATCH: u32 = 524296u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_IHV_OUI_MISSING: u32 = 524297u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_IHV_SECURITY_NOT_SUPPORTED: u32 = 524295u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_IHV_SECURITY_ONEX_MISSING: u32 = 524306u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_IHV_SETTINGS_MISSING: u32 = 524298u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_INTERNAL_FAILURE: u32 = 229392u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_INVALID_ADHOC_CONNECTION_MODE: u32 = 524302u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_INVALID_BSS_TYPE: u32 = 524301u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_INVALID_CHANNEL: u32 = 524311u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_INVALID_PHY_TYPE: u32 = 524293u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_INVALID_PROFILE_NAME: u32 = 524291u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_INVALID_PROFILE_SCHEMA: u32 = 524289u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_INVALID_PROFILE_TYPE: u32 = 524292u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_IN_BLOCKED_LIST: u32 = 163847u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_IN_FAILED_LIST: u32 = 163846u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_KEY_MISMATCH: u32 = 163853u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_AUTH_START_TIMEOUT: u32 = 294914u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_AUTH_SUCCESS_TIMEOUT: u32 = 294915u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_AUTH_WCN_COMPLETED: u32 = 294937u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_BASE: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_CANCELLED: u32 = 294929u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_DISCOVERY: u32 = 262165u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_MFP_NW_NIC: u32 = 262181u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_NETWORK: u32 = 262162u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_NIC: u32 = 262163u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE: u32 = 262164u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_AUTH: u32 = 262174u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_CIPHER: u32 = 262175u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_SAFE_MODE_NIC: u32 = 262177u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_SAFE_MODE_NW: u32 = 262178u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_CONNECT_BASE: u32 = 294912u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_DOWNGRADE_DETECTED: u32 = 294931u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_END: u32 = 327679u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_FORCED_FAILURE: u32 = 294933u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_GRP_KEY: u32 = 294925u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_KEY_DATA: u32 = 294924u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_MGMT_GRP_KEY: u32 = 294939u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_KEY_FORMAT: u32 = 294930u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_KEY_START_TIMEOUT: u32 = 294916u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_KEY_SUCCESS_TIMEOUT: u32 = 294917u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_M2_MISSING_IE: u32 = 294936u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_M2_MISSING_KEY_DATA: u32 = 294935u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_GRP_KEY: u32 = 294920u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_IE: u32 = 294919u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_KEY_DATA: u32 = 294918u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_MGMT_GRP_KEY: u32 = 294938u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_M3_TOO_MANY_RSNIE: u32 = 294934u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_MAX: u32 = 327679u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_MIN: u32 = 262144u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_MIXED_CELL: u32 = 262169u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_NIC_FAILURE: u32 = 294928u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_NO_AUTHENTICATOR: u32 = 294927u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_NO_PAIRWISE_KEY: u32 = 294923u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PEER_INDICATED_INSECURE: u32 = 294926u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_AUTH_TIMERS_INVALID: u32 = 262170u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_DUPLICATE_AUTH_CIPHER: u32 = 262151u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_AUTH_CIPHER: u32 = 262153u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_GKEY_INTV: u32 = 262171u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_KEY_INDEX: u32 = 262145u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_MODE: u32 = 262156u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_SIZE: u32 = 262157u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_TTL: u32 = 262158u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PREAUTH_MODE: u32 = 262159u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PREAUTH_THROTTLE: u32 = 262160u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEYMATERIAL_CHAR: u32 = 262167u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEY_LENGTH: u32 = 262147u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEY_UNMAPPED_CHAR: u32 = 262173u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_NO_AUTH_CIPHER_SPECIFIED: u32 = 262149u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_ONEX_DISABLED: u32 = 262154u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_ONEX_ENABLED: u32 = 262155u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PASSPHRASE_CHAR: u32 = 262166u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PREAUTH_ONLY_ENABLED: u32 = 262161u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PSK_LENGTH: u32 = 262148u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PSK_PRESENT: u32 = 262146u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_RAWDATA_INVALID: u32 = 262152u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_SAFE_MODE: u32 = 262176u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_TOO_MANY_AUTH_CIPHER_SPECIFIED: u32 = 262150u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_UNSUPPORTED_AUTH: u32 = 262179u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_UNSUPPORTED_CIPHER: u32 = 262180u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_WRONG_KEYTYPE: u32 = 262168u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PR_IE_MATCHING: u32 = 294921u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_PSK_MISMATCH_SUSPECTED: u32 = 294932u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_SEC_IE_MATCHING: u32 = 294922u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_TRANSITION_NETWORK: u32 = 262172u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSMSEC_UI_REQUEST_FAILURE: u32 = 294913u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSM_BASE: u32 = 196608u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSM_CONNECT_BASE: u32 = 229376u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSM_END: u32 = 262143u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_MSM_SECURITY_MISSING: u32 = 524294u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_NETWORK_NOT_AVAILABLE: u32 = 163851u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_NETWORK_NOT_COMPATIBLE: u32 = 131073u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_NON_BROADCAST_SET_FOR_ADHOC: u32 = 524303u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_NOT_VISIBLE: u32 = 163842u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_NO_AUTO_CONNECTION: u32 = 163841u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_NO_VISIBLE_AP: u32 = 229396u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_OPERATION_MODE_NOT_SUPPORTED: u32 = 524312u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_PHY_TYPE_UNMATCH: u32 = 196612u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_PRE_SECURITY_FAILURE: u32 = 229380u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_PROFILE_BASE: u32 = 524288u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_PROFILE_CHANGED_OR_DELETED: u32 = 163852u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_PROFILE_CONNECT_BASE: u32 = 557056u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_PROFILE_END: u32 = 589823u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_PROFILE_MISSING: u32 = 524290u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_PROFILE_NOT_COMPATIBLE: u32 = 131074u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_PROFILE_SSID_INVALID: u32 = 524307u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_RANGE_SIZE: u32 = 65536u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_RESERVED_BASE: u32 = 720896u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_RESERVED_END: u32 = 786431u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_ROAMING_FAILURE: u32 = 229384u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_ROAMING_SECURITY_FAILURE: u32 = 229385u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_SCAN_CALL_FAIL: u32 = 163850u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_SECURITY_FAILURE: u32 = 229382u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_SECURITY_MISSING: u32 = 524300u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_SECURITY_TIMEOUT: u32 = 229383u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_SSID_LIST_TOO_LONG: u32 = 163848u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_START_SECURITY_FAILURE: u32 = 229381u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_TOO_MANY_SECURITY_ATTEMPTS: u32 = 229394u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_TOO_MANY_SSID: u32 = 524308u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_UI_REQUEST_TIMEOUT: u32 = 229393u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_UNKNOWN: u32 = 65537u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_UNSUPPORTED_SECURITY_SET: u32 = 196610u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_UNSUPPORTED_SECURITY_SET_BY_OS: u32 = 196609u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_USER_CANCELLED: u32 = 229377u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_USER_DENIED: u32 = 163844u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_REASON_CODE_USER_NOT_RESPOND: u32 = 163854u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_UI_API_INITIAL_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_UI_API_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct CH_DESCRIPTION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ch_description_type_logical: CH_DESCRIPTION_TYPE = CH_DESCRIPTION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ch_description_type_center_frequency: CH_DESCRIPTION_TYPE = CH_DESCRIPTION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ch_description_type_phy_specific: CH_DESCRIPTION_TYPE = CH_DESCRIPTION_TYPE(3i32);
impl ::core::marker::Copy for CH_DESCRIPTION_TYPE {}
impl ::core::clone::Clone for CH_DESCRIPTION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for CH_DESCRIPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for CH_DESCRIPTION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for CH_DESCRIPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("CH_DESCRIPTION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11EXT_IHV_CONNECTION_PHASE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const connection_phase_any: DOT11EXT_IHV_CONNECTION_PHASE = DOT11EXT_IHV_CONNECTION_PHASE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const connection_phase_initial_connection: DOT11EXT_IHV_CONNECTION_PHASE = DOT11EXT_IHV_CONNECTION_PHASE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const connection_phase_post_l3_connection: DOT11EXT_IHV_CONNECTION_PHASE = DOT11EXT_IHV_CONNECTION_PHASE(2i32);
impl ::core::marker::Copy for DOT11EXT_IHV_CONNECTION_PHASE {}
impl ::core::clone::Clone for DOT11EXT_IHV_CONNECTION_PHASE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11EXT_IHV_CONNECTION_PHASE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11EXT_IHV_CONNECTION_PHASE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11EXT_IHV_CONNECTION_PHASE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11EXT_IHV_CONNECTION_PHASE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11EXT_IHV_INDICATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const IndicationTypeNicSpecificNotification: DOT11EXT_IHV_INDICATION_TYPE = DOT11EXT_IHV_INDICATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const IndicationTypePmkidCandidateList: DOT11EXT_IHV_INDICATION_TYPE = DOT11EXT_IHV_INDICATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const IndicationTypeTkipMicFailure: DOT11EXT_IHV_INDICATION_TYPE = DOT11EXT_IHV_INDICATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const IndicationTypePhyStateChange: DOT11EXT_IHV_INDICATION_TYPE = DOT11EXT_IHV_INDICATION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const IndicationTypeLinkQuality: DOT11EXT_IHV_INDICATION_TYPE = DOT11EXT_IHV_INDICATION_TYPE(4i32);
impl ::core::marker::Copy for DOT11EXT_IHV_INDICATION_TYPE {}
impl ::core::clone::Clone for DOT11EXT_IHV_INDICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11EXT_IHV_INDICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11EXT_IHV_INDICATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11EXT_IHV_INDICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11EXT_IHV_INDICATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_AC_PARAM(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_AC_param_BE: DOT11_AC_PARAM = DOT11_AC_PARAM(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_AC_param_BK: DOT11_AC_PARAM = DOT11_AC_PARAM(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_AC_param_VI: DOT11_AC_PARAM = DOT11_AC_PARAM(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_AC_param_VO: DOT11_AC_PARAM = DOT11_AC_PARAM(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_AC_param_max: DOT11_AC_PARAM = DOT11_AC_PARAM(4i32);
impl ::core::marker::Copy for DOT11_AC_PARAM {}
impl ::core::clone::Clone for DOT11_AC_PARAM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_AC_PARAM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_AC_PARAM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_AC_PARAM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_AC_PARAM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_ADHOC_AUTH_ALGORITHM(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_AUTH_ALGO_INVALID: DOT11_ADHOC_AUTH_ALGORITHM = DOT11_ADHOC_AUTH_ALGORITHM(-1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_AUTH_ALGO_80211_OPEN: DOT11_ADHOC_AUTH_ALGORITHM = DOT11_ADHOC_AUTH_ALGORITHM(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_AUTH_ALGO_RSNA_PSK: DOT11_ADHOC_AUTH_ALGORITHM = DOT11_ADHOC_AUTH_ALGORITHM(7i32);
impl ::core::marker::Copy for DOT11_ADHOC_AUTH_ALGORITHM {}
impl ::core::clone::Clone for DOT11_ADHOC_AUTH_ALGORITHM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_ADHOC_AUTH_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_ADHOC_AUTH_ALGORITHM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_ADHOC_AUTH_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_ADHOC_AUTH_ALGORITHM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_ADHOC_CIPHER_ALGORITHM(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_CIPHER_ALGO_INVALID: DOT11_ADHOC_CIPHER_ALGORITHM = DOT11_ADHOC_CIPHER_ALGORITHM(-1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_CIPHER_ALGO_NONE: DOT11_ADHOC_CIPHER_ALGORITHM = DOT11_ADHOC_CIPHER_ALGORITHM(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_CIPHER_ALGO_CCMP: DOT11_ADHOC_CIPHER_ALGORITHM = DOT11_ADHOC_CIPHER_ALGORITHM(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_CIPHER_ALGO_WEP: DOT11_ADHOC_CIPHER_ALGORITHM = DOT11_ADHOC_CIPHER_ALGORITHM(257i32);
impl ::core::marker::Copy for DOT11_ADHOC_CIPHER_ALGORITHM {}
impl ::core::clone::Clone for DOT11_ADHOC_CIPHER_ALGORITHM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_ADHOC_CIPHER_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_ADHOC_CIPHER_ALGORITHM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_ADHOC_CIPHER_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_ADHOC_CIPHER_ALGORITHM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_ADHOC_CONNECT_FAIL_REASON(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_CONNECT_FAIL_DOMAIN_MISMATCH: DOT11_ADHOC_CONNECT_FAIL_REASON = DOT11_ADHOC_CONNECT_FAIL_REASON(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_CONNECT_FAIL_PASSPHRASE_MISMATCH: DOT11_ADHOC_CONNECT_FAIL_REASON = DOT11_ADHOC_CONNECT_FAIL_REASON(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_CONNECT_FAIL_OTHER: DOT11_ADHOC_CONNECT_FAIL_REASON = DOT11_ADHOC_CONNECT_FAIL_REASON(2i32);
impl ::core::marker::Copy for DOT11_ADHOC_CONNECT_FAIL_REASON {}
impl ::core::clone::Clone for DOT11_ADHOC_CONNECT_FAIL_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_ADHOC_CONNECT_FAIL_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_ADHOC_CONNECT_FAIL_REASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_ADHOC_CONNECT_FAIL_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_ADHOC_CONNECT_FAIL_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_ADHOC_NETWORK_CONNECTION_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_INVALID: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = DOT11_ADHOC_NETWORK_CONNECTION_STATUS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_DISCONNECTED: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = DOT11_ADHOC_NETWORK_CONNECTION_STATUS(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_CONNECTING: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = DOT11_ADHOC_NETWORK_CONNECTION_STATUS(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_CONNECTED: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = DOT11_ADHOC_NETWORK_CONNECTION_STATUS(13i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_FORMED: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = DOT11_ADHOC_NETWORK_CONNECTION_STATUS(14i32);
impl ::core::marker::Copy for DOT11_ADHOC_NETWORK_CONNECTION_STATUS {}
impl ::core::clone::Clone for DOT11_ADHOC_NETWORK_CONNECTION_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_ADHOC_NETWORK_CONNECTION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_ADHOC_NETWORK_CONNECTION_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_ADHOC_NETWORK_CONNECTION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_ADHOC_NETWORK_CONNECTION_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_ANQP_QUERY_RESULT(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_ANQP_query_result_success: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_ANQP_query_result_failure: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_ANQP_query_result_timed_out: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_ANQP_query_result_resources: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_ANQP_query_result_advertisement_protocol_not_supported_on_remote: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_ANQP_query_result_gas_protocol_failure: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_ANQP_query_result_advertisement_server_not_responding: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_ANQP_query_result_access_issues: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(7i32);
impl ::core::marker::Copy for DOT11_ANQP_QUERY_RESULT {}
impl ::core::clone::Clone for DOT11_ANQP_QUERY_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_ANQP_QUERY_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_ANQP_QUERY_RESULT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_ANQP_QUERY_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_ANQP_QUERY_RESULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_ASSOCIATION_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_assoc_state_zero: DOT11_ASSOCIATION_STATE = DOT11_ASSOCIATION_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_assoc_state_unauth_unassoc: DOT11_ASSOCIATION_STATE = DOT11_ASSOCIATION_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_assoc_state_auth_unassoc: DOT11_ASSOCIATION_STATE = DOT11_ASSOCIATION_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_assoc_state_auth_assoc: DOT11_ASSOCIATION_STATE = DOT11_ASSOCIATION_STATE(3i32);
impl ::core::marker::Copy for DOT11_ASSOCIATION_STATE {}
impl ::core::clone::Clone for DOT11_ASSOCIATION_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_ASSOCIATION_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_ASSOCIATION_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_ASSOCIATION_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_ASSOCIATION_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_AUTH_ALGORITHM(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_80211_OPEN: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_80211_SHARED_KEY: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_WPA: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_WPA_PSK: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_WPA_NONE: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_RSNA: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_RSNA_PSK: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_WPA3: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_WPA3_ENT_192: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_WPA3_SAE: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_OWE: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_WPA3_ENT: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_IHV_START: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(-2147483648i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_AUTH_ALGO_IHV_END: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(-1i32);
impl ::core::marker::Copy for DOT11_AUTH_ALGORITHM {}
impl ::core::clone::Clone for DOT11_AUTH_ALGORITHM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_AUTH_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_AUTH_ALGORITHM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_AUTH_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_AUTH_ALGORITHM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_BAND(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_band_2p4g: DOT11_BAND = DOT11_BAND(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_band_4p9g: DOT11_BAND = DOT11_BAND(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_band_5g: DOT11_BAND = DOT11_BAND(3i32);
impl ::core::marker::Copy for DOT11_BAND {}
impl ::core::clone::Clone for DOT11_BAND {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_BAND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_BAND {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_BAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_BAND").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_BSS_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_BSS_type_infrastructure: DOT11_BSS_TYPE = DOT11_BSS_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_BSS_type_independent: DOT11_BSS_TYPE = DOT11_BSS_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_BSS_type_any: DOT11_BSS_TYPE = DOT11_BSS_TYPE(3i32);
impl ::core::marker::Copy for DOT11_BSS_TYPE {}
impl ::core::clone::Clone for DOT11_BSS_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_BSS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_BSS_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_BSS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_BSS_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_CIPHER_ALGORITHM(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_NONE: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_WEP40: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_TKIP: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_CCMP: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_WEP104: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_BIP: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_GCMP: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_GCMP_256: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_CCMP_256: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_BIP_GMAC_128: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_BIP_GMAC_256: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_BIP_CMAC_256: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(13i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_WPA_USE_GROUP: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(256i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_RSN_USE_GROUP: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(256i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_WEP: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(257i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_IHV_START: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(-2147483648i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_CIPHER_ALGO_IHV_END: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(-1i32);
impl ::core::marker::Copy for DOT11_CIPHER_ALGORITHM {}
impl ::core::clone::Clone for DOT11_CIPHER_ALGORITHM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_CIPHER_ALGORITHM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_CIPHER_ALGORITHM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_CIPHER_ALGORITHM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_CIPHER_ALGORITHM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_DIRECTION(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_DIR_INBOUND: DOT11_DIRECTION = DOT11_DIRECTION(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_DIR_OUTBOUND: DOT11_DIRECTION = DOT11_DIRECTION(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_DIR_BOTH: DOT11_DIRECTION = DOT11_DIRECTION(3i32);
impl ::core::marker::Copy for DOT11_DIRECTION {}
impl ::core::clone::Clone for DOT11_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_DIRECTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_DIRECTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_DIVERSITY_SUPPORT(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_diversity_support_unknown: DOT11_DIVERSITY_SUPPORT = DOT11_DIVERSITY_SUPPORT(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_diversity_support_fixedlist: DOT11_DIVERSITY_SUPPORT = DOT11_DIVERSITY_SUPPORT(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_diversity_support_notsupported: DOT11_DIVERSITY_SUPPORT = DOT11_DIVERSITY_SUPPORT(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_diversity_support_dynamic: DOT11_DIVERSITY_SUPPORT = DOT11_DIVERSITY_SUPPORT(3i32);
impl ::core::marker::Copy for DOT11_DIVERSITY_SUPPORT {}
impl ::core::clone::Clone for DOT11_DIVERSITY_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_DIVERSITY_SUPPORT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_DIVERSITY_SUPPORT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_DIVERSITY_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_DIVERSITY_SUPPORT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_DS_INFO(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_DS_CHANGED: DOT11_DS_INFO = DOT11_DS_INFO(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_DS_UNCHANGED: DOT11_DS_INFO = DOT11_DS_INFO(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_DS_UNKNOWN: DOT11_DS_INFO = DOT11_DS_INFO(2i32);
impl ::core::marker::Copy for DOT11_DS_INFO {}
impl ::core::clone::Clone for DOT11_DS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_DS_INFO {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_DS_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_DS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_DS_INFO").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_HOP_ALGO_ADOPTED(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_hop_algo_current: DOT11_HOP_ALGO_ADOPTED = DOT11_HOP_ALGO_ADOPTED(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_hop_algo_hop_index: DOT11_HOP_ALGO_ADOPTED = DOT11_HOP_ALGO_ADOPTED(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_hop_algo_hcc: DOT11_HOP_ALGO_ADOPTED = DOT11_HOP_ALGO_ADOPTED(2i32);
impl ::core::marker::Copy for DOT11_HOP_ALGO_ADOPTED {}
impl ::core::clone::Clone for DOT11_HOP_ALGO_ADOPTED {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_HOP_ALGO_ADOPTED {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_HOP_ALGO_ADOPTED {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_HOP_ALGO_ADOPTED {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_HOP_ALGO_ADOPTED").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_KEY_DIRECTION(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_key_direction_both: DOT11_KEY_DIRECTION = DOT11_KEY_DIRECTION(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_key_direction_inbound: DOT11_KEY_DIRECTION = DOT11_KEY_DIRECTION(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_key_direction_outbound: DOT11_KEY_DIRECTION = DOT11_KEY_DIRECTION(3i32);
impl ::core::marker::Copy for DOT11_KEY_DIRECTION {}
impl ::core::clone::Clone for DOT11_KEY_DIRECTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_KEY_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_KEY_DIRECTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_KEY_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_KEY_DIRECTION").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_MANUFACTURING_CALLBACK_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_callback_unknown: DOT11_MANUFACTURING_CALLBACK_TYPE = DOT11_MANUFACTURING_CALLBACK_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_callback_self_test_complete: DOT11_MANUFACTURING_CALLBACK_TYPE = DOT11_MANUFACTURING_CALLBACK_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_callback_sleep_complete: DOT11_MANUFACTURING_CALLBACK_TYPE = DOT11_MANUFACTURING_CALLBACK_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_callback_IHV_start: DOT11_MANUFACTURING_CALLBACK_TYPE = DOT11_MANUFACTURING_CALLBACK_TYPE(-2147483648i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_callback_IHV_end: DOT11_MANUFACTURING_CALLBACK_TYPE = DOT11_MANUFACTURING_CALLBACK_TYPE(-1i32);
impl ::core::marker::Copy for DOT11_MANUFACTURING_CALLBACK_TYPE {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_CALLBACK_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_MANUFACTURING_CALLBACK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_MANUFACTURING_CALLBACK_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_CALLBACK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_MANUFACTURING_CALLBACK_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_MANUFACTURING_SELF_TEST_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MANUFACTURING_SELF_TEST_TYPE_INTERFACE: DOT11_MANUFACTURING_SELF_TEST_TYPE = DOT11_MANUFACTURING_SELF_TEST_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MANUFACTURING_SELF_TEST_TYPE_RF_INTERFACE: DOT11_MANUFACTURING_SELF_TEST_TYPE = DOT11_MANUFACTURING_SELF_TEST_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MANUFACTURING_SELF_TEST_TYPE_BT_COEXISTENCE: DOT11_MANUFACTURING_SELF_TEST_TYPE = DOT11_MANUFACTURING_SELF_TEST_TYPE(3i32);
impl ::core::marker::Copy for DOT11_MANUFACTURING_SELF_TEST_TYPE {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_SELF_TEST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_MANUFACTURING_SELF_TEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_MANUFACTURING_SELF_TEST_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_SELF_TEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_MANUFACTURING_SELF_TEST_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_MANUFACTURING_TEST_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_test_unknown: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_test_self_start: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_test_self_query_result: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_test_rx: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_test_tx: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_test_query_adc: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_test_set_data: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_test_query_data: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_test_sleep: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_test_awake: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_test_IHV_start: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(-2147483648i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_manufacturing_test_IHV_end: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(-1i32);
impl ::core::marker::Copy for DOT11_MANUFACTURING_TEST_TYPE {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_TEST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_MANUFACTURING_TEST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_MANUFACTURING_TEST_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_TEST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_MANUFACTURING_TEST_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_MSONEX_RESULT(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MSONEX_SUCCESS: DOT11_MSONEX_RESULT = DOT11_MSONEX_RESULT(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MSONEX_FAILURE: DOT11_MSONEX_RESULT = DOT11_MSONEX_RESULT(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_MSONEX_IN_PROGRESS: DOT11_MSONEX_RESULT = DOT11_MSONEX_RESULT(2i32);
impl ::core::marker::Copy for DOT11_MSONEX_RESULT {}
impl ::core::clone::Clone for DOT11_MSONEX_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_MSONEX_RESULT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_MSONEX_RESULT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_MSONEX_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_MSONEX_RESULT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_OFFLOAD_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_offload_type_wep: DOT11_OFFLOAD_TYPE = DOT11_OFFLOAD_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_offload_type_auth: DOT11_OFFLOAD_TYPE = DOT11_OFFLOAD_TYPE(2i32);
impl ::core::marker::Copy for DOT11_OFFLOAD_TYPE {}
impl ::core::clone::Clone for DOT11_OFFLOAD_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_OFFLOAD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_OFFLOAD_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_OFFLOAD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_OFFLOAD_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_PHY_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_unknown: DOT11_PHY_TYPE = DOT11_PHY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_any: DOT11_PHY_TYPE = DOT11_PHY_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_fhss: DOT11_PHY_TYPE = DOT11_PHY_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_dsss: DOT11_PHY_TYPE = DOT11_PHY_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_irbaseband: DOT11_PHY_TYPE = DOT11_PHY_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_ofdm: DOT11_PHY_TYPE = DOT11_PHY_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_hrdsss: DOT11_PHY_TYPE = DOT11_PHY_TYPE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_erp: DOT11_PHY_TYPE = DOT11_PHY_TYPE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_ht: DOT11_PHY_TYPE = DOT11_PHY_TYPE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_vht: DOT11_PHY_TYPE = DOT11_PHY_TYPE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_dmg: DOT11_PHY_TYPE = DOT11_PHY_TYPE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_he: DOT11_PHY_TYPE = DOT11_PHY_TYPE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_eht: DOT11_PHY_TYPE = DOT11_PHY_TYPE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_IHV_start: DOT11_PHY_TYPE = DOT11_PHY_TYPE(-2147483648i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_phy_type_IHV_end: DOT11_PHY_TYPE = DOT11_PHY_TYPE(-1i32);
impl ::core::marker::Copy for DOT11_PHY_TYPE {}
impl ::core::clone::Clone for DOT11_PHY_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_PHY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_PHY_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_PHY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_PHY_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_POWER_MODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_power_mode_unknown: DOT11_POWER_MODE = DOT11_POWER_MODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_power_mode_active: DOT11_POWER_MODE = DOT11_POWER_MODE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_power_mode_powersave: DOT11_POWER_MODE = DOT11_POWER_MODE(2i32);
impl ::core::marker::Copy for DOT11_POWER_MODE {}
impl ::core::clone::Clone for DOT11_POWER_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_POWER_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_POWER_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_POWER_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_POWER_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_POWER_MODE_REASON(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_power_mode_reason_no_change: DOT11_POWER_MODE_REASON = DOT11_POWER_MODE_REASON(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_power_mode_reason_noncompliant_AP: DOT11_POWER_MODE_REASON = DOT11_POWER_MODE_REASON(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_power_mode_reason_legacy_WFD_device: DOT11_POWER_MODE_REASON = DOT11_POWER_MODE_REASON(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_power_mode_reason_compliant_AP: DOT11_POWER_MODE_REASON = DOT11_POWER_MODE_REASON(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_power_mode_reason_compliant_WFD_device: DOT11_POWER_MODE_REASON = DOT11_POWER_MODE_REASON(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_power_mode_reason_others: DOT11_POWER_MODE_REASON = DOT11_POWER_MODE_REASON(5i32);
impl ::core::marker::Copy for DOT11_POWER_MODE_REASON {}
impl ::core::clone::Clone for DOT11_POWER_MODE_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_POWER_MODE_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_POWER_MODE_REASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_POWER_MODE_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_POWER_MODE_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_RADIO_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_radio_state_unknown: DOT11_RADIO_STATE = DOT11_RADIO_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_radio_state_on: DOT11_RADIO_STATE = DOT11_RADIO_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_radio_state_off: DOT11_RADIO_STATE = DOT11_RADIO_STATE(2i32);
impl ::core::marker::Copy for DOT11_RADIO_STATE {}
impl ::core::clone::Clone for DOT11_RADIO_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_RADIO_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_RADIO_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_RADIO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_RADIO_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_RESET_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_reset_type_phy: DOT11_RESET_TYPE = DOT11_RESET_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_reset_type_mac: DOT11_RESET_TYPE = DOT11_RESET_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_reset_type_phy_and_mac: DOT11_RESET_TYPE = DOT11_RESET_TYPE(3i32);
impl ::core::marker::Copy for DOT11_RESET_TYPE {}
impl ::core::clone::Clone for DOT11_RESET_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_RESET_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_RESET_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_RESET_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_RESET_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_SCAN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_scan_type_active: DOT11_SCAN_TYPE = DOT11_SCAN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_scan_type_passive: DOT11_SCAN_TYPE = DOT11_SCAN_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_scan_type_auto: DOT11_SCAN_TYPE = DOT11_SCAN_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_scan_type_forced: DOT11_SCAN_TYPE = DOT11_SCAN_TYPE(-2147483648i32);
impl ::core::marker::Copy for DOT11_SCAN_TYPE {}
impl ::core::clone::Clone for DOT11_SCAN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_SCAN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_SCAN_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_SCAN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_SCAN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_TEMP_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_temp_type_unknown: DOT11_TEMP_TYPE = DOT11_TEMP_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_temp_type_1: DOT11_TEMP_TYPE = DOT11_TEMP_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_temp_type_2: DOT11_TEMP_TYPE = DOT11_TEMP_TYPE(2i32);
impl ::core::marker::Copy for DOT11_TEMP_TYPE {}
impl ::core::clone::Clone for DOT11_TEMP_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_TEMP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_TEMP_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_TEMP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_TEMP_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_UPDATE_IE_OP(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_update_ie_op_create_replace: DOT11_UPDATE_IE_OP = DOT11_UPDATE_IE_OP(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_update_ie_op_delete: DOT11_UPDATE_IE_OP = DOT11_UPDATE_IE_OP(2i32);
impl ::core::marker::Copy for DOT11_UPDATE_IE_OP {}
impl ::core::clone::Clone for DOT11_UPDATE_IE_OP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_UPDATE_IE_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_UPDATE_IE_OP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_UPDATE_IE_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_UPDATE_IE_OP").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_WFD_DISCOVER_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_wfd_discover_type_scan_only: DOT11_WFD_DISCOVER_TYPE = DOT11_WFD_DISCOVER_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_wfd_discover_type_find_only: DOT11_WFD_DISCOVER_TYPE = DOT11_WFD_DISCOVER_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_wfd_discover_type_auto: DOT11_WFD_DISCOVER_TYPE = DOT11_WFD_DISCOVER_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_wfd_discover_type_scan_social_channels: DOT11_WFD_DISCOVER_TYPE = DOT11_WFD_DISCOVER_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_wfd_discover_type_forced: DOT11_WFD_DISCOVER_TYPE = DOT11_WFD_DISCOVER_TYPE(-2147483648i32);
impl ::core::marker::Copy for DOT11_WFD_DISCOVER_TYPE {}
impl ::core::clone::Clone for DOT11_WFD_DISCOVER_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_WFD_DISCOVER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_DISCOVER_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_WFD_DISCOVER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_WFD_DISCOVER_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_WFD_SCAN_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_wfd_scan_type_active: DOT11_WFD_SCAN_TYPE = DOT11_WFD_SCAN_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_wfd_scan_type_passive: DOT11_WFD_SCAN_TYPE = DOT11_WFD_SCAN_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const dot11_wfd_scan_type_auto: DOT11_WFD_SCAN_TYPE = DOT11_WFD_SCAN_TYPE(3i32);
impl ::core::marker::Copy for DOT11_WFD_SCAN_TYPE {}
impl ::core::clone::Clone for DOT11_WFD_SCAN_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_WFD_SCAN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_SCAN_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_WFD_SCAN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_WFD_SCAN_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_WPS_CONFIG_METHOD(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_CONFIG_METHOD_NULL: DOT11_WPS_CONFIG_METHOD = DOT11_WPS_CONFIG_METHOD(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_CONFIG_METHOD_DISPLAY: DOT11_WPS_CONFIG_METHOD = DOT11_WPS_CONFIG_METHOD(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_CONFIG_METHOD_NFC_TAG: DOT11_WPS_CONFIG_METHOD = DOT11_WPS_CONFIG_METHOD(32i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_CONFIG_METHOD_NFC_INTERFACE: DOT11_WPS_CONFIG_METHOD = DOT11_WPS_CONFIG_METHOD(64i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_CONFIG_METHOD_PUSHBUTTON: DOT11_WPS_CONFIG_METHOD = DOT11_WPS_CONFIG_METHOD(128i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_CONFIG_METHOD_KEYPAD: DOT11_WPS_CONFIG_METHOD = DOT11_WPS_CONFIG_METHOD(256i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_CONFIG_METHOD_WFDS_DEFAULT: DOT11_WPS_CONFIG_METHOD = DOT11_WPS_CONFIG_METHOD(4096i32);
impl ::core::marker::Copy for DOT11_WPS_CONFIG_METHOD {}
impl ::core::clone::Clone for DOT11_WPS_CONFIG_METHOD {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_WPS_CONFIG_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_WPS_CONFIG_METHOD {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_WPS_CONFIG_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_WPS_CONFIG_METHOD").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct DOT11_WPS_DEVICE_PASSWORD_ID(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_PASSWORD_ID_DEFAULT: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_PASSWORD_ID_USER_SPECIFIED: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_PASSWORD_ID_MACHINE_SPECIFIED: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_PASSWORD_ID_REKEY: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_PASSWORD_ID_PUSHBUTTON: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_PASSWORD_ID_REGISTRAR_SPECIFIED: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_PASSWORD_ID_NFC_CONNECTION_HANDOVER: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_PASSWORD_ID_WFD_SERVICES: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_PASSWORD_ID_OOB_RANGE_MIN: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const DOT11_WPS_PASSWORD_ID_OOB_RANGE_MAX: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(65535i32);
impl ::core::marker::Copy for DOT11_WPS_DEVICE_PASSWORD_ID {}
impl ::core::clone::Clone for DOT11_WPS_DEVICE_PASSWORD_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for DOT11_WPS_DEVICE_PASSWORD_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for DOT11_WPS_DEVICE_PASSWORD_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for DOT11_WPS_DEVICE_PASSWORD_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DOT11_WPS_DEVICE_PASSWORD_ID").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ONEX_AUTH_IDENTITY(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXAuthIdentityNone: ONEX_AUTH_IDENTITY = ONEX_AUTH_IDENTITY(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXAuthIdentityMachine: ONEX_AUTH_IDENTITY = ONEX_AUTH_IDENTITY(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXAuthIdentityUser: ONEX_AUTH_IDENTITY = ONEX_AUTH_IDENTITY(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXAuthIdentityExplicitUser: ONEX_AUTH_IDENTITY = ONEX_AUTH_IDENTITY(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXAuthIdentityGuest: ONEX_AUTH_IDENTITY = ONEX_AUTH_IDENTITY(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXAuthIdentityInvalid: ONEX_AUTH_IDENTITY = ONEX_AUTH_IDENTITY(5i32);
impl ::core::marker::Copy for ONEX_AUTH_IDENTITY {}
impl ::core::clone::Clone for ONEX_AUTH_IDENTITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ONEX_AUTH_IDENTITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ONEX_AUTH_IDENTITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ONEX_AUTH_IDENTITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ONEX_AUTH_IDENTITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ONEX_AUTH_RESTART_REASON(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXRestartReasonPeerInitiated: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXRestartReasonMsmInitiated: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXRestartReasonOneXHeldStateTimeout: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXRestartReasonOneXAuthTimeout: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXRestartReasonOneXConfigurationChanged: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXRestartReasonOneXUserChanged: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXRestartReasonQuarantineStateChanged: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXRestartReasonAltCredsTrial: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXRestartReasonInvalid: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(8i32);
impl ::core::marker::Copy for ONEX_AUTH_RESTART_REASON {}
impl ::core::clone::Clone for ONEX_AUTH_RESTART_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ONEX_AUTH_RESTART_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ONEX_AUTH_RESTART_REASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ONEX_AUTH_RESTART_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ONEX_AUTH_RESTART_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ONEX_AUTH_STATUS(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXAuthNotStarted: ONEX_AUTH_STATUS = ONEX_AUTH_STATUS(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXAuthInProgress: ONEX_AUTH_STATUS = ONEX_AUTH_STATUS(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXAuthNoAuthenticatorFound: ONEX_AUTH_STATUS = ONEX_AUTH_STATUS(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXAuthSuccess: ONEX_AUTH_STATUS = ONEX_AUTH_STATUS(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXAuthFailure: ONEX_AUTH_STATUS = ONEX_AUTH_STATUS(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXAuthInvalid: ONEX_AUTH_STATUS = ONEX_AUTH_STATUS(5i32);
impl ::core::marker::Copy for ONEX_AUTH_STATUS {}
impl ::core::clone::Clone for ONEX_AUTH_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ONEX_AUTH_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ONEX_AUTH_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ONEX_AUTH_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ONEX_AUTH_STATUS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ONEX_EAP_METHOD_BACKEND_SUPPORT(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXEapMethodBackendSupportUnknown: ONEX_EAP_METHOD_BACKEND_SUPPORT = ONEX_EAP_METHOD_BACKEND_SUPPORT(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXEapMethodBackendSupported: ONEX_EAP_METHOD_BACKEND_SUPPORT = ONEX_EAP_METHOD_BACKEND_SUPPORT(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXEapMethodBackendUnsupported: ONEX_EAP_METHOD_BACKEND_SUPPORT = ONEX_EAP_METHOD_BACKEND_SUPPORT(2i32);
impl ::core::marker::Copy for ONEX_EAP_METHOD_BACKEND_SUPPORT {}
impl ::core::clone::Clone for ONEX_EAP_METHOD_BACKEND_SUPPORT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ONEX_EAP_METHOD_BACKEND_SUPPORT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ONEX_EAP_METHOD_BACKEND_SUPPORT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ONEX_EAP_METHOD_BACKEND_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ONEX_EAP_METHOD_BACKEND_SUPPORT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ONEX_NOTIFICATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXPublicNotificationBase: ONEX_NOTIFICATION_TYPE = ONEX_NOTIFICATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXNotificationTypeResultUpdate: ONEX_NOTIFICATION_TYPE = ONEX_NOTIFICATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXNotificationTypeAuthRestarted: ONEX_NOTIFICATION_TYPE = ONEX_NOTIFICATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXNotificationTypeEventInvalid: ONEX_NOTIFICATION_TYPE = ONEX_NOTIFICATION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const OneXNumNotifications: ONEX_NOTIFICATION_TYPE = ONEX_NOTIFICATION_TYPE(3i32);
impl ::core::marker::Copy for ONEX_NOTIFICATION_TYPE {}
impl ::core::clone::Clone for ONEX_NOTIFICATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ONEX_NOTIFICATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ONEX_NOTIFICATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ONEX_NOTIFICATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ONEX_NOTIFICATION_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct ONEX_REASON_CODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_REASON_CODE_SUCCESS: ONEX_REASON_CODE = ONEX_REASON_CODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_REASON_START: ONEX_REASON_CODE = ONEX_REASON_CODE(327680i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_UNABLE_TO_IDENTIFY_USER: ONEX_REASON_CODE = ONEX_REASON_CODE(327681i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_IDENTITY_NOT_FOUND: ONEX_REASON_CODE = ONEX_REASON_CODE(327682i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_UI_DISABLED: ONEX_REASON_CODE = ONEX_REASON_CODE(327683i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_UI_FAILURE: ONEX_REASON_CODE = ONEX_REASON_CODE(327684i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_EAP_FAILURE_RECEIVED: ONEX_REASON_CODE = ONEX_REASON_CODE(327685i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_AUTHENTICATOR_NO_LONGER_PRESENT: ONEX_REASON_CODE = ONEX_REASON_CODE(327686i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_NO_RESPONSE_TO_IDENTITY: ONEX_REASON_CODE = ONEX_REASON_CODE(327687i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_PROFILE_VERSION_NOT_SUPPORTED: ONEX_REASON_CODE = ONEX_REASON_CODE(327688i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_PROFILE_INVALID_LENGTH: ONEX_REASON_CODE = ONEX_REASON_CODE(327689i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_PROFILE_DISALLOWED_EAP_TYPE: ONEX_REASON_CODE = ONEX_REASON_CODE(327690i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_PROFILE_INVALID_EAP_TYPE_OR_FLAG: ONEX_REASON_CODE = ONEX_REASON_CODE(327691i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_PROFILE_INVALID_ONEX_FLAGS: ONEX_REASON_CODE = ONEX_REASON_CODE(327692i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_PROFILE_INVALID_TIMER_VALUE: ONEX_REASON_CODE = ONEX_REASON_CODE(327693i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_PROFILE_INVALID_SUPPLICANT_MODE: ONEX_REASON_CODE = ONEX_REASON_CODE(327694i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_PROFILE_INVALID_AUTH_MODE: ONEX_REASON_CODE = ONEX_REASON_CODE(327695i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_PROFILE_INVALID_EAP_CONNECTION_PROPERTIES: ONEX_REASON_CODE = ONEX_REASON_CODE(327696i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_UI_CANCELLED: ONEX_REASON_CODE = ONEX_REASON_CODE(327697i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_PROFILE_INVALID_EXPLICIT_CREDENTIALS: ONEX_REASON_CODE = ONEX_REASON_CODE(327698i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_PROFILE_EXPIRED_EXPLICIT_CREDENTIALS: ONEX_REASON_CODE = ONEX_REASON_CODE(327699i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const ONEX_UI_NOT_PERMITTED: ONEX_REASON_CODE = ONEX_REASON_CODE(327700i32);
impl ::core::marker::Copy for ONEX_REASON_CODE {}
impl ::core::clone::Clone for ONEX_REASON_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for ONEX_REASON_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for ONEX_REASON_CODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for ONEX_REASON_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ONEX_REASON_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WFD_ROLE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WFD_ROLE_TYPE_NONE: WFD_ROLE_TYPE = WFD_ROLE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WFD_ROLE_TYPE_DEVICE: WFD_ROLE_TYPE = WFD_ROLE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WFD_ROLE_TYPE_GROUP_OWNER: WFD_ROLE_TYPE = WFD_ROLE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WFD_ROLE_TYPE_CLIENT: WFD_ROLE_TYPE = WFD_ROLE_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WFD_ROLE_TYPE_MAX: WFD_ROLE_TYPE = WFD_ROLE_TYPE(5i32);
impl ::core::marker::Copy for WFD_ROLE_TYPE {}
impl ::core::clone::Clone for WFD_ROLE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WFD_ROLE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WFD_ROLE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WFD_ROLE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WFD_ROLE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_ADHOC_NETWORK_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_adhoc_network_state_formed: WLAN_ADHOC_NETWORK_STATE = WLAN_ADHOC_NETWORK_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_adhoc_network_state_connected: WLAN_ADHOC_NETWORK_STATE = WLAN_ADHOC_NETWORK_STATE(1i32);
impl ::core::marker::Copy for WLAN_ADHOC_NETWORK_STATE {}
impl ::core::clone::Clone for WLAN_ADHOC_NETWORK_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_ADHOC_NETWORK_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_ADHOC_NETWORK_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_ADHOC_NETWORK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_ADHOC_NETWORK_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_AUTOCONF_OPCODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_autoconf_opcode_start: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_autoconf_opcode_show_denied_networks: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_autoconf_opcode_power_setting: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_autoconf_opcode_only_use_gp_profiles_for_allowed_networks: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_autoconf_opcode_allow_explicit_creds: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_autoconf_opcode_block_period: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_autoconf_opcode_allow_virtual_station_extensibility: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_autoconf_opcode_end: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(7i32);
impl ::core::marker::Copy for WLAN_AUTOCONF_OPCODE {}
impl ::core::clone::Clone for WLAN_AUTOCONF_OPCODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_AUTOCONF_OPCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_AUTOCONF_OPCODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_AUTOCONF_OPCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_AUTOCONF_OPCODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_CONNECTION_MODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_connection_mode_profile: WLAN_CONNECTION_MODE = WLAN_CONNECTION_MODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_connection_mode_temporary_profile: WLAN_CONNECTION_MODE = WLAN_CONNECTION_MODE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_connection_mode_discovery_secure: WLAN_CONNECTION_MODE = WLAN_CONNECTION_MODE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_connection_mode_discovery_unsecure: WLAN_CONNECTION_MODE = WLAN_CONNECTION_MODE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_connection_mode_auto: WLAN_CONNECTION_MODE = WLAN_CONNECTION_MODE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_connection_mode_invalid: WLAN_CONNECTION_MODE = WLAN_CONNECTION_MODE(5i32);
impl ::core::marker::Copy for WLAN_CONNECTION_MODE {}
impl ::core::clone::Clone for WLAN_CONNECTION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_CONNECTION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_CONNECTION_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_CONNECTION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_CONNECTION_MODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_CONNECTION_NOTIFICATION_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_CONNECTION_NOTIFICATION_ADHOC_NETWORK_FORMED: WLAN_CONNECTION_NOTIFICATION_FLAGS = WLAN_CONNECTION_NOTIFICATION_FLAGS(1u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_CONNECTION_NOTIFICATION_CONSOLE_USER_PROFILE: WLAN_CONNECTION_NOTIFICATION_FLAGS = WLAN_CONNECTION_NOTIFICATION_FLAGS(4u32);
impl ::core::marker::Copy for WLAN_CONNECTION_NOTIFICATION_FLAGS {}
impl ::core::clone::Clone for WLAN_CONNECTION_NOTIFICATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_CONNECTION_NOTIFICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_CONNECTION_NOTIFICATION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_CONNECTION_NOTIFICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_CONNECTION_NOTIFICATION_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_FILTER_LIST_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_filter_list_type_gp_permit: WLAN_FILTER_LIST_TYPE = WLAN_FILTER_LIST_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_filter_list_type_gp_deny: WLAN_FILTER_LIST_TYPE = WLAN_FILTER_LIST_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_filter_list_type_user_permit: WLAN_FILTER_LIST_TYPE = WLAN_FILTER_LIST_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_filter_list_type_user_deny: WLAN_FILTER_LIST_TYPE = WLAN_FILTER_LIST_TYPE(3i32);
impl ::core::marker::Copy for WLAN_FILTER_LIST_TYPE {}
impl ::core::clone::Clone for WLAN_FILTER_LIST_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_FILTER_LIST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_FILTER_LIST_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_FILTER_LIST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_FILTER_LIST_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_HOSTED_NETWORK_NOTIFICATION_CODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_state_change: WLAN_HOSTED_NETWORK_NOTIFICATION_CODE = WLAN_HOSTED_NETWORK_NOTIFICATION_CODE(4096i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_peer_state_change: WLAN_HOSTED_NETWORK_NOTIFICATION_CODE = WLAN_HOSTED_NETWORK_NOTIFICATION_CODE(4097i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_radio_state_change: WLAN_HOSTED_NETWORK_NOTIFICATION_CODE = WLAN_HOSTED_NETWORK_NOTIFICATION_CODE(4098i32);
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_NOTIFICATION_CODE {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_NOTIFICATION_CODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_NOTIFICATION_CODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_HOSTED_NETWORK_NOTIFICATION_CODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_NOTIFICATION_CODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_HOSTED_NETWORK_NOTIFICATION_CODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_HOSTED_NETWORK_OPCODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_opcode_connection_settings: WLAN_HOSTED_NETWORK_OPCODE = WLAN_HOSTED_NETWORK_OPCODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_opcode_security_settings: WLAN_HOSTED_NETWORK_OPCODE = WLAN_HOSTED_NETWORK_OPCODE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_opcode_station_profile: WLAN_HOSTED_NETWORK_OPCODE = WLAN_HOSTED_NETWORK_OPCODE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_opcode_enable: WLAN_HOSTED_NETWORK_OPCODE = WLAN_HOSTED_NETWORK_OPCODE(3i32);
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_OPCODE {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_OPCODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_OPCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_HOSTED_NETWORK_OPCODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_OPCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_HOSTED_NETWORK_OPCODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_HOSTED_NETWORK_PEER_AUTH_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_peer_state_invalid: WLAN_HOSTED_NETWORK_PEER_AUTH_STATE = WLAN_HOSTED_NETWORK_PEER_AUTH_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_peer_state_authenticated: WLAN_HOSTED_NETWORK_PEER_AUTH_STATE = WLAN_HOSTED_NETWORK_PEER_AUTH_STATE(1i32);
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_PEER_AUTH_STATE {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_PEER_AUTH_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_PEER_AUTH_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_HOSTED_NETWORK_PEER_AUTH_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_PEER_AUTH_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_HOSTED_NETWORK_PEER_AUTH_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_HOSTED_NETWORK_REASON(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_success: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_unspecified: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_bad_parameters: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_service_shutting_down: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_insufficient_resources: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_elevation_required: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_read_only: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_persistence_failed: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_crypt_error: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_impersonation: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_stop_before_start: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_interface_available: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_interface_unavailable: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_miniport_stopped: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(13i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_miniport_started: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(14i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_incompatible_connection_started: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(15i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_incompatible_connection_stopped: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_user_action: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(17i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_client_abort: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(18i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_ap_start_failed: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(19i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_peer_arrived: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(20i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_peer_departed: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(21i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_peer_timeout: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(22i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_gp_denied: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(23i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_service_unavailable: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(24i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_device_change: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(25i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_properties_change: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(26i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_virtual_station_blocking_use: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(27i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_reason_service_available_on_virtual_station: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(28i32);
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_REASON {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_REASON {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_REASON {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_HOSTED_NETWORK_REASON {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_REASON {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_HOSTED_NETWORK_REASON").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_HOSTED_NETWORK_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_unavailable: WLAN_HOSTED_NETWORK_STATE = WLAN_HOSTED_NETWORK_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_idle: WLAN_HOSTED_NETWORK_STATE = WLAN_HOSTED_NETWORK_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_hosted_network_active: WLAN_HOSTED_NETWORK_STATE = WLAN_HOSTED_NETWORK_STATE(2i32);
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_STATE {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_HOSTED_NETWORK_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_HOSTED_NETWORK_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_IHV_CONTROL_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_ihv_control_type_service: WLAN_IHV_CONTROL_TYPE = WLAN_IHV_CONTROL_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_ihv_control_type_driver: WLAN_IHV_CONTROL_TYPE = WLAN_IHV_CONTROL_TYPE(1i32);
impl ::core::marker::Copy for WLAN_IHV_CONTROL_TYPE {}
impl ::core::clone::Clone for WLAN_IHV_CONTROL_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_IHV_CONTROL_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_IHV_CONTROL_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_IHV_CONTROL_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_IHV_CONTROL_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_INTERFACE_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_interface_state_not_ready: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_interface_state_connected: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_interface_state_ad_hoc_network_formed: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_interface_state_disconnecting: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_interface_state_disconnected: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_interface_state_associating: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_interface_state_discovering: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_interface_state_authenticating: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(7i32);
impl ::core::marker::Copy for WLAN_INTERFACE_STATE {}
impl ::core::clone::Clone for WLAN_INTERFACE_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_INTERFACE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_INTERFACE_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_INTERFACE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_INTERFACE_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_INTERFACE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_interface_type_emulated_802_11: WLAN_INTERFACE_TYPE = WLAN_INTERFACE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_interface_type_native_802_11: WLAN_INTERFACE_TYPE = WLAN_INTERFACE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_interface_type_invalid: WLAN_INTERFACE_TYPE = WLAN_INTERFACE_TYPE(2i32);
impl ::core::marker::Copy for WLAN_INTERFACE_TYPE {}
impl ::core::clone::Clone for WLAN_INTERFACE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_INTERFACE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_INTERFACE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_INTERFACE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_INTERFACE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_INTF_OPCODE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_autoconf_start: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_autoconf_enabled: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_background_scan_enabled: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_media_streaming_mode: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_radio_state: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_bss_type: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_interface_state: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_current_connection: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_channel_number: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_supported_infrastructure_auth_cipher_pairs: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_supported_adhoc_auth_cipher_pairs: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_supported_country_or_region_string_list: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_current_operation_mode: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_supported_safe_mode: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(13i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_certified_safe_mode: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(14i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_hosted_network_capable: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(15i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_management_frame_protection_capable: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_secondary_sta_interfaces: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(17i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_secondary_sta_synchronized_connections: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(18i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_autoconf_end: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(268435455i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_msm_start: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(268435712i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_statistics: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(268435713i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_rssi: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(268435714i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_msm_end: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(536870911i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_security_start: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(536936448i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_security_end: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(805306367i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_ihv_start: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(805306368i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_intf_opcode_ihv_end: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(1073741823i32);
impl ::core::marker::Copy for WLAN_INTF_OPCODE {}
impl ::core::clone::Clone for WLAN_INTF_OPCODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_INTF_OPCODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_INTF_OPCODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_INTF_OPCODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_INTF_OPCODE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_NOTIFICATION_ACM(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_start: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_autoconf_enabled: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_autoconf_disabled: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_background_scan_enabled: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_background_scan_disabled: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_bss_type_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_power_setting_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_scan_complete: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_scan_fail: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_connection_start: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_connection_complete: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_connection_attempt_fail: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_filter_list_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_interface_arrival: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(13i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_interface_removal: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(14i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_profile_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(15i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_profile_name_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_profiles_exhausted: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(17i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_network_not_available: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(18i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_network_available: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(19i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_disconnecting: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(20i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_disconnected: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(21i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_adhoc_network_state_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(22i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_profile_unblocked: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(23i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_screen_power_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(24i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_profile_blocked: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(25i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_scan_list_refresh: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(26i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_operational_state_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(27i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_acm_end: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(28i32);
impl ::core::marker::Copy for WLAN_NOTIFICATION_ACM {}
impl ::core::clone::Clone for WLAN_NOTIFICATION_ACM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_NOTIFICATION_ACM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_NOTIFICATION_ACM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_NOTIFICATION_ACM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_NOTIFICATION_ACM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_NOTIFICATION_MSM(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_start: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_associating: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_associated: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_authenticating: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_connected: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_roaming_start: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_roaming_end: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_radio_state_change: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_signal_quality_change: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_disassociating: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_disconnected: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_peer_join: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_peer_leave: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_adapter_removal: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(13i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_adapter_operation_mode_change: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(14i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_link_degraded: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(15i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_link_improved: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_msm_end: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(17i32);
impl ::core::marker::Copy for WLAN_NOTIFICATION_MSM {}
impl ::core::clone::Clone for WLAN_NOTIFICATION_MSM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_NOTIFICATION_MSM {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_NOTIFICATION_MSM {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_NOTIFICATION_MSM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_NOTIFICATION_MSM").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_NOTIFICATION_SECURITY(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_security_start: WLAN_NOTIFICATION_SECURITY = WLAN_NOTIFICATION_SECURITY(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_notification_security_end: WLAN_NOTIFICATION_SECURITY = WLAN_NOTIFICATION_SECURITY(1i32);
impl ::core::marker::Copy for WLAN_NOTIFICATION_SECURITY {}
impl ::core::clone::Clone for WLAN_NOTIFICATION_SECURITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_NOTIFICATION_SECURITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_NOTIFICATION_SECURITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_NOTIFICATION_SECURITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_NOTIFICATION_SECURITY").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_OPCODE_VALUE_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_opcode_value_type_query_only: WLAN_OPCODE_VALUE_TYPE = WLAN_OPCODE_VALUE_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_opcode_value_type_set_by_group_policy: WLAN_OPCODE_VALUE_TYPE = WLAN_OPCODE_VALUE_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_opcode_value_type_set_by_user: WLAN_OPCODE_VALUE_TYPE = WLAN_OPCODE_VALUE_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_opcode_value_type_invalid: WLAN_OPCODE_VALUE_TYPE = WLAN_OPCODE_VALUE_TYPE(3i32);
impl ::core::marker::Copy for WLAN_OPCODE_VALUE_TYPE {}
impl ::core::clone::Clone for WLAN_OPCODE_VALUE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_OPCODE_VALUE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_OPCODE_VALUE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_OPCODE_VALUE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_OPCODE_VALUE_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_OPERATIONAL_STATE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_operational_state_unknown: WLAN_OPERATIONAL_STATE = WLAN_OPERATIONAL_STATE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_operational_state_off: WLAN_OPERATIONAL_STATE = WLAN_OPERATIONAL_STATE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_operational_state_on: WLAN_OPERATIONAL_STATE = WLAN_OPERATIONAL_STATE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_operational_state_going_off: WLAN_OPERATIONAL_STATE = WLAN_OPERATIONAL_STATE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_operational_state_going_on: WLAN_OPERATIONAL_STATE = WLAN_OPERATIONAL_STATE(4i32);
impl ::core::marker::Copy for WLAN_OPERATIONAL_STATE {}
impl ::core::clone::Clone for WLAN_OPERATIONAL_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_OPERATIONAL_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_OPERATIONAL_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_OPERATIONAL_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_OPERATIONAL_STATE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_POWER_SETTING(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_power_setting_no_saving: WLAN_POWER_SETTING = WLAN_POWER_SETTING(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_power_setting_low_saving: WLAN_POWER_SETTING = WLAN_POWER_SETTING(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_power_setting_medium_saving: WLAN_POWER_SETTING = WLAN_POWER_SETTING(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_power_setting_maximum_saving: WLAN_POWER_SETTING = WLAN_POWER_SETTING(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_power_setting_invalid: WLAN_POWER_SETTING = WLAN_POWER_SETTING(4i32);
impl ::core::marker::Copy for WLAN_POWER_SETTING {}
impl ::core::clone::Clone for WLAN_POWER_SETTING {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_POWER_SETTING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_POWER_SETTING {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_POWER_SETTING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_POWER_SETTING").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_SECURABLE_OBJECT(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_permit_list: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_deny_list: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_ac_enabled: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_bc_scan_enabled: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_bss_type: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_show_denied: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_interface_properties: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(6i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_ihv_control: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(7i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_all_user_profiles_order: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(8i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_add_new_all_user_profiles: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(9i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_add_new_per_user_profiles: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(10i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_media_streaming_mode_enabled: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(11i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_current_operation_mode: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(12i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_get_plaintext_key: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(13i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_hosted_network_elevated_access: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(14i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_virtual_station_extensibility: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(15i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const wlan_secure_wfd_elevated_access: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(16i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_SECURABLE_OBJECT_COUNT: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(17i32);
impl ::core::marker::Copy for WLAN_SECURABLE_OBJECT {}
impl ::core::clone::Clone for WLAN_SECURABLE_OBJECT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_SECURABLE_OBJECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_SECURABLE_OBJECT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_SECURABLE_OBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_SECURABLE_OBJECT").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WLAN_SET_EAPHOST_FLAGS(pub u32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAN_SET_EAPHOST_DATA_ALL_USERS: WLAN_SET_EAPHOST_FLAGS = WLAN_SET_EAPHOST_FLAGS(1u32);
impl ::core::marker::Copy for WLAN_SET_EAPHOST_FLAGS {}
impl ::core::clone::Clone for WLAN_SET_EAPHOST_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WLAN_SET_EAPHOST_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WLAN_SET_EAPHOST_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WLAN_SET_EAPHOST_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WLAN_SET_EAPHOST_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct WL_DISPLAY_PAGES(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLConnectionPage: WL_DISPLAY_PAGES = WL_DISPLAY_PAGES(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLSecurityPage: WL_DISPLAY_PAGES = WL_DISPLAY_PAGES(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub const WLAdvPage: WL_DISPLAY_PAGES = WL_DISPLAY_PAGES(2i32);
impl ::core::marker::Copy for WL_DISPLAY_PAGES {}
impl ::core::clone::Clone for WL_DISPLAY_PAGES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WL_DISPLAY_PAGES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for WL_DISPLAY_PAGES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for WL_DISPLAY_PAGES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WL_DISPLAY_PAGES").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub struct DOT11EXT_APIS {
    pub Dot11ExtAllocateBuffer: DOT11EXT_ALLOCATE_BUFFER,
    pub Dot11ExtFreeBuffer: DOT11EXT_FREE_BUFFER,
    pub Dot11ExtSetProfileCustomUserData: DOT11EXT_SET_PROFILE_CUSTOM_USER_DATA,
    pub Dot11ExtGetProfileCustomUserData: DOT11EXT_GET_PROFILE_CUSTOM_USER_DATA,
    pub Dot11ExtSetCurrentProfile: DOT11EXT_SET_CURRENT_PROFILE,
    pub Dot11ExtSendUIRequest: DOT11EXT_SEND_UI_REQUEST,
    pub Dot11ExtPreAssociateCompletion: DOT11EXT_PRE_ASSOCIATE_COMPLETION,
    pub Dot11ExtPostAssociateCompletion: DOT11EXT_POST_ASSOCIATE_COMPLETION,
    pub Dot11ExtSendNotification: DOT11EXT_SEND_NOTIFICATION,
    pub Dot11ExtSendPacket: DOT11EXT_SEND_PACKET,
    pub Dot11ExtSetEtherTypeHandling: DOT11EXT_SET_ETHERTYPE_HANDLING,
    pub Dot11ExtSetAuthAlgorithm: DOT11EXT_SET_AUTH_ALGORITHM,
    pub Dot11ExtSetUnicastCipherAlgorithm: DOT11EXT_SET_UNICAST_CIPHER_ALGORITHM,
    pub Dot11ExtSetMulticastCipherAlgorithm: DOT11EXT_SET_MULTICAST_CIPHER_ALGORITHM,
    pub Dot11ExtSetDefaultKey: DOT11EXT_SET_DEFAULT_KEY,
    pub Dot11ExtSetKeyMappingKey: DOT11EXT_SET_KEY_MAPPING_KEY,
    pub Dot11ExtSetDefaultKeyId: DOT11EXT_SET_DEFAULT_KEY_ID,
    pub Dot11ExtNicSpecificExtension: DOT11EXT_NIC_SPECIFIC_EXTENSION,
    pub Dot11ExtSetExcludeUnencrypted: DOT11EXT_SET_EXCLUDE_UNENCRYPTED,
    pub Dot11ExtStartOneX: DOT11EXT_ONEX_START,
    pub Dot11ExtStopOneX: DOT11EXT_ONEX_STOP,
    pub Dot11ExtProcessSecurityPacket: DOT11EXT_PROCESS_ONEX_PACKET,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::marker::Copy for DOT11EXT_APIS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::clone::Clone for DOT11EXT_APIS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::fmt::Debug for DOT11EXT_APIS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_APIS").finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::windows::core::TypeKind for DOT11EXT_APIS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::default::Default for DOT11EXT_APIS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    pub pszXmlFragmentIhvConnectivity: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for DOT11EXT_IHV_CONNECTIVITY_PROFILE {}
impl ::core::clone::Clone for DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_CONNECTIVITY_PROFILE").field("pszXmlFragmentIhvConnectivity", &self.pszXmlFragmentIhvConnectivity).finish()
    }
}
impl ::windows::core::TypeKind for DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.pszXmlFragmentIhvConnectivity == other.pszXmlFragmentIhvConnectivity
    }
}
impl ::core::cmp::Eq for DOT11EXT_IHV_CONNECTIVITY_PROFILE {}
impl ::core::default::Default for DOT11EXT_IHV_CONNECTIVITY_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11EXT_IHV_DISCOVERY_PROFILE {
    pub IhvConnectivityProfile: DOT11EXT_IHV_CONNECTIVITY_PROFILE,
    pub IhvSecurityProfile: DOT11EXT_IHV_SECURITY_PROFILE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11EXT_IHV_DISCOVERY_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11EXT_IHV_DISCOVERY_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11EXT_IHV_DISCOVERY_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_DISCOVERY_PROFILE").field("IhvConnectivityProfile", &self.IhvConnectivityProfile).field("IhvSecurityProfile", &self.IhvSecurityProfile).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11EXT_IHV_DISCOVERY_PROFILE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11EXT_IHV_DISCOVERY_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.IhvConnectivityProfile == other.IhvConnectivityProfile && self.IhvSecurityProfile == other.IhvSecurityProfile
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11EXT_IHV_DISCOVERY_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11EXT_IHV_DISCOVERY_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    pub dwCount: u32,
    pub pIhvDiscoveryProfiles: *mut DOT11EXT_IHV_DISCOVERY_PROFILE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_DISCOVERY_PROFILE_LIST").field("dwCount", &self.dwCount).field("pIhvDiscoveryProfiles", &self.pIhvDiscoveryProfiles).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwCount == other.dwCount && self.pIhvDiscoveryProfiles == other.pIhvDiscoveryProfiles
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11EXT_IHV_DISCOVERY_PROFILE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_System_RemoteDesktop\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
pub struct DOT11EXT_IHV_HANDLERS {
    pub Dot11ExtIhvDeinitService: DOT11EXTIHV_DEINIT_SERVICE,
    pub Dot11ExtIhvInitAdapter: DOT11EXTIHV_INIT_ADAPTER,
    pub Dot11ExtIhvDeinitAdapter: DOT11EXTIHV_DEINIT_ADAPTER,
    pub Dot11ExtIhvPerformPreAssociate: DOT11EXTIHV_PERFORM_PRE_ASSOCIATE,
    pub Dot11ExtIhvAdapterReset: DOT11EXTIHV_ADAPTER_RESET,
    pub Dot11ExtIhvPerformPostAssociate: DOT11EXTIHV_PERFORM_POST_ASSOCIATE,
    pub Dot11ExtIhvStopPostAssociate: DOT11EXTIHV_STOP_POST_ASSOCIATE,
    pub Dot11ExtIhvValidateProfile: DOT11EXTIHV_VALIDATE_PROFILE,
    pub Dot11ExtIhvPerformCapabilityMatch: DOT11EXTIHV_PERFORM_CAPABILITY_MATCH,
    pub Dot11ExtIhvCreateDiscoveryProfiles: DOT11EXTIHV_CREATE_DISCOVERY_PROFILES,
    pub Dot11ExtIhvProcessSessionChange: DOT11EXTIHV_PROCESS_SESSION_CHANGE,
    pub Dot11ExtIhvReceiveIndication: DOT11EXTIHV_RECEIVE_INDICATION,
    pub Dot11ExtIhvReceivePacket: DOT11EXTIHV_RECEIVE_PACKET,
    pub Dot11ExtIhvSendPacketCompletion: DOT11EXTIHV_SEND_PACKET_COMPLETION,
    pub Dot11ExtIhvIsUIRequestPending: DOT11EXTIHV_IS_UI_REQUEST_PENDING,
    pub Dot11ExtIhvProcessUIResponse: DOT11EXTIHV_PROCESS_UI_RESPONSE,
    pub Dot11ExtIhvQueryUIRequest: DOT11EXTIHV_QUERY_UI_REQUEST,
    pub Dot11ExtIhvOnexIndicateResult: DOT11EXTIHV_ONEX_INDICATE_RESULT,
    pub Dot11ExtIhvControl: DOT11EXTIHV_CONTROL,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
impl ::core::marker::Copy for DOT11EXT_IHV_HANDLERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
impl ::core::clone::Clone for DOT11EXT_IHV_HANDLERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
impl ::core::fmt::Debug for DOT11EXT_IHV_HANDLERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_HANDLERS").finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
impl ::windows::core::TypeKind for DOT11EXT_IHV_HANDLERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
impl ::core::default::Default for DOT11EXT_IHV_HANDLERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub struct DOT11EXT_IHV_PARAMS {
    pub dot11ExtIhvProfileParams: DOT11EXT_IHV_PROFILE_PARAMS,
    pub wstrProfileName: [u16; 256],
    pub dwProfileTypeFlags: u32,
    pub interfaceGuid: ::windows::core::GUID,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::marker::Copy for DOT11EXT_IHV_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::clone::Clone for DOT11EXT_IHV_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::fmt::Debug for DOT11EXT_IHV_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_PARAMS").field("dot11ExtIhvProfileParams", &self.dot11ExtIhvProfileParams).field("wstrProfileName", &self.wstrProfileName).field("dwProfileTypeFlags", &self.dwProfileTypeFlags).field("interfaceGuid", &self.interfaceGuid).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::windows::core::TypeKind for DOT11EXT_IHV_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::cmp::PartialEq for DOT11EXT_IHV_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.dot11ExtIhvProfileParams == other.dot11ExtIhvProfileParams && self.wstrProfileName == other.wstrProfileName && self.dwProfileTypeFlags == other.dwProfileTypeFlags && self.interfaceGuid == other.interfaceGuid
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::cmp::Eq for DOT11EXT_IHV_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::default::Default for DOT11EXT_IHV_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub struct DOT11EXT_IHV_PROFILE_PARAMS {
    pub pSsidList: *mut DOT11EXT_IHV_SSID_LIST,
    pub BssType: DOT11_BSS_TYPE,
    pub pMSSecuritySettings: *mut DOT11_MSSECURITY_SETTINGS,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::marker::Copy for DOT11EXT_IHV_PROFILE_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::clone::Clone for DOT11EXT_IHV_PROFILE_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::fmt::Debug for DOT11EXT_IHV_PROFILE_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_PROFILE_PARAMS").field("pSsidList", &self.pSsidList).field("BssType", &self.BssType).field("pMSSecuritySettings", &self.pMSSecuritySettings).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::windows::core::TypeKind for DOT11EXT_IHV_PROFILE_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::cmp::PartialEq for DOT11EXT_IHV_PROFILE_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.pSsidList == other.pSsidList && self.BssType == other.BssType && self.pMSSecuritySettings == other.pMSSecuritySettings
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::cmp::Eq for DOT11EXT_IHV_PROFILE_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::default::Default for DOT11EXT_IHV_PROFILE_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11EXT_IHV_SECURITY_PROFILE {
    pub pszXmlFragmentIhvSecurity: ::windows::core::PWSTR,
    pub bUseMSOnex: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11EXT_IHV_SECURITY_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11EXT_IHV_SECURITY_PROFILE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11EXT_IHV_SECURITY_PROFILE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_SECURITY_PROFILE").field("pszXmlFragmentIhvSecurity", &self.pszXmlFragmentIhvSecurity).field("bUseMSOnex", &self.bUseMSOnex).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11EXT_IHV_SECURITY_PROFILE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11EXT_IHV_SECURITY_PROFILE {
    fn eq(&self, other: &Self) -> bool {
        self.pszXmlFragmentIhvSecurity == other.pszXmlFragmentIhvSecurity && self.bUseMSOnex == other.bUseMSOnex
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11EXT_IHV_SECURITY_PROFILE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11EXT_IHV_SECURITY_PROFILE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11EXT_IHV_SSID_LIST {
    pub ulCount: u32,
    pub SSIDs: [DOT11_SSID; 1],
}
impl ::core::marker::Copy for DOT11EXT_IHV_SSID_LIST {}
impl ::core::clone::Clone for DOT11EXT_IHV_SSID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11EXT_IHV_SSID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_SSID_LIST").field("ulCount", &self.ulCount).field("SSIDs", &self.SSIDs).finish()
    }
}
impl ::windows::core::TypeKind for DOT11EXT_IHV_SSID_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11EXT_IHV_SSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.ulCount == other.ulCount && self.SSIDs == other.SSIDs
    }
}
impl ::core::cmp::Eq for DOT11EXT_IHV_SSID_LIST {}
impl ::core::default::Default for DOT11EXT_IHV_SSID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11EXT_IHV_UI_REQUEST {
    pub dwSessionId: u32,
    pub guidUIRequest: ::windows::core::GUID,
    pub UIPageClsid: ::windows::core::GUID,
    pub dwByteCount: u32,
    pub pvUIRequest: *mut u8,
}
impl ::core::marker::Copy for DOT11EXT_IHV_UI_REQUEST {}
impl ::core::clone::Clone for DOT11EXT_IHV_UI_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11EXT_IHV_UI_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_IHV_UI_REQUEST").field("dwSessionId", &self.dwSessionId).field("guidUIRequest", &self.guidUIRequest).field("UIPageClsid", &self.UIPageClsid).field("dwByteCount", &self.dwByteCount).field("pvUIRequest", &self.pvUIRequest).finish()
    }
}
impl ::windows::core::TypeKind for DOT11EXT_IHV_UI_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11EXT_IHV_UI_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.dwSessionId == other.dwSessionId && self.guidUIRequest == other.guidUIRequest && self.UIPageClsid == other.UIPageClsid && self.dwByteCount == other.dwByteCount && self.pvUIRequest == other.pvUIRequest
    }
}
impl ::core::cmp::Eq for DOT11EXT_IHV_UI_REQUEST {}
impl ::core::default::Default for DOT11EXT_IHV_UI_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11EXT_VIRTUAL_STATION_APIS {
    pub Dot11ExtRequestVirtualStation: DOT11EXT_REQUEST_VIRTUAL_STATION,
    pub Dot11ExtReleaseVirtualStation: DOT11EXT_RELEASE_VIRTUAL_STATION,
    pub Dot11ExtQueryVirtualStationProperties: DOT11EXT_QUERY_VIRTUAL_STATION_PROPERTIES,
    pub Dot11ExtSetVirtualStationAPProperties: DOT11EXT_SET_VIRTUAL_STATION_AP_PROPERTIES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11EXT_VIRTUAL_STATION_APIS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11EXT_VIRTUAL_STATION_APIS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11EXT_VIRTUAL_STATION_APIS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_VIRTUAL_STATION_APIS").finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11EXT_VIRTUAL_STATION_APIS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11EXT_VIRTUAL_STATION_APIS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    pub dot11SSID: DOT11_SSID,
    pub dot11AuthAlgo: DOT11_AUTH_ALGORITHM,
    pub dot11CipherAlgo: DOT11_CIPHER_ALGORITHM,
    pub bIsPassPhrase: super::super::Foundation::BOOL,
    pub dwKeyLength: u32,
    pub ucKeyData: [u8; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11EXT_VIRTUAL_STATION_AP_PROPERTY").field("dot11SSID", &self.dot11SSID).field("dot11AuthAlgo", &self.dot11AuthAlgo).field("dot11CipherAlgo", &self.dot11CipherAlgo).field("bIsPassPhrase", &self.bIsPassPhrase).field("dwKeyLength", &self.dwKeyLength).field("ucKeyData", &self.ucKeyData).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    fn eq(&self, other: &Self) -> bool {
        self.dot11SSID == other.dot11SSID && self.dot11AuthAlgo == other.dot11AuthAlgo && self.dot11CipherAlgo == other.dot11CipherAlgo && self.bIsPassPhrase == other.bIsPassPhrase && self.dwKeyLength == other.dwKeyLength && self.ucKeyData == other.ucKeyData
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11EXT_VIRTUAL_STATION_AP_PROPERTY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_ACCESSNETWORKOPTIONS {
    pub AccessNetworkType: u8,
    pub Internet: u8,
    pub ASRA: u8,
    pub ESR: u8,
    pub UESA: u8,
}
impl ::core::marker::Copy for DOT11_ACCESSNETWORKOPTIONS {}
impl ::core::clone::Clone for DOT11_ACCESSNETWORKOPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_ACCESSNETWORKOPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ACCESSNETWORKOPTIONS").field("AccessNetworkType", &self.AccessNetworkType).field("Internet", &self.Internet).field("ASRA", &self.ASRA).field("ESR", &self.ESR).field("UESA", &self.UESA).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_ACCESSNETWORKOPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_ACCESSNETWORKOPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.AccessNetworkType == other.AccessNetworkType && self.Internet == other.Internet && self.ASRA == other.ASRA && self.ESR == other.ESR && self.UESA == other.UESA
    }
}
impl ::core::cmp::Eq for DOT11_ACCESSNETWORKOPTIONS {}
impl ::core::default::Default for DOT11_ACCESSNETWORKOPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_ADAPTER {
    pub gAdapterId: ::windows::core::GUID,
    pub pszDescription: ::windows::core::PWSTR,
    pub Dot11CurrentOpMode: DOT11_CURRENT_OPERATION_MODE,
}
impl ::core::marker::Copy for DOT11_ADAPTER {}
impl ::core::clone::Clone for DOT11_ADAPTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_ADAPTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ADAPTER").field("gAdapterId", &self.gAdapterId).field("pszDescription", &self.pszDescription).field("Dot11CurrentOpMode", &self.Dot11CurrentOpMode).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_ADAPTER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_ADAPTER {
    fn eq(&self, other: &Self) -> bool {
        self.gAdapterId == other.gAdapterId && self.pszDescription == other.pszDescription && self.Dot11CurrentOpMode == other.Dot11CurrentOpMode
    }
}
impl ::core::cmp::Eq for DOT11_ADAPTER {}
impl ::core::default::Default for DOT11_ADAPTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_ADDITIONAL_IE {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uBeaconIEsOffset: u32,
    pub uBeaconIEsLength: u32,
    pub uResponseIEsOffset: u32,
    pub uResponseIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_ADDITIONAL_IE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_ADDITIONAL_IE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_ADDITIONAL_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ADDITIONAL_IE").field("Header", &self.Header).field("uBeaconIEsOffset", &self.uBeaconIEsOffset).field("uBeaconIEsLength", &self.uBeaconIEsLength).field("uResponseIEsOffset", &self.uResponseIEsOffset).field("uResponseIEsLength", &self.uResponseIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_ADDITIONAL_IE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ADDITIONAL_IE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uBeaconIEsOffset == other.uBeaconIEsOffset && self.uBeaconIEsLength == other.uBeaconIEsLength && self.uResponseIEsOffset == other.uResponseIEsOffset && self.uResponseIEsLength == other.uResponseIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ADDITIONAL_IE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ADDITIONAL_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub Status: DOT11_ANQP_QUERY_RESULT,
    pub hContext: super::super::Foundation::HANDLE,
    pub uResponseLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ANQP_QUERY_COMPLETE_PARAMETERS").field("Header", &self.Header).field("Status", &self.Status).field("hContext", &self.hContext).field("uResponseLength", &self.uResponseLength).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Status == other.Status && self.hContext == other.hContext && self.uResponseLength == other.uResponseLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_ANQP_QUERY_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_AP_JOIN_REQUEST {
    pub uJoinFailureTimeout: u32,
    pub OperationalRateSet: DOT11_RATE_SET,
    pub uChCenterFrequency: u32,
    pub dot11BSSDescription: DOT11_BSS_DESCRIPTION,
}
impl ::core::marker::Copy for DOT11_AP_JOIN_REQUEST {}
impl ::core::clone::Clone for DOT11_AP_JOIN_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_AP_JOIN_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_AP_JOIN_REQUEST").field("uJoinFailureTimeout", &self.uJoinFailureTimeout).field("OperationalRateSet", &self.OperationalRateSet).field("uChCenterFrequency", &self.uChCenterFrequency).field("dot11BSSDescription", &self.dot11BSSDescription).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_AP_JOIN_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_AP_JOIN_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.uJoinFailureTimeout == other.uJoinFailureTimeout && self.OperationalRateSet == other.OperationalRateSet && self.uChCenterFrequency == other.uChCenterFrequency && self.dot11BSSDescription == other.dot11BSSDescription
    }
}
impl ::core::cmp::Eq for DOT11_AP_JOIN_REQUEST {}
impl ::core::default::Default for DOT11_AP_JOIN_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub MacAddr: [u8; 6],
    pub uStatus: u32,
    pub bReAssocReq: super::super::Foundation::BOOLEAN,
    pub bReAssocResp: super::super::Foundation::BOOLEAN,
    pub uAssocReqOffset: u32,
    pub uAssocReqSize: u32,
    pub uAssocRespOffset: u32,
    pub uAssocRespSize: u32,
    pub uBeaconOffset: u32,
    pub uBeaconSize: u32,
    pub uIHVDataOffset: u32,
    pub uIHVDataSize: u32,
    pub AuthAlgo: DOT11_AUTH_ALGORITHM,
    pub UnicastCipher: DOT11_CIPHER_ALGORITHM,
    pub MulticastCipher: DOT11_CIPHER_ALGORITHM,
    pub uActivePhyListOffset: u32,
    pub uActivePhyListSize: u32,
    pub bFourAddressSupported: super::super::Foundation::BOOLEAN,
    pub bPortAuthorized: super::super::Foundation::BOOLEAN,
    pub ucActiveQoSProtocol: u8,
    pub DSInfo: DOT11_DS_INFO,
    pub uEncapTableOffset: u32,
    pub uEncapTableSize: u32,
    pub MulticastMgmtCipher: DOT11_CIPHER_ALGORITHM,
    pub uAssocComebackTime: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ASSOCIATION_COMPLETION_PARAMETERS")
            .field("Header", &self.Header)
            .field("MacAddr", &self.MacAddr)
            .field("uStatus", &self.uStatus)
            .field("bReAssocReq", &self.bReAssocReq)
            .field("bReAssocResp", &self.bReAssocResp)
            .field("uAssocReqOffset", &self.uAssocReqOffset)
            .field("uAssocReqSize", &self.uAssocReqSize)
            .field("uAssocRespOffset", &self.uAssocRespOffset)
            .field("uAssocRespSize", &self.uAssocRespSize)
            .field("uBeaconOffset", &self.uBeaconOffset)
            .field("uBeaconSize", &self.uBeaconSize)
            .field("uIHVDataOffset", &self.uIHVDataOffset)
            .field("uIHVDataSize", &self.uIHVDataSize)
            .field("AuthAlgo", &self.AuthAlgo)
            .field("UnicastCipher", &self.UnicastCipher)
            .field("MulticastCipher", &self.MulticastCipher)
            .field("uActivePhyListOffset", &self.uActivePhyListOffset)
            .field("uActivePhyListSize", &self.uActivePhyListSize)
            .field("bFourAddressSupported", &self.bFourAddressSupported)
            .field("bPortAuthorized", &self.bPortAuthorized)
            .field("ucActiveQoSProtocol", &self.ucActiveQoSProtocol)
            .field("DSInfo", &self.DSInfo)
            .field("uEncapTableOffset", &self.uEncapTableOffset)
            .field("uEncapTableSize", &self.uEncapTableSize)
            .field("MulticastMgmtCipher", &self.MulticastMgmtCipher)
            .field("uAssocComebackTime", &self.uAssocComebackTime)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.MacAddr == other.MacAddr
            && self.uStatus == other.uStatus
            && self.bReAssocReq == other.bReAssocReq
            && self.bReAssocResp == other.bReAssocResp
            && self.uAssocReqOffset == other.uAssocReqOffset
            && self.uAssocReqSize == other.uAssocReqSize
            && self.uAssocRespOffset == other.uAssocRespOffset
            && self.uAssocRespSize == other.uAssocRespSize
            && self.uBeaconOffset == other.uBeaconOffset
            && self.uBeaconSize == other.uBeaconSize
            && self.uIHVDataOffset == other.uIHVDataOffset
            && self.uIHVDataSize == other.uIHVDataSize
            && self.AuthAlgo == other.AuthAlgo
            && self.UnicastCipher == other.UnicastCipher
            && self.MulticastCipher == other.MulticastCipher
            && self.uActivePhyListOffset == other.uActivePhyListOffset
            && self.uActivePhyListSize == other.uActivePhyListSize
            && self.bFourAddressSupported == other.bFourAddressSupported
            && self.bPortAuthorized == other.bPortAuthorized
            && self.ucActiveQoSProtocol == other.ucActiveQoSProtocol
            && self.DSInfo == other.DSInfo
            && self.uEncapTableOffset == other.uEncapTableOffset
            && self.uEncapTableSize == other.uEncapTableSize
            && self.MulticastMgmtCipher == other.MulticastMgmtCipher
            && self.uAssocComebackTime == other.uAssocComebackTime
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_ASSOCIATION_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_ASSOCIATION_INFO_EX {
    pub PeerMacAddress: [u8; 6],
    pub BSSID: [u8; 6],
    pub usCapabilityInformation: u16,
    pub usListenInterval: u16,
    pub ucPeerSupportedRates: [u8; 255],
    pub usAssociationID: u16,
    pub dot11AssociationState: DOT11_ASSOCIATION_STATE,
    pub dot11PowerMode: DOT11_POWER_MODE,
    pub liAssociationUpTime: i64,
    pub ullNumOfTxPacketSuccesses: u64,
    pub ullNumOfTxPacketFailures: u64,
    pub ullNumOfRxPacketSuccesses: u64,
    pub ullNumOfRxPacketFailures: u64,
}
impl ::core::marker::Copy for DOT11_ASSOCIATION_INFO_EX {}
impl ::core::clone::Clone for DOT11_ASSOCIATION_INFO_EX {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_ASSOCIATION_INFO_EX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ASSOCIATION_INFO_EX")
            .field("PeerMacAddress", &self.PeerMacAddress)
            .field("BSSID", &self.BSSID)
            .field("usCapabilityInformation", &self.usCapabilityInformation)
            .field("usListenInterval", &self.usListenInterval)
            .field("ucPeerSupportedRates", &self.ucPeerSupportedRates)
            .field("usAssociationID", &self.usAssociationID)
            .field("dot11AssociationState", &self.dot11AssociationState)
            .field("dot11PowerMode", &self.dot11PowerMode)
            .field("liAssociationUpTime", &self.liAssociationUpTime)
            .field("ullNumOfTxPacketSuccesses", &self.ullNumOfTxPacketSuccesses)
            .field("ullNumOfTxPacketFailures", &self.ullNumOfTxPacketFailures)
            .field("ullNumOfRxPacketSuccesses", &self.ullNumOfRxPacketSuccesses)
            .field("ullNumOfRxPacketFailures", &self.ullNumOfRxPacketFailures)
            .finish()
    }
}
impl ::windows::core::TypeKind for DOT11_ASSOCIATION_INFO_EX {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_INFO_EX {
    fn eq(&self, other: &Self) -> bool {
        self.PeerMacAddress == other.PeerMacAddress && self.BSSID == other.BSSID && self.usCapabilityInformation == other.usCapabilityInformation && self.usListenInterval == other.usListenInterval && self.ucPeerSupportedRates == other.ucPeerSupportedRates && self.usAssociationID == other.usAssociationID && self.dot11AssociationState == other.dot11AssociationState && self.dot11PowerMode == other.dot11PowerMode && self.liAssociationUpTime == other.liAssociationUpTime && self.ullNumOfTxPacketSuccesses == other.ullNumOfTxPacketSuccesses && self.ullNumOfTxPacketFailures == other.ullNumOfTxPacketFailures && self.ullNumOfRxPacketSuccesses == other.ullNumOfRxPacketSuccesses && self.ullNumOfRxPacketFailures == other.ullNumOfRxPacketFailures
    }
}
impl ::core::cmp::Eq for DOT11_ASSOCIATION_INFO_EX {}
impl ::core::default::Default for DOT11_ASSOCIATION_INFO_EX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_ASSOCIATION_INFO_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11AssocInfo: [DOT11_ASSOCIATION_INFO_EX; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_ASSOCIATION_INFO_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_ASSOCIATION_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_ASSOCIATION_INFO_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ASSOCIATION_INFO_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11AssocInfo", &self.dot11AssocInfo).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_ASSOCIATION_INFO_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11AssocInfo == other.dot11AssocInfo
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ASSOCIATION_INFO_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ASSOCIATION_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_ASSOCIATION_PARAMS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub BSSID: [u8; 6],
    pub uAssocRequestIEsOffset: u32,
    pub uAssocRequestIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_ASSOCIATION_PARAMS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_ASSOCIATION_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_ASSOCIATION_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ASSOCIATION_PARAMS").field("Header", &self.Header).field("BSSID", &self.BSSID).field("uAssocRequestIEsOffset", &self.uAssocRequestIEsOffset).field("uAssocRequestIEsLength", &self.uAssocRequestIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_ASSOCIATION_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.BSSID == other.BSSID && self.uAssocRequestIEsOffset == other.uAssocRequestIEsOffset && self.uAssocRequestIEsLength == other.uAssocRequestIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ASSOCIATION_PARAMS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ASSOCIATION_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_ASSOCIATION_START_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub MacAddr: [u8; 6],
    pub SSID: DOT11_SSID,
    pub uIHVDataOffset: u32,
    pub uIHVDataSize: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_ASSOCIATION_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_ASSOCIATION_START_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_ASSOCIATION_START_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ASSOCIATION_START_PARAMETERS").field("Header", &self.Header).field("MacAddr", &self.MacAddr).field("SSID", &self.SSID).field("uIHVDataOffset", &self.uIHVDataOffset).field("uIHVDataSize", &self.uIHVDataSize).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_ASSOCIATION_START_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ASSOCIATION_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MacAddr == other.MacAddr && self.SSID == other.SSID && self.uIHVDataOffset == other.uIHVDataOffset && self.uIHVDataSize == other.uIHVDataSize
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ASSOCIATION_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ASSOCIATION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_AUTH_ALGORITHM_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub AlgorithmIds: [DOT11_AUTH_ALGORITHM; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_AUTH_ALGORITHM_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_AUTH_ALGORITHM_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_AUTH_ALGORITHM_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_AUTH_ALGORITHM_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("AlgorithmIds", &self.AlgorithmIds).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_AUTH_ALGORITHM_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_AUTH_ALGORITHM_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.AlgorithmIds == other.AlgorithmIds
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_AUTH_ALGORITHM_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_AUTH_ALGORITHM_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_AUTH_CIPHER_PAIR {
    pub AuthAlgoId: DOT11_AUTH_ALGORITHM,
    pub CipherAlgoId: DOT11_CIPHER_ALGORITHM,
}
impl ::core::marker::Copy for DOT11_AUTH_CIPHER_PAIR {}
impl ::core::clone::Clone for DOT11_AUTH_CIPHER_PAIR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_AUTH_CIPHER_PAIR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_AUTH_CIPHER_PAIR").field("AuthAlgoId", &self.AuthAlgoId).field("CipherAlgoId", &self.CipherAlgoId).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_AUTH_CIPHER_PAIR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_AUTH_CIPHER_PAIR {
    fn eq(&self, other: &Self) -> bool {
        self.AuthAlgoId == other.AuthAlgoId && self.CipherAlgoId == other.CipherAlgoId
    }
}
impl ::core::cmp::Eq for DOT11_AUTH_CIPHER_PAIR {}
impl ::core::default::Default for DOT11_AUTH_CIPHER_PAIR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_AUTH_CIPHER_PAIR_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub AuthCipherPairs: [DOT11_AUTH_CIPHER_PAIR; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_AUTH_CIPHER_PAIR_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_AUTH_CIPHER_PAIR_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_AUTH_CIPHER_PAIR_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_AUTH_CIPHER_PAIR_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("AuthCipherPairs", &self.AuthCipherPairs).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_AUTH_CIPHER_PAIR_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_AUTH_CIPHER_PAIR_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.AuthCipherPairs == other.AuthCipherPairs
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_AUTH_CIPHER_PAIR_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_AUTH_CIPHER_PAIR_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_AVAILABLE_CHANNEL_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub uChannelNumber: [u32; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_AVAILABLE_CHANNEL_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_AVAILABLE_CHANNEL_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_AVAILABLE_CHANNEL_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_AVAILABLE_CHANNEL_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("uChannelNumber", &self.uChannelNumber).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_AVAILABLE_CHANNEL_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_AVAILABLE_CHANNEL_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.uChannelNumber == other.uChannelNumber
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_AVAILABLE_CHANNEL_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_AVAILABLE_CHANNEL_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_AVAILABLE_FREQUENCY_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub uFrequencyValue: [u32; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_AVAILABLE_FREQUENCY_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_AVAILABLE_FREQUENCY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_AVAILABLE_FREQUENCY_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_AVAILABLE_FREQUENCY_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("uFrequencyValue", &self.uFrequencyValue).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_AVAILABLE_FREQUENCY_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_AVAILABLE_FREQUENCY_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.uFrequencyValue == other.uFrequencyValue
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_AVAILABLE_FREQUENCY_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_AVAILABLE_FREQUENCY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_BSSID_CANDIDATE {
    pub BSSID: [u8; 6],
    pub uFlags: u32,
}
impl ::core::marker::Copy for DOT11_BSSID_CANDIDATE {}
impl ::core::clone::Clone for DOT11_BSSID_CANDIDATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_BSSID_CANDIDATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_BSSID_CANDIDATE").field("BSSID", &self.BSSID).field("uFlags", &self.uFlags).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_BSSID_CANDIDATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_BSSID_CANDIDATE {
    fn eq(&self, other: &Self) -> bool {
        self.BSSID == other.BSSID && self.uFlags == other.uFlags
    }
}
impl ::core::cmp::Eq for DOT11_BSSID_CANDIDATE {}
impl ::core::default::Default for DOT11_BSSID_CANDIDATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_BSSID_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub BSSIDs: [u8; 6],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_BSSID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_BSSID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_BSSID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_BSSID_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("BSSIDs", &self.BSSIDs).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_BSSID_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_BSSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.BSSIDs == other.BSSIDs
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_BSSID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_BSSID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_BSS_DESCRIPTION {
    pub uReserved: u32,
    pub dot11BSSID: [u8; 6],
    pub dot11BSSType: DOT11_BSS_TYPE,
    pub usBeaconPeriod: u16,
    pub ullTimestamp: u64,
    pub usCapabilityInformation: u16,
    pub uBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
impl ::core::marker::Copy for DOT11_BSS_DESCRIPTION {}
impl ::core::clone::Clone for DOT11_BSS_DESCRIPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_BSS_DESCRIPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_BSS_DESCRIPTION").field("uReserved", &self.uReserved).field("dot11BSSID", &self.dot11BSSID).field("dot11BSSType", &self.dot11BSSType).field("usBeaconPeriod", &self.usBeaconPeriod).field("ullTimestamp", &self.ullTimestamp).field("usCapabilityInformation", &self.usCapabilityInformation).field("uBufferLength", &self.uBufferLength).field("ucBuffer", &self.ucBuffer).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_BSS_DESCRIPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_BSS_DESCRIPTION {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.dot11BSSID == other.dot11BSSID && self.dot11BSSType == other.dot11BSSType && self.usBeaconPeriod == other.usBeaconPeriod && self.ullTimestamp == other.ullTimestamp && self.usCapabilityInformation == other.usCapabilityInformation && self.uBufferLength == other.uBufferLength && self.ucBuffer == other.ucBuffer
    }
}
impl ::core::cmp::Eq for DOT11_BSS_DESCRIPTION {}
impl ::core::default::Default for DOT11_BSS_DESCRIPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_BSS_ENTRY {
    pub uPhyId: u32,
    pub PhySpecificInfo: DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO,
    pub dot11BSSID: [u8; 6],
    pub dot11BSSType: DOT11_BSS_TYPE,
    pub lRSSI: i32,
    pub uLinkQuality: u32,
    pub bInRegDomain: super::super::Foundation::BOOLEAN,
    pub usBeaconPeriod: u16,
    pub ullTimestamp: u64,
    pub ullHostTimestamp: u64,
    pub usCapabilityInformation: u16,
    pub uBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_BSS_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_BSS_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_BSS_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_BSS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub union DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    pub uChCenterFrequency: u32,
    pub FHSS: DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0,
}
impl ::core::marker::Copy for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {}
impl ::core::clone::Clone for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {
    pub uHopPattern: u32,
    pub uHopSet: u32,
    pub uDwellTime: u32,
}
impl ::core::marker::Copy for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {}
impl ::core::clone::Clone for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0").field("uHopPattern", &self.uHopPattern).field("uHopSet", &self.uHopSet).field("uDwellTime", &self.uDwellTime).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {
    fn eq(&self, other: &Self) -> bool {
        self.uHopPattern == other.uHopPattern && self.uHopSet == other.uHopSet && self.uDwellTime == other.uDwellTime
    }
}
impl ::core::cmp::Eq for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {}
impl ::core::default::Default for DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_BSS_LIST {
    pub uNumOfBytes: u32,
    pub pucBuffer: *mut u8,
}
impl ::core::marker::Copy for DOT11_BSS_LIST {}
impl ::core::clone::Clone for DOT11_BSS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_BSS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_BSS_LIST").field("uNumOfBytes", &self.uNumOfBytes).field("pucBuffer", &self.pucBuffer).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_BSS_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_BSS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfBytes == other.uNumOfBytes && self.pucBuffer == other.pucBuffer
    }
}
impl ::core::cmp::Eq for DOT11_BSS_LIST {}
impl ::core::default::Default for DOT11_BSS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_BYTE_ARRAY {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfBytes: u32,
    pub uTotalNumOfBytes: u32,
    pub ucBuffer: [u8; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_BYTE_ARRAY {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_BYTE_ARRAY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_BYTE_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_BYTE_ARRAY").field("Header", &self.Header).field("uNumOfBytes", &self.uNumOfBytes).field("uTotalNumOfBytes", &self.uTotalNumOfBytes).field("ucBuffer", &self.ucBuffer).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_BYTE_ARRAY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_BYTE_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfBytes == other.uNumOfBytes && self.uTotalNumOfBytes == other.uTotalNumOfBytes && self.ucBuffer == other.ucBuffer
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_BYTE_ARRAY {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_BYTE_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ulReason: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_CAN_SUSTAIN_AP_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CAN_SUSTAIN_AP_PARAMETERS").field("Header", &self.Header).field("ulReason", &self.ulReason).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ulReason == other.ulReason
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_CAN_SUSTAIN_AP_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_CAN_SUSTAIN_AP_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_CHANNEL_HINT {
    pub Dot11PhyType: DOT11_PHY_TYPE,
    pub uChannelNumber: u32,
}
impl ::core::marker::Copy for DOT11_CHANNEL_HINT {}
impl ::core::clone::Clone for DOT11_CHANNEL_HINT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_CHANNEL_HINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CHANNEL_HINT").field("Dot11PhyType", &self.Dot11PhyType).field("uChannelNumber", &self.uChannelNumber).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_CHANNEL_HINT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_CHANNEL_HINT {
    fn eq(&self, other: &Self) -> bool {
        self.Dot11PhyType == other.Dot11PhyType && self.uChannelNumber == other.uChannelNumber
    }
}
impl ::core::cmp::Eq for DOT11_CHANNEL_HINT {}
impl ::core::default::Default for DOT11_CHANNEL_HINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_CIPHER_ALGORITHM_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub AlgorithmIds: [DOT11_CIPHER_ALGORITHM; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_CIPHER_ALGORITHM_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_CIPHER_ALGORITHM_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_CIPHER_ALGORITHM_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CIPHER_ALGORITHM_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("AlgorithmIds", &self.AlgorithmIds).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_CIPHER_ALGORITHM_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_CIPHER_ALGORITHM_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.AlgorithmIds == other.AlgorithmIds
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_CIPHER_ALGORITHM_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_CIPHER_ALGORITHM_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_CIPHER_DEFAULT_KEY_VALUE {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uKeyIndex: u32,
    pub AlgorithmId: DOT11_CIPHER_ALGORITHM,
    pub MacAddr: [u8; 6],
    pub bDelete: super::super::Foundation::BOOLEAN,
    pub bStatic: super::super::Foundation::BOOLEAN,
    pub usKeyLength: u16,
    pub ucKey: [u8; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_CIPHER_DEFAULT_KEY_VALUE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_CIPHER_DEFAULT_KEY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_CIPHER_DEFAULT_KEY_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CIPHER_DEFAULT_KEY_VALUE").field("Header", &self.Header).field("uKeyIndex", &self.uKeyIndex).field("AlgorithmId", &self.AlgorithmId).field("MacAddr", &self.MacAddr).field("bDelete", &self.bDelete).field("bStatic", &self.bStatic).field("usKeyLength", &self.usKeyLength).field("ucKey", &self.ucKey).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_CIPHER_DEFAULT_KEY_VALUE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_CIPHER_DEFAULT_KEY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uKeyIndex == other.uKeyIndex && self.AlgorithmId == other.AlgorithmId && self.MacAddr == other.MacAddr && self.bDelete == other.bDelete && self.bStatic == other.bStatic && self.usKeyLength == other.usKeyLength && self.ucKey == other.ucKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_CIPHER_DEFAULT_KEY_VALUE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_CIPHER_DEFAULT_KEY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    pub PeerMacAddr: [u8; 6],
    pub AlgorithmId: DOT11_CIPHER_ALGORITHM,
    pub Direction: DOT11_DIRECTION,
    pub bDelete: super::super::Foundation::BOOLEAN,
    pub bStatic: super::super::Foundation::BOOLEAN,
    pub usKeyLength: u16,
    pub ucKey: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CIPHER_KEY_MAPPING_KEY_VALUE").field("PeerMacAddr", &self.PeerMacAddr).field("AlgorithmId", &self.AlgorithmId).field("Direction", &self.Direction).field("bDelete", &self.bDelete).field("bStatic", &self.bStatic).field("usKeyLength", &self.usKeyLength).field("ucKey", &self.ucKey).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.PeerMacAddr == other.PeerMacAddr && self.AlgorithmId == other.AlgorithmId && self.Direction == other.Direction && self.bDelete == other.bDelete && self.bStatic == other.bStatic && self.usKeyLength == other.usKeyLength && self.ucKey == other.ucKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_CIPHER_KEY_MAPPING_KEY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_CONNECTION_COMPLETION_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uStatus: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_CONNECTION_COMPLETION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_CONNECTION_COMPLETION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_CONNECTION_COMPLETION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CONNECTION_COMPLETION_PARAMETERS").field("Header", &self.Header).field("uStatus", &self.uStatus).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_CONNECTION_COMPLETION_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_CONNECTION_COMPLETION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uStatus == other.uStatus
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_CONNECTION_COMPLETION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_CONNECTION_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_CONNECTION_START_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub BSSType: DOT11_BSS_TYPE,
    pub AdhocBSSID: [u8; 6],
    pub AdhocSSID: DOT11_SSID,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_CONNECTION_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_CONNECTION_START_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_CONNECTION_START_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CONNECTION_START_PARAMETERS").field("Header", &self.Header).field("BSSType", &self.BSSType).field("AdhocBSSID", &self.AdhocBSSID).field("AdhocSSID", &self.AdhocSSID).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_CONNECTION_START_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_CONNECTION_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.BSSType == other.BSSType && self.AdhocBSSID == other.AdhocBSSID && self.AdhocSSID == other.AdhocSSID
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_CONNECTION_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_CONNECTION_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_COUNTERS_ENTRY {
    pub uTransmittedFragmentCount: u32,
    pub uMulticastTransmittedFrameCount: u32,
    pub uFailedCount: u32,
    pub uRetryCount: u32,
    pub uMultipleRetryCount: u32,
    pub uFrameDuplicateCount: u32,
    pub uRTSSuccessCount: u32,
    pub uRTSFailureCount: u32,
    pub uACKFailureCount: u32,
    pub uReceivedFragmentCount: u32,
    pub uMulticastReceivedFrameCount: u32,
    pub uFCSErrorCount: u32,
    pub uTransmittedFrameCount: u32,
}
impl ::core::marker::Copy for DOT11_COUNTERS_ENTRY {}
impl ::core::clone::Clone for DOT11_COUNTERS_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_COUNTERS_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_COUNTERS_ENTRY")
            .field("uTransmittedFragmentCount", &self.uTransmittedFragmentCount)
            .field("uMulticastTransmittedFrameCount", &self.uMulticastTransmittedFrameCount)
            .field("uFailedCount", &self.uFailedCount)
            .field("uRetryCount", &self.uRetryCount)
            .field("uMultipleRetryCount", &self.uMultipleRetryCount)
            .field("uFrameDuplicateCount", &self.uFrameDuplicateCount)
            .field("uRTSSuccessCount", &self.uRTSSuccessCount)
            .field("uRTSFailureCount", &self.uRTSFailureCount)
            .field("uACKFailureCount", &self.uACKFailureCount)
            .field("uReceivedFragmentCount", &self.uReceivedFragmentCount)
            .field("uMulticastReceivedFrameCount", &self.uMulticastReceivedFrameCount)
            .field("uFCSErrorCount", &self.uFCSErrorCount)
            .field("uTransmittedFrameCount", &self.uTransmittedFrameCount)
            .finish()
    }
}
impl ::windows::core::TypeKind for DOT11_COUNTERS_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_COUNTERS_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.uTransmittedFragmentCount == other.uTransmittedFragmentCount && self.uMulticastTransmittedFrameCount == other.uMulticastTransmittedFrameCount && self.uFailedCount == other.uFailedCount && self.uRetryCount == other.uRetryCount && self.uMultipleRetryCount == other.uMultipleRetryCount && self.uFrameDuplicateCount == other.uFrameDuplicateCount && self.uRTSSuccessCount == other.uRTSSuccessCount && self.uRTSFailureCount == other.uRTSFailureCount && self.uACKFailureCount == other.uACKFailureCount && self.uReceivedFragmentCount == other.uReceivedFragmentCount && self.uMulticastReceivedFrameCount == other.uMulticastReceivedFrameCount && self.uFCSErrorCount == other.uFCSErrorCount && self.uTransmittedFrameCount == other.uTransmittedFrameCount
    }
}
impl ::core::cmp::Eq for DOT11_COUNTERS_ENTRY {}
impl ::core::default::Default for DOT11_COUNTERS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_COUNTRY_OR_REGION_STRING_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub CountryOrRegionStrings: [u8; 3],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_COUNTRY_OR_REGION_STRING_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_COUNTRY_OR_REGION_STRING_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_COUNTRY_OR_REGION_STRING_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_COUNTRY_OR_REGION_STRING_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("CountryOrRegionStrings", &self.CountryOrRegionStrings).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_COUNTRY_OR_REGION_STRING_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_COUNTRY_OR_REGION_STRING_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.CountryOrRegionStrings == other.CountryOrRegionStrings
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_COUNTRY_OR_REGION_STRING_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_COUNTRY_OR_REGION_STRING_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_CURRENT_OFFLOAD_CAPABILITY {
    pub uReserved: u32,
    pub uFlags: u32,
}
impl ::core::marker::Copy for DOT11_CURRENT_OFFLOAD_CAPABILITY {}
impl ::core::clone::Clone for DOT11_CURRENT_OFFLOAD_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_CURRENT_OFFLOAD_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CURRENT_OFFLOAD_CAPABILITY").field("uReserved", &self.uReserved).field("uFlags", &self.uFlags).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_CURRENT_OFFLOAD_CAPABILITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_CURRENT_OFFLOAD_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.uFlags == other.uFlags
    }
}
impl ::core::cmp::Eq for DOT11_CURRENT_OFFLOAD_CAPABILITY {}
impl ::core::default::Default for DOT11_CURRENT_OFFLOAD_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_CURRENT_OPERATION_MODE {
    pub uReserved: u32,
    pub uCurrentOpMode: u32,
}
impl ::core::marker::Copy for DOT11_CURRENT_OPERATION_MODE {}
impl ::core::clone::Clone for DOT11_CURRENT_OPERATION_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_CURRENT_OPERATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CURRENT_OPERATION_MODE").field("uReserved", &self.uReserved).field("uCurrentOpMode", &self.uCurrentOpMode).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_CURRENT_OPERATION_MODE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_CURRENT_OPERATION_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.uCurrentOpMode == other.uCurrentOpMode
    }
}
impl ::core::cmp::Eq for DOT11_CURRENT_OPERATION_MODE {}
impl ::core::default::Default for DOT11_CURRENT_OPERATION_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_CURRENT_OPTIONAL_CAPABILITY {
    pub uReserved: u32,
    pub bDot11CFPollable: super::super::Foundation::BOOLEAN,
    pub bDot11PCF: super::super::Foundation::BOOLEAN,
    pub bDot11PCFMPDUTransferToPC: super::super::Foundation::BOOLEAN,
    pub bStrictlyOrderedServiceClass: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_CURRENT_OPTIONAL_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_CURRENT_OPTIONAL_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_CURRENT_OPTIONAL_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_CURRENT_OPTIONAL_CAPABILITY").field("uReserved", &self.uReserved).field("bDot11CFPollable", &self.bDot11CFPollable).field("bDot11PCF", &self.bDot11PCF).field("bDot11PCFMPDUTransferToPC", &self.bDot11PCFMPDUTransferToPC).field("bStrictlyOrderedServiceClass", &self.bStrictlyOrderedServiceClass).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_CURRENT_OPTIONAL_CAPABILITY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_CURRENT_OPTIONAL_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.bDot11CFPollable == other.bDot11CFPollable && self.bDot11PCF == other.bDot11PCF && self.bDot11PCFMPDUTransferToPC == other.bDot11PCFMPDUTransferToPC && self.bStrictlyOrderedServiceClass == other.bStrictlyOrderedServiceClass
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_CURRENT_OPTIONAL_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_CURRENT_OPTIONAL_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_DATA_RATE_MAPPING_ENTRY {
    pub ucDataRateIndex: u8,
    pub ucDataRateFlag: u8,
    pub usDataRateValue: u16,
}
impl ::core::marker::Copy for DOT11_DATA_RATE_MAPPING_ENTRY {}
impl ::core::clone::Clone for DOT11_DATA_RATE_MAPPING_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_DATA_RATE_MAPPING_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DATA_RATE_MAPPING_ENTRY").field("ucDataRateIndex", &self.ucDataRateIndex).field("ucDataRateFlag", &self.ucDataRateFlag).field("usDataRateValue", &self.usDataRateValue).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_DATA_RATE_MAPPING_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_DATA_RATE_MAPPING_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.ucDataRateIndex == other.ucDataRateIndex && self.ucDataRateFlag == other.ucDataRateFlag && self.usDataRateValue == other.usDataRateValue
    }
}
impl ::core::cmp::Eq for DOT11_DATA_RATE_MAPPING_ENTRY {}
impl ::core::default::Default for DOT11_DATA_RATE_MAPPING_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_DATA_RATE_MAPPING_TABLE {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uDataRateMappingLength: u32,
    pub DataRateMappingEntries: [DOT11_DATA_RATE_MAPPING_ENTRY; 126],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_DATA_RATE_MAPPING_TABLE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_DATA_RATE_MAPPING_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_DATA_RATE_MAPPING_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DATA_RATE_MAPPING_TABLE").field("Header", &self.Header).field("uDataRateMappingLength", &self.uDataRateMappingLength).field("DataRateMappingEntries", &self.DataRateMappingEntries).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_DATA_RATE_MAPPING_TABLE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_DATA_RATE_MAPPING_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uDataRateMappingLength == other.uDataRateMappingLength && self.DataRateMappingEntries == other.DataRateMappingEntries
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_DATA_RATE_MAPPING_TABLE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_DATA_RATE_MAPPING_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_DEFAULT_WEP_OFFLOAD {
    pub uReserved: u32,
    pub hOffloadContext: super::super::Foundation::HANDLE,
    pub hOffload: super::super::Foundation::HANDLE,
    pub dwIndex: u32,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub dwAlgorithm: u32,
    pub uFlags: u32,
    pub dot11KeyDirection: DOT11_KEY_DIRECTION,
    pub ucMacAddress: [u8; 6],
    pub uNumOfRWsOnMe: u32,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [u16; 16],
    pub usKeyLength: u16,
    pub ucKey: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_DEFAULT_WEP_OFFLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_DEFAULT_WEP_OFFLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_DEFAULT_WEP_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DEFAULT_WEP_OFFLOAD")
            .field("uReserved", &self.uReserved)
            .field("hOffloadContext", &self.hOffloadContext)
            .field("hOffload", &self.hOffload)
            .field("dwIndex", &self.dwIndex)
            .field("dot11OffloadType", &self.dot11OffloadType)
            .field("dwAlgorithm", &self.dwAlgorithm)
            .field("uFlags", &self.uFlags)
            .field("dot11KeyDirection", &self.dot11KeyDirection)
            .field("ucMacAddress", &self.ucMacAddress)
            .field("uNumOfRWsOnMe", &self.uNumOfRWsOnMe)
            .field("dot11IV48Counters", &self.dot11IV48Counters)
            .field("usDot11RWBitMaps", &self.usDot11RWBitMaps)
            .field("usKeyLength", &self.usKeyLength)
            .field("ucKey", &self.ucKey)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_DEFAULT_WEP_OFFLOAD {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_DEFAULT_WEP_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.hOffloadContext == other.hOffloadContext && self.hOffload == other.hOffload && self.dwIndex == other.dwIndex && self.dot11OffloadType == other.dot11OffloadType && self.dwAlgorithm == other.dwAlgorithm && self.uFlags == other.uFlags && self.dot11KeyDirection == other.dot11KeyDirection && self.ucMacAddress == other.ucMacAddress && self.uNumOfRWsOnMe == other.uNumOfRWsOnMe && self.dot11IV48Counters == other.dot11IV48Counters && self.usDot11RWBitMaps == other.usDot11RWBitMaps && self.usKeyLength == other.usKeyLength && self.ucKey == other.ucKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_DEFAULT_WEP_OFFLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_DEFAULT_WEP_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_DEFAULT_WEP_UPLOAD {
    pub uReserved: u32,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub hOffload: super::super::Foundation::HANDLE,
    pub uNumOfRWsUsed: u32,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [u16; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_DEFAULT_WEP_UPLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_DEFAULT_WEP_UPLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_DEFAULT_WEP_UPLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DEFAULT_WEP_UPLOAD").field("uReserved", &self.uReserved).field("dot11OffloadType", &self.dot11OffloadType).field("hOffload", &self.hOffload).field("uNumOfRWsUsed", &self.uNumOfRWsUsed).field("dot11IV48Counters", &self.dot11IV48Counters).field("usDot11RWBitMaps", &self.usDot11RWBitMaps).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_DEFAULT_WEP_UPLOAD {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_DEFAULT_WEP_UPLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.dot11OffloadType == other.dot11OffloadType && self.hOffload == other.hOffload && self.uNumOfRWsUsed == other.uNumOfRWsUsed && self.dot11IV48Counters == other.dot11IV48Counters && self.usDot11RWBitMaps == other.usDot11RWBitMaps
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_DEFAULT_WEP_UPLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_DEFAULT_WEP_UPLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_DISASSOCIATE_PEER_REQUEST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: [u8; 6],
    pub usReason: u16,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_DISASSOCIATE_PEER_REQUEST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_DISASSOCIATE_PEER_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_DISASSOCIATE_PEER_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DISASSOCIATE_PEER_REQUEST").field("Header", &self.Header).field("PeerMacAddr", &self.PeerMacAddr).field("usReason", &self.usReason).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_DISASSOCIATE_PEER_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_DISASSOCIATE_PEER_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerMacAddr == other.PeerMacAddr && self.usReason == other.usReason
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_DISASSOCIATE_PEER_REQUEST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_DISASSOCIATE_PEER_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_DISASSOCIATION_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub MacAddr: [u8; 6],
    pub uReason: u32,
    pub uIHVDataOffset: u32,
    pub uIHVDataSize: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_DISASSOCIATION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_DISASSOCIATION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_DISASSOCIATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DISASSOCIATION_PARAMETERS").field("Header", &self.Header).field("MacAddr", &self.MacAddr).field("uReason", &self.uReason).field("uIHVDataOffset", &self.uIHVDataOffset).field("uIHVDataSize", &self.uIHVDataSize).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_DISASSOCIATION_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_DISASSOCIATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.MacAddr == other.MacAddr && self.uReason == other.uReason && self.uIHVDataOffset == other.uIHVDataOffset && self.uIHVDataSize == other.uIHVDataSize
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_DISASSOCIATION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_DISASSOCIATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_DIVERSITY_SELECTION_RX {
    pub uAntennaListIndex: u32,
    pub bDiversitySelectionRX: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_DIVERSITY_SELECTION_RX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_DIVERSITY_SELECTION_RX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_DIVERSITY_SELECTION_RX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DIVERSITY_SELECTION_RX").field("uAntennaListIndex", &self.uAntennaListIndex).field("bDiversitySelectionRX", &self.bDiversitySelectionRX).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_DIVERSITY_SELECTION_RX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_DIVERSITY_SELECTION_RX {
    fn eq(&self, other: &Self) -> bool {
        self.uAntennaListIndex == other.uAntennaListIndex && self.bDiversitySelectionRX == other.bDiversitySelectionRX
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_DIVERSITY_SELECTION_RX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_DIVERSITY_SELECTION_RX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_DIVERSITY_SELECTION_RX_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11DiversitySelectionRx: [DOT11_DIVERSITY_SELECTION_RX; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_DIVERSITY_SELECTION_RX_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_DIVERSITY_SELECTION_RX_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_DIVERSITY_SELECTION_RX_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_DIVERSITY_SELECTION_RX_LIST").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11DiversitySelectionRx", &self.dot11DiversitySelectionRx).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_DIVERSITY_SELECTION_RX_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_DIVERSITY_SELECTION_RX_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11DiversitySelectionRx == other.dot11DiversitySelectionRx
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_DIVERSITY_SELECTION_RX_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_DIVERSITY_SELECTION_RX_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
pub struct DOT11_EAP_RESULT {
    pub dwFailureReasonCode: u32,
    pub pAttribArray: *mut super::super::Security::ExtensibleAuthenticationProtocol::EAP_ATTRIBUTES,
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::marker::Copy for DOT11_EAP_RESULT {}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::clone::Clone for DOT11_EAP_RESULT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::fmt::Debug for DOT11_EAP_RESULT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_EAP_RESULT").field("dwFailureReasonCode", &self.dwFailureReasonCode).field("pAttribArray", &self.pAttribArray).finish()
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::windows::core::TypeKind for DOT11_EAP_RESULT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::cmp::PartialEq for DOT11_EAP_RESULT {
    fn eq(&self, other: &Self) -> bool {
        self.dwFailureReasonCode == other.dwFailureReasonCode && self.pAttribArray == other.pAttribArray
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::cmp::Eq for DOT11_EAP_RESULT {}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::default::Default for DOT11_EAP_RESULT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_ENCAP_ENTRY {
    pub usEtherType: u16,
    pub usEncapType: u16,
}
impl ::core::marker::Copy for DOT11_ENCAP_ENTRY {}
impl ::core::clone::Clone for DOT11_ENCAP_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_ENCAP_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ENCAP_ENTRY").field("usEtherType", &self.usEtherType).field("usEncapType", &self.usEncapType).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_ENCAP_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_ENCAP_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.usEtherType == other.usEtherType && self.usEncapType == other.usEncapType
    }
}
impl ::core::cmp::Eq for DOT11_ENCAP_ENTRY {}
impl ::core::default::Default for DOT11_ENCAP_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_ERP_PHY_ATTRIBUTES {
    pub HRDSSSAttributes: DOT11_HRDSSS_PHY_ATTRIBUTES,
    pub bERPPBCCOptionImplemented: super::super::Foundation::BOOLEAN,
    pub bDSSSOFDMOptionImplemented: super::super::Foundation::BOOLEAN,
    pub bShortSlotTimeOptionImplemented: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_ERP_PHY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_ERP_PHY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_ERP_PHY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ERP_PHY_ATTRIBUTES").field("HRDSSSAttributes", &self.HRDSSSAttributes).field("bERPPBCCOptionImplemented", &self.bERPPBCCOptionImplemented).field("bDSSSOFDMOptionImplemented", &self.bDSSSOFDMOptionImplemented).field("bShortSlotTimeOptionImplemented", &self.bShortSlotTimeOptionImplemented).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_ERP_PHY_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_ERP_PHY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.HRDSSSAttributes == other.HRDSSSAttributes && self.bERPPBCCOptionImplemented == other.bERPPBCCOptionImplemented && self.bDSSSOFDMOptionImplemented == other.bDSSSOFDMOptionImplemented && self.bShortSlotTimeOptionImplemented == other.bShortSlotTimeOptionImplemented
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_ERP_PHY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_ERP_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_EXTAP_ATTRIBUTES {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uScanSSIDListSize: u32,
    pub uDesiredSSIDListSize: u32,
    pub uPrivacyExemptionListSize: u32,
    pub uAssociationTableSize: u32,
    pub uDefaultKeyTableSize: u32,
    pub uWEPKeyValueMaxLength: u32,
    pub bStrictlyOrderedServiceClassImplemented: super::super::Foundation::BOOLEAN,
    pub uNumSupportedCountryOrRegionStrings: u32,
    pub pSupportedCountryOrRegionStrings: *mut u8,
    pub uInfraNumSupportedUcastAlgoPairs: u32,
    pub pInfraSupportedUcastAlgoPairs: *mut DOT11_AUTH_CIPHER_PAIR,
    pub uInfraNumSupportedMcastAlgoPairs: u32,
    pub pInfraSupportedMcastAlgoPairs: *mut DOT11_AUTH_CIPHER_PAIR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_EXTAP_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_EXTAP_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_EXTAP_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_EXTAP_ATTRIBUTES")
            .field("Header", &self.Header)
            .field("uScanSSIDListSize", &self.uScanSSIDListSize)
            .field("uDesiredSSIDListSize", &self.uDesiredSSIDListSize)
            .field("uPrivacyExemptionListSize", &self.uPrivacyExemptionListSize)
            .field("uAssociationTableSize", &self.uAssociationTableSize)
            .field("uDefaultKeyTableSize", &self.uDefaultKeyTableSize)
            .field("uWEPKeyValueMaxLength", &self.uWEPKeyValueMaxLength)
            .field("bStrictlyOrderedServiceClassImplemented", &self.bStrictlyOrderedServiceClassImplemented)
            .field("uNumSupportedCountryOrRegionStrings", &self.uNumSupportedCountryOrRegionStrings)
            .field("pSupportedCountryOrRegionStrings", &self.pSupportedCountryOrRegionStrings)
            .field("uInfraNumSupportedUcastAlgoPairs", &self.uInfraNumSupportedUcastAlgoPairs)
            .field("pInfraSupportedUcastAlgoPairs", &self.pInfraSupportedUcastAlgoPairs)
            .field("uInfraNumSupportedMcastAlgoPairs", &self.uInfraNumSupportedMcastAlgoPairs)
            .field("pInfraSupportedMcastAlgoPairs", &self.pInfraSupportedMcastAlgoPairs)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_EXTAP_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_EXTAP_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.uScanSSIDListSize == other.uScanSSIDListSize
            && self.uDesiredSSIDListSize == other.uDesiredSSIDListSize
            && self.uPrivacyExemptionListSize == other.uPrivacyExemptionListSize
            && self.uAssociationTableSize == other.uAssociationTableSize
            && self.uDefaultKeyTableSize == other.uDefaultKeyTableSize
            && self.uWEPKeyValueMaxLength == other.uWEPKeyValueMaxLength
            && self.bStrictlyOrderedServiceClassImplemented == other.bStrictlyOrderedServiceClassImplemented
            && self.uNumSupportedCountryOrRegionStrings == other.uNumSupportedCountryOrRegionStrings
            && self.pSupportedCountryOrRegionStrings == other.pSupportedCountryOrRegionStrings
            && self.uInfraNumSupportedUcastAlgoPairs == other.uInfraNumSupportedUcastAlgoPairs
            && self.pInfraSupportedUcastAlgoPairs == other.pInfraSupportedUcastAlgoPairs
            && self.uInfraNumSupportedMcastAlgoPairs == other.uInfraNumSupportedMcastAlgoPairs
            && self.pInfraSupportedMcastAlgoPairs == other.pInfraSupportedMcastAlgoPairs
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_EXTAP_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_EXTAP_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_EXTSTA_ATTRIBUTES {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uScanSSIDListSize: u32,
    pub uDesiredBSSIDListSize: u32,
    pub uDesiredSSIDListSize: u32,
    pub uExcludedMacAddressListSize: u32,
    pub uPrivacyExemptionListSize: u32,
    pub uKeyMappingTableSize: u32,
    pub uDefaultKeyTableSize: u32,
    pub uWEPKeyValueMaxLength: u32,
    pub uPMKIDCacheSize: u32,
    pub uMaxNumPerSTADefaultKeyTables: u32,
    pub bStrictlyOrderedServiceClassImplemented: super::super::Foundation::BOOLEAN,
    pub ucSupportedQoSProtocolFlags: u8,
    pub bSafeModeImplemented: super::super::Foundation::BOOLEAN,
    pub uNumSupportedCountryOrRegionStrings: u32,
    pub pSupportedCountryOrRegionStrings: *mut u8,
    pub uInfraNumSupportedUcastAlgoPairs: u32,
    pub pInfraSupportedUcastAlgoPairs: *mut DOT11_AUTH_CIPHER_PAIR,
    pub uInfraNumSupportedMcastAlgoPairs: u32,
    pub pInfraSupportedMcastAlgoPairs: *mut DOT11_AUTH_CIPHER_PAIR,
    pub uAdhocNumSupportedUcastAlgoPairs: u32,
    pub pAdhocSupportedUcastAlgoPairs: *mut DOT11_AUTH_CIPHER_PAIR,
    pub uAdhocNumSupportedMcastAlgoPairs: u32,
    pub pAdhocSupportedMcastAlgoPairs: *mut DOT11_AUTH_CIPHER_PAIR,
    pub bAutoPowerSaveMode: super::super::Foundation::BOOLEAN,
    pub uMaxNetworkOffloadListSize: u32,
    pub bMFPCapable: super::super::Foundation::BOOLEAN,
    pub uInfraNumSupportedMcastMgmtAlgoPairs: u32,
    pub pInfraSupportedMcastMgmtAlgoPairs: *mut DOT11_AUTH_CIPHER_PAIR,
    pub bNeighborReportSupported: super::super::Foundation::BOOLEAN,
    pub bAPChannelReportSupported: super::super::Foundation::BOOLEAN,
    pub bActionFramesSupported: super::super::Foundation::BOOLEAN,
    pub bANQPQueryOffloadSupported: super::super::Foundation::BOOLEAN,
    pub bHESSIDConnectionSupported: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_EXTSTA_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_EXTSTA_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_EXTSTA_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_EXTSTA_ATTRIBUTES")
            .field("Header", &self.Header)
            .field("uScanSSIDListSize", &self.uScanSSIDListSize)
            .field("uDesiredBSSIDListSize", &self.uDesiredBSSIDListSize)
            .field("uDesiredSSIDListSize", &self.uDesiredSSIDListSize)
            .field("uExcludedMacAddressListSize", &self.uExcludedMacAddressListSize)
            .field("uPrivacyExemptionListSize", &self.uPrivacyExemptionListSize)
            .field("uKeyMappingTableSize", &self.uKeyMappingTableSize)
            .field("uDefaultKeyTableSize", &self.uDefaultKeyTableSize)
            .field("uWEPKeyValueMaxLength", &self.uWEPKeyValueMaxLength)
            .field("uPMKIDCacheSize", &self.uPMKIDCacheSize)
            .field("uMaxNumPerSTADefaultKeyTables", &self.uMaxNumPerSTADefaultKeyTables)
            .field("bStrictlyOrderedServiceClassImplemented", &self.bStrictlyOrderedServiceClassImplemented)
            .field("ucSupportedQoSProtocolFlags", &self.ucSupportedQoSProtocolFlags)
            .field("bSafeModeImplemented", &self.bSafeModeImplemented)
            .field("uNumSupportedCountryOrRegionStrings", &self.uNumSupportedCountryOrRegionStrings)
            .field("pSupportedCountryOrRegionStrings", &self.pSupportedCountryOrRegionStrings)
            .field("uInfraNumSupportedUcastAlgoPairs", &self.uInfraNumSupportedUcastAlgoPairs)
            .field("pInfraSupportedUcastAlgoPairs", &self.pInfraSupportedUcastAlgoPairs)
            .field("uInfraNumSupportedMcastAlgoPairs", &self.uInfraNumSupportedMcastAlgoPairs)
            .field("pInfraSupportedMcastAlgoPairs", &self.pInfraSupportedMcastAlgoPairs)
            .field("uAdhocNumSupportedUcastAlgoPairs", &self.uAdhocNumSupportedUcastAlgoPairs)
            .field("pAdhocSupportedUcastAlgoPairs", &self.pAdhocSupportedUcastAlgoPairs)
            .field("uAdhocNumSupportedMcastAlgoPairs", &self.uAdhocNumSupportedMcastAlgoPairs)
            .field("pAdhocSupportedMcastAlgoPairs", &self.pAdhocSupportedMcastAlgoPairs)
            .field("bAutoPowerSaveMode", &self.bAutoPowerSaveMode)
            .field("uMaxNetworkOffloadListSize", &self.uMaxNetworkOffloadListSize)
            .field("bMFPCapable", &self.bMFPCapable)
            .field("uInfraNumSupportedMcastMgmtAlgoPairs", &self.uInfraNumSupportedMcastMgmtAlgoPairs)
            .field("pInfraSupportedMcastMgmtAlgoPairs", &self.pInfraSupportedMcastMgmtAlgoPairs)
            .field("bNeighborReportSupported", &self.bNeighborReportSupported)
            .field("bAPChannelReportSupported", &self.bAPChannelReportSupported)
            .field("bActionFramesSupported", &self.bActionFramesSupported)
            .field("bANQPQueryOffloadSupported", &self.bANQPQueryOffloadSupported)
            .field("bHESSIDConnectionSupported", &self.bHESSIDConnectionSupported)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_EXTSTA_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_EXTSTA_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.uScanSSIDListSize == other.uScanSSIDListSize
            && self.uDesiredBSSIDListSize == other.uDesiredBSSIDListSize
            && self.uDesiredSSIDListSize == other.uDesiredSSIDListSize
            && self.uExcludedMacAddressListSize == other.uExcludedMacAddressListSize
            && self.uPrivacyExemptionListSize == other.uPrivacyExemptionListSize
            && self.uKeyMappingTableSize == other.uKeyMappingTableSize
            && self.uDefaultKeyTableSize == other.uDefaultKeyTableSize
            && self.uWEPKeyValueMaxLength == other.uWEPKeyValueMaxLength
            && self.uPMKIDCacheSize == other.uPMKIDCacheSize
            && self.uMaxNumPerSTADefaultKeyTables == other.uMaxNumPerSTADefaultKeyTables
            && self.bStrictlyOrderedServiceClassImplemented == other.bStrictlyOrderedServiceClassImplemented
            && self.ucSupportedQoSProtocolFlags == other.ucSupportedQoSProtocolFlags
            && self.bSafeModeImplemented == other.bSafeModeImplemented
            && self.uNumSupportedCountryOrRegionStrings == other.uNumSupportedCountryOrRegionStrings
            && self.pSupportedCountryOrRegionStrings == other.pSupportedCountryOrRegionStrings
            && self.uInfraNumSupportedUcastAlgoPairs == other.uInfraNumSupportedUcastAlgoPairs
            && self.pInfraSupportedUcastAlgoPairs == other.pInfraSupportedUcastAlgoPairs
            && self.uInfraNumSupportedMcastAlgoPairs == other.uInfraNumSupportedMcastAlgoPairs
            && self.pInfraSupportedMcastAlgoPairs == other.pInfraSupportedMcastAlgoPairs
            && self.uAdhocNumSupportedUcastAlgoPairs == other.uAdhocNumSupportedUcastAlgoPairs
            && self.pAdhocSupportedUcastAlgoPairs == other.pAdhocSupportedUcastAlgoPairs
            && self.uAdhocNumSupportedMcastAlgoPairs == other.uAdhocNumSupportedMcastAlgoPairs
            && self.pAdhocSupportedMcastAlgoPairs == other.pAdhocSupportedMcastAlgoPairs
            && self.bAutoPowerSaveMode == other.bAutoPowerSaveMode
            && self.uMaxNetworkOffloadListSize == other.uMaxNetworkOffloadListSize
            && self.bMFPCapable == other.bMFPCapable
            && self.uInfraNumSupportedMcastMgmtAlgoPairs == other.uInfraNumSupportedMcastMgmtAlgoPairs
            && self.pInfraSupportedMcastMgmtAlgoPairs == other.pInfraSupportedMcastMgmtAlgoPairs
            && self.bNeighborReportSupported == other.bNeighborReportSupported
            && self.bAPChannelReportSupported == other.bAPChannelReportSupported
            && self.bActionFramesSupported == other.bActionFramesSupported
            && self.bANQPQueryOffloadSupported == other.bANQPQueryOffloadSupported
            && self.bHESSIDConnectionSupported == other.bHESSIDConnectionSupported
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_EXTSTA_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_EXTSTA_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_EXTSTA_CAPABILITY {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uScanSSIDListSize: u32,
    pub uDesiredBSSIDListSize: u32,
    pub uDesiredSSIDListSize: u32,
    pub uExcludedMacAddressListSize: u32,
    pub uPrivacyExemptionListSize: u32,
    pub uKeyMappingTableSize: u32,
    pub uDefaultKeyTableSize: u32,
    pub uWEPKeyValueMaxLength: u32,
    pub uPMKIDCacheSize: u32,
    pub uMaxNumPerSTADefaultKeyTables: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_EXTSTA_CAPABILITY {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_EXTSTA_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_EXTSTA_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_EXTSTA_CAPABILITY")
            .field("Header", &self.Header)
            .field("uScanSSIDListSize", &self.uScanSSIDListSize)
            .field("uDesiredBSSIDListSize", &self.uDesiredBSSIDListSize)
            .field("uDesiredSSIDListSize", &self.uDesiredSSIDListSize)
            .field("uExcludedMacAddressListSize", &self.uExcludedMacAddressListSize)
            .field("uPrivacyExemptionListSize", &self.uPrivacyExemptionListSize)
            .field("uKeyMappingTableSize", &self.uKeyMappingTableSize)
            .field("uDefaultKeyTableSize", &self.uDefaultKeyTableSize)
            .field("uWEPKeyValueMaxLength", &self.uWEPKeyValueMaxLength)
            .field("uPMKIDCacheSize", &self.uPMKIDCacheSize)
            .field("uMaxNumPerSTADefaultKeyTables", &self.uMaxNumPerSTADefaultKeyTables)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_EXTSTA_CAPABILITY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_EXTSTA_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uScanSSIDListSize == other.uScanSSIDListSize && self.uDesiredBSSIDListSize == other.uDesiredBSSIDListSize && self.uDesiredSSIDListSize == other.uDesiredSSIDListSize && self.uExcludedMacAddressListSize == other.uExcludedMacAddressListSize && self.uPrivacyExemptionListSize == other.uPrivacyExemptionListSize && self.uKeyMappingTableSize == other.uKeyMappingTableSize && self.uDefaultKeyTableSize == other.uDefaultKeyTableSize && self.uWEPKeyValueMaxLength == other.uWEPKeyValueMaxLength && self.uPMKIDCacheSize == other.uPMKIDCacheSize && self.uMaxNumPerSTADefaultKeyTables == other.uMaxNumPerSTADefaultKeyTables
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_EXTSTA_CAPABILITY {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_EXTSTA_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_EXTSTA_RECV_CONTEXT {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uReceiveFlags: u32,
    pub uPhyId: u32,
    pub uChCenterFrequency: u32,
    pub usNumberOfMPDUsReceived: u16,
    pub lRSSI: i32,
    pub ucDataRate: u8,
    pub uSizeMediaSpecificInfo: u32,
    pub pvMediaSpecificInfo: *mut ::core::ffi::c_void,
    pub ullTimestamp: u64,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_EXTSTA_RECV_CONTEXT {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_EXTSTA_RECV_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_EXTSTA_RECV_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_EXTSTA_RECV_CONTEXT")
            .field("Header", &self.Header)
            .field("uReceiveFlags", &self.uReceiveFlags)
            .field("uPhyId", &self.uPhyId)
            .field("uChCenterFrequency", &self.uChCenterFrequency)
            .field("usNumberOfMPDUsReceived", &self.usNumberOfMPDUsReceived)
            .field("lRSSI", &self.lRSSI)
            .field("ucDataRate", &self.ucDataRate)
            .field("uSizeMediaSpecificInfo", &self.uSizeMediaSpecificInfo)
            .field("pvMediaSpecificInfo", &self.pvMediaSpecificInfo)
            .field("ullTimestamp", &self.ullTimestamp)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_EXTSTA_RECV_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_EXTSTA_RECV_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uReceiveFlags == other.uReceiveFlags && self.uPhyId == other.uPhyId && self.uChCenterFrequency == other.uChCenterFrequency && self.usNumberOfMPDUsReceived == other.usNumberOfMPDUsReceived && self.lRSSI == other.lRSSI && self.ucDataRate == other.ucDataRate && self.uSizeMediaSpecificInfo == other.uSizeMediaSpecificInfo && self.pvMediaSpecificInfo == other.pvMediaSpecificInfo && self.ullTimestamp == other.ullTimestamp
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_EXTSTA_RECV_CONTEXT {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_EXTSTA_RECV_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_EXTSTA_SEND_CONTEXT {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub usExemptionActionType: u16,
    pub uPhyId: u32,
    pub uDelayedSleepValue: u32,
    pub pvMediaSpecificInfo: *mut ::core::ffi::c_void,
    pub uSendFlags: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_EXTSTA_SEND_CONTEXT {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_EXTSTA_SEND_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_EXTSTA_SEND_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_EXTSTA_SEND_CONTEXT").field("Header", &self.Header).field("usExemptionActionType", &self.usExemptionActionType).field("uPhyId", &self.uPhyId).field("uDelayedSleepValue", &self.uDelayedSleepValue).field("pvMediaSpecificInfo", &self.pvMediaSpecificInfo).field("uSendFlags", &self.uSendFlags).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_EXTSTA_SEND_CONTEXT {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_EXTSTA_SEND_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.usExemptionActionType == other.usExemptionActionType && self.uPhyId == other.uPhyId && self.uDelayedSleepValue == other.uDelayedSleepValue && self.pvMediaSpecificInfo == other.pvMediaSpecificInfo && self.uSendFlags == other.uSendFlags
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_EXTSTA_SEND_CONTEXT {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_EXTSTA_SEND_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_FRAGMENT_DESCRIPTOR {
    pub uOffset: u32,
    pub uLength: u32,
}
impl ::core::marker::Copy for DOT11_FRAGMENT_DESCRIPTOR {}
impl ::core::clone::Clone for DOT11_FRAGMENT_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_FRAGMENT_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_FRAGMENT_DESCRIPTOR").field("uOffset", &self.uOffset).field("uLength", &self.uLength).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_FRAGMENT_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_FRAGMENT_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.uOffset == other.uOffset && self.uLength == other.uLength
    }
}
impl ::core::cmp::Eq for DOT11_FRAGMENT_DESCRIPTOR {}
impl ::core::default::Default for DOT11_FRAGMENT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub Status: i32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("DialogToken", &self.DialogToken).field("Status", &self.Status).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.Status == other.Status && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub Status: i32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("DialogToken", &self.DialogToken).field("Status", &self.Status).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.Status == other.Status && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub Status: i32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("DialogToken", &self.DialogToken).field("Status", &self.Status).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.Status == other.Status && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_HOPPING_PATTERN_ENTRY {
    pub uHoppingPatternIndex: u32,
    pub uRandomTableFieldNumber: u32,
}
impl ::core::marker::Copy for DOT11_HOPPING_PATTERN_ENTRY {}
impl ::core::clone::Clone for DOT11_HOPPING_PATTERN_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_HOPPING_PATTERN_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_HOPPING_PATTERN_ENTRY").field("uHoppingPatternIndex", &self.uHoppingPatternIndex).field("uRandomTableFieldNumber", &self.uRandomTableFieldNumber).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_HOPPING_PATTERN_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_HOPPING_PATTERN_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.uHoppingPatternIndex == other.uHoppingPatternIndex && self.uRandomTableFieldNumber == other.uRandomTableFieldNumber
    }
}
impl ::core::cmp::Eq for DOT11_HOPPING_PATTERN_ENTRY {}
impl ::core::default::Default for DOT11_HOPPING_PATTERN_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_HOPPING_PATTERN_ENTRY_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11HoppingPatternEntry: [DOT11_HOPPING_PATTERN_ENTRY; 1],
}
impl ::core::marker::Copy for DOT11_HOPPING_PATTERN_ENTRY_LIST {}
impl ::core::clone::Clone for DOT11_HOPPING_PATTERN_ENTRY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_HOPPING_PATTERN_ENTRY_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_HOPPING_PATTERN_ENTRY_LIST").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11HoppingPatternEntry", &self.dot11HoppingPatternEntry).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_HOPPING_PATTERN_ENTRY_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_HOPPING_PATTERN_ENTRY_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11HoppingPatternEntry == other.dot11HoppingPatternEntry
    }
}
impl ::core::cmp::Eq for DOT11_HOPPING_PATTERN_ENTRY_LIST {}
impl ::core::default::Default for DOT11_HOPPING_PATTERN_ENTRY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_HRDSSS_PHY_ATTRIBUTES {
    pub bShortPreambleOptionImplemented: super::super::Foundation::BOOLEAN,
    pub bPBCCOptionImplemented: super::super::Foundation::BOOLEAN,
    pub bChannelAgilityPresent: super::super::Foundation::BOOLEAN,
    pub uHRCCAModeSupported: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_HRDSSS_PHY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_HRDSSS_PHY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_HRDSSS_PHY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_HRDSSS_PHY_ATTRIBUTES").field("bShortPreambleOptionImplemented", &self.bShortPreambleOptionImplemented).field("bPBCCOptionImplemented", &self.bPBCCOptionImplemented).field("bChannelAgilityPresent", &self.bChannelAgilityPresent).field("uHRCCAModeSupported", &self.uHRCCAModeSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_HRDSSS_PHY_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_HRDSSS_PHY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.bShortPreambleOptionImplemented == other.bShortPreambleOptionImplemented && self.bPBCCOptionImplemented == other.bPBCCOptionImplemented && self.bChannelAgilityPresent == other.bChannelAgilityPresent && self.uHRCCAModeSupported == other.uHRCCAModeSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_HRDSSS_PHY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_HRDSSS_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_IBSS_PARAMS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub bJoinOnly: super::super::Foundation::BOOLEAN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_IBSS_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_IBSS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_IBSS_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_IBSS_PARAMS").field("Header", &self.Header).field("bJoinOnly", &self.bJoinOnly).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_IBSS_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_IBSS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.bJoinOnly == other.bJoinOnly && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_IBSS_PARAMS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_IBSS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_IHV_VERSION_INFO {
    pub dwVerMin: u32,
    pub dwVerMax: u32,
}
impl ::core::marker::Copy for DOT11_IHV_VERSION_INFO {}
impl ::core::clone::Clone for DOT11_IHV_VERSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_IHV_VERSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_IHV_VERSION_INFO").field("dwVerMin", &self.dwVerMin).field("dwVerMax", &self.dwVerMax).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_IHV_VERSION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_IHV_VERSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dwVerMin == other.dwVerMin && self.dwVerMax == other.dwVerMax
    }
}
impl ::core::cmp::Eq for DOT11_IHV_VERSION_INFO {}
impl ::core::default::Default for DOT11_IHV_VERSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: [u8; 6],
    pub uStatus: u32,
    pub ucErrorSource: u8,
    pub bReAssocReq: super::super::Foundation::BOOLEAN,
    pub bReAssocResp: super::super::Foundation::BOOLEAN,
    pub uAssocReqOffset: u32,
    pub uAssocReqSize: u32,
    pub uAssocRespOffset: u32,
    pub uAssocRespSize: u32,
    pub AuthAlgo: DOT11_AUTH_ALGORITHM,
    pub UnicastCipher: DOT11_CIPHER_ALGORITHM,
    pub MulticastCipher: DOT11_CIPHER_ALGORITHM,
    pub uActivePhyListOffset: u32,
    pub uActivePhyListSize: u32,
    pub uBeaconOffset: u32,
    pub uBeaconSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS")
            .field("Header", &self.Header)
            .field("PeerMacAddr", &self.PeerMacAddr)
            .field("uStatus", &self.uStatus)
            .field("ucErrorSource", &self.ucErrorSource)
            .field("bReAssocReq", &self.bReAssocReq)
            .field("bReAssocResp", &self.bReAssocResp)
            .field("uAssocReqOffset", &self.uAssocReqOffset)
            .field("uAssocReqSize", &self.uAssocReqSize)
            .field("uAssocRespOffset", &self.uAssocRespOffset)
            .field("uAssocRespSize", &self.uAssocRespSize)
            .field("AuthAlgo", &self.AuthAlgo)
            .field("UnicastCipher", &self.UnicastCipher)
            .field("MulticastCipher", &self.MulticastCipher)
            .field("uActivePhyListOffset", &self.uActivePhyListOffset)
            .field("uActivePhyListSize", &self.uActivePhyListSize)
            .field("uBeaconOffset", &self.uBeaconOffset)
            .field("uBeaconSize", &self.uBeaconSize)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerMacAddr == other.PeerMacAddr && self.uStatus == other.uStatus && self.ucErrorSource == other.ucErrorSource && self.bReAssocReq == other.bReAssocReq && self.bReAssocResp == other.bReAssocResp && self.uAssocReqOffset == other.uAssocReqOffset && self.uAssocReqSize == other.uAssocReqSize && self.uAssocRespOffset == other.uAssocRespOffset && self.uAssocRespSize == other.uAssocRespSize && self.AuthAlgo == other.AuthAlgo && self.UnicastCipher == other.UnicastCipher && self.MulticastCipher == other.MulticastCipher && self.uActivePhyListOffset == other.uActivePhyListOffset && self.uActivePhyListSize == other.uActivePhyListSize && self.uBeaconOffset == other.uBeaconOffset && self.uBeaconSize == other.uBeaconSize
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_INCOMING_ASSOC_DECISION {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: [u8; 6],
    pub bAccept: super::super::Foundation::BOOLEAN,
    pub usReasonCode: u16,
    pub uAssocResponseIEsOffset: u32,
    pub uAssocResponseIEsLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_INCOMING_ASSOC_DECISION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_INCOMING_ASSOC_DECISION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_INCOMING_ASSOC_DECISION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_INCOMING_ASSOC_DECISION").field("Header", &self.Header).field("PeerMacAddr", &self.PeerMacAddr).field("bAccept", &self.bAccept).field("usReasonCode", &self.usReasonCode).field("uAssocResponseIEsOffset", &self.uAssocResponseIEsOffset).field("uAssocResponseIEsLength", &self.uAssocResponseIEsLength).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_INCOMING_ASSOC_DECISION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_DECISION {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerMacAddr == other.PeerMacAddr && self.bAccept == other.bAccept && self.usReasonCode == other.usReasonCode && self.uAssocResponseIEsOffset == other.uAssocResponseIEsOffset && self.uAssocResponseIEsLength == other.uAssocResponseIEsLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_DECISION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_DECISION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_INCOMING_ASSOC_DECISION_V2 {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: [u8; 6],
    pub bAccept: super::super::Foundation::BOOLEAN,
    pub usReasonCode: u16,
    pub uAssocResponseIEsOffset: u32,
    pub uAssocResponseIEsLength: u32,
    pub WFDStatus: u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_INCOMING_ASSOC_DECISION_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_INCOMING_ASSOC_DECISION_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_INCOMING_ASSOC_DECISION_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_INCOMING_ASSOC_DECISION_V2").field("Header", &self.Header).field("PeerMacAddr", &self.PeerMacAddr).field("bAccept", &self.bAccept).field("usReasonCode", &self.usReasonCode).field("uAssocResponseIEsOffset", &self.uAssocResponseIEsOffset).field("uAssocResponseIEsLength", &self.uAssocResponseIEsLength).field("WFDStatus", &self.WFDStatus).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_INCOMING_ASSOC_DECISION_V2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_DECISION_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerMacAddr == other.PeerMacAddr && self.bAccept == other.bAccept && self.usReasonCode == other.usReasonCode && self.uAssocResponseIEsOffset == other.uAssocResponseIEsOffset && self.uAssocResponseIEsLength == other.uAssocResponseIEsLength && self.WFDStatus == other.WFDStatus
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_DECISION_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_DECISION_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: [u8; 6],
    pub bReAssocReq: super::super::Foundation::BOOLEAN,
    pub uAssocReqOffset: u32,
    pub uAssocReqSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS").field("Header", &self.Header).field("PeerMacAddr", &self.PeerMacAddr).field("bReAssocReq", &self.bReAssocReq).field("uAssocReqOffset", &self.uAssocReqOffset).field("uAssocReqSize", &self.uAssocReqSize).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerMacAddr == other.PeerMacAddr && self.bReAssocReq == other.bReAssocReq && self.uAssocReqOffset == other.uAssocReqOffset && self.uAssocReqSize == other.uAssocReqSize
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerMacAddr: [u8; 6],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_INCOMING_ASSOC_STARTED_PARAMETERS").field("Header", &self.Header).field("PeerMacAddr", &self.PeerMacAddr).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerMacAddr == other.PeerMacAddr
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_INCOMING_ASSOC_STARTED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub ReceiverAddress: [u8; 6],
    pub DialogToken: u8,
    pub Status: i32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("ReceiverAddress", &self.ReceiverAddress).field("DialogToken", &self.DialogToken).field("Status", &self.Status).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.ReceiverAddress == other.ReceiverAddress && self.DialogToken == other.DialogToken && self.Status == other.Status && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub Status: i32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS").field("Header", &self.Header).field("ReceiverDeviceAddress", &self.ReceiverDeviceAddress).field("DialogToken", &self.DialogToken).field("Status", &self.Status).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ReceiverDeviceAddress == other.ReceiverDeviceAddress && self.DialogToken == other.DialogToken && self.Status == other.Status && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_IV48_COUNTER {
    pub uIV32Counter: u32,
    pub usIV16Counter: u16,
}
impl ::core::marker::Copy for DOT11_IV48_COUNTER {}
impl ::core::clone::Clone for DOT11_IV48_COUNTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_IV48_COUNTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_IV48_COUNTER").field("uIV32Counter", &self.uIV32Counter).field("usIV16Counter", &self.usIV16Counter).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_IV48_COUNTER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_IV48_COUNTER {
    fn eq(&self, other: &Self) -> bool {
        self.uIV32Counter == other.uIV32Counter && self.usIV16Counter == other.usIV16Counter
    }
}
impl ::core::cmp::Eq for DOT11_IV48_COUNTER {}
impl ::core::default::Default for DOT11_IV48_COUNTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_JOIN_REQUEST {
    pub uJoinFailureTimeout: u32,
    pub OperationalRateSet: DOT11_RATE_SET,
    pub uChCenterFrequency: u32,
    pub dot11BSSDescription: DOT11_BSS_DESCRIPTION,
}
impl ::core::marker::Copy for DOT11_JOIN_REQUEST {}
impl ::core::clone::Clone for DOT11_JOIN_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_JOIN_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_JOIN_REQUEST").field("uJoinFailureTimeout", &self.uJoinFailureTimeout).field("OperationalRateSet", &self.OperationalRateSet).field("uChCenterFrequency", &self.uChCenterFrequency).field("dot11BSSDescription", &self.dot11BSSDescription).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_JOIN_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_JOIN_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.uJoinFailureTimeout == other.uJoinFailureTimeout && self.OperationalRateSet == other.OperationalRateSet && self.uChCenterFrequency == other.uChCenterFrequency && self.dot11BSSDescription == other.dot11BSSDescription
    }
}
impl ::core::cmp::Eq for DOT11_JOIN_REQUEST {}
impl ::core::default::Default for DOT11_JOIN_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_KEY_ALGO_BIP {
    pub ucIPN: [u8; 6],
    pub ulBIPKeyLength: u32,
    pub ucBIPKey: [u8; 1],
}
impl ::core::marker::Copy for DOT11_KEY_ALGO_BIP {}
impl ::core::clone::Clone for DOT11_KEY_ALGO_BIP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_KEY_ALGO_BIP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_KEY_ALGO_BIP").field("ucIPN", &self.ucIPN).field("ulBIPKeyLength", &self.ulBIPKeyLength).field("ucBIPKey", &self.ucBIPKey).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_KEY_ALGO_BIP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_BIP {
    fn eq(&self, other: &Self) -> bool {
        self.ucIPN == other.ucIPN && self.ulBIPKeyLength == other.ulBIPKeyLength && self.ucBIPKey == other.ucBIPKey
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_BIP {}
impl ::core::default::Default for DOT11_KEY_ALGO_BIP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_KEY_ALGO_BIP_GMAC_256 {
    pub ucIPN: [u8; 6],
    pub ulBIPGmac256KeyLength: u32,
    pub ucBIPGmac256Key: [u8; 1],
}
impl ::core::marker::Copy for DOT11_KEY_ALGO_BIP_GMAC_256 {}
impl ::core::clone::Clone for DOT11_KEY_ALGO_BIP_GMAC_256 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_KEY_ALGO_BIP_GMAC_256 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_KEY_ALGO_BIP_GMAC_256").field("ucIPN", &self.ucIPN).field("ulBIPGmac256KeyLength", &self.ulBIPGmac256KeyLength).field("ucBIPGmac256Key", &self.ucBIPGmac256Key).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_KEY_ALGO_BIP_GMAC_256 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_BIP_GMAC_256 {
    fn eq(&self, other: &Self) -> bool {
        self.ucIPN == other.ucIPN && self.ulBIPGmac256KeyLength == other.ulBIPGmac256KeyLength && self.ucBIPGmac256Key == other.ucBIPGmac256Key
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_BIP_GMAC_256 {}
impl ::core::default::Default for DOT11_KEY_ALGO_BIP_GMAC_256 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_KEY_ALGO_CCMP {
    pub ucIV48Counter: [u8; 6],
    pub ulCCMPKeyLength: u32,
    pub ucCCMPKey: [u8; 1],
}
impl ::core::marker::Copy for DOT11_KEY_ALGO_CCMP {}
impl ::core::clone::Clone for DOT11_KEY_ALGO_CCMP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_KEY_ALGO_CCMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_KEY_ALGO_CCMP").field("ucIV48Counter", &self.ucIV48Counter).field("ulCCMPKeyLength", &self.ulCCMPKeyLength).field("ucCCMPKey", &self.ucCCMPKey).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_KEY_ALGO_CCMP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_CCMP {
    fn eq(&self, other: &Self) -> bool {
        self.ucIV48Counter == other.ucIV48Counter && self.ulCCMPKeyLength == other.ulCCMPKeyLength && self.ucCCMPKey == other.ucCCMPKey
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_CCMP {}
impl ::core::default::Default for DOT11_KEY_ALGO_CCMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_KEY_ALGO_GCMP {
    pub ucIV48Counter: [u8; 6],
    pub ulGCMPKeyLength: u32,
    pub ucGCMPKey: [u8; 1],
}
impl ::core::marker::Copy for DOT11_KEY_ALGO_GCMP {}
impl ::core::clone::Clone for DOT11_KEY_ALGO_GCMP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_KEY_ALGO_GCMP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_KEY_ALGO_GCMP").field("ucIV48Counter", &self.ucIV48Counter).field("ulGCMPKeyLength", &self.ulGCMPKeyLength).field("ucGCMPKey", &self.ucGCMPKey).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_KEY_ALGO_GCMP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_GCMP {
    fn eq(&self, other: &Self) -> bool {
        self.ucIV48Counter == other.ucIV48Counter && self.ulGCMPKeyLength == other.ulGCMPKeyLength && self.ucGCMPKey == other.ucGCMPKey
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_GCMP {}
impl ::core::default::Default for DOT11_KEY_ALGO_GCMP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_KEY_ALGO_GCMP_256 {
    pub ucIV48Counter: [u8; 6],
    pub ulGCMP256KeyLength: u32,
    pub ucGCMP256Key: [u8; 1],
}
impl ::core::marker::Copy for DOT11_KEY_ALGO_GCMP_256 {}
impl ::core::clone::Clone for DOT11_KEY_ALGO_GCMP_256 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_KEY_ALGO_GCMP_256 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_KEY_ALGO_GCMP_256").field("ucIV48Counter", &self.ucIV48Counter).field("ulGCMP256KeyLength", &self.ulGCMP256KeyLength).field("ucGCMP256Key", &self.ucGCMP256Key).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_KEY_ALGO_GCMP_256 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_GCMP_256 {
    fn eq(&self, other: &Self) -> bool {
        self.ucIV48Counter == other.ucIV48Counter && self.ulGCMP256KeyLength == other.ulGCMP256KeyLength && self.ucGCMP256Key == other.ucGCMP256Key
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_GCMP_256 {}
impl ::core::default::Default for DOT11_KEY_ALGO_GCMP_256 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_KEY_ALGO_TKIP_MIC {
    pub ucIV48Counter: [u8; 6],
    pub ulTKIPKeyLength: u32,
    pub ulMICKeyLength: u32,
    pub ucTKIPMICKeys: [u8; 1],
}
impl ::core::marker::Copy for DOT11_KEY_ALGO_TKIP_MIC {}
impl ::core::clone::Clone for DOT11_KEY_ALGO_TKIP_MIC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_KEY_ALGO_TKIP_MIC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_KEY_ALGO_TKIP_MIC").field("ucIV48Counter", &self.ucIV48Counter).field("ulTKIPKeyLength", &self.ulTKIPKeyLength).field("ulMICKeyLength", &self.ulMICKeyLength).field("ucTKIPMICKeys", &self.ucTKIPMICKeys).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_KEY_ALGO_TKIP_MIC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_KEY_ALGO_TKIP_MIC {
    fn eq(&self, other: &Self) -> bool {
        self.ucIV48Counter == other.ucIV48Counter && self.ulTKIPKeyLength == other.ulTKIPKeyLength && self.ulMICKeyLength == other.ulMICKeyLength && self.ucTKIPMICKeys == other.ucTKIPMICKeys
    }
}
impl ::core::cmp::Eq for DOT11_KEY_ALGO_TKIP_MIC {}
impl ::core::default::Default for DOT11_KEY_ALGO_TKIP_MIC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_LINK_QUALITY_ENTRY {
    pub PeerMacAddr: [u8; 6],
    pub ucLinkQuality: u8,
}
impl ::core::marker::Copy for DOT11_LINK_QUALITY_ENTRY {}
impl ::core::clone::Clone for DOT11_LINK_QUALITY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_LINK_QUALITY_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_LINK_QUALITY_ENTRY").field("PeerMacAddr", &self.PeerMacAddr).field("ucLinkQuality", &self.ucLinkQuality).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_LINK_QUALITY_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_LINK_QUALITY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.PeerMacAddr == other.PeerMacAddr && self.ucLinkQuality == other.ucLinkQuality
    }
}
impl ::core::cmp::Eq for DOT11_LINK_QUALITY_ENTRY {}
impl ::core::default::Default for DOT11_LINK_QUALITY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_LINK_QUALITY_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uLinkQualityListSize: u32,
    pub uLinkQualityListOffset: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_LINK_QUALITY_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_LINK_QUALITY_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_LINK_QUALITY_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_LINK_QUALITY_PARAMETERS").field("Header", &self.Header).field("uLinkQualityListSize", &self.uLinkQualityListSize).field("uLinkQualityListOffset", &self.uLinkQualityListOffset).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_LINK_QUALITY_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_LINK_QUALITY_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uLinkQualityListSize == other.uLinkQualityListSize && self.uLinkQualityListOffset == other.uLinkQualityListOffset
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_LINK_QUALITY_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_LINK_QUALITY_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_MAC_ADDRESS_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub MacAddrs: [u8; 6],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_MAC_ADDRESS_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_MAC_ADDRESS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_MAC_ADDRESS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MAC_ADDRESS_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("MacAddrs", &self.MacAddrs).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_MAC_ADDRESS_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_MAC_ADDRESS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.MacAddrs == other.MacAddrs
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_MAC_ADDRESS_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_MAC_ADDRESS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_MAC_FRAME_STATISTICS {
    pub ullTransmittedFrameCount: u64,
    pub ullReceivedFrameCount: u64,
    pub ullTransmittedFailureFrameCount: u64,
    pub ullReceivedFailureFrameCount: u64,
    pub ullWEPExcludedCount: u64,
    pub ullTKIPLocalMICFailures: u64,
    pub ullTKIPReplays: u64,
    pub ullTKIPICVErrorCount: u64,
    pub ullCCMPReplays: u64,
    pub ullCCMPDecryptErrors: u64,
    pub ullWEPUndecryptableCount: u64,
    pub ullWEPICVErrorCount: u64,
    pub ullDecryptSuccessCount: u64,
    pub ullDecryptFailureCount: u64,
}
impl ::core::marker::Copy for DOT11_MAC_FRAME_STATISTICS {}
impl ::core::clone::Clone for DOT11_MAC_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_MAC_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MAC_FRAME_STATISTICS")
            .field("ullTransmittedFrameCount", &self.ullTransmittedFrameCount)
            .field("ullReceivedFrameCount", &self.ullReceivedFrameCount)
            .field("ullTransmittedFailureFrameCount", &self.ullTransmittedFailureFrameCount)
            .field("ullReceivedFailureFrameCount", &self.ullReceivedFailureFrameCount)
            .field("ullWEPExcludedCount", &self.ullWEPExcludedCount)
            .field("ullTKIPLocalMICFailures", &self.ullTKIPLocalMICFailures)
            .field("ullTKIPReplays", &self.ullTKIPReplays)
            .field("ullTKIPICVErrorCount", &self.ullTKIPICVErrorCount)
            .field("ullCCMPReplays", &self.ullCCMPReplays)
            .field("ullCCMPDecryptErrors", &self.ullCCMPDecryptErrors)
            .field("ullWEPUndecryptableCount", &self.ullWEPUndecryptableCount)
            .field("ullWEPICVErrorCount", &self.ullWEPICVErrorCount)
            .field("ullDecryptSuccessCount", &self.ullDecryptSuccessCount)
            .field("ullDecryptFailureCount", &self.ullDecryptFailureCount)
            .finish()
    }
}
impl ::windows::core::TypeKind for DOT11_MAC_FRAME_STATISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_MAC_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.ullTransmittedFrameCount == other.ullTransmittedFrameCount
            && self.ullReceivedFrameCount == other.ullReceivedFrameCount
            && self.ullTransmittedFailureFrameCount == other.ullTransmittedFailureFrameCount
            && self.ullReceivedFailureFrameCount == other.ullReceivedFailureFrameCount
            && self.ullWEPExcludedCount == other.ullWEPExcludedCount
            && self.ullTKIPLocalMICFailures == other.ullTKIPLocalMICFailures
            && self.ullTKIPReplays == other.ullTKIPReplays
            && self.ullTKIPICVErrorCount == other.ullTKIPICVErrorCount
            && self.ullCCMPReplays == other.ullCCMPReplays
            && self.ullCCMPDecryptErrors == other.ullCCMPDecryptErrors
            && self.ullWEPUndecryptableCount == other.ullWEPUndecryptableCount
            && self.ullWEPICVErrorCount == other.ullWEPICVErrorCount
            && self.ullDecryptSuccessCount == other.ullDecryptSuccessCount
            && self.ullDecryptFailureCount == other.ullDecryptFailureCount
    }
}
impl ::core::cmp::Eq for DOT11_MAC_FRAME_STATISTICS {}
impl ::core::default::Default for DOT11_MAC_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_MAC_INFO {
    pub uReserved: u32,
    pub uNdisPortNumber: u32,
    pub MacAddr: [u8; 6],
}
impl ::core::marker::Copy for DOT11_MAC_INFO {}
impl ::core::clone::Clone for DOT11_MAC_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_MAC_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MAC_INFO").field("uReserved", &self.uReserved).field("uNdisPortNumber", &self.uNdisPortNumber).field("MacAddr", &self.MacAddr).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_MAC_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_MAC_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.uNdisPortNumber == other.uNdisPortNumber && self.MacAddr == other.MacAddr
    }
}
impl ::core::cmp::Eq for DOT11_MAC_INFO {}
impl ::core::default::Default for DOT11_MAC_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_MAC_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uOpmodeMask: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_MAC_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_MAC_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_MAC_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MAC_PARAMETERS").field("Header", &self.Header).field("uOpmodeMask", &self.uOpmodeMask).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_MAC_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_MAC_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uOpmodeMask == other.uOpmodeMask
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_MAC_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_MAC_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub dot11ManufacturingCallbackType: DOT11_MANUFACTURING_CALLBACK_TYPE,
    pub uStatus: u32,
    pub pvContext: *mut ::core::ffi::c_void,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_CALLBACK_PARAMETERS").field("Header", &self.Header).field("dot11ManufacturingCallbackType", &self.dot11ManufacturingCallbackType).field("uStatus", &self.uStatus).field("pvContext", &self.pvContext).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.dot11ManufacturingCallbackType == other.dot11ManufacturingCallbackType && self.uStatus == other.uStatus && self.pvContext == other.pvContext
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_MANUFACTURING_CALLBACK_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    pub Dot11Band: DOT11_BAND,
    pub uChannel: u32,
    pub ADCPowerLevel: i32,
}
impl ::core::marker::Copy for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC").field("Dot11Band", &self.Dot11Band).field("uChannel", &self.uChannel).field("ADCPowerLevel", &self.ADCPowerLevel).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    fn eq(&self, other: &Self) -> bool {
        self.Dot11Band == other.Dot11Band && self.uChannel == other.uChannel && self.ADCPowerLevel == other.ADCPowerLevel
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {}
impl ::core::default::Default for DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    pub bEnabled: super::super::Foundation::BOOLEAN,
    pub Dot11Band: DOT11_BAND,
    pub uChannel: u32,
    pub PowerLevel: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX").field("bEnabled", &self.bEnabled).field("Dot11Band", &self.Dot11Band).field("uChannel", &self.uChannel).field("PowerLevel", &self.PowerLevel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    fn eq(&self, other: &Self) -> bool {
        self.bEnabled == other.bEnabled && self.Dot11Band == other.Dot11Band && self.uChannel == other.uChannel && self.PowerLevel == other.PowerLevel
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    pub bEnable: super::super::Foundation::BOOLEAN,
    pub bOpenLoop: super::super::Foundation::BOOLEAN,
    pub Dot11Band: DOT11_BAND,
    pub uChannel: u32,
    pub uSetPowerLevel: u32,
    pub ADCPowerLevel: i32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX").field("bEnable", &self.bEnable).field("bOpenLoop", &self.bOpenLoop).field("Dot11Band", &self.Dot11Band).field("uChannel", &self.uChannel).field("uSetPowerLevel", &self.uSetPowerLevel).field("ADCPowerLevel", &self.ADCPowerLevel).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    fn eq(&self, other: &Self) -> bool {
        self.bEnable == other.bEnable && self.bOpenLoop == other.bOpenLoop && self.Dot11Band == other.Dot11Band && self.uChannel == other.uChannel && self.uSetPowerLevel == other.uSetPowerLevel && self.ADCPowerLevel == other.ADCPowerLevel
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    pub SelfTestType: DOT11_MANUFACTURING_SELF_TEST_TYPE,
    pub uTestID: u32,
    pub bResult: super::super::Foundation::BOOLEAN,
    pub uPinFailedBitMask: u32,
    pub pvContext: *mut ::core::ffi::c_void,
    pub uBytesWrittenOut: u32,
    pub ucBufferOut: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS").field("SelfTestType", &self.SelfTestType).field("uTestID", &self.uTestID).field("bResult", &self.bResult).field("uPinFailedBitMask", &self.uPinFailedBitMask).field("pvContext", &self.pvContext).field("uBytesWrittenOut", &self.uBytesWrittenOut).field("ucBufferOut", &self.ucBufferOut).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    fn eq(&self, other: &Self) -> bool {
        self.SelfTestType == other.SelfTestType && self.uTestID == other.uTestID && self.bResult == other.bResult && self.uPinFailedBitMask == other.uPinFailedBitMask && self.pvContext == other.pvContext && self.uBytesWrittenOut == other.uBytesWrittenOut && self.ucBufferOut == other.ucBufferOut
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    pub SelfTestType: DOT11_MANUFACTURING_SELF_TEST_TYPE,
    pub uTestID: u32,
    pub uPinBitMask: u32,
    pub pvContext: *mut ::core::ffi::c_void,
    pub uBufferLength: u32,
    pub ucBufferIn: [u8; 1],
}
impl ::core::marker::Copy for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS").field("SelfTestType", &self.SelfTestType).field("uTestID", &self.uTestID).field("uPinBitMask", &self.uPinBitMask).field("pvContext", &self.pvContext).field("uBufferLength", &self.uBufferLength).field("ucBufferIn", &self.ucBufferIn).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.SelfTestType == other.SelfTestType && self.uTestID == other.uTestID && self.uPinBitMask == other.uPinBitMask && self.pvContext == other.pvContext && self.uBufferLength == other.uBufferLength && self.ucBufferIn == other.ucBufferIn
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {}
impl ::core::default::Default for DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_MANUFACTURING_TEST {
    pub dot11ManufacturingTestType: DOT11_MANUFACTURING_TEST_TYPE,
    pub uBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
impl ::core::marker::Copy for DOT11_MANUFACTURING_TEST {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_TEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_TEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_TEST").field("dot11ManufacturingTestType", &self.dot11ManufacturingTestType).field("uBufferLength", &self.uBufferLength).field("ucBuffer", &self.ucBuffer).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_MANUFACTURING_TEST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_TEST {
    fn eq(&self, other: &Self) -> bool {
        self.dot11ManufacturingTestType == other.dot11ManufacturingTestType && self.uBufferLength == other.uBufferLength && self.ucBuffer == other.ucBuffer
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_TEST {}
impl ::core::default::Default for DOT11_MANUFACTURING_TEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_MANUFACTURING_TEST_QUERY_DATA {
    pub uKey: u32,
    pub uOffset: u32,
    pub uBufferLength: u32,
    pub uBytesRead: u32,
    pub ucBufferOut: [u8; 1],
}
impl ::core::marker::Copy for DOT11_MANUFACTURING_TEST_QUERY_DATA {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_TEST_QUERY_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_TEST_QUERY_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_TEST_QUERY_DATA").field("uKey", &self.uKey).field("uOffset", &self.uOffset).field("uBufferLength", &self.uBufferLength).field("uBytesRead", &self.uBytesRead).field("ucBufferOut", &self.ucBufferOut).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_MANUFACTURING_TEST_QUERY_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_TEST_QUERY_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.uKey == other.uKey && self.uOffset == other.uOffset && self.uBufferLength == other.uBufferLength && self.uBytesRead == other.uBytesRead && self.ucBufferOut == other.ucBufferOut
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_TEST_QUERY_DATA {}
impl ::core::default::Default for DOT11_MANUFACTURING_TEST_QUERY_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_MANUFACTURING_TEST_SET_DATA {
    pub uKey: u32,
    pub uOffset: u32,
    pub uBufferLength: u32,
    pub ucBufferIn: [u8; 1],
}
impl ::core::marker::Copy for DOT11_MANUFACTURING_TEST_SET_DATA {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_TEST_SET_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_TEST_SET_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_TEST_SET_DATA").field("uKey", &self.uKey).field("uOffset", &self.uOffset).field("uBufferLength", &self.uBufferLength).field("ucBufferIn", &self.ucBufferIn).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_MANUFACTURING_TEST_SET_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_TEST_SET_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.uKey == other.uKey && self.uOffset == other.uOffset && self.uBufferLength == other.uBufferLength && self.ucBufferIn == other.ucBufferIn
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_TEST_SET_DATA {}
impl ::core::default::Default for DOT11_MANUFACTURING_TEST_SET_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_MANUFACTURING_TEST_SLEEP {
    pub uSleepTime: u32,
    pub pvContext: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for DOT11_MANUFACTURING_TEST_SLEEP {}
impl ::core::clone::Clone for DOT11_MANUFACTURING_TEST_SLEEP {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_MANUFACTURING_TEST_SLEEP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MANUFACTURING_TEST_SLEEP").field("uSleepTime", &self.uSleepTime).field("pvContext", &self.pvContext).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_MANUFACTURING_TEST_SLEEP {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_MANUFACTURING_TEST_SLEEP {
    fn eq(&self, other: &Self) -> bool {
        self.uSleepTime == other.uSleepTime && self.pvContext == other.pvContext
    }
}
impl ::core::cmp::Eq for DOT11_MANUFACTURING_TEST_SLEEP {}
impl ::core::default::Default for DOT11_MANUFACTURING_TEST_SLEEP {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_MD_CAPABILITY_ENTRY_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11MDCapabilityEntry: [DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY; 1],
}
impl ::core::marker::Copy for DOT11_MD_CAPABILITY_ENTRY_LIST {}
impl ::core::clone::Clone for DOT11_MD_CAPABILITY_ENTRY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_MD_CAPABILITY_ENTRY_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MD_CAPABILITY_ENTRY_LIST").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11MDCapabilityEntry", &self.dot11MDCapabilityEntry).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_MD_CAPABILITY_ENTRY_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_MD_CAPABILITY_ENTRY_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11MDCapabilityEntry == other.dot11MDCapabilityEntry
    }
}
impl ::core::cmp::Eq for DOT11_MD_CAPABILITY_ENTRY_LIST {}
impl ::core::default::Default for DOT11_MD_CAPABILITY_ENTRY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_MPDU_MAX_LENGTH_INDICATION {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uPhyId: u32,
    pub uMPDUMaxLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_MPDU_MAX_LENGTH_INDICATION {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_MPDU_MAX_LENGTH_INDICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_MPDU_MAX_LENGTH_INDICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MPDU_MAX_LENGTH_INDICATION").field("Header", &self.Header).field("uPhyId", &self.uPhyId).field("uMPDUMaxLength", &self.uMPDUMaxLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_MPDU_MAX_LENGTH_INDICATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_MPDU_MAX_LENGTH_INDICATION {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uPhyId == other.uPhyId && self.uMPDUMaxLength == other.uMPDUMaxLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_MPDU_MAX_LENGTH_INDICATION {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_MPDU_MAX_LENGTH_INDICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
pub struct DOT11_MSONEX_RESULT_PARAMS {
    pub Dot11OnexAuthStatus: ONEX_AUTH_STATUS,
    pub Dot11OneXReasonCode: ONEX_REASON_CODE,
    pub pbMPPESendKey: *mut u8,
    pub dwMPPESendKeyLen: u32,
    pub pbMPPERecvKey: *mut u8,
    pub dwMPPERecvKeyLen: u32,
    pub pDot11EapResult: *mut DOT11_EAP_RESULT,
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::marker::Copy for DOT11_MSONEX_RESULT_PARAMS {}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::clone::Clone for DOT11_MSONEX_RESULT_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::fmt::Debug for DOT11_MSONEX_RESULT_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MSONEX_RESULT_PARAMS").field("Dot11OnexAuthStatus", &self.Dot11OnexAuthStatus).field("Dot11OneXReasonCode", &self.Dot11OneXReasonCode).field("pbMPPESendKey", &self.pbMPPESendKey).field("dwMPPESendKeyLen", &self.dwMPPESendKeyLen).field("pbMPPERecvKey", &self.pbMPPERecvKey).field("dwMPPERecvKeyLen", &self.dwMPPERecvKeyLen).field("pDot11EapResult", &self.pDot11EapResult).finish()
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::windows::core::TypeKind for DOT11_MSONEX_RESULT_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::cmp::PartialEq for DOT11_MSONEX_RESULT_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Dot11OnexAuthStatus == other.Dot11OnexAuthStatus && self.Dot11OneXReasonCode == other.Dot11OneXReasonCode && self.pbMPPESendKey == other.pbMPPESendKey && self.dwMPPESendKeyLen == other.dwMPPESendKeyLen && self.pbMPPERecvKey == other.pbMPPERecvKey && self.dwMPPERecvKeyLen == other.dwMPPERecvKeyLen && self.pDot11EapResult == other.pDot11EapResult
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::cmp::Eq for DOT11_MSONEX_RESULT_PARAMS {}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::default::Default for DOT11_MSONEX_RESULT_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub struct DOT11_MSSECURITY_SETTINGS {
    pub dot11AuthAlgorithm: DOT11_AUTH_ALGORITHM,
    pub dot11CipherAlgorithm: DOT11_CIPHER_ALGORITHM,
    pub fOneXEnabled: super::super::Foundation::BOOL,
    pub eapMethodType: super::super::Security::ExtensibleAuthenticationProtocol::EAP_METHOD_TYPE,
    pub dwEapConnectionDataLen: u32,
    pub pEapConnectionData: *mut u8,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::marker::Copy for DOT11_MSSECURITY_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::clone::Clone for DOT11_MSSECURITY_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::fmt::Debug for DOT11_MSSECURITY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MSSECURITY_SETTINGS").field("dot11AuthAlgorithm", &self.dot11AuthAlgorithm).field("dot11CipherAlgorithm", &self.dot11CipherAlgorithm).field("fOneXEnabled", &self.fOneXEnabled).field("eapMethodType", &self.eapMethodType).field("dwEapConnectionDataLen", &self.dwEapConnectionDataLen).field("pEapConnectionData", &self.pEapConnectionData).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::windows::core::TypeKind for DOT11_MSSECURITY_SETTINGS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::cmp::PartialEq for DOT11_MSSECURITY_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dot11AuthAlgorithm == other.dot11AuthAlgorithm && self.dot11CipherAlgorithm == other.dot11CipherAlgorithm && self.fOneXEnabled == other.fOneXEnabled && self.eapMethodType == other.eapMethodType && self.dwEapConnectionDataLen == other.dwEapConnectionDataLen && self.pEapConnectionData == other.pEapConnectionData
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::cmp::Eq for DOT11_MSSECURITY_SETTINGS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
impl ::core::default::Default for DOT11_MSSECURITY_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    pub uMultiDomainCapabilityIndex: u32,
    pub uFirstChannelNumber: u32,
    pub uNumberOfChannels: u32,
    pub lMaximumTransmitPowerLevel: i32,
}
impl ::core::marker::Copy for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {}
impl ::core::clone::Clone for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY").field("uMultiDomainCapabilityIndex", &self.uMultiDomainCapabilityIndex).field("uFirstChannelNumber", &self.uFirstChannelNumber).field("uNumberOfChannels", &self.uNumberOfChannels).field("lMaximumTransmitPowerLevel", &self.lMaximumTransmitPowerLevel).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.uMultiDomainCapabilityIndex == other.uMultiDomainCapabilityIndex && self.uFirstChannelNumber == other.uFirstChannelNumber && self.uNumberOfChannels == other.uNumberOfChannels && self.lMaximumTransmitPowerLevel == other.lMaximumTransmitPowerLevel
    }
}
impl ::core::cmp::Eq for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {}
impl ::core::default::Default for DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_NETWORK {
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
}
impl ::core::marker::Copy for DOT11_NETWORK {}
impl ::core::clone::Clone for DOT11_NETWORK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_NETWORK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_NETWORK").field("dot11Ssid", &self.dot11Ssid).field("dot11BssType", &self.dot11BssType).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_NETWORK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_NETWORK {
    fn eq(&self, other: &Self) -> bool {
        self.dot11Ssid == other.dot11Ssid && self.dot11BssType == other.dot11BssType
    }
}
impl ::core::cmp::Eq for DOT11_NETWORK {}
impl ::core::default::Default for DOT11_NETWORK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_NETWORK_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub Network: [DOT11_NETWORK; 1],
}
impl ::core::marker::Copy for DOT11_NETWORK_LIST {}
impl ::core::clone::Clone for DOT11_NETWORK_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_NETWORK_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_NETWORK_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("dwIndex", &self.dwIndex).field("Network", &self.Network).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_NETWORK_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_NETWORK_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.dwIndex == other.dwIndex && self.Network == other.Network
    }
}
impl ::core::cmp::Eq for DOT11_NETWORK_LIST {}
impl ::core::default::Default for DOT11_NETWORK_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_NIC_SPECIFIC_EXTENSION {
    pub uBufferLength: u32,
    pub uTotalBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
impl ::core::marker::Copy for DOT11_NIC_SPECIFIC_EXTENSION {}
impl ::core::clone::Clone for DOT11_NIC_SPECIFIC_EXTENSION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_NIC_SPECIFIC_EXTENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_NIC_SPECIFIC_EXTENSION").field("uBufferLength", &self.uBufferLength).field("uTotalBufferLength", &self.uTotalBufferLength).field("ucBuffer", &self.ucBuffer).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_NIC_SPECIFIC_EXTENSION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_NIC_SPECIFIC_EXTENSION {
    fn eq(&self, other: &Self) -> bool {
        self.uBufferLength == other.uBufferLength && self.uTotalBufferLength == other.uTotalBufferLength && self.ucBuffer == other.ucBuffer
    }
}
impl ::core::cmp::Eq for DOT11_NIC_SPECIFIC_EXTENSION {}
impl ::core::default::Default for DOT11_NIC_SPECIFIC_EXTENSION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_OFDM_PHY_ATTRIBUTES {
    pub uFrequencyBandsSupported: u32,
}
impl ::core::marker::Copy for DOT11_OFDM_PHY_ATTRIBUTES {}
impl ::core::clone::Clone for DOT11_OFDM_PHY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_OFDM_PHY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OFDM_PHY_ATTRIBUTES").field("uFrequencyBandsSupported", &self.uFrequencyBandsSupported).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_OFDM_PHY_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_OFDM_PHY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.uFrequencyBandsSupported == other.uFrequencyBandsSupported
    }
}
impl ::core::cmp::Eq for DOT11_OFDM_PHY_ATTRIBUTES {}
impl ::core::default::Default for DOT11_OFDM_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_OFFLOAD_CAPABILITY {
    pub uReserved: u32,
    pub uFlags: u32,
    pub uSupportedWEPAlgorithms: u32,
    pub uNumOfReplayWindows: u32,
    pub uMaxWEPKeyMappingLength: u32,
    pub uSupportedAuthAlgorithms: u32,
    pub uMaxAuthKeyMappingLength: u32,
}
impl ::core::marker::Copy for DOT11_OFFLOAD_CAPABILITY {}
impl ::core::clone::Clone for DOT11_OFFLOAD_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_OFFLOAD_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OFFLOAD_CAPABILITY").field("uReserved", &self.uReserved).field("uFlags", &self.uFlags).field("uSupportedWEPAlgorithms", &self.uSupportedWEPAlgorithms).field("uNumOfReplayWindows", &self.uNumOfReplayWindows).field("uMaxWEPKeyMappingLength", &self.uMaxWEPKeyMappingLength).field("uSupportedAuthAlgorithms", &self.uSupportedAuthAlgorithms).field("uMaxAuthKeyMappingLength", &self.uMaxAuthKeyMappingLength).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_OFFLOAD_CAPABILITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_OFFLOAD_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.uFlags == other.uFlags && self.uSupportedWEPAlgorithms == other.uSupportedWEPAlgorithms && self.uNumOfReplayWindows == other.uNumOfReplayWindows && self.uMaxWEPKeyMappingLength == other.uMaxWEPKeyMappingLength && self.uSupportedAuthAlgorithms == other.uSupportedAuthAlgorithms && self.uMaxAuthKeyMappingLength == other.uMaxAuthKeyMappingLength
    }
}
impl ::core::cmp::Eq for DOT11_OFFLOAD_CAPABILITY {}
impl ::core::default::Default for DOT11_OFFLOAD_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_OFFLOAD_NETWORK {
    pub Ssid: DOT11_SSID,
    pub UnicastCipher: DOT11_CIPHER_ALGORITHM,
    pub AuthAlgo: DOT11_AUTH_ALGORITHM,
    pub Dot11ChannelHints: [DOT11_CHANNEL_HINT; 4],
}
impl ::core::marker::Copy for DOT11_OFFLOAD_NETWORK {}
impl ::core::clone::Clone for DOT11_OFFLOAD_NETWORK {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_OFFLOAD_NETWORK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OFFLOAD_NETWORK").field("Ssid", &self.Ssid).field("UnicastCipher", &self.UnicastCipher).field("AuthAlgo", &self.AuthAlgo).field("Dot11ChannelHints", &self.Dot11ChannelHints).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_OFFLOAD_NETWORK {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_OFFLOAD_NETWORK {
    fn eq(&self, other: &Self) -> bool {
        self.Ssid == other.Ssid && self.UnicastCipher == other.UnicastCipher && self.AuthAlgo == other.AuthAlgo && self.Dot11ChannelHints == other.Dot11ChannelHints
    }
}
impl ::core::cmp::Eq for DOT11_OFFLOAD_NETWORK {}
impl ::core::default::Default for DOT11_OFFLOAD_NETWORK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_OFFLOAD_NETWORK_LIST_INFO {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ulFlags: u32,
    pub FastScanPeriod: u32,
    pub FastScanIterations: u32,
    pub SlowScanPeriod: u32,
    pub uNumOfEntries: u32,
    pub offloadNetworkList: [DOT11_OFFLOAD_NETWORK; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_OFFLOAD_NETWORK_LIST_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_OFFLOAD_NETWORK_LIST_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_OFFLOAD_NETWORK_LIST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OFFLOAD_NETWORK_LIST_INFO").field("Header", &self.Header).field("ulFlags", &self.ulFlags).field("FastScanPeriod", &self.FastScanPeriod).field("FastScanIterations", &self.FastScanIterations).field("SlowScanPeriod", &self.SlowScanPeriod).field("uNumOfEntries", &self.uNumOfEntries).field("offloadNetworkList", &self.offloadNetworkList).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_OFFLOAD_NETWORK_LIST_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_OFFLOAD_NETWORK_LIST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ulFlags == other.ulFlags && self.FastScanPeriod == other.FastScanPeriod && self.FastScanIterations == other.FastScanIterations && self.SlowScanPeriod == other.SlowScanPeriod && self.uNumOfEntries == other.uNumOfEntries && self.offloadNetworkList == other.offloadNetworkList
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_OFFLOAD_NETWORK_LIST_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_OFFLOAD_NETWORK_LIST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub Status: i32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS").field("Header", &self.Header).field("Status", &self.Status).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Status == other.Status
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_OI {
    pub OILength: u16,
    pub OI: [u8; 5],
}
impl ::core::marker::Copy for DOT11_OI {}
impl ::core::clone::Clone for DOT11_OI {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_OI {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OI").field("OILength", &self.OILength).field("OI", &self.OI).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_OI {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_OI {
    fn eq(&self, other: &Self) -> bool {
        self.OILength == other.OILength && self.OI == other.OI
    }
}
impl ::core::cmp::Eq for DOT11_OI {}
impl ::core::default::Default for DOT11_OI {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_OPERATION_MODE_CAPABILITY {
    pub uReserved: u32,
    pub uMajorVersion: u32,
    pub uMinorVersion: u32,
    pub uNumOfTXBuffers: u32,
    pub uNumOfRXBuffers: u32,
    pub uOpModeCapability: u32,
}
impl ::core::marker::Copy for DOT11_OPERATION_MODE_CAPABILITY {}
impl ::core::clone::Clone for DOT11_OPERATION_MODE_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_OPERATION_MODE_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OPERATION_MODE_CAPABILITY").field("uReserved", &self.uReserved).field("uMajorVersion", &self.uMajorVersion).field("uMinorVersion", &self.uMinorVersion).field("uNumOfTXBuffers", &self.uNumOfTXBuffers).field("uNumOfRXBuffers", &self.uNumOfRXBuffers).field("uOpModeCapability", &self.uOpModeCapability).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_OPERATION_MODE_CAPABILITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_OPERATION_MODE_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.uMajorVersion == other.uMajorVersion && self.uMinorVersion == other.uMinorVersion && self.uNumOfTXBuffers == other.uNumOfTXBuffers && self.uNumOfRXBuffers == other.uNumOfRXBuffers && self.uOpModeCapability == other.uOpModeCapability
    }
}
impl ::core::cmp::Eq for DOT11_OPERATION_MODE_CAPABILITY {}
impl ::core::default::Default for DOT11_OPERATION_MODE_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_OPTIONAL_CAPABILITY {
    pub uReserved: u32,
    pub bDot11PCF: super::super::Foundation::BOOLEAN,
    pub bDot11PCFMPDUTransferToPC: super::super::Foundation::BOOLEAN,
    pub bStrictlyOrderedServiceClass: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_OPTIONAL_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_OPTIONAL_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_OPTIONAL_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_OPTIONAL_CAPABILITY").field("uReserved", &self.uReserved).field("bDot11PCF", &self.bDot11PCF).field("bDot11PCFMPDUTransferToPC", &self.bDot11PCFMPDUTransferToPC).field("bStrictlyOrderedServiceClass", &self.bStrictlyOrderedServiceClass).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_OPTIONAL_CAPABILITY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_OPTIONAL_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.bDot11PCF == other.bDot11PCF && self.bDot11PCFMPDUTransferToPC == other.bDot11PCFMPDUTransferToPC && self.bStrictlyOrderedServiceClass == other.bStrictlyOrderedServiceClass
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_OPTIONAL_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_OPTIONAL_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_PEER_INFO {
    pub MacAddress: [u8; 6],
    pub usCapabilityInformation: u16,
    pub AuthAlgo: DOT11_AUTH_ALGORITHM,
    pub UnicastCipherAlgo: DOT11_CIPHER_ALGORITHM,
    pub MulticastCipherAlgo: DOT11_CIPHER_ALGORITHM,
    pub bWpsEnabled: super::super::Foundation::BOOLEAN,
    pub usListenInterval: u16,
    pub ucSupportedRates: [u8; 255],
    pub usAssociationID: u16,
    pub AssociationState: DOT11_ASSOCIATION_STATE,
    pub PowerMode: DOT11_POWER_MODE,
    pub liAssociationUpTime: i64,
    pub Statistics: DOT11_PEER_STATISTICS,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_PEER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_PEER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_PEER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PEER_INFO")
            .field("MacAddress", &self.MacAddress)
            .field("usCapabilityInformation", &self.usCapabilityInformation)
            .field("AuthAlgo", &self.AuthAlgo)
            .field("UnicastCipherAlgo", &self.UnicastCipherAlgo)
            .field("MulticastCipherAlgo", &self.MulticastCipherAlgo)
            .field("bWpsEnabled", &self.bWpsEnabled)
            .field("usListenInterval", &self.usListenInterval)
            .field("ucSupportedRates", &self.ucSupportedRates)
            .field("usAssociationID", &self.usAssociationID)
            .field("AssociationState", &self.AssociationState)
            .field("PowerMode", &self.PowerMode)
            .field("liAssociationUpTime", &self.liAssociationUpTime)
            .field("Statistics", &self.Statistics)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_PEER_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_PEER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.MacAddress == other.MacAddress && self.usCapabilityInformation == other.usCapabilityInformation && self.AuthAlgo == other.AuthAlgo && self.UnicastCipherAlgo == other.UnicastCipherAlgo && self.MulticastCipherAlgo == other.MulticastCipherAlgo && self.bWpsEnabled == other.bWpsEnabled && self.usListenInterval == other.usListenInterval && self.ucSupportedRates == other.ucSupportedRates && self.usAssociationID == other.usAssociationID && self.AssociationState == other.AssociationState && self.PowerMode == other.PowerMode && self.liAssociationUpTime == other.liAssociationUpTime && self.Statistics == other.Statistics
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_PEER_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_PEER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_PEER_INFO_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub PeerInfo: [DOT11_PEER_INFO; 1],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_PEER_INFO_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_PEER_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_PEER_INFO_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PEER_INFO_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("PeerInfo", &self.PeerInfo).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_PEER_INFO_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_PEER_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.PeerInfo == other.PeerInfo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_PEER_INFO_LIST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_PEER_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_PEER_STATISTICS {
    pub ullDecryptSuccessCount: u64,
    pub ullDecryptFailureCount: u64,
    pub ullTxPacketSuccessCount: u64,
    pub ullTxPacketFailureCount: u64,
    pub ullRxPacketSuccessCount: u64,
    pub ullRxPacketFailureCount: u64,
}
impl ::core::marker::Copy for DOT11_PEER_STATISTICS {}
impl ::core::clone::Clone for DOT11_PEER_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_PEER_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PEER_STATISTICS").field("ullDecryptSuccessCount", &self.ullDecryptSuccessCount).field("ullDecryptFailureCount", &self.ullDecryptFailureCount).field("ullTxPacketSuccessCount", &self.ullTxPacketSuccessCount).field("ullTxPacketFailureCount", &self.ullTxPacketFailureCount).field("ullRxPacketSuccessCount", &self.ullRxPacketSuccessCount).field("ullRxPacketFailureCount", &self.ullRxPacketFailureCount).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_PEER_STATISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_PEER_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.ullDecryptSuccessCount == other.ullDecryptSuccessCount && self.ullDecryptFailureCount == other.ullDecryptFailureCount && self.ullTxPacketSuccessCount == other.ullTxPacketSuccessCount && self.ullTxPacketFailureCount == other.ullTxPacketFailureCount && self.ullRxPacketSuccessCount == other.ullRxPacketSuccessCount && self.ullRxPacketFailureCount == other.ullRxPacketFailureCount
    }
}
impl ::core::cmp::Eq for DOT11_PEER_STATISTICS {}
impl ::core::default::Default for DOT11_PEER_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_PER_MSDU_COUNTERS {
    pub uTransmittedFragmentCount: u32,
    pub uRetryCount: u32,
    pub uRTSSuccessCount: u32,
    pub uRTSFailureCount: u32,
    pub uACKFailureCount: u32,
}
impl ::core::marker::Copy for DOT11_PER_MSDU_COUNTERS {}
impl ::core::clone::Clone for DOT11_PER_MSDU_COUNTERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_PER_MSDU_COUNTERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PER_MSDU_COUNTERS").field("uTransmittedFragmentCount", &self.uTransmittedFragmentCount).field("uRetryCount", &self.uRetryCount).field("uRTSSuccessCount", &self.uRTSSuccessCount).field("uRTSFailureCount", &self.uRTSFailureCount).field("uACKFailureCount", &self.uACKFailureCount).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_PER_MSDU_COUNTERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_PER_MSDU_COUNTERS {
    fn eq(&self, other: &Self) -> bool {
        self.uTransmittedFragmentCount == other.uTransmittedFragmentCount && self.uRetryCount == other.uRetryCount && self.uRTSSuccessCount == other.uRTSSuccessCount && self.uRTSFailureCount == other.uRTSFailureCount && self.uACKFailureCount == other.uACKFailureCount
    }
}
impl ::core::cmp::Eq for DOT11_PER_MSDU_COUNTERS {}
impl ::core::default::Default for DOT11_PER_MSDU_COUNTERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_PHY_ATTRIBUTES {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PhyType: DOT11_PHY_TYPE,
    pub bHardwarePhyState: super::super::Foundation::BOOLEAN,
    pub bSoftwarePhyState: super::super::Foundation::BOOLEAN,
    pub bCFPollable: super::super::Foundation::BOOLEAN,
    pub uMPDUMaxLength: u32,
    pub TempType: DOT11_TEMP_TYPE,
    pub DiversitySupport: DOT11_DIVERSITY_SUPPORT,
    pub PhySpecificAttributes: DOT11_PHY_ATTRIBUTES_0,
    pub uNumberSupportedPowerLevels: u32,
    pub TxPowerLevels: [u32; 8],
    pub uNumDataRateMappingEntries: u32,
    pub DataRateMappingEntries: [DOT11_DATA_RATE_MAPPING_ENTRY; 126],
    pub SupportedDataRatesValue: DOT11_SUPPORTED_DATA_RATES_VALUE_V2,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_PHY_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_PHY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_PHY_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_PHY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub union DOT11_PHY_ATTRIBUTES_0 {
    pub HRDSSSAttributes: DOT11_HRDSSS_PHY_ATTRIBUTES,
    pub OFDMAttributes: DOT11_OFDM_PHY_ATTRIBUTES,
    pub ERPAttributes: DOT11_ERP_PHY_ATTRIBUTES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_PHY_ATTRIBUTES_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_PHY_ATTRIBUTES_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_PHY_ATTRIBUTES_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_PHY_ATTRIBUTES_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_PHY_FRAME_STATISTICS {
    pub ullTransmittedFrameCount: u64,
    pub ullMulticastTransmittedFrameCount: u64,
    pub ullFailedCount: u64,
    pub ullRetryCount: u64,
    pub ullMultipleRetryCount: u64,
    pub ullMaxTXLifetimeExceededCount: u64,
    pub ullTransmittedFragmentCount: u64,
    pub ullRTSSuccessCount: u64,
    pub ullRTSFailureCount: u64,
    pub ullACKFailureCount: u64,
    pub ullReceivedFrameCount: u64,
    pub ullMulticastReceivedFrameCount: u64,
    pub ullPromiscuousReceivedFrameCount: u64,
    pub ullMaxRXLifetimeExceededCount: u64,
    pub ullFrameDuplicateCount: u64,
    pub ullReceivedFragmentCount: u64,
    pub ullPromiscuousReceivedFragmentCount: u64,
    pub ullFCSErrorCount: u64,
}
impl ::core::marker::Copy for DOT11_PHY_FRAME_STATISTICS {}
impl ::core::clone::Clone for DOT11_PHY_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_PHY_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PHY_FRAME_STATISTICS")
            .field("ullTransmittedFrameCount", &self.ullTransmittedFrameCount)
            .field("ullMulticastTransmittedFrameCount", &self.ullMulticastTransmittedFrameCount)
            .field("ullFailedCount", &self.ullFailedCount)
            .field("ullRetryCount", &self.ullRetryCount)
            .field("ullMultipleRetryCount", &self.ullMultipleRetryCount)
            .field("ullMaxTXLifetimeExceededCount", &self.ullMaxTXLifetimeExceededCount)
            .field("ullTransmittedFragmentCount", &self.ullTransmittedFragmentCount)
            .field("ullRTSSuccessCount", &self.ullRTSSuccessCount)
            .field("ullRTSFailureCount", &self.ullRTSFailureCount)
            .field("ullACKFailureCount", &self.ullACKFailureCount)
            .field("ullReceivedFrameCount", &self.ullReceivedFrameCount)
            .field("ullMulticastReceivedFrameCount", &self.ullMulticastReceivedFrameCount)
            .field("ullPromiscuousReceivedFrameCount", &self.ullPromiscuousReceivedFrameCount)
            .field("ullMaxRXLifetimeExceededCount", &self.ullMaxRXLifetimeExceededCount)
            .field("ullFrameDuplicateCount", &self.ullFrameDuplicateCount)
            .field("ullReceivedFragmentCount", &self.ullReceivedFragmentCount)
            .field("ullPromiscuousReceivedFragmentCount", &self.ullPromiscuousReceivedFragmentCount)
            .field("ullFCSErrorCount", &self.ullFCSErrorCount)
            .finish()
    }
}
impl ::windows::core::TypeKind for DOT11_PHY_FRAME_STATISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_PHY_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.ullTransmittedFrameCount == other.ullTransmittedFrameCount
            && self.ullMulticastTransmittedFrameCount == other.ullMulticastTransmittedFrameCount
            && self.ullFailedCount == other.ullFailedCount
            && self.ullRetryCount == other.ullRetryCount
            && self.ullMultipleRetryCount == other.ullMultipleRetryCount
            && self.ullMaxTXLifetimeExceededCount == other.ullMaxTXLifetimeExceededCount
            && self.ullTransmittedFragmentCount == other.ullTransmittedFragmentCount
            && self.ullRTSSuccessCount == other.ullRTSSuccessCount
            && self.ullRTSFailureCount == other.ullRTSFailureCount
            && self.ullACKFailureCount == other.ullACKFailureCount
            && self.ullReceivedFrameCount == other.ullReceivedFrameCount
            && self.ullMulticastReceivedFrameCount == other.ullMulticastReceivedFrameCount
            && self.ullPromiscuousReceivedFrameCount == other.ullPromiscuousReceivedFrameCount
            && self.ullMaxRXLifetimeExceededCount == other.ullMaxRXLifetimeExceededCount
            && self.ullFrameDuplicateCount == other.ullFrameDuplicateCount
            && self.ullReceivedFragmentCount == other.ullReceivedFragmentCount
            && self.ullPromiscuousReceivedFragmentCount == other.ullPromiscuousReceivedFragmentCount
            && self.ullFCSErrorCount == other.ullFCSErrorCount
    }
}
impl ::core::cmp::Eq for DOT11_PHY_FRAME_STATISTICS {}
impl ::core::default::Default for DOT11_PHY_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ulPhyId: u32,
    pub Anonymous: DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub union DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {
    pub ulChannel: u32,
    pub ulFrequency: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PHY_ID_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11PhyId: [u32; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PHY_ID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PHY_ID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_PHY_ID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PHY_ID_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11PhyId", &self.dot11PhyId).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_PHY_ID_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PHY_ID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11PhyId == other.dot11PhyId
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PHY_ID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PHY_ID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_PHY_STATE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uPhyId: u32,
    pub bHardwarePhyState: super::super::Foundation::BOOLEAN,
    pub bSoftwarePhyState: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_PHY_STATE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_PHY_STATE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_PHY_STATE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PHY_STATE_PARAMETERS").field("Header", &self.Header).field("uPhyId", &self.uPhyId).field("bHardwarePhyState", &self.bHardwarePhyState).field("bSoftwarePhyState", &self.bSoftwarePhyState).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_PHY_STATE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_PHY_STATE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uPhyId == other.uPhyId && self.bHardwarePhyState == other.bHardwarePhyState && self.bSoftwarePhyState == other.bSoftwarePhyState
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_PHY_STATE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_PHY_STATE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_PHY_TYPE_INFO {
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub bUseParameters: super::super::Foundation::BOOLEAN,
    pub uProbeDelay: u32,
    pub uMinChannelTime: u32,
    pub uMaxChannelTime: u32,
    pub ChDescriptionType: CH_DESCRIPTION_TYPE,
    pub uChannelListSize: u32,
    pub ucChannelListBuffer: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_PHY_TYPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_PHY_TYPE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_PHY_TYPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PHY_TYPE_INFO").field("dot11PhyType", &self.dot11PhyType).field("bUseParameters", &self.bUseParameters).field("uProbeDelay", &self.uProbeDelay).field("uMinChannelTime", &self.uMinChannelTime).field("uMaxChannelTime", &self.uMaxChannelTime).field("ChDescriptionType", &self.ChDescriptionType).field("uChannelListSize", &self.uChannelListSize).field("ucChannelListBuffer", &self.ucChannelListBuffer).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_PHY_TYPE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_PHY_TYPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.dot11PhyType == other.dot11PhyType && self.bUseParameters == other.bUseParameters && self.uProbeDelay == other.uProbeDelay && self.uMinChannelTime == other.uMinChannelTime && self.uMaxChannelTime == other.uMaxChannelTime && self.ChDescriptionType == other.ChDescriptionType && self.uChannelListSize == other.uChannelListSize && self.ucChannelListBuffer == other.ucChannelListBuffer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_PHY_TYPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_PHY_TYPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PHY_TYPE_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11PhyType: [DOT11_PHY_TYPE; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PHY_TYPE_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PHY_TYPE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_PHY_TYPE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PHY_TYPE_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11PhyType", &self.dot11PhyType).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_PHY_TYPE_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PHY_TYPE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11PhyType == other.dot11PhyType
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PHY_TYPE_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PHY_TYPE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uCandidateListSize: u32,
    pub uCandidateListOffset: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PMKID_CANDIDATE_LIST_PARAMETERS").field("Header", &self.Header).field("uCandidateListSize", &self.uCandidateListSize).field("uCandidateListOffset", &self.uCandidateListOffset).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uCandidateListSize == other.uCandidateListSize && self.uCandidateListOffset == other.uCandidateListOffset
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PMKID_CANDIDATE_LIST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_PMKID_ENTRY {
    pub BSSID: [u8; 6],
    pub PMKID: [u8; 16],
    pub uFlags: u32,
}
impl ::core::marker::Copy for DOT11_PMKID_ENTRY {}
impl ::core::clone::Clone for DOT11_PMKID_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_PMKID_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PMKID_ENTRY").field("BSSID", &self.BSSID).field("PMKID", &self.PMKID).field("uFlags", &self.uFlags).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_PMKID_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_PMKID_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.BSSID == other.BSSID && self.PMKID == other.PMKID && self.uFlags == other.uFlags
    }
}
impl ::core::cmp::Eq for DOT11_PMKID_ENTRY {}
impl ::core::default::Default for DOT11_PMKID_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PMKID_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub PMKIDs: [DOT11_PMKID_ENTRY; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PMKID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PMKID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_PMKID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PMKID_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("PMKIDs", &self.PMKIDs).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_PMKID_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PMKID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.PMKIDs == other.PMKIDs
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PMKID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PMKID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_PORT_STATE {
    pub PeerMacAddress: [u8; 6],
    pub uSessionId: u32,
    pub bPortControlled: super::super::Foundation::BOOL,
    pub bPortAuthorized: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_PORT_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_PORT_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_PORT_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PORT_STATE").field("PeerMacAddress", &self.PeerMacAddress).field("uSessionId", &self.uSessionId).field("bPortControlled", &self.bPortControlled).field("bPortAuthorized", &self.bPortAuthorized).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_PORT_STATE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_PORT_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.PeerMacAddress == other.PeerMacAddress && self.uSessionId == other.uSessionId && self.bPortControlled == other.bPortControlled && self.bPortAuthorized == other.bPortAuthorized
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_PORT_STATE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_PORT_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_PORT_STATE_NOTIFICATION {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerMac: [u8; 6],
    pub bOpen: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_PORT_STATE_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_PORT_STATE_NOTIFICATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_PORT_STATE_NOTIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PORT_STATE_NOTIFICATION").field("Header", &self.Header).field("PeerMac", &self.PeerMac).field("bOpen", &self.bOpen).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_PORT_STATE_NOTIFICATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_PORT_STATE_NOTIFICATION {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerMac == other.PeerMac && self.bOpen == other.bOpen
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_PORT_STATE_NOTIFICATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_PORT_STATE_NOTIFICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub bEnabled: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO").field("Header", &self.Header).field("bEnabled", &self.bEnabled).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.bEnabled == other.bEnabled
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_POWER_MGMT_MODE {
    pub dot11PowerMode: DOT11_POWER_MODE,
    pub uPowerSaveLevel: u32,
    pub usListenInterval: u16,
    pub usAID: u16,
    pub bReceiveDTIMs: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_POWER_MGMT_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_POWER_MGMT_MODE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_POWER_MGMT_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_POWER_MGMT_MODE").field("dot11PowerMode", &self.dot11PowerMode).field("uPowerSaveLevel", &self.uPowerSaveLevel).field("usListenInterval", &self.usListenInterval).field("usAID", &self.usAID).field("bReceiveDTIMs", &self.bReceiveDTIMs).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_POWER_MGMT_MODE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_POWER_MGMT_MODE {
    fn eq(&self, other: &Self) -> bool {
        self.dot11PowerMode == other.dot11PowerMode && self.uPowerSaveLevel == other.uPowerSaveLevel && self.usListenInterval == other.usListenInterval && self.usAID == other.usAID && self.bReceiveDTIMs == other.bReceiveDTIMs
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_POWER_MGMT_MODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_POWER_MGMT_MODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_POWER_MGMT_MODE_STATUS_INFO {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PowerSaveMode: DOT11_POWER_MODE,
    pub uPowerSaveLevel: u32,
    pub Reason: DOT11_POWER_MODE_REASON,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_POWER_MGMT_MODE_STATUS_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_POWER_MGMT_MODE_STATUS_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_POWER_MGMT_MODE_STATUS_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_POWER_MGMT_MODE_STATUS_INFO").field("Header", &self.Header).field("PowerSaveMode", &self.PowerSaveMode).field("uPowerSaveLevel", &self.uPowerSaveLevel).field("Reason", &self.Reason).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_POWER_MGMT_MODE_STATUS_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_POWER_MGMT_MODE_STATUS_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PowerSaveMode == other.PowerSaveMode && self.uPowerSaveLevel == other.uPowerSaveLevel && self.Reason == other.Reason
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_POWER_MGMT_MODE_STATUS_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_POWER_MGMT_MODE_STATUS_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_PRIVACY_EXEMPTION {
    pub usEtherType: u16,
    pub usExemptionActionType: u16,
    pub usExemptionPacketType: u16,
}
impl ::core::marker::Copy for DOT11_PRIVACY_EXEMPTION {}
impl ::core::clone::Clone for DOT11_PRIVACY_EXEMPTION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_PRIVACY_EXEMPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PRIVACY_EXEMPTION").field("usEtherType", &self.usEtherType).field("usExemptionActionType", &self.usExemptionActionType).field("usExemptionPacketType", &self.usExemptionPacketType).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_PRIVACY_EXEMPTION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_PRIVACY_EXEMPTION {
    fn eq(&self, other: &Self) -> bool {
        self.usEtherType == other.usEtherType && self.usExemptionActionType == other.usExemptionActionType && self.usExemptionPacketType == other.usExemptionPacketType
    }
}
impl ::core::cmp::Eq for DOT11_PRIVACY_EXEMPTION {}
impl ::core::default::Default for DOT11_PRIVACY_EXEMPTION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PRIVACY_EXEMPTION_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub PrivacyExemptionEntries: [DOT11_PRIVACY_EXEMPTION; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PRIVACY_EXEMPTION_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PRIVACY_EXEMPTION_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_PRIVACY_EXEMPTION_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PRIVACY_EXEMPTION_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("PrivacyExemptionEntries", &self.PrivacyExemptionEntries).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_PRIVACY_EXEMPTION_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PRIVACY_EXEMPTION_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.PrivacyExemptionEntries == other.PrivacyExemptionEntries
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PRIVACY_EXEMPTION_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PRIVACY_EXEMPTION_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub ReceiverAddress: [u8; 6],
    pub DialogToken: u8,
    pub Status: i32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("ReceiverAddress", &self.ReceiverAddress).field("DialogToken", &self.DialogToken).field("Status", &self.Status).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.ReceiverAddress == other.ReceiverAddress && self.DialogToken == other.DialogToken && self.Status == other.Status && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub Status: i32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS").field("Header", &self.Header).field("ReceiverDeviceAddress", &self.ReceiverDeviceAddress).field("DialogToken", &self.DialogToken).field("Status", &self.Status).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ReceiverDeviceAddress == other.ReceiverDeviceAddress && self.DialogToken == other.DialogToken && self.Status == other.Status && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_QOS_PARAMS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ucEnabledQoSProtocolFlags: u8,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_QOS_PARAMS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_QOS_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_QOS_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_QOS_PARAMS").field("Header", &self.Header).field("ucEnabledQoSProtocolFlags", &self.ucEnabledQoSProtocolFlags).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_QOS_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_QOS_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ucEnabledQoSProtocolFlags == other.ucEnabledQoSProtocolFlags
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_QOS_PARAMS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_QOS_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_QOS_TX_DURATION {
    pub uNominalMSDUSize: u32,
    pub uMinPHYRate: u32,
    pub uDuration: u32,
}
impl ::core::marker::Copy for DOT11_QOS_TX_DURATION {}
impl ::core::clone::Clone for DOT11_QOS_TX_DURATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_QOS_TX_DURATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_QOS_TX_DURATION").field("uNominalMSDUSize", &self.uNominalMSDUSize).field("uMinPHYRate", &self.uMinPHYRate).field("uDuration", &self.uDuration).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_QOS_TX_DURATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_QOS_TX_DURATION {
    fn eq(&self, other: &Self) -> bool {
        self.uNominalMSDUSize == other.uNominalMSDUSize && self.uMinPHYRate == other.uMinPHYRate && self.uDuration == other.uDuration
    }
}
impl ::core::cmp::Eq for DOT11_QOS_TX_DURATION {}
impl ::core::default::Default for DOT11_QOS_TX_DURATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_QOS_TX_MEDIUM_TIME {
    pub dot11PeerAddress: [u8; 6],
    pub ucQoSPriority: u8,
    pub uMediumTimeAdmited: u32,
}
impl ::core::marker::Copy for DOT11_QOS_TX_MEDIUM_TIME {}
impl ::core::clone::Clone for DOT11_QOS_TX_MEDIUM_TIME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_QOS_TX_MEDIUM_TIME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_QOS_TX_MEDIUM_TIME").field("dot11PeerAddress", &self.dot11PeerAddress).field("ucQoSPriority", &self.ucQoSPriority).field("uMediumTimeAdmited", &self.uMediumTimeAdmited).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_QOS_TX_MEDIUM_TIME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_QOS_TX_MEDIUM_TIME {
    fn eq(&self, other: &Self) -> bool {
        self.dot11PeerAddress == other.dot11PeerAddress && self.ucQoSPriority == other.ucQoSPriority && self.uMediumTimeAdmited == other.uMediumTimeAdmited
    }
}
impl ::core::cmp::Eq for DOT11_QOS_TX_MEDIUM_TIME {}
impl ::core::default::Default for DOT11_QOS_TX_MEDIUM_TIME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_RATE_SET {
    pub uRateSetLength: u32,
    pub ucRateSet: [u8; 126],
}
impl ::core::marker::Copy for DOT11_RATE_SET {}
impl ::core::clone::Clone for DOT11_RATE_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_RATE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RATE_SET").field("uRateSetLength", &self.uRateSetLength).field("ucRateSet", &self.ucRateSet).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_RATE_SET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_RATE_SET {
    fn eq(&self, other: &Self) -> bool {
        self.uRateSetLength == other.uRateSetLength && self.ucRateSet == other.ucRateSet
    }
}
impl ::core::cmp::Eq for DOT11_RATE_SET {}
impl ::core::default::Default for DOT11_RATE_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("DialogToken", &self.DialogToken).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub RequestContext: *mut ::core::ffi::c_void,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("DialogToken", &self.DialogToken).field("RequestContext", &self.RequestContext).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.RequestContext == other.RequestContext && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub ResponseContext: *mut ::core::ffi::c_void,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS").field("Header", &self.Header).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("DialogToken", &self.DialogToken).field("ResponseContext", &self.ResponseContext).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.ResponseContext == other.ResponseContext && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: [u8; 6],
    pub BSSID: [u8; 6],
    pub DialogToken: u8,
    pub RequestContext: *mut ::core::ffi::c_void,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS").field("Header", &self.Header).field("TransmitterDeviceAddress", &self.TransmitterDeviceAddress).field("BSSID", &self.BSSID).field("DialogToken", &self.DialogToken).field("RequestContext", &self.RequestContext).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.TransmitterDeviceAddress == other.TransmitterDeviceAddress && self.BSSID == other.BSSID && self.DialogToken == other.DialogToken && self.RequestContext == other.RequestContext && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: [u8; 6],
    pub BSSID: [u8; 6],
    pub DialogToken: u8,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS").field("Header", &self.Header).field("TransmitterDeviceAddress", &self.TransmitterDeviceAddress).field("BSSID", &self.BSSID).field("DialogToken", &self.DialogToken).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.TransmitterDeviceAddress == other.TransmitterDeviceAddress && self.BSSID == other.BSSID && self.DialogToken == other.DialogToken && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: [u8; 6],
    pub BSSID: [u8; 6],
    pub DialogToken: u8,
    pub RequestContext: *mut ::core::ffi::c_void,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS").field("Header", &self.Header).field("TransmitterDeviceAddress", &self.TransmitterDeviceAddress).field("BSSID", &self.BSSID).field("DialogToken", &self.DialogToken).field("RequestContext", &self.RequestContext).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.TransmitterDeviceAddress == other.TransmitterDeviceAddress && self.BSSID == other.BSSID && self.DialogToken == other.DialogToken && self.RequestContext == other.RequestContext && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub TransmitterDeviceAddress: [u8; 6],
    pub BSSID: [u8; 6],
    pub DialogToken: u8,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS").field("Header", &self.Header).field("TransmitterDeviceAddress", &self.TransmitterDeviceAddress).field("BSSID", &self.BSSID).field("DialogToken", &self.DialogToken).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.TransmitterDeviceAddress == other.TransmitterDeviceAddress && self.BSSID == other.BSSID && self.DialogToken == other.DialogToken && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_RECV_EXTENSION_INFO {
    pub uVersion: u32,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uChCenterFrequency: u32,
    pub lRSSI: i32,
    pub lRSSIMin: i32,
    pub lRSSIMax: i32,
    pub uRSSI: u32,
    pub ucPriority: u8,
    pub ucDataRate: u8,
    pub ucPeerMacAddress: [u8; 6],
    pub dwExtendedStatus: u32,
    pub hWEPOffloadContext: super::super::Foundation::HANDLE,
    pub hAuthOffloadContext: super::super::Foundation::HANDLE,
    pub usWEPAppliedMask: u16,
    pub usWPAMSDUPriority: u16,
    pub dot11LowestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11LeftRWBitMap: u16,
    pub dot11HighestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11RightRWBitMap: u16,
    pub usNumberOfMPDUsReceived: u16,
    pub usNumberOfFragments: u16,
    pub pNdisPackets: [*mut ::core::ffi::c_void; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_RECV_EXTENSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_RECV_EXTENSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_RECV_EXTENSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECV_EXTENSION_INFO")
            .field("uVersion", &self.uVersion)
            .field("pvReserved", &self.pvReserved)
            .field("dot11PhyType", &self.dot11PhyType)
            .field("uChCenterFrequency", &self.uChCenterFrequency)
            .field("lRSSI", &self.lRSSI)
            .field("lRSSIMin", &self.lRSSIMin)
            .field("lRSSIMax", &self.lRSSIMax)
            .field("uRSSI", &self.uRSSI)
            .field("ucPriority", &self.ucPriority)
            .field("ucDataRate", &self.ucDataRate)
            .field("ucPeerMacAddress", &self.ucPeerMacAddress)
            .field("dwExtendedStatus", &self.dwExtendedStatus)
            .field("hWEPOffloadContext", &self.hWEPOffloadContext)
            .field("hAuthOffloadContext", &self.hAuthOffloadContext)
            .field("usWEPAppliedMask", &self.usWEPAppliedMask)
            .field("usWPAMSDUPriority", &self.usWPAMSDUPriority)
            .field("dot11LowestIV48Counter", &self.dot11LowestIV48Counter)
            .field("usDot11LeftRWBitMap", &self.usDot11LeftRWBitMap)
            .field("dot11HighestIV48Counter", &self.dot11HighestIV48Counter)
            .field("usDot11RightRWBitMap", &self.usDot11RightRWBitMap)
            .field("usNumberOfMPDUsReceived", &self.usNumberOfMPDUsReceived)
            .field("usNumberOfFragments", &self.usNumberOfFragments)
            .field("pNdisPackets", &self.pNdisPackets)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_RECV_EXTENSION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_RECV_EXTENSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.uVersion == other.uVersion
            && self.pvReserved == other.pvReserved
            && self.dot11PhyType == other.dot11PhyType
            && self.uChCenterFrequency == other.uChCenterFrequency
            && self.lRSSI == other.lRSSI
            && self.lRSSIMin == other.lRSSIMin
            && self.lRSSIMax == other.lRSSIMax
            && self.uRSSI == other.uRSSI
            && self.ucPriority == other.ucPriority
            && self.ucDataRate == other.ucDataRate
            && self.ucPeerMacAddress == other.ucPeerMacAddress
            && self.dwExtendedStatus == other.dwExtendedStatus
            && self.hWEPOffloadContext == other.hWEPOffloadContext
            && self.hAuthOffloadContext == other.hAuthOffloadContext
            && self.usWEPAppliedMask == other.usWEPAppliedMask
            && self.usWPAMSDUPriority == other.usWPAMSDUPriority
            && self.dot11LowestIV48Counter == other.dot11LowestIV48Counter
            && self.usDot11LeftRWBitMap == other.usDot11LeftRWBitMap
            && self.dot11HighestIV48Counter == other.dot11HighestIV48Counter
            && self.usDot11RightRWBitMap == other.usDot11RightRWBitMap
            && self.usNumberOfMPDUsReceived == other.usNumberOfMPDUsReceived
            && self.usNumberOfFragments == other.usNumberOfFragments
            && self.pNdisPackets == other.pNdisPackets
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_RECV_EXTENSION_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_RECV_EXTENSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_RECV_EXTENSION_INFO_V2 {
    pub uVersion: u32,
    pub pvReserved: *mut ::core::ffi::c_void,
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uChCenterFrequency: u32,
    pub lRSSI: i32,
    pub uRSSI: u32,
    pub ucPriority: u8,
    pub ucDataRate: u8,
    pub ucPeerMacAddress: [u8; 6],
    pub dwExtendedStatus: u32,
    pub hWEPOffloadContext: super::super::Foundation::HANDLE,
    pub hAuthOffloadContext: super::super::Foundation::HANDLE,
    pub usWEPAppliedMask: u16,
    pub usWPAMSDUPriority: u16,
    pub dot11LowestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11LeftRWBitMap: u16,
    pub dot11HighestIV48Counter: DOT11_IV48_COUNTER,
    pub usDot11RightRWBitMap: u16,
    pub usNumberOfMPDUsReceived: u16,
    pub usNumberOfFragments: u16,
    pub pNdisPackets: [*mut ::core::ffi::c_void; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_RECV_EXTENSION_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_RECV_EXTENSION_INFO_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_RECV_EXTENSION_INFO_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECV_EXTENSION_INFO_V2")
            .field("uVersion", &self.uVersion)
            .field("pvReserved", &self.pvReserved)
            .field("dot11PhyType", &self.dot11PhyType)
            .field("uChCenterFrequency", &self.uChCenterFrequency)
            .field("lRSSI", &self.lRSSI)
            .field("uRSSI", &self.uRSSI)
            .field("ucPriority", &self.ucPriority)
            .field("ucDataRate", &self.ucDataRate)
            .field("ucPeerMacAddress", &self.ucPeerMacAddress)
            .field("dwExtendedStatus", &self.dwExtendedStatus)
            .field("hWEPOffloadContext", &self.hWEPOffloadContext)
            .field("hAuthOffloadContext", &self.hAuthOffloadContext)
            .field("usWEPAppliedMask", &self.usWEPAppliedMask)
            .field("usWPAMSDUPriority", &self.usWPAMSDUPriority)
            .field("dot11LowestIV48Counter", &self.dot11LowestIV48Counter)
            .field("usDot11LeftRWBitMap", &self.usDot11LeftRWBitMap)
            .field("dot11HighestIV48Counter", &self.dot11HighestIV48Counter)
            .field("usDot11RightRWBitMap", &self.usDot11RightRWBitMap)
            .field("usNumberOfMPDUsReceived", &self.usNumberOfMPDUsReceived)
            .field("usNumberOfFragments", &self.usNumberOfFragments)
            .field("pNdisPackets", &self.pNdisPackets)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_RECV_EXTENSION_INFO_V2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_RECV_EXTENSION_INFO_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.uVersion == other.uVersion
            && self.pvReserved == other.pvReserved
            && self.dot11PhyType == other.dot11PhyType
            && self.uChCenterFrequency == other.uChCenterFrequency
            && self.lRSSI == other.lRSSI
            && self.uRSSI == other.uRSSI
            && self.ucPriority == other.ucPriority
            && self.ucDataRate == other.ucDataRate
            && self.ucPeerMacAddress == other.ucPeerMacAddress
            && self.dwExtendedStatus == other.dwExtendedStatus
            && self.hWEPOffloadContext == other.hWEPOffloadContext
            && self.hAuthOffloadContext == other.hAuthOffloadContext
            && self.usWEPAppliedMask == other.usWEPAppliedMask
            && self.usWPAMSDUPriority == other.usWPAMSDUPriority
            && self.dot11LowestIV48Counter == other.dot11LowestIV48Counter
            && self.usDot11LeftRWBitMap == other.usDot11LeftRWBitMap
            && self.dot11HighestIV48Counter == other.dot11HighestIV48Counter
            && self.usDot11RightRWBitMap == other.usDot11RightRWBitMap
            && self.usNumberOfMPDUsReceived == other.usNumberOfMPDUsReceived
            && self.usNumberOfFragments == other.usNumberOfFragments
            && self.pNdisPackets == other.pNdisPackets
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_RECV_EXTENSION_INFO_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_RECV_EXTENSION_INFO_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_RECV_SENSITIVITY {
    pub ucDataRate: u8,
    pub lRSSIMin: i32,
    pub lRSSIMax: i32,
}
impl ::core::marker::Copy for DOT11_RECV_SENSITIVITY {}
impl ::core::clone::Clone for DOT11_RECV_SENSITIVITY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_RECV_SENSITIVITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RECV_SENSITIVITY").field("ucDataRate", &self.ucDataRate).field("lRSSIMin", &self.lRSSIMin).field("lRSSIMax", &self.lRSSIMax).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_RECV_SENSITIVITY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_RECV_SENSITIVITY {
    fn eq(&self, other: &Self) -> bool {
        self.ucDataRate == other.ucDataRate && self.lRSSIMin == other.lRSSIMin && self.lRSSIMax == other.lRSSIMax
    }
}
impl ::core::cmp::Eq for DOT11_RECV_SENSITIVITY {}
impl ::core::default::Default for DOT11_RECV_SENSITIVITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_RECV_SENSITIVITY_LIST {
    pub Anonymous: DOT11_RECV_SENSITIVITY_LIST_0,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11RecvSensitivity: [DOT11_RECV_SENSITIVITY; 1],
}
impl ::core::marker::Copy for DOT11_RECV_SENSITIVITY_LIST {}
impl ::core::clone::Clone for DOT11_RECV_SENSITIVITY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DOT11_RECV_SENSITIVITY_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DOT11_RECV_SENSITIVITY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub union DOT11_RECV_SENSITIVITY_LIST_0 {
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uPhyId: u32,
}
impl ::core::marker::Copy for DOT11_RECV_SENSITIVITY_LIST_0 {}
impl ::core::clone::Clone for DOT11_RECV_SENSITIVITY_LIST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DOT11_RECV_SENSITIVITY_LIST_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DOT11_RECV_SENSITIVITY_LIST_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_REG_DOMAINS_SUPPORT_VALUE {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11RegDomainValue: [DOT11_REG_DOMAIN_VALUE; 1],
}
impl ::core::marker::Copy for DOT11_REG_DOMAINS_SUPPORT_VALUE {}
impl ::core::clone::Clone for DOT11_REG_DOMAINS_SUPPORT_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_REG_DOMAINS_SUPPORT_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_REG_DOMAINS_SUPPORT_VALUE").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11RegDomainValue", &self.dot11RegDomainValue).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_REG_DOMAINS_SUPPORT_VALUE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_REG_DOMAINS_SUPPORT_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11RegDomainValue == other.dot11RegDomainValue
    }
}
impl ::core::cmp::Eq for DOT11_REG_DOMAINS_SUPPORT_VALUE {}
impl ::core::default::Default for DOT11_REG_DOMAINS_SUPPORT_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_REG_DOMAIN_VALUE {
    pub uRegDomainsSupportIndex: u32,
    pub uRegDomainsSupportValue: u32,
}
impl ::core::marker::Copy for DOT11_REG_DOMAIN_VALUE {}
impl ::core::clone::Clone for DOT11_REG_DOMAIN_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_REG_DOMAIN_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_REG_DOMAIN_VALUE").field("uRegDomainsSupportIndex", &self.uRegDomainsSupportIndex).field("uRegDomainsSupportValue", &self.uRegDomainsSupportValue).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_REG_DOMAIN_VALUE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_REG_DOMAIN_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.uRegDomainsSupportIndex == other.uRegDomainsSupportIndex && self.uRegDomainsSupportValue == other.uRegDomainsSupportValue
    }
}
impl ::core::cmp::Eq for DOT11_REG_DOMAIN_VALUE {}
impl ::core::default::Default for DOT11_REG_DOMAIN_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_RESET_REQUEST {
    pub dot11ResetType: DOT11_RESET_TYPE,
    pub dot11MacAddress: [u8; 6],
    pub bSetDefaultMIB: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_RESET_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_RESET_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_RESET_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RESET_REQUEST").field("dot11ResetType", &self.dot11ResetType).field("dot11MacAddress", &self.dot11MacAddress).field("bSetDefaultMIB", &self.bSetDefaultMIB).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_RESET_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_RESET_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.dot11ResetType == other.dot11ResetType && self.dot11MacAddress == other.dot11MacAddress && self.bSetDefaultMIB == other.bSetDefaultMIB
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_RESET_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_RESET_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_ROAMING_COMPLETION_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uStatus: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_ROAMING_COMPLETION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_ROAMING_COMPLETION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_ROAMING_COMPLETION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ROAMING_COMPLETION_PARAMETERS").field("Header", &self.Header).field("uStatus", &self.uStatus).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_ROAMING_COMPLETION_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ROAMING_COMPLETION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uStatus == other.uStatus
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ROAMING_COMPLETION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ROAMING_COMPLETION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_ROAMING_START_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub AdhocBSSID: [u8; 6],
    pub AdhocSSID: DOT11_SSID,
    pub uRoamingReason: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_ROAMING_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_ROAMING_START_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_ROAMING_START_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_ROAMING_START_PARAMETERS").field("Header", &self.Header).field("AdhocBSSID", &self.AdhocBSSID).field("AdhocSSID", &self.AdhocSSID).field("uRoamingReason", &self.uRoamingReason).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_ROAMING_START_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_ROAMING_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.AdhocBSSID == other.AdhocBSSID && self.AdhocSSID == other.AdhocSSID && self.uRoamingReason == other.uRoamingReason
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_ROAMING_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_ROAMING_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_RSSI_RANGE {
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uRSSIMin: u32,
    pub uRSSIMax: u32,
}
impl ::core::marker::Copy for DOT11_RSSI_RANGE {}
impl ::core::clone::Clone for DOT11_RSSI_RANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_RSSI_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_RSSI_RANGE").field("dot11PhyType", &self.dot11PhyType).field("uRSSIMin", &self.uRSSIMin).field("uRSSIMax", &self.uRSSIMax).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_RSSI_RANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_RSSI_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.dot11PhyType == other.dot11PhyType && self.uRSSIMin == other.uRSSIMin && self.uRSSIMax == other.uRSSIMax
    }
}
impl ::core::cmp::Eq for DOT11_RSSI_RANGE {}
impl ::core::default::Default for DOT11_RSSI_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_SCAN_REQUEST {
    pub dot11BSSType: DOT11_BSS_TYPE,
    pub dot11BSSID: [u8; 6],
    pub dot11SSID: DOT11_SSID,
    pub dot11ScanType: DOT11_SCAN_TYPE,
    pub bRestrictedScan: super::super::Foundation::BOOLEAN,
    pub bUseRequestIE: super::super::Foundation::BOOLEAN,
    pub uRequestIDsOffset: u32,
    pub uNumOfRequestIDs: u32,
    pub uPhyTypesOffset: u32,
    pub uNumOfPhyTypes: u32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
    pub ucBuffer: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_SCAN_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_SCAN_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_SCAN_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SCAN_REQUEST")
            .field("dot11BSSType", &self.dot11BSSType)
            .field("dot11BSSID", &self.dot11BSSID)
            .field("dot11SSID", &self.dot11SSID)
            .field("dot11ScanType", &self.dot11ScanType)
            .field("bRestrictedScan", &self.bRestrictedScan)
            .field("bUseRequestIE", &self.bUseRequestIE)
            .field("uRequestIDsOffset", &self.uRequestIDsOffset)
            .field("uNumOfRequestIDs", &self.uNumOfRequestIDs)
            .field("uPhyTypesOffset", &self.uPhyTypesOffset)
            .field("uNumOfPhyTypes", &self.uNumOfPhyTypes)
            .field("uIEsOffset", &self.uIEsOffset)
            .field("uIEsLength", &self.uIEsLength)
            .field("ucBuffer", &self.ucBuffer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_SCAN_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_SCAN_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.dot11BSSType == other.dot11BSSType && self.dot11BSSID == other.dot11BSSID && self.dot11SSID == other.dot11SSID && self.dot11ScanType == other.dot11ScanType && self.bRestrictedScan == other.bRestrictedScan && self.bUseRequestIE == other.bUseRequestIE && self.uRequestIDsOffset == other.uRequestIDsOffset && self.uNumOfRequestIDs == other.uNumOfRequestIDs && self.uPhyTypesOffset == other.uPhyTypesOffset && self.uNumOfPhyTypes == other.uNumOfPhyTypes && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength && self.ucBuffer == other.ucBuffer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_SCAN_REQUEST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_SCAN_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_SCAN_REQUEST_V2 {
    pub dot11BSSType: DOT11_BSS_TYPE,
    pub dot11BSSID: [u8; 6],
    pub dot11ScanType: DOT11_SCAN_TYPE,
    pub bRestrictedScan: super::super::Foundation::BOOLEAN,
    pub udot11SSIDsOffset: u32,
    pub uNumOfdot11SSIDs: u32,
    pub bUseRequestIE: super::super::Foundation::BOOLEAN,
    pub uRequestIDsOffset: u32,
    pub uNumOfRequestIDs: u32,
    pub uPhyTypeInfosOffset: u32,
    pub uNumOfPhyTypeInfos: u32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
    pub ucBuffer: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_SCAN_REQUEST_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_SCAN_REQUEST_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_SCAN_REQUEST_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SCAN_REQUEST_V2")
            .field("dot11BSSType", &self.dot11BSSType)
            .field("dot11BSSID", &self.dot11BSSID)
            .field("dot11ScanType", &self.dot11ScanType)
            .field("bRestrictedScan", &self.bRestrictedScan)
            .field("udot11SSIDsOffset", &self.udot11SSIDsOffset)
            .field("uNumOfdot11SSIDs", &self.uNumOfdot11SSIDs)
            .field("bUseRequestIE", &self.bUseRequestIE)
            .field("uRequestIDsOffset", &self.uRequestIDsOffset)
            .field("uNumOfRequestIDs", &self.uNumOfRequestIDs)
            .field("uPhyTypeInfosOffset", &self.uPhyTypeInfosOffset)
            .field("uNumOfPhyTypeInfos", &self.uNumOfPhyTypeInfos)
            .field("uIEsOffset", &self.uIEsOffset)
            .field("uIEsLength", &self.uIEsLength)
            .field("ucBuffer", &self.ucBuffer)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_SCAN_REQUEST_V2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_SCAN_REQUEST_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.dot11BSSType == other.dot11BSSType && self.dot11BSSID == other.dot11BSSID && self.dot11ScanType == other.dot11ScanType && self.bRestrictedScan == other.bRestrictedScan && self.udot11SSIDsOffset == other.udot11SSIDsOffset && self.uNumOfdot11SSIDs == other.uNumOfdot11SSIDs && self.bUseRequestIE == other.bUseRequestIE && self.uRequestIDsOffset == other.uRequestIDsOffset && self.uNumOfRequestIDs == other.uNumOfRequestIDs && self.uPhyTypeInfosOffset == other.uPhyTypeInfosOffset && self.uNumOfPhyTypeInfos == other.uNumOfPhyTypeInfos && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength && self.ucBuffer == other.ucBuffer
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_SCAN_REQUEST_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_SCAN_REQUEST_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C, packed(1))]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_SECURITY_PACKET_HEADER {
    pub PeerMac: [u8; 6],
    pub usEtherType: u16,
    pub Data: [u8; 1],
}
impl ::core::marker::Copy for DOT11_SECURITY_PACKET_HEADER {}
impl ::core::clone::Clone for DOT11_SECURITY_PACKET_HEADER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DOT11_SECURITY_PACKET_HEADER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DOT11_SECURITY_PACKET_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub ResponseContext: *mut ::core::ffi::c_void,
    pub uSendTimeout: u32,
    pub Status: u8,
    pub GroupCapability: u8,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bUseGroupID: super::super::Foundation::BOOLEAN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS")
            .field("Header", &self.Header)
            .field("PeerDeviceAddress", &self.PeerDeviceAddress)
            .field("DialogToken", &self.DialogToken)
            .field("ResponseContext", &self.ResponseContext)
            .field("uSendTimeout", &self.uSendTimeout)
            .field("Status", &self.Status)
            .field("GroupCapability", &self.GroupCapability)
            .field("GroupID", &self.GroupID)
            .field("bUseGroupID", &self.bUseGroupID)
            .field("uIEsOffset", &self.uIEsOffset)
            .field("uIEsLength", &self.uIEsLength)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.ResponseContext == other.ResponseContext && self.uSendTimeout == other.uSendTimeout && self.Status == other.Status && self.GroupCapability == other.GroupCapability && self.GroupID == other.GroupID && self.bUseGroupID == other.bUseGroupID && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub uSendTimeout: u32,
    pub GroupOwnerIntent: DOT11_WFD_GO_INTENT,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub IntendedInterfaceAddress: [u8; 6],
    pub GroupCapability: u8,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS")
            .field("Header", &self.Header)
            .field("PeerDeviceAddress", &self.PeerDeviceAddress)
            .field("DialogToken", &self.DialogToken)
            .field("uSendTimeout", &self.uSendTimeout)
            .field("GroupOwnerIntent", &self.GroupOwnerIntent)
            .field("MinimumConfigTimeout", &self.MinimumConfigTimeout)
            .field("IntendedInterfaceAddress", &self.IntendedInterfaceAddress)
            .field("GroupCapability", &self.GroupCapability)
            .field("uIEsOffset", &self.uIEsOffset)
            .field("uIEsLength", &self.uIEsLength)
            .finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.uSendTimeout == other.uSendTimeout && self.GroupOwnerIntent == other.GroupOwnerIntent && self.MinimumConfigTimeout == other.MinimumConfigTimeout && self.IntendedInterfaceAddress == other.IntendedInterfaceAddress && self.GroupCapability == other.GroupCapability && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub PeerDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub RequestContext: *mut ::core::ffi::c_void,
    pub uSendTimeout: u32,
    pub Status: u8,
    pub GroupOwnerIntent: DOT11_WFD_GO_INTENT,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub IntendedInterfaceAddress: [u8; 6],
    pub GroupCapability: u8,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bUseGroupID: super::super::Foundation::BOOLEAN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS")
            .field("Header", &self.Header)
            .field("PeerDeviceAddress", &self.PeerDeviceAddress)
            .field("DialogToken", &self.DialogToken)
            .field("RequestContext", &self.RequestContext)
            .field("uSendTimeout", &self.uSendTimeout)
            .field("Status", &self.Status)
            .field("GroupOwnerIntent", &self.GroupOwnerIntent)
            .field("MinimumConfigTimeout", &self.MinimumConfigTimeout)
            .field("IntendedInterfaceAddress", &self.IntendedInterfaceAddress)
            .field("GroupCapability", &self.GroupCapability)
            .field("GroupID", &self.GroupID)
            .field("bUseGroupID", &self.bUseGroupID)
            .field("uIEsOffset", &self.uIEsOffset)
            .field("uIEsLength", &self.uIEsLength)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.PeerDeviceAddress == other.PeerDeviceAddress && self.DialogToken == other.DialogToken && self.RequestContext == other.RequestContext && self.uSendTimeout == other.uSendTimeout && self.Status == other.Status && self.GroupOwnerIntent == other.GroupOwnerIntent && self.MinimumConfigTimeout == other.MinimumConfigTimeout && self.IntendedInterfaceAddress == other.IntendedInterfaceAddress && self.GroupCapability == other.GroupCapability && self.GroupID == other.GroupID && self.bUseGroupID == other.bUseGroupID && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub DialogToken: u8,
    pub PeerDeviceAddress: [u8; 6],
    pub uSendTimeout: u32,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub InvitationFlags: DOT11_WFD_INVITATION_FLAGS,
    pub GroupBSSID: [u8; 6],
    pub bUseGroupBSSID: super::super::Foundation::BOOLEAN,
    pub OperatingChannel: DOT11_WFD_CHANNEL,
    pub bUseSpecifiedOperatingChannel: super::super::Foundation::BOOLEAN,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bLocalGO: super::super::Foundation::BOOLEAN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SEND_INVITATION_REQUEST_PARAMETERS")
            .field("Header", &self.Header)
            .field("DialogToken", &self.DialogToken)
            .field("PeerDeviceAddress", &self.PeerDeviceAddress)
            .field("uSendTimeout", &self.uSendTimeout)
            .field("MinimumConfigTimeout", &self.MinimumConfigTimeout)
            .field("InvitationFlags", &self.InvitationFlags)
            .field("GroupBSSID", &self.GroupBSSID)
            .field("bUseGroupBSSID", &self.bUseGroupBSSID)
            .field("OperatingChannel", &self.OperatingChannel)
            .field("bUseSpecifiedOperatingChannel", &self.bUseSpecifiedOperatingChannel)
            .field("GroupID", &self.GroupID)
            .field("bLocalGO", &self.bLocalGO)
            .field("uIEsOffset", &self.uIEsOffset)
            .field("uIEsLength", &self.uIEsLength)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.DialogToken == other.DialogToken && self.PeerDeviceAddress == other.PeerDeviceAddress && self.uSendTimeout == other.uSendTimeout && self.MinimumConfigTimeout == other.MinimumConfigTimeout && self.InvitationFlags == other.InvitationFlags && self.GroupBSSID == other.GroupBSSID && self.bUseGroupBSSID == other.bUseGroupBSSID && self.OperatingChannel == other.OperatingChannel && self.bUseSpecifiedOperatingChannel == other.bUseSpecifiedOperatingChannel && self.GroupID == other.GroupID && self.bLocalGO == other.bLocalGO && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_INVITATION_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub RequestContext: *mut ::core::ffi::c_void,
    pub uSendTimeout: u32,
    pub Status: u8,
    pub MinimumConfigTimeout: DOT11_WFD_CONFIGURATION_TIMEOUT,
    pub GroupBSSID: [u8; 6],
    pub bUseGroupBSSID: super::super::Foundation::BOOLEAN,
    pub OperatingChannel: DOT11_WFD_CHANNEL,
    pub bUseSpecifiedOperatingChannel: super::super::Foundation::BOOLEAN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SEND_INVITATION_RESPONSE_PARAMETERS")
            .field("Header", &self.Header)
            .field("ReceiverDeviceAddress", &self.ReceiverDeviceAddress)
            .field("DialogToken", &self.DialogToken)
            .field("RequestContext", &self.RequestContext)
            .field("uSendTimeout", &self.uSendTimeout)
            .field("Status", &self.Status)
            .field("MinimumConfigTimeout", &self.MinimumConfigTimeout)
            .field("GroupBSSID", &self.GroupBSSID)
            .field("bUseGroupBSSID", &self.bUseGroupBSSID)
            .field("OperatingChannel", &self.OperatingChannel)
            .field("bUseSpecifiedOperatingChannel", &self.bUseSpecifiedOperatingChannel)
            .field("uIEsOffset", &self.uIEsOffset)
            .field("uIEsLength", &self.uIEsLength)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ReceiverDeviceAddress == other.ReceiverDeviceAddress && self.DialogToken == other.DialogToken && self.RequestContext == other.RequestContext && self.uSendTimeout == other.uSendTimeout && self.Status == other.Status && self.MinimumConfigTimeout == other.MinimumConfigTimeout && self.GroupBSSID == other.GroupBSSID && self.bUseGroupBSSID == other.bUseGroupBSSID && self.OperatingChannel == other.OperatingChannel && self.bUseSpecifiedOperatingChannel == other.bUseSpecifiedOperatingChannel && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_INVITATION_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub DialogToken: u8,
    pub PeerDeviceAddress: [u8; 6],
    pub uSendTimeout: u32,
    pub GroupCapability: u8,
    pub GroupID: DOT11_WFD_GROUP_ID,
    pub bUseGroupID: super::super::Foundation::BOOLEAN,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS").field("Header", &self.Header).field("DialogToken", &self.DialogToken).field("PeerDeviceAddress", &self.PeerDeviceAddress).field("uSendTimeout", &self.uSendTimeout).field("GroupCapability", &self.GroupCapability).field("GroupID", &self.GroupID).field("bUseGroupID", &self.bUseGroupID).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.DialogToken == other.DialogToken && self.PeerDeviceAddress == other.PeerDeviceAddress && self.uSendTimeout == other.uSendTimeout && self.GroupCapability == other.GroupCapability && self.GroupID == other.GroupID && self.bUseGroupID == other.bUseGroupID && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ReceiverDeviceAddress: [u8; 6],
    pub DialogToken: u8,
    pub RequestContext: *mut ::core::ffi::c_void,
    pub uSendTimeout: u32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS").field("Header", &self.Header).field("ReceiverDeviceAddress", &self.ReceiverDeviceAddress).field("DialogToken", &self.DialogToken).field("RequestContext", &self.RequestContext).field("uSendTimeout", &self.uSendTimeout).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ReceiverDeviceAddress == other.ReceiverDeviceAddress && self.DialogToken == other.DialogToken && self.RequestContext == other.RequestContext && self.uSendTimeout == other.uSendTimeout && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_SSID {
    pub uSSIDLength: u32,
    pub ucSSID: [u8; 32],
}
impl ::core::marker::Copy for DOT11_SSID {}
impl ::core::clone::Clone for DOT11_SSID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_SSID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SSID").field("uSSIDLength", &self.uSSIDLength).field("ucSSID", &self.ucSSID).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_SSID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_SSID {
    fn eq(&self, other: &Self) -> bool {
        self.uSSIDLength == other.uSSIDLength && self.ucSSID == other.ucSSID
    }
}
impl ::core::cmp::Eq for DOT11_SSID {}
impl ::core::default::Default for DOT11_SSID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_SSID_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub SSIDs: [DOT11_SSID; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_SSID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_SSID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_SSID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SSID_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("SSIDs", &self.SSIDs).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_SSID_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_SSID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.SSIDs == other.SSIDs
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_SSID_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_SSID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_START_REQUEST {
    pub uStartFailureTimeout: u32,
    pub OperationalRateSet: DOT11_RATE_SET,
    pub uChCenterFrequency: u32,
    pub dot11BSSDescription: DOT11_BSS_DESCRIPTION,
}
impl ::core::marker::Copy for DOT11_START_REQUEST {}
impl ::core::clone::Clone for DOT11_START_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_START_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_START_REQUEST").field("uStartFailureTimeout", &self.uStartFailureTimeout).field("OperationalRateSet", &self.OperationalRateSet).field("uChCenterFrequency", &self.uChCenterFrequency).field("dot11BSSDescription", &self.dot11BSSDescription).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_START_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_START_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.uStartFailureTimeout == other.uStartFailureTimeout && self.OperationalRateSet == other.OperationalRateSet && self.uChCenterFrequency == other.uChCenterFrequency && self.dot11BSSDescription == other.dot11BSSDescription
    }
}
impl ::core::cmp::Eq for DOT11_START_REQUEST {}
impl ::core::default::Default for DOT11_START_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_STATISTICS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ullFourWayHandshakeFailures: u64,
    pub ullTKIPCounterMeasuresInvoked: u64,
    pub ullReserved: u64,
    pub MacUcastCounters: DOT11_MAC_FRAME_STATISTICS,
    pub MacMcastCounters: DOT11_MAC_FRAME_STATISTICS,
    pub PhyCounters: [DOT11_PHY_FRAME_STATISTICS; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_STATISTICS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_STATISTICS").field("Header", &self.Header).field("ullFourWayHandshakeFailures", &self.ullFourWayHandshakeFailures).field("ullTKIPCounterMeasuresInvoked", &self.ullTKIPCounterMeasuresInvoked).field("ullReserved", &self.ullReserved).field("MacUcastCounters", &self.MacUcastCounters).field("MacMcastCounters", &self.MacMcastCounters).field("PhyCounters", &self.PhyCounters).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_STATISTICS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ullFourWayHandshakeFailures == other.ullFourWayHandshakeFailures && self.ullTKIPCounterMeasuresInvoked == other.ullTKIPCounterMeasuresInvoked && self.ullReserved == other.ullReserved && self.MacUcastCounters == other.MacUcastCounters && self.MacMcastCounters == other.MacMcastCounters && self.PhyCounters == other.PhyCounters
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_STATISTICS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_STATUS_INDICATION {
    pub uStatusType: u32,
    pub ndisStatus: i32,
}
impl ::core::marker::Copy for DOT11_STATUS_INDICATION {}
impl ::core::clone::Clone for DOT11_STATUS_INDICATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_STATUS_INDICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_STATUS_INDICATION").field("uStatusType", &self.uStatusType).field("ndisStatus", &self.ndisStatus).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_STATUS_INDICATION {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_STATUS_INDICATION {
    fn eq(&self, other: &Self) -> bool {
        self.uStatusType == other.uStatusType && self.ndisStatus == other.ndisStatus
    }
}
impl ::core::cmp::Eq for DOT11_STATUS_INDICATION {}
impl ::core::default::Default for DOT11_STATUS_INDICATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_STOP_AP_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ulReason: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_STOP_AP_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_STOP_AP_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_STOP_AP_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_STOP_AP_PARAMETERS").field("Header", &self.Header).field("ulReason", &self.ulReason).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_STOP_AP_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_STOP_AP_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ulReason == other.ulReason
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_STOP_AP_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_STOP_AP_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_SUPPORTED_ANTENNA {
    pub uAntennaListIndex: u32,
    pub bSupportedAntenna: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_SUPPORTED_ANTENNA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_SUPPORTED_ANTENNA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_SUPPORTED_ANTENNA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_ANTENNA").field("uAntennaListIndex", &self.uAntennaListIndex).field("bSupportedAntenna", &self.bSupportedAntenna).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_SUPPORTED_ANTENNA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_ANTENNA {
    fn eq(&self, other: &Self) -> bool {
        self.uAntennaListIndex == other.uAntennaListIndex && self.bSupportedAntenna == other.bSupportedAntenna
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_SUPPORTED_ANTENNA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_SUPPORTED_ANTENNA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_SUPPORTED_ANTENNA_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11SupportedAntenna: [DOT11_SUPPORTED_ANTENNA; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_SUPPORTED_ANTENNA_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_SUPPORTED_ANTENNA_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_SUPPORTED_ANTENNA_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_ANTENNA_LIST").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11SupportedAntenna", &self.dot11SupportedAntenna).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_SUPPORTED_ANTENNA_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_ANTENNA_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11SupportedAntenna == other.dot11SupportedAntenna
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_SUPPORTED_ANTENNA_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_SUPPORTED_ANTENNA_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_SUPPORTED_DATA_RATES_VALUE {
    pub ucSupportedTxDataRatesValue: [u8; 8],
    pub ucSupportedRxDataRatesValue: [u8; 8],
}
impl ::core::marker::Copy for DOT11_SUPPORTED_DATA_RATES_VALUE {}
impl ::core::clone::Clone for DOT11_SUPPORTED_DATA_RATES_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_SUPPORTED_DATA_RATES_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_DATA_RATES_VALUE").field("ucSupportedTxDataRatesValue", &self.ucSupportedTxDataRatesValue).field("ucSupportedRxDataRatesValue", &self.ucSupportedRxDataRatesValue).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_SUPPORTED_DATA_RATES_VALUE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_DATA_RATES_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.ucSupportedTxDataRatesValue == other.ucSupportedTxDataRatesValue && self.ucSupportedRxDataRatesValue == other.ucSupportedRxDataRatesValue
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_DATA_RATES_VALUE {}
impl ::core::default::Default for DOT11_SUPPORTED_DATA_RATES_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    pub ucSupportedTxDataRatesValue: [u8; 255],
    pub ucSupportedRxDataRatesValue: [u8; 255],
}
impl ::core::marker::Copy for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {}
impl ::core::clone::Clone for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_DATA_RATES_VALUE_V2").field("ucSupportedTxDataRatesValue", &self.ucSupportedTxDataRatesValue).field("ucSupportedRxDataRatesValue", &self.ucSupportedRxDataRatesValue).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.ucSupportedTxDataRatesValue == other.ucSupportedTxDataRatesValue && self.ucSupportedRxDataRatesValue == other.ucSupportedRxDataRatesValue
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {}
impl ::core::default::Default for DOT11_SUPPORTED_DATA_RATES_VALUE_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_SUPPORTED_DSSS_CHANNEL {
    pub uChannel: u32,
}
impl ::core::marker::Copy for DOT11_SUPPORTED_DSSS_CHANNEL {}
impl ::core::clone::Clone for DOT11_SUPPORTED_DSSS_CHANNEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_SUPPORTED_DSSS_CHANNEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_DSSS_CHANNEL").field("uChannel", &self.uChannel).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_SUPPORTED_DSSS_CHANNEL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_DSSS_CHANNEL {
    fn eq(&self, other: &Self) -> bool {
        self.uChannel == other.uChannel
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_DSSS_CHANNEL {}
impl ::core::default::Default for DOT11_SUPPORTED_DSSS_CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11SupportedDSSSChannel: [DOT11_SUPPORTED_DSSS_CHANNEL; 1],
}
impl ::core::marker::Copy for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {}
impl ::core::clone::Clone for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_DSSS_CHANNEL_LIST").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11SupportedDSSSChannel", &self.dot11SupportedDSSSChannel).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11SupportedDSSSChannel == other.dot11SupportedDSSSChannel
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {}
impl ::core::default::Default for DOT11_SUPPORTED_DSSS_CHANNEL_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_SUPPORTED_OFDM_FREQUENCY {
    pub uCenterFrequency: u32,
}
impl ::core::marker::Copy for DOT11_SUPPORTED_OFDM_FREQUENCY {}
impl ::core::clone::Clone for DOT11_SUPPORTED_OFDM_FREQUENCY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_SUPPORTED_OFDM_FREQUENCY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_OFDM_FREQUENCY").field("uCenterFrequency", &self.uCenterFrequency).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_SUPPORTED_OFDM_FREQUENCY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_OFDM_FREQUENCY {
    fn eq(&self, other: &Self) -> bool {
        self.uCenterFrequency == other.uCenterFrequency
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_OFDM_FREQUENCY {}
impl ::core::default::Default for DOT11_SUPPORTED_OFDM_FREQUENCY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11SupportedOFDMFrequency: [DOT11_SUPPORTED_OFDM_FREQUENCY; 1],
}
impl ::core::marker::Copy for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {}
impl ::core::clone::Clone for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_OFDM_FREQUENCY_LIST").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11SupportedOFDMFrequency", &self.dot11SupportedOFDMFrequency).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11SupportedOFDMFrequency == other.dot11SupportedOFDMFrequency
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {}
impl ::core::default::Default for DOT11_SUPPORTED_OFDM_FREQUENCY_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_SUPPORTED_PHY_TYPES {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11PHYType: [DOT11_PHY_TYPE; 1],
}
impl ::core::marker::Copy for DOT11_SUPPORTED_PHY_TYPES {}
impl ::core::clone::Clone for DOT11_SUPPORTED_PHY_TYPES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_SUPPORTED_PHY_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_PHY_TYPES").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11PHYType", &self.dot11PHYType).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_SUPPORTED_PHY_TYPES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_PHY_TYPES {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11PHYType == other.dot11PHYType
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_PHY_TYPES {}
impl ::core::default::Default for DOT11_SUPPORTED_PHY_TYPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_SUPPORTED_POWER_LEVELS {
    pub uNumOfSupportedPowerLevels: u32,
    pub uTxPowerLevelValues: [u32; 8],
}
impl ::core::marker::Copy for DOT11_SUPPORTED_POWER_LEVELS {}
impl ::core::clone::Clone for DOT11_SUPPORTED_POWER_LEVELS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_SUPPORTED_POWER_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_SUPPORTED_POWER_LEVELS").field("uNumOfSupportedPowerLevels", &self.uNumOfSupportedPowerLevels).field("uTxPowerLevelValues", &self.uTxPowerLevelValues).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_SUPPORTED_POWER_LEVELS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_SUPPORTED_POWER_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfSupportedPowerLevels == other.uNumOfSupportedPowerLevels && self.uTxPowerLevelValues == other.uTxPowerLevelValues
    }
}
impl ::core::cmp::Eq for DOT11_SUPPORTED_POWER_LEVELS {}
impl ::core::default::Default for DOT11_SUPPORTED_POWER_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_TKIPMIC_FAILURE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub bDefaultKeyFailure: super::super::Foundation::BOOLEAN,
    pub uKeyIndex: u32,
    pub PeerMac: [u8; 6],
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_TKIPMIC_FAILURE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_TKIPMIC_FAILURE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_TKIPMIC_FAILURE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_TKIPMIC_FAILURE_PARAMETERS").field("Header", &self.Header).field("bDefaultKeyFailure", &self.bDefaultKeyFailure).field("uKeyIndex", &self.uKeyIndex).field("PeerMac", &self.PeerMac).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_TKIPMIC_FAILURE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_TKIPMIC_FAILURE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.bDefaultKeyFailure == other.bDefaultKeyFailure && self.uKeyIndex == other.uKeyIndex && self.PeerMac == other.PeerMac
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_TKIPMIC_FAILURE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_TKIPMIC_FAILURE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_UPDATE_IE {
    pub dot11UpdateIEOp: DOT11_UPDATE_IE_OP,
    pub uBufferLength: u32,
    pub ucBuffer: [u8; 1],
}
impl ::core::marker::Copy for DOT11_UPDATE_IE {}
impl ::core::clone::Clone for DOT11_UPDATE_IE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_UPDATE_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_UPDATE_IE").field("dot11UpdateIEOp", &self.dot11UpdateIEOp).field("uBufferLength", &self.uBufferLength).field("ucBuffer", &self.ucBuffer).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_UPDATE_IE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_UPDATE_IE {
    fn eq(&self, other: &Self) -> bool {
        self.dot11UpdateIEOp == other.dot11UpdateIEOp && self.uBufferLength == other.uBufferLength && self.ucBuffer == other.ucBuffer
    }
}
impl ::core::cmp::Eq for DOT11_UPDATE_IE {}
impl ::core::default::Default for DOT11_UPDATE_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_VENUEINFO {
    pub VenueGroup: u8,
    pub VenueType: u8,
}
impl ::core::marker::Copy for DOT11_VENUEINFO {}
impl ::core::clone::Clone for DOT11_VENUEINFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_VENUEINFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_VENUEINFO").field("VenueGroup", &self.VenueGroup).field("VenueType", &self.VenueType).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_VENUEINFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_VENUEINFO {
    fn eq(&self, other: &Self) -> bool {
        self.VenueGroup == other.VenueGroup && self.VenueType == other.VenueType
    }
}
impl ::core::cmp::Eq for DOT11_VENUEINFO {}
impl ::core::default::Default for DOT11_VENUEINFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_VWIFI_ATTRIBUTES {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uTotalNumOfEntries: u32,
    pub Combinations: [DOT11_VWIFI_COMBINATION; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_VWIFI_ATTRIBUTES {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_VWIFI_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_VWIFI_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_VWIFI_ATTRIBUTES").field("Header", &self.Header).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("Combinations", &self.Combinations).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_VWIFI_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_VWIFI_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.Combinations == other.Combinations
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_VWIFI_ATTRIBUTES {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_VWIFI_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_VWIFI_COMBINATION {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumInfrastructure: u32,
    pub uNumAdhoc: u32,
    pub uNumSoftAP: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_VWIFI_COMBINATION {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_VWIFI_COMBINATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_VWIFI_COMBINATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_VWIFI_COMBINATION").field("Header", &self.Header).field("uNumInfrastructure", &self.uNumInfrastructure).field("uNumAdhoc", &self.uNumAdhoc).field("uNumSoftAP", &self.uNumSoftAP).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_VWIFI_COMBINATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_VWIFI_COMBINATION {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumInfrastructure == other.uNumInfrastructure && self.uNumAdhoc == other.uNumAdhoc && self.uNumSoftAP == other.uNumSoftAP
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_VWIFI_COMBINATION {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_VWIFI_COMBINATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_VWIFI_COMBINATION_V2 {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumInfrastructure: u32,
    pub uNumAdhoc: u32,
    pub uNumSoftAP: u32,
    pub uNumVirtualStation: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_VWIFI_COMBINATION_V2 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_VWIFI_COMBINATION_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_VWIFI_COMBINATION_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_VWIFI_COMBINATION_V2").field("Header", &self.Header).field("uNumInfrastructure", &self.uNumInfrastructure).field("uNumAdhoc", &self.uNumAdhoc).field("uNumSoftAP", &self.uNumSoftAP).field("uNumVirtualStation", &self.uNumVirtualStation).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_VWIFI_COMBINATION_V2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_VWIFI_COMBINATION_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumInfrastructure == other.uNumInfrastructure && self.uNumAdhoc == other.uNumAdhoc && self.uNumSoftAP == other.uNumSoftAP && self.uNumVirtualStation == other.uNumVirtualStation
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_VWIFI_COMBINATION_V2 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_VWIFI_COMBINATION_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_VWIFI_COMBINATION_V3 {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumInfrastructure: u32,
    pub uNumAdhoc: u32,
    pub uNumSoftAP: u32,
    pub uNumVirtualStation: u32,
    pub uNumWFDGroup: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_VWIFI_COMBINATION_V3 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_VWIFI_COMBINATION_V3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_VWIFI_COMBINATION_V3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_VWIFI_COMBINATION_V3").field("Header", &self.Header).field("uNumInfrastructure", &self.uNumInfrastructure).field("uNumAdhoc", &self.uNumAdhoc).field("uNumSoftAP", &self.uNumSoftAP).field("uNumVirtualStation", &self.uNumVirtualStation).field("uNumWFDGroup", &self.uNumWFDGroup).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_VWIFI_COMBINATION_V3 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_VWIFI_COMBINATION_V3 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumInfrastructure == other.uNumInfrastructure && self.uNumAdhoc == other.uNumAdhoc && self.uNumSoftAP == other.uNumSoftAP && self.uNumVirtualStation == other.uNumVirtualStation && self.uNumWFDGroup == other.uNumWFDGroup
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_VWIFI_COMBINATION_V3 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_VWIFI_COMBINATION_V3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_WEP_OFFLOAD {
    pub uReserved: u32,
    pub hOffloadContext: super::super::Foundation::HANDLE,
    pub hOffload: super::super::Foundation::HANDLE,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub dwAlgorithm: u32,
    pub bRowIsOutbound: super::super::Foundation::BOOLEAN,
    pub bUseDefault: super::super::Foundation::BOOLEAN,
    pub uFlags: u32,
    pub ucMacAddress: [u8; 6],
    pub uNumOfRWsOnPeer: u32,
    pub uNumOfRWsOnMe: u32,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [u16; 16],
    pub usKeyLength: u16,
    pub ucKey: [u8; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_WEP_OFFLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_WEP_OFFLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_WEP_OFFLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WEP_OFFLOAD")
            .field("uReserved", &self.uReserved)
            .field("hOffloadContext", &self.hOffloadContext)
            .field("hOffload", &self.hOffload)
            .field("dot11OffloadType", &self.dot11OffloadType)
            .field("dwAlgorithm", &self.dwAlgorithm)
            .field("bRowIsOutbound", &self.bRowIsOutbound)
            .field("bUseDefault", &self.bUseDefault)
            .field("uFlags", &self.uFlags)
            .field("ucMacAddress", &self.ucMacAddress)
            .field("uNumOfRWsOnPeer", &self.uNumOfRWsOnPeer)
            .field("uNumOfRWsOnMe", &self.uNumOfRWsOnMe)
            .field("dot11IV48Counters", &self.dot11IV48Counters)
            .field("usDot11RWBitMaps", &self.usDot11RWBitMaps)
            .field("usKeyLength", &self.usKeyLength)
            .field("ucKey", &self.ucKey)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_WEP_OFFLOAD {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_WEP_OFFLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.hOffloadContext == other.hOffloadContext && self.hOffload == other.hOffload && self.dot11OffloadType == other.dot11OffloadType && self.dwAlgorithm == other.dwAlgorithm && self.bRowIsOutbound == other.bRowIsOutbound && self.bUseDefault == other.bUseDefault && self.uFlags == other.uFlags && self.ucMacAddress == other.ucMacAddress && self.uNumOfRWsOnPeer == other.uNumOfRWsOnPeer && self.uNumOfRWsOnMe == other.uNumOfRWsOnMe && self.dot11IV48Counters == other.dot11IV48Counters && self.usDot11RWBitMaps == other.usDot11RWBitMaps && self.usKeyLength == other.usKeyLength && self.ucKey == other.ucKey
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_WEP_OFFLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_WEP_OFFLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_WEP_UPLOAD {
    pub uReserved: u32,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub hOffload: super::super::Foundation::HANDLE,
    pub uNumOfRWsUsed: u32,
    pub dot11IV48Counters: [DOT11_IV48_COUNTER; 16],
    pub usDot11RWBitMaps: [u16; 16],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_WEP_UPLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_WEP_UPLOAD {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_WEP_UPLOAD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WEP_UPLOAD").field("uReserved", &self.uReserved).field("dot11OffloadType", &self.dot11OffloadType).field("hOffload", &self.hOffload).field("uNumOfRWsUsed", &self.uNumOfRWsUsed).field("dot11IV48Counters", &self.dot11IV48Counters).field("usDot11RWBitMaps", &self.usDot11RWBitMaps).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_WEP_UPLOAD {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_WEP_UPLOAD {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.dot11OffloadType == other.dot11OffloadType && self.hOffload == other.hOffload && self.uNumOfRWsUsed == other.uNumOfRWsUsed && self.dot11IV48Counters == other.dot11IV48Counters && self.usDot11RWBitMaps == other.usDot11RWBitMaps
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_WEP_UPLOAD {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_WEP_UPLOAD {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_WFD_ADDITIONAL_IE {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uBeaconIEsOffset: u32,
    pub uBeaconIEsLength: u32,
    pub uProbeResponseIEsOffset: u32,
    pub uProbeResponseIEsLength: u32,
    pub uDefaultRequestIEsOffset: u32,
    pub uDefaultRequestIEsLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_WFD_ADDITIONAL_IE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_WFD_ADDITIONAL_IE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_WFD_ADDITIONAL_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_ADDITIONAL_IE").field("Header", &self.Header).field("uBeaconIEsOffset", &self.uBeaconIEsOffset).field("uBeaconIEsLength", &self.uBeaconIEsLength).field("uProbeResponseIEsOffset", &self.uProbeResponseIEsOffset).field("uProbeResponseIEsLength", &self.uProbeResponseIEsLength).field("uDefaultRequestIEsOffset", &self.uDefaultRequestIEsOffset).field("uDefaultRequestIEsLength", &self.uDefaultRequestIEsLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_WFD_ADDITIONAL_IE {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_ADDITIONAL_IE {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uBeaconIEsOffset == other.uBeaconIEsOffset && self.uBeaconIEsLength == other.uBeaconIEsLength && self.uProbeResponseIEsOffset == other.uProbeResponseIEsOffset && self.uProbeResponseIEsLength == other.uProbeResponseIEsLength && self.uDefaultRequestIEsOffset == other.uDefaultRequestIEsOffset && self.uDefaultRequestIEsLength == other.uDefaultRequestIEsLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_ADDITIONAL_IE {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_ADDITIONAL_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    pub AdvertisementID: u32,
    pub ConfigMethods: u16,
    pub ServiceNameLength: u8,
    pub ServiceName: [u8; 255],
}
impl ::core::marker::Copy for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {}
impl ::core::clone::Clone for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR").field("AdvertisementID", &self.AdvertisementID).field("ConfigMethods", &self.ConfigMethods).field("ServiceNameLength", &self.ServiceNameLength).field("ServiceName", &self.ServiceName).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.AdvertisementID == other.AdvertisementID && self.ConfigMethods == other.ConfigMethods && self.ServiceNameLength == other.ServiceNameLength && self.ServiceName == other.ServiceName
    }
}
impl ::core::cmp::Eq for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {}
impl ::core::default::Default for DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WFD_ADVERTISED_SERVICE_LIST {
    pub ServiceCount: u16,
    pub AdvertisedService: [DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR; 1],
}
impl ::core::marker::Copy for DOT11_WFD_ADVERTISED_SERVICE_LIST {}
impl ::core::clone::Clone for DOT11_WFD_ADVERTISED_SERVICE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WFD_ADVERTISED_SERVICE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_ADVERTISED_SERVICE_LIST").field("ServiceCount", &self.ServiceCount).field("AdvertisedService", &self.AdvertisedService).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_ADVERTISED_SERVICE_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WFD_ADVERTISED_SERVICE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.ServiceCount == other.ServiceCount && self.AdvertisedService == other.AdvertisedService
    }
}
impl ::core::cmp::Eq for DOT11_WFD_ADVERTISED_SERVICE_LIST {}
impl ::core::default::Default for DOT11_WFD_ADVERTISED_SERVICE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WFD_ADVERTISEMENT_ID {
    pub AdvertisementID: u32,
    pub ServiceAddress: [u8; 6],
}
impl ::core::marker::Copy for DOT11_WFD_ADVERTISEMENT_ID {}
impl ::core::clone::Clone for DOT11_WFD_ADVERTISEMENT_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WFD_ADVERTISEMENT_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_ADVERTISEMENT_ID").field("AdvertisementID", &self.AdvertisementID).field("ServiceAddress", &self.ServiceAddress).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_ADVERTISEMENT_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WFD_ADVERTISEMENT_ID {
    fn eq(&self, other: &Self) -> bool {
        self.AdvertisementID == other.AdvertisementID && self.ServiceAddress == other.ServiceAddress
    }
}
impl ::core::cmp::Eq for DOT11_WFD_ADVERTISEMENT_ID {}
impl ::core::default::Default for DOT11_WFD_ADVERTISEMENT_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_WFD_ATTRIBUTES {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumConcurrentGORole: u32,
    pub uNumConcurrentClientRole: u32,
    pub WPSVersionsSupported: u32,
    pub bServiceDiscoverySupported: super::super::Foundation::BOOLEAN,
    pub bClientDiscoverabilitySupported: super::super::Foundation::BOOLEAN,
    pub bInfrastructureManagementSupported: super::super::Foundation::BOOLEAN,
    pub uMaxSecondaryDeviceTypeListSize: u32,
    pub DeviceAddress: [u8; 6],
    pub uInterfaceAddressListCount: u32,
    pub pInterfaceAddressList: *mut u8,
    pub uNumSupportedCountryOrRegionStrings: u32,
    pub pSupportedCountryOrRegionStrings: *mut u8,
    pub uDiscoveryFilterListSize: u32,
    pub uGORoleClientTableSize: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_WFD_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_WFD_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_WFD_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_ATTRIBUTES")
            .field("Header", &self.Header)
            .field("uNumConcurrentGORole", &self.uNumConcurrentGORole)
            .field("uNumConcurrentClientRole", &self.uNumConcurrentClientRole)
            .field("WPSVersionsSupported", &self.WPSVersionsSupported)
            .field("bServiceDiscoverySupported", &self.bServiceDiscoverySupported)
            .field("bClientDiscoverabilitySupported", &self.bClientDiscoverabilitySupported)
            .field("bInfrastructureManagementSupported", &self.bInfrastructureManagementSupported)
            .field("uMaxSecondaryDeviceTypeListSize", &self.uMaxSecondaryDeviceTypeListSize)
            .field("DeviceAddress", &self.DeviceAddress)
            .field("uInterfaceAddressListCount", &self.uInterfaceAddressListCount)
            .field("pInterfaceAddressList", &self.pInterfaceAddressList)
            .field("uNumSupportedCountryOrRegionStrings", &self.uNumSupportedCountryOrRegionStrings)
            .field("pSupportedCountryOrRegionStrings", &self.pSupportedCountryOrRegionStrings)
            .field("uDiscoveryFilterListSize", &self.uDiscoveryFilterListSize)
            .field("uGORoleClientTableSize", &self.uGORoleClientTableSize)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_WFD_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header
            && self.uNumConcurrentGORole == other.uNumConcurrentGORole
            && self.uNumConcurrentClientRole == other.uNumConcurrentClientRole
            && self.WPSVersionsSupported == other.WPSVersionsSupported
            && self.bServiceDiscoverySupported == other.bServiceDiscoverySupported
            && self.bClientDiscoverabilitySupported == other.bClientDiscoverabilitySupported
            && self.bInfrastructureManagementSupported == other.bInfrastructureManagementSupported
            && self.uMaxSecondaryDeviceTypeListSize == other.uMaxSecondaryDeviceTypeListSize
            && self.DeviceAddress == other.DeviceAddress
            && self.uInterfaceAddressListCount == other.uInterfaceAddressListCount
            && self.pInterfaceAddressList == other.pInterfaceAddressList
            && self.uNumSupportedCountryOrRegionStrings == other.uNumSupportedCountryOrRegionStrings
            && self.pSupportedCountryOrRegionStrings == other.pSupportedCountryOrRegionStrings
            && self.uDiscoveryFilterListSize == other.uDiscoveryFilterListSize
            && self.uGORoleClientTableSize == other.uGORoleClientTableSize
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_ATTRIBUTES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WFD_CHANNEL {
    pub CountryRegionString: [u8; 3],
    pub OperatingClass: u8,
    pub ChannelNumber: u8,
}
impl ::core::marker::Copy for DOT11_WFD_CHANNEL {}
impl ::core::clone::Clone for DOT11_WFD_CHANNEL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WFD_CHANNEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_CHANNEL").field("CountryRegionString", &self.CountryRegionString).field("OperatingClass", &self.OperatingClass).field("ChannelNumber", &self.ChannelNumber).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_CHANNEL {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WFD_CHANNEL {
    fn eq(&self, other: &Self) -> bool {
        self.CountryRegionString == other.CountryRegionString && self.OperatingClass == other.OperatingClass && self.ChannelNumber == other.ChannelNumber
    }
}
impl ::core::cmp::Eq for DOT11_WFD_CHANNEL {}
impl ::core::default::Default for DOT11_WFD_CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WFD_CONFIGURATION_TIMEOUT {
    pub GOTimeout: u8,
    pub ClientTimeout: u8,
}
impl ::core::marker::Copy for DOT11_WFD_CONFIGURATION_TIMEOUT {}
impl ::core::clone::Clone for DOT11_WFD_CONFIGURATION_TIMEOUT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WFD_CONFIGURATION_TIMEOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_CONFIGURATION_TIMEOUT").field("GOTimeout", &self.GOTimeout).field("ClientTimeout", &self.ClientTimeout).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_CONFIGURATION_TIMEOUT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WFD_CONFIGURATION_TIMEOUT {
    fn eq(&self, other: &Self) -> bool {
        self.GOTimeout == other.GOTimeout && self.ClientTimeout == other.ClientTimeout
    }
}
impl ::core::cmp::Eq for DOT11_WFD_CONFIGURATION_TIMEOUT {}
impl ::core::default::Default for DOT11_WFD_CONFIGURATION_TIMEOUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub bServiceDiscoveryEnabled: super::super::Foundation::BOOLEAN,
    pub bClientDiscoverabilityEnabled: super::super::Foundation::BOOLEAN,
    pub bConcurrentOperationSupported: super::super::Foundation::BOOLEAN,
    pub bInfrastructureManagementEnabled: super::super::Foundation::BOOLEAN,
    pub bDeviceLimitReached: super::super::Foundation::BOOLEAN,
    pub bInvitationProcedureEnabled: super::super::Foundation::BOOLEAN,
    pub WPSVersionsEnabled: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_DEVICE_CAPABILITY_CONFIG")
            .field("Header", &self.Header)
            .field("bServiceDiscoveryEnabled", &self.bServiceDiscoveryEnabled)
            .field("bClientDiscoverabilityEnabled", &self.bClientDiscoverabilityEnabled)
            .field("bConcurrentOperationSupported", &self.bConcurrentOperationSupported)
            .field("bInfrastructureManagementEnabled", &self.bInfrastructureManagementEnabled)
            .field("bDeviceLimitReached", &self.bDeviceLimitReached)
            .field("bInvitationProcedureEnabled", &self.bInvitationProcedureEnabled)
            .field("WPSVersionsEnabled", &self.WPSVersionsEnabled)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.bServiceDiscoveryEnabled == other.bServiceDiscoveryEnabled && self.bClientDiscoverabilityEnabled == other.bClientDiscoverabilityEnabled && self.bConcurrentOperationSupported == other.bConcurrentOperationSupported && self.bInfrastructureManagementEnabled == other.bInfrastructureManagementEnabled && self.bDeviceLimitReached == other.bDeviceLimitReached && self.bInvitationProcedureEnabled == other.bInvitationProcedureEnabled && self.WPSVersionsEnabled == other.WPSVersionsEnabled
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_DEVICE_CAPABILITY_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WFD_DEVICE_ENTRY {
    pub uPhyId: u32,
    pub PhySpecificInfo: DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO,
    pub dot11BSSID: [u8; 6],
    pub dot11BSSType: DOT11_BSS_TYPE,
    pub TransmitterAddress: [u8; 6],
    pub lRSSI: i32,
    pub uLinkQuality: u32,
    pub usBeaconPeriod: u16,
    pub ullTimestamp: u64,
    pub ullBeaconHostTimestamp: u64,
    pub ullProbeResponseHostTimestamp: u64,
    pub usCapabilityInformation: u16,
    pub uBeaconIEsOffset: u32,
    pub uBeaconIEsLength: u32,
    pub uProbeResponseIEsOffset: u32,
    pub uProbeResponseIEsLength: u32,
}
impl ::core::marker::Copy for DOT11_WFD_DEVICE_ENTRY {}
impl ::core::clone::Clone for DOT11_WFD_DEVICE_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_DEVICE_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::default::Default for DOT11_WFD_DEVICE_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_WFD_DEVICE_INFO {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub DeviceAddress: [u8; 6],
    pub ConfigMethods: u16,
    pub PrimaryDeviceType: DOT11_WFD_DEVICE_TYPE,
    pub DeviceName: DOT11_WPS_DEVICE_NAME,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_WFD_DEVICE_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_WFD_DEVICE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_WFD_DEVICE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_DEVICE_INFO").field("Header", &self.Header).field("DeviceAddress", &self.DeviceAddress).field("ConfigMethods", &self.ConfigMethods).field("PrimaryDeviceType", &self.PrimaryDeviceType).field("DeviceName", &self.DeviceName).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_WFD_DEVICE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_DEVICE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.DeviceAddress == other.DeviceAddress && self.ConfigMethods == other.ConfigMethods && self.PrimaryDeviceType == other.PrimaryDeviceType && self.DeviceName == other.DeviceName
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_DEVICE_INFO {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_DEVICE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub ChannelNumber: u8,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_WFD_DEVICE_LISTEN_CHANNEL {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_DEVICE_LISTEN_CHANNEL").field("Header", &self.Header).field("ChannelNumber", &self.ChannelNumber).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.ChannelNumber == other.ChannelNumber
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_DEVICE_LISTEN_CHANNEL {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_DEVICE_LISTEN_CHANNEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WFD_DEVICE_TYPE {
    pub CategoryID: u16,
    pub SubCategoryID: u16,
    pub OUI: [u8; 4],
}
impl ::core::marker::Copy for DOT11_WFD_DEVICE_TYPE {}
impl ::core::clone::Clone for DOT11_WFD_DEVICE_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WFD_DEVICE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_DEVICE_TYPE").field("CategoryID", &self.CategoryID).field("SubCategoryID", &self.SubCategoryID).field("OUI", &self.OUI).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_DEVICE_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WFD_DEVICE_TYPE {
    fn eq(&self, other: &Self) -> bool {
        self.CategoryID == other.CategoryID && self.SubCategoryID == other.SubCategoryID && self.OUI == other.OUI
    }
}
impl ::core::cmp::Eq for DOT11_WFD_DEVICE_TYPE {}
impl ::core::default::Default for DOT11_WFD_DEVICE_TYPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub Status: i32,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub uListOffset: u32,
    pub uListLength: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS").field("Header", &self.Header).field("Status", &self.Status).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("uListOffset", &self.uListOffset).field("uListLength", &self.uListLength).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.Status == other.Status && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.uListOffset == other.uListOffset && self.uListLength == other.uListLength
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WFD_DISCOVER_DEVICE_FILTER {
    pub DeviceID: [u8; 6],
    pub ucBitmask: u8,
    pub GroupSSID: DOT11_SSID,
}
impl ::core::marker::Copy for DOT11_WFD_DISCOVER_DEVICE_FILTER {}
impl ::core::clone::Clone for DOT11_WFD_DISCOVER_DEVICE_FILTER {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WFD_DISCOVER_DEVICE_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_DISCOVER_DEVICE_FILTER").field("DeviceID", &self.DeviceID).field("ucBitmask", &self.ucBitmask).field("GroupSSID", &self.GroupSSID).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_DISCOVER_DEVICE_FILTER {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WFD_DISCOVER_DEVICE_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceID == other.DeviceID && self.ucBitmask == other.ucBitmask && self.GroupSSID == other.GroupSSID
    }
}
impl ::core::cmp::Eq for DOT11_WFD_DISCOVER_DEVICE_FILTER {}
impl ::core::default::Default for DOT11_WFD_DISCOVER_DEVICE_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_WFD_DISCOVER_REQUEST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub DiscoverType: DOT11_WFD_DISCOVER_TYPE,
    pub ScanType: DOT11_WFD_SCAN_TYPE,
    pub uDiscoverTimeout: u32,
    pub uDeviceFilterListOffset: u32,
    pub uNumDeviceFilters: u32,
    pub uIEsOffset: u32,
    pub uIEsLength: u32,
    pub bForceScanLegacyNetworks: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_WFD_DISCOVER_REQUEST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_WFD_DISCOVER_REQUEST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_WFD_DISCOVER_REQUEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_DISCOVER_REQUEST").field("Header", &self.Header).field("DiscoverType", &self.DiscoverType).field("ScanType", &self.ScanType).field("uDiscoverTimeout", &self.uDiscoverTimeout).field("uDeviceFilterListOffset", &self.uDeviceFilterListOffset).field("uNumDeviceFilters", &self.uNumDeviceFilters).field("uIEsOffset", &self.uIEsOffset).field("uIEsLength", &self.uIEsLength).field("bForceScanLegacyNetworks", &self.bForceScanLegacyNetworks).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_WFD_DISCOVER_REQUEST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_DISCOVER_REQUEST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.DiscoverType == other.DiscoverType && self.ScanType == other.ScanType && self.uDiscoverTimeout == other.uDiscoverTimeout && self.uDeviceFilterListOffset == other.uDeviceFilterListOffset && self.uNumDeviceFilters == other.uNumDeviceFilters && self.uIEsOffset == other.uIEsOffset && self.uIEsLength == other.uIEsLength && self.bForceScanLegacyNetworks == other.bForceScanLegacyNetworks
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_DISCOVER_REQUEST {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_DISCOVER_REQUEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WFD_GO_INTENT {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for DOT11_WFD_GO_INTENT {}
impl ::core::clone::Clone for DOT11_WFD_GO_INTENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WFD_GO_INTENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_GO_INTENT").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_GO_INTENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WFD_GO_INTENT {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DOT11_WFD_GO_INTENT {}
impl ::core::default::Default for DOT11_WFD_GO_INTENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WFD_GROUP_ID {
    pub DeviceAddress: [u8; 6],
    pub SSID: DOT11_SSID,
}
impl ::core::marker::Copy for DOT11_WFD_GROUP_ID {}
impl ::core::clone::Clone for DOT11_WFD_GROUP_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WFD_GROUP_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_GROUP_ID").field("DeviceAddress", &self.DeviceAddress).field("SSID", &self.SSID).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_GROUP_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_ID {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceAddress == other.DeviceAddress && self.SSID == other.SSID
    }
}
impl ::core::cmp::Eq for DOT11_WFD_GROUP_ID {}
impl ::core::default::Default for DOT11_WFD_GROUP_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_WFD_GROUP_JOIN_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub GOOperatingChannel: DOT11_WFD_CHANNEL,
    pub GOConfigTime: u32,
    pub bInGroupFormation: super::super::Foundation::BOOLEAN,
    pub bWaitForWPSReady: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_WFD_GROUP_JOIN_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_WFD_GROUP_JOIN_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_WFD_GROUP_JOIN_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_GROUP_JOIN_PARAMETERS").field("Header", &self.Header).field("GOOperatingChannel", &self.GOOperatingChannel).field("GOConfigTime", &self.GOConfigTime).field("bInGroupFormation", &self.bInGroupFormation).field("bWaitForWPSReady", &self.bWaitForWPSReady).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_WFD_GROUP_JOIN_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_JOIN_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.GOOperatingChannel == other.GOOperatingChannel && self.GOConfigTime == other.GOConfigTime && self.bInGroupFormation == other.bInGroupFormation && self.bWaitForWPSReady == other.bWaitForWPSReady
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_GROUP_JOIN_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_GROUP_JOIN_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub bPersistentGroupEnabled: super::super::Foundation::BOOLEAN,
    pub bIntraBSSDistributionSupported: super::super::Foundation::BOOLEAN,
    pub bCrossConnectionSupported: super::super::Foundation::BOOLEAN,
    pub bPersistentReconnectSupported: super::super::Foundation::BOOLEAN,
    pub bGroupFormationEnabled: super::super::Foundation::BOOLEAN,
    pub uMaximumGroupLimit: u32,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG")
            .field("Header", &self.Header)
            .field("bPersistentGroupEnabled", &self.bPersistentGroupEnabled)
            .field("bIntraBSSDistributionSupported", &self.bIntraBSSDistributionSupported)
            .field("bCrossConnectionSupported", &self.bCrossConnectionSupported)
            .field("bPersistentReconnectSupported", &self.bPersistentReconnectSupported)
            .field("bGroupFormationEnabled", &self.bGroupFormationEnabled)
            .field("uMaximumGroupLimit", &self.uMaximumGroupLimit)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.bPersistentGroupEnabled == other.bPersistentGroupEnabled && self.bIntraBSSDistributionSupported == other.bIntraBSSDistributionSupported && self.bCrossConnectionSupported == other.bCrossConnectionSupported && self.bPersistentReconnectSupported == other.bPersistentReconnectSupported && self.bGroupFormationEnabled == other.bGroupFormationEnabled && self.uMaximumGroupLimit == other.uMaximumGroupLimit
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub struct DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub bPersistentGroupEnabled: super::super::Foundation::BOOLEAN,
    pub bIntraBSSDistributionSupported: super::super::Foundation::BOOLEAN,
    pub bCrossConnectionSupported: super::super::Foundation::BOOLEAN,
    pub bPersistentReconnectSupported: super::super::Foundation::BOOLEAN,
    pub bGroupFormationEnabled: super::super::Foundation::BOOLEAN,
    pub uMaximumGroupLimit: u32,
    pub bEapolKeyIpAddressAllocationSupported: super::super::Foundation::BOOLEAN,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::marker::Copy for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::clone::Clone for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::fmt::Debug for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2")
            .field("Header", &self.Header)
            .field("bPersistentGroupEnabled", &self.bPersistentGroupEnabled)
            .field("bIntraBSSDistributionSupported", &self.bIntraBSSDistributionSupported)
            .field("bCrossConnectionSupported", &self.bCrossConnectionSupported)
            .field("bPersistentReconnectSupported", &self.bPersistentReconnectSupported)
            .field("bGroupFormationEnabled", &self.bGroupFormationEnabled)
            .field("uMaximumGroupLimit", &self.uMaximumGroupLimit)
            .field("bEapolKeyIpAddressAllocationSupported", &self.bEapolKeyIpAddressAllocationSupported)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::windows::core::TypeKind for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.bPersistentGroupEnabled == other.bPersistentGroupEnabled && self.bIntraBSSDistributionSupported == other.bIntraBSSDistributionSupported && self.bCrossConnectionSupported == other.bCrossConnectionSupported && self.bPersistentReconnectSupported == other.bPersistentReconnectSupported && self.bGroupFormationEnabled == other.bGroupFormationEnabled && self.uMaximumGroupLimit == other.uMaximumGroupLimit && self.bEapolKeyIpAddressAllocationSupported == other.bEapolKeyIpAddressAllocationSupported
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::cmp::Eq for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
impl ::core::default::Default for DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_WFD_GROUP_START_PARAMETERS {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub AdvertisedOperatingChannel: DOT11_WFD_CHANNEL,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_WFD_GROUP_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_WFD_GROUP_START_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_WFD_GROUP_START_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_GROUP_START_PARAMETERS").field("Header", &self.Header).field("AdvertisedOperatingChannel", &self.AdvertisedOperatingChannel).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_WFD_GROUP_START_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_GROUP_START_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.AdvertisedOperatingChannel == other.AdvertisedOperatingChannel
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_GROUP_START_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_GROUP_START_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WFD_INVITATION_FLAGS {
    pub _bitfield: u8,
}
impl ::core::marker::Copy for DOT11_WFD_INVITATION_FLAGS {}
impl ::core::clone::Clone for DOT11_WFD_INVITATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WFD_INVITATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_INVITATION_FLAGS").field("_bitfield", &self._bitfield).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_INVITATION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WFD_INVITATION_FLAGS {
    fn eq(&self, other: &Self) -> bool {
        self._bitfield == other._bitfield
    }
}
impl ::core::cmp::Eq for DOT11_WFD_INVITATION_FLAGS {}
impl ::core::default::Default for DOT11_WFD_INVITATION_FLAGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    pub Header: super::Ndis::NDIS_OBJECT_HEADER,
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub SecondaryDeviceTypes: [DOT11_WFD_DEVICE_TYPE; 1],
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST").field("Header", &self.Header).field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("SecondaryDeviceTypes", &self.SecondaryDeviceTypes).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.Header == other.Header && self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.SecondaryDeviceTypes == other.SecondaryDeviceTypes
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WFD_SERVICE_HASH_LIST {
    pub ServiceHashCount: u16,
    pub ServiceHash: [u8; 6],
}
impl ::core::marker::Copy for DOT11_WFD_SERVICE_HASH_LIST {}
impl ::core::clone::Clone for DOT11_WFD_SERVICE_HASH_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WFD_SERVICE_HASH_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_SERVICE_HASH_LIST").field("ServiceHashCount", &self.ServiceHashCount).field("ServiceHash", &self.ServiceHash).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_SERVICE_HASH_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WFD_SERVICE_HASH_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.ServiceHashCount == other.ServiceHashCount && self.ServiceHash == other.ServiceHash
    }
}
impl ::core::cmp::Eq for DOT11_WFD_SERVICE_HASH_LIST {}
impl ::core::default::Default for DOT11_WFD_SERVICE_HASH_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WFD_SESSION_ID {
    pub SessionID: u32,
    pub SessionAddress: [u8; 6],
}
impl ::core::marker::Copy for DOT11_WFD_SESSION_ID {}
impl ::core::clone::Clone for DOT11_WFD_SESSION_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WFD_SESSION_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_SESSION_ID").field("SessionID", &self.SessionID).field("SessionAddress", &self.SessionAddress).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_SESSION_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WFD_SESSION_ID {
    fn eq(&self, other: &Self) -> bool {
        self.SessionID == other.SessionID && self.SessionAddress == other.SessionAddress
    }
}
impl ::core::cmp::Eq for DOT11_WFD_SESSION_ID {}
impl ::core::default::Default for DOT11_WFD_SESSION_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WFD_SESSION_INFO {
    pub uSessionInfoLength: u16,
    pub ucSessionInfo: [u8; 144],
}
impl ::core::marker::Copy for DOT11_WFD_SESSION_INFO {}
impl ::core::clone::Clone for DOT11_WFD_SESSION_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WFD_SESSION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WFD_SESSION_INFO").field("uSessionInfoLength", &self.uSessionInfoLength).field("ucSessionInfo", &self.ucSessionInfo).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WFD_SESSION_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WFD_SESSION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.uSessionInfoLength == other.uSessionInfoLength && self.ucSessionInfo == other.ucSessionInfo
    }
}
impl ::core::cmp::Eq for DOT11_WFD_SESSION_INFO {}
impl ::core::default::Default for DOT11_WFD_SESSION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WME_AC_PARAMETERS {
    pub ucAccessCategoryIndex: u8,
    pub ucAIFSN: u8,
    pub ucECWmin: u8,
    pub ucECWmax: u8,
    pub usTXOPLimit: u16,
}
impl ::core::marker::Copy for DOT11_WME_AC_PARAMETERS {}
impl ::core::clone::Clone for DOT11_WME_AC_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WME_AC_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WME_AC_PARAMETERS").field("ucAccessCategoryIndex", &self.ucAccessCategoryIndex).field("ucAIFSN", &self.ucAIFSN).field("ucECWmin", &self.ucECWmin).field("ucECWmax", &self.ucECWmax).field("usTXOPLimit", &self.usTXOPLimit).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WME_AC_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WME_AC_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.ucAccessCategoryIndex == other.ucAccessCategoryIndex && self.ucAIFSN == other.ucAIFSN && self.ucECWmin == other.ucECWmin && self.ucECWmax == other.ucECWmax && self.usTXOPLimit == other.usTXOPLimit
    }
}
impl ::core::cmp::Eq for DOT11_WME_AC_PARAMETERS {}
impl ::core::default::Default for DOT11_WME_AC_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WME_AC_PARAMETERS_LIST {
    pub uNumOfEntries: u32,
    pub uTotalNumOfEntries: u32,
    pub dot11WMEACParameters: [DOT11_WME_AC_PARAMETERS; 1],
}
impl ::core::marker::Copy for DOT11_WME_AC_PARAMETERS_LIST {}
impl ::core::clone::Clone for DOT11_WME_AC_PARAMETERS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WME_AC_PARAMETERS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WME_AC_PARAMETERS_LIST").field("uNumOfEntries", &self.uNumOfEntries).field("uTotalNumOfEntries", &self.uTotalNumOfEntries).field("dot11WMEACParameters", &self.dot11WMEACParameters).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WME_AC_PARAMETERS_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WME_AC_PARAMETERS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.uNumOfEntries == other.uNumOfEntries && self.uTotalNumOfEntries == other.uTotalNumOfEntries && self.dot11WMEACParameters == other.dot11WMEACParameters
    }
}
impl ::core::cmp::Eq for DOT11_WME_AC_PARAMETERS_LIST {}
impl ::core::default::Default for DOT11_WME_AC_PARAMETERS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WME_UPDATE_IE {
    pub uParamElemMinBeaconIntervals: u32,
    pub uWMEInfoElemOffset: u32,
    pub uWMEInfoElemLength: u32,
    pub uWMEParamElemOffset: u32,
    pub uWMEParamElemLength: u32,
    pub ucBuffer: [u8; 1],
}
impl ::core::marker::Copy for DOT11_WME_UPDATE_IE {}
impl ::core::clone::Clone for DOT11_WME_UPDATE_IE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WME_UPDATE_IE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WME_UPDATE_IE").field("uParamElemMinBeaconIntervals", &self.uParamElemMinBeaconIntervals).field("uWMEInfoElemOffset", &self.uWMEInfoElemOffset).field("uWMEInfoElemLength", &self.uWMEInfoElemLength).field("uWMEParamElemOffset", &self.uWMEParamElemOffset).field("uWMEParamElemLength", &self.uWMEParamElemLength).field("ucBuffer", &self.ucBuffer).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WME_UPDATE_IE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WME_UPDATE_IE {
    fn eq(&self, other: &Self) -> bool {
        self.uParamElemMinBeaconIntervals == other.uParamElemMinBeaconIntervals && self.uWMEInfoElemOffset == other.uWMEInfoElemOffset && self.uWMEInfoElemLength == other.uWMEInfoElemLength && self.uWMEParamElemOffset == other.uWMEParamElemOffset && self.uWMEParamElemLength == other.uWMEParamElemLength && self.ucBuffer == other.ucBuffer
    }
}
impl ::core::cmp::Eq for DOT11_WME_UPDATE_IE {}
impl ::core::default::Default for DOT11_WME_UPDATE_IE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct DOT11_WPA_TSC {
    pub uReserved: u32,
    pub dot11OffloadType: DOT11_OFFLOAD_TYPE,
    pub hOffload: super::super::Foundation::HANDLE,
    pub dot11IV48Counter: DOT11_IV48_COUNTER,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for DOT11_WPA_TSC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for DOT11_WPA_TSC {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DOT11_WPA_TSC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WPA_TSC").field("uReserved", &self.uReserved).field("dot11OffloadType", &self.dot11OffloadType).field("hOffload", &self.hOffload).field("dot11IV48Counter", &self.dot11IV48Counter).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for DOT11_WPA_TSC {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DOT11_WPA_TSC {
    fn eq(&self, other: &Self) -> bool {
        self.uReserved == other.uReserved && self.dot11OffloadType == other.dot11OffloadType && self.hOffload == other.hOffload && self.dot11IV48Counter == other.dot11IV48Counter
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DOT11_WPA_TSC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DOT11_WPA_TSC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct DOT11_WPS_DEVICE_NAME {
    pub uDeviceNameLength: u32,
    pub ucDeviceName: [u8; 32],
}
impl ::core::marker::Copy for DOT11_WPS_DEVICE_NAME {}
impl ::core::clone::Clone for DOT11_WPS_DEVICE_NAME {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for DOT11_WPS_DEVICE_NAME {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DOT11_WPS_DEVICE_NAME").field("uDeviceNameLength", &self.uDeviceNameLength).field("ucDeviceName", &self.ucDeviceName).finish()
    }
}
impl ::windows::core::TypeKind for DOT11_WPS_DEVICE_NAME {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for DOT11_WPS_DEVICE_NAME {
    fn eq(&self, other: &Self) -> bool {
        self.uDeviceNameLength == other.uDeviceNameLength && self.ucDeviceName == other.ucDeviceName
    }
}
impl ::core::cmp::Eq for DOT11_WPS_DEVICE_NAME {}
impl ::core::default::Default for DOT11_WPS_DEVICE_NAME {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct L2_NOTIFICATION_DATA {
    pub NotificationSource: u32,
    pub NotificationCode: u32,
    pub InterfaceGuid: ::windows::core::GUID,
    pub dwDataSize: u32,
    pub pData: *mut ::core::ffi::c_void,
}
impl ::core::marker::Copy for L2_NOTIFICATION_DATA {}
impl ::core::clone::Clone for L2_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for L2_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("L2_NOTIFICATION_DATA").field("NotificationSource", &self.NotificationSource).field("NotificationCode", &self.NotificationCode).field("InterfaceGuid", &self.InterfaceGuid).field("dwDataSize", &self.dwDataSize).field("pData", &self.pData).finish()
    }
}
impl ::windows::core::TypeKind for L2_NOTIFICATION_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for L2_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.NotificationSource == other.NotificationSource && self.NotificationCode == other.NotificationCode && self.InterfaceGuid == other.InterfaceGuid && self.dwDataSize == other.dwDataSize && self.pData == other.pData
    }
}
impl ::core::cmp::Eq for L2_NOTIFICATION_DATA {}
impl ::core::default::Default for L2_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ONEX_AUTH_PARAMS {
    pub fUpdatePending: super::super::Foundation::BOOL,
    pub oneXConnProfile: ONEX_VARIABLE_BLOB,
    pub authIdentity: ONEX_AUTH_IDENTITY,
    pub dwQuarantineState: u32,
    pub _bitfield: u32,
    pub dwSessionId: u32,
    pub hUserToken: super::super::Foundation::HANDLE,
    pub OneXUserProfile: ONEX_VARIABLE_BLOB,
    pub Identity: ONEX_VARIABLE_BLOB,
    pub UserName: ONEX_VARIABLE_BLOB,
    pub Domain: ONEX_VARIABLE_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ONEX_AUTH_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ONEX_AUTH_PARAMS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ONEX_AUTH_PARAMS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ONEX_AUTH_PARAMS")
            .field("fUpdatePending", &self.fUpdatePending)
            .field("oneXConnProfile", &self.oneXConnProfile)
            .field("authIdentity", &self.authIdentity)
            .field("dwQuarantineState", &self.dwQuarantineState)
            .field("_bitfield", &self._bitfield)
            .field("dwSessionId", &self.dwSessionId)
            .field("hUserToken", &self.hUserToken)
            .field("OneXUserProfile", &self.OneXUserProfile)
            .field("Identity", &self.Identity)
            .field("UserName", &self.UserName)
            .field("Domain", &self.Domain)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for ONEX_AUTH_PARAMS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ONEX_AUTH_PARAMS {
    fn eq(&self, other: &Self) -> bool {
        self.fUpdatePending == other.fUpdatePending && self.oneXConnProfile == other.oneXConnProfile && self.authIdentity == other.authIdentity && self.dwQuarantineState == other.dwQuarantineState && self._bitfield == other._bitfield && self.dwSessionId == other.dwSessionId && self.hUserToken == other.hUserToken && self.OneXUserProfile == other.OneXUserProfile && self.Identity == other.Identity && self.UserName == other.UserName && self.Domain == other.Domain
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ONEX_AUTH_PARAMS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ONEX_AUTH_PARAMS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
pub struct ONEX_EAP_ERROR {
    pub dwWinError: u32,
    pub r#type: super::super::Security::ExtensibleAuthenticationProtocol::EAP_METHOD_TYPE,
    pub dwReasonCode: u32,
    pub rootCauseGuid: ::windows::core::GUID,
    pub repairGuid: ::windows::core::GUID,
    pub helpLinkGuid: ::windows::core::GUID,
    pub _bitfield: u32,
    pub RootCauseString: ONEX_VARIABLE_BLOB,
    pub RepairString: ONEX_VARIABLE_BLOB,
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::marker::Copy for ONEX_EAP_ERROR {}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::clone::Clone for ONEX_EAP_ERROR {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::fmt::Debug for ONEX_EAP_ERROR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ONEX_EAP_ERROR").field("dwWinError", &self.dwWinError).field("type", &self.r#type).field("dwReasonCode", &self.dwReasonCode).field("rootCauseGuid", &self.rootCauseGuid).field("repairGuid", &self.repairGuid).field("helpLinkGuid", &self.helpLinkGuid).field("_bitfield", &self._bitfield).field("RootCauseString", &self.RootCauseString).field("RepairString", &self.RepairString).finish()
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::windows::core::TypeKind for ONEX_EAP_ERROR {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::cmp::PartialEq for ONEX_EAP_ERROR {
    fn eq(&self, other: &Self) -> bool {
        self.dwWinError == other.dwWinError && self.r#type == other.r#type && self.dwReasonCode == other.dwReasonCode && self.rootCauseGuid == other.rootCauseGuid && self.repairGuid == other.repairGuid && self.helpLinkGuid == other.helpLinkGuid && self._bitfield == other._bitfield && self.RootCauseString == other.RootCauseString && self.RepairString == other.RepairString
    }
}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::cmp::Eq for ONEX_EAP_ERROR {}
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
impl ::core::default::Default for ONEX_EAP_ERROR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct ONEX_RESULT_UPDATE_DATA {
    pub oneXStatus: ONEX_STATUS,
    pub BackendSupport: ONEX_EAP_METHOD_BACKEND_SUPPORT,
    pub fBackendEngaged: super::super::Foundation::BOOL,
    pub _bitfield: u32,
    pub authParams: ONEX_VARIABLE_BLOB,
    pub eapError: ONEX_VARIABLE_BLOB,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for ONEX_RESULT_UPDATE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for ONEX_RESULT_UPDATE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for ONEX_RESULT_UPDATE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ONEX_RESULT_UPDATE_DATA").field("oneXStatus", &self.oneXStatus).field("BackendSupport", &self.BackendSupport).field("fBackendEngaged", &self.fBackendEngaged).field("_bitfield", &self._bitfield).field("authParams", &self.authParams).field("eapError", &self.eapError).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for ONEX_RESULT_UPDATE_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for ONEX_RESULT_UPDATE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.oneXStatus == other.oneXStatus && self.BackendSupport == other.BackendSupport && self.fBackendEngaged == other.fBackendEngaged && self._bitfield == other._bitfield && self.authParams == other.authParams && self.eapError == other.eapError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for ONEX_RESULT_UPDATE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for ONEX_RESULT_UPDATE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct ONEX_STATUS {
    pub authStatus: ONEX_AUTH_STATUS,
    pub dwReason: u32,
    pub dwError: u32,
}
impl ::core::marker::Copy for ONEX_STATUS {}
impl ::core::clone::Clone for ONEX_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ONEX_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ONEX_STATUS").field("authStatus", &self.authStatus).field("dwReason", &self.dwReason).field("dwError", &self.dwError).finish()
    }
}
impl ::windows::core::TypeKind for ONEX_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ONEX_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.authStatus == other.authStatus && self.dwReason == other.dwReason && self.dwError == other.dwError
    }
}
impl ::core::cmp::Eq for ONEX_STATUS {}
impl ::core::default::Default for ONEX_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct ONEX_USER_INFO {
    pub authIdentity: ONEX_AUTH_IDENTITY,
    pub _bitfield: u32,
    pub UserName: ONEX_VARIABLE_BLOB,
    pub DomainName: ONEX_VARIABLE_BLOB,
}
impl ::core::marker::Copy for ONEX_USER_INFO {}
impl ::core::clone::Clone for ONEX_USER_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ONEX_USER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ONEX_USER_INFO").field("authIdentity", &self.authIdentity).field("_bitfield", &self._bitfield).field("UserName", &self.UserName).field("DomainName", &self.DomainName).finish()
    }
}
impl ::windows::core::TypeKind for ONEX_USER_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ONEX_USER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.authIdentity == other.authIdentity && self._bitfield == other._bitfield && self.UserName == other.UserName && self.DomainName == other.DomainName
    }
}
impl ::core::cmp::Eq for ONEX_USER_INFO {}
impl ::core::default::Default for ONEX_USER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct ONEX_VARIABLE_BLOB {
    pub dwSize: u32,
    pub dwOffset: u32,
}
impl ::core::marker::Copy for ONEX_VARIABLE_BLOB {}
impl ::core::clone::Clone for ONEX_VARIABLE_BLOB {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for ONEX_VARIABLE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ONEX_VARIABLE_BLOB").field("dwSize", &self.dwSize).field("dwOffset", &self.dwOffset).finish()
    }
}
impl ::windows::core::TypeKind for ONEX_VARIABLE_BLOB {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for ONEX_VARIABLE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.dwSize == other.dwSize && self.dwOffset == other.dwOffset
    }
}
impl ::core::cmp::Eq for ONEX_VARIABLE_BLOB {}
impl ::core::default::Default for ONEX_VARIABLE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WDIAG_IHV_WLAN_ID {
    pub strProfileName: [u16; 256],
    pub Ssid: DOT11_SSID,
    pub BssType: DOT11_BSS_TYPE,
    pub dwFlags: u32,
    pub dwReasonCode: u32,
}
impl ::core::marker::Copy for WDIAG_IHV_WLAN_ID {}
impl ::core::clone::Clone for WDIAG_IHV_WLAN_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WDIAG_IHV_WLAN_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WDIAG_IHV_WLAN_ID").field("strProfileName", &self.strProfileName).field("Ssid", &self.Ssid).field("BssType", &self.BssType).field("dwFlags", &self.dwFlags).field("dwReasonCode", &self.dwReasonCode).finish()
    }
}
impl ::windows::core::TypeKind for WDIAG_IHV_WLAN_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WDIAG_IHV_WLAN_ID {
    fn eq(&self, other: &Self) -> bool {
        self.strProfileName == other.strProfileName && self.Ssid == other.Ssid && self.BssType == other.BssType && self.dwFlags == other.dwFlags && self.dwReasonCode == other.dwReasonCode
    }
}
impl ::core::cmp::Eq for WDIAG_IHV_WLAN_ID {}
impl ::core::default::Default for WDIAG_IHV_WLAN_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WFDSVC_CONNECTION_CAPABILITY {
    pub bNew: super::super::Foundation::BOOLEAN,
    pub bClient: super::super::Foundation::BOOLEAN,
    pub bGO: super::super::Foundation::BOOLEAN,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WFDSVC_CONNECTION_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WFDSVC_CONNECTION_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WFDSVC_CONNECTION_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WFDSVC_CONNECTION_CAPABILITY").field("bNew", &self.bNew).field("bClient", &self.bClient).field("bGO", &self.bGO).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WFDSVC_CONNECTION_CAPABILITY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WFDSVC_CONNECTION_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.bNew == other.bNew && self.bClient == other.bClient && self.bGO == other.bGO
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WFDSVC_CONNECTION_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WFDSVC_CONNECTION_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WFD_GROUP_ID {
    pub DeviceAddress: [u8; 6],
    pub GroupSSID: DOT11_SSID,
}
impl ::core::marker::Copy for WFD_GROUP_ID {}
impl ::core::clone::Clone for WFD_GROUP_ID {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WFD_GROUP_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WFD_GROUP_ID").field("DeviceAddress", &self.DeviceAddress).field("GroupSSID", &self.GroupSSID).finish()
    }
}
impl ::windows::core::TypeKind for WFD_GROUP_ID {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WFD_GROUP_ID {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceAddress == other.DeviceAddress && self.GroupSSID == other.GroupSSID
    }
}
impl ::core::cmp::Eq for WFD_GROUP_ID {}
impl ::core::default::Default for WFD_GROUP_ID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_ASSOCIATION_ATTRIBUTES {
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dot11Bssid: [u8; 6],
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub uDot11PhyIndex: u32,
    pub wlanSignalQuality: u32,
    pub ulRxRate: u32,
    pub ulTxRate: u32,
}
impl ::core::marker::Copy for WLAN_ASSOCIATION_ATTRIBUTES {}
impl ::core::clone::Clone for WLAN_ASSOCIATION_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_ASSOCIATION_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_ASSOCIATION_ATTRIBUTES").field("dot11Ssid", &self.dot11Ssid).field("dot11BssType", &self.dot11BssType).field("dot11Bssid", &self.dot11Bssid).field("dot11PhyType", &self.dot11PhyType).field("uDot11PhyIndex", &self.uDot11PhyIndex).field("wlanSignalQuality", &self.wlanSignalQuality).field("ulRxRate", &self.ulRxRate).field("ulTxRate", &self.ulTxRate).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_ASSOCIATION_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_ASSOCIATION_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.dot11Ssid == other.dot11Ssid && self.dot11BssType == other.dot11BssType && self.dot11Bssid == other.dot11Bssid && self.dot11PhyType == other.dot11PhyType && self.uDot11PhyIndex == other.uDot11PhyIndex && self.wlanSignalQuality == other.wlanSignalQuality && self.ulRxRate == other.ulRxRate && self.ulTxRate == other.ulTxRate
    }
}
impl ::core::cmp::Eq for WLAN_ASSOCIATION_ATTRIBUTES {}
impl ::core::default::Default for WLAN_ASSOCIATION_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_AUTH_CIPHER_PAIR_LIST {
    pub dwNumberOfItems: u32,
    pub pAuthCipherPairList: [DOT11_AUTH_CIPHER_PAIR; 1],
}
impl ::core::marker::Copy for WLAN_AUTH_CIPHER_PAIR_LIST {}
impl ::core::clone::Clone for WLAN_AUTH_CIPHER_PAIR_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_AUTH_CIPHER_PAIR_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_AUTH_CIPHER_PAIR_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("pAuthCipherPairList", &self.pAuthCipherPairList).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_AUTH_CIPHER_PAIR_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_AUTH_CIPHER_PAIR_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.pAuthCipherPairList == other.pAuthCipherPairList
    }
}
impl ::core::cmp::Eq for WLAN_AUTH_CIPHER_PAIR_LIST {}
impl ::core::default::Default for WLAN_AUTH_CIPHER_PAIR_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_AVAILABLE_NETWORK {
    pub strProfileName: [u16; 256],
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub uNumberOfBssids: u32,
    pub bNetworkConnectable: super::super::Foundation::BOOL,
    pub wlanNotConnectableReason: u32,
    pub uNumberOfPhyTypes: u32,
    pub dot11PhyTypes: [DOT11_PHY_TYPE; 8],
    pub bMorePhyTypes: super::super::Foundation::BOOL,
    pub wlanSignalQuality: u32,
    pub bSecurityEnabled: super::super::Foundation::BOOL,
    pub dot11DefaultAuthAlgorithm: DOT11_AUTH_ALGORITHM,
    pub dot11DefaultCipherAlgorithm: DOT11_CIPHER_ALGORITHM,
    pub dwFlags: u32,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_AVAILABLE_NETWORK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_AVAILABLE_NETWORK {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_AVAILABLE_NETWORK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_AVAILABLE_NETWORK")
            .field("strProfileName", &self.strProfileName)
            .field("dot11Ssid", &self.dot11Ssid)
            .field("dot11BssType", &self.dot11BssType)
            .field("uNumberOfBssids", &self.uNumberOfBssids)
            .field("bNetworkConnectable", &self.bNetworkConnectable)
            .field("wlanNotConnectableReason", &self.wlanNotConnectableReason)
            .field("uNumberOfPhyTypes", &self.uNumberOfPhyTypes)
            .field("dot11PhyTypes", &self.dot11PhyTypes)
            .field("bMorePhyTypes", &self.bMorePhyTypes)
            .field("wlanSignalQuality", &self.wlanSignalQuality)
            .field("bSecurityEnabled", &self.bSecurityEnabled)
            .field("dot11DefaultAuthAlgorithm", &self.dot11DefaultAuthAlgorithm)
            .field("dot11DefaultCipherAlgorithm", &self.dot11DefaultCipherAlgorithm)
            .field("dwFlags", &self.dwFlags)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WLAN_AVAILABLE_NETWORK {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_AVAILABLE_NETWORK {
    fn eq(&self, other: &Self) -> bool {
        self.strProfileName == other.strProfileName && self.dot11Ssid == other.dot11Ssid && self.dot11BssType == other.dot11BssType && self.uNumberOfBssids == other.uNumberOfBssids && self.bNetworkConnectable == other.bNetworkConnectable && self.wlanNotConnectableReason == other.wlanNotConnectableReason && self.uNumberOfPhyTypes == other.uNumberOfPhyTypes && self.dot11PhyTypes == other.dot11PhyTypes && self.bMorePhyTypes == other.bMorePhyTypes && self.wlanSignalQuality == other.wlanSignalQuality && self.bSecurityEnabled == other.bSecurityEnabled && self.dot11DefaultAuthAlgorithm == other.dot11DefaultAuthAlgorithm && self.dot11DefaultCipherAlgorithm == other.dot11DefaultCipherAlgorithm && self.dwFlags == other.dwFlags && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_AVAILABLE_NETWORK {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_AVAILABLE_NETWORK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_AVAILABLE_NETWORK_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub Network: [WLAN_AVAILABLE_NETWORK; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_AVAILABLE_NETWORK_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_AVAILABLE_NETWORK_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_AVAILABLE_NETWORK_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_AVAILABLE_NETWORK_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("dwIndex", &self.dwIndex).field("Network", &self.Network).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WLAN_AVAILABLE_NETWORK_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_AVAILABLE_NETWORK_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.dwIndex == other.dwIndex && self.Network == other.Network
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_AVAILABLE_NETWORK_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_AVAILABLE_NETWORK_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_AVAILABLE_NETWORK_LIST_V2 {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub Network: [WLAN_AVAILABLE_NETWORK_V2; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_AVAILABLE_NETWORK_LIST_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_AVAILABLE_NETWORK_LIST_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_AVAILABLE_NETWORK_LIST_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_AVAILABLE_NETWORK_LIST_V2").field("dwNumberOfItems", &self.dwNumberOfItems).field("dwIndex", &self.dwIndex).field("Network", &self.Network).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WLAN_AVAILABLE_NETWORK_LIST_V2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_AVAILABLE_NETWORK_LIST_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.dwIndex == other.dwIndex && self.Network == other.Network
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_AVAILABLE_NETWORK_LIST_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_AVAILABLE_NETWORK_LIST_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_AVAILABLE_NETWORK_V2 {
    pub strProfileName: [u16; 256],
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub uNumberOfBssids: u32,
    pub bNetworkConnectable: super::super::Foundation::BOOL,
    pub wlanNotConnectableReason: u32,
    pub uNumberOfPhyTypes: u32,
    pub dot11PhyTypes: [DOT11_PHY_TYPE; 8],
    pub bMorePhyTypes: super::super::Foundation::BOOL,
    pub wlanSignalQuality: u32,
    pub bSecurityEnabled: super::super::Foundation::BOOL,
    pub dot11DefaultAuthAlgorithm: DOT11_AUTH_ALGORITHM,
    pub dot11DefaultCipherAlgorithm: DOT11_CIPHER_ALGORITHM,
    pub dwFlags: u32,
    pub AccessNetworkOptions: DOT11_ACCESSNETWORKOPTIONS,
    pub dot11HESSID: [u8; 6],
    pub VenueInfo: DOT11_VENUEINFO,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_AVAILABLE_NETWORK_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_AVAILABLE_NETWORK_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_AVAILABLE_NETWORK_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_AVAILABLE_NETWORK_V2")
            .field("strProfileName", &self.strProfileName)
            .field("dot11Ssid", &self.dot11Ssid)
            .field("dot11BssType", &self.dot11BssType)
            .field("uNumberOfBssids", &self.uNumberOfBssids)
            .field("bNetworkConnectable", &self.bNetworkConnectable)
            .field("wlanNotConnectableReason", &self.wlanNotConnectableReason)
            .field("uNumberOfPhyTypes", &self.uNumberOfPhyTypes)
            .field("dot11PhyTypes", &self.dot11PhyTypes)
            .field("bMorePhyTypes", &self.bMorePhyTypes)
            .field("wlanSignalQuality", &self.wlanSignalQuality)
            .field("bSecurityEnabled", &self.bSecurityEnabled)
            .field("dot11DefaultAuthAlgorithm", &self.dot11DefaultAuthAlgorithm)
            .field("dot11DefaultCipherAlgorithm", &self.dot11DefaultCipherAlgorithm)
            .field("dwFlags", &self.dwFlags)
            .field("AccessNetworkOptions", &self.AccessNetworkOptions)
            .field("dot11HESSID", &self.dot11HESSID)
            .field("VenueInfo", &self.VenueInfo)
            .field("dwReserved", &self.dwReserved)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WLAN_AVAILABLE_NETWORK_V2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_AVAILABLE_NETWORK_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.strProfileName == other.strProfileName
            && self.dot11Ssid == other.dot11Ssid
            && self.dot11BssType == other.dot11BssType
            && self.uNumberOfBssids == other.uNumberOfBssids
            && self.bNetworkConnectable == other.bNetworkConnectable
            && self.wlanNotConnectableReason == other.wlanNotConnectableReason
            && self.uNumberOfPhyTypes == other.uNumberOfPhyTypes
            && self.dot11PhyTypes == other.dot11PhyTypes
            && self.bMorePhyTypes == other.bMorePhyTypes
            && self.wlanSignalQuality == other.wlanSignalQuality
            && self.bSecurityEnabled == other.bSecurityEnabled
            && self.dot11DefaultAuthAlgorithm == other.dot11DefaultAuthAlgorithm
            && self.dot11DefaultCipherAlgorithm == other.dot11DefaultCipherAlgorithm
            && self.dwFlags == other.dwFlags
            && self.AccessNetworkOptions == other.AccessNetworkOptions
            && self.dot11HESSID == other.dot11HESSID
            && self.VenueInfo == other.VenueInfo
            && self.dwReserved == other.dwReserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_AVAILABLE_NETWORK_V2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_AVAILABLE_NETWORK_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_BSS_ENTRY {
    pub dot11Ssid: DOT11_SSID,
    pub uPhyId: u32,
    pub dot11Bssid: [u8; 6],
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dot11BssPhyType: DOT11_PHY_TYPE,
    pub lRssi: i32,
    pub uLinkQuality: u32,
    pub bInRegDomain: super::super::Foundation::BOOLEAN,
    pub usBeaconPeriod: u16,
    pub ullTimestamp: u64,
    pub ullHostTimestamp: u64,
    pub usCapabilityInformation: u16,
    pub ulChCenterFrequency: u32,
    pub wlanRateSet: WLAN_RATE_SET,
    pub ulIeOffset: u32,
    pub ulIeSize: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_BSS_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_BSS_ENTRY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_BSS_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_BSS_ENTRY")
            .field("dot11Ssid", &self.dot11Ssid)
            .field("uPhyId", &self.uPhyId)
            .field("dot11Bssid", &self.dot11Bssid)
            .field("dot11BssType", &self.dot11BssType)
            .field("dot11BssPhyType", &self.dot11BssPhyType)
            .field("lRssi", &self.lRssi)
            .field("uLinkQuality", &self.uLinkQuality)
            .field("bInRegDomain", &self.bInRegDomain)
            .field("usBeaconPeriod", &self.usBeaconPeriod)
            .field("ullTimestamp", &self.ullTimestamp)
            .field("ullHostTimestamp", &self.ullHostTimestamp)
            .field("usCapabilityInformation", &self.usCapabilityInformation)
            .field("ulChCenterFrequency", &self.ulChCenterFrequency)
            .field("wlanRateSet", &self.wlanRateSet)
            .field("ulIeOffset", &self.ulIeOffset)
            .field("ulIeSize", &self.ulIeSize)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WLAN_BSS_ENTRY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_BSS_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.dot11Ssid == other.dot11Ssid && self.uPhyId == other.uPhyId && self.dot11Bssid == other.dot11Bssid && self.dot11BssType == other.dot11BssType && self.dot11BssPhyType == other.dot11BssPhyType && self.lRssi == other.lRssi && self.uLinkQuality == other.uLinkQuality && self.bInRegDomain == other.bInRegDomain && self.usBeaconPeriod == other.usBeaconPeriod && self.ullTimestamp == other.ullTimestamp && self.ullHostTimestamp == other.ullHostTimestamp && self.usCapabilityInformation == other.usCapabilityInformation && self.ulChCenterFrequency == other.ulChCenterFrequency && self.wlanRateSet == other.wlanRateSet && self.ulIeOffset == other.ulIeOffset && self.ulIeSize == other.ulIeSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_BSS_ENTRY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_BSS_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_BSS_LIST {
    pub dwTotalSize: u32,
    pub dwNumberOfItems: u32,
    pub wlanBssEntries: [WLAN_BSS_ENTRY; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_BSS_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_BSS_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_BSS_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_BSS_LIST").field("dwTotalSize", &self.dwTotalSize).field("dwNumberOfItems", &self.dwNumberOfItems).field("wlanBssEntries", &self.wlanBssEntries).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WLAN_BSS_LIST {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_BSS_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwTotalSize == other.dwTotalSize && self.dwNumberOfItems == other.dwNumberOfItems && self.wlanBssEntries == other.wlanBssEntries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_BSS_LIST {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_BSS_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_CONNECTION_ATTRIBUTES {
    pub isState: WLAN_INTERFACE_STATE,
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfileName: [u16; 256],
    pub wlanAssociationAttributes: WLAN_ASSOCIATION_ATTRIBUTES,
    pub wlanSecurityAttributes: WLAN_SECURITY_ATTRIBUTES,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_CONNECTION_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_CONNECTION_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_CONNECTION_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_CONNECTION_ATTRIBUTES").field("isState", &self.isState).field("wlanConnectionMode", &self.wlanConnectionMode).field("strProfileName", &self.strProfileName).field("wlanAssociationAttributes", &self.wlanAssociationAttributes).field("wlanSecurityAttributes", &self.wlanSecurityAttributes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WLAN_CONNECTION_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_CONNECTION_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.isState == other.isState && self.wlanConnectionMode == other.wlanConnectionMode && self.strProfileName == other.strProfileName && self.wlanAssociationAttributes == other.wlanAssociationAttributes && self.wlanSecurityAttributes == other.wlanSecurityAttributes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_CONNECTION_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_CONNECTION_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_CONNECTION_NOTIFICATION_DATA {
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfileName: [u16; 256],
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub bSecurityEnabled: super::super::Foundation::BOOL,
    pub wlanReasonCode: u32,
    pub dwFlags: WLAN_CONNECTION_NOTIFICATION_FLAGS,
    pub strProfileXml: [u16; 1],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_CONNECTION_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_CONNECTION_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_CONNECTION_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_CONNECTION_NOTIFICATION_DATA").field("wlanConnectionMode", &self.wlanConnectionMode).field("strProfileName", &self.strProfileName).field("dot11Ssid", &self.dot11Ssid).field("dot11BssType", &self.dot11BssType).field("bSecurityEnabled", &self.bSecurityEnabled).field("wlanReasonCode", &self.wlanReasonCode).field("dwFlags", &self.dwFlags).field("strProfileXml", &self.strProfileXml).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WLAN_CONNECTION_NOTIFICATION_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_CONNECTION_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wlanConnectionMode == other.wlanConnectionMode && self.strProfileName == other.strProfileName && self.dot11Ssid == other.dot11Ssid && self.dot11BssType == other.dot11BssType && self.bSecurityEnabled == other.bSecurityEnabled && self.wlanReasonCode == other.wlanReasonCode && self.dwFlags == other.dwFlags && self.strProfileXml == other.strProfileXml
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_CONNECTION_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_CONNECTION_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct WLAN_CONNECTION_PARAMETERS {
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfile: ::windows::core::PCWSTR,
    pub pDot11Ssid: *mut DOT11_SSID,
    pub pDesiredBssidList: *mut DOT11_BSSID_LIST,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dwFlags: u32,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for WLAN_CONNECTION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for WLAN_CONNECTION_PARAMETERS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for WLAN_CONNECTION_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_CONNECTION_PARAMETERS").field("wlanConnectionMode", &self.wlanConnectionMode).field("strProfile", &self.strProfile).field("pDot11Ssid", &self.pDot11Ssid).field("pDesiredBssidList", &self.pDesiredBssidList).field("dot11BssType", &self.dot11BssType).field("dwFlags", &self.dwFlags).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for WLAN_CONNECTION_PARAMETERS {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for WLAN_CONNECTION_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.wlanConnectionMode == other.wlanConnectionMode && self.strProfile == other.strProfile && self.pDot11Ssid == other.pDot11Ssid && self.pDesiredBssidList == other.pDesiredBssidList && self.dot11BssType == other.dot11BssType && self.dwFlags == other.dwFlags
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for WLAN_CONNECTION_PARAMETERS {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for WLAN_CONNECTION_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
pub struct WLAN_CONNECTION_PARAMETERS_V2 {
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfile: ::windows::core::PCWSTR,
    pub pDot11Ssid: *mut DOT11_SSID,
    pub pDot11Hessid: *mut u8,
    pub pDesiredBssidList: *mut DOT11_BSSID_LIST,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dwFlags: u32,
    pub pDot11AccessNetworkOptions: *mut DOT11_ACCESSNETWORKOPTIONS,
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::marker::Copy for WLAN_CONNECTION_PARAMETERS_V2 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::clone::Clone for WLAN_CONNECTION_PARAMETERS_V2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::fmt::Debug for WLAN_CONNECTION_PARAMETERS_V2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_CONNECTION_PARAMETERS_V2").field("wlanConnectionMode", &self.wlanConnectionMode).field("strProfile", &self.strProfile).field("pDot11Ssid", &self.pDot11Ssid).field("pDot11Hessid", &self.pDot11Hessid).field("pDesiredBssidList", &self.pDesiredBssidList).field("dot11BssType", &self.dot11BssType).field("dwFlags", &self.dwFlags).field("pDot11AccessNetworkOptions", &self.pDot11AccessNetworkOptions).finish()
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::windows::core::TypeKind for WLAN_CONNECTION_PARAMETERS_V2 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::PartialEq for WLAN_CONNECTION_PARAMETERS_V2 {
    fn eq(&self, other: &Self) -> bool {
        self.wlanConnectionMode == other.wlanConnectionMode && self.strProfile == other.strProfile && self.pDot11Ssid == other.pDot11Ssid && self.pDot11Hessid == other.pDot11Hessid && self.pDesiredBssidList == other.pDesiredBssidList && self.dot11BssType == other.dot11BssType && self.dwFlags == other.dwFlags && self.pDot11AccessNetworkOptions == other.pDot11AccessNetworkOptions
    }
}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::cmp::Eq for WLAN_CONNECTION_PARAMETERS_V2 {}
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
impl ::core::default::Default for WLAN_CONNECTION_PARAMETERS_V2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_COUNTRY_OR_REGION_STRING_LIST {
    pub dwNumberOfItems: u32,
    pub pCountryOrRegionStringList: [u8; 3],
}
impl ::core::marker::Copy for WLAN_COUNTRY_OR_REGION_STRING_LIST {}
impl ::core::clone::Clone for WLAN_COUNTRY_OR_REGION_STRING_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_COUNTRY_OR_REGION_STRING_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_COUNTRY_OR_REGION_STRING_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("pCountryOrRegionStringList", &self.pCountryOrRegionStringList).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_COUNTRY_OR_REGION_STRING_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_COUNTRY_OR_REGION_STRING_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.pCountryOrRegionStringList == other.pCountryOrRegionStringList
    }
}
impl ::core::cmp::Eq for WLAN_COUNTRY_OR_REGION_STRING_LIST {}
impl ::core::default::Default for WLAN_COUNTRY_OR_REGION_STRING_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_DEVICE_SERVICE_GUID_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub DeviceService: [::windows::core::GUID; 1],
}
impl ::core::marker::Copy for WLAN_DEVICE_SERVICE_GUID_LIST {}
impl ::core::clone::Clone for WLAN_DEVICE_SERVICE_GUID_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_DEVICE_SERVICE_GUID_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_DEVICE_SERVICE_GUID_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("dwIndex", &self.dwIndex).field("DeviceService", &self.DeviceService).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_DEVICE_SERVICE_GUID_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_DEVICE_SERVICE_GUID_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.dwIndex == other.dwIndex && self.DeviceService == other.DeviceService
    }
}
impl ::core::cmp::Eq for WLAN_DEVICE_SERVICE_GUID_LIST {}
impl ::core::default::Default for WLAN_DEVICE_SERVICE_GUID_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    pub DeviceService: ::windows::core::GUID,
    pub dwOpCode: u32,
    pub dwDataSize: u32,
    pub DataBlob: [u8; 1],
}
impl ::core::marker::Copy for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {}
impl ::core::clone::Clone for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_DEVICE_SERVICE_NOTIFICATION_DATA").field("DeviceService", &self.DeviceService).field("dwOpCode", &self.dwOpCode).field("dwDataSize", &self.dwDataSize).field("DataBlob", &self.DataBlob).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceService == other.DeviceService && self.dwOpCode == other.dwOpCode && self.dwDataSize == other.dwDataSize && self.DataBlob == other.DataBlob
    }
}
impl ::core::cmp::Eq for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {}
impl ::core::default::Default for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    pub hostedNetworkSSID: DOT11_SSID,
    pub dwMaxNumberOfPeers: u32,
}
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS").field("hostedNetworkSSID", &self.hostedNetworkSSID).field("dwMaxNumberOfPeers", &self.dwMaxNumberOfPeers).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.hostedNetworkSSID == other.hostedNetworkSSID && self.dwMaxNumberOfPeers == other.dwMaxNumberOfPeers
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    pub OldState: WLAN_HOSTED_NETWORK_PEER_STATE,
    pub NewState: WLAN_HOSTED_NETWORK_PEER_STATE,
    pub PeerStateChangeReason: WLAN_HOSTED_NETWORK_REASON,
}
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE").field("OldState", &self.OldState).field("NewState", &self.NewState).field("PeerStateChangeReason", &self.PeerStateChangeReason).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.OldState == other.OldState && self.NewState == other.NewState && self.PeerStateChangeReason == other.PeerStateChangeReason
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_HOSTED_NETWORK_PEER_STATE {
    pub PeerMacAddress: [u8; 6],
    pub PeerAuthState: WLAN_HOSTED_NETWORK_PEER_AUTH_STATE,
}
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_PEER_STATE {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_PEER_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_PEER_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_HOSTED_NETWORK_PEER_STATE").field("PeerMacAddress", &self.PeerMacAddress).field("PeerAuthState", &self.PeerAuthState).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_HOSTED_NETWORK_PEER_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_PEER_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.PeerMacAddress == other.PeerMacAddress && self.PeerAuthState == other.PeerAuthState
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_PEER_STATE {}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_PEER_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_HOSTED_NETWORK_RADIO_STATE {
    pub dot11SoftwareRadioState: DOT11_RADIO_STATE,
    pub dot11HardwareRadioState: DOT11_RADIO_STATE,
}
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_RADIO_STATE {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_RADIO_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_RADIO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_HOSTED_NETWORK_RADIO_STATE").field("dot11SoftwareRadioState", &self.dot11SoftwareRadioState).field("dot11HardwareRadioState", &self.dot11HardwareRadioState).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_HOSTED_NETWORK_RADIO_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_RADIO_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.dot11SoftwareRadioState == other.dot11SoftwareRadioState && self.dot11HardwareRadioState == other.dot11HardwareRadioState
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_RADIO_STATE {}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_RADIO_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    pub dot11AuthAlgo: DOT11_AUTH_ALGORITHM,
    pub dot11CipherAlgo: DOT11_CIPHER_ALGORITHM,
}
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_HOSTED_NETWORK_SECURITY_SETTINGS").field("dot11AuthAlgo", &self.dot11AuthAlgo).field("dot11CipherAlgo", &self.dot11CipherAlgo).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.dot11AuthAlgo == other.dot11AuthAlgo && self.dot11CipherAlgo == other.dot11CipherAlgo
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_HOSTED_NETWORK_STATE_CHANGE {
    pub OldState: WLAN_HOSTED_NETWORK_STATE,
    pub NewState: WLAN_HOSTED_NETWORK_STATE,
    pub StateChangeReason: WLAN_HOSTED_NETWORK_REASON,
}
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_STATE_CHANGE {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_STATE_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_STATE_CHANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_HOSTED_NETWORK_STATE_CHANGE").field("OldState", &self.OldState).field("NewState", &self.NewState).field("StateChangeReason", &self.StateChangeReason).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_HOSTED_NETWORK_STATE_CHANGE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_STATE_CHANGE {
    fn eq(&self, other: &Self) -> bool {
        self.OldState == other.OldState && self.NewState == other.NewState && self.StateChangeReason == other.StateChangeReason
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_STATE_CHANGE {}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_STATE_CHANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_HOSTED_NETWORK_STATUS {
    pub HostedNetworkState: WLAN_HOSTED_NETWORK_STATE,
    pub IPDeviceID: ::windows::core::GUID,
    pub wlanHostedNetworkBSSID: [u8; 6],
    pub dot11PhyType: DOT11_PHY_TYPE,
    pub ulChannelFrequency: u32,
    pub dwNumberOfPeers: u32,
    pub PeerList: [WLAN_HOSTED_NETWORK_PEER_STATE; 1],
}
impl ::core::marker::Copy for WLAN_HOSTED_NETWORK_STATUS {}
impl ::core::clone::Clone for WLAN_HOSTED_NETWORK_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_HOSTED_NETWORK_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_HOSTED_NETWORK_STATUS").field("HostedNetworkState", &self.HostedNetworkState).field("IPDeviceID", &self.IPDeviceID).field("wlanHostedNetworkBSSID", &self.wlanHostedNetworkBSSID).field("dot11PhyType", &self.dot11PhyType).field("ulChannelFrequency", &self.ulChannelFrequency).field("dwNumberOfPeers", &self.dwNumberOfPeers).field("PeerList", &self.PeerList).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_HOSTED_NETWORK_STATUS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_HOSTED_NETWORK_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.HostedNetworkState == other.HostedNetworkState && self.IPDeviceID == other.IPDeviceID && self.wlanHostedNetworkBSSID == other.wlanHostedNetworkBSSID && self.dot11PhyType == other.dot11PhyType && self.ulChannelFrequency == other.ulChannelFrequency && self.dwNumberOfPeers == other.dwNumberOfPeers && self.PeerList == other.PeerList
    }
}
impl ::core::cmp::Eq for WLAN_HOSTED_NETWORK_STATUS {}
impl ::core::default::Default for WLAN_HOSTED_NETWORK_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_INTERFACE_CAPABILITY {
    pub interfaceType: WLAN_INTERFACE_TYPE,
    pub bDot11DSupported: super::super::Foundation::BOOL,
    pub dwMaxDesiredSsidListSize: u32,
    pub dwMaxDesiredBssidListSize: u32,
    pub dwNumberOfSupportedPhys: u32,
    pub dot11PhyTypes: [DOT11_PHY_TYPE; 64],
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_INTERFACE_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_INTERFACE_CAPABILITY {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_INTERFACE_CAPABILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_INTERFACE_CAPABILITY").field("interfaceType", &self.interfaceType).field("bDot11DSupported", &self.bDot11DSupported).field("dwMaxDesiredSsidListSize", &self.dwMaxDesiredSsidListSize).field("dwMaxDesiredBssidListSize", &self.dwMaxDesiredBssidListSize).field("dwNumberOfSupportedPhys", &self.dwNumberOfSupportedPhys).field("dot11PhyTypes", &self.dot11PhyTypes).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WLAN_INTERFACE_CAPABILITY {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_INTERFACE_CAPABILITY {
    fn eq(&self, other: &Self) -> bool {
        self.interfaceType == other.interfaceType && self.bDot11DSupported == other.bDot11DSupported && self.dwMaxDesiredSsidListSize == other.dwMaxDesiredSsidListSize && self.dwMaxDesiredBssidListSize == other.dwMaxDesiredBssidListSize && self.dwNumberOfSupportedPhys == other.dwNumberOfSupportedPhys && self.dot11PhyTypes == other.dot11PhyTypes
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_INTERFACE_CAPABILITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_INTERFACE_CAPABILITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_INTERFACE_INFO {
    pub InterfaceGuid: ::windows::core::GUID,
    pub strInterfaceDescription: [u16; 256],
    pub isState: WLAN_INTERFACE_STATE,
}
impl ::core::marker::Copy for WLAN_INTERFACE_INFO {}
impl ::core::clone::Clone for WLAN_INTERFACE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_INTERFACE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_INTERFACE_INFO").field("InterfaceGuid", &self.InterfaceGuid).field("strInterfaceDescription", &self.strInterfaceDescription).field("isState", &self.isState).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_INTERFACE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_INTERFACE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.InterfaceGuid == other.InterfaceGuid && self.strInterfaceDescription == other.strInterfaceDescription && self.isState == other.isState
    }
}
impl ::core::cmp::Eq for WLAN_INTERFACE_INFO {}
impl ::core::default::Default for WLAN_INTERFACE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_INTERFACE_INFO_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub InterfaceInfo: [WLAN_INTERFACE_INFO; 1],
}
impl ::core::marker::Copy for WLAN_INTERFACE_INFO_LIST {}
impl ::core::clone::Clone for WLAN_INTERFACE_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_INTERFACE_INFO_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_INTERFACE_INFO_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("dwIndex", &self.dwIndex).field("InterfaceInfo", &self.InterfaceInfo).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_INTERFACE_INFO_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_INTERFACE_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.dwIndex == other.dwIndex && self.InterfaceInfo == other.InterfaceInfo
    }
}
impl ::core::cmp::Eq for WLAN_INTERFACE_INFO_LIST {}
impl ::core::default::Default for WLAN_INTERFACE_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_MAC_FRAME_STATISTICS {
    pub ullTransmittedFrameCount: u64,
    pub ullReceivedFrameCount: u64,
    pub ullWEPExcludedCount: u64,
    pub ullTKIPLocalMICFailures: u64,
    pub ullTKIPReplays: u64,
    pub ullTKIPICVErrorCount: u64,
    pub ullCCMPReplays: u64,
    pub ullCCMPDecryptErrors: u64,
    pub ullWEPUndecryptableCount: u64,
    pub ullWEPICVErrorCount: u64,
    pub ullDecryptSuccessCount: u64,
    pub ullDecryptFailureCount: u64,
}
impl ::core::marker::Copy for WLAN_MAC_FRAME_STATISTICS {}
impl ::core::clone::Clone for WLAN_MAC_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_MAC_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_MAC_FRAME_STATISTICS")
            .field("ullTransmittedFrameCount", &self.ullTransmittedFrameCount)
            .field("ullReceivedFrameCount", &self.ullReceivedFrameCount)
            .field("ullWEPExcludedCount", &self.ullWEPExcludedCount)
            .field("ullTKIPLocalMICFailures", &self.ullTKIPLocalMICFailures)
            .field("ullTKIPReplays", &self.ullTKIPReplays)
            .field("ullTKIPICVErrorCount", &self.ullTKIPICVErrorCount)
            .field("ullCCMPReplays", &self.ullCCMPReplays)
            .field("ullCCMPDecryptErrors", &self.ullCCMPDecryptErrors)
            .field("ullWEPUndecryptableCount", &self.ullWEPUndecryptableCount)
            .field("ullWEPICVErrorCount", &self.ullWEPICVErrorCount)
            .field("ullDecryptSuccessCount", &self.ullDecryptSuccessCount)
            .field("ullDecryptFailureCount", &self.ullDecryptFailureCount)
            .finish()
    }
}
impl ::windows::core::TypeKind for WLAN_MAC_FRAME_STATISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_MAC_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.ullTransmittedFrameCount == other.ullTransmittedFrameCount && self.ullReceivedFrameCount == other.ullReceivedFrameCount && self.ullWEPExcludedCount == other.ullWEPExcludedCount && self.ullTKIPLocalMICFailures == other.ullTKIPLocalMICFailures && self.ullTKIPReplays == other.ullTKIPReplays && self.ullTKIPICVErrorCount == other.ullTKIPICVErrorCount && self.ullCCMPReplays == other.ullCCMPReplays && self.ullCCMPDecryptErrors == other.ullCCMPDecryptErrors && self.ullWEPUndecryptableCount == other.ullWEPUndecryptableCount && self.ullWEPICVErrorCount == other.ullWEPICVErrorCount && self.ullDecryptSuccessCount == other.ullDecryptSuccessCount && self.ullDecryptFailureCount == other.ullDecryptFailureCount
    }
}
impl ::core::cmp::Eq for WLAN_MAC_FRAME_STATISTICS {}
impl ::core::default::Default for WLAN_MAC_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_MSM_NOTIFICATION_DATA {
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfileName: [u16; 256],
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dot11MacAddr: [u8; 6],
    pub bSecurityEnabled: super::super::Foundation::BOOL,
    pub bFirstPeer: super::super::Foundation::BOOL,
    pub bLastPeer: super::super::Foundation::BOOL,
    pub wlanReasonCode: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_MSM_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_MSM_NOTIFICATION_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_MSM_NOTIFICATION_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_MSM_NOTIFICATION_DATA").field("wlanConnectionMode", &self.wlanConnectionMode).field("strProfileName", &self.strProfileName).field("dot11Ssid", &self.dot11Ssid).field("dot11BssType", &self.dot11BssType).field("dot11MacAddr", &self.dot11MacAddr).field("bSecurityEnabled", &self.bSecurityEnabled).field("bFirstPeer", &self.bFirstPeer).field("bLastPeer", &self.bLastPeer).field("wlanReasonCode", &self.wlanReasonCode).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WLAN_MSM_NOTIFICATION_DATA {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_MSM_NOTIFICATION_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.wlanConnectionMode == other.wlanConnectionMode && self.strProfileName == other.strProfileName && self.dot11Ssid == other.dot11Ssid && self.dot11BssType == other.dot11BssType && self.dot11MacAddr == other.dot11MacAddr && self.bSecurityEnabled == other.bSecurityEnabled && self.bFirstPeer == other.bFirstPeer && self.bLastPeer == other.bLastPeer && self.wlanReasonCode == other.wlanReasonCode
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_MSM_NOTIFICATION_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_MSM_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_PHY_FRAME_STATISTICS {
    pub ullTransmittedFrameCount: u64,
    pub ullMulticastTransmittedFrameCount: u64,
    pub ullFailedCount: u64,
    pub ullRetryCount: u64,
    pub ullMultipleRetryCount: u64,
    pub ullMaxTXLifetimeExceededCount: u64,
    pub ullTransmittedFragmentCount: u64,
    pub ullRTSSuccessCount: u64,
    pub ullRTSFailureCount: u64,
    pub ullACKFailureCount: u64,
    pub ullReceivedFrameCount: u64,
    pub ullMulticastReceivedFrameCount: u64,
    pub ullPromiscuousReceivedFrameCount: u64,
    pub ullMaxRXLifetimeExceededCount: u64,
    pub ullFrameDuplicateCount: u64,
    pub ullReceivedFragmentCount: u64,
    pub ullPromiscuousReceivedFragmentCount: u64,
    pub ullFCSErrorCount: u64,
}
impl ::core::marker::Copy for WLAN_PHY_FRAME_STATISTICS {}
impl ::core::clone::Clone for WLAN_PHY_FRAME_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_PHY_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_PHY_FRAME_STATISTICS")
            .field("ullTransmittedFrameCount", &self.ullTransmittedFrameCount)
            .field("ullMulticastTransmittedFrameCount", &self.ullMulticastTransmittedFrameCount)
            .field("ullFailedCount", &self.ullFailedCount)
            .field("ullRetryCount", &self.ullRetryCount)
            .field("ullMultipleRetryCount", &self.ullMultipleRetryCount)
            .field("ullMaxTXLifetimeExceededCount", &self.ullMaxTXLifetimeExceededCount)
            .field("ullTransmittedFragmentCount", &self.ullTransmittedFragmentCount)
            .field("ullRTSSuccessCount", &self.ullRTSSuccessCount)
            .field("ullRTSFailureCount", &self.ullRTSFailureCount)
            .field("ullACKFailureCount", &self.ullACKFailureCount)
            .field("ullReceivedFrameCount", &self.ullReceivedFrameCount)
            .field("ullMulticastReceivedFrameCount", &self.ullMulticastReceivedFrameCount)
            .field("ullPromiscuousReceivedFrameCount", &self.ullPromiscuousReceivedFrameCount)
            .field("ullMaxRXLifetimeExceededCount", &self.ullMaxRXLifetimeExceededCount)
            .field("ullFrameDuplicateCount", &self.ullFrameDuplicateCount)
            .field("ullReceivedFragmentCount", &self.ullReceivedFragmentCount)
            .field("ullPromiscuousReceivedFragmentCount", &self.ullPromiscuousReceivedFragmentCount)
            .field("ullFCSErrorCount", &self.ullFCSErrorCount)
            .finish()
    }
}
impl ::windows::core::TypeKind for WLAN_PHY_FRAME_STATISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_PHY_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.ullTransmittedFrameCount == other.ullTransmittedFrameCount
            && self.ullMulticastTransmittedFrameCount == other.ullMulticastTransmittedFrameCount
            && self.ullFailedCount == other.ullFailedCount
            && self.ullRetryCount == other.ullRetryCount
            && self.ullMultipleRetryCount == other.ullMultipleRetryCount
            && self.ullMaxTXLifetimeExceededCount == other.ullMaxTXLifetimeExceededCount
            && self.ullTransmittedFragmentCount == other.ullTransmittedFragmentCount
            && self.ullRTSSuccessCount == other.ullRTSSuccessCount
            && self.ullRTSFailureCount == other.ullRTSFailureCount
            && self.ullACKFailureCount == other.ullACKFailureCount
            && self.ullReceivedFrameCount == other.ullReceivedFrameCount
            && self.ullMulticastReceivedFrameCount == other.ullMulticastReceivedFrameCount
            && self.ullPromiscuousReceivedFrameCount == other.ullPromiscuousReceivedFrameCount
            && self.ullMaxRXLifetimeExceededCount == other.ullMaxRXLifetimeExceededCount
            && self.ullFrameDuplicateCount == other.ullFrameDuplicateCount
            && self.ullReceivedFragmentCount == other.ullReceivedFragmentCount
            && self.ullPromiscuousReceivedFragmentCount == other.ullPromiscuousReceivedFragmentCount
            && self.ullFCSErrorCount == other.ullFCSErrorCount
    }
}
impl ::core::cmp::Eq for WLAN_PHY_FRAME_STATISTICS {}
impl ::core::default::Default for WLAN_PHY_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_PHY_RADIO_STATE {
    pub dwPhyIndex: u32,
    pub dot11SoftwareRadioState: DOT11_RADIO_STATE,
    pub dot11HardwareRadioState: DOT11_RADIO_STATE,
}
impl ::core::marker::Copy for WLAN_PHY_RADIO_STATE {}
impl ::core::clone::Clone for WLAN_PHY_RADIO_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_PHY_RADIO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_PHY_RADIO_STATE").field("dwPhyIndex", &self.dwPhyIndex).field("dot11SoftwareRadioState", &self.dot11SoftwareRadioState).field("dot11HardwareRadioState", &self.dot11HardwareRadioState).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_PHY_RADIO_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_PHY_RADIO_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.dwPhyIndex == other.dwPhyIndex && self.dot11SoftwareRadioState == other.dot11SoftwareRadioState && self.dot11HardwareRadioState == other.dot11HardwareRadioState
    }
}
impl ::core::cmp::Eq for WLAN_PHY_RADIO_STATE {}
impl ::core::default::Default for WLAN_PHY_RADIO_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_PROFILE_INFO {
    pub strProfileName: [u16; 256],
    pub dwFlags: u32,
}
impl ::core::marker::Copy for WLAN_PROFILE_INFO {}
impl ::core::clone::Clone for WLAN_PROFILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_PROFILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_PROFILE_INFO").field("strProfileName", &self.strProfileName).field("dwFlags", &self.dwFlags).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_PROFILE_INFO {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.strProfileName == other.strProfileName && self.dwFlags == other.dwFlags
    }
}
impl ::core::cmp::Eq for WLAN_PROFILE_INFO {}
impl ::core::default::Default for WLAN_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_PROFILE_INFO_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub ProfileInfo: [WLAN_PROFILE_INFO; 1],
}
impl ::core::marker::Copy for WLAN_PROFILE_INFO_LIST {}
impl ::core::clone::Clone for WLAN_PROFILE_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_PROFILE_INFO_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_PROFILE_INFO_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("dwIndex", &self.dwIndex).field("ProfileInfo", &self.ProfileInfo).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_PROFILE_INFO_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_PROFILE_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.dwIndex == other.dwIndex && self.ProfileInfo == other.ProfileInfo
    }
}
impl ::core::cmp::Eq for WLAN_PROFILE_INFO_LIST {}
impl ::core::default::Default for WLAN_PROFILE_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_RADIO_STATE {
    pub dwNumberOfPhys: u32,
    pub PhyRadioState: [WLAN_PHY_RADIO_STATE; 64],
}
impl ::core::marker::Copy for WLAN_RADIO_STATE {}
impl ::core::clone::Clone for WLAN_RADIO_STATE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_RADIO_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_RADIO_STATE").field("dwNumberOfPhys", &self.dwNumberOfPhys).field("PhyRadioState", &self.PhyRadioState).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_RADIO_STATE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_RADIO_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfPhys == other.dwNumberOfPhys && self.PhyRadioState == other.PhyRadioState
    }
}
impl ::core::cmp::Eq for WLAN_RADIO_STATE {}
impl ::core::default::Default for WLAN_RADIO_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_RATE_SET {
    pub uRateSetLength: u32,
    pub usRateSet: [u16; 126],
}
impl ::core::marker::Copy for WLAN_RATE_SET {}
impl ::core::clone::Clone for WLAN_RATE_SET {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_RATE_SET {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_RATE_SET").field("uRateSetLength", &self.uRateSetLength).field("usRateSet", &self.usRateSet).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_RATE_SET {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_RATE_SET {
    fn eq(&self, other: &Self) -> bool {
        self.uRateSetLength == other.uRateSetLength && self.usRateSet == other.usRateSet
    }
}
impl ::core::cmp::Eq for WLAN_RATE_SET {}
impl ::core::default::Default for WLAN_RATE_SET {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_RAW_DATA {
    pub dwDataSize: u32,
    pub DataBlob: [u8; 1],
}
impl ::core::marker::Copy for WLAN_RAW_DATA {}
impl ::core::clone::Clone for WLAN_RAW_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_RAW_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_RAW_DATA").field("dwDataSize", &self.dwDataSize).field("DataBlob", &self.DataBlob).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_RAW_DATA {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_RAW_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.dwDataSize == other.dwDataSize && self.DataBlob == other.DataBlob
    }
}
impl ::core::cmp::Eq for WLAN_RAW_DATA {}
impl ::core::default::Default for WLAN_RAW_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_RAW_DATA_LIST {
    pub dwTotalSize: u32,
    pub dwNumberOfItems: u32,
    pub DataList: [WLAN_RAW_DATA_LIST_0; 1],
}
impl ::core::marker::Copy for WLAN_RAW_DATA_LIST {}
impl ::core::clone::Clone for WLAN_RAW_DATA_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_RAW_DATA_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_RAW_DATA_LIST").field("dwTotalSize", &self.dwTotalSize).field("dwNumberOfItems", &self.dwNumberOfItems).field("DataList", &self.DataList).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_RAW_DATA_LIST {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_RAW_DATA_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwTotalSize == other.dwTotalSize && self.dwNumberOfItems == other.dwNumberOfItems && self.DataList == other.DataList
    }
}
impl ::core::cmp::Eq for WLAN_RAW_DATA_LIST {}
impl ::core::default::Default for WLAN_RAW_DATA_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_RAW_DATA_LIST_0 {
    pub dwDataOffset: u32,
    pub dwDataSize: u32,
}
impl ::core::marker::Copy for WLAN_RAW_DATA_LIST_0 {}
impl ::core::clone::Clone for WLAN_RAW_DATA_LIST_0 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_RAW_DATA_LIST_0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_RAW_DATA_LIST_0").field("dwDataOffset", &self.dwDataOffset).field("dwDataSize", &self.dwDataSize).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_RAW_DATA_LIST_0 {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_RAW_DATA_LIST_0 {
    fn eq(&self, other: &Self) -> bool {
        self.dwDataOffset == other.dwDataOffset && self.dwDataSize == other.dwDataSize
    }
}
impl ::core::cmp::Eq for WLAN_RAW_DATA_LIST_0 {}
impl ::core::default::Default for WLAN_RAW_DATA_LIST_0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WLAN_SECURITY_ATTRIBUTES {
    pub bSecurityEnabled: super::super::Foundation::BOOL,
    pub bOneXEnabled: super::super::Foundation::BOOL,
    pub dot11AuthAlgorithm: DOT11_AUTH_ALGORITHM,
    pub dot11CipherAlgorithm: DOT11_CIPHER_ALGORITHM,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WLAN_SECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WLAN_SECURITY_ATTRIBUTES {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WLAN_SECURITY_ATTRIBUTES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_SECURITY_ATTRIBUTES").field("bSecurityEnabled", &self.bSecurityEnabled).field("bOneXEnabled", &self.bOneXEnabled).field("dot11AuthAlgorithm", &self.dot11AuthAlgorithm).field("dot11CipherAlgorithm", &self.dot11CipherAlgorithm).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for WLAN_SECURITY_ATTRIBUTES {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WLAN_SECURITY_ATTRIBUTES {
    fn eq(&self, other: &Self) -> bool {
        self.bSecurityEnabled == other.bSecurityEnabled && self.bOneXEnabled == other.bOneXEnabled && self.dot11AuthAlgorithm == other.dot11AuthAlgorithm && self.dot11CipherAlgorithm == other.dot11CipherAlgorithm
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WLAN_SECURITY_ATTRIBUTES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WLAN_SECURITY_ATTRIBUTES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub struct WLAN_STATISTICS {
    pub ullFourWayHandshakeFailures: u64,
    pub ullTKIPCounterMeasuresInvoked: u64,
    pub ullReserved: u64,
    pub MacUcastCounters: WLAN_MAC_FRAME_STATISTICS,
    pub MacMcastCounters: WLAN_MAC_FRAME_STATISTICS,
    pub dwNumberOfPhys: u32,
    pub PhyCounters: [WLAN_PHY_FRAME_STATISTICS; 1],
}
impl ::core::marker::Copy for WLAN_STATISTICS {}
impl ::core::clone::Clone for WLAN_STATISTICS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WLAN_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WLAN_STATISTICS").field("ullFourWayHandshakeFailures", &self.ullFourWayHandshakeFailures).field("ullTKIPCounterMeasuresInvoked", &self.ullTKIPCounterMeasuresInvoked).field("ullReserved", &self.ullReserved).field("MacUcastCounters", &self.MacUcastCounters).field("MacMcastCounters", &self.MacMcastCounters).field("dwNumberOfPhys", &self.dwNumberOfPhys).field("PhyCounters", &self.PhyCounters).finish()
    }
}
impl ::windows::core::TypeKind for WLAN_STATISTICS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for WLAN_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.ullFourWayHandshakeFailures == other.ullFourWayHandshakeFailures && self.ullTKIPCounterMeasuresInvoked == other.ullTKIPCounterMeasuresInvoked && self.ullReserved == other.ullReserved && self.MacUcastCounters == other.MacUcastCounters && self.MacMcastCounters == other.MacMcastCounters && self.dwNumberOfPhys == other.dwNumberOfPhys && self.PhyCounters == other.PhyCounters
    }
}
impl ::core::cmp::Eq for WLAN_STATISTICS {}
impl ::core::default::Default for WLAN_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_ADAPTER_RESET = ::core::option::Option<unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_CONTROL = ::core::option::Option<unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, dwinbuffersize: u32, pinbuffer: *const u8, dwoutbuffersize: u32, poutbuffer: *mut u8, pdwbytesreturned: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub type DOT11EXTIHV_CREATE_DISCOVERY_PROFILES = ::core::option::Option<unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, binsecure: super::super::Foundation::BOOL, pihvprofileparams: *const DOT11EXT_IHV_PROFILE_PARAMS, pconnectablebssid: *const DOT11_BSS_LIST, pihvdiscoveryprofilelist: *mut DOT11EXT_IHV_DISCOVERY_PROFILE_LIST, pdwreasoncode: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_DEINIT_ADAPTER = ::core::option::Option<unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE) -> ()>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub type DOT11EXTIHV_DEINIT_SERVICE = ::core::option::Option<unsafe extern "system" fn() -> ()>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub type DOT11EXTIHV_GET_VERSION_INFO = ::core::option::Option<unsafe extern "system" fn(pdot11ihvversioninfo: *mut DOT11_IHV_VERSION_INFO) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_INIT_ADAPTER = ::core::option::Option<unsafe extern "system" fn(pdot11adapter: *const DOT11_ADAPTER, hdot11svchandle: super::super::Foundation::HANDLE, phihvextadapter: *mut super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`, `\"Win32_System_RemoteDesktop\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis", feature = "Win32_Security_ExtensibleAuthenticationProtocol", feature = "Win32_System_RemoteDesktop"))]
pub type DOT11EXTIHV_INIT_SERVICE = ::core::option::Option<unsafe extern "system" fn(dwvernumused: u32, pdot11extapi: *const DOT11EXT_APIS, pvreserved: *const ::core::ffi::c_void, pdot11ihvhandlers: *mut DOT11EXT_IHV_HANDLERS) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_INIT_VIRTUAL_STATION = ::core::option::Option<unsafe extern "system" fn(pdot11extvsapi: *const DOT11EXT_VIRTUAL_STATION_APIS, pvreserved: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_IS_UI_REQUEST_PENDING = ::core::option::Option<unsafe extern "system" fn(guiduirequest: ::windows::core::GUID, pbisrequestpending: *mut super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub type DOT11EXTIHV_ONEX_INDICATE_RESULT = ::core::option::Option<unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, msonexresult: DOT11_MSONEX_RESULT, pdot11msonexresultparams: *const DOT11_MSONEX_RESULT_PARAMS) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub type DOT11EXTIHV_PERFORM_CAPABILITY_MATCH = ::core::option::Option<unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, pihvprofileparams: *const DOT11EXT_IHV_PROFILE_PARAMS, pihvconnprofile: *const DOT11EXT_IHV_CONNECTIVITY_PROFILE, pihvsecprofile: *const DOT11EXT_IHV_SECURITY_PROFILE, pconnectablebssid: *const DOT11_BSS_LIST, pdwreasoncode: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub type DOT11EXTIHV_PERFORM_POST_ASSOCIATE = ::core::option::Option<unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, hsecuritysessionid: super::super::Foundation::HANDLE, pportstate: *const DOT11_PORT_STATE, udot11assocparamsbytes: u32, pdot11assocparams: *const DOT11_ASSOCIATION_COMPLETION_PARAMETERS) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub type DOT11EXTIHV_PERFORM_PRE_ASSOCIATE = ::core::option::Option<unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, hconnectsession: super::super::Foundation::HANDLE, pihvprofileparams: *const DOT11EXT_IHV_PROFILE_PARAMS, pihvconnprofile: *const DOT11EXT_IHV_CONNECTIVITY_PROFILE, pihvsecprofile: *const DOT11EXT_IHV_SECURITY_PROFILE, pconnectablebssid: *const DOT11_BSS_LIST, pdwreasoncode: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_System_RemoteDesktop\"`*"]
#[cfg(feature = "Win32_System_RemoteDesktop")]
pub type DOT11EXTIHV_PROCESS_SESSION_CHANGE = ::core::option::Option<unsafe extern "system" fn(ueventtype: u32, psessionnotification: *const super::super::System::RemoteDesktop::WTSSESSION_NOTIFICATION) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub type DOT11EXTIHV_PROCESS_UI_RESPONSE = ::core::option::Option<unsafe extern "system" fn(guiduirequest: ::windows::core::GUID, dwbytecount: u32, pvresponsebuffer: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_QUERY_UI_REQUEST = ::core::option::Option<unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, connectionphase: DOT11EXT_IHV_CONNECTION_PHASE, ppihvuirequest: *mut *mut DOT11EXT_IHV_UI_REQUEST) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_RECEIVE_INDICATION = ::core::option::Option<unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, indicationtype: DOT11EXT_IHV_INDICATION_TYPE, ubufferlength: u32, pvbuffer: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_RECEIVE_PACKET = ::core::option::Option<unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, dwinbuffersize: u32, pvinbuffer: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_SEND_PACKET_COMPLETION = ::core::option::Option<unsafe extern "system" fn(hsendcompletion: super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXTIHV_STOP_POST_ASSOCIATE = ::core::option::Option<unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, ppeer: *const *const u8, dot11assocstatus: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub type DOT11EXTIHV_VALIDATE_PROFILE = ::core::option::Option<unsafe extern "system" fn(hihvextadapter: super::super::Foundation::HANDLE, pihvprofileparams: *const DOT11EXT_IHV_PROFILE_PARAMS, pihvconnprofile: *const DOT11EXT_IHV_CONNECTIVITY_PROFILE, pihvsecprofile: *const DOT11EXT_IHV_SECURITY_PROFILE, pdwreasoncode: *mut u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub type DOT11EXT_ALLOCATE_BUFFER = ::core::option::Option<unsafe extern "system" fn(dwbytecount: u32, ppvbuffer: *mut *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub type DOT11EXT_FREE_BUFFER = ::core::option::Option<unsafe extern "system" fn(pvmemory: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_GET_PROFILE_CUSTOM_USER_DATA = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, hconnectsession: super::super::Foundation::HANDLE, dwsessionid: u32, pdwdatasize: *mut u32, ppvdata: *mut *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_NIC_SPECIFIC_EXTENSION = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, dwinbuffersize: u32, pvinbuffer: *const ::core::ffi::c_void, pdwoutbuffersize: *mut u32, pvoutbuffer: *mut ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_Security_ExtensibleAuthenticationProtocol\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
pub type DOT11EXT_ONEX_START = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, peapattributes: *const super::super::Security::ExtensibleAuthenticationProtocol::EAP_ATTRIBUTES) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_ONEX_STOP = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_POST_ASSOCIATE_COMPLETION = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, hsecuritysessionid: super::super::Foundation::HANDLE, ppeer: *const *const u8, dwreasoncode: u32, dwwin32error: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_PRE_ASSOCIATE_COMPLETION = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, hconnectsession: super::super::Foundation::HANDLE, dwreasoncode: u32, dwwin32error: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_PROCESS_ONEX_PACKET = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, dwinpacketsize: u32, pvinpacket: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_QUERY_VIRTUAL_STATION_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, pbisvirtualstation: *mut super::super::Foundation::BOOL, pgprimary: *mut ::windows::core::GUID, pvreserved: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_RELEASE_VIRTUAL_STATION = ::core::option::Option<unsafe extern "system" fn(hdot11primaryhandle: super::super::Foundation::HANDLE, pvreserved: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_REQUEST_VIRTUAL_STATION = ::core::option::Option<unsafe extern "system" fn(hdot11primaryhandle: super::super::Foundation::HANDLE, pvreserved: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SEND_NOTIFICATION = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, pnotificationdata: *const L2_NOTIFICATION_DATA) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SEND_PACKET = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, upacketlen: u32, pvpacket: *const ::core::ffi::c_void, hsendcompletion: super::super::Foundation::HANDLE) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SEND_UI_REQUEST = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, pihvuirequest: *const DOT11EXT_IHV_UI_REQUEST) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_AUTH_ALGORITHM = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, dwauthalgo: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_CURRENT_PROFILE = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, hconnectsession: super::super::Foundation::HANDLE, pihvconnprofile: *const DOT11EXT_IHV_CONNECTIVITY_PROFILE, pihvsecprofile: *const DOT11EXT_IHV_SECURITY_PROFILE) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`, `\"Win32_NetworkManagement_Ndis\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
pub type DOT11EXT_SET_DEFAULT_KEY = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, pkey: *const DOT11_CIPHER_DEFAULT_KEY_VALUE, dot11direction: DOT11_DIRECTION) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_DEFAULT_KEY_ID = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, udefaultkeyid: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_ETHERTYPE_HANDLING = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, umaxbacklog: u32, unumofexemption: u32, pexemption: *const DOT11_PRIVACY_EXEMPTION, unumofregistration: u32, pusregistration: *const u16) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_EXCLUDE_UNENCRYPTED = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, bexcludeunencrypted: super::super::Foundation::BOOL) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_KEY_MAPPING_KEY = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, pkey: *const DOT11_CIPHER_KEY_MAPPING_KEY_VALUE) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_MULTICAST_CIPHER_ALGORITHM = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, dwmulticastcipheralgo: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_PROFILE_CUSTOM_USER_DATA = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, hconnectsession: super::super::Foundation::HANDLE, dwsessionid: u32, dwdatasize: u32, pvdata: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_UNICAST_CIPHER_ALGORITHM = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, dwunicastcipheralgo: u32) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type DOT11EXT_SET_VIRTUAL_STATION_AP_PROPERTIES = ::core::option::Option<unsafe extern "system" fn(hdot11svchandle: super::super::Foundation::HANDLE, hconnectsession: super::super::Foundation::HANDLE, dwnumproperties: u32, pproperties: *const DOT11EXT_VIRTUAL_STATION_AP_PROPERTY, pvreserved: *const ::core::ffi::c_void) -> u32>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub type WFD_OPEN_SESSION_COMPLETE_CALLBACK = ::core::option::Option<unsafe extern "system" fn(hsessionhandle: super::super::Foundation::HANDLE, pvcontext: *const ::core::ffi::c_void, guidsessioninterface: ::windows::core::GUID, dwerror: u32, dwreasoncode: u32) -> ()>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WiFi\"`*"]
pub type WLAN_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *mut L2_NOTIFICATION_DATA, param1: *mut ::core::ffi::c_void) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
