#[inline]
pub unsafe fn WcmFreeMemory(pmemory: *mut core::ffi::c_void) {
    windows_core::link!("wcmapi.dll" "system" fn WcmFreeMemory(pmemory : *mut core::ffi::c_void));
    unsafe { WcmFreeMemory(pmemory as _) }
}
#[inline]
pub unsafe fn WcmGetProfileList(preserved: Option<*const core::ffi::c_void>, ppprofilelist: *mut *mut WCM_PROFILE_INFO_LIST) -> u32 {
    windows_core::link!("wcmapi.dll" "system" fn WcmGetProfileList(preserved : *const core::ffi::c_void, ppprofilelist : *mut *mut WCM_PROFILE_INFO_LIST) -> u32);
    unsafe { WcmGetProfileList(preserved.unwrap_or(core::mem::zeroed()) as _, ppprofilelist as _) }
}
#[cfg(feature = "minwindef")]
#[inline]
pub unsafe fn WcmQueryProperty<P1>(pinterface: Option<*const windows_core::GUID>, strprofilename: P1, property: WCM_PROPERTY, preserved: Option<*const core::ffi::c_void>, pdwdatasize: *mut u32, ppdata: *mut super::minwindef::PBYTE) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wcmapi.dll" "system" fn WcmQueryProperty(pinterface : *const windows_core::GUID, strprofilename : windows_core::PCWSTR, property : WCM_PROPERTY, preserved : *const core::ffi::c_void, pdwdatasize : *mut u32, ppdata : *mut super::minwindef::PBYTE) -> u32);
    unsafe { WcmQueryProperty(pinterface.unwrap_or(core::mem::zeroed()) as _, strprofilename.param().abi(), property, preserved.unwrap_or(core::mem::zeroed()) as _, pdwdatasize as _, ppdata as _) }
}
#[inline]
pub unsafe fn WcmSetProfileList(pprofilelist: *const WCM_PROFILE_INFO_LIST, dwposition: u32, fignoreunknownprofiles: bool, preserved: Option<*const core::ffi::c_void>) -> u32 {
    windows_core::link!("wcmapi.dll" "system" fn WcmSetProfileList(pprofilelist : *const WCM_PROFILE_INFO_LIST, dwposition : u32, fignoreunknownprofiles : windows_core::BOOL, preserved : *const core::ffi::c_void) -> u32);
    unsafe { WcmSetProfileList(pprofilelist, dwposition, fignoreunknownprofiles.into(), preserved.unwrap_or(core::mem::zeroed()) as _) }
}
#[inline]
pub unsafe fn WcmSetProperty<P1>(pinterface: Option<*const windows_core::GUID>, strprofilename: P1, property: WCM_PROPERTY, preserved: Option<*const core::ffi::c_void>, pbdata: Option<&[u8]>) -> u32
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("wcmapi.dll" "system" fn WcmSetProperty(pinterface : *const windows_core::GUID, strprofilename : windows_core::PCWSTR, property : WCM_PROPERTY, preserved : *const core::ffi::c_void, dwdatasize : u32, pbdata : *const u8) -> u32);
    unsafe { WcmSetProperty(pinterface.unwrap_or(core::mem::zeroed()) as _, strprofilename.param().abi(), property, preserved.unwrap_or(core::mem::zeroed()) as _, pbdata.map_or(0, |slice| slice.len().try_into().unwrap()), core::mem::transmute(pbdata.map_or(core::ptr::null(), |slice| slice.as_ptr()))) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWCM_CONNECTION_COST(pub *mut WCM_CONNECTION_COST);
impl PWCM_CONNECTION_COST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWCM_CONNECTION_COST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWCM_CONNECTION_COST_DATA(pub *mut WCM_CONNECTION_COST_DATA);
impl PWCM_CONNECTION_COST_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWCM_CONNECTION_COST_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWCM_CONNECTION_COST_SOURCE(pub *mut WCM_CONNECTION_COST_SOURCE);
impl PWCM_CONNECTION_COST_SOURCE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWCM_CONNECTION_COST_SOURCE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWCM_DATAPLAN_STATUS(pub *mut WCM_DATAPLAN_STATUS);
#[cfg(feature = "minwindef")]
impl PWCM_DATAPLAN_STATUS {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PWCM_DATAPLAN_STATUS {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWCM_MEDIA_TYPE(pub *mut WCM_MEDIA_TYPE);
impl PWCM_MEDIA_TYPE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWCM_MEDIA_TYPE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWCM_POLICY_VALUE(pub *mut WCM_POLICY_VALUE);
impl PWCM_POLICY_VALUE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWCM_POLICY_VALUE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWCM_PROFILE_INFO(pub *mut WCM_PROFILE_INFO);
impl PWCM_PROFILE_INFO {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWCM_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWCM_PROFILE_INFO_LIST(pub *mut WCM_PROFILE_INFO_LIST);
impl PWCM_PROFILE_INFO_LIST {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWCM_PROFILE_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWCM_PROPERTY(pub *mut WCM_PROPERTY);
impl PWCM_PROPERTY {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for PWCM_PROPERTY {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(feature = "minwindef")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PWCM_USAGE_DATA(pub *mut WCM_USAGE_DATA);
#[cfg(feature = "minwindef")]
impl PWCM_USAGE_DATA {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
#[cfg(feature = "minwindef")]
impl Default for PWCM_USAGE_DATA {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const WCM_API_VERSION: u32 = 1;
pub const WCM_API_VERSION_1_0: u32 = 1;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCM_BILLING_CYCLE_INFO {
    pub StartDate: super::minwindef::FILETIME,
    pub Duration: WCM_TIME_INTERVAL,
    pub Reset: windows_core::BOOL,
}
pub type WCM_CONNECTION_COST = i32;
pub const WCM_CONNECTION_COST_APPROACHINGDATALIMIT: WCM_CONNECTION_COST = 524288;
pub const WCM_CONNECTION_COST_CONGESTED: WCM_CONNECTION_COST = 131072;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCM_CONNECTION_COST_DATA {
    pub ConnectionCost: u32,
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
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCM_DATAPLAN_STATUS {
    pub UsageData: WCM_USAGE_DATA,
    pub DataLimitInMegabytes: u32,
    pub InboundBandwidthInKbps: u32,
    pub OutboundBandwidthInKbps: u32,
    pub BillingCycle: WCM_BILLING_CYCLE_INFO,
    pub MaxTransferSizeInMegabytes: u32,
    pub Reserved: u32,
}
pub const WCM_MAX_PROFILE_NAME: u32 = 256;
pub type WCM_MEDIA_TYPE = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCM_POLICY_VALUE {
    pub fValue: windows_core::BOOL,
    pub fIsGroupPolicy: windows_core::BOOL,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WCM_PROFILE_INFO {
    pub strProfileName: [u16; 256],
    pub AdapterGUID: windows_core::GUID,
    pub Media: WCM_MEDIA_TYPE,
}
impl Default for WCM_PROFILE_INFO {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WCM_PROFILE_INFO_LIST {
    pub dwNumberOfItems: u32,
    pub ProfileInfo: [WCM_PROFILE_INFO; 1],
}
impl Default for WCM_PROFILE_INFO_LIST {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type WCM_PROPERTY = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCM_TIME_INTERVAL {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
pub const WCM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295;
#[repr(C)]
#[cfg(feature = "minwindef")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WCM_USAGE_DATA {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::minwindef::FILETIME,
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
