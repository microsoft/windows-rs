#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDCancelOpenSession(hsessionhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDCloseHandle(hclienthandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDCloseSession(hsessionhandle: super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDOpenHandle(dwclientversion: u32, pdwnegotiatedversion: *mut u32, phclienthandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDOpenLegacySession(hclienthandle: super::super::Foundation::HANDLE, plegacymacaddress: *const *const u8, phsessionhandle: *mut super::super::Foundation::HANDLE, pguidsessioninterface: *mut ::windows::runtime::GUID) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDStartOpenSession(hclienthandle: super::super::Foundation::HANDLE, pdeviceaddress: *const *const u8, pvcontext: *const ::core::ffi::c_void, pfncallback: ::windows::runtime::RawPtr, phsessionhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
    pub fn WFDUpdateDeviceVisibility(pdeviceaddress: *const *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
    pub fn WlanAllocateMemory(dwmemorysize: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanCloseHandle(hclienthandle: super::super::Foundation::HANDLE, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn WlanConnect(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, pconnectionparameters: *const WLAN_CONNECTION_PARAMETERS, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn WlanConnect2(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, pconnectionparameters: *const WLAN_CONNECTION_PARAMETERS_V2, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanDeleteProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, strprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanDeviceServiceCommand(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, pdeviceserviceguid: *const ::windows::runtime::GUID, dwopcode: u32, dwinbuffersize: u32, pinbuffer: *const ::core::ffi::c_void, dwoutbuffersize: u32, poutbuffer: *mut ::core::ffi::c_void, pdwbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanDisconnect(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanEnumInterfaces(hclienthandle: super::super::Foundation::HANDLE, preserved: *mut ::core::ffi::c_void, ppinterfacelist: *mut *mut WLAN_INTERFACE_INFO_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanExtractPsdIEDataList(hclienthandle: super::super::Foundation::HANDLE, dwiedatasize: u32, prawiedata: *const u8, strformat: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void, pppsdiedatalist: *mut *mut WLAN_RAW_DATA_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
    pub fn WlanFreeMemory(pmemory: *const ::core::ffi::c_void);
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetAvailableNetworkList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, dwflags: u32, preserved: *mut ::core::ffi::c_void, ppavailablenetworklist: *mut *mut WLAN_AVAILABLE_NETWORK_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetAvailableNetworkList2(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, dwflags: u32, preserved: *mut ::core::ffi::c_void, ppavailablenetworklist: *mut *mut WLAN_AVAILABLE_NETWORK_LIST_V2) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetFilterList(hclienthandle: super::super::Foundation::HANDLE, wlanfilterlisttype: WLAN_FILTER_LIST_TYPE, preserved: *mut ::core::ffi::c_void, ppnetworklist: *mut *mut DOT11_NETWORK_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetInterfaceCapability(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, preserved: *mut ::core::ffi::c_void, ppcapability: *mut *mut WLAN_INTERFACE_CAPABILITY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetNetworkBssList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, pdot11ssid: *const DOT11_SSID, dot11bsstype: DOT11_BSS_TYPE, bsecurityenabled: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void, ppwlanbsslist: *mut *mut WLAN_BSS_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, strprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void, pstrprofilexml: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32, pdwgrantedaccess: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetProfileCustomUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, strprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetProfileList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, preserved: *mut ::core::ffi::c_void, ppprofilelist: *mut *mut WLAN_PROFILE_INFO_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetSecuritySettings(hclienthandle: super::super::Foundation::HANDLE, securableobject: WLAN_SECURABLE_OBJECT, pvaluetype: *mut WLAN_OPCODE_VALUE_TYPE, pstrcurrentsddl: *mut super::super::Foundation::PWSTR, pdwgrantedaccess: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetSupportedDeviceServices(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, ppdevsvcguidlist: *mut *mut WLAN_DEVICE_SERVICE_GUID_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkForceStart(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkForceStop(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkInitSettings(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkQueryProperty(hclienthandle: super::super::Foundation::HANDLE, opcode: WLAN_HOSTED_NETWORK_OPCODE, pdwdatasize: *mut u32, ppvdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkQuerySecondaryKey(hclienthandle: super::super::Foundation::HANDLE, pdwkeylength: *mut u32, ppuckeydata: *mut *mut u8, pbispassphrase: *mut super::super::Foundation::BOOL, pbpersistent: *mut super::super::Foundation::BOOL, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkQueryStatus(hclienthandle: super::super::Foundation::HANDLE, ppwlanhostednetworkstatus: *mut *mut WLAN_HOSTED_NETWORK_STATUS, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkRefreshSecuritySettings(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkSetProperty(hclienthandle: super::super::Foundation::HANDLE, opcode: WLAN_HOSTED_NETWORK_OPCODE, dwdatasize: u32, pvdata: *const ::core::ffi::c_void, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkSetSecondaryKey(hclienthandle: super::super::Foundation::HANDLE, dwkeylength: u32, puckeydata: *const u8, bispassphrase: super::super::Foundation::BOOL, bpersistent: super::super::Foundation::BOOL, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkStartUsing(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkStopUsing(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanIhvControl(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, r#type: WLAN_IHV_CONTROL_TYPE, dwinbuffersize: u32, pinbuffer: *const ::core::ffi::c_void, dwoutbuffersize: u32, poutbuffer: *mut ::core::ffi::c_void, pdwbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanOpenHandle(dwclientversion: u32, preserved: *mut ::core::ffi::c_void, pdwnegotiatedversion: *mut u32, phclienthandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanQueryAutoConfigParameter(hclienthandle: super::super::Foundation::HANDLE, opcode: WLAN_AUTOCONF_OPCODE, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanQueryInterface(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, opcode: WLAN_INTF_OPCODE, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanReasonCodeToString(dwreasoncode: u32, dwbuffersize: u32, pstringbuffer: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRegisterDeviceServiceNotification(hclienthandle: super::super::Foundation::HANDLE, pdevsvcguidlist: *const WLAN_DEVICE_SERVICE_GUID_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRegisterNotification(hclienthandle: super::super::Foundation::HANDLE, dwnotifsource: u32, bignoreduplicate: super::super::Foundation::BOOL, funccallback: ::windows::runtime::RawPtr, pcallbackcontext: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void, pdwprevnotifsource: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRegisterVirtualStationNotification(hclienthandle: super::super::Foundation::HANDLE, bregister: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRenameProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, stroldprofilename: super::super::Foundation::PWSTR, strnewprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSaveTemporaryProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, strprofilename: super::super::Foundation::PWSTR, stralluserprofilesecurity: super::super::Foundation::PWSTR, dwflags: u32, boverwrite: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanScan(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, pdot11ssid: *const DOT11_SSID, piedata: *const WLAN_RAW_DATA, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetAutoConfigParameter(hclienthandle: super::super::Foundation::HANDLE, opcode: WLAN_AUTOCONF_OPCODE, dwdatasize: u32, pdata: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetFilterList(hclienthandle: super::super::Foundation::HANDLE, wlanfilterlisttype: WLAN_FILTER_LIST_TYPE, pnetworklist: *const DOT11_NETWORK_LIST, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetInterface(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, opcode: WLAN_INTF_OPCODE, dwdatasize: u32, pdata: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, dwflags: u32, strprofilexml: super::super::Foundation::PWSTR, stralluserprofilesecurity: super::super::Foundation::PWSTR, boverwrite: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void, pdwreasoncode: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfileCustomUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, strprofilename: super::super::Foundation::PWSTR, dwdatasize: u32, pdata: *const u8, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
    pub fn WlanSetProfileEapUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, strprofilename: super::super::Foundation::PWSTR, eaptype: super::super::Security::ExtensibleAuthenticationProtocol::EAP_METHOD_TYPE, dwflags: WLAN_SET_EAPHOST_FLAGS, dweapuserdatasize: u32, pbeapuserdata: *const u8, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfileEapXmlUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, strprofilename: super::super::Foundation::PWSTR, dwflags: WLAN_SET_EAPHOST_FLAGS, streapxmluserdata: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfileList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, dwitems: u32, strprofilenames: *const super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfilePosition(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows::runtime::GUID, strprofilename: super::super::Foundation::PWSTR, dwposition: u32, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetPsdIEDataList(hclienthandle: super::super::Foundation::HANDLE, strformat: super::super::Foundation::PWSTR, ppsdiedatalist: *const WLAN_RAW_DATA_LIST, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetSecuritySettings(hclienthandle: super::super::Foundation::HANDLE, securableobject: WLAN_SECURABLE_OBJECT, strmodifiedsddl: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanUIEditProfile(dwclientversion: u32, wstrprofilename: super::super::Foundation::PWSTR, pinterfaceguid: *const ::windows::runtime::GUID, hwnd: super::super::Foundation::HWND, wlstartpage: WL_DISPLAY_PAGES, preserved: *mut ::core::ffi::c_void, pwlanreasoncode: *mut u32) -> u32;
}
