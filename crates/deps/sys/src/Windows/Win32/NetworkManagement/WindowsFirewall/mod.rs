#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername: super::super::Foundation::PWSTR, netisoerror: *mut NETISO_ERROR_TYPE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationEnumAppContainers(flags: u32, pdwnumpublicappcs: *mut u32, pppublicappcs: *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationFreeAppContainers(ppublicappcs: *const INET_FIREWALL_APP_CONTAINER) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs: *mut u32, appcontainersids: *mut *mut super::super::Security::SID_AND_ATTRIBUTES) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationRegisterForAppContainerChanges(flags: u32, callback: PAC_CHANGES_CALLBACK_FN, context: *const ::core::ffi::c_void, registrationobject: *mut super::super::Foundation::HANDLE) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs: u32, appcontainersids: *const super::super::Security::SID_AND_ATTRIBUTES) -> u32;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetworkIsolationSetupAppContainerBinaries(applicationcontainersid: super::super::Foundation::PSID, packagefullname: super::super::Foundation::PWSTR, packagefolder: super::super::Foundation::PWSTR, displayname: super::super::Foundation::PWSTR, bbinariesfullycomputed: super::super::Foundation::BOOL, binaries: *const super::super::Foundation::PWSTR, binariescount: u32) -> ::windows_sys::core::HRESULT;
    #[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject: super::super::Foundation::HANDLE) -> u32;
}
pub struct ICS_TARGETTYPE(i32);
pub struct IDynamicPortMapping(i32);
pub struct IDynamicPortMappingCollection(i32);
pub struct IEnumNetConnection(i32);
pub struct IEnumNetSharingEveryConnection(i32);
pub struct IEnumNetSharingPortMapping(i32);
pub struct IEnumNetSharingPrivateConnection(i32);
pub struct IEnumNetSharingPublicConnection(i32);
pub struct INATEventManager(i32);
pub struct INATExternalIPAddressCallback(i32);
pub struct INATNumberOfEntriesCallback(i32);
pub struct INET_FIREWALL_AC_BINARIES(i32);
pub struct INET_FIREWALL_AC_CAPABILITIES(i32);
pub struct INET_FIREWALL_AC_CHANGE(i32);
pub struct INET_FIREWALL_AC_CHANGE_TYPE(i32);
pub struct INET_FIREWALL_AC_CREATION_TYPE(i32);
pub struct INET_FIREWALL_APP_CONTAINER(i32);
pub struct INetConnection(i32);
pub struct INetConnectionConnectUi(i32);
pub struct INetConnectionManager(i32);
pub struct INetConnectionProps(i32);
pub struct INetFwAuthorizedApplication(i32);
pub struct INetFwAuthorizedApplications(i32);
pub struct INetFwIcmpSettings(i32);
pub struct INetFwMgr(i32);
pub struct INetFwOpenPort(i32);
pub struct INetFwOpenPorts(i32);
pub struct INetFwPolicy(i32);
pub struct INetFwPolicy2(i32);
pub struct INetFwProduct(i32);
pub struct INetFwProducts(i32);
pub struct INetFwProfile(i32);
pub struct INetFwRemoteAdminSettings(i32);
pub struct INetFwRule(i32);
pub struct INetFwRule2(i32);
pub struct INetFwRule3(i32);
pub struct INetFwRules(i32);
pub struct INetFwService(i32);
pub struct INetFwServiceRestriction(i32);
pub struct INetFwServices(i32);
pub struct INetSharingConfiguration(i32);
pub struct INetSharingEveryConnectionCollection(i32);
pub struct INetSharingManager(i32);
pub struct INetSharingPortMapping(i32);
pub struct INetSharingPortMappingCollection(i32);
pub struct INetSharingPortMappingProps(i32);
pub struct INetSharingPrivateConnectionCollection(i32);
pub struct INetSharingPublicConnectionCollection(i32);
pub struct IStaticPortMapping(i32);
pub struct IStaticPortMappingCollection(i32);
pub struct IUPnPNAT(i32);
pub struct NETCONMGR_ENUM_FLAGS(i32);
pub struct NETCONUI_CONNECT_FLAGS(i32);
pub struct NETCON_CHARACTERISTIC_FLAGS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
pub const NETCON_MAX_NAME_LEN: u32 = 256u32;
pub struct NETCON_MEDIATYPE(i32);
pub struct NETCON_PROPERTIES(i32);
pub struct NETCON_STATUS(i32);
pub struct NETCON_TYPE(i32);
pub struct NETISO_ERROR_TYPE(i32);
pub struct NETISO_FLAG(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
pub const NETISO_GEID_FOR_NEUTRAL_AWARE: u32 = 2u32;
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
pub const NETISO_GEID_FOR_WDAG: u32 = 1u32;
pub struct NET_FW_ACTION(i32);
pub struct NET_FW_AUTHENTICATE_TYPE(i32);
pub struct NET_FW_EDGE_TRAVERSAL_TYPE(i32);
pub struct NET_FW_IP_PROTOCOL(i32);
pub struct NET_FW_IP_VERSION(i32);
pub struct NET_FW_MODIFY_STATE(i32);
pub struct NET_FW_POLICY_TYPE(i32);
pub struct NET_FW_PROFILE_TYPE(i32);
pub struct NET_FW_PROFILE_TYPE2(i32);
pub struct NET_FW_RULE_CATEGORY(i32);
pub struct NET_FW_RULE_DIRECTION(i32);
pub struct NET_FW_SCOPE(i32);
pub struct NET_FW_SERVICE_TYPE(i32);
pub struct NetFwAuthorizedApplication(i32);
pub struct NetFwMgr(i32);
pub struct NetFwOpenPort(i32);
pub struct NetFwPolicy2(i32);
pub struct NetFwProduct(i32);
pub struct NetFwProducts(i32);
pub struct NetFwRule(i32);
pub struct NetSharingManager(i32);
pub struct PAC_CHANGES_CALLBACK_FN(i32);
pub struct PFN_FWADDDYNAMICKEYWORDADDRESS0(i32);
pub struct PFN_FWDELETEDYNAMICKEYWORDADDRESS0(i32);
pub struct PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0(i32);
pub struct PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0(i32);
pub struct PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0(i32);
pub struct PFN_FWUPDATEDYNAMICKEYWORDADDRESS0(i32);
pub struct PNETISO_EDP_ID_CALLBACK_FN(i32);
pub struct SHARINGCONNECTIONTYPE(i32);
pub struct SHARINGCONNECTION_ENUM_FLAGS(i32);
#[doc = "*Required features: `Win32_NetworkManagement_WindowsFirewall`*"]
pub const S_OBJECT_NO_LONGER_VALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2i32 as _);
pub struct UPnPNAT(i32);
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS0(i32);
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0(i32);
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(i32);
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS(i32);
pub struct _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(i32);
