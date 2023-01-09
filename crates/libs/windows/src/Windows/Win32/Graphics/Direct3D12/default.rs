impl ::core::default::Default for D3D12_AUTO_BREADCRUMB_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_AUTO_BREADCRUMB_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.pCommandListDebugNameA == other.pCommandListDebugNameA && self.pCommandListDebugNameW == other.pCommandListDebugNameW && self.pCommandQueueDebugNameA == other.pCommandQueueDebugNameA && self.pCommandQueueDebugNameW == other.pCommandQueueDebugNameW && self.pCommandList == other.pCommandList && self.pCommandQueue == other.pCommandQueue && self.BreadcrumbCount == other.BreadcrumbCount && self.pLastBreadcrumbValue == other.pLastBreadcrumbValue && self.pCommandHistory == other.pCommandHistory && self.pNext == other.pNext
    }
}
impl ::core::cmp::Eq for D3D12_AUTO_BREADCRUMB_NODE {}
impl ::core::fmt::Debug for D3D12_AUTO_BREADCRUMB_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_AUTO_BREADCRUMB_NODE")
            .field("pCommandListDebugNameA", &self.pCommandListDebugNameA)
            .field("pCommandListDebugNameW", &self.pCommandListDebugNameW)
            .field("pCommandQueueDebugNameA", &self.pCommandQueueDebugNameA)
            .field("pCommandQueueDebugNameW", &self.pCommandQueueDebugNameW)
            .field("pCommandList", &self.pCommandList)
            .field("pCommandQueue", &self.pCommandQueue)
            .field("BreadcrumbCount", &self.BreadcrumbCount)
            .field("pLastBreadcrumbValue", &self.pLastBreadcrumbValue)
            .field("pCommandHistory", &self.pCommandHistory)
            .field("pNext", &self.pNext)
            .finish()
    }
}
impl ::core::default::Default for D3D12_AUTO_BREADCRUMB_NODE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_AUTO_BREADCRUMB_NODE1 {
    fn eq(&self, other: &Self) -> bool {
        self.pCommandListDebugNameA == other.pCommandListDebugNameA && self.pCommandListDebugNameW == other.pCommandListDebugNameW && self.pCommandQueueDebugNameA == other.pCommandQueueDebugNameA && self.pCommandQueueDebugNameW == other.pCommandQueueDebugNameW && self.pCommandList == other.pCommandList && self.pCommandQueue == other.pCommandQueue && self.BreadcrumbCount == other.BreadcrumbCount && self.pLastBreadcrumbValue == other.pLastBreadcrumbValue && self.pCommandHistory == other.pCommandHistory && self.pNext == other.pNext && self.BreadcrumbContextsCount == other.BreadcrumbContextsCount && self.pBreadcrumbContexts == other.pBreadcrumbContexts
    }
}
impl ::core::cmp::Eq for D3D12_AUTO_BREADCRUMB_NODE1 {}
impl ::core::fmt::Debug for D3D12_AUTO_BREADCRUMB_NODE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_AUTO_BREADCRUMB_NODE1")
            .field("pCommandListDebugNameA", &self.pCommandListDebugNameA)
            .field("pCommandListDebugNameW", &self.pCommandListDebugNameW)
            .field("pCommandQueueDebugNameA", &self.pCommandQueueDebugNameA)
            .field("pCommandQueueDebugNameW", &self.pCommandQueueDebugNameW)
            .field("pCommandList", &self.pCommandList)
            .field("pCommandQueue", &self.pCommandQueue)
            .field("BreadcrumbCount", &self.BreadcrumbCount)
            .field("pLastBreadcrumbValue", &self.pLastBreadcrumbValue)
            .field("pCommandHistory", &self.pCommandHistory)
            .field("pNext", &self.pNext)
            .field("BreadcrumbContextsCount", &self.BreadcrumbContextsCount)
            .field("pBreadcrumbContexts", &self.pBreadcrumbContexts)
            .finish()
    }
}
impl ::core::default::Default for D3D12_AUTO_BREADCRUMB_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_AUTO_BREADCRUMB_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_AUTO_BREADCRUMB_OP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_AXIS_SHADING_RATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_AXIS_SHADING_RATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_AXIS_SHADING_RATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_BACKGROUND_PROCESSING_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_BACKGROUND_PROCESSING_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_BACKGROUND_PROCESSING_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_BLEND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_BLEND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_BLEND").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_BLEND_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_BLEND_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.AlphaToCoverageEnable == other.AlphaToCoverageEnable && self.IndependentBlendEnable == other.IndependentBlendEnable && self.RenderTarget == other.RenderTarget
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_BLEND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_BLEND_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_BLEND_DESC").field("AlphaToCoverageEnable", &self.AlphaToCoverageEnable).field("IndependentBlendEnable", &self.IndependentBlendEnable).field("RenderTarget", &self.RenderTarget).finish()
    }
}
impl ::core::default::Default for D3D12_BLEND_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_BLEND_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_BLEND_OP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_BOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_BOX {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.front == other.front && self.right == other.right && self.bottom == other.bottom && self.back == other.back
    }
}
impl ::core::cmp::Eq for D3D12_BOX {}
impl ::core::fmt::Debug for D3D12_BOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_BOX").field("left", &self.left).field("top", &self.top).field("front", &self.front).field("right", &self.right).field("bottom", &self.bottom).field("back", &self.back).finish()
    }
}
impl ::core::default::Default for D3D12_BUFFER_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_BUFFER_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.FirstElement == other.FirstElement && self.NumElements == other.NumElements
    }
}
impl ::core::cmp::Eq for D3D12_BUFFER_RTV {}
impl ::core::fmt::Debug for D3D12_BUFFER_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_BUFFER_RTV").field("FirstElement", &self.FirstElement).field("NumElements", &self.NumElements).finish()
    }
}
impl ::core::default::Default for D3D12_BUFFER_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_BUFFER_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.FirstElement == other.FirstElement && self.NumElements == other.NumElements && self.StructureByteStride == other.StructureByteStride && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D12_BUFFER_SRV {}
impl ::core::fmt::Debug for D3D12_BUFFER_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_BUFFER_SRV").field("FirstElement", &self.FirstElement).field("NumElements", &self.NumElements).field("StructureByteStride", &self.StructureByteStride).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D12_BUFFER_SRV_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_BUFFER_SRV_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_BUFFER_SRV_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_BUFFER_SRV_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_BUFFER_SRV_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_BUFFER_SRV_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_BUFFER_SRV_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_BUFFER_SRV_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_BUFFER_UAV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_BUFFER_UAV {
    fn eq(&self, other: &Self) -> bool {
        self.FirstElement == other.FirstElement && self.NumElements == other.NumElements && self.StructureByteStride == other.StructureByteStride && self.CounterOffsetInBytes == other.CounterOffsetInBytes && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D12_BUFFER_UAV {}
impl ::core::fmt::Debug for D3D12_BUFFER_UAV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_BUFFER_UAV").field("FirstElement", &self.FirstElement).field("NumElements", &self.NumElements).field("StructureByteStride", &self.StructureByteStride).field("CounterOffsetInBytes", &self.CounterOffsetInBytes).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D12_BUFFER_UAV_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_BUFFER_UAV_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_BUFFER_UAV_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_BUFFER_UAV_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_BUFFER_UAV_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_BUFFER_UAV_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_BUFFER_UAV_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_BUFFER_UAV_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.NumDescs == other.NumDescs
    }
}
impl ::core::cmp::Eq for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER {}
impl ::core::fmt::Debug for D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_TOOLS_VISUALIZATION_HEADER").field("Type", &self.Type).field("NumDescs", &self.NumDescs).finish()
    }
}
impl ::core::default::Default for D3D12_CACHED_PIPELINE_STATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_CACHED_PIPELINE_STATE {
    fn eq(&self, other: &Self) -> bool {
        self.pCachedBlob == other.pCachedBlob && self.CachedBlobSizeInBytes == other.CachedBlobSizeInBytes
    }
}
impl ::core::cmp::Eq for D3D12_CACHED_PIPELINE_STATE {}
impl ::core::fmt::Debug for D3D12_CACHED_PIPELINE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_CACHED_PIPELINE_STATE").field("pCachedBlob", &self.pCachedBlob).field("CachedBlobSizeInBytes", &self.CachedBlobSizeInBytes).finish()
    }
}
impl ::core::default::Default for D3D12_CLEAR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_CLEAR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_CLEAR_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_CLEAR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_CLEAR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_CLEAR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_CLEAR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_CLEAR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_CLEAR_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_COLOR_WRITE_ENABLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_COLOR_WRITE_ENABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_COLOR_WRITE_ENABLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_COMMAND_LIST_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_COMMAND_LIST_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_COMMAND_LIST_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_COMMAND_LIST_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_COMMAND_LIST_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_COMMAND_LIST_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_COMMAND_LIST_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_COMMAND_LIST_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_COMMAND_LIST_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_COMMAND_LIST_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_COMMAND_LIST_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_COMMAND_LIST_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_COMMAND_LIST_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_COMMAND_POOL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_COMMAND_POOL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_COMMAND_POOL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_COMMAND_POOL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_COMMAND_POOL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_COMMAND_POOL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_COMMAND_POOL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_COMMAND_POOL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_COMMAND_QUEUE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_COMMAND_QUEUE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Priority == other.Priority && self.Flags == other.Flags && self.NodeMask == other.NodeMask
    }
}
impl ::core::cmp::Eq for D3D12_COMMAND_QUEUE_DESC {}
impl ::core::fmt::Debug for D3D12_COMMAND_QUEUE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_COMMAND_QUEUE_DESC").field("Type", &self.Type).field("Priority", &self.Priority).field("Flags", &self.Flags).field("NodeMask", &self.NodeMask).finish()
    }
}
impl ::core::default::Default for D3D12_COMMAND_QUEUE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_COMMAND_QUEUE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_COMMAND_QUEUE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_COMMAND_QUEUE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_COMMAND_QUEUE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_COMMAND_QUEUE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_COMMAND_QUEUE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_COMMAND_QUEUE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_COMMAND_QUEUE_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_COMMAND_QUEUE_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_COMMAND_QUEUE_PRIORITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_COMMAND_RECORDER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_COMMAND_RECORDER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_COMMAND_RECORDER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_COMMAND_RECORDER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_COMMAND_RECORDER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_COMMAND_RECORDER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_COMMAND_RECORDER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_COMMAND_RECORDER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_COMMAND_SIGNATURE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_COMMAND_SIGNATURE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ByteStride == other.ByteStride && self.NumArgumentDescs == other.NumArgumentDescs && self.pArgumentDescs == other.pArgumentDescs && self.NodeMask == other.NodeMask
    }
}
impl ::core::cmp::Eq for D3D12_COMMAND_SIGNATURE_DESC {}
impl ::core::fmt::Debug for D3D12_COMMAND_SIGNATURE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_COMMAND_SIGNATURE_DESC").field("ByteStride", &self.ByteStride).field("NumArgumentDescs", &self.NumArgumentDescs).field("pArgumentDescs", &self.pArgumentDescs).field("NodeMask", &self.NodeMask).finish()
    }
}
impl ::core::default::Default for D3D12_COMPARISON_FUNC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_COMPARISON_FUNC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_COMPARISON_FUNC").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_COMPUTE_PIPELINE_STATE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_COMPUTE_PIPELINE_STATE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.pRootSignature == other.pRootSignature && self.CS == other.CS && self.NodeMask == other.NodeMask && self.CachedPSO == other.CachedPSO && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D12_COMPUTE_PIPELINE_STATE_DESC {}
impl ::core::fmt::Debug for D3D12_COMPUTE_PIPELINE_STATE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_COMPUTE_PIPELINE_STATE_DESC").field("pRootSignature", &self.pRootSignature).field("CS", &self.CS).field("NodeMask", &self.NodeMask).field("CachedPSO", &self.CachedPSO).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D12_CONSERVATIVE_RASTERIZATION_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_CONSERVATIVE_RASTERIZATION_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_CONSERVATIVE_RASTERIZATION_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_CONSERVATIVE_RASTERIZATION_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_CONSERVATIVE_RASTERIZATION_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_CONSERVATIVE_RASTERIZATION_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_CONSTANT_BUFFER_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_CONSTANT_BUFFER_VIEW_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.BufferLocation == other.BufferLocation && self.SizeInBytes == other.SizeInBytes
    }
}
impl ::core::cmp::Eq for D3D12_CONSTANT_BUFFER_VIEW_DESC {}
impl ::core::fmt::Debug for D3D12_CONSTANT_BUFFER_VIEW_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_CONSTANT_BUFFER_VIEW_DESC").field("BufferLocation", &self.BufferLocation).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
impl ::core::default::Default for D3D12_CPU_DESCRIPTOR_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_CPU_DESCRIPTOR_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}
impl ::core::cmp::Eq for D3D12_CPU_DESCRIPTOR_HANDLE {}
impl ::core::fmt::Debug for D3D12_CPU_DESCRIPTOR_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_CPU_DESCRIPTOR_HANDLE").field("ptr", &self.ptr).finish()
    }
}
impl ::core::default::Default for D3D12_CPU_PAGE_PROPERTY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_CPU_PAGE_PROPERTY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_CPU_PAGE_PROPERTY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_CROSS_NODE_SHARING_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_CROSS_NODE_SHARING_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_CROSS_NODE_SHARING_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_CULL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_CULL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_CULL_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.ShaderPatchMode == other.ShaderPatchMode
    }
}
impl ::core::cmp::Eq for D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS {}
impl ::core::fmt::Debug for D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DEBUG_COMMAND_LIST_GPU_BASED_VALIDATION_SETTINGS").field("ShaderPatchMode", &self.ShaderPatchMode).finish()
    }
}
impl ::core::default::Default for D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DEBUG_COMMAND_LIST_PARAMETER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS {
    fn eq(&self, other: &Self) -> bool {
        self.MaxMessagesPerCommandList == other.MaxMessagesPerCommandList && self.DefaultShaderPatchMode == other.DefaultShaderPatchMode && self.PipelineStateCreateFlags == other.PipelineStateCreateFlags
    }
}
impl ::core::cmp::Eq for D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS {}
impl ::core::fmt::Debug for D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DEBUG_DEVICE_GPU_BASED_VALIDATION_SETTINGS").field("MaxMessagesPerCommandList", &self.MaxMessagesPerCommandList).field("DefaultShaderPatchMode", &self.DefaultShaderPatchMode).field("PipelineStateCreateFlags", &self.PipelineStateCreateFlags).finish()
    }
}
impl ::core::default::Default for D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR {
    fn eq(&self, other: &Self) -> bool {
        self.SlowdownFactor == other.SlowdownFactor
    }
}
impl ::core::cmp::Eq for D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR {}
impl ::core::fmt::Debug for D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DEBUG_DEVICE_GPU_SLOWDOWN_PERFORMANCE_FACTOR").field("SlowdownFactor", &self.SlowdownFactor).finish()
    }
}
impl ::core::default::Default for D3D12_DEBUG_DEVICE_PARAMETER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DEBUG_DEVICE_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DEBUG_DEVICE_PARAMETER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_DEBUG_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DEBUG_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DEBUG_FEATURE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_DEPTH_STENCILOP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DEPTH_STENCILOP_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.StencilFailOp == other.StencilFailOp && self.StencilDepthFailOp == other.StencilDepthFailOp && self.StencilPassOp == other.StencilPassOp && self.StencilFunc == other.StencilFunc
    }
}
impl ::core::cmp::Eq for D3D12_DEPTH_STENCILOP_DESC {}
impl ::core::fmt::Debug for D3D12_DEPTH_STENCILOP_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DEPTH_STENCILOP_DESC").field("StencilFailOp", &self.StencilFailOp).field("StencilDepthFailOp", &self.StencilDepthFailOp).field("StencilPassOp", &self.StencilPassOp).field("StencilFunc", &self.StencilFunc).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_DEPTH_STENCIL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_DEPTH_STENCIL_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DepthEnable == other.DepthEnable && self.DepthWriteMask == other.DepthWriteMask && self.DepthFunc == other.DepthFunc && self.StencilEnable == other.StencilEnable && self.StencilReadMask == other.StencilReadMask && self.StencilWriteMask == other.StencilWriteMask && self.FrontFace == other.FrontFace && self.BackFace == other.BackFace
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_DEPTH_STENCIL_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_DEPTH_STENCIL_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DEPTH_STENCIL_DESC").field("DepthEnable", &self.DepthEnable).field("DepthWriteMask", &self.DepthWriteMask).field("DepthFunc", &self.DepthFunc).field("StencilEnable", &self.StencilEnable).field("StencilReadMask", &self.StencilReadMask).field("StencilWriteMask", &self.StencilWriteMask).field("FrontFace", &self.FrontFace).field("BackFace", &self.BackFace).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_DEPTH_STENCIL_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_DEPTH_STENCIL_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.DepthEnable == other.DepthEnable && self.DepthWriteMask == other.DepthWriteMask && self.DepthFunc == other.DepthFunc && self.StencilEnable == other.StencilEnable && self.StencilReadMask == other.StencilReadMask && self.StencilWriteMask == other.StencilWriteMask && self.FrontFace == other.FrontFace && self.BackFace == other.BackFace && self.DepthBoundsTestEnable == other.DepthBoundsTestEnable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_DEPTH_STENCIL_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_DEPTH_STENCIL_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DEPTH_STENCIL_DESC1").field("DepthEnable", &self.DepthEnable).field("DepthWriteMask", &self.DepthWriteMask).field("DepthFunc", &self.DepthFunc).field("StencilEnable", &self.StencilEnable).field("StencilReadMask", &self.StencilReadMask).field("StencilWriteMask", &self.StencilWriteMask).field("FrontFace", &self.FrontFace).field("BackFace", &self.BackFace).field("DepthBoundsTestEnable", &self.DepthBoundsTestEnable).finish()
    }
}
impl ::core::default::Default for D3D12_DEPTH_STENCIL_VALUE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DEPTH_STENCIL_VALUE {
    fn eq(&self, other: &Self) -> bool {
        self.Depth == other.Depth && self.Stencil == other.Stencil
    }
}
impl ::core::cmp::Eq for D3D12_DEPTH_STENCIL_VALUE {}
impl ::core::fmt::Debug for D3D12_DEPTH_STENCIL_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DEPTH_STENCIL_VALUE").field("Depth", &self.Depth).field("Stencil", &self.Stencil).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_DEPTH_WRITE_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DEPTH_WRITE_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DEPTH_WRITE_MASK").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_DESCRIPTOR_HEAP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DESCRIPTOR_HEAP_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.NumDescriptors == other.NumDescriptors && self.Flags == other.Flags && self.NodeMask == other.NodeMask
    }
}
impl ::core::cmp::Eq for D3D12_DESCRIPTOR_HEAP_DESC {}
impl ::core::fmt::Debug for D3D12_DESCRIPTOR_HEAP_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DESCRIPTOR_HEAP_DESC").field("Type", &self.Type).field("NumDescriptors", &self.NumDescriptors).field("Flags", &self.Flags).field("NodeMask", &self.NodeMask).finish()
    }
}
impl ::core::default::Default for D3D12_DESCRIPTOR_HEAP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DESCRIPTOR_HEAP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DESCRIPTOR_HEAP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_DESCRIPTOR_HEAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_DESCRIPTOR_HEAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_DESCRIPTOR_HEAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_DESCRIPTOR_HEAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_DESCRIPTOR_HEAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_DESCRIPTOR_HEAP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DESCRIPTOR_HEAP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DESCRIPTOR_HEAP_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_DESCRIPTOR_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DESCRIPTOR_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.RangeType == other.RangeType && self.NumDescriptors == other.NumDescriptors && self.BaseShaderRegister == other.BaseShaderRegister && self.RegisterSpace == other.RegisterSpace && self.OffsetInDescriptorsFromTableStart == other.OffsetInDescriptorsFromTableStart
    }
}
impl ::core::cmp::Eq for D3D12_DESCRIPTOR_RANGE {}
impl ::core::fmt::Debug for D3D12_DESCRIPTOR_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DESCRIPTOR_RANGE").field("RangeType", &self.RangeType).field("NumDescriptors", &self.NumDescriptors).field("BaseShaderRegister", &self.BaseShaderRegister).field("RegisterSpace", &self.RegisterSpace).field("OffsetInDescriptorsFromTableStart", &self.OffsetInDescriptorsFromTableStart).finish()
    }
}
impl ::core::default::Default for D3D12_DESCRIPTOR_RANGE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DESCRIPTOR_RANGE1 {
    fn eq(&self, other: &Self) -> bool {
        self.RangeType == other.RangeType && self.NumDescriptors == other.NumDescriptors && self.BaseShaderRegister == other.BaseShaderRegister && self.RegisterSpace == other.RegisterSpace && self.Flags == other.Flags && self.OffsetInDescriptorsFromTableStart == other.OffsetInDescriptorsFromTableStart
    }
}
impl ::core::cmp::Eq for D3D12_DESCRIPTOR_RANGE1 {}
impl ::core::fmt::Debug for D3D12_DESCRIPTOR_RANGE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DESCRIPTOR_RANGE1").field("RangeType", &self.RangeType).field("NumDescriptors", &self.NumDescriptors).field("BaseShaderRegister", &self.BaseShaderRegister).field("RegisterSpace", &self.RegisterSpace).field("Flags", &self.Flags).field("OffsetInDescriptorsFromTableStart", &self.OffsetInDescriptorsFromTableStart).finish()
    }
}
impl ::core::default::Default for D3D12_DESCRIPTOR_RANGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DESCRIPTOR_RANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DESCRIPTOR_RANGE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_DESCRIPTOR_RANGE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_DESCRIPTOR_RANGE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_DESCRIPTOR_RANGE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_DESCRIPTOR_RANGE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_DESCRIPTOR_RANGE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_DESCRIPTOR_RANGE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DESCRIPTOR_RANGE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DESCRIPTOR_RANGE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_DEVICE_REMOVED_EXTENDED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DEVICE_REMOVED_EXTENDED_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags && self.pHeadAutoBreadcrumbNode == other.pHeadAutoBreadcrumbNode
    }
}
impl ::core::cmp::Eq for D3D12_DEVICE_REMOVED_EXTENDED_DATA {}
impl ::core::fmt::Debug for D3D12_DEVICE_REMOVED_EXTENDED_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DEVICE_REMOVED_EXTENDED_DATA").field("Flags", &self.Flags).field("pHeadAutoBreadcrumbNode", &self.pHeadAutoBreadcrumbNode).finish()
    }
}
impl ::core::default::Default for D3D12_DEVICE_REMOVED_EXTENDED_DATA1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DEVICE_REMOVED_EXTENDED_DATA1 {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceRemovedReason == other.DeviceRemovedReason && self.AutoBreadcrumbsOutput == other.AutoBreadcrumbsOutput && self.PageFaultOutput == other.PageFaultOutput
    }
}
impl ::core::cmp::Eq for D3D12_DEVICE_REMOVED_EXTENDED_DATA1 {}
impl ::core::fmt::Debug for D3D12_DEVICE_REMOVED_EXTENDED_DATA1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DEVICE_REMOVED_EXTENDED_DATA1").field("DeviceRemovedReason", &self.DeviceRemovedReason).field("AutoBreadcrumbsOutput", &self.AutoBreadcrumbsOutput).field("PageFaultOutput", &self.PageFaultOutput).finish()
    }
}
impl ::core::default::Default for D3D12_DEVICE_REMOVED_EXTENDED_DATA2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DEVICE_REMOVED_EXTENDED_DATA2 {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceRemovedReason == other.DeviceRemovedReason && self.AutoBreadcrumbsOutput == other.AutoBreadcrumbsOutput && self.PageFaultOutput == other.PageFaultOutput
    }
}
impl ::core::cmp::Eq for D3D12_DEVICE_REMOVED_EXTENDED_DATA2 {}
impl ::core::fmt::Debug for D3D12_DEVICE_REMOVED_EXTENDED_DATA2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DEVICE_REMOVED_EXTENDED_DATA2").field("DeviceRemovedReason", &self.DeviceRemovedReason).field("AutoBreadcrumbsOutput", &self.AutoBreadcrumbsOutput).field("PageFaultOutput", &self.PageFaultOutput).finish()
    }
}
impl ::core::default::Default for D3D12_DEVICE_REMOVED_EXTENDED_DATA3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DEVICE_REMOVED_EXTENDED_DATA3 {
    fn eq(&self, other: &Self) -> bool {
        self.DeviceRemovedReason == other.DeviceRemovedReason && self.AutoBreadcrumbsOutput == other.AutoBreadcrumbsOutput && self.PageFaultOutput == other.PageFaultOutput && self.DeviceState == other.DeviceState
    }
}
impl ::core::cmp::Eq for D3D12_DEVICE_REMOVED_EXTENDED_DATA3 {}
impl ::core::fmt::Debug for D3D12_DEVICE_REMOVED_EXTENDED_DATA3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DEVICE_REMOVED_EXTENDED_DATA3").field("DeviceRemovedReason", &self.DeviceRemovedReason).field("AutoBreadcrumbsOutput", &self.AutoBreadcrumbsOutput).field("PageFaultOutput", &self.PageFaultOutput).field("DeviceState", &self.DeviceState).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_DISCARD_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_DISCARD_REGION {
    fn eq(&self, other: &Self) -> bool {
        self.NumRects == other.NumRects && self.pRects == other.pRects && self.FirstSubresource == other.FirstSubresource && self.NumSubresources == other.NumSubresources
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_DISCARD_REGION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_DISCARD_REGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DISCARD_REGION").field("NumRects", &self.NumRects).field("pRects", &self.pRects).field("FirstSubresource", &self.FirstSubresource).field("NumSubresources", &self.NumSubresources).finish()
    }
}
impl ::core::default::Default for D3D12_DISPATCH_ARGUMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DISPATCH_ARGUMENTS {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadGroupCountX == other.ThreadGroupCountX && self.ThreadGroupCountY == other.ThreadGroupCountY && self.ThreadGroupCountZ == other.ThreadGroupCountZ
    }
}
impl ::core::cmp::Eq for D3D12_DISPATCH_ARGUMENTS {}
impl ::core::fmt::Debug for D3D12_DISPATCH_ARGUMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DISPATCH_ARGUMENTS").field("ThreadGroupCountX", &self.ThreadGroupCountX).field("ThreadGroupCountY", &self.ThreadGroupCountY).field("ThreadGroupCountZ", &self.ThreadGroupCountZ).finish()
    }
}
impl ::core::default::Default for D3D12_DISPATCH_MESH_ARGUMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DISPATCH_MESH_ARGUMENTS {
    fn eq(&self, other: &Self) -> bool {
        self.ThreadGroupCountX == other.ThreadGroupCountX && self.ThreadGroupCountY == other.ThreadGroupCountY && self.ThreadGroupCountZ == other.ThreadGroupCountZ
    }
}
impl ::core::cmp::Eq for D3D12_DISPATCH_MESH_ARGUMENTS {}
impl ::core::fmt::Debug for D3D12_DISPATCH_MESH_ARGUMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DISPATCH_MESH_ARGUMENTS").field("ThreadGroupCountX", &self.ThreadGroupCountX).field("ThreadGroupCountY", &self.ThreadGroupCountY).field("ThreadGroupCountZ", &self.ThreadGroupCountZ).finish()
    }
}
impl ::core::default::Default for D3D12_DISPATCH_RAYS_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DISPATCH_RAYS_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.RayGenerationShaderRecord == other.RayGenerationShaderRecord && self.MissShaderTable == other.MissShaderTable && self.HitGroupTable == other.HitGroupTable && self.CallableShaderTable == other.CallableShaderTable && self.Width == other.Width && self.Height == other.Height && self.Depth == other.Depth
    }
}
impl ::core::cmp::Eq for D3D12_DISPATCH_RAYS_DESC {}
impl ::core::fmt::Debug for D3D12_DISPATCH_RAYS_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DISPATCH_RAYS_DESC").field("RayGenerationShaderRecord", &self.RayGenerationShaderRecord).field("MissShaderTable", &self.MissShaderTable).field("HitGroupTable", &self.HitGroupTable).field("CallableShaderTable", &self.CallableShaderTable).field("Width", &self.Width).field("Height", &self.Height).field("Depth", &self.Depth).finish()
    }
}
impl ::core::default::Default for D3D12_DRAW_ARGUMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DRAW_ARGUMENTS {
    fn eq(&self, other: &Self) -> bool {
        self.VertexCountPerInstance == other.VertexCountPerInstance && self.InstanceCount == other.InstanceCount && self.StartVertexLocation == other.StartVertexLocation && self.StartInstanceLocation == other.StartInstanceLocation
    }
}
impl ::core::cmp::Eq for D3D12_DRAW_ARGUMENTS {}
impl ::core::fmt::Debug for D3D12_DRAW_ARGUMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DRAW_ARGUMENTS").field("VertexCountPerInstance", &self.VertexCountPerInstance).field("InstanceCount", &self.InstanceCount).field("StartVertexLocation", &self.StartVertexLocation).field("StartInstanceLocation", &self.StartInstanceLocation).finish()
    }
}
impl ::core::default::Default for D3D12_DRAW_INDEXED_ARGUMENTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DRAW_INDEXED_ARGUMENTS {
    fn eq(&self, other: &Self) -> bool {
        self.IndexCountPerInstance == other.IndexCountPerInstance && self.InstanceCount == other.InstanceCount && self.StartIndexLocation == other.StartIndexLocation && self.BaseVertexLocation == other.BaseVertexLocation && self.StartInstanceLocation == other.StartInstanceLocation
    }
}
impl ::core::cmp::Eq for D3D12_DRAW_INDEXED_ARGUMENTS {}
impl ::core::fmt::Debug for D3D12_DRAW_INDEXED_ARGUMENTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DRAW_INDEXED_ARGUMENTS").field("IndexCountPerInstance", &self.IndexCountPerInstance).field("InstanceCount", &self.InstanceCount).field("StartIndexLocation", &self.StartIndexLocation).field("BaseVertexLocation", &self.BaseVertexLocation).field("StartInstanceLocation", &self.StartInstanceLocation).finish()
    }
}
impl ::core::default::Default for D3D12_DRED_ALLOCATION_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DRED_ALLOCATION_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectNameA == other.ObjectNameA && self.ObjectNameW == other.ObjectNameW && self.AllocationType == other.AllocationType && self.pNext == other.pNext
    }
}
impl ::core::cmp::Eq for D3D12_DRED_ALLOCATION_NODE {}
impl ::core::fmt::Debug for D3D12_DRED_ALLOCATION_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DRED_ALLOCATION_NODE").field("ObjectNameA", &self.ObjectNameA).field("ObjectNameW", &self.ObjectNameW).field("AllocationType", &self.AllocationType).field("pNext", &self.pNext).finish()
    }
}
impl ::core::default::Default for D3D12_DRED_ALLOCATION_NODE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DRED_ALLOCATION_NODE1 {
    fn eq(&self, other: &Self) -> bool {
        self.ObjectNameA == other.ObjectNameA && self.ObjectNameW == other.ObjectNameW && self.AllocationType == other.AllocationType && self.pNext == other.pNext && self.pObject == other.pObject
    }
}
impl ::core::cmp::Eq for D3D12_DRED_ALLOCATION_NODE1 {}
impl ::core::fmt::Debug for D3D12_DRED_ALLOCATION_NODE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DRED_ALLOCATION_NODE1").field("ObjectNameA", &self.ObjectNameA).field("ObjectNameW", &self.ObjectNameW).field("AllocationType", &self.AllocationType).field("pNext", &self.pNext).field("pObject", &self.pObject).finish()
    }
}
impl ::core::default::Default for D3D12_DRED_ALLOCATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DRED_ALLOCATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DRED_ALLOCATION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.pHeadAutoBreadcrumbNode == other.pHeadAutoBreadcrumbNode
    }
}
impl ::core::cmp::Eq for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT {}
impl ::core::fmt::Debug for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT").field("pHeadAutoBreadcrumbNode", &self.pHeadAutoBreadcrumbNode).finish()
    }
}
impl ::core::default::Default for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1 {
    fn eq(&self, other: &Self) -> bool {
        self.pHeadAutoBreadcrumbNode == other.pHeadAutoBreadcrumbNode
    }
}
impl ::core::cmp::Eq for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1 {}
impl ::core::fmt::Debug for D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1").field("pHeadAutoBreadcrumbNode", &self.pHeadAutoBreadcrumbNode).finish()
    }
}
impl ::core::default::Default for D3D12_DRED_BREADCRUMB_CONTEXT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DRED_BREADCRUMB_CONTEXT {
    fn eq(&self, other: &Self) -> bool {
        self.BreadcrumbIndex == other.BreadcrumbIndex && self.pContextString == other.pContextString
    }
}
impl ::core::cmp::Eq for D3D12_DRED_BREADCRUMB_CONTEXT {}
impl ::core::fmt::Debug for D3D12_DRED_BREADCRUMB_CONTEXT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DRED_BREADCRUMB_CONTEXT").field("BreadcrumbIndex", &self.BreadcrumbIndex).field("pContextString", &self.pContextString).finish()
    }
}
impl ::core::default::Default for D3D12_DRED_DEVICE_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DRED_DEVICE_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DRED_DEVICE_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_DRED_ENABLEMENT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DRED_ENABLEMENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DRED_ENABLEMENT").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_DRED_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DRED_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DRED_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_DRED_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_DRED_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_DRED_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_DRED_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_DRED_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_DRED_PAGE_FAULT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DRED_PAGE_FAULT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DRED_PAGE_FAULT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_DRED_PAGE_FAULT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_DRED_PAGE_FAULT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_DRED_PAGE_FAULT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_DRED_PAGE_FAULT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_DRED_PAGE_FAULT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_DRED_PAGE_FAULT_OUTPUT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DRED_PAGE_FAULT_OUTPUT {
    fn eq(&self, other: &Self) -> bool {
        self.PageFaultVA == other.PageFaultVA && self.pHeadExistingAllocationNode == other.pHeadExistingAllocationNode && self.pHeadRecentFreedAllocationNode == other.pHeadRecentFreedAllocationNode
    }
}
impl ::core::cmp::Eq for D3D12_DRED_PAGE_FAULT_OUTPUT {}
impl ::core::fmt::Debug for D3D12_DRED_PAGE_FAULT_OUTPUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DRED_PAGE_FAULT_OUTPUT").field("PageFaultVA", &self.PageFaultVA).field("pHeadExistingAllocationNode", &self.pHeadExistingAllocationNode).field("pHeadRecentFreedAllocationNode", &self.pHeadRecentFreedAllocationNode).finish()
    }
}
impl ::core::default::Default for D3D12_DRED_PAGE_FAULT_OUTPUT1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DRED_PAGE_FAULT_OUTPUT1 {
    fn eq(&self, other: &Self) -> bool {
        self.PageFaultVA == other.PageFaultVA && self.pHeadExistingAllocationNode == other.pHeadExistingAllocationNode && self.pHeadRecentFreedAllocationNode == other.pHeadRecentFreedAllocationNode
    }
}
impl ::core::cmp::Eq for D3D12_DRED_PAGE_FAULT_OUTPUT1 {}
impl ::core::fmt::Debug for D3D12_DRED_PAGE_FAULT_OUTPUT1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DRED_PAGE_FAULT_OUTPUT1").field("PageFaultVA", &self.PageFaultVA).field("pHeadExistingAllocationNode", &self.pHeadExistingAllocationNode).field("pHeadRecentFreedAllocationNode", &self.pHeadRecentFreedAllocationNode).finish()
    }
}
impl ::core::default::Default for D3D12_DRED_PAGE_FAULT_OUTPUT2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DRED_PAGE_FAULT_OUTPUT2 {
    fn eq(&self, other: &Self) -> bool {
        self.PageFaultVA == other.PageFaultVA && self.pHeadExistingAllocationNode == other.pHeadExistingAllocationNode && self.pHeadRecentFreedAllocationNode == other.pHeadRecentFreedAllocationNode && self.PageFaultFlags == other.PageFaultFlags
    }
}
impl ::core::cmp::Eq for D3D12_DRED_PAGE_FAULT_OUTPUT2 {}
impl ::core::fmt::Debug for D3D12_DRED_PAGE_FAULT_OUTPUT2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DRED_PAGE_FAULT_OUTPUT2").field("PageFaultVA", &self.PageFaultVA).field("pHeadExistingAllocationNode", &self.pHeadExistingAllocationNode).field("pHeadRecentFreedAllocationNode", &self.pHeadRecentFreedAllocationNode).field("PageFaultFlags", &self.PageFaultFlags).finish()
    }
}
impl ::core::default::Default for D3D12_DRED_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DRED_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DRED_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_DSV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DSV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DSV_DIMENSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_DSV_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_DSV_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_DSV_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_DSV_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_DSV_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_DSV_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_DSV_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_DSV_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_DXIL_LIBRARY_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DXIL_LIBRARY_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DXILLibrary == other.DXILLibrary && self.NumExports == other.NumExports && self.pExports == other.pExports
    }
}
impl ::core::cmp::Eq for D3D12_DXIL_LIBRARY_DESC {}
impl ::core::fmt::Debug for D3D12_DXIL_LIBRARY_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DXIL_LIBRARY_DESC").field("DXILLibrary", &self.DXILLibrary).field("NumExports", &self.NumExports).field("pExports", &self.pExports).finish()
    }
}
impl ::core::default::Default for D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    fn eq(&self, other: &Self) -> bool {
        self.SubobjectToAssociate == other.SubobjectToAssociate && self.NumExports == other.NumExports && self.pExports == other.pExports
    }
}
impl ::core::cmp::Eq for D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {}
impl ::core::fmt::Debug for D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION").field("SubobjectToAssociate", &self.SubobjectToAssociate).field("NumExports", &self.NumExports).field("pExports", &self.pExports).finish()
    }
}
impl ::core::default::Default for D3D12_ELEMENTS_LAYOUT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_ELEMENTS_LAYOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_ELEMENTS_LAYOUT").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_EXISTING_COLLECTION_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_EXISTING_COLLECTION_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.pExistingCollection == other.pExistingCollection && self.NumExports == other.NumExports && self.pExports == other.pExports
    }
}
impl ::core::cmp::Eq for D3D12_EXISTING_COLLECTION_DESC {}
impl ::core::fmt::Debug for D3D12_EXISTING_COLLECTION_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_EXISTING_COLLECTION_DESC").field("pExistingCollection", &self.pExistingCollection).field("NumExports", &self.NumExports).field("pExports", &self.pExports).finish()
    }
}
impl ::core::default::Default for D3D12_EXPORT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_EXPORT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.ExportToRename == other.ExportToRename && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D12_EXPORT_DESC {}
impl ::core::fmt::Debug for D3D12_EXPORT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_EXPORT_DESC").field("Name", &self.Name).field("ExportToRename", &self.ExportToRename).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D12_EXPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_EXPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_EXPORT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_EXPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_EXPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_EXPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_EXPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_EXPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_FEATURE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_FEATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_FEATURE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_ARCHITECTURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_ARCHITECTURE {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.TileBasedRenderer == other.TileBasedRenderer && self.UMA == other.UMA && self.CacheCoherentUMA == other.CacheCoherentUMA
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_ARCHITECTURE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_ARCHITECTURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_ARCHITECTURE").field("NodeIndex", &self.NodeIndex).field("TileBasedRenderer", &self.TileBasedRenderer).field("UMA", &self.UMA).field("CacheCoherentUMA", &self.CacheCoherentUMA).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_ARCHITECTURE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_ARCHITECTURE1 {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.TileBasedRenderer == other.TileBasedRenderer && self.UMA == other.UMA && self.CacheCoherentUMA == other.CacheCoherentUMA && self.IsolatedMMU == other.IsolatedMMU
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_ARCHITECTURE1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_ARCHITECTURE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_ARCHITECTURE1").field("NodeIndex", &self.NodeIndex).field("TileBasedRenderer", &self.TileBasedRenderer).field("UMA", &self.UMA).field("CacheCoherentUMA", &self.CacheCoherentUMA).field("IsolatedMMU", &self.IsolatedMMU).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
    fn eq(&self, other: &Self) -> bool {
        self.CommandListType == other.CommandListType && self.Priority == other.Priority && self.PriorityForTypeIsSupported == other.PriorityForTypeIsSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY").field("CommandListType", &self.CommandListType).field("Priority", &self.Priority).field("PriorityForTypeIsSupported", &self.PriorityForTypeIsSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_CROSS_NODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_CROSS_NODE {
    fn eq(&self, other: &Self) -> bool {
        self.SharingTier == other.SharingTier && self.AtomicShaderInstructions == other.AtomicShaderInstructions
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_CROSS_NODE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_CROSS_NODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_CROSS_NODE").field("SharingTier", &self.SharingTier).field("AtomicShaderInstructions", &self.AtomicShaderInstructions).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_D3D12_OPTIONS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_D3D12_OPTIONS {
    fn eq(&self, other: &Self) -> bool {
        self.DoublePrecisionFloatShaderOps == other.DoublePrecisionFloatShaderOps
            && self.OutputMergerLogicOp == other.OutputMergerLogicOp
            && self.MinPrecisionSupport == other.MinPrecisionSupport
            && self.TiledResourcesTier == other.TiledResourcesTier
            && self.ResourceBindingTier == other.ResourceBindingTier
            && self.PSSpecifiedStencilRefSupported == other.PSSpecifiedStencilRefSupported
            && self.TypedUAVLoadAdditionalFormats == other.TypedUAVLoadAdditionalFormats
            && self.ROVsSupported == other.ROVsSupported
            && self.ConservativeRasterizationTier == other.ConservativeRasterizationTier
            && self.MaxGPUVirtualAddressBitsPerResource == other.MaxGPUVirtualAddressBitsPerResource
            && self.StandardSwizzle64KBSupported == other.StandardSwizzle64KBSupported
            && self.CrossNodeSharingTier == other.CrossNodeSharingTier
            && self.CrossAdapterRowMajorTextureSupported == other.CrossAdapterRowMajorTextureSupported
            && self.VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation == other.VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation
            && self.ResourceHeapTier == other.ResourceHeapTier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_D3D12_OPTIONS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_D3D12_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_D3D12_OPTIONS")
            .field("DoublePrecisionFloatShaderOps", &self.DoublePrecisionFloatShaderOps)
            .field("OutputMergerLogicOp", &self.OutputMergerLogicOp)
            .field("MinPrecisionSupport", &self.MinPrecisionSupport)
            .field("TiledResourcesTier", &self.TiledResourcesTier)
            .field("ResourceBindingTier", &self.ResourceBindingTier)
            .field("PSSpecifiedStencilRefSupported", &self.PSSpecifiedStencilRefSupported)
            .field("TypedUAVLoadAdditionalFormats", &self.TypedUAVLoadAdditionalFormats)
            .field("ROVsSupported", &self.ROVsSupported)
            .field("ConservativeRasterizationTier", &self.ConservativeRasterizationTier)
            .field("MaxGPUVirtualAddressBitsPerResource", &self.MaxGPUVirtualAddressBitsPerResource)
            .field("StandardSwizzle64KBSupported", &self.StandardSwizzle64KBSupported)
            .field("CrossNodeSharingTier", &self.CrossNodeSharingTier)
            .field("CrossAdapterRowMajorTextureSupported", &self.CrossAdapterRowMajorTextureSupported)
            .field("VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation", &self.VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation)
            .field("ResourceHeapTier", &self.ResourceHeapTier)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_D3D12_OPTIONS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_D3D12_OPTIONS1 {
    fn eq(&self, other: &Self) -> bool {
        self.WaveOps == other.WaveOps && self.WaveLaneCountMin == other.WaveLaneCountMin && self.WaveLaneCountMax == other.WaveLaneCountMax && self.TotalLaneCount == other.TotalLaneCount && self.ExpandedComputeResourceStates == other.ExpandedComputeResourceStates && self.Int64ShaderOps == other.Int64ShaderOps
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_D3D12_OPTIONS1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_D3D12_OPTIONS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_D3D12_OPTIONS1").field("WaveOps", &self.WaveOps).field("WaveLaneCountMin", &self.WaveLaneCountMin).field("WaveLaneCountMax", &self.WaveLaneCountMax).field("TotalLaneCount", &self.TotalLaneCount).field("ExpandedComputeResourceStates", &self.ExpandedComputeResourceStates).field("Int64ShaderOps", &self.Int64ShaderOps).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_D3D12_OPTIONS10 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_D3D12_OPTIONS10 {
    fn eq(&self, other: &Self) -> bool {
        self.VariableRateShadingSumCombinerSupported == other.VariableRateShadingSumCombinerSupported && self.MeshShaderPerPrimitiveShadingRateSupported == other.MeshShaderPerPrimitiveShadingRateSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_D3D12_OPTIONS10 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_D3D12_OPTIONS10 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_D3D12_OPTIONS10").field("VariableRateShadingSumCombinerSupported", &self.VariableRateShadingSumCombinerSupported).field("MeshShaderPerPrimitiveShadingRateSupported", &self.MeshShaderPerPrimitiveShadingRateSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_D3D12_OPTIONS11 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_D3D12_OPTIONS11 {
    fn eq(&self, other: &Self) -> bool {
        self.AtomicInt64OnDescriptorHeapResourceSupported == other.AtomicInt64OnDescriptorHeapResourceSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_D3D12_OPTIONS11 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_D3D12_OPTIONS11 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_D3D12_OPTIONS11").field("AtomicInt64OnDescriptorHeapResourceSupported", &self.AtomicInt64OnDescriptorHeapResourceSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_D3D12_OPTIONS2 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_D3D12_OPTIONS2 {
    fn eq(&self, other: &Self) -> bool {
        self.DepthBoundsTestSupported == other.DepthBoundsTestSupported && self.ProgrammableSamplePositionsTier == other.ProgrammableSamplePositionsTier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_D3D12_OPTIONS2 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_D3D12_OPTIONS2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_D3D12_OPTIONS2").field("DepthBoundsTestSupported", &self.DepthBoundsTestSupported).field("ProgrammableSamplePositionsTier", &self.ProgrammableSamplePositionsTier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_D3D12_OPTIONS3 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_D3D12_OPTIONS3 {
    fn eq(&self, other: &Self) -> bool {
        self.CopyQueueTimestampQueriesSupported == other.CopyQueueTimestampQueriesSupported && self.CastingFullyTypedFormatSupported == other.CastingFullyTypedFormatSupported && self.WriteBufferImmediateSupportFlags == other.WriteBufferImmediateSupportFlags && self.ViewInstancingTier == other.ViewInstancingTier && self.BarycentricsSupported == other.BarycentricsSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_D3D12_OPTIONS3 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_D3D12_OPTIONS3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_D3D12_OPTIONS3").field("CopyQueueTimestampQueriesSupported", &self.CopyQueueTimestampQueriesSupported).field("CastingFullyTypedFormatSupported", &self.CastingFullyTypedFormatSupported).field("WriteBufferImmediateSupportFlags", &self.WriteBufferImmediateSupportFlags).field("ViewInstancingTier", &self.ViewInstancingTier).field("BarycentricsSupported", &self.BarycentricsSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_D3D12_OPTIONS4 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_D3D12_OPTIONS4 {
    fn eq(&self, other: &Self) -> bool {
        self.MSAA64KBAlignedTextureSupported == other.MSAA64KBAlignedTextureSupported && self.SharedResourceCompatibilityTier == other.SharedResourceCompatibilityTier && self.Native16BitShaderOpsSupported == other.Native16BitShaderOpsSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_D3D12_OPTIONS4 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_D3D12_OPTIONS4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_D3D12_OPTIONS4").field("MSAA64KBAlignedTextureSupported", &self.MSAA64KBAlignedTextureSupported).field("SharedResourceCompatibilityTier", &self.SharedResourceCompatibilityTier).field("Native16BitShaderOpsSupported", &self.Native16BitShaderOpsSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_D3D12_OPTIONS5 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_D3D12_OPTIONS5 {
    fn eq(&self, other: &Self) -> bool {
        self.SRVOnlyTiledResourceTier3 == other.SRVOnlyTiledResourceTier3 && self.RenderPassesTier == other.RenderPassesTier && self.RaytracingTier == other.RaytracingTier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_D3D12_OPTIONS5 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_D3D12_OPTIONS5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_D3D12_OPTIONS5").field("SRVOnlyTiledResourceTier3", &self.SRVOnlyTiledResourceTier3).field("RenderPassesTier", &self.RenderPassesTier).field("RaytracingTier", &self.RaytracingTier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_D3D12_OPTIONS6 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_D3D12_OPTIONS6 {
    fn eq(&self, other: &Self) -> bool {
        self.AdditionalShadingRatesSupported == other.AdditionalShadingRatesSupported && self.PerPrimitiveShadingRateSupportedWithViewportIndexing == other.PerPrimitiveShadingRateSupportedWithViewportIndexing && self.VariableShadingRateTier == other.VariableShadingRateTier && self.ShadingRateImageTileSize == other.ShadingRateImageTileSize && self.BackgroundProcessingSupported == other.BackgroundProcessingSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_D3D12_OPTIONS6 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_D3D12_OPTIONS6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_D3D12_OPTIONS6").field("AdditionalShadingRatesSupported", &self.AdditionalShadingRatesSupported).field("PerPrimitiveShadingRateSupportedWithViewportIndexing", &self.PerPrimitiveShadingRateSupportedWithViewportIndexing).field("VariableShadingRateTier", &self.VariableShadingRateTier).field("ShadingRateImageTileSize", &self.ShadingRateImageTileSize).field("BackgroundProcessingSupported", &self.BackgroundProcessingSupported).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_D3D12_OPTIONS7 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_D3D12_OPTIONS7 {
    fn eq(&self, other: &Self) -> bool {
        self.MeshShaderTier == other.MeshShaderTier && self.SamplerFeedbackTier == other.SamplerFeedbackTier
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_D3D12_OPTIONS7 {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_D3D12_OPTIONS7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_D3D12_OPTIONS7").field("MeshShaderTier", &self.MeshShaderTier).field("SamplerFeedbackTier", &self.SamplerFeedbackTier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_D3D12_OPTIONS8 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_D3D12_OPTIONS8 {
    fn eq(&self, other: &Self) -> bool {
        self.UnalignedBlockTexturesSupported == other.UnalignedBlockTexturesSupported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_D3D12_OPTIONS8 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_D3D12_OPTIONS8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_D3D12_OPTIONS8").field("UnalignedBlockTexturesSupported", &self.UnalignedBlockTexturesSupported).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_D3D12_OPTIONS9 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_D3D12_OPTIONS9 {
    fn eq(&self, other: &Self) -> bool {
        self.MeshShaderPipelineStatsSupported == other.MeshShaderPipelineStatsSupported && self.MeshShaderSupportsFullRangeRenderTargetArrayIndex == other.MeshShaderSupportsFullRangeRenderTargetArrayIndex && self.AtomicInt64OnTypedResourceSupported == other.AtomicInt64OnTypedResourceSupported && self.AtomicInt64OnGroupSharedSupported == other.AtomicInt64OnGroupSharedSupported && self.DerivativesInMeshAndAmplificationShadersSupported == other.DerivativesInMeshAndAmplificationShadersSupported && self.WaveMMATier == other.WaveMMATier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_D3D12_OPTIONS9 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_D3D12_OPTIONS9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_D3D12_OPTIONS9")
            .field("MeshShaderPipelineStatsSupported", &self.MeshShaderPipelineStatsSupported)
            .field("MeshShaderSupportsFullRangeRenderTargetArrayIndex", &self.MeshShaderSupportsFullRangeRenderTargetArrayIndex)
            .field("AtomicInt64OnTypedResourceSupported", &self.AtomicInt64OnTypedResourceSupported)
            .field("AtomicInt64OnGroupSharedSupported", &self.AtomicInt64OnGroupSharedSupported)
            .field("DerivativesInMeshAndAmplificationShadersSupported", &self.DerivativesInMeshAndAmplificationShadersSupported)
            .field("WaveMMATier", &self.WaveMMATier)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_DISPLAYABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_DISPLAYABLE {
    fn eq(&self, other: &Self) -> bool {
        self.DisplayableTexture == other.DisplayableTexture && self.SharedResourceCompatibilityTier == other.SharedResourceCompatibilityTier
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_DISPLAYABLE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_DISPLAYABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_DISPLAYABLE").field("DisplayableTexture", &self.DisplayableTexture).field("SharedResourceCompatibilityTier", &self.SharedResourceCompatibilityTier).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_FEATURE_DATA_EXISTING_HEAPS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_EXISTING_HEAPS {
    fn eq(&self, other: &Self) -> bool {
        self.Supported == other.Supported
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_EXISTING_HEAPS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_EXISTING_HEAPS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_EXISTING_HEAPS").field("Supported", &self.Supported).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D12_FEATURE_DATA_FEATURE_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_FEATURE_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        self.NumFeatureLevels == other.NumFeatureLevels && self.pFeatureLevelsRequested == other.pFeatureLevelsRequested && self.MaxSupportedFeatureLevel == other.MaxSupportedFeatureLevel
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_FEATURE_LEVELS {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_FEATURE_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_FEATURE_LEVELS").field("NumFeatureLevels", &self.NumFeatureLevels).field("pFeatureLevelsRequested", &self.pFeatureLevelsRequested).field("MaxSupportedFeatureLevel", &self.MaxSupportedFeatureLevel).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_FEATURE_DATA_FORMAT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_FORMAT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.PlaneCount == other.PlaneCount
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_FORMAT_INFO {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_FORMAT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_FORMAT_INFO").field("Format", &self.Format).field("PlaneCount", &self.PlaneCount).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_FEATURE_DATA_FORMAT_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_FORMAT_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.Support1 == other.Support1 && self.Support2 == other.Support2
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_FORMAT_SUPPORT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_FORMAT_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_FORMAT_SUPPORT").field("Format", &self.Format).field("Support1", &self.Support1).field("Support2", &self.Support2).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.MaxGPUVirtualAddressBitsPerResource == other.MaxGPUVirtualAddressBitsPerResource && self.MaxGPUVirtualAddressBitsPerProcess == other.MaxGPUVirtualAddressBitsPerProcess
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT").field("MaxGPUVirtualAddressBitsPerResource", &self.MaxGPUVirtualAddressBitsPerResource).field("MaxGPUVirtualAddressBitsPerProcess", &self.MaxGPUVirtualAddressBitsPerProcess).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.SampleCount == other.SampleCount && self.Flags == other.Flags && self.NumQualityLevels == other.NumQualityLevels
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS").field("Format", &self.Format).field("SampleCount", &self.SampleCount).field("Flags", &self.Flags).field("NumQualityLevels", &self.NumQualityLevels).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.Support == other.Support
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_SUPPORT").field("NodeIndex", &self.NodeIndex).field("Support", &self.Support).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.Count == other.Count && self.pTypes == other.pTypes
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPES").field("NodeIndex", &self.NodeIndex).field("Count", &self.Count).field("pTypes", &self.pTypes).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.Count == other.Count
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_PROTECTED_RESOURCE_SESSION_TYPE_COUNT").field("NodeIndex", &self.NodeIndex).field("Count", &self.Count).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_QUERY_META_COMMAND {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_QUERY_META_COMMAND {
    fn eq(&self, other: &Self) -> bool {
        self.CommandId == other.CommandId && self.NodeMask == other.NodeMask && self.pQueryInputData == other.pQueryInputData && self.QueryInputDataSizeInBytes == other.QueryInputDataSizeInBytes && self.pQueryOutputData == other.pQueryOutputData && self.QueryOutputDataSizeInBytes == other.QueryOutputDataSizeInBytes
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_QUERY_META_COMMAND {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_QUERY_META_COMMAND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_QUERY_META_COMMAND").field("CommandId", &self.CommandId).field("NodeMask", &self.NodeMask).field("pQueryInputData", &self.pQueryInputData).field("QueryInputDataSizeInBytes", &self.QueryInputDataSizeInBytes).field("pQueryOutputData", &self.pQueryOutputData).field("QueryOutputDataSizeInBytes", &self.QueryOutputDataSizeInBytes).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_ROOT_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_ROOT_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.HighestVersion == other.HighestVersion
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_ROOT_SIGNATURE {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_ROOT_SIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_ROOT_SIGNATURE").field("HighestVersion", &self.HighestVersion).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_SERIALIZATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_SERIALIZATION {
    fn eq(&self, other: &Self) -> bool {
        self.NodeIndex == other.NodeIndex && self.HeapSerializationTier == other.HeapSerializationTier
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_SERIALIZATION {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_SERIALIZATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_SERIALIZATION").field("NodeIndex", &self.NodeIndex).field("HeapSerializationTier", &self.HeapSerializationTier).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_SHADER_CACHE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_SHADER_CACHE {
    fn eq(&self, other: &Self) -> bool {
        self.SupportFlags == other.SupportFlags
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_SHADER_CACHE {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_SHADER_CACHE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_SHADER_CACHE").field("SupportFlags", &self.SupportFlags).finish()
    }
}
impl ::core::default::Default for D3D12_FEATURE_DATA_SHADER_MODEL {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_FEATURE_DATA_SHADER_MODEL {
    fn eq(&self, other: &Self) -> bool {
        self.HighestShaderModel == other.HighestShaderModel
    }
}
impl ::core::cmp::Eq for D3D12_FEATURE_DATA_SHADER_MODEL {}
impl ::core::fmt::Debug for D3D12_FEATURE_DATA_SHADER_MODEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FEATURE_DATA_SHADER_MODEL").field("HighestShaderModel", &self.HighestShaderModel).finish()
    }
}
impl ::core::default::Default for D3D12_FENCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_FENCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_FENCE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_FENCE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_FENCE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_FENCE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_FENCE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_FENCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_FILL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_FILL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_FILL_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_FILTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_FILTER_REDUCTION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_FILTER_REDUCTION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_FILTER_REDUCTION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_FILTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_FILTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_FILTER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_FORMAT_SUPPORT1 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_FORMAT_SUPPORT1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_FORMAT_SUPPORT1").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_FORMAT_SUPPORT1 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_FORMAT_SUPPORT1 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_FORMAT_SUPPORT1 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_FORMAT_SUPPORT1 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_FORMAT_SUPPORT1 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_FORMAT_SUPPORT2 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_FORMAT_SUPPORT2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_FORMAT_SUPPORT2").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_FORMAT_SUPPORT2 {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_FORMAT_SUPPORT2 {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_FORMAT_SUPPORT2 {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_FORMAT_SUPPORT2 {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_FORMAT_SUPPORT2 {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::default::Default for D3D12_FUNCTION_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::cmp::PartialEq for D3D12_FUNCTION_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Creator == other.Creator
            && self.Flags == other.Flags
            && self.ConstantBuffers == other.ConstantBuffers
            && self.BoundResources == other.BoundResources
            && self.InstructionCount == other.InstructionCount
            && self.TempRegisterCount == other.TempRegisterCount
            && self.TempArrayCount == other.TempArrayCount
            && self.DefCount == other.DefCount
            && self.DclCount == other.DclCount
            && self.TextureNormalInstructions == other.TextureNormalInstructions
            && self.TextureLoadInstructions == other.TextureLoadInstructions
            && self.TextureCompInstructions == other.TextureCompInstructions
            && self.TextureBiasInstructions == other.TextureBiasInstructions
            && self.TextureGradientInstructions == other.TextureGradientInstructions
            && self.FloatInstructionCount == other.FloatInstructionCount
            && self.IntInstructionCount == other.IntInstructionCount
            && self.UintInstructionCount == other.UintInstructionCount
            && self.StaticFlowControlCount == other.StaticFlowControlCount
            && self.DynamicFlowControlCount == other.DynamicFlowControlCount
            && self.MacroInstructionCount == other.MacroInstructionCount
            && self.ArrayInstructionCount == other.ArrayInstructionCount
            && self.MovInstructionCount == other.MovInstructionCount
            && self.MovcInstructionCount == other.MovcInstructionCount
            && self.ConversionInstructionCount == other.ConversionInstructionCount
            && self.BitwiseInstructionCount == other.BitwiseInstructionCount
            && self.MinFeatureLevel == other.MinFeatureLevel
            && self.RequiredFeatureFlags == other.RequiredFeatureFlags
            && self.Name == other.Name
            && self.FunctionParameterCount == other.FunctionParameterCount
            && self.HasReturn == other.HasReturn
            && self.Has10Level9VertexShader == other.Has10Level9VertexShader
            && self.Has10Level9PixelShader == other.Has10Level9PixelShader
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::cmp::Eq for D3D12_FUNCTION_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Direct3D"))]
impl ::core::fmt::Debug for D3D12_FUNCTION_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_FUNCTION_DESC")
            .field("Version", &self.Version)
            .field("Creator", &self.Creator)
            .field("Flags", &self.Flags)
            .field("ConstantBuffers", &self.ConstantBuffers)
            .field("BoundResources", &self.BoundResources)
            .field("InstructionCount", &self.InstructionCount)
            .field("TempRegisterCount", &self.TempRegisterCount)
            .field("TempArrayCount", &self.TempArrayCount)
            .field("DefCount", &self.DefCount)
            .field("DclCount", &self.DclCount)
            .field("TextureNormalInstructions", &self.TextureNormalInstructions)
            .field("TextureLoadInstructions", &self.TextureLoadInstructions)
            .field("TextureCompInstructions", &self.TextureCompInstructions)
            .field("TextureBiasInstructions", &self.TextureBiasInstructions)
            .field("TextureGradientInstructions", &self.TextureGradientInstructions)
            .field("FloatInstructionCount", &self.FloatInstructionCount)
            .field("IntInstructionCount", &self.IntInstructionCount)
            .field("UintInstructionCount", &self.UintInstructionCount)
            .field("StaticFlowControlCount", &self.StaticFlowControlCount)
            .field("DynamicFlowControlCount", &self.DynamicFlowControlCount)
            .field("MacroInstructionCount", &self.MacroInstructionCount)
            .field("ArrayInstructionCount", &self.ArrayInstructionCount)
            .field("MovInstructionCount", &self.MovInstructionCount)
            .field("MovcInstructionCount", &self.MovcInstructionCount)
            .field("ConversionInstructionCount", &self.ConversionInstructionCount)
            .field("BitwiseInstructionCount", &self.BitwiseInstructionCount)
            .field("MinFeatureLevel", &self.MinFeatureLevel)
            .field("RequiredFeatureFlags", &self.RequiredFeatureFlags)
            .field("Name", &self.Name)
            .field("FunctionParameterCount", &self.FunctionParameterCount)
            .field("HasReturn", &self.HasReturn)
            .field("Has10Level9VertexShader", &self.Has10Level9VertexShader)
            .field("Has10Level9PixelShader", &self.Has10Level9PixelShader)
            .finish()
    }
}
impl ::core::default::Default for D3D12_GLOBAL_ROOT_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_GLOBAL_ROOT_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.pGlobalRootSignature == other.pGlobalRootSignature
    }
}
impl ::core::cmp::Eq for D3D12_GLOBAL_ROOT_SIGNATURE {}
impl ::core::fmt::Debug for D3D12_GLOBAL_ROOT_SIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_GLOBAL_ROOT_SIGNATURE").field("pGlobalRootSignature", &self.pGlobalRootSignature).finish()
    }
}
impl ::core::default::Default for D3D12_GPU_BASED_VALIDATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_GPU_BASED_VALIDATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_GPU_BASED_VALIDATION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_GPU_BASED_VALIDATION_PIPELINE_STATE_CREATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_GPU_BASED_VALIDATION_SHADER_PATCH_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_GPU_DESCRIPTOR_HANDLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_GPU_DESCRIPTOR_HANDLE {
    fn eq(&self, other: &Self) -> bool {
        self.ptr == other.ptr
    }
}
impl ::core::cmp::Eq for D3D12_GPU_DESCRIPTOR_HANDLE {}
impl ::core::fmt::Debug for D3D12_GPU_DESCRIPTOR_HANDLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_GPU_DESCRIPTOR_HANDLE").field("ptr", &self.ptr).finish()
    }
}
impl ::core::default::Default for D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
    fn eq(&self, other: &Self) -> bool {
        self.StartAddress == other.StartAddress && self.StrideInBytes == other.StrideInBytes
    }
}
impl ::core::cmp::Eq for D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {}
impl ::core::fmt::Debug for D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE").field("StartAddress", &self.StartAddress).field("StrideInBytes", &self.StrideInBytes).finish()
    }
}
impl ::core::default::Default for D3D12_GPU_VIRTUAL_ADDRESS_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_GPU_VIRTUAL_ADDRESS_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.StartAddress == other.StartAddress && self.SizeInBytes == other.SizeInBytes
    }
}
impl ::core::cmp::Eq for D3D12_GPU_VIRTUAL_ADDRESS_RANGE {}
impl ::core::fmt::Debug for D3D12_GPU_VIRTUAL_ADDRESS_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_GPU_VIRTUAL_ADDRESS_RANGE").field("StartAddress", &self.StartAddress).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
impl ::core::default::Default for D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE {
    fn eq(&self, other: &Self) -> bool {
        self.StartAddress == other.StartAddress && self.SizeInBytes == other.SizeInBytes && self.StrideInBytes == other.StrideInBytes
    }
}
impl ::core::cmp::Eq for D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE {}
impl ::core::fmt::Debug for D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_GPU_VIRTUAL_ADDRESS_RANGE_AND_STRIDE").field("StartAddress", &self.StartAddress).field("SizeInBytes", &self.SizeInBytes).field("StrideInBytes", &self.StrideInBytes).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.pRootSignature == other.pRootSignature
            && self.VS == other.VS
            && self.PS == other.PS
            && self.DS == other.DS
            && self.HS == other.HS
            && self.GS == other.GS
            && self.StreamOutput == other.StreamOutput
            && self.BlendState == other.BlendState
            && self.SampleMask == other.SampleMask
            && self.RasterizerState == other.RasterizerState
            && self.DepthStencilState == other.DepthStencilState
            && self.InputLayout == other.InputLayout
            && self.IBStripCutValue == other.IBStripCutValue
            && self.PrimitiveTopologyType == other.PrimitiveTopologyType
            && self.NumRenderTargets == other.NumRenderTargets
            && self.RTVFormats == other.RTVFormats
            && self.DSVFormat == other.DSVFormat
            && self.SampleDesc == other.SampleDesc
            && self.NodeMask == other.NodeMask
            && self.CachedPSO == other.CachedPSO
            && self.Flags == other.Flags
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D12_GRAPHICS_PIPELINE_STATE_DESC {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D12_GRAPHICS_PIPELINE_STATE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_GRAPHICS_PIPELINE_STATE_DESC")
            .field("pRootSignature", &self.pRootSignature)
            .field("VS", &self.VS)
            .field("PS", &self.PS)
            .field("DS", &self.DS)
            .field("HS", &self.HS)
            .field("GS", &self.GS)
            .field("StreamOutput", &self.StreamOutput)
            .field("BlendState", &self.BlendState)
            .field("SampleMask", &self.SampleMask)
            .field("RasterizerState", &self.RasterizerState)
            .field("DepthStencilState", &self.DepthStencilState)
            .field("InputLayout", &self.InputLayout)
            .field("IBStripCutValue", &self.IBStripCutValue)
            .field("PrimitiveTopologyType", &self.PrimitiveTopologyType)
            .field("NumRenderTargets", &self.NumRenderTargets)
            .field("RTVFormats", &self.RTVFormats)
            .field("DSVFormat", &self.DSVFormat)
            .field("SampleDesc", &self.SampleDesc)
            .field("NodeMask", &self.NodeMask)
            .field("CachedPSO", &self.CachedPSO)
            .field("Flags", &self.Flags)
            .finish()
    }
}
impl ::core::default::Default for D3D12_GRAPHICS_STATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_GRAPHICS_STATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_GRAPHICS_STATES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_GRAPHICS_STATES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_GRAPHICS_STATES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_GRAPHICS_STATES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_GRAPHICS_STATES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_GRAPHICS_STATES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_HEAP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_HEAP_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.SizeInBytes == other.SizeInBytes && self.Properties == other.Properties && self.Alignment == other.Alignment && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D12_HEAP_DESC {}
impl ::core::fmt::Debug for D3D12_HEAP_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_HEAP_DESC").field("SizeInBytes", &self.SizeInBytes).field("Properties", &self.Properties).field("Alignment", &self.Alignment).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D12_HEAP_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_HEAP_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_HEAP_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_HEAP_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_HEAP_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_HEAP_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_HEAP_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_HEAP_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_HEAP_PROPERTIES {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_HEAP_PROPERTIES {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.CPUPageProperty == other.CPUPageProperty && self.MemoryPoolPreference == other.MemoryPoolPreference && self.CreationNodeMask == other.CreationNodeMask && self.VisibleNodeMask == other.VisibleNodeMask
    }
}
impl ::core::cmp::Eq for D3D12_HEAP_PROPERTIES {}
impl ::core::fmt::Debug for D3D12_HEAP_PROPERTIES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_HEAP_PROPERTIES").field("Type", &self.Type).field("CPUPageProperty", &self.CPUPageProperty).field("MemoryPoolPreference", &self.MemoryPoolPreference).field("CreationNodeMask", &self.CreationNodeMask).field("VisibleNodeMask", &self.VisibleNodeMask).finish()
    }
}
impl ::core::default::Default for D3D12_HEAP_SERIALIZATION_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_HEAP_SERIALIZATION_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_HEAP_SERIALIZATION_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_HEAP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_HEAP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_HEAP_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_HIT_GROUP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_HIT_GROUP_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.HitGroupExport == other.HitGroupExport && self.Type == other.Type && self.AnyHitShaderImport == other.AnyHitShaderImport && self.ClosestHitShaderImport == other.ClosestHitShaderImport && self.IntersectionShaderImport == other.IntersectionShaderImport
    }
}
impl ::core::cmp::Eq for D3D12_HIT_GROUP_DESC {}
impl ::core::fmt::Debug for D3D12_HIT_GROUP_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_HIT_GROUP_DESC").field("HitGroupExport", &self.HitGroupExport).field("Type", &self.Type).field("AnyHitShaderImport", &self.AnyHitShaderImport).field("ClosestHitShaderImport", &self.ClosestHitShaderImport).field("IntersectionShaderImport", &self.IntersectionShaderImport).finish()
    }
}
impl ::core::default::Default for D3D12_HIT_GROUP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_HIT_GROUP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_HIT_GROUP_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_HIT_KIND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_HIT_KIND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_HIT_KIND").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_INDEX_BUFFER_STRIP_CUT_VALUE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_INDEX_BUFFER_STRIP_CUT_VALUE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_INDEX_BUFFER_STRIP_CUT_VALUE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_INDEX_BUFFER_VIEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_INDEX_BUFFER_VIEW {
    fn eq(&self, other: &Self) -> bool {
        self.BufferLocation == other.BufferLocation && self.SizeInBytes == other.SizeInBytes && self.Format == other.Format
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_INDEX_BUFFER_VIEW {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_INDEX_BUFFER_VIEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_INDEX_BUFFER_VIEW").field("BufferLocation", &self.BufferLocation).field("SizeInBytes", &self.SizeInBytes).field("Format", &self.Format).finish()
    }
}
impl ::core::default::Default for D3D12_INDIRECT_ARGUMENT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_INDIRECT_ARGUMENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_INDIRECT_ARGUMENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_INDIRECT_ARGUMENT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_INFO_QUEUE_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_INFO_QUEUE_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.AllowList == other.AllowList && self.DenyList == other.DenyList
    }
}
impl ::core::cmp::Eq for D3D12_INFO_QUEUE_FILTER {}
impl ::core::fmt::Debug for D3D12_INFO_QUEUE_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_INFO_QUEUE_FILTER").field("AllowList", &self.AllowList).field("DenyList", &self.DenyList).finish()
    }
}
impl ::core::default::Default for D3D12_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_INFO_QUEUE_FILTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NumCategories == other.NumCategories && self.pCategoryList == other.pCategoryList && self.NumSeverities == other.NumSeverities && self.pSeverityList == other.pSeverityList && self.NumIDs == other.NumIDs && self.pIDList == other.pIDList
    }
}
impl ::core::cmp::Eq for D3D12_INFO_QUEUE_FILTER_DESC {}
impl ::core::fmt::Debug for D3D12_INFO_QUEUE_FILTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_INFO_QUEUE_FILTER_DESC").field("NumCategories", &self.NumCategories).field("pCategoryList", &self.pCategoryList).field("NumSeverities", &self.NumSeverities).field("pSeverityList", &self.pSeverityList).field("NumIDs", &self.NumIDs).field("pIDList", &self.pIDList).finish()
    }
}
impl ::core::default::Default for D3D12_INPUT_CLASSIFICATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_INPUT_CLASSIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_INPUT_CLASSIFICATION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_INPUT_ELEMENT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.SemanticName == other.SemanticName && self.SemanticIndex == other.SemanticIndex && self.Format == other.Format && self.InputSlot == other.InputSlot && self.AlignedByteOffset == other.AlignedByteOffset && self.InputSlotClass == other.InputSlotClass && self.InstanceDataStepRate == other.InstanceDataStepRate
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_INPUT_ELEMENT_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_INPUT_ELEMENT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_INPUT_ELEMENT_DESC").field("SemanticName", &self.SemanticName).field("SemanticIndex", &self.SemanticIndex).field("Format", &self.Format).field("InputSlot", &self.InputSlot).field("AlignedByteOffset", &self.AlignedByteOffset).field("InputSlotClass", &self.InputSlotClass).field("InstanceDataStepRate", &self.InstanceDataStepRate).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_INPUT_LAYOUT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_INPUT_LAYOUT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.pInputElementDescs == other.pInputElementDescs && self.NumElements == other.NumElements
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_INPUT_LAYOUT_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_INPUT_LAYOUT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_INPUT_LAYOUT_DESC").field("pInputElementDescs", &self.pInputElementDescs).field("NumElements", &self.NumElements).finish()
    }
}
impl ::core::default::Default for D3D12_LIBRARY_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_LIBRARY_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Creator == other.Creator && self.Flags == other.Flags && self.FunctionCount == other.FunctionCount
    }
}
impl ::core::cmp::Eq for D3D12_LIBRARY_DESC {}
impl ::core::fmt::Debug for D3D12_LIBRARY_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_LIBRARY_DESC").field("Creator", &self.Creator).field("Flags", &self.Flags).field("FunctionCount", &self.FunctionCount).finish()
    }
}
impl ::core::default::Default for D3D12_LIFETIME_STATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_LIFETIME_STATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_LIFETIME_STATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_LOCAL_ROOT_SIGNATURE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_LOCAL_ROOT_SIGNATURE {
    fn eq(&self, other: &Self) -> bool {
        self.pLocalRootSignature == other.pLocalRootSignature
    }
}
impl ::core::cmp::Eq for D3D12_LOCAL_ROOT_SIGNATURE {}
impl ::core::fmt::Debug for D3D12_LOCAL_ROOT_SIGNATURE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_LOCAL_ROOT_SIGNATURE").field("pLocalRootSignature", &self.pLocalRootSignature).finish()
    }
}
impl ::core::default::Default for D3D12_LOGIC_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_LOGIC_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_LOGIC_OP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_MEASUREMENTS_ACTION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_MEASUREMENTS_ACTION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_MEASUREMENTS_ACTION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_MEMCPY_DEST {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_MEMCPY_DEST {
    fn eq(&self, other: &Self) -> bool {
        self.pData == other.pData && self.RowPitch == other.RowPitch && self.SlicePitch == other.SlicePitch
    }
}
impl ::core::cmp::Eq for D3D12_MEMCPY_DEST {}
impl ::core::fmt::Debug for D3D12_MEMCPY_DEST {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_MEMCPY_DEST").field("pData", &self.pData).field("RowPitch", &self.RowPitch).field("SlicePitch", &self.SlicePitch).finish()
    }
}
impl ::core::default::Default for D3D12_MEMORY_POOL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_MEMORY_POOL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_MEMORY_POOL").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_MESH_SHADER_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_MESH_SHADER_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_MESH_SHADER_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Category == other.Category && self.Severity == other.Severity && self.ID == other.ID && self.pDescription == other.pDescription && self.DescriptionByteLength == other.DescriptionByteLength
    }
}
impl ::core::cmp::Eq for D3D12_MESSAGE {}
impl ::core::fmt::Debug for D3D12_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_MESSAGE").field("Category", &self.Category).field("Severity", &self.Severity).field("ID", &self.ID).field("pDescription", &self.pDescription).field("DescriptionByteLength", &self.DescriptionByteLength).finish()
    }
}
impl ::core::default::Default for D3D12_MESSAGE_CALLBACK_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_MESSAGE_CALLBACK_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_MESSAGE_CALLBACK_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_MESSAGE_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_MESSAGE_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_MESSAGE_CATEGORY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_MESSAGE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_MESSAGE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_MESSAGE_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_MESSAGE_SEVERITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_MESSAGE_SEVERITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_MESSAGE_SEVERITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_META_COMMAND_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_META_COMMAND_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.Name == other.Name && self.InitializationDirtyState == other.InitializationDirtyState && self.ExecutionDirtyState == other.ExecutionDirtyState
    }
}
impl ::core::cmp::Eq for D3D12_META_COMMAND_DESC {}
impl ::core::fmt::Debug for D3D12_META_COMMAND_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_META_COMMAND_DESC").field("Id", &self.Id).field("Name", &self.Name).field("InitializationDirtyState", &self.InitializationDirtyState).field("ExecutionDirtyState", &self.ExecutionDirtyState).finish()
    }
}
impl ::core::default::Default for D3D12_META_COMMAND_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_META_COMMAND_PARAMETER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Type == other.Type && self.Flags == other.Flags && self.RequiredResourceState == other.RequiredResourceState && self.StructureOffset == other.StructureOffset
    }
}
impl ::core::cmp::Eq for D3D12_META_COMMAND_PARAMETER_DESC {}
impl ::core::fmt::Debug for D3D12_META_COMMAND_PARAMETER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_META_COMMAND_PARAMETER_DESC").field("Name", &self.Name).field("Type", &self.Type).field("Flags", &self.Flags).field("RequiredResourceState", &self.RequiredResourceState).field("StructureOffset", &self.StructureOffset).finish()
    }
}
impl ::core::default::Default for D3D12_META_COMMAND_PARAMETER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_META_COMMAND_PARAMETER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_META_COMMAND_PARAMETER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_META_COMMAND_PARAMETER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_META_COMMAND_PARAMETER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_META_COMMAND_PARAMETER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_META_COMMAND_PARAMETER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_META_COMMAND_PARAMETER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_META_COMMAND_PARAMETER_STAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_META_COMMAND_PARAMETER_STAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_META_COMMAND_PARAMETER_STAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_META_COMMAND_PARAMETER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_META_COMMAND_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_META_COMMAND_PARAMETER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_MIP_REGION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_MIP_REGION {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Depth == other.Depth
    }
}
impl ::core::cmp::Eq for D3D12_MIP_REGION {}
impl ::core::fmt::Debug for D3D12_MIP_REGION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_MIP_REGION").field("Width", &self.Width).field("Height", &self.Height).field("Depth", &self.Depth).finish()
    }
}
impl ::core::default::Default for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_MULTIPLE_FENCE_WAIT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_MULTIPLE_FENCE_WAIT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_NODE_MASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_NODE_MASK {
    fn eq(&self, other: &Self) -> bool {
        self.NodeMask == other.NodeMask
    }
}
impl ::core::cmp::Eq for D3D12_NODE_MASK {}
impl ::core::fmt::Debug for D3D12_NODE_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_NODE_MASK").field("NodeMask", &self.NodeMask).finish()
    }
}
impl ::core::default::Default for D3D12_PACKED_MIP_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_PACKED_MIP_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.NumStandardMips == other.NumStandardMips && self.NumPackedMips == other.NumPackedMips && self.NumTilesForPackedMips == other.NumTilesForPackedMips && self.StartTileIndexInOverallResource == other.StartTileIndexInOverallResource
    }
}
impl ::core::cmp::Eq for D3D12_PACKED_MIP_INFO {}
impl ::core::fmt::Debug for D3D12_PACKED_MIP_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_PACKED_MIP_INFO").field("NumStandardMips", &self.NumStandardMips).field("NumPackedMips", &self.NumPackedMips).field("NumTilesForPackedMips", &self.NumTilesForPackedMips).field("StartTileIndexInOverallResource", &self.StartTileIndexInOverallResource).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D12_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D12_PARAMETER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.SemanticName == other.SemanticName && self.Type == other.Type && self.Class == other.Class && self.Rows == other.Rows && self.Columns == other.Columns && self.InterpolationMode == other.InterpolationMode && self.Flags == other.Flags && self.FirstInRegister == other.FirstInRegister && self.FirstInComponent == other.FirstInComponent && self.FirstOutRegister == other.FirstOutRegister && self.FirstOutComponent == other.FirstOutComponent
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D12_PARAMETER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D12_PARAMETER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_PARAMETER_DESC")
            .field("Name", &self.Name)
            .field("SemanticName", &self.SemanticName)
            .field("Type", &self.Type)
            .field("Class", &self.Class)
            .field("Rows", &self.Rows)
            .field("Columns", &self.Columns)
            .field("InterpolationMode", &self.InterpolationMode)
            .field("Flags", &self.Flags)
            .field("FirstInRegister", &self.FirstInRegister)
            .field("FirstInComponent", &self.FirstInComponent)
            .field("FirstOutRegister", &self.FirstOutRegister)
            .field("FirstOutComponent", &self.FirstOutComponent)
            .finish()
    }
}
impl ::core::default::Default for D3D12_PIPELINE_STATE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_PIPELINE_STATE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_PIPELINE_STATE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_PIPELINE_STATE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_PIPELINE_STATE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_PIPELINE_STATE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_PIPELINE_STATE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_PIPELINE_STATE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_PIPELINE_STATE_STREAM_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_PIPELINE_STATE_STREAM_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.SizeInBytes == other.SizeInBytes && self.pPipelineStateSubobjectStream == other.pPipelineStateSubobjectStream
    }
}
impl ::core::cmp::Eq for D3D12_PIPELINE_STATE_STREAM_DESC {}
impl ::core::fmt::Debug for D3D12_PIPELINE_STATE_STREAM_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_PIPELINE_STATE_STREAM_DESC").field("SizeInBytes", &self.SizeInBytes).field("pPipelineStateSubobjectStream", &self.pPipelineStateSubobjectStream).finish()
    }
}
impl ::core::default::Default for D3D12_PIPELINE_STATE_SUBOBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_PIPELINE_STATE_SUBOBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_PIPELINE_STATE_SUBOBJECT_TYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Footprint == other.Footprint
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_PLACED_SUBRESOURCE_FOOTPRINT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_PLACED_SUBRESOURCE_FOOTPRINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_PLACED_SUBRESOURCE_FOOTPRINT").field("Offset", &self.Offset).field("Footprint", &self.Footprint).finish()
    }
}
impl ::core::default::Default for D3D12_PREDICATION_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_PREDICATION_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_PREDICATION_OP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_PRIMITIVE_TOPOLOGY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_PRIMITIVE_TOPOLOGY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_PRIMITIVE_TOPOLOGY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_PROTECTED_RESOURCE_SESSION_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_PROTECTED_RESOURCE_SESSION_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NodeMask == other.NodeMask && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D12_PROTECTED_RESOURCE_SESSION_DESC {}
impl ::core::fmt::Debug for D3D12_PROTECTED_RESOURCE_SESSION_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_PROTECTED_RESOURCE_SESSION_DESC").field("NodeMask", &self.NodeMask).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.NodeMask == other.NodeMask && self.Flags == other.Flags && self.ProtectionType == other.ProtectionType
    }
}
impl ::core::cmp::Eq for D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {}
impl ::core::fmt::Debug for D3D12_PROTECTED_RESOURCE_SESSION_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_PROTECTED_RESOURCE_SESSION_DESC1").field("NodeMask", &self.NodeMask).field("Flags", &self.Flags).field("ProtectionType", &self.ProtectionType).finish()
    }
}
impl ::core::default::Default for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_PROTECTED_RESOURCE_SESSION_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_PROTECTED_RESOURCE_SESSION_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_PROTECTED_SESSION_STATUS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_PROTECTED_SESSION_STATUS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_PROTECTED_SESSION_STATUS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_QUERY_DATA_PIPELINE_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_QUERY_DATA_PIPELINE_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.IAVertices == other.IAVertices && self.IAPrimitives == other.IAPrimitives && self.VSInvocations == other.VSInvocations && self.GSInvocations == other.GSInvocations && self.GSPrimitives == other.GSPrimitives && self.CInvocations == other.CInvocations && self.CPrimitives == other.CPrimitives && self.PSInvocations == other.PSInvocations && self.HSInvocations == other.HSInvocations && self.DSInvocations == other.DSInvocations && self.CSInvocations == other.CSInvocations
    }
}
impl ::core::cmp::Eq for D3D12_QUERY_DATA_PIPELINE_STATISTICS {}
impl ::core::fmt::Debug for D3D12_QUERY_DATA_PIPELINE_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_QUERY_DATA_PIPELINE_STATISTICS")
            .field("IAVertices", &self.IAVertices)
            .field("IAPrimitives", &self.IAPrimitives)
            .field("VSInvocations", &self.VSInvocations)
            .field("GSInvocations", &self.GSInvocations)
            .field("GSPrimitives", &self.GSPrimitives)
            .field("CInvocations", &self.CInvocations)
            .field("CPrimitives", &self.CPrimitives)
            .field("PSInvocations", &self.PSInvocations)
            .field("HSInvocations", &self.HSInvocations)
            .field("DSInvocations", &self.DSInvocations)
            .field("CSInvocations", &self.CSInvocations)
            .finish()
    }
}
impl ::core::default::Default for D3D12_QUERY_DATA_PIPELINE_STATISTICS1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_QUERY_DATA_PIPELINE_STATISTICS1 {
    fn eq(&self, other: &Self) -> bool {
        self.IAVertices == other.IAVertices && self.IAPrimitives == other.IAPrimitives && self.VSInvocations == other.VSInvocations && self.GSInvocations == other.GSInvocations && self.GSPrimitives == other.GSPrimitives && self.CInvocations == other.CInvocations && self.CPrimitives == other.CPrimitives && self.PSInvocations == other.PSInvocations && self.HSInvocations == other.HSInvocations && self.DSInvocations == other.DSInvocations && self.CSInvocations == other.CSInvocations && self.ASInvocations == other.ASInvocations && self.MSInvocations == other.MSInvocations && self.MSPrimitives == other.MSPrimitives
    }
}
impl ::core::cmp::Eq for D3D12_QUERY_DATA_PIPELINE_STATISTICS1 {}
impl ::core::fmt::Debug for D3D12_QUERY_DATA_PIPELINE_STATISTICS1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_QUERY_DATA_PIPELINE_STATISTICS1")
            .field("IAVertices", &self.IAVertices)
            .field("IAPrimitives", &self.IAPrimitives)
            .field("VSInvocations", &self.VSInvocations)
            .field("GSInvocations", &self.GSInvocations)
            .field("GSPrimitives", &self.GSPrimitives)
            .field("CInvocations", &self.CInvocations)
            .field("CPrimitives", &self.CPrimitives)
            .field("PSInvocations", &self.PSInvocations)
            .field("HSInvocations", &self.HSInvocations)
            .field("DSInvocations", &self.DSInvocations)
            .field("CSInvocations", &self.CSInvocations)
            .field("ASInvocations", &self.ASInvocations)
            .field("MSInvocations", &self.MSInvocations)
            .field("MSPrimitives", &self.MSPrimitives)
            .finish()
    }
}
impl ::core::default::Default for D3D12_QUERY_DATA_SO_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_QUERY_DATA_SO_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.NumPrimitivesWritten == other.NumPrimitivesWritten && self.PrimitivesStorageNeeded == other.PrimitivesStorageNeeded
    }
}
impl ::core::cmp::Eq for D3D12_QUERY_DATA_SO_STATISTICS {}
impl ::core::fmt::Debug for D3D12_QUERY_DATA_SO_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_QUERY_DATA_SO_STATISTICS").field("NumPrimitivesWritten", &self.NumPrimitivesWritten).field("PrimitivesStorageNeeded", &self.PrimitivesStorageNeeded).finish()
    }
}
impl ::core::default::Default for D3D12_QUERY_HEAP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_QUERY_HEAP_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.Count == other.Count && self.NodeMask == other.NodeMask
    }
}
impl ::core::cmp::Eq for D3D12_QUERY_HEAP_DESC {}
impl ::core::fmt::Debug for D3D12_QUERY_HEAP_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_QUERY_HEAP_DESC").field("Type", &self.Type).field("Count", &self.Count).field("NodeMask", &self.NodeMask).finish()
    }
}
impl ::core::default::Default for D3D12_QUERY_HEAP_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_QUERY_HEAP_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_QUERY_HEAP_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_QUERY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_QUERY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_QUERY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_RANGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RANGE {
    fn eq(&self, other: &Self) -> bool {
        self.Begin == other.Begin && self.End == other.End
    }
}
impl ::core::cmp::Eq for D3D12_RANGE {}
impl ::core::fmt::Debug for D3D12_RANGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RANGE").field("Begin", &self.Begin).field("End", &self.End).finish()
    }
}
impl ::core::default::Default for D3D12_RANGE_UINT64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RANGE_UINT64 {
    fn eq(&self, other: &Self) -> bool {
        self.Begin == other.Begin && self.End == other.End
    }
}
impl ::core::cmp::Eq for D3D12_RANGE_UINT64 {}
impl ::core::fmt::Debug for D3D12_RANGE_UINT64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RANGE_UINT64").field("Begin", &self.Begin).field("End", &self.End).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_RASTERIZER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_RASTERIZER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.FillMode == other.FillMode && self.CullMode == other.CullMode && self.FrontCounterClockwise == other.FrontCounterClockwise && self.DepthBias == other.DepthBias && self.DepthBiasClamp == other.DepthBiasClamp && self.SlopeScaledDepthBias == other.SlopeScaledDepthBias && self.DepthClipEnable == other.DepthClipEnable && self.MultisampleEnable == other.MultisampleEnable && self.AntialiasedLineEnable == other.AntialiasedLineEnable && self.ForcedSampleCount == other.ForcedSampleCount && self.ConservativeRaster == other.ConservativeRaster
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_RASTERIZER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_RASTERIZER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RASTERIZER_DESC")
            .field("FillMode", &self.FillMode)
            .field("CullMode", &self.CullMode)
            .field("FrontCounterClockwise", &self.FrontCounterClockwise)
            .field("DepthBias", &self.DepthBias)
            .field("DepthBiasClamp", &self.DepthBiasClamp)
            .field("SlopeScaledDepthBias", &self.SlopeScaledDepthBias)
            .field("DepthClipEnable", &self.DepthClipEnable)
            .field("MultisampleEnable", &self.MultisampleEnable)
            .field("AntialiasedLineEnable", &self.AntialiasedLineEnable)
            .field("ForcedSampleCount", &self.ForcedSampleCount)
            .field("ConservativeRaster", &self.ConservativeRaster)
            .finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_AABB {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RAYTRACING_AABB {
    fn eq(&self, other: &Self) -> bool {
        self.MinX == other.MinX && self.MinY == other.MinY && self.MinZ == other.MinZ && self.MaxX == other.MaxX && self.MaxY == other.MaxY && self.MaxZ == other.MaxZ
    }
}
impl ::core::cmp::Eq for D3D12_RAYTRACING_AABB {}
impl ::core::fmt::Debug for D3D12_RAYTRACING_AABB {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RAYTRACING_AABB").field("MinX", &self.MinX).field("MinY", &self.MinY).field("MinZ", &self.MinZ).field("MaxX", &self.MaxX).field("MaxY", &self.MaxY).field("MaxZ", &self.MaxZ).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.CompactedSizeInBytes == other.CompactedSizeInBytes
    }
}
impl ::core::cmp::Eq for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC {}
impl ::core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE_DESC").field("CompactedSizeInBytes", &self.CompactedSizeInBytes).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.CurrentSizeInBytes == other.CurrentSizeInBytes
    }
}
impl ::core::cmp::Eq for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC {}
impl ::core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE_DESC").field("CurrentSizeInBytes", &self.CurrentSizeInBytes).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DestBuffer == other.DestBuffer && self.InfoType == other.InfoType
    }
}
impl ::core::cmp::Eq for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC {}
impl ::core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC").field("DestBuffer", &self.DestBuffer).field("InfoType", &self.InfoType).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.SerializedSizeInBytes == other.SerializedSizeInBytes && self.NumBottomLevelAccelerationStructurePointers == other.NumBottomLevelAccelerationStructurePointers
    }
}
impl ::core::cmp::Eq for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC {}
impl ::core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION_DESC").field("SerializedSizeInBytes", &self.SerializedSizeInBytes).field("NumBottomLevelAccelerationStructurePointers", &self.NumBottomLevelAccelerationStructurePointers).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DecodedSizeInBytes == other.DecodedSizeInBytes
    }
}
impl ::core::cmp::Eq for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC {}
impl ::core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION_DESC").field("DecodedSizeInBytes", &self.DecodedSizeInBytes).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ResultDataMaxSizeInBytes == other.ResultDataMaxSizeInBytes && self.ScratchDataSizeInBytes == other.ScratchDataSizeInBytes && self.UpdateScratchDataSizeInBytes == other.UpdateScratchDataSizeInBytes
    }
}
impl ::core::cmp::Eq for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO {}
impl ::core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO").field("ResultDataMaxSizeInBytes", &self.ResultDataMaxSizeInBytes).field("ScratchDataSizeInBytes", &self.ScratchDataSizeInBytes).field("UpdateScratchDataSizeInBytes", &self.UpdateScratchDataSizeInBytes).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.Location == other.Location
    }
}
impl ::core::cmp::Eq for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {}
impl ::core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_SRV").field("Location", &self.Location).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.AABBCount == other.AABBCount && self.AABBs == other.AABBs
    }
}
impl ::core::cmp::Eq for D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {}
impl ::core::fmt::Debug for D3D12_RAYTRACING_GEOMETRY_AABBS_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RAYTRACING_GEOMETRY_AABBS_DESC").field("AABBCount", &self.AABBCount).field("AABBs", &self.AABBs).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_RAYTRACING_GEOMETRY_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_GEOMETRY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_RAYTRACING_GEOMETRY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Transform3x4 == other.Transform3x4 && self.IndexFormat == other.IndexFormat && self.VertexFormat == other.VertexFormat && self.IndexCount == other.IndexCount && self.VertexCount == other.VertexCount && self.IndexBuffer == other.IndexBuffer && self.VertexBuffer == other.VertexBuffer
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RAYTRACING_GEOMETRY_TRIANGLES_DESC").field("Transform3x4", &self.Transform3x4).field("IndexFormat", &self.IndexFormat).field("VertexFormat", &self.VertexFormat).field("IndexCount", &self.IndexCount).field("VertexCount", &self.VertexCount).field("IndexBuffer", &self.IndexBuffer).field("VertexBuffer", &self.VertexBuffer).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_GEOMETRY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RAYTRACING_GEOMETRY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_GEOMETRY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_INSTANCE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RAYTRACING_INSTANCE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Transform == other.Transform && self._bitfield1 == other._bitfield1 && self._bitfield2 == other._bitfield2 && self.AccelerationStructure == other.AccelerationStructure
    }
}
impl ::core::cmp::Eq for D3D12_RAYTRACING_INSTANCE_DESC {}
impl ::core::fmt::Debug for D3D12_RAYTRACING_INSTANCE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RAYTRACING_INSTANCE_DESC").field("Transform", &self.Transform).field("_bitfield1", &self._bitfield1).field("_bitfield2", &self._bitfield2).field("AccelerationStructure", &self.AccelerationStructure).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_INSTANCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RAYTRACING_INSTANCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_INSTANCE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_RAYTRACING_INSTANCE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_RAYTRACING_INSTANCE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_RAYTRACING_INSTANCE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_RAYTRACING_INSTANCE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_RAYTRACING_INSTANCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_PIPELINE_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RAYTRACING_PIPELINE_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.MaxTraceRecursionDepth == other.MaxTraceRecursionDepth
    }
}
impl ::core::cmp::Eq for D3D12_RAYTRACING_PIPELINE_CONFIG {}
impl ::core::fmt::Debug for D3D12_RAYTRACING_PIPELINE_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RAYTRACING_PIPELINE_CONFIG").field("MaxTraceRecursionDepth", &self.MaxTraceRecursionDepth).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_PIPELINE_CONFIG1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RAYTRACING_PIPELINE_CONFIG1 {
    fn eq(&self, other: &Self) -> bool {
        self.MaxTraceRecursionDepth == other.MaxTraceRecursionDepth && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D12_RAYTRACING_PIPELINE_CONFIG1 {}
impl ::core::fmt::Debug for D3D12_RAYTRACING_PIPELINE_CONFIG1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RAYTRACING_PIPELINE_CONFIG1").field("MaxTraceRecursionDepth", &self.MaxTraceRecursionDepth).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_PIPELINE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RAYTRACING_PIPELINE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_PIPELINE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_RAYTRACING_PIPELINE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_RAYTRACING_PIPELINE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_RAYTRACING_PIPELINE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_RAYTRACING_PIPELINE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_RAYTRACING_PIPELINE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_SHADER_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RAYTRACING_SHADER_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.MaxPayloadSizeInBytes == other.MaxPayloadSizeInBytes && self.MaxAttributeSizeInBytes == other.MaxAttributeSizeInBytes
    }
}
impl ::core::cmp::Eq for D3D12_RAYTRACING_SHADER_CONFIG {}
impl ::core::fmt::Debug for D3D12_RAYTRACING_SHADER_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RAYTRACING_SHADER_CONFIG").field("MaxPayloadSizeInBytes", &self.MaxPayloadSizeInBytes).field("MaxAttributeSizeInBytes", &self.MaxAttributeSizeInBytes).finish()
    }
}
impl ::core::default::Default for D3D12_RAYTRACING_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RAYTRACING_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RAYTRACING_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_RAY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RAY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RAY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_RAY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_RAY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_RAY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_RAY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_RAY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_RENDER_PASS_BEGINNING_ACCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_RENDER_PASS_BEGINNING_ACCESS_CLEAR_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE").field(&self.0).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_RENDER_PASS_DEPTH_STENCIL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_RENDER_PASS_ENDING_ACCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::PartialEq for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.pSrcResource == other.pSrcResource && self.pDstResource == other.pDstResource && self.SubresourceCount == other.SubresourceCount && self.pSubresourceParameters == other.pSubresourceParameters && self.Format == other.Format && self.ResolveMode == other.ResolveMode && self.PreserveResolveSource == other.PreserveResolveSource
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::cmp::Eq for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::fmt::Debug for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_PARAMETERS").field("pSrcResource", &self.pSrcResource).field("pDstResource", &self.pDstResource).field("SubresourceCount", &self.SubresourceCount).field("pSubresourceParameters", &self.pSubresourceParameters).field("Format", &self.Format).field("ResolveMode", &self.ResolveMode).field("PreserveResolveSource", &self.PreserveResolveSource).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS {
    fn eq(&self, other: &Self) -> bool {
        self.SrcSubresource == other.SrcSubresource && self.DstSubresource == other.DstSubresource && self.DstX == other.DstX && self.DstY == other.DstY && self.SrcRect == other.SrcRect
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RENDER_PASS_ENDING_ACCESS_RESOLVE_SUBRESOURCE_PARAMETERS").field("SrcSubresource", &self.SrcSubresource).field("DstSubresource", &self.DstSubresource).field("DstX", &self.DstX).field("DstY", &self.DstY).field("SrcRect", &self.SrcRect).finish()
    }
}
impl ::core::default::Default for D3D12_RENDER_PASS_ENDING_ACCESS_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RENDER_PASS_ENDING_ACCESS_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RENDER_PASS_ENDING_ACCESS_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_RENDER_PASS_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RENDER_PASS_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RENDER_PASS_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_RENDER_PASS_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_RENDER_PASS_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_RENDER_PASS_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_RENDER_PASS_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_RENDER_PASS_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D12_RENDER_PASS_RENDER_TARGET_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_RENDER_PASS_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RENDER_PASS_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RENDER_PASS_TIER").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_RENDER_TARGET_BLEND_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_RENDER_TARGET_BLEND_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.BlendEnable == other.BlendEnable && self.LogicOpEnable == other.LogicOpEnable && self.SrcBlend == other.SrcBlend && self.DestBlend == other.DestBlend && self.BlendOp == other.BlendOp && self.SrcBlendAlpha == other.SrcBlendAlpha && self.DestBlendAlpha == other.DestBlendAlpha && self.BlendOpAlpha == other.BlendOpAlpha && self.LogicOp == other.LogicOp && self.RenderTargetWriteMask == other.RenderTargetWriteMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_RENDER_TARGET_BLEND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_RENDER_TARGET_BLEND_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RENDER_TARGET_BLEND_DESC").field("BlendEnable", &self.BlendEnable).field("LogicOpEnable", &self.LogicOpEnable).field("SrcBlend", &self.SrcBlend).field("DestBlend", &self.DestBlend).field("BlendOp", &self.BlendOp).field("SrcBlendAlpha", &self.SrcBlendAlpha).field("DestBlendAlpha", &self.DestBlendAlpha).field("BlendOpAlpha", &self.BlendOpAlpha).field("LogicOp", &self.LogicOp).field("RenderTargetWriteMask", &self.RenderTargetWriteMask).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_RESIDENCY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RESIDENCY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RESIDENCY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_RESIDENCY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_RESIDENCY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_RESIDENCY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_RESIDENCY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_RESIDENCY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_RESIDENCY_PRIORITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RESIDENCY_PRIORITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RESIDENCY_PRIORITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_RESOLVE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RESOLVE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RESOLVE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_RESOURCE_ALIASING_BARRIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RESOURCE_ALIASING_BARRIER {
    fn eq(&self, other: &Self) -> bool {
        self.pResourceBefore == other.pResourceBefore && self.pResourceAfter == other.pResourceAfter
    }
}
impl ::core::cmp::Eq for D3D12_RESOURCE_ALIASING_BARRIER {}
impl ::core::fmt::Debug for D3D12_RESOURCE_ALIASING_BARRIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RESOURCE_ALIASING_BARRIER").field("pResourceBefore", &self.pResourceBefore).field("pResourceAfter", &self.pResourceAfter).finish()
    }
}
impl ::core::default::Default for D3D12_RESOURCE_ALLOCATION_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RESOURCE_ALLOCATION_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.SizeInBytes == other.SizeInBytes && self.Alignment == other.Alignment
    }
}
impl ::core::cmp::Eq for D3D12_RESOURCE_ALLOCATION_INFO {}
impl ::core::fmt::Debug for D3D12_RESOURCE_ALLOCATION_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RESOURCE_ALLOCATION_INFO").field("SizeInBytes", &self.SizeInBytes).field("Alignment", &self.Alignment).finish()
    }
}
impl ::core::default::Default for D3D12_RESOURCE_ALLOCATION_INFO1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RESOURCE_ALLOCATION_INFO1 {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.Alignment == other.Alignment && self.SizeInBytes == other.SizeInBytes
    }
}
impl ::core::cmp::Eq for D3D12_RESOURCE_ALLOCATION_INFO1 {}
impl ::core::fmt::Debug for D3D12_RESOURCE_ALLOCATION_INFO1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RESOURCE_ALLOCATION_INFO1").field("Offset", &self.Offset).field("Alignment", &self.Alignment).field("SizeInBytes", &self.SizeInBytes).finish()
    }
}
impl ::core::default::Default for D3D12_RESOURCE_BARRIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_RESOURCE_BARRIER_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RESOURCE_BARRIER_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RESOURCE_BARRIER_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_RESOURCE_BARRIER_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_RESOURCE_BARRIER_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_RESOURCE_BARRIER_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_RESOURCE_BARRIER_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_RESOURCE_BARRIER_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_RESOURCE_BARRIER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RESOURCE_BARRIER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RESOURCE_BARRIER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_RESOURCE_BINDING_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RESOURCE_BINDING_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RESOURCE_BINDING_TIER").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_RESOURCE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_RESOURCE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Dimension == other.Dimension && self.Alignment == other.Alignment && self.Width == other.Width && self.Height == other.Height && self.DepthOrArraySize == other.DepthOrArraySize && self.MipLevels == other.MipLevels && self.Format == other.Format && self.SampleDesc == other.SampleDesc && self.Layout == other.Layout && self.Flags == other.Flags
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_RESOURCE_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_RESOURCE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RESOURCE_DESC").field("Dimension", &self.Dimension).field("Alignment", &self.Alignment).field("Width", &self.Width).field("Height", &self.Height).field("DepthOrArraySize", &self.DepthOrArraySize).field("MipLevels", &self.MipLevels).field("Format", &self.Format).field("SampleDesc", &self.SampleDesc).field("Layout", &self.Layout).field("Flags", &self.Flags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_RESOURCE_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_RESOURCE_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.Dimension == other.Dimension && self.Alignment == other.Alignment && self.Width == other.Width && self.Height == other.Height && self.DepthOrArraySize == other.DepthOrArraySize && self.MipLevels == other.MipLevels && self.Format == other.Format && self.SampleDesc == other.SampleDesc && self.Layout == other.Layout && self.Flags == other.Flags && self.SamplerFeedbackMipRegion == other.SamplerFeedbackMipRegion
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_RESOURCE_DESC1 {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_RESOURCE_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RESOURCE_DESC1").field("Dimension", &self.Dimension).field("Alignment", &self.Alignment).field("Width", &self.Width).field("Height", &self.Height).field("DepthOrArraySize", &self.DepthOrArraySize).field("MipLevels", &self.MipLevels).field("Format", &self.Format).field("SampleDesc", &self.SampleDesc).field("Layout", &self.Layout).field("Flags", &self.Flags).field("SamplerFeedbackMipRegion", &self.SamplerFeedbackMipRegion).finish()
    }
}
impl ::core::default::Default for D3D12_RESOURCE_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RESOURCE_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RESOURCE_DIMENSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_RESOURCE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RESOURCE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RESOURCE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_RESOURCE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_RESOURCE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_RESOURCE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_RESOURCE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_RESOURCE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_RESOURCE_HEAP_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RESOURCE_HEAP_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RESOURCE_HEAP_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_RESOURCE_STATES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RESOURCE_STATES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RESOURCE_STATES").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_RESOURCE_STATES {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_RESOURCE_STATES {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_RESOURCE_STATES {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_RESOURCE_STATES {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_RESOURCE_STATES {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_RESOURCE_TRANSITION_BARRIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RESOURCE_TRANSITION_BARRIER {
    fn eq(&self, other: &Self) -> bool {
        self.pResource == other.pResource && self.Subresource == other.Subresource && self.StateBefore == other.StateBefore && self.StateAfter == other.StateAfter
    }
}
impl ::core::cmp::Eq for D3D12_RESOURCE_TRANSITION_BARRIER {}
impl ::core::fmt::Debug for D3D12_RESOURCE_TRANSITION_BARRIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RESOURCE_TRANSITION_BARRIER").field("pResource", &self.pResource).field("Subresource", &self.Subresource).field("StateBefore", &self.StateBefore).field("StateAfter", &self.StateAfter).finish()
    }
}
impl ::core::default::Default for D3D12_RESOURCE_UAV_BARRIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_RESOURCE_UAV_BARRIER {
    fn eq(&self, other: &Self) -> bool {
        self.pResource == other.pResource
    }
}
impl ::core::cmp::Eq for D3D12_RESOURCE_UAV_BARRIER {}
impl ::core::fmt::Debug for D3D12_RESOURCE_UAV_BARRIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RESOURCE_UAV_BARRIER").field("pResource", &self.pResource).finish()
    }
}
impl ::core::default::Default for D3D12_RLDO_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RLDO_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RLDO_FLAGS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_ROOT_CONSTANTS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_ROOT_CONSTANTS {
    fn eq(&self, other: &Self) -> bool {
        self.ShaderRegister == other.ShaderRegister && self.RegisterSpace == other.RegisterSpace && self.Num32BitValues == other.Num32BitValues
    }
}
impl ::core::cmp::Eq for D3D12_ROOT_CONSTANTS {}
impl ::core::fmt::Debug for D3D12_ROOT_CONSTANTS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_ROOT_CONSTANTS").field("ShaderRegister", &self.ShaderRegister).field("RegisterSpace", &self.RegisterSpace).field("Num32BitValues", &self.Num32BitValues).finish()
    }
}
impl ::core::default::Default for D3D12_ROOT_DESCRIPTOR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_ROOT_DESCRIPTOR {
    fn eq(&self, other: &Self) -> bool {
        self.ShaderRegister == other.ShaderRegister && self.RegisterSpace == other.RegisterSpace
    }
}
impl ::core::cmp::Eq for D3D12_ROOT_DESCRIPTOR {}
impl ::core::fmt::Debug for D3D12_ROOT_DESCRIPTOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_ROOT_DESCRIPTOR").field("ShaderRegister", &self.ShaderRegister).field("RegisterSpace", &self.RegisterSpace).finish()
    }
}
impl ::core::default::Default for D3D12_ROOT_DESCRIPTOR1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_ROOT_DESCRIPTOR1 {
    fn eq(&self, other: &Self) -> bool {
        self.ShaderRegister == other.ShaderRegister && self.RegisterSpace == other.RegisterSpace && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D12_ROOT_DESCRIPTOR1 {}
impl ::core::fmt::Debug for D3D12_ROOT_DESCRIPTOR1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_ROOT_DESCRIPTOR1").field("ShaderRegister", &self.ShaderRegister).field("RegisterSpace", &self.RegisterSpace).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D12_ROOT_DESCRIPTOR_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_ROOT_DESCRIPTOR_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_ROOT_DESCRIPTOR_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_ROOT_DESCRIPTOR_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_ROOT_DESCRIPTOR_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_ROOT_DESCRIPTOR_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_ROOT_DESCRIPTOR_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_ROOT_DESCRIPTOR_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_ROOT_DESCRIPTOR_TABLE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_ROOT_DESCRIPTOR_TABLE {
    fn eq(&self, other: &Self) -> bool {
        self.NumDescriptorRanges == other.NumDescriptorRanges && self.pDescriptorRanges == other.pDescriptorRanges
    }
}
impl ::core::cmp::Eq for D3D12_ROOT_DESCRIPTOR_TABLE {}
impl ::core::fmt::Debug for D3D12_ROOT_DESCRIPTOR_TABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_ROOT_DESCRIPTOR_TABLE").field("NumDescriptorRanges", &self.NumDescriptorRanges).field("pDescriptorRanges", &self.pDescriptorRanges).finish()
    }
}
impl ::core::default::Default for D3D12_ROOT_DESCRIPTOR_TABLE1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_ROOT_DESCRIPTOR_TABLE1 {
    fn eq(&self, other: &Self) -> bool {
        self.NumDescriptorRanges == other.NumDescriptorRanges && self.pDescriptorRanges == other.pDescriptorRanges
    }
}
impl ::core::cmp::Eq for D3D12_ROOT_DESCRIPTOR_TABLE1 {}
impl ::core::fmt::Debug for D3D12_ROOT_DESCRIPTOR_TABLE1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_ROOT_DESCRIPTOR_TABLE1").field("NumDescriptorRanges", &self.NumDescriptorRanges).field("pDescriptorRanges", &self.pDescriptorRanges).finish()
    }
}
impl ::core::default::Default for D3D12_ROOT_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_ROOT_PARAMETER1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_ROOT_PARAMETER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_ROOT_PARAMETER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_ROOT_PARAMETER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_ROOT_SIGNATURE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_ROOT_SIGNATURE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NumParameters == other.NumParameters && self.pParameters == other.pParameters && self.NumStaticSamplers == other.NumStaticSamplers && self.pStaticSamplers == other.pStaticSamplers && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D12_ROOT_SIGNATURE_DESC {}
impl ::core::fmt::Debug for D3D12_ROOT_SIGNATURE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_ROOT_SIGNATURE_DESC").field("NumParameters", &self.NumParameters).field("pParameters", &self.pParameters).field("NumStaticSamplers", &self.NumStaticSamplers).field("pStaticSamplers", &self.pStaticSamplers).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D12_ROOT_SIGNATURE_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_ROOT_SIGNATURE_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.NumParameters == other.NumParameters && self.pParameters == other.pParameters && self.NumStaticSamplers == other.NumStaticSamplers && self.pStaticSamplers == other.pStaticSamplers && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D12_ROOT_SIGNATURE_DESC1 {}
impl ::core::fmt::Debug for D3D12_ROOT_SIGNATURE_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_ROOT_SIGNATURE_DESC1").field("NumParameters", &self.NumParameters).field("pParameters", &self.pParameters).field("NumStaticSamplers", &self.NumStaticSamplers).field("pStaticSamplers", &self.pStaticSamplers).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D12_ROOT_SIGNATURE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_ROOT_SIGNATURE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_ROOT_SIGNATURE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_ROOT_SIGNATURE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_ROOT_SIGNATURE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_ROOT_SIGNATURE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_ROOT_SIGNATURE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_ROOT_SIGNATURE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_RTV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_RTV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_RTV_DIMENSION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_RT_FORMAT_ARRAY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_RT_FORMAT_ARRAY {
    fn eq(&self, other: &Self) -> bool {
        self.RTFormats == other.RTFormats && self.NumRenderTargets == other.NumRenderTargets
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_RT_FORMAT_ARRAY {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_RT_FORMAT_ARRAY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_RT_FORMAT_ARRAY").field("RTFormats", &self.RTFormats).field("NumRenderTargets", &self.NumRenderTargets).finish()
    }
}
impl ::core::default::Default for D3D12_SAMPLER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_SAMPLER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Filter == other.Filter && self.AddressU == other.AddressU && self.AddressV == other.AddressV && self.AddressW == other.AddressW && self.MipLODBias == other.MipLODBias && self.MaxAnisotropy == other.MaxAnisotropy && self.ComparisonFunc == other.ComparisonFunc && self.BorderColor == other.BorderColor && self.MinLOD == other.MinLOD && self.MaxLOD == other.MaxLOD
    }
}
impl ::core::cmp::Eq for D3D12_SAMPLER_DESC {}
impl ::core::fmt::Debug for D3D12_SAMPLER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SAMPLER_DESC").field("Filter", &self.Filter).field("AddressU", &self.AddressU).field("AddressV", &self.AddressV).field("AddressW", &self.AddressW).field("MipLODBias", &self.MipLODBias).field("MaxAnisotropy", &self.MaxAnisotropy).field("ComparisonFunc", &self.ComparisonFunc).field("BorderColor", &self.BorderColor).field("MinLOD", &self.MinLOD).field("MaxLOD", &self.MaxLOD).finish()
    }
}
impl ::core::default::Default for D3D12_SAMPLER_FEEDBACK_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SAMPLER_FEEDBACK_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SAMPLER_FEEDBACK_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_SAMPLE_POSITION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_SAMPLE_POSITION {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y
    }
}
impl ::core::cmp::Eq for D3D12_SAMPLE_POSITION {}
impl ::core::fmt::Debug for D3D12_SAMPLE_POSITION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SAMPLE_POSITION").field("X", &self.X).field("Y", &self.Y).finish()
    }
}
impl ::core::default::Default for D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER {
    fn eq(&self, other: &Self) -> bool {
        self.DriverOpaqueGUID == other.DriverOpaqueGUID && self.DriverOpaqueVersioningData == other.DriverOpaqueVersioningData
    }
}
impl ::core::cmp::Eq for D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER {}
impl ::core::fmt::Debug for D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER").field("DriverOpaqueGUID", &self.DriverOpaqueGUID).field("DriverOpaqueVersioningData", &self.DriverOpaqueVersioningData).finish()
    }
}
impl ::core::default::Default for D3D12_SERIALIZED_DATA_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SERIALIZED_DATA_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SERIALIZED_DATA_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER {
    fn eq(&self, other: &Self) -> bool {
        self.DriverMatchingIdentifier == other.DriverMatchingIdentifier && self.SerializedSizeInBytesIncludingHeader == other.SerializedSizeInBytesIncludingHeader && self.DeserializedSizeInBytes == other.DeserializedSizeInBytes && self.NumBottomLevelAccelerationStructurePointersAfterHeader == other.NumBottomLevelAccelerationStructurePointersAfterHeader
    }
}
impl ::core::cmp::Eq for D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER {}
impl ::core::fmt::Debug for D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SERIALIZED_RAYTRACING_ACCELERATION_STRUCTURE_HEADER").field("DriverMatchingIdentifier", &self.DriverMatchingIdentifier).field("SerializedSizeInBytesIncludingHeader", &self.SerializedSizeInBytesIncludingHeader).field("DeserializedSizeInBytes", &self.DeserializedSizeInBytes).field("NumBottomLevelAccelerationStructurePointersAfterHeader", &self.NumBottomLevelAccelerationStructurePointersAfterHeader).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D12_SHADER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D12_SHADER_BUFFER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Type == other.Type && self.Variables == other.Variables && self.Size == other.Size && self.uFlags == other.uFlags
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D12_SHADER_BUFFER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D12_SHADER_BUFFER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SHADER_BUFFER_DESC").field("Name", &self.Name).field("Type", &self.Type).field("Variables", &self.Variables).field("Size", &self.Size).field("uFlags", &self.uFlags).finish()
    }
}
impl ::core::default::Default for D3D12_SHADER_BYTECODE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_SHADER_BYTECODE {
    fn eq(&self, other: &Self) -> bool {
        self.pShaderBytecode == other.pShaderBytecode && self.BytecodeLength == other.BytecodeLength
    }
}
impl ::core::cmp::Eq for D3D12_SHADER_BYTECODE {}
impl ::core::fmt::Debug for D3D12_SHADER_BYTECODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SHADER_BYTECODE").field("pShaderBytecode", &self.pShaderBytecode).field("BytecodeLength", &self.BytecodeLength).finish()
    }
}
impl ::core::default::Default for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_CACHE_CONTROL_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_SHADER_CACHE_CONTROL_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_SHADER_CACHE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SHADER_CACHE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_CACHE_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_SHADER_CACHE_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_SHADER_CACHE_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_SHADER_CACHE_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_SHADER_CACHE_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_SHADER_CACHE_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_SHADER_CACHE_KIND_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SHADER_CACHE_KIND_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_CACHE_KIND_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_SHADER_CACHE_KIND_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_SHADER_CACHE_KIND_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_SHADER_CACHE_KIND_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_SHADER_CACHE_KIND_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_SHADER_CACHE_KIND_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_SHADER_CACHE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SHADER_CACHE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_CACHE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_SHADER_CACHE_SESSION_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_SHADER_CACHE_SESSION_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Identifier == other.Identifier && self.Mode == other.Mode && self.Flags == other.Flags && self.MaximumInMemoryCacheSizeBytes == other.MaximumInMemoryCacheSizeBytes && self.MaximumInMemoryCacheEntries == other.MaximumInMemoryCacheEntries && self.MaximumValueFileSizeBytes == other.MaximumValueFileSizeBytes && self.Version == other.Version
    }
}
impl ::core::cmp::Eq for D3D12_SHADER_CACHE_SESSION_DESC {}
impl ::core::fmt::Debug for D3D12_SHADER_CACHE_SESSION_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SHADER_CACHE_SESSION_DESC").field("Identifier", &self.Identifier).field("Mode", &self.Mode).field("Flags", &self.Flags).field("MaximumInMemoryCacheSizeBytes", &self.MaximumInMemoryCacheSizeBytes).field("MaximumInMemoryCacheEntries", &self.MaximumInMemoryCacheEntries).field("MaximumValueFileSizeBytes", &self.MaximumValueFileSizeBytes).field("Version", &self.Version).finish()
    }
}
impl ::core::default::Default for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_CACHE_SUPPORT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_SHADER_CACHE_SUPPORT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_SHADER_COMPONENT_MAPPING {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SHADER_COMPONENT_MAPPING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_COMPONENT_MAPPING").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D12_SHADER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D12_SHADER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Version == other.Version
            && self.Creator == other.Creator
            && self.Flags == other.Flags
            && self.ConstantBuffers == other.ConstantBuffers
            && self.BoundResources == other.BoundResources
            && self.InputParameters == other.InputParameters
            && self.OutputParameters == other.OutputParameters
            && self.InstructionCount == other.InstructionCount
            && self.TempRegisterCount == other.TempRegisterCount
            && self.TempArrayCount == other.TempArrayCount
            && self.DefCount == other.DefCount
            && self.DclCount == other.DclCount
            && self.TextureNormalInstructions == other.TextureNormalInstructions
            && self.TextureLoadInstructions == other.TextureLoadInstructions
            && self.TextureCompInstructions == other.TextureCompInstructions
            && self.TextureBiasInstructions == other.TextureBiasInstructions
            && self.TextureGradientInstructions == other.TextureGradientInstructions
            && self.FloatInstructionCount == other.FloatInstructionCount
            && self.IntInstructionCount == other.IntInstructionCount
            && self.UintInstructionCount == other.UintInstructionCount
            && self.StaticFlowControlCount == other.StaticFlowControlCount
            && self.DynamicFlowControlCount == other.DynamicFlowControlCount
            && self.MacroInstructionCount == other.MacroInstructionCount
            && self.ArrayInstructionCount == other.ArrayInstructionCount
            && self.CutInstructionCount == other.CutInstructionCount
            && self.EmitInstructionCount == other.EmitInstructionCount
            && self.GSOutputTopology == other.GSOutputTopology
            && self.GSMaxOutputVertexCount == other.GSMaxOutputVertexCount
            && self.InputPrimitive == other.InputPrimitive
            && self.PatchConstantParameters == other.PatchConstantParameters
            && self.cGSInstanceCount == other.cGSInstanceCount
            && self.cControlPoints == other.cControlPoints
            && self.HSOutputPrimitive == other.HSOutputPrimitive
            && self.HSPartitioning == other.HSPartitioning
            && self.TessellatorDomain == other.TessellatorDomain
            && self.cBarrierInstructions == other.cBarrierInstructions
            && self.cInterlockedInstructions == other.cInterlockedInstructions
            && self.cTextureStoreInstructions == other.cTextureStoreInstructions
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D12_SHADER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D12_SHADER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SHADER_DESC")
            .field("Version", &self.Version)
            .field("Creator", &self.Creator)
            .field("Flags", &self.Flags)
            .field("ConstantBuffers", &self.ConstantBuffers)
            .field("BoundResources", &self.BoundResources)
            .field("InputParameters", &self.InputParameters)
            .field("OutputParameters", &self.OutputParameters)
            .field("InstructionCount", &self.InstructionCount)
            .field("TempRegisterCount", &self.TempRegisterCount)
            .field("TempArrayCount", &self.TempArrayCount)
            .field("DefCount", &self.DefCount)
            .field("DclCount", &self.DclCount)
            .field("TextureNormalInstructions", &self.TextureNormalInstructions)
            .field("TextureLoadInstructions", &self.TextureLoadInstructions)
            .field("TextureCompInstructions", &self.TextureCompInstructions)
            .field("TextureBiasInstructions", &self.TextureBiasInstructions)
            .field("TextureGradientInstructions", &self.TextureGradientInstructions)
            .field("FloatInstructionCount", &self.FloatInstructionCount)
            .field("IntInstructionCount", &self.IntInstructionCount)
            .field("UintInstructionCount", &self.UintInstructionCount)
            .field("StaticFlowControlCount", &self.StaticFlowControlCount)
            .field("DynamicFlowControlCount", &self.DynamicFlowControlCount)
            .field("MacroInstructionCount", &self.MacroInstructionCount)
            .field("ArrayInstructionCount", &self.ArrayInstructionCount)
            .field("CutInstructionCount", &self.CutInstructionCount)
            .field("EmitInstructionCount", &self.EmitInstructionCount)
            .field("GSOutputTopology", &self.GSOutputTopology)
            .field("GSMaxOutputVertexCount", &self.GSMaxOutputVertexCount)
            .field("InputPrimitive", &self.InputPrimitive)
            .field("PatchConstantParameters", &self.PatchConstantParameters)
            .field("cGSInstanceCount", &self.cGSInstanceCount)
            .field("cControlPoints", &self.cControlPoints)
            .field("HSOutputPrimitive", &self.HSOutputPrimitive)
            .field("HSPartitioning", &self.HSPartitioning)
            .field("TessellatorDomain", &self.TessellatorDomain)
            .field("cBarrierInstructions", &self.cBarrierInstructions)
            .field("cInterlockedInstructions", &self.cInterlockedInstructions)
            .field("cTextureStoreInstructions", &self.cTextureStoreInstructions)
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D12_SHADER_INPUT_BIND_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D12_SHADER_INPUT_BIND_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Type == other.Type && self.BindPoint == other.BindPoint && self.BindCount == other.BindCount && self.uFlags == other.uFlags && self.ReturnType == other.ReturnType && self.Dimension == other.Dimension && self.NumSamples == other.NumSamples && self.Space == other.Space && self.uID == other.uID
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D12_SHADER_INPUT_BIND_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D12_SHADER_INPUT_BIND_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SHADER_INPUT_BIND_DESC").field("Name", &self.Name).field("Type", &self.Type).field("BindPoint", &self.BindPoint).field("BindCount", &self.BindCount).field("uFlags", &self.uFlags).field("ReturnType", &self.ReturnType).field("Dimension", &self.Dimension).field("NumSamples", &self.NumSamples).field("Space", &self.Space).field("uID", &self.uID).finish()
    }
}
impl ::core::default::Default for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_MIN_PRECISION_SUPPORT").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_SHADER_MIN_PRECISION_SUPPORT {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D12_SHADER_TYPE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D12_SHADER_TYPE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Class == other.Class && self.Type == other.Type && self.Rows == other.Rows && self.Columns == other.Columns && self.Elements == other.Elements && self.Members == other.Members && self.Offset == other.Offset && self.Name == other.Name
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D12_SHADER_TYPE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D12_SHADER_TYPE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SHADER_TYPE_DESC").field("Class", &self.Class).field("Type", &self.Type).field("Rows", &self.Rows).field("Columns", &self.Columns).field("Elements", &self.Elements).field("Members", &self.Members).field("Offset", &self.Offset).field("Name", &self.Name).finish()
    }
}
impl ::core::default::Default for D3D12_SHADER_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_SHADER_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.StartOffset == other.StartOffset && self.Size == other.Size && self.uFlags == other.uFlags && self.DefaultValue == other.DefaultValue && self.StartTexture == other.StartTexture && self.TextureSize == other.TextureSize && self.StartSampler == other.StartSampler && self.SamplerSize == other.SamplerSize
    }
}
impl ::core::cmp::Eq for D3D12_SHADER_VARIABLE_DESC {}
impl ::core::fmt::Debug for D3D12_SHADER_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SHADER_VARIABLE_DESC").field("Name", &self.Name).field("StartOffset", &self.StartOffset).field("Size", &self.Size).field("uFlags", &self.uFlags).field("DefaultValue", &self.DefaultValue).field("StartTexture", &self.StartTexture).field("TextureSize", &self.TextureSize).field("StartSampler", &self.StartSampler).field("SamplerSize", &self.SamplerSize).finish()
    }
}
impl ::core::default::Default for D3D12_SHADER_VERSION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SHADER_VERSION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_VERSION_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_SHADER_VISIBILITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SHADER_VISIBILITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SHADER_VISIBILITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_SHADING_RATE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SHADING_RATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SHADING_RATE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_SHADING_RATE_COMBINER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SHADING_RATE_COMBINER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SHADING_RATE_COMBINER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D12_SIGNATURE_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D12_SIGNATURE_PARAMETER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.SemanticName == other.SemanticName && self.SemanticIndex == other.SemanticIndex && self.Register == other.Register && self.SystemValueType == other.SystemValueType && self.ComponentType == other.ComponentType && self.Mask == other.Mask && self.ReadWriteMask == other.ReadWriteMask && self.Stream == other.Stream && self.MinPrecision == other.MinPrecision
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D12_SIGNATURE_PARAMETER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D12_SIGNATURE_PARAMETER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SIGNATURE_PARAMETER_DESC").field("SemanticName", &self.SemanticName).field("SemanticIndex", &self.SemanticIndex).field("Register", &self.Register).field("SystemValueType", &self.SystemValueType).field("ComponentType", &self.ComponentType).field("Mask", &self.Mask).field("ReadWriteMask", &self.ReadWriteMask).field("Stream", &self.Stream).field("MinPrecision", &self.MinPrecision).finish()
    }
}
impl ::core::default::Default for D3D12_SO_DECLARATION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_SO_DECLARATION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.Stream == other.Stream && self.SemanticName == other.SemanticName && self.SemanticIndex == other.SemanticIndex && self.StartComponent == other.StartComponent && self.ComponentCount == other.ComponentCount && self.OutputSlot == other.OutputSlot
    }
}
impl ::core::cmp::Eq for D3D12_SO_DECLARATION_ENTRY {}
impl ::core::fmt::Debug for D3D12_SO_DECLARATION_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SO_DECLARATION_ENTRY").field("Stream", &self.Stream).field("SemanticName", &self.SemanticName).field("SemanticIndex", &self.SemanticIndex).field("StartComponent", &self.StartComponent).field("ComponentCount", &self.ComponentCount).field("OutputSlot", &self.OutputSlot).finish()
    }
}
impl ::core::default::Default for D3D12_SRV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_SRV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_SRV_DIMENSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_STATE_OBJECT_CONFIG {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_STATE_OBJECT_CONFIG {
    fn eq(&self, other: &Self) -> bool {
        self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D12_STATE_OBJECT_CONFIG {}
impl ::core::fmt::Debug for D3D12_STATE_OBJECT_CONFIG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_STATE_OBJECT_CONFIG").field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D12_STATE_OBJECT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_STATE_OBJECT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.NumSubobjects == other.NumSubobjects && self.pSubobjects == other.pSubobjects
    }
}
impl ::core::cmp::Eq for D3D12_STATE_OBJECT_DESC {}
impl ::core::fmt::Debug for D3D12_STATE_OBJECT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_STATE_OBJECT_DESC").field("Type", &self.Type).field("NumSubobjects", &self.NumSubobjects).field("pSubobjects", &self.pSubobjects).finish()
    }
}
impl ::core::default::Default for D3D12_STATE_OBJECT_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_STATE_OBJECT_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_STATE_OBJECT_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_STATE_OBJECT_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_STATE_OBJECT_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_STATE_OBJECT_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_STATE_OBJECT_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_STATE_OBJECT_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_STATE_OBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_STATE_OBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_STATE_OBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_STATE_SUBOBJECT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_STATE_SUBOBJECT {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.pDesc == other.pDesc
    }
}
impl ::core::cmp::Eq for D3D12_STATE_SUBOBJECT {}
impl ::core::fmt::Debug for D3D12_STATE_SUBOBJECT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_STATE_SUBOBJECT").field("Type", &self.Type).field("pDesc", &self.pDesc).finish()
    }
}
impl ::core::default::Default for D3D12_STATE_SUBOBJECT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_STATE_SUBOBJECT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_STATE_SUBOBJECT_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_STATIC_BORDER_COLOR {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_STATIC_BORDER_COLOR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_STATIC_BORDER_COLOR").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_STATIC_SAMPLER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_STATIC_SAMPLER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Filter == other.Filter && self.AddressU == other.AddressU && self.AddressV == other.AddressV && self.AddressW == other.AddressW && self.MipLODBias == other.MipLODBias && self.MaxAnisotropy == other.MaxAnisotropy && self.ComparisonFunc == other.ComparisonFunc && self.BorderColor == other.BorderColor && self.MinLOD == other.MinLOD && self.MaxLOD == other.MaxLOD && self.ShaderRegister == other.ShaderRegister && self.RegisterSpace == other.RegisterSpace && self.ShaderVisibility == other.ShaderVisibility
    }
}
impl ::core::cmp::Eq for D3D12_STATIC_SAMPLER_DESC {}
impl ::core::fmt::Debug for D3D12_STATIC_SAMPLER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_STATIC_SAMPLER_DESC")
            .field("Filter", &self.Filter)
            .field("AddressU", &self.AddressU)
            .field("AddressV", &self.AddressV)
            .field("AddressW", &self.AddressW)
            .field("MipLODBias", &self.MipLODBias)
            .field("MaxAnisotropy", &self.MaxAnisotropy)
            .field("ComparisonFunc", &self.ComparisonFunc)
            .field("BorderColor", &self.BorderColor)
            .field("MinLOD", &self.MinLOD)
            .field("MaxLOD", &self.MaxLOD)
            .field("ShaderRegister", &self.ShaderRegister)
            .field("RegisterSpace", &self.RegisterSpace)
            .field("ShaderVisibility", &self.ShaderVisibility)
            .finish()
    }
}
impl ::core::default::Default for D3D12_STENCIL_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_STENCIL_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_STENCIL_OP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_STREAM_OUTPUT_BUFFER_VIEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_STREAM_OUTPUT_BUFFER_VIEW {
    fn eq(&self, other: &Self) -> bool {
        self.BufferLocation == other.BufferLocation && self.SizeInBytes == other.SizeInBytes && self.BufferFilledSizeLocation == other.BufferFilledSizeLocation
    }
}
impl ::core::cmp::Eq for D3D12_STREAM_OUTPUT_BUFFER_VIEW {}
impl ::core::fmt::Debug for D3D12_STREAM_OUTPUT_BUFFER_VIEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_STREAM_OUTPUT_BUFFER_VIEW").field("BufferLocation", &self.BufferLocation).field("SizeInBytes", &self.SizeInBytes).field("BufferFilledSizeLocation", &self.BufferFilledSizeLocation).finish()
    }
}
impl ::core::default::Default for D3D12_STREAM_OUTPUT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_STREAM_OUTPUT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.pSODeclaration == other.pSODeclaration && self.NumEntries == other.NumEntries && self.pBufferStrides == other.pBufferStrides && self.NumStrides == other.NumStrides && self.RasterizedStream == other.RasterizedStream
    }
}
impl ::core::cmp::Eq for D3D12_STREAM_OUTPUT_DESC {}
impl ::core::fmt::Debug for D3D12_STREAM_OUTPUT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_STREAM_OUTPUT_DESC").field("pSODeclaration", &self.pSODeclaration).field("NumEntries", &self.NumEntries).field("pBufferStrides", &self.pBufferStrides).field("NumStrides", &self.NumStrides).field("RasterizedStream", &self.RasterizedStream).finish()
    }
}
impl ::core::default::Default for D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    fn eq(&self, other: &Self) -> bool {
        self.pSubobjectToAssociate == other.pSubobjectToAssociate && self.NumExports == other.NumExports && self.pExports == other.pExports
    }
}
impl ::core::cmp::Eq for D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {}
impl ::core::fmt::Debug for D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SUBOBJECT_TO_EXPORTS_ASSOCIATION").field("pSubobjectToAssociate", &self.pSubobjectToAssociate).field("NumExports", &self.NumExports).field("pExports", &self.pExports).finish()
    }
}
impl ::core::default::Default for D3D12_SUBRESOURCE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_SUBRESOURCE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pData == other.pData && self.RowPitch == other.RowPitch && self.SlicePitch == other.SlicePitch
    }
}
impl ::core::cmp::Eq for D3D12_SUBRESOURCE_DATA {}
impl ::core::fmt::Debug for D3D12_SUBRESOURCE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SUBRESOURCE_DATA").field("pData", &self.pData).field("RowPitch", &self.RowPitch).field("SlicePitch", &self.SlicePitch).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_SUBRESOURCE_FOOTPRINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D12_SUBRESOURCE_FOOTPRINT {
    fn eq(&self, other: &Self) -> bool {
        self.Format == other.Format && self.Width == other.Width && self.Height == other.Height && self.Depth == other.Depth && self.RowPitch == other.RowPitch
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D12_SUBRESOURCE_FOOTPRINT {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D12_SUBRESOURCE_FOOTPRINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SUBRESOURCE_FOOTPRINT").field("Format", &self.Format).field("Width", &self.Width).field("Height", &self.Height).field("Depth", &self.Depth).field("RowPitch", &self.RowPitch).finish()
    }
}
impl ::core::default::Default for D3D12_SUBRESOURCE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_SUBRESOURCE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Offset == other.Offset && self.RowPitch == other.RowPitch && self.DepthPitch == other.DepthPitch
    }
}
impl ::core::cmp::Eq for D3D12_SUBRESOURCE_INFO {}
impl ::core::fmt::Debug for D3D12_SUBRESOURCE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SUBRESOURCE_INFO").field("Offset", &self.Offset).field("RowPitch", &self.RowPitch).field("DepthPitch", &self.DepthPitch).finish()
    }
}
impl ::core::default::Default for D3D12_SUBRESOURCE_RANGE_UINT64 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_SUBRESOURCE_RANGE_UINT64 {
    fn eq(&self, other: &Self) -> bool {
        self.Subresource == other.Subresource && self.Range == other.Range
    }
}
impl ::core::cmp::Eq for D3D12_SUBRESOURCE_RANGE_UINT64 {}
impl ::core::fmt::Debug for D3D12_SUBRESOURCE_RANGE_UINT64 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SUBRESOURCE_RANGE_UINT64").field("Subresource", &self.Subresource).field("Range", &self.Range).finish()
    }
}
impl ::core::default::Default for D3D12_SUBRESOURCE_TILING {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_SUBRESOURCE_TILING {
    fn eq(&self, other: &Self) -> bool {
        self.WidthInTiles == other.WidthInTiles && self.HeightInTiles == other.HeightInTiles && self.DepthInTiles == other.DepthInTiles && self.StartTileIndexInOverallResource == other.StartTileIndexInOverallResource
    }
}
impl ::core::cmp::Eq for D3D12_SUBRESOURCE_TILING {}
impl ::core::fmt::Debug for D3D12_SUBRESOURCE_TILING {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_SUBRESOURCE_TILING").field("WidthInTiles", &self.WidthInTiles).field("HeightInTiles", &self.HeightInTiles).field("DepthInTiles", &self.DepthInTiles).field("StartTileIndexInOverallResource", &self.StartTileIndexInOverallResource).finish()
    }
}
impl ::core::default::Default for D3D12_TEX1D_ARRAY_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX1D_ARRAY_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D12_TEX1D_ARRAY_DSV {}
impl ::core::fmt::Debug for D3D12_TEX1D_ARRAY_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX1D_ARRAY_DSV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D12_TEX1D_ARRAY_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX1D_ARRAY_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D12_TEX1D_ARRAY_RTV {}
impl ::core::fmt::Debug for D3D12_TEX1D_ARRAY_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX1D_ARRAY_RTV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D12_TEX1D_ARRAY_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX1D_ARRAY_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize && self.ResourceMinLODClamp == other.ResourceMinLODClamp
    }
}
impl ::core::cmp::Eq for D3D12_TEX1D_ARRAY_SRV {}
impl ::core::fmt::Debug for D3D12_TEX1D_ARRAY_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX1D_ARRAY_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).field("ResourceMinLODClamp", &self.ResourceMinLODClamp).finish()
    }
}
impl ::core::default::Default for D3D12_TEX1D_ARRAY_UAV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX1D_ARRAY_UAV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D12_TEX1D_ARRAY_UAV {}
impl ::core::fmt::Debug for D3D12_TEX1D_ARRAY_UAV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX1D_ARRAY_UAV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D12_TEX1D_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX1D_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D12_TEX1D_DSV {}
impl ::core::fmt::Debug for D3D12_TEX1D_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX1D_DSV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D12_TEX1D_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX1D_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D12_TEX1D_RTV {}
impl ::core::fmt::Debug for D3D12_TEX1D_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX1D_RTV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D12_TEX1D_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX1D_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.ResourceMinLODClamp == other.ResourceMinLODClamp
    }
}
impl ::core::cmp::Eq for D3D12_TEX1D_SRV {}
impl ::core::fmt::Debug for D3D12_TEX1D_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX1D_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("ResourceMinLODClamp", &self.ResourceMinLODClamp).finish()
    }
}
impl ::core::default::Default for D3D12_TEX1D_UAV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX1D_UAV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D12_TEX1D_UAV {}
impl ::core::fmt::Debug for D3D12_TEX1D_UAV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX1D_UAV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D12_TEX2DMS_ARRAY_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX2DMS_ARRAY_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D12_TEX2DMS_ARRAY_DSV {}
impl ::core::fmt::Debug for D3D12_TEX2DMS_ARRAY_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX2DMS_ARRAY_DSV").field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D12_TEX2DMS_ARRAY_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX2DMS_ARRAY_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D12_TEX2DMS_ARRAY_RTV {}
impl ::core::fmt::Debug for D3D12_TEX2DMS_ARRAY_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX2DMS_ARRAY_RTV").field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D12_TEX2DMS_ARRAY_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX2DMS_ARRAY_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D12_TEX2DMS_ARRAY_SRV {}
impl ::core::fmt::Debug for D3D12_TEX2DMS_ARRAY_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX2DMS_ARRAY_SRV").field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D12_TEX2DMS_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX2DMS_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.UnusedField_NothingToDefine == other.UnusedField_NothingToDefine
    }
}
impl ::core::cmp::Eq for D3D12_TEX2DMS_DSV {}
impl ::core::fmt::Debug for D3D12_TEX2DMS_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX2DMS_DSV").field("UnusedField_NothingToDefine", &self.UnusedField_NothingToDefine).finish()
    }
}
impl ::core::default::Default for D3D12_TEX2DMS_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX2DMS_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.UnusedField_NothingToDefine == other.UnusedField_NothingToDefine
    }
}
impl ::core::cmp::Eq for D3D12_TEX2DMS_RTV {}
impl ::core::fmt::Debug for D3D12_TEX2DMS_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX2DMS_RTV").field("UnusedField_NothingToDefine", &self.UnusedField_NothingToDefine).finish()
    }
}
impl ::core::default::Default for D3D12_TEX2DMS_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX2DMS_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.UnusedField_NothingToDefine == other.UnusedField_NothingToDefine
    }
}
impl ::core::cmp::Eq for D3D12_TEX2DMS_SRV {}
impl ::core::fmt::Debug for D3D12_TEX2DMS_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX2DMS_SRV").field("UnusedField_NothingToDefine", &self.UnusedField_NothingToDefine).finish()
    }
}
impl ::core::default::Default for D3D12_TEX2D_ARRAY_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX2D_ARRAY_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D12_TEX2D_ARRAY_DSV {}
impl ::core::fmt::Debug for D3D12_TEX2D_ARRAY_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX2D_ARRAY_DSV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D12_TEX2D_ARRAY_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX2D_ARRAY_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize && self.PlaneSlice == other.PlaneSlice
    }
}
impl ::core::cmp::Eq for D3D12_TEX2D_ARRAY_RTV {}
impl ::core::fmt::Debug for D3D12_TEX2D_ARRAY_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX2D_ARRAY_RTV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).field("PlaneSlice", &self.PlaneSlice).finish()
    }
}
impl ::core::default::Default for D3D12_TEX2D_ARRAY_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX2D_ARRAY_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize && self.PlaneSlice == other.PlaneSlice && self.ResourceMinLODClamp == other.ResourceMinLODClamp
    }
}
impl ::core::cmp::Eq for D3D12_TEX2D_ARRAY_SRV {}
impl ::core::fmt::Debug for D3D12_TEX2D_ARRAY_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX2D_ARRAY_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).field("PlaneSlice", &self.PlaneSlice).field("ResourceMinLODClamp", &self.ResourceMinLODClamp).finish()
    }
}
impl ::core::default::Default for D3D12_TEX2D_ARRAY_UAV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX2D_ARRAY_UAV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize && self.PlaneSlice == other.PlaneSlice
    }
}
impl ::core::cmp::Eq for D3D12_TEX2D_ARRAY_UAV {}
impl ::core::fmt::Debug for D3D12_TEX2D_ARRAY_UAV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX2D_ARRAY_UAV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).field("PlaneSlice", &self.PlaneSlice).finish()
    }
}
impl ::core::default::Default for D3D12_TEX2D_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX2D_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D12_TEX2D_DSV {}
impl ::core::fmt::Debug for D3D12_TEX2D_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX2D_DSV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D12_TEX2D_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX2D_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.PlaneSlice == other.PlaneSlice
    }
}
impl ::core::cmp::Eq for D3D12_TEX2D_RTV {}
impl ::core::fmt::Debug for D3D12_TEX2D_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX2D_RTV").field("MipSlice", &self.MipSlice).field("PlaneSlice", &self.PlaneSlice).finish()
    }
}
impl ::core::default::Default for D3D12_TEX2D_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX2D_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.PlaneSlice == other.PlaneSlice && self.ResourceMinLODClamp == other.ResourceMinLODClamp
    }
}
impl ::core::cmp::Eq for D3D12_TEX2D_SRV {}
impl ::core::fmt::Debug for D3D12_TEX2D_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX2D_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("PlaneSlice", &self.PlaneSlice).field("ResourceMinLODClamp", &self.ResourceMinLODClamp).finish()
    }
}
impl ::core::default::Default for D3D12_TEX2D_UAV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX2D_UAV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.PlaneSlice == other.PlaneSlice
    }
}
impl ::core::cmp::Eq for D3D12_TEX2D_UAV {}
impl ::core::fmt::Debug for D3D12_TEX2D_UAV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX2D_UAV").field("MipSlice", &self.MipSlice).field("PlaneSlice", &self.PlaneSlice).finish()
    }
}
impl ::core::default::Default for D3D12_TEX3D_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX3D_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstWSlice == other.FirstWSlice && self.WSize == other.WSize
    }
}
impl ::core::cmp::Eq for D3D12_TEX3D_RTV {}
impl ::core::fmt::Debug for D3D12_TEX3D_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX3D_RTV").field("MipSlice", &self.MipSlice).field("FirstWSlice", &self.FirstWSlice).field("WSize", &self.WSize).finish()
    }
}
impl ::core::default::Default for D3D12_TEX3D_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX3D_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.ResourceMinLODClamp == other.ResourceMinLODClamp
    }
}
impl ::core::cmp::Eq for D3D12_TEX3D_SRV {}
impl ::core::fmt::Debug for D3D12_TEX3D_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX3D_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("ResourceMinLODClamp", &self.ResourceMinLODClamp).finish()
    }
}
impl ::core::default::Default for D3D12_TEX3D_UAV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEX3D_UAV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstWSlice == other.FirstWSlice && self.WSize == other.WSize
    }
}
impl ::core::cmp::Eq for D3D12_TEX3D_UAV {}
impl ::core::fmt::Debug for D3D12_TEX3D_UAV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEX3D_UAV").field("MipSlice", &self.MipSlice).field("FirstWSlice", &self.FirstWSlice).field("WSize", &self.WSize).finish()
    }
}
impl ::core::default::Default for D3D12_TEXCUBE_ARRAY_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEXCUBE_ARRAY_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.First2DArrayFace == other.First2DArrayFace && self.NumCubes == other.NumCubes && self.ResourceMinLODClamp == other.ResourceMinLODClamp
    }
}
impl ::core::cmp::Eq for D3D12_TEXCUBE_ARRAY_SRV {}
impl ::core::fmt::Debug for D3D12_TEXCUBE_ARRAY_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEXCUBE_ARRAY_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("First2DArrayFace", &self.First2DArrayFace).field("NumCubes", &self.NumCubes).field("ResourceMinLODClamp", &self.ResourceMinLODClamp).finish()
    }
}
impl ::core::default::Default for D3D12_TEXCUBE_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TEXCUBE_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.ResourceMinLODClamp == other.ResourceMinLODClamp
    }
}
impl ::core::cmp::Eq for D3D12_TEXCUBE_SRV {}
impl ::core::fmt::Debug for D3D12_TEXCUBE_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TEXCUBE_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("ResourceMinLODClamp", &self.ResourceMinLODClamp).finish()
    }
}
impl ::core::default::Default for D3D12_TEXTURE_ADDRESS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_TEXTURE_ADDRESS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_TEXTURE_ADDRESS_MODE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_TEXTURE_COPY_LOCATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_TEXTURE_COPY_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_TEXTURE_COPY_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_TEXTURE_COPY_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_TEXTURE_LAYOUT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_TEXTURE_LAYOUT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_TEXTURE_LAYOUT").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_TILED_RESOURCES_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_TILED_RESOURCES_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_TILED_RESOURCES_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_TILED_RESOURCE_COORDINATE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TILED_RESOURCE_COORDINATE {
    fn eq(&self, other: &Self) -> bool {
        self.X == other.X && self.Y == other.Y && self.Z == other.Z && self.Subresource == other.Subresource
    }
}
impl ::core::cmp::Eq for D3D12_TILED_RESOURCE_COORDINATE {}
impl ::core::fmt::Debug for D3D12_TILED_RESOURCE_COORDINATE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TILED_RESOURCE_COORDINATE").field("X", &self.X).field("Y", &self.Y).field("Z", &self.Z).field("Subresource", &self.Subresource).finish()
    }
}
impl ::core::default::Default for D3D12_TILE_COPY_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_TILE_COPY_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_TILE_COPY_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_TILE_COPY_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_TILE_COPY_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_TILE_COPY_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_TILE_COPY_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_TILE_COPY_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_TILE_MAPPING_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_TILE_MAPPING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_TILE_MAPPING_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_TILE_MAPPING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_TILE_MAPPING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_TILE_MAPPING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_TILE_MAPPING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_TILE_MAPPING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_TILE_RANGE_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_TILE_RANGE_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_TILE_RANGE_FLAGS").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D12_TILE_REGION_SIZE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D12_TILE_REGION_SIZE {
    fn eq(&self, other: &Self) -> bool {
        self.NumTiles == other.NumTiles && self.UseBox == other.UseBox && self.Width == other.Width && self.Height == other.Height && self.Depth == other.Depth
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D12_TILE_REGION_SIZE {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D12_TILE_REGION_SIZE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TILE_REGION_SIZE").field("NumTiles", &self.NumTiles).field("UseBox", &self.UseBox).field("Width", &self.Width).field("Height", &self.Height).field("Depth", &self.Depth).finish()
    }
}
impl ::core::default::Default for D3D12_TILE_SHAPE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_TILE_SHAPE {
    fn eq(&self, other: &Self) -> bool {
        self.WidthInTexels == other.WidthInTexels && self.HeightInTexels == other.HeightInTexels && self.DepthInTexels == other.DepthInTexels
    }
}
impl ::core::cmp::Eq for D3D12_TILE_SHAPE {}
impl ::core::fmt::Debug for D3D12_TILE_SHAPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_TILE_SHAPE").field("WidthInTexels", &self.WidthInTexels).field("HeightInTexels", &self.HeightInTexels).field("DepthInTexels", &self.DepthInTexels).finish()
    }
}
impl ::core::default::Default for D3D12_UAV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_UAV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_UAV_DIMENSION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D12_UNORDERED_ACCESS_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VARIABLE_SHADING_RATE_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VARIABLE_SHADING_RATE_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VARIABLE_SHADING_RATE_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_VERSIONED_DEVICE_REMOVED_EXTENDED_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VERSIONED_ROOT_SIGNATURE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D12_VERTEX_BUFFER_VIEW {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VERTEX_BUFFER_VIEW {
    fn eq(&self, other: &Self) -> bool {
        self.BufferLocation == other.BufferLocation && self.SizeInBytes == other.SizeInBytes && self.StrideInBytes == other.StrideInBytes
    }
}
impl ::core::cmp::Eq for D3D12_VERTEX_BUFFER_VIEW {}
impl ::core::fmt::Debug for D3D12_VERTEX_BUFFER_VIEW {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VERTEX_BUFFER_VIEW").field("BufferLocation", &self.BufferLocation).field("SizeInBytes", &self.SizeInBytes).field("StrideInBytes", &self.StrideInBytes).finish()
    }
}
impl ::core::default::Default for D3D12_VIEWPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIEWPORT {
    fn eq(&self, other: &Self) -> bool {
        self.TopLeftX == other.TopLeftX && self.TopLeftY == other.TopLeftY && self.Width == other.Width && self.Height == other.Height && self.MinDepth == other.MinDepth && self.MaxDepth == other.MaxDepth
    }
}
impl ::core::cmp::Eq for D3D12_VIEWPORT {}
impl ::core::fmt::Debug for D3D12_VIEWPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIEWPORT").field("TopLeftX", &self.TopLeftX).field("TopLeftY", &self.TopLeftY).field("Width", &self.Width).field("Height", &self.Height).field("MinDepth", &self.MinDepth).field("MaxDepth", &self.MaxDepth).finish()
    }
}
impl ::core::default::Default for D3D12_VIEW_INSTANCE_LOCATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIEW_INSTANCE_LOCATION {
    fn eq(&self, other: &Self) -> bool {
        self.ViewportArrayIndex == other.ViewportArrayIndex && self.RenderTargetArrayIndex == other.RenderTargetArrayIndex
    }
}
impl ::core::cmp::Eq for D3D12_VIEW_INSTANCE_LOCATION {}
impl ::core::fmt::Debug for D3D12_VIEW_INSTANCE_LOCATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIEW_INSTANCE_LOCATION").field("ViewportArrayIndex", &self.ViewportArrayIndex).field("RenderTargetArrayIndex", &self.RenderTargetArrayIndex).finish()
    }
}
impl ::core::default::Default for D3D12_VIEW_INSTANCING_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_VIEW_INSTANCING_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ViewInstanceCount == other.ViewInstanceCount && self.pViewInstanceLocations == other.pViewInstanceLocations && self.Flags == other.Flags
    }
}
impl ::core::cmp::Eq for D3D12_VIEW_INSTANCING_DESC {}
impl ::core::fmt::Debug for D3D12_VIEW_INSTANCING_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_VIEW_INSTANCING_DESC").field("ViewInstanceCount", &self.ViewInstanceCount).field("pViewInstanceLocations", &self.pViewInstanceLocations).field("Flags", &self.Flags).finish()
    }
}
impl ::core::default::Default for D3D12_VIEW_INSTANCING_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIEW_INSTANCING_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIEW_INSTANCING_FLAGS").field(&self.0).finish()
    }
}
impl ::core::ops::BitOr for D3D12_VIEW_INSTANCING_FLAGS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for D3D12_VIEW_INSTANCING_FLAGS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for D3D12_VIEW_INSTANCING_FLAGS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for D3D12_VIEW_INSTANCING_FLAGS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for D3D12_VIEW_INSTANCING_FLAGS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
impl ::core::default::Default for D3D12_VIEW_INSTANCING_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_VIEW_INSTANCING_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_VIEW_INSTANCING_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_WAVE_MMA_TIER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_WAVE_MMA_TIER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_WAVE_MMA_TIER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_WRITEBUFFERIMMEDIATE_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D12_WRITEBUFFERIMMEDIATE_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D12_WRITEBUFFERIMMEDIATE_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D12_WRITEBUFFERIMMEDIATE_PARAMETER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D12_WRITEBUFFERIMMEDIATE_PARAMETER {
    fn eq(&self, other: &Self) -> bool {
        self.Dest == other.Dest && self.Value == other.Value
    }
}
impl ::core::cmp::Eq for D3D12_WRITEBUFFERIMMEDIATE_PARAMETER {}
impl ::core::fmt::Debug for D3D12_WRITEBUFFERIMMEDIATE_PARAMETER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D12_WRITEBUFFERIMMEDIATE_PARAMETER").field("Dest", &self.Dest).field("Value", &self.Value).finish()
    }
}
impl ::core::default::Default for D3D_ROOT_SIGNATURE_VERSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_ROOT_SIGNATURE_VERSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_ROOT_SIGNATURE_VERSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D_SHADER_MODEL {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D_SHADER_MODEL {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D_SHADER_MODEL").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12CommandAllocator {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12CommandAllocator {}
impl ::core::fmt::Debug for ID3D12CommandAllocator {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12CommandAllocator").field(&self.0).finish()
    }
}
impl ID3D12CommandAllocator {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12CommandList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12CommandList {}
impl ::core::fmt::Debug for ID3D12CommandList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12CommandList").field(&self.0).finish()
    }
}
impl ID3D12CommandList {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12CommandQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12CommandQueue {}
impl ::core::fmt::Debug for ID3D12CommandQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12CommandQueue").field(&self.0).finish()
    }
}
impl ID3D12CommandQueue {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12CommandSignature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12CommandSignature {}
impl ::core::fmt::Debug for ID3D12CommandSignature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12CommandSignature").field(&self.0).finish()
    }
}
impl ID3D12CommandSignature {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12Debug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Debug {}
impl ::core::fmt::Debug for ID3D12Debug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Debug").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12Debug1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Debug1 {}
impl ::core::fmt::Debug for ID3D12Debug1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Debug1").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12Debug2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Debug2 {}
impl ::core::fmt::Debug for ID3D12Debug2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Debug2").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12Debug3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Debug3 {}
impl ::core::fmt::Debug for ID3D12Debug3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Debug3").field(&self.0).finish()
    }
}
impl ID3D12Debug3 {
    pub unsafe fn EnableDebugLayer(&self) {
        (::windows::core::Vtable::vtable(self).base__.EnableDebugLayer)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D12Debug4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Debug4 {}
impl ::core::fmt::Debug for ID3D12Debug4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Debug4").field(&self.0).finish()
    }
}
impl ID3D12Debug4 {
    pub unsafe fn EnableDebugLayer(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.EnableDebugLayer)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableGPUBasedValidation<P0>(&self, enable: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEnableGPUBasedValidation)(::windows::core::Vtable::as_raw(self), enable.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableSynchronizedCommandQueueValidation<P0>(&self, enable: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEnableSynchronizedCommandQueueValidation)(::windows::core::Vtable::as_raw(self), enable.into())
    }
    pub unsafe fn SetGPUBasedValidationFlags(&self, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
        (::windows::core::Vtable::vtable(self).base__.SetGPUBasedValidationFlags)(::windows::core::Vtable::as_raw(self), flags)
    }
}
impl ::core::cmp::PartialEq for ID3D12Debug5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Debug5 {}
impl ::core::fmt::Debug for ID3D12Debug5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Debug5").field(&self.0).finish()
    }
}
impl ID3D12Debug5 {
    pub unsafe fn EnableDebugLayer(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnableDebugLayer)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableGPUBasedValidation<P0>(&self, enable: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetEnableGPUBasedValidation)(::windows::core::Vtable::as_raw(self), enable.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEnableSynchronizedCommandQueueValidation<P0>(&self, enable: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetEnableSynchronizedCommandQueueValidation)(::windows::core::Vtable::as_raw(self), enable.into())
    }
    pub unsafe fn SetGPUBasedValidationFlags(&self, flags: D3D12_GPU_BASED_VALIDATION_FLAGS) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGPUBasedValidationFlags)(::windows::core::Vtable::as_raw(self), flags)
    }
    pub unsafe fn DisableDebugLayer(&self) {
        (::windows::core::Vtable::vtable(self).base__.DisableDebugLayer)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D12DebugCommandList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12DebugCommandList {}
impl ::core::fmt::Debug for ID3D12DebugCommandList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12DebugCommandList").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12DebugCommandList1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12DebugCommandList1 {}
impl ::core::fmt::Debug for ID3D12DebugCommandList1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12DebugCommandList1").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12DebugCommandList2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12DebugCommandList2 {}
impl ::core::fmt::Debug for ID3D12DebugCommandList2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12DebugCommandList2").field(&self.0).finish()
    }
}
impl ID3D12DebugCommandList2 {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn AssertResourceState<P0>(&self, presource: P0, subresource: u32, state: u32) -> super::super::Foundation::BOOL
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AssertResourceState)(::windows::core::Vtable::as_raw(self), presource.into().abi(), subresource, state)
    }
    pub unsafe fn SetFeatureMask(&self, mask: D3D12_DEBUG_FEATURE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFeatureMask)(::windows::core::Vtable::as_raw(self), mask).ok()
    }
    pub unsafe fn GetFeatureMask(&self) -> D3D12_DEBUG_FEATURE {
        (::windows::core::Vtable::vtable(self).base__.GetFeatureMask)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D12DebugCommandQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12DebugCommandQueue {}
impl ::core::fmt::Debug for ID3D12DebugCommandQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12DebugCommandQueue").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12DebugDevice {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12DebugDevice {}
impl ::core::fmt::Debug for ID3D12DebugDevice {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12DebugDevice").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12DebugDevice1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12DebugDevice1 {}
impl ::core::fmt::Debug for ID3D12DebugDevice1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12DebugDevice1").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12DebugDevice2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12DebugDevice2 {}
impl ::core::fmt::Debug for ID3D12DebugDevice2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12DebugDevice2").field(&self.0).finish()
    }
}
impl ID3D12DebugDevice2 {
    pub unsafe fn SetFeatureMask(&self, mask: D3D12_DEBUG_FEATURE) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetFeatureMask)(::windows::core::Vtable::as_raw(self), mask).ok()
    }
    pub unsafe fn GetFeatureMask(&self) -> D3D12_DEBUG_FEATURE {
        (::windows::core::Vtable::vtable(self).base__.GetFeatureMask)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ReportLiveDeviceObjects(&self, flags: D3D12_RLDO_FLAGS) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReportLiveDeviceObjects)(::windows::core::Vtable::as_raw(self), flags).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12DescriptorHeap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12DescriptorHeap {}
impl ::core::fmt::Debug for ID3D12DescriptorHeap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12DescriptorHeap").field(&self.0).finish()
    }
}
impl ID3D12DescriptorHeap {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12Device {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Device {}
impl ::core::fmt::Debug for ID3D12Device {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Device").field(&self.0).finish()
    }
}
impl ID3D12Device {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12Device1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Device1 {}
impl ::core::fmt::Debug for ID3D12Device1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Device1").field(&self.0).finish()
    }
}
impl ID3D12Device1 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetNodeCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetNodeCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CreateCommandQueue<T>(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCommandQueue)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandAllocator<T>(&self, r#type: D3D12_COMMAND_LIST_TYPE) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCommandAllocator)(::windows::core::Vtable::as_raw(self), r#type, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateGraphicsPipelineState<T>(&self, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateGraphicsPipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateComputePipelineState<T>(&self, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateComputePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList<P0, P1, T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: P0, pinitialstate: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), nodemask, r#type, pcommandallocator.into().abi(), pinitialstate.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn CreateDescriptorHeap<T>(&self, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateDescriptorHeap)(::windows::core::Vtable::as_raw(self), pdescriptorheapdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescriptorHandleIncrementSize(&self, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetDescriptorHandleIncrementSize)(::windows::core::Vtable::as_raw(self), descriptorheaptype)
    }
    pub unsafe fn CreateRootSignature<T>(&self, nodemask: u32, pblobwithrootsignature: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateRootSignature)(::windows::core::Vtable::as_raw(self), nodemask, ::core::mem::transmute(pblobwithrootsignature.as_ptr()), pblobwithrootsignature.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateConstantBufferView(&self, pdesc: ::core::option::Option<*const D3D12_CONSTANT_BUFFER_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.CreateConstantBufferView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_SHADER_RESOURCE_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0, P1>(&self, presource: P0, pcounterresource: P1, pdesc: ::core::option::Option<*const D3D12_UNORDERED_ACCESS_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), pcounterresource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_RENDER_TARGET_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_DEPTH_STENCIL_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CreateSampler(&self, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.CreateSampler)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CopyDescriptors(&self, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: ::core::option::Option<*const u32>, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: ::core::option::Option<*const u32>, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.CopyDescriptors)(::windows::core::Vtable::as_raw(self), numdestdescriptorranges, pdestdescriptorrangestarts, ::core::mem::transmute(pdestdescriptorrangesizes.unwrap_or(::std::ptr::null())), numsrcdescriptorranges, psrcdescriptorrangestarts, ::core::mem::transmute(psrcdescriptorrangesizes.unwrap_or(::std::ptr::null())), descriptorheapstype)
    }
    pub unsafe fn CopyDescriptorsSimple(&self, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.CopyDescriptorsSimple)(::windows::core::Vtable::as_raw(self), numdescriptors, ::core::mem::transmute(destdescriptorrangestart), ::core::mem::transmute(srcdescriptorrangestart), descriptorheapstype)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo(&self, visiblemask: u32, presourcedescs: &[D3D12_RESOURCE_DESC]) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResourceAllocationInfo)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, presourcedescs.len() as _, ::core::mem::transmute(presourcedescs.as_ptr()));
        result__
    }
    pub unsafe fn GetCustomHeapProperties(&self, nodemask: u32, heaptype: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES {
        let mut result__: D3D12_HEAP_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetCustomHeapProperties)(::windows::core::Vtable::as_raw(self), &mut result__, nodemask, heaptype);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource<T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateCommittedResource)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap<T>(&self, pdesc: *const D3D12_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Heap>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.CreatePlacedResource)(::windows::core::Vtable::as_raw(self), pheap.into().abi(), heapoffset, pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource<T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateReservedResource)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<P0, P1>(&self, pobject: P0, pattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, access: u32, name: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12DeviceChild>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateSharedHandle)(::windows::core::Vtable::as_raw(self), pobject.into().abi(), ::core::mem::transmute(pattributes.unwrap_or(::std::ptr::null())), access, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandle<P0, T>(&self, nthandle: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.OpenSharedHandle)(::windows::core::Vtable::as_raw(self), nthandle.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandleByName<P0>(&self, name: P0, access: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OpenSharedHandleByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), access, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MakeResident(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.MakeResident)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn Evict(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Evict)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn CreateFence<T>(&self, initialvalue: u64, flags: D3D12_FENCE_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateFence)(::windows::core::Vtable::as_raw(self), initialvalue, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCopyableFootprints(&self, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: ::core::option::Option<*mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT>, pnumrows: ::core::option::Option<*mut u32>, prowsizeinbytes: ::core::option::Option<*mut u64>, ptotalbytes: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.GetCopyableFootprints)(::windows::core::Vtable::as_raw(self), presourcedesc, firstsubresource, numsubresources, baseoffset, ::core::mem::transmute(playouts.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumrows.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(prowsizeinbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ptotalbytes.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CreateQueryHeap<T>(&self, pdesc: *const D3D12_QUERY_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateQueryHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStablePowerState<P0>(&self, enable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetStablePowerState)(::windows::core::Vtable::as_raw(self), enable.into()).ok()
    }
    pub unsafe fn CreateCommandSignature<P0, T>(&self, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateCommandSignature)(::windows::core::Vtable::as_raw(self), pdesc, prootsignature.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: ::core::option::Option<*mut u32>, ppackedmipdesc: ::core::option::Option<*mut D3D12_PACKED_MIP_INFO>, pstandardtileshapefornonpackedmips: ::core::option::Option<*mut D3D12_TILE_SHAPE>, pnumsubresourcetilings: ::core::option::Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetResourceTiling)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ::core::mem::transmute(pnumtilesforentireresource.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppackedmipdesc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumsubresourcetilings.unwrap_or(::std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterLuid(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAdapterLuid)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
impl ::core::cmp::PartialEq for ID3D12Device2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Device2 {}
impl ::core::fmt::Debug for ID3D12Device2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Device2").field(&self.0).finish()
    }
}
impl ID3D12Device2 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetNodeCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetNodeCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CreateCommandQueue<T>(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCommandQueue)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandAllocator<T>(&self, r#type: D3D12_COMMAND_LIST_TYPE) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCommandAllocator)(::windows::core::Vtable::as_raw(self), r#type, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateGraphicsPipelineState<T>(&self, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateGraphicsPipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateComputePipelineState<T>(&self, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateComputePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList<P0, P1, T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: P0, pinitialstate: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), nodemask, r#type, pcommandallocator.into().abi(), pinitialstate.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn CreateDescriptorHeap<T>(&self, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDescriptorHeap)(::windows::core::Vtable::as_raw(self), pdescriptorheapdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescriptorHandleIncrementSize(&self, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDescriptorHandleIncrementSize)(::windows::core::Vtable::as_raw(self), descriptorheaptype)
    }
    pub unsafe fn CreateRootSignature<T>(&self, nodemask: u32, pblobwithrootsignature: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateRootSignature)(::windows::core::Vtable::as_raw(self), nodemask, ::core::mem::transmute(pblobwithrootsignature.as_ptr()), pblobwithrootsignature.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateConstantBufferView(&self, pdesc: ::core::option::Option<*const D3D12_CONSTANT_BUFFER_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateConstantBufferView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_SHADER_RESOURCE_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0, P1>(&self, presource: P0, pcounterresource: P1, pdesc: ::core::option::Option<*const D3D12_UNORDERED_ACCESS_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), pcounterresource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_RENDER_TARGET_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_DEPTH_STENCIL_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CreateSampler(&self, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateSampler)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CopyDescriptors(&self, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: ::core::option::Option<*const u32>, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: ::core::option::Option<*const u32>, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyDescriptors)(::windows::core::Vtable::as_raw(self), numdestdescriptorranges, pdestdescriptorrangestarts, ::core::mem::transmute(pdestdescriptorrangesizes.unwrap_or(::std::ptr::null())), numsrcdescriptorranges, psrcdescriptorrangestarts, ::core::mem::transmute(psrcdescriptorrangesizes.unwrap_or(::std::ptr::null())), descriptorheapstype)
    }
    pub unsafe fn CopyDescriptorsSimple(&self, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyDescriptorsSimple)(::windows::core::Vtable::as_raw(self), numdescriptors, ::core::mem::transmute(destdescriptorrangestart), ::core::mem::transmute(srcdescriptorrangestart), descriptorheapstype)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo(&self, visiblemask: u32, presourcedescs: &[D3D12_RESOURCE_DESC]) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetResourceAllocationInfo)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, presourcedescs.len() as _, ::core::mem::transmute(presourcedescs.as_ptr()));
        result__
    }
    pub unsafe fn GetCustomHeapProperties(&self, nodemask: u32, heaptype: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES {
        let mut result__: D3D12_HEAP_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetCustomHeapProperties)(::windows::core::Vtable::as_raw(self), &mut result__, nodemask, heaptype);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource<T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCommittedResource)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap<T>(&self, pdesc: *const D3D12_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Heap>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreatePlacedResource)(::windows::core::Vtable::as_raw(self), pheap.into().abi(), heapoffset, pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource<T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateReservedResource)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<P0, P1>(&self, pobject: P0, pattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, access: u32, name: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12DeviceChild>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateSharedHandle)(::windows::core::Vtable::as_raw(self), pobject.into().abi(), ::core::mem::transmute(pattributes.unwrap_or(::std::ptr::null())), access, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandle<P0, T>(&self, nthandle: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OpenSharedHandle)(::windows::core::Vtable::as_raw(self), nthandle.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandleByName<P0>(&self, name: P0, access: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.OpenSharedHandleByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), access, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MakeResident(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.MakeResident)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn Evict(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Evict)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn CreateFence<T>(&self, initialvalue: u64, flags: D3D12_FENCE_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateFence)(::windows::core::Vtable::as_raw(self), initialvalue, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCopyableFootprints(&self, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: ::core::option::Option<*mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT>, pnumrows: ::core::option::Option<*mut u32>, prowsizeinbytes: ::core::option::Option<*mut u64>, ptotalbytes: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.base__.GetCopyableFootprints)(::windows::core::Vtable::as_raw(self), presourcedesc, firstsubresource, numsubresources, baseoffset, ::core::mem::transmute(playouts.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumrows.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(prowsizeinbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ptotalbytes.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CreateQueryHeap<T>(&self, pdesc: *const D3D12_QUERY_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateQueryHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStablePowerState<P0>(&self, enable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetStablePowerState)(::windows::core::Vtable::as_raw(self), enable.into()).ok()
    }
    pub unsafe fn CreateCommandSignature<P0, T>(&self, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCommandSignature)(::windows::core::Vtable::as_raw(self), pdesc, prootsignature.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: ::core::option::Option<*mut u32>, ppackedmipdesc: ::core::option::Option<*mut D3D12_PACKED_MIP_INFO>, pstandardtileshapefornonpackedmips: ::core::option::Option<*mut D3D12_TILE_SHAPE>, pnumsubresourcetilings: ::core::option::Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetResourceTiling)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ::core::mem::transmute(pnumtilesforentireresource.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppackedmipdesc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumsubresourcetilings.unwrap_or(::std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterLuid(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetAdapterLuid)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn CreatePipelineLibrary<T>(&self, plibraryblob: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePipelineLibrary)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plibraryblob.as_ptr()), plibraryblob.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventOnMultipleFenceCompletion<P0>(&self, ppfences: *const ::core::option::Option<ID3D12Fence>, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEventOnMultipleFenceCompletion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppfences), pfencevalues, numfences, flags, hevent.into()).ok()
    }
    pub unsafe fn SetResidencyPriority(&self, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetResidencyPriority)(::windows::core::Vtable::as_raw(self), numobjects, ::core::mem::transmute(ppobjects), ppriorities).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12Device3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Device3 {}
impl ::core::fmt::Debug for ID3D12Device3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Device3").field(&self.0).finish()
    }
}
impl ID3D12Device3 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetNodeCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetNodeCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CreateCommandQueue<T>(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCommandQueue)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandAllocator<T>(&self, r#type: D3D12_COMMAND_LIST_TYPE) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCommandAllocator)(::windows::core::Vtable::as_raw(self), r#type, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateGraphicsPipelineState<T>(&self, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateGraphicsPipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateComputePipelineState<T>(&self, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateComputePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList<P0, P1, T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: P0, pinitialstate: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), nodemask, r#type, pcommandallocator.into().abi(), pinitialstate.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn CreateDescriptorHeap<T>(&self, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDescriptorHeap)(::windows::core::Vtable::as_raw(self), pdescriptorheapdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescriptorHandleIncrementSize(&self, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDescriptorHandleIncrementSize)(::windows::core::Vtable::as_raw(self), descriptorheaptype)
    }
    pub unsafe fn CreateRootSignature<T>(&self, nodemask: u32, pblobwithrootsignature: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateRootSignature)(::windows::core::Vtable::as_raw(self), nodemask, ::core::mem::transmute(pblobwithrootsignature.as_ptr()), pblobwithrootsignature.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateConstantBufferView(&self, pdesc: ::core::option::Option<*const D3D12_CONSTANT_BUFFER_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateConstantBufferView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_SHADER_RESOURCE_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0, P1>(&self, presource: P0, pcounterresource: P1, pdesc: ::core::option::Option<*const D3D12_UNORDERED_ACCESS_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), pcounterresource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_RENDER_TARGET_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_DEPTH_STENCIL_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CreateSampler(&self, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateSampler)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CopyDescriptors(&self, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: ::core::option::Option<*const u32>, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: ::core::option::Option<*const u32>, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CopyDescriptors)(::windows::core::Vtable::as_raw(self), numdestdescriptorranges, pdestdescriptorrangestarts, ::core::mem::transmute(pdestdescriptorrangesizes.unwrap_or(::std::ptr::null())), numsrcdescriptorranges, psrcdescriptorrangestarts, ::core::mem::transmute(psrcdescriptorrangesizes.unwrap_or(::std::ptr::null())), descriptorheapstype)
    }
    pub unsafe fn CopyDescriptorsSimple(&self, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CopyDescriptorsSimple)(::windows::core::Vtable::as_raw(self), numdescriptors, ::core::mem::transmute(destdescriptorrangestart), ::core::mem::transmute(srcdescriptorrangestart), descriptorheapstype)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo(&self, visiblemask: u32, presourcedescs: &[D3D12_RESOURCE_DESC]) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetResourceAllocationInfo)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, presourcedescs.len() as _, ::core::mem::transmute(presourcedescs.as_ptr()));
        result__
    }
    pub unsafe fn GetCustomHeapProperties(&self, nodemask: u32, heaptype: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES {
        let mut result__: D3D12_HEAP_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCustomHeapProperties)(::windows::core::Vtable::as_raw(self), &mut result__, nodemask, heaptype);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource<T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCommittedResource)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap<T>(&self, pdesc: *const D3D12_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Heap>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreatePlacedResource)(::windows::core::Vtable::as_raw(self), pheap.into().abi(), heapoffset, pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource<T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateReservedResource)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<P0, P1>(&self, pobject: P0, pattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, access: u32, name: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12DeviceChild>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateSharedHandle)(::windows::core::Vtable::as_raw(self), pobject.into().abi(), ::core::mem::transmute(pattributes.unwrap_or(::std::ptr::null())), access, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandle<P0, T>(&self, nthandle: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OpenSharedHandle)(::windows::core::Vtable::as_raw(self), nthandle.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandleByName<P0>(&self, name: P0, access: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OpenSharedHandleByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), access, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MakeResident(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.MakeResident)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn Evict(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Evict)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn CreateFence<T>(&self, initialvalue: u64, flags: D3D12_FENCE_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateFence)(::windows::core::Vtable::as_raw(self), initialvalue, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCopyableFootprints(&self, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: ::core::option::Option<*mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT>, pnumrows: ::core::option::Option<*mut u32>, prowsizeinbytes: ::core::option::Option<*mut u64>, ptotalbytes: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetCopyableFootprints)(::windows::core::Vtable::as_raw(self), presourcedesc, firstsubresource, numsubresources, baseoffset, ::core::mem::transmute(playouts.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumrows.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(prowsizeinbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ptotalbytes.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CreateQueryHeap<T>(&self, pdesc: *const D3D12_QUERY_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateQueryHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStablePowerState<P0>(&self, enable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetStablePowerState)(::windows::core::Vtable::as_raw(self), enable.into()).ok()
    }
    pub unsafe fn CreateCommandSignature<P0, T>(&self, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCommandSignature)(::windows::core::Vtable::as_raw(self), pdesc, prootsignature.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: ::core::option::Option<*mut u32>, ppackedmipdesc: ::core::option::Option<*mut D3D12_PACKED_MIP_INFO>, pstandardtileshapefornonpackedmips: ::core::option::Option<*mut D3D12_TILE_SHAPE>, pnumsubresourcetilings: ::core::option::Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetResourceTiling)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ::core::mem::transmute(pnumtilesforentireresource.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppackedmipdesc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumsubresourcetilings.unwrap_or(::std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterLuid(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetAdapterLuid)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn CreatePipelineLibrary<T>(&self, plibraryblob: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreatePipelineLibrary)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plibraryblob.as_ptr()), plibraryblob.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventOnMultipleFenceCompletion<P0>(&self, ppfences: *const ::core::option::Option<ID3D12Fence>, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetEventOnMultipleFenceCompletion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppfences), pfencevalues, numfences, flags, hevent.into()).ok()
    }
    pub unsafe fn SetResidencyPriority(&self, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetResidencyPriority)(::windows::core::Vtable::as_raw(self), numobjects, ::core::mem::transmute(ppobjects), ppriorities).ok()
    }
    pub unsafe fn CreatePipelineState<T>(&self, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreatePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID3D12Device4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Device4 {}
impl ::core::fmt::Debug for ID3D12Device4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Device4").field(&self.0).finish()
    }
}
impl ID3D12Device4 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetNodeCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetNodeCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CreateCommandQueue<T>(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCommandQueue)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandAllocator<T>(&self, r#type: D3D12_COMMAND_LIST_TYPE) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCommandAllocator)(::windows::core::Vtable::as_raw(self), r#type, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateGraphicsPipelineState<T>(&self, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateGraphicsPipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateComputePipelineState<T>(&self, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateComputePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList<P0, P1, T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: P0, pinitialstate: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), nodemask, r#type, pcommandallocator.into().abi(), pinitialstate.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn CreateDescriptorHeap<T>(&self, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDescriptorHeap)(::windows::core::Vtable::as_raw(self), pdescriptorheapdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescriptorHandleIncrementSize(&self, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDescriptorHandleIncrementSize)(::windows::core::Vtable::as_raw(self), descriptorheaptype)
    }
    pub unsafe fn CreateRootSignature<T>(&self, nodemask: u32, pblobwithrootsignature: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateRootSignature)(::windows::core::Vtable::as_raw(self), nodemask, ::core::mem::transmute(pblobwithrootsignature.as_ptr()), pblobwithrootsignature.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateConstantBufferView(&self, pdesc: ::core::option::Option<*const D3D12_CONSTANT_BUFFER_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateConstantBufferView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_SHADER_RESOURCE_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0, P1>(&self, presource: P0, pcounterresource: P1, pdesc: ::core::option::Option<*const D3D12_UNORDERED_ACCESS_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), pcounterresource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_RENDER_TARGET_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_DEPTH_STENCIL_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CreateSampler(&self, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateSampler)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CopyDescriptors(&self, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: ::core::option::Option<*const u32>, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: ::core::option::Option<*const u32>, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CopyDescriptors)(::windows::core::Vtable::as_raw(self), numdestdescriptorranges, pdestdescriptorrangestarts, ::core::mem::transmute(pdestdescriptorrangesizes.unwrap_or(::std::ptr::null())), numsrcdescriptorranges, psrcdescriptorrangestarts, ::core::mem::transmute(psrcdescriptorrangesizes.unwrap_or(::std::ptr::null())), descriptorheapstype)
    }
    pub unsafe fn CopyDescriptorsSimple(&self, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CopyDescriptorsSimple)(::windows::core::Vtable::as_raw(self), numdescriptors, ::core::mem::transmute(destdescriptorrangestart), ::core::mem::transmute(srcdescriptorrangestart), descriptorheapstype)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo(&self, visiblemask: u32, presourcedescs: &[D3D12_RESOURCE_DESC]) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetResourceAllocationInfo)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, presourcedescs.len() as _, ::core::mem::transmute(presourcedescs.as_ptr()));
        result__
    }
    pub unsafe fn GetCustomHeapProperties(&self, nodemask: u32, heaptype: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES {
        let mut result__: D3D12_HEAP_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCustomHeapProperties)(::windows::core::Vtable::as_raw(self), &mut result__, nodemask, heaptype);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource<T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCommittedResource)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap<T>(&self, pdesc: *const D3D12_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Heap>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreatePlacedResource)(::windows::core::Vtable::as_raw(self), pheap.into().abi(), heapoffset, pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource<T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateReservedResource)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<P0, P1>(&self, pobject: P0, pattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, access: u32, name: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12DeviceChild>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateSharedHandle)(::windows::core::Vtable::as_raw(self), pobject.into().abi(), ::core::mem::transmute(pattributes.unwrap_or(::std::ptr::null())), access, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandle<P0, T>(&self, nthandle: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OpenSharedHandle)(::windows::core::Vtable::as_raw(self), nthandle.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandleByName<P0>(&self, name: P0, access: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OpenSharedHandleByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), access, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MakeResident(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.MakeResident)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn Evict(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Evict)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn CreateFence<T>(&self, initialvalue: u64, flags: D3D12_FENCE_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateFence)(::windows::core::Vtable::as_raw(self), initialvalue, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCopyableFootprints(&self, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: ::core::option::Option<*mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT>, pnumrows: ::core::option::Option<*mut u32>, prowsizeinbytes: ::core::option::Option<*mut u64>, ptotalbytes: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetCopyableFootprints)(::windows::core::Vtable::as_raw(self), presourcedesc, firstsubresource, numsubresources, baseoffset, ::core::mem::transmute(playouts.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumrows.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(prowsizeinbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ptotalbytes.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CreateQueryHeap<T>(&self, pdesc: *const D3D12_QUERY_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateQueryHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStablePowerState<P0>(&self, enable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetStablePowerState)(::windows::core::Vtable::as_raw(self), enable.into()).ok()
    }
    pub unsafe fn CreateCommandSignature<P0, T>(&self, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCommandSignature)(::windows::core::Vtable::as_raw(self), pdesc, prootsignature.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: ::core::option::Option<*mut u32>, ppackedmipdesc: ::core::option::Option<*mut D3D12_PACKED_MIP_INFO>, pstandardtileshapefornonpackedmips: ::core::option::Option<*mut D3D12_TILE_SHAPE>, pnumsubresourcetilings: ::core::option::Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetResourceTiling)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ::core::mem::transmute(pnumtilesforentireresource.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppackedmipdesc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumsubresourcetilings.unwrap_or(::std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterLuid(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetAdapterLuid)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn CreatePipelineLibrary<T>(&self, plibraryblob: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreatePipelineLibrary)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plibraryblob.as_ptr()), plibraryblob.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventOnMultipleFenceCompletion<P0>(&self, ppfences: *const ::core::option::Option<ID3D12Fence>, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetEventOnMultipleFenceCompletion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppfences), pfencevalues, numfences, flags, hevent.into()).ok()
    }
    pub unsafe fn SetResidencyPriority(&self, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetResidencyPriority)(::windows::core::Vtable::as_raw(self), numobjects, ::core::mem::transmute(ppobjects), ppriorities).ok()
    }
    pub unsafe fn CreatePipelineState<T>(&self, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreatePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenExistingHeapFromAddress<T>(&self, paddress: *const ::core::ffi::c_void) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OpenExistingHeapFromAddress)(::windows::core::Vtable::as_raw(self), paddress, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenExistingHeapFromFileMapping<P0, T>(&self, hfilemapping: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.OpenExistingHeapFromFileMapping)(::windows::core::Vtable::as_raw(self), hfilemapping.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnqueueMakeResident<P0>(&self, flags: D3D12_RESIDENCY_FLAGS, ppobjects: &[ID3D12Pageable], pfencetosignal: P0, fencevaluetosignal: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Fence>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EnqueueMakeResident)(::windows::core::Vtable::as_raw(self), flags, ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr()), pfencetosignal.into().abi(), fencevaluetosignal).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12Device5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Device5 {}
impl ::core::fmt::Debug for ID3D12Device5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Device5").field(&self.0).finish()
    }
}
impl ID3D12Device5 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetNodeCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetNodeCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CreateCommandQueue<T>(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCommandQueue)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandAllocator<T>(&self, r#type: D3D12_COMMAND_LIST_TYPE) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCommandAllocator)(::windows::core::Vtable::as_raw(self), r#type, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateGraphicsPipelineState<T>(&self, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateGraphicsPipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateComputePipelineState<T>(&self, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateComputePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList<P0, P1, T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: P0, pinitialstate: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), nodemask, r#type, pcommandallocator.into().abi(), pinitialstate.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn CreateDescriptorHeap<T>(&self, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateDescriptorHeap)(::windows::core::Vtable::as_raw(self), pdescriptorheapdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescriptorHandleIncrementSize(&self, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDescriptorHandleIncrementSize)(::windows::core::Vtable::as_raw(self), descriptorheaptype)
    }
    pub unsafe fn CreateRootSignature<T>(&self, nodemask: u32, pblobwithrootsignature: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateRootSignature)(::windows::core::Vtable::as_raw(self), nodemask, ::core::mem::transmute(pblobwithrootsignature.as_ptr()), pblobwithrootsignature.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateConstantBufferView(&self, pdesc: ::core::option::Option<*const D3D12_CONSTANT_BUFFER_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateConstantBufferView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_SHADER_RESOURCE_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0, P1>(&self, presource: P0, pcounterresource: P1, pdesc: ::core::option::Option<*const D3D12_UNORDERED_ACCESS_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), pcounterresource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_RENDER_TARGET_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_DEPTH_STENCIL_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CreateSampler(&self, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateSampler)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CopyDescriptors(&self, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: ::core::option::Option<*const u32>, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: ::core::option::Option<*const u32>, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CopyDescriptors)(::windows::core::Vtable::as_raw(self), numdestdescriptorranges, pdestdescriptorrangestarts, ::core::mem::transmute(pdestdescriptorrangesizes.unwrap_or(::std::ptr::null())), numsrcdescriptorranges, psrcdescriptorrangestarts, ::core::mem::transmute(psrcdescriptorrangesizes.unwrap_or(::std::ptr::null())), descriptorheapstype)
    }
    pub unsafe fn CopyDescriptorsSimple(&self, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CopyDescriptorsSimple)(::windows::core::Vtable::as_raw(self), numdescriptors, ::core::mem::transmute(destdescriptorrangestart), ::core::mem::transmute(srcdescriptorrangestart), descriptorheapstype)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo(&self, visiblemask: u32, presourcedescs: &[D3D12_RESOURCE_DESC]) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetResourceAllocationInfo)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, presourcedescs.len() as _, ::core::mem::transmute(presourcedescs.as_ptr()));
        result__
    }
    pub unsafe fn GetCustomHeapProperties(&self, nodemask: u32, heaptype: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES {
        let mut result__: D3D12_HEAP_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetCustomHeapProperties)(::windows::core::Vtable::as_raw(self), &mut result__, nodemask, heaptype);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource<T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCommittedResource)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap<T>(&self, pdesc: *const D3D12_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Heap>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreatePlacedResource)(::windows::core::Vtable::as_raw(self), pheap.into().abi(), heapoffset, pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource<T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateReservedResource)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<P0, P1>(&self, pobject: P0, pattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, access: u32, name: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12DeviceChild>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateSharedHandle)(::windows::core::Vtable::as_raw(self), pobject.into().abi(), ::core::mem::transmute(pattributes.unwrap_or(::std::ptr::null())), access, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandle<P0, T>(&self, nthandle: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.OpenSharedHandle)(::windows::core::Vtable::as_raw(self), nthandle.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandleByName<P0>(&self, name: P0, access: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.OpenSharedHandleByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), access, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MakeResident(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.MakeResident)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn Evict(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.Evict)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn CreateFence<T>(&self, initialvalue: u64, flags: D3D12_FENCE_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateFence)(::windows::core::Vtable::as_raw(self), initialvalue, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCopyableFootprints(&self, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: ::core::option::Option<*mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT>, pnumrows: ::core::option::Option<*mut u32>, prowsizeinbytes: ::core::option::Option<*mut u64>, ptotalbytes: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetCopyableFootprints)(::windows::core::Vtable::as_raw(self), presourcedesc, firstsubresource, numsubresources, baseoffset, ::core::mem::transmute(playouts.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumrows.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(prowsizeinbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ptotalbytes.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CreateQueryHeap<T>(&self, pdesc: *const D3D12_QUERY_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateQueryHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStablePowerState<P0>(&self, enable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetStablePowerState)(::windows::core::Vtable::as_raw(self), enable.into()).ok()
    }
    pub unsafe fn CreateCommandSignature<P0, T>(&self, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCommandSignature)(::windows::core::Vtable::as_raw(self), pdesc, prootsignature.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: ::core::option::Option<*mut u32>, ppackedmipdesc: ::core::option::Option<*mut D3D12_PACKED_MIP_INFO>, pstandardtileshapefornonpackedmips: ::core::option::Option<*mut D3D12_TILE_SHAPE>, pnumsubresourcetilings: ::core::option::Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetResourceTiling)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ::core::mem::transmute(pnumtilesforentireresource.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppackedmipdesc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumsubresourcetilings.unwrap_or(::std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterLuid(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetAdapterLuid)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn CreatePipelineLibrary<T>(&self, plibraryblob: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreatePipelineLibrary)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plibraryblob.as_ptr()), plibraryblob.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventOnMultipleFenceCompletion<P0>(&self, ppfences: *const ::core::option::Option<ID3D12Fence>, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetEventOnMultipleFenceCompletion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppfences), pfencevalues, numfences, flags, hevent.into()).ok()
    }
    pub unsafe fn SetResidencyPriority(&self, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetResidencyPriority)(::windows::core::Vtable::as_raw(self), numobjects, ::core::mem::transmute(ppobjects), ppriorities).ok()
    }
    pub unsafe fn CreatePipelineState<T>(&self, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreatePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenExistingHeapFromAddress<T>(&self, paddress: *const ::core::ffi::c_void) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.OpenExistingHeapFromAddress)(::windows::core::Vtable::as_raw(self), paddress, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenExistingHeapFromFileMapping<P0, T>(&self, hfilemapping: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.OpenExistingHeapFromFileMapping)(::windows::core::Vtable::as_raw(self), hfilemapping.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnqueueMakeResident<P0>(&self, flags: D3D12_RESIDENCY_FLAGS, ppobjects: &[ID3D12Pageable], pfencetosignal: P0, fencevaluetosignal: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Fence>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EnqueueMakeResident)(::windows::core::Vtable::as_raw(self), flags, ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr()), pfencetosignal.into().abi(), fencevaluetosignal).ok()
    }
    pub unsafe fn CreateCommandList1<T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateCommandList1)(::windows::core::Vtable::as_raw(self), nodemask, r#type, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateProtectedResourceSession<T>(&self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateProtectedResourceSession)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource1<P0, T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateCommittedResource1)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap1<P0, T>(&self, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateHeap1)(::windows::core::Vtable::as_raw(self), pdesc, pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource1<P0, T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateReservedResource1)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo1(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: ::core::option::Option<*mut D3D12_RESOURCE_ALLOCATION_INFO1>) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResourceAllocationInfo1)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, numresourcedescs, presourcedescs, ::core::mem::transmute(presourceallocationinfo1.unwrap_or(::std::ptr::null_mut())));
        result__
    }
}
impl ::core::cmp::PartialEq for ID3D12Device6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Device6 {}
impl ::core::fmt::Debug for ID3D12Device6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Device6").field(&self.0).finish()
    }
}
impl ID3D12Device6 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetNodeCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetNodeCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CreateCommandQueue<T>(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateCommandQueue)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandAllocator<T>(&self, r#type: D3D12_COMMAND_LIST_TYPE) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateCommandAllocator)(::windows::core::Vtable::as_raw(self), r#type, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateGraphicsPipelineState<T>(&self, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateGraphicsPipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateComputePipelineState<T>(&self, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateComputePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList<P0, P1, T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: P0, pinitialstate: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), nodemask, r#type, pcommandallocator.into().abi(), pinitialstate.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn CreateDescriptorHeap<T>(&self, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateDescriptorHeap)(::windows::core::Vtable::as_raw(self), pdescriptorheapdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescriptorHandleIncrementSize(&self, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetDescriptorHandleIncrementSize)(::windows::core::Vtable::as_raw(self), descriptorheaptype)
    }
    pub unsafe fn CreateRootSignature<T>(&self, nodemask: u32, pblobwithrootsignature: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateRootSignature)(::windows::core::Vtable::as_raw(self), nodemask, ::core::mem::transmute(pblobwithrootsignature.as_ptr()), pblobwithrootsignature.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateConstantBufferView(&self, pdesc: ::core::option::Option<*const D3D12_CONSTANT_BUFFER_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateConstantBufferView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_SHADER_RESOURCE_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0, P1>(&self, presource: P0, pcounterresource: P1, pdesc: ::core::option::Option<*const D3D12_UNORDERED_ACCESS_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), pcounterresource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_RENDER_TARGET_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_DEPTH_STENCIL_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CreateSampler(&self, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateSampler)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CopyDescriptors(&self, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: ::core::option::Option<*const u32>, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: ::core::option::Option<*const u32>, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CopyDescriptors)(::windows::core::Vtable::as_raw(self), numdestdescriptorranges, pdestdescriptorrangestarts, ::core::mem::transmute(pdestdescriptorrangesizes.unwrap_or(::std::ptr::null())), numsrcdescriptorranges, psrcdescriptorrangestarts, ::core::mem::transmute(psrcdescriptorrangesizes.unwrap_or(::std::ptr::null())), descriptorheapstype)
    }
    pub unsafe fn CopyDescriptorsSimple(&self, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CopyDescriptorsSimple)(::windows::core::Vtable::as_raw(self), numdescriptors, ::core::mem::transmute(destdescriptorrangestart), ::core::mem::transmute(srcdescriptorrangestart), descriptorheapstype)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo(&self, visiblemask: u32, presourcedescs: &[D3D12_RESOURCE_DESC]) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetResourceAllocationInfo)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, presourcedescs.len() as _, ::core::mem::transmute(presourcedescs.as_ptr()));
        result__
    }
    pub unsafe fn GetCustomHeapProperties(&self, nodemask: u32, heaptype: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES {
        let mut result__: D3D12_HEAP_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetCustomHeapProperties)(::windows::core::Vtable::as_raw(self), &mut result__, nodemask, heaptype);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource<T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateCommittedResource)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap<T>(&self, pdesc: *const D3D12_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Heap>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreatePlacedResource)(::windows::core::Vtable::as_raw(self), pheap.into().abi(), heapoffset, pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource<T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateReservedResource)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<P0, P1>(&self, pobject: P0, pattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, access: u32, name: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12DeviceChild>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateSharedHandle)(::windows::core::Vtable::as_raw(self), pobject.into().abi(), ::core::mem::transmute(pattributes.unwrap_or(::std::ptr::null())), access, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandle<P0, T>(&self, nthandle: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.OpenSharedHandle)(::windows::core::Vtable::as_raw(self), nthandle.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandleByName<P0>(&self, name: P0, access: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.OpenSharedHandleByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), access, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MakeResident(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.MakeResident)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn Evict(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.Evict)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn CreateFence<T>(&self, initialvalue: u64, flags: D3D12_FENCE_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateFence)(::windows::core::Vtable::as_raw(self), initialvalue, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCopyableFootprints(&self, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: ::core::option::Option<*mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT>, pnumrows: ::core::option::Option<*mut u32>, prowsizeinbytes: ::core::option::Option<*mut u64>, ptotalbytes: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetCopyableFootprints)(::windows::core::Vtable::as_raw(self), presourcedesc, firstsubresource, numsubresources, baseoffset, ::core::mem::transmute(playouts.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumrows.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(prowsizeinbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ptotalbytes.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CreateQueryHeap<T>(&self, pdesc: *const D3D12_QUERY_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateQueryHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStablePowerState<P0>(&self, enable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetStablePowerState)(::windows::core::Vtable::as_raw(self), enable.into()).ok()
    }
    pub unsafe fn CreateCommandSignature<P0, T>(&self, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreateCommandSignature)(::windows::core::Vtable::as_raw(self), pdesc, prootsignature.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: ::core::option::Option<*mut u32>, ppackedmipdesc: ::core::option::Option<*mut D3D12_PACKED_MIP_INFO>, pstandardtileshapefornonpackedmips: ::core::option::Option<*mut D3D12_TILE_SHAPE>, pnumsubresourcetilings: ::core::option::Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetResourceTiling)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ::core::mem::transmute(pnumtilesforentireresource.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppackedmipdesc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumsubresourcetilings.unwrap_or(::std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterLuid(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetAdapterLuid)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn CreatePipelineLibrary<T>(&self, plibraryblob: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreatePipelineLibrary)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plibraryblob.as_ptr()), plibraryblob.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventOnMultipleFenceCompletion<P0>(&self, ppfences: *const ::core::option::Option<ID3D12Fence>, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetEventOnMultipleFenceCompletion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppfences), pfencevalues, numfences, flags, hevent.into()).ok()
    }
    pub unsafe fn SetResidencyPriority(&self, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetResidencyPriority)(::windows::core::Vtable::as_raw(self), numobjects, ::core::mem::transmute(ppobjects), ppriorities).ok()
    }
    pub unsafe fn CreatePipelineState<T>(&self, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreatePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenExistingHeapFromAddress<T>(&self, paddress: *const ::core::ffi::c_void) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OpenExistingHeapFromAddress)(::windows::core::Vtable::as_raw(self), paddress, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenExistingHeapFromFileMapping<P0, T>(&self, hfilemapping: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OpenExistingHeapFromFileMapping)(::windows::core::Vtable::as_raw(self), hfilemapping.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnqueueMakeResident<P0>(&self, flags: D3D12_RESIDENCY_FLAGS, ppobjects: &[ID3D12Pageable], pfencetosignal: P0, fencevaluetosignal: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Fence>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnqueueMakeResident)(::windows::core::Vtable::as_raw(self), flags, ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr()), pfencetosignal.into().abi(), fencevaluetosignal).ok()
    }
    pub unsafe fn CreateCommandList1<T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCommandList1)(::windows::core::Vtable::as_raw(self), nodemask, r#type, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateProtectedResourceSession<T>(&self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateProtectedResourceSession)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource1<P0, T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateCommittedResource1)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap1<P0, T>(&self, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateHeap1)(::windows::core::Vtable::as_raw(self), pdesc, pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource1<P0, T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CreateReservedResource1)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo1(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: ::core::option::Option<*mut D3D12_RESOURCE_ALLOCATION_INFO1>) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetResourceAllocationInfo1)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, numresourcedescs, presourcedescs, ::core::mem::transmute(presourceallocationinfo1.unwrap_or(::std::ptr::null_mut())));
        result__
    }
    pub unsafe fn CreateLifetimeTracker<P0, T>(&self, powner: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12LifetimeOwner>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateLifetimeTracker)(::windows::core::Vtable::as_raw(self), powner.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveDevice(&self) {
        (::windows::core::Vtable::vtable(self).base__.RemoveDevice)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EnumerateMetaCommands(&self, pnummetacommands: *mut u32, pdescs: ::core::option::Option<*mut D3D12_META_COMMAND_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumerateMetaCommands)(::windows::core::Vtable::as_raw(self), pnummetacommands, ::core::mem::transmute(pdescs.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn EnumerateMetaCommandParameters(&self, commandid: *const ::windows::core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: ::core::option::Option<*mut u32>, pparametercount: *mut u32, pparameterdescs: ::core::option::Option<*mut D3D12_META_COMMAND_PARAMETER_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.EnumerateMetaCommandParameters)(::windows::core::Vtable::as_raw(self), commandid, stage, ::core::mem::transmute(ptotalstructuresizeinbytes.unwrap_or(::std::ptr::null_mut())), pparametercount, ::core::mem::transmute(pparameterdescs.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateMetaCommand<T>(&self, commandid: *const ::windows::core::GUID, nodemask: u32, pcreationparametersdata: ::core::option::Option<*const ::core::ffi::c_void>, creationparametersdatasizeinbytes: usize) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateMetaCommand)(::windows::core::Vtable::as_raw(self), commandid, nodemask, ::core::mem::transmute(pcreationparametersdata.unwrap_or(::std::ptr::null())), creationparametersdatasizeinbytes, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStateObject<T>(&self, pdesc: *const D3D12_STATE_OBJECT_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateStateObject)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRaytracingAccelerationStructurePrebuildInfo(&self, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) {
        (::windows::core::Vtable::vtable(self).base__.GetRaytracingAccelerationStructurePrebuildInfo)(::windows::core::Vtable::as_raw(self), pdesc, pinfo)
    }
    pub unsafe fn CheckDriverMatchingIdentifier(&self, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
        (::windows::core::Vtable::vtable(self).base__.CheckDriverMatchingIdentifier)(::windows::core::Vtable::as_raw(self), serializeddatatype, pidentifiertocheck)
    }
}
impl ::core::cmp::PartialEq for ID3D12Device7 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Device7 {}
impl ::core::fmt::Debug for ID3D12Device7 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Device7").field(&self.0).finish()
    }
}
impl ID3D12Device7 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetNodeCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetNodeCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CreateCommandQueue<T>(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateCommandQueue)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandAllocator<T>(&self, r#type: D3D12_COMMAND_LIST_TYPE) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateCommandAllocator)(::windows::core::Vtable::as_raw(self), r#type, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateGraphicsPipelineState<T>(&self, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateGraphicsPipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateComputePipelineState<T>(&self, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateComputePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList<P0, P1, T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: P0, pinitialstate: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), nodemask, r#type, pcommandallocator.into().abi(), pinitialstate.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn CreateDescriptorHeap<T>(&self, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateDescriptorHeap)(::windows::core::Vtable::as_raw(self), pdescriptorheapdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescriptorHandleIncrementSize(&self, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetDescriptorHandleIncrementSize)(::windows::core::Vtable::as_raw(self), descriptorheaptype)
    }
    pub unsafe fn CreateRootSignature<T>(&self, nodemask: u32, pblobwithrootsignature: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateRootSignature)(::windows::core::Vtable::as_raw(self), nodemask, ::core::mem::transmute(pblobwithrootsignature.as_ptr()), pblobwithrootsignature.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateConstantBufferView(&self, pdesc: ::core::option::Option<*const D3D12_CONSTANT_BUFFER_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateConstantBufferView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_SHADER_RESOURCE_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0, P1>(&self, presource: P0, pcounterresource: P1, pdesc: ::core::option::Option<*const D3D12_UNORDERED_ACCESS_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), pcounterresource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_RENDER_TARGET_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_DEPTH_STENCIL_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CreateSampler(&self, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateSampler)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CopyDescriptors(&self, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: ::core::option::Option<*const u32>, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: ::core::option::Option<*const u32>, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CopyDescriptors)(::windows::core::Vtable::as_raw(self), numdestdescriptorranges, pdestdescriptorrangestarts, ::core::mem::transmute(pdestdescriptorrangesizes.unwrap_or(::std::ptr::null())), numsrcdescriptorranges, psrcdescriptorrangestarts, ::core::mem::transmute(psrcdescriptorrangesizes.unwrap_or(::std::ptr::null())), descriptorheapstype)
    }
    pub unsafe fn CopyDescriptorsSimple(&self, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CopyDescriptorsSimple)(::windows::core::Vtable::as_raw(self), numdescriptors, ::core::mem::transmute(destdescriptorrangestart), ::core::mem::transmute(srcdescriptorrangestart), descriptorheapstype)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo(&self, visiblemask: u32, presourcedescs: &[D3D12_RESOURCE_DESC]) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetResourceAllocationInfo)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, presourcedescs.len() as _, ::core::mem::transmute(presourcedescs.as_ptr()));
        result__
    }
    pub unsafe fn GetCustomHeapProperties(&self, nodemask: u32, heaptype: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES {
        let mut result__: D3D12_HEAP_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetCustomHeapProperties)(::windows::core::Vtable::as_raw(self), &mut result__, nodemask, heaptype);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource<T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateCommittedResource)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap<T>(&self, pdesc: *const D3D12_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Heap>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreatePlacedResource)(::windows::core::Vtable::as_raw(self), pheap.into().abi(), heapoffset, pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource<T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateReservedResource)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<P0, P1>(&self, pobject: P0, pattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, access: u32, name: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12DeviceChild>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateSharedHandle)(::windows::core::Vtable::as_raw(self), pobject.into().abi(), ::core::mem::transmute(pattributes.unwrap_or(::std::ptr::null())), access, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandle<P0, T>(&self, nthandle: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.OpenSharedHandle)(::windows::core::Vtable::as_raw(self), nthandle.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandleByName<P0>(&self, name: P0, access: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.OpenSharedHandleByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), access, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MakeResident(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.MakeResident)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn Evict(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.Evict)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn CreateFence<T>(&self, initialvalue: u64, flags: D3D12_FENCE_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateFence)(::windows::core::Vtable::as_raw(self), initialvalue, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCopyableFootprints(&self, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: ::core::option::Option<*mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT>, pnumrows: ::core::option::Option<*mut u32>, prowsizeinbytes: ::core::option::Option<*mut u64>, ptotalbytes: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetCopyableFootprints)(::windows::core::Vtable::as_raw(self), presourcedesc, firstsubresource, numsubresources, baseoffset, ::core::mem::transmute(playouts.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumrows.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(prowsizeinbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ptotalbytes.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CreateQueryHeap<T>(&self, pdesc: *const D3D12_QUERY_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateQueryHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStablePowerState<P0>(&self, enable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetStablePowerState)(::windows::core::Vtable::as_raw(self), enable.into()).ok()
    }
    pub unsafe fn CreateCommandSignature<P0, T>(&self, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreateCommandSignature)(::windows::core::Vtable::as_raw(self), pdesc, prootsignature.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: ::core::option::Option<*mut u32>, ppackedmipdesc: ::core::option::Option<*mut D3D12_PACKED_MIP_INFO>, pstandardtileshapefornonpackedmips: ::core::option::Option<*mut D3D12_TILE_SHAPE>, pnumsubresourcetilings: ::core::option::Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetResourceTiling)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ::core::mem::transmute(pnumtilesforentireresource.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppackedmipdesc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumsubresourcetilings.unwrap_or(::std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterLuid(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetAdapterLuid)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn CreatePipelineLibrary<T>(&self, plibraryblob: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreatePipelineLibrary)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plibraryblob.as_ptr()), plibraryblob.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventOnMultipleFenceCompletion<P0>(&self, ppfences: *const ::core::option::Option<ID3D12Fence>, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetEventOnMultipleFenceCompletion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppfences), pfencevalues, numfences, flags, hevent.into()).ok()
    }
    pub unsafe fn SetResidencyPriority(&self, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetResidencyPriority)(::windows::core::Vtable::as_raw(self), numobjects, ::core::mem::transmute(ppobjects), ppriorities).ok()
    }
    pub unsafe fn CreatePipelineState<T>(&self, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreatePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenExistingHeapFromAddress<T>(&self, paddress: *const ::core::ffi::c_void) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OpenExistingHeapFromAddress)(::windows::core::Vtable::as_raw(self), paddress, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenExistingHeapFromFileMapping<P0, T>(&self, hfilemapping: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OpenExistingHeapFromFileMapping)(::windows::core::Vtable::as_raw(self), hfilemapping.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnqueueMakeResident<P0>(&self, flags: D3D12_RESIDENCY_FLAGS, ppobjects: &[ID3D12Pageable], pfencetosignal: P0, fencevaluetosignal: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Fence>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.EnqueueMakeResident)(::windows::core::Vtable::as_raw(self), flags, ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr()), pfencetosignal.into().abi(), fencevaluetosignal).ok()
    }
    pub unsafe fn CreateCommandList1<T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCommandList1)(::windows::core::Vtable::as_raw(self), nodemask, r#type, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateProtectedResourceSession<T>(&self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateProtectedResourceSession)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource1<P0, T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateCommittedResource1)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap1<P0, T>(&self, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateHeap1)(::windows::core::Vtable::as_raw(self), pdesc, pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource1<P0, T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateReservedResource1)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo1(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: ::core::option::Option<*mut D3D12_RESOURCE_ALLOCATION_INFO1>) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetResourceAllocationInfo1)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, numresourcedescs, presourcedescs, ::core::mem::transmute(presourceallocationinfo1.unwrap_or(::std::ptr::null_mut())));
        result__
    }
    pub unsafe fn CreateLifetimeTracker<P0, T>(&self, powner: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12LifetimeOwner>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateLifetimeTracker)(::windows::core::Vtable::as_raw(self), powner.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveDevice(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.RemoveDevice)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EnumerateMetaCommands(&self, pnummetacommands: *mut u32, pdescs: ::core::option::Option<*mut D3D12_META_COMMAND_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.EnumerateMetaCommands)(::windows::core::Vtable::as_raw(self), pnummetacommands, ::core::mem::transmute(pdescs.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn EnumerateMetaCommandParameters(&self, commandid: *const ::windows::core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: ::core::option::Option<*mut u32>, pparametercount: *mut u32, pparameterdescs: ::core::option::Option<*mut D3D12_META_COMMAND_PARAMETER_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.EnumerateMetaCommandParameters)(::windows::core::Vtable::as_raw(self), commandid, stage, ::core::mem::transmute(ptotalstructuresizeinbytes.unwrap_or(::std::ptr::null_mut())), pparametercount, ::core::mem::transmute(pparameterdescs.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateMetaCommand<T>(&self, commandid: *const ::windows::core::GUID, nodemask: u32, pcreationparametersdata: ::core::option::Option<*const ::core::ffi::c_void>, creationparametersdatasizeinbytes: usize) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateMetaCommand)(::windows::core::Vtable::as_raw(self), commandid, nodemask, ::core::mem::transmute(pcreationparametersdata.unwrap_or(::std::ptr::null())), creationparametersdatasizeinbytes, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStateObject<T>(&self, pdesc: *const D3D12_STATE_OBJECT_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateStateObject)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRaytracingAccelerationStructurePrebuildInfo(&self, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) {
        (::windows::core::Vtable::vtable(self).base__.base__.GetRaytracingAccelerationStructurePrebuildInfo)(::windows::core::Vtable::as_raw(self), pdesc, pinfo)
    }
    pub unsafe fn CheckDriverMatchingIdentifier(&self, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
        (::windows::core::Vtable::vtable(self).base__.base__.CheckDriverMatchingIdentifier)(::windows::core::Vtable::as_raw(self), serializeddatatype, pidentifiertocheck)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackgroundProcessingMode<P0>(&self, mode: D3D12_BACKGROUND_PROCESSING_MODE, measurementsaction: D3D12_MEASUREMENTS_ACTION, heventtosignaluponcompletion: P0, pbfurthermeasurementsdesired: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetBackgroundProcessingMode)(::windows::core::Vtable::as_raw(self), mode, measurementsaction, heventtosignaluponcompletion.into(), ::core::mem::transmute(pbfurthermeasurementsdesired.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12Device8 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Device8 {}
impl ::core::fmt::Debug for ID3D12Device8 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Device8").field(&self.0).finish()
    }
}
impl ID3D12Device8 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetNodeCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetNodeCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CreateCommandQueue<T>(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateCommandQueue)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandAllocator<T>(&self, r#type: D3D12_COMMAND_LIST_TYPE) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateCommandAllocator)(::windows::core::Vtable::as_raw(self), r#type, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateGraphicsPipelineState<T>(&self, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateGraphicsPipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateComputePipelineState<T>(&self, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateComputePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList<P0, P1, T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: P0, pinitialstate: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), nodemask, r#type, pcommandallocator.into().abi(), pinitialstate.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn CreateDescriptorHeap<T>(&self, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateDescriptorHeap)(::windows::core::Vtable::as_raw(self), pdescriptorheapdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescriptorHandleIncrementSize(&self, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetDescriptorHandleIncrementSize)(::windows::core::Vtable::as_raw(self), descriptorheaptype)
    }
    pub unsafe fn CreateRootSignature<T>(&self, nodemask: u32, pblobwithrootsignature: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateRootSignature)(::windows::core::Vtable::as_raw(self), nodemask, ::core::mem::transmute(pblobwithrootsignature.as_ptr()), pblobwithrootsignature.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateConstantBufferView(&self, pdesc: ::core::option::Option<*const D3D12_CONSTANT_BUFFER_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateConstantBufferView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_SHADER_RESOURCE_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0, P1>(&self, presource: P0, pcounterresource: P1, pdesc: ::core::option::Option<*const D3D12_UNORDERED_ACCESS_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), pcounterresource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_RENDER_TARGET_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_DEPTH_STENCIL_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CreateSampler(&self, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateSampler)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CopyDescriptors(&self, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: ::core::option::Option<*const u32>, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: ::core::option::Option<*const u32>, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CopyDescriptors)(::windows::core::Vtable::as_raw(self), numdestdescriptorranges, pdestdescriptorrangestarts, ::core::mem::transmute(pdestdescriptorrangesizes.unwrap_or(::std::ptr::null())), numsrcdescriptorranges, psrcdescriptorrangestarts, ::core::mem::transmute(psrcdescriptorrangesizes.unwrap_or(::std::ptr::null())), descriptorheapstype)
    }
    pub unsafe fn CopyDescriptorsSimple(&self, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CopyDescriptorsSimple)(::windows::core::Vtable::as_raw(self), numdescriptors, ::core::mem::transmute(destdescriptorrangestart), ::core::mem::transmute(srcdescriptorrangestart), descriptorheapstype)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo(&self, visiblemask: u32, presourcedescs: &[D3D12_RESOURCE_DESC]) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetResourceAllocationInfo)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, presourcedescs.len() as _, ::core::mem::transmute(presourcedescs.as_ptr()));
        result__
    }
    pub unsafe fn GetCustomHeapProperties(&self, nodemask: u32, heaptype: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES {
        let mut result__: D3D12_HEAP_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetCustomHeapProperties)(::windows::core::Vtable::as_raw(self), &mut result__, nodemask, heaptype);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource<T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateCommittedResource)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap<T>(&self, pdesc: *const D3D12_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Heap>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreatePlacedResource)(::windows::core::Vtable::as_raw(self), pheap.into().abi(), heapoffset, pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource<T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateReservedResource)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<P0, P1>(&self, pobject: P0, pattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, access: u32, name: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12DeviceChild>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateSharedHandle)(::windows::core::Vtable::as_raw(self), pobject.into().abi(), ::core::mem::transmute(pattributes.unwrap_or(::std::ptr::null())), access, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandle<P0, T>(&self, nthandle: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.OpenSharedHandle)(::windows::core::Vtable::as_raw(self), nthandle.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandleByName<P0>(&self, name: P0, access: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.OpenSharedHandleByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), access, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MakeResident(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.MakeResident)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn Evict(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.Evict)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn CreateFence<T>(&self, initialvalue: u64, flags: D3D12_FENCE_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateFence)(::windows::core::Vtable::as_raw(self), initialvalue, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCopyableFootprints(&self, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: ::core::option::Option<*mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT>, pnumrows: ::core::option::Option<*mut u32>, prowsizeinbytes: ::core::option::Option<*mut u64>, ptotalbytes: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetCopyableFootprints)(::windows::core::Vtable::as_raw(self), presourcedesc, firstsubresource, numsubresources, baseoffset, ::core::mem::transmute(playouts.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumrows.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(prowsizeinbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ptotalbytes.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CreateQueryHeap<T>(&self, pdesc: *const D3D12_QUERY_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateQueryHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStablePowerState<P0>(&self, enable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetStablePowerState)(::windows::core::Vtable::as_raw(self), enable.into()).ok()
    }
    pub unsafe fn CreateCommandSignature<P0, T>(&self, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreateCommandSignature)(::windows::core::Vtable::as_raw(self), pdesc, prootsignature.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: ::core::option::Option<*mut u32>, ppackedmipdesc: ::core::option::Option<*mut D3D12_PACKED_MIP_INFO>, pstandardtileshapefornonpackedmips: ::core::option::Option<*mut D3D12_TILE_SHAPE>, pnumsubresourcetilings: ::core::option::Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetResourceTiling)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ::core::mem::transmute(pnumtilesforentireresource.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppackedmipdesc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumsubresourcetilings.unwrap_or(::std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterLuid(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetAdapterLuid)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn CreatePipelineLibrary<T>(&self, plibraryblob: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreatePipelineLibrary)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plibraryblob.as_ptr()), plibraryblob.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventOnMultipleFenceCompletion<P0>(&self, ppfences: *const ::core::option::Option<ID3D12Fence>, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetEventOnMultipleFenceCompletion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppfences), pfencevalues, numfences, flags, hevent.into()).ok()
    }
    pub unsafe fn SetResidencyPriority(&self, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetResidencyPriority)(::windows::core::Vtable::as_raw(self), numobjects, ::core::mem::transmute(ppobjects), ppriorities).ok()
    }
    pub unsafe fn CreatePipelineState<T>(&self, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CreatePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenExistingHeapFromAddress<T>(&self, paddress: *const ::core::ffi::c_void) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.OpenExistingHeapFromAddress)(::windows::core::Vtable::as_raw(self), paddress, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenExistingHeapFromFileMapping<P0, T>(&self, hfilemapping: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.OpenExistingHeapFromFileMapping)(::windows::core::Vtable::as_raw(self), hfilemapping.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnqueueMakeResident<P0>(&self, flags: D3D12_RESIDENCY_FLAGS, ppobjects: &[ID3D12Pageable], pfencetosignal: P0, fencevaluetosignal: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Fence>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.EnqueueMakeResident)(::windows::core::Vtable::as_raw(self), flags, ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr()), pfencetosignal.into().abi(), fencevaluetosignal).ok()
    }
    pub unsafe fn CreateCommandList1<T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCommandList1)(::windows::core::Vtable::as_raw(self), nodemask, r#type, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateProtectedResourceSession<T>(&self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateProtectedResourceSession)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource1<P0, T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateCommittedResource1)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap1<P0, T>(&self, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateHeap1)(::windows::core::Vtable::as_raw(self), pdesc, pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource1<P0, T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateReservedResource1)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo1(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: ::core::option::Option<*mut D3D12_RESOURCE_ALLOCATION_INFO1>) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetResourceAllocationInfo1)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, numresourcedescs, presourcedescs, ::core::mem::transmute(presourceallocationinfo1.unwrap_or(::std::ptr::null_mut())));
        result__
    }
    pub unsafe fn CreateLifetimeTracker<P0, T>(&self, powner: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12LifetimeOwner>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateLifetimeTracker)(::windows::core::Vtable::as_raw(self), powner.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveDevice(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RemoveDevice)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EnumerateMetaCommands(&self, pnummetacommands: *mut u32, pdescs: ::core::option::Option<*mut D3D12_META_COMMAND_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnumerateMetaCommands)(::windows::core::Vtable::as_raw(self), pnummetacommands, ::core::mem::transmute(pdescs.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn EnumerateMetaCommandParameters(&self, commandid: *const ::windows::core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: ::core::option::Option<*mut u32>, pparametercount: *mut u32, pparameterdescs: ::core::option::Option<*mut D3D12_META_COMMAND_PARAMETER_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EnumerateMetaCommandParameters)(::windows::core::Vtable::as_raw(self), commandid, stage, ::core::mem::transmute(ptotalstructuresizeinbytes.unwrap_or(::std::ptr::null_mut())), pparametercount, ::core::mem::transmute(pparameterdescs.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateMetaCommand<T>(&self, commandid: *const ::windows::core::GUID, nodemask: u32, pcreationparametersdata: ::core::option::Option<*const ::core::ffi::c_void>, creationparametersdatasizeinbytes: usize) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateMetaCommand)(::windows::core::Vtable::as_raw(self), commandid, nodemask, ::core::mem::transmute(pcreationparametersdata.unwrap_or(::std::ptr::null())), creationparametersdatasizeinbytes, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStateObject<T>(&self, pdesc: *const D3D12_STATE_OBJECT_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CreateStateObject)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRaytracingAccelerationStructurePrebuildInfo(&self, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetRaytracingAccelerationStructurePrebuildInfo)(::windows::core::Vtable::as_raw(self), pdesc, pinfo)
    }
    pub unsafe fn CheckDriverMatchingIdentifier(&self, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CheckDriverMatchingIdentifier)(::windows::core::Vtable::as_raw(self), serializeddatatype, pidentifiertocheck)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackgroundProcessingMode<P0>(&self, mode: D3D12_BACKGROUND_PROCESSING_MODE, measurementsaction: D3D12_MEASUREMENTS_ACTION, heventtosignaluponcompletion: P0, pbfurthermeasurementsdesired: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetBackgroundProcessingMode)(::windows::core::Vtable::as_raw(self), mode, measurementsaction, heventtosignaluponcompletion.into(), ::core::mem::transmute(pbfurthermeasurementsdesired.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn AddToStateObject<P0, T>(&self, paddition: *const D3D12_STATE_OBJECT_DESC, pstateobjecttogrowfrom: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12StateObject>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.AddToStateObject)(::windows::core::Vtable::as_raw(self), paddition, pstateobjecttogrowfrom.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateProtectedResourceSession1<T>(&self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC1) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateProtectedResourceSession1)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID3D12Device9 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Device9 {}
impl ::core::fmt::Debug for ID3D12Device9 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Device9").field(&self.0).finish()
    }
}
impl ID3D12Device9 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetNodeCount(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.GetNodeCount)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CreateCommandQueue<T>(&self, pdesc: *const D3D12_COMMAND_QUEUE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateCommandQueue)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandAllocator<T>(&self, r#type: D3D12_COMMAND_LIST_TYPE) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateCommandAllocator)(::windows::core::Vtable::as_raw(self), r#type, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateGraphicsPipelineState<T>(&self, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateGraphicsPipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateComputePipelineState<T>(&self, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateComputePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateCommandList<P0, P1, T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, pcommandallocator: P0, pinitialstate: P1) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateCommandList)(::windows::core::Vtable::as_raw(self), nodemask, r#type, pcommandallocator.into().abi(), pinitialstate.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckFeatureSupport(&self, feature: D3D12_FEATURE, pfeaturesupportdata: *mut ::core::ffi::c_void, featuresupportdatasize: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CheckFeatureSupport)(::windows::core::Vtable::as_raw(self), feature, pfeaturesupportdata, featuresupportdatasize).ok()
    }
    pub unsafe fn CreateDescriptorHeap<T>(&self, pdescriptorheapdesc: *const D3D12_DESCRIPTOR_HEAP_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateDescriptorHeap)(::windows::core::Vtable::as_raw(self), pdescriptorheapdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDescriptorHandleIncrementSize(&self, descriptorheaptype: D3D12_DESCRIPTOR_HEAP_TYPE) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.GetDescriptorHandleIncrementSize)(::windows::core::Vtable::as_raw(self), descriptorheaptype)
    }
    pub unsafe fn CreateRootSignature<T>(&self, nodemask: u32, pblobwithrootsignature: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateRootSignature)(::windows::core::Vtable::as_raw(self), nodemask, ::core::mem::transmute(pblobwithrootsignature.as_ptr()), pblobwithrootsignature.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateConstantBufferView(&self, pdesc: ::core::option::Option<*const D3D12_CONSTANT_BUFFER_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateConstantBufferView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_SHADER_RESOURCE_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateUnorderedAccessView<P0, P1>(&self, presource: P0, pcounterresource: P1, pdesc: ::core::option::Option<*const D3D12_UNORDERED_ACCESS_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateUnorderedAccessView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), pcounterresource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_RENDER_TARGET_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D12_DEPTH_STENCIL_VIEW_DESC>, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CreateSampler(&self, pdesc: *const D3D12_SAMPLER_DESC, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateSampler)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(destdescriptor))
    }
    pub unsafe fn CopyDescriptors(&self, numdestdescriptorranges: u32, pdestdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, pdestdescriptorrangesizes: ::core::option::Option<*const u32>, numsrcdescriptorranges: u32, psrcdescriptorrangestarts: *const D3D12_CPU_DESCRIPTOR_HANDLE, psrcdescriptorrangesizes: ::core::option::Option<*const u32>, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CopyDescriptors)(::windows::core::Vtable::as_raw(self), numdestdescriptorranges, pdestdescriptorrangestarts, ::core::mem::transmute(pdestdescriptorrangesizes.unwrap_or(::std::ptr::null())), numsrcdescriptorranges, psrcdescriptorrangestarts, ::core::mem::transmute(psrcdescriptorrangesizes.unwrap_or(::std::ptr::null())), descriptorheapstype)
    }
    pub unsafe fn CopyDescriptorsSimple(&self, numdescriptors: u32, destdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, srcdescriptorrangestart: D3D12_CPU_DESCRIPTOR_HANDLE, descriptorheapstype: D3D12_DESCRIPTOR_HEAP_TYPE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CopyDescriptorsSimple)(::windows::core::Vtable::as_raw(self), numdescriptors, ::core::mem::transmute(destdescriptorrangestart), ::core::mem::transmute(srcdescriptorrangestart), descriptorheapstype)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo(&self, visiblemask: u32, presourcedescs: &[D3D12_RESOURCE_DESC]) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.GetResourceAllocationInfo)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, presourcedescs.len() as _, ::core::mem::transmute(presourcedescs.as_ptr()));
        result__
    }
    pub unsafe fn GetCustomHeapProperties(&self, nodemask: u32, heaptype: D3D12_HEAP_TYPE) -> D3D12_HEAP_PROPERTIES {
        let mut result__: D3D12_HEAP_PROPERTIES = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.GetCustomHeapProperties)(::windows::core::Vtable::as_raw(self), &mut result__, nodemask, heaptype);
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource<T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateCommittedResource)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap<T>(&self, pdesc: *const D3D12_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Heap>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreatePlacedResource)(::windows::core::Vtable::as_raw(self), pheap.into().abi(), heapoffset, pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource<T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateReservedResource)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub unsafe fn CreateSharedHandle<P0, P1>(&self, pobject: P0, pattributes: ::core::option::Option<*const super::super::Security::SECURITY_ATTRIBUTES>, access: u32, name: P1) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12DeviceChild>>,
        P1: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateSharedHandle)(::windows::core::Vtable::as_raw(self), pobject.into().abi(), ::core::mem::transmute(pattributes.unwrap_or(::std::ptr::null())), access, name.into().abi(), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandle<P0, T>(&self, nthandle: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.OpenSharedHandle)(::windows::core::Vtable::as_raw(self), nthandle.into(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedHandleByName<P0>(&self, name: P0, access: u32) -> ::windows::core::Result<super::super::Foundation::HANDLE>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.OpenSharedHandleByName)(::windows::core::Vtable::as_raw(self), name.into().abi(), access, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn MakeResident(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.MakeResident)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn Evict(&self, ppobjects: &[ID3D12Pageable]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.Evict)(::windows::core::Vtable::as_raw(self), ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr())).ok()
    }
    pub unsafe fn CreateFence<T>(&self, initialvalue: u64, flags: D3D12_FENCE_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateFence)(::windows::core::Vtable::as_raw(self), initialvalue, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCopyableFootprints(&self, presourcedesc: *const D3D12_RESOURCE_DESC, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: ::core::option::Option<*mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT>, pnumrows: ::core::option::Option<*mut u32>, prowsizeinbytes: ::core::option::Option<*mut u64>, ptotalbytes: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.GetCopyableFootprints)(::windows::core::Vtable::as_raw(self), presourcedesc, firstsubresource, numsubresources, baseoffset, ::core::mem::transmute(playouts.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumrows.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(prowsizeinbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ptotalbytes.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn CreateQueryHeap<T>(&self, pdesc: *const D3D12_QUERY_HEAP_DESC, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateQueryHeap)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetStablePowerState<P0>(&self, enable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.SetStablePowerState)(::windows::core::Vtable::as_raw(self), enable.into()).ok()
    }
    pub unsafe fn CreateCommandSignature<P0, T>(&self, pdesc: *const D3D12_COMMAND_SIGNATURE_DESC, prootsignature: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.CreateCommandSignature)(::windows::core::Vtable::as_raw(self), pdesc, prootsignature.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetResourceTiling<P0>(&self, ptiledresource: P0, pnumtilesforentireresource: ::core::option::Option<*mut u32>, ppackedmipdesc: ::core::option::Option<*mut D3D12_PACKED_MIP_INFO>, pstandardtileshapefornonpackedmips: ::core::option::Option<*mut D3D12_TILE_SHAPE>, pnumsubresourcetilings: ::core::option::Option<*mut u32>, firstsubresourcetilingtoget: u32, psubresourcetilingsfornonpackedmips: *mut D3D12_SUBRESOURCE_TILING)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.GetResourceTiling)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ::core::mem::transmute(pnumtilesforentireresource.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppackedmipdesc.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstandardtileshapefornonpackedmips.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumsubresourcetilings.unwrap_or(::std::ptr::null_mut())), firstsubresourcetilingtoget, psubresourcetilingsfornonpackedmips)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetAdapterLuid(&self) -> super::super::Foundation::LUID {
        let mut result__: super::super::Foundation::LUID = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.GetAdapterLuid)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn CreatePipelineLibrary<T>(&self, plibraryblob: &[u8]) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.CreatePipelineLibrary)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(plibraryblob.as_ptr()), plibraryblob.len() as _, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventOnMultipleFenceCompletion<P0>(&self, ppfences: *const ::core::option::Option<ID3D12Fence>, pfencevalues: *const u64, numfences: u32, flags: D3D12_MULTIPLE_FENCE_WAIT_FLAGS, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetEventOnMultipleFenceCompletion)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppfences), pfencevalues, numfences, flags, hevent.into()).ok()
    }
    pub unsafe fn SetResidencyPriority(&self, numobjects: u32, ppobjects: *const ::core::option::Option<ID3D12Pageable>, ppriorities: *const D3D12_RESIDENCY_PRIORITY) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetResidencyPriority)(::windows::core::Vtable::as_raw(self), numobjects, ::core::mem::transmute(ppobjects), ppriorities).ok()
    }
    pub unsafe fn CreatePipelineState<T>(&self, pdesc: *const D3D12_PIPELINE_STATE_STREAM_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.CreatePipelineState)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn OpenExistingHeapFromAddress<T>(&self, paddress: *const ::core::ffi::c_void) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.OpenExistingHeapFromAddress)(::windows::core::Vtable::as_raw(self), paddress, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenExistingHeapFromFileMapping<P0, T>(&self, hfilemapping: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.OpenExistingHeapFromFileMapping)(::windows::core::Vtable::as_raw(self), hfilemapping.into(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn EnqueueMakeResident<P0>(&self, flags: D3D12_RESIDENCY_FLAGS, ppobjects: &[ID3D12Pageable], pfencetosignal: P0, fencevaluetosignal: u64) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Fence>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.EnqueueMakeResident)(::windows::core::Vtable::as_raw(self), flags, ppobjects.len() as _, ::core::mem::transmute(ppobjects.as_ptr()), pfencetosignal.into().abi(), fencevaluetosignal).ok()
    }
    pub unsafe fn CreateCommandList1<T>(&self, nodemask: u32, r#type: D3D12_COMMAND_LIST_TYPE, flags: D3D12_COMMAND_LIST_FLAGS) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCommandList1)(::windows::core::Vtable::as_raw(self), nodemask, r#type, flags, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateProtectedResourceSession<T>(&self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateProtectedResourceSession)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource1<P0, T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateCommittedResource1)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateHeap1<P0, T>(&self, pdesc: *const D3D12_HEAP_DESC, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateHeap1)(::windows::core::Vtable::as_raw(self), pdesc, pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateReservedResource1<P0, T>(&self, pdesc: *const D3D12_RESOURCE_DESC, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CreateReservedResource1)(::windows::core::Vtable::as_raw(self), pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo1(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC, presourceallocationinfo1: ::core::option::Option<*mut D3D12_RESOURCE_ALLOCATION_INFO1>) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetResourceAllocationInfo1)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, numresourcedescs, presourcedescs, ::core::mem::transmute(presourceallocationinfo1.unwrap_or(::std::ptr::null_mut())));
        result__
    }
    pub unsafe fn CreateLifetimeTracker<P0, T>(&self, powner: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12LifetimeOwner>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateLifetimeTracker)(::windows::core::Vtable::as_raw(self), powner.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn RemoveDevice(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RemoveDevice)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn EnumerateMetaCommands(&self, pnummetacommands: *mut u32, pdescs: ::core::option::Option<*mut D3D12_META_COMMAND_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.EnumerateMetaCommands)(::windows::core::Vtable::as_raw(self), pnummetacommands, ::core::mem::transmute(pdescs.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn EnumerateMetaCommandParameters(&self, commandid: *const ::windows::core::GUID, stage: D3D12_META_COMMAND_PARAMETER_STAGE, ptotalstructuresizeinbytes: ::core::option::Option<*mut u32>, pparametercount: *mut u32, pparameterdescs: ::core::option::Option<*mut D3D12_META_COMMAND_PARAMETER_DESC>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.EnumerateMetaCommandParameters)(::windows::core::Vtable::as_raw(self), commandid, stage, ::core::mem::transmute(ptotalstructuresizeinbytes.unwrap_or(::std::ptr::null_mut())), pparametercount, ::core::mem::transmute(pparameterdescs.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateMetaCommand<T>(&self, commandid: *const ::windows::core::GUID, nodemask: u32, pcreationparametersdata: ::core::option::Option<*const ::core::ffi::c_void>, creationparametersdatasizeinbytes: usize) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateMetaCommand)(::windows::core::Vtable::as_raw(self), commandid, nodemask, ::core::mem::transmute(pcreationparametersdata.unwrap_or(::std::ptr::null())), creationparametersdatasizeinbytes, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateStateObject<T>(&self, pdesc: *const D3D12_STATE_OBJECT_DESC) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CreateStateObject)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetRaytracingAccelerationStructurePrebuildInfo(&self, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_INPUTS, pinfo: *mut D3D12_RAYTRACING_ACCELERATION_STRUCTURE_PREBUILD_INFO) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetRaytracingAccelerationStructurePrebuildInfo)(::windows::core::Vtable::as_raw(self), pdesc, pinfo)
    }
    pub unsafe fn CheckDriverMatchingIdentifier(&self, serializeddatatype: D3D12_SERIALIZED_DATA_TYPE, pidentifiertocheck: *const D3D12_SERIALIZED_DATA_DRIVER_MATCHING_IDENTIFIER) -> D3D12_DRIVER_MATCHING_IDENTIFIER_STATUS {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CheckDriverMatchingIdentifier)(::windows::core::Vtable::as_raw(self), serializeddatatype, pidentifiertocheck)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBackgroundProcessingMode<P0>(&self, mode: D3D12_BACKGROUND_PROCESSING_MODE, measurementsaction: D3D12_MEASUREMENTS_ACTION, heventtosignaluponcompletion: P0, pbfurthermeasurementsdesired: ::core::option::Option<*mut super::super::Foundation::BOOL>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetBackgroundProcessingMode)(::windows::core::Vtable::as_raw(self), mode, measurementsaction, heventtosignaluponcompletion.into(), ::core::mem::transmute(pbfurthermeasurementsdesired.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn AddToStateObject<P0, T>(&self, paddition: *const D3D12_STATE_OBJECT_DESC, pstateobjecttogrowfrom: P0) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12StateObject>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.AddToStateObject)(::windows::core::Vtable::as_raw(self), paddition, pstateobjecttogrowfrom.into().abi(), &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CreateProtectedResourceSession1<T>(&self, pdesc: *const D3D12_PROTECTED_RESOURCE_SESSION_DESC1) -> ::windows::core::Result<T>
    where
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.CreateProtectedResourceSession1)(::windows::core::Vtable::as_raw(self), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetResourceAllocationInfo2(&self, visiblemask: u32, numresourcedescs: u32, presourcedescs: *const D3D12_RESOURCE_DESC1, presourceallocationinfo1: ::core::option::Option<*mut D3D12_RESOURCE_ALLOCATION_INFO1>) -> D3D12_RESOURCE_ALLOCATION_INFO {
        let mut result__: D3D12_RESOURCE_ALLOCATION_INFO = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResourceAllocationInfo2)(::windows::core::Vtable::as_raw(self), &mut result__, visiblemask, numresourcedescs, presourcedescs, ::core::mem::transmute(presourceallocationinfo1.unwrap_or(::std::ptr::null_mut())));
        result__
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateCommittedResource2<P0, T>(&self, pheapproperties: *const D3D12_HEAP_PROPERTIES, heapflags: D3D12_HEAP_FLAGS, pdesc: *const D3D12_RESOURCE_DESC1, initialresourcestate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, pprotectedsession: P0, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateCommittedResource2)(::windows::core::Vtable::as_raw(self), pheapproperties, heapflags, pdesc, initialresourcestate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), pprotectedsession.into().abi(), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreatePlacedResource1<P0, T>(&self, pheap: P0, heapoffset: u64, pdesc: *const D3D12_RESOURCE_DESC1, initialstate: D3D12_RESOURCE_STATES, poptimizedclearvalue: ::core::option::Option<*const D3D12_CLEAR_VALUE>, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Heap>>,
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.CreatePlacedResource1)(::windows::core::Vtable::as_raw(self), pheap.into().abi(), heapoffset, pdesc, initialstate, ::core::mem::transmute(poptimizedclearvalue.unwrap_or(::std::ptr::null())), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn CreateSamplerFeedbackUnorderedAccessView<P0, P1>(&self, ptargetedresource: P0, pfeedbackresource: P1, destdescriptor: D3D12_CPU_DESCRIPTOR_HANDLE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateSamplerFeedbackUnorderedAccessView)(::windows::core::Vtable::as_raw(self), ptargetedresource.into().abi(), pfeedbackresource.into().abi(), ::core::mem::transmute(destdescriptor))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetCopyableFootprints1(&self, presourcedesc: *const D3D12_RESOURCE_DESC1, firstsubresource: u32, numsubresources: u32, baseoffset: u64, playouts: ::core::option::Option<*mut D3D12_PLACED_SUBRESOURCE_FOOTPRINT>, pnumrows: ::core::option::Option<*mut u32>, prowsizeinbytes: ::core::option::Option<*mut u64>, ptotalbytes: ::core::option::Option<*mut u64>) {
        (::windows::core::Vtable::vtable(self).base__.GetCopyableFootprints1)(::windows::core::Vtable::as_raw(self), presourcedesc, firstsubresource, numsubresources, baseoffset, ::core::mem::transmute(playouts.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pnumrows.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(prowsizeinbytes.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ptotalbytes.unwrap_or(::std::ptr::null_mut())))
    }
}
impl ::core::cmp::PartialEq for ID3D12DeviceChild {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12DeviceChild {}
impl ::core::fmt::Debug for ID3D12DeviceChild {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12DeviceChild").field(&self.0).finish()
    }
}
impl ID3D12DeviceChild {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12DeviceRemovedExtendedData {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12DeviceRemovedExtendedData {}
impl ::core::fmt::Debug for ID3D12DeviceRemovedExtendedData {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12DeviceRemovedExtendedData").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12DeviceRemovedExtendedData1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12DeviceRemovedExtendedData1 {}
impl ::core::fmt::Debug for ID3D12DeviceRemovedExtendedData1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12DeviceRemovedExtendedData1").field(&self.0).finish()
    }
}
impl ID3D12DeviceRemovedExtendedData1 {
    pub unsafe fn GetAutoBreadcrumbsOutput(&self) -> ::windows::core::Result<D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAutoBreadcrumbsOutput)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPageFaultAllocationOutput(&self) -> ::windows::core::Result<D3D12_DRED_PAGE_FAULT_OUTPUT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPageFaultAllocationOutput)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID3D12DeviceRemovedExtendedData2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12DeviceRemovedExtendedData2 {}
impl ::core::fmt::Debug for ID3D12DeviceRemovedExtendedData2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12DeviceRemovedExtendedData2").field(&self.0).finish()
    }
}
impl ID3D12DeviceRemovedExtendedData2 {
    pub unsafe fn GetAutoBreadcrumbsOutput(&self) -> ::windows::core::Result<D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetAutoBreadcrumbsOutput)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPageFaultAllocationOutput(&self) -> ::windows::core::Result<D3D12_DRED_PAGE_FAULT_OUTPUT> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetPageFaultAllocationOutput)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetAutoBreadcrumbsOutput1(&self) -> ::windows::core::Result<D3D12_DRED_AUTO_BREADCRUMBS_OUTPUT1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetAutoBreadcrumbsOutput1)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetPageFaultAllocationOutput1(&self) -> ::windows::core::Result<D3D12_DRED_PAGE_FAULT_OUTPUT1> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetPageFaultAllocationOutput1)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr()).from_abi(result__)
    }
}
impl ::core::cmp::PartialEq for ID3D12DeviceRemovedExtendedDataSettings {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12DeviceRemovedExtendedDataSettings {}
impl ::core::fmt::Debug for ID3D12DeviceRemovedExtendedDataSettings {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12DeviceRemovedExtendedDataSettings").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12DeviceRemovedExtendedDataSettings1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12DeviceRemovedExtendedDataSettings1 {}
impl ::core::fmt::Debug for ID3D12DeviceRemovedExtendedDataSettings1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12DeviceRemovedExtendedDataSettings1").field(&self.0).finish()
    }
}
impl ID3D12DeviceRemovedExtendedDataSettings1 {
    pub unsafe fn SetAutoBreadcrumbsEnablement(&self, enablement: D3D12_DRED_ENABLEMENT) {
        (::windows::core::Vtable::vtable(self).base__.SetAutoBreadcrumbsEnablement)(::windows::core::Vtable::as_raw(self), enablement)
    }
    pub unsafe fn SetPageFaultEnablement(&self, enablement: D3D12_DRED_ENABLEMENT) {
        (::windows::core::Vtable::vtable(self).base__.SetPageFaultEnablement)(::windows::core::Vtable::as_raw(self), enablement)
    }
    pub unsafe fn SetWatsonDumpEnablement(&self, enablement: D3D12_DRED_ENABLEMENT) {
        (::windows::core::Vtable::vtable(self).base__.SetWatsonDumpEnablement)(::windows::core::Vtable::as_raw(self), enablement)
    }
}
impl ::core::cmp::PartialEq for ID3D12Fence {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Fence {}
impl ::core::fmt::Debug for ID3D12Fence {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Fence").field(&self.0).finish()
    }
}
impl ID3D12Fence {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12Fence1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Fence1 {}
impl ::core::fmt::Debug for ID3D12Fence1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Fence1").field(&self.0).finish()
    }
}
impl ID3D12Fence1 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetCompletedValue(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetCompletedValue)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetEventOnCompletion<P0>(&self, value: u64, hevent: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetEventOnCompletion)(::windows::core::Vtable::as_raw(self), value, hevent.into()).ok()
    }
    pub unsafe fn Signal(&self, value: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Signal)(::windows::core::Vtable::as_raw(self), value).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12FunctionParameterReflection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12FunctionParameterReflection {}
impl ::core::fmt::Debug for ID3D12FunctionParameterReflection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12FunctionParameterReflection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12FunctionReflection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12FunctionReflection {}
impl ::core::fmt::Debug for ID3D12FunctionReflection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12FunctionReflection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12GraphicsCommandList {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12GraphicsCommandList {}
impl ::core::fmt::Debug for ID3D12GraphicsCommandList {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12GraphicsCommandList").field(&self.0).finish()
    }
}
impl ID3D12GraphicsCommandList {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetType(&self) -> D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D12GraphicsCommandList1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12GraphicsCommandList1 {}
impl ::core::fmt::Debug for ID3D12GraphicsCommandList1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12GraphicsCommandList1").field(&self.0).finish()
    }
}
impl ID3D12GraphicsCommandList1 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetType(&self) -> D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset<P0, P1>(&self, pallocator: P0, pinitialstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.Reset)(::windows::core::Vtable::as_raw(self), pallocator.into().abi(), pinitialstate.into().abi()).ok()
    }
    pub unsafe fn ClearState<P0>(&self, ppipelinestate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ClearState)(::windows::core::Vtable::as_raw(self), ppipelinestate.into().abi())
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.DrawInstanced)(::windows::core::Vtable::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.DrawIndexedInstanced)(::windows::core::Vtable::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (::windows::core::Vtable::vtable(self).base__.Dispatch)(::windows::core::Vtable::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
    pub unsafe fn CopyBufferRegion<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, numbytes: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyBufferRegion)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, numbytes)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CopyTextureRegion(&self, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: ::core::option::Option<*const D3D12_BOX>) {
        (::windows::core::Vtable::vtable(self).base__.CopyTextureRegion)(::windows::core::Vtable::as_raw(self), pdst, dstx, dsty, dstz, psrc, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyResource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), psrcresource.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiles<P0, P1>(&self, ptiledresource: P0, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: P1, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyTiles)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ptileregionstartcoordinate, ptileregionsize, pbuffer.into().abi(), bufferstartoffsetinbytes, flags)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ResolveSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, psrcresource.into().abi(), srcsubresource, format)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Vtable::vtable(self).base__.IASetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), primitivetopology)
    }
    pub unsafe fn RSSetViewports(&self, pviewports: &[D3D12_VIEWPORT]) {
        (::windows::core::Vtable::vtable(self).base__.RSSetViewports)(::windows::core::Vtable::as_raw(self), pviewports.len() as _, ::core::mem::transmute(pviewports.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.RSSetScissorRects)(::windows::core::Vtable::as_raw(self), prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    pub unsafe fn OMSetBlendFactor(&self, blendfactor: ::core::option::Option<&[f32; 4]>) {
        (::windows::core::Vtable::vtable(self).base__.OMSetBlendFactor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(blendfactor.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetStencilRef(&self, stencilref: u32) {
        (::windows::core::Vtable::vtable(self).base__.OMSetStencilRef)(::windows::core::Vtable::as_raw(self), stencilref)
    }
    pub unsafe fn SetPipelineState<P0>(&self, ppipelinestate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPipelineState)(::windows::core::Vtable::as_raw(self), ppipelinestate.into().abi())
    }
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[D3D12_RESOURCE_BARRIER]) {
        (::windows::core::Vtable::vtable(self).base__.ResourceBarrier)(::windows::core::Vtable::as_raw(self), pbarriers.len() as _, ::core::mem::transmute(pbarriers.as_ptr()))
    }
    pub unsafe fn ExecuteBundle<P0>(&self, pcommandlist: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12GraphicsCommandList>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ExecuteBundle)(::windows::core::Vtable::as_raw(self), pcommandlist.into().abi())
    }
    pub unsafe fn SetDescriptorHeaps(&self, ppdescriptorheaps: &[ID3D12DescriptorHeap]) {
        (::windows::core::Vtable::vtable(self).base__.SetDescriptorHeaps)(::windows::core::Vtable::as_raw(self), ppdescriptorheaps.len() as _, ::core::mem::transmute(ppdescriptorheaps.as_ptr()))
    }
    pub unsafe fn SetComputeRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetComputeRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into().abi())
    }
    pub unsafe fn SetGraphicsRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetGraphicsRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into().abi())
    }
    pub unsafe fn SetComputeRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.SetComputeRootDescriptorTable)(::windows::core::Vtable::as_raw(self), rootparameterindex, ::core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetGraphicsRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.SetGraphicsRootDescriptorTable)(::windows::core::Vtable::as_raw(self), rootparameterindex, ::core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetComputeRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetComputeRoot32BitConstant)(::windows::core::Vtable::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetGraphicsRoot32BitConstant)(::windows::core::Vtable::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetComputeRoot32BitConstants)(::windows::core::Vtable::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetGraphicsRoot32BitConstants)(::windows::core::Vtable::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.SetComputeRootConstantBufferView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.SetGraphicsRootConstantBufferView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.SetComputeRootShaderResourceView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.SetGraphicsRootShaderResourceView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.SetComputeRootUnorderedAccessView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.SetGraphicsRootUnorderedAccessView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer(&self, pview: ::core::option::Option<*const D3D12_INDEX_BUFFER_VIEW>) {
        (::windows::core::Vtable::vtable(self).base__.IASetIndexBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pview.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, pviews: ::core::option::Option<&[D3D12_VERTEX_BUFFER_VIEW]>) {
        (::windows::core::Vtable::vtable(self).base__.IASetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn SOSetTargets(&self, startslot: u32, pviews: ::core::option::Option<&[D3D12_STREAM_OUTPUT_BUFFER_VIEW]>) {
        (::windows::core::Vtable::vtable(self).base__.SOSetTargets)(::windows::core::Vtable::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OMSetRenderTargets<P0>(&self, numrendertargetdescriptors: u32, prendertargetdescriptors: ::core::option::Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>, rtssinglehandletodescriptorrange: P0, pdepthstencildescriptor: ::core::option::Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.OMSetRenderTargets)(::windows::core::Vtable::as_raw(self), numrendertargetdescriptors, ::core::mem::transmute(prendertargetdescriptors.unwrap_or(::std::ptr::null())), rtssinglehandletodescriptorrange.into(), ::core::mem::transmute(pdepthstencildescriptor.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearDepthStencilView(&self, depthstencilview: D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.ClearDepthStencilView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(depthstencilview), clearflags, depth, stencil, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearRenderTargetView(&self, rendertargetview: D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.ClearRenderTargetView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rendertargetview), colorrgba, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: *const u32, prects: &[super::super::Foundation::RECT])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ClearUnorderedAccessViewUint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewgpuhandleincurrentheap), ::core::mem::transmute(viewcpuhandle), presource.into().abi(), values, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: *const f32, prects: &[super::super::Foundation::RECT])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ClearUnorderedAccessViewFloat)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewgpuhandleincurrentheap), ::core::mem::transmute(viewcpuhandle), presource.into().abi(), values, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: ::core::option::Option<*const D3D12_DISCARD_REGION>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pregion.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.BeginQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.EndQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    pub unsafe fn ResolveQueryData<P0, P1>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P1, aligneddestinationbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ResolveQueryData)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, startindex, numqueries, pdestinationbuffer.into().abi(), aligneddestinationbufferoffset)
    }
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPredication)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi(), alignedbufferoffset, operation)
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetMarker)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.BeginEvent)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).base__.EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ExecuteIndirect<P0, P1, P2>(&self, pcommandsignature: P0, maxcommandcount: u32, pargumentbuffer: P1, argumentbufferoffset: u64, pcountbuffer: P2, countbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandSignature>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ExecuteIndirect)(::windows::core::Vtable::as_raw(self), pcommandsignature.into().abi(), maxcommandcount, pargumentbuffer.into().abi(), argumentbufferoffset, pcountbuffer.into().abi(), countbufferoffset)
    }
}
impl ::core::cmp::PartialEq for ID3D12GraphicsCommandList2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12GraphicsCommandList2 {}
impl ::core::fmt::Debug for ID3D12GraphicsCommandList2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12GraphicsCommandList2").field(&self.0).finish()
    }
}
impl ID3D12GraphicsCommandList2 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetType(&self) -> D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset<P0, P1>(&self, pallocator: P0, pinitialstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.Reset)(::windows::core::Vtable::as_raw(self), pallocator.into().abi(), pinitialstate.into().abi()).ok()
    }
    pub unsafe fn ClearState<P0>(&self, ppipelinestate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearState)(::windows::core::Vtable::as_raw(self), ppipelinestate.into().abi())
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawInstanced)(::windows::core::Vtable::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.DrawIndexedInstanced)(::windows::core::Vtable::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.Dispatch)(::windows::core::Vtable::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
    pub unsafe fn CopyBufferRegion<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, numbytes: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyBufferRegion)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, numbytes)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CopyTextureRegion(&self, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: ::core::option::Option<*const D3D12_BOX>) {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyTextureRegion)(::windows::core::Vtable::as_raw(self), pdst, dstx, dsty, dstz, psrc, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyResource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), psrcresource.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiles<P0, P1>(&self, ptiledresource: P0, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: P1, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyTiles)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ptileregionstartcoordinate, ptileregionsize, pbuffer.into().abi(), bufferstartoffsetinbytes, flags)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ResolveSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, psrcresource.into().abi(), srcsubresource, format)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Vtable::vtable(self).base__.base__.IASetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), primitivetopology)
    }
    pub unsafe fn RSSetViewports(&self, pviewports: &[D3D12_VIEWPORT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.RSSetViewports)(::windows::core::Vtable::as_raw(self), pviewports.len() as _, ::core::mem::transmute(pviewports.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.RSSetScissorRects)(::windows::core::Vtable::as_raw(self), prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    pub unsafe fn OMSetBlendFactor(&self, blendfactor: ::core::option::Option<&[f32; 4]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.OMSetBlendFactor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(blendfactor.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetStencilRef(&self, stencilref: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.OMSetStencilRef)(::windows::core::Vtable::as_raw(self), stencilref)
    }
    pub unsafe fn SetPipelineState<P0>(&self, ppipelinestate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPipelineState)(::windows::core::Vtable::as_raw(self), ppipelinestate.into().abi())
    }
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[D3D12_RESOURCE_BARRIER]) {
        (::windows::core::Vtable::vtable(self).base__.base__.ResourceBarrier)(::windows::core::Vtable::as_raw(self), pbarriers.len() as _, ::core::mem::transmute(pbarriers.as_ptr()))
    }
    pub unsafe fn ExecuteBundle<P0>(&self, pcommandlist: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12GraphicsCommandList>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ExecuteBundle)(::windows::core::Vtable::as_raw(self), pcommandlist.into().abi())
    }
    pub unsafe fn SetDescriptorHeaps(&self, ppdescriptorheaps: &[ID3D12DescriptorHeap]) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetDescriptorHeaps)(::windows::core::Vtable::as_raw(self), ppdescriptorheaps.len() as _, ::core::mem::transmute(ppdescriptorheaps.as_ptr()))
    }
    pub unsafe fn SetComputeRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetComputeRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into().abi())
    }
    pub unsafe fn SetGraphicsRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGraphicsRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into().abi())
    }
    pub unsafe fn SetComputeRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetComputeRootDescriptorTable)(::windows::core::Vtable::as_raw(self), rootparameterindex, ::core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetGraphicsRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGraphicsRootDescriptorTable)(::windows::core::Vtable::as_raw(self), rootparameterindex, ::core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetComputeRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetComputeRoot32BitConstant)(::windows::core::Vtable::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGraphicsRoot32BitConstant)(::windows::core::Vtable::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetComputeRoot32BitConstants)(::windows::core::Vtable::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGraphicsRoot32BitConstants)(::windows::core::Vtable::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetComputeRootConstantBufferView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGraphicsRootConstantBufferView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetComputeRootShaderResourceView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGraphicsRootShaderResourceView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetComputeRootUnorderedAccessView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetGraphicsRootUnorderedAccessView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer(&self, pview: ::core::option::Option<*const D3D12_INDEX_BUFFER_VIEW>) {
        (::windows::core::Vtable::vtable(self).base__.base__.IASetIndexBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pview.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, pviews: ::core::option::Option<&[D3D12_VERTEX_BUFFER_VIEW]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.IASetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn SOSetTargets(&self, startslot: u32, pviews: ::core::option::Option<&[D3D12_STREAM_OUTPUT_BUFFER_VIEW]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.SOSetTargets)(::windows::core::Vtable::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OMSetRenderTargets<P0>(&self, numrendertargetdescriptors: u32, prendertargetdescriptors: ::core::option::Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>, rtssinglehandletodescriptorrange: P0, pdepthstencildescriptor: ::core::option::Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.OMSetRenderTargets)(::windows::core::Vtable::as_raw(self), numrendertargetdescriptors, ::core::mem::transmute(prendertargetdescriptors.unwrap_or(::std::ptr::null())), rtssinglehandletodescriptorrange.into(), ::core::mem::transmute(pdepthstencildescriptor.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearDepthStencilView(&self, depthstencilview: D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearDepthStencilView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(depthstencilview), clearflags, depth, stencil, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearRenderTargetView(&self, rendertargetview: D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearRenderTargetView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rendertargetview), colorrgba, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: *const u32, prects: &[super::super::Foundation::RECT])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearUnorderedAccessViewUint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewgpuhandleincurrentheap), ::core::mem::transmute(viewcpuhandle), presource.into().abi(), values, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: *const f32, prects: &[super::super::Foundation::RECT])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ClearUnorderedAccessViewFloat)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewgpuhandleincurrentheap), ::core::mem::transmute(viewcpuhandle), presource.into().abi(), values, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: ::core::option::Option<*const D3D12_DISCARD_REGION>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pregion.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.BeginQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.EndQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    pub unsafe fn ResolveQueryData<P0, P1>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P1, aligneddestinationbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ResolveQueryData)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, startindex, numqueries, pdestinationbuffer.into().abi(), aligneddestinationbufferoffset)
    }
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPredication)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi(), alignedbufferoffset, operation)
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetMarker)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.BeginEvent)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ExecuteIndirect<P0, P1, P2>(&self, pcommandsignature: P0, maxcommandcount: u32, pargumentbuffer: P1, argumentbufferoffset: u64, pcountbuffer: P2, countbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandSignature>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ExecuteIndirect)(::windows::core::Vtable::as_raw(self), pcommandsignature.into().abi(), maxcommandcount, pargumentbuffer.into().abi(), argumentbufferoffset, pcountbuffer.into().abi(), countbufferoffset)
    }
    pub unsafe fn AtomicCopyBufferUINT<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AtomicCopyBufferUINT)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, dependencies, ::core::mem::transmute(ppdependentresources), pdependentsubresourceranges)
    }
    pub unsafe fn AtomicCopyBufferUINT64<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AtomicCopyBufferUINT64)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, dependencies, ::core::mem::transmute(ppdependentresources), pdependentsubresourceranges)
    }
    pub unsafe fn OMSetDepthBounds(&self, min: f32, max: f32) {
        (::windows::core::Vtable::vtable(self).base__.OMSetDepthBounds)(::windows::core::Vtable::as_raw(self), min, max)
    }
    pub unsafe fn SetSamplePositions(&self, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION) {
        (::windows::core::Vtable::vtable(self).base__.SetSamplePositions)(::windows::core::Vtable::as_raw(self), numsamplesperpixel, numpixels, psamplepositions)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn ResolveSubresourceRegion<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: P1, srcsubresource: u32, psrcrect: ::core::option::Option<*const super::super::Foundation::RECT>, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ResolveSubresourceRegion)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcrect.unwrap_or(::std::ptr::null())), format, resolvemode)
    }
    pub unsafe fn SetViewInstanceMask(&self, mask: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetViewInstanceMask)(::windows::core::Vtable::as_raw(self), mask)
    }
}
impl ::core::cmp::PartialEq for ID3D12GraphicsCommandList3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12GraphicsCommandList3 {}
impl ::core::fmt::Debug for ID3D12GraphicsCommandList3 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12GraphicsCommandList3").field(&self.0).finish()
    }
}
impl ID3D12GraphicsCommandList3 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetType(&self) -> D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset<P0, P1>(&self, pallocator: P0, pinitialstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Reset)(::windows::core::Vtable::as_raw(self), pallocator.into().abi(), pinitialstate.into().abi()).ok()
    }
    pub unsafe fn ClearState<P0>(&self, ppipelinestate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClearState)(::windows::core::Vtable::as_raw(self), ppipelinestate.into().abi())
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawInstanced)(::windows::core::Vtable::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DrawIndexedInstanced)(::windows::core::Vtable::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.Dispatch)(::windows::core::Vtable::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
    pub unsafe fn CopyBufferRegion<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, numbytes: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CopyBufferRegion)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, numbytes)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CopyTextureRegion(&self, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: ::core::option::Option<*const D3D12_BOX>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CopyTextureRegion)(::windows::core::Vtable::as_raw(self), pdst, dstx, dsty, dstz, psrc, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CopyResource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), psrcresource.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiles<P0, P1>(&self, ptiledresource: P0, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: P1, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.CopyTiles)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ptileregionstartcoordinate, ptileregionsize, pbuffer.into().abi(), bufferstartoffsetinbytes, flags)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ResolveSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, psrcresource.into().abi(), srcsubresource, format)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IASetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), primitivetopology)
    }
    pub unsafe fn RSSetViewports(&self, pviewports: &[D3D12_VIEWPORT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RSSetViewports)(::windows::core::Vtable::as_raw(self), pviewports.len() as _, ::core::mem::transmute(pviewports.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.RSSetScissorRects)(::windows::core::Vtable::as_raw(self), prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    pub unsafe fn OMSetBlendFactor(&self, blendfactor: ::core::option::Option<&[f32; 4]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OMSetBlendFactor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(blendfactor.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetStencilRef(&self, stencilref: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OMSetStencilRef)(::windows::core::Vtable::as_raw(self), stencilref)
    }
    pub unsafe fn SetPipelineState<P0>(&self, ppipelinestate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPipelineState)(::windows::core::Vtable::as_raw(self), ppipelinestate.into().abi())
    }
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[D3D12_RESOURCE_BARRIER]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ResourceBarrier)(::windows::core::Vtable::as_raw(self), pbarriers.len() as _, ::core::mem::transmute(pbarriers.as_ptr()))
    }
    pub unsafe fn ExecuteBundle<P0>(&self, pcommandlist: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12GraphicsCommandList>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ExecuteBundle)(::windows::core::Vtable::as_raw(self), pcommandlist.into().abi())
    }
    pub unsafe fn SetDescriptorHeaps(&self, ppdescriptorheaps: &[ID3D12DescriptorHeap]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetDescriptorHeaps)(::windows::core::Vtable::as_raw(self), ppdescriptorheaps.len() as _, ::core::mem::transmute(ppdescriptorheaps.as_ptr()))
    }
    pub unsafe fn SetComputeRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetComputeRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into().abi())
    }
    pub unsafe fn SetGraphicsRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetGraphicsRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into().abi())
    }
    pub unsafe fn SetComputeRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetComputeRootDescriptorTable)(::windows::core::Vtable::as_raw(self), rootparameterindex, ::core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetGraphicsRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetGraphicsRootDescriptorTable)(::windows::core::Vtable::as_raw(self), rootparameterindex, ::core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetComputeRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetComputeRoot32BitConstant)(::windows::core::Vtable::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetGraphicsRoot32BitConstant)(::windows::core::Vtable::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetComputeRoot32BitConstants)(::windows::core::Vtable::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetGraphicsRoot32BitConstants)(::windows::core::Vtable::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetComputeRootConstantBufferView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetGraphicsRootConstantBufferView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetComputeRootShaderResourceView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetGraphicsRootShaderResourceView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetComputeRootUnorderedAccessView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetGraphicsRootUnorderedAccessView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer(&self, pview: ::core::option::Option<*const D3D12_INDEX_BUFFER_VIEW>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IASetIndexBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pview.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, pviews: ::core::option::Option<&[D3D12_VERTEX_BUFFER_VIEW]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.IASetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn SOSetTargets(&self, startslot: u32, pviews: ::core::option::Option<&[D3D12_STREAM_OUTPUT_BUFFER_VIEW]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SOSetTargets)(::windows::core::Vtable::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OMSetRenderTargets<P0>(&self, numrendertargetdescriptors: u32, prendertargetdescriptors: ::core::option::Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>, rtssinglehandletodescriptorrange: P0, pdepthstencildescriptor: ::core::option::Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OMSetRenderTargets)(::windows::core::Vtable::as_raw(self), numrendertargetdescriptors, ::core::mem::transmute(prendertargetdescriptors.unwrap_or(::std::ptr::null())), rtssinglehandletodescriptorrange.into(), ::core::mem::transmute(pdepthstencildescriptor.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearDepthStencilView(&self, depthstencilview: D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClearDepthStencilView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(depthstencilview), clearflags, depth, stencil, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearRenderTargetView(&self, rendertargetview: D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClearRenderTargetView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rendertargetview), colorrgba, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: *const u32, prects: &[super::super::Foundation::RECT])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClearUnorderedAccessViewUint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewgpuhandleincurrentheap), ::core::mem::transmute(viewcpuhandle), presource.into().abi(), values, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: *const f32, prects: &[super::super::Foundation::RECT])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ClearUnorderedAccessViewFloat)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewgpuhandleincurrentheap), ::core::mem::transmute(viewcpuhandle), presource.into().abi(), values, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: ::core::option::Option<*const D3D12_DISCARD_REGION>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pregion.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.BeginQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EndQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    pub unsafe fn ResolveQueryData<P0, P1>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P1, aligneddestinationbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ResolveQueryData)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, startindex, numqueries, pdestinationbuffer.into().abi(), aligneddestinationbufferoffset)
    }
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPredication)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi(), alignedbufferoffset, operation)
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetMarker)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.BeginEvent)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ExecuteIndirect<P0, P1, P2>(&self, pcommandsignature: P0, maxcommandcount: u32, pargumentbuffer: P1, argumentbufferoffset: u64, pcountbuffer: P2, countbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandSignature>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ExecuteIndirect)(::windows::core::Vtable::as_raw(self), pcommandsignature.into().abi(), maxcommandcount, pargumentbuffer.into().abi(), argumentbufferoffset, pcountbuffer.into().abi(), countbufferoffset)
    }
    pub unsafe fn AtomicCopyBufferUINT<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AtomicCopyBufferUINT)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, dependencies, ::core::mem::transmute(ppdependentresources), pdependentsubresourceranges)
    }
    pub unsafe fn AtomicCopyBufferUINT64<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.AtomicCopyBufferUINT64)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, dependencies, ::core::mem::transmute(ppdependentresources), pdependentsubresourceranges)
    }
    pub unsafe fn OMSetDepthBounds(&self, min: f32, max: f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.OMSetDepthBounds)(::windows::core::Vtable::as_raw(self), min, max)
    }
    pub unsafe fn SetSamplePositions(&self, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetSamplePositions)(::windows::core::Vtable::as_raw(self), numsamplesperpixel, numpixels, psamplepositions)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn ResolveSubresourceRegion<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: P1, srcsubresource: u32, psrcrect: ::core::option::Option<*const super::super::Foundation::RECT>, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ResolveSubresourceRegion)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcrect.unwrap_or(::std::ptr::null())), format, resolvemode)
    }
    pub unsafe fn SetViewInstanceMask(&self, mask: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.SetViewInstanceMask)(::windows::core::Vtable::as_raw(self), mask)
    }
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: ::core::option::Option<*const D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        (::windows::core::Vtable::vtable(self).base__.WriteBufferImmediate)(::windows::core::Vtable::as_raw(self), count, pparams, ::core::mem::transmute(pmodes.unwrap_or(::std::ptr::null())))
    }
}
impl ::core::cmp::PartialEq for ID3D12GraphicsCommandList4 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12GraphicsCommandList4 {}
impl ::core::fmt::Debug for ID3D12GraphicsCommandList4 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12GraphicsCommandList4").field(&self.0).finish()
    }
}
impl ID3D12GraphicsCommandList4 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetType(&self) -> D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset<P0, P1>(&self, pallocator: P0, pinitialstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Reset)(::windows::core::Vtable::as_raw(self), pallocator.into().abi(), pinitialstate.into().abi()).ok()
    }
    pub unsafe fn ClearState<P0>(&self, ppipelinestate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ClearState)(::windows::core::Vtable::as_raw(self), ppipelinestate.into().abi())
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawInstanced)(::windows::core::Vtable::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DrawIndexedInstanced)(::windows::core::Vtable::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.Dispatch)(::windows::core::Vtable::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
    pub unsafe fn CopyBufferRegion<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, numbytes: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CopyBufferRegion)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, numbytes)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CopyTextureRegion(&self, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: ::core::option::Option<*const D3D12_BOX>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CopyTextureRegion)(::windows::core::Vtable::as_raw(self), pdst, dstx, dsty, dstz, psrc, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CopyResource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), psrcresource.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiles<P0, P1>(&self, ptiledresource: P0, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: P1, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.CopyTiles)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ptileregionstartcoordinate, ptileregionsize, pbuffer.into().abi(), bufferstartoffsetinbytes, flags)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ResolveSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, psrcresource.into().abi(), srcsubresource, format)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IASetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), primitivetopology)
    }
    pub unsafe fn RSSetViewports(&self, pviewports: &[D3D12_VIEWPORT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RSSetViewports)(::windows::core::Vtable::as_raw(self), pviewports.len() as _, ::core::mem::transmute(pviewports.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.RSSetScissorRects)(::windows::core::Vtable::as_raw(self), prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    pub unsafe fn OMSetBlendFactor(&self, blendfactor: ::core::option::Option<&[f32; 4]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OMSetBlendFactor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(blendfactor.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetStencilRef(&self, stencilref: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OMSetStencilRef)(::windows::core::Vtable::as_raw(self), stencilref)
    }
    pub unsafe fn SetPipelineState<P0>(&self, ppipelinestate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPipelineState)(::windows::core::Vtable::as_raw(self), ppipelinestate.into().abi())
    }
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[D3D12_RESOURCE_BARRIER]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ResourceBarrier)(::windows::core::Vtable::as_raw(self), pbarriers.len() as _, ::core::mem::transmute(pbarriers.as_ptr()))
    }
    pub unsafe fn ExecuteBundle<P0>(&self, pcommandlist: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12GraphicsCommandList>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ExecuteBundle)(::windows::core::Vtable::as_raw(self), pcommandlist.into().abi())
    }
    pub unsafe fn SetDescriptorHeaps(&self, ppdescriptorheaps: &[ID3D12DescriptorHeap]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetDescriptorHeaps)(::windows::core::Vtable::as_raw(self), ppdescriptorheaps.len() as _, ::core::mem::transmute(ppdescriptorheaps.as_ptr()))
    }
    pub unsafe fn SetComputeRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetComputeRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into().abi())
    }
    pub unsafe fn SetGraphicsRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetGraphicsRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into().abi())
    }
    pub unsafe fn SetComputeRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetComputeRootDescriptorTable)(::windows::core::Vtable::as_raw(self), rootparameterindex, ::core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetGraphicsRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetGraphicsRootDescriptorTable)(::windows::core::Vtable::as_raw(self), rootparameterindex, ::core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetComputeRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetComputeRoot32BitConstant)(::windows::core::Vtable::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetGraphicsRoot32BitConstant)(::windows::core::Vtable::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetComputeRoot32BitConstants)(::windows::core::Vtable::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetGraphicsRoot32BitConstants)(::windows::core::Vtable::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetComputeRootConstantBufferView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetGraphicsRootConstantBufferView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetComputeRootShaderResourceView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetGraphicsRootShaderResourceView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetComputeRootUnorderedAccessView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetGraphicsRootUnorderedAccessView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer(&self, pview: ::core::option::Option<*const D3D12_INDEX_BUFFER_VIEW>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IASetIndexBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pview.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, pviews: ::core::option::Option<&[D3D12_VERTEX_BUFFER_VIEW]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.IASetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn SOSetTargets(&self, startslot: u32, pviews: ::core::option::Option<&[D3D12_STREAM_OUTPUT_BUFFER_VIEW]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SOSetTargets)(::windows::core::Vtable::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OMSetRenderTargets<P0>(&self, numrendertargetdescriptors: u32, prendertargetdescriptors: ::core::option::Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>, rtssinglehandletodescriptorrange: P0, pdepthstencildescriptor: ::core::option::Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OMSetRenderTargets)(::windows::core::Vtable::as_raw(self), numrendertargetdescriptors, ::core::mem::transmute(prendertargetdescriptors.unwrap_or(::std::ptr::null())), rtssinglehandletodescriptorrange.into(), ::core::mem::transmute(pdepthstencildescriptor.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearDepthStencilView(&self, depthstencilview: D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ClearDepthStencilView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(depthstencilview), clearflags, depth, stencil, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearRenderTargetView(&self, rendertargetview: D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ClearRenderTargetView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rendertargetview), colorrgba, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: *const u32, prects: &[super::super::Foundation::RECT])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ClearUnorderedAccessViewUint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewgpuhandleincurrentheap), ::core::mem::transmute(viewcpuhandle), presource.into().abi(), values, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: *const f32, prects: &[super::super::Foundation::RECT])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ClearUnorderedAccessViewFloat)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewgpuhandleincurrentheap), ::core::mem::transmute(viewcpuhandle), presource.into().abi(), values, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: ::core::option::Option<*const D3D12_DISCARD_REGION>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pregion.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.BeginQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.EndQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    pub unsafe fn ResolveQueryData<P0, P1>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P1, aligneddestinationbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ResolveQueryData)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, startindex, numqueries, pdestinationbuffer.into().abi(), aligneddestinationbufferoffset)
    }
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPredication)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi(), alignedbufferoffset, operation)
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetMarker)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.BeginEvent)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ExecuteIndirect<P0, P1, P2>(&self, pcommandsignature: P0, maxcommandcount: u32, pargumentbuffer: P1, argumentbufferoffset: u64, pcountbuffer: P2, countbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandSignature>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ExecuteIndirect)(::windows::core::Vtable::as_raw(self), pcommandsignature.into().abi(), maxcommandcount, pargumentbuffer.into().abi(), argumentbufferoffset, pcountbuffer.into().abi(), countbufferoffset)
    }
    pub unsafe fn AtomicCopyBufferUINT<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AtomicCopyBufferUINT)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, dependencies, ::core::mem::transmute(ppdependentresources), pdependentsubresourceranges)
    }
    pub unsafe fn AtomicCopyBufferUINT64<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.AtomicCopyBufferUINT64)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, dependencies, ::core::mem::transmute(ppdependentresources), pdependentsubresourceranges)
    }
    pub unsafe fn OMSetDepthBounds(&self, min: f32, max: f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.OMSetDepthBounds)(::windows::core::Vtable::as_raw(self), min, max)
    }
    pub unsafe fn SetSamplePositions(&self, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetSamplePositions)(::windows::core::Vtable::as_raw(self), numsamplesperpixel, numpixels, psamplepositions)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn ResolveSubresourceRegion<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: P1, srcsubresource: u32, psrcrect: ::core::option::Option<*const super::super::Foundation::RECT>, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.ResolveSubresourceRegion)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcrect.unwrap_or(::std::ptr::null())), format, resolvemode)
    }
    pub unsafe fn SetViewInstanceMask(&self, mask: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetViewInstanceMask)(::windows::core::Vtable::as_raw(self), mask)
    }
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: ::core::option::Option<*const D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        (::windows::core::Vtable::vtable(self).base__.base__.WriteBufferImmediate)(::windows::core::Vtable::as_raw(self), count, pparams, ::core::mem::transmute(pmodes.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn SetProtectedResourceSession<P0>(&self, pprotectedresourcesession: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetProtectedResourceSession)(::windows::core::Vtable::as_raw(self), pprotectedresourcesession.into().abi())
    }
}
impl ::core::cmp::PartialEq for ID3D12GraphicsCommandList5 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12GraphicsCommandList5 {}
impl ::core::fmt::Debug for ID3D12GraphicsCommandList5 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12GraphicsCommandList5").field(&self.0).finish()
    }
}
impl ID3D12GraphicsCommandList5 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetType(&self) -> D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset<P0, P1>(&self, pallocator: P0, pinitialstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.Reset)(::windows::core::Vtable::as_raw(self), pallocator.into().abi(), pinitialstate.into().abi()).ok()
    }
    pub unsafe fn ClearState<P0>(&self, ppipelinestate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ClearState)(::windows::core::Vtable::as_raw(self), ppipelinestate.into().abi())
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawInstanced)(::windows::core::Vtable::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DrawIndexedInstanced)(::windows::core::Vtable::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.Dispatch)(::windows::core::Vtable::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
    pub unsafe fn CopyBufferRegion<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, numbytes: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CopyBufferRegion)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, numbytes)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CopyTextureRegion(&self, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: ::core::option::Option<*const D3D12_BOX>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CopyTextureRegion)(::windows::core::Vtable::as_raw(self), pdst, dstx, dsty, dstz, psrc, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CopyResource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), psrcresource.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiles<P0, P1>(&self, ptiledresource: P0, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: P1, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.CopyTiles)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ptileregionstartcoordinate, ptileregionsize, pbuffer.into().abi(), bufferstartoffsetinbytes, flags)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ResolveSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, psrcresource.into().abi(), srcsubresource, format)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IASetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), primitivetopology)
    }
    pub unsafe fn RSSetViewports(&self, pviewports: &[D3D12_VIEWPORT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RSSetViewports)(::windows::core::Vtable::as_raw(self), pviewports.len() as _, ::core::mem::transmute(pviewports.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.RSSetScissorRects)(::windows::core::Vtable::as_raw(self), prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    pub unsafe fn OMSetBlendFactor(&self, blendfactor: ::core::option::Option<&[f32; 4]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.OMSetBlendFactor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(blendfactor.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetStencilRef(&self, stencilref: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.OMSetStencilRef)(::windows::core::Vtable::as_raw(self), stencilref)
    }
    pub unsafe fn SetPipelineState<P0>(&self, ppipelinestate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPipelineState)(::windows::core::Vtable::as_raw(self), ppipelinestate.into().abi())
    }
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[D3D12_RESOURCE_BARRIER]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ResourceBarrier)(::windows::core::Vtable::as_raw(self), pbarriers.len() as _, ::core::mem::transmute(pbarriers.as_ptr()))
    }
    pub unsafe fn ExecuteBundle<P0>(&self, pcommandlist: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12GraphicsCommandList>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ExecuteBundle)(::windows::core::Vtable::as_raw(self), pcommandlist.into().abi())
    }
    pub unsafe fn SetDescriptorHeaps(&self, ppdescriptorheaps: &[ID3D12DescriptorHeap]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetDescriptorHeaps)(::windows::core::Vtable::as_raw(self), ppdescriptorheaps.len() as _, ::core::mem::transmute(ppdescriptorheaps.as_ptr()))
    }
    pub unsafe fn SetComputeRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetComputeRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into().abi())
    }
    pub unsafe fn SetGraphicsRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetGraphicsRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into().abi())
    }
    pub unsafe fn SetComputeRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetComputeRootDescriptorTable)(::windows::core::Vtable::as_raw(self), rootparameterindex, ::core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetGraphicsRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetGraphicsRootDescriptorTable)(::windows::core::Vtable::as_raw(self), rootparameterindex, ::core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetComputeRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetComputeRoot32BitConstant)(::windows::core::Vtable::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetGraphicsRoot32BitConstant)(::windows::core::Vtable::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetComputeRoot32BitConstants)(::windows::core::Vtable::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetGraphicsRoot32BitConstants)(::windows::core::Vtable::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetComputeRootConstantBufferView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetGraphicsRootConstantBufferView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetComputeRootShaderResourceView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetGraphicsRootShaderResourceView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetComputeRootUnorderedAccessView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetGraphicsRootUnorderedAccessView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer(&self, pview: ::core::option::Option<*const D3D12_INDEX_BUFFER_VIEW>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IASetIndexBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pview.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, pviews: ::core::option::Option<&[D3D12_VERTEX_BUFFER_VIEW]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.IASetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn SOSetTargets(&self, startslot: u32, pviews: ::core::option::Option<&[D3D12_STREAM_OUTPUT_BUFFER_VIEW]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SOSetTargets)(::windows::core::Vtable::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OMSetRenderTargets<P0>(&self, numrendertargetdescriptors: u32, prendertargetdescriptors: ::core::option::Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>, rtssinglehandletodescriptorrange: P0, pdepthstencildescriptor: ::core::option::Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.OMSetRenderTargets)(::windows::core::Vtable::as_raw(self), numrendertargetdescriptors, ::core::mem::transmute(prendertargetdescriptors.unwrap_or(::std::ptr::null())), rtssinglehandletodescriptorrange.into(), ::core::mem::transmute(pdepthstencildescriptor.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearDepthStencilView(&self, depthstencilview: D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ClearDepthStencilView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(depthstencilview), clearflags, depth, stencil, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearRenderTargetView(&self, rendertargetview: D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ClearRenderTargetView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rendertargetview), colorrgba, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: *const u32, prects: &[super::super::Foundation::RECT])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ClearUnorderedAccessViewUint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewgpuhandleincurrentheap), ::core::mem::transmute(viewcpuhandle), presource.into().abi(), values, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: *const f32, prects: &[super::super::Foundation::RECT])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ClearUnorderedAccessViewFloat)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewgpuhandleincurrentheap), ::core::mem::transmute(viewcpuhandle), presource.into().abi(), values, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: ::core::option::Option<*const D3D12_DISCARD_REGION>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pregion.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.BeginQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.EndQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    pub unsafe fn ResolveQueryData<P0, P1>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P1, aligneddestinationbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ResolveQueryData)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, startindex, numqueries, pdestinationbuffer.into().abi(), aligneddestinationbufferoffset)
    }
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPredication)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi(), alignedbufferoffset, operation)
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetMarker)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.BeginEvent)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ExecuteIndirect<P0, P1, P2>(&self, pcommandsignature: P0, maxcommandcount: u32, pargumentbuffer: P1, argumentbufferoffset: u64, pcountbuffer: P2, countbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandSignature>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ExecuteIndirect)(::windows::core::Vtable::as_raw(self), pcommandsignature.into().abi(), maxcommandcount, pargumentbuffer.into().abi(), argumentbufferoffset, pcountbuffer.into().abi(), countbufferoffset)
    }
    pub unsafe fn AtomicCopyBufferUINT<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AtomicCopyBufferUINT)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, dependencies, ::core::mem::transmute(ppdependentresources), pdependentsubresourceranges)
    }
    pub unsafe fn AtomicCopyBufferUINT64<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.AtomicCopyBufferUINT64)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, dependencies, ::core::mem::transmute(ppdependentresources), pdependentsubresourceranges)
    }
    pub unsafe fn OMSetDepthBounds(&self, min: f32, max: f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.OMSetDepthBounds)(::windows::core::Vtable::as_raw(self), min, max)
    }
    pub unsafe fn SetSamplePositions(&self, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetSamplePositions)(::windows::core::Vtable::as_raw(self), numsamplesperpixel, numpixels, psamplepositions)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn ResolveSubresourceRegion<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: P1, srcsubresource: u32, psrcrect: ::core::option::Option<*const super::super::Foundation::RECT>, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.ResolveSubresourceRegion)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcrect.unwrap_or(::std::ptr::null())), format, resolvemode)
    }
    pub unsafe fn SetViewInstanceMask(&self, mask: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetViewInstanceMask)(::windows::core::Vtable::as_raw(self), mask)
    }
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: ::core::option::Option<*const D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.WriteBufferImmediate)(::windows::core::Vtable::as_raw(self), count, pparams, ::core::mem::transmute(pmodes.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn SetProtectedResourceSession<P0>(&self, pprotectedresourcesession: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetProtectedResourceSession)(::windows::core::Vtable::as_raw(self), pprotectedresourcesession.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn BeginRenderPass(&self, prendertargets: ::core::option::Option<&[D3D12_RENDER_PASS_RENDER_TARGET_DESC]>, pdepthstencil: ::core::option::Option<*const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC>, flags: D3D12_RENDER_PASS_FLAGS) {
        (::windows::core::Vtable::vtable(self).base__.BeginRenderPass)(::windows::core::Vtable::as_raw(self), prendertargets.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(prendertargets.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(pdepthstencil.unwrap_or(::std::ptr::null())), flags)
    }
    pub unsafe fn EndRenderPass(&self) {
        (::windows::core::Vtable::vtable(self).base__.EndRenderPass)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn InitializeMetaCommand<P0>(&self, pmetacommand: P0, pinitializationparametersdata: ::core::option::Option<*const ::core::ffi::c_void>, initializationparametersdatasizeinbytes: usize)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12MetaCommand>>,
    {
        (::windows::core::Vtable::vtable(self).base__.InitializeMetaCommand)(::windows::core::Vtable::as_raw(self), pmetacommand.into().abi(), ::core::mem::transmute(pinitializationparametersdata.unwrap_or(::std::ptr::null())), initializationparametersdatasizeinbytes)
    }
    pub unsafe fn ExecuteMetaCommand<P0>(&self, pmetacommand: P0, pexecutionparametersdata: ::core::option::Option<*const ::core::ffi::c_void>, executionparametersdatasizeinbytes: usize)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12MetaCommand>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ExecuteMetaCommand)(::windows::core::Vtable::as_raw(self), pmetacommand.into().abi(), ::core::mem::transmute(pexecutionparametersdata.unwrap_or(::std::ptr::null())), executionparametersdatasizeinbytes)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn BuildRaytracingAccelerationStructure(&self, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, ppostbuildinfodescs: ::core::option::Option<&[D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC]>) {
        (::windows::core::Vtable::vtable(self).base__.BuildRaytracingAccelerationStructure)(::windows::core::Vtable::as_raw(self), pdesc, ppostbuildinfodescs.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppostbuildinfodescs.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn EmitRaytracingAccelerationStructurePostbuildInfo(&self, pdesc: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, psourceaccelerationstructuredata: &[u64]) {
        (::windows::core::Vtable::vtable(self).base__.EmitRaytracingAccelerationStructurePostbuildInfo)(::windows::core::Vtable::as_raw(self), pdesc, psourceaccelerationstructuredata.len() as _, ::core::mem::transmute(psourceaccelerationstructuredata.as_ptr()))
    }
    pub unsafe fn CopyRaytracingAccelerationStructure(&self, destaccelerationstructuredata: u64, sourceaccelerationstructuredata: u64, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE) {
        (::windows::core::Vtable::vtable(self).base__.CopyRaytracingAccelerationStructure)(::windows::core::Vtable::as_raw(self), destaccelerationstructuredata, sourceaccelerationstructuredata, mode)
    }
    pub unsafe fn SetPipelineState1<P0>(&self, pstateobject: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12StateObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPipelineState1)(::windows::core::Vtable::as_raw(self), pstateobject.into().abi())
    }
    pub unsafe fn DispatchRays(&self, pdesc: *const D3D12_DISPATCH_RAYS_DESC) {
        (::windows::core::Vtable::vtable(self).base__.DispatchRays)(::windows::core::Vtable::as_raw(self), pdesc)
    }
}
impl ::core::cmp::PartialEq for ID3D12GraphicsCommandList6 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12GraphicsCommandList6 {}
impl ::core::fmt::Debug for ID3D12GraphicsCommandList6 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12GraphicsCommandList6").field(&self.0).finish()
    }
}
impl ID3D12GraphicsCommandList6 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetType(&self) -> D3D12_COMMAND_LIST_TYPE {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Close(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.Close)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn Reset<P0, P1>(&self, pallocator: P0, pinitialstate: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandAllocator>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.Reset)(::windows::core::Vtable::as_raw(self), pallocator.into().abi(), pinitialstate.into().abi()).ok()
    }
    pub unsafe fn ClearState<P0>(&self, ppipelinestate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ClearState)(::windows::core::Vtable::as_raw(self), ppipelinestate.into().abi())
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawInstanced)(::windows::core::Vtable::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DrawIndexedInstanced)(::windows::core::Vtable::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn Dispatch(&self, threadgroupcountx: u32, threadgroupcounty: u32, threadgroupcountz: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.Dispatch)(::windows::core::Vtable::as_raw(self), threadgroupcountx, threadgroupcounty, threadgroupcountz)
    }
    pub unsafe fn CopyBufferRegion<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, numbytes: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CopyBufferRegion)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, numbytes)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CopyTextureRegion(&self, pdst: *const D3D12_TEXTURE_COPY_LOCATION, dstx: u32, dsty: u32, dstz: u32, psrc: *const D3D12_TEXTURE_COPY_LOCATION, psrcbox: ::core::option::Option<*const D3D12_BOX>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CopyTextureRegion)(::windows::core::Vtable::as_raw(self), pdst, dstx, dsty, dstz, psrc, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CopyResource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), psrcresource.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CopyTiles<P0, P1>(&self, ptiledresource: P0, ptileregionstartcoordinate: *const D3D12_TILED_RESOURCE_COORDINATE, ptileregionsize: *const D3D12_TILE_REGION_SIZE, pbuffer: P1, bufferstartoffsetinbytes: u64, flags: D3D12_TILE_COPY_FLAGS)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.CopyTiles)(::windows::core::Vtable::as_raw(self), ptiledresource.into().abi(), ptileregionstartcoordinate, ptileregionsize, pbuffer.into().abi(), bufferstartoffsetinbytes, flags)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ResolveSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, psrcresource.into().abi(), srcsubresource, format)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, primitivetopology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.IASetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), primitivetopology)
    }
    pub unsafe fn RSSetViewports(&self, pviewports: &[D3D12_VIEWPORT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.RSSetViewports)(::windows::core::Vtable::as_raw(self), pviewports.len() as _, ::core::mem::transmute(pviewports.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.RSSetScissorRects)(::windows::core::Vtable::as_raw(self), prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    pub unsafe fn OMSetBlendFactor(&self, blendfactor: ::core::option::Option<&[f32; 4]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.OMSetBlendFactor)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(blendfactor.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetStencilRef(&self, stencilref: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.OMSetStencilRef)(::windows::core::Vtable::as_raw(self), stencilref)
    }
    pub unsafe fn SetPipelineState<P0>(&self, ppipelinestate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetPipelineState)(::windows::core::Vtable::as_raw(self), ppipelinestate.into().abi())
    }
    pub unsafe fn ResourceBarrier(&self, pbarriers: &[D3D12_RESOURCE_BARRIER]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ResourceBarrier)(::windows::core::Vtable::as_raw(self), pbarriers.len() as _, ::core::mem::transmute(pbarriers.as_ptr()))
    }
    pub unsafe fn ExecuteBundle<P0>(&self, pcommandlist: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12GraphicsCommandList>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ExecuteBundle)(::windows::core::Vtable::as_raw(self), pcommandlist.into().abi())
    }
    pub unsafe fn SetDescriptorHeaps(&self, ppdescriptorheaps: &[ID3D12DescriptorHeap]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetDescriptorHeaps)(::windows::core::Vtable::as_raw(self), ppdescriptorheaps.len() as _, ::core::mem::transmute(ppdescriptorheaps.as_ptr()))
    }
    pub unsafe fn SetComputeRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetComputeRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into().abi())
    }
    pub unsafe fn SetGraphicsRootSignature<P0>(&self, prootsignature: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12RootSignature>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetGraphicsRootSignature)(::windows::core::Vtable::as_raw(self), prootsignature.into().abi())
    }
    pub unsafe fn SetComputeRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetComputeRootDescriptorTable)(::windows::core::Vtable::as_raw(self), rootparameterindex, ::core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetGraphicsRootDescriptorTable(&self, rootparameterindex: u32, basedescriptor: D3D12_GPU_DESCRIPTOR_HANDLE) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetGraphicsRootDescriptorTable)(::windows::core::Vtable::as_raw(self), rootparameterindex, ::core::mem::transmute(basedescriptor))
    }
    pub unsafe fn SetComputeRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetComputeRoot32BitConstant)(::windows::core::Vtable::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstant(&self, rootparameterindex: u32, srcdata: u32, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetGraphicsRoot32BitConstant)(::windows::core::Vtable::as_raw(self), rootparameterindex, srcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetComputeRoot32BitConstants)(::windows::core::Vtable::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetGraphicsRoot32BitConstants(&self, rootparameterindex: u32, num32bitvaluestoset: u32, psrcdata: *const ::core::ffi::c_void, destoffsetin32bitvalues: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetGraphicsRoot32BitConstants)(::windows::core::Vtable::as_raw(self), rootparameterindex, num32bitvaluestoset, psrcdata, destoffsetin32bitvalues)
    }
    pub unsafe fn SetComputeRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetComputeRootConstantBufferView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootConstantBufferView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetGraphicsRootConstantBufferView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetComputeRootShaderResourceView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootShaderResourceView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetGraphicsRootShaderResourceView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetComputeRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetComputeRootUnorderedAccessView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    pub unsafe fn SetGraphicsRootUnorderedAccessView(&self, rootparameterindex: u32, bufferlocation: u64) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetGraphicsRootUnorderedAccessView)(::windows::core::Vtable::as_raw(self), rootparameterindex, bufferlocation)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer(&self, pview: ::core::option::Option<*const D3D12_INDEX_BUFFER_VIEW>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.IASetIndexBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pview.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, pviews: ::core::option::Option<&[D3D12_VERTEX_BUFFER_VIEW]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.IASetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn SOSetTargets(&self, startslot: u32, pviews: ::core::option::Option<&[D3D12_STREAM_OUTPUT_BUFFER_VIEW]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SOSetTargets)(::windows::core::Vtable::as_raw(self), startslot, pviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OMSetRenderTargets<P0>(&self, numrendertargetdescriptors: u32, prendertargetdescriptors: ::core::option::Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>, rtssinglehandletodescriptorrange: P0, pdepthstencildescriptor: ::core::option::Option<*const D3D12_CPU_DESCRIPTOR_HANDLE>)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.OMSetRenderTargets)(::windows::core::Vtable::as_raw(self), numrendertargetdescriptors, ::core::mem::transmute(prendertargetdescriptors.unwrap_or(::std::ptr::null())), rtssinglehandletodescriptorrange.into(), ::core::mem::transmute(pdepthstencildescriptor.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearDepthStencilView(&self, depthstencilview: D3D12_CPU_DESCRIPTOR_HANDLE, clearflags: D3D12_CLEAR_FLAGS, depth: f32, stencil: u8, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ClearDepthStencilView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(depthstencilview), clearflags, depth, stencil, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearRenderTargetView(&self, rendertargetview: D3D12_CPU_DESCRIPTOR_HANDLE, colorrgba: *const f32, prects: &[super::super::Foundation::RECT]) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ClearRenderTargetView)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(rendertargetview), colorrgba, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearUnorderedAccessViewUint<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: *const u32, prects: &[super::super::Foundation::RECT])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ClearUnorderedAccessViewUint)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewgpuhandleincurrentheap), ::core::mem::transmute(viewcpuhandle), presource.into().abi(), values, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn ClearUnorderedAccessViewFloat<P0>(&self, viewgpuhandleincurrentheap: D3D12_GPU_DESCRIPTOR_HANDLE, viewcpuhandle: D3D12_CPU_DESCRIPTOR_HANDLE, presource: P0, values: *const f32, prects: &[super::super::Foundation::RECT])
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ClearUnorderedAccessViewFloat)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(viewgpuhandleincurrentheap), ::core::mem::transmute(viewcpuhandle), presource.into().abi(), values, prects.len() as _, ::core::mem::transmute(prects.as_ptr()))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn DiscardResource<P0>(&self, presource: P0, pregion: ::core::option::Option<*const D3D12_DISCARD_REGION>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.DiscardResource)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pregion.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn BeginQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.BeginQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    pub unsafe fn EndQuery<P0>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, index: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.EndQuery)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, index)
    }
    pub unsafe fn ResolveQueryData<P0, P1>(&self, pqueryheap: P0, r#type: D3D12_QUERY_TYPE, startindex: u32, numqueries: u32, pdestinationbuffer: P1, aligneddestinationbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12QueryHeap>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ResolveQueryData)(::windows::core::Vtable::as_raw(self), pqueryheap.into().abi(), r#type, startindex, numqueries, pdestinationbuffer.into().abi(), aligneddestinationbufferoffset)
    }
    pub unsafe fn SetPredication<P0>(&self, pbuffer: P0, alignedbufferoffset: u64, operation: D3D12_PREDICATION_OP)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetPredication)(::windows::core::Vtable::as_raw(self), pbuffer.into().abi(), alignedbufferoffset, operation)
    }
    pub unsafe fn SetMarker(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.SetMarker)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn BeginEvent(&self, metadata: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>, size: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.BeginEvent)(::windows::core::Vtable::as_raw(self), metadata, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null())), size)
    }
    pub unsafe fn EndEvent(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.EndEvent)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn ExecuteIndirect<P0, P1, P2>(&self, pcommandsignature: P0, maxcommandcount: u32, pargumentbuffer: P1, argumentbufferoffset: u64, pcountbuffer: P2, countbufferoffset: u64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12CommandSignature>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P2: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.base__.ExecuteIndirect)(::windows::core::Vtable::as_raw(self), pcommandsignature.into().abi(), maxcommandcount, pargumentbuffer.into().abi(), argumentbufferoffset, pcountbuffer.into().abi(), countbufferoffset)
    }
    pub unsafe fn AtomicCopyBufferUINT<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AtomicCopyBufferUINT)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, dependencies, ::core::mem::transmute(ppdependentresources), pdependentsubresourceranges)
    }
    pub unsafe fn AtomicCopyBufferUINT64<P0, P1>(&self, pdstbuffer: P0, dstoffset: u64, psrcbuffer: P1, srcoffset: u64, dependencies: u32, ppdependentresources: *const ::core::option::Option<ID3D12Resource>, pdependentsubresourceranges: *const D3D12_SUBRESOURCE_RANGE_UINT64)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.AtomicCopyBufferUINT64)(::windows::core::Vtable::as_raw(self), pdstbuffer.into().abi(), dstoffset, psrcbuffer.into().abi(), srcoffset, dependencies, ::core::mem::transmute(ppdependentresources), pdependentsubresourceranges)
    }
    pub unsafe fn OMSetDepthBounds(&self, min: f32, max: f32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.OMSetDepthBounds)(::windows::core::Vtable::as_raw(self), min, max)
    }
    pub unsafe fn SetSamplePositions(&self, numsamplesperpixel: u32, numpixels: u32, psamplepositions: *const D3D12_SAMPLE_POSITION) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetSamplePositions)(::windows::core::Vtable::as_raw(self), numsamplesperpixel, numpixels, psamplepositions)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn ResolveSubresourceRegion<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, psrcresource: P1, srcsubresource: u32, psrcrect: ::core::option::Option<*const super::super::Foundation::RECT>, format: super::Dxgi::Common::DXGI_FORMAT, resolvemode: D3D12_RESOLVE_MODE)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.ResolveSubresourceRegion)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcrect.unwrap_or(::std::ptr::null())), format, resolvemode)
    }
    pub unsafe fn SetViewInstanceMask(&self, mask: u32) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetViewInstanceMask)(::windows::core::Vtable::as_raw(self), mask)
    }
    pub unsafe fn WriteBufferImmediate(&self, count: u32, pparams: *const D3D12_WRITEBUFFERIMMEDIATE_PARAMETER, pmodes: ::core::option::Option<*const D3D12_WRITEBUFFERIMMEDIATE_MODE>) {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.WriteBufferImmediate)(::windows::core::Vtable::as_raw(self), count, pparams, ::core::mem::transmute(pmodes.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn SetProtectedResourceSession<P0>(&self, pprotectedresourcesession: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12ProtectedResourceSession>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetProtectedResourceSession)(::windows::core::Vtable::as_raw(self), pprotectedresourcesession.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn BeginRenderPass(&self, prendertargets: ::core::option::Option<&[D3D12_RENDER_PASS_RENDER_TARGET_DESC]>, pdepthstencil: ::core::option::Option<*const D3D12_RENDER_PASS_DEPTH_STENCIL_DESC>, flags: D3D12_RENDER_PASS_FLAGS) {
        (::windows::core::Vtable::vtable(self).base__.base__.BeginRenderPass)(::windows::core::Vtable::as_raw(self), prendertargets.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(prendertargets.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(pdepthstencil.unwrap_or(::std::ptr::null())), flags)
    }
    pub unsafe fn EndRenderPass(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.EndRenderPass)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn InitializeMetaCommand<P0>(&self, pmetacommand: P0, pinitializationparametersdata: ::core::option::Option<*const ::core::ffi::c_void>, initializationparametersdatasizeinbytes: usize)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12MetaCommand>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.InitializeMetaCommand)(::windows::core::Vtable::as_raw(self), pmetacommand.into().abi(), ::core::mem::transmute(pinitializationparametersdata.unwrap_or(::std::ptr::null())), initializationparametersdatasizeinbytes)
    }
    pub unsafe fn ExecuteMetaCommand<P0>(&self, pmetacommand: P0, pexecutionparametersdata: ::core::option::Option<*const ::core::ffi::c_void>, executionparametersdatasizeinbytes: usize)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12MetaCommand>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.ExecuteMetaCommand)(::windows::core::Vtable::as_raw(self), pmetacommand.into().abi(), ::core::mem::transmute(pexecutionparametersdata.unwrap_or(::std::ptr::null())), executionparametersdatasizeinbytes)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn BuildRaytracingAccelerationStructure(&self, pdesc: *const D3D12_BUILD_RAYTRACING_ACCELERATION_STRUCTURE_DESC, ppostbuildinfodescs: ::core::option::Option<&[D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC]>) {
        (::windows::core::Vtable::vtable(self).base__.base__.BuildRaytracingAccelerationStructure)(::windows::core::Vtable::as_raw(self), pdesc, ppostbuildinfodescs.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppostbuildinfodescs.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn EmitRaytracingAccelerationStructurePostbuildInfo(&self, pdesc: *const D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_DESC, psourceaccelerationstructuredata: &[u64]) {
        (::windows::core::Vtable::vtable(self).base__.base__.EmitRaytracingAccelerationStructurePostbuildInfo)(::windows::core::Vtable::as_raw(self), pdesc, psourceaccelerationstructuredata.len() as _, ::core::mem::transmute(psourceaccelerationstructuredata.as_ptr()))
    }
    pub unsafe fn CopyRaytracingAccelerationStructure(&self, destaccelerationstructuredata: u64, sourceaccelerationstructuredata: u64, mode: D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE) {
        (::windows::core::Vtable::vtable(self).base__.base__.CopyRaytracingAccelerationStructure)(::windows::core::Vtable::as_raw(self), destaccelerationstructuredata, sourceaccelerationstructuredata, mode)
    }
    pub unsafe fn SetPipelineState1<P0>(&self, pstateobject: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12StateObject>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPipelineState1)(::windows::core::Vtable::as_raw(self), pstateobject.into().abi())
    }
    pub unsafe fn DispatchRays(&self, pdesc: *const D3D12_DISPATCH_RAYS_DESC) {
        (::windows::core::Vtable::vtable(self).base__.base__.DispatchRays)(::windows::core::Vtable::as_raw(self), pdesc)
    }
    pub unsafe fn RSSetShadingRate(&self, baseshadingrate: D3D12_SHADING_RATE, combiners: ::core::option::Option<*const D3D12_SHADING_RATE_COMBINER>) {
        (::windows::core::Vtable::vtable(self).base__.RSSetShadingRate)(::windows::core::Vtable::as_raw(self), baseshadingrate, ::core::mem::transmute(combiners.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn RSSetShadingRateImage<P0>(&self, shadingrateimage: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D12Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RSSetShadingRateImage)(::windows::core::Vtable::as_raw(self), shadingrateimage.into().abi())
    }
}
impl ::core::cmp::PartialEq for ID3D12Heap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Heap {}
impl ::core::fmt::Debug for ID3D12Heap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Heap").field(&self.0).finish()
    }
}
impl ID3D12Heap {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12Heap1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Heap1 {}
impl ::core::fmt::Debug for ID3D12Heap1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Heap1").field(&self.0).finish()
    }
}
impl ID3D12Heap1 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetDesc(&self) -> D3D12_HEAP_DESC {
        let mut result__: D3D12_HEAP_DESC = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
impl ::core::cmp::PartialEq for ID3D12InfoQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12InfoQueue {}
impl ::core::fmt::Debug for ID3D12InfoQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12InfoQueue").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12InfoQueue1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12InfoQueue1 {}
impl ::core::fmt::Debug for ID3D12InfoQueue1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12InfoQueue1").field(&self.0).finish()
    }
}
impl ID3D12InfoQueue1 {
    pub unsafe fn SetMessageCountLimit(&self, messagecountlimit: u64) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetMessageCountLimit)(::windows::core::Vtable::as_raw(self), messagecountlimit).ok()
    }
    pub unsafe fn ClearStoredMessages(&self) {
        (::windows::core::Vtable::vtable(self).base__.ClearStoredMessages)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMessage(&self, messageindex: u64, pmessage: ::core::option::Option<*mut D3D12_MESSAGE>, pmessagebytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetMessage)(::windows::core::Vtable::as_raw(self), messageindex, ::core::mem::transmute(pmessage.unwrap_or(::std::ptr::null_mut())), pmessagebytelength).ok()
    }
    pub unsafe fn GetNumMessagesAllowedByStorageFilter(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetNumMessagesAllowedByStorageFilter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetNumMessagesDeniedByStorageFilter(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetNumMessagesDeniedByStorageFilter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetNumStoredMessages(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetNumStoredMessages)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetNumStoredMessagesAllowedByRetrievalFilter(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetNumStoredMessagesAllowedByRetrievalFilter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetNumMessagesDiscardedByMessageCountLimit(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetNumMessagesDiscardedByMessageCountLimit)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetMessageCountLimit(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetMessageCountLimit)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AddStorageFilterEntries(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddStorageFilterEntries)(::windows::core::Vtable::as_raw(self), pfilter).ok()
    }
    pub unsafe fn GetStorageFilter(&self, pfilter: ::core::option::Option<*mut D3D12_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetStorageFilter)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfilter.unwrap_or(::std::ptr::null_mut())), pfilterbytelength).ok()
    }
    pub unsafe fn ClearStorageFilter(&self) {
        (::windows::core::Vtable::vtable(self).base__.ClearStorageFilter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PushEmptyStorageFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PushEmptyStorageFilter)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PushCopyOfStorageFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PushCopyOfStorageFilter)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PushStorageFilter(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PushStorageFilter)(::windows::core::Vtable::as_raw(self), pfilter).ok()
    }
    pub unsafe fn PopStorageFilter(&self) {
        (::windows::core::Vtable::vtable(self).base__.PopStorageFilter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetStorageFilterStackSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetStorageFilterStackSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AddRetrievalFilterEntries(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.AddRetrievalFilterEntries)(::windows::core::Vtable::as_raw(self), pfilter).ok()
    }
    pub unsafe fn GetRetrievalFilter(&self, pfilter: ::core::option::Option<*mut D3D12_INFO_QUEUE_FILTER>, pfilterbytelength: *mut usize) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRetrievalFilter)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pfilter.unwrap_or(::std::ptr::null_mut())), pfilterbytelength).ok()
    }
    pub unsafe fn ClearRetrievalFilter(&self) {
        (::windows::core::Vtable::vtable(self).base__.ClearRetrievalFilter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn PushEmptyRetrievalFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PushEmptyRetrievalFilter)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PushCopyOfRetrievalFilter(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PushCopyOfRetrievalFilter)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn PushRetrievalFilter(&self, pfilter: *const D3D12_INFO_QUEUE_FILTER) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.PushRetrievalFilter)(::windows::core::Vtable::as_raw(self), pfilter).ok()
    }
    pub unsafe fn PopRetrievalFilter(&self) {
        (::windows::core::Vtable::vtable(self).base__.PopRetrievalFilter)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetRetrievalFilterStackSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetRetrievalFilterStackSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AddMessage<P0>(&self, category: D3D12_MESSAGE_CATEGORY, severity: D3D12_MESSAGE_SEVERITY, id: D3D12_MESSAGE_ID, pdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddMessage)(::windows::core::Vtable::as_raw(self), category, severity, id, pdescription.into().abi()).ok()
    }
    pub unsafe fn AddApplicationMessage<P0>(&self, severity: D3D12_MESSAGE_SEVERITY, pdescription: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.AddApplicationMessage)(::windows::core::Vtable::as_raw(self), severity, pdescription.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnCategory<P0>(&self, category: D3D12_MESSAGE_CATEGORY, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetBreakOnCategory)(::windows::core::Vtable::as_raw(self), category, benable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnSeverity<P0>(&self, severity: D3D12_MESSAGE_SEVERITY, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetBreakOnSeverity)(::windows::core::Vtable::as_raw(self), severity, benable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetBreakOnID<P0>(&self, id: D3D12_MESSAGE_ID, benable: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetBreakOnID)(::windows::core::Vtable::as_raw(self), id, benable.into()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnCategory(&self, category: D3D12_MESSAGE_CATEGORY) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.GetBreakOnCategory)(::windows::core::Vtable::as_raw(self), category)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnSeverity(&self, severity: D3D12_MESSAGE_SEVERITY) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.GetBreakOnSeverity)(::windows::core::Vtable::as_raw(self), severity)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetBreakOnID(&self, id: D3D12_MESSAGE_ID) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.GetBreakOnID)(::windows::core::Vtable::as_raw(self), id)
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetMuteDebugOutput<P0>(&self, bmute: P0)
    where
        P0: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetMuteDebugOutput)(::windows::core::Vtable::as_raw(self), bmute.into())
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetMuteDebugOutput(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.GetMuteDebugOutput)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D12LibraryReflection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12LibraryReflection {}
impl ::core::fmt::Debug for ID3D12LibraryReflection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12LibraryReflection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12LifetimeOwner {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12LifetimeOwner {}
impl ::core::fmt::Debug for ID3D12LifetimeOwner {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12LifetimeOwner").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12LifetimeTracker {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12LifetimeTracker {}
impl ::core::fmt::Debug for ID3D12LifetimeTracker {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12LifetimeTracker").field(&self.0).finish()
    }
}
impl ID3D12LifetimeTracker {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12MetaCommand {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12MetaCommand {}
impl ::core::fmt::Debug for ID3D12MetaCommand {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12MetaCommand").field(&self.0).finish()
    }
}
impl ID3D12MetaCommand {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12Object {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Object {}
impl ::core::fmt::Debug for ID3D12Object {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Object").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12Pageable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Pageable {}
impl ::core::fmt::Debug for ID3D12Pageable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Pageable").field(&self.0).finish()
    }
}
impl ID3D12Pageable {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12PipelineLibrary {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12PipelineLibrary {}
impl ::core::fmt::Debug for ID3D12PipelineLibrary {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12PipelineLibrary").field(&self.0).finish()
    }
}
impl ID3D12PipelineLibrary {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12PipelineLibrary1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12PipelineLibrary1 {}
impl ::core::fmt::Debug for ID3D12PipelineLibrary1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12PipelineLibrary1").field(&self.0).finish()
    }
}
impl ID3D12PipelineLibrary1 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn StorePipeline<P0, P1>(&self, pname: P0, ppipeline: P1) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D12PipelineState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.StorePipeline)(::windows::core::Vtable::as_raw(self), pname.into().abi(), ppipeline.into().abi()).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn LoadGraphicsPipeline<P0, T>(&self, pname: P0, pdesc: *const D3D12_GRAPHICS_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LoadGraphicsPipeline)(::windows::core::Vtable::as_raw(self), pname.into().abi(), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn LoadComputePipeline<P0, T>(&self, pname: P0, pdesc: *const D3D12_COMPUTE_PIPELINE_STATE_DESC) -> ::windows::core::Result<T>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
        T: ::windows::core::Interface,
    {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.LoadComputePipeline)(::windows::core::Vtable::as_raw(self), pname.into().abi(), pdesc, &<T as ::windows::core::Interface>::IID, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn GetSerializedSize(&self) -> usize {
        (::windows::core::Vtable::vtable(self).base__.GetSerializedSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Serialize(&self, pdata: &mut [u8]) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Serialize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata.as_ptr()), pdata.len() as _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12PipelineState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12PipelineState {}
impl ::core::fmt::Debug for ID3D12PipelineState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12PipelineState").field(&self.0).finish()
    }
}
impl ID3D12PipelineState {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12ProtectedResourceSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12ProtectedResourceSession {}
impl ::core::fmt::Debug for ID3D12ProtectedResourceSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12ProtectedResourceSession").field(&self.0).finish()
    }
}
impl ID3D12ProtectedResourceSession {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetStatusFence<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.GetStatusFence)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetSessionStatus(&self) -> D3D12_PROTECTED_SESSION_STATUS {
        (::windows::core::Vtable::vtable(self).base__.GetSessionStatus)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D12ProtectedResourceSession1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12ProtectedResourceSession1 {}
impl ::core::fmt::Debug for ID3D12ProtectedResourceSession1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12ProtectedResourceSession1").field(&self.0).finish()
    }
}
impl ID3D12ProtectedResourceSession1 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetStatusFence<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetStatusFence)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn GetSessionStatus(&self) -> D3D12_PROTECTED_SESSION_STATUS {
        (::windows::core::Vtable::vtable(self).base__.base__.GetSessionStatus)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> D3D12_PROTECTED_RESOURCE_SESSION_DESC {
        let mut result__: D3D12_PROTECTED_RESOURCE_SESSION_DESC = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
}
impl ::core::cmp::PartialEq for ID3D12ProtectedSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12ProtectedSession {}
impl ::core::fmt::Debug for ID3D12ProtectedSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12ProtectedSession").field(&self.0).finish()
    }
}
impl ID3D12ProtectedSession {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12QueryHeap {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12QueryHeap {}
impl ::core::fmt::Debug for ID3D12QueryHeap {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12QueryHeap").field(&self.0).finish()
    }
}
impl ID3D12QueryHeap {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12Resource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Resource {}
impl ::core::fmt::Debug for ID3D12Resource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Resource").field(&self.0).finish()
    }
}
impl ID3D12Resource {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12Resource1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Resource1 {}
impl ::core::fmt::Debug for ID3D12Resource1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Resource1").field(&self.0).finish()
    }
}
impl ID3D12Resource1 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn Map(&self, subresource: u32, preadrange: ::core::option::Option<*const D3D12_RANGE>, ppdata: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.Map)(::windows::core::Vtable::as_raw(self), subresource, ::core::mem::transmute(preadrange.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Unmap(&self, subresource: u32, pwrittenrange: ::core::option::Option<*const D3D12_RANGE>) {
        (::windows::core::Vtable::vtable(self).base__.Unmap)(::windows::core::Vtable::as_raw(self), subresource, ::core::mem::transmute(pwrittenrange.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self) -> D3D12_RESOURCE_DESC {
        let mut result__: D3D12_RESOURCE_DESC = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetGPUVirtualAddress(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.GetGPUVirtualAddress)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn WriteToSubresource(&self, dstsubresource: u32, pdstbox: ::core::option::Option<*const D3D12_BOX>, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.WriteToSubresource)(::windows::core::Vtable::as_raw(self), dstsubresource, ::core::mem::transmute(pdstbox.unwrap_or(::std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch).ok()
    }
    pub unsafe fn ReadFromSubresource(&self, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, srcsubresource: u32, psrcbox: ::core::option::Option<*const D3D12_BOX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.ReadFromSubresource)(::windows::core::Vtable::as_raw(self), pdstdata, dstrowpitch, dstdepthpitch, srcsubresource, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetHeapProperties(&self, pheapproperties: ::core::option::Option<*mut D3D12_HEAP_PROPERTIES>, pheapflags: ::core::option::Option<*mut D3D12_HEAP_FLAGS>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetHeapProperties)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pheapproperties.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pheapflags.unwrap_or(::std::ptr::null_mut()))).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12Resource2 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Resource2 {}
impl ::core::fmt::Debug for ID3D12Resource2 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Resource2").field(&self.0).finish()
    }
}
impl ID3D12Resource2 {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
    pub unsafe fn Map(&self, subresource: u32, preadrange: ::core::option::Option<*const D3D12_RANGE>, ppdata: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.Map)(::windows::core::Vtable::as_raw(self), subresource, ::core::mem::transmute(preadrange.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn Unmap(&self, subresource: u32, pwrittenrange: ::core::option::Option<*const D3D12_RANGE>) {
        (::windows::core::Vtable::vtable(self).base__.base__.Unmap)(::windows::core::Vtable::as_raw(self), subresource, ::core::mem::transmute(pwrittenrange.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetDesc(&self) -> D3D12_RESOURCE_DESC {
        let mut result__: D3D12_RESOURCE_DESC = ::core::mem::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDesc)(::windows::core::Vtable::as_raw(self), &mut result__);
        result__
    }
    pub unsafe fn GetGPUVirtualAddress(&self) -> u64 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetGPUVirtualAddress)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn WriteToSubresource(&self, dstsubresource: u32, pdstbox: ::core::option::Option<*const D3D12_BOX>, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.WriteToSubresource)(::windows::core::Vtable::as_raw(self), dstsubresource, ::core::mem::transmute(pdstbox.unwrap_or(::std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch).ok()
    }
    pub unsafe fn ReadFromSubresource(&self, pdstdata: *mut ::core::ffi::c_void, dstrowpitch: u32, dstdepthpitch: u32, srcsubresource: u32, psrcbox: ::core::option::Option<*const D3D12_BOX>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.ReadFromSubresource)(::windows::core::Vtable::as_raw(self), pdstdata, dstrowpitch, dstdepthpitch, srcsubresource, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn GetHeapProperties(&self, pheapproperties: ::core::option::Option<*mut D3D12_HEAP_PROPERTIES>, pheapflags: ::core::option::Option<*mut D3D12_HEAP_FLAGS>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetHeapProperties)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pheapproperties.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pheapflags.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetProtectedResourceSession<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.GetProtectedResourceSession)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12RootSignature {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12RootSignature {}
impl ::core::fmt::Debug for ID3D12RootSignature {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12RootSignature").field(&self.0).finish()
    }
}
impl ID3D12RootSignature {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12RootSignatureDeserializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12RootSignatureDeserializer {}
impl ::core::fmt::Debug for ID3D12RootSignatureDeserializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12RootSignatureDeserializer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12SDKConfiguration {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12SDKConfiguration {}
impl ::core::fmt::Debug for ID3D12SDKConfiguration {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12SDKConfiguration").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12ShaderCacheSession {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12ShaderCacheSession {}
impl ::core::fmt::Debug for ID3D12ShaderCacheSession {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12ShaderCacheSession").field(&self.0).finish()
    }
}
impl ID3D12ShaderCacheSession {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12ShaderReflection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12ShaderReflection {}
impl ::core::fmt::Debug for ID3D12ShaderReflection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12ShaderReflection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12ShaderReflectionConstantBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12ShaderReflectionConstantBuffer {}
impl ::core::fmt::Debug for ID3D12ShaderReflectionConstantBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12ShaderReflectionConstantBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12ShaderReflectionType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12ShaderReflectionType {}
impl ::core::fmt::Debug for ID3D12ShaderReflectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12ShaderReflectionType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12ShaderReflectionVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12ShaderReflectionVariable {}
impl ::core::fmt::Debug for ID3D12ShaderReflectionVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12ShaderReflectionVariable").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12SharingContract {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12SharingContract {}
impl ::core::fmt::Debug for ID3D12SharingContract {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12SharingContract").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12StateObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12StateObject {}
impl ::core::fmt::Debug for ID3D12StateObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12StateObject").field(&self.0).finish()
    }
}
impl ID3D12StateObject {
    pub unsafe fn GetPrivateData(&self, guid: *const ::windows::core::GUID, pdatasize: *mut u32, pdata: ::core::option::Option<*mut ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetPrivateData)(::windows::core::Vtable::as_raw(self), guid, pdatasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetPrivateData(&self, guid: *const ::windows::core::GUID, datasize: u32, pdata: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateData)(::windows::core::Vtable::as_raw(self), guid, datasize, ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null()))).ok()
    }
    pub unsafe fn SetPrivateDataInterface<P0>(&self, guid: *const ::windows::core::GUID, pdata: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::IUnknown>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetPrivateDataInterface)(::windows::core::Vtable::as_raw(self), guid, pdata.into().abi()).ok()
    }
    pub unsafe fn SetName<P0>(&self, name: P0) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCWSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.base__.SetName)(::windows::core::Vtable::as_raw(self), name.into().abi()).ok()
    }
    pub unsafe fn GetDevice<T>(&self, result__: *mut ::core::option::Option<T>) -> ::windows::core::Result<()>
    where
        T: ::windows::core::Interface,
    {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), &<T as ::windows::core::Interface>::IID, result__ as *mut _ as *mut _).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D12StateObjectProperties {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12StateObjectProperties {}
impl ::core::fmt::Debug for ID3D12StateObjectProperties {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12StateObjectProperties").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12SwapChainAssistant {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12SwapChainAssistant {}
impl ::core::fmt::Debug for ID3D12SwapChainAssistant {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12SwapChainAssistant").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12Tools {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12Tools {}
impl ::core::fmt::Debug for ID3D12Tools {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12Tools").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D12VersionedRootSignatureDeserializer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D12VersionedRootSignatureDeserializer {}
impl ::core::fmt::Debug for ID3D12VersionedRootSignatureDeserializer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D12VersionedRootSignatureDeserializer").field(&self.0).finish()
    }
}
