windows_link::link!("netshell.dll" "system" fn NcFreeNetconProperties(pprops : *mut NETCON_PROPERTIES));
windows_link::link!("netshell.dll" "system" fn NcIsValidConnectionName(pszwname : windows_sys::core::PCWSTR) -> windows_sys::core::BOOL);
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername : windows_sys::core::PCWSTR, netisoerror : *mut NETISO_ERROR_TYPE) -> u32);
#[cfg(feature = "Win32_Security")]
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationEnumAppContainers(flags : u32, pdwnumpublicappcs : *mut u32, pppublicappcs : *mut *mut INET_FIREWALL_APP_CONTAINER) -> u32);
#[cfg(feature = "Win32_System_Ole")]
windows_link::link!("firewallapi.dll" "system" fn NetworkIsolationEnumerateAppContainerRules(newenum : *mut *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(feature = "Win32_Security")]
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationFreeAppContainers(ppublicappcs : *const INET_FIREWALL_APP_CONTAINER) -> u32);
#[cfg(feature = "Win32_Security")]
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs : *mut u32, appcontainersids : *mut *mut super::super::Security::SID_AND_ATTRIBUTES) -> u32);
windows_link::link!("firewallapi.dll" "system" fn NetworkIsolationGetEnterpriseIdAsync(wszservername : windows_sys::core::PCWSTR, dwflags : u32, context : *const core::ffi::c_void, callback : PNETISO_EDP_ID_CALLBACK_FN, hoperation : *mut super::super::Foundation::HANDLE) -> u32);
windows_link::link!("firewallapi.dll" "system" fn NetworkIsolationGetEnterpriseIdClose(hoperation : super::super::Foundation::HANDLE, bwaitforoperation : windows_sys::core::BOOL) -> u32);
#[cfg(feature = "Win32_Security")]
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationRegisterForAppContainerChanges(flags : u32, callback : PAC_CHANGES_CALLBACK_FN, context : *const core::ffi::c_void, registrationobject : *mut super::super::Foundation::HANDLE) -> u32);
#[cfg(feature = "Win32_Security")]
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs : u32, appcontainersids : *const super::super::Security::SID_AND_ATTRIBUTES) -> u32);
#[cfg(feature = "Win32_Security")]
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationSetupAppContainerBinaries(applicationcontainersid : super::super::Security::PSID, packagefullname : windows_sys::core::PCWSTR, packagefolder : windows_sys::core::PCWSTR, displayname : windows_sys::core::PCWSTR, bbinariesfullycomputed : windows_sys::core::BOOL, binaries : *const windows_sys::core::PCWSTR, binariescount : u32) -> windows_sys::core::HRESULT);
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject : super::super::Foundation::HANDLE) -> u32);
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS0 {
    pub id: windows_sys::core::GUID,
    pub keyword: windows_sys::core::PCWSTR,
    pub flags: u32,
    pub addresses: windows_sys::core::PCWSTR,
}
impl Default for FW_DYNAMIC_KEYWORD_ADDRESS0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    pub dynamicKeywordAddress: FW_DYNAMIC_KEYWORD_ADDRESS0,
    pub next: *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0,
    pub schemaVersion: u16,
    pub originType: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE,
}
impl Default for FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = i32;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_ALL: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 3;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 1;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_NON_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 2;
pub type FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = i32;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = 1;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_INVALID: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 0;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_LOCAL: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 1;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_MDM: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 2;
pub type FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = i32;
pub const ICSSC_DEFAULT: SHARINGCONNECTION_ENUM_FLAGS = 0;
pub const ICSSC_ENABLED: SHARINGCONNECTION_ENUM_FLAGS = 1;
pub const ICSSHARINGTYPE_PRIVATE: SHARINGCONNECTIONTYPE = 1;
pub const ICSSHARINGTYPE_PUBLIC: SHARINGCONNECTIONTYPE = 0;
pub const ICSTT_IPADDRESS: ICS_TARGETTYPE = 1;
pub const ICSTT_NAME: ICS_TARGETTYPE = 0;
pub type ICS_TARGETTYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct INET_FIREWALL_AC_BINARIES {
    pub count: u32,
    pub binaries: *mut windows_sys::core::PWSTR,
}
impl Default for INET_FIREWALL_AC_BINARIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INET_FIREWALL_AC_BINARY: INET_FIREWALL_AC_CREATION_TYPE = 2;
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct INET_FIREWALL_AC_CAPABILITIES {
    pub count: u32,
    pub capabilities: *mut super::super::Security::SID_AND_ATTRIBUTES,
}
#[cfg(feature = "Win32_Security")]
impl Default for INET_FIREWALL_AC_CAPABILITIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct INET_FIREWALL_AC_CHANGE {
    pub changeType: INET_FIREWALL_AC_CHANGE_TYPE,
    pub createType: INET_FIREWALL_AC_CREATION_TYPE,
    pub appContainerSid: *mut super::super::Security::SID,
    pub userSid: *mut super::super::Security::SID,
    pub displayName: windows_sys::core::PWSTR,
    pub Anonymous: INET_FIREWALL_AC_CHANGE_0,
}
#[cfg(feature = "Win32_Security")]
impl Default for INET_FIREWALL_AC_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub union INET_FIREWALL_AC_CHANGE_0 {
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
}
#[cfg(feature = "Win32_Security")]
impl Default for INET_FIREWALL_AC_CHANGE_0 {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const INET_FIREWALL_AC_CHANGE_CREATE: INET_FIREWALL_AC_CHANGE_TYPE = 1;
pub const INET_FIREWALL_AC_CHANGE_DELETE: INET_FIREWALL_AC_CHANGE_TYPE = 2;
pub const INET_FIREWALL_AC_CHANGE_INVALID: INET_FIREWALL_AC_CHANGE_TYPE = 0;
pub const INET_FIREWALL_AC_CHANGE_MAX: INET_FIREWALL_AC_CHANGE_TYPE = 3;
pub type INET_FIREWALL_AC_CHANGE_TYPE = i32;
pub type INET_FIREWALL_AC_CREATION_TYPE = i32;
pub const INET_FIREWALL_AC_MAX: INET_FIREWALL_AC_CREATION_TYPE = 4;
pub const INET_FIREWALL_AC_NONE: INET_FIREWALL_AC_CREATION_TYPE = 0;
pub const INET_FIREWALL_AC_PACKAGE_ID_ONLY: INET_FIREWALL_AC_CREATION_TYPE = 1;
#[repr(C)]
#[cfg(feature = "Win32_Security")]
#[derive(Clone, Copy)]
pub struct INET_FIREWALL_APP_CONTAINER {
    pub appContainerSid: *mut super::super::Security::SID,
    pub userSid: *mut super::super::Security::SID,
    pub appContainerName: windows_sys::core::PWSTR,
    pub displayName: windows_sys::core::PWSTR,
    pub description: windows_sys::core::PWSTR,
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
    pub workingDirectory: windows_sys::core::PWSTR,
    pub packageFullName: windows_sys::core::PWSTR,
}
#[cfg(feature = "Win32_Security")]
impl Default for INET_FIREWALL_APP_CONTAINER {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const NCCF_ALLOW_DUPLICATION: NETCON_CHARACTERISTIC_FLAGS = 2;
pub const NCCF_ALLOW_REMOVAL: NETCON_CHARACTERISTIC_FLAGS = 4;
pub const NCCF_ALLOW_RENAME: NETCON_CHARACTERISTIC_FLAGS = 8;
pub const NCCF_ALL_USERS: NETCON_CHARACTERISTIC_FLAGS = 1;
pub const NCCF_BLUETOOTH_MASK: NETCON_CHARACTERISTIC_FLAGS = 983040;
pub const NCCF_BRANDED: NETCON_CHARACTERISTIC_FLAGS = 128;
pub const NCCF_BRIDGED: NETCON_CHARACTERISTIC_FLAGS = 512;
pub const NCCF_DEFAULT: NETCON_CHARACTERISTIC_FLAGS = 2048;
pub const NCCF_FIREWALLED: NETCON_CHARACTERISTIC_FLAGS = 1024;
pub const NCCF_HOMENET_CAPABLE: NETCON_CHARACTERISTIC_FLAGS = 4096;
pub const NCCF_HOSTED_NETWORK: NETCON_CHARACTERISTIC_FLAGS = 65536;
pub const NCCF_INCOMING_ONLY: NETCON_CHARACTERISTIC_FLAGS = 32;
pub const NCCF_LAN_MASK: NETCON_CHARACTERISTIC_FLAGS = 15728640;
pub const NCCF_NONE: NETCON_CHARACTERISTIC_FLAGS = 0;
pub const NCCF_OUTGOING_ONLY: NETCON_CHARACTERISTIC_FLAGS = 64;
pub const NCCF_QUARANTINED: NETCON_CHARACTERISTIC_FLAGS = 16384;
pub const NCCF_RESERVED: NETCON_CHARACTERISTIC_FLAGS = 32768;
pub const NCCF_SHARED: NETCON_CHARACTERISTIC_FLAGS = 256;
pub const NCCF_SHARED_PRIVATE: NETCON_CHARACTERISTIC_FLAGS = 8192;
pub const NCCF_VIRTUAL_STATION: NETCON_CHARACTERISTIC_FLAGS = 131072;
pub const NCCF_WIFI_DIRECT: NETCON_CHARACTERISTIC_FLAGS = 262144;
pub const NCME_DEFAULT: NETCONMGR_ENUM_FLAGS = 0;
pub const NCME_HIDDEN: NETCONMGR_ENUM_FLAGS = 1;
pub const NCM_BRIDGE: NETCON_MEDIATYPE = 7;
pub const NCM_DIRECT: NETCON_MEDIATYPE = 1;
pub const NCM_ISDN: NETCON_MEDIATYPE = 2;
pub const NCM_LAN: NETCON_MEDIATYPE = 3;
pub const NCM_NONE: NETCON_MEDIATYPE = 0;
pub const NCM_PHONE: NETCON_MEDIATYPE = 4;
pub const NCM_PPPOE: NETCON_MEDIATYPE = 6;
pub const NCM_SHAREDACCESSHOST_LAN: NETCON_MEDIATYPE = 8;
pub const NCM_SHAREDACCESSHOST_RAS: NETCON_MEDIATYPE = 9;
pub const NCM_TUNNEL: NETCON_MEDIATYPE = 5;
pub const NCS_ACTION_REQUIRED: NETCON_STATUS = 13;
pub const NCS_ACTION_REQUIRED_RETRY: NETCON_STATUS = 14;
pub const NCS_AUTHENTICATING: NETCON_STATUS = 8;
pub const NCS_AUTHENTICATION_FAILED: NETCON_STATUS = 10;
pub const NCS_AUTHENTICATION_SUCCEEDED: NETCON_STATUS = 9;
pub const NCS_CONNECTED: NETCON_STATUS = 2;
pub const NCS_CONNECTING: NETCON_STATUS = 1;
pub const NCS_CONNECT_FAILED: NETCON_STATUS = 15;
pub const NCS_CREDENTIALS_REQUIRED: NETCON_STATUS = 12;
pub const NCS_DISCONNECTED: NETCON_STATUS = 0;
pub const NCS_DISCONNECTING: NETCON_STATUS = 3;
pub const NCS_HARDWARE_DISABLED: NETCON_STATUS = 5;
pub const NCS_HARDWARE_MALFUNCTION: NETCON_STATUS = 6;
pub const NCS_HARDWARE_NOT_PRESENT: NETCON_STATUS = 4;
pub const NCS_INVALID_ADDRESS: NETCON_STATUS = 11;
pub const NCS_MEDIA_DISCONNECTED: NETCON_STATUS = 7;
pub const NCT_BRIDGE: NETCON_TYPE = 6;
pub const NCT_DIRECT_CONNECT: NETCON_TYPE = 0;
pub const NCT_INBOUND: NETCON_TYPE = 1;
pub const NCT_INTERNET: NETCON_TYPE = 2;
pub const NCT_LAN: NETCON_TYPE = 3;
pub const NCT_PHONE: NETCON_TYPE = 4;
pub const NCT_TUNNEL: NETCON_TYPE = 5;
pub const NCUC_DEFAULT: NETCONUI_CONNECT_FLAGS = 0;
pub const NCUC_ENABLE_DISABLE: NETCONUI_CONNECT_FLAGS = 2;
pub const NCUC_NO_UI: NETCONUI_CONNECT_FLAGS = 1;
pub type NETCONMGR_ENUM_FLAGS = i32;
pub type NETCONUI_CONNECT_FLAGS = i32;
pub type NETCON_CHARACTERISTIC_FLAGS = i32;
pub const NETCON_MAX_NAME_LEN: u32 = 256;
pub type NETCON_MEDIATYPE = i32;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct NETCON_PROPERTIES {
    pub guidId: windows_sys::core::GUID,
    pub pszwName: windows_sys::core::PWSTR,
    pub pszwDeviceName: windows_sys::core::PWSTR,
    pub Status: NETCON_STATUS,
    pub MediaType: NETCON_MEDIATYPE,
    pub dwCharacter: u32,
    pub clsidThisObject: windows_sys::core::GUID,
    pub clsidUiObject: windows_sys::core::GUID,
}
impl Default for NETCON_PROPERTIES {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type NETCON_STATUS = i32;
pub type NETCON_TYPE = i32;
pub type NETISO_ERROR_TYPE = i32;
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT: NETISO_ERROR_TYPE = 2;
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT_SERVER: NETISO_ERROR_TYPE = 3;
pub const NETISO_ERROR_TYPE_MAX: NETISO_ERROR_TYPE = 4;
pub const NETISO_ERROR_TYPE_NONE: NETISO_ERROR_TYPE = 0;
pub const NETISO_ERROR_TYPE_PRIVATE_NETWORK: NETISO_ERROR_TYPE = 1;
pub type NETISO_FLAG = i32;
pub const NETISO_FLAG_FORCE_COMPUTE_BINARIES: NETISO_FLAG = 1;
pub const NETISO_FLAG_MAX: NETISO_FLAG = 2;
pub const NETISO_GEID_FOR_NEUTRAL_AWARE: u32 = 2;
pub const NETISO_GEID_FOR_WDAG: u32 = 1;
pub type NET_FW_ACTION = i32;
pub const NET_FW_ACTION_ALLOW: NET_FW_ACTION = 1;
pub const NET_FW_ACTION_BLOCK: NET_FW_ACTION = 0;
pub const NET_FW_ACTION_MAX: NET_FW_ACTION = 2;
pub const NET_FW_AUTHENTICATE_AND_ENCRYPT: NET_FW_AUTHENTICATE_TYPE = 4;
pub const NET_FW_AUTHENTICATE_AND_NEGOTIATE_ENCRYPTION: NET_FW_AUTHENTICATE_TYPE = 3;
pub const NET_FW_AUTHENTICATE_NONE: NET_FW_AUTHENTICATE_TYPE = 0;
pub const NET_FW_AUTHENTICATE_NO_ENCAPSULATION: NET_FW_AUTHENTICATE_TYPE = 1;
pub type NET_FW_AUTHENTICATE_TYPE = i32;
pub const NET_FW_AUTHENTICATE_WITH_INTEGRITY: NET_FW_AUTHENTICATE_TYPE = 2;
pub type NET_FW_EDGE_TRAVERSAL_TYPE = i32;
pub const NET_FW_EDGE_TRAVERSAL_TYPE_ALLOW: NET_FW_EDGE_TRAVERSAL_TYPE = 1;
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_APP: NET_FW_EDGE_TRAVERSAL_TYPE = 2;
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DEFER_TO_USER: NET_FW_EDGE_TRAVERSAL_TYPE = 3;
pub const NET_FW_EDGE_TRAVERSAL_TYPE_DENY: NET_FW_EDGE_TRAVERSAL_TYPE = 0;
pub type NET_FW_IP_PROTOCOL = i32;
pub const NET_FW_IP_PROTOCOL_ANY: NET_FW_IP_PROTOCOL = 256;
pub const NET_FW_IP_PROTOCOL_TCP: NET_FW_IP_PROTOCOL = 6;
pub const NET_FW_IP_PROTOCOL_UDP: NET_FW_IP_PROTOCOL = 17;
pub type NET_FW_IP_VERSION = i32;
pub const NET_FW_IP_VERSION_ANY: NET_FW_IP_VERSION = 2;
pub const NET_FW_IP_VERSION_MAX: NET_FW_IP_VERSION = 3;
pub const NET_FW_IP_VERSION_V4: NET_FW_IP_VERSION = 0;
pub const NET_FW_IP_VERSION_V6: NET_FW_IP_VERSION = 1;
pub type NET_FW_MODIFY_STATE = i32;
pub const NET_FW_MODIFY_STATE_GP_OVERRIDE: NET_FW_MODIFY_STATE = 1;
pub const NET_FW_MODIFY_STATE_INBOUND_BLOCKED: NET_FW_MODIFY_STATE = 2;
pub const NET_FW_MODIFY_STATE_OK: NET_FW_MODIFY_STATE = 0;
pub const NET_FW_POLICY_EFFECTIVE: NET_FW_POLICY_TYPE = 2;
pub const NET_FW_POLICY_GROUP: NET_FW_POLICY_TYPE = 0;
pub const NET_FW_POLICY_LOCAL: NET_FW_POLICY_TYPE = 1;
pub type NET_FW_POLICY_TYPE = i32;
pub const NET_FW_POLICY_TYPE_MAX: NET_FW_POLICY_TYPE = 3;
pub const NET_FW_PROFILE2_ALL: NET_FW_PROFILE_TYPE2 = 2147483647;
pub const NET_FW_PROFILE2_DOMAIN: NET_FW_PROFILE_TYPE2 = 1;
pub const NET_FW_PROFILE2_PRIVATE: NET_FW_PROFILE_TYPE2 = 2;
pub const NET_FW_PROFILE2_PUBLIC: NET_FW_PROFILE_TYPE2 = 4;
pub const NET_FW_PROFILE_CURRENT: NET_FW_PROFILE_TYPE = 2;
pub const NET_FW_PROFILE_DOMAIN: NET_FW_PROFILE_TYPE = 0;
pub const NET_FW_PROFILE_STANDARD: NET_FW_PROFILE_TYPE = 1;
pub type NET_FW_PROFILE_TYPE = i32;
pub type NET_FW_PROFILE_TYPE2 = i32;
pub const NET_FW_PROFILE_TYPE_MAX: NET_FW_PROFILE_TYPE = 3;
pub type NET_FW_RULE_CATEGORY = i32;
pub const NET_FW_RULE_CATEGORY_BOOT: NET_FW_RULE_CATEGORY = 0;
pub const NET_FW_RULE_CATEGORY_CONSEC: NET_FW_RULE_CATEGORY = 3;
pub const NET_FW_RULE_CATEGORY_FIREWALL: NET_FW_RULE_CATEGORY = 2;
pub const NET_FW_RULE_CATEGORY_MAX: NET_FW_RULE_CATEGORY = 4;
pub const NET_FW_RULE_CATEGORY_STEALTH: NET_FW_RULE_CATEGORY = 1;
pub type NET_FW_RULE_DIRECTION = i32;
pub const NET_FW_RULE_DIR_IN: NET_FW_RULE_DIRECTION = 1;
pub const NET_FW_RULE_DIR_MAX: NET_FW_RULE_DIRECTION = 3;
pub const NET_FW_RULE_DIR_OUT: NET_FW_RULE_DIRECTION = 2;
pub type NET_FW_SCOPE = i32;
pub const NET_FW_SCOPE_ALL: NET_FW_SCOPE = 0;
pub const NET_FW_SCOPE_CUSTOM: NET_FW_SCOPE = 2;
pub const NET_FW_SCOPE_LOCAL_SUBNET: NET_FW_SCOPE = 1;
pub const NET_FW_SCOPE_MAX: NET_FW_SCOPE = 3;
pub const NET_FW_SERVICE_FILE_AND_PRINT: NET_FW_SERVICE_TYPE = 0;
pub const NET_FW_SERVICE_NONE: NET_FW_SERVICE_TYPE = 3;
pub const NET_FW_SERVICE_REMOTE_DESKTOP: NET_FW_SERVICE_TYPE = 2;
pub type NET_FW_SERVICE_TYPE = i32;
pub const NET_FW_SERVICE_TYPE_MAX: NET_FW_SERVICE_TYPE = 4;
pub const NET_FW_SERVICE_UPNP: NET_FW_SERVICE_TYPE = 1;
pub const NetFwAuthorizedApplication: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xec9846b3_2762_4a6b_a214_6acb603462d2);
pub const NetFwMgr: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x304ce942_6e39_40d8_943a_b913c40c9cd4);
pub const NetFwOpenPort: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0ca545c6_37ad_4a6c_bf92_9f7610067ef5);
pub const NetFwPolicy2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe2b3c97f_6ae1_41ac_817a_f6f92166d7dd);
pub const NetFwProduct: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9d745ed8_c514_4d1d_bf42_751fed2d5ac7);
pub const NetFwProducts: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcc19079b_8272_4d73_bb70_cdb533527b61);
pub const NetFwRule: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2c5bc43e_3369_4c33_ab0c_be9469677af4);
pub const NetSharingManager: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x5c63c1ad_3956_4ff8_8486_40034758315b);
#[cfg(feature = "Win32_Security")]
pub type PAC_CHANGES_CALLBACK_FN = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, pchange: *const INET_FIREWALL_AC_CHANGE)>;
pub type PFN_FWADDDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddress: *const FW_DYNAMIC_KEYWORD_ADDRESS0) -> u32>;
pub type PFN_FWDELETEDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_sys::core::GUID) -> u32>;
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_sys::core::GUID, dynamickeywordaddressdata: *mut *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0 = Option<unsafe extern "system" fn(flags: u32, dynamickeywordaddressdata: *mut *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0 = Option<unsafe extern "system" fn(dynamickeywordaddressdata: *const FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWUPDATEDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_sys::core::GUID, updatedaddresses: windows_sys::core::PCWSTR, append: windows_sys::core::BOOL) -> u32>;
pub type PNETISO_EDP_ID_CALLBACK_FN = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, wszenterpriseid: windows_sys::core::PCWSTR, dwerr: u32)>;
pub type SHARINGCONNECTIONTYPE = i32;
pub type SHARINGCONNECTION_ENUM_FLAGS = i32;
pub const S_OBJECT_NO_LONGER_VALID: windows_sys::core::HRESULT = 0x2_u32 as _;
pub const UPnPNAT: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xae1e00aa_3fd5_403c_8a27_2bbdc30cd0e1);
