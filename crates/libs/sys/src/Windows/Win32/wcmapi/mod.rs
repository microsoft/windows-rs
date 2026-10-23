windows_link::link!("wcmapi.dll" "system" fn WcmFreeMemory(pmemory : *mut core::ffi::c_void));
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wcmapi.dll" "system" fn WcmGetProfileList(preserved : *mut core::ffi::c_void, ppprofilelist : *mut *mut WCM_PROFILE_INFO_LIST) -> super::DWORD);
#[cfg(all(feature = "minwindef", feature = "shared"))]
windows_link::link!("wcmapi.dll" "system" fn WcmQueryProperty(pinterface : *const windows_sys::core::GUID, strprofilename : windows_sys::core::PCWSTR, property : WCM_PROPERTY, preserved : *mut core::ffi::c_void, pdwdatasize : super::PDWORD, ppdata : *mut super::PBYTE) -> super::DWORD);
#[cfg(all(feature = "shared", feature = "winnt"))]
windows_link::link!("wcmapi.dll" "system" fn WcmSetProfileList(pprofilelist : *const WCM_PROFILE_INFO_LIST, dwposition : super::DWORD, fignoreunknownprofiles : windows_sys::core::BOOL, preserved : *mut core::ffi::c_void) -> super::DWORD);
#[cfg(feature = "shared")]
windows_link::link!("wcmapi.dll" "system" fn WcmSetProperty(pinterface : *const windows_sys::core::GUID, strprofilename : windows_sys::core::PCWSTR, property : WCM_PROPERTY, preserved : *mut core::ffi::c_void, dwdatasize : super::DWORD, pbdata : *const super::BYTE) -> super::DWORD);
pub type PWCM_CONNECTION_COST = *mut WCM_CONNECTION_COST;
#[cfg(feature = "shared")]
pub type PWCM_CONNECTION_COST_DATA = *mut WCM_CONNECTION_COST_DATA;
pub type PWCM_CONNECTION_COST_SOURCE = *mut WCM_CONNECTION_COST_SOURCE;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PWCM_DATAPLAN_STATUS = *mut WCM_DATAPLAN_STATUS;
pub type PWCM_MEDIA_TYPE = *mut WCM_MEDIA_TYPE;
pub type PWCM_POLICY_VALUE = *mut WCM_POLICY_VALUE;
#[cfg(feature = "winnt")]
pub type PWCM_PROFILE_INFO = *mut WCM_PROFILE_INFO;
#[cfg(all(feature = "shared", feature = "winnt"))]
pub type PWCM_PROFILE_INFO_LIST = *mut WCM_PROFILE_INFO_LIST;
pub type PWCM_PROPERTY = *mut WCM_PROPERTY;
#[cfg(all(feature = "minwindef", feature = "shared"))]
pub type PWCM_USAGE_DATA = *mut WCM_USAGE_DATA;
pub const WCM_API_VERSION: i32 = 1;
pub const WCM_API_VERSION_1_0: i32 = 1;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct WCM_BILLING_CYCLE_INFO {
    pub StartDate: super::FILETIME,
    pub Duration: WCM_TIME_INTERVAL,
    pub Reset: windows_sys::core::BOOL,
}
pub type WCM_CONNECTION_COST = i32;
pub const WCM_CONNECTION_COST_APPROACHINGDATALIMIT: WCM_CONNECTION_COST = 524288;
pub const WCM_CONNECTION_COST_CONGESTED: WCM_CONNECTION_COST = 131072;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WCM_CONNECTION_COST_DATA {
    pub ConnectionCost: super::DWORD,
    pub CostSource: WCM_CONNECTION_COST_SOURCE,
}
pub const WCM_CONNECTION_COST_FIXED: WCM_CONNECTION_COST = 2;
pub const WCM_CONNECTION_COST_OVERDATALIMIT: WCM_CONNECTION_COST = 65536;
pub const WCM_CONNECTION_COST_ROAMING: WCM_CONNECTION_COST = 262144;
pub type WCM_CONNECTION_COST_SOURCE = i32;
pub const WCM_CONNECTION_COST_SOURCE_DEFAULT: WCM_CONNECTION_COST_SOURCE = 0;
pub const WCM_CONNECTION_COST_SOURCE_GP: WCM_CONNECTION_COST_SOURCE = 1;
pub const WCM_CONNECTION_COST_SOURCE_OPERATOR: WCM_CONNECTION_COST_SOURCE = 3;
pub const WCM_CONNECTION_COST_SOURCE_USER: WCM_CONNECTION_COST_SOURCE = 2;
pub const WCM_CONNECTION_COST_UNKNOWN: WCM_CONNECTION_COST = 0;
pub const WCM_CONNECTION_COST_UNRESTRICTED: WCM_CONNECTION_COST = 1;
pub const WCM_CONNECTION_COST_VARIABLE: WCM_CONNECTION_COST = 4;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct WCM_DATAPLAN_STATUS {
    pub UsageData: WCM_USAGE_DATA,
    pub DataLimitInMegabytes: super::DWORD,
    pub InboundBandwidthInKbps: super::DWORD,
    pub OutboundBandwidthInKbps: super::DWORD,
    pub BillingCycle: WCM_BILLING_CYCLE_INFO,
    pub MaxTransferSizeInMegabytes: super::DWORD,
    pub Reserved: super::DWORD,
}
pub const WCM_MAX_PROFILE_NAME: i32 = 256;
pub type WCM_MEDIA_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct WCM_POLICY_VALUE {
    pub fValue: windows_sys::core::BOOL,
    pub fIsGroupPolicy: windows_sys::core::BOOL,
}
#[repr(C)]
#[cfg(feature = "winnt")]
#[derive(Clone, Copy)]
pub struct WCM_PROFILE_INFO {
    pub strProfileName: [super::WCHAR; 256],
    pub AdapterGUID: windows_sys::core::GUID,
    pub Media: WCM_MEDIA_TYPE,
}
#[cfg(feature = "winnt")]
impl Default for WCM_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "winnt"))]
#[derive(Clone, Copy)]
pub struct WCM_PROFILE_INFO_LIST {
    pub dwNumberOfItems: super::DWORD,
    pub ProfileInfo: [WCM_PROFILE_INFO; 1],
}
#[cfg(all(feature = "shared", feature = "winnt"))]
impl Default for WCM_PROFILE_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WCM_PROPERTY = i32;
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Default)]
pub struct WCM_TIME_INTERVAL {
    pub wYear: super::WORD,
    pub wMonth: super::WORD,
    pub wDay: super::WORD,
    pub wHour: super::WORD,
    pub wMinute: super::WORD,
    pub wSecond: super::WORD,
    pub wMilliseconds: super::WORD,
}
pub const WCM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295;
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "shared"))]
#[derive(Clone, Copy, Default)]
pub struct WCM_USAGE_DATA {
    pub UsageInMegabytes: super::DWORD,
    pub LastSyncTime: super::FILETIME,
}
pub const wcm_global_property_domain_policy: WCM_PROPERTY = 0;
pub const wcm_global_property_minimize_policy: WCM_PROPERTY = 1;
pub const wcm_global_property_powermanagement_policy: WCM_PROPERTY = 3;
pub const wcm_global_property_roaming_policy: WCM_PROPERTY = 2;
pub const wcm_intf_property_connection_cost: WCM_PROPERTY = 4;
pub const wcm_intf_property_dataplan_status: WCM_PROPERTY = 5;
pub const wcm_intf_property_hotspot_profile: WCM_PROPERTY = 6;
pub const wcm_media_ethernet: WCM_MEDIA_TYPE = 1;
pub const wcm_media_invalid: WCM_MEDIA_TYPE = 4;
pub const wcm_media_max: WCM_MEDIA_TYPE = 5;
pub const wcm_media_mbn: WCM_MEDIA_TYPE = 3;
pub const wcm_media_unknown: WCM_MEDIA_TYPE = 0;
pub const wcm_media_wlan: WCM_MEDIA_TYPE = 2;
