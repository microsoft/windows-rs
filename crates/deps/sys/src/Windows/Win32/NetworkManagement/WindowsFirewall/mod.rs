#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername: super::super::Foundation::PWSTR, netisoerror: *mut NETISO_ERROR_TYPE) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationEnumAppContainers(flags: u32, pdwnumpublicappcs: *mut u32, pppublicappcs: *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationFreeAppContainers(ppublicappcs: *const INET_FIREWALL_APP_CONTAINER) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs: *mut u32, appcontainersids: *mut *mut super::super::Security::SID_AND_ATTRIBUTES) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationRegisterForAppContainerChanges(flags: u32, callback: PAC_CHANGES_CALLBACK_FN, context: *const ::core::ffi::c_void, registrationobject: *mut super::super::Foundation::HANDLE) -> u32;
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs: u32, appcontainersids: *const super::super::Security::SID_AND_ATTRIBUTES) -> u32;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetworkIsolationSetupAppContainerBinaries(applicationcontainersid: super::super::Foundation::PSID, packagefullname: super::super::Foundation::PWSTR, packagefolder: super::super::Foundation::PWSTR, displayname: super::super::Foundation::PWSTR, bbinariesfullycomputed: super::super::Foundation::BOOL, binaries: *const super::super::Foundation::PWSTR, binariescount: u32) -> ::windows_sys::core::HRESULT;
    #[cfg(feature = "Win32_Foundation")]
    pub fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject: super::super::Foundation::HANDLE) -> u32;
}
#[repr(C)]
pub struct ICS_TARGETTYPE(i32);
#[repr(transparent)]
pub struct IDynamicPortMapping(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IDynamicPortMappingCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumNetConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumNetSharingEveryConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumNetSharingPortMapping(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumNetSharingPrivateConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IEnumNetSharingPublicConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INATEventManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INATExternalIPAddressCallback(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INATNumberOfEntriesCallback(pub *mut ::core::ffi::c_void);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct INET_FIREWALL_AC_BINARIES(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct INET_FIREWALL_AC_CAPABILITIES(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct INET_FIREWALL_AC_CHANGE(i32);
#[repr(C)]
pub struct INET_FIREWALL_AC_CHANGE_TYPE(i32);
#[repr(C)]
pub struct INET_FIREWALL_AC_CREATION_TYPE(i32);
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[repr(C)]
pub struct INET_FIREWALL_APP_CONTAINER(i32);
#[repr(transparent)]
pub struct INetConnection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetConnectionConnectUi(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetConnectionManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetConnectionProps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwAuthorizedApplication(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwAuthorizedApplications(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwIcmpSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwMgr(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwOpenPort(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwOpenPorts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwPolicy(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwPolicy2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwProduct(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwProducts(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwProfile(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwRemoteAdminSettings(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwRule(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwRule2(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwRule3(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwRules(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwService(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwServiceRestriction(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetFwServices(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetSharingConfiguration(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetSharingEveryConnectionCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetSharingManager(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetSharingPortMapping(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetSharingPortMappingCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetSharingPortMappingProps(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetSharingPrivateConnectionCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct INetSharingPublicConnectionCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStaticPortMapping(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IStaticPortMappingCollection(pub *mut ::core::ffi::c_void);
#[repr(transparent)]
pub struct IUPnPNAT(pub *mut ::core::ffi::c_void);
#[repr(C)]
pub struct NETCONMGR_ENUM_FLAGS(i32);
#[repr(C)]
pub struct NETCONUI_CONNECT_FLAGS(i32);
#[repr(C)]
pub struct NETCON_CHARACTERISTIC_FLAGS(i32);
pub const NETCON_MAX_NAME_LEN: u32 = 256u32;
#[repr(C)]
pub struct NETCON_MEDIATYPE(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct NETCON_PROPERTIES(i32);
#[repr(C)]
pub struct NETCON_STATUS(i32);
#[repr(C)]
pub struct NETCON_TYPE(i32);
#[repr(C)]
pub struct NETISO_ERROR_TYPE(i32);
#[repr(C)]
pub struct NETISO_FLAG(i32);
pub const NETISO_GEID_FOR_NEUTRAL_AWARE: u32 = 2u32;
pub const NETISO_GEID_FOR_WDAG: u32 = 1u32;
#[repr(C)]
pub struct NET_FW_ACTION(i32);
#[repr(C)]
pub struct NET_FW_AUTHENTICATE_TYPE(i32);
#[repr(C)]
pub struct NET_FW_EDGE_TRAVERSAL_TYPE(i32);
#[repr(C)]
pub struct NET_FW_IP_PROTOCOL(i32);
#[repr(C)]
pub struct NET_FW_IP_VERSION(i32);
#[repr(C)]
pub struct NET_FW_MODIFY_STATE(i32);
#[repr(C)]
pub struct NET_FW_POLICY_TYPE(i32);
#[repr(C)]
pub struct NET_FW_PROFILE_TYPE(i32);
#[repr(C)]
pub struct NET_FW_PROFILE_TYPE2(i32);
#[repr(C)]
pub struct NET_FW_RULE_CATEGORY(i32);
#[repr(C)]
pub struct NET_FW_RULE_DIRECTION(i32);
#[repr(C)]
pub struct NET_FW_SCOPE(i32);
#[repr(C)]
pub struct NET_FW_SERVICE_TYPE(i32);
#[repr(C)]
pub struct NetFwAuthorizedApplication(i32);
#[repr(C)]
pub struct NetFwMgr(i32);
#[repr(C)]
pub struct NetFwOpenPort(i32);
#[repr(C)]
pub struct NetFwPolicy2(i32);
#[repr(C)]
pub struct NetFwProduct(i32);
#[repr(C)]
pub struct NetFwProducts(i32);
#[repr(C)]
pub struct NetFwRule(i32);
#[repr(C)]
pub struct NetSharingManager(i32);
#[repr(C)]
pub struct PAC_CHANGES_CALLBACK_FN(i32);
#[repr(C)]
pub struct PFN_FWADDDYNAMICKEYWORDADDRESS0(i32);
#[repr(C)]
pub struct PFN_FWDELETEDYNAMICKEYWORDADDRESS0(i32);
#[repr(C)]
pub struct PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0(i32);
#[repr(C)]
pub struct PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0(i32);
#[repr(C)]
pub struct PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0(i32);
#[repr(C)]
pub struct PFN_FWUPDATEDYNAMICKEYWORDADDRESS0(i32);
#[repr(C)]
pub struct PNETISO_EDP_ID_CALLBACK_FN(i32);
#[repr(C)]
pub struct SHARINGCONNECTIONTYPE(i32);
#[repr(C)]
pub struct SHARINGCONNECTION_ENUM_FLAGS(i32);
pub const S_OBJECT_NO_LONGER_VALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2i32 as _);
#[repr(C)]
pub struct UPnPNAT(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS0(i32);
#[cfg(feature = "Win32_Foundation")]
#[repr(C)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0(i32);
#[repr(C)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS(i32);
#[repr(C)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS(i32);
#[repr(C)]
pub struct _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE(i32);
