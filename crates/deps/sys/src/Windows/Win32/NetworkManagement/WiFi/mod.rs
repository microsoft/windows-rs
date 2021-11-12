#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDCancelOpenSession(hsessionhandle: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDCloseHandle(hclienthandle: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDCloseSession(hsessionhandle: super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDOpenHandle(dwclientversion: u32, pdwnegotiatedversion: *mut u32, phclienthandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDOpenLegacySession(hclienthandle: super::super::Foundation::HANDLE, plegacymacaddress: *const *const u8, phsessionhandle: *mut super::super::Foundation::HANDLE, pguidsessioninterface: *mut ::windows_sys::core::GUID) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WFDStartOpenSession(hclienthandle: super::super::Foundation::HANDLE, pdeviceaddress: *const *const u8, pvcontext: *const ::core::ffi::c_void, pfncallback: WFD_OPEN_SESSION_COMPLETE_CALLBACK, phsessionhandle: *mut super::super::Foundation::HANDLE) -> u32;
    pub fn WFDUpdateDeviceVisibility(pdeviceaddress: *const *const u8) -> u32;
    pub fn WlanAllocateMemory(dwmemorysize: u32) -> *mut ::core::ffi::c_void;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanCloseHandle(hclienthandle: super::super::Foundation::HANDLE, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn WlanConnect(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, pconnectionparameters: *const WLAN_CONNECTION_PARAMETERS, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
    pub fn WlanConnect2(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, pconnectionparameters: *const WLAN_CONNECTION_PARAMETERS_V2, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanDeleteProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanDeviceServiceCommand(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, pdeviceserviceguid: *const ::windows_sys::core::GUID, dwopcode: u32, dwinbuffersize: u32, pinbuffer: *const ::core::ffi::c_void, dwoutbuffersize: u32, poutbuffer: *mut ::core::ffi::c_void, pdwbytesreturned: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanDisconnect(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanEnumInterfaces(hclienthandle: super::super::Foundation::HANDLE, preserved: *mut ::core::ffi::c_void, ppinterfacelist: *mut *mut WLAN_INTERFACE_INFO_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanExtractPsdIEDataList(hclienthandle: super::super::Foundation::HANDLE, dwiedatasize: u32, prawiedata: *const u8, strformat: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void, pppsdiedatalist: *mut *mut WLAN_RAW_DATA_LIST) -> u32;
    pub fn WlanFreeMemory(pmemory: *const ::core::ffi::c_void);
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetAvailableNetworkList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, dwflags: u32, preserved: *mut ::core::ffi::c_void, ppavailablenetworklist: *mut *mut WLAN_AVAILABLE_NETWORK_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetAvailableNetworkList2(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, dwflags: u32, preserved: *mut ::core::ffi::c_void, ppavailablenetworklist: *mut *mut WLAN_AVAILABLE_NETWORK_LIST_V2) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetFilterList(hclienthandle: super::super::Foundation::HANDLE, wlanfilterlisttype: WLAN_FILTER_LIST_TYPE, preserved: *mut ::core::ffi::c_void, ppnetworklist: *mut *mut DOT11_NETWORK_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetInterfaceCapability(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, preserved: *mut ::core::ffi::c_void, ppcapability: *mut *mut WLAN_INTERFACE_CAPABILITY) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetNetworkBssList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, pdot11ssid: *const DOT11_SSID, dot11bsstype: DOT11_BSS_TYPE, bsecurityenabled: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void, ppwlanbsslist: *mut *mut WLAN_BSS_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void, pstrprofilexml: *mut super::super::Foundation::PWSTR, pdwflags: *mut u32, pdwgrantedaccess: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetProfileCustomUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetProfileList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, preserved: *mut ::core::ffi::c_void, ppprofilelist: *mut *mut WLAN_PROFILE_INFO_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetSecuritySettings(hclienthandle: super::super::Foundation::HANDLE, securableobject: WLAN_SECURABLE_OBJECT, pvaluetype: *mut WLAN_OPCODE_VALUE_TYPE, pstrcurrentsddl: *mut super::super::Foundation::PWSTR, pdwgrantedaccess: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanGetSupportedDeviceServices(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, ppdevsvcguidlist: *mut *mut WLAN_DEVICE_SERVICE_GUID_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkForceStart(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkForceStop(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkInitSettings(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkQueryProperty(hclienthandle: super::super::Foundation::HANDLE, opcode: WLAN_HOSTED_NETWORK_OPCODE, pdwdatasize: *mut u32, ppvdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkQuerySecondaryKey(hclienthandle: super::super::Foundation::HANDLE, pdwkeylength: *mut u32, ppuckeydata: *mut *mut u8, pbispassphrase: *mut super::super::Foundation::BOOL, pbpersistent: *mut super::super::Foundation::BOOL, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkQueryStatus(hclienthandle: super::super::Foundation::HANDLE, ppwlanhostednetworkstatus: *mut *mut WLAN_HOSTED_NETWORK_STATUS, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkRefreshSecuritySettings(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkSetProperty(hclienthandle: super::super::Foundation::HANDLE, opcode: WLAN_HOSTED_NETWORK_OPCODE, dwdatasize: u32, pvdata: *const ::core::ffi::c_void, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkSetSecondaryKey(hclienthandle: super::super::Foundation::HANDLE, dwkeylength: u32, puckeydata: *const u8, bispassphrase: super::super::Foundation::BOOL, bpersistent: super::super::Foundation::BOOL, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkStartUsing(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanHostedNetworkStopUsing(hclienthandle: super::super::Foundation::HANDLE, pfailreason: *mut WLAN_HOSTED_NETWORK_REASON, pvreserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanIhvControl(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, r#type: WLAN_IHV_CONTROL_TYPE, dwinbuffersize: u32, pinbuffer: *const ::core::ffi::c_void, dwoutbuffersize: u32, poutbuffer: *mut ::core::ffi::c_void, pdwbytesreturned: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanOpenHandle(dwclientversion: u32, preserved: *mut ::core::ffi::c_void, pdwnegotiatedversion: *mut u32, phclienthandle: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanQueryAutoConfigParameter(hclienthandle: super::super::Foundation::HANDLE, opcode: WLAN_AUTOCONF_OPCODE, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanQueryInterface(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, opcode: WLAN_INTF_OPCODE, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut ::core::ffi::c_void, pwlanopcodevaluetype: *mut WLAN_OPCODE_VALUE_TYPE) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanReasonCodeToString(dwreasoncode: u32, dwbuffersize: u32, pstringbuffer: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRegisterDeviceServiceNotification(hclienthandle: super::super::Foundation::HANDLE, pdevsvcguidlist: *const WLAN_DEVICE_SERVICE_GUID_LIST) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRegisterNotification(hclienthandle: super::super::Foundation::HANDLE, dwnotifsource: u32, bignoreduplicate: super::super::Foundation::BOOL, funccallback: WLAN_NOTIFICATION_CALLBACK, pcallbackcontext: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void, pdwprevnotifsource: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRegisterVirtualStationNotification(hclienthandle: super::super::Foundation::HANDLE, bregister: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanRenameProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, stroldprofilename: super::super::Foundation::PWSTR, strnewprofilename: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSaveTemporaryProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, stralluserprofilesecurity: super::super::Foundation::PWSTR, dwflags: u32, boverwrite: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanScan(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, pdot11ssid: *const DOT11_SSID, piedata: *const WLAN_RAW_DATA, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetAutoConfigParameter(hclienthandle: super::super::Foundation::HANDLE, opcode: WLAN_AUTOCONF_OPCODE, dwdatasize: u32, pdata: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetFilterList(hclienthandle: super::super::Foundation::HANDLE, wlanfilterlisttype: WLAN_FILTER_LIST_TYPE, pnetworklist: *const DOT11_NETWORK_LIST, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetInterface(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, opcode: WLAN_INTF_OPCODE, dwdatasize: u32, pdata: *const ::core::ffi::c_void, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfile(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, dwflags: u32, strprofilexml: super::super::Foundation::PWSTR, stralluserprofilesecurity: super::super::Foundation::PWSTR, boverwrite: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void, pdwreasoncode: *mut u32) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfileCustomUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, dwdatasize: u32, pdata: *const u8, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security_ExtensibleAuthenticationProtocol"))]
    pub fn WlanSetProfileEapUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, eaptype: super::super::Security::ExtensibleAuthenticationProtocol::EAP_METHOD_TYPE, dwflags: WLAN_SET_EAPHOST_FLAGS, dweapuserdatasize: u32, pbeapuserdata: *const u8, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfileEapXmlUserData(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, dwflags: WLAN_SET_EAPHOST_FLAGS, streapxmluserdata: super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfileList(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, dwitems: u32, strprofilenames: *const super::super::Foundation::PWSTR, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetProfilePosition(hclienthandle: super::super::Foundation::HANDLE, pinterfaceguid: *const ::windows_sys::core::GUID, strprofilename: super::super::Foundation::PWSTR, dwposition: u32, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetPsdIEDataList(hclienthandle: super::super::Foundation::HANDLE, strformat: super::super::Foundation::PWSTR, ppsdiedatalist: *const WLAN_RAW_DATA_LIST, preserved: *mut ::core::ffi::c_void) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanSetSecuritySettings(hclienthandle: super::super::Foundation::HANDLE, securableobject: WLAN_SECURABLE_OBJECT, strmodifiedsddl: super::super::Foundation::PWSTR) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn WlanUIEditProfile(dwclientversion: u32, wstrprofilename: super::super::Foundation::PWSTR, pinterfaceguid: *const ::windows_sys::core::GUID, hwnd: super::super::Foundation::HWND, wlstartpage: WL_DISPLAY_PAGES, preserved: *mut ::core::ffi::c_void, pwlanreasoncode: *mut u32) -> u32;
}
#[repr(transparent)]
pub struct CH_DESCRIPTION_TYPE(pub i32);
pub const ch_description_type_logical: CH_DESCRIPTION_TYPE = CH_DESCRIPTION_TYPE(1i32);
pub const ch_description_type_center_frequency: CH_DESCRIPTION_TYPE = CH_DESCRIPTION_TYPE(2i32);
pub const ch_description_type_phy_specific: CH_DESCRIPTION_TYPE = CH_DESCRIPTION_TYPE(3i32);
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_AccessPointBssid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 19u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_ChallengeAep: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 21u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_DevnodeAep: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 23u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_HostName_ResolutionMode: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 25u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_PinSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 29u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_RtspTcpConnectionParametersSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 30u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_SinkHostName: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 20u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_SinkIpAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 26u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_StreamSecuritySupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 18u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_InfraCast_Supported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 17u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
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
pub const DEVPKEY_WiFiDirect_DeviceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 1u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_DeviceAddressCopy: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 13u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_FoundWsbService: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 24u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_GroupId: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 4u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_InformationElements: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 12u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_InterfaceAddress: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 2u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_InterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 3u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_IsConnected: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 5u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_IsDMGCapable: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 22u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_IsLegacyDevice: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 7u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_IsMiracastLCPSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 9u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_IsRecentlyAssociated: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 14u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_IsVisible: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 6u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_LinkQuality: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 28u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_MiracastVersion: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 8u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_Miracast_SessionMgmtControlPort: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 31u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_NoMiracastAutoProject: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 16u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_RtspTcpConnectionParametersSupported: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 32u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_Service_Aeps: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 15u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_Services: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 10u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_SupportedChannelList: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 11u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFiDirect_TransientAssociation: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID { data1: 352752477, data2: 58343, data3: 17679, data4: [134, 55, 130, 35, 62, 190, 95, 110] },
    pid: 27u32,
};
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub const DEVPKEY_WiFi_InterfaceGuid: super::super::UI::Shell::PropertiesSystem::PROPERTYKEY = super::super::UI::Shell::PropertiesSystem::PROPERTYKEY {
    fmtid: ::windows_sys::GUID {
        data1: 4010895339,
        data2: 52220,
        data3: 17217,
        data4: [165, 104, 167, 201, 26, 104, 152, 44],
    },
    pid: 2u32,
};
pub const DISCOVERY_FILTER_BITMASK_ANY: u32 = 15u32;
pub const DISCOVERY_FILTER_BITMASK_DEVICE: u32 = 1u32;
pub const DISCOVERY_FILTER_BITMASK_GO: u32 = 2u32;
#[repr(C)]
pub struct DOT11_ACCESSNETWORKOPTIONS(i32);
#[repr(transparent)]
pub struct DOT11_AC_PARAM(pub i32);
pub const dot11_AC_param_BE: DOT11_AC_PARAM = DOT11_AC_PARAM(0i32);
pub const dot11_AC_param_BK: DOT11_AC_PARAM = DOT11_AC_PARAM(1i32);
pub const dot11_AC_param_VI: DOT11_AC_PARAM = DOT11_AC_PARAM(2i32);
pub const dot11_AC_param_VO: DOT11_AC_PARAM = DOT11_AC_PARAM(3i32);
pub const dot11_AC_param_max: DOT11_AC_PARAM = DOT11_AC_PARAM(4i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_ADDITIONAL_IE(i32);
pub const DOT11_ADDITIONAL_IE_REVISION_1: u32 = 1u32;
#[repr(transparent)]
pub struct DOT11_ADHOC_AUTH_ALGORITHM(pub i32);
pub const DOT11_ADHOC_AUTH_ALGO_INVALID: DOT11_ADHOC_AUTH_ALGORITHM = DOT11_ADHOC_AUTH_ALGORITHM(-1i32);
pub const DOT11_ADHOC_AUTH_ALGO_80211_OPEN: DOT11_ADHOC_AUTH_ALGORITHM = DOT11_ADHOC_AUTH_ALGORITHM(1i32);
pub const DOT11_ADHOC_AUTH_ALGO_RSNA_PSK: DOT11_ADHOC_AUTH_ALGORITHM = DOT11_ADHOC_AUTH_ALGORITHM(7i32);
#[repr(transparent)]
pub struct DOT11_ADHOC_CIPHER_ALGORITHM(pub i32);
pub const DOT11_ADHOC_CIPHER_ALGO_INVALID: DOT11_ADHOC_CIPHER_ALGORITHM = DOT11_ADHOC_CIPHER_ALGORITHM(-1i32);
pub const DOT11_ADHOC_CIPHER_ALGO_NONE: DOT11_ADHOC_CIPHER_ALGORITHM = DOT11_ADHOC_CIPHER_ALGORITHM(0i32);
pub const DOT11_ADHOC_CIPHER_ALGO_CCMP: DOT11_ADHOC_CIPHER_ALGORITHM = DOT11_ADHOC_CIPHER_ALGORITHM(4i32);
pub const DOT11_ADHOC_CIPHER_ALGO_WEP: DOT11_ADHOC_CIPHER_ALGORITHM = DOT11_ADHOC_CIPHER_ALGORITHM(257i32);
#[repr(transparent)]
pub struct DOT11_ADHOC_CONNECT_FAIL_REASON(pub i32);
pub const DOT11_ADHOC_CONNECT_FAIL_DOMAIN_MISMATCH: DOT11_ADHOC_CONNECT_FAIL_REASON = DOT11_ADHOC_CONNECT_FAIL_REASON(0i32);
pub const DOT11_ADHOC_CONNECT_FAIL_PASSPHRASE_MISMATCH: DOT11_ADHOC_CONNECT_FAIL_REASON = DOT11_ADHOC_CONNECT_FAIL_REASON(1i32);
pub const DOT11_ADHOC_CONNECT_FAIL_OTHER: DOT11_ADHOC_CONNECT_FAIL_REASON = DOT11_ADHOC_CONNECT_FAIL_REASON(2i32);
#[repr(transparent)]
pub struct DOT11_ADHOC_NETWORK_CONNECTION_STATUS(pub i32);
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_INVALID: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = DOT11_ADHOC_NETWORK_CONNECTION_STATUS(0i32);
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_DISCONNECTED: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = DOT11_ADHOC_NETWORK_CONNECTION_STATUS(11i32);
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_CONNECTING: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = DOT11_ADHOC_NETWORK_CONNECTION_STATUS(12i32);
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_CONNECTED: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = DOT11_ADHOC_NETWORK_CONNECTION_STATUS(13i32);
pub const DOT11_ADHOC_NETWORK_CONNECTION_STATUS_FORMED: DOT11_ADHOC_NETWORK_CONNECTION_STATUS = DOT11_ADHOC_NETWORK_CONNECTION_STATUS(14i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_ANQP_QUERY_COMPLETE_PARAMETERS(i32);
pub const DOT11_ANQP_QUERY_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(transparent)]
pub struct DOT11_ANQP_QUERY_RESULT(pub i32);
pub const dot11_ANQP_query_result_success: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(0i32);
pub const dot11_ANQP_query_result_failure: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(1i32);
pub const dot11_ANQP_query_result_timed_out: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(2i32);
pub const dot11_ANQP_query_result_resources: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(3i32);
pub const dot11_ANQP_query_result_advertisement_protocol_not_supported_on_remote: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(4i32);
pub const dot11_ANQP_query_result_gas_protocol_failure: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(5i32);
pub const dot11_ANQP_query_result_advertisement_server_not_responding: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(6i32);
pub const dot11_ANQP_query_result_access_issues: DOT11_ANQP_QUERY_RESULT = DOT11_ANQP_QUERY_RESULT(7i32);
#[repr(C)]
pub struct DOT11_AP_JOIN_REQUEST(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_ASSOCIATION_COMPLETION_PARAMETERS(i32);
pub const DOT11_ASSOCIATION_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_ASSOCIATION_COMPLETION_PARAMETERS_REVISION_2: u32 = 2u32;
#[repr(C)]
pub struct DOT11_ASSOCIATION_INFO_EX(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_ASSOCIATION_INFO_LIST(i32);
pub const DOT11_ASSOCIATION_INFO_LIST_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_ASSOCIATION_PARAMS(i32);
pub const DOT11_ASSOCIATION_PARAMS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_ASSOCIATION_START_PARAMETERS(i32);
pub const DOT11_ASSOCIATION_START_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(transparent)]
pub struct DOT11_ASSOCIATION_STATE(pub i32);
pub const dot11_assoc_state_zero: DOT11_ASSOCIATION_STATE = DOT11_ASSOCIATION_STATE(0i32);
pub const dot11_assoc_state_unauth_unassoc: DOT11_ASSOCIATION_STATE = DOT11_ASSOCIATION_STATE(1i32);
pub const dot11_assoc_state_auth_unassoc: DOT11_ASSOCIATION_STATE = DOT11_ASSOCIATION_STATE(2i32);
pub const dot11_assoc_state_auth_assoc: DOT11_ASSOCIATION_STATE = DOT11_ASSOCIATION_STATE(3i32);
pub const DOT11_ASSOC_ERROR_SOURCE_OS: u32 = 0u32;
pub const DOT11_ASSOC_ERROR_SOURCE_OTHER: u32 = 255u32;
pub const DOT11_ASSOC_ERROR_SOURCE_REMOTE: u32 = 1u32;
pub const DOT11_ASSOC_STATUS_SUCCESS: u32 = 0u32;
#[repr(transparent)]
pub struct DOT11_AUTH_ALGORITHM(pub i32);
pub const DOT11_AUTH_ALGO_80211_OPEN: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(1i32);
pub const DOT11_AUTH_ALGO_80211_SHARED_KEY: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(2i32);
pub const DOT11_AUTH_ALGO_WPA: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(3i32);
pub const DOT11_AUTH_ALGO_WPA_PSK: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(4i32);
pub const DOT11_AUTH_ALGO_WPA_NONE: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(5i32);
pub const DOT11_AUTH_ALGO_RSNA: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(6i32);
pub const DOT11_AUTH_ALGO_RSNA_PSK: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(7i32);
pub const DOT11_AUTH_ALGO_WPA3: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(8i32);
pub const DOT11_AUTH_ALGO_WPA3_ENT_192: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(8i32);
pub const DOT11_AUTH_ALGO_WPA3_SAE: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(9i32);
pub const DOT11_AUTH_ALGO_OWE: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(10i32);
pub const DOT11_AUTH_ALGO_WPA3_ENT: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(11i32);
pub const DOT11_AUTH_ALGO_IHV_START: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(-2147483648i32);
pub const DOT11_AUTH_ALGO_IHV_END: DOT11_AUTH_ALGORITHM = DOT11_AUTH_ALGORITHM(-1i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_AUTH_ALGORITHM_LIST(i32);
pub const DOT11_AUTH_ALGORITHM_LIST_REVISION_1: u32 = 1u32;
pub const DOT11_AUTH_ALGO_MICHAEL: u32 = 1u32;
#[repr(C)]
pub struct DOT11_AUTH_CIPHER_PAIR(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_AUTH_CIPHER_PAIR_LIST(i32);
pub const DOT11_AUTH_CIPHER_PAIR_LIST_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_AVAILABLE_CHANNEL_LIST(i32);
pub const DOT11_AVAILABLE_CHANNEL_LIST_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_AVAILABLE_FREQUENCY_LIST(i32);
pub const DOT11_AVAILABLE_FREQUENCY_LIST_REVISION_1: u32 = 1u32;
#[repr(transparent)]
pub struct DOT11_BAND(pub i32);
pub const dot11_band_2p4g: DOT11_BAND = DOT11_BAND(1i32);
pub const dot11_band_4p9g: DOT11_BAND = DOT11_BAND(2i32);
pub const dot11_band_5g: DOT11_BAND = DOT11_BAND(3i32);
#[repr(C)]
pub struct DOT11_BSSID_CANDIDATE(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_BSSID_LIST(i32);
pub const DOT11_BSSID_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_BSS_DESCRIPTION(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_BSS_ENTRY(i32);
pub const DOT11_BSS_ENTRY_BYTE_ARRAY_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_BSS_ENTRY_PHY_SPECIFIC_INFO(i32);
#[repr(transparent)]
pub struct DOT11_BSS_TYPE(pub i32);
pub const dot11_BSS_type_infrastructure: DOT11_BSS_TYPE = DOT11_BSS_TYPE(1i32);
pub const dot11_BSS_type_independent: DOT11_BSS_TYPE = DOT11_BSS_TYPE(2i32);
pub const dot11_BSS_type_any: DOT11_BSS_TYPE = DOT11_BSS_TYPE(3i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_BYTE_ARRAY(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_CAN_SUSTAIN_AP_PARAMETERS(i32);
pub const DOT11_CAN_SUSTAIN_AP_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_CAN_SUSTAIN_AP_REASON_IHV_END: u32 = 4294967295u32;
pub const DOT11_CAN_SUSTAIN_AP_REASON_IHV_START: u32 = 4278190080u32;
pub const DOT11_CAPABILITY_CHANNEL_AGILITY: u32 = 128u32;
pub const DOT11_CAPABILITY_DSSSOFDM: u32 = 8192u32;
pub const DOT11_CAPABILITY_INFO_CF_POLLABLE: u32 = 4u32;
pub const DOT11_CAPABILITY_INFO_CF_POLL_REQ: u32 = 8u32;
pub const DOT11_CAPABILITY_INFO_ESS: u32 = 1u32;
pub const DOT11_CAPABILITY_INFO_IBSS: u32 = 2u32;
pub const DOT11_CAPABILITY_INFO_PRIVACY: u32 = 16u32;
pub const DOT11_CAPABILITY_PBCC: u32 = 64u32;
pub const DOT11_CAPABILITY_SHORT_PREAMBLE: u32 = 32u32;
pub const DOT11_CAPABILITY_SHORT_SLOT_TIME: u32 = 1024u32;
pub const DOT11_CCA_MODE_CS_ONLY: u32 = 2u32;
pub const DOT11_CCA_MODE_CS_WITH_TIMER: u32 = 8u32;
pub const DOT11_CCA_MODE_ED_ONLY: u32 = 1u32;
pub const DOT11_CCA_MODE_ED_and_CS: u32 = 4u32;
pub const DOT11_CCA_MODE_HRCS_AND_ED: u32 = 16u32;
#[repr(C)]
pub struct DOT11_CHANNEL_HINT(i32);
#[repr(transparent)]
pub struct DOT11_CIPHER_ALGORITHM(pub i32);
pub const DOT11_CIPHER_ALGO_NONE: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(0i32);
pub const DOT11_CIPHER_ALGO_WEP40: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(1i32);
pub const DOT11_CIPHER_ALGO_TKIP: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(2i32);
pub const DOT11_CIPHER_ALGO_CCMP: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(4i32);
pub const DOT11_CIPHER_ALGO_WEP104: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(5i32);
pub const DOT11_CIPHER_ALGO_BIP: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(6i32);
pub const DOT11_CIPHER_ALGO_GCMP: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(8i32);
pub const DOT11_CIPHER_ALGO_GCMP_256: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(9i32);
pub const DOT11_CIPHER_ALGO_CCMP_256: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(10i32);
pub const DOT11_CIPHER_ALGO_BIP_GMAC_128: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(11i32);
pub const DOT11_CIPHER_ALGO_BIP_GMAC_256: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(12i32);
pub const DOT11_CIPHER_ALGO_BIP_CMAC_256: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(13i32);
pub const DOT11_CIPHER_ALGO_WPA_USE_GROUP: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(256i32);
pub const DOT11_CIPHER_ALGO_RSN_USE_GROUP: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(256i32);
pub const DOT11_CIPHER_ALGO_WEP: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(257i32);
pub const DOT11_CIPHER_ALGO_IHV_START: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(-2147483648i32);
pub const DOT11_CIPHER_ALGO_IHV_END: DOT11_CIPHER_ALGORITHM = DOT11_CIPHER_ALGORITHM(-1i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_CIPHER_ALGORITHM_LIST(i32);
pub const DOT11_CIPHER_ALGORITHM_LIST_REVISION_1: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_CIPHER_DEFAULT_KEY_VALUE(i32);
pub const DOT11_CIPHER_DEFAULT_KEY_VALUE_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_CIPHER_KEY_MAPPING_KEY_VALUE(i32);
pub const DOT11_CIPHER_KEY_MAPPING_KEY_VALUE_BYTE_ARRAY_REVISION_1: u32 = 1u32;
pub const DOT11_CONF_ALGO_TKIP: u32 = 2u32;
pub const DOT11_CONF_ALGO_WEP_RC4: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_CONNECTION_COMPLETION_PARAMETERS(i32);
pub const DOT11_CONNECTION_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_CONNECTION_START_PARAMETERS(i32);
pub const DOT11_CONNECTION_START_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_CONNECTION_STATUS_SUCCESS: u32 = 0u32;
#[repr(C)]
pub struct DOT11_COUNTERS_ENTRY(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_COUNTRY_OR_REGION_STRING_LIST(i32);
pub const DOT11_COUNTRY_OR_REGION_STRING_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_CURRENT_OFFLOAD_CAPABILITY(i32);
#[repr(C)]
pub struct DOT11_CURRENT_OPERATION_MODE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_CURRENT_OPTIONAL_CAPABILITY(i32);
#[repr(C)]
pub struct DOT11_DATA_RATE_MAPPING_ENTRY(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_DATA_RATE_MAPPING_TABLE(i32);
pub const DOT11_DATA_RATE_MAPPING_TABLE_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_DEFAULT_WEP_OFFLOAD(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_DEFAULT_WEP_UPLOAD(i32);
pub const DOT11_DEVICE_ENTRY_BYTE_ARRAY_REVISION_1: u32 = 1u32;
#[repr(transparent)]
pub struct DOT11_DIRECTION(pub i32);
pub const DOT11_DIR_INBOUND: DOT11_DIRECTION = DOT11_DIRECTION(1i32);
pub const DOT11_DIR_OUTBOUND: DOT11_DIRECTION = DOT11_DIRECTION(2i32);
pub const DOT11_DIR_BOTH: DOT11_DIRECTION = DOT11_DIRECTION(3i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_DISASSOCIATE_PEER_REQUEST(i32);
pub const DOT11_DISASSOCIATE_PEER_REQUEST_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_DISASSOCIATION_PARAMETERS(i32);
pub const DOT11_DISASSOCIATION_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_DIVERSITY_SELECTION_RX(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_DIVERSITY_SELECTION_RX_LIST(i32);
#[repr(transparent)]
pub struct DOT11_DIVERSITY_SUPPORT(pub i32);
pub const dot11_diversity_support_unknown: DOT11_DIVERSITY_SUPPORT = DOT11_DIVERSITY_SUPPORT(0i32);
pub const dot11_diversity_support_fixedlist: DOT11_DIVERSITY_SUPPORT = DOT11_DIVERSITY_SUPPORT(1i32);
pub const dot11_diversity_support_notsupported: DOT11_DIVERSITY_SUPPORT = DOT11_DIVERSITY_SUPPORT(2i32);
pub const dot11_diversity_support_dynamic: DOT11_DIVERSITY_SUPPORT = DOT11_DIVERSITY_SUPPORT(3i32);
#[repr(transparent)]
pub struct DOT11_DS_INFO(pub i32);
pub const DOT11_DS_CHANGED: DOT11_DS_INFO = DOT11_DS_INFO(0i32);
pub const DOT11_DS_UNCHANGED: DOT11_DS_INFO = DOT11_DS_INFO(1i32);
pub const DOT11_DS_UNKNOWN: DOT11_DS_INFO = DOT11_DS_INFO(2i32);
pub const DOT11_ENCAP_802_1H: u32 = 2u32;
#[repr(C)]
pub struct DOT11_ENCAP_ENTRY(i32);
pub const DOT11_ENCAP_RFC_1042: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_ERP_PHY_ATTRIBUTES(i32);
pub const DOT11_EXEMPT_ALWAYS: u32 = 1u32;
pub const DOT11_EXEMPT_BOTH: u32 = 3u32;
pub const DOT11_EXEMPT_MULTICAST: u32 = 2u32;
pub const DOT11_EXEMPT_NO_EXEMPTION: u32 = 0u32;
pub const DOT11_EXEMPT_ON_KEY_MAPPING_KEY_UNAVAILABLE: u32 = 2u32;
pub const DOT11_EXEMPT_UNICAST: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_EXTAP_ATTRIBUTES(i32);
pub const DOT11_EXTAP_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const DOT11_EXTAP_RECV_CONTEXT_REVISION_1: u32 = 1u32;
pub const DOT11_EXTAP_SEND_CONTEXT_REVISION_1: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_EXTSTA_ATTRIBUTES(i32);
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_1: u32 = 1u32;
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_2: u32 = 2u32;
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_3: u32 = 3u32;
pub const DOT11_EXTSTA_ATTRIBUTES_REVISION_4: u32 = 4u32;
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_CERTIFIED: u32 = 2u32;
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_OID_SUPPORTED: u32 = 1u32;
pub const DOT11_EXTSTA_ATTRIBUTES_SAFEMODE_RESERVED: u32 = 12u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_EXTSTA_CAPABILITY(i32);
pub const DOT11_EXTSTA_CAPABILITY_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_EXTSTA_RECV_CONTEXT(i32);
pub const DOT11_EXTSTA_RECV_CONTEXT_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_EXTSTA_SEND_CONTEXT(i32);
pub const DOT11_EXTSTA_SEND_CONTEXT_REVISION_1: u32 = 1u32;
pub const DOT11_FLAGS_80211B_CHANNEL_AGILITY: u32 = 4u32;
pub const DOT11_FLAGS_80211B_PBCC: u32 = 2u32;
pub const DOT11_FLAGS_80211B_SHORT_PREAMBLE: u32 = 1u32;
pub const DOT11_FLAGS_80211G_BARKER_PREAMBLE_MODE: u32 = 128u32;
pub const DOT11_FLAGS_80211G_DSSS_OFDM: u32 = 16u32;
pub const DOT11_FLAGS_80211G_NON_ERP_PRESENT: u32 = 64u32;
pub const DOT11_FLAGS_80211G_USE_PROTECTION: u32 = 32u32;
pub const DOT11_FLAGS_PS_ON: u32 = 8u32;
#[repr(C)]
pub struct DOT11_FRAGMENT_DESCRIPTOR(i32);
pub const DOT11_FREQUENCY_BANDS_LOWER: u32 = 1u32;
pub const DOT11_FREQUENCY_BANDS_MIDDLE: u32 = 2u32;
pub const DOT11_FREQUENCY_BANDS_UPPER: u32 = 4u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS(i32);
pub const DOT11_GO_NEGOTIATION_CONFIRMATION_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS(i32);
pub const DOT11_GO_NEGOTIATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS(i32);
pub const DOT11_GO_NEGOTIATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_HESSID_LENGTH: u32 = 6u32;
#[repr(C)]
pub struct DOT11_HOPPING_PATTERN_ENTRY(i32);
#[repr(C)]
pub struct DOT11_HOPPING_PATTERN_ENTRY_LIST(i32);
#[repr(transparent)]
pub struct DOT11_HOP_ALGO_ADOPTED(pub i32);
pub const dot11_hop_algo_current: DOT11_HOP_ALGO_ADOPTED = DOT11_HOP_ALGO_ADOPTED(0i32);
pub const dot11_hop_algo_hop_index: DOT11_HOP_ALGO_ADOPTED = DOT11_HOP_ALGO_ADOPTED(1i32);
pub const dot11_hop_algo_hcc: DOT11_HOP_ALGO_ADOPTED = DOT11_HOP_ALGO_ADOPTED(2i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_HRDSSS_PHY_ATTRIBUTES(i32);
pub const DOT11_HR_CCA_MODE_CS_AND_ED: u32 = 4u32;
pub const DOT11_HR_CCA_MODE_CS_ONLY: u32 = 2u32;
pub const DOT11_HR_CCA_MODE_CS_WITH_TIMER: u32 = 8u32;
pub const DOT11_HR_CCA_MODE_ED_ONLY: u32 = 1u32;
pub const DOT11_HR_CCA_MODE_HRCS_AND_ED: u32 = 16u32;
pub const DOT11_HW_DEFRAGMENTATION_SUPPORTED: u32 = 8u32;
pub const DOT11_HW_FRAGMENTATION_SUPPORTED: u32 = 4u32;
pub const DOT11_HW_MSDU_AUTH_SUPPORTED_RX: u32 = 32u32;
pub const DOT11_HW_MSDU_AUTH_SUPPORTED_TX: u32 = 16u32;
pub const DOT11_HW_WEP_SUPPORTED_RX: u32 = 2u32;
pub const DOT11_HW_WEP_SUPPORTED_TX: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_IBSS_PARAMS(i32);
pub const DOT11_IBSS_PARAMS_REVISION_1: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS(i32);
pub const DOT11_INCOMING_ASSOC_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_INCOMING_ASSOC_DECISION(i32);
pub const DOT11_INCOMING_ASSOC_DECISION_REVISION_1: u32 = 1u32;
pub const DOT11_INCOMING_ASSOC_DECISION_REVISION_2: u32 = 2u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_INCOMING_ASSOC_DECISION_V2(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS(i32);
pub const DOT11_INCOMING_ASSOC_REQUEST_RECEIVED_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_INCOMING_ASSOC_STARTED_PARAMETERS(i32);
pub const DOT11_INCOMING_ASSOC_STARTED_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_INVALID_CHANNEL_NUMBER: u32 = 0u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS(i32);
pub const DOT11_INVITATION_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS(i32);
pub const DOT11_INVITATION_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_IV48_COUNTER(i32);
#[repr(C)]
pub struct DOT11_JOIN_REQUEST(i32);
#[repr(C)]
pub struct DOT11_KEY_ALGO_BIP(i32);
#[repr(C)]
pub struct DOT11_KEY_ALGO_BIP_GMAC_256(i32);
#[repr(C)]
pub struct DOT11_KEY_ALGO_CCMP(i32);
#[repr(C)]
pub struct DOT11_KEY_ALGO_GCMP(i32);
#[repr(C)]
pub struct DOT11_KEY_ALGO_GCMP_256(i32);
#[repr(C)]
pub struct DOT11_KEY_ALGO_TKIP_MIC(i32);
#[repr(transparent)]
pub struct DOT11_KEY_DIRECTION(pub i32);
pub const dot11_key_direction_both: DOT11_KEY_DIRECTION = DOT11_KEY_DIRECTION(1i32);
pub const dot11_key_direction_inbound: DOT11_KEY_DIRECTION = DOT11_KEY_DIRECTION(2i32);
pub const dot11_key_direction_outbound: DOT11_KEY_DIRECTION = DOT11_KEY_DIRECTION(3i32);
#[repr(C)]
pub struct DOT11_LINK_QUALITY_ENTRY(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_LINK_QUALITY_PARAMETERS(i32);
pub const DOT11_LINK_QUALITY_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_MAC_ADDRESS_LIST(i32);
pub const DOT11_MAC_ADDRESS_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_MAC_FRAME_STATISTICS(i32);
#[repr(C)]
pub struct DOT11_MAC_INFO(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_MAC_PARAMETERS(i32);
pub const DOT11_MAC_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_MANUFACTURING_CALLBACK_PARAMETERS(i32);
pub const DOT11_MANUFACTURING_CALLBACK_REVISION_1: u32 = 1u32;
#[repr(transparent)]
pub struct DOT11_MANUFACTURING_CALLBACK_TYPE(pub i32);
pub const dot11_manufacturing_callback_unknown: DOT11_MANUFACTURING_CALLBACK_TYPE = DOT11_MANUFACTURING_CALLBACK_TYPE(0i32);
pub const dot11_manufacturing_callback_self_test_complete: DOT11_MANUFACTURING_CALLBACK_TYPE = DOT11_MANUFACTURING_CALLBACK_TYPE(1i32);
pub const dot11_manufacturing_callback_sleep_complete: DOT11_MANUFACTURING_CALLBACK_TYPE = DOT11_MANUFACTURING_CALLBACK_TYPE(2i32);
pub const dot11_manufacturing_callback_IHV_start: DOT11_MANUFACTURING_CALLBACK_TYPE = DOT11_MANUFACTURING_CALLBACK_TYPE(-2147483648i32);
pub const dot11_manufacturing_callback_IHV_end: DOT11_MANUFACTURING_CALLBACK_TYPE = DOT11_MANUFACTURING_CALLBACK_TYPE(-1i32);
#[repr(C)]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_QUERY_ADC(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_RX(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_MANUFACTURING_FUNCTIONAL_TEST_TX(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_MANUFACTURING_SELF_TEST_QUERY_RESULTS(i32);
#[repr(C)]
pub struct DOT11_MANUFACTURING_SELF_TEST_SET_PARAMS(i32);
#[repr(transparent)]
pub struct DOT11_MANUFACTURING_SELF_TEST_TYPE(pub i32);
pub const DOT11_MANUFACTURING_SELF_TEST_TYPE_INTERFACE: DOT11_MANUFACTURING_SELF_TEST_TYPE = DOT11_MANUFACTURING_SELF_TEST_TYPE(1i32);
pub const DOT11_MANUFACTURING_SELF_TEST_TYPE_RF_INTERFACE: DOT11_MANUFACTURING_SELF_TEST_TYPE = DOT11_MANUFACTURING_SELF_TEST_TYPE(2i32);
pub const DOT11_MANUFACTURING_SELF_TEST_TYPE_BT_COEXISTENCE: DOT11_MANUFACTURING_SELF_TEST_TYPE = DOT11_MANUFACTURING_SELF_TEST_TYPE(3i32);
#[repr(C)]
pub struct DOT11_MANUFACTURING_TEST(i32);
#[repr(C)]
pub struct DOT11_MANUFACTURING_TEST_QUERY_DATA(i32);
pub const DOT11_MANUFACTURING_TEST_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_MANUFACTURING_TEST_SET_DATA(i32);
#[repr(C)]
pub struct DOT11_MANUFACTURING_TEST_SLEEP(i32);
#[repr(transparent)]
pub struct DOT11_MANUFACTURING_TEST_TYPE(pub i32);
pub const dot11_manufacturing_test_unknown: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(0i32);
pub const dot11_manufacturing_test_self_start: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(1i32);
pub const dot11_manufacturing_test_self_query_result: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(2i32);
pub const dot11_manufacturing_test_rx: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(3i32);
pub const dot11_manufacturing_test_tx: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(4i32);
pub const dot11_manufacturing_test_query_adc: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(5i32);
pub const dot11_manufacturing_test_set_data: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(6i32);
pub const dot11_manufacturing_test_query_data: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(7i32);
pub const dot11_manufacturing_test_sleep: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(8i32);
pub const dot11_manufacturing_test_awake: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(9i32);
pub const dot11_manufacturing_test_IHV_start: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(-2147483648i32);
pub const dot11_manufacturing_test_IHV_end: DOT11_MANUFACTURING_TEST_TYPE = DOT11_MANUFACTURING_TEST_TYPE(-1i32);
pub const DOT11_MAX_CHANNEL_HINTS: u32 = 4u32;
pub const DOT11_MAX_NUM_DEFAULT_KEY: u32 = 4u32;
pub const DOT11_MAX_NUM_DEFAULT_KEY_MFP: u32 = 6u32;
pub const DOT11_MAX_NUM_OF_FRAGMENTS: u32 = 16u32;
pub const DOT11_MAX_PDU_SIZE: u32 = 2346u32;
pub const DOT11_MAX_REQUESTED_SERVICE_INFORMATION_LENGTH: u32 = 255u32;
#[repr(C)]
pub struct DOT11_MD_CAPABILITY_ENTRY_LIST(i32);
pub const DOT11_MIN_PDU_SIZE: u32 = 256u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_MPDU_MAX_LENGTH_INDICATION(i32);
pub const DOT11_MPDU_MAX_LENGTH_INDICATION_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_MULTI_DOMAIN_CAPABILITY_ENTRY(i32);
#[repr(C)]
pub struct DOT11_NETWORK(i32);
#[repr(C)]
pub struct DOT11_NETWORK_LIST(i32);
#[repr(C)]
pub struct DOT11_NIC_SPECIFIC_EXTENSION(i32);
pub const DOT11_NLO_FLAG_SCAN_AT_SYSTEM_RESUME: u32 = 4u32;
pub const DOT11_NLO_FLAG_SCAN_ON_AOAC_PLATFORM: u32 = 2u32;
pub const DOT11_NLO_FLAG_STOP_NLO_INDICATION: u32 = 1u32;
#[repr(C)]
pub struct DOT11_OFDM_PHY_ATTRIBUTES(i32);
#[repr(C)]
pub struct DOT11_OFFLOAD_CAPABILITY(i32);
#[repr(C)]
pub struct DOT11_OFFLOAD_NETWORK(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_OFFLOAD_NETWORK_LIST_INFO(i32);
pub const DOT11_OFFLOAD_NETWORK_LIST_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS(i32);
pub const DOT11_OFFLOAD_NETWORK_STATUS_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(transparent)]
pub struct DOT11_OFFLOAD_TYPE(pub i32);
pub const dot11_offload_type_wep: DOT11_OFFLOAD_TYPE = DOT11_OFFLOAD_TYPE(1i32);
pub const dot11_offload_type_auth: DOT11_OFFLOAD_TYPE = DOT11_OFFLOAD_TYPE(2i32);
#[repr(C)]
pub struct DOT11_OI(i32);
pub const DOT11_OI_MAX_LENGTH: u32 = 5u32;
pub const DOT11_OI_MIN_LENGTH: u32 = 3u32;
pub const DOT11_OPERATION_MODE_AP: u32 = 2u32;
#[repr(C)]
pub struct DOT11_OPERATION_MODE_CAPABILITY(i32);
pub const DOT11_OPERATION_MODE_EXTENSIBLE_AP: u32 = 8u32;
pub const DOT11_OPERATION_MODE_EXTENSIBLE_STATION: u32 = 4u32;
pub const DOT11_OPERATION_MODE_MANUFACTURING: u32 = 1073741824u32;
pub const DOT11_OPERATION_MODE_NETWORK_MONITOR: u32 = 2147483648u32;
pub const DOT11_OPERATION_MODE_STATION: u32 = 1u32;
pub const DOT11_OPERATION_MODE_UNKNOWN: u32 = 0u32;
pub const DOT11_OPERATION_MODE_WFD_CLIENT: u32 = 64u32;
pub const DOT11_OPERATION_MODE_WFD_DEVICE: u32 = 16u32;
pub const DOT11_OPERATION_MODE_WFD_GROUP_OWNER: u32 = 32u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_OPTIONAL_CAPABILITY(i32);
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_CTRL: u32 = 4096u32;
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_DATA: u32 = 16384u32;
pub const DOT11_PACKET_TYPE_ALL_MULTICAST_MGMT: u32 = 8192u32;
pub const DOT11_PACKET_TYPE_BROADCAST_CTRL: u32 = 64u32;
pub const DOT11_PACKET_TYPE_BROADCAST_DATA: u32 = 256u32;
pub const DOT11_PACKET_TYPE_BROADCAST_MGMT: u32 = 128u32;
pub const DOT11_PACKET_TYPE_DIRECTED_CTRL: u32 = 1u32;
pub const DOT11_PACKET_TYPE_DIRECTED_DATA: u32 = 4u32;
pub const DOT11_PACKET_TYPE_DIRECTED_MGMT: u32 = 2u32;
pub const DOT11_PACKET_TYPE_MULTICAST_CTRL: u32 = 8u32;
pub const DOT11_PACKET_TYPE_MULTICAST_DATA: u32 = 32u32;
pub const DOT11_PACKET_TYPE_MULTICAST_MGMT: u32 = 16u32;
pub const DOT11_PACKET_TYPE_PROMISCUOUS_CTRL: u32 = 512u32;
pub const DOT11_PACKET_TYPE_PROMISCUOUS_DATA: u32 = 2048u32;
pub const DOT11_PACKET_TYPE_PROMISCUOUS_MGMT: u32 = 1024u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_PEER_INFO(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_PEER_INFO_LIST(i32);
pub const DOT11_PEER_INFO_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_PEER_STATISTICS(i32);
#[repr(C)]
pub struct DOT11_PER_MSDU_COUNTERS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_PHY_ATTRIBUTES(i32);
pub const DOT11_PHY_ATTRIBUTES_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_PHY_FRAME_STATISTICS(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS(i32);
pub const DOT11_PHY_FREQUENCY_ADOPTED_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_PHY_ID_LIST(i32);
pub const DOT11_PHY_ID_LIST_REVISION_1: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_PHY_STATE_PARAMETERS(i32);
pub const DOT11_PHY_STATE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(transparent)]
pub struct DOT11_PHY_TYPE(pub i32);
pub const dot11_phy_type_unknown: DOT11_PHY_TYPE = DOT11_PHY_TYPE(0i32);
pub const dot11_phy_type_any: DOT11_PHY_TYPE = DOT11_PHY_TYPE(0i32);
pub const dot11_phy_type_fhss: DOT11_PHY_TYPE = DOT11_PHY_TYPE(1i32);
pub const dot11_phy_type_dsss: DOT11_PHY_TYPE = DOT11_PHY_TYPE(2i32);
pub const dot11_phy_type_irbaseband: DOT11_PHY_TYPE = DOT11_PHY_TYPE(3i32);
pub const dot11_phy_type_ofdm: DOT11_PHY_TYPE = DOT11_PHY_TYPE(4i32);
pub const dot11_phy_type_hrdsss: DOT11_PHY_TYPE = DOT11_PHY_TYPE(5i32);
pub const dot11_phy_type_erp: DOT11_PHY_TYPE = DOT11_PHY_TYPE(6i32);
pub const dot11_phy_type_ht: DOT11_PHY_TYPE = DOT11_PHY_TYPE(7i32);
pub const dot11_phy_type_vht: DOT11_PHY_TYPE = DOT11_PHY_TYPE(8i32);
pub const dot11_phy_type_dmg: DOT11_PHY_TYPE = DOT11_PHY_TYPE(9i32);
pub const dot11_phy_type_he: DOT11_PHY_TYPE = DOT11_PHY_TYPE(10i32);
pub const dot11_phy_type_IHV_start: DOT11_PHY_TYPE = DOT11_PHY_TYPE(-2147483648i32);
pub const dot11_phy_type_IHV_end: DOT11_PHY_TYPE = DOT11_PHY_TYPE(-1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_PHY_TYPE_INFO(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_PHY_TYPE_LIST(i32);
pub const DOT11_PHY_TYPE_LIST_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_PMKID_CANDIDATE_LIST_PARAMETERS(i32);
pub const DOT11_PMKID_CANDIDATE_LIST_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_PMKID_ENTRY(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_PMKID_LIST(i32);
pub const DOT11_PMKID_LIST_REVISION_1: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_PORT_STATE_NOTIFICATION(i32);
pub const DOT11_PORT_STATE_NOTIFICATION_REVISION_1: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_POWER_MGMT_AUTO_MODE_ENABLED_INFO(i32);
pub const DOT11_POWER_MGMT_AUTO_MODE_ENABLED_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_POWER_MGMT_MODE(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_POWER_MGMT_MODE_STATUS_INFO(i32);
pub const DOT11_POWER_MGMT_MODE_STATUS_INFO_REVISION_1: u32 = 1u32;
#[repr(transparent)]
pub struct DOT11_POWER_MODE(pub i32);
pub const dot11_power_mode_unknown: DOT11_POWER_MODE = DOT11_POWER_MODE(0i32);
pub const dot11_power_mode_active: DOT11_POWER_MODE = DOT11_POWER_MODE(1i32);
pub const dot11_power_mode_powersave: DOT11_POWER_MODE = DOT11_POWER_MODE(2i32);
#[repr(transparent)]
pub struct DOT11_POWER_MODE_REASON(pub i32);
pub const dot11_power_mode_reason_no_change: DOT11_POWER_MODE_REASON = DOT11_POWER_MODE_REASON(0i32);
pub const dot11_power_mode_reason_noncompliant_AP: DOT11_POWER_MODE_REASON = DOT11_POWER_MODE_REASON(1i32);
pub const dot11_power_mode_reason_legacy_WFD_device: DOT11_POWER_MODE_REASON = DOT11_POWER_MODE_REASON(2i32);
pub const dot11_power_mode_reason_compliant_AP: DOT11_POWER_MODE_REASON = DOT11_POWER_MODE_REASON(3i32);
pub const dot11_power_mode_reason_compliant_WFD_device: DOT11_POWER_MODE_REASON = DOT11_POWER_MODE_REASON(4i32);
pub const dot11_power_mode_reason_others: DOT11_POWER_MODE_REASON = DOT11_POWER_MODE_REASON(5i32);
pub const DOT11_POWER_SAVE_LEVEL_FAST_PSP: u32 = 2u32;
pub const DOT11_POWER_SAVE_LEVEL_MAX_PSP: u32 = 1u32;
pub const DOT11_POWER_SAVING_FAST_PSP: u32 = 8u32;
pub const DOT11_POWER_SAVING_MAXIMUM_LEVEL: u32 = 24u32;
pub const DOT11_POWER_SAVING_MAX_PSP: u32 = 16u32;
pub const DOT11_POWER_SAVING_NO_POWER_SAVING: u32 = 0u32;
pub const DOT11_PRIORITY_CONTENTION: u32 = 0u32;
pub const DOT11_PRIORITY_CONTENTION_FREE: u32 = 1u32;
#[repr(C)]
pub struct DOT11_PRIVACY_EXEMPTION(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_PRIVACY_EXEMPTION_LIST(i32);
pub const DOT11_PRIVACY_EXEMPTION_LIST_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS(i32);
pub const DOT11_PROVISION_DISCOVERY_REQUEST_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS(i32);
pub const DOT11_PROVISION_DISCOVERY_RESPONSE_SEND_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_PSD_IE_MAX_DATA_SIZE: u32 = 240u32;
pub const DOT11_PSD_IE_MAX_ENTRY_NUMBER: u32 = 5u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_QOS_PARAMS(i32);
pub const DOT11_QOS_PARAMS_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_QOS_TX_DURATION(i32);
#[repr(C)]
pub struct DOT11_QOS_TX_MEDIUM_TIME(i32);
#[repr(transparent)]
pub struct DOT11_RADIO_STATE(pub i32);
pub const dot11_radio_state_unknown: DOT11_RADIO_STATE = DOT11_RADIO_STATE(0i32);
pub const dot11_radio_state_on: DOT11_RADIO_STATE = DOT11_RADIO_STATE(1i32);
pub const dot11_radio_state_off: DOT11_RADIO_STATE = DOT11_RADIO_STATE(2i32);
#[repr(C)]
pub struct DOT11_RATE_SET(i32);
pub const DOT11_RATE_SET_MAX_LENGTH: u32 = 126u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS(i32);
pub const DOT11_RECEIVED_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS(i32);
pub const DOT11_RECEIVED_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS(i32);
pub const DOT11_RECEIVED_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS(i32);
pub const DOT11_RECEIVED_INVITATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS(i32);
pub const DOT11_RECEIVED_INVITATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS(i32);
pub const DOT11_RECEIVED_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS(i32);
pub const DOT11_RECEIVED_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_RECV_CONTEXT_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_RECV_EXTENSION_INFO(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_RECV_EXTENSION_INFO_V2(i32);
#[repr(C)]
pub struct DOT11_RECV_SENSITIVITY(i32);
#[repr(C)]
pub struct DOT11_RECV_SENSITIVITY_LIST(i32);
#[repr(C)]
pub struct DOT11_REG_DOMAINS_SUPPORT_VALUE(i32);
pub const DOT11_REG_DOMAIN_DOC: u32 = 32u32;
pub const DOT11_REG_DOMAIN_ETSI: u32 = 48u32;
pub const DOT11_REG_DOMAIN_FCC: u32 = 16u32;
pub const DOT11_REG_DOMAIN_FRANCE: u32 = 50u32;
pub const DOT11_REG_DOMAIN_MKK: u32 = 64u32;
pub const DOT11_REG_DOMAIN_OTHER: u32 = 0u32;
pub const DOT11_REG_DOMAIN_SPAIN: u32 = 49u32;
#[repr(C)]
pub struct DOT11_REG_DOMAIN_VALUE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_RESET_REQUEST(i32);
#[repr(transparent)]
pub struct DOT11_RESET_TYPE(pub i32);
pub const dot11_reset_type_phy: DOT11_RESET_TYPE = DOT11_RESET_TYPE(1i32);
pub const dot11_reset_type_mac: DOT11_RESET_TYPE = DOT11_RESET_TYPE(2i32);
pub const dot11_reset_type_phy_and_mac: DOT11_RESET_TYPE = DOT11_RESET_TYPE(3i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_ROAMING_COMPLETION_PARAMETERS(i32);
pub const DOT11_ROAMING_COMPLETION_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_ROAMING_START_PARAMETERS(i32);
pub const DOT11_ROAMING_START_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_RSSI_RANGE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_SCAN_REQUEST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_SCAN_REQUEST_V2(i32);
#[repr(transparent)]
pub struct DOT11_SCAN_TYPE(pub i32);
pub const dot11_scan_type_active: DOT11_SCAN_TYPE = DOT11_SCAN_TYPE(1i32);
pub const dot11_scan_type_passive: DOT11_SCAN_TYPE = DOT11_SCAN_TYPE(2i32);
pub const dot11_scan_type_auto: DOT11_SCAN_TYPE = DOT11_SCAN_TYPE(3i32);
pub const dot11_scan_type_forced: DOT11_SCAN_TYPE = DOT11_SCAN_TYPE(-2147483648i32);
pub const DOT11_SEND_CONTEXT_REVISION_1: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS(i32);
pub const DOT11_SEND_GO_NEGOTIATION_CONFIRMATION_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS(i32);
pub const DOT11_SEND_GO_NEGOTIATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS(i32);
pub const DOT11_SEND_GO_NEGOTIATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_SEND_INVITATION_REQUEST_PARAMETERS(i32);
pub const DOT11_SEND_INVITATION_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_SEND_INVITATION_RESPONSE_PARAMETERS(i32);
pub const DOT11_SEND_INVITATION_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS(i32);
pub const DOT11_SEND_PROVISION_DISCOVERY_REQUEST_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS(i32);
pub const DOT11_SEND_PROVISION_DISCOVERY_RESPONSE_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_SERVICE_CLASS_REORDERABLE_MULTICAST: u32 = 0u32;
pub const DOT11_SERVICE_CLASS_STRICTLY_ORDERED: u32 = 1u32;
#[repr(C)]
pub struct DOT11_SSID(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_SSID_LIST(i32);
pub const DOT11_SSID_LIST_REVISION_1: u32 = 1u32;
pub const DOT11_SSID_MAX_LENGTH: u32 = 32u32;
#[repr(C)]
pub struct DOT11_START_REQUEST(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_STATISTICS(i32);
pub const DOT11_STATISTICS_REVISION_1: u32 = 1u32;
pub const DOT11_STATUS_AP_JOIN_CONFIRM: u32 = 5u32;
pub const DOT11_STATUS_AUTH_FAILED: u32 = 131072u32;
pub const DOT11_STATUS_AUTH_NOT_VERIFIED: u32 = 32768u32;
pub const DOT11_STATUS_AUTH_VERIFIED: u32 = 65536u32;
pub const DOT11_STATUS_ENCRYPTION_FAILED: u32 = 512u32;
pub const DOT11_STATUS_EXCESSIVE_DATA_LENGTH: u32 = 256u32;
pub const DOT11_STATUS_GENERATE_AUTH_FAILED: u32 = 16384u32;
pub const DOT11_STATUS_ICV_VERIFIED: u32 = 2048u32;
#[repr(C)]
pub struct DOT11_STATUS_INDICATION(i32);
pub const DOT11_STATUS_JOIN_CONFIRM: u32 = 2u32;
pub const DOT11_STATUS_MPDU_MAX_LENGTH_CHANGED: u32 = 6u32;
pub const DOT11_STATUS_PACKET_NOT_REASSEMBLED: u32 = 8192u32;
pub const DOT11_STATUS_PACKET_REASSEMBLED: u32 = 4096u32;
pub const DOT11_STATUS_PS_LIFETIME_EXPIRED: u32 = 262144u32;
pub const DOT11_STATUS_RESET_CONFIRM: u32 = 4u32;
pub const DOT11_STATUS_RETRY_LIMIT_EXCEEDED: u32 = 2u32;
pub const DOT11_STATUS_SCAN_CONFIRM: u32 = 1u32;
pub const DOT11_STATUS_START_CONFIRM: u32 = 3u32;
pub const DOT11_STATUS_SUCCESS: u32 = 1u32;
pub const DOT11_STATUS_UNAVAILABLE_BSS: u32 = 128u32;
pub const DOT11_STATUS_UNAVAILABLE_PRIORITY: u32 = 16u32;
pub const DOT11_STATUS_UNAVAILABLE_SERVICE_CLASS: u32 = 32u32;
pub const DOT11_STATUS_UNSUPPORTED_PRIORITY: u32 = 4u32;
pub const DOT11_STATUS_UNSUPPORTED_SERVICE_CLASS: u32 = 8u32;
pub const DOT11_STATUS_WEP_KEY_UNAVAILABLE: u32 = 1024u32;
pub const DOT11_STATUS_XMIT_MSDU_TIMER_EXPIRED: u32 = 64u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_STOP_AP_PARAMETERS(i32);
pub const DOT11_STOP_AP_PARAMETERS_REVISION_1: u32 = 1u32;
pub const DOT11_STOP_AP_REASON_AP_ACTIVE: u32 = 3u32;
pub const DOT11_STOP_AP_REASON_CHANNEL_NOT_AVAILABLE: u32 = 2u32;
pub const DOT11_STOP_AP_REASON_FREQUENCY_NOT_AVAILABLE: u32 = 1u32;
pub const DOT11_STOP_AP_REASON_IHV_END: u32 = 4294967295u32;
pub const DOT11_STOP_AP_REASON_IHV_START: u32 = 4278190080u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_SUPPORTED_ANTENNA(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_SUPPORTED_ANTENNA_LIST(i32);
#[repr(C)]
pub struct DOT11_SUPPORTED_DATA_RATES_VALUE(i32);
#[repr(C)]
pub struct DOT11_SUPPORTED_DATA_RATES_VALUE_V2(i32);
#[repr(C)]
pub struct DOT11_SUPPORTED_DSSS_CHANNEL(i32);
#[repr(C)]
pub struct DOT11_SUPPORTED_DSSS_CHANNEL_LIST(i32);
#[repr(C)]
pub struct DOT11_SUPPORTED_OFDM_FREQUENCY(i32);
#[repr(C)]
pub struct DOT11_SUPPORTED_OFDM_FREQUENCY_LIST(i32);
#[repr(C)]
pub struct DOT11_SUPPORTED_PHY_TYPES(i32);
#[repr(C)]
pub struct DOT11_SUPPORTED_POWER_LEVELS(i32);
#[repr(transparent)]
pub struct DOT11_TEMP_TYPE(pub i32);
pub const dot11_temp_type_unknown: DOT11_TEMP_TYPE = DOT11_TEMP_TYPE(0i32);
pub const dot11_temp_type_1: DOT11_TEMP_TYPE = DOT11_TEMP_TYPE(1i32);
pub const dot11_temp_type_2: DOT11_TEMP_TYPE = DOT11_TEMP_TYPE(2i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_TKIPMIC_FAILURE_PARAMETERS(i32);
pub const DOT11_TKIPMIC_FAILURE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_UPDATE_IE(i32);
#[repr(transparent)]
pub struct DOT11_UPDATE_IE_OP(pub i32);
pub const dot11_update_ie_op_create_replace: DOT11_UPDATE_IE_OP = DOT11_UPDATE_IE_OP(1i32);
pub const dot11_update_ie_op_delete: DOT11_UPDATE_IE_OP = DOT11_UPDATE_IE_OP(2i32);
#[repr(C)]
pub struct DOT11_VENUEINFO(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_VWIFI_ATTRIBUTES(i32);
pub const DOT11_VWIFI_ATTRIBUTES_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_VWIFI_COMBINATION(i32);
pub const DOT11_VWIFI_COMBINATION_REVISION_1: u32 = 1u32;
pub const DOT11_VWIFI_COMBINATION_REVISION_2: u32 = 2u32;
pub const DOT11_VWIFI_COMBINATION_REVISION_3: u32 = 3u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_VWIFI_COMBINATION_V2(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_VWIFI_COMBINATION_V3(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_WEP_OFFLOAD(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_WEP_UPLOAD(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_WFD_ADDITIONAL_IE(i32);
pub const DOT11_WFD_ADDITIONAL_IE_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_WFD_ADVERTISED_SERVICE_DESCRIPTOR(i32);
#[repr(C)]
pub struct DOT11_WFD_ADVERTISED_SERVICE_LIST(i32);
#[repr(C)]
pub struct DOT11_WFD_ADVERTISEMENT_ID(i32);
pub const DOT11_WFD_APS2_SERVICE_TYPE_MAX_LENGTH: u32 = 21u32;
pub const DOT11_WFD_ASP2_INSTANCE_NAME_MAX_LENGTH: u32 = 63u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_WFD_ATTRIBUTES(i32);
pub const DOT11_WFD_ATTRIBUTES_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_WFD_CHANNEL(i32);
#[repr(C)]
pub struct DOT11_WFD_CONFIGURATION_TIMEOUT(i32);
pub const DOT11_WFD_DEVICE_AUTO_AVAILABILITY: u32 = 16u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_CONCURRENT_OPERATION: u32 = 4u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_WFD_DEVICE_CAPABILITY_CONFIG(i32);
pub const DOT11_WFD_DEVICE_CAPABILITY_CONFIG_REVISION_1: u32 = 1u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_CLIENT_DISCOVERABILITY: u32 = 2u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_DEVICE_LIMIT: u32 = 16u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_INFRASTRUCTURE_MANAGED: u32 = 8u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_P2P_INVITATION_PROCEDURE: u32 = 32u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_RESERVED_6: u32 = 64u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_RESERVED_7: u32 = 128u32;
pub const DOT11_WFD_DEVICE_CAPABILITY_SERVICE_DISCOVERY: u32 = 1u32;
#[repr(C)]
pub struct DOT11_WFD_DEVICE_ENTRY(i32);
pub const DOT11_WFD_DEVICE_HIGH_AVAILABILITY: u32 = 24u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_WFD_DEVICE_INFO(i32);
pub const DOT11_WFD_DEVICE_INFO_REVISION_1: u32 = 1u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_WFD_DEVICE_LISTEN_CHANNEL(i32);
pub const DOT11_WFD_DEVICE_LISTEN_CHANNEL_REVISION_1: u32 = 1u32;
pub const DOT11_WFD_DEVICE_NOT_DISCOVERABLE: u32 = 0u32;
#[repr(C)]
pub struct DOT11_WFD_DEVICE_TYPE(i32);
pub const DOT11_WFD_DISCOVER_COMPLETE_MAX_LIST_SIZE: u32 = 128u32;
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS(i32);
pub const DOT11_WFD_DISCOVER_COMPLETE_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_WFD_DISCOVER_DEVICE_FILTER(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_WFD_DISCOVER_REQUEST(i32);
pub const DOT11_WFD_DISCOVER_REQUEST_REVISION_1: u32 = 1u32;
#[repr(transparent)]
pub struct DOT11_WFD_DISCOVER_TYPE(pub i32);
pub const dot11_wfd_discover_type_scan_only: DOT11_WFD_DISCOVER_TYPE = DOT11_WFD_DISCOVER_TYPE(1i32);
pub const dot11_wfd_discover_type_find_only: DOT11_WFD_DISCOVER_TYPE = DOT11_WFD_DISCOVER_TYPE(2i32);
pub const dot11_wfd_discover_type_auto: DOT11_WFD_DISCOVER_TYPE = DOT11_WFD_DISCOVER_TYPE(3i32);
pub const dot11_wfd_discover_type_scan_social_channels: DOT11_WFD_DISCOVER_TYPE = DOT11_WFD_DISCOVER_TYPE(4i32);
pub const dot11_wfd_discover_type_forced: DOT11_WFD_DISCOVER_TYPE = DOT11_WFD_DISCOVER_TYPE(-2147483648i32);
#[repr(C)]
pub struct DOT11_WFD_GO_INTENT(i32);
pub const DOT11_WFD_GROUP_CAPABILITY_CROSS_CONNECTION_SUPPORTED: u32 = 16u32;
pub const DOT11_WFD_GROUP_CAPABILITY_EAPOL_KEY_IP_ADDRESS_ALLOCATION_SUPPORTED: u32 = 128u32;
pub const DOT11_WFD_GROUP_CAPABILITY_GROUP_LIMIT_REACHED: u32 = 4u32;
pub const DOT11_WFD_GROUP_CAPABILITY_GROUP_OWNER: u32 = 1u32;
pub const DOT11_WFD_GROUP_CAPABILITY_INTRABSS_DISTRIBUTION_SUPPORTED: u32 = 8u32;
pub const DOT11_WFD_GROUP_CAPABILITY_IN_GROUP_FORMATION: u32 = 64u32;
pub const DOT11_WFD_GROUP_CAPABILITY_NONE: u32 = 0u32;
pub const DOT11_WFD_GROUP_CAPABILITY_PERSISTENT_GROUP: u32 = 2u32;
pub const DOT11_WFD_GROUP_CAPABILITY_PERSISTENT_RECONNECT_SUPPORTED: u32 = 32u32;
pub const DOT11_WFD_GROUP_CAPABILITY_RESERVED_7: u32 = 128u32;
#[repr(C)]
pub struct DOT11_WFD_GROUP_ID(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_WFD_GROUP_JOIN_PARAMETERS(i32);
pub const DOT11_WFD_GROUP_JOIN_PARAMETERS_REVISION_1: u32 = 1u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG(i32);
pub const DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_REVISION_1: u32 = 1u32;
pub const DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_REVISION_2: u32 = 2u32;
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct DOT11_WFD_GROUP_OWNER_CAPABILITY_CONFIG_V2(i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_WFD_GROUP_START_PARAMETERS(i32);
pub const DOT11_WFD_GROUP_START_PARAMETERS_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_WFD_INVITATION_FLAGS(i32);
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_FROM_WLAN_CROSS_CONNECTION_POLICY: u32 = 1u32;
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_INFRASTRUCTURE_MANAGED_POLICY: u32 = 4u32;
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_NOT_MANAGED_INFRASTRUCTURE_CAPABLE: u32 = 2u32;
pub const DOT11_WFD_MINOR_REASON_DISASSOCIATED_WFD_COEXISTENCE_POLICY: u32 = 3u32;
pub const DOT11_WFD_MINOR_REASON_SUCCESS: u32 = 0u32;
#[repr(transparent)]
pub struct DOT11_WFD_SCAN_TYPE(pub i32);
pub const dot11_wfd_scan_type_active: DOT11_WFD_SCAN_TYPE = DOT11_WFD_SCAN_TYPE(1i32);
pub const dot11_wfd_scan_type_passive: DOT11_WFD_SCAN_TYPE = DOT11_WFD_SCAN_TYPE(2i32);
pub const dot11_wfd_scan_type_auto: DOT11_WFD_SCAN_TYPE = DOT11_WFD_SCAN_TYPE(3i32);
#[cfg(feature = "Win32_NetworkManagement_Ndis")]
#[repr(C)]
pub struct DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST(i32);
pub const DOT11_WFD_SECONDARY_DEVICE_TYPE_LIST_REVISION_1: u32 = 1u32;
#[repr(C)]
pub struct DOT11_WFD_SERVICE_HASH_LIST(i32);
pub const DOT11_WFD_SERVICE_INFORMATION_MAX_LENGTH: u32 = 65535u32;
pub const DOT11_WFD_SERVICE_NAME_MAX_LENGTH: u32 = 255u32;
#[repr(C)]
pub struct DOT11_WFD_SESSION_ID(i32);
#[repr(C)]
pub struct DOT11_WFD_SESSION_INFO(i32);
pub const DOT11_WFD_SESSION_INFO_MAX_LENGTH: u32 = 144u32;
pub const DOT11_WFD_STATUS_FAILED_INCOMPATIBLE_PARAMETERS: u32 = 2u32;
pub const DOT11_WFD_STATUS_FAILED_INCOMPATIBLE_PROVISIONING_METHOD: u32 = 10u32;
pub const DOT11_WFD_STATUS_FAILED_INFORMATION_IS_UNAVAILABLE: u32 = 1u32;
pub const DOT11_WFD_STATUS_FAILED_INVALID_PARAMETERS: u32 = 4u32;
pub const DOT11_WFD_STATUS_FAILED_LIMIT_REACHED: u32 = 3u32;
pub const DOT11_WFD_STATUS_FAILED_MATCHING_MAX_INTENT: u32 = 9u32;
pub const DOT11_WFD_STATUS_FAILED_NO_COMMON_CHANNELS: u32 = 7u32;
pub const DOT11_WFD_STATUS_FAILED_PREVIOUS_PROTOCOL_ERROR: u32 = 6u32;
pub const DOT11_WFD_STATUS_FAILED_REJECTED_BY_USER: u32 = 11u32;
pub const DOT11_WFD_STATUS_FAILED_UNABLE_TO_ACCOMODATE_REQUEST: u32 = 5u32;
pub const DOT11_WFD_STATUS_FAILED_UNKNOWN_WFD_GROUP: u32 = 8u32;
pub const DOT11_WFD_STATUS_SUCCESS: u32 = 0u32;
pub const DOT11_WFD_STATUS_SUCCESS_ACCEPTED_BY_USER: u32 = 12u32;
#[repr(C)]
pub struct DOT11_WME_AC_PARAMETERS(i32);
pub const DOT11_WME_PACKET: u32 = 256u32;
#[repr(C)]
pub struct DOT11_WME_UPDATE_IE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct DOT11_WPA_TSC(i32);
#[repr(transparent)]
pub struct DOT11_WPS_CONFIG_METHOD(pub i32);
pub const DOT11_WPS_CONFIG_METHOD_NULL: DOT11_WPS_CONFIG_METHOD = DOT11_WPS_CONFIG_METHOD(0i32);
pub const DOT11_WPS_CONFIG_METHOD_DISPLAY: DOT11_WPS_CONFIG_METHOD = DOT11_WPS_CONFIG_METHOD(8i32);
pub const DOT11_WPS_CONFIG_METHOD_NFC_TAG: DOT11_WPS_CONFIG_METHOD = DOT11_WPS_CONFIG_METHOD(32i32);
pub const DOT11_WPS_CONFIG_METHOD_NFC_INTERFACE: DOT11_WPS_CONFIG_METHOD = DOT11_WPS_CONFIG_METHOD(64i32);
pub const DOT11_WPS_CONFIG_METHOD_PUSHBUTTON: DOT11_WPS_CONFIG_METHOD = DOT11_WPS_CONFIG_METHOD(128i32);
pub const DOT11_WPS_CONFIG_METHOD_KEYPAD: DOT11_WPS_CONFIG_METHOD = DOT11_WPS_CONFIG_METHOD(256i32);
pub const DOT11_WPS_CONFIG_METHOD_WFDS_DEFAULT: DOT11_WPS_CONFIG_METHOD = DOT11_WPS_CONFIG_METHOD(4096i32);
#[repr(C)]
pub struct DOT11_WPS_DEVICE_NAME(i32);
pub const DOT11_WPS_DEVICE_NAME_MAX_LENGTH: u32 = 32u32;
#[repr(transparent)]
pub struct DOT11_WPS_DEVICE_PASSWORD_ID(pub i32);
pub const DOT11_WPS_PASSWORD_ID_DEFAULT: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(0i32);
pub const DOT11_WPS_PASSWORD_ID_USER_SPECIFIED: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(1i32);
pub const DOT11_WPS_PASSWORD_ID_MACHINE_SPECIFIED: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(2i32);
pub const DOT11_WPS_PASSWORD_ID_REKEY: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(3i32);
pub const DOT11_WPS_PASSWORD_ID_PUSHBUTTON: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(4i32);
pub const DOT11_WPS_PASSWORD_ID_REGISTRAR_SPECIFIED: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(5i32);
pub const DOT11_WPS_PASSWORD_ID_NFC_CONNECTION_HANDOVER: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(7i32);
pub const DOT11_WPS_PASSWORD_ID_WFD_SERVICES: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(8i32);
pub const DOT11_WPS_PASSWORD_ID_OOB_RANGE_MIN: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(16i32);
pub const DOT11_WPS_PASSWORD_ID_OOB_RANGE_MAX: DOT11_WPS_DEVICE_PASSWORD_ID = DOT11_WPS_DEVICE_PASSWORD_ID(65535i32);
pub const DOT11_WPS_MAX_MODEL_NAME_LENGTH: u32 = 32u32;
pub const DOT11_WPS_MAX_MODEL_NUMBER_LENGTH: u32 = 32u32;
pub const DOT11_WPS_MAX_PASSKEY_LENGTH: u32 = 8u32;
pub const DOT11_WPS_VERSION_1_0: u32 = 1u32;
pub const DOT11_WPS_VERSION_2_0: u32 = 2u32;
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_Enhanced: u32 = 4u32;
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_NoP2PSupported: u32 = 2u32;
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_NotSupported: u32 = 0u32;
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_SingleFunctionSupported: u32 = 1u32;
pub const DevProp_PciDevice_AcsCompatibleUpHierarchy_Supported: u32 = 3u32;
pub const DevProp_PciDevice_AcsSupport_Missing: u32 = 2u32;
pub const DevProp_PciDevice_AcsSupport_NotNeeded: u32 = 1u32;
pub const DevProp_PciDevice_AcsSupport_Present: u32 = 0u32;
pub const DevProp_PciDevice_BridgeType_PciConventional: u32 = 6u32;
pub const DevProp_PciDevice_BridgeType_PciExpressDownstreamSwitchPort: u32 = 10u32;
pub const DevProp_PciDevice_BridgeType_PciExpressEventCollector: u32 = 14u32;
pub const DevProp_PciDevice_BridgeType_PciExpressRootPort: u32 = 8u32;
pub const DevProp_PciDevice_BridgeType_PciExpressToPciXBridge: u32 = 11u32;
pub const DevProp_PciDevice_BridgeType_PciExpressTreatedAsPci: u32 = 13u32;
pub const DevProp_PciDevice_BridgeType_PciExpressUpstreamSwitchPort: u32 = 9u32;
pub const DevProp_PciDevice_BridgeType_PciX: u32 = 7u32;
pub const DevProp_PciDevice_BridgeType_PciXToExpressBridge: u32 = 12u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_100Mhz: u32 = 2u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_133MHZ: u32 = 3u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_66Mhz: u32 = 1u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_100Mhz: u32 = 6u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_133Mhz: u32 = 7u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode1_ECC_66Mhz: u32 = 5u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_100MHz: u32 = 10u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_133MHz: u32 = 11u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_266_66MHz: u32 = 9u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_100MHz: u32 = 14u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_133MHz: u32 = 15u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode2_533_66MHz: u32 = 13u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_PciX_Mode_Conventional_Pci: u32 = 0u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_Pci_Conventional_33MHz: u32 = 0u32;
pub const DevProp_PciDevice_CurrentSpeedAndMode_Pci_Conventional_66MHz: u32 = 1u32;
pub const DevProp_PciDevice_DeviceType_PciConventional: u32 = 0u32;
pub const DevProp_PciDevice_DeviceType_PciExpressEndpoint: u32 = 2u32;
pub const DevProp_PciDevice_DeviceType_PciExpressLegacyEndpoint: u32 = 3u32;
pub const DevProp_PciDevice_DeviceType_PciExpressRootComplexIntegratedEndpoint: u32 = 4u32;
pub const DevProp_PciDevice_DeviceType_PciExpressTreatedAsPci: u32 = 5u32;
pub const DevProp_PciDevice_DeviceType_PciX: u32 = 1u32;
pub const DevProp_PciDevice_InterruptType_LineBased: u32 = 1u32;
pub const DevProp_PciDevice_InterruptType_Msi: u32 = 2u32;
pub const DevProp_PciDevice_InterruptType_MsiX: u32 = 4u32;
pub const DevProp_PciDevice_SriovSupport_DidntGetVfBarSpace: u32 = 4u32;
pub const DevProp_PciDevice_SriovSupport_MissingAcs: u32 = 1u32;
pub const DevProp_PciDevice_SriovSupport_MissingPfDriver: u32 = 2u32;
pub const DevProp_PciDevice_SriovSupport_NoBusResource: u32 = 3u32;
pub const DevProp_PciDevice_SriovSupport_Ok: u32 = 0u32;
pub const DevProp_PciExpressDevice_LinkSpeed_Five_Gbps: u32 = 2u32;
pub const DevProp_PciExpressDevice_LinkSpeed_TwoAndHalf_Gbps: u32 = 1u32;
pub const DevProp_PciExpressDevice_LinkWidth_By_1: u32 = 1u32;
pub const DevProp_PciExpressDevice_LinkWidth_By_12: u32 = 12u32;
pub const DevProp_PciExpressDevice_LinkWidth_By_16: u32 = 16u32;
pub const DevProp_PciExpressDevice_LinkWidth_By_2: u32 = 2u32;
pub const DevProp_PciExpressDevice_LinkWidth_By_32: u32 = 32u32;
pub const DevProp_PciExpressDevice_LinkWidth_By_4: u32 = 4u32;
pub const DevProp_PciExpressDevice_LinkWidth_By_8: u32 = 8u32;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_1024Bytes: u32 = 3u32;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_128Bytes: u32 = 0u32;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_2048Bytes: u32 = 4u32;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_256Bytes: u32 = 1u32;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_4096Bytes: u32 = 5u32;
pub const DevProp_PciExpressDevice_PayloadOrRequestSize_512Bytes: u32 = 2u32;
pub const DevProp_PciExpressDevice_Spec_Version_10: u32 = 1u32;
pub const DevProp_PciExpressDevice_Spec_Version_11: u32 = 2u32;
pub const DevProp_PciRootBus_BusWidth_32Bits: u32 = 0u32;
pub const DevProp_PciRootBus_BusWidth_64Bits: u32 = 1u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_Conventional_33Mhz: u32 = 0u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_Conventional_66Mhz: u32 = 1u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_100Mhz: u32 = 9u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_133Mhz: u32 = 10u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_266_Mode2_66Mhz: u32 = 8u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_100Mhz: u32 = 12u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_133Mhz: u32 = 13u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_533_Mode2_66Mhz: u32 = 11u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_100Mhz: u32 = 3u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_133Mhz: u32 = 4u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_66Mhz: u32 = 2u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_100Mhz: u32 = 6u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_133Mhz: u32 = 7u32;
pub const DevProp_PciRootBus_CurrentSpeedAndMode_Pci_X_Mode1_ECC_66Mhz: u32 = 5u32;
pub const DevProp_PciRootBus_SecondaryInterface_PciConventional: u32 = 0u32;
pub const DevProp_PciRootBus_SecondaryInterface_PciExpress: u32 = 3u32;
pub const DevProp_PciRootBus_SecondaryInterface_PciXMode1: u32 = 1u32;
pub const DevProp_PciRootBus_SecondaryInterface_PciXMode2: u32 = 2u32;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_Conventional_33Mhz: u32 = 1u32;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_Conventional_66Mhz: u32 = 2u32;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_133Mhz: u32 = 8u32;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_266Mhz: u32 = 16u32;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_533Mhz: u32 = 32u32;
pub const DevProp_PciRootBus_SupportedSpeedsAndModes_Pci_X_66Mhz: u32 = 4u32;
pub const Dot11AdHocManager: ::windows_sys::core::GUID = ::windows_sys::GUID {
    data1: 3708201039,
    data2: 33725,
    data3: 19713,
    data4: [138, 185, 35, 137, 254, 160, 134, 158],
};
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
#[repr(transparent)]
pub struct IDot11AdHocInterface(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDot11AdHocInterfaceNotificationSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDot11AdHocManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDot11AdHocManagerNotificationSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDot11AdHocNetwork(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDot11AdHocNetworkNotificationSink(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDot11AdHocSecuritySettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumDot11AdHocInterfaces(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumDot11AdHocNetworks(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumDot11AdHocSecuritySettings(pub *mut ::core::ffi::c_void);
pub const L2_NOTIFICATION_CODE_GROUP_SIZE: u32 = 4096u32;
pub const L2_NOTIFICATION_CODE_PUBLIC_BEGIN: u32 = 0u32;
#[repr(C)]
pub struct L2_NOTIFICATION_DATA(i32);
pub const L2_NOTIFICATION_SOURCE_ALL: u32 = 65535u32;
pub const L2_NOTIFICATION_SOURCE_DOT3_AUTO_CONFIG: u32 = 1u32;
pub const L2_NOTIFICATION_SOURCE_NONE: u32 = 0u32;
pub const L2_NOTIFICATION_SOURCE_ONEX: u32 = 4u32;
pub const L2_NOTIFICATION_SOURCE_SECURITY: u32 = 2u32;
pub const L2_NOTIFICATION_SOURCE_WCM: u32 = 256u32;
pub const L2_NOTIFICATION_SOURCE_WCM_CSP: u32 = 512u32;
pub const L2_NOTIFICATION_SOURCE_WFD: u32 = 1024u32;
pub const L2_NOTIFICATION_SOURCE_WLAN_ACM: u32 = 8u32;
pub const L2_NOTIFICATION_SOURCE_WLAN_DEVICE_SERVICE: u32 = 2048u32;
pub const L2_NOTIFICATION_SOURCE_WLAN_HNWK: u32 = 128u32;
pub const L2_NOTIFICATION_SOURCE_WLAN_IHV: u32 = 64u32;
pub const L2_NOTIFICATION_SOURCE_WLAN_MSM: u32 = 16u32;
pub const L2_NOTIFICATION_SOURCE_WLAN_SECURITY: u32 = 32u32;
pub const L2_PROFILE_MAX_NAME_LENGTH: u32 = 256u32;
pub const L2_REASON_CODE_DOT11_AC_BASE: u32 = 131072u32;
pub const L2_REASON_CODE_DOT11_MSM_BASE: u32 = 196608u32;
pub const L2_REASON_CODE_DOT11_SECURITY_BASE: u32 = 262144u32;
pub const L2_REASON_CODE_DOT3_AC_BASE: u32 = 393216u32;
pub const L2_REASON_CODE_DOT3_MSM_BASE: u32 = 458752u32;
pub const L2_REASON_CODE_GEN_BASE: u32 = 65536u32;
pub const L2_REASON_CODE_GROUP_SIZE: u32 = 65536u32;
pub const L2_REASON_CODE_IHV_BASE: u32 = 589824u32;
pub const L2_REASON_CODE_ONEX_BASE: u32 = 327680u32;
pub const L2_REASON_CODE_PROFILE_BASE: u32 = 524288u32;
pub const L2_REASON_CODE_PROFILE_MISSING: u32 = 1u32;
pub const L2_REASON_CODE_RESERVED_BASE: u32 = 720896u32;
pub const L2_REASON_CODE_SUCCESS: u32 = 0u32;
pub const L2_REASON_CODE_UNKNOWN: u32 = 65537u32;
pub const L2_REASON_CODE_WIMAX_BASE: u32 = 655360u32;
pub const MAX_NUM_SUPPORTED_RATES: u32 = 8u32;
pub const MAX_NUM_SUPPORTED_RATES_V2: u32 = 255u32;
pub const NDIS_PACKET_TYPE_802_11_ALL_MULTICAST_DATA: u32 = 4u32;
pub const NDIS_PACKET_TYPE_802_11_BROADCAST_DATA: u32 = 8u32;
pub const NDIS_PACKET_TYPE_802_11_DIRECTED_DATA: u32 = 1u32;
pub const NDIS_PACKET_TYPE_802_11_MULTICAST_DATA: u32 = 2u32;
pub const NDIS_PACKET_TYPE_802_11_PROMISCUOUS_DATA: u32 = 32u32;
pub const OID_DOT11_AP_JOIN_REQUEST: u32 = 218170205u32;
pub const OID_DOT11_ATIM_WINDOW: u32 = 218170122u32;
pub const OID_DOT11_BEACON_PERIOD: u32 = 218170139u32;
pub const OID_DOT11_CCA_MODE_SUPPORTED: u32 = 218170166u32;
pub const OID_DOT11_CCA_WATCHDOG_COUNT_MAX: u32 = 218170170u32;
pub const OID_DOT11_CCA_WATCHDOG_COUNT_MIN: u32 = 218170172u32;
pub const OID_DOT11_CCA_WATCHDOG_TIMER_MAX: u32 = 218170169u32;
pub const OID_DOT11_CCA_WATCHDOG_TIMER_MIN: u32 = 218170171u32;
pub const OID_DOT11_CFP_MAX_DURATION: u32 = 218170136u32;
pub const OID_DOT11_CFP_PERIOD: u32 = 218170135u32;
pub const OID_DOT11_CF_POLLABLE: u32 = 218170134u32;
pub const OID_DOT11_CHANNEL_AGILITY_ENABLED: u32 = 218170184u32;
pub const OID_DOT11_CHANNEL_AGILITY_PRESENT: u32 = 218170183u32;
pub const OID_DOT11_COUNTERS_ENTRY: u32 = 218170149u32;
pub const OID_DOT11_COUNTRY_STRING: u32 = 218170188u32;
pub const OID_DOT11_CURRENT_ADDRESS: u32 = 218171138u32;
pub const OID_DOT11_CURRENT_CCA_MODE: u32 = 218170167u32;
pub const OID_DOT11_CURRENT_CHANNEL: u32 = 218170165u32;
pub const OID_DOT11_CURRENT_CHANNEL_NUMBER: u32 = 218170159u32;
pub const OID_DOT11_CURRENT_DWELL_TIME: u32 = 218170161u32;
pub const OID_DOT11_CURRENT_FREQUENCY: u32 = 218170178u32;
pub const OID_DOT11_CURRENT_INDEX: u32 = 218170164u32;
pub const OID_DOT11_CURRENT_OFFLOAD_CAPABILITY: u32 = 218170113u32;
pub const OID_DOT11_CURRENT_OPERATION_MODE: u32 = 218170120u32;
pub const OID_DOT11_CURRENT_OPTIONAL_CAPABILITY: u32 = 218170131u32;
pub const OID_DOT11_CURRENT_PACKET_FILTER: u32 = 218170121u32;
pub const OID_DOT11_CURRENT_PATTERN: u32 = 218170163u32;
pub const OID_DOT11_CURRENT_PHY_TYPE: u32 = 218170124u32;
pub const OID_DOT11_CURRENT_REG_DOMAIN: u32 = 218170151u32;
pub const OID_DOT11_CURRENT_RX_ANTENNA: u32 = 218170155u32;
pub const OID_DOT11_CURRENT_SET: u32 = 218170162u32;
pub const OID_DOT11_CURRENT_TX_ANTENNA: u32 = 218170153u32;
pub const OID_DOT11_CURRENT_TX_POWER_LEVEL: u32 = 218170157u32;
pub const OID_DOT11_DEFAULT_WEP_OFFLOAD: u32 = 218170116u32;
pub const OID_DOT11_DEFAULT_WEP_UPLOAD: u32 = 218170117u32;
pub const OID_DOT11_DIVERSITY_SELECTION_RX: u32 = 218170176u32;
pub const OID_DOT11_DIVERSITY_SUPPORT: u32 = 218170154u32;
pub const OID_DOT11_DSSS_OFDM_OPTION_ENABLED: u32 = 218170209u32;
pub const OID_DOT11_DSSS_OFDM_OPTION_IMPLEMENTED: u32 = 218170208u32;
pub const OID_DOT11_DTIM_PERIOD: u32 = 218170140u32;
pub const OID_DOT11_ED_THRESHOLD: u32 = 218170168u32;
pub const OID_DOT11_EHCC_CAPABILITY_ENABLED: u32 = 218170193u32;
pub const OID_DOT11_EHCC_CAPABILITY_IMPLEMENTED: u32 = 218170192u32;
pub const OID_DOT11_EHCC_NUMBER_OF_CHANNELS_FAMILY_INDEX: u32 = 218170191u32;
pub const OID_DOT11_EHCC_PRIME_RADIX: u32 = 218170190u32;
pub const OID_DOT11_ERP_PBCC_OPTION_ENABLED: u32 = 218170207u32;
pub const OID_DOT11_ERP_PBCC_OPTION_IMPLEMENTED: u32 = 218170206u32;
pub const OID_DOT11_FRAGMENTATION_THRESHOLD: u32 = 218170146u32;
pub const OID_DOT11_FREQUENCY_BANDS_SUPPORTED: u32 = 218170180u32;
pub const OID_DOT11_HOPPING_PATTERN: u32 = 218170199u32;
pub const OID_DOT11_HOP_ALGORITHM_ADOPTED: u32 = 218170194u32;
pub const OID_DOT11_HOP_MODULUS: u32 = 218170197u32;
pub const OID_DOT11_HOP_OFFSET: u32 = 218170198u32;
pub const OID_DOT11_HOP_TIME: u32 = 218170158u32;
pub const OID_DOT11_HR_CCA_MODE_SUPPORTED: u32 = 218170185u32;
pub const OID_DOT11_JOIN_REQUEST: u32 = 218170125u32;
pub const OID_DOT11_LONG_RETRY_LIMIT: u32 = 218170145u32;
pub const OID_DOT11_MAC_ADDRESS: u32 = 218170142u32;
pub const OID_DOT11_MAXIMUM_LIST_SIZE: u32 = 218171141u32;
pub const OID_DOT11_MAX_DWELL_TIME: u32 = 218170160u32;
pub const OID_DOT11_MAX_MAC_ADDRESS_STATES: u32 = 218170212u32;
pub const OID_DOT11_MAX_RECEIVE_LIFETIME: u32 = 218170148u32;
pub const OID_DOT11_MAX_TRANSMIT_MSDU_LIFETIME: u32 = 218170147u32;
pub const OID_DOT11_MEDIUM_OCCUPANCY_LIMIT: u32 = 218170133u32;
pub const OID_DOT11_MPDU_MAX_LENGTH: u32 = 218170118u32;
pub const OID_DOT11_MULTICAST_LIST: u32 = 218171140u32;
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY: u32 = 218170189u32;
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY_ENABLED: u32 = 218170187u32;
pub const OID_DOT11_MULTI_DOMAIN_CAPABILITY_IMPLEMENTED: u32 = 218170186u32;
pub const OID_DOT11_NDIS_START: u32 = 218170112u32;
pub const OID_DOT11_NIC_POWER_STATE: u32 = 218170129u32;
pub const OID_DOT11_NIC_SPECIFIC_EXTENSION: u32 = 218170204u32;
pub const OID_DOT11_NUMBER_OF_HOPPING_SETS: u32 = 218170196u32;
pub const OID_DOT11_OFFLOAD_CAPABILITY: u32 = 218170112u32;
pub const OID_DOT11_OPERATIONAL_RATE_SET: u32 = 218170138u32;
pub const OID_DOT11_OPERATION_MODE_CAPABILITY: u32 = 218170119u32;
pub const OID_DOT11_OPTIONAL_CAPABILITY: u32 = 218170130u32;
pub const OID_DOT11_PBCC_OPTION_IMPLEMENTED: u32 = 218170182u32;
pub const OID_DOT11_PERMANENT_ADDRESS: u32 = 218171139u32;
pub const OID_DOT11_POWER_MGMT_MODE: u32 = 218170137u32;
pub const OID_DOT11_PRIVATE_OIDS_START: u32 = 218171136u32;
pub const OID_DOT11_QOS_TX_DURATION: u32 = 218170219u32;
pub const OID_DOT11_QOS_TX_MEDIUM_TIME: u32 = 218170220u32;
pub const OID_DOT11_QOS_TX_QUEUES_SUPPORTED: u32 = 218170218u32;
pub const OID_DOT11_RANDOM_TABLE_FIELD_NUMBER: u32 = 218170200u32;
pub const OID_DOT11_RANDOM_TABLE_FLAG: u32 = 218170195u32;
pub const OID_DOT11_RECV_SENSITIVITY_LIST: u32 = 218170213u32;
pub const OID_DOT11_REG_DOMAINS_SUPPORT_VALUE: u32 = 218170173u32;
pub const OID_DOT11_RESET_REQUEST: u32 = 218170128u32;
pub const OID_DOT11_RF_USAGE: u32 = 218170203u32;
pub const OID_DOT11_RSSI_RANGE: u32 = 218170202u32;
pub const OID_DOT11_RTS_THRESHOLD: u32 = 218170143u32;
pub const OID_DOT11_SCAN_REQUEST: u32 = 218170123u32;
pub const OID_DOT11_SHORT_PREAMBLE_OPTION_IMPLEMENTED: u32 = 218170181u32;
pub const OID_DOT11_SHORT_RETRY_LIMIT: u32 = 218170144u32;
pub const OID_DOT11_SHORT_SLOT_TIME_OPTION_ENABLED: u32 = 218170211u32;
pub const OID_DOT11_SHORT_SLOT_TIME_OPTION_IMPLEMENTED: u32 = 218170210u32;
pub const OID_DOT11_START_REQUEST: u32 = 218170126u32;
pub const OID_DOT11_STATION_ID: u32 = 218170132u32;
pub const OID_DOT11_SUPPORTED_DATA_RATES_VALUE: u32 = 218170177u32;
pub const OID_DOT11_SUPPORTED_DSSS_CHANNEL_LIST: u32 = 218170222u32;
pub const OID_DOT11_SUPPORTED_OFDM_FREQUENCY_LIST: u32 = 218170221u32;
pub const OID_DOT11_SUPPORTED_PHY_TYPES: u32 = 218170150u32;
pub const OID_DOT11_SUPPORTED_POWER_LEVELS: u32 = 218170156u32;
pub const OID_DOT11_SUPPORTED_RX_ANTENNA: u32 = 218170175u32;
pub const OID_DOT11_SUPPORTED_TX_ANTENNA: u32 = 218170174u32;
pub const OID_DOT11_TEMP_TYPE: u32 = 218170152u32;
pub const OID_DOT11_TI_THRESHOLD: u32 = 218170179u32;
pub const OID_DOT11_UPDATE_IE: u32 = 218170127u32;
pub const OID_DOT11_WEP_ICV_ERROR_COUNT: u32 = 218170141u32;
pub const OID_DOT11_WEP_OFFLOAD: u32 = 218170114u32;
pub const OID_DOT11_WEP_UPLOAD: u32 = 218170115u32;
pub const OID_DOT11_WME_AC_PARAMETERS: u32 = 218170216u32;
pub const OID_DOT11_WME_ENABLED: u32 = 218170215u32;
pub const OID_DOT11_WME_IMPLEMENTED: u32 = 218170214u32;
pub const OID_DOT11_WME_UPDATE_IE: u32 = 218170217u32;
pub const OID_DOT11_WPA_TSC: u32 = 218170201u32;
#[repr(transparent)]
pub struct ONEX_AUTH_IDENTITY(pub i32);
pub const OneXAuthIdentityNone: ONEX_AUTH_IDENTITY = ONEX_AUTH_IDENTITY(0i32);
pub const OneXAuthIdentityMachine: ONEX_AUTH_IDENTITY = ONEX_AUTH_IDENTITY(1i32);
pub const OneXAuthIdentityUser: ONEX_AUTH_IDENTITY = ONEX_AUTH_IDENTITY(2i32);
pub const OneXAuthIdentityExplicitUser: ONEX_AUTH_IDENTITY = ONEX_AUTH_IDENTITY(3i32);
pub const OneXAuthIdentityGuest: ONEX_AUTH_IDENTITY = ONEX_AUTH_IDENTITY(4i32);
pub const OneXAuthIdentityInvalid: ONEX_AUTH_IDENTITY = ONEX_AUTH_IDENTITY(5i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ONEX_AUTH_PARAMS(i32);
#[repr(transparent)]
pub struct ONEX_AUTH_RESTART_REASON(pub i32);
pub const OneXRestartReasonPeerInitiated: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(0i32);
pub const OneXRestartReasonMsmInitiated: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(1i32);
pub const OneXRestartReasonOneXHeldStateTimeout: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(2i32);
pub const OneXRestartReasonOneXAuthTimeout: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(3i32);
pub const OneXRestartReasonOneXConfigurationChanged: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(4i32);
pub const OneXRestartReasonOneXUserChanged: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(5i32);
pub const OneXRestartReasonQuarantineStateChanged: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(6i32);
pub const OneXRestartReasonAltCredsTrial: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(7i32);
pub const OneXRestartReasonInvalid: ONEX_AUTH_RESTART_REASON = ONEX_AUTH_RESTART_REASON(8i32);
#[repr(transparent)]
pub struct ONEX_AUTH_STATUS(pub i32);
pub const OneXAuthNotStarted: ONEX_AUTH_STATUS = ONEX_AUTH_STATUS(0i32);
pub const OneXAuthInProgress: ONEX_AUTH_STATUS = ONEX_AUTH_STATUS(1i32);
pub const OneXAuthNoAuthenticatorFound: ONEX_AUTH_STATUS = ONEX_AUTH_STATUS(2i32);
pub const OneXAuthSuccess: ONEX_AUTH_STATUS = ONEX_AUTH_STATUS(3i32);
pub const OneXAuthFailure: ONEX_AUTH_STATUS = ONEX_AUTH_STATUS(4i32);
pub const OneXAuthInvalid: ONEX_AUTH_STATUS = ONEX_AUTH_STATUS(5i32);
#[cfg(feature = "Win32_Security_ExtensibleAuthenticationProtocol")]
#[repr(C)]
pub struct ONEX_EAP_ERROR(i32);
#[repr(transparent)]
pub struct ONEX_EAP_METHOD_BACKEND_SUPPORT(pub i32);
pub const OneXEapMethodBackendSupportUnknown: ONEX_EAP_METHOD_BACKEND_SUPPORT = ONEX_EAP_METHOD_BACKEND_SUPPORT(0i32);
pub const OneXEapMethodBackendSupported: ONEX_EAP_METHOD_BACKEND_SUPPORT = ONEX_EAP_METHOD_BACKEND_SUPPORT(1i32);
pub const OneXEapMethodBackendUnsupported: ONEX_EAP_METHOD_BACKEND_SUPPORT = ONEX_EAP_METHOD_BACKEND_SUPPORT(2i32);
#[repr(transparent)]
pub struct ONEX_NOTIFICATION_TYPE(pub i32);
pub const OneXPublicNotificationBase: ONEX_NOTIFICATION_TYPE = ONEX_NOTIFICATION_TYPE(0i32);
pub const OneXNotificationTypeResultUpdate: ONEX_NOTIFICATION_TYPE = ONEX_NOTIFICATION_TYPE(1i32);
pub const OneXNotificationTypeAuthRestarted: ONEX_NOTIFICATION_TYPE = ONEX_NOTIFICATION_TYPE(2i32);
pub const OneXNotificationTypeEventInvalid: ONEX_NOTIFICATION_TYPE = ONEX_NOTIFICATION_TYPE(3i32);
pub const OneXNumNotifications: ONEX_NOTIFICATION_TYPE = ONEX_NOTIFICATION_TYPE(3i32);
#[repr(transparent)]
pub struct ONEX_REASON_CODE(pub i32);
pub const ONEX_REASON_CODE_SUCCESS: ONEX_REASON_CODE = ONEX_REASON_CODE(0i32);
pub const ONEX_REASON_START: ONEX_REASON_CODE = ONEX_REASON_CODE(327680i32);
pub const ONEX_UNABLE_TO_IDENTIFY_USER: ONEX_REASON_CODE = ONEX_REASON_CODE(327681i32);
pub const ONEX_IDENTITY_NOT_FOUND: ONEX_REASON_CODE = ONEX_REASON_CODE(327682i32);
pub const ONEX_UI_DISABLED: ONEX_REASON_CODE = ONEX_REASON_CODE(327683i32);
pub const ONEX_UI_FAILURE: ONEX_REASON_CODE = ONEX_REASON_CODE(327684i32);
pub const ONEX_EAP_FAILURE_RECEIVED: ONEX_REASON_CODE = ONEX_REASON_CODE(327685i32);
pub const ONEX_AUTHENTICATOR_NO_LONGER_PRESENT: ONEX_REASON_CODE = ONEX_REASON_CODE(327686i32);
pub const ONEX_NO_RESPONSE_TO_IDENTITY: ONEX_REASON_CODE = ONEX_REASON_CODE(327687i32);
pub const ONEX_PROFILE_VERSION_NOT_SUPPORTED: ONEX_REASON_CODE = ONEX_REASON_CODE(327688i32);
pub const ONEX_PROFILE_INVALID_LENGTH: ONEX_REASON_CODE = ONEX_REASON_CODE(327689i32);
pub const ONEX_PROFILE_DISALLOWED_EAP_TYPE: ONEX_REASON_CODE = ONEX_REASON_CODE(327690i32);
pub const ONEX_PROFILE_INVALID_EAP_TYPE_OR_FLAG: ONEX_REASON_CODE = ONEX_REASON_CODE(327691i32);
pub const ONEX_PROFILE_INVALID_ONEX_FLAGS: ONEX_REASON_CODE = ONEX_REASON_CODE(327692i32);
pub const ONEX_PROFILE_INVALID_TIMER_VALUE: ONEX_REASON_CODE = ONEX_REASON_CODE(327693i32);
pub const ONEX_PROFILE_INVALID_SUPPLICANT_MODE: ONEX_REASON_CODE = ONEX_REASON_CODE(327694i32);
pub const ONEX_PROFILE_INVALID_AUTH_MODE: ONEX_REASON_CODE = ONEX_REASON_CODE(327695i32);
pub const ONEX_PROFILE_INVALID_EAP_CONNECTION_PROPERTIES: ONEX_REASON_CODE = ONEX_REASON_CODE(327696i32);
pub const ONEX_UI_CANCELLED: ONEX_REASON_CODE = ONEX_REASON_CODE(327697i32);
pub const ONEX_PROFILE_INVALID_EXPLICIT_CREDENTIALS: ONEX_REASON_CODE = ONEX_REASON_CODE(327698i32);
pub const ONEX_PROFILE_EXPIRED_EXPLICIT_CREDENTIALS: ONEX_REASON_CODE = ONEX_REASON_CODE(327699i32);
pub const ONEX_UI_NOT_PERMITTED: ONEX_REASON_CODE = ONEX_REASON_CODE(327700i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct ONEX_RESULT_UPDATE_DATA(i32);
#[repr(C)]
pub struct ONEX_STATUS(i32);
#[repr(C)]
pub struct ONEX_USER_INFO(i32);
#[repr(C)]
pub struct ONEX_VARIABLE_BLOB(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WFDSVC_CONNECTION_CAPABILITY(i32);
pub const WFDSVC_CONNECTION_CAPABILITY_CLIENT: u32 = 2u32;
pub const WFDSVC_CONNECTION_CAPABILITY_GO: u32 = 4u32;
pub const WFDSVC_CONNECTION_CAPABILITY_NEW: u32 = 1u32;
pub const WFD_API_VERSION: u32 = 1u32;
pub const WFD_API_VERSION_1_0: u32 = 1u32;
#[repr(C)]
pub struct WFD_GROUP_ID(i32);
#[cfg(feature = "Win32_Foundation")]
pub type WFD_OPEN_SESSION_COMPLETE_CALLBACK = unsafe extern "system" fn(hsessionhandle: super::super::Foundation::HANDLE, pvcontext: *const ::core::ffi::c_void, guidsessioninterface: ::windows_sys::core::GUID, dwerror: u32, dwreasoncode: u32);
#[repr(transparent)]
pub struct WFD_ROLE_TYPE(pub i32);
pub const WFD_ROLE_TYPE_NONE: WFD_ROLE_TYPE = WFD_ROLE_TYPE(0i32);
pub const WFD_ROLE_TYPE_DEVICE: WFD_ROLE_TYPE = WFD_ROLE_TYPE(1i32);
pub const WFD_ROLE_TYPE_GROUP_OWNER: WFD_ROLE_TYPE = WFD_ROLE_TYPE(2i32);
pub const WFD_ROLE_TYPE_CLIENT: WFD_ROLE_TYPE = WFD_ROLE_TYPE(4i32);
pub const WFD_ROLE_TYPE_MAX: WFD_ROLE_TYPE = WFD_ROLE_TYPE(5i32);
#[repr(transparent)]
pub struct WLAN_ADHOC_NETWORK_STATE(pub i32);
pub const wlan_adhoc_network_state_formed: WLAN_ADHOC_NETWORK_STATE = WLAN_ADHOC_NETWORK_STATE(0i32);
pub const wlan_adhoc_network_state_connected: WLAN_ADHOC_NETWORK_STATE = WLAN_ADHOC_NETWORK_STATE(1i32);
pub const WLAN_API_VERSION: u32 = 2u32;
pub const WLAN_API_VERSION_1_0: u32 = 1u32;
pub const WLAN_API_VERSION_2_0: u32 = 2u32;
#[repr(C)]
pub struct WLAN_ASSOCIATION_ATTRIBUTES(i32);
#[repr(C)]
pub struct WLAN_AUTH_CIPHER_PAIR_LIST(i32);
#[repr(transparent)]
pub struct WLAN_AUTOCONF_OPCODE(pub i32);
pub const wlan_autoconf_opcode_start: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(0i32);
pub const wlan_autoconf_opcode_show_denied_networks: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(1i32);
pub const wlan_autoconf_opcode_power_setting: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(2i32);
pub const wlan_autoconf_opcode_only_use_gp_profiles_for_allowed_networks: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(3i32);
pub const wlan_autoconf_opcode_allow_explicit_creds: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(4i32);
pub const wlan_autoconf_opcode_block_period: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(5i32);
pub const wlan_autoconf_opcode_allow_virtual_station_extensibility: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(6i32);
pub const wlan_autoconf_opcode_end: WLAN_AUTOCONF_OPCODE = WLAN_AUTOCONF_OPCODE(7i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLAN_AVAILABLE_NETWORK(i32);
pub const WLAN_AVAILABLE_NETWORK_ANQP_SUPPORTED: u32 = 32u32;
pub const WLAN_AVAILABLE_NETWORK_AUTO_CONNECT_FAILED: u32 = 256u32;
pub const WLAN_AVAILABLE_NETWORK_CONNECTED: u32 = 1u32;
pub const WLAN_AVAILABLE_NETWORK_CONSOLE_USER_PROFILE: u32 = 4u32;
pub const WLAN_AVAILABLE_NETWORK_HAS_PROFILE: u32 = 2u32;
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_DOMAIN: u32 = 64u32;
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_ENABLED: u32 = 16u32;
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_ROAMING: u32 = 128u32;
pub const WLAN_AVAILABLE_NETWORK_INCLUDE_ALL_ADHOC_PROFILES: u32 = 1u32;
pub const WLAN_AVAILABLE_NETWORK_INCLUDE_ALL_MANUAL_HIDDEN_PROFILES: u32 = 2u32;
pub const WLAN_AVAILABLE_NETWORK_INTERWORKING_SUPPORTED: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLAN_AVAILABLE_NETWORK_LIST(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLAN_AVAILABLE_NETWORK_LIST_V2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLAN_AVAILABLE_NETWORK_V2(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLAN_BSS_ENTRY(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLAN_BSS_LIST(i32);
pub const WLAN_CONNECTION_ADHOC_JOIN_ONLY: u32 = 2u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLAN_CONNECTION_ATTRIBUTES(i32);
pub const WLAN_CONNECTION_EAPOL_PASSTHROUGH: u32 = 8u32;
pub const WLAN_CONNECTION_HIDDEN_NETWORK: u32 = 1u32;
pub const WLAN_CONNECTION_IGNORE_PRIVACY_BIT: u32 = 4u32;
#[repr(transparent)]
pub struct WLAN_CONNECTION_MODE(pub i32);
pub const wlan_connection_mode_profile: WLAN_CONNECTION_MODE = WLAN_CONNECTION_MODE(0i32);
pub const wlan_connection_mode_temporary_profile: WLAN_CONNECTION_MODE = WLAN_CONNECTION_MODE(1i32);
pub const wlan_connection_mode_discovery_secure: WLAN_CONNECTION_MODE = WLAN_CONNECTION_MODE(2i32);
pub const wlan_connection_mode_discovery_unsecure: WLAN_CONNECTION_MODE = WLAN_CONNECTION_MODE(3i32);
pub const wlan_connection_mode_auto: WLAN_CONNECTION_MODE = WLAN_CONNECTION_MODE(4i32);
pub const wlan_connection_mode_invalid: WLAN_CONNECTION_MODE = WLAN_CONNECTION_MODE(5i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLAN_CONNECTION_NOTIFICATION_DATA(i32);
#[repr(transparent)]
pub struct WLAN_CONNECTION_NOTIFICATION_FLAGS(pub u32);
pub const WLAN_CONNECTION_NOTIFICATION_ADHOC_NETWORK_FORMED: WLAN_CONNECTION_NOTIFICATION_FLAGS = WLAN_CONNECTION_NOTIFICATION_FLAGS(1u32);
pub const WLAN_CONNECTION_NOTIFICATION_CONSOLE_USER_PROFILE: WLAN_CONNECTION_NOTIFICATION_FLAGS = WLAN_CONNECTION_NOTIFICATION_FLAGS(4u32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct WLAN_CONNECTION_PARAMETERS(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_NetworkManagement_Ndis"))]
#[repr(C)]
pub struct WLAN_CONNECTION_PARAMETERS_V2(i32);
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE: u32 = 16u32;
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE_CONNECTION_MODE_AUTO: u32 = 32u32;
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE_OVERWRITE_EXISTING: u32 = 64u32;
#[repr(C)]
pub struct WLAN_COUNTRY_OR_REGION_STRING_LIST(i32);
#[repr(C)]
pub struct WLAN_DEVICE_SERVICE_GUID_LIST(i32);
#[repr(C)]
pub struct WLAN_DEVICE_SERVICE_NOTIFICATION_DATA(i32);
#[repr(transparent)]
pub struct WLAN_FILTER_LIST_TYPE(pub i32);
pub const wlan_filter_list_type_gp_permit: WLAN_FILTER_LIST_TYPE = WLAN_FILTER_LIST_TYPE(0i32);
pub const wlan_filter_list_type_gp_deny: WLAN_FILTER_LIST_TYPE = WLAN_FILTER_LIST_TYPE(1i32);
pub const wlan_filter_list_type_user_permit: WLAN_FILTER_LIST_TYPE = WLAN_FILTER_LIST_TYPE(2i32);
pub const wlan_filter_list_type_user_deny: WLAN_FILTER_LIST_TYPE = WLAN_FILTER_LIST_TYPE(3i32);
#[repr(C)]
pub struct WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS(i32);
#[repr(C)]
pub struct WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE(i32);
#[repr(transparent)]
pub struct WLAN_HOSTED_NETWORK_NOTIFICATION_CODE(pub i32);
pub const wlan_hosted_network_state_change: WLAN_HOSTED_NETWORK_NOTIFICATION_CODE = WLAN_HOSTED_NETWORK_NOTIFICATION_CODE(4096i32);
pub const wlan_hosted_network_peer_state_change: WLAN_HOSTED_NETWORK_NOTIFICATION_CODE = WLAN_HOSTED_NETWORK_NOTIFICATION_CODE(4097i32);
pub const wlan_hosted_network_radio_state_change: WLAN_HOSTED_NETWORK_NOTIFICATION_CODE = WLAN_HOSTED_NETWORK_NOTIFICATION_CODE(4098i32);
#[repr(transparent)]
pub struct WLAN_HOSTED_NETWORK_OPCODE(pub i32);
pub const wlan_hosted_network_opcode_connection_settings: WLAN_HOSTED_NETWORK_OPCODE = WLAN_HOSTED_NETWORK_OPCODE(0i32);
pub const wlan_hosted_network_opcode_security_settings: WLAN_HOSTED_NETWORK_OPCODE = WLAN_HOSTED_NETWORK_OPCODE(1i32);
pub const wlan_hosted_network_opcode_station_profile: WLAN_HOSTED_NETWORK_OPCODE = WLAN_HOSTED_NETWORK_OPCODE(2i32);
pub const wlan_hosted_network_opcode_enable: WLAN_HOSTED_NETWORK_OPCODE = WLAN_HOSTED_NETWORK_OPCODE(3i32);
#[repr(transparent)]
pub struct WLAN_HOSTED_NETWORK_PEER_AUTH_STATE(pub i32);
pub const wlan_hosted_network_peer_state_invalid: WLAN_HOSTED_NETWORK_PEER_AUTH_STATE = WLAN_HOSTED_NETWORK_PEER_AUTH_STATE(0i32);
pub const wlan_hosted_network_peer_state_authenticated: WLAN_HOSTED_NETWORK_PEER_AUTH_STATE = WLAN_HOSTED_NETWORK_PEER_AUTH_STATE(1i32);
#[repr(C)]
pub struct WLAN_HOSTED_NETWORK_PEER_STATE(i32);
#[repr(C)]
pub struct WLAN_HOSTED_NETWORK_RADIO_STATE(i32);
#[repr(transparent)]
pub struct WLAN_HOSTED_NETWORK_REASON(pub i32);
pub const wlan_hosted_network_reason_success: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(0i32);
pub const wlan_hosted_network_reason_unspecified: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(1i32);
pub const wlan_hosted_network_reason_bad_parameters: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(2i32);
pub const wlan_hosted_network_reason_service_shutting_down: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(3i32);
pub const wlan_hosted_network_reason_insufficient_resources: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(4i32);
pub const wlan_hosted_network_reason_elevation_required: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(5i32);
pub const wlan_hosted_network_reason_read_only: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(6i32);
pub const wlan_hosted_network_reason_persistence_failed: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(7i32);
pub const wlan_hosted_network_reason_crypt_error: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(8i32);
pub const wlan_hosted_network_reason_impersonation: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(9i32);
pub const wlan_hosted_network_reason_stop_before_start: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(10i32);
pub const wlan_hosted_network_reason_interface_available: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(11i32);
pub const wlan_hosted_network_reason_interface_unavailable: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(12i32);
pub const wlan_hosted_network_reason_miniport_stopped: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(13i32);
pub const wlan_hosted_network_reason_miniport_started: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(14i32);
pub const wlan_hosted_network_reason_incompatible_connection_started: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(15i32);
pub const wlan_hosted_network_reason_incompatible_connection_stopped: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(16i32);
pub const wlan_hosted_network_reason_user_action: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(17i32);
pub const wlan_hosted_network_reason_client_abort: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(18i32);
pub const wlan_hosted_network_reason_ap_start_failed: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(19i32);
pub const wlan_hosted_network_reason_peer_arrived: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(20i32);
pub const wlan_hosted_network_reason_peer_departed: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(21i32);
pub const wlan_hosted_network_reason_peer_timeout: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(22i32);
pub const wlan_hosted_network_reason_gp_denied: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(23i32);
pub const wlan_hosted_network_reason_service_unavailable: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(24i32);
pub const wlan_hosted_network_reason_device_change: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(25i32);
pub const wlan_hosted_network_reason_properties_change: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(26i32);
pub const wlan_hosted_network_reason_virtual_station_blocking_use: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(27i32);
pub const wlan_hosted_network_reason_service_available_on_virtual_station: WLAN_HOSTED_NETWORK_REASON = WLAN_HOSTED_NETWORK_REASON(28i32);
#[repr(C)]
pub struct WLAN_HOSTED_NETWORK_SECURITY_SETTINGS(i32);
#[repr(transparent)]
pub struct WLAN_HOSTED_NETWORK_STATE(pub i32);
pub const wlan_hosted_network_unavailable: WLAN_HOSTED_NETWORK_STATE = WLAN_HOSTED_NETWORK_STATE(0i32);
pub const wlan_hosted_network_idle: WLAN_HOSTED_NETWORK_STATE = WLAN_HOSTED_NETWORK_STATE(1i32);
pub const wlan_hosted_network_active: WLAN_HOSTED_NETWORK_STATE = WLAN_HOSTED_NETWORK_STATE(2i32);
#[repr(C)]
pub struct WLAN_HOSTED_NETWORK_STATE_CHANGE(i32);
#[repr(C)]
pub struct WLAN_HOSTED_NETWORK_STATUS(i32);
#[repr(transparent)]
pub struct WLAN_IHV_CONTROL_TYPE(pub i32);
pub const wlan_ihv_control_type_service: WLAN_IHV_CONTROL_TYPE = WLAN_IHV_CONTROL_TYPE(0i32);
pub const wlan_ihv_control_type_driver: WLAN_IHV_CONTROL_TYPE = WLAN_IHV_CONTROL_TYPE(1i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLAN_INTERFACE_CAPABILITY(i32);
#[repr(C)]
pub struct WLAN_INTERFACE_INFO(i32);
#[repr(C)]
pub struct WLAN_INTERFACE_INFO_LIST(i32);
#[repr(transparent)]
pub struct WLAN_INTERFACE_STATE(pub i32);
pub const wlan_interface_state_not_ready: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(0i32);
pub const wlan_interface_state_connected: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(1i32);
pub const wlan_interface_state_ad_hoc_network_formed: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(2i32);
pub const wlan_interface_state_disconnecting: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(3i32);
pub const wlan_interface_state_disconnected: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(4i32);
pub const wlan_interface_state_associating: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(5i32);
pub const wlan_interface_state_discovering: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(6i32);
pub const wlan_interface_state_authenticating: WLAN_INTERFACE_STATE = WLAN_INTERFACE_STATE(7i32);
#[repr(transparent)]
pub struct WLAN_INTERFACE_TYPE(pub i32);
pub const wlan_interface_type_emulated_802_11: WLAN_INTERFACE_TYPE = WLAN_INTERFACE_TYPE(0i32);
pub const wlan_interface_type_native_802_11: WLAN_INTERFACE_TYPE = WLAN_INTERFACE_TYPE(1i32);
pub const wlan_interface_type_invalid: WLAN_INTERFACE_TYPE = WLAN_INTERFACE_TYPE(2i32);
#[repr(transparent)]
pub struct WLAN_INTF_OPCODE(pub i32);
pub const wlan_intf_opcode_autoconf_start: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(0i32);
pub const wlan_intf_opcode_autoconf_enabled: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(1i32);
pub const wlan_intf_opcode_background_scan_enabled: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(2i32);
pub const wlan_intf_opcode_media_streaming_mode: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(3i32);
pub const wlan_intf_opcode_radio_state: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(4i32);
pub const wlan_intf_opcode_bss_type: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(5i32);
pub const wlan_intf_opcode_interface_state: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(6i32);
pub const wlan_intf_opcode_current_connection: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(7i32);
pub const wlan_intf_opcode_channel_number: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(8i32);
pub const wlan_intf_opcode_supported_infrastructure_auth_cipher_pairs: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(9i32);
pub const wlan_intf_opcode_supported_adhoc_auth_cipher_pairs: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(10i32);
pub const wlan_intf_opcode_supported_country_or_region_string_list: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(11i32);
pub const wlan_intf_opcode_current_operation_mode: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(12i32);
pub const wlan_intf_opcode_supported_safe_mode: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(13i32);
pub const wlan_intf_opcode_certified_safe_mode: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(14i32);
pub const wlan_intf_opcode_hosted_network_capable: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(15i32);
pub const wlan_intf_opcode_management_frame_protection_capable: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(16i32);
pub const wlan_intf_opcode_secondary_sta_interfaces: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(17i32);
pub const wlan_intf_opcode_secondary_sta_synchronized_connections: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(18i32);
pub const wlan_intf_opcode_autoconf_end: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(268435455i32);
pub const wlan_intf_opcode_msm_start: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(268435712i32);
pub const wlan_intf_opcode_statistics: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(268435713i32);
pub const wlan_intf_opcode_rssi: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(268435714i32);
pub const wlan_intf_opcode_msm_end: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(536870911i32);
pub const wlan_intf_opcode_security_start: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(536936448i32);
pub const wlan_intf_opcode_security_end: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(805306367i32);
pub const wlan_intf_opcode_ihv_start: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(805306368i32);
pub const wlan_intf_opcode_ihv_end: WLAN_INTF_OPCODE = WLAN_INTF_OPCODE(1073741823i32);
#[repr(C)]
pub struct WLAN_MAC_FRAME_STATISTICS(i32);
pub const WLAN_MAX_NAME_LENGTH: u32 = 256u32;
pub const WLAN_MAX_PHY_INDEX: u32 = 64u32;
pub const WLAN_MAX_PHY_TYPE_NUMBER: u32 = 8u32;
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLAN_MSM_NOTIFICATION_DATA(i32);
#[repr(transparent)]
pub struct WLAN_NOTIFICATION_ACM(pub i32);
pub const wlan_notification_acm_start: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(0i32);
pub const wlan_notification_acm_autoconf_enabled: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(1i32);
pub const wlan_notification_acm_autoconf_disabled: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(2i32);
pub const wlan_notification_acm_background_scan_enabled: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(3i32);
pub const wlan_notification_acm_background_scan_disabled: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(4i32);
pub const wlan_notification_acm_bss_type_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(5i32);
pub const wlan_notification_acm_power_setting_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(6i32);
pub const wlan_notification_acm_scan_complete: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(7i32);
pub const wlan_notification_acm_scan_fail: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(8i32);
pub const wlan_notification_acm_connection_start: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(9i32);
pub const wlan_notification_acm_connection_complete: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(10i32);
pub const wlan_notification_acm_connection_attempt_fail: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(11i32);
pub const wlan_notification_acm_filter_list_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(12i32);
pub const wlan_notification_acm_interface_arrival: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(13i32);
pub const wlan_notification_acm_interface_removal: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(14i32);
pub const wlan_notification_acm_profile_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(15i32);
pub const wlan_notification_acm_profile_name_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(16i32);
pub const wlan_notification_acm_profiles_exhausted: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(17i32);
pub const wlan_notification_acm_network_not_available: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(18i32);
pub const wlan_notification_acm_network_available: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(19i32);
pub const wlan_notification_acm_disconnecting: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(20i32);
pub const wlan_notification_acm_disconnected: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(21i32);
pub const wlan_notification_acm_adhoc_network_state_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(22i32);
pub const wlan_notification_acm_profile_unblocked: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(23i32);
pub const wlan_notification_acm_screen_power_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(24i32);
pub const wlan_notification_acm_profile_blocked: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(25i32);
pub const wlan_notification_acm_scan_list_refresh: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(26i32);
pub const wlan_notification_acm_operational_state_change: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(27i32);
pub const wlan_notification_acm_end: WLAN_NOTIFICATION_ACM = WLAN_NOTIFICATION_ACM(28i32);
pub type WLAN_NOTIFICATION_CALLBACK = unsafe extern "system" fn(param0: *mut L2_NOTIFICATION_DATA, param1: *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct WLAN_NOTIFICATION_MSM(pub i32);
pub const wlan_notification_msm_start: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(0i32);
pub const wlan_notification_msm_associating: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(1i32);
pub const wlan_notification_msm_associated: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(2i32);
pub const wlan_notification_msm_authenticating: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(3i32);
pub const wlan_notification_msm_connected: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(4i32);
pub const wlan_notification_msm_roaming_start: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(5i32);
pub const wlan_notification_msm_roaming_end: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(6i32);
pub const wlan_notification_msm_radio_state_change: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(7i32);
pub const wlan_notification_msm_signal_quality_change: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(8i32);
pub const wlan_notification_msm_disassociating: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(9i32);
pub const wlan_notification_msm_disconnected: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(10i32);
pub const wlan_notification_msm_peer_join: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(11i32);
pub const wlan_notification_msm_peer_leave: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(12i32);
pub const wlan_notification_msm_adapter_removal: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(13i32);
pub const wlan_notification_msm_adapter_operation_mode_change: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(14i32);
pub const wlan_notification_msm_link_degraded: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(15i32);
pub const wlan_notification_msm_link_improved: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(16i32);
pub const wlan_notification_msm_end: WLAN_NOTIFICATION_MSM = WLAN_NOTIFICATION_MSM(17i32);
#[repr(transparent)]
pub struct WLAN_NOTIFICATION_SECURITY(pub i32);
pub const wlan_notification_security_start: WLAN_NOTIFICATION_SECURITY = WLAN_NOTIFICATION_SECURITY(0i32);
pub const wlan_notification_security_end: WLAN_NOTIFICATION_SECURITY = WLAN_NOTIFICATION_SECURITY(1i32);
pub const WLAN_NOTIFICATION_SOURCE_ACM: u32 = 8u32;
pub const WLAN_NOTIFICATION_SOURCE_ALL: u32 = 65535u32;
pub const WLAN_NOTIFICATION_SOURCE_DEVICE_SERVICE: u32 = 2048u32;
pub const WLAN_NOTIFICATION_SOURCE_HNWK: u32 = 128u32;
pub const WLAN_NOTIFICATION_SOURCE_IHV: u32 = 64u32;
pub const WLAN_NOTIFICATION_SOURCE_MSM: u32 = 16u32;
pub const WLAN_NOTIFICATION_SOURCE_NONE: u32 = 0u32;
pub const WLAN_NOTIFICATION_SOURCE_ONEX: u32 = 4u32;
pub const WLAN_NOTIFICATION_SOURCE_SECURITY: u32 = 32u32;
#[repr(transparent)]
pub struct WLAN_OPCODE_VALUE_TYPE(pub i32);
pub const wlan_opcode_value_type_query_only: WLAN_OPCODE_VALUE_TYPE = WLAN_OPCODE_VALUE_TYPE(0i32);
pub const wlan_opcode_value_type_set_by_group_policy: WLAN_OPCODE_VALUE_TYPE = WLAN_OPCODE_VALUE_TYPE(1i32);
pub const wlan_opcode_value_type_set_by_user: WLAN_OPCODE_VALUE_TYPE = WLAN_OPCODE_VALUE_TYPE(2i32);
pub const wlan_opcode_value_type_invalid: WLAN_OPCODE_VALUE_TYPE = WLAN_OPCODE_VALUE_TYPE(3i32);
#[repr(transparent)]
pub struct WLAN_OPERATIONAL_STATE(pub i32);
pub const wlan_operational_state_unknown: WLAN_OPERATIONAL_STATE = WLAN_OPERATIONAL_STATE(0i32);
pub const wlan_operational_state_off: WLAN_OPERATIONAL_STATE = WLAN_OPERATIONAL_STATE(1i32);
pub const wlan_operational_state_on: WLAN_OPERATIONAL_STATE = WLAN_OPERATIONAL_STATE(2i32);
pub const wlan_operational_state_going_off: WLAN_OPERATIONAL_STATE = WLAN_OPERATIONAL_STATE(3i32);
pub const wlan_operational_state_going_on: WLAN_OPERATIONAL_STATE = WLAN_OPERATIONAL_STATE(4i32);
#[repr(C)]
pub struct WLAN_PHY_FRAME_STATISTICS(i32);
#[repr(C)]
pub struct WLAN_PHY_RADIO_STATE(i32);
#[repr(transparent)]
pub struct WLAN_POWER_SETTING(pub i32);
pub const wlan_power_setting_no_saving: WLAN_POWER_SETTING = WLAN_POWER_SETTING(0i32);
pub const wlan_power_setting_low_saving: WLAN_POWER_SETTING = WLAN_POWER_SETTING(1i32);
pub const wlan_power_setting_medium_saving: WLAN_POWER_SETTING = WLAN_POWER_SETTING(2i32);
pub const wlan_power_setting_maximum_saving: WLAN_POWER_SETTING = WLAN_POWER_SETTING(3i32);
pub const wlan_power_setting_invalid: WLAN_POWER_SETTING = WLAN_POWER_SETTING(4i32);
pub const WLAN_PROFILE_CONNECTION_MODE_AUTO: u32 = 131072u32;
pub const WLAN_PROFILE_CONNECTION_MODE_SET_BY_CLIENT: u32 = 65536u32;
pub const WLAN_PROFILE_GET_PLAINTEXT_KEY: u32 = 4u32;
pub const WLAN_PROFILE_GROUP_POLICY: u32 = 1u32;
#[repr(C)]
pub struct WLAN_PROFILE_INFO(i32);
#[repr(C)]
pub struct WLAN_PROFILE_INFO_LIST(i32);
pub const WLAN_PROFILE_USER: u32 = 2u32;
#[repr(C)]
pub struct WLAN_RADIO_STATE(i32);
#[repr(C)]
pub struct WLAN_RATE_SET(i32);
#[repr(C)]
pub struct WLAN_RAW_DATA(i32);
#[repr(C)]
pub struct WLAN_RAW_DATA_LIST(i32);
pub const WLAN_REASON_CODE_AC_BASE: u32 = 131072u32;
pub const WLAN_REASON_CODE_AC_CONNECT_BASE: u32 = 163840u32;
pub const WLAN_REASON_CODE_AC_END: u32 = 196607u32;
pub const WLAN_REASON_CODE_ADHOC_SECURITY_FAILURE: u32 = 229386u32;
pub const WLAN_REASON_CODE_AP_PROFILE_NOT_ALLOWED: u32 = 163856u32;
pub const WLAN_REASON_CODE_AP_PROFILE_NOT_ALLOWED_FOR_CLIENT: u32 = 163855u32;
pub const WLAN_REASON_CODE_AP_STARTING_FAILURE: u32 = 229395u32;
pub const WLAN_REASON_CODE_ASSOCIATION_FAILURE: u32 = 229378u32;
pub const WLAN_REASON_CODE_ASSOCIATION_TIMEOUT: u32 = 229379u32;
pub const WLAN_REASON_CODE_AUTO_AP_PROFILE_NOT_ALLOWED: u32 = 524313u32;
pub const WLAN_REASON_CODE_AUTO_CONNECTION_NOT_ALLOWED: u32 = 524314u32;
pub const WLAN_REASON_CODE_AUTO_SWITCH_SET_FOR_ADHOC: u32 = 524304u32;
pub const WLAN_REASON_CODE_AUTO_SWITCH_SET_FOR_MANUAL_CONNECTION: u32 = 524305u32;
pub const WLAN_REASON_CODE_BAD_MAX_NUMBER_OF_CLIENTS_FOR_AP: u32 = 524310u32;
pub const WLAN_REASON_CODE_BASE: u32 = 131072u32;
pub const WLAN_REASON_CODE_BSS_TYPE_NOT_ALLOWED: u32 = 163845u32;
pub const WLAN_REASON_CODE_BSS_TYPE_UNMATCH: u32 = 196611u32;
pub const WLAN_REASON_CODE_CONFLICT_SECURITY: u32 = 524299u32;
pub const WLAN_REASON_CODE_CONNECT_CALL_FAIL: u32 = 163849u32;
pub const WLAN_REASON_CODE_DATARATE_UNMATCH: u32 = 196613u32;
pub const WLAN_REASON_CODE_DISCONNECT_TIMEOUT: u32 = 229391u32;
pub const WLAN_REASON_CODE_DRIVER_DISCONNECTED: u32 = 229387u32;
pub const WLAN_REASON_CODE_DRIVER_OPERATION_FAILURE: u32 = 229388u32;
pub const WLAN_REASON_CODE_GP_DENIED: u32 = 163843u32;
pub const WLAN_REASON_CODE_HOTSPOT2_PROFILE_DENIED: u32 = 163857u32;
pub const WLAN_REASON_CODE_HOTSPOT2_PROFILE_NOT_ALLOWED: u32 = 524315u32;
pub const WLAN_REASON_CODE_IHV_CONNECTIVITY_NOT_SUPPORTED: u32 = 524309u32;
pub const WLAN_REASON_CODE_IHV_NOT_AVAILABLE: u32 = 229389u32;
pub const WLAN_REASON_CODE_IHV_NOT_RESPONDING: u32 = 229390u32;
pub const WLAN_REASON_CODE_IHV_OUI_MISMATCH: u32 = 524296u32;
pub const WLAN_REASON_CODE_IHV_OUI_MISSING: u32 = 524297u32;
pub const WLAN_REASON_CODE_IHV_SECURITY_NOT_SUPPORTED: u32 = 524295u32;
pub const WLAN_REASON_CODE_IHV_SECURITY_ONEX_MISSING: u32 = 524306u32;
pub const WLAN_REASON_CODE_IHV_SETTINGS_MISSING: u32 = 524298u32;
pub const WLAN_REASON_CODE_INTERNAL_FAILURE: u32 = 229392u32;
pub const WLAN_REASON_CODE_INVALID_ADHOC_CONNECTION_MODE: u32 = 524302u32;
pub const WLAN_REASON_CODE_INVALID_BSS_TYPE: u32 = 524301u32;
pub const WLAN_REASON_CODE_INVALID_CHANNEL: u32 = 524311u32;
pub const WLAN_REASON_CODE_INVALID_PHY_TYPE: u32 = 524293u32;
pub const WLAN_REASON_CODE_INVALID_PROFILE_NAME: u32 = 524291u32;
pub const WLAN_REASON_CODE_INVALID_PROFILE_SCHEMA: u32 = 524289u32;
pub const WLAN_REASON_CODE_INVALID_PROFILE_TYPE: u32 = 524292u32;
pub const WLAN_REASON_CODE_IN_BLOCKED_LIST: u32 = 163847u32;
pub const WLAN_REASON_CODE_IN_FAILED_LIST: u32 = 163846u32;
pub const WLAN_REASON_CODE_KEY_MISMATCH: u32 = 163853u32;
pub const WLAN_REASON_CODE_MSMSEC_AUTH_START_TIMEOUT: u32 = 294914u32;
pub const WLAN_REASON_CODE_MSMSEC_AUTH_SUCCESS_TIMEOUT: u32 = 294915u32;
pub const WLAN_REASON_CODE_MSMSEC_AUTH_WCN_COMPLETED: u32 = 294937u32;
pub const WLAN_REASON_CODE_MSMSEC_BASE: u32 = 262144u32;
pub const WLAN_REASON_CODE_MSMSEC_CANCELLED: u32 = 294929u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_DISCOVERY: u32 = 262165u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_MFP_NW_NIC: u32 = 262181u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_NETWORK: u32 = 262162u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_NIC: u32 = 262163u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE: u32 = 262164u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_AUTH: u32 = 262174u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_CIPHER: u32 = 262175u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_SAFE_MODE_NIC: u32 = 262177u32;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_SAFE_MODE_NW: u32 = 262178u32;
pub const WLAN_REASON_CODE_MSMSEC_CONNECT_BASE: u32 = 294912u32;
pub const WLAN_REASON_CODE_MSMSEC_DOWNGRADE_DETECTED: u32 = 294931u32;
pub const WLAN_REASON_CODE_MSMSEC_END: u32 = 327679u32;
pub const WLAN_REASON_CODE_MSMSEC_FORCED_FAILURE: u32 = 294933u32;
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_GRP_KEY: u32 = 294925u32;
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_KEY_DATA: u32 = 294924u32;
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_MGMT_GRP_KEY: u32 = 294939u32;
pub const WLAN_REASON_CODE_MSMSEC_KEY_FORMAT: u32 = 294930u32;
pub const WLAN_REASON_CODE_MSMSEC_KEY_START_TIMEOUT: u32 = 294916u32;
pub const WLAN_REASON_CODE_MSMSEC_KEY_SUCCESS_TIMEOUT: u32 = 294917u32;
pub const WLAN_REASON_CODE_MSMSEC_M2_MISSING_IE: u32 = 294936u32;
pub const WLAN_REASON_CODE_MSMSEC_M2_MISSING_KEY_DATA: u32 = 294935u32;
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_GRP_KEY: u32 = 294920u32;
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_IE: u32 = 294919u32;
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_KEY_DATA: u32 = 294918u32;
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_MGMT_GRP_KEY: u32 = 294938u32;
pub const WLAN_REASON_CODE_MSMSEC_M3_TOO_MANY_RSNIE: u32 = 294934u32;
pub const WLAN_REASON_CODE_MSMSEC_MAX: u32 = 327679u32;
pub const WLAN_REASON_CODE_MSMSEC_MIN: u32 = 262144u32;
pub const WLAN_REASON_CODE_MSMSEC_MIXED_CELL: u32 = 262169u32;
pub const WLAN_REASON_CODE_MSMSEC_NIC_FAILURE: u32 = 294928u32;
pub const WLAN_REASON_CODE_MSMSEC_NO_AUTHENTICATOR: u32 = 294927u32;
pub const WLAN_REASON_CODE_MSMSEC_NO_PAIRWISE_KEY: u32 = 294923u32;
pub const WLAN_REASON_CODE_MSMSEC_PEER_INDICATED_INSECURE: u32 = 294926u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_AUTH_TIMERS_INVALID: u32 = 262170u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_DUPLICATE_AUTH_CIPHER: u32 = 262151u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_AUTH_CIPHER: u32 = 262153u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_GKEY_INTV: u32 = 262171u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_KEY_INDEX: u32 = 262145u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_MODE: u32 = 262156u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_SIZE: u32 = 262157u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_TTL: u32 = 262158u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PREAUTH_MODE: u32 = 262159u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PREAUTH_THROTTLE: u32 = 262160u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEYMATERIAL_CHAR: u32 = 262167u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEY_LENGTH: u32 = 262147u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEY_UNMAPPED_CHAR: u32 = 262173u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_NO_AUTH_CIPHER_SPECIFIED: u32 = 262149u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_ONEX_DISABLED: u32 = 262154u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_ONEX_ENABLED: u32 = 262155u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PASSPHRASE_CHAR: u32 = 262166u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PREAUTH_ONLY_ENABLED: u32 = 262161u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PSK_LENGTH: u32 = 262148u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PSK_PRESENT: u32 = 262146u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_RAWDATA_INVALID: u32 = 262152u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_SAFE_MODE: u32 = 262176u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_TOO_MANY_AUTH_CIPHER_SPECIFIED: u32 = 262150u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_UNSUPPORTED_AUTH: u32 = 262179u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_UNSUPPORTED_CIPHER: u32 = 262180u32;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_WRONG_KEYTYPE: u32 = 262168u32;
pub const WLAN_REASON_CODE_MSMSEC_PR_IE_MATCHING: u32 = 294921u32;
pub const WLAN_REASON_CODE_MSMSEC_PSK_MISMATCH_SUSPECTED: u32 = 294932u32;
pub const WLAN_REASON_CODE_MSMSEC_SEC_IE_MATCHING: u32 = 294922u32;
pub const WLAN_REASON_CODE_MSMSEC_TRANSITION_NETWORK: u32 = 262172u32;
pub const WLAN_REASON_CODE_MSMSEC_UI_REQUEST_FAILURE: u32 = 294913u32;
pub const WLAN_REASON_CODE_MSM_BASE: u32 = 196608u32;
pub const WLAN_REASON_CODE_MSM_CONNECT_BASE: u32 = 229376u32;
pub const WLAN_REASON_CODE_MSM_END: u32 = 262143u32;
pub const WLAN_REASON_CODE_MSM_SECURITY_MISSING: u32 = 524294u32;
pub const WLAN_REASON_CODE_NETWORK_NOT_AVAILABLE: u32 = 163851u32;
pub const WLAN_REASON_CODE_NETWORK_NOT_COMPATIBLE: u32 = 131073u32;
pub const WLAN_REASON_CODE_NON_BROADCAST_SET_FOR_ADHOC: u32 = 524303u32;
pub const WLAN_REASON_CODE_NOT_VISIBLE: u32 = 163842u32;
pub const WLAN_REASON_CODE_NO_AUTO_CONNECTION: u32 = 163841u32;
pub const WLAN_REASON_CODE_NO_VISIBLE_AP: u32 = 229396u32;
pub const WLAN_REASON_CODE_OPERATION_MODE_NOT_SUPPORTED: u32 = 524312u32;
pub const WLAN_REASON_CODE_PHY_TYPE_UNMATCH: u32 = 196612u32;
pub const WLAN_REASON_CODE_PRE_SECURITY_FAILURE: u32 = 229380u32;
pub const WLAN_REASON_CODE_PROFILE_BASE: u32 = 524288u32;
pub const WLAN_REASON_CODE_PROFILE_CHANGED_OR_DELETED: u32 = 163852u32;
pub const WLAN_REASON_CODE_PROFILE_CONNECT_BASE: u32 = 557056u32;
pub const WLAN_REASON_CODE_PROFILE_END: u32 = 589823u32;
pub const WLAN_REASON_CODE_PROFILE_MISSING: u32 = 524290u32;
pub const WLAN_REASON_CODE_PROFILE_NOT_COMPATIBLE: u32 = 131074u32;
pub const WLAN_REASON_CODE_PROFILE_SSID_INVALID: u32 = 524307u32;
pub const WLAN_REASON_CODE_RANGE_SIZE: u32 = 65536u32;
pub const WLAN_REASON_CODE_RESERVED_BASE: u32 = 720896u32;
pub const WLAN_REASON_CODE_RESERVED_END: u32 = 786431u32;
pub const WLAN_REASON_CODE_ROAMING_FAILURE: u32 = 229384u32;
pub const WLAN_REASON_CODE_ROAMING_SECURITY_FAILURE: u32 = 229385u32;
pub const WLAN_REASON_CODE_SCAN_CALL_FAIL: u32 = 163850u32;
pub const WLAN_REASON_CODE_SECURITY_FAILURE: u32 = 229382u32;
pub const WLAN_REASON_CODE_SECURITY_MISSING: u32 = 524300u32;
pub const WLAN_REASON_CODE_SECURITY_TIMEOUT: u32 = 229383u32;
pub const WLAN_REASON_CODE_SSID_LIST_TOO_LONG: u32 = 163848u32;
pub const WLAN_REASON_CODE_START_SECURITY_FAILURE: u32 = 229381u32;
pub const WLAN_REASON_CODE_SUCCESS: u32 = 0u32;
pub const WLAN_REASON_CODE_TOO_MANY_SECURITY_ATTEMPTS: u32 = 229394u32;
pub const WLAN_REASON_CODE_TOO_MANY_SSID: u32 = 524308u32;
pub const WLAN_REASON_CODE_UI_REQUEST_TIMEOUT: u32 = 229393u32;
pub const WLAN_REASON_CODE_UNKNOWN: u32 = 65537u32;
pub const WLAN_REASON_CODE_UNSUPPORTED_SECURITY_SET: u32 = 196610u32;
pub const WLAN_REASON_CODE_UNSUPPORTED_SECURITY_SET_BY_OS: u32 = 196609u32;
pub const WLAN_REASON_CODE_USER_CANCELLED: u32 = 229377u32;
pub const WLAN_REASON_CODE_USER_DENIED: u32 = 163844u32;
pub const WLAN_REASON_CODE_USER_NOT_RESPOND: u32 = 163854u32;
#[repr(transparent)]
pub struct WLAN_SECURABLE_OBJECT(pub i32);
pub const wlan_secure_permit_list: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(0i32);
pub const wlan_secure_deny_list: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(1i32);
pub const wlan_secure_ac_enabled: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(2i32);
pub const wlan_secure_bc_scan_enabled: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(3i32);
pub const wlan_secure_bss_type: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(4i32);
pub const wlan_secure_show_denied: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(5i32);
pub const wlan_secure_interface_properties: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(6i32);
pub const wlan_secure_ihv_control: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(7i32);
pub const wlan_secure_all_user_profiles_order: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(8i32);
pub const wlan_secure_add_new_all_user_profiles: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(9i32);
pub const wlan_secure_add_new_per_user_profiles: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(10i32);
pub const wlan_secure_media_streaming_mode_enabled: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(11i32);
pub const wlan_secure_current_operation_mode: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(12i32);
pub const wlan_secure_get_plaintext_key: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(13i32);
pub const wlan_secure_hosted_network_elevated_access: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(14i32);
pub const wlan_secure_virtual_station_extensibility: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(15i32);
pub const wlan_secure_wfd_elevated_access: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(16i32);
pub const WLAN_SECURABLE_OBJECT_COUNT: WLAN_SECURABLE_OBJECT = WLAN_SECURABLE_OBJECT(17i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct WLAN_SECURITY_ATTRIBUTES(i32);
#[repr(transparent)]
pub struct WLAN_SET_EAPHOST_FLAGS(pub u32);
pub const WLAN_SET_EAPHOST_DATA_ALL_USERS: WLAN_SET_EAPHOST_FLAGS = WLAN_SET_EAPHOST_FLAGS(1u32);
#[repr(C)]
pub struct WLAN_STATISTICS(i32);
pub const WLAN_UI_API_INITIAL_VERSION: u32 = 1u32;
pub const WLAN_UI_API_VERSION: u32 = 1u32;
#[repr(transparent)]
pub struct WL_DISPLAY_PAGES(pub i32);
pub const WLConnectionPage: WL_DISPLAY_PAGES = WL_DISPLAY_PAGES(0i32);
pub const WLSecurityPage: WL_DISPLAY_PAGES = WL_DISPLAY_PAGES(1i32);
pub const WLAdvPage: WL_DISPLAY_PAGES = WL_DISPLAY_PAGES(2i32);
#[repr(C)]
pub struct _DOT11_WME_AC_PARAMTERS_LIST(i32);
