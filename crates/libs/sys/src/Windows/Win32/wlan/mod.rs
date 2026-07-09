#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WFDCancelOpenSession(hsessionhandle : super::winnt::HANDLE) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WFDCloseHandle(hclienthandle : super::winnt::HANDLE) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WFDCloseSession(hsessionhandle : super::winnt::HANDLE) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WFDOpenHandle(dwclientversion : u32, pdwnegotiatedversion : *mut u32, phclienthandle : *mut super::winnt::HANDLE) -> u32);
#[cfg(all(feature = "Win32_windot11", feature = "Win32_winnt"))]
windows_link::link!("wlanapi.dll" "system" fn WFDOpenLegacySession(hclienthandle : super::winnt::HANDLE, plegacymacaddress : *const super::windot11::DOT11_MAC_ADDRESS, phsessionhandle : *mut super::winnt::HANDLE, pguidsessioninterface : *mut windows_sys::core::GUID) -> u32);
#[cfg(all(feature = "Win32_windot11", feature = "Win32_winnt"))]
windows_link::link!("wlanapi.dll" "system" fn WFDStartOpenSession(hclienthandle : super::winnt::HANDLE, pdeviceaddress : *const super::windot11::DOT11_MAC_ADDRESS, pvcontext : *const core::ffi::c_void, pfncallback : WFD_OPEN_SESSION_COMPLETE_CALLBACK, phsessionhandle : *mut super::winnt::HANDLE) -> u32);
#[cfg(feature = "Win32_windot11")]
windows_link::link!("wlanapi.dll" "system" fn WFDUpdateDeviceVisibility(pdeviceaddress : *const super::windot11::DOT11_MAC_ADDRESS) -> u32);
windows_link::link!("wlanapi.dll" "system" fn WlanAllocateMemory(dwmemorysize : u32) -> *mut core::ffi::c_void);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanCloseHandle(hclienthandle : super::winnt::HANDLE, preserved : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_windot11", feature = "Win32_winnt"))]
windows_link::link!("wlanapi.dll" "system" fn WlanConnect(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, pconnectionparameters : *const WLAN_CONNECTION_PARAMETERS, preserved : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_windot11", feature = "Win32_winnt"))]
windows_link::link!("wlanapi.dll" "system" fn WlanConnect2(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, pconnectionparameters : *const WLAN_CONNECTION_PARAMETERS_V2, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanDeleteProfile(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, strprofilename : windows_sys::core::PCWSTR, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanDeviceServiceCommand(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, pdeviceserviceguid : *const windows_sys::core::GUID, dwopcode : u32, dwinbuffersize : u32, pinbuffer : *const core::ffi::c_void, dwoutbuffersize : u32, poutbuffer : *mut core::ffi::c_void, pdwbytesreturned : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanDisconnect(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanEnumInterfaces(hclienthandle : super::winnt::HANDLE, preserved : *const core::ffi::c_void, ppinterfacelist : *mut PWLAN_INTERFACE_INFO_LIST) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanExtractPsdIEDataList(hclienthandle : super::winnt::HANDLE, dwiedatasize : u32, prawiedata : *const u8, strformat : windows_sys::core::PCWSTR, preserved : *const core::ffi::c_void, pppsdiedatalist : *mut PWLAN_RAW_DATA_LIST) -> u32);
windows_link::link!("wlanapi.dll" "system" fn WlanFreeMemory(pmemory : *const core::ffi::c_void));
#[cfg(all(feature = "Win32_windot11", feature = "Win32_winnt"))]
windows_link::link!("wlanapi.dll" "system" fn WlanGetAvailableNetworkList(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, dwflags : u32, preserved : *const core::ffi::c_void, ppavailablenetworklist : *mut PWLAN_AVAILABLE_NETWORK_LIST) -> u32);
#[cfg(all(feature = "Win32_windot11", feature = "Win32_winnt"))]
windows_link::link!("wlanapi.dll" "system" fn WlanGetAvailableNetworkList2(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, dwflags : u32, preserved : *const core::ffi::c_void, ppavailablenetworklist : *mut PWLAN_AVAILABLE_NETWORK_LIST_V2) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanGetFilterList(hclienthandle : super::winnt::HANDLE, wlanfilterlisttype : WLAN_FILTER_LIST_TYPE, preserved : *const core::ffi::c_void, ppnetworklist : *mut PDOT11_NETWORK_LIST) -> u32);
#[cfg(all(feature = "Win32_windot11", feature = "Win32_winnt"))]
windows_link::link!("wlanapi.dll" "system" fn WlanGetInterfaceCapability(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, preserved : *const core::ffi::c_void, ppcapability : *mut PWLAN_INTERFACE_CAPABILITY) -> u32);
#[cfg(all(feature = "Win32_windot11", feature = "Win32_winnt"))]
windows_link::link!("wlanapi.dll" "system" fn WlanGetNetworkBssList(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, pdot11ssid : *const DOT11_SSID, dot11bsstype : DOT11_BSS_TYPE, bsecurityenabled : windows_sys::core::BOOL, preserved : *const core::ffi::c_void, ppwlanbsslist : *mut PWLAN_BSS_LIST) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanGetProfile(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, strprofilename : windows_sys::core::PCWSTR, preserved : *const core::ffi::c_void, pstrprofilexml : *mut windows_sys::core::PWSTR, pdwflags : *mut u32, pdwgrantedaccess : *mut u32) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("wlanapi.dll" "system" fn WlanGetProfileCustomUserData(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, strprofilename : windows_sys::core::PCWSTR, preserved : *const core::ffi::c_void, pdwdatasize : *mut u32, ppdata : *mut super::minwindef::PBYTE) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanGetProfileList(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, preserved : *const core::ffi::c_void, ppprofilelist : *mut PWLAN_PROFILE_INFO_LIST) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanGetSecuritySettings(hclienthandle : super::winnt::HANDLE, securableobject : WLAN_SECURABLE_OBJECT, pvaluetype : *mut WLAN_OPCODE_VALUE_TYPE, pstrcurrentsddl : *mut windows_sys::core::PWSTR, pdwgrantedaccess : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanGetSupportedDeviceServices(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, ppdevsvcguidlist : *mut PWLAN_DEVICE_SERVICE_GUID_LIST) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanHostedNetworkForceStart(hclienthandle : super::winnt::HANDLE, pfailreason : *mut WLAN_HOSTED_NETWORK_REASON, pvreserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanHostedNetworkForceStop(hclienthandle : super::winnt::HANDLE, pfailreason : *mut WLAN_HOSTED_NETWORK_REASON, pvreserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanHostedNetworkInitSettings(hclienthandle : super::winnt::HANDLE, pfailreason : *mut WLAN_HOSTED_NETWORK_REASON, pvreserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanHostedNetworkQueryProperty(hclienthandle : super::winnt::HANDLE, opcode : WLAN_HOSTED_NETWORK_OPCODE, pdwdatasize : *mut u32, ppvdata : *mut *mut core::ffi::c_void, pwlanopcodevaluetype : *mut WLAN_OPCODE_VALUE_TYPE, pvreserved : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_winnt"))]
windows_link::link!("wlanapi.dll" "system" fn WlanHostedNetworkQuerySecondaryKey(hclienthandle : super::winnt::HANDLE, pdwkeylength : *mut u32, ppuckeydata : *mut super::minwindef::PUCHAR, pbispassphrase : *mut windows_sys::core::BOOL, pbpersistent : *mut windows_sys::core::BOOL, pfailreason : *mut WLAN_HOSTED_NETWORK_REASON, pvreserved : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "Win32_windot11", feature = "Win32_winnt"))]
windows_link::link!("wlanapi.dll" "system" fn WlanHostedNetworkQueryStatus(hclienthandle : super::winnt::HANDLE, ppwlanhostednetworkstatus : *mut PWLAN_HOSTED_NETWORK_STATUS, pvreserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanHostedNetworkRefreshSecuritySettings(hclienthandle : super::winnt::HANDLE, pfailreason : *mut WLAN_HOSTED_NETWORK_REASON, pvreserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanHostedNetworkSetProperty(hclienthandle : super::winnt::HANDLE, opcode : WLAN_HOSTED_NETWORK_OPCODE, dwdatasize : u32, pvdata : *const core::ffi::c_void, pfailreason : *mut WLAN_HOSTED_NETWORK_REASON, pvreserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanHostedNetworkSetSecondaryKey(hclienthandle : super::winnt::HANDLE, dwkeylength : u32, puckeydata : *const u8, bispassphrase : windows_sys::core::BOOL, bpersistent : windows_sys::core::BOOL, pfailreason : *mut WLAN_HOSTED_NETWORK_REASON, pvreserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanHostedNetworkStartUsing(hclienthandle : super::winnt::HANDLE, pfailreason : *mut WLAN_HOSTED_NETWORK_REASON, pvreserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanHostedNetworkStopUsing(hclienthandle : super::winnt::HANDLE, pfailreason : *mut WLAN_HOSTED_NETWORK_REASON, pvreserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanIhvControl(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, r#type : WLAN_IHV_CONTROL_TYPE, dwinbuffersize : u32, pinbuffer : *const core::ffi::c_void, dwoutbuffersize : u32, poutbuffer : *mut core::ffi::c_void, pdwbytesreturned : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanOpenHandle(dwclientversion : u32, preserved : *const core::ffi::c_void, pdwnegotiatedversion : *mut u32, phclienthandle : *mut super::winnt::HANDLE) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanQueryAutoConfigParameter(hclienthandle : super::winnt::HANDLE, opcode : WLAN_AUTOCONF_OPCODE, preserved : *const core::ffi::c_void, pdwdatasize : *mut u32, ppdata : *mut *mut core::ffi::c_void, pwlanopcodevaluetype : *mut WLAN_OPCODE_VALUE_TYPE) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanQueryInterface(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, opcode : WLAN_INTF_OPCODE, preserved : *const core::ffi::c_void, pdwdatasize : *mut u32, ppdata : *mut *mut core::ffi::c_void, pwlanopcodevaluetype : *mut WLAN_OPCODE_VALUE_TYPE) -> u32);
windows_link::link!("wlanapi.dll" "system" fn WlanReasonCodeToString(dwreasoncode : u32, dwbuffersize : u32, pstringbuffer : *const u16, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanRegisterDeviceServiceNotification(hclienthandle : super::winnt::HANDLE, pdevsvcguidlist : *const WLAN_DEVICE_SERVICE_GUID_LIST) -> u32);
#[cfg(all(feature = "Win32_l2cmn", feature = "Win32_winnt"))]
windows_link::link!("wlanapi.dll" "system" fn WlanRegisterNotification(hclienthandle : super::winnt::HANDLE, dwnotifsource : u32, bignoreduplicate : windows_sys::core::BOOL, funccallback : WLAN_NOTIFICATION_CALLBACK, pcallbackcontext : *const core::ffi::c_void, preserved : *const core::ffi::c_void, pdwprevnotifsource : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanRegisterVirtualStationNotification(hclienthandle : super::winnt::HANDLE, bregister : windows_sys::core::BOOL, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanRenameProfile(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, stroldprofilename : windows_sys::core::PCWSTR, strnewprofilename : windows_sys::core::PCWSTR, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanSaveTemporaryProfile(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, strprofilename : windows_sys::core::PCWSTR, stralluserprofilesecurity : windows_sys::core::PCWSTR, dwflags : u32, boverwrite : windows_sys::core::BOOL, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanScan(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, pdot11ssid : *const DOT11_SSID, piedata : *const WLAN_RAW_DATA, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanSetAutoConfigParameter(hclienthandle : super::winnt::HANDLE, opcode : WLAN_AUTOCONF_OPCODE, dwdatasize : u32, pdata : *const core::ffi::c_void, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanSetFilterList(hclienthandle : super::winnt::HANDLE, wlanfilterlisttype : WLAN_FILTER_LIST_TYPE, pnetworklist : *const DOT11_NETWORK_LIST, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanSetInterface(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, opcode : WLAN_INTF_OPCODE, dwdatasize : u32, pdata : *const core::ffi::c_void, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanSetProfile(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, dwflags : u32, strprofilexml : windows_sys::core::PCWSTR, stralluserprofilesecurity : windows_sys::core::PCWSTR, boverwrite : windows_sys::core::BOOL, preserved : *const core::ffi::c_void, pdwreasoncode : *mut u32) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanSetProfileCustomUserData(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, strprofilename : windows_sys::core::PCWSTR, dwdatasize : u32, pdata : *const u8, preserved : *const core::ffi::c_void) -> u32);
#[cfg(all(feature = "Win32_eaptypes", feature = "Win32_winnt"))]
windows_link::link!("wlanapi.dll" "system" fn WlanSetProfileEapUserData(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, strprofilename : windows_sys::core::PCWSTR, eaptype : super::eaptypes::EAP_METHOD_TYPE, dwflags : u32, dweapuserdatasize : u32, pbeapuserdata : *const u8, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanSetProfileEapXmlUserData(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, strprofilename : windows_sys::core::PCWSTR, dwflags : u32, streapxmluserdata : windows_sys::core::PCWSTR, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanSetProfileList(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, dwitems : u32, strprofilenames : *const windows_sys::core::PCWSTR, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanSetProfilePosition(hclienthandle : super::winnt::HANDLE, pinterfaceguid : *const windows_sys::core::GUID, strprofilename : windows_sys::core::PCWSTR, dwposition : u32, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanSetPsdIEDataList(hclienthandle : super::winnt::HANDLE, strformat : windows_sys::core::PCWSTR, ppsdiedatalist : *const WLAN_RAW_DATA_LIST, preserved : *const core::ffi::c_void) -> u32);
#[cfg(feature = "Win32_winnt")]
windows_link::link!("wlanapi.dll" "system" fn WlanSetSecuritySettings(hclienthandle : super::winnt::HANDLE, securableobject : WLAN_SECURABLE_OBJECT, strmodifiedsddl : windows_sys::core::PCWSTR) -> u32);
#[cfg(feature = "Win32_windef")]
windows_link::link!("wlanui.dll" "system" fn WlanUIEditProfile(dwclientversion : u32, wstrprofilename : windows_sys::core::PCWSTR, pinterfaceguid : *const windows_sys::core::GUID, hwnd : super::windef::HWND, wlstartpage : WL_DISPLAY_PAGES, preserved : *const core::ffi::c_void, pwlanreasoncode : *mut u32) -> u32);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DOT11_ACCESSNETWORKOPTIONS {
    pub AccessNetworkType: u8,
    pub Internet: u8,
    pub ASRA: u8,
    pub ESR: u8,
    pub UESA: u8,
}
pub type DOT11_AUTH_ALGORITHM = i32;
pub const DOT11_AUTH_ALGORITHM_OPEN_SYSTEM: u32 = 1;
pub const DOT11_AUTH_ALGORITHM_OWE: u32 = 10;
pub const DOT11_AUTH_ALGORITHM_RSNA: u32 = 6;
pub const DOT11_AUTH_ALGORITHM_RSNA_PSK: u32 = 7;
pub const DOT11_AUTH_ALGORITHM_SHARED_KEY: u32 = 2;
pub const DOT11_AUTH_ALGORITHM_WPA: u32 = 3;
pub const DOT11_AUTH_ALGORITHM_WPA3: u32 = 8;
pub const DOT11_AUTH_ALGORITHM_WPA3_ENT: u32 = 11;
pub const DOT11_AUTH_ALGORITHM_WPA3_ENT_192: u32 = 8;
pub const DOT11_AUTH_ALGORITHM_WPA3_SAE: u32 = 9;
pub const DOT11_AUTH_ALGORITHM_WPA_NONE: u32 = 5;
pub const DOT11_AUTH_ALGORITHM_WPA_PSK: u32 = 4;
pub const DOT11_AUTH_ALGO_80211_OPEN: DOT11_AUTH_ALGORITHM = 1;
pub const DOT11_AUTH_ALGO_80211_SHARED_KEY: DOT11_AUTH_ALGORITHM = 2;
pub const DOT11_AUTH_ALGO_IHV_END: DOT11_AUTH_ALGORITHM = -1;
pub const DOT11_AUTH_ALGO_IHV_START: DOT11_AUTH_ALGORITHM = -2147483648;
pub const DOT11_AUTH_ALGO_OWE: DOT11_AUTH_ALGORITHM = 10;
pub const DOT11_AUTH_ALGO_RSNA: DOT11_AUTH_ALGORITHM = 6;
pub const DOT11_AUTH_ALGO_RSNA_PSK: DOT11_AUTH_ALGORITHM = 7;
pub const DOT11_AUTH_ALGO_WPA: DOT11_AUTH_ALGORITHM = 3;
pub const DOT11_AUTH_ALGO_WPA3: DOT11_AUTH_ALGORITHM = 8;
pub const DOT11_AUTH_ALGO_WPA3_ENT: DOT11_AUTH_ALGORITHM = 11;
pub const DOT11_AUTH_ALGO_WPA3_ENT_192: DOT11_AUTH_ALGORITHM = 8;
pub const DOT11_AUTH_ALGO_WPA3_SAE: DOT11_AUTH_ALGORITHM = 9;
pub const DOT11_AUTH_ALGO_WPA_NONE: DOT11_AUTH_ALGORITHM = 5;
pub const DOT11_AUTH_ALGO_WPA_PSK: DOT11_AUTH_ALGORITHM = 4;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DOT11_AUTH_CIPHER_PAIR {
    pub AuthAlgoId: DOT11_AUTH_ALGORITHM,
    pub CipherAlgoId: DOT11_CIPHER_ALGORITHM,
}
pub type DOT11_BSS_TYPE = i32;
pub type DOT11_CIPHER_ALGORITHM = i32;
pub const DOT11_CIPHER_ALGO_BIP: DOT11_CIPHER_ALGORITHM = 6;
pub const DOT11_CIPHER_ALGO_BIP_CMAC_256: DOT11_CIPHER_ALGORITHM = 13;
pub const DOT11_CIPHER_ALGO_BIP_GMAC_128: DOT11_CIPHER_ALGORITHM = 11;
pub const DOT11_CIPHER_ALGO_BIP_GMAC_256: DOT11_CIPHER_ALGORITHM = 12;
pub const DOT11_CIPHER_ALGO_CCMP: DOT11_CIPHER_ALGORITHM = 4;
pub const DOT11_CIPHER_ALGO_CCMP_256: DOT11_CIPHER_ALGORITHM = 10;
pub const DOT11_CIPHER_ALGO_GCMP: DOT11_CIPHER_ALGORITHM = 8;
pub const DOT11_CIPHER_ALGO_GCMP_256: DOT11_CIPHER_ALGORITHM = 9;
pub const DOT11_CIPHER_ALGO_IHV_END: DOT11_CIPHER_ALGORITHM = -1;
pub const DOT11_CIPHER_ALGO_IHV_START: DOT11_CIPHER_ALGORITHM = -2147483648;
pub const DOT11_CIPHER_ALGO_NONE: DOT11_CIPHER_ALGORITHM = 0;
pub const DOT11_CIPHER_ALGO_RSN_USE_GROUP: DOT11_CIPHER_ALGORITHM = 256;
pub const DOT11_CIPHER_ALGO_TKIP: DOT11_CIPHER_ALGORITHM = 2;
pub const DOT11_CIPHER_ALGO_WEP: DOT11_CIPHER_ALGORITHM = 257;
pub const DOT11_CIPHER_ALGO_WEP104: DOT11_CIPHER_ALGORITHM = 5;
pub const DOT11_CIPHER_ALGO_WEP40: DOT11_CIPHER_ALGORITHM = 1;
pub const DOT11_CIPHER_ALGO_WPA_USE_GROUP: DOT11_CIPHER_ALGORITHM = 256;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DOT11_NETWORK {
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOT11_NETWORK_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub Network: [DOT11_NETWORK; 1],
}
impl Default for DOT11_NETWORK_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOT11_OI {
    pub OILength: u16,
    pub OI: [u8; 5],
}
impl Default for DOT11_OI {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_OI_MAX_LENGTH: u32 = 5;
pub const DOT11_OI_MIN_LENGTH: u32 = 3;
pub const DOT11_PSD_IE_MAX_DATA_SIZE: u32 = 240;
pub const DOT11_PSD_IE_MAX_ENTRY_NUMBER: u32 = 5;
pub type DOT11_RADIO_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct DOT11_SSID {
    pub uSSIDLength: u32,
    pub ucSSID: [u8; 32],
}
impl Default for DOT11_SSID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const DOT11_SSID_MAX_LENGTH: u32 = 32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct DOT11_VENUEINFO {
    pub VenueGroup: u8,
    pub VenueType: u8,
}
pub const GUID_AEPSERVICE_WIFIDIRECT_DEVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcc29827c_9caf_4928_99a9_18f7c2381389);
pub const GUID_DEVINTERFACE_ASP_INFRA_DEVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xff823995_7a72_4c80_8757_c67ee13d1a49);
pub const GUID_DEVINTERFACE_WIFIDIRECT_DEVICE: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x439b20af_8955_405b_99f0_a62af0c68d43);
pub type PDOT11_ACCESSNETWORKOPTIONS = *mut DOT11_ACCESSNETWORKOPTIONS;
pub type PDOT11_AUTH_ALGORITHM = *mut DOT11_AUTH_ALGORITHM;
pub type PDOT11_AUTH_CIPHER_PAIR = *mut DOT11_AUTH_CIPHER_PAIR;
pub type PDOT11_BSS_TYPE = *mut DOT11_BSS_TYPE;
pub type PDOT11_CIPHER_ALGORITHM = *mut DOT11_CIPHER_ALGORITHM;
pub type PDOT11_NETWORK = *mut DOT11_NETWORK;
pub type PDOT11_NETWORK_LIST = *mut DOT11_NETWORK_LIST;
pub type PDOT11_OI = *mut DOT11_OI;
pub type PDOT11_RADIO_STATE = *mut DOT11_RADIO_STATE;
pub type PDOT11_SSID = *mut DOT11_SSID;
pub type PDOT11_VENUEINFO = *mut DOT11_VENUEINFO;
#[cfg(feature = "Win32_windot11")]
pub type PWFD_GROUP_ID = *mut WFD_GROUP_ID;
pub type PWFD_ROLE_TYPE = *mut WFD_ROLE_TYPE;
pub type PWLAN_ADHOC_NETWORK_STATE = *mut WLAN_ADHOC_NETWORK_STATE;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_ASSOCIATION_ATTRIBUTES = *mut WLAN_ASSOCIATION_ATTRIBUTES;
pub type PWLAN_AUTH_CIPHER_PAIR_LIST = *mut WLAN_AUTH_CIPHER_PAIR_LIST;
pub type PWLAN_AUTOCONF_OPCODE = *mut WLAN_AUTOCONF_OPCODE;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_AVAILABLE_NETWORK = *mut WLAN_AVAILABLE_NETWORK;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_AVAILABLE_NETWORK_LIST = *mut WLAN_AVAILABLE_NETWORK_LIST;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_AVAILABLE_NETWORK_LIST_V2 = *mut WLAN_AVAILABLE_NETWORK_LIST_V2;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_AVAILABLE_NETWORK_V2 = *mut WLAN_AVAILABLE_NETWORK_V2;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_BSS_ENTRY = *mut WLAN_BSS_ENTRY;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_BSS_LIST = *mut WLAN_BSS_LIST;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_CONNECTION_ATTRIBUTES = *mut WLAN_CONNECTION_ATTRIBUTES;
pub type PWLAN_CONNECTION_MODE = *mut WLAN_CONNECTION_MODE;
pub type PWLAN_CONNECTION_NOTIFICATION_DATA = *mut WLAN_CONNECTION_NOTIFICATION_DATA;
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_windot11"))]
pub type PWLAN_CONNECTION_PARAMETERS = *mut WLAN_CONNECTION_PARAMETERS;
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_windot11"))]
pub type PWLAN_CONNECTION_PARAMETERS_V2 = *mut WLAN_CONNECTION_PARAMETERS_V2;
pub type PWLAN_CONNECTION_QOS_INFO = *mut WLAN_CONNECTION_QOS_INFO;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_COUNTRY_OR_REGION_STRING_LIST = *mut WLAN_COUNTRY_OR_REGION_STRING_LIST;
pub type PWLAN_DEVICE_SERVICE_GUID_LIST = *mut WLAN_DEVICE_SERVICE_GUID_LIST;
pub type PWLAN_DEVICE_SERVICE_NOTIFICATION_DATA = *mut WLAN_DEVICE_SERVICE_NOTIFICATION_DATA;
pub type PWLAN_FILTER_LIST_TYPE = *mut WLAN_FILTER_LIST_TYPE;
pub type PWLAN_HOSTED_NETWORK_CONNECTION_SETTINGS = *mut WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE = *mut WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE;
pub type PWLAN_HOSTED_NETWORK_NOTIFICATION_CODE = *mut WLAN_HOSTED_NETWORK_NOTIFICATION_CODE;
pub type PWLAN_HOSTED_NETWORK_OPCODE = *mut WLAN_HOSTED_NETWORK_OPCODE;
pub type PWLAN_HOSTED_NETWORK_PEER_AUTH_STATE = *mut WLAN_HOSTED_NETWORK_PEER_AUTH_STATE;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_HOSTED_NETWORK_PEER_STATE = *mut WLAN_HOSTED_NETWORK_PEER_STATE;
pub type PWLAN_HOSTED_NETWORK_RADIO_STATE = *mut WLAN_HOSTED_NETWORK_RADIO_STATE;
pub type PWLAN_HOSTED_NETWORK_REASON = *mut WLAN_HOSTED_NETWORK_REASON;
pub type PWLAN_HOSTED_NETWORK_SECURITY_SETTINGS = *mut WLAN_HOSTED_NETWORK_SECURITY_SETTINGS;
pub type PWLAN_HOSTED_NETWORK_STATE = *mut WLAN_HOSTED_NETWORK_STATE;
pub type PWLAN_HOSTED_NETWORK_STATE_CHANGE = *mut WLAN_HOSTED_NETWORK_STATE_CHANGE;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_HOSTED_NETWORK_STATUS = *mut WLAN_HOSTED_NETWORK_STATUS;
pub type PWLAN_IHV_CONTROL_TYPE = *mut WLAN_IHV_CONTROL_TYPE;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_INTERFACE_CAPABILITY = *mut WLAN_INTERFACE_CAPABILITY;
pub type PWLAN_INTERFACE_INFO = *mut WLAN_INTERFACE_INFO;
pub type PWLAN_INTERFACE_INFO_LIST = *mut WLAN_INTERFACE_INFO_LIST;
pub type PWLAN_INTERFACE_STATE = *mut WLAN_INTERFACE_STATE;
pub type PWLAN_INTERFACE_TYPE = *mut WLAN_INTERFACE_TYPE;
pub type PWLAN_INTF_OPCODE = *mut WLAN_INTF_OPCODE;
pub type PWLAN_MAC_FRAME_STATISTICS = *mut WLAN_MAC_FRAME_STATISTICS;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_MSM_NOTIFICATION_DATA = *mut WLAN_MSM_NOTIFICATION_DATA;
pub type PWLAN_NOTIFICATION_ACM = *mut WLAN_NOTIFICATION_ACM;
#[cfg(feature = "Win32_l2cmn")]
pub type PWLAN_NOTIFICATION_DATA = *mut super::l2cmn::L2_NOTIFICATION_DATA;
pub type PWLAN_NOTIFICATION_MSM = *mut WLAN_NOTIFICATION_MSM;
pub type PWLAN_NOTIFICATION_SECURITY = *mut WLAN_NOTIFICATION_SECURITY;
pub type PWLAN_OPCODE_VALUE_TYPE = *mut WLAN_OPCODE_VALUE_TYPE;
pub type PWLAN_OPERATIONAL_STATE = *mut WLAN_OPERATIONAL_STATE;
pub type PWLAN_PHY_FRAME_STATISTICS = *mut WLAN_PHY_FRAME_STATISTICS;
pub type PWLAN_PHY_RADIO_STATE = *mut WLAN_PHY_RADIO_STATE;
pub type PWLAN_POWER_SETTING = *mut WLAN_POWER_SETTING;
pub type PWLAN_PROFILE_INFO = *mut WLAN_PROFILE_INFO;
pub type PWLAN_PROFILE_INFO_LIST = *mut WLAN_PROFILE_INFO_LIST;
pub type PWLAN_QOS_CAPABILITIES = *mut WLAN_QOS_CAPABILITIES;
pub type PWLAN_QOS_INFO = *mut WLAN_QOS_INFO;
pub type PWLAN_RADIO_STATE = *mut WLAN_RADIO_STATE;
pub type PWLAN_RATE_SET = *mut WLAN_RATE_SET;
pub type PWLAN_RAW_DATA = *mut WLAN_RAW_DATA;
pub type PWLAN_RAW_DATA_LIST = *mut WLAN_RAW_DATA_LIST;
#[cfg(feature = "Win32_windot11")]
pub type PWLAN_REALTIME_CONNECTION_QUALITY = *mut WLAN_REALTIME_CONNECTION_QUALITY;
pub type PWLAN_REALTIME_CONNECTION_QUALITY_LINK_INFO = *mut WLAN_REALTIME_CONNECTION_QUALITY_LINK_INFO;
pub type PWLAN_REASON_CODE = *mut u32;
pub type PWLAN_SECURABLE_OBJECT = *mut WLAN_SECURABLE_OBJECT;
pub type PWLAN_SECURITY_ATTRIBUTES = *mut WLAN_SECURITY_ATTRIBUTES;
pub type PWLAN_SIGNAL_QUALITY = *mut u32;
pub type PWLAN_STATISTICS = *mut WLAN_STATISTICS;
pub type PWL_DISPLAY_PAGES = *mut WL_DISPLAY_PAGES;
pub const WFD_API_VERSION: u32 = 1;
pub const WFD_API_VERSION_1_0: u32 = 1;
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WFD_GROUP_ID {
    pub DeviceAddress: super::windot11::DOT11_MAC_ADDRESS,
    pub GroupSSID: DOT11_SSID,
}
#[cfg(feature = "Win32_windot11")]
impl Default for WFD_GROUP_ID {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_winnt")]
pub type WFD_OPEN_SESSION_COMPLETE_CALLBACK = Option<unsafe extern "system" fn(hsessionhandle: super::winnt::HANDLE, pvcontext: *const core::ffi::c_void, guidsessioninterface: windows_sys::core::GUID, dwerror: u32, dwreasoncode: u32)>;
pub type WFD_ROLE_TYPE = i32;
pub const WFD_ROLE_TYPE_CLIENT: WFD_ROLE_TYPE = 4;
pub const WFD_ROLE_TYPE_DEVICE: WFD_ROLE_TYPE = 1;
pub const WFD_ROLE_TYPE_GROUP_OWNER: WFD_ROLE_TYPE = 2;
pub const WFD_ROLE_TYPE_MAX: WFD_ROLE_TYPE = 5;
pub const WFD_ROLE_TYPE_NONE: WFD_ROLE_TYPE = 0;
pub type WLAN_ADHOC_NETWORK_STATE = i32;
pub const WLAN_API_VERSION: u32 = 2;
pub const WLAN_API_VERSION_1_0: u32 = 1;
pub const WLAN_API_VERSION_2_0: u32 = 2;
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WLAN_ASSOCIATION_ATTRIBUTES {
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dot11Bssid: super::windot11::DOT11_MAC_ADDRESS,
    pub dot11PhyType: super::windot11::DOT11_PHY_TYPE,
    pub uDot11PhyIndex: u32,
    pub wlanSignalQuality: WLAN_SIGNAL_QUALITY,
    pub ulRxRate: u32,
    pub ulTxRate: u32,
}
#[cfg(feature = "Win32_windot11")]
impl Default for WLAN_ASSOCIATION_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLAN_AUTH_CIPHER_PAIR_LIST {
    pub dwNumberOfItems: u32,
    pub pAuthCipherPairList: [DOT11_AUTH_CIPHER_PAIR; 1],
}
impl Default for WLAN_AUTH_CIPHER_PAIR_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WLAN_AUTOCONF_OPCODE = i32;
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WLAN_AVAILABLE_NETWORK {
    pub strProfileName: [u16; 256],
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub uNumberOfBssids: u32,
    pub bNetworkConnectable: windows_sys::core::BOOL,
    pub wlanNotConnectableReason: WLAN_REASON_CODE,
    pub uNumberOfPhyTypes: u32,
    pub dot11PhyTypes: [super::windot11::DOT11_PHY_TYPE; 8],
    pub bMorePhyTypes: windows_sys::core::BOOL,
    pub wlanSignalQuality: WLAN_SIGNAL_QUALITY,
    pub bSecurityEnabled: windows_sys::core::BOOL,
    pub dot11DefaultAuthAlgorithm: DOT11_AUTH_ALGORITHM,
    pub dot11DefaultCipherAlgorithm: DOT11_CIPHER_ALGORITHM,
    pub dwFlags: u32,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_windot11")]
impl Default for WLAN_AVAILABLE_NETWORK {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WLAN_AVAILABLE_NETWORK_ANQP_SUPPORTED: u32 = 32;
pub const WLAN_AVAILABLE_NETWORK_AUTO_CONNECT_FAILED: u32 = 256;
pub const WLAN_AVAILABLE_NETWORK_CONNECTED: u32 = 1;
pub const WLAN_AVAILABLE_NETWORK_CONSOLE_USER_PROFILE: u32 = 4;
pub const WLAN_AVAILABLE_NETWORK_HAS_PROFILE: u32 = 2;
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_DOMAIN: u32 = 64;
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_ENABLED: u32 = 16;
pub const WLAN_AVAILABLE_NETWORK_HOTSPOT2_ROAMING: u32 = 128;
pub const WLAN_AVAILABLE_NETWORK_INCLUDE_ALL_ADHOC_PROFILES: u32 = 1;
pub const WLAN_AVAILABLE_NETWORK_INCLUDE_ALL_MANUAL_HIDDEN_PROFILES: u32 = 2;
pub const WLAN_AVAILABLE_NETWORK_INTERWORKING_SUPPORTED: u32 = 8;
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WLAN_AVAILABLE_NETWORK_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub Network: [WLAN_AVAILABLE_NETWORK; 1],
}
#[cfg(feature = "Win32_windot11")]
impl Default for WLAN_AVAILABLE_NETWORK_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WLAN_AVAILABLE_NETWORK_LIST_V2 {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub Network: [WLAN_AVAILABLE_NETWORK_V2; 1],
}
#[cfg(feature = "Win32_windot11")]
impl Default for WLAN_AVAILABLE_NETWORK_LIST_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WLAN_AVAILABLE_NETWORK_V2 {
    pub strProfileName: [u16; 256],
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub uNumberOfBssids: u32,
    pub bNetworkConnectable: windows_sys::core::BOOL,
    pub wlanNotConnectableReason: WLAN_REASON_CODE,
    pub uNumberOfPhyTypes: u32,
    pub dot11PhyTypes: [super::windot11::DOT11_PHY_TYPE; 8],
    pub bMorePhyTypes: windows_sys::core::BOOL,
    pub wlanSignalQuality: WLAN_SIGNAL_QUALITY,
    pub bSecurityEnabled: windows_sys::core::BOOL,
    pub dot11DefaultAuthAlgorithm: DOT11_AUTH_ALGORITHM,
    pub dot11DefaultCipherAlgorithm: DOT11_CIPHER_ALGORITHM,
    pub dwFlags: u32,
    pub AccessNetworkOptions: DOT11_ACCESSNETWORKOPTIONS,
    pub dot11HESSID: super::windot11::DOT11_HESSID,
    pub VenueInfo: DOT11_VENUEINFO,
    pub dwReserved: u32,
}
#[cfg(feature = "Win32_windot11")]
impl Default for WLAN_AVAILABLE_NETWORK_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WLAN_BSS_ENTRY {
    pub dot11Ssid: DOT11_SSID,
    pub uPhyId: u32,
    pub dot11Bssid: super::windot11::DOT11_MAC_ADDRESS,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dot11BssPhyType: super::windot11::DOT11_PHY_TYPE,
    pub lRssi: i32,
    pub uLinkQuality: u32,
    pub bInRegDomain: bool,
    pub usBeaconPeriod: u16,
    pub ullTimestamp: u64,
    pub ullHostTimestamp: u64,
    pub usCapabilityInformation: u16,
    pub ulChCenterFrequency: u32,
    pub wlanRateSet: WLAN_RATE_SET,
    pub ulIeOffset: u32,
    pub ulIeSize: u32,
}
#[cfg(feature = "Win32_windot11")]
impl Default for WLAN_BSS_ENTRY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WLAN_BSS_LIST {
    pub dwTotalSize: u32,
    pub dwNumberOfItems: u32,
    pub wlanBssEntries: [WLAN_BSS_ENTRY; 1],
}
#[cfg(feature = "Win32_windot11")]
impl Default for WLAN_BSS_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WLAN_CONNECTION_ADHOC_JOIN_ONLY: u32 = 2;
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WLAN_CONNECTION_ATTRIBUTES {
    pub isState: WLAN_INTERFACE_STATE,
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfileName: [u16; 256],
    pub wlanAssociationAttributes: WLAN_ASSOCIATION_ATTRIBUTES,
    pub wlanSecurityAttributes: WLAN_SECURITY_ATTRIBUTES,
}
#[cfg(feature = "Win32_windot11")]
impl Default for WLAN_CONNECTION_ATTRIBUTES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WLAN_CONNECTION_EAPOL_PASSTHROUGH: u32 = 8;
pub const WLAN_CONNECTION_HIDDEN_NETWORK: u32 = 1;
pub const WLAN_CONNECTION_IGNORE_PRIVACY_BIT: u32 = 4;
pub type WLAN_CONNECTION_MODE = i32;
pub const WLAN_CONNECTION_NOTIFICATION_ADHOC_NETWORK_FORMED: u32 = 1;
pub const WLAN_CONNECTION_NOTIFICATION_CONSOLE_USER_PROFILE: u32 = 4;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLAN_CONNECTION_NOTIFICATION_DATA {
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfileName: [u16; 256],
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub bSecurityEnabled: windows_sys::core::BOOL,
    pub wlanReasonCode: WLAN_REASON_CODE,
    pub dwFlags: u32,
    pub strProfileXml: [u16; 1],
}
impl Default for WLAN_CONNECTION_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_windot11"))]
#[derive(Clone, Copy)]
pub struct WLAN_CONNECTION_PARAMETERS {
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfile: windows_sys::core::PCWSTR,
    pub pDot11Ssid: PDOT11_SSID,
    pub pDesiredBssidList: super::windot11::PDOT11_BSSID_LIST,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dwFlags: u32,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_windot11"))]
impl Default for WLAN_CONNECTION_PARAMETERS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_windot11"))]
#[derive(Clone, Copy)]
pub struct WLAN_CONNECTION_PARAMETERS_V2 {
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfile: windows_sys::core::PCWSTR,
    pub pDot11Ssid: PDOT11_SSID,
    pub pDot11Hessid: super::windot11::PDOT11_HESSID,
    pub pDesiredBssidList: super::windot11::PDOT11_BSSID_LIST,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dwFlags: u32,
    pub pDot11AccessNetworkOptions: PDOT11_ACCESSNETWORKOPTIONS,
}
#[cfg(all(feature = "Win32_objectheader", feature = "Win32_windot11"))]
impl Default for WLAN_CONNECTION_PARAMETERS_V2 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE: u32 = 16;
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE_CONNECTION_MODE_AUTO: u32 = 32;
pub const WLAN_CONNECTION_PERSIST_DISCOVERY_PROFILE_OVERWRITE_EXISTING: u32 = 64;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WLAN_CONNECTION_QOS_INFO {
    pub peerCapabilities: WLAN_QOS_CAPABILITIES,
    pub bMSCSConfigured: windows_sys::core::BOOL,
    pub bDSCPToUPMappingConfigured: windows_sys::core::BOOL,
    pub ulNumConfiguredSCSStreams: u32,
    pub ulNumConfiguredDSCPPolicies: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WLAN_COUNTRY_OR_REGION_STRING_LIST {
    pub dwNumberOfItems: u32,
    pub pCountryOrRegionStringList: [super::windot11::DOT11_COUNTRY_OR_REGION_STRING; 1],
}
#[cfg(feature = "Win32_windot11")]
impl Default for WLAN_COUNTRY_OR_REGION_STRING_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLAN_DEVICE_SERVICE_GUID_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub DeviceService: [windows_sys::core::GUID; 1],
}
impl Default for WLAN_DEVICE_SERVICE_GUID_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    pub DeviceService: windows_sys::core::GUID,
    pub dwOpCode: u32,
    pub dwDataSize: u32,
    pub DataBlob: [u8; 1],
}
impl Default for WLAN_DEVICE_SERVICE_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WLAN_EXECUTE_ACCESS: u32 = 131105;
pub type WLAN_FILTER_LIST_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WLAN_HOSTED_NETWORK_CONNECTION_SETTINGS {
    pub hostedNetworkSSID: DOT11_SSID,
    pub dwMaxNumberOfPeers: u32,
}
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy, Default)]
pub struct WLAN_HOSTED_NETWORK_DATA_PEER_STATE_CHANGE {
    pub OldState: WLAN_HOSTED_NETWORK_PEER_STATE,
    pub NewState: WLAN_HOSTED_NETWORK_PEER_STATE,
    pub PeerStateChangeReason: WLAN_HOSTED_NETWORK_REASON,
}
pub type WLAN_HOSTED_NETWORK_NOTIFICATION_CODE = i32;
pub type WLAN_HOSTED_NETWORK_OPCODE = i32;
pub type WLAN_HOSTED_NETWORK_PEER_AUTH_STATE = i32;
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WLAN_HOSTED_NETWORK_PEER_STATE {
    pub PeerMacAddress: super::windot11::DOT11_MAC_ADDRESS,
    pub PeerAuthState: WLAN_HOSTED_NETWORK_PEER_AUTH_STATE,
}
#[cfg(feature = "Win32_windot11")]
impl Default for WLAN_HOSTED_NETWORK_PEER_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WLAN_HOSTED_NETWORK_RADIO_STATE {
    pub dot11SoftwareRadioState: DOT11_RADIO_STATE,
    pub dot11HardwareRadioState: DOT11_RADIO_STATE,
}
pub type WLAN_HOSTED_NETWORK_REASON = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WLAN_HOSTED_NETWORK_SECURITY_SETTINGS {
    pub dot11AuthAlgo: DOT11_AUTH_ALGORITHM,
    pub dot11CipherAlgo: DOT11_CIPHER_ALGORITHM,
}
pub type WLAN_HOSTED_NETWORK_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WLAN_HOSTED_NETWORK_STATE_CHANGE {
    pub OldState: WLAN_HOSTED_NETWORK_STATE,
    pub NewState: WLAN_HOSTED_NETWORK_STATE,
    pub StateChangeReason: WLAN_HOSTED_NETWORK_REASON,
}
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WLAN_HOSTED_NETWORK_STATUS {
    pub HostedNetworkState: WLAN_HOSTED_NETWORK_STATE,
    pub IPDeviceID: windows_sys::core::GUID,
    pub wlanHostedNetworkBSSID: super::windot11::DOT11_MAC_ADDRESS,
    pub dot11PhyType: super::windot11::DOT11_PHY_TYPE,
    pub ulChannelFrequency: u32,
    pub dwNumberOfPeers: u32,
    pub PeerList: [WLAN_HOSTED_NETWORK_PEER_STATE; 1],
}
#[cfg(feature = "Win32_windot11")]
impl Default for WLAN_HOSTED_NETWORK_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WLAN_IHV_CONTROL_TYPE = i32;
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WLAN_INTERFACE_CAPABILITY {
    pub interfaceType: WLAN_INTERFACE_TYPE,
    pub bDot11DSupported: windows_sys::core::BOOL,
    pub dwMaxDesiredSsidListSize: u32,
    pub dwMaxDesiredBssidListSize: u32,
    pub dwNumberOfSupportedPhys: u32,
    pub dot11PhyTypes: [super::windot11::DOT11_PHY_TYPE; 64],
}
#[cfg(feature = "Win32_windot11")]
impl Default for WLAN_INTERFACE_CAPABILITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLAN_INTERFACE_INFO {
    pub InterfaceGuid: windows_sys::core::GUID,
    pub strInterfaceDescription: [u16; 256],
    pub isState: WLAN_INTERFACE_STATE,
}
impl Default for WLAN_INTERFACE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLAN_INTERFACE_INFO_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub InterfaceInfo: [WLAN_INTERFACE_INFO; 1],
}
impl Default for WLAN_INTERFACE_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WLAN_INTERFACE_STATE = i32;
pub type WLAN_INTERFACE_TYPE = i32;
pub type WLAN_INTF_OPCODE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
pub const WLAN_MAX_NAME_LENGTH: u32 = 256;
pub const WLAN_MAX_PHY_INDEX: u32 = 64;
pub const WLAN_MAX_PHY_TYPE_NUMBER: u32 = 8;
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WLAN_MSM_NOTIFICATION_DATA {
    pub wlanConnectionMode: WLAN_CONNECTION_MODE,
    pub strProfileName: [u16; 256],
    pub dot11Ssid: DOT11_SSID,
    pub dot11BssType: DOT11_BSS_TYPE,
    pub dot11MacAddr: super::windot11::DOT11_MAC_ADDRESS,
    pub bSecurityEnabled: windows_sys::core::BOOL,
    pub bFirstPeer: windows_sys::core::BOOL,
    pub bLastPeer: windows_sys::core::BOOL,
    pub wlanReasonCode: WLAN_REASON_CODE,
}
#[cfg(feature = "Win32_windot11")]
impl Default for WLAN_MSM_NOTIFICATION_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WLAN_NOTIFICATION_ACM = i32;
#[cfg(feature = "Win32_l2cmn")]
pub type WLAN_NOTIFICATION_CALLBACK = Option<unsafe extern "system" fn(param0: *mut super::l2cmn::L2_NOTIFICATION_DATA, param1: *mut core::ffi::c_void)>;
#[cfg(feature = "Win32_l2cmn")]
pub type WLAN_NOTIFICATION_DATA = super::l2cmn::L2_NOTIFICATION_DATA;
pub type WLAN_NOTIFICATION_MSM = i32;
pub type WLAN_NOTIFICATION_SECURITY = i32;
pub const WLAN_NOTIFICATION_SOURCE_ACM: u32 = 8;
pub const WLAN_NOTIFICATION_SOURCE_ALL: u32 = 65535;
pub const WLAN_NOTIFICATION_SOURCE_DEVICE_SERVICE: u32 = 2048;
pub const WLAN_NOTIFICATION_SOURCE_HNWK: u32 = 128;
pub const WLAN_NOTIFICATION_SOURCE_IHV: u32 = 64;
pub const WLAN_NOTIFICATION_SOURCE_MSM: u32 = 16;
pub const WLAN_NOTIFICATION_SOURCE_NONE: u32 = 0;
pub const WLAN_NOTIFICATION_SOURCE_ONEX: u32 = 4;
pub const WLAN_NOTIFICATION_SOURCE_SECURITY: u32 = 32;
pub type WLAN_OPCODE_VALUE_TYPE = i32;
pub type WLAN_OPERATIONAL_STATE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
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
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WLAN_PHY_RADIO_STATE {
    pub dwPhyIndex: u32,
    pub dot11SoftwareRadioState: DOT11_RADIO_STATE,
    pub dot11HardwareRadioState: DOT11_RADIO_STATE,
}
pub type WLAN_POWER_SETTING = i32;
pub const WLAN_PROFILE_CONNECTION_MODE_AUTO: u32 = 131072;
pub const WLAN_PROFILE_CONNECTION_MODE_SET_BY_CLIENT: u32 = 65536;
pub const WLAN_PROFILE_GET_PLAINTEXT_KEY: u32 = 4;
pub const WLAN_PROFILE_GROUP_POLICY: u32 = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLAN_PROFILE_INFO {
    pub strProfileName: [u16; 256],
    pub dwFlags: u32,
}
impl Default for WLAN_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLAN_PROFILE_INFO_LIST {
    pub dwNumberOfItems: u32,
    pub dwIndex: u32,
    pub ProfileInfo: [WLAN_PROFILE_INFO; 1],
}
impl Default for WLAN_PROFILE_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WLAN_PROFILE_USER: u32 = 2;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WLAN_QOS_CAPABILITIES {
    pub bMSCSSupported: windows_sys::core::BOOL,
    pub bDSCPToUPMappingSupported: windows_sys::core::BOOL,
    pub bSCSSupported: windows_sys::core::BOOL,
    pub bDSCPPolicySupported: windows_sys::core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WLAN_QOS_INFO {
    pub interfaceCapabilities: WLAN_QOS_CAPABILITIES,
    pub bConnected: windows_sys::core::BOOL,
    pub connectionQoSInfo: WLAN_CONNECTION_QOS_INFO,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLAN_RADIO_STATE {
    pub dwNumberOfPhys: u32,
    pub PhyRadioState: [WLAN_PHY_RADIO_STATE; 64],
}
impl Default for WLAN_RADIO_STATE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLAN_RATE_SET {
    pub uRateSetLength: u32,
    pub usRateSet: [u16; 126],
}
impl Default for WLAN_RATE_SET {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLAN_RAW_DATA {
    pub dwDataSize: u32,
    pub DataBlob: [u8; 1],
}
impl Default for WLAN_RAW_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLAN_RAW_DATA_LIST {
    pub dwTotalSize: u32,
    pub dwNumberOfItems: u32,
    pub DataList: [WLAN_RAW_DATA_LIST_0; 1],
}
impl Default for WLAN_RAW_DATA_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WLAN_RAW_DATA_LIST_0 {
    pub dwDataOffset: u32,
    pub dwDataSize: u32,
}
pub const WLAN_READ_ACCESS: u32 = 131073;
#[repr(C)]
#[cfg(feature = "Win32_windot11")]
#[derive(Clone, Copy)]
pub struct WLAN_REALTIME_CONNECTION_QUALITY {
    pub dot11PhyType: super::windot11::DOT11_PHY_TYPE,
    pub ulLinkQuality: u32,
    pub ulRxRate: u32,
    pub ulTxRate: u32,
    pub bIsMLOConnection: windows_sys::core::BOOL,
    pub ulNumLinks: u32,
    pub linksInfo: [WLAN_REALTIME_CONNECTION_QUALITY_LINK_INFO; 1],
}
#[cfg(feature = "Win32_windot11")]
impl Default for WLAN_REALTIME_CONNECTION_QUALITY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WLAN_REALTIME_CONNECTION_QUALITY_LINK_INFO {
    pub ucLinkID: u8,
    pub ulChannelCenterFrequencyMhz: u32,
    pub ulBandwidth: u32,
    pub lRssi: i32,
    pub wlanRateSet: WLAN_RATE_SET,
}
pub type WLAN_REASON_CODE = u32;
pub const WLAN_REASON_CODE_AC_BASE: u32 = 131072;
pub const WLAN_REASON_CODE_AC_CONNECT_BASE: u32 = 163840;
pub const WLAN_REASON_CODE_AC_END: u32 = 196607;
pub const WLAN_REASON_CODE_ADHOC_SECURITY_FAILURE: u32 = 229386;
pub const WLAN_REASON_CODE_AP_PROFILE_NOT_ALLOWED: u32 = 163856;
pub const WLAN_REASON_CODE_AP_PROFILE_NOT_ALLOWED_FOR_CLIENT: u32 = 163855;
pub const WLAN_REASON_CODE_AP_STARTING_FAILURE: u32 = 229395;
pub const WLAN_REASON_CODE_ASSOCIATION_FAILURE: u32 = 229378;
pub const WLAN_REASON_CODE_ASSOCIATION_TIMEOUT: u32 = 229379;
pub const WLAN_REASON_CODE_AUTO_AP_PROFILE_NOT_ALLOWED: u32 = 524313;
pub const WLAN_REASON_CODE_AUTO_CONNECTION_NOT_ALLOWED: u32 = 524314;
pub const WLAN_REASON_CODE_AUTO_SWITCH_SET_FOR_ADHOC: u32 = 524304;
pub const WLAN_REASON_CODE_AUTO_SWITCH_SET_FOR_MANUAL_CONNECTION: u32 = 524305;
pub const WLAN_REASON_CODE_BAD_MAX_NUMBER_OF_CLIENTS_FOR_AP: u32 = 524310;
pub const WLAN_REASON_CODE_BASE: u32 = 131072;
pub const WLAN_REASON_CODE_BSS_TYPE_NOT_ALLOWED: u32 = 163845;
pub const WLAN_REASON_CODE_BSS_TYPE_UNMATCH: u32 = 196611;
pub const WLAN_REASON_CODE_CONFLICT_SECURITY: u32 = 524299;
pub const WLAN_REASON_CODE_CONNECT_CALL_FAIL: u32 = 163849;
pub const WLAN_REASON_CODE_DATARATE_UNMATCH: u32 = 196613;
pub const WLAN_REASON_CODE_DISCONNECT_TIMEOUT: u32 = 229391;
pub const WLAN_REASON_CODE_DRIVER_DISCONNECTED: u32 = 229387;
pub const WLAN_REASON_CODE_DRIVER_OPERATION_FAILURE: u32 = 229388;
pub const WLAN_REASON_CODE_GP_DENIED: u32 = 163843;
pub const WLAN_REASON_CODE_HOTSPOT2_PROFILE_DENIED: u32 = 163857;
pub const WLAN_REASON_CODE_HOTSPOT2_PROFILE_NOT_ALLOWED: u32 = 524315;
pub const WLAN_REASON_CODE_IHV_CONNECTIVITY_NOT_SUPPORTED: u32 = 524309;
pub const WLAN_REASON_CODE_IHV_NOT_AVAILABLE: u32 = 229389;
pub const WLAN_REASON_CODE_IHV_NOT_RESPONDING: u32 = 229390;
pub const WLAN_REASON_CODE_IHV_OUI_MISMATCH: u32 = 524296;
pub const WLAN_REASON_CODE_IHV_OUI_MISSING: u32 = 524297;
pub const WLAN_REASON_CODE_IHV_SECURITY_NOT_SUPPORTED: u32 = 524295;
pub const WLAN_REASON_CODE_IHV_SECURITY_ONEX_MISSING: u32 = 524306;
pub const WLAN_REASON_CODE_IHV_SETTINGS_MISSING: u32 = 524298;
pub const WLAN_REASON_CODE_INTERNAL_FAILURE: u32 = 229392;
pub const WLAN_REASON_CODE_INVALID_ADHOC_CONNECTION_MODE: u32 = 524302;
pub const WLAN_REASON_CODE_INVALID_BSS_TYPE: u32 = 524301;
pub const WLAN_REASON_CODE_INVALID_CHANNEL: u32 = 524311;
pub const WLAN_REASON_CODE_INVALID_PHY_TYPE: u32 = 524293;
pub const WLAN_REASON_CODE_INVALID_PROFILE_NAME: u32 = 524291;
pub const WLAN_REASON_CODE_INVALID_PROFILE_SCHEMA: u32 = 524289;
pub const WLAN_REASON_CODE_INVALID_PROFILE_TYPE: u32 = 524292;
pub const WLAN_REASON_CODE_IN_BLOCKED_LIST: u32 = 163847;
pub const WLAN_REASON_CODE_IN_FAILED_LIST: u32 = 163846;
pub const WLAN_REASON_CODE_KEY_MISMATCH: u32 = 163853;
pub const WLAN_REASON_CODE_MSMSEC_AUTH_START_TIMEOUT: u32 = 294914;
pub const WLAN_REASON_CODE_MSMSEC_AUTH_SUCCESS_TIMEOUT: u32 = 294915;
pub const WLAN_REASON_CODE_MSMSEC_AUTH_WCN_COMPLETED: u32 = 294937;
pub const WLAN_REASON_CODE_MSMSEC_BASE: u32 = 262144;
pub const WLAN_REASON_CODE_MSMSEC_CANCELLED: u32 = 294929;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_DISCOVERY: u32 = 262165;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_MFP_NW_NIC: u32 = 262181;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_NETWORK: u32 = 262162;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_NIC: u32 = 262163;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE: u32 = 262164;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_AUTH: u32 = 262174;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_CIPHER: u32 = 262175;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_SAFE_MODE_NIC: u32 = 262177;
pub const WLAN_REASON_CODE_MSMSEC_CAPABILITY_PROFILE_SAFE_MODE_NW: u32 = 262178;
pub const WLAN_REASON_CODE_MSMSEC_CONNECT_BASE: u32 = 294912;
pub const WLAN_REASON_CODE_MSMSEC_DOWNGRADE_DETECTED: u32 = 294931;
pub const WLAN_REASON_CODE_MSMSEC_END: u32 = 327679;
pub const WLAN_REASON_CODE_MSMSEC_FORCED_FAILURE: u32 = 294933;
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_GRP_KEY: u32 = 294925;
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_KEY_DATA: u32 = 294924;
pub const WLAN_REASON_CODE_MSMSEC_G1_MISSING_MGMT_GRP_KEY: u32 = 294939;
pub const WLAN_REASON_CODE_MSMSEC_KEY_FORMAT: u32 = 294930;
pub const WLAN_REASON_CODE_MSMSEC_KEY_START_TIMEOUT: u32 = 294916;
pub const WLAN_REASON_CODE_MSMSEC_KEY_SUCCESS_TIMEOUT: u32 = 294917;
pub const WLAN_REASON_CODE_MSMSEC_M2_MISSING_IE: u32 = 294936;
pub const WLAN_REASON_CODE_MSMSEC_M2_MISSING_KEY_DATA: u32 = 294935;
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_GRP_KEY: u32 = 294920;
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_IE: u32 = 294919;
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_KEY_DATA: u32 = 294918;
pub const WLAN_REASON_CODE_MSMSEC_M3_MISSING_MGMT_GRP_KEY: u32 = 294938;
pub const WLAN_REASON_CODE_MSMSEC_M3_TOO_MANY_RSNIE: u32 = 294934;
pub const WLAN_REASON_CODE_MSMSEC_MAX: u32 = 327679;
pub const WLAN_REASON_CODE_MSMSEC_MIN: u32 = 262144;
pub const WLAN_REASON_CODE_MSMSEC_MIXED_CELL: u32 = 262169;
pub const WLAN_REASON_CODE_MSMSEC_NIC_FAILURE: u32 = 294928;
pub const WLAN_REASON_CODE_MSMSEC_NO_AUTHENTICATOR: u32 = 294927;
pub const WLAN_REASON_CODE_MSMSEC_NO_PAIRWISE_KEY: u32 = 294923;
pub const WLAN_REASON_CODE_MSMSEC_PEER_INDICATED_INSECURE: u32 = 294926;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_AUTH_TIMERS_INVALID: u32 = 262170;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_DUPLICATE_AUTH_CIPHER: u32 = 262151;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_AUTH_CIPHER: u32 = 262153;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_GKEY_INTV: u32 = 262171;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_KEY_INDEX: u32 = 262145;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_MODE: u32 = 262156;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_SIZE: u32 = 262157;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PMKCACHE_TTL: u32 = 262158;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PREAUTH_MODE: u32 = 262159;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_INVALID_PREAUTH_THROTTLE: u32 = 262160;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEYMATERIAL_CHAR: u32 = 262167;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEY_LENGTH: u32 = 262147;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_KEY_UNMAPPED_CHAR: u32 = 262173;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_NO_AUTH_CIPHER_SPECIFIED: u32 = 262149;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_ONEX_DISABLED: u32 = 262154;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_ONEX_ENABLED: u32 = 262155;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PASSPHRASE_CHAR: u32 = 262166;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PREAUTH_ONLY_ENABLED: u32 = 262161;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PSK_LENGTH: u32 = 262148;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_PSK_PRESENT: u32 = 262146;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_RAWDATA_INVALID: u32 = 262152;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_SAFE_MODE: u32 = 262176;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_TOO_MANY_AUTH_CIPHER_SPECIFIED: u32 = 262150;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_UNSUPPORTED_AUTH: u32 = 262179;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_UNSUPPORTED_CIPHER: u32 = 262180;
pub const WLAN_REASON_CODE_MSMSEC_PROFILE_WRONG_KEYTYPE: u32 = 262168;
pub const WLAN_REASON_CODE_MSMSEC_PR_IE_MATCHING: u32 = 294921;
pub const WLAN_REASON_CODE_MSMSEC_PSK_MISMATCH_SUSPECTED: u32 = 294932;
pub const WLAN_REASON_CODE_MSMSEC_SEC_IE_MATCHING: u32 = 294922;
pub const WLAN_REASON_CODE_MSMSEC_TRANSITION_NETWORK: u32 = 262172;
pub const WLAN_REASON_CODE_MSMSEC_UI_REQUEST_FAILURE: u32 = 294913;
pub const WLAN_REASON_CODE_MSM_BASE: u32 = 196608;
pub const WLAN_REASON_CODE_MSM_CONNECT_BASE: u32 = 229376;
pub const WLAN_REASON_CODE_MSM_END: u32 = 262143;
pub const WLAN_REASON_CODE_MSM_SECURITY_MISSING: u32 = 524294;
pub const WLAN_REASON_CODE_NETWORK_NOT_AVAILABLE: u32 = 163851;
pub const WLAN_REASON_CODE_NETWORK_NOT_COMPATIBLE: u32 = 131073;
pub const WLAN_REASON_CODE_NON_BROADCAST_SET_FOR_ADHOC: u32 = 524303;
pub const WLAN_REASON_CODE_NOT_VISIBLE: u32 = 163842;
pub const WLAN_REASON_CODE_NO_AUTO_CONNECTION: u32 = 163841;
pub const WLAN_REASON_CODE_NO_VISIBLE_AP: u32 = 229396;
pub const WLAN_REASON_CODE_OPERATION_MODE_NOT_SUPPORTED: u32 = 524312;
pub const WLAN_REASON_CODE_PHY_TYPE_UNMATCH: u32 = 196612;
pub const WLAN_REASON_CODE_PRE_SECURITY_FAILURE: u32 = 229380;
pub const WLAN_REASON_CODE_PROFILE_BASE: u32 = 524288;
pub const WLAN_REASON_CODE_PROFILE_CHANGED_OR_DELETED: u32 = 163852;
pub const WLAN_REASON_CODE_PROFILE_CONNECT_BASE: u32 = 557056;
pub const WLAN_REASON_CODE_PROFILE_END: u32 = 589823;
pub const WLAN_REASON_CODE_PROFILE_MISSING: u32 = 524290;
pub const WLAN_REASON_CODE_PROFILE_NOT_COMPATIBLE: u32 = 131074;
pub const WLAN_REASON_CODE_PROFILE_SSID_INVALID: u32 = 524307;
pub const WLAN_REASON_CODE_RANGE_SIZE: u32 = 65536;
pub const WLAN_REASON_CODE_RESERVED_BASE: u32 = 720896;
pub const WLAN_REASON_CODE_RESERVED_END: u32 = 786431;
pub const WLAN_REASON_CODE_ROAMING_FAILURE: u32 = 229384;
pub const WLAN_REASON_CODE_ROAMING_SECURITY_FAILURE: u32 = 229385;
pub const WLAN_REASON_CODE_SCAN_CALL_FAIL: u32 = 163850;
pub const WLAN_REASON_CODE_SECURITY_FAILURE: u32 = 229382;
pub const WLAN_REASON_CODE_SECURITY_MISSING: u32 = 524300;
pub const WLAN_REASON_CODE_SECURITY_TIMEOUT: u32 = 229383;
pub const WLAN_REASON_CODE_SSID_LIST_TOO_LONG: u32 = 163848;
pub const WLAN_REASON_CODE_START_SECURITY_FAILURE: u32 = 229381;
pub const WLAN_REASON_CODE_SUCCESS: u32 = 0;
pub const WLAN_REASON_CODE_TOO_MANY_SECURITY_ATTEMPTS: u32 = 229394;
pub const WLAN_REASON_CODE_TOO_MANY_SSID: u32 = 524308;
pub const WLAN_REASON_CODE_UI_REQUEST_TIMEOUT: u32 = 229393;
pub const WLAN_REASON_CODE_UNKNOWN: u32 = 65537;
pub const WLAN_REASON_CODE_UNSUPPORTED_SECURITY_SET: u32 = 196610;
pub const WLAN_REASON_CODE_UNSUPPORTED_SECURITY_SET_BY_OS: u32 = 196609;
pub const WLAN_REASON_CODE_USER_CANCELLED: u32 = 229377;
pub const WLAN_REASON_CODE_USER_DENIED: u32 = 163844;
pub const WLAN_REASON_CODE_USER_NOT_RESPOND: u32 = 163854;
pub type WLAN_SECURABLE_OBJECT = i32;
pub const WLAN_SECURABLE_OBJECT_COUNT: WLAN_SECURABLE_OBJECT = 17;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WLAN_SECURITY_ATTRIBUTES {
    pub bSecurityEnabled: windows_sys::core::BOOL,
    pub bOneXEnabled: windows_sys::core::BOOL,
    pub dot11AuthAlgorithm: DOT11_AUTH_ALGORITHM,
    pub dot11CipherAlgorithm: DOT11_CIPHER_ALGORITHM,
}
pub const WLAN_SET_EAPHOST_DATA_ALL_USERS: u32 = 1;
pub type WLAN_SIGNAL_QUALITY = u32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct WLAN_STATISTICS {
    pub ullFourWayHandshakeFailures: u64,
    pub ullTKIPCounterMeasuresInvoked: u64,
    pub ullReserved: u64,
    pub MacUcastCounters: WLAN_MAC_FRAME_STATISTICS,
    pub MacMcastCounters: WLAN_MAC_FRAME_STATISTICS,
    pub dwNumberOfPhys: u32,
    pub PhyCounters: [WLAN_PHY_FRAME_STATISTICS; 1],
}
impl Default for WLAN_STATISTICS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WLAN_UI_API_INITIAL_VERSION: u32 = 1;
pub const WLAN_UI_API_VERSION: u32 = 1;
pub const WLAN_WRITE_ACCESS: u32 = 458787;
pub const WLAdvPage: WL_DISPLAY_PAGES = 2;
pub const WLConnectionPage: WL_DISPLAY_PAGES = 0;
pub const WLSecurityPage: WL_DISPLAY_PAGES = 1;
pub type WL_DISPLAY_PAGES = i32;
pub const dot11_BSS_type_any: DOT11_BSS_TYPE = 3;
pub const dot11_BSS_type_independent: DOT11_BSS_TYPE = 2;
pub const dot11_BSS_type_infrastructure: DOT11_BSS_TYPE = 1;
pub const dot11_radio_state_off: DOT11_RADIO_STATE = 2;
pub const dot11_radio_state_on: DOT11_RADIO_STATE = 1;
pub const dot11_radio_state_unknown: DOT11_RADIO_STATE = 0;
pub const wlan_adhoc_network_state_connected: WLAN_ADHOC_NETWORK_STATE = 1;
pub const wlan_adhoc_network_state_formed: WLAN_ADHOC_NETWORK_STATE = 0;
pub const wlan_autoconf_opcode_allow_explicit_creds: WLAN_AUTOCONF_OPCODE = 4;
pub const wlan_autoconf_opcode_allow_virtual_station_extensibility: WLAN_AUTOCONF_OPCODE = 6;
pub const wlan_autoconf_opcode_block_period: WLAN_AUTOCONF_OPCODE = 5;
pub const wlan_autoconf_opcode_end: WLAN_AUTOCONF_OPCODE = 7;
pub const wlan_autoconf_opcode_only_use_gp_profiles_for_allowed_networks: WLAN_AUTOCONF_OPCODE = 3;
pub const wlan_autoconf_opcode_power_setting: WLAN_AUTOCONF_OPCODE = 2;
pub const wlan_autoconf_opcode_show_denied_networks: WLAN_AUTOCONF_OPCODE = 1;
pub const wlan_autoconf_opcode_start: WLAN_AUTOCONF_OPCODE = 0;
pub const wlan_connection_mode_auto: WLAN_CONNECTION_MODE = 4;
pub const wlan_connection_mode_discovery_secure: WLAN_CONNECTION_MODE = 2;
pub const wlan_connection_mode_discovery_unsecure: WLAN_CONNECTION_MODE = 3;
pub const wlan_connection_mode_invalid: WLAN_CONNECTION_MODE = 5;
pub const wlan_connection_mode_profile: WLAN_CONNECTION_MODE = 0;
pub const wlan_connection_mode_temporary_profile: WLAN_CONNECTION_MODE = 1;
pub const wlan_filter_list_type_gp_deny: WLAN_FILTER_LIST_TYPE = 1;
pub const wlan_filter_list_type_gp_permit: WLAN_FILTER_LIST_TYPE = 0;
pub const wlan_filter_list_type_user_deny: WLAN_FILTER_LIST_TYPE = 3;
pub const wlan_filter_list_type_user_permit: WLAN_FILTER_LIST_TYPE = 2;
pub const wlan_hosted_network_active: WLAN_HOSTED_NETWORK_STATE = 2;
pub const wlan_hosted_network_idle: WLAN_HOSTED_NETWORK_STATE = 1;
pub const wlan_hosted_network_opcode_connection_settings: WLAN_HOSTED_NETWORK_OPCODE = 0;
pub const wlan_hosted_network_opcode_enable: WLAN_HOSTED_NETWORK_OPCODE = 3;
pub const wlan_hosted_network_opcode_security_settings: WLAN_HOSTED_NETWORK_OPCODE = 1;
pub const wlan_hosted_network_opcode_station_profile: WLAN_HOSTED_NETWORK_OPCODE = 2;
pub const wlan_hosted_network_peer_state_authenticated: WLAN_HOSTED_NETWORK_PEER_AUTH_STATE = 1;
pub const wlan_hosted_network_peer_state_change: WLAN_HOSTED_NETWORK_NOTIFICATION_CODE = 4097;
pub const wlan_hosted_network_peer_state_invalid: WLAN_HOSTED_NETWORK_PEER_AUTH_STATE = 0;
pub const wlan_hosted_network_radio_state_change: WLAN_HOSTED_NETWORK_NOTIFICATION_CODE = 4098;
pub const wlan_hosted_network_reason_ap_start_failed: WLAN_HOSTED_NETWORK_REASON = 19;
pub const wlan_hosted_network_reason_bad_parameters: WLAN_HOSTED_NETWORK_REASON = 2;
pub const wlan_hosted_network_reason_client_abort: WLAN_HOSTED_NETWORK_REASON = 18;
pub const wlan_hosted_network_reason_crypt_error: WLAN_HOSTED_NETWORK_REASON = 8;
pub const wlan_hosted_network_reason_device_change: WLAN_HOSTED_NETWORK_REASON = 25;
pub const wlan_hosted_network_reason_elevation_required: WLAN_HOSTED_NETWORK_REASON = 5;
pub const wlan_hosted_network_reason_gp_denied: WLAN_HOSTED_NETWORK_REASON = 23;
pub const wlan_hosted_network_reason_impersonation: WLAN_HOSTED_NETWORK_REASON = 9;
pub const wlan_hosted_network_reason_incompatible_connection_started: WLAN_HOSTED_NETWORK_REASON = 15;
pub const wlan_hosted_network_reason_incompatible_connection_stopped: WLAN_HOSTED_NETWORK_REASON = 16;
pub const wlan_hosted_network_reason_insufficient_resources: WLAN_HOSTED_NETWORK_REASON = 4;
pub const wlan_hosted_network_reason_interface_available: WLAN_HOSTED_NETWORK_REASON = 11;
pub const wlan_hosted_network_reason_interface_unavailable: WLAN_HOSTED_NETWORK_REASON = 12;
pub const wlan_hosted_network_reason_miniport_started: WLAN_HOSTED_NETWORK_REASON = 14;
pub const wlan_hosted_network_reason_miniport_stopped: WLAN_HOSTED_NETWORK_REASON = 13;
pub const wlan_hosted_network_reason_peer_arrived: WLAN_HOSTED_NETWORK_REASON = 20;
pub const wlan_hosted_network_reason_peer_departed: WLAN_HOSTED_NETWORK_REASON = 21;
pub const wlan_hosted_network_reason_peer_timeout: WLAN_HOSTED_NETWORK_REASON = 22;
pub const wlan_hosted_network_reason_persistence_failed: WLAN_HOSTED_NETWORK_REASON = 7;
pub const wlan_hosted_network_reason_properties_change: WLAN_HOSTED_NETWORK_REASON = 26;
pub const wlan_hosted_network_reason_read_only: WLAN_HOSTED_NETWORK_REASON = 6;
pub const wlan_hosted_network_reason_service_available_on_virtual_station: WLAN_HOSTED_NETWORK_REASON = 28;
pub const wlan_hosted_network_reason_service_shutting_down: WLAN_HOSTED_NETWORK_REASON = 3;
pub const wlan_hosted_network_reason_service_unavailable: WLAN_HOSTED_NETWORK_REASON = 24;
pub const wlan_hosted_network_reason_stop_before_start: WLAN_HOSTED_NETWORK_REASON = 10;
pub const wlan_hosted_network_reason_success: WLAN_HOSTED_NETWORK_REASON = 0;
pub const wlan_hosted_network_reason_unspecified: WLAN_HOSTED_NETWORK_REASON = 1;
pub const wlan_hosted_network_reason_user_action: WLAN_HOSTED_NETWORK_REASON = 17;
pub const wlan_hosted_network_reason_virtual_station_blocking_use: WLAN_HOSTED_NETWORK_REASON = 27;
pub const wlan_hosted_network_state_change: WLAN_HOSTED_NETWORK_NOTIFICATION_CODE = 4096;
pub const wlan_hosted_network_unavailable: WLAN_HOSTED_NETWORK_STATE = 0;
pub const wlan_ihv_control_type_driver: WLAN_IHV_CONTROL_TYPE = 1;
pub const wlan_ihv_control_type_service: WLAN_IHV_CONTROL_TYPE = 0;
pub const wlan_interface_state_ad_hoc_network_formed: WLAN_INTERFACE_STATE = 2;
pub const wlan_interface_state_associating: WLAN_INTERFACE_STATE = 5;
pub const wlan_interface_state_authenticating: WLAN_INTERFACE_STATE = 7;
pub const wlan_interface_state_connected: WLAN_INTERFACE_STATE = 1;
pub const wlan_interface_state_disconnected: WLAN_INTERFACE_STATE = 4;
pub const wlan_interface_state_disconnecting: WLAN_INTERFACE_STATE = 3;
pub const wlan_interface_state_discovering: WLAN_INTERFACE_STATE = 6;
pub const wlan_interface_state_not_ready: WLAN_INTERFACE_STATE = 0;
pub const wlan_interface_type_emulated_802_11: WLAN_INTERFACE_TYPE = 0;
pub const wlan_interface_type_invalid: WLAN_INTERFACE_TYPE = 2;
pub const wlan_interface_type_native_802_11: WLAN_INTERFACE_TYPE = 1;
pub const wlan_intf_opcode_autoconf_enabled: WLAN_INTF_OPCODE = 1;
pub const wlan_intf_opcode_autoconf_end: WLAN_INTF_OPCODE = 268435455;
pub const wlan_intf_opcode_autoconf_start: WLAN_INTF_OPCODE = 0;
pub const wlan_intf_opcode_background_scan_enabled: WLAN_INTF_OPCODE = 2;
pub const wlan_intf_opcode_bss_type: WLAN_INTF_OPCODE = 5;
pub const wlan_intf_opcode_certified_safe_mode: WLAN_INTF_OPCODE = 14;
pub const wlan_intf_opcode_channel_number: WLAN_INTF_OPCODE = 8;
pub const wlan_intf_opcode_current_connection: WLAN_INTF_OPCODE = 7;
pub const wlan_intf_opcode_current_operation_mode: WLAN_INTF_OPCODE = 12;
pub const wlan_intf_opcode_hosted_network_capable: WLAN_INTF_OPCODE = 15;
pub const wlan_intf_opcode_ihv_end: WLAN_INTF_OPCODE = 1073741823;
pub const wlan_intf_opcode_ihv_start: WLAN_INTF_OPCODE = 805306368;
pub const wlan_intf_opcode_interface_state: WLAN_INTF_OPCODE = 6;
pub const wlan_intf_opcode_management_frame_protection_capable: WLAN_INTF_OPCODE = 16;
pub const wlan_intf_opcode_media_streaming_mode: WLAN_INTF_OPCODE = 3;
pub const wlan_intf_opcode_msm_end: WLAN_INTF_OPCODE = 536870911;
pub const wlan_intf_opcode_msm_start: WLAN_INTF_OPCODE = 268435712;
pub const wlan_intf_opcode_qos_info: WLAN_INTF_OPCODE = 20;
pub const wlan_intf_opcode_radio_state: WLAN_INTF_OPCODE = 4;
pub const wlan_intf_opcode_realtime_connection_quality: WLAN_INTF_OPCODE = 19;
pub const wlan_intf_opcode_rssi: WLAN_INTF_OPCODE = 268435714;
pub const wlan_intf_opcode_secondary_sta_interfaces: WLAN_INTF_OPCODE = 17;
pub const wlan_intf_opcode_secondary_sta_synchronized_connections: WLAN_INTF_OPCODE = 18;
pub const wlan_intf_opcode_security_end: WLAN_INTF_OPCODE = 805306367;
pub const wlan_intf_opcode_security_start: WLAN_INTF_OPCODE = 536936448;
pub const wlan_intf_opcode_statistics: WLAN_INTF_OPCODE = 268435713;
pub const wlan_intf_opcode_supported_adhoc_auth_cipher_pairs: WLAN_INTF_OPCODE = 10;
pub const wlan_intf_opcode_supported_country_or_region_string_list: WLAN_INTF_OPCODE = 11;
pub const wlan_intf_opcode_supported_infrastructure_auth_cipher_pairs: WLAN_INTF_OPCODE = 9;
pub const wlan_intf_opcode_supported_safe_mode: WLAN_INTF_OPCODE = 13;
pub const wlan_notification_acm_adhoc_network_state_change: WLAN_NOTIFICATION_ACM = 22;
pub const wlan_notification_acm_autoconf_disabled: WLAN_NOTIFICATION_ACM = 2;
pub const wlan_notification_acm_autoconf_enabled: WLAN_NOTIFICATION_ACM = 1;
pub const wlan_notification_acm_background_scan_disabled: WLAN_NOTIFICATION_ACM = 4;
pub const wlan_notification_acm_background_scan_enabled: WLAN_NOTIFICATION_ACM = 3;
pub const wlan_notification_acm_bss_type_change: WLAN_NOTIFICATION_ACM = 5;
pub const wlan_notification_acm_connection_attempt_fail: WLAN_NOTIFICATION_ACM = 11;
pub const wlan_notification_acm_connection_complete: WLAN_NOTIFICATION_ACM = 10;
pub const wlan_notification_acm_connection_start: WLAN_NOTIFICATION_ACM = 9;
pub const wlan_notification_acm_disconnected: WLAN_NOTIFICATION_ACM = 21;
pub const wlan_notification_acm_disconnecting: WLAN_NOTIFICATION_ACM = 20;
pub const wlan_notification_acm_end: WLAN_NOTIFICATION_ACM = 28;
pub const wlan_notification_acm_filter_list_change: WLAN_NOTIFICATION_ACM = 12;
pub const wlan_notification_acm_interface_arrival: WLAN_NOTIFICATION_ACM = 13;
pub const wlan_notification_acm_interface_removal: WLAN_NOTIFICATION_ACM = 14;
pub const wlan_notification_acm_network_available: WLAN_NOTIFICATION_ACM = 19;
pub const wlan_notification_acm_network_not_available: WLAN_NOTIFICATION_ACM = 18;
pub const wlan_notification_acm_operational_state_change: WLAN_NOTIFICATION_ACM = 27;
pub const wlan_notification_acm_power_setting_change: WLAN_NOTIFICATION_ACM = 6;
pub const wlan_notification_acm_profile_blocked: WLAN_NOTIFICATION_ACM = 25;
pub const wlan_notification_acm_profile_change: WLAN_NOTIFICATION_ACM = 15;
pub const wlan_notification_acm_profile_name_change: WLAN_NOTIFICATION_ACM = 16;
pub const wlan_notification_acm_profile_unblocked: WLAN_NOTIFICATION_ACM = 23;
pub const wlan_notification_acm_profiles_exhausted: WLAN_NOTIFICATION_ACM = 17;
pub const wlan_notification_acm_scan_complete: WLAN_NOTIFICATION_ACM = 7;
pub const wlan_notification_acm_scan_fail: WLAN_NOTIFICATION_ACM = 8;
pub const wlan_notification_acm_scan_list_refresh: WLAN_NOTIFICATION_ACM = 26;
pub const wlan_notification_acm_screen_power_change: WLAN_NOTIFICATION_ACM = 24;
pub const wlan_notification_acm_start: WLAN_NOTIFICATION_ACM = 0;
pub const wlan_notification_msm_adapter_operation_mode_change: WLAN_NOTIFICATION_MSM = 14;
pub const wlan_notification_msm_adapter_removal: WLAN_NOTIFICATION_MSM = 13;
pub const wlan_notification_msm_associated: WLAN_NOTIFICATION_MSM = 2;
pub const wlan_notification_msm_associating: WLAN_NOTIFICATION_MSM = 1;
pub const wlan_notification_msm_authenticating: WLAN_NOTIFICATION_MSM = 3;
pub const wlan_notification_msm_connected: WLAN_NOTIFICATION_MSM = 4;
pub const wlan_notification_msm_disassociating: WLAN_NOTIFICATION_MSM = 9;
pub const wlan_notification_msm_disconnected: WLAN_NOTIFICATION_MSM = 10;
pub const wlan_notification_msm_end: WLAN_NOTIFICATION_MSM = 17;
pub const wlan_notification_msm_link_degraded: WLAN_NOTIFICATION_MSM = 15;
pub const wlan_notification_msm_link_improved: WLAN_NOTIFICATION_MSM = 16;
pub const wlan_notification_msm_peer_join: WLAN_NOTIFICATION_MSM = 11;
pub const wlan_notification_msm_peer_leave: WLAN_NOTIFICATION_MSM = 12;
pub const wlan_notification_msm_radio_state_change: WLAN_NOTIFICATION_MSM = 7;
pub const wlan_notification_msm_roaming_end: WLAN_NOTIFICATION_MSM = 6;
pub const wlan_notification_msm_roaming_start: WLAN_NOTIFICATION_MSM = 5;
pub const wlan_notification_msm_signal_quality_change: WLAN_NOTIFICATION_MSM = 8;
pub const wlan_notification_msm_start: WLAN_NOTIFICATION_MSM = 0;
pub const wlan_notification_security_end: WLAN_NOTIFICATION_SECURITY = 1;
pub const wlan_notification_security_start: WLAN_NOTIFICATION_SECURITY = 0;
pub const wlan_opcode_value_type_invalid: WLAN_OPCODE_VALUE_TYPE = 3;
pub const wlan_opcode_value_type_query_only: WLAN_OPCODE_VALUE_TYPE = 0;
pub const wlan_opcode_value_type_set_by_group_policy: WLAN_OPCODE_VALUE_TYPE = 1;
pub const wlan_opcode_value_type_set_by_user: WLAN_OPCODE_VALUE_TYPE = 2;
pub const wlan_operational_state_going_off: WLAN_OPERATIONAL_STATE = 3;
pub const wlan_operational_state_going_on: WLAN_OPERATIONAL_STATE = 4;
pub const wlan_operational_state_off: WLAN_OPERATIONAL_STATE = 1;
pub const wlan_operational_state_on: WLAN_OPERATIONAL_STATE = 2;
pub const wlan_operational_state_unknown: WLAN_OPERATIONAL_STATE = 0;
pub const wlan_power_setting_invalid: WLAN_POWER_SETTING = 4;
pub const wlan_power_setting_low_saving: WLAN_POWER_SETTING = 1;
pub const wlan_power_setting_maximum_saving: WLAN_POWER_SETTING = 3;
pub const wlan_power_setting_medium_saving: WLAN_POWER_SETTING = 2;
pub const wlan_power_setting_no_saving: WLAN_POWER_SETTING = 0;
pub const wlan_secure_ac_enabled: WLAN_SECURABLE_OBJECT = 2;
pub const wlan_secure_add_new_all_user_profiles: WLAN_SECURABLE_OBJECT = 9;
pub const wlan_secure_add_new_per_user_profiles: WLAN_SECURABLE_OBJECT = 10;
pub const wlan_secure_all_user_profiles_order: WLAN_SECURABLE_OBJECT = 8;
pub const wlan_secure_bc_scan_enabled: WLAN_SECURABLE_OBJECT = 3;
pub const wlan_secure_bss_type: WLAN_SECURABLE_OBJECT = 4;
pub const wlan_secure_current_operation_mode: WLAN_SECURABLE_OBJECT = 12;
pub const wlan_secure_deny_list: WLAN_SECURABLE_OBJECT = 1;
pub const wlan_secure_get_plaintext_key: WLAN_SECURABLE_OBJECT = 13;
pub const wlan_secure_hosted_network_elevated_access: WLAN_SECURABLE_OBJECT = 14;
pub const wlan_secure_ihv_control: WLAN_SECURABLE_OBJECT = 7;
pub const wlan_secure_interface_properties: WLAN_SECURABLE_OBJECT = 6;
pub const wlan_secure_media_streaming_mode_enabled: WLAN_SECURABLE_OBJECT = 11;
pub const wlan_secure_permit_list: WLAN_SECURABLE_OBJECT = 0;
pub const wlan_secure_show_denied: WLAN_SECURABLE_OBJECT = 5;
pub const wlan_secure_virtual_station_extensibility: WLAN_SECURABLE_OBJECT = 15;
pub const wlan_secure_wfd_elevated_access: WLAN_SECURABLE_OBJECT = 16;
