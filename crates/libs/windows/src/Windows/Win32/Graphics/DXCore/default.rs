impl ::core::default::Default for DXCoreAdapterMemoryBudget {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXCoreAdapterMemoryBudget {
    fn eq(&self, other: &Self) -> bool {
        self.budget == other.budget && self.currentUsage == other.currentUsage && self.availableForReservation == other.availableForReservation && self.currentReservation == other.currentReservation
    }
}
impl ::core::cmp::Eq for DXCoreAdapterMemoryBudget {}
impl ::core::fmt::Debug for DXCoreAdapterMemoryBudget {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXCoreAdapterMemoryBudget").field("budget", &self.budget).field("currentUsage", &self.currentUsage).field("availableForReservation", &self.availableForReservation).field("currentReservation", &self.currentReservation).finish()
    }
}
impl ::core::default::Default for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn eq(&self, other: &Self) -> bool {
        self.nodeIndex == other.nodeIndex && self.segmentGroup == other.segmentGroup
    }
}
impl ::core::cmp::Eq for DXCoreAdapterMemoryBudgetNodeSegmentGroup {}
impl ::core::fmt::Debug for DXCoreAdapterMemoryBudgetNodeSegmentGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXCoreAdapterMemoryBudgetNodeSegmentGroup").field("nodeIndex", &self.nodeIndex).field("segmentGroup", &self.segmentGroup).finish()
    }
}
impl ::core::default::Default for DXCoreAdapterPreference {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXCoreAdapterPreference {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXCoreAdapterPreference").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXCoreAdapterProperty {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXCoreAdapterProperty {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXCoreAdapterProperty").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXCoreAdapterState {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXCoreAdapterState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXCoreAdapterState").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXCoreHardwareID {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXCoreHardwareID {
    fn eq(&self, other: &Self) -> bool {
        self.vendorID == other.vendorID && self.deviceID == other.deviceID && self.subSysID == other.subSysID && self.revision == other.revision
    }
}
impl ::core::cmp::Eq for DXCoreHardwareID {}
impl ::core::fmt::Debug for DXCoreHardwareID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXCoreHardwareID").field("vendorID", &self.vendorID).field("deviceID", &self.deviceID).field("subSysID", &self.subSysID).field("revision", &self.revision).finish()
    }
}
impl ::core::default::Default for DXCoreHardwareIDParts {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXCoreHardwareIDParts {
    fn eq(&self, other: &Self) -> bool {
        self.vendorID == other.vendorID && self.deviceID == other.deviceID && self.subSystemID == other.subSystemID && self.subVendorID == other.subVendorID && self.revisionID == other.revisionID
    }
}
impl ::core::cmp::Eq for DXCoreHardwareIDParts {}
impl ::core::fmt::Debug for DXCoreHardwareIDParts {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXCoreHardwareIDParts").field("vendorID", &self.vendorID).field("deviceID", &self.deviceID).field("subSystemID", &self.subSystemID).field("subVendorID", &self.subVendorID).field("revisionID", &self.revisionID).finish()
    }
}
impl ::core::default::Default for DXCoreNotificationType {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXCoreNotificationType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXCoreNotificationType").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXCoreSegmentGroup {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXCoreSegmentGroup {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXCoreSegmentGroup").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDXCoreAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXCoreAdapter {}
impl ::core::fmt::Debug for IDXCoreAdapter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXCoreAdapter").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDXCoreAdapterFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXCoreAdapterFactory {}
impl ::core::fmt::Debug for IDXCoreAdapterFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXCoreAdapterFactory").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDXCoreAdapterList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXCoreAdapterList {}
impl ::core::fmt::Debug for IDXCoreAdapterList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXCoreAdapterList").field(&self.0).finish()
    }
}
