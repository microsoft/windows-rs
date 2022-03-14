#![allow(non_snake_case, non_camel_case_types, non_upper_case_globals, clashing_extern_declarations, clippy::all)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn FreeInterfaceContextTable(interfacecontexttable: *const NET_INTERFACE_CONTEXT_TABLE) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn FreeInterfaceContextTable(interfacecontexttable: *const NET_INTERFACE_CONTEXT_TABLE);
        }
        FreeInterfaceContextTable(::core::mem::transmute(interfacecontexttable))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn GetInterfaceContextTableForHostName<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(hostname: Param0, proxyname: Param1, flags: u32, connectionprofilefilterrawdata: *const u8, connectionprofilefilterrawdatasize: u32) -> ::windows::core::Result<*mut NET_INTERFACE_CONTEXT_TABLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn GetInterfaceContextTableForHostName(hostname: ::windows::core::PCWSTR, proxyname: ::windows::core::PCWSTR, flags: u32, connectionprofilefilterrawdata: *const u8, connectionprofilefilterrawdatasize: u32, interfacecontexttable: *mut *mut NET_INTERFACE_CONTEXT_TABLE) -> ::windows::core::HRESULT;
        }
        let mut result__: *mut NET_INTERFACE_CONTEXT_TABLE = ::core::mem::zeroed();
        GetInterfaceContextTableForHostName(hostname.into_param().abi(), proxyname.into_param().abi(), ::core::mem::transmute(flags), ::core::mem::transmute(connectionprofilefilterrawdata), ::core::mem::transmute(connectionprofilefilterrawdatasize), ::core::mem::transmute(&mut result__)).from_abi::<*mut NET_INTERFACE_CONTEXT_TABLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub struct NET_INTERFACE_CONTEXT {
    pub InterfaceIndex: u32,
    pub ConfigurationName: ::windows::core::PWSTR,
}
impl ::core::marker::Copy for NET_INTERFACE_CONTEXT {}
impl ::core::clone::Clone for NET_INTERFACE_CONTEXT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for NET_INTERFACE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_INTERFACE_CONTEXT").field("InterfaceIndex", &self.InterfaceIndex).field("ConfigurationName", &self.ConfigurationName).finish()
    }
}
unsafe impl ::windows::core::Abi for NET_INTERFACE_CONTEXT {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for NET_INTERFACE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_INTERFACE_CONTEXT>()) == 0 }
    }
}
impl ::core::cmp::Eq for NET_INTERFACE_CONTEXT {}
impl ::core::default::Default for NET_INTERFACE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct NET_INTERFACE_CONTEXT_TABLE {
    pub InterfaceContextHandle: super::super::Foundation::HANDLE,
    pub NumberOfEntries: u32,
    pub InterfaceContextArray: *mut NET_INTERFACE_CONTEXT,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for NET_INTERFACE_CONTEXT_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for NET_INTERFACE_CONTEXT_TABLE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NET_INTERFACE_CONTEXT_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_INTERFACE_CONTEXT_TABLE").field("InterfaceContextHandle", &self.InterfaceContextHandle).field("NumberOfEntries", &self.NumberOfEntries).field("InterfaceContextArray", &self.InterfaceContextArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for NET_INTERFACE_CONTEXT_TABLE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_INTERFACE_CONTEXT_TABLE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<NET_INTERFACE_CONTEXT_TABLE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_INTERFACE_CONTEXT_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_INTERFACE_CONTEXT_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const NET_INTERFACE_FLAG_CONNECT_IF_NEEDED: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const NET_INTERFACE_FLAG_NONE: u32 = 0u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub type ONDEMAND_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(param0: *const ::core::ffi::c_void)>;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[inline]
pub unsafe fn OnDemandGetRoutingHint<'a, Param0: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(destinationhostname: Param0) -> ::windows::core::Result<u32> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OnDemandGetRoutingHint(destinationhostname: ::windows::core::PCWSTR, interfaceindex: *mut u32) -> ::windows::core::HRESULT;
        }
        let mut result__: u32 = ::core::mem::zeroed();
        OnDemandGetRoutingHint(destinationhostname.into_param().abi(), ::core::mem::transmute(&mut result__)).from_abi::<u32>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OnDemandRegisterNotification(callback: ONDEMAND_NOTIFICATION_CALLBACK, callbackcontext: *const ::core::ffi::c_void) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OnDemandRegisterNotification(callback: ::windows::core::RawPtr, callbackcontext: *const ::core::ffi::c_void, registrationhandle: *mut super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        let mut result__: super::super::Foundation::HANDLE = ::core::mem::zeroed();
        OnDemandRegisterNotification(::core::mem::transmute(callback), ::core::mem::transmute(callbackcontext), ::core::mem::transmute(&mut result__)).from_abi::<super::super::Foundation::HANDLE>(result__)
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn OnDemandUnRegisterNotification<'a, Param0: ::windows::core::IntoParam<'a, super::super::Foundation::HANDLE>>(registrationhandle: Param0) -> ::windows::core::Result<()> {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn OnDemandUnRegisterNotification(registrationhandle: super::super::Foundation::HANDLE) -> ::windows::core::HRESULT;
        }
        OnDemandUnRegisterNotification(registrationhandle.into_param().abi()).ok()
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_API_VERSION: u32 = 1u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_API_VERSION_1_0: u32 = 1u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WCM_BILLING_CYCLE_INFO {
    pub StartDate: super::super::Foundation::FILETIME,
    pub Duration: WCM_TIME_INTERVAL,
    pub Reset: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WCM_BILLING_CYCLE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WCM_BILLING_CYCLE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WCM_BILLING_CYCLE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_BILLING_CYCLE_INFO").field("StartDate", &self.StartDate).field("Duration", &self.Duration).field("Reset", &self.Reset).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WCM_BILLING_CYCLE_INFO {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WCM_BILLING_CYCLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WCM_BILLING_CYCLE_INFO>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WCM_BILLING_CYCLE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WCM_BILLING_CYCLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WCM_CONNECTION_COST(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_UNKNOWN: WCM_CONNECTION_COST = WCM_CONNECTION_COST(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_UNRESTRICTED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_FIXED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_VARIABLE: WCM_CONNECTION_COST = WCM_CONNECTION_COST(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_OVERDATALIMIT: WCM_CONNECTION_COST = WCM_CONNECTION_COST(65536i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_CONGESTED: WCM_CONNECTION_COST = WCM_CONNECTION_COST(131072i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_ROAMING: WCM_CONNECTION_COST = WCM_CONNECTION_COST(262144i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_APPROACHINGDATALIMIT: WCM_CONNECTION_COST = WCM_CONNECTION_COST(524288i32);
impl ::core::marker::Copy for WCM_CONNECTION_COST {}
impl ::core::clone::Clone for WCM_CONNECTION_COST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WCM_CONNECTION_COST {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WCM_CONNECTION_COST {
    type Abi = Self;
}
impl ::core::fmt::Debug for WCM_CONNECTION_COST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCM_CONNECTION_COST").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub struct WCM_CONNECTION_COST_DATA {
    pub ConnectionCost: u32,
    pub CostSource: WCM_CONNECTION_COST_SOURCE,
}
impl ::core::marker::Copy for WCM_CONNECTION_COST_DATA {}
impl ::core::clone::Clone for WCM_CONNECTION_COST_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WCM_CONNECTION_COST_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_CONNECTION_COST_DATA").field("ConnectionCost", &self.ConnectionCost).field("CostSource", &self.CostSource).finish()
    }
}
unsafe impl ::windows::core::Abi for WCM_CONNECTION_COST_DATA {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WCM_CONNECTION_COST_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WCM_CONNECTION_COST_DATA>()) == 0 }
    }
}
impl ::core::cmp::Eq for WCM_CONNECTION_COST_DATA {}
impl ::core::default::Default for WCM_CONNECTION_COST_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WCM_CONNECTION_COST_SOURCE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_SOURCE_DEFAULT: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_SOURCE_GP: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_SOURCE_USER: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_CONNECTION_COST_SOURCE_OPERATOR: WCM_CONNECTION_COST_SOURCE = WCM_CONNECTION_COST_SOURCE(3i32);
impl ::core::marker::Copy for WCM_CONNECTION_COST_SOURCE {}
impl ::core::clone::Clone for WCM_CONNECTION_COST_SOURCE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WCM_CONNECTION_COST_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WCM_CONNECTION_COST_SOURCE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WCM_CONNECTION_COST_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCM_CONNECTION_COST_SOURCE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WCM_DATAPLAN_STATUS {
    pub UsageData: WCM_USAGE_DATA,
    pub DataLimitInMegabytes: u32,
    pub InboundBandwidthInKbps: u32,
    pub OutboundBandwidthInKbps: u32,
    pub BillingCycle: WCM_BILLING_CYCLE_INFO,
    pub MaxTransferSizeInMegabytes: u32,
    pub Reserved: u32,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WCM_DATAPLAN_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WCM_DATAPLAN_STATUS {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WCM_DATAPLAN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_DATAPLAN_STATUS").field("UsageData", &self.UsageData).field("DataLimitInMegabytes", &self.DataLimitInMegabytes).field("InboundBandwidthInKbps", &self.InboundBandwidthInKbps).field("OutboundBandwidthInKbps", &self.OutboundBandwidthInKbps).field("BillingCycle", &self.BillingCycle).field("MaxTransferSizeInMegabytes", &self.MaxTransferSizeInMegabytes).field("Reserved", &self.Reserved).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WCM_DATAPLAN_STATUS {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WCM_DATAPLAN_STATUS {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WCM_DATAPLAN_STATUS>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WCM_DATAPLAN_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WCM_DATAPLAN_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_MAX_PROFILE_NAME: u32 = 256u32;
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WCM_MEDIA_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_media_unknown: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_media_ethernet: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_media_wlan: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_media_mbn: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_media_invalid: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_media_max: WCM_MEDIA_TYPE = WCM_MEDIA_TYPE(5i32);
impl ::core::marker::Copy for WCM_MEDIA_TYPE {}
impl ::core::clone::Clone for WCM_MEDIA_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WCM_MEDIA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WCM_MEDIA_TYPE {
    type Abi = Self;
}
impl ::core::fmt::Debug for WCM_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCM_MEDIA_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WCM_POLICY_VALUE {
    pub fValue: super::super::Foundation::BOOL,
    pub fIsGroupPolicy: super::super::Foundation::BOOL,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WCM_POLICY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WCM_POLICY_VALUE {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WCM_POLICY_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_POLICY_VALUE").field("fValue", &self.fValue).field("fIsGroupPolicy", &self.fIsGroupPolicy).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WCM_POLICY_VALUE {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WCM_POLICY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WCM_POLICY_VALUE>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WCM_POLICY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WCM_POLICY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub struct WCM_PROFILE_INFO {
    pub strProfileName: [u16; 256],
    pub AdapterGUID: ::windows::core::GUID,
    pub Media: WCM_MEDIA_TYPE,
}
impl ::core::marker::Copy for WCM_PROFILE_INFO {}
impl ::core::clone::Clone for WCM_PROFILE_INFO {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WCM_PROFILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_PROFILE_INFO").field("strProfileName", &self.strProfileName).field("AdapterGUID", &self.AdapterGUID).field("Media", &self.Media).finish()
    }
}
unsafe impl ::windows::core::Abi for WCM_PROFILE_INFO {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WCM_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WCM_PROFILE_INFO>()) == 0 }
    }
}
impl ::core::cmp::Eq for WCM_PROFILE_INFO {}
impl ::core::default::Default for WCM_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub struct WCM_PROFILE_INFO_LIST {
    pub dwNumberOfItems: u32,
    pub ProfileInfo: [WCM_PROFILE_INFO; 1],
}
impl ::core::marker::Copy for WCM_PROFILE_INFO_LIST {}
impl ::core::clone::Clone for WCM_PROFILE_INFO_LIST {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WCM_PROFILE_INFO_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_PROFILE_INFO_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("ProfileInfo", &self.ProfileInfo).finish()
    }
}
unsafe impl ::windows::core::Abi for WCM_PROFILE_INFO_LIST {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WCM_PROFILE_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WCM_PROFILE_INFO_LIST>()) == 0 }
    }
}
impl ::core::cmp::Eq for WCM_PROFILE_INFO_LIST {}
impl ::core::default::Default for WCM_PROFILE_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[repr(transparent)]
#[derive(:: core :: cmp :: PartialEq, :: core :: cmp :: Eq)]
pub struct WCM_PROPERTY(pub i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_global_property_domain_policy: WCM_PROPERTY = WCM_PROPERTY(0i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_global_property_minimize_policy: WCM_PROPERTY = WCM_PROPERTY(1i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_global_property_roaming_policy: WCM_PROPERTY = WCM_PROPERTY(2i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_global_property_powermanagement_policy: WCM_PROPERTY = WCM_PROPERTY(3i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_intf_property_connection_cost: WCM_PROPERTY = WCM_PROPERTY(4i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_intf_property_dataplan_status: WCM_PROPERTY = WCM_PROPERTY(5i32);
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const wcm_intf_property_hotspot_profile: WCM_PROPERTY = WCM_PROPERTY(6i32);
impl ::core::marker::Copy for WCM_PROPERTY {}
impl ::core::clone::Clone for WCM_PROPERTY {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for WCM_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
unsafe impl ::windows::core::Abi for WCM_PROPERTY {
    type Abi = Self;
}
impl ::core::fmt::Debug for WCM_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCM_PROPERTY").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub struct WCM_TIME_INTERVAL {
    pub wYear: u16,
    pub wMonth: u16,
    pub wDay: u16,
    pub wHour: u16,
    pub wMinute: u16,
    pub wSecond: u16,
    pub wMilliseconds: u16,
}
impl ::core::marker::Copy for WCM_TIME_INTERVAL {}
impl ::core::clone::Clone for WCM_TIME_INTERVAL {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for WCM_TIME_INTERVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_TIME_INTERVAL").field("wYear", &self.wYear).field("wMonth", &self.wMonth).field("wDay", &self.wDay).field("wHour", &self.wHour).field("wMinute", &self.wMinute).field("wSecond", &self.wSecond).field("wMilliseconds", &self.wMilliseconds).finish()
    }
}
unsafe impl ::windows::core::Abi for WCM_TIME_INTERVAL {
    type Abi = Self;
}
impl ::core::cmp::PartialEq for WCM_TIME_INTERVAL {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WCM_TIME_INTERVAL>()) == 0 }
    }
}
impl ::core::cmp::Eq for WCM_TIME_INTERVAL {}
impl ::core::default::Default for WCM_TIME_INTERVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
pub const WCM_UNKNOWN_DATAPLAN_STATUS: u32 = 4294967295u32;
#[repr(C)]
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct WCM_USAGE_DATA {
    pub UsageInMegabytes: u32,
    pub LastSyncTime: super::super::Foundation::FILETIME,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for WCM_USAGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for WCM_USAGE_DATA {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WCM_USAGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_USAGE_DATA").field("UsageInMegabytes", &self.UsageInMegabytes).field("LastSyncTime", &self.LastSyncTime).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
unsafe impl ::windows::core::Abi for WCM_USAGE_DATA {
    type Abi = Self;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WCM_USAGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ::windows::core::memcmp(self as *const _ as _, other as *const _ as _, core::mem::size_of::<WCM_USAGE_DATA>()) == 0 }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WCM_USAGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WCM_USAGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[inline]
pub unsafe fn WcmFreeMemory(pmemory: *mut ::core::ffi::c_void) {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcmFreeMemory(pmemory: *mut ::core::ffi::c_void);
        }
        WcmFreeMemory(::core::mem::transmute(pmemory))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[inline]
pub unsafe fn WcmGetProfileList(preserved: *mut ::core::ffi::c_void, ppprofilelist: *mut *mut WCM_PROFILE_INFO_LIST) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcmGetProfileList(preserved: *mut ::core::ffi::c_void, ppprofilelist: *mut *mut WCM_PROFILE_INFO_LIST) -> u32;
        }
        ::core::mem::transmute(WcmGetProfileList(::core::mem::transmute(preserved), ::core::mem::transmute(ppprofilelist)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[inline]
pub unsafe fn WcmQueryProperty<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pinterface: *const ::windows::core::GUID, strprofilename: Param1, property: WCM_PROPERTY, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcmQueryProperty(pinterface: *const ::windows::core::GUID, strprofilename: ::windows::core::PCWSTR, property: WCM_PROPERTY, preserved: *mut ::core::ffi::c_void, pdwdatasize: *mut u32, ppdata: *mut *mut u8) -> u32;
        }
        ::core::mem::transmute(WcmQueryProperty(::core::mem::transmute(pinterface), strprofilename.into_param().abi(), ::core::mem::transmute(property), ::core::mem::transmute(preserved), ::core::mem::transmute(pdwdatasize), ::core::mem::transmute(ppdata)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn WcmSetProfileList<'a, Param2: ::windows::core::IntoParam<'a, super::super::Foundation::BOOL>>(pprofilelist: *const WCM_PROFILE_INFO_LIST, dwposition: u32, fignoreunknownprofiles: Param2, preserved: *mut ::core::ffi::c_void) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcmSetProfileList(pprofilelist: *const WCM_PROFILE_INFO_LIST, dwposition: u32, fignoreunknownprofiles: super::super::Foundation::BOOL, preserved: *mut ::core::ffi::c_void) -> u32;
        }
        ::core::mem::transmute(WcmSetProfileList(::core::mem::transmute(pprofilelist), ::core::mem::transmute(dwposition), fignoreunknownprofiles.into_param().abi(), ::core::mem::transmute(preserved)))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[doc = "*Required features: `\"Win32_NetworkManagement_WindowsConnectionManager\"`*"]
#[inline]
pub unsafe fn WcmSetProperty<'a, Param1: ::windows::core::IntoParam<'a, ::windows::core::PCWSTR>>(pinterface: *const ::windows::core::GUID, strprofilename: Param1, property: WCM_PROPERTY, preserved: *mut ::core::ffi::c_void, pbdata: &[u8]) -> u32 {
    #[cfg(windows)]
    {
        #[link(name = "windows")]
        extern "system" {
            fn WcmSetProperty(pinterface: *const ::windows::core::GUID, strprofilename: ::windows::core::PCWSTR, property: WCM_PROPERTY, preserved: *mut ::core::ffi::c_void, dwdatasize: u32, pbdata: *const u8) -> u32;
        }
        ::core::mem::transmute(WcmSetProperty(::core::mem::transmute(pinterface), strprofilename.into_param().abi(), ::core::mem::transmute(property), ::core::mem::transmute(preserved), pbdata.len() as _, ::core::mem::transmute(::windows::core::as_ptr_or_null(pbdata))))
    }
    #[cfg(not(windows))]
    unimplemented!("Unsupported target OS");
}
#[cfg(feature = "implement")]
::core::include!("impl.rs");
