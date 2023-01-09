impl ::core::default::Default for DL_ADDRESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DL_ADDRESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DL_ADDRESS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FWPM_ACTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FWPM_APPC_NETWORK_CAPABILITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWPM_APPC_NETWORK_CAPABILITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_APPC_NETWORK_CAPABILITY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FWPM_CALLOUT0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_CALLOUT0 {
    fn eq(&self, other: &Self) -> bool {
        self.calloutKey == other.calloutKey && self.displayData == other.displayData && self.flags == other.flags && self.providerKey == other.providerKey && self.providerData == other.providerData && self.applicableLayer == other.applicableLayer && self.calloutId == other.calloutId
    }
}
impl ::core::cmp::Eq for FWPM_CALLOUT0 {}
impl ::core::fmt::Debug for FWPM_CALLOUT0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_CALLOUT0").field("calloutKey", &self.calloutKey).field("displayData", &self.displayData).field("flags", &self.flags).field("providerKey", &self.providerKey).field("providerData", &self.providerData).field("applicableLayer", &self.applicableLayer).field("calloutId", &self.calloutId).finish()
    }
}
impl ::core::default::Default for FWPM_CALLOUT_CHANGE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_CALLOUT_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        self.changeType == other.changeType && self.calloutKey == other.calloutKey && self.calloutId == other.calloutId
    }
}
impl ::core::cmp::Eq for FWPM_CALLOUT_CHANGE0 {}
impl ::core::fmt::Debug for FWPM_CALLOUT_CHANGE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_CALLOUT_CHANGE0").field("changeType", &self.changeType).field("calloutKey", &self.calloutKey).field("calloutId", &self.calloutId).finish()
    }
}
impl ::core::default::Default for FWPM_CALLOUT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_CALLOUT_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.providerKey == other.providerKey && self.layerKey == other.layerKey
    }
}
impl ::core::cmp::Eq for FWPM_CALLOUT_ENUM_TEMPLATE0 {}
impl ::core::fmt::Debug for FWPM_CALLOUT_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_CALLOUT_ENUM_TEMPLATE0").field("providerKey", &self.providerKey).field("layerKey", &self.layerKey).finish()
    }
}
impl ::core::default::Default for FWPM_CALLOUT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_CALLOUT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate && self.flags == other.flags && self.sessionKey == other.sessionKey
    }
}
impl ::core::cmp::Eq for FWPM_CALLOUT_SUBSCRIPTION0 {}
impl ::core::fmt::Debug for FWPM_CALLOUT_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_CALLOUT_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
impl ::core::default::Default for FWPM_CHANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWPM_CHANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_CHANGE_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_CLASSIFY_OPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_CLASSIFY_OPTIONS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_CLASSIFY_OPTIONS0 {
    fn eq(&self, other: &Self) -> bool {
        self.numOptions == other.numOptions && self.options == other.options
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_CLASSIFY_OPTIONS0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWPM_CLASSIFY_OPTIONS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_CLASSIFY_OPTIONS0").field("numOptions", &self.numOptions).field("options", &self.options).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_CONNECTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FWPM_CONNECTION_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_CONNECTION_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.connectionId == other.connectionId && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for FWPM_CONNECTION_ENUM_TEMPLATE0 {}
impl ::core::fmt::Debug for FWPM_CONNECTION_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_CONNECTION_ENUM_TEMPLATE0").field("connectionId", &self.connectionId).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for FWPM_CONNECTION_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWPM_CONNECTION_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_CONNECTION_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FWPM_CONNECTION_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_CONNECTION_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate && self.flags == other.flags && self.sessionKey == other.sessionKey
    }
}
impl ::core::cmp::Eq for FWPM_CONNECTION_SUBSCRIPTION0 {}
impl ::core::fmt::Debug for FWPM_CONNECTION_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_CONNECTION_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
impl ::core::default::Default for FWPM_DISPLAY_DATA0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_DISPLAY_DATA0 {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.description == other.description
    }
}
impl ::core::cmp::Eq for FWPM_DISPLAY_DATA0 {}
impl ::core::fmt::Debug for FWPM_DISPLAY_DATA0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_DISPLAY_DATA0").field("name", &self.name).field("description", &self.description).finish()
    }
}
impl ::core::default::Default for FWPM_ENGINE_OPTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWPM_ENGINE_OPTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_ENGINE_OPTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for FWPM_FIELD0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_FIELD0 {
    fn eq(&self, other: &Self) -> bool {
        self.fieldKey == other.fieldKey && self.r#type == other.r#type && self.dataType == other.dataType
    }
}
impl ::core::cmp::Eq for FWPM_FIELD0 {}
impl ::core::fmt::Debug for FWPM_FIELD0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_FIELD0").field("fieldKey", &self.fieldKey).field("type", &self.r#type).field("dataType", &self.dataType).finish()
    }
}
impl ::core::default::Default for FWPM_FIELD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWPM_FIELD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_FIELD_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_FILTER0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FWPM_FILTER_CHANGE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_FILTER_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        self.changeType == other.changeType && self.filterKey == other.filterKey && self.filterId == other.filterId
    }
}
impl ::core::cmp::Eq for FWPM_FILTER_CHANGE0 {}
impl ::core::fmt::Debug for FWPM_FILTER_CHANGE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_FILTER_CHANGE0").field("changeType", &self.changeType).field("filterKey", &self.filterKey).field("filterId", &self.filterId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_FILTER_CONDITION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_FILTER_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_FILTER_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.providerKey == other.providerKey && self.layerKey == other.layerKey && self.enumType == other.enumType && self.flags == other.flags && self.providerContextTemplate == other.providerContextTemplate && self.numFilterConditions == other.numFilterConditions && self.filterCondition == other.filterCondition && self.actionMask == other.actionMask && self.calloutKey == other.calloutKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_FILTER_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWPM_FILTER_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_FILTER_ENUM_TEMPLATE0").field("providerKey", &self.providerKey).field("layerKey", &self.layerKey).field("enumType", &self.enumType).field("flags", &self.flags).field("providerContextTemplate", &self.providerContextTemplate).field("numFilterConditions", &self.numFilterConditions).field("filterCondition", &self.filterCondition).field("actionMask", &self.actionMask).field("calloutKey", &self.calloutKey).finish()
    }
}
impl ::core::default::Default for FWPM_FILTER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWPM_FILTER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_FILTER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for FWPM_FILTER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for FWPM_FILTER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for FWPM_FILTER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for FWPM_FILTER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for FWPM_FILTER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_FILTER_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_FILTER_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate && self.flags == other.flags && self.sessionKey == other.sessionKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_FILTER_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWPM_FILTER_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_FILTER_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
impl ::core::default::Default for FWPM_LAYER0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_LAYER0 {
    fn eq(&self, other: &Self) -> bool {
        self.layerKey == other.layerKey && self.displayData == other.displayData && self.flags == other.flags && self.numFields == other.numFields && self.field == other.field && self.defaultSubLayerKey == other.defaultSubLayerKey && self.layerId == other.layerId
    }
}
impl ::core::cmp::Eq for FWPM_LAYER0 {}
impl ::core::fmt::Debug for FWPM_LAYER0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_LAYER0").field("layerKey", &self.layerKey).field("displayData", &self.displayData).field("flags", &self.flags).field("numFields", &self.numFields).field("field", &self.field).field("defaultSubLayerKey", &self.defaultSubLayerKey).field("layerId", &self.layerId).finish()
    }
}
impl ::core::default::Default for FWPM_LAYER_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_LAYER_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for FWPM_LAYER_ENUM_TEMPLATE0 {}
impl ::core::fmt::Debug for FWPM_LAYER_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_LAYER_ENUM_TEMPLATE0").field("reserved", &self.reserved).finish()
    }
}
impl ::core::default::Default for FWPM_LAYER_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_LAYER_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.layerId == other.layerId && self.classifyPermitCount == other.classifyPermitCount && self.classifyBlockCount == other.classifyBlockCount && self.classifyVetoCount == other.classifyVetoCount && self.numCacheEntries == other.numCacheEntries
    }
}
impl ::core::cmp::Eq for FWPM_LAYER_STATISTICS0 {}
impl ::core::fmt::Debug for FWPM_LAYER_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_LAYER_STATISTICS0").field("layerId", &self.layerId).field("classifyPermitCount", &self.classifyPermitCount).field("classifyBlockCount", &self.classifyBlockCount).field("classifyVetoCount", &self.classifyVetoCount).field("numCacheEntries", &self.numCacheEntries).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    fn eq(&self, other: &Self) -> bool {
        self.networkCapabilityId == other.networkCapabilityId && self.filterId == other.filterId && self.isLoopback == other.isLoopback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FWPM_NET_EVENT_CAPABILITY_ALLOW0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_CAPABILITY_ALLOW0").field("networkCapabilityId", &self.networkCapabilityId).field("filterId", &self.filterId).field("isLoopback", &self.isLoopback).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_NET_EVENT_CAPABILITY_DROP0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_CAPABILITY_DROP0 {
    fn eq(&self, other: &Self) -> bool {
        self.networkCapabilityId == other.networkCapabilityId && self.filterId == other.filterId && self.isLoopback == other.isLoopback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_NET_EVENT_CAPABILITY_DROP0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FWPM_NET_EVENT_CAPABILITY_DROP0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_CAPABILITY_DROP0").field("networkCapabilityId", &self.networkCapabilityId).field("filterId", &self.filterId).field("isLoopback", &self.isLoopback).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    fn eq(&self, other: &Self) -> bool {
        self.filterId == other.filterId && self.layerId == other.layerId && self.reauthReason == other.reauthReason && self.originalProfile == other.originalProfile && self.currentProfile == other.currentProfile && self.msFwpDirection == other.msFwpDirection && self.isLoopback == other.isLoopback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_ALLOW0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_CLASSIFY_ALLOW0").field("filterId", &self.filterId).field("layerId", &self.layerId).field("reauthReason", &self.reauthReason).field("originalProfile", &self.originalProfile).field("currentProfile", &self.currentProfile).field("msFwpDirection", &self.msFwpDirection).field("isLoopback", &self.isLoopback).finish()
    }
}
impl ::core::default::Default for FWPM_NET_EVENT_CLASSIFY_DROP0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_DROP0 {
    fn eq(&self, other: &Self) -> bool {
        self.filterId == other.filterId && self.layerId == other.layerId
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_DROP0 {}
impl ::core::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_DROP0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_CLASSIFY_DROP0").field("filterId", &self.filterId).field("layerId", &self.layerId).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_NET_EVENT_CLASSIFY_DROP1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_DROP1 {
    fn eq(&self, other: &Self) -> bool {
        self.filterId == other.filterId && self.layerId == other.layerId && self.reauthReason == other.reauthReason && self.originalProfile == other.originalProfile && self.currentProfile == other.currentProfile && self.msFwpDirection == other.msFwpDirection && self.isLoopback == other.isLoopback
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_DROP1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_DROP1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_CLASSIFY_DROP1").field("filterId", &self.filterId).field("layerId", &self.layerId).field("reauthReason", &self.reauthReason).field("originalProfile", &self.originalProfile).field("currentProfile", &self.currentProfile).field("msFwpDirection", &self.msFwpDirection).field("isLoopback", &self.isLoopback).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_NET_EVENT_CLASSIFY_DROP2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_DROP2 {
    fn eq(&self, other: &Self) -> bool {
        self.filterId == other.filterId && self.layerId == other.layerId && self.reauthReason == other.reauthReason && self.originalProfile == other.originalProfile && self.currentProfile == other.currentProfile && self.msFwpDirection == other.msFwpDirection && self.isLoopback == other.isLoopback && self.vSwitchId == other.vSwitchId && self.vSwitchSourcePort == other.vSwitchSourcePort && self.vSwitchDestinationPort == other.vSwitchDestinationPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_DROP2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_DROP2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_CLASSIFY_DROP2")
            .field("filterId", &self.filterId)
            .field("layerId", &self.layerId)
            .field("reauthReason", &self.reauthReason)
            .field("originalProfile", &self.originalProfile)
            .field("currentProfile", &self.currentProfile)
            .field("msFwpDirection", &self.msFwpDirection)
            .field("isLoopback", &self.isLoopback)
            .field("vSwitchId", &self.vSwitchId)
            .field("vSwitchSourcePort", &self.vSwitchSourcePort)
            .field("vSwitchDestinationPort", &self.vSwitchDestinationPort)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    fn eq(&self, other: &Self) -> bool {
        self.localMacAddr == other.localMacAddr && self.remoteMacAddr == other.remoteMacAddr && self.mediaType == other.mediaType && self.ifType == other.ifType && self.etherType == other.etherType && self.ndisPortNumber == other.ndisPortNumber && self.reserved == other.reserved && self.vlanTag == other.vlanTag && self.ifLuid == other.ifLuid && self.filterId == other.filterId && self.layerId == other.layerId && self.reauthReason == other.reauthReason && self.originalProfile == other.originalProfile && self.currentProfile == other.currentProfile && self.msFwpDirection == other.msFwpDirection && self.isLoopback == other.isLoopback && self.vSwitchId == other.vSwitchId && self.vSwitchSourcePort == other.vSwitchSourcePort && self.vSwitchDestinationPort == other.vSwitchDestinationPort
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for FWPM_NET_EVENT_CLASSIFY_DROP_MAC0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_CLASSIFY_DROP_MAC0")
            .field("localMacAddr", &self.localMacAddr)
            .field("remoteMacAddr", &self.remoteMacAddr)
            .field("mediaType", &self.mediaType)
            .field("ifType", &self.ifType)
            .field("etherType", &self.etherType)
            .field("ndisPortNumber", &self.ndisPortNumber)
            .field("reserved", &self.reserved)
            .field("vlanTag", &self.vlanTag)
            .field("ifLuid", &self.ifLuid)
            .field("filterId", &self.filterId)
            .field("layerId", &self.layerId)
            .field("reauthReason", &self.reauthReason)
            .field("originalProfile", &self.originalProfile)
            .field("currentProfile", &self.currentProfile)
            .field("msFwpDirection", &self.msFwpDirection)
            .field("isLoopback", &self.isLoopback)
            .field("vSwitchId", &self.vSwitchId)
            .field("vSwitchSourcePort", &self.vSwitchSourcePort)
            .field("vSwitchDestinationPort", &self.vSwitchDestinationPort)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.startTime == other.startTime && self.endTime == other.endTime && self.numFilterConditions == other.numFilterConditions && self.filterCondition == other.filterCondition
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_ENUM_TEMPLATE0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWPM_NET_EVENT_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_ENUM_TEMPLATE0").field("startTime", &self.startTime).field("endTime", &self.endTime).field("numFilterConditions", &self.numFilterConditions).field("filterCondition", &self.filterCondition).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_HEADER3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    fn eq(&self, other: &Self) -> bool {
        self.failureErrorCode == other.failureErrorCode && self.failurePoint == other.failurePoint && self.flags == other.flags && self.emState == other.emState && self.saRole == other.saRole && self.emAuthMethod == other.emAuthMethod && self.endCertHash == other.endCertHash && self.mmId == other.mmId && self.qmFilterId == other.qmFilterId
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {}
impl ::core::fmt::Debug for FWPM_NET_EVENT_IKEEXT_EM_FAILURE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_IKEEXT_EM_FAILURE0").field("failureErrorCode", &self.failureErrorCode).field("failurePoint", &self.failurePoint).field("flags", &self.flags).field("emState", &self.emState).field("saRole", &self.saRole).field("emAuthMethod", &self.emAuthMethod).field("endCertHash", &self.endCertHash).field("mmId", &self.mmId).field("qmFilterId", &self.qmFilterId).finish()
    }
}
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    fn eq(&self, other: &Self) -> bool {
        self.failureErrorCode == other.failureErrorCode
            && self.failurePoint == other.failurePoint
            && self.flags == other.flags
            && self.emState == other.emState
            && self.saRole == other.saRole
            && self.emAuthMethod == other.emAuthMethod
            && self.endCertHash == other.endCertHash
            && self.mmId == other.mmId
            && self.qmFilterId == other.qmFilterId
            && self.localPrincipalNameForAuth == other.localPrincipalNameForAuth
            && self.remotePrincipalNameForAuth == other.remotePrincipalNameForAuth
            && self.numLocalPrincipalGroupSids == other.numLocalPrincipalGroupSids
            && self.localPrincipalGroupSids == other.localPrincipalGroupSids
            && self.numRemotePrincipalGroupSids == other.numRemotePrincipalGroupSids
            && self.remotePrincipalGroupSids == other.remotePrincipalGroupSids
            && self.saTrafficType == other.saTrafficType
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {}
impl ::core::fmt::Debug for FWPM_NET_EVENT_IKEEXT_EM_FAILURE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_IKEEXT_EM_FAILURE1")
            .field("failureErrorCode", &self.failureErrorCode)
            .field("failurePoint", &self.failurePoint)
            .field("flags", &self.flags)
            .field("emState", &self.emState)
            .field("saRole", &self.saRole)
            .field("emAuthMethod", &self.emAuthMethod)
            .field("endCertHash", &self.endCertHash)
            .field("mmId", &self.mmId)
            .field("qmFilterId", &self.qmFilterId)
            .field("localPrincipalNameForAuth", &self.localPrincipalNameForAuth)
            .field("remotePrincipalNameForAuth", &self.remotePrincipalNameForAuth)
            .field("numLocalPrincipalGroupSids", &self.numLocalPrincipalGroupSids)
            .field("localPrincipalGroupSids", &self.localPrincipalGroupSids)
            .field("numRemotePrincipalGroupSids", &self.numRemotePrincipalGroupSids)
            .field("remotePrincipalGroupSids", &self.remotePrincipalGroupSids)
            .field("saTrafficType", &self.saTrafficType)
            .finish()
    }
}
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    fn eq(&self, other: &Self) -> bool {
        self.failureErrorCode == other.failureErrorCode && self.failurePoint == other.failurePoint && self.flags == other.flags && self.keyingModuleType == other.keyingModuleType && self.mmState == other.mmState && self.saRole == other.saRole && self.mmAuthMethod == other.mmAuthMethod && self.endCertHash == other.endCertHash && self.mmId == other.mmId && self.mmFilterId == other.mmFilterId
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {}
impl ::core::fmt::Debug for FWPM_NET_EVENT_IKEEXT_MM_FAILURE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_IKEEXT_MM_FAILURE0").field("failureErrorCode", &self.failureErrorCode).field("failurePoint", &self.failurePoint).field("flags", &self.flags).field("keyingModuleType", &self.keyingModuleType).field("mmState", &self.mmState).field("saRole", &self.saRole).field("mmAuthMethod", &self.mmAuthMethod).field("endCertHash", &self.endCertHash).field("mmId", &self.mmId).field("mmFilterId", &self.mmFilterId).finish()
    }
}
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    fn eq(&self, other: &Self) -> bool {
        self.failureErrorCode == other.failureErrorCode
            && self.failurePoint == other.failurePoint
            && self.flags == other.flags
            && self.keyingModuleType == other.keyingModuleType
            && self.mmState == other.mmState
            && self.saRole == other.saRole
            && self.mmAuthMethod == other.mmAuthMethod
            && self.endCertHash == other.endCertHash
            && self.mmId == other.mmId
            && self.mmFilterId == other.mmFilterId
            && self.localPrincipalNameForAuth == other.localPrincipalNameForAuth
            && self.remotePrincipalNameForAuth == other.remotePrincipalNameForAuth
            && self.numLocalPrincipalGroupSids == other.numLocalPrincipalGroupSids
            && self.localPrincipalGroupSids == other.localPrincipalGroupSids
            && self.numRemotePrincipalGroupSids == other.numRemotePrincipalGroupSids
            && self.remotePrincipalGroupSids == other.remotePrincipalGroupSids
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {}
impl ::core::fmt::Debug for FWPM_NET_EVENT_IKEEXT_MM_FAILURE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_IKEEXT_MM_FAILURE1")
            .field("failureErrorCode", &self.failureErrorCode)
            .field("failurePoint", &self.failurePoint)
            .field("flags", &self.flags)
            .field("keyingModuleType", &self.keyingModuleType)
            .field("mmState", &self.mmState)
            .field("saRole", &self.saRole)
            .field("mmAuthMethod", &self.mmAuthMethod)
            .field("endCertHash", &self.endCertHash)
            .field("mmId", &self.mmId)
            .field("mmFilterId", &self.mmFilterId)
            .field("localPrincipalNameForAuth", &self.localPrincipalNameForAuth)
            .field("remotePrincipalNameForAuth", &self.remotePrincipalNameForAuth)
            .field("numLocalPrincipalGroupSids", &self.numLocalPrincipalGroupSids)
            .field("localPrincipalGroupSids", &self.localPrincipalGroupSids)
            .field("numRemotePrincipalGroupSids", &self.numRemotePrincipalGroupSids)
            .field("remotePrincipalGroupSids", &self.remotePrincipalGroupSids)
            .finish()
    }
}
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2 {
    fn eq(&self, other: &Self) -> bool {
        self.failureErrorCode == other.failureErrorCode
            && self.failurePoint == other.failurePoint
            && self.flags == other.flags
            && self.keyingModuleType == other.keyingModuleType
            && self.mmState == other.mmState
            && self.saRole == other.saRole
            && self.mmAuthMethod == other.mmAuthMethod
            && self.endCertHash == other.endCertHash
            && self.mmId == other.mmId
            && self.mmFilterId == other.mmFilterId
            && self.localPrincipalNameForAuth == other.localPrincipalNameForAuth
            && self.remotePrincipalNameForAuth == other.remotePrincipalNameForAuth
            && self.numLocalPrincipalGroupSids == other.numLocalPrincipalGroupSids
            && self.localPrincipalGroupSids == other.localPrincipalGroupSids
            && self.numRemotePrincipalGroupSids == other.numRemotePrincipalGroupSids
            && self.remotePrincipalGroupSids == other.remotePrincipalGroupSids
            && self.providerContextKey == other.providerContextKey
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2 {}
impl ::core::fmt::Debug for FWPM_NET_EVENT_IKEEXT_MM_FAILURE2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_IKEEXT_MM_FAILURE2")
            .field("failureErrorCode", &self.failureErrorCode)
            .field("failurePoint", &self.failurePoint)
            .field("flags", &self.flags)
            .field("keyingModuleType", &self.keyingModuleType)
            .field("mmState", &self.mmState)
            .field("saRole", &self.saRole)
            .field("mmAuthMethod", &self.mmAuthMethod)
            .field("endCertHash", &self.endCertHash)
            .field("mmId", &self.mmId)
            .field("mmFilterId", &self.mmFilterId)
            .field("localPrincipalNameForAuth", &self.localPrincipalNameForAuth)
            .field("remotePrincipalNameForAuth", &self.remotePrincipalNameForAuth)
            .field("numLocalPrincipalGroupSids", &self.numLocalPrincipalGroupSids)
            .field("localPrincipalGroupSids", &self.localPrincipalGroupSids)
            .field("numRemotePrincipalGroupSids", &self.numRemotePrincipalGroupSids)
            .field("remotePrincipalGroupSids", &self.remotePrincipalGroupSids)
            .field("providerContextKey", &self.providerContextKey)
            .finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_IKEEXT_QM_FAILURE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FWPM_NET_EVENT_IPSEC_DOSP_DROP0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    fn eq(&self, other: &Self) -> bool {
        self.failureStatus == other.failureStatus && self.direction == other.direction && self.spi == other.spi && self.filterId == other.filterId && self.layerId == other.layerId
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {}
impl ::core::fmt::Debug for FWPM_NET_EVENT_IPSEC_KERNEL_DROP0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_IPSEC_KERNEL_DROP0").field("failureStatus", &self.failureStatus).field("direction", &self.direction).field("spi", &self.spi).field("filterId", &self.filterId).field("layerId", &self.layerId).finish()
    }
}
impl ::core::default::Default for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0 {
    fn eq(&self, other: &Self) -> bool {
        self.spi == other.spi
    }
}
impl ::core::cmp::Eq for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0 {}
impl ::core::fmt::Debug for FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_LPM_PACKET_ARRIVAL0").field("spi", &self.spi).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_NET_EVENT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_NET_EVENT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate && self.flags == other.flags && self.sessionKey == other.sessionKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_NET_EVENT_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWPM_NET_EVENT_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_NET_EVENT_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
impl ::core::default::Default for FWPM_NET_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWPM_NET_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_NET_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FWPM_PROVIDER0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_PROVIDER0 {
    fn eq(&self, other: &Self) -> bool {
        self.providerKey == other.providerKey && self.displayData == other.displayData && self.flags == other.flags && self.providerData == other.providerData && self.serviceName == other.serviceName
    }
}
impl ::core::cmp::Eq for FWPM_PROVIDER0 {}
impl ::core::fmt::Debug for FWPM_PROVIDER0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_PROVIDER0").field("providerKey", &self.providerKey).field("displayData", &self.displayData).field("flags", &self.flags).field("providerData", &self.providerData).field("serviceName", &self.serviceName).finish()
    }
}
impl ::core::default::Default for FWPM_PROVIDER_CHANGE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        self.changeType == other.changeType && self.providerKey == other.providerKey
    }
}
impl ::core::cmp::Eq for FWPM_PROVIDER_CHANGE0 {}
impl ::core::fmt::Debug for FWPM_PROVIDER_CHANGE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_PROVIDER_CHANGE0").field("changeType", &self.changeType).field("providerKey", &self.providerKey).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT_CHANGE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CONTEXT_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        self.changeType == other.changeType && self.providerContextKey == other.providerContextKey && self.providerContextId == other.providerContextId
    }
}
impl ::core::cmp::Eq for FWPM_PROVIDER_CONTEXT_CHANGE0 {}
impl ::core::fmt::Debug for FWPM_PROVIDER_CONTEXT_CHANGE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_PROVIDER_CONTEXT_CHANGE0").field("changeType", &self.changeType).field("providerContextKey", &self.providerContextKey).field("providerContextId", &self.providerContextId).finish()
    }
}
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.providerKey == other.providerKey && self.providerContextType == other.providerContextType
    }
}
impl ::core::cmp::Eq for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {}
impl ::core::fmt::Debug for FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_PROVIDER_CONTEXT_ENUM_TEMPLATE0").field("providerKey", &self.providerKey).field("providerContextType", &self.providerContextType).finish()
    }
}
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate && self.flags == other.flags && self.sessionKey == other.sessionKey
    }
}
impl ::core::cmp::Eq for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {}
impl ::core::fmt::Debug for FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_PROVIDER_CONTEXT_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
impl ::core::default::Default for FWPM_PROVIDER_CONTEXT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWPM_PROVIDER_CONTEXT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_PROVIDER_CONTEXT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FWPM_PROVIDER_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_PROVIDER_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for FWPM_PROVIDER_ENUM_TEMPLATE0 {}
impl ::core::fmt::Debug for FWPM_PROVIDER_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_PROVIDER_ENUM_TEMPLATE0").field("reserved", &self.reserved).finish()
    }
}
impl ::core::default::Default for FWPM_PROVIDER_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_PROVIDER_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate && self.flags == other.flags && self.sessionKey == other.sessionKey
    }
}
impl ::core::cmp::Eq for FWPM_PROVIDER_SUBSCRIPTION0 {}
impl ::core::fmt::Debug for FWPM_PROVIDER_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_PROVIDER_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
impl ::core::default::Default for FWPM_SERVICE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWPM_SERVICE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_SERVICE_STATE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWPM_SESSION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWPM_SESSION0 {
    fn eq(&self, other: &Self) -> bool {
        self.sessionKey == other.sessionKey && self.displayData == other.displayData && self.flags == other.flags && self.txnWaitTimeoutInMSec == other.txnWaitTimeoutInMSec && self.processId == other.processId && self.sid == other.sid && self.username == other.username && self.kernelMode == other.kernelMode
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWPM_SESSION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWPM_SESSION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SESSION0").field("sessionKey", &self.sessionKey).field("displayData", &self.displayData).field("flags", &self.flags).field("txnWaitTimeoutInMSec", &self.txnWaitTimeoutInMSec).field("processId", &self.processId).field("sid", &self.sid).field("username", &self.username).field("kernelMode", &self.kernelMode).finish()
    }
}
impl ::core::default::Default for FWPM_SESSION_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_SESSION_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.reserved == other.reserved
    }
}
impl ::core::cmp::Eq for FWPM_SESSION_ENUM_TEMPLATE0 {}
impl ::core::fmt::Debug for FWPM_SESSION_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SESSION_ENUM_TEMPLATE0").field("reserved", &self.reserved).finish()
    }
}
impl ::core::default::Default for FWPM_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.numLayerStatistics == other.numLayerStatistics
            && self.layerStatistics == other.layerStatistics
            && self.inboundAllowedConnectionsV4 == other.inboundAllowedConnectionsV4
            && self.inboundBlockedConnectionsV4 == other.inboundBlockedConnectionsV4
            && self.outboundAllowedConnectionsV4 == other.outboundAllowedConnectionsV4
            && self.outboundBlockedConnectionsV4 == other.outboundBlockedConnectionsV4
            && self.inboundAllowedConnectionsV6 == other.inboundAllowedConnectionsV6
            && self.inboundBlockedConnectionsV6 == other.inboundBlockedConnectionsV6
            && self.outboundAllowedConnectionsV6 == other.outboundAllowedConnectionsV6
            && self.outboundBlockedConnectionsV6 == other.outboundBlockedConnectionsV6
            && self.inboundActiveConnectionsV4 == other.inboundActiveConnectionsV4
            && self.outboundActiveConnectionsV4 == other.outboundActiveConnectionsV4
            && self.inboundActiveConnectionsV6 == other.inboundActiveConnectionsV6
            && self.outboundActiveConnectionsV6 == other.outboundActiveConnectionsV6
            && self.reauthDirInbound == other.reauthDirInbound
            && self.reauthDirOutbound == other.reauthDirOutbound
            && self.reauthFamilyV4 == other.reauthFamilyV4
            && self.reauthFamilyV6 == other.reauthFamilyV6
            && self.reauthProtoOther == other.reauthProtoOther
            && self.reauthProtoIPv4 == other.reauthProtoIPv4
            && self.reauthProtoIPv6 == other.reauthProtoIPv6
            && self.reauthProtoICMP == other.reauthProtoICMP
            && self.reauthProtoICMP6 == other.reauthProtoICMP6
            && self.reauthProtoUDP == other.reauthProtoUDP
            && self.reauthProtoTCP == other.reauthProtoTCP
            && self.reauthReasonPolicyChange == other.reauthReasonPolicyChange
            && self.reauthReasonNewArrivalInterface == other.reauthReasonNewArrivalInterface
            && self.reauthReasonNewNextHopInterface == other.reauthReasonNewNextHopInterface
            && self.reauthReasonProfileCrossing == other.reauthReasonProfileCrossing
            && self.reauthReasonClassifyCompletion == other.reauthReasonClassifyCompletion
            && self.reauthReasonIPSecPropertiesChanged == other.reauthReasonIPSecPropertiesChanged
            && self.reauthReasonMidStreamInspection == other.reauthReasonMidStreamInspection
            && self.reauthReasonSocketPropertyChanged == other.reauthReasonSocketPropertyChanged
            && self.reauthReasonNewInboundMCastBCastPacket == other.reauthReasonNewInboundMCastBCastPacket
            && self.reauthReasonEDPPolicyChanged == other.reauthReasonEDPPolicyChanged
            && self.reauthReasonProxyHandleChanged == other.reauthReasonProxyHandleChanged
    }
}
impl ::core::cmp::Eq for FWPM_STATISTICS0 {}
impl ::core::fmt::Debug for FWPM_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_STATISTICS0")
            .field("numLayerStatistics", &self.numLayerStatistics)
            .field("layerStatistics", &self.layerStatistics)
            .field("inboundAllowedConnectionsV4", &self.inboundAllowedConnectionsV4)
            .field("inboundBlockedConnectionsV4", &self.inboundBlockedConnectionsV4)
            .field("outboundAllowedConnectionsV4", &self.outboundAllowedConnectionsV4)
            .field("outboundBlockedConnectionsV4", &self.outboundBlockedConnectionsV4)
            .field("inboundAllowedConnectionsV6", &self.inboundAllowedConnectionsV6)
            .field("inboundBlockedConnectionsV6", &self.inboundBlockedConnectionsV6)
            .field("outboundAllowedConnectionsV6", &self.outboundAllowedConnectionsV6)
            .field("outboundBlockedConnectionsV6", &self.outboundBlockedConnectionsV6)
            .field("inboundActiveConnectionsV4", &self.inboundActiveConnectionsV4)
            .field("outboundActiveConnectionsV4", &self.outboundActiveConnectionsV4)
            .field("inboundActiveConnectionsV6", &self.inboundActiveConnectionsV6)
            .field("outboundActiveConnectionsV6", &self.outboundActiveConnectionsV6)
            .field("reauthDirInbound", &self.reauthDirInbound)
            .field("reauthDirOutbound", &self.reauthDirOutbound)
            .field("reauthFamilyV4", &self.reauthFamilyV4)
            .field("reauthFamilyV6", &self.reauthFamilyV6)
            .field("reauthProtoOther", &self.reauthProtoOther)
            .field("reauthProtoIPv4", &self.reauthProtoIPv4)
            .field("reauthProtoIPv6", &self.reauthProtoIPv6)
            .field("reauthProtoICMP", &self.reauthProtoICMP)
            .field("reauthProtoICMP6", &self.reauthProtoICMP6)
            .field("reauthProtoUDP", &self.reauthProtoUDP)
            .field("reauthProtoTCP", &self.reauthProtoTCP)
            .field("reauthReasonPolicyChange", &self.reauthReasonPolicyChange)
            .field("reauthReasonNewArrivalInterface", &self.reauthReasonNewArrivalInterface)
            .field("reauthReasonNewNextHopInterface", &self.reauthReasonNewNextHopInterface)
            .field("reauthReasonProfileCrossing", &self.reauthReasonProfileCrossing)
            .field("reauthReasonClassifyCompletion", &self.reauthReasonClassifyCompletion)
            .field("reauthReasonIPSecPropertiesChanged", &self.reauthReasonIPSecPropertiesChanged)
            .field("reauthReasonMidStreamInspection", &self.reauthReasonMidStreamInspection)
            .field("reauthReasonSocketPropertyChanged", &self.reauthReasonSocketPropertyChanged)
            .field("reauthReasonNewInboundMCastBCastPacket", &self.reauthReasonNewInboundMCastBCastPacket)
            .field("reauthReasonEDPPolicyChanged", &self.reauthReasonEDPPolicyChanged)
            .field("reauthReasonProxyHandleChanged", &self.reauthReasonProxyHandleChanged)
            .finish()
    }
}
impl ::core::default::Default for FWPM_SUBLAYER0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_SUBLAYER0 {
    fn eq(&self, other: &Self) -> bool {
        self.subLayerKey == other.subLayerKey && self.displayData == other.displayData && self.flags == other.flags && self.providerKey == other.providerKey && self.providerData == other.providerData && self.weight == other.weight
    }
}
impl ::core::cmp::Eq for FWPM_SUBLAYER0 {}
impl ::core::fmt::Debug for FWPM_SUBLAYER0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SUBLAYER0").field("subLayerKey", &self.subLayerKey).field("displayData", &self.displayData).field("flags", &self.flags).field("providerKey", &self.providerKey).field("providerData", &self.providerData).field("weight", &self.weight).finish()
    }
}
impl ::core::default::Default for FWPM_SUBLAYER_CHANGE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_SUBLAYER_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        self.changeType == other.changeType && self.subLayerKey == other.subLayerKey
    }
}
impl ::core::cmp::Eq for FWPM_SUBLAYER_CHANGE0 {}
impl ::core::fmt::Debug for FWPM_SUBLAYER_CHANGE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SUBLAYER_CHANGE0").field("changeType", &self.changeType).field("subLayerKey", &self.subLayerKey).finish()
    }
}
impl ::core::default::Default for FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.providerKey == other.providerKey
    }
}
impl ::core::cmp::Eq for FWPM_SUBLAYER_ENUM_TEMPLATE0 {}
impl ::core::fmt::Debug for FWPM_SUBLAYER_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SUBLAYER_ENUM_TEMPLATE0").field("providerKey", &self.providerKey).finish()
    }
}
impl ::core::default::Default for FWPM_SUBLAYER_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_SUBLAYER_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate && self.flags == other.flags && self.sessionKey == other.sessionKey
    }
}
impl ::core::cmp::Eq for FWPM_SUBLAYER_SUBSCRIPTION0 {}
impl ::core::fmt::Debug for FWPM_SUBLAYER_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SUBLAYER_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
impl ::core::default::Default for FWPM_SUBSCRIPTION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWPM_SUBSCRIPTION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_SUBSCRIPTION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for FWPM_SYSTEM_PORTS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_SYSTEM_PORTS0 {
    fn eq(&self, other: &Self) -> bool {
        self.numTypes == other.numTypes && self.types == other.types
    }
}
impl ::core::cmp::Eq for FWPM_SYSTEM_PORTS0 {}
impl ::core::fmt::Debug for FWPM_SYSTEM_PORTS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SYSTEM_PORTS0").field("numTypes", &self.numTypes).field("types", &self.types).finish()
    }
}
impl ::core::default::Default for FWPM_SYSTEM_PORTS_BY_TYPE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_SYSTEM_PORTS_BY_TYPE0 {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.numPorts == other.numPorts && self.ports == other.ports
    }
}
impl ::core::cmp::Eq for FWPM_SYSTEM_PORTS_BY_TYPE0 {}
impl ::core::fmt::Debug for FWPM_SYSTEM_PORTS_BY_TYPE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_SYSTEM_PORTS_BY_TYPE0").field("type", &self.r#type).field("numPorts", &self.numPorts).field("ports", &self.ports).finish()
    }
}
impl ::core::default::Default for FWPM_SYSTEM_PORT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWPM_SYSTEM_PORT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_SYSTEM_PORT_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for FWPM_VSWITCH_EVENT0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.sessionKey == other.sessionKey
    }
}
impl ::core::cmp::Eq for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {}
impl ::core::fmt::Debug for FWPM_VSWITCH_EVENT_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWPM_VSWITCH_EVENT_SUBSCRIPTION0").field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
impl ::core::default::Default for FWPM_VSWITCH_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWPM_VSWITCH_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWPM_VSWITCH_EVENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FWP_AF {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWP_AF {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_AF").field(&self.0).finish()
    }
}
impl ::core::default::Default for FWP_BYTE_ARRAY16 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWP_BYTE_ARRAY16 {
    fn eq(&self, other: &Self) -> bool {
        self.byteArray16 == other.byteArray16
    }
}
impl ::core::cmp::Eq for FWP_BYTE_ARRAY16 {}
impl ::core::fmt::Debug for FWP_BYTE_ARRAY16 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWP_BYTE_ARRAY16").field("byteArray16", &self.byteArray16).finish()
    }
}
impl ::core::default::Default for FWP_BYTE_ARRAY6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWP_BYTE_ARRAY6 {
    fn eq(&self, other: &Self) -> bool {
        self.byteArray6 == other.byteArray6
    }
}
impl ::core::cmp::Eq for FWP_BYTE_ARRAY6 {}
impl ::core::fmt::Debug for FWP_BYTE_ARRAY6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWP_BYTE_ARRAY6").field("byteArray6", &self.byteArray6).finish()
    }
}
impl ::core::default::Default for FWP_BYTE_BLOB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWP_BYTE_BLOB {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size && self.data == other.data
    }
}
impl ::core::cmp::Eq for FWP_BYTE_BLOB {}
impl ::core::fmt::Debug for FWP_BYTE_BLOB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWP_BYTE_BLOB").field("size", &self.size).field("data", &self.data).finish()
    }
}
impl ::core::default::Default for FWP_CLASSIFY_OPTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWP_CLASSIFY_OPTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_CLASSIFY_OPTION_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWP_CONDITION_VALUE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FWP_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWP_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FWP_DIRECTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWP_DIRECTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_DIRECTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for FWP_ETHER_ENCAP_METHOD {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWP_ETHER_ENCAP_METHOD {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_ETHER_ENCAP_METHOD").field(&self.0).finish()
    }
}
impl ::core::default::Default for FWP_FILTER_ENUM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWP_FILTER_ENUM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_FILTER_ENUM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for FWP_IP_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWP_IP_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_IP_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for FWP_MATCH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWP_MATCH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_MATCH_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWP_RANGE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWP_TOKEN_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for FWP_TOKEN_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.sidCount == other.sidCount && self.sids == other.sids && self.restrictedSidCount == other.restrictedSidCount && self.restrictedSids == other.restrictedSids
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for FWP_TOKEN_INFORMATION {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for FWP_TOKEN_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWP_TOKEN_INFORMATION").field("sidCount", &self.sidCount).field("sids", &self.sids).field("restrictedSidCount", &self.restrictedSidCount).field("restrictedSids", &self.restrictedSids).finish()
    }
}
impl ::core::default::Default for FWP_V4_ADDR_AND_MASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWP_V4_ADDR_AND_MASK {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr && self.mask == other.mask
    }
}
impl ::core::cmp::Eq for FWP_V4_ADDR_AND_MASK {}
impl ::core::fmt::Debug for FWP_V4_ADDR_AND_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWP_V4_ADDR_AND_MASK").field("addr", &self.addr).field("mask", &self.mask).finish()
    }
}
impl ::core::default::Default for FWP_V6_ADDR_AND_MASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for FWP_V6_ADDR_AND_MASK {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr && self.prefixLength == other.prefixLength
    }
}
impl ::core::cmp::Eq for FWP_V6_ADDR_AND_MASK {}
impl ::core::fmt::Debug for FWP_V6_ADDR_AND_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("FWP_V6_ADDR_AND_MASK").field("addr", &self.addr).field("prefixLength", &self.prefixLength).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for FWP_VALUE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for FWP_VSWITCH_NETWORK_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for FWP_VSWITCH_NETWORK_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("FWP_VSWITCH_NETWORK_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_AUTHENTICATION_IMPERSONATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKEEXT_AUTHENTICATION_METHOD0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_AUTHENTICATION_METHOD1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_AUTHENTICATION_METHOD2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_AUTHENTICATION_METHOD_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_AUTHENTICATION_METHOD_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_AUTHENTICATION_METHOD_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_CERTIFICATE_AUTHENTICATION2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_CERTIFICATE_CREDENTIAL0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_CREDENTIAL0 {
    fn eq(&self, other: &Self) -> bool {
        self.subjectName == other.subjectName && self.certHash == other.certHash && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_CREDENTIAL0 {}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_CREDENTIAL0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_CREDENTIAL0").field("subjectName", &self.subjectName).field("certHash", &self.certHash).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for IKEEXT_CERTIFICATE_CREDENTIAL1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_CREDENTIAL1 {
    fn eq(&self, other: &Self) -> bool {
        self.subjectName == other.subjectName && self.certHash == other.certHash && self.flags == other.flags && self.certificate == other.certificate
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_CREDENTIAL1 {}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_CREDENTIAL1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_CREDENTIAL1").field("subjectName", &self.subjectName).field("certHash", &self.certHash).field("flags", &self.flags).field("certificate", &self.certificate).finish()
    }
}
impl ::core::default::Default for IKEEXT_CERTIFICATE_CRITERIA0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_CERTIFICATE_CRITERIA0 {
    fn eq(&self, other: &Self) -> bool {
        self.certData == other.certData && self.certHash == other.certHash && self.eku == other.eku && self.name == other.name && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for IKEEXT_CERTIFICATE_CRITERIA0 {}
impl ::core::fmt::Debug for IKEEXT_CERTIFICATE_CRITERIA0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERTIFICATE_CRITERIA0").field("certData", &self.certData).field("certHash", &self.certHash).field("eku", &self.eku).field("name", &self.name).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for IKEEXT_CERT_AUTH {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_CERT_AUTH {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_CERT_AUTH").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IKEEXT_CERT_AUTH {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IKEEXT_CERT_AUTH {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IKEEXT_CERT_AUTH {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IKEEXT_CERT_AUTH {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IKEEXT_CERT_AUTH {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IKEEXT_CERT_CONFIG_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_CERT_CONFIG_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_CERT_CONFIG_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKEEXT_CERT_CRITERIA_NAME_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_CERT_CRITERIA_NAME_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_CERT_CRITERIA_NAME_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKEEXT_CERT_EKUS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_CERT_EKUS0 {
    fn eq(&self, other: &Self) -> bool {
        self.numEku == other.numEku && self.eku == other.eku
    }
}
impl ::core::cmp::Eq for IKEEXT_CERT_EKUS0 {}
impl ::core::fmt::Debug for IKEEXT_CERT_EKUS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERT_EKUS0").field("numEku", &self.numEku).field("eku", &self.eku).finish()
    }
}
impl ::core::default::Default for IKEEXT_CERT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_CERT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_CERT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IKEEXT_CERT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IKEEXT_CERT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IKEEXT_CERT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IKEEXT_CERT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IKEEXT_CERT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IKEEXT_CERT_NAME0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_CERT_NAME0 {
    fn eq(&self, other: &Self) -> bool {
        self.nameType == other.nameType && self.certName == other.certName
    }
}
impl ::core::cmp::Eq for IKEEXT_CERT_NAME0 {}
impl ::core::fmt::Debug for IKEEXT_CERT_NAME0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERT_NAME0").field("nameType", &self.nameType).field("certName", &self.certName).finish()
    }
}
impl ::core::default::Default for IKEEXT_CERT_ROOT_CONFIG0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_CERT_ROOT_CONFIG0 {
    fn eq(&self, other: &Self) -> bool {
        self.certData == other.certData && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for IKEEXT_CERT_ROOT_CONFIG0 {}
impl ::core::fmt::Debug for IKEEXT_CERT_ROOT_CONFIG0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CERT_ROOT_CONFIG0").field("certData", &self.certData).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for IKEEXT_CIPHER_ALGORITHM0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_CIPHER_ALGORITHM0 {
    fn eq(&self, other: &Self) -> bool {
        self.algoIdentifier == other.algoIdentifier && self.keyLen == other.keyLen && self.rounds == other.rounds
    }
}
impl ::core::cmp::Eq for IKEEXT_CIPHER_ALGORITHM0 {}
impl ::core::fmt::Debug for IKEEXT_CIPHER_ALGORITHM0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CIPHER_ALGORITHM0").field("algoIdentifier", &self.algoIdentifier).field("keyLen", &self.keyLen).field("rounds", &self.rounds).finish()
    }
}
impl ::core::default::Default for IKEEXT_CIPHER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_CIPHER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_CIPHER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKEEXT_COMMON_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_COMMON_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.v4Statistics == other.v4Statistics && self.v6Statistics == other.v6Statistics && self.totalPacketsReceived == other.totalPacketsReceived && self.totalInvalidPacketsReceived == other.totalInvalidPacketsReceived && self.currentQueuedWorkitems == other.currentQueuedWorkitems
    }
}
impl ::core::cmp::Eq for IKEEXT_COMMON_STATISTICS0 {}
impl ::core::fmt::Debug for IKEEXT_COMMON_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_COMMON_STATISTICS0").field("v4Statistics", &self.v4Statistics).field("v6Statistics", &self.v6Statistics).field("totalPacketsReceived", &self.totalPacketsReceived).field("totalInvalidPacketsReceived", &self.totalInvalidPacketsReceived).field("currentQueuedWorkitems", &self.currentQueuedWorkitems).finish()
    }
}
impl ::core::default::Default for IKEEXT_COMMON_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_COMMON_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.v4Statistics == other.v4Statistics && self.v6Statistics == other.v6Statistics && self.totalPacketsReceived == other.totalPacketsReceived && self.totalInvalidPacketsReceived == other.totalInvalidPacketsReceived && self.currentQueuedWorkitems == other.currentQueuedWorkitems
    }
}
impl ::core::cmp::Eq for IKEEXT_COMMON_STATISTICS1 {}
impl ::core::fmt::Debug for IKEEXT_COMMON_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_COMMON_STATISTICS1").field("v4Statistics", &self.v4Statistics).field("v6Statistics", &self.v6Statistics).field("totalPacketsReceived", &self.totalPacketsReceived).field("totalInvalidPacketsReceived", &self.totalInvalidPacketsReceived).field("currentQueuedWorkitems", &self.currentQueuedWorkitems).finish()
    }
}
impl ::core::default::Default for IKEEXT_COOKIE_PAIR0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_COOKIE_PAIR0 {
    fn eq(&self, other: &Self) -> bool {
        self.initiator == other.initiator && self.responder == other.responder
    }
}
impl ::core::cmp::Eq for IKEEXT_COOKIE_PAIR0 {}
impl ::core::fmt::Debug for IKEEXT_COOKIE_PAIR0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_COOKIE_PAIR0").field("initiator", &self.initiator).field("responder", &self.responder).finish()
    }
}
impl ::core::default::Default for IKEEXT_CREDENTIAL0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_CREDENTIAL1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_CREDENTIAL2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_CREDENTIALS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIALS0 {
    fn eq(&self, other: &Self) -> bool {
        self.numCredentials == other.numCredentials && self.credentials == other.credentials
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIALS0 {}
impl ::core::fmt::Debug for IKEEXT_CREDENTIALS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CREDENTIALS0").field("numCredentials", &self.numCredentials).field("credentials", &self.credentials).finish()
    }
}
impl ::core::default::Default for IKEEXT_CREDENTIALS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIALS1 {
    fn eq(&self, other: &Self) -> bool {
        self.numCredentials == other.numCredentials && self.credentials == other.credentials
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIALS1 {}
impl ::core::fmt::Debug for IKEEXT_CREDENTIALS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CREDENTIALS1").field("numCredentials", &self.numCredentials).field("credentials", &self.credentials).finish()
    }
}
impl ::core::default::Default for IKEEXT_CREDENTIALS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_CREDENTIALS2 {
    fn eq(&self, other: &Self) -> bool {
        self.numCredentials == other.numCredentials && self.credentials == other.credentials
    }
}
impl ::core::cmp::Eq for IKEEXT_CREDENTIALS2 {}
impl ::core::fmt::Debug for IKEEXT_CREDENTIALS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_CREDENTIALS2").field("numCredentials", &self.numCredentials).field("credentials", &self.credentials).finish()
    }
}
impl ::core::default::Default for IKEEXT_CREDENTIAL_PAIR0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_CREDENTIAL_PAIR1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_CREDENTIAL_PAIR2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_DH_GROUP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_DH_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_DH_GROUP").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKEEXT_EAP_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_EAP_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
    }
}
impl ::core::cmp::Eq for IKEEXT_EAP_AUTHENTICATION0 {}
impl ::core::fmt::Debug for IKEEXT_EAP_AUTHENTICATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_EAP_AUTHENTICATION0").field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_EAP_AUTHENTICATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IKEEXT_EAP_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IKEEXT_EM_POLICY0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_EM_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        self.numAuthenticationMethods == other.numAuthenticationMethods && self.authenticationMethods == other.authenticationMethods && self.initiatorImpersonationType == other.initiatorImpersonationType
    }
}
impl ::core::cmp::Eq for IKEEXT_EM_POLICY0 {}
impl ::core::fmt::Debug for IKEEXT_EM_POLICY0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_EM_POLICY0").field("numAuthenticationMethods", &self.numAuthenticationMethods).field("authenticationMethods", &self.authenticationMethods).field("initiatorImpersonationType", &self.initiatorImpersonationType).finish()
    }
}
impl ::core::default::Default for IKEEXT_EM_POLICY1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_EM_POLICY1 {
    fn eq(&self, other: &Self) -> bool {
        self.numAuthenticationMethods == other.numAuthenticationMethods && self.authenticationMethods == other.authenticationMethods && self.initiatorImpersonationType == other.initiatorImpersonationType
    }
}
impl ::core::cmp::Eq for IKEEXT_EM_POLICY1 {}
impl ::core::fmt::Debug for IKEEXT_EM_POLICY1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_EM_POLICY1").field("numAuthenticationMethods", &self.numAuthenticationMethods).field("authenticationMethods", &self.authenticationMethods).field("initiatorImpersonationType", &self.initiatorImpersonationType).finish()
    }
}
impl ::core::default::Default for IKEEXT_EM_POLICY2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_EM_POLICY2 {
    fn eq(&self, other: &Self) -> bool {
        self.numAuthenticationMethods == other.numAuthenticationMethods && self.authenticationMethods == other.authenticationMethods && self.initiatorImpersonationType == other.initiatorImpersonationType
    }
}
impl ::core::cmp::Eq for IKEEXT_EM_POLICY2 {}
impl ::core::fmt::Debug for IKEEXT_EM_POLICY2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_EM_POLICY2").field("numAuthenticationMethods", &self.numAuthenticationMethods).field("authenticationMethods", &self.authenticationMethods).field("initiatorImpersonationType", &self.initiatorImpersonationType).finish()
    }
}
impl ::core::default::Default for IKEEXT_EM_SA_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_EM_SA_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_EM_SA_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKEEXT_INTEGRITY_ALGORITHM0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_INTEGRITY_ALGORITHM0 {
    fn eq(&self, other: &Self) -> bool {
        self.algoIdentifier == other.algoIdentifier
    }
}
impl ::core::cmp::Eq for IKEEXT_INTEGRITY_ALGORITHM0 {}
impl ::core::fmt::Debug for IKEEXT_INTEGRITY_ALGORITHM0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_INTEGRITY_ALGORITHM0").field("algoIdentifier", &self.algoIdentifier).finish()
    }
}
impl ::core::default::Default for IKEEXT_INTEGRITY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_INTEGRITY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_INTEGRITY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.keyContainerName == other.keyContainerName && self.cspName == other.cspName && self.cspType == other.cspType && self.cgaModifier == other.cgaModifier && self.cgaCollisionCount == other.cgaCollisionCount
    }
}
impl ::core::cmp::Eq for IKEEXT_IPV6_CGA_AUTHENTICATION0 {}
impl ::core::fmt::Debug for IKEEXT_IPV6_CGA_AUTHENTICATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_IPV6_CGA_AUTHENTICATION0").field("keyContainerName", &self.keyContainerName).field("cspName", &self.cspName).field("cspType", &self.cspType).field("cgaModifier", &self.cgaModifier).field("cgaCollisionCount", &self.cgaCollisionCount).finish()
    }
}
impl ::core::default::Default for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.totalSocketReceiveFailures == other.totalSocketReceiveFailures && self.totalSocketSendFailures == other.totalSocketSendFailures
    }
}
impl ::core::cmp::Eq for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {}
impl ::core::fmt::Debug for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS0").field("totalSocketReceiveFailures", &self.totalSocketReceiveFailures).field("totalSocketSendFailures", &self.totalSocketSendFailures).finish()
    }
}
impl ::core::default::Default for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.totalSocketReceiveFailures == other.totalSocketReceiveFailures && self.totalSocketSendFailures == other.totalSocketSendFailures
    }
}
impl ::core::cmp::Eq for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {}
impl ::core::fmt::Debug for IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_IP_VERSION_SPECIFIC_COMMON_STATISTICS1").field("totalSocketReceiveFailures", &self.totalSocketReceiveFailures).field("totalSocketSendFailures", &self.totalSocketSendFailures).finish()
    }
}
impl ::core::default::Default for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.currentActiveMainModes == other.currentActiveMainModes
            && self.totalMainModesStarted == other.totalMainModesStarted
            && self.totalSuccessfulMainModes == other.totalSuccessfulMainModes
            && self.totalFailedMainModes == other.totalFailedMainModes
            && self.totalResponderMainModes == other.totalResponderMainModes
            && self.currentNewResponderMainModes == other.currentNewResponderMainModes
            && self.currentActiveQuickModes == other.currentActiveQuickModes
            && self.totalQuickModesStarted == other.totalQuickModesStarted
            && self.totalSuccessfulQuickModes == other.totalSuccessfulQuickModes
            && self.totalFailedQuickModes == other.totalFailedQuickModes
            && self.totalAcquires == other.totalAcquires
            && self.totalReinitAcquires == other.totalReinitAcquires
            && self.currentActiveExtendedModes == other.currentActiveExtendedModes
            && self.totalExtendedModesStarted == other.totalExtendedModesStarted
            && self.totalSuccessfulExtendedModes == other.totalSuccessfulExtendedModes
            && self.totalFailedExtendedModes == other.totalFailedExtendedModes
            && self.totalImpersonationExtendedModes == other.totalImpersonationExtendedModes
            && self.totalImpersonationMainModes == other.totalImpersonationMainModes
    }
}
impl ::core::cmp::Eq for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {}
impl ::core::fmt::Debug for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS0")
            .field("currentActiveMainModes", &self.currentActiveMainModes)
            .field("totalMainModesStarted", &self.totalMainModesStarted)
            .field("totalSuccessfulMainModes", &self.totalSuccessfulMainModes)
            .field("totalFailedMainModes", &self.totalFailedMainModes)
            .field("totalResponderMainModes", &self.totalResponderMainModes)
            .field("currentNewResponderMainModes", &self.currentNewResponderMainModes)
            .field("currentActiveQuickModes", &self.currentActiveQuickModes)
            .field("totalQuickModesStarted", &self.totalQuickModesStarted)
            .field("totalSuccessfulQuickModes", &self.totalSuccessfulQuickModes)
            .field("totalFailedQuickModes", &self.totalFailedQuickModes)
            .field("totalAcquires", &self.totalAcquires)
            .field("totalReinitAcquires", &self.totalReinitAcquires)
            .field("currentActiveExtendedModes", &self.currentActiveExtendedModes)
            .field("totalExtendedModesStarted", &self.totalExtendedModesStarted)
            .field("totalSuccessfulExtendedModes", &self.totalSuccessfulExtendedModes)
            .field("totalFailedExtendedModes", &self.totalFailedExtendedModes)
            .field("totalImpersonationExtendedModes", &self.totalImpersonationExtendedModes)
            .field("totalImpersonationMainModes", &self.totalImpersonationMainModes)
            .finish()
    }
}
impl ::core::default::Default for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.currentActiveMainModes == other.currentActiveMainModes
            && self.totalMainModesStarted == other.totalMainModesStarted
            && self.totalSuccessfulMainModes == other.totalSuccessfulMainModes
            && self.totalFailedMainModes == other.totalFailedMainModes
            && self.totalResponderMainModes == other.totalResponderMainModes
            && self.currentNewResponderMainModes == other.currentNewResponderMainModes
            && self.currentActiveQuickModes == other.currentActiveQuickModes
            && self.totalQuickModesStarted == other.totalQuickModesStarted
            && self.totalSuccessfulQuickModes == other.totalSuccessfulQuickModes
            && self.totalFailedQuickModes == other.totalFailedQuickModes
            && self.totalAcquires == other.totalAcquires
            && self.totalReinitAcquires == other.totalReinitAcquires
            && self.currentActiveExtendedModes == other.currentActiveExtendedModes
            && self.totalExtendedModesStarted == other.totalExtendedModesStarted
            && self.totalSuccessfulExtendedModes == other.totalSuccessfulExtendedModes
            && self.totalFailedExtendedModes == other.totalFailedExtendedModes
            && self.totalImpersonationExtendedModes == other.totalImpersonationExtendedModes
            && self.totalImpersonationMainModes == other.totalImpersonationMainModes
    }
}
impl ::core::cmp::Eq for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {}
impl ::core::fmt::Debug for IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_IP_VERSION_SPECIFIC_KEYMODULE_STATISTICS1")
            .field("currentActiveMainModes", &self.currentActiveMainModes)
            .field("totalMainModesStarted", &self.totalMainModesStarted)
            .field("totalSuccessfulMainModes", &self.totalSuccessfulMainModes)
            .field("totalFailedMainModes", &self.totalFailedMainModes)
            .field("totalResponderMainModes", &self.totalResponderMainModes)
            .field("currentNewResponderMainModes", &self.currentNewResponderMainModes)
            .field("currentActiveQuickModes", &self.currentActiveQuickModes)
            .field("totalQuickModesStarted", &self.totalQuickModesStarted)
            .field("totalSuccessfulQuickModes", &self.totalSuccessfulQuickModes)
            .field("totalFailedQuickModes", &self.totalFailedQuickModes)
            .field("totalAcquires", &self.totalAcquires)
            .field("totalReinitAcquires", &self.totalReinitAcquires)
            .field("currentActiveExtendedModes", &self.currentActiveExtendedModes)
            .field("totalExtendedModesStarted", &self.totalExtendedModesStarted)
            .field("totalSuccessfulExtendedModes", &self.totalSuccessfulExtendedModes)
            .field("totalFailedExtendedModes", &self.totalFailedExtendedModes)
            .field("totalImpersonationExtendedModes", &self.totalImpersonationExtendedModes)
            .field("totalImpersonationMainModes", &self.totalImpersonationMainModes)
            .finish()
    }
}
impl ::core::default::Default for IKEEXT_KERBEROS_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_KERBEROS_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
    }
}
impl ::core::cmp::Eq for IKEEXT_KERBEROS_AUTHENTICATION0 {}
impl ::core::fmt::Debug for IKEEXT_KERBEROS_AUTHENTICATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_KERBEROS_AUTHENTICATION0").field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for IKEEXT_KERBEROS_AUTHENTICATION1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_KERBEROS_AUTHENTICATION1 {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.proxyServer == other.proxyServer
    }
}
impl ::core::cmp::Eq for IKEEXT_KERBEROS_AUTHENTICATION1 {}
impl ::core::fmt::Debug for IKEEXT_KERBEROS_AUTHENTICATION1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_KERBEROS_AUTHENTICATION1").field("flags", &self.flags).field("proxyServer", &self.proxyServer).finish()
    }
}
impl ::core::default::Default for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_KERBEROS_AUTHENTICATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IKEEXT_KERBEROS_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IKEEXT_KEYMODULE_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_KEYMODULE_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.v4Statistics == other.v4Statistics && self.v6Statistics == other.v6Statistics && self.errorFrequencyTable == other.errorFrequencyTable && self.mainModeNegotiationTime == other.mainModeNegotiationTime && self.quickModeNegotiationTime == other.quickModeNegotiationTime && self.extendedModeNegotiationTime == other.extendedModeNegotiationTime
    }
}
impl ::core::cmp::Eq for IKEEXT_KEYMODULE_STATISTICS0 {}
impl ::core::fmt::Debug for IKEEXT_KEYMODULE_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_KEYMODULE_STATISTICS0").field("v4Statistics", &self.v4Statistics).field("v6Statistics", &self.v6Statistics).field("errorFrequencyTable", &self.errorFrequencyTable).field("mainModeNegotiationTime", &self.mainModeNegotiationTime).field("quickModeNegotiationTime", &self.quickModeNegotiationTime).field("extendedModeNegotiationTime", &self.extendedModeNegotiationTime).finish()
    }
}
impl ::core::default::Default for IKEEXT_KEYMODULE_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_KEYMODULE_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.v4Statistics == other.v4Statistics && self.v6Statistics == other.v6Statistics && self.errorFrequencyTable == other.errorFrequencyTable && self.mainModeNegotiationTime == other.mainModeNegotiationTime && self.quickModeNegotiationTime == other.quickModeNegotiationTime && self.extendedModeNegotiationTime == other.extendedModeNegotiationTime
    }
}
impl ::core::cmp::Eq for IKEEXT_KEYMODULE_STATISTICS1 {}
impl ::core::fmt::Debug for IKEEXT_KEYMODULE_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_KEYMODULE_STATISTICS1").field("v4Statistics", &self.v4Statistics).field("v6Statistics", &self.v6Statistics).field("errorFrequencyTable", &self.errorFrequencyTable).field("mainModeNegotiationTime", &self.mainModeNegotiationTime).field("quickModeNegotiationTime", &self.quickModeNegotiationTime).field("extendedModeNegotiationTime", &self.extendedModeNegotiationTime).finish()
    }
}
impl ::core::default::Default for IKEEXT_KEY_MODULE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_KEY_MODULE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_KEY_MODULE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKEEXT_MM_SA_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_MM_SA_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_MM_SA_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKEEXT_NAME_CREDENTIAL0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_NAME_CREDENTIAL0 {
    fn eq(&self, other: &Self) -> bool {
        self.principalName == other.principalName
    }
}
impl ::core::cmp::Eq for IKEEXT_NAME_CREDENTIAL0 {}
impl ::core::fmt::Debug for IKEEXT_NAME_CREDENTIAL0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_NAME_CREDENTIAL0").field("principalName", &self.principalName).finish()
    }
}
impl ::core::default::Default for IKEEXT_NTLM_V2_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_NTLM_V2_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
    }
}
impl ::core::cmp::Eq for IKEEXT_NTLM_V2_AUTHENTICATION0 {}
impl ::core::fmt::Debug for IKEEXT_NTLM_V2_AUTHENTICATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_NTLM_V2_AUTHENTICATION0").field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for IKEEXT_POLICY0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        self.softExpirationTime == other.softExpirationTime && self.numAuthenticationMethods == other.numAuthenticationMethods && self.authenticationMethods == other.authenticationMethods && self.initiatorImpersonationType == other.initiatorImpersonationType && self.numIkeProposals == other.numIkeProposals && self.ikeProposals == other.ikeProposals && self.flags == other.flags && self.maxDynamicFilters == other.maxDynamicFilters
    }
}
impl ::core::cmp::Eq for IKEEXT_POLICY0 {}
impl ::core::fmt::Debug for IKEEXT_POLICY0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_POLICY0").field("softExpirationTime", &self.softExpirationTime).field("numAuthenticationMethods", &self.numAuthenticationMethods).field("authenticationMethods", &self.authenticationMethods).field("initiatorImpersonationType", &self.initiatorImpersonationType).field("numIkeProposals", &self.numIkeProposals).field("ikeProposals", &self.ikeProposals).field("flags", &self.flags).field("maxDynamicFilters", &self.maxDynamicFilters).finish()
    }
}
impl ::core::default::Default for IKEEXT_POLICY1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_POLICY1 {
    fn eq(&self, other: &Self) -> bool {
        self.softExpirationTime == other.softExpirationTime && self.numAuthenticationMethods == other.numAuthenticationMethods && self.authenticationMethods == other.authenticationMethods && self.initiatorImpersonationType == other.initiatorImpersonationType && self.numIkeProposals == other.numIkeProposals && self.ikeProposals == other.ikeProposals && self.flags == other.flags && self.maxDynamicFilters == other.maxDynamicFilters && self.retransmitDurationSecs == other.retransmitDurationSecs
    }
}
impl ::core::cmp::Eq for IKEEXT_POLICY1 {}
impl ::core::fmt::Debug for IKEEXT_POLICY1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_POLICY1")
            .field("softExpirationTime", &self.softExpirationTime)
            .field("numAuthenticationMethods", &self.numAuthenticationMethods)
            .field("authenticationMethods", &self.authenticationMethods)
            .field("initiatorImpersonationType", &self.initiatorImpersonationType)
            .field("numIkeProposals", &self.numIkeProposals)
            .field("ikeProposals", &self.ikeProposals)
            .field("flags", &self.flags)
            .field("maxDynamicFilters", &self.maxDynamicFilters)
            .field("retransmitDurationSecs", &self.retransmitDurationSecs)
            .finish()
    }
}
impl ::core::default::Default for IKEEXT_POLICY2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_POLICY2 {
    fn eq(&self, other: &Self) -> bool {
        self.softExpirationTime == other.softExpirationTime && self.numAuthenticationMethods == other.numAuthenticationMethods && self.authenticationMethods == other.authenticationMethods && self.initiatorImpersonationType == other.initiatorImpersonationType && self.numIkeProposals == other.numIkeProposals && self.ikeProposals == other.ikeProposals && self.flags == other.flags && self.maxDynamicFilters == other.maxDynamicFilters && self.retransmitDurationSecs == other.retransmitDurationSecs
    }
}
impl ::core::cmp::Eq for IKEEXT_POLICY2 {}
impl ::core::fmt::Debug for IKEEXT_POLICY2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_POLICY2")
            .field("softExpirationTime", &self.softExpirationTime)
            .field("numAuthenticationMethods", &self.numAuthenticationMethods)
            .field("authenticationMethods", &self.authenticationMethods)
            .field("initiatorImpersonationType", &self.initiatorImpersonationType)
            .field("numIkeProposals", &self.numIkeProposals)
            .field("ikeProposals", &self.ikeProposals)
            .field("flags", &self.flags)
            .field("maxDynamicFilters", &self.maxDynamicFilters)
            .field("retransmitDurationSecs", &self.retransmitDurationSecs)
            .finish()
    }
}
impl ::core::default::Default for IKEEXT_POLICY_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_POLICY_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_POLICY_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IKEEXT_POLICY_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IKEEXT_POLICY_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IKEEXT_POLICY_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IKEEXT_POLICY_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IKEEXT_POLICY_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.presharedKey == other.presharedKey
    }
}
impl ::core::cmp::Eq for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {}
impl ::core::fmt::Debug for IKEEXT_PRESHARED_KEY_AUTHENTICATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_PRESHARED_KEY_AUTHENTICATION0").field("presharedKey", &self.presharedKey).finish()
    }
}
impl ::core::default::Default for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    fn eq(&self, other: &Self) -> bool {
        self.presharedKey == other.presharedKey && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {}
impl ::core::fmt::Debug for IKEEXT_PRESHARED_KEY_AUTHENTICATION1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_PRESHARED_KEY_AUTHENTICATION1").field("presharedKey", &self.presharedKey).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IKEEXT_PRESHARED_KEY_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IKEEXT_PROPOSAL0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_PROPOSAL0 {
    fn eq(&self, other: &Self) -> bool {
        self.cipherAlgorithm == other.cipherAlgorithm && self.integrityAlgorithm == other.integrityAlgorithm && self.maxLifetimeSeconds == other.maxLifetimeSeconds && self.dhGroup == other.dhGroup && self.quickModeLimit == other.quickModeLimit
    }
}
impl ::core::cmp::Eq for IKEEXT_PROPOSAL0 {}
impl ::core::fmt::Debug for IKEEXT_PROPOSAL0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_PROPOSAL0").field("cipherAlgorithm", &self.cipherAlgorithm).field("integrityAlgorithm", &self.integrityAlgorithm).field("maxLifetimeSeconds", &self.maxLifetimeSeconds).field("dhGroup", &self.dhGroup).field("quickModeLimit", &self.quickModeLimit).finish()
    }
}
impl ::core::default::Default for IKEEXT_QM_SA_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_QM_SA_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_QM_SA_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKEEXT_RESERVED_AUTHENTICATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_RESERVED_AUTHENTICATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags
    }
}
impl ::core::cmp::Eq for IKEEXT_RESERVED_AUTHENTICATION0 {}
impl ::core::fmt::Debug for IKEEXT_RESERVED_AUTHENTICATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_RESERVED_AUTHENTICATION0").field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_RESERVED_AUTHENTICATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IKEEXT_RESERVED_AUTHENTICATION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IKEEXT_SA_DETAILS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_SA_DETAILS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_SA_DETAILS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IKEEXT_SA_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IKEEXT_SA_ROLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IKEEXT_SA_ROLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IKEEXT_SA_ROLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IKEEXT_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.ikeStatistics == other.ikeStatistics && self.authipStatistics == other.authipStatistics && self.commonStatistics == other.commonStatistics
    }
}
impl ::core::cmp::Eq for IKEEXT_STATISTICS0 {}
impl ::core::fmt::Debug for IKEEXT_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_STATISTICS0").field("ikeStatistics", &self.ikeStatistics).field("authipStatistics", &self.authipStatistics).field("commonStatistics", &self.commonStatistics).finish()
    }
}
impl ::core::default::Default for IKEEXT_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IKEEXT_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.ikeStatistics == other.ikeStatistics && self.authipStatistics == other.authipStatistics && self.ikeV2Statistics == other.ikeV2Statistics && self.commonStatistics == other.commonStatistics
    }
}
impl ::core::cmp::Eq for IKEEXT_STATISTICS1 {}
impl ::core::fmt::Debug for IKEEXT_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IKEEXT_STATISTICS1").field("ikeStatistics", &self.ikeStatistics).field("authipStatistics", &self.authipStatistics).field("ikeV2Statistics", &self.ikeV2Statistics).field("commonStatistics", &self.commonStatistics).finish()
    }
}
impl ::core::default::Default for IKEEXT_TRAFFIC0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_ADDRESS_INFO0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_ADDRESS_INFO0 {
    fn eq(&self, other: &Self) -> bool {
        self.numV4Addresses == other.numV4Addresses && self.v4Addresses == other.v4Addresses && self.numV6Addresses == other.numV6Addresses && self.v6Addresses == other.v6Addresses
    }
}
impl ::core::cmp::Eq for IPSEC_ADDRESS_INFO0 {}
impl ::core::fmt::Debug for IPSEC_ADDRESS_INFO0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_ADDRESS_INFO0").field("numV4Addresses", &self.numV4Addresses).field("v4Addresses", &self.v4Addresses).field("numV6Addresses", &self.numV6Addresses).field("v6Addresses", &self.v6Addresses).finish()
    }
}
impl ::core::default::Default for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.invalidSpisOnInbound == other.invalidSpisOnInbound && self.decryptionFailuresOnInbound == other.decryptionFailuresOnInbound && self.authenticationFailuresOnInbound == other.authenticationFailuresOnInbound && self.udpEspValidationFailuresOnInbound == other.udpEspValidationFailuresOnInbound && self.replayCheckFailuresOnInbound == other.replayCheckFailuresOnInbound && self.invalidClearTextInbound == other.invalidClearTextInbound && self.saNotInitializedOnInbound == other.saNotInitializedOnInbound && self.receiveOverIncorrectSaInbound == other.receiveOverIncorrectSaInbound && self.secureReceivesNotMatchingFilters == other.secureReceivesNotMatchingFilters
    }
}
impl ::core::cmp::Eq for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {}
impl ::core::fmt::Debug for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_AGGREGATE_DROP_PACKET_STATISTICS0")
            .field("invalidSpisOnInbound", &self.invalidSpisOnInbound)
            .field("decryptionFailuresOnInbound", &self.decryptionFailuresOnInbound)
            .field("authenticationFailuresOnInbound", &self.authenticationFailuresOnInbound)
            .field("udpEspValidationFailuresOnInbound", &self.udpEspValidationFailuresOnInbound)
            .field("replayCheckFailuresOnInbound", &self.replayCheckFailuresOnInbound)
            .field("invalidClearTextInbound", &self.invalidClearTextInbound)
            .field("saNotInitializedOnInbound", &self.saNotInitializedOnInbound)
            .field("receiveOverIncorrectSaInbound", &self.receiveOverIncorrectSaInbound)
            .field("secureReceivesNotMatchingFilters", &self.secureReceivesNotMatchingFilters)
            .finish()
    }
}
impl ::core::default::Default for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.invalidSpisOnInbound == other.invalidSpisOnInbound && self.decryptionFailuresOnInbound == other.decryptionFailuresOnInbound && self.authenticationFailuresOnInbound == other.authenticationFailuresOnInbound && self.udpEspValidationFailuresOnInbound == other.udpEspValidationFailuresOnInbound && self.replayCheckFailuresOnInbound == other.replayCheckFailuresOnInbound && self.invalidClearTextInbound == other.invalidClearTextInbound && self.saNotInitializedOnInbound == other.saNotInitializedOnInbound && self.receiveOverIncorrectSaInbound == other.receiveOverIncorrectSaInbound && self.secureReceivesNotMatchingFilters == other.secureReceivesNotMatchingFilters && self.totalDropPacketsInbound == other.totalDropPacketsInbound
    }
}
impl ::core::cmp::Eq for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {}
impl ::core::fmt::Debug for IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_AGGREGATE_DROP_PACKET_STATISTICS1")
            .field("invalidSpisOnInbound", &self.invalidSpisOnInbound)
            .field("decryptionFailuresOnInbound", &self.decryptionFailuresOnInbound)
            .field("authenticationFailuresOnInbound", &self.authenticationFailuresOnInbound)
            .field("udpEspValidationFailuresOnInbound", &self.udpEspValidationFailuresOnInbound)
            .field("replayCheckFailuresOnInbound", &self.replayCheckFailuresOnInbound)
            .field("invalidClearTextInbound", &self.invalidClearTextInbound)
            .field("saNotInitializedOnInbound", &self.saNotInitializedOnInbound)
            .field("receiveOverIncorrectSaInbound", &self.receiveOverIncorrectSaInbound)
            .field("secureReceivesNotMatchingFilters", &self.secureReceivesNotMatchingFilters)
            .field("totalDropPacketsInbound", &self.totalDropPacketsInbound)
            .finish()
    }
}
impl ::core::default::Default for IPSEC_AGGREGATE_SA_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_AGGREGATE_SA_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.activeSas == other.activeSas && self.pendingSaNegotiations == other.pendingSaNegotiations && self.totalSasAdded == other.totalSasAdded && self.totalSasDeleted == other.totalSasDeleted && self.successfulRekeys == other.successfulRekeys && self.activeTunnels == other.activeTunnels && self.offloadedSas == other.offloadedSas
    }
}
impl ::core::cmp::Eq for IPSEC_AGGREGATE_SA_STATISTICS0 {}
impl ::core::fmt::Debug for IPSEC_AGGREGATE_SA_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_AGGREGATE_SA_STATISTICS0").field("activeSas", &self.activeSas).field("pendingSaNegotiations", &self.pendingSaNegotiations).field("totalSasAdded", &self.totalSasAdded).field("totalSasDeleted", &self.totalSasDeleted).field("successfulRekeys", &self.successfulRekeys).field("activeTunnels", &self.activeTunnels).field("offloadedSas", &self.offloadedSas).finish()
    }
}
impl ::core::default::Default for IPSEC_AH_DROP_PACKET_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_AH_DROP_PACKET_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.invalidSpisOnInbound == other.invalidSpisOnInbound && self.authenticationFailuresOnInbound == other.authenticationFailuresOnInbound && self.replayCheckFailuresOnInbound == other.replayCheckFailuresOnInbound && self.saNotInitializedOnInbound == other.saNotInitializedOnInbound
    }
}
impl ::core::cmp::Eq for IPSEC_AH_DROP_PACKET_STATISTICS0 {}
impl ::core::fmt::Debug for IPSEC_AH_DROP_PACKET_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_AH_DROP_PACKET_STATISTICS0").field("invalidSpisOnInbound", &self.invalidSpisOnInbound).field("authenticationFailuresOnInbound", &self.authenticationFailuresOnInbound).field("replayCheckFailuresOnInbound", &self.replayCheckFailuresOnInbound).field("saNotInitializedOnInbound", &self.saNotInitializedOnInbound).finish()
    }
}
impl ::core::default::Default for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    fn eq(&self, other: &Self) -> bool {
        self.authTransform == other.authTransform && self.cipherTransform == other.cipherTransform
    }
}
impl ::core::cmp::Eq for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {}
impl ::core::fmt::Debug for IPSEC_AUTH_AND_CIPHER_TRANSFORM0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_AUTH_AND_CIPHER_TRANSFORM0").field("authTransform", &self.authTransform).field("cipherTransform", &self.cipherTransform).finish()
    }
}
impl ::core::default::Default for IPSEC_AUTH_TRANSFORM0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_AUTH_TRANSFORM0 {
    fn eq(&self, other: &Self) -> bool {
        self.authTransformId == other.authTransformId && self.cryptoModuleId == other.cryptoModuleId
    }
}
impl ::core::cmp::Eq for IPSEC_AUTH_TRANSFORM0 {}
impl ::core::fmt::Debug for IPSEC_AUTH_TRANSFORM0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_AUTH_TRANSFORM0").field("authTransformId", &self.authTransformId).field("cryptoModuleId", &self.cryptoModuleId).finish()
    }
}
impl ::core::default::Default for IPSEC_AUTH_TRANSFORM_ID0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_AUTH_TRANSFORM_ID0 {
    fn eq(&self, other: &Self) -> bool {
        self.authType == other.authType && self.authConfig == other.authConfig
    }
}
impl ::core::cmp::Eq for IPSEC_AUTH_TRANSFORM_ID0 {}
impl ::core::fmt::Debug for IPSEC_AUTH_TRANSFORM_ID0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_AUTH_TRANSFORM_ID0").field("authType", &self.authType).field("authConfig", &self.authConfig).finish()
    }
}
impl ::core::default::Default for IPSEC_AUTH_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPSEC_AUTH_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_AUTH_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IPSEC_CIPHER_TRANSFORM0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_CIPHER_TRANSFORM0 {
    fn eq(&self, other: &Self) -> bool {
        self.cipherTransformId == other.cipherTransformId && self.cryptoModuleId == other.cryptoModuleId
    }
}
impl ::core::cmp::Eq for IPSEC_CIPHER_TRANSFORM0 {}
impl ::core::fmt::Debug for IPSEC_CIPHER_TRANSFORM0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_CIPHER_TRANSFORM0").field("cipherTransformId", &self.cipherTransformId).field("cryptoModuleId", &self.cryptoModuleId).finish()
    }
}
impl ::core::default::Default for IPSEC_CIPHER_TRANSFORM_ID0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_CIPHER_TRANSFORM_ID0 {
    fn eq(&self, other: &Self) -> bool {
        self.cipherType == other.cipherType && self.cipherConfig == other.cipherConfig
    }
}
impl ::core::cmp::Eq for IPSEC_CIPHER_TRANSFORM_ID0 {}
impl ::core::fmt::Debug for IPSEC_CIPHER_TRANSFORM_ID0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_CIPHER_TRANSFORM_ID0").field("cipherType", &self.cipherType).field("cipherConfig", &self.cipherConfig).finish()
    }
}
impl ::core::default::Default for IPSEC_CIPHER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPSEC_CIPHER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_CIPHER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IPSEC_DOSP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPSEC_DOSP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_DOSP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IPSEC_DOSP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IPSEC_DOSP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IPSEC_DOSP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IPSEC_DOSP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IPSEC_DOSP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IPSEC_DOSP_OPTIONS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_DOSP_OPTIONS0 {
    fn eq(&self, other: &Self) -> bool {
        self.stateIdleTimeoutSeconds == other.stateIdleTimeoutSeconds
            && self.perIPRateLimitQueueIdleTimeoutSeconds == other.perIPRateLimitQueueIdleTimeoutSeconds
            && self.ipV6IPsecUnauthDscp == other.ipV6IPsecUnauthDscp
            && self.ipV6IPsecUnauthRateLimitBytesPerSec == other.ipV6IPsecUnauthRateLimitBytesPerSec
            && self.ipV6IPsecUnauthPerIPRateLimitBytesPerSec == other.ipV6IPsecUnauthPerIPRateLimitBytesPerSec
            && self.ipV6IPsecAuthDscp == other.ipV6IPsecAuthDscp
            && self.ipV6IPsecAuthRateLimitBytesPerSec == other.ipV6IPsecAuthRateLimitBytesPerSec
            && self.icmpV6Dscp == other.icmpV6Dscp
            && self.icmpV6RateLimitBytesPerSec == other.icmpV6RateLimitBytesPerSec
            && self.ipV6FilterExemptDscp == other.ipV6FilterExemptDscp
            && self.ipV6FilterExemptRateLimitBytesPerSec == other.ipV6FilterExemptRateLimitBytesPerSec
            && self.defBlockExemptDscp == other.defBlockExemptDscp
            && self.defBlockExemptRateLimitBytesPerSec == other.defBlockExemptRateLimitBytesPerSec
            && self.maxStateEntries == other.maxStateEntries
            && self.maxPerIPRateLimitQueues == other.maxPerIPRateLimitQueues
            && self.flags == other.flags
            && self.numPublicIFLuids == other.numPublicIFLuids
            && self.publicIFLuids == other.publicIFLuids
            && self.numInternalIFLuids == other.numInternalIFLuids
            && self.internalIFLuids == other.internalIFLuids
            && self.publicV6AddrMask == other.publicV6AddrMask
            && self.internalV6AddrMask == other.internalV6AddrMask
    }
}
impl ::core::cmp::Eq for IPSEC_DOSP_OPTIONS0 {}
impl ::core::fmt::Debug for IPSEC_DOSP_OPTIONS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_DOSP_OPTIONS0")
            .field("stateIdleTimeoutSeconds", &self.stateIdleTimeoutSeconds)
            .field("perIPRateLimitQueueIdleTimeoutSeconds", &self.perIPRateLimitQueueIdleTimeoutSeconds)
            .field("ipV6IPsecUnauthDscp", &self.ipV6IPsecUnauthDscp)
            .field("ipV6IPsecUnauthRateLimitBytesPerSec", &self.ipV6IPsecUnauthRateLimitBytesPerSec)
            .field("ipV6IPsecUnauthPerIPRateLimitBytesPerSec", &self.ipV6IPsecUnauthPerIPRateLimitBytesPerSec)
            .field("ipV6IPsecAuthDscp", &self.ipV6IPsecAuthDscp)
            .field("ipV6IPsecAuthRateLimitBytesPerSec", &self.ipV6IPsecAuthRateLimitBytesPerSec)
            .field("icmpV6Dscp", &self.icmpV6Dscp)
            .field("icmpV6RateLimitBytesPerSec", &self.icmpV6RateLimitBytesPerSec)
            .field("ipV6FilterExemptDscp", &self.ipV6FilterExemptDscp)
            .field("ipV6FilterExemptRateLimitBytesPerSec", &self.ipV6FilterExemptRateLimitBytesPerSec)
            .field("defBlockExemptDscp", &self.defBlockExemptDscp)
            .field("defBlockExemptRateLimitBytesPerSec", &self.defBlockExemptRateLimitBytesPerSec)
            .field("maxStateEntries", &self.maxStateEntries)
            .field("maxPerIPRateLimitQueues", &self.maxPerIPRateLimitQueues)
            .field("flags", &self.flags)
            .field("numPublicIFLuids", &self.numPublicIFLuids)
            .field("publicIFLuids", &self.publicIFLuids)
            .field("numInternalIFLuids", &self.numInternalIFLuids)
            .field("internalIFLuids", &self.internalIFLuids)
            .field("publicV6AddrMask", &self.publicV6AddrMask)
            .field("internalV6AddrMask", &self.internalV6AddrMask)
            .finish()
    }
}
impl ::core::default::Default for IPSEC_DOSP_STATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_DOSP_STATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.publicHostV6Addr == other.publicHostV6Addr && self.internalHostV6Addr == other.internalHostV6Addr && self.totalInboundIPv6IPsecAuthPackets == other.totalInboundIPv6IPsecAuthPackets && self.totalOutboundIPv6IPsecAuthPackets == other.totalOutboundIPv6IPsecAuthPackets && self.durationSecs == other.durationSecs
    }
}
impl ::core::cmp::Eq for IPSEC_DOSP_STATE0 {}
impl ::core::fmt::Debug for IPSEC_DOSP_STATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_DOSP_STATE0").field("publicHostV6Addr", &self.publicHostV6Addr).field("internalHostV6Addr", &self.internalHostV6Addr).field("totalInboundIPv6IPsecAuthPackets", &self.totalInboundIPv6IPsecAuthPackets).field("totalOutboundIPv6IPsecAuthPackets", &self.totalOutboundIPv6IPsecAuthPackets).field("durationSecs", &self.durationSecs).finish()
    }
}
impl ::core::default::Default for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.publicV6AddrMask == other.publicV6AddrMask && self.internalV6AddrMask == other.internalV6AddrMask
    }
}
impl ::core::cmp::Eq for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {}
impl ::core::fmt::Debug for IPSEC_DOSP_STATE_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_DOSP_STATE_ENUM_TEMPLATE0").field("publicV6AddrMask", &self.publicV6AddrMask).field("internalV6AddrMask", &self.internalV6AddrMask).finish()
    }
}
impl ::core::default::Default for IPSEC_DOSP_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_DOSP_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.totalStateEntriesCreated == other.totalStateEntriesCreated
            && self.currentStateEntries == other.currentStateEntries
            && self.totalInboundAllowedIPv6IPsecUnauthPkts == other.totalInboundAllowedIPv6IPsecUnauthPkts
            && self.totalInboundRatelimitDiscardedIPv6IPsecUnauthPkts == other.totalInboundRatelimitDiscardedIPv6IPsecUnauthPkts
            && self.totalInboundPerIPRatelimitDiscardedIPv6IPsecUnauthPkts == other.totalInboundPerIPRatelimitDiscardedIPv6IPsecUnauthPkts
            && self.totalInboundOtherDiscardedIPv6IPsecUnauthPkts == other.totalInboundOtherDiscardedIPv6IPsecUnauthPkts
            && self.totalInboundAllowedIPv6IPsecAuthPkts == other.totalInboundAllowedIPv6IPsecAuthPkts
            && self.totalInboundRatelimitDiscardedIPv6IPsecAuthPkts == other.totalInboundRatelimitDiscardedIPv6IPsecAuthPkts
            && self.totalInboundOtherDiscardedIPv6IPsecAuthPkts == other.totalInboundOtherDiscardedIPv6IPsecAuthPkts
            && self.totalInboundAllowedICMPv6Pkts == other.totalInboundAllowedICMPv6Pkts
            && self.totalInboundRatelimitDiscardedICMPv6Pkts == other.totalInboundRatelimitDiscardedICMPv6Pkts
            && self.totalInboundAllowedIPv6FilterExemptPkts == other.totalInboundAllowedIPv6FilterExemptPkts
            && self.totalInboundRatelimitDiscardedIPv6FilterExemptPkts == other.totalInboundRatelimitDiscardedIPv6FilterExemptPkts
            && self.totalInboundDiscardedIPv6FilterBlockPkts == other.totalInboundDiscardedIPv6FilterBlockPkts
            && self.totalInboundAllowedDefBlockExemptPkts == other.totalInboundAllowedDefBlockExemptPkts
            && self.totalInboundRatelimitDiscardedDefBlockExemptPkts == other.totalInboundRatelimitDiscardedDefBlockExemptPkts
            && self.totalInboundDiscardedDefBlockPkts == other.totalInboundDiscardedDefBlockPkts
            && self.currentInboundIPv6IPsecUnauthPerIPRateLimitQueues == other.currentInboundIPv6IPsecUnauthPerIPRateLimitQueues
    }
}
impl ::core::cmp::Eq for IPSEC_DOSP_STATISTICS0 {}
impl ::core::fmt::Debug for IPSEC_DOSP_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_DOSP_STATISTICS0")
            .field("totalStateEntriesCreated", &self.totalStateEntriesCreated)
            .field("currentStateEntries", &self.currentStateEntries)
            .field("totalInboundAllowedIPv6IPsecUnauthPkts", &self.totalInboundAllowedIPv6IPsecUnauthPkts)
            .field("totalInboundRatelimitDiscardedIPv6IPsecUnauthPkts", &self.totalInboundRatelimitDiscardedIPv6IPsecUnauthPkts)
            .field("totalInboundPerIPRatelimitDiscardedIPv6IPsecUnauthPkts", &self.totalInboundPerIPRatelimitDiscardedIPv6IPsecUnauthPkts)
            .field("totalInboundOtherDiscardedIPv6IPsecUnauthPkts", &self.totalInboundOtherDiscardedIPv6IPsecUnauthPkts)
            .field("totalInboundAllowedIPv6IPsecAuthPkts", &self.totalInboundAllowedIPv6IPsecAuthPkts)
            .field("totalInboundRatelimitDiscardedIPv6IPsecAuthPkts", &self.totalInboundRatelimitDiscardedIPv6IPsecAuthPkts)
            .field("totalInboundOtherDiscardedIPv6IPsecAuthPkts", &self.totalInboundOtherDiscardedIPv6IPsecAuthPkts)
            .field("totalInboundAllowedICMPv6Pkts", &self.totalInboundAllowedICMPv6Pkts)
            .field("totalInboundRatelimitDiscardedICMPv6Pkts", &self.totalInboundRatelimitDiscardedICMPv6Pkts)
            .field("totalInboundAllowedIPv6FilterExemptPkts", &self.totalInboundAllowedIPv6FilterExemptPkts)
            .field("totalInboundRatelimitDiscardedIPv6FilterExemptPkts", &self.totalInboundRatelimitDiscardedIPv6FilterExemptPkts)
            .field("totalInboundDiscardedIPv6FilterBlockPkts", &self.totalInboundDiscardedIPv6FilterBlockPkts)
            .field("totalInboundAllowedDefBlockExemptPkts", &self.totalInboundAllowedDefBlockExemptPkts)
            .field("totalInboundRatelimitDiscardedDefBlockExemptPkts", &self.totalInboundRatelimitDiscardedDefBlockExemptPkts)
            .field("totalInboundDiscardedDefBlockPkts", &self.totalInboundDiscardedDefBlockPkts)
            .field("currentInboundIPv6IPsecUnauthPerIPRateLimitQueues", &self.currentInboundIPv6IPsecUnauthPerIPRateLimitQueues)
            .finish()
    }
}
impl ::core::default::Default for IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.invalidSpisOnInbound == other.invalidSpisOnInbound && self.decryptionFailuresOnInbound == other.decryptionFailuresOnInbound && self.authenticationFailuresOnInbound == other.authenticationFailuresOnInbound && self.replayCheckFailuresOnInbound == other.replayCheckFailuresOnInbound && self.saNotInitializedOnInbound == other.saNotInitializedOnInbound
    }
}
impl ::core::cmp::Eq for IPSEC_ESP_DROP_PACKET_STATISTICS0 {}
impl ::core::fmt::Debug for IPSEC_ESP_DROP_PACKET_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_ESP_DROP_PACKET_STATISTICS0").field("invalidSpisOnInbound", &self.invalidSpisOnInbound).field("decryptionFailuresOnInbound", &self.decryptionFailuresOnInbound).field("authenticationFailuresOnInbound", &self.authenticationFailuresOnInbound).field("replayCheckFailuresOnInbound", &self.replayCheckFailuresOnInbound).field("saNotInitializedOnInbound", &self.saNotInitializedOnInbound).finish()
    }
}
impl ::core::default::Default for IPSEC_FAILURE_POINT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPSEC_FAILURE_POINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_FAILURE_POINT").field(&self.0).finish()
    }
}
impl ::core::default::Default for IPSEC_GETSPI0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_GETSPI1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_ID0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_ID0 {
    fn eq(&self, other: &Self) -> bool {
        self.mmTargetName == other.mmTargetName && self.emTargetName == other.emTargetName && self.numTokens == other.numTokens && self.tokens == other.tokens && self.explicitCredentials == other.explicitCredentials && self.logonId == other.logonId
    }
}
impl ::core::cmp::Eq for IPSEC_ID0 {}
impl ::core::fmt::Debug for IPSEC_ID0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_ID0").field("mmTargetName", &self.mmTargetName).field("emTargetName", &self.emTargetName).field("numTokens", &self.numTokens).field("tokens", &self.tokens).field("explicitCredentials", &self.explicitCredentials).field("logonId", &self.logonId).finish()
    }
}
impl ::core::default::Default for IPSEC_KEYING_POLICY0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_KEYING_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        self.numKeyMods == other.numKeyMods && self.keyModKeys == other.keyModKeys
    }
}
impl ::core::cmp::Eq for IPSEC_KEYING_POLICY0 {}
impl ::core::fmt::Debug for IPSEC_KEYING_POLICY0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_KEYING_POLICY0").field("numKeyMods", &self.numKeyMods).field("keyModKeys", &self.keyModKeys).finish()
    }
}
impl ::core::default::Default for IPSEC_KEYING_POLICY1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_KEYING_POLICY1 {
    fn eq(&self, other: &Self) -> bool {
        self.numKeyMods == other.numKeyMods && self.keyModKeys == other.keyModKeys && self.flags == other.flags
    }
}
impl ::core::cmp::Eq for IPSEC_KEYING_POLICY1 {}
impl ::core::fmt::Debug for IPSEC_KEYING_POLICY1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_KEYING_POLICY1").field("numKeyMods", &self.numKeyMods).field("keyModKeys", &self.keyModKeys).field("flags", &self.flags).finish()
    }
}
impl ::core::default::Default for IPSEC_KEYMODULE_STATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_KEYMODULE_STATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.keyModuleKey == other.keyModuleKey && self.stateBlob == other.stateBlob
    }
}
impl ::core::cmp::Eq for IPSEC_KEYMODULE_STATE0 {}
impl ::core::fmt::Debug for IPSEC_KEYMODULE_STATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_KEYMODULE_STATE0").field("keyModuleKey", &self.keyModuleKey).field("stateBlob", &self.stateBlob).finish()
    }
}
impl ::core::default::Default for IPSEC_KEY_MANAGER0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_KEY_MANAGER0 {
    fn eq(&self, other: &Self) -> bool {
        self.keyManagerKey == other.keyManagerKey && self.displayData == other.displayData && self.flags == other.flags && self.keyDictationTimeoutHint == other.keyDictationTimeoutHint
    }
}
impl ::core::cmp::Eq for IPSEC_KEY_MANAGER0 {}
impl ::core::fmt::Debug for IPSEC_KEY_MANAGER0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_KEY_MANAGER0").field("keyManagerKey", &self.keyManagerKey).field("displayData", &self.displayData).field("flags", &self.flags).field("keyDictationTimeoutHint", &self.keyDictationTimeoutHint).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_KEY_MANAGER_CALLBACKS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_PFS_GROUP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPSEC_PFS_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_PFS_GROUP").field(&self.0).finish()
    }
}
impl ::core::default::Default for IPSEC_POLICY_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPSEC_POLICY_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_POLICY_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IPSEC_POLICY_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IPSEC_POLICY_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IPSEC_POLICY_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IPSEC_POLICY_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IPSEC_POLICY_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IPSEC_PROPOSAL0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_PROPOSAL0 {
    fn eq(&self, other: &Self) -> bool {
        self.lifetime == other.lifetime && self.numSaTransforms == other.numSaTransforms && self.saTransforms == other.saTransforms && self.pfsGroup == other.pfsGroup
    }
}
impl ::core::cmp::Eq for IPSEC_PROPOSAL0 {}
impl ::core::fmt::Debug for IPSEC_PROPOSAL0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_PROPOSAL0").field("lifetime", &self.lifetime).field("numSaTransforms", &self.numSaTransforms).field("saTransforms", &self.saTransforms).field("pfsGroup", &self.pfsGroup).finish()
    }
}
impl ::core::default::Default for IPSEC_SA0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.saCipherInformation == other.saCipherInformation && self.saAuthInformation == other.saAuthInformation
    }
}
impl ::core::cmp::Eq for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {}
impl ::core::fmt::Debug for IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_AUTH_AND_CIPHER_INFORMATION0").field("saCipherInformation", &self.saCipherInformation).field("saAuthInformation", &self.saAuthInformation).finish()
    }
}
impl ::core::default::Default for IPSEC_SA_AUTH_INFORMATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_SA_AUTH_INFORMATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.authTransform == other.authTransform && self.authKey == other.authKey
    }
}
impl ::core::cmp::Eq for IPSEC_SA_AUTH_INFORMATION0 {}
impl ::core::fmt::Debug for IPSEC_SA_AUTH_INFORMATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_AUTH_INFORMATION0").field("authTransform", &self.authTransform).field("authKey", &self.authKey).finish()
    }
}
impl ::core::default::Default for IPSEC_SA_BUNDLE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_SA_BUNDLE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_SA_BUNDLE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPSEC_SA_BUNDLE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_SA_BUNDLE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for IPSEC_SA_BUNDLE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for IPSEC_SA_BUNDLE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for IPSEC_SA_BUNDLE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for IPSEC_SA_BUNDLE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for IPSEC_SA_BUNDLE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for IPSEC_SA_CIPHER_INFORMATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_SA_CIPHER_INFORMATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.cipherTransform == other.cipherTransform && self.cipherKey == other.cipherKey
    }
}
impl ::core::cmp::Eq for IPSEC_SA_CIPHER_INFORMATION0 {}
impl ::core::fmt::Debug for IPSEC_SA_CIPHER_INFORMATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_CIPHER_INFORMATION0").field("cipherTransform", &self.cipherTransform).field("cipherKey", &self.cipherKey).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_SA_CONTEXT0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for IPSEC_SA_CONTEXT0 {
    fn eq(&self, other: &Self) -> bool {
        self.saContextId == other.saContextId && self.inboundSa == other.inboundSa && self.outboundSa == other.outboundSa
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for IPSEC_SA_CONTEXT0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for IPSEC_SA_CONTEXT0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_CONTEXT0").field("saContextId", &self.saContextId).field("inboundSa", &self.inboundSa).field("outboundSa", &self.outboundSa).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_SA_CONTEXT1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for IPSEC_SA_CONTEXT1 {
    fn eq(&self, other: &Self) -> bool {
        self.saContextId == other.saContextId && self.inboundSa == other.inboundSa && self.outboundSa == other.outboundSa
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for IPSEC_SA_CONTEXT1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for IPSEC_SA_CONTEXT1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_CONTEXT1").field("saContextId", &self.saContextId).field("inboundSa", &self.inboundSa).field("outboundSa", &self.outboundSa).finish()
    }
}
impl ::core::default::Default for IPSEC_SA_CONTEXT_CHANGE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_SA_CONTEXT_CHANGE0 {
    fn eq(&self, other: &Self) -> bool {
        self.changeType == other.changeType && self.saContextId == other.saContextId
    }
}
impl ::core::cmp::Eq for IPSEC_SA_CONTEXT_CHANGE0 {}
impl ::core::fmt::Debug for IPSEC_SA_CONTEXT_CHANGE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_CONTEXT_CHANGE0").field("changeType", &self.changeType).field("saContextId", &self.saContextId).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_SA_CONTEXT_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_SA_CONTEXT_EVENT_TYPE0 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPSEC_SA_CONTEXT_EVENT_TYPE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_SA_CONTEXT_EVENT_TYPE0").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::PartialEq for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    fn eq(&self, other: &Self) -> bool {
        self.enumTemplate == other.enumTemplate && self.flags == other.flags && self.sessionKey == other.sessionKey
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::cmp::Eq for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for IPSEC_SA_CONTEXT_SUBSCRIPTION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_CONTEXT_SUBSCRIPTION0").field("enumTemplate", &self.enumTemplate).field("flags", &self.flags).field("sessionKey", &self.sessionKey).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_SA_DETAILS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for IPSEC_SA_DETAILS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_SA_ENUM_TEMPLATE0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_SA_ENUM_TEMPLATE0 {
    fn eq(&self, other: &Self) -> bool {
        self.saDirection == other.saDirection
    }
}
impl ::core::cmp::Eq for IPSEC_SA_ENUM_TEMPLATE0 {}
impl ::core::fmt::Debug for IPSEC_SA_ENUM_TEMPLATE0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_ENUM_TEMPLATE0").field("saDirection", &self.saDirection).finish()
    }
}
impl ::core::default::Default for IPSEC_SA_IDLE_TIMEOUT0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_SA_IDLE_TIMEOUT0 {
    fn eq(&self, other: &Self) -> bool {
        self.idleTimeoutSeconds == other.idleTimeoutSeconds && self.idleTimeoutSecondsFailOver == other.idleTimeoutSecondsFailOver
    }
}
impl ::core::cmp::Eq for IPSEC_SA_IDLE_TIMEOUT0 {}
impl ::core::fmt::Debug for IPSEC_SA_IDLE_TIMEOUT0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_IDLE_TIMEOUT0").field("idleTimeoutSeconds", &self.idleTimeoutSeconds).field("idleTimeoutSecondsFailOver", &self.idleTimeoutSecondsFailOver).finish()
    }
}
impl ::core::default::Default for IPSEC_SA_LIFETIME0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_SA_LIFETIME0 {
    fn eq(&self, other: &Self) -> bool {
        self.lifetimeSeconds == other.lifetimeSeconds && self.lifetimeKilobytes == other.lifetimeKilobytes && self.lifetimePackets == other.lifetimePackets
    }
}
impl ::core::cmp::Eq for IPSEC_SA_LIFETIME0 {}
impl ::core::fmt::Debug for IPSEC_SA_LIFETIME0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_SA_LIFETIME0").field("lifetimeSeconds", &self.lifetimeSeconds).field("lifetimeKilobytes", &self.lifetimeKilobytes).field("lifetimePackets", &self.lifetimePackets).finish()
    }
}
impl ::core::default::Default for IPSEC_SA_TRANSFORM0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.aggregateSaStatistics == other.aggregateSaStatistics && self.espDropPacketStatistics == other.espDropPacketStatistics && self.ahDropPacketStatistics == other.ahDropPacketStatistics && self.aggregateDropPacketStatistics == other.aggregateDropPacketStatistics && self.inboundTrafficStatistics == other.inboundTrafficStatistics && self.outboundTrafficStatistics == other.outboundTrafficStatistics
    }
}
impl ::core::cmp::Eq for IPSEC_STATISTICS0 {}
impl ::core::fmt::Debug for IPSEC_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_STATISTICS0").field("aggregateSaStatistics", &self.aggregateSaStatistics).field("espDropPacketStatistics", &self.espDropPacketStatistics).field("ahDropPacketStatistics", &self.ahDropPacketStatistics).field("aggregateDropPacketStatistics", &self.aggregateDropPacketStatistics).field("inboundTrafficStatistics", &self.inboundTrafficStatistics).field("outboundTrafficStatistics", &self.outboundTrafficStatistics).finish()
    }
}
impl ::core::default::Default for IPSEC_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.aggregateSaStatistics == other.aggregateSaStatistics && self.espDropPacketStatistics == other.espDropPacketStatistics && self.ahDropPacketStatistics == other.ahDropPacketStatistics && self.aggregateDropPacketStatistics == other.aggregateDropPacketStatistics && self.inboundTrafficStatistics == other.inboundTrafficStatistics && self.outboundTrafficStatistics == other.outboundTrafficStatistics
    }
}
impl ::core::cmp::Eq for IPSEC_STATISTICS1 {}
impl ::core::fmt::Debug for IPSEC_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_STATISTICS1").field("aggregateSaStatistics", &self.aggregateSaStatistics).field("espDropPacketStatistics", &self.espDropPacketStatistics).field("ahDropPacketStatistics", &self.ahDropPacketStatistics).field("aggregateDropPacketStatistics", &self.aggregateDropPacketStatistics).field("inboundTrafficStatistics", &self.inboundTrafficStatistics).field("outboundTrafficStatistics", &self.outboundTrafficStatistics).finish()
    }
}
impl ::core::default::Default for IPSEC_TOKEN0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_TOKEN0 {
    fn eq(&self, other: &Self) -> bool {
        self.r#type == other.r#type && self.principal == other.principal && self.mode == other.mode && self.token == other.token
    }
}
impl ::core::cmp::Eq for IPSEC_TOKEN0 {}
impl ::core::fmt::Debug for IPSEC_TOKEN0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_TOKEN0").field("type", &self.r#type).field("principal", &self.principal).field("mode", &self.mode).field("token", &self.token).finish()
    }
}
impl ::core::default::Default for IPSEC_TOKEN_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPSEC_TOKEN_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_TOKEN_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IPSEC_TOKEN_PRINCIPAL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPSEC_TOKEN_PRINCIPAL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_TOKEN_PRINCIPAL").field(&self.0).finish()
    }
}
impl ::core::default::Default for IPSEC_TOKEN_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPSEC_TOKEN_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_TOKEN_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IPSEC_TRAFFIC0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_TRAFFIC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_TRAFFIC_SELECTOR0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_TRAFFIC_SELECTOR_POLICY0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC_SELECTOR_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        self.flags == other.flags && self.numLocalTrafficSelectors == other.numLocalTrafficSelectors && self.localTrafficSelectors == other.localTrafficSelectors && self.numRemoteTrafficSelectors == other.numRemoteTrafficSelectors && self.remoteTrafficSelectors == other.remoteTrafficSelectors
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC_SELECTOR_POLICY0 {}
impl ::core::fmt::Debug for IPSEC_TRAFFIC_SELECTOR_POLICY0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_TRAFFIC_SELECTOR_POLICY0").field("flags", &self.flags).field("numLocalTrafficSelectors", &self.numLocalTrafficSelectors).field("localTrafficSelectors", &self.localTrafficSelectors).field("numRemoteTrafficSelectors", &self.numRemoteTrafficSelectors).field("remoteTrafficSelectors", &self.remoteTrafficSelectors).finish()
    }
}
impl ::core::default::Default for IPSEC_TRAFFIC_STATISTICS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC_STATISTICS0 {
    fn eq(&self, other: &Self) -> bool {
        self.encryptedByteCount == other.encryptedByteCount && self.authenticatedAHByteCount == other.authenticatedAHByteCount && self.authenticatedESPByteCount == other.authenticatedESPByteCount && self.transportByteCount == other.transportByteCount && self.tunnelByteCount == other.tunnelByteCount && self.offloadByteCount == other.offloadByteCount
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC_STATISTICS0 {}
impl ::core::fmt::Debug for IPSEC_TRAFFIC_STATISTICS0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_TRAFFIC_STATISTICS0").field("encryptedByteCount", &self.encryptedByteCount).field("authenticatedAHByteCount", &self.authenticatedAHByteCount).field("authenticatedESPByteCount", &self.authenticatedESPByteCount).field("transportByteCount", &self.transportByteCount).field("tunnelByteCount", &self.tunnelByteCount).field("offloadByteCount", &self.offloadByteCount).finish()
    }
}
impl ::core::default::Default for IPSEC_TRAFFIC_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_TRAFFIC_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.encryptedByteCount == other.encryptedByteCount && self.authenticatedAHByteCount == other.authenticatedAHByteCount && self.authenticatedESPByteCount == other.authenticatedESPByteCount && self.transportByteCount == other.transportByteCount && self.tunnelByteCount == other.tunnelByteCount && self.offloadByteCount == other.offloadByteCount && self.totalSuccessfulPackets == other.totalSuccessfulPackets
    }
}
impl ::core::cmp::Eq for IPSEC_TRAFFIC_STATISTICS1 {}
impl ::core::fmt::Debug for IPSEC_TRAFFIC_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_TRAFFIC_STATISTICS1").field("encryptedByteCount", &self.encryptedByteCount).field("authenticatedAHByteCount", &self.authenticatedAHByteCount).field("authenticatedESPByteCount", &self.authenticatedESPByteCount).field("transportByteCount", &self.transportByteCount).field("tunnelByteCount", &self.tunnelByteCount).field("offloadByteCount", &self.offloadByteCount).field("totalSuccessfulPackets", &self.totalSuccessfulPackets).finish()
    }
}
impl ::core::default::Default for IPSEC_TRAFFIC_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPSEC_TRAFFIC_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_TRAFFIC_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IPSEC_TRANSFORM_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for IPSEC_TRANSFORM_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IPSEC_TRANSFORM_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for IPSEC_TRANSPORT_POLICY0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_TRANSPORT_POLICY0 {
    fn eq(&self, other: &Self) -> bool {
        self.numIpsecProposals == other.numIpsecProposals && self.ipsecProposals == other.ipsecProposals && self.flags == other.flags && self.ndAllowClearTimeoutSeconds == other.ndAllowClearTimeoutSeconds && self.saIdleTimeout == other.saIdleTimeout && self.emPolicy == other.emPolicy
    }
}
impl ::core::cmp::Eq for IPSEC_TRANSPORT_POLICY0 {}
impl ::core::fmt::Debug for IPSEC_TRANSPORT_POLICY0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_TRANSPORT_POLICY0").field("numIpsecProposals", &self.numIpsecProposals).field("ipsecProposals", &self.ipsecProposals).field("flags", &self.flags).field("ndAllowClearTimeoutSeconds", &self.ndAllowClearTimeoutSeconds).field("saIdleTimeout", &self.saIdleTimeout).field("emPolicy", &self.emPolicy).finish()
    }
}
impl ::core::default::Default for IPSEC_TRANSPORT_POLICY1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_TRANSPORT_POLICY1 {
    fn eq(&self, other: &Self) -> bool {
        self.numIpsecProposals == other.numIpsecProposals && self.ipsecProposals == other.ipsecProposals && self.flags == other.flags && self.ndAllowClearTimeoutSeconds == other.ndAllowClearTimeoutSeconds && self.saIdleTimeout == other.saIdleTimeout && self.emPolicy == other.emPolicy
    }
}
impl ::core::cmp::Eq for IPSEC_TRANSPORT_POLICY1 {}
impl ::core::fmt::Debug for IPSEC_TRANSPORT_POLICY1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_TRANSPORT_POLICY1").field("numIpsecProposals", &self.numIpsecProposals).field("ipsecProposals", &self.ipsecProposals).field("flags", &self.flags).field("ndAllowClearTimeoutSeconds", &self.ndAllowClearTimeoutSeconds).field("saIdleTimeout", &self.saIdleTimeout).field("emPolicy", &self.emPolicy).finish()
    }
}
impl ::core::default::Default for IPSEC_TRANSPORT_POLICY2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_TRANSPORT_POLICY2 {
    fn eq(&self, other: &Self) -> bool {
        self.numIpsecProposals == other.numIpsecProposals && self.ipsecProposals == other.ipsecProposals && self.flags == other.flags && self.ndAllowClearTimeoutSeconds == other.ndAllowClearTimeoutSeconds && self.saIdleTimeout == other.saIdleTimeout && self.emPolicy == other.emPolicy
    }
}
impl ::core::cmp::Eq for IPSEC_TRANSPORT_POLICY2 {}
impl ::core::fmt::Debug for IPSEC_TRANSPORT_POLICY2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_TRANSPORT_POLICY2").field("numIpsecProposals", &self.numIpsecProposals).field("ipsecProposals", &self.ipsecProposals).field("flags", &self.flags).field("ndAllowClearTimeoutSeconds", &self.ndAllowClearTimeoutSeconds).field("saIdleTimeout", &self.saIdleTimeout).field("emPolicy", &self.emPolicy).finish()
    }
}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINT0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINTS0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINTS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_TUNNEL_ENDPOINTS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_TUNNEL_POLICY0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_TUNNEL_POLICY1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_TUNNEL_POLICY2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_TUNNEL_POLICY3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for IPSEC_V4_UDP_ENCAPSULATION0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_V4_UDP_ENCAPSULATION0 {
    fn eq(&self, other: &Self) -> bool {
        self.localUdpEncapPort == other.localUdpEncapPort && self.remoteUdpEncapPort == other.remoteUdpEncapPort
    }
}
impl ::core::cmp::Eq for IPSEC_V4_UDP_ENCAPSULATION0 {}
impl ::core::fmt::Debug for IPSEC_V4_UDP_ENCAPSULATION0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_V4_UDP_ENCAPSULATION0").field("localUdpEncapPort", &self.localUdpEncapPort).field("remoteUdpEncapPort", &self.remoteUdpEncapPort).finish()
    }
}
impl ::core::default::Default for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    fn eq(&self, other: &Self) -> bool {
        self.virtualIfTunnelId == other.virtualIfTunnelId && self.trafficSelectorId == other.trafficSelectorId
    }
}
impl ::core::cmp::Eq for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {}
impl ::core::fmt::Debug for IPSEC_VIRTUAL_IF_TUNNEL_INFO0 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("IPSEC_VIRTUAL_IF_TUNNEL_INFO0").field("virtualIfTunnelId", &self.virtualIfTunnelId).field("trafficSelectorId", &self.trafficSelectorId).finish()
    }
}
