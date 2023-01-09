impl ::core::default::Default for NET_INTERFACE_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for NET_INTERFACE_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.InterfaceIndex == other.InterfaceIndex && self.ConfigurationName == other.ConfigurationName
    }
}
impl ::core::cmp::Eq for NET_INTERFACE_CONTEXT {}
impl ::core::fmt::Debug for NET_INTERFACE_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_INTERFACE_CONTEXT").field("InterfaceIndex", &self.InterfaceIndex).field("ConfigurationName", &self.ConfigurationName).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for NET_INTERFACE_CONTEXT_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for NET_INTERFACE_CONTEXT_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.InterfaceContextHandle == other.InterfaceContextHandle && self.NumberOfEntries == other.NumberOfEntries && self.InterfaceContextArray == other.InterfaceContextArray
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for NET_INTERFACE_CONTEXT_TABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for NET_INTERFACE_CONTEXT_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("NET_INTERFACE_CONTEXT_TABLE").field("InterfaceContextHandle", &self.InterfaceContextHandle).field("NumberOfEntries", &self.NumberOfEntries).field("InterfaceContextArray", &self.InterfaceContextArray).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WCM_BILLING_CYCLE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WCM_BILLING_CYCLE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.StartDate == other.StartDate && self.Duration == other.Duration && self.Reset == other.Reset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WCM_BILLING_CYCLE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WCM_BILLING_CYCLE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_BILLING_CYCLE_INFO").field("StartDate", &self.StartDate).field("Duration", &self.Duration).field("Reset", &self.Reset).finish()
    }
}
impl ::core::default::Default for WCM_CONNECTION_COST {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCM_CONNECTION_COST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCM_CONNECTION_COST").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCM_CONNECTION_COST_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WCM_CONNECTION_COST_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.ConnectionCost == other.ConnectionCost && self.CostSource == other.CostSource
    }
}
impl ::core::cmp::Eq for WCM_CONNECTION_COST_DATA {}
impl ::core::fmt::Debug for WCM_CONNECTION_COST_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_CONNECTION_COST_DATA").field("ConnectionCost", &self.ConnectionCost).field("CostSource", &self.CostSource).finish()
    }
}
impl ::core::default::Default for WCM_CONNECTION_COST_SOURCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCM_CONNECTION_COST_SOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCM_CONNECTION_COST_SOURCE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WCM_DATAPLAN_STATUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WCM_DATAPLAN_STATUS {
    fn eq(&self, other: &Self) -> bool {
        self.UsageData == other.UsageData && self.DataLimitInMegabytes == other.DataLimitInMegabytes && self.InboundBandwidthInKbps == other.InboundBandwidthInKbps && self.OutboundBandwidthInKbps == other.OutboundBandwidthInKbps && self.BillingCycle == other.BillingCycle && self.MaxTransferSizeInMegabytes == other.MaxTransferSizeInMegabytes && self.Reserved == other.Reserved
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WCM_DATAPLAN_STATUS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WCM_DATAPLAN_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_DATAPLAN_STATUS").field("UsageData", &self.UsageData).field("DataLimitInMegabytes", &self.DataLimitInMegabytes).field("InboundBandwidthInKbps", &self.InboundBandwidthInKbps).field("OutboundBandwidthInKbps", &self.OutboundBandwidthInKbps).field("BillingCycle", &self.BillingCycle).field("MaxTransferSizeInMegabytes", &self.MaxTransferSizeInMegabytes).field("Reserved", &self.Reserved).finish()
    }
}
impl ::core::default::Default for WCM_MEDIA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCM_MEDIA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCM_MEDIA_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WCM_POLICY_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WCM_POLICY_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.fValue == other.fValue && self.fIsGroupPolicy == other.fIsGroupPolicy
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WCM_POLICY_VALUE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WCM_POLICY_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_POLICY_VALUE").field("fValue", &self.fValue).field("fIsGroupPolicy", &self.fIsGroupPolicy).finish()
    }
}
impl ::core::default::Default for WCM_PROFILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WCM_PROFILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.strProfileName == other.strProfileName && self.AdapterGUID == other.AdapterGUID && self.Media == other.Media
    }
}
impl ::core::cmp::Eq for WCM_PROFILE_INFO {}
impl ::core::fmt::Debug for WCM_PROFILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_PROFILE_INFO").field("strProfileName", &self.strProfileName).field("AdapterGUID", &self.AdapterGUID).field("Media", &self.Media).finish()
    }
}
impl ::core::default::Default for WCM_PROFILE_INFO_LIST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WCM_PROFILE_INFO_LIST {
    fn eq(&self, other: &Self) -> bool {
        self.dwNumberOfItems == other.dwNumberOfItems && self.ProfileInfo == other.ProfileInfo
    }
}
impl ::core::cmp::Eq for WCM_PROFILE_INFO_LIST {}
impl ::core::fmt::Debug for WCM_PROFILE_INFO_LIST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_PROFILE_INFO_LIST").field("dwNumberOfItems", &self.dwNumberOfItems).field("ProfileInfo", &self.ProfileInfo).finish()
    }
}
impl ::core::default::Default for WCM_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for WCM_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("WCM_PROPERTY").field(&self.0).finish()
    }
}
impl ::core::default::Default for WCM_TIME_INTERVAL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for WCM_TIME_INTERVAL {
    fn eq(&self, other: &Self) -> bool {
        self.wYear == other.wYear && self.wMonth == other.wMonth && self.wDay == other.wDay && self.wHour == other.wHour && self.wMinute == other.wMinute && self.wSecond == other.wSecond && self.wMilliseconds == other.wMilliseconds
    }
}
impl ::core::cmp::Eq for WCM_TIME_INTERVAL {}
impl ::core::fmt::Debug for WCM_TIME_INTERVAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_TIME_INTERVAL").field("wYear", &self.wYear).field("wMonth", &self.wMonth).field("wDay", &self.wDay).field("wHour", &self.wHour).field("wMinute", &self.wMinute).field("wSecond", &self.wSecond).field("wMilliseconds", &self.wMilliseconds).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for WCM_USAGE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for WCM_USAGE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.UsageInMegabytes == other.UsageInMegabytes && self.LastSyncTime == other.LastSyncTime
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for WCM_USAGE_DATA {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for WCM_USAGE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("WCM_USAGE_DATA").field("UsageInMegabytes", &self.UsageInMegabytes).field("LastSyncTime", &self.LastSyncTime).finish()
    }
}
