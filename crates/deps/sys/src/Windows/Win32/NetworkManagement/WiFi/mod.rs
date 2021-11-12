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
    pub fn WFDOpenLegacySession(hclienthandle: super::super::Foundation::HANDLE, plegacymacaddress: *const *const u8, phsessionhandle: *mut super::super::Foundation::HANDLE, pguidsessioninterface: *mut ::windows_sys::core::GUID) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDStartOpenSession(hclienthandle: super::super::Foundation::HANDLE, pdeviceaddress: *const *const u8, pvcontext: *const ::core::ffi::c_void, pfncallback: WFD_OPEN_SESSION_COMPLETE_CALLBACK, phsessionhandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
    pub fn WFDUpdateDeviceVisibility(pdeviceaddress: *const *const u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
    pub fn WlanAllocateMemory(dwmemorysize: u32) -> *mut ::core::ffi::c_void;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanCloseHandle(hclienthandle: super::super::Foundation::HANDLE, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn WlanConnect(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, pconnectionparameters: *const WLAN_CONNECTION_PARAMETERS, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`, `Win32_NetworkManagement_Ndis`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn WlanConnect2(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, pconnectionparameters: *const WLAN_CONNECTION_PARAMETERS_V2, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanDeleteProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanDeviceServiceCommand(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, pdeviceserviceguid: *const ::windows_sys::core::GUID, dwopcode: u32, dwinbuffersize: u32, pinbuffer: *const ::core::ffi::c_void, dwoutbuffersize: u32, poutbuffer: *mut ::core::ffi::c_void, pdwbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanDisconnect(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, preserved: *mut ::core::ffi::c_void) -> u32;
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
    pub fn WlanGetAvailableNetworkList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, dwflags: u32, preserved: *mut ::core::ffi::c_void, ppavailablenetworklist: *mut *mut WLAN_AVAILABLE_NETWORK_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetAvailableNetworkList2(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, dwflags: u32, preserved: *mut ::core::ffi::c_void, ppavailablenetworklist: *mut *mut WLAN_AVAILABLE_NETWORK_LIST_V2) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetFilterList(hclienthandle: super::super::Foundation::HANDLE, wlanfilterlisttype: WLAN_FILTER_LIST_TYPE, preserved: *mut ::core::ffi::c_void, ppnetworklist: *mut *mut DOT11_NETWORK_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetInterfaceCapability(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, preserved: *mut ::core::ffi::c_void, ppcapability: *mut *mut WLAN_INTERFACE_CAPABILITY) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetNetworkBssList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, pdot11ssid: *const DOT11_SSID, dot11bsstype: DOT11_BSS_TYPE, bsecurityenabled: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void, ppwlanbsslist: *mut *mut WLAN_BSS_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void, pstrprofilexml: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32, pdwgrantedaccess: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetProfileCustomUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetProfileList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, preserved: *mut ::core::ffi::c_void, ppprofilelist: *mut *mut WLAN_PROFILE_INFO_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetSecuritySettings(hclienthandle: super::super::Foundation::HANDLE, securableobject: WLAN_SECURABLE_OBJECT, pvaluetype: *mut WLAN_OPCODE_VALUE_TYPE, pstrcurrentsddl: *mut super::super::Foundation::PWSTR, pdwgrantedaccess: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetSupportedDeviceServices(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, ppdevsvcguidlist: *mut *mut WLAN_DEVICE_SERVICE_GUID_LIST) -> u32;
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
    pub fn WlanIhvControl(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, r#type: WLAN_IHV_CONTROL_TYPE, dwinbuffersize: u32, pinbuffer: *const ::core::ffi::c_void, dwoutbuffersize: u32, poutbuffer: *mut ::core::ffi::c_void, pdwbytesreturned: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanOpenHandle(dwclientversion: u32, preserved: *mut ::core::ffi::c_void, pdwnegotiatedversion: *mut u32, phclienthandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanQueryAutoConfigParameter(hclienthandle: super::super::Foundation::HANDLE, opcode: WLAN_AUTOCONF_OPCODE, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanQueryInterface(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, opcode: WLAN_INTF_OPCODE, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanReasonCodeToString(dwreasoncode: u32, dwbuffersize: u32, pstringbuffer: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRegisterDeviceServiceNotification(hclienthandle: super::super::Foundation::HANDLE, pdevsvcguidlist: *const WLAN_DEVICE_SERVICE_GUID_LIST) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRegisterNotification(hclienthandle: super::super::Foundation::HANDLE, dwnotifsource: u32, bignoreduplicate: super::super::Foundation::BOOL, funccallback: WLAN_NOTIFICATION_CALLBACK, pcallbackcontext: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void, pdwprevnotifsource: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRegisterVirtualStationNotification(hclienthandle: super::super::Foundation::HANDLE, bregister: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRenameProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, stroldprofilename: super::super::Foundation::PWSTR, strnewprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSaveTemporaryProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, stralluserprofilesecurity: super::super::Foundation::PWSTR, dwflags: u32, boverwrite: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanScan(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, pdot11ssid: *const DOT11_SSID, piedata: *const WLAN_RAW_DATA, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetAutoConfigParameter(hclienthandle: super::super::Foundation::HANDLE, opcode: WLAN_AUTOCONF_OPCODE, dwdatasize: u32, pdata: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetFilterList(hclienthandle: super::super::Foundation::HANDLE, wlanfilterlisttype: WLAN_FILTER_LIST_TYPE, pnetworklist: *const DOT11_NETWORK_LIST, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetInterface(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, opcode: WLAN_INTF_OPCODE, dwdatasize: u32, pdata: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, dwflags: u32, strprofilexml: super::super::Foundation::PWSTR, stralluserprofilesecurity: super::super::Foundation::PWSTR, boverwrite: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void, pdwreasoncode: *mut u32) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfileCustomUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, dwdatasize: u32, pdata: *const u8, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`, `Win32_Security_ExtensibleAuthenticationProtocol`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
    pub fn WlanSetProfileEapUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, eaptype: super::super::Security::ExtensibleAuthenticationProtocol::EAP_METHOD_TYPE, dwflags: WLAN_SET_EAPHOST_FLAGS, dweapuserdatasize: u32, pbeapuserdata: *const u8, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfileEapXmlUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, dwflags: WLAN_SET_EAPHOST_FLAGS, streapxmluserdata: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfileList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, dwitems: u32, strprofilenames: *const super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfilePosition(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, dwposition: u32, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetPsdIEDataList(hclienthandle: super::super::Foundation::HANDLE, strformat: super::super::Foundation::PWSTR, ppsdiedatalist: *const WLAN_RAW_DATA_LIST, preserved: *mut ::core::ffi::c_void) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetSecuritySettings(hclienthandle: super::super::Foundation::HANDLE, securableobject: WLAN_SECURABLE_OBJECT, strmodifiedsddl: super::super::Foundation::PWSTR) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanUIEditProfile(dwclientversion: u32, wstrprofilename: super::super::Foundation::PWSTR, pinterfaceguid: *const ::windows_sys::core::GUID, hwnd: super::super::Foundation::HWND, wlstartpage: WL_DISPLAY_PAGES, preserved: *mut ::core::ffi::c_void, pwlanreasoncode: *mut u32) -> u32;
}
pub struct CH_DESCRIPTION_TYPE(i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_InfraCast_AccessPointBssid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_InfraCast_ChallengeAep: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_InfraCast_DevnodeAep: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_InfraCast_HostName_ResolutionMode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_InfraCast_PinSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_InfraCast_RtspTcpConnectionParametersSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 30u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_InfraCast_SinkHostName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_InfraCast_SinkIpAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_InfraCast_StreamSecuritySupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_InfraCast_Supported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirectServices_AdvertisementId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 833845059,
        data2: 31838,
        data3: 16389,
        data4: [147, 230, 233, 83, 249, 43, 130, 233],
    },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirectServices_RequestServiceInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 833845059,
        data2: 31838,
        data3: 16389,
        data4: [147, 230, 233, 83, 249, 43, 130, 233],
    },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirectServices_ServiceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 833845059,
        data2: 31838,
        data3: 16389,
        data4: [147, 230, 233, 83, 249, 43, 130, 233],
    },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirectServices_ServiceConfigMethods: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 833845059,
        data2: 31838,
        data3: 16389,
        data4: [147, 230, 233, 83, 249, 43, 130, 233],
    },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirectServices_ServiceInformation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 833845059,
        data2: 31838,
        data3: 16389,
        data4: [147, 230, 233, 83, 249, 43, 130, 233],
    },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirectServices_ServiceName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 833845059,
        data2: 31838,
        data3: 16389,
        data4: [147, 230, 233, 83, 249, 43, 130, 233],
    },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_DeviceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_DeviceAddressCopy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_FoundWsbService: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_GroupId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_InformationElements: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_InterfaceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_InterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_IsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_IsDMGCapable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 22u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_IsLegacyDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_IsMiracastLCPSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_IsRecentlyAssociated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_IsVisible: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_LinkQuality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_MiracastVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_Miracast_SessionMgmtControlPort: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 31u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_NoMiracastAutoProject: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_RtspTcpConnectionParametersSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 32u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_Service_Aeps: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_Services: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_SupportedChannelList: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFiDirect_TransientAssociation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`, `Win32_UI_Shell_PropertiesSystem`*"]
pub const DEVPKEY_WiFi_InterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 4010895339,
        data2: 52220,
        data3: 17217,
        data4: [165, 104, 167, 201, 26, 104, 152, 44],
    },
    pid: 2u32,
};
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DISCOVERY_FILTER_BITMASK_ANY: u32 = 15u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DISCOVERY_FILTER_BITMASK_DEVICE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DISCOVERY_FILTER_BITMASK_GO: u32 = 2u32;
pub struct DOT11_ACCESSNETWORKOPTIONS(i32);
pub struct DOT11_AC_PARAM(i32);
pub struct DOT11_ADDITIONAL_IE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ADDITIONAL_IE_REVISION_1: u32 = 1u32;
pub struct DOT11_ADHOC_AUTH_ALGORITHM(i32);
pub struct DOT11_ADHOC_CIPHER_ALGORITHM(i32);
pub struct DOT11_ADHOC_CONNECT_FAIL_REASON(i32);
pub struct DOT11_ADHOC_NETWORK_CONNECTION_STATUS(i32);
pub struct DOT11_ANQP_QUERY_COMPLETE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ANQP_QUERY_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_ANQP_QUERY_RESULT(i32);
pub struct DOT11_AP_JOIN_REQUEST(i32);
pub struct DOT11_ASSOCIATION_COMPLETION_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ASSOCIATION_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ASSOCIATION_COMPLETION_PARAMETERS_REVISION_2: u32 = 2u32;
pub struct DOT11_ASSOCIATION_INFO_EX(i32);
pub struct DOT11_ASSOCIATION_INFO_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ASSOCIATION_INFO_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_ASSOCIATION_PARAMS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ASSOCIATION_PARAMS_REVISION_1: u32 = 1u32;
pub struct DOT11_ASSOCIATION_START_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ASSOCIATION_START_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_ASSOCIATION_STATE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ASSOC_ERROR_SOURCE_OS: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ASSOC_ERROR_SOURCE_OTHER: u32 = 255u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ASSOC_ERROR_SOURCE_REMOTE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ASSOC_STATUS_SUCCESS: u32 = 0u32;
pub struct DOT11_AUTH_ALGORITHM(i32);
pub struct DOT11_AUTH_ALGORITHM_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_AUTH_ALGORITHM_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_AUTH_ALGO_MICHAEL: u32 = 1u32;
pub struct DOT11_AUTH_CIPHER_PAIR(i32);
pub struct DOT11_AUTH_CIPHER_PAIR_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_AUTH_CIPHER_PAIR_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_AVAILABLE_CHANNEL_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_AVAILABLE_CHANNEL_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_AVAILABLE_FREQUENCY_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_AVAILABLE_FREQUENCY_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_BAND(i32);
pub struct DOT11_BSSID_CANDIDATE(i32);
pub struct DOT11_BSSID_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_BSSID_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_BSS_DESCRIPTION(i32);
pub struct DOT11_BSS_ENTRY(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_BSS_ENTRY_BYTE_ARRAY_REVISION_1: u32 = 1u32;
pub struct DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO(i32);
pub struct DOT11_BSS_TYPE(i32);
pub struct DOT11_BYTE_ARRAY(i32);
pub struct DOT11_CAN_SUSTAIN_AP_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CAN_SUSTAIN_AP_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CAN_SUSTAIN_AP_REASON_IHV_END: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CAN_SUSTAIN_AP_REASON_IHV_START: u32 = 4278190080u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CAPABILITY_CHANNEL_AGILITY: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CAPABILITY_DSSSOFDM: u32 = 8192u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CAPABILITY_INFO_CF_POLLABLE: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CAPABILITY_INFO_CF_POLL_REQ: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CAPABILITY_INFO_ESS: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CAPABILITY_INFO_IBSS: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CAPABILITY_INFO_PRIVACY: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CAPABILITY_PBCC: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CAPABILITY_SHORT_PREAMBLE: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CAPABILITY_SHORT_SLOT_TIME: u32 = 1024u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CCA_MODE_CS_ONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CCA_MODE_CS_WITH_TIMER: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CCA_MODE_ED_ONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CCA_MODE_ED_and_CS: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CCA_MODE_HRCS_AND_ED: u32 = 16u32;
pub struct DOT11_CHANNEL_HINT(i32);
pub struct DOT11_CIPHER_ALGORITHM(i32);
pub struct DOT11_CIPHER_ALGORITHM_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CIPHER_ALGORITHM_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_CIPHER_DEFAULT_KEY_VALUE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CIPHER_DEFAULT_KEY_VALUE_REVISION_1: u32 = 1u32;
pub struct DOT11_CIPHER_KEY_MAPPING_KEY_VALUE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CIPHER_KEY_MAPPING_KEY_VALUE_BYTE_ARRAY_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CONF_ALGO_TKIP: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CONF_ALGO_WEP_RC4: u32 = 1u32;
pub struct DOT11_CONNECTION_COMPLETION_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CONNECTION_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_CONNECTION_START_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CONNECTION_START_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_CONNECTION_STATUS_SUCCESS: u32 = 0u32;
pub struct DOT11_COUNTERS_ENTRY(i32);
pub struct DOT11_COUNTRY_OR_REGION_STRING_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_COUNTRY_OR_REGION_STRING_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_CURRENT_OFFLOAD_CAPABILITY(i32);
pub struct DOT11_CURRENT_OPERATION_MODE(i32);
pub struct DOT11_CURRENT_OPTIONAL_CAPABILITY(i32);
pub struct DOT11_DATA_RATE_MAPPING_ENTRY(i32);
pub struct DOT11_DATA_RATE_MAPPING_TABLE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_DATA_RATE_MAPPING_TABLE_REVISION_1: u32 = 1u32;
pub struct DOT11_DEFAULT_WEP_OFFLOAD(i32);
pub struct DOT11_DEFAULT_WEP_UPLOAD(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_DEVICE_ENTRY_BYTE_ARRAY_REVISION_1: u32 = 1u32;
pub struct DOT11_DIRECTION(i32);
pub struct DOT11_DISASSOCIATE_PEER_REQUEST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_DISASSOCIATE_PEER_REQUEST_REVISION_1: u32 = 1u32;
pub struct DOT11_DISASSOCIATION_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_DISASSOCIATION_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_DIVERSITY_SELECTION_RX(i32);
pub struct DOT11_DIVERSITY_SELECTION_RX_LIST(i32);
pub struct DOT11_DIVERSITY_SUPPORT(i32);
pub struct DOT11_DS_INFO(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ENCAP_802_1H: u32 = 2u32;
pub struct DOT11_ENCAP_ENTRY(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ENCAP_RFC_1042: u32 = 1u32;
pub struct DOT11_ERP_PHY_ATTRIBUTES(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXEMPT_ALWAYS: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXEMPT_BOTH: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXEMPT_MULTICAST: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXEMPT_NO_EXEMPTION: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXEMPT_ON_KEY_MAPPING_KEY_UNAVAILABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXEMPT_UNICAST: u32 = 1u32;
pub struct DOT11_EXTAP_ATTRIBUTES(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXTAP_ATTRIBUTES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXTAP_RECV_CONTEXT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXTAP_SEND_CONTEXT_REVISION_1: u32 = 1u32;
pub struct DOT11_EXTSTA_ATTRIBUTES(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_3: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_4: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_CERTIFIED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_OID_SUPPORTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_RESERVED: u32 = 12u32;
pub struct DOT11_EXTSTA_CAPABILITY(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXTSTA_CAPABILITY_REVISION_1: u32 = 1u32;
pub struct DOT11_EXTSTA_RECV_CONTEXT(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXTSTA_RECV_CONTEXT_REVISION_1: u32 = 1u32;
pub struct DOT11_EXTSTA_SEND_CONTEXT(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_EXTSTA_SEND_CONTEXT_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_FLAGS_80211B_CHANNEL_AGILITY: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_FLAGS_80211B_PBCC: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_FLAGS_80211B_SHORT_PREAMBLE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_FLAGS_80211G_BARKER_PREAMBLE_MODE: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_FLAGS_80211G_DSSS_OFDM: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_FLAGS_80211G_NON_ERP_PRESENT: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_FLAGS_80211G_USE_PROTECTION: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_FLAGS_PS_ON: u32 = 8u32;
pub struct DOT11_FRAGMENT_DESCRIPTOR(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_FREQUENCY_BANDS_LOWER: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_FREQUENCY_BANDS_MIDDLE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_FREQUENCY_BANDS_UPPER: u32 = 4u32;
pub struct DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_HESSID_LENGTH: u32 = 6u32;
pub struct DOT11_HOPPING_PATTERN_ENTRY(i32);
pub struct DOT11_HOPPING_PATTERN_ENTRY_LIST(i32);
pub struct DOT11_HOP_ALGO_ADOPTED(i32);
pub struct DOT11_HRDSSS_PHY_ATTRIBUTES(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_HR_CCA_MODE_CS_AND_ED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_HR_CCA_MODE_CS_ONLY: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_HR_CCA_MODE_CS_WITH_TIMER: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_HR_CCA_MODE_ED_ONLY: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_HR_CCA_MODE_HRCS_AND_ED: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_HW_DEFRAGMENTATION_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_HW_FRAGMENTATION_SUPPORTED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_HW_MSDU_AUTH_SUPPORTED_RX: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_HW_MSDU_AUTH_SUPPORTED_TX: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_HW_WEP_SUPPORTED_RX: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_HW_WEP_SUPPORTED_TX: u32 = 1u32;
pub struct DOT11_IBSS_PARAMS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_IBSS_PARAMS_REVISION_1: u32 = 1u32;
pub struct DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_INCOMING_ASSOC_DECISION(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_INCOMING_ASSOC_DECISION_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_INCOMING_ASSOC_DECISION_REVISION_2: u32 = 2u32;
pub struct DOT11_INCOMING_ASSOC_DECISION_V2(i32);
pub struct DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_INCOMING_ASSOC_STARTED_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_INCOMING_ASSOC_STARTED_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_INVALID_CHANNEL_NUMBER: u32 = 0u32;
pub struct DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_IV48_COUNTER(i32);
pub struct DOT11_JOIN_REQUEST(i32);
pub struct DOT11_KEY_ALGO_BIP(i32);
pub struct DOT11_KEY_ALGO_BIP_GMAC_256(i32);
pub struct DOT11_KEY_ALGO_CCMP(i32);
pub struct DOT11_KEY_ALGO_GCMP(i32);
pub struct DOT11_KEY_ALGO_GCMP_256(i32);
pub struct DOT11_KEY_ALGO_TKIP_MIC(i32);
pub struct DOT11_KEY_DIRECTION(i32);
pub struct DOT11_LINK_QUALITY_ENTRY(i32);
pub struct DOT11_LINK_QUALITY_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_LINK_QUALITY_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_MAC_ADDRESS_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_MAC_ADDRESS_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_MAC_FRAME_STATISTICS(i32);
pub struct DOT11_MAC_INFO(i32);
pub struct DOT11_MAC_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_MAC_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_MANUFACTURING_CALLBACK_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_MANUFACTURING_CALLBACK_REVISION_1: u32 = 1u32;
pub struct DOT11_MANUFACTURING_CALLBACK_TYPE(i32);
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC(i32);
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX(i32);
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX(i32);
pub struct DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS(i32);
pub struct DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS(i32);
pub struct DOT11_MANUFACTURING_SELF_TEST_TYPE(i32);
pub struct DOT11_MANUFACTURING_TEST(i32);
pub struct DOT11_MANUFACTURING_TEST_QUERY_DATA(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_MANUFACTURING_TEST_REVISION_1: u32 = 1u32;
pub struct DOT11_MANUFACTURING_TEST_SET_DATA(i32);
pub struct DOT11_MANUFACTURING_TEST_SLEEP(i32);
pub struct DOT11_MANUFACTURING_TEST_TYPE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_MAX_CHANNEL_HINTS: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_MAX_NUM_DEFAULT_KEY: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_MAX_NUM_DEFAULT_KEY_MFP: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_MAX_NUM_OF_FRAGMENTS: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_MAX_PDU_SIZE: u32 = 2346u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_MAX_REQUESTED_SERVICE_INFORMATION_LENGTH: u32 = 255u32;
pub struct DOT11_MD_CAPABILITY_ENTRY_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_MIN_PDU_SIZE: u32 = 256u32;
pub struct DOT11_MPDU_MAX_LENGTH_INDICATION(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_MPDU_MAX_LENGTH_INDICATION_REVISION_1: u32 = 1u32;
pub struct DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY(i32);
pub struct DOT11_NETWORK(i32);
pub struct DOT11_NETWORK_LIST(i32);
pub struct DOT11_NIC_SPECIFIC_EXTENSION(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_NLO_FLAG_SCAN_AT_SYSTEM_RESUME: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_NLO_FLAG_SCAN_ON_AOAC_PLATFORM: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_NLO_FLAG_STOP_NLO_INDICATION: u32 = 1u32;
pub struct DOT11_OFDM_PHY_ATTRIBUTES(i32);
pub struct DOT11_OFFLOAD_CAPABILITY(i32);
pub struct DOT11_OFFLOAD_NETWORK(i32);
pub struct DOT11_OFFLOAD_NETWORK_LIST_INFO(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_OFFLOAD_NETWORK_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_OFFLOAD_TYPE(i32);
pub struct DOT11_OI(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_OI_MAX_LENGTH: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_OI_MIN_LENGTH: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_OPERATION_MODE_AP: u32 = 2u32;
pub struct DOT11_OPERATION_MODE_CAPABILITY(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_OPERATION_MODE_EXTENSIBLE_AP: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_OPERATION_MODE_EXTENSIBLE_STATION: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_OPERATION_MODE_MANUFACTURING: u32 = 1073741824u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_OPERATION_MODE_NETWORK_MONITOR: u32 = 2147483648u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_OPERATION_MODE_STATION: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_OPERATION_MODE_UNKNOWN: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_OPERATION_MODE_WFD_CLIENT: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_OPERATION_MODE_WFD_DEVICE: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_OPERATION_MODE_WFD_GROUP_OWNER: u32 = 32u32;
pub struct DOT11_OPTIONAL_CAPABILITY(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_CTRL: u32 = 4096u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_DATA: u32 = 16384u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_MGMT: u32 = 8192u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_BROADCAST_CTRL: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_BROADCAST_DATA: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_BROADCAST_MGMT: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_DIRECTED_CTRL: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_DIRECTED_DATA: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_DIRECTED_MGMT: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_MULTICAST_CTRL: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_MULTICAST_DATA: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_MULTICAST_MGMT: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_PROMISCUOUS_CTRL: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_PROMISCUOUS_DATA: u32 = 2048u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PACKET_TYPE_PROMISCUOUS_MGMT: u32 = 1024u32;
pub struct DOT11_PEER_INFO(i32);
pub struct DOT11_PEER_INFO_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PEER_INFO_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_PEER_STATISTICS(i32);
pub struct DOT11_PER_MSDU_COUNTERS(i32);
pub struct DOT11_PHY_ATTRIBUTES(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PHY_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub struct DOT11_PHY_FRAME_STATISTICS(i32);
pub struct DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_PHY_ID_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PHY_ID_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_PHY_STATE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PHY_STATE_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_PHY_TYPE(i32);
pub struct DOT11_PHY_TYPE_INFO(i32);
pub struct DOT11_PHY_TYPE_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PHY_TYPE_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_PMKID_CANDIDATE_LIST_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PMKID_CANDIDATE_LIST_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_PMKID_ENTRY(i32);
pub struct DOT11_PMKID_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PMKID_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_PORT_STATE_NOTIFICATION(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PORT_STATE_NOTIFICATION_REVISION_1: u32 = 1u32;
pub struct DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_POWER_MGMT_AUTO_MODE_ENABLED_REVISION_1: u32 = 1u32;
pub struct DOT11_POWER_MGMT_MODE(i32);
pub struct DOT11_POWER_MGMT_MODE_STATUS_INFO(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_POWER_MGMT_MODE_STATUS_INFO_REVISION_1: u32 = 1u32;
pub struct DOT11_POWER_MODE(i32);
pub struct DOT11_POWER_MODE_REASON(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_POWER_SAVE_LEVEL_FAST_PSP: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_POWER_SAVE_LEVEL_MAX_PSP: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_POWER_SAVING_FAST_PSP: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_POWER_SAVING_MAXIMUM_LEVEL: u32 = 24u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_POWER_SAVING_MAX_PSP: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_POWER_SAVING_NO_POWER_SAVING: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PRIORITY_CONTENTION: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PRIORITY_CONTENTION_FREE: u32 = 1u32;
pub struct DOT11_PRIVACY_EXEMPTION(i32);
pub struct DOT11_PRIVACY_EXEMPTION_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PRIVACY_EXEMPTION_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PSD_IE_MAX_DATA_SIZE: u32 = 240u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_PSD_IE_MAX_ENTRY_NUMBER: u32 = 5u32;
pub struct DOT11_QOS_PARAMS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_QOS_PARAMS_REVISION_1: u32 = 1u32;
pub struct DOT11_QOS_TX_DURATION(i32);
pub struct DOT11_QOS_TX_MEDIUM_TIME(i32);
pub struct DOT11_RADIO_STATE(i32);
pub struct DOT11_RATE_SET(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_RATE_SET_MAX_LENGTH: u32 = 126u32;
pub struct DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_RECV_CONTEXT_REVISION_1: u32 = 1u32;
pub struct DOT11_RECV_EXTENSION_INFO(i32);
pub struct DOT11_RECV_EXTENSION_INFO_V2(i32);
pub struct DOT11_RECV_SENSITIVITY(i32);
pub struct DOT11_RECV_SENSITIVITY_LIST(i32);
pub struct DOT11_REG_DOMAINS_SUPPORT_VALUE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_REG_DOMAIN_DOC: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_REG_DOMAIN_ETSI: u32 = 48u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_REG_DOMAIN_FCC: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_REG_DOMAIN_FRANCE: u32 = 50u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_REG_DOMAIN_MKK: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_REG_DOMAIN_OTHER: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_REG_DOMAIN_SPAIN: u32 = 49u32;
pub struct DOT11_REG_DOMAIN_VALUE(i32);
pub struct DOT11_RESET_REQUEST(i32);
pub struct DOT11_RESET_TYPE(i32);
pub struct DOT11_ROAMING_COMPLETION_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ROAMING_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_ROAMING_START_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_ROAMING_START_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_RSSI_RANGE(i32);
pub struct DOT11_SCAN_REQUEST(i32);
pub struct DOT11_SCAN_REQUEST_V2(i32);
pub struct DOT11_SCAN_TYPE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_SEND_CONTEXT_REVISION_1: u32 = 1u32;
pub struct DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_SEND_INVITATION_REQUEST_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_SEND_INVITATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_SEND_INVITATION_RESPONSE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_SEND_INVITATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_SERVICE_CLASS_REORDERABLE_MULTICAST: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_SERVICE_CLASS_STRICTLY_ORDERED: u32 = 1u32;
pub struct DOT11_SSID(i32);
pub struct DOT11_SSID_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_SSID_LIST_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_SSID_MAX_LENGTH: u32 = 32u32;
pub struct DOT11_START_REQUEST(i32);
pub struct DOT11_STATISTICS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATISTICS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_AP_JOIN_CONFIRM: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_AUTH_FAILED: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_AUTH_NOT_VERIFIED: u32 = 32768u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_AUTH_VERIFIED: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_ENCRYPTION_FAILED: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_EXCESSIVE_DATA_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_GENERATE_AUTH_FAILED: u32 = 16384u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_ICV_VERIFIED: u32 = 2048u32;
pub struct DOT11_STATUS_INDICATION(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_JOIN_CONFIRM: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_MPDU_MAX_LENGTH_CHANGED: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_PACKET_NOT_REASSEMBLED: u32 = 8192u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_PACKET_REASSEMBLED: u32 = 4096u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_PS_LIFETIME_EXPIRED: u32 = 262144u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_RESET_CONFIRM: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_RETRY_LIMIT_EXCEEDED: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_SCAN_CONFIRM: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_START_CONFIRM: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_SUCCESS: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_UNAVAILABLE_BSS: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_UNAVAILABLE_PRIORITY: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_UNAVAILABLE_SERVICE_CLASS: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_UNSUPPORTED_PRIORITY: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_UNSUPPORTED_SERVICE_CLASS: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_WEP_KEY_UNAVAILABLE: u32 = 1024u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STATUS_XMIT_MSDU_TIMER_EXPIRED: u32 = 64u32;
pub struct DOT11_STOP_AP_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STOP_AP_PARAMETERS_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STOP_AP_REASON_AP_ACTIVE: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STOP_AP_REASON_CHANNEL_NOT_AVAILABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STOP_AP_REASON_FREQUENCY_NOT_AVAILABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STOP_AP_REASON_IHV_END: u32 = 4294967295u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_STOP_AP_REASON_IHV_START: u32 = 4278190080u32;
pub struct DOT11_SUPPORTED_ANTENNA(i32);
pub struct DOT11_SUPPORTED_ANTENNA_LIST(i32);
pub struct DOT11_SUPPORTED_DATA_RATES_VALUE(i32);
pub struct DOT11_SUPPORTED_DATA_RATES_VALUE_V2(i32);
pub struct DOT11_SUPPORTED_DSSS_CHANNEL(i32);
pub struct DOT11_SUPPORTED_DSSS_CHANNEL_LIST(i32);
pub struct DOT11_SUPPORTED_OFDM_FREQUENCY(i32);
pub struct DOT11_SUPPORTED_OFDM_FREQUENCY_LIST(i32);
pub struct DOT11_SUPPORTED_PHY_TYPES(i32);
pub struct DOT11_SUPPORTED_POWER_LEVELS(i32);
pub struct DOT11_TEMP_TYPE(i32);
pub struct DOT11_TKIPMIC_FAILURE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_TKIPMIC_FAILURE_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_UPDATE_IE(i32);
pub struct DOT11_UPDATE_IE_OP(i32);
pub struct DOT11_VENUEINFO(i32);
pub struct DOT11_VWIFI_ATTRIBUTES(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_VWIFI_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub struct DOT11_VWIFI_COMBINATION(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_VWIFI_COMBINATION_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_VWIFI_COMBINATION_REVISION_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_VWIFI_COMBINATION_REVISION_3: u32 = 3u32;
pub struct DOT11_VWIFI_COMBINATION_V2(i32);
pub struct DOT11_VWIFI_COMBINATION_V3(i32);
pub struct DOT11_WEP_OFFLOAD(i32);
pub struct DOT11_WEP_UPLOAD(i32);
pub struct DOT11_WFD_ADDITIONAL_IE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_ADDITIONAL_IE_REVISION_1: u32 = 1u32;
pub struct DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR(i32);
pub struct DOT11_WFD_ADVERTISED_SERVICE_LIST(i32);
pub struct DOT11_WFD_ADVERTISEMENT_ID(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_APS2_SERVICE_TYPE_MAX_LENGTH: u32 = 21u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_ASP2_INSTANCE_NAME_MAX_LENGTH: u32 = 63u32;
pub struct DOT11_WFD_ATTRIBUTES(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub struct DOT11_WFD_CHANNEL(i32);
pub struct DOT11_WFD_CONFIGURATION_TIMEOUT(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DEVICE_AUTO_AVAILABILITY: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_CONCURRENT_OPERATION: u32 = 4u32;
pub struct DOT11_WFD_DEVICE_CAPABILITY_CONFIG(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_CONFIG_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_CLIENT_DISCOVERABILITY: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_DEVICE_LIMIT: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_INFRASTRUCTURE_MANAGED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_INVITATION_PROCEDURE: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_RESERVED_6: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_RESERVED_7: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DEVICE_CAPABILITY_SERVICE_DISCOVERY: u32 = 1u32;
pub struct DOT11_WFD_DEVICE_ENTRY(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DEVICE_HIGH_AVAILABILITY: u32 = 24u32;
pub struct DOT11_WFD_DEVICE_INFO(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DEVICE_INFO_REVISION_1: u32 = 1u32;
pub struct DOT11_WFD_DEVICE_LISTEN_CHANNEL(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DEVICE_LISTEN_CHANNEL_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DEVICE_NOT_DISCOVERABLE: u32 = 0u32;
pub struct DOT11_WFD_DEVICE_TYPE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DISCOVER_COMPLETE_MAX_LIST_SIZE: u32 = 128u32;
pub struct DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_WFD_DISCOVER_DEVICE_FILTER(i32);
pub struct DOT11_WFD_DISCOVER_REQUEST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_DISCOVER_REQUEST_REVISION_1: u32 = 1u32;
pub struct DOT11_WFD_DISCOVER_TYPE(i32);
pub struct DOT11_WFD_GO_INTENT(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_CROSS_CONNECTION_SUPPORTED: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_EAPOL_KEY_IP_ADDRESS_ALLOCATION_SUPPORTED: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_GROUP_LIMIT_REACHED: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_GROUP_OWNER: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_INTRABSS_DISTRIBUTION_SUPPORTED: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_IN_GROUP_FORMATION: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_PERSISTENT_GROUP: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_PERSISTENT_RECONNECT_SUPPORTED: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_GROUP_CAPABILITY_RESERVED_7: u32 = 128u32;
pub struct DOT11_WFD_GROUP_ID(i32);
pub struct DOT11_WFD_GROUP_JOIN_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_GROUP_JOIN_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_REVISION_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_REVISION_2: u32 = 2u32;
pub struct DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2(i32);
pub struct DOT11_WFD_GROUP_START_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_GROUP_START_PARAMETERS_REVISION_1: u32 = 1u32;
pub struct DOT11_WFD_INVITATION_FLAGS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_FROM_WLAN_CROSS_CONNECTION_POLICY: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_INFRASTRUCTURE_MANAGED_POLICY: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_NOT_MANAGED_INFRASTRUCTURE_CAPABLE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_WFD_COEXISTENCE_POLICY: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_MINOR_REASON_SUCCESS: u32 = 0u32;
pub struct DOT11_WFD_SCAN_TYPE(i32);
pub struct DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST_REVISION_1: u32 = 1u32;
pub struct DOT11_WFD_SERVICE_HASH_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_SERVICE_INFORMATION_MAX_LENGTH: u32 = 65535u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_SERVICE_NAME_MAX_LENGTH: u32 = 255u32;
pub struct DOT11_WFD_SESSION_ID(i32);
pub struct DOT11_WFD_SESSION_INFO(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_SESSION_INFO_MAX_LENGTH: u32 = 144u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_STATUS_FAILED_INCOMPATIBLE_PARAMETERS: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_STATUS_FAILED_INCOMPATIBLE_PROVISIONING_METHOD: u32 = 10u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_STATUS_FAILED_INFORMATION_IS_UNAVAILABLE: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_STATUS_FAILED_INVALID_PARAMETERS: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_STATUS_FAILED_LIMIT_REACHED: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_STATUS_FAILED_MATCHING_MAX_INTENT: u32 = 9u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_STATUS_FAILED_NO_COMMON_CHANNELS: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_STATUS_FAILED_PREVIOUS_PROTOCOL_ERROR: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_STATUS_FAILED_REJECTED_BY_USER: u32 = 11u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_STATUS_FAILED_UNABLE_TO_ACCOMODATE_REQUEST: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_STATUS_FAILED_UNKNOWN_WFD_GROUP: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_STATUS_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WFD_STATUS_SUCCESS_ACCEPTED_BY_USER: u32 = 12u32;
pub struct DOT11_WME_AC_PARAMETERS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WME_PACKET: u32 = 256u32;
pub struct DOT11_WME_UPDATE_IE(i32);
pub struct DOT11_WPA_TSC(i32);
pub struct DOT11_WPS_CONFIG_METHOD(i32);
pub struct DOT11_WPS_DEVICE_NAME(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WPS_DEVICE_NAME_MAX_LENGTH: u32 = 32u32;
pub struct DOT11_WPS_DEVICE_PASSWORD_ID(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WPS_MAX_MODEL_NAME_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WPS_MAX_MODEL_NUMBER_LENGTH: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WPS_MAX_PASSKEY_LENGTH: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WPS_VERSION_1_0: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DOT11_WPS_VERSION_2_0: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_Enhanced: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_NoP2PSupported: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_NotSupported: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_SingleFunctionSupported: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_Supported: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_AcsSupport_Missing: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_AcsSupport_NotNeeded: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_AcsSupport_Present: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_BridgeType_PciConventional: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_BridgeType_PciExpressDownstreamSwitchPort: u32 = 10u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_BridgeType_PciExpressEventCollector: u32 = 14u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_BridgeType_PciExpressRootPort: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_BridgeType_PciExpressToPciXBridge: u32 = 11u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_BridgeType_PciExpressTreatedAsPci: u32 = 13u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_BridgeType_PciExpressUpstreamSwitchPort: u32 = 9u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_BridgeType_PciX: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_BridgeType_PciXToExpressBridge: u32 = 12u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_100Mhz: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_133MHZ: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_66Mhz: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_100Mhz: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_133Mhz: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_66Mhz: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_100MHz: u32 = 10u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_133MHz: u32 = 11u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_66MHz: u32 = 9u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_100MHz: u32 = 14u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_133MHz: u32 = 15u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_66MHz: u32 = 13u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode_Conventional_Pci: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_Pci_Conventional_33MHz: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_CurrentSpeedAndMode_Pci_Conventional_66MHz: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_DeviceType_PciConventional: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_DeviceType_PciExpressEndpoint: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_DeviceType_PciExpressLegacyEndpoint: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_DeviceType_PciExpressRootComplexIntegratedEndpoint: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_DeviceType_PciExpressTreatedAsPci: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_DeviceType_PciX: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_InterruptType_LineBased: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_InterruptType_Msi: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_InterruptType_MsiX: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_SriovSupport_DidntGetVfBarSpace: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_SriovSupport_MissingAcs: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_SriovSupport_MissingPfDriver: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_SriovSupport_NoBusResource: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciDevice_SriovSupport_Ok: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_LinkSpeed_Five_Gbps: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_LinkSpeed_TwoAndHalf_Gbps: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_LinkWidth_By_1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_LinkWidth_By_12: u32 = 12u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_LinkWidth_By_16: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_LinkWidth_By_2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_LinkWidth_By_32: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_LinkWidth_By_4: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_LinkWidth_By_8: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_1024Bytes: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_128Bytes: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_2048Bytes: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_256Bytes: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_4096Bytes: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_512Bytes: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_Spec_Version_10: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciExpressDevice_Spec_Version_11: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_BusWidth_32Bits: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_BusWidth_64Bits: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_Conventional_33Mhz: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_Conventional_66Mhz: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_100Mhz: u32 = 9u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_133Mhz: u32 = 10u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_66Mhz: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_100Mhz: u32 = 12u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_133Mhz: u32 = 13u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_66Mhz: u32 = 11u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_100Mhz: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_133Mhz: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_66Mhz: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_100Mhz: u32 = 6u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_133Mhz: u32 = 7u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_66Mhz: u32 = 5u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_SecondaryInterface_PciConventional: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_SecondaryInterface_PciExpress: u32 = 3u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_SecondaryInterface_PciXMode1: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_SecondaryInterface_PciXMode2: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_Conventional_33Mhz: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_Conventional_66Mhz: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_133Mhz: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_266Mhz: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_533Mhz: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_66Mhz: u32 = 4u32;
pub struct Dot11AdHocManager(i32);
pub const GUID_AEPSERVICE_WIFIDIRECT_DEVICE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3425272444,
    data2: 40111,
    data3: 18728,
    data4: [153, 169, 24, 247, 194, 56, 19, 137],
};
pub const GUID_DEVINTERFACE_ASP_INFRA_DEVICE: ::windows_sys::core::GUID = ::windows_sys::GUID { data1: 4286724501, data2: 31346, data3: 19584, data4: [135, 87, 198, 126, 225, 61, 26, 73] };
pub const GUID_DEVINTERFACE_WIFIDIRECT_DEVICE: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 1134239919,
    data2: 35157,
    data3: 16475,
    data4: [153, 240, 166, 42, 240, 198, 141, 67],
};
pub struct IDot11AdHocInterface(i32);
pub struct IDot11AdHocInterfaceNotificationSink(i32);
pub struct IDot11AdHocManager(i32);
pub struct IDot11AdHocManagerNotificationSink(i32);
pub struct IDot11AdHocNetwork(i32);
pub struct IDot11AdHocNetworkNotificationSink(i32);
pub struct IDot11AdHocSecuritySettings(i32);
pub struct IEnumDot11AdHocInterfaces(i32);
pub struct IEnumDot11AdHocNetworks(i32);
pub struct IEnumDot11AdHocSecuritySettings(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_CODE_GROUP_SIZE: u32 = 4096u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_CODE_PUBLIC_BEGIN: u32 = 0u32;
pub struct L2_NOTIFICATION_DATA(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_SOURCE_ALL: u32 = 65535u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_SOURCE_DOT3_AUTO_CONFIG: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_SOURCE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_SOURCE_ONEX: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_SOURCE_SECURITY: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_SOURCE_WCM: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_SOURCE_WCM_CSP: u32 = 512u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_SOURCE_WFD: u32 = 1024u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_SOURCE_WLAN_ACM: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_SOURCE_WLAN_DEVICE_SERVICE: u32 = 2048u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_SOURCE_WLAN_HNWK: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_SOURCE_WLAN_IHV: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_SOURCE_WLAN_MSM: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_NOTIFICATION_SOURCE_WLAN_SECURITY: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_PROFILE_MAX_NAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_DOT11_AC_BASE: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_DOT11_MSM_BASE: u32 = 196608u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_DOT11_SECURITY_BASE: u32 = 262144u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_DOT3_AC_BASE: u32 = 393216u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_DOT3_MSM_BASE: u32 = 458752u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_GEN_BASE: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_GROUP_SIZE: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_IHV_BASE: u32 = 589824u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_ONEX_BASE: u32 = 327680u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_PROFILE_BASE: u32 = 524288u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_PROFILE_MISSING: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_RESERVED_BASE: u32 = 720896u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_UNKNOWN: u32 = 65537u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const L2_REASON_CODE_WIMAX_BASE: u32 = 655360u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const MAX_NUM_SUPPORTED_RATES: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const MAX_NUM_SUPPORTED_RATES_V2: u32 = 255u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const NDIS_PACKET_TYPE_802_11_ALL_MULTICAST_DATA: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const NDIS_PACKET_TYPE_802_11_BROADCAST_DATA: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const NDIS_PACKET_TYPE_802_11_DIRECTED_DATA: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const NDIS_PACKET_TYPE_802_11_MULTICAST_DATA: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const NDIS_PACKET_TYPE_802_11_PROMISCUOUS_DATA: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_AP_JOIN_REQUEST: u32 = 218170205u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_ATIM_WINDOW: u32 = 218170122u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_BEACON_PERIOD: u32 = 218170139u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CCA_MODE_SUPPORTED: u32 = 218170166u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CCA_WATCHDOG_COUNT_MAX: u32 = 218170170u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CCA_WATCHDOG_COUNT_MIN: u32 = 218170172u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CCA_WATCHDOG_TIMER_MAX: u32 = 218170169u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CCA_WATCHDOG_TIMER_MIN: u32 = 218170171u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CFP_MAX_DURATION: u32 = 218170136u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CFP_PERIOD: u32 = 218170135u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CF_POLLABLE: u32 = 218170134u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CHANNEL_AGILITY_ENABLED: u32 = 218170184u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CHANNEL_AGILITY_PRESENT: u32 = 218170183u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_COUNTERS_ENTRY: u32 = 218170149u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_COUNTRY_STRING: u32 = 218170188u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_ADDRESS: u32 = 218171138u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_CCA_MODE: u32 = 218170167u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_CHANNEL: u32 = 218170165u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_CHANNEL_NUMBER: u32 = 218170159u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_DWELL_TIME: u32 = 218170161u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_FREQUENCY: u32 = 218170178u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_INDEX: u32 = 218170164u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_OFFLOAD_CAPABILITY: u32 = 218170113u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_OPERATION_MODE: u32 = 218170120u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_OPTIONAL_CAPABILITY: u32 = 218170131u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_PACKET_FILTER: u32 = 218170121u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_PATTERN: u32 = 218170163u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_PHY_TYPE: u32 = 218170124u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_REG_DOMAIN: u32 = 218170151u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_RX_ANTENNA: u32 = 218170155u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_SET: u32 = 218170162u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_TX_ANTENNA: u32 = 218170153u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_CURRENT_TX_POWER_LEVEL: u32 = 218170157u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_DEFAULT_WEP_OFFLOAD: u32 = 218170116u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_DEFAULT_WEP_UPLOAD: u32 = 218170117u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_DIVERSITY_SELECTION_RX: u32 = 218170176u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_DIVERSITY_SUPPORT: u32 = 218170154u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_DSSS_OFDM_OPTION_ENABLED: u32 = 218170209u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_DSSS_OFDM_OPTION_IMPLEMENTED: u32 = 218170208u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_DTIM_PERIOD: u32 = 218170140u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_ED_THRESHOLD: u32 = 218170168u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_EHCC_CAPABILITY_ENABLED: u32 = 218170193u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_EHCC_CAPABILITY_IMPLEMENTED: u32 = 218170192u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_EHCC_NUMBER_OF_CHANNELS_FAMILY_INDEX: u32 = 218170191u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_EHCC_PRIME_RADIX: u32 = 218170190u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_ERP_PBCC_OPTION_ENABLED: u32 = 218170207u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_ERP_PBCC_OPTION_IMPLEMENTED: u32 = 218170206u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_FRAGMENTATION_THRESHOLD: u32 = 218170146u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_FREQUENCY_BANDS_SUPPORTED: u32 = 218170180u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_HOPPING_PATTERN: u32 = 218170199u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_HOP_ALGORITHM_ADOPTED: u32 = 218170194u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_HOP_MODULUS: u32 = 218170197u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_HOP_OFFSET: u32 = 218170198u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_HOP_TIME: u32 = 218170158u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_HR_CCA_MODE_SUPPORTED: u32 = 218170185u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_JOIN_REQUEST: u32 = 218170125u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_LONG_RETRY_LIMIT: u32 = 218170145u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_MAC_ADDRESS: u32 = 218170142u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_MAXIMUM_LIST_SIZE: u32 = 218171141u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_MAX_DWELL_TIME: u32 = 218170160u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_MAX_MAC_ADDRESS_STATES: u32 = 218170212u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_MAX_RECEIVE_LIFETIME: u32 = 218170148u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_MAX_TRANSMIT_MSDU_LIFETIME: u32 = 218170147u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_MEDIUM_OCCUPANCY_LIMIT: u32 = 218170133u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_MPDU_MAX_LENGTH: u32 = 218170118u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_MULTICAST_LIST: u32 = 218171140u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY: u32 = 218170189u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY_ENABLED: u32 = 218170187u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY_IMPLEMENTED: u32 = 218170186u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_NDIS_START: u32 = 218170112u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_NIC_POWER_STATE: u32 = 218170129u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_NIC_SPECIFIC_EXTENSION: u32 = 218170204u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_NUMBER_OF_HOPPING_SETS: u32 = 218170196u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_OFFLOAD_CAPABILITY: u32 = 218170112u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_OPERATIONAL_RATE_SET: u32 = 218170138u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_OPERATION_MODE_CAPABILITY: u32 = 218170119u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_OPTIONAL_CAPABILITY: u32 = 218170130u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_PBCC_OPTION_IMPLEMENTED: u32 = 218170182u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_PERMANENT_ADDRESS: u32 = 218171139u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_POWER_MGMT_MODE: u32 = 218170137u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_PRIVATE_OIDS_START: u32 = 218171136u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_QOS_TX_DURATION: u32 = 218170219u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_QOS_TX_MEDIUM_TIME: u32 = 218170220u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_QOS_TX_QUEUES_SUPPORTED: u32 = 218170218u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_RANDOM_TABLE_FIELD_NUMBER: u32 = 218170200u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_RANDOM_TABLE_FLAG: u32 = 218170195u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_RECV_SENSITIVITY_LIST: u32 = 218170213u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_REG_DOMAINS_SUPPORT_VALUE: u32 = 218170173u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_RESET_REQUEST: u32 = 218170128u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_RF_USAGE: u32 = 218170203u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_RSSI_RANGE: u32 = 218170202u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_RTS_THRESHOLD: u32 = 218170143u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_SCAN_REQUEST: u32 = 218170123u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_SHORT_PREAMBLE_OPTION_IMPLEMENTED: u32 = 218170181u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_SHORT_RETRY_LIMIT: u32 = 218170144u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_SHORT_SLOT_TIME_OPTION_ENABLED: u32 = 218170211u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_SHORT_SLOT_TIME_OPTION_IMPLEMENTED: u32 = 218170210u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_START_REQUEST: u32 = 218170126u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_STATION_ID: u32 = 218170132u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_SUPPORTED_DATA_RATES_VALUE: u32 = 218170177u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_SUPPORTED_DSSS_CHANNEL_LIST: u32 = 218170222u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_SUPPORTED_OFDM_FREQUENCY_LIST: u32 = 218170221u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_SUPPORTED_PHY_TYPES: u32 = 218170150u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_SUPPORTED_POWER_LEVELS: u32 = 218170156u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_SUPPORTED_RX_ANTENNA: u32 = 218170175u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_SUPPORTED_TX_ANTENNA: u32 = 218170174u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_TEMP_TYPE: u32 = 218170152u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_TI_THRESHOLD: u32 = 218170179u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_UPDATE_IE: u32 = 218170127u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_WEP_ICV_ERROR_COUNT: u32 = 218170141u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_WEP_OFFLOAD: u32 = 218170114u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_WEP_UPLOAD: u32 = 218170115u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_WME_AC_PARAMETERS: u32 = 218170216u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_WME_ENABLED: u32 = 218170215u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_WME_IMPLEMENTED: u32 = 218170214u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_WME_UPDATE_IE: u32 = 218170217u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const OID_DOT11_WPA_TSC: u32 = 218170201u32;
pub struct ONEX_AUTH_IDENTITY(i32);
pub struct ONEX_AUTH_PARAMS(i32);
pub struct ONEX_AUTH_RESTART_REASON(i32);
pub struct ONEX_AUTH_STATUS(i32);
pub struct ONEX_EAP_ERROR(i32);
pub struct ONEX_EAP_METHOD_BACKEND_SUPPORT(i32);
pub struct ONEX_NOTIFICATION_TYPE(i32);
pub struct ONEX_REASON_CODE(i32);
pub struct ONEX_RESULT_UPDATE_DATA(i32);
pub struct ONEX_STATUS(i32);
pub struct ONEX_USER_INFO(i32);
pub struct ONEX_VARIABLE_BLOB(i32);
pub struct WFDSVC_CONNECTION_CAPABILITY(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WFDSVC_CONNECTION_CAPABILITY_CLIENT: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WFDSVC_CONNECTION_CAPABILITY_GO: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WFDSVC_CONNECTION_CAPABILITY_NEW: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WFD_API_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WFD_API_VERSION_1_0: u32 = 1u32;
pub struct WFD_GROUP_ID(i32);
pub struct WFD_OPEN_SESSION_COMPLETE_CALLBACK(i32);
pub struct WFD_ROLE_TYPE(i32);
pub struct WLAN_ADHOC_NETWORK_STATE(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_API_VERSION: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_API_VERSION_1_0: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_API_VERSION_2_0: u32 = 2u32;
pub struct WLAN_ASSOCIATION_ATTRIBUTES(i32);
pub struct WLAN_AUTH_CIPHER_PAIR_LIST(i32);
pub struct WLAN_AUTOCONF_OPCODE(i32);
pub struct WLAN_AVAILABLE_NETWORK(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_AVAILABLE_NETWORK_ANQP_SUPPORTED: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_AVAILABLE_NETWORK_AUTO_CONNECT_FAILED: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_AVAILABLE_NETWORK_CONNECTED: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_AVAILABLE_NETWORK_CONSOLE_USER_PROFILE: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_AVAILABLE_NETWORK_HAS_PROFILE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_DOMAIN: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_ENABLED: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_ROAMING: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_AVAILABLE_NETWORK_INCLUDE_ALL_ADHOC_PROFILES: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_AVAILABLE_NETWORK_INCLUDE_ALL_MANUAL_HIDDEN_PROFILES: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_AVAILABLE_NETWORK_INTERWORKING_SUPPORTED: u32 = 8u32;
pub struct WLAN_AVAILABLE_NETWORK_LIST(i32);
pub struct WLAN_AVAILABLE_NETWORK_LIST_V2(i32);
pub struct WLAN_AVAILABLE_NETWORK_V2(i32);
pub struct WLAN_BSS_ENTRY(i32);
pub struct WLAN_BSS_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_CONNECTION_ADHOC_JOIN_ONLY: u32 = 2u32;
pub struct WLAN_CONNECTION_ATTRIBUTES(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_CONNECTION_EAPOL_PASSTHROUGH: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_CONNECTION_HIDDEN_NETWORK: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_CONNECTION_IGNORE_PRIVACY_BIT: u32 = 4u32;
pub struct WLAN_CONNECTION_MODE(i32);
pub struct WLAN_CONNECTION_NOTIFICATION_DATA(i32);
pub struct WLAN_CONNECTION_NOTIFICATION_FLAGS(i32);
pub struct WLAN_CONNECTION_PARAMETERS(i32);
pub struct WLAN_CONNECTION_PARAMETERS_V2(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE_CONNECTION_MODE_AUTO: u32 = 32u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE_OVERWRITE_EXISTING: u32 = 64u32;
pub struct WLAN_COUNTRY_OR_REGION_STRING_LIST(i32);
pub struct WLAN_DEVICE_SERVICE_GUID_LIST(i32);
pub struct WLAN_DEVICE_SERVICE_NOTIFICATION_DATA(i32);
pub struct WLAN_FILTER_LIST_TYPE(i32);
pub struct WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS(i32);
pub struct WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE(i32);
pub struct WLAN_HOSTED_NETWORK_NOTIFICATION_CODE(i32);
pub struct WLAN_HOSTED_NETWORK_OPCODE(i32);
pub struct WLAN_HOSTED_NETWORK_PEER_AUTH_STATE(i32);
pub struct WLAN_HOSTED_NETWORK_PEER_STATE(i32);
pub struct WLAN_HOSTED_NETWORK_RADIO_STATE(i32);
pub struct WLAN_HOSTED_NETWORK_REASON(i32);
pub struct WLAN_HOSTED_NETWORK_SECURITY_SETTINGS(i32);
pub struct WLAN_HOSTED_NETWORK_STATE(i32);
pub struct WLAN_HOSTED_NETWORK_STATE_CHANGE(i32);
pub struct WLAN_HOSTED_NETWORK_STATUS(i32);
pub struct WLAN_IHV_CONTROL_TYPE(i32);
pub struct WLAN_INTERFACE_CAPABILITY(i32);
pub struct WLAN_INTERFACE_INFO(i32);
pub struct WLAN_INTERFACE_INFO_LIST(i32);
pub struct WLAN_INTERFACE_STATE(i32);
pub struct WLAN_INTERFACE_TYPE(i32);
pub struct WLAN_INTF_OPCODE(i32);
pub struct WLAN_MAC_FRAME_STATISTICS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_MAX_NAME_LENGTH: u32 = 256u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_MAX_PHY_INDEX: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_MAX_PHY_TYPE_NUMBER: u32 = 8u32;
pub struct WLAN_MSM_NOTIFICATION_DATA(i32);
pub struct WLAN_NOTIFICATION_ACM(i32);
pub struct WLAN_NOTIFICATION_CALLBACK(i32);
pub struct WLAN_NOTIFICATION_MSM(i32);
pub struct WLAN_NOTIFICATION_SECURITY(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_NOTIFICATION_SOURCE_ACM: u32 = 8u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_NOTIFICATION_SOURCE_ALL: u32 = 65535u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_NOTIFICATION_SOURCE_DEVICE_SERVICE: u32 = 2048u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_NOTIFICATION_SOURCE_HNWK: u32 = 128u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_NOTIFICATION_SOURCE_IHV: u32 = 64u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_NOTIFICATION_SOURCE_MSM: u32 = 16u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_NOTIFICATION_SOURCE_NONE: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_NOTIFICATION_SOURCE_ONEX: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_NOTIFICATION_SOURCE_SECURITY: u32 = 32u32;
pub struct WLAN_OPCODE_VALUE_TYPE(i32);
pub struct WLAN_OPERATIONAL_STATE(i32);
pub struct WLAN_PHY_FRAME_STATISTICS(i32);
pub struct WLAN_PHY_RADIO_STATE(i32);
pub struct WLAN_POWER_SETTING(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_PROFILE_CONNECTION_MODE_AUTO: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_PROFILE_CONNECTION_MODE_SET_BY_CLIENT: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_PROFILE_GET_PLAINTEXT_KEY: u32 = 4u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_PROFILE_GROUP_POLICY: u32 = 1u32;
pub struct WLAN_PROFILE_INFO(i32);
pub struct WLAN_PROFILE_INFO_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_PROFILE_USER: u32 = 2u32;
pub struct WLAN_RADIO_STATE(i32);
pub struct WLAN_RATE_SET(i32);
pub struct WLAN_RAW_DATA(i32);
pub struct WLAN_RAW_DATA_LIST(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_AC_BASE: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_AC_CONNECT_BASE: u32 = 163840u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_AC_END: u32 = 196607u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_ADHOC_SECURITY_FAILURE: u32 = 229386u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_AP_PROFILE_NOT_ALLOWED: u32 = 163856u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_AP_PROFILE_NOT_ALLOWED_FOR_CLIENT: u32 = 163855u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_AP_STARTING_FAILURE: u32 = 229395u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_ASSOCIATION_FAILURE: u32 = 229378u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_ASSOCIATION_TIMEOUT: u32 = 229379u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_AUTO_AP_PROFILE_NOT_ALLOWED: u32 = 524313u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_AUTO_CONNECTION_NOT_ALLOWED: u32 = 524314u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_AUTO_SWITCH_SET_FOR_ADHOC: u32 = 524304u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_AUTO_SWITCH_SET_FOR_MANUAL_CONNECTION: u32 = 524305u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_BAD_MAX_NUMBER_OF_CLIENTS_FOR_AP: u32 = 524310u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_BASE: u32 = 131072u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_BSS_TYPE_NOT_ALLOWED: u32 = 163845u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_BSS_TYPE_UNMATCH: u32 = 196611u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_CONFLICT_SECURITY: u32 = 524299u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_CONNECT_CALL_FAIL: u32 = 163849u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_DATARATE_UNMATCH: u32 = 196613u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_DISCONNECT_TIMEOUT: u32 = 229391u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_DRIVER_DISCONNECTED: u32 = 229387u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_DRIVER_OPERATION_FAILURE: u32 = 229388u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_GP_DENIED: u32 = 163843u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_HOTSPOT2_PROFILE_DENIED: u32 = 163857u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_HOTSPOT2_PROFILE_NOT_ALLOWED: u32 = 524315u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_IHV_CONNECTIVITY_NOT_SUPPORTED: u32 = 524309u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_IHV_NOT_AVAILABLE: u32 = 229389u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_IHV_NOT_RESPONDING: u32 = 229390u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_IHV_OUI_MISMATCH: u32 = 524296u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_IHV_OUI_MISSING: u32 = 524297u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_IHV_SECURITY_NOT_SUPPORTED: u32 = 524295u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_IHV_SECURITY_ONEX_MISSING: u32 = 524306u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_IHV_SETTINGS_MISSING: u32 = 524298u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_INTERNAL_FAILURE: u32 = 229392u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_INVALID_ADHOC_CONNECTION_MODE: u32 = 524302u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_INVALID_BSS_TYPE: u32 = 524301u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_INVALID_CHANNEL: u32 = 524311u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_INVALID_PHY_TYPE: u32 = 524293u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_INVALID_PROFILE_NAME: u32 = 524291u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_INVALID_PROFILE_SCHEMA: u32 = 524289u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_INVALID_PROFILE_TYPE: u32 = 524292u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_IN_BLOCKED_LIST: u32 = 163847u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_IN_FAILED_LIST: u32 = 163846u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_KEY_MISMATCH: u32 = 163853u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_AUTH_START_TIMEOUT: u32 = 294914u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_AUTH_SUCCESS_TIMEOUT: u32 = 294915u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_AUTH_WCN_COMPLETED: u32 = 294937u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_BASE: u32 = 262144u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_CANCELLED: u32 = 294929u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_DISCOVERY: u32 = 262165u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_MFP_NW_NIC: u32 = 262181u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_NETWORK: u32 = 262162u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_NIC: u32 = 262163u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE: u32 = 262164u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_AUTH: u32 = 262174u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_CIPHER: u32 = 262175u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_SAFE_MODE_NIC: u32 = 262177u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_SAFE_MODE_NW: u32 = 262178u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_CONNECT_BASE: u32 = 294912u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_DOWNGRADE_DETECTED: u32 = 294931u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_END: u32 = 327679u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_FORCED_FAILURE: u32 = 294933u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_GRP_KEY: u32 = 294925u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_KEY_DATA: u32 = 294924u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_MGMT_GRP_KEY: u32 = 294939u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_KEY_FORMAT: u32 = 294930u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_KEY_START_TIMEOUT: u32 = 294916u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_KEY_SUCCESS_TIMEOUT: u32 = 294917u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_M2_MISSING_IE: u32 = 294936u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_M2_MISSING_KEY_DATA: u32 = 294935u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_GRP_KEY: u32 = 294920u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_IE: u32 = 294919u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_KEY_DATA: u32 = 294918u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_MGMT_GRP_KEY: u32 = 294938u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_M3_TOO_MANY_RSNIE: u32 = 294934u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_MAX: u32 = 327679u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_MIN: u32 = 262144u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_MIXED_CELL: u32 = 262169u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_NIC_FAILURE: u32 = 294928u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_NO_AUTHENTICATOR: u32 = 294927u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_NO_PAIRWISE_KEY: u32 = 294923u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PEER_INDICATED_INSECURE: u32 = 294926u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_AUTH_TIMERS_INVALID: u32 = 262170u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_DUPLICATE_AUTH_CIPHER: u32 = 262151u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_AUTH_CIPHER: u32 = 262153u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_GKEY_INTV: u32 = 262171u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_KEY_INDEX: u32 = 262145u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_MODE: u32 = 262156u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_SIZE: u32 = 262157u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_TTL: u32 = 262158u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PREAUTH_MODE: u32 = 262159u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PREAUTH_THROTTLE: u32 = 262160u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEYMATERIAL_CHAR: u32 = 262167u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEY_LENGTH: u32 = 262147u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEY_UNMAPPED_CHAR: u32 = 262173u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_NO_AUTH_CIPHER_SPECIFIED: u32 = 262149u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_ONEX_DISABLED: u32 = 262154u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_ONEX_ENABLED: u32 = 262155u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PASSPHRASE_CHAR: u32 = 262166u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PREAUTH_ONLY_ENABLED: u32 = 262161u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PSK_LENGTH: u32 = 262148u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PSK_PRESENT: u32 = 262146u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_RAWDATA_INVALID: u32 = 262152u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_SAFE_MODE: u32 = 262176u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_TOO_MANY_AUTH_CIPHER_SPECIFIED: u32 = 262150u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_UNSUPPORTED_AUTH: u32 = 262179u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_UNSUPPORTED_CIPHER: u32 = 262180u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_WRONG_KEYTYPE: u32 = 262168u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PR_IE_MATCHING: u32 = 294921u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_PSK_MISMATCH_SUSPECTED: u32 = 294932u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_SEC_IE_MATCHING: u32 = 294922u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_TRANSITION_NETWORK: u32 = 262172u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSMSEC_UI_REQUEST_FAILURE: u32 = 294913u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSM_BASE: u32 = 196608u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSM_CONNECT_BASE: u32 = 229376u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSM_END: u32 = 262143u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_MSM_SECURITY_MISSING: u32 = 524294u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_NETWORK_NOT_AVAILABLE: u32 = 163851u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_NETWORK_NOT_COMPATIBLE: u32 = 131073u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_NON_BROADCAST_SET_FOR_ADHOC: u32 = 524303u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_NOT_VISIBLE: u32 = 163842u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_NO_AUTO_CONNECTION: u32 = 163841u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_NO_VISIBLE_AP: u32 = 229396u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_OPERATION_MODE_NOT_SUPPORTED: u32 = 524312u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_PHY_TYPE_UNMATCH: u32 = 196612u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_PRE_SECURITY_FAILURE: u32 = 229380u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_PROFILE_BASE: u32 = 524288u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_PROFILE_CHANGED_OR_DELETED: u32 = 163852u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_PROFILE_CONNECT_BASE: u32 = 557056u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_PROFILE_END: u32 = 589823u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_PROFILE_MISSING: u32 = 524290u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_PROFILE_NOT_COMPATIBLE: u32 = 131074u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_PROFILE_SSID_INVALID: u32 = 524307u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_RANGE_SIZE: u32 = 65536u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_RESERVED_BASE: u32 = 720896u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_RESERVED_END: u32 = 786431u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_ROAMING_FAILURE: u32 = 229384u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_ROAMING_SECURITY_FAILURE: u32 = 229385u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_SCAN_CALL_FAIL: u32 = 163850u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_SECURITY_FAILURE: u32 = 229382u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_SECURITY_MISSING: u32 = 524300u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_SECURITY_TIMEOUT: u32 = 229383u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_SSID_LIST_TOO_LONG: u32 = 163848u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_START_SECURITY_FAILURE: u32 = 229381u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_SUCCESS: u32 = 0u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_TOO_MANY_SECURITY_ATTEMPTS: u32 = 229394u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_TOO_MANY_SSID: u32 = 524308u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_UI_REQUEST_TIMEOUT: u32 = 229393u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_UNKNOWN: u32 = 65537u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_UNSUPPORTED_SECURITY_SET: u32 = 196610u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_UNSUPPORTED_SECURITY_SET_BY_OS: u32 = 196609u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_USER_CANCELLED: u32 = 229377u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_USER_DENIED: u32 = 163844u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_REASON_CODE_USER_NOT_RESPOND: u32 = 163854u32;
pub struct WLAN_SECURABLE_OBJECT(i32);
pub struct WLAN_SECURITY_ATTRIBUTES(i32);
pub struct WLAN_SET_EAPHOST_FLAGS(i32);
pub struct WLAN_STATISTICS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_UI_API_INITIAL_VERSION: u32 = 1u32;
#[doc = "*Required features: `Win32_NetworkManagement_WiFi`*"]
pub const WLAN_UI_API_VERSION: u32 = 1u32;
pub struct WL_DISPLAY_PAGES(i32);
pub struct _DOT11_WME_AC_PARAMTERS_LIST(i32);
