impl ::core::default::Default for D3D10_ASYNC_GETDATA_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_ASYNC_GETDATA_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_ASYNC_GETDATA_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_BIND_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_BIND_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_BIND_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_BLEND {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_BLEND {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_BLEND").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_BLEND_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_BLEND_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.AlphaToCoverageEnable == other.AlphaToCoverageEnable && self.BlendEnable == other.BlendEnable && self.SrcBlend == other.SrcBlend && self.DestBlend == other.DestBlend && self.BlendOp == other.BlendOp && self.SrcBlendAlpha == other.SrcBlendAlpha && self.DestBlendAlpha == other.DestBlendAlpha && self.BlendOpAlpha == other.BlendOpAlpha && self.RenderTargetWriteMask == other.RenderTargetWriteMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_BLEND_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_BLEND_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_BLEND_DESC").field("AlphaToCoverageEnable", &self.AlphaToCoverageEnable).field("BlendEnable", &self.BlendEnable).field("SrcBlend", &self.SrcBlend).field("DestBlend", &self.DestBlend).field("BlendOp", &self.BlendOp).field("SrcBlendAlpha", &self.SrcBlendAlpha).field("DestBlendAlpha", &self.DestBlendAlpha).field("BlendOpAlpha", &self.BlendOpAlpha).field("RenderTargetWriteMask", &self.RenderTargetWriteMask).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_BLEND_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.AlphaToCoverageEnable == other.AlphaToCoverageEnable && self.IndependentBlendEnable == other.IndependentBlendEnable && self.RenderTarget == other.RenderTarget
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_BLEND_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_BLEND_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_BLEND_DESC1").field("AlphaToCoverageEnable", &self.AlphaToCoverageEnable).field("IndependentBlendEnable", &self.IndependentBlendEnable).field("RenderTarget", &self.RenderTarget).finish()
    }
}
impl ::core::default::Default for D3D10_BLEND_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_BLEND_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_BLEND_OP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_BOX {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_BOX {
    fn eq(&self, other: &Self) -> bool {
        self.left == other.left && self.top == other.top && self.front == other.front && self.right == other.right && self.bottom == other.bottom && self.back == other.back
    }
}
impl ::core::cmp::Eq for D3D10_BOX {}
impl ::core::fmt::Debug for D3D10_BOX {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_BOX").field("left", &self.left).field("top", &self.top).field("front", &self.front).field("right", &self.right).field("bottom", &self.bottom).field("back", &self.back).finish()
    }
}
impl ::core::default::Default for D3D10_BUFFER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_BUFFER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.ByteWidth == other.ByteWidth && self.Usage == other.Usage && self.BindFlags == other.BindFlags && self.CPUAccessFlags == other.CPUAccessFlags && self.MiscFlags == other.MiscFlags
    }
}
impl ::core::cmp::Eq for D3D10_BUFFER_DESC {}
impl ::core::fmt::Debug for D3D10_BUFFER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_BUFFER_DESC").field("ByteWidth", &self.ByteWidth).field("Usage", &self.Usage).field("BindFlags", &self.BindFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("MiscFlags", &self.MiscFlags).finish()
    }
}
impl ::core::default::Default for D3D10_BUFFER_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D10_BUFFER_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D10_CLEAR_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_CLEAR_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_CLEAR_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_COLOR_WRITE_ENABLE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_COLOR_WRITE_ENABLE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_COLOR_WRITE_ENABLE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_COMPARISON_FUNC {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_COMPARISON_FUNC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_COMPARISON_FUNC").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_COUNTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_COUNTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_COUNTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_COUNTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_COUNTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Counter == other.Counter && self.MiscFlags == other.MiscFlags
    }
}
impl ::core::cmp::Eq for D3D10_COUNTER_DESC {}
impl ::core::fmt::Debug for D3D10_COUNTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_COUNTER_DESC").field("Counter", &self.Counter).field("MiscFlags", &self.MiscFlags).finish()
    }
}
impl ::core::default::Default for D3D10_COUNTER_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_COUNTER_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.LastDeviceDependentCounter == other.LastDeviceDependentCounter && self.NumSimultaneousCounters == other.NumSimultaneousCounters && self.NumDetectableParallelUnits == other.NumDetectableParallelUnits
    }
}
impl ::core::cmp::Eq for D3D10_COUNTER_INFO {}
impl ::core::fmt::Debug for D3D10_COUNTER_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_COUNTER_INFO").field("LastDeviceDependentCounter", &self.LastDeviceDependentCounter).field("NumSimultaneousCounters", &self.NumSimultaneousCounters).field("NumDetectableParallelUnits", &self.NumDetectableParallelUnits).finish()
    }
}
impl ::core::default::Default for D3D10_COUNTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_COUNTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_COUNTER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_CPU_ACCESS_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_CPU_ACCESS_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_CPU_ACCESS_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_CREATE_DEVICE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_CREATE_DEVICE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_CREATE_DEVICE_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_CULL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_CULL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_CULL_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_DEPTH_STENCILOP_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_DEPTH_STENCILOP_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.StencilFailOp == other.StencilFailOp && self.StencilDepthFailOp == other.StencilDepthFailOp && self.StencilPassOp == other.StencilPassOp && self.StencilFunc == other.StencilFunc
    }
}
impl ::core::cmp::Eq for D3D10_DEPTH_STENCILOP_DESC {}
impl ::core::fmt::Debug for D3D10_DEPTH_STENCILOP_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_DEPTH_STENCILOP_DESC").field("StencilFailOp", &self.StencilFailOp).field("StencilDepthFailOp", &self.StencilDepthFailOp).field("StencilPassOp", &self.StencilPassOp).field("StencilFunc", &self.StencilFunc).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_DEPTH_STENCIL_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_DEPTH_STENCIL_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.DepthEnable == other.DepthEnable && self.DepthWriteMask == other.DepthWriteMask && self.DepthFunc == other.DepthFunc && self.StencilEnable == other.StencilEnable && self.StencilReadMask == other.StencilReadMask && self.StencilWriteMask == other.StencilWriteMask && self.FrontFace == other.FrontFace && self.BackFace == other.BackFace
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_DEPTH_STENCIL_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_DEPTH_STENCIL_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_DEPTH_STENCIL_DESC").field("DepthEnable", &self.DepthEnable).field("DepthWriteMask", &self.DepthWriteMask).field("DepthFunc", &self.DepthFunc).field("StencilEnable", &self.StencilEnable).field("StencilReadMask", &self.StencilReadMask).field("StencilWriteMask", &self.StencilWriteMask).field("FrontFace", &self.FrontFace).field("BackFace", &self.BackFace).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D10_DEPTH_STENCIL_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D10_DEPTH_WRITE_MASK {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_DEPTH_WRITE_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_DEPTH_WRITE_MASK").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_DEVICE_STATE_TYPES {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_DEVICE_STATE_TYPES {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_DEVICE_STATE_TYPES").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_DRIVER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_DRIVER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_DRIVER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_DSV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_DSV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_DSV_DIMENSION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_EFFECT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_EFFECT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.IsChildEffect == other.IsChildEffect && self.ConstantBuffers == other.ConstantBuffers && self.SharedConstantBuffers == other.SharedConstantBuffers && self.GlobalVariables == other.GlobalVariables && self.SharedGlobalVariables == other.SharedGlobalVariables && self.Techniques == other.Techniques
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_EFFECT_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_EFFECT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_EFFECT_DESC").field("IsChildEffect", &self.IsChildEffect).field("ConstantBuffers", &self.ConstantBuffers).field("SharedConstantBuffers", &self.SharedConstantBuffers).field("GlobalVariables", &self.GlobalVariables).field("SharedGlobalVariables", &self.SharedGlobalVariables).field("Techniques", &self.Techniques).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_EFFECT_SHADER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_EFFECT_SHADER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.pInputSignature == other.pInputSignature && self.IsInline == other.IsInline && self.pBytecode == other.pBytecode && self.BytecodeLength == other.BytecodeLength && self.SODecl == other.SODecl && self.NumInputSignatureEntries == other.NumInputSignatureEntries && self.NumOutputSignatureEntries == other.NumOutputSignatureEntries
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_EFFECT_SHADER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_EFFECT_SHADER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_EFFECT_SHADER_DESC").field("pInputSignature", &self.pInputSignature).field("IsInline", &self.IsInline).field("pBytecode", &self.pBytecode).field("BytecodeLength", &self.BytecodeLength).field("SODecl", &self.SODecl).field("NumInputSignatureEntries", &self.NumInputSignatureEntries).field("NumOutputSignatureEntries", &self.NumOutputSignatureEntries).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_EFFECT_TYPE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_EFFECT_TYPE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.TypeName == other.TypeName && self.Class == other.Class && self.Type == other.Type && self.Elements == other.Elements && self.Members == other.Members && self.Rows == other.Rows && self.Columns == other.Columns && self.PackedSize == other.PackedSize && self.UnpackedSize == other.UnpackedSize && self.Stride == other.Stride
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_EFFECT_TYPE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_EFFECT_TYPE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_EFFECT_TYPE_DESC").field("TypeName", &self.TypeName).field("Class", &self.Class).field("Type", &self.Type).field("Elements", &self.Elements).field("Members", &self.Members).field("Rows", &self.Rows).field("Columns", &self.Columns).field("PackedSize", &self.PackedSize).field("UnpackedSize", &self.UnpackedSize).field("Stride", &self.Stride).finish()
    }
}
impl ::core::default::Default for D3D10_EFFECT_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_EFFECT_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Semantic == other.Semantic && self.Flags == other.Flags && self.Annotations == other.Annotations && self.BufferOffset == other.BufferOffset && self.ExplicitBindPoint == other.ExplicitBindPoint
    }
}
impl ::core::cmp::Eq for D3D10_EFFECT_VARIABLE_DESC {}
impl ::core::fmt::Debug for D3D10_EFFECT_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_EFFECT_VARIABLE_DESC").field("Name", &self.Name).field("Semantic", &self.Semantic).field("Flags", &self.Flags).field("Annotations", &self.Annotations).field("BufferOffset", &self.BufferOffset).field("ExplicitBindPoint", &self.ExplicitBindPoint).finish()
    }
}
impl ::core::default::Default for D3D10_FEATURE_LEVEL1 {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_FEATURE_LEVEL1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_FEATURE_LEVEL1").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_FILL_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_FILL_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_FILL_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_FILTER {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_FILTER").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_FILTER_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_FILTER_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_FILTER_TYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_FORMAT_SUPPORT {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_FORMAT_SUPPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_FORMAT_SUPPORT").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_INFO_QUEUE_FILTER {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_INFO_QUEUE_FILTER {
    fn eq(&self, other: &Self) -> bool {
        self.AllowList == other.AllowList && self.DenyList == other.DenyList
    }
}
impl ::core::cmp::Eq for D3D10_INFO_QUEUE_FILTER {}
impl ::core::fmt::Debug for D3D10_INFO_QUEUE_FILTER {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_INFO_QUEUE_FILTER").field("AllowList", &self.AllowList).field("DenyList", &self.DenyList).finish()
    }
}
impl ::core::default::Default for D3D10_INFO_QUEUE_FILTER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_INFO_QUEUE_FILTER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.NumCategories == other.NumCategories && self.pCategoryList == other.pCategoryList && self.NumSeverities == other.NumSeverities && self.pSeverityList == other.pSeverityList && self.NumIDs == other.NumIDs && self.pIDList == other.pIDList
    }
}
impl ::core::cmp::Eq for D3D10_INFO_QUEUE_FILTER_DESC {}
impl ::core::fmt::Debug for D3D10_INFO_QUEUE_FILTER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_INFO_QUEUE_FILTER_DESC").field("NumCategories", &self.NumCategories).field("pCategoryList", &self.pCategoryList).field("NumSeverities", &self.NumSeverities).field("pSeverityList", &self.pSeverityList).field("NumIDs", &self.NumIDs).field("pIDList", &self.pIDList).finish()
    }
}
impl ::core::default::Default for D3D10_INPUT_CLASSIFICATION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_INPUT_CLASSIFICATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_INPUT_CLASSIFICATION").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D10_INPUT_ELEMENT_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D10_INPUT_ELEMENT_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.SemanticName == other.SemanticName && self.SemanticIndex == other.SemanticIndex && self.Format == other.Format && self.InputSlot == other.InputSlot && self.AlignedByteOffset == other.AlignedByteOffset && self.InputSlotClass == other.InputSlotClass && self.InstanceDataStepRate == other.InstanceDataStepRate
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D10_INPUT_ELEMENT_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D10_INPUT_ELEMENT_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_INPUT_ELEMENT_DESC").field("SemanticName", &self.SemanticName).field("SemanticIndex", &self.SemanticIndex).field("Format", &self.Format).field("InputSlot", &self.InputSlot).field("AlignedByteOffset", &self.AlignedByteOffset).field("InputSlotClass", &self.InputSlotClass).field("InstanceDataStepRate", &self.InstanceDataStepRate).finish()
    }
}
impl ::core::default::Default for D3D10_MAP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_MAP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_MAP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_MAPPED_TEXTURE2D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_MAPPED_TEXTURE2D {
    fn eq(&self, other: &Self) -> bool {
        self.pData == other.pData && self.RowPitch == other.RowPitch
    }
}
impl ::core::cmp::Eq for D3D10_MAPPED_TEXTURE2D {}
impl ::core::fmt::Debug for D3D10_MAPPED_TEXTURE2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_MAPPED_TEXTURE2D").field("pData", &self.pData).field("RowPitch", &self.RowPitch).finish()
    }
}
impl ::core::default::Default for D3D10_MAPPED_TEXTURE3D {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_MAPPED_TEXTURE3D {
    fn eq(&self, other: &Self) -> bool {
        self.pData == other.pData && self.RowPitch == other.RowPitch && self.DepthPitch == other.DepthPitch
    }
}
impl ::core::cmp::Eq for D3D10_MAPPED_TEXTURE3D {}
impl ::core::fmt::Debug for D3D10_MAPPED_TEXTURE3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_MAPPED_TEXTURE3D").field("pData", &self.pData).field("RowPitch", &self.RowPitch).field("DepthPitch", &self.DepthPitch).finish()
    }
}
impl ::core::default::Default for D3D10_MAP_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_MAP_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_MAP_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_MESSAGE {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_MESSAGE {
    fn eq(&self, other: &Self) -> bool {
        self.Category == other.Category && self.Severity == other.Severity && self.ID == other.ID && self.pDescription == other.pDescription && self.DescriptionByteLength == other.DescriptionByteLength
    }
}
impl ::core::cmp::Eq for D3D10_MESSAGE {}
impl ::core::fmt::Debug for D3D10_MESSAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_MESSAGE").field("Category", &self.Category).field("Severity", &self.Severity).field("ID", &self.ID).field("pDescription", &self.pDescription).field("DescriptionByteLength", &self.DescriptionByteLength).finish()
    }
}
impl ::core::default::Default for D3D10_MESSAGE_CATEGORY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_MESSAGE_CATEGORY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_MESSAGE_CATEGORY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_MESSAGE_ID {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_MESSAGE_ID {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_MESSAGE_ID").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_MESSAGE_SEVERITY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_MESSAGE_SEVERITY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_MESSAGE_SEVERITY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_PASS_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_PASS_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Annotations == other.Annotations && self.pIAInputSignature == other.pIAInputSignature && self.IAInputSignatureSize == other.IAInputSignatureSize && self.StencilRef == other.StencilRef && self.SampleMask == other.SampleMask && self.BlendFactor == other.BlendFactor
    }
}
impl ::core::cmp::Eq for D3D10_PASS_DESC {}
impl ::core::fmt::Debug for D3D10_PASS_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_PASS_DESC").field("Name", &self.Name).field("Annotations", &self.Annotations).field("pIAInputSignature", &self.pIAInputSignature).field("IAInputSignatureSize", &self.IAInputSignatureSize).field("StencilRef", &self.StencilRef).field("SampleMask", &self.SampleMask).field("BlendFactor", &self.BlendFactor).finish()
    }
}
impl ::core::default::Default for D3D10_PASS_SHADER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_PASS_SHADER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.pShaderVariable == other.pShaderVariable && self.ShaderIndex == other.ShaderIndex
    }
}
impl ::core::cmp::Eq for D3D10_PASS_SHADER_DESC {}
impl ::core::fmt::Debug for D3D10_PASS_SHADER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_PASS_SHADER_DESC").field("pShaderVariable", &self.pShaderVariable).field("ShaderIndex", &self.ShaderIndex).finish()
    }
}
impl ::core::default::Default for D3D10_QUERY {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_QUERY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_QUERY").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_QUERY_DATA_PIPELINE_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_QUERY_DATA_PIPELINE_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.IAVertices == other.IAVertices && self.IAPrimitives == other.IAPrimitives && self.VSInvocations == other.VSInvocations && self.GSInvocations == other.GSInvocations && self.GSPrimitives == other.GSPrimitives && self.CInvocations == other.CInvocations && self.CPrimitives == other.CPrimitives && self.PSInvocations == other.PSInvocations
    }
}
impl ::core::cmp::Eq for D3D10_QUERY_DATA_PIPELINE_STATISTICS {}
impl ::core::fmt::Debug for D3D10_QUERY_DATA_PIPELINE_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_QUERY_DATA_PIPELINE_STATISTICS").field("IAVertices", &self.IAVertices).field("IAPrimitives", &self.IAPrimitives).field("VSInvocations", &self.VSInvocations).field("GSInvocations", &self.GSInvocations).field("GSPrimitives", &self.GSPrimitives).field("CInvocations", &self.CInvocations).field("CPrimitives", &self.CPrimitives).field("PSInvocations", &self.PSInvocations).finish()
    }
}
impl ::core::default::Default for D3D10_QUERY_DATA_SO_STATISTICS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_QUERY_DATA_SO_STATISTICS {
    fn eq(&self, other: &Self) -> bool {
        self.NumPrimitivesWritten == other.NumPrimitivesWritten && self.PrimitivesStorageNeeded == other.PrimitivesStorageNeeded
    }
}
impl ::core::cmp::Eq for D3D10_QUERY_DATA_SO_STATISTICS {}
impl ::core::fmt::Debug for D3D10_QUERY_DATA_SO_STATISTICS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_QUERY_DATA_SO_STATISTICS").field("NumPrimitivesWritten", &self.NumPrimitivesWritten).field("PrimitivesStorageNeeded", &self.PrimitivesStorageNeeded).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn eq(&self, other: &Self) -> bool {
        self.Frequency == other.Frequency && self.Disjoint == other.Disjoint
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_QUERY_DATA_TIMESTAMP_DISJOINT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_QUERY_DATA_TIMESTAMP_DISJOINT").field("Frequency", &self.Frequency).field("Disjoint", &self.Disjoint).finish()
    }
}
impl ::core::default::Default for D3D10_QUERY_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_QUERY_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Query == other.Query && self.MiscFlags == other.MiscFlags
    }
}
impl ::core::cmp::Eq for D3D10_QUERY_DESC {}
impl ::core::fmt::Debug for D3D10_QUERY_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_QUERY_DESC").field("Query", &self.Query).field("MiscFlags", &self.MiscFlags).finish()
    }
}
impl ::core::default::Default for D3D10_QUERY_MISC_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_QUERY_MISC_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_QUERY_MISC_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_RAISE_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_RAISE_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_RAISE_FLAG").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_RASTERIZER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_RASTERIZER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.FillMode == other.FillMode && self.CullMode == other.CullMode && self.FrontCounterClockwise == other.FrontCounterClockwise && self.DepthBias == other.DepthBias && self.DepthBiasClamp == other.DepthBiasClamp && self.SlopeScaledDepthBias == other.SlopeScaledDepthBias && self.DepthClipEnable == other.DepthClipEnable && self.ScissorEnable == other.ScissorEnable && self.MultisampleEnable == other.MultisampleEnable && self.AntialiasedLineEnable == other.AntialiasedLineEnable
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_RASTERIZER_DESC {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_RASTERIZER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_RASTERIZER_DESC")
            .field("FillMode", &self.FillMode)
            .field("CullMode", &self.CullMode)
            .field("FrontCounterClockwise", &self.FrontCounterClockwise)
            .field("DepthBias", &self.DepthBias)
            .field("DepthBiasClamp", &self.DepthBiasClamp)
            .field("SlopeScaledDepthBias", &self.SlopeScaledDepthBias)
            .field("DepthClipEnable", &self.DepthClipEnable)
            .field("ScissorEnable", &self.ScissorEnable)
            .field("MultisampleEnable", &self.MultisampleEnable)
            .field("AntialiasedLineEnable", &self.AntialiasedLineEnable)
            .finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_RENDER_TARGET_BLEND_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_RENDER_TARGET_BLEND_DESC1 {
    fn eq(&self, other: &Self) -> bool {
        self.BlendEnable == other.BlendEnable && self.SrcBlend == other.SrcBlend && self.DestBlend == other.DestBlend && self.BlendOp == other.BlendOp && self.SrcBlendAlpha == other.SrcBlendAlpha && self.DestBlendAlpha == other.DestBlendAlpha && self.BlendOpAlpha == other.BlendOpAlpha && self.RenderTargetWriteMask == other.RenderTargetWriteMask
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_RENDER_TARGET_BLEND_DESC1 {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_RENDER_TARGET_BLEND_DESC1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_RENDER_TARGET_BLEND_DESC1").field("BlendEnable", &self.BlendEnable).field("SrcBlend", &self.SrcBlend).field("DestBlend", &self.DestBlend).field("BlendOp", &self.BlendOp).field("SrcBlendAlpha", &self.SrcBlendAlpha).field("DestBlendAlpha", &self.DestBlendAlpha).field("BlendOpAlpha", &self.BlendOpAlpha).field("RenderTargetWriteMask", &self.RenderTargetWriteMask).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D10_RENDER_TARGET_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::default::Default for D3D10_RESOURCE_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_RESOURCE_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_RESOURCE_DIMENSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_RESOURCE_MISC_FLAG {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_RESOURCE_MISC_FLAG {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_RESOURCE_MISC_FLAG").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_RTV_DIMENSION {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_RTV_DIMENSION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_RTV_DIMENSION").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_SAMPLER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_SAMPLER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Filter == other.Filter && self.AddressU == other.AddressU && self.AddressV == other.AddressV && self.AddressW == other.AddressW && self.MipLODBias == other.MipLODBias && self.MaxAnisotropy == other.MaxAnisotropy && self.ComparisonFunc == other.ComparisonFunc && self.BorderColor == other.BorderColor && self.MinLOD == other.MinLOD && self.MaxLOD == other.MaxLOD
    }
}
impl ::core::cmp::Eq for D3D10_SAMPLER_DESC {}
impl ::core::fmt::Debug for D3D10_SAMPLER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SAMPLER_DESC").field("Filter", &self.Filter).field("AddressU", &self.AddressU).field("AddressV", &self.AddressV).field("AddressW", &self.AddressW).field("MipLODBias", &self.MipLODBias).field("MaxAnisotropy", &self.MaxAnisotropy).field("ComparisonFunc", &self.ComparisonFunc).field("BorderColor", &self.BorderColor).field("MinLOD", &self.MinLOD).field("MaxLOD", &self.MaxLOD).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_SHADER_BUFFER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_SHADER_BUFFER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Type == other.Type && self.Variables == other.Variables && self.Size == other.Size && self.uFlags == other.uFlags
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_SHADER_BUFFER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_SHADER_BUFFER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_BUFFER_DESC").field("Name", &self.Name).field("Type", &self.Type).field("Variables", &self.Variables).field("Size", &self.Size).field("uFlags", &self.uFlags).finish()
    }
}
impl ::core::default::Default for D3D10_SHADER_DEBUG_FILE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_FILE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.FileName == other.FileName && self.FileNameLen == other.FileNameLen && self.FileData == other.FileData && self.FileLen == other.FileLen
    }
}
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_FILE_INFO {}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_FILE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_FILE_INFO").field("FileName", &self.FileName).field("FileNameLen", &self.FileNameLen).field("FileData", &self.FileData).field("FileLen", &self.FileLen).finish()
    }
}
impl ::core::default::Default for D3D10_SHADER_DEBUG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Size == other.Size
            && self.Creator == other.Creator
            && self.EntrypointName == other.EntrypointName
            && self.ShaderTarget == other.ShaderTarget
            && self.CompileFlags == other.CompileFlags
            && self.Files == other.Files
            && self.FileInfo == other.FileInfo
            && self.Instructions == other.Instructions
            && self.InstructionInfo == other.InstructionInfo
            && self.Variables == other.Variables
            && self.VariableInfo == other.VariableInfo
            && self.InputVariables == other.InputVariables
            && self.InputVariableInfo == other.InputVariableInfo
            && self.Tokens == other.Tokens
            && self.TokenInfo == other.TokenInfo
            && self.Scopes == other.Scopes
            && self.ScopeInfo == other.ScopeInfo
            && self.ScopeVariables == other.ScopeVariables
            && self.ScopeVariableInfo == other.ScopeVariableInfo
            && self.UintOffset == other.UintOffset
            && self.StringOffset == other.StringOffset
    }
}
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_INFO {}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_INFO")
            .field("Size", &self.Size)
            .field("Creator", &self.Creator)
            .field("EntrypointName", &self.EntrypointName)
            .field("ShaderTarget", &self.ShaderTarget)
            .field("CompileFlags", &self.CompileFlags)
            .field("Files", &self.Files)
            .field("FileInfo", &self.FileInfo)
            .field("Instructions", &self.Instructions)
            .field("InstructionInfo", &self.InstructionInfo)
            .field("Variables", &self.Variables)
            .field("VariableInfo", &self.VariableInfo)
            .field("InputVariables", &self.InputVariables)
            .field("InputVariableInfo", &self.InputVariableInfo)
            .field("Tokens", &self.Tokens)
            .field("TokenInfo", &self.TokenInfo)
            .field("Scopes", &self.Scopes)
            .field("ScopeInfo", &self.ScopeInfo)
            .field("ScopeVariables", &self.ScopeVariables)
            .field("ScopeVariableInfo", &self.ScopeVariableInfo)
            .field("UintOffset", &self.UintOffset)
            .field("StringOffset", &self.StringOffset)
            .finish()
    }
}
impl ::core::default::Default for D3D10_SHADER_DEBUG_INPUT_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_INPUT_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Var == other.Var && self.InitialRegisterSet == other.InitialRegisterSet && self.InitialBank == other.InitialBank && self.InitialRegister == other.InitialRegister && self.InitialComponent == other.InitialComponent && self.InitialValue == other.InitialValue
    }
}
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_INPUT_INFO {}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_INPUT_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_INPUT_INFO").field("Var", &self.Var).field("InitialRegisterSet", &self.InitialRegisterSet).field("InitialBank", &self.InitialBank).field("InitialRegister", &self.InitialRegister).field("InitialComponent", &self.InitialComponent).field("InitialValue", &self.InitialValue).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_SHADER_DEBUG_INST_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_INST_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.Id == other.Id && self.Opcode == other.Opcode && self.uOutputs == other.uOutputs && self.pOutputs == other.pOutputs && self.TokenId == other.TokenId && self.NestingLevel == other.NestingLevel && self.Scopes == other.Scopes && self.ScopeInfo == other.ScopeInfo && self.AccessedVars == other.AccessedVars && self.AccessedVarsInfo == other.AccessedVarsInfo
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_INST_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_INST_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_INST_INFO").field("Id", &self.Id).field("Opcode", &self.Opcode).field("uOutputs", &self.uOutputs).field("pOutputs", &self.pOutputs).field("TokenId", &self.TokenId).field("NestingLevel", &self.NestingLevel).field("Scopes", &self.Scopes).field("ScopeInfo", &self.ScopeInfo).field("AccessedVars", &self.AccessedVars).field("AccessedVarsInfo", &self.AccessedVarsInfo).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.OutputRegisterSet == other.OutputRegisterSet && self.OutputReg == other.OutputReg && self.TempArrayReg == other.TempArrayReg && self.OutputComponents == other.OutputComponents && self.OutputVars == other.OutputVars && self.IndexReg == other.IndexReg && self.IndexComp == other.IndexComp
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_OUTPUTREG_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_OUTPUTREG_INFO").field("OutputRegisterSet", &self.OutputRegisterSet).field("OutputReg", &self.OutputReg).field("TempArrayReg", &self.TempArrayReg).field("OutputComponents", &self.OutputComponents).field("OutputVars", &self.OutputVars).field("IndexReg", &self.IndexReg).field("IndexComp", &self.IndexComp).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for D3D10_SHADER_DEBUG_OUTPUTVAR {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_OUTPUTVAR {
    fn eq(&self, other: &Self) -> bool {
        self.Var == other.Var && self.uValueMin == other.uValueMin && self.uValueMax == other.uValueMax && self.iValueMin == other.iValueMin && self.iValueMax == other.iValueMax && self.fValueMin == other.fValueMin && self.fValueMax == other.fValueMax && self.bNaNPossible == other.bNaNPossible && self.bInfPossible == other.bInfPossible
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_OUTPUTVAR {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_OUTPUTVAR {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_OUTPUTVAR").field("Var", &self.Var).field("uValueMin", &self.uValueMin).field("uValueMax", &self.uValueMax).field("iValueMin", &self.iValueMin).field("iValueMax", &self.iValueMax).field("fValueMin", &self.fValueMin).field("fValueMax", &self.fValueMax).field("bNaNPossible", &self.bNaNPossible).field("bInfPossible", &self.bInfPossible).finish()
    }
}
impl ::core::default::Default for D3D10_SHADER_DEBUG_REGTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_REGTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_SHADER_DEBUG_REGTYPE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_SHADER_DEBUG_SCOPETYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_SCOPETYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_SHADER_DEBUG_SCOPETYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.TokenId == other.TokenId && self.VarType == other.VarType && self.Class == other.Class && self.Rows == other.Rows && self.Columns == other.Columns && self.StructMemberScope == other.StructMemberScope && self.uArrayIndices == other.uArrayIndices && self.ArrayElements == other.ArrayElements && self.ArrayStrides == other.ArrayStrides && self.uVariables == other.uVariables && self.uFirstVariable == other.uFirstVariable
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_SCOPEVAR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_SCOPEVAR_INFO").field("TokenId", &self.TokenId).field("VarType", &self.VarType).field("Class", &self.Class).field("Rows", &self.Rows).field("Columns", &self.Columns).field("StructMemberScope", &self.StructMemberScope).field("uArrayIndices", &self.uArrayIndices).field("ArrayElements", &self.ArrayElements).field("ArrayStrides", &self.ArrayStrides).field("uVariables", &self.uVariables).field("uFirstVariable", &self.uFirstVariable).finish()
    }
}
impl ::core::default::Default for D3D10_SHADER_DEBUG_SCOPE_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_SCOPE_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.ScopeType == other.ScopeType && self.Name == other.Name && self.uNameLen == other.uNameLen && self.uVariables == other.uVariables && self.VariableData == other.VariableData
    }
}
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_SCOPE_INFO {}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_SCOPE_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_SCOPE_INFO").field("ScopeType", &self.ScopeType).field("Name", &self.Name).field("uNameLen", &self.uNameLen).field("uVariables", &self.uVariables).field("VariableData", &self.VariableData).finish()
    }
}
impl ::core::default::Default for D3D10_SHADER_DEBUG_TOKEN_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_TOKEN_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.File == other.File && self.Line == other.Line && self.Column == other.Column && self.TokenLength == other.TokenLength && self.TokenId == other.TokenId
    }
}
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_TOKEN_INFO {}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_TOKEN_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_TOKEN_INFO").field("File", &self.File).field("Line", &self.Line).field("Column", &self.Column).field("TokenLength", &self.TokenLength).field("TokenId", &self.TokenId).finish()
    }
}
impl ::core::default::Default for D3D10_SHADER_DEBUG_VARTYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_VARTYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_SHADER_DEBUG_VARTYPE").field(&self.0).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_SHADER_DEBUG_VAR_INFO {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_SHADER_DEBUG_VAR_INFO {
    fn eq(&self, other: &Self) -> bool {
        self.TokenId == other.TokenId && self.Type == other.Type && self.Register == other.Register && self.Component == other.Component && self.ScopeVar == other.ScopeVar && self.ScopeVarOffset == other.ScopeVarOffset
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_SHADER_DEBUG_VAR_INFO {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_SHADER_DEBUG_VAR_INFO {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DEBUG_VAR_INFO").field("TokenId", &self.TokenId).field("Type", &self.Type).field("Register", &self.Register).field("Component", &self.Component).field("ScopeVar", &self.ScopeVar).field("ScopeVarOffset", &self.ScopeVarOffset).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_SHADER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_SHADER_DESC {
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
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_SHADER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_SHADER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_DESC")
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
            .finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_SHADER_INPUT_BIND_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_SHADER_INPUT_BIND_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Type == other.Type && self.BindPoint == other.BindPoint && self.BindCount == other.BindCount && self.uFlags == other.uFlags && self.ReturnType == other.ReturnType && self.Dimension == other.Dimension && self.NumSamples == other.NumSamples
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_SHADER_INPUT_BIND_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_SHADER_INPUT_BIND_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_INPUT_BIND_DESC").field("Name", &self.Name).field("Type", &self.Type).field("BindPoint", &self.BindPoint).field("BindCount", &self.BindCount).field("uFlags", &self.uFlags).field("ReturnType", &self.ReturnType).field("Dimension", &self.Dimension).field("NumSamples", &self.NumSamples).finish()
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D10_SHADER_RESOURCE_VIEW_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
impl ::core::default::Default for D3D10_SHADER_RESOURCE_VIEW_DESC1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_SHADER_TYPE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_SHADER_TYPE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Class == other.Class && self.Type == other.Type && self.Rows == other.Rows && self.Columns == other.Columns && self.Elements == other.Elements && self.Members == other.Members && self.Offset == other.Offset
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_SHADER_TYPE_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_SHADER_TYPE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_TYPE_DESC").field("Class", &self.Class).field("Type", &self.Type).field("Rows", &self.Rows).field("Columns", &self.Columns).field("Elements", &self.Elements).field("Members", &self.Members).field("Offset", &self.Offset).finish()
    }
}
impl ::core::default::Default for D3D10_SHADER_VARIABLE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_SHADER_VARIABLE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.StartOffset == other.StartOffset && self.Size == other.Size && self.uFlags == other.uFlags && self.DefaultValue == other.DefaultValue
    }
}
impl ::core::cmp::Eq for D3D10_SHADER_VARIABLE_DESC {}
impl ::core::fmt::Debug for D3D10_SHADER_VARIABLE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SHADER_VARIABLE_DESC").field("Name", &self.Name).field("StartOffset", &self.StartOffset).field("Size", &self.Size).field("uFlags", &self.uFlags).field("DefaultValue", &self.DefaultValue).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::default::Default for D3D10_SIGNATURE_PARAMETER_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::PartialEq for D3D10_SIGNATURE_PARAMETER_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.SemanticName == other.SemanticName && self.SemanticIndex == other.SemanticIndex && self.Register == other.Register && self.SystemValueType == other.SystemValueType && self.ComponentType == other.ComponentType && self.Mask == other.Mask && self.ReadWriteMask == other.ReadWriteMask
    }
}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::cmp::Eq for D3D10_SIGNATURE_PARAMETER_DESC {}
#[cfg(feature = "Win32_Graphics_Direct3D")]
impl ::core::fmt::Debug for D3D10_SIGNATURE_PARAMETER_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SIGNATURE_PARAMETER_DESC").field("SemanticName", &self.SemanticName).field("SemanticIndex", &self.SemanticIndex).field("Register", &self.Register).field("SystemValueType", &self.SystemValueType).field("ComponentType", &self.ComponentType).field("Mask", &self.Mask).field("ReadWriteMask", &self.ReadWriteMask).finish()
    }
}
impl ::core::default::Default for D3D10_SO_DECLARATION_ENTRY {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_SO_DECLARATION_ENTRY {
    fn eq(&self, other: &Self) -> bool {
        self.SemanticName == other.SemanticName && self.SemanticIndex == other.SemanticIndex && self.StartComponent == other.StartComponent && self.ComponentCount == other.ComponentCount && self.OutputSlot == other.OutputSlot
    }
}
impl ::core::cmp::Eq for D3D10_SO_DECLARATION_ENTRY {}
impl ::core::fmt::Debug for D3D10_SO_DECLARATION_ENTRY {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SO_DECLARATION_ENTRY").field("SemanticName", &self.SemanticName).field("SemanticIndex", &self.SemanticIndex).field("StartComponent", &self.StartComponent).field("ComponentCount", &self.ComponentCount).field("OutputSlot", &self.OutputSlot).finish()
    }
}
impl ::core::default::Default for D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_STANDARD_MULTISAMPLE_QUALITY_LEVELS").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_STATE_BLOCK_MASK {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_STATE_BLOCK_MASK {
    fn eq(&self, other: &Self) -> bool {
        self.VS == other.VS
            && self.VSSamplers == other.VSSamplers
            && self.VSShaderResources == other.VSShaderResources
            && self.VSConstantBuffers == other.VSConstantBuffers
            && self.GS == other.GS
            && self.GSSamplers == other.GSSamplers
            && self.GSShaderResources == other.GSShaderResources
            && self.GSConstantBuffers == other.GSConstantBuffers
            && self.PS == other.PS
            && self.PSSamplers == other.PSSamplers
            && self.PSShaderResources == other.PSShaderResources
            && self.PSConstantBuffers == other.PSConstantBuffers
            && self.IAVertexBuffers == other.IAVertexBuffers
            && self.IAIndexBuffer == other.IAIndexBuffer
            && self.IAInputLayout == other.IAInputLayout
            && self.IAPrimitiveTopology == other.IAPrimitiveTopology
            && self.OMRenderTargets == other.OMRenderTargets
            && self.OMDepthStencilState == other.OMDepthStencilState
            && self.OMBlendState == other.OMBlendState
            && self.RSViewports == other.RSViewports
            && self.RSScissorRects == other.RSScissorRects
            && self.RSRasterizerState == other.RSRasterizerState
            && self.SOBuffers == other.SOBuffers
            && self.Predication == other.Predication
    }
}
impl ::core::cmp::Eq for D3D10_STATE_BLOCK_MASK {}
impl ::core::fmt::Debug for D3D10_STATE_BLOCK_MASK {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_STATE_BLOCK_MASK")
            .field("VS", &self.VS)
            .field("VSSamplers", &self.VSSamplers)
            .field("VSShaderResources", &self.VSShaderResources)
            .field("VSConstantBuffers", &self.VSConstantBuffers)
            .field("GS", &self.GS)
            .field("GSSamplers", &self.GSSamplers)
            .field("GSShaderResources", &self.GSShaderResources)
            .field("GSConstantBuffers", &self.GSConstantBuffers)
            .field("PS", &self.PS)
            .field("PSSamplers", &self.PSSamplers)
            .field("PSShaderResources", &self.PSShaderResources)
            .field("PSConstantBuffers", &self.PSConstantBuffers)
            .field("IAVertexBuffers", &self.IAVertexBuffers)
            .field("IAIndexBuffer", &self.IAIndexBuffer)
            .field("IAInputLayout", &self.IAInputLayout)
            .field("IAPrimitiveTopology", &self.IAPrimitiveTopology)
            .field("OMRenderTargets", &self.OMRenderTargets)
            .field("OMDepthStencilState", &self.OMDepthStencilState)
            .field("OMBlendState", &self.OMBlendState)
            .field("RSViewports", &self.RSViewports)
            .field("RSScissorRects", &self.RSScissorRects)
            .field("RSRasterizerState", &self.RSRasterizerState)
            .field("SOBuffers", &self.SOBuffers)
            .field("Predication", &self.Predication)
            .finish()
    }
}
impl ::core::default::Default for D3D10_STENCIL_OP {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_STENCIL_OP {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_STENCIL_OP").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_SUBRESOURCE_DATA {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_SUBRESOURCE_DATA {
    fn eq(&self, other: &Self) -> bool {
        self.pSysMem == other.pSysMem && self.SysMemPitch == other.SysMemPitch && self.SysMemSlicePitch == other.SysMemSlicePitch
    }
}
impl ::core::cmp::Eq for D3D10_SUBRESOURCE_DATA {}
impl ::core::fmt::Debug for D3D10_SUBRESOURCE_DATA {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_SUBRESOURCE_DATA").field("pSysMem", &self.pSysMem).field("SysMemPitch", &self.SysMemPitch).field("SysMemSlicePitch", &self.SysMemSlicePitch).finish()
    }
}
impl ::core::default::Default for D3D10_TECHNIQUE_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TECHNIQUE_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Name == other.Name && self.Passes == other.Passes && self.Annotations == other.Annotations
    }
}
impl ::core::cmp::Eq for D3D10_TECHNIQUE_DESC {}
impl ::core::fmt::Debug for D3D10_TECHNIQUE_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TECHNIQUE_DESC").field("Name", &self.Name).field("Passes", &self.Passes).field("Annotations", &self.Annotations).finish()
    }
}
impl ::core::default::Default for D3D10_TEX1D_ARRAY_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX1D_ARRAY_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D10_TEX1D_ARRAY_DSV {}
impl ::core::fmt::Debug for D3D10_TEX1D_ARRAY_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX1D_ARRAY_DSV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D10_TEX1D_ARRAY_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX1D_ARRAY_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D10_TEX1D_ARRAY_RTV {}
impl ::core::fmt::Debug for D3D10_TEX1D_ARRAY_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX1D_ARRAY_RTV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D10_TEX1D_ARRAY_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX1D_ARRAY_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D10_TEX1D_ARRAY_SRV {}
impl ::core::fmt::Debug for D3D10_TEX1D_ARRAY_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX1D_ARRAY_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D10_TEX1D_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX1D_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D10_TEX1D_DSV {}
impl ::core::fmt::Debug for D3D10_TEX1D_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX1D_DSV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D10_TEX1D_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX1D_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D10_TEX1D_RTV {}
impl ::core::fmt::Debug for D3D10_TEX1D_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX1D_RTV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D10_TEX1D_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX1D_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels
    }
}
impl ::core::cmp::Eq for D3D10_TEX1D_SRV {}
impl ::core::fmt::Debug for D3D10_TEX1D_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX1D_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).finish()
    }
}
impl ::core::default::Default for D3D10_TEX2DMS_ARRAY_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX2DMS_ARRAY_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D10_TEX2DMS_ARRAY_DSV {}
impl ::core::fmt::Debug for D3D10_TEX2DMS_ARRAY_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2DMS_ARRAY_DSV").field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D10_TEX2DMS_ARRAY_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX2DMS_ARRAY_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D10_TEX2DMS_ARRAY_RTV {}
impl ::core::fmt::Debug for D3D10_TEX2DMS_ARRAY_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2DMS_ARRAY_RTV").field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D10_TEX2DMS_ARRAY_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX2DMS_ARRAY_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D10_TEX2DMS_ARRAY_SRV {}
impl ::core::fmt::Debug for D3D10_TEX2DMS_ARRAY_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2DMS_ARRAY_SRV").field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D10_TEX2DMS_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX2DMS_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.UnusedField_NothingToDefine == other.UnusedField_NothingToDefine
    }
}
impl ::core::cmp::Eq for D3D10_TEX2DMS_DSV {}
impl ::core::fmt::Debug for D3D10_TEX2DMS_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2DMS_DSV").field("UnusedField_NothingToDefine", &self.UnusedField_NothingToDefine).finish()
    }
}
impl ::core::default::Default for D3D10_TEX2DMS_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX2DMS_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.UnusedField_NothingToDefine == other.UnusedField_NothingToDefine
    }
}
impl ::core::cmp::Eq for D3D10_TEX2DMS_RTV {}
impl ::core::fmt::Debug for D3D10_TEX2DMS_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2DMS_RTV").field("UnusedField_NothingToDefine", &self.UnusedField_NothingToDefine).finish()
    }
}
impl ::core::default::Default for D3D10_TEX2DMS_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX2DMS_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.UnusedField_NothingToDefine == other.UnusedField_NothingToDefine
    }
}
impl ::core::cmp::Eq for D3D10_TEX2DMS_SRV {}
impl ::core::fmt::Debug for D3D10_TEX2DMS_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2DMS_SRV").field("UnusedField_NothingToDefine", &self.UnusedField_NothingToDefine).finish()
    }
}
impl ::core::default::Default for D3D10_TEX2D_ARRAY_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX2D_ARRAY_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D10_TEX2D_ARRAY_DSV {}
impl ::core::fmt::Debug for D3D10_TEX2D_ARRAY_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2D_ARRAY_DSV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D10_TEX2D_ARRAY_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX2D_ARRAY_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D10_TEX2D_ARRAY_RTV {}
impl ::core::fmt::Debug for D3D10_TEX2D_ARRAY_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2D_ARRAY_RTV").field("MipSlice", &self.MipSlice).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D10_TEX2D_ARRAY_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX2D_ARRAY_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.FirstArraySlice == other.FirstArraySlice && self.ArraySize == other.ArraySize
    }
}
impl ::core::cmp::Eq for D3D10_TEX2D_ARRAY_SRV {}
impl ::core::fmt::Debug for D3D10_TEX2D_ARRAY_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2D_ARRAY_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("FirstArraySlice", &self.FirstArraySlice).field("ArraySize", &self.ArraySize).finish()
    }
}
impl ::core::default::Default for D3D10_TEX2D_DSV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX2D_DSV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D10_TEX2D_DSV {}
impl ::core::fmt::Debug for D3D10_TEX2D_DSV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2D_DSV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D10_TEX2D_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX2D_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice
    }
}
impl ::core::cmp::Eq for D3D10_TEX2D_RTV {}
impl ::core::fmt::Debug for D3D10_TEX2D_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2D_RTV").field("MipSlice", &self.MipSlice).finish()
    }
}
impl ::core::default::Default for D3D10_TEX2D_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX2D_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels
    }
}
impl ::core::cmp::Eq for D3D10_TEX2D_SRV {}
impl ::core::fmt::Debug for D3D10_TEX2D_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX2D_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).finish()
    }
}
impl ::core::default::Default for D3D10_TEX3D_RTV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX3D_RTV {
    fn eq(&self, other: &Self) -> bool {
        self.MipSlice == other.MipSlice && self.FirstWSlice == other.FirstWSlice && self.WSize == other.WSize
    }
}
impl ::core::cmp::Eq for D3D10_TEX3D_RTV {}
impl ::core::fmt::Debug for D3D10_TEX3D_RTV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX3D_RTV").field("MipSlice", &self.MipSlice).field("FirstWSlice", &self.FirstWSlice).field("WSize", &self.WSize).finish()
    }
}
impl ::core::default::Default for D3D10_TEX3D_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEX3D_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels
    }
}
impl ::core::cmp::Eq for D3D10_TEX3D_SRV {}
impl ::core::fmt::Debug for D3D10_TEX3D_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEX3D_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).finish()
    }
}
impl ::core::default::Default for D3D10_TEXCUBE_ARRAY_SRV1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEXCUBE_ARRAY_SRV1 {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels && self.First2DArrayFace == other.First2DArrayFace && self.NumCubes == other.NumCubes
    }
}
impl ::core::cmp::Eq for D3D10_TEXCUBE_ARRAY_SRV1 {}
impl ::core::fmt::Debug for D3D10_TEXCUBE_ARRAY_SRV1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEXCUBE_ARRAY_SRV1").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).field("First2DArrayFace", &self.First2DArrayFace).field("NumCubes", &self.NumCubes).finish()
    }
}
impl ::core::default::Default for D3D10_TEXCUBE_SRV {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_TEXCUBE_SRV {
    fn eq(&self, other: &Self) -> bool {
        self.MostDetailedMip == other.MostDetailedMip && self.MipLevels == other.MipLevels
    }
}
impl ::core::cmp::Eq for D3D10_TEXCUBE_SRV {}
impl ::core::fmt::Debug for D3D10_TEXCUBE_SRV {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEXCUBE_SRV").field("MostDetailedMip", &self.MostDetailedMip).field("MipLevels", &self.MipLevels).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D10_TEXTURE1D_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D10_TEXTURE1D_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.MipLevels == other.MipLevels && self.ArraySize == other.ArraySize && self.Format == other.Format && self.Usage == other.Usage && self.BindFlags == other.BindFlags && self.CPUAccessFlags == other.CPUAccessFlags && self.MiscFlags == other.MiscFlags
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D10_TEXTURE1D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D10_TEXTURE1D_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEXTURE1D_DESC").field("Width", &self.Width).field("MipLevels", &self.MipLevels).field("ArraySize", &self.ArraySize).field("Format", &self.Format).field("Usage", &self.Usage).field("BindFlags", &self.BindFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("MiscFlags", &self.MiscFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D10_TEXTURE2D_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D10_TEXTURE2D_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.MipLevels == other.MipLevels && self.ArraySize == other.ArraySize && self.Format == other.Format && self.SampleDesc == other.SampleDesc && self.Usage == other.Usage && self.BindFlags == other.BindFlags && self.CPUAccessFlags == other.CPUAccessFlags && self.MiscFlags == other.MiscFlags
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D10_TEXTURE2D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D10_TEXTURE2D_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEXTURE2D_DESC").field("Width", &self.Width).field("Height", &self.Height).field("MipLevels", &self.MipLevels).field("ArraySize", &self.ArraySize).field("Format", &self.Format).field("SampleDesc", &self.SampleDesc).field("Usage", &self.Usage).field("BindFlags", &self.BindFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("MiscFlags", &self.MiscFlags).finish()
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::default::Default for D3D10_TEXTURE3D_DESC {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::PartialEq for D3D10_TEXTURE3D_DESC {
    fn eq(&self, other: &Self) -> bool {
        self.Width == other.Width && self.Height == other.Height && self.Depth == other.Depth && self.MipLevels == other.MipLevels && self.Format == other.Format && self.Usage == other.Usage && self.BindFlags == other.BindFlags && self.CPUAccessFlags == other.CPUAccessFlags && self.MiscFlags == other.MiscFlags
    }
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::cmp::Eq for D3D10_TEXTURE3D_DESC {}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl ::core::fmt::Debug for D3D10_TEXTURE3D_DESC {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_TEXTURE3D_DESC").field("Width", &self.Width).field("Height", &self.Height).field("Depth", &self.Depth).field("MipLevels", &self.MipLevels).field("Format", &self.Format).field("Usage", &self.Usage).field("BindFlags", &self.BindFlags).field("CPUAccessFlags", &self.CPUAccessFlags).field("MiscFlags", &self.MiscFlags).finish()
    }
}
impl ::core::default::Default for D3D10_TEXTURECUBE_FACE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_TEXTURECUBE_FACE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_TEXTURECUBE_FACE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_TEXTURE_ADDRESS_MODE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_TEXTURE_ADDRESS_MODE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_TEXTURE_ADDRESS_MODE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_USAGE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::core::fmt::Debug for D3D10_USAGE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("D3D10_USAGE").field(&self.0).finish()
    }
}
impl ::core::default::Default for D3D10_VIEWPORT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::cmp::PartialEq for D3D10_VIEWPORT {
    fn eq(&self, other: &Self) -> bool {
        self.TopLeftX == other.TopLeftX && self.TopLeftY == other.TopLeftY && self.Width == other.Width && self.Height == other.Height && self.MinDepth == other.MinDepth && self.MaxDepth == other.MaxDepth
    }
}
impl ::core::cmp::Eq for D3D10_VIEWPORT {}
impl ::core::fmt::Debug for D3D10_VIEWPORT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("D3D10_VIEWPORT").field("TopLeftX", &self.TopLeftX).field("TopLeftY", &self.TopLeftY).field("Width", &self.Width).field("Height", &self.Height).field("MinDepth", &self.MinDepth).field("MaxDepth", &self.MaxDepth).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10Asynchronous {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Asynchronous {}
impl ::core::fmt::Debug for ID3D10Asynchronous {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Asynchronous").field(&self.0).finish()
    }
}
impl ID3D10Asynchronous {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
}
impl ::core::cmp::PartialEq for ID3D10BlendState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10BlendState {}
impl ::core::fmt::Debug for ID3D10BlendState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10BlendState").field(&self.0).finish()
    }
}
impl ID3D10BlendState {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
}
impl ::core::cmp::PartialEq for ID3D10BlendState1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10BlendState1 {}
impl ::core::fmt::Debug for ID3D10BlendState1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10BlendState1").field(&self.0).finish()
    }
}
impl ID3D10BlendState1 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_BLEND_DESC) {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc)
    }
}
impl ::core::cmp::PartialEq for ID3D10Buffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Buffer {}
impl ::core::fmt::Debug for ID3D10Buffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Buffer").field(&self.0).finish()
    }
}
impl ID3D10Buffer {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
    pub unsafe fn GetType(&self) -> D3D10_RESOURCE_DIMENSION {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetEvictionPriority)(::windows::core::Vtable::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetEvictionPriority)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D10Counter {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Counter {}
impl ::core::fmt::Debug for ID3D10Counter {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Counter").field(&self.0).finish()
    }
}
impl ID3D10Counter {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
    pub unsafe fn Begin(&self) {
        (::windows::core::Vtable::vtable(self).base__.Begin)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn End(&self) {
        (::windows::core::Vtable::vtable(self).base__.End)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetData(&self, pdata: ::core::option::Option<*mut ::core::ffi::c_void>, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut())), datasize, getdataflags).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetDataSize)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D10Debug {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Debug {}
impl ::core::fmt::Debug for ID3D10Debug {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Debug").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10DepthStencilState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10DepthStencilState {}
impl ::core::fmt::Debug for ID3D10DepthStencilState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10DepthStencilState").field(&self.0).finish()
    }
}
impl ID3D10DepthStencilState {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
}
impl ::core::cmp::PartialEq for ID3D10DepthStencilView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10DepthStencilView {}
impl ::core::fmt::Debug for ID3D10DepthStencilView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10DepthStencilView").field(&self.0).finish()
    }
}
impl ID3D10DepthStencilView {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D10Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Resource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID3D10Device {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Device {}
impl ::core::fmt::Debug for ID3D10Device {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Device").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10Device1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Device1 {}
impl ::core::fmt::Debug for ID3D10Device1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Device1").field(&self.0).finish()
    }
}
impl ID3D10Device1 {
    pub unsafe fn VSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D10Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.VSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D10ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.PSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSSetShader<P0>(&self, ppixelshader: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10PixelShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.PSSetShader)(::windows::core::Vtable::as_raw(self), ppixelshader.into().abi())
    }
    pub unsafe fn PSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D10SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.PSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSSetShader<P0>(&self, pvertexshader: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10VertexShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.VSSetShader)(::windows::core::Vtable::as_raw(self), pvertexshader.into().abi())
    }
    pub unsafe fn DrawIndexed(&self, indexcount: u32, startindexlocation: u32, basevertexlocation: i32) {
        (::windows::core::Vtable::vtable(self).base__.DrawIndexed)(::windows::core::Vtable::as_raw(self), indexcount, startindexlocation, basevertexlocation)
    }
    pub unsafe fn Draw(&self, vertexcount: u32, startvertexlocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.Draw)(::windows::core::Vtable::as_raw(self), vertexcount, startvertexlocation)
    }
    pub unsafe fn PSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D10Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.PSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn IASetInputLayout<P0>(&self, pinputlayout: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10InputLayout>>,
    {
        (::windows::core::Vtable::vtable(self).base__.IASetInputLayout)(::windows::core::Vtable::as_raw(self), pinputlayout.into().abi())
    }
    pub unsafe fn IASetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: ::core::option::Option<*const ::core::option::Option<ID3D10Buffer>>, pstrides: ::core::option::Option<*const u32>, poffsets: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.IASetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppvertexbuffers.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pstrides.unwrap_or(::std::ptr::null())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IASetIndexBuffer<P0>(&self, pindexbuffer: P0, format: super::Dxgi::Common::DXGI_FORMAT, offset: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10Buffer>>,
    {
        (::windows::core::Vtable::vtable(self).base__.IASetIndexBuffer)(::windows::core::Vtable::as_raw(self), pindexbuffer.into().abi(), format, offset)
    }
    pub unsafe fn DrawIndexedInstanced(&self, indexcountperinstance: u32, instancecount: u32, startindexlocation: u32, basevertexlocation: i32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.DrawIndexedInstanced)(::windows::core::Vtable::as_raw(self), indexcountperinstance, instancecount, startindexlocation, basevertexlocation, startinstancelocation)
    }
    pub unsafe fn DrawInstanced(&self, vertexcountperinstance: u32, instancecount: u32, startvertexlocation: u32, startinstancelocation: u32) {
        (::windows::core::Vtable::vtable(self).base__.DrawInstanced)(::windows::core::Vtable::as_raw(self), vertexcountperinstance, instancecount, startvertexlocation, startinstancelocation)
    }
    pub unsafe fn GSSetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&[ID3D10Buffer]>) {
        (::windows::core::Vtable::vtable(self).base__.GSSetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSSetShader<P0>(&self, pshader: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10GeometryShader>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GSSetShader)(::windows::core::Vtable::as_raw(self), pshader.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IASetPrimitiveTopology(&self, topology: super::Direct3D::D3D_PRIMITIVE_TOPOLOGY) {
        (::windows::core::Vtable::vtable(self).base__.IASetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), topology)
    }
    pub unsafe fn VSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D10ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.VSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D10SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.VSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn SetPredication<P0, P1>(&self, ppredicate: P0, predicatevalue: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10Predicate>>,
        P1: ::std::convert::Into<super::super::Foundation::BOOL>,
    {
        (::windows::core::Vtable::vtable(self).base__.SetPredication)(::windows::core::Vtable::as_raw(self), ppredicate.into().abi(), predicatevalue.into())
    }
    pub unsafe fn GSSetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&[ID3D10ShaderResourceView]>) {
        (::windows::core::Vtable::vtable(self).base__.GSSetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSSetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&[ID3D10SamplerState]>) {
        (::windows::core::Vtable::vtable(self).base__.GSSetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMSetRenderTargets<P0>(&self, pprendertargetviews: ::core::option::Option<&[ID3D10RenderTargetView]>, pdepthstencilview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10DepthStencilView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OMSetRenderTargets)(::windows::core::Vtable::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), pdepthstencilview.into().abi())
    }
    pub unsafe fn OMSetBlendState<P0>(&self, pblendstate: P0, blendfactor: *const f32, samplemask: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10BlendState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OMSetBlendState)(::windows::core::Vtable::as_raw(self), pblendstate.into().abi(), blendfactor, samplemask)
    }
    pub unsafe fn OMSetDepthStencilState<P0>(&self, pdepthstencilstate: P0, stencilref: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10DepthStencilState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.OMSetDepthStencilState)(::windows::core::Vtable::as_raw(self), pdepthstencilstate.into().abi(), stencilref)
    }
    pub unsafe fn SOSetTargets(&self, numbuffers: u32, ppsotargets: ::core::option::Option<*const ::core::option::Option<ID3D10Buffer>>, poffsets: ::core::option::Option<*const u32>) {
        (::windows::core::Vtable::vtable(self).base__.SOSetTargets)(::windows::core::Vtable::as_raw(self), numbuffers, ::core::mem::transmute(ppsotargets.unwrap_or(::std::ptr::null())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn DrawAuto(&self) {
        (::windows::core::Vtable::vtable(self).base__.DrawAuto)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn RSSetState<P0>(&self, prasterizerstate: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10RasterizerState>>,
    {
        (::windows::core::Vtable::vtable(self).base__.RSSetState)(::windows::core::Vtable::as_raw(self), prasterizerstate.into().abi())
    }
    pub unsafe fn RSSetViewports(&self, pviewports: ::core::option::Option<&[D3D10_VIEWPORT]>) {
        (::windows::core::Vtable::vtable(self).base__.RSSetViewports)(::windows::core::Vtable::as_raw(self), pviewports.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pviewports.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSSetScissorRects(&self, prects: ::core::option::Option<&[super::super::Foundation::RECT]>) {
        (::windows::core::Vtable::vtable(self).base__.RSSetScissorRects)(::windows::core::Vtable::as_raw(self), prects.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(prects.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn CopySubresourceRegion<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, dstx: u32, dsty: u32, dstz: u32, psrcresource: P1, srcsubresource: u32, psrcbox: ::core::option::Option<*const D3D10_BOX>)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D10Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopySubresourceRegion)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, dstx, dsty, dstz, psrcresource.into().abi(), srcsubresource, ::core::mem::transmute(psrcbox.unwrap_or(::std::ptr::null())))
    }
    pub unsafe fn CopyResource<P0, P1>(&self, pdstresource: P0, psrcresource: P1)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D10Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CopyResource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), psrcresource.into().abi())
    }
    pub unsafe fn UpdateSubresource<P0>(&self, pdstresource: P0, dstsubresource: u32, pdstbox: ::core::option::Option<*const D3D10_BOX>, psrcdata: *const ::core::ffi::c_void, srcrowpitch: u32, srcdepthpitch: u32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.UpdateSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, ::core::mem::transmute(pdstbox.unwrap_or(::std::ptr::null())), psrcdata, srcrowpitch, srcdepthpitch)
    }
    pub unsafe fn ClearRenderTargetView<P0>(&self, prendertargetview: P0, colorrgba: *const f32)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10RenderTargetView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ClearRenderTargetView)(::windows::core::Vtable::as_raw(self), prendertargetview.into().abi(), colorrgba)
    }
    pub unsafe fn ClearDepthStencilView<P0>(&self, pdepthstencilview: P0, clearflags: u32, depth: f32, stencil: u8)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10DepthStencilView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ClearDepthStencilView)(::windows::core::Vtable::as_raw(self), pdepthstencilview.into().abi(), clearflags, depth, stencil)
    }
    pub unsafe fn GenerateMips<P0>(&self, pshaderresourceview: P0)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10ShaderResourceView>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GenerateMips)(::windows::core::Vtable::as_raw(self), pshaderresourceview.into().abi())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn ResolveSubresource<P0, P1>(&self, pdstresource: P0, dstsubresource: u32, psrcresource: P1, srcsubresource: u32, format: super::Dxgi::Common::DXGI_FORMAT)
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10Resource>>,
        P1: ::std::convert::Into<::windows::core::InParam<ID3D10Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.ResolveSubresource)(::windows::core::Vtable::as_raw(self), pdstresource.into().abi(), dstsubresource, psrcresource.into().abi(), srcsubresource, format)
    }
    pub unsafe fn VSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D10Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.VSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D10ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.PSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn PSGetShader(&self) -> ::windows::core::Result<ID3D10PixelShader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.PSGetShader)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10PixelShader as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn PSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D10SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.PSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetShader(&self) -> ::windows::core::Result<ID3D10VertexShader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.VSGetShader)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10VertexShader as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn PSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D10Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.PSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn IAGetInputLayout(&self) -> ::windows::core::Result<ID3D10InputLayout> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IAGetInputLayout)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10InputLayout as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn IAGetVertexBuffers(&self, startslot: u32, numbuffers: u32, ppvertexbuffers: ::core::option::Option<*mut ::core::option::Option<ID3D10Buffer>>, pstrides: ::core::option::Option<*mut u32>, poffsets: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.IAGetVertexBuffers)(::windows::core::Vtable::as_raw(self), startslot, numbuffers, ::core::mem::transmute(ppvertexbuffers.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstrides.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn IAGetIndexBuffer(&self, pindexbuffer: ::core::option::Option<*mut ::core::option::Option<ID3D10Buffer>>, format: ::core::option::Option<*mut super::Dxgi::Common::DXGI_FORMAT>, offset: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.IAGetIndexBuffer)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pindexbuffer.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(format.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(offset.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GSGetConstantBuffers(&self, startslot: u32, ppconstantbuffers: ::core::option::Option<&mut [::core::option::Option<ID3D10Buffer>]>) {
        (::windows::core::Vtable::vtable(self).base__.GSGetConstantBuffers)(::windows::core::Vtable::as_raw(self), startslot, ppconstantbuffers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppconstantbuffers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSGetShader(&self) -> ::windows::core::Result<ID3D10GeometryShader> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GSGetShader)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10GeometryShader as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`*"]
    #[cfg(feature = "Win32_Graphics_Direct3D")]
    pub unsafe fn IAGetPrimitiveTopology(&self) -> super::Direct3D::D3D_PRIMITIVE_TOPOLOGY {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.IAGetPrimitiveTopology)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn VSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D10ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.VSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn VSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D10SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.VSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn GetPredication(&self, pppredicate: ::core::option::Option<*mut ::core::option::Option<ID3D10Predicate>>, ppredicatevalue: ::core::option::Option<*mut super::super::Foundation::BOOL>) {
        (::windows::core::Vtable::vtable(self).base__.GetPredication)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pppredicate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(ppredicatevalue.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GSGetShaderResources(&self, startslot: u32, ppshaderresourceviews: ::core::option::Option<&mut [::core::option::Option<ID3D10ShaderResourceView>]>) {
        (::windows::core::Vtable::vtable(self).base__.GSGetShaderResources)(::windows::core::Vtable::as_raw(self), startslot, ppshaderresourceviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppshaderresourceviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn GSGetSamplers(&self, startslot: u32, ppsamplers: ::core::option::Option<&mut [::core::option::Option<ID3D10SamplerState>]>) {
        (::windows::core::Vtable::vtable(self).base__.GSGetSamplers)(::windows::core::Vtable::as_raw(self), startslot, ppsamplers.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(ppsamplers.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())))
    }
    pub unsafe fn OMGetRenderTargets(&self, pprendertargetviews: ::core::option::Option<&mut [::core::option::Option<ID3D10RenderTargetView>]>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D10DepthStencilView>>) {
        (::windows::core::Vtable::vtable(self).base__.OMGetRenderTargets)(::windows::core::Vtable::as_raw(self), pprendertargetviews.as_deref().map_or(0, |slice| slice.len() as _), ::core::mem::transmute(pprendertargetviews.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn OMGetBlendState(&self, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D10BlendState>>, blendfactor: ::core::option::Option<*mut f32>, psamplemask: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.OMGetBlendState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(blendfactor.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(psamplemask.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn OMGetDepthStencilState(&self, ppdepthstencilstate: ::core::option::Option<*mut ::core::option::Option<ID3D10DepthStencilState>>, pstencilref: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.OMGetDepthStencilState)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(ppdepthstencilstate.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pstencilref.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn SOGetTargets(&self, numbuffers: u32, ppsotargets: ::core::option::Option<*mut ::core::option::Option<ID3D10Buffer>>, poffsets: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.SOGetTargets)(::windows::core::Vtable::as_raw(self), numbuffers, ::core::mem::transmute(ppsotargets.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(poffsets.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn RSGetState(&self) -> ::windows::core::Result<ID3D10RasterizerState> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.RSGetState)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10RasterizerState as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    pub unsafe fn RSGetViewports(&self, numviewports: *mut u32, pviewports: ::core::option::Option<*mut D3D10_VIEWPORT>) {
        (::windows::core::Vtable::vtable(self).base__.RSGetViewports)(::windows::core::Vtable::as_raw(self), numviewports, ::core::mem::transmute(pviewports.unwrap_or(::std::ptr::null_mut())))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn RSGetScissorRects(&self, numrects: *mut u32, prects: ::core::option::Option<*mut super::super::Foundation::RECT>) {
        (::windows::core::Vtable::vtable(self).base__.RSGetScissorRects)(::windows::core::Vtable::as_raw(self), numrects, ::core::mem::transmute(prects.unwrap_or(::std::ptr::null_mut())))
    }
    pub unsafe fn GetDeviceRemovedReason(&self) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDeviceRemovedReason)(::windows::core::Vtable::as_raw(self)).ok()
    }
    pub unsafe fn SetExceptionMode(&self, raiseflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetExceptionMode)(::windows::core::Vtable::as_raw(self), raiseflags).ok()
    }
    pub unsafe fn GetExceptionMode(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetExceptionMode)(::windows::core::Vtable::as_raw(self))
    }
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
    pub unsafe fn ClearState(&self) {
        (::windows::core::Vtable::vtable(self).base__.ClearState)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn Flush(&self) {
        (::windows::core::Vtable::vtable(self).base__.Flush)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn CreateBuffer(&self, pdesc: *const D3D10_BUFFER_DESC, pinitialdata: ::core::option::Option<*const D3D10_SUBRESOURCE_DATA>, ppbuffer: ::core::option::Option<*mut ::core::option::Option<ID3D10Buffer>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateBuffer)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppbuffer.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture1D(&self, pdesc: *const D3D10_TEXTURE1D_DESC, pinitialdata: ::core::option::Option<*const D3D10_SUBRESOURCE_DATA>) -> ::windows::core::Result<ID3D10Texture1D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTexture1D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture2D(&self, pdesc: *const D3D10_TEXTURE2D_DESC, pinitialdata: ::core::option::Option<*const D3D10_SUBRESOURCE_DATA>) -> ::windows::core::Result<ID3D10Texture2D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTexture2D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateTexture3D(&self, pdesc: *const D3D10_TEXTURE3D_DESC, pinitialdata: ::core::option::Option<*const D3D10_SUBRESOURCE_DATA>) -> ::windows::core::Result<ID3D10Texture3D> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CreateTexture3D)(::windows::core::Vtable::as_raw(self), pdesc, ::core::mem::transmute(pinitialdata.unwrap_or(::std::ptr::null())), result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn CreateShaderResourceView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D10_SHADER_RESOURCE_VIEW_DESC>, ppsrview: ::core::option::Option<*mut ::core::option::Option<ID3D10ShaderResourceView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateShaderResourceView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppsrview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateRenderTargetView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D10_RENDER_TARGET_VIEW_DESC>, pprtview: ::core::option::Option<*mut ::core::option::Option<ID3D10RenderTargetView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateRenderTargetView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(pprtview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateDepthStencilView<P0>(&self, presource: P0, pdesc: ::core::option::Option<*const D3D10_DEPTH_STENCIL_VIEW_DESC>, ppdepthstencilview: ::core::option::Option<*mut ::core::option::Option<ID3D10DepthStencilView>>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<::windows::core::InParam<ID3D10Resource>>,
    {
        (::windows::core::Vtable::vtable(self).base__.CreateDepthStencilView)(::windows::core::Vtable::as_raw(self), presource.into().abi(), ::core::mem::transmute(pdesc.unwrap_or(::std::ptr::null())), ::core::mem::transmute(ppdepthstencilview.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CreateInputLayout(&self, pinputelementdescs: &[D3D10_INPUT_ELEMENT_DESC], pshaderbytecodewithinputsignature: &[u8], ppinputlayout: ::core::option::Option<*mut ::core::option::Option<ID3D10InputLayout>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateInputLayout)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pinputelementdescs.as_ptr()), pinputelementdescs.len() as _, ::core::mem::transmute(pshaderbytecodewithinputsignature.as_ptr()), pshaderbytecodewithinputsignature.len() as _, ::core::mem::transmute(ppinputlayout.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateVertexShader(&self, pshaderbytecode: &[u8], ppvertexshader: ::core::option::Option<*mut ::core::option::Option<ID3D10VertexShader>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateVertexShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, ::core::mem::transmute(ppvertexshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShader(&self, pshaderbytecode: &[u8], ppgeometryshader: ::core::option::Option<*mut ::core::option::Option<ID3D10GeometryShader>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateGeometryShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, ::core::mem::transmute(ppgeometryshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateGeometryShaderWithStreamOutput(&self, pshaderbytecode: &[u8], psodeclaration: ::core::option::Option<&[D3D10_SO_DECLARATION_ENTRY]>, outputstreamstride: u32, ppgeometryshader: ::core::option::Option<*mut ::core::option::Option<ID3D10GeometryShader>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateGeometryShaderWithStreamOutput)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, ::core::mem::transmute(psodeclaration.as_deref().map_or(::core::ptr::null(), |slice| slice.as_ptr())), psodeclaration.as_deref().map_or(0, |slice| slice.len() as _), outputstreamstride, ::core::mem::transmute(ppgeometryshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreatePixelShader(&self, pshaderbytecode: &[u8], pppixelshader: ::core::option::Option<*mut ::core::option::Option<ID3D10PixelShader>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreatePixelShader)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pshaderbytecode.as_ptr()), pshaderbytecode.len() as _, ::core::mem::transmute(pppixelshader.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateBlendState(&self, pblendstatedesc: *const D3D10_BLEND_DESC, ppblendstate: ::core::option::Option<*mut ::core::option::Option<ID3D10BlendState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateBlendState)(::windows::core::Vtable::as_raw(self), pblendstatedesc, ::core::mem::transmute(ppblendstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateDepthStencilState(&self, pdepthstencildesc: *const D3D10_DEPTH_STENCIL_DESC, ppdepthstencilstate: ::core::option::Option<*mut ::core::option::Option<ID3D10DepthStencilState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateDepthStencilState)(::windows::core::Vtable::as_raw(self), pdepthstencildesc, ::core::mem::transmute(ppdepthstencilstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn CreateRasterizerState(&self, prasterizerdesc: *const D3D10_RASTERIZER_DESC, pprasterizerstate: ::core::option::Option<*mut ::core::option::Option<ID3D10RasterizerState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateRasterizerState)(::windows::core::Vtable::as_raw(self), prasterizerdesc, ::core::mem::transmute(pprasterizerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateSamplerState(&self, psamplerdesc: *const D3D10_SAMPLER_DESC, ppsamplerstate: ::core::option::Option<*mut ::core::option::Option<ID3D10SamplerState>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateSamplerState)(::windows::core::Vtable::as_raw(self), psamplerdesc, ::core::mem::transmute(ppsamplerstate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateQuery(&self, pquerydesc: *const D3D10_QUERY_DESC, ppquery: ::core::option::Option<*mut ::core::option::Option<ID3D10Query>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateQuery)(::windows::core::Vtable::as_raw(self), pquerydesc, ::core::mem::transmute(ppquery.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreatePredicate(&self, ppredicatedesc: *const D3D10_QUERY_DESC, pppredicate: ::core::option::Option<*mut ::core::option::Option<ID3D10Predicate>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreatePredicate)(::windows::core::Vtable::as_raw(self), ppredicatedesc, ::core::mem::transmute(pppredicate.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn CreateCounter(&self, pcounterdesc: *const D3D10_COUNTER_DESC, ppcounter: ::core::option::Option<*mut ::core::option::Option<ID3D10Counter>>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CreateCounter)(::windows::core::Vtable::as_raw(self), pcounterdesc, ::core::mem::transmute(ppcounter.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckFormatSupport(&self, format: super::Dxgi::Common::DXGI_FORMAT) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckFormatSupport)(::windows::core::Vtable::as_raw(self), format, result__.as_mut_ptr()).from_abi(result__)
    }
    #[doc = "*Required features: `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn CheckMultisampleQualityLevels(&self, format: super::Dxgi::Common::DXGI_FORMAT, samplecount: u32) -> ::windows::core::Result<u32> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckMultisampleQualityLevels)(::windows::core::Vtable::as_raw(self), format, samplecount, result__.as_mut_ptr()).from_abi(result__)
    }
    pub unsafe fn CheckCounterInfo(&self) -> D3D10_COUNTER_INFO {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.CheckCounterInfo)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn CheckCounter(&self, pdesc: *const D3D10_COUNTER_DESC, ptype: *mut D3D10_COUNTER_TYPE, pactivecounters: *mut u32, szname: ::windows::core::PSTR, pnamelength: ::core::option::Option<*mut u32>, szunits: ::windows::core::PSTR, punitslength: ::core::option::Option<*mut u32>, szdescription: ::windows::core::PSTR, pdescriptionlength: ::core::option::Option<*mut u32>) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.CheckCounter)(::windows::core::Vtable::as_raw(self), pdesc, ptype, pactivecounters, ::core::mem::transmute(szname), ::core::mem::transmute(pnamelength.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szunits), ::core::mem::transmute(punitslength.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(szdescription), ::core::mem::transmute(pdescriptionlength.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn GetCreationFlags(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetCreationFlags)(::windows::core::Vtable::as_raw(self))
    }
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn OpenSharedResource<P0>(&self, hresource: P0, returnedinterface: *const ::windows::core::GUID, ppresource: ::core::option::Option<*mut *mut ::core::ffi::c_void>) -> ::windows::core::Result<()>
    where
        P0: ::std::convert::Into<super::super::Foundation::HANDLE>,
    {
        (::windows::core::Vtable::vtable(self).base__.OpenSharedResource)(::windows::core::Vtable::as_raw(self), hresource.into(), returnedinterface, ::core::mem::transmute(ppresource.unwrap_or(::std::ptr::null_mut()))).ok()
    }
    pub unsafe fn SetTextFilterSize(&self, width: u32, height: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetTextFilterSize)(::windows::core::Vtable::as_raw(self), width, height)
    }
    pub unsafe fn GetTextFilterSize(&self, pwidth: ::core::option::Option<*mut u32>, pheight: ::core::option::Option<*mut u32>) {
        (::windows::core::Vtable::vtable(self).base__.GetTextFilterSize)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pwidth.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(pheight.unwrap_or(::std::ptr::null_mut())))
    }
}
impl ::core::cmp::PartialEq for ID3D10DeviceChild {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10DeviceChild {}
impl ::core::fmt::Debug for ID3D10DeviceChild {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10DeviceChild").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10Effect {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Effect {}
impl ::core::fmt::Debug for ID3D10Effect {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Effect").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectBlendVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectBlendVariable {}
impl ::core::fmt::Debug for ID3D10EffectBlendVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectBlendVariable").field(&self.0).finish()
    }
}
impl ID3D10EffectBlendVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsValid)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberBySemantic)(::windows::core::Vtable::as_raw(self), semantic.into().abi())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsScalar)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsVector)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsMatrix)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsString)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShaderResource)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRenderTargetView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencilView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.AsConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShader)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencil)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRasterizer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsSampler)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectConstantBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectConstantBuffer {}
impl ::core::fmt::Debug for ID3D10EffectConstantBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectConstantBuffer").field(&self.0).finish()
    }
}
impl ID3D10EffectConstantBuffer {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsValid)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberBySemantic)(::windows::core::Vtable::as_raw(self), semantic.into().abi())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsScalar)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsVector)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsMatrix)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsString)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShaderResource)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRenderTargetView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencilView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.AsConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShader)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencil)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRasterizer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsSampler)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectDepthStencilVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectDepthStencilVariable {}
impl ::core::fmt::Debug for ID3D10EffectDepthStencilVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectDepthStencilVariable").field(&self.0).finish()
    }
}
impl ID3D10EffectDepthStencilVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsValid)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberBySemantic)(::windows::core::Vtable::as_raw(self), semantic.into().abi())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsScalar)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsVector)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsMatrix)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsString)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShaderResource)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRenderTargetView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencilView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.AsConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShader)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencil)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRasterizer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsSampler)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectDepthStencilViewVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectDepthStencilViewVariable {}
impl ::core::fmt::Debug for ID3D10EffectDepthStencilViewVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectDepthStencilViewVariable").field(&self.0).finish()
    }
}
impl ID3D10EffectDepthStencilViewVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsValid)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberBySemantic)(::windows::core::Vtable::as_raw(self), semantic.into().abi())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsScalar)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsVector)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsMatrix)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsString)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShaderResource)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRenderTargetView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencilView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.AsConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShader)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencil)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRasterizer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsSampler)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectMatrixVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectMatrixVariable {}
impl ::core::fmt::Debug for ID3D10EffectMatrixVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectMatrixVariable").field(&self.0).finish()
    }
}
impl ID3D10EffectMatrixVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsValid)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberBySemantic)(::windows::core::Vtable::as_raw(self), semantic.into().abi())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsScalar)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsVector)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsMatrix)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsString)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShaderResource)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRenderTargetView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencilView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.AsConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShader)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencil)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRasterizer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsSampler)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectPass {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectPass {}
impl ::core::fmt::Debug for ID3D10EffectPass {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectPass").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectPool {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectPool {}
impl ::core::fmt::Debug for ID3D10EffectPool {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectPool").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectRasterizerVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectRasterizerVariable {}
impl ::core::fmt::Debug for ID3D10EffectRasterizerVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectRasterizerVariable").field(&self.0).finish()
    }
}
impl ID3D10EffectRasterizerVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsValid)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberBySemantic)(::windows::core::Vtable::as_raw(self), semantic.into().abi())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsScalar)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsVector)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsMatrix)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsString)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShaderResource)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRenderTargetView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencilView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.AsConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShader)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencil)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRasterizer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsSampler)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectRenderTargetViewVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectRenderTargetViewVariable {}
impl ::core::fmt::Debug for ID3D10EffectRenderTargetViewVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectRenderTargetViewVariable").field(&self.0).finish()
    }
}
impl ID3D10EffectRenderTargetViewVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsValid)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberBySemantic)(::windows::core::Vtable::as_raw(self), semantic.into().abi())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsScalar)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsVector)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsMatrix)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsString)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShaderResource)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRenderTargetView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencilView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.AsConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShader)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencil)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRasterizer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsSampler)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectSamplerVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectSamplerVariable {}
impl ::core::fmt::Debug for ID3D10EffectSamplerVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectSamplerVariable").field(&self.0).finish()
    }
}
impl ID3D10EffectSamplerVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsValid)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberBySemantic)(::windows::core::Vtable::as_raw(self), semantic.into().abi())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsScalar)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsVector)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsMatrix)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsString)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShaderResource)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRenderTargetView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencilView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.AsConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShader)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencil)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRasterizer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsSampler)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectScalarVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectScalarVariable {}
impl ::core::fmt::Debug for ID3D10EffectScalarVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectScalarVariable").field(&self.0).finish()
    }
}
impl ID3D10EffectScalarVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsValid)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberBySemantic)(::windows::core::Vtable::as_raw(self), semantic.into().abi())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsScalar)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsVector)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsMatrix)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsString)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShaderResource)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRenderTargetView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencilView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.AsConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShader)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencil)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRasterizer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsSampler)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectShaderResourceVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectShaderResourceVariable {}
impl ::core::fmt::Debug for ID3D10EffectShaderResourceVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectShaderResourceVariable").field(&self.0).finish()
    }
}
impl ID3D10EffectShaderResourceVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsValid)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberBySemantic)(::windows::core::Vtable::as_raw(self), semantic.into().abi())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsScalar)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsVector)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsMatrix)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsString)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShaderResource)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRenderTargetView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencilView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.AsConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShader)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencil)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRasterizer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsSampler)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectShaderVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectShaderVariable {}
impl ::core::fmt::Debug for ID3D10EffectShaderVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectShaderVariable").field(&self.0).finish()
    }
}
impl ID3D10EffectShaderVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsValid)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberBySemantic)(::windows::core::Vtable::as_raw(self), semantic.into().abi())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsScalar)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsVector)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsMatrix)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsString)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShaderResource)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRenderTargetView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencilView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.AsConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShader)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencil)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRasterizer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsSampler)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectStringVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectStringVariable {}
impl ::core::fmt::Debug for ID3D10EffectStringVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectStringVariable").field(&self.0).finish()
    }
}
impl ID3D10EffectStringVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsValid)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberBySemantic)(::windows::core::Vtable::as_raw(self), semantic.into().abi())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsScalar)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsVector)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsMatrix)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsString)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShaderResource)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRenderTargetView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencilView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.AsConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShader)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencil)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRasterizer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsSampler)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectTechnique {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectTechnique {}
impl ::core::fmt::Debug for ID3D10EffectTechnique {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectTechnique").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectType {}
impl ::core::fmt::Debug for ID3D10EffectType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectVariable {}
impl ::core::fmt::Debug for ID3D10EffectVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectVariable").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10EffectVectorVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10EffectVectorVariable {}
impl ::core::fmt::Debug for ID3D10EffectVectorVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10EffectVectorVariable").field(&self.0).finish()
    }
}
impl ID3D10EffectVectorVariable {
    #[doc = "*Required features: `\"Win32_Foundation\"`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub unsafe fn IsValid(&self) -> super::super::Foundation::BOOL {
        (::windows::core::Vtable::vtable(self).base__.IsValid)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetType(&self) -> ::core::option::Option<ID3D10EffectType> {
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_EFFECT_VARIABLE_DESC) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc).ok()
    }
    pub unsafe fn GetAnnotationByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetAnnotationByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetAnnotationByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberByIndex(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByIndex)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetMemberByName<P0>(&self, name: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberByName)(::windows::core::Vtable::as_raw(self), name.into().abi())
    }
    pub unsafe fn GetMemberBySemantic<P0>(&self, semantic: P0) -> ::core::option::Option<ID3D10EffectVariable>
    where
        P0: ::std::convert::Into<::windows::core::InParam<::windows::core::PCSTR>>,
    {
        (::windows::core::Vtable::vtable(self).base__.GetMemberBySemantic)(::windows::core::Vtable::as_raw(self), semantic.into().abi())
    }
    pub unsafe fn GetElement(&self, index: u32) -> ::core::option::Option<ID3D10EffectVariable> {
        (::windows::core::Vtable::vtable(self).base__.GetElement)(::windows::core::Vtable::as_raw(self), index)
    }
    pub unsafe fn GetParentConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.GetParentConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsScalar(&self) -> ::core::option::Option<ID3D10EffectScalarVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsScalar)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsVector(&self) -> ::core::option::Option<ID3D10EffectVectorVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsVector)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsMatrix(&self) -> ::core::option::Option<ID3D10EffectMatrixVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsMatrix)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsString(&self) -> ::core::option::Option<ID3D10EffectStringVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsString)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShaderResource(&self) -> ::core::option::Option<ID3D10EffectShaderResourceVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShaderResource)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRenderTargetView(&self) -> ::core::option::Option<ID3D10EffectRenderTargetViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRenderTargetView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencilView(&self) -> ::core::option::Option<ID3D10EffectDepthStencilViewVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencilView)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsConstantBuffer(&self) -> ::core::option::Option<ID3D10EffectConstantBuffer> {
        (::windows::core::Vtable::vtable(self).base__.AsConstantBuffer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsShader(&self) -> ::core::option::Option<ID3D10EffectShaderVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsShader)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsBlend(&self) -> ::core::option::Option<ID3D10EffectBlendVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsBlend)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsDepthStencil(&self) -> ::core::option::Option<ID3D10EffectDepthStencilVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsDepthStencil)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsRasterizer(&self) -> ::core::option::Option<ID3D10EffectRasterizerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsRasterizer)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn AsSampler(&self) -> ::core::option::Option<ID3D10EffectSamplerVariable> {
        (::windows::core::Vtable::vtable(self).base__.AsSampler)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn SetRawValue(&self, pdata: *const ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.SetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
    pub unsafe fn GetRawValue(&self, pdata: *mut ::core::ffi::c_void, offset: u32, bytecount: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetRawValue)(::windows::core::Vtable::as_raw(self), pdata, offset, bytecount).ok()
    }
}
impl ::core::cmp::PartialEq for ID3D10GeometryShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10GeometryShader {}
impl ::core::fmt::Debug for ID3D10GeometryShader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10GeometryShader").field(&self.0).finish()
    }
}
impl ID3D10GeometryShader {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
}
impl ::core::cmp::PartialEq for ID3D10InfoQueue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10InfoQueue {}
impl ::core::fmt::Debug for ID3D10InfoQueue {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10InfoQueue").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10InputLayout {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10InputLayout {}
impl ::core::fmt::Debug for ID3D10InputLayout {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10InputLayout").field(&self.0).finish()
    }
}
impl ID3D10InputLayout {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
}
impl ::core::cmp::PartialEq for ID3D10Multithread {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Multithread {}
impl ::core::fmt::Debug for ID3D10Multithread {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Multithread").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10PixelShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10PixelShader {}
impl ::core::fmt::Debug for ID3D10PixelShader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10PixelShader").field(&self.0).finish()
    }
}
impl ID3D10PixelShader {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
}
impl ::core::cmp::PartialEq for ID3D10Predicate {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Predicate {}
impl ::core::fmt::Debug for ID3D10Predicate {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Predicate").field(&self.0).finish()
    }
}
impl ID3D10Predicate {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
    pub unsafe fn Begin(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.Begin)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn End(&self) {
        (::windows::core::Vtable::vtable(self).base__.base__.End)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetData(&self, pdata: ::core::option::Option<*mut ::core::ffi::c_void>, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.base__.GetData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut())), datasize, getdataflags).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.base__.GetDataSize)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetDesc(&self) -> D3D10_QUERY_DESC {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
}
impl ::core::cmp::PartialEq for ID3D10Query {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Query {}
impl ::core::fmt::Debug for ID3D10Query {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Query").field(&self.0).finish()
    }
}
impl ID3D10Query {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
    pub unsafe fn Begin(&self) {
        (::windows::core::Vtable::vtable(self).base__.Begin)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn End(&self) {
        (::windows::core::Vtable::vtable(self).base__.End)(::windows::core::Vtable::as_raw(self))
    }
    pub unsafe fn GetData(&self, pdata: ::core::option::Option<*mut ::core::ffi::c_void>, datasize: u32, getdataflags: u32) -> ::windows::core::Result<()> {
        (::windows::core::Vtable::vtable(self).base__.GetData)(::windows::core::Vtable::as_raw(self), ::core::mem::transmute(pdata.unwrap_or(::std::ptr::null_mut())), datasize, getdataflags).ok()
    }
    pub unsafe fn GetDataSize(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetDataSize)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D10RasterizerState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10RasterizerState {}
impl ::core::fmt::Debug for ID3D10RasterizerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10RasterizerState").field(&self.0).finish()
    }
}
impl ID3D10RasterizerState {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
}
impl ::core::cmp::PartialEq for ID3D10RenderTargetView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10RenderTargetView {}
impl ::core::fmt::Debug for ID3D10RenderTargetView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10RenderTargetView").field(&self.0).finish()
    }
}
impl ID3D10RenderTargetView {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D10Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Resource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID3D10Resource {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Resource {}
impl ::core::fmt::Debug for ID3D10Resource {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Resource").field(&self.0).finish()
    }
}
impl ID3D10Resource {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
}
impl ::core::cmp::PartialEq for ID3D10SamplerState {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10SamplerState {}
impl ::core::fmt::Debug for ID3D10SamplerState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10SamplerState").field(&self.0).finish()
    }
}
impl ID3D10SamplerState {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
}
impl ::core::cmp::PartialEq for ID3D10ShaderReflection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10ShaderReflection {}
impl ::core::fmt::Debug for ID3D10ShaderReflection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10ShaderReflection").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10ShaderReflection1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10ShaderReflection1 {}
impl ::core::fmt::Debug for ID3D10ShaderReflection1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10ShaderReflection1").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10ShaderReflectionConstantBuffer {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10ShaderReflectionConstantBuffer {}
impl ::core::fmt::Debug for ID3D10ShaderReflectionConstantBuffer {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10ShaderReflectionConstantBuffer").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10ShaderReflectionType {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10ShaderReflectionType {}
impl ::core::fmt::Debug for ID3D10ShaderReflectionType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10ShaderReflectionType").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10ShaderReflectionVariable {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10ShaderReflectionVariable {}
impl ::core::fmt::Debug for ID3D10ShaderReflectionVariable {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10ShaderReflectionVariable").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10ShaderResourceView {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10ShaderResourceView {}
impl ::core::fmt::Debug for ID3D10ShaderResourceView {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10ShaderResourceView").field(&self.0).finish()
    }
}
impl ID3D10ShaderResourceView {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D10Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Resource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
}
impl ::core::cmp::PartialEq for ID3D10ShaderResourceView1 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10ShaderResourceView1 {}
impl ::core::fmt::Debug for ID3D10ShaderResourceView1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10ShaderResourceView1").field(&self.0).finish()
    }
}
impl ID3D10ShaderResourceView1 {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
    pub unsafe fn GetResource(&self) -> ::windows::core::Result<ID3D10Resource> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetResource)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Resource as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
    #[doc = "*Required features: `\"Win32_Graphics_Direct3D\"`, `\"Win32_Graphics_Dxgi_Common\"`*"]
    #[cfg(all(feature = "Win32_Graphics_Direct3D", feature = "Win32_Graphics_Dxgi_Common"))]
    pub unsafe fn GetDesc(&self, pdesc: *mut D3D10_SHADER_RESOURCE_VIEW_DESC) {
        (::windows::core::Vtable::vtable(self).base__.GetDesc)(::windows::core::Vtable::as_raw(self), pdesc)
    }
}
impl ::core::cmp::PartialEq for ID3D10StateBlock {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10StateBlock {}
impl ::core::fmt::Debug for ID3D10StateBlock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10StateBlock").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10SwitchToRef {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10SwitchToRef {}
impl ::core::fmt::Debug for ID3D10SwitchToRef {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10SwitchToRef").field(&self.0).finish()
    }
}
impl ::core::cmp::PartialEq for ID3D10Texture1D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Texture1D {}
impl ::core::fmt::Debug for ID3D10Texture1D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Texture1D").field(&self.0).finish()
    }
}
impl ID3D10Texture1D {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
    pub unsafe fn GetType(&self) -> D3D10_RESOURCE_DIMENSION {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetEvictionPriority)(::windows::core::Vtable::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetEvictionPriority)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D10Texture2D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Texture2D {}
impl ::core::fmt::Debug for ID3D10Texture2D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Texture2D").field(&self.0).finish()
    }
}
impl ID3D10Texture2D {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
    pub unsafe fn GetType(&self) -> D3D10_RESOURCE_DIMENSION {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetEvictionPriority)(::windows::core::Vtable::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetEvictionPriority)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D10Texture3D {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10Texture3D {}
impl ::core::fmt::Debug for ID3D10Texture3D {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10Texture3D").field(&self.0).finish()
    }
}
impl ID3D10Texture3D {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
    pub unsafe fn GetType(&self) -> D3D10_RESOURCE_DIMENSION {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetType)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        result__.assume_init()
    }
    pub unsafe fn SetEvictionPriority(&self, evictionpriority: u32) {
        (::windows::core::Vtable::vtable(self).base__.SetEvictionPriority)(::windows::core::Vtable::as_raw(self), evictionpriority)
    }
    pub unsafe fn GetEvictionPriority(&self) -> u32 {
        (::windows::core::Vtable::vtable(self).base__.GetEvictionPriority)(::windows::core::Vtable::as_raw(self))
    }
}
impl ::core::cmp::PartialEq for ID3D10VertexShader {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10VertexShader {}
impl ::core::fmt::Debug for ID3D10VertexShader {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10VertexShader").field(&self.0).finish()
    }
}
impl ID3D10VertexShader {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
}
impl ::core::cmp::PartialEq for ID3D10View {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for ID3D10View {}
impl ::core::fmt::Debug for ID3D10View {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("ID3D10View").field(&self.0).finish()
    }
}
impl ID3D10View {
    pub unsafe fn GetDevice(&self) -> ::windows::core::Result<ID3D10Device> {
        let mut result__ = ::core::mem::MaybeUninit::zeroed();
        (::windows::core::Vtable::vtable(self).base__.GetDevice)(::windows::core::Vtable::as_raw(self), result__.as_mut_ptr());
        <ID3D10Device as ::windows::core::Abi>::from_abi(result__.assume_init())
    }
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
}
