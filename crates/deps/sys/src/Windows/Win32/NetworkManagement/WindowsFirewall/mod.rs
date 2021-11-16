#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
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
pub const ICSTT_NAME: i32 = 0i32;
pub const ICSTT_IPADDRESS: i32 = 1i32;
#[repr(transparent)]
pub struct IDynamicPortMapping(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDynamicPortMapping {}
impl ::core::clone::Clone for IDynamicPortMapping {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IDynamicPortMappingCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IDynamicPortMappingCollection {}
impl ::core::clone::Clone for IDynamicPortMappingCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumNetConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumNetConnection {}
impl ::core::clone::Clone for IEnumNetConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumNetSharingEveryConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumNetSharingEveryConnection {}
impl ::core::clone::Clone for IEnumNetSharingEveryConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumNetSharingPortMapping(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumNetSharingPortMapping {}
impl ::core::clone::Clone for IEnumNetSharingPortMapping {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumNetSharingPrivateConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumNetSharingPrivateConnection {}
impl ::core::clone::Clone for IEnumNetSharingPrivateConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IEnumNetSharingPublicConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IEnumNetSharingPublicConnection {}
impl ::core::clone::Clone for IEnumNetSharingPublicConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INATEventManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INATEventManager {}
impl ::core::clone::Clone for INATEventManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INATExternalIPAddressCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INATExternalIPAddressCallback {}
impl ::core::clone::Clone for INATExternalIPAddressCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INATNumberOfEntriesCallback(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INATNumberOfEntriesCallback {}
impl ::core::clone::Clone for INATNumberOfEntriesCallback {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct INET_FIREWALL_AC_BINARIES {
    pub count: u32,
    pub binaries: *mut super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for INET_FIREWALL_AC_BINARIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for INET_FIREWALL_AC_BINARIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct INET_FIREWALL_AC_CAPABILITIES {
    pub count: u32,
    pub capabilities: *mut super::super::Security::SID_AND_ATTRIBUTES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for INET_FIREWALL_AC_CAPABILITIES {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_AC_CAPABILITIES {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct INET_FIREWALL_AC_CHANGE {
    pub changeType: INET_FIREWALL_AC_CHANGE_TYPE,
    pub createType: INET_FIREWALL_AC_CREATION_TYPE,
    pub appContainerSid: *mut super::super::Security::SID,
    pub userSid: *mut super::super::Security::SID,
    pub displayName: super::super::Foundation::PWSTR,
    pub Anonymous: INET_FIREWALL_AC_CHANGE_0,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for INET_FIREWALL_AC_CHANGE {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_AC_CHANGE {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub union INET_FIREWALL_AC_CHANGE_0 {
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for INET_FIREWALL_AC_CHANGE_0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_AC_CHANGE_0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const INET_FIREWALL_AC_CHANGE_INVALID: i32 = 0i32;
pub const INET_FIREWALL_AC_CHANGE_CREATE: i32 = 1i32;
pub const INET_FIREWALL_AC_CHANGE_DELETE: i32 = 2i32;
pub const INET_FIREWALL_AC_CHANGE_MAX: i32 = 3i32;
pub const INET_FIREWALL_AC_NONE: i32 = 0i32;
pub const INET_FIREWALL_AC_PACKAGE_ID_ONLY: i32 = 1i32;
pub const INET_FIREWALL_AC_BINARY: i32 = 2i32;
pub const INET_FIREWALL_AC_MAX: i32 = 4i32;
#[repr(C)]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct INET_FIREWALL_APP_CONTAINER {
    pub appContainerSid: *mut super::super::Security::SID,
    pub userSid: *mut super::super::Security::SID,
    pub appContainerName: super::super::Foundation::PWSTR,
    pub displayName: super::super::Foundation::PWSTR,
    pub description: super::super::Foundation::PWSTR,
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
    pub workingDirectory: super::super::Foundation::PWSTR,
    pub packageFullName: super::super::Foundation::PWSTR,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for INET_FIREWALL_APP_CONTAINER {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for INET_FIREWALL_APP_CONTAINER {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetConnection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetConnection {}
impl ::core::clone::Clone for INetConnection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetConnectionConnectUi(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetConnectionConnectUi {}
impl ::core::clone::Clone for INetConnectionConnectUi {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetConnectionManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetConnectionManager {}
impl ::core::clone::Clone for INetConnectionManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetConnectionProps(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetConnectionProps {}
impl ::core::clone::Clone for INetConnectionProps {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwAuthorizedApplication(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwAuthorizedApplication {}
impl ::core::clone::Clone for INetFwAuthorizedApplication {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwAuthorizedApplications(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwAuthorizedApplications {}
impl ::core::clone::Clone for INetFwAuthorizedApplications {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwIcmpSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwIcmpSettings {}
impl ::core::clone::Clone for INetFwIcmpSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwMgr(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwMgr {}
impl ::core::clone::Clone for INetFwMgr {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwOpenPort(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwOpenPort {}
impl ::core::clone::Clone for INetFwOpenPort {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwOpenPorts(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwOpenPorts {}
impl ::core::clone::Clone for INetFwOpenPorts {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwPolicy(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwPolicy {}
impl ::core::clone::Clone for INetFwPolicy {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwPolicy2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwPolicy2 {}
impl ::core::clone::Clone for INetFwPolicy2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwProduct(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwProduct {}
impl ::core::clone::Clone for INetFwProduct {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwProducts(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwProducts {}
impl ::core::clone::Clone for INetFwProducts {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwProfile(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwProfile {}
impl ::core::clone::Clone for INetFwProfile {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwRemoteAdminSettings(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwRemoteAdminSettings {}
impl ::core::clone::Clone for INetFwRemoteAdminSettings {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwRule(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwRule {}
impl ::core::clone::Clone for INetFwRule {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwRule2(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwRule2 {}
impl ::core::clone::Clone for INetFwRule2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwRule3(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwRule3 {}
impl ::core::clone::Clone for INetFwRule3 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwRules(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwRules {}
impl ::core::clone::Clone for INetFwRules {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwService(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwService {}
impl ::core::clone::Clone for INetFwService {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwServiceRestriction(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwServiceRestriction {}
impl ::core::clone::Clone for INetFwServiceRestriction {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetFwServices(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetFwServices {}
impl ::core::clone::Clone for INetFwServices {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetSharingConfiguration(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetSharingConfiguration {}
impl ::core::clone::Clone for INetSharingConfiguration {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetSharingEveryConnectionCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetSharingEveryConnectionCollection {}
impl ::core::clone::Clone for INetSharingEveryConnectionCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetSharingManager(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetSharingManager {}
impl ::core::clone::Clone for INetSharingManager {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetSharingPortMapping(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetSharingPortMapping {}
impl ::core::clone::Clone for INetSharingPortMapping {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetSharingPortMappingCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetSharingPortMappingCollection {}
impl ::core::clone::Clone for INetSharingPortMappingCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetSharingPortMappingProps(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetSharingPortMappingProps {}
impl ::core::clone::Clone for INetSharingPortMappingProps {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetSharingPrivateConnectionCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetSharingPrivateConnectionCollection {}
impl ::core::clone::Clone for INetSharingPrivateConnectionCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct INetSharingPublicConnectionCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for INetSharingPublicConnectionCollection {}
impl ::core::clone::Clone for INetSharingPublicConnectionCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStaticPortMapping(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStaticPortMapping {}
impl ::core::clone::Clone for IStaticPortMapping {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IStaticPortMappingCollection(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IStaticPortMappingCollection {}
impl ::core::clone::Clone for IStaticPortMappingCollection {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(transparent)]
pub struct IUPnPNAT(pub *mut ::core::ffi::c_void);
impl ::core::marker::Copy for IUPnPNAT {}
impl ::core::clone::Clone for IUPnPNAT {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NCME_DEFAULT: i32 = 0i32;
pub const NCME_HIDDEN: i32 = 1i32;
pub const NCUC_DEFAULT: i32 = 0i32;
pub const NCUC_NO_UI: i32 = 1i32;
pub const NCUC_ENABLE_DISABLE: i32 = 2i32;
pub const NCCF_NONE: i32 = 0i32;
pub const NCCF_ALL_USERS: i32 = 1i32;
pub const NCCF_ALLOW_DUPLICATION: i32 = 2i32;
pub const NCCF_ALLOW_REMOVAL: i32 = 4i32;
pub const NCCF_ALLOW_RENAME: i32 = 8i32;
pub const NCCF_INCOMING_ONLY: i32 = 32i32;
pub const NCCF_OUTGOING_ONLY: i32 = 64i32;
pub const NCCF_BRANDED: i32 = 128i32;
pub const NCCF_SHARED: i32 = 256i32;
pub const NCCF_BRIDGED: i32 = 512i32;
pub const NCCF_FIREWALLED: i32 = 1024i32;
pub const NCCF_DEFAULT: i32 = 2048i32;
pub const NCCF_HOMENET_CAPABLE: i32 = 4096i32;
pub const NCCF_SHARED_PRIVATE: i32 = 8192i32;
pub const NCCF_QUARANTINED: i32 = 16384i32;
pub const NCCF_RESERVED: i32 = 32768i32;
pub const NCCF_HOSTED_NETWORK: i32 = 65536i32;
pub const NCCF_VIRTUAL_STATION: i32 = 131072i32;
pub const NCCF_WIFI_DIRECT: i32 = 262144i32;
pub const NCCF_BLUETOOTH_MASK: i32 = 983040i32;
pub const NCCF_LAN_MASK: i32 = 15728640i32;
pub const NETCON_MAX_NAME_LEN: u32 = 256u32;
pub const NCM_NONE: i32 = 0i32;
pub const NCM_DIRECT: i32 = 1i32;
pub const NCM_ISDN: i32 = 2i32;
pub const NCM_LAN: i32 = 3i32;
pub const NCM_PHONE: i32 = 4i32;
pub const NCM_TUNNEL: i32 = 5i32;
pub const NCM_PPPOE: i32 = 6i32;
pub const NCM_BRIDGE: i32 = 7i32;
pub const NCM_SHAREDACCESSHOST_LAN: i32 = 8i32;
pub const NCM_SHAREDACCESSHOST_RAS: i32 = 9i32;
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct NETCON_PROPERTIES {
    pub guidId: ::windows_sys::core::GUID,
    pub pszwName: super::super::Foundation::PWSTR,
    pub pszwDeviceName: super::super::Foundation::PWSTR,
    pub Status: NETCON_STATUS,
    pub MediaType: NETCON_MEDIATYPE,
    pub dwCharacter: u32,
    pub clsidThisObject: ::windows_sys::core::GUID,
    pub clsidUiObject: ::windows_sys::core::GUID,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NETCON_PROPERTIES {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NETCON_PROPERTIES {
    fn clone(&self) -> Self {
        *self
    }
}
pub const NCS_DISCONNECTED: i32 = 0i32;
pub const NCS_CONNECTING: i32 = 1i32;
pub const NCS_CONNECTED: i32 = 2i32;
pub const NCS_DISCONNECTING: i32 = 3i32;
pub const NCS_HARDWARE_NOT_PRESENT: i32 = 4i32;
pub const NCS_HARDWARE_DISABLED: i32 = 5i32;
pub const NCS_HARDWARE_MALFUNCTION: i32 = 6i32;
pub const NCS_MEDIA_DISCONNECTED: i32 = 7i32;
pub const NCS_AUTHENTICATING: i32 = 8i32;
pub const NCS_AUTHENTICATION_SUCCEEDED: i32 = 9i32;
pub const NCS_AUTHENTICATION_FAILED: i32 = 10i32;
pub const NCS_INVALID_ADDRESS: i32 = 11i32;
pub const NCS_CREDENTIALS_REQUIRED: i32 = 12i32;
pub const NCS_ACTION_REQUIRED: i32 = 13i32;
pub const NCS_ACTION_REQUIRED_RETRY: i32 = 14i32;
pub const NCS_CONNECT_FAILED: i32 = 15i32;
pub const NCT_DIRECT_CONNECT: i32 = 0i32;
pub const NCT_INBOUND: i32 = 1i32;
pub const NCT_INTERNET: i32 = 2i32;
pub const NCT_LAN: i32 = 3i32;
pub const NCT_PHONE: i32 = 4i32;
pub const NCT_TUNNEL: i32 = 5i32;
pub const NCT_BRIDGE: i32 = 6i32;
pub const NETISO_ERROR_TYPE_NONE: i32 = 0i32;
pub const NETISO_ERROR_TYPE_PRIVATE_NETWORK: i32 = 1i32;
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT: i32 = 2i32;
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT_SERVER: i32 = 3i32;
pub const NETISO_ERROR_TYPE_MAX: i32 = 4i32;
pub const NETISO_FLAG_FORCE_COMPUTE_BINARIES: i32 = 1i32;
pub const NETISO_FLAG_MAX: i32 = 2i32;
pub const NETISO_GEID_FOR_NEUTRAL_AWARE: u32 = 2u32;
pub const NETISO_GEID_FOR_WDAG: u32 = 1u32;
pub const NET_FW_ACTION_BLOCK: i32 = 0i32;
pub const NET_FW_ACTION_ALLOW: i32 = 1i32;
pub const NET_FW_ACTION_MAX: i32 = 2i32;
pub const NET_FW_AUTHENTICATE_NONE: i32 = 0i32;
pub const NET_FW_AUTHENTICATE_NO_ENCAPSULATION: i32 = 1i32;
pub const NET_FW_AUTHENTICATE_WITH_INTEGRITY: i32 = 2i32;
pub const NET_FW_AUTHENTICATE_AND_NEGOTIATE_ENCRYPTION: i32 = 3i32;
pub const NET_FW_AUTHENTICATE_AND_ENCRYPT: i32 = 4i32;
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DENY: i32 = 0i32;
pub const NET_FW_EDGE_TRAVERSAL_TYPE_ALLOW: i32 = 1i32;
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_APP: i32 = 2i32;
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_USER: i32 = 3i32;
pub const NET_FW_IP_PROTOCOL_TCP: i32 = 6i32;
pub const NET_FW_IP_PROTOCOL_UDP: i32 = 17i32;
pub const NET_FW_IP_PROTOCOL_ANY: i32 = 256i32;
pub const NET_FW_IP_VERSION_V4: i32 = 0i32;
pub const NET_FW_IP_VERSION_V6: i32 = 1i32;
pub const NET_FW_IP_VERSION_ANY: i32 = 2i32;
pub const NET_FW_IP_VERSION_MAX: i32 = 3i32;
pub const NET_FW_MODIFY_STATE_OK: i32 = 0i32;
pub const NET_FW_MODIFY_STATE_GP_OVERRIDE: i32 = 1i32;
pub const NET_FW_MODIFY_STATE_INBOUND_BLOCKED: i32 = 2i32;
pub const NET_FW_POLICY_GROUP: i32 = 0i32;
pub const NET_FW_POLICY_LOCAL: i32 = 1i32;
pub const NET_FW_POLICY_EFFECTIVE: i32 = 2i32;
pub const NET_FW_POLICY_TYPE_MAX: i32 = 3i32;
pub const NET_FW_PROFILE_DOMAIN: i32 = 0i32;
pub const NET_FW_PROFILE_STANDARD: i32 = 1i32;
pub const NET_FW_PROFILE_CURRENT: i32 = 2i32;
pub const NET_FW_PROFILE_TYPE_MAX: i32 = 3i32;
pub const NET_FW_PROFILE2_DOMAIN: i32 = 1i32;
pub const NET_FW_PROFILE2_PRIVATE: i32 = 2i32;
pub const NET_FW_PROFILE2_PUBLIC: i32 = 4i32;
pub const NET_FW_PROFILE2_ALL: i32 = 2147483647i32;
pub const NET_FW_RULE_CATEGORY_BOOT: i32 = 0i32;
pub const NET_FW_RULE_CATEGORY_STEALTH: i32 = 1i32;
pub const NET_FW_RULE_CATEGORY_FIREWALL: i32 = 2i32;
pub const NET_FW_RULE_CATEGORY_CONSEC: i32 = 3i32;
pub const NET_FW_RULE_CATEGORY_MAX: i32 = 4i32;
pub const NET_FW_RULE_DIR_IN: i32 = 1i32;
pub const NET_FW_RULE_DIR_OUT: i32 = 2i32;
pub const NET_FW_RULE_DIR_MAX: i32 = 3i32;
pub const NET_FW_SCOPE_ALL: i32 = 0i32;
pub const NET_FW_SCOPE_LOCAL_SUBNET: i32 = 1i32;
pub const NET_FW_SCOPE_CUSTOM: i32 = 2i32;
pub const NET_FW_SCOPE_MAX: i32 = 3i32;
pub const NET_FW_SERVICE_FILE_AND_PRINT: i32 = 0i32;
pub const NET_FW_SERVICE_UPNP: i32 = 1i32;
pub const NET_FW_SERVICE_REMOTE_DESKTOP: i32 = 2i32;
pub const NET_FW_SERVICE_NONE: i32 = 3i32;
pub const NET_FW_SERVICE_TYPE_MAX: i32 = 4i32;
pub const NetFwAuthorizedApplication: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 3969402547, data2: 10082, data3: 19051, data4: [162, 20, 106, 203, 96, 52, 98, 210] };
pub const NetFwMgr: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 810346818, data2: 28217, data3: 16600, data4: [148, 58, 185, 19, 196, 12, 156, 212] };
pub const NetFwOpenPort: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 212157894, data2: 14253, data3: 19052, data4: [191, 146, 159, 118, 16, 6, 126, 245] };
pub const NetFwPolicy2: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3803433343,
    data2: 27361,
    data3: 16812,
    data4: [129, 122, 246, 249, 33, 102, 215, 221],
};
pub const NetFwProduct: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 2641649368, data2: 50452, data3: 19741, data4: [191, 66, 117, 31, 237, 45, 90, 199] };
pub const NetFwProducts: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 3424192411,
    data2: 33394,
    data3: 19827,
    data4: [187, 112, 205, 181, 51, 82, 123, 97],
};
pub const NetFwRule: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 744211518,
    data2: 13161,
    data3: 19507,
    data4: [171, 12, 190, 148, 105, 103, 122, 244],
};
pub const NetSharingManager: ::windows_sys::core::GUID = ::windows_sys::core::GUID { data1: 1550041517, data2: 14678, data3: 20472, data4: [132, 134, 64, 3, 71, 88, 49, 91] };
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub type PAC_CHANGES_CALLBACK_FN = unsafe extern "system" fn(context: *const ::core::ffi::c_void, pchange: *const INET_FIREWALL_AC_CHANGE);
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWADDDYNAMICKEYWORDADDRESS0 = unsafe extern "system" fn(dynamickeywordaddress: *const _tag_FW_DYNAMIC_KEYWORD_ADDRESS0) -> u32;
pub type PFN_FWDELETEDYNAMICKEYWORDADDRESS0 = unsafe extern "system" fn(dynamickeywordaddressid: ::windows_sys::core::GUID) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0 = unsafe extern "system" fn(dynamickeywordaddressid: ::windows_sys::core::GUID, dynamickeywordaddressdata: *mut *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0 = unsafe extern "system" fn(flags: u32, dynamickeywordaddressdata: *mut *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0 = unsafe extern "system" fn(dynamickeywordaddressdata: *const _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PFN_FWUPDATEDYNAMICKEYWORDADDRESS0 = unsafe extern "system" fn(dynamickeywordaddressid: ::windows_sys::core::GUID, updatedaddresses: super::super::Foundation::PWSTR, append: super::super::Foundation::BOOL) -> u32;
#[cfg(feature = "Win32_Foundation")]
pub type PNETISO_EDP_ID_CALLBACK_FN = unsafe extern "system" fn(context: *mut ::core::ffi::c_void, wszenterpriseid: super::super::Foundation::PWSTR, dwerr: u32);
pub const ICSSHARINGTYPE_PUBLIC: i32 = 0i32;
pub const ICSSHARINGTYPE_PRIVATE: i32 = 1i32;
pub const ICSSC_DEFAULT: i32 = 0i32;
pub const ICSSC_ENABLED: i32 = 1i32;
pub const S_OBJECT_NO_LONGER_VALID: ::windows_sys::core::HRESULT = ::windows_sys::core::HRESULT(2i32 as _);
pub const UPnPNAT: ::windows_sys::core::GUID = ::windows_sys::core::GUID {
    data1: 2921201834,
    data2: 16341,
    data3: 16444,
    data4: [138, 39, 43, 189, 195, 12, 208, 225],
};
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    pub id: ::windows_sys::core::GUID,
    pub keyword: super::super::Foundation::PWSTR,
    pub flags: u32,
    pub addresses: super::super::Foundation::PWSTR,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _tag_FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Foundation")]
pub struct _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    pub dynamicKeywordAddress: _tag_FW_DYNAMIC_KEYWORD_ADDRESS0,
    pub next: *mut _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0,
    pub schemaVersion: u16,
    pub originType: _tag_FW_DYNAMIC_KEYWORD_ORIGIN_TYPE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for _tag_FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn clone(&self) -> Self {
        *self
    }
}
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_AUTO_RESOLVE: i32 = 1i32;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_NON_AUTO_RESOLVE: i32 = 2i32;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_ALL: i32 = 3i32;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS_AUTO_RESOLVE: i32 = 1i32;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_INVALID: i32 = 0i32;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_LOCAL: i32 = 1i32;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_MDM: i32 = 2i32;
