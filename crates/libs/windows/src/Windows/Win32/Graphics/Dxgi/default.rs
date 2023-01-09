#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_ADAPTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description && self.VendorId == other.VendorId && self.DeviceId == other.DeviceId && self.SubSysId == other.SubSysId && self.Revision == other.Revision && self.DedicatedVideoMemory == other.DedicatedVideoMemory && self.DedicatedSystemMemory == other.DedicatedSystemMemory && self.SharedSystemMemory == other.SharedSystemMemory && self.AdapterLuid == other.AdapterLuid
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC").field("Description", &self.Description).field("VendorId", &self.VendorId).field("DeviceId", &self.DeviceId).field("SubSysId", &self.SubSysId).field("Revision", &self.Revision).field("DedicatedVideoMemory", &self.DedicatedVideoMemory).field("DedicatedSystemMemory", &self.DedicatedSystemMemory).field("SharedSystemMemory", &self.SharedSystemMemory).field("AdapterLuid", &self.AdapterLuid).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_ADAPTER_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description && self.VendorId == other.VendorId && self.DeviceId == other.DeviceId && self.SubSysId == other.SubSysId && self.Revision == other.Revision && self.DedicatedVideoMemory == other.DedicatedVideoMemory && self.DedicatedSystemMemory == other.DedicatedSystemMemory && self.SharedSystemMemory == other.SharedSystemMemory && self.AdapterLuid == other.AdapterLuid && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC1").field("Description", &self.Description).field("VendorId", &self.VendorId).field("DeviceId", &self.DeviceId).field("SubSysId", &self.SubSysId).field("Revision", &self.Revision).field("DedicatedVideoMemory", &self.DedicatedVideoMemory).field("DedicatedSystemMemory", &self.DedicatedSystemMemory).field("SharedSystemMemory", &self.SharedSystemMemory).field("AdapterLuid", &self.AdapterLuid).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_ADAPTER_DESC2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC2 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description && self.VendorId == other.VendorId && self.DeviceId == other.DeviceId && self.SubSysId == other.SubSysId && self.Revision == other.Revision && self.DedicatedVideoMemory == other.DedicatedVideoMemory && self.DedicatedSystemMemory == other.DedicatedSystemMemory && self.SharedSystemMemory == other.SharedSystemMemory && self.AdapterLuid == other.AdapterLuid && self.Flags == other.Flags && self.GraphicsPreemptionGranularity == other.GraphicsPreemptionGranularity && self.ComputePreemptionGranularity == other.ComputePreemptionGranularity
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC2")
            .field("Description", &self.Description)
            .field("VendorId", &self.VendorId)
            .field("DeviceId", &self.DeviceId)
            .field("SubSysId", &self.SubSysId)
            .field("Revision", &self.Revision)
            .field("DedicatedVideoMemory", &self.DedicatedVideoMemory)
            .field("DedicatedSystemMemory", &self.DedicatedSystemMemory)
            .field("SharedSystemMemory", &self.SharedSystemMemory)
            .field("AdapterLuid", &self.AdapterLuid)
            .field("Flags", &self.Flags)
            .field("GraphicsPreemptionGranularity", &self.GraphicsPreemptionGranularity)
            .field("ComputePreemptionGranularity", &self.ComputePreemptionGranularity)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_ADAPTER_DESC3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_ADAPTER_DESC3 {
    fn eq(&self, other: &Self) -> bool {
        self.Description == other.Description && self.VendorId == other.VendorId && self.DeviceId == other.DeviceId && self.SubSysId == other.SubSysId && self.Revision == other.Revision && self.DedicatedVideoMemory == other.DedicatedVideoMemory && self.DedicatedSystemMemory == other.DedicatedSystemMemory && self.SharedSystemMemory == other.SharedSystemMemory && self.AdapterLuid == other.AdapterLuid && self.Flags == other.Flags && self.GraphicsPreemptionGranularity == other.GraphicsPreemptionGranularity && self.ComputePreemptionGranularity == other.ComputePreemptionGranularity
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_ADAPTER_DESC3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_ADAPTER_DESC3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_ADAPTER_DESC3")
            .field("Description", &self.Description)
            .field("VendorId", &self.VendorId)
            .field("DeviceId", &self.DeviceId)
            .field("SubSysId", &self.SubSysId)
            .field("Revision", &self.Revision)
            .field("DedicatedVideoMemory", &self.DedicatedVideoMemory)
            .field("DedicatedSystemMemory", &self.DedicatedSystemMemory)
            .field("SharedSystemMemory", &self.SharedSystemMemory)
            .field("AdapterLuid", &self.AdapterLuid)
            .field("Flags", &self.Flags)
            .field("GraphicsPreemptionGranularity", &self.GraphicsPreemptionGranularity)
            .field("ComputePreemptionGranularity", &self.ComputePreemptionGranularity)
            .finish()
    }
}
impl ::core::default::Default for DXGI_ADAPTER_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_ADAPTER_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_ADAPTER_FLAG").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_ADAPTER_FLAG {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_ADAPTER_FLAG {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_ADAPTER_FLAG {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DXGI_ADAPTER_FLAG3 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_ADAPTER_FLAG3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_ADAPTER_FLAG3").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_ADAPTER_FLAG3 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_ADAPTER_FLAG3 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_ADAPTER_FLAG3 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_COMPUTE_PREEMPTION_GRANULARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_COMPUTE_PREEMPTION_GRANULARITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_DEBUG_RLO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_DEBUG_RLO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_DEBUG_RLO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_DEBUG_RLO_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_DEBUG_RLO_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_DEBUG_RLO_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for DXGI_DECODE_SWAP_CHAIN_DESC {}
impl ::core::fmt::Debug for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_DECODE_SWAP_CHAIN_DESC").field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for DXGI_DISPLAY_COLOR_SPACE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_DISPLAY_COLOR_SPACE {
    fn eq(&self, other: &Self) -> bool {
        self.PrimaryCoordinates == other.PrimaryCoordinates && self.WhitePoints == other.WhitePoints
    }
}
impl ::core::cmp::Eq for DXGI_DISPLAY_COLOR_SPACE {}
impl ::core::fmt::Debug for DXGI_DISPLAY_COLOR_SPACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_DISPLAY_COLOR_SPACE").field("PrimaryCoordinates", &self.PrimaryCoordinates).field("WhitePoints", &self.WhitePoints).finish()
    }
}
impl ::core::default::Default for DXGI_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_FEATURE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_FRAME_PRESENTATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_FRAME_PRESENTATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_FRAME_PRESENTATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_FRAME_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_FRAME_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.PresentCount == other.PresentCount && self.PresentRefreshCount == other.PresentRefreshCount && self.SyncRefreshCount == other.SyncRefreshCount && self.SyncQPCTime == other.SyncQPCTime && self.SyncGPUTime == other.SyncGPUTime
    }
}
impl ::core::cmp::Eq for DXGI_FRAME_STATISTICS {}
impl ::core::fmt::Debug for DXGI_FRAME_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_FRAME_STATISTICS").field("PresentCount", &self.PresentCount).field("PresentRefreshCount", &self.PresentRefreshCount).field("SyncRefreshCount", &self.SyncRefreshCount).field("SyncQPCTime", &self.SyncQPCTime).field("SyncGPUTime", &self.SyncGPUTime).finish()
    }
}
impl ::core::default::Default for DXGI_FRAME_STATISTICS_MEDIA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_FRAME_STATISTICS_MEDIA {
    fn eq(&self, other: &Self) -> bool {
        self.PresentCount == other.PresentCount && self.PresentRefreshCount == other.PresentRefreshCount && self.SyncRefreshCount == other.SyncRefreshCount && self.SyncQPCTime == other.SyncQPCTime && self.SyncGPUTime == other.SyncGPUTime && self.CompositionMode == other.CompositionMode && self.ApprovedPresentDuration == other.ApprovedPresentDuration
    }
}
impl ::core::cmp::Eq for DXGI_FRAME_STATISTICS_MEDIA {}
impl ::core::fmt::Debug for DXGI_FRAME_STATISTICS_MEDIA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_FRAME_STATISTICS_MEDIA").field("PresentCount", &self.PresentCount).field("PresentRefreshCount", &self.PresentRefreshCount).field("SyncRefreshCount", &self.SyncRefreshCount).field("SyncQPCTime", &self.SyncQPCTime).field("SyncGPUTime", &self.SyncGPUTime).field("CompositionMode", &self.CompositionMode).field("ApprovedPresentDuration", &self.ApprovedPresentDuration).finish()
    }
}
impl ::core::default::Default for DXGI_GPU_PREFERENCE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_GPU_PREFERENCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_GPU_PREFERENCE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_GRAPHICS_PREEMPTION_GRANULARITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_GRAPHICS_PREEMPTION_GRANULARITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for DXGI_HDR_METADATA_HDR10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_HDR_METADATA_HDR10 {
    fn eq(&self, other: &Self) -> bool {
        self.RedPrimary == other.RedPrimary && self.GreenPrimary == other.GreenPrimary && self.BluePrimary == other.BluePrimary && self.WhitePoint == other.WhitePoint && self.MaxMasteringLuminance == other.MaxMasteringLuminance && self.MinMasteringLuminance == other.MinMasteringLuminance && self.MaxContentLightLevel == other.MaxContentLightLevel && self.MaxFrameAverageLightLevel == other.MaxFrameAverageLightLevel
    }
}
impl ::core::cmp::Eq for DXGI_HDR_METADATA_HDR10 {}
impl ::core::fmt::Debug for DXGI_HDR_METADATA_HDR10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_HDR_METADATA_HDR10").field("RedPrimary", &self.RedPrimary).field("GreenPrimary", &self.GreenPrimary).field("BluePrimary", &self.BluePrimary).field("WhitePoint", &self.WhitePoint).field("MaxMasteringLuminance", &self.MaxMasteringLuminance).field("MinMasteringLuminance", &self.MinMasteringLuminance).field("MaxContentLightLevel", &self.MaxContentLightLevel).field("MaxFrameAverageLightLevel", &self.MaxFrameAverageLightLevel).finish()
    }
}
impl ::core::default::Default for DXGI_HDR_METADATA_HDR10PLUS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_HDR_METADATA_HDR10PLUS {
    fn eq(&self, other: &Self) -> bool {
        self.Data == other.Data
    }
}
impl ::core::cmp::Eq for DXGI_HDR_METADATA_HDR10PLUS {}
impl ::core::fmt::Debug for DXGI_HDR_METADATA_HDR10PLUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_HDR_METADATA_HDR10PLUS").field("Data", &self.Data).finish()
    }
}
impl ::core::default::Default for DXGI_HDR_METADATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_HDR_METADATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_HDR_METADATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_INFO_QUEUE_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_INFO_QUEUE_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.AllowList == other.AllowList && self.DenyList == other.DenyList
    }
}
impl ::core::cmp::Eq for DXGI_INFO_QUEUE_FILTER {}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_INFO_QUEUE_FILTER").field("AllowList", &self.AllowList).field("DenyList", &self.DenyList).finish()
    }
}
impl ::core::default::Default for DXGI_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_INFO_QUEUE_FILTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NumCategories == other.NumCategories && self.pCategoryList == other.pCategoryList && self.NumSeverities == other.NumSeverities && self.pSeverityList == other.pSeverityList && self.NumIDs == other.NumIDs && self.pIDList == other.pIDList
    }
}
impl ::core::cmp::Eq for DXGI_INFO_QUEUE_FILTER_DESC {}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_FILTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_INFO_QUEUE_FILTER_DESC").field("NumCategories", &self.NumCategories).field("pCategoryList", &self.pCategoryList).field("NumSeverities", &self.NumSeverities).field("pSeverityList", &self.pSeverityList).field("NumIDs", &self.NumIDs).field("pIDList", &self.pIDList).finish()
    }
}
impl ::core::default::Default for DXGI_INFO_QUEUE_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_INFO_QUEUE_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Producer == other.Producer && self.Category == other.Category && self.Severity == other.Severity && self.ID == other.ID && self.pDescription == other.pDescription && self.DescriptionByteLength == other.DescriptionByteLength
    }
}
impl ::core::cmp::Eq for DXGI_INFO_QUEUE_MESSAGE {}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_INFO_QUEUE_MESSAGE").field("Producer", &self.Producer).field("Category", &self.Category).field("Severity", &self.Severity).field("ID", &self.ID).field("pDescription", &self.pDescription).field("DescriptionByteLength", &self.DescriptionByteLength).finish()
    }
}
impl ::core::default::Default for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_INFO_QUEUE_MESSAGE_CATEGORY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_INFO_QUEUE_MESSAGE_SEVERITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_INFO_QUEUE_MESSAGE_SEVERITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_MAPPED_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_MAPPED_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.Pitch == other.Pitch && self.pBits == other.pBits
    }
}
impl ::core::cmp::Eq for DXGI_MAPPED_RECT {}
impl ::core::fmt::Debug for DXGI_MAPPED_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_MAPPED_RECT").field("Pitch", &self.Pitch).field("pBits", &self.pBits).finish()
    }
}
impl ::core::default::Default for DXGI_MATRIX_3X2_F {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_MATRIX_3X2_F {
    fn eq(&self, other: &Self) -> bool {
        self._11 == other._11 && self._12 == other._12 && self._21 == other._21 && self._22 == other._22 && self._31 == other._31 && self._32 == other._32
    }
}
impl ::core::cmp::Eq for DXGI_MATRIX_3X2_F {}
impl ::core::fmt::Debug for DXGI_MATRIX_3X2_F {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_MATRIX_3X2_F").field("_11", &self._11).field("_12", &self._12).field("_21", &self._21).field("_22", &self._22).field("_31", &self._31).field("_32", &self._32).finish()
    }
}
impl ::core::default::Default for DXGI_MEMORY_SEGMENT_GROUP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_MEMORY_SEGMENT_GROUP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_MEMORY_SEGMENT_GROUP").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_MODE_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_MODE_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.RefreshRate == other.RefreshRate && self.Format == other.Format && self.ScanlineOrdering == other.ScanlineOrdering && self.Scaling == other.Scaling && self.Stereo == other.Stereo
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_MODE_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_MODE_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_MODE_DESC1").field("Width", &self.Width).field("Height", &self.Height).field("RefreshRate", &self.RefreshRate).field("Format", &self.Format).field("ScanlineOrdering", &self.ScanlineOrdering).field("Scaling", &self.Scaling).field("Stereo", &self.Stereo).finish()
    }
}
impl ::core::default::Default for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_Message_Id {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_Message_Id {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_Message_Id").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_OFFER_RESOURCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_OFFER_RESOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OFFER_RESOURCE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_OFFER_RESOURCE_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_OFFER_RESOURCE_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OFFER_RESOURCE_PRIORITY").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_OUTDUPL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ModeDesc == other.ModeDesc && self.Rotation == other.Rotation && self.DesktopImageInSystemMemory == other.DesktopImageInSystemMemory
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_OUTDUPL_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_OUTDUPL_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_DESC").field("ModeDesc", &self.ModeDesc).field("Rotation", &self.Rotation).field("DesktopImageInSystemMemory", &self.DesktopImageInSystemMemory).finish()
    }
}
impl ::core::default::Default for DXGI_OUTDUPL_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_OUTDUPL_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OUTDUPL_FLAG").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_OUTDUPL_FRAME_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_FRAME_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LastPresentTime == other.LastPresentTime && self.LastMouseUpdateTime == other.LastMouseUpdateTime && self.AccumulatedFrames == other.AccumulatedFrames && self.RectsCoalesced == other.RectsCoalesced && self.ProtectedContentMaskedOut == other.ProtectedContentMaskedOut && self.PointerPosition == other.PointerPosition && self.TotalMetadataBufferSize == other.TotalMetadataBufferSize && self.PointerShapeBufferSize == other.PointerShapeBufferSize
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_OUTDUPL_FRAME_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_OUTDUPL_FRAME_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_FRAME_INFO")
            .field("LastPresentTime", &self.LastPresentTime)
            .field("LastMouseUpdateTime", &self.LastMouseUpdateTime)
            .field("AccumulatedFrames", &self.AccumulatedFrames)
            .field("RectsCoalesced", &self.RectsCoalesced)
            .field("ProtectedContentMaskedOut", &self.ProtectedContentMaskedOut)
            .field("PointerPosition", &self.PointerPosition)
            .field("TotalMetadataBufferSize", &self.TotalMetadataBufferSize)
            .field("PointerShapeBufferSize", &self.PointerShapeBufferSize)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_OUTDUPL_MOVE_RECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_MOVE_RECT {
    fn eq(&self, other: &Self) -> bool {
        self.SourcePoint == other.SourcePoint && self.DestinationRect == other.DestinationRect
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_OUTDUPL_MOVE_RECT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_OUTDUPL_MOVE_RECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_MOVE_RECT").field("SourcePoint", &self.SourcePoint).field("DestinationRect", &self.DestinationRect).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_OUTDUPL_POINTER_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_POINTER_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.Position == other.Position && self.Visible == other.Visible
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_OUTDUPL_POINTER_POSITION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_OUTDUPL_POINTER_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_POINTER_POSITION").field("Position", &self.Position).field("Visible", &self.Visible).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Width == other.Width && self.Height == other.Height && self.Pitch == other.Pitch && self.HotSpot == other.HotSpot
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_OUTDUPL_POINTER_SHAPE_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_OUTDUPL_POINTER_SHAPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTDUPL_POINTER_SHAPE_INFO").field("Type", &self.Type).field("Width", &self.Width).field("Height", &self.Height).field("Pitch", &self.Pitch).field("HotSpot", &self.HotSpot).finish()
    }
}
impl ::core::default::Default for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_OUTDUPL_POINTER_SHAPE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OUTDUPL_POINTER_SHAPE_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DXGI_OUTPUT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DXGI_OUTPUT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName && self.DesktopCoordinates == other.DesktopCoordinates && self.AttachedToDesktop == other.AttachedToDesktop && self.Rotation == other.Rotation && self.Monitor == other.Monitor
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DXGI_OUTPUT_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DXGI_OUTPUT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTPUT_DESC").field("DeviceName", &self.DeviceName).field("DesktopCoordinates", &self.DesktopCoordinates).field("AttachedToDesktop", &self.AttachedToDesktop).field("Rotation", &self.Rotation).field("Monitor", &self.Monitor).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::default::Default for DXGI_OUTPUT_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::PartialEq for DXGI_OUTPUT_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceName == other.DeviceName && self.DesktopCoordinates == other.DesktopCoordinates && self.AttachedToDesktop == other.AttachedToDesktop && self.Rotation == other.Rotation && self.Monitor == other.Monitor && self.BitsPerColor == other.BitsPerColor && self.ColorSpace == other.ColorSpace && self.RedPrimary == other.RedPrimary && self.GreenPrimary == other.GreenPrimary && self.BluePrimary == other.BluePrimary && self.WhitePoint == other.WhitePoint && self.MinLuminance == other.MinLuminance && self.MaxLuminance == other.MaxLuminance && self.MaxFullFrameLuminance == other.MaxFullFrameLuminance
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::cmp::Eq for DXGI_OUTPUT_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
impl ::core::fmt::Debug for DXGI_OUTPUT_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_OUTPUT_DESC1")
            .field("DeviceName", &self.DeviceName)
            .field("DesktopCoordinates", &self.DesktopCoordinates)
            .field("AttachedToDesktop", &self.AttachedToDesktop)
            .field("Rotation", &self.Rotation)
            .field("Monitor", &self.Monitor)
            .field("BitsPerColor", &self.BitsPerColor)
            .field("ColorSpace", &self.ColorSpace)
            .field("RedPrimary", &self.RedPrimary)
            .field("GreenPrimary", &self.GreenPrimary)
            .field("BluePrimary", &self.BluePrimary)
            .field("WhitePoint", &self.WhitePoint)
            .field("MinLuminance", &self.MinLuminance)
            .field("MaxLuminance", &self.MaxLuminance)
            .field("MaxFullFrameLuminance", &self.MaxFullFrameLuminance)
            .finish()
    }
}
impl ::core::default::Default for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_OVERLAY_SUPPORT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_OVERLAY_SUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_OVERLAY_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_PRESENT_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_PRESENT_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.DirtyRectsCount == other.DirtyRectsCount && self.pDirtyRects == other.pDirtyRects && self.pScrollRect == other.pScrollRect && self.pScrollOffset == other.pScrollOffset
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_PRESENT_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_PRESENT_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_PRESENT_PARAMETERS").field("DirtyRectsCount", &self.DirtyRectsCount).field("pDirtyRects", &self.pDirtyRects).field("pScrollRect", &self.pScrollRect).field("pScrollOffset", &self.pScrollOffset).finish()
    }
}
impl ::core::default::Default for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Budget == other.Budget && self.CurrentUsage == other.CurrentUsage && self.AvailableForReservation == other.AvailableForReservation && self.CurrentReservation == other.CurrentReservation
    }
}
impl ::core::cmp::Eq for DXGI_QUERY_VIDEO_MEMORY_INFO {}
impl ::core::fmt::Debug for DXGI_QUERY_VIDEO_MEMORY_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_QUERY_VIDEO_MEMORY_INFO").field("Budget", &self.Budget).field("CurrentUsage", &self.CurrentUsage).field("AvailableForReservation", &self.AvailableForReservation).field("CurrentReservation", &self.CurrentReservation).finish()
    }
}
impl ::core::default::Default for DXGI_RECLAIM_RESOURCE_RESULTS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_RECLAIM_RESOURCE_RESULTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_RECLAIM_RESOURCE_RESULTS").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_RESIDENCY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_RESIDENCY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_RESIDENCY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_RESOURCE_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_RESOURCE_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_RESOURCE_PRIORITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_RGBA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for DXGI_RGBA {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b && self.a == other.a
    }
}
impl ::core::cmp::Eq for DXGI_RGBA {}
impl ::core::fmt::Debug for DXGI_RGBA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_RGBA").field("r", &self.r).field("g", &self.g).field("b", &self.b).field("a", &self.a).finish()
    }
}
impl ::core::default::Default for DXGI_SCALING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_SCALING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SCALING").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for DXGI_SHARED_RESOURCE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for DXGI_SHARED_RESOURCE {
    fn eq(&self, other: &Self) -> bool {
        self.Handle == other.Handle
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for DXGI_SHARED_RESOURCE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for DXGI_SHARED_RESOURCE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SHARED_RESOURCE").field("Handle", &self.Handle).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for DXGI_SURFACE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for DXGI_SURFACE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Format == other.Format && self.SampleDesc == other.SampleDesc
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for DXGI_SURFACE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for DXGI_SURFACE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SURFACE_DESC").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("SampleDesc", &self.SampleDesc).finish()
    }
}
impl ::core::default::Default for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_SWAP_CHAIN_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_SWAP_CHAIN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.BufferDesc == other.BufferDesc && self.SampleDesc == other.SampleDesc && self.BufferUsage == other.BufferUsage && self.BufferCount == other.BufferCount && self.OutputWindow == other.OutputWindow && self.Windowed == other.Windowed && self.SwapEffect == other.SwapEffect && self.Flags == other.Flags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_SWAP_CHAIN_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SWAP_CHAIN_DESC").field("BufferDesc", &self.BufferDesc).field("SampleDesc", &self.SampleDesc).field("BufferUsage", &self.BufferUsage).field("BufferCount", &self.BufferCount).field("OutputWindow", &self.OutputWindow).field("Windowed", &self.Windowed).field("SwapEffect", &self.SwapEffect).field("Flags", &self.Flags).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_SWAP_CHAIN_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_SWAP_CHAIN_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Format == other.Format && self.Stereo == other.Stereo && self.SampleDesc == other.SampleDesc && self.BufferUsage == other.BufferUsage && self.BufferCount == other.BufferCount && self.Scaling == other.Scaling && self.SwapEffect == other.SwapEffect && self.AlphaMode == other.AlphaMode && self.Flags == other.Flags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_SWAP_CHAIN_DESC1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SWAP_CHAIN_DESC1").field("Width", &self.Width).field("Height", &self.Height).field("Format", &self.Format).field("Stereo", &self.Stereo).field("SampleDesc", &self.SampleDesc).field("BufferUsage", &self.BufferUsage).field("BufferCount", &self.BufferCount).field("Scaling", &self.Scaling).field("SwapEffect", &self.SwapEffect).field("AlphaMode", &self.AlphaMode).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for DXGI_SWAP_CHAIN_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SWAP_CHAIN_FLAG").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.RefreshRate == other.RefreshRate && self.ScanlineOrdering == other.ScanlineOrdering && self.Scaling == other.Scaling && self.Windowed == other.Windowed
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("DXGI_SWAP_CHAIN_FULLSCREEN_DESC").field("RefreshRate", &self.RefreshRate).field("ScanlineOrdering", &self.ScanlineOrdering).field("Scaling", &self.Scaling).field("Windowed", &self.Windowed).finish()
    }
}
impl ::core::default::Default for DXGI_SWAP_EFFECT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_SWAP_EFFECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_SWAP_EFFECT").field(&self.0).finish()
    }
}
impl ::core::default::Default for DXGI_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for DXGI_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("DXGI_USAGE").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for DXGI_USAGE {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for DXGI_USAGE {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for DXGI_USAGE {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for DXGI_USAGE {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for DXGI_USAGE {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::cmp::PartialEq for IDXGIAdapter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter {}
impl ::core::fmt::Debug for IDXGIAdapter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter").field(&self.0).finish()
    }
}
impl IDXGIAdapter {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIAdapter1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter1 {}
impl ::core::fmt::Debug for IDXGIAdapter1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter1").field(&self.0).finish()
    }
}
impl IDXGIAdapter1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumOutputs)(::windows::core::Vtable::as_raw(self), output, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckInterfaceSupport)(::windows::core::Vtable::as_raw(self), interfacename, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIAdapter2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter2 {}
impl ::core::fmt::Debug for IDXGIAdapter2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter2").field(&self.0).finish()
    }
}
impl IDXGIAdapter2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumOutputs)(::windows::core::Vtable::as_raw(self), output, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CheckInterfaceSupport)(::windows::core::Vtable::as_raw(self), interfacename, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc1)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
}
impl ::core::cmp::PartialEq for IDXGIAdapter3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter3 {}
impl ::core::fmt::Debug for IDXGIAdapter3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter3").field(&self.0).finish()
    }
}
impl IDXGIAdapter3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnumOutputs)(::windows::core::Vtable::as_raw(self), output, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CheckInterfaceSupport)(::windows::core::Vtable::as_raw(self), interfacename, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDesc1)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc2)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
}
impl ::core::cmp::PartialEq for IDXGIAdapter4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIAdapter4 {}
impl ::core::fmt::Debug for IDXGIAdapter4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIAdapter4").field(&self.0).finish()
    }
}
impl IDXGIAdapter4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumOutputs(&self, output: u32) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.EnumOutputs)(::windows::core::Vtable::as_raw(self), output, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_ADAPTER_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn CheckInterfaceSupport(&self, interfacename: *const ::windows::core::GUID) -> ::windows::core::Result<i64> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CheckInterfaceSupport)(::windows::core::Vtable::as_raw(self), interfacename, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_ADAPTER_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDesc1)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc2(&self, pdesc: *mut DXGI_ADAPTER_DESC2) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDesc2)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterHardwareContentProtectionTeardownStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RegisterHardwareContentProtectionTeardownStatusEvent)(::windows::core::Vtable::as_raw(self), hevent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterHardwareContentProtectionTeardownStatus(&self, dwcookie: u32) {
        (::windows::core::Vtable::vtable(self).base__.UnregisterHardwareContentProtectionTeardownStatus)(::windows::core::Vtable::as_raw(self), dwcookie)
    }
    pub unsafe fn QueryVideoMemoryInfo(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, pvideomemoryinfo: *mut DXGI_QUERY_VIDEO_MEMORY_INFO) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueryVideoMemoryInfo)(::windows::core::Vtable::as_raw(self), nodeindex, memorysegmentgroup, pvideomemoryinfo).ok()
    }
    pub unsafe fn SetVideoMemoryReservation(&self, nodeindex: u32, memorysegmentgroup: DXGI_MEMORY_SEGMENT_GROUP, reservation: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetVideoMemoryReservation)(::windows::core::Vtable::as_raw(self), nodeindex, memorysegmentgroup, reservation).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterVideoMemoryBudgetChangeNotificationEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RegisterVideoMemoryBudgetChangeNotificationEvent)(::windows::core::Vtable::as_raw(self), hevent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterVideoMemoryBudgetChangeNotification(&self, dwcookie: u32) {
        (::windows::core::Vtable::vtable(self).base__.UnregisterVideoMemoryBudgetChangeNotification)(::windows::core::Vtable::as_raw(self), dwcookie)
    }
}
impl ::core::cmp::PartialEq for IDXGIDebug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDebug {}
impl ::core::fmt::Debug for IDXGIDebug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDebug").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDXGIDebug1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDebug1 {}
impl ::core::fmt::Debug for IDXGIDebug1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDebug1").field(&self.0).finish()
    }
}
impl IDXGIDebug1 {
    pub unsafe fn ReportLiveObjects(&self, apiid: ::windows::core::GUID, flags: DXGI_DEBUG_RLO_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReportLiveObjects)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(apiid), flags).ok()
    }
}
impl ::core::cmp::PartialEq for IDXGIDecodeSwapChain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDecodeSwapChain {}
impl ::core::fmt::Debug for IDXGIDecodeSwapChain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDecodeSwapChain").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDXGIDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice {}
impl ::core::fmt::Debug for IDXGIDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice").field(&self.0).finish()
    }
}
impl IDXGIDevice {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIDevice1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice1 {}
impl ::core::fmt::Debug for IDXGIDevice1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice1").field(&self.0).finish()
    }
}
impl IDXGIDevice1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAdapter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: ::core::option::Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateSurface)(::windows::core::Vtable::as_raw(self), pdesc, ppsurface.len() as _, usage, ::core::mem::transmute(psharedresource.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsurface.as_ptr())).ok()
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows::core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.QueryResourceResidency)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppresources), presidencystatus, numresources).ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGPUThreadPriority)(::windows::core::Vtable::as_raw(self), priority).ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetGPUThreadPriority)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice2 {}
impl ::core::fmt::Debug for IDXGIDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice2").field(&self.0).finish()
    }
}
impl IDXGIDevice2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetAdapter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: ::core::option::Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateSurface)(::windows::core::Vtable::as_raw(self), pdesc, ppsurface.len() as _, usage, ::core::mem::transmute(psharedresource.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsurface.as_ptr())).ok()
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows::core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.QueryResourceResidency)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppresources), presidencystatus, numresources).ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGPUThreadPriority)(::windows::core::Vtable::as_raw(self), priority).ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetGPUThreadPriority)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMaximumFrameLatency)(::windows::core::Vtable::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMaximumFrameLatency)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIDevice3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice3 {}
impl ::core::fmt::Debug for IDXGIDevice3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice3").field(&self.0).finish()
    }
}
impl IDXGIDevice3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetAdapter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: ::core::option::Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateSurface)(::windows::core::Vtable::as_raw(self), pdesc, ppsurface.len() as _, usage, ::core::mem::transmute(psharedresource.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsurface.as_ptr())).ok()
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows::core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.QueryResourceResidency)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppresources), presidencystatus, numresources).ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetGPUThreadPriority)(::windows::core::Vtable::as_raw(self), priority).ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetGPUThreadPriority)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMaximumFrameLatency)(::windows::core::Vtable::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMaximumFrameLatency)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OfferResources(&self, ppresources: &[IDXGIResource], priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.OfferResources)(::windows::core::Vtable::as_raw(self), ppresources.len() as _, ::core::mem::transmute(ppresources.as_ptr()), priority).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReclaimResources(&self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, pdiscarded: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReclaimResources)(::windows::core::Vtable::as_raw(self), numresources, ::core::mem::transmute(ppresources), ::core::mem::transmute(pdiscarded.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnqueueSetEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnqueueSetEvent)(::windows::core::Vtable::as_raw(self), hevent.into()).ok()
    }
}
impl ::core::cmp::PartialEq for IDXGIDevice4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDevice4 {}
impl ::core::fmt::Debug for IDXGIDevice4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDevice4").field(&self.0).finish()
    }
}
impl IDXGIDevice4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAdapter(&self) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetAdapter)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSurface(&self, pdesc: *const DXGI_SURFACE_DESC, usage: DXGI_USAGE, psharedresource: ::core::option::Option<*const DXGI_SHARED_RESOURCE>, ppsurface: &mut [::core::option::Option<IDXGISurface>]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateSurface)(::windows::core::Vtable::as_raw(self), pdesc, ppsurface.len() as _, usage, ::core::mem::transmute(psharedresource.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsurface.as_ptr())).ok()
    }
    pub unsafe fn QueryResourceResidency(&self, ppresources: *const ::core::option::Option<::windows::core::IUnknown>, presidencystatus: *mut DXGI_RESIDENCY, numresources: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.QueryResourceResidency)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppresources), presidencystatus, numresources).ok()
    }
    pub unsafe fn SetGPUThreadPriority(&self, priority: i32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetGPUThreadPriority)(::windows::core::Vtable::as_raw(self), priority).ok()
    }
    pub unsafe fn GetGPUThreadPriority(&self) -> ::windows::core::Result<i32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetGPUThreadPriority)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetMaximumFrameLatency)(::windows::core::Vtable::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetMaximumFrameLatency)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OfferResources(&self, ppresources: &[IDXGIResource], priority: DXGI_OFFER_RESOURCE_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.OfferResources)(::windows::core::Vtable::as_raw(self), ppresources.len() as _, ::core::mem::transmute(ppresources.as_ptr()), priority).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReclaimResources(&self, numresources: u32, ppresources: *const ::core::option::Option<IDXGIResource>, pdiscarded: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ReclaimResources)(::windows::core::Vtable::as_raw(self), numresources, ::core::mem::transmute(ppresources), ::core::mem::transmute(pdiscarded.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnqueueSetEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EnqueueSetEvent)(::windows::core::Vtable::as_raw(self), hevent.into()).ok()
    }
    pub unsafe fn Trim(&self) {
        (::windows::core::Vtable::vtable(self).base__.Trim)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDXGIDeviceSubObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDeviceSubObject {}
impl ::core::fmt::Debug for IDXGIDeviceSubObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDeviceSubObject").field(&self.0).finish()
    }
}
impl IDXGIDeviceSubObject {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIDisplayControl {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIDisplayControl {}
impl ::core::fmt::Debug for IDXGIDisplayControl {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIDisplayControl").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory {}
impl ::core::fmt::Debug for IDXGIFactory {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory").field(&self.0).finish()
    }
}
impl IDXGIFactory {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory1 {}
impl ::core::fmt::Debug for IDXGIFactory1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory1").field(&self.0).finish()
    }
}
impl IDXGIFactory1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumAdapters)(::windows::core::Vtable::as_raw(self), adapter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.MakeWindowAssociation)(::windows::core::Vtable::as_raw(self), windowhandle.into(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetWindowAssociation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateSwapChain)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSoftwareAdapter)(::windows::core::Vtable::as_raw(self), module.into(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory2 {}
impl ::core::fmt::Debug for IDXGIFactory2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory2").field(&self.0).finish()
    }
}
impl IDXGIFactory2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumAdapters)(::windows::core::Vtable::as_raw(self), adapter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.MakeWindowAssociation)(::windows::core::Vtable::as_raw(self), windowhandle.into(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetWindowAssociation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateSwapChain)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateSoftwareAdapter)(::windows::core::Vtable::as_raw(self), module.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumAdapters1)(::windows::core::Vtable::as_raw(self), adapter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsCurrent)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory3 {}
impl ::core::fmt::Debug for IDXGIFactory3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory3").field(&self.0).finish()
    }
}
impl IDXGIFactory3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnumAdapters)(::windows::core::Vtable::as_raw(self), adapter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MakeWindowAssociation)(::windows::core::Vtable::as_raw(self), windowhandle.into(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetWindowAssociation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateSwapChain)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateSoftwareAdapter)(::windows::core::Vtable::as_raw(self), module.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumAdapters1)(::windows::core::Vtable::as_raw(self), adapter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsCurrent)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsWindowedStereoEnabled)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        P2: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSwapChainForHwnd)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), hwnd.into(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSwapChainForCoreWindow)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pwindow.into().abi(), pdesc, prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows::core::Result<super::super::Foundation::LUID>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSharedResourceAdapterLuid)(::windows::core::Vtable::as_raw(self), hresource.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RegisterStereoStatusWindow)(::windows::core::Vtable::as_raw(self), windowhandle.into(), wmsg, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RegisterStereoStatusEvent)(::windows::core::Vtable::as_raw(self), hevent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Vtable::vtable(self).base__.UnregisterStereoStatus)(::windows::core::Vtable::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RegisterOcclusionStatusWindow)(::windows::core::Vtable::as_raw(self), windowhandle.into(), wmsg, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RegisterOcclusionStatusEvent)(::windows::core::Vtable::as_raw(self), hevent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Vtable::vtable(self).base__.UnregisterOcclusionStatus)(::windows::core::Vtable::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSwapChainForComposition)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pdesc, prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory4 {}
impl ::core::fmt::Debug for IDXGIFactory4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory4").field(&self.0).finish()
    }
}
impl IDXGIFactory4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.EnumAdapters)(::windows::core::Vtable::as_raw(self), adapter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.MakeWindowAssociation)(::windows::core::Vtable::as_raw(self), windowhandle.into(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetWindowAssociation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateSwapChain)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateSoftwareAdapter)(::windows::core::Vtable::as_raw(self), module.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnumAdapters1)(::windows::core::Vtable::as_raw(self), adapter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsCurrent)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsWindowedStereoEnabled)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        P2: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateSwapChainForHwnd)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), hwnd.into(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateSwapChainForCoreWindow)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pwindow.into().abi(), pdesc, prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows::core::Result<super::super::Foundation::LUID>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetSharedResourceAdapterLuid)(::windows::core::Vtable::as_raw(self), hresource.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RegisterStereoStatusWindow)(::windows::core::Vtable::as_raw(self), windowhandle.into(), wmsg, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RegisterStereoStatusEvent)(::windows::core::Vtable::as_raw(self), hevent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.UnregisterStereoStatus)(::windows::core::Vtable::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RegisterOcclusionStatusWindow)(::windows::core::Vtable::as_raw(self), windowhandle.into(), wmsg, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.RegisterOcclusionStatusEvent)(::windows::core::Vtable::as_raw(self), hevent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.UnregisterOcclusionStatus)(::windows::core::Vtable::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateSwapChainForComposition)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pdesc, prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetCreationFlags)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory5 {}
impl ::core::fmt::Debug for IDXGIFactory5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory5").field(&self.0).finish()
    }
}
impl IDXGIFactory5 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.EnumAdapters)(::windows::core::Vtable::as_raw(self), adapter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.MakeWindowAssociation)(::windows::core::Vtable::as_raw(self), windowhandle.into(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetWindowAssociation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateSwapChain)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateSoftwareAdapter)(::windows::core::Vtable::as_raw(self), module.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.EnumAdapters1)(::windows::core::Vtable::as_raw(self), adapter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsCurrent)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsWindowedStereoEnabled)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        P2: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateSwapChainForHwnd)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), hwnd.into(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateSwapChainForCoreWindow)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pwindow.into().abi(), pdesc, prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows::core::Result<super::super::Foundation::LUID>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetSharedResourceAdapterLuid)(::windows::core::Vtable::as_raw(self), hresource.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RegisterStereoStatusWindow)(::windows::core::Vtable::as_raw(self), windowhandle.into(), wmsg, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RegisterStereoStatusEvent)(::windows::core::Vtable::as_raw(self), hevent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.UnregisterStereoStatus)(::windows::core::Vtable::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RegisterOcclusionStatusWindow)(::windows::core::Vtable::as_raw(self), windowhandle.into(), wmsg, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RegisterOcclusionStatusEvent)(::windows::core::Vtable::as_raw(self), hevent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.UnregisterOcclusionStatus)(::windows::core::Vtable::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateSwapChainForComposition)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pdesc, prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetCreationFlags)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAdapterByLuid<T>(&self, adapterluid: super::super::Foundation::LUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumAdapterByLuid)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(adapterluid), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumWarpAdapter<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumWarpAdapter)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory6 {}
impl ::core::fmt::Debug for IDXGIFactory6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory6").field(&self.0).finish()
    }
}
impl IDXGIFactory6 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.EnumAdapters)(::windows::core::Vtable::as_raw(self), adapter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.MakeWindowAssociation)(::windows::core::Vtable::as_raw(self), windowhandle.into(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetWindowAssociation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateSwapChain)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateSoftwareAdapter)(::windows::core::Vtable::as_raw(self), module.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.EnumAdapters1)(::windows::core::Vtable::as_raw(self), adapter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IsCurrent)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IsWindowedStereoEnabled)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        P2: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateSwapChainForHwnd)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), hwnd.into(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateSwapChainForCoreWindow)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pwindow.into().abi(), pdesc, prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows::core::Result<super::super::Foundation::LUID>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetSharedResourceAdapterLuid)(::windows::core::Vtable::as_raw(self), hresource.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RegisterStereoStatusWindow)(::windows::core::Vtable::as_raw(self), windowhandle.into(), wmsg, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RegisterStereoStatusEvent)(::windows::core::Vtable::as_raw(self), hevent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.UnregisterStereoStatus)(::windows::core::Vtable::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RegisterOcclusionStatusWindow)(::windows::core::Vtable::as_raw(self), windowhandle.into(), wmsg, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RegisterOcclusionStatusEvent)(::windows::core::Vtable::as_raw(self), hevent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.UnregisterOcclusionStatus)(::windows::core::Vtable::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateSwapChainForComposition)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pdesc, prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCreationFlags)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAdapterByLuid<T>(&self, adapterluid: super::super::Foundation::LUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumAdapterByLuid)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(adapterluid), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumWarpAdapter<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.EnumWarpAdapter)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
}
impl ::core::cmp::PartialEq for IDXGIFactory7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactory7 {}
impl ::core::fmt::Debug for IDXGIFactory7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactory7").field(&self.0).finish()
    }
}
impl IDXGIFactory7 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumAdapters(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.EnumAdapters)(::windows::core::Vtable::as_raw(self), adapter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn MakeWindowAssociation<P0>(&self, windowhandle: P0, flags: u32) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.MakeWindowAssociation)(::windows::core::Vtable::as_raw(self), windowhandle.into(), flags).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetWindowAssociation(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetWindowAssociation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChain<P0>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC, ppswapchain: *mut ::core::option::Option<IDXGISwapChain>) -> ::windows::core::HRESULT
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateSwapChain)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pdesc, ::core::mem::transmute(ppswapchain))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateSoftwareAdapter<P0>(&self, module: P0) -> ::windows::core::Result<IDXGIAdapter>
    where
        P0: ::std::convert::Into<super::super::Foundation::HINSTANCE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateSoftwareAdapter)(::windows::core::Vtable::as_raw(self), module.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumAdapters1(&self, adapter: u32) -> ::windows::core::Result<IDXGIAdapter1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.EnumAdapters1)(::windows::core::Vtable::as_raw(self), adapter, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsCurrent(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.IsCurrent)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsWindowedStereoEnabled(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IsWindowedStereoEnabled)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForHwnd<P0, P1, P2>(&self, pdevice: P0, hwnd: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, pfullscreendesc: ::core::option::Option<*const DXGI_SWAP_CHAIN_FULLSCREEN_DESC>, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::HWND>,
        P2: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateSwapChainForHwnd)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), hwnd.into(), pdesc, ::core::mem::transmute(pfullscreendesc.unwrap_or(::std::ptr::null())), prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForCoreWindow<P0, P1, P2>(&self, pdevice: P0, pwindow: P1, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P2) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P2: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateSwapChainForCoreWindow)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pwindow.into().abi(), pdesc, prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedResourceAdapterLuid<P0>(&self, hresource: P0) -> ::windows::core::Result<super::super::Foundation::LUID>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetSharedResourceAdapterLuid)(::windows::core::Vtable::as_raw(self), hresource.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RegisterStereoStatusWindow)(::windows::core::Vtable::as_raw(self), windowhandle.into(), wmsg, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterStereoStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RegisterStereoStatusEvent)(::windows::core::Vtable::as_raw(self), hevent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterStereoStatus(&self, dwcookie: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.UnregisterStereoStatus)(::windows::core::Vtable::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusWindow<P0>(&self, windowhandle: P0, wmsg: u32) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HWND>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RegisterOcclusionStatusWindow)(::windows::core::Vtable::as_raw(self), windowhandle.into(), wmsg, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RegisterOcclusionStatusEvent<P0>(&self, hevent: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RegisterOcclusionStatusEvent)(::windows::core::Vtable::as_raw(self), hevent.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn UnregisterOcclusionStatus(&self, dwcookie: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.UnregisterOcclusionStatus)(::windows::core::Vtable::as_raw(self), dwcookie)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateSwapChainForComposition<P0, P1>(&self, pdevice: P0, pdesc: *const DXGI_SWAP_CHAIN_DESC1, prestricttooutput: P1) -> ::windows::core::Result<IDXGISwapChain1>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateSwapChainForComposition)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), pdesc, prestricttooutput.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCreationFlags)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn EnumAdapterByLuid<T>(&self, adapterluid: super::super::Foundation::LUID) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnumAdapterByLuid)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(adapterluid), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnumWarpAdapter<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnumWarpAdapter)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: DXGI_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn EnumAdapterByGpuPreference<T>(&self, adapter: u32, gpupreference: DXGI_GPU_PREFERENCE) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.EnumAdapterByGpuPreference)(::windows::core::Vtable::as_raw(self), adapter, gpupreference, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIFactoryMedia {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIFactoryMedia {}
impl ::core::fmt::Debug for IDXGIFactoryMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIFactoryMedia").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDXGIInfoQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIInfoQueue {}
impl ::core::fmt::Debug for IDXGIInfoQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIInfoQueue").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDXGIKeyedMutex {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIKeyedMutex {}
impl ::core::fmt::Debug for IDXGIKeyedMutex {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIKeyedMutex").field(&self.0).finish()
    }
}
impl IDXGIKeyedMutex {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIObject {}
impl ::core::fmt::Debug for IDXGIObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIObject").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDXGIOutput {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput {}
impl ::core::fmt::Debug for IDXGIOutput {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput").field(&self.0).finish()
    }
}
impl IDXGIOutput {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIOutput1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput1 {}
impl ::core::fmt::Debug for IDXGIOutput1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput1").field(&self.0).finish()
    }
}
impl IDXGIOutput1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDisplayModeList)(::windows::core::Vtable::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FindClosestMatchingMode)(::windows::core::Vtable::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.WaitForVBlank)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.TakeOwnership)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), exclusive.into()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Vtable::vtable(self).base__.ReleaseOwnership)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetGammaControlCapabilities)(::windows::core::Vtable::as_raw(self), pgammacaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetGammaControl)(::windows::core::Vtable::as_raw(self), parray).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetGammaControl)(::windows::core::Vtable::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGISurface>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetDisplaySurface)(::windows::core::Vtable::as_raw(self), pscanoutsurface.into().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGISurface>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDisplaySurfaceData)(::windows::core::Vtable::as_raw(self), pdestination.into().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFrameStatistics)(::windows::core::Vtable::as_raw(self), pstats).ok()
    }
}
impl ::core::cmp::PartialEq for IDXGIOutput2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput2 {}
impl ::core::fmt::Debug for IDXGIOutput2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput2").field(&self.0).finish()
    }
}
impl IDXGIOutput2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDisplayModeList)(::windows::core::Vtable::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FindClosestMatchingMode)(::windows::core::Vtable::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.WaitForVBlank)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.TakeOwnership)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), exclusive.into()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.ReleaseOwnership)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGammaControlCapabilities)(::windows::core::Vtable::as_raw(self), pgammacaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGammaControl)(::windows::core::Vtable::as_raw(self), parray).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGammaControl)(::windows::core::Vtable::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGISurface>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDisplaySurface)(::windows::core::Vtable::as_raw(self), pscanoutsurface.into().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGISurface>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDisplaySurfaceData)(::windows::core::Vtable::as_raw(self), pdestination.into().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFrameStatistics)(::windows::core::Vtable::as_raw(self), pstats).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDisplayModeList1)(::windows::core::Vtable::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.FindClosestMatchingMode1)(::windows::core::Vtable::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGIResource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDisplaySurfaceData1)(::windows::core::Vtable::as_raw(self), pdestination.into().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows::core::Result<IDXGIOutputDuplication>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DuplicateOutput)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIOutput3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput3 {}
impl ::core::fmt::Debug for IDXGIOutput3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput3").field(&self.0).finish()
    }
}
impl IDXGIOutput3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDisplayModeList)(::windows::core::Vtable::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindClosestMatchingMode)(::windows::core::Vtable::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.WaitForVBlank)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.TakeOwnership)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), exclusive.into()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ReleaseOwnership)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetGammaControlCapabilities)(::windows::core::Vtable::as_raw(self), pgammacaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetGammaControl)(::windows::core::Vtable::as_raw(self), parray).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetGammaControl)(::windows::core::Vtable::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGISurface>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetDisplaySurface)(::windows::core::Vtable::as_raw(self), pscanoutsurface.into().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGISurface>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDisplaySurfaceData)(::windows::core::Vtable::as_raw(self), pdestination.into().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFrameStatistics)(::windows::core::Vtable::as_raw(self), pstats).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDisplayModeList1)(::windows::core::Vtable::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.FindClosestMatchingMode1)(::windows::core::Vtable::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGIResource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDisplaySurfaceData1)(::windows::core::Vtable::as_raw(self), pdestination.into().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows::core::Result<IDXGIOutputDuplication>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.DuplicateOutput)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.SupportsOverlays)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for IDXGIOutput4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput4 {}
impl ::core::fmt::Debug for IDXGIOutput4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput4").field(&self.0).finish()
    }
}
impl IDXGIOutput4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDisplayModeList)(::windows::core::Vtable::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FindClosestMatchingMode)(::windows::core::Vtable::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.WaitForVBlank)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.TakeOwnership)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), exclusive.into()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ReleaseOwnership)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetGammaControlCapabilities)(::windows::core::Vtable::as_raw(self), pgammacaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetGammaControl)(::windows::core::Vtable::as_raw(self), parray).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetGammaControl)(::windows::core::Vtable::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGISurface>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetDisplaySurface)(::windows::core::Vtable::as_raw(self), pscanoutsurface.into().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGISurface>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDisplaySurfaceData)(::windows::core::Vtable::as_raw(self), pdestination.into().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFrameStatistics)(::windows::core::Vtable::as_raw(self), pstats).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDisplayModeList1)(::windows::core::Vtable::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.FindClosestMatchingMode1)(::windows::core::Vtable::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGIResource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDisplaySurfaceData1)(::windows::core::Vtable::as_raw(self), pdestination.into().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows::core::Result<IDXGIOutputDuplication>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DuplicateOutput)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.SupportsOverlays)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<P0>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckOverlaySupport)(::windows::core::Vtable::as_raw(self), enumformat, pconcerneddevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIOutput5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput5 {}
impl ::core::fmt::Debug for IDXGIOutput5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput5").field(&self.0).finish()
    }
}
impl IDXGIOutput5 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDisplayModeList)(::windows::core::Vtable::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FindClosestMatchingMode)(::windows::core::Vtable::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.WaitForVBlank)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.TakeOwnership)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), exclusive.into()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ReleaseOwnership)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetGammaControlCapabilities)(::windows::core::Vtable::as_raw(self), pgammacaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetGammaControl)(::windows::core::Vtable::as_raw(self), parray).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetGammaControl)(::windows::core::Vtable::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGISurface>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetDisplaySurface)(::windows::core::Vtable::as_raw(self), pscanoutsurface.into().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGISurface>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDisplaySurfaceData)(::windows::core::Vtable::as_raw(self), pdestination.into().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetFrameStatistics)(::windows::core::Vtable::as_raw(self), pstats).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDisplayModeList1)(::windows::core::Vtable::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.FindClosestMatchingMode1)(::windows::core::Vtable::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGIResource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDisplaySurfaceData1)(::windows::core::Vtable::as_raw(self), pdestination.into().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows::core::Result<IDXGIOutputDuplication>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DuplicateOutput)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SupportsOverlays)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<P0>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CheckOverlaySupport)(::windows::core::Vtable::as_raw(self), enumformat, pconcerneddevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlayColorSpaceSupport<P0>(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckOverlayColorSpaceSupport)(::windows::core::Vtable::as_raw(self), format, colorspace, pconcerneddevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIOutput6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutput6 {}
impl ::core::fmt::Debug for IDXGIOutput6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutput6").field(&self.0).finish()
    }
}
impl IDXGIOutput6 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_OUTPUT_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDisplayModeList(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut Common::DXGI_MODE_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetDisplayModeList)(::windows::core::Vtable::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn FindClosestMatchingMode<P0>(&self, pmodetomatch: *const Common::DXGI_MODE_DESC, pclosestmatch: *mut Common::DXGI_MODE_DESC, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.FindClosestMatchingMode)(::windows::core::Vtable::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into().abi()).ok()
    }
    pub unsafe fn WaitForVBlank(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.WaitForVBlank)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn TakeOwnership<P0, P1>(&self, pdevice: P0, exclusive: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.TakeOwnership)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), exclusive.into()).ok()
    }
    pub unsafe fn ReleaseOwnership(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ReleaseOwnership)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetGammaControlCapabilities(&self, pgammacaps: *mut Common::DXGI_GAMMA_CONTROL_CAPABILITIES) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetGammaControlCapabilities)(::windows::core::Vtable::as_raw(self), pgammacaps).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetGammaControl(&self, parray: *const Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetGammaControl)(::windows::core::Vtable::as_raw(self), parray).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetGammaControl(&self, parray: *mut Common::DXGI_GAMMA_CONTROL) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetGammaControl)(::windows::core::Vtable::as_raw(self), parray).ok()
    }
    pub unsafe fn SetDisplaySurface<P0>(&self, pscanoutsurface: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGISurface>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetDisplaySurface)(::windows::core::Vtable::as_raw(self), pscanoutsurface.into().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGISurface>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetDisplaySurfaceData)(::windows::core::Vtable::as_raw(self), pdestination.into().abi()).ok()
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetFrameStatistics)(::windows::core::Vtable::as_raw(self), pstats).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDisplayModeList1(&self, enumformat: Common::DXGI_FORMAT, flags: u32, pnummodes: *mut u32, pdesc: ::core::option::Option<*mut DXGI_MODE_DESC1>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDisplayModeList1)(::windows::core::Vtable::as_raw(self), enumformat, flags, pnummodes, ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn FindClosestMatchingMode1<P0>(&self, pmodetomatch: *const DXGI_MODE_DESC1, pclosestmatch: *mut DXGI_MODE_DESC1, pconcerneddevice: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.FindClosestMatchingMode1)(::windows::core::Vtable::as_raw(self), pmodetomatch, pclosestmatch, pconcerneddevice.into().abi()).ok()
    }
    pub unsafe fn GetDisplaySurfaceData1<P0>(&self, pdestination: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<IDXGIResource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDisplaySurfaceData1)(::windows::core::Vtable::as_raw(self), pdestination.into().abi()).ok()
    }
    pub unsafe fn DuplicateOutput<P0>(&self, pdevice: P0) -> ::windows::core::Result<IDXGIOutputDuplication>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DuplicateOutput)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SupportsOverlays(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SupportsOverlays)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlaySupport<P0>(&self, enumformat: Common::DXGI_FORMAT, pconcerneddevice: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CheckOverlaySupport)(::windows::core::Vtable::as_raw(self), enumformat, pconcerneddevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckOverlayColorSpaceSupport<P0>(&self, format: Common::DXGI_FORMAT, colorspace: Common::DXGI_COLOR_SPACE_TYPE, pconcerneddevice: P0) -> ::windows::core::Result<u32>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CheckOverlayColorSpaceSupport)(::windows::core::Vtable::as_raw(self), format, colorspace, pconcerneddevice.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn DuplicateOutput1<P0>(&self, pdevice: P0, flags: u32, psupportedformats: &[Common::DXGI_FORMAT]) -> ::windows::core::Result<IDXGIOutputDuplication>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.DuplicateOutput1)(::windows::core::Vtable::as_raw(self), pdevice.into().abi(), flags, psupportedformats.len() as _, ::core::mem::transmute(psupportedformats.as_ptr()), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIOutputDuplication {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIOutputDuplication {}
impl ::core::fmt::Debug for IDXGIOutputDuplication {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIOutputDuplication").field(&self.0).finish()
    }
}
impl IDXGIOutputDuplication {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIResource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIResource {}
impl ::core::fmt::Debug for IDXGIResource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIResource").field(&self.0).finish()
    }
}
impl IDXGIResource {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGIResource1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGIResource1 {}
impl ::core::fmt::Debug for IDXGIResource1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGIResource1").field(&self.0).finish()
    }
}
impl IDXGIResource1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetSharedHandle(&self) -> ::windows::core::Result<super::super::Foundation::HANDLE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetSharedHandle)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetUsage(&self) -> ::windows::core::Result<DXGI_USAGE> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetUsage)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: DXGI_RESOURCE_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetEvictionPriority)(::windows::core::Vtable::as_raw(self), evictionpriority).ok()
    }
    pub unsafe fn GetEvictionPriority(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetEvictionPriority)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGISurface {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISurface {}
impl ::core::fmt::Debug for IDXGISurface {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISurface").field(&self.0).finish()
    }
}
impl IDXGISurface {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGISurface1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISurface1 {}
impl ::core::fmt::Debug for IDXGISurface1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISurface1").field(&self.0).finish()
    }
}
impl IDXGISurface1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Map)(::windows::core::Vtable::as_raw(self), plockedrect, mapflags).ok()
    }
    pub unsafe fn Unmap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Unmap)(::windows::core::Vtable::as_raw(self)).ok()
    }
}
impl ::core::cmp::PartialEq for IDXGISurface2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISurface2 {}
impl ::core::fmt::Debug for IDXGISurface2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISurface2").field(&self.0).finish()
    }
}
impl IDXGISurface2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SURFACE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn Map(&self, plockedrect: *mut DXGI_MAPPED_RECT, mapflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Map)(::windows::core::Vtable::as_raw(self), plockedrect, mapflags).ok()
    }
    pub unsafe fn Unmap(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Unmap)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Gdi\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub unsafe fn GetDC<P0>(&self, discard: P0) -> ::windows::core::Result<super::Gdi::HDC>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDC)(::windows::core::Vtable::as_raw(self), discard.into(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ReleaseDC(&self, pdirtyrect: ::core::option::Option<*const super::super::Foundation::RECT>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReleaseDC)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdirtyrect.unwrap_or(::std::ptr::null()))).ok()
    }
}
impl ::core::cmp::PartialEq for IDXGISwapChain {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain {}
impl ::core::fmt::Debug for IDXGISwapChain {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain").field(&self.0).finish()
    }
}
impl IDXGISwapChain {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGISwapChain1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain1 {}
impl ::core::fmt::Debug for IDXGISwapChain1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain1").field(&self.0).finish()
    }
}
impl IDXGISwapChain1 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.Present)(::windows::core::Vtable::as_raw(self), syncinterval, flags)
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBuffer)(::windows::core::Vtable::as_raw(self), buffer, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<P0, P1>(&self, fullscreen: P0, ptarget: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetFullscreenState)(::windows::core::Vtable::as_raw(self), fullscreen.into(), ptarget.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(&self, pfullscreen: ::core::option::Option<*mut super::super::Foundation::BOOL>, pptarget: ::core::option::Option<*mut ::core::option::Option<IDXGIOutput>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFullscreenState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfullscreen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pptarget.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ResizeBuffers)(::windows::core::Vtable::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ResizeTarget)(::windows::core::Vtable::as_raw(self), pnewtargetparameters).ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetContainingOutput)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFrameStatistics)(::windows::core::Vtable::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetLastPresentCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGISwapChain2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain2 {}
impl ::core::fmt::Debug for IDXGISwapChain2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain2").field(&self.0).finish()
    }
}
impl IDXGISwapChain2 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.Present)(::windows::core::Vtable::as_raw(self), syncinterval, flags)
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetBuffer)(::windows::core::Vtable::as_raw(self), buffer, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<P0, P1>(&self, fullscreen: P0, ptarget: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetFullscreenState)(::windows::core::Vtable::as_raw(self), fullscreen.into(), ptarget.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(&self, pfullscreen: ::core::option::Option<*mut super::super::Foundation::BOOL>, pptarget: ::core::option::Option<*mut ::core::option::Option<IDXGIOutput>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFullscreenState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfullscreen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pptarget.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ResizeBuffers)(::windows::core::Vtable::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ResizeTarget)(::windows::core::Vtable::as_raw(self), pnewtargetparameters).ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetContainingOutput)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFrameStatistics)(::windows::core::Vtable::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetLastPresentCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc1)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetFullscreenDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetHwnd)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCoreWindow)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.Present1)(::windows::core::Vtable::as_raw(self), syncinterval, presentflags, ppresentparameters)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsTemporaryMonoSupported)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRestrictToOutput)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetBackgroundColor)(::windows::core::Vtable::as_raw(self), pcolor).ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::core::Result<DXGI_RGBA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetBackgroundColor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRotation)(::windows::core::Vtable::as_raw(self), rotation).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows::core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetRotation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for IDXGISwapChain3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain3 {}
impl ::core::fmt::Debug for IDXGISwapChain3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain3").field(&self.0).finish()
    }
}
impl IDXGISwapChain3 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Present)(::windows::core::Vtable::as_raw(self), syncinterval, flags)
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetBuffer)(::windows::core::Vtable::as_raw(self), buffer, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<P0, P1>(&self, fullscreen: P0, ptarget: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetFullscreenState)(::windows::core::Vtable::as_raw(self), fullscreen.into(), ptarget.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(&self, pfullscreen: ::core::option::Option<*mut super::super::Foundation::BOOL>, pptarget: ::core::option::Option<*mut ::core::option::Option<IDXGIOutput>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFullscreenState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfullscreen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pptarget.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ResizeBuffers)(::windows::core::Vtable::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ResizeTarget)(::windows::core::Vtable::as_raw(self), pnewtargetparameters).ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetContainingOutput)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFrameStatistics)(::windows::core::Vtable::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetLastPresentCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDesc1)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFullscreenDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetHwnd)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCoreWindow)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.Present1)(::windows::core::Vtable::as_raw(self), syncinterval, presentflags, ppresentparameters)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.IsTemporaryMonoSupported)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetRestrictToOutput)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetBackgroundColor)(::windows::core::Vtable::as_raw(self), pcolor).ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::core::Result<DXGI_RGBA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetBackgroundColor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetRotation)(::windows::core::Vtable::as_raw(self), rotation).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows::core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetRotation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetSourceSize)(::windows::core::Vtable::as_raw(self), width, height).ok()
    }
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetSourceSize)(::windows::core::Vtable::as_raw(self), pwidth, pheight).ok()
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMaximumFrameLatency)(::windows::core::Vtable::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetMaximumFrameLatency)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Vtable::vtable(self).base__.GetFrameLatencyWaitableObject)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMatrixTransform)(::windows::core::Vtable::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMatrixTransform)(::windows::core::Vtable::as_raw(self), pmatrix).ok()
    }
}
impl ::core::cmp::PartialEq for IDXGISwapChain4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChain4 {}
impl ::core::fmt::Debug for IDXGISwapChain4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChain4").field(&self.0).finish()
    }
}
impl IDXGISwapChain4 {
    pub unsafe fn SetPrivateData(&self, name: *const ::windows::core::GUID, datasize: u32, pdata: *const ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), name, datasize, pdata).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, name: *const ::windows::core::GUID, punknown: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), name, punknown.into().abi()).ok()
    }
    pub unsafe fn GetPrivateData(&self, name: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: *mut ::core::ffi::c_void) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), name, pdatasize, pdata).ok()
    }
    pub unsafe fn GetParent<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetParent)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDevice<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn Present(&self, syncinterval: u32, flags: u32) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Present)(::windows::core::Vtable::as_raw(self), syncinterval, flags)
    }
    pub unsafe fn GetBuffer<T>(&self, buffer: u32) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetBuffer)(::windows::core::Vtable::as_raw(self), buffer, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetFullscreenState<P0, P1>(&self, fullscreen: P0, ptarget: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
        P1: ::std::convert::Into<::windows::core::InParam<IDXGIOutput>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetFullscreenState)(::windows::core::Vtable::as_raw(self), fullscreen.into(), ptarget.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFullscreenState(&self, pfullscreen: ::core::option::Option<*mut super::super::Foundation::BOOL>, pptarget: ::core::option::Option<*mut ::core::option::Option<IDXGIOutput>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFullscreenState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfullscreen.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pptarget.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers(&self, buffercount: u32, width: u32, height: u32, newformat: Common::DXGI_FORMAT, swapchainflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ResizeBuffers)(::windows::core::Vtable::as_raw(self), buffercount, width, height, newformat, swapchainflags).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeTarget(&self, pnewtargetparameters: *const Common::DXGI_MODE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ResizeTarget)(::windows::core::Vtable::as_raw(self), pnewtargetparameters).ok()
    }
    pub unsafe fn GetContainingOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetContainingOutput)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetFrameStatistics(&self, pstats: *mut DXGI_FRAME_STATISTICS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetFrameStatistics)(::windows::core::Vtable::as_raw(self), pstats).ok()
    }
    pub unsafe fn GetLastPresentCount(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetLastPresentCount)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc1(&self, pdesc: *mut DXGI_SWAP_CHAIN_DESC1) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDesc1)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetFullscreenDesc(&self, pdesc: *mut DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetFullscreenDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetHwnd(&self) -> ::windows::core::Result<super::super::Foundation::HWND> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetHwnd)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetCoreWindow<T>(&self) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCoreWindow)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn Present1(&self, syncinterval: u32, presentflags: u32, ppresentparameters: *const DXGI_PRESENT_PARAMETERS) -> ::windows::core::HRESULT {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Present1)(::windows::core::Vtable::as_raw(self), syncinterval, presentflags, ppresentparameters)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsTemporaryMonoSupported(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IsTemporaryMonoSupported)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetRestrictToOutput(&self) -> ::windows::core::Result<IDXGIOutput> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRestrictToOutput)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetBackgroundColor(&self, pcolor: *const DXGI_RGBA) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetBackgroundColor)(::windows::core::Vtable::as_raw(self), pcolor).ok()
    }
    pub unsafe fn GetBackgroundColor(&self) -> ::windows::core::Result<DXGI_RGBA> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetBackgroundColor)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetRotation(&self, rotation: Common::DXGI_MODE_ROTATION) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetRotation)(::windows::core::Vtable::as_raw(self), rotation).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRotation(&self) -> ::windows::core::Result<Common::DXGI_MODE_ROTATION> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRotation)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn SetSourceSize(&self, width: u32, height: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSourceSize)(::windows::core::Vtable::as_raw(self), width, height).ok()
    }
    pub unsafe fn GetSourceSize(&self, pwidth: *mut u32, pheight: *mut u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSourceSize)(::windows::core::Vtable::as_raw(self), pwidth, pheight).ok()
    }
    pub unsafe fn SetMaximumFrameLatency(&self, maxlatency: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMaximumFrameLatency)(::windows::core::Vtable::as_raw(self), maxlatency).ok()
    }
    pub unsafe fn GetMaximumFrameLatency(&self) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetMaximumFrameLatency)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetFrameLatencyWaitableObject(&self) -> super::super::Foundation::HANDLE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetFrameLatencyWaitableObject)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetMatrixTransform(&self, pmatrix: *const DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMatrixTransform)(::windows::core::Vtable::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetMatrixTransform(&self, pmatrix: *mut DXGI_MATRIX_3X2_F) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetMatrixTransform)(::windows::core::Vtable::as_raw(self), pmatrix).ok()
    }
    pub unsafe fn GetCurrentBackBufferIndex(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetCurrentBackBufferIndex)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckColorSpaceSupport(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckColorSpaceSupport)(::windows::core::Vtable::as_raw(self), colorspace, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn SetColorSpace1(&self, colorspace: Common::DXGI_COLOR_SPACE_TYPE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetColorSpace1)(::windows::core::Vtable::as_raw(self), colorspace).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResizeBuffers1(&self, buffercount: u32, width: u32, height: u32, format: Common::DXGI_FORMAT, swapchainflags: u32, pcreationnodemask: *const u32, pppresentqueue: *const ::core::option::Option<::windows::core::IUnknown>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ResizeBuffers1)(::windows::core::Vtable::as_raw(self), buffercount, width, height, format, swapchainflags, pcreationnodemask, ::core::mem::transmute(pppresentqueue)).ok()
    }
}
impl ::core::cmp::PartialEq for IDXGISwapChainMedia {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGISwapChainMedia {}
impl ::core::fmt::Debug for IDXGISwapChainMedia {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGISwapChainMedia").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for IDXGraphicsAnalysis {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for IDXGraphicsAnalysis {}
impl ::core::fmt::Debug for IDXGraphicsAnalysis {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("IDXGraphicsAnalysis").field(&self.0).finish()
    }
}
