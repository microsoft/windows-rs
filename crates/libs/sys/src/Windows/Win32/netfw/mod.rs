windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationDiagnoseConnectFailureAndGetInfo(wszservername : windows_sys::core::PCWSTR, netisoerror : *mut NETISO_ERROR_TYPE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationEnumAppContainers(flags : u32, pdwnumpublicappcs : *mut u32, pppublicappcs : *mut PINET_FIREWALL_APP_CONTAINER) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationFreeAppContainers(ppublicappcs : *const INET_FIREWALL_APP_CONTAINER) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationGetAppContainerConfig(pdwnumpublicappcs : *mut u32, appcontainersids : *mut super::PSID_AND_ATTRIBUTES) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationRegisterForAppContainerChanges(flags : u32, callback : PAC_CHANGES_CALLBACK_FN, context : *const core::ffi::c_void, registrationobject : *mut super::HANDLE) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationSetAppContainerConfig(dwnumpublicappcs : u32, appcontainersids : *const super::SID_AND_ATTRIBUTES) -> u32);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationSetupAppContainerBinaries(applicationcontainersid : super::PSID, packagefullname : windows_sys::core::PCWSTR, packagefolder : windows_sys::core::PCWSTR, displayname : windows_sys::core::PCWSTR, bbinariesfullycomputed : windows_sys::core::BOOL, binaries : *const windows_sys::core::PCWSTR, binariescount : u32) -> windows_sys::core::HRESULT);
#[cfg(feature = "winnt")]
windows_link::link!("api-ms-win-net-isolation-l1-1-0.dll" "system" fn NetworkIsolationUnregisterForAppContainerChanges(registrationobject : super::HANDLE) -> u32);
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS0 {
    pub id: windows_sys::core::GUID,
    pub keyword: windows_sys::core::PCWSTR,
    pub flags: u32,
    pub addresses: windows_sys::core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct FW_DYNAMIC_KEYWORD_ADDRESS_DATA0 {
    pub dynamicKeywordAddress: FW_DYNAMIC_KEYWORD_ADDRESS0,
    pub next: *mut Self,
    pub schemaVersion: u16,
    pub originType: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE,
}
pub type FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = u32;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_ALL: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 3;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 1;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS_NON_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_ENUM_FLAGS = 2;
pub type FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = u32;
pub const FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS_AUTO_RESOLVE: FW_DYNAMIC_KEYWORD_ADDRESS_FLAGS = 1;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_INVALID: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 0;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_LOCAL: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 1;
pub const FW_DYNAMIC_KEYWORD_ORIGIN_MDM: FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = 2;
pub type FW_DYNAMIC_KEYWORD_ORIGIN_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct INET_FIREWALL_AC_BINARIES {
    pub count: u32,
    pub binaries: *mut windows_sys::core::PWSTR,
}
pub const INET_FIREWALL_AC_BINARY: INET_FIREWALL_AC_CREATION_TYPE = 2;
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct INET_FIREWALL_AC_CAPABILITIES {
    pub count: u32,
    pub capabilities: *mut super::SID_AND_ATTRIBUTES,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct INET_FIREWALL_AC_CHANGE {
    pub changeType: INET_FIREWALL_AC_CHANGE_TYPE,
    pub createType: INET_FIREWALL_AC_CREATION_TYPE,
    pub appContainerSid: *mut super::SID,
    pub userSid: *mut super::SID,
    pub displayName: windows_sys::core::PWSTR,
    pub Anonymous: INET_FIREWALL_AC_CHANGE_0,
}
#[cfg(feature = "winnt")]
impl Default for INET_FIREWALL_AC_CHANGE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub union INET_FIREWALL_AC_CHANGE_0 {
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
}
#[cfg(feature = "winnt")]
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
#[cfg(feature = "winnt")]
#[derive(Clone, Copy, Default)]
pub struct INET_FIREWALL_APP_CONTAINER {
    pub appContainerSid: *mut super::SID,
    pub userSid: *mut super::SID,
    pub appContainerName: windows_sys::core::PWSTR,
    pub displayName: windows_sys::core::PWSTR,
    pub description: windows_sys::core::PWSTR,
    pub capabilities: INET_FIREWALL_AC_CAPABILITIES,
    pub binaries: INET_FIREWALL_AC_BINARIES,
    pub workingDirectory: windows_sys::core::PWSTR,
    pub packageFullName: windows_sys::core::PWSTR,
}
pub type NETISO_ERROR_TYPE = i32;
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT: NETISO_ERROR_TYPE = 2;
pub const NETISO_ERROR_TYPE_INTERNET_CLIENT_SERVER: NETISO_ERROR_TYPE = 3;
pub const NETISO_ERROR_TYPE_MAX: NETISO_ERROR_TYPE = 4;
pub const NETISO_ERROR_TYPE_NONE: NETISO_ERROR_TYPE = 0;
pub const NETISO_ERROR_TYPE_PRIVATE_NETWORK: NETISO_ERROR_TYPE = 1;
pub type NETISO_FLAG = i32;
pub const NETISO_FLAG_FORCE_COMPUTE_BINARIES: NETISO_FLAG = 1;
pub const NETISO_FLAG_MAX: NETISO_FLAG = 2;
pub const NETISO_GEID_FOR_NEUTRAL_AWARE: i32 = 2;
pub const NETISO_GEID_FOR_WDAG: i32 = 1;
pub const NetFwAuthorizedApplication: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xec9846b3_2762_4a6b_a214_6acb603462d2);
pub const NetFwMgr: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x304ce942_6e39_40d8_943a_b913c40c9cd4);
pub const NetFwOpenPort: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x0ca545c6_37ad_4a6c_bf92_9f7610067ef5);
pub const NetFwPolicy2: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe2b3c97f_6ae1_41ac_817a_f6f92166d7dd);
pub const NetFwProduct: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x9d745ed8_c514_4d1d_bf42_751fed2d5ac7);
pub const NetFwProducts: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xcc19079b_8272_4d73_bb70_cdb533527b61);
pub const NetFwRule: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x2c5bc43e_3369_4c33_ab0c_be9469677af4);
#[cfg(feature = "winnt")]
pub type PAC_CHANGES_CALLBACK_FN = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, pchange: *const INET_FIREWALL_AC_CHANGE)>;
pub type PFN_FWADDDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddress: *const FW_DYNAMIC_KEYWORD_ADDRESS0) -> u32>;
pub type PFN_FWDELETEDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_sys::core::GUID) -> u32>;
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSBYID0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_sys::core::GUID, dynamickeywordaddressdata: *mut PFW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWENUMDYNAMICKEYWORDADDRESSESBYTYPE0 = Option<unsafe extern "system" fn(flags: u32, dynamickeywordaddressdata: *mut PFW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWFREEDYNAMICKEYWORDADDRESSDATA0 = Option<unsafe extern "system" fn(dynamickeywordaddressdata: *const FW_DYNAMIC_KEYWORD_ADDRESS_DATA0) -> u32>;
pub type PFN_FWUPDATEDYNAMICKEYWORDADDRESS0 = Option<unsafe extern "system" fn(dynamickeywordaddressid: windows_sys::core::GUID, updatedaddresses: windows_sys::core::PCWSTR, append: windows_sys::core::BOOL) -> u32>;
pub type PFW_DYNAMIC_KEYWORD_ADDRESS0 = *mut FW_DYNAMIC_KEYWORD_ADDRESS0;
pub type PFW_DYNAMIC_KEYWORD_ADDRESS_DATA0 = *mut FW_DYNAMIC_KEYWORD_ADDRESS_DATA0;
pub type PINET_FIREWALL_AC_BINARIES = *mut INET_FIREWALL_AC_BINARIES;
#[cfg(feature = "winnt")]
pub type PINET_FIREWALL_AC_CAPABILITIES = *mut INET_FIREWALL_AC_CAPABILITIES;
#[cfg(feature = "winnt")]
pub type PINET_FIREWALL_AC_CHANGE = *mut INET_FIREWALL_AC_CHANGE;
#[cfg(feature = "winnt")]
pub type PINET_FIREWALL_APP_CONTAINER = *mut INET_FIREWALL_APP_CONTAINER;
pub type PNETISO_EDP_ID_CALLBACK_FN = Option<unsafe extern "system" fn(context: *mut core::ffi::c_void, wszenterpriseid: windows_sys::core::PCWSTR, dwerr: u32)>;
